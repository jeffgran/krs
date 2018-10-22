#[macro_use] extern crate quicli;
use quicli::prelude::*;

#[derive(StructOpt)]
#[structopt(name = "krs", about = "crypto.rs -- A Cryptography/Math CLI Library")]
enum Cryptors {
    #[structopt(name = "gcd", help = "Returns the greatest common divisor of two numbers <a> and <b>")]
    Gcd {
        #[structopt(help = "First number", parse(from_os_str))]
        a: i64
        #[structopt(help = "Second number", parse(from_os_str))]
        b: i64
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {return b};
    if b == 0 {return a};
    if a == 1 || b == 1 {return 1};
    let bigger: i64;
    let smaller: i64;
    if a > b { bigger = a; smaller = b} else { bigger = b; smaller = a }
    return gcd(smaller, bigger % smaller);
}


main!(|args: Cli, log_level: Verbosity| {
    println!("{}", gcd(args.a, args.b))
})
