# Round7 CONTRACT — 链1 字段级：真实快照 serde schema（手段 A 实证）

> 逆向 agent ① 交付。**新证据来源**：发行树真实快照样本
> `D:/game/ljxsj_92385/Tiny Glade/assets/starting-builds/demo/00-demo2/snapshots/11519500871291546481.snapshot`
> （369205B，369 KB）。**直接反推真实 JSON serde schema**。
> 状态：**Walls/Decorators/Terrain/Raster 节 JSON 字段名+形状定案可直接落地（serde 结构）**。

---

## 1. 快照文件格式（O，真实样本首字节）

```
"Snapshot" magic (8B) + u32 JSON 长度 (0x18d0=6352) + 8B 填充 + JSON 主体 + 二进制栅格尾
```
- JSON 主体 = serde_json 对象，节键：`Decorators, GateColors, Raster, Roofs, Terrain, Trees,
  Version, WallColors, Walls`（9 键）。
- 二进制尾（362837B）= **栅格缓冲**（首字节 `0x0a`=10 长度 + "PathRaster" + 数据；各命名栅格
  逐一落盘）。JSON `Raster` 节列出在场栅格名，实际缓冲在二进制尾。

## 2. Walls 节 JSON serde（O，真实条目全字段）

```
"Walls": [[28, {"arch_height": 2.8, "arch_ty": "FollowTerrain", "curve": {
    "length": 3.6879, "points": [[x,y,z],...], "points_u": [0.0,...]},
    "elevation": 0.0, "has_skirts": false, "height": 1.5993,
    "is_enclosed": "No", "structure_ty": "Wall"}], [27, {...}], [29, {...}]]
```
- 每元素 = `[WallId, WallState 对象]`（8 字段）：
  `arch_height` f32 / `arch_ty` 枚举串（FollowTerrain 等）/ `curve`{length f32, points
  Vec<[f32;3]>, points_u Vec<f32>} / `elevation` f32 / `has_skirts` bool / `height` f32 /
  `is_enclosed` 枚举串（"No"） / `structure_ty` 枚举串（"Wall"）。
- **0x78B 记录对应**：`{WallId u64@0, WallState 0x70B@8}`；WallState 二进制布局 = 上 8 字段
  （curve 点列内联；+0x54 height f32、+0x60 freehand、+0x64 arch、+0x67 removed_supports 与
  round6 r6-walls-0x78b 已证偏移相容）。
- **② serde 落地**：`WallState` serde 结构按上 8 字段（现 crate WallState{curve,height,style,...}
  是简化——补 arch_height/arch_ty/elevation/has_skirts/is_enclosed/structure_ty + curve.points_u）。

## 3. Terrain 节 JSON serde（O，strokes 在 JSON 内非二进制）

```
"Terrain": {"next_stroke_index": 3, "strokes": {"0": {"falloff": 0.836, "height_ws": 4.609,
    "index": 0, "points": [[8.22,0.526,-2.66],...]}, "2": {...}}}
```
- **strokes = 以 id 字符串为键的 map**；每 stroke = `{falloff f32, height_ws f32, index int,
  points Vec<[f32;3]>}`。
- **⚠ 纠偏**：Terrain strokes 在真实样本中是 **JSON 明文**（非 flate2 压缩）！flate2 deflate
  （"decompressing terrain data" @0x142cdc800）用于**二进制尾栅格/高度数据**，非 strokes。
  → ② 的 `inflate_terrain` 应按"二进制尾"而非 JSON Terrain 节接。
- **② serde 落地**：`TerrainEditingData{strokes: HashMap<String, TerrainStroke{falloff,
  height_ws, index, points}>, next_stroke_index}`（strokes map 键=索引字符串）。

## 4. Decorators 节 JSON serde（O）

```
"Decorators": {"states": [["Terrain", {"state": []}]]}
```
- states = **pair 列表**：`[地址判别串（"Terrain"/"Wall"/"Stair"/id）, {state: [...]}]`。
- 每 {state} 内容 = 该地址的装饰条目（真实样本空；非空形态随 capture 内容，字段名待证）。

## 5. Raster / 其他节（O）

- `"Raster": ["PathRaster","WaterRaster","GardenRaster"]` = 在场栅格名表；缓冲在二进制尾。
- `Version: 2`（int）；`Roofs/WallColors/GateColors/Trees: []`（空数组）。

## 6. 5 域写入序（O，真实样本 + round5 c-apply-targets 节序）

- 快照 JSON 节键序（writer 序）：Decorators, GateColors, Raster, Roofs, Terrain, Trees,
  Version, WallColors, Walls。
- 解析序（round5 §1 validate 0x1417a6e50）：Version → Walls → Roofs → Decorators → WallColors
  → GateColors → Terrain → Trees → Raster（按名分派）。
- **② apply 序**：Version 门 → Walls → Roofs → Decorators → WallColors → GateColors → Terrain
  → Trees → Raster（各节反序列化 → 二进制记录 → load_state/restore/写入）。

## 7. GameSaveData 240B — 真实存档节 serde（O，同 starting-build history.json，27739B）

存档 = serde_json 对象，节键：`Camera, History, Sheep, Time of day, Version`（starting-build 精简
集；完整存档加 Identity/Colors/chain_index/history_start/snapshot，B 契约节序）：
- **Camera**：`{camera_rig_arm_length, pitch_cache, pitch_offset, position_cache:[3], yaw_cache,
  yaw_offset}` = CameraRigParametersCache 6 字段（crate 已对齐）。
- **History**：`{chain_index, edit_chain:[{edits:[{EditType…}]}], snapshots:[{chain_index,
  snapshot_id}]}`。
- **Sheep**：`{sheep:[{goal:[3], move_state:{Walking:{force_reach_goal, goal_change_timeout,
  timeout_s}}, position:[3]}]}` = SheepControlSnapshot wire（crate 已对齐）。
- **Time of day**：`{latitude_degrees, sun_xz_rotation, time_of_day}` 3 字段。
- **Version**：17（int）。

## 7b. History edit_chain EditType serde（O，真实存档权威；round4 EditType 表终验）

edit 变体（真实数据）：**WallNewSystem**(13) / **Path**(2) / **Terrain**(5)。
- **WallNewSystem**（tag 7）：`[{CreateWall:{after:{wall state 8 字段}, color_id, elevation,
  wall_height, wall_index}}]`（含 roof/其他子变体族）。
- **Path**（tag 1）：`{brush_size f32, is_additive bool, points Vec<[f32;3]>}` = RasterStroke serde。
- **Terrain**（tag 0，**round4 visit_str "Terrain"→0 终验**）：`{NewTerrainStroke:{height_ws,
  index, stroke_pts}, EditTerrainStroke:{index, new_falloff, new_height_ws, prev_falloff,
  prev_height_ws}}` = TerrainEdit serde。

→ **② serde 落地**：EditType serde = 上述变体（现 crate Terrain 变体/Path 载荷对齐；
WallNewSystem 内 CreateWall 子结构按真实样本）。**EditType::Terrain=tag0 / Tree=tag12 由真实
存档数据证实**（round4 裁定 + 现源码待仲裁改）。

## 8. 可直接落地汇总
- Walls/Decorators/Terrain/Raster 节 JSON serde 结构（真实快照权威）。
- 存档节（Camera/History/Sheep/Time of day/Version）+ EditType serde（真实存档权威）。
- 5 域节序 + apply 序。
- Terrain strokes 纠偏（JSON 明文，非 deflate）。

## 9. 证据索引
- 真实快照：`assets/starting-builds/demo/00-demo2/snapshots/11519500871291546481.snapshot`（369KB）。
- 真实存档：`assets/starting-builds/demo/00-demo2/history.json`（27KB）。
- round5 c-apply-targets §1（节序）、round6 r6-walls-0x78b/r6-decorators-serde（0x78B/0x28B 记录）。
