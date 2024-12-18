use libc::c_uchar;

use crate::crypto::consts::*;
use super::{consts::NEWHOPE_N, poly::Poly}; 

extern "C"{
    fn crypto_kem_keypair(pk: *mut c_uchar, sk: *mut c_uchar) -> i32; 
    fn crypto_kem_enc(ct: *mut c_uchar, ss: *mut c_uchar, pk: *const c_uchar) -> i32;   
    fn crypto_kem_dec(ss: *mut c_uchar, ct: *const c_uchar, sk: *const c_uchar) -> i32;    
    fn poly_sample(r: *mut Poly, seed: *const c_uchar, nonce: c_uchar); 
    fn poly_ntt(r: *mut Poly); 
    fn poly_mul_pointwise(r: *mut Poly, a: *const Poly, b: *const Poly); 
    fn poly_add(r: *mut Poly, a: *const Poly, b: *const Poly);  
    fn poly_tobytes(r: *mut c_uchar, p: *const Poly);
    fn decode_pk(pk: *mut Poly, seed: *mut c_uchar, r: *const c_uchar); 
    fn gen_a(a: *mut Poly, seed: *const c_uchar);

}

pub fn rlwe_kem_keypair() -> ([u8; PUBLIC_KEY_BYTES], [u8; SECRET_KEY_BYTES]){
    let mut pk: [c_uchar; PUBLIC_KEY_BYTES] = [0; PUBLIC_KEY_BYTES]; 
    let mut sk: [c_uchar; SECRET_KEY_BYTES] = [0; SECRET_KEY_BYTES];

    unsafe{
        let res = crypto_kem_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()); 
        assert!(res == 0); 
    }

    (pk, sk)
}

pub fn rlwe_kem_encaps(pk: &[u8]) -> ([u8; CIPHERTEXT_BYTES], [u8; SS_BYTES]){ 
    let mut ct: [c_uchar; CIPHERTEXT_BYTES] = [0; CIPHERTEXT_BYTES]; 
    let mut ss: [c_uchar; SS_BYTES] = [0; SS_BYTES];

    unsafe { 
        let res = crypto_kem_enc(ct.as_mut_ptr(), ss.as_mut_ptr(), pk.as_ptr()); 
        assert!(res == 0); 
    }; 

    (ct, ss)
}

pub fn rlwe_kem_decaps(ct: &[u8], sk: &[u8]) -> [u8; SS_BYTES]{
    let mut ss: [c_uchar; SS_BYTES] = [0; SS_BYTES]; 

    unsafe{
        let res = crypto_kem_dec(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr()); 
        assert!(res == 0);
    }

    ss 
}

/// Takes spending key and shared secret and outputs A*XOF(S) + K 
/// 
/// ### Arguments 
/// * `ss` - 32 byte shared secret 
/// * `spending_key` - public key of form (rho, t)  
/// 
/// ### Returns
/// * `r` - RLWE sample
pub fn rlwe_sample(spending_key: &[u8], ss: &[u8]) -> [u8; STEALTH_ADDRESS_BYTES] {
    unsafe{
        let mut b_hat = Poly{coeffs: [0u16; NEWHOPE_N]}; 
        let mut a_hat = Poly{coeffs: [0u16; NEWHOPE_N]};
        let mut s_hat = Poly{coeffs: [0u16; NEWHOPE_N]}; 
        let mut u_hat = Poly{coeffs: [0u16; NEWHOPE_N]}; 

        let mut seed = [0; SS_BYTES]; 
        let mut r = [0; STEALTH_ADDRESS_BYTES]; 

        decode_pk(&mut b_hat, seed.as_mut_ptr(), spending_key.as_ptr()); 
        gen_a(&mut a_hat, seed.as_ptr());

        poly_sample(&mut s_hat, ss.as_ptr(), 0);  
    
        poly_ntt(&mut s_hat); 

        poly_mul_pointwise(&mut u_hat,  &a_hat, &s_hat); 
        poly_add(&mut u_hat, &u_hat, &b_hat);

        poly_tobytes(r.as_mut_ptr(), &u_hat); 
        
        r
    }
}
