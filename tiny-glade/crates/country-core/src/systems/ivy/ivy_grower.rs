//! B `ivy_grower` 0x14094a910: the segment-growth producer whose completed
//! events feed the leaf spawner consumer (`IvySegmentDoneGrowingMsg`).
//!
//! Machine-code verified growth progression (0x14094b1bb..b267):
//! - `progress += delta / 0.075` (divisor 0x142a9abd4 = 0.075)
//! - clamp progress to [0, 1] (max 0x142925904 = 1.0, maxss/minss order)
//! - interpolate the tail point from pending `from` toward `to`
//!   (0x14094b1e8, `from*(1-t) + to*t` written at 0x14094b249)
//! - when progress reaches 0.99 (0x142a948d8) clear pending, reset progress,
//!   look up the last two point IDs (0x140869540 = `Ivy::last_segment_point_ids`)
//!   and write the completion event (0x14094b29b).
//!
//! New-segment placement is a documented candidate chain: the grower proposes a
//! direction through `IvyDirectionProposer::get_direction` (call 0x14094b4cc),
//! converts a wall coordinate to world space (`WallCoord::into_world_space`,
//! call 0x14094b7b9), appends the point (`Ivy::add_point`, call 0x14094bf76)
//! and stores the next pending segment (0x14094bf85). The exact wall-spatial
//! hash traversal and the acceptance height/noise thresholds are not
//! reconstructed; the nearest-wall lookup below is the recovered-helper
//! approximation and is reported separately from the verified completion path.

use crate::resources::{
    ivy::{Ivy, IvyDirectionProposer, IvyGrowthSegment, IvyPointId, IvyStorage},
    time::GameTime,
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use super::{ivy_leaf_spawner::IvySegmentDoneGrowingMsg, WallSegment};

/// Growth divisor: one segment takes 0.075 s at unit delta (0x142a9abd4).
pub const SEGMENT_DURATION: f32 = 0.075;
/// Segment completes at progress 0.99 (0x142a948d8).
pub const GROWTH_COMPLETE: f32 = 0.99;
/// Progress clamp maximum (0x142925904).
pub const PROGRESS_MAX: f32 = 1.0;
/// New growth may start once progress has settled at or below 0.001
/// (0x1429cd060). Root points spawn directly; the grower only extends ivies
/// that already have points.
pub const SETTLED_THRESHOLD: f32 = 0.001;

/// Advances one ivy's pending segment by `delta` seconds and emits the
/// segment-completion event when it finishes. Point IDs come from the deque
/// tail order (`Ivy::last_segment_point_ids`); a missing pending segment is a
/// no-op exactly like the native empty-option path.
pub fn advance_growth(
    ivy: &mut Ivy,
    delta: f32,
    completed: &mut EventWriter<IvySegmentDoneGrowingMsg>,
) {
    let Some(segment) = ivy.pending_growth else {
        return;
    };
    let progress = (ivy.growth_progress + delta / SEGMENT_DURATION).clamp(0.0, PROGRESS_MAX);
    ivy.growth_progress = progress;
    let position = segment.from * (1.0 - progress) + segment.to * progress;
    ivy.set_last_point_position(position);
    if progress >= GROWTH_COMPLETE {
        ivy.pending_growth = None;
        ivy.growth_progress = 0.0;
        let (point_a, point_b) = ivy.last_segment_point_ids();
        completed.write(IvySegmentDoneGrowingMsg {
            point_a,
            point_b,
            ivy: ivy.id,
        });
    }
}

/// Appends a root point and arms the next pending segment. This mirrors B
/// `Ivy::add_point` (0x140869460 via 0x14094bf76) followed by the pending
/// store at 0x14094bf85 (`from` = the new point, `to` = the next target).
/// `next` is the world-space position the tail will animate toward.
pub fn start_growth_segment(
    ivy: &mut Ivy,
    position: Vec3,
    direction: Vec3,
    next: Vec3,
) -> IvyPointId {
    let id = ivy.add_point(position, direction);
    ivy.pending_growth = Some(IvyGrowthSegment {
        from: position,
        to: next,
    });
    ivy.growth_progress = 0.0;
    id
}

/// Double-sided wall normal (B 0x14094caa0). Given a curve and a world point,
/// returns the unit perpendicular to the nearest segment, oriented to face the
/// point (`normal` when the point is on the +normal side, `-normal` otherwise).
/// The original resolves the segment through the curve's `coord_at_u`
/// (0x141467ce0) and flips the normal with `xorps` when the point lies on the
/// opposite side (0x14094cbbd..cbf9).
pub fn get_wall_double_sided_normal(curve: &system_roof::surface::Curve2, point: Vec2) -> Vec2 {
    let curve = curve.clone();
    let index = curve.coord_at_u((point.x / curve.length.max(1e-20)).clamp(0.0, 1.0)).0;
    let index = index.min(curve.points.len().saturating_sub(2));
    let a = Vec2::from(curve.points[index]);
    let b = Vec2::from(curve.points[index + 1]);
    let tangent = (b - a).normalize_or_zero();
    let normal = Vec2::new(-tangent.y, tangent.x);
    if normal.dot(point - a) >= 0.0 {
        normal
    } else {
        -normal
    }
}

/// Segment/ray intersection used to stop growth that would cross a wall
/// (B 0x14094cc60). The original calls the engine's ray-intersection helper
/// (0x141240b30) and re-derives the contact along the ivy growth ray. Returns
/// the parametric `t` along `from -> to` of the first contact, if any.
pub fn intersect_ivy_growth_w_wall_segment(from: Vec2, to: Vec2, a: Vec2, b: Vec2) -> Option<f32> {
    let direction = to - from;
    let denominator = direction.perp_dot(b - a);
    if denominator.abs() < 1e-20 {
        return None;
    }
    let t = (a - from).perp_dot(b - a) / denominator;
    let u = (a - from).perp_dot(direction) / denominator;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(t)
    } else {
        None
    }
}

/// Builds `WallSegment` rows from a published wall's simplified curve. Heights
/// come from the published segment heights when present, otherwise zero; this
/// is the producer used by the nearest-wall candidate chain.
pub fn wall_segments_from_state(
    state: &crate::resources::walls::public_wall_state::PublicWallState,
) -> Vec<WallSegment> {
    let curve = state.simplified_curve_2d();
    let heights = &state.segment_heights;
    curve
        .points
        .windows(2)
        .enumerate()
        .map(|(i, pair)| WallSegment {
            p0: Vec2::from(pair[0]),
            p1: Vec2::from(pair[1]),
            extra_a: 0.0,
            extra_b: 0.0,
            h0: heights.get(i).copied().unwrap_or(0.0),
            h1: heights.get(i + 1).copied().unwrap_or(0.0),
        })
        .collect()
}

/// Nearest wall within the search radius, from the candidate chain. Returns the
/// wall id and the double-sided normal at `position`.
pub fn nearest_wall_normal(
    walls: &PublicWalls,
    position: Vec3,
) -> Option<(crate::resources::walls::WallId, Vec2)> {
    let point = Vec2::new(position.x, position.z);
    let mut best: Option<(crate::resources::walls::WallId, f32, Vec2)> = None;
    for (id, state) in walls.all() {
        let segments = wall_segments_from_state(state);
        let Ok(Some(hit)) = super::closest_segment_index(point, &segments, f32::MAX) else {
            continue;
        };
        if hit.distance <= crate::systems::ivy::INITIAL_BEST_DISTANCE {
            let normal = get_wall_double_sided_normal(state.simplified_curve_2d(), point);
            match best {
                Some((_, d, _)) if d <= hit.distance => {}
                _ => best = Some((id, hit.distance, normal)),
            }
        }
    }
    best.map(|(id, _, normal)| (id, normal))
}

/// Runs the growth producer for every stored ivy. Existing pending segments
/// always advance (verified path). When an ivy has room to grow and no pending
/// segment, the candidate chain proposes a direction off the nearest wall and
/// extends the root point. GPU/upload consumption is out of scope here.
pub fn ivy_grower(
    mut storage: ResMut<IvyStorage>,
    time: Res<GameTime>,
    proposer: Res<IvyDirectionProposer>,
    walls: Res<PublicWalls>,
    mut completed: EventWriter<IvySegmentDoneGrowingMsg>,
) {
    for (_, ivy) in storage.iter_mut() {
        if ivy.pending_growth.is_some() {
            advance_growth(ivy, time.delta, &mut completed);
            continue;
        }
        if ivy.point_count() == 0 || ivy.point_count() >= ivy.maximum_point_count {
            continue;
        }
        if ivy.growth_progress > SETTLED_THRESHOLD {
            continue;
        }
        // Candidate chain: nearest-wall normal + direction proposal. The exact
        // original acceptance gates are not reconstructed (see module docs).
        let tail = ivy.last_point().position;
        let Some((_, normal)) = nearest_wall_normal(&walls, tail) else {
            continue;
        };
        let direction = proposer.get_direction(tail, ivy.source_60);
        let world = Vec3::new(direction.x, 0.0, direction.y);
        let side = if world.dot(Vec3::new(normal.x, 0.0, normal.y)) >= 0.0 {
            1.0
        } else {
            -1.0
        };
        let next = tail + world * side * SEGMENT_DURATION;
        start_growth_segment(ivy, tail, Vec3::new(normal.x, 0.0, normal.y), next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ivy::IvyStorage;
    use bevy_ecs::world::World;
    use glam::Vec3;

    #[test]
    fn growth_completes_and_emits_one_event_per_segment() {
        let mut world = World::new();
        world.init_resource::<Events<IvySegmentDoneGrowingMsg>>();
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        {
            let ivy = storage.get_mut(id).unwrap();
            let _ = ivy.add_point(Vec3::X, Vec3::X);
            ivy.pending_growth = Some(IvyGrowthSegment {
                from: Vec3::X,
                to: Vec3::X * 2.0,
            });
        }
        let mut events = world.resource_mut::<Events<IvySegmentDoneGrowingMsg>>();
        {
            let mut writer = EventWriter::new(&mut *events);
            let mut ivy = storage.get_mut(id).unwrap();
            advance_growth(&mut ivy, 0.075, &mut writer);
            assert_eq!(ivy.pending_growth, None);
            assert_eq!(ivy.growth_progress, 0.0);
            assert!(ivy.last_point().position.abs_diff_eq(Vec3::X * 2.0, 1e-6));
            // A second tick must not emit again.
            advance_growth(&mut ivy, 0.075, &mut writer);
        }
        assert_eq!(events.len(), 1);
        let (point_a, point_b) = events.drain().next().map(|e| (e.point_a, e.point_b)).unwrap();
        assert_eq!((point_a, point_b), (0, 2));
    }

    #[test]
    fn growth_interpolates_tail_at_partial_progress() {
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        let mut ivy = storage.get_mut(id).unwrap();
        let _ = ivy.add_point(Vec3::X, Vec3::X);
        ivy.pending_growth = Some(IvyGrowthSegment {
            from: Vec3::X,
            to: Vec3::X * 2.0,
        });
        let mut world = World::new();
        world.init_resource::<Events<IvySegmentDoneGrowingMsg>>();
        let mut events = world.resource_mut::<Events<IvySegmentDoneGrowingMsg>>();
        let mut writer = EventWriter::new(&mut *events);
        // 0.0375 s is half of 0.075: tail sits midway at (1.5, 0, 0).
        advance_growth(&mut ivy, 0.0375, &mut writer);
        assert!(ivy.last_point().position.abs_diff_eq(Vec3::new(1.5, 0.0, 0.0), 1e-6));
        assert!(ivy.pending_growth.is_some());
        assert!(events.is_empty());
    }

    #[test]
    fn start_growth_appends_point_and_arms_pending_segment() {
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 7);
        let mut ivy = storage.get_mut(id).unwrap();
        let point_id = start_growth_segment(&mut ivy, Vec3::X, Vec3::Y, Vec3::X + Vec3::Y);
        assert_eq!(point_id, 2);
        assert_eq!(ivy.point_count(), 2);
        assert!(ivy.last_point().position.abs_diff_eq(Vec3::X, 1e-6));
        assert_eq!(ivy.pending_growth.map(|s| (s.from, s.to)), Some((Vec3::X, Vec3::X + Vec3::Y)));
        assert_eq!(ivy.growth_progress, 0.0);
    }
}
