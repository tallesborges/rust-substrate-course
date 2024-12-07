fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    println!("Encrypting 'hello' with a shift of 13");
    let encrypted = caeser_cipher("hello", 13, Mode::Encrypt, &alphabet);
    println!("Encrypted text: {}", encrypted);

    let decrypted = caeser_cipher(&encrypted, 13, Mode::Decrypt, &alphabet);
    println!("Decrypted text: {}", decrypted);
}

enum Mode {
    Encrypt,
    Decrypt,
}

fn caeser_cipher(text: &str, shift: usize, mode: Mode, alphabet: &str) -> String {
    let mut result = String::new();

    for c in text.chars() {
        let index = alphabet.find(c).unwrap();
        let shifted_index = match mode {
            Mode::Encrypt => (index + shift) % alphabet.len(),
            Mode::Decrypt => (index - shift) % alphabet.len(),
        };
        result.push(alphabet.chars().nth(shifted_index).unwrap());
    }

    result
}
