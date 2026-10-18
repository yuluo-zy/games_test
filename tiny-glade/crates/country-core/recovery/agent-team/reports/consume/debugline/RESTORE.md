# RESTORE — consume/debugline：`display_glade_border` DebugLine/show + water `closure$0`

> agent: restorer-consume-debugline，2026-10-17 晚。
> 范围：两个消费者接线——`crates/systems/glade-border`（`DebugLine::show` 分支 + perimeter
> `SignifierFactory` 唤醒）与 `crates/systems/water`（`draw_water::closure$0`，0x141ba9700）。
> 硬标准：fresh 反编译/反汇编逐语句；只改上述两个 crate 的 src；shared 数据库只读
> （无 `--write` worker，未改库）。
> 证据：`work/restorer-consume-debugline/{jobs,results,decomp}/`：
> `fresh1.json`（`get_functions 0x141be0ad0/0x141ba9700` + `disassemble_function`
> 两函数）、`fresh2.json`（disasm `0x141be1080 / 0x14089d8b0 / 0x14089dc00 /
> 0x14089dc60 / 0x14089dd50`）；导出的 `decomp/step1.txt`（water closure$0 85 条）、
> `decomp/display_glade_border_asm.txt`（246 条）、`decomp/closure1_141be1080.txt`（155 条）、
> `decomp/factory_new_14089d8b0.txt`（57 条）、`position/scale/build` 反汇编。
> 全部 fresh；旧 water/glade-border RESTORE 仅作对照，不作判等依据。

## 0. 结论速览

| 函数 / 项 | verdict | 交付 |
|---|---|---|
| `display_glade_border` 0x141be0ad0（事件门 + DebugLine 构建链 + show 调用点 + perimeter 写路径） | **判等**（本批范围逐句；其余分支沿用 wave3 判等） | `display_glade_border.rs`：`line.show(&mut signifiers)` |
| `display_glade_border::closure$1` 0x141be1080 | **判等**（155 指令逐句；**返回类型纠正**） | 返回 `Vec<Entity>`；factory 以 `from_closure` 唤醒 |
| `draw_water::closure$0` 0x141ba9700 | **判等**（85 指令逐句；83 条路径 + 2 条 alloc panic 桩） | `draw_water.rs::closure_0` + `from_closure` 写流 |
| 橡皮 signifier 记录/唤醒 | **判等**（0x141ba9384-0x141ba940b 逐句） | 由 raw `UiAppendSignifier` 改为原生 40 B factory 记录 |
| `SignifierFactory::new` 0x14089d8b0 | **判等**（复核，57 指令） | 既有 core 实现语义一致（本批未改 core） |
| `SignifierEntityBuilder::{position,scale}` 0x14089dc00/c60 | **判等**（复核，13/11 指令） | core 无同名方法；water 侧按 pub 字段逐语义写入（见 §4 I-2） |

计数：**判等 3**（display_glade_border 本批范围 / closure$1 / water closure$0）·
**重建 0**（无新增近似体；既有实现按机器码纠正）· **未实现 0 新增**。
既有 GAP 登记 4（§5）。

## 1. fresh 证据与关键校正

### 1.1 PDB `closure_env$0` 的读法（更正 water RESTORE §1.1）

`draw_water::closure$0` 的 PDB 名 `closure_env$0` 并不表示捕获
`(AssetMeshLibrary, AssetShaderLibrary, Commands)`——那三项是闭包 **实参**
（vtable call 面 `Fn(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands) -> Vec<Entity>`，
即 core `SignifierClosure`）。真实捕获（ArcInner data）只有 **`{position: Vec3, scale: f32}`
= 0x10 B**：

- 0x141ba97c8 `MOV EAX,[R14+8]` → 位置 z；0x141ba97d2 `MOV RAX,[R14]` → (x,y)；
  0x141ba97f0 `MOVSS XMM2,[R14+0xc]` → scale。R14 = RDX = env。
- 与创建侧一致：`draw_water` 橡皮分支 alloc 0x20 写
  `{1@0, 1@8, position@+0x10, scale@+0x1c}`（0x141ba93ac-0x141ba93c9），
  `inner` = ArcInner 基址，data = +0x10。
- vtable call 面：`R8=&AssetMeshLibrary`、`R9=&AssetShaderLibrary`、栈参 = `&mut Commands`
  （closure$0 0x141ba9827-0x141ba982d 以 R8=RBX / R9=RDI / [RSP+0x20]=[RBP+0x140] 调 build）。

### 1.2 `SignifierFactory::new`（0x14089d8b0，57 指令）

实参 `RCX=sret、RDX=name_ptr、R8=name_len、R9=inner、[RBP+0x70]=marker`；体：
`alloc(name_len,1)` + `memcpy`（名字**自有拷贝**），写
`out+0=name_len / +8=name_ptr(拷贝) / +0x10=name_len / +0x18=inner / +0x20=marker`。
即 core 既有 `SignifierFactory::new` 语义（`{len,ptr,len,inner,marker}`）判等。
唤醒端 data_ptr = `inner + 0x10`（ArcInner 头 `{strong=1@0, weak=1@8}`）。

### 1.3 `SignifierEntityBuilder::{position,scale}`（本批首次逐指令解出）

- `position` 0x14089dc00：`[RDX+0x84] = dword [R8+8]`（pos.z）、
  `[RDX+0x7c] = qword [R8]`（pos.x,pos.y），随后整块 0x90 B 拷贝到 sret。
- `scale` 0x14089dc60：`[RDX+0x50] = 1`、`[RDX+0x54] = XMM2`，整块拷贝到 sret。

## 2. `display_glade_border` 0x141be0ad0（判等，本批范围）

fresh 246 条反汇编逐段：

1. 模式门 `CMP dword[*in_RCX],0x13; JZ 尾部`（0x141be0b00）→ 现 Rust 一致。
2. `as_curve2`（0x1411ff2b0）→ `from_iter<Vec3, Map<Iter<Vec2>, closure_env_0>>`
   （0x141be6110）→ 折线 Vec 输出在 `RBP+0x80`（cap@0 / ptr@8 / len@0x10，由 12*len
   dealloc 与 memcpy src 实证）→ 现 Rust 一致。
3. EventReader 门（0x141be0b8e-0x141be0bde：两缓冲 `saturating_sub` 计数
   `(u8>=u8 && !=0) || (...)`，命中时把游标推进到当前计数）→ `.read().last().is_some()`。
4. **DebugLine 构建链**（0x141be0be4-0x141be0ce9）：
   - 复制折线（`alloc 12*len` + memcpy）→ `DebugLine::new`（0x140b04e70）；
   - `color`（0x140b04eb0）：`[RBP-0x50] = 1` → **`DebugColor::Red`**；
   - `line_width`（0x140b04f40）：`XMM2 = [0x142934694] = 4.0`；
   - `set_ui_pass`（0x140b04ee0）：`R8D = 0` → false；
   - **`show`（0x140b04f70）：`RCX=&DebugLine`、`RDX=[RSI]`（=流指针）**。
   现 Rust 改为 `line.show(&mut signifiers);`，参数/顺序逐条一致。
5. demo 门 `CMP byte[*0x14362c098],1`（0x141be0cee-0x141be0cf8）→ overlay 写。
6. perimeter 门 `CMP byte[*config+0x121],0; JZ`（0x141be0d4f-0x141be0d56）。
7. perimeter 记录（0x141be0d82-0x141be0e97）：
   - `ShapeParameters::perimeter` 0x1411ff640 → XMM6；
   - `format!("glade_border{}", game_time.frame_count)`（格式表 @0x142efa140，
     参数 = `GameTime+0x2c`，格式化器 0x140d8d560）；
   - `alloc 0x30`：`{1@0, 1@8, points.cap@+0x10, points.ptr@+0x18, points.len@+0x20,
     perimeter@+0x28}`（三 qword 来自 `[RBP+0x80/0x88/0x90]` = cap/ptr/len）；
   - `SignifierFactory::new(out, name_ptr, name_len, inner=alloc, marker=0x142efa150)`；
   - `UiSignifierStream::write`（0x140b2cf90）；
   - 随后 `__rust_dealloc` 临时名字串（0x141be0e9c-0x141be0eb2），**折线 Vec 保留**
     （所有权在 ArcInner，drop vtable 0x14014b880 释放）。
   crate 侧改为 `SignifierFactory::from_closure(&label, Arc::new(closure))` +
   `write_factory`：名字同样自有拷贝、闭包持有折线 Vec 与 perimeter；不再 `mem::forget`
   （旧实现泄漏折线 Vec）。`signifier_create` 的 Factory 分支按
   `(WidgetId, Some(name))` 哈希去重后唤醒闭包。

## 3. `closure$1` 0x141be1080（判等）

env（RDX）＝ ArcInner data：`cap@0（只被 drop 用）、ptr@+8、len@+0x10、length@+0x18`。
155 条逐句：

1. `R15=[RDI+0x10]`（len）、`R13=[RDI+8]`（ptr）；`alloc 12*len` + memcpy；
   长度 0 时用 `R14=4`（dangling）——Rust `Vec` 同。
2. 载荷 `[RBP+0x40] = 0x8000000000000002`、`+0x48=len`、`+0x50=ptr`、`+0x58=len`、
   `+0x60/+0x68` 栈未初始化 → `new`（0x14089d9b0）；
3. `set_ui_pass(0)`（0x14089dad0）→ `animation_duration(0.0)`（0x14089dc90）→
   `custom_shader("dashed_line", len 0xb)`（0x14089db00）→ `build`（0x14089dd50，
   实参 `&AssetMeshLibrary/&AssetShaderLibrary/&mut Commands` 来自入口 R8/R9/栈）；
4. 2× `Commands::insert`（按原顺序）：`DrawTopology` 值 `0x4000000000000001`
   = `Alt(2.0)`；`DashedLineLength` = `[RDI+0x18] × 0x1429cd030(0.7)`；
5. **返回 1 元素 `Vec<Entity>`**：sret 写 `{cap=1@0, ptr=heap(Entity)@8, len=1@0x10}`，
   entity 取自 build 的返回。旧实现返回 `()`，已纠正为 `Vec<Entity>`（缺失时
   `signifier_create` 无法给实体插 `SignifierId`）。

## 4. water `closure$0` 0x141ba9700（判等）

85 条反汇编（`decomp/step1.txt`）逐句：

1. `alloc(8,8)`（返回 Vec 的实体槽；失败 `handle_alloc_error`）；
2. `alloc(0x15,1)` + `strncpy("brush_circle_scribble", 0x15)`（21 B，源 0x142eebc98）；
   失败 `raw_vec::handle_error`；
3. 载荷 `{0x8000000000000000@0x40, 0x15@0x48, name_ptr@0x50, 0x15@0x58}`（尾 2 qword
   栈未初始化）→ `new`（0x14089d9b0，out=`RBP-0x50`）；
4. `animation_duration(0.0)`（XORPS XMM2）；
5. **`position(env.position)`**：`[R14]` qword + `[R14+8]` dword；
6. **`scale(env.scale)`**：`XMM2 = [R14+0xc]`；
7. `set_ui_pass(0)`（R8D=0）；
8. `build`（R8=R14=入口 R8=&AssetMeshLibrary、R9=RBX=入口 R9=&AssetShaderLibrary、
   `[RSP+0x20]=[RBP+0x140]`=&mut Commands）；
9. sret 写 `{1, ptr, 1}`，返回 `RAX=RSI`。

实现 `draw_water::closure_0(position, scale, commands)`；橡皮分支以
`SignifierFactory::from_closure("eraser_water_", Arc::new(move |_,_,commands| closure_0(point, scale, commands)))`
入流（对应创建侧 0x141ba9384-0x141ba940b：inner = ArcInner `{1,1,position,scale}`、
marker vtable、名字格式串在 write 后释放）。

## 5. 登记（未实现 / GAP，不冒充）

| # | 项 | 状态 | 说明 / 解锁条件 |
|---|---|---|---|
| 1 | `SignifierEntityBuilder::build` mesh/shader 解析面 | **既有 GAP** | 原生 `AssetMeshLibrary::add/try_get_handle_by_name`、`AssetShaderLibrary::get_raster_handle_by_name`、`mesh_from_pts` 未落库；本批闭包保留两库实参形状，build 仍为 spawn+`SignifierAnimation`+占位 `SignifierId(0)`。 |
| 2 | builder `position/scale` 无同名方法 | **I（表示差异，本批绕行）** | core `SignifierEntityBuilder` 未暴露 `position/scale`；water 侧按 pub 字段逐语义写入 `u7c/u84`（position）与 `flag50=1/f54`（scale），数据流与 0x14089dc00/c60 逐字段一致。若 core 后续补方法，可换回。 |
| 3 | payload 尾 2 qword | **I** | 原生 `[RBP+0x60]/[RBP+0x68]`（water）/同名栈槽（glade-border）为未初始化栈；crate 取确定 0，build 的 mesh 解析未落库前不可观察。 |
| 3b | water 网格名指针 | **I（表示差异）** | 原生每次 `alloc(0x15,1)` 拷贝 `"brush_circle_scribble"` 入载荷；crate 用 `&'static str` 指针（build 的 mesh 解析面 GAP、且原生 closure$0 内未见该串 dealloc）。build 落库后如要求自有分配，改为 `String` 并随载荷移交。 |
| 4 | `DashedLineLength` 类型统一 | **已按反编译改（reviewer-consumers 复核后）** | glade-border `closure_1` 已改插真实组件 `country_core::components::DashedLineLength`（与 `DebugLine::show` 同型，TypeId 0x4782fcee8a8c578b）。渲染读取面 `draw_callbacks_a` 仍用同 TypeId 的镜像 `DashedLineLengthView`——跨模块登记（非本 crate 范围）：两类型不互通，渲染接线统一前 glade-border 的虚线长度不会被当前渲染读取面看到。 |
| 5 | `signifier_create` 的 `UiState` 配置表克隆 | **既有 GAP**（unlock 报告 §5） | 原生先克隆 `{String,Arc}` 进 `UiState+0x20` indexmap 再唤醒；crate 直接遍历本帧流。可观察 spawn 效果一致。 |
| 6 | `system-water` 2 个既有编译错误 | **既有阻塞（非本批）** | `water_audio_grid.rs:173/174`（`WaterGridCell.water_level`/`position` 语义投影与 core 新布局不符，属音频域裁决）。本批 `draw_water.rs` 无错误；该文件未改。 |

## 6. 验证

```text
cargo check -p system-glade-border      -> 通过（0 error；仅 core 既有 warning）
cargo check -p system-glade-border -p system-water
  -> system-glade-border：通过
  -> system-water：2 个既有错误（water_audio_grid.rs:173/174，基线同样存在）
     draw_water.rs 无 error（rustc 全 crate 类型检查后仅报上述两处）
cargo test -p system-glade-border --lib -> 1 passed / 0 failed
  display_glade_border::tests::closure_1_wakes_and_inserts_dashed_border_components
    （返回 1 实体；DrawTopology::Alt(2.0)；components::DashedLineLength(4.0×0.7=2.8)）
```

未跑 `cargo test -p system-water`：被上述既有编译错误阻断（与 baseline 相同）。

## 7. 冲突登记

| 文件 | 处理 |
|---|---|
| `crates/systems/glade-border/src/display_glade_border.rs` | 本批独占修改（加 `#[cfg(test)]` 1 测试）。 |
| `crates/systems/water/src/draw_water.rs` | 本批独占修改。 |
| `crates/country-core/src/systems/ui/signifiers/**`、`utils/debug_viz.rs` | **未改**（只读复核；按红线不改他模块）。 |
| `crates/systems/water/src/water_audio_grid.rs` | **未改**（既有错误，音频域待裁决）。 |
