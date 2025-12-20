#[allow(unused_imports)]
use crate::Hashable;
use digest::Digest;

pub trait Hasher {
    type Output: AsRef<[u8]> + Sized + Copy + Clone + Hashable;
    fn hash<T: Hashable>(data: T) -> Self::Output;
    fn zeros() -> Self::Output;
}

macro_rules! def_hasher {
    ($crt: ident, $ty: ident, $func:ident, $sz: expr) => {
        pub struct $ty;

        impl Hasher for $ty {
            type Output = [u8; $sz];
            fn hash<T: Hashable>(data: T) -> Self::Output {
                $func(data)
            }
            fn zeros() -> Self::Output {
                [0; $sz]
            }
        }

        pub fn $func<T: Hashable>(data: T) -> [u8; $sz] {
            use $crt::$ty;

            let mut hasher = $ty::new();
            data.update_hasher(&mut |bytes| {
                hasher.update(bytes);
            });
            hasher.finalize().into()
        }
    };
}

#[cfg(feature = "md5")]
def_hasher!(md5, Md5, md5, 16);

#[cfg(feature = "sha2")]
def_hasher!(sha1, Sha1, sha1, 20);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha224, sha224, 28);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha256, sha256, 32);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha384, sha384, 48);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha512, sha512, 64);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha512_224, sha512_224, 28);
#[cfg(feature = "sha2")]
def_hasher!(sha2, Sha512_256, sha512_256, 32);

#[cfg(feature = "sha3")]
def_hasher!(sha3, Sha3_224, sha3_224, 28);
#[cfg(feature = "sha3")]
def_hasher!(sha3, Sha3_256, sha3_256, 32);
#[cfg(feature = "sha3")]
def_hasher!(sha3, Sha3_384, sha3_384, 48);
#[cfg(feature = "sha3")]
def_hasher!(sha3, Sha3_512, sha3_512, 64);
#[cfg(feature = "sha3")]
def_hasher!(sha3, Keccak256, keccak256, 32);
#[cfg(feature = "sha3")]
def_hasher!(sha3, Keccak512, keccak512, 64);

#[cfg(feature = "blake2")]
def_hasher!(blake2_helper, Blake2b256, blake2b_256, 32);
#[cfg(feature = "blake2")]
def_hasher!(blake2_helper, Blake2b384, blake2b_384, 48);
#[cfg(feature = "blake2")]
def_hasher!(blake2_helper, Blake2b512, blake2b_512, 64);
#[cfg(feature = "blake2")]
def_hasher!(blake2_helper, Blake2s128, blake2s_128, 16);
#[cfg(feature = "blake2")]
def_hasher!(blake2_helper, Blake2s256, blake2s_256, 32);

#[cfg(feature = "blake2")]
mod blake2_helper {
    pub type Blake2b256 = blake2::Blake2b<digest::consts::U32>;
    pub type Blake2b384 = blake2::Blake2b<digest::consts::U48>;
    pub type Blake2s128 = blake2::Blake2s<digest::consts::U16>;
    pub type Blake2s256 = blake2::Blake2s256;
    pub type Blake2b512 = blake2::Blake2b512;
}

#[cfg(feature = "blake3")]
pub use blake3_helper::*;
#[cfg(feature = "blake3")]
mod blake3_helper;
