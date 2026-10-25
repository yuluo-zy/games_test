# REVIEW — rebuild-save + rebuild-roofsstairs 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 rebuild-save（round21）+ rebuild-roofsstairs（round22）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（10 警告，含他组在途）；
> 全 lib test 目标仍被并行组 in-flight 打断（非 state 文件）。本模块证据为
> `planning/logic-save/decomp/*` + `planning/logic-roofsstairs/*` fresh 反编译/harness。

## Round 21（rebuild-save）

### 分类
- 19 需重做 = **17 真业务 + 2 错锚** 确认（`AutosaveErrorState::is_ok` VA 0x140b92210 实为
  `ShapeStream::is_empty`；`last_error_state` 为 +0x10 指针访问器判等）。无胶水混入。

### 关键：commander_update case0 门控极性 —— **round5 判据被 fresh 反编译推翻**
- 我 round5（REVIEW5）依据 RE-SESSION-SWITCH §1 核准 `!matches!(13|14)→return`（"在范围→主路径"）。
  **fresh decomp 0x14091f180 case0 显示分支标签相反**：`if (mode-6<0xf && mode-0xd<2) { push_front 重入队 }
  else { new_pair_blocked → append_on_load<Fence> → Commands::insert_resource<NextSession> → push_front }`。
- 即 **mode∈{13,14} → 原样重入队等待；mode∉ → 建会话**。语义合理（Replay/LoadGame 中不启动新会话切换）。
- src 已纠正：`matches!(13|14) → undo_consume(original)`；否则 `commands.insert_resource(session)`（
  匹配 FUN_1412e0b90 Commands 语义）+ fence strong=2 镜像。**verdict：OK（round5 判据作废，以 fresh 为准）**。

### load_game 两遍回放
| 项 | verdict | 说明 |
|---|---|---|
| 解码 clamp `>2→1`（非 min(2)） | OK | `decoded>2 → 1`（≥3 分派不同）。 |
| variant1 载 3 payload / variant2 内联 fence 释放（删 FenceRelease） | OK | OnLoadEntry::Command 3 payload；variant2 unblock +0x10 + Arc drop 内联。 |
| 次遍 SwitchAppMode 仅 log / Apply 载 payload_b/c 真值 | OK | 不再 push SwitchAppMode；Apply 带 b/c。 |

### save_thread / execute_save_cmd / AutoSaveState
| 项 | verdict | 说明 |
|---|---|---|
| Shutdown 无 ack：flush:true→flush+退出；flush:false→deferred+10ms+continue | OK | 删 shutdown_ack；flush_deferred_and_sync 删末尾 sync_all（原生只 apply）。 |
| case3/4 前档 flush + 失败 UnsavedFileList::add + drop | OK | execute_save_cmd 增 unsaved 参数。 |
| AutoSaveState close_file 先发 CloseFile(4)（失败 panic）+ 清状态 | OK | 0x140a43840 语义。 |
| create_backup_of 同步请求-应答（ManualBackup reply 通道）→ Result | OK | 替代旧 bool + 直连 fs。 |
| copy_dir_all 纯递归（删 sort/intro 跳过） | OK | 0x141740c00 无 sort/intro。 |
| is_session_ok_to_load 三值（unsaved→1/loading.tmp 缺→0/在→2） | OK | 旧折叠 Ok(0) 已纠正。 |
| save_game_system 主体（cooldown ChangeSection(5)+write_to_disk+Flush(0xa)） | OK | 节 cooldown 秒表值待取证（变更即发不伪造）；域采集面宿主接线。 |
| 域 Query 面登记（capture/validate/init/add_game_startup_systems/load_glade_audio/parse 全量节） | OK（诚实） | 6 项登记，不伪造。 |

## Round 22（rebuild-roofsstairs）

### 分类 / clear
- 16 真业务 + 1 render 胶水（MagicSupports=Arc/mesh render 资源非 HashSet）确认；
  **0x140869a30 更正 = StairsStructuralGraph::clear**（src clear 清 graph/双向映射/高/connections/edge_index；
  LOGIC §5 误判 MagicSupports 驳回）。

### roof_noise hash 链
- finalizer（`combine_partial(hash1(z), x) + seed`）→ 掩码 `0x9d2c5680`/`0xefc60000`/`0x7fffff`；
  SIMD（每 z 行两角：scalar 角 `hash2(x,z,SCALAR_CORNER_SEED)` vs simd 角 `hash2(x,z,CORNER_SEED)`）双线性。
  harness 9 点参考证据（noise_check.rs/disasm_noise.txt/bytes_noise.json）——**9 点全过**。verdict：**OK**。
  （相对 round8 版本为细化，round8 简版哈希被 fresh 反汇编精化。）

### roof_shape_at_intersection_y y==floor
- 拷 roof[8..0x24] 原始帧体（rect：frame[+0xc..0x18]+dimensions[+0x1c,+0x20]；circle：frame[+0xc,+0x10]+
  radius[+0x14]+word[+0x18]）——取代 round19 的 `section(b, 0.0)` 近似。verdict：**OK**。

### StairsRemovedSupports / replay_stairs / structural_graph
| 项 | verdict | 说明 |
|---|---|---|
| StairsRemovedSupports = `HashMap<StairGraphEdge,()>`（round12 曾见 u32,u32——机器码全边键集合） | OK（纠偏） | erase/contains 按 StairGraphEdge。 |
| replay_stairs 双路径 undo（instant 反）+ 下限 **0.05** | OK（纠偏） | `fVar44=duration/speed; if(fVar44<=0.05) fVar44=0.05`；round13 曾用 0.1。 |
| disconnect_node_by_collider 每邻边单端点入 connections | OK | 与 R7 核一致。 |
| 依赖链（get_height_01/clamped/apply/revert）自动传导；roof_uv 族登记 R1 | OK（登记） | |

## 结论与 DISPATCH 建议
- **round21+round22 需重做项 → 全部闭合**。关键修正：commander_update case0 门控极性（round5 判据
  作废，以 fresh decomp 为准）；StairsRemovedSupports 全边键；FLOOR 0.05；y==floor 原始帧体。
- 残留登记（不伪造）：save_game_system 域采集面（6 项）、parse 全量节/upgrade 备份名、节 cooldown 秒表值、
  roof_uv 族 R1、fence 生产者在 run_game_thread 接线。
- save/session/music/history/startup/roofs/stairs/meta_commander → **review_passed（限定已实现范围）**；
  完整 core 验收仍 blocked（外部依赖 + 全局调度 + 并行组 in-flight 收口后重验全 crate 绿窗口）。
