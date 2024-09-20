use scalarff::Bn128FieldElement;
use scalarff::FieldElement;

use crate::and;
use crate::constrain_assign;
use crate::constrain_eq;
use crate::shl;
use crate::shr;
use crate::Signal;

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
    let n2b = Num2Bits(254, v);
    AliasCheck(n2b.clone());
    n2b
}

/// https://github.com/iden3/circomlib/blob/master/circuits/aliascheck.circom
pub fn AliasCheck(v: Vec<Bn128FieldElement>) {
    let negone = -Bn128FieldElement::one();
    constrain_eq(CompConstant(negone, v), Bn128FieldElement::zero());
}

/// https://github.com/iden3/circomlib/blob/master/circuits/compconstant.circom
pub fn CompConstant(ct: Bn128FieldElement, v: Vec<Bn128FieldElement>) -> Bn128FieldElement {
    let mut parts: Vec<Signal> = vec![None; 127];
    let mut sout: Signal = None;

    let mut clsb;
    let mut cmsb;
    let mut slsb;
    let mut smsb;

    let mut sum = Bn128FieldElement::zero();

    let mut b = shl(&Bn128FieldElement::one(), 128) - Bn128FieldElement::one();
    let mut a = Bn128FieldElement::one();
    let mut e = Bn128FieldElement::one();

    for i in 0..127 {
        clsb = and(&shr(&ct, i * 2), 1);
        cmsb = and(&shr(&ct, i * 2 + 1), 1);
        slsb = v[i * 2];
        smsb = v[i * 2 + 1];

        if cmsb == Bn128FieldElement::zero() && clsb == Bn128FieldElement::zero() {
            constrain_assign(-b * smsb * slsb + b * smsb + b * slsb, &mut parts[i]);
        } else if cmsb == Bn128FieldElement::zero() && clsb == Bn128FieldElement::one() {
            constrain_assign(
                a * smsb * slsb - a * slsb + b * smsb - a * smsb + a,
                &mut parts[i],
            );
        } else if cmsb == Bn128FieldElement::one() && clsb == Bn128FieldElement::zero() {
            constrain_assign(b * smsb * slsb - a * smsb + a, &mut parts[i]);
        } else {
            constrain_assign(-a * smsb * slsb + a, &mut parts[i]);
        }

        sum = sum + parts[i].unwrap();
        b = b - e;
        a = a + e;
        e = e * Bn128FieldElement::from(2);
    }

    constrain_assign(sum, &mut sout);
    Num2Bits(135, sout.unwrap())[127]
}
