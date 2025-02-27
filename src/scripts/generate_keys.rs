use colored::Colorize;
use pq_sap::crypto::kem::key_pair;
use rand::rngs::OsRng;
use secp256k1::Secp256k1;
use serde_json::json;

fn main(){
    // generated spedning and viewing key in JSON format to register on ENS 
    let secp = Secp256k1::new(); 
    let (k_priv, k_pub) = secp.generate_keypair(&mut OsRng);  
    let (v_pub, v_priv) = key_pair();  

    let ens_info_json = json!({"viewing_pk": hex::encode(v_pub), "spending_pk": hex::encode(k_pub.serialize_uncompressed())});
    let private_key_info_json = json!({"viewing_sk": hex::encode(v_priv), "spending_sk": hex::encode(k_priv.secret_bytes())});
  
    println!("Public keys for ENS: ");
    println!("{}", serde_json::to_string_pretty(&ens_info_json).unwrap());

    println!("Private keys ({}: don't publish on ENS)","WARNING".red());
    println!("{}", serde_json::to_string_pretty(&private_key_info_json).unwrap());
}