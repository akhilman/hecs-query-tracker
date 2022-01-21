use crate::{Changes, TrackableRef};
use core::iter::{IntoIterator, Iterator};
use hecs::{Entity, Query, QueryBorrow, QueryItem, QueryIter};

pub struct TrackedQueryBorrow<'w, Q>
where
    Q: Query,
    QueryItem<'w, Q>: TrackableRef<'w>,
{
    inner: QueryBorrow<'w, Q>,
    changes: &'w Changes,
}

impl<'w, Q> TrackedQueryBorrow<'w, Q>
where
    Q: Query,
    QueryItem<'w, Q>: TrackableRef<'w>,
{
    pub fn new(inner: QueryBorrow<'w, Q>, changes: &'w Changes) -> Self {
        Self { inner, changes }
    }

    // The lifetime narrowing here is required for soundness.
    pub fn iter(&mut self) -> TrackedQueryIter<'_, Q> {
        TrackedQueryIter::new(self.inner.iter(), self.changes)
    }
}

impl<'q, Q> IntoIterator for &'q mut TrackedQueryBorrow<'q, Q>
where
    Q: Query,
    QueryItem<'q, Q>: TrackableRef<'q>,
{
    type IntoIter = TrackedQueryIter<'q, Q>;
    type Item = (Entity, <QueryItem<'q, Q> as TrackableRef<'q>>::Tracked);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct TrackedQueryIter<'q, Q>
where
    Q: Query,
{
    inner: QueryIter<'q, Q>,
    changes: &'q Changes,
}

impl<'q, Q> TrackedQueryIter<'q, Q>
where
    Q: Query,
{
    fn new(inner: QueryIter<'q, Q>, changes: &'q Changes) -> Self {
        Self { inner, changes }
    }
}

impl<'q, Q> Iterator for TrackedQueryIter<'q, Q>
where
    Q: Query,
    QueryItem<'q, Q>: TrackableRef<'q>,
{
    type Item = (Entity, <QueryItem<'q, Q> as TrackableRef<'q>>::Tracked);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(entity, components)| (entity, components.into_tracked(self.changes)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'q, Q> ExactSizeIterator for TrackedQueryIter<'q, Q>
where
    Q: Query,
    QueryItem<'q, Q>: TrackableRef<'q>,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::TrackedQueryBorrow;
    use crate::Changes;
    use core::any::TypeId;
    use hecs::*;

    #[test]
    fn tracked_query() {
        fn nullify_ten_plus(world: &World) -> Vec<TypeId> {
            let mut changes = Changes::new();
            changes.reserve(TypeId::of::<u32>());
            changes.reserve(TypeId::of::<i32>());

            let query = world.query::<(&mut u32, &mut i32)>();
            let mut tracked_query = TrackedQueryBorrow::new(query, &changes);

            tracked_query.iter().for_each(|(_, (mut a, mut b))| {
                if *a >= 10 {
                    *a = 0;
                }
                if *b >= 10 {
                    *b = 0;
                }
            });

            changes
                .iter()
                .filter_map(|item| match item {
                    (type_id, true) => Some(type_id),
                    _ => None,
                })
                .collect()
        }

        let mut world = World::default();
        world.spawn((0u32, 0i32));
        world.spawn((1u32, 1i32));
        let changes = nullify_ten_plus(&world);
        assert!(changes.is_empty());

        world.spawn((10u32, 3i32));
        let changes = nullify_ten_plus(&world);
        assert_eq!(changes.len(), 1);
        assert!(changes.contains(&TypeId::of::<u32>()));

        world.spawn((1u32, 11i32));
        let changes = nullify_ten_plus(&world);
        assert_eq!(changes.len(), 1);
        assert!(changes.contains(&TypeId::of::<i32>()));

        world.spawn((12u32, 13i32));
        let changes = nullify_ten_plus(&world);
        assert_eq!(changes.len(), 2);
        assert!(changes.contains(&TypeId::of::<i32>()));
        assert!(changes.contains(&TypeId::of::<u32>()));
    }
}
