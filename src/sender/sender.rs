use ethers::abi::{Address, Bytes};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use crate::versions::v1::{sender_computes_stealth_pub_key_and_viewtag, stealth_pub_key_to_address};




/// Takes stealth meta address M = (K,V) as `SenderInputData` calculates stealth public key P, ephemeral public key R and view tag 
/// 
/// ### Arguments 
/// * `json_input_string` - String of json corresponding to `SenderInputData`
/// 
/// ### Returns
/// * `ephemeral_pub_key` - ephemeral public key(ciphertext from `KyberKEM`)
/// * `stealth_pub_key` - stealth public key 
/// * `view_tag` - view tag that is a first byte of h(S), where S is shared secret
pub fn send(json_input_string: &String) -> (Address, Bytes, Bytes){
    let sender_input_data: SenderInputData = serde_json::from_str(json_input_string).unwrap();
    let byte_len = sender_input_data.spending_pk.len()/2;

    let mut k_pub_bytes = vec![0u8; byte_len]; 
    hex::decode_to_slice(sender_input_data.spending_pk, &mut k_pub_bytes).expect("Failed to decode hex");
    
    let k_pub = PublicKey::from_slice(&k_pub_bytes).unwrap();
    let v_pub_bytes: &[u8] = &hex::decode(sender_input_data.viewing_pk).expect("Invalid hex");  

    let (stealth_pub_key, ephemeral_pub_key, view_tag) = sender_computes_stealth_pub_key_and_viewtag(v_pub_bytes, &k_pub); 

    let stealth_address = stealth_pub_key_to_address(&stealth_pub_key);
    
    (stealth_address, ephemeral_pub_key.into(), [view_tag].into())
}

#[derive(Deserialize, Serialize)]
pub struct SenderInputData{
    /// Recipient's public spending key in hex format
    pub spending_pk: String, 

    /// Recipient's public viewing key in hex format
    pub viewing_pk: String, 

}
