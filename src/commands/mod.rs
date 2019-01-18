extern crate rand;
use rand::prelude::*;

pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {return b};
    if b == 0 {return a};
    if a == 1 || b == 1 {return 1};
    let bigger: i64;
    let smaller: i64;
    if a > b { bigger = a; smaller = b} else { bigger = b; smaller = a }
    return gcd(smaller, bigger % smaller);
}

pub fn mpow(base: i64, exponent: i64, modulus: i64) -> i64 {
    let mut bit: i64 = 1;
    let mut result: i64 = 1;
    let mut factor = base;

    while bit <= exponent {
        if exponent & bit == bit { // if this bit is set in the exponent
            result = (result * factor) % modulus;
        }
        factor = (factor * factor) % modulus;
        bit <<= 1; // move the bit left
    }
    return result;
}

pub fn mult_inverse(num: i64, modulus: i64) -> Option<i64> {
    let mut r = Vec::new();
    let mut a = Vec::new();

    r.push(modulus);
    r.push(num);
    a.push(0);
    a.push(1);

    let mut i = 1;
    while r[i] != 0 && r[i] != 1 {
        i = i + 1;
        r.push(r[i-2] % r[i-1]);
        let q = r[i-2] / r[i-1];
        let nexta = (a[i-2] - (q * a[i-1])) % modulus;
        a.push(nexta);
    }

    if r[i] == 0 {
        return None;
    } else {
        if a[i] < 0 { return Some(a[i] + modulus); } else { return Some(a[i]) }
    }
}

fn digits(n: i64) -> Vec<i64> {
    fn x_inner(n: i64, xs: &mut Vec<i64>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10 as i64);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}


pub fn is_divisible(num: i64, divisor: i64) -> bool {
    let mut residue = 0;
    let ds = digits(num);

    for digit in ds {
        residue = ((residue * 10) + digit) % divisor
    }
    return residue == 0;
}

pub fn is_prime_naive_trial_division(num: i64)  -> bool {
    for i in 2..num {
        if num % i == 0 {return false};
    }
    return true;
}


pub fn is_probable_prime_fermat(num: i64) -> bool {
    let mut rng = thread_rng();
    for _ in 1..15 {
        let a: i64 = rng.gen_range(2, num - 1);
        //println!("{} ^ {} % {} = {}", a, num - 1, num, mpow(a, num - 1, num));
        if (mpow(a, num - 1, num)) != 1 { return false; }
    }
    return true
}

const FIRST_15_PRIMES: [i64; 15] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
pub fn is_probable_prime_miller_rabin(num: i64) -> bool {
    let mut m = num - 1;
    let mut trail = Vec::new();
    while m % 2 == 0 {
        trail.insert(0, m);
        m = m / 2;
    }
    println!("{:?}", trail);
    for a in FIRST_15_PRIMES.iter() {
        let x = mpow(*a, trail[0], num);
        println!("{} ^ {} % {} = {}", *a, trail[0], num, mpow(*a, trail[0], num));
        if x == 1 || x == -1 { continue; }

        let mut found = false;
        for d in trail.iter() {
            // let y = mpow(x, 2, num);
            // println!("{} ^ {} % {} = {}", x, 2, num, mpow(x, 2, num));
            // if y == -1 { found = true; break; }
            if mpow(*a, *d, num) == -1 { found = true; break; }
        }
        if found == true { continue; } else { return false; }
    }

    return true
}

pub fn rsa_encrypt(msg: i64, n: i64, e: u32) -> i64 {
    return msg.pow(e) % n;
}

pub fn rsa_decrypt(ciphertext: i64, d: u32, n: i64) -> i64 {
    return ciphertext.pow(d) % n;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn big_exponent() {
        assert_eq!(mpow(47, 69, 143), 125);
    }

    #[test]
    fn huge_number() {
        assert_eq!(mpow(12345, 6789, 143), 125);
    }

    #[test]
    fn exponent_1() {
        assert_eq!(mpow(47, 1, 143), 47);
    }

    #[test]
    fn exponent_0() {
        assert_eq!(mpow(47, 0, 143), 1);
    }

    #[test]
    fn gcd1() {
        assert_eq!(gcd(10, 4), 2);
    }

    #[test]
    fn gcd2() {
        assert_eq!(gcd(504, 8), 8);
    }

    #[test]
    fn gcd_3() {
        assert_eq!(gcd(237803, 242653), 1);
        assert_eq!(gcd(240199, 242653), 1);
        assert_eq!(gcd(240199, 274327), 1);
        assert_eq!(gcd(242653, 274327), 1);
    }

    #[test]
    fn div_0() {
        assert_eq!(is_divisible(102030, 10), true);
        assert_eq!(is_divisible(102030, 9), false);
        assert_eq!(is_divisible(102030, 5), true);
        assert_eq!(is_divisible(102030, 99), false);
        assert_eq!(is_divisible(12345, 10), false);
    }

    #[test]
    fn test_mult_inverse() {
        assert_eq!(mult_inverse(6, 7), Some(6));
        assert_eq!(mult_inverse(99, 100), Some(99));
        assert_eq!(mult_inverse(5, 7), Some(3));
        assert_eq!(mult_inverse(1, 7), Some(1));
        assert_eq!(mult_inverse(37, 12345), Some(6673));
    }

    #[test]
    fn test_rsa_encrypt() {
        assert_eq!(rsa_encrypt(9, 143, 7), 48);
    }

    #[test]
    fn is_prime_naive_trial_division_test() {
        assert_eq!(is_prime_naive_trial_division(3), true);
        assert_eq!(is_prime_naive_trial_division(31), true);
        assert_eq!(is_prime_naive_trial_division(143), false);
        assert_eq!(is_prime_naive_trial_division(561), false);
        assert_eq!(is_prime_naive_trial_division(1009), true);
    }

    #[test]
    fn fermat_test() {
        assert_eq!(is_probable_prime_fermat(191), true);
        assert_eq!(is_probable_prime_fermat(1009), true);
        assert_eq!(is_probable_prime_fermat(1000), false);
        assert_eq!(is_probable_prime_fermat(561), false); // carmichael number! can be false positive.
        assert_eq!(is_probable_prime_fermat(19), true);
    }

    #[test]
    fn miller_rabin_test() {
        assert_eq!(is_probable_prime_miller_rabin(191), true);
        assert_eq!(is_probable_prime_miller_rabin(1009), true);
        assert_eq!(is_probable_prime_miller_rabin(1000), false);
        assert_eq!(is_probable_prime_miller_rabin(561), false); // carmichael number! can be false positive.
        assert_eq!(is_probable_prime_miller_rabin(19), true);
    }

}
