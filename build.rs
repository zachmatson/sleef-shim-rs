fn main() {
    cmake::Config::new("sleef")
        .define("BUILD_TESTS", "FALSE")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .build();
}