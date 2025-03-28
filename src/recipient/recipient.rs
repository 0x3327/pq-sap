use std::error::Error;
use ethers::abi::Address;
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use crate::crypto::kem::decaps;
use crate::versions::v1::{calculate_stealth_pub_key, stealth_pub_key_to_address};
use crate::versions::v0::calculate_view_tag;

/// Searches ephemeral public key registry and finds potential stealth public key
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `RecipientInputData`
/// 
/// ### Returns
/// A vector of potential stealth addresses 
pub fn scan(recipient_input_data: RecipientInputData) -> Result<(Vec<[u8; 32]>, Vec<Address>), Box<dyn Error>>{
    let ephemeral_pub_key_reg = recipient_input_data.ephemeral_pub_key_reg; 
    let view_tags = recipient_input_data.view_tags;

  
    let k_pk_bytes = hex::decode(recipient_input_data.k_pk)?;
    let k_pub = PublicKey::from_slice(&k_pk_bytes)?;

    let v_bytes: &[u8] = &hex::decode(recipient_input_data.v)?; 
    
    let mut shared_secrets: Vec<[u8; 32]> = vec![]; 
    let mut stealth_addresses: Vec<Address> = vec![]; 

   
    for (i, ephemeral_pub_key) in ephemeral_pub_key_reg.iter().enumerate(){
        let ephemeral_pub_key_bytes: &[u8] = &hex::decode(ephemeral_pub_key).unwrap();  
        
        let ss = decaps(ephemeral_pub_key_bytes, v_bytes);
      
        let view_tag = calculate_view_tag(&ss); 
    
        if hex::decode(&view_tags[i])?[0] == view_tag{
            let stealth_pub_key = calculate_stealth_pub_key(&ss, &k_pub);  
            let stealth_address = stealth_pub_key_to_address(&stealth_pub_key); 
            if stealth_address == recipient_input_data.stealth_addresses[i]{
                shared_secrets.push(ss);
                stealth_addresses.push(stealth_address);
            }
        }   
        
    }
    Ok((shared_secrets, stealth_addresses))
} 


#[derive(Deserialize, Serialize)]
pub struct RecipientInputData{
    pub stealth_addresses: Vec<Address>, 
    /// ephemeral public key registry in hex format
    pub ephemeral_pub_key_reg: Vec<String>, 
    /// Corresponding view tags in hex format
    pub view_tags: Vec<String>, 
    /// Spending public key in hex format
    pub k_pk: String, 
    /// Viewing private key in hex format
    pub v: String,  
}

#[derive(Deserialize, Serialize)]
pub enum Version{
    V0, 
    V1 
}

impl RecipientInputData{
    pub fn new(ephemeral_pub_key_reg: Vec<String>, view_tags: Vec<String>, stealth_addresses: Vec<Address>, k_pk: String, v: String) -> RecipientInputData{
        return RecipientInputData{
            ephemeral_pub_key_reg, stealth_addresses, view_tags, k_pk, v,
        }
    }
}