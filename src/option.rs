use crate::Trackable;
use core::any::TypeId;

impl<'a, T, F> Trackable<F> for Option<T>
where
    F: Fn(TypeId) + 'a,
    T: Trackable<F> + 'a,
{
    type Tracked = Option<<T as Trackable<F>>::Tracked>;

    fn into_tracked(self, on_mut: F) -> Self::Tracked {
        self.map(|value| value.into_tracked(on_mut))
    }
}

#[cfg(test)]
mod tests {
    use crate::Trackable;
    use core::any::TypeId;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn tracked_option() {
        let mut value = 72u32;
        let reference = Some(&mut value);
        let mutated = AtomicBool::default();
        let on_mut = |type_id| {
            assert_eq!(type_id, TypeId::of::<u32>());
            mutated.store(true, Ordering::Relaxed);
        };
        let mut tracked = reference.into_tracked(on_mut);

        let read_value: Option<u32> = tracked.as_deref().cloned();
        assert_eq!(read_value, Some(72));
        assert_eq!(mutated.load(Ordering::Relaxed), false);

        if let Some(tracked_value) = &mut tracked {
            **tracked_value = 69;
        }
        let read_value: Option<u32> = tracked.as_deref().cloned();
        assert_eq!(read_value, Some(69));
        assert_eq!(mutated.load(Ordering::Relaxed), true);

        assert_eq!(value, 69);
    }
}
