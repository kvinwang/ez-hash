pub trait Hashable {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8]));
}

impl Hashable for u8 {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        hasher(core::slice::from_ref(self))
    }
}

impl Hashable for [u8] {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        hasher(self)
    }
}

impl Hashable for str {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        hasher(str::as_bytes(self))
    }
}

impl Hashable for String {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        hasher(String::as_bytes(self))
    }
}

impl<T: Hashable + ?Sized> Hashable for &T {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        (**self).update_hasher(hasher)
    }
}

impl<T: Hashable, const N: usize> Hashable for [T; N] {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        for item in self {
            item.update_hasher(hasher);
        }
    }
}

impl<T: Hashable> Hashable for Vec<T> {
    fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
        for item in self {
            item.update_hasher(hasher);
        }
    }
}

macro_rules! impl_tuple {
    ($($T:ident),+) => {
        impl<$($T: Hashable),+> Hashable for ($($T,)+) {
            fn update_hasher(&self, hasher: &mut dyn FnMut(&[u8])) {
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                $($T.update_hasher(hasher);)+
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
