# Convert Str to Binary, hexadecimal and Octal:

## Cargo.toml
```
[package]
name = "rtype"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/refere>

[dependencies]
clap = "*"
to-binary = "*"
fmt = "*"
```

## Usages:
```rust
./rtype -h
```
```
To String Converter 0.1.0
lux-cut <lucas.charignon@protonmail.com>
String Converter

USAGE:
    rtype [OPTIONS]

OPTIONS:
    -b, --binary     Traduct string in binary
    -h, --help       Print help information
    -o, --octal      Traduct string to octal
    -V, --version    Print version information
    -x, --hexa       Traduct string to hexadecimale
```
## Compilation:

### Cargo:
```
cargo build --release
```

### rustc:
```
rustc rtype.rs
```