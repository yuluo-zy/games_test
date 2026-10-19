# render_frame 候选独立静态审查

审查对象：`TG-0ef1874c29a03d0dec/main-chain-frame-20261018/world-graph-20261019/render_frame.rs` 及 `render_frame.patch`。before SHA `2d7a32e36a86f79f7936f0651c1e42712d7385819c3464a36010b7fecacd77f7`；作者首份 candidate SHA `d89f8239845daa9a6236edced2f59356fe5be35d316a74a01878adde8c8c54bf`。尚未接受生产整合或运行结果。

已独立核当前生产 `render_frame.rs`：主编译 `Err(_)` 不分原因均进入降尺度，丢失 declaration/unsupported/timeout/device-lost 原因；fallback 与 presentation 使用 panic。生产 callbacks 另存 `ProductionRenderer.first_error: Option<anyhow::Error>`，原外层继续执行会把失败后效和主帧结束混在一起。

候选把 loading/primary/fallback/presentation 错误沿 Result 返回；只有显式 OOM eligible 继续原单次降尺度/32×32 fallback，其非展示规则保留。每个可失败 void callback 后检查错误；pacer 用 copy，只有 render 成功才 commit；Err 关闭 scope 并 abort。这些改变符合当前宿主错误透明要求。ErrorKind 筛选明确是本工程适配政策，不把它写成 B RNE 已有分类。

跨文件必须解决后才可接受：

1. `take_render_error` 会在 `abort_frame` 前取走 first_error，abort 必须无条件取消当前 presentation/acquired/unfinished frame，不能复用仅在 first_error 存在时执行的旧 end_frame 分支。
2. 当前 `CompileError = rhapsody::Error` 与 `first_error = anyhow::Error` 不一致。生产实现必须保留原错误链，不能转成字符串后伪造 rhapsody kind；OOM 分类检查实际源错误。
3. 全调用者必须接收 Result；GameFrameDriver/GameLoop 不得在 Err 后推进 temporal、首帧日志或成功观察计数。实际已提交/已排队展示事实仍需保留，不能因为后续失败反向改成未发生。

已把前两项交给 02 主图 owner，与 01 对齐第三项。此审查尚未包含 02 的 abort/error 代码，也未执行作者新增测试。等待 root 同步源并整合必要消费者后，由 root 运行限定 core 外层测试及真实 fullgame WSI。完整原函数/PDB/RNE绑定由作者补齐后单独核验，当前不声称原版行为等价。
