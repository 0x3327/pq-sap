use std::mem;

use crate::crypto::consts::STEALTH_ADDRESS_BYTES;

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
use super::consts::*;


use libc::c_longlong;
#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
use libc::c_uchar;

extern "C"{
    //int crypto_kem_keypair_Frodo640(unsigned char *pk, unsigned char *sk);
    #[cfg(feature = "frodo640")]
    fn crypto_kem_keypair_Frodo640(pk: *mut c_uchar, sk: *mut c_uchar) -> i32; 
    #[cfg(feature = "frodo640")]
    fn crypto_kem_enc_Frodo640(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar) -> i32;
    #[cfg(feature = "frodo640")]
    fn crypto_kem_dec_Frodo640(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar) -> i32;
    

    #[cfg(feature = "frodo976")]
    fn crypto_kem_keypair_Frodo976(pk: *mut c_uchar, sk: *mut c_uchar) -> i32; 
    #[cfg(feature = "frodo976")]
    fn crypto_kem_enc_Frodo976(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar) -> i32;
    #[cfg(feature = "frodo976")]
    fn crypto_kem_dec_Frodo976(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar) -> i32;

    #[cfg(feature = "frodo1344")]
    fn crypto_kem_keypair_Frodo1344(pk: *mut c_uchar, sk: *mut c_uchar) -> i32; 
    #[cfg(feature = "frodo1344")]
    fn crypto_kem_enc_Frodo1344(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar) -> i32;
    #[cfg(feature = "frodo1344")]
    fn crypto_kem_dec_Frodo1344(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar) -> i32;

    fn shake128(output: *mut c_uchar, outlen: c_longlong, input: *const c_uchar, inlen: c_longlong);
    fn frodo_sample_n(s: *mut u16, n: usize);
    fn frodo_mul_add_as_plus_e(out: *mut u16, s: *const u16, e: *const u16, seed_A: *const u8) -> i32;
    fn frodo_unpack(out: *mut u16, outlen: usize, r#in: *const c_uchar, inlen: usize, lsb: c_uchar);
    fn frodo_pack(out: *mut c_uchar, outlen: usize, r#in: *const u16, inlen: usize, lsb: c_uchar);

}

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub fn lwe_kem_keypair() -> ([u8; CRYPTO_PUBLICKEYBYTES], [u8; CRYPTO_SECRETKEYBYTES]){
    let mut pk: [c_uchar; CRYPTO_PUBLICKEYBYTES] = [0; CRYPTO_PUBLICKEYBYTES]; 
    let mut sk: [c_uchar; CRYPTO_SECRETKEYBYTES] = [0; CRYPTO_SECRETKEYBYTES];

    unsafe{
        #[cfg(feature = "frodo640")]
        crypto_kem_keypair_Frodo640(pk.as_mut_ptr(), sk.as_mut_ptr());

        #[cfg(feature = "frodo976")]
        crypto_kem_keypair_Frodo976(pk.as_mut_ptr(), sk.as_mut_ptr());

        #[cfg(feature = "frodo1344")]
        crypto_kem_keypair_Frodo1344(pk.as_mut_ptr(), sk.as_mut_ptr());
        
    }
    (pk, sk)
}

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub fn lwe_kem_encaps(pk: &[u8]) ->([u8; CRYPTO_CIPHERTEXTBYTES], [u8; CRYPTO_BYTES]){
    let mut ct: [c_uchar; CRYPTO_CIPHERTEXTBYTES] = [0; CRYPTO_CIPHERTEXTBYTES]; 
    let mut ss: [c_uchar; CRYPTO_BYTES] = [0; CRYPTO_BYTES];

    unsafe{
        #[cfg(feature = "frodo640")]
        crypto_kem_enc_Frodo640(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr());
        #[cfg(feature = "frodo976")]
        crypto_kem_enc_Frodo976(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr());
        #[cfg(feature = "frodo1344")]
        crypto_kem_enc_Frodo1344(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr());
    }

    (ct, ss)
}

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
pub fn lwe_kem_decaps(ct: &[u8], sk: &[u8]) -> [u8; CRYPTO_BYTES] {
    let mut ss: [c_uchar; CRYPTO_BYTES] = [0; CRYPTO_BYTES];

    unsafe{
        #[cfg(feature = "frodo640")]
        crypto_kem_dec_Frodo640(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr());
        #[cfg(feature = "frodo976")]
        crypto_kem_dec_Frodo976(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr());
        #[cfg(feature = "frodo1344")]
        crypto_kem_dec_Frodo1344(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr());
    }

    ss 
}

pub fn lwe_sample(spending_key: &[u8],ss: &[u8]) -> [u8; STEALTH_ADDRESS_BYTES]{
    let mut s_poly = [0u16; (2*PARAMS_N+PARAMS_NBAR)*PARAMS_NBAR];
    let mut stealth_pub_key_poly = [0u16; PARAMS_N*PARAMS_NBAR];
    let mut stealth_pub_key = [0u8; STEALTH_ADDRESS_BYTES];
    let mut spending_key_b = [0u16; PARAMS_N*PARAMS_NBAR];
    
    unsafe{
        // Use shake128 to derive polynomial secret S
        shake128(s_poly.as_mut_ptr() as *mut c_uchar, ((2*PARAMS_N+PARAMS_NBAR)*PARAMS_NBAR*mem::size_of::<u16>()) as i64, ss.as_ptr(), CRYPTO_BYTES as i64);  
        for elem in s_poly.iter_mut() {
        *elem = u16::from_le(*elem);
        }
        frodo_sample_n(s_poly.as_mut_ptr(), PARAMS_N*PARAMS_NBAR);
    
        // seed to derive matrix A from spending key K 
        let seed_a = &spending_key[0..BYTES_SEED_A];
    
        // unpack to get K
        let pk_b = &spending_key[BYTES_SEED_A..];
        frodo_unpack(spending_key_b.as_mut_ptr(), PARAMS_N*PARAMS_NBAR, pk_b.as_ptr(), CRYPTO_PUBLICKEYBYTES - BYTES_SEED_A, PARAMS_LOGQ as u8);

        // calculate A*S + K, A is derived by sending seed 
        frodo_mul_add_as_plus_e(stealth_pub_key_poly.as_mut_ptr(), s_poly.as_ptr(), spending_key_b.as_ptr(), seed_a.as_ptr());

        // pack stealth public key into u8 
        frodo_pack(stealth_pub_key.as_mut_ptr(), STEALTH_ADDRESS_BYTES, stealth_pub_key_poly.as_ptr(), PARAMS_N*PARAMS_NBAR, PARAMS_LOGQ as u8);
    }

    stealth_pub_key


}