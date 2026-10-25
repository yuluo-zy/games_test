# reset_all_temporals 退休所有权独立审查

生产源首审链：`rhapsody/src/rg/vulkan.rs` 的 `reset_all_temporals`、`collect_named_retired`、`TemporalGpuResources::release_temporal`、`wait_frame/finish_frame`；`rg/temporal.rs` 的 `clear/release_retired`。尚未审收到的补丁，也未运行候选。

已证源码缺口：`state.clear()` 将旧 token 放入 `state.retired_tokens`；`collect_named_retired` 仅遍历 `resources.retired`。没有随后成功 lookup 时，旧 named owner 未进入后端退休流程，表面 reset 后仍持分配。

预期窄修是在 clear 后调用 `state.release_retired(&mut resources)`，再执行既有 collect。`release_temporal` 移除 named 的 Arc 和 defined 标志，推入 resources.retired；backend.resources 中另有已导入的真实强引用。collect 逐一检查在飞 pending 的 compiled.snapshot.physical_mapping，只在无 pending 使用该 token 时移除 backend 资源/family/history。不能直接清 backend 或 pending，不需要伪造 completion。

必须保留 active_prepared 拒绝；此时不得修改任一表或丢失当前编译图的引用。重复 reset 应幂等。正常 reset 只做旧 token 数量对应的退休转移，不加入每帧设备等待；实际释放仍服从既有 completion/Arc 所有权。

重要验收边界：当前 `wait_submits_done → wait_frame → finish_frame` 不直接调用 collect_named_retired。reset时 pending 仍含旧 token，则 backend owner 必须保留；等真实 GPU 完成后，下一 begin_frame 的 collect 才将其从 backend 移除。OOM fallback 的新 build 正会经过 begin_frame。测试不能为了迎合断言擅自清 pending，或把 wait 返回误作已经执行所有退休维护。

要求 root 运行的最小有界 GPU 回归：创建/执行真实 temporal 资源并保留 pending；reset后核 named ownership消失但pending物理映射/后端owner仍在；真实wait完成，下一begin_frame后旧token不再backend持有，新temporal必须是新token且history无效；另核重复reset与active_prepared拒绝无副作用。未改源码的375项基线不算此候选通过。

## 冻结补丁与专用测试

已独立读补丁 `91eda30137d40feabfbd719819af411cfb667fba8a1ab7dca3c4884815a8b3aa`：仅增加 clear 后 release_retired 与解释，保留既有 active_prepared gate/pending guard，静态 `passed_scoped`。root 已记录生产整合，旧兼容 GPU 回归与本次新测试分别保存。

新测试源 `bd6c696cb3c7a50135387a261ba412b9b131e7c4cbce83a5a70896f6f3bf2a2c` 用真实外部 timeline gate，poll 和零超时 wait 证明 GPU 仍未完成；reset 后核 named 移交与 backend/Weak 继续保活，开闸、真实 wait、下一 begin 后回收，四轮无旧 owner 累积。另一例核 prepared 拒绝、cancel 后闲置释放、重复 reset。局部 Unblock guard 先于外层 Session 析构，避免测试断言先失败时遗留 GPU 等待。test-support 已由 crate dev 自依赖启用。

已读 root `logs/temporal-reset-v4-gpu.log`：两用例通过，零失败/忽略。它证明资源 owner 退休闭包，不是 allocator 字节预算/显存峰值测量，也不升级完整游戏。root `validation/temporal-reset-v4.json` 绑定真实 binary 与同测试/生产源指纹；严格运行环境、完整退出码由统一 runner 保留。
