# B 主题三库静态逆向与补全

2026-10-14，实际交付位于 `D:/game/tiny-glade/crates/theme-defs`、`theme-fetcher`、`theme-server-proto`。已完成 B 客户端已定位库内业务及类型合同的静态复核和源码补全。没有请求真实主题服务，没有启动原游戏，也没有分析 tiny-garden。作者原始文本、完整原入口行为等价和库外游戏集成没有因此获得验收。

## 实现结果

| 库 | 本轮结果 |
|---|---|
| theme-defs | 完整盘点已发射的活动/错误传输合同；核对 PathBuf、两个 SystemTime 以及两个单位错误和 owned String 错误；补完整结果身份测试，保留 Windows 可表示的 100ns 亚秒差异 |
| theme-server-proto | 核对 start/end:i64、cid:String、events；复用原 serde，补对象/结构序列、必填、重复、递归忽略未知字段、整数边界、非法类型和序列长度的合同测试 |
| theme-fetcher | 修正 Date/ETag/Content-Length/body 的处理及失败副作用顺序、真实响应释放、分阶段时钟采样、CAS mtime 错误传播、ZIP 父目录和清单写入顺序；恢复有原字节和级别证据的 21 处日志记录；保留原依赖和原缓存失败语义 |

fetcher 保留原作规则：条件 ETag、200/304、默认 36000 秒刷新、失败退避 1/2/4…300 秒且成功不重置、所有 Err blob 每秒重试、事件尚未激活仍预取、已有目录跳过下载、内容 BLAKE3 cid、最后命中的半开活动区间生效、结果变化才发布、消费者断开时退出。CAS 严格超过 14 天才清理，未来 mtime 继续验证；ZIP 清单记录完整目标路径，按归档顺序将路径与文件 hash 拼入 aggregate BLAKE3。解包失败留下的部分目录仍会被下一次目录命中跳过下载，启动 trim 才负责校验清理；没有添加原作没有的回滚或每次 cache-hit 校验。

`Transport`、`run_once` 和部分公开 helper 是恢复测试边界，不声称作者原 Rust API/ABI。源码和 README 的当前 SHA 见 [integration/summary.json](integration/summary.json)。历史 shared-foundation 恢复副本没有同步修改；旧提交与旧验收报告不能覆盖本轮输入。

## 原作证据

B patched EXE SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`；发行 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

- [schema/REPORT.md](schema/REPORT.md)：fresh LLVM IPI/TPI、proto module 2162、MCP public inventory 和原字段内存。defs 的 16 个 public 匹配均为 std/ArcSwap 胶水；proto 的 7 个匹配为 2 个 serde visitor 和 5 个 Drop 胶水，均不计手写业务算法。
- [fetcher/REPORT.md](fetcher/REPORT.md)：复用身份绑定的 new `0x14054aa00`、run `0x140618310`、ZIP extract `0x140d921c0`、verify `0x140d939a0` 反编译/机器码；fetch/download/trim/validate 是 run 内联业务。新查询核对标准库时钟/mtime 的真实入口，标准库本身直接复用。
- [fetcher/log-formats.json](fetcher/log-formats.json)：从原 RNE 读取日志 pieces/count、文字及原级别词，不从文案猜测级别。
- [integration/consumer-prepare.json](integration/consumer-prepare.json)：按 fresh PDB module 170 的原码大小及 SHA 准备 build `0x1403d0f70` / update_daily_theme `0x1403d1750`，取得限定反编译；`void()` 占位未升级为真实 ABI。独立 Windows worker 已保存并重开核查，未同步原库或历史 Mac overlay。

消费方在活动目录读取 `langs.json` 和 `meta.json`，检查 `history.json` 并处理可选 `credits.json`；meta 交给 `country_core::resources::session::SessionMeta::try_load_from_path`。路径常量见 [consumer-strings.json](integration/consumer-strings.json)，内联 try_load_theme_info 归属见 [pdb-plugin-170.txt](integration/pdb-plugin-170.txt)。这些内容格式归 tiny_glade/country_core，不能加进传输 crate 充当“完整主题 schema”。本轮只核查其边界，未实现或声称完成整个 plugin。

旧 2026-10-13 contract 中“下载错误等新列表”已过期；旧 Compare 的 mtime 错误吞掉、清单 to_writer 等价表述也由本轮纠正。旧源 SHA 的差分报告不覆盖改变后的代码。

## 验证

原 `build-info/Cargo.lock` 是依赖权威。交付锁中三个库的可达闭包含 138 个公共包，version/source/checksum 全匹配，0 差异；见 [dependency-audit.json](integration/dependency-audit.json)。新增的 serde_json 测试边和 log 依赖均使用原锁已有版本，未获取新依赖。

最终实际 `tiny-glade/Cargo.toml` 门禁：

- `cargo test --offline --locked -p theme-defs -p theme-fetcher -p theme-server-proto`：26/26 通过、无忽略，含 3 个跨库离线整链用例；[完整日志](integration/test-delivery.log)。
- `cargo clippy --offline --locked --all-targets ... -- -D warnings`：通过；[完整日志](integration/clippy-delivery.log)。
- 三库 `cargo fmt --check` 通过。

最初两次测试记录保留：CAS/identity 的 1ns 差异在 Windows FILETIME 中不可表示，测试改为 100ns 后收口，不是改写 std 时间实现。最后门禁记录了锁文件前后 SHA，确认交付锁在测试/Clippy 期间不变。全 workspace metadata 的一次离线探查因无缓存的 cfg-if 0.1.10 失败，未因此联网；这不影响已完成的三库限定门禁。

门禁后共享 workspace 的其他工作更新了整个 Cargo.lock；本三库源码/manifest SHA 以及可达 138 公共依赖身份逐项保持相同。测试输入锁与后续审计锁分别保存，不把后续整个锁 SHA 冒充测试输入；见 [post-gate-lock-comparison.json](integration/post-gate-lock-comparison.json)。

## 剩余边界

B 库内已定位业务面没有已知未实现算法。defs 的作者精确字段/变体名与两个单位错误 Display 文案未由已发射证据提供，现有名称/文案仍为明确标注的重建接口。客户端没有提供完整服务端 API、未发射源码或 A/C 的验收证据。本轮没有运行完整原 HTTP/CAS/ZIP oracle，也没有验收真实游戏主题插件、内容消费、真实服务及视觉效果；26 项候选测试不能替代这些验收。
