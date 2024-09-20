use scalarff::Bn128FieldElement;
use scalarff::FieldElement;

use crate::and;
use crate::constrain_eq;
use crate::shr;

pub fn Num2Bits(n: usize, v: Bn128FieldElement) -> Vec<Bn128FieldElement> {
    let mut lc1 = Bn128FieldElement::zero();
    let mut e2 = Bn128FieldElement::one();
    let mut out = vec![Bn128FieldElement::zero(); n];

    for i in 0..n {
        out[i] = and(&shr(&v, i), 1);
        constrain_eq(
            out[i] * (out[i] - Bn128FieldElement::one()),
            Bn128FieldElement::zero(),
        );
        lc1 += out[i] * e2;
        e2 += e2;
    }
    out
}

pub fn Num2Bits_strict(v: Bn128FieldElement) -> Vec<Bn128FieldElement> {
    Num2Bits(254, v)
}
