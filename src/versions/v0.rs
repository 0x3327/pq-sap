use pqc_kyber::reference::indcpa::{gen_a, unpack_pk};
use pqc_kyber::reference::poly::{poly_getnoise_eta1, poly_tomont};
use pqc_kyber::{decapsulate, encapsulate, KyberError, KYBER_CIPHERTEXTBYTES, KYBER_K, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, KYBER_SYMBYTES};
use pqc_kyber::reference::polyvec::*;
use pqc_kyber::KYBER_POLYBYTES;
use sha2::{Digest, Sha256};

pub fn recipient_computes_stealth_pub_key(K: &[u8; KYBER_PUBLICKEYBYTES], R: &[u8; KYBER_CIPHERTEXTBYTES], v: &[u8; KYBER_SECRETKEYBYTES]) -> Result<[u8; KYBER_K*KYBER_POLYBYTES], KyberError>{
    // Calculate shared secret 
    let S = decapsulate(R, v)?;  

    // Calculate stealth public key 
    let P = calculate_stealth_pub_key(&S, K); 
    
    Ok((P))
} 

pub fn sender_computes_stealth_pub_key_and_viewtag(V: &[u8], K: &[u8]) -> Result<([u8; KYBER_K*KYBER_POLYBYTES], [u8; KYBER_CIPHERTEXTBYTES], u8), KyberError>{
    assert!(V.len() == KYBER_PUBLICKEYBYTES); 
    assert!(K.len() == KYBER_PUBLICKEYBYTES); 
    
    let mut rng = rand::thread_rng(); 

    // Calculate shared secret and ciphertext used in Kyber
    let (R, S) = encapsulate(V, &mut rng)?; 
   
    // Calculate stealth public key 
    let P = calculate_stealth_pub_key(&S, K);

    // Calculate view tag by taking first byte of hash 
    let view_tag = calculate_view_tag(&S);

    Ok((P, R, view_tag))
}

pub fn calculate_view_tag(S: &[u8]) -> u8{
    assert!(S.len() == KYBER_SSBYTES);

    let mut hasher = Sha256::new(); 
    hasher.update(S);
    hasher.finalize()[0]
}

pub fn calculate_stealth_pub_key(S: &[u8], K: &[u8]) -> [u8; KYBER_K*KYBER_POLYBYTES]{
   
    // Get the encryption of spending key and seed used to derive matrix A
    let (mut pkpv, mut skpv)  = (Polyvec::new(), Polyvec::new());
    let mut public_seed = [0u8; KYBER_SYMBYTES];
    unpack_pk(&mut pkpv, &mut public_seed, K);

    // derive matrix A 
    let mut a = [Polyvec::new(); KYBER_K];
    gen_a(&mut a, &public_seed); 
    
    // Convert shared secret to polynomial 
    let mut nonce = 0; 
    for i in 0..KYBER_K {
        poly_getnoise_eta1(&mut skpv.vec[i], &S, nonce);
        nonce += 1;
    }

    // Compute A*S + K 
    let mut p_poly = Polyvec::new();
    for i in 0..KYBER_K{
        polyvec_basemul_acc_montgomery(&mut p_poly.vec[i], &a[i], &skpv);
        poly_tomont(&mut p_poly.vec[i]);
    }
    polyvec_add(&mut p_poly, &pkpv); 
    polyvec_reduce(&mut p_poly); 

    // Convert stealth public key from polynomial to bytes 
    let mut P = [0u8; KYBER_K*KYBER_POLYBYTES];
    polyvec_tobytes(&mut P, &p_poly);
    
    P 
}