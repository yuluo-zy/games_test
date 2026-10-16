# rhapsody 架构重构计划

2026-10-16 起，rhapsody 不再以"还原原版"为目标，**正确性与高性能是唯一标准**。原版地址、布局、panic 行为不再约束设计；原版 oracle 测试直接删除；对外 API 允许破坏性变更（下游 `tiny-glade`、`tg-text` 同步修改）。本文件是各阶段的进度入口。

## 目标架构

```
rhapsody
├─ error.rs        Error { DeviceLost, OutOfMemory{device|host}, SurfaceOutOfDate, SurfaceLost,
│                  Timeout, Unsupported(..), InvalidUsage(..) } + Result<T>
├─ device/         Device（唯一设备对象）、DeviceConfig/QueueConfig、Queue（各自锁 + 时间线）、SyncPoint
├─ resource/       Handle<T>（slotmap 代数键）、ResourceName(Ustr)、buffer/texture/sampler/view、Retirement
├─ pipeline/       shader、descriptor、render pass、管线缓存
├─ graph/          类型化 TextureId/BufferId → 编译（屏障、别名、跨队列同步）→ 执行
├─ frame.rs        FrameLoop：begin → acquire → main → present_graph → submit → present
└─ swapchain.rs    自带锁；二值信号量按图像槽；最小化/过期 → Skip/Recreate
```

关键约定：

- **同步与回收**：每个 `Queue` 一条时间线；提交在本队列锁内分配 `next_value`，`vkQueueSubmit` 成功后才提交该值（失败不占用值，不做 CPU 补发 signal）。资源与命令池记录 `SyncPoint{queue, value}` 集合，`Retirement` 是唯一回收真相源。设备丢失进入 `DeviceLost`，等待立即返回错误。
- **锁**：无全局锁。锁顺序 Swapchain → Queue → Retirement；分配器自带锁；内存持久映射。
- **错误**：公共 API 一律 `Result<T, Error>`；误用在 debug 下 `debug_assert!`、release 下返回 `InvalidUsage`；帧失败回滚本帧，只有 `DeviceLost` 使对象不可用。
- **资源身份**：raw 层 `Handle<T>`，过期句柄可检测；图内 `TextureId`/`BufferId` 区分类型；跨帧资源以 `ResourceName` 为主键，temporal 为按名字的乒乓对，框架负责奇偶与首帧 reset；调试名自动设置。
- **多队列**：`QueueConfig { graphics: 1, async_compute: Off|Prefer|Require, transfer: Off|Prefer|Require }`；`Prefer` 不支持时别名到 graphics；pass 声明 `QueueClass`，编译器插入跨队列时间线同步与 ownership transfer（独占/CONCURRENT 可配置）。
- **性能**：命令池按帧环 × 线程整池复位；描述符每帧池 reset；屏障按 pass 批量 `vkCmdPipelineBarrier2`。

## 阶段

| 阶段 | 内容 | 状态 |
| --- | --- | --- |
| P0 | 删除原版 oracle 测试；1000 帧稳定性测试（P3 前 ignore）；性能基线 | 完成 |
| P1 | 统一错误模型（去 `Result<_, String>` 与原版 panic） | 未开始 |
| P2 | 设备合一 + 可配置队列 + 去全局 `queue_lock`（下游同步迁移） | 未开始 |
| P3 | 统一同步/回收：`SyncPoint` + `Retirement`；删 graveyard/SequenceRuntime/SubmissionEngine/销毁桶/events 日志 | 未开始 |
| P4 | 语义化资源身份：`Handle<T>`、`ResourceName`、命名 temporal | 未开始 |
| P5 | 帧控制流闭合：acquire→主图→展示图→present；headless 整帧测试；帧失败回滚 | 未开始 |
| P6 | 图内多队列调度（lavapipe 只能用别名队列验证；真实多族需真机） | 未开始 |
| P7 | 删重复实现与原版注释；性能三项并对比基线；更新文档与 provenance 规则 | 未开始 |

## P0 结果

- 删除 9 个测试文件与 45 个断言原版机器码/布局/panic 文案的测试函数；保留行为测试。测试 342 → 298 通过 + 2 ignored。
- `tests/p0_stability.rs`：1000 帧会话簿记（在飞帧、资源/命令数、生命周期桶、托管待销毁、导入释放）全部平稳；`events` 日志 4324 → 27504（约 29 条/帧）无限增长，该用例 ignore 待 P3。
- `tests/p0_perf_baseline.rs`（ignore）：6 pass 图（清除、填充、光栅三角、两次拷贝、读回导出），300 帧去 10 帧预热。

| 构建 | mean | p50 | p95 | 每帧命令缓冲 | 每帧屏障命令/转换 | 每帧 vkQueueSubmit |
| --- | --- | --- | --- | --- | --- | --- |
| release | 0.30 ms | 0.29 ms | 0.41 ms | 3.03 | 5 / 11 | 1 |
| debug | 0.45 ms | 0.44 ms | 0.56 ms | 3.11 | 5 / 11 | 1 |

运行：`cargo test --release -p rhapsody --test p0_perf_baseline -- --ignored --nocapture`。

## 待修缺陷登记（按新标准，原版行为不再是保留理由）

| 缺陷 | 位置 | 归属阶段 |
| --- | --- | --- |
| `resolve_view_mip_range` 用 `base as u8` 截断（base=256 通过检查）；`n >= avail` 拒绝"恰好剩余全部 mip" | `src/rg/binding.rs:352` | P1 |
| cube 初值上传只写 1 个面（按 `array_elements` 而非 6 层） | `src/raw/setup.rs`、`texture_upload.rs` | P4 |
| 合法 BC 格式（BC1_RGBA、BC2、BC4_SNORM、BC6H 等）初值上传被拒（原版块字节表不全） | `src/raw/format_table.rs` | P4 |
| 单批 6 pass 图每帧录 3 个命令缓冲：session 强制原版 PGO 分块 | `src/rg/parallel.rs`、`vulkan.rs` | P7（P2 先改成可配置） |
| `src/` 内仍有原版布局单测（约 66 处 `size_of`/`offset_of`）与 `original_*` 辅助 API | 多处 | P1 起随模块重写删除 |

不在本轮：country-core `RenderHost` 的宿主实现（依赖 `ustr_contract::intern` 恢复）。

## 验证口径

在 `tiny-glade/` 下，`CARGO_TARGET_DIR=/tmp/tg-target`、`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`：

```bash
cargo test -p rhapsody -p shader-compiler -p shader-pipeline [--all-features] --no-fail-fast
cargo clippy -p rhapsody -p shader-compiler -p shader-pipeline --no-deps --all-targets [--all-features] -- -D warnings
cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets
```

每阶段全绿后单独提交。
