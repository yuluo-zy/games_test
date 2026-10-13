# theme-fetcher — 业务逻辑（按函数）

来源：B 版本（`tiny_glade.pdb` sha256 2d9686b8…，RNE sha256 f507a6ce…）。
VA = 0x140000000 + RVA。原始编译实例：`new` @ 0x14054aa00、`run` @ 0x140618310、
`extract_with_checksums` @ 0x140d921c0、`verify_folder_checksums` @ 0x140d939a0；
`fetch_events`/`download_blob`/`validate_cas_cache_entry`/`trim_cas` 内联于 `run`。

## ThemeFetcher::new（0x14054aa00）

1. `DirBuilder::_create()` 先于一切客户端配置；失败直接返回 `Err(io::Error)`（同时丢弃传入的 sender）。
2. 创建成功后构建 reqwest blocking ClientBuilder：超时字段 = 0x3c（60 秒）。
3. `build()` 失败走 `core::result::unwrap_failed()`（对应当前 Rust 的 `.expect(...)`）。
4. 初始化字段：cache_folder、client、events 指向空 Vec 的 Arc、etag 为空、
   sender、last_published（哨兵值）、fetch_error/events 等。
5. `create_dir_all` 先于客户端 build，与 Rust 顺序一致。

## ThemeFetcher::run / run_with_sender（0x140618310）

主循环（`while(true)`）：

1. **启动 trim（一次）**：`read_dir(cache_folder)` 逐项：
   - 非目录 → `remove_file`（rogue），记日志。
   - `modified` 年龄 > 0x127500（1_209_600 秒 = 14 天）→ `remove_dir_all`（old），记日志。
   - 其余 `verify_folder_checksums`；失败 → `remove_dir_all`（corrupt），记日志。
   - 单项错误隔离，继续下一项；整体错误记 "Error trimming CAS cache"。
   - 记日志："Verifying CAS cache..."、"Error validating CAS cache entry" 等。

2. **按时取事件**：`SystemTime::now()` 与 `next_fetch`（单调时钟）比较；到期才 `fetch_events`：
   - 读 `Date` 头，`httpdate` 解析 → `server_minus_local = server - local`（整秒），记日志 "Local timestamp: , diff: "。
   - 读 `Cache-Control`（`max_age`）；缺省 0x8ca0（36_000 秒 = DEFAULT_REFRESH）。
   - **304**：保留 etag；`next_fetch = now + refresh`；记日志 "The last events response is up-to-date."。
   - **200**：若带 `ETag` 先更新 etag（仅 200 更新），记日志 "ETag of the fetched data: "；
     必须带 `Content-Length`（缺失 → "No Content-Length header"）；
     `bytes()`；长度精确匹配（不匹配 → "Response body length does not match Content-Length header"）；
     `serde_json` 解析 `EventsResponse`；构造 `MaybePendingActiveEvent` 列表
     （初始 data = `Err(Pending)`，0x8000000000000000）；`ArcSwap::store`；
     `fetch_error = None`；`next_fetch = now + refresh`；记日志 "Fetched a new events response."。
   - 其它状态 → "Failed to fetch events. Status: {status}"。
   - **失败**：`fetch_error = Some(error)`；`next_fetch = now + retry_seconds`；
     `retry_seconds = retry_seconds * 2`，上限 300（`if (299 < x) x = 300`）。

3. **下载循环（每事件）**：遍历 events，`event.data` 为任意 `Err` 变体
   （判词：`SBORROW8(0, data[2])`，即 Result tag != 0；Ok tag = 0）即触发 `download_blob`：
   - `cache_folder.join(cid)`；`is_dir()` → 记日志 "CAS cache hit for {cid}. Skipping download."，返回 Ok(path)。
   - GET `{BLOB_URL}/{cid}`；状态 != 200 → "Failed to fetch {cid}. Status: {status}"。
   - `content_length` 存在时长度必须精确匹配（不匹配同文案）。
   - `bytes()` → `blake3::hash` 转十六进制与 cid 逐字节比较（不一致 → "Checksum mismatch"）。
   - `ZipArchive::new` → `extract_with_checksums`；成功记日志 "Fetched {cid} and saved it locally."。
   - 失败：`event.data.store(Err(Error(err)))`，记日志 "Error downloading theme blob: {err}"；
     **该 Err 会在下一轮循环再次触发下载**。
   - 下载循环与 fetch 相互独立：未到期时仍执行下载。

4. **活动选择（last-active-wins）**：遍历 events，半开区间 `start <= now < end`
   （`(start <= now) && (now < end)`，机器码两处有符号比较）才候选；
   命中即覆盖 `active = event.data 的克隆`；循环顺序 = 数组顺序，故"最后命中者胜"。

5. **fetch_error 替换**：若 `active` 为 `Err(NoActiveEvent)`（0x8000000000000002）
   且 fetch_error 为 Some → `active = Err(Error(fetch_error))`。

6. **发布（publish-only-changes）**：与 `last_published` 比较：
   - 变体不同 → 发布；同为 `Err(Error(String))` → 字符串 memcmp 相等则跳过；
   - 同为 `Ok(ActiveEvent)` → 比较 start/end（SystemTime 两个词）与 data_folder
     （反向 path components `eq_by` 相等）→ 相等跳过、不等发布。
   - 首次：`last_published` 为哨兵 → 必然发布。
   - 发布经 `std::sync::mpmc::Sender::send`；发送失败（消费者断开）→
     记日志 "Theme fetcher shutting down."，`drop_in_place<ThemeFetcher>()` 并返回。

7. **休眠**：每轮末尾 `std::thread::sleep()`；机器码 0x14061be55 `mov ecx,1; xor edx,edx`
   = `Duration::from_secs(1)`（1 秒）。

## checked_zip::extract_with_checksums（0x140d921c0）

1. 按归档顺序 `by_index`（0..len）遍历；`by_index` 失败 → `from<ZipError>`。
2. `enclosed_name()` 失败 → "Invalid file path"（0x142b593f5，长 0x11=17）。
3. `target = folder.join(name)`；名字以 `'/'` 结尾 → `DirBuilder::_create(target)`，continue。
4. 普通文件：`target.parent()` 存在且 stat 为 NotFound 时 `DirBuilder::_create(parent)`；
   `default_read_to_end` 读全量；`fs::write(target, bytes)`。
5. 写后 `Path::to_str()` 失败 → `unwrap_failed()`（对应 `.expect(...)`）。
6. 逐文件：`blake3::hash(bytes)` → checksum；并把 (path, checksum) 压入 files。
7. **folder_crc**：单例 blake3 Hasher（IV 0x6a09e667…）在遍历中按序
   `update(path)` + `update(checksum)`，循环后 `finalize()` 十六进制。
8. 生成 `.checksum`：`File::create` 失败带上下文 "Creating .checksum file"（0x142b593a1，长 0x17=23）；
   `serde_json` 序列化 `{"files":[...],"crc":...}`（先 files 后 crc 两个 serialize_entry）；
   `write_all` 失败带上下文 "Writing .checksum file"。

## checked_zip::verify_folder_checksums（0x140d939a0）

1. `fs::read(folder/.checksum)` 失败 → "Reading .checksum file"。
2. `serde_json::from_slice` 失败 → "Parsing .checksum file"。
3. 重算 `folder_crc`（blake3 逐文件 path+checksum）与清单 `crc` 不一致 → "CRC check failed"。
4. 逐清单项：`File::open(path)`，失败 with_context "Reading {path}"；
   `memmap2` 先 mmap、失败回退 `fs::read`；`blake3::hash` 与清单 checksum 不一致 →
   "Checksum failed for {path}"。
5. 额外文件不校验（不做目录遍历）。
