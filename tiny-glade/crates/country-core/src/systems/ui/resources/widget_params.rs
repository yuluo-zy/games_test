//! `widget_params.rs` — WidgetParams 0xb0 B 参数块（B 0x1409a8e10 起，O）。
//!
//! 证据：`chapters/systems/ui/resources/widget_params.rs.md`、`ui-input/DIGEST.md` §2.4：
//! - 默认 u32 组 `{0, 8, 0, 1, 0, 2, 3, 3}` @ `{0, 8, 0x10, 0x18, 0x20, 0x30, 0x48, 0x60}`；
//! - `UiSounds` 4×24 B @0x18..0x78；原volume为32字节枚举，字段因variant而异：
//!   canvas的layer@1/size@4，sphere的radius@4/XYZ@8（byte1是padding）；
//!   当前逻辑类型保留旧canvas字段，新增sphere center，不强行复制native union ABI；
//! - u16 @0xa8 = 0x101；flags 字节 @0xa9（**bit0 = record_history_when_used、bit2 =
//!   allow_negative_distance**；默认 flags=0x01；`no_history_recording` 清 bit0——
//!   2026-10-17 fresh 反编译纠错，早前 RESTORE 记反）；`occlude_by_world` = 字节 @0xa8
//!   整字节（review-audit 纠错，非 bit0）；
//! - `UiFocus` u32 {None=0, Toolbar=1, Mei=2, World=3}；
//! - `clearing_canvas` volume = {tag 4, layer, 130×130}（`0x43020000_43020000` 双 130.0）。
//!
//! 组 A restorer 接线：`PlayUiSound`（`add_ui` 0x140945c60 注册的 2 事件之一；
//! system_color/system_glade_border/system_tree 消费）与 `UiNavState`（同上插入的
//! 资源）落位本模块。
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// Original PlayUiSound business payload is 24B. UiSounds stores four such
/// values. Native Events<EventInstance<PlayUiSound>> appends its own 8B EventId;
/// that counter is not part of the game event and must not be stored here.
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayUiSound {
    pub kind: u32,
    /// Native +4..24, including nested enum tags at +4. Never drop this word.
    pub _payload: [u8; 20],
}
impl PlayUiSound {
    pub fn from_native_slot(slot: [u8; 24]) -> Self {
        Self { kind: u32::from_le_bytes(slot[..4].try_into().unwrap()),
            _payload: slot[4..].try_into().unwrap() }
    }
    pub fn native_slot(self) -> [u8; 24] {
        let mut slot = [0; 24];
        slot[..4].copy_from_slice(&self.kind.to_le_bytes());
        slot[4..].copy_from_slice(&self._payload);
        slot
    }
    pub fn variant4() -> Self { Self { kind: 4, ..Default::default() } }
}


/// Original camera-mode sound notification (B1409d709a): byte0 selects
/// FirstPersonOn, nonzero selects FirstPersonOff. This is not OS window focus.
#[repr(transparent)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayPhotomodeSound(pub u8);

/// UI 导航状态（O：`add_ui` 0x140945c60 `insert_resource<UiNavState>`）。
///
/// 原生 **0x38 B**，2026-10-17 fresh 反编译/机器码（B patched, shared worker）逐字段：
/// - `+0x00`：焦点暂存槽（16 B，`UiFocus` 同形）。`render_taffy_ui_to_widgets`
///   （0x142338990）在 `focus==Toolbar` 且输入按下/光标位移 > 1 时，把 `+0x10`
///   的 16 B 拷到 `+0x00` 并把 `+0x10` 清零；`ui_nav_update`（0x14233eed0）
///   在另一分支把 `+0x00` 的 16 B 提升回 `+0x10`。语义为两个焦点槽的交换/暂存（名 I）。
/// - `+0x10`：当前焦点 `UiFocus`（tag u32@0x10、pad@0x14、payload/id u64@0x18）。
///   `mei_pre_update` 0x1406842e0 `cmp dword [nav+0x10],2` / `mov rax,[nav+0x18]`；
///   `ui_nav_update` 0x14233eed0 写 `[nav+0x10]=1` + `[nav+0x18]=payload`（Toolbar），
///   清焦点写 `qword [nav+0x10]=0`。
/// - `+0x20`：`Option<Instant>`（16 B，std Windows `Timespec { secs i64 @0x20,
///   nsec u32 @0x28, pad @0x2c }`；`None` niche = nsec `1_000_000_000`）。
///   `mei_pre_update`：`call Instant::now; mov [nav+0x20],rax; mov [nav+0x28],edx`；
///   随后 `cmp edx,0x3b9aca00` 判 `None`，否则 `Instant::elapsed` 取时长。
/// - `+0x30`：`hold` 字节。`retain<ActionSetAction, closure_env$2`（0x140fcdae0）
///   以 `movzx r15d, byte [r8]` 捕获该字节；`mei_post_update` 0x1406884c0
///   `cmp byte [nav+0x30],1`。
/// - `+0x31..0x38`：对齐填充。
///
/// `hold` 的更新语义（0x1406849bd..0x140684a45 逐指令）：
/// `if 有导航动作 just_pressed { nav.timer = Some(Instant::now()) }`
/// `nav.hold = match nav.timer { Some(t) => t.elapsed().as_secs() < 120, None => false }`。
#[repr(C)]
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct UiNavState {
    /// +0x00：焦点暂存/交换槽（语义见类型注释，名 I）。
    pub staged_focus: UiFocus,
    /// +0x10：当前焦点（tag@0x10 / id@0x18）。
    pub focus: UiFocus,
    /// +0x20：`Option<Instant>`（None niche = nsec `1e9`）。
    pub timer: Option<std::time::Instant>,
    /// +0x30：hold 字节（非零为保持）。
    pub hold: bool,
    /// +0x31：对齐填充。
    _pad: [u8; 7],
}

impl UiNavState {
    /// `UiFocus::as_mei` 语义的焦点 id（`+0x10/+0x18`）。
    pub fn focused_id(&self) -> Option<u64> {
        self.focus.as_mei()
    }

    /// 0x1406849bd..0x140684a45：`flag` = 本帧是否有导航动作 just_pressed。
    ///
    /// `if flag { timer = Some(Instant::now()) }`；
    /// `hold = timer.map_or(false, |t| t.elapsed().as_secs() < 120)`。
    pub fn update_hold(&mut self, nav_action_pressed: bool) {
        if nav_action_pressed {
            self.timer = Some(std::time::Instant::now());
        }
        self.hold = self
            .timer
            .map(|t| t.elapsed().as_secs() < 120)
            .unwrap_or(false);
    }
}

/// 默认 u32 组（O：0x1409a8e10 默认值序列）。
pub const DEFAULT_U32S: [u32; 8] = [0, 8, 0, 1, 0, 2, 3, 3];

/// 焦点（O：16 B `{ tag: u32 @0, pad @4, payload: u64 @8 }`）。
///
/// 2026-10-17 fresh：`as_mei` 机器码 `xor eax,eax; cmp dword [rcx],2; setz al;
/// mov rdx,[rcx+8]; ret` 返回 `Option<u64>`（判别在 RAX、载荷在 RDX）——**必须**保留
/// +0x8 载荷，旧 crate 的 tag-only enum 无法承载该数据流。判别值语义不变：
/// {None=0, Toolbar=1, Mei=2, World=3}。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiFocus {
    /// @+0x0：判别（0..=3）。
    pub tag: u32,
    /// @+0x4：填充。
    pub _pad: u32,
    /// @+0x8：Mei 变体载荷（`as_mei` 原样返回）。
    pub payload: u64,
}

impl UiFocus {
    pub const NONE: Self = Self { tag: 0, _pad: 0, payload: 0 };
    pub const TOOLBAR: Self = Self { tag: 1, _pad: 0, payload: 0 };
    pub const MEI: Self = Self { tag: 2, _pad: 0, payload: 0 };
    pub const WORLD: Self = Self { tag: 3, _pad: 0, payload: 0 };

    pub const fn new(tag: u32) -> Self {
        Self { tag, _pad: 0, payload: 0 }
    }

    /// 0x140985f30：`== 0`（VA 与 `AllocationHandle::is_null` 同址折叠；语义平凡匹配）。
    pub fn is_none(&self) -> bool {
        self.tag == 0
    }
    /// 0x1409a9150：`== 1`。
    pub fn is_toolbar(&self) -> bool {
        self.tag == 1
    }
    /// 0x1409a9140：`== 2`（与 WelcomeScreenState::is_closed 同址折叠，语义巧合）。
    pub fn is_mei(&self) -> bool {
        self.tag == 2
    }
    /// 0x1409a9130：**条件返回**（O 机器码 `xor eax,eax; cmp [rcx],2; sete al;
    /// mov rdx,[rcx+8]; ret`）。语义 = `(tag==2).then(|| payload)`：
    /// `Option<u64>` 的判别在 RAX、载荷在 RDX；非 Mei 时返回 None。
    pub fn as_mei(&self) -> Option<u64> {
        if self.tag == 2 {
            Some(self.payload)
        } else {
            None
        }
    }
}

/// Widget volume payload. Retain existing canvas tag/layer/size while adding
/// the original sphere's world-space center. `size.x` carries sphere radius;
/// `layer` is only meaningful for variants whose original producer writes it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WidgetVolumeShape {
    pub tag: u8,
    pub layer: u8,
    pub size: Vec2,
    pub center: Vec3,
}

impl WidgetVolumeShape {
    /// B ui_continue_wall: tag0, radius@+4, XYZ@+8. Native byte1 is padding.
    pub fn sphere(center: Vec3, radius: f32) -> Self {
        Self { tag: 0, layer: 0, size: Vec2::new(radius, 0.0), center }
    }

    /// 0x1409a9110：`clearing_canvas` = {tag 4, layer, 130×130}。
    pub fn clearing_canvas(layer: u8) -> Self {
        Self {
            tag: 4,
            layer,
            size: Vec2::splat(130.0),
            center: Vec3::ZERO,
        }
    }
}

/// 声音槽（4×24 B @0x18..0x78；O 布局）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiSounds {
    pub slots: [[u8; 24]; 4],
}

impl UiSounds {
    /// 原 WidgetParams::new 1409a8e30..4d 写入的四个声音变体。
    /// 与 empty 不同；保留这一区别，真实 ui_update 才能发出 hover/抓取/释放声音。
    pub fn widget_default() -> Self {
        let mut sounds = Self::empty();
        for (slot, tag) in sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
            slot[..4].copy_from_slice(&tag.to_le_bytes());
        }
        sounds
    }
    /// 0x1409a8fb0：清 4 槽首 u32（偏移 0/0x18/0x30/0x48）。
    pub fn empty() -> Self {
        Self { slots: [[0; 24]; 4] }
    }
}

/// Widget 标志字节 @0xa9（O：bit0 = record_history_when_used、bit2 = allow_negative_distance；
/// 默认 `0x01` 即 bit0 置位）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WidgetFlags(pub u8);

impl WidgetFlags {
    /// bit0（O：默认置位；`no_history_recording` **清位**）。
    pub const RECORD_HISTORY_WHEN_USED: u8 = 1 << 0;
    /// bit2（O：`allow_negative_distance(bool)` 带参设置）。
    pub const ALLOW_NEGATIVE_DISTANCE: u8 = 1 << 2;

    /// 0x1409a8f10：字节 @0xa9 bit0 是否已清（即 no-history-recording 生效）。
    pub fn no_history_recording(&self) -> bool {
        self.0 & Self::RECORD_HISTORY_WHEN_USED == 0
    }
    /// 0x1409a8f40：字节 @0xa9 bit2。
    pub fn allow_negative_distance(&self) -> bool {
        self.0 & Self::ALLOW_NEGATIVE_DISTANCE != 0
    }
}

/// Complete native volume discriminants from closure2@140ac3f50's eight-entry
/// jump table142adfb28. Keep original owned Vec payloads and all scalar fields.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetGeometry {
    Sphere {
        center: Vec3,
        radius: f32,
    },
    SphereCloud {
        centers: Vec<Vec3>,
        radius: f32,
    },
    TriangleMesh(Vec<utils::geometry::intersect::Triangle>),
    ScreenAabb {
        min: Vec2,
        max: Vec2,
        distance: f32,
    },
    WorldCanvas {
        extent: Vec2,
        layer: u8,
    },
    TerrainCanvas {
        distance: f32,
        layer: u8,
    },
    PlaneDisc {
        point: Vec3,
        normal: Vec3,
        radius: f32,
    },
    Empty,
}

impl WidgetGeometry {
    pub fn native_tag(&self) -> u8 {
        match self {
            Self::Sphere { .. } => 0,
            Self::SphereCloud { .. } => 1,
            Self::TriangleMesh(_) => 2,
            Self::ScreenAabb { .. } => 3,
            Self::WorldCanvas { .. } => 4,
            Self::TerrainCanvas { .. } => 5,
            Self::PlaneDisc { .. } => 6,
            Self::Empty => 7,
        }
    }
}

/// 0xb0 B 参数块（O：默认 u32 组 + UiSounds + WidgetVolumeShape + u16@0xa8=0x101 +
/// flags@0xa9）。
#[derive(Clone, Debug)]
pub struct WidgetParams {
    /// Original captured widget owner of named signifier factory callbacks.
    pub signifiers: Vec<crate::systems::ui::signifiers::SignifierFactory>,
    /// Complete owned geometry for native volume variants.
    pub geometry: Option<WidgetGeometry>,
    /// 默认 u32 组（@0/8/0x10/0x18/0x20/0x30/0x48/0x60）。
    pub fields: [u32; 8],
    /// 声音槽（@0x18..0x78）。
    pub sounds: UiSounds,
    /// Native +0x78..0x88 brush-preview enum, preserved bit for bit:
    /// tag0=None, tag1=kind with cursor fallback, tag2=kind plus Vec3 offset.
    /// ui_update applies the full geometry-specific origin before publication.
    pub extra_78: [u64; 2],
    /// 音量形状（@0x88..0xa8；O：`new` 第一参数 32 B 整块拷入）。
    pub volume: WidgetVolumeShape,
    /// @0xa8：u16 = 0x101 + 整字节 occlude_by_world（O：review-audit 纠错）。
    pub occlude_by_world: bool,
    /// @0xa9 flags 字节。
    pub flags: WidgetFlags,

}

impl Default for WidgetParams {
    fn default() -> Self {
        Self::new(WidgetVolumeShape::default(), [0; 2])
    }
}

impl WidgetParams {
    pub fn with_signifiers(mut self, signifiers: Vec<crate::systems::ui::signifiers::SignifierFactory>) -> Self {
        self.signifiers = signifiers;
        self
    }

    /// Native tag3 stores the complete screen-space min/max rectangle.
    pub fn screen_aabb(mut self, min: Vec2, max: Vec2, distance: f32) -> Self {
        self.volume.tag = 3;
        self.geometry = Some(WidgetGeometry::ScreenAabb { min, max, distance });
        self
    }

    pub fn geometry(mut self, geometry: WidgetGeometry) -> Self {
        self.volume.tag = geometry.native_tag();
        self.geometry = Some(geometry);
        self
    }

    /// Older scalar producers encode complete sphere/canvas variants in their
    /// existing public fields. Never fabricate a missing Vec/plane/AABB.
    pub fn resolved_geometry(&self) -> std::borrow::Cow<'_, WidgetGeometry> {
        if let Some(geometry) = &self.geometry {
            assert_eq!(self.volume.tag, geometry.native_tag());
            return std::borrow::Cow::Borrowed(geometry);
        }
        std::borrow::Cow::Owned(match self.volume.tag {
            0 => WidgetGeometry::Sphere {
                center: self.volume.center,
                radius: self.volume.size.x,
            },
            4 => WidgetGeometry::WorldCanvas {
                extent: self.volume.size,
                layer: self.volume.layer,
            },
            5 => WidgetGeometry::TerrainCanvas {
                distance: self.volume.size.x,
                layer: self.volume.layer,
            },
            7 => WidgetGeometry::Empty,
            tag => panic!("native UI volume tag{tag} requires its full owned payload"),
        })
    }


    /// 0x1409a8e10（O，fresh 机器码）：
    /// - `RCX` = 返回块（0xb0 B）；`[RDX..RDX+0x20]` 整块拷到 `+0x88..0xa8`（volume）；
    ///   `[R8..R8+0x10]` 拷到 `+0x78..0x88`（extra_78）；
    /// - 默认 u32 组 {0,8,0,1,0,2,3,3} 写 `{0,8,0x10,0x18,0x20,0x30,0x48,0x60}`；
    /// - `word@+0xa8 = 0x101` → occlude_by_world byte = 0x01（默认 true）、
    ///   flags byte = 0x01（bit0 = record_history_when_used 默认置位）。
    ///
    /// 原签名带两个按引用传递的聚合参数（32 B + 16 B），crate 以
    /// [`WidgetVolumeShape`] 与 16 B 原始 qword 承载。
    pub fn new(volume: WidgetVolumeShape, extra_78: [u64; 2]) -> Self {
        Self {
            signifiers: Vec::new(),
            fields: DEFAULT_U32S,
            sounds: UiSounds::widget_default(),
            extra_78,
            volume,
            occlude_by_world: true,
            flags: WidgetFlags(0x01),
            geometry: None,
        }
    }

    /// Volume variant tag 2: triangle mesh (original `wall_splitter` volume).
    pub fn mesh_volume(mut self, triangles: Vec<utils::geometry::intersect::Triangle>) -> Self {
        self.volume = WidgetVolumeShape {
            tag: 2,
            ..WidgetVolumeShape::default()
        };
        self.geometry = Some(WidgetGeometry::TriangleMesh(triangles));
        self
    }

    /// 兼容面：同时承接 sounds（O：`new` 之后由 `sounds()` 覆写 0x18..0x78）。
    pub fn new_with_sounds_and_volume(sounds: UiSounds, volume: WidgetVolumeShape) -> Self {
        Self {
            sounds,
            ..Self::new(volume, [0; 2])
        }
    }

    /// 0x1409a8e80：写 96 B 声音区。
    pub fn sounds(mut self, sounds: UiSounds) -> Self {
        self.sounds = sounds;
        self
    }

    /// 0x1409a8ee0：`occlude_by_world` = 字节 @0xa8 整字节（非 bit0）。
    pub fn occlude_by_world(mut self, value: bool) -> Self {
        self.occlude_by_world = value;
        self
    }

    /// 音量形状写入（O 面：create_decorator_widget 0x141282300 族 volume
    /// `{tag=5, layer=param_4}`；字节@0x78..0xa8）。
    pub fn volume(mut self, volume: WidgetVolumeShape) -> Self {
        self.volume = volume;
        self.geometry = None;
        self
    }

    /// 0x1409a8f10：`no_history_recording` = 字节 @0xa9 **清 bit0**（`&= 0xfe`）。
    pub fn no_history_recording(mut self) -> Self {
        self.flags.0 &= !WidgetFlags::RECORD_HISTORY_WHEN_USED;
        self
    }

    /// 0x1409a8f40：`allow_negative_distance(bool)` = 字节 @0xa9 **带参** `arg<<2 | byte&0xfb`。
    pub fn allow_negative_distance(mut self, enabled: bool) -> Self {
        self.flags.0 = (enabled as u8) << 2 | (self.flags.0 & 0xfb);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matches_native_defaults_and_copies_both_arguments() {
        let volume = WidgetVolumeShape {
            tag: 5,
            layer: 7,
            size: Vec2::new(1.5, 2.5),
            ..Default::default()
        };
        let params = WidgetParams::new(volume, [0x11, 0x22]);
        assert_eq!(params.fields, [0, 8, 0, 1, 0, 2, 3, 3]);
        // 原构造器真实写声音区；fields 只记录观察值，运行时读取的是 sounds。
        assert_eq!(params.sounds.slots.map(|slot| PlayUiSound::from_native_slot(slot).kind),
            [1, 2, 3, 3]);
        assert_eq!(&params.sounds.slots[0][8..16], &[0; 8]);
        assert_eq!(params.volume, volume);
        assert_eq!(params.extra_78, [0x11, 0x22]);
        // O: word@0xa8 = 0x101 → occlude byte 0x01 + flags 0x01。
        assert!(params.occlude_by_world);
        assert_eq!(params.flags, WidgetFlags(0x01));
    }

    #[test]
    fn flags_byte_matches_native_bit_ops() {
        // no_history_recording：byte &= 0xfe（清 bit0）。
        let cleared = WidgetParams::new(WidgetVolumeShape::default(), [0; 2]).no_history_recording();
        assert_eq!(cleared.flags.0, 0x00);
        // allow_negative_distance：arg<<2 | byte&0xfb（可置位/清位 bit2）。
        let on = WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
            .allow_negative_distance(true);
        assert_eq!(on.flags.0, 0x05);
        let off = on.allow_negative_distance(false);
        assert_eq!(off.flags.0, 0x01);
    }

    #[test]
    fn as_mei_returns_payload_only_for_tag_2() {
        let mei = UiFocus { tag: 2, _pad: 0, payload: 0xdead_beef };
        assert_eq!(mei.as_mei(), Some(0xdead_beef));
        assert_eq!(UiFocus::TOOLBAR.as_mei(), None);
        assert_eq!(UiFocus::NONE.as_mei(), None);
        // 与判等谓词一致。
        assert!(mei.is_mei());
        assert!(!mei.is_toolbar());
        assert!(!mei.is_none());
    }

    #[test]
    fn volume_shape_offsets_match_clearing_canvas_writes() {
        let v = WidgetVolumeShape::clearing_canvas(9);
        assert_eq!(v.tag, 4);
        assert_eq!(v.layer, 9);
        assert_eq!(v.size, Vec2::splat(130.0));
        let base = &v as *const _ as usize;
        assert_eq!(&v.tag as *const _ as usize - base, 0);
        assert_eq!(&v.layer as *const _ as usize - base, 1);
        assert_eq!(&v.size as *const _ as usize - base, 4);
    }

    /// 原生 0x38 B：`staged_focus@0x00`、`focus@0x10`、`Option<Instant>@0x20`、
    /// `hold@0x30`（fresh 0x1406842e0 / 0x142338990 / 0x14233eed0）。
    #[test]
    fn nav_state_offsets_match_native_0x38() {
        assert_eq!(core::mem::size_of::<UiNavState>(), 0x38);
        assert_eq!(core::mem::size_of::<UiFocus>(), 0x10);
        assert_eq!(core::mem::size_of::<Option<std::time::Instant>>(), 0x10);
        let s = UiNavState::default();
        let base = &s as *const _ as usize;
        assert_eq!(&s.staged_focus as *const _ as usize - base, 0x00);
        assert_eq!(&s.focus as *const _ as usize - base, 0x10);
        assert_eq!(&s.timer as *const _ as usize - base, 0x20);
        assert_eq!(&s.hold as *const _ as usize - base, 0x30);
    }

    #[test]
    fn nav_focus_id_and_hold_semantics() {
        let mut nav = UiNavState::default();
        assert_eq!(nav.focused_id(), None);
        assert!(!nav.hold);
        nav.focus = UiFocus {
            tag: 2,
            _pad: 0,
            payload: 42,
        };
        assert_eq!(nav.focused_id(), Some(42));
        // 有新导航动作 → timer=now → hold 在 120 s 内为真。
        nav.update_hold(true);
        assert!(nav.timer.is_some());
        assert!(nav.hold);
        // 无动作但 timer 仍新 → hold 保持真；清 timer → 假。
        nav.update_hold(false);
        assert!(nav.hold);
        nav.timer = None;
        nav.update_hold(false);
        assert!(!nav.hold);
    }
}
