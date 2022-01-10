use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .static_flag(true)
        .file("lib/starkware-crypto/src/starkware/algebra/prime_field_element.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/elliptic_curve_constants.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ffi/pedersen_hash.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ffi/utils.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/pedersen_hash.cc")
        .include("lib/starkware-crypto/src")
        .flag("-std=c++17")
        .flag("-Werror")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-fno-strict-aliasing")
        .flag("-fPIC")
        .compile("starkware-crypto");
}
