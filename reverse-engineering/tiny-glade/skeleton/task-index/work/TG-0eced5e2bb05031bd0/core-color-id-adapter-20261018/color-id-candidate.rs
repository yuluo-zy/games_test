//! ColorId newtype observed in B components and the render consumers.
use bevy_ecs::resource::Resource;
#[derive(
    bevy_ecs::component::Component,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct ColorId(pub u32);

impl ColorId {
    /// B 0x14099ca50.
    pub fn default_value() -> Self {
        Self(0)
    }
    pub fn is_default(&self) -> bool {
        self.0 == 0
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 墙颜色表（O：`WallColorIds::new` 0x140b19890 建空表；CPU 侧 = Vec<u32>
/// 按墙索引（上限 MAX_NUMBER_OF_WALLS=20480，0x4fff 断言），GPU 侧
/// `WallColorIdSsbo`（re-analyst-A 契约 §2 SSBO 写链）。
#[derive(Resource, Clone, Debug)]
pub struct WallColorIds(pub Vec<u32>);

impl Default for WallColorIds {
    fn default() -> Self {
        Self::new()
    }
}

impl WallColorIds {
    /// `WallColorIds::new`（0x140b19890，O）。
    pub fn new() -> Self {
        Self(vec![0; MAX_NUMBER_OF_WALLS])
    }

    /// `iter_nondefault`（行 85，内联；O：跳过缺省 ColorId）。
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != ColorId::default_value().0)
            .map(|(i, c)| (i, *c))
    }
}

/// 门颜色表（O：`GateColorIds::new` 0x140b19900；tag 7 记录族）。
#[derive(Resource, Clone, Debug)]
pub struct GateColorIds(pub Vec<u32>);

impl Default for GateColorIds {
    fn default() -> Self {
        Self::new()
    }
}

impl GateColorIds {
    /// `GateColorIds::new`（0x140b19900，O）。
    pub fn new() -> Self {
        Self(vec![0; MAX_NUMBER_OF_WALLS])
    }

    /// `iter_nondefault`（行 153，内联）。
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != ColorId::default_value().0)
            .map(|(i, c)| (i, *c))
    }
}

/// 半木颜色表（O：`HalftimberColorIds::new` 0x140b19970；tag 0xc 记录族）。
#[derive(Resource, Clone, Debug)]
pub struct HalftimberColorIds(pub Vec<u32>);

impl Default for HalftimberColorIds {
    fn default() -> Self {
        Self::new()
    }
}

impl HalftimberColorIds {
    /// `HalftimberColorIds::new`（0x140b19970，O）。
    pub fn new() -> Self {
        Self(vec![0; MAX_NUMBER_OF_WALLS])
    }

    /// `iter_nondefault`（行 222，内联）。
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != ColorId::default_value().0)
            .map(|(i, c)| (i, *c))
    }
}

/// 屋顶颜色表（O：`color_id_roof` 域；**默认值 = 1**（0x14099ca50，纠错——
/// coordinator 契约 #2），与墙/门/半木的 0 不同）。
#[derive(Resource, Clone, Debug)]
pub struct RoofColorIds(pub Vec<u32>);

impl Default for RoofColorIds {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofColorIds {
    /// `RoofColorIds::new`（O：默认填 1）。
    pub fn new() -> Self {
        Self(vec![1; MAX_NUMBER_OF_WALLS])
    }

    /// `iter_nondefault`（O：跳过屋顶默认 1）。
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != 1)
            .map(|(i, c)| (i, *c))
    }
}

/// 墙/门/半木/屋顶索引共用上限（O：断言 `idx < 0x4fff`，MAX_NUMBER_OF_WALLS=20480）。
pub const MAX_NUMBER_OF_WALLS: usize = 20480;

/// 墙颜色设置命令（O：`color_id::SetWallColorIdCmd`；color_menu 0x142291230
/// 发射、set_wall_color_id 0x142290820 消费）。
///
/// 载荷（0x18B EventInstance = 0x10B payload）：`{wall_id u64@+8,
/// color u32@+0x10, custom u8@+0x14}`。现接口保留低/高两个u32组成完整WallId。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetWallColorIdCmd {
    /// WallId low32; use wall_id() for the actual complete identity.
    pub id: u32,
    /// WallId high32; original producers/consumers copy the whole u64.
    pub _gap4: u32,
    /// 新颜色（u32 @+0x10）。
    pub color: u32,
    /// 自定义标志（u8 @+0x14）。
    pub custom: u8,
    /// +0x15..+0x18 对齐填充。
    pub _pad: [u8; 3],
}

/// 门颜色设置命令（O：`color_id::SetGateColorIdCmd`；set_gate_color_id
/// 0x142299840 消费；门与墙共享索引空间）。载荷同 `SetWallColorIdCmd`。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetGateColorIdCmd {
    /// WallId low32; use wall_id() for the actual complete identity.
    pub id: u32,
    /// WallId high32; original producers/consumers copy the whole u64.
    pub _gap4: u32,
    /// 新颜色（u32 @+0x10）。
    pub color: u32,
    /// 自定义标志（u8 @+0x14）。
    pub custom: u8,
    /// +0x15..+0x18 对齐填充。
    pub _pad: [u8; 3],
}

/// 半木颜色设置命令（O：`color_id::SetHalftimberColorIdCmd`；set_halftimber_color_id
/// 0x142299250 消费）。载荷同 `SetWallColorIdCmd`。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetHalftimberColorIdCmd {
    /// WallId low32; use wall_id() for the actual complete identity.
    pub id: u32,
    /// WallId high32; original producers/consumers copy the whole u64.
    pub _gap4: u32,
    /// 新颜色（u32 @+0x10）。
    pub color: u32,
    /// 自定义标志（u8 @+0x14）。
    pub custom: u8,
    /// +0x15..+0x18 对齐填充。
    pub _pad: [u8; 3],
}

impl SetWallColorIdCmd {
    /// Lossless adapter for the existing split-field representation. B callers
    /// pass the full event+8 qword to the identity-index leaf1400885d0.
    pub const fn wall_id(&self) -> crate::resources::walls::WallId {
        crate::resources::walls::WallId((self.id as u64) | ((self._gap4 as u64) << 32))
    }
    pub const fn new(wall_id: crate::resources::walls::WallId, color: u32, custom: u8) -> Self {
        Self {
            id: wall_id.0 as u32,
            _gap4: (wall_id.0 >> 32) as u32,
            color,
            custom,
            _pad: [0; 3],
        }
    }
}

impl SetGateColorIdCmd {
    /// Lossless adapter for the existing split-field representation. B callers
    /// pass the full event+8 qword to the identity-index leaf1400885d0.
    pub const fn wall_id(&self) -> crate::resources::walls::WallId {
        crate::resources::walls::WallId((self.id as u64) | ((self._gap4 as u64) << 32))
    }
    pub const fn new(wall_id: crate::resources::walls::WallId, color: u32, custom: u8) -> Self {
        Self {
            id: wall_id.0 as u32,
            _gap4: (wall_id.0 >> 32) as u32,
            color,
            custom,
            _pad: [0; 3],
        }
    }
}

impl SetHalftimberColorIdCmd {
    /// Lossless adapter for the existing split-field representation. B callers
    /// pass the full event+8 qword to the identity-index leaf1400885d0.
    pub const fn wall_id(&self) -> crate::resources::walls::WallId {
        crate::resources::walls::WallId((self.id as u64) | ((self._gap4 as u64) << 32))
    }
    pub const fn new(wall_id: crate::resources::walls::WallId, color: u32, custom: u8) -> Self {
        Self {
            id: wall_id.0 as u32,
            _gap4: (wall_id.0 >> 32) as u32,
            color,
            custom,
            _pad: [0; 3],
        }
    }
}

/// 装饰颜色设置命令（O：`resources::color_id::SetDecoratorColorIdCmd`；
/// set_decorator_color_id 0x1422af0c0 消费，5 层事件源均 stride 0x18）。
/// 载荷（0x18B EventInstance = 0x10B 载荷，reviewer-coregap-2 fresh 反编译）：
/// `{id u32@+8, value u32@+0xc, custom u8@+0x10}` —— **无 gap**（reviewer §3 纠错，
/// 与墙族 SetWallColorIdCmd 的 +4 间隙不同）。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorColorIdCmd {
    /// 目标装饰 id（u32 @+8）。
    pub id: u32,
    /// 新颜色/值（u32 @+0xc）。
    pub value: u32,
    /// 自定义标志（u8 @+0x10）。
    pub custom: u8,
    /// +0x11..+0x18 对齐填充。
    pub _pad: [u8; 7],
}

/// 窗灯命令（O：`resources::color_id::SetWindowLightCmd`；set_window_light
/// 0x142298260 消费，stride 0x18）。载荷 `{id u32@+8, value u32@+0xc,
/// custom u8@+0x10}`（**无 gap**，reviewer §3 纠错）。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetWindowLightCmd {
    /// 目标装饰 id（u32 @+8）。
    pub id: u32,
    /// 亮度值（u32 @+0xc，写 WindowExtra.0）。
    pub value: u32,
    /// 自定义标志（u8 @+0x10）。
    pub custom: u8,
    /// +0x11..+0x18 对齐填充。
    pub _pad: [u8; 7],
}

/// 梯子颜色命令（O：`resources::color_id::SetLadderColorIdCmd`；二进制 register_event
/// 全路径名，非 `SetLadderCmd`——reviewer §3 纠错）。stride 0x18。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetLadderColorIdCmd {
    /// 目标 id（u32 @+8）。
    pub id: u32,
    /// 值（u32 @+0xc）。
    pub value: u32,
    /// 自定义标志（u8 @+0x10）。
    pub custom: u8,
    /// +0x11..+0x18 对齐填充。
    pub _pad: [u8; 7],
}

/// 魔法颜色命令（O：`resources::color_id::SetMagicColorIdCmd`；二进制 register_event
/// 全路径名，非 `SetMagicCmd`——reviewer §3 纠错）。stride 0x18。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetMagicColorIdCmd {
    /// 目标 id（u32 @+8）。
    pub id: u32,
    /// 值（u32 @+0xc）。
    pub value: u32,
    /// 自定义标志（u8 @+0x10）。
    pub custom: u8,
    /// +0x11..+0x18 对齐填充。
    pub _pad: [u8; 7],
}

/// 装饰亮度命令（O：`resources::color_id::SetDecoratorBrightnessCmd`；二进制
/// register_event 全路径名，非 `SetBrightnessCmd`——reviewer §3 纠错）。stride 0x18。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetDecoratorBrightnessCmd {
    /// 目标 id（u32 @+8）。
    pub id: u32,
    /// 值（u32 @+0xc）。
    pub value: u32,
    /// 自定义标志（u8 @+0x10）。
    pub custom: u8,
    /// +0x11..+0x18 对齐填充。
    pub _pad: [u8; 7],
}

/// 楼梯栏杆命令（O：`resources::color_id::SetStairRailingCmd`；color 域 **0x10B**
/// 事件）。载荷 = `{id u32, value u8, custom u8}`（`repr(C)` 0x8B 载荷 → 0x10B
/// EventInstance，re-analyst §2 / RESTORE 纠错 #4）。
#[repr(C)]
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetStairRailingCmd {
    /// 目标 id（u32 @+8）。
    pub id: u32,
    /// 值（u8 @+0xc）。
    pub value: u8,
    /// 自定义标志（u8 @+0xd）。
    pub custom: u8,
    /// +0xe..+0x10 对齐填充。
    pub _pad: [u8; 2],
}

// ---- 颜色表 SSBO 元素（typed-gap 落库）----
// 全名（type_name）必须与库内 `assert_eq!(state.name, type_name::<T>())` 及
// `get_buffer::<T>` 断言一致；短名（SsboId 键）必须与着色器槽名一致（复数）。

/// 墙颜色表 SSBO 元素（O：`init_color_ids_buffer` 0x142290750 的
/// `add_new<color_id::WallColorIdSsbo>`；`set_wall_color_id` 0x142290820 的
/// `FrozenMap::get<SsboId..>` memcmp 0x32 = 50B = `"country_core::resources::
/// color_id::WallColorIdSsbo"` 全名；槽名 `"WallColorIds"` 复数，SsboId =
/// `xxh3_64("WallColorIds")` = 0xaf42315d263aa22a，O draw_builder.rs 已核对）。
/// 缓冲 = `TypedStorageBuffer`（写面经 vtable +0x18 读字节容量，无 CPU 影子）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallColorIdSsbo(pub u32);

/// 门颜色表 SSBO 元素（O：`add_new<color_id::GateColorIdSsbo>`；槽名
/// `"GateColorIds"`，draw_callbacks_a `E14_GATE_COLOR_BINDING` 已登记）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GateColorIdSsbo(pub u32);

/// 半木颜色表 SSBO 元素（O：`add_new<color_id::HalftimberColorIdSsbo>`；槽名
/// `"HalftimberColorIds"`，draw_callbacks_a `HALFTIMBER_COLOR_BINDING` 已登记）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HalftimberColorIdSsbo(pub u32);

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};

impl StorageElement for WallColorIdSsbo {
    /// 槽名（O：SsboId 键 = `xxh3_64("WallColorIds")`）。
    const NAME: &'static str = "WallColorIds";
    /// 固定 `MAX_NUMBER_OF_WALLS` 个 u32（O：与 WallColorIds Vec 等长，无初始数据也创建）。
    const BUFFER_LEN: usize = MAX_NUMBER_OF_WALLS;
}

impl LibrarySsbo for WallColorIdSsbo {
    type Storage = TypedStorageBuffer<WallColorIdSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::<WallColorIdSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for WallColorIdSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(Self::NAME).0
    }
}

impl StorageElement for GateColorIdSsbo {
    const NAME: &'static str = "GateColorIds";
    const BUFFER_LEN: usize = MAX_NUMBER_OF_WALLS;
}

impl LibrarySsbo for GateColorIdSsbo {
    type Storage = TypedStorageBuffer<GateColorIdSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::<GateColorIdSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for GateColorIdSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(Self::NAME).0
    }
}

impl StorageElement for HalftimberColorIdSsbo {
    const NAME: &'static str = "HalftimberColorIds";
    const BUFFER_LEN: usize = MAX_NUMBER_OF_WALLS;
}

impl LibrarySsbo for HalftimberColorIdSsbo {
    type Storage = TypedStorageBuffer<HalftimberColorIdSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        TypedStorageBuffer::<HalftimberColorIdSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for HalftimberColorIdSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(Self::NAME).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::SsboId;

    #[test]
    fn color_ssbo_full_names_match_the_memcmp_assert_lengths() {
        // `set_wall_color_id` 0x142290820 的 `memcmp(.., state.name, 0x32)`（O）。
        assert_eq!(
            core::any::type_name::<WallColorIdSsbo>(),
            "country_core::resources::color_id::WallColorIdSsbo"
        );
        assert_eq!(core::any::type_name::<WallColorIdSsbo>().len(), 0x32);
        // `write_roof_color_id_to_gpu` 0x142298520 的 memcmp 0x37（O，见 color_id_roof.rs）。
        assert_eq!(
            core::any::type_name::<crate::resources::color_id_roof::RoofColorIdSsbo>().len(),
            0x37
        );
        // 门/半木：set_gate_color_id 0x142299840 memcmp 0x32 / set_halftimber_color_id
        // 0x142299250 memcmp 0x38（O）。
        assert_eq!(core::any::type_name::<GateColorIdSsbo>().len(), 0x32);
        assert_eq!(core::any::type_name::<HalftimberColorIdSsbo>().len(), 0x38);
    }

    #[test]
    fn color_ssbo_short_names_hash_to_the_shader_binding_ssbo_ids() {
        // 槽名 = 复数（O：draw_builder WALL_COLOR_ID_SLOT_NAME；ssbo_tags 短名登记）。
        assert_eq!(WallColorIdSsbo::NAME, "WallColorIds");
        assert_eq!(GateColorIdSsbo::NAME, "GateColorIds");
        assert_eq!(HalftimberColorIdSsbo::NAME, "HalftimberColorIds");
        assert_eq!(
            crate::resources::color_id_roof::RoofColorIdSsbo::NAME,
            "RoofColorIds"
        );
        // SsboId 与着色器绑定立即数（O：bind_resources `0xaf42315d263aa22a`；
        // ssbo_tags `RoofColorIdTag` 0x8cce418b17d89f9b）。
        assert_eq!(SsboId::of::<WallColorIdSsbo>().0.0, 0xaf42315d263aa22a);
        assert_eq!(
            SsboId::of::<crate::resources::color_id_roof::RoofColorIdSsbo>()
                .0
                .0,
            0x8cce418b17d89f9b
        );
    }

    #[test]
    fn color_commands_have_the_observed_event_strides() {
        // 墙族 0x18B 事件（{id u32, +4 U, color u32, custom u8} 0x10B 载荷）。
        assert_eq!(core::mem::size_of::<SetWallColorIdCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetGateColorIdCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetHalftimberColorIdCmd>(), 0x10);
        // 装饰/窗族 0x18B 事件（{id u32, value u32, custom u8} 0x10B 载荷，无 gap）。
        assert_eq!(core::mem::size_of::<SetDecoratorColorIdCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetWindowLightCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetLadderColorIdCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetMagicColorIdCmd>(), 0x10);
        assert_eq!(core::mem::size_of::<SetDecoratorBrightnessCmd>(), 0x10);
        // 0x10B 事件（{id u32, value u8, custom u8} 0x8B 载荷）。
        assert_eq!(core::mem::size_of::<SetStairRailingCmd>(), 0x8);
    }
}

#[cfg(test)]
mod full_wall_id_tests {
    use super::*;
    #[test]
    fn wall_color_commands_keep_all_identity_bits_and_current_layout() {
        for id in [0, 7, 0xffff_ffff, 0x1_0000_0007, u64::MAX] {
            let wall = crate::resources::walls::WallId(id);
            for (low, high, decoded) in [
                {
                    let c = SetWallColorIdCmd::new(wall, 12, 255);
                    (c.id, c._gap4, c.wall_id())
                },
                {
                    let c = SetGateColorIdCmd::new(wall, 12, 255);
                    (c.id, c._gap4, c.wall_id())
                },
                {
                    let c = SetHalftimberColorIdCmd::new(wall, 12, 255);
                    (c.id, c._gap4, c.wall_id())
                },
            ] {
                assert_eq!(low, id as u32);
                assert_eq!(high, (id >> 32) as u32);
                assert_eq!(decoded, wall);
            }
        }
        assert_eq!(core::mem::size_of::<SetWallColorIdCmd>(), 16);
        assert_eq!(core::mem::offset_of!(SetWallColorIdCmd, _gap4), 4);
        assert_eq!(core::mem::offset_of!(SetWallColorIdCmd, color), 8);
        assert_eq!(core::mem::offset_of!(SetWallColorIdCmd, custom), 12);
    }
}
