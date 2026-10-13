# B 阶段 d_main：systems/render_loop/draw_passes（不含 draw_shaders/）

范围：`reconstruction/country-core/src/systems/render_loop/draw_passes/` 下 17 个文件
（common_draw_params, custom_cursor_pass, deferred_raster, draw_lists, feedback_report_bg, final_post, foliage_exclusion_mask,
logo_overlay, main_pass, material_id, mod, object_grass_push, shadow_pass, transparency_pass, ui_2d_pass, ui_pass, wireframe_pass）
+ 新增 `d_main_support.rs`（共用运行器）、`d_main_tests.rs`（pass 级测试）。标注：O = 机器码观测，I = 推断，U = 未知。
`grep todo!/unimplemented!` 在这些文件里为空。未 commit，未改共享文件（lib.rs / rhapsody_types.rs / rhapsody_contract / A4 文件）。

## 1. 关键发现（推翻旧骨架）
1. **`DrawLists` = 21 个 `Vec<(DrawItem, fn)>`**（0x1f8 B，每个 24 B），不是 `Vec<DrawData>`；元素 0x80 B = `DrawItem`(0x70 B)+回调指针（+0x70）。
   排序键 = **i32 @ 元素+0x60**，稳定排序（<0x15 插入排序 / 否则 driftsort = `sort_by_key`）。`DrawItem` 字段：+0x20/0x30/0x3c 变换（`bind_common` 的 transform 参数就是元素指针）、
   +0x50 包头 u64（`DrawPacket.head[1]`，`head[0]=1`）、+0x58 着色器句柄、+0x60 排序键、+0x64 `Option<u8>` 模板参考值（`unwrap`，只有 main/oc2/late 读）。
   与 A4 `draw_list::DrawListItem{sort_key:u64, shader_id:u64}`（I）不符，本模块用自己的 `DrawItem`（见 §5）。
2. 21 个列表下标（由各 pass 读的 Vec 偏移 `(ptr_off-8)/0x18` 得到，O）：0 shadow、1..6 main（G-buffer/emissive/velocity/Forward/ForwardVelocity/Velocity）、
   7..11 OC/2、12/13 Late Deferred/Forward、14 refraction、16 pre-TAA、17 post-TAA、18 loading、19 UI、20 wireframe（15 未被本范围 pass 读取，U）。
3. **主 pass 6 个子 pass**（名字/颜色 tag/深度 tag/列表/状态改写全部来自机器码，见 main_pass.rs 头注）。`ColorAttachment` 出现了 **tag 2**
   （首个 G-buffer、feedback_report_bg、mei_pass 的临时目标）：访问 0x2e、描述里 `+4=2`；A4 的枚举只有 tag0/1，本模块在 `DrawPassCtx::raster_pass` 里建完 pass 后补丁（`pass.colors[i].tag=2; accesses[i].access=0x2e`）。
4. `Forward`/`ForwardVelocity` 由第二个资源（枚举 `ShaderRefactoringState`，首个 dword）**== 0（disabled）** 门控；OC/2 没有该门。Late 两个子 pass 与 post-TAA 在列表为空时**不创建 pass**。
5. **`bind_common` 的布尔字节实际是 u8**（`movzx r14d, byte [params+4]`，`write_uniform<u32>` 写整个值）；`shadow_pass` 传 `cascade+2`。A4 的 `CommonDrawParams.flag: bool` 装不下 → 本模块在 `ItemMode::Shadow` 里 `bind_common` 之后再 `uniform(0xc5ab673fb08651f5, flag as u32)` 覆盖（常量名在 A4 里是私有 const，本模块复制了一份 `d_main_support::BIND_COMMON_FLAG_INT_UNIFORM`）。
6. `shadow_pass`：3 个级联各建 pass（深度 tag0、layer=i、清除 1.0；无颜色），`DrawBuilder::new(variant=1)`，`bind_common` 后 `state.byte0 &= 0xcf`；读列表 0。
7. A4 在本轮同时落地了 `DrawBuilder::bind_brick_erase_highlight`（与我独立反汇编 0x14090be10 的结果逐项一致：bool 0x203fd61c57064e54=`!blocked && active`、Vec3 0x06bdce4199980bf7=光标位置、f32 0xb585ce21b6c728c4）——本模块只保留资源值 → A4 入参的转换 `BrickEraseInputs::to_a4`。

## 2. 过程 → Rust 项
| 地址 | PDB | Rust | 结论 |
|---|---|---|---|
| 0x1409ba340 | `DrawLists::new` | `DrawLists::new` | O（21×{0,0x10,0}）|
| 0x1409ba4b0 | `DrawLists::sort` | `DrawLists::sort` | O（键 i32@+0x60，稳定）|
| 0x1409ba540 | `build_draw_lists` | `draw_lists::build_draw_lists(&[DrawListSource], &mut DrawLists)` | **部分**：推入列表的逻辑 O；着色器名 → (闭包, 列表) 分发表（~70 个名字，17 个 push 站点）**U 未逐项解码**（ECS 查询与分发表由调用方给出 `DrawListSource`）|
| 0x1409ccb20 | `wireframe_pass` | `wireframe_pass::wireframe_pass` | O |
| 0x140837c40 / 0x140837ff0 | `ui_pass` / `loading_screen_pass` | `ui_pass::{ui_pass, loading_screen_pass}` | O（同一名字 "UI"，列表 19/18，无状态改写）|
| 0x140833bc0 | `main_pass` | `main_pass::main_pass` | O（6 子 pass）|
| 0x140835a40 | `main_pass_oc2` | `main_pass::main_pass_oc2` | O（5 子 pass）|
| 0x1408373b0 | `late_forward_and_deferred_pass` | `main_pass::late_forward_and_deferred_pass` | O（空列表守卫；`r9` 未用 U）|
| 0x1409cc270 | `shadow_pass` | `shadow_pass::shadow_pass` | O |
| 0x1409bf320 | `refraction_copy_pass` | `transparency_pass::refraction_copy_pass` | O |
| 0x1409c3a80 / 0x1409c40b0 | `pre/post_taa_transparency_pass` | 同名 | O |
| 0x1409bf800 / 0x1409c08d0 | `water/ice_surface_pass` | `WATER_PLAN` / `ICE_PLAN`（操作序列）+ `water_surface_pass()`/`ice_surface_pass()`/`surface_plan_for` | **部分**：每个 pass 的调用序列+绑定名哈希逐条由机器码提取（脚本，非手抄）；资源接线 U（见 §4）|
| 0x1409d1710 | `custom_cursor_pass` | `custom_cursor_pass::custom_cursor_pass` | O |
| 0x1408a5a70 | `render_feedback_report_bg` | `feedback_report_bg::render_feedback_report_bg` | O |
| 0x1408a5f50 / 0x1408a5f40 | `render_logo_overlay` / `LogoOverlayPlugin::build` | `logo_overlay::*`（build = 空，函数体仅 `ret`）| O |
| 0x1409cb7b0 | `rasterize_foliage_exclusion_mask` | `foliage_exclusion_mask::*`（含静态脏标志 0x143634cf8）| O（脏标志初值 U）|
| 0x1409daaa0 | `object_grass_push_pass` | `object_grass_push::object_grass_push_pass(objects)` | **部分**：pass/绘制/uniform O；`Vec<Vec4>` 的 ECS 组装与半径选择条件 U（常量 6.0/7.5/7.125/9.0 已取）|
| 0x1409c4470 | `ui_2d_pass` | `ui_2d_pass::ui_2d_pass` | O 流程与 uniform；组件字段名 U（见 §4）|
| 0x1409c6700 | `mei_pass` | `ui_2d_pass::mei_pass` | O；`TextRenderer::render`（tg_text）与 `create_raw_resource("mei temp")` 由调用方提供产出 |
| 0x1409d20b0 | `deferred_raster` | `deferred_raster::{DEFERRED_OPS, shader_uri, …}` | **部分**：绑定序列表 O；资源接线 U |
| 0x14090d850 | `resolve_to_swapchain` | `final_post::resolve_to_swapchain` | O（`sample_count != 1` → msaa 着色器；闭包 0x1409d00a0）|
| 0x14090d000 | `final_post` | `final_post::{FINAL_POST_OPS, …}` | **部分**：闭包 0x1409ce780 操作序列 O；值来自 `ImageFormationConfigs`/`ColorGradingState` U |
| 0x14085da60 | `<MaterialId as Debug>::fmt` | `material_id::MaterialId`（Solid/Grass/Plant/Canopy/Forward/COUNT，读自 EXE 字符串表）| O |
| — | `CommonDrawParams::new`（inline 行 26）| `common_draw_params::CommonDrawParams::new(Duration, flag)` | O（`Duration::as_secs_f32`）|
| 0x1409cdec0/ce780/d00a0 | `raster_single_object` ×3 | A4（`render/raster_single.rs`）；闭包本体在 `mei_pass` / `resolve_to_swapchain` / final_post 计划里 | O |

其余 PDB 条目（`bevy_ecs::…FunctionSystem<…>`、`downcast_rs`、`ssbo_library::impl$0` 等 ~150 个）是 bevy/库的单态化实例，按硬规则 1 不恢复；
`read_storage_buffer<CustomCursorSsbo/ShapeStreamSsbo/TextPrimSsbo>` 与 `read_access_buffer<…>` 由 A4 泛型覆盖，SsboId 立即数在本模块 `SsboType` impl 中取自 `get_rg_buffer<T>`（0x1412c7550 / 0x1412cbb70 / 0x1412d1b10）。

## 3. 本模块新增的共享层
`d_main_support.rs`（无独立 PDB 过程）：
- `DrawPassCtx{ world:&mut World, elapsed:Duration(=Res<Time>), 宿主工厂, ShaderLookup }`：`raster_pass()`（new_impl，含 tag2 补丁）、`common_params()`、`shader(name)`（`get_raster_handle_by_name`，未命中由各 pass `expect`）。
- `run_list_pass(ctx, ListPass{name,colors,depth,viewport,flag,mode,variant,extra}, items)`：O 的内联序列 `DrawPacket::new(head=[1,item.+0x50], shader=item.+0x58, 默认状态) → DrawBuilder::new → [stencil_ref unwrap] → bind_common(params,item) → 额外绑定 → 状态改写 → 回调(item+0x70) → render`；`ItemMode::{Plain,Wireframe,Stencil,Velocity,Shadow,Transparent}`。
- 状态位辅助：`with_stencil_write`（byte1=(b&0x8e)|0x21）、`velocity_state`（(s&0xffffff8f)|0x20，**在构造包之前**）、`wireframe_state`。

## 4. U / I 清单（不编造）
- U `build_draw_lists` 着色器名分发表；列表 15 的使用者；`DrawItem` 的 +0x00..0x20 / +0x48..0x50 / +0x66..0x70；`head[1]` 语义；`stencil_ref` 写入 `packet.+0xb1` 的名称（I：`DrawBuilder::stencil_ref`）。
- I 位名：`set_polygon_mode`=byte3 bit7、`set_depth_test_enable(false)`=byte0 bit7 清除（与 A4 mei_pass 实例同一位；`RasterState` 各位名称仍 U）。
- U `mei_pass` 临时目标颜色 tag（`ebx`，I 取 tag2）；`late_forward…` 第 4 参数。
- U `ui_2d_pass` 元素元组（0x30 B）各项的组件真名：`Ui2dExtent`（4×f32）、`Ui2dImage`（名字 &str@+8/+0x10、层号@+0x18）、tint（f32@+0x18、bool@+0x21）、button（bool@+2）、排序键 `Option<&u64>`@+0x20（I：None 最小）。`components::Ui2d*` 在 crate 里是 typed gap，本模块定义了只含被读字段的记录。
- U 水/冰/deferred/final_post 的**资源句柄接线**：计划表里每个 `BindTexture*`/`WriteUniform` 的哈希是 O，值对应的 `Handle<Texture>`/配置字段 U；冰 pass 的 RG 资源名顺序是 I（池里字符串边界）。
- U `object_grass_push` 半径选择条件（`[res+4] ∈ {0,3}`、`[res+0x4d0]==3`）。
- U 全局脏标志初值（`FOLIAGE_EXCLUSION_DIRTY`，取 false）。

## 5. 对公共 API 的改动（旧 → 新，原因均为机器码）
- 所有 pass：`fn(&mut Gap, …)` → `fn(&mut DrawPassCtx, <显式纹理/列表/输入>)`。无外部调用者（grep 确认）。资源由调用方读出后传入（`world: &mut Gap` 无法表达 TypeId 资源查找）。
- `DrawLists(pub Vec<DrawData>)` → 21 个列表；`DrawData`/`DrawData::new*`/`apply_topology` 删除（原版是 `build_draw_lists` 内联闭包，无对应函数）；`DrawLists::new(&Gap)` → `new()`。
- `common_draw_params::CommonDrawParams(pub Gap)` 删除，改为 A4 `draw_builder::CommonDrawParams` 上的 `new(Duration, bool)`；删除无 PDB 对应的 `bind_roof_color_customization` / `bind_window_color_customization_textures`（源码行号 84/89 的 closure 内联）。
- `ShapeStreamSsbo/TextPrimSsbo/CustomCursorSsbo(pub Gap)` 现在带 `SsboType`（SsboId 取自机器码）；`MaterialId(pub Gap)` → `#[repr(u32)]` 枚举；`transparency_pass::water_surface_pass(world,…)`/`ice_surface_pass` 现在返回 `&'static SurfacePlan`。

## 6. 给集成者 / A4 的请求（共享改动，未执行）
1. `draw_builder::CommonDrawParams.flag: bool` → **u8**（`bind_common` 读 `movzx` 字节，uniform 写整个 u32）；落实后 `ItemMode::Shadow` 里的覆盖写可删。
2. `framebuffer_binding::ColorAttachment` 增加 tag2 变体（访问 0x2e，`+8` 引用）；落实后 `raster_pass` 里的补丁可删。
3. `draw_list::DrawListItem/DrawListFn/EntityDrawParams` 与本模块的 `DrawItem/DrawFn` 重复且字段不同；建议 A4 的 `draw_list.rs` 改为 `pub use` 本模块的类型，或各自保留（本模块不依赖 A4 的 `DrawListItem`）。
4. `draw_builder.rs` 的 `BIND_COMMON_FLAG_INT_UNIFORM` 等 const 为私有；若设为 `pub`，可删 `d_main_support` 里的副本。
5. `PassHost` 缺：`AssetShaderLibrary::get_raster_handle_by_name`（本模块用 `ShaderLookup` 补）、`RenderGraph::create_raw_resource`/`TextureDesc::new_2d`（mei_pass 的 `mei temp`、final_post 的 `msaa color out`、水/冰的 RG 资源，目前由调用方提供）。
6. `lib.rs` 的 `consume::` 再导出里若仍引用 `draw_passes::draw_lists::DrawData` 等旧名需删；`resources/session::GladeType::is_winter` 被 `transparency_pass` 使用（沿用旧 import）。

## 7. 验证
隔离测试台 `/tmp/h_d_main`（A4 harness 副本，`#[path]` 挂载本范围 17 个文件 + `d_main_tests.rs` + A4 的 draw_builder/draw_stream/framebuffer_binding/draw_list/raster_single + 真实 `texture_presets`/`name_hash`/`rhapsody_types`；`ColorLibrarySsbo`、`GladeType`、`rhapsody_contract` 为最小替身）：
```
L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_d_main CARGO_TARGET_DIR=/tmp/l0h-target /tmp/h_d_main/run.sh
→ test result: ok. 103 passed; 0 failed   （含 A4 的 67 个；本范围新增 36 个，0 warning）
```
本范围 18 个 `.rs`（mod.rs 除外，rustfmt 会递归 draw_shaders）已 `rustfmt --edition 2024 --config max_width=120`。
覆盖：draw_lists（21 列表、有符号稳定排序、>0x15 元素走 driftsort、列表下标↔Vec 偏移、build 推入）、各 pass 的名字/附件 tag/访问码(0x36/0x2e/0x2f/0x17)/清除/列表选择/门控（Forward 门、Late 空列表、post-TAA 空列表、cursor 可见性、feedback 三个门、foliage 的三态+脏标志）、状态位掩码、`stencil_ref=None` panic、shadow 级联(layer/clear/variant=1)、ui_2d 排序+着色器选择+ws 在后、mei 两个 pass、resolve 的 msaa 选择、MaterialId 字符串表、负立即数常量（`as u64` 对照）、`Duration::as_secs_f32` 精度（2^24 以上）、窗口尺寸 f32→u32 饱和转换。
**未验证**：真实 crate 整体编译（其它 agent 并发修改中；本范围依赖 `crate::asset_libraries::color_library::ColorLibrarySsbo`（需实现 `pass_resource_access::SsboType`，bind_resources.rs:207 已有）、`crate::name_hash`、`crate::resources::session::GladeType`）。

## 8. 遗留
1. `build_draw_lists` 分发表（~70 个着色器名 → draw_shaders 闭包 + 目标列表）与 ECS 查询——需逐个解码 0x1409ba540 内的 17 个 push 站点与 25 个函数指针；依赖 draw_shaders 的最终签名。
2. 水/冰/deferred/final_post 目前是“操作计划”，要变成可执行需要 RG 图录制层（`RenderGraph::create_raw_resource`、`ComputePassInit`）与各资源句柄来源。
3. `ui_2d_pass`/`object_grass_push` 的组件/资源真名（见 §4）。
4. `ui_2d_pass` 的 `TextureViewDesc`（view_type 1 / base_layer / layer_count）A4 里是单元占位，目前只记录在 `Ui2dImageRef::view()` 并以 `Some(&default)` 传入。
