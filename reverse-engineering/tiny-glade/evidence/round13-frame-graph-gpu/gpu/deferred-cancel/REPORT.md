# with_session 重入 Drop 修复候选

`candidate.patch` 追加于 **temporal 最终冻结 v2**，不是生成时尚未整合 v2 的生产文件。两个 base SHA 与候选 SHA 在 `manifest.json`。未改冻结 v2 文件/manifest，未改生产，未运行 Cargo/GPU。两候选文件最终 rustfmt parse/check exit 0。

## API 合法性

现 public `CoreRenderBridge::with_session<T>(&mut self, impl FnOnce(&mut VulkanGraphSession)->T)` 无禁止捕获/丢弃图 guard 的合同；Rust 允许把外部 built/compiled guard move 到这个 closure 并在 closure 返回、返回 Err 或 panic 解栈时 Drop。guard 不借用 bridge 本体，只强持原 session Rc，因此不是借用检查器拒绝的调用。跨 App 也允许 `origin.with_session(|_| other.execute_built(foreign))`。原 guard Drop 使用 `borrow_mut`，这些调用会重入 panic，原 body 正在 unwind 时可 double panic 导致进程 abort。不能只补一条文字禁令或吞掉取消。

## 修复

- 每个 bridge 新增一个独立 Rc/RefCell cancellation 队列，在 App 建立时 fallibly 预留一个元素容量。原单 built/prepared lease 决定同一 origin 最多有一个待取消图；一个 guard 只能 Option.take 一次，队列不在普通每帧分配，不新建图/资源后端。
- BuiltCoreGraph、CompiledCoreGraph 和 builder Drop 均尝试 `try_borrow_mut` 原 session；可借用就走现取消，忙则把 **完整原 native BuiltGraph 或 PreparedSessionFrame** 移入原 session 队列，保留 callbacks、retire、imports、allocation/registry 和图 identity。
- deferred 元素不强持 session 或队列自身，避免引入所有权环。bridge/guard 仍持真实 session Rc。built 的 `pending_build` 直到实际 cancel 才清除，prepared 的 `active_prepared` 仍由现 `cancel_prepared` 管理。
- public with_session 采用栈清理 guard；RefMut 位于更内层作用域，在正常、Err 和 unwind 时均先释放，再 drain cancellation。collect/wait/execute/cancel 等原 session 方法通过相同入口，覆盖它们运行 retire callbacks 时产生的 graph Drop。
- drain 在无待取消工作时快速返回；运行原取消/retire callback 时不持队列 borrow，callback 可以安全排入待取消工作。retire panic 继续由现 rhapsody `run_retire_callbacks` 捕获，日志保留，不能替换原 body unwind。
- 下次 build/warmup transfer 和 bridge Drop 也尝试 drain；无全局队列、无 CPU/GPU idle、无故意泄漏、无提前释放资源或额外提交。

## 验收准备

`core_render_bridge` 增加 9 个限定回归：built/prepared × 正常/Err/panic 六条路径；origin session 正借用时拒绝 foreign built/compiled 两条路径；body 正在 unwind 且 retire callback 自身 panic 一条路径。

回归验证：原 retirement 恰好一次；正常/Err/panic 的原结果保持；完成判定发生在任何下次 session query **之前**，证明尾段/解栈清理真正执行；实际 BufferHost frame-import 回收为零；没有任何提交；原 session 可继续构图。测试中的 side-effect pass 在错误执行时会 panic，不以“录制成功”替代取消验收。

root 应在严格 GPU 模式统一执行：

```powershell
$env:RHAPSODY_REQUIRE_VULKAN = '1'
cargo test --manifest-path D:/game/tiny-glade/Cargo.toml -p tiny-glade --test core_render_bridge --locked --offline -- --nocapture --test-threads=1
```

以上仅为准备好的命令；此 worker 没有运行它，不能预记通过。现资源回收与 pending/timeline/temporal parity 仍全部复用 rhapsody 原当前实现。
