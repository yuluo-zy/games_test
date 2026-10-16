# Round7 CONTRACT — 链1：save/load 主链（AsyncSaveCmd 载荷 / GameSaveData / Decorators / Terrain / 5 域）

> 逆向 agent ① 交付。证据：ALGORITHMS §2.1/§2.6 + save_game decomp（0x140a3f470 81KB）+
> 既有 round4/5/6 契约。状态：**判别已对齐可直接落地；载荷字节布局待取证（中优先）**。

---

## 1. AsyncSaveCmd 变体 + 判别（O，ALGORITHMS §2.1 save_thread 0x141720680）

| 判别 | 变体 | 载荷（O 面 / 字段名待证） |
|---|---|---|
| 3 | OpenFile | save_folder_path+"history.json"（0x142cca6a8,12）；建 ShadowFile；旧态 flush→全量（字段 U） |
| 4 | CloseFile | 载荷含 session id（语义；分段字段 U） |
| 5 | ChangeSection | 改节 + mark_dirty（字段 U） |
| 6 | ReplaceEdits | 截断 + 追加编辑（字段 U） |
| 7 | BackupSave | `history-auto` 前缀备份（kind: BackupSaveFileKind） |
| 8 | ManualBackup | execute_save_cmd 落 default（kind） |
| 9 | CaptureSnapshot | {Arc<SnapshotSaveData>, …} → snapshots 目录（字段 U） |
| 10 | Flush | 无载荷（=10） |
| 0xb(11) | Shutdown | {flush: bool} |

- **② 待办**：现源判别值已对齐（round6 r6-async-savecmd）；载荷字段名（OpenFile 参数、
  ChangeSection 节/区间、ReplaceEdits 编辑载荷、BackupSave/ManualBackup 目标路径、CaptureSnapshot
  Arc 载荷）**待取证**（拆 save_game 0x140a3f470 构造段 + save_thread 消费段，中优先）。

## 2. GameSaveData 240B — **待取证（中优先）**

- ALGORITHMS §2.6：从 ~9 资源拷贝（历史链/时间/相机/羊/glade 字节等，+0x28..+0x100 密集字段）
  → save_game_to_file。240B 逐字段名待拆 save_game 构造段（Box<GameSaveData> drop 0x14146cb40 面）。
- ② 保持语义字段 + U 标注（save-DELIVERY 已纠错），不编造。

## 3. Decorators JSON serde + 0x16B 子记录 + restore 逐槽 — **部分定案，逐字段待取证**

- 0x28B 记录 {u32 id, 8B×4, u32×2} + ID 旋转 XOR 16B 掩码 0x1429cd080 + 子侧状态公式（round6
  r6-decorators-serde，O）。
- JSON 逐字段名 + 0x16B 子记录逐槽（+0x52 状态字节）**待取证**（拆 capture serde）。
- restore 逐槽接线（round6 §1）：遍历 0x28B → ID 解码 → FUN_140b776d0 还原 → 写 DecoratorInfo。
  **可直接落地**（crate take/restore 已存在，对齐节格式）。

## 4. Terrain strokes 格式 — **待取证**（tg_terrain serde；flate2 解压入口 O @0x142cdc800）

## 5. 快照 5 域（Roofs/WallColors/GateColors/Trees/Raster）— **待取证（中优先）**

- 节序 O（round5 c-apply-targets §1：Version→Walls→Roofs→Decorators→WallColors→GateColors→
  Terrain→Trees→Raster）；各域 apply 函数/目标资源映射待取证。
- ② 保持登记不伪造写入。

## 6. 证据索引
- ALGORITHMS §2.1/§2.6；round5/decomp-r5-0x140a3f470.txt；round6 r6-async-savecmd/r6-decorators-serde。
- crate systems/save/save_game.rs（AsyncSaveCmd 判别已对齐 132-188 行）。
