# theme-fetcher — 验证报告

## 比对范围

B 版本 4 个编译实例 fresh 反编译（Ghidra，只读 shared worker；jobs 在
`/tmp/tg-closures-validation-Ckh1/jobs/`）：

| 函数 | 地址 | 结果文件 |
|---|---|---|
| `ThemeFetcher::new<PathBuf>` | 0x14054aa00 | `func_0x14054aa00.txt` |
| `ThemeFetcher::run`（含内联 fetch_events/download_blob/validate_cas_cache_entry/trim_cas） | 0x140618310 | `func_0x140618310_full.txt`（decompiled_code 120000 字符截断，已覆盖全部业务段） |
| `checked_zip::extract_with_checksums` | 0x140d921c0 | `func_0x140d921c0.txt` |
| `checked_zip::verify_folder_checksums` | 0x140d939a0 | `func_0x140d939a0.txt` |

关键字符串/常量经 `read_memory` 确认（ETag 日志、length 文案、Checksum mismatch、
.checksum 上下文、CAS 日志等）；睡眠时长经机器码 0x14061be55 确认
（`mov ecx,1; xor edx,edx; call` = `Duration::from_secs(1)`）。

## 差异与修复

### D1 — 下载循环触发条件（唯一实差，已修复）

- **地址证据**：`run` @ 0x140618310。下载循环对每个事件
  `if (SBORROW8(0, ppppppppuVar25[2])) goto code_r0x00014061a2cd;`
  —— 检查的是 Result 的判别字（ArcInner 偏移 +2；Ok 判词为 0，全部 Err 变体非 0），
  命中即进入下载路径块 `code_r0x00014061a2cd`（0x14061a2cd，内联 download_blob：
  is_dir 短路 → BLOB URL → status/content_length/blake3 cid → ZipArchive+extract）。
  因此 **任意 Err（含上一轮失败缓存的 `Err(Error)`）都会在每轮 1s 循环重试下载**。
- **Rust 差异**：`run_once` 原本 `if matches!(..., Err(EventDataError::Pending))`，只重试 Pending，
  失败被缓存、直到新事件响应才再试。
- **修复**（`reconstruction/shared-foundation/crates/theme-fetcher/src/lib.rs` `run_once` 下载循环，
  附地址证据注释）：
  - 循环条件改为 `if event.data.load().as_ref().is_err()`；
  - 更新 `run_once` doc 注释。
- **测试**：`checksum_failure_does_not_create_cas_and_future_events_are_downloaded`
  新增第三个 `response(200, zip_bytes())` fixture，第二次 `run_once` 后
  `requests.len()` 断言 2 → 3（并保留 `root.join("wrong").exists() == false`）。

## 确认匹配的关键行为（未改动）

- `new`：create_dir_all 先于客户端；超时 60s（0x3c）；build 失败 `.expect`。
- fetch_events：httpdate Date → server_minus_local；Cache-Control max_age 缺省 36000（0x8ca0）；
  304 保留 etag + 更新 next_fetch；ETag 仅 200 更新；Content-Length 缺失报错、长度精确匹配；
  serde_json EventsResponse；事件 data 初始 Err(Pending)。
- 取事件失败：`fetch_error=Some`、retry 翻倍封顶 300。
- download_blob：is_dir 短路；BLOB 状态 !=200 报错；content_length 存在时精确匹配；
  BLAKE3 cid 比对（"Checksum mismatch"）；ZipArchive → extract。
- 活动选择：半开 `start<=now<end`、last-active-wins、fetch_error 替换 NoActiveEvent、
  publish-only-changes（变体/字符串/path 比较）。
- trim_cas / validate_cas_cache_entry：非目录 remove_file；>14 天（0x127500=1_209_600）remove_dir_all；
  verify 失败 remove_dir_all；单项错误隔离。
- checked_zip：归档顺序、"Invalid file path"、"Checksum failed for {path}"、CRC 先校验、
  mmap 回退 fs::read、folder_crc 更新序（path 后 checksum）、.checksum 上下文文案全部一致。

## 测试结果

`/tmp/tg-closures-validation-Ckh1/native`（离线、`--locked`）：

- `cargo test -p theme-fetcher --offline --locked`：**11/11 通过**（基线保持绿，含更新后的测试）。
- `cargo clippy -p theme-fetcher --offline --locked --all-targets -- -D warnings`：**无告警**。

## 剩余缺口

- `pool_max_idle_per_host(0)`：reqwest ClientBuilder 为 0x290 字节不透明结构，
  反编译无法直接读出该字段；保留既有分析结论，未发现矛盾。
- `run` 的 `decompiled_code` 被 Ghidra 截断在 120000 字符（下载路径 store 收尾与循环回边处）；
  已覆盖全部业务逻辑段，截断段无独立业务行为。
- 未对 `run` 的 send 值构造与 mpmc `Sender::send` 返回值编码逐位核对（语义已确认一致）。
