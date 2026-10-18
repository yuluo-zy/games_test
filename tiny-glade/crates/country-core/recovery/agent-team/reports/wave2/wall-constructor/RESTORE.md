# wave2 wall-constructor 还原报告（restorer-wave2-wallconstructor）

日期：2026-10-17。范围：`crates/systems/wall-constructor`（package `system-wall-constructor`）。
标准：以 RE-ANALYSIS（hardpoints-interaction §3 / hardpoints-ecstate §4）的解码成果为输入，
逐语句判等；重建不了如实标未实现（硬标准，不造近似体）。

## 本次成果

### ✅ 重建（1）：`BrickRowSine`（utils/sine_wave.rs，module 401 inlinee 24627/24641/24629）

宿主 `WallConstructor::from_curve`（0x140769760）体内公式完整提取，逐语句判等
（证据 `re-analysis/hardpoints-ecstate/evidence/decomp/from_curve_140769760.c` 行 919-971/1194-1221）：

- `BrickRowSine::new(x, y, phase)`：以 random_f 装配 4 值。
- `BrickRowSine::random_f(x, y, phase) -> (r0, r1, p0, p1)`：
  - `a = sin((x*10 + phase)*12.9898)`；`r0 = frac(a*43758.547)`；
  - `b = sin((y*10 + phase)*12.9898)`；`r1 = frac(b*43758.547)`；
  - `p0 = frac(sin((r0+r0)*12.9898)*43758.547)*1000`；
  - `p1 = frac(sin((r1+r1)*12.9898)*43758.547)*1000`。
  - `frac` = `x - trunc(x)`，与 Rust `f32::fract()` 逐位一致。
  - 注：宿主 `sin` 实参为 `(r0+r0)*12.9898`（先相加再乘），非 `2*r0*A` 折叠；
    与 RE-ANALYSIS §4.3 表述 "sin(2*a)" 有折叠差异，以反编译字节为准。
  - 注：`frac` 为 `x - trunc(x)`，**符号随 x 可负**（实测 r1 可 −0.0098）；
    RE-ANALYSIS §4.3 的 "[0,1)" 表述不精确，本实现逐位对齐二进制（范围
    (−1,1)），已用独立复现校验。
- `BrickRowSine::sine_wave_deform(x, width, closed) -> (顶边, 底边)`：
  - `s = 0.5 + 0.5*sin(x*0.2)`；
  - 顶边 `= 0.06*sin((r0*s + 1)*x + p0)`，门 `0.01 <= width`；
  - 底边 `= 0.06*sin(x*(s*r1 + 1) + p1)`，`closed` 侧不取（宿主 `bVar45`）。

### 🟡 重建（纯逻辑块）：`merge_nearby_proposals`（pillars_process_proposals.rs）

`process_playermade_stair_pillars_proposals`（宿主 0x140178a70）业务块步骤 2 的
独立重建。宿主反编译（`ghidra-0x140178a70-*.c.txt` 行 373-445）逐语句：

- 44B 提议记录布局坐实：`{pos x/z @0..8, f32 @8(size_x), f32 @0xc(size_z),
  u64 @0x10, u64 @0x18, u64 @0x20, kind u8 @0x28, u16 @0x29, u8 @0x2b}`（stride 0x2c）。
- 合并：同 key 桶内 `dist(existing.pos, new.pos) < 0.01` 则合并；
  - `+8`（size_x）= `existing.is_nan() ? new : (new <= existing ? existing : new)`（max）；
  - `+0xc`（size_z）= `existing.is_nan() ? new : (existing <= new ? existing : new)`（min）；
  - 其余字段保留 existing（宿主只写 `*existing + 8/+0xc`）。
- kind 字节 `== 0x02` 终止本桶剩余处理（宿主 `if (cVar37 == '\x02') break;`）。
- 未命中则 44B 完整追加。

## 未实现登记（如实，含解码结构）

| 函数 | 地址 | 判定 | 说明 |
|---|---|---|---|
| `half_timbering` | 0x141218e30 | ❌ | pdg 前导已解码并写入 `utils/half_timber.rs` 模块文档（逐语句可判等：puffin 门控 → try_get_wall_shape 0x140b2a600 空返回 → Roof::gable_sides 0x1408e33e0 / is_flat 0x1408e3380 → SSBO 注册表 0x141209c40 查 WoodenPlankSsbo（0x3d=61 字节 memcmp）→ wall+0x180 → +0x18 LOCK 握手 → is_flat 分 `140b2a620`/`140f86cd0+141206480+140b2a680` → split_height_into_skirts 0x140b34f80 → Vec<f32> 追加 0x15 阈值扩容 → 扫描 `while (*(wall+0x280)+1.0 <= fVar110)`）；**逐梁 plank 坐标/closure 装配（函数中段 ~0x6860B）pdg 分页截断未取证**，重建不了 |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | ❌ | Ghidra fresh 97KB 已解全流程（模块文档逐段登记：SSBO FrozenMap::get+BrickTransformSsbo 0x3a 字节 memcmp → drain StairsArchSupportAssemblyParams → StairGraphEdge RawTable 探测 → preprocess_curve → closure_env_1..9 各 from_iter/walk 装配 → ArchWalker::walk<closure_env_6<false>> → raycast_simple（命中 0x11/0x01/0x02 分支）+ bilinear_sample_at_world_xz → StairsPillarProposals::add → from_curve → SparseInstanceBufferWriter::push<BrickTransformSsbo>）；**9 个 closure 单态体与 from_curve/raycast 装配未逐语句重建**，重建不了。前版近似体（walk 简化 + 44B 直落）已移除为 `todo!()` |
| `process_playermade_stair_pillars_proposals` | 宿主 0x140178a70 | ❌ | 合并块（步骤 2）已重建（见上）；**步骤 4-6 依赖 `StairColliderRegistry`/`RtWorld`/`RtMeshHandle`/`UpdateRtWorldEvent`/`StairGraphEdge(Identity)` 未落 country-core**（core-type-gap；`resources/stairs/stairs_collider.rs` 仅标记资源、`resources/rt/` 不存在）——接线待类型落地后按宿主反编译全序实现 |

## 协作注意（同 crate 并行修改）

- 同 crate 其他文件（`arch_walker.rs`/`place_pillars.rs`/`circle_categorize.rs`/
  `place_support_beams.rs`/`construct_platform_stairs/mod.rs`/`lib.rs`）存在其他
  成员的在途修改（git status M），本会话未触碰。
- 验收期间 country-core `save_game.rs` 出现过另一成员在途 E0502（行 394 borrow
  冲突），属他组任务；复跑已通过（对方已修复）。本 crate 自身检查无来自本批
  文件的告警。

## 验证

`cargo check -p system-wall-constructor` 通过（Finished dev profile；9 条警告
全部来自 pre-existing 文件，本批文件零告警）。

## 证据索引

- `re-analysis/hardpoints-interaction/evidence/ghidra-0x140178a70-*.c.txt`（47KB，pillars 宿主）
- `re-analysis/hardpoints-interaction/evidence/ghidra-0x140496800-*.c.txt`（97KB，arches_n_walls<true>）
- `re-analysis/hardpoints-interaction/evidence/r2-141218e30-half_timbering.pdg*.txt`（13KB pdg + 16KB disasm）
- `re-analysis/hardpoints-ecstate/evidence/decomp/from_curve_140769760.c`（55KB，sine_wave 宿主）
- `re-analysis/hardpoints-ecstate/evidence/decomp/r2_half_timbering_full_141218e30.c`（13.5KB，交叉验证）

## 结论

交付 1 个完整重建（`BrickRowSine`，纯数学公式逐语句判等）+ 1 个纯逻辑块重建
（`merge_nearby_proposals`，pillars 合并语义逐语句）+ 3 个如实未实现登记
（half_timbering / arches_n_walls<true> / pillars 系统，各含已解码结构与未闭合
点）。`cargo check -p system-wall-constructor` 通过。
