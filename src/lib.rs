extern "C" {
    pub fn Hash(
        in1: *const ::std::os::raw::c_char,
        in2: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn hash(in1: &[u8; 32], in2: &[u8; 32]) -> Result<[u8; 32], i32> {
    let mut buffer = [0u8; 1024];

    let res = unsafe {
        Hash(
            in1.as_ptr() as *const i8,
            in2.as_ptr() as *const i8,
            buffer.as_mut_ptr() as *mut i8,
        )
    };

    if res == 0 {
        let mut output = [0u8; 32];
        output.copy_from_slice(&buffer[0..32]);
        Ok(output)
    } else {
        Err(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_literal::hex;

    // Test cases ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    fn test_hash() {
        let mut in1 = hex!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
        let mut in2 = hex!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
        let mut expected_hash =
            hex!("030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662");

        // Little endian
        in1.reverse();
        in2.reverse();
        expected_hash.reverse();

        match hash(&in1, &in2) {
            Ok(output) => assert_eq!(output, expected_hash),
            Err(err_code) => panic!("Hash() failed with error code: {}", err_code),
        };
    }
}
