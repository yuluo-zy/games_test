//! 比较同一体块结构的完整解析、公共缓存命中和持续变更时的缓存失效成本。
//! 仅测 CPU 结构规则，不能推导 GPU 帧率或整游戏延迟。
use garden_domain::{
    BlockDraft, BlockId, Building, BuildingEdit, BuildingId, Facade, Roof, Stories, sample_building,
};
use garden_generation::structure::{BuildingRule, RuleCache, StructureRule};
use garden_geometry::Rect;
use std::{hint::black_box, time::Instant};

fn measure(mut run: impl FnMut(), iterations: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..iterations {
        run();
    }
    start.elapsed().as_nanos() as f64 / iterations as f64
}
fn main() {
    let mut draft = sample_building(BuildingId::new(1).unwrap(), 10., 3.);
    draft.blocks[0].footprint.depth = 10.;
    for (id, x) in [(2, 1.), (3, 5.)] {
        draft.blocks.push(BlockDraft {
            id: BlockId::new(id).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: Rect {
                x,
                z: 1.,
                width: 3.,
                depth: 3.,
            },
            height: 3.,
            stories: Stories::Locked(1),
            roof_intent: Roof::Hipped,
            facade: Facade::Stone,
        });
    }
    let building = Building::try_new(draft).unwrap();
    let changed = building
        .edited(BuildingEdit::SetRoof {
            block: BlockId::new(2).unwrap(),
            roof: Roof::Gabled,
        })
        .unwrap();
    let iterations = 100_000;
    let full = measure(
        || {
            black_box(BuildingRule.resolve(black_box(&building)));
        },
        iterations,
    );
    let mut cache = RuleCache::<BuildingRule>::default();
    black_box(cache.resolve(&building));
    let hit = measure(
        || {
            black_box(cache.resolve(black_box(&building)));
        },
        iterations,
    );
    let mut alternate = false;
    let miss = measure(
        || {
            alternate = !alternate;
            black_box(cache.resolve(black_box(if alternate { &changed } else { &building })));
        },
        iterations,
    );
    println!(
        "structure_rules: blocks=3 iterations={iterations} full_ns={full:.1} cache_hit_ns={hit:.1} cache_miss_ns={miss:.1}"
    );
}
