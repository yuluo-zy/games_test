//! B session owners. Metadata field names follow the original serializers.
use super::image_formation::ImageFormationConfig;
use crate::meta_commander::MetaCommand;
use bevy_ecs::prelude::{EventReader, Res, ResMut};
use bevy_ecs::resource::Resource;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::systems::save::save_game::{DeleteSessionCmd, SavingGameComplete};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);
impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn save_folder_path(&self) -> PathBuf {
        paths::saves().join(self.0.to_string())
    }
    pub fn backup_folder_path(&self) -> PathBuf {
        paths::backup_saves().join(self.0.to_string())
    }
    pub fn snapshots_folder_path(&self) -> PathBuf {
        self.save_folder_path().join("snapshots")
    }
    /// B 0x140b46300 returns `Vec<RelativePathBuf>`（相对路径，**不** join save_folder）。
    /// 顺序经 rdata 确认：history.json / screenshot.jpg / meta.json（0x142aef4f7/0x503/0x511）。
    #[allow(clippy::unused_self)]
    pub fn session_file_list(&self) -> Vec<RelativePathBuf> {
        ["history.json", "screenshot.jpg", "meta.json"]
            .into_iter()
            .map(RelativePathBuf::from)
            .collect()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
pub enum GladeType {
    #[default]
    Summer = 0,
    Olden = 1,
    Flowery = 2,
    Autumn = 3,
    Winter = 4,
}
impl GladeType {
    pub const ALL: [Self; 5] = [
        Self::Summer,
        Self::Olden,
        Self::Flowery,
        Self::Autumn,
        Self::Winter,
    ];
    pub fn native_tag(self) -> u8 {
        self as u8
    }
    pub fn assets_folder(self) -> &'static str {
        match self {
            Self::Summer => "summer",
            Self::Olden => "olden",
            Self::Flowery => "flowery",
            Self::Autumn => "autumn",
            Self::Winter => "winter",
        }
    }
    pub fn is_olden(self) -> bool {
        self == Self::Olden
    }
    pub fn is_flowery(self) -> bool {
        self == Self::Flowery
    }
    pub fn is_autumn(self) -> bool {
        self == Self::Autumn
    }
    pub fn is_winter(self) -> bool {
        self == Self::Winter
    }
    pub fn audio_id(self) -> f32 {
        [0.0, 3.0, 1.0, 2.0, 4.0][self as usize]
    }
    fn parse<E: serde::de::Error>(name: &str) -> Result<Self, E> {
        match name {
            "Summer" | "summer" => Ok(Self::Summer),
            "Olden" | "olden" => Ok(Self::Olden),
            "Flowery" | "flowery" => Ok(Self::Flowery),
            "Autumn" | "autumn" => Ok(Self::Autumn),
            "Winter" | "winter" => Ok(Self::Winter),
            _ => Err(E::unknown_variant(
                name,
                &["Summer", "Olden", "Flowery", "Autumn", "Winter"],
            )),
        }
    }
}
impl From<u8> for GladeType {
    fn from(tag: u8) -> Self {
        Self::ALL
            .get(tag as usize)
            .copied()
            .expect("invalid native GladeType")
    }
}
impl<'de> Deserialize<'de> for GladeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GladeType;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a GladeType variant")
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                GladeType::parse(value)
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                let (name, variant) = data.variant::<String>()?;
                let theme = GladeType::parse(&name)?;
                serde::de::VariantAccess::unit_variant(variant)?;
                Ok(theme)
            }
        }
        deserializer.deserialize_enum(
            "GladeType",
            &["Summer", "Olden", "Flowery", "Autumn", "Winter"],
            Visitor,
        )
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionMutator {
    Halloween2024 = 0,
    Halloween2025 = 1,
    Xmas2024 = 2,
}

/// Exactly three fields (0x141329ca0, table @0x142c3a890). `state` is an
/// outer file-envelope field. Mutators default empty; image None is omitted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionMeta {
    pub glade: GladeType,
    #[serde(default)]
    pub mutators: Vec<SessionMutator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageFormationConfig>,
}
impl SessionMeta {
    pub fn assets_root(&self) -> PathBuf {
        Path::new("assets/glade").join(self.glade.assets_folder())
    }
    pub fn is_halloween2024(&self) -> bool {
        self.mutators.contains(&SessionMutator::Halloween2024)
    }
    pub fn is_halloween_any(&self) -> bool {
        self.mutators.iter().any(|m| {
            matches!(
                m,
                SessionMutator::Halloween2024 | SessionMutator::Halloween2025
            )
        })
    }
    pub fn is_xmas2024(&self) -> bool {
        self.mutators.contains(&SessionMutator::Xmas2024)
    }
    /// 0x140b46480: read JSON, get `state`, deserialize. Outer version is not checked.
    pub fn try_load_from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
        Ok(serde_json::from_value(
            value.get("state").cloned().unwrap_or_default(),
        )?)
    }
    pub fn write_to_disk(&self, id: SessionId) -> anyhow::Result<()> {
        self.write_to_path(id.save_folder_path().join("meta.json"))
    }
    pub fn write_to_path(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        use std::io::Write;
        let path = path.as_ref();
        // 0x1417522e0：外层 envelope `{"version": 2, "state": SessionMeta}`（BTreeMap
        // insert "version"→2 + "state"→serialize）。Rust 此前误写 version=1。
        let envelope = serde_json::json!({"version": 2, "state": self});
        // 0x1417522e0: ensure parent, compact Value display, write/sync the
        // sibling with extension "tmp", close it, then rename to the target.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let temporary = path.with_extension("tmp");
        let data = envelope.to_string();
        let mut file = std::fs::File::create(&temporary)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(temporary, path)?;
        Ok(())
    }
}

/// Native serializer 0x1403b5920: meta @0, id @0x60, no path/name fields.
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSession {
    pub id: SessionId,
    pub meta: SessionMeta,
}
impl ActiveSession {
    pub fn new(id: SessionId, meta: SessionMeta) -> Self {
        Self { id, meta }
    }
    pub fn assets_root(&self) -> PathBuf {
        self.meta.assets_root()
    }
    pub fn is_olden(&self) -> bool {
        self.meta.glade.is_olden()
    }
    pub fn is_flowery(&self) -> bool {
        self.meta.glade.is_flowery()
    }
    pub fn is_autumn(&self) -> bool {
        self.meta.glade.is_autumn()
    }
    pub fn is_winter(&self) -> bool {
        self.meta.glade.is_winter()
    }
}

// ---------------------------------------------------------------------------
// NextSession / SessionOnLoadCmd / NextSessionKind（PDB：resources::session）
// ---------------------------------------------------------------------------

/// 会话载入尝试（PDB：`SessionLoadAttempt`；startup_session，内联行 79）。
///
/// 布局（`startup_session.rs.md`）：`*param_1+0x10` 空 → theme 默认 3；否则
/// attempt → theme byte @+6。theme 从会话 meta 读取，失败回退默认 3（Autumn）。
#[derive(Clone, Debug, PartialEq)]
pub struct SessionLoadAttempt {
    /// 目标会话 id（None = 无有效会话 → 新会话路径）。
    pub id: Option<SessionId>,
    /// 主题（`theme byte @+6`；O 默认 3 = Autumn）。
    pub theme: GladeType,
}
impl SessionLoadAttempt {
    /// `from_session_id`（内联行 79）：由会话 id 构造载入尝试；meta 读取失败回退
    /// 默认 theme 3（O：`*param_1+0x10` 空 → 3）。
    pub fn from_session_id(id: SessionId) -> Self {
        let theme = SessionMeta::try_load_from_path(id.save_folder_path().join("meta.json"))
            .map(|meta| meta.glade)
            .unwrap_or(GladeType::Autumn);
        Self {
            id: Some(id),
            theme,
        }
    }
    /// 继续最近会话（O：'Latest game session ' 路径）；列表空 → 新会话。
    pub fn continue_latest(list: &SavedSessionList) -> Self {
        match list.sessions.first() {
            Some(session) => Self::from_session_id(session.id),
            None => Self {
                id: None,
                theme: GladeType::Autumn,
            },
        }
    }
}

/// 会话 meta 来源（PDB：`SessionMetaSource`；枚举类型，sysres/public-session-names，
/// 字段 U——只登记判别路径，不编造变体载荷）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SessionMetaSource {
    /// 继续/已有会话（load_game_session case 1/2 'Latest/Continuing game session '）。
    Existing,
    /// 新会话（'Starting a new game session'）。
    New,
}

/// `copy_dir_all<PathBuf,PathBuf>` 0x141740c00：**纯递归复制**——`create_dir_all` →
/// `read_dir` → 目录条目递归 `copy_dir_all` / 文件条目 `std::fs::copy`。原生 disasm
/// 全 CALL 清单**无排序、无 "intro" 比较**（旧描述"intro 跳过"已按 fresh 作废）。
/// `load_game_session` 新会话起始 build 使用。注意 `startup/startup_session` 的
/// `copy_dir_all` 才是 0x141740c00 锚点；本函数不重复锚定该 VA。
pub fn copy_dir_all(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &to.join(name))?;
        } else {
            std::fs::copy(entry.path(), to.join(name))?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// NextSession / SessionOnLoadCmd / NextSessionKind（PDB：resources::session）
// ---------------------------------------------------------------------------

/// 会话请求的语义判别。原 startup_session 14166e7d0 有三个高位 niche
/// （最近/指定/已解析），新建则是持有路径和完整元数据的数据变体。
/// 此安全枚举不是原版 Rust enum ABI；不能与下游 LoadGameSource 的判别混用。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum NextSessionKind {
    /// decoded 0：无 on_load 语义。
    None = 0x8000_0000_0000_0000,
    /// decoded 1：parse_saved_game_data（含 upgrade 备份）。
    LoadSavedGame = 0x8000_0000_0000_0001,
    /// decoded 2：on_load 命令回放 + 内存快照（`NextSession::load` 默认）。
    LoadOnLoad = 0x8000_0000_0000_0002,
    /// Load UI already parsed this save; the request owns the complete result.
    LoadParsedGame = 0x8000_0000_0000_0003,
    /// 安全 Rust 适配标记：原版是持有路径和完整 SessionMeta 的数据变体，
    /// 使用 String 容量 niche，不能把首 qword 当作固定整数 tag。
    Create = 0x8000_0000_0000_0004,
}
impl NextSessionKind {
    pub const fn decoded(self) -> u64 {
        (self as u64) ^ 0x8000_0000_0000_0000
    }
}

/// 会话 on_load 条目（PDB：`SessionOnLoadCmd`；**0x20B = 4 qword
/// `{tag, payload_a, payload_b, payload_c}`**，fresh 反编译 0x14146d030 逐条目 +0x20 步进
/// 与 append_on_load 0x141328ac0 的 `+len*0x20` 写入互证）。
///
/// - `append_on_load<MetaCommandFence>` 0x141328ac0 写
///   `Command{tag=0x8000000000000000, payload_a=fence Arc}`；
/// - `append_on_load<[MetaCommand;3]>` 0x1403b5150 写 **批量条目 `{tag=3, Box<0xa8B>}`**
///   （reviewer 二轮修正：tag=3，非 ON_LOAD_VARIANT_MASK|1；载荷为 0xa8B 批量缓冲）；
/// - load_game 两遍回放以 fence 终止（tag 0x8000000000000003 break）。
/// 变体解码 `decoded = tag ^ 0x8000000000000000`，**`decoded > 2 → 1`**（非 min(·,2)）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OnLoadEntry {
    /// 加载完成信号；持有 Arc，复制、取消或重试不会泄漏引用或解引用裸地址。
    Fence(crate::meta_commander::MetaCommandFence),

    /// 单命令条目：0x20B = {tag, payload_a, payload_b, payload_c}。
    Command {
        tag: u64,
        payload_a: u64,
        payload_b: u64,
        payload_c: u64,
    },
    /// 批量条目（机器码 tag=3）：0x20B = {tag=3, Box<0xa8B>}，展开为 3 条 MetaCommand。
    Batch { commands: Vec<MetaCommand> },
}

/// on_load 变体判别位（0x8000000000000000）。
pub const ON_LOAD_VARIANT_MASK: u64 = 0x8000_0000_0000_0000;
/// on_load 终止 fence（0x8000000000000003）。
pub const ON_LOAD_FENCE: u64 = 0x8000_0000_0000_0003;

/// 下一个会话请求的安全所有者。原版0xa0B的+0x18..+0x98是整个数据枚举，
/// 不是固定kind+uuid+整数载荷：新建含路径和SessionMeta，已解析含拥有的解析结果。
/// 整数载荷仅为既有纯ABI探针兼容保留；生产新建/已解析消费者读取各自的拥有值。
#[derive(Resource, Clone, Debug)]
pub struct NextSession {
    pub on_load: Vec<OnLoadEntry>,
    pub kind: NextSessionKind,
    pub id: SessionId,
    /// 旧纯构造探针的整数载荷；不能包含分配地址，也不能用于恢复新建会话。
    pub template_payload: [u64; 9],
    /// +0x78..+0x98：`create` 的 32B 尾部载荷（4 qword，字段语义 U，原样复制）。
    pub tail_payload: [u64; 4],
    pub ignore_snapshot: bool,
    /// Safe owner replacing the native Box+metadata wrapper at +0x30.
    /// Never put an allocation address in template_payload.
    parsed_data: Option<Box<crate::systems::save::load_game::ParsedGameData>>,
    new_session: Option<NewSessionOptions>,
}

/// 原版新建请求拥有的完整数据。空路径选择标准起始场景；每日主题拥有其 history.json 路径。
/// 不把原生 String、Vec 或 Option 的指针和填充编码到整数数组中。
#[derive(Clone, Debug, PartialEq)]
pub struct NewSessionOptions {
    pub history_path: Option<PathBuf>,
    pub metadata: SessionMeta,
}

/// 既有纯构造探针适配器。原140b46960只宽拷贝96B+32B数据，
/// 其前24B在新建分支实际是String，而非kind/id。生产新建必须用new_game拥有真实数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NextSessionTemplate {
    pub kind: NextSessionKind,
    pub id: SessionId,
    /// 载荷（9 qword，字段语义 U；build/测试等默认零）。
    pub payload: [u64; 9],
}
impl NextSessionTemplate {
    pub fn new(id: SessionId, kind: NextSessionKind) -> Self {
        Self {
            kind,
            id,
            payload: [0; 9],
        }
    }
}

/// `NextSession::create` 的 32B 尾部（O 0x140b46960 r8 4 qword：`+0x78..+0x98`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NextSessionTail {
    /// 尾部载荷（4 qword，字段语义 U）。
    pub words: [u64; 4],
}
impl NextSessionTail {
    pub const EMPTY: Self = Self { words: [0; 4] };
}

impl NextSession {
    /// `NextSession::load` 0x140b46920：{cap-first 空 Vec(cap=8),
    /// kind=0x8000000000000002, uuid, 载荷零, ignore_snapshot=false}。
    pub fn load(id: SessionId) -> Self {
        Self {
            on_load: Vec::new(),
            kind: NextSessionKind::LoadOnLoad,
            id,
            template_payload: [0; 9],
            tail_payload: [0; 4],
            ignore_snapshot: false,
            parsed_data: None,
            new_session: None,
        }
    }
    /// 保留旧探针和启动最近会话入口；不会把整数伪装成字符串、元数据或解析结果。
    /// 新建和已解析请求分别用new_game/from_parsed。
    pub fn create(template: &NextSessionTemplate, tail: &NextSessionTail) -> Self {
        Self {
            on_load: Vec::new(),
            kind: template.kind,
            id: template.id,
            template_payload: template.payload,
            tail_payload: tail.words,
            ignore_snapshot: false,
            parsed_data: None,
            new_session: None,
        }
    }
    /// 原 B 新建菜单 @141ebb501..5ad：转交起始历史路径和所选主题的完整元数据。
    pub fn new_game(metadata: SessionMeta, history_path: Option<PathBuf>) -> Self {
        let mut request = Self::load(SessionId(Uuid::nil()));
        request.kind = NextSessionKind::Create;
        request.new_session = Some(NewSessionOptions {
            history_path,
            metadata,
        });
        request
    }

    pub fn new_session_options(&self) -> Option<&NewSessionOptions> {
        self.new_session.as_ref()
    }
    /// Ordinary load, backup restore and theme switch transfer the actual parsed
    /// result (@141e20d5c..e1a / @140fc4cc3..4d6f), without reopening history.json.
    pub fn from_parsed(
        id: SessionId,
        data: crate::systems::save::load_game::ParsedGameData,
    ) -> Self {
        let mut request = Self::load(id);
        request.kind = NextSessionKind::LoadParsedGame;
        request.parsed_data = Some(Box::new(data));
        request
    }

    /// Borrow the actual owned parse result without cloning or exposing mutation.
    /// Startup retains the sole consuming operation through take_parsed_data.
    pub fn parsed_data(&self) -> Option<&crate::systems::save::load_game::ParsedGameData> {
        self.parsed_data.as_deref()
    }

    /// Move only after startup has prepared the destination metadata/settings.
    /// A malformed public template must fail before publishing partial state.
    pub(crate) fn take_parsed_data(
        &mut self,
    ) -> anyhow::Result<Box<crate::systems::save::load_game::ParsedGameData>> {
        anyhow::ensure!(
            self.kind == NextSessionKind::LoadParsedGame,
            "NextSession does not own parsed game data"
        );
        self.parsed_data.take().ok_or_else(|| {
            anyhow::anyhow!("NextSession parsed variant has no owned ParsedGameData")
        })
    }

    /// `NextSession::ignore_snapshot` 0x140b46a20：写 +0x98 bool（builder）。
    pub fn ignore_snapshot(mut self, ignore: bool) -> Self {
        self.ignore_snapshot = ignore;
        self
    }
    /// `NextSession::load_latest`（内联，行 331，kind 解码 sel==1）：载入**最近会话**。
    ///
    /// 真实语义：从 [`SavedSessionList`]（最近在前排序）取第一个会话 id；列表空时
    /// 回退新会话 id（并标记 I：UI/会话流生产者为空时的默认路径）。
    pub fn load_latest(list: &SavedSessionList) -> Self {
        match list.sessions.first() {
            Some(session) => Self::load(session.id),
            None => Self::load(SessionId::new()),
        }
    }
    /// 加载完成后释放信号。请求、管理器和调用方各自持有安全的 Arc 引用。
    pub fn append_fence(&mut self, fence: crate::meta_commander::MetaCommandFence) {
        self.on_load.push(OnLoadEntry::Fence(fence));
    }
    /// 追加两遍回放终止哨兵 `{tag=0x8000000000000003}`（0x8000000000000003 =
    /// 回放 break 条件；生产者写入残留待追源，RE-SESSION-SWITCH）。
    pub fn append_replay_terminator(&mut self) {
        self.on_load.push(OnLoadEntry::Command {
            tag: ON_LOAD_FENCE,
            payload_a: 0,
            payload_b: 0,
            payload_c: 0,
        });
    }
    /// `append_on_load<[MetaCommand;3]>` 0x1403b5150：追加**批量条目**（机器码 tag=3，
    /// 载荷 = Box<0xa8B>；reviewer 二轮修正，非单条 payload）。
    pub fn extend_on_load(&mut self, commands: impl IntoIterator<Item = MetaCommand>) {
        let commands: Vec<MetaCommand> = commands.into_iter().collect();
        self.on_load.push(OnLoadEntry::Batch { commands });
    }
}

// ---------------------------------------------------------------------------
// SavedSession / SavedSessionList / 会话列表与删除
// ---------------------------------------------------------------------------

/// 会话列表条目（PDB：`SavedSession`）。rhapsody 缩略图元数据消费待接入。
#[derive(Clone, Debug)]
pub struct SavedSession {
    pub id: SessionId,
    pub modified: std::time::SystemTime,
    /// B +0x18, written by load_saved_session_list closure @0x1413285f4:
    /// is_session_ok_to_load returns tags 0/1/2; the thumbnail shader receives !=0.
    pub readiness: crate::systems::save::load_completion_checker::SessionLoadReadiness,
    pub screenshot_exists: bool,
    pub loading_background_exists: bool,
}
impl SavedSession {
    /// `SavedSession::loading_background_path` 0x140b46a50：save_folder +
    /// "loading_background.jpg"（PDB 名 screenshot_path 错位）。
    pub fn loading_background_path(&self) -> PathBuf {
        self.id.save_folder_path().join("loading_background.jpg")
    }
    /// 实际截图路径（行 403；与 loading_background_path 相对）。
    pub fn screenshot_path(&self) -> PathBuf {
        self.id.save_folder_path().join("screenshot.jpg")
    }
}

/// 会话列表资源（PDB：`SavedSessionList`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SavedSessionList {
    pub sessions: Vec<SavedSession>,
}

/// `load_saved_session_list`（内联，行 420）：扫 `paths::saves()`，对 UUID 子目录
/// 构建 `SavedSession`，按修改时间排序（`sort_by_key`，O）。rhapsody 缩略图过滤
/// 待接入。
pub fn load_saved_session_list() -> Vec<SavedSession> {
    load_saved_session_list_in(paths::saves(), None)
}

/// Native filter_map closure @0x141328260: UUID directory + history.json
/// regular file + successful history modification time. The optional save
/// owner's unsaved list participates in the saved readiness byte.
pub fn load_saved_session_list_in(
    root: impl AsRef<Path>,
    unsaved: Option<&crate::systems::save::save_thread::UnsavedFileList>,
) -> Vec<SavedSession> {
    use crate::systems::save::load_completion_checker::{
        SessionLoadReadiness, is_session_ok_to_load_in,
    };
    let mut sessions = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return sessions;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Ok(uuid) = Uuid::try_parse(&name) else {
            continue;
        };
        let history = path.join("history.json");
        if !path.is_dir() || !history.is_file() {
            continue;
        }
        let Ok(modified) = std::fs::metadata(&history).and_then(|m| m.modified()) else {
            continue;
        };
        let readiness = match unsaved {
            Some(unsaved) => is_session_ok_to_load_in(&path, unsaved),
            None if path.join("loading.tmp").exists() => SessionLoadReadiness::NotReady,
            None => SessionLoadReadiness::Ok,
        };
        sessions.push(SavedSession {
            id: SessionId(uuid),
            modified,
            readiness,
            screenshot_exists: path.join("screenshot.jpg").exists(),
            loading_background_exists: path.join("loading_background.jpg").exists(),
        });
    }
    sessions.sort_by_key(|session| std::cmp::Reverse(session.modified));
    sessions
}

/// `refresh_session_list` 0x140b46b00：清空后由 `paths::saves()` readdir 重建。
pub fn refresh_session_list(list: &mut SavedSessionList) {
    list.sessions = load_saved_session_list();
}

/// `delete_session` 0x140b46d60：构 source+deleted(uuid) 路径 → exists() 冲突检查 →
/// 冲突时重生成 uuid → rename(source,deleted) → 失败 log "Error deleting glade: "。
/// **无 create_dir_all / 写 ".deleted" 标记 / remove_dir_all**（Rust 旧实现自造三段，已删）。
pub fn delete_session(id: SessionId) {
    let source = id.save_folder_path();
    let mut deleted = paths::deleted_saves().join(id.0.to_string());
    if deleted.exists() {
        deleted = paths::deleted_saves().join(Uuid::new_v4().to_string());
    }
    if let Err(error) = std::fs::rename(&source, &deleted) {
        log::error!("Error deleting glade: {error}");
    }
}

/// 会话事件（PDB：`NewSessionStartedCmd` / `SessionLoadedCmd`）。载荷待取证。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NewSessionStartedCmd;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SessionLoadedCmd;

/// `SessionFlowPlugin`（PDB：`session::SessionFlowPlugin`；Bevy Plugin）。
/// country-core 不依赖 bevy_app；`build` 登记资源插入与事件注册合同，系统经
/// [`configure_session_flow_systems`] 由宿主接 Schedule。
#[derive(Clone, Copy, Debug, Default)]
pub struct SessionFlowPlugin;
impl SessionFlowPlugin {
    /// 原 build 0x140b47840（fresh 2026-10-17）：
    /// 1. insert **空** SavedSessionList（Vec 空；**不急切 refresh**——刷新由 Startup 系统做）；
    /// 2. insert NextSession::create（kind=`0x8000000000000001`=LoadSavedGame，nil uuid，零载荷）；
    /// 3. Startup `refresh_session_list_system`（`ResMut<SavedSessionList>,
    ///    Option<Res<AutoSaveState>>`；build **不插入** AutoSaveState）；
    /// 4. Update `delete_session_system.run_if(on_event<DeleteSessionCmd>)` +
    ///    `refresh_session_list_system.run_if(or(on_event<SavingGameComplete>,
    ///    on_event<DeleteSessionCmd>))`（见 [`configure_session_flow_systems`]）；
    /// 5. register_event NewSessionStartedCmd / SessionLoadedCmd / DeleteSessionCmd。
    /// **不插 ActiveSession**（原版无此资源；ActiveSession 由 load_game_session 生产）。
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        world.insert_resource(SavedSessionList::default());
        world.insert_resource(NextSession::create(
            &NextSessionTemplate::new(SessionId(Uuid::nil()), NextSessionKind::LoadSavedGame),
            &NextSessionTail::EMPTY,
        ));
        world.init_resource::<bevy_ecs::event::Events<NewSessionStartedCmd>>();
        world.init_resource::<bevy_ecs::event::Events<SessionLoadedCmd>>();
        world.init_resource::<bevy_ecs::event::Events<DeleteSessionCmd>>();
    }
}

/// SessionFlowPlugin 的 Startup 刷新系统（O build `add_systems<Startup,
/// fn(ResMut<SavedSessionList>, Option<Res<AutoSaveState>>)>`）。AutoSaveState 为可选参
/// 匹配原生签名（crate 内刷新逻辑不消费它）。
pub fn refresh_session_list_system(
    mut list: ResMut<SavedSessionList>,
    auto_save: Option<Res<crate::systems::save::save_game::AutoSaveState>>,
) {
    list.sessions = load_saved_session_list_in(
        paths::saves(),
        auto_save.as_ref().map(|save| save.unsaved_files.as_ref()),
    );
}

/// DeleteSessionCmd 处理器（O：`fn(EventReader<DeleteSessionCmd>)`）。
///
/// Fresh 反编译 0x140b46d60（2026-10-17）：函数开头把 reader 游标写成 events 总计数
/// （完整消费），随后只取**最末**事件的 16 B 载荷（`read().last()`，事件步进 0x18 =
/// event_id + SessionId）执行一次删除。旧实现 `for ... in reader.read()` 会删除本帧
/// 每个事件，与原生只删最末一个不符。
pub fn delete_session_system(mut reader: EventReader<DeleteSessionCmd>) {
    if let Some(cmd) = reader.read().last() {
        delete_session(cmd.session);
    }
}

/// 装配 SessionFlowPlugin 的系统（O 0x140b47840，fresh 2026-10-17）：
/// - Startup 刷新系统（`refresh_session_list_system`）；
/// - Update `(delete_session_system.run_if(on_event::<DeleteSessionCmd>),
///   refresh_session_list_system.run_if(or(on_event<SavingGameComplete>,
///   on_event<DeleteSessionCmd>)))`。
/// crate 无 bevy_app 的 Startup/Update 分表，宿主按运行期语义接入同一 Schedule。
pub fn configure_session_flow_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    use bevy_ecs::schedule::Condition as _;
    use bevy_ecs::schedule::IntoScheduleConfigs as _;
    use bevy_ecs::schedule::common_conditions::on_event;
    schedule.add_systems(refresh_session_list_system);
    schedule.add_systems((
        delete_session_system.run_if(on_event::<DeleteSessionCmd>),
        refresh_session_list_system
            .run_if(on_event::<DeleteSessionCmd>.or(on_event::<SavingGameComplete>)),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_session_metadata_schema_and_file_roundtrip() {
        for (tag, names) in [
            (GladeType::Summer, ["Summer", "summer"]),
            (GladeType::Olden, ["Olden", "olden"]),
            (GladeType::Flowery, ["Flowery", "flowery"]),
            (GladeType::Autumn, ["Autumn", "autumn"]),
            (GladeType::Winter, ["Winter", "winter"]),
        ] {
            for name in names {
                assert_eq!(
                    serde_json::from_value::<GladeType>(name.into()).unwrap(),
                    tag
                );
            }
        }
        assert!(serde_json::from_str::<GladeType>("\"SuMmEr\"").is_err());
        assert!(serde_json::from_str::<GladeType>("\"SUMMER\"").is_err());
        let absent: SessionMeta = serde_json::from_str(r#"{"glade":"summer"}"#).unwrap();
        assert!(absent.mutators.is_empty());
        assert!(absent.image.is_none());
        assert_eq!(
            serde_json::to_value(&absent).unwrap(),
            serde_json::json!({"glade":"Summer", "mutators":[]})
        );

        let meta = SessionMeta {
            glade: GladeType::Winter,
            mutators: vec![SessionMutator::Xmas2024],
            image: Some(ImageFormationConfig {
                name: "warm".into(),
                params_18: [0.8, -0.5, 1.1, 0.9],
                flag_28: 1,
                params_30: [0.2, 0.015],
                word_38: 0.3_f32.to_bits(),
                flag_3c: 1,
                flag_3d: 0,
                ..ImageFormationConfig::default()
            }),
        };
        let value = serde_json::to_value(&meta).unwrap();
        assert_eq!(
            value["image"],
            serde_json::json!({
                "grading": {"bypass_color":true,"current_grading_lut":"warm",
                    "grading_strength":0.8_f32,"ev_shift":-0.5_f32,
                    "contrast":1.1_f32,"saturation":0.9_f32},
                "camera_lens":{"vignette":0.2_f32,"chromatic_aberration":0.015_f32,"barrel_distortion":0.3_f32},
                "artistic":{"lighting_only":true,"tree_see_thru":false}
            })
        );
        let mut missing = value["image"].clone();
        missing["grading"]
            .as_object_mut()
            .unwrap()
            .remove("saturation");
        assert!(serde_json::from_value::<ImageFormationConfig>(missing).is_err());
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("nested/meta.json");
        meta.write_to_path(&path).unwrap();
        assert_eq!(SessionMeta::try_load_from_path(&path).unwrap(), meta);
        assert!(!path.with_extension("tmp").exists());
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(saved.as_object().unwrap().len(), 2);
        assert_eq!(saved["version"], 2);
        assert_eq!(
            serde_json::from_value::<SessionMeta>(saved["state"].clone()).unwrap(),
            meta
        );
        let active = ActiveSession::new(SessionId(Uuid::nil()), meta);
        let value = serde_json::to_value(&active).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 2);
        assert!(value.get("id").is_some() && value.get("meta").is_some());
    }
}
