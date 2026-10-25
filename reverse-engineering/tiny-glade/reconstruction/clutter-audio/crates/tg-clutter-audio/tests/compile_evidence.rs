//! tg-clutter-audio 编译验证 + 证据表抽查（B 主基线）。
//!
//! 这些测试只验证草稿接入后可编译、且已证据化的常量表/计算与反汇编记录一致；
//! 不构成原游戏行为验收。

use tg_clutter_audio::audio::music::{compute_music_richness, compute_new_glade_time, snap_off_boundaries, MusicAudioStateFields};
use tg_clutter_audio::audio::stairs::{stair_kind, StairKind};
use tg_clutter_audio::audio::wall_kinds::{wall_kind, WallKind};
use tg_clutter_audio::clutter::auto_clutter_edit::{replay_auto_clutter, AutoClutterEdit, EditedClutterProtection, ReplayCommand};

#[test]
fn autoclutter_edit_serde_matches_evidence() {
    // 证据：serialize 0x14003d300 写 {"Delete":{"id":<u64>}}（外层 "Delete"、内层 "id"）。
    let json = serde_json::to_string(&AutoClutterEdit::Delete { id: 5 }).unwrap();
    assert_eq!(json, r#"{"Delete":{"id":5}}"#);
    let back: AutoClutterEdit = serde_json::from_str(r#"{"Delete":{"id":5}}"#).unwrap();
    assert!(matches!(back, AutoClutterEdit::Delete { id: 5 }));
}

#[test]
fn stair_kind_table_matches_evidence() {
    // 跳表 0x142bad9f0（kind 0–18）。
    assert!(matches!(stair_kind(0), StairKind::Loop(0)));
    assert!(matches!(stair_kind(3), StairKind::OneShot("event:/SFX/Stairs/StairPlace")));
    assert!(matches!(stair_kind(16), StairKind::Ignored));
    assert!(matches!(stair_kind(18), StairKind::OneShot("event:/SFX/Stairs/StairSupportAppear")));
    assert!(matches!(stair_kind(19), StairKind::Ignored)); // 越界
}

#[test]
fn wall_kind_table_matches_evidence() {
    // 跳表 0x142ab4998（kind 0–23）。
    match wall_kind(1) {
        WallKind::LoopParam { loop_index, param, inverse_lerp_bounds } => {
            assert_eq!(*loop_index, 1);
            assert_eq!(*param, "WallHeight");
            assert_eq!(*inverse_lerp_bounds, (0.5, 14.0));
        }
        _ => panic!("kind 1 应为 LoopParam(WallHeight)"),
    }
    match wall_kind(10) {
        WallKind::LoopBoolParam { loop_index, param, flag_offset } => {
            assert_eq!(*loop_index, 5);
            assert_eq!(*param, "HasBricks");
            assert_eq!(*flag_offset, 0x9);
        }
        _ => panic!("kind 10 应为 LoopBoolParam(HasBricks)"),
    }
    assert!(matches!(wall_kind(24), WallKind::Ignored)); // 越界
}

#[test]
fn music_richness_boundaries_match_evidence() {
    // 证据常量 0x142ad58e4/0x142ad58e0/0x142ad58dc；区间 [0.24,0.25) / [0.49,0.5] / [0.74,0.75]。
    assert_eq!(snap_off_boundaries(0.24), 0.251);
    assert_eq!(snap_off_boundaries(0.2499), 0.251);
    assert_eq!(snap_off_boundaries(0.25), 0.25); // 上半开，0.25 不属于该带
    assert_eq!(snap_off_boundaries(0.5), 0.501);
    assert_eq!(snap_off_boundaries(0.74), 0.751);
    assert_eq!(snap_off_boundaries(0.75), 0.751);
    assert_eq!(snap_off_boundaries(0.3), 0.3);

    let mut s = MusicAudioStateFields::default();
    assert_eq!(compute_new_glade_time(0.5, &mut s), 0.5);
    assert_eq!(compute_new_glade_time(0.5, &mut s), 0.8); // clamp(0.5+0.5, 0, 0.8) 上限 0.8
    // 0 + 120/120 → clamp/snap → 1.0。
    assert_eq!(compute_music_richness(120.0, &mut s, 0), 1.0);
}

#[test]
fn replay_auto_clutter_removes_completed_commands() {
    // replay_auto_clutter（0x140aa4a60）：f = clamp(count/delta, 0.05)；f < grow → 移除；
    // 否则 grow += delta；grow==0 抹保护。
    let mut cmds = vec![ReplayCommand {
        tag: 9,
        marked: false,
        count: 0.5,
        wall: 7,
        grow: 0.0,
        flag: 0,
        _rest: [0; 16],
    }];
    let mut prot = EditedClutterProtection { walls: Default::default() };
    replay_auto_clutter(0.1, &mut cmds, &mut prot);
    // delta=0.1 → f=5.0 > grow=0.0 → 生长推进，命令保留，墙 7 进保护集。
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0].grow, 0.1);
    assert!(prot.walls.contains_key(&7));

    // grow 越过 f → 移除。
    let mut cmds = vec![ReplayCommand {
        tag: 9,
        marked: false,
        count: 0.5,
        wall: 8,
        grow: 6.0,
        flag: 1,
        _rest: [0; 16],
    }];
    let mut prot = EditedClutterProtection { walls: Default::default() };
    replay_auto_clutter(0.1, &mut cmds, &mut prot);
    assert!(cmds.is_empty());
}
