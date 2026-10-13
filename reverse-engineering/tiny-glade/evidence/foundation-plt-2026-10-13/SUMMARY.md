# paths / lang-lib / tabloid 原生复核摘要

2026-10-13。共享 Ghidra B 后端（sha256 `cf048e27...`，program `/tiny-glade.exe`）逐函数反编译复核三个基础库；本目录 `functions/*.txt` 为 29 个函数反编译原文，raw asm 交叉来自 `evidence/shared-foundation-2026-10-13/paths-language/lang-root.asm.txt`。

## 完整性结论

| 库 | 完整发射清单 | 现有实现覆盖 | 无实现功能 |
|---|---|---|---|
| paths | 10 函数（module 637/1834，见 paths-language/procedures.json）：config/saves/backup_saves/tmp/deleted_saves/installed/standard_app_root_path/find_standard_app_root_path/initialize_paths/ensure_path | 全部实现并逐函数核对匹配 | 无 |
| lang-lib | 6 根 + 2 方法（module 636/1825/1826）：lang_pack_path/load_locale/load_locale_impl/load_last_used_locale/lookup/game_locale + LanguageLibrary::{get,try_from_yml} | 全部实现并逐函数核对匹配 | 无（CSV/difference/dev 工具在 B 发射面中不存在） |
| tabloid | publish/subscribe/take（泛型，全部单态化仅 DirtyTiles 与 ColliderDiff 两类） | 通用实现全部匹配 | 无（游戏类型 State/StateDelta 实现在 country-core 库外） |

## 逐函数核对

- **paths**：`initialize_paths`(0x140b9df10) 目录创建顺序 saves→beta-saves→tmp→deleted-saves→backup-saves→config 根、失败保留先前目录、先建目录后 `current_dir()`、重初始化 panic；getter(0x140b9db30 等) 未初始化 unwrap_failed panic；`find_standard_app_root_path`(0x140b9dc80) KnownFolder→USERPROFILE fallback。均在 `functions/*.txt`。
- **lang-lib**：`load_locale_impl`(0x140b9c4a0) 请求包→失败 log+en-US 回退→失败返回 Err；LIBRARY 先存→env 检查→FALLBACK 条件加载。`lookup`(0x140b9d480) 两级查找、未初始化 log+返回 key。`LanguageLibrary::get`(0x141848140) entries→missing-cache 探测→log→format+insert `?_{key}`。`load_last_used_locale`(0x140b9d180) 文件文本不 trim、失败 en-US。`game_locale`(0x140b9d9b0) 未初始化 en-US。
- **tabloid**：`publish`(0x141cf6500) sequence+1→retain 闭包（strong_count==1 回收，否则写锁 apply_to+更新 sequence）→逐订阅 pending 共享新批次→末尾 push。`Subscriber::take`(0x141247c90) 取走清空 pending→更新 last_sequence→owning read guard；无 pending 返 None。`subscribe`(0x141247b10) 新订阅 last_sequence=0/pending=None。

## TG_DISABLE_LANG_FALLBACK 判别式（已解决）

`std::env::_var` 的 Result 判别式在 raw asm（load_locale_impl 0x140b9ca67 段、find_standard_app_root_path 0x140b9dc80 段）中三路分发：`0x8000000000000000`→加载 fallback、`0x8000000000000001`→跳过、`0`→加载。跨两个函数交叉验证 **`0x8000000000000001`=Ok**、`0x8000000000000000`=Err(NotPresent)、`0`=Err(NotUnicode)。因此现有 `is_err()` 语义正确：Ok 跳过 fallback、两种 Err 加载 fallback。**不改行为**。

## load_locale 写失败偏差（登记）

`load_locale`(0x140b9c300) 原始对 `last_used_locale` 写失败**静默丢弃**（raw asm 仅 anyhow Error drop，无 log 调用）。候选保留 `log::warn!` 为**登记偏差**（用户确认保留），代码未改。

## tabloid 实例归属修正

原 utils-tabloid/CONTRACT.md 把 0x141453350/0x141453c00/0x141454290/0x141454410/0x141454590 全部标为 ColliderDiff 族。fresh 反编译修正：

| 通道 | State/StateDelta | publish | subscribe | take |
|---|---|---|---|---|
| DirtyTiles | DirtyTiles / DirtyTiles | 0x141cf6500, 0x1425db920, 0x141453350 | 0x141247b10, 0x141cf68e0, 0x141292860, 0x141454290 | 0x141247c90, 0x1412929e0, 0x141454590 |
| ColliderDiff | ColliderDiff / ColliderDelta | 0x141453c00 | 0x141454410, 0x1412926e0 | （未定位，泛型覆盖） |

DirtyTiles State 契约：`new_clear`(0x140960ac0)=512B 全零（`State::empty`）；`apply_to`(0x140960ef0)=逐 u64 按位 OR（`StateDelta::apply_to`）。真实消费者：autoclutter/stairs（DirtyTiles）、structural_graph（ColliderDiff）——在 country-core 库外。完整生产→消费接线图与缺口闭合步骤见 [TABLOID_WIRING_GAP.md](TABLOID_WIRING_GAP.md)。

## 交付改动

- `crates/tabloid/tests/dirty_tiles.rs`：DirtyTiles 语义契约测试（新增，闭合 tabloid:06 库内契约）。
- `crates/paths/src/lib.rs`：初始化限制边界测试（新增，验证 paths:05）。
- `crates/lang-lib/src/lib.rs`：仅注释登记偏差（无行为改动）。

## 验证

`cargo test -p lang-lib -p tabloid -p paths`、严格 all-targets Clippy、fmt --check 及 verify_locale_policy 5 模式探针见本轮交付记录。
