use core::any::TypeId;
pub trait TrackableRef<'a> {
    type Tracked: 'a;

    fn count_types() -> usize;

    /// Invoke `f` for every type that may be borrowed and whether the borrow is unique.
    /// The second argument of `f` is `true` if the component is borrowed mutable.
    fn for_each_type(f: impl FnMut(TypeId, bool));

    fn into_tracked(self, changes: &'a Changes) -> Self::Tracked;
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

mod changes;
mod option;
mod query;
mod references;
mod tuples;

pub use changes::Changes;
pub use references::{TrackedMut, TrackedRef};
