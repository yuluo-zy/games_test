# RE-ANALYSIS — resources/session + meta_commander（会话切换链）

分析师: re-analyst-state
日期: 2026-10-15
范围: session.rs 未挖透重点（NextSession/on_load/append_on_load）+ meta_commander.rs 前置证据
（MetaCommand 结构 / commander_update 分派 / MetaCommandFence）+ load_game 的 on_load 回放消费端。
本批聚焦 DISPATCH 跨组裁决的「会话切换链」：**MetaCommand → on_load → load_game 回放 → fence**。
证据 = fresh 反编译（shared 只读，落盘 `evidence/`）+ RNE 原字节。

---

## 0. 会话切换链总览（新证据整合）

```
[生产者]                        [排队]                 [消费/回放]
run_game_thread 0x1405a3440 ──→ MetaCommander(VecDeque<MetaCommand 0x38B>)
  ├─ append_on_load<[MetaCommand;3]>  0x1403b5150    每帧 commander_update 0x14091f180
  │    → on_load 条目 {3, Box<[MetaCommand;3]>}          case 0(会话加载)：
  └─ append_on_load<MetaCommandFence> 0x141328ac0         新 fence(FUN_14091ef90)
       → on_load 条目 {0x8000000000000000, fence_arc}     → 建临时 on_load Vec + fence 条目
                                                        → FUN_1412e0b90 应用(置 NextSession)
                                                        → 重排队命令(带新 fence)
                                     load_game 0x14146d030 回放 on_load：
                                       Pass1 MetaCommand：变体0=manager状态+24B事件 /
                                         变体1=push MetaCommand / 变体2=unblock+drop Arc
                                       Pass2 PostGameLoadCommand：变体0=忽略 SwitchAppMode /
                                         变体1=push 24B命令 / 变体2+=push{tag 0}
                                       fence 0x8000000000000003 终止两遍
```

**关键新证**：on_load 列表元素 = **0x20B（非 0x38B）**，load_game 遍历步长 `ptr + len*0x20`
（decomp-0x14146d030 行 368/734：`ptr + len*4` 以 8B 指针单位 = `len*0x20`）。这与
DIGEST「on_load 4B 条目」措辞不同——是 32B 条目、tag 为首 8B。

---

## 1. MetaCommand 结构（0x38B = 56B，O 机器码）

证据：commander_update 0x14091f180（decomp-0x14091f180.txt 行 267-275 读取 7 qword）、
consume 0x14091f100（fresh，consume 的 pop_front 读 7 qword）。

| 偏移 | 大小 | 语义 | 证据 |
|---|---|---|---|
| +0x00 | u64 | **tag / 判别值**（niche 基址 0x8000000000000000；变体 i = 0x8000..+i） | consume/update 首读 |
| +0x08 | u64 | 载荷 qword 1 | |
| +0x10 | u64 | 载荷 qword 2 | |
| +0x18 | u64 | 载荷 qword 3 | |
| +0x20 | code* | **函数指针字段**（部分变体用；case 1 重排布时以指针载荷回写） | decomp 行 272 |
| +0x28 | u64 | 载荷 qword 5 | |
| +0x30 | u64 | 载荷 qword 6 | |

- **空哨兵 = 0x8000000000000014**（= 变体 0x14=20）：`consume` 空队列返回该值
  （decomp-0x14091f100 尾行 `*in_RCX = 0x8000000000000014`）；commander_update 读 tag == 该值 → return。
  旧章节「空哨兵 0x8000000000000000」**修正**：0x8000000000000000 是变体 0 的判别（会话加载），
  空是 0x8000000000000014。
- **consume pop_front**（fresh 0x14091f100）：VecDeque 环 {cap@+0, ptr@+8, head@+0x10, len@+0x18}；
  `head = (head - (cap<=head+1 ? cap : 0)) + 1` 环绕；元素步长 0x38；7 qword 拷贝至输出。
- **peek**（fresh 0x14091f0c0）：`[rcx+0x18](len) != 0` 判空；39B 短函数（front 值内联）。

### 1.1 commander_update 0x14091f180 分派（switch `tag ^ 0x8000000000000000`，case 0..0x13）

| case | tag(基址+case) | 行为（O） | 消费者/证据 |
|---|---|---|---|
| 0 | 0x8000000000000000 | **会话加载**：AppMode 门控（param_2 值检查）→ 新 fence `FUN_14091ef90` → 建临时空 Vec{0,8,0} → `FUN_141328ac0`(append_on_load<Fence>) → `FUN_1412e0b90`(应用，置 NextSession/PostGameLoadCommand) → 把命令重新入队（带新 fence Arc 载荷）；+0x18 载荷为 Arc 时按 `[Arc+2]` 标志决定 unblock/重排/drop | decomp 行 301-421 |
| 1 | 0x8000000000000001 | 命令重排（载荷 {+0x20 fn ptr, +0x28, +0x30} 重写入队） | 行 422-446 |
| 2 | 0x8000000000000002 | **profiler_sinks HashMap 迭代**（hashbrown control-byte 位扫描）+ 重排 {tag 3, countdown 0x12c} | 行 447-513 |
| 3 | 0x8000000000000003 | HashMap 迭代（直到 control==0x01 变体）；载荷非 0 → 重排 {tag 3, count-1}；log @0x142aade70 | 行 557-686 |
| 4 | 0x8000000000000004 | 状态清零（param_10/11 多槽置 0，尾 1） | 行 687-702 |
| 5 | 0x8000000000000005 | 载荷非 0 → 重排 {tag 5, count-1}（等待/延后） | 行 703-720 |
| 6 | 0x8000000000000006 | **float→十进制格式化**（指数位处理 + 1e9 乘）→ push 32B 事件 {FUN_1412d99c0, 2, mant, exp}；负值/越界 → panic @0x142aad5e0/61b | 行 721-790 |
| 7 | 0x8000000000000007 | 清零 + `FUN_1412e1400(param_13, Vec, 0x142aadea8)`（TimeUpdateStrategy 应用） | 行 791-822 |
| 8 | 0x8000000000000008 | **追加 0x30B 条目到 lStack_338 Vec**（{+0x08,+0x10,+0x18(fn),+0x20,+0x28,+0x30}；cap-first cap@+0x20/ptr@+0x28/len@+0x30/cnt@+0x40） | 行 823-846 |
| 9 | 0x8000000000000009 | **建目录**：FUN_140d76ed0 → FUN_140d61cd0 → FUN_140d5a150；空 → panic @0x142aadec0；失败 log @0x142aadef0 | 行 847-921 |
| 10 | 0x800000000000000a | 置 [puStack_258+0x31] = 1 | 行 922-929 |
| 0xb | 0x800000000000000b | 置 [puStack_258+0x31] = 0（与 case 10 配对开关） | 行 930-937 |
| 0xc | 0x800000000000000c | **GPU/内存报告**：alloc 0x68B 结构 → Arc(LOCK inc) → `FUN_140c17740`(hashbrown insert，键=ppppppplVar40+1) → `FUN_141528e40` 回写；verbosity>2 log | 行 938-1161 |
| default | ≥0xd | 未知命令 → verbosity>2 log（@0x142aadfc8）+ 断点 | 行 1162+ |

> case 0xd..0x13 未逐 case 展开（进入 default log+break 的合并路径）；旧章节「0xd 合并/共享」维持。
> case 2/3 的 hashbrown 位扫描 = profiler_sinks HashMap 迭代（puffin profiling 数据发送），
> 与「复队/TimeUpdateStrategy/InputRecordReplayState/手动时间/设备列表」命名待逐 case 对上——已按机器码登记行为。

### 1.2 MetaCommandFence（O，机器码）

- **new_pair_blocked 0x14091ef90**（RNE 原字节 0x14091ef90）：
  alloc 0x18 → `[+0x00]=1`(strong) `[+0x08]=1`(weak) `[+0x10]=1`(标志字节) → `LOCK inc [+0]`(Arc 克隆)
  → strong=2 返回。**unblock 偏移已裁决 = +0x10**（reanalyst 卡2：commander_update 0x14091f450
  原字节 `movzx eax,[rax+0x10]` 与 new_pair_blocked 机器码 `c6 40 10 01` 一致；旧"+2"为 Ghidra
  指针算术伪影。详见 RE-SESSION-SWITCH.md §1/§2）。
- **核心新证**：FUN_1412e0b90 = **Commands::insert_resource<NextSession>**（fresh 反编译）
  ——会话切换经 NextSession 资源接力（commander case0 发起 → load_game 回填）。

---

## 2. NextSession.on_load 列表（0x20B 条目，O）

### 2.1 append_on_load<MetaCommandFence> 0x141328ac0（fresh + RNE 原字节）

- 推送元素 **0x20B**：`{u64 0x8000000000000000, u64 param_3(fence)}` + 0x10B 残留
  （机器码：`mov [rbp-0x30]=0x8000000000000000; mov [rbp-0x28]=r8; movups xmm0→[base]; 
  movups xmm1(garbage)→[base+0x10]`）。
- 旧章节「{u32 tag=0, u32 0x80000000, u64 payload}」**修正**：实为单个 u64 `0x8000000000000000`
  （little-endian 下 Ghidra 拆成 {0, 0x80000000} 两 u32）。
- Vec 增长：`len==cap` → `FUN_140562c30`（0x20B 元素扩容族）。
- 返回：`memcpy(self, vec, 0xa0)` builder 形态（0xa0B NextSession 拷贝返回）。

### 2.2 append_on_load<[MetaCommand;3]> 0x1403b5150（fresh + RNE 原字节）

- `alloc 0xa8(=3×0x38)` → `memcpy(_Dst, r8, 0xa8)`（Box 化 3 条 MetaCommand 数组）→
  push 0x20B 条目 `{u64 3, Box<[MetaCommand;3]>}`。
- 调用方（xref）：**run_game_thread 0x1405a3440**（`call 0x1403b5150` at 0x1405a3440）。
  run_game_thread 在建新会话时：`lea rcx,[rbx+0x2590]`(NextSession) / `lea rdx,[rbx+0x440]`(&on_load Vec)
  / `lea r8,[rbx+0xb00]`(&[MetaCommand;3])。数组内容在 rbx+0xb00..0xba8 逐条构造；
  已见 cmd2 tag = 0x8000000000000008（= 变体 8）、cmd2[+0x28]=rdi；cmd0/cmd1 在更早字节区。
  → **3 条命令的完整载荷需对 run_game_thread 0x1405a33xx 前段做定点原字节扫描（残留，见 §5）**。

### 2.3 load_game on_load 回放（0x14146d030，O 全解）

元素步长 0x20，`mem::take` 清空 {cap=0,ptr=8,len=0} 后遍历：

**Pass 1（MetaCommand 回放，行 372-429）**：
| 变体 | tag | 行为 |
|---|---|---|
| 0 | 0x8000000000000000 | 写 LoadGameManager 状态 `*(int*)(manager+x) = ...` + push 24B 事件 {elem[+8], elem[+0x10], +0x40}（grow FUN_140562db0） |
| 1 | 0x8000000000000001 | `FUN_140b5a180` = **MetaCommander::extend_script（Vec::extend<spec_extend>）** push MetaCommand {tag, elem[+8], elem[+0x10]}（src 路径串 0x142c5ae38） |
| 2 | 0x8000000000000002 | LOCK `[elem[+8]+2]=0`(unblock) + LOCK `[elem[+8]]--`(Arc strong--) + 归零 drop |
| — | 0x8000000000000003 | **fence 终止 break** |

**Pass 2（PostGameLoadCommand 回放，行 733-811）**：
| 变体 | tag | 行为 |
|---|---|---|
| 0 | 0x8000000000000000 | log「Ignoring PostGameLoadCommand::SwitchAppMode」（@0x142c642e0）跳过 |
| 1 | 0x8000000000000001 | push 24B PostGameLoadCommand {tag, elem[+8], elem[+0x10]}（grow FUN_140562c30 @0x142c64308） |
| 2+ | 0x8000000000000002 | push 24B {0x8000000000000000, elem[+8], ...} |
| — | 0x8000000000000003 | fence 终止 break |

收尾：`FUN_14087b2f0`(drop on_load Vec) → `memcpy(&result, &PostGameLoadCommand_list, 0xa0)` →
`FUN_1412e0b90(param_8, &list, &UNK_142c64298)` 应用命令列表。

- **FUN_1412e0b90** = 「应用命令列表」辅助：commander_update case 0 也调用（param_13, &list, &UNK_142aade20）——
  即会话加载命令由 commander 侧触发、load_game 侧应用，同一函数族。

---

## 3. GameSaveData（共享类型，定点进度）

- **定义**：save_game.rs（skeleton 行 6162，`fields/size=unknown`，过程引用数 4）。
  引用：save_thread.rs 行 350（`drop_in_place<Box<GameSaveData>>` @ execute_save_cmd 0x141721aa0 域）、
  load_game.rs 行 187（`drop_in_place<Box<GameSaveData>>` @ 0x14146d030 域）。
- **save_game 组装**（0x140a3f470，O 部分）：栈结构 `uStack_2b8`（{ptr,len} @+0x60/+0x68 序列化缓冲）
  由 ~9+ 资源填充。**系统参数面（skeleton PDB tuple，O）**：
  `ResMut<History>, ResMut<AutoSaveState>, Res<InnerWalls>, Query<&Roof, Without<RoofMarkedForDeletion>>,
  Res<MainCamera>, Res<TimeOfDay>, Res<TerrainEditingData>, Res<TerrainHeightsData>,
  ResMut<ActiveSession>, Res<DecoratorStorage>, Res<WallColorIds>, Res<GateColorIds>,
  Res<HalftimberColorIds>, Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>,
  Res<GladeColorCustomization>, Res<DeletedAutoClutter>` + 局部面
  `AllWorldRasterRead, Res<GameTime>, Res<AnimateWallBulldozeBuffer>, Res<ReplayBuffer>,
  Res<StairsState>, Res<StairsRemovedSupports>`…
- **字段写区**（decomp-0x140a3f470 行 1705-1717，O）：`lVar38(AutoSaveState ResMut)` 密集字段
  `+0x18/+0x28/+0x30/+0x38`(各 16B)、`+0x40/0x45/0x47/0x48/0x50/0x54/0x55`、`+0x60/+0x68`({ptr,len})。
- **同步保存路径**（行 1728-1843，O）：write_to_disk(0x140b463c0, meta) → `FUN_1408d9a20` →
  `FUN_1408da280` 发 **AsyncSaveCmd::Flush(10)** → 同步响应分派（FUN_1416cc990/1416741f0/1416798b0）→
  `save_folder_path(0x140b46000)` + push 0x28B 会话条目到 Vec@+0x20。
- **结构级结论（可交 restorer）**：GameSaveData 语义 = **序列化完整存档所需的数据集合**
  （History 链 + 相机 + 时间 + 地形 + 羊/装饰 + 三色 ID + glade 颜色 + ActiveSession），
  以 Box 形式经 AsyncSaveCmd 通道传 save_thread / 由 load_game 重建。字段级完整映射
  需对 save_game 0x140a3f470 分段逐槽比对（残留，见 §5）——但 **type/schema 的消费边界已闭合**。

---

## 4. 会话切换生命周期（行为级步骤，供 restorer 落地）

1. `NextSession::create`(0x140b46960)/`load`(0x140b46920) 构造 {Vec(cap8), kind niche, uuid@+0x20,
   ignore_snapshot@+0x98}。
2. `run_game_thread`（新 glade 启动）用 `append_on_load<[MetaCommand;3]>` 把 3 条 MetaCommand
   （变体 8 等）压入 on_load；`append_on_load<MetaCommandFence>` 压入 fence 条目 {0x8000000000000000, fence}。
3. `MetaCommander`（VecDeque<0x38B>）持命令；`commander_update` 每帧 `consume` 一条 →
   case 0（会话加载）：新 fence → on_load 应用（FUN_1412e0b90）→ 命令带 fence 重入队。
4. `load_game`（AppMode==0xe 门控）状态机 → kind 解码 → 两遍 on_load 回放（§2.3）→
   fence 0x8000000000000003 终止 → PostGameLoadCommand 应用 → 版本开关/颜色升级/wall-id 恢复 →
   `init_game_state_from_snapshot`(0x1417a9c50)。
5. 失败分支：`Couldn't load save file from `、`Can't skip `、`Version mismatch`、mode 非法 panic。

**明确可解性结论**：MetaCommand / on_load / fence 链路 **已可解**（本报告给出结构+分派+回放三要素）；
GameSaveData 的**结构消费边界**可解，**逐字段映射**仍是残留（save_game 分段比对工作量）。

---

## 5. 未决疑问

1. GameSaveData 逐字段 layout（save_game 0x140a3f470 分段逐槽比对，最大残留）。
2. run_game_thread 3 条 MetaCommand 的完整载荷（rbx+0xb00 区逐 cmd 原字节扫描）。
3. ~~变体 2 unblock 标志偏移 +2 vs new_pair_blocked 的 +0x10~~ → **已裁决 = +0x10**（原字节
   0x14091f450，见 RE-SESSION-SWITCH.md §1/§2）。
4. commander_update case 0xd..0x13 各自语义（default log 合并路径，可后续逐 case 展开）。
5. append_on_load<MetaCommandFence> 元素 +0x10..+0x20 残留字节来源（alloc 未初始化栈）。
6. **fence 两条 tag 的关系**：append_on_load<MetaCommandFence> 写 {tag=0x8000000000000000,
   fence}（load_game 按变体 0 处理），而 load_game 两遍回放的终止哨兵是 0x8000000000000003——
   该 0x8000000000000003 哨兵由谁写入、与 append 的 fence 条目如何衔接，需对 load_game 回放前
   的 on_load 列表构造方（run_game_thread / commander_update case 0 后段）逐字节追源。
