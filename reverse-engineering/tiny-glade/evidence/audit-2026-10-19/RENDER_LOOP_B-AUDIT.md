# render_loop（B）逐文件复核 — 除 draw_passes / compute_passes 外的 11 个文件

审计日期：2026-10-19
范围：`tiny-glade/crates/country-core/src/systems/render_loop/` 下
`indirect_params.rs`、`lut_gallery.rs`、`menu_background.rs`、`mod.rs`、`painterly.rs`、
`render_frame.rs`、`render_world.rs`、`resources/{critters.rs, mod.rs}`、
`session_selection/mod.rs`、`vermeer.rs`。
性质：只读审计；未修改任何源码。基线 `recovery/STUBS-2026-10-19.md` 仅作参考，按下述证据重新判定。

数据口径：
- 行数 / 测试数：直接 `wc -l` 与 `#[test]` 计数；`prod~` = `#[cfg(test)]` 之前部分。
- 骨架对照：`skeleton/B/.../render_loop` 的 `// 函数符号:` 记录 + `evidence/audit-2026-10-19/fn_audit.json`。
- 接线判定：全 workspace `crates/` 内 `grep` 调用点/实现点；只在 tests 中出现不算生产接线。

---

## 1. 文件级状态表

| 文件（相对 `systems/render_loop/`） | 行数 | prod/test | 文件性质 | 真实实现 | 空壳/占位 | fn_audit 行数 | 生产接线 |
|---|---:|---:|---|---|---|---:|---|
| `mod.rs` | 264 | 66 / 198 | 混合：RenderContext + clear/copy 助手 + 测试 | 助手逻辑真实 | 见 §2 | 4 in_file | 否。仅 `lib.rs:31` 再导出；`RenderContext` 无生产使用者 |
| `indirect_params.rs` | 224 | 116 / 108 | 纯数据/类型 + `create_indirect` 构造器（宏生成 4 类型） | 是 | 无 | 无行（骨架 functions=0） | 是（`startup/init_populate_ssbo_library.rs:85-92`、`reset_indirect_args`） |
| `lut_gallery.rs` | 474 | 219 / 255 | 混合：状态 + 完整绘制逻辑 + host 缝 + 测试 | 绘制逻辑真实 | `build` 仅返回 default；`startup` 无函数 | 2 in_file | 否。`LutGalleryHost` 仅测试实现 |
| `menu_background.rs` | 371 | 170 / 201 | 混合：状态 + 模式门控 + 绘制逻辑 | 门控/绘制真实 | `build` 仅工厂返回 | 2 in_file | 否。`MenuBackgroundHost` 仅测试实现 |
| `painterly.rs` | 1174 | 670 / 504 | 混合：描述符/状态机/完整 pass 链 | 是（流程完整） | 无 `todo`；plugin 有 app 缝 | 对应骨架 painterly/mod.rs 4 moved | 否。`painterly_pass`、`startup_painterly` 无生产调用（`EffectsHost` 实现了 `PassHost` 但未被用于 painterly） |
| `render_frame.rs` | 873 | 372 / 501 | 真实行为：整帧编排（host 缝） | 是 | 无 | 无行（B 属 tiny_glade crate） | 是（`game_loop.rs:139`），但 world 图编译返回 Err（见 §6） |
| `render_world.rs` | 1451 | 645 / 806 | 真实行为：W1–W54 次序 + 63 方法 `FramePasses` 缝 | 是 | 无 | 无行（B 属 tiny_glade crate） | 否。仅测试调用；无生产 `FramePasses` 实现 |
| `resources/critters.rs` | 296 | 189 / 107 | 纯数据/类型（4 个 SSBO + 构造器） | 是 | 无 | 无行（骨架 functions=0） | 是（`init_populate_ssbo_library.rs:68-73`、draw/compute pass 引用） |
| `resources/mod.rs` | 1 | 1 / 0 | 仅声明 `pub mod critters;` | — | — | 无行 | 编译期声明 |
| `session_selection/mod.rs` | 1252 | 637 / 615 | 混合：状态 + 缩略图算法 + 预览绘制 + host 缝 | 算法/绘制真实 | `build` 仅工厂返回；`panic!("TODO")` 是原版复刻 | 3 in_file | 否。`SessionPreviewHost`/`ThumbnailStartupHost` 仅测试实现 |
| `vermeer.rs` | 1341 | 636 / 705 | 混合：状态机 + 完整绘制链 + host 缝 | 是 | 无 | 8 in_file | 部分。`vermeer_render_from_target`（`loading_graph.rs:227`）、`startup_vermeer`（`production_renderer.rs:72`）、状态 helper（`run_game.rs:721-739,986`）已接；`vermeer_pass` 无生产调用 |

合计：7721 行（其中测试 4000 行，生产约 3721 行），140 个单元测试。

骨架布局差异（非缺失）：
- 骨架 `painterly/mod.rs` ↔ 恢复 `painterly.rs`（同名逻辑，moved）。
- 骨架没有 `render_frame.rs` / `render_world.rs`：这两个函数在 B PDB 中属于 **tiny_glade** crate（`render` 0x1404e8010、`render_world` 0x1404ec110），恢复时归位到 country-core；因此 `fn_audit.json` 不会为它们产生行，也不能按本表外的 `no_trace` 统计。
- 骨架没有 `resources/mod.rs`，恢复侧新增 1 行声明文件。

---

## 2. 空壳 / 占位 / 仅默认返回清单

扫描 `todo!`/`unimplemented!`/空体/仅日志/仅默认返回/“占位/待取证/待接线/未实现”：
本范围内 **没有** `todo!` 或 `unimplemented!`。`session_selection/mod.rs:507` 的 `panic!("TODO")` 是对原版 `panic!("TODO")`（源码行 135）的忠实复刻，不是恢复占位；测试 `mod.rs:1225` 也按 `should_panic(expected="TODO")` 验证。

仍属“签名壳/未接系统”的点：

| 位置 | 内容 | 判定 |
|---|---|---|
| `lut_gallery.rs:215-217` | `LutGalleryPlugin::build` 只 `LutGalleryState::default()`，不注册 `PreStartup startup` | 签名壳（原 build 是 `insert_resource + add_systems`） |
| `lut_gallery.rs:206-208` | 只有 `STARTUP_SHADERS` 常量；骨架 `lut_gallery::startup`（源码行 41，inline instances=6）没有对应可调用函数 | 真缺失（着色器加载函数） |
| `menu_background.rs:166-168` | `MenuBackgroundPlugin::build` 只返回 default | 语义近似（原 build 本身只 insert_resource），但无接线 |
| `session_selection/mod.rs:629-635` | `SessionThumbnailsPlugin::build` 只返回 default；不注册 `PreStartup thumbnails_startup` | 签名壳 |
| `mod.rs:105-124` | `RenderContext::allocate_dynamic` 在非 test 且未注入 allocator 时返回 `Unsupported` | 有意接缝；但 `RenderContext` 本身无生产使用者 |
| `indirect_params.rs:19-23` | 注释表明旧的 `(Gap)` 同名类型已改为 `pub use`（`resources/render_ssbo.rs:23`），已处理 | 非缺口 |

其余短函数（`has_target`、`mode_hides_selection`、`card_highlight`、`max_stroke_count`、`is_active`、`x86_min/max`、`floor_u64` 等）均为原函数的直接语义，不是占位。

---

## 3. 骨架同路径 `no_trace` 对照

`fn_audit.json` 在本范围的全部记录（23 条）：

| 文件 | 行数 | 分类 |
|---|---:|---|
| `mod.rs` | 4 | 全部 `in_file`（finish / clear_rg_buffer / rg_buffer_copy_from_slice<u32> / clear_rg_color_image） |
| `lut_gallery.rs` | 2 | 全部 `in_file`（build / render_photomode_lut_gallery） |
| `menu_background.rs` | 2 | 全部 `in_file`（build / render_menu_bg） |
| `painterly/mod.rs` | 4 | 全部 `moved`（has_target / build / painterly_pass / startup_painterly） |
| `session_selection/mod.rs` | 3 | 全部 `in_file`（build / thumbnails_startup / render_session_selection_previews） |
| `vermeer.rs` | 8 | 全部 `in_file` |
| `indirect_params.rs` / `resources/critters.rs` / `resources/mod.rs` / `render_frame.rs` / `render_world.rs` | 0 | 骨架该文件 functions=0，或函数属 tiny_glade crate |

**结论：fn_audit 在本范围 `no_trace = 0`。** 但 fn_audit 只覆盖 36 个非闭包骨架符号中的 23 个，不能据此宣称全覆盖。手工比对骨架 `// 函数符号:`（去闭包）36 项：

- **独立函数仍在：31 项**。其中 3 项从方法改为自由函数（`ThumbnailTextures::{extract_thumbnails_to_display,find_backed_thumbnail_range,assign_pool_indices}`），属搬移/签名调整，逻辑在。
- **合并进 pass 函数（逻辑在，符号消失）3 项**：
  - `painterly::raster_strokes`（骨架源码行 270）→ 内联于 `painterly_pass` 第 9 步（`painterly.rs:560-595` 的 raster 提交）。
  - `vermeer::raster_strokes`（骨架源码行 347）→ 内联于 `vermeer_render_from_target` 第 4 步（`vermeer.rs:449-486`）。
  - `vermeer::raster_combine`（骨架源码行 395）→ 内联于 `vermeer_render_from_target` 第 5 步（`vermeer.rs:487-526`）。
- **真缺失（无独立实现、也无可明确对应的合并体）2 项**：
  - `render_loop::clear_buffer`（骨架源码行 59，inline instances=4）。恢复侧只在 `mod.rs:14` 注释里提到“已移除”，`rg_helpers` 中也没有同名/等价 World→render-graph 包装层；`clear_rg_buffer` 是另一符号。**需人工判定**是否遗漏。
  - `lut_gallery::startup`（骨架源码行 41，inline instances=6）。恢复侧只有常量表，无加载调用。

另：`vermeer::VermeerState::activate`、`vermeer::max_stroke_count` 在恢复侧存在，但 fn_audit 未列（说明审计表本身不完整）。

---

## 4. 十个最大文件的关键函数抽查（真实实现 vs 签名壳）

1. `render_frame.rs:258 render` — **真实编排**。按机器码顺序实现：puffin scope → device/GameConfig 断言 → draw lists → 模式分支（loading vs world）→ 帧间隔 → begin/drain → execute_main → acquire/presentation/present → screenshot/stats/memory → GPU pacing 环 → scope end；`recompile_at_reduced_scale`（335）含 -0.1 步长、OOM 文案、`[32,32]` 兜底与原 panic 文案。非签名壳。
2. `render_world.rs:442 render_world` — **真实次序**。W1–W54 的 63 个 seam 调用、`world_render_dims` 断言、prev_depth temporal、gbuffer/velocity、HZB 分支 `flag_c0d8`、折射/水面/冰面 `flag_c098`、DoF/TAA/TGSR 分支、输出三件套均按注释地址实现；无 `todo`。属“逻辑真实、接线缺失”。
3. `painterly.rs:451 painterly_pass` — **真实实现**。instant 判定、阶段重置、extent `(w<<8)/h × 256`、两个 temporal、acquire/convolve 采集分支、4096 B 笔触 SSBO、progress 步进（1638.4/s、1024 封顶）、push constants `[start,count,timer]`、`6×floor(progress)` 顶点、timer；`acquire_and_convolve:598` 也是真实描述符序列。
4. `vermeer.rs:540 vermeer_pass` — **真实状态机**。Disabled/restart 重置、temporal acquire、dt clamp 1/30、`frame<2||hold` 不推进、`progress += dt/3`、`≥1.0` 时释放背景/Disabled/WriteSettingsFileCmd/Info 日志、返回 `orig_phase != Disabled`。`vermeer_render_from_target:366` 为真实三/五段绘制链。
5. `session_selection/mod.rs:491 render_session_selection_previews` — **真实绘制顺序**。模式门控 → 4 shader → 轮播锁（`mode_tag==1` 断言、`active` 原版 panic）→ sessions/pool → 窗口 → clear assigned/extract/assign/pool_index → refresh → 背景/新建卡/thumbnail 卡/painterly 卡 → render；`refresh_screenshot:434` 的 modified 比较、decode 分流、错误去重也是真实逻辑。
6. `lut_gallery.rs:153 render_photomode_lut_gallery` — **真实逻辑**：demo 短路、`EscMenuState::Photomode(true)` 门控、preview None 双分支、`zip(luts, positions)`、逐格 uniform 顺序、raster state 改写、每格 6 顶点。唯一的壳是 plugin/startup。
7. `menu_background.rs:70 menu_bg_mode_allows` — **真实位运算门控**（`0xaf/0x50` 移位表、`state==2/0xa/7` 特例）；`render_menu_bg:138` 的 take pending/changed tick/旧句柄释放/全屏三角形/绑定/state 清位均真实。
8. `mod.rs:141-164` 三个 RG 助手 — **真实但薄**：登记 pass 名/access/回调，`rg_buffer_copy_from_slice` 做容量前置检查后转 `rg_helpers`；`RenderContext` 是恢复用登记表，不是原对象完整布局。
9. `vermeer.rs:200-238` 状态 helper 与 `run_game.rs:986 into_disable_hold` — 真实且在 loading→gameplay 交接中实际调用。
10. `indirect_params.rs` / `resources/critters.rs` — 纯类型与构造器，字段/大小/NAME/默认值都与机器码与测试一致；已在 `init_populate_ssbo_library` 实际注册，不是壳。

---

## 5. 重点：session_selection、主循环、帧组织、rhapsody ExecutionChain

### 5.1 session_selection（预览/切换）
- 预览绘制与缩略图刷新逻辑真实且覆盖完整（见 §4.5），`panic!("TODO")` 是原版。
- **未接线**：`SessionPreviewHost`、`ThumbnailStartupHost` 在整个 workspace 只有测试实现；`render_session_selection_previews`/`thumbnails_startup` 在 `crates/country-core/src/systems/render_loop/` 之外零调用点。
- **系统注册缺失**：`SessionThumbnailsPlugin::build`（629-635）不产生 `add_systems(PreStartup, thumbnails_startup)`，因此即使 App 插入 `ThumbnailTextures` 也不会自动启动。
- **切换逻辑不在本文件**：本文件只有预览渲染与磁盘缩略图刷新；选卡/切换输入逻辑未在此范围出现。`CarouselState` 仍有 `unobserved_14/30/38` 明确标 U。

### 5.2 render_loop 主循环与帧组织
- `game_loop.rs:139` 每个 tick 真实调用 `render(&mut host, &mut pacer, mode)`；`frame_render_mode`（59-64）按 `MainCamera` 是否存在选 `WorldGpuProfiled` / `LoadingScreen`。
- `ProductionRenderHost`（`production_renderer.rs:259` 起）真实实现 `RenderHost`：draw lists、loading graph、execute、acquire、presentation、present、memory/stats 都落到 `CoreRenderBridge`/rhapsody/swapchain。
- **致命未闭环**：`compile_world_graph`（322-324）直接返回 `Err(world_consumer_missing())`；在 world 模式下 `render()` 会进入 `recompile_at_reduced_scale`（`render_frame.rs:335`），再次调用 `compile_world_graph(...).expect(FALLBACK_FAILED)`，因此 **gameplay 帧无法完成**（会 panic 在 `render_frame.rs:357`）。加载画面路径可跑。
- `warm_up_gameplay`（`production_renderer.rs:37-42`）同样固定返回 `unsupported("world FramePasses consumer is not connected")`，gameplay App 预热不可能成功；而更完整的 `warm_up_gameplay_with_world_builder`（`render_bridge.rs:887`）已存在但 **没有调用者**。
- 加载图路径真实：`loading_graph.rs:227` 调用 `vermeer_render_from_target`，`production_renderer.rs:72` 调用 `startup_vermeer`。
- 截图路径：`finish_screenshot_capture`（422 起）在有截图请求时主动报 `unsupported`（“request-to-GPU-texture association is not recovered”），属显式登记的未复现项。

### 5.3 rhapsody ExecutionChain 实际调用点
- **恢复代码没有直接调用 `ExecutionChain::new`**。`rhapsody_types.rs:388-391` 只是转出 `rhapsody::rg::executor::{ExecutionChain,...}`，workspace 内没有使用者（只有 `rhapsody` 自身与测试）。
- 真实执行路径是：`CoreRenderBridge::{execute_built, compile_built, execute_compiled}`（`render_bridge.rs:245-272`）→ `rhapsody::rg::vulkan::VulkanGraphSession::{execute_frame, compile_frame, execute_prepared}` → rhapsody 内部 `new_execution_chain`（`crates/rhapsody/src/rg/vulkan.rs:1259`）。这是依赖实现，按项目规则不复刻，属“间接真接”。
- `rhapsody_contract.rs:341` 的注释声称“见 `render_frame.rs` 的 ExecutionChain 接线”，但 `render_frame.rs` 全文不含 `ExecutionChain`；该注释是 **陈旧/不准确**，需更正（本轮不改源码）。
- 因此：`ExecutionChain` 的公开转发面是占位/未使用接口；真实提交链在 rhapsody session 内，已通过 `SessionFrameReceipt` 实际使用。

---

## 6. 仍未复现清单（带证据）

1. **gameplay world 图未接线**：`production_renderer.rs:322-324` `compile_world_graph` 返回 `Err(world_consumer_missing())`；`production_renderer.rs:45` 文案 `"original compile_main 0x140161cd0: world FramePasses consumer is not connected"`。结论：world 模式首帧即失败。
2. **gameplay 预热未接线**：`production_renderer.rs:37-42` `warm_up_gameplay` 固定 Err；`game_loop.rs:312` 正是调用它；更完整的 `render_bridge.rs:887 warm_up_gameplay_with_world_builder` 无调用者。
3. **`render_world` 的生产消费者不存在**：`render_world.rs:442` 的 `render_world` 仅被同文件测试调用；63 方法 `FramePasses` 只有测试 `Rec`（`render_world.rs:686`）实现；`scene_compute.rs:1-8` 自述“not a substitute 54-method FramePasses implementation”。
4. **存在平行实现但未挂到主循环**：`world_graph.rs:42 build_world` 已写出 W1–W54 的 ECS 接线版本，但 `compile_world_graph` 没有调用它（grep 显示只有 `render_bridge.rs` 的预热 seam 引用，且该 seam 无调用者）。
5. **session_selection 未接入**：`SessionPreviewHost`/`ThumbnailStartupHost` 仅测试实现；`render_session_selection_previews`/`thumbnails_startup` 零生产调用；`session_selection/mod.rs:629-635` build 不注册系统。
6. **lut_gallery startup 缺失**：骨架 `lut_gallery::startup`（源码行 41，instances=6）无对应函数；`lut_gallery.rs:206-208` 只有常量，`lut_gallery.rs:465-473` 的测试只断言常量表；`LutGalleryPlugin::build` 不注册系统。
7. **menu_background 未接入**：`render_menu_bg` 无生产调用；`MenuBackgroundHost` 仅测试实现；`menu_background.rs:166-168` build 仅工厂。
8. **painterly 链未接入**：`painterly_pass`/`startup_painterly` 无生产调用；App 未插入 `PainterlyState`（`run_game.rs` 只有 VermeerState）；`production_renderer.rs:72` 只加载 Vermeer 着色器，未加载 painterly 的两个着色器。
9. **vermeer 状态机未接入**：`vermeer_pass` 无生产调用（仅 tests）；`VermeerPlugin::build`（`vermeer.rs:270-286`）未被使用；`run_game.rs:721-739` 手工插入状态但未注册 `startup_vermeer`/OnExit `deactivate`。仅 `vermeer_render_from_target` 在加载图真实调用。
10. **`clear_buffer` 未显式复现**：骨架 `mod.rs` 源码行 59 / inline instances=4；恢复侧 `mod.rs:14` 仅注释“已移除”，`render/rg_helpers.rs` 无同名等价包装。**需人工**确认是否原版仅有 inline 包装。
11. **`ExecutionChain` 公示面未使用且文档陈旧**：`rhapsody_types.rs:388-391` 仅转发无使用者；`rhapsody_contract.rs:341` 错误指向 `render_frame.rs`。真实链在 rhapsody `rg/vulkan.rs:1259`。
12. **截图 GPU 纹理关联未复现**：`production_renderer.rs:422-429` 对活动截图请求返回 `unsupported`。

---

## 7. 完成度评估

| 维度 | 估计 | 依据 | 置信度 |
|---|---:|---|---|
| 源码逻辑（本范围文件内） | **94%** | 骨架 36 个非闭包符号：31 独立存在 + 3 合并进 pass + 2 无实现（`clear_buffer`、`lut_gallery::startup`）。140 个单测覆盖边界/常量/顺序。 | 中高（无 GPU 真机行为验证；测试是自证） |
| 生产接线 | **约 25%** | 8 个行为模块中：1 个完全接（`render_frame` 的外层/加载路径）、1 个部分接（vermeer，仅渲染链）、6 个未接；2 个数据模块（indirect/critters）已接；且 world 主路径被 `Err` 短路。按 10 个实质文件计 (2 全 + 1 半)/10 ≈ 25%。 | 高（grep 调用点 + 显式 `Err` 文本） |
| 综合（逻辑 0.7 / 接线 0.3） | **约 73%**（区间 68–77%） | 0.7×0.94 + 0.3×0.25 ≈ 0.733 | 中 |

说明：
- 本百分比 **只针对本 11 个文件**，不代表整个 render_loop（draw/compute passes 未审）。
- “逻辑存在”不等于“语义验收”：没有 GPU 真机、没有与原 B 资源/帧输出对齐；`STUBS-2026-10-19.md` 已声明 `semantic_acceptance: false`。
- 若把“合并进 pass”视为未独立复现，则源码逻辑严格口径为 **86%**（31/36）。

---

## 8. 需人工判定 / 后续复核

1. `clear_buffer`（骨架源码行 59）是否确有独立源级函数体；若只是 inline 资源查找包装，应在审计中明确“合并”，否则补齐。
2. `lut_gallery::startup` 的缺失应补一个可调用函数（目前仅有常量表），并决定是否注册 `PreStartup`。
3. `compile_world_graph` 与 `world_graph::build_world` 的接线应由集成 owner 裁决：二选一（挂接 `render_world` 泛型实现，或直接把 `build_world` 接进 host），不能长期保留双实现且都不运行。
4. `world_graph::build_world` 与 `render_world.rs` 的顺序/分支是否逐条等价，尚未做差分；两条实现并存有漂移风险。
5. `ExecutionChain` 转发面与 `rhapsody_contract.rs:341` 注释需要更新，避免后续把“公开转发类型”误认为已接线。
6. `session_selection` 的选卡/切换输入逻辑不在本文件，需在 UI/mode 相关模块另行追踪。
