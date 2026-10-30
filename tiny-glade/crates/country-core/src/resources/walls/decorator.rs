//! B `resources/walls/decorator.rs` — 装饰附着/信息类型族（原 131 函数：业务 56）。
//!
//! 证据：`evidence/country-core-deep/chapters/resources/walls/decorator.rs.md`
//! （DecoratorType 谓词位运算全表、判别访问、TransientDecoratorIdGen、
//! DeferredDecoratorOpEvents）+ `wall/decomp/decomp-*.txt` 定点反编译。
//! 记号：O=机器码证实 / I=推断 / U=未恢复；证据不足项在注释登记「待取证」。
//!
//! 关键纠错（RE-DECORATOR-STORAGE §纠错）：DecoratorId 的 serde 是 u32
//! 十进制化（newtype serde，0x1402ddd20），**不是** DecoratorInfo 的序列化。

use glam::{Mat2, Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Condvar, Mutex};

use super::WallId;
use utils::wall_space::WallSpaceCoord;

/// B DecoratorType：u8 判别。类型值域（O）：0x38 cottage 窗、0x39 gothic 窗、
/// 0x3a 第三窗型（不可上屋顶）、0x3b 窗类、0x3c..0x3e 楼梯顶可放、0x3f 角吸附型。
/// 谓词均为原机器码位运算（章节 §DecoratorType 谓词位运算族，全部 O）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecoratorType(pub u8);

impl DecoratorType {
    /// 0x140833130：`(t+0xc8)<3 → 3 否则 1`（u8 回绕）。窗族 max_rank=3，其余 1。
    pub fn max_rank(self) -> u8 {
        if self.0.wrapping_add(0xc8) < 3 { 3 } else { 1 }
    }

    /// 0x140833150：`(t+0xc8)<8 && ((0x27 >> (t&7)) & 1) != 0` → t∈{0x38,0x39,0x3a,0x3d}。
    pub fn can_change_seed(self) -> bool {
        (self.0.wrapping_add(0xc8) < 8) && ((0x27u8 >> (self.0 & 7)) & 1) != 0
    }

    /// 0x140833160：`(t+0xc8)<3` → t∈{0x38,0x39,0x3a}。
    pub fn is_window(self) -> bool {
        self.0.wrapping_add(0xc8) < 3
    }

    /// 0x140833170：`(t&0x3e)==0x38` → t∈{0x38,0x39}。
    pub fn spawns_clutter(self) -> bool {
        self.0 & 0x3e == 0x38
    }

    /// 0x140833180：`t!=0x3a`（原名 can_place_on_roof / can_place_on_flat_roof）。
    pub fn can_place_on_roof(self) -> bool {
        self.0 != 0x3a
    }

    /// 0x140833190（RNE 手解）：`(t+0xc4)<3 || (t&0x38)!=0x38`。仅窗族 {0x38..0x3b}
    /// 不可放楼梯顶。
    pub fn can_place_on_stair_top(self) -> bool {
        self.0.wrapping_add(0xc4) < 3 || (self.0 & 0x38) != 0x38
    }

    /// 0x1408331b0：`(t&0x3c)!=0x38` → t∉{0x38..0x3b}。
    pub fn can_place_on_terrain(self) -> bool {
        self.0 & 0x3c != 0x38
    }

    /// 0x1408331c0：`t!=0x3f`。
    pub fn should_offer_snap_to_corner(self) -> bool {
        self.0 != 0x3f
    }
}

/// B DecoratorId：装饰唯一 id。serde = u32 十进制化（newtype serde，0x1402ddd20）。
/// `new` 0x14067dd80 = 恒等返回；`Display` 0x1408331d0。
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct DecoratorId(pub u32);

/// Wall/roof UI removal request consumed by the decorator operation system.
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct ExternalDecoratorCmd {
    pub wall: u64,
    pub decorator_id: u32,
    pub dst: crate::resources::walls::decorator_storage::DecoratorStorageAddr,
    pub payload: [u8; 12],
    pub stream: u8,
}

impl DecoratorId {
    /// 0x14067dd80：恒等返回（也复用为 DeferredDecoratorOpEvents::events）。
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// 行 166（内联）。
    #[inline]
    pub fn get_inner(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for DecoratorId {
    /// 0x1408331d0：十进制 u32 输出。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// B DecoratorIdGen：顺序 id 生成器。`reserve`/`generate` 行 806/811（内联）。
/// 结构 I：`next: u32`；generate 递增返回。
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorIdGen {
    /// I：下一个待分配 id。
    next: u32,
}

impl DecoratorIdGen {
    /// 原瞬时生成器非提交路径只观察当前基数；预览不能消耗正式装饰 ID。
    #[inline]
    pub fn peek(&self) -> DecoratorId {
        DecoratorId(self.next)
    }

    /// 原1408339c0：预览借用当前基数，提交直写同一个正式计数器。
    /// 借用绑定整个栈上生成器，不能在计数器释放或再次借用后继续生成ID。
    pub fn transient(&mut self, direct_write: bool) -> TransientDecoratorIdGen<'_> {
        TransientDecoratorIdGen::new(&mut self.next, direct_write)
    }

    /// 行 806（内联）：预留到至少 `id`。fresh `restore_decorators` 0x14155c4e1 逐句：
    /// `next = max(u32(next), u32(id) + 1)`（`MOV RDX,id; INC RDX; CMP RDX,next;
    /// CMOVBE RDX,next; MOV [gen],EDX`——64 位加/比较后截断存 u32）。用于历史回放
    /// 重插装饰后防止后续 generate 复用已存在 id。
    #[inline]
    pub fn reserve(&mut self, id: DecoratorId) {
        let next = u64::from(self.next).max(u64::from(id.0) + 1);
        self.next = next as u32;
    }

    /// 行 811（内联）：分配下一个 id。
    #[inline]
    pub fn generate(&mut self) -> DecoratorId {
        let id = DecoratorId(self.next);
        self.next = self.next.wrapping_add(1);
        id
    }
}

/// B TransientDecoratorIdGen：瞬时 id 生成器（0x1408339c0/0x1408339e0，O）。
/// 原结构：`{base:*mut u32@+0, +0x8:u64 偏移游标, +0x10:bool 直写模式}`。
/// `generate`：直写模式（flag=1）→ `*base += 1` 返回旧值；否则 `+0x8 += 1`，
/// 返回 `*base + 旧偏移`。现代接口保留对正式计数器的独占借用，不保留悬空裸地址。
pub struct TransientDecoratorIdGen<'a> {
    /// +0x00：栈上暂借调用者的正式基数。
    base: &'a mut u32,
    /// +0x08：偏移游标。
    offset: u64,
    /// +0x10：直写模式。
    direct_write: bool,
}

impl<'a> TransientDecoratorIdGen<'a> {
    /// 原1408339c0：offset=0，模式来自调用者的实际预览/提交阶段。
    pub fn new(base: &'a mut u32, direct_write: bool) -> Self {
        Self {
            base,
            offset: 0,
            direct_write,
        }
    }

    /// 0x1408339e0（O）。直写模式：`*base += 1` 返回旧值；否则 `+0x8 += 1`，
    /// 返回 `*base + 旧偏移`。
    pub fn generate(&mut self) -> u32 {
        if self.direct_write {
            let slot = &mut *self.base;
            let old = *slot;
            *slot = old.wrapping_add(1);
            old
        } else {
            let old = self.offset;
            self.offset = old.wrapping_add(1);
            let base = *self.base;
            base.wrapping_add(old as u32)
        }
    }
}

/// B DecoratorRank：装饰 rank（0x140833250 = `mov eax,ecx` 字节直通，O）。
/// `is_atomic`/`is_compound`/`x` 为内联谓词（行 256/261/266，I：语义待取证）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecoratorRank(pub u8);

impl DecoratorRank {
    /// 0x140833250（O）：字节直通。
    pub fn new(x: u8) -> Self {
        Self(x)
    }

    /// 行 256（内联，I）：原子装饰（非多开复合）。
    pub fn is_atomic(self) -> bool {
        self.0 == 0
    }

    /// 行 261（内联，I）：复合装饰。
    pub fn is_compound(self) -> bool {
        self.0 != 0
    }

    /// 行 266（内联，I）：原始值。
    pub fn x(self) -> u8 {
        self.0
    }
}

/// B 140833240/15B：定位键由实际存储地址和独立装饰 ID 组成。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorLocator {
    pub addr: super::decorator_storage::DecoratorStorageAddr,
    /// 原构造器 +0x10 的 u32；不能用可变 Info.word2c 代替。
    pub decorator: u32,
}

impl DecoratorLocator {
    pub fn new(addr: super::decorator_storage::DecoratorStorageAddr, decorator: u32) -> Self {
        Self { addr, decorator }
    }
}

/// B ClutterVersion：clutter 版本号（0x140833230 is_initial = `cmp byte,0; sete`，O）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClutterVersion(pub u8);

impl ClutterVersion {
    /// 行 205（内联，U：latest 数值待取证）。
    pub fn latest() -> Self {
        Self(1)
    }

    /// 0x140833230（O）：版本 == 初始值（0）。
    pub fn is_initial(self) -> bool {
        self.0 == 0
    }
}

/// B DecoratorExtraInfo：装饰附加信息联合，判别字节位于 DecoratorInfo+0x11。
///
/// 判别值（2026-10-17 fresh disasm 定案，与既有注释的 Window/Lantern/Chimney
/// 值错位已修正）：
/// - **disc 0/1：楼梯**（`as_stair` 0x1408394b0 = `(disc|0xf8)<0xfa` ⇔ disc∈{0,1}；
///   载荷 20B {16B@+0, u32@+0x10}，对照 impl$97 0x140839560 无判别写入的整载荷拷贝）
/// - **disc 4：烟囱**（`as_chimney` 0x1408392d0 `cmp byte[+0x11],0x4`；载荷 {f32@+0,
///   u8@+4}，From 0x140839520 写 f32+u8）
/// - **disc 5：旗**（`as_flag` 0x140839340 `cmp,0x5`；载荷 {u8@+0}）
/// - **disc 6：窗**（`as_window` 0x1408393b0 `cmp,0x6`；载荷 {u32@+0}，From
///   0x140839540 写 EDX u32——非 {f32,u8}）
/// - **disc 7：灯**（`as_lantern` 0x140839420 `cmp,0x7`；载荷 {f32@+0}，From
///   0x140839550 写 XMM1 f32——非 {u32}）
///
/// From 变体地址映射（fresh disasm）：0x140839520=Chimney、0x140839530=Flag、
/// 0x140839540=Window、0x140839550=Lantern、0x140839560=Stair。
///
/// 载荷含 f32（窗/烟囱），只派生 PartialEq。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorExtraInfo {
    /// disc 0/1：楼梯附加信息（20B 载荷，字段语义 U）。
    Stair(StairExtra),
    /// disc 6：窗附加信息 {u32@+0}。
    Window(WindowExtra),
    /// disc 5：旗附加信息 {u8@+0}。
    Flag(FlagExtra),
    /// disc 7：灯附加信息 {f32@+0}。
    Lantern(LanternExtra),
    /// disc 4：烟囱附加信息 {f32@+0, u8@+4}。
    Chimney(ChimneyExtra),
}

/// 楼梯附加信息（20B：{16B@+0, u32@+0x10}）。字段语义 U。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StairExtra(pub [u8; 20]);

/// 窗附加信息（disc 6：{u32@+0}）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WindowExtra(pub u32);

/// 旗附加信息（disc 5：{u8@+0}）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FlagExtra(pub u8);

/// 灯附加信息（disc 7：{f32@+0}）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LanternExtra(pub f32);

/// 烟囱附加信息（disc 4：{f32@+0, u8@+4}，O：`set_flag_connection_and_chimney_smoke`
/// 0x142298970 经 `as_chimney()` 后写 `*(u8*)(+4) = 事件值`——烟值建模 +4 字段）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChimneyExtra {
    /// +0：f32 载荷（判别访问 as_chimney 返回）。
    pub f: f32,
    /// +4：烟开关/浓度字节。
    pub smoke: u8,
}

impl DecoratorExtraInfo {
    /// 判别字节 +0x11 值（fresh 0x1408392d0/340/3b0/420/4b0 定案：烟囱=4、旗=5、
    /// 窗=6、灯=7、楼梯=0/1）。
    pub fn disc(self) -> u8 {
        match self {
            DecoratorExtraInfo::Stair(value) => value.0[17],
            DecoratorExtraInfo::Window(_) => 6,
            DecoratorExtraInfo::Flag(_) => 5,
            DecoratorExtraInfo::Lantern(_) => 7,
            DecoratorExtraInfo::Chimney(_) => 4,
        }
    }

    /// 0x140839490（O）：disc∈{0,1} → Some，否则 None。
    pub fn try_as_stair(self) -> Option<StairExtra> {
        match self {
            DecoratorExtraInfo::Stair(v) => Some(v),
            _ => None,
        }
    }

    /// 行 368（内联，I）。
    pub fn try_as_chimney(self) -> Option<ChimneyExtra> {
        match self {
            DecoratorExtraInfo::Chimney(v) => Some(v),
            _ => None,
        }
    }

    /// 行 368（内联，I）。
    pub fn try_as_flag(self) -> Option<FlagExtra> {
        match self {
            DecoratorExtraInfo::Flag(v) => Some(v),
            _ => None,
        }
    }

    /// 行 368（内联，I）。
    pub fn try_as_lantern(self) -> Option<LanternExtra> {
        match self {
            DecoratorExtraInfo::Lantern(v) => Some(v),
            _ => None,
        }
    }

    /// 行 368（内联，I）。
    pub fn try_as_window(self) -> Option<WindowExtra> {
        match self {
            DecoratorExtraInfo::Window(v) => Some(v),
            _ => None,
        }
    }

    /// 0x1408392d0（O）：烟囱访问（disc==4），非烟囱 fmt panic。
    pub fn as_chimney(self) -> ChimneyExtra {
        self.try_as_chimney()
            .expect("DecoratorExtraInfo::as_chimney: not a chimney")
    }

    /// 0x1408392d0（O，mut 同址）。
    pub fn as_chimney_mut(&mut self) -> &mut ChimneyExtra {
        match self {
            DecoratorExtraInfo::Chimney(v) => v,
            _ => panic!("DecoratorExtraInfo::as_chimney_mut: not a chimney"),
        }
    }

    /// 0x140839340（O）：旗访问，非旗 fmt panic。
    pub fn as_flag(self) -> FlagExtra {
        self.try_as_flag()
            .expect("DecoratorExtraInfo::as_flag: not a flag")
    }

    /// 0x140839340（O，mut 同址）。
    pub fn as_flag_mut(&mut self) -> &mut FlagExtra {
        match self {
            DecoratorExtraInfo::Flag(v) => v,
            _ => panic!("DecoratorExtraInfo::as_flag_mut: not a flag"),
        }
    }

    /// 0x1408393b0（O）：窗访问（disc==6）。
    pub fn as_window(self) -> WindowExtra {
        self.try_as_window()
            .expect("DecoratorExtraInfo::as_window: not a window")
    }

    /// 0x1408393b0（I，mut 同址）。
    pub fn as_window_mut(&mut self) -> &mut WindowExtra {
        match self {
            DecoratorExtraInfo::Window(v) => v,
            _ => panic!("DecoratorExtraInfo::as_window_mut: not a window"),
        }
    }

    /// 0x140839420（O）：灯访问（disc==7）。
    pub fn as_lantern(self) -> LanternExtra {
        self.try_as_lantern()
            .expect("DecoratorExtraInfo::as_lantern: not a lantern")
    }

    /// 0x140839420（I，mut 同址）。
    pub fn as_lantern_mut(&mut self) -> &mut LanternExtra {
        match self {
            DecoratorExtraInfo::Lantern(v) => v,
            _ => panic!("DecoratorExtraInfo::as_lantern_mut: not a lantern"),
        }
    }

    /// 0x1408394b0（O）：楼梯访问，非楼梯 fmt panic。
    pub fn as_stair(self) -> StairExtra {
        self.try_as_stair()
            .expect("DecoratorExtraInfo::as_stair: not a stair")
    }

    /// 0x1408394b0（O，mut 同址）。
    pub fn as_stair_mut(&mut self) -> &mut StairExtra {
        match self {
            DecoratorExtraInfo::Stair(v) => v,
            _ => panic!("DecoratorExtraInfo::as_stair_mut: not a stair"),
        }
    }
}

// From 变体（fresh disasm 2026-10-17）。判别值写入原 +0x11；Rust 枚举判别即变体序。
impl From<StairExtra> for DecoratorExtraInfo {
    /// 0x140839560（Stair）：整载荷拷贝（16B@+0 + u32@+0x10），判别不重写
    /// （源载荷已带正确判别）。
    fn from(v: StairExtra) -> Self {
        DecoratorExtraInfo::Stair(v)
    }
}
impl From<WindowExtra> for DecoratorExtraInfo {
    /// 0x140839540（Window，disc=6）：u32 载荷写 +0（EDX）。
    fn from(v: WindowExtra) -> Self {
        DecoratorExtraInfo::Window(v)
    }
}
impl From<FlagExtra> for DecoratorExtraInfo {
    /// 0x140839530（Flag，disc=5）：u8 载荷写 +0（DL）。
    fn from(v: FlagExtra) -> Self {
        DecoratorExtraInfo::Flag(v)
    }
}
impl From<LanternExtra> for DecoratorExtraInfo {
    /// 0x140839550（Lantern，disc=7）：f32 载荷写 +0（XMM1）。
    fn from(v: LanternExtra) -> Self {
        DecoratorExtraInfo::Lantern(v)
    }
}
impl From<ChimneyExtra> for DecoratorExtraInfo {
    /// 0x140839520（Chimney，disc=4）：{f32@+0, u8@+4} 载荷写（XMM1 + R8B）。
    fn from(v: ChimneyExtra) -> Self {
        DecoratorExtraInfo::Chimney(v)
    }
}

/// Actual wall destination payload written by B1412801f0/512B.
/// The authority wall belongs to DecoratorStorageAddr; coord is not its ID.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallDecoratorDst {
    pub coord: WallSpaceCoord,
    pub platform: WallId,
    pub relative_y: f32,
    pub position: Vec3,
    pub corner_attachment: u8,
    pub bottom_as_door: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofDecoratorDst {
    pub coord: Vec2,
    pub yaw: DecoratorRotationXZ,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainDecoratorDst {
    pub position_xz: Vec2,
    pub yaw: DecoratorRotationXZ,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairDecoratorDst {
    pub first_id: u32,
    pub second_id: u32,
    /// Signed lateral coordinate and longitudinal U.
    pub coordinate: Vec2,
    pub yaw: DecoratorRotationXZ,
}

/// Canonical storage destination. Tags0/1/2/3/4 correspond to B byte+22;
/// this logical Rust value does not impose the original union ABI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorDst {
    /// tag 0：墙目的地。
    Wall(WallDecoratorDst),
    /// tag 1：墙目的地（复合墙，同存储桶；差异 U）。
    WallCompound(WallDecoratorDst),
    /// tag 2：屋顶目的地。
    Roof(RoofDecoratorDst),
    /// tag 3：地形目的地。
    Terrain(TerrainDecoratorDst),
    /// tag 4：楼梯目的地。
    Stair(StairDecoratorDst),
}

impl DecoratorDst {
    /// 判别字节 +0x22 值。
    pub fn tag(self) -> u8 {
        match self {
            DecoratorDst::Wall(_) => 0,
            DecoratorDst::WallCompound(_) => 1,
            DecoratorDst::Roof(_) => 2,
            DecoratorDst::Terrain(_) => 3,
            DecoratorDst::Stair(_) => 4,
        }
    }

    /// 0x1408334d0（O）：`tag<2`。
    pub fn is_wall(self) -> bool {
        self.tag() < 2
    }

    /// 0x1408334e0（O）：tag==2 → &self，否则 panic（行 0x28）。
    pub fn as_roof(&self) -> &RoofDecoratorDst {
        match self {
            Self::Roof(roof) => roof,
            _ => panic!("DecoratorDst::as_roof: not a roof destination"),
        }
    }

    /// 0x140833510（O）：tag<2 → &self，否则 panic。
    pub fn as_wall(&self) -> &WallDecoratorDst {
        match self {
            Self::Wall(wall) | Self::WallCompound(wall) => wall,
            _ => panic!("DecoratorDst::as_wall: not a wall destination"),
        }
    }

    /// 0x140833540（O）：tag<2 → &mut self，否则 unreachable。
    pub fn as_wall_mut(&mut self) -> &mut WallDecoratorDst {
        match self {
            Self::Wall(wall) | Self::WallCompound(wall) => wall,
            _ => unreachable!("DecoratorDst::as_wall_mut: not a wall destination"),
        }
    }
}

/// B DecoratorInfo：72B（0x48）权威装饰记录。
///
/// 布局（apply_decorator_place 0x140a20c70 读面 + decorator.rs 判别访问，O；
/// 2026-10-16 fresh 反编译复核）：`+0x00/+0x08/+0x10 u64` 目的、`+0x22` dst tag、
/// `+0x2c` 可变业务 word、`+0x44` rank、`+0x46` DecoratorType、`+0x47` 原始 flags。
/// insert 0x14155afe0 写2的是返回 Option<DecoratorInfo> 的 None niche，
/// 不是向 live map 写 flags=2；存储条目保留传入 flags。独立 map key 由操作和
/// DecoratorLocator 承载，不是 Info.word2c，其完整业务语义仍未知。E2 取证（fresh
/// 0x140a20670）：移除拷贝 0x48B = {8×u64@+0x00..0x40, state u8@+0x47}，与
/// `+0x44..0x47` rank/state 簇一致（rank@+0x44、**+0x45 1 字节 U**、type@+0x46、
/// state@+0x47）。完整逐字段语义表仍 U（RE-DECORATOR-STORAGE §6.1）；未知字段
/// 用 raw 名，不编造业务含义。**无 bevy Entity 字段**——实体由消费系统读
/// DecoratorInfo 后另行生成，关联靠 DecoratorId 组件（RE §2 实体机制）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorInfo {
    /// +0x00：destination payload (wall/roof coordinates, terrain XZ, or stair IDs).
    pub dst_wall: u64,
    /// +0x08：目的次级 id。
    pub dst_secondary: u64,
    /// +0x10：目的载荷（DecoratorExtraInfo 判别字节 @+0x11）。
    pub dst_payload: u64,
    /// +0x18：墙空间数据（字段语义 U）。
    pub u18: u64,
    /// +0x20：u32（dst tag 字节 @+0x22）。
    pub u20: u32,
    /// +0x24：u32（U）。
    pub u24: u32,
    /// +0x28：u32（U）。
    pub u28: u32,
    /// +0x2c：可变业务 word；保留旧字段名，完整语义未知，不是独立 map key。
    pub decorator_id: u32,
    /// +0x30：u64（U）。
    pub u30: u64,
    /// +0x38：u32（U）。
    pub u38: u32,
    /// +0x3c：u32（U）。
    pub u3c: u32,
    /// +0x40：u32（U）。
    pub u40: u32,
    /// +0x44：rank（u8）。
    pub rank: u8,
    /// +0x45：u8（E2 取证 2026-10-16：rank(+0x44) 与 type(+0x46) 间 1 字节，
    /// rank/state 簇 +0x44..0x47，精确语义 U——待装饰渲染/实体消费端读面）。
    pub u45: u8,
    /// +0x46：DecoratorType（u8）。
    pub ty: u8,
    /// +0x47：原始 flags；2是 Option 返回值的 None niche，不是 live 条目状态。
    /// 原始访问器保留该字节，不在这里屏蔽或规范化未知值。
    pub state: u8,
}

impl DecoratorInfo {
    /// +0x41：style/判别字节（`u40` 的 byte1）。
    ///
    /// `DecoratorHistoryState::from`（0x140b3f930）与
    /// `convert_before_after_states_to_changes`（0x140b3fab0）以该字节分变体
    /// （2..8 夹取；style==4 Chimney{ f32,u8 }、5 Flag{ u8 }、6 Window{ u32 }、
    /// 7 Lantern{ f32 }、8+ 默认簇）。机器码 `mov r?,[rdx+0x41]` 直读 byte1 of +0x40。
    #[inline]
    pub fn extra_style(self) -> u8 {
        ((self.u40 >> 8) & 0xff) as u8
    }

    /// 原窗口访问（B 0x1408393b0）：只接受Info+0x41的窗判别，
    /// 返回Info+0x30的u32载荷；目的地载荷和窗框颜色属于独立字段。
    #[inline]
    pub fn window_extra(&self) -> WindowExtra {
        if self.extra_style() != 6 {
            panic!("DecoratorExtraInfo::as_window: not a window");
        }
        WindowExtra(self.u30 as u32)
    }

    /// +0x22：目的 tag。
    #[inline]
    pub fn dst_tag(self) -> u8 {
        ((self.u20 >> 16) & 0xff) as u8
    }

    /// Decode only the first two f32 words. Original wall/default readers
    /// perform their own tag dispatch, including raw tags outside the typed Dst.
    pub fn wall_space_coord(&self) -> WallSpaceCoord {
        WallSpaceCoord::new_unchecked(
            f32::from_bits(self.dst_wall as u32),
            f32::from_bits((self.dst_wall >> 32) as u32),
        )
    }

    /// Read only the defined payload of the selected canonical destination.
    /// Raw padding is never interpreted as a field or normalized.
    pub fn destination(&self) -> DecoratorDst {
        let coord = self.wall_space_coord();
        match self.dst_tag() {
            tag @ (0 | 1) => {
                let bottom_as_door = match (self.u20 >> 8) as u8 {
                    0 => false,
                    1 => true,
                    value => panic!("invalid wall destination boolean {value}"),
                };
                let wall = WallDecoratorDst {
                    coord,
                    platform: WallId(self.dst_secondary),
                    relative_y: f32::from_bits(self.dst_payload as u32),
                    position: Vec3::new(
                        f32::from_bits((self.dst_payload >> 32) as u32),
                        f32::from_bits(self.u18 as u32),
                        f32::from_bits((self.u18 >> 32) as u32),
                    ),
                    corner_attachment: self.u20 as u8,
                    bottom_as_door,
                };
                if tag == 0 { DecoratorDst::Wall(wall) } else { DecoratorDst::WallCompound(wall) }
            }
            2 => DecoratorDst::Roof(RoofDecoratorDst {
                coord: Vec2::new(coord.u, coord.y),
                yaw: DecoratorRotationXZ(f32::from_bits(self.dst_secondary as u32)),
            }),
            3 => DecoratorDst::Terrain(TerrainDecoratorDst {
                position_xz: Vec2::new(coord.u, coord.y),
                yaw: DecoratorRotationXZ(f32::from_bits(self.dst_secondary as u32)),
            }),
            4 => DecoratorDst::Stair(StairDecoratorDst {
                first_id: self.dst_wall as u32,
                second_id: (self.dst_wall >> 32) as u32,
                coordinate: Vec2::new(
                    f32::from_bits(self.dst_secondary as u32),
                    f32::from_bits((self.dst_secondary >> 32) as u32),
                ),
                yaw: DecoratorRotationXZ(f32::from_bits(self.dst_payload as u32)),
            }),
            tag => panic!("invalid decorator destination tag {tag}"),
        }
    }

    /// Update only bytes written for this variant by original calculate_move_dst.
    /// All other Info fields and variant padding retain their previous bits.
    pub fn set_destination(&mut self, destination: DecoratorDst) {
        fn pair(first: f32, second: f32) -> u64 {
            u64::from(first.to_bits()) | (u64::from(second.to_bits()) << 32)
        }
        let tag = destination.tag();
        match destination {
            DecoratorDst::Wall(wall) | DecoratorDst::WallCompound(wall) => {
                self.dst_wall = pair(wall.coord.u, wall.coord.y);
                self.dst_secondary = wall.platform.0;
                self.dst_payload = pair(wall.relative_y, wall.position.x);
                self.u18 = pair(wall.position.y, wall.position.z);
                self.u20 = (self.u20 & 0xff00_0000)
                    | u32::from(wall.corner_attachment)
                    | (u32::from(wall.bottom_as_door) << 8)
                    | (u32::from(tag) << 16);
            }
            DecoratorDst::Roof(roof) => {
                self.dst_wall = pair(roof.coord.x, roof.coord.y);
                self.dst_secondary = (self.dst_secondary & 0xffff_ffff_0000_0000)
                    | u64::from(roof.yaw.0.to_bits());
                self.u20 = (self.u20 & 0xff00_ffff) | (u32::from(tag) << 16);
            }
            DecoratorDst::Terrain(terrain) => {
                self.dst_wall = pair(terrain.position_xz.x, terrain.position_xz.y);
                self.dst_secondary = (self.dst_secondary & 0xffff_ffff_0000_0000)
                    | u64::from(terrain.yaw.0.to_bits());
                self.u20 = (self.u20 & 0xff00_ffff) | (u32::from(tag) << 16);
            }
            DecoratorDst::Stair(stair) => {
                self.dst_wall = u64::from(stair.first_id) | (u64::from(stair.second_id) << 32);
                self.dst_secondary = pair(stair.coordinate.x, stair.coordinate.y);
                self.dst_payload = (self.dst_payload & 0xffff_ffff_0000_0000)
                    | u64::from(stair.yaw.0.to_bits());
                self.u20 = (self.u20 & 0xff00_ffff) | (u32::from(tag) << 16);
            }
        }
    }

    /// +0x44：rank。
    #[inline]
    pub fn rank(self) -> u8 {
        self.rank
    }

    /// +0x46：DecoratorType。
    #[inline]
    pub fn decorator_type(self) -> DecoratorType {
        DecoratorType(self.ty)
    }

    /// +0x47：原始 flags 直通；2仅在 Option 返回值中充当 None niche。
    #[inline]
    pub fn state(self) -> u8 {
        self.state
    }

    /// +0x2c：旧类型包装访问器；完整业务语义未知，不能用它寻址独立 map key。
    #[inline]
    pub fn decorator_id(self) -> DecoratorId {
        DecoratorId(self.decorator_id)
    }

    /// 目的是否为墙（tag<2）。
    #[inline]
    pub fn is_wall_dst(self) -> bool {
        self.dst_tag() < 2
    }

    /// 0x140833260（O）：tag<2 → &self，否则 panic（行 40）。
    pub fn dst_as_wall(&self) -> &Self {
        if self.is_wall_dst() {
            self
        } else {
            panic!("DecoratorInfo::dst_as_wall: destination is not a wall")
        }
    }

    /// 0x1408332c0（O）：tag<2 → &mut self，否则 `unreachable!("dst is
    /// Roof/Stair/Terrain")`。
    pub fn dst_as_wall_mut(&mut self) -> &mut Self {
        if self.is_wall_dst() {
            self
        } else {
            unreachable!("dst is Roof/Stair/Terrain")
        }
    }

    /// 0x1408333a0（O）：tag<2 → &self，否则 null（Option::None）。
    pub fn try_dst_as_wall(&self) -> Option<&Self> {
        self.is_wall_dst().then_some(self)
    }

    /// 0x1408333b0（I）：tag==2 判别访问，非屋顶 panic。
    pub fn dst_as_roof(&self) -> &Self {
        if self.dst_tag() == 2 {
            self
        } else {
            panic!("DecoratorInfo::dst_as_roof: destination is not a roof")
        }
    }

    /// 0x140833410（O）：tag==3 → &self，否则 panic。
    pub fn dst_as_terrain(&self) -> &Self {
        if self.dst_tag() == 3 {
            self
        } else {
            panic!("DecoratorInfo::dst_as_terrain: destination is not terrain")
        }
    }

    /// 0x140833470（I）：tag==4 判别访问。
    pub fn dst_as_stair(&self) -> &Self {
        if self.dst_tag() == 4 {
            self
        } else {
            panic!("DecoratorInfo::dst_as_stair: destination is not a stair")
        }
    }

    /// 行 602（内联）：是否为默认（全零）记录。
    pub fn is_default(&self) -> bool {
        *self == DecoratorInfo::default()
    }

    /// Native ExtraInfo is the twenty-byte region Info+0x30..0x44.
    /// Its enum niche is Extra+0x11 = Info+0x41 (u40 byte 1).
    /// The destination's +0x10 payload is a different, independent union.
    #[inline]
    pub fn extra_disc(self) -> u8 {
        ((self.u40 >> 8) & 0xff) as u8
    }

    fn extra_bytes(self) -> [u8; 20] {
        let mut raw = [0u8; 20];
        raw[..8].copy_from_slice(&self.u30.to_le_bytes());
        raw[8..12].copy_from_slice(&self.u38.to_le_bytes());
        raw[12..16].copy_from_slice(&self.u3c.to_le_bytes());
        raw[16..20].copy_from_slice(&self.u40.to_le_bytes());
        raw
    }

    pub fn extra(self) -> DecoratorExtraInfo {
        let raw = self.extra_bytes();
        match self.extra_disc() {
            0 | 1 => DecoratorExtraInfo::Stair(StairExtra(raw)),
            4 => DecoratorExtraInfo::Chimney(ChimneyExtra {
                f: f32::from_le_bytes(raw[..4].try_into().unwrap()),
                smoke: raw[4],
            }),
            5 => DecoratorExtraInfo::Flag(FlagExtra(raw[0])),
            6 => DecoratorExtraInfo::Window(WindowExtra(u32::from_le_bytes(
                raw[..4].try_into().unwrap(),
            ))),
            7 => DecoratorExtraInfo::Lantern(LanternExtra(f32::from_le_bytes(
                raw[..4].try_into().unwrap(),
            ))),
            d => panic!("unknown DecoratorExtraInfo discriminator {d:#x}"),
        }
    }

    /// B From leaves 140839520/530/540/550 only write their payload and
    /// enum niche; preserve every other byte. Stair From 140839560 copies all
    /// twenty bytes, including its own niche, without inventing a tag.
    pub fn set_extra(&mut self, extra: DecoratorExtraInfo) {
        let mut raw = self.extra_bytes();
        match extra {
            DecoratorExtraInfo::Stair(v) => raw = v.0,
            DecoratorExtraInfo::Window(v) => {
                raw[..4].copy_from_slice(&v.0.to_le_bytes());
                raw[17] = 6;
            }
            DecoratorExtraInfo::Flag(v) => {
                raw[0] = v.0;
                raw[17] = 5;
            }
            DecoratorExtraInfo::Lantern(v) => {
                raw[..4].copy_from_slice(&v.0.to_le_bytes());
                raw[17] = 7;
            }
            DecoratorExtraInfo::Chimney(v) => {
                raw[..4].copy_from_slice(&v.f.to_le_bytes());
                raw[4] = v.smoke;
                raw[17] = 4;
            }
        }
        self.u30 = u64::from_le_bytes(raw[..8].try_into().unwrap());
        self.u38 = u32::from_le_bytes(raw[8..12].try_into().unwrap());
        self.u3c = u32::from_le_bytes(raw[12..16].try_into().unwrap());
        self.u40 = u32::from_le_bytes(raw[16..20].try_into().unwrap());
    }

    /// 目的墙 id（tag<=1 时 +0x00）。
    #[inline]
    pub fn dst_wall(self) -> WallId {
        WallId(self.dst_wall)
    }
}

/// B DecoratorOpEventKind：装饰操作事件种类（0x88B 事件记录的判别视图）。
///
/// `id` 0x140833570（fresh disasm，2026-10-17）：
/// ```
/// MOV RAX,[RCX]            ; 读事件记录判别 @+0
/// LEA RDX,[RAX-3]; ADD RAX,-2; XOR R8D,R8D; CMP RDX,2; CMOVC R8,RAX
///                          ; disc∈{3,4} → R8=disc-2 ∈ {1,2}，其余 → R8=0
/// LEA RAX,[0x142a92e78]; MOV RAX,[RAX+R8*8]   ; 跳表 {0x68,0x40,0x60}
/// MOV EAX,[RCX+RAX]; RET  ; 读 [RCX + offset] 的 u32
/// ```
/// 跳表实测（`read_memory 0x142a92e78`）= {0x68, 0x40, 0x60}：
/// disc==3 → +0x40；disc==4 → +0x60；其余 → +0x68。即从事件记录按判别
/// 取不同字段偏移的 u32，**非** 固定变体→0/1/2。旧 Place/Remove/Change 三变体
/// 近似已弃（载荷待取证，U：disc 0/1/2 的变体名/语义未单解）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecoratorOpEventKind(pub [u8; 0x88]);

/// Original EventWriter consumes this same136B payload; keep a single event
/// identity rather than copying it into a second host-only wrapper.
pub use DecoratorOpEventKind as DecoratorOpEvent;

#[cfg(test)]
mod canonical_event_owner_tests {
    use super::*;

    #[test]
    fn real_event_counter_survives_rotation_and_the_sentinel_discards_the_remaining_batch() {
        let queue = DeferredDecoratorOpEvents::default();
        let mut events = bevy_ecs::event::Events::<DecoratorOpEvent>::default();
        let payload = |tag: u64, id: u32| {
            let mut raw = [0u8; 0x88];
            raw[..8].copy_from_slice(&tag.to_le_bytes());
            raw[0x68..0x6c].copy_from_slice(&id.to_le_bytes());
            raw
        };
        queue.send(payload(0, 10));
        queue.send(payload(5, 0));
        queue.send(payload(0, 99));
        queue.unleash_to_events(&mut events);
        let mut reader = events.get_cursor();
        assert_eq!(reader.read_with_id(&events).map(|(e, id)| (e.id(), id.id))
            .collect::<Vec<_>>(), [(10, 0)]);
        events.update();
        events.update();
        assert!(events.is_empty());
        queue.send(payload(0, 20));
        queue.unleash_to_events(&mut events);
        assert_eq!(reader.read_with_id(&events).map(|(e, id)| (e.id(), id.id))
            .collect::<Vec<_>>(), [(20, 1)]);
        queue.unleash_to_events(&mut events);
        assert_eq!(events.len(), 1, "drained and sentinel-tail events must not reappear");
    }
}

impl DecoratorOpEventKind {
    /// 0x140833570：事件判别 → 跳表偏移 → 返回该字段 u32。
    pub fn id(&self) -> u32 {
        let disc = u64::from_le_bytes(self.0[0..8].try_into().unwrap());
        let offset = match disc {
            3 => 0x40,
            4 => 0x60,
            _ => 0x68,
        };
        u32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}

/// B DeferredDecoratorOpEvents：帧末延迟装饰操作并发队列。
///
/// 事件记录 **0x88B=136B**（send 0x1408336b0 push；unleash 0x1408337c0 drain）。
/// 原实现为 futex 等待队列（send 争用时 park 1e9ns，唤醒全局等待队列
/// plRam00000001436308f0）；Rust 侧以 Mutex+Condvar 提供语义等价（推送/冲刷
/// 顺序一致）。载荷字段语义 U。
#[derive(bevy_ecs::resource::Resource, Debug, Default)]
pub struct DeferredDecoratorOpEvents {
    inner: Mutex<Vec<[u8; 0x88]>>,
    wake: Condvar,
}

impl Clone for DeferredDecoratorOpEvents {
    fn clone(&self) -> Self {
        // 原1408335a0在共享锁内克隆事件Vec，备份队列与活队列独立拥有载荷。
        DeferredDecoratorOpEvents {
            inner: Mutex::new(self.inner.lock().unwrap().clone()),
            wake: Condvar::new(),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        // 保留现有同步对象身份；独占目标资源时只覆盖锁内Vec并复用容量。
        self.inner.get_mut().unwrap().clone_from(&source.inner.lock().unwrap());
    }
}

#[cfg(test)]
mod backup_queue_tests {
    use super::*;
    #[test]
    fn backup_queue_clone_from_preserves_lock_owner_and_vector_capacity() {
        let source = DeferredDecoratorOpEvents::default();
        let mut target = DeferredDecoratorOpEvents::default();
        let mut event = [7; 0x88]; event[..8].copy_from_slice(&1u64.to_le_bytes());
        source.send(event);
        for _ in 0..64 { target.send([0; 0x88]); }
        let lock = &target.inner as *const _ as usize;
        let wake = &target.wake as *const _ as usize;
        let (pointer, capacity) = { let data = target.inner.lock().unwrap(); (data.as_ptr() as usize, data.capacity()) };
        target.clone_from(&source);
        assert_eq!(&target.inner as *const _ as usize, lock);
        assert_eq!(&target.wake as *const _ as usize, wake);
        let data = target.inner.lock().unwrap();
        assert_eq!(data.as_ptr() as usize, pointer); assert_eq!(data.capacity(), capacity);
        assert_eq!(&*data, &[event]); drop(data);
        source.send([9; 0x88]);
        assert_eq!(target.with_events(|events| events.len()), 1);
        assert_eq!(target.with_events(|events| events[0]), event);
        assert_eq!(source.with_events(|events| events.len()), 2);
    }
}

impl DeferredDecoratorOpEvents {
    /// 原备份与楼梯继续消费在锁内借用完整队列，不排空、不复制0x88B载荷。
    pub fn with_events<R>(&self, read: impl FnOnce(&[[u8; 0x88]]) -> R) -> R {
        read(&self.inner.lock().unwrap())
    }
    /// 0x14067dd80（O）：恒等返回（共享内部队列句柄）。
    pub fn events(&self) -> &Self {
        self
    }

    /// 0x1408336b0（O）：推送一条 0x88B 事件记录。原 send 在标记被占用时
    /// park 等待（1e9ns 超时）；Mutex 阻塞即等价。扩容 panic 文件
    /// decorator.rs（0x142a92948）。
    pub fn send(&self, event: [u8; 0x88]) {
        let mut g = self.inner.lock().unwrap();
        g.push(event);
        self.wake.notify_all();
    }

    /// 0x1408337c0（O，fresh 2026-10-17）：逐条 drain 待发事件到 out。
    ///
    /// fresh 结构：
    /// 1. 队列 len 清零（`*(inner+0x18)=0`）后遍历旧元素；
    /// 2. 每条读 tag@+0（u64）；**tag==5 哨兵 → 提前 return**（剩余已清空元素
    ///    不再输出）；
    /// 3. 其余元素拷贝为 **0x90B 输出记录**：tag@+0 + 0x80B 载荷 @+8..+0x88 +
    ///    计数器 @+0x88（`out+0x40` 递增的下一事件号，BeVy Events 缓冲区语义）。
    ///
    /// 载荷字段（16 qword @+8..+0x88）由 send 方填写，本函数原样搬运；
    /// 计数器在 Rust 侧以 `out.len()` 为当前序号（无 BeVy Events 结构，登记 U）。
    pub fn unleash(&self, out: &mut Vec<[u8; 0x90]>) {
        self.drain_events(|event| {
            let mut record = [0u8; 0x90];
            record[..0x88].copy_from_slice(&event);
            record[0x88..0x90].copy_from_slice(&(out.len() as u64).to_le_bytes());
            out.push(record);
        });
    }

    /// B @1408337c0/511B writes the real Bevy event counter, not the current
    /// retained buffer length. The dependency owns IDs across rotations.
    pub fn unleash_to_events(&self, out: &mut bevy_ecs::event::Events<DecoratorOpEvent>) {
        self.drain_events(|event| { out.send(DecoratorOpEventKind(event)); });
    }

    fn drain_events(&self, mut emit: impl FnMut([u8; 0x88])) {
        let mut g = self.inner.lock().unwrap();
        let drained = std::mem::take(&mut *g);
        drop(g);
        for event in drained {
            let tag = u64::from_le_bytes(event[0..8].try_into().unwrap());
            if tag == 5 {
                // 哨兵：后续已清空元素不再输出。
                return;
            }
            emit(event);
        }
    }

    /// 挂起至队列非空（帧末等待装饰操作到达）。
    pub fn wait_until_pending(&self) {
        let mut g = self.inner.lock().unwrap();
        while g.is_empty() {
            g = self.wake.wait(g).unwrap();
        }
    }
}

/// B 0x1408337c0: drain the authoritative deferred resource into the canonical
/// EventWriter. Reuse the owner's existing drain/sentinel implementation.
pub fn unleash_decorator_op_events(
    mut deferred: bevy_ecs::prelude::ResMut<DeferredDecoratorOpEvents>,
    mut writer: bevy_ecs::event::EventWriter<DecoratorOpEvent>,
) {
    // Original mutable resource access updates its change tick even if empty.
    let deferred = &mut *deferred;
    deferred.drain_events(|event| { writer.write(DecoratorOpEvent(event)); });
}

/// B DecoratorRotationXZ：XZ 平面旋转（角度编码，O，2026-10-17 fresh disasm）。
///
/// - `from_xz_forward_vector` 0x140833a00（O）：`MOVAPS XMM2,[0x1429258f0]`
///   （符号掩码 0x80000000×4）+ `XORPS XMM0/XMM1`（**取负 x、y 两分量**）→
///   `JMP atan2`——即 `atan2(-x, -y)`，非既有 `atan2(x,y)`。
/// - `from_quat` 0x140833a20（O）：`atan2(2(wy+zx), w²+z²-x²-y²)`（通用公式）。
/// - `to_quat` 0x140833ac0（O）：(x=0, **y=sin(a/2)**, z=0, **w=cos(a/2)`)——
///   非既有 (y=cos, w=sin)。
/// - `to_projective_xz_mat2` 0x140833b20（O）：列 = {cos, sin} / {-sin, cos}——
///   即旋转阵 [[cos,-sin],[sin,cos]]，非既有 {sin,-cos}/{cos,sin}。
/// - `offset` 0x140833b80（O）：`fmodf(a+amount, 6.2831855f)`（2π 回绕）。
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecoratorRotationXZ(pub f32);

impl DecoratorRotationXZ {
    /// 行 883（内联，I）：XZ 前向向量。
    pub fn to_xz_forward_vector(self) -> Vec3 {
        Vec3::new(self.0.sin(), 0.0, self.0.cos())
    }

    /// 0x140833a00（O）：`atan2f(-x, -y)`（fresh disasm 符号掩码取负两分量后
    /// 转 atan2）。
    pub fn from_xz_forward_vector(v: Vec2) -> Self {
        Self((-v.x).atan2(-v.y))
    }

    /// B140833a20先完整旋转NEG_Z，再把(-x,-z)送到atan2f；不能代数约简。
    /// 非单位四元数、signed-zero与NaN仍保留原乘加顺序，不归一化。
    pub fn from_quat(q: Quat) -> Self {
        let forward = Self::negative_z_from_quat(q);
        Self((-forward.x).atan2(-forward.z))
    }

    /// 原同一SSE内联主体，供Intent的支持探针复用；不是新的旋转策略。
    pub fn negative_z_from_quat(q: Quat) -> Vec3 {
        let length_sq = q.z * q.z + (q.y * q.y + q.x * q.x);
        let dot = q.z * -1.0 + (q.y * -0.0 + q.x * -0.0);
        let twice_dot = dot + dot;
        let twice_w = q.w + q.w;
        Vec3::new(
            twice_w * (q.y * -1.0 - q.z * -0.0)
                + (twice_dot * q.x + (q.w * q.w - length_sq) * -0.0),
            twice_w * (q.z * -0.0 - q.x * -1.0)
                + (twice_dot * q.y + (q.w * q.w - length_sq) * 0.0),
            twice_w * (q.x * -0.0 - q.y * -0.0)
                + (twice_dot * q.z + (q.w * q.w - length_sq) * -1.0),
        )
    }

    /// 0x140833ac0（O）：(x=0, y=sin(a·0.5), z=0, w=cos(a·0.5))。
    pub fn to_quat(self) -> Quat {
        let a = self.0 * 0.5;
        Quat::from_xyzw(0.0, a.sin(), 0.0, a.cos())
    }

    /// 0x140833b20（O）：旋转矩阵列 {cos, sin} / {-sin, cos}（[[cos,-sin],[sin,cos]]）。
    pub fn to_projective_xz_mat2(self) -> Mat2 {
        Mat2::from_cols(
            Vec2::new(self.0.cos(), self.0.sin()),
            Vec2::new(-self.0.sin(), self.0.cos()),
        )
    }

    /// 0x140833b80（O）：`fmod(a+amount, 2π)`（C fmod 语义，可负）。
    pub fn offset(self, amount: f32) -> Self {
        Self((self.0 + amount) % std::f32::consts::TAU)
    }
}

/// B is_default 自由谓词（行 602，内联）——用于装饰记录的默认判定。
pub fn is_default(info: &DecoratorInfo) -> bool {
    info.is_default()
}

/// 非业务辅助：让 Arc 依赖可见（DeferredDecoratorOpEvents 的 futex 等待队列
/// 在原实现为进程级全局；此处保留 Arc 导入以登记该共享语义）。
#[allow(dead_code)]
fn _shared_queue_dependency() -> Option<Arc<()>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_extra_keeps_the_canonical_payload_and_does_not_mutate_info() {
        let mut info = destination_sentinel();
        // 目的地、框颜色和u30高位均故意使用不同值，防止误读相邻union。
        info.dst_payload = 0x0606_0606_a5a5_a5a5;
        info.u28 = 0x0123_4567;
        info.u40 = (info.u40 & !0xff00) | (6 << 8);
        for payload in [0, 0x8000_0000, 0xdead_beef, u32::MAX] {
            info.u30 = 0x7654_3210_0000_0000 | u64::from(payload);
            let before = info;
            assert_eq!(info.window_extra(), WindowExtra(payload));
            assert_eq!(info, before);
        }
    }

    #[test]
    fn window_extra_rejects_other_variants_even_with_window_like_payloads() {
        for style in 0u8..=u8::MAX {
            if style == 6 { continue; }
            let info = DecoratorInfo {
                dst_payload: 6 << 8,
                u30: 0x0606_0606_0000_1234,
                // 相邻三字节均为6，只有真正的Info+0x41保存被测判别。
                u40: 0x0606_0006 | (u32::from(style) << 8),
                ..DecoratorInfo::default()
            };
            assert!(std::panic::catch_unwind(|| info.window_extra()).is_err(),
                "非窗判别{style:#x}必须保留原required失败");
        }
    }

    #[test]
    fn decorator_type_predicate_truth_tables() {
        // 全 u8 值域按原机器码位运算核对（章节"→ t∈{}"为类型值域内的意图，
        // 原始位运算对全 u8 匹配更多值）。
        for t in 0u8..=0xff {
            let ty = DecoratorType(t);
            assert_eq!(
                ty.max_rank(),
                if t.wrapping_add(0xc8) < 3 { 3 } else { 1 },
                "max_rank t={:#x}",
                t
            );
            assert_eq!(
                ty.can_change_seed(),
                (t.wrapping_add(0xc8) < 8) && ((0x27u8 >> (t & 7)) & 1) != 0,
                "can_change_seed t={:#x}",
                t
            );
            assert_eq!(
                ty.is_window(),
                t.wrapping_add(0xc8) < 3,
                "is_window t={:#x}",
                t
            );
            assert_eq!(
                ty.spawns_clutter(),
                (t & 0x3e) == 0x38,
                "spawns_clutter t={:#x}",
                t
            );
            assert_eq!(
                ty.can_place_on_roof(),
                t != 0x3a,
                "can_place_on_roof t={:#x}",
                t
            );
            assert_eq!(
                ty.can_place_on_terrain(),
                (t & 0x3c) != 0x38,
                "can_place_on_terrain t={:#x}",
                t
            );
            assert_eq!(
                ty.can_place_on_stair_top(),
                (t.wrapping_add(0xc4) < 3) || (t & 0x38) != 0x38,
                "can_place_on_stair_top t={:#x}",
                t
            );
            assert_eq!(
                ty.should_offer_snap_to_corner(),
                t != 0x3f,
                "snap_to_corner t={:#x}",
                t
            );
        }
        // 类型值域 {0x38..0x3f} 内：窗族 {0x38..0x3b} 不可放楼梯顶；{0x3c,0x3d,0x3e}
        // 可放；0x3f（角吸附）同样不可放（RNE 手解位运算证实，章节"仅窗族"为近似）。
        for t in 0x38u8..=0x3b {
            assert!(
                !DecoratorType(t).can_place_on_stair_top(),
                "window t={:#x} on stair top",
                t
            );
            assert!(
                !DecoratorType(t).can_place_on_terrain(),
                "window t={:#x} on terrain",
                t
            );
        }
        for t in [0x3cu8, 0x3d, 0x3e] {
            assert!(
                DecoratorType(t).can_place_on_stair_top(),
                "stair-top t={:#x}",
                t
            );
        }
        assert!(!DecoratorType(0x3f).can_place_on_stair_top());
        assert!(DecoratorType(0x38).spawns_clutter());
        assert!(DecoratorType(0x39).spawns_clutter());
    }

    #[test]
    fn decorator_info_accessors() {
        let mut info = DecoratorInfo {
            dst_wall: 42,
            u20: 3 << 16,
            decorator_id: 9,
            rank: 7,
            ty: 0x39,
            state: 2,
            ..DecoratorInfo::default()
        };
        assert_eq!(info.dst_tag(), 3); // Terrain
        assert!(info.try_dst_as_wall().is_none());
        assert!(info.dst_as_terrain().dst_wall == 42);
        assert_eq!(info.rank(), 7);
        assert_eq!(info.decorator_type(), DecoratorType(0x39));
        assert_eq!(info.decorator_id(), DecoratorId(9));
        assert!(!info.is_default());

        info.u20 = 0; // Wall tag 0
        assert!(info.dst_as_wall().dst_wall == 42);
        assert!(info.try_dst_as_wall().is_some());
    }

    #[test]
    fn decorator_dst_is_wall_and_tags() {
        let wall = destination_wall_fixture();
        let mut dst = DecoratorDst::Wall(wall);
        assert!(dst.is_wall());
        assert_eq!(*dst.as_wall(), wall);
        dst.as_wall_mut().coord.y = 9.0;
        assert_eq!(dst.as_wall().coord.y, 9.0);
        assert!(DecoratorDst::WallCompound(wall).is_wall());
        let roof = DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::new(0.25, 0.8),
            yaw: DecoratorRotationXZ(0.7),
        });
        assert!(!roof.is_wall());
        assert_eq!(roof.tag(), 2);
        assert_eq!(roof.as_roof().coord, Vec2::new(0.25, 0.8));
        assert!(std::panic::catch_unwind(|| roof.as_wall()).is_err());
        assert!(std::panic::catch_unwind(|| dst.as_roof()).is_err());
        // The authority wall belongs to the storage address. Destination
        // coordinates and optional platform must not become that owner.
        let storage = super::super::decorator_storage::DecoratorStorageAddr::Wall(WallId(77));
        assert_eq!(storage.try_as_wall(), Some(WallId(77)));
        assert_ne!(storage.try_as_wall(), Some(wall.platform));
    }

    fn destination_wall_fixture() -> WallDecoratorDst {
        WallDecoratorDst {
            coord: WallSpaceCoord::new_unchecked(1.25, -2.5),
            platform: WallId(0x0002_0003_0004_0005),
            relative_y: 0.5,
            position: Vec3::new(1.0, 2.0, 3.0),
            corner_attachment: 3,
            bottom_as_door: true,
        }
    }

    fn destination_sentinel() -> DecoratorInfo {
        DecoratorInfo {
            dst_wall: 0x1716_1514_1312_1110,
            dst_secondary: 0x1f1e_1d1c_1b1a_1918,
            dst_payload: 0x2726_2524_2322_2120,
            u18: 0x2f2e_2d2c_2b2a_2928,
            u20: 0x3332_3130,
            u24: 0x3736_3534,
            u28: 0x3b3a_3938,
            decorator_id: 0x3f3e_3d3c,
            u30: 0x4746_4544_4342_4140,
            u38: 0x4b4a_4948,
            u3c: 0x4f4e_4d4c,
            u40: 0x5352_5150,
            rank: 0x54,
            u45: 0x55,
            ty: 0x56,
            state: 0x57,
        }
    }

    fn destination_info_bytes(info: &DecoratorInfo) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(0x48);
        for word in [info.dst_wall, info.dst_secondary, info.dst_payload, info.u18] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        for word in [info.u20, info.u24, info.u28, info.decorator_id] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes.extend_from_slice(&info.u30.to_le_bytes());
        for word in [info.u38, info.u3c, info.u40] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes.extend_from_slice(&[info.rank, info.u45, info.ty, info.state]);
        bytes
    }

    fn assert_destination_bytes(destination: DecoratorDst, writes: &[(usize, &[u8])]) {
        let mut info = destination_sentinel();
        let mut expected = destination_info_bytes(&info);
        for &(offset, bytes) in writes {
            expected[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
        info.set_destination(destination);
        assert_eq!(destination_info_bytes(&info), expected);
        // Decode/encode must preserve payload bits, including nonfinite yaw
        // and signed zero; PartialEq would not check the NaN roundtrip.
        let decoded = info.destination();
        let mut roundtrip = destination_sentinel();
        roundtrip.set_destination(decoded);
        assert_eq!(destination_info_bytes(&roundtrip), expected);
    }

    #[test]
    fn destination_wall_codec_preserves_defined_bytes_and_all_other_info() {
        let bytes = [
            0, 0, 0xa0, 0x3f, 0, 0, 0x20, 0xc0,
            5, 0, 4, 0, 3, 0, 2, 0,
            0, 0, 0, 0x3f, 0, 0, 0x80, 0x3f,
            0, 0, 0, 0x40, 0, 0, 0x40, 0x40,
        ];
        assert_destination_bytes(
            DecoratorDst::Wall(destination_wall_fixture()),
            &[(0, &bytes), (0x20, &[3, 1, 0])],
        );
        let mut compound = destination_wall_fixture();
        compound.bottom_as_door = false;
        assert_destination_bytes(
            DecoratorDst::WallCompound(compound),
            &[(0, &bytes), (0x20, &[3, 0, 1])],
        );
    }

    #[test]
    fn destination_roof_and_terrain_codec_preserves_yaw_bits_and_padding() {
        assert_destination_bytes(
            DecoratorDst::Roof(RoofDecoratorDst {
                coord: Vec2::new(0.25, 0.8),
                yaw: DecoratorRotationXZ(f32::from_bits(0x7fc1_2345)),
            }),
            &[(0, &[0, 0, 0x80, 0x3e, 0xcd, 0xcc, 0x4c, 0x3f]),
              (8, &[0x45, 0x23, 0xc1, 0x7f]), (0x22, &[2])],
        );
        assert_destination_bytes(
            DecoratorDst::Terrain(TerrainDecoratorDst {
                position_xz: Vec2::new(-0.0, 1.0),
                yaw: DecoratorRotationXZ(-7.5),
            }),
            &[(0, &[0, 0, 0, 0x80, 0, 0, 0x80, 0x3f]),
              (8, &[0, 0, 0xf0, 0xc0]), (0x22, &[3])],
        );
    }

    #[test]
    fn destination_stair_codec_preserves_both_ids_coordinates_and_padding() {
        assert_destination_bytes(
            DecoratorDst::Stair(StairDecoratorDst {
                first_id: 0x0123_4567,
                second_id: 0x89ab_cdef,
                coordinate: Vec2::new(f32::from_bits(0x7fc1_2345), -0.0),
                yaw: DecoratorRotationXZ(-7.5),
            }),
            &[(0, &[0x67, 0x45, 0x23, 1, 0xef, 0xcd, 0xab, 0x89,
                     0x45, 0x23, 0xc1, 0x7f, 0, 0, 0, 0x80,
                     0, 0, 0xf0, 0xc0]), (0x22, &[4])],
        );
    }

    #[test]
    fn destination_decoder_rejects_malformed_bool_and_unknown_tag() {
        for tag in [0u8, 1] {
            for boolean in [2u8, 0xff] {
                let mut info = destination_sentinel();
                info.u20 = u32::from(tag) << 16 | u32::from(boolean) << 8;
                assert!(std::panic::catch_unwind(|| info.destination()).is_err());
            }
        }
        for tag in [5u8, 0xff] {
            let mut info = destination_sentinel();
            info.u20 = u32::from(tag) << 16;
            assert!(std::panic::catch_unwind(|| info.destination()).is_err());
        }
    }

    #[test]
    fn raw_wall_coordinate_accessor_keeps_original_default_tag_and_nan_domain() {
        let info = DecoratorInfo {
            dst_wall: 0x7fc1_2345_8000_0000,
            u20: 0xff << 16,
            ..DecoratorInfo::default()
        };
        let coordinate = info.wall_space_coord();
        assert_eq!(coordinate.u.to_bits(), 0x8000_0000);
        assert_eq!(coordinate.y.to_bits(), 0x7fc1_2345);
        assert_eq!(info.dst_tag(), 0xff);
    }

    #[test]
    fn decorator_extra_info_roundtrip() {
        let mut info = DecoratorInfo::default();
        info.set_extra(DecoratorExtraInfo::Flag(FlagExtra(0xab)));
        assert_eq!(info.extra_disc(), 5);
        assert_eq!(info.extra(), DecoratorExtraInfo::Flag(FlagExtra(0xab)));

        // 原生判别：烟囱=4（fresh 0x1408392d0 `cmp byte[+0x11],0x4`）。
        info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
            f: 1.5,
            smoke: 0xab,
        }));
        assert_eq!(info.extra_disc(), 4);
        assert_eq!(
            info.extra(),
            DecoratorExtraInfo::Chimney(ChimneyExtra {
                f: 1.5,
                smoke: 0xab
            })
        );

        // 原生判别：灯=7（fresh 0x140839420 `cmp,0x7`；载荷 f32）。
        info.set_extra(DecoratorExtraInfo::Lantern(LanternExtra(4660.0)));
        assert_eq!(info.extra_disc(), 7);
        assert_eq!(
            info.extra(),
            DecoratorExtraInfo::Lantern(LanternExtra(4660.0))
        );

        // 原生判别：窗=6（fresh 0x1408393b0 `cmp,0x6`；载荷 u32）。
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0x1234)));
        assert_eq!(info.extra_disc(), 6);
        assert_eq!(
            info.extra(),
            DecoratorExtraInfo::Window(WindowExtra(0x1234))
        );
    }

    #[test]
    fn decorator_preview_does_not_consume_committed_id() {
        let mut ids = DecoratorIdGen::default();
        assert_eq!(ids.peek(), DecoratorId(0));
        assert_eq!(ids.peek(), DecoratorId(0));
        assert_eq!(ids.generate(), DecoratorId(0));
        assert_eq!(ids.peek(), DecoratorId(1));
        ids.reserve(DecoratorId(u32::MAX));
        assert_eq!(ids.peek(), DecoratorId(0));
    }

    #[test]
    fn transient_id_gen_modes() {
        let mut base = 100u32;
        // 直写模式：返回旧值并递增共享基数。
        let mut id_gen = TransientDecoratorIdGen::new(&mut base, true);
        assert_eq!(id_gen.generate(), 100);
        assert_eq!(id_gen.generate(), 101);
        assert_eq!(base, 102);

        // 偏移模式：返回 base + 偏移，基数不变。
        let mut id_gen = TransientDecoratorIdGen::new(&mut base, false);
        assert_eq!(id_gen.generate(), 102);
        assert_eq!(id_gen.generate(), 103);
        assert_eq!(base, 102);
    }

    #[test]
    fn canonical_transient_generation_preserves_preview_and_failure_side_effects() {
        let mut ids = DecoratorIdGen { next: u32::MAX };
        {
            let mut preview = ids.transient(false);
            assert_eq!(preview.generate(), u32::MAX);
            assert_eq!(preview.generate(), 0);
        }
        assert_eq!(ids.peek(), DecoratorId(u32::MAX));
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut committed = ids.transient(true);
            assert_eq!(committed.generate(), u32::MAX);
            panic!("模拟原下游资源查找失败");
        }));
        assert!(failure.is_err());
        assert_eq!(ids.peek(), DecoratorId(0));
    }

    #[test]
    fn rotation_quat_roundtrip() {
        let rot = DecoratorRotationXZ(0.7);
        let q = rot.to_quat();
        let back = DecoratorRotationXZ::from_quat(q);
        let diff = (back.0 - rot.0).abs();
        assert!(diff < 1e-5, "rotation roundtrip diff {}", diff);
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 旗连接设置命令（O：`walls::decorator::SetFlagConnectionCmd`；color_menu
/// 0x142291230 的 EventWriter 面、set_flag_connection_and_chimney_smoke 0x142298970
/// 消费）。载荷 `{id u32@+8, value u8@+0xc, custom u8@+0xd}`（0x10B EventInstance，
/// RESTORE 纠错 #4；`repr(C)` 对齐 0x8B 载荷）。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetFlagConnectionCmd {
    /// 目标装饰 id（u32 @+8）。
    pub id: u32,
    /// 旗值（u8 @+0xc，写入 FlagExtra.0）。
    pub value: u8,
    /// 自定义标志（u8 @+0xd，`!= 0`，O）。
    pub custom: u8,
    /// +0xe..+0x10 对齐填充。
    pub _pad: [u8; 2],
}

/// 烟囱烟开关命令（O：`walls::decorator::SetChimneySmokeCmd`；同族消费）。
/// 载荷 `{id u32@+8, value u8@+0xc, custom u8@+0xd}`（0x10B；`value` 写入
/// ChimneyExtra+4，custom `== 1`，O）。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetChimneySmokeCmd {
    /// 目标装饰 id（u32 @+8）。
    pub id: u32,
    /// 烟值（u8 @+0xc，写入 ChimneyExtra+4）。
    pub value: u8,
    /// 自定义标志（u8 @+0xd，`== 1`，O）。
    pub custom: u8,
    /// +0xe..+0x10 对齐填充。
    pub _pad: [u8; 2],
}

pub mod anchors;

pub mod placement;

#[cfg(test)]
mod canonical_decorator_event_tests {
    use super::*;
    use bevy_ecs::{event::Events, prelude::{Schedule, World}};
    use crate::resources::history::decorator_edit::DecoratorDiffEvent;

    fn payload(tag: u64, fill: u8) -> [u8; 0x88] {
        let mut event = [fill; 0x88];
        event[..8].copy_from_slice(&tag.to_le_bytes());
        event
    }

    #[test]
    fn canonical_aliases_share_type_id_and_tuple_constructor() {
        use std::any::TypeId;
        assert_eq!(TypeId::of::<DecoratorOpEvent>(), TypeId::of::<DecoratorOpEventKind>());
        assert_eq!(TypeId::of::<DecoratorOpEvent>(), TypeId::of::<DecoratorDiffEvent>());
        assert_eq!(DecoratorOpEventKind([0; 0x88]), DecoratorDiffEvent([0; 0x88]));
        // Native id dispatch compares the full u64 tag, not its lower dword.
        let mut event = payload((1_u64 << 32) | 3, 0);
        event[0x40..0x44].copy_from_slice(&7_u32.to_le_bytes());
        event[0x68..0x6c].copy_from_slice(&9_u32.to_le_bytes());
        assert_eq!(DecoratorDiffEvent(event).decorator_id(), 9);
    }

    #[test]
    fn actual_world_drain_preserves_payloads_and_bevy_ids_across_frames() {
        let mut world = World::new();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<Events<DecoratorOpEvent>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(unleash_decorator_op_events);
        let a = payload(3, 0xa1);
        let b = payload(4, 0xb2);
        let c = payload(2, 0xc3);
        world.resource::<DeferredDecoratorOpEvents>().send(a);
        world.resource::<DeferredDecoratorOpEvents>().send(b);
        schedule.run(&mut world);
        let mut cursor = world.resource::<Events<DecoratorOpEvent>>().get_cursor();
        let first: Vec<_> = cursor.read_with_id(world.resource::<Events<DecoratorOpEvent>>())
            .map(|(event, id)| (event.0, id.id)).collect();
        assert_eq!(first, vec![(a, 0), (b, 1)]);
        schedule.run(&mut world);
        assert_eq!(cursor.read(world.resource::<Events<DecoratorOpEvent>>()).count(), 0);
        world.resource_mut::<Events<DecoratorOpEvent>>().update();
        world.resource::<DeferredDecoratorOpEvents>().send(c);
        schedule.run(&mut world);
        let next: Vec<_> = cursor.read_with_id(world.resource::<Events<DecoratorOpEvent>>())
            .map(|(event, id)| (event.0, id.id)).collect();
        assert_eq!(next, vec![(c, 2)]);
    }

    #[test]
    fn empty_unleash_still_marks_the_deferred_resource_changed() {
        use bevy_ecs::change_detection::DetectChanges;
        let mut world = World::new();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<Events<DecoratorOpEvent>>();
        world.clear_trackers();
        let mut schedule = Schedule::default();
        schedule.add_systems(unleash_decorator_op_events);
        schedule.run(&mut world);
        assert!(world.get_resource_ref::<DeferredDecoratorOpEvents>().unwrap().is_changed());
        assert!(world.resource::<Events<DecoratorOpEvent>>().is_empty());
    }

    #[test]
    fn existing_history_diff_producer_reaches_the_canonical_stream() {
        use crate::resources::history::decorator_edit::{
            send_state_diff_events, DecoratorBeforeAfterWithStorage,
            DecoratorHistoryState, DecoratorStateWithStorage,
        };
        let mut world = World::new();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<Events<DecoratorOpEvent>>();
        let after = DecoratorStateWithStorage {
            storage_disc: 1, storage_payload: 0,
            state: DecoratorHistoryState { ty: 0x3c, ..Default::default() },
        };
        let diffs = std::collections::HashMap::from([(
            DecoratorId(71), DecoratorBeforeAfterWithStorage {
                before: DecoratorStateWithStorage { storage_disc: 3, ..Default::default() },
                after,
            },
        )]);
        send_state_diff_events(&diffs, world.resource::<DeferredDecoratorOpEvents>());
        let mut schedule = Schedule::default();
        schedule.add_systems(unleash_decorator_op_events);
        schedule.run(&mut world);
        let events = world.resource::<Events<DecoratorOpEvent>>();
        let actual: Vec<_> = events.get_cursor().read(events).copied().collect();
        assert_eq!(actual, vec![DecoratorDiffEvent::from_added(DecoratorId(71), &after)]);
        assert_eq!(actual[0].decorator_id(), 71);
    }

    #[test]
    fn sentinel_discards_current_remainder_and_allows_later_batch() {
        let mut world = World::new();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<Events<DecoratorOpEvent>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(unleash_decorator_op_events);
        let a = payload(1, 10);
        let b = payload(0, 20);
        for value in [a, payload(5, 0), payload(3, 30)] {
            world.resource::<DeferredDecoratorOpEvents>().send(value);
        }
        schedule.run(&mut world);
        let mut cursor = world.resource::<Events<DecoratorOpEvent>>().get_cursor();
        let first: Vec<_> = cursor.read(world.resource::<Events<DecoratorOpEvent>>()).map(|e| e.0).collect();
        assert_eq!(first, vec![a]);
        world.resource::<DeferredDecoratorOpEvents>().send(b);
        schedule.run(&mut world);
        let next: Vec<_> = cursor.read(world.resource::<Events<DecoratorOpEvent>>()).map(|e| e.0).collect();
        assert_eq!(next, vec![b]);
    }
}

/// All four native tags, directly executed through GCC serializer14085ee90.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StairRailingType {
    #[default]
    None=0,
    Low=1,
    High=2,
    Wooden=3,
}
impl StairRailingType {
    pub fn native_tag(self)->u8 { self as u8 }
}
impl TryFrom<u8> for StairRailingType {
    type Error=u8;
    fn try_from(value:u8)->Result<Self,u8> {
        match value {0=>Ok(Self::None),1=>Ok(Self::Low),2=>Ok(Self::High),3=>Ok(Self::Wooden),_=>Err(value)}
    }
}

#[cfg(test)]
mod m01_original_quaternion_angle_tests {
    use super::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-decorator-quat-atan-native-vectors.rs"));

    #[test]
    fn original_complete_quaternion_atan_inputs_preserve_finite_bits_and_nonfinite_class() {
        for &(bits,expected) in DECORATOR_QUAT_ATAN_NATIVE {
            let forward=DecoratorRotationXZ::negative_z_from_quat(Quat::from_array(bits.map(f32::from_bits)));
            let actual=[(-forward.x).to_bits(),(-forward.z).to_bits()];
            if bits.iter().all(|x| f32::from_bits(*x).is_finite()) {
                assert_eq!(actual,expected,"原有限四元数{bits:x?}");
            } else {
                // 原入口先断言所有分量有限；孤立数学helper仍可传播NaN。
                // ARM/x86对Inf*0所产NaN符号不同，不伪称此无效输入位型跨平台一致。
                assert_eq!(actual.map(|x|f32::from_bits(x).is_nan()),expected.map(|x|f32::from_bits(x).is_nan()));
            }
            // 此测试的证据边界是Windows atan2f之前的两个输入，未伪称CRT最终角度逐位。
        }
    }
}
