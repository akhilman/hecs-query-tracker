use crate::Trackable;
use core::any::TypeId;
use core::iter::{IntoIterator, Iterator};
use hecs::{Entity, Query, QueryBorrow, QueryItem, QueryIter};

impl<'w, Q, F> Trackable<F> for QueryBorrow<'w, Q>
where
    Q: Query,
    F: 'w + Fn(TypeId) + Clone,
    QueryItem<'w, Q>: Trackable<F>,
{
    type Tracked = TrackedQueryBorrow<'w, Q, F>;

    fn into_tracked(self, on_mut: F) -> Self::Tracked {
        TrackedQueryBorrow::new(self, on_mut)
    }
}

pub struct TrackedQueryBorrow<'w, Q, F>
where
    Q: Query,
    F: 'w + Fn(TypeId) + Clone,
    QueryItem<'w, Q>: Trackable<F>,
{
    inner: QueryBorrow<'w, Q>,
    on_mut: F,
}

impl<'w, Q, F> TrackedQueryBorrow<'w, Q, F>
where
    Q: Query,
    F: 'w + Fn(TypeId) + Clone,
    QueryItem<'w, Q>: Trackable<F>,
{
    fn new(inner: QueryBorrow<'w, Q>, on_mut: F) -> Self {
        Self { inner, on_mut }
    }

    // The lifetime narrowing here is required for soundness.
    pub fn iter(&mut self) -> TrackedQueryIter<'_, Q, F> {
        let iter = self.inner.iter();
        TrackedQueryIter::new(iter, self.on_mut.clone())
    }
}

impl<'q, Q, F> IntoIterator for &'q mut TrackedQueryBorrow<'q, Q, F>
where
    Q: Query,
    F: 'q + Fn(TypeId) + Clone,
    QueryItem<'q, Q>: Trackable<F>,
{
    type IntoIter = TrackedQueryIter<'q, Q, F>;
    type Item = (Entity, <QueryItem<'q, Q> as Trackable<F>>::Tracked);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct TrackedQueryIter<'q, Q, F>
where
    Q: Query,
    F: 'q + Fn(TypeId) + Clone,
{
    inner: QueryIter<'q, Q>,
    on_mut: F,
}

impl<'q, Q, F> TrackedQueryIter<'q, Q, F>
where
    Q: Query,
    F: 'q + Fn(TypeId) + Clone,
{
    fn new(inner: QueryIter<'q, Q>, on_mut: F) -> Self {
        Self { inner, on_mut }
    }
}

impl<'q, Q, F> Iterator for TrackedQueryIter<'q, Q, F>
where
    Q: Query,
    F: 'q + Fn(TypeId) + Clone,
    QueryItem<'q, Q>: Trackable<F>,
{
    type Item = (Entity, <QueryItem<'q, Q> as Trackable<F>>::Tracked);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(entity, components)| (entity, components.into_tracked(self.on_mut.clone())))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'q, Q, F> ExactSizeIterator for TrackedQueryIter<'q, Q, F>
where
    Q: Query,
    F: 'q + Fn(TypeId) + Clone,
    QueryItem<'q, Q>: Trackable<F>,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Trackable;
    use core::any::TypeId;
    use hecs::*;
    use std::cell::RefCell;
    use std::collections::BTreeSet;

    #[test]
    fn tracked_query() {
        fn nullify_ten_plus(world: &mut World, on_mut: impl Fn(TypeId) + Clone) {
            world
                .query::<(&mut u32, &mut i32)>()
                .into_tracked(on_mut)
                .iter()
                .for_each(|(_, (mut a, mut b))| {
                    if *a >= 10 {
                        *a = 0;
                    }
                    if *b >= 10 {
                        *b = 0;
                    }
                });
        }

        let mut world = World::default();
        let changes = RefCell::new(BTreeSet::new());
        let on_mut = |type_id| {
            changes.borrow_mut().insert(type_id);
        };

        world.spawn((0u32, 0i32));
        world.spawn((1u32, 1i32));
        nullify_ten_plus(&mut world, &on_mut);
        assert!(changes.borrow().is_empty());

        changes.borrow_mut().clear();
        world.spawn((10u32, 3i32));
        nullify_ten_plus(&mut world, &on_mut);
        assert_eq!(changes.borrow().len(), 1);
        assert!(changes.borrow().contains(&TypeId::of::<u32>()));

        changes.borrow_mut().clear();
        world.spawn((1u32, 11i32));
        nullify_ten_plus(&mut world, &on_mut);
        assert_eq!(changes.borrow().len(), 1);
        assert!(changes.borrow().contains(&TypeId::of::<i32>()));

        changes.borrow_mut().clear();
        world.spawn((12u32, 13i32));
        nullify_ten_plus(&mut world, &on_mut);
        assert_eq!(changes.borrow().len(), 2);
        assert!(changes.borrow().contains(&TypeId::of::<i32>()));
        assert!(changes.borrow().contains(&TypeId::of::<u32>()));
    }
}
