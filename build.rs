extern crate cc;


fn main(){

    #[cfg(any(feature = "newhope1024", feature = "newhope512"))]
    newhope_build(); 

    #[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
    frodokem_build();

}

#[cfg(any(feature = "frodo640", feature = "frodo976", feature = "frodo1344"))]
fn frodokem_build(){
    const FRODO_KEM_DIR: &str = "src/libs/PQCrypto-LWEKE/FrodoKEM/src/";
    const FRODO_COMMON_DIR: &str = "src/libs/PQCrypto-LWEKE/common/";

   
    let mut build = cc::Build::new();

    #[cfg(feature = "frodo640")]
    build.file(format!("{}{}", FRODO_KEM_DIR, "frodo640.c")); 
    #[cfg(feature = "frodo976")]
    build.file(format!("{}{}", FRODO_KEM_DIR, "frodo976.c")); 
    #[cfg(feature = "frodo1344")]
    build.file(format!("{}{}", FRODO_KEM_DIR, "frodo1344.c")); 

    build
    .file(format!("{}{}", FRODO_KEM_DIR, "util.c"))
    .file(format!("{}{}", FRODO_COMMON_DIR, "random/random.c"))
    .file(format!("{}{}", FRODO_COMMON_DIR, "sha3/fips202.c"));

    if cfg!(target_family = "unix"){
        build.define("NIX", "1"); 
    }else if cfg!(target_family = "windows"){
        build.define("WINDOWS", "1");
    }else{
        println!("Unknown OS"); 
        return;
    }

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match target_arch.as_str() {
        "x86_64" => {
            build.define("_AMD64_", Some("1"));
            build.define("TARGET", Some("TARGET_AMD64"));
        }
        "x86" => {
            build.define("_X86_", Some("1"));
            build.define("TARGET", Some("TARGET_X86"));
        }
        "arm" => {
            build.define("_ARM_", Some("1"));
            build.define("TARGET", Some("TARGET_ARM"));
        }
        "aarch64" => {
            build.define("_ARM_", Some("1"));
            build.define("TARGET", Some("TARGET_ARM"));
        }
        "powerpc" => {
            build.define("_PPC_", Some("1"));
            build.define("TARGET", Some("TARGET_PPC"));
        }
        "s390x" => {
            build.define("_S390X_", Some("1"));
            build.define("TARGET", Some("TARGET_S390X"));
        }
        _ => {
            panic!("Unsupported architecture: {}", target_arch);
        }
    }
    
    build.define("_FAST_GENERIC_", Some("1"))
    .define("_SHAKE128_FOR_A_", Some("1"))
    .compile("frodo");
}

#[cfg(any(feature = "newhope1024", feature = "newhope512"))]
fn newhope_build(){
    const NEWHOPE_DIR: &str = "src/libs/newhope/ref/";

    #[cfg(feature = "newhope1024")]
    let newhope_n = "1024"; 

    #[cfg(feature = "newhope512")]
    let newhope_n = "512"; 

    cc::Build::new()
    .file(format!("{}{}", NEWHOPE_DIR, "ccakem.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "cpapke.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "poly.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "ntt.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "randombytes.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "precomp.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "fips202.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "reduce.c"))
    .file(format!("{}{}", NEWHOPE_DIR, "verify.c"))
    .define("NEWHOPE_N", newhope_n)
    .compile("newhope");
}