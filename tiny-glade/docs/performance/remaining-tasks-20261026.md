# 当前进度和未完成任务（2026-10-26）

**源码候选已分项提交并有真实测试证据；测量闭环、候选性能和 Collect 完整验收仍未完成。** 当前不能报告整帧收益或 16ms 达标。旧 Release ON/OFF 仍冻结在 `29095eb`，不包含新候选。

新提交为：`a84ad66` 测量身份/诊断工具、`44956f8` CPU 预算 owner、`b79443e` Collect 反射 scope、`cca55ad` typed VAO 查询、`6ecdb2f` 同帧 VAO wrapper、`19c52eb` view generation/FrameGlobals、`c5a6125` 无损 trace archive 工具；新增 `709d717` 常量批量、`a36fbd3` 有界 Commit scratch、`6a85ec8` 声明首错保留及公开 `into_parts` 四元组消费者。`f4e3e04` 已提交 synthetic queue-family 夹具修复，`fb778bd` 已提交 surface 对照开关与 worker 顺序测试；`e38ae7f` 已提交 management modes、常量/scratch 账单及累计 CPU API 的 QPC brackets；`fa90e4a` 已提交原 rocky-lake 水/冰生产图捕获夹具。当前源码提交至 `fa90e4a`，未冻结新性能候选。

新 Graph API 的默认全目标编译通过（2m40s），probe 全目标也通过（2m36s）。修复后实际 Vulkan/验证层运行的 **core16 为 16 pass/0 fail/0 ignored**，新增 graph 首错 5 项为 **5 pass/0 fail/0 ignored**，两轮均无 VUID。`f4e3e04` 修复 synthetic 假 family 1→0 向实际单 family GPU 提交后，全 Rhapsody lib 严格复验为 **458 pass/0 fail/2 ignored，全日志无 VUID/Validation Error**；ignored 不计通过。原 core 14/2、首轮全 lib 的 buffer-09095 VUID 和中间夹具失败日志保留，不能追溯改为通过；真实不同 queue family 硬件路径仍待验。

country-core 的 pass_builder 166 项（含 scratch 3 项及拒收）、lease owner 1 项、draw-passes 262 项均通过，0 fail/0 ignored。以上是分项正确性/编译证据；代表性水/冰真实 GPU 像素对照、四场景收益与稳定性仍未验。

更早的 Rhapsody 453、tiny 默认常量 9、反射 scope 6、typed VAO 3、原生常量 3、纹理 4、原 shader GPU 2、VAO scope 4 成功分别属于其检查快照，不能自动覆盖后续源码；最新全库重跑结果单独登记。当前工具为 `c5a6125`，76 项通过、0 skip；此前 63 项摘要和原始日志保留。所有候选收益仍未验。

## 测量闭环的实际状态

冻结的是来源 `29095eb` 的同配置 ON/OFF Release **集成测试 EXE/PDB**，默认 mimalloc、`-j1`、`CARGO_INCREMENTAL=0`、相同锁文件/工具链/驱动。二者请求的测试目标均生成成功；自动选中的其他游戏 binary 编译被所属 Job 停止，不能称整次 Cargo 构建或生产窗口通过。冻结身份见 [构建配对记录](C:/game/.perf/headless-builds/closure-feature-pair.json)。

| 已运行的闭环检查 | 结果 |
|---|---|
| 60 预热 /300 采样，原始 recorder | 300/300，导出完整；ETW owner/进程与 OS 累计值均超 2%，该轮 invalid |
| 600 预热 /3000 采样，owner ETW 对 GetThreadTimes | 相对差异 **0.53423%**，这一项通过 |
| 同一长轮，进程 ETW 对 GetProcessTimes | 相对差异 **4.75882%**，失败，**整轮仍 invalid** |
| 同一长轮，自算进程 ETW 对 Microsoft 全进程 CSwitch 积分 | 相对差异 **0.0003502%**；同源 ETW 交叉证据不能覆盖独立 OS 失败 |
| 完整 feature-off 探针开关均值影响 ≤1% | 尚未放行 |

详细原始值和文件身份见 [Windows 记账调查](C:/game/docs/performance/windows-clock-accounting-20261026.md)。两个运行的预热与真实模拟起点不同，不能拿其均值相减当性能 A/B。App/update 与渲染共用 headless owner，其他线程 CPU 包含 worker、FMOD、驱动/runtime helpers，不能统称 worker。该入口不替代 Windows 生产渲染线程或 WSI 验收。

## 已提交的早期源码候选

下表证据属于 **`53bbbef` 及以前的源码**。本轮修改后必须重跑，不能沿用这些数量宣布当前候选通过。

| 内容 | 提交 | 历史证据 |
|---|---|---|
| VAO/纹理 view 旧 owner 锁外释放；动态常量锁外写入 | b792e4c | 生命周期及像素回归 |
| committed graph current_version 索引 | 50d8885 | 随机图/资源契约；10 万次 debug 查询约 3.793s→5.303ms，仅局部查询诊断 |
| 单次分配共用完成快照及回收配额 | c5171f3 | 配额耗尽后不刷新，后续维护继续推进 |
| 草、UI/world-space、水冰 surface 的 worker Collect | 1da8d40 | 260 项 draw-pass 单测；页边界、真实 worker、稳定顺序 |
| 原生创建前预留退役节点、跨队列迁移、忙池重试、统计释放 | 67ddafa | 445 项 Rhapsody 单测、55 项集成通过；2 个 ignored 不计通过 |
| DeviceLost 注入前完成真实上传 | 53e9bb1 | core_raster_draw 12、original_raster_gpu 2、vao_scope 1 再跑通过 |
| 真实游戏图使用 session 自有队列限流 | 53bbbef | 1280×720、30 帧，实际 720 Collect /90 cross-pass jobs；正确性诊断 |

## 本轮已应用候选与分项验证

| 实际源码变更 | 保留的边界与未完成验收 |
|---|---|
| Collect scope 的不可变 `Arc<ReflectionScope>`，准入及 worker 共享 `(shader,variant)` 反射；本地 32MiB 上限并预留真实 CPU capacity/rehash peak | 固定单 draw 保留原 resolver；每 draw 的返回 Arc 仍存在；原全局 ReflectionCache 未因此全部纳入预算。6 项 scope 单测通过；完整生命周期/GPU/四场景实测仍待验 |
| 由 DrawListItem 的实体读取真实 typed VAO；仅无法解析项回退 ECS 扫描；同帧 full→terrain subset→full 借用旧 wrapper 作为候选 | 两次刷新及地形中途替表仍保留；每次重新校验真实 generation/物理 owner，不是跨帧直接复用原生对象。表 backing 与 owner 有 CPU reservation；typed 3 项及 VAO scope GPU 4 项通过，实际收益待测 |
| 常量块按任务领取库存、批量归还；owner 每批最多 64 项集中领取动态常量区间，锁外写入与 owner 替换 | opaque `CpuBytes`/非 Clone credit 保持物理 backing 生命周期；超界回退逐项协议；写失败保留 CPU lease；默认 pool 9 项及 native 常量 3 项通过，修复后 core16 全部通过；仍需完整帧及性能对照 |
| Commit/remap/merge scratch 接入已有有界 CPU pool，独占 lease；最多 8 个 lease、总量 8MiB、缓存 1MiB，单 lease 1MiB | 按 binding/access 实际 domain 预留；归还时清空 graph IDs/owner，不能跨图保留。底层 CPU storage/bytes/scratch 新增 8 项 lib 测试通过；修复后 core16/graph5 通过；完整帧及性能仍需验证 |
| texture view miss 锁外原生创建；发布时重验完整 generation 和物理 owner，重复/过期 candidate 锁外释放 | 这是生命周期正确性修复；允许竞争产生重复原生 candidate，4 项纹理生命周期测试通过；成本未量化，不能宣布性能收益 |
| FrameGlobals 锁内只取/换 Arc，表遍历及旧 owner 释放移到锁外 | 多一次 Arc 引用操作；真实全局 buffer/offset/size、页生命周期不变，原 shader GPU 2 项通过；完整帧/并发回归及收益仍待测 |
| 水/冰 test-support 独立 inline/真实 worker 对照；distinct VAO 顺序测试；透明像素、真实未提交图取消、CPU/native 常量拒收与写失败恢复测试源码 | 修复声明首错传播后 core16 全部通过，严格验证无 VUID；原 14/2 失败日志保留。取消覆盖 Collect 完成、owner 尚未提交的现有边界，不声称支持运行中 worker 中断 |
| 原 rocky-lake Autumn 水 /Winter 冰代表性场景夹具和原始 RGBA/命令记录 | 源码已加入；须实际运行并严格比较同季节、相同相机/时钟/工作量的 inline/Collect，再做候选回归 |

Graph 首次声明错误保留与 `into_parts` 四元组消费者已在 `6a85ec8` 提交，默认全目标、core16 和新增 graph5 通过；未改断言接受错误的错误类型。新 API 的 probe 与修正夹具后的全库验证层复验已通过；代表性场景及性能仍开放。

这些项目目前是未放行的组合候选，不代表每项有独立稳定收益。测量 build 的开关用于归因；最终默认路径须同时满足正确性、相同工作量及性能门槛。

## 余下工作，按依赖推进

1. **先关闭测量缺口**：外部逐 TID API/QPC brackets、退出线程 handle 保留及完整覆盖检查已有真实结果。一次同工作量 quick pair 的 burst 是 OS **多计**（worker 1453.125ms 对 ETW 993.6653ms，差 31.6187%）；游戏外部诊断中各线程有正有负，聚合仅约 1.56% 是抵消，不能放行全部 worker 或覆盖原 3000 帧 invalid。helper 17 项通过；预声明的长校准 AB/BA/AB 三对已完成，4 workers、每 worker 4096×1MiB、burst 等待 10ms；三个 continuous 进程 signed 差异为 +0.2859%、+0.5298%、+0.1883%，三个 burst 为 -11.4130%、-13.7380%、-18.7441%，均完整覆盖、相同工作量签名。后者 OS 少计，与 quick 单 worker 的多计方向不同，不能套统一修正因子。clock policy 尚未放行，仍需先审查再重新冻结、重跑干净 A/B。原 2% 和 16ms gate 未改，旧轮未重分类。
2. **修复并重验当前组合**：新声明错误传播 API 默认/probe 全目标、graph5、core16 已通过；修正 synthetic queue-family 夹具后，全 lib 严格复验 458 pass/2 ignored 且无验证错误。原失败证据保留。真实跨 family 硬件仍待验。GPU 初始化失败、零测试、ignored 不计通过。
3. **取得有效账单后量化未变化管理成本**：保留自主动物/配件变化，分开 draw/resource 重复管理与实际变化；统计 draw 数、唯一 shader/variant/VAO/owner、常量/绑定/声明次数和独占阶段 CPU。旧 debug 差分每帧为 777 次 reflection 查询（全 hits）、384 次 constant takes、19,636B collected uniform writes、1301 个 resource inputs、361 次 bindless draw prepare；VAO 两次 full refresh 汇总 344 个 list entries、190 次实体查询、128 次 unique visits、3 次发布、63 个新 wrapper。118 是 cache 存量，128 是两次刷新累计访问，均不能当整帧独立资源数。这些是路径计数，不是毫秒成本。
4. **逐项候选对照与选择**：对反射、VAO、常量批量和 scratch，先定向证明目标成本下降，再四场景比较。不稳定或回退项目不得直接以“已实现”进入最终默认候选。
5. **四场景真实工作量与性能**：固定 1280×720、scale1、1/60、原画质；static、实际 draw-list ≥1000 的 many_draws、每 30 帧 PathRaster 编辑、每帧编辑；600 预热/3000 有效帧、各三轮。配对起止、轨迹、draw/task/dirty/upload 字节必须一致。
6. **完成 Collect 验收**：严格水/冰原始像素与命令对照、真实取消、CPU budget/native allocation/write 拒收、部分 64 draws 保留及恢复。Autumn 与 Winter 分别对照；不能用更改季节作为 A/B。四场景无退化仍需有效测量。
7. **其余所有权候选先测后定**：绑定模板/静态 draw 描述、上传 payload 锁外准备、COW budget lease、绑定 requirement 扫描/兼容 bind 去重。现在没有足够的重复率、锁等待、实际 copy/descriptor 成本证据，尚未实现这些大范围缓存或协议改动。
8. **压力/最终验收**：持续压力、完整故障注入及解除后预算回落；legacy raw-handle retire 同步兜底仍开放。最终冻结候选后 AB/BA/AB，四场景每轮 render CPU p95≤16ms；p95/p99 回退 >5% 拒绝。三类至少 30 分钟/10000 帧长跑及全部窗口生命周期待验。
9. **平台边界**：WSI 的 swapchainMaintenance1 兼容问题按用户要求暂缓；macOS 编译/记录器及运行回归仍待验。headless 成功不能替代这两项。

## 证据与交付

历史失败记录保留：[debug 动态输入诊断](C:/game/.perf/closure-debug-smoke.log)、[短 ETW 轮](C:/game/.perf/headless-runs/closure-etw-smoke/run.json)、[长 ETW 轮](C:/game/.perf/headless-runs/closure-static-long-1/run.json)、[本轮 check1](C:/game/.perf/candidate-ownership-default-check.log)、[本轮 check2](C:/game/.perf/candidate-ownership-default-check-2.log)、[本轮 check3](C:/game/.perf/candidate-ownership-default-check-3.log)。本轮新证据：[default check4](C:/game/.perf/candidate-ownership-default-check-4.log)、[probe check](C:/game/.perf/candidate-ownership-probe-check.log)、[完整 63 项工具测试](C:/game/.perf/tool-integration-20261026-tests-full.log)、[新源码 Rhapsody 453 项 lib 测试](C:/game/.perf/candidate-ownership-rhapsody-lib.log)。更早的 Drop 64B 分配、并行编译 OOM、旧回执夹具失败及修复后证据继续登记在 [status.json](C:/game/docs/performance/status.json)。历史 30 帧阶段墙钟不能当线程 CPU p95；用户 HTML 是源码审计设计，未执行新的整帧测量。

用户已授权全部完成后交付 `git@github.com:yuluo-zy/games_test.git`，目标新分支 `codex/perf-windows-20261026`；**尚未推送**。真实 ancestry 是远程 `8fd2a3c`→本地来源 `6957077f9`。发布须保留该上游历史，将独立 Cgame `b91d512` 之后的变更映射到 `tiny-glade/`，显式处理四文件 CRLF/LF 字节差异；保留未复制的 RE/资产/报告，不能把缺失文件视为删除或 force 推送孤立 import 根。发布前重查远程并核验映射源码身份及阶段提交对应关系。

## 新增时钟诊断与失败边界

外部诊断使用旧冻结 EXE，600 预热/600 采样；ready/go 至进程退出的外窗为 86.3495494s，包含清单、drain、关闭，不能与内部 601 帧 envelope 混用。保留并终止后读取 21 个线程对象；OS 线程累计和恰等于进程值 26796.875ms。进程 OS/ETW 差异 1.5634%（另一 sandwich 窗 1.5556%）并未证明逐线程准确：compute(0) 多计 591.1493ms，而 unnamed/BVH 少计 818.4044/666.8491ms。微秒 API bracket 及约毫秒级窗差不能解释这些偏差；仅支持负载模式相关记账失真，不宣称已证明 Windows 内部算法。

真实记录：[game thread-cpu.json](C:/game/.perf/thread-cpu-diagnostics/thread-clock-game-w600-n600-1/thread-cpu.json)、[quick-pair summary](C:/game/.perf/thread-cpu-calibrations/thread-clock-quick-1/summary.json)。失败的 [core16](C:/game/.perf/candidate-ownership-raster-gpu.log) 和 [OOM 被 UninitializedRead 覆盖的诊断](C:/game/.perf/candidate-budget-error-diagnostic.log) 均保留；[新声明 API 默认全目标](C:/game/.perf/declaration-rejection-workspace-check.log) 仅是编译通过。[当前 76 项工具记录](C:/game/.perf/trace-archive-tools-20261026.json) 验证归档无损/身份/Job 等工具行为，不是性能验收。新 archive 只处理符合协议的已完成新 capture，不回写旧失败结果。

## 最新 Graph 与长校准证据

[修复后 core16](C:/game/.perf/declaration-rejection-core-raster.log) 和 [graph 首错 5 项](C:/game/.perf/declaration-rejection-five-strict.log) 为实际严格 Vulkan 成功；[全 lib 验证错误](C:/game/.perf/declaration-rejection-rhapsody-lib.log) 仍是失败证据。ignored 不计成功，不能忽略验证层错误。

[预声明三对长校准](C:/game/.perf/thread-cpu-calibrations/thread-clock-4w-long-1/summary.json) 的六轮均已完成，continuous 全在既定 2% 内，burst 三轮全超限，原始 OS/ETW 差异保留。这些是时钟诊断，不是优化性能 A/B；不改变既有 gate、不重分类旧 invalid 轮。

## 冻结前最新正确性快照

`f4e3e04` 后的 [全 Rhapsody 严格复验](C:/game/.perf/rhapsody-validation-fixtures-fixed-2.log) 为 458/0/2 ignored、无 VUID/Validation Error；[新 API probe 全目标](C:/game/.perf/declaration-rejection-probe-check.log) 通过。[pass_builder 166](C:/game/.perf/candidate-country-pass-builder.log)、[lease 1](C:/game/.perf/candidate-country-1.log)、[draw-passes 262](C:/game/.perf/candidate-country-262.log) 均通过且 0 ignored。`fb778bd` 的 surface inline/Collect 控制及顺序测试、`e38ae7f` 的测量模式/API brackets、`fa90e4a` 的原湖水/冰生产图夹具均已提交；真实代表性水/冰 fixture 尚未运行，GPU 像素仍待跑。

clock policy 只有 staging 实现的 92 项测试和独立审查证据；审查发现 P2 正在修复，未放行、未改变旧 gate，也没有闭环或性能验收。本文档本批完成后停止实际写入，供冻结源码身份；后续记录先暂存。
