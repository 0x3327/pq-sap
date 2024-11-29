#[cfg(test)]
mod protocol_tests{
    use pqc_kyber::keypair;
    use mlwe_sap::versions::v0::{recipient_computes_stealth_pub_key, sender_computes_stealth_pub_key_and_viewtag};

    #[test]
    fn test_sender_recipient_compute_same_value(){
        // tests whether sender and recipient computed the same value of P 
        let mut rng = rand::thread_rng(); 
        let spending_key = keypair(&mut rng).expect("Error in generating keys"); 
        let viewing_key = keypair(&mut rng).expect("Error in generating keys");  
        let k_pub = spending_key.public;  
        let v_pub = viewing_key.public;
        

        let (stealth_pub_key_sender, ephemeral_pub_key, _) = sender_computes_stealth_pub_key_and_viewtag(&v_pub, &k_pub).expect("Error in creating public stealth key");

        let stealh_pub_key_recipient = recipient_computes_stealth_pub_key(&k_pub, &ephemeral_pub_key, &viewing_key.secret).expect("Error in computing public stealth key");  

        assert_eq!(stealh_pub_key_recipient, stealth_pub_key_sender);
        
    }
}