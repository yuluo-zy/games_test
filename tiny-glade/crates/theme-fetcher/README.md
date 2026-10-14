# theme-fetcher

周期获取活动主题列表与内容包，维护 HTTP 缓存、CAS 目录和活动数据发布状态。

## 功能与目录

src/lib.rs（ThemeFetcher/ReqwestTransport）；src/checked_zip.rs（校验/解包/检查）；examples/probe_events_once.rs（显式单次请求和已保存响应的离线回放）。

## 来源

- 历史恢复起点：[shared-foundation/theme-fetcher](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/theme-fetcher/)。当前源码在本目录，后续修正没有同步到历史副本。
- 原 B 包/PDB 归属：`crates/theme-fetcher/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 最初导入提交：`3e0387cd`、`c05fad96`；历史导入 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/REPORT.md)；[BusinessLogic.md](../../../reverse-engineering/tiny-glade/evidence/closures-2026-10-13/theme-fetcher/BusinessLogic.md)；[Verification.md](../../../reverse-engineering/tiny-glade/evidence/closures-2026-10-13/theme-fetcher/Verification.md)。

## 已完成

ETag/Date/Cache-Control、200/304、时钟调整、backoff、半开活动范围、最后匹配胜出、变化才发布、CAS/ZIP/BLAKE3 校验和清理已恢复；run 循环与注入 Transport 可独立验证。

2026-10-14 静态复核补齐：生产 HTTP 按 Date/本地时钟 → Cache-Control → ETag → Content-Length → bytes 顺序处理；读取失败仍保留之前的 skew/ETag 副作用。活动时间在事件请求后、blob 下载前采样，fetch deadline 仍使用请求前的单调时钟。无 body 的响应立即释放；原目录 mtime 错误传播，未来 mtime 仍继续验证；ZIP 清单先创建，再缓冲 JSON，最后 write_all。恢复 21 处原日志记录的文字、target 与四种级别，直接复用原 log 0.4.27。

原实现允许失败解包留下部分目录，并在下一次 download_blob 的 is_dir 短路中命中该目录；重新校验属于启动 trim 的职责。`run_once(now, monotonic_now)` 保留冻结时钟的 fixture 接口，生产 `run` 使用真实分阶段采样。Transport 的延迟 body/头错误/Date 采时钩子是重建边界，不宣称作者原 API。

本轮逐项证据与修复：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/fetcher/REPORT.md)；原 RNE 日志字符串与级别：[log-formats.json](../../../reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/fetcher/log-formats.json)。

## 一次真实请求验证

2026-10-14 获授权向 `http://ryeland.pouncelight.games/v1/events` 发起一次 GET，禁用重定向，没有重试或下载 blob。服务返回 HTTP 200，Content-Length 与实际响应体均为 123 字节，ETag/Date 可读取，Cache-Control 为 `max-age=3600`。响应解析出 1 个活动，同一保存响应经 `theme-server-proto` 和 `ThemeFetcher::fetch_events` 离线复验均成功，fetcher 返回 Replaced 和 3600 秒刷新间隔。

首次汇总报告因原 serde_json 不支持 u128 字段写入失败，已改为 u64；随后只复验保存的响应，没有重发请求。原响应、响应头和解析结果见 [验证报告](../../../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14/REPORT.md) 与 [result.json](../../../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14/result.json)。

在 workspace 根可离线回放该响应；此模式不会发送请求：

```powershell
cargo run --offline --locked -p theme-fetcher --example probe_events_once -- --replay ../reverse-engineering/tiny-glade/evidence/theme-live-once-2026-10-14
```

单次请求示例不会由普通测试自动执行；live 模式的目录标记阻止对同一输出目录重复请求。

## 尚差的功能与验收

B 库内已定位业务面有对应实现与静态分支证据，当前 events 接口的一次真实请求及响应解析已验证。主题包下载、真实 CAS/ZIP 链、全量原入口 oracle 和 A/C 尚未验收。tiny-glade plugin/内容 consumer 属于库外接线，不从列表请求成功推断其完成；离线 fixture 测试也不等于原机器码执行。

在 workspace 根执行 `cargo test -p theme-fetcher --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
