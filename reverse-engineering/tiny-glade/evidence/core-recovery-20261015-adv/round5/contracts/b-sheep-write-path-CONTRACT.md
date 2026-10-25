# Round5 B CONTRACT — sheep 写盘链（节组装 + 写盘序）

> 逆向 agent ① 交付。证据：procs.tsv（to_json_string / write_json_sections）+ 现源 save_game.rs
> 对照 + sync save 0x140a3f470（81KB）反编译头。
> 状态：**节序 + Sheep 节 producer 已定案（现源已对齐）；X4 活羊 Query 身份待取证（中优先）**。

---

## 1. Sheep 节原生 producer（O，procs.tsv 权威）

`country_core::systems::save::save_game::impl$2::to_json_string<Option<SheepControlSnapshot>>`
@ **0x140a51d40**。即原生 Sheep 节载荷 = `to_json_string::<Option<SheepControlSnapshot>>`
（None → null）。现源 `sheep_section_serialized` → `sheep::control::sheep_section_json`
（B 组接口）与之对齐。

## 2. 节写盘序（O，现源 SaveFileSection::ALL 10 节 + 原生节序一致）

| tag | 节 | 载荷 producer |
|---|---|---|
| 0 | Version | i32（load_game 版本门控 0x14362c308/0x14362c011） |
| 1 | Identity | `to_json_string<HistoryIdentity>` 0x140b385e0 |
| 2 | BackwardCompatibility | 0 |
| 3 | Camera | CameraRigParametersCache 6 字段 |
| 4 | TimeOfDay | SerializedJson（域 producer） |
| 5 | **Sheep** | **`to_json_string<Option<SheepControlSnapshot>>` 0x140a51d40** |
| 6 | Colors | GladeColorCustomization 15 字段 |
| 7 | ChainIndex | SerializedJson |
| 8 | HistoryStart | SerializedJson |
| 9 | Snapshots | SerializedJson |

- 现源 `assemble_game_save_sections` 按此序拼装 `GameSaveSections.values[10]`。
- 写盘 = `write_json_sections`（原生 0x140242b40 `<File>` / 0x141751c90 `<Cursor<Vec<u8>>>`）。
- 同步保存：`assemble_game_save_sections → AsyncSaveCmd::OpenFile`（现源 L206 由宿主事件响应触发）。

## 3. X4 Query 活羊捕获 — **待取证（中优先）**

- `capture_game_snapshot_save_data` 的第 4 个 Query（X4）= 活羊 → `SheepControlSnapshot` 捕获。
- 已解 3 个采集器（Round4 T3）：Tree=0x141305170、Wall=0x141308b40、u64-set=0x141309840。
  **Sheep 采集 Query 面未在 procs.tsv 直接现形**（可能为 `from_iter<(…, SheepControlSnapshot)>`
  的隐式闭包）——需解 sync save 0x140a3f470（81KB，round5/decomp-r5-0x140a3f470.txt）中
  sheep 捕获段（中优先）。
- 现源：`sheep_section_serialized` 接收调用方传入的 `Option<&SheepControlSnapshot>`——X4 捕获
  即其真实调用方；宿主/② 接 X4 Query 后闭链。

## 4. 可实现性
- **可直接落地**：节序（§2，现源已对齐）+ Sheep producer 接口（§1）+ write_json_sections。
- **需再取证**：X4 Query 活羊→SheepControlSnapshot 的精确 Query 面与序列化字段（中优先）；
  同步保存 OpenFile 触发点（现源 L206 宿主接线已登记）。

## 5. 证据索引
- procs.tsv：0x140a51d40（to_json_string<Option<SheepControlSnapshot>>）、0x140b385e0
  （to_json_string<HistoryIdentity>）、0x140242b40/0x141751c90（write_json_sections）。
- 现源：systems/save/save_game.rs（assemble_game_save_sections L408、sheep_section_serialized
  L396、write_json_sections L72）、resources/save_file.rs（SaveFileSection 10 节序）。
- round5/decomp-r5-0x140a3f470.txt（sync save 81KB，X4 段待拆）。
