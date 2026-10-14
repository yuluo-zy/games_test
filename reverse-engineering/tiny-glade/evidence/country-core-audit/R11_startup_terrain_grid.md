# R11：startup 包的已闭合部分（`grid_gen_cw` / `grid_gen_ccw`）

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/startup/{mod,startup_terrain}.rs`。

## EXE 核对
`grid_gen_cw`（0x140928100，281 行反编译）与 `grid_gen_ccw`（0x140928350，328 行）逐行对照落码：
* 参数 `(Vec<u32>, col_start, col_end, rows, cells_per_row)`；顶点按行排布，行距 `S = cells_per_row + 1`。
* CW 每 quad 推 `(p, q, p+1)`、`(p+1, q, s)`；CCW 推 `(p, p+1, q)`、`(q, p+1, s)`（`p = r·S+k`、`q = (r+1)·S+k`、`s = q+1`，都加列起点）。
* 外层分块：`(end + 1) - head > 5` 时递归处理 4 列、`start` 前移 4，余下在本层。输出顺序因此是"4 列一块、块内逐行逐列"，三角形集合与整行实现一致。
* `start > end` → `Option::unwrap` 失败（0x1428d9310，位置 `startup_terrain.rs:290:27`）。0x1428d9310 的身份取自本会话 R7 里同一个单参数 panic 的用法（`rewire_enclosed_curve` 的 `first().unwrap()`），I。

## 没做的（startup 其余 60 个函数，约 158 KB）
`load_prefabs`（含 `reload_prefabs` 910 B、`PrefabWatcher`）、`startup_terrain` 本体（载入 8 个网格 + 6 个着色器 + 生成实体）、`startup_nani::reload_nani`、`startup_walls`、`startup_roof`、`startup_ivy`、`startup_grass`、`startup_garden_brush`、`load_glade_*`、`load_audio`、`load_shaders`、`load_textures`、`startup_session`、`glade_themes::flowery`、`clearing`…

它们都是 ECS 系统或资产库调用链：依赖尚未建立的组件层与 `Commands`/`Assets`，也依赖 `load_prefabs` 之外的多个 DIGEST U 项。**第 5 项没有完成**；这一个包只收了能独立核对的纯函数。

测试 +5。
