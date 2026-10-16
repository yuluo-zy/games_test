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
//!   progress@+0x1c / target@+0x20、标志字节 @+0x30、8B 双计时 @+0x34、幅度 @+0x58
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

pub use country_core::resources::critters::CritterGoal;
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

/// `CritterLocomotion`（8B 槽，u32@+0；值 2/3/4/6/7 与位测，见模块文档）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CritterLocomotion(pub u32);

impl CritterLocomotion {
    /// 掩码 `99u >> (v & 31) & 1` 选中 {0,1,5,6}：纯 WallSpatialHash 墙体避让（无 spherecast）。
    pub fn wall_hash_only(self) -> bool {
        (99u32 >> (self.0 & 31)) & 1 == 1
    }
    /// ==2：采样 terrain_rock_mask。
    pub fn samples_rock_mask(self) -> bool {
        self.0 == 2
    }
    /// ==4：跳过水面双采样。
    pub fn skips_water_sample(self) -> bool {
        self.0 == 4
    }
    /// ==3：跳过 PublicWalls::get。
    pub fn skips_public_walls(self) -> bool {
        self.0 == 3
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

/// `DuckState`（1B 枚举）。B 反汇编确认：1=沿轨迹正向、2=自由游、3=沿轨迹反向（critters-extra
/// §6）；0 语义推断（不伪造）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DuckState {
    #[default]
    FreeSwim = 2,
    TrajectoryForward = 1,
    TrajectoryReverse = 3,
}

impl DuckState {
    /// 原生字节（O：判别 1/2/3）。
    pub fn from_byte(b: u8) -> Self {
        match b {
            1 => DuckState::TrajectoryForward,
            3 => DuckState::TrajectoryReverse,
            _ => DuckState::FreeSwim,
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            DuckState::TrajectoryForward => 1,
            DuckState::TrajectoryReverse => 3,
            DuckState::FreeSwim => 2,
        }
    }
}

/// `Sheep`（stride 0x68，字段证据见模块文档）。
#[derive(Component, Clone, Copy, Debug)]
pub struct Sheep {
    /// +8：行为状态 u32（==1 跟随相机/光标）。
    pub behavior_state: u32,
    /// +0xc：目标计时。
    pub goal_timer: f32,
    /// +0x18/+0x1c/+0x20：宠物动画 phase/progress/target（sheep_pet 行证据）。
    pub pet_phase: u32,
    pub pet_progress: f32,
    pub pet_target: u64,
    /// +0x30：标志字节（共享目标刷新开关）。
    pub flag_30: u8,
    /// +0x34：8B 双 f32 共享目标计时。
    pub shared_timers: [f32; 2],
    /// +0x58：幅度（being_petted = > 0.01；`Sheep::being_petted` 0x14089d4c0）。
    pub amplitude: f32,
    /// +0x60：控制字节。
    pub control_flag: u8,
}

impl Sheep {
    /// `Sheep::being_petted`（O 0x14089d4c0）：`f32@+0x58 > 0.01`。
    pub fn being_petted(&self) -> bool {
        self.amplitude > f32::from_bits(0x3c23d70a)
    }
}

/// 羊抚摸控制（pet 动画消费侧；control_flag 的宿主投影）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepPetControl {
    pub allow_walk_animation: bool,
    pub amplitude: f32,
}

/// `Butterfly`（stride 0x18；state@+0、目标计时@+8（goals `timer@+2` 段）、dwell@+0xc、
/// phase@+0x10、phase_ref@+0x14；phase_ref 是 GPU `anim_t`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Butterfly {
    pub state: u32,
    /// +8：目标计时（update_butterfly_goals 读写）。
    pub goal_timer: f32,
    /// +0xc：状态机 dwell 计时。
    pub dwell: f32,
    /// +0x10：相位。
    pub phase: f32,
    /// +0x14：相位参考（= GPU anim_t）。
    pub phase_ref: f32,
}

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

/// 鱼群（`FishBoids`，fish.rs PDB 命名实体）：CPU 侧 boid 记录
/// （0x18：pos+vel；`turn_timer` 为 0x38 槽随机转向计时的实体侧投影）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct FishBoids {
    pub pos: Vec3,
    pub vel: Vec3,
    pub turn_timer: f32,
}

/// 小动物总开关/强度资源（update_firefly_goals 的 control 门：`*res+4 == 1` 才跑；
/// render_loop 的 `CritterControl.intensity` 是萤火虫强度）。
#[derive(Resource, Clone, Copy, Debug)]
pub struct CritterControlState {
    /// +4 字节：control 门（==1 运行）。
    pub enabled: bool,
    /// 萤火虫强度（f32@+0；draw_shaders 消费）。
    pub intensity: f32,
}

impl Default for CritterControlState {
    fn default() -> Self {
        Self {
            enabled: false,
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
    /// `DuckSpawnLocations::new`（O 0x140f8ee90）：z 十档
    /// -52,-40.22,-28.56,-17.33,-5.78,5.78,17.33,28.56,40.22,52；x 同十档。
    pub fn new() -> Self {
        let x_steps: [f32; 10] = [
            -52.0, -40.22, -28.56, -17.33, -5.78, 5.78, 17.33, 28.56, 40.22, 52.0,
        ];
        let mut candidates = Vec::with_capacity(100);
        for &zx in &x_steps {
            for &xz in &x_steps {
                candidates.push(Vec3::new(zx, 0.0, xz));
            }
        }
        Self {
            candidates,
            used: vec![false; 100],
            timer: 0.0,
        }
    }
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
    pub fn new() -> Self {
        let mut candidates = Vec::with_capacity(625);
        for iy in 0..25 {
            let fy = (iy as f32 / 24.0) * 2.0 - 1.0;
            for ix in 0..25 {
                let fx = (ix as f32 / 24.0) * 2.0 - 1.0;
                candidates.push(Vec3::new(fx * 52.0, 0.0, fy * 52.0));
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
