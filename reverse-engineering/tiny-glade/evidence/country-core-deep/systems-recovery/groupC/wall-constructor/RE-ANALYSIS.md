# 组C re-analyst RE-ANALYSIS — system-wall-constructor

日期：2026-10-16。agent：re-analyst-C（dbid 会话 `tg_xc1`，小写；任务书 `tg_xC1`）。
范围：`skeleton/B/crates/systems/wall-constructor`（53 文件 / 97 业务）。
证据落盘：本目录 `decomp/`（87 文件 = 52 fresh RE-MCP 反编译 + 35 building-* 既有 .c 拷贝，均带 `// name:` 头）。

## 0. 盘点结论（sourcemap 126 业务行 / 唯一 VA 全闭合）

sourcemap（`_sourcemap-roof-wall.txt`）business 行 = 126（wall-constructor 93 + roof 33；含泛型实例多行同 VA）。
唯一业务 VA = 126。证据覆盖：**125/125 真实业务 VA**（1 项 `rhapsody::raw::device::GraphicsDevice::create_buffer` 0x141fdca60 按 DEPENDENCY_FILTERING 筛为 rhapsody 依赖，不计业务，与 reviewer 一致）。

| 类别 | 数量 | 说明 |
|---|---|---|
| groupC/decomp 落盘 | 125 | 93 wall-constructor + 32 roof（fresh RE-MCP 67 + 既有 .c 拷贝 55 + 手解 3 含 asm） |
| Ghidra NotFound 手解 | 2 | 0x14074c2d0（ArchWalker::new，17B）、0x141218e30（half_timbering，26.7KB asm 落盘） |
| 仍 U / 待定点 | 2 | `ArchWalker::new` 参数名（字段布局已证）、`half_timbering` 内部逐块（结构已解） |
| 补充批（reviewer 缺失清单 6 项） | 6 | 0x141233380（既有 .c 已拷）、0x14048ab50、0x1412269b0、0x141fe1de0、0x142418f50 fresh 补反编译 ✓、0x141fdca60（rhapsody 筛除） |

## 1. arch_walker（构造算法核心，全族 O）

### 1.1 ArchWalker 结构（0x14074c2d0 `ArchWalker::new`，17B 手解）
```
mov rax,rcx; movss [rcx+0x8],xmm1; mov dword [rcx+0xc],0x3dcccccd; mov [rcx],r8d; movss [rcx+0x4],xmm3; ret
```
- +0x00: u32（new 实参 r8d；调用点 0x14049877e 传 0；walk 未用）
- +0x04: f32（new 实参 xmm3 ← [rbp+0x3e4]；walk 未用）
- +0x08: f32 brick_len（new 实参 xmm1 ← xmm12）
- +0x0c: f32 = **0.1**（常量，非参数；尾砖阈值系数）
- 调用点：0x14049877e、0x14049eb1c（均 construct_playermade_stairs_arches_n_walls 内，栈上构造）。

### 1.2 walk 算法（4 实例同构，O 机器码差分）
实例：walk<arches> 0x141232bb0、walk<lintels> 0x141233380、walk<brackets> 0x141231ea0、walk<stairs_false> 0x14048ab50。输入面：RCX=&ArchWalker、两条点列（A/B 计数 in_stack_0x28/0x38）、in_R8=A 点列计数、closure（泛型实参）。算法：
1. `total_len = Σ||p[i+1]-p[i]||`（3D 距离，两列取 min 计数）。
2. `n = max(1, floor(total_len / brick_len + 0.5))`（0x3f000000=0.5）；`brick_len' = total_len / n`。
3. 沿曲线逐段推进 fVar5；`while (brick_len' < fVar5 + seg_len)`：`t = (brick_len' - fVar5).max(0)` → `utils::wall_space::subtract/project_clamp` 插值；**t 越界 [0,1] 或 NaN → panic_fmt（payload=f32 fmt）**；closure(t) 每砖调用。
4. 收尾：`if (brick_len' * 0.1 < fVar5)`（剩余 > 10% 砖长）→ 再发一砖。
- 依赖：utils::comparative_space::ComparativeSpace::lerp_clamp、utils::curve::curve_u::CurveU::new、glam Vec3 length/sub/dot（PDB inline 注解，模块 1450）。
- **供 restorer 的数值契约**：砖长 = 总长/整数砖数（非固定 0.5m 步进）；尾砖阈值系数 0.1。

## 2. construct_arches（0x14075f680，149KB）+ profile

- 系统主流程：读取共享队列（见 §9）→ 每段 PublicWalls 查询（packed loc 模式 0x14 行 + 0x48 掩码位集）→ `Roof::is_flat` 门控 → **随机位 bVar74 = GlobalId hash >> 0x39 & 1**（`hash_one<RandomState,GlobalId>`）→ 砖排随机化（pshuflw 展开）。
- **shape2 分支（reviewer 关注点）**：`if (*(int*)(plVar46-9) == 2) goto LAB_1407629b1` — 样式判别 ==2 时**跳过共享拱装配路径，直达 MaterialStats 记录**（set_arches_segments_count/add_arches_length/add_gate_length/set_gates_segments_count）。判别源 = 局部基 -9*8 处 int（推断为 WallStyle/wooden 枚举，**未覆盖登记：C 版本 wooden_arch_frame 分支未做原码差分，留 reviewer/后续**）。
- 调用 create_stone_arch_profile（0x1407645f0）+ `clear<WoodenPlankSsbo>`（0x1412269b0 邻域 roof_buffers）。
- `profile::create_stone_arch_profile`（0x1407645f0，O）：输入墙顶 Vec3 点列 + Option<&Vec3> + wall_state(+600 计数、+0xe0/+0xe8 切片)；`if len > 0.1875`（0x3e400000）才进：`trim = min(0.95f, 0.1875/len)`（0x3f733333/0x3e400000），部分范围时 `slice_into(1-trim)`；重采样 n 点 `t = i/(n-1)`，**断言 t∈[0,1] 且 !NaN**（Curve::get_coord_at_u 前）；x/z = lerp(p[u],p[u+1],u)，y = max_y 偏移（尾部 local_184 循环）。常量：0.1875 最小长度、0.95 最大修剪、-0.0 累加器初值（0x80000000）。
- `ArchFunction::remap_t`（0x140765300，fresh decomp 落盘）：t 重映射函数（拱高函数）。

## 3. segmentation 族（8 业务，全部 O，既有 building-* .c 复用）

| VA | 函数 | 要点 |
|---|---|---|
| 0x14074f7a0 | calc_path_wall_segmentation | puffin scope + 分段主流程（B-core .c 12.9KB） |
| 0x14074f310 | calculate_path_wall_segmentation | 分段计算主入口 |
| 0x140750420 | process_segment | 段处理 |
| 0x140751240 | add_splits_in_empty_range | 常量 1.0/0.5/1.5 + u32 转换守卫 0x5f000000/0x5f7fffff |
| 0x140751610 | score_split_candidate | 分裂候选评分 |
| 0x14074c720 | sample_wall_path_intersections | 采样交点（39.6KB） |
| 0x140751960 | split_wall_path_segment_into_platform_stairs | 平台阶梯切分 |
| 0x14122ff90 | calculate_wall_path_segments_impl | 段计算实现（26.9KB） |

## 4. construct_elevation_supports 族（13 文件 21 业务，fresh decomp 全落盘）

reviewer 要求**原码差分**：已对 circle_categorize 完成 asm↔decomp↔.rdata 三方核对（下述 ✓），其余函数 decomp 与常量已落盘，建议 reviewer 按同法抽查（circle_pillars/rectangle_pillars 常量已提取）。

### 4.1 circle_categorize（0x14124af60，O 机器码差分 ✓）
```
bool circle_categorize(&ShapeParams, _, &pillars{ptr@+8,len@+0x10}, query)
```
- `if (radius@[2] >= 1.75)`（0x142c20388=1.75）否则返回 `pillars.len == 0`。
- `t = clamp01((r - 1.75)/1.25)`（-1.75@0x142c2038c，1.25@0x142a8d600）。
- `s = smoothstep(t) = (3-2t)t²`（3.0@0x1429cd078，asm: `movss xmm12,[rip+0x1782053]→0x1429cd078; subss; mulss` ✓）。
- `threshold = 0.6*s + 0.3*(1-s)`（0.6@0x142925908，0.3@0x1429258d8，asm rip 目标核对 ✓）。
- 2r×2r 网格采样：`step = 2r / (u32)(2r)`；`func_0x000140c97b10(shape, x, y)` = 形状内点判定（每格内 fVar24++）；对每根支柱（stride 0x50，entity@+0x38）`PublicWalls::get` → `PublicWallState::get_wall_shape`(0x140b2a5d0) → `ShapeParameters::contains_point`(0x140ad2950，与 GladeBorder::is_pos_inside 同码去重)（fVar22++）。
- **返回 `fVar22/fVar24 < threshold`**（支柱覆盖占比低于阈值 ⇒ 圆形）。
- panic 源：`crates/systems/wall-constructor/src/construct_elevation_supports/circle_categorize.rs`（0x142c203e8/20400）。

### 4.2 族内其余（fresh decomp，常量提取）
- circle_pillars 0x14124ba30（1016 行）、circle_bottom_layer 0x14124b2c0 + add_bottom_brick 0x141fe1de0（与 rectangle 版同地址去重）、circle_turret 0x14124cc30（1902 行）、rectangle_bottom_layer 0x14124eb80、rectangle_pillars 0x14124fcb0 + find_stone_column_start_y 0x141250720、rectangle_small_brackets 0x141250da0、rectangle_large_brackets 0x141253fc0、util_pillar_construction 0x141fe2150/0x141fe3240、util_cross_brick_pillar 0x142418f50、maintain_stone_column_colliders 0x14078a6e0、find_overhang_support_segments 0x141248620 + OverhangSupportFinderCache::new 0x141248300、mod.rs 5 系统（overhang_subscribe_to_tabloids 0x140786440 / detect_overhang_support_segments 0x1407867d0 / construct_circle_supports 0x140788160 / construct_rectangle_columns 0x140788a80 / construct_rectangle_brackets 0x1407896b0）。
- 支柱实体在 wall 条目 +0x38，支柱数组 stride 0x50；`~id * 0x30` 反向索引模式（见 §9.2）。
- 补充批（2026-10-16 复核 reviewer 缺失清单后补）：`add_bottom_brick`（0x141fe1de0，11 参 {6 f32 + 5 ptr/int}，circle/rect 两闭包共用同一体，发底部砖）、`construct_crossbrick_pillar`（0x142418f50，15.9KB）、`generate_plaster_mesh_and_entities`（0x1412269b0，71.6KB，Map<Filter<Iter<(Aabb2,HoleType,HoleOrigin)>>> 泛型实例）、`walk<stairs closure<false>>`（0x14048ab50，8.4KB）均已 fresh 落盘。

## 5. place_pillars 族

### 5.1 place_pillars（0x140753670，52KB，O）
- 双缓冲队列 walk（§9.1）→ 每段（0x88B 元素：+0x38/+0x40/+0x50/+0x58/+0x68/+0x70/+0x78 三数组 ptr/count）支柱候选数组（stride 8 {i32,f32}）→ `if iStack_238==2` 分支调 0x141200370/0x141200380/0x141200350（wall_path_segment 访问器族，见 building-segmentation/B-owners 既有）→ FUN_141206c50/FUN_141207a10（+**0.1875** 常量 0x142a7c1d0）两变体（半砖偏移）。
- 索引越界 panic 串 `i: `（0x142c192f8，utils/curve/mod.rs）+ `assertion failed: i: < len`（0x142c19318）。
- **SSBO 注册表访问**：`FUN_141209c40(map, key)`（0x142c1a878 键 = 24B {hash,ptr,len} 静态条目；0x142c1a9a5 = "country_core::geometry::instanced_wall::BrickTransformSsbo" 58B）→ `(**(code**)(entry[1]+0x18))(entry)` 虚调取缓冲；panic 源 ssbo_library.rs（0x142c1a860）与 sparse_instance_buffer.rs（0x142c1aac0/1aaa8）。
- 尾段 Arc 标志锁：`LOCK; [p+0x18]==0→置1; UNLOCK` + `FUN_140c81c70(p+0x18)`（condvar 等待），完成 `FUN_140c81e10`（signal）；同 half_timbering/assemble_roof_tiles 的 `OnceLock/condvar` 模式 —— **rhapsody SSBO 后台上传握手**。
- 每支柱调 `assemble_stone_pillar(0x14120d670)`。

### 5.2 PillarProposals::add_proposal（0x1407534d0）+ assemble_stone_pillar（0x14120d670）
- assemble_stone_pillar 常量（.rdata 直读，O）：**0.2/0.2**（0x142a8c6f0/4 十字偏移）、**1.2**（0x1429cd128 砖长）、**0.01**（0x142960674 ε）、**0.5×3**（0x142925870/8e0/8e4）、**-0.2**（0x1429df16c）、**2.1**（0x142ae0cf0 高度限）、**±0.25**（0x142a82c10/0x142925950）、**0.56**（0x1429dd9ec）、**1.26**（0x142c1af30）、0x7fffffff（0x14292e980 abs 掩码）。
- 计算：`x+=0.2; z+=0.2`（半砖）；`if z <= 1.26`；`0.25/0.56` 除法（行距）；`fVar26 = conv(fVar28, 0.25/0.56)`；`if 2.1 < param_3` 边界。PillarProposals::add_proposal 位于 0x1407610f3 调用点（place_pillars 内 0x1407610f3 → 0x1407534d0）。

## 6. plaster_systems 族（O 结构级）

- maintain_plaster（0x14075b810，53KB）：多 ResMut 参数面（tick 回写 +0x1c ×多处：param_1、param_9、param_10）；bevy_ecs-0.16.0 event/collections.rs 断言（0x142c1c4c8）→ 消费/写 plaster 事件；puffin scope（plaster_systems.rs 串 0x142a7d0f0..150）。
- create_plaster 0x14075d460、split_wall_into_sides 0x14075dc60、generate_plaster_mesh_and_entities 0x1412269b0（Map/Filter 泛型实例）、register_required_components 0x14075e9e0。常量 0x7fffffff/0.01 同族。
- 既有 evidence：building-* 无冻结差分 —— **本族同 half_timber 处理：decomp 已落盘，O 判定待 reviewer 抽查机器码**。

## 7. construct_playermade_stairs 族

- construct_playermade_stairs_arches_n_walls<true> 0x140496800（fresh）、walk<false> 0x14048ab50、preprocess_curve 0x140780e20、brackets::construct_playermade_stair_brackets 0x1407834f0（closure$7 → walk 0x141231ea0）、magic_support::playermade_stair_floating_bricks 0x14077a2f0、pillars_visual::construct_playermade_stair_pillars_visuals 0x14077d520。
- ArchWalker::new 两调用点均在此族（0x14049877e/0x14049eb1c，xmm1=砖长、r8=0、xmm3=[rbp+0x3e4]）。

## 8. half_timbering（0x141218e30，26.7KB asm 手解；Ghidra 仅建 1B 桩，decomp 不可用）

- 范围：0x141218e30..0x14121f68f（~0x6860 B）；完整 disasm 落盘 `half_timbering-asm.txt`（5281 行）。
- 签名（construct_walls 0x140758e2f 调用点，asm 实参）：(rcx=wall entity→`PublicWallState::try_get_wall_shape`(0x140b2a600，空→return), rdx=?, r8=[rbp+0xf0], r9=&[rbp+0x2f0], 栈参 [rsp+0x20]=[rbp+0x220], +0x28=[rbp+0x2a8], +0x30=XMM0 16B(→[rbp+0x6e8]，查表键), +0x40=[rbp+0x400], +0x48=[rbp+0x3d8], +0x50=[rbp+0x2d0], +0x58=RDI(**Roof**，[rbp+0x718])) — **11 参数**。
- 体内：`Roof::gable_sides`(0x1408e33e0)→[rbp+0x208]、`Roof::is_flat`(0x1408e3380)→[rbp+0x5c4]；0x141209c40(键=[rbp+0x6e8] 16B, 0x14321a7d0) 注册表查找 + 0x3d=61 长度校验 + 0x142923bf0 串比；puffin 门控（`cmp byte [rax],0; jne 0x14121f62e`）；尾部 `FUN_140c81e10` + `LOCK; cmpxchg [rdi+0x18]` 握手；随后 0x14121f690 panic 桩（line 0x1d）。
- **构造调用图**：construct_walls → (half_timbering → 若 is_gable → roof_outline 0x141217be0)。restorer 落码时以 26.7KB 大函数为准，注意其为**逐墙网格生成**（半木构纹理）。
- 无 building-* 冻结差分 → **O 判定待 reviewer 以 asm 复核**。

## 9. 共享资源契约（跨 5+ 系统，本组最重要）

### 9.1 双缓冲队列 walk（create_roof/update_roof_spatial/generate_roof_stone_floor/place_pillars/maintain_plaster/assemble_roof_tiles 同构）
```
资源（cap-first）：
+0x00: progress u64（每消费 1 条目 +1；*res 为指针）
+0x08: VecA.cap | +0x10: VecA.ptr | +0x18: VecA.len
+0x20: VecB.cap | +0x28: VecB.ptr | +0x30: VecB.len
+0x38: 计数（cursorB 公式用：progress - [0x38] 饱和 0）
+0x40: total（入队总数）
```
- walk：`cursorA = max(0, progress-[0x38])*stride + [0x08]`；`cursorB = max(0, progress-[0x18]...`（create_roof 实测 +0x18/+0x38 交叉）。A/B 元素 stride 依系统（0x10/0x40/0x48 等）；条目 +8 = entity。
- 循环出口：`while cursorA==endA → 切 B`；`*progress += 1` 每条目。
- 同一资源 = `PublicWalls` 系（lVar5+0x1a8 0x48 步长 roof 条目 +0x38/+0x40 实体数组 + 0xa0 步长 wall 条目；lVar4+0x118/+0x120 = 双游标）。**供 restorer：统一实现一个双队列迭代器即可覆盖 6 个系统。**

### 9.2 `~id * 0x30` 反向索引（delete_roofs/update_roof_spatial 同）
`*(base + 0x10 + ~id * 0x30)` = `base + 0x10 - (id+1)*0x30` ⇒ **槽位 = u64::MAX - id**（id 为取反存储的 roof id；None=0 哨兵不相撞）。0x30 步长数组 = 每屋顶浮点/状态。

## 10. 纠错/错位登记

- **PDB 名错位**：0x1408e3250 v3 名 `Roof::ty` 实为"高度上界判定"（见 roof RE-ANALYSIS §9）；行为以机器码为准。
- **movss 常量目标异常**：0x1408e3250/3380/3390/33b0/33e0 的 `movss [rip+disp]` 目标（0x142a85984/0x142a85f84）在 RNE 原版与 B patched 字节一致、内容为字符串表；语义阈值 0.1（0x142925984 浮点表）由 gable_sides 旧 DB decomp（`_DAT_142925984`）与 roof_state.rs 恢复链双重证实。**登记为待澄清项：PDB 地址与 .rdata 布局疑有错位，后续以符号级 DB 复核。**
- **半木构/高程支撑无 building-* 冻结差分**（reviewer 提示）：本组已补 asm↔decomp 核对样例（circle_categorize 全常量 ✓），其余待 reviewer 抽查。

## 11. panic/串清单（本组新增）

| 串 | 地址 | 位置 |
|---|---|---|
| `assertion failed: i: < len`（"i: " 前缀） | 0x142c192f8 / 0x142c19318 | place_pillars（utils/curve/mod.rs） |
| `crates/systems/wall-constructor/src/place_pillars.rs` | 0x142a7c618/630/648/678 | place_pillars |
| `crates/systems/wall-constructor/src/plaster_systems.rs` | 0x142a7d0f0..150 | plaster |
| `crates/systems/wall-constructor/src/construct_elevation_supports/circle_categorize.rs` | 0x142c203e8/20400 | circle_categorize |
| `Arc counter overflow` | 0x142eab990 | assemble_roof_tiles 邻域 |
| ssbo_library.rs / sparse_instance_buffer.rs 源路径 | 0x142c1a860 / 0x142c1aac0 | place_pillars SSBO 断言 |
