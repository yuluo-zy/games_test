# LOGIC — resources/session（逐函数逻辑判等审计）

> 审计员: logic-verifier-session。日期: 2026-10-17。
> 依据: 2026-10-16-logic-verify-protocol.md；全部以 Ghidra shared 只读 **fresh 反编译** + 机器码(PDB void() 占位处已用 read_memory 核对)为准。
> 范围: session.rs 已实现业务函数（排除 derive/serde visitor/注册胶水）。未实现(无 Rust body)不在本表。

## 函数判等表

| VA | 原函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x140a9fa70 | GladeType::is_olden | `session.rs::is_olden` | **判等** | 机器码 `80 39 01 0f 94 c0`=cmp byte[+0]=1; sete（DB void() 占位使反编译成 return，机器码为准） |
| 0x14092e6a0 | GladeType::is_flowery | `is_flowery` | **判等** | cmp byte[+0]=2; sete |
| 0x14092e6c0 | GladeType::is_autumn | `is_autumn` | **判等** | cmp byte[+0]=3; sete |
| 0x140ae8320 | GladeType::is_winter | `is_winter` | **判等** | cmp byte[+0]=4; sete |
| 0x140916420 | SessionId::new | `SessionId::new` | **判等** | 机器码=call Uuid::new_v4 返回（DB 符号名错位为 game_profile::default，不影响逻辑） |
| 0x140b46000 | SessionId::save_folder_path | `save_folder_path` | **判等** | saves() + uuid fmt(join) |
| 0x140b46180 | SessionId::backup_folder_path | `backup_folder_path` | **判等** | backup_saves() + uuid fmt(join) |
| 0x140b46300 | SessionId::session_file_list | `session_file_list` | **未实现-需重做** | 原 from_iter<**RelativePathBuf**> 对 3 个相对路径(history.json/screenshot.jpg/meta.json,顺序经 rdata 0x142aef4f7/503/511 确认)映射;**Rust 返回绝对 PathBuf=save_folder.join(…)**。返回值语义相对 vs 绝对不判等 |
| 0x140b46370 | SessionMeta::assets_root | `assets_root` | **判等** | 读 glade 字节→两表(0x142aef620 名表 summer/olden/flowery/autumn/winter + 0x142aef648 长表 6/5/7/6/6)→join "assets/glade"(0x142aef51a,len 0xc)。与 Rust assets_folder 全符 |
| 0x140b463c0 | SessionMeta::write_to_disk | `write_to_disk`→`write_to_path` | **未实现-需重做** | 委托 save_session_meta_to_file 0x1417522e0 fresh 反编译: 外层 `{"version": **2**, "state": SessionMeta}`（BTreeMap insert "version"→2 + "state"→serialize）。**Rust 写 version=1**（测试也断言 1）。tmp sibling + sync + close + rename 其余一致 |
| 0x140b46480 | SessionMeta::try_load_from_path | `try_load_from_path` | **判等** | read→from_str<Value>→index_into("state")→deserialize_struct；err→Err；version 不校验（src 注释准确） |
| 0x140b46860 | SessionMeta::is_halloween2024 | `is_halloween2024` | **判等** | 遍历 mutators Vec(data@+8,len@+0x10) 元素==0 |
| 0x140b46890 | SessionMeta::is_halloween_any | `is_halloween_any` | **判等** | 遍历元素 `1 < byte` 即 >1(非 Halloween)才停；=contains 0/1 |
| 0x140b468c0 | SessionMeta::is_xmas2024 | `is_xmas2024` | **判等** | 遍历元素==2 |
| 0x140b46920 | NextSession::load | `NextSession::load` | **判等** | kind=0x8000000000000002@+0x18、id@+0x20、ignore_snapshot@+0x98=0、Vec 空(cap=8；Rust Vec::new cap=0，非可观察差异) |
| 0x140b46960 | NextSession::create | `NextSession::create` | **未实现-需重做** | 原从 96B 模板复制 12 qword(+0x18..+0x78)+32B 尾部(+0x78..+0x98)+Vec cap8；**Rust 仅保留 id+kind，+0x30..+0x98 载荷字段全清零/不建模**。字段语义 U 不得以登记通过 |
| 0x140b46a20 | NextSession::ignore_snapshot | `ignore_snapshot` | **判等** | 写 +0x98 bool + memcpy 0xa0 返回（builder 语义一致） |
| 0x1403b5150 | append_on_load<[MetaCommand;3]> | `extend_on_load` | **判等** | __rust_alloc 0xa8B+memcpy 3 命令；push 条目 tag=3+Box 载荷；len++。Rust 用 Vec 表示批量，tag=3 一致。Command 条目 +0x10/+0x18 两 qword U 已注记 |
| 0x141328ac0 | append_on_load<MetaCommandFence> | `append_fence` | **判等** | push 条目 {tag=0x8000000000000000, payload=fence Arc}。一致 |
| 0x140b46a50 | SavedSession::loading_background_path | `loading_background_path` | **判等** | join "loading_background.jpg"(0x142aef557, len 0x16=22 已读)。PDB 名 screenshot_path 错位确认 |
| 0x140b46b00 | refresh_session_list | `refresh_session_list` | **判等(限)** | 外层全核对: 清空 len + readdir spec_extend(FilterMap) + sort_by_key(SystemTime, ascending) + **手动整表反转**(0x1cB 元素对调)→ 最近在前，=Rust sort_by_key(Reverse)。逐条目 SavedSession 字段填充在无法独立反编译的内联闭包（uuid 解析/截图 exists 未逐条核） |
| 0x140b46d60 | delete_session | `delete_session` | **未实现-需重做** | 原: 构 source+deleted(uuid) 路径 → **exists() 检查 + 冲突时重生成 uuid** → rename(source,deleted) → 失败 log。**无 create_dir_all / 写标记文件 / remove_dir_all**。Rust 自造三段（create_dir_all+write ".deleted" 标记+remove_dir_all）且缺 exists 冲突处理 |
| 0x140b47840 | SessionFlowPlugin::build | `SessionFlowPlugin::build` | **未实现-需重做** | 原: insert 空 SavedSessionList(cap4)+NextSession(**kind=0x8000000000000001=LoadSavedGame**) + **register_event<DeleteSessionCmd>** + **Startup 刷新系统**(ResMut<SavedSessionList>,Option<Res<AutoSaveState>>) + **Update DeleteSession/SavingGameComplete 处理器**(run_if+or)。**不插入 ActiveSession**。Rust: 无 DeleteSessionCmd 注册/无系统注册；自造 ActiveSession 占位插入+急切 refresh+load_latest(kind=LoadOnLoad) |

## serde / 胶水（非业务，不判等）

GladeType/SessionId 的 visit_str/deserialize/serialize 诸变体（0x1403b5a20/141328d00/1403b52d0/1403b5500/141328bb0 等）、impl$13::fmt、clone/eq、closure$0 等按 AGENTS.md 计 serde visitor/注册胶水。

## 内联（无独立 VA，无法 fresh 反编译）

`SessionLoadAttempt`(from_session_id/continue_latest)、`copy_dir_all`(0x141740c00 调用方)、`NextSessionKind::decoded`、`load_saved_session_list`(body 在 refresh 内联)、`SessionMetaSource`、`ActiveSession::*`、`SavedSession::screenshot_path`。这些非独立原函数，未逐条判等。

## 汇总

- 判等: 18（含 refresh_session_list 限定）
- 未实现-需重做: 5 —— session_file_list / write_to_disk / NextSession::create / delete_session / SessionFlowPlugin::build
- 重点分歧: delete_session 自造三段流程（原无）；build 缺 DeleteSessionCmd 注册与系统装配且 ActiveSession 为自造；write_to_disk 外层 version 原=2/Rust=1；create 丢失 +0x30..+0x98 模板载荷；session_file_list 相对/绝对路径。
