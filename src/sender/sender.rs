use serde::{Deserialize, Serialize};

use crate::versions::v0::sender_computes_stealth_pub_key_and_viewtag;

/// Takes stealth meta address M = (K,V) as `SenderInputData` calculates stealth public key P, ephemeral public key R and view tag 
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `SenderInputData`
/// 
/// ### Returns
/// * `ephemeral_pub_key` - ephemeral public key(ciphertext from `KyberKEM`)
/// * `stealth_pub_key` - stealth public key 
/// * `view_tag` - view tag that is a first byte of h(S), where S is shared secret
pub fn send(json_input_string: &String) -> (String, String, String){
    let sender_input_data: SenderInputData = serde_json::from_str(json_input_string).unwrap();

    let k_pub_bytes: &[u8] = &hex::decode(sender_input_data.k_pub).expect("Invalid hex"); 
    let v_pub_bytes: &[u8] = &hex::decode(sender_input_data.v_pub).expect("Invalid hex");  

    let (stealth_pub_key, ephemeral_pub_key, view_tag) = sender_computes_stealth_pub_key_and_viewtag(v_pub_bytes, k_pub_bytes).unwrap(); 

    (hex::encode(ephemeral_pub_key), hex::encode(stealth_pub_key), hex::encode([view_tag]))
}

#[derive(Deserialize, Serialize)]
pub struct SenderInputData{
    /// Recipient's public spending key in hex format
    pub k_pub: String, 

    /// Recipient's public viewing key in hex format
    pub v_pub: String, 
}
