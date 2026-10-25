# RESTORE — system-decorator ui_systems（D2 restorer tg_d2，批次 2 组 D）

## 交付摘要

- **落点**：`tiny-glade/crates/systems/decorator/src/ui_systems/`（16 业务文件 + mod.rs），
  顶层交互族（放置/移动/旋转/高度/楼梯/分裂）全部落盘；另补齐共享消费面
  （`DecoratorAffordanceDispatcher`/`MoveDecorationInteractionState`/
  `RaycastResources` 等 lib.rs 符号）。
- **证据**：`evidence/country-core-deep/systems-recovery/groupD/ui_systems/`
  ——33 个业务 VA fresh RE-MCP 反编译（`decomp/decomp-0x14*.txt`，dbid tg_d2）+ 
  4 个小函数（ui_inactive 0x140791400 / stairs_reset 0x14079c350 / dispatcher.clear
  0x1407af330 / SystemParam 胶水 0x1407a8550）NotFound → capstone 机器码手解 +
  6 个分裂/旋转处理器子函数跳表目标逐地址解算。
- **编译**：`cargo check -p system-decorator` 的 **ui_systems 零错误**；crate 级剩余
  错误全为 D1（decorator_visual/）、D3（cull_oob/remove_ivy/audio/dormer_holes/
  move_decorators_following_anchors）、D4（lib.rs）在途文件（互斥分区，见 DISPATCH §4）。
  依赖 country-core 增补（`UiState::add/add_with_id/widget/widget_context` +
  `UiWidgetContext::{clicked,hovered,is_active}` + `WidgetParams::volume`，均按
  ui-input 章 O 证据）随本次编译验证通过。

## SOURCE_MAP 对齐表（30 业务 / 33 唯一 VA；apply/new_archetype/queue 同址折叠）

| 源文件 | 函数 | VA | 落点 | 判定 |
|---|---|---|---|---|
| calculate_decorator_dst.rs | convert_raycast_hit_to_decorator_dst | 0x14127c990 | 同 | O（unreachable 串 0x142c240d8/0x142c24068） |
| calculate_decorator_grab_offset.rs | try_get_grab_offset_ss_if_valid | 0x141277bb0 | 同 | O 结构（跳表 0x142c23990 墙独立/其余同址） |
| calculate_decorator_grab_offset.rs | raycast_with_ss_offset | 0x141278050 | 同 | O 结构（遮罩 0x30ab/0x23、地形 0x1412773f0） |
| decorator_interaction_intent.rs | snap_balcony_door | 0x14127f0e0 | 同 | O 结构（几何 I） |
| decorator_interaction_intent.rs | calculate_decorator_interaction_intent | 0x14127cf70 | 同 | O 结构（case0 墙/1 屋顶门控；0x98B 输出 I） |
| handle_splitting_roof_decorators.rs | try_display_roof_decorator_parts_and_get_hovered_part_id | 0x14127a790 | 同 | O 结构（rank 遍历 + 指示符） |
| handle_splitting_roof_decorators.rs | handle_moving_out_roof_decorator_part | 0x14127bab0 | 同 | O 结构（0x14127c7f0 内层 u 回绕 [0,1)） |
| handle_splitting_wall_decorators.rs | display_decorator_wall_parts_and_get_hovered_part_id | 0x1412782b0 | 同 | O 结构（0x14125cf90 部件位置） |
| handle_splitting_wall_decorators.rs | handle_moving_out_wall_decorator_part | 0x141279410 | 同 | O 结构（0x14127a480 0x40B 记录） |
| move_ops.rs | show_decorator_remove_signifier | 0x14127fe10 | 同 | O 结构（48B 指示符记录） |
| move_ops.rs | calculate_move_dst | 0x1412801f0 | 同 | O 结构（跳表 0x142c248b0 四路） |
| ui_change_chimney_height.rs | ui_change_chimney_height | 0x1407ae2e0 | 同 | O 结构（相位机 + 0x12 op + apply） |
| ui_change_chimney_height.rs | show_signifiers | 0x1407ae910 | 同 | O（'chimney_height_'/‘height_gizmo_' 0x142a833b0/0x142a83410） |
| ui_change_stair_height.rs | ui_change_stair_height | 0x1407adea0 | 同 | O 结构（gate4+'?'@0x5c；跳表 0x142a832b0） |
| ui_decorator_affordance_dispatch.rs | DecoratorAffordanceDispatcher::clear | 0x1407af330 | 同 | O 机器码（写 {6,0}） |
| ui_decorator_affordance_dispatch.rs | ui_dispatch_decorator_affordance | 0x1407af340 | 同 | O 结构（state∈{0,3,4,11,20}；EscMenu is_open） |
| ui_decorator_affordance_dispatch.rs | create_decorator_widget\<&str> | 0x1412823c0 | 同 | O 结构（volume tag5 + flags bit1/bit3） |
| ui_decorator_affordance_dispatch.rs | create_decorator_widget\<DecoratorHandleName> | 0x141282300 | 同 | O 结构（0x14125d180 单态） |
| ui_focus_stairs.rs | ui_focus_stairs | 0x14079c6d0 | 同 | O 结构（'rotate_single_node_stairs_widget' 0x142a81fb0） |
| ui_inactive.rs | ui_decorator_inactive | 0x140791400 | 同 | O 机器码手解（gate 6 → 复位 + clear 尾调） |
| ui_move_decorator.rs | ui_move_decorator | 0x1407a4850 | 同 | O 结构（相位机 + 0x21 op + 跳表 0x142a829e8） |
| ui_move_decorator.rs | init_state | 0x1407a84d0 | derive 胶水 | 跳过（SystemParam derive） |
| ui_move_decorator.rs | get_param | 0x1403f8dd0 | derive 胶水 | 跳过 |
| ui_move_decorator.rs | validate_param | 0x1403f8c50 | derive 胶水 | 跳过 |
| ui_move_decorator.rs | apply/new_archetype/queue | 0x1407a8550 | derive 胶水 | 跳过（jmp 0x14003bb80） |
| ui_move_decorator.rs | find_stair_placement_candidate | 0x1407a7e80 | 同 | O 结构（0x50B 记录；遮罩 0x22） |
| ui_place_decorator.rs | ui_place_decorator | 0x1407a9c10 | 同 | O 结构（EvInit 事件对；跳表 case 3..0x14） |
| ui_place_decorator.rs | show_decorator_icon | 0x1407adb20 | 同 | O 结构（48B 记录） |
| ui_place_decorator.rs | decorator_cursor_icon | 0x1407adcb0 | 同 | O 结构（SignifierEntityBuilder 族宿主接线） |
| ui_rotate_lantern.rs | ui_rotate_decorator | 0x1407a8580 | 同 | O 结构（gate 3；跳表 0x142a82be8 四路） |
| ui_stairs_misc.rs | stairs_reset_for_new_frame | 0x14079c350 | 同 | O 机器码手解（state∈{4,5,11+'?'}） |
| ui_stairs_misc.rs | continue_stairs | 0x14079c3a0 | 同 | O 结构（0x88B 记录 → add_node/edge/endpoint） |
| ui_stairs_viz.rs | visualize_ui_stairs | 0x140791510 | 同 | O 结构（5 节点 + 0x130B 记录 I） |
| ui_stairs_viz.rs | insert_point_ui | 0x14079bc20 | 同 | O 结构 |

## 判定汇总

- **O（机器码证实）**：dispatcher gate 值域（3/4/6/11/20，128 位 state==N&&payload==0
  检查）、`dragged` 相位机（`bVar20=(c-2)<3?c-2:3`，释放→写 op）、复位写入
  `*[+0x10]←[+0x1c]`+`[*+0x48]=1`、signifier 记录 48B {1,1,enc_id,id,pos,scale}→
  `UiSignifierStream::write`（40B 拷贝）、0x28B op 记录 {seq,kind(0x12 烟囱/0x21 移动)}、
  gate 常量、跳表地址与索引公式 `(tag-2)<3?tag-1:0`、clear 写 {6,0}、'?'/':' 类型
  判别、遮罩 0x23/0x30ab/0x22、panic 串与行号。
- **I（结构级，细节登记）**：每个 dst 处理器的精确几何（四角吸附/合并/阳台门/
  旋转/分裂部件位置）、0x98B/0x130B 输出记录字段语义、`interaction` 指针槽宿主、
  SignifierEntityBuilder/SignifierFactory 实体机制（batch 3 toolbar/宿主）。
- **U**：无凭空臆造字段——未知字段一律 I 标注 + 原文偏移。

## 关键事实登记

- **gate 结构 = `DecoratorAffordanceDispatcher`**（非 DecoratorBackup）：ui_inactive
  state==6 触发 clear 尾调（jmp 0x140823660）；`DecoratorBackup::clear` 属 D3 模块
  （本 D2 面只消费 `clear()` + `events: Vec<[u8;0x88]>`）。
- **目的 tag 双空间（纠错）**：`DecoratorInfo` 目的 tag {0,1}=wall/2=roof/3=terrain/
  4=stair（0x1408334e0/0x140833410/0x140833470 机器码复核）；`convert_raycast_hit_
  to_decorator_dst` 输出为**自有 tag 空间** {0=wall,1=roof,2=terrain,3=stair-top,
  4=invalid}（16B {tag@0,payload@8}；调用方按 `tag!=4` 门控查各自跳表）——与 core
  `DecoratorDst` 不同，登记。
- **0x28B 与 0x20B 双队列（overlay 已由 D3 裁决）**：UI 系统推 0x28B
  `DeferredDecoratorOpQueue`（kind 0x12/0x21）；`apply_decorator_op` 0x141276e30
  写 0x20B `DeferredDecoratorEditHistory`（kind 3/4/5）+ 0x88B 事件。D2 曾误把
  0x28B 队列命名为 DeferredDecoratorEditHistory——D3 以排空面 0x140817850 证据
  更正，D2 的 0x28B 类型保留为 `DeferredDecoratorOp`。
- **History tag 纠错（D3）**：D2 初稿把 apply_decorator_op 的 0xd/0xe/0x10 读为
  History tag；实际为归档标签串长（"place preview"=13 等，rne-read 实证）——D3
  更正，本交付采纳。
- **写字串（O）**：'ui_decide_decorator_interaction'（0x142a835e6）、
  'rotate_single_node_stairs_widget'（0x142a81fb0）、'chimney_height_'/
  'height_gizmo_'（0x142a833b0/0x142a83410）、'link'（0x6b6e696c，0x14079b860）。
- **panic 串（O）**：convert_raycast_hit 的 "Decorators should ignore other
  decorators when raycasting..."（0x142c240d8/0x142c24068）；旋转/高度/分裂的
  file 串（0x142a82ae0/0x142a83298/0x142a83380/0x142c23ae0）。

## 跨领域线索（供中央索引）

- 调用 core：`DecoratorStorage`（lookup_storage 0x140a1c3e0 / decorator_info
  0x140a1c5f0 / wall_decorators 0x140a1e190 / apply_decorator_place/remove/change
  0x140a20c70/0x140a20670/0x140a20890）、`UiSignifierStream::write` 0x140b2cf90、
  `UiWidgetContext::{is_active,clicked,dragged}` 0x14092e680/0x14092e5f0/0x14092df10、
  `UiState::add/add_with_id` 0x1409236d0/0x14092e3e0、`WidgetParams` 族、
  `StairsState::{add_node,add_edge,try_set_endpoint}`、`CachedDecoratorTransforms`、
  `PublicWalls::get` 0x140a91ef0、`RoofTriggerVolume`、`TerrainHeightsData`、
  `EscMenuState::is_open` 0x14093fbc0、`MainCamera::xz_projective_rotation`
  0x140911e10、`WindowSize::as_vec2` 0x140949c10、`TransientDecoratorIdGen`、
  `DeferredDecoratorOpEvents`、`DecoratorArchivist`。
- core 缺口登记（宿主接线，非本批）：`RequestRotateDecorator`（ui_clutter）、
  `NotifyHint*`（onboarding）、`SignifierFactory/SignifierEntityBuilder`、
  `WallCornerAttachment`、`DecoratorOutline`。

## 错位/纠错

- skeleton lib.rs 把 `apply/new_archetype/queue` 记为独立业务 VA（0x1407a8550）；
  机器码 = 单条 `jmp 0x14003bb80`（Bevy SystemParam derive 胶水）→ 按 AGENTS.md
  不计业务，跳过。
- 旋转/楼梯高度/烟囱高度各 dst 处理器（0x1407a8766/0x1407a87a6/0x1407a8814/
  0x1407a91bb、0x1407ad91a/0x1407ad998/0x1407adba7）为函数尾部内联段（Ghidra
  NotFound，<40B 未建函数）→ 跳表目标逐个 capstone 解算，处理器以 I 登记。
