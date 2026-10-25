# motomoto 当前功能边界

2026-10-17 源码重构后，正常文件格式保持兼容，Reader 使用零拷贝切片解析，支持末尾空段。所有残缺输入和超大长度安全返回一次 `UnexpectedEof`，随后终止。

Writer 支持部分写入、非零起始偏移、显式 chunk 完成与 Drop 回填。位置运算受检；首次 I/O 失败后永久停止访问底层流，调用方必须丢弃该输出。`flush` 不完成 chunk，`Writer::finish` 不刷新底层缓冲流。

当前十个测试覆盖字节布局、tag、截断边界与极端长度、实际 File / BufWriter、I/O 故障矩阵、位置运算及 Interrupted / WriteZero。详见 [README.md](README.md)。

历史 B 原码验证仅适用于重构前快照；本次按源码合同修正错误行为，未重新逆向。完整游戏存档消费者、schema、跨版本读写与原游戏进程仍未验收。
