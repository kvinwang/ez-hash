use super::*;

pub fn blake3<const N: usize, T: Hashable>(data: T) -> [u8; N] {
    let mut hasher = blake3::Hasher::new();
    data.update_hasher(&mut |bytes| {
        hasher.update(bytes);
    });

    let mut out = [0u8; N];
    hasher.finalize_xof().fill(&mut out);
    out
}

pub struct Blake3_256;
pub struct Blake3_512;

impl Hasher for Blake3_256 {
    type Output = [u8; 32];

    fn hash<T: Hashable>(data: T) -> Self::Output {
        blake3(data)
    }

    fn zeros() -> Self::Output {
        [0; 32]
    }
}

impl Hasher for Blake3_512 {
    type Output = [u8; 64];

    fn hash<T: Hashable>(data: T) -> Self::Output {
        blake3(data)
    }

    fn zeros() -> Self::Output {
        [0; 64]
    }
}
