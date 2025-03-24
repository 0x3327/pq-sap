use ethers::{abi::{decode, Address}, contract::abigen, middleware::SignerMiddleware, providers::{Http, Middleware, Provider}, signers::{LocalWallet, Signer}, types::{BlockNumber, Filter, TransactionRequest, H256, U256, U64}, utils::parse_ether};
use pqc_kyber::KYBER_PUBLICKEYBYTES;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::{env, error::Error, sync::Arc};
use crate::{on_chain::rest::{controller::blockchain_controller::{ScanEntry, ScanRequest, SendEthResponse}, repository::meta_data_repository::MetaDataRepository}, sender::sender::SenderInputData, versions::v1::calculate_stealth_priv_key};

abigen!(
    PQSAP_Announcer,
    r#"[
        "function sendEthViaProxy(address _stealthAddress, bytes _R, bytes _viewTag) payable"
    ]"#
);


pub struct BlockchainService{
    meta_data_repo: Arc<MetaDataRepository>, 
}
impl BlockchainService {
    pub fn new(meta_data_repo: MetaDataRepository) -> Self {
        Self { meta_data_repo: Arc::new(meta_data_repo) }
    }
    /// Sends `value` of eth to a generated stealth address, announces ephemeral key and calculated view tag.  
    /// 
    /// ### Arguments 
    /// * `value` - how much eth to send to a stealth address
    /// * `wallet` - private key of wallet 
    /// * `meta_address` - meta address fetched from ENS 
    pub async fn send_eth(value: f64, wallet: &String, ens_name: &String) -> Result<(Address, H256), Box<dyn Error>>{
        let meta_address_string = fetch_meta_address(&ens_name).await?;
        let meta_address: SenderInputData = serde_json::from_str(&meta_address_string)?;

        if meta_address.viewing_pk.len()/2 != KYBER_PUBLICKEYBYTES{
            return Err(format!("Error: spending key must be of size {}", KYBER_PUBLICKEYBYTES).into())
        }

        let endpoint =  env::var("CONNECTION_STRING").expect("Incorrect connection string.");
        let client = reqwest::Client::new(); 
        let response = client
        .post(format!("http://{}/send-eth", endpoint))
        .json(&meta_address)
        .send()
        .await?;

        let response_data: SendEthResponse = response.json().await?;
   
        let provider_string = env::var("PROVIDER_STRING").expect("Provider not set");
        let provider = Provider::<Http>::try_from(provider_string)?;
        let chain_id = provider.get_chainid().await?;
        
        let wallet = wallet.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64()); 
        
        let client = Arc::new(SignerMiddleware::new(provider, wallet)); 

        let contract_address:Address = env::var("CONTRACT_ADDRESS").expect("Contract address not set").parse()?;
        let contract = PQSAP_Announcer::new(contract_address, client); 

        let value = U256::from(parse_ether(value)?);

        let ephemeral_pub_key = hex::decode(&response_data.ephemeral_public_key)?;
        let view_tag = hex::decode(&response_data.viewtag)?;

        let tx_hash = contract.send_eth_via_proxy(response_data.stealth_address, ephemeral_pub_key.into(), view_tag.into())
        .value(value).send().await?.tx_hash();



        Ok((response_data.stealth_address, tx_hash))
    }


    /// Receives the max amount possible of eth from newest transactions found since the last scan. 
    /// 
    /// ### Arguments
    /// * `k_hex` - hex encoding of private spending key `k`, `secp256k1` curve `SecretKey`
    /// * `v_hex` - hex encoding of private viewing key `v`, `Kyber` private key 
    /// * `destination_wallet` - wallet to send eth to the hex encoded wallet address
    pub async fn receive_eth(&self, k_hex: &String, v_hex: &String, destination_wallet: &String) -> Result<Vec<(Address, u128)>, Box<dyn Error>>{  
        let k_bytes = hex::decode(k_hex)?;
        let k = SecretKey::from_slice(&k_bytes)?;
      
        let secp = Secp256k1::new();
        let k_pub = PublicKey::from_secret_key(&secp, &k); 
        let k_pub = hex::encode(k_pub.serialize_uncompressed()); 


        let scan_request = ScanRequest{
            k_pub, 
            v: v_hex.to_string(), 
            destination_wallet: destination_wallet.to_string(),
        }; 

        // scan for transactions needed 
        let endpoint = env::var("CONNECTION_STRING").expect("Incorrect connection string.");
        let client = reqwest::Client::new(); 
        let response = client
        .post(format!("http://{}/scan-eth", endpoint))
        .json(&scan_request)
        .send()
        .await?;

        let scan_result: Vec<ScanEntry> = response.json().await?; 
    
        let provider_string = env::var("PROVIDER_STRING").expect("Provider not set");
        let provider = Provider::<Http>::try_from(provider_string)?; 
        let chain_id = provider.get_chainid().await?;

        let mut result: Vec<(Address, u128)> = vec![]; 

        if scan_result.len() > 0{
            for s in scan_result{
                let mut ss: [u8; 32] = [0; 32]; 
                hex::decode_to_slice(s.shared_secret, &mut ss)?;

                let p = calculate_stealth_priv_key(&ss, &k);
  
                let wallet = p.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
            
                let client = SignerMiddleware::new(provider.clone(), wallet.clone());
                
                let destination = destination_wallet.parse::<Address>()?;
                
                let balance = client.get_balance(wallet.address(), None).await?;

                // Estimate needed gas 
                let dummy_tx = TransactionRequest::new()
                    .to(destination)
                    .value(U256::from(0))
                    .from(wallet.address());
                let gas_price = client.get_gas_price().await?;
                let gas_estimate = client.estimate_gas(&dummy_tx.into(), None).await?;
                let gas_cost = gas_price * gas_estimate;

                if balance > gas_cost{
                    let max_amount = balance - gas_cost; 

                    let tx = TransactionRequest::new()
                    .to(destination)
                    .value(max_amount)
                    .from(wallet.address()); 

                
                    let pending_tx = client.send_transaction(tx, None).await?;
                    let _ = pending_tx.await?;   
                    result.push((s.stealth_address, max_amount.as_u128()))
                }else{
                    result.push((s.stealth_address, 0u128));
                }
                
            }
        }
        Ok(result)
    }

    /// Finds the newest transactions, i.e since the last scan and isolates stealth addresses, ephemeral public keys and viewtags for those transactions.
    /// 
    /// ### Returns 
    /// * `stealth_addresses` - Stealth addresses as a vector of `Address`
    /// * `ephemeral_public_keys` - Ephemeral public keys for corresponding stealth addresses, hex encoding of Kyber ciphertext 
    /// * `view_tags` - A vector of view tags that represent one byte of hash of shared secret, hex encoding of one byte 
    pub async fn fetch_transactions(&self, wallet: &String) -> Result<(Vec<Address>, Vec<String>, Vec<String>), Box<dyn Error>>{
        let provider_string = env::var("PROVIDER_STRING").expect("Provider not set");
        let provider = Provider::<Http>::try_from(provider_string)?; 
        let contract_address:Address = env::var("CONTRACT_ADDRESS").expect("Contract address not set").parse()?; 
        
        // Read latest block accessed 
        let result = self.meta_data_repo.get_meta_data_by_wallet(wallet).await?;
        let latest_block = result.map_or(0, |r| r.last_block);

        // Check if there are any new transactions
        let filter= Filter::new().address(contract_address).from_block(latest_block).to_block(BlockNumber::Latest);
        let logs = provider.get_logs(&filter).await?;
        if logs.len()>0{
            if logs.last().unwrap().block_number.unwrap().as_u64() == latest_block{
                return Ok((vec![], vec![], vec![]));
            }
        }

        // Get new stealth addresses, the ones that appeared since the last scan
        let latest_block = if latest_block == 0{
            0 
        }else{
            latest_block+1
        };


        // Filter the new logs and access relevant information 
        let filter= Filter::new().address(contract_address).from_block(BlockNumber::Number(U64::from(latest_block))).to_block(BlockNumber::Latest);
        let logs = provider.get_logs(&filter).await?;

        let mut stealth_addresses: Vec<Address> = vec![];
        let mut ephemeral_pub_keys: Vec<String> = vec![]; 
        let mut view_tags: Vec<String> = vec![]; 

        for l in &logs{
            let stealth_address: Address= l.topics[2].into();
            let tokens =  decode(&[ethers::abi::ParamType::Bytes, ethers::abi::ParamType::Bytes], &l.data.0).unwrap();
            let eph_pub_key = hex::encode(tokens[0].clone().into_bytes().unwrap());
            let view_tag = hex::encode(tokens[1].clone().into_bytes().unwrap());

            stealth_addresses.push(stealth_address);
            ephemeral_pub_keys.push(eph_pub_key);
            view_tags.push(view_tag);
        }
        
        // Write down the latest block accessed 
        self.meta_data_repo.insert_meta_data_entry(&wallet, logs.last().unwrap().block_number.unwrap().as_u64()).await?;

        Ok((stealth_addresses, ephemeral_pub_keys, view_tags))
    }

}



/// Fetches meta address from ENS.
/// 
/// ### Arguments 
/// * `recipient` - String of ENS domain, e.g. `strahinjap.eth`
/// 
/// ### Returns
/// * `meta_address` - meta address as a json string fetched from ENS 
async fn fetch_meta_address(recipient: &str) -> Result<String, Box<dyn Error>>{
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let meta_address = provider.resolve_field(recipient, "stealth_keys").await?;
    
    Ok(meta_address)
}