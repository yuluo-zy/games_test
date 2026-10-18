# RESTORE — mei 收尾（restorer-mei-final）

> 2026-10-18。身份：B patched EXE `cf048e27…6323`，Ghidra `--worker shared` 只读，
> 未写任何数据库。硬标准：逐函数 fresh 反编译 → 逐语句/逐指令判等；取不到即登记，
> 不落近似体。证据：`work/restorer-mei-final/{jobs,results,evidence}/`。
> 红线：只改 `crates/systems/toolbar/src/`；未改 shared(country-core)、未改他模块。

## 0. 结论速览

| # | 面 | VA | verdict |
|---|---|---|---|
| 1 | `render_taffy_ui_to_widgets` 焦点交换（`staged_focus`） | 0x142338c60..cc6 | **判等** |
| 2 | 同上：per-widget keep 面（`UiInteractionState+1`） | 0x1423394c3..515 | **判等** |
| 3 | 同上：无焦点→首个活跃 UiNav 按钮接管 Toolbar 焦点 | 0x142339890..904 | **判等** |
| 4 | 同上：共享尾部 `UiClickEvent{entity}`（所有 widget） | 0x142339910..976 | **判等** |
| 5 | 同上：共享尾部 drag 事件 `{entity, state}` stride 0x18 | 0x14233997e..9d2 | **判等** |
| 6 | 同上：`expand = GameConfig.ui.icon_size_mult * 10 * size_mult` | 0x142339522 | **判等** |
| 7 | `mei_pre_update`：父子链 clear + `children→parent` 重建 | 0x1406845ac..6709 | **判等** |
| 8 | 同上：焦点门控 + `process_input_action_handlers` 链式双遍 | 0x140684874..497a | **重建**（2B 状态实参 U） |
| 9 | 同上：`clicked()` else 焦点二项（`mei+0xc8` just_pressed） | 0x140684f90..fcd | **判等** |
| 10 | 同上：retain(0x13..0x17+hold) / hover 焦点保持 / drag 三位 | 0x140684d0e..f66 | **判等**（fresh 复核） |
| 11 | `mei_post_update`：Mei 焦点节点缺失→清焦点 | 0x140689bc8..d01 | **判等** |
| 12 | 同上：4 方向 `DigitalActionState` pairwise 公式 | 0x140689e04..a1fd | **判等** |
| 13 | 同上：全节点扫描（默认焦点/any/first/blocked/current/center） | 0x14068a261..a35a | **判等** |
| 14 | 同上：`iter_subtree<GlobalId, closure_env$0>` | 0x140f9e3e0 | **判等** |
| 15 | 同上：父链上溯 + swizzle 轴向 + 兄弟段前后/逆序规则 | 0x14068a46f..a6a4 | **判等** |
| 16 | 同上：`iter_subtree<(), closure_env$2>` 候选比较 | 0x140f9e610 | **判等** |
| 17 | 同上：命中 → `nav.focus = Mei(candidate)` | 0x14068aca8 | **判等** |
| 18 | 同上：`as_mei` → 焦点节点 `state |= 0x40` | 0x140689d1c / 0x14068a9e8 | **判等** |
| 19 | 同上：z 比较器 | 0x140fe4040 | **判等**（`f32::total_cmp`） |
| 20 | 同上：`focus.is_none()` 的 qword 语义 | 0x140985f30 | **判等**（tag+pad==0） |
| 21 | `slider3d` 消费面（is_some/is_started/is_finished；`!=2` 事件） | 0x14092df10..6d0 | **判等**（已在 10/5 内，fresh 复核） |

**计数：判等 21 / 重建 2 /（新增）未实现 3 + 登记 4。**
`cargo check -p toolbar` **0 error**；`cargo test -p toolbar --lib` **15 passed / 0 failed**
（新增 4 个 update_nav/父子链测试）。

## 1. 三项"剩余未取证"的本轮结果

| 旧登记 | 结果 | 证据 |
|---|---|---|
| `pressed`/cursor 来源 | **已取证**：焦点交换用 `ActionSetStates.digital[0]`（"click"）`.pressed()` 与 `analog[1]`（"hardware_window_space_cursor_move"）`.length() > 1.0` | 0x142338c69..ca4；常量 0x142925904 = 1.0 |
| `update_nav` 闭包 | **已取证**：closure_env$0 = `flags & 0x20 (with_nav)` 短路递归；closure_env$2 = `delta=(pos+size*0.5)-focus_center`，`dot=delta.normalize_or_zero()·dir.normalize_or_zero()`，`dot>0 && len<best` 取最近 | 0x140f9e3e0、0x140f9e610 全函数反汇编 |
| z 比较器 | **已取证**：`insertion_sort_shift_left<&mut Node, impl$17::post_update::closure_env$0>` 对 `[Node+0xac]` 做 `v ^ ((v>>31)>>1)` 无符号比较 = `f32::total_cmp` 升序 | 0x140fe4040..40f5 |

另：`is_toolbar` = `tag==1`；`as_mei` = `tag==2 → payload`；`is_null`（UiFocus）=
`qword [focus]==0`（tag+pad）；`just_pressed`/`pressed` 均逐指令核对。

## 2. 关键语义（实现依据）

### 2.1 `update_nav`（0x140689bb9..0x14068aca8）
1. `focus.tag==2` 且 payload 节点缺失 → 焦点 qword 清零（payload 不写）。
2. `focus.tag==1`（Toolbar）→ 跳过导航输入。
3. 方向（`mei+0xd0/d8/e0/e8` = left/right/down/up 内存序）：
   `left = js(d0)&&!js(d8)`、`right = js(d8)&&!js(d0)`、
   `down = js(e0)&&!js(e8)`、`up = js(e8)&&!js(e0)`；
   **`dir = (right-left, down-up)`**（旧注释 `up-down` 为反向，已纠）。
   负向集合 `[0x38] = up|left`，水平集合 `[0x98] = left|right`。
4. 全节点扫描：`flags&8` → blocked+默认焦点；clicked(`state&0x10`) 时按
   `default_focus/with_nav` 组合；`with_nav` → 首个节点；焦点命中 → 中心
   `pos+size*0.5`。
5. 焦点修复：缺失清空；候选子树含 with_nav → 设焦点；无焦点且有 with_nav →
   设首个 with_nav。
6. 移动门：`current_found && !blocked` 且恰好一个方向 且 `focus.tag==2`。
7. 父链上溯：`parents: HashMap<GlobalId,GlobalId>`（child→parent）；
   父 `style.swizzle==0`（XY）⟺ 水平输入，否则上行一层；取当前节点在父
   `children` 的位置 idx：
   - 负向：`children[..idx]` **逆序**（近→远）；
   - 正向：`children[idx+1..]` **原序**。
   逐兄弟 `subtree_search`，命中即写焦点。
8. `iter_subtree` 两处：closure0 为 with_nav 短路；closure2 遍历**全部**子孙
   （无短路），仅 with_nav 节点参与候选。

### 2.2 `mei_pre_update` 父子链
`mei+0x1d0` 的 hashbrown 表是 `HashMap<GlobalId,GlobalId>`（stride 0x10），
不是旧模型的 12B handler。帧首 clear → 遍历节点表 `children` 逐项
`insert(child, node_id)`。`process_input_action_handlers(focus_id, action)`：
由 focus 沿 parent 链收集并 reverse（root→focus）；第一遍状态 `{0,0}`，
第二遍（focus→root）状态 `{1,handled}`，命中即返回。

### 2.3 `render_taffy_ui_to_widgets`
- 焦点交换条件：`focus.is_toolbar() && (digital[0].pressed() || analog[1].len>1.0)`；
  `staged_focus = focus`；`*(qword*)(focus)=0`（**payload 保留**）。
- keep（所有 widget）：`=0`；若 `focus.tag==1 && payload==entity.to_bits()` → `=1`。
- 焦点接管：迭代前快照 `focus.is_none()`；逐项 `state==1 && is_active &&
  Option<UiNav>.is_some()` → keep=1；当前焦点仍空 → `focus={Toolbar, entity}`。
- click/drag 事件在**分支汇合后的共享尾部**对所有 widget 发射。

## 3. 未实现 / 登记（本轮新增）

1. **未实现**：绘制后的焦点矩形链（`as_mei` + `new_edit_sequence` 0x140a82b20
   → `fill_color` 0x140b920b0 → `encode_no_images` 0x140b92220 → `rect`
   0x140b8f510；条件 `flags&0x40 && nav.hold`）——依赖 rhapsody 编辑序列面，
   本 crate 无承接 API。
2. **未实现**：`SignifierFactory::new` 的 inner/marker 闭包胖指针（0x14089d8b0
   调用点已判等，但闭包体/落点未展开）；保留已判等载荷构造，不伪造闭包。
3. **未实现**：`impl_17::perform_layout` /
   `compute_min_size_with_children<Map<Copied<...>>>`（0x140fe0e10）映射闭包
   （沿用上一批登记）。
4. **登记（U）**：`mei+0xd0..0xef` 4 个方向 `DigitalActionState` 与 `mei+0xc8`
   点击动作的**写入方系统**未取证；消费面已按 fresh 判等落库
   （`MeiContext.nav_actions` / `click_action`）。
5. **登记（U）**：`process_input_action_handlers` 的 vtable 第三实参 2 B 状态
   `{0,0}`/`{1,prev}` 未进入 `NodeContent::on_input` trait；当前唯一实现为
   `EmptyContent` 空壳，无行为差。解锁条件：widgets 内容实现落库后扩参。
6. **登记（U/I）**：默认焦点/首项 with_nav 依赖 `HashMap` 迭代序；Rust 与原生
   hashbrown 桶序不同，多候选时取胜者 U。`WidgetParams` volume `extra_78`
   字段身份 U（I 投影）。
7. **登记（U）**：`UiDragEvent` 的 Ghidra `grow_one` 符号为
   `tuple<&str, SettingsMenuState>`，但机器码载荷为 `{entity u64@+8, state u8@+0x10}`
   （stride 0x18）；实现按机器码，符号名作 U。

## 4. 代码变更

| 文件 | 变更 |
|---|---|
| `src/mei/core.rs` | `handlers: HashMap<GlobalId,InputActionHandler>` → `parents: HashMap<GlobalId,GlobalId>`；新增 `nav_actions[4]`/`click_action`；`process_input_action_handlers(focus_id, action)` 链式双遍重写；`build_parent_links()` 替代 `snapshot_child_ids()`；测试更新 + 2 新测试 |
| `src/mei/systems.rs` | pre：父子链重建、焦点门控动作分发、clicked 焦点二项；post：`update_nav`（方向/默认焦点/父链兄弟搜索/两个闭包/焦点写）、焦点位写、`f32::total_cmp` 排序；新增 4 个测试 |
| `src/ui_interaction.rs` | 焦点交换、keep 面、焦点接管、共享 click/drag 尾部、`icon_size_mult` 扩张；`UiDragEvent{entity,state}`；文档判等更新 |
| `src/lib.rs` | 模块状态说明追加 restorer-mei-final 段 |

## 5. 编译 / 测试

```
cargo check -p toolbar        # 0 error（toolbar 侧 0 warning；country-core 14 条既有 warning）
cargo test  -p toolbar --lib  # 15 passed; 0 failed
```

## 6. 证据索引（`work/restorer-mei-final/`）

- `jobs/fresh-disasm.json`、`disasm3.json`、`proc.json`、`proch.json`、`iter.json`、
  `tiny.json`、`closure2.json`、`grow.json`、`mem*.json` + `results/`。
- `evidence/mei_pre_update.asm.txt`、`mei_post_update.asm.txt`、
  `render_taffy_ui_to_widgets.asm.txt`（1061/1739/902 指令）。
- `evidence/iter_subtree_closure0.asm.txt`、`iter_subtree_closure2.asm.txt`、
  `insertion_sort_shift_left_0x140fe4040.asm.txt`、`process_handlers.asm.txt`、
  `tiny-helpers.asm.txt`、`constants.txt`、`helper-*.c`。
