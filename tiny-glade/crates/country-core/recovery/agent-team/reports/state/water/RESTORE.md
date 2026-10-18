# system-water RESTORE 报告（state 组 restorer，2026-10-17 fresh 重建）

- 目标 crate：`crates/systems/water`（package `system-water`）
- 方法：fresh 反编译（ghidra shared worker）+ 原指令反汇编（disassemble_function）+ 只读字节（read_memory）+ 常量内存直读；逐函数重建控制流图，Rust body 与之逐语句判等。
- 证据落盘：`reports/state/water/_scratch/`（`result_fresh.json`、`result_disasm.json`、`result_tail2.json`、`result_mem.json`、`result_verify.json`、`draw_water_asm.txt`、`tail2.bin` 等；`disvenv/` 仅本地 capstone 反汇编器，非证据）。
- 编译：`cargo check -p system-water` **通过**；`cargo clippy -p system-water` 无 system-water 告警。core 侧另有并发 agent 的 13 条 warning，无 error。

## 函数判定汇总

| 函数 | 地址 | 判定 | 说明 |
|---|---|---|---|
| `draw_water` | 0x141ba9130 | **重建**（业务） | 逐语句对应 fresh 反编译 + 原指令；本报告 §1 |
| `draw_water::closure$0` | 0x141ba9700 | **未落库-依赖 core signifier 域**（O 面登记） | 逻辑全量分析（O）；依赖 core `SignifierEntityBuilder`（core signifier 域未恢复），crate 内无实现——见 §1.1 + 交付缺口清单 §5 |
| `draw_water::closure$1/2/3` | inline | **glue-legitimate** | `EventWriter::write`/`write_default` 胶水，行 161/164/167 |
| `update_water_audio_grid` | 0x141baefe0 | **重建**（业务） | 逐语句对应 fresh 反编译 + 全反汇编；§2 |
| `record_water_history` | inline（宿主 0x140e92a80） | **重建**（业务） | `read().last()` 未读 → 单次 `record_raster_history`；§3 |
| `WaterPlugin::build` | 0x1406501d0 | **glue-legitimate** | Bevy 注册胶水；注册顺序核对应原面；§4 |

## 1. `draw_water`（0x141ba9130，重建）

Fresh 证据：`result_fresh.json`（decomp）、`result_disasm.json` + `result_tail2.json`（原指令，含被 disassemble_function 跳过的 switch/tail 块，capstone 重解）、`result_verify.json`（0x141ba9130 开头字节复核）。

控制流图（与 Rust body 逐语句对应）：
1. **门禁**：`*0x14362c098 != 0 → return`（0x141ba915e/61）。
2. **模式分派**：`sel = tag - 5`；`sel==4`（tag 9 橡皮，`layer&3==0` → All/Water）、`sel==10`（tag 15 水）、其余 return。
   - 橡皮：`t = inverse_lerp(0.3, 2.0, eraser_mult)`（0x140c904a0，XMM0=0x1429258d8、XMM1=0x14292e6f0、XMM2=[brush]）→ `brush_size = t*4 + (1-t)`（0x142934694/0x142925904）→ `assert!(brush_size.is_finite())`（panic 串 0x142eebba0，R8=0x142eebc10）→ `clearing_canvas("eraser",6,0)`（0x142eebc08，R8=6、R9=0）→ `cursor.position()`（0x140a12650，**死调用**，结果被覆盖）→ `intersect_with_xz_plane(0.0)`（0x1408ef180，XMM2=0）→ `format!` 名字串 "eraser_water_"（模板 @0x142eebc2d，13 字节）→ `SignifierFactory::new`（0x14089d8b0）→ `UiSignifierStream::write` 40B 记录（0x140b2cf90）；临时名字串随后 `__rust_dealloc`。`inside=true`。
   - 水：`brush_size = BrushSizes.water`（偏移 4）→ `assert`（R8=0x142eebbf0）→ `clearing_canvas("watereraser_water_",5,4)`（**名字为 "watereraser_water_"，非 "water"**，0x142eebc28）→ `position()`×2（RBP+0x18=draw_raster 点、RBP+0x28=界内圈圆心）→ `Circle2d::new(center, 0.8*brush_size)`（0x140c97540，0x142a16cb8）→ `is_shape_inside`（0x140ad2970）→ 界外 `DisplayOutOfBorder` + 拖拽中 `FeedbackInputOutOfBorder`（`dragged()!=2` = `into_drag_state().is_some()`，0x14092df10→0x14092e6b0）。
3. **共用尾**：`DrawState::update(mode_id=dragged, raw=inside)`（0x140ae4bd0）→ 4 相位 switch（0x142eebcb0 jumptable，case0→return、case1→disc 0、case2→extend、case3→disc 3）。case2：`!inside→return`；draw 时**重取** `position()`+`Circle2d::new(radius=brush_size)`+高度 `pos.y+1.5`（0x1429258d0）+union 标志 0x100 → `ShowPinnedDecoClutter`（0x142260a60）。
4. **尾**：`RecalculateHeightMapCmd` 单元事件 → `on_terrain_modified`（0x140aabdc0）→ `draw_raster<WaterRasterStagingSsbo, WorldRaster<512>>`（0x142260ae0）。

关键纠错（相对旧源码）：
- 水 `clearing_canvas` 名：`"water"` → **`"watereraser_water_"`**（O 字节）。
- 橡皮 `draw_raster` 点：旧源码传 `cursor.position()`，原版传 **`intersect_with_xz_plane` 交点**（RBP+0x18 被覆盖）——已修。
- 图钉：case2 **重取** `cursor.position()`（旧源码用缓存 position）；半径=brush_size（非 0.8×）。
- `assert!` 变量名 `brush_size`（panic 串原文），无自定义消息（旧源码带 "water brush radius must be finite"）。

橡皮 signifier 名（reviewer FIX-1 修正）：原版 `name` **非恒 0**——`format!` 生成名字串
"eraser_water_"（模板 @0x142eebc2d，13 字节），`SignifierFactory::new`（0x14089d8b0，
decomp = `{name_len,name_ptr,name_len,&inner,marker}`）把名字串**拷贝**进 40 B 记录。
**core `UiAppendSignifier`（tag/kind/pos/scale/name）布局疑与原 40 B 记录不一致**（tag/kind/
pos/scale 对应原记录的内层 `{1,1,pos,scale}`；name:u64 无法承载名字串）——**向 core signifier
域登记复核**（本 crate 以 `name` = 名字长度 13 表达"记录带名"，见 draw_water.rs
`ERASER_SIGNIFIER_NAME`）。

### 1.1 closure$0（0x141ba9700，未落库-依赖 core signifier 域）

Fresh：`result_fresh.json`。体（O）：`"brush_circle_scribble"` 名（0x21 字节）→ `SignifierEntityBuilder::new().animation_duration().position().scale().set_ui_pass().build()` → 返回 1 元素 `Vec<Entity>`（len/cap=1）。捕获 `(AssetMeshLibrary, AssetShaderLibrary, Commands)`（PDB closure_env$0）。

**判定为未落库**：crate 内无该闭包实现（依赖 core `SignifierEntityBuilder`，core signifier 域未恢复），仅 O 面登记。见交付缺口清单 §5。

### 1.2 closure$1/2/3（inline，glue-legitimate）

行 161/164/167：`EventWriter::write<WaterDrawEvent>` / `write<WaterEraseEvent>` / `write_default<RecordWaterEditToHistoryCmd>`（PDB inlinee 7657/9992）。即 `draw_raster` 的 on_draw/on_erase/on_record 闭包，载荷 {position Vec3, radius f32}（O 消费面）。**旧源码 on_record 闭包已改 write_default 语义一致**。

## 2. `update_water_audio_grid`（0x141baefe0，重建）

Fresh：`result_fresh.json`、`uwag_full.txt`（369 条原指令）+ `result_disasm.json`（复核一致）、`result_mem.json`（常量 0x142eed8c0=0x000e000e、0x142eed8d0=-7 对、0x142eed8e0=0.0625、0x142eed8f0=13.0、0x142eed900=6400.0、0x142925870=0.5、0x142a7b658=130.0、0x43020000=260.0、0x142925904=1.0）。

控制流图：
1. **puffin 门控**（glue）：`are_scopes_on()`；scope 名 rfind 处理 + begin/end_scope（profiling，glue-legitimate）。DEMO 门禁 `*0x14362c098 != 0 → return`（scopes off 面 `JZ main_body`）。
2. **帧计数**：`state.frame = f + 1`（pre-increment，0x141baf318）。
3. **帧位/抖动**：`frame_bit = f + (f+1)*10 + 1 = 11*(f+1)`；`bit = 1 << (frame_bit & 63)`（SHL R13,CL）；增量对 `d = {((2*fb)&0xe)-7, ((fb>>2)&0xe)-7}`（PAND 0x000e000e + PADDW -7，i16 回绕）；位置抖动 `jitter = d * 0.0625 * 13.0`（CVTDQ2PS→MULPS×2）。
4. **相机眼位**：`MainCamera::eye_position`（0x140911dd0）。
5. **遍历 24B 单元**（步长 0x18）：
   - `sample = 单元基址{x,z} + jitter` → `x0y`（0x140c9cdc0）→ 点 `{x,0,z}`；
   - 距离² `(ex-px)² + ey² + (ez-pz)² ≥ 6400 → 清位`（UCOMISS+JBE，拒 NaN）；
   - 水位 `bilinear_sample_at_world_xz(px,pz,130,260)`（0x140a14df0）`≤ 0.5 → 清位`；
   - `intersects_any(ray={点, 眼-点}, mask=0x32a3)`（0x1408cc320）命中 → 清位；
   - 置位：`window |= bit`；0→1 时 `counter += d`（PADDW）；清位：`window &= ~bit`；1→0 时 `counter -= d`（PSUBW）。
6. **窗口/计数器**：原版存于单元 +0x08/+0x10（本 crate 移入 `WaterAudioGridUpdateState` Local，差登记）。

关键纠错（相对旧源码）：
- **采样点含每帧抖动**：旧源码用纯基址采样；fresh 确认 `sample = 基址 + jitter`。
- **计数器 ±d（增量对）**：旧源码累加 `coord`（格坐标）；fresh 确认累加 `{sVar14,sVar16}` 增量对（与 core 叶子 `apply_audio_bit` 的 PADDW/PSUBW 语义一致；`recovered_systems/water.rs` 已据此标注）。

### 分歧点（登记）
- **`WaterAudioGrid` 类型合同**：原版 24B 单元 = {f32 x, f32 z, u64 窗 @+8, u16×2 计数 @+0x10}，`update_water_audio_grid` **只写窗口/计数器**；core 当前 `WaterAudioGrid` 为语义模型（`water_level`/`position`，见 `systems/audio/water_ambience_audio.rs`）。本 crate 把真实窗口/计数器计算放入 Local 状态，并把窗口占比 `count_ones()/64` 投影为 `water_level`、把计数器质心投影为 `position`——投影为 crate 内 I 面（原版消费方从窗口/计数器推导，音频域待裁决）。
- **`intersects_any` 参数面（语义差，验收需显式确认）**：原版 `(ray, max=1.0, mask=0x32a3)`——max=1.0 只测采样点附近 1.0 距离；core 当前 API 为 `(terrain, origin, dir, mask)`（mask 为 u32、**无 max，地形短路用 f32::MAX**）。行为差：core 版本会命中更远地形 → 清位更多。本 crate 增加 `terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>` 参数并适配；若 core 增加 max 参数则回填。**此差异已登记并需验收显式确认纳入 core API 合同。**
- **NaN 剔除边界（微差）**：原版 `UCOMISS+JBE` 在 dist²/水位为 unordered(NaN) 时清位；Rust 已用 `!(x<y)`/`!(x>y)` 形式对齐（NaN → 清位）。输入（相机眼位、格坐标、水位采样）有限时不可达。
- 网格规模/单元基址：原版基址由栅格构建方（音频域 init）预写；本 crate 按 `cell_coord(index)*spacing`（I）重建（-7..7 × 0.8125）。

## 3. `record_water_history`（inline，宿主 0x140e92a80，重建）

Fresh：`result_fresh.json`（宿主 `run_unsafe<FunctionSystem<fn record_water_history>>` decomp，module 1011）。

逻辑（O）：取资源 → EventReader 双缓冲未读事件计数（`saturating_sub`）→ **任一缓冲有未读**（`uVar4>=uVar9 && !=0 || uVar3>=uVar5 && !=0`）→ 调 `country_core::systems::world_raster::record_raster_history`（0x957990）→ 更新游标 last_event_count + last_run tick。取**末条**（`.read().last()`），非逐条。

关键纠错（相对旧源码）：
- **删除 `HistoryRecordingFailed` 发射**（旧源码按 core garden `record_garden_history` 惯例近似）：原版调用后结果被忽略、无错误发射。签名还原为 `(EventReader, ResMut<ActiveRasterStrokesStorage>, ResMut<History<S>>)`；`register_water_events` 同步移除 `Events<HistoryRecordingFailed>`。
- 槽位 4 / `EditType::Water`（O：ANALYSIS §4 + 当前 core `record_raster_history` 契约）。

## 4. `WaterPlugin::build`（0x1406501d0，glue-legitimate）

Fresh：`result_fresh.json`（decomp 完整）。

注册面（O）：① `record_water_history.in_set(GameSystemSet)` → `add_systems(Update, …)`；② `update_water_audio_grid.run_if(Option<Res<WaterAudioGrid>>.is_some())` → `Interner::intern<dyn SystemSet>` + `in_set_inner` → `add_systems(Update, …)`；③ `DirtyTilePublisher::unique()` + `PathRaster::new()` → `insert_resource<WaterRaster>`；④ `register_event`×3（RecordWaterEditToHistoryCmd / WaterDrawEvent / WaterEraseEvent，各带"已存在则跳过"判空，第三个命中时 return）。

按分类规则标 glue-legitimate 不重建。crate 的 `register_water_events`/`add_water_cpu_systems` 为**自足接线**（追加注册自身系统写出的 core 事件、追加注册 `draw_water`——原 build 只 add 两系统，`draw_water` 属模块外接线），差登记：注册顺序与原面一致（资源→事件），系统集合名/`PathRaster::new()` 去向按 ANALYSIS 记为 I。

## 5. 交付缺口清单（未落库/待裁决）

| # | 项 | 判定 | 处置 |
|---|---|---|---|
| 1 | `draw_water::closure$0`（0x141ba9700） | 未落库-依赖 core signifier 域 | 体已 O 面全量分析（`SignifierEntityBuilder` 链 + "brush_circle_scribble"）；core `SignifierEntityBuilder` 恢复后落库（reviewer FIX-2）。 |
| 2 | 橡皮 signifier 记录布局 | 待 core signifier 域复核 | 原 40 B 记录 = `{name_len,name_ptr,name_len,&inner,marker}` vs core `UiAppendSignifier`（tag/kind/pos/scale/name）——已登记（reviewer FIX-1）。 |
| 3 | `intersects_any` max=1.0 | 语义差，待验收显式确认 | core API 无 max（f32::MAX 地形短路）；若 core 增加 max 参数则回填。 |
| 4 | `WaterAudioGrid` 语义投影 | 待音频域裁决 | 原窗口/计数器 → 本 crate Local + 投影 water_level/position。 |

## 附：fresh 证据索引

| 证据 | 位置 | 内容 |
|---|---|---|
| fresh decomp | `_scratch/result_fresh.json` | draw_water / update_water_audio_grid / build / closure0 / run_unsafe(0x140e92a80) |
| 原指令 | `_scratch/result_disasm.json` + `draw_water_asm.txt` | draw_water + update_water_audio_grid 全反汇编 |
| switch/tail 块 | `_scratch/result_tail2.json` + `tail2.bin` | 被 disassemble_function 跳过的 0x141ba94ed..0x141ba963f |
| 常量内存 | `_scratch/result_mem.json` | 0x142eebc00 串区（"eraser"/"watereraser_water_"/"brush_circle_scribble"）、0x142eed8c0 常量区、0x142eebcb0 jumptable、0x142eebba0 panic 串 |
| 复核 | `_scratch/result_verify.json` / `result_verify2.json` | 函数头字节 + jumptable 目标 + panic 串 "assertion failed: brush_size.is_finite()crates/systems/water/src/draw_water.rs" |
| 独立复核（reviewer） | `_review_scratch/result_factory.json` / `result_name.json` / `result_write.json` | SignifierFactory(0x14089d8b0) decomp（`{name_len,name_ptr,name_len,&inner,marker}`）、0x142eebc20 串区（"eraser_water_"@0x142eebc2d）、EditType::Water 构造器标签 |
| 既有分析 | `upper-modules-plan/systems/water/{ANALYSIS.md,TASKS.md}` | 指引非凭据，fresh 已复核/纠错 |
