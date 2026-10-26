# 当前进度和未完成任务（2026-10-26）

代码候选已推进到阶段 5；Windows 完整四场景基线与最终性能验收尚未完成。因此不按投入工时计算完成率，也不宣布已达到 16ms。WSI 兼容问题按用户要求暂缓。

## 已实现的源码候选

| 内容 | 提交 | 已有证据 |
|---|---|---|
| VAO/纹理 view 旧 owner 锁外释放；动态常量锁外写入 | b792e4c | 生命周期及像素回归 |
| committed graph current_version 索引 | 50d8885 | 随机图与旧扫描对照、资源契约；10 万次 debug 查询约 3.793s → 5.303ms，仅局部诊断 |
| 单次分配共用完成快照与回收配额 | c5171f3 | 配额耗尽后不刷新，后续维护继续推进 |
| 草、UI/world-space、水冰 surface Collect 收敛 | 1da8d40 | 260 项 draw-pass 单测，真实 worker、页边界及稳定顺序 |
| 原生对象创建前预留退役 ticket；跨队列迁移、忙池重试；统计释放无分配 | 67ddafa | 445 项 Rhapsody 单测通过、2 个 ignored 不计通过；55 项集成通过，含 6 项 reservation 诊断 |
| 注入 DeviceLost 前完成真实上传的夹具修复 | 53e9bb1 | core_raster_draw 12、original_raster_gpu 2、vao_scope 1 再次通过 |
| 真实游戏图按 session 自有队列限流，修复夹具过期回执 | 53bbbef | 1280×720、30 帧、验证层开启，720 Collect jobs /90 cross-pass jobs，存活 descriptor owners 与预期一致 |

这些是有正确性证据的候选，尚无可接受的整帧提升百分比。默认依赖、分配器、画质和工作量规则未改变。

## 尚需实现或定位

| 顺序 | 任务 | 完成条件 |
|---|---|---|
| P0 | 静态帧账单：对齐实际 draws、唯一变体/VAO/物理 owner、原生 bind/writes、分类字节与阶段区间 | 用完成点/ETW 计算独占线程 CPU，等待和多线程累计单列，拒绝嵌套重复计时 |
| P1 | Collect scope 不可变反射快照，准入和 worker 共用 | 查询按唯一 shader/variant 增长；保持 App、热替换、预算及 owner 生命周期 |
| P1 | 查清两次 VAO 刷新消费者，按失效原因缩小枚举和发布 | 保留 terrain 中途替表、真实 buffer generation 和物理 owner 校验，不能直接删一遍 |
| P1 | 提交/合并 scratch 有界独占租借，按唯一资源量预留 | 无跨帧 graph ID/guard 泄漏，保持有序封存集合、透明和副作用顺序 |
| P1 | 常量块和预算 credit 按任务领取/归还；owner uniform chunks 批量处理 | 清零、拒收、失败保脏、取消归还及完成点保活不变；既有 callback 参数 chunks 批量领取不重复重做 |
| P2 | 静态 draw/material/VAO 描述按变化更新 | 先证明全量提取/排序是热点；覆盖相机、可见性、增删实体、pass 分类、时间和资源代际 |
| P2 | 绑定资源模板与动态参数分离，完整身份成立时复用 | offset/size/view/layout/代际、frame globals、页与 lease 一致；不盲用 previous draw |
| P2 | callback 内兼容 descriptor-set bind 去重 | 真实 set/heap/layout 兼容；每 draw 声明校验、保活和 parameter_index push 继续执行 |
| P2 | 计划命中时 declare/match/instantiate 分段定位 | 仅在证据支持时做稳定声明模板；现有版本索引和两形状 projection 应保留 |
| 待 profile | 上传 payload 锁外准备、COW budget lease、view miss 锁外创建/二次校验、缓存淘汰锁外释放 | 独立证明成本与正确性；view 创建和释放竞争的代际拒绝需要单独处理 |

P2 是调查候选，不代表全部都应进入默认实现。没有稳定收益的性能候选不作发布放行；正确性修复单独评估成本。

## 已做代码但尚未完成的验收

1. 测量闭环：游戏负载下 ETW/独立 OS 累计偏差 <=2%，探针开关均值影响 <=1%；失败/取消、App 切换、暂停恢复和容量溢出都要真实运行验证。
2. Windows 四场景基线：静态、大量绘制实际 >=1000 条、每 30 帧局部编辑、每帧连续编辑。固定 1280×720、scale 1、1/60、原画质与工作量；每轮 600 预热 +3000 有效观察帧，独立三轮。
3. 热点归因：干净测量、ETW 栈、分配诊断、GPU 正确性分开；确认 projection 的实际复用及每项改动的目标成本下降，同时检查主线程、worker、进程 CPU 和 GPU。
4. Collect：30 帧真实游戏图已证明 worker/cross-pass 实际执行；剩余透明 water/ice 的代表性像素/命令对照、真实取消、预算耗尽和恢复，以及四场景性能回归。
5. 压力回收：更全面的部分创建/提交、DeviceLost、析构重入、持续内存压力和解除后预算回落；公开 legacy raw-handle Device::retire 仍有无预留节点的同步 HostOOM 兜底，内部资源 Drop 已迁出该入口。
6. 冻结 release 基线/候选和 EXE/PDB 哈希；当前没有通过验收的冻结 release 候选。构建内存问题需用固定单任务配置处理，不升级依赖或更改系统分页。
7. 最终 AB/BA/AB：四场景每轮 render CPU p95 <=16ms；任一 p95/p99 回退 >5% 拒绝，接近阈值扩至五轮。
8. 稳定负载、效果切换、resize 各至少 30 分钟且 >=10000 帧；覆盖 App 切换、全屏、暂停、截图取消、Pending 关闭和晚相机输入；验证完整释放、验证层及像素。
9. macOS 编译和记录器回归；无 macOS 实跑的项目保留待验。

## HTML 审查结论

用户指定的 static-frame-cpu-report.html 已由独立 agent 核对：它是源码审计与测量设计，清单 executed_game_or_benchmarks=false，没有新的静态帧 counts/bytes/阶段 CPU/FPS。在 agent 审查时，报告哈希匹配，22 个引用中 21 个匹配，profiling.rs 已随退役改动变化。硬件、时钟校准、窗口失败日志均匹配。因此上述候选按验证优先级排序，不能按“实测毫秒收益”排序。


## 新增真实运行诊断的边界

30 帧 headless 回归在 debug、Vulkan 验证层和阶段日志开启下通过；强制要求 cross-pass 任务发生。它包含截图和历史收敛，不是 600/3000 帧的静态干净基线，也不是 Windows WSI 运行。原始记录：.perf/candidate-headless-gameplay-receipts.log；图像、分配清单与最后一帧 pass 记录位于 .perf/candidate-headless-gameplay/。EXE/PDB 与证据哈希见 status.json。

本轮保留两次失败证据：并行编译内存不足，以及夹具重复保存回执、与 session 自动回收不同步。后者改用现有 wait_gpu_queue_trim + collect_completed_frames，保留在飞帧数限制和全部原断言后通过。不能删除失败日志后把第一次运行计为成功。

可优先定位的短程信号是 UI 2D / Shadows / G-buffer 录制、declare 的 VAO/准入/等待/合并，以及计划 miss 原因。最后一帧 UI 2D 50.510ms、Shadows 33.304ms（3 次累计）、G-buffer 22.994ms 都是 callback 墙钟；跨录制线程不能相加成 owner CPU。日志的阶段墙钟求和 p95 也不是渲染线程 CPU p95。这些数值只决定下一轮诊断入口，不证明优化收益。
