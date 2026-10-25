//! `replay_tree` (VA 0x141bc34f0, replay_tree.rs:17-121).
//! Evidence: tree-tool-rest-analysis.md §2.

use crate::resources::GameTime;
use crate::types::*;
use bevy_ecs::prelude::*;

/// Lower clamp for the per-command delay (0x3d4ccccd = 0.05).
pub const MIN_REPLAY_DELAY: f32 = 0.05;

/// TreeEdit payload. Raw variant u32 @+0x10: 1=Move, 2=Insert, other=Delete.
/// Field layout beyond the evidenced indices is UNKNOWN(evidence: tree-tool-rest-analysis.md §2).
#[derive(Debug, Clone, PartialEq)]
pub enum TreeEdit {
    Move { index: u64, position: glam::Vec3 },
    Insert(InsertTreeCmd),
    Delete { index: u64 },
}

impl TreeEdit {
    /// Raw discriminant as seen by the replay switch.
    pub fn raw_variant(&self) -> u32 {
        match self {
            TreeEdit::Move { .. } => 1,
            TreeEdit::Insert(_) => 2,
            TreeEdit::Delete { .. } => 0,
        }
    }
}

/// What a ReplayCmd (stride 0x30) carries. Tag 8 = TreeEdit; other tags belong
/// to other replay systems and are left untouched.
#[derive(Debug, Clone, PartialEq)]
pub enum ReplayPayload {
    TreeEdit(TreeEdit),
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplayCmd {
    pub payload: ReplayPayload,
    /// flags byte @+8 bit0: 0 = immediate, 1 = delayed
    pub delayed: bool,
    /// f32 @+0xc
    pub delay: f32,
    /// f32 @+0x18
    pub elapsed: f32,
}

/// ReplayBuffer front bucket + divisor f32 @+0x28.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct ReplayBuffer {
    pub front: Vec<ReplayCmd>,
    /// semantics (replay speed) UNKNOWN(evidence: tree-tool-rest-analysis.md)
    pub divisor: f32,
}

/// System: ResMut<ReplayBuffer>, EventWriter<InsertTreeCmd>, EventWriter<MoveTreeCmd>,
/// EventWriter<DeleteTreeCmd>, Res<GameTime>.
/// Per TreeEdit cmd: if elapsed == 0.0 re-emit; if not delayed -> remove; else
/// `delay = max(delay/divisor, 0.05)`; if `delay < elapsed` remove else
/// `elapsed += game_time.delta`. Removal is in descending index order.
pub fn replay_tree(
    mut buffer: ResMut<ReplayBuffer>,
    mut insert_cmds: EventWriter<InsertTreeCmd>,
    mut move_cmds: EventWriter<MoveTreeCmd>,
    mut delete_cmds: EventWriter<DeleteTreeCmd>,
    game_time: Res<GameTime>,
) {
    let divisor = buffer.divisor;
    let mut to_remove = Vec::new();
    for (i, cmd) in buffer.front.iter_mut().enumerate() {
        let ReplayPayload::TreeEdit(edit) = &cmd.payload else { continue };
        if cmd.elapsed == 0.0 {
            match edit {
                TreeEdit::Move { index, position } => {
                    move_cmds.write(MoveTreeCmd { index: *index, position: *position });
                }
                TreeEdit::Insert(cmd) => {
                    insert_cmds.write(cmd.clone());
                }
                TreeEdit::Delete { index } => {
                    delete_cmds.write(DeleteTreeCmd { index: *index });
                }
            }
        }
        if !cmd.delayed {
            to_remove.push(i);
            continue;
        }
        // NaN/zero-divisor behaviour (MAXSS operand order) UNKNOWN; plain compare used.
        let mut delay = cmd.delay / divisor;
        if delay < MIN_REPLAY_DELAY {
            delay = MIN_REPLAY_DELAY;
        }
        if delay < cmd.elapsed {
            to_remove.push(i);
        } else {
            cmd.elapsed += game_time.delta;
        }
    }
    for &i in to_remove.iter().rev() {
        buffer.front.remove(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use bevy_app::prelude::*;
    use glam::Vec3;

    fn cmd(edit: TreeEdit, delayed: bool, delay: f32, elapsed: f32) -> ReplayCmd {
        ReplayCmd { payload: ReplayPayload::TreeEdit(edit), delayed, delay, elapsed }
    }

    fn app(front: Vec<ReplayCmd>, divisor: f32, dt: f32) -> App {
        let mut app = App::new();
        app.add_event::<InsertTreeCmd>()
            .add_event::<MoveTreeCmd>()
            .add_event::<DeleteTreeCmd>()
            .init_resource::<Seen<InsertTreeCmd>>()
            .init_resource::<Seen<MoveTreeCmd>>()
            .init_resource::<Seen<DeleteTreeCmd>>()
            .insert_resource(ReplayBuffer { front, divisor })
            .insert_resource(GameTime { delta: dt })
            .add_systems(
                Update,
                (replay_tree,
                 (collect::<InsertTreeCmd>, collect::<MoveTreeCmd>, collect::<DeleteTreeCmd>))
                    .chain(),
            );
        app
    }
    fn front(app: &App) -> &Vec<ReplayCmd> {
        &app.world().resource::<ReplayBuffer>().front
    }
    fn seen<E: Event>(app: &App) -> &Vec<E> {
        &app.world().resource::<Seen<E>>().0
    }
    fn ins() -> InsertTreeCmd {
        InsertTreeCmd { name: "t".into(), position: Vec3::ZERO, height: 0.0, field_28_unknown: 0.0,
            flags: 0, byte_30_unknown: 0, index: 4 }
    }

    #[test]
    fn variants_map_and_immediate_cmds_emit_then_leave() {
        let mut a = app(vec![
            cmd(TreeEdit::Move { index: 1, position: Vec3::X }, false, 0.0, 0.0),
            cmd(TreeEdit::Insert(ins()), false, 0.0, 0.0),
            cmd(TreeEdit::Delete { index: 3 }, false, 0.0, 0.0),
        ], 1.0, 0.1);
        a.update();
        assert_eq!((seen::<MoveTreeCmd>(&a).len(), seen::<InsertTreeCmd>(&a).len(), seen::<DeleteTreeCmd>(&a).len()), (1, 1, 1));
        assert!(front(&a).is_empty());
        assert_eq!(seen::<InsertTreeCmd>(&a)[0].index, 4);
        assert_eq!(TreeEdit::Delete { index: 0 }.raw_variant(), 0);
        assert_eq!(TreeEdit::Insert(ins()).raw_variant(), 2);
    }

    #[test]
    fn delayed_cmd_accumulates_then_expires_and_emits_once() {
        // delay = 0.4 / 2 = 0.2
        let mut a = app(vec![cmd(TreeEdit::Delete { index: 1 }, true, 0.4, 0.0)], 2.0, 0.1);
        let mut frames = 0;
        while !front(&a).is_empty() {
            a.update();
            frames += 1;
            assert!(frames < 20);
        }
        assert_eq!(seen::<DeleteTreeCmd>(&a).len(), 1);
        assert_eq!(frames, 4); // elapsed 0 ->0.1 ->0.2 ->0.3 ; 0.2<0.3 removes on 4th
    }

    #[test]
    fn delay_clamped_to_min() {
        let mut a = app(vec![cmd(TreeEdit::Delete { index: 1 }, true, 1.0, 0.06)], 100.0, 0.1);
        a.update();
        assert!(front(&a).is_empty()); // 0.05 < 0.06
        let mut a = app(vec![cmd(TreeEdit::Delete { index: 1 }, true, 1.0, 0.04)], 100.0, 0.1);
        a.update();
        assert_eq!(front(&a).len(), 1);
        assert!((front(&a)[0].elapsed - 0.14).abs() < 1e-6);
    }

    #[test]
    fn other_payloads_untouched_and_removal_order_preserved() {
        let mut a = app(vec![
            ReplayCmd { payload: ReplayPayload::Other, delayed: false, delay: 0.0, elapsed: 0.0 },
            cmd(TreeEdit::Delete { index: 7 }, false, 0.0, 0.0),
            cmd(TreeEdit::Delete { index: 8 }, false, 0.0, 0.0),
        ], 1.0, 0.0);
        a.update();
        assert_eq!(front(&a).len(), 1);
        let idx: Vec<_> = seen::<DeleteTreeCmd>(&a).iter().map(|d| d.index).collect();
        assert_eq!(idx, vec![7, 8]);
    }

    #[test]
    fn zero_delta_re_emits_every_frame() {
        let mut a = app(vec![cmd(TreeEdit::Delete { index: 1 }, true, 5.0, 0.0)], 1.0, 0.0);
        a.update();
        a.update();
        assert_eq!(seen::<DeleteTreeCmd>(&a).len(), 2);
    }
}
