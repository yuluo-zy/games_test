# input + geometry 未探究闭合（round3，2026-10-15）

> 闭合 groupE-newmodules 的 input + geometry 两模块未探究项（清单 #6/#7/#8/#9/#14/#10 + NotFound 卫生）。
> 证据方法：RE-MCP 定点反编译（dbid `tg_ci1`，3 目标全部 NotFound）→ 全部改 **capstone 机器码手解**
> （`evidence/country-core-deep/tools-extra/disasm-va.py` / `disasm-consts.py`，对 B patched EXE），
> 复用既有 geometry-gap/expansion-b2 decomp。**不改 reconstruction 源码**。

---

## 1. InputStream `deserialize::visit_str` 0x1413fe800 —— 闭合（算法级，O）

组 E 清单 #9（"算法未深挖，仅结论级"）。RE-MCP 数据库 NotFound（函数未建），按 PLAYBOOK §1.3 机器码手解，
得到完整算法。落盘 `input/decomp/hand-0x1413fe800-visitstr.txt`（capstone + rip 相对常量解析，0x108B）。

**身份**：`InputFrame` 的 Deserialize visitor `visit_str`（键名→serde 字段序号）。与 `serialize<Pretty>`
0x1413fe6f0（InputFrame 4 键序列化）紧邻；const 窗取自有 `GpuWorldSubsetKind` 无关的独立融合串池
@0x142c5aeae..0x142c5af10（含 `action_set_value`/`main_action_set_`/`time_delta_secon`/`lized_cursor_pos`/
`prev_normalized_`/`translation_stat`/`ightType`，后缀共享融合）。
**命名修正**：清单/章节原标「InputStream deserialize visit_str」→ 实为 **InputFrame（InputStream 帧元素）**
的 visit_str，匹配 4 个 InputFrame 键（非顶层 sync_points/frames）；顶层 InputStream 的 visit_str 另在
邻域（0x1413fea80 等，tg_b2/tg_ci1 均 NotFound）。

**算法**：
```
visit_str(out: &mut [u8;2], key: &str)   # out[0]=0 固定，out[1]=字段序号（0..3），4=unknown
r8 = key.len - 0x11                       # 长度以 0x11=17 为基
if r8 unsigned > 9 → default（写 4）       # 有效长度仅 17/18/21/26
jump table @0x142c5baa0（按 r8，10 项）分派：
  len 17 → 0x1413fe822：SIMD 16B [rdx] vs const"translation_stat"@0x142c5af00
                        + 1B [rdx+0x10] vs 'e'@0x142937ee0   → 命中写 out[1]=0
  len 21 → 0x1413fe85a：SIMD 16B [rdx] vs "main_action_set_"@0x142c5aec0
                        + 16B [rdx+5]  vs "action_set_value"@0x142c5aeb0 → 命中写 out[1]=3
  len 18 → 0x1413fe88e：SIMD 16B [rdx] vs "time_delta_secon"@0x142c5aed0
                        + 2B  word [rdx+0x10] vs "ds"@0x14295db20   → 命中写 out[1]=2
  len 26 → 0x1413fe8c2：SIMD 16B [rdx] vs "prev_normalized_"@0x142c5aef0
                        + 16B [rdx+0xa] vs "lized_cursor_pos"@0x142c5aee0 → 命中写 out[1]=1
  len 19/20/22..25 → default（写 4）
len 17/18/21/26 未命中 / 其余长度 → out[1]=4（unknown 哨兵）
```
要点：**后缀共享 SIMD 双窗**（输入串两处错位读 vs 融合串池 16B 常量窗），与 action_set visit_str
0x1416f92d0 同族；但 InputFrame 侧更紧凑（每键单/双窗，无 46 槽跳表）。

**新结论（serde 字段序，O）**：InputFrame serde 字段序 = variant 序 =
`[translation_state(0), prev_normalized_cursor_pos(1), time_delta_seconds(2), main_action_set_value(3)]`
（serialize<Pretty> 0x1413fe6f0 写序一致：translation_state → prev_normalized_cursor_pos →
time_delta_seconds → main_action_set_value）。**修正 ALGORITHMS §2.2 旧写法**
（原列 `translation_state@0x00 → main_action_set_value@0x68 → prev_normalized_cursor_pos@0xc0 →
time_delta_seconds@0xc8` 混入内存偏移，字段序实为上述顺序；内存偏移照旧）。
serde 值偏移（serialize<Pretty>，O）：translation_state@0x00（11 qword 88B）、main_action_set_value@0x68
（Vec<[u8;12]> 24B）、prev_normalized_cursor_pos@0xc0（Vec2）、time_delta_seconds@0xc8（f32）。

## 2. MainActionSet 双形态裁决（清单 #6，O 双侧）

- **运行时形态**（0x170B，digital-first）：`deconstruct` 0x140946b80 首条读 `movsd xmm6,[rdx+0x140]`
  = analog 槽 0 @0x140；digital 槽 {disc,val} 从 8k 字节（@0x00..0x140）读（本轮 capstone 复采头 0x70B）。
- **serde 形态**（0x58B，analog-first）：`serialize<Compact>` 0x1400ed690（tg_m5 有效 decomp）读
  `param_1[0..0xb]`（12 f32 = 6×Vec2 @0x00..0x30）+ `param_1+0xc..0x57`（40 字节 digital @0x30..0x58）。
- **裁决**：两形态字段序相反，机器码双侧都硬。`reconstruction/country-core/src/input/action_set.rs`
  `pub struct MainActionSet<A,D>{ analog, digital }`（analog 在前，注释「Field layout U」）——**只匹配 serde
  形态（analog@0x00），不匹配运行时 digital-first 0x170B 形态**。建议：双形态建模（运行时
  `MainActionSet<Vec2,DigitalActionState>` 0x170B + serde 专用 0x58B 表示，如 `<Vec2,bool>` 独立类型或
  `#[serde(...)]` 转换），恢复落码时按 ALGORITHMS §1.1 两表分别建模。不改源码。

## 3. InputFrame 两视图偏移冲突裁决（清单 #7，O 双侧，非同一字段）

- **serde 视图**（serialize<Pretty> 0x1413fe6f0 + visit_str 0x1413fe800）：translation_state@+0x00（88B）、
  main_action_set_value@+0x68（Vec 24B）、prev_normalized_cursor_pos@+0xc0、time_delta_seconds@+0xc8。
- **回放视图**（input_record_replay 0x1409423d0，case 2 + 录制 default 分支，tg_m5 有效 decomp）：
  帧内 **11-qword 状态块 @+0x68..0xb8**（读到 InputTranslationState 资源 @+0x58..0xa8）、游标 @+0x40、
  除数 @+0x44、prev_pos @+0xc0。录制路径（default 分支）把当前状态 11 qword（state+0x58..0xb0 读）
  写入帧 @+0x68..0xb8，与回放读一致。
- **裁决**：serde `translation_state`（@0x00..0x58，88B 平坦持久化表示）与回放状态块（@0x68..0xb8）
  **不是同一字段的不同偏移，而是两个不同子结构/表示**。两视图仅在 @0xc0（Vec2 prev_pos/
  prev_normalized_cursor_pos）一致；@0x68 在 serde 是 `main_action_set_value` Vec、在回放是状态块 qword0。
  恢复时 serde 布局与回放帧布局须分别建模（勿以单一 0xd0B 结构同时满足两偏移）。ALGORITHMS §6.2 沿用并加裁决。

## 4. InstancedWall `new` + `impl$1::from`（清单 #14，O 机器码，真 U 闭合）

两函数 RE-MCP NotFound → capstone 手解（落盘 `components-geometry/decomp/hand-0x140858800-new.txt`、
`hand-0x140858790-from.txt`）。

### `InstancedWall::new` 0x140858800（0x24=36B）
```
new(this, id: u64, bricks: Vec<BrickTransformSsbo>, writer: Option, storage: Option)
  [out+0x00] = id (rdx)
  [out+0x08..0x18] = bricks Vec{cap@8, ptr@0x10}（movups [r8] 16B）
  [out+0x18] = bricks.len（[r8+0x10]）
  [out+0x20] = writer（r9）
  [out+0x28] = storage（[rsp+0x28] 第 5 参）
```
要点：Vec **整体换入**（拷贝 cap/ptr/len，不 clone），4 字段一次写全 = 48B 布局
`{id@0, Vec{cap@8,ptr@0x10,len@0x18}, writer@0x20, storage@0x28}`（与 ALGORITHMS §5 一致，O）。

### `impl$1::from` 0x140858790（= `From<u64> for BrickSourceId`，0x25=37B）
```
From::from(wall_id: u64) -> BrickSourceId
  ecx = (u32)wall_id - 100000（add 0xfffe7960）
  if (u32)(wall_id-100000) <= 11:                       # wall_id ∈ [100000, 100011]
    if bit(wall_id-100000) set in 0x80f (=bits 0,1,2,3,11):   # bt edx,ecx; jae skip
      eax = table[(wall_id-100000)]   # table @0x142a953f0（12 项，round3 同 agent 复验）
      → 返回 0x80000000 + idx（TransientStairEdge 判别编码，niche 0x80000000 基址，PLAYBOOK §2.3）
  else / bit 未置 → 返回 wall_id 原值（= Wall 判别）
```
**与 `from_wall_id` 0x140858750 完全相同**（同表 @0x142a953f0；0x140858750 用 `lea ecx,[rax-0x186a0]`，
0x140858790 用 `add ecx,0xfffe7960`，等价）——PDB 里 `from_wall_id`（= `From<ColorId>`）与 `impl$1::from`
（= `From<WallId>`）是重复实现。查表 12 项 = `[0x80000000..0x80000003, 0x80000000×7, 0x8000000b]`
（idx 0..3 为 0x80000000|idx，idx 4..10 为 0x80000000，idx 11 为 0x8000000b）；0x80f 掩码仅 {0,1,2,3,11}
可达查表。**纠错（并发 agent 复验，本轮确认）**：`with_transient_stair_edge_id` 常量 = **0x40000000**
（bit30 家族位，`or eax,0x40000000`），非旧文 0x40；`new` 实为 0x24=36B（并发 agent 记 33B，差值登记）。
注意：**并发 agent 已同步闭合本行与 §5**（components-geometry/ALGORITHMS.md §5 + geometry/instanced_wall
章节均已含 new/from/掩码/0x40000000），本 closure 与其一致，无需重复落码。

## 5. 图集 parts key 常量语义（清单 #10，O 闭合）

load_mesh_atlases 程序化 parts 的 key = **`GpuWorldSubsetKind`（27 变体 u8）判别值**，不是哈希/任意索引。
FUN_140a371d0(reg, key, name, len, mesh) 把 part 网格注册进 `MeshAtlasLibrary.world_subsets:
HashMap<GpuWorldSubsetKind, WorldSubsetMeshAtlas>`（A3 res_render 0x50B 布局，O）；消费端 nani 各
`nani_update_*` 系统经 `owned_subset_mut(kind)` 按同一 kind 索引取网格（`reconstruction/country-core/src/render/
nani/gpu_world.rs` `GpuWorldSubsetKind` 枚举判别 + systems.rs kind 立即数，均已 O）。

| part 名（startup） | key | GpuWorldSubsetKind 变体（判别值） | 用途 |
|---|---|---|---|
| `stone_floor_` | 9 | RoofFloor=9（nani_update_stone_floor kind 9，StoneFloorInstanceData 0x40B） | 石地板/屋顶地板网格 |
| `timber_` | 0x18=24 | CurvedHalfTimber=24（nani_update_curved_halftimber kind 0x18） | 弧形半木构墙网格 |
| `plaster` | 0x13=19 | Plaster=19（nani_update_plaster kind 19） | 灰泥墙网格 |
| `stairs_floor` | 0x1a=26 | StairsFloor=26（nani_update_stairs_floor kind 0x1a） | 楼梯地板网格 |
| `smoke` | 0x15=21 | Smoke=21（nani_update_smoke kind 21） | 烟网格 |

注：enum 变体名 `RoofFloor=9` 为 reconstruction 命名，机器码侧同 kind 系统/网格名 `stone_floor`
（nani_update_stone_floor）；登记命名差异，语义=石地板渲染子集。落地 startup/ALGORITHMS §6.7 与
load_mesh_atlases 章节。

## 6. NotFound 坏证据检查（清单 #6 项，input/geometry 已卫生）

- `chapters/input/**`、`chapters/geometry/**` 无行把 NotFound 错误文件当唯一证据：
  - input/mod.rs.md:14 明确「0x140958f30/0x1408c5680 在 tg_m5 数据库 NotFound，以原字节为准」（已卫生）。
  - geometry/instanced_wall.rs.md:11 `from_wall_id` 注明「Ghidra NotFound，手解」+ 机器码证据（已卫生）。
- 引用 decomp 均有效：`geometry-gap/decomp/decomp-0x140858880.txt`（6632B）、
  `geometry-gap/decomp/decomp-0x140a06b00.txt`（9172B）。
- 本轮新增手解证据落盘（见 §7），无新 NotFound 引用。

## 7. 证据落盘

- `input/decomp/hand-0x1413fe800-visitstr.txt`（visit_str 全函数 capstone + const 解析，0x108B）
- `components-geometry/decomp/hand-0x140858800-new.txt`、`hand-0x140858790-from.txt`
- `components-geometry/decomp/decomp-0x140858880.txt`（copy geometry-gap）、
  `decomp-0x140946b80.txt`（copy expansion-b2，deconstruct 复采）
- job：`input/decomp/job-ci1.json`（open/wait/decompile×3/close，dbid tg_ci1，3 目标 NotFound 已登记）
- 工具：`evidence/country-core-deep/tools-extra/disasm-va.py`、`disasm-consts.py`（capstone 手解）
- 更新：`input/ALGORITHMS.md`、`components-geometry/ALGORITHMS.md`、`startup/ALGORITHMS.md`、
  `chapters/input/input_stream.rs.md`、`chapters/geometry/instanced_wall.rs.md`、
  `chapters/startup/load_mesh_atlases.rs.md`
