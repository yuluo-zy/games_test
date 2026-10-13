//! `resources::critters` — 小动物 CPU 状态（B 骨架 `resources/critters.rs` + `resources/critters/{sheep.rs,sheep/control.rs}`）。
//!
//! 反编译（Ghidra）：
//!   - `CritterControl` 控制开关：`update_firefly_goals` 0x142316940 读 `*res+4 == 1` 门控（O）。
//!   - `BirdSpotManager::gen_id` 0x140f86330：`+8 += 1`（u32 计数）。
//!   - `BirdSpotManager::create_spots` 0x140f86340：池（BirdSpotRecycler）复用/分配。
//!   - `Duck::new` 0x140f85bb0：三 FMOD 事件 `event:/SFX/Ducks/{Quack,TakeOff,Land}`。
//!   - `ButterflyAudio::set_position` 0x140f85ba0：kind==1 时 `set_3d_attributes`。
//! 组件字段（除已反汇编者）→ typed gap，不臆造。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use glam::Vec3;
use perchance::PerchanceContext;

use crate::typed_gap::Gap;

pub mod sheep;

/// 小动物全局控制资源（O 名；`impl$0::default` 行 32、`impl$6::drop` 行 293）。
/// `+4` 为控制开关字节（update_firefly_goals 门控，O）。
#[derive(Resource)]
pub struct CritterControl {
    /// 控制开关（`+4`，O）。`false` 时小动物目标更新跳过。
    pub control_enabled: bool,
    /// `impl$6::drop` 清理的 Arc+Vec 等其它字段（布局 U）。
    pub _gap: Gap,
}

impl Default for CritterControl {
    fn default() -> Self {
        // `impl$0::default`（0x140f85b90）为空实现（O）。
        CritterControl {
            control_enabled: false,
            _gap: Gap,
        }
    }
}

/// 小动物通用组件（O 名；字段 U）。
#[derive(Component, Default)]
pub struct Critter(pub Gap);

/// 目标点组件（0x10 槽，reva/duck 恢复：`active u32 @+0`，`pos Vec3 @+4`）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CritterGoal {
    /// 目标活动标志（bit0 参与位测）。
    pub active: bool,
    /// 目标位置。
    pub pos: Vec3,
}

impl Default for CritterGoal {
    fn default() -> Self {
        CritterGoal {
            active: false,
            pos: Vec3::ZERO,
        }
    }
}

/// 步态组件（O 名；反汇编出现值 2/3/4/6/7 + 位测 `99 >> (v&0x1f) & 1`，枚举语义 U）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CritterLocomotion(pub u32);

/// 小动物 RNG 资源（O 名；系统经 `impl$15/impl$23::clone_behavior` 注入 perchance 上下文）。
/// 原实现用 `perchance::PerchanceContext`（0.4.0，`uniform_f32`/`get_u128`）。
#[derive(Resource, Clone)]
pub struct CritterRng(pub PerchanceContext);

/// 速度目标组件（0x10 槽：`dir Vec3 @+0..+8`，`retention_per_second f32 @+0xc`；O/I）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub struct CritterVelocityTarget {
    pub direction: Vec3,
    pub retention_per_second: f32,
}

/// 速度乘数组件（f32 @+0；查询闭包环境写 u32，语义 U）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub struct CritterVelocityMultiplier(pub f32);

/// 速度组件（12B = Vec3；move_critters 查询硬证据）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity(pub Vec3);

/// 萤火虫组件（O；GPU 发送上限 0x60=96，panic 断言）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub struct Firefly {
    /// 目标刷新倒数计时（firefly 核心）。
    pub goal_timer: f32,
    /// 其余字段 U。
    pub _gap: Gap,
}

/// 蝴蝶组件（O；GPU 发送上限 0x18=24，panic 断言）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Default)]
pub struct Butterfly(pub Gap);

/// 蝴蝶音频组件（O；`set_position` 行 144，kind==1 时 FMOD `set_3d_attributes`）。
#[derive(Component, Clone, Debug, Default)]
pub struct ButterflyAudio {
    /// `+0` kind/标志（`set_position` 判 ==1，O）。
    pub kind: u32,
    /// FMOD 事件实例（音频句柄，布局 U）。
    pub _gap: Gap,
}

impl ButterflyAudio {
    /// `ButterflyAudio::set_position`（行 144，O）：kind==1 时把位置同步到 FMOD 事件。
    pub fn set_position(&mut self, _position: Vec3) {
        if self.kind == 1 {
            // libfmod::EventInstance::set_3d_attributes（O callee）；内部依赖不还原。
        }
    }
}

/// 鸭子状态枚举（O 名；set_duck_velocities 只处理 ==2，2/3 语义推断为游水/水面，U）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DuckState(pub u8);

/// 鸭子组件（O；`Duck::new` 行 208 建三 FMOD 事件）。
#[derive(Component, Clone, Debug, Default)]
pub struct Duck {
    /// FMOD quack 事件（`event:/SFX/Ducks/Quack`，O 字符串）。
    pub quack: Gap,
    /// FMOD take-off 事件（`event:/SFX/Ducks/TakeOff`，O 字符串）。
    pub take_off: Gap,
    /// FMOD land 事件（`event:/SFX/Ducks/Land`，O 字符串）。
    pub land: Gap,
    /// 目标链读写字段（`+0xe0` moving / `+0xe1` aux / `+0xe4,+0xe8` dwell / `+0xec` 轨迹进度 /
    /// `+0xfc` 冻结字节，reva 反汇编；见 systems::critters::duck）。
    pub _gap: Gap,
}

impl Duck {
    /// `Duck::new`（行 208，O）：初始化三 FMOD 鸭子事件。
    pub fn new() -> Self {
        Duck {
            quack: Gap,
            take_off: Gap,
            land: Gap,
            _gap: Gap,
        }
    }

    /// `Duck::being_petted`（行 136，O 名；语义 U）。
    pub fn being_petted(&self) -> bool {
        false
    }
}

/// 鹪鹩组件（O；`nani_update_wren`（WP6）与 `system_critters::wren::update_wrens` 读写）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Wren(pub Gap);

/// 鱼组件（O；`system_critters::fish::fish` Boid 驱鱼）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Fish(pub Gap);

/// 鸟位管理器资源（O；`gen_id` 行 317 / `create_spots` 行 322 / `recycle` 行 329）。
#[derive(Resource)]
pub struct BirdSpotManager {
    /// `+8` u32 计数器（`gen_id` 自增，O）。
    pub next_id: u32,
    /// `+0` 及其它字段 U。
    pub _gap: Gap,
}

impl Default for BirdSpotManager {
    fn default() -> Self {
        BirdSpotManager {
            next_id: 0,
            _gap: Gap,
        }
    }
}

impl BirdSpotManager {
    /// `BirdSpotManager::gen_id`（行 317，O）：`+8 += 1`。
    pub fn gen_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id
    }

    /// `BirdSpotManager::create_spots`（行 322，O）：经 BirdSpotRecycler 池复用/分配一个鸟位。
    pub fn create_spots(&mut self, recycler: &mut BirdSpotRecycler) -> BirdSpot {
        recycler.pop_or_new(self)
    }

    /// `BirdSpotManager::recycle<closure_env$3>`（行 329，O）：`update_roof_bird_spots` 闭包回收。
    pub fn recycle<F>(&mut self, _recycler: &mut BirdSpotRecycler, _f: F)
    where
        F: FnOnce(&mut BirdSpot),
    {
    }
}

/// 鸟位记录（O 名；字段 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BirdSpot(pub Gap);

/// 鸟位池资源（O 名；`create_spots` 的池 Vec）。
#[derive(Resource, Default)]
pub struct BirdSpotRecycler {
    /// 空闲鸟位池（布局 U）。
    pub pool: Vec<BirdSpot>,
}

impl BirdSpotRecycler {
    /// 池复用或新建（O：create_spots 反汇编 0x140f86340 的池路径）。
    pub fn pop_or_new(&mut self, _manager: &mut BirdSpotManager) -> BirdSpot {
        self.pool.pop().unwrap_or_default()
    }
}

/// 实体鸟位组件（O 名；`update_roof_bird_spots`/`update_decorator_bird_spots` 写）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BirdSpots(pub Gap);
