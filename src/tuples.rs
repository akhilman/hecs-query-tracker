use crate::Trackable;
use core::any::TypeId;

macro_rules! tracked_tuple_impl {
    ($($name: ident), *) => {
        impl<'a, OnMut, $($name),*> Trackable<OnMut> for ($($name,)*)
        where
            OnMut: Fn(TypeId) + Clone + 'a,
            $(
                $name: Trackable<OnMut> + 'a,
            )*
        {
            type Tracked = (
                $(
                    <$name as Trackable<OnMut>>::Tracked,
                )*
            );

            #[allow(unused_variables)]
            fn into_tracked(self, on_mut: OnMut) -> Self::Tracked {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                (
                    $(
                        $name.into_tracked(on_mut.clone()),
                    )*
                )
            }
        }
    }
}

#[rustfmt::skip]
smaller_tuples_too!(tracked_tuple_impl, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);
// smaller_tuples_too!(tracked_tuple_impl, B, A);

#[cfg(test)]
mod tests {
    use crate::Trackable;
    use core::any::TypeId;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn tracked_tuple() {
        let mut value = (Some(false), 72u32);
        let reference = (value.0.as_mut(), &mut value.1);
        let mutated = (AtomicBool::default(), AtomicBool::default());
        let on_mut = |type_id| {
            if type_id == TypeId::of::<bool>() {
                mutated.0.store(true, Ordering::Relaxed)
            } else if type_id == TypeId::of::<u32>() {
                mutated.1.store(true, Ordering::Relaxed)
            } else {
                unreachable!()
            }
        };
        let tracked = reference.into_tracked(&on_mut);

        let (mut a, mut b) = tracked;
        a.as_ref()
            .map_or_else(|| unreachable!("a is None"), |a| assert_eq!(**a, false));
        assert_eq!(*b, 72);
        assert_eq!(mutated.0.load(Ordering::Relaxed), false);
        assert_eq!(mutated.1.load(Ordering::Relaxed), false);

        *b = 69;
        assert_eq!(mutated.0.load(Ordering::Relaxed), false);
        assert_eq!(mutated.1.load(Ordering::Relaxed), true);

        a.as_mut().map(|a| **a = true);
        assert_eq!(mutated.0.load(Ordering::Relaxed), true);
        assert_eq!(mutated.1.load(Ordering::Relaxed), true);

        assert_eq!(value, (Some(true), 69u32));
    }
}
