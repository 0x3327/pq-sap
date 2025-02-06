use pqc_kyber::{decapsulate, public};
use serde::{Deserialize, Serialize};
use crate::versions::v0::{calculate_stealth_pub_key, calculate_view_tag};

/// Searches ephemeral public key registry and finds potential stealth public key
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `RecipientInputData`
/// 
/// ### Returns
/// A vector of potential stealth addresses 
pub fn scan(json_input_string: String) -> Vec<String>{
    let recipient_input_data: RecipientInputData = serde_json::from_str(&json_input_string).unwrap();
    let ephemeral_pub_key_reg = recipient_input_data.ephemeral_pub_key_reg; 
    let view_tags = recipient_input_data.view_tags;

    let k_bytes: &[u8] = &hex::decode(recipient_input_data.k).expect("Invalid hex"); 
    let v_bytes: &[u8] = &hex::decode(recipient_input_data.v).expect("Invalid hex"); 
    
    let mut stealth_addresses: Vec<String> = vec![]; 
    let k_pub = public(k_bytes); 

    for (i, ephemeral_pub_key) in ephemeral_pub_key_reg.iter().enumerate(){
        let ephemeral_pub_key_bytes: &[u8] = &hex::decode(ephemeral_pub_key).expect("Invalid hex");  
        
        let ss = decapsulate(ephemeral_pub_key_bytes, v_bytes).unwrap();
      
        let view_tag = calculate_view_tag(&ss); 
    
        if hex::decode(&view_tags[i]).expect("Invalid hex")[0] == view_tag{
            let stealth_pub_key = calculate_stealth_pub_key(&ss, &k_pub);  
            
            stealth_addresses.push(hex::encode(stealth_pub_key));
        }   
        
    }
    return stealth_addresses;
} 


#[derive(Deserialize, Serialize)]
pub struct RecipientInputData{
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
    pub fn new(ephemeral_pub_key_reg: Vec<String>, view_tags: Vec<String>, k: String, v: String, version: Version) -> RecipientInputData{
        return RecipientInputData{
            ephemeral_pub_key_reg, view_tags, k, v, version
        }
    }
}