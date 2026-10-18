# REPORTS-HOST-WIRING（impl-host-wiring 交付,2026-10-17）

## 库内已接（5 文件 + 1 登记）

1. **startup/run_game.rs**:register_core_startup 补齐域系统——并入墙域 register_wall_cpu_resources+add_wall_cpu_systems(InnerWalls/PublicWalls/DecoratorStorage 权威存储+编辑/发布/空间哈希/洞/砖/删除链);新增 History 资源注册(缺失时插 History::<()>::empty_glade())。
2. **resources/history/mod.rs**:HistoryTransactions::<EditType,S>::empty_glade()(空 entries/cursor=-1/undo_anchor=-1/count_58=0/nonce=0,不猜启动值)。
3. **systems/history.rs**:add_history_cpu_systems Record 槽新增 record_wall_history::<S>(墙录制→History::add)。
4. **systems/save/save_game.rs**:capture_snapshot_system 新增 Option<Res<InnerWalls>>/Option<ResMut<TerrainEditingData>>/Option<Res<StairsState>> 域采集(walls/terrain/stairs 真实填充 SnapshotDomainData;is_backed_up() 守卫防 panic;可选资源不注册不阻断)。
5. **systems/save/snapshot.rs**:init Stairs-Raster 写回接线——apply_stairs_writeback(0x1408ff590 接线点)+apply_raster_writeback(write_named)+decode_raster_payload(flate2 解压,形状待①);apply_loaded_snapshot 新增 rasters:Option<&mut AllWorldRasterWrite>+"Raster" 分支按 6 白名单键解码写回。
6. **systems/render_loop/compute_passes/roof_tiles_gravity.rs**:登记 RoofTileSsbo 存储形态(原 SparseInstanceBuffer<RoofTileSsbo> vs crate ssbo_carrier! TypedStorageBuffer;转换需元素布局+CHUNK_COUNT 均 U,按 WoodenPlankSsbo 模板待 roof 域取证)。

## fresh 反编译关键定案(shared worker)

- **add_game_startup_systems 0x140941110(134B,由 run_game 0x1405949c5 调用)= 5 次 SubApp::add_systems:PreStartup→OnEnter<AppLoadingState>→OnExit<AppLoadingState>→再两次,非 Startup 单调度**(startup/mod.rs 注释已纠偏)。组构建器:G1 0x140a2bf40(10)/G2 0x140839b80(8)/G3 0x1415f4750(1)/G4 0x1409c8600(1)/G5 0x141736760(11)/G6 0x140b6bd50(7),into_configs 模板实参=系统签名已录。
- **StairsState::restore_snapshot 0x1408ff590**:checksum→iter_edges_and_stubs 建边→reduce/apply_diff→重建备份→backed_up=1;crate 版(stairs_state.rs:313)语义对齐。
- **AllWorldRasterWrite::write 0x14083ebe0**:逐槽 None 清空/Some 长度校验拷贝+位图置脏+flag;非 6 项 unwrap panic;crate write/write_named/would_write_succeed 对齐。

## 宿主接线点(已登记,不实现宿主 crate)

全局调度执行序(PreStartup/OnEnter/OnExit);宿主资源供给(MusicBackend/Studio/GraphicsDeviceResource/Asset*Library/GameTime/AppMode);G1-G4 asset 载入(GraphicsDevice 依赖);库外 3 消费系统(add_preplaced_autoclutter/detect_overhang_support_segments/sheep_unstuck);存档线程 AutoSaveState::attach_command_sender+AsyncSaveCmd 分发;load_glade_audio 调用;**SparkleSsbo 写面=上层 critters_particles 粒子域**(sparkle.rs 元素/存储/消费 env36 已实现,写面生产者宿主接线);RoofTileSsbo 缓冲注册/gpu_flush;init_game_state_from_snapshot 全链(Walls/Decorators/Terrain 可接,Stairs/Raster 经新写回入口,Raster 解码形状待①)。

## 验收层面(仍 open)

G1-G4/G5-G6 资产与音频组宿主/render 域;完整快照往返(域序列化器+GameSaveData 20+ 资源组装+init 全链)blocked;run_game_thread cmd0/cmd1 载荷残留。不因编译通过宣布调度完成。

## 编译/测试

cargo check --lib 0 error(13 警告=既有基线);全 crate --lib 测试 **1842/0**。
