# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

# 1) history_process: 4 rows "316B 跳板" wrong + jump table base correction
p1 = os.path.join(CH, "systems/history/history_process.rs.md")
txt = io.open(p1, encoding="utf-8").read()
old = "按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板）"
new = "按 HistoryEdit.tag 经跳表分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同；round3 round2 修正：实函数约 10.4KB 非 316B 跳板——316B 为 Ghidra 跳表恢复失败截断；各单态跳表独立，0x142c5f248 仅属 0x14143d2c0 单态，water 单态 0x141444c70 跳表基址 0x142c5f2fc）"
n = txt.count(old)
txt = txt.replace(old, new)
io.open(p1, "w", encoding="utf-8", newline="").write(txt)
print("history_process 4 rows fixed:", n)

# 2) replay_edit row: evid + 作用
old2 = "| `country_core::systems::history::history_process::replay_edit` | 0x140af6170 | 258 | 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log | O | decomp-0x140af5b30.txt；DIGEST §3.3 |"
new2 = "| `country_core::systems::history::history_process::replay_edit` | 0x140af6170 | 258 | undo/redo 游标+replay_edit+桶 push+第二事件流+log（round3 round2 修正：原作用列为 history_process 描述复制） | O | decomp-0x140af6170.txt（round3 round2 修正：原错引相邻函数 decomp-0x140af5b30.txt）；DIGEST §3.3 |"
if old2 in txt:
    txt = txt.replace(old2, new2, 1)
    print("replay_edit row fixed")
else:
    print("replay_edit NOT FOUND")
io.open(p1, "w", encoding="utf-8", newline="").write(txt)

# 3) replay_path: row + header
p3 = os.path.join(CH, "systems/history/replay_path.rs.md")
txt = io.open(p3, encoding="utf-8").read()
old3 = "| `country_core::systems::history::replay_path::generate_path_replay_cmds` | 0x1416286d0 | 44 | Path/Cobblestone 编辑 → ReplayCmd：niche 判别 0..3 过滤后尾调 0x1414433xx 共享核心 | O | history\\decomp\\decomp-0x1416286d0.txt |"
new3 = "| `country_core::systems::history::replay_path::generate_path_replay_cmds` | 0x1416286d0 | 44 | Path/Cobblestone 编辑 → ReplayCmd：niche 判别 0..3 过滤后尾调共享核心 | O（round3 round2 修正：尾调目标=0x1414423e0（generate_replay_cmds_from_edit<path closure_env$0>），非 0x1414433xx；e9 rel32 化验算） | history\\decomp\\decomp-0x1416286d0.txt |"
if old3 in txt:
    txt = txt.replace(old3, new3, 1)
    print("replay_path row fixed")
else:
    print("replay_path row NOT FOUND")
old3h = "> 文件作用：路径/鹅卵石光栅回放：replay_raster 的 4 个领域实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 判别 0..3 过滤后尾调共享核心 0x1414433xx）。"
new3h = "> 文件作用：路径/鹅卵石光栅回放：replay_raster 的 4 个领域实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 判别 0..3 过滤后尾调共享核心 0x1414423e0；round3 round2 修正原 0x1414433xx）。"
if old3h in txt:
    txt = txt.replace(old3h, new3h, 1)
    print("replay_path header fixed")
else:
    print("replay_path header NOT FOUND")
io.open(p3, "w", encoding="utf-8", newline="").write(txt)

# 4) flush_deferred_and_sync: string attribution
p4 = os.path.join(CH, "systems/save/save_thread.rs.md")
txt = io.open(p4, encoding="utf-8").read()
old4 = "| ``country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync`` | 0x14171f7f0 | 174 | 待推断 | O：flush_deferred_and_sync（游戏线程断开时落盘；错误日志 Error flushing save file on game thread disconnect.） | session-save\\decomp\\decomp-0x14171f7f0.txt  |"
new4 = "| ``country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync`` | 0x14171f7f0 | 174 | 待推断 | O（round3 round2 修正串归属）：flush_deferred_and_sync（TLS 守卫+断言+vtable 调用，断开时落盘结构吻合）；\u201cError flushing save file on game thread disconnect.\u201d（0x142ccaa28）唯一 xref 在 save_thread 主循环 0x141720680（0x1417212c0 处），不在本函数 | session-save\\decomp\\decomp-0x14171f7f0.txt |"
if old4 in txt:
    txt = txt.replace(old4, new4, 1)
    print("flush_deferred_and_sync row fixed")
else:
    print("flush_deferred row NOT FOUND")
io.open(p4, "w", encoding="utf-8", newline="").write(txt)
