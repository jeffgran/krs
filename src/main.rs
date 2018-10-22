#[macro_use] extern crate quicli;
use quicli::prelude::*;
mod commands;

#[derive(StructOpt)]
#[structopt(name = "krs", about = "crypto.rs -- A Cryptography/Math CLI Library")]
enum Cryptors {
    #[structopt(name = "gcd", help = "Returns the greatest common divisor of two numbers <a> and <b>")]
    Gcd {
        #[structopt(help = "First number")]
        a: i64,
        #[structopt(help = "Second number")]
        b: i64
    },

    #[structopt(name = "mpow", help = "Modular Exponentiation, i.e. a^b(mod n)")]
    Mpow {
        #[structopt(help = "Base")]
        a: i64,
        #[structopt(help = "Exponent")]
        b: i64,
        #[structopt(help = "Modulus")]
        n: i64
    },

    #[structopt(name = "isdiv", help = "Is <a> evenly divisible by <b>?")]
    Isdiv {
        #[structopt(help = "Number")]
        a: i64,
        #[structopt(help = "Divisor")]
        b: i64,
    },

}



main!(|args: Cryptors| {
    match args {
        Cryptors::Gcd { a, b } => println!("{}", commands::gcd(a, b)),
        Cryptors::Mpow { a, b, n } => println!("{}", commands::mpow(a, b, n)),
        Cryptors::Isdiv { a, b } => println!("{}", commands::is_divisible(a, b)),
    }
});
