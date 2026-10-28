# 所有权与临界区审查

只读 agent 审查，2026-10-26。未修改源码，未测量性能收益。沿用现有模块和资源所有权协议。

## 优先验证的三处

1. [VAO 表发布](C:/game/crates/tiny-glade/src/raster_host.rs:619)：赋值会在 assets 锁内析构旧 HashMap/Arc。锁内用 mem::replace 发布新表，将旧表移动到锁外析构；保留 generation/Arc 身份检查及 revision 更新顺序。回归入口：vao_scope。
2. [Collect 常量提交](C:/game/crates/tiny-glade/src/raster_host.rs:1061)：dynamic.lock().allocate(...).and_then(...) 的临时 guard 活到整句结束。先在短作用域领取拥有 Arc<Buffer>/offset/size 的 chunk，锁外 write_bytes 和 Arc 包装。回归：core_raster_draw::collected_constants_and_stable_depth_draws_match_inline_pixels。
3. [纹理 view 释放](C:/game/crates/tiny-glade/src/render_host.rs:1681)：retain 在 view_variants 锁内析构 owner。应在锁内移出目标 owner、锁外析构，保持完整 texture id 和 generation 撤销。不能 mem::take 整表后解锁筛选再写回，会丢失并发插入。新增多显式 view、其他纹理保留、外部 Arc 和在飞 GPU 生命周期回归。

## 需要 profile 决定的候选

| 位置 | 所有权方向 | 必须保持的协议 |
|---|---|---|
| [CPU 常量池](C:/game/crates/tiny-glade/src/draw_constant_pool.rs:19) | 淘汰块锁外释放；任务批次独占 lease、批量归还 | 16MiB、1024 块上限，索引容量计费，unset uniform 清零，取消归还 |
| [上传 payload](C:/game/crates/tiny-glade/src/render_host.rs:966) | 复用 write_now_owned，让有预算的 payload 在服务锁外准备 | publication 顺序、OrderedDelta、已接纳前缀不重放、拒收归还 |
| [COW 预算](C:/game/crates/tiny-glade/src/render_host.rs:1253) | RAII fork credit 锁内领取，GPU 创建和上传锁外执行 | canonical version、generation、已接收上传的 Weak owner 计费；不能直接删除 publication 锁 |
| [view cache miss](C:/game/crates/tiny-glade/src/render_host.rs:1569) | owned texture 支撑锁外原生创建，短锁校验后发布，竞争失败者锁外 Drop | 创建期间释放纹理不得让 stale id 重新发布 |
| [提交资源 scratch](C:/game/crates/rhapsody/src/resource/tracked.rs:550) | 已封存集合只读借用；游标和 owner scratch 独占租借有界池 | 排序/去重、gate 覆盖准备→提交→发布/失败；guard 不能擦除生命周期跨帧入池 |
| [surface trace](C:/game/crates/country-core/src/systems/render_loop/draw_passes/transparency_pass/surface_support.rs:129) | worker 独占 trace 和 DrawBatch，owner 按 pass/page/draw 顺序合并 | 透明顺序、World/RG owner；不 unsafe Send，不新增共享全局锁 |
| [反射查询](C:/game/crates/tiny-glade/src/raster_host.rs:481) | 每 Collect scope 只发布实际使用的只读反射快照；淘汰结果锁外 Drop | shader/variant 身份、热替换、真实容量预算；不复制整张缓存 |

## 已实现，应保留

- SSBO 查询锁内 clone lease，锁外处理图资源。
- Collect 已按 scope 持有 Arc<CollectAssets>。
- parameter chunks 已按 callback 一次批量加锁，不应重复声称尚未批量。
- 首个 DrawBatch 已移动接管容器。
- 退役对象先出队、释放队列锁，再执行销毁；剩余问题是入队分配和 OOM 同步等待。
- 提交使用集已按唯一 owner 计数，不应再建议从 draw 总量改为 unique 数量。

## 测量要求

先冻结 Windows 基线。每个候选单独提交，记录锁持有/等待、分配、Arc 操作、worker 任务和预算峰值；诊断与干净性能运行分开。不能仅因临界区变短就宣称 p95 改善。原生回归要求 Vulkan 初始化成功，不计 skipped、ignored 或 filtered=0。
