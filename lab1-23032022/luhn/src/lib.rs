/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut processed = Vec::new();
    for c in code.chars() {
        if c == ' ' { continue; }
        match c.to_digit(10) {
            Some(v) => processed.push(v),
            None => return false
        }
    }
    if processed.len() <= 1 { return false; }
    for i in (0..processed.len()-1).rev().step_by(2) {
        processed[i] *= 2;
        if processed[i] > 9 { processed[i] -= 9; }
    }
    // The turbofish notation is necessary cause rust cannot infer the type, so it must
    // be explicitly written (could also use a variable notation)
    processed.iter().sum::<u32>() % 10 == 0
}
