use serde::{Deserialize, Serialize};

use crate::versions::v0::sender_computes_stealth_pub_key_and_viewtag;

/// Takes stealth meta address M = (K,V) as `SenderInputData` calculates stealth public key P, ephemeral public key R and view tag 
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `SenderInputData`
/// 
/// ### Returns
/// * `R` - ephemeral public key(ciphertext from `KyberKEM`)
/// * `P` - stealth public key 
/// * `view_tag` - view tag that is a first byte of h(S), where S is shared secret
pub fn send(json_input_string: &String) -> (String, String, String){
    let sender_input_data: SenderInputData = serde_json::from_str(json_input_string).unwrap();

    let K_bytes: &[u8] = &hex::decode(sender_input_data.K).expect("Invalid hex"); 
    let V_bytes: &[u8] = &hex::decode(sender_input_data.V).expect("Invalid hex");  

    let (P, R, view_tag) = sender_computes_stealth_pub_key_and_viewtag(V_bytes, K_bytes).unwrap(); 

    (hex::encode(R), hex::encode(P), hex::encode([view_tag]))
}

#[derive(Deserialize, Serialize)]
pub struct SenderInputData{
    /// Recipient's public spending key in hex format
    pub K: String, 

    /// Recipient's public viewing key in hex format
    pub V: String, 
}
