//! B `resources/history/tree_edit.rs` — 树编辑负载（章节 O；25 函数）。
//!
//! `TreeEdit`：{Place/Move} + tree{id, position, scale, rotation_degrees, prefab_name,
//! is_preplaced}；missing_prefab 0x1409e6210；TreeCanopyMaxCullDistance。
//! serde 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60。
//! 树编辑为普通编辑（无独立 replay 文件；place/move 数据恢复）。
use glam::Vec3;

/// 树编辑操作。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum TreeEditKind {
    #[default]
    Place = 0,
    Move = 1,
    /// 删除（O：0x58B 树命令变体 2；reviewer-A 审查结论——删除语义不得以
    /// Move 静默冒充，组 A 修复补入）。
    Delete = 2,
}

/// 树数据载荷（字段序按 fresh decomp-0x14003d4e0 重建：
/// `position(Vec3) → scale(f32) → rot(f32) → id(i32) → prefab(String) →
/// is_preplaced(bool)`；旧 src 的 `id` 首字段且为 u64 不符原序/型）。
/// serde 由下方手写 `Serialize` 提供（原版手写 `serialize` 族）。
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct TreeEditData {
    /// Original Place/Delete outer index (u64), or Move index. This runtime
    /// binding is serialized by TreeEdit, never as a seventh TreeData field.
    /// Standalone TreeData decoding cannot infer a unique storage identity.
    #[serde(skip)]
    pub storage_index: Option<crate::resources::tree_storage::TreeStorageIndex>,
    pub position: Vec3,
    pub scale: f32,
    /// 度制旋转（原字段名登记 `rot`；序列化第三 entry）。
    pub rotation_degrees: f32,
    /// Placement variant ID (source placement+20), independent of the unique
    /// TreeStorage index carried by the outer TreeEdit. Original serde i32.
    pub id: i32,
    pub prefab_name: String,
    pub is_preplaced: bool,
    /// **移动前位置**（运行时旧位；O：live TreeEdit payload `+0x14/+0x1c`，
    /// `replay_tree` Move-undo 0x141bc34f0 与 ui_move_tree History 载荷使用）。
    /// 存档格式（0x14003d4e0）只有六 entry、不含旧位 → 下方手写 `Serialize`
    /// **不写**该字段；反序列化缺省为零。
    #[serde(default)]
    pub previous_position: Vec3,
}

/// 手写 `Serialize`（0x14003d4e0）：单 map，六 entry 序
/// `position / scale / rotation_degrees / id(i32) / prefab_name / is_preplaced`。
///
/// fresh 0x14003d4e0 disasm 键常量：0x14292c1bf len 8 `"position"`、
/// 0x14292c1c7 len 5 `"scale"`、0x14292c1cc len 0x10 `"rotation_degrees"`、
/// 0x14292c1bd len 2 `"id"`、0x14292c1dc len 0xb `"prefab_name"`、
/// 0x14292c1e7 len 0xc `"is_preplaced"`（旧 src 的 `"rot"`/`"prefab"` 被证伪）。
impl serde::Serialize for TreeEditData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("position", &self.position)?;
        map.serialize_entry("scale", &self.scale)?;
        map.serialize_entry("rotation_degrees", &self.rotation_degrees)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("prefab_name", &self.prefab_name)?;
        map.serialize_entry("is_preplaced", &self.is_preplaced)?;
        map.end()
    }
}

/// 树编辑负载。
#[derive(Clone, Debug, PartialEq)]
pub enum TreeEdit {
    Place(TreeEditData),
    Move(TreeEditData),
    /// 删除（O：变体 2）。
    Delete(TreeEditData),
}
impl Default for TreeEdit {
    fn default() -> Self {
        Self::Place(TreeEditData::default())
    }
}

impl TreeEditData {
    /// Original Delete/Place capture: separate the storage index from the first
    /// 24 bytes of the existing PrefabInstance placement. No ID is synthesized.
    pub fn from_placement(
        index: crate::resources::tree_storage::TreeStorageIndex,
        placement: &crate::resources::placement_data::PlacementDataSsbo,
        prefab_name: String,
        is_preplaced: bool,
    ) -> Self {
        Self {
            storage_index: Some(index),
            position: placement.position,
            scale: placement.scale,
            rotation_degrees: placement.rotation_angle,
            id: placement.id,
            prefab_name,
            is_preplaced,
            previous_position: placement.position,
        }
    }

    /// Existing enum API represents Move with TreeEditData as well. Native
    /// Move has only index/from/to; the other fields below are inactive host
    /// representation and are never emitted by TreeEdit's Move serializer.
    pub fn for_move(
        index: crate::resources::tree_storage::TreeStorageIndex,
        from: Vec3,
        to: Vec3,
    ) -> Self {
        Self {
            storage_index: Some(index),
            position: to,
            previous_position: from,
            scale: 0.0,
            rotation_degrees: 0.0,
            id: 0,
            prefab_name: String::new(),
            is_preplaced: false,
        }
    }
}

// B 14003d650 is the outer serializer, distinct from the six-field TreeData
// serializer 14003d4e0. Place/Delete save index and nested tree; Move saves
// index/from/to. The current public enum constructors remain unchanged.
impl serde::Serialize for TreeEdit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(serde::Serialize)]
        enum Saved<'a> {
            Place { index: u64, tree: &'a TreeEditData },
            Move { index: u64, from: Vec3, to: Vec3 },
            Delete { index: u64, tree: &'a TreeEditData },
        }
        let data = match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d,
        };
        let index = data
            .storage_index
            .ok_or_else(|| {
                <S::Error as serde::ser::Error>::custom(
                    "TreeEdit has no bound TreeStorageIndex; placement id is not a storage index",
                )
            })?
            .0;
        let saved = match self {
            Self::Place(tree) => Saved::Place { index, tree },
            Self::Move(d) => Saved::Move {
                index,
                from: d.previous_position,
                to: d.position,
            },
            Self::Delete(tree) => Saved::Delete { index, tree },
        };
        serde::Serialize::serialize(&saved, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for TreeEdit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        enum Saved {
            Place { index: u64, tree: TreeEditData },
            Move { index: u64, from: Vec3, to: Vec3 },
            Delete { index: u64, tree: TreeEditData },
        }
        use crate::resources::tree_storage::TreeStorageIndex;
        Ok(
            match <Saved as serde::Deserialize<'de>>::deserialize(deserializer)? {
                Saved::Place { index, mut tree } => {
                    tree.storage_index = Some(TreeStorageIndex(index));
                    Self::Place(tree)
                }
                Saved::Move { index, from, to } => {
                    Self::Move(TreeEditData::for_move(TreeStorageIndex(index), from, to))
                }
                Saved::Delete { index, mut tree } => {
                    tree.storage_index = Some(TreeStorageIndex(index));
                    Self::Delete(tree)
                }
            },
        )
    }
}

/// B `missing_prefab`（0x1409e6210）：返回 24 字节字符串常量
/// `"MISSING_TREE_PREFAB_NAME"`（机器码三 qword 字节序拼装：
/// `0x5f474e495353494d`= "MISSING_"、`0x4552505f45455254`= "TREE_PRE"、
/// `0x454d414e5f424146`= "FAB_NAME"；len/cap=0x18）。
pub fn missing_prefab() -> String {
    "MISSING_TREE_PREFAB_NAME".to_string()
}

// ---------------------------------------------------------------- 组 A 接线

/// 树 canopy 最大剔除距离（O：`tree_edit::TreeCanopyMaxCullDistance` 载荷
/// `(pub f32)`；render 域 `draw_callbacks_b` 记为无 Resource 的 f32 载荷——本处
/// 按 BASELINE 参数面 `ResMut<TreeCanopyMaxCullDistance>`（forest::tree_animation
/// 0x142279500）落地为 Resource，差异登记（render 侧视图待同步）。
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
pub struct TreeCanopyMaxCullDistance(pub f32);

impl Default for TreeCanopyMaxCullDistance {
    fn default() -> Self {
        Self(0.0)
    }
}

/// 树编辑命令事件族（O，BASELINE 参数面；`system_tree`/`forest` 消费）。
/// 载荷语义 U（0x58B 树命令记录族见 re-analyst-A 契约 §2），按类型合同登记。

/// 放置树命令（O：`tree_edit::InsertTreeCmd`；ui_place_tree 0x142269170 发射、
/// process_tree_cmds 0x142267430 消费。载荷 = 树 id + 位置 + 缩放 + prefab 名
/// （0x18B/0x48B 记录族，组 A 修复按真实载荷落地）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct InsertTreeCmd {
    /// Unique TreeStorage index, separate from placement variant.
    pub id: u64,
    /// Original Insert payload+0x28 (rotation), +0x2c (variant), +0x30 (bool).
    /// Native EventInstance+0x40 is EventId, not an additional height field.
    pub rotation_degrees: f32,
    pub variant: i32,
    pub is_preplaced: bool,
    /// 世界位置。
    pub position: glam::Vec3,
    /// 缩放（O：lerp(0.4922, 1.0) 族）。
    pub scale: f32,
    /// prefab 名（O：random_tree_prefab 分派）。
    pub prefab_name: std::string::String,
}

/// 删除树命令（O：`tree_edit::DeleteTreeCmd`；ui_delete_tree 0x141bd2510 /
/// hide_trees / convert_preplaced_trees 发射、process_tree_cmds 消费。
/// 载荷 = 真实树 id——组 A 修复：删除链按 TreeStorage 遍历/索引，禁止假 id）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct DeleteTreeCmd {
    /// 树 id（O：TreeStorage 索引）。
    pub id: u64,
}

/// 预置树批量生成命令（O：`tree_edit::SpawnPreplacedTrees`；convert_preplaced_trees
/// 0x14226be70 消费）。
/// Native load_game14146ea49..eb4e writes an 8B EventInstance (only EventId).
/// The legacy save compatibility route sends this same unit event.
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct SpawnPreplacedTrees;

/// 树回放命令（O：`tree_edit::TreeReplayCmd`；`ReplayCommand::Tree` 载荷，
/// `system_tree::replay_tree` 0x141bc34f0 消费。命令记录 = 0x58B
/// {1, 1, 变体, 树 id, …} 族，re-analyst-A 契约 §2；变体 0=放置/1=移动/2=删除）。
#[derive(Clone, Debug, Default)]
pub struct TreeReplayCmd {
    /// 回放编辑数据。
    pub edit: TreeEdit,
    /// 撤销方向。
    pub undo: bool,
    /// 即时模式（时间动画跳过）。
    pub instant: bool,
}

impl TreeReplayCmd {
    /// 桶展开（O 面：变体 0/1/2 → 命令分派由 replay_tree 执行）。
    pub fn expanded(edit: &TreeEdit, undo: bool, instant: bool) -> Self {
        Self {
            edit: edit.clone(),
            undo,
            instant,
        }
    }

    /// `make_instant`（O：ReplayBuffer::make_animations_instant 遍历面）。
    pub fn make_instant(&mut self) {
        self.instant = true;
    }
}

impl TreeEdit {
    /// 编辑种类（O：变体 0=Place / 1=Move / 2=Delete）。
    pub fn kind(&self) -> TreeEditKind {
        match self {
            Self::Place(_) => TreeEditKind::Place,
            Self::Move(_) => TreeEditKind::Move,
            Self::Delete(_) => TreeEditKind::Delete,
        }
    }

    /// Unique runtime TreeStorage identity. Original Place/Delete read Arc+24
    /// and Move reads Arc+48, separate from the nested placement variant i32.
    pub fn storage_index(&self) -> Option<crate::resources::tree_storage::TreeStorageIndex> {
        match self {
            Self::Place(data) | Self::Move(data) | Self::Delete(data) => data.storage_index,
        }
    }

    /// Existing consumer API; an unbound standalone TreeData cannot be replayed.
    /// Never widen placement.variant as a fallback, including negative IDs.
    pub fn id(&self) -> u64 {
        self.storage_index()
            .expect("TreeEdit runtime TreeStorageIndex is not bound")
            .0
    }
}

impl TreeEdit {
    /// 回放位置（I：TreeEditData 载荷面；`system_tree::replay_tree` 消费）。
    pub fn edit_position(&self) -> glam::Vec3 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.position,
        }
    }
    /// Placement variant used by actual InsertTreeCmd, independent of storage ID.
    pub fn placement_id(&self) -> i32 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.id,
        }
    }
    pub fn rotation_degrees(&self) -> f32 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.rotation_degrees,
        }
    }
    pub fn is_preplaced(&self) -> bool {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.is_preplaced,
        }
    }
    /// 回放缩放（I）。
    pub fn edit_scale(&self) -> f32 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.scale,
        }
    }
    /// 回放 prefab 名（I）。
    pub fn prefab_name(&self) -> &str {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => &d.prefab_name,
        }
    }
    /// 移动前位置（O：Move 编辑 live payload `+0x14/+0x1c`；非 Move 无旧位 →
    /// 返回当前位置）。`replay_tree` Move-undo 与 ui_move_tree History 使用。
    pub fn previous_position(&self) -> glam::Vec3 {
        match self {
            Self::Move(d) => d.previous_position,
            Self::Place(d) | Self::Delete(d) => d.position,
        }
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    use crate::resources::{placement_data::PlacementDataSsbo, tree_storage::TreeStorageIndex};
    #[test]
    fn placement_variant_and_storage_identity_survive_original_outer_schema() {
        let mut placement = PlacementDataSsbo::default();
        placement.position = Vec3::new(1.0, 2.0, 3.0);
        placement.scale = 0.75;
        placement.rotation_angle = 45.0;
        placement.id = -17;
        let tree = TreeEditData::from_placement(
            TreeStorageIndex(0x123456789abcdef0),
            &placement,
            "Oak".into(),
            true,
        );
        for edit in [
            TreeEdit::Place(tree.clone()),
            TreeEdit::Delete(tree.clone()),
        ] {
            let json = serde_json::to_string(&edit).unwrap();
            let kind = if edit.kind() == TreeEditKind::Place {
                "Place"
            } else {
                "Delete"
            };
            assert_eq!(
                json,
                format!(
                    r#"{{"{kind}":{{"index":1311768467463790320,"tree":{{"position":[1.0,2.0,3.0],"scale":0.75,"rotation_degrees":45.0,"id":-17,"prefab_name":"Oak","is_preplaced":true}}}}}}"#
                )
            );
            let decoded: TreeEdit = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded.id(), 0x123456789abcdef0);
            match decoded {
                TreeEdit::Place(d) | TreeEdit::Delete(d) => assert_eq!(d.id, -17),
                _ => unreachable!(),
            }
        }
        let data_json = serde_json::to_value(&tree).unwrap();
        assert_eq!(data_json.as_object().unwrap().len(), 6);
        assert!(
            serde_json::from_value::<TreeEditData>(data_json)
                .unwrap()
                .storage_index
                .is_none()
        );
    }
    #[test]
    fn move_original_schema_has_index_from_to_only() {
        let edit = TreeEdit::Move(TreeEditData::for_move(
            TreeStorageIndex(u64::MAX),
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
        ));
        let json = serde_json::to_string(&edit).unwrap();
        assert_eq!(
            json,
            r#"{"Move":{"index":18446744073709551615,"from":[1.0,2.0,3.0],"to":[4.0,5.0,6.0]}}"#
        );
        let decoded: TreeEdit = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id(), u64::MAX);
        assert_eq!(decoded.previous_position(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(decoded.edit_position(), Vec3::new(4.0, 5.0, 6.0));
    }
    #[test]
    fn unbound_tree_data_is_never_serialized_as_unique_tree_zero() {
        let edit = TreeEdit::Delete(TreeEditData {
            id: -1,
            ..Default::default()
        });
        assert!(
            serde_json::to_string(&edit)
                .unwrap_err()
                .to_string()
                .contains("no bound TreeStorageIndex")
        );
        assert!(serde_json::from_str::<TreeEdit>(r#"{"Delete":{"tree":{"position":[0,0,0],"scale":1,"rotation_degrees":0,"id":12,"prefab_name":"Oak","is_preplaced":false}}}"#).is_err());
    }
}
