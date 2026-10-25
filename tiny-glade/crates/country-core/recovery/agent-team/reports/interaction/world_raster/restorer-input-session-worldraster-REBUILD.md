# REBUILD — world_raster 5（restorer，2026-10-17）

agent: restorer-input-session-worldraster
方法: ghidra `--worker shared` fresh 反编译（0x140ae4c20 / 0x140ae4d00 /
0x140b1c270 / 0x140b1c010 / 0x140b68e40 + 0x140ae4bd0 / 0x14092df50 / 0x140aba770 /
0x140ab8090 / 0x140ab4750）+ `disassemble_function` + radare2 逐段反汇编（draw_path
跳表目标 0x140ae5311/0x540a/0x54f1/0x55d4/0x56b7、draw_raster 实例 prologue/begin）
+ `read_memory`（跳表 0x142ae2fa4、常量 0x142a16cb8/0x142a95dd0/0x1429258d0/
0x142a8d600/0x142925904、字符串区 0x142ae2e80、draw_path 全文 1677 B）。
证据: `reports/interaction/world_raster/evidence-restorer-2026-10-17/`
（fresh-batch1/2/3.json、disasm-wr.json、disasm-cases.json、r2-*.json、bytes-dp.json、mem-*.json）。

## 1. 逐函数 verdict

| 函数 | VA | verdict | 说明 |
|---|---|---|---|
| PathRastersResources::as_float_grid | 0x140ae4c20 | 判等（签名重写） | 原生**无 index 参数**：sret 写 4×{ptr,len}=64 B，顺序 Path/C1/C2/C3，各按 `selector@+0x298` 选 composite(+8/+10) 或 scratch(+20/+28)。Rust 改为 `as_float_grid(&self) -> [RasterBufferRef<'_,512>;4]`（`WorldRaster::as_ref()` 已按 selector 选当前缓冲）。 |
| draw_path | 0x140ae4d00 | 判等（结构级逐分支；2 项登记） | 见 §2.1。 |
| record_path_history | 0x140b1c270 | 判等 | 4 个同构块（Path/C1/C2/C3）：reader 有未读事件 → 对应槽 take_history_stroke + `History::add(EditType::…)`；槽基址 +0x48/+0x4b8/+0x928/+0xd98（= slot 0..3，stride 0x470；garden slot5=0x1678 已由 eco/garden LOGIC 互证）。事件载荷被忽略。 |
| path_brush_preview_size | 0x140b1c010 | 判等 | 门 `AppMode` tag ∈ {7,8}；`size = BrushSizes.path * 1.25`（0x142a8d600 实读 0x3fa00000）；`scale.x=scale.z=size`，`scale.y = MAXSS(size,1.0)`（0x142925904=1.0；NaN→1.0）。原生只写第一个匹配实体。 |
| impl$0::build（PathsPlugin） | 0x140b68e40 | 判等 | `add_systems(Update, record_path_history.in_set(GameSystemSet))`；insert PathRaster/C1/C2/C3（WorldRaster::new<512>）+ BrushSizes；register_event RecordPath/RecordCobblestone1/2/3EditToHistoryCmd + PathDrawEvent + PathEraseEvent。 |

## 2. 重建明细

### 2.1 draw_path 0x140ae4d00（fresh 结构）
1. `AppMode` tag：Path=7、Cobblestone=8、Eraser=9；`r=tag-5 ∈ {2,3,4}`，其余 return。
   - Cobblestone payload 断言：`payload-1 < 3` → payload ∈ {1,2,3}；
   - Eraser：`layer & 5 != 0 → return`（只擦 All/Path）。
2. 半径：Path/Cobblestone = `BrushSizes.path`；Eraser = `EraserSizeMultiplier * 1.68`
   （0x142a95dd0 实读 0x3fd70a3d = 1.68；旧常量 0.5525 已修正）。三者
   `assert!(brush_size.is_finite())`（0x140ae57c6/57de/57f6）。
3. `UiState::clearing_canvas_w_layer_and_sound(name, arg, 3)`：
   ("path",3) / ("cobblestone",3) / ("eraser",7)（字符串 0x142ae2e80/ef0/f10，r9b 实读）。
4. `Circle2d::new((x,z), radius*0.8)`（0x142a16cb8=0.8）+ `GladeBorder::is_shape_inside`；
   `allowed = !draw || inside`（Eraser 原生短路，不经边框检查）。
5. 圆外：`DisplayOutOfBorder` 恒写；`widget.dragged() != 2` 时写 `FeedbackInputOutOfBorder`
   （0x140ae506f..0x50cb niche 检查）。
6. `DrawState::update` 内联：r=phase-2 min3；r∈{1,2} 且旧 state∈{1,2} → state=2 +
   `PathPinnedClutterCircle{circle: Circle2d::new(center, radius), height: pos.y+1.5|1.5}`
   （仅 draw）；r∈{1,2} 否则 → state=allowed；r∈{0,≥3} → 仅旧 state∈{1,2} 时 state=3，
   否则 state=0 + return。命令 discriminant：0（begin）/2（extend）/3（commit）。
7. 分派（jump table @0x142ae2fa4，5 项）：
   | selector | 目标 | 先擦（draw=false，空回调） | 目标调用 |
   |---|---|---|---|
   | 0 Path | slot 0 | C1,C2,C3 | PathRasterStagingSsbo |
   | 1 C1 | slot 1 | Path,C2,C3 | Cobblestone1RasterStagingSsbo |
   | 2 C2 | slot 2 | C1,Path,C3 | Cobblestone2RasterStagingSsbo |
   | 3 C3 | slot 3 | C1,C2,Path | Cobblestone3RasterStagingSsbo |
   | 4 Eraser | slot 0 | C1,C2,C3 | PathRasterStagingSsbo（draw=false→on_erase） |
   目标回调写 PathDrawEvent/PathEraseEvent（position,radius,observed_flag）+ 对应
   Record*EditToHistoryCmd。draw_raster 实例按地址核对：Path draw=0x140aba770、
   Path erase=0x140ab4750、C1 erase=0x140ab2340、C2 erase=0x140ab6de0、
   C3 erase=0x140ab8090、C1/C2/C3 draw=0x140abba80/0x140ab0f10/0x140ab9340。
8. 点 = `cursor.position()`（tag&1 才非零，0x140ae52be..0x52eb）。

### 2.2 record_path_history
`RasterHistoryRecordResources`（原生在 `systems::world_raster`）= `ResMut<
ActiveRasterStrokesStorage>` + `ResMut<History<()>>`；4 个事件命令类型定义在
`systems::paths::record_path_history`。实现复用 `world_raster::record_raster_history`
（与 garden 同族）。

### 2.3 build
`PathsPlugin::build(world, schedule)` 按原生插入 4 个栅格 + BrushSizes、注册原生 7 个
事件、把 `record_path_history` 加入 Schedule。另注册本 crate 的
`PathPinnedClutterCircle` 特化事件（原生 `ShowPinnedDecoClutter` 由 ui_clutter 域注册）。
crate 无 bevy_app / `GameSystemSet`：宿主按 Update + 组顺序接入（登记）。

## 3. 登记（U，非阻断）

1. **`RasterStrokeCmd.strength`**：native draw_path 两条命令栈槽 +0x8c/+0x9c 从未写入
   （反编译、反汇编与原始字节 ModRM 扫描均确认）；读取方 `start_new` 把它当 param_b。
   本实现置 `PATH_STRENGTH = 0.0`，登记。影响：rasterize_stroke 的 blend strength。
2. **`erase_other_path_variants` 的 9 个闭包载荷**：原生 `closure_env_0..8` 捕获其它
   栅格类型；本实现按"擦除、无事件回调"执行这三类调用（与 draw_raster 的 on_* 语义
   一致的部分已逐分支还原；闭包内部未逐字节恢复）。
3. **PathPinnedClutterCircle**：对应原生 `ShowPinnedDecoClutter` 56 B union 的
   circle/height 特化（tag word 0x100、circle@+0x04、height@+0x18、event_id@+0x30）。
   本 crate 以同族特化类型承载（garden 同做法），union 其余载荷与渲染器未恢复。
4. **`PathDrawEvent/PathEraseEvent` 载荷**：以 `{position, radius, observed_flag}` 承载
   draw_raster 回调实参（与 GardenDrawEvent 同形）；`resources::path_stroke` 原缺
   PathEraseEvent，已补。逐字节 0x40 B 事件布局（未走 draw_raster 回调的边框事件）未建模。
5. **`as_float_grid` 返回裸 {ptr,len} 对**：Rust 用 `RasterBufferRef`（含 `&[u8]`）等价。

## 4. 自检

- `cargo check -p country-core --lib`：通过（本域文件 0 warning）。
- `cargo test -p country-core --lib`：**1979 passed / 0 failed**（交付时全 crate；基线 1966 + 本域新增 9 + 并行组增量）；新增 9 个测试（world_raster 6 + input 3）：
  - `path_begin_extend_commit_records_the_path_slot_once`（begin/extend/commit 全链 +
    draw/pin 事件 + PathRaster 像素 + History EditType::Path）
  - `cobblestone_targets_their_own_slot_and_widget_argument`
  - `eraser_layer_gate_and_erase_target`（layer&5 门 + 1.68 半径 + draw=false）
  - `preview_size_sets_uniform_scale_with_a_minimum_height`
  - `as_float_grid_returns_all_four_current_buffers_in_path_order`
  - `record_path_history_records_each_slot_only_when_its_reader_has_events`
  - （input 域 2 个见 input REBUILD）
