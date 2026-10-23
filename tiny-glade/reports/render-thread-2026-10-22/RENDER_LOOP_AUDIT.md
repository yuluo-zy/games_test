# 完整渲染循环复核（2026-10-23）

用户授权增加一个只读审计 agent，从主循环、提取、声明、编译、录制、呈现直到结果回收，核对堵塞、串行及旧路径。以下区分已修代码、局部验证与剩余设计工作；本报告不表示全部重构完成。

## 已发现并修正

1. **共享池录制权限漏洞（P1）**：Collect 有 ECS 准入，图录制没有；默认 TaskPool scope 会在渲染线程执行无关 ECS 任务，可能以渲染线程身份越过下一帧上传的读取门禁。`rg/parallel.rs` 改用 `scope_with_executor(false, None)`，只在调用方执行本 scope 的任务，稳定索引合并；`render_bridge.rs` 两个 execute 入口同时取得准入，竞争时 Inline，避免共享池全被上传任务占用时互等。RAII 恢复 runner，保留 chunk/计划键；正常、错误、unwind 均需验证。agent 对补丁复核通过。后端单元 421、p0 2、分块 8、并行图 3 通过；宿主回归在执行。
2. **加载帧无条件 reclaim**：`GameLoop::frame` 原先每次准备轮询前 settle，即使 gameplay 未完成，也迫使加载 Update 等待渲染。拆出 `ready_to_handoff`，未就绪仅推进 Logo/准备状态，真正换 App 前才 settle。保留旧 World 的反馈与资源移交顺序；代码已修，完整加载/窗口循环待复验。
3. **手动缩放与原版帧尺寸发布缺失**：补齐两 App 的 PreUpdate、resize、OOM 的统一刷新，output 截断，inner 从未截断值乘 TGSR 比例后按 32 对齐，限制 32..10912；相机提交采用重试后的实际渲染候选。原版本地证据 `frame-resolution-recovery-v1/native-pack-v2/mainloop-zero-adaptive-resolution.asm.txt` 地址 1404dd1f0..1404dd6ac。None 模式保留已有 scale，避免每帧强制重置 1；**原版 VRAM/adaptive 调节器尚未恢复**。
4. **诊断导致隐式等待**：frame_observation 已改为固定容量的发布快照；读取诊断不再通过 worker.call 回收当前帧。对应三帧 WSI 验证通过。`frame_present_complete` 仍是同步接口，不能据此称所有诊断都非阻塞。
5. **mapped 宿主写入错误走异步 staging**：即时 CPU 读可能看见旧内容，重复 GPU 测试捕获此竞态。可映射宿主路径改用经过边界/范围验证的 ManagedBuffer::write_bytes；减少 staging 复制与提交。宿主 19 项已通过，重复压力测试仍需统计。一次原生 SIGSEGV 的因果尚未确认，不能仅凭重跑成功称已根治。

## 仍存在的串行或等待

| 项目 | 当前影响 | 后续修正与边界 |
|---|---|---|
| ReadLease 包住完整 render_back | 取交换链、呈现、GPU 限流期间仍挡住下一帧物理写入 | 先明确最后物理读取点与版本，再缩窄；不能直接提前 drop |
| 冷管线 prewarm / 编译队列退避 | 首帧和新组合可同步等待，最长 180 秒；还占渲染读边界 | 恢复异步 Prepare/Ready，保留真实错误/取消及幂等上传，不直接删等待 |
| AwaitRender 先返回再处理 OS 队列 | 关闭/窗口控制可延后，resize/focus/vsync/suspend 同步 call 仍 reclaim | 独立控制消息/代际协议，保证关闭与暂停唤醒和已接受帧结算 |
| 后台 gameplay.update 不走同一准入 | 当前独立 App/BufferHost，未证明存在同资源循环等待 | 明确独立资源隔离；若共享写入或任务依赖变化必须统一准入 |
| 跨 pass Collect 尚未并行 | 已支持单 pass 连续 draw 的稳定 Collect/Commit，依赖层仍串行 | 建显式 Prepare/Collect 依赖后按层并行，保持原提交顺序 |
| 普通 SSBO 影子与修改扫描 | 稀疏视图/删除日志/Arc 版本已优化，普通影子与变更 tick 扫描仍有成本 | 生产者变更日志、脏区间/瓦片、所有权消息，回退必须有计数 |
| 完整资产上传协议未完成 | CPU 门禁仍承载安全性，不等于物理多版本或 GPU fence | 单调操作序号、幂等分阶段确认、范围依赖与有界多版本 |
| 真正晚相机未完成 | 候选历史已按实际提交反馈，未包含晚输入及精确交互冻结 | 统一剔除/LOD/阴影/TAA 相机版本，交互绑定输入序号 |

## 已核实的主路径与验收限制

- 主游戏声明/编译留在常驻渲染线程，没有发现回到原游戏线程声明的主路径。
- ECS 冲突边重建、共享池、独占 Collect 输出、稳定 Commit、App 代际反馈、资源身份检查已接入；它们不证明完整上传版本协议或全部相机功能完成。
- Inline 回退也会产生多个命令 chunk；`parallel_batches` 不能直接解释为实际并行任务数或并行率。
- 旧增量构建已通过真实 GameLoop 1001 次世界呈现、关闭/保存/GPU 退出。最新改动需重新长跑；1000 帧三轮四场景及真机 p95 对照尚未完成。
- 当前环境为 Mesa lavapipe，后端全量有 7 项因缺少独立计算队列而 Unsupported；不得计为通过。默认并行 Collect 仍关闭，等待真实硬件性能验收。
- 所有源码、资产读取、构建、日志和显示测试仅使用本地磁盘；网络命令遵守清除进程代理的用户偏好。


## 实际窗口回归暴露的旧路径

`/tmp/render-window-cycle-raw-diagnostic.log` 确认全屏返回后消费 `Raw(UiAppendSignifier { tag: 5, kind: 5, ... })`，来源是 `telegraph_walls_are_merged`。旧实现遍历所有墙，把曲线端点误装为 UI 原始记录；inactive UI 保留流，后续首次悬停消费时 panic。

审计 agent 结合真实注册签名、完整函数与本地机器码确认：原版无 UiSignifierStream 参数，按 PlayLineAcrossWall 事件直接创建 dashed_line 实体，更新自身 Curve3/elapsed，0.75秒后回收。已在原模块增量修正，复用 SignifierEntityBuilder、TransientMesh、dirty mesh 上传；flag 非零反向的是 easing 输入，墙删除不提前终止已有动画，音频 kind5 用实际曲线首点。删除了错误的全墙计时资源；新增无事件无生产、事件/音频/真实网格/墙删除后完成和资源回收、方向边界测试。该修复仍待最新构建及完整窗口复验。其他形状、地形和装饰交互的 Raw/无闭包 Factory 尚未全部恢复，不能泛化为已经清除所有旧路径。

## 后续验证进展

- `/tmp/render-drained-ownership-final.log`：189 主程序单元、原版游戏链 6 项（含真实30帧和全部历史）、WSI三帧通过。描述符断言改为真实 GPU drain 后逐类 live 数精确等于当前 FrameGlobals/SamplerPages 所有者数，同时验证页容量稳态；未增加固定容差。瞬时峰值可受录制分配及完成时序影响，不再把它任意固定为第10帧峰值+16。
- `/tmp/render-host-mapped-repeat-final.log`：19项宿主GPU测试连续20轮通过，未再次出现即时映射读取竞态或原生崩溃。此前一次SIGSEGV因果仍未确认。
- 最新强制Collect长跑 `/tmp/render-parallel-resolution-1000.log` 仍在进行。当前记录显示准入竞争让Collect回退内联，**启用开关不等于覆盖实际并行**；该运行只能算对应回退路径长跑。图计划缓存每帧GraphStructure miss正在进一步审计。
- 修正诊断采样：declare_probe 以前只在打印帧排空，使第100帧行混入此前未打印帧累计数据；现改为每个启用计时的帧排空。全局原子计数仍可包含同期后台准备，不能作为严格单App归因或真机性能结论。


### 新增闭环（2026-10-23）

- 修复通用 worker shutdown 永久等待 Pending：先停止并唤醒，worker 归还已接受的 queued/Pending 包并释放读边界；已有 idle 包无需再次 poll。新增 Pending shutdown 单元已通过，idle 竞态修正随最新窗口回归。
- 自适应/OOM 统一使用 canonical AdaptiveRenderState 和 FrameRenderConfig effective scale；用户 GameConfig 目标不被覆盖。字段反馈保留期间 resize 与 flag_20，并拒绝较新倍率请求、其他 World 或资源重建。
- shadow 三级联跨 pass 并行收集，根图稳定提交。实际 30 帧强制运行记录 cross_pass_tasks=90；第一次回退遗漏 render 的断言已修复并通过真实游戏链。
- Raster 生产快照只提取消费元数据，原 CPU 像素/hash 不进入渲染 World，避免快照导致下一笔整块 COW；agent 复核真实消费者仅访问 flags/dirty/staging SSBO。6 项快照测试通过。

完整物理内容多版本仍未实现；当前共享读准入保持正确性，不能提前移除。无独立 compute queue 的 lavapipe 仍不能验收对应硬件能力用例，也不能作为真实 GPU p95 性能结论。


### 多 agent 交叉复核：上传提交与映射读写（2026-10-23）

本轮新发现并进入实现的实际缺陷，不能由先前稳态通过结果排除：

1. **上传入队 ACK 与提交失败**：Sparse flush 在队列接纳后清 dirty，旧 TransferManager 非 Timeout 提交失败会删除 payload。提交期 OOM 后若渲染降分辨率重试，会缺失先前已确认接纳的数据。已新增 opt-in `flush_uploads_preserving_oom`，保留原队列 payload/physical owner/FIFO/ticket，不加副本且该调用立即报错；明确恢复后才再次 pump。DeviceLost/InvalidUsage 仍终态。`/tmp/render-upload-mapped-regression.log` transfers 9 项通过，含真实 allocator host-budget 提交 OOM 后恢复、不同 logical key 同 physical owner FIFO 的 GPU 回读测试；host_buffer_ranges 既有 3 项通过。
2. **多 chunk 中途失败**：后续 chunk 入队/pressure flush 失败可能留下已接受 prefix；稳定 owner 的上层调用若仅记录错误继续运行，图可能看到部分更新。仍在实现 producer write 事务完整性检查，不能将上面的 OOM 恢复当作该问题已修。
3. **CPU 直接写 GPU 映射**：world_raster、history replay 与 upload_to_staging 宏直接 copy_to_staging 修改映射，旧 Update gate 仅覆盖 CPU 声明/提交，不保证 GPU 已读完。生产入口已迁移受真实 GPU range hazard 保护的映射 closure，复用现有映射、不加整块 CPU shadow。新增未 signal timeline 阻塞 GPU copy 的测试已写，tiny lib 尚待执行。
4. **CPU 直接读 GPU 映射**：鼠标深度、地形延迟复制、stroke index、mask 等实际消费者仍可能读取尚在 GPU 写入的映射。底层 `try_cpu_read` 已提供完成检查与 invalidate，Busy 不执行读取 closure；正在迁移消费者，保留旧 CPU 值/重试请求。不能用“两次 Update 延时”作为完成 fence，也不能按 MAPPED_MEMORY 数值单独判断方向（MouseTerrainRaycast 使用 memory=2，仍由 GPU 写入）。

其他已验证增量：`/tmp/render-main-cross-pass-integrated.log` Raster 12 和实际游戏链 6 通过；`/tmp/render-expanded-forced-parallel.log` 30 帧强制并行通过，540 个共享池 Collect 任务、420 个跨 pass 任务，GPU 完成后的描述符 actual=expected=[8,0,5,72]。该结果覆盖扩展主/OC2/late pass，但不覆盖之后的映射/上传修复，也不是硬件性能验收。

窗口晚相机测试的两次构建曾分别失败于测试 helper 漏 `Ok(())`、新 GPU 测试 read_buffer 长度 u64/usize 类型不符；均已修，等待新一轮统一构建。未将这两次日志记为通过。
