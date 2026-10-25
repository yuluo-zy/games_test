//! 物种组件族（B 布局证据，见各字段注释）。
//!
//! 布局来源汇总（critters-behavior-analysis.md §5 表 + critters-extra-analysis.md）：
//! - CritterGoal 0x10：active u32@+0、pos Vec3@+4..+0xc（country-core
//!   `resources::critters::CritterGoal` 为 16B Option<Vec3>，本 crate 直接复用）。
//! - CritterVelocityTarget 0x10：dir Vec3@+0..+8、retention_per_second f32@+0xc。
//! - Velocity 0xc：x@+0,y@+4,z@+8（各系统一致）。
//! - CritterVelocityMultiplier 4B：f32@+0。
//! - CritterLocomotion 8B：枚举 u32@+0（值 2/3/4/6/7；掩码 `99u >> (v&31) & 1` 选 {0,1,5,6}；
//!   ==2 采样岩石掩码、==4 跳过水面双采样、==3 跳过 PublicWalls::get）。
//! - Duck 0x100：轨迹 Vec<Vec3> @+0x98、点数 @+0xa0、目标活动字节 @+0xe0、次标志 @+0xe1、
//!   8B 双计时 @+0xe4、进度 @+0xec、冻结字节 @+0xfc、渲染尾字段 @+0xf0/+0xf4/+0xf8。
//! - DuckState 1B 枚举：1=沿轨迹正向、2=自由游（set_duck_velocities 只处理）、3=沿轨迹反向。
//! - Sheep 0x68：行为状态 u32@+8（==1 跟随）、目标计时 @+0xc、宠物动画 phase@+0x18 /
//!   progress@+0x1c / target@+0x20、移动状态 @+0x30（含force @+0x31和计时 @+0x34/+0x38）、幅度 @+0x58
//!   （being_petted = >0.01）、控制字节 @+0x60（sheep_pet 行证据 0x14293a350）。
//! - Butterfly 0x18：state u32@+0、dwell @+0xc、phase @+0x10、phase_ref @+0x14
//!   （phase_ref 是 GPU 侧 `anim_t`，写 ButterflySsbo@+0xc）。
//! - Firefly：goal_timer f32（record 语义；出生记录另写 +0x70=1.0，U）。
//! - Boid 0x18：pos Vec3 + vel Vec3；BoidFlock 另有 0x38 槽记录（随机转向计时，U 精确布局）。
//! - BirdSpots 0x18 记录：停靠点（move_critters 更新位置；wren 消费）。
//! - Fish 记录 stride 0x30：转向前提 byte==2。
//! - DuckSpawnLocations：10×10=100 候选点 x∈[-52,52]、z 十档（0x140f8ee90）；
//!   资源尾 +0x18=25.0（水面检测半径，语义 I）。
//! - FishSpawnLocations：25×25=625 候选点 ∈[-52,52]²（0x140f909f0）。

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use glam::{Quat, Vec3};

pub use country_core::resources::critters::{Critter, CritterGoal};
pub use country_core::utils::components::transform::Transform;

/// 轨迹状态（Duck+0xe0/+0xe1/+0xe4 段；与 reconstruction duck.rs DuckGoalState 一致）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DuckTrajectoryState {
    pub moving: bool,
    pub aux: u8,
    pub dwell: [f32; 2],
}

/// `Velocity`（0xc）：x@+0、y@+4、z@+8（O）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Velocity(pub Vec3);

/// `CritterVelocityTarget`（0x10）：dir Vec3@+0、retention_per_second f32@+0xc（O）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct CritterVelocityTarget {
    pub direction: Vec3,
    pub retention_per_second: f32,
}

/// `CritterVelocityMultiplier`（4B f32@+0；各系统写查询环境值，语义 unknown）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct CritterVelocityMultiplier(pub f32);

/// Locomotion's observed scalar discriminant and collision predicates.
/// Complete original enum payload/ABI remains unresolved.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CritterLocomotion(pub u32);

impl CritterLocomotion {
    pub fn samples_rock_mask(self) -> bool {
        self.0 == 2
    }
    pub fn skips_water_sample(self) -> bool {
        self.0 == 4
    }
    pub fn skips_public_walls(self) -> bool {
        self.0 == 3
    }
    pub fn wall_hash_only(self) -> bool {
        (99u32 >> (self.0 & 31)) & 1 == 1
    }
}

/// `Duck`（stride 0x100）。字段证据见模块文档；`audio` 为 FMOD 事件实例（event:/SFX/Ducks/…）
/// 的宿主缝（见 audio 模块），非机器码布局字段。
#[derive(Component, Debug)]
pub struct Duck {
    /// +0x98：轨迹点（cap-first 原生 Vec 的语义等价；stride 0xc）。
    pub trajectory: Vec<Vec3>,
    /// +0xe0：目标活动标志字节（update_duck_goals 读写）。
    pub moving: bool,
    /// +0xe1：次标志字节（每帧归零，语义 unknown）。
    pub aux: u8,
    /// +0xe4：8B 双 f32 目标计时（区间 [1,4] / [0.5,1]）。
    pub dwell: [f32; 2],
    /// +0xec：沿轨迹进度（move_ducks 写）。
    pub progress: f32,
    /// +0xfc：冻结字节（==1 → set_duck_velocities 清 VelocityTarget）。
    pub frozen: bool,
    /// +0xf0/+0xf4/+0xf8：渲染尾字段（render_loop draw_shaders 读；语义 U）。
    pub f_f0: f32,
    pub f_f4: f32,
    pub f_f8: f32,
    /// 音频（Quack/TakeOff/Land 三事件；宿主缝）。
    pub audio: crate::audio::DuckAudio,
}

/// `DuckState`（1B 枚举）。fresh 反汇编（move_ducks 0x142312200、plan_duck 0x14230ff00）：
/// 0=出生初态（沿轨迹正向，t>0.25 后升为 1）、1=沿轨迹正向巡航、2=自由游、3=沿轨迹反向。
/// move_ducks 的 `lVar25 = state>1 ? state-1 : 0` 映射：0/1→正向、2→跳过、3→反向。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DuckState {
    /// 0：plan 出生态（plan_duck spawn bundle 写 0；move_ducks t>0.25 时 0→1）。
    #[default]
    Initial = 0,
    TrajectoryForward = 1,
    FreeSwim = 2,
    TrajectoryReverse = 3,
}

impl DuckState {
    /// 原生字节（O：判别 0/1/2/3）。
    pub fn from_byte(b: u8) -> Self {
        match b {
            1 => DuckState::TrajectoryForward,
            2 => DuckState::FreeSwim,
            3 => DuckState::TrajectoryReverse,
            _ => DuckState::Initial,
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            DuckState::Initial => 0,
            DuckState::TrajectoryForward => 1,
            DuckState::TrajectoryReverse => 3,
            DuckState::FreeSwim => 2,
        }
    }
}

/// Shared ECS identity with core snapshot/save/freeze consumers.
pub use country_core::resources::critters::sheep::{Sheep, SheepMoveState};

/// 羊抚摸控制（pet 动画消费侧；control_flag 的宿主投影）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepPetControl {
    pub allow_walk_animation: bool,
    pub amplitude: f32,
}

// Shared by the camera and species systems; keep a single component identity.
pub use country_core::resources::critters::Butterfly;

/// `Firefly`（goal_timer f32；其余出生记录字段 U）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Firefly {
    pub goal_timer: f32,
}

/// `Wren`（鹪鹩状态；`nani_update_wren` 读 prev/current 变换 + 参数，布局 U——
/// 只落状态机消费的语义字段）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Wren {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub phase: f32,
    pub hop_timer: f32,
}

/// 鹪鹩停靠点记录（stride 0x18；move_critters 更新位置，wren 系统读写）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct BirdSpots {
    pub spots: Vec<BirdSpot>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BirdSpot {
    pub pos: Vec3,
    /// 占用标志（u32@+0xc 段，语义 I）。
    pub occupied: bool,
}

/// `DecoratorBirdSpots`（wren.rs PDB 命名实体；装饰物上的鸟点，语义同 BirdSpots）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct DecoratorBirdSpots {
    pub spots: Vec<BirdSpot>,
}

/// `Fish` 记录（stride 0x30；转向逻辑前提 byte==2）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Fish {
    /// 状态字节（==2 时向 2D 距离 <2.0 的目标 lerp 转向）。
    pub state: u8,
}

/// 鱼群（`FishBoids`，fish.rs PDB 命名实体）。B 侧是 **Resource**（`insert_resource<
/// FishBoids>` 符号实证 0x140f84e80；`despawn_fish(ResMut<FishBoids>,
/// ResMut<FishSpawnLocations>)` 在 `OnExit<AppLoadingState>` 重置资源）。crate 以
/// 每鱼实体上的 0x18 槽投影（pos+vel；`turn_timer` = 0x38 槽计时）承载，并同时
/// derive `Resource` 以便 `despawn_fish` 重置资源侧表示。
#[derive(Component, Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FishBoids {
    pub pos: Vec3,
    pub vel: Vec3,
    pub turn_timer: f32,
}

/// 小动物总开关/强度资源。
/// - firefly 门：`*res+4 == 1` 才跑；
/// - **wren 门**：`*res+5`（fresh sim_wren_boids 0x142326a80 `TEST byte[*in_R8+5]`）
///   非 0 才模拟，否则清空 WrenBoids 并 despawn 全部鹪鹩；
/// - render_loop 的 `CritterControl.intensity` 是萤火虫强度（f32@+0）。
#[derive(Resource, Clone, Copy, Debug)]
pub struct CritterControlState {
    /// +4 字节：control 门（==1 运行）。
    pub enabled: bool,
    /// +5 字节：wren/bird 模拟门（!=0 运行）。
    pub birds_enabled: bool,
    /// 萤火虫强度（f32@+0；draw_shaders 消费）。
    pub intensity: f32,
}

impl Default for CritterControlState {
    fn default() -> Self {
        Self {
            enabled: false,
            birds_enabled: false,
            intensity: 0.0,
        }
    }
}

/// `DuckSpawnLocations`（0x140f8ee90）：10×10=100 候选点；`timer` 资源尾字段
/// （spawn_ducks 累加 dt，>=30 才生；+0x18=25.0 常数语义 I）。
#[derive(Resource, Clone, Debug)]
pub struct DuckSpawnLocations {
    pub candidates: Vec<Vec3>,
    pub used: Vec<bool>,
    pub timer: f32,
}

impl Default for DuckSpawnLocations {
    fn default() -> Self {
        Self::new()
    }
}

impl DuckSpawnLocations {
    /// `DuckSpawnLocations::new`（O 0x140f8ee90，fresh decomp evidence
    /// `work/purelogic-integration/evidence/0x140f8ee90_new.c`）：
    /// 10×10=100 候选点，元素 12B = `(Vec2{x@0,z@4}, bool used@8)`（stride 0xc）；
    /// 行值（外层 i=0..9）= `(i/9)*52 - (1-i/9)*52`（逐 f32：-52,-40.444443,
    /// -28.888891,-17.333334,-5.7777786,5.7777786,17.333334,28.888891,40.444443,52），
    /// 列值（内层 j）= 同十档常量（0xc2500000/-52.0、0xc221c71c/-40.444443、
    /// 0xc1e71c73/-28.888891、0xc18aaaa9/-17.33333、0xc0b8e390/-5.7777786、
    /// 0x40b8e390/5.7777786、0x418aaaac/17.333335、0x41e71c73/28.888891、
    /// 0x4221c71c/40.444443、0x42500000/52.0）——与行公式逐位一致；
    /// 资源尾 timer f32@+0x18 = **25.0**（0x41c80000，初始非 0）。
    ///
    /// 旧实现用硬编码近似 x_steps（-40.22/-28.56 等错误 lane）且 timer=0，
    /// 已改为公式驱动（restorer-purelogic-integration 2026-10-17）。
    pub fn new() -> Self {
        let steps = duck_spawn_steps();
        let mut candidates = Vec::with_capacity(100);
        // FIX-P2：原生外层 i 值→+0→世界 x，内层 j 值→+4→世界 z（原 `Vec3::new(col,0,row)`
        // 行列转置，已改为 `Vec3::new(row,0,col)`）。
        for &row in &steps {
            for &col in &steps {
                candidates.push(Vec3::new(row, 0.0, col));
            }
        }
        Self {
            candidates,
            used: vec![false; 100],
            // O 0x140f8ee90 尾部 `*(undefined4 *)(in_RCX + 3) = 0x41c80000` → 25.0。
            timer: 25.0,
        }
    }
}

/// 鸭子出生网格十档值（O：0x140f8ee90 行公式 `(i/9)*52-(1-i/9)*52`，逐 i=0..9；
/// 列常量与公式逐位一致，见 `DuckSpawnLocations::new`）。纯函数，供网格构建复用。
pub fn duck_spawn_steps() -> [f32; 10] {
    let mut steps = [0.0f32; 10];
    for (i, s) in steps.iter_mut().enumerate() {
        let t = i as f32 / 9.0;
        *s = t * 52.0 - (1.0 - t) * 52.0;
    }
    steps
}

/// `FishSpawnLocations`（O 0x140f909f0）：25×25=625 点 ∈[-52,52]²。
#[derive(Resource, Clone, Debug)]
pub struct FishSpawnLocations {
    pub candidates: Vec<Vec3>,
}

impl Default for FishSpawnLocations {
    fn default() -> Self {
        Self::new()
    }
}

impl FishSpawnLocations {
    /// `FishSpawnLocations::new`（0x140f909f0）：外层 i=0..24、内层 j=0..24 共 625；
    /// 原生元素 stride 0xc：+0=外层值→世界 x、+4=内层值→世界 z、+8=used(0)。
    /// 行/列公式均为 `(k/24)*52 - (1-k/24)*52`（DIVSS 24、SUBSS 1、MULSS 52×2、SUBSS）。
    ///
    /// FIX-P1：① 行列转置——x=外层(i)、z=内层(j)（原 `Vec3::new(fx*52,0,fy*52)` 为
    /// x=内层/z=外层）；② 公式改逐位等价 `t*52-(1-t)*52`（原 `(t*2-1)*52` 在 7/25 档差 1 ULP）。
    pub fn new() -> Self {
        let mut candidates = Vec::with_capacity(625);
        for i in 0..25 {
            let ti = i as f32 / 24.0;
            let x = ti * 52.0 - (1.0 - ti) * 52.0;
            for j in 0..25 {
                let tj = j as f32 / 24.0;
                let z = tj * 52.0 - (1.0 - tj) * 52.0;
                candidates.push(Vec3::new(x, 0.0, z));
            }
        }
        Self { candidates }
    }
}

/// `PrevTransform`（鸭 shader 消费的上一帧变换；布局 = Transform{旋转@0,平移@0x10,缩放@0x1c}）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrevTransform {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
}

impl Default for PrevTransform {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}

impl Default for Duck {
    fn default() -> Self {
        Self {
            trajectory: Vec::new(),
            moving: false,
            aux: 0,
            dwell: [1.0, 1.0],
            progress: 0.0,
            frozen: false,
            f_f0: 0.0,
            f_f4: 0.0,
            f_f8: 0.0,
            audio: crate::audio::DuckAudio::default(),
        }
    }
}

#[cfg(test)]
mod locomotion_tests {
    use super::*;
    #[test]
    fn original_locomotion_mask_selects_observed_values() {
        let selected: Vec<u32> = (0..32)
            .filter(|&value| CritterLocomotion(value).wall_hash_only())
            .collect();
        assert_eq!(selected, [0, 1, 5, 6]);
    }
    #[test]
    fn original_other_branch_predicates() {
        assert!(CritterLocomotion(2).samples_rock_mask());
        assert!(CritterLocomotion(4).skips_water_sample());
        assert!(CritterLocomotion(3).skips_public_walls());
        assert!(!CritterLocomotion(7).wall_hash_only());
    }
}
