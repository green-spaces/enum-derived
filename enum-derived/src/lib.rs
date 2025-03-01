#![doc = include_str!("../README.md")]

use std::{
    mem::{self, MaybeUninit},
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand {
    fn rand() -> Self;
}

impl<T: Rand> Rand for Option<T> {
    fn rand() -> Self {
        if bool::rand() {
            Some(T::rand())
        } else {
            None
        }
    }
}

impl<T: Rand> Rand for Vec<T> {
    fn rand() -> Self {
        let size = (rand::random::<usize>() % 63) + 1;
        let mut out = Vec::with_capacity(size);

        for _ in 0..size {
            out.push(T::rand());
        }

        debug_assert!(out.len() == size);
        out
    }
}

impl Rand for String {
    fn rand() -> Self {
        let size = (rand::random::<usize>() % 63) + 1;
        let mut out = Vec::with_capacity(size);

        for _ in 0..size {
            out.push(char::rand());
        }

        debug_assert!(out.len() == size);
        String::from_iter(out.iter())
    }
}

impl<T: Rand, const N: usize> Rand for [T; N] {
    fn rand() -> Self {
        let mut buff: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for elem in &mut buff {
            *elem = MaybeUninit::new(T::rand());
        }

        unsafe { mem::transmute_copy::<_, _>(&buff) }
    }
}

macro_rules! impl_rand {
    ($type:ty) => {
        impl Rand for $type {
            fn rand() -> Self {
                ::rand::random()
            }
        }
    };
}

impl_rand!(bool);
impl_rand!(char);
impl_rand!(f32);
impl_rand!(f64);
impl_rand!(i8);
impl_rand!(i16);
impl_rand!(i32);
impl_rand!(i64);
impl_rand!(i128);
impl_rand!(isize);
impl_rand!(u8);
impl_rand!(u16);
impl_rand!(u32);
impl_rand!(u64);
impl_rand!(u128);
impl_rand!(());
impl_rand!(usize);
impl_rand!(NonZeroU8);
impl_rand!(NonZeroU16);
impl_rand!(NonZeroU32);
impl_rand!(NonZeroU64);
impl_rand!(NonZeroU128);
impl_rand!(NonZeroUsize);

macro_rules! impl_tuple_rand {
    // use variables to indicate the arity of the tuple
    ($($tyvar:ident),* ) => {
        // the trailing commas are for the 1 tuple
        impl< $( $tyvar ),* >
            Rand
            for ( $( $tyvar ),* , )
            where $( $tyvar: Rand ),*
        {
            #[inline]
            fn rand() -> ( $( $tyvar ),* , ) {
                (
                    // use the $tyvar's to get the appropriate number of
                    // repeats (they're not actually needed)
                    $(
                        <$tyvar as Rand>::rand()
                    ),*
                    ,
                )
            }
        }
    }
}

impl_tuple_rand! {A}
impl_tuple_rand! {A, B}
impl_tuple_rand! {A, B, C}
impl_tuple_rand! {A, B, C, D}
impl_tuple_rand! {A, B, C, D, E}
impl_tuple_rand! {A, B, C, D, E, F}
impl_tuple_rand! {A, B, C, D, E, F, G}
impl_tuple_rand! {A, B, C, D, E, F, G, H}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T}
impl_tuple_rand! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U}
