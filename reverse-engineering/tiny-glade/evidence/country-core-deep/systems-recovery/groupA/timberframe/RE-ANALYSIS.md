# groupA RE-ANALYSIS — timberframe（木框架系统）

dbid: tg_xa1 · 2026-10-16 · decomp: `groupA/timberframe/decomp/`

## 1. 覆盖盘点

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x140630a20 | impl$0::add_timberframe | O | 木框架插件注册（add_timberframe 名实为 Plugin::build 族，见下） |
| 0x1421b6610 | timberframe_beam_count_change_audio | O | 梁数变化→音频事件记录 |
| 0x1421b67a0 | construct_timberframe_circle | O | 圆顶木框架梁构建 |
| 0x141a41ba0 | construct_timberframe_rectangle | O | 矩形木框架梁构建 |
| — | timberframe_on_glade_unload (line 50) | U(内联) | 谷地卸载回调内联 |

## 2. 契约

**add_timberframe (0x140630a20)**（4KB）：bevy 注册壳（0x140de6810/62d0/6f80/80e0/8d10/9470/6e00/b400 = App 方法族）+ 1 个格式化 panic（0x1428d9390，串区 0x142b77b10/0x142b77138/0x142b77268 = rustc 路径）→ 内部注册 construct_timberframe_* 与 beam_count_change_audio。

**timberframe_beam_count_change_audio (0x1421b6610)**（4 参）：
- param_1 = Res（墙/形状空间哈希，0x28B 条目）；param_3 = Res（每梁计数资源：`*(int*)(*param_3+0x2c)` = 当前梁数）；param_4 = ResMut<Vec<0x20B 事件记录>>（tick 回写）。
- 变化检测：`(当前梁数 - 上次梁数 == 1) && 非空` → 触发。
- 事件记录 0x20B = {u64 打包位置（`ShapeParameters::center` 0x1411ff2a0 + unpack 0x140c9cdc0）, u32 `get_building_size_01`（0x1417d3f10，audio 域建筑尺寸归一化）, u8 kind=6, ...} push 到输出 Vec（grow 0x140562c30）。
- **语义**：梁数 +1 → 向音频系统发 {pos, size_01, kind} 事件（梁添加音效触发器）。

**construct_timberframe_circle (0x1421b67a0)**（28KB，主业务）：
- 输入：墙形状（ShapeParameters 域）+ 屋顶参数。
- 算法骨架：`Circle2d::perimeter`（0x140c97970）+ `as_points2_ptcount`（0x140c97ba0）×3 + `get_pos_at_u`（0x140c975a0）×3（u 采样梁位置）；`roof_height_ws_from_wall_shape`（0x1408e3c70）；`rotation_from_normal`（0x140c90a40）×2；`0x140b34f80`（PDB 双名：walls::HighlightColor::to_rgb / skeleton 标 split_height_into_skirts —— 机器码按"颜色/裙边 f32→字段"用，**归属冲突登记**）。
- 关键检查：`u ∈ [0,1]` 界外 → 格式化 panic `'assertion failed: !u.is_nan()'`（0x1431025d8，位置 fat 0x143102648 = utils/src/geometry/circle.rs —— 即 Circle2d::get_pos_at_u 的断言，调用点透传）。
- 常量：0.2f32 ×2（0x142a8c6f0 —— 梁间距系数）；-1.0/-0.5（0x14295a0e4）；0.4922/1.0（0x142925870/904）；log 域 0x140d4a330 ×1（info! 级，文案 U）。
- 输出：梁实例（位置/朝向/高度）写往 **WoodenPlankSsbo**（类型名字串 0x142ff8cf0 = 'country_core::resources::roofs::roof_buffers::WoodenPlankSsbo'）—— 木框架 plank GPU 缓冲属 roofs 域（roof_buffers）。

**construct_timberframe_rectangle (0x141a41ba0)**（44KB）：
- `Rectangle2d::as_curve2`（0x140c98cf0）×2 / `as_curve3`（0x140c991f0）×3 / `get_point`（0x140c99600）×2 / `orientation_mat3`（0x140c99570）×2 / `perimeter`（0x140c99530）；`RoofRidgeDims::calc`（0x1408e2700）、`ridge_length_01_from_rectangle_ratio`（0x1408e3b60）、`roof_height_ws_from_wall_shape`；`rotation_from_normal` ×4；同上 panic 族（u.is_nan）。
- 常量：0.2×2（间距）、0.4922（0x142925870 ×9 —— 缩放）、0.63/1.5（0x1429258e0 区）、1.0/-0.3（0x142925900）；`0x142b14ea0..14eac` = 实例范围标记 {start=0, count=2}（矩形四边成对）。
- 输出：矩形四边梁实例 → WoodenPlankSsbo（同 circle 版本）。

## 3. 关键小函数

- **0x1408e3c70 roof_height_ws_from_wall_shape** / **0x1408e2700 RoofRidgeDims::calc** / **0x1408e3b60 ridge_length_01_from_rectangle_ratio**：roofs 域（roof_shape/roof_ridge 章节化）—— 木框架梁高/屋脊依赖屋顶域公式。
- **0x140c97ba0/0x140c975a0/0x140c97970**：utils::geometry::circle（Circle2d 圆周长/采样）—— utils 域已章节化。
- **0x140c90a40 rotation_from_normal**：utils 域法向旋转矩阵。
- **0x140b34f80**：地址在 walls/mod.rs（HighlightColor::to_rgb 归属）—— timberframe 用它做梁颜色/裙边转换，PDB 双名冲突待墙域裁决。

## 4. panic / 串

| 地址 | 内容 |
|---|---|
| 0x1431025d8 | `assertion failed: !u.is_nan()`（+ 0x143102648 → /home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs） |
| 0x142ff8cf0 | `country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`（类型名串） |
| 0x143102588 | fat 位置对（timberframe_beam_count_change_audio.rs） |
| 0x142b77b10 族 | add_timberframe 的 rustc 路径（bevy 内部 panic） |

## 5. 消费者线索

- WoodenPlankSsbo —— roofs 域 render/roof_buffers 消费（GPU 后置）。
- get_building_size_01（0x1417d3f10）—— audio/building_audio 域（音效尺寸归一化）。
- ShapeParameters（墙域 valid_enclosure）—— 与 glade-border display 同源数据。

## 6. U 项

- 梁实例 0x20B 记录字段全表（kind=6 语义）。
- 0x140b34f80 的真实归属（HighlightColor::to_rgb vs split_height_into_skirts）。
- log 文案（0x140d4a330 调用点字符串）。
