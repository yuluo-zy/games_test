# WP19 契约 3/4 — LUT 资源与画廊（gpu_lut / grading_lut / lut_gallery）

版本 **B**。模块：`resources/gpu_lut.rs`、`resources/grading_lut.rs`、
`systems/render_loop/lut_gallery.rs`。数据源：本包反编译 + PDB 骨架 +
`INDEX.md §107/§135`（final_post 读 `grading_lut_tex(3D)/icc_lut(3D)/tony_mc_mapface_lut(3D)`）、
`pass-builder/shader-uri-list.json`（`/lut/brdf_fg.cs.hlsl`、`/lut/water_noise.cs.hlsl`、
`/lut_gallery/thumbnail.raster.hlsl`）。

标注：**O**=反编译/PDB；**I**=推断；**U**=未恢复。

## 1. gpu_lut（`resources/gpu_lut.rs`，modules 504/553）

### 类型（O；布局 U）
- `GpuLutPlugin`、`GpuLutTracker`、`GpuLutApi`（trait）、`GpuLutDesc`。

### `GpuLutPlugin::build`（O，0x14090e080）
构造 `GpuLutTracker` 预置 **2 个 GpuLutApi 条目**（O；非 demo 分支 2 个，demo 分支 1 个），
`insert_resource<GpuLutTracker>()`。条目默认字段 `{+0x08=0, +0x10=8, +0x18=0, +0x20=1}`，
名称指针/0 长度（I：两个 LUT 生成器 = `/lut/brdf_fg.cs.hlsl` 与 `/lut/water_noise.cs.hlsl`，
与 shader-uri-list 一致；name 字符串在 anon 符号，U）。

### `GpuLutTracker::render_luts`（O，0x14090e280）
若 `count@+0x10 != 0`：遍历 `Vec@+0x08`（stride **0x30**）每条目，若 `rendered@+0x28 == 0`：
`+0x10=0`；`(**(vtable@+0x20 + 0x28))(api@+0x18, ...)`（O：trait 对象 vtable 调用）→
`+0x28 = 1`。

### `compute_impl`（O，0x14090e310，`impl$3::render_luts::impl$0::compute_impl`）
vtable+0x18 调用 + `ComputePass::new<&dyn AsComputeShaderSource>()` +
GpuLutApi 方法（`(*(api+0x18))(api, pass)`）——单条 LUT 的 compute 运行辅助。

### build closures（O，0x140a716c0/0x140a71980）
`FnOnce::call_once<build::closure_env_{0,1}, (&mut dyn GpuLutApi)>` → vtable+0x18。
`impl$1::compute`（inline @63，宿主即两闭包）= 每 LUT 的 compute 体；
`impl$0::build::closure$X::closure$0`（inline，`ComputePass` 闭包 @29/@37）。
`GpuLutDesc::new`（inline @80）。

### typed gaps
`GpuLutApi` trait 全方法面（vtable +0x18/+0x28 槽语义）；`GpuLutDesc` 字段；两条 LUT 名字
（I：brdf_fg/water_noise）；`GpuLutTracker` 剩余布局。

## 2. grading_lut（`resources/grading_lut.rs`，modules 16..2845 共 23 个）

### 类型（O；布局 U）
- `GradingLut`（单条 3D LUT）、`GradingLuts`（目录/集合）、`ColorGradingState`
  （当前分级选择）、`LutGalleryState`（画廊状态；由 lut_gallery::build 插入，O）。

### `ColorGradingState::flattened`（O，0x140b00b20）
输入 `ColorGradingState`：`String name@+0x00`、`{f32,f32,f32,f32}@+0x18/+0x1c/+0x20/+0x24`、
`byte@+0x28`（是否已命名）。输出扁平结构：
- 已命名：clone name 字符串，复制 4 值。
- 默认：name = `"default"`（O 字符串，8 字节），值 `{1.0, 0.0, 1.0, 1.0}`。
输出 `{String, +0x18..+0x24 4×f32, +0x28 byte}`。**O**：被 `draw_passes::final_post@50`
与 `systems::backend::prepare_frame_constants@50` inline 消费——分级值进帧常量与最终后处理
shader uniforms。

### `GradingLut::load_unwrapped`（O，0x140b00bc0）
`format_inner`（拼路径）→ `image::Reader::open_impl` → `decode<BufReader<File>>`
（DynamicImage）→ `to_rgba8` → drop → `TextureDesc::new_3d + sampled` +
`GraphicsDevice::create_texture<&[u8]>`（**3D 纹理**）。返回 Result（unwrap_failed 或 Ok）。
语义（I）：`.png` 展平 LUT → 3D RGBA8 纹理。

### `GradingLut::load_cube`（O，0x140b01050，完整 .cube 解析器）
1. `fs::read_to_string`。逐行（memchr '\n'/'\r'、trim）。跳过空行与 `#` 注释。
2. 关键字（O 比较串）：
   - `TITLE` → 跳过。
   - `LUT_1D_SIZE <n>` → 错误 “1D LUTs are not supported”（`PTR_s_1D_LUTs_are_not_supported`）。
   - `LUT_3D_SIZE <n>` → `size=n`，数据缓冲 resize 到 **n³**（`memset` 0）——
     n 数字手工解析（逐位 ×10）。
   - `DOMAIN_MIN r g b` / `DOMAIN_MAX r g b` → `parse_rgb`；非 {0,0,0}/{1,1,1} → 错误。
3. 数据行：`parse_rgb` → `utils::rgb9e5::pack_rgb9e5` → 压入 `Vec<u32>`（RGB9E5）；
   越界 → 错误 “Too many entries in the LUT”（`PTR_s_Too_many_entries_in_the_LUT`）。
4. EOF → `TextureDesc::new_3d + sampled` + `create_texture<&[u8]>`；写回 `GradingLut`：
   `name(String)@+0x00`、`size u32@+0x18/+0x1c/+0x20`、纹理句柄区 `+0x28..+0x58`。
   （`local_88[0..11]` 复制，O；精确 3D 纹理句柄布局 U。）

### `impl$2::load_cube::parse_rgb`（O，0x140b01cc0）
按空白 token 解析 3 个 `f32`（`str::SplitWhitespace::next` + `f32::from_str`）→
`{ok:0, r, g, b}` 或 `{ok:1, error}`。

### `GradingLuts::load`（O，0x140b01df0）
`readdir()`（LUT 目录）→ 逐 `DirEntry` 调 `impl$3::load::closure_env$0`
（inline @178/186：每文件 load_cube/load_unwrapped，I）→ 追加到两个 Vec：
`Vec<stride 0x28>`（@+0x00，{ptr,cap,len}@+0x00/+0x08/+0x10）+ `Vec<stride 0x38>`
（@+0x18）→ 写回 `GradingLuts`。

### serde（O 名称，泛型胶水）
`_::impl$0::serialize<&mut Serializer<Vec<u8>,CompactFormatter>>`（0x1400ed2c0 域）、
`serialize<value::ser::Serializer>`、`_::impl$0::deserialize`（serde_json struct）、
`impl$7::eq`（save_game@24 用），`impl$5::clone`、`impl$0::default`
（image_formation::default@36）。这些是 `#[derive(Serialize,Deserialize,Clone,Default,PartialEq)]`
代码生成胶水，不计游戏逻辑。

### typed gaps
`ColorGradingState` 4 值语义（exposure/contrast/saturation/…，U）；`GradingLut` 纹理句柄
布局；`.cube` 的 `LUT_3D_SIZE` 上限；LUT 目录路径；`LutGalleryState` 全布局
（build 默认 `{+0x00=1,+0x08=1,+0x10=0,+0x14=0,+0x18..+0x30=0,+0x35=0,+0x3d=0}`）。

## 3. lut_gallery（`systems/render_loop/lut_gallery.rs`，modules 489/1596/1611/1734）

### 类型（O；布局 U）
- `LutGalleryPlugin`、`render_photomode_lut_gallery::PreviewParams`（闭包内参）。

### `LutGalleryPlugin::build`（O，0x1408b04c0）
分配 `LutGalleryState`（默认见上，**O 偏移**）→ `insert_resource<LutGalleryState>()` +
`add_systems<PreStartup, startup>`（签名 `(Res<GraphicsDeviceResource>, Res<AssetShaderLibrary>,
Res<ShaderWatch>)`，O）。`impl$9::default`（grading_lut，inline @203）同值。

### `render_photomode_lut_gallery`（O，0x1408b0560，4110 字节系统）
1. `_DEV_OVERRIDE_IS_DEMO_MODE != 0` → 返回（O：demo 分支跳过）。
2. 取资源 hash `0xaca0d6d443d3ae2e`（I：`GradingLuts`）；取 `Res<AppMode>`
   （hash `0xd38dae617fb79c7a`）：仅当 `state==10 && byte@+0x08==1`（**摄影模式**，I）。
3. 锁 `GradingLuts` 内部（parking_lot；`+0x4e==7` 守卫，O）。
4. 取 `AssetTextureLibrary`（hash `0x4a142e09642375b9`）查找图标纹理（名称 map
   hashmap 检索，O：16 字节 SIMD 比较内联）与 LUT 相关纹理名。
5. `get_raster_handle_by_name("lut_gallery_thumbnail")`（**O 字符串**）。
6. 取资源 hash `0xd387f1aea6230aea`（I：`GpuLutTracker`/LUT 列表）；`from_iter<Vec2,
   Map<Range<u32>, resources::gui::impl$6::positions::closure_env_0>>`（**O**：画廊网格位置）。
7. 每格位置：`DrawBuilder::new(lut_gallery_thumbnail)`：
   - `bind_texture_to_draw_builder<&Texture>`（图标）、`<TextureFromDisk>` ×2
     （名 hash `0xa647e80af4e9395f` + `NameHash::compute` 结果——分级 LUT 纹理，
     `GradingLut::lut_name_to_asset_name` @82 / `asset_name` @78，O inline）；
   - `optional_uniform<[u32;2]>`（extent）+ uniforms f32 / Vec2 / Vec2 / bool / bool / bool；
   - `draw_no_vao`。
8. `RasterPass::render`；释放锁。

语义（I）：摄影模式下把 GradingLuts 目录逐条绘制为图标+预览的 LUT 画廊缩略图
（`_lut_gallery_thumbnail`，读 `icon_tex`+`grading_lut_tex`，INDEX.md §21）。

### typed gaps
`PreviewParams` 字段；画廊位置布局；每个 LUT 条目的资源句柄来源；`compute` callee
（0x140c9cd80）身份。
