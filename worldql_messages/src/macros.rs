macro_rules! impl_into_super {
    ($variant:ident, $suffix:ident, $super:ty) => {
        paste::paste! {
            #[automatically_derived]
            impl From<[< $variant $suffix >]> for $super {
                #[inline]
                #[must_use]
                fn from(from: [< $variant $suffix >]) -> Self {
                    Self::$variant(from)
                }
            }
        }
    };
}

pub(crate) use impl_into_super;
