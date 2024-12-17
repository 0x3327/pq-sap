#[cfg(test)]
mod kem_tests{
    use mlwe_sap::crypto::kem::{decaps, encaps, key_pair};

   
    #[test]
    fn test_same_shared_secret_computed(){
        let (pk, sk) = key_pair(); 
        let (ct, ss1) = encaps(&pk);
        let ss2 = decaps(&ct, &sk);
        
        assert!(ss1 == ss2);
    }
}