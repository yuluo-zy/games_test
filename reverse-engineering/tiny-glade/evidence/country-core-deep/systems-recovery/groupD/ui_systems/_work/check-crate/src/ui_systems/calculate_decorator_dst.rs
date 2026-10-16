//! B `ui_systems/calculate_decorator_dst.rs` — 射线命中 → 装饰目的转换。
//!
//! - `convert_raycast_hit_to_decorator_dst` 0x14127c990（O）：按命中记录判别写
//!   16B 目的记录；未支持命中（disc∈{5,9,0xc,0xf,0x10}）= 碰撞体（装饰/杂物）
//!   走 `unreachable!`（panic "internal error: entered unreachable code:
//!   Decorators should ignore other decorators when raycasting. Make sure that
//!   the raycast passed to this function filters out decorators collision geo"
//!   0x142c240d8/0x142c24068，行 40）。
//!
//! **目的 tag 空间（O，与 core `DecoratorDst` 不同——core 为 DecoratorInfo
//! 目的 tag {0,1}=wall/2=roof/3=terrain/4=stair；本函数输出自有 tag）**：
//! `{0=Wall(WallId), 1=Roof{sub,payload}, 2=Terrain, 3=StairTop(u32), 4=Invalid}`。
//! 调用方（ui_move 0x1407a6485 / ui_place 0x7abcdd）以 `tag != 4` 门控后按
//! tag 查各自跳表（ui_move UNK_142a829e8）。
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorInfo;

/// 射线命中目的记录（16B：{tag u8@0, sub u8@1, payload u64@8}；I：字段名）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RaycastDecoratorDst {
    /// tag 0：墙（payload = WallId@+8）。
    Wall(WallId),
    /// tag 1：屋顶（sub@+1，payload@+8；I：payload = 屋顶次级 id）。
    Roof { sub: u8, payload: u64 },
    /// tag 2：地形。
    Terrain,
    /// tag 3：楼梯顶（payload u32@+4，8B 写）。
    StairTop(u32),
    /// tag 4：无效/不支持。
    Invalid,
}

impl Default for RaycastDecoratorDst {
    fn default() -> Self {
        RaycastDecoratorDst::Invalid
    }
}

impl RaycastDecoratorDst {
    /// 判别字节（O：switch 目标写值）。
    pub fn tag(self) -> u8 {
        match self {
            RaycastDecoratorDst::Wall(_) => 0,
            RaycastDecoratorDst::Roof { .. } => 1,
            RaycastDecoratorDst::Terrain => 2,
            RaycastDecoratorDst::StairTop(_) => 3,
            RaycastDecoratorDst::Invalid => 4,
        }
    }
}

/// 命中记录判别（I：hit disc 枚举；disc 0=wall / 2=roof / 0xb=terrain /
/// 0xd=stair-top / {5,9,0xc,0xf,0x10}=碰撞体 unreachable）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RaycastHitDisc {
    /// disc 0：墙命中（载荷 WallId@+8）。
    Wall(WallId),
    /// disc 2：屋顶命中（flag@+1、sub@+2、payload@+8）。
    Roof { flag: u8, sub: u8, payload: u64 },
    /// disc 0xb：地形命中。
    Terrain,
    /// disc 0xd：楼梯顶命中（载荷 u32@+4）。
    StairTop(u32),
    /// disc ∈ {5,9,0xc,0xf,0x10}：碰撞体命中（unreachable）。
    Collider,
}

/// B 0x14127c990（O）：命中记录 + DecoratorInfo → 目的记录。
///
/// - disc 0（wall）：tag 0 + WallId@+8；
/// - disc 2（roof）：`flag@+1 == 0 || type.can_place_on_roof()`（0x140833180）
///   → tag 1 + sub@+1 + payload@+8；否则 tag 4；
/// - disc 0xb（terrain）：`type.can_place_on_terrain()`（0x1408331b0）→ tag 2；
/// - disc 0xd（stair-top）：`type.can_place_on_stair_top()`（0x140833190）
///   → tag 3 + 载荷 u32@+4；否则 tag 4；
/// - 其余 → unreachable（碰撞体必须被射线过滤）。
pub fn convert_raycast_hit_to_decorator_dst(
    hit: RaycastHitDisc,
    info: &DecoratorInfo,
) -> RaycastDecoratorDst {
    match hit {
        RaycastHitDisc::Wall(wall) => RaycastDecoratorDst::Wall(wall),
        RaycastHitDisc::Roof { flag, sub, payload } => {
            if flag == 0 || info.decorator_type().can_place_on_roof() {
                RaycastDecoratorDst::Roof { sub, payload }
            } else {
                RaycastDecoratorDst::Invalid
            }
        }
        RaycastHitDisc::Terrain => {
            if info.decorator_type().can_place_on_terrain() {
                RaycastDecoratorDst::Terrain
            } else {
                RaycastDecoratorDst::Invalid
            }
        }
        RaycastHitDisc::StairTop(payload) => {
            if info.decorator_type().can_place_on_stair_top() {
                RaycastDecoratorDst::StairTop(payload)
            } else {
                RaycastDecoratorDst::Invalid
            }
        }
        RaycastHitDisc::Collider => {
            unreachable!(
                "internal error: entered unreachable code: Decorators should ignore other \
                 decorators when raycasting. Make sure that the raycast passed to this \
                 function filters out decorators collision geo"
            )
        }
    }
}
