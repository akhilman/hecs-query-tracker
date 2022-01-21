use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hecs::World;
use hecs_query_tracker::{Changes, TrackableQuery};

fn bench_compare_untracked(world: &World) {
    world
        .query::<(&mut u64, &mut u32)>()
        .iter()
        .for_each(|(_, (a, b))| {
            let _ = *a == *b as u64;
        });
}

fn bench_compare_tracked(world: &World, changes: &Changes) {
    <(&mut u64, &mut u32)>::track(&changes)
        .query(&world)
        .iter()
        .for_each(|(_, (a, b))| {
            let _ = *a == *b as u64;
        });
}

fn bench_copy_untracked(world: &World) {
    world
        .query::<(&mut u64, &mut u32)>()
        .iter()
        .for_each(|(_, (a, b))| *a = *b as u64);
}

fn bench_copy_tracked(world: &World, changes: &Changes) {
    <(&mut u64, &mut u32)>::track(&changes)
        .query(&world)
        .iter()
        .for_each(|(_, (mut a, b))| *a = *b as u64);
}

pub fn tracked_vs_untracked(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tracked vs untracked");

    let changes = Changes::new_for::<(&u64, &u32)>();

    for count in [10, 100, 1000, 10000] {
        let mut world = World::default();
        for n in 1..=count {
            world.spawn((n as u64, n as u32 + 1));
        }
        group.bench_function(BenchmarkId::new("Untracked compare", count), |b| {
            b.iter(|| bench_compare_untracked(&world))
        });
        group.bench_function(BenchmarkId::new("Tracked compare", count), |b| {
            b.iter(|| bench_compare_tracked(&world, &changes))
        });

        group.bench_function(BenchmarkId::new("Untracked copy", count), |b| {
            b.iter(|| bench_copy_untracked(&world))
        });
        group.bench_function(BenchmarkId::new("Tracked copy", count), |b| {
            b.iter(|| bench_copy_tracked(&world, &changes))
        });
    }
}

criterion_group!(benches, tracked_vs_untracked);
criterion_main!(benches);
