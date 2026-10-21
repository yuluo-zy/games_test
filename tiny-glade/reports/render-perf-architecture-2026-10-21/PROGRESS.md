# 实施进度

每项的验证口径与完成定义见 IMPLEMENTATION.md。本机只能用 lavapipe 验证功能与计数器,所有耗时收益待真机。

| 任务 | 状态 | 提交 | 说明 |
|---|---|---|---|
| P0 0.1 render_stages 采样间隔可配 | 完成 | f1a7d841c | `TG_FRAME_TIMING_EVERY=1` 逐帧 |
| P0 0.2 GPU 逐 pass 计时接线 | 完成 | f1a7d841c | `TG_GPU_TIMING=1`;样本带 pass 名;未回填 `last_gpu_frame_ns`,帧节奏不变 |
| P0 0.3/0.5 上传等待、prewarm 计数 | 完成 | f1a7d841c | `frame_counters` 行 |
| P0 0.4 VAO 刷新计数 | 已有 | — | `vao_full_refresh` 行此前已存在 |
| P0 0.6 真机采集 | **待用户** | — | 需要 Windows 真机 |
| A1 帧在飞按显示帧计数 | 完成 | 86216d4f8 | 新增 `frame_throttle_ms` |
| A2.1 上传异步 ticket | **未做** | — | 多处【未核实】、flush 挂点多,本机无法端到端验证 |
| A2.2 W11 仅变化时上传 | 完成 | c6de66272 | 宿主端到端本机无覆盖 |
| A3 prewarm 待决集 | 完成 | e0a2d2fb5 | |
| A4 VAO 快照一次 | 未做 | — | |
| A5 bevy_ecs multi_threaded | **暂缓** | — | 会改变全局系统并行度,需先审计顺序依赖并在真机验证 |
| A6.1 PGO 按图分槽 | 未做 | — | |
| A6.2 主循环专用线程 | 未做 | — | 改动最大 |
| A7.1 预算查询节流 | 完成 | b79f1bf0d | |
| A7.2 PGO 计时门控 | 未做 | — | 随 A6.1 |

## 本机构建环境(Linux,无 sudo)

- 依赖镜像:`cargo ... --config /tmp/ustc.toml --locked`(去掉 `--offline`,上游新增了 `lcms2`)。
- tiny-glade 依赖 GTK 系统库(经 rfd)。用占位 `.pc`(`PKG_CONFIG_PATH=/tmp/stub-pc`)过 `cargo check`,
  链接测试用 `LIBRARY_PATH=/tmp/stub-lib`(指向系统 `.so.0` 的符号链接)。
- `CARGO_TARGET_DIR=/tmp/tg-target`(约 32G,复用)。磁盘仅余约 26G,不要新建第二份 target。

## HEAD 上已存在的断裂(与本次改动无关)

- `country-core` lib 无法编译:`systems/ui/ui_update.rs:9` 从 `resources::gui` 导入 `EscapeMenuJustOpened`,该类型实际在 `resources::events`。
- `country-core` lib 测试目标无法编译:`ui_update_chain_tests.rs` 依赖 `use super::*` 带出的导入;`hover_intersection.rs:349` 闭包参数需标注类型。
- 以上是上游状态,本次没有修改也没有提交修补。
