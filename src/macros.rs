// use ...

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[allow(unused_macro_rules)]
macro_rules! is_variant {
    ($inner:ident, $( ($fn_name:ident, $variant_name:expr) ),* ) => {
        $(
            is_variant!($inner, $fn_name, $variant_name);
        )*
    };
    ($( ($fn_name:ident, $variant_name:expr) ),* ) => {
        $(
            is_variant!($fn_name, $variant_name);
        )*
    };
    ($inner:ident, $fn_name:ident, $variant_name:expr) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<is_ $fn_name>](&self) -> bool {
                matches!(&self.$inner, $variant_name)
            }
        }
    };
    ($fn_name:ident, $variant_name:expr) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<is_ $fn_name>](&self) -> bool {
                matches!(&self, Self::$variant_name)
            }
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! as_variant {
    ($inner:ident, $fn_name:ident, $variant_name:expr, $type_name:ty) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<as_ $fn_name>](&self) -> Option<&$type_name> {
                match self.$inner {
                    $variant_name(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
    ($fn_name:ident, $variant_name:expr, $type_name:ty) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<as_ $fn_name>](&self) -> Option<&$type_name> {
                match self {
                    Self::$variant_name(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! into_variant {
    ($( ($fn_name:ident, $variant_name:expr, $type_name:ty) ),* ) => {
        $(
            into_variant!($fn_name, $variant_name, $type_name);
        )*
    };
     ($fn_name:ident, $variant_name:expr, $type_name:ty) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<into_ $fn_name>](self) -> Option<$type_name> {
                match self {
                    Self::$variant_name(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! as_variant_mut {
    ($inner:ident, $fn_name:ident, $variant_name:expr, $type_name:ty) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<as_ $fn_name _mut>](&mut self) -> Option<&mut $type_name> {
                match self.$inner {
                    $variant_name(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
    ($fn_name:ident, $variant_name:expr, $type_name:ty) => {
        ::paste::paste! {
            #[inline(always)]
            pub fn [<as_ $fn_name _mut>](&mut self) -> Option<&mut $type_name> {
                match self {
                    Self::$variant_name(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! is_as_variant {
    ($inner:ident, $( ($fn_name:ident, $variant_name:expr, $type_name:ty) ),* ) => {
        $(
            is_as_variant!($inner, $fn_name, $variant_name, $type_name);
        )*
    };
    ($( ($fn_name:ident, $variant_name:expr, $type_name:ty) ),* ) => {
        $(
            is_as_variant!($fn_name, $variant_name, $type_name);
        )*
    };
     ($inner:ident, $fn_name:ident, $variant_name:expr, $type_name:ty) => {
        is_variant!($inner, $fn_name, $variant_name(_));
        as_variant!($inner, $fn_name, $variant_name, $type_name);
        as_variant_mut!($inner, $fn_name, $variant_name, $type_name);
    };
    ($fn_name:ident, $variant_name:expr, $type_name:ty) => {
        is_variant!($fn_name, $variant_name(_));
        as_variant!($fn_name, $variant_name, $type_name);
        as_variant_mut!($fn_name, $variant_name, $type_name);
    };
}

macro_rules! impl_display_into_str {
    ($value_type:ty, $( ($variant:pat => $string:literal) ),+ ) => {
        impl ::std::fmt::Display for $value_type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match_into_str!(self, $( ($variant => $string) ),+ )
                 )
            }
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! match_into_str {
   ($value:expr, $( ($variant:pat => $string:literal) ),+ ) => {
        match $value {
            $(
                $variant => $string,
            )+
        }
    };
   ($value:expr, $( ($variant:pat => $string:expr) ),+ ) => {
        match $value {
            $(
                $variant => $string,
            )+
        }
    };
}
