use std::error::Error;

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
pub fn send(sender_input_data: &SenderInputData) -> Result<(Address, Bytes, Bytes), Box<dyn Error>>{
    
    let k_pub_bytes = hex::decode(&sender_input_data.spending_pk)?;

    let k_pub = PublicKey::from_slice(&k_pub_bytes)?;
    let v_pub_bytes: &[u8] = &hex::decode(&sender_input_data.viewing_pk)?;  

    let (stealth_pub_key, ephemeral_pub_key, view_tag) = sender_computes_stealth_pub_key_and_viewtag(v_pub_bytes, &k_pub); 

    let stealth_address = stealth_pub_key_to_address(&stealth_pub_key);
    
    Ok((stealth_address, ephemeral_pub_key.into(), [view_tag].into()))
}

#[derive(Deserialize, Serialize)]
pub struct SenderInputData{
    /// Recipient's public spending key in hex format
    pub spending_pk: String, 

    /// Recipient's public viewing key in hex format
    pub viewing_pk: String, 

}
