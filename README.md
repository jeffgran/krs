### A Rust Cryptography Toy

This is just a little toy command line program I wrote while taking Coursera Cryptography classes. I implemented some helper functions based on algorithms we learned (square-and-multiply for fast exponentiation, modular exponentiation without overflowing integers, raw RSA math, etc).

Here's what it looks like:

```bash
$ krs
krs 0.1.0
Jeff Gran <jeff.gran@gmail.com>
crypto.rs -- A Cryptography/Math CLI Library

USAGE:
    krs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    gcd        Returns the greatest common divisor of two numbers <a> and <b>
    help       Prints this message or the help of the given subcommand(s)
    isdiv      Is <a> evenly divisible by <b>?
    isfmt      Is the number probably prime (per Fermat)?
    isprime    Is the number prime?
    mi         What is the multiplicative inverse of <a> mod <n> (if it exists)?
    mpow       Modular Exponentiation, i.e. a^b(mod n)
    rsae       RSA encrypt integer 'message'

$ krs help mi
Usage: krs mi <a> <n>

$ krs mi 14 53
Some(19)

$ krs mi 6 18
None
```
