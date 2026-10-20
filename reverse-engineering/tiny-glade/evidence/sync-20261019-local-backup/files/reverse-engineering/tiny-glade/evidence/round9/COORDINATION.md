# 第 9 轮协调台账（2026-10-19）

任务：按 2026-10-19 逐文件复核的缺口，继续逆向 + 实现 + 复核。
结构：2 组 × 3 agent（①逆向契约 / ②实现 / ③review）。

## G1 存档/读档主链

文件权（②写）：`systems/save/load_game.rs`、`systems/save/snapshot.rs`、必要时 `resources/history/snapshots.rs`；新增仅限 `systems/save/`。

禁止：rhapsody、roofs/geometry、systems/roof、systems/wall、resources/walls、systems/render_loop、tiny-glade（G2/G3 或用户区）。

已知缺口：
1. `load_game.rs:267` 对有效 `.snapshot` 直接 bail。
2. `init_game_state_from_snapshot` 无生产调用者。
3. apply 仅 4/9 域（Walls/Decorators/Terrain/Raster）；Roofs/WallColors/GateColors/Trees 仅 log。
4. Walls 仅 id+height；0x70B 曲线区/枚举未打包。
5. Decorators 0x28B 逐槽/0x16B/掩码未定。
6. 快照 GC 无生产调用者。

产物：
- `contracts/g1-snapshot-apply-CONTRACT.md`
- `deliveries/g1-snapshot-apply-DELIVERY.md`
- `reviews/g1-review.md`

## G2 渲染构图/执行链

文件权（②写）：`systems/render_loop/**`、`render/execution_host.rs`、`tiny-glade/src/production_renderer.rs` 的 `compile_world_graph` 及必要 helper。

禁止：rhapsody 本体、roof、wall、save。

已知缺口：
1. `production_renderer.rs:322` 的 `compile_world_graph` 固定返回 `Err(world_consumer_missing())`。
2. `render_world.rs:219 trait FramePasses` 无生产实现；`render_world` 63 个 seam 只有测试 host。
3. `painterly.rs:276 PassHost`、`pass_kit.rs:153 PassFrame/ComputeExec` 同样只有测试实现。
4. `render/execution_host.rs` 无调用者；真实执行在宿主 `render_bridge.rs` 的 `VulkanGraphSession`。
5. 缺 `clear_buffer`、`lut_gallery::startup`、`water_level_at_xz`、`blur_exclusion_mask`、`logo_overlay::startup`。

产物：
- `contracts/g2-world-frame-CONTRACT.md`
- `deliveries/g2-world-frame-DELIVERY.md`
- `reviews/g2-review.md`

## 中央裁决规则

- 两组的②不得互改对方文件；跨界需求先报中央。
- 验收门：真实调用链 + 真实消费者 + 原证据；禁止静默 no-op、测试 double 冒充生产、log 冒充写入。
- 既有阻塞（不属本轮）：`systems/save/save_thread.rs:63` 的 `UnsavedFileEntry == 0x80` 在 Windows 上因 `PathBuf` 32B 失败（实际 0x88）。②不得为此改该文件；③在报告中注明。
- 目录只读基线：`recovery/STUBS-2026-10-19.md`、`evidence/audit-2026-10-19/*-AUDIT.md`。

## 进度与裁决记录（2026-10-19）

### G1① 契约完成
`contracts/g1-snapshot-apply-CONTRACT.md`。关键更正：
1. 原版 init 经静态系统表间接调度；快照可打开即权威，不得回退。
2. Walls 0x70 是含 Vec 头的内存 WallState，点列堆外，不存在内联打包；样本 2..104 点、1870 点全 [x,0,z]。
3. Roofs/三色表/Trees schema 与目标资源已定；TreeEditData 缺 serde default（50/590 条缺字段）。
4. Decorators 内层 0x58（非 0x16B），seed 写 +0x2c。
5. 0x1429cd080/090 是 aHash 32B 固定种子，**不是 ID XOR 掩码**；旧 rotate^mask 语义已判错并通知删除。

### G1② 实现（进行中，已见真实接线）
- `load_game.rs` 的 `bail!` 已删除；`load_closest_valid_snapshot` → `load_snapshot_file` → `init_game_state_from_snapshot(world, &resources)`，失败为硬错误。
- `snapshot.rs` 新增 Trees/Roofs/三色表解析与 World 级 apply；报告 `gaps` 并区分“部分应用”。
- 仍需 G1③ 核实：所有域是否真写回、旧掩码/0x16B/内联曲线是否彻底清除、编译与测试。

### G2① 契约完成
`contracts/g2-world-frame-CONTRACT.md`。关键结论：世界帧真实消费者已存在 `tiny-glade/src/world_graph.rs:42 build_world`（W1–W54）；`compile_world_graph` 只是没接线。最小帧 = 新建 "main color"(0x7a,mip6) → clear(access 0x34) → export 0x22。

### G2② 实现（已交付，复核中）
- `production_renderer.rs` 的 `compile_world_graph` 不再固定 Err；producer 齐全走 `build_world`，缺失走 minimal clear+export（已记 warn）。
- `world_frame.rs` 新增 `ProductionFramePasses`（63 seam，56 个返回 unsupported）。
- **中央复核发现**（已回退给②/③）：
  1. `ProductionFramePasses`/`render_world_frame` 全 workspace 无生产调用点，是死代码；真实路径是 `build_world`。不得按“63 seam 已实现”计入。
  2. producer 缺失时默认返回 Ok(minimal) 可能掩盖缺口；默认应硬错误，仅显式诊断开关允许 minimal。
- 既有阻塞：`crates/systems/critters/src/duck.rs:812` 的 `device.device()` 类型错误会阻塞 tiny-glade check（非本轮文件，待统一修）。

### G1③ 复核（有条件通过，水 3）
- 主链已真实接通；契约四条纠偏均落地。
- 阻断 M1：`DecoratorAddrJson::visit_str` 不接受真实样本的 `"Stairs"` 地址（5/36 文件、9 条），整节 fatal。
- 高 M2：`load_game.rs:992` 在快照 apply 前推进到 `ApplyCounter`，失败后半应用世界被当完成。
- 高 M3：解析失败后每帧重试坏文件。
- 低 M7：文档“无 Stairs 节”与样本/写出侧矛盾。
- 已回给②修复；中央同时授权②修 `save_thread.rs` 的 Windows 0x80 布局（path 改 24B String），以解除测试编译阻塞。
- G1③ 已独立用样本复核计数：162 walls/73 roofs/134 decorators/431/0/20/590/104/5/50。

### G2③ 复核（有条件通过，水 1）
- 四类硬水分（静默 no-op / 测试冒充生产 / 假成功 / 伪造 API）均为 0；唯一水分是把原生 `+0x1c=6` 误读为 usage，导致 P1。
- P1 高：`main color` 缺 `.mip_levels(6)`，DoF 的 `generate_mips` 会读 mips=0。
- P2 高（链）：`warm_up_gameplay` 恒 Err，默认 `skip_splash=false` → gameplay 永不发布，`compile_world_graph` 默认不可达。
- P3 中：降分辨率重试因静态 `extent_10` + `max` 实际不降。
- P5 低：`world_consumer_missing` 文案过期。
- 已回给②修复；授权其按需改 game_loop/run_game 的发布接线。

### 中央修复与里程碑（2026-10-19）
- 中央修复 `crates/systems/critters/src/duck.rs:818`：`device.device().update_buffer_bytes(buffer, ...)` 误用固有方法且参数类型不符；改为 UFCS 调用 boundary trait：`GraphicsDevice::update_buffer_bytes(device.device(), &buffer, &bytes, &[0..528], 0)`。
- `cargo check -p country-core` **通过**（165 warnings）。
- `cargo check -p tiny-glade` **通过**（Windows，27s）。这是本轮首个“整宿主可编译”里程碑。
- G1② 修复后 `cargo test -p country-core --lib systems::save`：**49 passed / 0 failed**，含 36 样本回归（5 个 Stairs 桶）。

### G2② 复核修订（③ 反馈，2026-10-19）
- P1 已修：`main color` 现由 `world_frame::main_color_desc` 创建，含 `mip_levels(6)`
  （0x140bd5460 `mov [rdx+0x1c], r8b`）；`main depth` 显式 `mip_levels(1)`。
  新增测试 `main_color_mip_count_matches_generate_mips_input`（format 0x7a、mips 6、<=13）。
- P2 已接线：`warm_up_gameplay(owners, world)` → `render_bridge.rs:887
  warm_up_gameplay_with_world_builder`；`game_loop.rs:305` 在移动 gameplay SubApp 前
  `warmup_owners()`，发布门仍由 `CompletedGameplayWarmup` 的 Arc 身份校验。
  **阻断级继承缺口（登记）**：builder 仅覆盖 W1..W54 + TAA 导出，不含原生 compile_main 闭包尾部
  grading/blur/final_post/UI，故 `render_bridge.rs:883-884` 的“complete compile_main”文档暂不成立；
  补齐尾部前不得宣称 warm-up 等价完整 compile_main。
- P3 已修：`world_extent` 改为 `min_extent` 覆盖语义（契约 §5.1 M1），否则
  `WindowSize × GameConfig.video.world_resolution_scale`，使 OOM 降分辨率实际生效；原生 scale
  更新点未定位（I），已在交付记录取证点。
- P5 已修：删除 `world_consumer_missing`。
- 验证（真树，中央修复 save/duck 后复跑）：`cargo check -p country-core`、`-p tiny-glade`
  均通过；`world_frame` 7 passed、`generate_mips` 10 passed、`dof` 23 passed、
  `tiny-glade --lib world_extent` 3 passed。

### 中央平台修复与真树验证（2026-10-19）
- `input/recording.rs`：`NamedTempFile::persist` 在 Windows 不能覆盖已存在文件。改为 Windows 分支“旧文件移到同目录唯一备份 → persist → 成功删备份 / 失败恢复”，保留“失败保留原文件”契约。
- 资源测试路径：`../../../../ljxsj_92385`（落到 `D:\ljxsj_92385`）改为 `../../../ljxsj_92385`（`D:\game\ljxsj_92385`），涉及 `rocky_terrain/load_prefabs/startup_terrain/startup_water`。
- 真树验证：
  - `cargo check -p country-core` → **通过**（165 warnings）
  - `cargo check -p tiny-glade` → **通过**
  - `cargo test -p country-core --lib` → **2467 passed / 0 failed / 0 ignored**（13.29s）
  - 含 G1 新增的 36 样本快照语料回归、5 个 `"Stairs"` 桶、资源加载测试。

### 最终复核结论（2026-10-19）
- **G1③：通过**。水分 2（均为非阻断的文档/接缝项）。M1/M2/M3/M7 全部修复并实测：
  - `cargo check -p country-core` 退出 0；
  - `cargo test -p country-core --lib systems::save` 真实执行 49 passed / 0 failed（snapshot 15/15）；
  - 36 样本语料实跑通过，162/73/134/431/0/20/590 等计数命中。
  - 残留非阻断：M4/M5/M6/M8，以及 R1（终态失败未发宿主事件，跨组接缝）。
- **G2③：通过（G2 范围）**。水分 0。P1/P2/P3/P5 全部修复并真树验证：
  - `cargo check -p country-core`、`-p tiny-glade` 均 Finished；
  - `world_frame` 7 passed、`generate_mips` 10 passed、`tiny-glade world_extent` 3 passed；
  - **继承缺口（阻断级登记）**：warm-up 的 world builder 只覆盖 W1–W54 + TAA 导出，未含原生 compile_main 闭包尾部 grading/blur/final_post/UI；补齐前不得宣称 warm-up 等价完整 compile_main。
- GPU 运行时仍未验证（项目后置）；本轮结论均为源码/CPU/图编译层。





