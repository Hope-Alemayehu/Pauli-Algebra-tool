use pauli_algebra::{simplify_pauli_string, Pauli, Phase};
fn main() {
    let input = "XYZXXYY";
    let (phase, symbol) = simplify_pauli_string(input);

    let phase_str = match phase {
        Phase::Plus1 => "1",
        Phase::Minus1 => "-1",
        Phase::I => "i",
        Phase::MinusI => "-i",
    };

    println!("Final result: {} {:?}", phase_str, symbol);
}
