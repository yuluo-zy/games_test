# RE-ANALYSIS — eco/state 反编译卡点与残留补证

- 角色：`re-analyst-hardpoints-ecstate`（re-analyst）
- 日期：2026-10-17
- 依据总账：`recovery/agent-team/planning/2026-10-17-unimplemented-ledger.md` §4 decompile-hardpoint + §总表 re-analysis 条目
- 范围：eco（glade-border/forest/tree-tool/critters）与 state（water/timberframe/roof/color/decorator）模块的卡点，另含任务点名项（half_timbering、color_menu/show_color_options、playermade_stairs_assemble、calc_gothic_half_dormer_profile_ws、populate_autoclutter_regions、sine_wave inlinee、add_autoclutter_on_windows）
- 方法：Ghidra shared 只读批量 → ReVa write worker（专属 clone，长超时定点反编译）→ radare2 全函数反汇编/跳表 .rdata 手工解码 → read_memory 常量
- 只读后端复用；ReVa 写 worker `hdpts-ecstate` 仅用于脚本定点反编译（不改库，fresh clone）

---

## 摘要

| 函数 | 地址 | 分类（原） | 结果 |
|---|---|---|---|
| `update_wrens` | 0x14231e390 | decompile-hardpoint | 攻克（跳表 0x14304b638 解码 + 5 handler 反汇编 + 常量） |
| `sheep_walk_animation` | 0x14231a630 | decompile-hardpoint | 攻克（跳表 0x14304b0f8 解码 + 2 handler 逻辑） |
| `sheep_pet_animation` | 0x14231ac20 | decompile-hardpoint | 攻克（跳表 0x14304b178 解码 + 5 状态 handler + 参数 0.3/1.0/0.2） |
| `update_butterfly_states` | 0x141d84d40 | decompile-hardpoint | 攻克（跳表 0x142f68008 解码 + 状态 4→vel 表转发） |
| `set_butterfly_velocities_towards_goals` | 0x141d867b0 | decompile-hardpoint | 攻克（跳表 0x142f68018 解码 + 4 handler） |
| `chimney_smoke_anim_thickness` | 0x140818df0 | decompile-hardpoint | 攻克（ReVa write 全量 31KB；divisor 公式还原） |
| `color_menu` | 0x142291230 | decompile-hardpoint | 攻克（ReVa write 全量 136KB；Ghidra codelen=0 三路失败根因=shared DB 函数边界坏） |
| `show_color_options` | 0x14229a7f0 | decompile-hardpoint | 攻克（ReVa write 全量 267KB；同上根因） |
| `half_timbering` | 0x141218e30 | decompile-hardpoint | 攻克（radare2 `aa` 后 pdg 可反编译；Ghidra/ReVa "Unable to find unique hash for varnode"） |
| `populate_autoclutter_regions` | 0x1422d6ed0 | decompile-hardpoint | 攻克（Ghidra 已可全量 120KB；旧"44KB 碎片"实为跳表分派已恢复） |
| `add_autoclutter_on_windows` | 0x1422cd7d0 | decompile-hardpoint | 攻克（Ghidra bad instruction 截断；radare2 全反汇编 + 跳表 0x143042370 解码） |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | decompile-hardpoint | 攻克（Ghidra 全量 97KB；`<false>` closure 表已定位） |
| `sine_wave`（BrickRowSine inlinee）| module 401 inlinee | decompile-hardpoint | 攻克（宿主 from_curve 0x140769760 体内提取，随机+变形公式完整） |
| `calc_gothic_half_dormer_profile_ws` | 0x1407eb160 | （任务点名） | 攻克（Ghidra 全量；profile curve 3 档常量表解码） |
| `manage_tree_prefabs` has_construct_shape 门控 | 0x141bf16c0 | re-analysis | 补证（门控位 = +0x18 字节，SSBO 处理状态，非事件存在性） |
| `move_critters` 残留 | 0x142318fa0 | re-analysis | 补证（spherecast hitcode 2/3 语义 + mult==0 墙路径） |
| `critters_avoid_collisions` 残留 | 0x1423179a0 | re-analysis | 补证（hitcode==2 保留路径来源） |

---

## 1. eco/critters —— 状态跳表族（全部攻克）

### 1.1 `update_wrens`（0x14231e390）—— 状态跳表 0x14304b638

**失败原因**：Ghidra 主体可反编译但跳表 handler 未取证（`Could not recover jumptable`，间接跳转按 call 处理）。

**攻克方法**：
- Ghidra `read_memory` 读 0x14304b638 跳表（相对偏移 int32，基址=表地址）
- 表内 5 个有效项（第 6 项起为 /rustc 字符串，判定表长=5）
- radare2 对 5 个 handler 逐一定点反汇编 + Ghidra 常量 read_memory

**跳表解码**（`*(code *)(&DAT_14304b638 + *(int *)(&DAT_14304b638 + state*4))`，state=`*(uint*)(spot+0x30)`，spot stride 0x50）：

| index | 目标 | handler |
|---|---|---|
| 0 | 0x14231ebf8 | h0：timer `[spot+0x34] -= dt`；timer 到 0 → 跳 h1；未到 0 → 距 spot 判交 |
| 1 | 0x14231e930 | h1：写音频槽 `[rsp+0x70]`、写 `[spot+0x44]`、设 `[spot+0x30]=2`（state→2）、`call 0x140a57300`（音频 set_position） |
| 2 | 0x14231e9a9 | h2：主循环续（实体扫描/状态保持） |
| 3 | 0x14231ec82 | h3：smoothstep 位置移动（`t=clamp([spot+0x40]/0.3,0,1)`；`s=t*t*(3-2t)` 变体；`lerp` + 速度 `*[0x142b2e410]=(4,2,0,0)`；pos+=方向*dt；timer `[spot+0x40]+=dt`；>1 阈值 `[0x142934690]=6.0` → 重置） |
| 4 | 0x14231edf4 | h4：嵌套实体扫描（另一个实体列表遍历） |

**handler 常量**（已读内存）：
- `[0x142925870]=0.5`、`[0x14304b390]=(-0.5,0,0,0.5)`（XMM7 初始）
- `[0x142925904]=1.0`、`[0x1429cd078]=3.0`（smoothstep 用）
- `[0x14292e6b0]=(1,1,0,0)`、`[0x14304b3a0]=(10,0.6)`、`[0x142a7cd48]=7.0`、`[0x142934690]=6.0`、`[0x1429258d8]=0.3`、`[0x142934674]=2π`、`[0x142f9b080]=-0.7`
- 表 `[0x14304b3b0]=(-50,5)`（h1 尾部：`state==0 ? dt*-50 : dt*5` 作高度增量，clamp01）

**逻辑结论**：per-spot 状态机（0x50 记录 +0x30 状态字节）。state0=计时/等待；到点→state1（音频+高度+state→2）；state3=朝 spot 移动（smoothstep）；state2/4=扫描续。

**剩余未知**：handler 内实体定位的具体资源指针身份（0x100/0x1a8 表）；音频 call 0x140a57300 载荷。

### 1.2 `sheep_walk_animation`（0x14231a630）—— handler 跳表 0x14304b0f8

**跳表解码**（index=`*(uint*)(spot+0x10)`=state）：

| index | 目标 |
|---|---|
| 0 | 0x14231a729（循环继续=无动画 handler） |
| 1 | 0x14231aa29（walk anim handler） |
| 2 | 0x14231aade（transition handler） |
| 3 | 0x14231a729（同 0，无 handler） |

**主逻辑**（Ghidra 主体已全量反编译，机器码核对）：`speed=|Δpos|/dt` 存 `[spot+0x48]`；`speed<=0.2 && state==1 → state=2, progress=0`；`speed>0.2 → state=1`（state==1 时保持 progress）；写 `[spot+0x10]=state`、`[spot+0x14]=progress`、写音频槽；`being_petted()` 非 petted 才跳转 handler。

**handler 0x14231aa29（state1）**（radare2 全反汇编）：
- `[spot+0x5c] += dt`（accum 计时）
- `x = fmodf(accum, 1.25)`（call 0x142923f10=fmodf）
- `idx = clamp_f32_to_i64(x * 24.0)`（0x142934600=24.0；`cvttss2si` 饱和转换序列 0x5f000000/0x5f7fffff）
- `[spot+0x28] = (idx + n*20 + 3) mod 30`（`n=[spot+0x20]`；`0x8888888888888889`=除 30 魔数 → `(idx+n*20+3) - 30*floor((idx+n*20+3)/30)` 即 mod 30）

**handler 0x14231aade（state2）**：
- `[spot+0x5c] += dt`；同上 fmodf/idx
- `progress += dt / 0.1`（0x142925984=0.1）
- `progress < 1.0`（0x142925904=1.0）→ 保持 state2、写 progress；否则 `state=3`（无 handler，回主循环）

**逻辑结论**：sheep walk 状态机 = {0:idle,1:walk,2:走完过渡,3:静止}。speed 阈值 0.2 驱动 1↔2；state1 走 fmod 动画帧索引 mod 30；state2 在 dt/0.1 秒后→3。

### 1.3 `sheep_pet_animation`（0x14231ac20）—— handler 跳表 0x14304b178

**跳表解码**（index=`*(uint*)(spot+0x18)`=state，5 项）：

| index | 目标 | 语义 |
|---|---|---|
| 0 | 0x14231afe4 | state0：若 r11b（事件实体匹配）→ `[spot+0x60]=1`、state→1、写音频槽 |
| 1 | 0x14231b0b3 | state1：`progress += dt/0.3`；`>1.0 → state=2, progress=0`；否则保持；`[spot+0x58]=progress` |
| 2 | 0x14231b022 | state2：`progress += dt/0.2`；`<=1.0 或 匹配 → 保持 state2`；否则 state→3、progress=0；`[spot+0x58]=1.0` |
| 3 | 0x14231b068 | state3：`progress += dt/0.3`；`<=1.0 → 保持`；否则 state→4；`[spot+0x60]=0`；`[spot+0x58]=1.0-progress` |
| 4 | 0x14231b006 | state4：state→0、`[spot+0x58]=0` |

**参数**：两次分派调用常量 `(0x3e99999a, 0x3f800000, 0x3e4ccccd)` = (0.3, 1.0, 0.2)。已证 XMM0/XMM1/XMM2 = dt 除数/阈值/除数（XMM0=0.3 由 [0x1429258d8]、XMM1=1.0 由 [0x142925904]、XMM2=0.2 由 [0x1429cd068] 装入）。

**逻辑结论**：pet 动画状态机 {0:idle,1,2,3,4}，事件匹配（首羊）从 state0 启动；3 个阶段时长 0.3/0.2/0.3（progress 用 dt 除以对应常量）；[spot+0x58] 存进度/1-进度（供视觉），[spot+0x60] 标志字节 pet 中置 1。

### 1.4 `update_butterfly_states`（0x141d84d40）—— 状态跳表 0x142f68008

**跳表解码**（index=`*puVar2`=butterfly state，8 项）：

| index | 目标 | 语义 |
|---|---|---|
| 0 | 0x141d85983 | 距目标处理 + phase 递减 + 状态迁移 |
| 1 | 0x141d85a22 | state1：`goal` 条件 + `0x1409ea800`（水检查）+ 墙高度 + 阈值 `[0x142925908]=0.6` |
| 2 | 0x141d85ada | state2：`[spot+4] -= [0x1429258d8]=0.3` 门；`5.0`（0x142925988）相关 |
| 3 | 0x141d85b08 | state3：距 1.5（0x1429258d0）判断 → 飞行迁移 |
| 4 | 0x141d86dd5 | **状态 4+ 转发**：`lea r12,[0x142f68018]; jmp [r12 + rsi*4]`（按 vel 表再分派） |
| 5 | 0x141d86e29 | （同区域字节，指向 vel 表区） |
| 6 | 0x141d86df9 | （同区域字节，指向 vel 表区） |
| 7 | 0x141d86e1a | （同区域字节，指向 vel 表区） |

**关键发现**：0x142f68008 表第 4-7 项与 0x142f68018（vel 表）共享同一相对偏移序列但基址差 0x10 → 状态 4 进入"速度/动画"子状态机（转发到 vel 表）。这解释了此前"handler 语义 U"。

### 1.5 `set_butterfly_velocities_towards_goals`（0x141d867b0）—— 动画跳表 0x142f68018

**跳表解码**（index=`*puVar1`，4 项）：

| index | 目标 | 语义 |
|---|---|---|
| 0 | 0x141d86de5 | state0：清零 target/velocity：`[rbx+8]=0; [rbx]=0; [rdi]=0; [rdx]=[rsp+0x4c]` |
| 1 | 0x141d86e39 | state1：`[rbx] *= xmm3; [rbx+8] *= xmm3`（衰减）；`[rbx+4]=0xbf6147ae`（-0.8856 y） |
| 2 | 0x141d86e09 | state2：`xmm4 > [r15+0x14]` 门；`[rbx+4] = max([rbx+4], xmm5)` |
| 3 | 0x141d86e2a | state3：`[rbx+4] = 0x3f6147ae`（0.8856 y） |

**逻辑结论**：velocity 动画子状态机 4 态：0=静止清零、1=速度衰减+向下、2=速度钳制取大、3=固定向上分量。xmm3=每帧衰减因子（调用点常量 0x3f000000=0.5）。

---

## 2. state/color —— 三路失败函数（ReVa write worker 攻克）

### 2.1 `color_menu`（0x142291230）

**失败原因（坐实）**：Ghidra shared DB `decompiled_code len=0`（函数边界/指令 overlap 分析损坏，codelen=0）；ReVa shared `get-decompilation` 10s 超时（大函数+无 P-code 预热）；radare2 pdg "No function at this offset"（无 PDB TPI 符号）。

**攻克方法**：**ReVa 专属写 worker（`hdpts-ecstate`，从原 seed 独立 APFS clone，fresh 分析）** + `run-script` 内 `DecompInterface.decompileFunction(f, 600, monitor)` 长超时 → **成功产出 136KB 全量反编译**（`color_menu_142291230_full.c`）。关键：shared DB 的 codelen=0 是 overlay 损坏，fresh clone 无此问题。

**逻辑结论**（从 136KB 全量提取）：
- 系统签名：Bevy system（RCX/RDX/R8/R9 + stack 多参数）
- 主循环按 `local_100[0x30]`（menu state byte）switch：case 0..0x15
- case 0/2/4/8/0xd/0x10/0x13/0x14 等走不同分支；大量 `SettingsMenuState::just_pressed`、`DigitalActionState::just_pressed`、`WidgetParams::new`、`UiState::add`、`History::add`
- 写 `DecoratorId`、`WindowFocusEvent`、`PublicWalls::get`、`History::try_start_input_recording`
- UI 交互：`UiWidgetContext::is_active/clicked`、`UiSounds::on_hover_start/on_click`、`ColorMenuBrightnessValue::to_f32`

**剩余未知**：case 全枚举对应 UI 组件身份需 restorer 结合 toolbar/mei 侧落库后对齐（属 core-type-gap 面）。

### 2.2 `show_color_options`（0x14229a7f0）

**失败原因**：同 color_menu 三路失败；Ghidra shared 仅 687 字节函数体（真实函数体 0x14229a7f0..0x1422a0e18，`read_memory`/disasm 证实）。

**攻克方法**：ReVa write worker + 600s 超时 → **267KB 全量反编译**（`show_color_options_14229a7f0_full.c`）。

**逻辑结论**：
- 函数名实为 `system_color::show_color_menu::show_color_options`
- 遍历选项 Vec（stride 0x20），按 `*puVar19`（kind byte）switch 0..0x26（约 39 case）
- case0：构建 plaster/wall/roof 颜色选项（`plaster_colors`/`sk_beige`/`sk_ylw`/`sk_hglt`/`sk_cgrad` 串确认）+ `PublicWalls::get` + 多 `__rust_alloc`
- 后端选项分类：`DecoratorStorage::lookup_decorator_info`、`DecoratorExtraInfo::try_as_stair/as_flag/as_chimney/as_window/as_lantern/as_stair`、`MainCamera::eye_position`、`StairsState::get_all_connected_nodes`、`Roof::is_flat`
- 大 switch 第二段按 `(*pauVar26)[0]` 再分派（嵌套 kind）

**剩余未知**：39 case 精确映射（UI 组件/图标目录）需 restorer 对齐 CustomizationIcon/Ui2dExtent/Ui2dImage 类型（core-type-gap）。

---

## 3. state/decorator —— chimney_smoke_anim_thickness（0x140818df0）

**失败原因**：更新公式 divisor 反编译打散（CONCAT13 字节重拼）。

**攻克方法**：ReVa write worker 全量（31KB，`reva_chimney_smoke_140818df0.c`）+ Ghidra shared 已有 30KB 主体交叉核对。

**逻辑结论**（公式还原）：
- 每 spot（0x28 记录）：`fVar59 = (plVar42[0xd]==0) ? lerp(1.0,3.0,rng) : lerp(2.0,4.0,rng)`（即 rng*3+(1-rng) 或 rng*4+(1-rng)*2）
- **更新公式**：`new = clamp01(old + fVar59 * dt / CONCAT13(divisor))`，其中 `CONCAT13(uVar54..uVar57)` 为刚才 `fVar59` 的字节重拼 → **除数即目标值本身** → 实际为 `new = clamp01(old + dt / target)`（目标大→增速慢）
- 尾部 `HashMap<GlobalId,f32>::retain(closure_env_0)` 过滤
- 常量确认：0x1429258d8=0.3、0x1429258d0=1.5、0x142925908=0.6、0x142925988=5.0、0x1429345a8=10.0、0x142960674=0.01、0x142a7b658=130.0

**剩余未知**：target 的 f32 直接用于 divisor 的语义（作为"烟雾厚度目标值"，restorer 按 `thickness += dt/target` 实现即可对字节级判等）。

---

## 4. interaction/wall-constructor —— 任务点名卡点

### 4.1 `half_timbering`（0x141218e30）

**失败原因**：Ghidra/ReVa `void()` 占位 + **"Unable to find unique hash for varnode"**（函数 27KB/5494 指令，特定 varnode 无法 hash）；radare2 pdg 无 PDB 符号 "No function at this offset"。

**攻克方法**：**radare2 `aa`（analyze level 2）后 `decompile_function` → 成功反编译**（`r2_half_timbering_full_141218e30.c`，13.5KB）。Ghidra 侧还提供了全函数 5494 条指令反汇编（`ghidra-disasm-empties`）+ 110 个 call 目标清单（已存 evidence）。

**call 目标身份**（Ghidra 查签名）：`center`、`width`、`max_y`、`WoodenPlankSsbo::push`、`inverse_lerp`、`to_world_space`、`x0y`、`xz_rotation_bad`、`fmodf`、`__rust_alloc/dealloc`、`panic_bounds_check` 等 → 属 wall-constructor 半木梁生成（板条 push + 曲线坐标变换）。

**逻辑结论**：pdg 可反编译但保留 `fcn.` 匿名名（radare2 无 PDB 类型），函数边界 `[[141218e30,14121920c],...]` 已核。业务=半木（timberframe）填充梁的生成循环：curve 上每段按 0.5 间距放置梁、max_y 高度取整、WoodenPlankSsbo 写缓冲。

**剩余未知**：radare2 pdg 产物仍含未命名 `fcn.*` 调用；需用 Ghidra 侧 call 目标签名回填后由 restorer 重建（半木主循环）。跳表无（此函数无 jumptable）。

### 4.2 `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）

**失败原因**：两泛型 `<true>/<false>` 翻转、closure 绑定多，此前未定点。

**攻克方法**：Ghidra shared 已可全量反编译（97KB，`construct_playermade_stairs_arches_n_walls_140496800.c`）。`find_functions` 定位到 `<false>` 的 closure_env$1/2/5/8/9<false>、walk<closure_env$6<false>>、fold<closure_env$2/4/7> 等全部单态地址（已列 evidence）。

**逻辑结论**：主函数 `[RCX..]` 多参数系统；closure$1..9 绑定 StairArchType/Option<f32>/Vec2/Vec3/CurveU 迭代。调用 `from_iter` 族 + `walk`/`fold`（ArchWalker）。

**剩余未知**：`<false>` 独立入口地址未在 PDB 命名（内联或同名折叠）；restorer 以 `<true>` 反编译为基，翻转布尔常量即可。

### 4.3 `sine_wave`（BrickRowSine inlinee，module 401）

**失败原因**：无独立编译实例（inlinee 24627/24641/24629），宿主 `from_curve`（0x140769760）体内按行号提取。

**攻克方法**：Ghidra 全量 `from_curve`（55KB）→ 定位正弦波 inlinee 块（source_line 12/45/82 对应体内 3 段）。

**逻辑结论**（完整公式）：
- **random_f**（hash rng）：`a = sin((x*10 + phase)*12.9898); frac(a*43758.547)` 生成 4 个随机值
  - `local_278 = frac(sin((x*10+p)*12.9898)*43758.547)`（[0,1)）
  - `local_274 = frac(sin((y*10+p)*12.9898)*43758.547)`（[0,1)）
  - `local_270 = frac(sin(2*a)*43758.547)*1000`、`local_26c` 同理（相位×1000）
- **deform**：`s = 0.5 + 0.5*sin(x*0.2)`；`y1 = 0.06*sin((local_278*s + 1)*x + local_270)`；`y2 = 0.06*sin(x*(s*local_274 + 1) + local_26c)`
- 应用于 BrickRowSine（BrickRow 顶/底边正弦波偏移，幅度 0.06）

**剩余未知**：宿主 from_curve 内 inlinee 精确边界行号（12/45/82 待 LLVM 行号核对）；但公式语义完整可复现。

### 4.4 `calc_gothic_half_dormer_profile_ws`（0x1407eb160，任务点名）

**失败原因**：任务点名；此前未在 13 模块报告单列（§4 注 "如需要再取证"）。

**攻克方法**：Ghidra 全量 6 函数族（calc/curve/door_curve/dormer_curve/generate/get_hat_bones）+ 常量表 read_memory。

**逻辑结论**：
- `gothic_half_dormer_profile_curve`（0x1409a4cc0）：3 档 profile 点表（每点 `(u, h)`）
  - case1：6 点 `[(0,1.26),(0.2,1.09),(0.35,0.87),(0.44,0.62),(0.48,0.36),(0.48,0)]`
  - case2：8 点 `[(0,1.7),(0.21,1.6),(0.39,1.46),(0.54,1.29),(0.65,1.09),(0.72,0.88),(0.75,0.65),(0.75,0)]`
  - case3：10 点 `[(0,2.25),(0.21,2.14),(0.4,2.0),(0.57,1.82),(0.71,1.63),(0.82,1.42),(0.9,1.19),(0.95,0.96),(0.96,0.72),(0.96,0)]`
- `calc_gothic_half_dormer_profile_ws` 主体：遍历 curve 点，`closure$0`（0x1407e7dd0）逐点求 `Vec3(u, h*max_y_clamp, 0)` 并插入墙 max_y 截断插值点；`call_once closure_env_0`（0x1412ac9b0）= 圆/直墙 inverse_lerp + `is_circle? 0.38 : 表 [0x142c2b180]={0.25,0.3,0.29}` 半径偏移

**剩余未知**：profile 档位（1/2/3）对应 gothic dormer 类型语义（restorer 按枚举对齐）。

---

## 5. interaction/clutter —— 任务点名卡点

### 5.1 `populate_autoclutter_regions`（0x1422d6ed0）

**失败原因（现状）**：总账记 "44KB 碎片 16 段/跳表分派" —— 本轮实测 **Ghidra shared 已可全量反编译 120KB**（`populate_autoclutter_regions_1422d6ed0.c`），不再碎片。

**逻辑结论**：主 switch 按 `*(char *)(local_138-7)`（clutter kind byte 0/1/2/...0x19）分派，逐 kind 构建 `AutoClutterSpawn` 记录（0x98 stride 每格 {p+scale=0x10B}）。case0：9 字段（scale 常量 0x3f99999a=1.2、0x40200000=2.5、0x3f800000=1.0、0x3f99999a 等，Option 判别 `[6]==-1→0`）。调用 `Rectangle3d`、`Circle2d::contains_point`、`Aabb2::from_center_radius`、`Rectangle2d::contains_point_padding`、`terrain_height::sample`、`TerrainRockMask::sample`、`bushes_near_point`。

**剩余未知**：无（逻辑可直接实现；clutter kind 枚举 → 具体资产需 restorer 对资产清单）。

### 5.2 `add_autoclutter_on_windows`（0x1422cd7d0）

**失败原因**：Ghidra bad instruction data（边界 364B 截断）。

**攻克方法**：radare2 全函数反汇编（364B 完整）+ 跳表 0x143042370 手工解码。

**跳表解码**（index=kind byte `[r9-0x50]`，6 项）：

| kind | 目标 | 语义 |
|---|---|---|
| 0 | 0x1422cd9b7 | 四元数旋转生成 `[spot..]+`（0x50 记录） |
| 1 | 0x1422cdbd8 | 同 + 边界检查（`jae → panic_bounds_check`） |
| 2 | 0x1422cdac7 | 同 + `not r12b`（翻转标志） |
| 3 | 0x1422cdb50 | 同 |
| 4 | 0x1422cda3f | 同 + `r12b=2` |
| 5 | 0x1422cdc72 | 同 |

**逻辑结论**：遍历 window 记录（stride 0x50），kind 分派各 handler 计算四元数旋转偏移（XMM6=0.5、XMM7=1.0 常量），写 `AutoClutterSpawn`（0x68 stride，`[rax+rcx]` 起 9 字段）+ `[r15+0x58]` 计数 + 音频 `[rbp+0x50]`。尾部 `[rbp-0x10]` 增长路径 + `call 0x14078c880`（Vec grow）。

**剩余未知**：四元数精确分量（可逐步实现，SIMD 序列已录）。

---

## 6. re-analysis 残留补证

### 6.1 `manage_tree_prefabs`（0x141bf16c0）has_construct_shape 门控位来源

**补证**：`*(char *)(lVar24 + 0x18) == '\x01'`（lVar24=query cursor/placement data 记录）：
- 循环首判 `[记录+0x18]==1` → `fVar50 = rng*0.75 + (1-rng)*0.3`（随机 scale）；否则读 `PlacementDataSsbo::position` 高度
- 循环尾 `*(char *)(lVar24+0x18) = 1`（本帧处理过）
- **结论**：门控位 = **SSBO 缓冲/资源记录的处理状态字节（+0x18）**，非"事件存在性"；首帧置 1 后后续帧走随机 scale 分支。总账推断正确。

### 6.2 `move_critters`（0x142318fa0）残留

**补证**：
- spherecast 命中码 `*puVar29`（legacy_spherecast 返回）：
  - `==2` → 岩石路径：`sample_at_world_xz > 0` 且 AL 标志 → 跳过移动（阻塞）
  - `==3` → 墙体循环跳过 gable 检查（`*puVar29 != 3` 才做 `max_y_gable`）
  - 其他码/`6<码`+`99U>>(码&0x1f)&1` 位集 → 非位集 locomotion（loc∉{0,1,5,6}）才 spherecast
- **mult==0 墙路径位移来源**：`fVar32 = *pfVar18`（multiplier）；`local_130 = [puVar15+1] * _Y * fVar32`（y=vel.y*dt*mult）；`local_138 = CONCAT44(fVar32*vel.z*dt, fVar32*vel.x*dt)`；`fVar32!=0`（含 NaN）才进入 LAB_142319bc1 位移分支 → **multiplier==0 时位移项恒 0，仅墙体 relate/朝向分支**（总账残留"位移来源"已证）
- 朝向：`Quat::from_rotation_arc(dir_x0z, up)` + `nlerp`（`fVar31=powf(0.01,dt)` 平滑）

### 6.3 `critters_avoid_collisions`（0x1423179a0）残留

**补证**：
- `*local_108`（loc type 查询结果）：
  - `!=2` 或 `(==2 && rock_sample<=0 && !AL)` → 进入墙体/水面路径
  - `==2 && rock_sample>0` → 跳过碰撞（保留目标）
- **hitcode==2 保留路径**：`local_258==2 || 球距>=1.0` → `goto LAB_142317e4f`（保留目标）；`球距<0.4 && water8` → `EventWriter::write<DespawnDuckCmd>`（清目标+deferred despawn）
- water8：`8×bilinear_sample` 求和 clamp[0,1] `fVar26<=0` 才走 spherecast；`fVar26>0` 直达清目标

**剩余未知**：`critters_avoid_collisions` 中 water8 标志→Option 组件映射（以 `!samples_rock_mask` 近似）；RaycastWorld::legacy_spherecast 的 ABI 返回结构（hitcode 字段偏移）。

---

## 7. 交付物

- 报告：`reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md`
- 证据：`reports/re-analysis/hardpoints-ecstate/evidence/`
  - `decomp/`：36 个反编译/反汇编文件（含 5 个 critters 跳表函数、color_menu/show_color_options 全量 136KB/267KB、half_timbering r2 pdg、chimney 31KB、from_curve 55KB、gothic 族、populate_autoclutter 120KB）
  - `jobs/` + `results/`：全部工具 job 与结果 JSON（可复核）
  - 跳表解码：6 张表（critters 5 张 + add_autoclutter 0x143042370）
- 说明：任务提示的「roof/water 跳表破坏族」——water `draw_water` DrawState::update 4 相位跳表 0x142eebcb0 已由 water RESTORE 先行解码（case0→return/case1→disc 0/case2→extend/case3→disc 3），roof 模块无未解跳表项；本批次不重复取证。

---

## 8. 遗留（未攻克/需继续）

| 函数 | 地址 | 状态 |
|---|---|---|
| `process_playermade_stair_pillars_proposals` 宿主 | 0x140178a70 | bevy impl$3::call_mut 系统包装，内层业务块未定点（本批次未处理；任务列表含 playermade 两泛型主函数已攻克） |
| `hint_position_update` / `hint_focused_edit_hint` | 0x141c50510 / 0x14227dd60 | onboarding 模块（非 eco/state 范围，未处理） |
| toolbar `ui_sounds`/`toolbar_auto_hide` 族/`place_childen_impl`/`add_toolbar_item`/photomode `ui_slider` | 0x14233c450 区等 | toolbar 模块（非 eco/state 范围，未处理） |
| `add_autoclutter_around_gates` 等 in-module 纯逻辑 | — | 依赖已具备，按总账 §5 implement-in-module 直接重建 |

> 注：`half_timbering` 的 Ghidra/ReVa "varnode hash" 在 shared/专属 clone 均复现（即使修签名）；radare2 `aa`+pdg 是可行替代。`color_menu`/`show_color_options` 的 shared DB codelen=0 为 overlay 损坏，fresh clone 反编译正常 —— 后续 restorer 重建须用 write worker clone 的反编译为基，不要依赖 shared DB 空反编译。
