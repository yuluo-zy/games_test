# crates/theme-fetcher/src

2026-10-14 当前交付以实际 `tiny-glade/crates/theme-fetcher` 为准：B已定位库内业务/合同静态补全，三库26离线测试、严格Clippy/fmt通过。历史镜像与其旧源SHA不覆盖新代码；完整原入口与库外游戏接线仍未验收。详见 [最新报告](../../evidence/theme-static-2026-10-14/REPORT.md) 和 [机器台账](../theme-static-recovery.json)。

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B主题HTTP/ETag/date/cache-control、backoff/half-open活动、CAS/ZIP校验与库内producer。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

2026-10-14 库内 fresh 反编译复核（B）：new@0x14054aa00/run@0x140618310（内联 fetch/download/validate/trim）/extract_with_checksums@0x140d921c0/verify_folder_checksums@0x140d939a0 逐项对照，关键常量与 1s 睡眠机器码核验；修复下载循环触发（原对任意 Err 每 1s 重试，非仅 Pending）。隔离验证 workspace 11 测试/clippy -D warnings 通过。库外主题 plugin 消费与 A/C 仍留。

## 已恢复的函数和类型范围

- 原endpoint、ETag更新顺序、Date偏移、Cache-Control与304保留Result、backoff不因成功重置
- 半开时间窗、负时间clamp、重叠最后项获胜、只发布变化结果
- 目录才命中；200/长度/内容cid hash后提取，失败保留到新列表
- checksum验证；过期严格>14天，未来时间继续校验；错误条目独立
- 原enclosed_name/顺序/失败保留文件；完整目标path与hash构成CRC；只验证列出的文件
- rogue文件移除，老/坏checksum目录删除，条目错误独立
- 库内run下载所有Pending含未来事件；sender只发布变化，断开退出

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| theme_fetcher::ThemeFetcher::new | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`new` | B：原定位 0x14054aa00；candidate implementation; config confirmed by PDBinline and native fields；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::ThemeFetcher::fetch_events | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`fetch_events` | B：原定位 inlined in 0x140618310; original line59；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::ThemeFetcher::download_blob | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`download_blob` | B：原定位 inlined in 0x140618310; original line164；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::ThemeFetcher::trim_cas | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`trim_cas` | B：原定位 inlined in 0x140618310; original line204；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::ThemeFetcher::validate_cas_cache_entry | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`validate_cas_cache_entry` | B：原定位 inlined in 0x140618310; original line226；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::ThemeFetcher::run | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：`run` | B：原定位 0x140618310; PDBmodule25924789B；candidate library producer; game plugin unknown；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::checked_zip::extract_with_checksums | [reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs)：`extract_with_checksums` | B：原定位 0x140d921c0; PDBmodule8455120B；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| theme_fetcher::checked_zip::verify_folder_checksums | [reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs)：`verify_folder_checksums` | B：原定位 0x140d939a0; PDBmodule8474579B；candidate implementation；完整原入口/consumer未验收 | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |

## 明确缺口

- 完整原run入口oracle和tiny-glade主题plugin/ECS消费
- 原完整reqwest feature/日志callback与A/C适配

下一闭包：完整原run入口oracle和tiny-glade主题plugin/ECS消费；原完整reqwest feature/日志callback与A/C适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json)
- [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/theme-fetcher/src/checked_zip.rs` | A,B,C | [reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | theme_fetcher::checked_zip::extract_with_checksums（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::checked_zip::verify_folder_checksums（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B主题HTTP/ETag/date/cache-control、backoff/half-open活动、CAS/ZIP校验与库内producer。：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::checked_zip::extract_with_checksums：原定位 0x140d921c0; PDBmodule8455120B；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::checked_zip::verify_folder_checksums：原定位 0x140d939a0; PDBmodule8474579B；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |
| `crates/theme-fetcher/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | theme_fetcher::ThemeFetcher::new（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::ThemeFetcher::fetch_events（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::ThemeFetcher::download_blob（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::ThemeFetcher::trim_cas（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::ThemeFetcher::validate_cas_cache_entry（candidate_business_rule_mapping；业务等价以限定证据为准）<br>theme_fetcher::ThemeFetcher::run（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B主题HTTP/ETag/date/cache-control、backoff/half-open活动、CAS/ZIP校验与库内producer。：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::new：原定位 0x14054aa00；candidate implementation; config confirmed by PDBinline and native fields；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::fetch_events：原定位 inlined in 0x140618310; original line59；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::download_blob：原定位 inlined in 0x140618310; original line164；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::trim_cas：原定位 inlined in 0x140618310; original line204；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::validate_cas_cache_entry：原定位 inlined in 0x140618310; original line226；candidate implementation；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json)<br>theme_fetcher::ThemeFetcher::run：原定位 0x140618310; PDBmodule25924789B；candidate library producer; game plugin unknown；完整原入口/consumer未验收：[evidence/foundation-unstarted-2026-10-13/themes/contract.json](../../evidence/foundation-unstarted-2026-10-13/themes/contract.json), [evidence/foundation-unstarted-2026-10-13/themes/handoff.json](../../evidence/foundation-unstarted-2026-10-13/themes/handoff.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs

[reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/checked_zip.rs)

SHA256：`8bfed2b97114ef8506180e8161570bcd269e7f1838e9319acec6160ac4803377`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `folder_crc` | 21 | 未由扫描证明；以证据限定范围为准 |
| `extract_with_checksums` | 32 | 未由扫描证明；以证据限定范围为准 |
| `verify_folder_checksums` | 73 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs

[reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs)

SHA256：`a76758b4a970b221eee114975fd3892d559e4d8dce44b80375bf366751135c22`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get` | 35 | 未由扫描证明；以证据限定范围为准 |
| `new` | 42 | 未由扫描证明；以证据限定范围为准 |
| `get` | 47 | 未由扫描证明；以证据限定范围为准 |
| `event_local_time` | 106 | 未由扫描证明；以证据限定范围为准 |
| `new` | 148 | 未由扫描证明；以证据限定范围为准 |
| `with_transport` | 167 | 未由扫描证明；以证据限定范围为准 |
| `next_fetch` | 182 | 未由扫描证明；以证据限定范围为准 |
| `retry_seconds` | 185 | 未由扫描证明；以证据限定范围为准 |
| `fetch_events` | 191 | 未由扫描证明；以证据限定范围为准 |
| `download_blob` | 240 | 未由扫描证明；以证据限定范围为准 |
| `validate_cas_cache_entry` | 265 | 未由扫描证明；以证据限定范围为准 |
| `trim_cas` | 285 | 未由扫描证明；以证据限定范围为准 |
| `run_once` | 333 | 未由扫描证明；以证据限定范围为准 |
| `run` | 392 | 未由扫描证明；以证据限定范围为准 |
| `run_with_sender` | 403 | 未由扫描证明；以证据限定范围为准 |
