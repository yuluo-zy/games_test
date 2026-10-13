# WP17 — 领域回放系统（replay systems）业务逻辑合同

B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。本文件记录
`crates/country-core/src/systems/history/replay_{wall,terrain,auto_clutter,
stairs,raster,path,water,garden}.rs` 的 **Observed / Inferred / Unknown** 证据。
实现见 `src/systems/history/`。覆盖 11 个回放系统 skeleton 单元
（finish_replay/replay_mode/signifier_replay_active 属编排，见
`replay-orchestration.md`）。

## 通用回放节奏（O：四个领域反编译共用同一骨架）

每个领域回放系统：
1. `ResMut<ReplayBuffer>` 非空（`+0x18 len != 0`）→ 取前桶
   `front_mut_bucket()`；桶内 `Vec<ReplayCmd>`（ptr@+0x8、len@+0x10）。
2. 遍历 ReplayCmd（0x30 元素）；只处理匹配本领域判别的命令。
3. 命令字段（O）：`instant` bit @+0x08；`duration` f32 @+0x0c；
   payload @+0x10；`elapsed` f32 @+0x18；`undo` u8 @+0x1c。
   - **instant**（`(byte@+0x8 & 1)==0`）→ 立即应用领域状态，索引加入
     “完成”列表。
   - **animated** → `progress = elapsed/duration`；`progress < clamp(duration/dt,
     0.05, ..)` 时累加 `elapsed += GameTime.delta`；否则加入“完成”列表。
4. 对“完成”索引排序后从桶 `Vec<ReplayCmd>` 逐条 remove
   （memmove + drop glue 按判别释放 Arc/内联负载）。
5. 桶内命令清空后由 `replay_pop_bucket` 弹出桶并应用装饰/楼梯 diff。

## 1. replay_wall（0x140951b80，2534B）

### O — 直接观察
- 系统签名（enriched 7d099771…）：
  `(ResMut<History>, ResMut<PendingHistoryCommands>, ResMut<ReplayBuffer>,
  EventReader<HistoryCmd>, EventReader<DebugHistoryMsg>,
  EventWriter<PlayReplayAudio>, EventWriter<NewReplayCommandEvent>,
  EventWriter<InterruptTimelapse>, Res<AnimateWallBulldozeBuffer>)`。
- `WallReplayCmd`（判别 4，内联负载）：`new` @源码行158（构造自
  `ConcurrentEdits`，4 宿主点=generate_replay_cmds_from_edit 各实例化）、
  `is_single_adjust_height_cmd` @行152（单 AdjustHeight 判定）。
- `WallReplayEditCmd`（0x18 元素，`Vec<WallReplayEditCmd>` 在 WallReplayCmd
  内）：`process<closure_env$0>` 0x140abfb30（9188B）、`process_instant`
  @行307、`process_animated` @行618、`new` @行170（map 每个
  `Arc<WallHistoryEdit>` → WallReplayEditCmd）。
- `replay_wall`（反编译）：
  - 前桶遍历判别==4（WallReplayCmd）的命令。
  - 每命令内 `Vec<WallReplayEditCmd>`（0x18 元素）逐个
    `WallReplayEditCmd::process<closure_env$0>`（apply 到墙状态）。
  - `bVar25 &= byte@+0x15`（WallReplayEditCmd 的 done 标志）：全部完成时
    向 HashMap<WallId,()> 插已改墙集合（墙状态刷新）。
  - instant/动画累加节奏同上；完成列表移除。
- `WallReplayEditCmd::process<closure_env$0>`（反编译 0x140abfb30）：
  `if done(+0x15) return; if instant(bit0@+0x0)：`cubic_in_out` 缓动后按
  判别跳表（DAT_142adf44c）分派应用；否则标记 done(+0x15=1) 后按
  第二跳表（DAT_142adf408）分派。负载 `Arc<WallHistoryEdit>` 指针 @+0x8。
- `_deprecated_morph_curve` 0x140952570（@源码行875，556B）：已弃用
  形态曲线（`utils::CurveU` 应用；保留函数符号，标注 deprecated）。

### U
- WallReplayEditCmd 判别跳表（DAT_142adf408/DAT_142adf44c）的具体
  领域应用子过程（Create/Delete/Morph/AdjustHeight…）。
- WallReplayCmd 中 `Vec<WallReplayEditCmd>` 之外字段（HashMap 已改集合）。

## 2. replay_terrain（0x140995150，2337B）

### O — 直接观察
- 系统签名（enriched 7e9c5a15…）：
  `(Res<GameTime>, ResMut<TerrainEditingData>, ResMut<ReplayBuffer>,
  EventWriter<EditTerrainStrokeCmd>, EventWriter<EvTerrainStrokeChanged>,
  EventWriter<SetTerrainStrokePtsCmd>)`。
- `TerrainReplayCmd`：`new` @行50、`forward` 0x140994f30、
  `backward` 0x140995040（各 266B）。`ease_out` @行168。
- `forward`（反编译）：`edit` 有曲线点 Vec（+0x10 非空）时
  `duration = clamp(0.4, Σ点距*0.01, 3.0)`（enumerate+fold，
  `duration::closure_env$0`）；否则 `0.3`。写命令
  `{payload@+0x0, +0x4=0, +0x8=2, duration@+0xc, +0x14=0}`。
  `backward` 同构（判别/方向位不同）。
- `replay_terrain`（反编译）：
  - 前桶遍历判别==3（Terrain）。`lVar12 = payload Arc(+0x8)` 解引用：
    `+0x10==i64::MIN` → 空 TerrainEdit（用 `+0x18` 处状态）。
  - 新命令启动时把 TerrainEdit 的 stroke 状态**重新写入**
    `TerrainEditingData`（`HashMap<TerrainStrokeId, TerrainStroke>` insert，
    从 `+0x20/+0x28` 拷贝 stroke；`+0x38` 容量增长）。
  - 动画：`elapsed(+0x10)/duration(+0x14)`，`1 - t^3`（`ease_out`）；
    `undo(+0x1c)==0` → `TerrainEdit::reapply()`；`!=0` →
    `TerrainEdit::revert()`。完成时移除。

### U
- TerrainEdit 内 stroke 副本的精确字段；`+0x08==2` 语义（判别/方向位）。

## 3. replay_auto_clutter（0x140aa4a60，1651B）

### O — 直接观察
- 系统签名（enriched 0103bc1d…）：
  `(ResMut<ReplayBuffer>, Res<GameTime>, ResMut<DeletedAutoClutter>)`。
- 逻辑（反编译）：
  - 前桶遍历判别==9（`'\t'`）的命令。
  - 新命令（`elapsed(+0x18)==0`）：
    `undo(+0x1c)==0` → `HashMap<WallId,()>` 插墙（恢复自动杂物宿主）；
    `!=0` → 从 `DeletedAutoClutter` 的 `HashSet/HashMap` erase
    （`RawTable::erase<(DecoratorId,DecoratorId)>`）与 WallId 哈希表。
  - instant/动画节奏同上；`progress = duration/dt`，下限 0.05。

## 4. replay_stairs（0x140af75b0，2640B）

### O — 直接观察
- 系统签名（enriched f762bb08…）：
  `(ResMut<ReplayBuffer>, Res<GameTime>, ResMut<StairsRemovedSupports>,
  EventWriter<PlayStairsAudioCmd>, Res<CachedDecoratorTransforms>,
  Res<DecoratorStorage>)`。
- 逻辑（反编译）：
  - 前桶遍历判别==10（`'\n'`）的命令。`payload(+0x10)` → StairsEdit
    （`+0x10` 处 StairGraphEdge 信息；`iVar7 = *(int*)(lVar24+4)` 边键）。
  - `undo(+0x1c)==0` → `HashMap<StairGraphEdge,()>` insert（恢复边）；
    `!=0` → `RawTable::remove_entry<StairGraphEdge>`（删除边）。
  - `DecoratorStorage::lookup_decorator_info()` 查附挂装饰；
    `CachedDecoratorTransforms` 哈希检索（SIMD 16 字节散列匹配）取边的
    装饰变换；发 `PlayStairsAudioCmd` 事件（`grow_one<EventInstance<...>>`）。
  - instant/动画节奏同上。

## 5. replay_raster（泛型 系统）+ replay_path / replay_water / replay_garden

### O — 直接观察（PDB 骨架 + enriched 接口 + 反编译 0x1409febf0）
- 泛型系统 `replay_raster<T, R, F>`（T=StagingSsbo，R=WorldRaster<512>，
  F=命令过滤闭包）。六个实例化（PDB 编译实例）：
  | 领域 | SSBO | 光栅 | 过滤闭包 | RVA |
  |---|---|---|---|---|
  | Path | PathRasterStagingSsbo | PathRaster | closure_env$1<DirtPathFilter> | 0x1409febf0 |
  | Cobblestone1 | Cobblestone1RasterStagingSsbo | Cobblestone1Raster | closure_env$1<Cobblestone1PathFilter> | 0x1409fc290 |
  | Cobblestone2 | Cobblestone2RasterStagingSsbo | Cobblestone2Raster | closure_env$1<Cobblestone2PathFilter> | 0x1409f1cf0 |
  | Cobblestone3 | Cobblestone3RasterStagingSsbo | Cobblestone3Raster | closure_env$1<Cobblestone3PathFilter> | 0x1409f9930 |
  | Garden | GardenRasterStagingSsbo | GardenRaster | closure_env$2 | 0x1409f6ff0 |
  | Water | WaterRasterStagingSsbo | WaterRaster | closure_env$2 | 0x1409f4650 |
- 系统签名：
  - `replay_path`：`(RasterReplayResources, ResMut<PathRaster/Cobblestone*Raster>,
    Commands)`（4 个：Path/C1/C2/C3，@0xae1620/0xae2130/0xae1a40/0xae1830
    宿主）。`generate_path_replay_cmds` 0x14016286d0（PDB 编译实例）。
  - `replay_water`：`(RasterReplayResources, EventWriter<WaterDrawEvent>,
    ResMut<WaterRaster>, EventWriter<RecalculateHeightMapCmd>,
    Res<ScheduledTerrainReadback>)`（@0xae1c50 宿主）。
  - `replay_garden`：`(RasterReplayResources, EventWriter<GardenDrawEvent>,
    ResMut<GardenRaster>)`（@0x14c7090 宿主）。
- `RasterReplayResources`（replay_raster 内 SystemParam bundle）：
  `get_param` 0x140ae5830/0x1401507610、`init_state`、`validate_param`、
  `impl$8::default`。含 ReplayBuffer + GameTime + StagingSsbo 访问。
- `RasterReplayCmd`：`forward` 0x1401700430、`backward` 0x1401700620、
  `forward_instant`/`backward_instant`（74 宿主点）、`new`（149 宿主点）。
  负载 = `Arc<RasterEdit>`（path_edit.rs）。
- `replay_raster<Path>`（反编译）核心：
  - 过滤闭包 F 把 ReplayCmd → `Option<&RasterEdit>`（非 Path 命令返回 None
    跳过）。`started(+0x19)` 未启动时：
    - `undo(payload+0x3)` 非零 → `clear_buffer<512>()` + 遍历 staging SSBO
      （`local_a0[1]/[2]` 记录的各 stroke）`rasterize_stroke<512>` 重绘
      “before” 状态。
    - 否则 `rasterize_stroke<512>` 画编辑 stroke；`redo` 标志(+0x1a)==1 时
      `Commands::send_event<PathDrawEvent>`。
  - `DirtyTiles::rasterize_world_space_stroke<...>`（closure_env$2 映射
    `raster_stroke::impl$3::iter_points_xz`）把点写入 dirty tiles。
  - instant/动画节奏同上。
- `replay_path::PathFilter` 系列：`DirtPathFilter`/`Cobblestone1..3PathFilter`
  （`replay_path::closure_env$1<T>` 的命令过滤类型）。
- 领域 edit → ReplayCmd 生成闭包（`generate_replay_cmds_from_edit` 宿主）：
  `replay_garden::closure$1::closure_env$0` @0x143d2c0、
  `replay_path::generate_path_replay_cmds::closure_env$0` @0x14423e0、
  `replay_water::closure$1::closure_env$0` @0x1444c70；
  `replay_path::generate_path_replay_cmds::closure$0` @0x14475a0。

### U
- RasterEdit 内 stroke 记录布局（points/brush_size/is_additive/blend_mode/
  原始曲线的精确字段，观察 schema 见 edit-history-types 合同）。
- `replay_raster` 泛型的 staging SSBO 同步与 HeightMap 重建时序。
- RasterReplayResources 的精确 SystemParam 字段。

## 跨系统记录
- 复用 `replay_pop_bucket` 的装饰/楼梯 diff 应用（桶空时）。
- 复用 `reconstruction/{terrain,wall,path,...}` 领域 apply 函数边界；
  本批不重推导（调用链证据标注在各系统）。
