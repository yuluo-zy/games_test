//! edit_rules.rs — 屋顶编辑数值核心（WP9，复用 prior work）。
//!
//! 机器码验证实现：`reconstruction/roof/edit_rules.rs`（4514 原指令案例通过；
//! `evidence/source-edits/findings.md`）。本文件为其骨架对齐移植：事件 64B、模式
//! 0=Keep/1=Absolute/2=Delta、字段 clamp、反馈事件。数学复用 glam 0.29.3。

#![allow(dead_code, unused_imports)]

use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

#[derive(Clone, Copy, Debug)]
pub struct ScalarEdit {
    pub mode: u32,
    pub value: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct TipEdit {
    pub mode: u32,
    pub value: Vec2,
}
#[derive(Clone, Copy, Debug)]
pub struct RoofEditEvent {
    pub profile: ScalarEdit,
    pub height: ScalarEdit,
    pub ridge: ScalarEdit,
    pub eave: ScalarEdit,
    pub wall_id: u64,
    pub tip: TipEdit,
    pub source_flag: u8,
}
impl RoofEditEvent {
    pub fn from_observed(b: &[u8; 64]) -> Self {
        let u = |o| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        let f = |o| f32::from_bits(u(o));
        let scalar = |o| ScalarEdit { mode: u(o), value: f(o + 4) };
        Self {
            profile: scalar(0),
            height: scalar(8),
            ridge: scalar(16),
            eave: scalar(24),
            wall_id: u64::from_le_bytes(b[32..40].try_into().unwrap()),
            tip: TipEdit { mode: u(40), value: Vec2::new(f(44), f(48)) },
            source_flag: b[52],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EditChanges {
    pub profile: f32,
    pub height: f32,
    pub ridge: f32,
    pub eave: f32,
    pub tip_distance: f32,
}
#[derive(Clone, Copy, Debug)]
pub enum EditFeedback {
    GableEnter { pivot: Vec3 },
    GableLeave { pivot: Vec3 },
    Scalar { gable_after: bool, ceil_height: f32, ceil_other: f32, weight: f32, pivot: Vec3 },
    Tip { current_distance: f32, change_distance: f32, pivot: Vec3 },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeightCrossing {
    Down,
    Up,
}
#[derive(Clone, Debug)]
pub struct EditOutcome {
    pub changed_fields: u8,
    pub changes: EditChanges,
    pub previous_height: f32,
    pub new_height: f32,
    pub gable_before_ridge: bool,
    pub gable_after_ridge: bool,
    pub rebuild_gable: bool,
    pub feedback: Vec<EditFeedback>,
    pub height_crossing: Option<HeightCrossing>,
    pub resolved_update: bool,
}

// ---- 已验证数值规则（edit_rules.rs 移植，机器码等价） ----
fn get(b: &[u8; 88], o: usize) -> f32 {
    f32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn put(b: &mut [u8; 88], o: usize, v: f32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}
fn maxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn minss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn clamp_observed(v: f32, lo: f32) -> f32 {
    minss(1.0, maxss(lo, v))
}
fn add_observed(a: f32, b: f32) -> f32 {
    a + b
}
fn sub_observed(a: f32, b: f32) -> f32 {
    a - b
}
fn unordered_ne_zero(v: f32) -> bool {
    !v.is_nan() && v != 0.0
}
fn gable(b: &[u8; 88]) -> bool {
    (get(b, 0x30) > 0.1 || get(b, 0x30).is_nan()) && get(b, 0x34) == 1.0
}
fn limited_tip(v: Vec2) -> Vec2 {
    v.clamp_length_max(1.0)
}
fn pivot(b: &[u8; 88]) -> Vec3 {
    let tag = u32::from_le_bytes(b[8..12].try_into().unwrap()) as usize;
    Vec3::new(get(b, 0x0c + tag * 8), get(b, 0x4c), get(b, 0x10 + tag * 8))
}

/// 完整参数编辑核心（O：edit_rules.rs `apply_edit`）。
pub fn apply_edit(
    roof: &mut [u8; 88],
    event: &RoofEditEvent,
    changed_tick: &mut u32,
    current_tick: u32,
) -> EditOutcome {
    let mut mask = 0u8;
    let mut changes = EditChanges { profile: 0.0, height: 0.0, ridge: 0.0, eave: 0.0, tip_distance: 0.0 };
    let mut mark = |bit: u8| {
        mask |= bit;
        *changed_tick = current_tick;
    };
    if event.profile.mode != 0 {
        mark(1);
        let old = get(roof, 0x2c);
        let value = if event.profile.mode == 1 {
            changes.profile = sub_observed(event.profile.value, old);
            event.profile.value
        } else {
            changes.profile = event.profile.value;
            add_observed(old, event.profile.value)
        };
        put(roof, 0x2c, clamp_observed(value, 0.0));
    }
    let previous_height = get(roof, 0x30);
    if event.height.mode != 0 {
        mark(2);
        let (value, lo) = if event.height.mode == 2 {
            changes.height = event.height.value;
            (add_observed(previous_height, event.height.value), 0.1)
        } else {
            changes.height = sub_observed(event.height.value, previous_height);
            (event.height.value, 0.0)
        };
        put(roof, 0x30, clamp_observed(value, lo));
    }
    let new_height = get(roof, 0x30);
    if event.tip.mode != 0 {
        mark(16);
        let old = Vec2::new(get(roof, 0x24), get(roof, 0x28));
        let (value, delta) = if event.tip.mode == 2 {
            (limited_tip(old + event.tip.value), event.tip.value)
        } else {
            (event.tip.value, event.tip.value - limited_tip(old))
        };
        put(roof, 0x24, value.x);
        put(roof, 0x28, value.y);
        changes.tip_distance = delta.length();
    }
    if event.eave.mode != 0 {
        mark(8);
        let old = get(roof, 0x38);
        if event.eave.mode == 2 {
            put(roof, 0x38, clamp_observed(add_observed(old, event.eave.value), 0.0));
            changes.eave = event.eave.value;
        } else {
            put(roof, 0x38, event.eave.value);
            changes.eave = sub_observed(event.eave.value, clamp_observed(old, 0.0));
        }
    }
    let gable_before_ridge = gable(roof);
    if event.ridge.mode != 0 {
        mark(4);
        let old = get(roof, 0x34);
        if event.ridge.mode == 2 {
            put(roof, 0x34, clamp_observed(add_observed(old, event.ridge.value), 0.0));
            changes.ridge = event.ridge.value;
        } else {
            put(roof, 0x34, event.ridge.value);
            changes.ridge = sub_observed(event.ridge.value, clamp_observed(old, 0.0));
        }
    }
    let gable_after_ridge = gable(roof);
    let rebuild_gable = gable_before_ridge != gable_after_ridge
        || ((unordered_ne_zero(changes.profile) || event.height.mode != 0) && gable_after_ridge)
        || (unordered_ne_zero(changes.eave) && gable_after_ridge);
    let mut feedback = Vec::new();
    if gable_before_ridge && !gable_after_ridge {
        feedback.push(EditFeedback::GableLeave { pivot: pivot(roof) });
    }
    if !gable_before_ridge && gable_after_ridge {
        feedback.push(EditFeedback::GableEnter { pivot: pivot(roof) });
    }
    if [changes.profile, changes.height, changes.ridge, changes.eave]
        .iter()
        .any(|&v| unordered_ne_zero(v))
    {
        let h = changes.height.abs().ceil();
        let p = changes.profile.abs().ceil();
        let r = changes.ridge.abs().ceil();
        let e = changes.eave.abs().ceil();
        let weight = if h > 0.0 {
            changes.height.abs()
        } else {
            (changes.ridge.abs() + changes.profile.abs()) + changes.eave.abs()
        };
        feedback.push(EditFeedback::Scalar {
            gable_after: gable_after_ridge,
            ceil_height: h,
            ceil_other: e + (r + p),
            weight,
            pivot: pivot(roof),
        });
    }
    if changes.tip_distance > 0.0 {
        let tip = Vec2::new(get(roof, 0x24), get(roof, 0x28));
        feedback.push(EditFeedback::Tip {
            current_distance: tip.length(),
            change_distance: changes.tip_distance,
            pivot: pivot(roof),
        });
    }
    let height_crossing = if previous_height > 0.1 && new_height <= 0.1 {
        Some(HeightCrossing::Down)
    } else if previous_height <= 0.1 && new_height > 0.1 {
        Some(HeightCrossing::Up)
    } else {
        None
    };
    EditOutcome {
        changed_fields: mask,
        changes,
        previous_height,
        new_height,
        gable_before_ridge,
        gable_after_ridge,
        rebuild_gable,
        feedback,
        height_crossing,
        resolved_update: true,
    }
}

/// 供集成期使用：无外部依赖占位。
#[allow(dead_code)]
pub fn _gap(_: Gap) {}
