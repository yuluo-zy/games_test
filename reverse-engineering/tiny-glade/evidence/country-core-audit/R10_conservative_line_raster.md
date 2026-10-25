# R10：conservative-line-raster 与 `DirtyTiles::rasterize_world_space_stroke`

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/utils/{conservative_line_raster,dirty_tiles}.rs`。

## 来源
`conservative_line_raster` 是原版自有 crate（`crates/conservative-line-raster/src/lib.rs`，PDB 的 `impl$0..$3`、`conservative_plot_line_low`、两个 `quantize_range`、两个 `assert_delta`；`skeleton/B` 里只有元数据）。工作区没有为它单开 crate，作为 `country_core::utils::conservative_line_raster` 模块落地。

## EXE 核对
* `conservative_plot_line` 三个实例：0x140b1e460（DirtyTiles，3501 B）、0x142820fb0（draw_raster 用，3501 B）、0x1402582d0（楼梯，2774 B）。**0x140b1e460 与 0x142820fb0 逐字节对比，148 处差异全是 4 字节一组的 rel32，即同一份代码。**
* 算法完整读自 0x140b1e460 的 977 行汇编，步骤写在模块注释里：主轴选择（`|dx| > |dy|` 严格）、`slope` 非有限取 0、`c = r(1-|slope|)`、`h = hypotf(slope,1)·r`（0x142923cf0 = `_hypotf`，导入表实读）、首列/中间列/末列三种行区间（正负斜率各一套公式）、每个 `trunc` 等价 Rust 的 `as i32`（`cvttss2si` + 对 0x4effffff 的饱和 + NaN→0）、闭区间、`(x|y) as u32 <= 63` 才置位。
* `assert_delta`（0x140b84e50 / 0x140b84ec0）：前者 `x < 0 或 NaN` 则 panic，后者 `x > 0 或 NaN`；只在斜率符号与分支矛盾时触发，调用点的分支条件已排除，所以没复刻 panic 文本（0x142af6f30 起的格式串）。
* `rasterize_world_space_stroke`（0x140ade620 / 0x1425d9e90，同一函数体）：少于 2 点直接返回；`r_cells = radius·0x3efc0fc1 + 0x39c9a634`（0x142925874 / 0x142925878，`mulss`+`addss`，不是 fma）；每点 `(x, z)` 各 `fmaf(v, 0x3efc0fc1, 32.0)`；相邻点对交给 plot。R5 的 "+0x299" 注释里指的就是它。

## 偏差/不确定
* 其它两个实例的回调不同；`&mut bool` 提前停止标志只由外层检查，DirtyTiles 实例不碰它，所以 API 里没有。
* 没有 x86 模拟器可做差分测试（`unicorn`/`capstone` 都没有，venv 没带 pip）。测试只验证性质（水平/垂直/对角/反向对称/NaN 不死循环/裁剪），不是对原函数输出的逐格比对。**这是本轮验证最弱的一块。**
* 未做：`draw_raster` 状态机（0x142260ae0 等三个实例，672 行，含 SSBO 映射与事件）、`systems::world_raster` 的其它系统。`draw_raster` 里"清 +0x270 → 光栅化 → 两次 extend → 512 B 互换"这段，`ActiveRasterStroke` 已有互换，现在 `rasterize_world_space_stroke` 也有了，可以接线。

测试 +9（plot 7、dirty_tiles 2）。
