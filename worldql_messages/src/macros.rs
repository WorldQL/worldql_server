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

macro_rules! impl_into_message {
    ($from:ident, $variant:ident, $super:ty) => {
        paste::paste! {
            #[automatically_derived]
            impl From<[< $from $variant >]> for $super {
                #[inline]
                #[must_use]
                fn from(from: [< $from $variant >]) -> Self {
                    Self::$variant(from.into())
                }
            }
        }
    };
}

pub(crate) use impl_into_message;
