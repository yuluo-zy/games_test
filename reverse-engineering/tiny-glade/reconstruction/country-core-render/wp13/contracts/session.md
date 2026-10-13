# WP13 — `resources::session` 业务逻辑合同

B 版本。`crates/country-core/src/resources/session.rs`（767 PDB 元记录 / 70 函数）。
恢复 `ActiveSession` / `GladeType` / `NextSession` / `SessionId` / `SessionMeta` /
`SavedSession(List)` 会话状态。实现见 `src/resources/session/`。

## O — 直接观察

### GladeType（enum，判别字节 @+0 位）
原机器码逐字节证实（`cmp byte[rcx], imm; sete al; ret`）：

| 方法 | 地址 | 机器码判定 |
|---|---|---|
| `is_olden` | 0x140a9fa70 | `80 39 01 0f94c0` → `tag == 1` |
| `is_flowery` | 0x14092e6a0 | `80 39 02 …` → `tag == 2` |
| `is_autumn` | 0x14092e6c0 | `80 39 03 …` → `tag == 3` |
| `is_winter` | 0x140ae8320 | `80 39 04 …` → `tag == 4` |

同一地址因 identical code folding 还绑定 `DragState::is_started` 等（共享后端注释
不同名，机器码一致）。还有 `assets_folder` / `audio_id` / `theme_id`（地址见
public-symbols；语义 U）。serde serialize 0x1403b5500 / deserialize 0x1403b55a0。

### ActiveSession（Resource）
- `is_winter` / `is_olden` / `is_autumn` / `is_flowery` 委托给 GladeType。
- 会话字节 @+0x58 == 4 选冬季水面（O：`startup_water` 0x1408f1a20、
  `water_planes_draw` 0x1409d9bc0 `cVar1==4` 选雪面；WP7）。
- `assets_root`（PDB 名存在）、`new`。serde serialize 0x1403b5920。
- Bevy 类型 id `0x74430dfcc6d8719b`（I：WP7 推断该 type-id 归属本资源）。

### NextSession（Resource，0xa0 = 160 字节，O：ignore_snapshot `memcpy 0xa0`）
- `SessionId`（16B uuid）@+0x20..+0x30。
- `NextSessionKind` 枚举字段 @+0x18（load 写 `[3]=&0x8000000000000002`）。
- 待应用命令 Vec 在结构开头（append_on_load push `(3, MetaCommandBlock)`）。
- `ignore_snapshot` 0x140b46a20：写 bool @+0x98，整结构复制 0xa0。
- `create` 0x140b46960：从 12+ qword 参数构造（session 参数/模板）。
- `load` 0x140b46920：`(session_id, 空Vec, kind=..)`。
- `load_latest` / `append_on_load<[MetaCommand;3]>` 0x1403b5150 /
  `append_on_load<MetaCommandFence>`（PDB 名）。

### SessionId
- `new` 0x140916420：`uuid::Uuid::new_v4()`（O，PDB 名 `new`）。
- `save_folder_path` 0x140b46000：`paths::saves().join(uuid)`（O）。
- `backup_folder_path` 0x140b46180、`snapshots_folder_path`、`leaf_folder_name`、
  `session_file_list` 0x140b46300（O：路径拼接/目录枚举）。

### SessionMeta
- `write_to_disk` 0x140b463c0：`save_folder_path.join(meta).save_session_meta_to_file`。
- `try_load_from_path` 0x140b46480：`std::fs::read_to_string` + anyhow context。
- `assets_root` 0x140b46370：`Path::join`。
- `is_halloween2024` 0x140b46860 / `is_xmas2024` 0x140b468c0 /
  `is_halloween_any` 0x140b46890：对字节字段（ptr@+0x8, len@+0x10）扫描特定字节
  （0 / 2 / >1），逐字符；语义 U（I：季节/主题标签编码）。

### 会话流
- `SavedSession`：`screenshot_path` / `loading_background_path` 0x140b46a50；
  `load_saved_session_list` 按 `SystemTime` 排序（O：sort_by_key closure）。
- `SavedSessionList`：`refresh_session_list` 0x140b46b00 清空后
  `paths::saves()` readdir 重建。
- `delete_session` 0x140b46d60：校验待处理工作后移入 `paths::deleted_saves()`
  （两段删除）。
- 事件：`NewSessionStartedCmd` / `SessionLoadedCmd` / `SessionOnLoadCmd`(enum) /
  `SessionMutator`(serde Vec)。Plugin `SessionFlowPlugin`。

## I — 推断

- GladeType 判别值域 {0?,1,2,3,4}：0 未证实（默认主题）；1..4 已证。
- `NextSessionKind` 变体 = 新会话/已有会话/起始建筑等（未从运行时枚举值确认）。
- `SessionMutator` 为会话变更命令的 serde 载体。

## U — 未知（typed gap）

- `ActiveSession` / `SessionMeta` / `SavedSession` 精确字段布局与完整 serde 字段序。
- `SessionMeta` 季节谓词的字节字段语义。
- `NextSessionKind` 变体集合；`SessionOnLoadCmd` 变体。
- `paths::saves()` / `deleted_saves()` 的精确路径规则。
