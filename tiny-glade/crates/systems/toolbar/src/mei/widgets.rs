//! B LabelContent / FrameContent, identified through their actual content
//! vtables. See task-index work TG-22952476abbbac3806/meihost-20261018.
//! Public fields carry caller-produced text/shapes; no placeholder UI is made.
use crate::mei::{
    core::{NodeContent, NodeDrawContext, NodePrepareContext},
    layout::LengthPercentage,
};
use tg_text::{RawShapeStream, ShapeStream, TextFragment};
mod menu_button;
pub mod tweaker;
pub use menu_button::MenuButtonContent;

/// 原 TabSwitcherContent：只累积本帧切页增量，布局准备时清零。
#[derive(Default)]
pub struct TabSwitcherContent {
    pub delta: i32,
}
impl DefaultNodeContent for TabSwitcherContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self { Self::default() }
}
impl NodeContent for TabSwitcherContent {
    fn type_name(&self) -> &'static str { "toolbar::mei::widgets::TabSwitcherContent" }
    fn interactive(&self) -> bool { true }
    fn on_input_phase(&mut self, action: &country_core::input::action_set::ActionSetAction, phase: [bool;2]) -> bool {
        // 原 140680630：phase[0]为true的回程不处理，prev优先，增量按i32回绕。
        if phase[0] { return false; }
        let delta = if action.just_pressed(24) { -1 }
            else if action.just_pressed(25) { 1 } else { return false; };
        self.delta = self.delta.wrapping_add(delta);
        true
    }
    fn prepare_layout(&mut self, _: &mut NodePrepareContext<'_>) -> Option<[LengthPercentage;2]> {
        self.delta = 0;
        Some([LengthPercentage::absolute(0.);2])
    }
}

impl crate::mei::core::NodeContext {
    /// 原14068ca90只递增匿名子节点计数并创建空节点；两个浮点实参没有读指令。
    /// 保留原行为，不凭space名称补造尺寸。原+0x10是该计数器，不是恒零填充。
    pub fn space(&mut self, _width: f32, _height: f32) -> &mut Self {
        let name = self._pad.wrapping_add(0xcafebabe);
        self._pad = self._pad.wrapping_add(1);
        let local = crate::mei::core::calculate_hash(&(name, 0xa79b7268a2a968d9u64));
        self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local))));
        self
    }
    /// 原 14032dce0：具体类型参与身份散列，布局 expand，不以普通容器充替。
    pub fn tab_switcher(&self, name: &str) -> Self {
        let value = crate::mei::core::calculate_hash(&(name, ()));
        let local = crate::mei::core::calculate_hash(&(value, 0x332e25b8f90bd461u64));
        let child = self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local))));
        child.get_content_mut_impl::<TabSwitcherContent>(false);
        child.layout(crate::mei::layout::LayoutStyle::default().expand());
        child
    }
    pub fn tab_delta(&self) -> i32 { self.content_mut::<TabSwitcherContent>().delta }
}

/// 原 ActionHintContent：手柄动作源的轮换图标。无动作源时不生成替代标签。
pub struct ActionHintContent {
    pub action: u8,
    created: std::time::Instant,
    selected: Option<std::path::PathBuf>,
    image: Option<ImageContent>,
}
impl DefaultNodeContent for ActionHintContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self { action: 0, created: std::time::Instant::now(), selected: None, image: None }
    }
}
impl ActionHintContent {
    /// 原680983：elapsed.as_secs()/2 后对全部动作源数取模。
    fn select_path(&mut self, glyphs: &[String], elapsed: std::time::Duration) {
        let selected = if glyphs.is_empty() { None } else {
            Some(std::path::PathBuf::from(&glyphs[(elapsed.as_secs()/2 % glyphs.len() as u64) as usize]))
        };
        if self.selected != selected {
            self.image = selected.as_ref().map(|path| {
                let mut image = ImageContent::default();
                image.image = format!("file:{}",path.to_string_lossy());
                image.height = 32.;
                image
            });
            self.selected = selected;
        }
    }
}
impl NodeContent for ActionHintContent {
    fn type_name(&self) -> &'static str { "toolbar::mei::widgets::ActionHintContent" }
    fn prepare_layout(&mut self, ctx: &mut NodePrepareContext<'_>) -> Option<[LengthPercentage;2]> {
        let mut glyphs = Vec::new();
        if ctx.steam_client.is_some() {
            if let Some(input) = ctx.steam_input {
                if let Some(actions) = input.actions_by_type.get(&u16::from(self.action)) {
                    for action in actions {
                        // 原9e8da0把枚举的kind/index转换为查询码，低字节加1。
                        if let Some(paths) = input.get_action_origin_glyphs(
                            country_core::resources::steam::ActionOriginGlyphQuery(action.code().wrapping_add(1))) {
                            glyphs.extend(paths);
                        }
                    }
                }
            }
        }
        self.select_path(&glyphs, self.created.elapsed());
        self.image.as_mut().map_or(Some([LengthPercentage::absolute(0.);2]), |image| image.prepare_layout(ctx))
    }
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        if let Some(image) = self.image.as_mut() { image.draw(ctx); }
    }
}

impl crate::mei::core::NodeContext {
    /// 原 action_hint<u64>，沿用现有 NodeContext，具体控件类型参与身份。
    pub fn action_hint(&self, name: u64, action: u8) -> Self {
        let local = crate::mei::core::calculate_hash(&(name, 0x3a97789c06031ab2u64));
        let child = self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local))));
        child.get_content_mut_impl::<ActionHintContent>(false).action = action;
        child
    }

    /// 原 hotkey tooltip：独立相对根，位于按钮右下方的反向锚点，z=200。
    fn make_hotkey_tooltip(&self, action: u8) {
        if action == 18 { return; }
        let id = crate::mei::core::GlobalId(crate::mei::core::calculate_hash(&(self.id, action)) ^ crate::mei::core::GLOBAL_ID_MASK);
        let root = self.mei_mut().new_relative_with_id(id, self.id, None,
            glam::Vec2::new(0.,-16.),glam::Vec2::ONE,glam::Vec2::splat(-1.),false);
        root.node_mut().local_z = 200.;
        root.action_hint(self.id.0.wrapping_mul(0xcafebabe),action);
    }
}

/// 原 14067eba0 的成功路径缓存。按 App/纹理库隔离，避免原静态缓存中的
/// bindless 下标跨设备、跨测试宿主复用；缓存和实际纹理具有同一生命周期。
#[derive(bevy_ecs::prelude::Resource)]
pub struct FileImageCache {
    storage: std::sync::Arc<FileImageStorage>,
    images: std::collections::HashMap<String, (u32, glam::Vec2)>,
    reported_errors: std::collections::HashSet<String>,
}

/// MEI渲染快照也保留此owner；游戏World卸载不能使尚未开始的帧失去文件纹理槽位。
pub struct FileImageStorage {
    device: std::sync::Arc<dyn country_core::render::load_texture::boundary::TextureDevice + Send + Sync>,
    owned: std::sync::Mutex<Vec<country_core::rhapsody_types::Texture>>,
}

impl FileImageCache {
    pub fn new(device: std::sync::Arc<dyn country_core::render::load_texture::boundary::TextureDevice + Send + Sync>) -> Self {
        Self { storage: std::sync::Arc::new(FileImageStorage {device, owned: Default::default()}),
            images: Default::default(), reported_errors: Default::default() }
    }

    pub fn retained_textures(&self) -> std::sync::Arc<FileImageStorage> { self.storage.clone() }

    /// 原版只缓存成功结果；失败会重试，但同一错误只记录一次。
    fn resolve(&mut self, path: &str, textures: &mut country_core::asset_libraries::texture_library::AssetTextureLibrary) -> (u32, glam::Vec2) {
        if let Some(&cached) = self.images.get(path) { return cached; }
        let loaded = (|| -> anyhow::Result<_> {
            use anyhow::Context;
            log::debug!("Loading texture from file: {path}");
            let image = image::io::Reader::open(path).context("Failed to open image")?
                .decode().context("Failed to decode image")?.to_rgba8();
            let extent = [image.width(), image.height()];
            let desc = country_core::rhapsody_types::TextureDesc::new_2d(extent, 0x2b).sampled();
            let texture = self.storage.device.create_texture(path, &desc, image.as_raw())
                .context("Failed to create texture")?;
            let handle = textures.add(None, texture.clone());
            self.storage.owned.lock().unwrap_or_else(|poison|poison.into_inner()).push(texture);
            Ok((textures.handle_to_bindless_index(handle).0,
                glam::Vec2::new(extent[0] as f32, extent[1] as f32)))
        })();
        match loaded {
            Ok(cached) => { self.images.insert(path.to_owned(), cached); cached }
            Err(error) => {
                let message = format!("{path}: {error:#}");
                if self.reported_errors.insert(message.clone()) { log::error!("{message}"); }
                // 原 14067f6cf 的固定 NameHash，不能自造纯色后备纹理。
                let (handle, texture) = textures.get_handle_and_texture_by_name(
                    country_core::name_hash::NameHash(0x806c29bbf7c5030d))
                    .expect("原 MEI 文件图片后备纹理未加载");
                let extent = glam::Vec2::new(texture.width as f32, texture.height as f32);
                (textures.handle_to_bindless_index(handle).0, extent)
            }
        }
    }
}

impl Drop for FileImageStorage {
    fn drop(&mut self) {
        // TextureHost/rhapsody 的托管引用保留在途帧；这里只释放本缓存的宿主槽位。
        for texture in self.owned.get_mut().unwrap_or_else(|poison|poison.into_inner()) { self.device.release_texture(texture); }
    }
}

/// Native 0x260-byte owner: 600-byte TextFragment and packed color @0x258.
pub struct LabelContent {
    pub text: TextFragment,
    pub color: u32,
    /// Source adapter: this tg-text API needs the renderer when setting text.
    /// Apply caller-produced text at the original prepare boundary.
    pub pending_text: Option<String>,
}

impl NodeContent for LabelContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::LabelContent"
    }

    /// B 0x14067e510: prepare actual glyphs and return absolute text bounds.
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        menu_button::prepare_text(&mut self.text, &mut self.pending_text, ctx)
    }

    /// B 0x14067e560: tint is the original packed color scaled by opacity.
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let tint = tg_text::scale_u32_packed_srgba_color(self.color, ctx.opacity);
        self.text
            .render(ctx.text, ctx.center.to_array(), ctx.extent.to_array(), tint)
            .expect("render label text");
    }
}

/// Native two-Vec owner: original ShapeStream followed by cached encoded words.
#[derive(Default)]
pub struct FrameContent {
    pub shapes: ShapeStream,
    pub raw_shapes: RawShapeStream,
}

/// B PillButtonContent, size0x48: two encoded shape streams, an asset &str,
/// tint strength and an optional digital action (0x28 means absent).
pub struct PillButtonContent {
    background: RawShapeStream,
    foreground: RawShapeStream,
    pub image: String,
    /// Native `+0x40` (`strength`): inactive image tint, written by
    /// `PillButtonView::inactive_opacity` (O 0x1406814f0).
    pub strength: f32,
    /// 原版 +0x44 的提示投影；0x12 仅跳过提示建树，仍可绑定 ui_cancel。
    /// `pill_hotkey` 必须同步更新输入消费者使用的 action。
    pub hotkey: u8,
    /// 原版 +0x44 的输入投影；0x28 表示不绑定输入。
    pub action: u8,
}
impl DefaultNodeContent for PillButtonContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self {
            background: RawShapeStream::new(),
            foreground: RawShapeStream::new(),
            image: String::new(),
            strength: 1.,
            hotkey: 0x12,
            action: 0x28,
        }
    }
}
impl NodeContent for PillButtonContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::PillButtonContent"
    }
    fn interactive(&self) -> bool {
        true
    }
    fn process_input_action(&mut self, _: &crate::mei::core::MeiInputEvent) -> bool {
        false
    }
    fn pre_update(
        &mut self,
        actions: &country_core::input::action_set::MainActionSet,
        _: glam::Vec2,
        state: &mut u8,
    ) {
        if self.action != 0x28 && actions.digital[usize::from(self.action)].just_pressed() {
            *state |= 0x20;
        }
    }
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        self.background.clear();
        let hovered = *ctx.state & 1 != 0;
        let alpha = if hovered { 0xffffff40 } else { 0xffffff20 };
        self.background
            .append(
                &ShapeStream::new().fill_color(alpha).corner_radius(10.),
                &mut |name| ctx.textures.image_name_to_index(name).0,
            )
            .expect("pill background");
        self.foreground.clear();
        let mut shape = ShapeStream::new().image(
            self.image.as_str(),
            tg_text::shape_stream::ImageScaling::FitHeight,
        );
        if !hovered && self.strength < 1. {
            shape = shape.fill_color(0xffffff00 | (self.strength * 255.) as u32);
        }
        self.foreground
            .append(&shape, &mut |name| ctx.textures.image_name_to_index(name).0)
            .expect("pill image");
        Some([LengthPercentage::absolute(0.); 2])
    }
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let origin = ctx.center - ctx.extent * 0.5;
        ctx.text
            .rect(
                origin.to_array(),
                ctx.extent.to_array(),
                ctx.opacity,
                &self.background,
            )
            .expect("pill background draw");
        ctx.text
            .rect(
                origin.to_array(),
                ctx.extent.to_array(),
                ctx.opacity,
                &self.foreground,
            )
            .expect("pill foreground draw");
    }
}

impl NodeContent for FrameContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::FrameContent"
    }

    /// B 0x140680540: clear → append, using the real texture descriptor index.
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        self.raw_shapes.clear();
        self.raw_shapes
            .append(&self.shapes, &mut |name| {
                ctx.textures.image_name_to_index(name).0
            })
            .expect("encode frame shape stream");
        Some([LengthPercentage::default(); 2])
    }

    /// B 0x140680590: an empty ShapeStream emits nothing; rect accepts origin.
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        if !self.shapes.is_empty() {
            // Original impl$9::render calls RenderContext::draw_rect, which
            // converts center/extent to the rect origin.
            let (center, extent, opacity) = (ctx.center, ctx.extent, ctx.opacity);
            ctx.draw_rect(center, extent, opacity, &self.raw_shapes);
        }
    }
}

/// B get_content_mut_impl initializes the original concrete content using the
/// shared font table. This is a source-level ownership adapter, not a vtable ABI.
pub trait DefaultNodeContent: NodeContent + Sized {
    fn from_font_table(fonts: std::sync::Arc<tg_text::FontTable>) -> Self;
}
impl DefaultNodeContent for FrameContent {
    fn from_font_table(_fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self::default()
    }
}
impl DefaultNodeContent for LabelContent {
    /// Original140f9da6a uses48.0 and role0(Label), then white packed color.
    fn from_font_table(fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self {
            text: TextFragment::new(48., tg_text::FontFamily::Label, fonts)
                .expect("MEI label font role"),
            color: u32::MAX,
            pending_text: None,
        }
    }
}
impl LabelContent {
    pub fn set_text(&mut self, text: String) {
        self.pending_text = Some(text);
    }
}

/// B ImageContent (vtable142baf630)：命名资产与 file: 两条真实图片路径。
pub struct ImageContent {
    pub image: String,
    pub height: f32,
    pub fit_parent_height: bool,
    pub stretch_width: bool,
    /// Native `+0x44`: `ImageView::blur_extend` flag (O 0x14067eaf0).
    pub blur_extend: bool,
    pub color: u32,
    pub size: glam::Vec2,
    pub raw_shapes: RawShapeStream,
}
impl DefaultNodeContent for ImageContent {
    fn from_font_table(_fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self::default()
    }
}
impl Default for ImageContent {
    fn default() -> Self {
        Self {
            image: String::new(),
            height: 0.,
            fit_parent_height: false,
            stretch_width: false,
            blur_extend: false,
            color: u32::MAX,
            size: glam::Vec2::ZERO,
            raw_shapes: RawShapeStream::new(),
        }
    }
}
impl NodeContent for ImageContent {
    /// Original vtable142baf630+40 is14003bb70, the true-returning leaf.
    fn interactive(&self) -> bool {
        true
    }
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::ImageContent"
    }
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        let (index, extent) = if let Some(path) = self.image.strip_prefix("file:") {
            ctx.file_images.as_deref_mut().expect("MEI 文件图片设备未安装")
                .resolve(path, ctx.textures)
        } else {
            let (handle, texture) = ctx.textures.get_handle_and_texture_by_name(
                country_core::name_hash::NameHash::compute(&self.image))
                .unwrap_or_else(|| panic!("MEI image texture is missing: {}", self.image));
            let extent = glam::Vec2::new(texture.width as f32, texture.height as f32);
            (ctx.textures.handle_to_bindless_index(handle).0, extent)
        };
        self.size = extent * (self.height / extent.y);
        let scaling = if self.stretch_width || self.blur_extend {
            tg_text::ImageScaling::BlurExtend
        } else {
            tg_text::ImageScaling::Stretch
        };
        let shapes = ShapeStream::new()
            .image(index, scaling)
            .fill_color(self.color);
        self.raw_shapes.clear();
        self.raw_shapes
            .append(&shapes, &mut |_| {
                unreachable!("image index already resolved")
            })
            .expect("encode MEI image");
        Some(if self.fit_parent_height {
            [LengthPercentage::default(); 2]
        } else {
            self.size.to_array().map(LengthPercentage::absolute)
        })
    }
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let mut size = self.size;
        if self.fit_parent_height {
            size *= ctx.extent.y / size.y.max(1.);
        }
        if self.stretch_width {
            size.x = ctx.extent.x;
        }
        ctx.text
            .rect(
                (ctx.center - size * 0.5).to_array(),
                size.to_array(),
                ctx.opacity,
                &self.raw_shapes,
            )
            .expect("draw MEI image");
    }
}
impl crate::mei::core::NodeContext {
    /// Native140fa0130 and1406811c0. Returns the retained node; interaction
    /// bits0x10/0x20 are consumed by the original caller on the next UI pass.
    pub fn back_button(&self, name: &str) -> Self {
        use crate::mei::{
            core::{GlobalId, calculate_hash},
            layout::{LayoutStyle, LengthPercentage},
        };
        let value = calculate_hash(&(name, ()));
        let local = calculate_hash(&(value, 0xa6ddf7d2d741de4du64));
        let child = self.child_with_global_id(GlobalId(calculate_hash(&(self.id.0, local))));
        child.get_content_mut_impl::<PillButtonContent>(false).image = "menu/back".into();
        let node = child.node_mut();
        node.flags &= !0x20;
        node.set_sounds(0, 0x0d);
        node.style = LayoutStyle {
            size: LengthPercentage::absolute(50.),
            min: LengthPercentage::absolute(50.),
            align: 2,
            justify: crate::mei::layout::Justify(1),
            ..Default::default()
        };
        child
    }
    /// Original child<&str>: hash with the native u64 TypeId high word.
    pub fn child(&self, name: &str) -> Self {
        let value = crate::mei::core::calculate_hash(&(name, ()));
        let local = crate::mei::core::calculate_hash(&(value, 0xa79b7268a2a968d9u64));
        self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local)),
        ))
    }
    /// Original image<&str>140f9fca0 creates a distinct image-content identity.
    pub fn image(&self, name: &str, image: &str, height: f32) -> Self {        let value = crate::mei::core::calculate_hash(&(name, ()));
        let local = crate::mei::core::calculate_hash(&(value, 0x01f6d60c8858b8ffu64));
        let child = self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local)),
        ));
        let content = child.get_content_mut_impl::<ImageContent>(false);
        content.image = image.into();
        content.height = height;
        content.fit_parent_height = false;
        child
    }
    /// O 0x14067eaf0 `ImageView::blur_extend`: sets the native `ImageContent`
    /// byte at `+0x44` and returns the same node context.
    pub fn blur_extend(&self) -> Self {
        self.content_mut::<ImageContent>().blur_extend = true;
        *self
    }
}

#[cfg(test)]
mod file_image_tests {
    use super::*;
    use country_core::{
        asset_libraries::texture_library::{AssetTextureLibrary, DescriptorImageUpdate, ImageView, TextureLibraryDevice, TextureUpdate},
        render::load_texture::boundary::TextureDevice,
        rhapsody_contract::DescriptorSet,
        rhapsody_types::{BackendError, Texture, TextureDesc},
    };
    use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

    #[test]
    fn original_tab_switcher_routes_actions_once_and_retains_owner() {
        use country_core::input::action_set::{ActionSetAction,DigitalActionPhase,DigitalActionState};
        let mut mei=crate::mei::core::MeiContext::default();
        let root=mei.safe_frame("tabs",false);
        let tab=root.tab_switcher("settings");
        let page=tab.child("page");
        mei.build_parent_links();
        let action=|slot,phase|ActionSetAction::Digital {slot,state:DigitalActionState {phase,timer_seconds:0.}};
        assert!(mei.process_input_action_handlers(page.id,&action(24,DigitalActionPhase::JustPressed)));
        assert_eq!(tab.tab_delta(),-1);
        assert!(mei.process_input_action_handlers(page.id,&action(25,DigitalActionPhase::JustPressed)));
        assert_eq!(tab.tab_delta(),0);
        assert!(!mei.process_input_action_handlers(page.id,&action(25,DigitalActionPhase::Held)));
        assert!(!mei.process_input_action_handlers(page.id,&action(19,DigitalActionPhase::JustPressed)));
        let content=tab.content_mut::<TabSwitcherContent>();
        content.delta=i32::MAX;
        assert!(content.on_input_phase(&action(25,DigitalActionPhase::JustPressed),[false,true]));
        assert_eq!(content.delta,i32::MIN);
        assert!(!content.on_input_phase(&action(25,DigitalActionPhase::JustPressed),[true,false]));
        assert_eq!(root.tab_switcher("settings").id,tab.id);
        assert_eq!(tab.node().style.expand,1.);
    }

    #[test]
    fn original_action_hint_cycles_every_two_seconds_and_removes_empty_glyph() {
        let mut hint=ActionHintContent::from_font_table(Arc::new(Default::default()));
        let paths=vec!["/icons/a.png".into(),"/icons/b.png".into()];
        for (seconds,name) in [(0,"a"),(1,"a"),(2,"b"),(3,"b"),(4,"a"),(99,"b")] {
            hint.select_path(&paths,std::time::Duration::from_secs(seconds));
            assert_eq!(hint.image.as_ref().unwrap().image,format!("file:/icons/{name}.png"));
            assert_eq!(hint.image.as_ref().unwrap().height,32.);
        }
        hint.select_path(&[],std::time::Duration::from_secs(100));
        assert!(hint.image.is_none());
        assert!(hint.selected.is_none());
    }

    #[test]
    fn original_hotkey_tooltip_uses_action_keyed_relative_root_and_native_hint_identity() {
        use crate::mei::core::{calculate_hash,FrameKind,GLOBAL_ID_MASK,GlobalId,MeiContext};
        let mut mei=MeiContext::default();
        let button=mei.safe_frame("hint-test",false).pill_button("test","menu/back");
        let before=mei.nodes.len();
        button.pill_hotkey(18);
        assert_eq!(mei.nodes.len(),before);
        button.pill_hotkey(19);
        let root_id=GlobalId(calculate_hash(&(button.id,19u8))^GLOBAL_ID_MASK);
        let root=mei.nodes.get(&root_id).unwrap();
        assert_eq!(root.local_z,200.);
        assert!(mei.frame_events.iter().any(|event|event.id==root_id && matches!(event.kind,FrameKind::Relative {
            relative_to,offset,reference_anchor,anchor,..
        } if relative_to==button.id && offset==glam::Vec2::new(0.,-16.) && reference_anchor==glam::Vec2::ONE && anchor==glam::Vec2::splat(-1.))));
        let hint=button.mei_mut().new_relative_with_id(root_id,button.id,None,glam::Vec2::new(0.,-16.),glam::Vec2::ONE,glam::Vec2::splat(-1.),false)
            .action_hint(button.id.0.wrapping_mul(0xcafebabe),19);
        assert_eq!(hint.content_mut::<ActionHintContent>().action,19);
    }

    #[derive(Default)]
    struct Device {
        uploads: Mutex<Vec<(TextureDesc, Vec<u8>)>>,
        released: Mutex<Vec<u64>>,
        fail: AtomicBool,
    }
    impl TextureDevice for Device {
        fn create_texture(&self, _: &str, desc: &TextureDesc, data: &[u8]) -> Result<Texture, BackendError> {
            if self.fail.load(Ordering::Relaxed) { return Err(BackendError::OutOfDeviceMemory); }
            let mut uploads = self.uploads.lock().unwrap();
            uploads.push((*desc, data.to_vec()));
            Ok(Texture { id: uploads.len() as u64, width: desc.extent[0], height: desc.extent[1], format: desc.format })
        }
        fn release_texture(&self, texture: &Texture) { self.released.lock().unwrap().push(texture.id); }
    }
    impl TextureLibraryDevice for Device {
        fn create_persistent_descriptor_set(&self, _: &str) -> Result<DescriptorSet, BackendError> { Ok(Default::default()) }
        fn update_persistent_descriptor_set(&self, _: &DescriptorSet, _: &[DescriptorImageUpdate]) -> Result<(), BackendError> { Ok(()) }
        fn get_vk_image_view(&self, texture: &Texture, _: u32) -> Result<ImageView, BackendError> { Ok(ImageView(texture.id)) }
        fn update_texture(&self, _: &Texture, _: &TextureUpdate) -> Result<(), BackendError> { unreachable!("文件图片在创建时上传") }
    }

    #[test]
    fn original_file_decode_cache_retry_fallback_and_release() {
        let directory = std::env::temp_dir().join(format!("toolbar-images-{}-{}", std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("glyph.png");
        let device = Arc::new(Device::default());
        let mut library = AssetTextureLibrary::new(device.as_ref());
        // 原指令固定 NameHash 的真实打包资产名是 cloth_ferris，不能猜成error。
        assert_eq!(country_core::name_hash::NameHash::compute("cloth_ferris").0, 0x806c29bbf7c5030d);
        let fallback = library.add(Some("cloth_ferris".into()), Texture { id: 99, width: 8, height: 4, format: 43 });
        let fallback_index = library.handle_to_bindless_index(fallback).0;
        let mut cache = FileImageCache::new(device.clone());
        let name = path.to_str().unwrap();
        for _ in 0..2 {
            assert_eq!(cache.resolve(name, &mut library), (fallback_index, glam::Vec2::new(8.,4.)));
        }
        assert!(cache.images.is_empty());
        assert_eq!(cache.reported_errors.len(),1);
        std::fs::write(&path, b"not an image").unwrap();
        assert_eq!(cache.resolve(name, &mut library).0, fallback_index);
        let pixels = vec![255,0,0,255, 0,255,0,128];
        image::RgbaImage::from_raw(2,1,pixels.clone()).unwrap().save(&path).unwrap();
        device.fail.store(true, Ordering::Relaxed);
        assert_eq!(cache.resolve(name, &mut library).0, fallback_index);
        assert!(cache.images.is_empty());
        device.fail.store(false, Ordering::Relaxed);
        let loaded = cache.resolve(name, &mut library);
        assert_ne!(loaded.0, fallback_index);
        assert_eq!(loaded.1, glam::Vec2::new(2.,1.));
        std::fs::remove_file(&path).unwrap();
        assert_eq!(cache.resolve(name, &mut library), loaded);
        assert_eq!(device.uploads.lock().unwrap().as_slice(), &[(TextureDesc::new_2d([2,1],43).sampled(),pixels)]);
        assert_eq!(library.len(),2);
        let pending_frame=cache.retained_textures();
        drop(cache);
        assert!(device.released.lock().unwrap().is_empty());
        drop(pending_frame);
        assert_eq!(*device.released.lock().unwrap(), [1]);
        std::fs::remove_dir(&directory).unwrap();
    }
}
