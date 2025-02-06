use pq_sap::{crypto::{consts::CIPHERTEXT_BYTES, kem::{decaps, encaps, key_pair}}, versions::v0::calculate_stealth_pub_key};
use rand::{seq::SliceRandom, thread_rng};
use sha2::{Digest, Sha256};
use std::time::Instant;
fn main(){

    let ns = [5000, 10000, 20000, 40000, 80000, 1000000];
    let mut res = vec![];
    for n in ns{
        run(n, 100, &mut res);
    }
    for r in res{
        println!("{}", r);
    }
}

fn run(n: usize, m: usize, res: &mut Vec<String>){
    let mut t = 0u128;
    for k in 0..m{
        println!("\x1B[2J\x1B[1;1H");
        println!("Iteration = {}", k+1);

        // Recipient calculates his meta address (K, V)
        let (k_pub, _) = key_pair();
        let (v_pub, v_priv) = key_pair();

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
        registry.shuffle(&mut rng);


        let start = Instant::now(); 
        for entry in registry.iter(){
            
            let ss = decaps(&entry.0, &v_priv);
    
            let view_tag = hash_val(&ss); 

            // If found 
            if entry.1 == view_tag{
                // Calculate stealth pub key
                let _ = calculate_stealth_pub_key(&ss, &k_pub);
                break; 
            }   
            
        }

        t+=start.elapsed().as_millis();
    }
    res.push(format!("N = {}, {} ms", n,t/(m as u128)));

}

pub fn hash_val(x: &[u8]) -> String{
    let mut hasher = Sha256::new(); 
    hasher.update(x);
    hex::encode(hasher.finalize()) 
}