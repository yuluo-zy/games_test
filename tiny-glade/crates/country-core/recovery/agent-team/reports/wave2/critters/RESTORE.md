# system-critters wave2 还原报告（critters 状态跳表族落库）

- restorer：`restorer-wave2-critters`（本报告）。
- 日期：2026-10-17。
- 依据：`reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md` §1（eco/critters 状态跳表族全部攻克）
  + `evidence/`（handlers-batch1.json / handlers-butterfly.json / 5 个跳表函数反汇编 / 常量读）。
- 范围：把已解码跳表逻辑实现进 `crates/systems/critters/src` 函数体（逐语句判等，硬标准）：
  `update_wrens`(0x14231e390)、`sheep_walk_animation`(0x14231a630)、`sheep_pet_animation`(0x14231ac20)、
  `update_butterfly_states`(0x141d84d40)、`set_butterfly_velocities_towards_goals`(0x141d867b0)；
  另修 `shared.rs` 命中码语义（RE-ANALYSIS §6.2/6.3）。
- 常量：本批新读内存（ghidra read_memory，`--worker shared`，job/result 见
  `evidence/results/ghidra-wave2-consts.json`）确认 0x142a83cb8=0x3ee147ae、0x142f67d40={1.76018,1.76018,0,0}、
  0x142934674=2π、0x142f9b080=-0.7、0x142b2e410={4,2,0,0}、0x14304b3b0=(-50,5)、0x142a8d600=1.25、
  0x142934600=24.0、0x14295a0dc/0x14295a0e0=饱和常量、0x1429cd068=0.2 等；跳表实读
  0x142f68008 8 项解码（slot4-7 = vel[0-3]，基址差 0x10）。

## 交付内容（逐函数）

| 函数 | 地址 | 落库内容 | 判等状态 |
|---|---|---|---|
| sheep_walk_animation | 0x14231a630 | 跳表 0x14304b0f8 handler：state1（0x14231aa29）`accum+=dt`→`fmodf(accum,1.25)`→`idx=clamp_f32_to_i64(accum*24.0)`→`[+0x28]=(idx+n*20+3) mod 30`（n=Sheep+0x20=pet_target，除 30 魔数 0x8888888888888889）；state2（0x14231aade）同帧索引 + `progress+=dt/0.1`、`>=1.0→state3` 否则保持 2；state0/3=主循环续。修正旧 `return`（二进制 dispatch 后回主循环续下一羊）。SheepWalkRecord 补 +0x28 frame_index / +0x5c accum 字段 | 判等（handler 逐语句；音频槽写 U） |
| sheep_pet_animation | 0x14231ac20 | 跳表 0x14304b178 5 handler 与 `advance_sheep_pet` 逐语句一致（phase0 匹配门/+0x60、1 起 dt/0.3、2 保持 `!(p>1)\|\|matched`、3 收 +0x60=0 + amplitude=1-p、4 归零）；`sheep_pet_animation` 接入 `advance_sheep_pet(&mut sheep, matched, dt)` | 判等（差异：phase0 额外写 pet_progress=0，随后被覆盖，无行为差） |
| set_butterfly_velocities_towards_goals | 0x141d867b0 | 跳表 0x142f68018 4 handler：state0 清零 velocity+multiplier、state1 衰减 x/z×0.5+ y=-0xbf6147ae、state2 `1.5(0x1429258d0)>phase_ref → y=max(y, 0x3ee147ae)`、state3 y=0x3f6147ae | 判等（常量逐位 from_bits；R9/RDX 槽查询值 U） |
| update_butterfly_states | 0x141d84d40 | 跳表 0x142f68008 handler 接入 `butterfly_state_dispatch`（state1：goal+高度 0.6 门 →1/0/2，水/墙查询 U；state3：高度 1.5 →3/2 + rng 值；state4-7=vel 表转发语义核验）；`step_state` 传 goal_active/height | 部分判等（state0/2 的 `[+0x4]` 字段/xmm14/标志 U，water/wall 资源 core-type-gap；见残留表） |
| update_wrens | 0x14231e390 | per-spot 状态机（新 `WrenSpotState` 组件）：state0 timer-=dt → 到点/H1 中途；state1 音频过渡（state→2 + 高度更新）；state2/4 扫描续；state3 smoothstep 移动（t=clamp(progress/0.3)、smooth=t²(3-2t)、phase=0.5+0.5·sin(max(0,progress-0.7)·2π)、**pos.x+=vx·dir_x·dt / pos.y+=vx·dt·(dvy·dir_y+dir_z) / pos.z 不变**（REVIEW 需修 1 精确式）、progress+=dt、>6.0 重置）+ 高度更新尾部 `clamp01(height+dt·(state==0?-50:5))` + **尾部 anim_val=progress×7.0**（REVIEW 需修 2）；audio set_position 接线 | 部分判等（0x140a12650 距目标查询资源、audio call 载荷、>6.0 panic 路径 U；见残留表） |
| shared.rs 命中码 | 0x142318fa0 / 0x1423179a0 | move_critters：spherecast **双层门** `((local_fc\|\|local_100)) && ((6<loc)\|\|(99>>(loc&31)&1)==0)`（外层=Duck/Sheep 存在，以 `duck.is_some()\|\|sheep.is_some()` 近似，REVIEW 需修 2 已补）+ `hitcode!=2 且 球距<1.0 才阻塞`；critters_avoid_collisions：`hitcode==2 或 球距>=1.0 → 保留目标`，`<0.4 且 water8 → DespawnDuckCmd` 后清目标；SpherecastResult 补 hitcode 字段；mult==0 语义注释更新 | 判等（外层门以 Duck/Sheep 近似登记；water2/water8 标志映射仍以近似） |

## 残留 / 未实现（hard standard 诚实登记）

- **update_wrens**：0x140a12650 位置查询（h0 距目标判交的 dist 源）身份 U → 该分支不建模；
  h1 音频槽写/0x140a57300 call 载荷 U；`progress>6.0` 的 0x140d14770 panic/assert 路径以复位
  progress/state 承接；DEBUG debug_viz / PlacementDataSsbo（core-type-gap）。
- **update_butterfly_states**：state0 `[+0x4]` 字段、xmm8(flag) 修改、xmm14 阈值 U；state2
  `[+0x4]`/xmm14/xmm1 门 U；0x1409ea800 水检查 / 0x140a14df0 墙高查询资源缺（core-type-gap，
  state1 默认"非水无墙"）；state4-7 转发到 vel 表（0x142f68008 slot4 为有效 forwarder
  `lea r12,[0x142f68018]`，slot5-7 非对齐字节非有效 handler，实践中不触发——REVIEW 核验修正
  原"state4-7 全转发"表述）；**state3 NaN 次差**（REVIEW 补充登记：NaN height 时二进制
  `jae` 保持 3、Rust `height<1.5` NaN=false → 2；实际高度为实数不触发）。
- **shared.rs**：move_critters 外层门 `(local_fc\|\|local_100)`（Duck/Sheep 存在标志）以
  `duck.is_some()\|\|sheep.is_some()` 近似（REVIEW 补充登记）；water2(local_100)/water8(local_fc)
  查询标志→Option 组件映射仍以 `!samples_rock_mask` / `水<=0` 近似；**critters_avoid_collisions
  墙 relate 循环语义**（REVIEW 补充登记：fresh `do{...}while(relate!=0)` = 任一墙 relate==0 →
  清目标、遍历完 → 保留；Rust 为"任一 relate!=Disjoint → keep"，模块头 shared.rs:32 已登记）。

## REVIEW 复核处理（reviewer-wave2-critters-anim，REVIEW.md）

| # | 项 | 处理 |
|---|---|---|
| 1 | wren h3 移动精确式 | 已改 `pos.x+=vx·dir_x·dt`、`pos.y+=vx·dt·(dvy·dir_y+dir_z)`、`pos.z` 不变（原 y/z 分列） |
| 2 | wren h3 anim_val ×7 | 已改 `spot.anim_val = progress × 0x40e00000(7.0)`（原缺 ×7，未登记） |
| 3 | move_critters 外层门 | 已补 `(duck.is_some()\|\|sheep.is_some())` 与 loc 门叠加（原只复现内层门） |
| 4 | sheep_pet h0 文档 | 修正：二进制 h0 `mov qword [r13+0x18],1`（qword store 同时清零 +0x1c progress）——Rust phase0 写 pet_progress=0 与之匹配，无行为差 |
| 5 | sheep_walk h1 饱和 | 修正：饱和序列为 u64（fptoui）非 i64；accum∈[0,1.25) 两值一致无行为差 |

## 验证

- **独立编译验证通过**：`cargo check -p system-critters` 被上游 **country-core 并发在途编辑**
  阻断（`crates/country-core/src/render/shader.rs:104` `error.into_boxed_dyn_error()`——
  workspace 钉 anyhow `=1.0.86` 无该方法，他组在途改动；`crates/rhapsody/src/rg/attachments.rs:490`
  `self.framebuffers` 断链亦曾出现，均为他组并发，已按约束不代改）。country-core 硬依赖
  rhapsody → 全图失败。为验证本批代码，建临时独立 crate（`/tmp/critters-wave2/verify/`）以
  **country_core/utils stub 子 crate** 绕过断链，`#[path]` 引入 5 个修改文件，
  `cargo check` **0 error**（期间修出 1 个真实错误：`sheep_pet_animation` 的 `Mut<Sheep>`
  需 `mut` 绑定才能 `&mut` 传参）。
- 上游修复后应跑 `cargo check -p system-critters` 复核（预期 0 error + 既有 warning）。

## 证据链

- 解码：`reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md` §1 + `evidence/decomp/handlers-batch1.json`
  （sheep_pet/wrens handler 反汇编）、`handlers-butterfly.json`（butterfly 两表 handler 反汇编）、
  `0x14231a630_sheep_walk.asm`、`0x14231ac20_sheep_pet.asm`、`0x14231e390_update_wrens.asm`、
  `0x141d84d40_update_butterfly_states.asm`、`0x141d867b0_set_butterfly_vel.asm`。
- 常量：本批新读 `evidence/results/ghidra-wave2-consts.json`；跳表实读
  `evidence/results/ghidra-bfly-tables.json`（0x142f68008 8 项，slot4-7 = 0x141d86de5/0x141d86e3d/
  0x141d86e11/0x141d86e36，共享偏移基址差 0x10）。
- 落库文件：`crates/systems/critters/src/shared.rs`、`sheep/sheep_walk_animation.rs`、
  `sheep/sheep_pet_animation.rs`、`butterfly.rs`、`wren.rs`。
