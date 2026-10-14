# 依赖与差异审计

原 motomoto 0.1.0 仅依赖 byteorder 1.5.0。当前 Cargo.toml 精确 pin =1.5.0，与原锁一致。

| 功能 | 当前使用 | 裁决 |
|---|---|---|
| LE数值读写 | byteorder 1.5.0 LittleEndian/ReadBytesExt/WriteBytesExt | dependency_reuse；不手写字节序库 |
| Cursor读/seek/write、partial write_all、IO errors | Rust std::io | dependency_reuse；不还原std实现 |
| 存储及slice | std Vec/slice/Option/Result | dependency_reuse |
| tag不超过8-byte、trunc、header布局、长度回填、>=EOF门限 | 游戏 motomoto 协议 | private_business_reconstruction |
| JSON/Deflate消费者 | 原serde_json/flate2；本crate不编解码 | consumer_scope_pending；不得手工造JSON/压缩库 |
| LLVM/PE/ctypes原指令摘取和执行 | evidence目录Python/LLVM工具 | analysis_tooling_only，不计游戏恢复实现 |

差异修正：Drop槽始终替换为Result.err()；地址/长度恢复原release wrapping；EOF equality保留原行为。AsTag和Chunk Rust命名/可见性是可读表示选择，不claim作者源码1:1。finish为额外接口；flush实际原实现unknown。Reader正常/尾部/有限失败173原code返回分类、payload、cursor position零差异；原panic仅静态验证，泛型IO失败仅候选验证；Writer运行对照仍未完成。
