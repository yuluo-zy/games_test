# RESTORE — gap: utils 数学（rotation_from_normal / orientation_mat3）+ toolbar 模块内类型/事件载荷面

> restorer-gap-utils-toolbar，2026-10-17。任务：实现深度 gap 中不依赖 rhapsody 稳定的部分
> （utils 数学 + toolbar 模块内类型/事件）。硬标准逐语句判等；还原不了标未实现（分歧点）；
> 他组在途文件出合同+登记，不硬写。
> 方法：ghidra `--worker shared` fresh 反编译（`evidence/jobs/fresh-utils.json` +
> `fresh-toolbar-buttons.json`，本目录 evidence/）+ 复用 wave3/reviewer 既有 fresh 反编译
> （`reports/wave3/toolbar/evidence/`，2026-10-17 同日独立复核通过）。身份：B patched EXE
> （ghidra shared 核对：`tiny-glade.exe`，function_count 131517）。

## 0. 结论速览

| 交付 | VA | 判定 | 状态 |
|---|---|---|---|
| `utils::rotation_from_normal` | 0x140c90a40 | **判等（4 case 半 sqrt 逐语句 + 有效掩码 有限非零）** | 已落 `crates/utils/src/geometry/mod.rs` + 测试 |
| `Rectangle2d::orientation_mat3` | 0x140c99570 | **判等（9 store 逐字段）** | 已落 `crates/utils/src/geometry/rectangle.rs` |
| toolbar per-widget stride-3 字节记录容器 | — | **判等（布局）；dense↔per-entity 分歧 I** | `WidgetByteState`/`PerWidgetButtonState`/`FocusedWidgetSubtree` 已落库 |
| `UiNav` / `HistoryButton` 组件 | — | 类型已落库；布局 U（typed gap） | 已落库 |
| 事件载荷面 `WindowFocusEvent` | 0x1423427f0 | **判等**（1B 载荷、`EventInstance` stride 0x10；`RequestToolbarCategoryChange` newtype） | toolbar 侧引用 country-core |
| 事件载荷面 `RequestAppModeChange` | 0x142341fe0 | **16B `AppMode`**（reviewer-gap-round FIX-1；`grow_one<tuple>` 为 ICF 伪名） | core-events 组回退；toolbar 空壳仅引用类型 |
| `toolbar_button_keep_focus` keep 字节 | 0x1423414b0 | **判等（算法换真实 keep 字节）** | 已改 |
| `ui_animation_state_update` 字节判别 | 0x14233b3f0 | **判等（公式+字节判别）** | 已改 |
| `switch_toolbar_category_button` 事件写面 | 0x1423427f0 | **判等（WindowFocusEvent push + `+2` 选中）** | 已改 |
| `tod_v2_button` `+2` 写序 | 0x142342f30 | **判等（值=visible；FIX-3 写序移到 toggle 后）** | 已改 |
| `switch_app_mode_button` | 0x142341fe0 | **未实现**（载荷源 U，保持诚实壳） | 未改核心 |

**类型落库 6**：`WidgetByteState`（stride-3 记录）、`PerWidgetButtonState`（dense 容器）、
`FocusedWidgetSubtree`/`FocusedWidget`（focused-window 子树表）、`UiNav`、`HistoryButton`。

**编译**：`cargo check -p utils -p toolbar` **0 错误 / 0 警告（两 crate 侧）**（13 警告均在
country-core 依赖侧，非本批）；`cargo test -p utils --lib` **92/92 过**（含新增
rotation_from_normal 3 用例：z_axis/degenerate/overflow）；`cargo test -p toolbar --lib`
**2/2 过**（LogSlider + stride-3 容器用例）。**并发阻断记录**：country-core
`mode_manager.rs`/`resources/window.rs` 由 mode_manager/core-events 组在途（FIX-1 载荷回退中）；
timberframe 文件由 tg-tr 组持有（beam_rotation 换用合同见 §6）。

## 1. `utils::rotation_from_normal`（0x140c90a40，935B）· 判等

### 1.1 函数语义
法向→四元数（把默认方向 Z 轴旋转对齐到 `normal`；梁/朝向消费面，timberframe 裙层/屋脊/
circle 梁）。**判等证明**：函数输出的 4 分支半 sqrt 四元数 = `Quat::from_mat3(&Mat3::from_cols(
(w.y, w.z, w.x), c, n))`（逐 case 手算对 fresh 反编译 + glam 0.29.3 实测双证），其中：

- `w = normalize(n.x, -n.z, 0)`，退化（`n.x==n.z==0` 或平方溢出 inf → `inv` 非有效）→ `(0, 1, 0)`；
- `c = normalize(n.y·w.x - n.z·w.z, n.z·w.y - n.x·w.x, n.x·w.z - n.y·w.y)`，退化 → `(0, 1, 0)`。

glam `from_rotation_axes`（DirectXMath `XMQuaternionRotationMatrix`）的 4 分支
`0.5/sqrt(...)` 与反编译 4 case 逐语句对应（fresh 0x140c90a40）：

| 反编译 case | 判定条件 | 分量 |
|---|---|---|
| A | `nz>0 && wy+cy'≤0` | `(f(wx+nx), f(cz'+ny), f·s', f(wz-cx'))`，s'=(nz+1)-(wy+cy') |
| B | `nz>0 && wy+cy'>0` | `(f(cz'-ny), f(nx-wx), f(wz-cx'), f·s'')`，s''=s+nz+1 |
| C | `nz≤0 && cy'-wy≤0` | `(f·t', f(cx'+wz), f(nx+wx), f(cz'-ny))`，t'=(1-nz)-(cy'-wy) |
| D | `nz≤0 && cy'-wy>0` | `(f(wz+cx'), f·t'', f(ny+cz'), f(nx-wx))`，t''=(1-nz)+(cy'-wy) |

守卫：三分量非有限 → panic（`ABS<0x7f800000`）；输出四分量任一非有限 → 单位四元数
（`movmskps != 0xf`）。

### 1.2 测试值（fresh 手算 + glam 双证）
| n | q (x,y,z,w) |
|---|---|
| (1,0,0) | (0.7071068, 0, 0.7071068, 0) |
| (0,0,1) | (0, 0, 1, 0) |
| (0,0,-1) | (1, 0, 0, 0) |
| (0,0.5,0.8660254) | (0, 0.258819, 0.9659258, 0) |
| (0.6,0.8,0) | (0.6324555, 0.3162277, 0.6324555, -0.3162277) |
| (0,1,0)（退化 v） | (0.7071068, 0, 0, -0.7071068)（-90° 绕 X；Z→n 仍成立） |

**FIX-2（reviewer-gap-round，已修）**：归一化有效掩码原生 = 位型 `[0x1, 0x7f7fffff]`
（**有限且非零**）；原实现仅 `is_finite()`，在 `inv==0`（分量有限但平方溢出 inf，如
n=(3e38,0,3e38)）时行为相反。已改两处为 `is_finite() && != 0.0`（`inv=1/sqrt(≥0)` 非负，
`!=0.0 ≡ >0.0`），退化路径 `(0,1,0)` 与原生一致；新增 `overflow_normal_uses_degenerate_fallback`
测试。**附注（I，非阻断）**：c 归一化平方和顺序原生 `z²+y²+x²`，实现已按原生顺序显式写
（glam `Vec3::length` 为 x²+y²+z²，单精度非结合，极端输入位级或不同）。输入不要求单位化
（原函数不归一化 n，直接用分量作矩阵项）。

## 2. `Rectangle2d::orientation_mat3`（0x140c99570，74B）· 判等

反编译 9 处 store（列主序）：`[0]=axis.x, [1]=0, [2]=axis.y`；`[3..5]=0x3f80000000000000`
→ 列 1 = Y `(0,1,0)`；`[6]=-axis.y, [7]=0, [8]=axis.x`。Rust：

```rust
pub fn orientation_mat3(self) -> Mat3 {
    Mat3::from_cols(Vec3::new(self.axis.x, 0.0, self.axis.y), Vec3::Y, Vec3::new(-self.axis.y, 0.0, self.axis.x))
}
```
**逐字段判等**（`[3]`=0.0、`[4]`=1.0 的 u64 双字面已核对——列 1 确为 Y，非 X）。

## 3. toolbar per-widget stride-3 字节记录容器 · 判等（布局）+ 分歧（I）

### 3.1 证据（本 restorer fresh 0x1423414b0/0x14233b3f0/0x1423427f0/0x142342f30）
- keep 字节 `+1`：keep_focus `*(char*)(col+1+id*3)` 读（行 173/214）与写 `=1`（行 421/453）；
  ui_animation `pcVar20[1]`（行 209）；
- appear 符号 `+0`：ui_animation `cVar7 = *pcVar20`（行 242），`==0` → 负号；
- 选中标志 `+2`：switch_category `*(bool*)(col+2+id*3) = (toolbar.current==category)`
  （行 232/298）与动画插入门 `!= '\x01'`（行 209/275）；tod_v2 `*(undefined1*)(col+2+id*3)
  = *(byte*)(TimeOfDayScreen+0xa)`（行 185/218，+0xa 偏移分歧登记）。
- focused-window 子树表：窗口组件 +0x50 子列表 `{entity u64@+0, widget_id u32@+8}`，
  stride 0x10（fresh 各函数 fetch 面）。

### 3.2 落库
- `WidgetByteState`：repr(C) 3 字节 `{appear_sign@+0, keep@+1, selected@+2}`（size 断言 3，
  offset_of 测试）；
- `PerWidgetButtonState`：dense 容器 `Vec<WidgetByteState>`，`records[widget_id]`（B 模型
  `widget_id * 3` 字节列），访问器 get/set_keep/set_selected/appear_sign（写自动扩表）；
- `FocusedWidgetSubtree` + `FocusedWidget{entity, widget_id}`：focused-window 子树表。

**分歧（I 级）**：B 为窗口实体 dense 数组（Query 三列 cooldown/交互/字节记录 + 子树 id 迭代）；
本实现以 per-entity `WidgetByteState` 组件承载同一三字节（bevy Query 归一化，三系统签名
已按 B 三列对齐为三组件 Query）。子树表接线后按 `PerWidgetButtonState` dense 模型换回。

## 4. toolbar 组件/事件

- `UiNav` / `HistoryButton`：类型已落库（`resouces_and_components`），**布局 U（typed gap
  0x10）**——render_taffy_ui_to_widgets `Option<&UiNav>/Option<&HistoryButton>` Query 面与
  link_up_nav/UndoRedoLayout 消费面目标类型具备。
- 事件载荷面（reviewer-gap-round 复核后修正）：
  - `RequestToolbarCategoryChange(pub WindowFocusEvent)`（stride 0x10，fresh
    `grow_one<EventInstance<WindowFocusEvent>>`）——**判等**（newtype 名义事件，1B 载荷
    `WindowFocusEvent`）；`WindowFocusEvent(pub u8)` 落 `country_core::resources::window`——
    **判等**。toolbar 侧不重复定义，直接引用。
  - `RequestAppModeChange`——**16B `AppMode`**（reviewer-gap-round FIX-1：原
    `grow_one<tuple<ref<str>,enum2<SettingsMenuState>>>` 符号是 **ICF（相同代码折叠）伪名**，
    `EventInstance<AppMode>`(24B) 与 `EventInstance<(&str,SettingsMenuState)>` 的 grow_one
    机器码同形；决定性证据 = `check_if_can_switch<mode_manager::closure_env_3>` 0x140afa2c0
    把末条事件 16B 整块直写 AppMode resource 并按变体 tag 分派）。**该载荷面回退归 core-events
    组**（`mode_manager.rs`）；toolbar 的 `switch_app_mode_button` 为空壳、仅引用
    `EventWriter<RequestAppModeChange>` 类型名，不受载荷字段影响。此前 RESTORE 的
    `(String, SettingsMenuState)` 表述作废。

## 5. toolbar 系统更新（补字节写面 / 事件写面）

- `toolbar_button_keep_focus`：Query 加 `&mut WidgetByteState`，keep 字节判别替换
  `cooldown>0` 代理（FIX-2 代理非判等→判等）；步骤 3/4（any_keep→keep==0 冷却 0、
  cooldown>0→keep=1）落库。`uVar6`（tick）写 UiInteractionState U 登记。
- `ui_animation_state_update`：Query 加 `&WidgetByteState`，keep 字节 → `entering`、
  `+0` → appear 符号（替换 hardcode `entering=false/sign=+1` 单路径，FIX-F 补分派）。
- `switch_toolbar_category_button`：click 匹配 → `RequestToolbarCategoryChange(
  WindowFocusEvent(button.category as u8))`（原"click→WindowFocusEvent 未实现"→判等）；
  per-widget `+2` 选中字节替换 `prev_selected` 代理；FIX-C 两门保留。
- `tod_v2_button`：per-widget `+2` = visible 写面落库；**写序 FIX-3**（reviewer-gap-round）：
  原 Rust 在 click/toggle **之前**写 `+2`（同帧拿到 toggle 前值），已把 per-widget 写循环
  移到 click 处理**之后**（原生：click 块内先 `visible ^= 1` 并插动画，随后 per-widget 写
  `+2`）；`Query<Entity, &T>` 过滤器修正为 `With<T>`（bevy 0.16 `&T` 非 QueryFilter——wave3
  遗留编译错误，本批修正登记）。
- `switch_app_mode_button`：**未实现**（FIX-B 降级保留）——B 载荷为 16B `AppMode`（FIX-1），
  载荷来源按 `check_if_can_switch` 消费面待 core-events 组回退；本函数为空壳、仅引用
  `EventWriter<RequestAppModeChange>` 类型名，不伪造事件写。

## 6. 冲突 / 合同登记（他组在途，不硬写）

- **country-core `systems/mode_manager.rs` + `resources/window.rs`**：mode_manager 组在途
  （2026-10-17 载荷面修正，消费接线可能仍中途——本批 `cargo check` 时已一致编译）。
  toolbar 侧只引用，不编辑、不重复定义。
- **`crates/systems/timberframe/`**（timberframe_rectangle.rs/circle.rs）：tg-tr 组持有
  （wave3-timber-roof 在途）。**合同**：`beam_rotation`/`orientation_mat3` 本地镜像换用
  `utils::rotation_from_normal` + `utils::geometry::Rectangle2d::orientation_mat3`——
  换回路径：垂直/等距梁 `rotation_from_normal(rect.axis 3D)`、裙层/屋脊梁
  `rotation_from_normal(edge.normalize())`（原 I 级意图等价升为逐语句判等）。
- **`WidgetByteState` per-entity 组件 vs dense 容器**：subtree 迭代接线归 toolbar 后续
  （render_taffy_ui_to_widgets 13 参接线），容器类型已备。

## 7. 编译 / 测试

- `cargo check -p utils -p toolbar`：**0 错误 / 0 警告**（两 crate 侧；13 警告均在
  country-core 依赖侧，非本批）。
- `cargo test -p utils --lib`：92/92 过（含 rotation_from_normal `z_axis_maps_to_normal` /
  `degenerate_vertical_normal` / `overflow_normal_uses_degenerate_fallback` 3 用例）。
- `cargo test -p toolbar --lib`：2/2 过（LogSlider + `widget_byte_state_layout_and_accessors`）。

## 8. 证据索引（`reports/gap/utils-toolbar/evidence/`）

- `jobs/fresh-utils.json` + `fresh-utils.result.json`：identity + 0x140c90a40 / 0x140c99570
  反编译 + xrefs。
- `jobs/fresh-toolbar-buttons.json` + 结果：identity + 0x1423414b0/0x142341fe0/0x1423427f0/
  0x142342f30/0x14233b3f0 fresh 反编译（存 `fresh-*.c.txt` 五份）。
- 复用 wave3 fresh：`reports/wave3/toolbar/evidence/*.c.txt`（reviewer 独立复核通过）。

## 9. reviewer-gap-round 修正（2026-10-17）

> 复核报告：`reports/gap/review/REVIEW.md`（reviewer-gap-round，fresh 证据
> `work/reviewer-gap-round/`）。本 restorer 判等项：orientation_mat3 / toolbar stride-3 记录 /
> keep 四步 / ui_animation 公式 / switch_category 复核通过。需修 2 项已落实：

| FIX | 处置 | src |
|---|---|---|
| FIX-2 `rotation_from_normal` 掩码 | **已改**：两处 `is_finite()` → `is_finite() && != 0.0`（有效 = 位型 `[0x1,0x7f7fffff]` 有限非零；`inv==0` 溢出输入与原生同走退化 (0,1,0)）；c 归一化平方和按原生 `z²+y²+x²` 顺序显式写；新增 overflow 测试 | utils/src/geometry/mod.rs |
| FIX-3 `tod_v2_button` `+2` 写序 | **已改**：per-widget `+2` 写循环移到 click/toggle **之后**（原生 click 块内先 `visible ^= 1` 再 per-widget 写，同帧读到 toggle 后值） | toolbar/src/toolbar_button.rs |

**FIX-1（`RequestAppModeChange` 16B `AppMode`）不属本任务**：归 core-events 组回退
`country-core/src/systems/mode_manager.rs`；toolbar `switch_app_mode_button` 为空壳、仅引用
类型名，不受影响（§4 已纠正此前 `(String, SettingsMenuState)` 表述）。
