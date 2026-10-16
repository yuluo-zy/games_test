# Round7 链1 字段级 DELIVERY（2026-10-16，真实发行样本实证）

> 实现对象：① r7-chain1-field-CONTRACT.md（**手段 A：真实快照/存档样本反推 serde**）。
> 文件权内（systems/save/snapshot.rs）。**未碰 rhapsody/roof/wall 禁区**（decorator_storage/
> inner_wall_state 仅此前已落标注，无新增修改）。country-core 自身零错误。

## 真实代码接线（真实 serde schema 落地）

1. **Terrain 节 JSON 明文 serde**（snapshot.rs，r7-chain1 §3 纠偏）：
   `TerrainSectionJson {next_stroke_index, strokes: {id 串: {falloff, height_ws, index,
   points}}}` → `TerrainStrokeJson` → `into_editing_data()` 转 crate TerrainEditingData
   （tg_terrain：id 串 → StrokeId，next_stroke_index → next_index）。**strokes 为 JSON 明文
   （非 deflate；flate2 只用于二进制栅格尾）**。apply Terrain 分支改用 JSON serde。
2. **Walls 节真实 serde 结构**（snapshot.rs，r7-chain1 §2）：`WallStateJson`（8 字段：
   arch_height / arch_ty / curve{length,points,points_u} / elevation / has_skirts / height /
   is_enclosed / structure_ty）+ `parse_wall_section` 按 `Vec<(u64, WallStateJson)>` 反序列化
   （输入模型纠偏——此前 Vec<[u64;15]> 候选是错的）。**WallStateJson → 0x70B 打包（曲线区
   偏移）待①**（round6 r6-walls-0x78b 仅 +0x54/+0x60/+0x64/+0x67 O），登记不构造。
3. **5 域 apply 序**（snapshot.rs，r7-chain1 §6）：apply_loaded_snapshot 改为按
   Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster
   序分派（validate 0x1417a6e50 按名分派）。
4. **EditType serde 结构登记**（r7-chain1 §7b 终验）：真实存档证 Terrain=tag0（round4 裁定
   + 现源码已 tag0）；Path(tag1)=`{brush_size, is_additive, points}`（crate RasterStroke
   wire 形态待①）；WallNewSystem(tag7) CreateWall 子结构；tg_terrain `TerrainEdit` 变体
   （NewTerrainStroke/EditTerrainStroke）已匹配。crate EditType serde derive 待接线。

## candidate 链测试

- 本组文件 lib 类型检查零错误；country-core 自身零错误（排除 rhapsody）。
- TerrainSectionJson/WallStateJson serde 编译验证（真实样本字段名）。

## 原侧验证

- ① r7-chain1-field：真实快照
  `assets/starting-builds/demo/00-demo2/snapshots/11519500871291546481.snapshot`（369KB）
  + 真实存档 `history.json`（27KB）反推（serde schema + EditType 终验）。

## 仍缺项（待①/待接线，登记不造）

1. WallStateJson → 0x70B 打包（曲线区偏移；load_state 真写回需此）。
2. Decorators 0x28B/0x16B 逐槽 + JSON serde 逐字段名（真实样本 states 空，非空形态待证）。
3. GameSaveData 240B 二进制布局（JSON 面已闭：Camera 6 字段/Sheep/Time of day 3 字段已证）。
4. EditType serde derive（RasterStroke Path wire 形态 {brush_size,is_additive,points} 待①）。
5. 5 域 Roofs/WallColors/GateColors/Trees/Raster 目标资源 + Raster 二进制尾缓冲接线。
6. 跨轮待①：sheep movement+X4、ui_clutter 0x58B、ivy count_range、mod_key 表2、
   camera_rig/weather/time_of_day 深层、28-tuple 槽位 + Schedule.run。

## 组外申请记录

- 无本轮组外修改。未碰 rhapsody/roof/wall 禁区。
