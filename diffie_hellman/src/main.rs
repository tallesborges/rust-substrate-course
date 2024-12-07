const PRIME: u128 = 997;
const BASE: u64 = 3;

fn main() {
    const ALICE_SECRET: u128 = 123;
    const BOB_SECRET: u128 = 456;

    let pub_key_alice = modpow(BASE, ALICE_SECRET, PRIME);
    let pub_key_bob = modpow(BASE, BOB_SECRET, PRIME);

    println!("Alice's public key: {}", pub_key_alice);
    println!("Bob's public key: {}", pub_key_bob);

    let alice_shared_secret = modpow(pub_key_bob, ALICE_SECRET, PRIME);
    let bob_shared_secret = modpow(pub_key_alice, BOB_SECRET, PRIME);

    println!("Alice's shared secret: {}", alice_shared_secret);
    println!("Bob's shared secret: {}", bob_shared_secret);
}

fn print_modpow(base: u64, exp: u128, p: u64) {
    println!("{base} ** {exp} % {p} == {}", modpow(base, exp, p));
}

fn modpow(base: u64, exp: u128, p: u64) -> u64 {
    let mut expo = exp;
    let mut result = 1;
    let mut double = base;
    while expo > 0 {
        if expo % 2 == 1 {
            result = (result * double) % p;
        }
        expo /= 2;
        double = (double * double) % p;
    }
    result
}
