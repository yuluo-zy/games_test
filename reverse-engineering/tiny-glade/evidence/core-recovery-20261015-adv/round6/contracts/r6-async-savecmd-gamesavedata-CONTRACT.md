# Round6 CONTRACT — AsyncSaveCmd 变体 + GameSaveData 240B（save 域字段）

> 逆向 agent ① 交付。证据：session-save/ALGORITHMS §2.1（save_thread/分发变体全解）+ procs.tsv。
> 状态：**AsyncSaveCmd 判别值定案可直接落地；变体载荷字段名待取证（中优先）**。

---

## 1. AsyncSaveCmd 变体判别（O，ALGORITHMS §2.1 save_thread 0x141720680 / 分发 0x141721aa0）

| 判别 | 变体 | 行为（O） |
|---|---|---|
| 0/1/2 | （死/历史） | 无专属 case、无 debug 串（I：不存在或历史） |
| **3** | **OpenFile** | save_folder_path+"history.json"（0x142cca6a8,12）；log "AsyncSaveCmd::OpenFile("；旧态 flush→全量；存为新 CurrentSaveFile |
| **4** | **CloseFile** | log "AsyncSaveCmd::CloseFile("；公共关闭路径 |
| **9** | **CaptureSnapshot** | 参数 {Arc<SnapshotSaveData>?, …}；state==3 才有效（"No active save file, but AsyncSaveCmd::CaptureSnapshot…"）；snapshots 目录 |
| **10** | **Flush** | state==3 才有效；否则 No-active log |
| **0xb(11)** | **Shutdown** | 需 flush（标志）→ state=3 → CurrentSaveFile::flush_deferred_and_sync |

- mpmc 通道（std::sync::mpmc list/array/zero，procs 多处实例）；`AsyncSaveCmd` = `enum2$<…>`。
- **② 待办**：现源 AsyncSaveCmd 变体字段名（save-DELIVERY 标"编造需纠错"）按上表判别值
  落 3/4/9/10/0xb；载荷字段名（OpenFile 参数、CaptureSnapshot 的 Arc 载荷）**待取证**（中优先）。

## 2. GameSaveData 240B — **字段名待取证（中优先）**

- ALGORITHMS §2.6：GameSaveData 组装从 ~9 资源拷贝（历史链/时间/相机/羊/glade 字节等，
  +0x28..+0x100 密集字段）→ save_game_to_file。
- save-DELIVERY 已纠错（语义字段+自定名显式标注 U）；240B 逐字段名**待取证**
  （拆 save_game 0x140a3f470 的 GameSaveData 构造段 + Box<GameSaveData> drop 面 0x14146cb40）。
- ② 保持语义字段 + U 标注，不编造。

## 3. session create 96B+32B+24B 载荷 — **待取证（中优先）**

- `NextSession::create` 0x140b46960：96B 模板（rdx）+ 32B 尾部（r8）+ 24B 可选串/路径 + 字节
  旗标（session-DELIVERY §2；new-session-ui.asm 0x141ebb5ad）。
- 载荷字段名 U；② 保持 id+kind 语义入口 + 载荷面登记。

## 4. 证据索引
- session-save/ALGORITHMS.md §2.1（save_thread 0x141720680 变体表、分发 0x141721aa0）、
  §2.6（GameSaveData 组装）。
- procs.tsv：AsyncSaveCmd 多处 mpmc 实例、GameSaveData drop 0x14088eb40/0x140fa9980、
  Box<GameSaveData> 0x14146cb40。
