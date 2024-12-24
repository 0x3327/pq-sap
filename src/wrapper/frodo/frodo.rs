
#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
use super::consts::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};


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
