macro_rules! make_from {
    ($fromType:ident, $toType:ident) => {
        impl From<$fromType> for $toType {
            fn from(value: $fromType) -> Self {
                Self::$fromType(value)
            }
        }
    };
}

pub(crate) use make_from;
