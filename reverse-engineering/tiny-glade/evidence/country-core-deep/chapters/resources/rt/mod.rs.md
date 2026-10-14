# resources/rt/mod.rs

> 文件作用：**RT 世界 CPU 侧**——`RtMesh`/`RtMeshHandle`/`RtWorld`（哈希表容器，key=WallId/RtMeshHandle）、
> `DirtyCow<T>`（引用计数 copy-on-write，脏标记即"持有唯一所有权可原地改"）、`RtBvhBuilderGlueResource`，
> 以及两个系统 `update_rt_world` / `upload_rt_world`（消费世界帧差→标记脏→触发 BVH 重建→上传 3 个 GPU SSBO）。
> 源码路径 `crates/country-core/src/resources/rt/mod.rs`；puffin-0.14.1 profiler；软件 BVH 接口与
> evidence/render-passes 的 rt_bl_bvh/rt_triangles/rt_triangle_meta 对应（无硬件光追）。

函数总计 31：业务 11 / 内联无独立符号 12 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$1::from` | 0x1408bc460 | 55 | 打包转换：`trunc(sqrt(x)*K+B)` 夹到 [0,M)（越界→-1 哨兵）——浮点值→紧凑 u64 字段 | I | 手解（rne-read --hex 0x1408bc460） |
| `impl$2::from` | 0x1408bc510 | 60 | 构造 `{tag=1, value=arg2}`（枚举变体打包） | O | 手解（rne-read --hex 0x1408bc510） |
| `DirtyCow::get_mut<RtMesh>` | 0x1415626f0 | 93 | `LOCK; if refcount==1 → 原地 mut（改引用计数语义）；else 克隆 RtMesh`；返回 `plVar23+2`（RtMesh 指针）；读网格字段 0x24/0x2c/…/0x64/0x6c | O | roofs-rt/decomp/decomp-0x1415626f0.txt |
| `impl$5::from` | 0x1408bc520 | 130 | `From<WallId>`：`xor eax,eax; mov rdx,rcx; ret` —— 空/零 key（RtWorld 键转换） | O | 手解（rne-read --hex 0x1408bc520） |
| `RtWorld::get_mut<WallId>` | 0x1412259c0 | 173 | 键转换（impl$5::from）→ 哈希查找（FUN_1405b14b0 + bucket 探测）→ 条目 mut 指针 | O | roofs-rt/decomp/decomp-0x1412259c0.txt |
| `RtWorld::get_mut<RtMeshHandle>` | 0x1400cce50 | 173 | 同上（RtMeshHandle 键变体） | O | roofs-rt/decomp/decomp-0x1400cce50.txt |
| `RtWorld::get<RtMeshHandle>` | 0x141562cf0 | 183 | 哈希只读查找 | O | roofs-rt/decomp/decomp-0x141562cf0.txt |
| `RtBvhBuilderGlueResource::new` | 0x14067dd80 | 207 | 共享去重体：`mov rax,rcx; ret`（返回 self/空构造；真实逻辑内联进 update_rt_world 调用点） | O（胶水） | 手解（rne-read --hex 0x14067dd80；原 audio-misc 等 decomp 均 NotFound 副本，弃用） |
| `RtBvhBuilderGlueResource::is_idle` | 0x1408bc530 | 211 | 谓词：busy 计数==0（小函数，手解待定） | I | 手解（rne-read --hex 0x1408bc530） |
| `update_rt_world` | 0x1408bc650 | 220 | 系统：比较帧/计数器差产生两类脏位→写回；脏且全局帧>4 时 `RtBvhBuilderGlueResource::new`；对 RtWorld 条目 OR 脏位（16 字节条目的位图提取）；panic "UpdateRtWorldEvent requested"（UNK_142aa09f8），puffin 剖面 | O | roofs-rt/decomp/decomp-0x1408bc650.txt |
| `upload_rt_world` | 0x1408bcff0 | 252 | 系统：逐脏条目置 `0x8000000000000000`（None 哨兵，占用标记）→ 按名称（FUN_140969720×3，字典查找）取三角 → 收集到本地缓冲 → `func_0x000140613c20` 上传 3 个 SSBO（stride 0x50/0x18/4×u32） | O | roofs-rt/decomp/decomp-0x1408bcff0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `impl$0::from`（行 37）— 颜色/通道打包 From（inline）
- `impl$0::from::pack_channel`（行 39）— 单通道打包（inline）
- `RtMesh::set_albedo`（行 67）— 反照率写入（inline）
- `DirtyCow::get_mut`（行 93）/ `get`（行 99）/ `consume`（行 109）— 引用计数读写/消费脏标记（inline）
- `RtWorld::reset`（行 147）— 全清；`consume`（行 152）— 取走并清脏；`generate_handle`（行 167）— 生成句柄；`remove`（行 190）— 删除；`is_dirty`（行 196）— 脏谓词
- `GpuRtTriangle::new`（行 310）— 三角→GPU 记录构造（inline）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`eq`、`hash` 等 8 项（RtMeshHandle/RtMesh 的 Clone/Eq/Hash + DirtyCow/RtWorld 默认 + get/consume 闭包）

## 布局/结构要点

- RtWorld 哈希表：`{entries_base@+0x0, bucket_mask@+0x8, …, len@+0x18}`，key 查找 FUN_1405b14b0 探测 16 字节条目。
- DirtyCow 引用计数经 LOCK/INC/DEC（PLAYBOOK 事实 4 的 Arc 语义）；refcount==1 → get_mut 原地（copy-on-write 就绪）。
- update/upload 两系统经 func_0x000140c18130（世界共享锁）+ 全局 0x14362c3a8（帧阈值）与 0x14362c420（worker 停止标志）协调。
- 交叉域：与 walls::WallId（wall 域）、mesh_atlas_library（render 域）、render-passes 软件 BVH SSBO、roof_tiles_gravity 等关联。
