use std::time::Instant;

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
    let Rs = recipient_input_data.Rs; 
    let view_tags = recipient_input_data.view_tags;

    let k_bytes: &[u8] = &hex::decode(recipient_input_data.k).expect("Invalid hex"); 
    let v_bytes: &[u8] = &hex::decode(recipient_input_data.v).expect("Invalid hex"); 
    
    let mut stealth_addresses: Vec<String> = vec![]; 
    let K = public(k_bytes); 

    for (i, R) in Rs.iter().enumerate(){
        let R_bytes: &[u8] = &hex::decode(R).expect("Invalid hex");  
        
        let S = decapsulate(R_bytes, v_bytes).unwrap();
      
        let view_tag = calculate_view_tag(&S); 
    
        if hex::decode(&view_tags[i]).expect("Invalid hex")[0] == view_tag{
            let start = Instant::now();
            let P = calculate_stealth_pub_key(&S, &K);  
            
            stealth_addresses.push(hex::encode(P));
        }   
        
    }
    return stealth_addresses;
} 


#[derive(Deserialize, Serialize)]
pub struct RecipientInputData{
    /// ephemeral public key registry in hex format
    pub Rs: Vec<String>, 
    /// Corresponding view tags in hex format
    pub view_tags: Vec<String>, 
    /// Spending private key in hex format
    k: String, 
    /// Viewing private key in hex format
    v: String,  
}

impl RecipientInputData{
    pub fn new(Rs: Vec<String>, view_tags: Vec<String>, k: String, v: String) -> RecipientInputData{
        return RecipientInputData{
            Rs, view_tags, k, v
        }
    }
}