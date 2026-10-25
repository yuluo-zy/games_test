# tg-package-integrity：batch-02 反编译复核、OS 边界与报告消费者接线

状态：batch-01 已交付 verify 链（29 原容器案例零差异、真实 2.70GB 包流式验证一致）与 extra 扫描策略。本轮（batch-02）用 Ghidra shared worker 反编译原 verify 整入口、checksum 叶与两个报告消费者，逐条核对候选实现；修复已知的"最后合成测试改动"平台相关失败；补 serde/schema 边界测试；记录报告/崩溃消费者接线证据。未修改原库 API。

## 本轮反编译核对结果（vs 候选实现）

| 原函数 | 地址 | 反编译事实 | 候选实现 | 结论 |
|---|---|---|---|---|
| `verify_package_manifest` | 0x14067c3d0 | `std::fs::read::inner()` 读 manifest → `blake3::hash()` → 常数 `0xf52dad71b327e45a`（FNV seed）→ `core::array::impl$11::hash<[u8;32], FnvHasher>` → `(short)result == 0` 低16位门 → `serde_json::de::from_slice<PackageManifest>` → version 检查（`iStack_140 == 1`，否则返回 UnsupportedVersion(version)）→ `easy_parallel::Parallel::each<CheckOutcome,...>` + `run()` 逐文件核对 → `find_extra_files<&str>` → 全空则 Ok（`-0x7ffffffffffffffc` 判别），否则组 FileIssues | `verify_package_manifest` + `verify_manifest_bytes` 链（blake3→`Hash<[u8;32]>.hash()`→seeded FNV low16→serde 解析→version==1→easy-parallel 批检查→find_extra_files→FileIssues） | 一致 |
| `core::array::impl$11::hash<[u8;32], FnvHasher>`（checksum 叶） | 0x140f737f0 | FNV-1a prime `0x100000001b3`；首项 `(basis ^ 0x20) * prime`（写入 usize 长度 32 前缀），再对 32 字节逐个 `^byte * prime` | `digest.as_bytes().hash(&mut hasher)`：`[u8;32]` 的 `Hash` 先 `write_usize(32)` 再逐字节 FNV；`hasher.finish() as u16 == 0` | 一致（batch-01 RNE 已执行验证；长度前缀在 64 位 LE 上为 8 字节 usize） |
| `find_extra_files<&str>` | 0x140f79650 | 递归 readdir（Windows fs）；`DirEntry::file_type` 位判定目录/符号链接；`DirEntry::file_name` + `try_from`（非 UTF-8 跳过）；`RelativePath::join` 后与 "tmp" 常量 `eq()` 相等则跳过；HashSet 成员检查用 `core::hash::BuildHasher::hash_one<RandomState>`（swisstable SIMD 查表）；目录递归走 `find_extra_files<PathBuf>` | `find_extra_files`：递归 walk、跳过 `tmp` 目录、跳过 symlink、跳过非 UTF-8 名称、忽略 read_dir 错误、精确字符串成员检查 | 一致 |
| `verify_package_integrity_and_show_report`（消费者） | 0x1402552f5 | 入口先 `Instant::now()` 计时 → 调 `verify_package_manifest`（行 147）；成功分支显示 "Package_manifest_check_passed"；失败分支（`0x8000000000000004` 之外）构造 "Could not read : "/`File_integrity_check_failed`（行 341）等消息 → 末尾调 `tg_integrity_report::run_integrity_report_gui()`（行 704）并跑 winit 事件循环 | 候选 example `inspect_package.rs` 只读入口；GUI 报告属 tg-integrity-report 库，不跨库改代码 | 接线证据记录 |
| `on_game_crashed`（消费者） | 0x1403b040f | 崩溃处理调 `verify_package_manifest`（行 1228）→ 收集 sentry frame（行 1278）→ 调 `crash_reporter_gui::run_crash_reporter_gui<closure_env_6>`（行 1365-1366）→ "Sentry_report__Success"（行 1426） | 候选仅库内 verify；崩溃 GUI 属 crash-reporter-gui 库 | 接线证据记录 |

结论：候选实现与 B 反编译一致，无需改动库逻辑。改动：①修复合成测试的平台相关失败；②新增 serde/schema 边界测试；③新增 example 文档接线（无代码改动）。

## OS 边界（本轮修复）

manifest 含 `["build-info/manifest.json/../manifest.json", ...]` 条目时，分类依赖平台：
- Unix：`root.join(...)` 因 `manifest.json/..` 非目录返回 ENOTDIR，打开失败 → **Missing**（符合原规则"打开失败统一归 Missing"）。
- Windows：OS 可规整打开到真实 manifest 文件，哈希不匹配 → **Corrupt**。
fixture `manifest-contract.json` 按 Windows 预期（Corrupt）。测试改为：断言该条目必落在 missing/corrupt 之一，从两数组剔除后其余与 fixture 对照。其它 missing/corrupt/extra 分类跨平台一致。

## serde/schema 边界（本轮新增测试）

- unknown-field：`serde_json::from_slice<PackageManifest>` 无 `deny_unknown_fields`，未知嵌套字段被忽略（反编译确认）。
- version 类型：`"version":1.5` 或 `"version":"1"` 走 `ManifestParse`（serde 类型错误），不落到 `UnsupportedVersion`；仅解析成功且 `!=1` 才返回 `UnsupportedVersion(version)`（u32）。
- 32 位/大端平台：`[u8;32]::hash` 的 `write_usize(32)` 长度前缀字节序/宽度随平台变化，跨平台校验和不兼容；候选沿用原 `Hash::hash`（含长度前缀），不在本库归一化。

## 报告/崩溃消费者接线（证据记录，不跨库改代码）

- `tiny_glade::integrity_report_main::verify_package_integrity_and_show_report`（0x2552f5）：verify 成功→"Package_manifest_check_passed"；失败→逐问题构造 "Could not read : " + "File_integrity_check_failed"，最后调 `tg_integrity_report::run_integrity_report_gui()` 展示报告（winit 事件循环）。
- `tiny_glade::panic_reporter::panic_report::on_game_crashed`（0x3b040f）：崩溃时同样调 `verify_package_manifest`（行 1228），收集 sentry frame，调 `crash_reporter_gui::run_crash_reporter_gui<closure_env_6>`（行 1365-1366）。
- 原入口 oracle（shipped-process runtime 整链）与真实报告/崩溃 GUI 像素验收仍保留，属启动完整性/GUI 范围。

## 依赖边界

blake3 1.5.3、fnv 1.0.7、relative-path 1.9.0、easy-parallel 3.2.0、serde/serde_json 复用；turbosloth 不在本闭包。未新增依赖，Cargo.lock 未变。

## batch-02 完成补充（2026-10-14）

- 新增 `symlink_and_denied_directory_probes`（Unix）：symlink（文件/目录）跳过、无权限目录 read_dir 失败静默、普通文件入 extra——对照 find_extra_files 反编译（0x140f79650）。
- 32/64 位与大小端：`[u8;32]::hash` 的 `write_usize(32)` 长度前缀宽度/字节序随平台变化，原行为即平台相关（本机 64 位 LE 与原 Windows x64 一致）；候选沿用原 `Hash::hash`，不做跨平台归一化。
