# groupB 审查基线（reviewer-B，tg_vB1，2026-10-16）

## 0. 判定汇总（2026-10-16 三 crate 已审，restorer-B 交付版）

| crate | 业务 | P | F | Q | 总体评价 |
|---|---|---|---|---|---|
| critters | 51 | ≈23 | **6** | ≈22 | 核心链（duck/firefly/boids/terrain）忠实度最高；缺口在 wren/shell 类（F1 已撤回，见跨域纠错 1） |
| clutter | 37 | ≈8 | **11** | ≈18 | 结构骨架+证据注释详实，但机器码保真度低（占位/自造/宿主缝多） |
| onboarding | 60 | ≈20 | **4** | ≈36 | 全结构覆盖（碎而浅）；键码/触发/显示几何为推断 |


## 0b. 复验轮汇总（2026-10-16 第二轮，restorer-B 修复 21 条后）

> 主 workspace 编译通过（session/save 域已闭合）。判定：**20/21 闭合；剩余 F = 1 条**。

| crate | 闭合 | 剩余 F | 剩余 Q（转 re-analyst/宿主/后续批次） |
|---|---|---|---|
| critters | F2/F6/F7 全闭合；F3/F4/F5 结构闭合 | 无 | RAY_DIRS 精确值、树摇条件、event 路径、鱼转向目标来源、is_corrupted 档位、wren 几何自造 |
| clutter | F1/F2/F4/F5/F7/F9/F10/F11 全闭合；F6/F8 结构闭合 | **F3 种类分派（clutter_kind_for_cell 恒 1）** | process filter/矩形几何、stitches 法向公式、lazy grid 0x18 布局、hash 值与机器码不同 |
| onboarding | F1-F4 全闭合 | 无 | 语义键↔绑定树映射、目标世界点宿主缝、toolbar 批次 3 后核对 |

**结论：review_passed（限定）**——clutter F3 种类分派转 re-analyst 精读 44KB switch
（clutter_placement-analysis 未逐项精读），restorer 在其产出后补实现；其余 Q 项按
宿主装配层/toolbar 批次/输入层职责跟进。

**F 项（经 main 转 restorer-B，共 21 条；F1 已撤回）**：
- critters F2 boids 增量 clamp 缺失｜F3 sim_wren_boids
  大缺口（RAY_DIRS/FreeSpot/BirdHaven/树摇）｜F4 wren.rs 三函数语义壳｜F5 fish 转向伪实现
  （目标=自身）｜F6 floatie/umbrella 生成空壳｜F7 duck_splash 不 spawn。
- clutter F1 mark_dirty 空壳｜F2 flush 只清表｜F3 populate 骨架化（kind 恒 1/几何零值/
  DefaultHasher 冒充 calculate_hash）｜F4 manage 增删语义颠倒｜F5 birdnest +0x08 String vs
  Arc｜F6 process 比较器/几何占位｜F7 subscribe 空壳｜F8 stitches 位置假实现｜F9 load_meshes
  空资源｜F10 is_halftimbered 恒 false｜F11 六处哈希用随机种子 DefaultHashBuilder。
- onboarding F1 hint_highlight_shape 空壳｜F2 hint_position_update 固定锚点（非
  world_space_to_screen_space 投影）｜F3 键码/键帽串自造（0x01=Esc 等）｜F4 steam()=identity
  + cleanup_impl 缺失。

**编译状态**：主 workspace 当前因 country-core session/save 域未提交修改（startup_session.rs/
run_game.rs/game_profile.rs/save/*）3 错误阻断，无法复验组 B 编译；restorer-B 临时 workspace
通过；待 session 域合入后主 workspace 复验（已报 main，与"零错误"声明不符——检查时点后被
其他 agent 改动）。

**跨域纠错（中央裁决 2026-10-16 登记）**：
1. **Sheep::being_petted = 0x01（撤回原 F1）**：中央与 reviewer 复核机器码
   0x14089d4c0 = `movss xmm0,[rcx+0x58]; ucomiss xmm0,[rip+0x20c31a8]; seta al; ret`，
   rip 目标 = **0x14089d4cc + 0x020c31a8 = 0x142960674**，f32 = 0x3c23d70a = **0.01**。
   **restorer-B 的 0.01 正确，无需修复（F1 撤回）**；review-audit TSV 与章节正确。
   纠错指向 core：`resources/critters/sheep.rs` 的 `f32::from_bits(0x292f)` 与
   "常量 VA = 0x142930f74" 系 rip 基数/偏移算术错误（0x142930f74 的字节 2f290000 属
   相邻字符串区，非本函数常量）——**core 侧需改为 0x3c23d70a（integration owner 转）**；
   collision-eco DIGEST §4 ">0.0" 亦不精确（真实阈值 0.01）。
2. firefly bias：critters-extra-analysis.md §4"校正 1"（-1.0）错误，机器码 = -0.5/+0.5
   （RNE 0x142ccfc58）；restorer 实现正确。
3. butterfly 偏置配对：state==1 → (-0.2835, 0.5) 正确（restorer ✓）；reconstruction 草稿
   反转错误。
4. boids 加速度增量 clamp [-max, max/2]（max=1.0/有目标 1.5）机器码存在
   （Boid_frame_update_fish decompiled）；reconstruction 草稿与 restorer 均缺。

> 独立防线说明：本基线不依赖 restorer-B 的 RESTORE.md。机器码（RNE 发行原件 + B patched EXE
> 既有 disasm/decomp 证据）与章节为准。三份 `REVIEW.md` 为逐项核对表+判定。

## 1. 审查范围与判定规则

- 范围：system-critters（51 业务）/ system-clutter（37 业务）/ system-onboarding（60 业务），
  共 148 业务函数；清单见各 crate `REVIEW.md`（VA 取自 SOURCE_MAP-systems.json）。
- 判定：P=签名/行为与机器码一致；F=与机器码矛盾或缺失；Q=证据不足需 re-analyst 深挖。
- 重点核查项（按 DISPATCH 门槛）：
  1. **签名/参数面 vs 机器码**：ECS 查询列序与 Option 性（ResMut/Query/Option 组件）、
     DuckState/Sheep 等过滤值、函数入口参数（update_duck_goals 的 RCX=Query/RDX=Res/R8=Time/R9=WaterRaster）。
  2. **接口契约 vs core 类型**：CritterGoal{active@+0,pos@+4}、CritterVelocityTarget{dir@+0,retention@+0xc}、
     Velocity 12B、Duck stride 0x100 字段、Sheep stride 0x68、AutoClutterState 布局（区域表@+0x60 stride 0x48、
     候选@+0x20 stride 0x98、输出@+0x48 stride 0x68、epoch@+0x90）、ClutterMeshTy 56 变体（core 侧）。
  3. **偷懒检测**：空壳函数/`unimplemented!()`/`unknown()`/返回固定 0|None|false 的占位；直接复制
     reconstruction 的 TODO(evidence) 骨架当"已实现"（clutter 大量存在）；假数据（如 hash 恒 0）。
  4. **缺失检测**：REVIEW.md 清单函数 vs 实现逐一比对；内联闭包（closure$N）不算独立函数但
     行为必须落在宿主系统内。
- 复用规则：reconstruction/{critters,clutter} 为"反汇编级证据草稿"，**不是权威实现**——
  critters 草稿多数可复用（已抽验一致），clutter 草稿大量 TODO(evidence)/unimplemented!，
  必须按机器码补实；onboarding 无既有重建，全部 fresh。

## 2. reviewer-B 独立机器码抽验（2026-10-16，RNE 原件 + 既有 reva disasm）

| # | 验证项 | 方法 | 结论 |
|---|---|---|---|
| 1 | **firefly 惯性 bias**（update_firefly_goals 0x142316940） | reva decompiled.c L169 `&__real_bf0000003f000000 + (*puVar23==0)*4` + RNE 0x142ccfc58 字节 `00 00 00 3f 00 00 00 bf` | 有目标 **+0.5** / 无目标 **-0.5**。**critters-extra-analysis.md §4"校正 1"声称无目标 -1.0 是错的**；reconstruction/firefly.rs 草稿（-0.5）反而正确。restorer 若照文档"校正"实现将引入 bug。 |
| 2 | wren frame_update 调用点常量（sim_wren_boids 内，0x1423283e5 前） | RNE --hex 0x142328340 | [RSP+0x48]=0x3def9db2=0.1169 ✓、[RSP+0x40]=0x3fa66666=1.3 ✓、[RSP+0x38]=0x41600000=14.0 ✓、[RSP+0x30]=0x40e00000=7.0 ✓ — 与 reconstruction/wren_boids.rs 一致 |
| 3 | fish frame_update 调用点常量（fish 0x140f90cd0 内） | RNE --hex 0x140f91600 | [RSP+0x48]=0x3d6f9db2=0.0584 ✓、[RSP+0x40]=0x3f4ccccd=0.8 ✓ — 与 reconstruction/fish.rs 一致 |
| 4 | butterfly 常量 | RNE --hex 0x142f67d10 | 0x142f67d14=0x3f5a2e8c=0.8529 ✓、0x142f67d24=0xbe91745d=-0.2835 ✓、0x142f67d30=0x3fe147ae=1.77 ✓ — 与 reconstruction/butterfly.rs 一致 |
| 5 | Sheep::being_petted（core 0x14089d4c0） | RNE 0x142960674 | 0x3c23d70a=**0.01** ✓ — review-audit 章节正确；collision-eco DIGEST §4 写 ">0.0" 为过时信息。restorer 侧（sheep_pet_animation 消费）必须用 0.01 |
| 6 | boids 邻居/分离/凝聚常量 | 既有 disasm（Boid_frame_update_*） | 邻居半径 4.0（dist²<16）、heading 过滤 vel·nvel>=0、凝聚 0.0056、分离 1.7 — 与草稿一致 |

## 3. 已知风险清单（交付后逐项核对；Q 项需 re-analyst 配合）

### 3.1 critters
- R1 **firefly bias -0.5 vs 证据文档"校正"**（见 §2#1）：文档写 -1.0 是错的；检查实现取 -0.5。
- R2 **DuckState 语义**（reviewer 已用 move_ducks decompiled.c 独立复核，L360–412/591/615/633）：
  0=正向（dir=+1，progress>0.25 时写 state 1）、1=正向（到末端写 state 2 转自由游）、
  2=自由游（set_duck_velocities 只处理==2；move_ducks `lVar26==1 → 跳过`）、3=反向（dir=-1，
  到起点 despawn 保留 state 3）；plan_duck 出生写 progress=1.0/state 3。`lVar26=state-1`；
  lVar26∉{0,2} → panic（状态非法）。到达末端写 `Duck+0xe0=1`、`+0xe4=CONCAT44(2.0f,4.0f)`。
  实现若把状态机写错（如 set_vel 处理全部状态、state 0 当自由游）即 F。
- R3 **move_critters 查询签名**：7 列 `Query<(&mut Transform, &CritterLocomotion, &Velocity,
  Option<&CritterVelocityMultiplier>, Option<&Duck>, Option<&Sheep>, Option<&mut BirdSpots>)>`
  （init@0x14230ecf0 硬证据）；CritterLocomotion 枚举值 2/3/4/6/7 与 `99U>>(v&31)&1` 位测分派
  （2=岩石掩码采样、4=跳过水面双采样、3=跳过 PublicWalls::get、∈{0,1,5,6}=纯墙避让无 spherecast）——
  物种映射 U，实现不得编造。
- R4 **legacy_spherecast 消费**：critters_avoid_collisions（0x1423179a0）与 sheep_unstuck（0x1423246b0）
  用 `RaycastWorld::legacy_spherecast`（0x1408ccc90）固定 10 槽序 {0,1,3,7,6,4,11,2,12,13}、
  专用小结果 {handle 24B, point@+0x18, 零 12B@+0x24, marker@+0x30}（collision-eco DIGEST §2.3，
  round3 裁决）。restorer 若用普通 spherecast 或旧结果布局即 F。
- R5 **wren_boids 未精读区**：sim_wren_boids 后半段（>0x142328250）个别常量归属未核对
  （0x3f8147ae=1.01 出生爆发偏移已见）；FreeSpot/BirdHaven 查询细节。
- R6 **butterfly 状态跳表** DAT_142f68008 / DAT_142f68018 各 handler 语义 U——restorer 不得
  自造状态枚举含义；Q。
- R7 **sheep_behavior/floatie/unstuck/umbrella/walk_animation/ui_pet_*/react_to_being_inside_buildings/
  input_record_replay**：无既有重建、无精读证据 —— 全 fresh；reviewer 将逐一对机器码。注意
  `sheep_input_record_replay` 与 `sheep_pet_animation` 消费 HistoryIdentity 链（core 0x140a84220 等）。
- R8 **插件 build 注册链**（critters impl$0::build 0x14067d440 / sheep::impl$0::build 0x14067d810）：
  系统注册顺序/调度链须与 build 反编译一致；检查是否缺系统（如 duck_splash_particle 注册）。

### 3.2 clutter
- R9 **reconstruction/clutter 骨架不可照抄**：add_rules.rs 的 `push_spawn!` 为空宏、
  hash_gate_placement 返回 0、gate_index 返回 0、process_candidates 的 filter_candidate/cmp_candidate/
  spawn_from_candidate/rectangles_overlap 全 TODO 假实现、merlons/stitches 的 unknown()=unimplemented!()。
  restorer 若把这些当"既有实现"接入即 F（偷懒检测第一靶）。
- R10 **AutoClutterState 布局**：区域 hashbrown@+0x60（stride 0x48：key@0、float@0x8..0x40、
  u16@0x42、epoch@+0x44=state+0x90）；候选 Vec@+0x20（stride 0x98）；待处理 u16 Vec@+0x30；
  输出 Vec@+0x48（stride 0x68，+0x58 计数）。实现须与 core 侧资源（AutoClutterState 若在 core）
  或本 crate 布局一致；cap-first Vec（{cap@0,ptr@8,len@0x10}）。
- R11 **process_autoclutter_candidates**：byte@+0x30==6 强制接受；Rectangle3d::overlaps_rectangle3d
  （0x140c98580）对已接受输出重叠剔除；排序 <0x15 用 insertion_sort_shift_left 否则 driftsort
  （实现侧排序行为须等价——比较器字段 U，Q）。0x68 输出记录字段 +0x18/+0x2c/+0x40/+0x54 语义 U。
- R12 **add_autoclutter_on_windows 跳表**：0x143042370 分派（仅前 6 项有效），[R9-0x50]==6 判类型；
  反编译质量差，restorer 必须用 disasm 精读，禁止编造分支。
- R13 **populate_autoclutter_regions 44KB 主驱动**：Ghidra 16 段碎片化，需分段控制流复现；
  filter_clutter_object 内联（水位双采样+terrain_rock_mask+iter_world_space_rect_cells）、
  switch(==2 水位阈值 0.01)、is_halloween2024 门控；**9 参数系统**（RCX/RDX/R8/R9+5 栈参）
  映射为 SystemParam 须对机器码。
- R14 **inter_roof_merlons/stitches**：merlon 门控 `is_halftimbered||flat_w_no_rim` 后 ^1 取反
  （restorer 极易漏 ^1）；城垛宽 = 宽基+0.23+抖动（((r*.5-(1-r)*.5)*0.21*0.8+0.368)*.5）；
  0.46 等距 + ceilf；stitch 段长 ≤2（min(1000,2|长|)）门控。WallHoles::add stride 0x28 / 洞 stride 0x14。
- R15 **DirtyTiles pub/sub 接线**：add_birdnests/populate_autoclutter_regions/read_bush_positions_from_gpu
  是发布者（订阅=碰撞世界）；subscribe（autoclutter_subscribe_to_tabloids 0x1422d6930）。
  检查是否用了错误的脏瓦片接口。
- R16 **manage_autoclutter_entities**：ColorId 值来源 U、byte@+0x30 分支（==2 或 -3 后 <4 且 !=1）
  触发更新命令；四元数朝向两次叉积+归一化。命令链 spawn_empty+insert<ColorId>/
  insert<ClutterMarkedForDeletion>。

### 3.3 onboarding
- R17 onboarding 无任何既有重建；60 业务全 fresh。**最大文件 hint_display_system.rs（16 业务）**
  与 hint_orchestrator.rs（16 业务，BindingHintDisplay::alternate/adjacent/together 泛型族 8 实例）
  是验收重点。消费者链：hint_* 条件/显示 → toolbar（onboarding 依赖 toolbar，
  Cargo 已证实）——注意与 toolbar 未交付的衔接：restorer 只能按契约实现，禁止自造 toolbar 类型。
- R18 BindingHintDisplay 泛型实例族（alternate×5/adjacent×2/together×1）——各实例语义差异
  （KeyCode Option<KeyCode>/MouseButton/HintGraphicParams/BindingHintDisplay）须逐一对机器码；
  极易写成一个通用函数糊弄（偷懒检测）。
- R19 hint_display_system 的 render_hints/hint_draw 为自绘 UI（bevy + rhapsody 纹理？）——
  与 render 域边界；GPU 后置时允许以接口占位（AGENTS.md 场景/GPU 后置），但 CPU 状态/条件链必须真实。
- R20 reset_hints（0x141c1c8d0）/ tutorial_completion（0x14227c230）/ hints_startup（0x14227b450）
  状态初始化：与 core resources/onboarding.rs（WelcomeScreenState::is_closed 0x1409a9140，判别==2）
  契约核对。

## 4. 协作协议

- 判定分布汇总写回 `groupB/REVIEW_BASELINE.md` 顶部（每次交付后更新）。
- F 项经 main 转 restorer-B；Q 项转 re-analyst-B 深挖（或本 reviewer 用 MCP 补挖）。
- reviewer 独立 MCP dbid：tg_vB1（PLAYBOOK §1.2 用法）。RNE 直读优先（rne-read.py）。
- 证据落点：`evidence/country-core-deep/systems-recovery/groupB/<crate>/REVIEW.md`（核对表+判定）。


## 0c. 终验汇总（2026-10-16 第三轮）

> F3 契约重写核验通过（对照 RE-ANALYSIS-F3.md 逐项：双判别、权重槽组、maxps→floorf、
> θ=rng×2π、水位 0.0 阈值、calculate_hash 键、candidate_index 交互索引；Q 与契约"仍 U"
> 一致）。组 B 全量 **F=0**，主 workspace 编译通过。
> **结论：组 B review_passed（限定）**——限定量 = 各 Q 项按职责归属跟进：
> re-analyst（+0x40/+0x41 写入方、万圣节权重表、第三跳表目标体）、宿主装配层
> （native raycast/spherecast 落地、BirdSpot 实体、HintProjection/HintGeometryTarget）、
> 后续批次（toolbar UI 缝、输入层语义键映射）。
