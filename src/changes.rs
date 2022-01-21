use crate::TrackableRef;
use core::iter::{IntoIterator, Iterator};
use core::{
    any::TypeId,
    sync::atomic::{AtomicBool, Ordering},
};
use std::collections::BTreeMap;

pub struct Changes {
    changes: BTreeMap<TypeId, AtomicBool>,
}

impl Changes {
    pub fn new() -> Self {
        Self {
            changes: BTreeMap::new(),
        }
    }
    pub fn new_for<'a, T: TrackableRef<'a>>() -> Self {
        let mut changes = Self::new();
        T::for_each_type(|t, _| changes.reserve(t));
        changes
    }

    pub fn reserve(&mut self, type_id: TypeId) {
        use std::collections::btree_map::Entry;
        match self.changes.entry(type_id) {
            Entry::Vacant(entry) => {
                entry.insert(AtomicBool::new(false));
            }
            Entry::Occupied(_) => (),
        }
    }

    pub fn reset(&mut self) {
        self.changes
            .iter_mut()
            .for_each(|(_, v)| v.store(false, Ordering::Relaxed));
    }

    pub fn for_each_changed(&self, mut f: impl FnMut(TypeId)) {
        self.changes.iter().for_each(|(t, c)| {
            if c.load(Ordering::Relaxed) {
                f(*t)
            }
        })
    }

    pub fn set_changed(&self, type_id: TypeId) {
        if let Some(value) = self.changes.get(&type_id) {
            value.store(true, Ordering::Relaxed);
        } else {
            panic!("Changed flag for type_id is not reserved");
        }
    }

    pub fn is_changed(&self, type_id: TypeId) -> bool {
        match self.changes.get(&type_id) {
            Some(value) => value.load(Ordering::Relaxed),
            None => false,
        }
    }

    pub fn iter(&self) -> ChangesIter<'_> {
        ChangesIter::new(self.changes.iter())
    }
}

type ChangesInnerIter<'a> = std::collections::btree_map::Iter<'a, TypeId, AtomicBool>;

pub struct ChangesIter<'a> {
    inner: ChangesInnerIter<'a>,
}

impl<'a> ChangesIter<'a> {
    fn new(inner: ChangesInnerIter<'a>) -> ChangesIter {
        Self { inner }
    }
}

impl<'a> Iterator for ChangesIter<'a> {
    type Item = (TypeId, bool);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();
        if let Some((type_id, atomic)) = next {
            Some((type_id.clone(), atomic.load(Ordering::Relaxed)))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Changes {
    type Item = (TypeId, bool);
    type IntoIter = ChangesIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
