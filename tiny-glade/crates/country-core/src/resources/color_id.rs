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
/// 发射、set_wall_color_id 0x142290820 消费（0x18B 条目 {id@+8, 新色 u32@+0x10,
/// custom u8@+0x14}，re-analyst §2））。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetWallColorIdCmd {
    /// 目标墙 id。
    pub wall_id: u64,
    /// 新颜色。
    pub color: u32,
    /// 自定义标志。
    pub custom: u8,
}

/// 门颜色设置命令（O：`color_id::SetGateColorIdCmd`；set_gate_color_id
/// 0x142299840 消费；门与墙共享索引空间）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetGateColorIdCmd {
    /// 目标门 id。
    pub gate_id: u64,
    /// 新颜色。
    pub color: u32,
    /// 自定义标志。
    pub custom: u8,
}

/// 半木颜色设置命令（O：`color_id::SetHalftimberColorIdCmd`；set_halftimber_color_id
/// 0x142299250 消费）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct SetHalftimberColorIdCmd {
    /// 目标 id。
    pub id: u64,
    /// 新颜色。
    pub color: u32,
    /// 自定义标志。
    pub custom: u8,
}
