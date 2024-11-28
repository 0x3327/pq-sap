use std::time::Instant;
use mlwe_sap::versions::v0::calculate_stealth_pub_key;
use pqc_kyber::{decapsulate, encapsulate, keypair, KYBER_CIPHERTEXTBYTES};

fn main(){

    let ns = [5000, 10000, 20000, 40000, 80000];
    for n in ns{
        run(n, 10);
    }

}

fn run(n: usize, m: usize){
    let mut t = 0;
    for _ in 0..m{
        let mut rng = rand::thread_rng();
        let spending_key = keypair(&mut rng).unwrap(); 
        let viewing_key = keypair(&mut rng).unwrap(); 

        let mut ephemeral_pub_key_reg: Vec<[u8; KYBER_CIPHERTEXTBYTES]> = vec![];
        let mut view_tags: Vec<u8> = vec![];

        for _ in 0..n{
            let mut rng = rand::thread_rng(); 
            let v_pub_i = keypair(&mut rng).unwrap().public;

            let (ephemeral_pub_key, ss) = encapsulate(&v_pub_i, &mut rng).unwrap(); 
        
            ephemeral_pub_key_reg.push(ephemeral_pub_key);
            view_tags.push(ss[0]);
        }

        let start = Instant::now(); 
        for (i, ephemeral_pub_key) in ephemeral_pub_key_reg.iter().enumerate(){
            
            let ss = decapsulate(ephemeral_pub_key, &viewing_key.secret).unwrap();
        
            let view_tag = ss[0]; 

            if view_tags[i] == view_tag{
                let _ = calculate_stealth_pub_key(&ss, &spending_key.public);  
            }   
            
        }

        t+=start.elapsed().subsec_millis();
    }
    println!("{}", t/(m as u32));

}