//! `resources::session` — 会话资源（B 版本骨架对齐，WP13）。
//!
//! 范围（业务逻辑合同见 `contracts/session.md`）：
//!   - `GladeType`    场景主题枚举，判别字节值 1=Olden/2=Flowery/3=Autumn/4=Winter（O）
//!   - `ActiveSession` 当前会话资源（委托 GladeType；字节 @+0x58==4 选冬季，O）
//!   - `NextSession`   下一个会话请求（0xa0 字节，含 SessionId + NextSessionKind）
//!   - `SessionId`     uuid::Uuid v4 包装（new = Uuid::new_v4，O）
//!   - `SessionMeta`   session 元数据（write_to_disk / try_load_from_path / 季节谓词）
//!   - `SavedSession` / `SavedSessionList` 存档列表（按 SystemTime 排序）
//!   - 会话流事件：`NewSessionStartedCmd` / `SessionLoadedCmd` / `SessionOnLoadCmd`
//!
//! 布局未证实字段一律 typed gap；绝不补造原字段名/签名。

use bevy_ecs::resource::Resource;
use uuid::Uuid;

use crate::typed_gap::Gap;

/// 场景主题判别（O：四个 `is_*` 方法为 `cmp byte[rcx], imm; sete al; ret` 常量函数，
/// 逐字节证实 1/2/3/4）。判别值 0（默认主题）未证实，不臆造变体。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GladeType {
    Olden,
    Flowery,
    Autumn,
    Winter,
    /// 未证实值域（非 1..4）。布局 U：原判别字节载体。
    Raw(u8),
}

impl GladeType {
    /// O：0x140a9fa70 `cmp byte[rcx],1` — 古旧主题。
    pub fn is_olden(self) -> bool {
        matches!(self, GladeType::Olden)
    }
    /// O：0x14092e6a0 `cmp byte[rcx],2` — 花草主题。
    pub fn is_flowery(self) -> bool {
        matches!(self, GladeType::Flowery)
    }
    /// O：0x14092e6c0 `cmp byte[rcx],3` — 秋季主题。
    pub fn is_autumn(self) -> bool {
        matches!(self, GladeType::Autumn)
    }
    /// O：0x140ae8320 `cmp byte[rcx],4` — 冬季主题（会话字节 @+0x58==4 选冬季水面）。
    pub fn is_winter(self) -> bool {
        matches!(self, GladeType::Winter)
    }
    /// O：方法存在（`assets_folder`，地址 0x140a9fa80 域）；语义 U。
    pub fn assets_folder(&self) -> Gap {
        Gap
    }
    /// O：方法存在（`audio_id` / `theme_id`）；语义 U。
    pub fn audio_id(&self) -> Gap {
        Gap
    }
    /// O：方法存在（`theme_id`）；语义 U。
    pub fn theme_id(&self) -> Gap {
        Gap
    }
}

/// 判别字节 → 枚举（O：值域 1..4 证实；其余进 `Raw`）。
impl From<u8> for GladeType {
    fn from(tag: u8) -> Self {
        match tag {
            1 => GladeType::Olden,
            2 => GladeType::Flowery,
            3 => GladeType::Autumn,
            4 => GladeType::Winter,
            other => GladeType::Raw(other),
        }
    }
}

/// 当前会话资源（O：`Res<ActiveSession>`；`water_spawn_vegetation` 0x140a13970 经
/// type-id `0x74430dfcc6d8719b` 断言存在并读 `is_autumn`，I：该 type-id 归属本资源）。
/// 布局 U；水面变体字节 @+0x58（O）。
#[derive(Resource, Clone, Debug, Default)]
#[allow(dead_code)]
pub struct ActiveSession {
    /// O：会话字节 @+0x58 存判别；布局 U，故用 gap 承载。
    glade: Gap,
}

impl ActiveSession {
    /// O：方法存在（`ActiveSession::new`）；构造输入 U。
    pub fn new() -> Self {
        Self::default()
    }
    /// O：委托 GladeType::is_winter（WP7 已证实冬季选择路径）。
    pub fn is_winter(&self) -> bool {
        // 判别字节 @+0x58 == 4（O）；此处通过语义化 gap 保留入口，不读未知布局。
        false
    }
    pub fn is_olden(&self) -> bool {
        false
    }
    pub fn is_autumn(&self) -> bool {
        false
    }
    pub fn is_flowery(&self) -> bool {
        false
    }
    /// O：方法存在（`ActiveSession::assets_root`）；语义 U。
    pub fn assets_root(&self) -> Gap {
        Gap
    }
}

/// 下一个会话请求（O：`NextSession` 0xa0=160 字节；`ignore_snapshot` 0x140b46a20
/// 写 bool @+0x98 后整结构复制 0xa0）。布局 U，字段按观察拆分。
#[derive(Resource, Clone, Debug)]
pub struct NextSession {
    /// 待应用 on-load 命令（append_on_load push `(3, block)`；结构开头 Vec，O）。
    pub on_load: Vec<Gap>,
    /// `NextSessionKind` 枚举字段 @+0x18（O：load 写 `[3]=&0x8000000000000002`）。
    pub kind: NextSessionKind,
    /// SessionId（uuid 16B）@+0x20..+0x30（O）。
    pub session_id: SessionId,
    /// `ignore_snapshot` 标志 @+0x98（O）。
    pub ignore_snapshot: bool,
    /// 其余字段（会话参数/模板）。布局 U。
    pub rest: Gap,
}

impl Default for NextSession {
    fn default() -> Self {
        Self {
            on_load: Vec::new(),
            kind: NextSessionKind::default(),
            session_id: SessionId::default(),
            ignore_snapshot: false,
            rest: Gap,
        }
    }
}

impl NextSession {
    /// O：`create` 0x140b46960 从大量参数构造（12+ qword）；参数语义 U。
    pub fn create(session_id: SessionId, kind: NextSessionKind, rest: Gap) -> Self {
        Self {
            on_load: Vec::new(),
            kind,
            session_id,
            ignore_snapshot: false,
            rest,
        }
    }
    /// O：`load` 0x140b46920 构造 `{session_id, 空Vec, kind=..}`。
    pub fn load(session_id: SessionId) -> Self {
        Self {
            on_load: Vec::new(),
            kind: NextSessionKind::default(),
            session_id,
            ignore_snapshot: false,
            rest: Gap,
        }
    }
    /// O：方法存在（`load_latest`）；语义 U。
    pub fn load_latest() -> Self {
        Self::default()
    }
    /// O：`ignore_snapshot` 0x140b46a20 写 bool @+0x98。
    pub fn set_ignore_snapshot(&mut self, value: bool) {
        self.ignore_snapshot = value;
    }
    /// O：`append_on_load<[MetaCommand;3]>` 0x1403b5150 push `(3, block)`；
    /// block 字节数 0xa8。命令内容 U。
    pub fn append_on_load(&mut self, _block: Gap) {
        self.on_load.push(Gap);
    }
}

/// `NextSessionKind` 枚举（O：PDB `enum2$<NextSessionKind>` + drop_in_place；
/// 变体集合 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum NextSessionKind {
    #[default]
    /// 未证变体。原判别/负载 U。
    Unknown,
}

/// 会话 ID = uuid v4（O：`SessionId::new` 0x140916420 `uuid::Uuid::new_v4()`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SessionId(pub Uuid);

impl Default for SessionId {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl SessionId {
    /// O：`new` 0x140916420 = `Uuid::new_v4()`。
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    /// O：`save_folder_path` 0x140b46000 = `paths::saves().join(uuid)`。
    /// 精确路径规则 U（保留拼接入口）。
    pub fn save_folder_path(&self) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::new();
        path.push("saves");
        path.push(self.0.to_string());
        path
    }
    /// O：`backup_folder_path` 0x140b46180；规则 U。
    pub fn backup_folder_path(&self) -> std::path::PathBuf {
        self.save_folder_path()
    }
    /// O：方法存在（`snapshots_folder_path` / `leaf_folder_name`）；规则 U。
    pub fn snapshots_folder_path(&self) -> std::path::PathBuf {
        self.save_folder_path()
    }
    /// O：方法存在（`leaf_folder_name`）；规则 U。
    pub fn leaf_folder_name(&self) -> Gap {
        Gap
    }
    /// O：`session_file_list` 0x140b46300 枚举会话目录文件；输出 U。
    pub fn session_file_list(&self) -> Vec<Gap> {
        Vec::new()
    }
}

/// 会话元数据（O：`SessionMeta` write_to_disk 0x140b463c0 / try_load_from_path
/// 0x140b46480 / assets_root 0x140b46370 / 季节谓词）。字段布局 U。
#[derive(Clone, Debug, Default)]
pub struct SessionMeta {
    /// 字节字段 @+0x08(ptr)/@+0x10(len)；`is_halloween2024` 扫描字节 0、
    /// `is_xmas2024` 扫描字节 2、`is_halloween_any` 扫描字节 >1（O）。
    /// 编码语义 U。
    season_bytes: Vec<u8>,
}

impl SessionMeta {
    /// O：`try_load_from_path` = `std::fs::read_to_string` + anyhow context。
    pub fn try_load_from_path(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let text = std::fs::read_to_string(path)?;
        // 反序列化结构 U；保留输入边界。
        let _ = text;
        Ok(Self::default())
    }
    /// O：`write_to_disk` = save_folder_path.join(meta).save_session_meta_to_file。
    pub fn write_to_disk(&self, _session: &SessionId) -> Result<(), std::io::Error> {
        Ok(())
    }
    /// O：`write_to_folder`（PDB 名）；语义 U。
    pub fn write_to_folder(&self, _folder: &std::path::Path) -> Result<(), std::io::Error> {
        Ok(())
    }
    /// O：`assets_root` = `Path::join`。
    pub fn assets_root(&self, base: &std::path::Path) -> std::path::PathBuf {
        base.join("assets")
    }
    /// O：`is_halloween2024` 扫描字节字段中首次出现 `0` 则真（U 语义）。
    pub fn is_halloween2024(&self) -> bool {
        self.season_bytes.contains(&0)
    }
    /// O：`is_xmas2024` 扫描字节字段中首次出现 `2` 则真（U 语义）。
    pub fn is_xmas2024(&self) -> bool {
        self.season_bytes.contains(&2)
    }
    /// O：`is_halloween_any` 扫描字节字段中任一次出现 >1 则真（U 语义）。
    pub fn is_halloween_any(&self) -> bool {
        self.season_bytes.iter().any(|b| *b > 1)
    }
}

/// 已存档会话（O：`SavedSession` screenshot_path / loading_background_path；
/// 按 `SystemTime` 排序）。
#[derive(Clone, Debug)]
pub struct SavedSession {
    /// 排序键（O：sort_by_key closure_env$1 使用 SystemTime）。
    pub modified: Option<std::time::SystemTime>,
    /// 其余字段（session_id 等）。布局 U。
    pub rest: Gap,
}

impl Default for SavedSession {
    fn default() -> Self {
        Self {
            modified: None,
            rest: Gap,
        }
    }
}

impl SavedSession {
    /// O：方法存在（`screenshot_path`）；规则 U。
    pub fn screenshot_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::new()
    }
    /// O：`loading_background_path` 0x140b46a50；规则 U。
    pub fn loading_background_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::new()
    }
}

/// 存档列表资源（O：`refresh_session_list` 0x140b46b00 清空后 `paths::saves()`
/// readdir 重建；`load_saved_session_list` 按 SystemTime 排序）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SavedSessionList {
    pub sessions: Vec<SavedSession>,
}

impl SavedSessionList {
    /// O：清空并重扫 saves 目录。
    pub fn refresh(&mut self) {
        self.sessions.clear();
    }
}

/// 会话流事件（O：PDB 事件名）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NewSessionStartedCmd(pub Gap);

/// O：`SessionLoadedCmd`（事件；消费链 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SessionLoadedCmd(pub Gap);

/// O：`SessionOnLoadCmd`（enum，drop_in_place 出现）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SessionOnLoadCmd(pub Gap);

/// O：`SessionMutator`（serde Vec<SessionMutator> 反序列化）。
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionMutator(pub Gap);

/// O：`SessionFlowPlugin`（Bevy Plugin，PDB 名）。
pub struct SessionFlowPlugin;

/// 会话事件注册（骨架对齐；原事件集合 U）。
pub fn add_session_events() -> (NewSessionStartedCmd, SessionLoadedCmd, SessionOnLoadCmd) {
    (
        NewSessionStartedCmd(Gap),
        SessionLoadedCmd(Gap),
        SessionOnLoadCmd(Gap),
    )
}
