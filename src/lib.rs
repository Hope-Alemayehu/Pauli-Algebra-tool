#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pauli {
    I, //identity
    X,
    Y,
    Z,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Minus1, //-1
    Plus1,  //+1
    I,      //+i
    MinusI, //-i
}

impl Phase {
    pub fn multiply(self, other: Phase) -> Phase {
        use Phase::*;
        match (self, other) {
            (Plus1, Plus1) => Plus1,
            (Plus1, Minus1) => Minus1,
            (Plus1, I) => I,
            (Plus1, MinusI) => MinusI,

            (Minus1, Plus1) => Minus1,
            (Minus1, Minus1) => Plus1,
            (Minus1, I) => MinusI,
            (Minus1, MinusI) => I,

            (I, Plus1) => I,
            (I, Minus1) => MinusI,
            (I, I) => Minus1,
            (I, MinusI) => Plus1,

            (MinusI, Plus1) => MinusI,
            (MinusI, Minus1) => I,
            (MinusI, I) => Plus1,
            (MinusI, MinusI) => Minus1,
        }
    }
}

pub fn multiply_pauli(a: Pauli, b: Pauli) -> (Phase, Pauli) {
    use Pauli::*;
    match (a, b) {
        (I, p) | (p, I) => (Phase::Plus1, p),
        (X, X) | (Y, Y) | (Z, Z) => (Phase::Plus1, I),
        (X, Y) => (Phase::I, Z),
        (Y, X) => (Phase::MinusI, Z),
        (Y, Z) => (Phase::I, X),
        (Z, Y) => (Phase::MinusI, X),
        (Z, X) => (Phase::I, Y),
        (X, Z) => (Phase::MinusI, Y),
    }
}

pub fn simplify_pauli_string(input: &str) -> (Phase, Pauli) {
    let mut global_phase = Phase::Plus1;
    let mut accum = Pauli::I;

    for c in input.chars() {
        let next = match c.to_ascii_uppercase() {
            'I' => Pauli::I,
            'X' => Pauli::X,
            'Y' => Pauli::Y,
            'Z' => Pauli::Z,
            _ => continue, //ignores the invalid characters
        };

        let (phase, new_accum) = multiply_pauli(accum, next);
        global_phase = global_phase.multiply(phase);
        accum = new_accum;
    }
    (global_phase, accum)
}
