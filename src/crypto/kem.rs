#[cfg(feature = "mlwe")] 
use pqc_kyber::{keypair, decapsulate, encapsulate};

#[cfg(feature = "rlwe")]
use crate::wrapper::newhope::{rlwe_kem_keypair, rlwe_kem_encaps, rlwe_kem_decaps};

use super::consts::{CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SS_BYTES};

pub fn key_pair() -> ([u8; PUBLIC_KEY_BYTES], [u8; SECRET_KEY_BYTES]) {
    #[cfg(feature = "mlwe")] 
    {   
        let mut rng = rand::thread_rng(); 
        let keypair = keypair(&mut rng).expect("Error in generating keys"); 
        (keypair.public, keypair.secret)
    }
    #[cfg(feature = "rlwe")]
    rlwe_kem_keypair()


}

pub fn encaps(pk: &[u8]) -> ([u8; CIPHERTEXT_BYTES], [u8; SS_BYTES]){
    #[cfg(feature = "mlwe")] 
    {
    let mut rng = rand::thread_rng(); 
    encapsulate(&pk, &mut rng).expect("Error encapsulating.")
    } 
    #[cfg(feature = "rlwe")]
    rlwe_kem_encaps(&pk)

} 

pub fn decaps(ct: &[u8], sk: &[u8]) -> [u8; SS_BYTES]{
    #[cfg(feature = "mlwe")] 
    {
    let ss = decapsulate(ct, sk).expect("Cannot decapsulate"); 
    ss 
    }
    #[cfg(feature = "rlwe")]
    rlwe_kem_decaps(ct, sk)
}