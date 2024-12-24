#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024"))))]
use pqc_kyber::{KYBER_CIPHERTEXTBYTES, KYBER_K, KYBER_POLYBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES};

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
use crate::wrapper::newhope::consts::*; 

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
use crate::wrapper::frodo::consts::*; 

#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
pub const PUBLIC_KEY_BYTES: usize = KYBER_PUBLICKEYBYTES; 

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
pub const PUBLIC_KEY_BYTES: usize = NEWHOPE_CCAKEM_PUBLICKEYBYTES;  

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub const PUBLIC_KEY_BYTES: usize = CRYPTO_PUBLICKEYBYTES; 

#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]

pub const SECRET_KEY_BYTES: usize = KYBER_SECRETKEYBYTES;

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
pub const SECRET_KEY_BYTES: usize = NEWHOPE_CCAKEM_SECRETKEYBYTES;

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub const SECRET_KEY_BYTES: usize = CRYPTO_SECRETKEYBYTES; 

#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
pub const CIPHERTEXT_BYTES: usize = KYBER_CIPHERTEXTBYTES; 

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
pub const CIPHERTEXT_BYTES: usize = NEWHOPE_CCAKEM_CIPHERTEXTBYTES; 

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub const CIPHERTEXT_BYTES: usize = CRYPTO_CIPHERTEXTBYTES; 

#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024"))
))]
pub const STEALTH_ADDRESS_BYTES: usize = KYBER_K*KYBER_POLYBYTES;

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
pub const STEALTH_ADDRESS_BYTES: usize = NEWHOPE_POLYBYTES;

#[cfg(any( feature = "kyber512", feature = "kyber768", feature = "kyber1024", not(any(feature = "kyber512",feature = "kyber768", feature = "kyber1024", feature = "newhope512",   feature = "newhope1024", feature = "frodo640", feature = "frodo976", feature = "frodo1344"))))]
pub const SS_BYTES: usize = KYBER_SSBYTES;

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
pub const SS_BYTES: usize = NEWHOPE_SYMBYTES;

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub const SS_BYTES: usize = CRYPTO_BYTES; 
