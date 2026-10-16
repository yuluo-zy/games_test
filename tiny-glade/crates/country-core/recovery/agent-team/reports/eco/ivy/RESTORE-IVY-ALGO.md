# restorer-ivy — ivy 库内主算法闭合 RESTORE 报告

日期：2026-10-16
角色：restorer-ivy（ivy 主算法专项，country-core 团队动态派发）
状态：in_review（grower 主生长闭包逐相实现 + send_to_gpu 48B 布局按 shader 定案 + pruner 空间哈希）
涉及文件：`systems/ivy/ivy_grower.rs`、`systems/ivy/ivy_spawner.rs`、`systems/ivy/ivy_pruner.rs`、
`systems/ivy/ivy_send_to_gpu_leaf.rs`、`systems/ivy/ivy_send_to_gpu_segments.rs`、
`resources/ivy/ivy_storage.rs`、`recovered_building/wall_spatial_index.rs`
证据落点：`reports/eco/ivy/evidence/decomp-*.txt`（grower/spawner/pruner/GPU fresh 反编译）、
`reverse-engineering/tiny-glade/evidence/country-core-deep/collision-eco/ivy-grower-disasm/*.txt`
（0x14094a910 全相体 4066 条指令分段落盘）

## 一、关键证据裁决（2026-10-16 fresh 反汇编/反编译）

1. **grower 相表 0x142ab3be0 逐相确认**（rne 直读 10 项 + 相体反汇编）：
   相 0→0x14094c09f（守卫翻转）；相 1/3/4→0x14094ac20（共用主入口）；
   相 2→0x14094ac31（计时 `ucomiss 0, [state+0x11c]`）；相 5→0x14094f2f2（二级表 0x142ab3bf4，5 项全 panic）；
   相 6→0x14094f36c、7→0x14094f31e、8→0x14094f356、9→0x14094f308（panic 尾）。
2. **0x142923ee0 = CRT `floorf`（IAT thunk 20，api-ms-win-crt-math-l1-1-0.dll）**——
   任务/证据"RNG 0x142923ee0×2"系误读，实为对格坐标的两次 `floorf`（与 spawner/pruner 的
   `floorf(auVar29._4_4_/_0_4_)` 完全一致）。grower 新段提案的"随机格采样"= 尖端位置的确定性格换算，
   **不是 perchance RNG**。
3. **主生长闭包 = 守卫体 0x14094ac51（非 0x14094c0c0）**：0x14094c0c0 是共享 epilogue（restore+ret）。
   PDB 函数边界：ivy_grower 0x14094a910..0x14094c7fb（7915B，procs.tsv）；其后 closest_segment_index
   0x14094c800、get_wall_double_sided_normal 0x14094caa0、intersect 0x14094cc60、ivy_spawner 0x14094cf90。
4. **守卫位语义**：相 0/2 翻转 +0x48 守卫位后运行守卫体（=生长闭包，带 `time.delta`）；位清 → epilogue 返回。
   无"delta=0 守卫体"与"delta 主体"之分——旧实现的分裂模型为近似，已按机器码合并为单一闭包。
5. **每株生长延迟 = native +0x88（= `spawn_ivy` 的 scalar 域）**：`delay > 0.001`（0x1429cd060）时
   `delay -= time.delta`（0x14094adf5）并跳过该株。
6. **新段提案流程**（0x14094aef9..）：尖端位置→格换算（floor×2）→格内段 `closest_segment_index(0.5)`
   →`0.365`（0x142ab3190）接受→所属墙 `>=2` 段且高>0→`IvyDirectionProposer::get_direction`（0x14094b4cc）
   →`approx_u_from_pos`（0x14094b536）→`WallCoord::into_world_space`（0x14094b7b9）→高度带
   （0x14094b73d..b77a）→`add_point`（0x14094bf76）。
7. **48B GPU 元素布局按 shader 定案**：`instanced_ivy_leaf.raster.hlsl` 的
   `IvyLeaf{position,size,forward_floattor,seed,branch_dir,arch_height}` 与
   `instanced_ivy_branch.raster.hlsl` 的 `IvySegment{start_pos,start_thickness,end_pos,end_thickness,wall_normal,arch_height}`。
   旧 `IvyLeafGpu{position,vector_2c,vector_58,size,branch_thickness,growth}` 字段集与 shader 不符，已修正。
8. **is_point_still_valid（0x140998dc0）**：格内 closest_segment_index(0.5)→0.365→墙有效
   （段数≥2、高>0）→插值高带 `(-0.01/h, 1+0.01/h)`（0x140998dc0）→附加样点（±0.1 AABB）。

## 二、实现目标 → Rust 位置 → 证据

| 目标 | Rust 位置 | 证据 |
|---|---|---|
| grower 10 相分派（守卫翻转/主入口/计时/panic 尾） | ivy_grower.rs `ivy_grower` 486 行 | 跳表 0x142ab3be0 + 相体反汇编（ivy-grower-disasm/entry+phase5_tail） |
| 单生长闭包（守卫体，带 delta） | ivy_grower.rs `run_growth_pass` 312 行 | 守卫体 0x14094ac51（guard_body_annotated.txt） |
| 每株生长延迟（+0x88） | ivy_storage.rs `growth_delay`（由 scalar_88 改名） + run_growth_pass | 0x14094aea9/0x14094adf5、0x1429cd060 |
| 新段提案（格采样+0.365+墙有效+高度带） | ivy_grower.rs `propose_new_segment` 386 行 | 0x14094aef9..0x14094bf76 |
| 格换算（floor×2，CLAMP 边界） | wall_spatial_index.rs `cell_index_of_world_point` | 0x14094af3c..af87（floorf 定案）；spawner 0x14094cf90 同式 |
| advance_growth（验证路径） | ivy_grower.rs `advance_growth`（保留） | 0x14094b1bb..b267 |
| send_to_gpu 48B 打包（shader 布局） | ivy_send_to_gpu_leaf.rs `IvyLeafGpu`/`pack_leaf`；ivy_send_to_gpu_segments.rs `IvySegmentGpu`/`pack_ivy_segments` | shader instanced_ivy_leaf/branch；0x1408c1580/0x140a9b4e0 fresh 反编译 |
| pruner 空间哈希 | ivy_pruner.rs `is_point_still_valid` + `ivy_pruner` 接 `Res<WallSpatialHash>` | 0x140998dc0 fresh 反编译 |
| spawner 路径 B 噪声门严格性 | ivy_spawner.rs `density <= noise.threshold → continue`（`>` 通过） | 0x14094cf90：`if noise<t \|\| noise==t → return` |
| SsboId 登记 | ivy_send_to_gpu_leaf/segments NAME 常量（全路径串） | 名表 0x142a9eb87/0x142a9da84；键 0x9e3cfa.../0x1d6de5.../0x2002d2... |

## 三、新增/修正（相对 RESTORE.md 第五版）

1. **grower 主生长闭包逐相实现**（原 GAP-2 闭合）：合并 run_guard_body/run_main_body 为
   `run_growth_pass(delta)`；守卫位门控（相 0 翻转后即运行、相 1/3/4 按位运行、相 2 计时门、相 5/6-9 panic）。
2. **每株生长延迟**：`Ivy::scalar_88` 改名 `growth_delay`（+0x88，spawn 播种），闭包中递减门控。
3. **新段提案改空间哈希格采样**：删 `nearest_wall_normal`（全墙最近墙近似），改 `cell_index_of_world_point`
   + `segments_in_cell` + `closest_segment_index(0.5)` + 0.365 接受。
4. **墙有效性与高度带**：段数≥2、墙高>0；提案点高须在 `[0, 墙高]` 且 ≤ `arch + 子型带`
   （flag 字节→+0.2；arch_type≥2→−0.46；否则 −0.79+0.2）。
5. **方向提案**：`get_growth_point_proposal` 改经 `WallCoord::into_world_space`（机器码 0x14094b7b9 调用）；
   `approx_u_from_pos` 投影；`DIRECTION_SCALE=0.21`（0x142ab3194）。
6. **48B GPU 布局**：leaf/segment 元素字段集改按 shader 定案；`seed`=frame、`arch_height`=0.0 标注待取证。
7. **pruner**：`is_point_still_valid` 改格采样 + 0.365 + 墙有效 + 插值高带；`ivy_pruner` 系统接
   `Res<WallSpatialHash>`。
8. **spawner**：噪声门 `<=`（strictly-greater 通过）对齐机器码；其余按 RE-ANALYSIS2 O 现状核验通过。

## 四、仍待取证（诚实登记，不宣布完成）

1. **方向 SIMD 精确式**（0x14094ba5d..0x14094bd34）：提案点 u/高度混合的精确 SIMD 仅部分重构——
   结构/门/常量已证，`into_world_space` 的 u 输入公式（`墙高*u_tip+lateral`）与实现
   （`u_next` 推进）有出入，登记待取证。
2. **墙字段映射**：native `[wall-0x270]`（墙高）/`[wall-0x1c]`（arch）/`[wall-0xa]`（flag）/`[wall-0x48]`（子型）
   到 crate `calculated_max_y`/`arch_height`/`source_flag_65`/`arch_type` 为重建映射（待 fresh 定案）。
3. **is_point_still_valid 附加样点**（±0.1 AABB 复检）：地形样点栅格未还原，以格查询为近似。
4. **send_to_gpu `seed`/`arch_height` 精确来源**：seed=frame、arch_height=0.0 为候选，待 leaf/segment
   闭包反编译 + HLSL 消费交叉。
5. **ivy SsboId NAME 串**：三个键（0x9e3cfa.../0x1d6de5.../0x2002d2...）非 xxh3_64(名表串)，
   产生键的 NAME 未定位（卡6 结论维持）；NAME 常量按名表串登记。
6. **spawner 路径 A 格采样**：光标根仍用全墙最近墙近似（机器码为格内 closest），维持待取证。
7. **max_ivy_grow_height 精确 getter**：pruner/spawner 用 0.5（0x3f000000）机器码值，grower 墙高用
   calculated_max_y 映射（待取证）。
8. **grower 相值写回**：机器码经 +0x10 指针写回 world+0x118（Bevy Local 槽）；本实现以 `Local<GrowerState>`
   建模，相值不改写（机器码自身也不推进相值，翻转靠守卫位）。

## 五、编译与测试

- `cargo check -p country-core --all-targets` 通过。
- `cargo test -p country-core --lib`：1725 过 / 1 败（`geometry::sphere` 他组，非 eco/ivy）。
- ivy 21 测试全过（含 `phase_cycle_keeps_advancing_growth_and_fires_completion_events`——
  新守卫位语义下 60 帧 → ~30 生长帧 ×0.075>0.99 完成，测试仍捕获旧闩死）。
- startup 32 测试全过。

## 六、reviewer 核对点

1. grower 守卫位语义（相 0 翻转→运行守卫体=生长闭包；清位→epilogue）vs 跳表 0x142ab3be0。
2. 0x142923ee0=floorf 定案（IAT thunk 20）；"RNG×2"旧读撤销。
3. 新段提案的格采样+0.365+墙有效+高度带常量（0x142ab3190/0x3f000000/0x142ab3194/0x1429cd068/
   0x142a7f250/0x142a7e320）。
4. 48B GPU 布局 vs shader instanced_ivy_leaf/branch。
5. is_point_still_valid 格采样+插值高带 vs 0x140998dc0。
6. 待取证项是否诚实、未伪造（方向 SIMD/墙字段/seed/arch_height/SsboId NAME）。
