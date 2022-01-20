use crate::Trackable;
use core::any::TypeId;
use core::ops::{Deref, DerefMut};

impl<'a, T, F> Trackable<F> for &'a T
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    type Tracked = TrackedRef<'a, T, F>;

    fn into_tracked(self, on_mut: F) -> Self::Tracked {
        TrackedRef::new(self, on_mut)
    }
}

impl<'a, T, F> Trackable<F> for &'a mut T
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    type Tracked = TrackedMut<'a, T, F>;

    fn into_tracked(self, on_mut: F) -> Self::Tracked {
        TrackedMut::new(self, on_mut)
    }
}

pub struct TrackedRef<'a, T, F>
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    value: &'a T,
    on_mut: F,
}

impl<'a, T, F> TrackedRef<'a, T, F>
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    fn new(value: &'a T, on_mut: F) -> Self {
        Self { value, on_mut }
    }
    pub fn set_mutated(&self) {
        (self.on_mut)(TypeId::of::<T>())
    }
}

pub struct TrackedMut<'a, T, F>
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    value: &'a mut T,
    on_mut: F,
}

impl<'a, T, F> TrackedMut<'a, T, F>
where
    T: 'static,
    F: Fn(TypeId) + 'a,
{
    fn new(value: &'a mut T, on_mut: F) -> Self {
        Self { value, on_mut }
    }
    pub fn set_mutated(&self) {
        (self.on_mut)(TypeId::of::<T>())
    }
}

impl<'a, T, F> core::fmt::Debug for TrackedRef<'a, T, F>
where
    T: core::fmt::Debug,
    F: Fn(TypeId) + 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("TrackedRef<{}>", std::any::type_name::<T>()).as_str())
            .field("value", &self.value)
            .finish()
    }
}

impl<'a, T, F> core::fmt::Debug for TrackedMut<'a, T, F>
where
    T: core::fmt::Debug,
    F: Fn(TypeId) + 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("TrackedMut<{}>", std::any::type_name::<T>()).as_str())
            .field("value", &self.value)
            .finish()
    }
}

impl<'a, T, F> Deref for TrackedRef<'a, T, F>
where
    F: Fn(TypeId) + 'a,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*(self.value)
    }
}

impl<'a, T, F> Deref for TrackedMut<'a, T, F>
where
    F: Fn(TypeId) + 'a,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*(self.value)
    }
}

impl<'a, T, F> DerefMut for TrackedMut<'a, T, F>
where
    F: Fn(TypeId) + 'a,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.set_mutated();
        &mut *(self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Trackable;
    use core::any::TypeId;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn tracked_ref() {
        let value = 72u32;
        let reference = &value;
        let mutated = AtomicBool::default();
        let on_mut = |type_id| {
            assert_eq!(type_id, TypeId::of::<u32>());
            mutated.store(true, Ordering::Relaxed);
        };
        let tracked = reference.into_tracked(on_mut);

        let read_value: u32 = *tracked;
        assert_eq!(read_value, 72);
        assert_eq!(mutated.load(Ordering::Relaxed), false);

        tracked.set_mutated();
        assert_eq!(mutated.load(Ordering::Relaxed), true);
    }

    #[test]
    fn tracked_mut() {
        let mut value = 72u32;
        let reference = &mut value;
        let mutated = AtomicBool::default();
        let on_mut = |type_id| {
            assert_eq!(type_id, TypeId::of::<u32>());
            mutated.store(true, Ordering::Relaxed);
        };
        let mut tracked = reference.into_tracked(on_mut);

        let read_value: u32 = *tracked;
        assert_eq!(read_value, 72);
        assert_eq!(mutated.load(Ordering::Relaxed), false);

        tracked.set_mutated();
        assert_eq!(mutated.load(Ordering::Relaxed), true);

        mutated.store(false, Ordering::Relaxed);
        *tracked = 69;
        let read_value: u32 = *tracked;
        assert_eq!(read_value, 69);
        assert_eq!(mutated.load(Ordering::Relaxed), true);

        assert_eq!(value, 69);
    }
}
