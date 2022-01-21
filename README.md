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

    <(&mut i32, &mut u32)>::track(&changes)
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

## Benchmark

| Benchmark | Entities | Unracked | Tracked |
| :-: | :-: | :-: | :-: |
| Read only | 10 | [35.549 ns 35.844 ns 36.212 ns] | [28.424 ns 28.580 ns 28.753 ns] |
| Read/Write | 10 | [37.867 ns 38.245 ns 38.617 ns] | [69.694 ns 70.304 ns 71.073 ns] |
| Read only | 1000 | [300.27 ns 302.32 ns 304.76 ns] | [296.02 ns 297.02 ns 298.32 ns] |
| Read/Write | 1000 | [388.21 ns 389.70 ns 391.45 ns] | [3.6327 us 3.6752 us 3.7111 us] |
