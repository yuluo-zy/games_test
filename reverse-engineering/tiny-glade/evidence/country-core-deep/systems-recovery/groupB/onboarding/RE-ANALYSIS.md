# system-onboarding RE-ANALYSIS（组 B re-analyst，dbid tg_xb1，2026-10-16）

> 深挖范围：skeleton/B/crates/systems/onboarding（60 业务；15 个 register_required_components 为 Bevy 胶水，按 AGENTS.md 不计独立业务）。
> 新增反编译 60/60（decomp-0x14227*.txt / 0x141c3d*.txt / 0x141c4*.txt / 0x141c1c8d0 / 0x140650d50）。
> 前置：无（该 crate 无章节证据）。O=机器码证实 / I=推断 / U=未知。

## 1. 体系总览（O）

- **HintState 资源**（orchestrator/backoff 读写）：+0x18 u32（hint 步骤码，0xc4 哨兵）、+0x48 byte（显示中标志）、+0x49 byte（当前 hint 索引 0..0x12）、
  +0x1e4 f32（指数退避值，0 时置 720.0f=0x44340000 初值 0x142aba9ac）、+0x1f0 f32（总运行时间）、+0x1f5/+500(0x1f4) byte 标志。
- **Hint 列表/队列资源**（头 UNK_1430384d0）：各 hint_*_hint 经 `grow_one/grow_one_zeroed`（0x140562f30/0x140562870/0x140562630/0x140562c30）追加/清空；
  条目 = {2 个 fn 指针 + 载荷}（hint 显示闭包 + drop/cleanup 闭包，登记进 0x140562xxx 扩容的 Vec）。
- **Hint 候选查询**：0x58-stride hint 记录（hint_orchestrator param_2）；查询初始化 id UNK_143038710（FUN_141c58da0 / 141c58550 共用）。
- **每 hint 显示闭包表**（hints_startup 注册，0x142288xxx 族，hint_orchestrator.rs 内）：FUN_142288180/3d0/c90/bf0/1d0/610/8a0/380/c40/210/750/250…（~0x12 项，按 hint id 索引）。
- **BindingHintDisplay**（enum，alternate/adjacent/together 组合树）：构造入口 0x141c4e4f0/6d0/8b0/ead0/ecb0/ee80/f060/f230；
  resolve（Steam/普通解析）0x14227e5e0 递归树走、cleanup 0x14227f4a0（跳表 0x143038d04/d44/d64）、filter_to_mode 0x1422808d0（跳表 0x143038d84）；
  panic 串：`Call resolve_steam() and cleanup() on BindingHintDisplay before sending it in[to…]`（0x142f14706 后）。
- **输入绑定取键**：0x1409e8d80/0x1409e8d90（alias，country-core input action_set 族；0x1409e8d90(0xf)=吸附切换键、(2)/(3)=undo/redo、(0)/(1)=LBM/RBM…）。
- **帧消费门（cursor）模式**：与 critters 相同的两 Vec 游标布局（+0x10/+0x18、+0x30/+0x38、+0x40），"有剩余=用户在操作 → 重置计时"。

## 2. hint 触发函数清单（O，逐个取证）

| VA | 函数 | 触发/行为契约 |
|---|---|---|
| 0x14227c780 | hint_camera_enable_condition | Local{i32,acc,idle}；`state[0]!=0 → 早退`；事件有剩余 → 重置 idle 计时；idle ≥ 阈值(0x1429345a8) 才放行；state[0]=1 |
| 0x14227c910 | hint_camera_display_condition | `Local==1` 门；`state+0x1f6/0x1f7/0x1f8 == 1/!0/1 → state=2`（已展示）；mode ∈ {6+sub1, 6+sub2, 2, 1, 10} 且 `0x1408c5680` 谓词假 → 注册 3 个相机 hint（FUN_14227d000/2f0/680，id 0xb/0xc/3） |
| 0x14227d000 | camera_pan_hint | binding(0)+binding(1) → alternate/adjacent 组合；键图形 "hint/rmb-move…"（0x1430385d6 len 0xd）；常量 0.7（0x3f333333）/100.0（0x42c80000） |
| 0x14227d2f0 | camera_rotate_hint | binding(1) 组合 together(0x141c4f230)；同上图标串（0x1430385c4/5ce/5d6 三层 "hint/mouse/hint/rmb/hint/scroll…"） |
| 0x14227d680 | camera_zoom_hint | 同族（id 3） |
| 0x14227bd30 | hint_camera_progress_update | 进度累积 + hint 列表 push（0x1430384d0） |
| 0x14227c070 | hint_camera_progress_reset | 游标/进度重置；panic `internal error: entered unreachable code` @ hint_camera_progress.rs（0x1430384e8/548） |
| 0x14227da70 | hint_focused_edit_condition | 列表 grow/clear（0x140562630/0x140562c30） |
| 0x14227dd60 | hint_focused_edit_hint | **跳表分派**（0x1430386a0，按编辑状态 byte）→ 各状态构建 hint |
| 0x14227e140 | hint_disable_snapping | binding(0xf) → alternate<Option<KeyCode>>（0x141c4e4f0）→ push |
| 0x14227d760 | hint_unlink_windows | binding(0xf) → alternate → push |
| 0x141c3d890 | hint_clutter_focused_edit_condition | 列表 grow/clear |
| 0x141c3db80 | hint_clutter_focused_edit_hint | `func_0x000140002450` 谓词（焦点状态）→ push |
| 0x141c3de00 | make_focus_or_camera_binding_hint | binding(1) → alternate<HintGraphicParams>（0x141c4e8b0）+ alternate<MouseButton>（0x141c4e6d0，`param_2+0x1c6` 副键） |
| 0x142283640 | hint_terrain_steepness | **make_lbm_drag_binding_hint**（0x142285270）→ push |
| 0x142283980 | hint_terrain_focus | push（无 binding 构造） |
| 0x142283cc0 | hint_terrain_layering | binding(0xf) → alternate → push |
| 0x142284220 | hint_undo | `0x14093fa90(x,y)` 撤销谓词 ×2（状态+0x80/0x84/0x88/0x8c）；binding(2)+binding(3) → together(0x141c4f060)+adjacent(0x141c4ee80)+alternate(0x141c4ecb0) → push |
| 0x142284ed0 | hint_roof_shape | `func_0x0001408e3380`（屋顶形状谓词，0x1408e3xxx 屋顶族）→ make_lbm_drag → push；`FUN_14227ba70`（binding 校验，0x143038e88） |
| 0x142285270 | make_lbm_drag_binding_hint | binding(0)+binding(1) → together(0x141c4f230)；图标 "hint/lmb"+"hint/lmb-move"（0x143038ea0/ea8，0x20 槽分配）→ alternate<HintGraphicParams>(0x141c4e8b0) |
| 0x1422853c0 | hint_deselect_tool | `0x140a86b40`（工具栏谓词）+ `0x14093fa70(x,y)`（2 参谓词）→ grow/clear |
| 0x142285a20 | hint_stairs | binding(0xf) → alternate；binding(1)+binding(0) → alternate<HintGraphicParams>（图标 "hint/mouse"/"hint/lmb"/"hint/rmb"，0x143038ed8/ee2/eea）→ push |
| 0x14227bb70 | prompt_rollercoaster | binding(0x14) → alternate<Option<KeyCode>>（0x141c4e4f0）→ push（过山车提示） |
| 0x142284800 | exponential_back_off | **退避定时**：事件有剩余 → timer=0；idle timer < 0x1429cd034 → 累积；`state+0x1e4==0 → 置 720.0`；退避值驱动下一次 hint 展示间隔（0x140d8b480 浮点工具，日志串 0x143038e40） |
| 0x14227b450 | hints_startup | **注册 ~20 个 hint 显示/步进闭包**（FUN_142288180/3d0/c90/bf0/1d0/610/8a0/380/c40/210/750/250…，0x18/10/9/0x10/0x1c/0x2c 字节槽） |
| 0x14227c230 | tutorial_completion | 清空 hint 列表（0x1430384d0） |
| 0x141c1c8d0 | reset_hints | 清空 hint 列表 |

## 3. hint_orchestrator @0x14227fc30（O，主调度）

- 参数：param_1=状态（int，==0xd/==0x13 门）、param_2=Hint 查询（0x58 槽记录）、param_3/5=游标资源、param_4=候选查询、param_6=谓词资源、param_7=hint 任务 Vec。
- 流程：找候选 hint（param_4 查询首实体）；**门控**：候选查询游标有剩余 → 重置；`bVar13 = state==0xd || state==0x13 || 0x14093fbc0(*param_6)`。
- 无候选：按 `state+0x49`（当前 hint 索引）分派：==0x12 → 带图提示（FUN_140b9d480 分配、FUN_140d79f90 UI）；==2 → 带图；其余 → 通用文本；
  然后 `FUN_142289260`（更新显示）→ `state[0]==10` → push 步进闭包；`FUN_142282cb0(step)` + `FUN_1422890b0(resolve)`；`state+0x48!=0` → push 更多闭包。
- 有候选（pbVar28）：`bVar13 && id < 0x12` → 跳过（低 id 抑制）；`id == state+0x49`（已在显示）→ 更新显示闭包（FUN_142288420）+ `state+0x18 != 0xc4` → 步进 + resolve；
  否则 → push 切换闭包（FUN_142287f90/1422887b0 + `hint 参数表 0x143038ae8[id]`）。
- 0x143038ae8 = **per-hint 4B 参数表**（hint id → 参数，供显示闭包）。

## 4. hint_display_system（O，结构级）

| VA | 函数 | 契约 |
|---|---|---|
| 0x141c4f400 | hint_animation_system | 动画推进（游标模式 + hint 列表） |
| 0x141c4f790 | hint_deletion_cooldown_system | 冷却闭包注册（FUN_1422882a0/ea0/7b0/89af0 push 0x18/0x20 槽） |
| 0x141c4fb00 | hint_audio | 音频（列表 clear + 音频状态） |
| 0x141c4fea0 | hint_deletion_system | 删除（游标模式） |
| 0x141c50120 | hint_highlight_shape | 高亮形状（列表 grow 0x140562db0） |
| 0x141c50510 | hint_position_update | **曲线驱动屏位**：0x140949c10（曲线长）、0x140c9cde0/0x140c96df0（Rect/Vec2）、0x140ac8780（采样写位）——同 sheep_reaction_emoji 的 0x140ac8780 族 |
| 0x141c51a60 | hint_draw | 绘制：shader "hint"（0x142f1468f len 4）、文本 "text"（0x142f1468b）、图标/行 "icons"/"rows"（0x142f146a8/ad）；闭包族 FUN_14228a4e0/ab40/bb90/bfd0/cde0/89f80 |
| 0x141c52700 | render_hints | 渲染主：`0x142923ee0(x/y)` 除法/atan、FUN_14228a4e0（sprite，shader "hint"）、0x14228b3d0/bdb0（文本，"key"/"other/hint_wasd…" 0x142f14706/718）、0x140683c10（UI 上下文）、panic 0x142f14790 |
| 0x141c530f0 | progress_bar_update_system | 进度条（panic "internal error: entered unreachable code" hint_camera_progress.rs）+ 列表 clear |

## 5. 对其他 crate 的接口（restorer 接线）

- **Hint 显示图标资产**（render 侧）："hint/lmb"、"hint/lmb-move"、"hint/mouse"、"hint/rmb"、"hint/mmb"、"hint/scroll"、"hint/top_row"、"hint/row"（0x143038ea0/ed8/5c4… 串表）。
- **输入绑定取键** 0x1409e8d80/90：country-core input action_set（DigitalActionState::just_pressed 同族，见 hint_undo.rs 骨架注解）。
- **camera mode**（state 6/2/1/10 = MainCamera mode 值，与 critters 的 mode==3 同族）；`0x1408c5680`/`0x14093fbc0`/`0x14093fa90`/`0x14093fa70`/`0x140a86b40`/`0x140002450` 为 country-core 状态谓词（登记待身份）。
- onboarding build @0x140650d50：插件注册（panic 串 0x142b90460）。

## 6. 仍 U（不臆造）

- 0x14093fa90/fa70/a86b40/8c5680/93fbc0/002450 谓词的精确定义（country-core 侧，非本 crate）。
- hint 显示闭包族（FUN_142288xxx / FUN_14228xxx）内部 UI 布局细节（hint_orchestrator.rs 内私有，语义按 §3 分派表接线）。
- 0x143038ae8 per-hint 参数表内容（4B/项，20 项）逐项值。
- 0x140d8b480 在 exponential_back_off 中的精确语义（浮点 3 参工具，疑 clamp/log）。
- resolve_steam_impl/cleanup_impl/filter_to_mode_impl 的跳表目标表（0x143038d04/d44/d64/d84）逐项。
