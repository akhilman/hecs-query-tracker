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
| Read only | 10 | [34.974 ns 35.310 ns 35.705 ns] | [89.549 ns 90.037 ns 90.657 ns] |
| Read/Write | 10 | [30.151 ns 30.261 ns 30.403 ns] | [124.67 ns 124.96 ns 125.27 ns] |
| Read only | 1000 | [307.72 ns 312.63 ns 318.22 ns] | [364.46 ns 369.00 ns 376.36 ns] |
| Read/Write | 1000 | [404.20 ns 406.38 ns 409.26 ns] | [3.7053 us 3.7798 us 3.8526 us] |
