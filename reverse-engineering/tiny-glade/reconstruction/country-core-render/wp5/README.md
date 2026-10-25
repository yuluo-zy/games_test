# WP5 — country-core render_loop 帧 passes 交付总结（B 版本）

日期：2026-10-13。目标 **B**（`tiny-glade.exe` sha256 cf048e27…；PDB `tiny_glade.pdb` 2d9686b8…）。
范围：`country_core::systems::render_loop` 全部帧 pass 的 **CPU 侧组合**（构造哪个 ComputePass/DrawBuilder、用哪个 ShaderUri、
声明哪些 typed buffer 读/写、dispatch/draw 模式、W# 帧序与分支条件）。

本包是**业务契约 + 骨架对齐 Rust**，不是作者原文；rhapsody / bevy_ecs / glam / ash / enum-iterator / itertools 为复用依赖，不重实现。
**不重新定义兄弟包的类型**——全部以 `consume::` 契约模块 import 式引用。

## 交付内容

| 目录 | 内容 |
|---|---|
| `contracts/frame-wiring.md` | 帧接线（R1–R20、W1–W54、8 分支、生产/消费、typed gap） |
| `contracts/compute-passes.md` | 44 个 compute pass 模块逐 pass 契约（URI、资源读写、dispatch 模式、帧序） |
| `contracts/draw-passes.md` | 16 个 draw pass 模块逐 pass 契约（RasterPass/DrawPacket/管线设置、URI） |
| `src/…` | 骨架对齐 Rust（`systems/render_loop/{compute_passes,draw_passes}` 全模块树 + `consume::` 契约 + `resources::` typed gap） |
| `evidence.json` | W1–W54 地址→PDB 名全表、bundle SHA、解码注记、gap |
| `scripts/` | bundle 解析辅助（`parse_bundle.py` / `summarize_bundle.py`） |

## 覆盖

- **W1–W54 全部 54 个帧 pass + R2（DrawLists build/sort）**：每个 pass 的入口函数写成 Rust 骨架，标注 O/I/U；
  内部 body 多为 `unimplemented!()`（业务逻辑已文档化在 contracts，非空猜）。
- **深度恢复**：`gpu_generate_draw_lists.rs`（W22/W28）— 0x140987dd0.c 全链解码 + 14 个子函数 + pdb.jsonl 行号锚；
  `cull_and_bucket` 的 `enum_iterator::All<GpuWorldSubsetKind>` filter_map + multiunzip 结构（O）；round 分支与
  `AssetSsboLibrary::get_rg_buffer`/`clear_rg_buffer`/`dispatch_with`/`dispatch_indirect_with` 接线（O）。
- **未覆盖**：`draw_shaders/*`（每材质 shader body）记为命名边界；`resources::` 游戏状态（水/地形/屋顶/草 SSBO 字段）为 typed gap。
- `cargo check --offline` 通过（0 error / 0 warning；依赖无关，原依赖以注释记录为复用边界）。

## 消费的兄弟包共享类型（consume 面）

- wp1 `pass_builder`：ComputePass / DrawBuilder / RasterPass / DrawPacket / ShaderUri / UniformBinder / PassResources / AsComputeShaderSource / ComputeDispatchApi / ComputeDispatchIndirectApi。
- wp2 `render`：TypedStorageBuffer / InstanceBuffer / ShadowedBuffer / Draw(Indexed)IndirectBuffer / DispatchIndirectBuffer / AtomicCounterBuffer。
- wp3 `render::nani`：GpuWorld / OwnedGpuWorldSubset / GpuWorldSubsetKind / NaniDrawList*Ssbo / NaniMeshAtlasPartInfo / NaniDrawListInfo / NaniDrawListParams / cull_shaders（NaniCullAndBucket*）。
- wp4 `render`/`asset_libraries`：Mesh / ViewConstants / FrameConstants / DirectionalLight / SunSkyColors / ColorLibrarySsbo / LanternSsbo / SpatialResolveOffsetsSsbo / ShadowMask(Histogram|Mean)Ssbo / DofAutofocusSsbo / MouseDepthValueSsbo / RtTrianglesSsbo / RtTriangleMetaSsbo / RtBvhSsbo / WallColorIdSsbo / Asset{Shader,Ssbo,Mesh,Texture}Library。

## 接线主张

- `DrawLists::new/build/sort`（0x1409ba340/540/4b0）被 `tiny_glade::render_loop::render`（R2）消费；各 draw pass
  （main_pass/shadow_pass/transparency_pass/deferred_raster/ui_2d_pass）遍历 `DrawListItem → to_draw_packet` 绘制。
- `gpu_generate_draw_lists`（0x140987dd0）第 1 轮（W22，无 hzb）→ shadow/main → `generate_hzb` → 条件第 2 轮
  （W27/W28，带 hzb，`\*0x14362c0d8==0`）→ grass/main_pass_oc2 → …；是 nani GPU-driven 实例化链（W25/W30/W34/W46/W53 消费 WP3 nani_draw 间接绘制）。
- DoF/TAA 后处理链（W49/W50/W51/W52）受 param_6 字节门控；TAA↔TGSR 互斥；水/冰按 is_winter 二选一（consumed from sequence.md + 0x1404ec110.c）。

## 剩余 gap（详见表单 evidence.json + contracts 各节）

1. 全局 \*0x14362c0d8 / \*0x14362c098、param_6 字节、settings+0xac 语义：U。
2. 每 pass 绑定的确切 shader 变体（两 16-hex 变体对）；dispatch/draw 模式判别→Rust 枚举名：U。
3. `resources::` 游戏状态与各 `*Ssbo` 字段布局：U（不臆造）。
4. `DrawLists/DrawData/DrawListItem` 字段与 draw_shaders body：U。
5. W54 render_shader_refactoring 作用：U。
6. 编译验收仅 `cargo check`；需与 rhapsody-rebuild + wp1–wp4 联调后 `cargo check` 完整工程（同 wp1/wp2 交付口径）。
