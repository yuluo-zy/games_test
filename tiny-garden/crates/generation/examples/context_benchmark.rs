//! 固定规模的上下文 CPU 基准；比较冷解析与精确缓存命中，不声称 GPU 帧率。
use garden_domain::{Building, BuildingId, context::*, sample_building, strokes::*, terrain::*};
use garden_generation::{context::*, context_cache::ContextCache};
use std::{hint::black_box, sync::Arc, time::Instant};
fn main() {
    let mut input = ContextInput {
        buildings: Vec::new(),
        strokes: Vec::new(),
        scene: Arc::default(),
    };
    for i in 0..6 {
        let mut draft = sample_building(BuildingId::new(i + 1).unwrap(), 6., 3.);
        draft.placement.x = (i % 3) as f64 * 12. - 18.;
        draft.placement.z = (i / 3) as f64 * 12. - 9.;
        input
            .buildings
            .push(Arc::new(Building::try_new(draft).unwrap()));
    }
    for i in 0..32 {
        let points = (0..128)
            .map(|n| Point {
                x: -20. + n as f64 * 0.25,
                z: -12. + i as f64 * 0.7,
            })
            .collect();
        input.strokes.push(Arc::new(Stroke {
            id: StrokeId(i + 1),
            kind: StrokeKind::Path,
            points,
            width: 0.4,
            height: 0.,
        }));
    }
    let scene = Arc::make_mut(&mut input.scene);
    for i in 0..4 {
        scene.terrain = scene
            .terrain
            .brushed(BrushSample {
                center: Point { x: -15., z: -6. },
                radius: 2.,
                amount: 0.2,
                kind: BrushKind::Raise,
            })
            .unwrap();
        black_box(i);
    }
    let start = Instant::now();
    let cold = black_box(resolve(&input));
    let cold_ms = start.elapsed().as_secs_f64() * 1000.;
    let mut cache = ContextCache::default();
    black_box(resolve_cached(&input, &mut cache));
    let count = 50;
    let start = Instant::now();
    for _ in 0..count {
        black_box(resolve_cached(&input, &mut cache));
    }
    println!(
        "strokes=32 points=4096 buildings=6 edges={} cold_ms={:.3} cached_ms={:.3} counters={:?}",
        cold.network.edges.len(),
        cold_ms,
        start.elapsed().as_secs_f64() * 1000. / count as f64,
        cache.stats
    );
    let b = &input.buildings[0];
    let host = WallAnchor {
        building: b.id(),
        block: b.blocks()[0].id,
        face: WallFace::Front,
    };
    Arc::make_mut(&mut input.scene).openings.insert(
        OpeningId(1),
        OpeningIntent {
            id: OpeningId(1),
            host,
            along: 2.,
            elevation: 1.4,
            width: 0.9,
            height: 1.2,
            style: 0,
        },
    );
    let start = Instant::now();
    black_box(resolve_cached(&input, &mut cache));
    println!(
        "window_edit_ms={:.3} counters={:?}",
        start.elapsed().as_secs_f64() * 1000.,
        cache.stats
    );
}
