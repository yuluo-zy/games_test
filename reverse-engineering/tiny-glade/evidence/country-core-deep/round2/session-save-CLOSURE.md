# session-save 第二轮 U 项闭合报告（tg_a3，2026-10-14）

针对 gaps-v1.json session-save.digest_gaps 6 项 + 特别任务（S_PUB32=0 查证）。
工具：v2 索引 + 定点反编译（tg_a3r2 会话）+ capstone/rne-read。证据文件：
`round2/jobs/r2-xrefs-out.json`、`round2/decomp-*.txt`、`session-save/decomp/*`。

## 1. NextSessionKind 变体 1/2 语义 —— 部分闭合（消费点已证，生产点卡住）

**消费点（O）**：load_game 0x14146d030 行 568-585：`decoded = kind ^ 0x8000000000000000`，
`sel = min(decoded, 2)`；`sel==1` → parse_saved_game_data（`Making save file backup for
upgrade`→`Reading file data`→`Parsing JSON`→`Version`→`BackwardCompatibility`→`Camera`→
`Time of day` 全串路径）；`sel==2` → 内存路径（manager 状态字节 +9:=2、无快照标记
+0xd:=0xffffffff、on_load 命令回放）。**语义：1=从存档文件解析加载（含 upgrade 备份）、
2=常规加载**（NextSession::load 0x140b46920 写 0x8000000000000002，O）。
on_load 列表 fence = 0x8000000000000003（O）。

**生产点（卡点）**：get_xrefs_to 0x140b46920/0x140b46960/0x140b46a20 仅返回 pdata
DATA 引用（0x14369F900/0x14369F90C 12B 步长 = RUNTIME_FUNCTION 表），无直接调用者。
NextSession::create 0x140b46960 机器码（capstone 全解）：从 96B 模板（rdx:+0..+0x60，
其中 [rdx+0] 16B → out+0x18 即 kind 槽）与 32B（r8 → out+0x78..+0x98）构造；
kind 值由调用方模板决定 → 变体 1 的生产者在 UI/会话流（vtable 间接调用，Ghidra
xref 索引未覆盖）。**排除项**：v2 索引无 load_latest 符号（wp13 该名为误记）。

## 2. backup_folder_path 叶名 —— 闭合（无叶名，独立根目录）

机器码对比（capstone）：save_folder_path 0x140b46000 与 backup_folder_path
0x140b46180 逐指令同构，唯一差异 = 根目录调用 **0x140b9db60（=paths::saves()）vs
0x140b9db90（=paths::backups()）**；两者共用同一 fmt::Arguments 静态 @0x142aef328
（= {drop fn 0x1405d45f0, …}，0x1405d45f0 = Vec 类析构器），格式化结果 = uuid 串，
join 叶名 = uuid（无额外叶名）。**结论：backup_folder_path = paths::backups()/<uuid>
——备份在独立顶级目录，不在 saves()/uuid/ 下**。session.rs .rdata 串块（0x142aef3a0-
0x142aef620 全 dump）确无 "backups" 串（它在 paths.rs）。附带闭合：
- 主题子目录表 @0x142aef620（ptr 表）/0x142aef648（len 表）= **{summer(6), olden(5),
  flowery(7), autumn(6), winter(6)}** → **GladeType 0=Summer**（值域 0..4 全闭合）。
- SessionMeta::assets_root 0x140b46370 = join("assets/glade", 主题表[+0x58 字节])。
- SessionMeta::write_to_disk 0x140b463c0 = save_folder_path + "meta.json" +
  save_session_meta_to_file（0x1417522e0）。
- "snapshots" 目录名在串块 0x142aef4e8（"snapshotshistory.json…"），与 save_thread 侧
  "snapshots"（0x142ccaf28）互证。

## 3. execute_save_cmd 变体 0/1/2 —— 半闭合（0/1/2 无专属 case/无 debug 串）

switch（0x141721aa0 行 79）case 集 = {3,4,5,6,7,9,10} + default（行 2651）。
- 变体 8=ManualBackup 落 **default 区**（2651-3230 含 `AsyncSaveCmd::ManualBackup`/
  `Copying inactive save file to backup`/`Writing backup from shadow copy`/
  `Error flushing history file for backup`）——Ghidra 把 8 并入 default 或表项折叠。
- 变体 0/1/2：**无专属 case、无 debug 串**（全部 `AsyncSaveCmd::*` 串 = 8 个有名
  变体 + Shutdown 由 0xb 判定）。default 区其余逻辑 = 按载荷比对文件身份字节
  （param_2+0x53/+0x299 等）与 "snapshot"（0x746f687370616e73 常量）处理。
  **结论（I）**：0/1/2 为死/历史变体或不存在（枚举首变体 3=OpenFile）；精确判定需
  频道发送端清单（send 0x1408da280 的 xref 不可得——Ghidra 对间接调用无索引，卡点）。
- OpenFile（3）体内含：join "snapshots"（0x142ccaf28）、`Rewriting history file`、
  `Saving the game to a temp file`、`creating snapshot folder`、`Snapshot GC thread
  finished in ` → 开档即触发历史重写与快照 GC 编排（O）。

## 4. update_loading_screen 的 UUID 键 —— 闭合到占位 UUID

16B 常量 @0x1429503e0 = `0x33333333333333333333333333333333` = 字符串
"3333333333333333" = 占位 UUID **33333333-3333-3333-3333-333333333333**（O）。
两张 hashbrown 表 @param+0x70/+0xd8 以此键存取 0xd0B 条目 {state tag@+0（2=完成类，
drop 前判定）、progress u32@+0x58（源 @+0x5b0）、+0x120 等}；插入写 0x80 控制字节、
删除写 0xff 墓碑、rehash 0x140d3fa40。**含义（I）**：LoadingScreenProgress 资源内的
按会话/资产 16B id 索引的加载进度条目；占位 UUID 为“无会话”态（boot 屏）固定键。
业务增删时机仍 U（卡点：需 loading 流程宿主系统反编译）。

## 5. try_approx_parse_save_file_identity —— 闭合到扫描规则

0x141e58f20（v2: save_file_identity.rs）：分配 0x800B 缓冲 → Windows ReadFile →
UTF-8 转换 → **搜索字面量 `"Identity"`（含引号，串 @0x142f92a40 len 10）** → 解析其
后 JSON 值为 SaveFileIdentity。closure 0x141673140 = 失败回退：SystemTime 取时 +
FUN_14145aa70 二次尝试 + 文件元数据派生（字段 U）。`derive_fallback_save_file_identity`
名（wp13）与 closure 行为一致（I）。批量串 `"Identity"` 与 SaveFileSection::json_key
"Identity"（len 8，rdata 0x142ace218 块）互为印证（搜索用带引号 10 字符变体）。

## 6. 特别任务：resources/session.rs S_PUB32=0 查证 —— “无独立符号”不成立

- **v2 CU 索引（sysres-procedures-v2.json）resources/session.rs = 32 个过程**（含
  hashed 全名：SessionId::{new,save_folder_path,backup_folder_path,session_file_list}、
  SessionMeta::{assets_root,write_to_disk,try_load_from_path,is_halloween*}、
  NextSession::{load,create,ignore_snapshot,append_on_load×2}、SavedSession::
  loading_background_path、refresh_session_list、delete_session、GladeType::is_*、
  serde 8 项）。gaps-v1 的 "v2=0" 是**文件键不匹配**（索引键 session.rs vs 源码
  session/mod.rs）造成的统计假象，非事实。
- **非全内联**：这些地址是独立出线函数（第一轮 10 个全部反编译成功、非平凡控制流）。
- **直接调用点存在**：反编译语料内 FUN_140b46000（save_folder_path）×14 出现于
  save_game、check_load_completion、is_session_ok_to_load、backup_folder_path、
  session_file_list、loading_background_path、delete_session、load_game、save_thread、
  execute_save_cmd（10 文件）；0x140b46180 ×2、0x140b46d60/0x140b46a50/0x140b46480/
  0x140b463c0/0x140b46300 各 ≥1。
- **ghidra get_xrefs_to 仅返回 pdata DATA 引用**（0x14369F900 系，12B RUNTIME_FUNCTION）
  = xref 索引对游戏内直接调用未收录（非“无调用”证据）。
- **结论**：S_PUB32=0 仅表示这些函数未进 public symbol 表（非导出/被 strip），
  CU 级符号与调用关系真实存在；恢复以 v2 索引地址 + 机器码为准，无“全内联”障碍。

## 仍卡住的 U（已排除候选注明）

1. NextSessionKind 变体 1 的生产者（模板填充点）——排除：非 load/create 自身；卡：
   UI 会话流为 vtable 间接调用，Ghidra xref 无索引。
2. execute_save_cmd 0/1/2 存在性——排除：8 个有名变体 + 0xb 之外无 debug 串；
   卡：需发送端清单。
3. update_loading_screen 条目增删时机——排除：非网格/着色器库本身；卡：宿主 loading
   流程系统未反编译（属 render/菜单域）。
4. try_approx_parse 回退字段（closure 0x141673140 元数据字段序）。
5. on_load 条目头 {0, 0x80000000} 的字段含义（MetaCommand 头，U）。
