use scalarff::BigUint;
use scalarff::Bn128FieldElement;
use scalarff::FieldElement;
use zk_kit::SafeLessThan;
use zk_kit::SafeLessThanProposed;

mod circomlib;
mod zk_kit;

fn main() {
    {
        let n = 8;
        let out = circomlib::bitify::Num2Bits(n, Bn128FieldElement::from(100));
        println!(
            "Num2Bits: n = {n}, out = {}",
            out.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    }

    safe_less_than_counter();
    safe_less_than_alias_check();
}

pub type Signal = Option<Bn128FieldElement>;

/// find a counter example for the SafeLessThan component
/// where the wrong value is returned
fn safe_less_than_counter() {
    let n = 8;
    let in1 = Bn128FieldElement::from(2_u64.pow(9));
    let expected = Bn128FieldElement::zero();

    for x in 1000..10000 {
        let actual = SafeLessThan(n, (Bn128FieldElement::from(x), in1));
        if actual != expected {
            println!(
                "Found SafeLessThan counter example:
n: {n}
in: [{x}, {in1}]
Expected: {expected} Got: {actual}"
            );
            break;
        }
    }
}

/// Attempt to cause a constraint failure by passing largest possible values
/// to the circuit
fn safe_less_than_alias_check() {
    // circuit contains an assertion that this is the max value
    let n = 252;

    // choose the largest in0 possible, 2^252-1
    let in0 = Bn128FieldElement::from_biguint(&(BigUint::from(2_u64).pow(252) - 1_u64));

    // circuit subtracts in1 from in0, so set it to 0
    let in1 = Bn128FieldElement::zero();
    let expected = Bn128FieldElement::zero();

    let actual = SafeLessThanProposed(n, (in0, in1));
    if actual != expected {
        println!(
            "safe_less_than_alias_check:
n: {n}
in: [{in0}, {in1}]
Expected: {expected} Got: {actual}"
        );
    } else {
        println!("alias check was not triggered")
    }
}

/// shift a field element to the right using it's real representation
fn shr(f: &Bn128FieldElement, amount: usize) -> Bn128FieldElement {
    Bn128FieldElement::from_biguint(&(f.to_biguint() >> amount))
}

/// shift a field element to the left using it's real representation
fn shl(f: &Bn128FieldElement, amount: usize) -> Bn128FieldElement {
    Bn128FieldElement::from_biguint(&(f.to_biguint() << amount))
}

/// Bitwise and between the real representation of a field element and a usize
fn and(f: &Bn128FieldElement, other: usize) -> Bn128FieldElement {
    Bn128FieldElement::from_biguint(&(f.to_biguint() & BigUint::from(other)))
}

/// Constrain equality between two elements
fn constrain_eq(v0: Bn128FieldElement, v1: Bn128FieldElement) {
    if v0 != v1 {
        panic!("constraint failed");
    }
}

fn constrain_assign(v: Bn128FieldElement, into: &mut Signal) {
    if let Some(_) = into {
        panic!("attempting to assign multiple values to signal");
    } else {
        *into = Some(v);
    }
}
