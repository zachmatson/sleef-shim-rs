fn main() {
    let dst = cmake::Config::new("sleef")
        .define("BUILD_TESTS", "FALSE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("BUILD_GNUABI_LIBS", "FALSE")
        .profile("Release")
        .build();
    
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=sleef");

    println!("cargo:rerun-if-changed=sleef");
}
