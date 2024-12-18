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
}