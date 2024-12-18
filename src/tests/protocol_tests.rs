#[cfg(test)]
mod protocol_tests{
    use pq_sap::{crypto::kem::key_pair, versions::v0::{recipient_computes_stealth_pub_key, sender_computes_stealth_pub_key_and_viewtag}};

    #[test]
    fn test_sender_recipient_compute_same_value(){
        // tests whether sender and recipient computed the same value of P
        let (k_pub, _) = key_pair(); 
        let (v_pub, v_priv) = key_pair(); 
        

        let (stealth_pub_key_sender, ephemeral_pub_key, _) = sender_computes_stealth_pub_key_and_viewtag(&v_pub, &k_pub);

        let stealh_pub_key_recipient = recipient_computes_stealth_pub_key(&k_pub, &ephemeral_pub_key, &v_priv);  

        assert_eq!(stealh_pub_key_recipient, stealth_pub_key_sender);
        
    }
}