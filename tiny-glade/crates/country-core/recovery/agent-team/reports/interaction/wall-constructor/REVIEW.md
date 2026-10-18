# wall-constructor 独立复核报告（reviewer-interaction-wallconstructor）

日期：2026-10-17。范围：`crates/systems/wall-constructor`（package `system-wall-constructor`）。
复核方式：对 restorer 的判等 6 / 重建 2 / ArchWalker 分歧 / 卡点攻克逐项 **fresh 反编译**（共享只读 ghidra worker）与 **RNE 机器码 + 权威 B 内存**（radare2 独立 worker + ghidra read_memory），独立复述原函数控制流后再与 crate 落盘 Rust 逐语句对照。

## 0. 复核证据（本会话 fresh 落盘）

| 证据 | 内容 |
|---|---|
| `/tmp/wc-review/dec/141240d30_perturb_row_ends.c`、`141240ec0_split_horizontally.c`、`14123fff0_from_center_dims.c` | perturb_row_ends / split_horizontally fresh decomp |
| `/tmp/wc-review/dec/141232bb0_walk<arches>.c`、`141231ea0_walk<brackets>.c`、`141233380_walk<lintels>.c`、`14048ab50_walk<arches_n_walls>.c` | ArchWalker::walk 4 单态 fresh decomp |
| `/tmp/wc-review/dec/1412411c0_resolve_hole_overlap.c` | resolve_hole_overlap fresh decomp（667 行） |
| `/tmp/wc-review/dec/14123eef0_trim_rows.c`、`141fe0750_split_heightmap_by_threshold.c`、`14123fab0_above_to_below.c`、`14123fb70_below_to_above.c`、`14123e110_split_into_random_rows.c` | 行工具 fresh decomp |
| `/tmp/wc-review/dec/14074c2d0_new.c`、`14003bb80_finish<MainCameraPlugin>.c`、`141250720_find_stone_column_start_y.c`、`141218e30_half_timbering.c`（0B 空）、`141fe09c0_propose.c`、`141214ed0_add_quad.c`、`140786440_subscribe.c` | 其余抽查 fresh decomp |
| radare2：0x14003bb80、0x1412400c0/2e0/2f0、0x141250720 尾段；**ghidra read_memory 0x142c20af8 80B（权威 B patched EXE）** | 机器码/跳表裁决 |

## 1. 判等 6 独立复核

### 1.1 debug_force_update_arches（0x14003bb80）— **判等成立**（机器码证实）
RNE hexdump `c3 cc cc…`（ret+int3）；PDB 符号首条位置 RVA 0x3bb80=VA 0x14003bb80（skeleton B lib.rs L3857）。crate 空函数体 `pub fn debug_force_update_arches(){}` == ret 存根。**FIX④已处理**：crate lib.rs 注释已统一为"1B ret 存根（C3+0xCC 填充，无真实业务）；空函数体与之判等"——**fix_verified**。

### 1.2 resolve_hole_overlap（0x1412411c0）— **判等（G04 320 例复用）＋残留分歧 1 处（方向修正）**
- 独立复述（fresh 667 行）：入口 gate `width>0 && height>0 && deque.len≠0`；逐 40B MyBrick，四角 contains 计数 → switch：
  - **case0**：外层守卫 `if (cStack_193 == 0)`——注意 cStack_193 = (uVar9>>0x28) = brick 偏移 0x25 字节 = **split_x**，cStack_194 = (uVar9>>0x20) = 偏移 0x24 字节 = **split_y**（前版复核曾将二者互换，此处修正）。`split_x==0` 下：`!Xoverlap`（`max_x≤hole.max_x || hole.min_x≤min_x`）→ 嵌套 `!Yoverlap || split_y!=0` 则 skip、否则**纵切**（split_y=1）；X 重叠 → **横切**（split_x=1）。deformation=[0,0,d2*0.5,d3*0.5]、副 seed+100、主替换/副 push_back。
  - **case1**：inverse_lerp+clamp01，`full.seed=(iVar34+seed)%10`、`small.seed=(seed+1)%10`，push_front×2+swap_remove_back(index+2)。
  - **case2**：按四角模式 overlap 裁 aabb、零化 deformation 分量、seed%10；**case4**：swap_remove_back(index)；**default(3)**：panic。
- 对照：country-core `resolve_hole_overlap` 逐 case 一致（case0 横/纵切、seed+100、case1 %10+push_front+swap_remove_back、case2 overlap、case4 swap_remove_back）。
- **残留分歧（修正后）**：原版 case0 以 **split_x==0 为外层守卫**（横切、纵切均须 split_x==0）；country-core 仅横切查 split_x==0，**纵切分支无 split_x 守卫**（`else if b.split_y==0 && Yoverlap`）。当 `split_x==1 && split_y==0 && Yoverlap && !Xoverlap`（横切后某半仍被洞整段纵向覆盖、洞横跨切分线）时：原版 skip，country-core 纵切。G04 320 例未覆盖此输入。**仍登记 FIX⑤：需补定向冻结用例**（方向与首版复核相反，特此修正）。

### 1.3 trim_rows（0x14123eef0）/ above_to_below（0x14123fab0）/ below_to_above（0x14123fb70）— **判等（复用，G04 200 例）**
trim_rows fresh 主为 puffin 探针遮蔽，实质 = 逐 boundary 窗口调 `split_heightmap_by_threshold<RowTrimmerSink>`（0x141fe0750），输出 Vec len=0/cap=8。split_heightmap fresh：`threshold<heights[0]` 时 push/改写末行；逐 i：`prev≤thr<thr<curr`→below_to_above（上行穿越，push {0,0,id,lerp(u),1.0}）；`prev>thr && curr≤thr`→above_to_below（下行穿越，写末行 u[1]）。below_to_above fresh：断言 t∈[0,1]+行号界、lerp、push 32B{0,0,id,u,1.0}；above_to_below fresh：断言+lerp+写末行末 f32。country-core `trim_rows` crossing 逻辑逐语句对应；WallRow 32B 布局一致。**判等成立**。注：above_to_below/below_to_above 为 sink 回调，经 trim_rows 复用覆盖属正当复用。

### 1.4 split_into_random_rows（0x14123e110）— **判等（复用，G04 160 例）**
fresh：windows(2)、`segments=floor(span/0.46)` 下限 2、random_splits_into、`low=h0/scale, high=h1/scale`、previous=-100、`(prev-val).abs()<0.046/scale` 去近边。country-core 逐语句一致。**判等成立**。

## 2. 重建 2 独立复核（均判等）

### 2.1 perturb_row_ends（0x141240d30）— 判等
fresh + Aabb2::center 机器码（`movss[rcx];addss[rcx+8];mulss 0.5`→center.x=(a0+a2)*0.5）：前向 `t=1.0−0.5*(rows[next]+rows[next−1])`、`v=NaN感知min(center,t)*1.99(0x3ffeb852)`、`min(amplitude,v)`、`aabb[0]=v*mag+aabb[0]`；后向 `next−1!=end→return`、`mid=0.5*(rows[0]+rows[1])`、`t=1.0−center`、同两阶钳制、`aabb[2]`。crate 逐语句等价。

### 2.2 split_horizontally（0x141240ec0）— 判等
fresh + Aabb2 width=`[2]−[0]`/height=`[3]−[1]` 机器码：`t=split*0.4+0.3`；`new_x=t*max_x+(1−t)*min_x`、`new_y=t*max_y+(1−t)*min_y`；lower aabb={min_x,min_y,max_x,new_y} deformation={min,new} seed 原 split=0；upper aabb={min_x,new_y,max_x,max_y} deformation={new,max} seed+1 split=0；40B 布局（seed@0x20、split_y@0x24、split_x@0x25）与 crate 一致。crate 逐语句等价。

## 3. ArchWalker::walk 分歧确认（未实现-需重做成立）

0x141232bb0 fresh：total=Σ‖Δ‖（min 计数）、`n=max(1,floor(total/brick_len+0.5))`、`step=total/n`；逐段 `fVar6=travelled+dist`，`if step<fVar6` 进内环：`traversed=(step−travelled).max(0)`；u 经 `wall_space::subtract()+project_clamp()` 周期投影后 `panic_fmt/panic`；`travelled=(travelled−step).max(0)`；发砖；`while step<(dist−traversed)+travelled`；尾砖 `step*0.1<fVar5`（循环末 travelled）。crate `walk()`：`travelled=step*emitted`、normalize/dir 直插值、`while step>=travelled+seg*t`、尾砖 `step*0.1<total−travelled`——**4 处分歧全部为实**。0x141233380/0x141231ea0/0x14048ab50 fresh 均含 wall_space subtract+project_clamp 同构。**未实现-需重做**成立。ArchWalker::new（0x14074c2d0）fresh `[0]=in_R8D(u32)、[4]=in_XMM3(f32)、[8]=in_XMM1(f32)、[12]=0.1` 与 crate `new(kind,extra,brick_len)` 布局一致，字段语义未定（🟡 合理）。

## 4. 卡点裁决：find_stone_column_start_y 跳表（fix_verified，restorer 修正解码为权威）

- Ghidra fresh 截断确认（"Bad instruction"/"Sanity check requires truncation of jumptable"）。表基址 0x142c20af8，`cmp byte,0x11` 前特判 0x11，17 项（0..0x10），0x17 后为数据。
- **权威字节**（ghidra read_memory 0x142c20af8 80B，B patched EXE，hex）：`36fd62fe 49fe62fe 35fe62fe 57fe62fe 57fe62fe 57fe62fe 36fd62fe 36fd62fe 36fd62fe 36fd62fe 36fd62fe 36fd62fe 57fe62fe 3ffe62fe 36fd62fe 57fe62fe 57fe62fe 90c2753d 9a9999bd 00000000`。
- 逐项解码（`movsxd rax,[rcx+rax*4]; add rax,rcx`；目标 0x14125082e=命中续、0x14125092d/0x141250937=常量路径、0x141250941=返0、0x14125094f=panic unreachable）：
  - idx0=0xFE62FD36→0x14125082e（续）；idx1=0xFE62FE49→0x141250941（返0）；idx2=0xFE62FE35→0x14125092d（常量 test）；idx3/4/5=0xFE62FE57→0x14125094f（panic）；idx6/7/8/9/10/11=0xFE62FD36→续；idx12=0xFE62FE57→panic；idx13=0xFE62FE3F→0x141250937（常量直入 add）；idx14=0xFE62FD36→续；idx15/16=0xFE62FE57→panic。
- **权威分类**：续 **{0,6,7,8,9,10,11,14}**；常量 **{2,13}**；返0 **{1}**；panic **{3,4,5,12,15,16}**。此与 restorer 修正后主张**完全一致**。
- **裁决说明**：首版复核曾主张续{0,7-12,15}/常量{2,14}/panic{3,4,5,6,13,16}，系首轮 radare2 hexdump 在 idx6 出现单 dword 读差 + 复算时 hex 串转录错位（idx12-15）所致，**以本版权威内存字节为准**。RESTORE.md 首版分组（0x06-0x0D/0x10 续、0x02/0x0F 常量、0x03-0x05/0x0E panic）经复核确有 5 项错（12/13/14/15/16），但 restorer 修正解码已正确，**fix_verified**。

## 5. 过度断言 / 需降级项（复核后状态）

1. **支撑梁 4 函数 + construct_platform_stairs 原标 🟡 实为壳**——**FIX②已处理，fix_verified**：crate `place_support_beams`（0x141242530）、`add_diagonal_support`（0x141243740）、`emit_support_beams`（0x141243c20）、`SupportBeamGrower::propose`（0x141fe09c0）、`construct_platform_stairs`（0x1407744d0）现均 `todo!("未实现: …")`。propose fresh 真实逻辑：`flags[next]&0xf→stopped(tag)`、3D 归一方向、`total=len+acc`、`≤3.15` 守卫、`cost=total*(‖Δdir‖+.5)`（Δdir 按 flags bit31 选择新旧方向）、写 index/xz_len/方向/cost/point{x,y,z}/tag；crate 无一实现。**正确降级**。
2. **DoorMeshInProgress::add_quad（0x141214ed0）**——**FIX③已处理，fix_verified**：crate 无独立符号，经 country-core gates `construct_gate_cpu`（G06 冻结）行为层覆盖；结构差异保留登记（原版 3 并行数组 positions/colors(DecoratorId,DecoratorId,u8)/uvs，country-core 4 数组含 normals）。不作 🟡 重建。
3. **debug_force_update_arches 注释矛盾**——**FIX④已处理，fix_verified**（见 §1.1）。
4. **resolve_hole_overlap case0 split_x 守卫**——**仍 FIX（方向修正）**（见 §1.2）：原版以 split_x==0 为外层守卫（横/纵切均查），country-core 纵切分支缺 split_x 守卫。

## 6. 其余未实现项抽验（与 crate 状态一致）

- **half_timbering（0x141218e30）**：fresh 反编译 0 字节，空反编译卡点属实。
- **circle_categorize / stairs_signifiers_new / add_wall_corners / add_skirts / overlapping_windows / get_window_aabb / check_for_overlap / PillarProposals::add_proposal / assemble_stone_pillar / sine_wave**：crate 已逐一切 `todo!("未实现: …")`（共 10 处），诚实降级。
- **segmentation 三函数**：crate `process_segment`（flag≠0 推 [0.0;4]）、`add_splits_in_empty_range`（ceil(len/1.5)）、`score_split_candidate`（(mid−0.5).abs()）为近似体，与 RESTORE.md 登记分歧点逐字对应，❌ 成立。
- `cargo check -p system-wall-constructor` 通过（9 警告，本会话实测）。

## 7. 胶水复核

- `overhang_subscribe_to_tabloids`（0x140786440）：fresh 反复 `RaycastWorld::subscribe_to_changes()`（8 次）+ 1×`subscribe<DirtyTiles>`，纯订阅 push。**glue-legitimate**。
- `add_wall_constructor_systems`（注册胶水）、`SupportBeamGrower::new`/`check_for_hole_overlap`/`normalize_holes`（内联/复用）：**glue-legitimate**。

## 8. Verdict 汇总

- **判等 8**：debug_force_update_arches、resolve_hole_overlap（带 §1.2 残留）、trim_rows、above_to_below、below_to_above、split_into_random_rows、perturb_row_ends、split_horizontally。
- **胶水 5**：overhang_subscribe_to_tabloids、SupportBeamGrower::new、check_for_hole_overlap、add_wall_constructor_systems、normalize_holes。
- **未实现约 60**：ArchWalker::walk 族（4 分歧已证）、支撑梁 5 函数（已降 todo）、segmentation 3 函数、立面支撑整族、门/栅栏/地板/楼梯族、半木/抹灰族、UI 族、utils 近似体族、find_stone_column_start_y 等。crate 内全部以 todo!/壳/近似标注，无冒充实实现。

## 9. FIX 状态

| # | 项 | 状态 |
|---|---|---|
| ① | find_stone_column_start_y 跳表分组 | **fix_verified**——restorer 修正解码（续{0,6,7,8,9,10,11,14}、常量{2,13}、返0{1}、panic{3,4,5,12,15,16}）与权威 B 内存逐字节一致；本报告首版解码已更正为权威版 |
| ② | 支撑梁 4 函数 + construct_platform_stairs 降未实现 | **fix_verified**——crate 已改 todo!() |
| ③ | add_quad 改标 G06 复用覆盖 | **fix_verified**——不作 🟡 重建，结构差异登记 |
| ④ | debug_force_update_arches 注释统一 1B ret | **fix_verified**——lib.rs 已改 |
| ⑤ | resolve_hole_overlap split_x 守卫残留 | **仍有 FIX**——方向修正为：原版横/纵切均查 split_x==0，country-core 纵切缺守卫；补定向冻结用例后关闭 |
