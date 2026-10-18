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
/// 结构：`{base:*mut u32@+0, +0x8:u64 偏移游标, +0x10:bool 直写模式}`。
/// `generate`：直写模式（flag=1）→ `*base += 1` 返回旧值；否则 `+0x8 += 1`，
/// 返回 `*base + 旧偏移`。base 为外部共享计数器指针（U 待取证：调用方所有权）。
pub struct TransientDecoratorIdGen {
    /// +0x00：共享基数指针（外部所有权）。
    base: *mut u32,
    /// +0x08：偏移游标。
    offset: u64,
    /// +0x10：直写模式。
    direct_write: bool,
}

// SAFETY: 原 struct 不携带 Sync 保证（内部裸指针只经 TransientDecoratorIdGen 方法使用）。
// Rust 中裸指针默认 !Send/!Sync，保持与所有权未定（U）一致，不额外标注。

impl TransientDecoratorIdGen {
    /// 0x1408339c0（I：构造器初值；游标/模式初值待取证）。
    pub fn new(base: *mut u32, direct_write: bool) -> Self {
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
            // base 非空为调用方合同；空指针保持原崩溃语义。
            let slot = unsafe { &mut *self.base };
            let old = *slot;
            *slot = old.wrapping_add(1);
            old
        } else {
            let old = self.offset;
            self.offset = old.wrapping_add(1);
            let base = unsafe { *self.base };
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

/// B DecoratorLocator：装饰定位（0x140833240 = new，I；字段布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorLocator {
    /// I：目的存储地址（U：完整字段表待取证）。
    pub addr: super::decorator_storage::DecoratorStorageAddr,
}

impl DecoratorLocator {
    /// 0x140833240（I）。
    pub fn new(addr: super::decorator_storage::DecoratorStorageAddr) -> Self {
        Self { addr }
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
            DecoratorExtraInfo::Stair(_) => 0,
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

/// B DecoratorDst：目的地联合，判别字节位于 +0x22。
/// - tag 0/1：墙（`is_wall` 0x1408334d0 = `tag<2`；墙 id 载荷位于 +0x00）
/// - tag 2：屋顶（`as_roof` 0x1408334e0 判别 ==2）
/// - tag 3：地形（`dst_as_terrain` 0x140833410 判别 ==3）
/// - tag 4：楼梯（`dst_as_stair` 0x140833470，I）
///
/// tag 0 与 tag 1 的差异（原子/复合墙）待取证（U）。WallId 未实现 serde，
/// 故本枚举不派生序列化。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecoratorDst {
    /// tag 0：墙目的地。
    Wall(WallId),
    /// tag 1：墙目的地（复合墙，同存储桶；差异 U）。
    WallCompound(WallId),
    /// tag 2：屋顶目的地。
    Roof,
    /// tag 3：地形目的地。
    Terrain,
    /// tag 4：楼梯目的地。
    Stair,
}

impl DecoratorDst {
    /// 判别字节 +0x22 值。
    pub fn tag(self) -> u8 {
        match self {
            DecoratorDst::Wall(_) => 0,
            DecoratorDst::WallCompound(_) => 1,
            DecoratorDst::Roof => 2,
            DecoratorDst::Terrain => 3,
            DecoratorDst::Stair => 4,
        }
    }

    /// 0x1408334d0（O）：`tag<2`。
    pub fn is_wall(self) -> bool {
        self.tag() < 2
    }

    /// 0x1408334e0（O）：tag==2 → &self，否则 panic（行 0x28）。
    pub fn as_roof(&self) -> &Self {
        if matches!(self, DecoratorDst::Roof) {
            self
        } else {
            panic!("DecoratorDst::as_roof: not a roof destination")
        }
    }

    /// 0x140833510（O）：tag<2 → &self，否则 panic。
    pub fn as_wall(&self) -> &Self {
        if self.is_wall() {
            self
        } else {
            panic!("DecoratorDst::as_wall: not a wall destination")
        }
    }

    /// 0x140833540（O）：tag<2 → &mut self，否则 unreachable。
    pub fn as_wall_mut(&mut self) -> &mut Self {
        if self.is_wall() {
            self
        } else {
            unreachable!("DecoratorDst::as_wall_mut: not a wall destination")
        }
    }

    /// 墙 id 载荷（tag 0/1）。I：复合墙载荷语义待取证。
    pub fn wall_id(self) -> WallId {
        match self {
            DecoratorDst::Wall(id) | DecoratorDst::WallCompound(id) => id,
            _ => panic!("DecoratorDst::wall_id: not a wall destination"),
        }
    }
}

/// B DecoratorInfo：72B（0x48）权威装饰记录。
///
/// 布局（apply_decorator_place 0x140a20c70 读面 + decorator.rs 判别访问，O；
/// 2026-10-16 fresh 反编译复核）：`+0x00/+0x08/+0x10 u64` 目的、`+0x22` dst tag、
/// `+0x2c` DecoratorId、`+0x44` rank、`+0x46` DecoratorType、`+0x47` 状态字节
/// （insert 0x14155afe0 写 2=live/placed）。**+0x48/+0x4c/+0x50/+0x55/+0x56 属于
/// 包裹记录（op 事件），非 DecoratorInfo**——+0x4c 是插入键、+0x2c 是记录内
/// DecoratorId，二者并存（fresh 0x14155afe0）。E2 取证（2026-10-16，fresh
/// 0x140a20670）：移除拷贝 0x48B = {8×u64@+0x00..0x40, state u8@+0x47}，与
/// `+0x44..0x47` rank/state 簇一致（rank@+0x44、**+0x45 1 字节 U**、type@+0x46、
/// state@+0x47）。完整逐字段语义表仍 U（RE-DECORATOR-STORAGE §6.1）；未知字段
/// 用 raw 名，不编造业务含义。**无 bevy Entity 字段**——实体由消费系统读
/// DecoratorInfo 后另行生成，关联靠 DecoratorId 组件（RE §2 实体机制）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorInfo {
    /// +0x00：目的墙 id（WallId），dst tag<=1 有效。
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
    /// +0x2c：DecoratorId。
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
    /// +0x47：状态字节（fresh 0x14155afe0：insert 写入 2=live/placed；U：状态语义
    /// 待取证）。
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

    /// +0x22：目的 tag。
    #[inline]
    pub fn dst_tag(self) -> u8 {
        ((self.u20 >> 16) & 0xff) as u8
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

    /// +0x47：状态字节（2 = live/placed，fresh 0x14155afe0 O）。
    #[inline]
    pub fn state(self) -> u8 {
        self.state
    }

    /// +0x2c：DecoratorId。
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

    /// +0x11：DecoratorExtraInfo 判别字节。
    ///
    /// 原二进制判别位于 DecoratorInfo+0x11（= dst_payload 字节 1），与 f32/u32
    /// 载荷字节 1 重叠（From 变体先写载荷后写判别，存储即破坏载荷字节 1）。
    /// ExtraInfo 完整字节布局 U（RE §6.1）；Rust 侧将判别存于 dst_payload
    /// 高位字节，使载荷无损往返。偏离已登记。
    #[inline]
    pub fn extra_disc(self) -> u8 {
        ((self.dst_payload >> 56) & 0xff) as u8
    }

    /// 解码 DecoratorExtraInfo（判别存于 dst_payload 高位字节；载荷 +0x10..）。
    /// 楼梯 20B 载荷跨 dst_payload(+0x10..0x18) 与 u18(+0x18..0x20)、u20(+0x20..0x24)。
    /// 判别值按 fresh 0x1408392d0/340/3b0/420/4b0：烟囱=4、旗=5、窗=6、灯=7。
    pub fn extra(self) -> DecoratorExtraInfo {
        match self.extra_disc() {
            0 | 1 => {
                let mut raw = [0u8; 20];
                raw[0..8].copy_from_slice(&self.dst_payload.to_le_bytes());
                raw[8..16].copy_from_slice(&self.u18.to_le_bytes());
                raw[16..20].copy_from_slice(&self.u20.to_le_bytes()[..4]);
                DecoratorExtraInfo::Stair(StairExtra(raw))
            }
            4 => {
                let bytes = self.dst_payload.to_le_bytes();
                DecoratorExtraInfo::Chimney(ChimneyExtra {
                    f: f32::from_le_bytes(bytes[0..4].try_into().unwrap()),
                    smoke: bytes[4],
                })
            }
            5 => {
                let bytes = self.dst_payload.to_le_bytes();
                DecoratorExtraInfo::Flag(FlagExtra(bytes[0]))
            }
            6 => {
                let bytes = self.dst_payload.to_le_bytes();
                DecoratorExtraInfo::Window(WindowExtra(u32::from_le_bytes(
                    bytes[0..4].try_into().unwrap(),
                )))
            }
            7 => {
                let bytes = self.dst_payload.to_le_bytes();
                DecoratorExtraInfo::Lantern(LanternExtra(f32::from_le_bytes(
                    bytes[0..4].try_into().unwrap(),
                )))
            }
            d => {
                // U：未知判别（含 2/3）保持不识别。
                debug_assert!(false, "unknown DecoratorExtraInfo discriminator {:#x}", d);
                DecoratorExtraInfo::Stair(StairExtra([0; 20]))
            }
        }
    }

    /// 写 DecoratorExtraInfo（判别存于 dst_payload 高位字节；载荷 @+0x10..）。
    /// 楼梯 20B 载荷跨 dst_payload(+0x10..0x18)、u18(+0x18..0x20)、u20(+0x20..0x24)；
    /// 判别字节 7（+0x17）与楼梯载荷字节 7 重叠（原二进制判别 @+0x11 同样嵌入
    /// 载荷，属忠实行为，偏离仅移位）。
    pub fn set_extra(&mut self, extra: DecoratorExtraInfo) {
        let disc = match extra {
            DecoratorExtraInfo::Stair(_) => 0u8,
            DecoratorExtraInfo::Window(_) => 6,
            DecoratorExtraInfo::Flag(_) => 5,
            DecoratorExtraInfo::Lantern(_) => 7,
            DecoratorExtraInfo::Chimney(_) => 4,
        };
        let mut payload = [0u8; 8];
        match extra {
            DecoratorExtraInfo::Stair(v) => {
                // 20B 载荷：dst_payload[0..8]、u18[0..8]、u20[0..4]。
                payload.copy_from_slice(&v.0[0..8]);
                let mut u18 = [0u8; 8];
                u18.copy_from_slice(&v.0[8..16]);
                let mut u20 = [0u8; 4];
                u20.copy_from_slice(&v.0[16..20]);
                self.u18 = u64::from_le_bytes(u18);
                self.u20 = u32::from_le_bytes(u20);
            }
            DecoratorExtraInfo::Window(v) => payload[0..4].copy_from_slice(&v.0.to_le_bytes()),
            DecoratorExtraInfo::Flag(v) => payload[0] = v.0,
            DecoratorExtraInfo::Lantern(v) => payload[0..4].copy_from_slice(&v.0.to_le_bytes()),
            DecoratorExtraInfo::Chimney(v) => {
                payload[0..4].copy_from_slice(&v.f.to_le_bytes());
                payload[4] = v.smoke;
            }
        }
        // 判别存于高位字节（见 extra_disc 注释：偏离原 +0x11 以避免载荷字节 1
        // 被判别覆盖）。
        self.dst_payload = u64::from_le_bytes(payload) | ((disc as u64) << 56);
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
pub type DecoratorOpEvent = DecoratorOpEventKind;

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
        // 原资源语义：共享同一内部队列（资源级并发句柄）。Arc 包装见 send 注释。
        DeferredDecoratorOpEvents {
            inner: Mutex::new(self.inner.lock().unwrap().clone()),
            wake: Condvar::new(),
        }
    }
}

impl DeferredDecoratorOpEvents {
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

    /// 0x140833a20（O）：`atan2(2(wy+zx), w²+z²-x²-y²)`（fresh 反编译展开；
    /// 对纯 Y 旋转 {y=sin(a/2), w=cos(a/2)} 退化为 a）。
    pub fn from_quat(q: Quat) -> Self {
        let y_arg = 2.0 * (q.w * q.y + q.z * q.x);
        let x_arg = q.w * q.w + q.z * q.z - q.x * q.x - q.y * q.y;
        Self(y_arg.atan2(x_arg))
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
        assert!(DecoratorDst::Wall(WallId(1)).is_wall());
        assert!(DecoratorDst::WallCompound(WallId(1)).is_wall());
        assert!(!DecoratorDst::Roof.is_wall());
        assert_eq!(DecoratorDst::Terrain.tag(), 3);
        assert_eq!(DecoratorDst::Stair.tag(), 4);
        assert_eq!(DecoratorDst::Wall(WallId(1)).wall_id(), WallId(1));
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
    fn transient_id_gen_modes() {
        let mut base = 100u32;
        // 直写模式：返回旧值并递增共享基数。
        let mut id_gen = TransientDecoratorIdGen::new(&mut base as *mut u32, true);
        assert_eq!(id_gen.generate(), 100);
        assert_eq!(id_gen.generate(), 101);
        assert_eq!(base, 102);

        // 偏移模式：返回 base + 偏移，基数不变。
        let mut id_gen = TransientDecoratorIdGen::new(&mut base as *mut u32, false);
        assert_eq!(id_gen.generate(), 102);
        assert_eq!(id_gen.generate(), 103);
        assert_eq!(base, 102);
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
