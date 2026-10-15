# systems/water 模块内部逻辑分析（B 版）

> 由批次 1 分析 agent 产出，父级代为落盘。证据临时文件在 `_scratch/`（反编译/反汇编/常量）。

分析对象：B patched EXE（`tiny-glade.exe`，sha256 `cf048e27…`），PDB `2d9686b8…`。
数据源：骨架 `skeleton/B/crates/systems/water/src/*`、Ghidra 定点反编译、原指令反汇编与只读内存常量、既有 `evidence/country-core-deep/terrain-water/DIGEST.md`。

口径：**只分析模块内部实现**；依赖库直接复用；不展开 caller/callee 与 GPU 下游。**O**=机器码/PDB 直接支持，**I**=调用链推断，**U**=未知。地址：骨架给 RVA，Ghidra 用 VA，基址 `0x140000000`。

本 crate 命名空间下 4 个 src 文件、4 个本 ns 函数 + 4 个闭包。

---

## 1. `src/lib.rs` — `impl Plugin::build`（`system_water::impl$0::build`）

- 符号：`system_water::impl$0::build`；`compiled_procedure`；源码行 21。
- 地址：RVA `0x6501d0` / **VA `0x1406501d0`**。
- PDB 命名实体：`system_water::WaterPlugin`（字段 unknown；`Plugin::{name,ready,finish,is_unique}` 关系齐全 → 确认 `impl Plugin for WaterPlugin`）。

### 内部逻辑（O，`_scratch/plugin_build.txt`）
1. `in_set< fn record_water_history, …, GameSystemSet >()` → 系统放进 `country_core::systems::GameSystemSet`。
2. `SubApp::add_systems::<…, Update, …>()` → 注册 **#1 = record_water_history** 到 `Update`。
3. `run_if< fn update_water_audio_grid, …, bool(Option<Res<WaterAudioGrid>>) >()` → 挂运行条件 `Option<Res<WaterAudioGrid>>.is_some()`。
4. `Interner::intern<dyn SystemSet>` + `in_set_inner`。
5. `add_systems` → 注册 **#2 = update_water_audio_grid** 到 `Update`。
6. `DirtyTilePublisher::unique()`（core）；7. `PathRaster::new()`（core）；8. `App::insert_resource<WaterRaster>()`。
9. 三次 `EventRegistry::register_event`：`RecordWaterEditToHistoryCmd`、`WaterDrawEvent`、`WaterEraseEvent`（各带“已存在则跳过”判空）。

### 关键结论
- **本插件只 add 两个系统**：`record_water_history`（`in_set(GameSystemSet)`）、`update_water_audio_grid`（`run_if(Option<Res<WaterAudioGrid>>.is_some())` + `in_set`）。**`draw_water` 不由本插件注册**（build 内无第三次 add_systems；其注册在别的插件/集成层，属模块外接线）。
- 注册 3 个事件；资源插入 `WaterRaster`（明确），`DirtyTilePublisher::unique()`/`PathRaster::new()` 亦在 build 内构造（单态内联，**I**）。

### 算法/常量
- 无自研算法，纯 Bevy 注册胶水。无常量。

### 未知
- `PathRaster::new()`/`DirtyTilePublisher::unique()` 是否各走 `insert_resource`（单态内联）→ **U（低风险）**。
- `add_systems` 的 before/after 标签（只见 in_set）→ **U**。

---

## 2. `src/draw_water.rs` — `draw_water`（+ closure 0..3）

| 项 | 值 |
|---|---|
| 符号 | `system_water::draw_water::draw_water`；源码行 28 |
| 地址 | RVA `0x1ba9130` / **VA `0x141ba9130`** |
| closure$0 | RVA `0x1ba9700` / VA `0x141ba9700`（独立实例，源码行 89） |
| closure$1/2/3 | 仅 `inline_instance`（源码行 161/164/167）；内联宿主 = `country_core::systems::world_raster::draw_raster<WaterRasterStagingSsbo, WorldRaster<512>, env$1, env$2, env$3>` |

### 签名（PDB 文件级候选接口，O 结构 / I 字段名）
```
fn draw_water(
    mode: Res<AppMode>, raster_draw: RasterDrawResources, ui_state: Res<UiState>,
    water_raster: ResMut<WaterRaster>,
    water_draw: EventWriter<WaterDrawEvent>, water_erase: EventWriter<WaterEraseEvent>,
    record_hist: EventWriter<RecordWaterEditToHistoryCmd>,
    recalc_height: EventWriter<RecalculateHeightMapCmd>,
    scheduled_readback: Res<ScheduledTerrainReadback>, cursor_terrain: Res<CursorTerrainRaycast>,
    (glade: Res<GladeBorder>, out_of_border: EventWriter<DisplayOutOfBorder>,
     fb_out_of_border: EventWriter<FeedbackInputOutOfBorder>),
    draw_state: Local<DrawState>, signifiers: Res<UiSignifierStream>,
    camera_cursor: Res<CameraCursorRay>,
    (eraser_mult: Res<EraserSizeMultiplier>, brush: Res<BrushSizes>),
    show_pins: EventWriter<ShowPinnedDecoClutter>,
)
```

### 内部逻辑（O，`_scratch/draw_water.txt`）
水面笔刷工具系统（AppMode 分派 + DrawState 状态机 + 事件发射 + 光栅上传调度）：
1. **全局门控**：`if *country_core::_DEV_OVERRIDE_IS_DEMO_MODE == 0` 才执行主体。
2. **工具分派**：读 `AppMode` 判别式，`sel = AppMode-5`（AppMode∈[6,20]）。
   - `sel==4`（**AppMode=9，橡皮**）：`utils::inverse_lerp` 求 t，`radius = t*4.0 + (1-t)`（lerp[1,4]），`*0.5` 存 signifier `+0x1c`（半径 0.5..2.0）；`UiState::clearing_canvas()`；`CursorTerrainRaycast::position()`+`CameraCursorRay::intersect_with_xz_plane()` 求光标点；`SignifierFactory::new()`→`UiSignifierStream::write()` 发 **"eraser"** 光标符。
   - `sel==10`（**AppMode=15，水笔**）：读若干 core 资源字段；`UiState::clearing_canvas()`；两次 `CursorTerrainRaycast::position()`+`Circle2d::new()`（圆刷）；`GladeBorder::is_shape_inside()` 判界内。界内→向事件缓冲 push（`grow_one`+写 `EventInstance`）；`slider3d::into_drag_state()`+`DragState::is_some()`；拖拽中再 push；`EventWriter::write<ShowPinnedDecoClutter>`。界外→发 `DisplayOutOfBorder`/`FeedbackInputOutOfBorder`。发 **"watereraser_water_"** 光标符（scale=`fStack_10c+1.5`）。
3. **DrawState 状态机**：`DrawState::update()` 后对 `*draw_state` 做 4 路 switch（case 0..3 相位），按相位写 WaterDraw/WaterErase 事件。
4. **后处理**：写 `RecalculateHeightMapCmd`（`grow_one`）；`ScheduledTerrainReadback::on_terrain_modified()`。
5. **上传**：`world_raster::draw_raster<WaterRasterStagingSsbo, WorldRaster<512>, env$1, env$2, env$3>()` 把本帧笔触经 core 写入 `WaterRaster` 并上传 staging（GPU 消费后置）。

### 闭包
- **closure$0**（VA 0x141ba9700，O）：捕获 `(AssetMeshLibrary, AssetShaderLibrary, Commands)`；体：`builtin_strncpy("brush_circle_scribble",21)`→`SignifierEntityBuilder::new().animation_duration().position().scale().set_ui_pass().build()`→返回 1 元素 Vec（笔刷圆符实体）。
- **closure$1**（inline）：`EventWriter::write`→**WaterDrawEvent**。
- **closure$2**（inline）：`EventWriter::write`→**WaterEraseEvent**。
- **closure$3**（inline）：`EventWriter::write_default`→Default 事件（**U 类型**）。

### 算法/常量
- 算法：笔刷半径 lerp 映射；光标射线→XZ 平面求交；圆刷几何 `Circle2d`；林地边界包含 `is_shape_inside`；DrawState 相位状态机（core）。
- 常量：橡皮半径 lerp 端点 1.0/4.0 再 `*0.5`；水笔符 scale `+1.5`；串 `"eraser"`/`"watereraser_water_"`/`"brush_circle_scribble"`；**AppMode 9=橡皮、15=水笔**。
- 数据结构：`WaterRaster=WorldRaster<512>`（512×512 u8）、`DrawState`、`Circle2d`、signifier 实体。

### 未知
- DrawState 四相位语义及事件对应 → **U**。
- 水笔分支读的 core 资源字段偏移归属 → **U（core 侧）**。
- closure$3 的 `write_default` 事件类型 → **U**。
- `RasterDrawResources` 内部组成 → **U（模块外）**。

---

## 3. `src/water_audio_grid.rs` — `update_water_audio_grid`

| 项 | 值 |
|---|---|
| 符号 | `system_water::water_audio_grid::update_water_audio_grid`；源码行 21 |
| 地址 | RVA `0x1baefe0` / **VA `0x141baefe0`** |
| PDB 命名实体 | `WaterAudioGridUpdateState`（Local 状态，字段 unknown） |

### 签名（高置信）
```
fn update_water_audio_grid(
    state: Local<WaterAudioGridUpdateState>, water_raster: Res<WaterRaster>,
    raycasting: Raycasting, camera: Res<MainCamera>, audio_grid: ResMut<WaterAudioGrid>,
)
```

### 内部逻辑（O，`_scratch/update_water_audio_grid.txt` + `uwag_full.txt` 反汇编）
1. **puffin 门控**：`are_scopes_on()`；未开则 `if *DEMO_MODE!=0 {return}`；开了则 rfind 处理作用域名（`"...stem_water::water_audio_grid::update_water_audio_grid::f"`），`begin_scope()`；尾对称 `end_scope()`。
2. **帧计数**：`c0=*state; *state=c0+1`。
3. **帧位/抖动**：
   - `frame_bit = c0+(c0+1)*10+1`（=`11*c0+11`），低 6 位作滚动窗位索引（`1<<(frame_bit&63)`）。
   - 派生两 16 位增量 `d=(dx,dy)`：`dx=((2*frame_bit)&0xe)-7`，`dy=(((frame_bit>>2)&0xe))-7`（各∈{-7,-5,…,7}；反汇编 `PAND …0x000e000e; PADDW …0xfff9(-7)`）。
   - 派生每帧位置抖动 `jitter=d*0.0625*13.0=d*0.8125`（常量 @0x142eed8e0=0.0625、@0x142eed8f0=13.0）。
4. **相机眼位**：`MainCamera::eye_position()`。
5. **遍历单元**（Vec，**步长 0x18=24B**）：
   - 读单元基址 2×f32(x,z)+`jitter` 得采样点（`utils::swizzlers::x0y()` 重排）。
   - **距离剔除**：`(采样点-相机眼)^2 >= 6400.0`（=80²，@0x142eed900）→非活跃。
   - **水位采样**：`utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>(x,z,scale=130.0)`；`<=0.5`（@0x142925870）→非活跃。
   - **遮挡剔除**：`Raycasting::intersects_any(ray, max=1.0, mask=0x32a3)`；真→非活跃。
   - **活跃**：置位 `mask|=bit`（`+0x08` u64）；0→1 则计数对 `+0x10` 累加 `+d`。
   - **非活跃**：清位 `mask&=~bit`；1→0 则累减 `-d`。

### 数据结构（O，反汇编证实）
`WaterAudioGrid` 单元：`+0x00` f32 x、`+0x04` f32 z、`+0x08` u64 64 位滚动存在窗、`+0x10` u16 counter0、`+0x12` u16 counter1、`+0x14..0x17` 未观察/填充（U）；步长 **0x18=24B**。

### 算法/常量
- 算法：滚动窗位掩码；帧位哈希 `11c+11`；8 级锯齿抖动；相机距离剔除(80)；水位双线性采样(阈值0.5)；射线遮挡剔除。
- 常量：6400.0、0.5、130.0、0.0625、13.0、掩码 0x000e000e、偏置 -7、max=1.0、mask=0x32a3。
- 复用：`utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>`(VA 0x140a14df0)、`utils::swizzlers::x0y`(VA 0x140c9cdc0)、`MainCamera::eye_position`、`Raycasting::intersects_any`、`puffin`。

### 未知
- **计数器对 `+0x10` 语义**：原指令做 `±d`（派生增量对，非恒 ±1）；core 既有叶子 `recovered_systems/water.rs::apply_audio_bit` 建模为 `±1`，**二者不一致**，需裁定 → **U**。
- 网格规模/维度（-7 偏置+8 级锯齿暗示半宽 7，实际由 core 资源决定）→ **U**。
- 单元 `+0x14` 字段、单元基址写入方/是否随相机重建 → **U（core 侧）**。
- `intersects_any` 的 `0x32a3` 语义 → **U**。

---

## 4. `src/record_water_history.rs` — `record_water_history`（无独立符号）

| 项 | 值 |
|---|---|
| 符号 | `system_water::record_water_history::record_water_history`；源码行 12 |
| 记录形式 | `inline_instance, inlinee_declaration; instances=5` → **无独立地址** |
| 内联宿主 | `bevy_ecs::…::run_unsafe<FunctionSystem<fn record_water_history,…>>` **VA `0x140e92a80`**（module 1011, inline_record 5196） |
| PDB 命名实体 | `RecordWaterEditToHistoryCmd`（事件，字段 unknown；procedure 引用 75） |

### 签名（高置信）
```
fn record_water_history(
    res: RasterHistoryRecordResources,               // core SystemParam 打包
    reader: EventReader<RecordWaterEditToHistoryCmd>,
)
```

### 内部逻辑（O，`_scratch/rwh_host.txt`）
1. 用资源 id `in_RCX+8/+0x10/+0x20` 从 World 取参数资源（缺失→Bevy panic）。
2. `EventReader` 游标读事件：对缓冲 `local_50` 字段 `+0x10/+0x18/+0x30/+0x38/+0x40` 做 `saturating_sub`（`EventCursor::read/read_with_id`、`EventIteratorWithId::new`、`iterators::impl$0::last`/`impl$3::last`、`chain::last`）→ 取**最新一条** `RecordWaterEditToHistoryCmd`。
3. 窗口内有未读事件则调用 **`country_core::systems::world_raster::record_raster_history(...)`**（core，VA 0x957990）提交历史事务。
4. 更新 reader 游标（`in_RCX+0x18`、`+0x160` 变更 tick）。

即：**把本帧水面编辑事件 `RecordWaterEditToHistoryCmd` 提交给 core 的 `record_raster_history`**。

### 算法/常量
- 算法：无自研，纯 Bevy `EventReader` 消费（`.read().last()` 语义）+ 转交 core 历史记录。
- 数据结构：`RecordWaterEditToHistoryCmd`（本 crate 事件，字段 **U**）、`RasterHistoryRecordResources`（core）。
- 常量：无（panic 串 `"country_core::resources::history::History"`、`"…ActiveRasterStrokesStorageSystem built with a different world…"`、`"Events<RecordWaterEditToHistoryCmd> Resource requested by … does not exist"`）。

### 未知
- `RecordWaterEditToHistoryCmd` 字段/载荷 → **U**（PDB name-only）。
- `RasterHistoryRecordResources` 内部组成 → **U（模块外）**。
- 取最后一条 vs 逐条 read → **U**（倾向 `.read()` 遍历转交）。

---

## 5. 总览 + 算法清单 + 未知清单

### 逻辑总览
`system-water` 是**水面工具的上层薄壳**：把用户输入翻译成 core 的水面编辑事件，驱动 core 的光栅/音频/历史子系统。
```
输入(AppMode,光标,笔刷,DrawState) ─draw_water─▶ WaterDraw/WaterErase/RecordWaterEditToHistoryCmd/
                                                  RecalculateHeightMapCmd 事件 + draw_raster<Water…> 上传
RecordWaterEditToHistoryCmd ─record_water_history─▶ core record_raster_history
WaterRaster+相机+遮挡 ─update_water_audio_grid─▶ WaterAudioGrid（逐帧滚动窗/计数，驱动水声）
```
水面 CPU 状态（512×512 u8 WorldRaster<512>）、光栅写入、历史事务、音频资源全在 core；本模块只做调度/事件翻译/音频网格标记，不重复实现栅格。

### 算法清单（9 类）
1. 笔刷半径 lerp 映射（`t*4+(1-t)`→`*0.5`）— draw_water，O
2. 光标射线→XZ 平面求交 / 圆刷几何 / 林地边界包含 — draw_water，O（复用）
3. DrawState 4 相位状态机 — draw_water，O 结构/U 语义
4. 闭包事件发射（WaterDraw/WaterErase/Default）— draw_water closure$1/2/3，O
5. 64 帧滚动窗位掩码 + 帧位哈希 `11c+11` — update_water_audio_grid，O
6. 8 级锯齿位置抖动（×0.8125）— update_water_audio_grid，O
7. 相机距离(80)+水位双线性(阈值0.5)+射线遮挡三重剔除 — update_water_audio_grid，O
8. EventReader 取事件→core 历史记录 — record_water_history，O 框架/U 细节
9. Bevy 插件注册 — WaterPlugin::build，O

### 未知清单（7 条）
1. update_water_audio_grid 计数器语义：原指令 ±d vs core 叶子 apply_audio_bit 的 ±1，不一致需裁定。
2. 音频网格规模/维度、单元 `+0x14` 字段、单元基址写入方（core 侧）。
3. `RecordWaterEditToHistoryCmd` 字段/载荷（PDB name-only）。
4. record_water_history：取最后一条 vs 逐条 read；`RasterHistoryRecordResources` 组成。
5. draw_water：DrawState 四相位语义与事件对应；closure$3 的 write_default 事件类型；水笔分支 core 字段归属。
6. WaterPlugin::build：PathRaster::new()/DirtyTilePublisher::unique() 插入方式；add_systems 的 before/after 标签。
7. `intersects_any` 的 `0x32a3` 参数语义。

---

## 6. 最小 Rust 实现草案

仅 `WaterPlugin::build` 内部逻辑完全确定且只用既有 core 类型，给注册草案（**不写入工程**）；其余三函数均需 core 接口（draw_raster/record_raster_history/WaterAudioGrid 布局），按约束**不写实现**。
```rust
impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, record_water_history.in_set(GameSystemSet /*core*/));
        app.add_systems(
            Update,
            update_water_audio_grid
                .run_if(|g: Option<Res<WaterAudioGrid>>| g.is_some())
                .in_set(/*GameSystemSet? 见未知6*/),
        );
        app.insert_resource(DirtyTilePublisher::unique());
        app.insert_resource(PathRaster::new());
        app.insert_resource(WaterRaster::default());
        app.add_event::<RecordWaterEditToHistoryCmd>();
        app.add_event::<WaterDrawEvent>();
        app.add_event::<WaterEraseEvent>();
    }
}
```
（注：`draw_water` **不在**本插件注册——见 §1.2。）

---

### 附：证据落盘（`_scratch/`，非交付）
反编译 `draw_water.txt`/`closure0.txt`/`update_water_audio_grid.txt`/`plugin_build.txt`/`rwh_host.txt`；反汇编 `uwag_full.txt`（369 条）；MCP 结果 `result1..6.json`；关键常量内存 `0x142eed8c0`=0x000e000e、`0x142eed8d0`=-7对、`0x142eed8e0`=0.0625、`0x142eed8f0`=13.0、`0x142eed900`=6400.0、`0x142925870`=0.5、`0x142925904`=1.0。
