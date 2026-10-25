# RESTORE — unlock-debugline-signifier：`DebugLine::show` + UiSignifierStream 闭包槽/唤醒

> agent: unlock-debugline-signifier，2026-10-17。
> 范围：解依赖线 #3——`DebugLine::show`(0x140b04f70) 落库 + `UiSignifierStream` 承接
> `SignifierFactory` 40 B 闭包记录（vtable 0x142ae70d0 / 0x142efa150）与消费唤醒。
> 硬标准：fresh 反编译逐语句；只改 country-core；shared 只读（无 `--write` worker，未改库）。
> 证据：`work/unlock-debugline-signifier/{jobs,results,decomp}/`（10 组 Ghidra fresh job +
> 本机 objdump 字节级复核；全部 fresh，未复用旧 decomp 作判等依据）。

## 0. 结论速览

| 项 | verdict | 交付 |
|---|---|---|
| `DebugLine::show`（0x140b04f70） | **判等**（逐语句；见 §2） | `utils/debug_viz.rs::DebugLine::show` |
| **FIX-A1**（reviewer-A-unlock） | **已修 + 边界测试** | 闭包 payload q3 由 `points.capacity()` 改为 `colors.capacity()`（build default 分支的 `Vec<[f32;3]>` cap 槽）；新增 points.len≠colors.len 测试 |
| `DebugLine` 布局纠正 | **判等**（fresh 决定性） | +0x18 由 `label: Option<String>` 纠正为 `colors: Option<Vec<Vec3>>`；`new`/测试同步 |
| `DebugColor::as_array` + `Debug` | **判等**（跳表 0x142ae7334 六分支 + `debug_tuple_field1_finish`） | 手写 `Debug`（五个原名 + `Custom([r,g,b])`） |
| `SignifierFactory` 闭包记录 | **重建**（40 B `{String, Arc<dyn Fn..>}`；见 §3） | `closure` 槽 + `from_closure`/`wake`/`has_closure`/`name` |
| `UiSignifierStream` 承接/唤醒 | **重建** | `UiSignifierEntry::{Raw,Factory}`；`take_entries`；`signifier_create` 唤醒 |
| `signifier_create` 消费面 | **重建 + 1 GAP** | `Res<AssetMeshLibrary>/Res<AssetShaderLibrary>` + vtable 唤醒 + `SignifierId(hash)`；UiState 配置表克隆面登记 GAP |
| `SignifierEntityBuilder::new` 潜在 UB | **修复**（本批触发） | struct-update 于 `Drop` 类型导致默认 shader 串悬垂；改为显式字段赋值 |

**`cargo check -p country-core`：通过（0 error；14 warning 全为既有）。**
**`cargo test -p country-core --lib`：2093 通过 / 0 失败（含本批 6 个新测试）。**
`cargo check -p system-glade-border`：通过（新 API 兼容）。
`cargo check -p system-water`：2 个**既有**错误（`water_audio_grid.rs` 的 `water_level`/`position`
类型，属 `WaterAudioGrid` 语义投影差，非本批引入）。

计数：**判等 3**（show / 布局+颜色 Debug / 长度）· **重建 3**（factory 记录 / stream 承接 /
consumer 唤醒）· **修复 1**（builder::new UB）· **未实现/登记 2**（UiState 配置表克隆路径、
builder build 的 mesh/shader 解析面）。

## 1. fresh 证据（Ghidra shared 只读 + objdump）

| 证据 | 内容 |
|---|---|
| `jobs/fresh.json` → `decomp/0x140b04f70_show.c` / `_show.asm` | show 全反编译 + 235 指令反汇编；`body_end=0x140b053ec` |
| `decomp/0x140b04f70_show_fallback_and_colorfmt.asm` | objdump 字节级：colors=None 六分支常量块（0x140b0507d..0x140b05119） |
| `jobs/fresh2.json` → `decomp/0x140b055a0_closure$1.c` / `0x140b055a0_closure1.asm` | 闭包体（0x140b055a0，1222 B）全反编译；builder payload/insert 逐指令 |
| `decomp/0x140b00080_drop_in_place<closure_env$1>.c` | `ArcInner<closure_env$1>` drop：两个 Vec 的 cap 释放 |
| `jobs/fresh3.json` → `0x14089d9b0_new.c`、`0x141be1080_closure$1.c` | builder new + glade-border 对照闭包 |
| `jobs/fresh4.json` | 闭包 vtable 0x142ae70d0 = `{0x140b00080, 0x40, 8, 0x140affd20}`；glade-border vtable 0x142efa150 = `{0x14014b880, 0x20, 8, 0x141be0a30}` |
| `jobs/fresh5.json` → `0x140b055a0_closure1.asm` | 闭包入口：`RDI=RDX=env`，读 `+8/+0x10`、`+0x20/+0x28`、`+0x30/+0x34/+0x38/+0x39` |
| `jobs/fresh6.json` → `0x140b2cfa0_signifier_create.c` / `_create.asm`、`0x140b2cf90_write.c` | 消费面全反编译 + 527 指令；write 5 qword 拷贝/提交游标 |
| `jobs/fresh7/8/9/10.json` | 跳表 0x142ae7334 解码、颜色常量 0x14292e6b0 等、`0x140d872d0 = debug_tuple_field1_finish` |

## 2. `DebugLine::show`（0x140b04f70）判等

### 2.1 布局纠正（关键）

fresh 决定性：`+0x18` 字段是 **`Option<Vec<Vec3>>`**，不是 `Option<String>`：
- `new`（0x140b04e70）只写 `+0x18 = 0x8000000000000000`（其余 16 B 不写）——这是 `Option<Vec<_>>`
  在 **cap 槽**上的 niche（`isize::MIN`）；`Option<String>` 的 niche 在 ptr 且 None=0，不符。
- show 的 `Option::is_some` 内联为 `CMP 0,[+0x18]; SETNO`（0 减 cap 不溢出 ⇔ cap != isize::MIN）。
- `Option::unwrap_or_else` 的 else 分支按颜色跳表生成 `Vec<Vec3>`（见 2.3）。
- 闭包体把 `+0x18..+0x30` 当作第二个 `Vec<Vec3>`（元素步长 0xc）。

**落库**：`DebugLine.colors: Option<Vec<Vec3>>`（语义 = 逐点颜色；None = 用统一颜色填充）。
`DebugLine::new` 仍写 None；无 setter（原生 PDB 也只有 color/set_ui_pass/dashed/line_width）。

### 2.2 name 格式（判等）

show 起始 `format_inner(RCX=返回值, RDX=&args)`，args 四组 `(data, formatter)`：
1. `data=self`、formatter **0x1412f7540 = `<Vec<Vec3> as Debug>::fmt`**（debug_list 逐元素）；
2. `data=&RBP+0x75`（`SETNO` = `colors.is_some()`）、formatter 0x140156710（bool Display）；
3. `data=self+0x30`（DebugColor）、formatter **0x140b05c70 = `impl$7::fmt`**；
4. `data=self+0x40`（line_width）、formatter 0x140d8c890（f32 Display）。
片段表 0x142ae6f68 四项均为 `{ptr=1,len=0}` ⇒ **四段空字面**。故：

```rust
format!("{:?}{}{:?}{}", self.points, self.colors.is_some(), self.color, self.line_width)
```
已落库，逐参数一致。

### 2.3 `colors == None` 的颜色跳表（判等）

`JNO`（label cap != isize::MIN）→ 直接用 `Option` 内数据；否则按 `color.tag @+0x30` 走跳表
0x142ae731c（六项，全部解出目标 0x140b0507d..0x140b05119），把 **12 B 常量 vec3** 写入
`[RBP+0x18..0x24]`，再 `CALL 0x1412f7260`（`slice::repeat/to_vec`）生成长度 = `points.len()` 的
`Vec<Vec3>`：

| tag | 常量地址 | 值 |
|---|---|---|
| 0 White | 0x14292e6b0 + 1.0 | (1,1,1) |
| 1 Red | 0x142925904 + 0 | (1,0,0) |
| 2 Blue | 0 + 1.0 | (0,0,1) |
| 3 Green | 0x1429345b0 + 0 | (0,1,0) |
| 4 Grey | 0x142930d60 + 0.1 | (0.1,0.1,0.1) |
| 5 Custom | `self+0x34` 12 B | 自定义 RGB |

与 `DebugColor::rgb` 完全一致；落库为
`self.colors.clone().unwrap_or_else(|| vec![self.color.rgb(); self.points.len()])`。

### 2.4 折线总长（判等）

PDB 内联：`itertools::tuple_impl::impl$7::next` + `glam::Vec3::distance` + `f32::sqrt`
⇒ 源码级 `points.iter().tuple_windows().map(|(a,b)| a.distance(*b)).sum::<f32>()`。

机器码为两段展开的 SIMD 累加；其中 0x140b051a1 的 `JZ`（紧邻 `SQRTSS`）经复核**不是**零长检查：
SSE 标量算术不改 EFLAGS，该 JZ 复用 0x140b050ab `CMP RBX,0x2` 的 ZF ⇒ **len==2 直接跳过展开环**
（结果仍 = 唯一段长）。len<2 → 0；len>=3 → 12*(len-3) 的奇偶分支 peel 一段后按 2 段/迭代。
Rust `tuple_windows` 版本语义逐分支等价。

### 2.5 记录打包（判等）

- 闭包环境 = **`ArcInner<closure_env$1>`**：`alloc(0x50)`，`+0 {strong=1, weak=1}`（ArcInner 头），
  `+0x10` 起为 0x40 B 数据（`Arc::new` 在 PDB 内联；`drop_in_place<ArcInner<closure_env$1>>` 0x140b00080）：
  ```
  data+0x00 Vec1.cap   data+0x08 Vec1.ptr   data+0x10 Vec1.len   (points)
  data+0x18 Vec2.cap   data+0x20 Vec2.ptr   data+0x28 Vec2.len   (colors)
  data+0x30 f32 length data+0x34 f32 width  data+0x38 u8 ui_pass data+0x39 u8 dashed
  ```
- 记录 40 B = `{name_len@+0, name_ptr@+8, name_len@+0x10, inner@+0x18, marker@+0x20}`，
  `inner` = ArcInner 基址、`marker` = **0x142ae70d0**（vtable：drop 0x140b00080 / size 0x40 / align 8 /
  call_once 0x140affd20）。name = 2.2 的格式化串（`Box<[u8]>` 自有分配）。
- 经 `AppendOnlyVec::push<UiAppendSignifier>`（0x14146c880）入流。

闭包体（0x140b055a0）判等要点：
- payload 6 qword = `{len1, ptr1, len1, colors.cap, colors.ptr, colors.len}` →
  `SignifierEntityBuilder::new`。**reviewer-A-unlock FIX-A1 已核**：0x140b055a0 的
  `[RBP-0x38]` 来自 `[env+0x28]`（colors.len）且捕获 Vec `clone()`/`vec!` 的 cap==len；
  `build` 0x14089dd50 default 分支把 `{payload[3],payload[4],payload[5]}` 作为
  `Vec<[f32;3]>` 传给 `set_attribute`（`{cap,ptr,len}`），故 q3 = **colors 的 cap 槽**，
  不是 points 的 cap。已改 `mesh_payload(&points,&colors,colors.capacity())`，并补
  `mesh_payload_q3_uses_the_colors_capacity_slot` / `show_accepts_colors_with_a_different_length_than_points`
  两个边界测试（points.len=2 vs colors.len=5）。
- `set_ui_pass(env+0x38)`（写 `+0x8a = pass^1`）、`animation_duration(0.0)`（`+0x78=0`）、
  `custom_shader(dashed ? "dashed_line" : "vertex_color_shader")`（长度 11/19，字串 0x142ae7033/0x142ae7020）；
- `build`；dashed 时 `insert DashedLineLength(length * 4.0)`（常量 0x142934694=4.0），
  随后 `insert DrawTopology { tag=1, f32=line_width }`（≡ `DrawTopology::Alt(width)`）；
- 返回 `Vec<Entity>`（长度 1）。

**落库**：`DebugLine::show(&self, &mut UiSignifierStream)` 逐项实现（build 的 mesh/shader 解析面
仍为既有 GAP，见 §5）。

### 2.6 `DebugColor` Debug / as_array（判等）

`impl$7::fmt` 0x140b05c70 跳表 0x142ae7334 六目标：0x140b05c89/ccd/c9b/cb4/c92/ce6，
字符串 0x142ae71c1="White"、0x142ae71c6="Red"、0x142ae71c9="Blue"、0x142ae71cd="Green"、
0x142ae71d2="Grey"；Custom 分支 `addq $4,%rcx`（RGB 起点）+ `CALL 0x140d872d0`
**`debug_tuple_field1_finish`** + `fmt<array<f32,3>>`（0x1408fbe60）⇒ `Custom([r, g, b])`。
落库为手写 `Debug` + `as_array() -> [f32;3]`（去掉 `derive(Debug)`）。

## 3. `SignifierFactory` / `UiSignifierStream` 承接 + 唤醒

### 3.1 原生记录 = `{String, Arc<dyn Fn(..) -> Vec<Entity> + Send + Sync>}`

- 40 B 分解：`{cap/len@0, ptr@8, len@0x10}`（String）+ `{data@0x18, vtable@0x20}`（Arc 胖指针）。
- PDB 内联：show 的 `SignifierFactory::new`（source L112）、`Arc::new`、`Arc::drop_slow<T=dyn Fn(...)
  -> Vec<Entity> + Send + Sync>`（0x14064f9d0）。
- 消费端 `signifier_create` 0x140b2cfa0：
  - 先把每条记录 `String::clone` + Arc 引用计数 +1（`LOCK; *p += 1`）克隆进 `UiState+0x20`
    的 0x28 B/条 indexmap；
  - 去重键 = `calculate_hash<(WidgetId, Option<&str>)>`（WidgetId = `UiState+0x38`，name = 记录串）；
  - 未命中 → `(*(vtable+0x28))(sret, data_ptr, &AssetMeshLibrary, &AssetShaderLibrary, &mut Commands)`；
    data_ptr 还原 = `stored + 0x10 + ((align-1) & !0xf)`（align=8 → +0x10，即 ArcInner 数据偏移）；
  - 对返回 `Vec<Entity>` 的每个实体 `insert<SignifierId>`（载荷 hash）。
- glade-border 记录（对照）：env 0x30 B = `{1,1,label cap/ptr/len, perimeter}`，vtable 0x142efa150
  = `{drop 0x14014b880, size 0x20, align 8, call_once 0x141be0a30}`；同样 40 B `{name, inner, marker}`。

### 3.2 落库

- `SignifierFactory`：保留原 5-qword 视图（`name_len/name_ptr/name_len_dup/inner/marker`，旧 wall
  调用点 `new(payload, inner, marker)` 不变），新增 **crate 侧 `closure: Option<Arc<SignifierClosure>>`**
  槽；`from_closure(&str, Arc<SignifierClosure>)`、`has_closure()`、`name()`、`wake(&mesh,&shader,&mut commands)
  -> Vec<Entity>`。
- `SignifierClosure = dyn Fn(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands<'_,'_>) -> Vec<Entity>
  + Send + Sync`（与 `Arc::drop_slow` 单态一致）。
- `UiSignifierStream.slots: Vec<UiSignifierEntry>`：
  - `Raw(UiAppendSignifier)` ← `write`/`write_raw`/无闭包 `write_factory`（旧 5-qword 兼容）；
  - `Factory(SignifierFactory)` ← 带闭包的 `write_factory`；
  - `take_entries()` 全量排空（consumer 用）；`take_all()`/`drain()` 仅返回 Raw（旧消费面兼容，
    文档标注会丢弃 Factory 槽）。
- `signifier_create`：系统参数补 `Res<AssetMeshLibrary>`、`Res<AssetShaderLibrary>`（`AssetShaderLibrary`
  补 `Resource` marker impl）；Factory 分支按 `(UiState.widget, Some(name))` 哈希去重 → `wake` →
  逐实体 `insert(SignifierId(hash))`。

## 4. 解锁验证（只改 core）

1. **glade-border `display_glade_border`**：`cargo check -p system-glade-border` 通过。现状
   `let _line = DebugLine::new(...)...;`（`display_glade_border.rs:104`）→ 后续 restorer 仅需改为
   `line.show(&mut signifiers);`（system 已有 `mut signifiers: ResMut<UiSignifierStream>`）。
   core 侧测试 `show_packs_the_record_and_wakes_the_dashed_line_closure` 按同一调用形状验证：
   name 含 `Red`/`false`/`2.5`，记录为 Factory 槽，唤醒后 `DashedLineLength(20.0)`（|(3,4,0)|=5 ×4）
   与 `DrawTopology::Alt(2.5)` 落组件。
2. **water closure$0**：`system-water` 现有 2 个**既有**错误在 `water_audio_grid.rs`（与本 API 无关）。
   新 API 形状用测试 `external_factory_shape_matches_water_closure_zero` 验证：
   `SignifierFactory::from_closure("brush_circle_scribble", Arc::new(closure))` + `write_factory`
   → Factory 槽；water restorer 可据此替换现有 `UiAppendSignifier{tag,kind,pos,scale,name_len}` 近似写法。

## 5. 登记（未实现 / GAP，不冒充）

| 项 | 状态 | 说明 / 解锁条件 |
|---|---|---|
| `signifier_create` 的 `UiState` 配置表克隆 | **未实现（GAP）** | 原生先把 `{String,Arc}` 克隆进 `UiState+0x20` indexmap，再遍历该表唤醒；crate `UiState` 未落库该表。当前直接遍历本帧流记录，spawn 效果一致；持久化/跨帧重建语义缺。解锁：UiState 补 0x28 B 配置 indexmap 与 hash 键合同。 |
| `SignifierEntityBuilder::build` mesh/shader 解析面 | **未实现（既有 GAP）** | 原生内联 `AssetMeshLibrary::add`×4 / `try_get_handle_by_name` / `AssetShaderLibrary::get_raster_handle_by_name` / `mesh_from_pts`×3；crate build 仍 spawn+`SignifierAnimation`+占位 `SignifierId(0)`。 |
| 旧 Raw 5-qword 模型 | **兼容保留（I）** | 既有 wall/terrain 调用点语义仍是 crate 近似；本批未改。流内以枚举区分，避免与原生闭包记录混淆。 |
| `SignifierFactory::from_closure` 的 raw `inner/marker` | **I（表示差异）** | crate 用类型化 `Arc` 槽持有闭包；原生 5-qword 的 ArcInner 基址 + vtable 数值未复刻到裸字段（`from_closure` 不供字节级 raw 消费）。唤醒语义等价。 |
| 其余 `show`（Sphere/Rectangle3d/Rectangle2d） | **未实现（登记）** | 仍是既有状态；非本依赖线。 |

## 6. 冲突登记

| 文件 | 处理 |
|---|---|
| `utils/debug_viz.rs` | 本线独占（自上轮 2026-10-15 未改动）。 |
| `systems/ui/signifiers/{signifier_create.rs,signifier_factory.rs,mod.rs}` | 最近改动 2026-10-17 19:06（他组）；本批 22:2x 后未观察到并发写入。改动为追加式（enum/闭包槽/唤醒），旧 API 保留。 |
| `asset_libraries/shader_library.rs` | 仅追加 `impl Resource for AssetShaderLibrary {}`（PDB 系统参数要求）。 |
| `components/mod.rs` | 仅追加 `DashedLineLength(pub f32)`（原生组件；registry 已有注册名 0x140849b70）。 |
| 上层 `system-glade-border` / `system-water` | **未改**；water 的 2 个编译错误是既有 `WaterAudioGrid` 语义差。 |

## 7. 发现并修复的既有缺陷

`SignifierEntityBuilder::new`（0x14089d9b0 落库实现）原先写：

```rust
Self { mesh_kind: payload[0], mesh_payload: [...], ..Self::default() }
```

`SignifierEntityBuilder` 实现 `Drop`，结构体更新语法让 `Self::default()` 临时值释放其
`shader_name_replace` 分配的默认 shader 串，导致新构建器的 `shader_ptr` 悬垂；随后
`custom_shader` 释放该指针时在 macOS 上 SIGTRAP（本批首次真实调用 builder 时暴露）。
已改为显式字段赋值（先 `let mut builder = Self::default();` 再覆盖 `mesh_kind/mesh_payload`）。
此修复使 glade-border/water 的闭包唤醒路径可安全运行。

## 8. 验证

```text
cargo check -p country-core          -> 0 error（14 warning 全既有）
cargo test  -p country-core --lib    -> 2093 passed / 0 failed
  utils::debug_viz::tests::show_packs_the_record_and_wakes_the_dashed_line_closure  ok
  utils::debug_viz::tests::external_factory_shape_matches_water_closure_zero       ok
  utils::debug_viz::tests::mesh_payload_q3_uses_the_colors_capacity_slot            ok (FIX-A1)
  utils::debug_viz::tests::show_accepts_colors_with_a_different_length_than_points  ok (FIX-A1)
  systems::ui::signifiers::signifier_create::tests::{closure_factory...,legacy_take_all...} ok
cargo check -p system-glade-border   -> 0 error
cargo check -p system-water          -> 2 既有错误（water_audio_grid.rs:173/174，非本批）
```
