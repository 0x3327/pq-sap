#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
use pqc_kyber::{keypair, decapsulate, encapsulate};

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
use crate::wrapper::frodo::frodo::{lwe_kem_decaps, lwe_kem_encaps, lwe_kem_keypair};

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
use crate::wrapper::newhope::newhope::{rlwe_kem_keypair, rlwe_kem_encaps, rlwe_kem_decaps};

use super::consts::{CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SS_BYTES};

pub fn key_pair() -> ([u8; PUBLIC_KEY_BYTES], [u8; SECRET_KEY_BYTES]) {
    #[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
    {   
        let mut rng = rand::thread_rng(); 
        let keypair = keypair(&mut rng).expect("Error in generating keys"); 
        (keypair.public, keypair.secret)
    }
    #[cfg(any(feature = "newhope1024", feature = "newhope512"))]
    return rlwe_kem_keypair();
    
    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    lwe_kem_keypair()

}

pub fn encaps(pk: &[u8]) -> ([u8; CIPHERTEXT_BYTES], [u8; SS_BYTES]){
    #[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
    {
    let mut rng = rand::thread_rng(); 
    encapsulate(&pk, &mut rng).expect("Error encapsulating.")
    } 

    #[cfg(any(feature = "newhope1024", feature = "newhope512"))]
    return rlwe_kem_encaps(&pk);

    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    lwe_kem_encaps(&pk)
    

} 

pub fn decaps(ct: &[u8], sk: &[u8]) -> [u8; SS_BYTES]{
    #[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
    {
    let ss = decapsulate(ct, sk).expect("Cannot decapsulate"); 
    ss 
    }
    #[cfg(any(feature = "newhope1024", feature = "newhope512"))]
    return rlwe_kem_decaps(ct, sk);

    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    lwe_kem_decaps(ct, sk)
}