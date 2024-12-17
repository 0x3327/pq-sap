use std::time::Instant;

use mlwe_sap::{crypto::{consts::CIPHERTEXT_BYTES, kem::{decaps, encaps, key_pair}}, versions::v0::calculate_stealth_pub_key};
fn main(){

    let ns = [5000, 10000, 20000, 40000, 80000];
    for n in ns{
        run(n, 10);
    }

}

fn run(n: usize, m: usize){
    let mut t = 0u128;
    for _ in 0..m{
        let (k_pub, _) = key_pair();
        let (v_pub, v_priv) = key_pair();
    

        let mut ephemeral_pub_key_reg: Vec<[u8; CIPHERTEXT_BYTES]> = vec![];
        let mut view_tags: Vec<u8> = vec![];

        for _ in 0..n{
            let (v_pub_i, _) = key_pair();

            let (ephemeral_pub_key, ss) = encaps(&v_pub_i);
        
            ephemeral_pub_key_reg.push(ephemeral_pub_key);
            view_tags.push(ss[0]);
        }

        let start = Instant::now(); 
        for (i, ephemeral_pub_key) in ephemeral_pub_key_reg.iter().enumerate(){
            
            let ss = decaps(ephemeral_pub_key, &v_priv);
        
            let view_tag = ss[0]; 

            if view_tags[i] == view_tag{
                let _ = calculate_stealth_pub_key(&ss, &k_pub);  
            }   
            
        }

        t+=start.elapsed().as_millis();
    }
    println!("N = {}, {} ms", n,t/(m as u128));

}
