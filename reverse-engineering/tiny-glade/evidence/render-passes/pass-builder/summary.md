# Summary - country-core pass declaration API and shader URIs

只读 Ghidra 反编译 55 个 pass_builder 相关函数，扫描发行 `tiny-glade.exe` 得到 255 条 shader URI。结论：

## API shape
- `ComputePass`（400 字节，按值返回）声明 compute pass：`new(uri)` 经 `AssetShaderLibrary::get_compute_by_name` 缓存 shader 程序，`read_rg_storage_buffer`/`write_rg_storage_buffer(_concurrent)` 解析 RG 资源句柄并追加 0x44 字节 `RenderPassBinding` + 访问范围；`dispatch_1d/2d/3d/groups`、`dispatch_with(closure)`（模式 3）、`dispatch_indirect_raw`（模式 2）、`dispatch_indirect_with(closure)`（模式 4）都落到 `rhapsody::rg::pass_builder::PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`。
- `DrawBuilder`（0xf8 字节）声明 raster/draw：间接绘制走 `AssetSsboLibrary::get_rg_buffer<DrawIndexedIndirectBuffer>` + `multi_draw_indexed_indirect` 等（draw 模式 7，命令步长 0x14），向命令 Vec 追加 0xb8 字节记录。
- `pass_resource_access::read/write_access_buffer<T>` 通用辅助：取 SSBO 句柄、登记访问、追加 binding。观测到 `AtomicCounterBuffer`、`NaniDrawListLengthSsbo`、`NaniNextVisibilityBitmaskSsbo`、`ColorLibrarySsbo`、`Cobblestone*RasterStagingSsbo`、`MouseDepthValueSsbo`、`RtTrianglesSsbo`、`RtTriangleMetaSsbo`、`RtBvhSsbo`、`WallColorIdSsbo`。
- traits：`BindReadOnlyResources`（`read_storage_buffer<T>`、`optional_uniform<T>`）、`BindGameResources`（`bind_color_library`->`ColorLibrarySsbo`、`bind_ray_tracing`->RT SSBO 组）。
- `draw_stream`：`RasterPass::new_impl`/`render`（<=5 附件->`RasterPassColorAttachment`，`HashMap<RenderPassDesc, RenderPass>` 去重，`GraphicsDevice::create_render_pass`）。
- `shader_uri::ShaderUri::parse`：按最后 `:` `/` `.` 切成 3 个 &str 切片，URI 格式 `/<path>/<name>.(cs|raster).hlsl:<entry>`。
- `AsComputeShaderSource` trait（`as_compute_pipeline`/`source_hash`）的 18 个游戏结构体实现 = compute pass 清单。

## Wired pass families（URI 证据）
Nani GPU cull/bucket（main+bush/curved_halftimber/flowers/plaster/roof_tile/smoke/stairs_floor/wooden_plank）、Nani draw-list/prefix-sum 家族、wall generate_draw_lists cs_pass1/2、light_binning、GI/SSGI、DoF（fullres/all_lod0/all_lod1/halfres/halfres_all）、TAA/TGSR、shadow_mask_mean/apply_screen_shadows/volumetrics、HZB/depth-normals、deferred*_light_sun_sky、final_post、water/ice、grass、ui_2d/egui、screenshot 等。254/255 URI 与 compiled-shaders 404 个 .bin 文件名匹配。`shadow_pass`/`main_pass`/`late_forward_and_deferred_pass` 无同名 shader——是对 nani_* + deferred/* + final_post 的组合。

## Gaps
- FSR 编译产物（fsr_copy_history/fsr_init_reactive 等）无静态 `/<name>.cs.hlsl` URI；`/clearing/` 家族为动态拼接。
- `ColorAttachment::desc` 反编译为空占位；draw/dispatch 模式枚举值 2/3/4/7 未映射回 Rust 枚举名；0x44 字节 RenderPassBinding 字段布局未恢复。

## 交付文件
- `pass-declaration.md`、`symbol-locations.json`、`shader-uri-list.json`、`compiled-shaders-full-list.txt`、`dec{1,2,3}_*.c`（55 个反编译）。
