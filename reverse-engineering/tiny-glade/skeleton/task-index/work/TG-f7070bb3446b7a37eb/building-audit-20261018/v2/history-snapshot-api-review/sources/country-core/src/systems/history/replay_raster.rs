//! B 0x1409f4650 and its five sibling instances. Undo rebuilds the composite
//! from the shared history, stopping at Arc identity, then animates the remaining
//! prefix in scratch. No second history ledger or saved fake pixel snapshot.
use crate::{
    asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType},
    render::ssbo::MappedStorageBuffer,
    resources::{
        history::{EditType, History, ReplayBuffer, ReplayCommand},
        raster_stroke::RasterStroke,
        time::GameTime,
        world_raster::WorldRaster,
        world_raster_defs::*,
    },
    utils::dirty_tiles::DirtyTiles,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use std::sync::Arc;

/// 4 组资源参数面（B `replay_raster::_::impl$0::get_param` 0x140ae5830 输出）。
///
/// fresh 反编译（impl-history-deep 2026-10-17，`results/batch2-reva-history-process.json`）：
/// 输出元素 = 4 组 × 0x20B，每组 `{data u64 @+0x00, p58 ptr @+0x08, p5c ptr @+0x10,
/// flag u32 @+0x18, u1c u32 @+0x1c}`；末尾 `[0x80] = in_RDX + 4`（索引数组尾指针）。
/// 每组从稀疏资源表槽（元素 0x60B）取：`data = *(slot + 0x10)`（载荷数据指针）、
/// `p58 = &slot[0x58]`、`p5c = &slot[0x5c]`（资源槽 +0x58/+0x5c 子字段指针）、
/// `flag = *(in_R8 + 0x130)`（u32，四组共享）、`u1c = in_stack_00000028`（入参 u32）。
///
/// 原版从稀疏资源表按 4 索引解析（in_RDX[0..4]）；任一缺失 → panic
/// `"Resource requested by <type> does not exist"`（类型名常量：
/// `GameTimeResource` @0x142ae20a7 / `History` @0x142ae225e /
/// `AssetSsboLibrary` @0x142ae2287 / `ReplayBuffer` @0x142ae250d）。
/// src 回放系统经 Bevy `Res` 注入同四资源（缺失即装配错误）；本函数按机器
/// 控制流实现取参并组装 4 组参数面。
///
/// **载荷指针 @+0x58/+0x5c 语义**：为各资源槽 0x60B 元素内部第 0x58/0x5c 字节的
/// 子字段地址（`&slot[0x58]`/`&slot[0x5c]`），由调用方 replay_raster 按资源类型
/// 解引用；各资源原生槽内 0x58/0x5c 字段的精确含义随资源类型而异
/// （GameTime/History/Ssbo/Replay 各自原生布局待对账，不编造字段含义）。
#[derive(Clone, Copy)]
pub struct RasterReplayParams<'a, S: Send + Sync + 'static> {
    /// 组 0（GameTimeResource）。
    pub time: &'a GameTime,
    /// 组 1（History）。
    pub history: &'a History<S>,
    /// 组 2（AssetSsboLibrary）。
    pub ssbo: &'a AssetSsboLibrary,
    /// 组 3（ReplayBuffer）。
    pub replay: &'a ReplayBuffer,
}

/// B `get_param`（0x140ae5830）：4 组资源取参（判别语义见 `RasterReplayParams`）。
pub fn get_param<'a, S: Send + Sync + 'static>(
    time: &'a GameTime,
    history: &'a History<S>,
    ssbo: &'a AssetSsboLibrary,
    replay: &'a ReplayBuffer,
) -> RasterReplayParams<'a, S> {
    // 机器码 4 索引稀疏表判缺失（in_RDX[i] < 表长 && 元素非空才通过）；
    // src 资源已注入存在，缺失即系统装配错误，保留原 panic 语义面。
    RasterReplayParams {
        time,
        history,
        ssbo,
        replay,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RasterDomain {
    Path,
    Cobblestone1,
    Cobblestone2,
    Cobblestone3,
    Water,
    Garden,
}

#[derive(Clone, Debug)]
pub struct RasterReplayCmd {
    pub domain: RasterDomain,
    pub stroke: Arc<RasterStroke>,
    pub elapsed: f32,
    pub duration: f32,
    pub undo: bool,
    pub started: bool,
    pub emit_feedback: bool,
}
impl RasterReplayCmd {
    pub fn new(
        domain: RasterDomain,
        stroke: Arc<RasterStroke>,
        undo: bool,
        instant: bool,
        emit_feedback: bool,
    ) -> Self {
        let duration = if instant {
            0.0
        } else {
            duration_based_on_stroke_length(&stroke.points)
        };
        Self {
            domain,
            stroke,
            elapsed: 0.0,
            duration,
            undo,
            started: false,
            emit_feedback,
        }
    }
    pub fn make_instant(&mut self) {
        self.duration = 0.0;
    }
}

/// B 0x141700430/620. The native pair-unrolled sum has no upper duration cap.
pub fn duration_based_on_stroke_length(points: &[Vec3]) -> f32 {
    assert!(
        !points.is_empty(),
        "raster replay duration requires a nonempty stroke"
    );
    let distance = |a: Vec3, b: Vec3| {
        let d = a - b;
        ((d.z * d.z + d.y * d.y) + d.x * d.x).sqrt()
    };
    let mut length = 0.0;
    let mut i = 0;
    while i + 2 < points.len() {
        length =
            (distance(points[i + 1], points[i + 2]) + distance(points[i], points[i + 1])) + length;
        i += 2;
    }
    if i + 1 < points.len() {
        length += distance(points[i], points[i + 1]);
    }
    let duration = (length / 5.0) * 0.025;
    // Native unordered comparison leaves NaN intact.
    if duration <= 0.4 { 0.4 } else { duration }
}

/// Original Curve<Vec3,CurveCoordSemantic3D> dependency operations, not a new
/// slicing algorithm. Original invalid full curves panic; invalid tiny slices
/// are represented as an empty preview.
pub fn get_stroke_from_0_to_t(stroke: &RasterStroke, t: f32) -> Vec<Vec3> {
    if t < 0.01 {
        return Vec::new();
    }
    let curve = utils::curve::Curve3::try_new_from_points(stroke.points.to_vec(), false)
        .expect("cannot replay an invalid raster curve");
    assert!((0.0..=1.0).contains(&t), "invalid CurveU");
    let mut points = Vec::new();
    curve.slice_into(0.0, t, &mut points);
    match utils::curve::Curve3::try_new_from_points(points, false) {
        Ok(curve) => curve.points,
        Err(_) => Vec::new(),
    }
}

#[derive(Default)]
pub struct RasterReplayDirty {
    previous: DirtyTiles,
    current: DirtyTiles,
}
#[derive(Default)]
pub struct RasterReplayScratch {
    slots: [RasterReplayDirty; 6],
}

/// Caller-owned replay sample. The original owner converts it to its water,
/// garden or path event and any associated audio/hint notifications.
#[derive(Event, Clone, Copy, Debug)]
pub struct RasterReplayFeedback {
    pub domain: RasterDomain,
    pub position: Vec3,
    pub radius: f32,
    pub replay: bool,
}

fn paint(raster: &mut WorldRaster<512>, selector: u8, stroke: &RasterStroke, points: &[Vec3]) {
    raster.rasterize_stroke(
        selector,
        points,
        stroke.radius,
        stroke.draw,
        stroke.flags,
        stroke.strength,
    );
}

/// Returns true only after actual pixel/staging work and feedback have run.
#[allow(clippy::too_many_arguments)]
pub fn advance_raster_replay<S>(
    command: &mut RasterReplayCmd,
    delta: f32,
    speed: f32,
    history: &History<S>,
    raster: &mut WorldRaster<512>,
    dirty: &mut RasterReplayDirty,
    mut stage: impl FnMut(&mut WorldRaster<512>, u8),
    mut feedback: impl FnMut(RasterReplayFeedback),
) -> bool {
    if !command.started {
        assert!(
            !command.stroke.points.is_empty(),
            "assertion failed: !cmd.edit.stroke.points.is_empty()"
        );
        command.started = true;
        dirty.previous.clear();
        dirty.current.clear();
        if command.undo {
            raster.clear_buffer(0);
            'history: for entry in &history.ledger().entries {
                for edit in &entry.edits {
                    if let Some((domain, stroke)) = edit.raster() {
                        if domain != command.domain {
                            continue;
                        }
                        if Arc::ptr_eq(&stroke, &command.stroke) {
                            break 'history;
                        }
                        paint(raster, 0, &stroke, &stroke.points);
                    }
                }
            }
        }
        if command.duration < 0.01 {
            if !command.undo {
                paint(raster, 0, &command.stroke, &command.stroke.points);
                if command.emit_feedback {
                    feedback(RasterReplayFeedback {
                        domain: command.domain,
                        position: command.stroke.points[0],
                        radius: command.stroke.radius,
                        replay: true,
                    });
                }
            }
            dirty
                .current
                .rasterize_world_space_stroke(&command.stroke.points, command.stroke.radius);
            raster.extend_dirty(&dirty.previous);
            raster.extend_dirty(&dirty.current);
            stage(raster, 0);
            return true;
        }
    }
    command.elapsed += speed * delta;
    let progress = (command.elapsed / command.duration).max(0.0).min(1.0);
    let t = utils::easing_functions::cubic_in_out(if command.undo {
        1.0 - progress
    } else {
        progress
    });
    let points = get_stroke_from_0_to_t(&command.stroke, t);
    dirty.current.clear();
    if command.undo {
        let first = points.len().saturating_sub(3);
        dirty
            .current
            .rasterize_world_space_stroke(&command.stroke.points[first..], command.stroke.radius);
    } else {
        dirty
            .current
            .rasterize_world_space_stroke(&points, command.stroke.radius);
    }
    raster.extend_dirty(&dirty.current);
    raster.extend_dirty(&dirty.previous);
    std::mem::swap(&mut dirty.current, &mut dirty.previous);
    raster.copy_composite_to_scratch();
    paint(raster, 1, &command.stroke, &points);
    stage(raster, 1);
    if command.emit_feedback {
        feedback(RasterReplayFeedback {
            domain: command.domain,
            position: points.last().copied().unwrap_or(command.stroke.points[0]),
            radius: command.stroke.radius,
            replay: true,
        });
    }
    if progress >= 1.0 && !command.undo {
        raster.commit_scratch_to_composite();
    }
    progress >= 1.0
}

fn upload<T>(library: &mut AssetSsboLibrary, raster: &mut WorldRaster<512>, selector: u8)
where
    T: SsboType<Storage = MappedStorageBuffer<'static, T>> + bytemuck::Pod,
{
    let staging = library
        .get_mut::<T>()
        .expect("called `Option::unwrap()` on a `None` value");
    raster.copy_to_staging(selector, bytemuck::cast_slice_mut(staging.mapped));
}

/// Six original domain filters share the same front bucket and the same History.
/// This combined CPU host preserves command order; original global system order
/// across independent raster domains has not been claimed.
pub fn replay_rasters<S: Send + Sync + 'static>(
    mut replay: ResMut<ReplayBuffer>,
    history: Res<History<S>>,
    time: Res<GameTime>,
    mut rasters: WorldRasterResources,
    mut library: ResMut<AssetSsboLibrary>,
    mut scratch: Local<RasterReplayScratch>,
    mut feedback: EventWriter<RasterReplayFeedback>,
) {
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let mut finished = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        let ReplayCommand::Raster(command) = command else {
            continue;
        };
        let dirty = &mut scratch.slots[command.domain as usize];
        macro_rules! run {
            ($field:ident, $staging:ty) => {
                advance_raster_replay(
                    command,
                    time.delta,
                    speed,
                    &history,
                    &mut rasters.$field,
                    dirty,
                    |raster, selector| upload::<$staging>(&mut library, raster, selector),
                    |event| {
                        feedback.write(event);
                    },
                )
            };
        }
        let done = match command.domain {
            RasterDomain::Path => run!(path, PathRasterStagingSsbo),
            RasterDomain::Cobblestone1 => run!(cobblestone1, Cobblestone1RasterStagingSsbo),
            RasterDomain::Cobblestone2 => run!(cobblestone2, Cobblestone2RasterStagingSsbo),
            RasterDomain::Cobblestone3 => run!(cobblestone3, Cobblestone3RasterStagingSsbo),
            RasterDomain::Water => run!(water, WaterRasterStagingSsbo),
            RasterDomain::Garden => run!(garden, GardenRasterStagingSsbo),
        };
        if done {
            finished.push(index);
        }
    }
    for index in finished.into_iter().rev() {
        bucket.commands.remove(index);
    }
}

impl EditType {
    pub fn raster(&self) -> Option<(RasterDomain, Arc<RasterStroke>)> {
        Some(match self {
            Self::Path(s) => (RasterDomain::Path, s.clone()),
            Self::Cobblestone1(s) => (RasterDomain::Cobblestone1, s.clone()),
            Self::Cobblestone2(s) => (RasterDomain::Cobblestone2, s.clone()),
            Self::Cobblestone3(s) => (RasterDomain::Cobblestone3, s.clone()),
            Self::Water(s) => (RasterDomain::Water, s.clone()),
            Self::Garden(s) => (RasterDomain::Garden, s.clone()),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::transactions::HistoryEntry;
    use crate::resources::history::transactions::ObservedHistoryLedger;
    use crate::resources::world_raster::DirtyTilePublisher;
    fn stroke(a: f32, b: f32) -> Arc<RasterStroke> {
        Arc::new(RasterStroke {
            flags: 0,
            strength: 1.0,
            radius: 1.0,
            draw: true,
            points: vec![Vec3::new(a, 0.0, 0.0), Vec3::new(b, 0.0, 0.0)].into(),
        })
    }
    fn raster() -> WorldRaster<512> {
        WorldRaster::new(DirtyTilePublisher::unique())
    }
    fn history(a: &Arc<RasterStroke>, b: &Arc<RasterStroke>) -> History {
        History::from_observed(ObservedHistoryLedger {
            entries: vec![HistoryEntry::from_edits(vec![
                EditType::Path(a.clone()),
                EditType::Path(b.clone()),
            ])],
            cursor: 0,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        })
    }
    #[test]
    fn instant_undo_rebuilds_real_prior_strokes_and_redo_restores_pixels() {
        let (a, b) = (stroke(-4.0, -1.0), stroke(1.0, 4.0));
        let history = history(&a, &b);
        let mut world = raster();
        paint(&mut world, 0, &a, &a.points);
        paint(&mut world, 0, &b, &b.points);
        let complete = world.composite().to_vec();
        let mut expected = raster();
        paint(&mut expected, 0, &a, &a.points);
        let mut staging = vec![0; 512 * 512];
        let mut dirty = RasterReplayDirty::default();
        let mut events = Vec::new();
        for undo in [true, false] {
            let mut cmd = RasterReplayCmd::new(RasterDomain::Path, b.clone(), undo, true, true);
            assert!(advance_raster_replay(
                &mut cmd,
                0.0,
                1.0,
                &history,
                &mut world,
                &mut dirty,
                |r, selector| {
                    assert_eq!(selector, 0);
                    r.copy_to_staging(selector, &mut staging);
                },
                |event| events.push(event)
            ));
            if undo {
                assert_eq!(world.composite(), expected.composite());
            } else {
                assert_eq!(world.composite(), complete);
            }
        }
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].position, b.points[0]);
    }
    #[test]
    fn animated_replay_uses_scratch_then_commits_and_has_no_upper_duration_cap() {
        let (a, b) = (stroke(-4.0, -1.0), stroke(1.0, 4.0));
        let history = history(&a, &b);
        let mut world = raster();
        paint(&mut world, 0, &a, &a.points);
        let before = world.composite().to_vec();
        let mut dirty = RasterReplayDirty::default();
        let mut staging = vec![0; 512 * 512];
        let mut cmd = RasterReplayCmd::new(RasterDomain::Path, b.clone(), false, false, false);
        assert!(!advance_raster_replay(
            &mut cmd,
            0.2,
            1.0,
            &history,
            &mut world,
            &mut dirty,
            |r, s| r.copy_to_staging(s, &mut staging),
            |_| panic!("feedback disabled")
        ));
        assert_eq!(world.composite(), before);
        assert_ne!(world.buffer(1), before);
        assert!(advance_raster_replay(
            &mut cmd,
            0.2,
            1.0,
            &history,
            &mut world,
            &mut dirty,
            |r, s| r.copy_to_staging(s, &mut staging),
            |_| panic!("feedback disabled")
        ));
        let mut expected = raster();
        paint(&mut expected, 0, &a, &a.points);
        paint(&mut expected, 0, &b, &b.points);
        assert_eq!(world.composite(), expected.composite());
        assert_eq!(
            duration_based_on_stroke_length(&[Vec3::ZERO, Vec3::X * 1000.0]),
            5.0
        );
        assert_eq!(duration_based_on_stroke_length(&[Vec3::ZERO]), 0.4);
    }
}
