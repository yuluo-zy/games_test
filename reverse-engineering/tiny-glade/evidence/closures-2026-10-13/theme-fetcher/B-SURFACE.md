# theme-fetcher — B 发射面原函数清单（骨架提取）

来源：`skeleton-enriched/B/crates/theme-fetcher/src/*.enriched.json.gz`（PDB 绑定 B 发行，
`tiny_glade.pdb` sha256 2d9686b8…，RNE sha256 f507a6ce…）。VA = 0x140000000 + RVA。

## lib.rs（16 函数，9 procedure_instances，7 inline_sites）

| 原函数 | RVA | VA | compiled | callers |
|---|---|---|---|---|
| `theme_fetcher::ThemeFetcher::new<std::path::PathBuf>` | 5548544 (0x54AA00) | 0x14054AA00 | 是 | 1（caller_entry 4001648） |
| `theme_fetcher::ThemeFetcher::run` | 6390544 (0x618310) | 0x140618310 | 是 | 1 |
| `theme_fetcher::ThemeFetcher::trim_cas` | — | — | 无独立实例（内联） | 0 |
| `theme_fetcher::ThemeFetcher::validate_cas_cache_entry` | — | — | 无独立实例（内联） | 0 |
| `theme_fetcher::ThemeFetcher::fetch_events` | — | — | 无独立实例（内联） | 0 |
| `theme_fetcher::ThemeFetcher::download_blob` | — | — | 无独立实例（内联） | 0 |
| `theme_fetcher::impl$0::run::closure$0` / `closure$1` | — | — | 无 | 0 |
| `theme_fetcher::impl$0::fetch_events::closure$0` | — | — | 无 | 0 |

（其余：serde_json de 实例、std 函数 = 依赖，不计游戏逻辑。）

## checked_zip.rs（10 函数，2 procedure_instances，8 inline_sites）

| 原函数 | RVA | VA | compiled | callers |
|---|---|---|---|---|
| `theme_fetcher::checked_zip::extract_with_checksums<Cursor<Bytes>>` | 14229952 (0xD921C0) | 0x140D921C0 | 是 | 1 |
| `theme_fetcher::checked_zip::verify_folder_checksums` | 14236064 (0xD939A0) | 0x140D939A0 | 是 | 1 |
| `checked_zip::_::impl$0::{deserialize,serialize}` + visitor | — | — | serde 派生 | 不计 |

## 对照结论（当前 Rust 实现 coverage）

Rust `lib.rs`：`new`/`with_transport`/`fetch_events`/`download_blob`/`validate_cas_cache_entry`/
`trim_cas`/`run_once`/`run`/`run_with_sender`/`event_local_time`/`ReqwestTransport`/`ThemeFetcher`。
Rust `checked_zip.rs`：`FolderChecksumInfo`/`folder_crc`/`extract_with_checksums`/`verify_folder_checksums`。

库内函数名一一对应；待 fresh 反编译核对 `new`/`run`/`extract_with_checksums`/`verify_folder_checksums`
及内联业务（fetch_events/trim_cas/download_blob）数值与分支语义。

## 关键数值/边界（差分核对点）

`DEFAULT_REFRESH=36000`、`CAS_MAX_AGE=1_209_600`、retry 上限 300、retry 初值 1、
half-open `start<=now<end`、负 skew clamp `max(0)`、ETag 在 304/缺头时保留、length 精确匹配、
BLAKE3 cid 校验、ZIP checksum 目录校验。
