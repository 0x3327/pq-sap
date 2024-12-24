use crate::wrapper::newhope::consts::NEWHOPE_N;

#[repr(C)] 
pub struct Poly{
    pub coeffs: [u16; NEWHOPE_N], 
}