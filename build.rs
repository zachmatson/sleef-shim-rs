fn main() {
    cmake::Config::new("sleef")
        .define("BUILD_TESTS", "FALSE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("BUILD_GNUABI_LIBS", "FALSE")
        .build();

    println!("cargo:rerun-if-changed=sleef");
}