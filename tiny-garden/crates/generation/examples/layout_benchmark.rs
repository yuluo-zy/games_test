//! CPU layout / mesh microbenchmarks, NOT frame time / Android performance.
use garden_domain::{
    BlockDraft, BlockId, Building, BuildingEdit, BuildingId, Facade, Roof, Stories, sample_building,
};
use garden_generation::compile;
use garden_generation::mesh::{GeometryProfile, compile_mesh};
use garden_geometry::Rect;
use std::{hint::black_box, time::Instant};

fn main() {
    let mut building =
        Building::try_new(sample_building(BuildingId::new(1).unwrap(), 10.0, 6.0)).unwrap();
    for (id, x) in [(2, 0.3), (3, 5.2)] {
        building = building
            .edited(BuildingEdit::AddBlock(BlockDraft {
                id: BlockId::new(id).unwrap(),
                parent: Some(BlockId::new(1).unwrap()),
                footprint: Rect {
                    x,
                    z: 0.3,
                    width: 4.0,
                    depth: 3.0,
                },
                height: 6.0,
                stories: Stories::Locked(2),
                roof_intent: Roof::Hipped,
                facade: Facade::Stone,
            }))
            .unwrap();
    }
    benchmark("stacked_10m", &building, 100_000);
    benchmark_mesh("stacked_10m", &building, 10_000);
    let mut large = sample_building(BuildingId::new(2).unwrap(), 100.0, 12.0);
    large.blocks[0].footprint.depth = 100.0;
    let large = Building::try_new(large).unwrap();
    benchmark("large_100m", &large, 100_000);
    benchmark_mesh("large_100m", &large, 10_000);
}

fn benchmark_mesh(label: &str, building: &Building, iterations: u32) {
    let layout = compile(building);
    let profile = GeometryProfile::default();
    for _ in 0..100 {
        black_box(compile_mesh(black_box(&layout), profile).unwrap());
    }
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(compile_mesh(black_box(&layout), profile).unwrap());
    }
    let elapsed = start.elapsed();
    let mesh = compile_mesh(&layout, profile).unwrap();
    println!(
        "mesh only ({label}): {} triangles, {} batches; {iterations} iterations; {:.3} us/building; {:?} total",
        mesh.triangles(),
        mesh.batches.len(),
        elapsed.as_secs_f64() * 1_000_000.0 / f64::from(iterations),
        elapsed
    );
}

fn benchmark(label: &str, building: &Building, iterations: u32) {
    for _ in 0..1_000 {
        black_box(compile(black_box(building)));
    }
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(compile(black_box(building)));
    }
    let elapsed = start.elapsed();
    let windows: usize = compile(building)
        .blocks
        .iter()
        .map(|b| b.windows.len())
        .sum();
    println!(
        "layout only ({label}): {} blocks, {windows} windows; {iterations} iterations; {:.3} us/building; {:?} total",
        building.blocks().len(),
        elapsed.as_secs_f64() * 1_000_000.0 / f64::from(iterations),
        elapsed
    );
}
