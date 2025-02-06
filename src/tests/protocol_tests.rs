#[cfg(test)]
mod protocol_tests{
    use pq_sap::{crypto::kem::key_pair, versions::{v0::{recipient_computes_stealth_pub_key, sender_computes_stealth_pub_key_and_viewtag}, v1::{sender_computes_stealth_pub_key_and_viewtag as sender_computes_stealth_pub_key_and_viewtag_v1, recipient_computes_stealth_pub_key as recipient_computes_stealth_pub_key_v1}}};
    use secp256k1::Secp256k1;
    use secp256k1::rand::rngs::OsRng; 

    #[test]
    fn test_sender_recipient_compute_same_value_v0(){
        // tests whether sender and recipient computed the same value of P
        let (k_pub, _) = key_pair(); 
        let (v_pub, v_priv) = key_pair(); 
        

        let (stealth_pub_key_sender, ephemeral_pub_key, _) = sender_computes_stealth_pub_key_and_viewtag(&v_pub, &k_pub);

        let stealh_pub_key_recipient = recipient_computes_stealth_pub_key(&k_pub, &ephemeral_pub_key, &v_priv);  

        assert_eq!(stealh_pub_key_recipient, stealth_pub_key_sender);
        
    }

    #[test]
    fn test_sender_recipient_compute_same_value_v1(){
        let secp = Secp256k1::new(); 
        let (_, k_pub) = secp.generate_keypair(&mut OsRng);  
        let (v_pub, v_priv) = key_pair();

        let (stealth_pub_key_sender, ephemeral_pub_key, _) = sender_computes_stealth_pub_key_and_viewtag_v1(&v_pub, &k_pub);

        let stealth_pub_key_recipient = recipient_computes_stealth_pub_key_v1(&k_pub, &ephemeral_pub_key, &v_priv); 

        assert_eq!(stealth_pub_key_recipient, stealth_pub_key_sender);

    }
}