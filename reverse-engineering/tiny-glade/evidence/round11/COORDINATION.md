# 第 11 轮协调台账（2026-10-19）

用户指示：集中力量推进三个上层模块 `toolbar`、`wall-constructor`、`decorator`。
并发限制：**最多 5 个 agent 同时运行**（用户 2026-10-19 指示）。

## 文件权（互不重叠）

| 组 | 可写 |
|---|---|
| WC（wall-constructor） | `crates/systems/wall-constructor/**` |
| DEC（decorator） | `crates/systems/decorator/**` |
| TB（toolbar） | `crates/systems/toolbar/**` |

跨界需求（需要 country-core 新 API/字段）必须先报中央；不得改用户重构区
（rhapsody 本体、roof、wall 含 snapping/signifier）。T13 反馈捕获按 AGENTS.md
明确排除，不推进。

## 当前基线（第 10 轮末）

- `cargo check -p country-core`、`-p tiny-glade` 通过。
- `cargo test -p country-core --lib` 2510 passed / 0 failed。
- `cargo test -p tiny-glade --lib` 37 passed / 0 failed。
- 三 crate 现状（初步扫描）：
  - `toolbar`：18 文件 / 5,325 行 / 0 `todo!`；上层扫描 82 个骨架函数无迹象。
  - `wall-constructor`：53 文件 / 3,170 行 / **42 `todo!`**；上层扫描 37 todo + 28 无迹象。
  - `decorator`：78 文件 / 9,288 行 / 0 `todo!`；上层扫描 62 空壳 + 18 无迹象。

## 波次（≤5）

- 第 1 波（5）：WC①契约、WC②实现、DEC①契约、DEC②实现、TB①契约。
- 第 2 波（按槽位）：TB②实现、WC③复核、DEC③复核、TB③复核。

## 验收门

- 真实业务代码 + 真实消费者；禁止 `todo!` 长期留存、空体、log 冒充写入、
  伪造字段/常量/API、默认值掩盖分支。
- 保留原作复杂业务与分支、状态、边界、副作用、生命周期、跨模块交互；
  证据不足的显式标 U，不简化。
- 每个 crate 至少：关键链真实接线 + 新增测试 + `cargo check -p <crate>`。
- 三轮完成后由中央逐项复核。

## 编译隔离（用户指示 2026-10-19）

**每个 agent 使用独立 CARGO_TARGET_DIR，不再共享 `target/`，避免 Cargo 锁互等。**
约定：
- WC②：`/d/game/tiny-glade/target-agent-wc`
- DEC②：`/d/game/tiny-glade/target-agent-dec`
- WC③：`/d/game/tiny-glade/target-agent-wc-review`
- TB③：`/d/game/tiny-glade/target-agent-tb-review`
- 后续 agent：`target-agent-<组><角色>`，互不重叠。
首次全量编译较慢，但不会互相等待。

## 并发与接线更新（2026-10-19 17:5x）

- 用户取消并发上限；新增 WC-C（测试）、DEC-C（烟雾/资源/可视状态）、TB-C（菜单/拍照/文本）三路，
  文件权互不重叠。
- 中央已把三个宿主入口接进 `crates/tiny-glade/src/run_game.rs`：
  - `toolbar_plugin::ToolbarPlugin`
  - `decorator_plugin::build`
  - `wall_constructor_plugin::build` + `register_cpu_systems`
  - `WallCpuStage::Edit/Bricks/Holes` chain，且 `Edit.after(GameSystemSet(19))`
- `crates/tiny-glade/src/lib.rs` 增加 `pub mod wall_constructor_plugin;`。
  宿主编译验证进行中。

## 方向调整（用户 2026-10-19）

- **不接入宿主接线**：中央已撤回 `run_game.rs`/`lib.rs` 的插件接线；`toolbar_plugin.rs`、
  `wall_constructor_plugin.rs`、`decorator_plugin::build` 仅作为 crate 侧入口保留，不在本轮挂 App。
- **集中 toolbar 与 decorator**；wall-constructor 暂停（WC②/WC-B/WC-C 已停止）。
- 并发上限取消，多 agent 并行；每 agent 独立 target 目录。
- 当前在跑：
  - toolbar：TB③（增量复核）、TB-C（菜单/拍照/文本）、TB-D（toolbar_systems/update/taffy_update）、TB-E（测试）
  - decorator：DEC②（P0/UI/生命周期）、DEC-B（依附/合并/子类型）、DEC-C（烟雾/资源/可视状态）、DEC-D（视觉构件）、DEC-E（音频/测试）

## 工具超时规则（用户指示 2026-10-19）

**单个工具调用必须有硬超时，禁止超过 10 分钟。**
- 所有 agent 的 Bash/编译命令前置 `timeout 600`，并设 `CARGO_BUILD_JOBS=4`。
- 单次超时不得原样重试；缩小范围（单 crate/单测试过滤）、复用 target，或记录“截断”。
- 中央自身所有 Bash 调用显式设置 `timeout <= 600000ms`。
- 中央看门狗：agent 超过 ~20 分钟无新 DELIVERY/进展则停止并收窄任务重派，避免 1h+ 空转。
