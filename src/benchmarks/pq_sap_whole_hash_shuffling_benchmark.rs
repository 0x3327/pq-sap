use actix_web::rt::time;
use pq_sap::{crypto::{consts::CIPHERTEXT_BYTES, kem::{decaps, encaps, key_pair}}};
use pq_sap::versions::v2::{gen_meta_address, gen_stealth_pub_key}; 
use pqc_dilithium::polyvec::Polyveck;
use rand::{seq::SliceRandom, thread_rng};
use sha2::{Digest, Sha256};
use std::time::Instant;
use sha3::{Digest as KeccakDigest, Keccak256}; 
fn main(){

    println!("{}", pqc_dilithium::K);

    let ns = [5000 ,10000, 20000, 40000, 80000, 1000000];
    let mut res = vec![];
    for n in ns{
        run(n, 10, &mut res);
    }
    for r in res{
        println!("{}", r);
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn std_dev(v: &[f64], mean: f64) -> f64 {
    let variance =
        v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64;
    variance.sqrt()
}

fn run(n: usize, m: usize, res: &mut Vec<String>){
    let mut t = 0u128;
    let mut times: Vec<f64> = vec![];
    for k in 0..m{
        //println!("\x1B[2J\x1B[1;1H");
        //println!("Iteration = {}", k+1);

        // Recipient calculates his meta address (K, V)
        //let (k_pub, _) = key_pair();
        //let (v_pub, v_priv) = key_pair();

        let (V, K) = gen_meta_address();
        let k_pub = K.1; 
        let rho = K.0; 
        let v_pub = V.public; 
        let v_priv = V.secret;

        // Registry containing ephemeral pub keys and corresponding view tags
        let mut registry: Vec<([u8; CIPHERTEXT_BYTES], String)> = vec![];

        // Calculate random existing ephemeral public keys
        for _ in 0..n-1{
            let (v_pub_i, _) = key_pair();
            let (ephemeral_pub_key, ss) = encaps(&v_pub_i);
        
            let view_tag = hash_val(&ss);
            registry.push((ephemeral_pub_key, view_tag)); 
        }

        // Calculate one with recipient's meta-address
        let (ephemeral_pub_key, ss) = encaps(&v_pub);
        let view_tag = hash_val(&ss); 
        registry.push((ephemeral_pub_key, view_tag)); 


        // Shuffle registry
        let mut rng = thread_rng(); 
        //registry.shuffle(&mut rng);


        let start = Instant::now(); 
        for entry in registry.iter(){
            
            let ss = decaps(&entry.0, &v_priv);
    
            let view_tag = hash_val(&ss); 

            
            // If found 
            if entry.1 == view_tag{
                // Calculate stealth pub key
                let P = gen_stealth_pub_key(&ss, &k_pub, &rho); 
                let hash = Keccak256::digest(&P[1..]);
                let mut address_bytes = [0u8; 20];
                address_bytes.copy_from_slice(&hash[12..32]);
                
                
                break; 
            }    
            
        }
        let tmp  = start.elapsed().as_millis();
        times.push(tmp as f64);
    }
    println!("times: {:?}", times);
    let mean = mean(&times);
    let std_dev = std_dev(&times, mean);
    res.push(format!("N = {}, mean = {} ms, standard deviation = {}ms", n, mean, std_dev));

}

pub fn hash_val(x: &[u8]) -> String{
    let mut hasher = Sha256::new(); 
    hasher.update(x);
    hex::encode(hasher.finalize()) 
}