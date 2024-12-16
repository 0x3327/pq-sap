use libc::c_uchar;

use crate::utils::consts::{NEWHOPE_CCAKEM_CIPHERTEXTBYTES, NEWHOPE_CCAKEM_PUBLICKEYBYTES, NEWHOPE_CCAKEM_SECRETKEYBYTES, NEWHOPE_SYMBYTES}; 

extern "C"{
    fn crypto_kem_keypair(pk: *mut c_uchar, sk: *mut c_uchar) -> i32; 
    fn crypto_kem_enc(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar) -> i32;   
    fn crypto_kem_dec(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar) -> i32; 
}

pub fn rlwe_kem_keypair() -> ([u8; NEWHOPE_CCAKEM_PUBLICKEYBYTES], [u8; NEWHOPE_CCAKEM_SECRETKEYBYTES]){
    let mut pk: [c_uchar; NEWHOPE_CCAKEM_PUBLICKEYBYTES] = [0; NEWHOPE_CCAKEM_PUBLICKEYBYTES]; 
    let mut sk: [c_uchar; NEWHOPE_CCAKEM_SECRETKEYBYTES] = [0; NEWHOPE_CCAKEM_SECRETKEYBYTES];

    unsafe{
        let res = crypto_kem_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()); 
        assert!(res == 0); 
    }

    (pk, sk)
}

pub fn rlwe_kem_encaps(pk: &[u8]) -> ([u8; NEWHOPE_CCAKEM_CIPHERTEXTBYTES], [u8; NEWHOPE_SYMBYTES]){ 
    let mut ct: [c_uchar; NEWHOPE_CCAKEM_CIPHERTEXTBYTES] = [0; NEWHOPE_CCAKEM_CIPHERTEXTBYTES]; 
    let mut ss: [c_uchar; NEWHOPE_SYMBYTES] = [0; NEWHOPE_SYMBYTES];

    unsafe { 
        let res = crypto_kem_enc(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr()); 
        assert!(res == 0); 
    }; 

    (ct, ss)
}

pub fn rlwe_kem_decaps(ct: &[u8; NEWHOPE_CCAKEM_CIPHERTEXTBYTES], sk: &[u8; NEWHOPE_CCAKEM_SECRETKEYBYTES]) -> [u8; NEWHOPE_SYMBYTES]{
    let mut ss: [c_uchar; NEWHOPE_SYMBYTES] = [0; NEWHOPE_SYMBYTES]; 

    unsafe{
        let res = crypto_kem_dec(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr()); 
        assert!(res == 0);
    }

    ss 
}

