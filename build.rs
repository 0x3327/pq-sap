extern crate cc;

fn main(){
    #[cfg(feature = "newhope1024")]
    let newhope_n = "1024"; 

    #[cfg(feature = "newhope512")]
    let newhope_n = "512"; 
    

    #[cfg(any(feature = "newhope1024", feature = "newhope512"))]
    cc::Build::new()
    .file("src/newhope/ref/ccakem.c")
    .file("src/newhope/ref/cpapke.c")
    .file("src/newhope/ref/poly.c")
    .file("src/newhope/ref/ntt.c")
    .file("src/newhope/ref/randombytes.c")
    .file("src/newhope/ref/precomp.c")
    .file("src/newhope/ref/fips202.c")
    .file("src/newhope/ref/reduce.c") 
    .file("src/newhope/ref/verify.c")
    .define("NEWHOPE_N", newhope_n)
    .compile("newhope");

    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    let mut build = cc::Build::new();

    #[cfg(feature = "frodo640")]
    build.file("src/PQCrypto-LWEKE/FrodoKEM/src/frodo640.c"); 
    #[cfg(feature = "frodo976")]
    build.file("src/PQCrypto-LWEKE/FrodoKEM/src/frodo976.c"); 
    #[cfg(feature = "frodo1344")]
    build.file("src/PQCrypto-LWEKE/FrodoKEM/src/frodo1344.c"); 

    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    build
    .file("src/PQCrypto-LWEKE/FrodoKEM/src/util.c")
    .file("src/PQCrypto-LWEKE/common/random/random.c")
    .file("src/PQCrypto-LWEKE/common/sha3/fips202.c")
    .define("NIX", "1")
    .define("_ARM_", Some("1"))
    .define("TARGET", Some("TARGET_ARM"))
    .define("_FAST_GENERIC_", Some("1"))
    .define("_SHAKE128_FOR_A_", Some("1"))
    .compile("frodo");

}