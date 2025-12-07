use pqc_dilithium::Keypair as DilithiumKeypair;
use pqc_dilithium::L_U16;
use pqc_dilithium::packing::pack_pk;
use pqc_dilithium::polyvec::Polyveck;
use pqc_dilithium::polyvec::polyvec_matrix_expand;
use pqc_dilithium::polyvec::polyvec_matrix_pointwise_montgomery;
use pqc_dilithium::polyvec::polyveck_add;
use pqc_dilithium::polyvec::polyveck_caddq;
use pqc_dilithium::polyvec::polyveck_invntt_tomont;
use pqc_dilithium::polyvec::polyveck_power2round;
use pqc_dilithium::polyvec::polyveck_reduce;
use pqc_dilithium::polyvec::polyveck_uniform_eta;
use pqc_dilithium::polyvec::polyvecl_ntt;
use pqc_kyber::keypair;
use pqc_kyber::Keypair as KyberKeypair; 
use pqc_dilithium::PUBLICKEYBYTES;
use pqc_dilithium::SEEDBYTES;
use pqc_dilithium::K; 
use pqc_dilithium::fips202::shake256; 
use pqc_dilithium::polyvec::{Polyvecl, polyvecl_uniform_eta}; 
use pqc_dilithium::randombytes::randombytes; 

pub const SEED_BYTES: usize = 64; 

pub fn gen_meta_address() -> (KyberKeypair, ([u8; 32], Polyveck, Polyvecl)){
    let mut rng = rand::thread_rng(); 
    let kyber_keypair = keypair(&mut rng).expect("Error in generating keys"); 

    let mut mat_seed = [0u8; 32];
    let mut vec_seed = [0u8; 64]; 
    randombytes(&mut mat_seed, 32);
    randombytes(&mut vec_seed, 64);

    let mut e = Polyveck::default(); 
    let mut k = Polyvecl::default();
    polyvecl_uniform_eta(&mut k, &vec_seed, 0);
    polyveck_uniform_eta(&mut e, &vec_seed, L_U16);

    let mut mat = [Polyvecl::default(); K];
    polyvec_matrix_expand(&mut mat, &mat_seed);

    let mut k_pub = Polyveck::default(); 
    // A*s
    let mut k_ntt = k; 
    polyvecl_ntt(&mut k_ntt); 
    polyvec_matrix_pointwise_montgomery(&mut k_pub, &mat, &k_ntt);
    polyveck_reduce(&mut k_pub);
    polyveck_invntt_tomont(&mut k_pub);

    // A*s + K 
    polyveck_add(&mut k_pub, &e);
    polyveck_caddq(&mut k_pub);
    
   
    (kyber_keypair, (mat_seed, k_pub, k))
}

pub fn gen_stealth_pub_key(ss: &[u8; 32], k_pub: &Polyveck, rho: &[u8; SEEDBYTES]) ->[u8; PUBLICKEYBYTES]{
    let mut seed_ss = [0u8; SEED_BYTES];
    shake256(&mut seed_ss, SEED_BYTES, ss, ss.len());

    let mut s = Polyvecl::default();
    polyvecl_uniform_eta(&mut s, &seed_ss, 0);



    // expand A 
    let mut mat = [Polyvecl::default(); K];
    polyvec_matrix_expand(&mut mat, rho);


    let mut stealth_pub_key = Polyveck::default(); 

    // A*s
    polyvecl_ntt(&mut s); 
    polyvec_matrix_pointwise_montgomery(&mut stealth_pub_key, &mat, &s);
    polyveck_reduce(&mut stealth_pub_key);
    polyveck_invntt_tomont(&mut stealth_pub_key);

    // A*s + K 
    polyveck_add(&mut stealth_pub_key, &k_pub);
    polyveck_caddq(&mut stealth_pub_key);

    let mut t0 = Polyveck::default();
    polyveck_power2round(&mut stealth_pub_key, &mut t0);

    let mut stealth_pub_key_bytes: [u8; PUBLICKEYBYTES] = [0u8; PUBLICKEYBYTES];
    pack_pk(&mut stealth_pub_key_bytes, rho, &stealth_pub_key);

    return stealth_pub_key_bytes; 

}

