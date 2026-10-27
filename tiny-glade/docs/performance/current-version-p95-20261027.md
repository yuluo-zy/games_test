# 当前版本 p95 与 WIP 待完成项（2026-10-27）

用户现已明确要求先提交并推送当前修改及待完成内容。本次是 WIP checkpoint，尚未完成原性能计划的全部验收。

## 当前实际测量

测量源码 `bd7e1098284028d8d38eed92ea7b1415269d3a09`；Release / mimalloc / cpu-probe；EXE SHA256 `2bc6617596641e41412d7e6db2f3161a4bada23a4d78f32bb224bb367744ad88`。1280×720 输出、world scale 1.0，保留原内部对齐尺寸与画质。静态场景预热 600 帧、记录 3,001 个完成点，形成 3,000 个观察帧；其余三个场景尚未启动本轮采样。

| 当前场景 | 渲染阶段 CPU mean | p95 | p99 | 状态 |
| --- | ---: | ---: | ---: | --- |
| 静态 | 9.5748ms | **12.6120ms** | 14.7553ms | owner-only 派生检查通过；原全进程 run 仍失败 |
| 大量绘制（draw-list ≥1,000） | — | — | — | 待测 |
| 每30帧局部编辑 | — | — | — | 待测 |
| 每帧连续编辑 | — | — | — | 待测 |

渲染阶段为每帧六段 CPU 运行时间之和后取分位数，含帧后 maintenance/夹具维护；不含 App 更新或 GPU 等待。不含 maintenance 的五阶段 p95 为 12.5740ms；含 App 的整帧 owner p95 为 15.7644ms。该 headless 路径中 App 和渲染共用 owner，不能声称已经测到独立生产渲染线程或完成 16ms/WSI 验收。

owner 调度无缺项、ETW 丢失事件/缓冲为零、生命周期包围完整窗口。累计 CPU 相对 GetThreadTimes 偏差约 0.948%，相对 Microsoft CSwitch 约 0.000761%。原完整报告因非 owner TID 11788 在 T-End 后同一 PID 再调度触发验证器条件而失败；失败状态和原始证据保留，不事后放宽全进程判定。[完整 owner 证据](current-static-owner-p95-20261027.json) 保留各检查、原策略、原始文件 SHA 和数值。

## 本轮额外验证

当前 `bd7e1098284028d8d38eed92ea7b1415269d3a09` 的 workspace 全目标 `tiny-glade/cpu-probe,tiny-glade/surface-compare-test` 类型检查通过（2m58s）；日志 `.perf/bd7e109-workspace-probe-surface-all-targets-check.log`，SHA256 `08ae802d880a7fe5df1723d2538a81e1b896911c4d30cf039e24309773d499d2`。这没有执行 GPU 测试，也不代表默认配置检查或下面待验项已通过。

## 接下来按依赖完成

1. 大量绘制、局部编辑、连续编辑：尚未启动本次当前版本采样；分别预热 600 / 采样 3,000 帧。
2. 静态全进程校验：保留原失败，修正并验证线程 T-End 后同一代 kernel cleanup 再次调度的识别；不能将现有 owner 派生结果升级为全进程通过。
3. 最新默认 workspace 全目标检查，以及 canonical 5 / local surface 6 / pending-acquire 9 / Tiny observer 2 / EffectsHost lifecycle 1 的实际执行仍待补齐；探针+surface workspace 类型检查已通过，不替代原生执行。
4. 水/冰严格像素、实际 GPU 输入闭环、Collect 代表性取消/预算耗尽恢复和四场景回归仍待完成。
5. 探针开销 ≤1%、正式多轮性能验收、完整压力回收及稳定负载/效果切换/resize 各 30 分钟且 10,000 帧仍待验。
6. 默认 suballocation 的 Drop/回收仍有 Rust 堆分配；Painterly Ready/progress、一次性 LUT 与 capture 发布事务的 OOM 恢复方案尚未实现。
7. 真实窗口 WSI 兼容按此前指示暂缓，macOS 待验。只测当前版本不能计算相对优化前的提升百分比。

原始 ETL/CSV、程序/PDB、资产和构建生成物仍留在被忽略的 `.perf`，本次 Git 只保存源码、工具、证据摘要及开放项。
