use pauli_algebra::{Pauli, Phase, simplify_pauli_string};

#[test]
fn test_xyzxxyy() {
    let (phase, symbol) = simplify_pauli_string("XYZXXYY");
    assert_eq!(phase, Phase::I);
    assert_eq!(symbol, Pauli::I);
}

#[test]
fn test_xxy() {
    let (phase, symbol) = simplify_pauli_string("XXY");
    assert_eq!(phase, Phase::Plus1);
    assert_eq!(symbol, Pauli::Y);
}
