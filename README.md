# sleef-shim-rs

This respository allows installation of statically linked SLEEF library in a Cargo toolchain without actually creating FFI bindings or linking to rustc.  
It is intended mainly for making SLEEF available to other C code inside of a Rust project, rather than using SLEEF from Rust as in a `-sys` crate.

Because it was created for a specific and narrow use case I had and may or may not be generally useful. If you think this crate could be useful to your workflow with
some changes that don't break the existing concept, feel free to open an issue or PR.
