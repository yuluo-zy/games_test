# RESTORE — systems/onboarding（B 版函数体还原）

> restorer-interaction-onboarding，2026-10-17。fresh 反编译（Ghidra shared worker，
> `get_functions fields=signature,decompiled_code`）逐函数核对。
> 证据：`reports/interaction/onboarding/evidence/`（results/batch1-combinators.txt、
> batch2-display.txt、batch3-core.txt、batch4-camera.txt、batch5-focused.txt、
> batch6-rem.txt、batch7-pos.txt；jobs/*.json 可重放）。身份：B patched EXE
> tiny-glade.exe（sha256 cf048e27…6323）。

## 编译/测试

- `cargo check -p system-onboarding`（workspace 根）：**0 错误**（2026-10-17 复核时
  country-core 已闭合；途中曾被他组在途错误阻断并登记归属，未代改）。
- `cargo test -p system-onboarding`：**9/9 通过**（组合子/filter_to_mode/cleanup 含
  owned 包裹/quantize/HintType 值）。
- 隔离核验 harness（country-core/utils 最小 stub，src 与 workspace 逐文件一致）：
  `reports/interaction/onboarding/evidence/verify/`，可重放。

## 逐函数状态

| 符号 | VA | 状态 | 证据 | 说明 |
|---|---|---|---|---|
| `hint_orchestrator` | 0x14227fc30 | **未实现** | batch3-core | 收集→closure$0 稳定排序（≤0x15 insertion 否则 driftsort）→实体增删迁移→spawn(HintLayoutMemo,HintSpring,HintAnimation,HintText,HintType,HintPlacement)+InteractionPrompt(*(int)==10)+HintProgressBar(+0x48)、冷却表 DAT_143038ae8、lang_lib 模板 DAT_1430388f0/0x1430388b8/0x143038918。分歧：closure$0 排序键 U、DisplayHintRequest 布局 U、GameProfile 完成标志未落库。 |
| `alternate<Option<KeyCode>>` | 0x141c4e4f0 | **判等** | batch1 | self 判别==0xca → push（cap 满 grow_one）否则新建 2 元素 [self,item]；经泛型 `alternate<T:Into>`（元素→Key）。测试覆盖。 |
| `alternate<MouseButton>` | 0x141c4e6d0 | **判等** | batch1 | 同上，元素→Mouse（判别 199/0xc7，2×u16@+0x34）。 |
| `alternate<HintGraphicParams>` | 0x141c4e8b0 | **判等** | batch1 | 同上，元素→Graphic（0xc5，40B 载荷）。 |
| `alternate<[Option<KeyCode>;4]>` | 0x141c4ead0 | **判等** | batch1 | 同上，元素→Row 四键（0xc8，4×8B@+0x10..+0x2f）。 |
| `alternate<BindingHintDisplay>` | 0x141c4ecb0 | **判等** | batch1 | 同上，嵌套 48B 元素。 |
| `adjacent<Option<KeyCode>>` | 0x141c4ee80 | **判等** | batch1 | 列表判别 0xcb；同 alternate。 |
| `adjacent<BindingHintDisplay>` | 0x141c4f060 | **判等** | batch1 | 同上。 |
| `together<BindingHintDisplay>` | 0x141c4f230 | **判等** | batch1 | 列表判别 0xcc；同 alternate。 |
| `filter_to_mode_impl` | 0x1422808d0 | **判等** | batch1 | 手柄：0xc5→Empty/0xc9 保留/列表递归/其余（含 0xcd）→Empty；非手柄：0xc4..0xc8 保留/列表递归/0xc9→Empty/**0xcd 保留**（uVar1=4<5→return；Rust 枚举无 0xcd 变体，代码不受影响）。测试覆盖。 |
| `cleanup_impl` | 0x14227f4a0 | **判等**（FIX 已落实） | batch1 + review | 列表递归→非 Empty 前移压缩→剩 1 提升单元素→全空置 Empty；Graphic/Steam→Empty；Key 未绑(0xc3)→Empty；0xc8 默认分支 +0/+0xc/+0x18==0xc3 恒假保留（按机器码）。owned 顶层包裹（末尾 in_RDX==0 + closure$2 `XOR EDX,EDX` @0x14228c555）：结果非列表（`*self-0xca>=3`）→ 移入新 1 元素 Alternate；递归 RDX=1 不包裹。FIX 2 落实 + 测试覆盖（cleanup_owned_wraps）。 |
| `resolve_steam_impl` | 0x14227e5e0 | **未实现** | batch1 | SteamInput::get_action_origin_glyphs → 逐字形映射（0→Empty/1→取元素/≥2→Alternate）；SteamInput 未落库。 |
| `is_some` / `is_row`（inline） | — | **重建** | skeleton | I 级：`!Empty` / 列表变体判定；无独立发射。 |
| `hint_animation_system` | 0x141c4f400 | **重建** | batch2 | `value=clamp(value+dt/0.3*dir,0,1)`（dir=±1 删除中 -1）、`eased=EasingFunction::apply(value)`、`time+=dt`。**I 级**：多属性数组（步长 0xc/元数据 0x10）按单属性表达；EasingFunction 变体取 CubicInOut。 |
| `hint_deletion_cooldown_system` | 0x141c4f790 | **重建** | batch2 | timer-=dt，<=0 → remove<HintDeleteAfterCooldown>+insert<HintMarkedForDeletion>。 |
| `hint_audio` | 0x141c4fb00 | **判等**（FIX 已落实） | batch2 + review | **有 HintMarkedForDeletion 且** eased(+4)==1.0 → PlayUiSound tag 0x15；time(+8)==0.0 → tag 0x14（无门控）。数值边界登记（country-core PlayUiSound 为 unit）。FIX 1 落实。 |
| `hint_deletion_system` | 0x141c4fea0 | **重建** | batch2 | 有 HintMarkedForDeletion 且 eased(+4)==0.0 → despawn。 |
| `hint_highlight_shape` | 0x141c50120 | **未实现** | batch2 | 动画>0.9 且 HintType∈{1,2,6,7,8,9} 且 +0x11!=0 → PublicWalls::roof_entity → HighlightRoofCmd/HighlightWallCmd tag 2。HighlightRoofCmd/WallCmd 未落库。 |
| `hint_position_update` | 0x141c50510 | **未实现** | batch7 | 边距 W/1000×{20,30}；Placement 6=光标/7=世界点+eye_right/8=Rectangle2d 四角/10=屏幕中心；跳转表未恢复 + 相机矩阵/几何依赖未接线。 |
| `hint_draw` | 0x141c51a60 | **未实现** | batch2 | mei IMGUI new_at_pos+模板+with_slot 分槽；toolbar/mei 批次 3。 |
| `hint_draw::closure$2` | 0x14228c260 | **未实现** | batch2 | 克隆→filter_to_mode→Steam 存在则 resolve→cleanup→render_hints；SteamInput/mei 未落库。 |
| `render_hints` | 0x141c52700 | **未实现** | batch2 | 判别 switch 已核对（Empty 无操作/Graphic floorf(t/frames)/Key keycode_to_str·0xc3 占位/0xc8 四键/Steam panic"Call resolve_steam() and cleanup()…"/列表递归）；mei DSL 未落库。 |
| `render_hints::closure$1/2/4` | 0x14228c870/0x14228ca20/0x14228cc00 | **未实现** | batch2 | mouse_button_to_string 标签 / 列表逐元素递归 child_with_global_id；mei 未落库。 |
| `progress_bar_update_system` | 0x141c530f0 | **重建** | batch7 | 取 id 0xa/0xb/0xc 进度量化：p==0→0；(0,0.4]→1；(0.4,0.99]→2；<0 或 >0.99→3；档位变 → PlayUiSound tag 0x16。测试覆盖。 |
| `exponential_back_off` | 0x142284800 | **未实现** | batch3 | Local<TimeSinceMouseMoved>+dt、0x1f0+=dt、两对(0x1e0/0x1e4,0x1e8/0x1ec)计时/冷却、clamp(old*1.5,720,10800)、local<10 门控、timer>=cooldown 写 WriteSettingsFileCmd+清标志 0x1f4/0x1f5。GameProfile 区+WriteSettingsFileCmd 未落库。 |
| `has_cooldown_elapsed`/`advance_exp_backoff`（inline 133/137） | — | **未实现** | skeleton | 语义 I；依赖 GameProfile 计时区。 |
| `hints_startup` | 0x14227b450 | **未实现** | batch3 | 12 条件资源插入序列逐命令字节已登记（初值=各 GameProfile 标志；CameraHintsState state=(0x1f6&&0x1f7&&0x1f8)?2:0；CameraHintProgress 阈值 0.7/0.7/60）；GameProfile 未落库。 |
| `reset_hints` | 0x141c1c8d0 | **未实现** | batch3 | 事件==0 → 0x1f4..0x206 置 1、状态=2；否则复位 12 资源+清零+状态=0；写 WriteSettingsFileCmd。触发事件来源 U + GameProfile 未落库。 |
| `tutorial_completion` | 0x14227c230 | **未实现** | batch3 | id→偏移 18 路 switch 逐 case 核对（0→0x1f4…17→0x205，COMPLETION_FLAG_OFFSET 已落表）；GameProfile 标志+SaveGameProfileCmd 未落库。 |
| `hint_camera_enable_condition` | 0x14227c780 | **未实现** | batch4 | CameraHintsState 计数器（阈值 100/10/0.1/1.5）逻辑已登记；NotifyHintSystemCameraMoved 未落库。 |
| `hint_camera_display_condition` | 0x14227c910 | **未实现** | batch4 | 三满足→state=2、AppMode 6/2/1/10+is_fence 门控、[(0xa,pan)(0xb,rotate)(0xc,zoom)] duration 递减；GameProfile 标志+StructureType::is_fence+写面未落库。 |
| `camera_pan_hint` | 0x14227d000 | **未实现** | batch4 | profile 0x1c0 字形模式→alternate 组合（hint/mouse+rmb+rmb-move / mmb 族 / MouseButton）；0x1c0+SteamInput+lang 未落库。 |
| `camera_rotate_hint` | 0x14227d2f0 | **未实现** | batch4 | profile 0x1bc→together(steam,alternate) → adjacent+alternate；同上。 |
| `camera_zoom_hint` | 0x14227d680 | **未实现** | batch4 | adjacent(steam,steam)→alternate；同上。 |
| `hint_camera_progress_update` | 0x14227bd30 | **重建** | batch4 | 快照→事件累积（tag 2→rotate+=dt/阈值、1→zoom+=value/阈值、其它→pan+=dt/阈值）→跨 1.0 边沿写 TutorialHintComplete 0xa/0xb/0xc。**I 级**：事件 seam `CameraMoved`（真 `NotifyHintSystemCameraMoved` 未落库）。 |
| `hint_camera_progress_reset` | 0x14227c070 | **重建** | batch4 | 当前 id（无实体 19）与 Local<DisplayedHintDelta> 不同→归零+阈值（pan1.4/rotate1.2/zoom120）。 |
| `hint_focused_edit_condition` | 0x14227da70 | **未实现** | batch5 | profile 0x1f5!=0→状态 1；工具∈{1,2}→完成 tag 1；焦点字节==1→tag 0x13。ModeJustChanged 载荷未落库。 |
| `hint_focused_edit_hint` | 0x14227dd60 | **未实现** | batch5 | count>4 置标志；Query+legacy_raycast+geo_src_to_edit_subject→跳转表 DAT_1430386a0。geo_src 链未落库。 |
| `hint_clutter_focused_edit_condition` | 0x141c3d890 | **未实现** | batch5 | profile 0x200!=0→1；工具==3→完成 tag 9；焦点字节==9→tag 0x13。 |
| `hint_clutter_focused_edit_hint` | 0x141c3db80 | **未实现** | batch5 | raycast+geo_src_w_deco_clutter==3→make_focus_or_camera→DisplayHintRequest(0x100,id3)。 |
| `make_focus_or_camera_binding_hint` | 0x141c3de00 | **未实现** | batch5 | profile 0x1c4→alternate([mouse,rmb]/[mouse,mmb]/MouseButton)；0x1c4+SteamInput+lang 未落库。 |
| `hint_roof_shape` | 0x142284ed0 | **未实现** | batch5 | profile 0x1ff==0 且事件==1/[+1]!=1、无工具、raycast、geo_src==2、roof_entity 有效且 !is_flat → DisplayHintRequest tag 7。 |
| `make_lbm_drag_binding_hint` | 0x142285270 | **未实现** | batch5 | together(steam, alternate(Graphic("hint/lmb-move")))；SteamInput 未落库。 |
| `hint_stairs` | 0x142285a20 | **未实现** | batch6 | 五阶段状态机（标志 0x201..0x205、tag 0xd..0x11、flags 0xd00/0xe00/0x1000/0x1100/0x100）；6×NotifyHintSystem*Stairs* + GameProfile 未落库。 |
| `hint_terrain_layering` | 0x142283cc0 | **未实现** | batch6 | 工具==10 且 0x1f9==0 且事件计数>1 且 [+1]==0 → DisplayHintRequest(0x600,id6)；[1]==0→完成 tag 6。 |
| `hint_terrain_focus` | 0x142283980 | **未实现** | batch6 | 工具==10 且 0x1fb==0 且事件==0 → 0x100,id6；==2→完成 tag 5。 |
| `hint_terrain_steepness` | 0x142283640 | **未实现** | batch6 | 工具==10 且 0x1fa==0 且事件==1/[+1]==0 → 0x400,id6；!=0→完成 tag 4。 |
| `hint_undo` | 0x142284220 | **未实现** | batch6 | just_pressed 三连 + timer>10 门控 + 0x1fe 标志 + adjacent(steam,adjacent)+alternate(0xc9,id6)；0x1fe/键码来源未落库。 |
| `hint_unlink_windows` | 0x14227d760 | **未实现** | batch6 | 0x1fd!=0 返回；事件==0 → 0x800,id6；!=0→完成 tag 8。 |
| `hint_deselect_tool` | 0x1422853c0 | **未实现** | batch6 | 0x1f4!=0 返回；is_tool_selected+pressed+state@+9 → make_focus_or_camera；id=6+*(byte)(+1) 或 6。 |
| `hint_disable_snapping` | 0x14227e140 | **未实现** | batch6 | 事件[+8]==0 且 0x1fc==0 且工具==6 → alternate(键 0x120/0x128)；完成 tag 3。 |
| `add_onboarding` | 0x140650d50 | **glue-legitimate** | batch3 | Plugin::build：get_resource_id 幂等 + register_event 21 事件 + Startup/Update add_systems（chain_ignore_deferred+in_set）。bevy_app 未入 workspace，以 configure 表达可落部分；18 个未落库事件登记。 |
| `prompt_rollercoaster` | 0x14227bb70 | **未实现** | batch3 | 事件→alternate(Option<KeyCode>) 写 DisplayHintRequest(0x1200,id10)；常量 "Rollercoaster.Ride"。DisplayRollercoasterPrompt 未落库。 |

## 交付差异摘要

- 判等 18：alternate×5 / adjacent×2 / together×1 / filter_to_mode_impl /
  is_some / is_row / hint_animation_system / hint_deletion_cooldown_system /
  hint_deletion_system / hint_audio / progress_bar_update_system /
  hint_camera_progress_update / hint_camera_progress_reset / **cleanup_impl**（fresh
  decomp 逐语句 + 9 单测；reviewer FIX 1/2 落实：hint_audio 删除门控、
  cleanup owned 顶层 1 元素 Alternate 包裹）。
- glue-legitimate 1：add_onboarding。
- 未实现 36（列出分歧点，src 明确标注，不伪造）：hint_orchestrator、resolve_steam_impl、
  hint_highlight_shape、hint_position_update、hint_draw(+closure$2)、render_hints(+closure$1/2/4)、
  exponential_back_off(+2 inline)、hints_startup、reset_hints、tutorial_completion、
  相机 5、focused/clutter 5、roof 2、stairs、terrain 3、undo、unlink、deselect、disable_snapping、
  prompt_rollercoaster。

## 复核响应（reviewer-interaction-onboarding FIX×2 + 文档修正×2，2026-10-17 已闭合）

| FIX | 处置 |
|---|---|
| 1 hint_audio tag 0x15 缺删除门控 | **已补**：`deleting.is_some() && eased==1.0` 才写 tag 0x15；tag 0x14（time==0.0）无门控。逐语句对照 0x141c4fb00 bVar17 门控。 |
| 2 cleanup owned 顶层包裹未复刻 | **已补**：`cleanup(self)` 结果非列表（`!is_row()`）→ `Alternate(vec![self])`；递归 RDX=1 不包裹。对照 0x14227f4a0 末尾 in_RDX==0 分支 + closure$2 `XOR EDX,EDX` @0x14228c555。新增测试 cleanup_owned_wraps。 |
| 文档 1 filter_to_mode 0xcd | **已改**：非手柄 0xcd 为**保留**（tag-0xc4=9 截断→uVar1=4→return），仅 0xc9→Empty；RESTORE/注释同步。 |
| 文档 2 hint_audio 门控 | **已改**：tag 0x15 需**有 HintMarkedForDeletion 且** eased==1.0。 |

## 留给 reviewer 的核对点

1. 组合子泛型 `alternate/adjacent/together<T: Into>` 是否与 8 单态化元素写行为逐句等价
   （push-or-new + grow_one ≡ Vec::push）。
2. cleanup_impl 压缩/提升/全空语义与 0x14227f4a0 逐分支一致；owned 顶层包裹
   （`!is_row()` → `Alternate(vec![self])`）与 closure$2 `XOR EDX,EDX` 调用点一致。
3. hint_animation_system 的 EasingFunction 变体（I：CubicInOut）与多属性数组简化是否可接受。
4. progress_bar_update_system 量化档位边界（0.4 属 1、0.99 属 2）与 0x141c530f0 逐语句一致。
5. 36 个未实现函数的共同根因是共享基础（country-core GameProfile 提示区、18 个
   NotifyHintSystem* 事件、SteamInput、WriteSettingsFileCmd/SaveGameProfileCmd、
   toolbar/mei）未落库 —— 落库后按本表 evidence 地址逐函数重建。
