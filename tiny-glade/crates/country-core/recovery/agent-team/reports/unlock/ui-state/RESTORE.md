# RESTORE — unlock-ui-state（UiNavState / Ui2dExtent / slider3d::DragState）

> agent: `unlock-ui-state`，2026-10-17。身份：B patched EXE
> (`cf048e27…6323`)，Ghidra/radare2 只读，未写任何数据库。
> 硬标准：fresh 反编译/机器码逐字段判等；未取证项登记，不造近似体。
> 证据：`work/unlock-ui-state/{jobs,results,evidence}/`（本批 fresh）。
> 红线：只改 `country-core` 的 ui 资源/组件与 `toolbar` 消费代码；未改 shared / 他模块。

## 0. 三态速览

| 项 | 原生 | verdict | 落点 |
|---|---|---|---|
| `UiNavState` | 0x38 B | **已落库（逐字段判等）** | `country-core/.../resources/widget_params.rs` |
| `Ui2dExtent` | 0x10 B | **已落库（pos/size 判等）** | `country-core/src/components/trees.rs` |
| `DragState` + `into_drag_state` | 1 B enum / 26 B 函数 | **已落库（逐指令判等）** | `country-core/.../resources/ui_state.rs` + `slider3d.rs` |
| mei hover / drag 状态位 | — | **已接线** | `toolbar/src/mei/systems.rs` |
| retention（hold）/ extent / drag 消费 | — | **已接线** | `toolbar/src/mei/systems.rs`、`ui_interaction.rs` |

编译：`cargo check -p country-core -p toolbar` → **0 error**
（country-core 14 条既有 warning，本批未新增）。
测试：`cargo test -p country-core --lib ui::resources` → **14 passed / 0 failed**；
`components::trees` → **1 passed**。`cargo test -p toolbar --lib` 见 §7（后台执行）。

## 1. 证据（fresh，只读 shared）

| VA | 名称 | 证据文件 |
|---|---|---|
| 0x1406842e0 | `mei_pre_update` | `evidence/0x1406842e0-mei_pre_update.c`、`evidence/mei_pre_update.disasm.txt` |
| 0x1406884c0 | `mei_post_update` | `evidence/0x1406884c0-mei_post_update.c` |
| 0x142338990 | `render_taffy_ui_to_widgets` | `evidence/0x142338990-render_taffy_ui_to_widgets.c` |
| 0x14233eed0 | `ui_nav_update` | `evidence/0x14233eed0-ui_nav_update.c` |
| 0x140fcdae0 | `Vec::retain<ActionSetAction, mei_pre_update::closure_env$2>` | `evidence/disasm-retain.txt` |
| 0x14092df10 | `slider3d::impl$1::into_drag_state`（ICF = `UiWidgetContext::dragged`） | `evidence/0x14092df10-into_drag_state.c` |
| 0x14092e6b0/c0/d0 | `DragState::is_some/is_started/is_finished` | `evidence/0x14092e6*.c` |
| 0x1422979b0 | `color_button_animations`（Ui2dExtent 写面） | `evidence/0x1422979b0-color_button_animations.c` |
| 0x140946b80 | `MainActionSet::deconstruct`（digital/analog 布局核对） | `evidence/0x140946b80-deconstruct.c` |

## 2. `UiNavState` 0x38（逐字段）

fresh 机器码（mei_pre_update）：
- `movzx eax, byte [rax+0x30]`（0x140684992）→ hold 字节送入 retain 闭包；
- `cmp dword [rax+0x10], 2`（0x140684f97）→ focus tag；
- `mov rax,[rax+0x18]`（0x140684fa4）→ focus payload/id；
- `call Instant::now; mov [rcx+0x20],rax; mov [rcx+0x28],edx`（0x1406849df/e3）；
- `cmp edx, 0x3b9aca00`（0x1406849e6）→ `Option<Instant>` 的 None niche（nsec=1e9，
  与 `Instant::elapsed` 0x140d63910 的 niche 处理一致）；
- `cmp rax, 0x78; setb`（0x140684a12）→ `elapsed.as_secs() < 120`；
- `mov byte [rcx+0x30], al`（0x140684a42）→ hold 写回。

| 偏移 | 字段 | 类型 | 证据/语义 |
|---|---|---|---|
| +0x00 | `staged_focus` | `UiFocus`（16 B） | `render_taffy` 0x142338990：`focus==Toolbar && (pressed || cursor.len>1)` 时把 +0x10 的 16 B 拷入并把 +0x10 清零；`ui_nav_update` 0x14233eed0 反向提升（L550-561）。名称 I |
| +0x10 | `focus` | `UiFocus`（tag u32@0x10、pad@0x14、id u64@0x18） | 见上；另 `ui_nav_update` 写 `[+0x10]=1`+`[+0x18]=payload`，清焦点写 `qword[+0x10]=0` |
| +0x20 | `timer` | `Option<Instant>`（16 B，None niche nsec=1e9） | `Instant::now` 存 `rax@0x20/edx@0x28`；`elapsed` 读同一 16 B |
| +0x30 | `hold` | `bool` | retain 捕获；`mei_post_update` `cmp byte [nav+0x30],1` |
| +0x31 | `_pad` | `[u8;7]` | 对齐到 0x38 |

更新语义（精确）：
```
if 本帧任一动作 just_pressed(slot ∈ {0x13,0x14,0x15,0x16,0x17}) { timer = Some(Instant::now()) }
hold = timer.map_or(false, |t| t.elapsed().as_secs() < 120)
```
retain 条件（0x140fcdae0）：`!(just_pressed(0x13..0x17)) || hold`；
任一命中即置 flag=1（无论是否保留）。

`sizes/offsets` 由测试 `nav_state_offsets_match_native_0x38` 锁定
（host `size_of::<Option<Instant>>() == 0x10`，与 Windows/Unix std 的
`Timespec{secs i64, nsec u32}` niche 一致）。

## 3. `Ui2dExtent` 0x10

`render_taffy_ui_to_widgets`（0x142338990）逐寄存器：
- 元素步长 `idx * 0x10`（`puVar22 = uVar17*0x10 + local_f8`）；
- `uVar3 = *puVar22`（+0x00 Vec2）→ `pos`；
- `uVar2 = puVar22[1]`（+0x08 Vec2）→ `size`；
- `half = (size + expand) * 0.5`；`rect = pos - half .. pos + half`。

`color_button_animations`（0x1422979b0）同样以 `idx*0x10` 取元素，分别写
`+0x00/+0x08` 两个 Vec2（位置插值/尺寸缩放）。

```rust
#[repr(C)] pub struct Ui2dExtent { pub pos: Vec2, pub size: Vec2 } // 0x10
```
单测 `components::trees::tests::ui2d_extent_matches_native_0x10`。

## 4. `DragState` / `into_drag_state`

PDB 符号：`country_core::systems::ui::resources::ui_state::DragState`
（`is_some` 0x14092e6b0、`is_started` 0x14092e6c0、`is_finished` 0x14092e6d0），
`slider3d::impl$1::into_drag_state` 0x14092df10 与 `UiWidgetContext::dragged`
同址 ICF。

fresh 字节：
```
is_some     : 80 39 02 0f 95 c0 c3   cmp byte [rcx],2; setne   → !=2
is_started  : 80 39 03 0f 94 c0 c3   cmp byte [rcx],3; sete    → ==3
is_finished : 80 39 02 0f 92 c0 c3   cmp byte [rcx],2; setb    → <2
into_drag_state (0x14092df10):
  48 8b 11            mov rdx,[rcx]        ; UiState*
  80 7a 18 02         cmp byte [rdx+0x18],2
  72 0e               jb  ret(2)
  4c 8b 42 38         mov r8,[rdx+0x38]    ; active widget id
  4c 3b 41 08         cmp r8,[rcx+8]       ; context id
  75 04               jne ret(2)
  0f b6 42 19         movzx eax,byte [rdx+0x19]
  c3                  ret
```
判别：`0/1 = finished`（`clicked` 只认 1，`0x14092e5f0` 的 `&1`）、`2 = None`、`3 = Started`。
落库为 `#[repr(transparent)] pub struct DragState(pub u8)`（保留任意原始判别值），
方法逐指令对应；`UiWidgetContext::into_drag_state()` 定义在 `slider3d.rs`
（原生模块归属），`ui_state::dragged()` 保留 `u8` 兼容面（其它 crate 仍按数字比较）。

mei 消费（0x140684f09..f66）：`into_drag_state` → `is_some` 写 bit2、
`is_started` 写 bit1、`is_finished` 写 bit3（每位置位前先清位），已接线。

## 5. 代码变更

| 文件 | 变更 |
|---|---|
| `crates/country-core/src/systems/ui/resources/widget_params.rs` | `UiNavState(u64)` → 0x38 `repr(C)` 结构（staged_focus/focus/timer/hold）+ `focused_id()`/`update_hold()` + 2 个布局/语义测试 |
| `crates/country-core/src/components/trees.rs` | `Ui2dExtent{_gap:u64}` → `{pos:Vec2,size:Vec2}` + 偏移测试 |
| `crates/country-core/src/systems/ui/resources/ui_state.rs` | 新增 `DragState`（1 B）+ `is_some/is_started/is_finished`，`drag_state_raw()`；`dragged()` 保持 u8 兼容 |
| `crates/country-core/src/systems/ui/resources/slider3d.rs` | `pub use DragState` + `impl UiWidgetContext::into_drag_state()` |
| `crates/systems/toolbar/src/mei/systems.rs` | retain(hold/0x13..0x17) + `update_hold`、hover 焦点项、drag 三位接线 |
| `crates/systems/toolbar/src/ui_interaction.rs` | `extent.size` 参与 rect；`into_drag_state()`+谓词替代魔数 |

## 6. 未实现 / 残余（诚实登记）

1. `mei_pre_update` clicked 的焦点二项：fresh 0x140684f90 读
   `MeiContext+0xc8/+0xcc` 的 `DigitalActionState`（非 UiNavState 字段）；
   `MeiContext` 未承载该槽 → 未接线（core-layout-gap，已改注释）。
2. `mei_post_update` 中 4 个 `DigitalActionState::just_pressed` 的载荷源与
   `UiFocus::as_mei` 消费（0x1406884c0 L1056/1202/1424/1602）——类型已可用，
   但本批未展开完整控制流，保持未实现。
3. `render_taffy` 前段焦点交换（`focus==Toolbar && pressed/cursor`）：
   `pressed` 来源为宿主参（`in_stack_00000050` 邻域），未在本签名内取证。
4. `update_nav` 两遍 `iter_subtree` 闭包体、z 比较器、taffy `perform_layout`
   映射闭包仍未取证（与 mei-body RESTORE 一致）。
5. `Ui2dExtent.pos` 已可读；`WidgetParams::new` 的 volume 载荷是否使用
   `pos±half` 仍需按 fresh 逐参判等（本批只以 `size+expand` 填 size）。
6. `staged_focus` 的字段真名为 I；行为（交换/暂存）有 fresh 证据。

## 7. 文件在途合同 / 并行登记

- 本批进行中检测到 `crates/systems/toolbar/src/ui_interaction.rs` 在我方
  读取后被并行进程改动（mtime 23:35，warn: modified on disk）；我方编辑
  （extent/drag 段）在该版本上 **clean applied**，未覆盖其它区域。
- 合同：该文件本轮编辑边界仅限
  `render_taffy_ui_to_widgets` 的 extent/drag 两个消费点与对应文档注释；
  若并行 owner 需要回退/重排，以本 RESTORE §5 的行为判等为准。
- 未改 `country-core` 以外他模块；未使用 `--write`，未改 Ghidra/ReVa 数据库。

## 8. 编译 / 测试

```
cargo check -p country-core -p toolbar     # 0 error（仅既有 warning）
cargo test -p country-core --lib ui::resources   # 14 passed; 0 failed
cargo test -p country-core --lib components::trees # 1 passed
cargo test -p toolbar --lib                # 10 passed; 0 failed
```
