# systems/wall/snapping/snapping_signifiers.rs

> 文件作用：吸附预览线系统：mode 2 画端吸附预览、mode 3 走 PublicWalls + WallSpatialHash 画段吸附预览线（DebugLine/DebugSphere + preview_snapping_line）。PDB 中归入 `snapping_maintain.rs`（见 snapping_maintain.rs.md 表格）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers` | 0x140a6a650 | 40 | snapping_signifiers（段/端双查询预览线） | O | wall\decomp\decomp-0x140a6a650.txt |

## 实现方式（算法级）

### snapping_signifiers 0x140a6a650（O，结构级；signifier 流布局 U）

签名（6 参）：param_1 = &SnappingMode（pbVar6），param_2/param_3 = 两个 UiSignifierStream 相关资源（+0xf8 索引数组、+0x100 计数×4、+0x108 标志、+0x118 游标；两个流分支用 0xa0B / 0x48B 元素表），param_4 = &mut Query\<(&SnappingToEndPreview, &mut Transform), Without\<SnappingToSegmentPreview>>（串 0x142ad4dd5 证实）+ 另一 Query\<(&SnappingToSegmentPreview, &mut Transform), Without\<SnappingToEndPreview>>，param_5/param_6 = 资源。

处理链：
1. `mode <= 1` → return。
2. **mode==2（端吸附）**：构造预览记录写入 signifier 流：`{u64@+0x10 = xvy(pos@+8, height@+4)（func_0x000140c9cde0 = utils::swizzlers::xvy 27B）`, `u32@+0x18`, `u64@+0x1c = {1.5f, 1.5f}（_UNK_142ad3650 @0x142ad3650）`, `u32@+0x24 = 1.5f`}。即以吸附点为中心 1.5×1.5 的端预览。
3. **mode==3（段吸附）**：以 `SnappingMode.wall_id@+0x10` 在 PublicWalls（步长 0x2a8）hashbrown 探针；命中校验 `count>=2 && length>0`（public_walls.rs:50 断言，UNK_142ad5250）→ 失败 panic + `swi(3)`。→ `preview_snapping_line`（0x1408dfc60 = shape_placement_signifiers::preview_snapping_line，构造段预览曲线）。返回 `i64::MIN`（None 哨兵）→ return。
4. 预览数据栈帧含 `uStack_d0 = 0x8000000000000000`（None 哨兵）、`0x40000000 = 2.0f`、`0x3e4ccccd = 0.2f`、`0x3f800000 = 1.0f`；随后调 `DebugLine::show`（0x140b04f70）与 `DebugSphere::show`（0x140b02520）绘制。
5. 前置的 stream 遍历（param_2/param_3，两条分支 bVar27==0/1）为预览实体回收/迭代（0xa0B 与 0x48B 元素，I）。

panic/断言：
- `called \`Result::unwrap()\` on an \`Err\` value`（0x142ad4ad0）@ snapping_maintain.rs:43（FUN_1428d9760，行 0x2b）。
- `assertion failed: w.simplified_curve_2d.is_valid()`（UNK_142ada193）@ public_walls.rs:50（行 0x32，UNK_142ad5250）。

跨模块调用：WallSpatialHash 查询（FUN_140b04f70/FUN_140b02520 = DebugLine/DebugSphere::show）、preview_snapping_line（shape_placement 域）、swizzlers::xvy、AppendOnlyVec 流（经 xvy 间接）。

### 遗留 U
- signifier 流资源 +0xf8/+0x100/+0x108/+0x118 的精确结构（两条 0xa0B/0x48B 分支含义）。
- mode 3 预览的 0.2f/2.0f/1.0f 常量语义。
