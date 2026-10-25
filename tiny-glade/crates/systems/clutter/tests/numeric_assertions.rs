//! 已判等函数 / 原生权重的数值断言（tester-numeric-assertions，2026-10-17）。
//!
//! 证据：
//! - `populate_autoclutter_regions` 0x1422d6ed0：`reports/wave2/clutter/RESTORE.md`
//!   §2「kind → 网格槽表（写块逐槽提取，已编码 KIND_MESH_SLOTS）」+ reviewer
//!   分歧点①（kind-0 mesh-10 权重 1.0 → **1.5**）已修并复核。
//! - `select_mesh` mesh-type 跳表（DAT_14304382c）：3/4 → 固定候选槽 `0x25/0x26`，
//!   2 → 跳过；加权选中算子 `do{acc+=w/total; idx++}while(acc<=r)`（reviewer ②已修）。
//! - `weighted_mesh_landing`：落点 `maxps(0.01, Δ) → floorf → xvy`（0.01 = 0x1429cd300）。
//!
//! 未判等（不写期望）：`construct_inter_roof_merlons`（0x1422d4870，RESTORE **未实现**）的
//! 0.46/0.21/0.368 城垛公式；`cell_center_world`（注释自标 I 级近似）。

use glam::{Vec2, Vec3};
use system_clutter::autoclutter::ClutterRegionId;
use system_clutter::autoclutter::populate_autoclutter_regions::{
    MeshEntry, MeshGroupWeights, circular_position, mesh_slots_for_kind, select_mesh,
    weighted_mesh_landing,
};

fn slot_weight(slots: &[(u8, f32)], mesh: u8) -> Option<f32> {
    slots.iter().find(|(id, _)| *id == mesh).map(|(_, w)| *w)
}

/// kind 0/2/3/4 槽数与逐槽权重（0x1422d6ed0 写块直读）。
#[test]
fn kind_mesh_slot_tables_native_weights() {
    let kind0 = mesh_slots_for_kind(0).expect("kind 0 table");
    assert_eq!(kind0.len(), 9);
    assert_eq!(
        kind0,
        &[
            (9, 1.2),
            (8, 1.2),
            (3, 2.5),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.5),
            (11, 1.0),
        ]
    );
    // reviewer 分歧点①：mesh-10 权重 1.5（scale×1.5 链）。
    assert_eq!(slot_weight(kind0, 10), Some(1.5));

    let kind2 = mesh_slots_for_kind(2).expect("kind 2 table");
    assert_eq!(kind2.len(), 25);
    assert_eq!(kind2[0], (10, 1.5));
    assert_eq!(kind2[1], (25, 0.75));
    assert_eq!(kind2[2], (24, 0.5));
    assert_eq!(kind2[14], (3, 2.0));
    assert_eq!(kind2[15], (18, 1.5));
    assert_eq!(slot_weight(kind2, 28), Some(0.5));
    assert_eq!(slot_weight(kind2, 29), Some(0.5));
    assert_eq!(slot_weight(kind2, 26), Some(0.5));
    assert_eq!(slot_weight(kind2, 27), Some(0.5));
    // 权重总和 = 22.9（逐槽手算：1.5+.75+.5+1+.75+.75+.5+.75+.75+.75+1+1+1.2+1.2
    //          +2.0+1.5+1+1+1+1+1+.5+.5+.5+.5）。
    let total: f32 = kind2.iter().map(|(_, w)| *w).sum();
    assert!((total - 22.9).abs() < 1e-5, "kind2 total={total}");

    let kind3 = mesh_slots_for_kind(3).expect("kind 3 table");
    assert_eq!(kind3.len(), 25);
    assert_eq!(kind3[6], (17, 0.75));
    assert_eq!(kind3[8], (22, 0.5));
    assert_eq!(kind3[14], (26, 1.2));

    let kind4 = mesh_slots_for_kind(4).expect("kind 4 table");
    assert_eq!(kind4.len(), 37);
    assert!(kind4.iter().all(|(_, weight)| *weight == 1.0));

    // kind 1 独立处理器 / default 5–25 为 U，不伪造槽表。
    assert!(mesh_slots_for_kind(1).is_none());
    assert!(mesh_slots_for_kind(5).is_none());
}

/// mesh-type 跳表固定分派：2 跳过、3 → 0x25、4 → 0x26；0/1 加权选中必落在槽表。
#[test]
fn select_mesh_dispatch_constants() {
    let key = ClutterRegionId(0x1234_5678_9abc_def0);
    assert_eq!(select_mesh(2, 0, key), None);
    assert_eq!(select_mesh(3, 0, key), Some(0x25));
    assert_eq!(select_mesh(4, 0, key), Some(0x26));
    assert_eq!(select_mesh(0, 5, key), None); // 无槽表 → None。

    let slots = mesh_slots_for_kind(0).unwrap();
    let picked = select_mesh(0, 0, key).expect("weighted pick");
    assert!(
        slots.iter().any(|(id, _)| *id == picked),
        "picked {picked} not in kind-0 table"
    );
    // 确定性：同键同结果。
    assert_eq!(select_mesh(0, 0, key), Some(picked));
    // mesh_type 1 与 0 同表分派。
    assert!(select_mesh(1, 0, key).is_some());
}

/// `weighted_mesh_landing`：权重归一 + 位面 `max(0.01, Δ)` → floor → xvy。
#[test]
fn weighted_mesh_landing_floors_and_clamps() {
    let groups = MeshGroupWeights {
        group_0: vec![MeshEntry {
            weight: 1.0,
            offset: Vec2::ZERO,
        }],
        group_1: Vec::new(),
        halloween: Vec::new(),
    };
    let key = ClutterRegionId(7);
    let landed = weighted_mesh_landing(key, &groups, 0, 1.9, 2.1);
    assert_eq!(landed, Vec3::new(1.0, 0.0, 2.0));
    // maxps(0.01, Δ)：负坐标夹到 0.01 后 floor = 0。
    let clamped = weighted_mesh_landing(key, &groups, 0, -5.0, -5.0);
    assert_eq!(clamped, Vec3::new(0.0, 0.0, 0.0));
    // 偏移参与 Δ 相加。
    let offset_groups = MeshGroupWeights {
        group_0: vec![MeshEntry {
            weight: 1.0,
            offset: Vec2::new(0.5, 0.5),
        }],
        group_1: Vec::new(),
        halloween: Vec::new(),
    };
    assert_eq!(
        weighted_mesh_landing(key, &offset_groups, 0, 1.9, 2.1),
        Vec3::new(2.0, 0.0, 2.0)
    );
}

/// `circular_position`：θ = rng×2π 的单位圆落点（x²+z²=1）。
#[test]
fn circular_position_is_unit_circle() {
    for key in [0u64, 1, 0xdead_beef, u64::MAX] {
        let p = circular_position(ClutterRegionId(key));
        assert!((p.x * p.x + p.z * p.z - 1.0).abs() < 1e-6, "key={key} p={p:?}");
        assert_eq!(p.y, 0.0);
        assert_eq!(p, circular_position(ClutterRegionId(key)));
    }
}
