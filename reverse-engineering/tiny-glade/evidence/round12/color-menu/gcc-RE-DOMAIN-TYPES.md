# RE-DOMAIN-TYPES — 阻塞 save/session 验收的 4 个域类型证据包

分析师: reanalyst-state
日期: 2026-10-15
目标: 解锁 GameSaveData 组装（save_game 0x140a3f470 读这些类型做 Colors/Camera/Identity 存档节）
与 load_completion_checker 完整系统。每类型给 ①原文件/inventory ②结构/字段/serde schema
③default ④真实消费者 ⑤地址/行为级结论。证据 = fresh 反编译（落盘 evidence/）+ RNE 原字节
+ 既有章节/DIGEST。

**workspace 现状**（实测 grep）：4 类型在 reconstruction **均未定义**——仅 save_game.rs /
save_file_identity.rs / save/mod.rs 注释引用。需 restorer 新建。

---

## 1. CameraRigParametersCache（0x24B=36B）

### ① 原文件与 inventory
- 原文件: `systems/camera_rig.rs`（camera-time 域，inventory `present_requires_semantic_review`）。
- 相关原函数: `store_parameters` 0x140911710、`load_parameters` 0x140911520、
  `impl$0::serialize<serde_json Compact>` 0x1403b4ca0（session.rs 侧记录序列化）、
  `impl$0::deserialize::visit_str` 0x1403b5ba0、`to_json_string<CameraRigParametersCache>`
  0x1416f8470（save_game.rs，**Camera 存档节载荷**）。

### ② 结构/字段/serde schema（O，to_json_string 0x1416f8470 fresh + RNE 原字节）
| 偏移 | 字段 | serde key | 类型 | 证据 |
|---|---|---|---|---|
| +0x00 | yaw | `"yaw"`(3) | f32 | 0x1416f8470 写 key@0x142c921de len3, param+0 |
| +0x04 | pitch | `"pitch"`(5) | f32 | key@0x142c921e1 len5, param+4 |
| +0x08 | position | `"position"`(8) | Vec3(12B) | key@0x142c91f4c len8, param+8（FUN_141583cf0 向量序列化） |
| +0x14 | camera_rig_arm_length | `"camera_rig_arm_length"`(21) | f32 | key@0x142c921e6 len0x15, param+0x14 |
| +0x1c | pitch_offset | `"pitch_offset"`(12) | f32 | key@0x142c921fb len0xc, param+0x1c |
| +0x20 | yaw_offset | `"yaw_offset"`(10) | f32 | key@0x142c92207 len10, param+0x20 |

- **位置串"position_cache"在 blob 0x1429cca3b 存在但 save 序列化 6 字段不含它**——
  camera-time DIGEST 的 7 字段表（含 position_cache）**修正**：存档 Camera 节 = 6 字段；
  position_cache 仅反序列化接受（0x1403b5ba0 visit_str）或为内部缓存字段。
- 序列化写 `'{'` 开头（手工 JSON），关闭 `'}'`（0x1416f8470 尾 FUN_1412f4520）。

### ③ default
- 由 `MainCamera::new`/`reset_camera` 构造默认；`store_parameters` NaN→0，position 默认
  `(0,5,0)`@0x142aab4c0，四条日志"…invalid. Overriding param cache value."。

### ④ 真实消费者
- **save_game 0x140a3f470 GameSaveData 组装**：Camera 存档节 = `to_json_string<CameraRigParametersCache>`
  0x1416f8470（json_key "Camera"，padding 128，SaveFileSection tag 3）。
- load_game（Camera 节解析）、reset_camera（清 Vec + WriteSettingsFileCmd）。

### ⑤ 结论
`pub struct CameraRigParametersCache { yaw: f32, pitch: f32, position: Vec3,
camera_rig_arm_length: f32, pitch_offset: f32, yaw_offset: f32 }`（Serialize/Deserialize；
6 字段；`#[serde(default)]` position 缺省 (0,5,0)）。

---

## 2. HistoryIdentity（u64 新类型）

### ① 原文件与 inventory
- 原文件: `resources/history/mod.rs`（history 域）。
- 原函数: `HistoryIdentity::generate` 0x140a84220（fastrand u64 全范围）、`mutate`（行 102 内联）、
  `impl$1::serialize`（行 120 内联）、`to_json_string<HistoryIdentity>` 0x140b385e0（save_game.rs）。

### ② 结构/serde schema（O，0x140b385e0 fresh）
- `pub struct HistoryIdentity(u64)`——to_json_string 0x140b385e0：`uVar7 = *param_2` 单 u64，
  十进制数字格式化（/10000 分组 + 0x142aedc18 双字节数字表 + `|0x30` 单字节）。
- **存档 Identity 节载荷 = 裸 u64 十进制数字**（非串、非对象）。

### ③ default
- `generate` = fastrand u64 全范围；`mutate` = 重随机（内联）。无固定默认。

### ④ 真实消费者
- save_game Identity 节（json_key "Identity"，padding 16）。
- `save_file_identity.rs`：`try_approx_parse_save_file_identity` 0x141e58f20 搜字面量 `"Identity"`
  （含引号，@0x142f92a40 len10）后解析 JSON 数字为 SaveFileIdentity。

### ⑤ 结论
`#[derive(Serialize, Deserialize)] pub struct HistoryIdentity(u64)`；序列化为 u64 数字。

---

## 3. GladeColorCustomization（≥0x48B，15 序列化字段）

### ① 原文件与 inventory
- 原文件: `resources/glade_color_customization.rs`（audio-misc 域，全 serde 派生）。
- 原函数: `deserialize::visit_str` 0x14085f550、`serialize<value>` 0x1401746e0、
  `serialize<Compact>` 0x14085f980、`to_json_string<GladeColorCustomization>` 0x14085ee90（save_game.rs）。

### ② 结构/字段/serde schema（O，0x14085ee90 + 0x14085f550 fresh 全解）
**字段名/索引（visit_str 0x14085f550 → index）与 to_json_string 0x14085ee90 偏移**：

| index | 字段名 | 偏移 | 类型线索 | 证据 |
|---|---|---|---|---|
| 10 | `windows` | +0x00 | ColorId(u32) | 0x14085ee90 key@0x142a95773 len7, param+0 |
| 14 | `stairs` | +0x08 | ColorId(u32) | key@0x142a9579f len6, param+8 |
| 0 | `freeform_wall` | +0x18 | ColorId(u32) | key@0x142a9570f len13, param+0x18 |
| 1 | `building` | +0x1c | ColorId(u32) | key@0x142a9571c len8, param+0x1c |
| 2 | `roof` | +0x20 | ColorId(u32) | key@0x142a95724 len4, param+0x20 |
| 5 | `halftimber` | +0x24 | ColorId(u32) | key@0x142a95746 len10, param+0x24 |
| 6 | `lantern_light` | +0x28 | ColorId(u32) | key@0x142a95750 len13, param+0x28 |
| 7 | `fence` | +0x2c | ColorId(u32) | key@0x142a9575d len6, param+0x2c |
| 8 | `window_light` | +0x30 | ColorId(u32) | key@0x142a95763 len12, param+0x30 |
| 9 | `flag` | +0x34 | bool | key@0x142a9576f len4, param+0x34 |
| 11 | `gates` | +0x38 | ColorId(u32) | key@0x142a9577a len5, param+0x38 |
| 13 | `lantern_brightness` | +0x3c | f32/u32 | key@0x142a9578d len18, param+0x3c |
| 4 | `freehand_ty` | +0x40 | 字节枚举 | key@0x142a9573b len11, param+0x40 |
| 12 | `flat_rim_style` | +0x41 | 字节枚举 | key@0x142a9577f len14, param+0x41 |
| 3 | `building_wall_style` | +0x42 | 字节枚举(StairRailingType?) | key@0x142a95728 len19, param+0x42 |

- 字段名串（RNE 原字节 0x142a956f0 起，**连续打包无分隔符**）：
  `TimberedGladeColorCustomization`(struct 名) + `freeform_wall` + `building` + `roof` +
  `building_wall_style` + `freehand_ty` + `halftimber` + `lantern_light` + `fence` +
  `window_light` + `flag` + `windows` + `gates` + `flat_rim_style` + `lantern_brightness` + `stairs`。
- visit_str 0x14085f550 长度分派（case 4/5/6/7/8/0xb/0xc/0xd/0xe/0x12/0x13）逐名比对 → index
  0..0xe（0xf=unknown）。**字段名修正**：旧章节未列字段名（I 级）→ 本证据包给出 15 名全表。
- serde 类型（0x14085f980 serialize_entry 泛型）：多为 `ColorId`，含 1 个 `StairRailingType`
  （+0x42 building_wall_style 最可能）；+0x40/+0x41/+0x42 三字段 1 字节相邻 → 字节枚举。
- 序列化写 `'{'` 开头，关闭 `'}'`（0x14085ee90 尾 FUN_1412f4520）。

### ③ default
- 默认字节 O（audio-misc 域）；各 ColorId 字段默认 0（`default_value` 族）。

### ④ 真实消费者
- **save_game Colors 存档节** = `to_json_string<GladeColorCustomization>` 0x14085ee90
  （json_key "Colors"，padding 64，SaveFileSection tag 6）。
- load_game 解析 Colors 节（升级逻辑见 load_game 颜色 ID 映射）。

### ⑤ 结论
`pub struct GladeColorCustomization { windows: ColorId, stairs: ColorId, freeform_wall:
ColorId, building: ColorId, roof: ColorId, halftimber: ColorId, lantern_light: ColorId,
fence: ColorId, window_light: ColorId, flag: bool, gates: ColorId, lantern_brightness: f32,
freehand_ty: ?, flat_rim_style: ?, building_wall_style: StairRailingType }`
（字段序按 index 0..0xe；偏移表见上）。

---

## 4. GameProfile（0x208B=520B）

### ① 原文件与 inventory
- 原文件: `game_profile.rs`（顶层；inventory `no_formal_mapping`）。
- 原函数: `deserialize::visit_str<ron>` 0x1404b0ed0、`serialize<ron>` 0x140a658a0、
  `load_game_profile` 0x140916480、`save_game_profile` 0x140917230、
  `write_profile_via_tmp` 0x140917360、`profile_file_path` 0x140916440、
  `impl$1::default` 0x140916420（= Uuid::new_v4，与 SessionId::new 同址）。

### ② 结构/serde schema（O，fresh + RNE 原字节）
- **ron 序列化字段（serialize 0x140a658a0 顺序）**：
  `GameProfile`(struct 名) → `tutorial`(@+500) → `tutorial_cooldowns`(@+0x1e0) →
  `game_installation_id`(@+0x1c8) → `key_bindings`(@+0) → `new_glade_count`(@+0x1d8)。
- **deserialize 字段（visit_str 0x1404b0ed0 长度分派）**：
  `tutorial`(8)→0、`tutorial_cooldowns`(18)→1、`player_id`(9)→2、`key_bindings`(12)→3、
  `new_glade_count`(15)→4、`game_installation_id`(20)→5（**raw 字节 `mov byte[rax+1],X`
  于 0x1404b0f09/0xfeb/0xf82/0xf34/0xfa4 逐一确认 0,1,2,3,4；game_installation_id 的 Ghidra
  共享标签 index=2 为地址合并伪影，按声明序应为 5**）。
- **关键发现**：serialize 只写 **5 字段**（struct 头计数 5）——`player_id` **不序列化**，
  但 deserialize 接受（旧档兼容/内部身份）。
- 布局偏移（serialize 读字段）：key_bindings@+0x000、game_installation_id@+0x1c8(Uuid 16B)、
  new_glade_count@+0x1d8(u32)、tutorial_cooldowns@+0x1e0、tutorial@+0x1f4(bool)；总 0x208B。
- 字段名串（RNE 0x142ad4578 起）：
  `GameProfile` + `tutorial` + `tutorial_cooldowns` + `game_installation_id` + `key_bindings` +
  `new_glade_count` + `player_id`。

### ③ default
- `impl$1::default` 0x140916420 = Uuid::new_v4（生成 player_id）+ 默认键位（game_key_bindings
  impl$0::default 内联）；load_game_profile 缺失/失败回退默认 + `write_profile_via_tmp` + insert_resource。

### ④ 真实消费者
- **load_game 0x14146d030**：SystemParam 第 3 参 = `Res<GameProfile>`（新 glade → new_glade_count
  递增/读写，消费边界确认）。
- **load_completion_checker 澄清**：`check_load_completion` 0x140a9f810 参数 = (Res<AppMode>,
  EventReader<LoadingGameComplete>, Local<LoadingCompletionState>, Res<ActiveSession>)——
  **不含 GameProfile**；`is_session_ok_to_load` 0x140a9fa80 = (SessionId, &UnsavedFileList)——
  也不含 GameProfile。skeleton 中 load_completion_checker.rs 的 GameProfile 引用是 load_game
  tuple 的 PDB 注释串，非本文件参数。**GameProfile 真实消费方 = load_game / save_game / startup**。
- save_game：同步保存前读 GameProfile（新 glade 计数等）。
- startup_session / run_game：load_game_profile + save_game_profile。

### ⑤ 结论
`pub struct GameProfile { key_bindings: GameKeyBindings, game_installation_id: Uuid,
new_glade_count: u32, tutorial_cooldowns: TutorialCooldowns, tutorial: bool,
player_id: Uuid }`（ron；player_id `#[serde(skip_serializing)]`）。

---

## 5. 消费者接线小结（供 restorer 落地）

- **save_game GameSaveData 组装**用这 4 类型构造存档节：Identity(HistoryIdentity u64) /
  Camera(CameraRigParametersCache 6 字段) / Colors(GladeColorCustomization 15 字段)；
  GameProfile 供新 glade 计数。三者 serde schema 本报告已给全 → **GameSaveData 组装不再阻塞**。
- **load_completion_checker 完整系统**不依赖 GameProfile——需的是 UnsavedFileList(+0x118)/
  AppMode/LoadingCompletionState/ActiveSession（其 SystemParam 面已列）。
- **位置证据文件**：evidence/decomp-0x14085f980-fresh-shared.txt（serialize 派生的类型泛型）、
  evidence/decomp-0x14085f550-fresh-shared.txt（visit_str 15 字段名全表）。
