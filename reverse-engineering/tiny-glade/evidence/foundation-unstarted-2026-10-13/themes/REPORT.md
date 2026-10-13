B 主题库恢复交接

实际实现位于共享workspace原目录 theme-fetcher/src/lib.rs、checked_zip.rs 与 theme-server-proto/src/lib.rs。proto仅协议字段，serde derive不计业务算法。

已恢复构造与sender持有、条件获取与Date时钟修正、max-age刷新与失败退避、每项下载/校验/ZIP提取、CAS年龄/腐败清理、最后匹配的半开活动时间窗、仅变化发布与消费者断开退出。

checked_zip保留原作特殊行为：清单记录完整目标路径，按ZIP顺序hash(path UTF8 + 文件BLAKE3十六进制)，crc实际也是BLAKE3；先校验清单crc，再打开其中的路径，额外文件不扫描；前置文件不会在提取失败时回滚。

原证据及Observed/Inferred/Unknown详见contract.json。独立Windows Ghidra worker checkpoint后已正常停止，只改独立数据库，没有宣称同步原库。公共reqwest/zip/serde/blake3/cache_control/httpdate/memmap2/arc-swap均直接复用B版本依赖。

最终共享workspace使用原B锁定依赖门禁：11/11业务测试通过，Clippy --all-targets -D warnings通过。完整日志cargo-test-final.log、cargo-clippy-final.log；原依赖身份由集成owner核对。临时lib.path诊断workspace已全部删除，不作交付或替代共享门禁。未发送真实游戏服务HTTP请求。

未验收：原游戏插件/ECS真实接线、完整原系统oracle、全部reqwest特性来源及日志callback。Rust公开签名是恢复接口，不是ABI保证。
