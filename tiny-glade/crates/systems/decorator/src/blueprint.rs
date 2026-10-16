//! B `blueprint.rs` — 装饰蓝图库（D1 消费契约）。
//!
//! 证据：`DecoratorBlueprints::insert` 0x1408258b0（值 = 0x140B 蓝图结构体，
//! +0x50/+0x94/+0x138 字段、+0x120 door stairs 参数指针）、`get` 0x140825ff0
//! （hashbrown 探测查询，未中返回 0）、`add_door_stairs` 0x140825830（分配 0x20B
//! = 8×u32 参数块，替换 +0x120）、`set_interaction_mesh` 0x1408255e0（+0x80 区域
//! {u32@+0x80=0, ptr@+0x84, f32@+0x8c, Arc@+0x90, slice ptr+len@+0xa0/+0xa8}，
//! NaN 校验 + 旧 Arc 释放）。
//!
//! 本模块为 decorator_visual 调用闭合所需的最小真实接线；0x140B 完整字段表与
//! 蓝图主流程归 blueprint.rs 的 D-owner。方法签名按调用点（generate_* 各函数）
//! 与上述反编译 O 事实。

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use country_core::geometry::instanced_wall::BrickSourceId;
use country_core::resources::walls::decorator_subtype::DecoratorCollision;
use hashbrown::HashMap;

/// 交互网格记录（O：0x1408255e0 写入面 +0x80..+0xb0）。
#[derive(Clone, Debug)]
pub struct InteractionMesh {
    /// 三浮点载荷（NaN 校验通过后存 ptr@+0x84 + f32@+0x8c）。
    pub xyz: [f32; 3],
    /// Arc 网格句柄（+0x90）。
    pub mesh: Arc<()>,
    /// 网格切片（+0xa0/+0xa8，U：载荷语义待取证）。
    pub data: (usize, usize),
}

/// B DecoratorBlueprint：装饰蓝图记录（原 0x140B=320B 结构体）。
/// 仅恢复 decorator_visual 消费面字段；其余偏移按 U 保留命名。
#[derive(Clone, Debug, Default)]
pub struct DecoratorBlueprint {
    /// +0x00：u64（旗写面 = {u32 1, u32 0}；O generate_flags 0x1407ff6c0）。
    pub u50: u64,
    /// +0x80..+0xb0：交互网格（set_interaction_mesh 0x1408255e0 写；None=未设）。
    pub interaction: Option<InteractionMesh>,
    /// +0x94：u64（U）。
    pub u94: u64,
    /// +0x120：door stairs 装配参数（8×u32，0x140825830 分配替换）。
    pub door_stairs: Option<Box<[u32; 8]>>,
    /// +0x10/+0x18：旗布基（O 写面 {0.5,-0.7071,0.7071,0.5} = 四元数分量；
    /// generate_flags 0x1407ff6c0）。
    pub flag_cloth_basis: [f32; 4],
    /// +0x20..0x40：变换槽（O 写面 pos/quat；CachedDecoratorTransform 语义 U）。
    pub transform_slots: [u64; 4],
    /// +0x138：u8（U）。
    pub u138: u8,
    /// 碰撞体列表（set_colliders 0x14017a22f0/0x1408ed6f0 消费面；原 +0x0 区域）。
    pub colliders: Vec<DecoratorCollision>,
    /// 生成变换（I：generate_* 写面承载；与 transform_slots 对应）。
    pub transform: Option<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform>,
    /// 描边网格（set_outline 消费面；I：原 +0x20 区域，U：载荷语义）。
    pub outline: Option<(f32, f32, f32)>,
    /// 烟囱参数（add_chimney 消费面，U：载荷语义）。
    pub chimney: Option<f32>,
    /// 布（add_cloth 消费面，U：载荷语义）。
    pub cloth: Option<u8>,
}

impl DecoratorBlueprint {
    /// new（内联，I）：全零蓝图。
    pub fn new() -> Self {
        Self::default()
    }

    /// 0x14017a22f0（O 结构）/0x1408ed6f0：设置碰撞体列表（替换 +0x0 区域）。
    pub fn set_colliders(&mut self, colliders: Vec<DecoratorCollision>) {
        self.colliders = colliders;
    }

    /// set_interaction_mesh 0x1408255e0（O）：交互网格写 +0x80 区域。NaN 校验
    /// （3 个 f32 指数位 < 0x7f800000）失败则保持原值。
    pub fn set_interaction_mesh(&mut self, xyz: [f32; 3], data: (usize, usize)) {
        if xyz.iter().all(|v| (v.to_bits() & 0x7fff_ffff) < 0x7f80_0000) {
            self.interaction = Some(InteractionMesh {
                xyz,
                mesh: Arc::new(()),
                data,
            });
        }
    }

    /// set_outline（内联，I）：描边网格（载荷 U）。
    pub fn set_outline(&mut self, outline: (f32, f32, f32)) {
        self.outline = Some(outline);
    }

    /// 0x140825830（O）：door stairs 装配参数（0x20B = 8×u32）替换 +0x120
    /// （旧块释放）。
    pub fn add_door_stairs(&mut self, params: [u32; 8]) {
        self.door_stairs = Some(Box::new(params));
    }

    /// add_chimney（内联，I）：烟囱参数（载荷 U）。
    pub fn add_chimney(&mut self, v: f32) {
        self.chimney = Some(v);
    }

    /// add_cloth（内联，I）：布（载荷 U）。
    pub fn add_cloth(&mut self, v: u8) {
        self.cloth = Some(v);
    }

    /// unset_collider（内联，I）：清空碰撞体列表。
    pub fn unset_collider(&mut self) {
        self.colliders.clear();
    }

    /// 砖源 id 消费面（generate_* 装配读：BrickSourceId 为墙砖源，U：精确用途）。
    pub fn brick_source(&self) -> Option<BrickSourceId> {
        None
    }
}

/// B DecoratorBlueprints：DecoratorId(u32) → DecoratorBlueprint 蓝图库。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorBlueprints {
    /// 蓝图表。
    pub map: HashMap<u32, DecoratorBlueprint>,
}

impl DecoratorBlueprints {
    /// 0x1408258b0（O）：插入蓝图（键 u32；冲突行为 U，D1 以覆盖语义承载）。
    pub fn insert(&mut self, id: u32, blueprint: DecoratorBlueprint) {
        self.map.insert(id, blueprint);
    }

    /// 0x140825ff0（O）：查询蓝图，未中返回 None（原返回 0）。
    pub fn get(&self, id: u32) -> Option<&DecoratorBlueprint> {
        self.map.get(&id)
    }

    /// 可变查询（内联，I）。
    pub fn get_mut(&mut self, id: u32) -> Option<&mut DecoratorBlueprint> {
        self.map.get_mut(&id)
    }

    /// 删除（内联，I；移动/移除族消费）。
    pub fn remove(&mut self, id: u32) -> Option<DecoratorBlueprint> {
        self.map.remove(&id)
    }
}
