# Chapter 2, A tour of Rust

The best way to install rust is using [rustup](https://rustup.rs).

This will install:
- **cargo**, rusts compilation manager, package manager, and general purpose tool.
- **rustc**, rusts compiler.
- **rustdoc**, rusts documentation tool. If you write documentation in your code, it will turn it into HTML.

Cargo can create a new Rust package for us using the command `cargo new <name>`.

This command will create a new package which will contain a `Cargo.toml` file, which will be used 
to add dependencies to our package. It will also create a `.git` directory, this can be avoided by 
passing `--vcs`.

We can compile and run the program using the `cargo run` command. 

## Functions in rust
Rust has a very unoriginal syntax by design. Here is an example function

```rust
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        
        m = m % n;
    }
    //! We dont need to write `return`, if the last line doesnt contain `;`
    //! it will be returned by the function
    n
}
```

## Testing
Rust has simple support for testing built in. To test the gcd function above, we would write this in the same file:
```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    
    assert_eq!(gcd(
       2 * 3 * 5 * 11 * 17,
       3 * 7 * 11 * 13* 19 
    ), 3 * 11);
}
```

The `#[test]` marks `test_gcd` as a test function, and it tells cargo to skip when compiling, but to run when using `cargo test`. 