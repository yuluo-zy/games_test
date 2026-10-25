# Round6 批次 2 DELIVERY（2026-10-16 第六轮 ②，避开三区）

> 实现对象：① round6 批次 2 两契约（r6-decorators-serde / r6-async-savecmd-gamesavedata）。
> 文件权内（systems/save/save_game.rs、systems/save/snapshot.rs）。**未碰 rhapsody/roof/wall
> （含 snapping/signifier）**——decorator_storage.rs 现只保留此前已标注，不新增修改。
> country-core 自身 `cargo check` 零错误（workspace 级 rhapsody vulkan.rs:470 为 D组并发，
> 不归本组）。

## 真实代码接线

1. **Decorators 0x28B 组装 + 解码**（`systems/save/snapshot.rs`，① r6-decorators-serde §1/§4）：
   - `DecoratorSectionRecord(pub [u8; 0x28])` = `{u32 id@0（快照编码）, 8B×4 @4..0x14,
     u32×2 @0x18..0x20}`（0x28B 记录形状，O）。
   - `encoded_id()`（u32@0）、`encoded_u64()`（ID 旋转 XOR：`u64::from(id).rotate_right(32)
     ^ mask`，掩码 0x1429cd080 待①=0，同 crate encode_snapshot_id 语义）、
     `side_from_state(b)`（`(b-2)<=2 ? b-1 : 0` 子侧状态公式，O）。
   - `parse_decorator_section(JSON)` → 按候选形状 `Vec<[u64;5]>` 反序列化 → 组装 0x28B
     记录（serde 架构待①失败返回 None）。apply Decorators 分支登记 0x28B 记录数；
     → restore_snapshot 逐槽接线待①（crate 16B≈0x28B 标注，wall 禁不新增改）。
2. **AsyncSaveCmd 判别值纠错**（`systems/save/save_game.rs`，① r6-async-savecmd §1）：
   - `#[repr(u64)]` + 显式判别值 3..11 对齐原生（ALGORITHMS §2.1 save_thread 0x141720680 /
     分发 0x141721aa0 权威）：3 OpenFile / 4 CloseFile / 5 ChangeSection / 6 ReplaceEdits /
     7 BackupSave / 8 ManualBackup / 9 CaptureSnapshot / 10 Flush / 0xb(11) Shutdown。
   - 字段名保持语义字段 + U 标注（save-DELIVERY 已纠错）；载荷字段名待①。

## candidate 链测试

- 本组文件在 lib 类型检查零错误；country-core 自身零错误。
- AsyncSaveCmd 显式判别值 = 原生 3..11（repr u64 编译验证）。

## 原侧验证

- ① r6-decorators-serde：0x28B 逐槽形状 + ID 旋转 XOR 0x1429cd080 + 子侧公式
  （decorator_storage.rs.md 章节 take/restore 全解 + decomp-0x140a20d70）。
- ① r6-async-savecmd：ALGORITHMS §2.1（save_thread 0x141720680 变体表、分发 0x141721aa0）。

## 仍缺项（待①，登记不造）

1. Decorators JSON serde 逐字段名 + 0x16B 子记录逐槽 + restore_snapshot 逐槽接线。
2. AsyncSaveCmd 载荷字段名（OpenFile 参数、CaptureSnapshot Arc 载荷）。
3. GameSaveData 240B / session create 96B+32B+24B 字段名。
4. sheep movement 状态机 + X4 活羊 Query、Terrain strokes、ui_clutter 0x58B、
   ivy count_range RNG 界、mod_key 表2 键码、camera_rig/weather/time_of_day 深层、
   28-系统逐名（r6-batch2-status 交叉引用）。

## 组外申请记录

- 无本轮组外修改（save_game/snapshot 为②文件权）。未碰 rhapsody/roof/wall 禁区；
  decorator_storage.rs 仅保留此前已落标注，无新增修改。
