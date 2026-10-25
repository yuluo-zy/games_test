# B snapshot apply 独立原审查（2026-10-18）

目标B原RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。fresh Ghidra正文4143B，原RNE全函数反汇编同目录。生产冻结，以下candidate未挂runtime；不能用空字段制造ready。

## 1. 纠正错误ABI

init_game_state_from_snapshot@1417a9c50：RCX是owned LoadedSnapshotData；R8是SnapshotLoadResources SystemParam借用包。R8前0xc0是六个ResMut raster，每个32B，不是快照文件头；随后14个ResMut/Events各32B。+0x10是changed Tick指针，+0x1c是this_run Tick，**不是目标数据指针/数量**。当前SnapshotPrefix::from_bytes、SnapshotDomainDescriptor以及其0x280B合成测试镜像的是错误猜测，不构成原快照格式或游戏恢复。不得喂伪造header绕过。

原SystemParam派生init_state@1417ab290的具名callee与store顺序明确：

| R8 offset | 真实借用对象 |
|---|---|
| 000 | PathRaster |
| 020 | Cobblestone1Raster |
| 040 | Cobblestone2Raster |
| 060 | Cobblestone3Raster |
| 080 | WaterRaster |
| 0a0 | GardenRaster |
| 0c0 | DecoratorStorage |
| 0e0 | DecoratorIdGen |
| 100 | TerrainEditingData |
| 120 | TerrainHeightsData<TerrainHeightSsbo> |
| 140 | Events<CreateRoofCmd> |
| 160 | Events<UploadWorldRasterBuffersToStagingCmd> |
| 180 | Events<SetWallColorIdCmd> |
| 1a0 | Events<SetGateColorIdCmd> |
| 1c0 | Events<SetHalftimberColorIdCmd> |
| 1e0 | Events<EvTerrainStrokeChanged> |
| 200 | Events<InsertTreeCmd> |
| 220 | DeletedAutoClutter |
| 240 | StairsState |
| 260 | StairsRemovedSupports |

InnerWalls是单独传入者（不在该20字段中），实际调用load_state先发生。全部是同World canonical owner；不创建第二套view影子状态。

## 2. LoadedSnapshotData真实owned字段及consumer顺序

以下偏移是原RCX中被本函数消费的字段，不是可移植文件格式，不许直接跨平台解引用原Vec。

| offset | 已证数据/consumer |
|---|---|
| 000..028 | StairsSnapshot，最终调用StairsState::restore_snapshot |
| 028..040 | Walls Vec（120B记录），先InnerWalls::load_state |
| 040..058 | Roofs Vec（88B记录），逐条发送CreateRoofCmd |
| 058..070 | DecoratorSnapshot Vec → restore_snapshot，并写DecoratorIdGen |
| 070..088 | WallColors pairs u64/u32 → SetWallColorIdCmd(custom=false) |
| 088..0a0 | GateColors pairs → SetGateColorIdCmd(custom=false) |
| 0a0..0b8 | HalftimberColors pairs → SetHalftimberColorIdCmd(custom=false) |
| 0b8..0d0 | 六栅格named stream → AllWorldRasterWrite::write |
| 0d0..0e8 | CPU terrain-height Vec；原len检查后memcpy len<<2 |
| 0e8..100 | Trees Vec（64B源记录）→ InsertTreeCmd，72B事件槽 |
| 100..148 | TerrainEditingData owner，先对旧stroke ID发EvTerrainStrokeChanged，再整体替换 |
| 148..178 | DeletedAutoClutter owner替换 |
| 178..1a8 | StairsRemovedSupports owner替换 |
| 1a8..1b0 | 写TerrainHeightsData+50的8B头；意义需跟height类型核对 |

原CreateRoof事件payload映射在candidate snapshot-command-producers.rs，保留原前缀由真实Bevy event id语义适配；其他字段来自真实Roof bytes，未自造屋顶实体。原三色事件完整u64 ID，当前canonical命令把其高32位误列_gap4；候选保留高32位，不清零丢失。

## 3. 当前Rust的实缺口（直接读生产源码）

1. snapshot::parse_wall_section只写id/height，曲线和枚举全清0；InnerWalls::load_state再通过from_record_bytes创建空点列。不能称真实恢复墙。应恢复WallState真实serde与owned Vec load入口，绝不能把Rust heap指针写入字节record。
2. apply_loaded_snapshot对Roofs/WallColors/GateColors/Trees只trace；真实行为应发上表事件，由canonical域消费者创建和发布。HalftimberColors也不能遗漏。
3. InsertTreeCmd现只有id/position/scale/prefab；tree-tool明确省略rotation/variant/is_preplaced，不能承接原snapshot中的64B TreeData。共享事件和消费者必须一起修。
4. TerrainHeightsData当前Vec<u16> API与此函数原memcpy len*4尚不闭合；不能按现有类型猜原字段，不可unsafe复制。
5. DecoratorIdGen没有Resource实现，不能构建原ResMut SystemParam；现snapshot资源结构仅loaded:Option也不是原合同。
6. 原真实数据需Walls/Decorator完整serde；现LoadedSnapshotData以JSON Value保存半数节，不含完整Stairs/height/删除集owner。当前validate函数同名不等于整个closest-snapshot搜索/兼容/领域数据恢复已完成。

## 4. 最小可闭合启动链

直接读原发行资产（非统计）：full/demo intro均只有history.json，没有snapshots目录。Version16；History.chain_index=12，13个compound的edits仅WallNewSystem，包含CreateWall/DeleteWall/AdjustHeight/WallChanged。

因此标准首次启动应走原“closest snapshot不可用 → 从History起点回放13条墙历史”的真实fallback，而非构造空snapshot或强制跳过世界。可复用已有WallAuthoritySnapshot、WallHistoryEdit、apply_instant_wall_edit、InnerWalls/发布算法；仍缺实际旧版WallState与compound serde→canonical typed owner、原HistoryCmd/instant replay调度桥及完整墙发布资源producer。恢复后才能让renderer消费真实世界。

当前candidate只实现已证Roof/三色命令生产片段；它不发LoadingGameComplete，不是全snapshot apply，也没有绕过上面缺口。
