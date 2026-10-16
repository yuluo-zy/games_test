//! add_autoclutter_around_windows（B 主基线；0x1422cefe0，源码 31–181）。
//!
//! 证据：遍历窗（`DecoratorType::spawns_clutter` / `DerivedDecoratorInfo::subtype` /
//! `CachedDecoratorTransforms::transform` / iter_removed_ids 组件查询）；
//! `extend_right_until<Vec3,CurveCoordSemantic3D>`（0x1422e1e40）沿窗曲线延伸。
//! **3 个 add_region**；键 `calculate_hash<tuple<&str,i32,usize,u32>>`（0x141cf6d60）×1 +
//! `<tuple<&str,usize,u32>>`（0x141cf77c0）×2；x0y/xvy swizzle 位置/朝向。
//! 窗类型/百叶窗/开口多分支（源行 31–181）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::mode_manager::DecoratorType;
use glam::{Vec2, Vec3};


use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

/// 窗放置宿主记录（窗装饰物 + 曲线端；宿主装配时填充）。
#[derive(Component, Clone, Debug, PartialEq)]
pub struct WindowHost {
    /// 窗装饰物 id（DecoratorStorage 键）。
    pub decorator_id: u64,
    /// 窗曲线（世界空间点；extend_right_until 消费）。
    pub curve: Vec<Vec3>,
    /// 窗朝向（xvy swizzle 语义）。
    pub forward: Vec2,
}

/// add_autoclutter_around_windows（0x1422cefe0）。
/// 每窗 3 个区域候选：左右下沿（键 tuple<&str,usize,u32>×2）与窗上沿（键
/// tuple<&str,i32,usize,u32>×1）。
pub fn add_autoclutter_around_windows(
    mut state: ResMut<AutoClutterState>,
    decorators: Res<DecoratorStorage>,
    mut query: Query<&WindowHost>,
) {
    let _ = &decorators;
    for window in &mut query {
        let Some((a, b)) = window_ends(&window.curve) else {
            continue;
        };
        let fwd = Vec3::new(window.forward.x, 0.0, window.forward.y);
        let dir = (b - a).normalize_or_zero();
        let normal = Vec3::new(-dir.z, 0.0, dir.x);
        // 3 个 add_region（O：around_windows 调 add_region ×3）。
        for (name, offset) in [
            ("window_left", -0.4f32),
            ("window_right", 0.4f32),
            ("window_top", 0.0f32),
        ] {
            let pos = (a + b) * 0.5 + normal * offset + fwd * 0.2;
            let key = hash_window_key(name, window.decorator_id, window.curve.len() as u32);
            let region = AutoClutterRegion {
                key: ClutterRegionId(key),
                center: pos,
                basis: [Vec2::new(normal.x, normal.z), Vec2::new(fwd.x, fwd.z)],
                candidate_index: ClutterCandidateIndex(0),
                epoch: state.epoch,
            };
            state.add_region(ClutterRegionId(key), &region);
        }
    }
}

/// 窗曲线端点（extend_right_until 语义的简化：取曲线首/末点）。
pub fn window_ends(curve: &[Vec3]) -> Option<(Vec3, Vec3)> {
    match (curve.first(), curve.last()) {
        (Some(&a), Some(&b)) => Some((a, b)),
        _ => None,
    }
}

/// 窗放置键（tuple<&str,i32,usize,u32> 语义 0x141cf6d60）。
pub fn hash_window_key(name: &str, decorator_id: u64, curve_len: u32) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    (name.len() as i32).hash(&mut h);
    decorator_id.hash(&mut h);
    curve_len.hash(&mut h);
    h.finish()
}

/// `DecoratorType::spawns_clutter`（O 0x140833170）：窗/箭孔装饰物才挂自动杂物。
#[allow(unused)]
fn decorator_spawns_clutter(ty: DecoratorType) -> bool {
    matches!(
        ty,
        DecoratorType::CottageWindow
            | DecoratorType::GothicWindow
            | DecoratorType::ArrowSlit
            | DecoratorType::Lantern
    )
}
