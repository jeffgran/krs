#[macro_use] extern crate quicli;
use quicli::prelude::*;
mod commands;

#[derive(StructOpt)]
#[structopt(name = "krs", about = "crypto.rs -- A Cryptography/Math CLI Library")]
enum Cryptors {
    #[structopt(name = "gcd", about = "Returns the greatest common divisor of two numbers <a> and <b>", help = "Usage: krs gcd <a> <b>")]
    Gcd {
        #[structopt(help = "First number")]
        a: i64,
        #[structopt(help = "Second number")]
        b: i64
    },

    #[structopt(name = "mpow", about = "Modular Exponentiation, i.e. a^b(mod n)", help = "Usage: krs mpow <a> <b> <n>")]
    Mpow {
        #[structopt(help = "Base")]
        a: i64,
        #[structopt(help = "Exponent")]
        b: i64,
        #[structopt(help = "Modulus")]
        n: i64
    },

    #[structopt(name = "isdiv", about = "Is <a> evenly divisible by <b>?", help = "Usage: krs isdiv  <a> <b>")]
    Isdiv {
        #[structopt(help = "Number")]
        a: i64,
        #[structopt(help = "Divisor")]
        b: i64,
    },

    #[structopt(name = "mi", about = "What is the multiplicative inverse of <a> mod <n> (if it exists)?", help = "Usage: krs mi <a> <n>")]
    MultInverse {
        #[structopt(help = "Number")]
        a: i64,
        #[structopt(help = "Modulus")]
        n: i64,
    },

    #[structopt(name = "rsae", about = "RSA encrypt integer 'message'", help = "usage: krs rsae <message> <modulus> <exponent>")]
    RsaEncrypt {
        #[structopt(help = "Message")]
        message: i64,
        #[structopt(help = "Modulus")]
        n: i64,
        #[structopt(help = "Exponent")]
        e: u32,
    },

    #[structopt(name = "isprime", about = "Is the number prime?", help = "usage: krs isprime <number>")]
    IsPrime {
        #[structopt(help = "Number")]
        num: i64
    },

    #[structopt(name = "isfmt", about = "Is the number probably prime (per Fermat)?", help = "usage: krs isfmt <number>")]
    IsFermat {
        #[structopt(help = "Number")]
        num: i64
    },

}



main!(|args: Cryptors| {
    match args {
        Cryptors::Gcd { a, b } => println!("{}", commands::gcd(a, b)),
        Cryptors::Mpow { a, b, n } => println!("{}", commands::mpow(a, b, n)),
        Cryptors::Isdiv { a, b } => println!("{}", commands::is_divisible(a, b)),
        Cryptors::MultInverse { a, n } => println!("{:?}", commands::mult_inverse(a, n)),
        Cryptors::RsaEncrypt { message, n, e } => println!("{:?}", commands::rsa_encrypt(message, n, e)),
        Cryptors::IsPrime { num } => println!("{:?}", commands::is_prime_naive_trial_division(num)),
        Cryptors::IsFermat { num } => println!("{:?}", commands::is_probable_prime_fermat(num)),
    }
});
