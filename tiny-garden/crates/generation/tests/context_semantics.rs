//! 跨解析阶段的行为回归，验证来源、约束及局部缓存，不复刻解析实现。
use garden_domain::{Building, BuildingId, context::*, sample_building, strokes::*, terrain::*};
use garden_generation::{
    context::*,
    context_cache::ContextCache,
    context_mesh::{DerivedGroup, GroupRegistry},
};
use std::sync::Arc;
fn path(id: u64, points: Vec<Point>) -> Arc<Stroke> {
    Arc::new(Stroke {
        id: StrokeId(id),
        kind: StrokeKind::Path,
        points,
        width: 0.8,
        height: 0.,
    })
}
fn p(x: f64, z: f64) -> Point {
    Point { x, z }
}
fn scene(strokes: Vec<Arc<Stroke>>) -> ContextInput {
    ContextInput {
        buildings: Vec::new(),
        strokes,
        scene: Arc::default(),
    }
}
#[test]
fn endpoint_t_and_overlap_keep_stable_sources() {
    let a = path(1, vec![p(-3., 0.), p(3., 0.)]);
    let mut input = scene(vec![a.clone(), path(2, vec![p(0., 0.1), p(0., 3.)])]);
    let first = resolve(&input);
    assert_eq!(first.network.edges.len(), 3);
    assert!(first.network.nodes.iter().any(|n| n.sources.len() == 2));
    Arc::make_mut(&mut input.scene).decisions = first.decisions;
    input.strokes[1] = path(2, vec![p(0., 0.24), p(0., 3.)]);
    assert_eq!(resolve(&input).network.edges.len(), 3);
    Arc::make_mut(&mut input.scene).decisions = DecisionState::default();
    assert_eq!(resolve(&input).network.edges.len(), 2);
    let overlap = resolve(&scene(vec![a, path(3, vec![p(0., 0.), p(4., 0.)])]));
    let shared = overlap
        .network
        .edges
        .iter()
        .find(|e| e.contributions.len() == 2)
        .unwrap();
    assert_eq!(
        shared
            .contributions
            .iter()
            .map(|c| c.0.stroke)
            .collect::<Vec<_>>(),
        vec![StrokeId(1), StrokeId(3)]
    );
    let end = resolve(&scene(vec![
        path(1, vec![p(-3., 0.), p(0., 0.)]),
        path(2, vec![p(0.1, 0.), p(3., 1.)]),
    ]));
    assert_eq!(end.network.nodes.len(), 3);
}
#[test]
fn steep_ground_blocks_passage_and_pick_samples_actual_triangles() {
    let mut input = scene(vec![path(1, vec![p(-1., 0.), p(1., 0.)])]);
    let mut terrain = TerrainDocument::default();
    for _ in 0..4 {
        terrain = terrain
            .brushed(BrushSample {
                center: p(0., 0.),
                radius: 0.25,
                amount: 0.25,
                kind: BrushKind::Raise,
            })
            .unwrap();
    }
    let hit = terrain_hit([0., 5., 0.], [0., -1., 0.], &terrain).unwrap();
    assert!((hit[1] - terrain.height(p(0., 0.))).abs() < 1e-9);
    Arc::make_mut(&mut input.scene).terrain = terrain;
    let resolved = resolve(&input);
    assert!(resolved.network.edges.iter().all(|e| !e.passable));
    assert!(
        resolved
            .diagnostics
            .contains(&Diagnostic::Steep(StrokeId(1)))
    );
}
fn house() -> Arc<Building> {
    Arc::new(Building::try_new(sample_building(BuildingId::new(1).unwrap(), 10., 3.)).unwrap())
}
fn host(b: &Building) -> WallAnchor {
    WallAnchor {
        building: b.id(),
        block: b.blocks()[0].id,
        face: WallFace::Front,
    }
}
fn window(id: u64, host: WallAnchor, along: f64) -> OpeningIntent {
    OpeningIntent {
        id: OpeningId(id),
        host,
        along,
        elevation: 1.4,
        width: 0.9,
        height: 1.2,
        style: 0,
    }
}
#[test]
fn assembly_limits_styles_sleeping_and_hysteresis_are_explicit() {
    let b = house();
    let h = host(&b);
    let mut input = scene(Vec::new());
    input.buildings.push(b);
    for id in 1..=5 {
        Arc::make_mut(&mut input.scene)
            .openings
            .insert(OpeningId(id), window(id, h, 0.8 + (id - 1) as f64));
    }
    let out = resolve(&input);
    assert_eq!(
        out.facades[&h]
            .assemblies
            .iter()
            .map(|a| a.members.len())
            .collect::<Vec<_>>(),
        vec![4, 1]
    );
    Arc::make_mut(&mut input.scene).decisions = out.decisions;
    Arc::make_mut(&mut input.scene)
        .openings
        .get_mut(&OpeningId(2))
        .unwrap()
        .along = 1.94;
    assert!(
        resolve(&input).facades[&h].assemblies[0]
            .members
            .contains(&OpeningId(2))
    );
    Arc::make_mut(&mut input.scene)
        .openings
        .get_mut(&OpeningId(2))
        .unwrap()
        .style = 1;
    assert_eq!(
        resolve(&input).facades[&h].assemblies[0].members,
        vec![OpeningId(1)]
    );
    input.buildings.clear();
    assert_eq!(
        resolve(&input)
            .diagnostics
            .iter()
            .filter(|d| matches!(d, Diagnostic::Sleeping(_)))
            .count(),
        5
    );
}
#[test]
fn window_and_remote_terrain_changes_reuse_foundations_and_network() {
    let b = house();
    let h = host(&b);
    let mut input = scene(vec![path(1, vec![p(-3., -3.), p(3., -3.)])]);
    input.buildings.push(b);
    Arc::make_mut(&mut input.scene).terrain = TerrainDocument::default()
        .brushed(BrushSample {
            center: p(2., 2.),
            radius: 1.,
            amount: 0.2,
            kind: BrushKind::Raise,
        })
        .unwrap();
    let mut cache = ContextCache::default();
    let initial = resolve_cached(&input, &mut cache);
    assert_eq!(cache.stats.foundations_built, 1);
    Arc::make_mut(&mut input.scene)
        .openings
        .insert(OpeningId(1), window(1, h, 2.));
    let out = resolve_cached(&input, &mut cache);
    assert_eq!(cache.stats.foundations_built, 0);
    assert_eq!(cache.stats.network_built, 0);
    assert_eq!(cache.stats.facades_built, 1);
    assert_eq!(out, resolve(&input));
    let terrain = input
        .scene
        .terrain
        .brushed(BrushSample {
            center: p(20., 12.),
            radius: 1.,
            amount: 0.2,
            kind: BrushKind::Raise,
        })
        .unwrap();
    Arc::make_mut(&mut input.scene).terrain = terrain;
    assert_eq!(
        resolve_cached(&input, &mut cache).foundations,
        initial.foundations
    );
    assert_eq!(cache.stats.foundations_built, 0);
}
#[test]
fn typed_group_registry_cannot_alias_large_host_ids_and_released_groups() {
    let mut ids = GroupRegistry::default();
    let a = ids.id(DerivedGroup::Foundation(BuildingId::new(u64::MAX).unwrap()));
    let b = ids.id(DerivedGroup::Foundation(
        BuildingId::new((1 << 61) - 1).unwrap(),
    ));
    assert_ne!(a, b);
    ids.finish();
    ids.begin();
    ids.finish();
    ids.begin();
    assert_ne!(
        a,
        ids.id(DerivedGroup::Foundation(BuildingId::new(u64::MAX).unwrap()))
    );
}
#[test]
fn rules_reject_invalid_hysteresis_and_nonfinite_values() {
    let rules = ContextRules {
        connect_exit: 0.01,
        ..Default::default()
    };
    assert!(rules.validate().is_err());
    let rules = ContextRules {
        wall_enter: f64::NAN,
        ..Default::default()
    };
    assert!(rules.validate().is_err());
}

#[test]
fn automatic_door_parts_follow_sources_when_an_earlier_door_is_removed() {
    let b = house();
    let h = host(&b);
    let mut input = scene(vec![
        path(1, vec![p(2., -2.), p(2., 2.)]),
        path(2, vec![p(7., -2.), p(7., 2.)]),
    ]);
    input.buildings.push(b);
    let first = resolve(&input);
    assert_eq!(first.facades[&h].doors.len(), 2);
    let mut parts = OpeningParts::default();
    parts.update(&first);
    let keep = parts.slot(h, &first.facades[&h].doors[1]);
    input.strokes.remove(0);
    let next = resolve(&input);
    parts.update(&next);
    assert_eq!(parts.slot(h, &next.facades[&h].doors[0]), keep);
}
#[test]
fn boundary_endpoints_create_one_shared_corner_and_keep_original_intents() {
    let mut input = scene(Vec::new());
    for (id, a, b) in [
        (1, p(-3., 0.), p(0., 0.)),
        (2, p(0.1, 0.), p(0., 3.)),
        (3, p(0.05, -0.05), p(3., 0.)),
    ] {
        let mut stroke = path(id, vec![a, b]);
        Arc::make_mut(&mut stroke).kind = StrokeKind::Fence;
        Arc::make_mut(&mut stroke).height = 1.;
        Arc::make_mut(&mut stroke).width = 0.4;
        input.strokes.push(stroke);
    }
    let out = resolve(&input);
    assert_eq!(out.boundaries.len(), 1);
    assert_eq!(out.boundaries[0].members.len(), 3);
    assert_eq!(input.strokes[0].points[1], p(0., 0.));
    for line in out.linear.values() {
        let mesh = garden_generation::context_mesh::boundary(
            line,
            &[],
            &out.boundaries,
            &input.scene.terrain,
            &garden_generation::incremental::Cancellation::default(),
        )
        .unwrap();
        for batch in mesh.batches {
            batch.data.validate().unwrap();
        }
    }
}

#[test]
fn foundation_closes_the_horizontal_floor_above_an_inner_depression() {
    let b = house();
    let terrain = TerrainDocument::default()
        .brushed(BrushSample {
            center: p(3., 3.),
            radius: 1.,
            amount: 0.25,
            kind: BrushKind::Lower,
        })
        .unwrap();
    let mesh = garden_generation::context_mesh::foundation_mesh(&b, 0., &terrain).unwrap();
    assert!(!mesh.batches.is_empty());
    for batch in &mesh.batches {
        batch.data.validate().unwrap();
    }
    assert!(
        mesh.batches
            .iter()
            .flat_map(|b| b.data.positions.iter().zip(&b.data.normals))
            .any(|(p, n)| p[1] == 0. && n[1] > 0.99)
    );
}
