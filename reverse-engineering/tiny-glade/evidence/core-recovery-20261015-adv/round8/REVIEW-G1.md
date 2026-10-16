# 第八轮 G1 环境域 ③ REVIEW（2026-10-16）

审查者：③ review agent。审查对象：G1 环境域（camera_rig 深层 / weather·time_of_day 深层 / 水系统 / terrain 剩余）。
基准：`D:/game/tiny-glade/crates/country-core/src/` 当前树 + 未提交 diff（② 正在写）+ 证据
`evidence/country-core-deep/{camera-time,terrain-water}/` DIGEST/ALGORITHMS/decomp 逐条核对。
准则：字段/常量可溯源原地址、真实实现非桩、常量逐字节核对。

**审查状态**：本文落盘时（2026-10-16 18:30）`round8/contracts/G1-*` 与 `round8/deliveries/G1-*` **均未落盘**
（round8/ 只有 G2/G4 的 decomp/out，PROGRESS.md 记 G1"待①契约"）。② 在 `camera_rig/main_camera.rs`、
`systems/terrain/*`、`resources/water.rs`（新文件）有未提交改动。本 REVIEW 基于当前树 + 进行中 diff；
最终 DELIVERY 落盘后需按本表复核一遍。禁区（rhapsody/roof/wall）未审，仅登记交叉引用。

---

## 1. 缺口核对表（现状 = 缺/水分/已实现/待证）

### 1.1 camera_rig（`systems/camera_rig/*`）

| 缺口/项 | 源码位置 | 现状 | 证据（原地址） | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| `MainCamera::new` 六 body 默认值 | `main_camera.rs` `reset()` | **缺**（853 行反编译未解；`reset()` 仅清缓冲+Normal fov=45） | 0x14090ec70 | 出六 body 默认值契约（Normal/TiltShift/Free/Walk/TopDown/Butterfly 的 pos/yaw/pitch/fov/radial/pitch_min/max 等；含 terrain_clearance 默认） | ① 契约到位后替换 `reset()` 的局部重建 |
| `main_camera_update` 13 模式跳表 | `main_camera.rs` 无 | **缺**（整系统未实现） | 0x140912c20；跳表 0x142aabd04 / 0x142aabd30 | 按模式分派 + ActionSetStates(+0x10) 键位面 + DebugGui 滑块（+0x11c/0x11d、6→0） | ① 契约到位后补主相机系统 |
| `paused_main_camera_update` case 6..11 | 无 | **缺**（case 0..5 body 指针/偏移已 O，case 7..10 fov 对已录，6/11 反编译损坏） | 0x1409145a0 | 重解损坏 case 6/11；补全 (fov_target,fov_current) 对 | ① 契约到位后补 |
| `immediately_finish_interpolating` | 无 | **缺**（2466 行反编译；模式+快照字节 0x134 双 jumptable、fov 系数 +0xec×0.1、winter==4 分支） | 0x140911b50 | 双 jumptable 分派契约 + 各分支调用面 | ① 契约到位后补 |
| `NormalCamera::update` | `normal_body.rs` | **缺** | 0x140949fe0 | smooth/radial_max@+0x60 clamp/loose@+0x6c 投影/pitch·yaw 增量/retain 阈值全量 | ① 契约到位后补 |
| `NormalCamera::translate_screen_space` | `normal_body.rs` | **缺**（旋转四元数来源未证） | 0x140949e70 | 旋转四元数构造来源（`ratio=max(y/−fwd.y,1)/50` 绕 yaw） | ① 契约到位后补 |
| `NormalCamera::load_parameters` | `normal_body.rs` | **缺** | — | ActiveRotation 队列/π/180 轨道四元数/rotations Vec@+0x100 布局 | ① 契约到位后补 |
| **`rotate_yaw_pitch`（free/walk 共享）** | `free_camera.rs` 无 | **缺**（**与 normal_body 版是不同函数**：yaw=fmod(yaw+Δ·0.5,720)、pitch=clamp(±90)、增量折半 ×0.5@0x142925870） | 0x140993750 | 出 free/walk 版 rotate 契约（dolly driver 查找 + 720/±90 常量） | ① 契约到位后补 `FreeCamera::rotate_yaw_pitch` |
| `FreeCamera::translate_screen_space` | `free_camera.rs` | **缺**（依赖地形采样 0x1409e9640，未解） | 0x1409938b0 | 地形采样函数身份 + clamp ±91/0.75/20/60/exp(−8dt) 常量 | ① 契约到位后补 |
| `CameraModeStates` per-mode (body,fov) 对 | `mod.rs` | **水分**（只有 mode 字节 + 六 body 偏移；case 6 body@+0 是猜测） | 0x1409149d0 jumptable | case 6..12 完整 (fov_current,fov_target) 对 + 模式 6 body 区真相 | ② ① 契约后改 `current_dyn` |
| `CameraState`（快照字节@+0x134 / fov 系数@+0xec） | `mod.rs` | **缺**（I 标注，无读写器） | 0x140911e40 / +0x134 | 快照更新分发语义（is_interpolating 实为按模式分发） | ① 契约到位后补 |
| `CameraRotationPivot` 7 变体枚举 | 无 | **缺** | ron 0x14003ac90 变体名 | 7 变体名 + 消费点（哪种模式用哪种 pivot） | ① 契约到位后补枚举 |
| `constrain_to_be_above_terrain` 常量 0.75 | `normal_body.rs` 用 `terrain_clearance` 字段 | **待证** | camera-time DIGEST §3 `max(terrain+0.75, y+pending.y)` | 0.75 是硬常量还是 terrain_clearance@+0x68 的默认值（需 MainCamera::new 默认值） | ② 测试默认值 2.0 是编造的，待①定案 |
| `ActiveRotation` +0x00/+0x10/+0x14/+0x18 语义 | `normal_body.rs` | **缺**（创建写 0/-0.0/-0.0/-1.0，语义 U） | chase_butterfly 0x140911520 | 四字段语义 | ② 保留 U 标注 |
| `ZoomClampPanic`（radial_max<min 或 NaN） | `normal_body.rs` | **已实现**（O：0x140d8b480 断言） | 0x140911280 汇编手解 | — | 保留 |

### 1.2 weather（`systems/weather_system.rs`）

| 缺口/项 | 源码位置 | 现状 | 证据 | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| `weather_system` ECS 参数面 | `weather_system.rs` 纯函数（snow/&mut、game_time、delta、glade_type 四参） | **缺**（无 ECS 系统接线） | decomp-0x1408e5b00：`param_1`=Weather 资源（强度@+0xc）、`param_2`=GameTime、`param_3`=GladeSettings（`*param_3+0x58`==4） | 定 Weather/GladeSettings 资源身份 | ② 转 ECS 系统 `Res<GladeSettings> + ResMut<Weather> + Res<GameTime>` |
| `SnowIntensity` 布局（只 4B intensity） | `weather_system.rs` | **水分**（资源实际 ≥0x10B：decomp 尾 `*(u32*)*param_1[2] = tick` 写资源+0；强度在 +0xc） | decomp-0x1408e5b00 L30 `*(float*)(*param_1+0xc)`；tick 写回 | Weather 全布局（tick@0 + 前 0xc 未知字段身份） | ② 扩成 ≥0x10B 资源或改名 Weather，注明 tick@0 |
| 算法本身（sin/300·π/±0.1·dt clamp） | `weather_system.rs` | **已实现**（O 逐条） | 常量 1e9@0x1429a7640、300@0x142aa5140、π@0x1429edba4、0.1@0x142925984、1.0@0x142925904 | — | 保留 |

### 1.3 time_of_day（`systems/time_of_day.rs` + `resources/time_of_day.rs`）

| 缺口/项 | 源码位置 | 现状 | 证据 | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| `TIME_OF_DAY_SPEED` 全局 | `time_of_day.rs` 用 1.0 默认 | **缺**（全局 f32 值待读） | decomp-0x140a73460 L310 `fVar48 = *(float*)(*plStack_100+0x28) * fRam000000014362c0b8` | `rne-read 0x14362c0b8` 读 f32 值 | ② 替换 1.0 |
| `suncalc::get_position` 集成 | 无 | **缺**（只算 `day_of_year|0xfce00` 后丢弃） | decomp L330 `FUN_1417ab4b0(&fStack_168, lVar20+lVar30*1000, lat, 0)`；3600000@0x142ad58f0 | lat/lon 常量（0x142ad5910..5984 与 basis 的关联）+ 日期位打包 `uVar13|0xfce00` 全解 | ① 契约到位后接 |
| 光照写盘（TodLightingSnapshot 插值 + SunSkyColors 两套） | `TimeOfDayLightingSink{updated}` 空适配 | **水分/缺**（param_6=SunSkyColors 0x140b18830、param_8=光照输出 ~0x78B、param_3 尾部 f32 全未接） | decomp L410/L455 `FUN_140b18830(*param_6,…)`；0x140b067d0 snapshot_at_tod | param_6/param_8/param_3 资源面 + 载荷布局 | ② ① 契约后换掉空适配 |
| 音频上报（TOD-Enabled/Timetravel/DayNight） | 无 | **缺** | decomp L70/L89 `FUN_140a53db0(...,0xb/10)`；串 0x142ad63ef/fa/404 | 已 O 证实；三个 set_parameter 面 | ① 契约到位后接 |
| **dev 热键字节 @+0xe3** | `time_of_day.rs` `dev_on = dev_speed_mult != 0` | **水分**（dev 判定用 dev_speed_mult 代理；原版是独立热键字节轮询切换） | decomp L265 `FUN_14089aef0(...)` → `*pbVar1 ^= 1`；`+0xe3` | 已 O 证实 | ② 加 `dev_hotkey: bool` 独立字段，去掉不等于推断 |
| **当前态布局注释 +0x3 纠偏** | `time_of_day.rs` 注释"角度过渡@+3" | **缺（注释错）**：decomp `*(char*)(pfVar21+3)` 是 **float 指针 +3 = 字节 +0xc**（pfVar21[1]=angle@4、+0xa=hour 过渡、+0xb=instant） | decomp-0x140a73460 L92/L108 `*(char *)(pfVar21 + 3)`、`(char*)(pfVar21+0xb)` | 定 +0x8 未知字段（pfVar21[2]）身份 | ② 改注释（angle 过渡@+0xc 非 +3）；字段序对齐 +0xa/+0xb/+0xc |
| hour 推进公式精确化 | `advance_tod_hour` | **待证**（源码 `progress·24+8+dt·speed` 与 decomp 结构 `…·fVar52·(dt·TOD_SPEED)·dev_mult + target_hour + 1.0` 有出入；DIGEST 记录 `progress·24+8+dt·TOD_SPEED·speed·trig`） | decomp L310-325 | 逐指令核对 hour 推进公式（含 trig 项身份） | ① 定案后对齐 |
| `suncalc_pos_to_vec` 矩阵系数 | `basis` 模块 | **待证**（源码矩阵未含 `_UNK_142a81460` 打包常量；decomp 中该常量经 fVar56/fVar58 进入部分项，若干项被 ×0.0 消去） | decomp L370-416；`_UNK_142a81460` | 复核 3×3 矩阵（A..P）与 `_UNK_142a81460` 的贡献 | ① 定案后对齐 |
| `remap_hour_to_fmod_hour` | `resources/time_of_day.rs` | **已实现**（O：x/y 关键帧、冬季 4.52/5.0/6.2/9.0 插入、8 项长度） | 0x140a74790；0x142ad5990/59b0；decomp L324 `FUN_140a74790(uVar42, winter)` | — | 保留 |
| 插值/播放状态机/小时推进核心 | `time_of_day.rs` 纯算法 | **已实现**（O：2π−angle、±π wrap、exp(−3dt)、±12h wrap、0/1/2 状态机） | 2π@0x142934674、−3@0x142ad58e8、dt/3、dt/−5 | — | 保留 |
| `TimeOfDayState` 合并单资源 vs 原三资源 | `time_of_day.rs` | **水分**（原版 param_7 播放态 / param_9 dev 态 / param_12 当前态三个资源；合并建模改变 ECS 面） | decomp 参数面 | ① 定三资源布局后按面拆分或保留合并+登记 | ② ① 契约后对齐 |

### 1.4 水系统（`resources/water*.rs`、`water_mask.rs`）

| 缺口/项 | 源码位置 | 现状 | 证据 | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| `RippleSourceSsbo` hash=0 | `water.rs` | **缺**（typed gap：`rg_buffer_hash()=0` 占位，GPU 绑定会断） | 名 hash 族常量 0x142c2e210 区 | 读 `utils::name_hash("…RippleSourceSsbo")` 实值 | ② ① 定值后替换 |
| `WaterMaskSsbo` hash=0 / 布局 | `water_mask.rs` | **缺**（同上） | 0x142c2e210 族；DIGEST §5.3.7 | hash + 字段布局 | ② ① 定值后替换 |
| `WaterExcludeClutterMaskSsbo` hash=0 | `water_exclude_clutter_mask.rs` | **缺**（同上） | — | 同上 | ② ① 定值后替换 |
| `WaterDrawEvent`/`WaterEraseEvent` 生产者 | `water.rs` 事件已定义，无生产者 | **缺**（`system_water::draw_water` 0x141ba9130 跨 crate，未实现；`WaterPlugin::build` 0x1406501d0 未接） | 0x141ba9130 / 0x1406501d0 | draw_water 参数面 + on_draw/on_erase 回调载荷 | ① 契约到位后接生产者 |
| `WaterRaster` CPU 状态 | `resources/world_raster.rs` WorldRaster（G4 域） | **已实现**（composite/scratch 双缓冲 + 脏协议；DIGEST 布局全 O） | 0x140a15530 等 | —（G4 域） | — |
| 水 mask 常量（512/0.5/0.2/1/130/65.0/2.0） | `water_mask.rs` | **已实现**（O：shader 源） | `read_water_mask.cs.hlsl`、`draw_water_mask.cs.hlsl` | — | 保留 |
| water_spawn_vegetation（lilypads/reeds） | 无 | **缺**（GPU，后置） | 0xa13970 | GPU 后置，登记 | 不实现 |

### 1.5 rocky_terrain（`resources/rocky_terrain.rs`）

| 缺口/项 | 源码位置 | 现状 | 证据 | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| `RockyTerrainJson` 真实 schema | `rocky_terrain.rs`（`position[3]+extra[6]` 标 I） | **缺**（JSON 字段名/顶点布局未证） | 0x8bbf50 | 找 rocky terrain 资产 JSON（vertex 源 0x24B：position/normal/uv?）+ 字段名 | ① 定案后改 Loaded 结构 |
| `packed_vertex` 0x24B→0x30B | `rocky_terrain.rs`（前 12 floats 透传） | **缺**（源布局未证） | 0x8bbf50 | 顶点 0x24B 源布局（位置/法线/UV 分块） | ① 定案后改打包 |
| `packed_triangle` 3 索引 + pad | `rocky_terrain.rs` | **待证**（0x10B→0x20B，3 u32 + pad） | 0x8bbf50 | 三角形 0x10B 布局（3 索引 + 1 pad？） | ① 定案后对齐 |

### 1.6 terrain 剩余（`systems/terrain/*`、`terrain_rock_mask.rs`、`glade_border.rs`、`mouse_terrain_raycast.rs`、`cursor_terrain_raycast.rs`）

| 缺口/项 | 源码位置 | 现状 | 证据 | 深挖方向（①） | 修复建议（②） |
|---|---|---|---|---|---|
| **`terrain_rock_mask::sample_at_world_xz` 位索引** | `terrain_rock_mask.rs` | **错误**：源码 `mask.get(zi*512+xi)` 按每格一个 u32 读，越界恒 0；原版是 **位字段** `(z*512|x)>>5` 字索引 + 取位 | 0x1409ea800；terrain-water ALGORITHMS §3.5 | 已 O 定案 | **② 立即改位提取**：`bit=zi*512+xi; (mask[bit>>5]>>(bit&31))&1` |
| `terrain_replay_from_edit` | `replay_terrain.rs` | **缺（桩）**：恒返回 `None` | — | TerrainEdit 0x38B 点集载荷（falloff/height/position/id 序） | **② 诚实标注待接，或 ① 定案后接；不要以返回 None 冒充完成** |
| `revert` 全移除 vs 动画局部消除 | `replay_terrain.rs` `strokes.remove` | **水分**（整条移除；原版 revert 0x140858d30 按 CurveU 进度局部消除） | 0x140858d30 | 共享 terrain_edit owner 的动画消除语义 | ① owner 恢复后对齐 |
| `ui_draw_terrain` state 3（读笔）不可达 | `ui_draw_terrain.rs` | **缺**（`TerrainHoverInput` 无 release 事件，state 3 永远不可达；state2 续笔已实现） | 0x140a9fdc0 decomp L134-165 | release/读笔输入面 | **② 补 release 事件模型**，使 state 3 可达 |
| `MORPH_SPACING` 常量 | `ui_draw_terrain.rs` 1.0 标 I | **缺** | 0x140a97880 内 `movss xmm2,[rip+…]` | 读该常量 | ① 定案后替换 |
| `DrawState` 8B 元素语义 | `ui_draw_terrain.rs` `records.push(0)` | **缺**（元素 {qword,count} 语义 U） | decomp L111-120 | 8B 元素两字段语义 | ① 定案后对齐 |
| `ui_edit_terrain` 16 位脏掩码其余 13 位 | `ui_edit_terrain.rs`（位 0/1/2 已映射） | **缺** | 0x140aef1f0 decomp L270-285 | 其余 13 位参数位语义 | ① 定案后补 |
| `StairsPlacementEntry` 0x28B | `ui_edit_terrain.rs` | **缺**（I：tag@0/float@+4/align@+8/双 DecoratorId@+0xc/+0x1c/+0x20） | decomp case 2 | 0x28B 记录逐字段 | ① 定案后对齐 |
| `EditCommand` ABI 字节序 | `ui_edit_terrain.rs` 走 tg_terrain `{id,falloff,height,position}` | **水分**（原版 `{falloff,height,position,id}` id@+0x1c，RE-EDITCMD E7 定案；已登记差异） | RE-EDITCMD E7 | 已定案 | ② 保持登记；宿主序列化时按原序 |
| `TerrainEdit` 0x38B 记录字段 | `deferred_terrain_history.rs` `{pending[4], previous_state}` | **缺**（字段语义 U） | 0x140ac9770 | 0x38B 记录（tag 对 + pending 4 qword + 旧 state）逐字段 | ① 定案后对齐 |
| `MouseTerrainRaycastSsbo` 布局 | `terrain_hover_idx.rs` 适配资源 | **缺**（id@+0x10、距离@+8、悬停位已 O，但 SSBO 真身未解） | 0x140aff720 decomp；0x1408ef320 | SSBO 全布局 + `[param_7+0x58]` 除数身份 | ① 定案后删适配层接真 SSBO |
| `is_shape_inside` body | `glade_border.rs` | **缺**（4 组合 match 是 I；`as_points2(1.0)` 圆采样步骤待证） | 0x140ad2970（is_pos_inside 0xad2950 跳板） | 圆/矩形 4 组合精确语义（矩形边框 step 1.0 采样） | ① 定案后对齐 |
| `mouse_terrain_raycast` 系统 | `mouse_terrain_raycast.rs`（只有 intersect_with_xz_plane） | **缺**（CameraCursorRay 有效→xz 平面；否则 intersect_ray 高度场与 y=0 比距；写 tag/pos/delta/speed） | 0x1408ef320 | 完整系统 + param_7 距离场除数 | ① 契约到位后补 |
| `CursorTerrainRaycast::update` | `cursor_terrain_raycast.rs` | **水分**（`update(_hit: Option<Vec3>)` 是桩，输入语义 U；真实 update 由 mouse_terrain_raycast 维护） | — | ① 契约到位后对齐 | ② 诚实标注或删桩 |
| `ui_terrain_focus_mode` 内联 switch 表 | `ui_terrain_focus_mode.rs` | **缺**（167KB 内联体转移 switch 表 U；只落门+常量表+触发事件） | 0x1409a4fb0；4 闭包 0x9a3620/8640/88c0/8b40 | 内联体 switch 表语义 | ① 深挖后补 |

---

## 2. 功能缺失（消费者链断 / 无生产 / 身份未定）

- **F1** time_of_day **太阳→光照→音频链全断**：`suncalc::get_position` 未接、`TodLightingSnapshot`/`SunSkyColors` 写盘未接、`TOD-Enabled/Timetravel/DayNight` 音频上报未接。`TimeOfDayLightingSink{updated:bool}` 是空适配，无真实业务。
- **F2** weather_system **无 ECS 系统/资源接线**：`SnowIntensity` 纯函数四参未映射到真实 `Res<Weather>+Res<GladeSettings>+Res<GameTime>`；强度字段在资源 +0xc 而结构只有 4B。
- **F3** `WaterDrawEvent`/`WaterEraseEvent` **无生产者**：`system_water::draw_water` 0x141ba9130 跨 crate 未实现；water 编辑事件环断。
- **F4** `RippleSourceSsbo`/`WaterMaskSsbo`/`WaterExcludeClutterMaskSsbo` **`rg_buffer_hash()=0` 占位**：GPU 绑定（`AssetSsboLibrary::get`）会取到错误 hash。
- **F5** `terrain_hover_idx` 用**适配资源**（`MouseTerrainHoverReadback`/`TerrainWaterSample`）而非真 `MouseTerrainRaycastSsbo` 回读——悬停/水位/距离三路信号无真实生产者。
- **F6** `replay_terrain::terrain_replay_from_edit` **恒 None**：历史→回放链断；`TerrainEdit` 载荷无点集。
- **F7** `ui_draw_terrain` **state 3（读笔）不可达**：`TerrainHoverInput` 无 release/读笔事件。
- **F8** camera_rig **主相机系统链全缺**：`main_camera_update`/`paused_main_camera_update`/`immediately_finish_interpolating`/`MainCamera::new`/`NormalCamera::update` 均未实现；`MainCameraMoved` 事件（roof ui_edit_roof 0x141a18dd0 消费）已补定义但无生产者。
- **F9** `CursorTerrainRaycast::update` 桩：无真实光标变换生产者。
- **F10** `terrain_rock_mask` 位索引错误导致岩石采样恒 0（见 1.6 首行）——最直接影响。

---

## 3. 深挖方向清单（给 ①，按优先级）

1. **terrain_rock_mask 位索引**（已 O 定案，无需深挖）→ ② 立即修。
2. **camera_rig 主链契约**（最高优先，缺整系统）：
   - `MainCamera::new` 0x14090ec70 六 body 默认值（含 terrain_clearance 默认，解 1.1 待证项）
   - `main_camera_update` 0x140912c20 13 模式分派 + ActionSetStates 键位 + 跳表 0x142aabd04/0x142aabd30
   - `paused_main_camera_update` 0x1409145a0 case 6/11 损坏重解 + case 7..10 fov 对补全
   - `immediately_finish_interpolating` 0x140911b50 双 jumptable 分派
   - `NormalCamera::update` 0x140949fe0 / `translate_screen_space` 0x140949e70 / `load_parameters`
   - **free/walk `rotate_yaw_pitch` 0x140993750**（与 normal 版不同函数）+ `FreeCamera::translate_screen_space` 0x1409938b0（地形采样 0x1409e9640）
   - `CameraModeStates` case 6..12 (fov_current,fov_target) 对；`CameraRotationPivot` 7 变体名
3. **time_of_day 接缝**：`TIME_OF_DAY_SPEED`（rne-read 0x14362c0b8 f32）；suncalc 输入 lat/lon 常量（0x142ad5910..5984 与 basis 关联）+ 日期位打包；光照写盘 param_6/param_8/param_3 三资源面；音频三 set_parameter；hour 推进公式逐指令核对（decomp L310-325）；`suncalc_pos_to_vec` 矩阵补 `_UNK_142a81460` 贡献。
4. **water 身份**：`RippleSourceSsbo`/`WaterMaskSsbo`/`WaterExcludeClutterMaskSsbo` 的 name_hash（0x142c2e210 族常量表）；`system_water::draw_water` 0x141ba9130 参数面 + `WaterPlugin::build` 0x1406501d0 注册面。
5. **terrain 剩余**：`is_shape_inside` 0x140ad2970 4 组合精确语义；`mouse_terrain_raycast` 0x1408ef320 完整系统 + `[param_7+0x58]` 除数；`MouseTerrainRaycastSsbo` 全布局；`ui_draw_terrain` release 输入面 + `MORPH_SPACING` 常量（0x140a97880）；`ui_edit_terrain` 16 位脏掩码其余 13 位 + `StairsPlacementEntry` 0x28B 逐字段；`TerrainEdit` 0x38B 记录字段；共享 `revert` 0x140858d30 动画局部消除语义；`ui_terrain_focus_mode` 167KB 内联 switch 表。
6. **weather 资源布局**：Weather/SnowIntensity 全布局（tick@0 + 前 0xc 未知字段身份）。
7. **rocky_terrain JSON schema**：资产 JSON（顶点 0x24B 源布局 + 字段名）；三角形 0x10B 布局。

---

## 4. 给 ② 的修复项（无需新契约，立即可改）

1. **`terrain_rock_mask::sample_at_world_xz` 位提取**（O 定案）：`bit = zi*512+xi; (mask[bit>>5] >> (bit&31)) & 1`，替换 `mask.get(zi*512+xi)`。
2. **time_of_day 当前态注释纠偏**：angle 过渡标志在 **+0xc**（非注释写的 +3；decomp `*(char*)(pfVar21+3)` 是 float* 指针 +3 = 字节 +0xc）；+0xa=hour 过渡、+0xb=instant 已对。
3. **dev 热键独立化**：`dev_on` 改独立 `dev_hotkey` 字节（O：`+0xe3` 轮询 `^=1`），不要用 `dev_speed_mult != 0` 代理。
4. **`terrain_replay_from_edit`** 不返回 None 冒充完成：诚实登记待接，或 ① 定 `TerrainEdit` 点集载荷后接。
5. **`ui_draw_terrain` release 事件模型**：`TerrainHoverInput` 补 release/读笔事件，使 state 3 可达（state 3 = get_last_mut_stroke + 点集遍历，round4 契约已定案）。
6. **`weather_system` ECS 化**：`Res<GladeSettings> + ResMut<Weather> + Res<GameTime>`，`SnowIntensity` 扩到 ≥0x10B（tick@0）。
7. **`SnowIntensity`/`TimeOfDayLightingSink` 诚实标注**：两者都是部分模型，DELIVERY 里登记"待 ① 契约"，不要列"已实现"。
8. **`CursorTerrainRaycast::update` 桩标注**：删桩或标注 I 待接，勿当已完成。
9. **`MainCameraMoved` 事件**：注册进事件列表并注明"无生产者（主相机系统未实现）"，roof 消费面登记。
10. **free/walk `rotate_yaw_pitch`**：补 `FreeCamera::rotate_yaw_pitch`（0x140993750，yaw mod 720 / pitch ±90 / 增量 ×0.5），与 normal 版区分。

---

## 5. ② 进行中 diff 复审快评（2026-10-16 当前未提交）

- `main_camera.rs` 补 `MainCameraMoved` 事件：方向对（roof ui_edit_roof 0x141a18dd0 消费），但无生产者，见 §4.9。
- `terrain_hover_idx.rs` 两分支门控重写：**与 decomp 一致**（高度≤0 且 id 有效要写；高度>0 走悬停位+距离 0.5 门），O 证据引用完整，通过。
- `terrain/mod.rs` 挂 `replay_terrain` + `StairsPlacementBucket`：合理；但 `replay_terrain` 消费 `ReplayBuffer` 前桶 `ReplayCommand::Terrain` 需与共享 `replay_cmd_buffer.rs` 仲裁一致。
- `ui_draw_terrain.rs`/`ui_edit_terrain.rs`：见 §1.6 各行；核心门/常量 O 引用正确。

---

## 6. 结论（给父）

- **G1 域当前交付度**：纯算法层（normal_body zoom/rotate/constrain、time_of_day 插值/播放机/remap、weather 振荡公式、terrain 各门系统）已有扎实 O 证据，无编造常量；但 **整域系统链未闭合**：camera_rig 主相机链（new/update/paused/interpolating）全缺，time_of_day 太阳→光照→音频链全缺，water 事件生产者缺，terrain 多个 SSBO 身份缺。
- **两处硬伤需立即处理**：① `terrain_rock_mask::sample_at_world_xz` 位索引错误（O 定案，恒返回 0）；② `replay_terrain::terrain_replay_from_edit` 恒 None 桩。
- **一处证据纠偏**：time_of_day 当前态 angle 过渡标志偏移注释 +3 应为 +0xc（decomp float 指针算术）。
- **⑧轮验收门建议**：G1 通过需至少（a）主相机系统链契约+实现或诚实登记；（b）time_of_day suncalc/光照/音频接缝接入或登记；（c）rock mask 位索引修复；（d）terrain_replay_from_edit 不再恒 None。
- round8 契约/DELIVERY 落盘后需按 §1 表复核（当前为进行中状态快照）。

禁审登记：rhapsody（render 链/GPU）、roof（`resources/roofs/*`、`systems/roof/*`）、wall（含 snapping/signifier）未审，仅登记交叉引用（MainCameraMoved→roof ui_edit_roof 消费；OnTerrainChange→collision/wall 链）。
