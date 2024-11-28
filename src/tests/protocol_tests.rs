#[cfg(test)]
mod protocol_tests{
    use mlwe_sap::versions::v0::{recipient_computes_stealth_pub_key, sender_computes_stealth_pub_key_and_viewtag};
    use pqc_kyber::keypair;


    #[test]
    fn test_sender_recipient_compute_same_value(){
        // tests whether sender and recipient computed the same value of P 
        let mut rng = rand::thread_rng(); 
        let spending_key = keypair(&mut rng).expect("Error in generating keys"); 
        let viewing_key = keypair(&mut rng).expect("Error in generating keys");  
        let K = spending_key.public;  
        let V = viewing_key.public;
        

        let (P_sender, R, _) = sender_computes_stealth_pub_key_and_viewtag(&V, &K).expect("Error in creating public stealth key");

        let P_recipient = recipient_computes_stealth_pub_key(&K, &R, &viewing_key.secret).expect("Error in computing public stealth key");  

        assert_eq!(P_recipient, P_sender);
        
    }
}