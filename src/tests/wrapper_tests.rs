#[cfg(test)]
mod wrapper_tests{
    use mlwe_sap::wrapper::newhope::{rlwe_kem_decaps, rlwe_kem_encaps, rlwe_kem_keypair};

    #[test]
    fn test_wrapped_kem(){
        let (pk,sk) = rlwe_kem_keypair(); 
        let (ct, ss_a) = rlwe_kem_encaps(&pk); 
        let ss_b = rlwe_kem_decaps(&ct, &sk);
        
        assert!(ss_a == ss_b);
    }
}