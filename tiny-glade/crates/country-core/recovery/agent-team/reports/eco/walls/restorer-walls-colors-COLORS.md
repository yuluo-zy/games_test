# COLORS — walls 颜色链：`get_building_and_roof_colors` 0x140a94880 + create_circle/rectangle elevation/style 接线
（restorer-walls-colors，2026-10-17）

> 方法：只以本轮 fresh 反编译/反汇编为准（`reverse-operator.py ghidra --worker shared`）。
> fresh 证据落盘 `planning/impl-walls-colors/`：
> `ghidra-batch1-result.json`（7 函数 decomp+callees）、`ghidra-params-result.json`、
> `ghidra-disasm-colors-result.json`（0x140a94880 全 314 指令）、
> `ghidra-disasm-callers-result.json`（create_rectangle 849 指令 / create_circle 816 指令）、
> `ghidra-strings-result.json`（panic 串）＋ `decomp/` 逐函数 txt。
> 未采信此前 restorer/reviewer 的结构描述；**发现并纠正派单 §D 的输出偏移**（见 §1）。

## 0. 结论

- `get_building_and_roof_colors` 0x140a94880：**已实现（逐分支判等）**。落在
  `recovered_building/building_colors.rs`，复用既有类型合同 `SelectedCreationAppearance`。
- `create_rectangle` / `create_circle`：**elevation/style 调用序已接线**（两系统顺序不同，
  fresh 逐条定案）。颜色库/默认值/顶层屋顶色与 appearance 已写入 CreateRoofCmd。
- 登记 seam 3 条：create 事件 `Option<GlobalId>` previous（+0/+8）、height/elevation
  （rect +0x28/+0x2c，circle +0x20/+0x24）、0x50B 输出其余字节。
- 新增 `RoofColorComponentTables` 资源（原生数据内嵌 `InnerWalls`，本 crate 的
  `InnerWalls` 未建模）；producer 为登记 seam。

## 1. 0x140a94880 fresh 结构（含 §D 偏移纠错）

### 1.1 输出（sret RCX；调用方缓冲 0x50B，实际只写前 0x1cB）

fresh disasm 的全部写点（0x140a94983-0x140a9499d）：

```
[out+0x18] = EAX      [out+0x11] = RAX(8B)   [out+0]    = EBP
[out+4]    = R8D      [out+0xc]  = R13B      [out+8]    = R14D
[out+0x10] = BL
```

因此：`building_color@0`、`roof_color@4`、`halftimber_color@8`、`wall_style@0xc`、
`roof_appearance@0x10..0x1c`（`flag2`@0x10 + 11B pair@0x11..0x1c）。
**派单 §D 的 `pair@0x30/@0x38` 与本函数不符**：那组偏移属于 history 的 0x50B 记录，
本函数输出只有 0x1cB 有效载荷（0x11 的 8B 写与 0x18 的 4B 写重叠，拼成 12B appearance）。

### 1.2 默认值（`stack+0x40 = &GladeColorCustomization`）

`+0x1c` building、`+0x20` roof、`+0x24` halftimber、`+0x42` building_wall_style
（与 `resources/glade_color_customization.rs` 的 RE-DOMAIN-TYPES 偏移表逐项一致）。

### 1.3 ABI 与查询链（逐分支）

| 位置 | 原生 | 语义 |
|---|---|---|
| RDX | `TEST DL,1` | previous Option 判别位；0 → 全默认 |
| R8 | previous `GlobalId`（u64==WallId） | 探针键 |
| R9 | `&Vec<u32>` | WallColorIds |
| stack+0x28 | `&Vec<u32>` | HalftimberColorIds |
| stack+0x30 | `&PublicWalls` | map A `records`(stride 0x2a8)@+0；map B `roofs`(16B)@+0x30 |
| stack+0x38 | `&InnerWalls` 密集表 | `[0]`位集(+0x28 words/+0x38 bits/+0x110 slot)；`[+8]`records Vec(+8/+0x10，stride 0x14)；`[+0x1a8]`chunk 表(stride 0x48) |
| stack+0x40 | `&GladeColorCustomization` | 四默认值 |

1. map A（`[PublicWalls+0x18]==0` 跳过）：命中后 `point_count(+0x18) < 2` 或
   `length(+0x38) <= 0`（`UCOMISS/JBE`，**NaN 同样 panic**）→ `core::panicking::panic`
   （串 `assertion failed: w.simplified_curve_2d.is_valid()`，0x142ada193 实测）；
   `shape_tag(+0x260) > 1` → 直接默认值（c1 复位）→ 发。
2. `building=WallColorIds[prev]`、`halftimber=HalftimberColorIds[prev]`，各带
   `panic_bounds_check`（先 building 后 halftimber）。
3. map B：空/未命中/high32==0 → 跳到 5。值低 32 = records 下标：
   越界、`records[idx].generation(+0) != high32`、位图位
   `words[bit>>6] >> (bit&0x3f) & 1 == 0` 任一失败 → 5（**不 panic**）。
4. 深层：`chunks[+0xc]`(0x48) 的 `+0x38` 数组按 `presence+0x110` 取值 V →
   `~V*0x30` 取 `+0x18` 数组项 → `+0x10` 叶数组 → `element_index(+0x10)*0x58`：
   `roof_color=leaf+0x50`、`appearance=leaf+0x40..0x4c`。
5. 回探 map A：命中 → `wall_style = +0x2a1`；未中 → `core::option::unwrap_failed`。

## 2. 落地的类型（recovered_building 颜色域）

`crates/country-core/src/recovered_building/building_colors.rs`（新文件，mod.rs 以
`building_color_selection` 别名注出）：

- `PublicWallColorHost{point_count,curve_length,shape_tag,wall_style}` + trait
  `PublicWallColorSource`（map A/B 探针）；为 `PublicWalls` 实现：
  - `color_host` 用 `get_unchecked`（原生未校验探针，校验留在策略内以保 panic 顺序）；
  - `roof_entity_bits` = `roof_entity().to_bits()`（generation 在高 32）。
- `RoofSparseRecord`（0x14：generation@0/bit@4/U@8/chunk@0xc/element@0x10）。
- `RoofPresenceBitset`（words@0x28、bit_len@0x38、slot@0x110）。
- `RoofChunk`（entries@0x18、reverse@0x38，其余 `unobserved_*` 如实留空）。
- `RoofChunkEntry`（只消费 +0x10 叶数组指针；恢复模型内联 `colors: Vec<RoofColorEntry>`）。
- `RoofColorEntry`（0x58：appearance@0x40..0x4c、color_id@0x50）。
- `RoofColorComponentTables{records,presence,chunks}`（`Resource`，seam 资源）。
- `get_building_and_roof_colors(...) -> SelectedCreationAppearance`：逐语句对应 §1.3；
  panic 三处按原生（validation panic / bounds panic / unwrap failed）。
- 单元测试 9 条：默认/缺 host/shape_tag>1/继承/深层覆盖/generation0+位图+generation
  失配/default/bounds panic/NaN panic。

**表示差异（已登记，均为安全超集）**：原生深层读取是无边界 raw load；恢复模型
`get()` 失败归入同一 miss 分支（有效数据下行为一致）。叶数组用内联 Vec 代替裸指针。

## 3. create_rectangle / create_circle 接线（fresh 顺序，二者不同！）

- **create_rectangle 0x140a93790**：`new_wall_w_params`(0x140a93c2d) →
  `update_elevation`(0x140a93c4f) → `get_building_and_roof_colors`(0x140a93c8f) →
  `update_style`(0x140a93ce6) → `update_wall_state_enclosure`(0x140a93df2)。
- **create_circle 0x14097e560**：`new_wall_w_params`(0x14097ea56) →
  `update_elevation`(0x14097ea73) → `update_wall_state_enclosure`(0x14097eaa3) →
  `get_building_and_roof_colors`(0x14097eaea) → `update_style`(0x14097eb40)。
  （派单只写了矩形顺序；圆圈 fresh 明确不同，按原生实现。）

CreateRoofCmd 载荷同步补齐（两处均 fresh）：`+0x10 = out.roof_color`、
`+0x14..0x20 = out.roof_appearance`、`+0x38 = ridge_dir`（矩形
`height>width`；圆圈恒 0）、`+0x39 = settings+0x41 flat_rim_style`。
`build_roof_create_request`（walls 接线文件）签名相应扩展；两个调用点均本任务文件。

消费已有入口：`WallColorIds.0` / `HalftimberColorIds.0` / `PublicWalls::get_unchecked`
/ `PublicWalls::roof_entity` / `GladeColorCustomization`。
`InnerWalls.update_elevation/update_style/update_wall_state_enclosure` 的原生写偏移
（record+0x60/+0x70/+0x40）与现有实现一致（fresh 复核）。

注册：`register_wall_cpu_resources` 增 init 四个资源，系统测试与
`ecs_wall_create_update_dirty_publication_and_delete` 仍可运行。

## 4. 登记 seam（缺数据，不编造）

1. **previous 提示**：原生 create 事件 `+0`（Option tag，仅 bit0）与 `+8`（GlobalId）。
   本轮期间他组 restorer 已给 `CreateRectangleCmd` 增 `previous_wall: Option<WallId>`
   并接入本函数与旧屋顶继承（见 §6）；`CreateCircleCmd` 仍无该字段 → 圆圈走原生
   「无 previous」默认分支。解锁条件：UI 生产者把 `Option<WallId>` 写入 circle create 事件。
2. **height/elevation**：矩形事件 `+0x28/+0x2c`、圆圈 `+0x20/+0x24`；恢复命令未携带。
   现 `new_wall_w_params` 与 `update_elevation` 均使用既有 `SHAPE_ELEVATION=4.0`。
   解锁条件：create 事件补 height/elevation 字段。
3. **旧屋顶继承**：原生 `if *prev_tag==1` 读旧 roof `+0x3d` style、`+0x30` pitch
   （pitch<=0.1 覆写 height_01）——矩形已由他组 concurrent restorer 用
   `RoofColorEntry::style()/pitch_01()` 接入；圆圈无 previous 面。
4. **0x50B 输出余量**：out+0x1c..0x50 原生同样不写（调用方只用前 0x1c），非缺口。
5. `RoofColorComponentTables` producer：原生表内嵌 `InnerWalls+0x1a8`；本 crate 的
   `InnerWalls` 未建模该密集表。解锁条件：roof/inner-walls 侧落地真实数据填充。
6. 旧近似体 `wall_authoring::select_creation_appearance` 仍在 placement 路径
   （非本 create 链），其 validate-fail 仍返回 `Err`；native 语义以本函数为准，
   未改它组在途调用面。

## 5. 验证（最终状态）

- `cargo check -p country-core --lib`：**0 error**（仅既有 warning）。
- `cargo test -p country-core --lib building_colors`：**9 passed**。
- `cargo test -p country-core --lib create_rectangle`：**5 passed**。
- `cargo test -p country-core --lib create_circle`：**6 passed**。
- `cargo test -p country-core --lib wall`：**142 passed**（含
  `registration::tests::ecs_wall_create_update_dirty_publication_and_delete`）。
- `cargo test -p country-core --lib`（全库）：**1966 passed / 0 failed**。

过程说明：验证收尾阶段工作树被他组在途编辑多次短暂打断（`public_walls.rs` 测试的
`Curve<Vec2>` vs `Curve2`、`decorator_storage.rs` 测试的 `gen` 保留字、
`ui_create_*` 的 `flag` bool/u8、`input_replay_for_trailers` 可变性）；均由对应
agent 随后修复，未代改他域，最终全库测试通过。

## 6. 并发集成说明（他组 restorer 在本轮期间修改）

`create_rectangle.rs` / `shapes/mod.rs` / 本任务新增的 `building_colors.rs` 在
19:40-19:47 被他组 restorer 并发编辑并**采纳了本轮产出**：
- `CreateRectangleCmd` 增 `previous_wall: Option<WallId>`，本函数改为传入真实
  previous（seam 1 矩形侧闭合）；
- `RoofColorEntry` 增 `pitch_01()`（+0x30）与 `style()`（+0x3d），矩形旧屋顶
  style/pitch 继承接入（seam 3 矩形侧闭合）；
- 增 `RecordRoofHistory`/`RoofHistoryEdit::create` 两条 History 写入。
这些改动与本报告实现兼容（`get_building_and_roof_colors` 签名与 `RoofColorEntry`
布局未变）；本报告 §1-§3 的 fresh 结论仍适用。冲突处理：本任务只修正了交叉编辑后
自身测试的借用错误，未回退他组改动。
