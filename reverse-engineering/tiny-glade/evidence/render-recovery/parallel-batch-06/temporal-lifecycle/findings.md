# R8 temporal compile ownership

目标二进制 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。新Rust候选为 `reconstruction/render/work-packages/temporal-lifecycle-r8/`，旧temporal包保持不变。

## 新闭合的原码控制流

1. 所选compile原入口 `0x140166510` 首先将persistent graph移出（+0写None sentinel）。graph缺失时在temporal操作之前失败；graph有效但temporal缺失时，graph已经移出，不作事务回滚。
2. temporal Option位于persistent state +0x90，尺寸0x60。取出时只清第一qword，其余payload留在原位置，但所有权已经移到编译栈。48个原入口prefix案例比较完整graph与temporal快照，直到 `0x140166617`，不执行游戏构图闭包。
3. export返回后，`0x140166d2a..0x140166e19` 将新temporal状态写入persistent +0x90，再处理旧状态的HashMap/drop边界。因此后续population失败不能推断为恢复旧temporal历史。48个注入合法post-export frame案例验证字段交换、None/Some分支与旧records销毁调用；旧parity map为空，标准容器实现外供。
4. `country_core::FrameConstants::should_reset_temporal` 的原leaf `0x140976880..0x14097688b` 只判断+0x80的u32等于0。1005原入口案例通过。`0x140161cd0`游戏构图调用者把它用于分支选择，这个leaf本身没有清temporal map或parity。该辅助游戏消费者不计为rhapsody整个模块完成。

`PersistentState<G,T>`和`DetachedState<G,T>`使用Rust Option及标准drop；原失败在panic前的所有权状态显式表示为TakeFailure，不声称恢复作者异常/unwind ABI。快照适配只用于oracle，生产API为泛型类型而非opaque字节结构。

## 新鲜取证与限制

通过新只读SDK runner实际连接已配置RE-MCP，4个目标反编译中3个成功；frame reset leaf未在数据库定义，错误完整保留，随后用原PE/PDB地址的限定机器码恢复。8个操作结束后close(save:false)成功、共享锁释放。没有修改数据库或全局配置。

`direct-callers.json`针对四个指定目标的E8/E9编码，使用PE RUNTIME_FUNCTION内解码验证9个直接调用点。只解码9个函数；未覆盖间接/内联/无runtime span，不作为全程序负证据。整状态drop及ResourceValue的标准泛型胶水仅用于说明游戏字段所有权，未重实现HashMap、std drop或分配器。

仍未闭合：独立整cache reset API、unused-key eviction、map桶迭代/资源释放的完整上层触发、compile构图闭包失败unwind和GPU跨帧实际安全。新源代码只推进已观察的take/restore合同，不把history reset flag当作cache.clear，也不发明淘汰规则。
