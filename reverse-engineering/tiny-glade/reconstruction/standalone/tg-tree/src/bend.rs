//! `tree_move_bend` (VA 0x1422689c0, tree_move_bend.rs:8-42).
//! Evidence: tree-tool-rest-analysis.md §4 and shared deps table.

use crate::resources::GameTime;
use crate::types::*;
use bevy_ecs::prelude::*;
use glam::Vec2;

pub const LERP_RATE: f32 = 5.0; // 0x142925988
pub const MIN_DIST: f32 = 0.1; // 0x142925984
pub const STEP_RATE: f32 = 3.22; // 0x142925904
pub const MAX_DIST: f32 = 5.0;

/// `utils::time_continuous_lerp` (0x140c904e0): target + (current-target)*exp(-delta*rate).
pub fn time_continuous_lerp(current: f32, target: f32, delta: f32, rate: f32) -> f32 {
    target + (current - target) * (-delta * rate).exp()
}

/// Pure per-tree step. Returns (new bend, Some(step) if the dist>=0.1 branch ran).
/// When `dist < 0.1` the bend stays at its lerped value and step is None
/// (whether bend_step is touched then is UNKNOWN(evidence: tree-tool-rest-analysis.md)).
pub fn bend_step(bend: Vec2, target: Vec2, delta: f32) -> (Vec2, Option<Vec2>) {
    let lerped = Vec2::new(
        time_continuous_lerp(bend.x, target.x, delta, LERP_RATE),
        time_continuous_lerp(bend.y, target.y, delta, LERP_RATE),
    );
    let diff = lerped - target;
    let dist = diff.length();
    if dist >= MIN_DIST {
        let step = dist.min(MAX_DIST) * (STEP_RATE / dist) * diff;
        (target + step, Some(step))
    } else {
        (lerped, None)
    }
}

/// System: Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>.
pub fn tree_move_bend(
    mut trees: Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>,
    game_time: Res<GameTime>,
) {
    for (mut bend, mut inst) in &mut trees {
        let target = Vec2::new(inst.position.x, inst.position.z);
        let (new_bend, step) = bend_step(bend.0, target, game_time.delta);
        bend.0 = new_bend;
        // change detection: only write when differing from stored +0x24/+0x28
        if let Some(step) = step {
            if step != inst.bend_step {
                inst.bend_step = step;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::prelude::*;
    use glam::Vec3;

    #[test]
    fn lerp_limits() {
        assert_eq!(time_continuous_lerp(10.0, 2.0, 0.0, 5.0), 10.0);
        assert!((time_continuous_lerp(10.0, 2.0, 100.0, 5.0) - 2.0).abs() < 1e-5);
        let v = time_continuous_lerp(10.0, 2.0, 0.2, 5.0);
        assert!((v - (2.0 + 8.0 * (-1.0f32).exp())).abs() < 1e-5);
    }

    #[test]
    fn far_branch_scales_step_to_3_22() {
        // dist after lerp = 1.0 (delta 0): step length = min(1,5)*3.22
        let (b, s) = bend_step(Vec2::new(1.0, 0.0), Vec2::ZERO, 0.0);
        let s = s.unwrap();
        assert!((s.length() - 3.22).abs() < 1e-5);
        assert!((b - s).length() < 1e-6);
    }

    #[test]
    fn far_branch_caps_dist_at_5() {
        let (_, s) = bend_step(Vec2::new(0.0, 100.0), Vec2::ZERO, 0.0);
        assert!((s.unwrap().length() - 5.0 * 3.22).abs() < 1e-3);
    }

    #[test]
    fn near_branch_keeps_lerped_value() {
        let (b, s) = bend_step(Vec2::new(0.05, 0.0), Vec2::ZERO, 0.0);
        assert_eq!(s, None);
        assert_eq!(b, Vec2::new(0.05, 0.0));
        let (_, s) = bend_step(Vec2::new(0.1, 0.0), Vec2::ZERO, 0.0);
        assert!(s.is_some()); // >= 0.1 inclusive
    }

    #[test]
    fn system_writes_bend_and_step() {
        let mut app = App::new();
        app.insert_resource(GameTime { delta: 0.0 }).add_systems(Update, tree_move_bend);
        let e = app.world_mut().spawn((
            PrefabInstanceData::at(Vec3::new(3.0, 0.0, 4.0)),
            TreeMoveBend(Vec2::ZERO),
        )).id();
        // bend without PrefabInstanceData is not matched by the query
        let orphan = app.world_mut().spawn(TreeMoveBend(Vec2::ONE)).id();
        app.update();
        let step = app.world().get::<PrefabInstanceData>(e).unwrap().bend_step;
        assert!((step.length() - 5.0 * 3.22).abs() < 1e-4);
        assert_eq!(app.world().get::<TreeMoveBend>(orphan).unwrap().0, Vec2::ONE);
    }
}
