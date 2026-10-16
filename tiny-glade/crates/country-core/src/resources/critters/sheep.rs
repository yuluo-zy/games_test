//! B Sheep's recovered movement state, embedded at native Sheep +0x30.
pub mod control;

use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

/// Original externally tagged wire shape, verified by 0x1413ff520 and
/// Debug 0x1416f2800. This is embedded state, not a separate ECS component.
///
/// Native memory layout (re-analyst, 2026-10-15): discriminant byte at +0,
/// `Idle { timeout_s }` when 0, otherwise `Walking { goal_change_timeout,
/// timeout_s, force_reach_goal }`. The *serde* order (what the save wire
/// carries) is timeout_s, goal_change_timeout, force_reach_goal — the current
/// declaration matches that order; the memory order of the Walking fields is
/// `goal_change_timeout, <f32>, force_reach_goal` with the middle field name
/// 待取证.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SheepMoveState {
    Idle {
        timeout_s: f32,
    },
    Walking {
        timeout_s: f32,
        goal_change_timeout: f32,
        force_reach_goal: bool,
    },
}

/// Native `Sheep` ECS component (stride 0x68 = 104 B). Only the fields with
/// assembly evidence are declared; the rest of the native layout is 待取证
/// (no consumer function reads those offsets — see sheep-SPEC §2.1).
///
/// Recovered offsets (O, ① r6-sheep-layout 契约确认)：
/// - `+0x30` 12 B embedded [`SheepMoveState`]（判别@+0x30：0=Idle、1=Walking；
///   `+0x31` force_reach_goal bool、`+0x38` timeout_s f32；snapshot iterator
///   0x1412f03f0 steps `uVar * 0x68`）。
/// - `+0x58` f32 read by `Sheep::being_petted` 0x14089d4c0（`[+0x58] > 0.01`，
///   常量 0x142960674=0.01）。
/// movement 状态机（Idle↔Walking 转移 + picking 分支 move_state=0x101）与 X4 活羊
/// Query 待①（r6 契约 §2/§3，中优先）；现源不造。
///
/// ① T3 sheep register_required_components（procs.tsv 0x14089f340 impl$7）：原生
/// `resources::critters::sheep` 有 **5 个 ECS 组件**（Sheep/SheepAudio 0x14089ec50 /
/// UmbrellaHolder 0x14089fa30 / SheepAccessory 0x1408a0120 / FloatieHolder 0x1408a0810，
/// 组件名串 0x142c3652c/384ea/36a9f/37bb5/36fd6）。当前仅 `Sheep` 落地，其余 4 宿主组件
/// 待恢复（字段/结构 U）。
///
/// **required 集冲突登记（定案 = 空集）**：① 契约 §2 推断 `Sheep >= require(Transform,
/// CritterLocomotion)`（sheep_unstuck Query 证据，标注 I，**Query 需求 ≠ #[require]**）；但
/// crate 自有恢复 `components/registry.rs`（0x6f0/0x700 单态模板，151 个注册函数无跨组件
/// 调用）断言原版 required=空集。**③ REVIEW.md §7 独立证实**：5 个 sheep
/// `register_required_components`（0x14089ec50/0x14089f340/0x14089fa30/0x1408a0120/
/// 0x1408a0810）全为 1776B=0x6f0，与空模板 `FollowMouse`（0x140844f20=1776B）同尺寸 →
/// **required = 空集**。按此本组件**不补 #[require]**（Transform/可见性由外部
/// bundle/spawn 显式插入，O）。
///
/// 待取证: `+0x00..0x2f` / `+0x3c..0x57` / `+0x5c..0x67` — no machine-code
/// evidence (possibly animation state; GPU side `SheepAnimationSsbo` is
/// deferred). Do not invent field names or defaults for these ranges.
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sheep {
    /// Native `+0x30` (12 B embedded movement state).
    pub move_state: SheepMoveState,
    /// Native `+0x58` (being-petted timer, compared against 0x142960674 = 0.01).
    pub being_petted_timer: f32,
}

impl Sheep {
    /// `Sheep::being_petted` 0x14089d4c0:
    /// `movss xmm0,[rcx+0x58]; ucomiss xmm0,[rip+0x20c31a8]; seta al; ret`.
    ///
    /// Constant VA = 0x14089d4c0 + 5 + 7 + 0x20c31a8 = **0x142960674**, raw
    /// bytes `0a d7 23 3c` = `f32::from_bits(0x3c23d70a)` = **0.01**.
    /// (2026-10-16 correction: the earlier 0x142930f74 / `2f 29 00 00` was a
    /// rip-relative base miscalculation; 0x142930f74 bytes belong to the
    /// adjacent string region, not this function's constant.)
    /// `ucomiss` + `seta` is *strictly greater*, which (like `>`) is false for
    /// NaN operands.
    pub fn being_petted(&self) -> bool {
        self.being_petted_timer > f32::from_bits(0x3c23_d70a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn being_petted_uses_the_original_threshold_bytes() {
        // Native constant 0x142960674 = raw `0a d7 23 3c` = 0.01.
        assert_eq!(f32::from_bits(0x3c23_d70a).to_bits(), 0x3c23_d70a);
        let pet = Sheep {
            move_state: SheepMoveState::Idle { timeout_s: 0.0 },
            being_petted_timer: 0.011,
        };
        assert!(pet.being_petted());
        let just_under = Sheep {
            move_state: SheepMoveState::Idle { timeout_s: 0.0 },
            being_petted_timer: 0.009,
        };
        assert!(!just_under.being_petted());
        let idle = Sheep {
            move_state: SheepMoveState::Idle { timeout_s: 0.0 },
            being_petted_timer: 0.0,
        };
        assert!(!idle.being_petted());
        // NaN is unordered: ucomiss+seta yields false, matching `>`.
        let nan = Sheep {
            move_state: SheepMoveState::Idle { timeout_s: 0.0 },
            being_petted_timer: f32::NAN,
        };
        assert!(!nan.being_petted());
    }

    #[test]
    fn wire_shape_roundtrips_through_serde() {
        let sheep = Sheep {
            move_state: SheepMoveState::Walking {
                timeout_s: 4.0,
                goal_change_timeout: 5.0,
                force_reach_goal: true,
            },
            being_petted_timer: 2.5,
        };
        let json = serde_json::to_string(&sheep).unwrap();
        let back: Sheep = serde_json::from_str(&json).unwrap();
        assert_eq!(back, sheep);
        assert_eq!(
            json,
            r#"{"move_state":{"Walking":{"timeout_s":4.0,"goal_change_timeout":5.0,"force_reach_goal":true}},"being_petted_timer":2.5}"#
        );
    }
}
