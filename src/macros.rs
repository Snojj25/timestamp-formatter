#![allow(unused)]

#[cfg(feature = "nightly")]
pub use core::intrinsics::{assume, likely, unlikely};

// borrows technique from https://github.com/rust-lang/hashbrown/pull/209
#[cfg(not(feature = "nightly"))]
#[inline]
#[cold]
fn cold() {}

#[cfg(not(feature = "nightly"))]
#[rustfmt::skip]
#[inline(always)]
pub unsafe fn likely(b: bool) -> bool {
    if !b { cold() } b
}

#[cfg(not(feature = "nightly"))]
#[rustfmt::skip]
#[inline(always)]
pub unsafe fn unlikely(b: bool) -> bool {
    if b { cold() } b
}

#[cfg(not(feature = "nightly"))]
#[rustfmt::skip]
#[inline(always)]
pub unsafe fn assume(b: bool) {
    if !b { core::hint::unreachable_unchecked() }
}

#[rustfmt::skip]
macro_rules! likely {
    ($e:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::macros::likely($e) }
    }};
}

#[rustfmt::skip]
macro_rules! unlikely {
    ($e:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::macros::unlikely($e) }
    }};
}

#[rustfmt::skip]
macro_rules! assume {
    ($e:expr) => { $crate::macros::assume($e) }
}
