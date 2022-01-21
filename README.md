# Query tracker for hecs

This crate allows you to track whether the query has changed the components. 

```rust
use core::any::{type_name, TypeId};
use hecs::World;
use hecs_query_tracker::{Changes, TrackableQuery};

fn main() {
    let mut world = World::default();
    world.spawn((1i32, 2u32));

    let changes = Changes::new_for::<(&i32, &u32)>();

    <(&'static mut i32, &'static mut u32)>::track(&changes)
        .query(&world)
        .iter()
        .for_each(|(_, (mut a, b))| *a = *b as i32);

    for (type_id, type_name) in [
        (TypeId::of::<i32>(), type_name::<i32>()),
        (TypeId::of::<u32>(), type_name::<u32>()),
    ] {
        if changes.is_changed(type_id) {
            println!("{} is changed", type_name);
        } else {
            println!("{} is not changed", type_name);
        }
    }
}
```
