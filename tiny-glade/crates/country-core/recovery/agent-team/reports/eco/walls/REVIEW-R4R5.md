# REVIEW-R4R5 — valid_enclosure + snapping + eraser + shapes（reviewer-eco，2026-10-16）

复核对象：restorer-walls R4+R5（RESTORE-WALLS-R4R5.md）。实测：cargo check 过；cargo test
--lib **1709/1**（唯一败 geometry/sphere 他组，wall_spatial_hash 元组 Eq/Hash 编译阻断已修、
测试目标恢复）；valid_enclosure 5/5、snapping 8/8 过（模块 22 测试全过）。未改 src。

## 逐项核对（协调者 5 点）

### 1. valid_enclosure — ✅ 抽查命中，2 项待取证诚实

- `ShapeParameters` 28B{tag@0..4, payload@4..28} 手写 serde {tag,payload:[f32;6]}；`Shape{Circle,
  Rectangle}`（Circle payload {cx,cy,r,rot} / Rectangle {cos,sin,cx,cy,hw,hh} 字节往返测试过）✓。
- `IsEnclosed {No,Yes,PremadeShape(ShapeParameters)}`，serde rename
  "IsEnclosedNoYesPremadeShapeIsEnclosedV2"，变体 0/1="No"/"Yes"（0x1416eb640）✓。
- 方法族：center/as_curve2/as_curve3/contains_point/contains_shape/get_normal_at_u/get_pos_at_u/
  get_closest_u_from_pos/perimeter/expand 等——圆 `r+=a` 且 +0x14/+0x18 车道 `+=2a`、矩形
  hw/hh=**2a 覆写**（O 机器码，调用方语义 U）✓。
- **待取证（诚实）**：① contains_shape 自圈分支用 `other.center()` vs 章节/叶
  `get_pos_at_u(0)` 出入——登记，未定案；② ShapeParameters JSON 精确 wire U；③ expand 矩形
  覆写调用方语义（render 预览域）。

### 2. snapping 7 系统 — ✅ 抽查命中

- `snapping_init`：重置→debug 门→无进行中墙→**曲线长≤1.65 早退**→**首段投影步行（1.5 阈值,
  eps 0x1e3ce508=1e-20）**→`mode=1` + `snapped`（stop_index != points.len()）✓（测试覆盖直/弯曲线）。
- `check_for_disable_snapping`（mode∈{2,3}→DisableSnapping{flag}，flag=1 清 mode）、
  `snapping_to_ends`（1e8 哨兵/1.5 阈值/+0x29e 字节）、`snapping_to_segments`（±8.0 AABB+
  1.5+地形采样）、`snapping_audio`（mode2→kind1/mode3→kind2，xvy swizzle）、`snapping_signifiers`、
  `telegraph_walls_are_merged`——均按 O 结构实现。
- **+0x284/+0x29e 字节字段名待取证**（投影访问器已加）✓。

### 3. eraser / shapes — ✅ 抽查命中

- `wall_eraser`（stroke→切片→enclosure tag2→height 0.75..14→平屋顶 -0.79→archivist 缓存）、
  `WallEraserArchivist`（affected/cache_if_original/shrink_wall/finish→RecordWallBulldoze）、
  `ui_bulldozer`（is_bulldozer_layer=AppMode::Eraser(Bulldoze) tag==9&&+4==3）、
  `animate_wall_bulldoze`（**phase+=dt/0.2@0x3e4ccccd→CubicInOut→切片[low*eased, high*eased+
  1-eased]→phase==1 移除 authority→retain_finished**）——代码与 O 结构逐条对上 ✓。
- shapes：calc_shape_height（base rect 6/circle 8、-0.46、clamp[0.7,14]、CalcShapeHeight 事件）、
  create/edit/ui/finish circle+rectangle、shape_placement（AABB 查墙+1.5 吸附+法向）、signifiers。
  **create_circle/rectangle 的 roof_height_01/ridge_length_01 结构近似（常量待取证）** ✓。

### 4. 依赖补齐 — ✅ 登记

- `InProgressCurve`（resources/in_progress_curve.rs 新建）：new/try_add_raw_input_points/first/
  last/raw_input_curve_length_ws/is_raw_input_curve_valid 按章节；**compute_final_curve 16KB
  重采样待取证（现直连点近似）** ✓。
- `WallSpatialHash::query_with_aabb_clamped`（0x140aa5a10）补齐；双 `Curve2` to_authority_curve
  桥接（eraser 处 WallId 双域显式转换）✓。

### 5. 诚实 — ✅

RESTORE §6 明确分层：待取证（compute_final_curve/ridge 脊/roof_height_01 常量/ShapeParameters
wire/expand 覆写/telegraph +0x284/snapping 代理资源/lerp rate/dims 投影）+ 待接线（render/UI/
输入域：signifier 实体 spawn/hammer 图标/widget/trigger_volume/输入 action 0x1409236d0/射线拾取/
Bulldoze run_if 宿主/装饰恢复 extract_before_after_all/ChildWall 分裂）——未伪造；**未因编译通过
标 review_passed** ✓。

## 结论与 DISPATCH 建议

- **R4+R5 = review_passed（限定），闭合**：valid_enclosure（28B/Shape/IsEnclosed/expand）、
  snapping 7 系统、eraser+shapes 库内主体、依赖补齐（InProgressCurve/WallSpatialHash 桥接）——
  抽查与 O 结构/常量对上；模块 22 测试过。
- **登记项（不阻塞）**：contains_shape 自圈分支出入、compute_final_curve 重采样、ridge 脊/
  roof_height_01 常量、expand 覆写语义、+0x284/+0x29e 字段名、render/UI/输入域接线（§6）。
- 建议 DISPATCH：R4+R5=review_passed（限定）；验收层（消费端接线+全局调度）按 §6 登记。
