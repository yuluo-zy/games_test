# country-core

游戏侧渲染声明、资产注册和已经恢复的 CPU 领域规则。目录按 B 版原功能树组织，正常构建直接编译这些实现；完整 GPU 桥、游戏 ECS 场景和未知业务不靠占位算法补齐。

此前正式工程这里只保留了墙高、屋顶等 8 个局部文件，确实漏掉了 Git 中后续提交的渲染恢复。此次按 Git 记录重新整理了已实现函数及其必要输入/后端接口，没有直接搬入旧的整树骨架。

## 功能与代码位置

| 正式模块 | 概述及已恢复功能 | 仍需完成 |
|---|---|---|
| `src/render/pass_builder/` | Shader URI、反射绑定查找、资源读写访问声明、uniform、compute dispatch、光栅附件、绘制包及 draw builder 状态/快照规则 | 与真实 rhapsody graph/recording/descriptor 的统一适配；全部原入口 ABI 和整链行为验收 |
| `src/render/` | Typed/Mapped/Instance/Shadowed/Sparse buffer 的容量、上传区间、脏范围及稀疏分配；Mesh/VAO、纹理包解析、shader 程序缓存、view/frame/灯光常量 | GPU 对象生命周期接线、实际上传/管线创建；部分描述字段语义仍只有调用点合同 |
| `src/render/nani/` | 27 个真实子集、实例 ID/稠密索引与数据更新、mesh atlas、meshlet 切块/属性打包、包围球、间接绘制及 cull shader 描述；`instance_updates.rs`：各 `nani_update_*` 系统的实例载荷构建函数 `build_*`（plant/solid_vertex_color/albedo_map/window_glass/bricks/wren/gothic/flower/curved_halftimber/stone_floor/lantern/plaster/cloth/smoke/balcony_door/uber/velocity_only/stairs_floor）及打包辅助、50 项测试 | `nani/systems.rs` 的 ECS 包装层（`nani_update_*`、`nani_maintain_instances`、`nani_flush_instance_updates`）——等 `NaniActiveInstance`/`NaniInstanceDesc`/`Wren`/`Plaster` 等组件恢复后接上；证据见 [R2_nani_instance_updates.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R2_nani_instance_updates.md)；原整帧验证 |
| `src/asset_libraries/` | Mesh/VAO/shader/SSBO/texture 的名字与类型索引、注册/重载策略、颜色槽、placement data、audio trigger collection；文件监听策略 | 完整资产启动生产者、网格 JSON 加载 schema；原 frozen 本地库来源与实现身份 |
| `src/systems/render_loop/compute_passes/` | 地形/水/建筑栅格、草地/植被、光照/GI、HZB/mips、TAA/TGSR、DoF、读取及间接绘制等 pass 的参数、绑定、资源尺寸、顺序和 dispatch 规则；`DofAutofocus`/`DofFocusReadback`/`ShadowMaskHistogram`/`ShadowMaskMean` 四个 SSBO 的真实元素布局（HLSL + `new_storage_buffer` 汇编核对，`BUFFER_LEN>0` 的 `new_storage_buffer` 形态） | shader_refactoring 状态 6–9、reset_indirect_args 临时区等源合同中的未恢复分支；真实 GPU backend |
| `src/systems/render_loop/draw_passes/` | 主、阴影、透明、水/冰、UI 等 raster pass，draw shader 绑定及已恢复 draw callbacks | 不完整游戏资源的正式类型生产者；各 `*View` 目前是显式输入视图，不能称完整原资源 ABI |
| `src/systems/render_loop/` | `render_world` / `render_frame` CPU 帧编排、Painterly/Vermeer、预览/LUT gallery/session selection 的参数和分支策略 | `FramePasses` 到真实各 pass/backend 的消费桥；LUT 文件加载与全游戏场景 |
| `src/resources/` | 已验证墙高规则、屋顶/窗口局部规则，渲染资源布局/默认值、会话主题谓词及明确的渲染输入视图 | 完整墙/存档/会话资源图、组件身份和未知字段 |
| `src/resources/grading_lut.rs`、`grading_lut_cube.rs` | 调色 LUT 的 `.cube` 加载：逐行解析（TITLE/DOMAIN_MIN·MAX/LUT_1D_SIZE 拒绝/LUT_3D_SIZE/数据行，含原错误文案与行号上下文）、RGB9E5 打包（`utils::rgb9e5`）、`TextureDesc::new_3d(..,0x7b).sampled()` 建 3D 纹理 | `load_unwrapped`（PNG）、`GradingLuts::load`（闭包 0x140ae5bb0 未反编译）、`ColorGradingState::flattened`；`TextureDesc` 未建模维度码 `+0x1e`；证据见 [R1_grading_lut.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R1_grading_lut.md) |
| `src/utils/dirty_tiles.rs`、`src/resources/{world_raster,world_raster_defs,brush_sizes,active_raster_stroke,terrain_height}.rs` | 世界栅格与地形的 CPU 侧：统一的 64×64 `DirtyTiles`（建筑碰撞的 `DirtyTiles64` 现为其别名）；`WorldRaster<512>` 的双缓冲、`rasterize_stroke`（zeno，三种混合）、瓦片哈希（ahash 回退，全零瓦片 = 初始状态字 `0x07954dbb`）、`copy_to_staging` 与脏位图发布；6 个命名栅格和 `AllWorldRasterWrite`；`BrushSizes`；6 槽 `ActiveRasterStroke`（`add_point` 状态机、固化、10 遍平滑）；`TerrainHeightsData` 的双线性采样、`copy_from_gpu` 与 32 步 + 7 次二分的 `intersect_ray`。逐项见 [R5](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R5_world_raster_terrain.md) | `rasterize_world_space_stroke`（原版自有的 conservative-line-raster crate，尚未进工作区）、存档 serde、`+0x29b` 置位点、`cell_from_world_space`、`draw_raster` 状态机、`mouse_terrain_raycast`、`TerrainRockMask`；渲染 pass 里的栅格旗标替身尚未接线到本类型 |
| `src/utils/{project_cursor_onto_ray,load_json,geo,debug_viz,child_wall}.rs`、`src/utils/mod.rs` | 屏幕空间换算（`ProportionalScreenSpace`、`project_onto_ray`、`world_space_to_screen_space`）、JSON 网格加载、`mesh_from_pts` / `update_curve_mesh` / `rewire_enclosed_curve`、`debug_viz` 三类构造器、`ChildWall::slice`。逐项见 [R6](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R6_utils_screen_space.md)、[R7](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R7_utils_remainder.md) | `cell_from_world_space`；`get_last_event`/`drain_events`（bevy 资源访问，归 ECS 层）、`audio_utils`（归音频）、`debug_viz::*::show`（`Commands` 闭包） |
| `src/resources/gothic_window_profile.rs` | Gothic 半老虎窗/门的两组完整轮廓函数；6 个变体共 41 个点逐 f32 位核对现有原码常量 | Rust 参数宽度、原 panic 文案和完整开口消费者 ABI 未由这些常量测试证明 |
| `src/systems/camera_rig/` | B `default_vertical_fov_degrees` 的已闭合 45 度默认值叶函数 | 完整相机控制器、模式与正式场景接线；没有搬入旧停止恢复目录 |
| [`src/recovered_building/`](src/recovered_building/README.md) | 早期原指令/状态证据约束的建筑、墙、门窗、历史和生命周期 CPU 实现 | 详见子目录 README；这些是已解析输入的 CPU 投影，与渲染侧类型不是已证明相同的原 ABI |
| [`src/recovered_systems/`](src/recovered_systems/README.md) | 树删除条件、水栅格闭包、动物 locomotion predicate、墙音频 kind 表/音乐边界、clutter 删除 schema 的已闭合 CPU 子函数 | 各系统完整状态机、真实查询/资源、音频和 replay 消费；详见子目录来源与范围 |
| `src/utils/` | 稳定 ID 压缩和世界栅格采样已委托共享 `utils`；本地仅保留消费接口适配及自有 Transform | [索引/采样原码验收](../../../reverse-engineering/tiny-glade/evidence/utils-gap-recovery-2026-10-14/REPORT.md)；Transform 未恢复的 `look_at`/`abs_diff_eq` 未纳入 |

`recovered_building/nani_solid_vertex_color.rs` 是早期已解析输入的 CPU 数据编码规则；`render/nani/` 管理实际渲染声明和实例集合。二者角色不同，并存不表示原内存布局、生产者链或 ECS 类型身份已经统一。

## 逆向来源

- Git `664f6a65`：`feat(country-core): 全量恢复 21 工作包并整合为可编译 crate`。其大部分是路径/类型骨架，不能整体当成已恢复功能。
- Git `b5965593`：`feat(country-core): 渲染模块 A1–A4/N/D/C/S/H 按原机器码恢复，附统一测试台与审计工具`。此次渲染实现整理的主要依据。
- 实现来源：[reconstruction/country-core/src/](../../../reverse-engineering/tiny-glade/reconstruction/country-core/src/)。原模块树与每个函数的 B 地址、O/I/U 注释保留在源码中。
- 原始取证及合同：[reconstruction/country-core-render/](../../../reverse-engineering/tiny-glade/reconstruction/country-core-render/) 下 `wp*/contracts`、原机器码/反编译材料。
- 独立审计：[AUDIT_REPORT.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/AUDIT_REPORT.md)、[L0_NOTES.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/L0_NOTES.md)、[B_SUMMARY.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/B_SUMMARY.md) 和同目录 A3/B 分域报告。
- 旧测试挂载清单：[l0-harness](../../../reverse-engineering/tiny-glade/tools/country-core-audit/l0-harness/)。这里只用它核对遗漏的真实源码和已有测试；没有引入 `parking_lot_shim`、`slotmap_shim`、`half_shim`。
- 逐文件映射、原输入哈希和工程适配记录见 [render-source-map.json](render-source-map.json)。建筑来源单列于 `src/recovered_building/README.md`。
- 对旧 `country-core/src` 的 595 份源码另作覆盖清单，逐文件裁决了 41 份未映射且无显式 Gap/TODO 的候选，见 [country-source-coverage.json](country-source-coverage.json)。该扫描仍区分空体、写死示例、仅数据/容器转发、停止候选和实际已恢复函数；含显式缺口文件并未因此被整体宣布无可用函数。
- 相机叶来源：`reconstruction/camera/schema.rs`（Git `8107b475`）；原码证据 `evidence/parallel-camera/B-decompile/140911e30.c`。

## 纳入与排除边界

函数已经恢复，而其 GPU 消费方尚未接通，不是删除该函数的理由。因此 pass 声明/CPU 算法正常纳入，后端用 `GraphicsDevice`、`ShaderBackend`、`TextureDevice`、`RgResources`、`PassHost`、`FramePasses` 等明确接口提供。

`rhapsody_contract` / `rhapsody_types` 描述当前游戏侧 API 边界，不能当成 rhapsody 库实现。生产路径中的未连接操作明确报错或 panic；不会返回虚构管线、零页映射或成功空上传。CPU 记录型图/设备只在 `cfg(test)` 中存在。动态 staging 和 buffer import/export 需要调用方提供后端。真实 rhapsody 已恢复实现在相邻包，完整桥仍待开发。

`IvyDrawView` 和 `LeafParticleCountView` 仅承载已观察的 draw 输入；没有拷贝旧 IvyStorage 的猜测增长器或 GladeSettings 的空加载方法。后者保留原 callback 的 `u64` 输入，避免旧 `u32` 字段适配截断。`FrozenMap` 暂为稳定引用容器的工程适配，不计作原 frozen 库恢复；其上游身份和完整语义仍待核实。

未纳入 grading LUT 中仍是 stub 的部分（`.cube` 加载已纳入，见上表）、旧 loading/logo 启动骨架、全游戏 nani ECS 系统、未确认的 bucket/marker/世界栅格类型及整树 `Gap` 资源。原二进制本身的 `TODO(vulkan)` panic 和不支持格式的 panic 是已观察行为，保留并在对应函数注释中说明。

追加覆盖核查发现 Gothic 门曲线旧转录存在 4 个错误：`-0.148 → -1.48`、`1.58 → 1.57`、`-1.58 → -1.57`、`-0.6 → -1.65`。正式函数已按 `wp21/evidence/g8-misc-2.result.json` 的原 f32/qword 常量纠正，六组完整点表使用独立原常量 fixture 逐位验证。旧 `delete_wall` 的两张 Entity map 所有权存在合同/保存反编译冲突，未猜测双 despawn 算法；原因登记于覆盖清单。

公开依赖直接使用原 B 版本，包括 Bevy ECS 0.16.0、glam 0.29.3、half 2.4.1、bytemuck 1.16.0、slotmap 1.0.6、nohash-hasher 0.2.0、relative-path 1.9.0 和原 hotwatch Git commit。没有迁入旧手写 NoHashHasher / RelativePathBuf 代替实现；生产 watcher 直接收到原 notify 事件，测试注入器只在 `cfg(test)` 中使用。Bevy 分叉仓库/commit 仍未知，使用这些 API 不等于已恢复引擎分叉。

## 验证与开发

在 workspace 根执行：

```powershell
cargo check -p country-core --locked --offline
cargo test -p country-core --locked --offline
cargo clippy -p country-core --all-targets --locked --offline -- -D warnings
```

本轮已以真实依赖编译；启用 `native-physics` 的 1142 项库内测试全部通过，默认及 `native-physics` 的全部目标严格 Clippy 通过。已有机器码/O/I/U 和隔离测试证据的范围仍然各自有效；测试通过不能替代原二进制整链、GPU 输出或完整游戏验收。新增/迁移建筑测试的当前数目以 workspace 验证结果为准。

恢复源码保留机器码的 NaN 比较、max/min、运算顺序和调用合同。准确的文件级 lint 允许项登记于 [lint-preservation.json](lint-preservation.json)：numeric 项保留 `max/min` 的 NaN 行为、比较方向、浮点常量和操作数顺序；API 项保留原调用形状及缓冲范围；style/fixture 项保留取证文档和测试表达。没有允许整个 Clippy，也没有通过更改数值算法来消除报告。

默认与全部目标的严格 Clippy 已通过。`native-physics` 是旧建筑 CPU 实现到原 Rapier 依赖的可选适配，不是物理引擎重写；对应状态见建筑子目录和 workspace 验证结果。
