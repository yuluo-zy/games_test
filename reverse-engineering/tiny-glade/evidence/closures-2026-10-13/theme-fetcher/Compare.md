# theme-fetcher — 原函数 vs Rust 逐项对照

对照基线：B 版本 fresh 反编译（`/tmp/tg-closures-validation-Ckh1/jobs/`），
Rust 源为 `reconstruction/shared-foundation/crates/theme-fetcher/src/{lib.rs,checked_zip.rs}`。

## 1. `new<std::path::PathBuf>` @ 0x14054aa00 — 匹配

| 行为 | 原 | Rust | 结论 |
|---|---|---|---|
| create_dir_all 先于客户端 | `DirBuilder::_create()` 在 `Client::builder()` 之前；失败即返回 Err | `fs::create_dir_all(&cache_folder)?;` 在前 | 匹配 |
| 超时 | builder 字段 0x3c = 60s | `.timeout(Duration::from_secs(60))` | 匹配 |
| build 失败 | `core::result::unwrap_failed()` | `.expect("original ThemeFetcher client construction failed")` | 匹配 |
| 连接池 `pool_max_idle_per_host(0)` | builder 为 0x290 字节结构，配置不透明，无法从反编译直接读取 | `.pool_max_idle_per_host(0)` | 无法直接观察；与既有分析一致，未发现矛盾 |
| sender 保留 | 构造参数写入结构体 | `fetcher.sender = Some(sender)` | 匹配 |

## 2. `run` / `run_with_sender` @ 0x140618310（含内联 fetch_events/download_blob/trim_cas/validate_cas_cache_entry）

| 行为 | 原证据 | Rust | 结论 |
|---|---|---|---|
| 启动 trim 一次 | run 内联 read_dir + 逐项 remove_file/remove_dir_all；记日志 "Verifying CAS cache..." 等 | `run_with_sender` 首行 `trim_cas(SystemTime::now())` | 匹配 |
| 非目录 → remove_file | trim 内联 `if !is_dir → remove_file` | `trim_cas` `!metadata.is_dir() → fs::remove_file` | 匹配 |
| 年龄 > 14 天 → remove_dir_all | `0x127500 < age`（=1_209_600） | `age > CAS_MAX_AGE`（1_209_600） | 匹配 |
| 校验失败 → remove_dir_all | verify 失败分支 remove_dir_all | `verify_folder_checksums(&path).is_err() → remove_dir_all` | 匹配 |
| Date 解析 server_minus_local | httpdate → `server_minus_local = server - local`；日志 "Local timestamp: , diff: " | `fetch_events` date 解析、`server_minus_local = server.wrapping_sub(local)` | 匹配 |
| Cache-Control 缺省 | `max_age` 缺省 0x8ca0 = 36000 | `.unwrap_or(DEFAULT_REFRESH)`（36_000） | 匹配 |
| 304 | 日志 "The last events response is up-to-date."，`next_fetch = now + refresh` | `Ok((NotModified, refresh))`；run_once 更新 next_fetch | 匹配 |
| ETag 仅在 200 更新 / 304 或缺头保留 | 仅 200 分支写 etag 字段（0x50） | `200 => if let Some(etag) = response.etag`；304 不触碰 | 匹配 |
| Content-Length 缺失 → 报错 | events 路径 content_length None → 错误（"No Content-Length header"） | `.context("No Content-Length header")?` | 匹配 |
| 长度精确匹配 | 200 分支长度比较失败 → "Response body length does not match Content-Length header" | `body.len() as u64 != expected → bail!` | 匹配 |
| 新事件初始 Pending | fetch closure 以 0x8000000000000000 初始化 data | `ArcSwap::from_pointee(Err(EventDataError::Pending))` | 匹配 |
| 取事件失败 retry 倍增封顶 | `local_488 = local_488 * 2; if (299 < local_488) local_488 = 300;` | `(retry_seconds * 2).min(300)`；初值 1 | 匹配 |
| fetch_error 成功清空 / 失败置 Some | Replaced 分支清 fetch_error；失败分支置错误字符串 | `Replaced => None`；`Err => Some(...)` | 匹配 |
| **下载循环触发条件** | **`if (SBORROW8(0, data[2])) goto download` —— tag != 0，即任意 Err 变体（Pending 与下载错误都重试）** | **改前仅 `Err(EventDataError::Pending)`** | **差异，已修复** |
| is_dir 短路下载 | 下载路径先 `is_dir()`；日志 "CAS cache hit for {cid}. Skipping download." | `path.is_dir() → return Ok(path)` | 匹配 |
| BLOB 状态 != 200 | "Failed to fetch {cid}. Status: {status}" | 同文案 | 匹配 |
| blob 长度（可缺省） | download 路径 content_length Some 才比较 | `.is_some_and(|l| body.len() as u64 != l)` | 匹配 |
| BLAKE3 cid 校验 | blake3::hash → hex → memcmp 与 cid；失败 "Checksum mismatch" | `blake3::hash(&body).to_string() != cid → bail!` | 匹配 |
| ZipArchive + extract | `ZipArchive::new` → `extract_with_checksums`；成功日志 "Fetched {cid} and saved it locally." | 同 | 匹配 |
| 下载失败日志 | "Error downloading theme blob: {err}" | `map_err(EventDataError::Error(...))` | 匹配 |
| 半开区间 | `(start <= now) && (now < end)` 两条有符号比较 | `is_active_interval = start <= now && now < end` | 匹配 |
| last-active-wins | 循环内每次命中覆盖 active | 循环内 `active = ...clone()` | 匹配 |
| fetch_error 替换 NoActiveEvent | `active==NoActiveEvent && fetch_error!=None → 替换` | `Some(error) if matches!(active, Err(NoActiveEvent))` | 匹配 |
| publish-only-changes | 变体比较 / Error 字符串 memcmp / Ok 比较 start、end 与 path components eq_by | `last_published.as_ref() == Some(&active)`（派生 PartialEq 比较 data_folder+start+end） | 匹配 |
| send / 断开退出 | `std::sync::mpmc::Sender::send`；失败日志 "Theme fetcher shutting down." + drop + return | `sender.send(value).is_err() → return` | 匹配 |
| 睡眠 1s | 机器码 0x14061be55 `mov ecx,1; xor edx,edx; call sleep` = Duration(1,0) | `Duration::from_secs(1)` | 匹配 |

## 3. `extract_with_checksums` @ 0x140d921c0 — 匹配

| 行为 | 原证据 | Rust | 结论 |
|---|---|---|---|
| 归档顺序遍历 | `do { by_index } while (uVar11 < len)` | `for index in 0..archive.len()` | 匹配 |
| by_index 失败 | `from<ZipError>` | `archive.by_index(index)?` | 匹配 |
| enclosed_name 失败 | "Invalid file path"（0x142b593f5 长 17） | `.context("Invalid file path")` | 匹配 |
| `'/'` 结尾 → 建目录 | 末字符 `'/'` → `DirBuilder::_create(target)` | `file.name().ends_with('/') → create_dir_all` | 匹配 |
| 父目录创建 | 原：`parent()` 存在且 stat NotFound 才 create | Rust：`if let Some(parent) → create_dir_all(parent)` | 行为等价（幂等），非差异 |
| 写文件 | `default_read_to_end` + `fs::write` | `read_to_end` + `fs::write` | 匹配 |
| to_str 失败 panic | `try_from` 失败 → `option::unwrap_failed()` | `.expect("theme checksum path must be UTF-8")` | 匹配 |
| 逐文件 blake3 | 每文件 `blake3::hash` | `blake3::hash(&bytes)` | 匹配 |
| folder_crc | 单 hasher 循环 `update(path)`+`update(checksum)`，finalize hex | `folder_crc` 同序 update | 匹配 |
| .checksum 创建上下文 | "Creating .checksum file"（0x142b593a1 长 23） | `File::create(...).context("Creating .checksum file")` | 匹配 |
| JSON 结构 | serialize_entry files 再 crc | `serde_json::to_writer` 序列化 `{files, crc}` | 匹配 |
| 写清单上下文 | write_all 失败带 ref<str> 上下文 | `.context("Writing .checksum file")` | 匹配 |

## 4. `verify_folder_checksums` @ 0x140d939a0 — 匹配

| 行为 | 原证据 | Rust | 结论 |
|---|---|---|---|
| 读清单上下文 | "Reading .checksum file" | 同 | 匹配 |
| 解析清单上下文 | "Parsing .checksum file" | 同 | 匹配 |
| CRC 先校验 | blake3 循环 + memcmp → "CRC check failed" | `folder_crc(&files) != info.crc → bail!` | 匹配 |
| 逐项打开上下文 | `with_context` "Reading {path}" | 同 | 匹配 |
| mmap 优先、fs::read 回退 | `memmap2::os::file_len/map` → 失败 `fs::read` | `MmapOptions::new().map` → `fs::read` fallback | 匹配 |
| 单文件校验失败 | "Checksum failed for {path}" | 同 | 匹配 |
| 忽略额外文件 | 仅校验清单路径，无目录遍历 | 同 | 匹配 |

## 差异汇总

| # | 差异 | 地址证据 | 处置 |
|---|---|---|---|
| D1 | 下载循环触发条件：原任意 Err（含失败缓存）都重试；Rust 仅 Pending | run @ 0x140618310：下载循环判词 `if (SBORROW8(0, data[2])) goto download`（Result tag != 0），下载路径块 `code_r0x00014061a2cd` | **已修复**（lib.rs `run_once` 下载循环改为 `is_err()`）；测试同步更新 |

## 未直接可观察项

- `pool_max_idle_per_host(0)`：reqwest builder 为不透明大结构，反编译不可见；保留既有结论。
- `run_once` 返回 None（publish-only-changes）与 send 发送值的具体时序在反编译中表现为 goto/状态位，语义与 Rust 一致。
