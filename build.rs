extern crate cc; 

fn main(){
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
    .compile("newhope");
}