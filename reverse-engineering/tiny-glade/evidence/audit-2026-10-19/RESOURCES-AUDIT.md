# country-core `resources` 逐文件还原度复核（B，2026-10-19）

范围：`D:/game/tiny-glade/crates/country-core/src/resources`
- 根目录全部 `.rs`（排除 `walls/`、`roofs/`）
- `resources/{stairs,history,ivy,critters,rt,render,world_raster}/**`
- 共 **102 文件 / 28,292 行 / 365 个 `#[test]`**（其中 41 个文件无测试）

输入：
- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/resources`
- 机械统计：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`
- 基线：`tiny-glade/crates/country-core/recovery/STUBS-2026-10-19.md`（仅作线索，本报告以当前源码为准）

方法：逐文件读源码/模块头；用脚本做括号配平的空体与短体扫描；用 `fn_audit.json` 的 355 条范围内骨架函数做名字/地址对照；抽读 12 个最大文件。**未运行编译/测试**，未重新反编译；证据均来自当前源码注释与其引用地址。

---

## 0. 结论速览

| 指标 | 结果 | 依据 |
|---|---:|---|
| 文件数 | 102 | 任务范围 |
| 行数 | 28,292 | `os.walk` 行计数 |
| 单元测试 | 365 | `#[test]` 计数 |
| 骨架具名函数（范围映射后） | 355 | `fn_audit.json` |
| 名字/原地址可对上（in_file） | 233（65.6%） | `fn_audit` |
| 搬移（moved） | 28（7.9%） | `fn_audit` |
| 仅名字/相似名（name_only） | 30（8.5%） | `fn_audit` |
| 仅原地址（va_only） | 5（1.4%） | `fn_audit` |
| 无名字无地址（no_trace） | 59（16.6%） | 其中约 50 条是 serde/泛型/派生符号，非业务缺口 |
| **综合完成度（本范围）** | **84%** | 见 §6；置信度中 |

主要残留不在“空函数体”，而在：
1. 少数**整函数缺失**（glade_border 搜索、stairs assembly request、camera mouse ray）；
2. **未恢复的枚举/载荷变体**（WallHistoryEdit 5..16、若干事件载荷、颜色字节枚举）；
3. **纯逻辑已恢复但宿主/消费者未闭合**（save_file 主循环、stairs render 写面、audio DelayedBankLoader、audio_settings 未接线）；
4. **语义验收未做**（`inventory.json` 的 `semantic_acceptance: false`；现有测试是合同/单测，不等于原版行为等价）。

---

## 1. 文件级状态表

图例：性质 — **真**=真实行为；**数**=数据合同（布局/serde/常量/资源类型）；**混**=混合；**声**=声明/重导出/纯事件族。  
“缺口”只登记本文件内的具体问题；跨 crate 的消费者问题见 §5。

### 1.1 根目录（62 文件）

| 文件 | 行数 | 性质 | 判定 / 缺口 |
|---|---:|---|---|
| active_raster_stroke.rs | 485 | 真 | 6 槽笔画状态机、平滑/固化/换脏位；14 测试；`param_a/param_b` 用途未恢复（:17-18） |
| audio_instance.rs | 225 | 真 | 播放状态判定与循环播放器；FMOD 经 trait 注入 |
| audio_resources.rs | 928 | 混 | FMOD 边界 + 银行/VCA/实例策略为真实；`visualize_sounds` 返回空、`DelayedBankLoader` 为半模型（见 §2） |
| audio_settings.rs | 34 | 数 | 4×f32 原 serde 面；**未接线**，`systems::audio` 仍有 5 音量占位（:10-11） |
| brush_sizes.rs | 138 | 真 | 三工具默认/范围、maxss/minss NaN 语义；5 测试 |
| color_icon.rs | 53 | 数 | 亮度三 tag + 两个 ZST 图标标记；变体拼写未取证 |
| color_id.rs | 520 | 混 | ColorId 及墙/门/屋顶等颜色表（Vec<u32>）+ iter；数据面为主 |
| color_id_roof.rs | 67 | 数 | RoofColorIdSsbo 4B、名称/容量锚定；1 测试 |
| compute_mouse_depth_value.rs | 72 | 数 | MouseDepthValueSsbo [f32;4] 与适配器 |
| cursor_terrain_raycast.rs | 76 | 混 | 32B 布局 + position 方法；速度除数身份待取证（:13,33） |
| drawing_input_smoother.rs | 2081 | 真 | 仅 1 个 `process` + 3 个巨型原版夹具测试；真实行为、重测试 |
| editing_highlight.rs | 18 | 数 | Vec3+bool，clear 与 13B 存储序列对应 |
| events.rs | 93 | 数 | 命令事件族；`ModeJustChanged` 0xc..0x20、`roof_gate` 语义 U |
| game_clock.rs | 289 | 真 | 0x38B 时钟与 time_update 门控/panic 语义；6 测试 |
| garden_mask.rs | 36 | 数 | GardenMaskSsbo 0x2000 回读 |
| glade_border.rs | 59 | 混 | 圆/矩形内外判定；**缺 `search_lerped_shape_that_fits_into_glade`**（0x140ad2ce0） |
| glade_color_customization.rs | 81 | 数 | 15 字段 serde；3 个字节枚举变体名待取证（:13,23,25,41） |
| glade_settings.rs | 124 | 数 | 11 键 JSON schema + 默认值；文件加载回退；测试完整 |
| gothic_window_profile.rs | 180 | 真 | 1/2/3 号哥特剖面常量表（两函数） |
| gpu_lut.rs | 145 | 真 | BRDF/水 LUT 图与 once-per-tracker 渲染；插件不自动安装（设计） |
| grading_lut.rs | 449 | 混 | .cube/PNG→3D 纹理设备接线；解析在 grading_lut_cube |
| grading_lut_cube.rs | 276 | 真 | .cube 逐行解析、错误文本；14 测试 |
| gui.rs | 740 | 真 | Esc/加载/设置菜单策略、缩略图坐标；13 测试 |
| image_formation.rs | 353 | 混 | 三档 0x40 配置 + JSON 视图；进程布尔写入者未找到（:230） |
| in_progress_curve.rs | 1355 | 真 | 原始点权重 + 增量 RDP；18 个原版链测试 |
| logo_animation.rs | 290 | 真 | 五阶段进度/时长与 clamp 语义；8 测试 |
| mod.rs | 72 | 声 | 模块声明 |
| mouse_capture.rs | 194 | 真 | 捕获位置/策略/回放所有权；1 测试 |
| mouse_terrain_raycast.rs | 32 | 数 | CameraCursorRay + intersect；**`camera_mouse_ray` 0x1408ef1c0 无痕迹** |
| onboarding.rs | 225 | 数 | 16 通知 + TutorialOp 映射；事件载荷仅已知字节 |
| path_stroke.rs | 30 | 数 | PathDraw/EraseEvent 两事件 |
| photo_mode.rs | 60 | 数 | 拍照相机循环表 + CameraMode 转换；1 测试 |
| placement_data.rs | 119 | 数 | PlacementDataSsbo 48B stride |
| platform.rs | 22 | 数 | 1 字节运行平台推导 |
| prefab.rs | 237 | 混 | PrefabLibrary add/get/has + 部件/音频触发；4 测试 |
| raster_stroke.rs | 22 | 数 | 光栅历史载荷 Arc<[Vec3]> |
| render_input_views.rs | 10 | 声 | 仅 IvyDrawView；明确“非权威 schema” |
| render_ssbo.rs | 119 | 声 | 间接缓冲重导出 + 两个适配类型；旧 Gap 已删 |
| rocky_terrain.rs | 415 | 混 | JSON→36/16B→GPU 48/32B 打包；5 测试（1 需游戏资源） |
| roof_tile_ssbo.rs | 290 | 数 | 0x40000 元素、64B、0x2000 chunk 锚定；4 测试 |
| save_file.rs | 434 | 混 | 节表/脏块/追加/升级纯逻辑；**完整 CurrentSaveFile、异步主循环、领域快照采集未恢复**（:16） |
| session.rs | 708 | 真 | SessionId/Meta/NextSession/列表/删除/Flow；事件载荷待取证（:555）、缩略图过滤待接（:495） |
| shader_refactoring_state.rs | 17 | 声 | 重导出既有资源 + `disabled()` |
| shutdown.rs | 38 | 真 | 关闭命令→ZST 资源；1 测试 |
| sparkle.rs | 118 | 数 | SparkleSsbo 32B、0x300 chunk |
| terrain_editing.rs | 29 | 声 | ECS 包装 `tg_terrain::TerrainEditingData`；原 startup 初始化另处 |
| terrain_height.rs | 478 | 真 | 512² u16 镜像、采样/射线/脏块；12 测试；**原构造点未恢复**（:94） |
| terrain_rock_mask.rs | 111 | 数 | TerrainRockMaskSsbo + 位采样；2 测试 |
| texture_presets.rs | 257 | 数 | 25 预设常量表 + 用途位；critter/ivy 纹理名待资源库（:11） |
| time.rs | 4 | 声 | `GameTime = GameClock` 重导出 |
| time_of_day.rs | 523 | 真 | 小时重映射 + 关键帧序列插值；11 测试 |
| toolbar.rs | 246 | 真 | 分类/循环/switch_mode；2 测试 |
| tree_storage.rs | 127 | 数 | 索引→实体表；typed gap 其余字段 U（:74） |
| ui_settings.rs | 47 | 数 | UiSettings schema/默认值 |
| water.rs | 93 | 数 | RippleSourceSsbo 528B 原始词 + 两事件；CPU 生产者字段未解 |
| water_exclude_clutter_mask.rs | 37 | 数 | 0x2000 回读 SSBO |
| water_mask.rs | 211 | 数 | 位图语义/阈值/采样常量；2 个库构造测试 |
| weather.rs | 28 | 数 | Weather 16B 布局 |
| window.rs | 102 | 数 | WindowSize u32×2 + OsWindowCmd |
| world_raster.rs | 838 | 真 | 512² 双缓冲、脏 tile、blend、rasterize；24 测试；一个原版 `unimplemented!` 分支被忠实保留 |
| world_raster/blend_policy.rs | 70 | 数 | WriteMode 投影；None 表示原 panic 组合 |
| world_raster_defs.rs | 451 | 混 | 六个命名栅格 + 读写/上传/校验；5 测试 |

### 1.2 `critters/`、`ivy/`、`rt/`、`render/`、`stairs/`（27 文件）

| 文件 | 行数 | 性质 | 判定 / 缺口 |
|---|---:|---|---|
| critters/mod.rs | 16 | 数 | CritterGoal（position+tag） |
| critters/butterfly.rs | 20 | 数 | 0x18 stride 六字段；`state_t` 原名 U |
| critters/sheep.rs | 81 | 数 | Sheep 组件 + SheepMoveState + being_petted；1 测试 |
| critters/sheep/control.rs | 201 | 混 | 存档快照 JSON；3 测试 |
| ivy/mod.rs | 10 | 声 | 重导出 |
| ivy/ivy_direction_proposer.rs | 105 | 真 | bracket_noise/perchance 组合方向提议 |
| ivy/ivy_leaf_storage.rs | 98 | 混 | 叶片表 + 删除谓词；RNG 流独立 |
| ivy/ivy_storage.rs | 205 | 真 | 折线/根/待生长段/冷却；4 测试（骨架） |
| ivy/wall_coord.rs | 47 | 真 | 墙坐标→世界坐标，含法线 Y 槽修正 |
| rt.rs | 891 | 真 | RtWorld/RtMesh/DirtyCow/SSBO/上传；14 测试 |
| rt/bvh.rs | 111 | 真 | obvhs 参数选择/三角形重排；2 测试 |
| rt/bvh_builder.rs | 215 | 真 | 单 pending/单 completed 异步重建；3 测试 |
| render/mod.rs | 104 | 混 | Affine3Packed 4 函数 + 子模块 |
| render/mesh_atlas_library.rs | 460 | 真 | 0x50B 库、27 subset、名称转换；nohash 用 std HashMap 替代（不可观察）；10 测试 |
| render/spatial_resolve_offsets.rs | 241 | 数 | 32×16 偏移表逐项转录；5 测试 |
| stairs/mod.rs | 22 | 声 | 模块与重导出 |
| stairs/invalidate.rs | 83 | 真 | 脏边 drain→contains→remove/reset；碰撞体/RT 撤除为**待接线胶水**（:31） |
| stairs/stairs_assembly_params.rs | 318 | 混 | AlignKind/PreferredNodeAlignment/长度；**assembly request owner 与 `prev_requests` 未恢复** |
| stairs/stairs_collider.rs | 11 | 声 | ZST 标记资源（原文件本身无业务函数） |
| stairs/stairs_removed_supports.rs | 36 | 数 | HashMap<StairGraphEdge,()> 存档集 |
| stairs/stairs_state.rs | 431 | 真 | petgraph 权威图/快照/校验和/diff；endpoint 24B 载荷 schema U |
| stairs/stairs_top_down.rs | 12 | 数 | ManagedHandle + 顶点数 |
| stairs/structural_graph.rs | 328 | 真 | 6 表 + 图操作；**+0x120/+0x180 两表、edge_index 落点 U**（:8,100） |
| stairs/visual.rs | 357 | 混 | 存储核心 remove/reset；**RtMesh/实体 despawn/0x110+ 字段为占位**（:85-96） |
| stairs_walls.rs | 77 | 混 | 44B 支柱提议 HashMap append；记录字段语义 U |

### 1.3 `history/`（14 文件）

| 文件 | 行数 | 性质 | 判定 / 缺口 |
|---|---:|---|---|
| history/mod.rs | 336 | 混 | EditType 15 变体 tag 表 + 录制校验；未恢复载荷不造空数据 |
| history/auto_clutter_edit.rs | 47 | 数 | Delete{id} 外部标签 |
| history/building_transactions.rs | 530 | 混 | ledger/事务 + roof/wall/height 回放命令；仅已证并发组可执行 |
| history/color_edit.rs | 522 | 混 | 按 target 互异载荷的手写 serde；4 测试 |
| history/decorator_edit.rs | 1386 | 混 | 0x4fB/0x50/0x58/0x60/0x88 布局 + diff/事件/serde；**stair width 偏移 U**（:110）、wire key 占位编码（:811） |
| history/path_edit.rs | 120 | 混 | RasterEdit 手写 serde；`From` 源 +0/+8/+30/+38 布局 U（:15） |
| history/replay_cmd_buffer.rs | 604 | 真 | 15 tag→命令展开、aux diff、timing；7 测试；tag11 `unreachable!` 与原版一致 |
| history/serialized.rs | 404 | 混 | 存档 Wall/MultiEdit 解码；**未恢复变体解码失败**（不伪造） |
| history/snapshots.rs | 554 | 真 | 最近快照二分、pending id、resolved 过滤；15 测试 |
| history/stairs_history_edit.rs | 204 | 混 | 单边枚举 serde；2 测试 |
| history/terrain_edit.rs | 923 | 混 | 4 函数 + 25 原版链测试；实际 schema/命令委托 `tg_terrain` |
| history/transactions.rs | 420 | 真 | History 录制/分支/cursor/事务错误；26 函数 |
| history/tree_edit.rs | 435 | 混 | 树载荷六字段序 + Delete 变体；3 测试 |
| history/wall_edit.rs | 842 | 混 | 复合编辑/快照/即时写入；**原生变体 5..16 载荷未恢复**（:102-165） |

---

## 2. 空壳与“水分”清单（文件:行）

### 2.1 真 `todo!/unimplemented!` / 等价 panic

| 位置 | 内容 | 判定 |
|---|---|---|
| `world_raster.rs:179-192` | `StrokeBlendMode::select` 在 `flags&1!=0 && !draw` 时 `panic!("not implemented")` | **不是缺口**：模块注释（:166）说明原版源码行 398 就是 `unimplemented!()`，宏被清单禁用故改写；测试 `odd_flags_without_draw_hit_unimplemented`（:783）覆盖 |
| `history/wall_edit.rs:281` | `panic_deprecated_operation` | 原版 panic 语义（待取证文案，:280） |
| `render/mesh_atlas_library.rs:224` | `get_by_world_subset` 未命中 panic | 原版 panic |
| `stairs/stairs_assembly_params.rs:112,184` | `AlignKind::None` outgoing_dir/position_offset panic | 原版分支 |
| `history/replay_cmd_buffer.rs:257-259,274` | `unreachable!` 文本 | 原版 tag11/内部错误 |
| `history/decorator_edit.rs:682` | `unreachable!("decorator diff: both sides have no storage")` | 不变量断言 |

本范围**没有**当前仍以 `todo!()`/`unimplemented!()` 留空的函数。基线的 5 个 `todo!` 均不在本范围（`world_raster.rs:179` 已按原语义改写）。

### 2.2 空体 / 仅默认值 / 仅 no-op

| 位置 | 函数 | 说明 |
|---|---|---|
| `history/transactions.rs:10` | `validate_for_recording` | trait 默认空体；原版其他 edit 无 add 期校验，**不是缺口** |
| `history/snapshots.rs:233` | `resolution_state` for `()` | 测试夹具返回 0，生产用 `MaybePendingSnapshotId` |
| `history/snapshots.rs:45` | `MaybePendingSnapshotId::default` | `Self::new()`（0 = pending），真实默认 |
| `audio_resources.rs:76` | `EventInstance::release` | trait 默认 `Ok(())`；FMOD 释放由实现方 |
| `color_id.rs:37/63/89/116`、`ivy/*::default`、`terrain_height.rs:305` 等 | `Default`/`new` | 真实空表/空网格构造 |
| `stairs/visual.rs:96` | `_opaque: ()` | **占位字段**，RtMesh 句柄等 0x110+ 字段 U |
| `stairs/stairs_collider.rs:11` | `StairsCollider` ZST | 原文件只有标记资源 |

### 2.3 半实现 / 占位模型（有函数体但语义未闭合）

| 位置 | 内容 | 证据 |
|---|---|---|
| `audio_resources.rs:566-573` | `visualize_sounds` 返回空 `Vec` | 注释“待取证: exact per-instance position access”，渲染在宿主 |
| `audio_resources.rs:595-614` | `DelayedBankLoader::load` / `pending_bank` | `pending_bank` 恒 `Some(())`；原生耦合“待取证”，模型仅做 deferred 包装 |
| `audio_settings.rs:10-11` | 本类型未接线 | `systems::audio::audio_update::AudioSettings` 仍是 5 音量占位；本文件明确“解锁条件：audio_update 改用本类型后删除那份占位” |
| `stairs/visual.rs:85-96,133` | `StairsEdgeVisual` 字段占位、dirty 语义 U | 渲染写面另置 `reset_edge_with`，但实体 despawn/缓存未建模 |
| `stairs/structural_graph.rs:219-223` | `register_edge_index` | 填充源 U；原为 +0x48 IndexMap |
| `history/decorator_edit.rs:811-833` | diff map wire key | 原生 map 键未取证，使用可逆占位编码 `wall:<id>`/`terrain`/`stair` |
| `history/path_edit.rs:52-62` | `from_points` | 源结构其余四字段按默认填充登记 U |
| `save_file.rs:15-16` | 存档主循环/枚举/快照采集 | 明确“尚未恢复完整 CurrentSaveFile 枚举、异步存档主循环/命令与所有领域快照采集” |
| `terrain_height.rs:94` | `new()` 全零网格 | 原构造点未恢复（U），仅提供可用起点 |
| `tree_storage.rs:74` | typed gap | 其余字段 U |
| `texture_presets.rs:11` | critter/ivy 纹理名 | 待资源库恢复（U） |
| `image_formation.rs:230` | 进程级布尔 | .bss 初值 0，写入者未找到（U） |
| `session.rs:495,555` | 缩略图过滤/事件载荷 | “待接入”；`NewSessionStartedCmd`/`SessionLoadedCmd` 载荷待取证 |
| `cursor_terrain_raycast.rs:13,33` | 速度除数 | 资源身份待取证 |
| `stairs/invalidate.rs:31` | 碰撞体/RT 撤除 | 胶水登记，collision/rt 域待接线 |
| `stairs/visual.rs:11` | remove/reset 的 RtWorld 删除 | 胶水登记待接线 |

### 2.4 “文档水分”评估

本范围不存在“几百行注释 + 空实现”的文件。最重的注释型文件（`world_raster.rs`、`grading_lut.rs`、`rt.rs`、`stairs_state.rs`）都有对应可执行主体与测试。需要警惕的是**测试与源码的比例**：

- `drawing_input_smoother.rs`：2081 行中约 1900 行是 3 个原版夹具测试，业务主体只有 `process` 一个函数（仍属真实实现）。
- `in_progress_curve.rs`：1355 行中 18 个测试占约 1100 行。
- `history/terrain_edit.rs`：923 行中 25 个测试占约 750 行，业务函数 4 个。
- `grading_lut_cube.rs`、`time_of_day.rs`、`active_raster_stroke.rs` 类似：测试占比高，但非空壳。

---

## 3. 骨架对照：no_trace 函数

`fn_audit.json` 中范围内 355 条骨架函数的状态：in_file 233 / moved 28 / name_only 30 / va_only 5 / no_trace 59。

### 3.1 非业务 no_trace（派生/serde/泛型符号，约 50 条）

以下形式全部是 serde derive、泛型单态化或编译器生成的 visitor 符号，**不计业务缺口**：

`Serializer>`、`Value>`、`Global>`、`Error>`、`String>`、`StrRead>`、`SliceRead>`、`SeqDeserializer>`、`MapDeserializer>`、`WallId>`、`VisualTerrainHeightSsbo>`、以及 `color_id`/`ui_settings`/`toolbar`/`time_of_day`/`session`/`placement_data`/`raster_stroke`/`glade_settings`/`grading_lut`/`rocky_terrain`/`image_formation`/`world_raster`/`audio_settings`/`critters/sheep`/`history/*` 中的同类项。

### 3.2 需逐个判定的 no_trace（9 条）

| 骨架文件:函数 | 原地址 | 判定 | 证据 |
|---|---|---|---|
| `critters.rs:create_spots` | 0x140f86340 | **搬移** | `crates/systems/critters/src/bird_spots.rs:37` 同名实现；`wren.rs:101` 调用 |
| `critters.rs:closure_env$3>` | 0x1422f86e0 | **搬移** | `bird_spots.rs:3` 注释直接给出 `recycle`（0x1422f86e0） |
| `glade_border.rs:search_lerped_shape_that_fits_into_glade` | 0x140ad2ce0 | **真缺失** | 整个 tiny-glade workspace 无此名/地址；当前 `glade_border.rs` 只有 `is_pos_inside`/`is_shape_inside` |
| `stairs/stairs_assembly_params.rs:prev_requests` | 0x140274480 | **真缺失 / 待人工** | workspace 无此名/地址；模块头说 assembly-request owner 与 graph-to-geometry producer 是分开的 |
| `mouse_terrain_raycast.rs:camera_mouse_ray` | 0x1408ef1c0 | **真缺失 / 待人工** | workspace 无此名/地址；当前只有 `intersect_with_xz_plane`（0x1408ef180），可能为另一函数的改名/内联，需人工 |
| `mouse_terrain_raycast.rs:closure$0` | 0x1408efa70 | 闭包/待人工 | 源码无地址；可能内联进 camera ray 生产者 |
| `audio_resources.rs:closure$0` | 0x140a537e0 | 闭包/待人工 | 源码无地址；可能在 `AudioCore` 某方法内联 |
| `history/mod.rs:constructor$0` | 0x1416bb7f0 | 闭包/派生 | 与 `EditType` 序列化/构造相关，非独立业务 |
| `stairs/stairs_state.rs:closure_env$0>` | 0x14157d090 | serde/闭包 | 文件内无同名闭包；判定为派生/内联 |
| `gpu_lut.rs:closure$0` | 0x140a71980 | **改名/内联** | 当前实现为 `GpuLutTracker::new` 中的 `GpuLutDesc::new(|api| ...)` 闭包；行为存在 |

**结论**：真正的非派生业务缺口集中在 3 个函数（glade border 搜索、stairs prev_requests、camera_mouse_ray），另有 3 个闭包需人工确认；其余 50 条是 serde/泛型噪声。

### 3.3 基线中“完全没迹象”的本范围条目复核

基线 §2 列出的本范围条目：
- `history/snapshots.rs:233 resolution_state`：**已实现**（trait 夹具返回 0，生产实现读原子值）。
- `history/transactions.rs:10 validate_for_recording`：**trait 默认空体**，符合原版（其他 edit 无校验）。
- `image_formation.rs:238 initial`：**已实现**（`Self::default()`，kind=1）。
- `world_raster.rs:179 select`：**已实现**（含原版 panic 分支）。
- `terrain_editing.rs get_stroke/last_stroke_id/set_stroke`：**委托** `tg_terrain::TerrainEditingData` 的字段/方法（`crates/systems/terrain/src/stroke.rs`）。
- `glade_border is_curve2_inside/init_glade_border`：`is_shape_inside` 覆盖前者语义；`init_glade_border` 未单独落地（启动器在上层/系统域，需人工）。
- `history/mod.rs closest_snapshot` 等 8 项：当前由 `snapshots.rs` / `transactions.rs` 分文件实现（搬移）。
- `audio_resources is_paused/fmod_to_anyhow`：未在当前文件出现；`is_playing` 存在，`fmod_to_anyhow` 未见（需人工）。

---

## 4. 抽读 12 个最大文件：真实实现 vs 签名壳

| # | 文件 | 行/业务函数/测试 | 结论 |
|---|---|---:|---|
| 1 | `drawing_input_smoother.rs` | 2081 / 1 / 3 | **真实实现**。`process` 按原始时钟/相机/上一接受点重采样；1900 行原版夹具逐位断言。不是签名壳。 |
| 2 | `in_progress_curve.rs` | 1355 / 13 / 18 | **真实实现**。原始点权重、RDP 增量缓存、`compute_final_curve` 平滑；18 条原版曲线链测试。 |
| 3 | `history/decorator_edit.rs` | 1386 / 36 / 19 | **真实实现（有未决字段）**。0x4fB 状态/0x50/0x58/0x60/0x88 记录、diff 事件与手写 serde 齐全；`stair width` 偏移与 wire key 仍 U。 |
| 4 | `audio_resources.rs` | 928 / 67 / 14 | **大部分真实，两处半模型**。实例判定/循环/VCA/银行加载有汇编锚点；`visualize_sounds` 空返回，`DelayedBankLoader` 为伪耦合。 |
| 5 | `history/terrain_edit.rs` | 923 / 4 / 25 | **真实但薄**。业务只有 id/duration/update_curve/apply；schema 与命令在 `tg_terrain`；25 条原版链测试是主要证据。 |
| 6 | `rt.rs` | 891 / 42 / 14 | **真实实现**。RtWorld/RtMesh/DirtyCow/上传边界与容量 panic；14 测试；异步 builder 在 `rt/bvh_builder.rs`。 |
| 7 | `history/wall_edit.rs` | 842 / 19 / 6 | **真实实现 + 明确的数据缺口**。变体 0-4 的载荷/即时写入已恢复；变体 5..16 以 `UnresolvedWallEditPayload` 占位，注释逐项登记判别值。 |
| 8 | `world_raster.rs` | 838 / 42 / 24 | **真实实现**。双缓冲/脏 tile/hash/blend/rasterize；一个原版 `unimplemented!` 被忠实保留；24 测试。 |
| 9 | `session.rs` | 708 / 53 / 1 | **真实实现（宿主面部分）**。Meta/GladeType/NextSession/列表/删除/Flow；缩略图过滤与两个事件载荷尚未闭合；测试集中在一个大用例。 |
| 10 | `gui.rs` | 740 / 41 / 2 | **真实实现**。Esc/加载/设置菜单状态与视图策略全为跳表/位语义；缩略图坐标有测试。 |
| 11 | `history/snapshots.rs` | 554 / 26 / 15 | **真实实现**。最近快照无分支二分的平局取后者、resolved>=2、pending 原子 id；15 测试。 |
| 12 | `history/replay_cmd_buffer.rs` | 604 / 18 / 7 | **真实实现**。15 tag→命令、undo 逆序、aux diff、timing；tag11 `unreachable!` 与原版一致。 |

补充（非最大但关键）：`save_file.rs` 是**纯逻辑真实 + 主循环缺失**；`stairs/visual.rs` 是**存储核心真实 + 渲染写面占位**；`terrain_editing.rs` 是**薄委托**而非独立壳。

---

## 5. 字段级未知、数据格式未定、生命周期/消费者缺失

### 5.1 字段级未知（文件:行）

| 位置 | 未知内容 |
|---|---|
| `glade_color_customization.rs:13,23,25,41` | 3 个字节枚举（`building_wall_style`/`freehand_ty`/`flat_rim_style`）的变体名与语义 |
| `events.rs:43-49` | `ModeJustChanged` 载荷 0xc..0x20（仅 tool 已知）；`ConstructTimberframeCmd.roof_gate` 名义语义 |
| `critters/butterfly.rs:10` | `state_t` 原字段名 U（行为已有） |
| `cursor_terrain_raycast.rs:13,33` | speed 除数资源身份 |
| `history/decorator_edit.rs:110` | stair width 字段偏移（与 lantern 同址，仅 `x==1.0`） |
| `history/decorator_edit.rs:811-833` | 原生 diff map 键编码 |
| `history/path_edit.rs:52-62` | `From` 源结构 +0x00/+0x08/+0x30/+0x38 四字段 |
| `history/wall_edit.rs:102-165` | 原生变体 5..16 的载荷（判别值与操作号已确认，字段未恢复） |
| `stairs/stairs_state.rs:4,69-89` | endpoint 24B 的活跃载荷 schema；未激活字节无原值 |
| `stairs/structural_graph.rs:8,100-101` | +0x120/+0x180 两表语义；`edge_index` 原生落点 |
| `stairs/visual.rs:85-96` | `StairsEdgeVisual` 的 RtMesh 句柄/0x110+ 缓存字段 |
| `stairs_walls.rs:16` | 44B 支柱提议记录字段语义 |
| `tree_storage.rs:74` | typed gap 其余字段 |
| `water.rs:37-38` | `RippleSourceSsbo` 528B 的 CPU 生产者字段（保留原始词） |
| `image_formation.rs:230` | 进程级布尔写入者 |
| `save_file.rs:16` | `CurrentSaveFile` 枚举与命令载荷 |
| `session.rs:555` | `NewSessionStartedCmd`/`SessionLoadedCmd` 载荷 |
| `audio_resources.rs:254-269,569,599-614` | 环停止阈值/循环标志默认值；可视化位置访问；DelayedBankLoader 耦合 |
| `glade_border.rs` | `search_lerped_shape_that_fits_into_glade` 的整个函数语义（缺失） |

### 5.2 数据格式未定

- **存档/事件主格式是 JSON**（`serde_json`；`systems/save/save_game.rs` 还有 `CompactFormatter` 路径）。本范围未发现 bincode/postcard/rmp/ron 使用。
- JSON 未定项：
  - Wall 复合编辑历史：`history/serialized.rs` 明确“未恢复变体解码失败，不变成空 edit”；
  - `path_edit` 的 `RasterEdit` 手写 map 四键已定，但源结构四字段未对齐；
  - `tree_edit` 六键已定，`previous_position` 不写盘（运行时旧位）；
  - `glade_color_customization` 三字节枚举以数字/字节保留；
  - `decorator_edit` diff map 键使用可逆字符串占位。
- 二进制未定项：
  - `save_file` 节表/0x58 记录/0x40 升级映射已部分恢复，但完整文件名册与异步命令缺；
  - `stairs/stairs_state` 24B endpoint 载荷；
  - `stairs/structural_graph` 未建模表；
  - `water::RippleSourceSsbo` 528B 原始词；
  - `audio` 循环/银行延迟结构；
  - `image_formation` 单档 0x40 的字段语义（按偏移命名）；
  - `stairs/visual` 0x110+ 状态。

### 5.3 资源生命周期 / 消费者缺失

| 资源/类型 | 问题 |
|---|---|
| `AudioSettings` | 原类型已落地，但 `systems::audio` 尚未改用；旧 5 音量占位仍在（`audio_settings.rs:10-11`） |
| `visualize_sounds` | 库侧返回空，宿主渲染消费者未接线 |
| `DelayedBankLoader` | 与 `AudioCoreInner` 的耦合未取证 |
| `save_file` | 影子 I/O/节更新/备份节拍器已接 `systems::save`，但异步存档主循环/命令/领域快照采集未恢复 |
| `stairs/invalidate` | 碰撞体×3 撤除、Arc drop、RtMesh 删除为“待接线”胶水 |
| `stairs/visual` | `remove_edge/reset_edge` 的 RtWorld/实体 despawn 写面未接 |
| `render_input_views::IvyDrawView` | 明确只是 draw callback 输入视图；IvyStorage 创建/生长未导入该视图 |
| `ShaderRefactoringState` | 重导出同一资源身份（避免双资源），消费者在 render/game-time |
| `GpuLutPlugin` | 原插件读进程标志；宿主显式传入 demo 标志，加载 App 不自动安装 |
| `water/garden/rock mask` | 构造器与 pass 引用有测试；回读消费链在 `systems/render_loop`，不属本范围 |
| `session` | 缩略图过滤“待接入”；列表/删除已实现 |
| `StairsTopDownMesh` | 无默认资源，由楼梯网格生产者插入/移除（生命周期正确，但生产者不在本范围） |
| `terrain_editing` | 仅 ECS 包装；startup 初始化与消费者在 `systems/terrain` |
| `events.rs` / `onboarding.rs` / `path_stroke.rs` / `glade_border.rs` 事件 | 生产/消费者多在上层 crate；本范围只落类型合同 |

---

## 6. 仍未复现清单（≥12 条，带证据）

1. **`glade_border::search_lerped_shape_that_fits_into_glade`（0x140ad2ce0）**：骨架有、当前 59 行文件无、workspace 无同名/同址。影响“形状放入林地边界”的二分搜索语义。
2. **`stairs_assembly_params::prev_requests`（0x140274480）**：workspace 无痕迹；模块头承认 assembly-request owner 与 graph-to-geometry producer 尚未恢复。
3. **`mouse_terrain_raycast::camera_mouse_ray`（0x1408ef1c0）**：workspace 无痕迹；当前只有 `intersect_with_xz_plane`（0x1408ef180），二者地址不同，需人工确认是否另有相机射线生产者。
4. **`save_file` 的完整 `CurrentSaveFile` 枚举、异步存档主循环/命令、所有领域快照采集**：源码自述（`save_file.rs:15-16`）；已恢复的只是节表/脏块/追加/升级纯逻辑。
5. **`WallHistoryEdit` 原生变体 5..16 的载荷**：`wall_edit.rs:102-165` 逐项登记判别值 `0x8000000000000004..f`，但载荷为 `UnresolvedWallEditPayload`；`wall_id()`/即时写入面对这些变体返回 None/不承接（:207,369）。
6. **`audio_resources::visualize_sounds` 的实例位置读取**：`audio_resources.rs:566-573` 返回空 Vec，注释“待取证”。
7. **`DelayedBankLoader` 的真实银行查找/与 `AudioCoreInner` 耦合**：`audio_resources.rs:595-614`，`pending_bank` 恒 `Some(())`。
8. **`AudioSettings` 接线**：`audio_settings.rs:10-11` 明确旧 5 音量占位仍在，未删除。
9. **`stairs/visual` 的 RtMesh 句柄/实体 despawn/0x110+ 缓存**：`visual.rs:85-96,133`；`reset_edge_with` 只覆盖两个 mesh-atlas 写面。
10. **`stairs/structural_graph` 的 +0x120/+0x180 两张附加表与 `edge_index` 原生落点**：`structural_graph.rs:8,100-101`；当前只建模 6 表。
11. **`stairs/stairs_state` 的 24B endpoint 活跃载荷 schema**：`stairs_state.rs:4,69-89`；未激活字节“无原值”。
12. **`glade_color_customization` 三个字节枚举的变体名/语义**：`glade_color_customization.rs:13,23,25,41`。
13. **`events::ModeJustChanged` 0xc..0x20 载荷与 `ConstructTimberframeCmd.roof_gate` 语义**：`events.rs:43-49`。
14. **`history/decorator_edit` 的 stair width 偏移与 diff wire key 编码**：`decorator_edit.rs:110,811-833`。
15. **`history/path_edit` 的源结构 +0x00/+0x08/+0x30/+0x38 四字段**：`path_edit.rs:15,52-62`。
16. **`water::RippleSourceSsbo` 的 CPU 生产者字段**：`water.rs:37-38`（仅保留 528B 原始词）。
17. **`terrain_height` 的原构造点**：`terrain_height.rs:94`；当前 `new()` 全零网格 + 默认范围。
18. **`tree_storage` typed gap 其余字段**：`tree_storage.rs:74`。
19. **`texture_presets` 的 critter/ivy 纹理名**：`texture_presets.rs:11`。
20. **`session` 的缩略图过滤与两个会话事件载荷**：`session.rs:495,555`。
21. **`image_formation` 进程级布尔的写入者**：`image_formation.rs:230`。
22. **`stairs/invalidate`/`stairs/visual` 的 collision/rt 胶水接线**：`invalidate.rs:31`、`visual.rs:11`。
23. **`audio_resources is_paused/fmod_to_anyhow`（基线点名）**：当前文件未见对应实现；需人工确认是否已合并进 `is_playing`/`Result` 适配。

---

## 7. 完成度与置信度

### 7.1 分项

| 维度 | 估计 | 依据 |
|---|---:|---|
| 结构/命名覆盖 | **96%** | 355 条骨架函数中 296 条有 in_file/moved/name_only/va_only 证据（83.4%），59 条 no_trace 中约 50 条为 serde/泛型噪声；真实非派生缺口约 3-6 条（1-2%） |
| 数据合同（布局/serde/常量） | **90%** | 大多数 SSBO/记录有字节偏移与测试；残留为 3 字节枚举、Wall 变体载荷、stairs 两表、RippleSource 字段、endpoint 载荷 |
| 行为闭包（本范围可执行代码） | **85%** | history/rt/world_raster/gui/session/terrain 主体真实；save_file 主循环、audio 两处半模型、stairs render 写面、glade border 搜索未闭合 |
| 语义验收 | **低（未做）** | `recovery/inventory.json` `semantic_acceptance: false`；现有 365 测试为合同/单测，不是原版执行等价验证 |

### 7.2 综合

**本范围（102 文件、28.3k 行）综合还原度约 84%，置信度：中。**

- 支持 84% 的证据：名称/地址面覆盖约 96%；绝大部分文件有真实主体与布局锚点；空壳仅数处且多为原版 panic 语义。
- 压低到 84% 的证据：至少 3 个整函数缺失、12 个 Wall 变体载荷未恢复、save_file 主循环/audio/stairs render 三块生命周期未闭合、若干事件/枚举字段未知；并且语义等价未验证。
- 不确定项（需人工）：`camera_mouse_ray` 是否只是改名/内联；`audio closure$0`、`history constructor$0`、`stairs closure_env$0>` 的归属；`init_glade_border` 是否已在上层；`is_paused/fmod_to_anyhow` 是否被合并。

### 7.3 与基线数字的差异

- 基线“`resources` 226/267 对得上、29 完全没迹象”是按目录粗算；本报告按 102 文件精确映射得到 355 条骨架函数、59 条 no_trace，并剔除了约 50 条 serde/泛型噪声。
- 基线称 `resources` 仅 1 个 `todo!`、3 个空壳；本范围复核后确认：**当前无 `todo!`**，`world_raster.rs:179` 已按原语义改写为等价 panic；真正的空壳是 3 处半模型 + 1 处 ZST 标记 + 若干 `Default`。
- 基线“完全没迹象 304 个（全 crate）”中，本范围真正可归为业务缺失的只有 3 个函数，其余大多是派生符号或已搬移到 `systems/*`。

---

## 8. 建议的下一步（按收益排序）

1. 补 `glade_border::search_lerped_shape_that_fits_into_glade`（0x140ad2ce0），并确认 `init_glade_border` 的宿主位置。
2. 恢复 stairs assembly-request owner 与 `prev_requests`（0x140274480），闭合 `stairs_assembly_params` 的上游。
3. 确认 `camera_mouse_ray`（0x1408ef1c0）是改名/内联还是真缺失；若是生产者，补到 `mouse_terrain_raycast`。
4. 继续 `WallHistoryEdit` 5..16 载荷（与 `wall_compound_edit` 章节证据对齐），否则 replay/存档对这些变体只能失败。
5. 把 `save_file` 的 `CurrentSaveFile`/异步主循环/领域快照采集从 `systems/save` 反向对齐到本模块的纯逻辑；区分“宿主接线”与“未恢复逻辑”。
6. `AudioSettings` 接线并删除 `systems::audio` 的 5 音量占位；补 `visualize_sounds` 的实例位置 getter；给 `DelayedBankLoader` 定性（真模型或删除）。
7. `stairs/visual` 的 RtMesh/实体 despawn 写面与 `stairs/structural_graph` 两张未建模表。
8. 为所有标注 U 的字段建立“字段级未决台账”，避免后续把占位值当语义。
