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

}
