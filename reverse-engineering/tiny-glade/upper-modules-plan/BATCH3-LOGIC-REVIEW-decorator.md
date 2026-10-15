# 批次 3 · systems/decorator 业务逻辑复核

> 复核对象：`upper-modules-plan/systems/decorator/ANALYSIS.md`（A 主干 + B visual 族非楼梯 + C 楼梯视觉/ui_systems/utils 三部分合并；已逐行核对三份原分件内容 ⊆ 合并文件，无缺失）。
> 覆盖基准：`skeleton/B/crates/systems/decorator/` 全部 `.rs` 骨架中 `system_decorator::` 命名空间、`记录形式: compiled_procedure`（有独立符号/编译实例 RVA）的发射函数。
> 抽查：ghidra 共享只读 worker 定点反编译核对可疑结论（地址一律 B VA，`VA = 0x140000000 + RVA`）。
> 范围：只核模块内部实现与覆盖，不分析上下层调用关系；不猜默认值。

---

## 1. 复核范围与结论摘要

- **发射清单**：从骨架共提取 **226 个**去重 `system_decorator::` 发射函数（按文件合计 228 条，2 条跨文件重复；lib.rs 另有 10 条同地址注册副本，README 的 236 口径即 226+10）。其中闭包命名 58 个（与 README「58 闭包」一致）、bevy `_::impl$0::*` 参数胶水 6 个、default/fmt impl 5 个。
- **覆盖**：约 **166/226**（按函数 RVA 或 VA 在 ANALYSIS.md 出现计）被分析条目点名；**60 个**发射函数在 ANALYSIS.md 中无任何 RVA/VA 引用。其中 4 个为 default/fmt/bevy 胶水（按 AGENTS.md 口径可不计），**约 10 个是真正有实体的顶级函数被误标为「内联/U」或漏列**，其余为宿主系统已覆盖但闭包未逐条枚举（详见 §2/§4）。
- **总体评价**：A 主干质量高（逐函数逻辑、算法、未知标注基本可信，且证据目录 `evidence/toolchain/decor-trunk/decompiled/*.c` 与描述一致）；B/C 两份无落盘反编译证据（仅 A 有 `decor-trunk/decompiled/`），且存在一类系统性错误——**把若干有独立符号、可反编译的发射函数写成「内联」**，集中在 decorator_visual 的砖列装配、dormer 瓦片、差分追踪、门下楼梯检测。此错误直接影响「逐函数逻辑是否讲清」与 unknown 清单的准确性，需修正。

---

## 2. 覆盖核对表

发射函数按 B src 文件分组；「分析条目」= ANALYSIS.md 三部分中是否点名（RVA 或 VA）。`*` 标漏列/误标。

| B 源文件 | 发射 | 分析点名 | 遗漏/误标（VA） |
|---|---:|---:|---|
| apply_decorator_op.rs | 1 | 1 | —（write_*_op_history 确为 inline，标注正确） |
| audio/op_confirm_audio.rs | 2 | 2 | — |
| audio/op_preview_audio.rs | 1 | 1 | — |
| audio/op_stairs_audio.rs | 2 | 2 | —（closure$9 已点） |
| backup.rs | 3 | 2 | * `impl$3::default` 0x140824680 漏列（DecoratorBackup::Default，发射） |
| blueprint.rs | 13 | 12 | * `validate_blueprints::closure$0::closure$0` 0x1412ba810 漏列 |
| cache_decorator_transforms.rs | 1 | 1 | — |
| chimney_smoke_anim_thickness.rs | 1 | 1 | — |
| collision_detection.rs | 2 | 2 | —（intersect_wall_decorators 两单态已点，_impl 内联判断可信） |
| cull_oob_decorators.rs | 9 | 9 | — |
| decorator_on_freehand_wall_edited.rs | 2 | 2 | — |
| decorator_on_move_shape.rs | 1 | 1 | — |
| decorator_on_rectangle_edited.rs | 1 | 1 | — |
| decorator_on_wall_height_changed.rs | 2 | 2 | — |
| decorator_visual/arrow_slit.rs | 1 | 1 | — |
| decorator_visual/chimney_assemble_bricks.rs | 3 | 1 | * `construct_crossbrick_chimney` 0x14200bcd0（2348B，134 行反编译）被误标「主体内联 U」；`impl$4::fmt` 0x141289820 胶水 |
| decorator_visual/chimney_roof.rs | 1 | 1 | — |
| decorator_visual/chimney_wall.rs | 1 | 1 | — |
| decorator_visual/cottage_balcony_door.rs | 2 | 2 | —（add_door_autoclutter 已点） |
| decorator_visual/cottage_corner_window.rs | 1 | 1 | — |
| decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs | 3 | 1 | * `assemble_dormer_roof_tiles` 0x14128db20（9547B，938 行反编译）被误标「内联」；closure$3 0x1412905d0 漏列 |
| decorator_visual/cottage_dormers/collision_geo.rs | 1 | 1 | — |
| decorator_visual/cottage_dormers/cottage_dormer.rs | 2 | 2 | * `is_out_of_roof_bounds` 0x1407ca6c0（1560B，139 行）被降格为「I：返回极性」推断，实为可反编译函数 |
| decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs | 1 | 1 | — |
| decorator_visual/cottage_dormers/util_assemble_gothic_dormer.rs | 1 | 1* | `assemble_gothic_dormer_tiles` 0x141290b30 仅出现在流程句，未标为发射函数 |
| decorator_visual/cottage_wall_window.rs | 3 | 3 | — |
| decorator_visual/deco_differential_state.rs | 8 | 4 | * `with_diffs`×2 0x1412ae660（2820B/607 行）/0x1412af3a0（1469B/579 行）被误标「均内联」；`impl$2::default<DoorStairs>` 0x1410b940 漏列 |
| decorator_visual/decorator_clutter.rs | 1 | 1 | — |
| decorator_visual/door_stairs_assemble_bricks.rs | 2 | 1 | * `construct_door_stairs` 0x14200c7f0（3685B，257 行）被误标「内联 U」 |
| decorator_visual/flag.rs | 1 | 1 | — |
| decorator_visual/flag_connections.rs | 3 | 3 | — |
| decorator_visual/generate_clothline.rs | 1 | 1 | — |
| decorator_visual/gothic_balcony_door.rs | 2 | 1 | * `check_for_door_stairs` 0x1407f3340（1330B，109 行）被断言「内联（无独立符号）」——**直接错误** |
| decorator_visual/gothic_dormer.rs | 2 | 2 | — |
| decorator_visual/gothic_wall_window.rs | 3 | 3 | —（calc_gothic_half_dormer_profile_ws 打散标注正确） |
| decorator_visual/lantern.rs | 6 | 2 | * `compare_light_lists`×2 0x141257fa0/0x1412583d0 被误标「内联 U」；`impl$0::default` 0x140790220、`maintain_lantern_lights::closure$5` 0x141257c50 漏列 |
| decorator_visual/maintain_visual_entities.rs | 2 | 2 | — |
| decorator_visual/stairs.rs | 13 | 11 | * closure$1<true> 0x140285a90（点名「closure_1<true>」但无地址）、walk_shape::closure$0 0x1412b2540 漏列 |
| decorator_visual/stairs_assemble.rs | 23 | 18 | * `playermade_stairs_assemble` 的 4 个闭包 closure$8/9<true>、closure$24/25<false>（0x140284b0/0x275b0/0x286d0/0x27840）全部漏列——空反编译函数的分派逻辑很可能在这些闭包里；calculate_adjusted_ladder_positions::closure$0 0x1407d9240 漏列 |
| decorator_visual/stairs_railings_assemble.rs | 5 | 5 | * railings_tips::closure$0 0x1407fdb70 漏列 |
| decorator_visual/trap_doors.rs | 1 | 1 | — |
| decorator_visual_state.rs | 5 | 5 | — |
| deferred_decorator_history.rs | 1 | 1 | — |
| derive_decorator_subtype.rs | 3 | 3 | —（可复用标注正确） |
| dormer_holes.rs | 1 | 1 | — |
| handle_external_decorator_cmds.rs | 1 | 1 | — |
| load_decorators.rs | 5 | 5 | — |
| merge_proposal_roof_decorators.rs | 2 | 2 | — |
| merge_proposal_wall_decorators.rs | 2 | 2 | — |
| move_decorators_following_anchors.rs | 1 | 1 | — |
| remove_ivy_under_windows.rs | 1 | 1 | — |
| replay_decorator.rs | 1 | 1 | — |
| restore_archived_decorators.rs | 1 | 1 | — |
| ui_systems/calculate_decorator_dst.rs | 1 | 1 | —（抽查正确） |
| ui_systems/calculate_decorator_grab_offset.rs | 2 | 2 | — |
| ui_systems/decorator_interaction_intent.rs | 2 | 2 | — |
| ui_systems/handle_splitting_roof_decorators.rs | 6 | 2 | * 4 个闭包 0x14127b570/0x14127b730/0x14127b8f0/0x14127c7f0 漏列 |
| ui_systems/handle_splitting_wall_decorators.rs | 6 | 2 | * 4 个闭包 0x141278ed0/0x141279090/0x141279250/0x14127a480 漏列 |
| ui_systems/move_ops.rs | 3 | 2 | * show_decorator_remove_signifier::closure$0 0x141280020 漏列 |
| ui_systems/ui_change_chimney_height.rs | 4 | 2 | * show_signifiers::closure$0/1 0x1407aecc0/0x1407aeea0 漏列 |
| ui_systems/ui_change_stair_height.rs | 1 | 1 | — |
| ui_systems/ui_decorator_affordance_dispatch.rs | 5 | 4 | * ui_dispatch_decorator_affordance::closure$5 0x1407b17c0 漏列 |
| ui_systems/ui_focus_stairs.rs | 5 | 1 | * 4 个闭包 0x1407a3db0/0x1407a3fe0/0x1407a41c0/0x1407a43b0 漏列（函数体 U，闭包即可能承载子逻辑） |
| ui_systems/ui_inactive.rs | 1 | 1 | — |
| ui_systems/ui_move_decorator.rs | 10 | 2 | * 闭包 closure$3 0x1407a7a50、highlight_child_autoclutter::closure$0 0x1407a82f0 漏列；6 个 `_::impl$0::*` 为 bevy 参数胶水（`apply/new_archetype/queue` 同址 0x1407a8550），应分类排除 |
| ui_systems/ui_place_decorator.rs | 6 | 3 | * closure$14/15 0x1407ad8c0/0x1407ad9f0、show_decorator_icon::closure$0 0x1407a96a0（= FnOnce::call_once 同址别名）漏列 |
| ui_systems/ui_rotate_lantern.rs | 2 | 1 | * ui_rotate_decorator::closure$0 0x1407a9390 漏列 |
| ui_systems/ui_stairs_misc.rs | 2 | 2 | — |
| ui_systems/ui_stairs_viz.rs | 9 | 2 | * visualize_ui_stairs 的 7 个闭包 0x14079af50/0x14079af80/0x14079b0b0/0x14079b100/0x14079b4f0/0x14079b860/0x14079ba40 漏列（「分态绘制」的实际逻辑载体） |
| utils/visual_entity_builder.rs | 2 | 2 | — |
| lib.rs | 0 独立 | — | Plugin::build 无独立符号（标注正确；lib.rs 的 10 条为注册副本） |

**小结**：主函数级覆盖完整（A 主干 19/19 文件全覆盖，B/C 主函数基本全覆盖）；缺口集中在 **11 个被误判/漏列的有实体函数** 与 **约 40 个宿主已覆盖但未逐条枚举的闭包**。

---

## 3. 逻辑总清单（逐函数逻辑 / 算法 / 可复用）

### 3.1 逐函数逻辑抽查结论

- **apply_decorator_op 0x141276e30**（抽查反编译）：A 版描述与反编译逐条吻合——`*op-3<2 → op-2` 分派、lVar1==0 读 `in_RCX[2]` 子 tag、tag!=3 二次 archive、历史向量 push 3/4/5、HashMap<DecoratorLocator,()> 记录、apply_change/place/remove、末尾 `DeferredDecoratorOpEvents::send`。**新增细节**：历史缓冲条目实为 0x20 步长 `(DecoratorId, Vec<system_decorator::...stairs::PreferredNodeAlignment>)`，A 版未点出负载类型。
- **convert_raycast_hit_to_decorator_dst 0x14127c990**（抽查反编译）：C 版「0=Free/2=Wall(can_place_on_roof)/0xb=Terrain(can_place_on_terrain)/0xd=StairTop(can_place_on_stair_top)/默认=Invalid」**正确**。补充：case 2 与 case 0xd 各携带一个负载字段（+8 / +4），case 3 panic_fmt、case 5/9/0xc/0xf/0x10 panic——C 版未提 panic 路径与负载结构。
- **check_for_door_stairs 0x1407f3340**（抽查反编译）：真实业务函数，含地形 bilinear_sample、RasterBufferRef<512> 采样、门高/接地判据。B 版「内联（无独立符号；调用记录 0x7f2025→0x7f5e70）」**错误**——独立符号存在，0x7f5e70 实为 add_door_autoclutter。
- **is_out_of_roof_bounds 0x1407ca6c0**（抽查反编译）：139 行实体，B 版仅以「I」记返回极性，低估了该函数。

### 3.2 其余函数逻辑判断

- A 主干逐函数描述（§1-§19）内部一致、无自相矛盾；blueprint 差分重建、cull_oob 越界裁剪+碰撞重叠保护、merge 提案、move_following 重挂、墙形变化响应、历史/回放、audio 映射主体的 O/I/U 划分合理，字段偏移均为「反编译观察级」的诚实标注。
- B visual 族（除 §4 列的误标外）逻辑描述自洽；generate_arrow_slits/chimneys/windows/doors/dormers/flags/lanterns/trap_doors/clutter 的 mesh/shader 名+len、碰撞块、DormerHoleSsbo、等级等的拆分符合反编译风格。
- C 楼梯/UI（除漏列闭包外）逻辑描述自洽；`playermade_stairs_assemble` 空反编译、`calc_gothic_half_dormer_profile_ws` 打散、`determine_segment_type` 只剩骨架等卡点标注与我的抽查一致。

### 3.3 算法点名

- 已点名且正确：Arc::make_mut 式增量重建、hashbrown 探测、差分重建+seal、subtype 跳转表+is_half_dormer flat/gable 边判定、DormerKey 量化缓存凸包、旗帜贪心连接（容量+高度+遮挡）、布条等分插值、dormer 剖面硬编码曲线、坡度分类(0.25/2.5)+子段合并、样条密集采样(2/单位)、凹顶点剔除、受力支撑分割(-1.17/0.5)、结构图 Dijkstra/BFS 畸变洪泛(3/64)、帧间插值差分(USE_STAIRS_LAZINESS)、RNG 序列（0xa0761d6478bd642f 等）等。
- 需补点名的：`construct_crossbrick_chimney`/`construct_door_stairs`（**砖列块装配**）、`assemble_dormer_roof_tiles`（**瓦片装配主体，938 行**）、`with_diffs`（**双层差分应用**）、`compare_light_lists`（**双缓冲差分上传**）——这四个算法目前落在「内联 U」区，实际是发射函数，应移入算法清单。

### 3.4 可复用标注核对

| 标注 | 目标 | 核对 |
|---|---|---|
| 墙面四族/subtype 复用 `reconstruction/building/decorator_subtype.rs` | 存在；含 wall_opening_family（cottage 0/1、gothic 6/7、门 4/9）与 b_wall_subtype_collision_asset_name | ✓ 正确 |
| is_half_dormer「以 reconstruction 为准待原指令差分复验」 | `evidence/building-decorator-info/` 有 0x140813d00/0x140814470/0x140814110 等原指令 disasm | ✓ 正确 |
| 楼梯候选 `reconstruction/country-core/src/resources/stairs/*`（WP21） | 存在 stairs_state/structural_graph/stairs_assembly_params/stairs_collider/invalidate/stairs_removed_supports/visual | ✓ 正确 |
| `evidence/building-placement/`（placement/info 限定报告） | 存在 | ✓ 正确 |

---

## 4. 冲突与需裁定项

### 4.1 分析内部错误（需改 ANALYSIS，不改代码）

| # | 位置 | 原断言 | 事实（复核） | 裁定方向 |
|---|---|---|---|---|
| C1 | B §19 gothic_balcony_door | 「check_for_door_stairs：内联（无独立符号）」 | 发射 0x1407f3340（1330B），逻辑=地形/门高检测 | 改写为发射函数条目，单独分析 |
| C2 | B §1 deco_differential_state | 「with_diffs … 均内联（模块 1561）」 | with_diffs×2 发射 0x1412ae660/0x1412af3a0（607/579 行） | 改写为发射函数；40B params 布局可从其反编译恢复 |
| C3 | B §2/§3 chimney/door_stairs | 「closure_env_0 砖列主体（内联，U）」 | construct_crossbrick_chimney 0x14200bcd0、construct_door_stairs 0x14200c7f0 发射 | 移入主函数清单；砖列算法可恢复 |
| C4 | B §21 generate_dormer_roofs | 「其余→assemble_dormer_roof_tiles(内联)」 | 发射 0x14128db20（9547B）+ closure$3 0x1412905d0 | 改写为发射函数；瓦片装配主体 |
| C5 | B 未知清单 #7 lantern | 「compare_light_lists（内联，U）」 | 发射×2 0x141257fa0/0x1412583d0 | 改写为发射函数 |
| C6 | B §25 | is_out_of_roof_bounds 仅记「I 返回极性」 | 发射 0x1407ca6c0（139 行） | 升级为独立条目 |
| C7 | B §21 | assemble_gothic_dormer_tiles 仅在流程句中 | 发射 0x141290b30 | 补独立条目 |
| C8 | 多处 | 宿主已覆盖系统之闭包未逐条枚举（约 40 个） | 均为发射 | 在对应条目下枚举/分类；对 body-U 系统（ui_focus_stairs、visualize_ui_stairs、playermade_stairs_assemble）闭包很可能是子逻辑载体，应优先反编译 |

### 4.2 需裁定项（不涉及代码，交集成 owner）

- **D1（卡点缓解）**：`playermade_stairs_assemble<true/false>` 空反编译（0x1408c90/0x14018040）→ 其 4 个闭包 closure$8/9<true>/closure$24/25<false> 是发射函数，分派/循环逻辑大概率在其中。裁定：先反编译 4 个闭包再判定是否仍需 P-code 攻壳。
- **D2（卡点缓解）**：`WallAttachedDecoTracker` 40B params 布局未知 → with_diffs + construct_crossbrick_chimney/construct_door_stairs 反编译即可读出字段，不必单列「内联不可恢复」。
- **D3（证据一致性）**：A 有落盘证据（`evidence/toolchain/decor-trunk/decompiled/*.c`），B/C 无。裁定：B/C 结论应补落盘反编译证据（尤其 C1-C8 修正后），否则后续实现 agent 无法逐函数复核。
- **D4（closure 归类）**：bevy `_::impl$0::*`（ui_move_decorator 6 个）已在骨架中出现，按 AGENTS.md 归「Bevy 注册胶水」排除，但 ANALYSIS 应显式登记其存在，避免后续误判为漏项。
- **D5（历史负载类型）**：apply_decorator_op 历史缓冲元素为 `(DecoratorId, Vec<PreferredNodeAlignment>)`，与 A 版「push 历史 tag=3」并存——裁定历史条目 schema 按此补充。

### 4.3 与既有实现的冲突

- 未发现 ANALYSIS 三部分之间对同一函数的口径冲突（合并文件内容 ⊆ 三原分件，无丢失）。
- `reconstruction/building/decorator_subtype.rs` 自述「roof UV destinations and trigger construction are separate work」，与 A 版把 derive_decorator_subtype/is_half_dormer 整链标「可复用」存在**边界差异**：subtype 派生可复用，但 roof 触发/UV 部分仍在重建侧待闭合。裁定：可复用标注建议收敛到「subtype 判定」范围。

---

## 5. 遗留 unknown 汇总（复核后）

**维持 U（合理，无过度断言）**
1. `playermade_stairs_assemble<true/false>` 函数体（空反编译）→ 见 D1。
2. `calc_gothic_half_dormer_profile_ws` 剖面生成（反编译打散，需 P-code）——标注正确。
3. `derive_decorator_subtype` 跳转表精确映射、`is_decorator_location_valid` 跳转表分支——合理。
4. `WallAttachedDecoTracker` 40B params 布局 → 见 D2（改判可恢复）。
5. audio tag→FMOD 事件映射与 AudioEventKey、op_stairs_audio closure9 规则——合理（跨切面，global 卡点 #2）。
6. DecoratorBlueprint 全字段 TPI 布局、`+0x47/+0x280..282` 备份标志三态、DecoratorLocationCheckResult 变体/DecoratorRemoveInfo、DecoratorHistoryEdit 非 3/4/5 tag——合理。
7. `check_if_dormer_ridge_is_within_roof_bounds` 的「返回 1+脊世界点」两态——描述性 O，边界常量可复验。
8. GPU 后置（dormer SSBO 布局、lantern compare_light_lists/ColorId）——compare_light_lists 改判可反编译，SSBO 布局维持后置。

**修正/新增 unknown**
- 原「砖列主体（内联 U）」「assemble_dormer_roof_tiles（内联）」「compare_light_lists（内联 U）」「check_for_door_stairs（无符号）」四条**撤销**，改为主函数条目（§4 C1-C5）。
- 新增：各 UI 系统闭包（约 40 个）是否承载独立业务需逐条确认；`visualize_ui_stairs`/`ui_focus_stairs`/`playermade_stairs_assemble` 的闭包为主要候选。

---

## 6. 整体评价与建议

**评价**
- A 主干是本批次三份中质量最高：逐函数 O/I 诚实、算法点全、未知清单克制、有落盘证据；抽查 apply_decorator_op 与其描述逐条吻合。
- B/C 主函数覆盖也充分（B 全族主函数、C 楼梯 23+13+UI 系），但存在系统性「把发射函数写内联」的错误（C1-C8），集中于 decorator_visual 装配/差分子例程。这既造成 unknown 失真，也让「砖列/瓦片装配」这一核心算法被错误后置。
- 覆盖缺口主要是闭包层（约 40 个）：宿主描述正确，但闭包未逐条枚举，尤其对 body-U 系统（ui_focus_stairs、visualize_ui_stairs、playermade_stairs_assemble）闭包即逻辑载体，漏列会直接导致实现阶段缺证据。
- 未知清单总体合理，无明显过度断言；个别「O」描述（is_out_of_roof_bounds、check_for_door_stairs）在无证据文件时被降格/误标，属可修正偏差。

**建议**
1. 先修 C1-C8（改 ANALYSIS，不改代码），把 10 个被误标函数与 ~40 个闭包补进各自条目，撤销 4 条错误「内联 U」。
2. 对 body-U 系统优先反编译其闭包（D1：playermade_stairs_assemble 的 4 闭包；D4：ui_focus/stairs_viz 闭包），再决定是否需要 P-code 攻壳。
3. B/C 结论补落盘反编译证据（D3），保持与 A 的证据可追溯性一致。
4. 可复用标注收敛到 subtype 判定范围（4.3），避免与 reconstruction 侧边界混淆。
