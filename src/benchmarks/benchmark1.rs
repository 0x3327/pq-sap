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


        let K = hex::encode(spending_key.public);
        let V =  hex::encode(viewing_key.public); 
        let k = hex::encode(spending_key.secret); 
        let v = hex::encode(viewing_key.secret); 


        let mut Rs: Vec<[u8; KYBER_CIPHERTEXTBYTES]> = vec![];
        let mut view_tags: Vec<u8> = vec![];

        for _ in 0..n{
            let mut rng = rand::thread_rng(); 
            let V_i = keypair(&mut rng).unwrap().public;

            let (R, S) = encapsulate(&V_i, &mut rng).unwrap(); 
        
            Rs.push(R);
            view_tags.push(S[0]);
        }

        let start = Instant::now(); 
        for (i, R) in Rs.iter().enumerate(){
            
            let S = decapsulate(R, &viewing_key.secret).unwrap();
        
            let view_tag = S[0]; 

            if view_tags[i] == view_tag{
                let P = calculate_stealth_pub_key(&S, &spending_key.public);  
            }   
            
        }
        t+=start.elapsed().subsec_millis();
    }
    println!("{}", t/(m as u32));

}