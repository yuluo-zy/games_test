//! 支柱放置（place_pillars.rs）。
//!
//! `place_pillars`（0x140753670，52KB）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, EventReader<RecalculateWallBricks>,
//!  EventReader<CreateRoofCmd>, Res<PublicWalls>, Res<AssetSsboLibrary>, …)`。
//!
//! 机器码（re-analyst-C §5）：
//! - 双缓冲队列 walk → 每段（0x88B 元素）支柱候选数组（stride 8 {i32,f32}）
//!   → `if kind==2` 分支调 wall_path_segment 访问器族 → FUN_141206c50/
//!   FUN_141207a10（+0.1875 常量 0x142a7c1d0）两变体（半砖偏移）。
//! - SSBO 注册表访问（FUN_141209c40；键 0x142c1a878 = "…BrickTransformSsbo"
//!   58B）→ 虚调取缓冲 → 稀疏实例追加 + Arc/condvar 握手（rhapsody SSBO
//!   后台上传）。
//! - 每支柱调 `assemble_stone_pillar`（0x14120d670）。
//! - `PillarProposals::add_proposal`（0x1407534d0）：proposals 装配 + 96B 记录。
//! - `assemble_stone_pillar` 常量（O）：0.2/0.2 十字偏移、1.2 砖长、0.01 ε、
//!   0.5×3、−0.2、2.1 高度限、±0.25、0.56 行距、1.26。
//! - panic 串：`assertion failed: i: < len`（0x142c192f8/318）。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::commands::{CreateRoofCmd, RecalculateWallBricks},
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use bevy_ecs::prelude::*;

/// B `PillarProposals`（0x1407534d0 装配；提议记录 stride 8 {i32, f32}）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct PillarProposals {
    pub proposals: Vec<[u8; 8]>,
}

impl PillarProposals {
    /// B `add_proposal`（0x1407534d0）。**未实现（近似体）**：真实逻辑为
    /// `PillarProposals{Vec<PillarPlacementProposal 0x1cB>, Vec<f32> scores,
    /// Vec<usize> keys}` 三向量；新 key 三向量 push；同 key score NaN-aware
    /// min，新 score 更小则替换末提案 = 每墙最优。现有 8B 记录实现不匹配。
    pub fn add_proposal(&mut self, _index: i32, _height: f32) {
        todo!("未实现: PillarProposals::add_proposal 0x1407534d0")
    }
}

/// B `assemble_stone_pillar`（0x14120d670）。**未实现（近似体）**：真实逻辑含
/// bottom+0.2≤1.26 单段否则双段且 1.2<|Δ| 标记、span=(top−bottom)−0.25、
/// count=ceil(span/0.56)、random_splits、−0.2 偏移+fastrand jitter
/// （0.42/0.7 或顶段固定 0.7/0.84）、bottom+0.2>2.1 盖砖、closure$4 发 96B。
/// 现有装配不匹配。
pub fn assemble_stone_pillar(
    _x: f32,
    _z: f32,
    _height: f32,
    _seed: u32,
    _out: &mut Vec<[u8; 96]>,
) {
    todo!("未实现: assemble_stone_pillar 0x14120d670")
}

/// B 0x140753670。
pub fn place_pillars(
    mut changed: EventReader<OnWallChanged>,
    mut bricks: EventReader<RecalculateWallBricks>,
    mut create_roof: EventReader<CreateRoofCmd>,
    walls: Res<PublicWalls>,
    ssbo: Res<AssetSsboLibrary>,
    mut proposals: ResMut<PillarProposals>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0 .0)
        .chain(bricks.read().map(|e| e.0 .0))
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let curve = wall.simplified_curve_2d();
        // 支柱候选：每 0.1875 半砖偏移采样（FUN_141206c50/FUN_141207a10）。
        let total = curve.length;
        let count = ((total / 1.2).ceil() as usize).clamp(0, 64);
        for i in 0..count {
            let t = (i as f32 + 0.5) / count.max(1) as f32;
            let idx = ((t * (curve.points.len() - 1).max(1) as f32) as usize)
                .min(curve.points.len().saturating_sub(2));
            let p = curve.points[idx];
            proposals.add_proposal(i as i32, p[1]);
        }
        let _ = ssbo;
    }
}
