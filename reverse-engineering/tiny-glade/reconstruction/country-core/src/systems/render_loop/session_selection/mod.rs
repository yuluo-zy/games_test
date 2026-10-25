//! `session_selection/mod.rs` — session-selection preview thumbnails.
//!
//! 证据（O = 机器码观测，I = 推断，U = 未知）：
//! - `SessionThumbnailsPlugin::build`（0x1408f68c0，63 B）：`add_systems(PreStartup, thumbnails_startup)` +
//!   `insert_resource(ThumbnailTextures(Vec::new()))`（`{cap 0, ptr 8, len 0}`）。（O）
//! - `thumbnails_startup`（0x1408f6900，759 B）：4 个 `load_shader_into_library`，再 6 次
//!   `thumbnails_startup::closure$0`（0x1408a3bc0）创建 512x512 缩略图纹理并放进池。（O）
//! - `render_session_selection_previews`（0x1408f6c00，20953 B）：见 [`render_session_selection_previews`]。
//!   内联宿主 `extract_thumbnails_to_display` / `find_backed_thumbnail_range` / `assign_pool_indices` /
//!   `refresh_screenshot`（PDB 内联名，源码行 79/97/134/135/169/228 等来自 panic Location）。
//!
//! 外部调用（world 资源、文件系统、图片解码、GPU）经 [`SessionPreviewHost`] / [`ThumbnailStartupHost`] 缝表达。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::Mutex;

use super::menu_background::ManagedTexture;

// ---------------------------------------------------------------- 常量（均 O）

/// 缩略图池大小（O：assertion 文本 `THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())`；启动循环 6 次）。
pub const THUMBNAIL_POOL_SIZE: usize = 6;
/// 缩略图纹理：512x512（O：`movabs rdx, 0x20000000200`）、格式 0x2b、sampled、mip_levels(1)。
pub const THUMBNAIL_EXTENT: [u32; 2] = [512, 512];
pub const THUMBNAIL_FORMAT: u32 = 0x2b;
pub const THUMBNAIL_MIP_LEVELS: u8 = 1;
/// 创建纹理的调试名（O：`create_texture(.., "save thumbnail")`）。
pub const THUMBNAIL_TEXTURE_NAME: &str = "save thumbnail";
/// 创建失败的日志文案（O，`log::error!`，目标 `country_core::systems::render_loop::session_selection`）。
pub const CREATE_FAILED_MESSAGE: &str = "Failed creating session thumbnail texture: ";

/// 着色器（URI, 名字）清单，加载顺序（O：`thumbnails_startup` 调用序）。
pub const STARTUP_SHADERS: [(&str, &str); 4] = [
    (
        "/session_selection/background.raster.hlsl",
        "session_selection_background",
    ),
    (
        "/session_selection/thumbnail.raster.hlsl",
        "session_selection_thumbnail",
    ),
    (
        "/session_selection/new_glade_thumbnail.raster.hlsl",
        "session_selection_new_glade_thumbnail",
    ),
    (
        "/session_selection/thumbnail_painterly.raster.hlsl",
        "session_selection_thumbnail_painterly",
    ),
];

/// `render_*` 里 `get_raster_handle_by_name` 的查找顺序（O）：background、new_glade、thumbnail、painterly。
pub const RENDER_SHADER_LOOKUP_ORDER: [&str; 4] = [
    "session_selection_background",
    "session_selection_new_glade_thumbnail",
    "session_selection_thumbnail",
    "session_selection_thumbnail_painterly",
];

pub const PASS_NAME: &str = "Session selection thumbnail";
pub const SCREENSHOT_FILE_NAME: &str = "screenshot.jpg";

/// 绘制顶点数（O：`draw_no_vao(edx)`）。
pub const BACKGROUND_VERTICES: u32 = 3;
pub const CARD_VERTICES: u32 = 6;
pub const PAINTERLY_VERTICES: u32 = 0x300;

pub const TEXTURE_ACCESS: u8 = 0x1d;

/// uniform 名哈希（O）。
pub const U_TARGET_EXTENT: u64 = 0x531b_b931_8eea_4971; // optional [u32;2]
pub const U_HIGHLIGHT: u64 = 0x54f2_5b7b_526c_bbad; // f32
pub const U_POSITION: u64 = 0x0277_63b3_d9cc_2a8a; // Vec2
pub const U_SIZE: u64 = 0x7038_b90f_2875_5919; // Vec2
pub const U_SLOT: u64 = 0x57ca_25b5_7d60_d824; // u32
pub const U_FLAG: u64 = 0x10b7_1131_3e59_5441; // bool

/// 纹理绑定与磁盘纹理哈希（O）。
pub const BG_TEXTURE_BINDING: u64 = 0x7f41_ce75_31a0_6fe3;
pub const BG_TEXTURE_DISK_HASH: u64 = 0x7300_3282_1e11_adf2;
pub const CARD_TEXTURE_BINDING: u64 = -0x19c2_e600_89aa_009f_i64 as u64;
pub const PLACEHOLDER_DISK_HASH: u64 = -0x7f93_d644_083a_fcf3_i64 as u64;
pub const NEW_GLADE_DISK_HASH: u64 = -0x1309_912d_2adc_9e49_i64 as u64;
pub const PAINTERLY_TEXTURE_BINDING: u64 = 0x7337_daaf_f5b5_b2d5;
pub const PAINTERLY_DISK_HASH: u64 = -0x0857_a171_164d_2fc4_i64 as u64;

/// `RasterState` 改写（O）。
pub const RASTER_STATE_BYTE0_MASK: u8 = 0x7f;
pub const RASTER_STATE_BYTE3_KEEP: u8 = 0x9f;
pub const RASTER_STATE_BYTE3_SET: u8 = 0x40;

/// 新分配池项的 `modified`（O：`0x019db1ded53e8000` = 1970-01-01 的 FILETIME，强制首次重载）。
pub const FILETIME_UNIX_EPOCH: i64 = 0x019d_b1de_d53e_8000;

/// `error_once` 宏展开的去重键前缀（O：栈上写入的字面量 `"module_path! ()"` + `"$crate :: Level :: Error"`）。
pub const ERROR_ONCE_KEY_PREFIX: &str = "module_path! ()$crate :: Level :: Error";
pub const REFRESH_FAILED_PREFIX: &str = "Failed to refresh screenshot ";
pub const LOADED_MESSAGE: &str = "Loaded session thumbnail image";

// ---------------------------------------------------------------- 数据

/// 池中单块缩略图（O 0x68 B）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PooledThumbnail {
    /// `+0x00`：`Option<PathBuf>`（Windows 上 0x20 B；`cap == 0x8000…` 即 `None`）。
    pub path: Option<PathBuf>,
    /// `+0x20`：已载入文件的修改时间（FILETIME，有符号比较，O：`jge`）。
    pub modified: i64,
    /// `+0x28..+0x58`：GPU 纹理（布局 U，仅持有 id）。
    pub texture: ManagedTexture,
    /// `+0x60`：本帧是否已被分配（每帧开头清零，O）。
    pub assigned: bool,
}

/// 会话缩略图纹理池资源（O：`Vec<PooledThumbnail>` @ `{cap, ptr@+8, len@+0x10}`）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThumbnailTextures {
    pub pool: Vec<PooledThumbnail>,
}

/// 轮播状态（锁内拷贝；`Arc<Mutex<_>>` 数据区从 Arc+0x14 起；O 读取点，名字 I）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CarouselState {
    /// `+0x14`：8 B（读后仅被拷贝，用途 U）。
    pub unobserved_14: u64,
    /// `+0x1c`、`+0x20`：第 0 张卡片的位置（Vec2 uniform `U_POSITION` 的基准）。
    pub origin: [f32; 2],
    /// `+0x24`、`+0x28`：卡片尺寸（Vec2 uniform `U_SIZE`）。
    pub size: [f32; 2],
    /// `+0x2c`：相邻卡片的 x 间距。
    pub spacing: f32,
    /// `+0x30..+0x34`：U。
    pub unobserved_30: u32,
    /// `+0x34`：滚动位置（`max(0)` 后参与窗口计算）。
    pub scroll: f32,
    /// `+0x38..+0x40`：U。
    pub unobserved_38: [u32; 2],
    /// `+0x40`：必须 `== 1`（O：`assert_eq!(.., 1)`，`assert_failed<u32,u32>`）。
    pub mode_tag: u32,
    /// `+0x44`：必须非 0，否则 `panic!("TODO")`（O：源码行 135）。
    pub active: bool,
}

/// `Res<AppMode>`（hash `0xd38dae617fb79c7a`）里本系统读取的部分。
#[derive(Clone, Debug)]
pub struct SelectionModeView {
    /// `+0x00`。
    pub state: u64,
    /// `+0x08`：被选中的槽位（0 = “新建”卡，`i+1` = 第 i 个会话）。
    pub selected_slot: u64,
    /// `+0x30`：轮播状态共享体。
    pub carousel: Arc<Mutex<CarouselState>>,
}

/// 画布上一个会话条目（O 28 B；`+0x18` 一个标志字节，`SessionId` 在开头）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionRow {
    pub screenshot_path: PathBuf,
    /// `+0x18` 的字节（painterly 缩略图开关，I）。
    pub flag: bool,
}

/// 解码得到的 RGBA 图（`DynamicImage::to_rgba8`）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbaImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

/// `refresh_screenshot` 的错误：anyhow context 链，`{}` 只显示最外层 context（O：`anyhow::Error` Display）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefreshError {
    pub context: &'static str,
    pub source: String,
}

impl core::fmt::Display for RefreshError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.context)
    }
}

/// 图片解码结果的三种去向（O：`0x1408f7a75..0x1408f7ac6`）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeOutcome {
    Image(RgbaImage),
    /// `Reader::decode` 返回特定的 `ImageError` 变体 10 且载荷为 0：既不报错也不更新，直接跳过本项（U：语义）。
    Skip,
    /// `Reader::open` / `decode` 失败（context `"Loading the image"`）。
    Failed(String),
}

// ---------------------------------------------------------------- 纯算法

/// `AppMode` 门控（O：0x1408f6cec..0x1408f6d00）：`state - 2 < 9 && state - 2 != 4` 时本系统不画。
pub fn mode_hides_selection(state: u64) -> bool {
    let k = state.wrapping_sub(2);
    k < 9 && k != 4
}

/// x86 `MAXSS dst, src`：`dst > src ? dst : src`（NaN 取 src）。
fn x86_max(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}
/// x86 `MINSS dst, src`。
fn x86_min(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}

/// `find_backed_thumbnail_range`（0x1408f6fa9..0x1408f7075）：窗口 `[start, end)`。
///
/// `start = floor(min(len, max(0, scroll)) - pool_len / 2 + 0.5)`（f32，饱和转 u64；`floor` 为 I：调用的是 CRT 导入
/// 桩 0x142923fa0），`end = min(len, start + pool_len)`，并断言
/// `backed_range.len() <= THUMBNAIL_POOL_SIZE.min(sessions.len())`（O 文案）。
pub fn find_backed_thumbnail_range(sessions_len: usize, scroll: f32, pool_len: usize) -> core::ops::Range<usize> {
    let len_f = sessions_len as u64 as f32;
    let clamped = x86_min(len_f, x86_max(0.0, scroll));
    let x = clamped - (pool_len as u64 as f32) * 0.5 + 0.5;
    let start = x86_max(x.floor(), 0.0) as u64 as usize;
    let upper = start.wrapping_add(pool_len);
    let end = upper.min(sessions_len);
    let range_len = end.saturating_sub(start);
    assert!(
        range_len <= THUMBNAIL_POOL_SIZE.min(sessions_len),
        "assertion failed: backed_range.len() <= THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())"
    );
    start..end
}

/// `extract_thumbnails_to_display`（0x1408f6fa9 起，pass D）：清空全部 `assigned`，再对窗口内每个会话在池里找
/// 第一个 `path == Some(同一截图路径)` 的项（逐组件比较，O：`eq_by<Rev<Components>>`），命中则置 `assigned`
/// 并记录池下标。
pub fn extract_thumbnails_to_display(pool: &mut [PooledThumbnail], paths: &[PathBuf]) -> Vec<Option<usize>> {
    for e in pool.iter_mut() {
        e.assigned = false;
    }
    let count = paths.len().min(THUMBNAIL_POOL_SIZE);
    let mut out = vec![None; count];
    for (i, p) in paths.iter().take(count).enumerate() {
        for (idx, e) in pool.iter_mut().enumerate() {
            let Some(existing) = e.path.as_ref() else { continue };
            if path_components_rev_eq(existing, p) {
                e.assigned = true;
                out[i] = Some(idx);
                break;
            }
        }
    }
    out
}

/// 两条路径逐组件（从末尾向前）相等（O：`Iterator::eq_by<Rev<Components>, Rev<Components>>`）。
pub fn path_components_rev_eq(a: &Path, b: &Path) -> bool {
    a.components().rev().eq(b.components().rev())
}

/// `assign_pool_indices`（0x1408f7521 起，pass E）：对仍为 `None` 的窗口槽位，取池中第一个未 `assigned` 的项
/// （没有 → `expect("thumb pool entry")`），置 `assigned`，路径改为新截图路径，`modified` 复位到 1970（强制重载）。
pub fn assign_pool_indices(pool: &mut [PooledThumbnail], out: &mut [Option<usize>], paths: &[PathBuf]) {
    for (i, slot) in out.iter_mut().enumerate() {
        if slot.is_some() {
            continue;
        }
        let idx = pool.iter().position(|e| !e.assigned).expect("thumb pool entry");
        let e = &mut pool[idx];
        e.assigned = true;
        e.path = Some(paths[i].clone());
        e.modified = FILETIME_UNIX_EPOCH;
        *slot = Some(idx);
    }
}

/// 把 `Option<usize>` 填成 6 槽下标数组（O：0x1408f7624..0x1408f7703；窗口内每项必须 `Some`，否则 `unwrap` panic；
/// 其余为 `usize::MAX`），并断言每个窗口下标 `< pool.len()`（O：`assertion failed: idx < self.texture_pool.len()`）。
pub fn pool_index_array(out: &[Option<usize>], pool_len: usize) -> [usize; THUMBNAIL_POOL_SIZE] {
    let mut a = [usize::MAX; THUMBNAIL_POOL_SIZE];
    for (k, o) in out.iter().enumerate() {
        a[k] = o.expect("called `Option::unwrap()` on a `None` value");
    }
    for &idx in &a[..out.len()] {
        assert!(idx < pool_len, "assertion failed: idx < self.texture_pool.len()");
    }
    a
}

/// `error_once` 的去重键（O：字面量前缀 + 格式化后的消息）。
pub fn error_once_key(message: &str) -> String {
    format!("{ERROR_ONCE_KEY_PREFIX}{message}")
}

/// 一次性错误消息（O：pieces `["Failed to refresh screenshot ", ": "]`，参数 = `utils::file::tail(path, 3)` 与错误 Display）。
pub fn refresh_failed_message(tail: &str, err: &RefreshError) -> String {
    format!("{REFRESH_FAILED_PREFIX}{tail}: {err}")
}

/// 一个会话卡片的 uniform（O：0x1408f8a6d..0x1408f9309）。`slot = 会话下标 + 1`（0 留给“新建”卡）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardUniforms {
    pub highlight: f32,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub slot: u32,
}

/// 高亮：`state & 1 != 0 && slot == selected_slot` → 1.0，否则 0.0（O：`cmp rsi,[mode+8]` + `test al,1`）。
pub fn card_highlight(state: u64, selected_slot: u64, slot: u64) -> f32 {
    if state & 1 != 0 && slot == selected_slot {
        1.0
    } else {
        0.0
    }
}

/// 卡片 uniform：位置 = `(slot * spacing + origin.x, origin.y)`（O：`mulss xmm8; addss xmm6`）。
pub fn card_uniforms(c: &CarouselState, mode: &SelectionModeView, slot: u64) -> CardUniforms {
    CardUniforms {
        highlight: card_highlight(mode.state, mode.selected_slot, slot),
        position: [(slot as f32) * c.spacing + c.origin[0], c.origin[1]],
        size: c.size,
        slot: slot as u32,
    }
}

/// bool uniform `U_FLAG`：`(index >= 3 && DEMO_OVERRIDE) || session.flag`（O：0x1408f8d63..0x1408f8d9b）。
pub fn card_flag(index: usize, demo_override: bool, session_flag: bool) -> bool {
    (index >= 3 && demo_override) || session_flag
}

// ---------------------------------------------------------------- 宿主缝

/// `thumbnails_startup` 的外部面。
pub trait ThumbnailStartupHost {
    /// `startup::load_shader_into_library(uri, name)`。
    fn load_shader_into_library(&mut self, uri: &str, name: &str);
    /// `TextureDesc::new_2d(0x2b, 512x512).sampled().mip_levels(1)` + `GraphicsDevice::create_texture(desc, "save thumbnail", None)`。
    fn create_thumbnail_texture(&mut self) -> Result<ManagedTexture, String>;
    /// `log::error!`。
    fn log_error(&mut self, message: &str);
}

/// 缩略图池初始化（O：0x1408f6900 + 闭包 0x1408a3bc0）：失败的纹理只记日志、不进池。
pub fn thumbnails_startup<H: ThumbnailStartupHost>(host: &mut H, thumbnails: &mut ThumbnailTextures) {
    for (uri, name) in STARTUP_SHADERS {
        host.load_shader_into_library(uri, name);
    }
    for _ in 0..THUMBNAIL_POOL_SIZE {
        match host.create_thumbnail_texture() {
            Ok(texture) => thumbnails.pool.push(PooledThumbnail {
                path: None,
                modified: 0, // U：闭包不写 +0x20
                texture,
                assigned: false,
            }),
            Err(e) => host.log_error(&format!("{CREATE_FAILED_MESSAGE}{e}")),
        }
    }
}

/// `render_session_selection_previews` 的外部面（调用点 O）。
pub trait SessionPreviewHost {
    /// `Res<AppMode>`（缺失 → unwrap panic，O @0x1408f979e）。
    fn app_mode(&self) -> SelectionModeView;
    /// `AssetShaderLibrary::get_raster_handle_by_name`（缺失 → unwrap panic）。
    fn raster_shader(&self, name: &str) -> u64;
    /// `Res<SavedSessionList>`（hash `0x4da5153112b09210`，O @0x1408f6e57）的会话行。
    fn sessions(&self) -> Vec<SessionRow>;
    /// `ResMut<ThumbnailTextures>`（hash `-0x5e53d91ba7838a7…`，O @0x1408f6eeb）；写入资源的 changed tick。
    fn thumbnails(&mut self) -> &mut ThumbnailTextures;
    fn mark_thumbnails_changed(&mut self);
    /// 静态字节 0x143634d90（`_DEV_OVERRIDE_IS_DEMO_MODE`）。
    fn demo_mode_override(&self) -> bool;

    // --- refresh_screenshot（O：0x1408f77fc..0x1408f8336）
    /// `fs::metadata(path)` → `Metadata::modified()` → FILETIME。错误 context 分别为 `"Reading metadata"` / `"Checking modification time"`。
    fn modified_filetime(&self, path: &Path) -> Result<i64, RefreshError>;
    /// `image::io::Reader::open(path)` + `decode()`。
    fn decode_image(&self, path: &Path) -> DecodeOutcome;
    /// `GraphicsDevice::update_texture(texture, rgba)`（访问 0x22，层 1）。
    fn update_texture(&mut self, texture: &ManagedTexture, image: &RgbaImage) -> Result<(), String>;
    /// `utils::file::tail(path, 3)` 的 Display。
    fn path_tail(&self, path: &Path) -> String;
    /// 全局 `Mutex<BTreeSet<String>>`（`"Mutex was poisonned"` expect）；返回 `true` = 新插入。
    fn insert_error_key(&mut self, key: String) -> bool;
    fn log_error(&mut self, message: &str);
    fn log_debug(&mut self, message: &str);

    // --- 绘制
    fn target_extent(&self) -> [u32; 2];
    fn begin_pass(&mut self, name: &str);
    fn begin_draw(&mut self, shader: u64);
    fn optional_uniform_u32x2(&mut self, key: u64, v: [u32; 2]);
    fn uniform_f32(&mut self, key: u64, v: f32);
    fn uniform_vec2(&mut self, key: u64, v: [f32; 2]);
    fn uniform_u32(&mut self, key: u64, v: u32);
    fn uniform_bool(&mut self, key: u64, v: bool);
    fn bind_texture(&mut self, tex: &ManagedTexture, binding: u64, access: u8);
    fn bind_disk_texture(&mut self, hash: u64, binding: u64, access: u8);
    fn edit_raster_state(&mut self, byte0_mask: u8, byte3_keep: u8, byte3_set: u8);
    fn draw_no_vao(&mut self, vertices: u32);
    fn render_pass(&mut self);
}

/// `refresh_screenshot`（内联 @235 / 0x1408f77fc 循环体）：对一个已分配的池项。
///
/// 路径为 `None` → `expect("current_content")`；`modified >= 文件修改时间` → 不动；否则解码并 `update_texture`，成功后
/// 写回 `modified` 并（日志级别 ≥ Debug）记 [`LOADED_MESSAGE`]；任何一步失败都不更新 `modified`（下一帧重试），
/// 错误消息按 [`error_once_key`] 去重后记一次。
pub fn refresh_screenshot<H: SessionPreviewHost>(host: &mut H, idx: usize) {
    let (path, last_modified, texture) = {
        let e = &host.thumbnails().pool[idx];
        (e.path.clone().expect("current_content"), e.modified, e.texture.clone())
    };
    let result = refresh_one(host, &path, last_modified, &texture);
    match result {
        Ok(Some(new_modified)) => {
            host.thumbnails().pool[idx].modified = new_modified;
            host.log_debug(LOADED_MESSAGE);
        }
        Ok(None) => {}
        Err(err) => {
            let message = refresh_failed_message(&host.path_tail(&path), &err);
            if host.insert_error_key(error_once_key(&message)) {
                host.log_error(&message);
            }
        }
    }
}

fn refresh_one<H: SessionPreviewHost>(
    host: &mut H,
    path: &Path,
    last_modified: i64,
    texture: &ManagedTexture,
) -> Result<Option<i64>, RefreshError> {
    let modified = host.modified_filetime(path)?;
    if last_modified >= modified {
        return Ok(None);
    }
    match host.decode_image(path) {
        DecodeOutcome::Skip => Ok(None),
        DecodeOutcome::Failed(source) => Err(RefreshError {
            context: "Loading the image",
            source,
        }),
        DecodeOutcome::Image(img) => {
            host.update_texture(texture, &img).map_err(|source| RefreshError {
                context: "updating texture",
                source,
            })?;
            Ok(Some(modified))
        }
    }
}

/// O：0x1408f6c00，参数 `(pass_resources, world, target: &Texture)`。
///
/// 顺序（O）：`AppMode` 门控 → 取 4 个着色器 → 锁轮播状态并拷贝（`mode_tag == 1` 断言、`active` 非 0）→ 取会话列表与
/// 池（写 changed tick）→ 窗口 → 清 `assigned`、pass D、pass E → 6 槽下标数组 → 刷新所有已分配项的截图 → 纹理列表 →
/// "Session selection thumbnail" 光栅 pass：背景、“新建”卡、会话卡（thumbnail 着色器）、会话卡（painterly 着色器）→ render。
pub fn render_session_selection_previews<H: SessionPreviewHost>(host: &mut H) {
    let mode = host.app_mode();
    if mode_hides_selection(mode.state) {
        return;
    }
    let background = host.raster_shader(RENDER_SHADER_LOOKUP_ORDER[0]);
    let new_glade = host.raster_shader(RENDER_SHADER_LOOKUP_ORDER[1]);
    let thumbnail = host.raster_shader(RENDER_SHADER_LOOKUP_ORDER[2]);
    let painterly = host.raster_shader(RENDER_SHADER_LOOKUP_ORDER[3]);

    let carousel = *mode.carousel.lock();
    let sessions = host.sessions();
    host.mark_thumbnails_changed();

    assert_eq!(carousel.mode_tag, 1);
    if !carousel.active {
        panic!("TODO");
    }

    let pool_len = host.thumbnails().pool.len();
    let range = find_backed_thumbnail_range(sessions.len(), carousel.scroll, pool_len);
    let count = range.len().min(THUMBNAIL_POOL_SIZE);
    let paths: Vec<PathBuf> = sessions[range.start..range.start + count]
        .iter()
        .map(|s| s.screenshot_path.clone())
        .collect();

    let array = {
        let pool = &mut host.thumbnails().pool;
        let mut out = extract_thumbnails_to_display(pool, &paths);
        assign_pool_indices(pool, &mut out, &paths);
        pool_index_array(&out, pool.len())
    };

    let assigned: Vec<usize> = host
        .thumbnails()
        .pool
        .iter()
        .enumerate()
        .filter(|(_, e)| e.assigned)
        .map(|(i, _)| i)
        .collect();
    for idx in assigned {
        refresh_screenshot(host, idx);
    }

    let textures: Vec<Option<ManagedTexture>> = array[..count]
        .iter()
        .map(|&i| host.thumbnails().pool.get(i).map(|e| e.texture.clone()))
        .collect();
    let items: Vec<(&SessionRow, Option<&ManagedTexture>)> = sessions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let tex = if range.contains(&i) {
                textures.get(i - range.start).and_then(|t| t.as_ref())
            } else {
                None
            };
            (s, tex)
        })
        .collect();

    let extent = host.target_extent();
    let demo = host.demo_mode_override();
    host.begin_pass(PASS_NAME);

    // 背景：第 0 号位置，磁盘纹理，3 顶点。
    host.begin_draw(background);
    host.optional_uniform_u32x2(U_TARGET_EXTENT, extent);
    host.uniform_vec2(U_POSITION, carousel.origin);
    host.bind_disk_texture(BG_TEXTURE_DISK_HASH, BG_TEXTURE_BINDING, TEXTURE_ACCESS);
    host.edit_raster_state(RASTER_STATE_BYTE0_MASK, 0xff, 0x00);
    host.draw_no_vao(BACKGROUND_VERTICES);

    // “新建”卡（槽位 0）。
    let u = card_uniforms(&carousel, &mode, 0);
    host.begin_draw(new_glade);
    host.optional_uniform_u32x2(U_TARGET_EXTENT, extent);
    host.uniform_f32(U_HIGHLIGHT, u.highlight);
    host.uniform_vec2(U_POSITION, u.position);
    host.uniform_vec2(U_SIZE, u.size);
    host.uniform_u32(U_SLOT, u.slot);
    host.bind_disk_texture(NEW_GLADE_DISK_HASH, CARD_TEXTURE_BINDING, TEXTURE_ACCESS);
    set_card_state(host);
    host.draw_no_vao(CARD_VERTICES);

    // 会话卡：先 thumbnail 着色器，再 painterly 着色器。
    for (index, (session, tex)) in items.iter().enumerate() {
        let u = card_uniforms(&carousel, &mode, index as u64 + 1);
        host.begin_draw(thumbnail);
        host.optional_uniform_u32x2(U_TARGET_EXTENT, extent);
        host.uniform_f32(U_HIGHLIGHT, u.highlight);
        host.uniform_vec2(U_POSITION, u.position);
        host.uniform_vec2(U_SIZE, u.size);
        host.uniform_u32(U_SLOT, u.slot);
        host.uniform_bool(U_FLAG, card_flag(index, demo, session.flag));
        bind_card_texture(host, *tex);
        set_card_state(host);
        host.draw_no_vao(CARD_VERTICES);
    }
    for (index, (session, tex)) in items.iter().enumerate() {
        let u = card_uniforms(&carousel, &mode, index as u64 + 1);
        host.begin_draw(painterly);
        host.optional_uniform_u32x2(U_TARGET_EXTENT, extent);
        host.uniform_f32(U_HIGHLIGHT, u.highlight);
        host.uniform_vec2(U_POSITION, u.position);
        host.uniform_vec2(U_SIZE, u.size);
        host.bind_disk_texture(PAINTERLY_DISK_HASH, PAINTERLY_TEXTURE_BINDING, TEXTURE_ACCESS);
        host.uniform_u32(U_SLOT, u.slot);
        host.uniform_bool(U_FLAG, card_flag(index, demo, session.flag));
        bind_card_texture(host, *tex);
        set_card_state(host);
        host.draw_no_vao(PAINTERLY_VERTICES);
    }
    host.render_pass();
}

fn bind_card_texture<H: SessionPreviewHost>(host: &mut H, tex: Option<&ManagedTexture>) {
    match tex {
        Some(t) => host.bind_texture(t, CARD_TEXTURE_BINDING, TEXTURE_ACCESS),
        None => host.bind_disk_texture(PLACEHOLDER_DISK_HASH, CARD_TEXTURE_BINDING, TEXTURE_ACCESS),
    }
}

fn set_card_state<H: SessionPreviewHost>(host: &mut H) {
    host.edit_raster_state(RASTER_STATE_BYTE0_MASK, RASTER_STATE_BYTE3_KEEP, RASTER_STATE_BYTE3_SET);
}

/// SessionSelection bevy Plugin。
pub struct SessionThumbnailsPlugin;

impl SessionThumbnailsPlugin {
    /// O：0x1408f68c0。`add_systems(PreStartup, thumbnails_startup)` + `insert_resource(ThumbnailTextures(空 Vec))`。
    pub fn build(&self) -> ThumbnailTextures {
        ThumbnailTextures::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashMap};

    fn entry(path: Option<&str>, id: u64) -> PooledThumbnail {
        PooledThumbnail {
            path: path.map(PathBuf::from),
            modified: 0,
            texture: ManagedTexture(id),
            assigned: false,
        }
    }

    #[test]
    fn mode_gate_hides_for_states_2_to_10_except_6() {
        let hidden: Vec<u64> = (0..16).filter(|&s| mode_hides_selection(s)).collect();
        assert_eq!(hidden, [2, 3, 4, 5, 7, 8, 9, 10]);
        assert!(!mode_hides_selection(6) && !mode_hides_selection(0) && !mode_hides_selection(1));
        assert!(!mode_hides_selection(11) && !mode_hides_selection(u64::MAX));
    }

    #[test]
    fn window_follows_scroll_and_clamps() {
        // pool 6：start = floor(clamp(scroll) - 3 + 0.5)
        assert_eq!(find_backed_thumbnail_range(10, 0.0, 6), 0..6);
        assert_eq!(find_backed_thumbnail_range(10, 5.0, 6), 2..8);
        assert_eq!(find_backed_thumbnail_range(10, 7.2, 6), 4..10);
        assert_eq!(find_backed_thumbnail_range(10, 100.0, 6), 7..10); // scroll 被夹到 len
        assert_eq!(find_backed_thumbnail_range(2, 1.0, 6), 0..2);
        assert_eq!(find_backed_thumbnail_range(0, 3.0, 6), 0..0);
        assert_eq!(find_backed_thumbnail_range(10, -5.0, 6), 0..6);
        assert_eq!(find_backed_thumbnail_range(10, f32::NAN, 6), 0..6);
    }

    #[test]
    fn extract_marks_matching_paths_and_clears_old_flags() {
        let mut pool = vec![
            entry(Some("a/s1/screenshot.jpg"), 0),
            entry(None, 1),
            entry(Some("a/s2/screenshot.jpg"), 2),
        ];
        pool[0].assigned = true;
        pool[1].assigned = true;
        let paths = vec![
            PathBuf::from("a/s2/screenshot.jpg"),
            PathBuf::from("a/s9/screenshot.jpg"),
        ];
        let out = extract_thumbnails_to_display(&mut pool, &paths);
        assert_eq!(out, [Some(2), None]);
        assert_eq!(
            pool.iter().map(|e| e.assigned).collect::<Vec<_>>(),
            [false, false, true]
        );
    }

    #[test]
    fn extract_takes_the_first_match_even_if_already_taken() {
        let mut pool = vec![entry(Some("x/screenshot.jpg"), 0), entry(Some("x/screenshot.jpg"), 1)];
        let paths = vec![PathBuf::from("x/screenshot.jpg"), PathBuf::from("x/screenshot.jpg")];
        assert_eq!(extract_thumbnails_to_display(&mut pool, &paths), [Some(0), Some(0)]);
    }

    #[test]
    fn components_compare_ignores_separator_style() {
        assert!(path_components_rev_eq(
            Path::new("a/b/c.jpg"),
            Path::new("a//b/./c.jpg")
        ));
        assert!(!path_components_rev_eq(Path::new("a/b/c.jpg"), Path::new("a/b/d.jpg")));
    }

    #[test]
    fn assign_uses_first_free_entry_and_resets_modified() {
        let mut pool = vec![entry(Some("old"), 0), entry(Some("old2"), 1), entry(None, 2)];
        pool[0].assigned = true;
        pool[0].modified = 77;
        pool[1].modified = 88;
        let paths = vec![PathBuf::from("n1"), PathBuf::from("n2")];
        let mut out = vec![Some(0), None];
        assign_pool_indices(&mut pool, &mut out, &paths);
        assert_eq!(out, [Some(0), Some(1)]);
        assert_eq!(pool[1].path.as_deref(), Some(Path::new("n2")));
        assert_eq!(pool[1].modified, FILETIME_UNIX_EPOCH);
        assert_eq!(pool[0].modified, 77, "已有的不动");
        assert!(pool[1].assigned && !pool[2].assigned);
    }

    #[test]
    #[should_panic(expected = "thumb pool entry")]
    fn assign_panics_when_pool_is_exhausted() {
        let mut pool = vec![entry(None, 0)];
        pool[0].assigned = true;
        let mut out = vec![None];
        assign_pool_indices(&mut pool, &mut out, &[PathBuf::from("p")]);
    }

    #[test]
    fn index_array_pads_with_max_and_checks_bounds() {
        assert_eq!(
            pool_index_array(&[Some(3), Some(1)], 6),
            [3, 1, usize::MAX, usize::MAX, usize::MAX, usize::MAX]
        );
        assert_eq!(pool_index_array(&[], 6), [usize::MAX; 6]);
    }

    #[test]
    #[should_panic(expected = "assertion failed: idx < self.texture_pool.len()")]
    fn index_array_rejects_out_of_range() {
        pool_index_array(&[Some(6)], 6);
    }

    #[test]
    #[should_panic(expected = "Option::unwrap()")]
    fn index_array_rejects_unassigned_window_slot() {
        pool_index_array(&[None], 6);
    }

    #[test]
    fn highlight_and_position_formulas() {
        assert_eq!(card_highlight(1, 3, 3), 1.0);
        assert_eq!(card_highlight(2, 3, 3), 0.0); // state 偶数
        assert_eq!(card_highlight(1, 3, 2), 0.0);
        let c = CarouselState {
            origin: [10.0, 4.0],
            size: [2.0, 3.0],
            spacing: 5.0,
            ..Default::default()
        };
        let mode = SelectionModeView {
            state: 1,
            selected_slot: 0,
            carousel: Arc::new(Mutex::new(c)),
        };
        let u = card_uniforms(&c, &mode, 0);
        assert_eq!(
            (u.highlight, u.position, u.size, u.slot),
            (1.0, [10.0, 4.0], [2.0, 3.0], 0)
        );
        assert_eq!(card_uniforms(&c, &mode, 3).position, [25.0, 4.0]);
    }

    #[test]
    fn flag_uniform_table() {
        assert!(!card_flag(0, true, false));
        assert!(!card_flag(2, true, false));
        assert!(card_flag(3, true, false));
        assert!(!card_flag(3, false, false));
        assert!(card_flag(0, false, true));
    }

    #[test]
    fn error_key_and_message_texts() {
        let e = RefreshError {
            context: "Loading the image",
            source: "io".into(),
        };
        let m = refresh_failed_message("a/b/screenshot.jpg", &e);
        assert_eq!(m, "Failed to refresh screenshot a/b/screenshot.jpg: Loading the image");
        assert_eq!(
            error_once_key(&m),
            format!("module_path! ()$crate :: Level :: Error{m}")
        );
        assert_eq!(ERROR_ONCE_KEY_PREFIX.len(), 0x27);
    }

    #[test]
    fn startup_shader_table_matches_string_table() {
        let uris: Vec<_> = STARTUP_SHADERS.iter().map(|s| s.0.len()).collect();
        assert_eq!(uris, [0x29, 0x28, 0x32, 0x32]);
        let names: Vec<_> = STARTUP_SHADERS.iter().map(|s| s.1.len()).collect();
        assert_eq!(names, [0x1c, 0x1b, 0x25, 0x25]);
        assert_eq!(THUMBNAIL_EXTENT, [512, 512]);
        assert_eq!(FILETIME_UNIX_EPOCH, 116_444_736_000_000_000);
    }

    // ---------------- 宿主录制

    #[derive(Default)]
    struct StartupRec {
        log: Vec<String>,
        fail_at: Option<usize>,
        created: usize,
    }
    impl ThumbnailStartupHost for StartupRec {
        fn load_shader_into_library(&mut self, uri: &str, name: &str) {
            self.log.push(format!("shader {uri} {name}"));
        }
        fn create_thumbnail_texture(&mut self) -> Result<ManagedTexture, String> {
            let n = self.created;
            self.created += 1;
            if self.fail_at == Some(n) {
                Err("oom".into())
            } else {
                Ok(ManagedTexture(100 + n as u64))
            }
        }
        fn log_error(&mut self, m: &str) {
            self.log.push(format!("error {m}"));
        }
    }

    #[test]
    fn startup_loads_four_shaders_then_six_textures() {
        let mut h = StartupRec::default();
        let mut t = ThumbnailTextures::default();
        thumbnails_startup(&mut h, &mut t);
        assert_eq!(h.log.len(), 4);
        assert!(h.log[0].starts_with("shader /session_selection/background.raster.hlsl "));
        assert_eq!(t.pool.len(), 6);
        assert!(t.pool.iter().all(|e| e.path.is_none() && !e.assigned));
        assert_eq!(t.pool[5].texture, ManagedTexture(105));
    }

    #[test]
    fn startup_skips_failed_textures_and_logs() {
        let mut h = StartupRec {
            fail_at: Some(2),
            ..Default::default()
        };
        let mut t = ThumbnailTextures::default();
        thumbnails_startup(&mut h, &mut t);
        assert_eq!(t.pool.len(), 5);
        assert_eq!(
            h.log.last().unwrap(),
            "error Failed creating session thumbnail texture: oom"
        );
    }

    struct Rec {
        mode: SelectionModeView,
        sessions: Vec<SessionRow>,
        thumbs: ThumbnailTextures,
        mtimes: HashMap<PathBuf, Result<i64, RefreshError>>,
        decode: HashMap<PathBuf, DecodeOutcome>,
        update_fail: bool,
        demo: bool,
        keys: BTreeSet<String>,
        log: Vec<String>,
    }

    impl SessionPreviewHost for Rec {
        fn app_mode(&self) -> SelectionModeView {
            self.mode.clone()
        }
        fn raster_shader(&self, name: &str) -> u64 {
            self_shader(name)
        }
        fn sessions(&self) -> Vec<SessionRow> {
            self.sessions.clone()
        }
        fn thumbnails(&mut self) -> &mut ThumbnailTextures {
            &mut self.thumbs
        }
        fn mark_thumbnails_changed(&mut self) {
            self.log.push("changed".into());
        }
        fn demo_mode_override(&self) -> bool {
            self.demo
        }
        fn modified_filetime(&self, p: &Path) -> Result<i64, RefreshError> {
            self.mtimes.get(p).cloned().unwrap_or(Ok(FILETIME_UNIX_EPOCH + 1000))
        }
        fn decode_image(&self, p: &Path) -> DecodeOutcome {
            self.decode.get(p).cloned().unwrap_or(DecodeOutcome::Image(RgbaImage {
                width: 1,
                height: 1,
                pixels: vec![0; 4],
            }))
        }
        fn update_texture(&mut self, t: &ManagedTexture, _i: &RgbaImage) -> Result<(), String> {
            self.log.push(format!("update {}", t.0));
            if self.update_fail {
                Err("device lost".into())
            } else {
                Ok(())
            }
        }
        fn path_tail(&self, p: &Path) -> String {
            p.display().to_string()
        }
        fn insert_error_key(&mut self, k: String) -> bool {
            self.keys.insert(k)
        }
        fn log_error(&mut self, m: &str) {
            self.log.push(format!("error {m}"));
        }
        fn log_debug(&mut self, m: &str) {
            self.log.push(format!("debug {m}"));
        }
        fn target_extent(&self) -> [u32; 2] {
            [1280, 720]
        }
        fn begin_pass(&mut self, n: &str) {
            self.log.push(format!("pass {n}"));
        }
        fn begin_draw(&mut self, s: u64) {
            self.log.push(format!("draw {s}"));
        }
        fn optional_uniform_u32x2(&mut self, _k: u64, v: [u32; 2]) {
            self.log.push(format!("extent {v:?}"));
        }
        fn uniform_f32(&mut self, k: u64, v: f32) {
            assert_eq!(k, U_HIGHLIGHT);
            self.log.push(format!("hl {v}"));
        }
        fn uniform_vec2(&mut self, k: u64, v: [f32; 2]) {
            self.log
                .push(format!("{} {v:?}", if k == U_POSITION { "pos" } else { "size" }));
        }
        fn uniform_u32(&mut self, _k: u64, v: u32) {
            self.log.push(format!("slot {v}"));
        }
        fn uniform_bool(&mut self, _k: u64, v: bool) {
            self.log.push(format!("flag {v}"));
        }
        fn bind_texture(&mut self, t: &ManagedTexture, _b: u64, _a: u8) {
            self.log.push(format!("tex {}", t.0));
        }
        fn bind_disk_texture(&mut self, h: u64, b: u64, _a: u8) {
            self.log.push(format!("disk {h:#x} {b:#x}"));
        }
        fn edit_raster_state(&mut self, a: u8, b: u8, c: u8) {
            self.log.push(format!("state {a:#x} {b:#x} {c:#x}"));
        }
        fn draw_no_vao(&mut self, v: u32) {
            self.log.push(format!("vao {v}"));
        }
        fn render_pass(&mut self) {
            self.log.push("render".into());
        }
    }

    fn self_shader(name: &str) -> u64 {
        RENDER_SHADER_LOOKUP_ORDER
            .iter()
            .position(|n| *n == name)
            .expect("shader name") as u64
            + 10
    }

    fn rec(n_sessions: usize, state: u64, selected: u64) -> Rec {
        let carousel = CarouselState {
            origin: [1.0, 2.0],
            size: [3.0, 4.0],
            spacing: 10.0,
            scroll: 0.0,
            mode_tag: 1,
            active: true,
            ..Default::default()
        };
        Rec {
            mode: SelectionModeView {
                state,
                selected_slot: selected,
                carousel: Arc::new(Mutex::new(carousel)),
            },
            sessions: (0..n_sessions)
                .map(|i| SessionRow {
                    screenshot_path: PathBuf::from(format!("saves/s{i}/screenshot.jpg")),
                    flag: i == 1,
                })
                .collect(),
            thumbs: ThumbnailTextures {
                pool: (0..6).map(|i| entry(None, i)).collect(),
            },
            mtimes: HashMap::new(),
            decode: HashMap::new(),
            update_fail: false,
            demo: false,
            keys: BTreeSet::new(),
            log: vec![],
        }
    }

    #[test]
    fn hidden_modes_do_nothing() {
        let mut h = rec(2, 3, 0);
        render_session_selection_previews(&mut h);
        assert!(h.log.is_empty());
    }

    #[test]
    fn full_frame_orders_background_new_glade_cards_then_painterly() {
        let mut h = rec(2, 11, 2); // 11 是奇数 → 高亮可用
        render_session_selection_previews(&mut h);
        let l = &h.log;
        assert_eq!(l[0], "changed");
        // 2 个会话 → 2 次刷新（update 两次，各 debug 一次）
        assert_eq!(l.iter().filter(|s| s.starts_with("update")).count(), 2);
        assert_eq!(
            l.iter()
                .filter(|s| *s == "debug Loaded session thumbnail image")
                .count(),
            2
        );
        let p = l.iter().position(|s| s == "pass Session selection thumbnail").unwrap();
        let tail = &l[p..];
        // 背景
        assert_eq!(
            &tail[1..7],
            [
                "draw 10",
                "extent [1280, 720]",
                "pos [1.0, 2.0]",
                "disk 0x730032821e11adf2 0x7f41ce7531a06fe3",
                "state 0x7f 0xff 0x0",
                "vao 3"
            ]
        );
        // 新建卡（selected_slot = 2 → 不高亮）
        assert_eq!(
            &tail[7..16],
            [
                "draw 11",
                "extent [1280, 720]",
                "hl 0",
                "pos [1.0, 2.0]",
                "size [3.0, 4.0]",
                "slot 0",
                &format!("disk {NEW_GLADE_DISK_HASH:#x} {CARD_TEXTURE_BINDING:#x}"),
                "state 0x7f 0x9f 0x40",
                "vao 6"
            ]
        );
        // 第 1 张卡（slot 1）：thumbnail 着色器 12，位置 x = 1*10+1
        assert_eq!(tail[16], "draw 12");
        assert_eq!(tail[19], "pos [11.0, 2.0]");
        assert!(tail.iter().any(|s| s == "hl 1")); // slot 2 被选中
        assert_eq!(tail.iter().filter(|s| *s == "vao 6").count(), 3); // 新建 + 2 张
        // painterly：着色器 13，顶点 0x300，共 2 次
        assert_eq!(tail.iter().filter(|s| *s == "vao 768").count(), 2);
        assert_eq!(tail.iter().filter(|s| *s == "draw 13").count(), 2);
        assert_eq!(l.last().unwrap(), "render");
        // 池里记录了路径与新的 modified
        assert_eq!(
            h.thumbs.pool[0].path.as_deref(),
            Some(Path::new("saves/s0/screenshot.jpg"))
        );
        assert_eq!(h.thumbs.pool[0].modified, FILETIME_UNIX_EPOCH + 1000);
    }

    #[test]
    fn up_to_date_screenshots_are_not_reloaded_but_new_assignments_always_are() {
        let mut h = rec(1, 11, 0);
        h.mtimes
            .insert(PathBuf::from("saves/s0/screenshot.jpg"), Ok(FILETIME_UNIX_EPOCH)); // 与复位值相等 → 不重载
        render_session_selection_previews(&mut h);
        assert!(!h.log.iter().any(|s| s.starts_with("update")));
        // 第二帧：路径命中已有项，未变化
        let before = h.log.len();
        render_session_selection_previews(&mut h);
        assert!(!h.log[before..].iter().any(|s| s.starts_with("update")));
        assert_eq!(h.thumbs.pool.iter().filter(|e| e.path.is_some()).count(), 1);
    }

    #[test]
    fn refresh_errors_are_logged_once_and_retried() {
        let mut h = rec(1, 11, 0);
        h.decode.insert(
            PathBuf::from("saves/s0/screenshot.jpg"),
            DecodeOutcome::Failed("bad jpeg".into()),
        );
        render_session_selection_previews(&mut h);
        render_session_selection_previews(&mut h);
        let errs: Vec<_> = h.log.iter().filter(|s| s.starts_with("error")).collect();
        assert_eq!(errs.len(), 1, "{errs:?}");
        assert_eq!(
            errs[0],
            "error Failed to refresh screenshot saves/s0/screenshot.jpg: Loading the image"
        );
        assert_eq!(h.thumbs.pool[0].modified, FILETIME_UNIX_EPOCH, "失败不更新 modified");
    }

    #[test]
    fn update_texture_failure_uses_updating_texture_context() {
        let mut h = rec(1, 11, 0);
        h.update_fail = true;
        render_session_selection_previews(&mut h);
        assert!(
            h.log
                .iter()
                .any(|s| s == "error Failed to refresh screenshot saves/s0/screenshot.jpg: updating texture")
        );
        assert!(!h.log.iter().any(|s| s.starts_with("debug")));
    }

    #[test]
    fn metadata_errors_propagate_their_context() {
        let mut h = rec(1, 11, 0);
        h.mtimes.insert(
            PathBuf::from("saves/s0/screenshot.jpg"),
            Err(RefreshError {
                context: "Reading metadata",
                source: "enoent".into(),
            }),
        );
        render_session_selection_previews(&mut h);
        assert!(h.log.iter().any(|s| s.ends_with(": Reading metadata")));
    }

    #[test]
    fn skip_outcome_neither_updates_nor_logs() {
        let mut h = rec(1, 11, 0);
        h.decode
            .insert(PathBuf::from("saves/s0/screenshot.jpg"), DecodeOutcome::Skip);
        render_session_selection_previews(&mut h);
        assert!(
            !h.log
                .iter()
                .any(|s| s.starts_with("update") || s.starts_with("error") || s.starts_with("debug"))
        );
        assert_eq!(h.thumbs.pool[0].modified, FILETIME_UNIX_EPOCH);
    }

    #[test]
    fn sessions_outside_the_window_use_the_placeholder_texture() {
        let mut h = rec(9, 11, 0);
        h.mode.carousel.lock().scroll = 0.0; // 窗口 0..6
        render_session_selection_previews(&mut h);
        let placeholder = format!("disk {PLACEHOLDER_DISK_HASH:#x} {CARD_TEXTURE_BINDING:#x}");
        // 9 张卡 × 2 轮 = 18 次绑定；其中后 3 张（窗口外）每轮用占位纹理 → 6 次
        assert_eq!(h.log.iter().filter(|s| **s == placeholder).count(), 6);
        assert_eq!(h.log.iter().filter(|s| s.starts_with("tex ")).count(), 12);
    }

    #[test]
    fn flag_uniform_follows_session_flag_and_demo_override() {
        let mut h = rec(5, 11, 0);
        h.demo = true;
        render_session_selection_previews(&mut h);
        let flags: Vec<_> = h.log.iter().filter(|s| s.starts_with("flag")).cloned().collect();
        // 每轮 5 张：index 0..5，flag(1)=true；demo → index>=3 也为 true
        assert_eq!(
            &flags[..5],
            ["flag false", "flag true", "flag false", "flag true", "flag true"]
        );
    }

    #[test]
    #[should_panic(expected = "TODO")]
    fn inactive_carousel_panics_with_todo() {
        let mut h = rec(1, 11, 0);
        h.mode.carousel.lock().active = false;
        render_session_selection_previews(&mut h);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn wrong_mode_tag_fails_the_assert_eq() {
        let mut h = rec(1, 11, 0);
        h.mode.carousel.lock().mode_tag = 2;
        render_session_selection_previews(&mut h);
    }

    #[test]
    fn empty_session_list_still_draws_background_and_new_card() {
        let mut h = rec(0, 11, 0);
        render_session_selection_previews(&mut h);
        assert_eq!(h.log.iter().filter(|s| s.starts_with("draw ")).count(), 2);
        assert_eq!(h.log.last().unwrap(), "render");
    }

    #[test]
    fn build_creates_an_empty_pool() {
        assert!(SessionThumbnailsPlugin.build().pool.is_empty());
    }
}
