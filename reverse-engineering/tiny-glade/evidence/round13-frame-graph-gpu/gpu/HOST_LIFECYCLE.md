# ProductionRenderer 宿主生命周期复核

范围：当前 Windows 宿主及当前 rhapsody 所有权模型。仅静态代码与既有原证复核；worker 不运行 Cargo 或 GPU，不重建旧架构。

## 确认修复

`win32-surface-owner.patch`（root 已整合）纠正先前 Windows adapter 对 `Surface::from_raw` 的调用：现 rhapsody 的 `from_raw` 设置 `win32=false`，`from_win32`/`from_win32_raw` 才设置 true。fullscreen extension 判断读取该字段。此修复直接复用既有 Win32 constructor。

`surface-native-lifetime.patch`（候选，尚不预记整合/验收）修复失败析构的实际窗口生存期：

- 正常 `ProductionRenderer` 字段销毁次序为 bridge/egui/swapchain/device/.../platform/window；正常 shutdown 回收并清理工作后，Surface 先于 Window 析构，次序正确。
- 但是现 `Swapchain::drop` 在非 device-lost 且无法证明 WSI 完成时，按安全协议有意 `mem::forget` Surface/Device/信号量/旧代。原宿主仅靠字段顺序持 Window，renderer 随后仍销毁 HWND，无法满足仍被保留的 Surface 的 native handle 生存期合同。
- 候选给 Surface 增加一个有界 native owner 槽 `Option<Arc<dyn Any + Send + Sync>>`，位于字段末尾。旧 constructor 签名/行为保留；新增 `from_win32_retained`，与旧 Win32 constructor 复用同一个原生创建 helper。
- Windows PlatformSurface 将真实 `Arc<winit::Window>` 交给这个 Surface owner。正常 `Surface::drop` 先调用 `vkDestroySurfaceKHR`，再释放 native owner；失败时 rhapsody 保留的同一个 Surface clone 同时保留 HWND。没有全局窗口泄漏策略，没有更改 Device/Swapchain teardown 安全决策，也没有为每个 swapchain generation 增加 owner 列表。
- `Any + Send + Sync` 保持 Surface 原 Send/Sync 使用约束；Windows host 的 `Arc<Window>` 到该类型的转换由 root 的 Windows 编译验证。worker 两候选文件 rustfmt check exit 0。

必要受影响验证：Windows `tiny-glade` check（包括真实 Window owner admission），以及现严格 `original_loading_present` 实际 WSI 命令。没有通过注入 GPU 超时证明异常路径；此项生存期结论来自当前 `Swapchain::drop` 的完整可达控制流和同一 Arc 的传递，不能声称已执行失败 teardown。

## acquisition / presentation 退出表

| 路径 | 当前处理与证据 | 本次裁决 |
|---|---|---|
| 未 acquire / minimized | `Swapchain::validate...` 可返回 None；宿主 loading render 不能接受 None | frame owner 另获原 `main_loop 1404dc94b..dc971` 正证：宽或高为零时 1ms wait 并跳过整个帧的 egui/update/render，不采用本 worker 早先仅跳过 render 的建议 |
| 正常 resize | resize 只标 stale；下一次 acquire 在无活跃 acquisition、retired generation 预算允许时创建新代，旧代由实际 WSI 完成证明回收 | 未发现需要宿主销毁/重建 Surface 的理由；现 native owner 随同一 Surface 进入新代 |
| SUBOPTIMAL | 只在 surface extent/transform 与创建时不同才标 stale | 不重复每帧重建 |
| present OUT_OF_DATE / exclusive lost | `present_result` 返回成功并标 stale；present 的二进制等待被视为已入队，下一次 acquire 重建 | 不应在宿主额外重试 present；本 worker 未新增此类重试 |
| 声明/预热/编译失败 | `execute_presentation_graph` 的错误包装取消当前 session presentation；graph cancel/reap 退还未执行 acquisition | linear lease/owner 的 Option.take 防止二次退还；实际失败执行仍待验收 |
| 部分 submit 后失败 | `GraphPresentation` 记录 acquire_point/rendered_point/gpu_use；已消费 acquire 不会再 wait；无完整内容按现策略 release 或真实 clear+present；已完整 rendered 的放弃路径最多 present 一次 | 保留当前 rhapsody 协议，不添加另一份 rescue 或 CPU idle 替代 |
| present 已消费后失败 | session `present` 先 take 当前 presentation；raw `present_submitted` 区分 wait 已入队/被拒绝，后者平衡 rendered semaphore 并退休旧代 | 宿主只记录错误，不再直接 present 同 image |
| device lost | native WSI wrapper 将 loss 写入 sticky timelines；Acquired/GraphPresentation drop 检查 loss，不作恢复 submit；Swapchain drop 可在 device-lost 特例销毁 | 保留现失效状态，不以新 Device 或假成功继续 |
| SurfaceLost | Surface latch lost，后续 acquire 不再调用该 surface；宿主现返回错误并进入 shutdown | 没有冒充 surface 已恢复。完整宿主 surface/device recovery 策略未在本候选中实现 |
| shutdown 超时/无最终呈现证明 | 当前 rhapsody 保留真实 owners 以避免提前销毁；root 可见实际失败，不算清理成功 | 候选补齐其 native Window owner，保留原失败状态和可观测诊断 |

## 关键现源码

- `rhapsody/src/raw/swapchain.rs`：Surface constructor/drop、present_result、generation replacement、AcquiredImageLease/PresentTicket linear state、Swapchain::drop。
- `rhapsody/src/raw/swapchain/graph_presentation.rs`：acquire_consumed、所有成功提交的 gpu_use、abandon/drop。
- `rhapsody/src/rg/vulkan_presentation.rs`：import、present、cancel、begin/finish_presentation_execution。
- `tiny-glade/src/production_renderer.rs`：acquire、presentation builder/cancel、present、collect、shutdown 与最后的 Windows adapter。

旧 `gpu/REPORT.md` 对 native lifetime 的“字段次序正确”只适用于正常 teardown；本文件明确补充非 device-lost WSI 安全保留分支。
