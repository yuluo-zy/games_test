# tg-package-integrity 功能与当前验收

恢复原作 B / Windows x86_64 的包校验业务，使用原锁定依赖；候选公开接口采用显式 package root。

| 业务功能 | 实现与合同 |
|---|---|
| 原始 manifest 校验门 | 文件 bytes 先经 BLAKE3、带数组长度前缀的 seeded FNV；只接受 low16=0。空白也在校验覆盖范围。 |
| 清单 schema / version | version:u32，files:(String,String) 列表；未知字段允许、字段重复/缺失拒绝，支持 struct sequence；只有version=1进入文件校验。 |
| 文件检查与聚合 | self 精确路径跳过；打开/读取失败归 missing，小写完整 hash 精确不等归 corrupt；重复 entries独立检查，结果保留清单顺序。 |
| extra 遍历 | RelativePath递归拼接与原字符串集合比较；只根tmp跳过；symlink、非法Unicode、目录枚举/entry/type错误跳过；extra保持文件系统枚举顺序。 |

原骨架里的私有closure、两个find_extra泛型实例、calculate_fnv_hash和serde visitor归入上面业务/类型合同，依赖泛型和derive不重复恢复。

当前接口维持 verify_package_manifest(root)、verify_manifest_bytes(root,bytes)、is_valid_manifest_checksum(bytes)、find_extra_files(root,known) 与原候选类型。没有引入GUI、游戏report/crash consumer、删除文件或发送消息。

本轮新增26个绑定原码的schema回归输入和跨批次三类错误测试，6项测试通过。29个磁盘case执行了原完整verify机器码的隔离DLL容器，全阶段及错误文字、missing/corrupt顺序、extra集合均与当前候选匹配。容器禁用了EXE入口/TLS callbacks，剔除4个非目标imports；由Windows loader处理系统imports/relocs/static TLS。它不是原发行进程入口验收。

真实2.70GB目录的旧release实现峰值working set约1.39GiB。本轮改用原blake3 1.5.3 Hasher::update_reader，仍原easy-parallel批64，未手写hash/缓冲器/线程池。Stable filesystem结果对齐；逐文件读取粒度、并发修改时序与原作不同，不承诺timing等价。

真实包after已在150秒期限内完整返回，首次103.55秒/峰值9.40MiB，唯一一次warm复核85.71秒/峰值9.66MiB，均missing0/corrupt2/extra288，与旧结果一致。旧耗时26.56秒/峰值约1.39GiB；cache与并发负载不同，不宣称速度提升或确定速度回退。内存成本与最终功能结果已闭合。反向缓存单文件对照未证明BufReader有改善，因此保留原reader接口，不额外加缓冲。

本轮[验收报告](../../../../evidence/small-libs-development-2026-10-13/tg-package-integrity/REPORT.md)区分静态恢复、原码隔离运行、候选验证与性能剩余。

发布测试仅依赖合成manifest-contract.json，不compile-time嵌入原B-manifest。最后测试改动由根集成验证；原B清单的本地验证由inspect_package/实际包运行完成。
