# Powercap Rust Bindings

This Rust crate provides declarations and linkage for the `powercap` C library.

The latest `powercap` C library can be found at [https://github.com/powercap/powercap](https://github.com/powercap/powercap).


## Dependencies

The `powercap` library and headers must be installed where they can be found by `pkg-config`.
In Debian Unstable and Ubuntu Linux >= 18.04, install the `libpowercap-dev` package.
Otherwise install it from the source (linked above).

Bindings are generated using the Rust [bindgen](https://github.com/rust-lang/rust-bindgen) crate.


## Usage
Add `powercap-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
powercap-sys = "0.2"
```


## Project Source

Find this and related project sources at the [powercap organization on GitHub](https://github.com/powercap).  
This project originates at: https://github.com/powercap/powercap-sys

Bug reports and pull requests for bug fixes and enhancements are welcome.
