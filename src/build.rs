use cc::Build;

fn main() {
    // Linux & macOS & Windows-GNU
    // Build::new()
    //     .cpp(true)
    //     .static_flag(true)
    //     .file("lib/starkware-crypto/src/starkware/algebra/prime_field_element.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/ecdsa.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/elliptic_curve_constants.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/ffi/ecdsa.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/ffi/pedersen_hash.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/ffi/utils.cc")
    //     .file("lib/starkware-crypto/src/starkware/crypto/pedersen_hash.cc")
    //     .include("lib/starkware-crypto/src")
    //     .flag("-std=c++17")
    //     .flag("-Werror")
    //     .flag("-Wall")
    //     .flag("-Wextra")
    //     .flag("-fno-strict-aliasing")
    //     .flag("-fPIC")
    //     .opt_level(3)
    //     .compile("starkware-crypto");

    // Windows-MSVC
    Build::new()
        .cpp(true)
        .file("lib/starkware-crypto/src/starkware/algebra/prime_field_element.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ecdsa.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/elliptic_curve_constants.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ffi/ecdsa.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ffi/pedersen_hash.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/ffi/utils.cc")
        .file("lib/starkware-crypto/src/starkware/crypto/pedersen_hash.cc")
        .include("lib/starkware-crypto/src")
        //
        // CHANGE this to your $BOOST_ROOT
        //
        .include("C:\\PATH\\TO\\BOOST\\ROOT")
        //
        // CHANGE this to your $BOOST_ROOT
        //
        .flag("/std:c++17")
        .flag_if_supported("/w")
        .flag_if_supported("/EHsc")
        .flag_if_supported("/O2")
        .compile("starkware-crypto");
}
