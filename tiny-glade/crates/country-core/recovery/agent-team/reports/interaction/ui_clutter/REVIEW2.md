# REVIEW2 — systems/ui_clutter(二轮复验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer-interaction RESTORE.md + 新增 5 文件;一轮 FIX 落地复核。

## 逐项复验

### 1. delete_auto_clutter 删除门修复 — **OK(落地正确)**

- 删除集合插入已**无条件**(`deleted.0.insert` 先于录音判断)✓(原 rmb_autoclutter
  0x140976ca0 直接 push 删除事件+历史)。
- 录音门只影响写历史:`history.is_recording()` → records;否则 HistoryRecordingFailed ✓。
  一轮"未录音时跳过删除"的 FIX 项已正确解决。

### 2. 函数名映射登记 — OK

`defered_autoclutter_history`(实现名)= 原 `history_record_auto_clutter`@0x1409a9160 已在
doc/RESTORE/REVIEW 登记,不以改名代替闭合 ✓。

### 3. rmb_autoclutter @0x140976ca0 — PARTIAL(适配器,核心业务缺)

- 结构:消费 RmbAutoClutterHit(宿主解析实体 id)→ 删除集合 + 无条件历史(RecordAutoClutterHistory)
  ✓。
- **缺**(已标注):原 hashbrown SIMD 实体查找、旋转三向量点积/开方链、0x48B
  EvRightClickedAutoClutter 完整生成(pos/rot)依赖实体机制;`get_clutter_rot_in_roof_space`
  简化为 `dot.clamp(-1,1).acos()`(原 `func_0x000142923e20(...)^0x1429258f0` + FUN_1408b83d0
  链)。适配器语义与原函数不符,属结构占位。

### 4. ui_convert_autoclutter_to_decoclutter @0x14099ca60 — PARTIAL + **FIX(旋转输入疑似错)**

- 0x48B spawn 事件布局常量(tag=2@0、subtype=3@+0x29、type@+0x3c、entity@+0x40)登记 ✓。
- **疑似 FIX**:旋转 = `get_clutter_rot_in_roof_space(click.position.y)` —— 把 position.y 当
  三向量点积输入。原版旋转 = FUN_1408b83d0×4 + 常量区 UNK_1429cd1/1429258 的变换,
  position.y 与"点积"语义无证据对应。**待取证**(需 re-analyst 按 decomp 0x14099ca60 复核
  dot 输入来源),暂标 FIX-待取证。
- `AutoClutterParentMap` 是计数存根(行 410 原维护映射,宿主待接线)。

### 5. debug_spawn_clutter @0x1409a1f50 — PARTIAL(结构,常量/类型表未取证)

- 螺旋结构 42 项:`r=sqrt(n)`、`θ=n*SPIRAL_ANGLE`、cos/sin ✓(章节)。
- **FIX-待取证**:`SPIRAL_ANGLE = 2.399963`(黄金角近似,原 `_UNK_142abbcec` 精确值未 RNE
  取证);类型字节 `(&UNK_142aa0013)[n]` 以 `n%4` 假造代替(原逐项类型表);地形高度
  FUN_1409e9640 采样走宿主 DebugTerrainHeight。**不能**把螺旋位置近似计作闭合。

### 6. decoclutter_show_pins @0x1412357c0 — PARTIAL(结构,组件机制缺)

- 遍历 pinned 集合 → 可见性事件 ✓(结构);`register_required_components` **空存根**(
  原 @0x141237200 bevy 组件注册,宿主组件类型未接线);clone_behavior 折叠标记 ✓。
- 图钉实体可见性切换机制待宿主。

### 7. ui_focus_decoclutter @0x1409a92c0 — GAP(仅门存根)

- 当前 = tag!=0 门 + 空事件。原 11.3KB 状态机(RequestMoveClutter/RequestRotateDecorator/
  LastClonedClutter 移动/旋转、4 闭包)全缺。属真实 GAP。

## 结论与对 DISPATCH 建议

- 一轮 FIX(无条件删除、原名映射)已正确落地 ✓。
- 新 FIX/待取证:ui_convert 旋转输入来源、debug_spawn 螺旋角/类型表、decoclutter 组件注册
  空存根、ui_focus 门存根。
- 建议:状态 `in_progress`,待 re-analyst 补挖旋转/常量/类型表与宿主实体接线后复验;
  ui_focus_decoclutter 单标 GAP。
