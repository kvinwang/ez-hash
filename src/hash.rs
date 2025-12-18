#[allow(unused_imports)]
use crate::Hashable;

#[cfg(feature = "sha1")]
pub fn sha1<T: Hashable>(data: T) -> [u8; 20] {
    use sha1::{Digest as _, Sha1};

    let mut hasher = Sha1::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha224<T: Hashable>(data: T) -> [u8; 28] {
    use sha2::{Digest as _, Sha224};

    let mut hasher = Sha224::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha256<T: Hashable>(data: T) -> [u8; 32] {
    use sha2::{Digest as _, Sha256};

    let mut hasher = Sha256::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha384<T: Hashable>(data: T) -> [u8; 48] {
    use sha2::{Digest as _, Sha384};

    let mut hasher = Sha384::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha512<T: Hashable>(data: T) -> [u8; 64] {
    use sha2::{Digest as _, Sha512};

    let mut hasher = Sha512::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha512_224<T: Hashable>(data: T) -> [u8; 28] {
    use sha2::{Digest as _, Sha512_224};

    let mut hasher = Sha512_224::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha2")]
pub fn sha512_256<T: Hashable>(data: T) -> [u8; 32] {
    use sha2::{Digest as _, Sha512_256};

    let mut hasher = Sha512_256::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn sha3_224<T: Hashable>(data: T) -> [u8; 28] {
    use sha3::{Digest as _, Sha3_224};

    let mut hasher = Sha3_224::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn sha3_256<T: Hashable>(data: T) -> [u8; 32] {
    use sha3::{Digest as _, Sha3_256};

    let mut hasher = Sha3_256::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn sha3_384<T: Hashable>(data: T) -> [u8; 48] {
    use sha3::{Digest as _, Sha3_384};

    let mut hasher = Sha3_384::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn sha3_512<T: Hashable>(data: T) -> [u8; 64] {
    use sha3::{Digest as _, Sha3_512};

    let mut hasher = Sha3_512::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn keccak256<T: Hashable>(data: T) -> [u8; 32] {
    use sha3::{Digest as _, Keccak256};

    let mut hasher = Keccak256::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "sha3")]
pub fn keccak512<T: Hashable>(data: T) -> [u8; 64] {
    use sha3::{Digest as _, Keccak512};

    let mut hasher = Keccak512::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake2")]
pub fn blake2b_256<T: Hashable>(data: T) -> [u8; 32] {
    use blake2::{Blake2b, Digest as _};
    use digest::consts::U32;

    let mut hasher = Blake2b::<U32>::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake2")]
pub fn blake2b_384<T: Hashable>(data: T) -> [u8; 48] {
    use blake2::{Blake2b, Digest as _};
    use digest::consts::U48;

    let mut hasher = Blake2b::<U48>::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake2")]
pub fn blake2b_512<T: Hashable>(data: T) -> [u8; 64] {
    use blake2::{Blake2b512, Digest as _};

    let mut hasher = Blake2b512::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake2")]
pub fn blake2s_128<T: Hashable>(data: T) -> [u8; 16] {
    use blake2::{Blake2s, Digest as _};
    use digest::consts::U16;

    let mut hasher = Blake2s::<U16>::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake2")]
pub fn blake2s_256<T: Hashable>(data: T) -> [u8; 32] {
    use blake2::{Blake2s256, Digest as _};

    let mut hasher = Blake2s256::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}

#[cfg(feature = "blake3")]
pub fn blake3<T: Hashable>(data: T) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    *hasher.finalize().as_bytes()
}

#[cfg(feature = "md5")]
pub fn md5<T: Hashable>(data: T) -> [u8; 16] {
    use md5::{Digest as _, Md5};

    let mut hasher = Md5::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });
    hasher.finalize().into()
}
