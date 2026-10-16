# RE-EDITTYPE — EditType 与 WallHistoryEdit 逐 tag→变体名→载荷结构

分析师: reanalyst-state
日期: 2026-10-16
目标: 扩 src EditType 到完整变体。证据 = EditType deserialize visit_str **0x140b3b630**
（fresh 反编译，落盘 evidence/）+ WallHistoryEdit deserialize visit_str **0x1414022c0**
（fresh 反编译）+ 变体名串 blob（RNE 0x1429bb130 区）+ 两张 Debug fmt 跳表（0x142ad88d0/0x142ad888c，
RNE 原字节）+ MultiEdit deserialize 跳表 0x142aee31c。

**核心结论**：EditType 实际 **15 变体（tag 0..14）**，WallHistoryEdit **17 操作变体（tag 0..16）**。
「40 变体」是 60 项跳表的误读——跳表按 niche tag 空间 0..59 铺满，未用 tag 落到 default/panic 块，
**有效变体仅 15/17**。src 的 10 变体缺 5 个且 tag 有误。

---

## 1. EditType 逐 tag→变体名→载荷（O，visit_str 0x140b3b630）

| tag | 变体名 | serde 长度 | 载荷（章节） |
|---|---|---|---|
| 0 | **Tree** | 7 | TreeEdit{Place/Move + tree{id,position,scale,rotation_degrees,prefab_name,is_preplaced}}（tree_edit.rs，src 缺） |
| 1 | **Path** | 4 | RasterStroke Arc（path_edit.rs，src 缺 RasterEdit） |
| 2 | **Cobblestone1** | 12 | RasterStroke Arc |
| 3 | **Cobblestone2** | 12 | RasterStroke Arc |
| 4 | **Cobblestone3** | 12 | RasterStroke Arc |
| 5 | **Water** | 5 | RasterStroke Arc |
| 6 | **Garden** | 6 | RasterStroke Arc |
| 7 | **Wall**（alias "WallNew…" 13） | 4 | WallCompoundHistoryEdit 0x108B（WallHistoryEdit 操作枚举，见 §2） |
| 8 | **Roof** | 4 | RoofHistoryEdit（RoofEdit 操作枚举） |
| 9 | **Color** | 5 | ColorEdit（color_edit.rs，src 缺） |
| 10 | **DecoratorV5**（alias "Decorator5"） | 11/10 | DecoratorHistoryEdit（decorator_edit.rs，src 缺） |
| 11 | **Decorator**（alias "Decorator4"） | 9/10 | DecoratorHistoryEdit（decorator_edit.rs） |
| 12 | （4 字符名，anon_74 未解） | 4 | U（残留 §5） |
| 13 | **AutoClutter** | 11 | AutoClutterEdit{host wall_id, decorator_ids}（src ✓） |
| 14 | **Stairs** | 6 | StairsHistoryEdit/Diff（stairs_history_edit.rs，src 缺） |

**修正（对 src resources/history/mod.rs 的 EditType）**：
- **缺变体**：Tree(0)、Color(9)、DecoratorV5(10)、Decorator(11)、Stairs(14)、tag 12 未知。
- **tag 错位**：src `Terrain → 14` **应为 `Stairs → 14`**（visit_str case 6 anon_76 = "Stairs" → tag 0xe）；
  src 无 Terrain 变体（"TerrainPath" 串在 blob 0x1429bb26d 但非 EditType visit_str 命中名——
  属 RasterEdit 族或他枚举名，见 §5）。
- src native_tag 表：Path=1/Cobble1-3=2-4/Water=5/Garden=6/Wall=7/Roof=8/AutoClutter=13 全部与
  原版一致 ✓；**只差 Tree=0/Color=9/Decorator=10,11/Stairs=14**。

---

## 2. WallHistoryEdit 逐 tag→操作名→载荷（O，visit_str 0x1414022c0）

| tag | 操作名 | serde 长度 | 语义（章节 wall_compound_edit） |
|---|---|---|---|
| 0 | **WallChanged** | 11 | 墙状态变更 |
| 1 | **CreateWall** | 10 | 建墙 |
| 2 | **DeleteWall** | 10 | 删墙 |
| 3 | **AdjustHeight** | 12 | 调墙高 |
| 4 | **ChangeIsEnclosedState** | 21 | 封闭状态切换 |
| 5 | **MorphPremadeShape** | 17 | 预制形状形变 |
| 6 | **ChangeElevation** | 15 | 标高 |
| 7 | **ChangeArchHeight**（anon_83, 16 字符） | 16 | 拱高（残留在 blob 的 "ChangeArchHeight" 16 字，与 tag 12 ChangeArchType 区分） |
| 8 | **ChangeSupports** | 14 | 支撑变更（字段 before/after_removed_supports） |
| 9 | **ChangeStairsType** | 11 | 楼梯类型（阶梯变化） |
| 10 | **ChangePlasterPeelAmount** | 23 | 石膏剥落量 |
| 11 | **ChangeFreehandHeightType** | 24 | 手绘高度类型 |
| 12 | **ChangeArchType** | 14 | 拱类型 |
| 13 | **MorphFreehandWall** | 17 | 手绘墙形变 |
| 14 | **DeprecatedMorphOriginal**（15）+ 5 字符名（anon_100 未解） | 15/5 | 已弃用形变（panic_deprecated_operation 0x140a82e20） |
| 15 | （13 字符名，anon_95 未解） | 13 | U（残留 §5） |
| 16 | **ChangeSkirt** | 12 | 墙裙（字段 chain_index/snapshot_id/enclosure_params/is_valid） |

**判别空间**：首字段 niche（`x - 0x7fffffffffffffff` = 变体，fmt 0x140a82eb0 跳表 0x142ad888c
16 项互证）。**wall 子编辑判别与顶层 EditType 判别不共用**——Wall(EditType tag 7) 载荷是
0x108B WallCompoundHistoryEdit，内部含 WallHistoryEdit 变体（tag 0..16）。

---

## 3. 跳表互证（RNE 原字节）

| 表 | 地址 | 项数 | 结论 |
|---|---|---|---|
| EditType Debug::fmt | 0x142ad88d0 | 60（有效 15，未用 tag → default 块） | tag 0..59 niche 空间，有效变体 0..14 |
| WallHistoryEdit Debug::fmt | 0x142ad888c | ≥60（有效 17） | tag 0..16 有效 |
| MultiEdit deserialize | 0x142aee31c | 60 | EditType/HistoryEdit 混合分发（tag 0..59），与 visit_str 双表一致 |

→ 三表同构（60 项、niche 基址 0x8000000000000000）；「40 变体」为 60 项跳表项数误读。

---

## 4. 对 restorer 的落地清单

1. **EditType 扩到 15 变体**：加 Tree(0)/Color(9)/DecoratorV5(10)/Decorator(11)/Stairs(14)；
   **改 Terrain→Stairs**；native_tag 修正；新增类型 ColorEdit/DecoratorHistoryEdit/TreeEdit/
   StairsHistoryEdit 依 §1 载荷与章节实现。
2. **WallHistoryEdit 扩到 17 操作**：WallCompoundHistoryEdit 载荷内按 tag 0..16 分派（当前
   recovered_building/wall_history_edit.rs 是否覆盖需 restorer 对照 §2 表）。
3. **serde 对齐**：save 文件里 EditType 名即 §1 表（save_game_to_file MultiEdit::serialize 写
   这些名字；load_game MultiEdit deserialize 0x140b36460 按 visit_str 0x140b3b630 反序列化）。
4. **判别空间**：EditType niche 0x8000000000000000+tag；WallHistoryEdit niche 同基址+tag(0..16)。

---

## 5. 未决疑问

1. EditType tag 12 的 4 字符名（anon_74，0x140b3b630 内联串，需 lea 目标字节解串）。
2. WallHistoryEdit tag 14 的 5 字符名、tag 15 的 13 字符名（anon_100/anon_95）。
3. "TerrainPath" 串的归属（0x1429bb26d，非 EditType visit_str 命中——疑为 RasterEdit 族或
   TerrainPath 变体名，需 path_edit/terrain_edit 域交叉）。
4. DecoratorV5/Decorator 两变体的语义差异（版本化装饰编辑 vs 普通装饰编辑）。
