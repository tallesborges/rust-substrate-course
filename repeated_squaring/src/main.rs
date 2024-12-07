const PRIME: u64 = 997;
const BASE: u64 = 3;
const EXPONENTE: u128 = 123_456_789_123_456_678;

fn main() {
    print_modpow(BASE, EXPONENTE, PRIME);
    print_modpow(BASE, 123_123_123_123, PRIME);
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
