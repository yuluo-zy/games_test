# REVIEW — systems/terrain + ui_clutter 真实逻辑重建(rebuild-terrain 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-terrain REBUILD.md×2(terrain 7 + ui_clutter 5 全真业务,fresh 反编译);
`cargo check --lib` exit 0(独立复验,13 警告)。

## 逐项复验

### ① edit_terrain_stroke / terrain_stroke_changed — PASS
- 删自造 PlayTerrainSound 载荷事件:原版把 tag 0/1/2/3/4 观测写 **StairsPlacementBucket**
  (indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind> 0x28B),**无音频载荷事件**;
  terrain_stroke_changed 补 tag=0 记录,顺序 8B 事件→readback LOCK INC→stairs 桶 ✓。
- 保留 recalc/readback/magnitude(delta·√/0.001)/apply_resolved;PlayTerrainSound 类型/
  注册保留(audio 模块依赖,红线不改他模块),生产面已停写 ✓。

### ② terrain_hover_idx — PASS
输出布局改 **{tag u64@0, id i64@+8}**(fresh `*puVar4=1; puVar4[1]=iVar7`,8B tag+8B id,
原 id@+4 错)✓;函数开头**无条件 tag=0 复位** ✓;门 `mode&0x1f` 移位掩码 ✓。

### ③ ui_draw_terrain — PASS
画布门反向修正 **`height>0 ∧ (¬hovered ∨ dist≤0.5)`**(非悬停帧原版也注册)✓;prev u8 回绕
修正 `(byte)(prev-1)>1 / <2`(prev=0→255 算>2,早期 `prev>2/<=2` 相反)✓;结尾 jumptable
state 分派(1=新笔 new_stroke / 2=续笔 insert_or_morph / 3=释放 / 0=无动作)✓。

### ④ ui_edit_terrain — PASS(部分)
逐笔画 `UiState::add<String>`(type=`(iVar37!=id)*3+4`);事件 subtype switch
(&DAT_142ae404c case 0-4/6/8/0xc/0xf);stairs 桶 tag=2 写;case 3 TerrainEdit 哨兵
**0x8000000000000001** + 校验上一哨兵 0x...0002 panic;**删自造 MouseActivityTimer/16 位
脏掩码→EditTerrainStrokeCmd/SetTerrainStrokePtsCmd**(原版脏判定=事件队列非空)✓;
case 4 UNRECOVERED_JUMPTABLE 待取证(诚实)。

### ⑤ ui_terrain_focus_mode — PASS(部分)
**fresh 本轮反编译成功(120KB)**:门 = `AppMode != 5`(Editing(Terrain) 判别 5),早期
"状态首 u32==1"未复现已修正 ✓;closure$1 "vertex_color_shader" signifier + bevy command
push `insert<DrawTopology>` 登记 ✓;复杂体(焦点 HashMap/as_curve3/CircularF32/
RequestAppModeChange/相机旋转/AppendOnlyVec::push<UiAppendSignifier>×4)依赖宿主实体机制,
登记待接(诚实)。

### ⑥ terrain_rock_mask — PASS
越界改 **panic_bounds_check**(`index>>5 < len`,原 unwrap_or(0) 静默)✓;**词位读取**
(mask[index>>5] 的位 index&31,每 u32=32 格,原按整 u32 词返回错)✓;`x/130.0` DIVSS(原
x*(1/130))✓;clamp NaN 保持 → as u32 饱和 0(不用 f32::clamp)✓。

### ⑦ ui_clutter 5 函数 — PASS
- rmb:旋转改 **atan2f 链 + fmod + 半角 sin/cos**(删自造
  `|sqrt(-2wy-2x²)-(-π/2-fmod(entry))|·0.5`,fresh 无对应);probe 未中 **unwrap_failed
  panic**(原 log+continue);AutoClutterRecord 补 z/roof_space/local 输入 ✓。
- ui_convert:函数内维护 parent 映射(ResMut AutoClutterParentMap,原独立资源无人填充)✓。
- debug_spawn:两循环 **History::start_input_recording()**;螺旋 rotation@+0x2c=0(原
  n·SPIRAL_ANGLE 错);单条位置=事件载荷 uStack_9a(不采地形高度);螺旋高度经
  TerrainHeightsData::sample_at_world_xz 直采 ✓。
- decoclutter_show_pins:几何/探测/条件门依赖宿主 ECS(登记待接);双 Vec 缓冲 +
  PinVisibilityToggle 保留 ✓。
- ui_focus_decoclutter:门改 **disc==3**(原 tag==0 错);DecoratorStorage::get +
  [result+0x48]!=0 门;补 PlayDecoratorSoundCmd/RequestAppModeChange/RequestRotateDecorator/
  RequestMoveClutter/UiAppendSignifier 多路事件 ✓。

## 结论与 DISPATCH 建议

- terrain + ui_clutter 真实逻辑重建复验通过:**12 个需重做函数全部闭合**,编译 exit 0
  (独立复验)。
- 建议:terrain/ui_clutter 需重做项 → 闭合;登记待接项(focus_mode 复杂体、ui_edit case 4
  jump table、decoclutter 几何/实体 spawn、stairs/decorator 消费链宿主接线)归后续卡。
  红线遵守:判等 10 函数未动、PlayTerrainSound 类型保留供 audio 模块后续。
