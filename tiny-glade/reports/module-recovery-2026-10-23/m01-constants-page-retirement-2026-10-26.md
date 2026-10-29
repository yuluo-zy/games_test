# M01 配套 GPU：动态常量页在投影缓存中的退役

## 结论与范围

已在本机无窗口 Vulkan/MoltenVK 后端复现并修复 `device_address_chunks_keep_same_page_ranges_until_cancel_or_real_completion` 的真实 `Timeout`。错误来自空闲 managed projection 缓存继续强持有动态常量页，不是 GPU 内存预算不足，也不是缺少设备级等待。

本轮只修改既有 `rhapsody/src/rg/pass_bindings/projection.rs`、`rhapsody/src/rg/vulkan.rs` 以及对应现有测试文件；保留当前手工重构结构、上传环算法、预算、完整范围断言和强/弱所有权合同。本轮属于现有实现的生命周期修复，没有新声称完成原版业务算法逆向。

## 根因和完整所有权链

1. `DynamicConstants` 的一页预算耗尽后，页重置同时要求原始 Buffer 的唯一 CPU 所有权、无 Weak，以及全部真实队列时间线完成；这些检查正确，未修改。
2. `GraphPasses::add_compute` 将两块 BDA 读取导入为同一物理页的两个精确范围。图声明租约保留未提交页，取消后退役；录制后各命令独立持有实际资源/SyncCell，实际完成后由 `recycle_completed_commands → release_completed_owners` 释放。
3. `collect_recorded_imports` 退役图逻辑 token，同时删除 active managed snapshot。但投影池的空闲槽仍持有 `ManagedBuffer → Arc<Buffer>`。因此 `frame_import_count == 0` 并不能证明上传页已归还。
4. 原始页仍有投影缓存强 owner，上传环不能 `Arc::get_mut`；配置固定一页，正确返回 Timeout。增大预算、删 Weak 检查或等待 idle 都不能修复错误的 CPU owner 合同。

## 修复

- 投影状态记录本次物理注册表是否包含正式动态常量页。空闲快照仅在 `Arc::get_mut` 成功时，移除这些页的 managed row 及对应物理身份索引。
- 普通 Buffer/Texture 的物理 owner 缓存继续复用，所有注册表与索引 Vec 的容量保留。没有创建下一帧表，也没有清空整个缓存来使测试通过。
- 动态页索引与 row 一起删除，防止同一原始指针重用后命中已退役 handle。
- 所有收集边界都会重试空闲投影退役，即使没有新的 import token 被删除。外部 Arc 或 Weak 暂时固定的旧快照释放后，不依赖下一帧重建才能归还页。
- 外部 Arc/Weak 快照保持不可变。在途命令仍有独立实际 owner 和时间线，空闲 CPU 投影释放不会提前解除 GPU 使用约束。

## 验证证据

使用 `RHAPSODY_REQUIRE_VULKAN=1`，初始化失败即失败，没有 SKIP。沙箱内进程初始化受系统 GPU 访问限制，因此实际 GPU 验证使用获准的无窗口进程。后端配置：

```sh
VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json
VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d
DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/lib
RHAPSODY_REQUIRE_VULKAN=1
```

| 验证 | 结果 | 核验内容 |
| --- | --- | --- |
| 原样真实 GPU 复现 | 失败，Timeout | 原测试取消/完成后 allocate(16) 失败；保留修复前日志 |
| 原 `render_host` BDA 测试 | 1/1 通过 | 未提交必须 Timeout；取消/真实提交完成后同一页 offset0；两精确范围、原上传字节保留；真实 compute 输出 100..103；idle_waits 无增长 |
| `managed_projection` | 6/6 通过 | 普通资源两种图形状反复复用，无新增 managed allocation/容量增长；外部 Arc/Weak 不可变；预算失败发生在录制前；缓存物理 owner 可回收 |
| 新增常量页外部快照测试 | 包含在上项 | 真 GPU copy 与字节读回；强快照仍在时 Timeout；只剩 Weak 时仍 Timeout、内容不变；Weak 释放后无新导入、无下一帧、无设备 idle 即可复用同页 |
| `frame_imports` | 5/5 通过 | 时间线门实际阻塞时命令仍持有资源；取消释放声明；导出保留到完成；录制失败、部分已提交前缀失败后正确回收；真实字节读回 |
| 动态常量 CPU 合同 | 15/15 通过 | 多队列完成、强/弱 owner、已提交页封闭、预算、OOM/DeviceLost 原样传播、flush 失败保持状态等 |

证据日志在本目录 `m01-constants-{before-gpu,projection-gpu,frame-imports-gpu,cpu,render-host-gpu}.log`；源码与日志 SHA-256 见 `m01-constants-sha256.txt`。构建使用现有共享 target、`--offline`，未清依赖或新建 target。

## 下一轮复用的经验

先分别列出图声明、回调、命令、managed handle、空闲投影、外部 Arc/Weak、上传环的 owner，再用实际时间线证明何时能释放。逻辑 token 数归零和 GPU 完成都不等于所有 CPU owner 归零。对有限容量可重用资源，空闲缓存必须有与资源种类匹配的退役策略，同时保留外部观察者不可变与普通资源缓存命中率。用固定一页、同指针/offset0、真实字节读回、无 idle 增长构成验收，避免用扩大容量掩盖泄漏。

## 未覆盖边界

本轮证明当前本机 Vulkan/MoltenVK 的无窗口 GPU 链路与相关 CPU 故障合同。未进行 OS 窗口输入、present/视觉验收或整游戏长稳，也没有重跑 Windows 机器；不能据此宣称整个渲染、toolbar 或 M01 全部业务完成。
