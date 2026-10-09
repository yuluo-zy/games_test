//! Reproducible CPU comparison, NOT end-to-end screen-present latency.
use garden_domain::{
    BlockId, Building, BuildingEdit, BuildingId, Placement, Roof, sample_building,
};
use garden_generation::{
    compile,
    incremental::{Cancellation, PreparedBuilding},
    mesh::GeometryProfile,
};
use garden_presentation::building_kit::BuildingKit;
use std::time::Instant;
fn prepare(mesh: &mut PreparedBuilding, cancel: &Cancellation) {
    #[cfg(feature = "desktop")]
    garden_bevy::prepare::prepare(mesh, cancel).unwrap();
    #[cfg(not(feature = "desktop"))]
    let _ = (mesh, cancel);
}
fn main() {
    let kit = BuildingKit::warm_stone();
    let profile = GeometryProfile::default();
    let cancel = Cancellation::default();
    for scenario in ["move", "roof", "height"] {
        let mut building =
            Building::try_new(sample_building(BuildingId::new(1).unwrap(), 10., 6.)).unwrap();
        let mut baseline = kit
            .compile_incremental(&mut compile(&building), profile, None, &cancel)
            .unwrap();
        prepare(&mut baseline, &cancel);
        let mut full_times = Vec::new();
        let mut delta_times = Vec::new();
        let mut generated = 0;
        let mut total = 0;
        let mut reused = 0;
        for i in 0..30 {
            let edit = match scenario {
                "move" => BuildingEdit::Move(Placement {
                    x: i as f64 + 1.,
                    z: 4.,
                    elevation: 0.,
                    yaw: 0.2,
                }),
                "roof" => BuildingEdit::SetRoof {
                    block: BlockId::new(1).unwrap(),
                    roof: if i % 2 == 0 { Roof::Flat } else { Roof::Gabled },
                },
                _ => BuildingEdit::Resize {
                    block: BlockId::new(1).unwrap(),
                    footprint: building.blocks()[0].footprint,
                    height: [9., 12., 15., 6.][i % 4],
                },
            };
            building = building.edited(edit).unwrap();
            let start = Instant::now();
            let mut full =
                PreparedBuilding::whole(kit.compile(&mut compile(&building), profile).unwrap());
            prepare(&mut full, &cancel);
            full_times.push(start.elapsed().as_secs_f64() * 1000.);
            let start = Instant::now();
            let mut delta = kit
                .compile_incremental(&mut compile(&building), profile, Some(&baseline), &cancel)
                .unwrap();
            prepare(&mut delta, &cancel);
            delta_times.push(start.elapsed().as_secs_f64() * 1000.);
            generated += delta.generated_vertices;
            total += full.vertices();
            reused += delta.reused_parts;
            assert_eq!(full.triangles(), delta.triangles());
            baseline = delta;
        }
        full_times.sort_by(f64::total_cmp);
        delta_times.sort_by(f64::total_cmp);
        println!(
            "{scenario}: CPU full p50={:.3}ms p95={:.3}ms; incremental p50={:.3}ms p95={:.3}ms; generated vertices={generated}/{total}; reused parts={reused}; batches={}",
            full_times[15],
            full_times[28],
            delta_times[15],
            delta_times[28],
            baseline.batches.len()
        );
    }
}
