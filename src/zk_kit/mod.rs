use scalarff::Bn128FieldElement;
use scalarff::FieldElement;

use super::circomlib::bitify::Num2Bits_strict;
use super::shl;

pub fn SafeLessThan(n: usize, v: (Bn128FieldElement, Bn128FieldElement)) -> Bn128FieldElement {
    if n > 252 {
        panic!("assertion failed in SafeLessThan");
    }

    let out = Num2Bits_strict(v.0 + shl(&Bn128FieldElement::one(), n) - v.1);
    Bn128FieldElement::one() - out[n]
}
