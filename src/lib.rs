use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

const CURVE_ORDER_LE: [u8; 32] = [
    47, 77, 198, 173, 65, 162, 102, 30, 50, 178, 231, 202, 109, 18, 129, 183, 255, 255, 255, 255,
    255, 255, 255, 255, 16, 0, 0, 0, 0, 0, 0, 8,
];

extern "C" {
    pub fn Hash(
        in1: *const ::std::os::raw::c_char,
        in2: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn GetPublicKey(
        private_key: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Verify(
        stark_key: *const ::std::os::raw::c_char,
        msg_hash: *const ::std::os::raw::c_char,
        r_bytes: *const ::std::os::raw::c_char,
        w_bytes: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn Sign(
        private_key: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        k: *const ::std::os::raw::c_char,
        out: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
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

pub fn get_public_key(private_key: &[u8; 32]) -> Result<[u8; 32], i32> {
    let mut buffer = [0u8; 1024];

    let res = unsafe {
        GetPublicKey(
            private_key.as_ptr() as *const i8,
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

pub fn verify(
    stark_key: &[u8; 32],
    msg_hash: &[u8; 32],
    r_bytes: &[u8; 32],
    s_bytes: &[u8; 32],
) -> bool {
    let w_bytes = invert_on_curve(s_bytes);

    let res = unsafe {
        Verify(
            stark_key.as_ptr() as *const i8,
            msg_hash.as_ptr() as *const i8,
            r_bytes.as_ptr() as *const i8,
            w_bytes.as_ptr() as *const i8,
        )
    };

    res != 0
}

pub fn sign(private_key: &[u8; 32], message: &[u8; 32], k: &[u8; 32]) -> Result<Signature, i32> {
    let mut buffer = [0u8; 1024];

    let res = unsafe {
        Sign(
            private_key.as_ptr() as *const i8,
            message.as_ptr() as *const i8,
            k.as_ptr() as *const i8,
            buffer.as_mut_ptr() as *mut i8,
        )
    };

    if res == 0 {
        let mut output_r = [0u8; 32];
        let mut output_w = [0u8; 32];
        output_r.copy_from_slice(&buffer[0..32]);
        output_w.copy_from_slice(&buffer[32..64]);

        let output_s = invert_on_curve(&output_w);

        Ok(Signature {
            r: output_r,
            s: output_s,
        })
    } else {
        Err(res)
    }
}

fn invert_on_curve(num: &[u8; 32]) -> [u8; 32] {
    let num = BigInt::from_bytes_le(num_bigint::Sign::Plus, num);
    let curve_order = BigInt::from_bytes_le(num_bigint::Sign::Plus, &CURVE_ORDER_LE);

    // Ported from:
    //   https://github.com/dignifiedquire/num-bigint/blob/56576b592fea6341b7e1711a1629e4cc1bfc419c/src/algorithms/mod_inverse.rs#L11
    let extended_gcd = num.extended_gcd(&curve_order);
    if extended_gcd.gcd != BigInt::one() {
        panic!("GCD must be one");
    }
    let mod_inverse = if extended_gcd.x < BigInt::zero() {
        extended_gcd.x + curve_order
    } else {
        extended_gcd.x
    };

    let (_, buffer) = mod_inverse.to_bytes_le();
    let mut result = [0u8; 32];
    result[0..buffer.len()].copy_from_slice(&buffer[..]);

    result
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

    #[test]
    fn test_get_public_key() {
        let mut private_key =
            hex!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let mut expected_key =
            hex!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43");

        // Little endian
        private_key.reverse();
        expected_key.reverse();

        match get_public_key(&private_key) {
            Ok(output) => assert_eq!(output, expected_key),
            Err(err_code) => panic!("GetPublicKey() failed with error code: {}", err_code),
        };
    }

    #[test]
    fn test_verify_valid_message() {
        let mut stark_key =
            hex!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca");
        let mut msg_hash = hex!("0000000000000000000000000000000000000000000000000000000000000002");
        let mut r_bytes = hex!("0411494b501a98abd8262b0da1351e17899a0c4ef23dd2f96fec5ba847310b20");
        let mut s_bytes = hex!("0405c3191ab3883ef2b763af35bc5f5d15b3b4e99461d70e84c654a351a7c81b");

        // Little endian
        stark_key.reverse();
        msg_hash.reverse();
        r_bytes.reverse();
        s_bytes.reverse();

        assert_eq!(verify(&stark_key, &msg_hash, &r_bytes, &s_bytes), true);
    }

    #[test]
    fn test_verify_invalid_message() {
        let mut stark_key =
            hex!("077a4b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43");
        let mut msg_hash = hex!("0397e76d1667c4454bfb83514e120583af836f8e32a516765497823eabe16a3f");
        let mut r_bytes = hex!("0173fd03d8b008ee7432977ac27d1e9d1a1f6c98b1a2f05fa84a21c84c44e882");
        let mut s_bytes = hex!("01f2c44a7798f55192f153b4c48ea5c1241fbb69e6132cc8a0da9c5b62a4286e");

        // Little endian
        stark_key.reverse();
        msg_hash.reverse();
        r_bytes.reverse();
        s_bytes.reverse();

        assert_eq!(verify(&stark_key, &msg_hash, &r_bytes, &s_bytes), false);
    }

    #[test]
    fn test_sign() {
        let mut private_key =
            hex!("0000000000000000000000000000000000000000000000000000000000000001");
        let mut message = hex!("0000000000000000000000000000000000000000000000000000000000000002");
        let mut k = hex!("0000000000000000000000000000000000000000000000000000000000000003");

        // Little endian
        private_key.reverse();
        message.reverse();
        k.reverse();

        let signature = match sign(&private_key, &message, &k) {
            Ok(output) => output,
            Err(err_code) => panic!("Sign() failed with error code: {}", err_code),
        };

        let public_key = get_public_key(&private_key).unwrap();

        assert_eq!(
            verify(&public_key, &message, &signature.r, &signature.s),
            true
        );
    }
}
