use pq_sap::{crypto::{consts::CIPHERTEXT_BYTES, kem::{decaps, encaps, key_pair}}}; 
use pq_sap::versions::v2::{gen_meta_address, gen_stealth_pub_key};
//use rand::{seq::SliceRandom, thread_rng};
use sha2::{Digest, Sha256};
use std::time::Instant;

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn std_dev(v: &[f64], mean: f64) -> f64 {
    let variance =
        v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64;
    variance.sqrt()
}

fn main(){

    let ns = [5000, 10000, 20000, 40000, 80000, 1000000];
    let mut res = vec![];
    let mut res2: Vec<String> = vec![];
    let mut res3: Vec<String> = vec![]; 

    for n in ns{
        run(n, 10, &mut res, &mut res2, &mut res3);
    }

    println!("-----Results while using a whole hash as the view tag-----");
    for r in res{
        println!("{}", r);
    }

    println!("-----Results while using one byte of a hash as the view tag-----");
    for r in res2{
        println!("{}", r);
    }

    println!("-----Results without using the viewtag"); 
    for r in res3{
        println!("{}", r);
    }

    
}

fn run(n: usize, m: usize, res: &mut Vec<String>, res2: &mut Vec<String>, res3: &mut Vec<String>){
    let mut times1: Vec<f64> = vec![];
    let mut times2: Vec<f64> = vec![];
    let mut times3: Vec<f64> = vec![];
    

    for k in 0..m{
        println!("\x1B[2J\x1B[1;1H");
        println!("Iteration = {}, announcements = {}", k+1, n);

        // Recipient calculates his meta address (K, V)
        let (V, K) = gen_meta_address();
        let k_pub = K.1; 
        let rho = K.0; 
        let v_pub = V.public; 
        let v_priv = V.secret;

        // Registry containing ephemeral pub keys and corresponding view tags
        let mut registry: Vec<([u8; CIPHERTEXT_BYTES], String, String)> = vec![];

        // Calculate random existing ephemeral public keys
        for _ in 0..n-1{
            let (v_pub_i, _) = key_pair();
            let (ephemeral_pub_key, ss) = encaps(&v_pub_i);
        
            let view_tag = hash_val(&ss);
            let view_tag_one_byte = hash_val_one_byte(&ss);
            registry.push((ephemeral_pub_key, view_tag, view_tag_one_byte)); 
        }

        // Calculate one with recipient's meta-address
        let (ephemeral_pub_key, ss) = encaps(&v_pub);
        let view_tag = hash_val(&ss);  
        let view_tag_one_byte = hash_val_one_byte(&ss);
        registry.push((ephemeral_pub_key, view_tag, view_tag_one_byte)); 


        // Shuffle registry
        // let mut rng = thread_rng(); 
        // registry.shuffle(&mut rng);

        // running with one byte of hash
        let start = Instant::now(); 
        for entry in registry.iter(){     
            let ss = decaps(&entry.0, &v_priv);
        
            let view_tag = hash_val_one_byte(&ss);

            if entry.2 == view_tag{
                let _ = gen_stealth_pub_key(&ss, &k_pub, &rho);  
            }    
        }
        times2.push(start.elapsed().as_millis() as f64);

        // running with whole hash 
        let start = Instant::now(); 
        for entry in registry.iter(){
            let ss = decaps(&entry.0, &v_priv);
    
            let view_tag = hash_val(&ss); 

            // If found 
            if entry.1 == view_tag{
                // Calculate stealth pub key
                let _ = gen_stealth_pub_key(&ss, &k_pub, &rho); 
                break; 
            }   
        }
        times1.push(start.elapsed().as_millis() as f64);

      

        // running without the viewtag 
        let start = Instant::now(); 
        for entry in registry.iter(){
            let ss = decaps(&entry.0, &v_priv); 
            let P = gen_stealth_pub_key(&ss, &k_pub, &rho); 
        }
        times3.push(start.elapsed().as_millis() as f64);

    }

    res.push(format!("N = {}, mean = {} ms, std deviation = {}", n,mean(&times1), std_dev(&times1, mean(&times1))));
    res2.push(format!("N = {}, mean = {} ms, std deviation = {}", n,mean(&times2), std_dev(&times2, mean(&times2))));
    res3.push(format!("N = {}, mean = {} ms, std deviation = {}", n,mean(&times3), std_dev(&times3, mean(&times3))));
    

}

pub fn hash_val(x: &[u8]) -> String{
    let mut hasher = Sha256::new(); 
    hasher.update(x); 
    hex::encode(hasher.finalize()) 
}

pub fn hash_val_one_byte(x: &[u8]) -> String{
    // hash ss and return first byte
    let mut hasher = Sha256::new(); 
    hasher.update(x); 
    hex::encode(hasher.finalize())[0..2].to_string()
}

