# 统一实现修正清单（第二轮审查合并，277 条）

数据：round2/FIX-LIST.json；源：8 份 `<domain>-REVIEW.md`。状态语义：
OK=与证据一致；FIX=有错误需修；WRONG=严重错误；PARTIAL=部分；GAP=证据全但未实现；STUB=占位；LATER=待下轮。

| 域 | OK | FIX | WRONG | PARTIAL | GAP | STUB | LATER |
|---|---|---|---|---|---|---|---|
| audio-misc | 26 | 18 | 0 | 0 | 19 | 0 | 0 |
| camera-time | 11 | 11 | 0 | 0 | 0 | 0 | 0 |
| collision-eco | 5 | 0 | 5 | 15 | 0 | 0 | 0 |
| history | 13 | 14 | 0 | 0 | 1 | 0 | 0 |
| session-save | 2 | 13 | 0 | 0 | 0 | 0 | 1 |
| terrain-water | 9 | 9 | 0 | 0 | 0 | 7 | 3 |
| ui-input | 7 | 0 | 10 | 16 | 0 | 0 | 0 |
| wall | 26 | 33 | 0 | 0 | 0 | 0 | 2 |
| **合计** | 99 | 98 | 15 | 31 | 20 | 7 | 6 |

## 高优先级 FIX/WRONG（每域摘最重 3 条）

### audio-misc
- `decorator_audio.rs` — 16 名单与 O 串表映射 I 且缺 `Cosmetics/Clutter_Rotate`、`Cosmetics/MoveGround`（合同串名 O）；`AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32)` 参数假设未证
- `elevation_supports_audio.rs` — 只记 `WoodenBracketsAppear`；O 串表 3 个（+`BrickColumnAppear`/`WoodenColumnAppear`，DIGEST §2.13）
- `path_audio.rs` — init 只有 CobblestoneDraw×2（loop+snappy）；O 证据 5 事件：`CobblestoneDraw` + **`Path Draw`(0x142ac1660)** + **`Erase Draw`(0x142ac1690)** + `PathWaterDraw/PathWaterEras

### camera-time
- `systems/camera_rig/mod.rs` — 文档声称"模式字节@+0x4d0 选 6 body"——证据：模式槽 0..12（13 槽，focus/lens 表各 13 项 @0x142aabb68/@0x142aabb98）；MainCamera@+0x0..0x100 = CameraMatrices 矩阵块未记；`MainCamera::new` @0x1
- `systems/camera_rig/walk.rs` — `teleport` 用 quat 基构造方向——实为方向向量→yaw/pitch 度欧拉转换（FUN_140334d80+FUN_141766a90，rad2deg 常量 0x142aba9e0/9e4）；`rotation()` = 拷贝 [self+0x18] 16B（分叉 CameraTransform rot
- `systems/main_camera.rs` — `camera_snapshot` 签名错：实参 =（Mut<快照：mode u32@+0x18、f32@+0x1c>、Res<时间：dt@+0x28>、Mut<…>、Res<MainCamera：fov 系数@+0x0ec>）逐模式 jumptable——skeleton 写成 (ResMut<MainCamera>

### collision-eco
- `world.rs` — ① `SubworldIndex` 16 变体顺序错（Terrain 真实槽=5，重建=13；真实 17 槽）② `SubworldColliderHandle` 建模 `{subworld,index}` 错（真实 24B 带负载枚举，Empty=0x11，`new` 写 {u32@+0,u32@+4,u8@+8}=
- `stairs_state.rs` — ① `restore_backup` 置 `dirty=false` 与机器码相反（+0x2f=1）；且把 backed_up（+0x178，add_node 清零/restore 置 1）与脏标志混用 ② stubs/脏边集合实际在 **+0x148**（重建 `edges/stubs: HashMap` 独立字段无
- `stairs_assembly_params.rs` — ① `StairNodeAnchor` Wall 判别=**2**（重建 enum{Wall,Free} Wall=0 错）② `AlignKind::position_offset` 实际计算 {flag u8@+1,Vec2@+4,Vec2@+0xc}+常量缩放（重建返 ZERO stub 错）③ `Playerm

### history
- `replay_cmd_buffer.rs` — 文档头 "ReplayBuffer 环形 {head,buf,cap,len,modification_counter}" 无证据；证据（replay_pop_bucket 0x140b38250）：头部 = 桶 Vec（cap-first，元素 0x40）@+0x00..+0x18 + 活跃计数 @+0x18。`po
- `decorator_edit.rs` — ① `is_default_lantern_brightness(_record: &DecoratorRecord)->bool` 签名错——证据（0x140b3f910 机器码）：`(f32)->bool`，语义 `x == 1.0f`，且与 `is_default_stair_width` 同址同码（ICF）。②
- `wall_history.rs` — `ev_height_changed` 建模为 `(&mut self, WallId)->Option<EditType>`——证据：它是 History 的 trait 方法，真流程 = `EditType::new_height_changed(buf, wall, h0, h1)`（0x140a83fb0，写 

### session-save
- `resources/session/mod.rs` — 1) `SavedSession::screenshot_path`/`loading_background_path` 均指 0x140b46a50：v2 名 = **loading_background_path**，实为 save_folder_path+`"loading_background.jpg"`（串 
- `resources/loading_screen.rs` — 1) `Plugin::build 0x140a1a190` 注释地址错：0x140a1a190 = 实际 update_loading_screen 主体（hashbrown 管理函数）；Plugin::build 地址未证。2) `update_loading_screen` 注释签名 (ResMut<Loadin
- `resources/logo_animation.rs` — 1) 布局 `_f0/_f1/wait_progress/_f2/_stages` 错：机器码 = **progress[5] f32 @+0..+0x14 + durations[5] f32 @+0x14..+0x28**。2) `is_waiting` 注释“wait 未完成”：机器码 = (+0x0c) > 0

### terrain-water
- `resources/world_raster.rs` — ① `WorldRaster(pub Gap)` 整体占位；② `RasterBufferRef` 注释“调用方从 WorldRaster 按 +0x298 字节选缓冲构造”方向对，但 `WorldRaster` 本体布局已 O，应落地
- `resources/world_raster_defs.rs` — ① 六个 `*Raster(pub RasterBufferRef<512>)` 与 O 不符——实为 Deref 到 WorldRaster 的资源（水位本体在 `WorldRaster<512>`，prior wp7 已证）；② `AllWorldRasterWrite(pub Gap)` 占位
- `resources/active_raster_stroke.rs` — 全 stub；`resample_and_smooth`/`index`/`index_mut` unimplemented

### ui-input
- `systems/mode_manager.rs` — ① AppMode 枚举缺 **Replay** 变体（判别 0xd，check_if_can_switch 写 *state=0xd）；② `AppMode::is_tool_selected @0x140a8a5d0` 是伪声明（该地址 = `timelapse_replay::cleanup_timelapse`
- `systems/mod_key.rs` — `Modifiers` 布局错：代码 4 个独立 bool；机器码 `disable_snapping` = **byte [rcx+1]**（位标志字节组）。`update_mod_key_state` 地址（0x140af9000 实为 Toolbar::new）
- `resources/toolbar.rs` — ① Toolbar::new 地址 0xaf8c50 → 实 **0x140af9000**；② 类别表：缺 **id 0 类别**；`Roofs=8` 存疑（8 未出现在 Toolbar::new 的 11 类 id 中，屋顶=Decor 0x38..0x3f 区间模式）；Buildings 列表含 (0xb,0x3

### wall
- `mod.rs` — `split_height_into_skirts` 循环条件实现 `height > f+1.4` 与机器码一致，ok；但缺 **WallId::is_valid 真身地址注记**（0x140b34f20，代码注释把它挂在 get_inner 0x140885d0 名下，见 DIGEST §7——两个函数都存在，注释
- `inner_wall_state.rs` — ① `StructureType::is_fence` 写 `discriminant() != Wall`，证据 0x1408c5680 仅"判别读取、比较在调用方内联"——Fence 值未证实（I），且 `!= Wall` 把 Other 也算 fence，**错误**（应为单变体判别，值 U）。② `is_def
- `public_wall_state.rs` — ① `point_count()`/`source_curve_length()` 注释称"偏移歧义"——已解：calc_wall_bend 读的是 0x2a8 步长条目内偏移（map 值区 +0x18 起算），与 raw 曲线 +0x10 是两个不同数组（+0x10 为 raw Curve2 点计数 O）。歧义注释应