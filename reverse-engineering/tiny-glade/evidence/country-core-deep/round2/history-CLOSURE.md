# history 域 gap 闭合（round2，2026-10-14，agent tg_a1）

对 gaps-v1.json history 域 6 个 digest_gaps + DIGEST U 项的逐项闭合结果。
方法：v2 索引（sysres-procedures-v2.json，已对齐 S_PUB32）+ MCP 定点反编译/
xrefs（dbid tg_a1，job 存 round2/hist-closure/）+ rne-read 原字节 + CU
函数边界表（.rdata 0x14369a000/0x143682f74/0x143697308 等，按编译单元列出
全部函数含本地符号）。

## 1. remove_snapshot_record / update_compatibility / TerrainEdit::duration 真实地址

### remove_snapshot_record —— 卡点：无独立符号，疑似内联

排除过程（CU 边界表 0x14369a000 给出 0x140a84xxx 簇的全部本地函数，逐个反编译/
机器码解）：
- 0x140a849c0（0x28B）：Option 门控 Arc drop（[rdx+0x50] refcount--，归零清
  [rdx+0x5f] 标志并调 0x1402e5d80 族）→ drop glue，排除。
- 0x140a849fd/0x140a84a00（MCP 反编译确认）：`if [rdx+0x5f]!=0 { lock dec
  [[rdx+0x50]]; if 0 → drop([rdx+0x38]) }` → drop glue，排除。
- 0x140a84a3c/0x140a84a40 = iter_resolved_snapshots（mov rax,[rcx+0x20];
  mov rdx,[rcx+0x28]; shl rdx,4）排除。
- 0x140a84dea/0x140a84df0（机器码解）：0x28 步长元素 drop 循环（ptr@+0x68、
  count@+0x70）→ HistoryEditChain drop glue，排除。
- 0x140a84e60（机器码解）：History 类初始化/clone（{cap=0,ptr=8(dangling),
  len=0, +0x20=8,+0x28=0, +0x30=i64::MIN 哨兵, 3×16B 拷贝, +0x68=-1
  (chain_index), +0x70=0(tail)}）→ init/clone glue，排除。

结论：PDB 元数据中的 remove_snapshot_record 名在 mod.rs CU 内无对应独立代码
（0x140a84220..0x140a854a0 全部函数已accounted），LLVM 全内联。sysres-v1 的
地址（0x140a84ba0=debug_display）确认错位。**卡点**：除非启用其他版本/调试
构建，无法恢复独立签名；重建侧保留占位即可。

### update_compatibility —— 卡点：无独立符号，疑似内联

排除过程（候选区间 0x140a82940-0x140a82b20，机器码解）：
- 0x140a829b6（0xF9B）：3+1 组 Vec<u32 对齐 4> 的 dealloc 循环（align=4 的
  free 族）→ drop glue，排除。
- 0x140a82abf/0x140a82abc（0x45B）：niche 判别（test [rcx],1）分派，两支分别
  以 r8d=4 + 两个静态串（0x142b1f8e3/0x142b1f8f7 区）调 0x140e3f2xx 族 → 
  fmt/辅助胶水，排除。
- 0x140a82aaf（0xEB）：Arc drop 微助手，排除。

结论：与 remove_snapshot_record 同类——名在 PDB 元数据、码已内联。sysres-v1
地址 0x140a82eb0 = WallHistoryEdit Debug::fmt（S_PUB32）确认错位。**卡点**：
WallHistoryEdit 的"兼容性"更新语义转 wall 域（deferred_wall_history 的
CompoundEditBuilder::wall_deleted 0x140a876a0 是仅存的相邻公开入口）。

### TerrainEdit::duration —— 闭合：无独立符号，公式 O 于 forward 内联

- 排除：0x140858d30=revert（sysres-v1 名错）、0x1408592d0=reapply、
  0x1408596d0/0x140859d90（机器码：drop glue）、0x140859740（MCP 反编译
  19KB：reapply 闭包宿主——hashbrown 48B 元素 stroke 表查 id，命中后拷贝
  12B 点集，FUN_141469230 fold，除以静态 0x142925984 求进度，与全局时间
  FUN_140c95ef0(&0x142a94d48) 做 (1-t)*u+t lerp，0..1 范围断言（复用
  "Tried to init CurveU from an invalid u: " / "assertion failed" 族），
  回写 stroke 记录 Vec）。
- O 位置：`TerrainReplayCmd::forward` 0x140994f30 内联
  `duration = clamp(Σ点距*0.01, 0.4, 3.0)`；无点默认 0.3；
  "assertion failed: stroke_length.is_finite()"（terrain_edit.rs:43）。
- 结论：duration 是内联/const 方法（无独立符号）；reconstruction 的
  `TerrainEdit::duration()` 公式实现正确，仅注释地址需改为 0x140994f30。

## 2. closest_history_snapshot 独立符号是否存在 —— 闭合：不存在

- S_PUB32（101,405 条）与 v2 索引均无 closest_history_snapshot。
- sysres-v1 的 "closest_history_snapshot@0x140b36460" = 名址双错
  （0x140b36460 = `<MultiEdit as Deserialize>::deserialize`）。
- 真实"最近快照"逻辑归属：`History::current_snapshot` 0x140a84650 /
  `History::closest_resolved_snapshot` 0x140a84a50（history 域）+ 
  `save::snapshot::validate_and_load_closest_snapshot_data` 0x1417a6e50
  （session-save 域，转交该 owner）。

## 3. 快照负载 T 的完整类型 —— 部分闭合 + 纠正 DIGEST

- **纠正**：DIGEST 曾推断"快照负载含 sun 环境字段（time_of_day/
  sun_xz_rotation/latitude_degrees）"——**错**。这三串属于邻接模块
  `resources::time_of_day` 的 serde（TodLightingSnapshot::serialize
  0x14007ff80、TimeOfDay::serialize 0x1400802c0、deserialize visitor 族
  0x140b0cc40/0x140b3bc00/0x140b3bd80 区），Ghidra 因函数区重叠
  （0x140b36460 的 decomp 越过 0x140b3bc83 边界，文件头有 "Instruction at
  (ram,0x000140b3bc83) overlaps" 警告）把 time_of_day 的 serde 并入了
  history serde 反编译。
- 真实属于 history 信封的字段串：{chain_index, history_start, edit_chain,
  snapshots, snapshot_id}（0x142aee01f 区 blob；snapshot_id 0x142aee0ac）。
- ArcInner<T> drop 符号检索：history 域内仅有
  `ArcInner<TerrainEdit>`（0x140878100），**无快照负载类型的 ArcInner drop**
  → T 定义在 history 域外（session/save 域）。
- 部分闭合：HistoryChainSnapshots 的 deserialize = 0x140b363e0、
  drop_in_place = 0x140600290/0x14088ff60（两个实例化）、
  HistoryChainSnapshot 的 InPlaceDrop = 0x141737070、HistorySnapshotId::
  serialize = 0x140b39a20、HashSet<HistorySnapshotId> drop = 0x140149f50。
- **卡点转 session-save**：T 的具体类型与 capture_game_snapshot
  （0x1417a1d00）/ capture_game_snapshot_save_data（0x1417a0430）/
  init_game_state_from_snapshot（0x1417a9c50）/ SnapshotLoadResources
  （0x1417ab290）绑定，属 save::snapshot 域。

## 4. 跳表 0x142ae5420 / 0x142ae545c 逐 case —— 闭合（rne-read 解码 40 项）

两表均为**编译器生成的静态只读跳表**（.rdata；无运行时写入者；xrefs 为空因
RIP-relative `lea` 引用不被 Ghidra 索引；引用点在 history_process 0x140af5b30
内，反编译可见）。

**0x142ae5420（28 项，第二事件流 16B 记录 byte[+9] 分派）**：
| case | 目标 | 语义（对 decomp 标签） |
|---|---|---|
| 0x0 | 0x140af5d12 | 检查末命令标志字节==1 门控 |
| 0x1 | 0x140af5d54 | 检查末命令标志字节==0 门控 |
| 0x2 | 0x140af5e00 | 消费/跳过 |
| 0x3 | 0x140af5dec | ReplayBuffer::make_animations_instant（0x1409f1ab0） |
| 0x4 | 0x140af64cf | 校验分支（byte==4/8 检查） |
| 0x5-0x7 | 0x140af6664/8/c | 校验/错误路径 |
| 0x8-0xe | 0x140af651e/0x140af650b/0x140af6505/0x140af659d/0x140af649a/0x140af6684/0x140af652c | 各记录类型校验/合并分支 |
| 0xf-0x18 | 0x140af7d88/0x140af7e13/0x140af7dc6/0x140af7de5/0x140af7d00/0x140af7e3e/0x140af7e5d/0x140af7e10/0x140af7e9b/0x140af7dc7 | 命令移除/清理簇（0x30 步长记录 pop、memmove、分派） |
| 0x19-0x1b | 0x142ae5484/8/c | **表尾数据（无效目标）**——判别 0x19+ 不合法 |

**0x142ae545c（12 项，ReplayCmd 判别字节 0-9 的 drop/清理分派表）**：
case 0x0-0x9 → 0x140af7d88/0x140af7e13/0x140af7dc6/0x140af7de5/0x140af7d00/
0x140af7e3e/0x140af7e5d/0x140af7e10/0x140af7e9b/0x140af7dc7（与上表 0xf-0x18
同簇 = 各判别对应的记录 drop/清理 handler）；case 0xa/0xb → 0x142ae5484/8
（表尾数据 = 无效）。反编译中 `if (10 < tag) goto 0x140af6660`（OOM/容量
错误路径）与之一致：**ReplayCmd 有效判别 0..=10**（10=Stairs 由另一路径处理，
不在此表）。此表即 `drop_in_place<ReplayCmd>` 的 match 分派（Arc 变体 LOCK dec
+ 0x1402e5d80 族；Wall 内联变体逐字段 drop），语义与 replay_cmd_buffer.rs 的
ReplayCmd 枚举判别一致。

## 5. add_decorator_diff 的 remap 表（param_4）—— 部分闭合

- 结构 O：hashbrown RawTable（+0x00 ctrl、+0x08 bucket_mask、+0x10
  growth_left、+0x18 items；+0x20 起 entries）；键 = 16B u32 对 {min,max}
  （边对归一化）；查表函数 FUN_1405b1be0（loop A）/FUN_140b76ce0（loop B，
  返回 enum）。
- 语义 I：楼梯边对 → 新增/移除 的映射/去重表（loop A 命中 → push tag 0xE
  "移除边"；loop B 命中且==1 → push tag 1 "新增边"）。
- **卡点**：表内容的构造方在楼梯域（stairs 的边注册/图），history 域无
  写入证据；转 wall/stairs owner。

## 6. History 完整 serde 手写 impl 位置 —— 闭合

- **序列化侧（写入 save 文件）**：`save_game::add_history_edit_to_save_file`
  0x140a439b0（MCP 反编译全文已读）：`(buffer, edit: &MultiEdit, ids: &mut
  Vec<u64>)` —— 把 [History+0x38]（快照 id 域）推入 id Vec，向 save 缓冲
  追加 ",\n"（u16 0x0a2c），然后调 **MultiEdit::serialize 0x140b39b80**
  （h63c9 单态，直接以 param_2=&MultiEdit 调用，实锤）写 JSON。信封
  {edit_chain, chain_index, snapshots, history_start, snapshot_id} 的组装在
  save_game 调用循环内联（session-save 域）。
- **反序列化侧**：合并区 0x140b3ad60-0x140b3bc83：EditType::deserialize
  0x140b3ad60、MultiEdit::deserialize 0x140b36460、visit_seq 0x140b365e0、
  visit_map 0x140b36840；信封字段串（chain_index/history_start/edit_chain/
  snapshots/snapshot_id，0x142aee01f blob）在 visit_seq 区使用
  （FUN_14158ac70/FUN_141583c10 调用点，反编译 621/625 行）。
- **serialize 侧地址清单（v2，全部 S_PUB32 对齐）**：EditType 0x140b39cd0/
  0x140316660；MultiEdit 0x140b39b80/0x140317160；ResolvedHistoryChainSnapshot
  0x140b397c0/0x140b398c0/0x140316500；HistorySnapshotId 0x140b39a20；
  HistoryChainIndex 0x1408aa9f0；HistoryChainSnapshots::deserialize
  0x140b363e0。
- 结论：**无独立 `<History as Serialize/Deserialize>` S_PUB32 符号**（信封
  serde 内联于 save 系统 + 合并区），与 PDB 元数据无此条目一致；DIGEST 的
  "疑似内联" 定论。

## 附带纠正（对 DIGEST / 重建的更新）

- 0x1409dd790 = `history_finish_recording`（v2 + S_PUB32 双证），DIGEST 首轮
  沿 sysres-v1 标为 history_start_recording 是错的；history_start_recording
  无独立符号。
- 0x1408c5680 = ICF 多符号同址：is_default_chimney_smoke + 
  is_default_allow_connection（均 decorator_edit）+ ~10 个跨域谓词
  （is_controller/is_length/toggle/is_default_arch_height/…）；reconstruction
  引用该地址有效，但语义裁决需看具体谓词体。
- 新增 v2 符号：tree_edit::missing_prefab 0x1409e6210、
  decorator_edit::send_state_diff_events 0x1417bdae0（reconstruction 的
  0x14017bdae0 丢高位 1）、EditType::Garden 构造 0x1416960d0、
  EditType::Water 0x1416bb7f0、roof_edit Deserialize 0x140b38b80。
- CU 函数边界表（.rdata）：0x14369a000（mod.rs CU）、0x143682f74
  （terrain_edit CU）、0x143697308（save_game CU）——可作本地函数枚举的
  通用手段（跨域可复用）。

## 未闭合卡点清单（转交）

| 卡点 | 已排除 | 转交域 |
|---|---|---|
| remove_snapshot_record 独立签名 | 0x140a849b6-0x140a84e60 全部本地函数（drop/init glue） | 无（内联，占位保留） |
| update_compatibility 独立签名 | 0x140a82940-0x140a82b20 全部（drop/fmt glue） | wall |
| add_decorator_diff remap 表内容构造方 | history 域无写入证据 | stairs/wall |
| 快照负载 T 具体类型 | time_of_day 串排除（邻接模块）；ArcInner 无 history 内类型 | session-save |
| 跳表 case 0xf-0x18 簇内部逐 handler 的字段级语义 | 表解码完成（handler 地址已知） | 无（属 history_process 内部 drop glue，低价值） |
