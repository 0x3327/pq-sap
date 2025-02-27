use ethers::abi::Address;
use pqc_kyber::decapsulate;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use crate::versions::v1::{calculate_stealth_pub_key, stealth_pub_key_to_address};
use crate::versions::v0::calculate_view_tag;

/// Searches ephemeral public key registry and finds potential stealth public key
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `RecipientInputData`
/// 
/// ### Returns
/// A vector of potential stealth addresses 
pub fn scan(recipient_input_data: RecipientInputData) -> (Vec<[u8; 32]>, Vec<Address>){
    let ephemeral_pub_key_reg = recipient_input_data.ephemeral_pub_key_reg; 
    let view_tags = recipient_input_data.view_tags;

    let byte_len = recipient_input_data.k.len()/2;
    let mut k_priv_bytes = vec![0u8; byte_len]; 
    hex::decode_to_slice(recipient_input_data.k, &mut k_priv_bytes).expect("Failed to decode hex");

    let k_priv = SecretKey::from_slice(&k_priv_bytes).unwrap();

    let v_bytes: &[u8] = &hex::decode(recipient_input_data.v).expect("Invalid hex"); 
    
    let mut shared_secrets: Vec<[u8; 32]> = vec![]; 
    let mut stealth_addresses: Vec<Address> = vec![]; 

    let k_pub = PublicKey::from_secret_key(&Secp256k1::new(), &k_priv);

    for (i, ephemeral_pub_key) in ephemeral_pub_key_reg.iter().enumerate(){
        let ephemeral_pub_key_bytes: &[u8] = &hex::decode(ephemeral_pub_key).expect("Invalid hex");  
        
        let ss = decapsulate(ephemeral_pub_key_bytes, v_bytes).unwrap();
      
        let view_tag = calculate_view_tag(&ss); 
    
        if hex::decode(&view_tags[i]).expect("Invalid hex")[0] == view_tag{
            let stealth_pub_key = calculate_stealth_pub_key(&ss, &k_pub);  
            let stealth_address = stealth_pub_key_to_address(&stealth_pub_key); 
            if stealth_address == recipient_input_data.stealth_addresses[i]{
                shared_secrets.push(ss);
                stealth_addresses.push(stealth_address);
            }
        }   
        
    }
    (shared_secrets, stealth_addresses)
} 


#[derive(Deserialize, Serialize)]
pub struct RecipientInputData{
    pub stealth_addresses: Vec<Address>, 
    /// ephemeral public key registry in hex format
    pub ephemeral_pub_key_reg: Vec<String>, 
    /// Corresponding view tags in hex format
    pub view_tags: Vec<String>, 
    /// Spending private key in hex format
    k: String, 
    /// Viewing private key in hex format
    v: String,  
    /// Version of protocol 
    version: Version
}

#[derive(Deserialize, Serialize)]
pub enum Version{
    V0, 
    V1 
}

impl RecipientInputData{
    pub fn new(ephemeral_pub_key_reg: Vec<String>, view_tags: Vec<String>, stealth_addresses: Vec<Address>, k: String, v: String, version: Version) -> RecipientInputData{
        return RecipientInputData{
            ephemeral_pub_key_reg, stealth_addresses, view_tags, k, v, version,
        }
    }
}