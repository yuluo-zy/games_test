# REVIEW — gap 轮落库独立复核（core-events + utils-toolbar）

> reviewer-gap-round · 2026-10-17。
> 范围：`reports/gap/core-events/RESTORE.md`、`reports/gap/utils-toolbar/RESTORE.md` 所列全部交付。
> 硬标准：fresh 反编译 → 独立复述 → 与 Rust 逐语句/逐字段判等；只读（未改 src）。
> 方法：ghidra `--worker shared` fresh（本报告证据 `work/reviewer-gap-round/`，identity：
> `tiny-glade.exe`、function_count 131517，B patched EXE，与 restorer 身份一致）+
> `disassemble_function` 机器码核对 + glam 0.29.3 源码逐分支映射 + 现有证据交叉复核。
> 分歧检查表 8 条逐条扫（见 §4）。

## 0. 三态结论

| # | 条目 | verdict | 要点 |
|---|---|---|---|
| 1 | `RequestAppModeChange` 载荷面（core-events §1.1） | **需修** | 原生载荷是 **16B `AppMode`**，不是 `(String, SettingsMenuState)`；本批改动把正确旧型 `{mode: AppMode}` 改坏 |
| 2 | `RequestToolbarCategoryChange` 载荷面（§1.2） | **判等** | `EventWriter<RequestToolbarCategoryChange>` 为名义类型（非 alias）；1B 载荷 `WindowFocusEvent`；`EventInstance` stride 0x10 |
| 3 | `WindowFocusEvent(pub u8)`（§1.3） | **判等** | 三处写面均 1B 载荷 @+8、stride 0x10；u8 成立 |
| 4 | `HistoryTransactions::ledger_mut()`（§2） | **判等** | 访问器无原生对照面；其承接的 `insert_starting_edit` 0x140a844f0 机器码逐语句复核通过 |
| 5 | `UiSignifierStream::write_raw/write_factory`（§3） | **判等** | 0x140b2cf90 = `JMP 0x14146c880` 确认；记录 stride 0x28、流 0x170B fresh 确认；5 qword 位型保持 |
| 6 | `Set*/Decorator*/Ladder/Magic/Brightness`（§1.4） | **判等（沿用 coregap-2）** | 本批未改、无新证据；复核定义未见新缺口（本轮未重新反编译，诚实标注） |
| 7 | `utils::rotation_from_normal`（utils §1） | **需修** | 4 分支半 sqrt 映射逐语句判等；归一化有效掩码 `is_finite()` 与原生「有限且非零」位掩码不等（溢出输入行为相反） |
| 8 | `Rectangle2d::orientation_mat3`（§2） | **判等** | 9 store 逐字段；列 1 = Y；`-axis.y` ≡ XOR 0x80000000 |
| 9 | toolbar stride-3 记录 / keep / ui_animation / switch_category（§3/§5） | **判等** | 3 字节偏移、keep 四步算法、动画公式、选中写面逐语句判等（tick 写面 U 已登记） |
| 10 | `tod_v2_button` per-widget `+2` 写面（§5） | **需修** | 值 = visible 判等；但 Rust 在 click/toggle **之前**写 `+2`，原版在 toggle **之后**写 |
| 11 | `switch_app_mode_button` | **未实现（诚实）** | 空壳 + 登记；其引用的载荷类型须随 #1 回退 |
| 12 | `UiNav`/`HistoryButton` 类型（§4） | **未实现（布局 U）** | typed gap 0x10，诚实登记 |

计数：**判等 7 / 需修 3 / 未实现 2**。

## 1. 需修项（FIX 清单）

### FIX-1（高，core-events）：`RequestAppModeChange` 载荷回退为 `AppMode`（16B）

**原版证据（fresh）**

- `switch_app_mode_button` 0x142341fe0 热路径（`work/reviewer-gap-round/decomp/0x142341fe0-switch_app_mode_button.c` 行 123-139；
  机器码 `work/reviewer-gap-round/decomp/disasm-0.txt` 0x1423420b8..0x1423420d8）：
  `*slot = {16B payload @+0, event_count u64 @+0x10}`，stride **0x18**。
  16B payload 来自按钮组件数据（`MOVUPS XMM0,[RAX+RCX]`，组件 16B），随后 `3rd qword = Events+0x40`（EventId）。
  ⇒ `EventInstance<E>` = 8 + 16 = 24B，**E = 16B**。若 E = `(String, SettingsMenuState)`
  （String 24B + enum = 32B），`EventInstance` 会是 0x28，**与原 stride 0x18 不符**。
- **实测 Rust 尺寸**（`rustc` 链接当前 `libcountry_core` 直接打印，2026-10-17）：
  `size_of::<AppMode>() == 16`、`size_of::<WindowFocusEvent>() == 1`、
  `size_of::<RequestToolbarCategoryChange>() == 1`、`size_of::<SettingsMenuState>() == 1`、
  而现 `RequestAppModeChange`（String+enum）**== 32**。故现型的 `EventInstance` = 40 (0x28)，
  与原 0x18 差 16B；`RequestAppModeChange(pub AppMode)` 恰为 16 → EventInstance 24 (0x18) 对齐。
- 消费面 `check_if_can_switch<mode_manager::closure_env_3>` 0x140afa2c0（fresh
  `decomp/0x140afa2c0-...c`）：`puVar5 = in_R8[1]`（末条事件载荷指针）→
  `uVar8=*puVar5; uVar10=puVar5[1]` → **16B 整块写回 `*plVar4`（AppMode resource）**；
  随后 `local_28 = (int)uVar8; if (local_28-6 < 0xf) lVar9=(tag)-5;
  if (lVar9==0xf || lVar9==0) {toolbar 特路} else {Toolbar::switch_mode()}`。
  该 tag 分派与 Rust `select_app_mode` 的 `Editing(_) | CloneClutter` 特路逐分支对应
  （AppMode 原生 tag：Editing = 0..5，CloneClutter = 0x14，见 `types.rs` 290-312 行）。
  **载荷被直接当 AppMode 写资源并按变体 tag 分派 → 载荷值就是 AppMode，不是字符串。**
- 旁证：`ui_focus_decoclutter` 0x1409a92c0 以 **`EventWriter::write<RequestAppModeChange>()` 方法调用**
  写事件（`work/restorer-terrain-clutter/jobs/decomp/1409a92c0-ui_focus_decoclutter.txt` 行 998），
  不是元组 Vec push。

**restorer 误判根因（证据真实性问题）**

- RESTORE 依据的 `grow_one<tuple<ref<str>, enum2<SettingsMenuState>>>` 是 **ICF（相同代码折叠）后的
  符号**：`RawVec::grow_one<T>` 的机器码只取决于 `size_of::<T>()/align`；
  `EventInstance<AppMode>`（24B, align 8）与 `EventInstance<(&str,SettingsMenuState)>` 的
  grow_one 都是 24B/8 对齐，链器可合并，Ghidra 取到其中一个名字。
- 决定性反问：`run_unsafe<FunctionSystem<... EventWriter<RequestAppModeChange> ...>>`
  的 mangled 名是**名义类型** `RequestAppModeChange`。Rust 的 type alias 在符号中会被擦除为底层类型，
  因此它**不是** `type RequestAppModeChange = (...)`；它是一个 struct。其字段类型不能由
  `grow_one` 符号推出，而由 closure_env_3 的 16B 直写资源定案为 `AppMode`。

**FIX**（`crates/country-core/src/systems/mode_manager.rs`）

- `pub struct RequestAppModeChange(pub AppMode);`（保留 `{mode: AppMode}` 旧面亦可）；
  删除 `RequestAppModeChange::named(...)` 与 `app_mode_from_request_name(...)`
  （`"All"/"Wall"/...` 名字映射在二进制中不存在，属**事件/载荷自造**）。
- 消费面：`requested_modes.read().last()` 后直接 `select_app_mode(request.0, &mut toolbar, &mut mode)`；
  删除 `request.1` 的 `SettingsMenuState` 读面注释（该字段不存在）。
- 下游连带：`systems/ui_clutter/ui_focus_decoclutter.rs:105-108` 现以
  `RequestAppModeChange::named("DecoClutter", SettingsMenuState::Ui)` 伪造载荷；应改为原生等价
  AppMode（该函数 fresh 0x1409a92c0 有真实 `EventWriter::write<RequestAppModeChange>` 调用，
  载荷来源需按该函数局部再取证；在取证前不得用字符串名冒充）。
- 测试同步：`mode_manager::recovery_tests::*`、`ui_clutter::tests` 中的 `named(...)` 调用。

### FIX-2（中，utils）：`rotation_from_normal` 归一化有效掩码

- fresh 0x140c90a40（`work/reviewer-gap-round/decomp/0x140c90a40-rotation_from_normal.c`）：
  `bVar4 = (absbits(fVar12) - 0x800000) < 0x7f000000`、`bVar5 = (bits(fVar12) - 1) < 0x7fffff`、
  `bVar1 = bVar5 || bVar4 && -1 < (int)fVar12` ⇒ 有效 = **bit ∈ [0x1, 0x7f7fffff] = 有限且非零**
  （含正规与次正规，排除 +0）。第二处缩放的 `fVar15` 用**同一**位掩码。
- Rust（`crates/utils/src/geometry/mod.rs:57,70`）：`inv.is_finite()` / `c_inv.is_finite()`。
  `+0.0.is_finite() == true`，**两者在 `inv == 0` 时相反**。`inv == 0` 可由「分量有限但平方溢出为
  inf」（如 n=(3e38,0,3e38)）触发：原生走退化 `(0,1,0)`，Rust 走有效分支得 `(0,-0,0)` 并进一步
  产生不同的 c/四元数。RESTORE 括号里「len²=inf→inv=0 时同真」的等价声明**不成立**。
- FIX：两处改为 `inv.is_finite() && inv != 0.0`（或 `inv.is_finite() && inv > 0.0`；`inv = 1/sqrt(≥0)`
  非负，二者等价）。其余 4 分支半 sqrt 与 glam `from_rotation_axes`（0.29.3 源码 198-255 行）
  逐分支、逐分量已复核一致（A/B/C/D 与 m22/dif10/sum10 的对应见 §3）。
- 附注（I，非阻断）：c 归一化的平方和顺序，原生 `z²+y²+x²`，glam `Vec3::length` 为 `x²+y²+z²`；
  单精度非结合，极端输入可能位级不同。建议登记，不要求修。

### FIX-3（低，toolbar）：`tod_v2_button` `+2` 写序

- fresh 0x142342f30：click 匹配块内先 `visible ^= 1`（`*(byte*)(lVar10+10) = bVar2 ^ 1`）并插动画，
  **随后**进入 per-widget 循环写 `*(u8*)(col+2+id*3) = *(u8*)(lVar10+10)`（读当前 visible）。
- Rust（`crates/toolbar/src/toolbar_button.rs:199-219`）：先 `visible=false`（清路径）→ per-widget
  `byte.selected = visible` → 再逐 click `visible ^= true`。同帧 `+2` 拿到的是 toggle **前**的值。
- FIX：把 per-widget 写入循环移到 click 处理之后（与原生程序序一致）；`!=0xd` 判别与 tick 写面
  维持现有 U 登记。

## 2. 判等项证据摘要

### 2.1 `RequestToolbarCategoryChange` = 1B 载荷名义事件（判等）
- caller mangled：`EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>`
  ——type alias 会被擦除，故它是名义 struct（newtype 成立）。
- fresh 0x1423427f0（`decomp/0x1423427f0-switch_toolbar_category_button.c` 行 106-140）：
  `grow_one<EventInstance<WindowFocusEvent>>`；`{event_id u64@+0, payload u8@+8}`，stride 0x10。
  payload 取自按钮 stride-1 列（widget id 索引）；Rust 以 `button.category as u8` 代理（I 已登记）。
- `RequestToolbarCategoryChange(pub WindowFocusEvent)` 的 `EventInstance` 同样 0x10，载荷平面判等。
  （newtype 与 alias 在 1B 上不可区分；caller 名义名已排除 alias，newtype 是正确选择。）

### 2.2 `WindowFocusEvent(pub u8)`（判等）
- 写面：switch_toolbar_category_button（stride 0x10，`u8@+8`）；render_taffy_ui_to_widgets
  0x142338990 两处 `EventInstance<WindowFocusEvent>`（`work/gap-core-events/decomp/...` 行 569/616，
  `{id@0, byte@+8}`）；finish_timelapse 0x140a8a460（`{id@0, byte@+8}`）。
- **RESTORE 措辞纠正（非实现分歧）**：finish_timelapse 写入的载荷是 `bVar1 = *pbVar2`
  （一个状态字节），不是字面 `u8=0`；类型 u8 结论不受影响。

### 2.3 `ledger_mut()` + `insert_starting_edit`（判等）
- `ledger_mut` 为 Rust 侧访问器（无原生函数面）；需求由 `convert_preplaced_trees` 的
  `&mut ObservedHistoryLedger` 调用面成立。
- 0x140a844f0 机器码（`decomp/disasm-3.txt`）逐条对照
  `history_snapshots.rs::insert_starting_edit`：
  `CMP cursor,-2; JLE panic`（= `assert!(cursor >= -1)`）→ `INC cursor`；
  链首插入（`memmove(base+0x28, base, len*0x28)` + 写新条目）；
  `fastrand` 取 nonce 写 `+0x60`；`+0x58 = 0`（`observed_count_58`）。判等。
  （nonce 由参数传入是实现接口选择，原版内部取 `fastrand`；已有文档注明。）

### 2.4 `write_raw` / `write_factory`（判等，接口面）
- 0x140b2cf90 fresh disasm = `JMP 0x14146c880`（append-only push），与 RESTORE 一致。
- 0x140b2cfa0 fresh：流整块 `0x170B`（`memcpy 0x170` + `memset`），记录 stride **0x28**。
- Rust：`UiAppendSignifier` repr(C) 40B（tag@0/kind@8/Vec3@0x10/scale@0x1c/name@0x20）；
  `from_raw_qwords` 对 q2/q3 做 bit 级拆分-重组（`f32::from_bits`→`to_bits`，无算术），
  位型保持；`write_factory` 按 {name_len,name_ptr,name_len_dup,inner,marker} 5 qword 入流。
  接口面判等。
- 观察（沿用 RESTORE 已登记、本批不改）：`write` 的提交/读取游标语义与 `take_all` 的整批消费
  需 signifier 域复核；注意 RESTORE §3 末段「drain 读 [read_cursor, write_counter)」与当前
  `drain = take_all` 代码不一致，属**文档自相矛盾**，应更正描述（不影响本批新增方法）。

### 2.5 `orientation_mat3`（判等）
- fresh 0x140c99570：`out[0]=axis.x, [1]=0, [2]=axis.y`；`(out+3) u64 = 0x3f80000000000000`
  ⇒ `[3]=0.0, [4]=1.0`（列 1 = Y）；`[6]=axis.y^0x80000000`；`[7]=0; [8]=axis.x`。
  Rust `Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))` 逐字段一致；
  `-axis.y` 与 XOR 符号位在含 NaN 时也位级等价。

### 2.6 toolbar 3 字节记录族（判等）
- `WidgetByteState`：`+0` appear 符号、`+1` keep、`+2` selected，三处 fresh 读/写偏移逐条对上
  （keep_focus 0x1423414b0 `col+1+id*3`；ui_animation 0x14233b3f0 `*pcVar20`/`pcVar20[1]`；
  switch_category/tod `+2`）。size=3、offset_of 断言在位。
- `toolbar_button_keep_focus`：四步（keep≠0→0.3+any_keep / keep=0→max(cd-dt,0) /
  reset 事件非空→全 0 + 读游标复位 / any_keep→keep=0 项冷却 0 / cd>0→keep=1）逐语句判等；
  tick（UiInteractionState）写面 U 登记。
- `ui_animation_state_update`：leave `state==0→2`、否则 `select-=dt/0.2,state=1`；enter
  `state!=1→select+=dt/0.2,state=0`、否则 `state=2`；clamp01；appear `+=4dt·sign`
  （sign 由 `+0` 字节==0 取负）逐语句判等。
- `switch_toolbar_category_button`：事件 push（1B payload）+ `+2 = (toolbar.current == category)`
  + 动画门 `ModeJustChanged 存在 && +2!=1 && category 相等` 判等；常量 `0.1/0.2`
  （0x3dcccccd/0x3e4ccccd）逐位对上。点击实体解析以 `query.get(click.entity)` 代理原生
  UiNavState 层级查找（I 已登记）。

## 3. `rotation_from_normal` 4 分支映射（独立复算）

`glam::Quat::from_mat3(Mat3::from_cols(x=(w.y,w.z,w.x), y=c, z=n))` 展开为
`from_rotation_axes`（glam 0.29.3 scalar 198-255 行），与 fresh 反编译逐分支对应：

| glam 分支 | 条件 | 反编译 | 对应 |
|---|---|---|---|
| m22≤0 ∧ dif10≤0 | `z≤0 ∧ c.y-w.y≤0` | `t'=(1-nz)-(cy-wy)`，`q=(f·t', f(cz+wx), f(wx+nx), f(cz-ny))` | C 分支逐分量一致 |
| m22≤0 ∧ dif10>0 | `q=(f(wz+cx), f·t'', f(cz+ny), f(nx-wx))` | D 分支一致 |
| m22>0 ∧ sum10≤0 | `s'=(nz+1)-(wy+cy)`，`q=(f(wx+nx), f(cx+wz), f·s', f(wz-cx))` | A 分支一致 |
| m22>0 ∧ sum10>0 | `s''=s+nz+1`，`q=(f(cz-ny), f(nx-wx), f(wz-cx), f·s'')` | B 分支一致 |

w/c 前置向量亦逐项复算一致（`w=normalize(nx,-nz,0)`、`c=(ny·wx-nz·wz, nz·wy-nx·wx, nx·wz-ny·wy)`；
原生变量 `fVar8=w.z, fVar13=w.y, fVar9=w.x`）。仅 FIX-2 的掩码不等。

## 4. 分歧检查表 8 条扫描

| # | 检查项 | 结果 |
|---|---|---|
| 1 | panic-vs-return 反 | 未命中（rotation 有限守卫 panic 一致；insert_starting_edit assert 一致；tod/switch 无 panic 面） |
| 2 | 门控极性反 | 未命中（switch_category 门、keep 四步、tod clear/toggle 门方向一致；**tod 写序非极性**，见 FIX-3） |
| 3 | 载荷平面近似 | **命中 1 处**：`RequestAppModeChange` 被近似为 `(String,SettingsMenuState)`（FIX-1）；WindowFocusEvent/u8、stride-3 记录、color 族未见 |
| 4 | 事件/命令自造 | **命中 1 处**：`app_mode_from_request_name` 的字符串→AppMode 名字映射（及 `named("DecoClutter",…)`）为自造消费语义（FIX-1 连带） |
| 5 | callee 转发错 | 未命中（grow_one 名称问题属 ICF 归属，已按热路径/closure 语义定案） |
| 6 | 返回结构不符 | 不适用/未命中 |
| 7 | 排序取最近 vs 首个 | 未命中（mode_manager `.last()` 沿用既有判等；本批未改） |
| 8 | 偏移表/字段错位 | 未命中（orientation 9 store、3 字节记录、0x28 记录、0x10B/0x18B color 族逐字段对上） |

## 5. 证据真实性与高估检查

1. **`RequestAppModeChange` 结论高估**：RESTORE 把 ICF 折叠后的 `grow_one` 名字当成载荷类型，
   并以 stride 0x18 宣称「24B = (str, SettingsMenuState) 判等」——两条证据自相矛盾
   （该 payload 若成立 EventInstance 应为 0x20/0x28）。closure_env_3 的 16B 直写资源 +
   tag 分派是决定性反证。属**误判**，不是诚实登记。
2. **finish_timelapse「失焦 u8=0」**：原生写的是 `*pbVar2` 状态字节，非字面 0。类型结论不变，
   描述应改为「状态字节（finish 路径取值 U）」。
3. **`rotation_from_normal` 掩码等价声明错误**：RESTORE 称 `len²=inf→inv=0 时同真`；
   实为原生判无效、Rust 判有效（FIX-2）。
4. **「B dense 容器 vs per-entity」定性过强**：fresh 各函数以 Query 列基址 + row/wid*3 读写
   3 字节记录（即 per-entity 组件列），当前 Rust per-entity 组件实际更接近原生；
   `PerWidgetButtonState` 为额外备用容器。不是错误实现，但 I 级分歧描述应下调/更正。
5. **可复现**：RESTORE 的编译/测试声明未发现夸大；证据目录中的 `fresh-*.c.txt` 与本轮 fresh
   反编译逐字节一致（同 DB、同函数）。

## 6. 本轮 fresh 证据索引（`work/reviewer-gap-round/`）

- `jobs/fresh-review.json` + `results/fresh-review.result.json`：metadata + 9 函数 fresh 反编译
  （switch_app_mode_button / switch_toolbar_category_button / toolbar_button_keep_focus /
  ui_animation_state_update / tod_v2_button / rotation_from_normal / orientation_mat3 /
  signifier_create / finish_timelapse）。
- `decomp/`：上述 `.c` 落盘；`disasm-0..3.txt`：switch_app_mode_button、
  switch_toolbar_category_button、0x140b2cf90（`JMP 0x14146c880`）、0x140a844f0 机器码。
- `jobs/closures.json` + `decomp/0x140afa2c0-*`：RequestAppModeChange 消费 closure fresh。
- `jobs/types.json`/`findfns.json`/`xref.json`：PDB 无事件结构类型（仅符号）；事件名符号表核对。
- 交叉复核：`work/restorer-terrain-clutter/jobs/decomp/1409a92c0-ui_focus_decoclutter.txt`
  （`EventWriter::write<RequestAppModeChange>` 方法调用面）。
- 类型尺寸实测：`rustc` 链接 `target/debug/deps/libcountry_core-*.rlib` 打印
  `AppMode=16 / RequestAppModeChange=32 / WindowFocusEvent=1 / RequestToolbarCategoryChange=1 /
  SettingsMenuState=1`。

### 6.1 复跑状态（环境限制，非代码结论）

- `cargo test -p utils --lib rotation_from_normal`：**2/2 通过**（复跑）。
- `cargo test -p toolbar --lib`、`cargo test -p country-core --lib ledger_mut`：复跑时构建盘
  **ENOSPC（No space left on device）**，编译中断，未取得测试结果；失败原因是磁盘空间，
  不是断言失败。RESTORE 的「toolbar 2/2、core 1846 通过」本轮**未独立复跑确认**（证据沿用其声明）。
