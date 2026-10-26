//! `resources::onboarding` — 教学/提示系统通知事件族（B `system_onboarding` 消费）。
//!
//! 事件清单与名称取自 `add_onboarding` 0x140650d50 的 `register_event` 序列
//! （restorer-interaction-onboarding 2026-10-17 fresh 反编译，evidence
//! `reports/interaction/onboarding/evidence/results/batch3-core.txt`）：
//! 本模块承载 16 个 `NotifyHintSystem*` 通知 + `TutorialOp` 教学操作事件；
//! `DisplayHintRequest` / `LingerHintRequest` / `TutorialHintComplete` 属于
//! `system_onboarding::hint_orchestrator` / `tutorial_completion` 域（上层模块），
//! 不在本文件；`DisplayRollercoasterPrompt` 原模块路径为
//! `country_core::resources::events`（见 events.rs）。
//!
//! CameraMoved has two u32 payload words. Terrain and TerrainSteepness carry
//! one observed native byte; all other current notifications remain unit until
//! their own producer/consumer evidence requires payload. The old event+1 bool
//! claim for Terrain confused a hint-condition field with event data.

/// Original welcome owner, initialized to state0 by MenuFlowPlugin.
/// Its elapsed time belongs to the showing state; closing does not open Esc.
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub enum WelcomeScreenState {
    #[default]
    Pending,
    Showing { elapsed: f32 },
    Closed,
}

impl WelcomeScreenState {
    /// Native is_closed1409a9140 compares the state tag with2.
    pub fn is_closed(&self) -> bool { matches!(self, Self::Closed) }
}

/// 墙面/屋顶形状编辑已执行通知（O：add_onboarding register_event 顺序 1）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemShapeEditWasPerformed;

/// 相机移动通知（0x10B EventInstance：`{tag u32@+8, value u32@+0xc}`，O）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NotifyHintSystemCameraMoved {
    /// 相机动作 tag（O：1 = zoom、2 = rotate、其它 = pan）。
    pub tag: u32,
    /// 附加量（O：tag 1 时 = 滚轮增量 `piVar13[1]`）。
    pub value: u32,
}

/// 吸附通知（O：register_event 顺序 4）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemSnapping(pub bool);

/// Terrain notification: original event first byte0=draw committed,1=hovered terrain,2=focus entered.
/// No event+1 bool is observed; Default0 is only host construction convenience.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemTerrain {
    pub tag: u8,
}

/// 窗通知（O：register_event 顺序 9）。事件实例 `{EventId 8B, flag u8 @+8}`：
/// `hint_unlink_windows` 0x14227d760 逐事件读该字节，`!=0` → 完成 id 8，`==0` → 展示提示。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemWindow {
    pub flag: u8,
}

/// 工具刚使用通知（O：register_event 顺序 10；既有）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemToolWasJustUsed;

/// 某物被擦除通知（O：register_event 顺序 11）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemSomethingWasErased;

/// Native first byte0 on edit release;1 after horizontal drag accumulation>0.3.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemTerrainSteepness {
    pub tag: u8,
}

/// 屋顶形状通知（O：register_event 顺序 13）。事件实例 `{EventId 8B, flag u8 @+8}`：
/// `hint_roof_shape` 0x142284ed0 逐事件读该字节，`==0` → `hint_done=1`；`!=0` →
/// `event_byte=1` 并完成 id 7。生产者在屋顶编辑系统（`ui_edit_roof`，不属 M01）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemRoofShape {
    pub flag: u8,
}

/// 杂乱移动通知（O：register_event 顺序 14）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemClutterWasMoved;

/// 楼梯放置通知（O：register_event 顺序 15）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemStairsPlaced;

/// 悬停楼梯通知（O：register_event 顺序 16）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemHoveringOverStairs;

/// 开始续建楼梯通知（O：register_event 顺序 17）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemStartedStairsContinue;

/// 可续建楼梯通知（O：register_event 顺序 18）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemCanContinueStairs;

/// 楼梯脱离墙通知（O：register_event 顺序 19）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemStairsDetachedFromWall;

/// 楼梯可从墙脱离通知（O：register_event 顺序 20）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NotifyHintSystemStairsCanDetachFromWall;

/// 原设置页140247173发0/1；reset_hints的PDB参数明确消费此事件。
/// 完成通知的0..17编号属于TutorialHintComplete，不能复用为该命令判别。
#[derive(bevy_ecs::event::Event,Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
pub enum TutorialOp {DisableHints=0,ResetHints=1}

/// 完成通知编号的内部映射（不是TutorialOp事件）；
/// `tutorial_completion` 0x14227c230 消费：载荷 u8 id @+8，0..17 对应 GameProfile
/// 完成标志偏移 0x1f4..0x205 的 18 路 switch）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TutorialCompletionId {
    /// id 0 → GameProfile+0x1f4（ron 序列化的 `tutorial` 字段）。
    #[default]
    Tutorial = 0,
    /// id 1 → +0x1f5（focused_edit 完成）。
    FocusedEdit = 1,
    /// id 2 → +0x1fe（undo 完成）。
    Undo = 2,
    /// id 3 → +0x1fc（disable_snapping 完成）。
    DisableSnapping = 3,
    /// id 4 → +0x1fa（terrain_steepness 完成）。
    TerrainSteepness = 4,
    /// id 5 → +0x1fb（terrain_focus 完成）。
    TerrainFocus = 5,
    /// id 6 → +0x1f9（terrain_layering 完成）。
    TerrainLayering = 6,
    /// id 7 → +0x1ff（roof_shape 完成）。
    RoofShape = 7,
    /// id 8 → +0x1fd（unlink_windows 完成）。
    UnlinkWindows = 8,
    /// id 9 → +0x200（clutter_focused_edit 完成）。
    ClutterFocusedEdit = 9,
    /// id 10 → +0x1f6（相机提示完成标志之一）。
    CameraPan = 10,
    /// id 11 → +0x1f7（相机提示完成标志之一）。
    CameraRotate = 11,
    /// id 12 → +0x1f8（相机提示完成标志之一）。
    CameraZoom = 12,
    /// id 13 → +0x201（楼梯提示阶段 1）。
    StairsStage1 = 13,
    /// id 14 → +0x202（楼梯提示阶段 2）。
    StairsStage2 = 14,
    /// id 15 → +0x203（楼梯提示阶段 3）。
    StairsStage3 = 15,
    /// id 16 → +0x204（楼梯提示阶段 4）。
    StairsStage4 = 16,
    /// id 17 → +0x205（楼梯提示阶段 5）。
    StairsStage5 = 17,
}

impl TutorialCompletionId {
    /// 反序列化 id（0..17）。越界返回 `None`（原二进制 switch 无 default 分支即
    /// 回绕，见 tutorial_completion 注释）。
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Tutorial),
            1 => Some(Self::FocusedEdit),
            2 => Some(Self::Undo),
            3 => Some(Self::DisableSnapping),
            4 => Some(Self::TerrainSteepness),
            5 => Some(Self::TerrainFocus),
            6 => Some(Self::TerrainLayering),
            7 => Some(Self::RoofShape),
            8 => Some(Self::UnlinkWindows),
            9 => Some(Self::ClutterFocusedEdit),
            10 => Some(Self::CameraPan),
            11 => Some(Self::CameraRotate),
            12 => Some(Self::CameraZoom),
            13 => Some(Self::StairsStage1),
            14 => Some(Self::StairsStage2),
            15 => Some(Self::StairsStage3),
            16 => Some(Self::StairsStage4),
            17 => Some(Self::StairsStage5),
            _ => None,
        }
    }

    /// 完成标志在 GameProfile 内的字节偏移（O：tutorial_completion 0x14227c230
    /// 18 路 switch 的写面）。**非连续**：id → 偏移按 case 表（0→0x1f4、1→0x1f5、
    /// 2→0x1fe、3→0x1fc、4→0x1fa、5→0x1fb、6→0x1f9、7→0x1ff、8→0x1fd、
    /// 9→0x200、10→0x1f6、11→0x1f7、12→0x1f8、13..17→0x201..0x205）。
    pub fn completion_flag_offset(self) -> usize {
        match self {
            Self::Tutorial => 0x1f4,
            Self::FocusedEdit => 0x1f5,
            Self::Undo => 0x1fe,
            Self::DisableSnapping => 0x1fc,
            Self::TerrainSteepness => 0x1fa,
            Self::TerrainFocus => 0x1fb,
            Self::TerrainLayering => 0x1f9,
            Self::RoofShape => 0x1ff,
            Self::UnlinkWindows => 0x1fd,
            Self::ClutterFocusedEdit => 0x200,
            Self::CameraPan => 0x1f6,
            Self::CameraRotate => 0x1f7,
            Self::CameraZoom => 0x1f8,
            Self::StairsStage1 => 0x201,
            Self::StairsStage2 => 0x202,
            Self::StairsStage3 => 0x203,
            Self::StairsStage4 => 0x204,
            Self::StairsStage5 => 0x205,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tutorial_op_ids_map_to_the_completion_flag_offsets() {
        let table = [
            (TutorialCompletionId::Tutorial, 0, 0x1f4),
            (TutorialCompletionId::FocusedEdit, 1, 0x1f5),
            (TutorialCompletionId::Undo, 2, 0x1fe),
            (TutorialCompletionId::DisableSnapping, 3, 0x1fc),
            (TutorialCompletionId::TerrainSteepness, 4, 0x1fa),
            (TutorialCompletionId::TerrainFocus, 5, 0x1fb),
            (TutorialCompletionId::TerrainLayering, 6, 0x1f9),
            (TutorialCompletionId::RoofShape, 7, 0x1ff),
            (TutorialCompletionId::UnlinkWindows, 8, 0x1fd),
            (TutorialCompletionId::ClutterFocusedEdit, 9, 0x200),
            (TutorialCompletionId::CameraPan, 10, 0x1f6),
            (TutorialCompletionId::CameraRotate, 11, 0x1f7),
            (TutorialCompletionId::CameraZoom, 12, 0x1f8),
            (TutorialCompletionId::StairsStage1, 13, 0x201),
            (TutorialCompletionId::StairsStage2, 14, 0x202),
            (TutorialCompletionId::StairsStage3, 15, 0x203),
            (TutorialCompletionId::StairsStage4, 16, 0x204),
            (TutorialCompletionId::StairsStage5, 17, 0x205),
        ];
        for (op, id, offset) in table {
            assert_eq!(op as u8, id, "{op:?}");
            assert_eq!(op.completion_flag_offset(), offset, "{op:?}");
            assert_eq!(TutorialCompletionId::from_id(id), Some(op));
        }
        assert_eq!(TutorialCompletionId::from_id(18), None);
        assert_eq!(TutorialCompletionId::from_id(255), None);
    }
}
