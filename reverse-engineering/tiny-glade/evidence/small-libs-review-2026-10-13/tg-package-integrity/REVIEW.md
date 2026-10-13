# tg-package-integrity 独立复核（2026-10-13）

结论：当前 B / Windows x86_64 核心校验策略未发现已确认的业务算法缺陷；依赖复用符合要求。不能称整库与真实游戏业务已完整验收：原完整 verify 入口、罕见 serde schema 和实际 report/crash consumer 尚未闭合。确认一项性能成本，属于现有调度/读法的成本，未证明相对原作回归。没有修改 crate、Cargo 或全局 tracking。

机器可读结论见 [findings.json](findings.json)。重要项为 INTEGRITY-GAP-01（消费者闭合）、INTEGRITY-PERF-01（线程与内存成本）、INTEGRITY-UNKNOWN-01（schema 原侧验证）。其余为继承的故障边界、有意表示差异与非当前平台限制；没有把未知或未采用 ABI 1:1 当作 bug。

## 原业务与代码审查

| 规则 | 独立复核结果 | 证据边界 |
|---|---|---|
| raw bytes → BLAKE3 → 带 32 长度前缀的 seeded FNV → low16==0 | 当前 x64 候选正确 | 原 RNE 的 FNV/array-Hash 叶实际执行，BLAKE3 复用原锁定依赖；不等于原整 verify 运行 |
| 校验先于 JSON，version 只接受 1 | 当前主流程符合原静态控制流 | 原 verify 0x14067c3d0 与候选 lib.rs:59/62/64；罕见 schema 见下 |
| 只有精确 self path 跳过逐文件 hash | 正确 | 原 0x140f77f30 的 24-byte 字符串比较；未增加路径归一化 |
| read 失败均 Missing，hash 小写完整文本精确比较 | 正确 | 原 closure 0x140f77f30；目录当文件读的受控候选输入返回 Missing，uppercase hash 返回 Corrupt |
| 重复条目独立检查，聚合顺序保留 | 正确 | 原 easy-parallel tuple Vec / 聚合；130 missing 跨 3 批仍完整且顺序正确 |
| extra 递归、原 String exact membership、只根 tmp 忽略 | 正确 | 原 0x140f78630 / 0x140f79650；sub/tmp/nested 仍 extra |
| symlink/name-surrogate reparse、非法 Unicode 名称忽略 | 已进一步验证 | 当前 NTFS junction 显示 is_symlink=true，被跳过；孤立 UTF-16 surrogate 文件创建成功但被 skip；中文/emoji 正常返回 |
| read_dir / DirEntry / file_type 错误静默跳过 | 与原静态策略一致 | 未人为更改权限或制造每种 OS 错误，不能概括全条件验收 |
| missing/corrupt/extra 三类都空才成功 | 正确 | 原聚合尾段与 lib.rs:70；受控候选成功/失败覆盖 |

原证据复用的是实际 MCP 留存反编译、原 PDB 与原发行物，而非仅相信前 owner 的文字结论。旧反编译的间接跳表 warning 仍保留，聚合解释结合原已取证跳表/closure 返回值；本轮没有把失败的新工具启动当作反编译成功。

## 独立运行验证

- [native-hash-result.json](native-hash-result.json)：从只读 tiny-glade.rne 提取 0x140f737f0 的 432 字节，SHA256 为 1416d4aaf39049cc47f19cf412894163310c13ee296f77f3c584dd63d43535fa，与已记录范围一致。Windows native ABI 执行 4096 个 digest 输入，完整 FNV 状态与原锁定 fnv + std Hash 候选一致；另 128 个原 manifest 空白变化组合的 low16 gate 一致。代码/身份分别在 native-hash-probe.rs、hash-leaf.bin、hash-leaf-identity.json。这是原 FNV 叶执行与依赖 BLAKE3 的组合对照，未执行原 BLAKE3 整体或原 verify/read_dir 入口。
- [probe-result.txt](probe-result.txt)：8 类 gated schema 输入；未知字段与 struct sequence form 被候选接受；负数/overflow/float version、重复 version、missing files、bad tuple 被候选拒绝。PDB 已有 visit_seq 与 serde u32 相关记录，但无法只由这些记录证明所有 serde attrs；罕见输入没有原侧 runtime 对照，保持 unknown。
- 同一 probe 验证 130 个 Missing 跨批保持输入顺序、目录 read 失败归 Missing、重复文件 good/uppercase 分别 Valid/Corrupt、self 特判、root tmp 与 nested tmp、合法/非法 Unicode。
- [junction-result.json](junction-result.json)：Windows Junction symlink=true、dir=false；extra 只返回 target/actual，没有 alias/actual。
- [source-test-binding.json](source-test-binding.json)：当前 lib.rs SHA256 094761f47e50eb4c479fab2ca745468e19d00392206f311a940e30eb286ced8a，source mtime 早于所链接 rlib。探针是独立 rustc harness，链接已有 workspace dev rlib；未重跑无关 workspace 测试，也未修改 Cargo.lock。fixture 与探针均只写本 review evidence 子树，未进行递归删除，发行物只读。

## 性能与依赖

[perf-results.json](perf-results.json) 是当前 dev 库、优化 harness、单次合成运行；重复访问同一文件，不能用作完整发行包或原作性能结论。

| workload | 耗时 | sampled peak working set |
|---|---:|---:|
| 1 条空文件 | 0.677 ms | 3.78 MB |
| 64 条重复空文件 | 9.893 ms | 4.62 MB |
| 1792 条重复空文件 | 109.601 ms | 5.46 MB |
| 64 条重复 2 MiB 文件 | 37.890 ms | 22.33 MB（baseline 3.40 MB）|

从直接复用的 easy-parallel 3.2.0 源码确认：run 把最后一个 closure 放在调用线程，其余每项新建线程。原 1792 项一次 run 生成 1791 个新线程；候选 28 批共 1764 个，但峰值最多 63 个新线程，另有调用线程。故 64 批是合理的资源峰值改进，却没有消除 O(N) thread churn。fs::read 仍把并行读取的文件完整驻留，内存上界随 live 文件大小增加。这里没有原作计时，也未测发布模式，不宣称性能回归。高频消费者接线后再按实际 workload 判断是否需要原锁定池依赖和 blake3::Hasher::update_reader；无需为此手写线程池或哈希。

easy-parallel 内部 NoPanic 在 thread::spawn 失败引发 unwinding 时可能 abort；正常 worker panic 在 join 后传播。候选继承此依赖语义，原作也使用它，64 批已降低并发需求。本轮没有故意耗尽用户机器资源，故没有把该条件写为已复现的新 bug。

[dependency-audit-independent.json](dependency-audit-independent.json) 独立比较 original-Cargo.lock 与候选 Cargo.lock：blake3 1.5.3、fnv 1.0.7、relative-path 1.9.0、easy-parallel 3.2.0、serde 1.0.200、serde_json 1.0.85 的 version/source/checksum 全一致。JSON/schema 由 serde；BLAKE3、FNV、hex 格式由依赖；路径组件由 relative-path；文件系统、String/HashSet 由 std；并发由 easy-parallel。递归 walk 实现的是游戏自身筛选/忽略策略，不是重写库。未发现重复造轮子。

## 骨架参考与完整性

[skeleton-reference-symbols.json](skeleton-reference-symbols.json) 只抽出本私有模块的 15 个 distinct symbol 字符串，不把依赖泛型算入游戏接口；不是原全部函数分母。

- verify 主流程和 closure$0..3 的错误转换/检查/聚合策略已在现有实现对应，不能因闭包名不同判缺失。
- find_extra_files 的 str / PathBuf 两个单态对应一个递归逻辑；当前 nested walk 覆盖，未要求单态数量 1:1。
- is_valid_manifest_checksum::calculate_fnv_hash 已用原 fnv Hasher 组合实现，属于正确依赖复用；不需要另外手写私有 FNV 函数。
- 私有 derive 的 deserialize / visit_str / visit_map / visit_seq / expecting 应恢复 schema、复用 serde；这些符号提醒罕见 schema 仍需原侧核实，不能以 3 个公共函数完成直接推断整库全闭合。
- [consumer-search.txt](consumer-search.txt) 对 shared-foundation Rust 的有限搜索只找到 library/tests/inspect_package example。原真实 report 与 crash reporter 消费者仍未接线；不把 bounded 搜索当全游戏不存在任何 caller。

有意差异保留：显式 root 代替 CWD、可读 error enum、64 批代替一次性 run。它们不因非 1:1 自动成为缺陷。稳定文件系统的 observable policy 已覆盖；并发修改时序、32-bit/big-endian std Hash prefix、精确 errno 文案和其他特殊 reparse 类别仍限定范围。

## 工具生命周期与剩余工作

本次独立 SDK worker review-integrity-20261013 在 ready 前失败：先 APPDATA 缺失，再 LaunchSupport 的 Windows shell 引号问题；仅 evidence 内的 sitecustomize 固定已验证 JDK，未修改安装包/共享脚本，但原默认 OSGi cache.lock 写入被拒绝，最终 startup timeout。没有新 MCP schema/metadata/parser查询成功，也没有新的数据库修改/原整入口执行。

[worker-status.json](worker-status.json) 留存 SDK --check 和 --stop（均 exit 0，running=false、pid=null、status=null）。由于启动失败没有 ready PID/create-time 身份，不能据此证明初始化 daemon 全部退出。CIM 查询亦被拒绝，未猜 PID 或停他人进程；残留可能性登记给集成 owner 后续核对。未把“stop 命令成功返回”夸为已确认后台 shutdown/save。

下一步按价值：真实本地 report consumer 接线 → 原 parser/verify 受控边界对照 → consumer 发布模式性能测量。核心库本轮没有需要立即改动的已确认算法 bug。
