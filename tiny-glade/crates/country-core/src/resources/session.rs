//! B session owners. Metadata field names follow the original serializers.
use super::image_formation::ImageFormationConfig;
use crate::meta_commander::MetaCommand;
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

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
    /// B 0x140b46300 returns these paths in this order.
    pub fn session_file_list(&self) -> [PathBuf; 3] {
        let folder = self.save_folder_path();
        [
            folder.join("history.json"),
            folder.join("screenshot.jpg"),
            folder.join("meta.json"),
        ]
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
        let envelope = serde_json::json!({"version": 1, "state": self});
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

/// `copy_dir_all<PathBuf,PathBuf>` 0x141740c00：递归目录复制；**"intro" 子目录跳过**
/// （O：名 5 字符 == "intro" 0x72746e69+o）。load_game_session 新会话起始 build 使用。
pub fn copy_dir_all(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        if file_type.is_dir() {
            // "intro" 子目录跳过（O：0x141740c00 名比较）。
            if name.as_encoded_bytes() == b"intro" {
                continue;
            }
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

/// 会话请求的载入方式（PDB：`NextSessionKind`）。原 kind niche =
/// `0x8000000000000000 | decoded`；`load_game` 0x14146d030 用
/// `decoded = kind ^ 0x8000000000000000; sel = min(decoded, 2)` 解码。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum NextSessionKind {
    /// decoded 0：无 on_load 语义。
    None = 0x8000_0000_0000_0000,
    /// decoded 1：parse_saved_game_data（含 upgrade 备份）。
    LoadSavedGame = 0x8000_0000_0000_0001,
    /// decoded 2：on_load 命令回放 + 内存快照（`NextSession::load` 默认）。
    LoadOnLoad = 0x8000_0000_0000_0002,
}
impl NextSessionKind {
    pub const fn decoded(self) -> u64 {
        (self as u64) ^ 0x8000_0000_0000_0000
    }
}

/// 会话 on_load 条目（PDB：`SessionOnLoadCmd`；**0x20B = {tag qword, payload qword}**，
/// re-analyst RNE 原字节修正）。
///
/// - `append_on_load<MetaCommandFence>` 0x141328ac0 写
///   `Command{tag=0x8000000000000000, payload=0x8000000000000003}`；
/// - `append_on_load<[MetaCommand;3]>` 0x1403b5150 写 **批量条目 `{tag=3, Box<0xa8B>}`**
///   （reviewer 二轮修正：tag=3，非 ON_LOAD_VARIANT_MASK|1；载荷为 0xa8B 批量缓冲，
///   非单条 payload）；
/// - load_game 两遍回放以 fence 终止（fence 值 0x8000000000000003）。
/// 变体解码 `tag ^ 0x8000000000000000`，`min(·,2)`。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OnLoadEntry {
    /// 单命令条目：0x20B = {tag, payload}。
    Command { tag: u64, payload: u64 },
    /// 批量条目（机器码 tag=3）：0x20B = {tag=3, Box<0xa8B>}，展开为 3 条 MetaCommand。
    Batch { commands: Vec<MetaCommand> },
}

/// on_load 变体判别位（0x8000000000000000）。
pub const ON_LOAD_VARIANT_MASK: u64 = 0x8000_0000_0000_0000;
/// on_load 终止 fence（0x8000000000000003）。
pub const ON_LOAD_FENCE: u64 = 0x8000_0000_0000_0003;

/// 下一个会话请求资源（PDB：`NextSession`；0xa0B：on_load Vec + kind niche +
/// uuid + ignore_snapshot bool）。
#[derive(Resource, Clone, Debug)]
pub struct NextSession {
    pub on_load: Vec<OnLoadEntry>,
    pub kind: NextSessionKind,
    pub id: SessionId,
    pub ignore_snapshot: bool,
}
impl NextSession {
    /// `NextSession::load` 0x140b46920：{cap-first 空 Vec(cap=8),
    /// kind=0x8000000000000002, uuid, ignore_snapshot=false}。
    pub fn load(id: SessionId) -> Self {
        Self {
            on_load: Vec::new(),
            kind: NextSessionKind::LoadOnLoad,
            id,
            ignore_snapshot: false,
        }
    }
    /// `NextSession::create` 0x140b46960：kind 由调用方模板决定。
    ///
    /// **载荷面（O 调用点 0x141ebb5ad）**：`create(out NextSession, meta_template96B,
    /// option32B)`——96B 模板（rdx）+ 32B 尾部（r8）构造；kind@+0x18 = 模板首 qword；
    /// Vec 清空；另有 24B 可选串/路径 + 字节旗标随模板传递。模板/选项的精确字段语义
    /// U（待取证，不编造），本实现仅保留语义入口 `id + kind`。
    pub fn create(id: SessionId, kind: NextSessionKind) -> Self {
        Self {
            on_load: Vec::new(),
            kind,
            id,
            ignore_snapshot: false,
        }
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
    /// `append_on_load<MetaCommandFence>` 0x141328ac0：追加**变体 0 会话加载命令**
    /// 条目 `{tag=0x8000000000000000, payload=fence}`（RE-SESSION-SWITCH 定案：
    /// payload 是 fence Arc，**非** 0x8000000000000003；fence 释放走变体 2 条目）。
    pub fn append_fence(&mut self, fence: crate::meta_commander::MetaCommandFence) {
        self.on_load.push(OnLoadEntry::Command {
            tag: ON_LOAD_VARIANT_MASK,
            payload: fence.as_payload(),
        });
    }
    /// 追加两遍回放终止哨兵 `{tag=0x8000000000000003, payload=0}`（0x8000000000000003 =
    /// 回放 break 条件；生产者写入残留待追源，RE-SESSION-SWITCH）。
    pub fn append_replay_terminator(&mut self) {
        self.on_load.push(OnLoadEntry::Command {
            tag: ON_LOAD_FENCE,
            payload: 0,
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
    let mut sessions = Vec::new();
    let Ok(entries) = std::fs::read_dir(paths::saves()) else {
        return sessions;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Ok(uuid) = Uuid::try_parse(name) else {
            continue;
        };
        let modified = entry
            .metadata()
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .unwrap_or(std::time::UNIX_EPOCH);
        sessions.push(SavedSession {
            id: SessionId(uuid),
            modified,
            screenshot_exists: path.join("screenshot.jpg").exists(),
            loading_background_exists: path.join("loading_background.jpg").exists(),
        });
    }
    // 最近会话在前（"Latest game session " 语义）。
    sessions.sort_by_key(|session| std::cmp::Reverse(session.modified));
    sessions
}

/// `refresh_session_list` 0x140b46b00：清空后由 `paths::saves()` readdir 重建。
pub fn refresh_session_list(list: &mut SavedSessionList) {
    list.sessions = load_saved_session_list();
}

/// `delete_session` 0x140b46d60：deleted_saves()/uuid 标记文件 → rename 移动 →
/// 两段删除；失败 log "Error deleting glade: "。
pub fn delete_session(id: SessionId) {
    let source = id.save_folder_path();
    let deleted = paths::deleted_saves().join(Uuid::new_v4().to_string());
    if let Err(error) = std::fs::create_dir_all(paths::deleted_saves()) {
        log::error!("Error deleting glade: {error}");
        return;
    }
    // 第一段：写标记文件（rename 的原子前置）。
    if let Err(error) = std::fs::write(deleted.with_extension("deleted"), b"") {
        log::error!("Error deleting glade: {error}");
        return;
    }
    // rename 移动进 deleted-saves。
    if let Err(error) = std::fs::rename(&source, &deleted) {
        log::error!("Error deleting glade: {error}");
        return;
    }
    // 第二段：删除已移动的会话目录。
    if let Err(error) = std::fs::remove_dir_all(&deleted) {
        log::error!("Error deleting glade: {error}");
    }
}

/// 会话事件（PDB：`NewSessionStartedCmd` / `SessionLoadedCmd`）。载荷待取证。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NewSessionStartedCmd;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SessionLoadedCmd;

/// `SessionFlowPlugin`（PDB：`session::SessionFlowPlugin`；Bevy Plugin）。
/// country-core 不依赖 bevy_app；`build` 登记资源插入与事件注册合同，由宿主 App 装配。
#[derive(Clone, Copy, Debug, Default)]
pub struct SessionFlowPlugin;
impl SessionFlowPlugin {
    /// 原 build 0x140b47840：插入 ActiveSession/SavedSessionList/NextSession/
    /// AutoSaveState 资源并注册会话事件。ActiveSession 为占位默认，真实会话启动后替换。
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        let mut list = SavedSessionList::default();
        refresh_session_list(&mut list);
        world.insert_resource(list);
        world.insert_resource(NextSession::load_latest(world.resource::<SavedSessionList>()));
        world.init_resource::<crate::systems::save::save_game::AutoSaveState>();
        world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        world.init_resource::<bevy_ecs::event::Events<NewSessionStartedCmd>>();
        world.init_resource::<bevy_ecs::event::Events<SessionLoadedCmd>>();
    }
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
        assert_eq!(saved["version"], 1);
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
