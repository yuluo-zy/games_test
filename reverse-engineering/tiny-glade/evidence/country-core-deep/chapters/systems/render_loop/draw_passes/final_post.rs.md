# systems/render_loop/draw_passes/final_post.rs

> 文件作用：最终后处理（最终图）：`final_post`（色调映射/grading LUT/ICC LUT）、`resolve_to_swapchain`（MSAA 解析到交换链）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::final_post::final_post` | 0x14090d000 | 44 | 最终后处理：串证据 `Final blit` + `default`/`assets/luts/.png`（LUT 资产默认名）+ msaa color out；ImageFormationConfigs::current + ColorGradingState::flattened + RenderGraphRecorder::create；闭包绑定 LUT（GradingLut::lut_name_to_asset_name）+ read_texture + uniform；`_final_post.raster`（色调映射/grading_lut/icc_lut/tony_mc_mapface_lut/shadow_mask_mean） | O | render-loop\decomp\decomp-0x14090d000.txt（串：Final blit、assets/luts/.png、msaa color out）；wp5 draw-passes §6 |
| `country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain` | 0x14090d850 | 195 | 解析到交换链：串证据 `Resolve to swapchain` + `raster_resolve_to_swapchain` + `raster_resolve_msaa_to_swapchain`；Handle::desc 对比 + get_resource；`_blit_resolve_to_swapchain`/`_blit_resolve_msaa_to_swapchain`（MSAA 档）/`_blit_blit_stretch_decode_srgb` | O | render-loop\decomp\decomp-0x14090d850.txt（串：Resolve to swapchain、raster_resolve_to_swapchain、raster_resolve_msaa_to_swapchain）；wp5 draw-passes §6 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（closure$0/closure$1 行 120/244：read_texture + uniform）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `final_post`（0x14090d000 + 闭包 0x1409ce780）—— 完整接线（d_gap §4）
- **`FinalPostEnv`（0x98B，字段偏移 = rbp-0x40+k）**：缩放比分类（`ratio < 0.95 → 4`，`< 1.05 → 1`，否则 `2`）、`LutSelect{Custom/Keep/Default}`（模式 0/1/3，niche i64::MIN）、`GladeTheme == 0x13`、自定义 LUT 纹理 Option。
- **`final_blit_body`（闭包逐调用序列）**：12 次 `write_uniform` + `optional_uniform<[u32;2]>`；**动态名绑定**（`format!("grading_lut:{}")` + `NameHash::compute`）+ `ShadowMaskMean` 缓冲读取（旧版 FINAL_POST_OPS 漏了这两处，d_gap 已更正）。
- **资源 TypeId 角色**（O）：`0x7e3bf548…` = `Option<Handle<Shader>>`；`0x4a142e09…` = 当前档配置数组（stride 0x40，选择字节 +0xc0 = ImageFormationConfigs kind）；`0xc2b8e941…/0x59f3c6d1…` = GladeTheme；`0x5f06b298…` = 自定义 LUT。
- 颜色附件 `ColorAttachment::Tag2`（i_api 已提供）+ `raster_single_object::<1>("Final blit", …)`。"msaa color out" 由调用方经 `PassHost::rg_resources().create_raw_texture` 创建（format 0x2b、sample_count 4）。
- 着色器：`_final_post.raster`（色调映射/grading_lut/icc_lut/tony_mc_mapface_lut/shadow_mask_mean）。

### `resolve_to_swapchain`（0x14090d850）
`sample_count != 1` → msaa 着色器（`_blit_resolve_msaa_to_swapchain`）否则 `_blit_resolve_to_swapchain`；`_blit_blit_stretch_decode_srgb`（菜单/缩略图 3 顶点拉伸）。`Handle::desc` 对比 + get_resource。
- 帧序：最终图（R12-R14，展示图 execute）内 final_post → resolve_to_swapchain → present；`lut_gallery`/`vermeer`/`menu_background`/`session_selection` 等 UI 后处理同图。

## 关键事实

- 最终图属 `tiny_glade::render_loop`（R12–R14），country-core 仅提供 final_post 组装（wp5 O 名）。
- LUT 默认资产：`assets/luts/.png`（占位名，ColorGrading 域）。
