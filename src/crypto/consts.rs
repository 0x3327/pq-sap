#[cfg(feature = "mlwe")] 
use pqc_kyber::{KYBER_CIPHERTEXTBYTES, KYBER_K, KYBER_POLYBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES};

#[cfg(feature = "rlwe")]
use crate::wrapper::consts::*; 

#[cfg(feature = "mlwe")] 
pub const PUBLIC_KEY_BYTES: usize = KYBER_PUBLICKEYBYTES; 

#[cfg(feature = "rlwe")] 
pub const PUBLIC_KEY_BYTES: usize = NEWHOPE_CCAKEM_PUBLICKEYBYTES;  


#[cfg(feature = "mlwe")] 
pub const SECRET_KEY_BYTES: usize = KYBER_SECRETKEYBYTES;

#[cfg(feature = "rlwe")] 
pub const SECRET_KEY_BYTES: usize = NEWHOPE_CCAKEM_SECRETKEYBYTES;

#[cfg(feature = "mlwe")]
pub const CIPHERTEXT_BYTES: usize = KYBER_CIPHERTEXTBYTES; 

#[cfg(feature = "rlwe")]
pub const CIPHERTEXT_BYTES: usize = NEWHOPE_CCAKEM_CIPHERTEXTBYTES; 

#[cfg(feature = "mlwe")]
pub const STEALTH_ADDRESS_BYTES: usize = KYBER_K*KYBER_POLYBYTES;

#[cfg(feature = "rlwe")]
pub const STEALTH_ADDRESS_BYTES: usize = NEWHOPE_POLYBYTES;

#[cfg(feature = "mlwe")]
pub const SS_BYTES: usize = KYBER_SSBYTES;

#[cfg(feature = "rlwe")]
pub const SS_BYTES: usize = NEWHOPE_SYMBYTES;