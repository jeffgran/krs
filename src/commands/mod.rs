pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {return b};
    if b == 0 {return a};
    if a == 1 || b == 1 {return 1};
    let bigger: i64;
    let smaller: i64;
    if a > b { bigger = a; smaller = b} else { bigger = b; smaller = a }
    return gcd(smaller, bigger % smaller);
}
