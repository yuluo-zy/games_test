# tg-package-integrity 后续复核，2026-10-14

当前工程 `tiny-glade/crates/tg-package-integrity` 的库内已恢复业务没有发现新算法差异。本轮把此前 Windows 特殊路径范围进一步闭合，新增 11 个实际原 verify 案例驱动的回归合同，并修复诊断 example 提前读取/解析导致 panic、绕过原阶段顺序的问题。没有把新增测试/工具脚本计作新恢复算法，也没有增加 CWD 封装或依赖。

## 原函数与证据

归属为原 PDB `crates/tg-package-integrity/src`；已读取 batch-01、独立 review、development 和 batch-02 最新证据。review 中罕见 schema 的旧 unknown 已由后续 development 的 29 原 verify loader 案例覆盖，不能再当当前库内漏实现。本次继续检查的是 Windows 文件系统与精确字符串策略。

| 原函数 | 原范围（PE exception table） | 本轮证据 |
|---|---|---|
| `verify_package_manifest` | `0x14067c3d0`，2447 字节 | [native-boundaries.json](native-boundaries.json)，11 个原入口案例；SHA `5646cdb57855ea9bc2b4e28dd93a21b5465c1f42016a6d6d6d9f85bd602e1a39` |
| 逐文件检查闭包 | `0x140f77f30`，787 字节 | 原字节与 loader 相等；精确 self、OS 路径读取与错误分类对照；SHA `93ba26844db63f275c55a42c080ae869bdc06010e53e0ad21004363d7fa6fb2c` |
| `find_extra_files<&str>` | `0x140f79650`，2029 字节 | 原字节与 loader 相等；root tmp、大小写、alias、Unicode 扫描对照；SHA `e3e22148916e5019558b334e3454e12dc052bb52351067d4f435c6fb6ab23f74` |

原 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；复用既有 loader DLL SHA `281fc876c5543e3553856fdfc0fb925bbf9be72e2dce375b39ec43aec1771772`。读取原发行物与旧容器，未修改两者。容器 header/import/TLS patch 全量记录于 native-boundaries，未运行 EXE main/GUI；不能冒称发行进程等价。本轮没有新 MCP 数据库修改；C:/Users/liyu/.codex/mcp-tools 不存在，已确认可用 Windows SDK 入口是 `scripts/reverse_tools/reverse_operator.py`，本轮问题可由已保留静态证据加原函数实际执行解决，不为交付重复启动工具。

## 11 个新原入口边界

- root `tmp` 普通文件与目录一样被 extra 忽略；清单列出的 `tmp` 仍校验内容，错误 hash 进入 corrupt。
- `Tmp/file` 仍为 extra；只跳过精确字符串 `tmp`。
- Windows 文件读取接受 `CASE`→磁盘 `Case`、`sub\\file`→磁盘 `sub/file`、`./good`→磁盘 `good`；extra 的 exact String membership 仍分别报告磁盘名称。
- `./build-info/manifest.json` 不属于精确 self 特判，实际 hash 不符进入 corrupt。
- 空名称、NUL 名称与目录文件条目均归 missing；known directory 不抑制其未列出的子文件 extra。
- 中文/emoji 名称正常核对，磁盘孤立 UTF-16 surrogate 可创建但扫描转换失败而跳过。

全部原入口与当前候选结果相同。missing/corrupt 按原输入顺序比较，extra 仅对比较值排序，不向生产实现新增排序保证。合成 bytes、磁盘构造与原 expected 冻结于当前 crate `fixtures/windows-boundaries.json`，新增一个 Windows 回归测试覆盖全部 11 条。

## 代码修复与检查

`examples/inspect_package.rs` 先 `verify_package_manifest` 得到结果，再可选收集 metadata；缺清单、bad checksum、通过 gate 的 bad JSON、unsupported version、成功均输出 JSON，不再由此前 read/parse `expect` 提前 panic。文件系统并发变化时，辅助 metadata 的第二次读取仍可能不同；它只用于诊断，不是原验证 carrier。见 [example-results.json](example-results.json)：5/5。

当前源码的 Windows 同源独立 Cargo 工程使用绝对 lib/example 路径，没有复制/分叉实现；8 Rust tests 全过，0 失败、0 忽略，严格 `clippy --all-targets -- -D warnings` 通过，实际 fixture-driver/example 构建通过。父工程全 workspace 验证另由集成 owner 负责。检查记录 [summary.json](summary.json)、[clippy.log](clippy.log)、[build.log](build.log)。18 个公共 package 的 version/source/checksum 全匹配原 build-info/Cargo.lock，见 [dependency-identity.json](dependency-identity.json)，未改共享 Cargo.lock。

## 保留边界

库内核心生产策略不改，保留 64 批与 BLAKE3 reader 资源优化；本轮稳定磁盘路径对照不证明并发修改、线程启动失败/panic或全部 OS/reparse 类别等价。完整 shipped-process 启动/报告/崩溃真实 consumer、GUI、A/C、32 位/大端 Hash 前缀仍未验收。没有发现可据证修复的新库内算法，不声明全部原作等价。
