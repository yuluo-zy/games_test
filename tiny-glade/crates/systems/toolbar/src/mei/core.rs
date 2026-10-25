//! B MEI retained-node UI context, original140683ec0/1406884c0.
//!
//! Root events create the traversal, minimum sizes are computed bottom-up, and
//! root/child placement runs forward before real text/shape drawing. Each frame
//! retains previous content owners and computed layouts, while rebuilding style
//! and parent/child relationships. Original-frame constructors return NodeContext.
//!
//! Evidence and limits: task-index work TG-70a753eeccd8998d55 and
//! TG-19924eac9946ff87be/meihost-20261018/LAYOUT_CONTRACT.md. Source-level
//! ownership/containers use Rust; no native layout compatibility is claimed.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::prelude::Resource;
use country_core::input::action_set::{
    ActionSetAction, DigitalActionState, MainActionSet as CoreMainActionSet,
};
use glam::Vec2;
use tg_text::font_table::FontTable;

/// 节点 id 混淆常量（O：safe_frame 0xf99230 / new_at_pos 0xf992e0 /
/// decorate_hovered 0x67df20 逐字节 `hash ^ 0xb1695320004fb443`）。
pub const GLOBAL_ID_MASK: u64 = 0xb1695320004fb443;

/// `utils::calculate_hash`（O 0x1405bdda0：`sip::write<Sip13Rounds>`×2，
/// 375B 无 RandomState/thread-local 机制 → 零键 `DefaultHasher::new()`（SipHasher13）。
pub use ::utils::calculate_hash;

/// 节点全局 id（O：原类型路径 `country_core::systems::ui::GlobalId`，crate 内暂驻；
/// 恢复后应移至 country-core）。Hash = std SipHash（impl$0::as_id 0x683c10）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct GlobalId(pub u64);

/// Original `find_node_path` (inline, 2 instances; host
/// `process_input_action_handlers`): walk the child→parent table from `focus`
/// to the root, then reverse so the result is root→focus. Native always calls
/// `slice::reverse`, which is a no-op for a single element.
pub fn find_node_path(parents: &HashMap<GlobalId, GlobalId>, focus: GlobalId) -> Vec<GlobalId> {
    let mut path = vec![focus];
    let mut current = focus;
    while let Some(&parent) = parents.get(&current) {
        path.push(parent);
        current = parent;
    }
    path.reverse();
    path
}

impl GlobalId {
    /// 名字节点 id：`calculate_hash((name, ())) ^ GLOBAL_ID_MASK`（零键 SipHasher13）。
    /// `<String>` 单态（new_at_pos<String> 0x142289f80）经 `WidgetId::from_name`
    /// =同一 `calculate_hash` 函数 → 两单态同值（复核坐实）。
    pub fn new(name: &str) -> GlobalId {
        GlobalId(calculate_hash(&(name, ())) ^ GLOBAL_ID_MASK)
    }
}

/// 行为标志字节（Node+0xb7）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeFlags(pub u8);

impl NodeFlags {
    /// bit0：block_input / force_interactive。
    pub const FORCE_INTERACTIVE: u8 = 1;
    /// bit1：ignore_input。
    pub const IGNORE_INPUT: u8 = 2;
    /// bit2：default_focus。
    pub const DEFAULT_FOCUS: u8 = 4;
    /// bit5：with_nav。
    pub const WITH_NAV: u8 = 0x20;
}

/// 状态字节（Node+0xb6）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeStateBits(pub u8);

impl NodeStateBits {
    pub const HOVER: u8 = 1 << 0;
    pub const DRAG_STARTED: u8 = 1 << 1;
    pub const DRAG: u8 = 1 << 2;
    pub const DRAG_FINISHED: u8 = 1 << 3;
    pub const CLICKED: u8 = 1 << 4;
    pub const FOCUS: u8 = 1 << 6;
}

/// Safe projection of B's prepare-layout context, constructed at
/// 0x140688941..0x14068899f. References preserve the actual world resources;
/// their offsets and ABI are not represented by this Rust structure.
pub struct NodePrepareContext<'a> {
    pub text: &'a mut tg_text::TextRenderer,
    pub device: &'a country_core::render::device::GraphicsDeviceResource,
    pub textures: &'a mut country_core::asset_libraries::texture_library::AssetTextureLibrary,
    /// 当前 App 的文件图片缓存和真实纹理设备；纯 CPU 控件测试可不安装。
    pub file_images: Option<&'a mut crate::mei::widgets::FileImageCache>,
    pub state: &'a mut u8,
    pub actions: &'a CoreMainActionSet,
    pub window_points: Vec2,
    pub steam_client: Option<&'a country_core::systems::steam::SteamClientResource>,
    pub steam_input: Option<&'a country_core::resources::steam::SteamInput>,
    pub pixels_per_point: f32,
    pub cursor: Vec2,
    pub elapsed: f32,
}

/// B 0x140689f48..0x140689f82, original render-context 48-byte read surface.
/// `center` is exactly `node.pos + 0.5 * node.size`; Node.pos is the origin.
pub struct NodeDrawContext<'a> {
    pub text: &'a mut tg_text::TextRenderer,
    pub center: Vec2,
    pub extent: Vec2,
    pub opacity: f32,
    pub pixels_per_point: f32,
    pub cursor: Vec2,
    pub elapsed: f32,
    pub state: u8,
}

impl NodeDrawContext<'_> {
    /// Original `RenderContext::draw_rect` (inline, 10 instances; deps:
    /// `Vec2::sub` + `[f32;2]` conversion). The native helper converts the
    /// center/extent pair to an origin then emits the encoded rect; this
    /// projection keeps the caller parameters explicit (I: exact parameter
    /// order U).
    pub fn draw_rect(
        &mut self,
        center: Vec2,
        extent: Vec2,
        opacity: f32,
        shapes: &tg_text::RawShapeStream,
    ) {
        self.text
            .rect(
                (center - extent * 0.5).to_array(),
                extent.to_array(),
                opacity,
                shapes,
            )
            .expect("draw MEI rect");
    }
}

/// 节点内容 trait（O：NodeContent 虚表 +0x30=type_name、+0x40=interactive、
/// +0x48=process_input_action、+0x50=on_input、+0x58=pre_update、
/// +0x60=prepare_layout、+0x68=render）。`draw` is the Rust spelling of the
/// recovered +0x68 render consumer, not an extra native virtual method.
/// `Send + Sync` 为 `MeiContext: Resource` 承载所需（重建约束，I）。
pub trait NodeContent: std::any::Any + Send + Sync {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::core::NodeContent"
    }
    fn interactive(&self) -> bool {
        false
    }
    /// 是否为占位空内容（原生 Node 内容指针为空的等价判别；mei_pre_update 的
    /// widget 注册/事件分发只对非空内容节点执行）。
    fn is_empty(&self) -> bool {
        false
    }
    /// 输入事件分发（vtable +0x48）。O：mei_pre_update 对焦点节点/悬停节点以
    /// `MeiInputEvent` 载荷调用（`0xc3` 滚轮 / `0xc4` / `0xc5` / `0xc6`）。
    fn process_input_action(&mut self, event: &MeiInputEvent) -> bool {
        let _ = event;
        false
    }
    /// 动作集处理（vtable +0x50）。O：`process_input_action_handlers` 逐
    /// 12B `ActionSetAction` 记录调用，返回是否 handled。
    fn on_input(&mut self, action: &ActionSetAction) -> bool {
        let _ = action;
        false
    }
    /// Original vtable+50 receives the capture/bubble phase and whether a
    /// descendant handled the action. Keep the legacy adapter for old content.
    fn on_input_phase(&mut self, action: &ActionSetAction, phase: [bool; 2]) -> bool {
        let _ = phase;
        self.on_input(action)
    }
    /// 帧前更新（vtable +0x58）。O：mei_pre_update 0x140684? 对每个节点调用
    /// `(&MeiContext.actions, {&cursor_units, &mut node.state_bits})`。
    /// 默认空实现（对应 0x3bb70 空壳）。
    fn pre_update(&mut self, actions: &CoreMainActionSet, cursor_units: Vec2, state_bits: &mut u8) {
        let _ = (actions, cursor_units, state_bits);
    }
    fn prepare_layout(
        &mut self,
        _ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[crate::mei::layout::LengthPercentage; 2]> {
        None
    }
    fn draw(&mut self, _ctx: &mut NodeDrawContext<'_>) {}
}

/// Source-level projection of B's 56-byte frame-event variants. Payloads are
/// decoded at original post_update140689091..140689b60; no native ABI is claimed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameKind {
    FullWindow,
    Safe,
    WideSafe,
    Relative {
        requested_size: Option<Vec2>,
        relative_to: GlobalId,
        offset: Vec2,
        reference_anchor: Vec2,
        anchor: Vec2,
    },
    AtPos {
        position: Vec2,
        requested_size: Option<Vec2>,
        anchor: Vec2,
    },
    /// Synthesized by the native traversal, not a top-level producer.
    Child,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameEvent {
    pub kind: FrameKind,
    pub id: GlobalId,
}
impl FrameEvent {
    pub fn safe(id: GlobalId) -> Self {
        Self {
            kind: FrameKind::Safe,
            id,
        }
    }
    pub fn at_pos(
        id: GlobalId,
        position: Vec2,
        requested_size: Option<Vec2>,
        anchor: Vec2,
    ) -> Self {
        Self {
            kind: FrameKind::AtPos {
                position,
                requested_size,
                anchor,
            },
            id,
        }
    }
}

/// mei 输入事件（stride 0x70；O：tag@+0x58/+0x68∈{0xc3,0xc4,0xc5,0xc6}，剪贴板=0xc5）。
/// 字段语义 I：on_key_event 把键盘事件 10×u64 + 3×u64 复制进 0x70 载荷并写 tag 字节。
#[derive(Clone, Debug)]
#[repr(C, align(8))]
pub struct MeiInputEvent {
    /// 0x70 字节载荷（原枚举体；字段名 U）。
    pub data: [u64; 14],
}

impl MeiInputEvent {
    /// 键盘输入事件（O 0x140685fc0：tag 字节 in_R8B 写 +0x68）。
    pub fn keyboard(payload: [u64; 14], tag: u8) -> MeiInputEvent {
        let mut data = payload;
        data[13] = (data[13] & !0xff) | tag as u64;
        MeiInputEvent { data }
    }

    /// 剪贴板粘贴事件（O 0x1406860d0：tag 0xc5 写 +0x58）。
    pub fn clipboard_paste(payload: [u64; 14]) -> MeiInputEvent {
        let mut data = payload;
        data[11] = (data[11] & !0xff) | 0xc5;
        MeiInputEvent { data }
    }

    /// 鼠标滚轮事件（O：mei_pre_update 0x140684826 段构造，tag 0xc3、
    /// 载荷 = 滚轮累计 Vec2 写首 8 字节）。
    pub fn scroll(delta: Vec2) -> MeiInputEvent {
        let mut data = [0u64; 14];
        data[0] = (delta.x.to_bits() as u64) | ((delta.y.to_bits() as u64) << 32);
        data[11] = (data[11] & !0xff) | 0xc3;
        MeiInputEvent { data }
    }

    /// 变体 tag（O：键盘事件读 +0x68、剪贴板/滚轮读 +0x58；本实现取非零者）。
    pub fn tag(&self) -> u8 {
        let keyboard = (self.data[13] & 0xff) as u8;
        if keyboard != 0 {
            keyboard
        } else {
            (self.data[11] & 0xff) as u8
        }
    }

    /// 首 8 字节载荷按 Vec2 解释（O：滚动事件 `fVar46/fVar47` 累计源）。
    pub fn payload_vec2(&self) -> Vec2 {
        Vec2::new(
            f32::from_bits(self.data[0] as u32),
            f32::from_bits((self.data[0] >> 32) as u32),
        )
    }
}

/// 输入动作集（复用 country-core MainActionSet 语义；本 crate 内暂驻，动作字节未列）。
#[derive(Clone, Copy, Debug, Default)]
pub struct MainActionSet {
    /// 动作 id 集合（O：Deconstruct/Reconstruct 的位集，字节语义 U）。
    pub bits: u64,
}

/// 输入动作 handler 表条目（历史投影，2026-10-18 fresh 判等后已不再使用）。
///
/// fresh 反编译 0x140687eb0/0x1406884c0 显示 `mei+0x1d0..+0x1e8` 的 hashbrown
/// RawTable 是 **`HashMap<GlobalId, GlobalId>`（stride 0x10，键 = 子节点 id、
/// 值 = 父节点 id）**，不是 12B 动作位集记录；frozenset 语义见
/// `MeiContext::parents`。此类型保留仅为兼容旧注释/外部重命名，无运行路径。
#[derive(Clone, Copy, Debug)]
pub struct InputActionHandler {
    /// 关联节点 id。
    pub id: GlobalId,
    /// 动作位集。
    pub action: MainActionSet,
}

/// 节点（map 条目 stride 0xc0，键 GlobalId@+0，Node@+0x08；字段偏移 I，语义 O）。
pub struct Node {
    /// 子节点 id 列表（Vec<GlobalId>，Node+0x00）。
    pub children: Vec<GlobalId>,
    /// 盒模型样式（72B，Node+0x18）。
    pub style: crate::mei::layout::LayoutStyle,
    /// Actual prepare_layout dimensions copied to B Node+0x60..0x70.
    /// Native reuse_or_create_node zeros these 16 bytes for a new node. The
    /// former NodeState projection mistook style's justify byte for state.
    pub intrinsic_size: [crate::mei::layout::LengthPercentage; 2],
    /// 计算尺寸 f32×2。
    pub size: Vec2,
    /// 计算位置 f32×2。
    pub pos: Vec2,
    /// 内容对象（Box 数据 +0x80，vtable +0x88）。
    pub content: Box<dyn NodeContent>,
    /// 新建/身份回收标志（Node+0x90）。
    pub created: u32,
    /// Effective draw opacity at originalNode+a4, default1.
    pub opacity: f32,
    /// Original local opacity at+b0; +a4 is the effective draw opacity.
    pub local_opacity: f32,
    /// Local z at+a8, accumulated z at+ac.
    pub local_z: f32,
    /// Previous frame computed size/origin at+94..a4.
    pub previous_layout: crate::mei::layout::ComputedLayout,
    /// hover 声音 id（Node+0xb4，默认 0x0d）。
    pub sound_hover: u8,
    /// click 声音 id（Node+0xb5，默认 0x0d）。
    pub sound_click: u8,
    /// 状态字节（Node+0xb6）。
    pub state_bits: u8,
    /// 行为标志字节（Node+0xb7）。
    pub flags: u8,
    /// Original effective z at+ac = incoming parent z + local z at+a8.
    pub z: f32,
}

impl Node {
    /// 新节点（O 0x140683ec0 插入路径：children 空 cap=8、状态 {0,0,4,0}、
    /// opacity=1.0、声音 0x0d、0x0d、其余清零）。
    pub fn new(content: Box<dyn NodeContent>) -> Node {
        Node {
            children: Vec::with_capacity(8),
            style: crate::mei::layout::LayoutStyle::default(),
            intrinsic_size: [crate::mei::layout::LengthPercentage::default(); 2],
            size: Vec2::ZERO,
            pos: Vec2::ZERO,
            content,
            created: 0,
            opacity: 1.0,
            local_opacity: 1.0,
            local_z: 0.0,
            previous_layout: crate::mei::layout::ComputedLayout::default(),
            sound_hover: 0x0d,
            sound_click: 0x0d,
            state_bits: 0,
            flags: 0,
            z: 0.0,
        }
    }
}

// ------------------------------------------------------------------
// Node 方法（O 0x140683c20..0x140683cb0）
// ------------------------------------------------------------------

impl Node {
    /// O 0x140683c20：写 hover/click 声音 id（+0xb4/+0xb5）。**判等**。
    pub fn set_sounds(&mut self, hover: u8, click: u8) {
        self.sound_hover = hover;
        self.sound_click = click;
    }

    /// O 0x140683c30：`+0xb7 |= 2`。**判等**。
    pub fn ignore_input(&mut self) {
        self.flags |= NodeFlags::IGNORE_INPUT;
    }

    /// O 0x140683c40：`+0xb7 |= 1`。**判等**。
    pub fn force_interactive(&mut self) {
        self.flags |= NodeFlags::FORCE_INTERACTIVE;
    }

    /// O 0x140683c50：`+0xb7 |= 4`。**判等**。
    pub fn default_focus(&mut self) {
        self.flags |= NodeFlags::DEFAULT_FOCUS;
    }

    /// O 0x140683c60：`+0xb7 = (b<<5) | (+0xb7 & 0xdf)`。**判等**。
    pub fn with_nav(&mut self, nav: bool) {
        self.flags = (nav as u8) << 5 | (self.flags & !NodeFlags::WITH_NAV);
    }

    /// 原steal_focus写bit3，由update_nav的优先焦点分支消费。
    pub fn steal_focus(&mut self) {self.flags|=8;}
    /// 原PDB与with_nav在140683c60物理合并；保留原API的同一bit5 setter。
    pub fn lose_focus_on_invalid_nav(&mut self,enabled:bool) {self.with_nav(enabled);}

    /// B RNE0x140683c80/11B: testb $0x30,+0xb6; setne al; ret.
    /// Pointer/key activation is a predicate; this does not mutate the node.
    pub fn used(&self) -> bool {
        self.state_bits & 0x30 != 0
    }

    /// O 0x140683c90：hover getter 内联（物理 return;）；state bit0。**重建**（I）。
    pub fn hover(&self) -> bool {
        self.state_bits & NodeStateBits::HOVER != 0
    }

    /// O 0x140683ca0：has_focus getter 内联（物理 return;）；state bit6。**重建**（I）。
    pub fn has_focus(&self) -> bool {
        self.state_bits & NodeStateBits::FOCUS != 0
    }

    /// B reuse_or_create_node saves the old computed layout at+94..a4.
    pub fn prev_computed_layout(&self) -> crate::mei::layout::ComputedLayout {
        self.previous_layout
    }

    /// O：block_input（+0xb7=1）。**重建**（I：置位方式按 ANALYSIS-A 1.7）。
    pub fn block_input(&mut self) {
        self.flags = NodeFlags::FORCE_INTERACTIVE;
    }
}

// ------------------------------------------------------------------
// MeiContext
// ------------------------------------------------------------------

/// mei 上下文（O：+0x08/+0x10 z 序表；+0x18..+0x30 输入事件 Vec<MeiInputEvent>；
/// +0x1a0..+0x1b8 当前帧节点表；+0x1d0..+0x1e8 输入 handler 表；+0x200.. 上一帧
/// 节点表双缓冲；+0x260 Arc<FontTable>）。帧事件缓冲在本 crate 内驻于
/// `frame_events`（原二进制由绘制链线程化 `Vec<Event>`，见 safe_frame 注）。
#[derive(Resource)]
pub struct MeiContext {
    /// z 序有序列表（stride 0x38 观察；条目 U）。
    pub z_order: Vec<GlobalId>,
    /// 输入事件缓冲（stride 0x70）。
    pub input_events: Vec<MeiInputEvent>,
    /// 帧事件缓冲（stride 0x38；safe_frame/new_at_pos 追加，mei_post_update 消费）。
    pub frame_events: Vec<FrameEvent>,
    /// 当前帧节点表（O：hashbrown HashMap<GlobalId, Node>）。
    pub nodes: HashMap<GlobalId, Node>,
    /// 父子链表（O：`mei+0x1d0/+0x1d8/+0x1e8` 的 `HashMap<GlobalId, GlobalId>`，
    /// stride 0x10）。键 = 子节点 id，值 = 父节点 id。
    ///
    /// fresh 2026-10-18：`mei_pre_update` 帧首 clear 该表（`memset(ctrl,0xff)` +
    /// len=0），随后遍历节点表、对每个 `children` 条目 `insert(child, node_id)`
    /// （0x1406845ac..0x140684709）。`mei_post_update::update_nav` 从焦点 id
    /// 沿该链上溯找兄弟组；`process_input_action_handlers` 从焦点 id 收集
    /// root→focus 链。
    pub parents: HashMap<GlobalId, GlobalId>,
    /// 上一帧节点表（双缓冲，帧末交换+clear 自动 GC）。
    pub previous_nodes: HashMap<GlobalId, Node>,
    /// 字体表（+0x260 Arc<FontTable>）。
    pub font_table: Arc<FontTable>,
    /// 配置表（O 0x140682e70：`HashMap<String, Arc<NodeConfig>>`，load_configs 写入）。
    pub configs: HashMap<String, Arc<crate::mei::config::NodeConfig>>,
    /// 帧内临时计数（+0x268/+0x270 观察；语义 U）。
    pub temp: u64,
    /// 本帧过滤后的动作集（O：mei_pre_update 末尾 `memcpy(mei+0x30, set, 0x170)`；
    /// 0x170 = `MainActionSet` 大小）。
    pub actions: CoreMainActionSet,
    /// 方向输入状态（O：`mei+0xd0`/`+0xd8`/`+0xe0`/`+0xe8` 四个
    /// `DigitalActionState`，内存序 = `[left, right, down, up]`）。
    ///
    /// `mei_post_update` 0x14068a0d0..0x14068a1f8 以 pairwise
    /// `just_pressed` 求 `dir = (right - left, down - up)`；`negative =
    /// up || left` 决定兄弟组取前/后段。**写入方未取证**（`mei+0xd0..` 无
    /// 已恢复系统写面），登记为 U；消费面按 fresh 判等落库。
    pub nav_actions: [DigitalActionState; 4],
    /// 点击/确认动作（O：`mei+0xc8` 一个 `DigitalActionState`）。
    ///
    /// `mei_pre_update` 0x140684fb1..0x140684fcd：`clicked()` 为假时，
    /// 若 `focus==Mei(id)` 且 `just_pressed(mei+0xc8)` 仍置 CLICKED 位。
    /// **写入方未取证**，登记为 U。
    pub click_action: DigitalActionState,
    /// 光标（mei 单元 = 屏幕像素 / (window_height/1000)，O：mei+0x268 双 f32）。
    pub cursor_units: Vec2,
    /// Legacy source name; actual `Time::delta_secs` (B Time+0x58).
    pub elapsed_wrapped: f32,
}

impl Default for MeiContext {
    fn default() -> Self {
        MeiContext {
            z_order: Vec::new(),
            input_events: Vec::new(),
            frame_events: Vec::new(),
            nodes: HashMap::new(),
            parents: HashMap::new(),
            previous_nodes: HashMap::new(),
            font_table: Arc::new(FontTable::default()),
            configs: HashMap::new(),
            temp: 0,
            actions: CoreMainActionSet::default(),
            nav_actions: [DigitalActionState::default(); 4],
            click_action: DigitalActionState::default(),
            cursor_units: Vec2::ZERO,
            elapsed_wrapped: 0.0,
        }
    }
}

impl MeiContext {
    /// B140683ec0 returns both a node reference and whether it already existed
    /// in the current table. Recycled previous-frame content retains its owner,
    /// interaction bits and previous computed layout; frame style is rebuilt.
    pub fn reuse_or_create_node(&mut self, id: GlobalId) -> &mut Node {
        if !self.nodes.contains_key(&id) {
            let node = if let Some(old) = self.previous_nodes.remove(&id) {
                let mut node = Node::new(old.content);
                node.created = 1;
                node.flags = old.flags & 0xf3;
                node.state_bits = old.state_bits;
                node.previous_layout = crate::mei::layout::ComputedLayout {
                    size: old.size,
                    pos: old.pos,
                };
                node
            } else {
                Node::new(Box::new(EmptyContent))
            };
            self.nodes.insert(id, node);
        }
        self.nodes.get_mut(&id).unwrap()
    }

    /// Original MeiContext::new_root (inline, 85 instances): shared root
    /// creation used by fullscreen/safe_frame/new_at_pos/new_relative.
    fn new_root(&mut self, kind: FrameKind, name: &str, opaque: bool) -> NodeContext {
        self.new_root_with_id(kind, GlobalId::new(name), opaque)
    }

    fn new_root_with_id(&mut self, kind: FrameKind, id: GlobalId, opaque: bool) -> NodeContext {
        let existed = self.nodes.contains_key(&id);
        let node = self.reuse_or_create_node(id);
        node.style.align = 0;
        if opaque {
            node.flags = NodeFlags::FORCE_INTERACTIVE;
        }
        if !existed {
            self.frame_events.push(FrameEvent { kind, id });
        }
        NodeContext::new(self, id)
    }

    /// Original MeiContext::fullscreen (inline, 12 instances): a full-window
    /// root that blocks input.
    pub fn fullscreen(&mut self, name: &str) -> NodeContext {
        self.new_root(FrameKind::FullWindow, name, true)
    }

    /// Original MeiContext::fullscreen_without_block_input (inline, 4
    /// instances): a full-window root that does not set FORCE_INTERACTIVE.
    pub fn fullscreen_without_block_input(&mut self, name: &str) -> NodeContext {
        self.new_root(FrameKind::FullWindow, name, false)
    }

    /// Original140f99230: enqueue once when this current-frame root is first
    /// created. `test dl,1` tests the helper's `existed` return, not id parity.
    pub fn safe_frame(&mut self, name: &str, opaque: bool) -> NodeContext {
        self.new_root(FrameKind::Safe, name, opaque)
    }

    /// 原宽安全区根复用同一事件和节点owner，布局阶段已有独立WideSafe宽度合同。
    pub fn wide_safe_frame(&mut self,name:&str,opaque:bool)->NodeContext {
        self.new_root(FrameKind::WideSafe,name,opaque)
    }

    /// Original140f992e0: position, optional size and anchor are independent
    /// payloads. The old scalar signature confused Option discriminant/values.
    pub fn new_at_pos(
        &mut self,
        name: &str,
        position: Vec2,
        requested_size: Option<Vec2>,
        anchor: Vec2,
        opaque: bool,
    ) -> NodeContext {
        self.new_root(
            FrameKind::AtPos {
                position,
                requested_size,
                anchor,
            },
            name,
            opaque,
        )
    }
    /// Original relative-frame source adapter. All five native payload fields
    /// are retained; new_root supplies the same per-frame reuse/event lifecycle
    /// as fullscreen, safe_frame and new_at_pos.
    #[allow(clippy::too_many_arguments)]
    pub fn new_relative(
        &mut self,
        name: &str,
        relative_to: GlobalId,
        requested_size: Option<Vec2>,
        offset: Vec2,
        reference_anchor: Vec2,
        anchor: Vec2,
        opaque: bool,
    ) -> NodeContext {
        self.new_root(
            FrameKind::Relative { requested_size, relative_to, offset, reference_anchor, anchor },
            name,
            opaque,
        )
    }

    /// Original decorators supply a tuple-derived global ID rather than a name.
    #[allow(clippy::too_many_arguments)]
    pub fn new_relative_with_id(
        &mut self,
        id: GlobalId,
        relative_to: GlobalId,
        requested_size: Option<Vec2>,
        offset: Vec2,
        reference_anchor: Vec2,
        anchor: Vec2,
        opaque: bool,
    ) -> NodeContext {
        self.new_root_with_id(
            FrameKind::Relative { requested_size, relative_to, offset, reference_anchor, anchor },
            id,
            opaque,
        )
    }

    pub fn new_at_pos_string(
        &mut self,
        name: String,
        position: Vec2,
        requested_size: Option<Vec2>,
        anchor: Vec2,
        opaque: bool,
    ) -> NodeContext {
        self.new_at_pos(&name, position, requested_size, anchor, opaque)
    }

    /// Native pre_update14068548e/1406854f1. Do this after processing last
    /// frame's input, before systems rebuild this frame's nodes.
    pub fn recycle_frame_nodes(&mut self) {
        self.frame_events.clear();
        std::mem::swap(&mut self.nodes, &mut self.previous_nodes);
        self.nodes.clear();
    }

    /// Original frame-events -> reverse minimum sizes -> forward placement.
    /// This is the layout stage within mei_post_update, after content prepare.
    pub fn perform_layout(&mut self, window_points: Vec2) {
        use crate::mei::layout::{ChildLayoutInput, ComputedLayout, ParentLayout, place_childen};
        let mut stack: Vec<_> = self
            .frame_events
            .iter()
            .copied()
            .map(|event| (event, 1.0f32, 0u8))
            .collect();
        stack.reverse();
        let mut order = Vec::new();
        while let Some((event, z, flags)) = stack.pop() {
            order.push(event);
            let node = self
                .nodes
                .get_mut(&event.id)
                .expect("MEI frame root/child missing");
            node.z = z + node.local_z;
            node.flags |= flags;
            let child_z = node.z + 1.;
            let child_flags = (node.flags & NodeFlags::IGNORE_INPUT) | flags;
            for child in &node.children {
                stack.push((
                    FrameEvent {
                        kind: FrameKind::Child,
                        id: *child,
                    },
                    child_z,
                    child_flags,
                ));
            }
        }
        for event in order.iter().rev() {
            let node = self.nodes.get(&event.id).unwrap();
            let size = node.style.compute_min_size_with_children(
                node.intrinsic_size,
                node.children
                    .iter()
                    .map(|id| self.nodes.get(id).expect("MEI minimum child missing").size),
            );
            self.nodes.get_mut(&event.id).unwrap().size = size;
        }
        let mut opacity = 1.0f32;
        for event in &order {
            let minimum = self.nodes.get(&event.id).unwrap().size;
            let resolved = match event.kind {
                FrameKind::FullWindow => Some(ComputedLayout {
                    size: window_points,
                    pos: Vec2::ZERO,
                }),
                FrameKind::Safe | FrameKind::WideSafe => {
                    let width = window_points.x.min(if event.kind == FrameKind::Safe {
                        1500.
                    } else {
                        1777.7778
                    });
                    Some(ComputedLayout {
                        size: Vec2::new(width, window_points.y),
                        pos: Vec2::new((window_points.x - width) * 0.5, 0.),
                    })
                }
                FrameKind::AtPos {
                    position,
                    requested_size,
                    anchor,
                } => {
                    let size = requested_size.unwrap_or(minimum);
                    Some(ComputedLayout {
                        size,
                        pos: position - (anchor + Vec2::ONE) * 0.5 * size,
                    })
                }
                FrameKind::Relative {
                    requested_size,
                    relative_to,
                    offset,
                    reference_anchor,
                    anchor,
                } => {
                    let reference = self
                        .nodes
                        .get(&relative_to)
                        .expect("MEI relative frame reference missing");
                    let size = requested_size.unwrap_or(minimum);
                    let point =
                        reference.pos + (reference_anchor + Vec2::ONE) * 0.5 * reference.size;
                    Some(ComputedLayout {
                        size,
                        pos: point + offset - (anchor + Vec2::ONE) * 0.5 * size,
                    })
                }
                FrameKind::Child => None,
            };
            let node = self.nodes.get_mut(&event.id).unwrap();
            if let Some(layout) = resolved {
                node.size = layout.size;
                node.pos = layout.pos;
            }
            // Native child tag5 keeps xmm11, root tags reset it to1.
            // Preserve the observed traversal accumulation, including siblings.
            if event.kind != FrameKind::Child {
                opacity = 1.;
            }
            opacity *= node.local_opacity;
            node.opacity = opacity;
            let children = node.children.clone();
            let style = node.style;
            let parent = ParentLayout {
                main: node.size.x,
                cross: node.size.y,
                offset: node.pos,
            };
            let inputs: Vec<_> = children
                .iter()
                .map(|id| {
                    let child = self.nodes.get(id).expect("MEI child layout missing");
                    ChildLayoutInput {
                        style: &child.style,
                        padding: &child.intrinsic_size,
                        prev_x: child.size.x,
                        prev_y: child.size.y,
                    }
                })
                .collect();
            let layouts = place_childen(&style, &parent, &inputs);
            for (id, layout) in children.iter().zip(layouts) {
                let node = self.nodes.get_mut(id).unwrap();
                node.size = layout.size;
                node.pos = layout.pos;
            }
        }
        self.z_order = order.iter().map(|e| e.id).collect();
    }

    /// O 0x140685fc0 `on_key_event(key_event, tag)`：push MeiInputEvent（stride 0x70，
    /// cap@+0x18/data@+0x20/len@+0x28，满时 grow_one）。**判等**（载荷字段语义 I）。
    pub fn on_key_event(&mut self, payload: [u64; 14], tag: u8) {
        self.input_events
            .push(MeiInputEvent::keyboard(payload, tag));
    }

    /// O 0x1406860d0 `on_clipboard_paste(payload)`：push MeiInputEvent，tag 0xc5。
    /// **判等**（载荷字段语义 I）。
    pub fn on_clipboard_paste(&mut self, payload: [u64; 14]) {
        self.input_events
            .push(MeiInputEvent::clipboard_paste(payload));
    }

    /// Load and validate a font table before replacing the shared configuration.
    /// On an I/O, parsing or validation error, the existing table stays installed.
    pub fn load_font_table(&mut self, path: &std::path::Path) -> tg_text::TextResult<()> {
        let table: FontTable = crate::mei::config::load_font_table(path)?;
        self.font_table = Arc::new(table);
        Ok(())
    }

    /// O 0x140682e70 `load_configs(&mut self, dir)`：walkdir 扫描目录，扩展名=="ron"
    /// 的文件 relative_to(dir).file_stem() 作键，read_node_config_from_file 解析，
    /// log "Loaded UI config from {stem}"，插入 `self.configs: HashMap<String,
    /// Arc<NodeConfig>>`（原存 self 字段，FIX 6）。**重建**（I：walkdir 以 std::fs
    /// 递归等价）。
    pub fn load_configs(&mut self, dir: &std::path::Path) {
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                if let Ok(relative) = path.strip_prefix(dir) {
                    let key = relative
                        .with_extension("")
                        .to_string_lossy()
                        .replace('\\', "/");
                    let config = crate::mei::config::read_node_config_from_file(path)
                        .unwrap_or_else(|e| panic!("{}", e));
                    log::info!("Loaded UI config from {key}");
                    self.configs.insert(key, Arc::new(config));
                }
            }
        }
    }

    /// O 0x140687eb0 `process_input_action_handlers(&mut self, focus_id, action)`。
    ///
    /// fresh 2026-10-18 逐指令（`work/restorer-mei-final/evidence/process_handlers.asm.txt`）：
    /// 1. `vec = [focus_id]`；循环 `current = parents.get(current)` 直到缺失，
    ///    每个父 id 追加；`len > 1` 时 **reverse**（得到 root→focus）。
    /// 2. 正向遍历 root→focus：仅当 parents 表非空且节点存在时，调
    ///    `content.on_input(action)`（vtable +0x50，状态实参 `{0,0}`）；首个
    ///    返回 true 即记 `handled=1` 并跳到第 3 步。
    /// 3. 反向遍历 focus→root：状态实参 `{1, handled}`；首个 true 即
    ///    `handled=1` 返回。
    /// 4. 返回 `handled`。
    ///
    /// Capture truncates the bubbling prefix to the handler, as at140688386;
    /// every callback receives the original two-byte phase state.
    pub fn process_input_action_handlers(
        &mut self,
        focus_id: GlobalId,
        action: &ActionSetAction,
    ) -> bool {
        let chain = find_node_path(&self.parents, focus_id);

        let mut handled = false;
        let mut bubble_len = chain.len();
        for (index, id) in chain.iter().enumerate() {
            if self.parents.is_empty() {
                break;
            }
            if let Some(node) = self.nodes.get_mut(id) {
                if node.content.on_input_phase(action, [false, false]) {
                    handled = true;
                    bubble_len = index + 1;
                    break;
                }
            }
        }
        for id in chain[..bubble_len].iter().rev() {
            if self.parents.is_empty() {
                break;
            }
            if let Some(node) = self.nodes.get_mut(id) {
                if node.content.on_input_phase(action, [true, handled]) {
                    handled = true;
                    break;
                }
            }
        }
        handled
    }

    /// 帧首清空父子链表（O：mei_pre_update 开头 0x1406845ac..0x14068460a 段对
    /// `mei+0x1d0..+0x1e8` 的 hashbrown `clear`：ctrl 字节 0xff + len=0，
    /// 容量字段保留）。**判等**。
    pub fn frame_begin(&mut self) {
        self.parents.clear();
    }

    /// 重建父子链（O：mei_pre_update 0x14068463b..0x140684709 段）：
    /// 遍历节点表，对每个节点的 `children` 逐项 `parents.insert(child, node_id)`。
    /// **判等**（遍历顺序 = HashMap 桶序，重复子项时后者覆盖，U）。
    pub fn build_parent_links(&mut self) {
        let links: Vec<(GlobalId, GlobalId)> = self
            .nodes
            .iter()
            .flat_map(|(id, node)| node.children.iter().map(move |child| (*child, *id)))
            .collect();
        for (child, parent) in links {
            self.parents.insert(child, parent);
        }
    }
}

/// Original root constructors return an actual node context. The former
/// SafeFrame projection incorrectly interpreted its first pointer as only Vec.
pub type SafeFrame = NodeContext;

/// 空内容（默认 NodeContent；对应 0x3bb70/0x3bb80 空壳默认实现）。
#[derive(Clone, Debug)]
pub struct EmptyContent;

impl NodeContent for EmptyContent {
    fn is_empty(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------
// NodeContext
// ------------------------------------------------------------------

/// 节点上下文 `(&MeiContext, GlobalId, u64)`（O：NodeContext+0x00 = mei 指针、
/// +0x08 = id）。mei 为**指向 MeiContext 的指针**（镜像原二进制裸指针语义，
/// FIX 3）；方法经 `unsafe { &mut *self.mei }` 重借调用链持有的同一帧内
/// `&mut MeiContext`。
#[derive(Clone, Copy, Debug)]
pub struct NodeContext {
    /// &MeiContext 指针（原 NodeContext+0x00）。
    pub mei: *mut MeiContext,
    /// 节点 id（原 +0x08）。
    pub id: GlobalId,
    /// 第3字段（原+0x10）：匿名space节点计数，普通上下文初始为0。
    pub _pad: u64,
}

impl NodeContext {
    /// 从 (mei, id) 构造（expect_child_with_global_id 0x140686530：仅构造，
    /// 断言在调用侧）。**判等**。
    pub fn new(mei: &mut MeiContext, id: GlobalId) -> NodeContext {
        NodeContext { mei, id, _pad: 0 }
    }

    /// 重借 mei 指针为 &mut MeiContext（调用链持有同一帧内 &mut MeiContext）。
    pub(crate) fn mei_mut(&self) -> &mut MeiContext {
        unsafe { &mut *self.mei }
    }

    /// 重借 mei 指针为 &MeiContext（只读）。
    pub(crate) fn mei_ref(&self) -> &MeiContext {
        unsafe { &*self.mei }
    }

    /// O 0x140681d50 `node(&self)`：探表返 &Node，缺失 **expect_failed** panic
    /// （FIX 2：原为 expect，非 unwrap；expect 串 U）。**判等**。
    pub fn node(&self) -> &Node {
        self.mei_ref()
            .nodes
            .get(&self.id)
            .expect("NodeContext::node: node missing")
    }

    /// O 0x140681e50 `node_mut(&self)`：探表返 &mut Node，缺失 unwrap_failed panic。
    /// **判等**。
    pub fn node_mut(&self) -> &mut Node {
        self.mei_mut().nodes.get_mut(&self.id).unwrap()
    }

    /// Original typed-content consumer: initialize an empty owner only when
    /// require_existing=false; a different existing concrete type is an error.
    /// Any performs real checked downcasting without native vtable imitation.
    pub fn get_content_mut_impl<T: crate::mei::widgets::DefaultNodeContent>(
        &self,
        require_existing: bool,
    ) -> &mut T {
        let mei = self.mei_mut();
        let fonts = mei.font_table.clone();
        let node = mei
            .nodes
            .get_mut(&self.id)
            .expect("MEI content node missing");
        if node.content.is_empty() {
            assert!(!require_existing, "MEI required content is missing");
            node.content = Box::new(T::from_font_table(fonts));
        }
        let any: &mut dyn std::any::Any = node.content.as_mut();
        any.downcast_mut::<T>()
            .expect("MEI node content has a different concrete type")
    }

    /// Original `content_mut` (inline, 223 instances): mutable typed content,
    /// initializing an empty owner as `get_content_mut_impl(false)`.
    pub fn content_mut<T: crate::mei::widgets::DefaultNodeContent>(&self) -> &mut T {
        self.get_content_mut_impl::<T>(false)
    }

    /// Original `get_content_impl` (inline, 12 instances; deps:
    /// `downcast_ref` + `Option::expect`): read-only typed content.
    pub fn get_content_impl<T: crate::mei::widgets::DefaultNodeContent>(&self) -> &T {
        let any: &dyn std::any::Any = self.node().content.as_ref();
        any.downcast_ref::<T>()
            .expect("MEI node content has a different concrete type")
    }

    /// Original `child_with_content` (inline, 119 instances; direct deps:
    /// `NodeContext::child` + `get_content_mut_impl`): named child with a
    /// typed content owner initialized when absent.
    pub fn child_with_content<T: crate::mei::widgets::DefaultNodeContent>(
        &self,
        name: &str,
    ) -> NodeContext {
        let child = self.child(name);
        child.get_content_mut_impl::<T>(false);
        child
    }

    /// Original `into_child` (inline, 4 instances): same construction as
    /// `into_child_with_global_id` for a computed child id.
    pub fn into_child(&self, child: GlobalId) -> NodeContext {
        self.into_child_with_global_id(child)
    }

    /// O 0x1406861b0 `child_with_global_id(child)`：先 `reuse_or_create_node(child)`
    /// 创建/取得子节点；再探**父**节点表（self.id），**缺失→unwrap_failed panic**
    /// （FIX 1：不静默创建父节点），命中把子 id append 到父 children；返回
    /// `NodeContext{mei, child}`。**判等**（mei 指针类型见 FIX 3）。
    pub fn child_with_global_id(&self, child: GlobalId) -> NodeContext {
        self.into_child_with_global_id(child)
    }

    /// O 0x140686300 `into_child_with_global_id(child)`：与 child_with_global_id
    /// 同构（同一物理函数族）。**判等**。
    pub fn into_child_with_global_id(&self, child: GlobalId) -> NodeContext {
        self.mei_mut().reuse_or_create_node(child);
        self.mei_mut()
            .nodes
            .get_mut(&self.id)
            .unwrap()
            .children
            .push(child);
        NodeContext {
            mei: self.mei,
            id: child,
            _pad: 0,
        }
    }

    /// O 0x140686450 `has_global_id(id)`：探表返 bool（AL；命中 true，探空 false）。
    /// **判等**。
    pub fn has_global_id(&self, id: GlobalId) -> bool {
        self.mei_ref().nodes.contains_key(&id)
    }

    /// O 0x140686530 `expect_child_with_global_id(id)`：仅构造（断言调用侧）。
    /// **判等**。
    pub fn expect_child_with_global_id(&self, id: GlobalId) -> NodeContext {
        NodeContext {
            mei: self.mei,
            id,
            _pad: 0,
        }
    }

    /// O 0x140686550 `layout(&self, style)`：写本节点样式（Node+0x18..+0x60）。
    /// **判等**。
    pub fn layout(&self, style: crate::mei::layout::LayoutStyle) {
        self.node_mut().style = style;
    }

    /// O 0x140686690 `NodeContext::ignore_input(&self)`：透传 Node::ignore_input。
    /// **判等**。
    pub fn ignore_input(&self) {
        self.node_mut().ignore_input();
    }

    /// O 0x140686790 `NodeContext::with_nav(&self, nav)`：透传 Node::with_nav。
    /// **判等**。
    pub fn with_nav(&self, nav: bool) {
        self.node_mut().with_nav(nav);
    }

    /// B140683ce0: checked node lookup, then state_bits &1 in AL.
    pub fn hover(&self) -> bool {
        self.node().hover()
    }

    /// B140683dd0: checked node lookup, then the complete interaction byte.
    pub fn interaction(&self) -> u8 {
        self.node().state_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O：mei_pre_update 滚轮派发事件（tag 0xc3，载荷 = 累计滚轮 Vec2）。
    #[test]
    fn scroll_input_event_tag_and_payload() {
        let ev = MeiInputEvent::scroll(Vec2::new(1.5, -2.25));
        assert_eq!(ev.tag(), 0xc3);
        assert_eq!(ev.payload_vec2(), Vec2::new(1.5, -2.25));
    }

    /// O：帧首父子链表清空（mei_pre_update 0x1406845ac 段）。
    #[test]
    fn frame_begin_clears_parent_table() {
        let mut mei = MeiContext::default();
        mei.parents.insert(GlobalId(7), GlobalId(9));
        assert_eq!(mei.parents.len(), 1);
        mei.frame_begin();
        assert!(mei.parents.is_empty());
    }

    /// O：父子链由 children 反向建立（child → parent）。
    #[test]
    fn build_parent_links_maps_child_to_parent() {
        let mut mei = MeiContext::default();
        let mut parent = Node::new(Box::new(EmptyContent));
        parent.children = vec![GlobalId(1), GlobalId(2)];
        mei.nodes.insert(GlobalId(9), parent);
        mei.build_parent_links();
        assert_eq!(mei.parents.get(&GlobalId(1)), Some(&GlobalId(9)));
        assert_eq!(mei.parents.get(&GlobalId(2)), Some(&GlobalId(9)));
        assert_eq!(mei.parents.get(&GlobalId(9)), None);
    }

    #[test]
    fn roots_enqueue_once_per_frame_and_recycle_content_and_geometry() {
        let mut mei = MeiContext::default();
        for name in ["root-a", "root-b", "root-c", "root-d"] {
            let ctx = mei.safe_frame(name, false);
            ctx.node_mut().size = Vec2::new(40., 30.);
            ctx.node_mut().pos = Vec2::new(6., 7.);
            ctx.node_mut().state_bits = NodeStateBits::HOVER;
            ctx.node_mut().flags = 0xff;
            mei.safe_frame(name, false);
        }
        assert_eq!(mei.frame_events.len(), 4);
        mei.recycle_frame_nodes();
        assert!(mei.nodes.is_empty());
        assert_eq!(mei.previous_nodes.len(), 4);
        let node = mei.safe_frame("root-a", false);
        assert_eq!(node.node().created, 1);
        assert_eq!(node.node().previous_layout.pos, Vec2::new(6., 7.));
        assert_eq!(node.node().previous_layout.size, Vec2::new(40., 30.));
        assert_eq!(node.node().flags, 0xf3);
        assert_eq!(node.node().state_bits, NodeStateBits::HOVER);
    }
    #[test]
    fn frame_events_drive_root_anchor_and_child_minimum_layout() {
        use crate::mei::layout::{Justify, LengthPercentage};
        let mut mei = MeiContext::default();
        let root = mei.new_at_pos("root", Vec2::new(100., 80.), None, Vec2::ZERO, false);
        root.node_mut().style.justify = Justify(0);
        let child = root.child_with_global_id(GlobalId(17));
        child.node_mut().intrinsic_size = [
            LengthPercentage::absolute(40.),
            LengthPercentage::absolute(20.),
        ];
        child.node_mut().local_z = 2.;
        let ignored = GlobalId(99);
        mei.reuse_or_create_node(ignored).local_z = 50.;
        mei.perform_layout(Vec2::new(1600., 1000.));
        assert_eq!(mei.z_order, vec![root.id, child.id]);
        assert_eq!(root.node().size, Vec2::new(40., 20.));
        assert_eq!(root.node().pos, Vec2::new(80., 70.));
        assert_eq!(child.node().pos, Vec2::new(80., 70.));
        assert_eq!(child.node().z, 4.);
    }

    /// O: find_node_path walks child→parent then reverses to root→focus.
    #[test]
    fn find_node_path_reverses_parent_chain() {
        let mut parents = std::collections::HashMap::new();
        parents.insert(GlobalId(3), GlobalId(2));
        parents.insert(GlobalId(2), GlobalId(1));
        assert_eq!(
            find_node_path(&parents, GlobalId(3)),
            vec![GlobalId(1), GlobalId(2), GlobalId(3)]
        );
        assert_eq!(find_node_path(&parents, GlobalId(1)), vec![GlobalId(1)]);
    }

    /// O: fullscreen enqueues FullWindow; the without_block_input variant
    /// leaves FORCE_INTERACTIVE clear.
    #[test]
    fn fullscreen_variants_share_root_creation() {
        let mut mei = MeiContext::default();
        let blocking = mei.fullscreen("menu");
        assert_eq!(blocking.node().flags, NodeFlags::FORCE_INTERACTIVE);
        let passive = mei.fullscreen_without_block_input("menu-passive");
        assert_eq!(passive.node().flags, 0);
        assert_eq!(mei.frame_events.len(), 2);
        assert!(
            mei.frame_events
                .iter()
                .all(|event| event.kind == FrameKind::FullWindow)
        );
    }

    /// O: child_with_content = child(typed id) + content_mut initialization.
    #[test]
    fn child_with_content_initializes_typed_owner_once() {
        use crate::mei::widgets::FrameContent;
        let mut mei = MeiContext::default();
        let root = mei.safe_frame("root", false);
        let child = root.child_with_content::<FrameContent>("frame");
        assert_eq!(root.node().children, vec![child.id]);
        let first = child.content_mut::<FrameContent>() as *mut FrameContent;
        let second = child.content_mut::<FrameContent>() as *mut FrameContent;
        assert_eq!(first, second);
        assert_eq!(
            child.get_content_impl::<FrameContent>().type_name(),
            "toolbar::mei::widgets::FrameContent"
        );
    }
}
