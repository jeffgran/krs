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
    }
}



main!(|args: Cryptors| {
    match args {
        Cryptors::Gcd { a, b } => println!("{}", commands::gcd(a, b)),
    }
});
