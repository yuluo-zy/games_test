# wave4 geometry（wall-constructor 剩余族 + consume-types）独立复核

reviewer-wave4-geometry · 2026-10-17。未改 src。
复核对象：
- `reports/remaining/wall-constructor/RESTORE.md`（find_stone_column_start_y、
  circle_categorize、10/11 处 `todo!` 降级）
- `reports/consume/types/RESTORE.md`（replay_tree、gpu_flush_buffers、
  maintain_dormer_tiles、process_tree_cmds Insert、ui_move_tree、纠错声明）

方法：Ghidra `--worker shared` **fresh** 只读反编译 + 反汇编 + 跳表/常量逐字节，
独立复述后与 Rust 逐语句对照；未复用 restorer 结论作为证据。
证据落盘：`work/reviewer-wave4-geometry/evidence/`（decomp_*.c、b2_*.txt、
tree_cmds_disasm.txt、ui_move_tree_disasm.txt），jobs 可重放。

fresh 函数：0x141250720、0x14124af60、0x141bc34f0、0x140de50a0、0x1407ce920、
0x142267430、0x14226b1a0、0x1405b0d70、0x140053320、0x141259000、0x140ad2950、
0x140c98980、0x140c98ab0、0x140c97b10、0x14226a170；跳表 0x142c20af8、
0x143035538；常量 0x1429ddd00/0x142925980/0x142925904/0x1429cd030/0x14295b630/
0x1429df16c/0x14295a0e4/0x142c20388/0x142a8d600/0x1429cd078/0x142925908/
0x1429258d8/0x14295a0dc/0x14295a0e0。

## 0. 三态结论

| # | 条目 | RESTORE 宣称 | 本复核 verdict |
|---|---|---|---|
| 1 | find_stone_column_start_y 0x141250720 | 重建 | **判等** |
| 2 | circle_categorize 0x14124af60 | 重建 | **需修**（NaN/∞ 语义 1 处） |
| 3 | `todo!` 降级（11 文件 / 17 个 todo!） | 10 文件、诚实 | **未实现（诚实）**；计数口径错（见 §4） |
| 4 | replay_tree 0x141bc34f0 | **判等** | **需修**（Move undo 全路径未实现；判等高估） |
| 5 | gpu_flush_buffers 0x140de50a0 | 判等 | **判等** |
| 6 | maintain_dormer_tiles 0x1407ce920 | **判等** | **需修**（id 写面 push vs 单槽 store） |
| 7 | process_tree_cmds Insert 0x142267430 | 重建 | **需修**（random_tree_prefab selector 可表达却传 0） |
| 8 | ui_move_tree 0x14226b1a0 | 重建 | **需修**（DEMO 门缺失；状态机附加 guard/clear/clamp） |
| 9 | 纠错声明 0x1405b0d70/0x140053320/0x141259000 | hash_one/remove_entry/drop | **成立** |

三态：**判等 2 / 需修 5 / 未实现（诚实）17 个 `todo!` 函数 + 1 个登记空壳（11 文件）**。

## 1. wall-constructor

### 1.1 find_stone_column_start_y（0x141250720）——判等

fresh 105 条指令（525B），跳表 17 项按 `MOVSXD` 相对基址 0x142c20af8 解码：

- idx 0/6..=0x0b/0x0e → 0x14125082e（继续取矩形轴）；idx 13 → 0x141250937
  （`ADDSS XMM0,[0x1429df16c=-0.2]`，即 `0x0d → y-0.2`）；idx 1 → 0x141250941
  （`XOR EAX,EAX`，None）；idx 3/4/5/12/15/16 → 0x14125094f panic 桩；
  0x12+ 越界（decompiler 标 halt_baddata）。Rust 分支集合**逐项相同**。
- 0x1412507bb `CMP byte[RSP+0x48],0x11; JNZ`：未命中 → `sample_at_world_xz(x,z)`
  （0x141246fb0）→ `xvy`（0x140c9cde0，结果写栈后弃用）→ 0x82e；Rust
  `let _ = xvy(...)` 等价（dead value）。
- 0x141250815 `MOVSS XMM0,[RSP+0xac]`＝hit.position.y（handle 24B@0、position@0x18）；
  0x14125092d `TEST byte[RSP+0x91],1`＝handle 第 2 字节 bit0＝`handle[0] u64 bit8`；
  Rust `hit.handle[0] & 0x100` 正确（`handle: [u64;3]`）。bit set → -0.2，否则 -1.0。
- 轴：`CMP byte[RDI],0`（Option tag）→ None `XORPS XMM0`+XMM3=1.0 → (1.0,0.0)；
  Some → XMM3=[RDI+4]=axis.x、XMM0=[RDI+8]=axis.y。Rust 相同。
- `Rectangle2d::new` 0x140c98980 fresh 反编译确认机器实参**确为**
  `(size.x=XMM1, size.y=XMM2, axis.x=XMM3, axis.y=stack1, center.x=stack2,
  center.y=stack3)`，struct 写 `axis@0/center@8/size@0x10`；调用点
  XMM1=XMM2=0.7、XMM3=axis.x、stack1=axis.y、stack2/3=(x,z)。RESTORE §5 的
  参数序登记**属实**，Rust 按 utils 现行签名构造出的 `{axis,center,size}` 正确。
- `MINSS` 链：`XMM6=s0; MINSS XMM6,[1000]; XMM7=s1; MINSS XMM7,XMM6; ...`
  = `minss(s3,minss(s2,minss(s1,minss(s0,1000))))`，与 Rust 完全一致；
  unordered 返回 src 的手写 `minss` 保留正确。
- 常量逐字节：0x1429ddd00=(-0.0,-1.0)、0x142925980=f32::MAX、
  0x14295b630=1000.0、0x1429df16c=-0.2、0x14295a0e4=-1.0、0x1429cd030=0.7。
- 返回判别 EAX=1/0 ↔ Option<f32>。

**裁决：判等。** 无 FIX。

### 1.2 circle_categorize（0x14124af60）——需修（1 FIX）

fresh 197 条指令（837B；**RESTORE 写「105 条」是错的**，105 是 find_stone 的条数）。

逐段核对：
- `UCOMISS 1.75,radius; JBE main`（unordered 也走 main）；小半径
  `CMP [R8+0x10],0; SETZ` → `walls.is_empty()`。Rust 相同（NaN 亦相同）。
- `(-1.75+radius)/1.25` → `MAXSS 0` → `MINSS 1.0`；`t²(3-2t)`；
  `s*0.6+(1-s)*0.3`（常量 0x142925908/0x1429258d8）。Rust 公式相同。
- `ceilf(2r)`（0x142923e60 为 ceilf thunk）→ 完整 u64 饱和 cast 序列
  （2^63/2^64 f32 常量 0x14295a0dc=0x5f000000、0x14295a0e0=0x5f7fffff；
  NaN/负→0、>MAX→u64::MAX）→ `step=2r/(n as f32)`。Rust `as usize` 相同。
- 双层 `0..=n` 闭区间：外/内 loop 的 `CMP+ADC` 自增与 `i<=n` 退出条件逐条对上；
  `x=center.x-r+i*step`、`y=center.y-r+j*step`。
- `is_point_inside`（0x140c97b10：`radius > sqrt(dx²+dy²)`，SETA）Rust
  `Circle2d::is_point_inside` 逐语句一致。命中 `inside+=1`。
- 墙循环：条目 stride 0x50、`MOV RDX,[R12+0x38]`；`PublicWalls::get`
  （0x140a91ef0）返回空 → `unwrap_failed`（Rust `.expect` panic 等价）；
  `get_wall_shape`（0x140b2a5d0）→ `is_pos_inside`（0x140ad2950）→
  `in_wall+=1; break`。Rust 相同。
- `is_pos_inside` fresh：`TEST byte[RCX],1; JNZ rect`，圆路径 `LEA RCX,[RCX+4]`
  → `Circle2d::contains_point`（0x140c980f0）；矩形读 axis@+4、center@+0xc、
  size@+0x14，四条件 `-local<=half && local<=half`。Rust `shape_contains` 的
  28B 字节解释**逐偏移正确**，调用 core 几何方法（未复制公式）。
- `DIVSS + UCOMISS + SETC`：返回 `ratio < threshold`，**unordered 也算 true**。

**FIX-G1（低危，NaN/∞ 退化输入）**：Rust `((radius-1.75)/1.25).max(0.0).min(1.0)`
与 `ratio < threshold` 走 Rust `f32::max/min` 和 `<`，NaN 语义与 `MAXSS/MINSS`
和 `SETC` 不同（NaN radius 机器得 t=NaN；`+∞` radius 机器 inside=0 → 0/0=NaN →
`UCOMISS` unordered → `SETC=1`；Rust 两种情况分别 t=0/比较 false）。正常有限输入
全部等价。按本仓硬标准（find_stone 已为 MINSS 单独保留语义），应改用
`maxss/minss` helper，并把末比较复刻为 `UCOMISS+SETC`（`ratio < threshold ||
unordered`）。

另外 `PublicWalls::get` 缺失用 `.expect("...")` 文案与原生 `unwrap` 不同，
非语义差异，不单列。

### 1.3 `todo!` 降级（诚实性）

逐个核对文件与 fresh 函数身份/大小（get_functions）：

| Rust 文件 | 函数 | VA | fresh size |
|---|---|---|---|
| circle_pillars.rs | construct_circle_pillars | 0x14124ba30 | 3228B |
| circle_turret.rs | construct_turret | 0x14124cc30 | 5523B |
| circle_bottom_layer.rs | add_bottom_brick / construct_circle_bottom_layer | 0x141fe1de0 / 0x14124b2c0 | 880B / 1620B |
| rectangle_bottom_layer.rs | construct_rect_bottom_layer | 0x14124eb80 | 3658B |
| rectangle_small_brackets.rs | construct_rectangle_small_brackets | 0x141250da0 | 7701B |
| rectangle_large_brackets.rs | construct_rectangle_large_brackets | 0x141253fc0 | 5200B |
| util_pillar_construction.rs | wooden / brick / crossbrick | 0x141fe2150 / 0x141fe3240 / 0x142418f50 | 3442 / 668 / 2592B |
| find_overlap_segments.rs | find_overhang_support_segments | 0x141248620 | 6703B |
| maintain_stone_column_colliders.rs | maintain_stone_column_colliders | 0x14078a6e0 | 2744B |
| rectangle_pillars.rs | construct_rectangle_pillars | 0x14124fcb0 | 详见契约 |
| mod.rs | circle_supports / rect_columns / rect_brackets / detect_overhang | 0x140788160/0x140788a80/0x1407896b0/0x1407867d0 | 2305/2674/2626/4258B |

- 11 个文件内所有目标函数均为 `todo!`，无近似体残留；argument 签名保留。
- `mod.rs::overhang_subscribe_to_tabloids`（0x140786440）仍是 `{}`，文件注释
  标「未恢复（壳登记）」，但 RESTORE §1 表未列它、且称「空壳/假循环已改为
  todo!」——**表格覆盖不全**（该空壳为诚实登记，但不在降级表内）。
- RESTORE §1 标题与 §4 计数写「10 个文件」，表实际 **11 行/11 文件**；
  17 个 `todo!`。计数口径错误（非实现造假）。
- 「近似体已移除」诚实：抽查文件全部只剩 todo!/类型/注释，无假循环。

**裁决：未实现（诚实）。** 附计数/覆盖纠正。

## 2. consume-types

### 2.1 replay_tree（0x141bc34f0）——需修（判等高估）

fresh decomp 逐分支：

- 入口 `front_mut_bucket()`；返回空则 return。命令记录 stride 0x30，`tag==8`
  才处理。`elapsed@+0x18 == 0.0 && !NAN` 门、`undo@+0x1c`、`edit=+0x10`。
- disc `*(int*)(edit+0x10)`：2=Delete、1=Move、其余=Place。
- Delete：`undo==0 → DeleteTreeCmd`（0x10 槽，payload=edit+0x18）；
  `undo!=0 → 0x48 InsertTreeCmd` 重建（edit+0x18/+0x20/+0x38/+0x40/+0x44/
  +0x4c/+0x50 + clone 名）。Rust 相同（Insert 载荷 T2）。
- Move：`undo==0 → MoveTreeCmd`（0x20 槽，+8=edit+0x30、+0x10/0x18=
  edit+0x20/+0x28 **新位置**）；`undo!=0 → MoveTreeCmd` with
  **edit+0x14/+0x1c 旧位置**。Rust **只实现非 undo**，undo 分支
  `if !cmd.undo` 直接**不发事件**——不是载荷缺字段，而是整条事件缺失。
- Place：`undo==0 → 0x48 InsertTreeCmd`；`undo!=0 → DeleteTreeCmd`。Rust 相同。
- 时序：`flags@+8 bit0==0 → removal`；否则 `delay=[+0xc]/speed`、`delay<=0.05
  →0.05`、`delay<elapsed → removal`、`elapsed += GameTime.delta(0x28)`。
  Rust 逐字相同（含 NaN 路径）。
- 移除：索引列表先**反转**，再逐 `Vec::remove`（assert OOB）+ 按 tag drop 载荷；
  Rust `removal.into_iter().rev()` + commands/timing 双删除 ✓。

**FIX-G2**：`replay_tree.rs:77-84` Move-undo 路径必须至少发射
`MoveTreeCmd{ position: 旧位置 }`；在共享 `TreeEditData` 落旧位前，RESTORE 不能
标「判等」——应降级为 **重建/未实现路径** 并在总账记该路径缺失。T2 的
Insert 载荷（height/variant/rotation/flags）是已登记缺口，可保留。

### 2.2 gpu_flush_buffers（0x140de50a0）——判等

fresh 0x140de50a0 是 `run_unsafe<FunctionSystem<fn(Res<GraphicsDeviceResource>,
ResMut<AssetSsboLibrary>)>>`，体内已被内联展开：资源存在性分支（缺失 →
`panic_fmt`，串 0x2c/0x3d B）→ `AssetSsboLibrary::get_mut<RoofTileSsbo>` →
None `unwrap_failed` → `GraphicsDeviceResource::deref`（0x1408xxx impl$0）→
`SparseInstanceBuffer::gpu_flush<RoofTileSsbo>` → `*(int*)(in_RCX+0x150)=tick`。
Rust 5 行（`get_mut::<RoofTileSsbo>().unwrap()` + `buffer.gpu_flush(device.device())`）
语义、参数顺序、panic 面一致；ResMut 变更 tick 由 Bevy 自动。无 FIX。

### 2.3 maintain_dormer_tiles（0x1407ce920）——需修

fresh 97 条指令逐条：

- `*cursor`（[RDX] 解引用）、`prev.saturating_sub(countA@+0x18)`、
  `saturating_sub(countB@+0x38)`；`lenA'=lenA(-startA)`（len<start→0，空哨兵
  8）、`lenB'` 同；游标**循环前**写 `total-lenB'-lenA'`；A[startA..]→B[startB..]
  chaining（stride 0x18）逐条对上。
- 每瓦：`INC [cursor]`；`MOV [R13],EBP`（**R13=[state+0x10] 只加载一次、
  永不递增**）；`hash_one<RandomState,&GlobalId>`（0x1405b0d70，输入=元素+8）；
  `RawTable::remove_entry<u64,V>`（0x140053320）；返回槽首 qword 非 0 →
  `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`（0x141259000）。
- **纠错声明成立**：fresh `get_functions` 签名确认 0x1405b0d70 =
  `hash_one<RandomState,ref<GlobalId>>`、0x140053320 =
  `RawTable::remove_entry<tuple<u64,Box<dyn FnOnce...>>>`、0x141259000 =
  `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`；wave3 的
  「虚构变换写 helper」判定确应撤销。PDB 的 `Box<dyn FnOnce>` 与 drop 符号
  冲突为 ICF 别名，登记合理。

**FIX-G3**：`maintain_dormer_tiles_draw.rs:133` 写成
`state.ids.push(state.value)`，原生是**同一地址的单次 u32 store**（`*slot = value`，
无容量检查/无 len 更新）。Rust 的 push 会追加 N 个元素并改 len，与机器不符。
应按 U 面落 `ids: *mut u32`/单槽写（或把该字段登记为 U 并在实现中只写一个槽）。
其余（cursor/链遍历/hash/remove/drop）判等。
附带：`DormerTiles` 在 core 仍为 `{_marker}` 占位，本函数的 state 是本地模型，
非真正资源接线（报告未把它计为判等接线，可接受）。

### 2.4 process_tree_cmds Insert（0x142267430）——需修

fresh asm/decomp 逐语句核对（R12=事件基址，payload 从 +8 起）：

- 名称门：长度 0x19/0x1a/0x27 + 16B 前缀（XMM7=[0x143034d40]="clearing/naked_t"）
  + 后缀常量 0x143034d30 "naked_tree/large"、0x143034d50 "aked_tree/naked1"、
  0x143034d60 "aked_tree/naked2"、0x143034f28/f4f no_mushrooms。Rust 5 串**全对**。
- `deny_load` 0x140a9fa70、`is_corrupted` 0x140ae8320 调用；`BL=naked&deny`；
  `MAXSS(scale,0.7)` 仅该组合生效（0x142267be8-0x142267c08），Rust 相同。
- α：`inverse_lerp(0.5,0.9,scale@+0x24)`→`r*0.75+(1-r)*0.55`（0x142267a64..
  0x142267a9e、0x142267c93 ADDSS），用的是**事件原 scale**；Rust 用 `cmd.scale` ✓。
- spawn_empty + 4 条 insert：TreeComponent → TreeAnimation(α) → TreeMoveBend
  (x,z 打包) → TreeAmbience(x, y+scale*10, z)。ambience 用**钳后** scale
  （XMM12，0x142267d39-0x142267d95），Rust 的局部 `scale` ✓。
- MISSING_TREE_PREFAB_NAME(0x18) 比较 → `random_tree_prefab`（0x14226a170）；
  否则 name clone。`spawns.write(SpawnPrefabCmd::default())`（T5 已登记 0x50 载荷）。
- TreeStorage::insert（0x140a38560，key=element+0x30、entry）；冲突日志串
  0x143034f76 "Tree  already exists. Replacing it with a new tree"（`{}` 为格式
  参数）与 Rust 文案一致；DespawnPrefabCmd 分支存在。
- 水面 130/130、`SETA` → selector；PlayTreeSoundCmd 槽 +8 kind=0、+9 selector、
  +a extra=naked&(deny|corrupted)、+c value=r、+10/18 position。Rust 逐字段 ✓。

**FIX-G4**：`random_tree_prefab` 的 selector。fresh 调用点 0x142268146
`MOV RCX,[RDI]`，RDI=LEA[R12+0x38] → **selector = 元素+0x30（即共享
`InsertTreeCmd.id`）的低字节**；0x14226a170 体内 `in_CL & 1` 取变体位。Rust 传
`0` 并在 T4 称 selector 由 variant/flags 合成而「共享槽缺失」——**该理由不成立**：
`cmd.id` 在共享类型中可用，应写 `(cmd.id & 1) as u8`。iVar22（variant/flags
合成）实际写进 SpawnPrefabCmd 载荷 [RBP+0xbc]，不是 random_tree_prefab 入参。
`is_corrupted=false` 仍是真实 T4 缺口（宿主资源未取证），保留登记。
其余 Insert 语句判等（可表达面）。

### 2.5 ui_move_tree（0x14226b1a0）——需修

fresh 454 条指令 + decomp：

- 门禁：`tag=(mode∈[6,0x14])?mode-5:0`；`tag==0xb || (tag==0 && mode==0)`，再
  `MOV RAX,[0x14362c098]; CMP byte[RAX],0; JZ continue`——**显式 !DEMO**。Rust 只判 mode tag，
  **没有 DEMO 门**。
- 候选循环原生来自 `Decorators::iter_removed_ids` + wall/decorator 表定位；
  Rust 用 `storage.iter()` 代理（RESTORE 已登记分歧 9）。位置也取自候选变换
  记录（0x30 步长）而 Rust 取 `TreeStorageEntry.position`——同一代理的一部分。
- widget 名格式串/`UiState::add`；`dragged` 实调 0x14092df10（非 decomp 名
  into_drag_state）✓；`AL-2`、`CMP AL,3; CMOVNC EAX,3`（**≥5 全部钳到 3**）→
  跳表 0x143035538：case0(空闲2)=0x14226b493、case1=0x14226b430、
  case2=0x14226b8e8、case3=0x14226b9c0。
- 光标 y<0 重映射 `(y/-2.5)*-1.2+(1-y/-2.5)*0.0` ✓（Rust 相同）。
- case1 写 Local：原位置（+0/+8）与 `树xz − 光标xz`（+0xc/+0x14）✓。
- case2：目标=光标+偏移、地形采样、`is_pos_inside`；**界内** MoveTreeCmd
  （0x20 槽，+8 id、+0x10.. 位置，`y=max(h,0)`）；**界外**两个单位事件
  （stride 8）DisplayOutOfBorder + FeedbackInputOutOfBorder ✓。
- case3：界内 MoveTreeCmd + History::add（0x58 载荷：tag 1/1/1、旧位
  +0x14/+0x1c、新位 +0x20/+0x24/+0x28、id +0x30）；界外无事件。Rust 只写新位
  （T3 登记）。
- **原生 case2/case3 没有 `dragging==Some(id)` 门，也不在 case3 清 Local**；
  Rust 增加了 `cache.dragging`（case1 置位、case2/3 门控、case3 清空），该字段
  在机器上不存在。

**FIX-G5**：`ui_move_tree.rs:75-83` 补 `!DEMO`（`country_core` 的 demo 覆盖字节/
资源，按 0x14362c098 语义）。
**FIX-G6**：删除或明确登记 `cache.dragging` 附加门与 case3 清空；原生用 Local
旧偏移无 id 门。另 `match state {3,4,5}` 应复刻 `≥5 → case3` 的钳位
（`state>=5` 走 case3）。
MoveTreeCmd 载荷模型中 `id@0/index@8/position@0x10` 与原生 wire
（EventInstance 头 8B 后 +8=id、+0x10=position）存在 8B 偏移歧义；生产/消费
两侧都用同一共享结构时逻辑自洽，但**不构成逐字节判等**（见 §5 附带登记）。

## 3. 8 条分歧检查表扫描

| # | 检查项 | 结果 |
|---|---|---|
| 1 | panic-vs-return 反 | 未命中：find_stone 跳表 panic 桩/None 逐项；circle 缺墙 unwrap；replay 移除 assert 一致 |
| 2 | 门控极性反 | **命中 1**：ui_move_tree 缺 `!DEMO`（FIX-G5）；其余（radius 早退、rect None、naked&deny、undo/flags/delay）方向正确 |
| 3 | 载荷平面近似 | **命中 2**：maintain_dormer ids push vs 单槽 store（FIX-G3）；MoveTreeCmd index 模型（附带登记）。Insert T2、SpawnPrefab T5 已登记 |
| 4 | 事件/命令自造 | **命中 1**：process_tree Insert 的 selector=0 理由不成立（FIX-G4）；不存在无机器依据的新事件 |
| 5 | callee 转发错 | 未命中：0x1405b0d70/0x140053320 纠错**成立**；find_stone 的 raycast/sample/xvy/new/as_points2 与 callee 身份一致 |
| 6 | 返回结构不符 | 未命中：Option<f32> EAX 判别、bool AL、slide 状态 u8 |
| 7 | 排序取最近 vs 首个 | 未命中：minss 链顺序、A→B 迭代顺序、移除索引反转均对上 |
| 8 | 偏移表/字段错位 | **命中 2**：circle 的 28B shape 解释正确（无错位）；但 dormer ids 槽（FIX-G3）与 MoveTreeCmd index 模型属字段面问题；process_tree Insert 事件 8B 头/元素基址经复算无错位 |

## 4. FIX 清单

- **FIX-G1**（circle_categorize，低危 NaN/∞）：`circle_categorize.rs:105` 的
  `.max(0.0).min(1.0)` 与 `:133` 的 `<` 未复刻 `MAXSS/MINSS` 和 `UCOMISS+SETC`
  的 unordered 语义。改用显式 `maxss/minss` 与 NaN/∞-inclusive 比较。
- **FIX-G2**（replay_tree，中危）：`replay_tree.rs:77-84` Move-undo 分支
  **完全不发事件**；原生发 `MoveTreeCmd`（旧位置 edit+0x14/+0x1c）。在
  `TreeEditData` 落旧位前，把该路径在总账标为「未实现路径」，verdict 由
  「判等」降为「重建」。
- **FIX-G3**（maintain_dormer_tiles，中危）：`:133` `ids.push(value)` 改为
  单槽 store（`*slot = value` / U 面指针字段）；与 fresh `MOV [R13],EBP`
  （R13 循环内不变）一致。
- **FIX-G4**（process_tree_cmds Insert）：`process_tree_cmds.rs:209-217`
  `random_tree_prefab(0, ...)` 的 selector 应为 `(cmd.id & 1) as u8`
  （fresh 0x142268146 传入元素+0x30=id）；同步修正 T4 的 selector 理由。
- **FIX-G5**（ui_move_tree）：`:75-83` 补 `!DEMO` 门（0x14362c098 指向的 DEMO 覆盖字节）。
- **FIX-G6**（ui_move_tree）：移除/登记 `cache.dragging` 附加门与 case3 清除；
  复刻 `state>=5 → case3` 钳位。
- **文档纠正（非 src）**：circle_categorize 的 fresh 指令数应为 **197**（837B），
  不是 105；`todo!` 降级文件数为 **11**（表 11 行、17 个 `todo!`），并补列
  `overhang_subscribe_to_tabloids`（0x140786440，空壳登记）。

## 5. 高估检查

1. **replay_tree「判等」高估**：Move-undo 是**整条事件缺失**，不是 T3 所述的
   「载荷只写新位」；其余分支判等属实。
2. **maintain_dormer_tiles「判等」高估**：id 写面 push vs 单槽 store 未体现；
   其余四步与 cursor/链遍历判等。
3. **process_tree Insert 的 T4 selector 论证不成立**：selector 来自共享
   `id` 低位，可表达；只有 `is_corrupted` 资源面确为缺证。
4. **ui_move_tree 的 RESTORE 门禁描述与代码不符**：RESTORE 写「&& !DEMO」，
   Rust 未实现；状态机另加了原生不存在的 id 门。
5. **circle_categorize 证据描述高估**：837B/197 条被写成 105 条。
6. **todo 降级计数**：10 vs 11 文件；`overhang_subscribe_to_tabloids` 空壳
   未进表（但它是诚实登记，不构成假实现）。
7. 未发现新的自造事件、虚假 callee 转发或把近似体标成判等的情况；
   `0x1405b0d70/0x140053320` 的纠错声明经 fresh 签名核实**成立**。

## 6. 复跑建议（未在本轮执行）

`cargo check -p system-wall-constructor -p system-tree -p system-roof -p system-decorator`
与 `cargo test -p system-wall-constructor --lib`（circle_categorize 2 单测）。
本轮只做只读反编译对照，未运行构建。
