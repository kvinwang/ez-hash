use ez_hash::{
    blake2b_256, blake2b_512, blake2s_256, blake3, keccak256, md5, sha1, sha224, sha256, sha384,
    sha3_256, sha3_512, sha512, sha512_256,
};
use hex_literal::hex;

#[test]
fn test_sha256_abc() {
    let expected: [u8; 32] =
        hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(sha256("abc"), expected);
    assert_eq!(sha256(("a", "bc")), expected);
}

#[test]
fn test_sha1_abc() {
    let expected: [u8; 20] = hex!("a9993e364706816aba3e25717850c26c9cd0d89d");
    assert_eq!(sha1("abc"), expected);
    assert_eq!(sha1(("a", "bc")), expected);
}

#[test]
fn test_sha512_abc() {
    let expected: [u8; 64] = hex!("ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f");
    assert_eq!(sha512("abc"), expected);
    assert_eq!(sha512(("a", "bc")), expected);
}

#[test]
fn test_md5_abc() {
    let expected: [u8; 16] = hex!("900150983cd24fb0d6963f7d28e17f72");
    assert_eq!(md5("abc"), expected);
    assert_eq!(md5(("a", "bc")), expected);
}

#[test]
fn test_blake3_abc_equivalence() {
    assert_eq!(blake3("abc"), blake3(("a", "bc")));
}

#[test]
fn test_bytes_hashable() {
    let bytes = 1u32.to_le_bytes();
    let a = sha256(bytes);
    let b = sha256(&bytes[..]);
    assert_eq!(a, b);
}

#[test]
fn test_array_of_strings() {
    let parts: [&str; 3] = ["a", "b", "c"];
    assert_eq!(sha256("abc"), sha256(parts));
}

#[test]
fn test_vec_of_strings() {
    let parts: Vec<&str> = vec!["a", "b", "c"];
    assert_eq!(sha256("abc"), sha256(parts));
}

#[test]
fn test_sha224_abc() {
    let expected: [u8; 28] = hex!("23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7");
    assert_eq!(sha224("abc"), expected);
}

#[test]
fn test_sha384_abc() {
    let expected: [u8; 48] = hex!("cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7");
    assert_eq!(sha384("abc"), expected);
}

#[test]
fn test_sha512_256_abc() {
    let expected: [u8; 32] =
        hex!("53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23");
    assert_eq!(sha512_256("abc"), expected);
}

#[test]
fn test_sha3_256_abc() {
    let expected: [u8; 32] =
        hex!("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
    assert_eq!(sha3_256("abc"), expected);
}

#[test]
fn test_sha3_512_abc() {
    let expected: [u8; 64] = hex!("b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0");
    assert_eq!(sha3_512("abc"), expected);
}

#[test]
fn test_keccak256_abc() {
    let expected: [u8; 32] =
        hex!("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45");
    assert_eq!(keccak256("abc"), expected);
}

#[test]
fn test_blake2b_256_abc() {
    let expected: [u8; 32] =
        hex!("bddd813c634239723171ef3fee98579b94964e3bb1cb3e427262c8c068d52319");
    assert_eq!(blake2b_256("abc"), expected);
}

#[test]
fn test_blake2b_512_abc() {
    let expected: [u8; 64] = hex!("ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923");
    assert_eq!(blake2b_512("abc"), expected);
}

#[test]
fn test_blake2s_256_abc() {
    let expected: [u8; 32] =
        hex!("508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982");
    assert_eq!(blake2s_256("abc"), expected);
}
