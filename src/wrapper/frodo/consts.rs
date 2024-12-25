#[cfg(feature = "frodo640")]
pub const CRYPTO_SECRETKEYBYTES: usize = 19888;
#[cfg(feature = "frodo976")]
pub const CRYPTO_SECRETKEYBYTES: usize = 31296;
#[cfg(feature = "frodo1344")]
pub const CRYPTO_SECRETKEYBYTES: usize = 43088;


#[cfg(feature = "frodo640")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 9616;
#[cfg(feature = "frodo976")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 15632;
#[cfg(feature = "frodo1344")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 21520;

#[cfg(feature = "frodo640")]
pub const CRYPTO_BYTES: usize = 16;
#[cfg(feature = "frodo976")]
pub const CRYPTO_BYTES: usize = 24;
#[cfg(feature = "frodo1344")]
pub const CRYPTO_BYTES: usize = 32;

#[cfg(feature = "frodo640")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 9752;
#[cfg(feature = "frodo976")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 15792;
#[cfg(feature = "frodo1344")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 21696;


#[cfg(feature = "frodo640")]
pub const PARAMS_N: usize = 640;
#[cfg(feature = "frodo976")]
pub const PARAMS_N: usize = 976;
#[cfg(feature = "frodo1344")]
pub const PARAMS_N: usize = 1344;

#[cfg(feature = "frodo640")]
pub const PARAMS_LOGQ: usize = 15;
#[cfg(feature = "frodo976")]
pub const PARAMS_LOGQ: usize = 16;
#[cfg(feature = "frodo1344")]
pub const PARAMS_LOGQ: usize = 16;

pub const PARAMS_NBAR: usize = 8; 
pub const BYTES_SEED_A: usize = 16; 