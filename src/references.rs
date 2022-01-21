use crate::{Changes, TrackableRef};
use core::any::TypeId;
use core::ops::{Deref, DerefMut};

impl<'a, T> TrackableRef<'a> for &'a T
where
    T: 'static,
{
    type Tracked = TrackedRef<'a, T>;

    #[inline]
    fn count_types() -> usize {
        1
    }

    #[inline]
    fn for_each_type(mut f: impl FnMut(TypeId, bool)) {
        f(TypeId::of::<T>(), false);
    }

    #[inline]
    fn into_tracked(self, changes: &'a Changes) -> Self::Tracked {
        TrackedRef::new(self, changes)
    }
}

impl<'a, T> TrackableRef<'a> for &'a mut T
where
    T: 'static,
{
    type Tracked = TrackedMut<'a, T>;

    #[inline]
    fn count_types() -> usize {
        1
    }

    #[inline]
    fn for_each_type(mut f: impl FnMut(TypeId, bool)) {
        f(TypeId::of::<T>(), true);
    }

    #[inline]
    fn into_tracked(self, changes: &'a Changes) -> Self::Tracked {
        TrackedMut::new(self, changes)
    }
}

pub struct TrackedRef<'a, T>
where
    T: 'static,
{
    value: &'a T,
    changes: &'a Changes,
}

impl<'a, T> TrackedRef<'a, T>
where
    T: 'static,
{
    #[inline]
    fn new(value: &'a T, changes: &'a Changes) -> Self {
        Self { value, changes }
    }
    #[inline]
    pub fn set_mutated(&self) {
        self.changes.set_changed(TypeId::of::<T>())
    }
}

pub struct TrackedMut<'a, T>
where
    T: 'static,
{
    value: &'a mut T,
    changes: &'a Changes,
}

impl<'a, T> TrackedMut<'a, T>
where
    T: 'static,
{
    fn new(value: &'a mut T, changes: &'a Changes) -> Self {
        Self { value, changes }
    }
    #[inline]
    pub fn set_mutated(&self) {
        self.changes.set_changed(TypeId::of::<T>())
    }
}

impl<'a, T> core::fmt::Debug for TrackedRef<'a, T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("TrackedRef<{}>", std::any::type_name::<T>()).as_str())
            .field("value", &self.value)
            .field("mutated", &self.changes.is_changed(TypeId::of::<T>()))
            .finish()
    }
}

impl<'a, T> core::fmt::Debug for TrackedMut<'a, T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("TrackedMut<{}>", std::any::type_name::<T>()).as_str())
            .field("value", &self.value)
            .field("mutated", &self.changes.is_changed(TypeId::of::<T>()))
            .finish()
    }
}

impl<'a, T> Deref for TrackedRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*(self.value)
    }
}

impl<'a, T> Deref for TrackedMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*(self.value)
    }
}

impl<'a, T> DerefMut for TrackedMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.set_mutated();
        &mut *(self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Changes, TrackableRef};
    use core::any::TypeId;

    #[test]
    fn tracked_ref_metadata() {
        type QueryType<'a> = &'a u32;

        assert_eq!(QueryType::count_types(), 1);

        let mut all_types = vec![];
        QueryType::for_each_type(|t, m| all_types.push((t, m)));
        assert_eq!(all_types.as_slice(), &[(TypeId::of::<u32>(), false)]);
    }

    #[test]
    fn tracked_mut_metadata() {
        type QueryType<'a> = &'a mut u32;

        assert_eq!(QueryType::count_types(), 1);

        let mut all_types = vec![];
        QueryType::for_each_type(|t, m| all_types.push((t, m)));
        assert_eq!(all_types.as_slice(), &[(TypeId::of::<u32>(), true)]);
    }

    #[test]
    fn tracked_ref_deref() {
        let value = 72u32;
        let reference = &value;
        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        let tracked = reference.into_tracked(&changes);

        let read_value: u32 = *tracked;
        assert_eq!(read_value, 72);
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), false);
    }

    #[test]
    fn tracked_ref_set() {
        let value = 72u32;
        let reference = &value;
        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        let tracked = reference.into_tracked(&changes);

        tracked.set_mutated();
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), true);
    }

    #[test]
    fn tracked_mut_set() {
        let mut value = 72u32;
        let reference = &mut value;
        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        let tracked = reference.into_tracked(&changes);

        tracked.set_mutated();
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), true);
    }

    #[test]
    fn tracked_mut_deref() {
        let mut value = 72u32;
        let reference = &mut value;
        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        let mut tracked = reference.into_tracked(&changes);

        let read_value: u32 = *tracked;
        assert_eq!(read_value, 72);
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), false);

        *tracked = 69;
        let read_value: u32 = *tracked;
        assert_eq!(read_value, 69);
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), true);

        assert_eq!(value, 69);
    }
}
