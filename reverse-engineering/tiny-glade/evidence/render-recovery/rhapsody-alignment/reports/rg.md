# rg 域复现报告（部分完成）

图例：✅原版对齐[证据] / ⚠️候选或假设 / ❌未做或读不出。验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features`，无 FAILED。

## 已做
1. 接线（工作包 1，部分）
   - ✅ `rg/frame_throttle.rs::FrameThrottle` 已接入 `VulkanGraphSession`，替换原 `VecDeque`，补 `front/pop_front/is_empty`。语义出自 0x140beabd0（begin_frame 弹到只剩 1）、0x140bec400（end_frame 入队）、0x140bea680（trim 只等队首、不弹）。
   - ✅ 新增 `VulkanGraphSession::attach_graphics_device`：调用 `attach_submission_engine`，引擎镜像 `chain.device` 的队列状态。每次 end_frame、wait_frame 完成、`collect_after_completion` 后同步并推进回收队列。
   - ⚠️ 原 `wait_idle_and_recycle_resources` 取 `&mut self`，Arc 持有者无法调用，所以经 `with_submission_engine` 复现其第 4 步。需 raw 归属人改成 `&self`。
   - ✅ rg 内只有 2 处 `wait_idle`：backend Drop（拆除）和 `collect_after_completion`（缩放/收尾）。帧级等待早已走时间线。其余 14 处在 `raw/`，不属于我。
   - ⚠️ `begin_frame` 仍调用自身 `wait_frame` 循环，没有改用 `FrameThrottle::begin_frame` 闭包，因为会遇到 `&mut self.backend` 借用冲突。
2. temporal 状态机（工作包 3）
   - ✅ 判定：`parallel-batch-03/temporal-full` 的 `temporal-state-machine.json` 与 ALGORITHMS §5.4 一致。0=raw 纹理、1=raw 缓冲、2=已导出；复用时回到 0/1，复用的返回 tag 才是 1。SYNTHESIS 的“→1”是把返回 tag 当成了状态。现有 `TemporalRecordState` 取值正确，无需改码。
   - ✅ 新测试 `tests/rg_temporal_oracle.rs`：回放 `parallel-batch-06/temporal-lifecycle/native-oracle.json`，take 48 + restore 48 例通过。mode2（`should_reset_temporal`，1005 例）的实现在 country-core，本测试只核对期望值格式，未对实现回放。
   - ✅ 新测试 `tests/rg_population_oracle.rs`：回放 `population-validation.json`，97 例中 95 例通过。
   - ⚠️ 2 例 `synthetic_local_reuse` 被跳过，原 oracle 自己声明那是未证实的合成夹具，重建没有 local reuse map。
3. 未回放 oracle（工作包 4）
   - ✅ gpu-lifecycle、device-counters、pipeline-layout、pipeline-boundary、descriptor-binding、sequence-runtime 的回放测试已存在于 `tests/original_oracle*.rs`（PLAN.md 里写“待重放”已过时）。这些测试没有回归。

## 未做 / 遗留
- ❌ 工作包 2 全部未动：compile 三闭包、`CompiledRenderGraph` 0x218B、`GraphExecutor::new`、`last_known_frame_duration_ns`（实返 bool）、`PassPgoNameId` 指针语义、回调 `FnOnce(u64)`、`ImportedBuffer` 20B、resource key 改 u64。
- ⚠️ `PassPgoNameId`：ALGORITHMS §5.1 已证原版 `from_name` 为恒等（ID=&str 指针），现 FNV-1a 是偏差。可用 intern 池（泄漏 `Box<str>` 取稳定指针）改成指针语义，未实施。
- ❌ 工作包 5 未做：`submit_pending_cbs` 20 ms 阈值、`queue_submit` 体、54 个 pass 入口清单。
- ❌ 帧顺序（compile → begin_frame → execute → retire → acquire → 展示图 → end_frame → present → trim）没有逐函数核对 `ExecutionChain`。rg 里没有 acquire/present，它们在 `raw/swapchain.rs`。
- 需 raw 归属人：`wait_idle_and_recycle_resources` 改 `&self`；init 路径应调用 `attach_physical_device_info`。

## 改动文件
`crates/rhapsody/src/rg/{frame_throttle.rs,vulkan.rs}`；新增 `tests/rg_temporal_oracle.rs`、`tests/rg_population_oracle.rs`。
