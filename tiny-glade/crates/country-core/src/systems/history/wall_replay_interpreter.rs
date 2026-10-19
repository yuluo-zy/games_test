//! B timed new-wall state growth and original internal concurrent-group boundary.
//! Whole native process proof is kept in evidence/building-wall-history.
use crate::resources::history::wall_edit::InstantWallReplayOutcome;
use crate::resources::history::wall_edit::WallAuthoritySnapshot;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::history::wall_edit::WallReplayAudio;
use crate::resources::history::wall_edit::WallReplaySourceError;
use crate::resources::history::wall_edit::WallSnapshotError;
use crate::resources::history::wall_edit::apply_instant_wall_edit;
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallId;
#[derive(Clone, Debug)]
pub enum WallReplayError {
    Source(WallReplaySourceError),
    /// Older portable input omitted the original byte required to restore a
    /// missing authority row. Existing rows can retain their observed value.
    MissingSource66(WallId),
    Snapshot(WallSnapshotError),
    UnrestoredOperation(u32),
    Enclosure(crate::resources::walls::freeform_height_cache::ShapeHeightError),
    UnrestoredConcurrentGroup,
}
#[derive(Clone, Debug)]
pub struct WallReplayCommand {
    pub edit: WallHistoryEdit,
    pub timed: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
    pub finished: bool,
}
#[derive(Clone, Debug, Default)]
pub struct TimedWallReplayOutcome {
    pub updated_source_notifications: Vec<WallId>,
    pub height_request:
        Option<crate::resources::history::building_transactions::WallHeightReplayRequest>,
    pub audio: Option<WallReplayAudio>,
    pub instantaneous: Option<InstantWallReplayOutcome>,
}

#[derive(Clone, Debug, Default)]
pub struct WallConcurrentReplayOutcome {
    /// In actual command order; the adapter publishes each recovered effect.
    pub effects: Vec<TimedWallReplayOutcome>,
    pub completed_groups: usize,
    pub finished: bool,
}
fn wall_id(edit: &WallHistoryEdit) -> WallId {
    // 未恢复载荷变体（5..16）无 wall 字段，当前 producer 不产生；WallId(0) 为哨兵。
    edit.wall_id().unwrap_or(WallId(0))
}

/// Full replay_wall's internal boundary: completed adjacent groups continue in
/// the same invocation only when their WallId sets are disjoint. In particular
/// ev_new_wall's Create and StateChange groups always require separate calls,
/// including instant replay. This never pops or enters an outer ReplayBucket.
pub fn advance_wall_groups(
    groups: &mut std::collections::VecDeque<Vec<WallReplayCommand>>,
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    dt: f32,
    terrain: &crate::resources::walls::publication_source::TerrainHeightGrid512,
) -> Result<WallConcurrentReplayOutcome, WallReplayError> {
    advance_wall_groups_observed(groups, sources, dt, terrain, |_, _| Ok(()))
}

/// Same interpreter with an adapter observation point after each command.
/// This preserves A→B→A effects across disjoint adjacent groups without
/// reconstructing intermediate writes from the final map. The callback does
/// not own replay timing or group advancement.
pub fn advance_wall_groups_observed(
    groups: &mut std::collections::VecDeque<Vec<WallReplayCommand>>,
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    dt: f32,
    terrain: &crate::resources::walls::publication_source::TerrainHeightGrid512,
    mut observe: impl FnMut(
        &TimedWallReplayOutcome,
        &std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    ) -> Result<(), WallReplayError>,
) -> Result<WallConcurrentReplayOutcome, WallReplayError> {
    let mut out = WallConcurrentReplayOutcome::default();
    loop {
        let Some(front) = groups.front_mut() else {
            out.finished = true;
            break;
        };
        for command in front.iter_mut() {
            let effect = crate::systems::history::wall_replay_transition::advance_wall_transition(
                command, sources, dt, terrain,
            )?;
            observe(&effect, sources)?;
            out.effects.push(effect);
        }
        if front.iter().any(|command| !command.finished) {
            break;
        }
        let current: std::collections::HashSet<_> =
            front.iter().map(|c| wall_id(&c.edit)).collect();
        let next: std::collections::HashSet<_> = groups
            .get(1)
            .into_iter()
            .flatten()
            .map(|c| wall_id(&c.edit))
            .collect();
        let disjoint = current.is_disjoint(&next);
        groups.pop_front();
        out.completed_groups += 1;
        if !disjoint {
            break;
        }
    }
    out.finished = groups.is_empty();
    Ok(out)
}

/// B closure 140abf660 / mapper 1414769d0 allocates and maps every item in
/// each original group, including zero-length and multi-operation groups.
/// Undo reverses the sequence of groups; it does not flatten their commands.
pub fn expand_wall_compound(
    edit: &WallCompoundHistoryEdit,
    undo: bool,
    instant: bool,
) -> Result<std::collections::VecDeque<Vec<WallReplayCommand>>, WallReplayError> {
    let mut groups: Vec<_> = edit
        .groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|e| WallReplayCommand {
                    edit: e.clone(),
                    timed: !instant
                        && matches!(
                            e,
                            WallHistoryEdit::StateChange { .. }
                                | WallHistoryEdit::AdjustHeight { .. }
                                | WallHistoryEdit::MorphPremadeShape(..)
                                | WallHistoryEdit::ChangeElevation(..)
                                | WallHistoryEdit::ChangeArchHeight(..)
                                | WallHistoryEdit::MorphFreehandWall(..)
                                | WallHistoryEdit::DeprecatedMorphOriginal(..)
                        ),
                    duration: 0.4,
                    elapsed: 0.0,
                    undo,
                    finished: false,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    if undo {
        groups.reverse();
    }
    Ok(groups.into())
}

fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let x = (t + t) + (-2.0);
        ((0.5 * x) * x) * x + 1.0
    }
}
fn divide(a: f32, b: f32) -> f32 {
    let v = a / b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
impl WallReplayCommand {
    /// Original progress is evaluated before elapsed+=dt. Finished commands
    /// skip without advancing. Constructor growth keeps the seed enclosure and
    /// metadata until completion, changing only curve/elevation along the way.
    pub fn advance(
        &mut self,
        sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
        dt: f32,
        terrain: &crate::resources::walls::publication_source::TerrainHeightGrid512,
    ) -> Result<TimedWallReplayOutcome, WallReplayError> {
        let mut out = TimedWallReplayOutcome::default();
        if self.finished {
            return Ok(out);
        }
        // Native operation5 owns geometric interpolation; operation4 is a
        // discrete IsEnclosed update and never reaches the timed branch.
        if let WallHistoryEdit::MorphPremadeShape(edit) = &self.edit {
            use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
            let wall = WallId(edit.wall_index);
            let (a, b) = if self.undo {
                (edit.after, edit.before)
            } else {
                (edit.before, edit.after)
            };
            let ratio = divide(self.elapsed, self.duration);
            let p = cubic_in_out(if ratio < 0.0 {
                0.0
            } else if ratio > 1.0 {
                1.0
            } else {
                ratio
            });
            let shape = if !self.timed {
                b
            } else {
                match (a.get_shape(), b.get_shape()) {
                    (Some(Shape::Circle(a)), Some(Shape::Circle(b))) => {
                        ShapeParameters::from_shape(Shape::Circle(a.lerp(b, p)))
                    }
                    (Some(Shape::Rectangle(_)), Some(Shape::Rectangle(_))) => {
                        ShapeParameters::from_snapshot(
                            &crate::resources::history::wall_edit::interpolate_rectangle_enclosure(
                                &a.to_snapshot(),
                                &b.to_snapshot(),
                                p,
                            )
                            .map_err(WallReplayError::Enclosure)?,
                        )
                    }
                    _ => return Err(WallReplayError::UnrestoredOperation(5)),
                }
            };
            if !shape.is_premade() {
                return Err(WallReplayError::UnrestoredOperation(5));
            }
            let curve = shape.as_curve2();
            if let Some(target) = sources.get_mut(&wall) {
                target.source.shape = shape.to_snapshot();
                target.source.curve_kind_tag = shape.tag();
                out.updated_source_notifications.push(wall);
                target.source.curve = crate::resources::roofs::geometry::surface::Curve2 {
                    points: curve.points.into_iter().map(|p| p.to_array()).collect(),
                    points_u: curve.points_u,
                    length: curve.length,
                };
                out.updated_source_notifications.push(wall);
            }
            if self.timed {
                let c = b.center();
                out.audio = Some(WallReplayAudio {
                    wall,
                    anchor: [c.x, terrain.sample_at_world_xz(c.x, c.y), c.y],
                    kind: 0,
                    flag: 1,
                });
                self.elapsed = add(self.elapsed, dt);
            }
            self.finished = !self.timed || p >= 1.0 || p.is_nan();
            return Ok(out);
        }
        if let WallHistoryEdit::MorphFreehandWall(edit) = &self.edit {
            let ratio = divide(self.elapsed, self.duration);
            let p = if self.timed {
                cubic_in_out(if ratio < 0.0 {
                    0.0
                } else if ratio > 1.0 {
                    1.0
                } else {
                    ratio
                })
            } else {
                1.0
            };
            let t = if self.undo { 1.0 - p } else { p };
            // _deprecated_morph_curve140952570 clones before, with its cached
            // u/length, exactly at t==0. Every other factor calls the existing
            // crashy_lerp utility and retains its equal-count panic domain.
            let curve = if t == 0.0 {
                crate::resources::roofs::geometry::surface::Curve2 {
                    points: edit.before.points.clone(),
                    points_u: edit.before.points_u.clone(),
                    length: edit.before.length,
                }
            } else {
                let owned = |c: &crate::resources::history::wall_edit::WallCurveHistory| {
                    utils::curve::Curve2::from_cached_parts(
                        c.points
                            .iter()
                            .copied()
                            .map(glam::Vec2::from_array)
                            .collect(),
                        c.points_u.clone(),
                        c.length,
                    )
                };
                let curve = owned(&edit.before).crashy_lerp(&owned(&edit.after), t);
                crate::resources::roofs::geometry::surface::Curve2 {
                    points: curve.points.into_iter().map(|p| p.to_array()).collect(),
                    points_u: curve.points_u,
                    length: curve.length,
                }
            };
            let wall = WallId(edit.wall_index);
            if let Some(target) = sources.get_mut(&wall) {
                target.source.curve = curve;
                out.updated_source_notifications.push(wall);
            }
            self.finished = !self.timed || p >= 1.0 || p.is_nan();
            if self.timed {
                self.elapsed = add(self.elapsed, dt);
            }
            return Ok(out);
        }
        if let WallHistoryEdit::ChangeElevation(edit) | WallHistoryEdit::ChangeArchHeight(edit) =
            &self.edit
        {
            let wall = WallId(edit.wall_index);
            let ratio = divide(self.elapsed, self.duration);
            let p = cubic_in_out(if ratio < 0.0 {
                0.0
            } else if ratio > 1.0 {
                1.0
            } else {
                ratio
            });
            let (a, b) = if self.undo {
                (edit.after, edit.before)
            } else {
                (edit.before, edit.after)
            };
            let value = if !self.timed || p >= 1.0 || p.is_nan() {
                b
            } else {
                add(a * (1.0 - p), b * p)
            };
            if let Some(target) = sources.get_mut(&wall) {
                if matches!(self.edit, WallHistoryEdit::ChangeElevation(..)) {
                    target.source.terrain_floor = value;
                } else {
                    target.source.observed_path_scalar = value;
                }
                out.updated_source_notifications.push(wall);
            }
            self.finished = !self.timed || p >= 1.0 || p.is_nan();
            if self.timed {
                self.elapsed = add(self.elapsed, dt);
            }
            return Ok(out);
        }
        if !self.timed {
            self.finished = true;
            // fresh 0x140abfc49..：instant 先写 done 字节（+0x15），再按
            // instant 跳表 0x142adf408 分派 17 操作。
            match self.edit.operation_tag() {
                // 变体 15（Unresolved15）→ panic_deprecated_operation
                // （0x140ac15b7，消息 "WallHistoryEdit::MarkEnclosure"）。
                crate::resources::history::wall_edit::WallHistoryOperationTag::Unresolved15 => {
                    panic!("WallHistoryEdit::MarkEnclosure")
                }
                // AdjustHeight（0x140ac042c）：取 undo^1 侧的高度值写 height 请求。
                crate::resources::history::wall_edit::WallHistoryOperationTag::AdjustHeight => {
                    if let WallHistoryEdit::AdjustHeight { wall, from, to } = &self.edit {
                        out.height_request = Some(
                            crate::resources::history::building_transactions::WallHeightReplayRequest {
                                wall_id: wall.0,
                                absolute_height: if self.undo { *from } else { *to },
                                observed_audio_flag: 1,
                            },
                        );
                    } else {
                        return Err(WallReplayError::UnrestoredOperation(u32::MAX));
                    }
                }
                tag => {
                    out.instantaneous = Some(
                        apply_instant_wall_edit(sources, &self.edit, self.undo, terrain)
                            .map_err(WallReplayError::Source)?,
                    );
                    let _ = tag;
                }
            }
            return Ok(out);
        }
        // timed 路径（fresh 0x140abfbd0..：先 cubic_in_out(elapsed/duration 夹取)
        // 再按 timed 跳表 0x142adf44c 分派）。
        match self.edit.operation_tag() {
            // Native operation3 emits the height command consumed by InnerWalls.
            crate::resources::history::wall_edit::WallHistoryOperationTag::AdjustHeight => {
                let WallHistoryEdit::AdjustHeight { wall, from, to } = &self.edit else {
                    return Err(WallReplayError::UnrestoredOperation(u32::MAX));
                };
                let ratio = divide(self.elapsed, self.duration);
                let ratio = if ratio < 0.0 {
                    0.0
                } else if ratio > 1.0 {
                    1.0
                } else {
                    ratio
                };
                let progress = cubic_in_out(ratio);
                if progress >= 1.0 || progress.is_nan() {
                    let value = if self.undo { *from } else { *to };
                    out.height_request = Some(
                        crate::resources::history::building_transactions::WallHeightReplayRequest {
                            wall_id: wall.0,
                            absolute_height: value,
                            observed_audio_flag: 1,
                        },
                    );
                    self.finished = true;
                } else {
                    let (a, b) = if self.undo { (*to, *from) } else { (*from, *to) };
                    out.height_request = Some(
                        crate::resources::history::building_transactions::WallHeightReplayRequest {
                            wall_id: wall.0,
                            absolute_height: add((1.0 - progress) * a, b * progress),
                            observed_audio_flag: 1,
                        },
                    );
                }
                self.elapsed = add(self.elapsed, dt);
                return Ok(out);
            }
            // timed 跳表允许动画但载荷未恢复的变体（5/6/7/13/14）→ 登记缺类型。
            crate::resources::history::wall_edit::WallHistoryOperationTag::MorphPremadeShape
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeElevation
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeArchHeight
            | crate::resources::history::wall_edit::WallHistoryOperationTag::MorphFreehandWall
            | crate::resources::history::wall_edit::WallHistoryOperationTag::DeprecatedMorphOriginal => {
                return Err(WallReplayError::UnrestoredOperation(
                    self.edit.operation_tag() as u32,
                ))
            }
            // timed 跳表 0x142adf44c 的其余项跳 0x140ac1576：
            // "CreateWall, DeleteWall, MarkEnclosure, ChangeArchType and ChangeSkirt
            //  do not implement animation. The code should not have reached here"
            // （0x142adf1e0 len 0x87；panic_fmt）。
            crate::resources::history::wall_edit::WallHistoryOperationTag::Unresolved15 => {
                panic!("WallHistoryEdit::MarkEnclosure")
            }
            crate::resources::history::wall_edit::WallHistoryOperationTag::CreateWall
            | crate::resources::history::wall_edit::WallHistoryOperationTag::DeleteWall
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeIsEnclosedState
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeSupports
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeSkirt
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangePlasterPeelAmount
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeFreehandHeightType
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeArchType
            | crate::resources::history::wall_edit::WallHistoryOperationTag::ChangeStyle => panic!(
                "CreateWall, DeleteWall, MarkEnclosure, ChangeArchType and ChangeSkirt do not implement animation. The code should not have reached here"
            ),
            _ => {}
        }
        let WallHistoryEdit::StateChange {
            wall,
            previous,
            next,
            observed_operation,
        } = &self.edit
        else {
            return Err(WallReplayError::UnrestoredOperation(u32::MAX));
        };
        let operation = u32::from_le_bytes(observed_operation[..4].try_into().unwrap());
        if operation != 0 {
            return Err(WallReplayError::UnrestoredOperation(operation));
        }
        let ratio = divide(self.elapsed, self.duration);
        let ratio = if ratio < 0.0 { 0.0 } else { ratio };
        let ratio = if ratio > 1.0 { 1.0 } else { ratio };
        let progress = cubic_in_out(ratio);
        if progress >= 1.0 || progress.is_nan() {
            if let Some(target) = sources.get_mut(wall) {
                *target = if self.undo { previous } else { next }.clone();
                out.updated_source_notifications.push(*wall);
            }
            self.finished = true;
        } else {
            let progress = if self.undo { 1.0 - progress } else { progress };
            let old = previous.source.extra;
            let new = next.source.extra;
            if old != new || old.is_nan() || new.is_nan() {
                out.height_request = Some(
                    crate::resources::history::building_transactions::WallHeightReplayRequest {
                        wall_id: wall.0,
                        absolute_height: add((1.0 - progress) * old, new * progress),
                        observed_audio_flag: 1,
                    },
                );
            }
            let start = f32::from_le_bytes(observed_operation[4..8].try_into().unwrap());
            let end = f32::from_le_bytes(observed_operation[8..12].try_into().unwrap());
            let low = add(start * (1.0 - progress), 0.0 * progress);
            let high = add((1.0 - progress) * end, progress);
            let curve = slice_source_curve(&next.source.curve, low, high)
                .map_err(WallReplayError::Snapshot)?;
            if old == new && curve.points.len() >= 2 && curve.length > 0.0 {
                let [x, z] = *curve.points.last().unwrap();
                out.audio = Some(WallReplayAudio {
                    wall: *wall,
                    anchor: [x, terrain.sample_at_world_xz(x, z), z],
                    kind: 2,
                    flag: 1,
                });
            }
            let x = 1.0 - progress;
            let elevation_progress = 1.0 - ((x * x) * x);
            let floor = add(
                previous.source.terrain_floor * (1.0 - elevation_progress),
                next.source.terrain_floor * elevation_progress,
            );
            if let Some(target) = sources.get_mut(wall) {
                target.source.curve = curve;
                out.updated_source_notifications.push(*wall);
                target.source.terrain_floor = floor;
                out.updated_source_notifications.push(*wall);
            }
        }
        self.elapsed = add(self.elapsed, dt);
        Ok(out)
    }
}

#[cfg(test)]
mod dispatch_tests {
    use super::*;
    use crate::resources::history::wall_edit::UnresolvedWallEditPayload;
    use crate::resources::history::wall_edit::WallAuthoritySnapshot;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::inner_wall_state::WallState;

    fn wall_snapshot() -> WallAuthoritySnapshot {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        WallState::new(curve, 2.0, 3, 0, false).snapshot().clone()
    }
    use crate::resources::walls::publication_source::TerrainHeightGrid512;

    fn terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }
    }

    fn command(edit: WallHistoryEdit, timed: bool, undo: bool) -> WallReplayCommand {
        WallReplayCommand {
            edit,
            timed,
            duration: 0.4,
            elapsed: 0.0,
            undo,
            finished: false,
        }
    }

    fn sources() -> std::collections::HashMap<WallId, WallAuthoritySnapshot> {
        std::collections::HashMap::new()
    }

    #[test]
    fn original_freehand_curve_and_discrete_policy_replay_redo_then_undo() {
        let rows: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-wall-variants-b.json"
        ))
        .unwrap();
        let edit: WallHistoryEdit =
            serde_json::from_value(rows["MorphFreehandWall"].clone()).unwrap();
        let wall = edit.wall_id().unwrap();
        let before = rows["MorphFreehandWall"]["MorphFreehandWall"]["before"].clone();
        let mut states = std::collections::HashMap::from([(wall, wall_snapshot())]);
        let effect = command(edit.clone(), false, false)
            .advance(&mut states, 0.1, &terrain())
            .unwrap();
        assert_eq!(effect.updated_source_notifications, [wall]);
        assert_eq!(states[&wall].source.curve.points[0], [2.958064, 0.5108361]);
        command(edit, false, true)
            .advance(&mut states, 0.1, &terrain())
            .unwrap();
        assert_eq!(states[&wall].source.curve.points_u, [0.0, 1.0]);
        assert_eq!(
            states[&wall].source.curve.length,
            before["length"].as_f64().unwrap() as f32
        );
        assert_eq!(
            states[&wall].source.curve.points[0],
            [-1.1893375, 0.37190735]
        );
        let edit: WallHistoryEdit = serde_json::from_value(rows["ChangeArchType"].clone()).unwrap();
        let wall = edit.wall_id().unwrap();
        states.insert(wall, wall_snapshot());
        assert_eq!(edit.operation_tag() as u64, 8);
        command(edit.clone(), false, false)
            .advance(&mut states, 0.1, &terrain())
            .unwrap();
        assert_eq!(states[&wall].source.observed_source_flag_64, 1);
        command(edit, false, true)
            .advance(&mut states, 0.1, &terrain())
            .unwrap();
        assert_eq!(states[&wall].source.observed_source_flag_64, 0);
        // Arch type must not overwrite the independent support byte (native12).
        assert_eq!(states[&wall].metadata.source_67, 0);
    }

    #[test]
    fn instant_adjust_height_emits_undo_selected_height_and_marks_done() {
        // fresh 0x140ac042c：`[edit+0x20/0x24]` 由 undo^1 选侧，写 height 事件。
        let mut cmd = command(
            WallHistoryEdit::AdjustHeight {
                wall: WallId(3),
                from: 2.0,
                to: 5.0,
            },
            false,
            false,
        );
        let out = cmd.advance(&mut sources(), 0.1, &terrain()).unwrap();
        assert!(cmd.finished);
        let request = out.height_request.unwrap();
        assert_eq!(request.wall_id, 3);
        assert_eq!(request.absolute_height, 5.0);

        let mut undo = command(
            WallHistoryEdit::AdjustHeight {
                wall: WallId(3),
                from: 2.0,
                to: 5.0,
            },
            false,
            true,
        );
        let out = undo.advance(&mut sources(), 0.1, &terrain()).unwrap();
        assert_eq!(out.height_request.unwrap().absolute_height, 2.0);
    }

    #[test]
    fn timed_adjust_height_interpolates_then_finishes() {
        let mut cmd = command(
            WallHistoryEdit::AdjustHeight {
                wall: WallId(3),
                from: 0.0,
                to: 4.0,
            },
            true,
            false,
        );
        let out = cmd.advance(&mut sources(), 0.1, &terrain()).unwrap();
        assert!(!cmd.finished);
        // 第一帧 ratio=0 → eased=0 → 取 from。
        assert_eq!(out.height_request.unwrap().absolute_height, 0.0);
        let out = cmd.advance(&mut sources(), 0.1, &terrain()).unwrap();
        assert!(out.height_request.unwrap().absolute_height > 0.0);
        // duration 0.4 → 后续帧完成。
        for _ in 0..5 {
            let _ = cmd.advance(&mut sources(), 0.1, &terrain()).unwrap();
        }
        assert!(cmd.finished);
    }

    #[test]
    #[should_panic(expected = "do not implement animation")]
    fn timed_create_wall_panics_with_observed_message() {
        // fresh timed 跳表 case 1 → 0x140ac1576 panic_fmt（0x142adf1e0）。
        let mut cmd = command(
            WallHistoryEdit::Create {
                wall: WallId(1),
                source: wall_snapshot(),
                color_id: 0,
                restored_height: 1.0,
                restored_elevation: 0.0,
            },
            true,
            false,
        );
        let _ = cmd.advance(&mut sources(), 0.1, &terrain());
    }

    #[test]
    #[should_panic(expected = "WallHistoryEdit::MarkEnclosure")]
    fn unresolved15_panics_deprecated_operation() {
        let mut cmd = command(
            WallHistoryEdit::Unresolved15(UnresolvedWallEditPayload),
            false,
            false,
        );
        let _ = cmd.advance(&mut sources(), 0.1, &terrain());
    }

    #[test]
    fn plaster_peel_instant_replay_preserves_selected_bits_and_direction() {
        use crate::resources::history::wall_edit::WallValueEdit;
        let mut map = sources();
        map.insert(WallId(1), wall_snapshot());
        let mut cmd = command(
            WallHistoryEdit::ChangePlasterPeelAmount(WallValueEdit {
                wall_index: 1,
                before: -0.0,
                after: f32::from_bits(0x3f12_3456),
            }),
            false,
            false,
        );
        let effect = cmd.advance(&mut map, 0.1, &terrain()).unwrap();
        assert_eq!(map[&WallId(1)].metadata.source_60_bits, 0x3f12_3456);
        assert_eq!(
            effect.instantaneous.unwrap().updated_source_notification,
            Some(WallId(1))
        );
        cmd.undo = true;
        cmd.finished = false;
        cmd.advance(&mut map, 0.1, &terrain()).unwrap();
        assert_eq!(
            map[&WallId(1)].metadata.source_60_bits,
            (-0.0_f32).to_bits()
        );
    }
}
