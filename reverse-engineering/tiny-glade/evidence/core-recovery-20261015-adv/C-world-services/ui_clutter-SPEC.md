# ui_clutter-SPEC — 原 RMB picking + parent/roof transform + auto→decorator/stair 转换 + 消费删除 ID 的 generation/mesh owner（C 世界，2026-10-15 第二轮）

> 历史分析结果，供同组实时复合 agent 直接落地。已实现边界见 `evidence/core-recovery-20261015/ui-clutter/REPORT.md`（DeletedAutoClutter / DeleteAutoClutterCmd / RecordAutoClutterHistory / replay 均已就位；`DeleteAutoClutterCmd` 是显式 ID 适配器，**不是**原 RMB handler）。
> 证据源：`evidence/core-recovery-20261015/ui-clutter/{record,expand,replay}.asm`、`replay.compact.c`、`evidence/country-core-deep/ui-input/decomp/decomp-0x140976ca0.txt`、章节 `chapters/systems/ui_clutter/{rmb_autoclutter,ui_autoclutter,ui_focus_decoclutter,decoclutter_show_pins}.rs.md`、sysres v2。
> **跨组文件**：`resources/history/*`（EditType::AutoClutter 已存在，tag 13）属共享——本域已有登记，新增改动须仲裁。

## 状态判定

- **可直接落地**：`handle_rmb_autoclutter`（0x140976ca0，decomp 全文已读）；`ui_convert_autoclutter_to_decoclutter`（0x14099ca60，章节算法级）；`ui_focus_decoclutter`（0x1409a92c0）；`decoclutter_show_pins`（0x1412357c0）。
- **需先取证**：`maintain_autoclutter_parent_hashmap` 的完整映射表布局（I）；`get_clutter_rot_in_roof_space` 的精确旋转公式（I，常量区 UNK_1429cd1/1429258）；generation/mesh owner 的渲染消费（GPU 后置，先留真接口）。

---

## 1. 原 RMB picking：`rmb_autoclutter::handle_rmb_autoclutter` 0x140976ca0（O，decomp 297 行已读，可直接落地）

### 1.1 参数面（decomp 签名）

`FUN_140976ca0(param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9)`：
- `param_1` = 光标射线（float×4：`fVar23/fVar24/fVar27/fVar25`）；
- `param_2` = 点击实体状态（`*(int*)(param_2+0x18)==0` 判别无点击；`param_2+0x28` = 实体 id；`param_2+0x30/+0x38` = 位置数据）；
- `param_3` = 点击 clutter 索引（hashbrown 表，`param_3+0x48` 非零时有内容；`param_3+0x30` = 控制字节 ptr、`param_3+0x38` = mask）；
- `param_4` = 实体表（`param_4[1]` = 组件表，`*param_4` = 实体位集表）；
- `param_7` = 事件队列（push **0x48B 事件实例**）；
- `param_8/param_9` = History（经 `FUN_140a84ec0` = `History::add`）。

### 1.2 查找流程（hashbrown SIMD probe）

- `FUN_1405b0d70(param_3+0x50, &entity_id)` = hash；
- 控制字节 probe（16B 比较 + 位掩码折叠）→ 逐位命中检查：`*(longlong*)(lVar22 - 0x10 + idx*-0x10) == entity_id`；
- 命中条目：`uVar19 = *(ulonglong*)(lVar22-8+…)`（低 32 = 桶下标，高 32 = generation）；
- `param_4[1]`（组件表）+ `*param_4`（位集）：`(uVar19>>32)` 与桶下标双重校验；`in_R10 >> (idx & 0x3f) & 1` 位集判定实体存在；
- `uVar19 = *(uint*)(组件表 + idx*0x14 + 0xc)` = clutter 实体下标 → `param_3+0x110` 计数校验 → `lVar17 = 组件表(+0x1a8 表) + 下标*0x48 + …` = clutter 组件指针；
- `unaff_R14 = lVar17 + *(uint*)(组件表+idx*0x14+0x10)*0x58` = **目标 clutter 记录（0x58B 步长）**；
- 未命中路径 → `FUN_1428d9310(&UNK_142ab6950)` log + 走无点击分支。

### 1.3 旋转计算（O）

- `fVar30 = _UNK_1429a9088 * fVar27 + _UNK_1429a9084 * fVar24 + _UNK_1429a9080 * fVar23`（**三向量点积 a·p**，常量 @0x1429a9080..88 区 = 相机方向）；
- `func_0x000142923e20((…二次/交叉项…) ^ _UNK_1429258f0)`（**sqrt**，位翻转 0x1429258f0）；
- `fVar23 = FUN_1416f3210(&record+…)` → `fVar23 = _UNK_142a10ff0 - fVar23`；
- `fVar23 = sqrt(angle - fVar23)` → `fVar24 = sin`、`fVar25 = cos`（func_0x142923fd0/0x142923e80）；
- `uVar14 = FUN_1408b83d0(*(param_2+0x30), *(param_2+0x38))`（四元数/旋转位换算，见章节"经 FUN_1408b83d0 换算"）。

### 1.4 事件 + 历史（O）

**push 0x48B `EvRightClickedAutoClutter` 事件**（`param_7` 队列，`@+0x29 = 3` 载荷判别字节）：
```
@+0x00..0x0f：uStack_98/uStack_90（位置/实体数据）
@+0x10：uStack_88（向量）
@+0x18：uStack_80/uStack_7f（位置）
@+0x29：3（事件 payload tag）
@+0x2c：uVar26（旋转角）
@+0x30：*(param_1+4)（射线方向）
@+0x38：param_1[6]（float）
@+0x3c：uVar14（旋转位/旗标）
@+0x40：事件计数
```

随后 **alloc 0x18B Arc**：`*puVar16=1; puVar16[1]=1; puVar16[2]=uVar6(组件 ptr)` → `FUN_140a84ec0(param_8, &{0xd, puVar16}, &UNK_142ab6980)` = **`History::add` 直接记 tag 0xd（AutoClutter）**。

panic 分支（O）：`FUN_1428d9760(&UNK_142ab66f0, 0x2b, …, &UNK_142ab6968)`（**bounds 检查 @+0x1a8 组件表越界**）+ `swi(3)`。

### 1.5 与现有实现的差异（落地要点）

- 现有 `DeleteAutoClutterCmd`（显式 ID 适配器）**保留**作已解析 ID 路径，但**原 RMB handler 不同**：
  - 原版直接 push `EvRightClickedAutoClutter`（0x48B）+ 直接 `History::add` tag 13；
  - 原版**不触碰 `DeletedAutoClutter` 集合**（该集合由 generation/mesh owner 消费，见 §4）；
  - 原版记录走 `record.asm 0x1409a9160`（history_record_auto_clutter）所消费的 Vec<u64> 通道——`record.asm` 逐 ID alloc 0x18B Arc → `History::add` tag 0xd（`0x1409a9250..0x1409a9280`）。**落地**：RMB handler 命中后把 ID 推入一个 unread 列表，`defered_autoclutter_history`（现有实现）或新 record 系统消费并 `History::add`。

### 1.6 待取证（ui_clutter）

- `EvRightClickedAutoClutter` 事件的确切 Rust 类型名/字段（0x48B 布局已实，字段名 U）。
- `FUN_140a86270(param_8, param_9)` 在 RMB handler 中的作用（可能是 start/finish 录制节拍——现有 `History::start_input_recording` 0x140a86270 **同名同址**，即"开启输入录制"；RMB 末尾调用它 = 录制会话门控）。**落地**：复用现有 History::start_input_recording 语义接线。

---

## 2. parent/roof transform 解析 + auto→decorator/stair 转换：`ui_autoclutter::ui_convert_autoclutter_to_decoclutter` 0x14099ca60（O，8.1KB，可直接落地）

- 事件族：**InstaCreateDeco / InstaCreateStairPoint / StartPlaceDecoClutterInteraction / EvRightClickedAutoClutter**（wp20 合同）；
- 变换计算：`FUN_1408b83d0`×4 + 常量区 **UNK_1429cd1 / 1429258**（旋转/变换）→ 生成放置数据 → push **0x48B 事件**；
- 内联函数（无独立符号，行号揭示意图）：
  - **`get_clutter_rot_in_roof_space`**（行 403）：屋顶空间旋转查询/计算（精确公式 U）；
  - **`maintain_autoclutter_parent_hashmap`**（行 410）：维护 clutter→parent 映射（**parent 解析核心**；映射表布局 U）；
- panic：`FUN_1428d9940`（path/越界）；
- **落地**：实现 `ui_convert_autoclutter_to_decoclutter` 消费 EvRightClickedAutoClutter → 解析 parent（roof/ground）→ 生成 InstaCreateDeco（decorator 放置）或 InstaCreateStairPoint（台阶点）→ 与 ui_autoclutter 的 `StartPlaceDecoClutterInteraction` 联动。事件类型在全局事件 registry（本域先定义事件 struct）。

### 2.1 待取证（ui_clutter）

- `get_clutter_rot_in_roof_space` 精确旋转公式（常量 UNK_1429cd1/1429258 的数值语义）。
- `maintain_autoclutter_parent_hashmap` 映射表槽位布局。

---

## 3. UI 交互：`ui_focus_decoclutter` 0x1409a92c0 + `decoclutter_show_pins` 0x1412357c0（O）

### 3.1 `ui_focus_decoclutter`（11.3KB，58KB 反编译，10 参数 ECS 系统）

- 消费 `RequestMoveClutter` / `RequestRotateDecorator` / `LastClonedClutter`（wp20 合同），每请求移动/旋转对应 clutter 实体；
- 4 个闭包（O：闭包单态）——按请求变体分发：移动 gizmo 沿射线、旋转按光标角增量；
- 与 ui_edit_focus 的 deco/clutter 路径联动。

### 3.2 `decoclutter_show_pins`（6.7KB）

- 遍历 pinned clutter 集合，按 `ShowPinnedDecoClutter` 状态切换图钉实体可见性；局部 helper `FUN_141233f80`（图钉槽查找，I）；
- 事件/串区 UNK_142c1e9/142c1e6；`FUN_1428d9390`（索引分发表）；
- `register_required_components 0x141237200`（O）：bevy 组件注册（`DecoClutterLock` 等）；`clone_behavior 0x140002370` = ret 0。

### 3.3 待取证（ui_clutter）

- `RequestMoveClutter`/`RequestRotateDecorator`/`LastClonedClutter`/`ShowPinnedDecoClutter`/`DecoClutterLock` 的字段布局（事件名已见，字段 U）。

---

## 4. 消费删除 ID 的 generation/mesh owner（CPU 侧 consumer 边界，GPU 后置）

- **现状**：`DeletedAutoClutter(HashSet<u64>)` 为已实现权威删除集合（replay 时按删除 ID 应用 insert/remove）。generation/mesh owner 消费删除 ID 抑制渲染 clutter 尚未恢复。
- **真实 CPU 侧消费边界（本 SPEC 定义）**：一个资源/系统读 `DeletedAutoClutter`，对每个已被删除的 clutter id 标记其 generation 已失效、不再加入 mesh 实例化列表；**渲染（GPU 上传/绘制）后置**，但**先留真接口**（一个系统查询 DeletedAutoClutter → 输出 suppression 标志/集合）。
- `History::add_decorator_diff 0x140a854a0`（history 域）push tag 0xE/1——**decorator diff 表**，deco-clutter 删除经此进入 diff 链（跨域，经仲裁）。
- **待取证**：原版删除 ID 消费链的精确 owner（generation 计数资源 vs 直接读 HashSet）——本域先定义 `DeletedAutoClutter` 的只读消费接口，不编造实体删除。

---

## 5. 真实消费者接线清单

| 生产者 | 消费者 | 状态 |
|---|---|---|
| RMB 点击（输入域） | handle_rmb_autoclutter 0x140976ca0 → EvRightClickedAutoClutter(0x48B) + History::add(tag 0xd) | 未接线 |
| handle_rmb_autoclutter | history_record_auto_clutter 0x1409a9160（=现有 defered_autoclutter_history 语义）| 需把 ID 列表接到现有 record 系统 |
| EvRightClickedAutoClutter | ui_convert_autoclutter_to_decoclutter 0x14099ca60 → InstaCreateDeco/StairPoint | 未接线 |
| RequestMoveClutter/RotateDecorator | ui_focus_decoclutter 0x1409a92c0 | 未接线 |
| ShowPinnedDecoClutter | decoclutter_show_pins 0x1412357c0 | 未接线 |
| DeletedAutoClutter（已实现） | generation/mesh owner（suppression，GPU 后置留接口）| 未接线 |

## 6. 明确排除项

- GPU 渲染消费后置；不编造实体 despawn。
- 不改 `resources/history/*` 的 EditType::AutoClutter 现有 tag 13（已登记）；新增 decorator diff 变体须仲裁。
- `DeleteAutoClutterCmd` 适配器保留，但**不得**冒充原 RMB handler。
