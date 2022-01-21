use crate::{Changes, TrackableRef};
use core::any::TypeId;

impl<'a, T> TrackableRef<'a> for Option<T>
where
    T: TrackableRef<'a>,
{
    type Tracked = Option<<T as TrackableRef<'a>>::Tracked>;

    fn count_types() -> usize {
        <T as TrackableRef>::count_types()
    }

    fn for_each_type(f: impl FnMut(TypeId, bool)) {
        <T as TrackableRef>::for_each_type(f)
    }

    fn into_tracked(self, changes: &'a Changes) -> Self::Tracked {
        self.map(|value| value.into_tracked(changes))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Changes, TrackableRef};
    use core::any::TypeId;

    #[test]
    fn tracked_option_metadata() {
        type QueryType<'a> = Option<&'a u32>;

        assert_eq!(QueryType::count_types(), 1);

        let mut all_types = vec![];
        QueryType::for_each_type(|t, m| all_types.push((t, m)));
        assert_eq!(all_types.as_slice(), &[(TypeId::of::<u32>(), false)]);
    }

    #[test]
    fn tracked_option_deref() {
        let mut value = 72u32;
        let reference = Some(&mut value);
        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        let mut tracked = reference.into_tracked(&changes);

        let read_value: Option<u32> = tracked.as_deref().cloned();
        assert_eq!(read_value, Some(72));
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), false);

        if let Some(tracked_value) = &mut tracked {
            **tracked_value = 69;
        }
        let read_value: Option<u32> = tracked.as_deref().cloned();
        assert_eq!(read_value, Some(69));
        assert_eq!(changes.is_changed(TypeId::of::<u32>()), true);

        assert_eq!(value, 69);
    }
}
