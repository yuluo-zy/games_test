# 2026-10-27 checkpoint: f39dc61

`f39dc61eef85edebda2f491b4d31d9ff9e85c144` 是此次原生验证使用的 clean checkpoint，不是当前工程 HEAD。工程已推进到 `fa98f8d4367cc6ce24fd5d7a5e03cc82b36008d7`；本次文档交付前仅有主线程复制的两份 OOM 设计文档未跟踪。后续工具安装和 EffectsHost 正确性修复不能继承 f39 的原生验证结果，最新 Tiny/workspace、原生回归、严格水冰像素与正式 CPU 验收仍未完成。

| 已提交内容 | 提交 | 当前验证边界 |
| --- | --- | --- |
| pending-acquire 独占 reservation / lease 与 W2 空闲压力回收 | `a688644` | 集成 Rhapsody 类型检查通过；8 个 storage 测试和 1 个真实 Device maintenance 测试尚待执行 |
| 同前缀、原格式 surface 对照夹具 | `5e198a0` | 含 static-lifetime 修复；6 个相关 Rust 测试尚待执行，Python 通过不替代它们 |
| 默认 suballocation Drop 的堆分配诊断 | `d014743` | 2 个真实原生诊断已执行，round2 通过；round1 Job 清理门失败原判保留 |
| 可选、保留 owner 的 canonical layout 准备 | `f39dc61` | 5 个测试尚待执行；最新 Rhapsody 两种类型检查已覆盖本提交的 read-only getters |

## 实际通过及保留失败

- **f39 补充结果：**主线程在 clean `f39dc61` 串行完成 Rhapsody cpu-probe/default `--all-targets` 类型检查，分别 **31.52s / 31.00s** 通过，包含最新 canonical getters。此结果补足下述早期 Rhapsody 检查的源码范围；没有运行新原生测试，也没有启动重 Tiny 检查，不能扩大为 Tiny、country-core 或 workspace 通过。
- clean `6d1cf8a73333541bfcabaa32b96fadfe6923b8c2` 的两个 Rhapsody weak-upload observer 测试通过，0 ignored；v2 private Job `valid=true`、自然退出且最终为空，源码前后清单一致。本轮复用 `fresh=true` 的已编译测试程序，仅覆盖这两个 observer 测试，不覆盖后续 W2。旧 v1 同样执行了两个通过测试，但因 Job 清理超时整体 `valid=false`，旧失败保持原判。
- 集成 W2、local R2 和 suballocation 后，Rhapsody `--all-targets` cpu-probe/default 检查分别 30.87s / 29.91s 通过。这发生在 `f39dc61` canonical read-only getters 之前；仅凭这两份早期日志不能覆盖最终 f39。最新 Rhapsody 范围由上方补充结果单独成立，Tiny 和整个 workspace 仍未通过。最初 E0521 检查日志保留，后续成功的 `-2` 日志单独记录修复结果。
- 实际 Python comparator/controller 回归为 **63 tests / 13.846s / OK**。它们未执行 Rust GPU 测试。
- Tiny observer fresh build 在编译器申请 **14,942,224 bytes** 时失败，compile exit 101，没有生成可接受测试 artifact，也没有运行两个目标测试或 GPU。
- 最新 Tiny `cpu-probe,surface-compare-test` 组合的 metadata 检查在编译 country-core 时申请 **262,144 bytes** 失败。记录的 rustc abort 发生于 allocation failure 后；该轮没有完成类型检查或运行 GPU，不据此推断产品 GPU OOM 或缓冲区错误。

### 新增 f39 原生尝试与堆诊断

pending-acquire 9-test 目标的 libtest 编译申请 **2,097,152 bytes** 失败，compile exit 101、artifact/test_exit 均 null；Cargo 38.234s、整轮 42.578s，没有执行这 9 个测试或 GPU。虽然 Cargo 已产出普通库 rlib，这不代表 libtest 目标完成。pre-GO Job 分配、自然退出后为空及源码前后相等只证明这轮失败受控，不构成测试通过。

较小的 default-suballocation heap 目标实际执行两个测试：round1 compile/test exit 均 0、2 passed / 0 ignored，但 private Job 在 30s drain 截止仍有一个进程，整体 `valid=false`。记录以同一个已打开进程 handle 观察到 PID 7388、`member_of_this_job=true`、image 为 VS2022 BuildTools 的 `vctip.exe`，本 Job cleanup 后归零；这只能解释本次 heap round1 的已证身份，不能推广为旧 W1 Job 失败的原因。cached round2 使用相同 EXE/PDB，2 passed / 0 ignored、test 0.60s，Job 自然清空、无 errors，`valid=true`；机器记录的 elapsed 为 5.188s。三轮源码前后 SHA 相同，9 / 15 / 15 个封存文件 SHA 均独立核对，两轮 6 条原始诊断记录完全相同：

| 实际 phase | 默认 suballocation Rust alloc 次数 / 请求 bytes | dedicated control | collected native items |
| --- | --- | --- | --- |
| unused Drop，保留 live anchor | 1 / 64 | 0 / 0 | 未执行 collect |
| completed submission 的 Drop 入队 | 0 / 0 | 0 / 0 | 未执行 collect |
| fragmented completed reclaim，保留 live anchors | 33 / 2,640 | 0 / 0 | 双方各 33 |

这确认默认 suballocation 的部分析构/回收路径仍会申请 Rust 堆，不能宣称 Drop 全部无分配。计数器是现有 `CountingAllocator<System>`，只统计调用线程 Rust alloc/alloc_zeroed/realloc 的尝试与请求 bytes；不测 C/driver 堆、其他线程、dealloc、peak/live heap、thread/frame CPU 或生产 mimalloc 成本，不给 CPU 收益结论。

## 真实 surface 与 trace 仍有开放项

`c98be8e` whole-scene 水冰对照的两个 child 各为 1 passed / 0 ignored / 零 VUID，20 个封存文件及 raw hashes 已核验。每季 30 帧的完整记录输入、1060 个 u32 FrameConstants、history/config 和 retained pass order 相等；严格像素仍失败：

| capture | 改变像素 / 921,600 | 最大通道差 |
| --- | --- | --- |
| water 20 | 88,897 | 3 |
| water 30 | 81,320 | 4 |
| ice 20 | 328,728 | 17 |
| ice 30 | 290,961 | 6 |

四份 Nani GPU-byte 观察均为 `Unavailable`：真实 owner 为 DEVICE_LOCAL、非 HOST_VISIBLE。因此已记录输入相等不等于完整 GPU 输入相等，也不能将像素差异直接归因于 Collect。local 同前缀夹具与 canonical 可选准备将用于新一轮定位，不能重标旧失败。

同 c98 rich EXE 的 trace 调查定位到 502 个 Wall bricks 的真实 sparse 段数差异：OFF 3 段、ON 4 段；观察行 direct dispatch 为 355 / 357。两个 Wall pass 各 92 个 scope，每个多一次 dispatch，解释整个 trace 的 184 次差值。其余 111 个 pass 名称分布以及 Nani 不变。14 行 graph command 与 14 行 graph chunk 差异作为管理成本继续保留。trace 包含启动/预热且没有提交 receipt join；502 个数相同不证明 payload 或像素相同，startup 根因尚未证明。两个 raw CPU run 仍 `valid=false`，更早的失败 pair 也不改判。这些计数不提供 CPU 毫秒收益。

可回溯的 Wall 定位记录见[版本化报告](C:/game/docs/performance/trace-source-wall-layout-20261027.md)及其[机器可读摘要](C:/game/docs/performance/trace-source-wall-layout-20261027.json)。c98 whole-scene 的[失败审计](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/surface-c98-update/audit.json)和[说明](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/surface-c98-update/summary.md)保留 sealed-file/raw-pixel 核验细节，实际原始数据位于 `C:/game/.perf/surface-runs/lake-native-source-c98be8e-1`。本次仅引用这些历史数据，不改写原始报告或判定。

## 后续工程与工具集成范围

- `b0c7a3d` 集成 canonical input contracts 与可选 trace 诊断工具：实际 Python 135 tests / 57.513s、binding 44 tests / 0.264s 通过。
- `c1dd0b3` 单独修复 EffectsHost 两处 reused 判断，从 `== 0` 改为 `!= 0`，对应真实 history-valid=1。新增 1 个 8-step 原生 EffectsHost 生命周期测试，含真实提交/读回及首个读回 `first[0] > 0` 断言；当前只有源码审查及语法解析，未 Tiny 类型检查或 GPU 执行。它不证明 OOM 恢复完成，Painterly Ready/progress 和 menu/LUT 事务仍开放。
- `fa98f8d` 集成 surface local freeze 工具；strict policy fixture 经 Git 应用后的 LF 差异导致旧一轮 91 tests / 2 ERROR，该失败日志保留。最窄 LF 修复后的新一轮实际 **94 tests / 15.139s / OK**。主线程核对 14 payload SHA、7 qualified 工具与旧 importer 原字节；旧 `2cbf` 冻结身份可冷读验证，原 metadata `75ea` 未改。这里只接收工具回归，不接收新 binary/GPU；common4 隔离基线尚未激活。

可恢复编译 OOM 的[设计说明](C:/game/docs/performance/recoverable-compile-oom-design-20261027.txt)和[源锚点](C:/game/docs/performance/recoverable-compile-oom-source-anchors-20261027.json)是设计交付，不是已实现行为。fatal-OOM 与 automatic retry 均未应用；阶段 5 的完整有界与恢复语义仍开放。

## 下一步及环境约束

1. Rhapsody 最终 f39 两种 all-target 类型检查及 heap 2-test 诊断已补齐；解除编译内存约束后完成最新 Tiny 与必要 workspace 检查，再执行 canonical 5 / local-surface 6 / pending-acquire 9、Tiny observer 2 和 EffectsHost lifecycle 1 test。ignored、零命中和 Vulkan 初始化失败均不算通过。
2. 用受控的新 canonical / 同前缀夹具冻结程序，保留旧 surface 失败，取得 native input/像素闭环后再冻结共同候选；common4 隔离基线尚未激活。
3. 完成 Release ETW、探针开销 ≤1%、四场景 600/3000 AB/BA/AB、成本转移审查、压力与长跑；未取得正式 CPU 收益或 p95 ≤16ms 结论。WSI 用户暂缓，macOS 待验。

控制会话早期观察为 32GiB RAM、无 pagefile，可用 commit 约 1.4–1.8GiB，保留为历史范围。主线程约 2026-10-26 18:30 UTC 的只读样本为 commit 4,824,317,952 bytes / physical 6,015,111,168 bytes；后续封存的 `current-reader-memory-2.json` 又降至 commit **1,208,508,416 bytes（约 1.13GiB）** / physical 3,220,877,312 bytes。后一个 reader 的 immediate Job flags=10240，未配置 process/job memory limit；这不是祖先 Job 枚举，也不是历史失败编译器的 peak。主线程已停止进一步 native 或重 Tiny 编译。请用户手动关闭不用的高内存程序以释放约 8GB 可提交内存的问题仍待答复；没有请求或应用 pagefile 更改，未更改系统设置、终止无关应用或推送。

机器可读范围、原始文件大小和 SHA256 见 [status.json](C:/game/docs/performance/status.json) 的 `latest_progress_20261027_f39dc61`。此前 c98 与更早记录作为历史保留。

## 此次读取的证据 SHA256

- `rhapsody_observers_valid`：[result.json](C:/game/.perf/w1-rhapsody-observers-jobdiag-6d1cf8a-1/result.json)，`1445d944d3de971d8bc1510132057773f384ee147655dc5782a0f3b69f09cd82`（8497 bytes）。
- `rhapsody_observers_test_log`：[test.stdout.log](C:/game/.perf/w1-rhapsody-observers-jobdiag-6d1cf8a-1/test.stdout.log)，`2251f3ab9dc2730e62972ead8b63e5c2b2b446400d69d8d96fb7a7f52d4c490c`（336 bytes）。
- `rhapsody_observers_source_before`：[source-before.json](C:/game/.perf/w1-rhapsody-observers-jobdiag-6d1cf8a-1/source-before.json)，`039a390410d4cdef5c6acf4a949fcb1a6e9498de9fa459430135b8b912d9e22a`（359237 bytes）。
- `rhapsody_observers_source_after`：[source-after.json](C:/game/.perf/w1-rhapsody-observers-jobdiag-6d1cf8a-1/source-after.json)，`039a390410d4cdef5c6acf4a949fcb1a6e9498de9fa459430135b8b912d9e22a`（359237 bytes）。
- `rhapsody_observers_original_job_failure`：[result.json](C:/game/.perf/w1-rhapsody-observers-6d1cf8a/result.json)，`f146592295dfd518ecf87b3f395363610357d7e830f101c6cbf9ac0d2c8f7a5d`（8005 bytes）。
- `tiny_observers_build_failure`：[result.json](C:/game/.perf/w1-tiny-target-observers-jobdiag-6d1cf8a-1/result.json)，`f644c18493c01ad6838a6721e773fb2184c799c92e0d5fa8dfe55a8a35c3e7da`（4262 bytes）。
- `tiny_observers_compiler_log`：[cargo.stderr.log](C:/game/.perf/w1-tiny-target-observers-jobdiag-6d1cf8a-1/cargo.stderr.log)，`1a22271846dcc688b14a6465cd13afb0e0637e19e2fdad3bfcbb3277106ae422`（31788 bytes）。
- `rhapsody_probe_initial_type_error`：[pending-surface-suballoc-rhapsody-probe-check.log](C:/game/.perf/pending-surface-suballoc-rhapsody-probe-check.log)，`43d4506b9817479fb00539ac775bade7175efc19c1d569dcaaa952694651b856`（902 bytes）。
- `rhapsody_probe_typecheck_after_fix`：[pending-surface-suballoc-rhapsody-probe-check-2.log](C:/game/.perf/pending-surface-suballoc-rhapsody-probe-check-2.log)，`546a2e90ba21add9a38d8fd259f93fe8e989899f899601aabdbcbc4cacf48d9c`（1788 bytes）。
- `rhapsody_default_typecheck`：[pending-surface-suballoc-rhapsody-default-check.log](C:/game/.perf/pending-surface-suballoc-rhapsody-default-check.log)，`812091c86d46654bd72e5965327169fbd880a3d8ccea1ac8dae57f4bc3f0ad7e`（1788 bytes）。
- `f39_rhapsody_probe_all_targets`：[f39-rhapsody-probe-all-targets-check.log](C:/game/.perf/f39-rhapsody-probe-all-targets-check.log)，`0955dff4feaad7d98fda9c3cf72dc4805f6dce99a5f0469826b7439733ccf71f`（1788 bytes）。
- `f39_rhapsody_default_all_targets`：[f39-rhapsody-default-all-targets-check.log](C:/game/.perf/f39-rhapsody-default-all-targets-check.log)，`da3abe6b88110ab34fd4775a7e767ee79e8b97a3e6ba5a416d3eb24a50697945`（1788 bytes）。
- `python_63`：[surface-integrated-python-tests.log](C:/game/.perf/surface-integrated-python-tests.log)，`55589a121583d9048584930b8707e6ab884b2aa93c7f01cf4c24040973b923d2`（9199 bytes）。
- `tiny_canonical_typecheck_oom`：[integrated-surface-canonical-tiny-typecheck.log](C:/game/.perf/integrated-surface-canonical-tiny-typecheck.log)，`bce186ef47a8065af6ddd62e1ab6e9c8eedfdbfb454689e568b842e002e5a2d6`（14369 bytes）。
- `c98_surface_audit`：[audit.json](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/surface-c98-update/audit.json)，`c8b5ddfa7792d4a7dbef02d4ab28ead8e85f5f971247f74944b0c2d67040ec73`（19636 bytes）。
- `c98_surface_summary`：[summary.md](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/surface-c98-update/summary.md)，`9aa11a1676d37161886c55844c38ca19f91f5075867cd64b50a645b9ddae1552`（3397 bytes）。
- `c98_trace_summary`：[summary.json](C:/game/.perf/binding-trace-source/rich-workload-debug-c98be8e-trace-1/summary.json)，`4384d58f4506d06853f5a3013203a8330525055fab7e08d3e9a716ac48d31725`（177822 bytes）。
- `tools_canonical_135`：[canonical-cold-tools-integrated-regressions.log](C:/game/.perf/canonical-cold-tools-integrated-regressions.log)，`8611a4bfc715dbd7675073a25e604e72994da045a3b31321a67f8790494b6148`（241 bytes）。
- `tools_binding_44`：[binding-canonical-integrated-regressions.log](C:/game/.perf/binding-canonical-integrated-regressions.log)，`0f06c5028e5fce9fda358d45326390b985066fc4518b2bbdca423eb665edebdf`（148 bytes）。
- `tools_surface_original_91_errors2`：[surface-local-freeze-integrated-regressions.log](C:/game/.perf/surface-local-freeze-integrated-regressions.log)，`d9c1bc3a471a062aa4c3e1716081b94aa095fdf4cfdfa1ff6858b9380737af0b`（3458 bytes）。
- `tools_surface_lf_fixed_94`：[surface-local-freeze-integrated-regressions-lf-fixed.log](C:/game/.perf/surface-local-freeze-integrated-regressions-lf-fixed.log)，`4eb97fa29c0dac788c44ac8ebd4d6f2d6683ca751cb28a2f64262cc152b4ca39`（199 bytes）。
- `effects_host_latest_source`：[effects_host.rs](C:/game/crates/tiny-glade/src/effects_host.rs)，`e55ecbf7e02331a5e3b0c9b93c4b0e8e26b078aaa50a80bd8c50d32f648bed7c`（14531 bytes）。
- `effects_host_native_test_source`：[original_scene_compute_gpu.rs](C:/game/crates/tiny-glade/tests/original_scene_compute_gpu.rs)，`c2ddf26803539382e3b7e06ea9f6d607112515e4b4c258e426f914cb7602b850`（99695 bytes）。
- `memory_latest_read_only`：[current-reader-memory-2.json](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/w1-memory-and-future-guard/current-reader-memory-2.json)，`7515932266e85b77e213740ee3386d3a41a0979f64389feccd23843254dcb9d8`（1707 bytes）。
- `recoverable_oom_design_only`：[recoverable-compile-oom-design-20261027.txt](C:/game/docs/performance/recoverable-compile-oom-design-20261027.txt)，`74fae5c396309bb86a99b39f1e204721f25277be2231339b4054e049ab072aee`（18275 bytes）。
- `recoverable_oom_design_source_anchors`：[recoverable-compile-oom-source-anchors-20261027.json](C:/game/docs/performance/recoverable-compile-oom-source-anchors-20261027.json)，`631ca91fa84c8bc022ceaae8d28c738fca70141a30d903fd0a58c5cf95ed450d`（4410 bytes）。
- `pending9_result.json`：[result.json](C:/game/.perf/rhap-pending-acquire-storage-f39dc61-bounded-1/result.json)，`853b967b1ab9c20a0146e74da0e0afdd9a47abcbe7fd06158886f07691bab0d6`（5736 bytes）。
- `pending9_source-before.json`：[source-before.json](C:/game/.perf/rhap-pending-acquire-storage-f39dc61-bounded-1/source-before.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `pending9_source-after.json`：[source-after.json](C:/game/.perf/rhap-pending-acquire-storage-f39dc61-bounded-1/source-after.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `pending9_owned-job-observations.jsonl`：[owned-job-observations.jsonl](C:/game/.perf/rhap-pending-acquire-storage-f39dc61-bounded-1/owned-job-observations.jsonl)，`a5819453fff8c12c2b321306dfd8c16f3c87250eb444ffa023b0d78c29edc9dc`（1857 bytes）。
- `pending9_cargo.stderr.log`：[cargo.stderr.log](C:/game/.perf/rhap-pending-acquire-storage-f39dc61-bounded-1/cargo.stderr.log)，`b12c9d963de03b2fb8af3eb9a20801610b344c78df5ae9577c8b9e1521cf8f01`（5074 bytes）。
- `heap_round1_result.json`：[result.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/result.json)，`5bd9a0e08e90ad96d62f54d393106be2c134635f55ed6ea69e0d063edeb4d367`（21359 bytes）。
- `heap_round1_source-before.json`：[source-before.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/source-before.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `heap_round1_source-after.json`：[source-after.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/source-after.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `heap_round1_owned-job-observations.jsonl`：[owned-job-observations.jsonl](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/owned-job-observations.jsonl)，`403d6a9f2fd25fc32eb8446bb40c2ebf93157f1e4bec71a557d92cc758c69e04`（40088 bytes）。
- `heap_round1_test.stdout.log`：[test.stdout.log](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/test.stdout.log)，`64290375ab9d08f75409f1df98ec8bc6d830550abc452c43dc7053ba0bc46e08`（8707 bytes）。
- `heap_round1_test.stderr.log`：[test.stderr.log](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-1/test.stderr.log)，`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`（0 bytes）。
- `heap_round2_result.json`：[result.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/result.json)，`3bb8b732c6463ac3b162c0b28391d9194adfa7a23b9a778edeb5eed9bf7a32ad`（21240 bytes）。
- `heap_round2_source-before.json`：[source-before.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/source-before.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `heap_round2_source-after.json`：[source-after.json](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/source-after.json)，`ae152138529ac892763168e7bab919a1ad46a97f3398e9eddb3a078e4c07ca17`（360105 bytes）。
- `heap_round2_owned-job-observations.jsonl`：[owned-job-observations.jsonl](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/owned-job-observations.jsonl)，`280f6a16b76d9c87e730b6b6f71e054210cae6c53492fc2e64b50a626bd1de44`（1829 bytes）。
- `heap_round2_test.stdout.log`：[test.stdout.log](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/test.stdout.log)，`8b9e1835977db2bfdf00d838c8305b0e187060f99bc6c1c38f7bcafd2b335eaa`（8707 bytes）。
- `heap_round2_test.stderr.log`：[test.stderr.log](C:/game/.perf/rhap-suballocation-drop-heap-f39dc61-bounded-2/test.stderr.log)，`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`（0 bytes）。
