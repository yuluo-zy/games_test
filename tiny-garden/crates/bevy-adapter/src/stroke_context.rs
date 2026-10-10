//! 上下文派生组的精确输入缓存；网格缓存不使用整个场景修订号作为失效键。
use super::*;
use garden_domain::{
    context::*,
    strokes::StrokeKind,
    terrain::{TerrainDocument, TerrainTile, TileId},
};
use garden_generation::{
    context::*,
    context_mesh::{self, DerivedGroup},
};
#[derive(Clone, PartialEq)]
pub(super) enum ContextMeshKey {
    Boundary(
        LinearStructure,
        Vec<garden_generation::strokes::Opening>,
        Vec<BoundaryJunction>,
        Vec<(TileId, Arc<TerrainTile>)>,
    ),
    Road(Vec<RoadEdgeKey>, Vec<(TileId, Arc<TerrainTile>)>),
    Terrain(Vec<(TileId, Arc<TerrainTile>)>),
    Foundation(
        [f64; 4],
        garden_domain::Placement,
        f64,
        Vec<(TileId, Arc<TerrainTile>)>,
    ),
    Assembly(WallSurface, OpeningAssembly),
}
#[derive(Clone, PartialEq)]
pub(super) struct RoadEdgeKey {
    source: SegmentSource,
    interval: [f64; 2],
    a: Point,
    b: Point,
    heights: [f64; 2],
    width: f64,
}
use garden_domain::strokes::Point;
fn ground(terrain: &TerrainDocument, bounds: [f64; 4]) -> Vec<(TileId, Arc<TerrainTile>)> {
    TerrainDocument::tiles_in([
        bounds[0] - 0.25,
        bounds[1] - 0.25,
        bounds[2] + 0.25,
        bounds[3] + 0.25,
    ])
    .into_iter()
    .filter_map(|id| terrain.tiles.get(&id).map(|t| (id, t.clone())))
    .collect()
}
pub(super) fn build_context(
    revision: u64,
    input: &ContextInput,
    semantic: &ResolvedContext,
    baseline: Cache,
    cancel: &Cancellation,
) -> Result<Output, MeshError> {
    let mut registry = baseline.groups.clone();
    registry.begin();
    let mut cache = Cache::default();
    let mut generated = 0;
    let mut reused = 0;
    let terrain = &input.scene.terrain;
    let mut install =
        |id: StrokeId,
         key: ContextMeshKey,
         compile: &mut dyn FnMut() -> Result<garden_generation::mesh::BuildingMesh, MeshError>|
         -> Result<(), MeshError> {
            cancel.check()?;
            let mesh = if baseline.context_keys.get(&id) == Some(&key) {
                reused += 1;
                baseline.meshes[&id].clone()
            } else {
                generated += 1;
                let mut prepared = PreparedBuilding::whole(compile()?);
                super::super::prepare::prepare(&mut prepared, cancel)?;
                Arc::new(prepared)
            };
            if !mesh.batches.is_empty() {
                cache.meshes.insert(id, mesh);
                cache.context_keys.insert(id, key);
            }
            Ok(())
        };
    for line in semantic
        .linear
        .values()
        .filter(|l| l.kind != StrokeKind::Path)
    {
        let arches = semantic
            .wall_openings
            .get(&line.stroke.id)
            .cloned()
            .unwrap_or_default();
        let junctions = semantic
            .boundaries
            .iter()
            .filter(|j| j.members.iter().any(|m| m.0 == line.stroke.id))
            .cloned()
            .collect::<Vec<_>>();
        let key = ContextMeshKey::Boundary(
            line.clone(),
            arches.clone(),
            junctions.clone(),
            ground(terrain, line.stroke.bounds()),
        );
        install(line.stroke.id, key, &mut || {
            context_mesh::boundary(line, &arches, &junctions, terrain, cancel)
        })?;
    }
    for id in context_mesh::road_tiles(&semantic.network) {
        let bounds = [
            id.0 as f64 * 4.,
            id.1 as f64 * 4.,
            (id.0 + 1) as f64 * 4.,
            (id.1 + 1) as f64 * 4.,
        ];
        let edges = semantic
            .network
            .edges
            .iter()
            .filter_map(|e| {
                let a = &semantic.network.nodes[e.nodes[0]];
                let b = &semantic.network.nodes[e.nodes[1]];
                bounds_overlap(
                    bounds,
                    [
                        a.point.x.min(b.point.x) - e.width / 2.,
                        a.point.z.min(b.point.z) - e.width / 2.,
                        a.point.x.max(b.point.x) + e.width / 2.,
                        a.point.z.max(b.point.z) + e.width / 2.,
                    ],
                )
                .then_some(RoadEdgeKey {
                    source: e.source,
                    interval: e.interval,
                    a: a.point,
                    b: b.point,
                    heights: [a.height, b.height],
                    width: e.width,
                })
            })
            .collect();
        install(
            registry.id(DerivedGroup::Road(id)),
            ContextMeshKey::Road(edges, ground(terrain, bounds)),
            &mut || context_mesh::road_tile(id, &semantic.network, terrain, cancel),
        )?;
    }
    // 平坦地面也使用同一分块表示，首次笔刷和撤销不需要替换另一套地面模型。
    {
        for id in TerrainDocument::tiles_in([-25., -15.5, 25., 15.5]) {
            let bounds = [
                id.0 as f64 * 4.,
                id.1 as f64 * 4.,
                (id.0 + 1) as f64 * 4.,
                (id.1 + 1) as f64 * 4.,
            ];
            install(
                registry.id(DerivedGroup::Terrain(id)),
                ContextMeshKey::Terrain(ground(terrain, bounds)),
                &mut || context_mesh::terrain_tile(id, terrain, cancel),
            )?;
        }
    }
    if !terrain.tiles.is_empty() || !input.scene.foundations.is_empty() {
        for building in &input.buildings {
            let base = semantic.foundations[&building.id()];
            let mut bounds = [
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ];
            for facade in semantic
                .facades
                .values()
                .filter(|f| f.surface.anchor.building == building.id())
            {
                for p in [facade.surface.start, facade.surface.end] {
                    bounds = [
                        bounds[0].min(p.x),
                        bounds[1].min(p.z),
                        bounds[2].max(p.x),
                        bounds[3].max(p.z),
                    ];
                }
            }
            install(
                registry.id(DerivedGroup::Foundation(building.id())),
                ContextMeshKey::Foundation(
                    {
                        let r = building
                            .blocks()
                            .iter()
                            .find(|b| b.parent.is_none())
                            .unwrap()
                            .footprint;
                        [r.x, r.z, r.width, r.depth]
                    },
                    building.placement(),
                    base,
                    ground(terrain, bounds),
                ),
                &mut || context_mesh::foundation_mesh(building, base, terrain),
            )?;
        }
    }
    for facade in semantic.facades.values() {
        for assembly in &facade.assemblies {
            if assembly.members.len() < 2 {
                continue;
            }
            install(
                registry.id(DerivedGroup::Assembly(facade.surface.anchor, {
                    let mut members = assembly.members.clone();
                    members.sort();
                    members
                })),
                ContextMeshKey::Assembly(facade.surface.clone(), assembly.clone()),
                &mut || context_mesh::assembly(facade, assembly),
            )?;
        }
    }
    registry.finish();
    cache.groups = registry;
    Ok(Output {
        revision,
        cache,
        generated,
        reused,
    })
}
