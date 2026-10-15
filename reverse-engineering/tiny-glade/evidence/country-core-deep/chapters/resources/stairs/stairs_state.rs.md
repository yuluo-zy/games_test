# resources/stairs/stairs_state.rs

> 文件作用：**权威楼梯结构图：petgraph GraphMap（@+0x28）+端点/脏边集合(@+0x148)/备份快照 diff（历史撤销用）**

函数总计 58：业务 27 / 内联无独立符号 11 / 派生胶水 20

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::stairs::stairs_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1402e0630 | 38 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::stairs::stairs_state::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x14157d500 | 38 | 派生胶水 | O：serde derive（复用依赖，不计业务） |  |
| `country_core::resources::stairs::stairs_state::impl$1::from` | 0x1408fe5c0 | 50 | 转换 | I：From 转换 |  |
| `country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator::get_or_allocate` | 0x1408fe5e0 | 79 | 构造 | O：TransientStairEdgeIdAllocator::get_or_allocate——已有返 id，否则插 next+1 | collision-eco\decomp\decomp-0x1408fe5e0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::try_set_endpoint` | 0x1408fe7b0 | 98 | 系统:写入 | O：try_set_endpoint——候选邻边**≥2**→Unset，否则 Single（`7 < len*8-8`；review-audit 纠错：阈值 8→2） | collision-eco\decomp\decomp-0x1408fe7b0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::unset_endpoint` | 0x1408fe880 | 107 | 系统:写入 | O：unset_endpoint=写 qword[+0x10]=3 | collision-eco\decomp\decomp-0x1408fe880.txt |
| `country_core::resources::stairs::stairs_state::StairsState::endpoint` | 0x1408fe890 | 110 | 查询/计算 | O：endpoint getter——拷贝 24B（判别 qword@+0x10+数据@+0x18/+0x20） | collision-eco\decomp\decomp-0x1408fe890.txt |
| `country_core::resources::stairs::stairs_state::StairsState::graph` | 0x1408fe8b0 | 114 | 查询/计算 | O：graph getter=&self+0x28（GraphMap 起点） | collision-eco\decomp\decomp-0x1408fe8b0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes` | 0x1408fe8c0 | 118 | 查询/计算 | O：get_all_connected_nodes——BFS 连通集+排序去重 | collision-eco\decomp\decomp-0x1408fe8c0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::make_backup` | 0x1408fefe0 | 140 | 系统:备份 | O：make_backup——0x90B 图状态备份 | collision-eco\decomp\decomp-0x1408fefe0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::restore_backup` | 0x1408ff210 | 160 | 系统:备份 | O：restore_backup——0x90B 回拷+置脏 +0x2f=1 | collision-eco\decomp\decomp-0x1408ff210.txt |
| `country_core::resources::stairs::stairs_state::StairsState::compute_checksum` | 0x1408ff430 | 202 | 系统:备份 | O：compute_checksum——iter_edges_and_stubs→StairsSnapshot::new→utils::calculate_hash（wp21 合同；decomp 1450B） | collision-eco\decomp\decomp-0x1408ff430.txt |
| `country_core::resources::stairs::stairs_state::StairsState::restore_snapshot` | 0x1408ff590 | 213 | 系统:备份 | O：restore_snapshot——checksum→diff→apply | collision-eco\decomp\decomp-0x1408ff590.txt |
| `country_core::resources::stairs::stairs_state::StairsState::add_node` | 0x1408ffd30 | 231 | 系统:写入 | O：add_node（64B 手解）——+0x00/+0x178 清零→GraphMap::add_node(self+0x28)→+0x148 插 (n,n) 自环桩 | collision-eco\decomp\decomp-0x1408ffd30.txt |
| `country_core::resources::stairs::stairs_state::StairsState::add_edge` | 0x1408ffd70 | 243 | 系统:写入 | O：add_edge——自环 panic；归一化(min,max)；端点命中短路；3 处边/桩表插 | collision-eco\decomp\decomp-0x1408ffd70.txt |
| `country_core::resources::stairs::stairs_state::StairsState::disconnect_edge` | 0x1408ffe80 | 266 | 系统:写入 | O：disconnect_edge——同端点→remove_node；否则 remove_edge+插桩 | collision-eco\decomp\decomp-0x1408ffe80.txt |
| `country_core::resources::stairs::stairs_state::StairsState::remove_node` | 0x1408ffef0 | 298 | 系统:写入 | O：remove_node——邻边插桩 (min,max)+自环 (n,n)→GraphMap::remove_node | collision-eco\decomp\decomp-0x1408ffef0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::extract_diff_and_make_backup` | 0x140900010 | 320 | 系统:备份 | O：extract_diff_and_make_backup——两快照差集 {removed,added}+备份 | collision-eco\decomp\decomp-0x140900010.txt |
| `country_core::resources::stairs::stairs_state::StairsState::reduce_diff` | 0x1409007b0 | 338 | 系统:备份 | O：reduce_diff——差异化简（与 apply_diff 0x900ba0 成对；移除/添加边对预处理，wp21 合同；decomp 8550B） | collision-eco\decomp\decomp-0x1409007b0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl$5::collect_edges::closure_env$0>` | 0x14157d090 | 362 | 查询/计算 | O：iter_edges_and_stubs 实例2（reduce_diff 闭包） | collision-eco\decomp\decomp-0x14157d090.txt |
| `country_core::resources::stairs::stairs_state::StairsState::iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl$5::restore_backup::closure_env$0>` | 0x14157cec0 | 362 | 查询/计算 | O：iter_edges_and_stubs 实例1（restore_backup 闭包） | collision-eco\decomp\decomp-0x14157cec0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl$5::take_snapshot_impl::closure_env$0>` | 0x14157d200 | 362 | 查询/计算 | O：iter_edges_and_stubs 实例3（take_snapshot 闭包） | collision-eco\decomp\decomp-0x14157d200.txt |
| `country_core::resources::stairs::stairs_state::StairsState::apply_diff` | 0x140900ba0 | 394 | 系统:备份 | O：apply_diff——移除 removed/添加 added | collision-eco\decomp\decomp-0x140900ba0.txt |
| `country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub` | 0x140900e80 | 474 | 查询/计算 | O：contains_edge_or_stub——a==b 节点存在性；否则边/桩存在性（+0x38/+0x80 单元素快路径） | collision-eco\decomp\decomp-0x140900e80.txt |
| `country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub_impl` | 0x140900fd0 | 478 | 查询/计算 | O：contains_edge_or_stub_impl（非内联面） | collision-eco\decomp\decomp-0x140900fd0.txt |
| `country_core::resources::stairs::stairs_state::StairsSnapshot::new` | 0x140901120 | 528 | 构造 | O：StairsSnapshot::new——边排序→checksum | collision-eco\decomp\decomp-0x140901120.txt |
| `country_core::resources::stairs::stairs_state::StairSegment::new` | 0x140901260 | 576 | 构造 | O：StairGraphEdge::new 与 StairSegment::new ICF 折叠——(min,max) 归一化 helper | collision-eco\decomp\decomp-0x140901260.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::stairs::stairs_state::StairGraphEdge::new`（行 56）— 构造
- `country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator::get`（行 89）— 查询/计算
- `country_core::resources::stairs::stairs_state::StairsState::clear_backup`（行 129）— 删除/清理
- `country_core::resources::stairs::stairs_state::StairsState::force_make_backup_no_cleanup`（行 152）— I：强制备份（不清临时状态；make_backup 变体）
- `country_core::resources::stairs::stairs_state::StairsState::take_snapshot`（行 188）— I：取快照（iter_edges_and_stubs→StairsSnapshot）
- `country_core::resources::stairs::stairs_state::StairsState::take_snapshot_impl`（行 192）— I：取快照（iter_edges_and_stubs→StairsSnapshot）
- `country_core::resources::stairs::stairs_state::StairsState::collect_edges`（行 384）— 迭代/变换
- `country_core::resources::stairs::stairs_state::StairsState::drain_dirty_edges`（行 489）— I：排空脏边集合（+0x148 集合，供 invalidate 消费）
- `country_core::resources::stairs::stairs_state::StairsHistoryDiff::is_empty`（行 511）— 谓词/校验
- `country_core::resources::stairs::stairs_state::StairsSnapshot::edges`（行 540）— O：StairsSnapshot::edges getter（I 内联）
- `country_core::resources::stairs::stairs_state::StairsSnapshot::compute_checksum`（行 544）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`hash`、`visit_map`、`visit_seq`、`visit_str` 等 20 项

## 实现方式（算法级）

完整算法见 `collision-eco/ALGORITHMS.md` §3。要点（含机器码更正）：

### 备份/恢复机制（机器码裁决，推翻 DIGEST 两处）
- **备份区 = +0xb8 起 0x90B**（图的镜像区，与图区 +0x28..+0xb8 等长；原记 +0xb0 差 8）。
- **restore_backup 只写 +0x178**（开头 0、结尾 1）；**无 +0x2f 写入** —— 原"+0x2f 脏字节"为误记，实际为 +0x178 `backed_up` 标志（add_node/remove_node 清零；make_backup 守卫后置 1；restore_backup 置 1）。
- **make_backup**（0x1408fefe0）：`if (+0x178==0)` 守卫 → iter_edges_and_stubs（graph@+0x28, backup@+0xb8, stubs@+0x148）+ 收集当前边 → 读图字段（+0x60..0x6c/+0xa8/+0xb0）→ 释放旧备份缓冲（+0x100/+0x108、+0x118/+0x120）→ 组装 0x90B blob → memcpy(+0xb8) → +0x178=1。
- **restore_backup**（0x1408ff210）：`qword[+0]=0`、`byte[+0x178]=0` → iter_edges_and_stubs + 收集备份边（来自 +0xb8）→ 读备份字段（+0xf0..0xfc/+0x100/+0x138）→ **释放当前图内部 Vec 缓冲（+0x70/+0x78、+0x88/+0x90）** → memcpy 0x90B 覆盖图区 +0x28（整个 GraphMap 被备份替换）→ +0x178=1。

### add_node（64B 手解）／remove_node（llvm-objdump 全解）
- add_node：`qword[+0]=0` → `byte[+0x178]=0` → GraphMap::add_node(self+0x28)（0x140cb10c0）→ 尾调用插桩 (n,n) 进 +0x148。
- remove_node：`qword[+0]=0`、`byte[+0x178]=0` → 节点查找（nodes_len@+0x38：0 跳过 / 1 单节点快路径 / hash 查找 0x140106b70+0x140b4f360；未找到跳过；idx≥len panic）→ 读邻接（arena+0x8/+0x10 + idx·0x28 = 邻接表 ptr/计数）→ 每邻边 cmov 归一化 (min,max) 插桩进 +0x148（0x1402c8640）→ 自环桩 (n,n) → GraphMap::remove_node（0x14093a220）。

### contains_edge_or_stub（0x140900e80）
a==b：节点存在性（单节点/空快路径 + 双次 hash 查找 + 越界 panic 0x142c812b0 + 邻接条目空判定）；a!=b：归一化后边/桩存在性（edges_len@+0x80==1 单边快路径 {a@+8,b@+0xc}；否则 FUN_140106840+FUN_140b4f270 于 +0x70）。
