use core::any::TypeId;
pub trait Trackable<F>
where
    F: Fn(TypeId),
{
    type Tracked;

    fn into_tracked(self, on_mut: F) -> Self::Tracked;
}

/// Imagine macro parameters, but more like those Russian dolls.
///
/// Calls m!(A, B, C), m!(A, B), m!(B), and m!() for i.e. (m, A, B, C)
/// where m is any macro, for any number of parameters.
macro_rules! smaller_tuples_too {
    ($m: ident, $ty: ident) => {
        $m!{}
        $m!{$ty}
    };
    ($m: ident, $ty: ident, $($tt: ident),*) => {
        smaller_tuples_too!{$m, $($tt),*}
        $m!{$ty, $($tt),*}
    };
}

mod option;
mod query;
mod references;
mod tuples;

pub use references::{TrackedMut, TrackedRef};
