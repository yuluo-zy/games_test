# geometry/instanced_wall.rs

> 文件作用：**砖实例墙的几何载体与 GPU 上传**（MODULE_GAP 判 0% 覆盖，本节补齐）。`BrickSourceId`（墙体砖 vs 楼梯瞬态砖）、`InstancedWall::update`（换缓冲）、`upload_to_storage_buffer`（砖变换→SparseInstanceBuffer，stride 0x60）。布局 O 见各函数。

函数总计 8：业务 6 / register 胶水 2（impl$58 0x14085caa0、impl$59 0x14085d190，bevy 单态化，见 components-geometry/ALGORITHMS §1）。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `BrickSourceId::from_wall_id` | 0x140858750 | 60 | **业务** 砖源 id 构造（= `From<ColorId>`，PDB）：机器码 `mov eax,ecx; lea ecx,[rax-0x186a0(=100000)]; cmp ecx,0xb; ja …; mov edx,0x80f; bt edx,ecx; jae …` —— **0x80f 位掩码判别**：`id-100000 ≤ 11 且 bit(id-100000)∈{0,1,2,3,11}` → 查表 0x142a953f0 返回 `0x80000000|idx`（瞬态楼梯边，i32::MIN 高判别位）；否则原样返回 id（Wall）。**纠错**：非"全 [100000,100011]"、返回值非"100000+idx"（Ghidra NotFound，手解）。 | **O** | 机器码 0x140858750（36B） |
| `BrickSourceId::impl$1::from` | 0x140858790 | 97 | **业务** = `From<WallId>`（37B，round3 补行）：与 `from_wall_id` 同 0x80f 掩码逻辑（`mov rax,rcx; add ecx,0xfffe7960; cmp ecx,0xb; ja; mov edx,0x80f; bt edx,ecx; jae; mov eax,ecx; lea rcx,[表0x142a953f0]; mov eax,[rcx+4*rax]; ret`）——瞬态边 → 表值 `0x80000000|idx`，Wall → 原 id。 | **O** | 机器码 0x140858790（37B） |
| `BrickSourceId::with_transient_stair_edge_id` | 0x140858780 | 79 | **业务** 瞬态楼梯边 id 构造：机器码 `mov eax,edx; shl eax,4; or eax,ecx; or eax,0x40000000; ret`（13B）—— **edge_id<<4 + self + 0x40000000（bit30）家族位**。**纠错**：常量是 0x40000000，非 0x40。 | **O** | 机器码 0x140858780（13B） |
| `InstancedWall::new` | 0x140858800 | 240 | **业务** 构造（33B，round3 补行）：`new(id, bricks: Vec<BrickTransformSsbo>（24B 按值→指针传）, writer: Option, storage: Option)` 整体拷贝 → `{id@0, Vec{cap@+8,ptr@+0x10,len@+0x18}, writer@+0x20, storage@+0x28}`（`mov [rax],rdx; movups xmm0,[r8]→[rax+8]; mov rdx,[r8+0x10]→[rax+0x18]; mov [rax+0x20],r9; mov [rax+0x28],[rsp+0x28]`）；不深拷贝砖数据。 | **O** | 机器码 0x140858800（33B） |
| `InstancedWall::update` | 0x140858830 | 252 | **业务** 换入新砖缓冲（77B）：`lea rsi,[rcx+8]`（Vec@self+8）；旧 cap@+8 非空（0<cap，负=空哨兵 i64::MIN）→ `dealloc(ptr@+0x10, cap*0x60, align 4)`（0x140613c20）；再把新 Vec {ptr,len,cap} 拷入 self+8/+0x10/+0x18（cap-first 布局：`[rdx+0x10]`→+0x18、16B xmm → +8/+0x10）。**语义：整体替换砖变换缓冲（旧缓冲释放）**。 | **O** | 机器码 0x140858830（objdump 全文） |
| `InstancedWall::upload_to_storage_buffer` | 0x140858880 | 257 | **业务** 砖变换上传：见下方算法节。 | **O** | decomp-0x140858880.txt |

## 内联/无独立符号函数

无。

## 算法级结论（round 算法深挖，2026-10-15）

### 1. `BrickSourceId` 判别编码（O，机器码，round3 修正）

- `from_wall_id(id)`（= `From<ColorId>`）与 `impl$1::from`（= `From<WallId>`，0x140858790）同逻辑：`id-100000 ≤ 11 且 bit(id-100000)∈0x80f 掩码 {0,1,2,3,11}` → 查表 `0x142a953f0[id-100000]` = **`0x80000000|idx`**（i32::MIN 高判别位 + 楼梯边索引，idx∈{0,1,2,3,11} 对应 0x80000000/…/0x8000000b）；否则原样返回 id（Wall 家族）。**纠错**：非"wall_id∈[100000,100011] 全为瞬态边"、非"100000+idx 偏移编码"；reconstruction `enum BrickSourceId { Wall(u64), TransientStairEdge(u64) }` 语义对，但 0x80000000 判别位与查表编码未建模。
- `with_transient_stair_edge_id(edge_id)`：`self | (edge_id<<4) | 0x40000000`——**bit30（0x40000000）家族位** + 4 位移位（edge_id 上限 15 与高家族位共存）。**纠错**：旧文"0x40"错误。

### 2. `InstancedWall` 布局（O，cap-first）

`{ id: u64 @+0, brick_transforms: Vec<BrickTransformSsbo 96B> {cap@+8, ptr@+0x10, len@+0x18}, writer: Option @+0x20, storage: Option @+0x28 }`（48B）。
- **修正**：旧行"`update` 的 Vec{ptr@0,len@8,cap@0x10}"与"+8 有 len"表述错误——cap-first 为 {cap@+8, ptr@+0x10, len@+0x18}，两函数一致；`update` 的"释放旧"判定为 `0 < cap`（cap==0 或 cap==i64::MIN 哨兵跳过）。

### 3. `upload_to_storage_buffer`（0x140858880）全算法

1. **取走 Vec**：保存 cap@+8/ptr@+0x10/len@+0x18，把 cap@+8 写 i64::MIN（moved-from 哨兵）。
2. **writer 检查**：Option@+0x20==None → panic（instanced_wall.rs，0x142a94b68 区域串表）。
3. **storage=Some(lVar3)**（@+0x28，即 `SparseInstanceBufferWriter<BrickTransformSsbo>`）：`parking_lot` 锁其内部 Mutex@+0x18（acquire_resource 0x140c81c70 / release 0x140c81e10 包裹）；storage 空（ptr==0||len==0）→ 起始槽 = 首砖参数字段(+0x28)-1，否则槽 = 空闲槽栈栈顶（storage+8 数组最后一个 u32）。
4. **clear + push**：`SparseInstanceBufferWriter::clear<BrickTransformSsbo>`（0x1412e4760）清空 writer；然后对每块砖（96B，栈上拷贝：16×u32 + 4×u64）调 `SparseInstanceBufferWriter::push<BrickTransformSsbo>`（0x1412e4450）——失败（返回非 0）→ **panic `Out of bricks.`**（0x142a94b90，源 instanced_wall.rs）；其余 panic（0x142c2eb90/0x142c2eb78）来自 sparse_instance_buffer.rs 内部断言。
5. **归还内存**：`dealloc(ptr@+0x10, cap_orig*0x60, align 4)` 释放被取走的砖 Vec。
6. 解锁（waiters 唤醒路径 0x1428cdb90）。

要点：上传 = **clear-then-push** 整体重写 sparse writer（非增量）；`BrickTransformSsbo` stride **0x60=96B**；writer 内部自持锁（与 transient_mesh 的 Mutex+队列不同——这里是直接锁定稀疏缓冲写路径）。

### 4. 修正/补充

- 旧行"调 SparseInstanceBuffer 族（FUN_140c81c70/0x140c81e10…），字段 U"→ 已命名：0x140c81c70=parking_lot deadlock acquire、0x140c81e10=release、0x1412e4450/0x1412e4760 = `SparseInstanceBufferWriter::push/clear<BrickTransformSsbo>`（PDB 权威）。
- panic 串：`Out of bricks.`@0x142a94b90（instanced_wall.rs）；`Out of bricks.` 后接 instanced_wall.rs 与 sparse_instance_buffer.rs 双源路径串表。
