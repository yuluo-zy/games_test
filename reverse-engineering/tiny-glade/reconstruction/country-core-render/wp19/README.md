# WP19 — country-core render_loop misc + stylized domains (B)

恢复包（交付目录），版本 **B**（`tiny-glade.exe` sha256 `cf048e27…`，PDB `2d9686b8…`）。
范围：country-core 渲染域的 **风格化/菜单/杂项** 模块，11 个骨架单元：

| 骨架单元 | 恢复模块 | 内容 |
|---|---|---|
| `systems/render_loop/painterly/mod.rs` | `src/systems/render_loop/painterly.rs` | 素描/粉彩渐进笔触链（acquire→convolve→generate_strokes→raster_strokes） |
| `systems/render_loop/vermeer.rs` | `src/systems/render_loop/vermeer.rs` | 维米尔风格揭示链（GenerateStrokes→raster_strokes→combine） |
| `systems/render_loop/menu_background.rs` | `src/systems/render_loop/menu_background.rs` | 菜单背景 blit（`blit_stretch_decode_srgb`） |
| `systems/render_loop/session_selection/mod.rs` | `src/systems/render_loop/session_selection/mod.rs` | 会话选择缩略图池 + 预览绘制 |
| `systems/render_loop/lut_gallery.rs` | `src/systems/render_loop/lut_gallery.rs` | 摄影模式 LUT 画廊 |
| `systems/render_loop/indirect_params.rs` | `src/systems/render_loop/indirect_params.rs` | 间接参数 SSBO 类型标记（6 型） |
| `systems/render_loop/resources/critters.rs` | `src/systems/render_loop/resources/critters.rs` | 小动物 SSBO 类型标记（4 型） |
| `resources/gpu_lut.rs` | `src/resources/gpu_lut.rs` | GPU LUT 生成器（GpuLutTracker/GpuLutApi） |
| `resources/grading_lut.rs` | `src/resources/grading_lut.rs` | 分级 LUT（.cube 解析 / 展平 PNG → 3D 纹理） |
| `resources/loading_screen.rs` | `src/resources/loading_screen.rs` | 加载屏启动/更新 |
| `resources/logo_animation.rs` | `src/resources/logo_animation.rs` | 5 阶段 Logo 动画 |

## 方法

业务逻辑→整理→骨架实现（项目工作流；差分仅用于裁决关键数值歧义，不实现）。
每模块：Ghidra 只读反编译 + 调用链 + PDB 骨架证据 → 契约（O/I/U + typed gap）→
骨架对齐 Rust。**未修改共享数据库**（只读 shared worker）。

## 证据

- `scripts/result-wp19-v2.json`：Ghidra `get_functions` 39 个函数反编译（2 批）。
- `analysis/decomp/*.c`：逐函数反编译。
- `scripts/job-strings.json`：数据段 shader 名读取（O 字符串：
  `session_selection_background`/`_new_glade_thumbnail`/`_thumbnail`/`_thumbnail_painterly`、
  `blit_stretch_decode_srgb`）。
- `render_session_selection_previews`（11245B）Ghidra/ReVa 反编译超时，复用
  `evidence/render-recovery/producer-closure/g7-post-ui/decomp/0x1408F6C00.c` 原反编译
  + callee 清单 + PDB inline 宿主复述。
- `scripts/all-bundles.txt`：11 骨架 bundle 摘要（summarize_all.py）。

## 关键观察（O）

- **vermeer 绘制链** `vermeer_render_from_target` 0x1408a8d70：`ComputePass::new<GenerateStrokes>`
  （URI `/vermeer/generate_strokes.cs.hlsl:cs_main`）写 strokes SSBO → `vermeer_raster_strokes`+
  `vermeer_combine` 光栅；笔触总量 0x8000/0xa00（O）。
- **painterly 链** `painterly_pass` 0x1408a6410：acquire→convolve 对（“Acquiring painterly
  target”日志）→ 每帧按 `floor(progress)` 递增区间 dispatch generate_strokes；progress 0..1024（f64）。
- **揭示完成**：progress≥1.0 → 释放主相机 + state=2 + `WriteSettingsFileCmd` + “Game reveal complete”。
- **LUT 资源**：`ColorGradingState::flattened`（final_post/prepare_frame_constants 消费）、
  `GradingLut::load_cube`（完整 .cube 解析→RGB9E5→3D）、`load_unwrapped`（PNG→RGBA8→3D）。
- **VermeerState 布局**（O 偏移）：frame@+0x00、progress@+0x40、timer@+0x44、state@+0x48、
  handle@+0x4c、camera_released@+0x3e、hold@+0x54。

## 编译门禁

```sh
cd reconstruction/country-core-render/wp19
cargo check --offline          # 0 错误 / 0 警告
cargo clippy --offline --all-targets  # 干净
```

独立离线 crate（无外部依赖）；共享类型经 `crate::consume`（Gap 占位）引用，不重定义。

## 集成说明（owner）

1. 在共享 `reconstruction/country-core/src/systems/render_loop/mod.rs` 声明
   `painterly`/`vermeer`/`menu_background`/`session_selection`/`lut_gallery`/
   `indirect_params`/`resources` 子模块；在 `src/resources/mod.rs` 声明 `gpu_lut`/
   `grading_lut`/`loading_screen`/`logo_animation`。
2. 把 `crate::consume` 占位替换为真实 wp1/wp2/wp4/wp15/wp13 类型。
3. `LoadShaderIntoLibrary`/`load_mesh_into_library` 起点在 `startup` 域；`blur_pyramid`、
   `rg_buffer`、`pass_resource_access` 等消费 wp1/w2 已恢复表面。

## 状态

全部 `implemented_unverified`（无 runtime/GPU 验收）。契约是当前可审阅依据；
反编译失败函数（`render_session_selection_previews`）以 callee/inline 宿主证据复述并如实登记 U。
