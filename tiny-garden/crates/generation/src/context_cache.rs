//! 专用解析缓存：每个宿主只保留最近一次精确输入，删除对象时回收。
//! 地基只依赖根轮廓、姿态和局部地形；窗户修改不会再次裁剪整栋地形。
use crate::context::*;
use garden_domain::{
    Building, BuildingId, Placement,
    context::*,
    strokes::{StrokeId, StrokeKind},
    terrain::{TerrainDocument, TerrainTile, TileId},
};
use garden_geometry::Rect;
use std::{collections::BTreeMap, sync::Arc};
type Ground = Vec<(TileId, Arc<TerrainTile>)>;
type NetworkResult = (PathNetwork, Vec<ConnectionHint>, Vec<Diagnostic>);
#[derive(Clone, PartialEq)]
struct NetworkKey {
    lines: Vec<LinearStructure>,
    ground: Ground,
    hints: Vec<ConnectionHint>,
    rules: ContextRules,
}
#[derive(Clone, PartialEq)]
struct FoundationKey {
    footprint: Rect,
    placement: Placement,
    intent: FoundationIntent,
    ground: Ground,
}
#[derive(Clone, PartialEq)]
struct FacadeKey {
    surface: WallSurface,
    doors: Vec<DoorRequest>,
    openings: Vec<OpeningIntent>,
    groups: Vec<Vec<OpeningId>>,
    gaps: [f64; 2],
}
/// 最近一次解析的命中/计算次数，用于回归和 CPU 基准，不代表 GPU 帧率。
#[derive(Debug, Clone, Copy, Default)]
pub struct CacheStats {
    pub network_built: u64,
    pub foundations_built: u64,
    pub facades_built: u64,
    pub reused: u64,
}
#[derive(Clone, Default)]
pub struct ContextCache {
    network: Option<Arc<(NetworkKey, NetworkResult)>>,
    foundations: BTreeMap<BuildingId, Arc<(FoundationKey, f64)>>,
    facades: BTreeMap<WallAnchor, Arc<(FacadeKey, FacadeStructure, Vec<Diagnostic>)>>,
    pub stats: CacheStats,
}
/// 查询块包括半格边界；相邻块新出现时也会改变键，捕获原先为空的地形依赖。
fn ground(terrain: &TerrainDocument, bounds: [f64; 4]) -> Ground {
    TerrainDocument::tiles_in([
        bounds[0] - 0.25,
        bounds[1] - 0.25,
        bounds[2] + 0.25,
        bounds[3] + 0.25,
    ])
    .into_iter()
    .filter_map(|id| terrain.tiles.get(&id).map(|tile| (id, tile.clone())))
    .collect()
}
impl ContextCache {
    pub(super) fn begin(&mut self) {
        self.stats = CacheStats::default();
    }
    pub(super) fn prune(&mut self, input: &ContextInput) {
        self.foundations
            .retain(|id, _| input.buildings.iter().any(|b| b.id() == *id));
        self.facades.retain(|host, _| {
            input
                .buildings
                .iter()
                .any(|b| b.id() == host.building && b.blocks().iter().any(|s| s.id == host.block))
        });
    }
    pub(super) fn network(
        &mut self,
        lines: &BTreeMap<StrokeId, LinearStructure>,
        input: &ContextInput,
    ) -> NetworkResult {
        let paths = lines
            .values()
            .filter(|l| l.kind == StrokeKind::Path)
            .cloned()
            .collect::<Vec<_>>();
        let mut tiles = BTreeMap::new();
        for line in &paths {
            tiles.extend(ground(&input.scene.terrain, line.stroke.bounds()));
        }
        let key = NetworkKey {
            lines: paths,
            ground: tiles.into_iter().collect(),
            hints: input
                .scene
                .decisions
                .connections
                .iter()
                .filter(|h| h.kind == StrokeKind::Path)
                .copied()
                .collect(),
            rules: input.scene.rules,
        };
        if let Some(entry) = &self.network
            && entry.0 == key
        {
            self.stats.reused += 1;
            return entry.1.clone();
        }
        let result = path_network(
            lines,
            &input.scene.terrain,
            &input.scene.decisions,
            input.scene.rules,
        );
        self.network = Some(Arc::new((key, result.clone())));
        self.stats.network_built += 1;
        result
    }
    /// 单栋候选池只保留当前建筑，取消和新建不会累积过往候选编号。
    pub fn preview_foundation(&mut self, building: &Building, scene: &SceneContext) -> f64 {
        self.foundations.retain(|id, _| *id == building.id());
        self.foundation(building, scene)
    }
    /// 也供建筑候选轮廓使用，正式地基和预览共享裁剪算法及缓存合同。
    pub fn foundation(&mut self, building: &Building, scene: &SceneContext) -> f64 {
        let r = building
            .blocks()
            .iter()
            .find(|b| b.parent.is_none())
            .unwrap()
            .footprint;
        let p = building.placement();
        let points = [
            [r.x, r.z],
            [r.right(), r.z],
            [r.right(), r.back()],
            [r.x, r.back()],
        ];
        let mut bounds = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for q in points {
            let q = world_point(
                garden_domain::strokes::Point { x: q[0], z: q[1] },
                p.x,
                p.z,
                p.yaw,
            );
            bounds = [
                bounds[0].min(q.x),
                bounds[1].min(q.z),
                bounds[2].max(q.x),
                bounds[3].max(q.z),
            ];
        }
        let intent = scene
            .foundations
            .get(&building.id())
            .copied()
            .unwrap_or(FoundationIntent {
                reference_y: p.elevation,
                clearance: 0.,
            });
        let key = FoundationKey {
            footprint: r,
            placement: p,
            intent,
            ground: ground(&scene.terrain, bounds),
        };
        if let Some(entry) = self.foundations.get(&building.id())
            && entry.0 == key
        {
            self.stats.reused += 1;
            return entry.1;
        }
        let base = if key.ground.is_empty() {
            // 局部没有高度块时地面就是零，无需因远处笔刷遍历平坦三角形。
            if scene.terrain.tiles.is_empty() && !scene.foundations.contains_key(&building.id()) {
                p.elevation
            } else {
                intent.reference_y.max(0.) + intent.clearance
            }
        } else {
            foundation_with_intent(building, &scene.terrain, intent)
        };
        self.foundations
            .insert(building.id(), Arc::new((key, base)));
        self.stats.foundations_built += 1;
        base
    }
    pub(super) fn facade(
        &mut self,
        surface: WallSurface,
        input: &ContextInput,
        network: &PathNetwork,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> FacadeStructure {
        // 穿越候选单独计算，门请求相同即可复用连窗解析，不把整张路网放进立面键。
        let requests = doors(&surface, network, &input.scene.terrain, diagnostics);
        let openings = input
            .scene
            .openings
            .values()
            .filter(|w| w.host == surface.anchor)
            .cloned()
            .collect::<Vec<_>>();
        let groups = input
            .scene
            .decisions
            .window_groups
            .iter()
            .filter(|g| g.iter().any(|id| openings.iter().any(|w| w.id == *id)))
            .cloned()
            .collect();
        let key = FacadeKey {
            surface: surface.clone(),
            doors: requests.clone(),
            openings,
            groups,
            gaps: [
                input.scene.rules.window_enter,
                input.scene.rules.window_exit,
            ],
        };
        if let Some(entry) = self.facades.get(&surface.anchor)
            && entry.0 == key
        {
            self.stats.reused += 1;
            diagnostics.extend(entry.2.clone());
            return entry.1.clone();
        }
        let mut local = Vec::new();
        let value = facade_with_doors(surface, input, requests, &mut local);
        diagnostics.extend(local.clone());
        self.facades
            .insert(value.surface.anchor, Arc::new((key, value.clone(), local)));
        self.stats.facades_built += 1;
        value
    }
}
