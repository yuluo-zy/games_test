# -*- coding: utf-8 -*-
"""terrain-water 域章节填充器：按 round1/2 证据就地更新 chapters/<file>.md。
用法：python scripts/fill-chapters-terrain-water.py
只写 chapters/ 下的 markdown，不动 reconstruction 源码。
"""
import re, os, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade"
CH = os.path.join(ROOT, "evidence", "country-core-deep", "chapters")
DECOMP = r"terrain-water\decomp\decomp-{va}.txt"
DIGEST = "terrain-water/DIGEST.md"
PRIOR = "multi-baseline/water-cpu-analysis.md"
ROUND2 = "round2/terrain-water-CLOSURE.md"

# file -> (文件作用, {business func 子串 -> (作用, 结论, 证据)}, {inline 子串 -> 作用})
DATA = {}

D = DATA

D["chapters/resources/world_raster.rs.md"] = (
 "世界栅格核心容器 WorldRaster<512>（composite/scratch 双缓冲 u8 栅格 + tabloid DirtyTiles 脏瓦片协议 + 订阅发布）与 world_raster_defs 子模块（命名栅格资源 + AllWorldRasterRead/Write 全栅格 SystemParam）。水/路/鹅卵石/花园六种栅格的公共基础设施；栅格化走 zeno 依赖。",
 {
  "impl$2::clear_buffer<512>": ("按选择字节清空 composite 或 scratch 单缓冲（0x200 段或全 0x40000B）", "O", DECOMP.format(va="0x140a15a10")),
  "rasterize_stroke<512,core::iter::adapters::map::Map": ("点列→zeno Command 表（28B/条，首点 move-to 其余 line-to，grid=(world/130+0.5)*512）并按模式字节+bit0 选三 blend 闭包（min/max/饱和加×笔刷）写入目标层", "O", DECOMP.format(va="0x140a16960")),
  "DirtyTilePublisher::unique": ("构造 unique 变体发布器：判别 0 + 两个空 Vec(cap=8)（38B）", "O", DIGEST),
  "WorldRaster::new<512>": ("分配 composite/scratch 各 0x40000B（512×512 u8）+ DirtyTiles 位图 0x4000B（4096 u32 瓦片状态，初值 0x07954dbb）+ 拷贝 56B DirtyTiles 头模板（含 parking_lot Mutex + 订阅 Vec）+ 清零 staging 0x204B", "O", DECOMP.format(va="0x140a15530")),
  "WorldRaster::subscribe<512>": ("创建 0x28B Arc 订阅记录压入 +0x78 订阅 Vec；非 primary 发布器 panic 'Can't subscribe to a secondary dirty tile publisher'", "O", DECOMP.format(va="0x140a158e0")),
  "_::impl$0::deserialize<enum2$<serde_json::value::Value> >": ("存档反序列化：按 raster 名读取栅格（错误串 'Expected raster '/'End of stream while expecting raster '）", "O", DECOMP.format(va="0x140a197c0")),
  "_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer": ("存档序列化（CompactFormatter）——serde 胶水", "O", DIGEST),
  "impl$2::clear_all<512>": ("双缓冲全清 + 全部 4096 瓦片状态重置为 0x07954dbb（SIMD 四字 + 标量尾）", "O", DECOMP.format(va="0x140a15da0")),
  "impl$2::copy_composite_to_scratch<512>": ("composite→scratch 纯 memcpy + len 断言（'Length mismatch; expected '）", "O", DECOMP.format(va="0x140a161a0")),
  "impl$2::commit_scratch_to_composite<512>": ("scratch→composite 纯 memcpy + len 断言", "O", DECOMP.format(va="0x140a16580")),
  "impl$2::copy_to_staging<512>": ("+0x299 脏→清零 out 区；+0x298=选择字节；DirtyTiles::retain 按脏瓦片拷入 out(SSBO 映射)；+0x29a=1 发布", "O", DECOMP.format(va="0x140a172d0")),
  "build_tile_hasher": ("写 4×u64 种子常量（ahash 风格 4-key RandomState）到 +0/+8/+0x10/+0x18——瓦片哈希器构造", "O", DECOMP.format(va="0x1409393e0")),
  "world_raster_defs::PathRaster::new": ("PathRaster 资源构造（委托 WorldRaster::new）", "O", DECOMP.format(va="0x14083eaf0")),
  "_::impl$0::apply": ("SystemParam apply（ICF 折叠=裸 ret，无状态）", "O", DECOMP.format(va="0x14003bb80")),
  "_::impl$0::init_state": ("SystemParam init_state：注册 6 个栅格资源（get_resource 调用序列）", "O", DECOMP.format(va="0x14083fe20")),
  "_::impl$0::new_archetype": ("SystemParam new_archetype（ICF=裸 ret）", "O", DECOMP.format(va="0x14003bb80")),
  "_::impl$0::queue": ("SystemParam queue（ICF=裸 ret）", "O", DECOMP.format(va="0x14003bb80")),
  "AllWorldRasterWrite::ordered_names": ("按 6 槽名序（PathRaster/Cobblestone1-3/WaterRaster/GardenRaster）输出类型名清单", "O", DECOMP.format(va="0x14083eb10")),
  "AllWorldRasterWrite::write": ("每槽：borrow 标志→清 composite；否则 len 断言+memcpy 入 composite；memset(staging@+0x98,0xff,0x200) 全脏 + +0x299=1", "O", DECOMP.format(va="0x14083ebe0")),
  "AllWorldRasterWrite::would_write_succeed": ("按槽位比对类型名（'PathRaster' 等 0xa 字符）做写冲突检测；缺失资源走 Bevy panic 'Resource does not exist…'", "O", DECOMP.format(va="0x14083f410")),
  "AllWorldRasterWrite::upload_to_staging": ("每槽按 type-id（0x142c2e210 等 6 个）查 StagingSsbo（memcmp 全名）→ vtable 取映射指针 → copy_to_staging(selector=0) 发布 composite", "O", DECOMP.format(va="0x14083fa60")),
 },
 {
  "DirtyTilePublisher::shared": "构造 shared 变体发布器（行 68；共享发布，subscribe 仅限 primary）",
  "DirtyTilePublisher::secondary": "构造 secondary 变体发布器（行 72；不可订阅，见 subscribe panic）",
  "WorldRaster::as_float_grid": "将栅格视为 FloatGrid 视图（71 处内联；供 bilinear_sample_at_world_xz 采样）",
  "WorldRaster::ws_to_px": "世界坐标→栅格像素（(w/130+0.5)*512；8 处内联）",
  "WorldRaster::last_staged": "返回最近 staged 的内容视图（14 处内联）",
  "WorldRaster::replace_composite": "整体替换 composite 缓冲（7 处内联；save 加载路径）",
  "WorldRaster::clear_composite": "清空 composite（7 处内联）",
  "WorldRaster::prepare_tile": "准备/分配瓦片状态（4 处内联）",
  "impl$2::extend_dirty": "扩展脏瓦片集合（rasterize_world_space_stroke 辅助）",
  "impl$2::on_uploaded_to_texture": "上传到纹理后回调（事件响应）",
  "impl$2::on_texture_processed": "纹理处理完成后回调（事件响应）",
  "impl$4::load_index_unchecked": "按索引直接加载栅格单元（无边界检查）",
  "world_raster_defs::Cobblestone1Raster::new": "鹅卵石 1 光栅构造（同 PathRaster::new 模板）",
  "world_raster_defs::Cobblestone2Raster::new": "鹅卵石 2 光栅构造",
  "world_raster_defs::Cobblestone3Raster::new": "鹅卵石 3 光栅构造",
  "world_raster_defs::GardenRaster::new": "花园光栅构造",
  "_::impl$0::get_param": "SystemParam get_param（AllWorldRasterRead 实例 0x1401507ce0/0x1401508130）",
  "_::impl$0::validate_param": "SystemParam validate_param（谓词校验）",
  "AllWorldRasterRead::read": "读全部 6 个栅格的当前缓冲视图（IO/解析）",
 },
)

D["chapters/resources/active_raster_stroke.rs.md"] = (
 "活动栅格笔画：ActiveRasterStroke 0x470B 记录（三列 Vec：固化曲线/进行中输入点/平滑后曲线 + 弧长 + 笔刷/模式/blend 标志 + 双脏位图）与 InProgressRasterStroke 流体输入状态机（add_point/solidify）。draw_raster 与 rasterize_terrain_stroke 的输入源。",
 {
  "ActiveRasterStroke::start_new": ("重置三列 Vec（cap=0, ptr=dangling）→ 写笔刷@+0x60/模式@+0x64/blend flag@+0x68/计数@+0x6c → add_point(首点) + 推入平滑列", "O", DECOMP.format(va="0x140868d00")),
  "ActiveRasterStroke::resample_and_smooth": ("固化曲线列 + 剩余流体点（0.5 间距重采样 solidify）合并 → 对全列做 10 遍 0.25/0.5/0.25 三点平滑", "O", DECOMP.format(va="0x140868e40")),
  "InProgressRasterStroke::add_point": ("输入点列 push（12B Vec3）；累计弧长@+0x40 加 dist(last,new)；弧长>1.0 触发 solidify_fluid_into 固化前缀并 memmove 剩余点", "O", DECOMP.format(va="0x140868fb0")),
  "InProgressRasterStroke::solidify_fluid_into": ("把流体输入点列以 0.5 世界单位定距重采样后并入目标 Vec（<2 点走单点拷贝；尾点合并追加）", "O", DECOMP.format(va="0x1408691b0")),
 },
 {},
)

D["chapters/resources/brush_sizes.rs.md"] = (
 "笔刷尺寸资源：path@+0 / water@+4 / garden@+8 三个 f32 槽位，各自 set=clamp / reset=默认。draw_water/draw_path/draw_garden 读取对应槽位。",
 {
  "BrushSizes::reset_path": ("path 笔刷重置为 0.8（mov [rcx], 0x3f4ccccd）", "O", DIGEST),
  "BrushSizes::set_path": ("path 笔刷 = clamp(value, 0.32, 1.2)（maxss/minss 常量 0x142ac56c0/0x1429cd128）", "O", DIGEST),
  "BrushSizes::reset_water": ("水笔刷重置为 2.0（[rcx+4]=0x40000000）", "O", DIGEST),
  "BrushSizes::set_water": ("水笔刷 = clamp(value, 1.0, 5.0)（0x142925904/0x142925988）", "O", DIGEST),
  "BrushSizes::reset_garden": ("花园笔刷重置为 3.0（[rcx+8]=0x40400000）", "O", DIGEST),
  "BrushSizes::set_garden": ("花园笔刷 = clamp(value, 2.0, 4.0)（0x14292e6f0/0x142934694）", "O", DIGEST),
 },
 {},
)

D["chapters/resources/cursor_terrain_raycast.rs.md"] = (
 "光标地形射线资源：{tag u8@0, pos Vec3@4}，Option<Vec3> 语义。地形类工具（water/garden/path/terrain）读取光标落点。",
 {
  "CursorTerrainRaycast::position": ("tag&1 非零返回 pos（+4/+0xc），否则 {0,0,0}", "O", DIGEST),
 },
 {
  "CursorTerrainRaycast::update": "由射线更新写入命中位置（Some→tag=1+pos；None→tag=0）",
  "CursorTerrainRaycast::new": "构造空资源（tag=0, pos=0）",
 },
)

D["chapters/resources/drawing_input_smoother.rs.md"] = (
 "绘制输入平滑器：把徒手原始输入折线做弧长均匀重采样 + 最小间距抽稀，输出 (x,z) 点对序列（drawing 类工具的输入预处理）。",
 {
  "DrawingInputSmoother::process": ("沿输入折线 t=i/N 参数步进 lerp 取样，最小间距门控（fVar6<dist && fVar7<dist*fVar11）抽稀，输出 8B(x,z) 对入 Vec，回写首点+置位标志", "O", DECOMP.format(va="0x14145c350")),
 },
 {
  "RawInputSmoothed::point_count": "平滑后点数",
  "RawInputSmoothed::is_empty": "平滑后点列空谓词",
  "RawInputSmoothed::points": "平滑后点列视图",
 },
)

D["chapters/resources/glade_border.rs.md"] = (
 "小树林边界：GladeBorder 判定世界位置/形状是否在可编辑边界内（is_pos_inside=跳板→is_shape_inside 主体；依赖 utils Circle2d/CurveU——u∉[0,1) panic 'Tried to init CurveU from an invalid u: '）。",
 {
  "GladeBorder::is_pos_inside": ("跳板：test [rax],1 后跳入 is_shape_inside 共享主体（世界位置界内判定）", "O", DIGEST),
  "GladeBorder::is_shape_inside": ("形状（Circle2d 等）界内判定主体：CurveU 边界校验 + 形状采样比较", "O", DECOMP.format(va="0x140ad2970")),
  "GladeBorder::search_lerped_shape_that_fits_into_glade": ("在界内搜索可容纳的 lerp 缩放后形状（形状放置缩放搜索，6.6KB）", "O", DECOMP.format(va="0x140ad2ce0")),
 },
 {
  "GladeBorder::is_curve2_inside": "曲线（Curve2）界内判定",
  "init_glade_border": "初始化边界资源（曲线/形状数据）",
 },
)

D["chapters/resources/in_progress_curve.rs.md"] = (
 "编辑中曲线（徒手墙原始输入曲线）：raw 输入点累积、验证与终结曲线计算。wall 域（freehand 墙）为主消费者。",
 {
  "InProgressCurve::compute_final_curve": ("由原始输入点计算终结曲线（重采样/拟合，16KB）", "O", DECOMP.format(va="0x140ad41a0")),
  "InProgressCurve::new": ("构造空曲线（Vec 初始化，17KB 含验证分支）", "O", DECOMP.format(va="0x140ad3070")),
  "InProgressCurve::try_add_raw_input_points": ("先调 FUN_141466860 输入验证（返回 3 为合法），非法分支清字段；合法则批量加入原始输入点", "O", DECOMP.format(va="0x140ad3f50")),
 },
 {
  "InProgressCurve::is_raw_input_curve_valid": "原始输入曲线有效性谓词",
  "InProgressCurve::last_raw_input_point": "最后一个原始输入点",
  "InProgressCurve::first_raw_input_point": "第一个原始输入点",
  "InProgressCurve::add_weight_to_the_last_point": "给末点加权重（曲线权重累积）",
  "InProgressCurve::raw_input_curve_length_ws": "原始输入曲线世界长度",
 },
)

D["chapters/resources/mouse_terrain_raycast.rs.md"] = (
 "鼠标→地形拾取链：CameraCursorRay {tag u32@0, pos@4, dir@0x10, length@0x1c} 相机光标射线 + camera_mouse_ray 屏幕→世界射线 + mouse_terrain_raycast 地形拾取（xz 平面快路径 / TerrainHeightsData::intersect_ray 高度场求交 / y=0 平面比距取近 / closure$0 实例网格精化）。",
 {
  "CameraCursorRay::intersect_with_xz_plane": ("t=(y-pos.y)/dir.y；返回 pos+t*dir（xz 平面交点）", "O", DECOMP.format(va="0x1408ef180")),
  "camera_mouse_ray": ("由 CursorPositionSS 屏幕位置经 Ray::from_mouse_ss 构建世界射线（utils::geometry::ray 依赖）", "O", DECOMP.format(va="0x1408ef1c0")),
  "mouse_terrain_raycast": ("CameraCursorRay 有效→xz 平面解；否则 TerrainHeightsData::intersect_ray 高度场求交（32 步行进+3 二分），与 y=0 平面交点比距取近，写入 pos/dir-delta/length；closure$0 做实例网格精化", "O", DECOMP.format(va="0x1408ef320")),
 },
 {},
)

D["chapters/resources/raster_stroke.rs.md"] = (
 "栅格笔刷点集：RasterStroke {points: Arc<[Vec3]>, radius: f32} + iter_points_xz 迭代器。rasterize_stroke 的输入（Map<Iter<Vec3>, iter_points_xz>）。",
 {
  "impl$0::fmt<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3": ("Debug 格式化（derive 胶水）", "O", DECOMP.format(va="0x1409f1c90")),
  "RasterStroke::iter_points_xz": ("points 迭代映射为 (x,z) 对（rasterize_stroke 输入迭代器，50 处内联宿主）", "O", DECOMP.format(va="0x140a97860")),
 },
 {
  "RasterStroke::radius": "返回笔刷半径",
  "RasterStroke::add_point": "追加点（Arc 重建）",
  "RasterStroke::smooth": "平滑点列（11 处内联）",
  "RasterStroke::length": "折线总长",
  "impl$4::from": "From 转换（derive）",
 },
)

D["chapters/resources/read_mouse_depth_value.rs.md"] = (
 "鼠标深度回读：MouseDepthValue {w f32@0, xyz@+8, 帧哨兵@+0x40}，来自 MouseDepthValueSsbo（2×float4）的 GPU→CPU 映射回读。",
 {
  "read_mouse_depth_value_from_gpu": ("帧哨兵@+0x40==i64::MIN+3 早退；查 SSBO（名 0x47=71 字符）→ 读 2×float4 做 NaN 过滤（&0x7fffffff<0x7f800000）→ 写 {w, xyz}", "O", DECOMP.format(va="0x140a1a480")),
 },
 {
  "MouseDepthValue::new": "构造（帧哨兵初始化）",
  "from_ndc_to_view": "NDC→视空间坐标转换（深度值辅助）",
 },
)

D["chapters/resources/rocky_terrain.rs.md"] = (
 "岩质地层网格：RockyTerrainMesh 资源 + JSON 中间结构（LoadedRockyTerrainVertex/Triangle）→ 两个 SSBO（RockyTerrainVertexSSBO/TriangleSSBO），displace_rocky_terrain 每帧 GPU 变形。",
 {
  "_::impl$0::deserialize::impl$0::visit_str<serde_json::e": ("JSON 反序列化 visitor（derive 胶水）", "O", DIGEST),
  "RockyTerrainMesh::load": ("fs::read_to_string → serde_json 解析 → add_new_with_data<VertexSSBO/TriangleSSBO>（顶点源 0x24B→0x30B、三角形 0x10B→0x20B）", "O", DECOMP.format(va="0x1408bbf50")),
 },
 {},
)

D["chapters/resources/terrain_editing.rs.md"] = (
 "地形编辑数据：TerrainEditingData {HashMap<TerrainStrokeId, TerrainStroke>（数据@+8/掩码@+0x10/计数@+0x20）, last_stroke_id u32@+0x38}；new_stroke 上限 0x3fffff。编辑笔画的生命周期管理。",
 {
  "color_id::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<re": ("ColorId serde 序列化（ICF 同址共享，derive 胶水）", "O", DIGEST),
  "terrain_editing::_::impl$0::serialize<ref_mut$<serde_json::ser::Serial": ("TerrainEditingData 存档序列化（CompactFormatter）", "O", DECOMP.format(va="0x1408e5bc0")),
  "terrain_editing::_::impl$0::serialize<serde_json::value::ser::Serializ": ("TerrainEditingData serde_json::Value 序列化", "O", DECOMP.format(va="0x1408e6b70")),
  "TerrainEditingData::get_last_mut_stroke": ("按 last_stroke_id 查 HashMap 取末笔画可变引用", "O", DECOMP.format(va="0x140924900")),
  "TerrainEditingData::new_stroke": ("last_stroke_id<0x3fffff 则 +1 并 HashMap::insert，否则 None", "O", DECOMP.format(va="0x140924a10")),
 },
 {
  "TerrainEditingData::new": "构造空数据",
  "TerrainEditingData::get_mut_stroke": "按 id 取可变引用",
  "TerrainEditingData::get_stroke": "按 id 取只读引用",
  "TerrainEditingData::last_stroke_id": "当前笔画 id",
  "TerrainEditingData::set_stroke": "置入笔画",
  "TerrainEditingData::clear": "清空 + id 归零",
 },
)

D["chapters/resources/terrain_height.rs.md"] = (
 "地形高度场：TerrainHeightsData<T> 泛型资源双实例（TerrainHeightSsbo 权威 / VisualTerrainHeightSsbo 视觉）。u16 512×512 网格@+0x08，min/max f32@+0x50/54（默认 -2.5/24.0）；世界采样/射线求交/GPU 回拷差分。全地形系统的数据底座。",
 {
  "TerrainHeightsData::sample_at_world_xz<country_core::r": ("(x/130+0.5) clamp[0,1] → idx=u*512-0.5+1 clamp[0,511] → 返回 u16 网格值；debug 全局字节 *0x14362c308≠0 走 +0x201 偏移视觉网格变体", "O", DECOMP.format(va="0x1409e9640")),
  "TerrainHeightsData::intersect_ray<country_core::resour": ("高度场 AABB（min/max@+0x50/54）进出 t → 32 步行进 + 每步 3 次 0.5 二分逼近交点，逐步采样", "O", DECOMP.format(va="0x1409e9a20")),
  "TerrainHeightsData::copy_from_gpu<country_core::resour": ("min/max 非默认→复位 [-2.5,24.0]；8×8 瓦片粒度 16B 块差分拷贝（不同才拷）并置 64 位脏位图位", "O", DECOMP.format(va="0x1409ea020")),
  "impl$7::load_index_unchecked": ("按索引直接装载网格单元（u16→f32 解码含 min/max；视觉双线性实例 0x141246fb0 内 4 次调用）", "O", DECOMP.format(va="0x1408b54c0")),
 },
 {
  "TerrainHeightsData::sample_at_world_xz": "世界 XZ→高度采样（另见业务行：视觉实例双线性变体）",
  "TerrainHeightsData::heightfield_intersect_ray": "高度场求交核心（intersect_ray 内部）",
  "TerrainHeightsData::subscribe": "订阅地形变化（construct_elevation_supports 消费）",
  "TerrainHeightsData::as_raw": "原始字节视图（save snapshot load）",
  "TerrainHeightsData::as_mut_raw": "原始可变字节视图",
  "TerrainHeightsData::as_bytes": "字节视图（save snapshot capture）",
  "TerrainHeightsData::height_range": "高度范围 [min_y, max_y]",
 },
)

D["chapters/resources/terrain_rock_mask.rs.md"] = (
 "岩体掩码：512×512 位栅格（8192 u32 字，按位打包），世界范围 ±130。GPU 回拷 TerrainRockMaskSsbo 后 CPU 查询（nani 摆放/碰撞过滤）。",
 {
  "impl$1::sample_at_world_xz": ("(x/130+0.5) clamp[0,1]→×512 索引；(y*512|x)>>5 字索引边界检查后取位", "O", DECOMP.format(va="0x1409ea800")),
 },
 {},
)

D["chapters/resources/terrain_stroke.rs.md"] = (
 "地形笔画：TerrainStroke 点列（Vec<Vec3>，上限 1024）+ 插入/形变末点规则（0.001 单点阈值 + spacing 比较）。terrain 编辑数据单元。",
 {
  "_::impl$0::deserialize::impl$0::visit_str<serde_json::": ("serde 反序列化 visitor（derive 胶水）", "O", DIGEST),
  "_::impl$0::serialize<serde_json::value::ser::Serialize": ("serde_json::Value 序列化（derive 胶水）", "O", DIGEST),
  "TerrainStroke::insert_or_morph_last_point": ("len==0→push；len==1 且 dist>0.001→push；len≥2 且 dist(last,prev)≥spacing→push（<1024），否则覆盖 last（morph）", "O", DECOMP.format(va="0x140a97880")),
 },
 {
  "TerrainStroke::new": "构造空笔画",
 },
)

D["chapters/resources/water.rs.md"] = (
 "水内容资源模块：RippleSourceSsbo（鸭子入水波纹源，system_critters 生产/water_planes_draw 消费）+ WaterDrawEvent/WaterEraseEvent 事件。CPU 水位本体在 world_raster_defs::WaterRaster（WorldRaster<512>）。",
 {},
 {},
)

D["chapters/resources/water_mask.rs.md"] = (
 "水 mask 回读缓冲 WaterMaskSsbo：512×512 水 mask 纹理 .y 通道打包回读（>0.5 水位/0.2 clutter 位）。name-only（布局 U）。",
 {},
 {},
)

D["chapters/resources/water_exclude_clutter_mask.rs.md"] = (
 "水域排除 clutter 回读缓冲 WaterExcludeClutterMaskSsbo：.y>0.2 的 shoreline 位打包回读（水域内不摆 clutter）。name-only。",
 {},
 {},
)

D["chapters/resources/garden_mask.rs.md"] = (
 "花园掩码 SSBO（GardenMaskSsbo）：世界栅格 512² 域的花园覆盖回读。name-only。",
 {},
 {},
)

D["chapters/resources/terrain_rock_mask.rs.md"] = (
 "岩体掩码：512×512 位栅格（8192 u32 字，按位打包），世界范围 ±130。GPU 回拷 TerrainRockMaskSsbo 后 CPU 查询（nani 摆放/碰撞过滤）。",
 {
  "impl$1::sample_at_world_xz": ("(x/130+0.5) clamp[0,1]→×512 索引；(y*512|x)>>5 字索引边界检查后取位", "O", DECOMP.format(va="0x1409ea800")),
 },
 {},
)

D["chapters/systems/world_raster.rs.md"] = (
 "世界栅格系统：record_raster_history（栅格历史记录）、upload_world_raster_buffers_to_staging（脏流门 + 6 光栅发布到 staging SSBO）、draw_raster<T> 泛型状态机（10 实例：Path/Cobblestone1-3/Garden/Water，start_new→add_point→resample_and_smooth→rasterize_stroke→copy_to_staging 链）。",
 {
  "draw_raster<country_core::resources::world_raster_defs::Co": ("draw_raster<Cobblestone1RasterStagingSsbo> 实例：同泛型状态机（erase_other_path_variants 闭包变体）", "O", DECOMP.format(va="0x140ab2340")),
  "draw_raster<country_core::resources::world_raster_defs::Ga": ("draw_raster<GardenRasterStagingSsbo> 实例（draw_garden 闭包）", "O", DECOMP.format(va="0x140ab59e0")),
  "draw_raster<country_core::resources::world_raster_defs::Pa": ("draw_raster<PathRasterStagingSsbo> 实例（draw_path 闭包）", "O", DECOMP.format(va="0x140ab4750")),
  "draw_raster<country_core::resources::world_raster_defs::Wa": ("draw_raster<WaterRasterStagingSsbo> 实例：状态 0 start_new；状态 1 last 距>0.0005→add_point+resample_and_smooth+DirtyTiles::clear+rasterize_world_space_stroke+extend×2+交换双位图+copy_composite_to_scratch+rasterize_stroke+SSBO map+copy_to_staging；其余 commit_scratch_to_composite 发布；按 +0x64 发 WaterDraw/WaterErase 事件", "O", DECOMP.format(va="0x142260ae0")),
  "_::impl$0::apply": ("SystemParam apply（ICF=裸 ret）", "O", DIGEST),
  "_::impl$0::get_param": ("RasterDrawResources/RasterHistoryRecordResources SystemParam get_param（另一实例 0x1401507ce0）", "I", DECOMP.format(va="0x141507ce0")),
  "_::impl$0::init_state": ("SystemParam init_state（6 栅格资源注册，两实例 0x140958040/0x1409580e0）", "O", DECOMP.format(va="0x140958040")),
  "_::impl$0::new_archetype": ("SystemParam new_archetype（ICF=裸 ret）", "O", DIGEST),
  "_::impl$0::queue": ("SystemParam queue（ICF=裸 ret）", "O", DIGEST),
  "record_raster_history": ("按 variant·0x470 槽位取 12B 元素笔画 Vec → Arc 克隆 → History::add → 重置 Vec(cap 4)", "O", DECOMP.format(va="0x140957990")),
  "upload_world_raster_buffers_to_staging": ("脏输入流非空 → AllWorldRasterWrite::upload_to_staging（6 光栅 composite→staging SSBO 发布）", "O", DECOMP.format(va="0x140957bf0")),
 },
 {},
)

D["chapters/systems/garden.rs.md"] = (
 "花园系统插件：build 注册 draw_garden/record_garden_history/replay 等系统与 GardenRaster 资源。",
 {
  "impl$0::build": ("插件 build：注册花园绘制/历史记录/回放系统（FunctionSystem 记录佐证：RasterReplayResources+EventWriter<GardenDrawEvent> 等）", "O", DECOMP.format(va="0x140b6d290")),
 },
 {},
)

D["chapters/systems/garden/draw_garden.rs.md"] = (
 "花园画笔系统：工具 id 分发（9=橡皮/17=花丛），半径=笔刷+1.5，界内校验后走 draw_raster<GardenRasterStagingSsbo> 绘制花园掩码。",
 {
  "draw_garden::draw_garden": ("工具 id-6<0xf 且 id-5∈{4,0xc} 分支；Circle2d 半径=笔刷+1.5（常量 1.5@0x1429258d0）；GladeBorder::is_shape_inside 界内校验 → draw_raster<GardenRasterStagingSsbo>", "O", DECOMP.format(va="0x140ae4620")),
 },
 {},
)

D["chapters/systems/garden/record_garden_history.rs.md"] = (
 "花园编辑历史记录：把本帧 12B 编辑单元封装 Arc 进 History::add。",
 {
  "record_garden_history::record_garden_history": ("把当前花园编辑记录（12B 编辑单元 Arc）封装成历史项 History::add", "O", DECOMP.format(va="0x141695e10")),
 },
 {},
)

D["chapters/systems/mesh_follow_terrain.rs.md"] = (
 "网格贴地系统：Query<(&MeshFollowTerrain, &mut Transform, &Handle<Mesh>)> + 地形资源；取 mesh 'Vertex_Position' 属性计算位置 → 采样地形高度 → Transform.y 更新；句柄无效 panic 'MeshFollowTerrain got an invalid mesh handle'。",
 {
  "mesh_follow_terrain::mesh_follow_terrain": ("ECS Query 迭代 + 2×Res（地形高度数据）；FUN_14092d300 取 mesh、FUN_140a07b30 取几何、FUN_1412fd550 位置计算（内联地形采样）→ Transform 更新；无效句柄 panic", "O", DECOMP.format(va="0x140a68430")),
 },
 {},
)

D["chapters/systems/terrain/mod.rs.md"] = (
 "地形系统模块注册：add_terrain_systems/insert_terrain_resources/add_terrain_events 三个注册函数 + 子模块声明 + 跨模块 ECS 类型门面。",
 {},
 {},
)

D["chapters/systems/terrain/deferred_terrain_history.rs.md"] = (
 "延迟地形历史：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史记录 History::add + DebugHistoryMsg。",
 {
  "history_record_terrain": ("PendingTerrainHistory 置脏标记；首次变化构建历史记录并 History::add，发 DebugHistoryMsg；之后不重复记录", "O", DECOMP.format(va="0x140ac9770")),
 },
 {},
)

D["chapters/systems/terrain/delayed_terrain_changed_msg.rs.md"] = (
 "延迟地形变更：ScheduledTerrainReadback {counter AtomicUsize@0, flag bool@8, delays Vec<u32>}；每帧递减 delays→0 触发 GPU 高度回拷 copy_from_gpu×2（权威+视觉），延迟 2 帧发 OnTerrainChange。",
 {
  "ScheduledTerrainReadback::on_terra": ("修改计数原子自增（LOCK inc [rcx]; ret）", "O", DIGEST),
  "delayed_terrain_changed_msg::delayed_terrain_changed_msg": ("delays 逐项递减→0：FrozenMap 取 SSBO（尺寸断言 0x3a/0x40）→ cpu_readback → copy_from_gpu×2；flag 置位 push(2)（延迟 2 帧）；压实完成项", "O", DECOMP.format(va="0x140aabdd0")),
 },
 {
  "ScheduledTerrainReadback::on_gpu_copy_issued": "GPU 拷贝已发出（read_terrain_data 内联）",
 },
)

D["chapters/systems/terrain/draw_terrain_stroke.rs.md"] = (
 "笔画变化转发：EvTerrainStrokeChanged 事件 → RecalculateHeightMapCmd + PlayTerrainSound。",
 {
  "terrain_stroke_changed": ("EvTerrainStrokeChanged 非空 → 发 RecalculateHeightMapCmd + PlayTerrainSound", "O", DECOMP.format(va="0x140a03ba0")),
 },
 {},
)

D["chapters/systems/terrain/edit_terrain_stroke.rs.md"] = (
 "编辑地形笔画：EditTerrainStrokeCmd/SetTerrainStrokePtsCmd 事件 → TerrainEditingData 查询（缺失 panic）→ falloff/height 应用（Absolute 直写/Delta 限幅）→ RecalculateHeightMapCmd + 声音。",
 {
  "edit_terrain_stroke::edit_terrain_stroke": ("按命令 id 查 TerrainEditingData（缺失 panic 'Was requested to edit a terrain stroke  that doesnt exist'）；falloff∈[0,1]/height Abs≥0/Delta≥0.1 应用；发 RecalculateHeightMapCmd + PlayTerrainSound", "O", DECOMP.format(va="0x14094e000")),
 },
 {},
)

D["chapters/systems/terrain/terrain_hover_idx.rs.md"] = (
 "地形笔画 hover：AppMode 位掩码 0x421 门 → MouseTerrainRaycastSsbo 读笔画 id（+0x10，-1 哨兵）→ 水位/光标/距离 0.5 门控 → 写 HoveredTerrainStrokeId。",
 {
  "terrain_hover_idx::terrain_hover_idx": ("AppMode 0x421 位掩码门；SSBO（名 0x6c 字符）映射读 id@+0x10；水位采样≤0/光标无效/距离≤0.5 不变；否则写 HoveredTerrainStrokeId{tag=1,id}", "O", DECOMP.format(va="0x140aff720")),
 },
 {},
)

D["chapters/systems/terrain/ui_draw_terrain.rs.md"] = (
 "UI 绘制地形：高亮/编辑模式点按 → new_stroke + EvTerrainStrokeChanged；界外反馈（DisplayOutOfBorder/FeedbackInputOutOfBorder）。",
 {
  "ui_draw_terrain::ui_draw_terrain": ("Res<AppMode, CursorTerrainRaycast, AssetSsboLibrary, WaterRaster, UiState, GladeBorder> + ResMut<TerrainEditingData, PendingTerrainHistory, DecoratorArchivist> + 事件组；点按新建笔画 new_stroke + EvTerrainStrokeChanged（ui-input 域细查状态机）", "O", DECOMP.format(va="0x140a9fdc0")),
 },
 {},
)

D["chapters/systems/terrain/ui_edit_terrain.rs.md"] = (
 "UI 编辑地形：按 hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；维护 MouseActivityTimer/TerrainStrokeParamsCache。ui-input 域共享。",
 {
  "ui_edit_terrain::ui_edit_terrain": ("按鼠标落点+hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；事件组含 CursorCaptureCmd/EnableArrowGizmoStretch/AddTerrainHighlightCmd 等（ui-input 域主责）", "O", DECOMP.format(va="0x140aef1f0")),
 },
 {
  "MouseActivityTimer::new": "鼠标活动计时器构造（ui_edit_terrain 自有资源）",
 },
)

D["chapters/systems/terrain/ui_terrain_focus_mode.rs.md"] = (
 "地形聚焦模式 UI：进入/退出地形聚焦的状态机（167KB 反编译几乎全内联），4 个闭包（曲线采样辅助/高亮/OutlineWallCmd 状态转换）。ui-input 域共享。",
 {
  "ui_terrain_focus_mode::ui_terrain_focus_mode": ("聚焦模式状态机：alloc/memcpy 为主 + HashMap helper 0x140c9cde0×9；闭包 0x9a3620（FUN_140c97e10 曲线采样，0.5 参数）/0x9a8640/0x9a88c0/0x9a8b40（状态转换，OutlineWallCmd/PublicWalls 相关）", "O", DECOMP.format(va="0x1409a4fb0")),
 },
 {},
)

D["chapters/systems/history/replay_terrain.rs.md"] = (
 "地形历史回放：TerrainReplayCmd 0x40B 条目（重量 f32@+0x14 + 标志）forward/backward 应用/撤销，replay_terrain 迭代重放。",
 {
  "replay_terrain::replay_terrain": ("迭代 ReplayCmd 0x40B 条目缓冲（cursor 范围/计数/重量/标志），逐条处理（回放地形编辑历史）", "O", DECOMP.format(va="0x140995150")),
  "TerrainReplayCmd::forward": ("前向应用一条地形回放命令（笔触应用/重算）", "O", DECOMP.format(va="0x140994f30")),
  "TerrainReplayCmd::backward": ("后向撤销一条地形回放命令（笔触移除/恢复）", "O", DECOMP.format(va="0x140995040")),
 },
 {
  "TerrainReplayCmd::new": "构造地形回放命令",
  "ease_out": "缓出缓动（回放时间曲线）",
 },
)

D["chapters/systems/history/replay_raster.rs.md"] = (
 "栅格历史回放：replay_raster<StagingSsbo, WorldRaster<512>, 闭包> 泛型（6 实例）——把存档 ReplayCmd 笔画重跑 rasterize_stroke→staging→composite 链恢复栅格；RasterReplayCmd::{forward,backward} 单条应用/撤销。",
 {
  "RasterReplayCmd::forward": ("前向应用一条栅格回放命令（重跑 rasterize 链）", "O", DECOMP.format(va="0x141700430")),
  "RasterReplayCmd::backward": ("后向撤销一条栅格回放命令", "O", DECOMP.format(va="0x141700620")),
  "_::impl$0::get_param": ("RasterReplayResources SystemParam get_param", "O", DECOMP.format(va="0x140ae5830")),
  "replay_raster<country_core::resources::world_ras": ("replay_raster<Cobblestone1> 实例（replay_path 闭包；模板同水实例）", "O", DECOMP.format(va="0x1409fc290")),
  "replay_raster<country_core::resources::world_ras": ("replay_raster<Path> 实例（DirtPathFilter 闭包）", "O", DECOMP.format(va="0x1409f1cf0")),
 },
 {
  "impl$6::eq": "RasterReplayCmd 相等比较（derive）",
  "impl$0::from": "From 转换（derive）",
  "RasterReplayCmd::forward_instant": "立即前向应用（无缓动）",
  "RasterReplayCmd::backward_instant": "立即后向撤销",
  "RasterReplayCmd::new": "构造回放命令",
  "_::impl$0::init_state": "RasterReplayResources SystemParam init_state",
  "_::impl$0::validate_param": "SystemParam 校验",
  "impl$8::default": "Default 实现",
 },
)

D["chapters/systems/history/replay_water.rs.md"] = (
 "水域历史回放：replay_water 闭包（EditType→ReplayCmd 转换 + 完成通知）；主逻辑经 replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, closure$2>。",
 {},
 {
  "replay_water::replay_water": "水域回放入口（事件/命令编排）",
  "replay_water::replay_water::closure$0": "回放闭包 0",
  "replay_water::replay_water::closure$1": "EditType→ReplayCmd 转换闭包",
 },
)

D["chapters/systems/history/replay_garden.rs.md"] = (
 "花园历史回放：replay_garden 闭包；主逻辑经 replay_raster<GardenRasterStagingSsbo, WorldRaster<512>, closure$2>。",
 {},
 {
  "replay_garden::replay_garden": "花园回放入口",
  "replay_garden::replay_garden::closure$0": "回放闭包 0",
  "replay_garden::replay_garden::closure$1": "EditType→ReplayCmd 转换闭包",
  "replay_garden::replay_garden::closure$2": "完成通知闭包（replay_raster 泛型参数）",
 },
)

D["chapters/systems/audio/water_audio.rs.md"] = (
 "水音频系统：water_audio_init_system 初始化 WaterAudioState；water_audio_system 消费 WaterRaster 水位驱动音频混合。audio-misc 域主责。",
 {
  "water_audio::water_audio_system": ("按 WaterRaster 水位采样驱动音频混合（audio-misc 域细查）", "O", DECOMP.format(va="0x140873630")),
  "water_audio::water_audio_init_system": ("初始化 WaterAudioState 资源/音频源", "O", DECOMP.format(va="0x140873140")),
 },
 {},
)

D["chapters/systems/audio/water_ambience_audio.rs.md"] = (
 "水环境音频：WaterAudioGrid 消费方（64 位滚动窗 + 双 u16 入水计数）；update_water_audio_grid（system_water）生产网格。audio-misc 域主责。",
 {
  "impl$0::load_index_unchecked": ("网格单元装载（音频网格索引访问）", "O", DECOMP.format(va="0x1409ebe80")),
  "water_ambience_audio_init_system": ("初始化 WaterAmbienceAudio/网格资源", "O", DECOMP.format(va="0x1409ebf20")),
  "water_ambience_audio_system": ("消费 WaterAudioGrid 入水计数驱动水面环境音", "O", DECOMP.format(va="0x1409ecea0")),
 },
 {},
)

D["chapters/systems/audio/garden_audio.rs.md"] = (
 "花园音频：garden_audio_init_system 初始化 + garden_audio_system 消费花园编辑事件/掩码驱动音效。audio-misc 域主责。",
 {
  "garden_audio_init_system": ("初始化花园音频资源", "O", DECOMP.format(va="0x140a88fa0")),
  "garden_audio_system": ("消费花园编辑/掩码事件驱动音频", "O", DECOMP.format(va="0x140a89480")),
 },
 {},
)

D["chapters/systems/audio/terrain_audio.rs.md"] = (
 "地形音频：terrain_audio_init_system 初始化 + terrain_audio_system 消费 PlayTerrainSound 事件播放地形音效。audio-misc 域主责。",
 {
  "terrain_audio::terrain_audio_system": ("消费 PlayTerrainSound 事件播放地形音效", "O", DECOMP.format(va="0x1414618c0")),
  "terrain_audio::terrain_audio_init_system": ("初始化地形音频资源（7.6KB）", "O", DECOMP.format(va="0x1414613e0")),
 },
 {
  "EditStateAudio::new": "编辑状态音频构造",
  "EditStateAudio::update": "编辑状态音频更新",
 },
)

D["chapters/systems/audio/terrain_stairs_audio.rs.md"] = (
 "地形楼梯音频：terrain_stairs_audio_system 消费楼梯/地形交互事件。audio-misc 域主责。",
 {
  "terrain_stairs_audio::terrain_stairs_audio_system": ("消费楼梯地形交互事件驱动音频", "O", DECOMP.format(va="0x141462dc0")),
 },
 {
  "terrain_stairs_audio_init_system": "初始化地形楼梯音频资源",
 },
)

D["chapters/systems/brush_preview.rs.md"] = (
 "笔刷预览：BrushPreview 组件（克隆行为=判别 2 + 克隆函数指针 0x1412eff70）+ brush_preview 系统（预览网格跟随光标）。",
 {
  "impl$6::clone_behavior": ("返回 ComponentCloneBehavior 枚举：判别 2 + 克隆函数指针 0x1412eff70（手解 12B：lea rdx,[rip+X]; mov eax,2; ret）", "O", DIGEST),
  "impl$6::register_required_components": ("注册 BrushPreview 组件所需组件（Bevy 派生胶水，27KB 内联）", "O", DECOMP.format(va="0x140866800")),
  "brush_preview": ("笔刷预览系统：预览网格跟随光标/笔刷大小更新（26KB）", "O", DECOMP.format(va="0x140865050")),
 },
 {
  "brush_size": "笔刷尺寸读取（BrushSizes 访问辅助）",
 },
)

D["chapters/systems/paths/path_brush_preview_size.rs.md"] = (
 "路径笔刷预览尺寸：PathBrushPreviewSize 组件注册 + path_brush_preview_size 系统（路径工具预览）。paths 域主责，本域仅地址确认。",
 {
  "impl$0::register_required_components": ("PathBrushPreviewSize 组件注册（Bevy 胶水）", "O", DECOMP.format(va="0x140b1dd70")),
  "path_brush_preview_size": ("路径笔刷预览尺寸更新系统（paths 域）", "O", DECOMP.format(va="0x140b1c010")),
 },
 {},
)

D["chapters/resources/glade_border.rs.md"] = D["chapters/resources/glade_border.rs.md"]  # 去重占位（无重复键）

# 执行替换
def fill_table_row(txt, key, act, concl, evid):
    pat = re.compile(r"(\| `([^`]*" + re.escape(key) + r"[^`]*)` \| 0x[0-9a-f]+ \| \d+ \| )([^|]*)( \| )([^|]*)( \| )(.*?)(\n)")
    m = pat.search(txt)
    if not m:
        return txt, False
    repl = f"{m.group(1)}{act}{m.group(4)}{concl}{m.group(6)}{evid}\n"
    return txt[:m.start()] + repl + txt[m.end():], True

changed = 0
missing = []
for rel, (purpose, biz, inline) in DATA.items():
    path = os.path.join(CH, rel.replace("chapters/", "").replace("/", os.sep))
    if not os.path.exists(path):
        missing.append(rel)
        continue
    txt = open(path, encoding="utf-8").read()
    # 1) 文件作用
    txt, ok = re.subn(
        r"^> 文件作用：\*\*待推断\*\*.*$",
        f"> 文件作用：**{purpose}**", txt, count=1, flags=re.M)
    changed += ok
    # 2) business rows
    for key, (act, concl, evid) in biz.items():
        txt, ok = fill_table_row(txt, key, act, concl, evid)
        if ok:
            changed += 1
        else:
            missing.append(f"{rel} :: BIZ {key[:60]}")
    # 3) inline list
    for key, act in inline.items():
        pat = re.compile(r"^- (`[^`]*" + re.escape(key) + r"[^`]*`（行 \d+）— )[^\n]*$", re.M)
        txt, ok = pat.subn(lambda m: f"- {m.group(1)}{act}", txt, count=1)
        changed += ok
        if not ok:
            missing.append(f"{rel} :: INL {key[:60]}")
    open(path, "w", encoding="utf-8").write(txt)

print(f"changed: {changed}")
print(f"missing patterns: {len(missing)}")
for m in missing[:30]:
    print("  MISS:", m)
