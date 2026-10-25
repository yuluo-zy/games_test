# COMPILE-GREEN2 — country-core 全 crate 测试目标绿窗口收口

> 角色：compile-green2（country-core 全 crate 测试目标绿窗口收口 restorer）。
> 命令：`cargo check -p country-core --tests` 与 `cargo test -p country-core --lib`。
> 范围：编译级最小改动，不改业务逻辑、不删测试；rhapsody 兄弟 crate 在途错误先查是否他 agent WIP。
> 日期：2026-10-17。

## 结果

- **`cargo check -p country-core --tests`：0 error**（lib / lib-test / 集成测试三态编译通过，仅剩警告）。
- **`cargo test -p country-core --lib`：全绿** —— `1837 passed; 0 failed; 0 ignored`。
- 初始现场：`cargo check --tests` 已 0 error（上轮 COMPILE-GREEN 收口成果保持），但 lib 测试有 **4 项失败**（1833 通过 / 4 失败）。本轮逐一定位并修复，终态 1837 通过 / 0 失败。
- 兄弟 crate：`git status` + mtime 核对 rhapsody 相关文件（lib.rs / executor.rs / vulkan.rs）在途但已 settle，且 `cargo check -p country-core --tests` 可正常编译链接（country-core 侧无 rhapsody 阻断错误），未触碰 rhapsody 文件。
- 终态复验（cargo 自带锁）：`cargo check --tests` 0 error；`cargo test --lib` 0 failed。

---

## 1. 错误→修复清单（4 项失败测试）

### 1.1 `geometry::sphere::tests::uv_formula_matches_the_closure`（既有精度）

- 现象：`assert_eq!(sphere_uv(Vec3::NEG_Y), [0.5, 1.0])` 实际得 `[0.5, 0.99999994]`。
- 根因：南极点 `v = acos(-1.0f32)/PI`；本机 libm `acos(-1.0f32)` 返回 `3.1415925`，比 f32 `PI=3.1415927` 低 **1 ulp**，相除得 `0.99999994`。文件头已注明恢复的 CRT 调用末位可能与 Windows CRT 差 1 ulp。
- 修复：`sphere_uv` 公式不动；测试对南极点改浮点容差断言（`(v-1.0).abs() < 1e-5`），并加注释说明 1 ulp 来源。测试未删、业务未改。
- 文件：`crates/country-core/src/geometry/sphere.rs`。

### 1.2 `systems::camera_rig::main_camera::tests::forwarders_read_the_maintained_camera_matrices`

- 现象：写入 Normal body 位置 `(3,4,5)` 后 `rebuild_camera_matrices_from_normal_body()`，`eye_position()` 仍返回 `(0,0,0)`。
- 根因：`rebuild_camera_matrices_from_normal_body` 经 `self.eye_position()` 读**维护矩阵**（@+0xc0），而该矩阵正是本函数要重建的 → 循环依赖；默认缓冲全 0，故围绕 `(0,0,0)` 重建。
- 修复：位置改为直接从 Normal body 偏移（`NORMAL_POSITION_OFFSET` +0x138/+0x13c/+0x140）读取，再配 yaw/pitch 重建 `inverse_view`。此为 crate 内转发辅助（I：projection=IDENTITY），非原 EXE 业务改动。
- 文件：`crates/country-core/src/systems/camera_rig/main_camera.rs`。

### 1.3 `systems::history::replay_stairs::tests::animated_redo_records_and_undo_erases_support_edge`

- 现象：动画路径 redo（instant=false）push 后只跑一次 schedule，`StairsRemovedSupports` 不含边 `(3,4)`。
- 根因：动画门控（O 0x140af75b0）`elapsed <= max(duration/speed, 0.05)` **先判后加**；speed=1.0、duration=0.1 → required=0.1，dt=0.2：run1 只把 elapsed 0→0.2 而未处理，需 run2 才处理。测试只跑 1 帧。
- 修复：动画路径两处各补一次 `schedule.run`（redo/undo 各 run2 帧）。replay 门控逻辑（`StairsReplayCmd::advance` / `REPLAY_DURATION_FLOOR`）不动。
- 文件：`crates/country-core/src/systems/history/replay_stairs.rs`。

### 1.4 `systems::save::snapshot::tests::decorators_json_into_storage_snapshot_and_apply_restore`

- 现象：`serde_json::from_value::<DecoratorsSectionJson>` 报 `data did not match any variant of untagged enum DecoratorAddrJson`。
- 根因：样本契约 `{"Wall": <id>}` / `"Terrain"` / `{"Stair": <id>}` 中 `"Terrain"` 是 **JSON 字符串**，而 `#[serde(untagged)]` derive 对单元变体 `Terrain` 生成的代码只接受 `null`（`deserialize_unit`），字符串解不到任何变体（已用最小 serde 用例实证）。
- 修复：给 `DecoratorAddrJson` 手写 `Deserialize` visitor：`visit_str("Terrain")` → `Terrain`，`visit_map` 单键 `Wall`/`Stair` → 对应变体，其余报 `unknown_variant`。枚举形状不变（`to_storage_addr` 的 match 零改动）。
- 文件：`crates/country-core/src/systems/save/snapshot.rs`。

---

## 2. 最终测试计数

| 阶段 | `cargo check -p country-core --tests` | `cargo test -p country-core --lib` |
|---|---|---|
| 初始 | 0 error（仅警告） | 1833 passed / **4 failed** |
| 修复后 | **0 error**（仅警告） | **1837 passed / 0 failed** |

4 项失败归属标注：
- 1 项为既有浮点精度（acos 末位 1 ulp），作容差适配；
- 3 项为本轮收口现场遗留（1 处转发辅助循环依赖、1 处测试缺动画门控帧、1 处 serde untagged 单元变体契约），均以最小编译/测试级改动闭环，未改业务逻辑、未删测试。
