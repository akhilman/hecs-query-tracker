mod query;
use crate::{Changes, TrackableRef};
use core::marker::PhantomData;
use hecs::{Query, QueryItem, World};

pub use query::{TrackedQueryBorrow, TrackedQueryIter};

pub trait TrackableQuery
where
    Self: Query + Sized,
{
    fn track<'a>(changes: &'a Changes) -> TrackedQueryBuilder<'a, Self>
    where
        QueryItem<'a, Self>: TrackableRef<'a>,
    {
        TrackedQueryBuilder::<'a, Self>::new(changes)
    }
}

impl<'a, Q> TrackableQuery for Q where Q: 'a + Query {}

pub struct TrackedQueryBuilder<'a, Q>
where
    Q: 'a + Query,
{
    changes: &'a Changes,
    phantom: PhantomData<&'a Q>,
}

impl<'a, Q> TrackedQueryBuilder<'a, Q>
where
    Q: 'a + Query,
{
    fn new(changes: &'a Changes) -> Self {
        Self {
            changes,
            phantom: PhantomData,
        }
    }

    pub fn query<'w>(&self, world: &'w World) -> TrackedQueryBorrow<'w, Q>
    where
        'a: 'w,
        QueryItem<'w, Q>: TrackableRef<'w>,
    {
        TrackedQueryBorrow::new(world.query::<Q>(), self.changes)
    }
}

#[cfg(test)]
mod tests {
    use super::TrackableQuery;
    use crate::Changes;
    use core::any::TypeId;
    use hecs::World;

    #[test]
    fn query_builder() {
        let mut world = World::default();
        world.spawn((0i32, 0u32));

        let mut changes = Changes::new();
        changes.reserve(TypeId::of::<u32>());
        changes.reserve(TypeId::of::<i32>());

        <(&'static mut i32, &'static mut u32)>::track(&changes)
            .query(&world)
            .iter()
            .for_each(|(_, (mut a, b))| *a = *b as i32);

        let changes: Vec<_> = changes
            .iter()
            .filter_map(|item| match item {
                (type_id, true) => Some(type_id),
                _ => None,
            })
            .collect();
        assert_eq!(changes.len(), 1);
        assert!(changes.contains(&TypeId::of::<i32>()));
    }
}
