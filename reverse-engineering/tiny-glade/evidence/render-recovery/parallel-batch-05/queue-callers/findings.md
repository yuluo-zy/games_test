# R7 queue caller evidence

目标 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。本包复用实际 RE-MCP 已保存的原作伪码与限定反汇编，再执行未经替换的 `ensure_active_cb` 原入口；没有打开新数据库，没有执行 live Vulkan。

## 观察与恢复

- `GraphExecutionRecorder::ensure_active_cb` 为 `0x140bebc90`，349字节 PDB 范围。executor `+0x40/+0x48/+0x50` 是 command vector；非零 length 直接返回。
- 空 vector 先读取 executor `+0x178` device、device `+0xad4` pool family selector，将后者传给 `create_command_buffer`。随后 begin 的 `r8d` 在 `0x140bebcf0` 被清零，即主队列；pool family selector 与 recording queue identity 是不同输入。
- normal return 将 begin 后的88字节命令对象移入 vector，length=1。旧capacity为0时调用标准Vec reserve边界，否则复用已有存储。源码候选调用 Vec API，不重建std。
- `callsite-inventory.json`从现有450地址工作图的原机器码解码得到8个直接begin、1个所选queue_submit调用点，9处调用前最后写入 `r8d` 均是 `xor r8d,r8d`。包括两个 begin_frame 调用、ensure_active以及五个 setup 泛型实例。记录完整12条前序指令窗口及索引输入哈希。
- batch02/03 持久Ghidra的相应 `get_xrefs_to` 仅返回PDB DATA，并没有上述真实direct call。因此本包不把这些xref结果用作负证据。

Rust位于 `reconstruction/render/work-packages/queue-callers-r7/`，公开 `ensure_active_cb` 和显式 `CommandBackend` 合同。`FixtureBackend` 仅用于边界oracle，不是恢复后的Vulkan后端。

## 验证与界限

180个原入口/候选案例零差异：空/1/2/4个已有command、空vector reserve与预分配、4个pool family selector、3个全对象填充值和3个sequence值；比较完整88字节对象输出及create/begin顺序。原 `ensure_active_cb` 指令不替换，`create_command_buffer`、`CommandBuffer::begin`、Vec reserve是显式fixture边界。本包不新增恢复begin/counter实现，也不把fixture输入sequence声称为原counter初始化。

已索引的调用路径使用主队列，故这些直接调用点不能构造R6A所示跨队列同号bucket碰撞。这是限定静态范围结论；不能推导整个游戏从不用辅助队列。编译器内联/未索引游戏调用、间接入口、其他queue_submit单态化以及实际多队列同时使用仍未知。没有人为改造原bucket key或宣称修复原作bug。

资源/sequence合同来自R5B/R6A，不改变其CPU retire可早于GPU完成、GPU安全需tracked handle保护的结论。原padding、panic unwinding、OOM、并发和GPU执行均未验收。
