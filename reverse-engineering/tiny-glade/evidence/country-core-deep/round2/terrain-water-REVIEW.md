# terrain-water 域 round2 审查表（对照 DIGEST O 条目逐文件）

审查依据：`../terrain-water/DIGEST.md`（O=机器码/PDB 证实）、v2 索引、round1 反编译（decomp/，124 份）。
状态记号：**OK**=与 O 证据一致；**FIX**=有具体错误；**STUB**=占位未实现（不视为错误，但缺项必须列出）；
**LATER**=属 typed gap，当前占位可接受。不修改源码，只登记。

| file（reconstruction/country-core/src/…） | 状态 | 具体错误 | 缺失项（按 O 证据） |
|---|---|---|---|
| resources/world_raster.rs | FIX | ① `WorldRaster(pub Gap)` 整体占位；② `RasterBufferRef` 注释“调用方从 WorldRaster 按 +0x298 字节选缓冲构造”方向对，但 `WorldRaster` 本体布局已 O，应落地 | composite/scratch 双缓冲（+0x08/+0x10、+0x20/+0x28，各 0x40000B）；DirtyTiles 位图 Vec +0x30..+0x47（0x4000B=4096 u32 瓦片状态，初始 0x07954dbb）；次级 Vec +0x48..+0x5f；56B 头拷贝 +0x60..+0x97（含 parking_lot Mutex + 订阅者 Vec @+0x78/80/88）；staging 0x200B@+0x98 + 4 标志字节 +0x298..+0x29b |
| resources/world_raster_defs.rs | FIX | ① 六个 `*Raster(pub RasterBufferRef<512>)` 与 O 不符——实为 Deref 到 WorldRaster 的资源（水位本体在 `WorldRaster<512>`，prior wp7 已证）；② `AllWorldRasterWrite(pub Gap)` 占位 | 6 槽名序 PathRaster/Cobblestone1/2/3Raster/WaterRaster/GardenRaster（0x142a93ca8 区胖指针表）；AllWorldRasterRead/Write 的 SystemParam 骨架（apply/queue=new_archetype=ICF 裸 ret、init_state=0x83fe20、get_param=0x1507ce0/0x1508130）；staging SSBO type-id 家族（0x142c2e210 区常量表） |
| resources/active_raster_stroke.rs | FIX | 全 stub；`resample_and_smooth`/`index`/`index_mut` unimplemented | 0x470B 记录布局（三 Vec 列 +0x10/+0x28/+0x48、弧长 f32@+0x40、笔刷 f32@+0x60、模式字节@+0x64、blend flag u32@+0x68、计数 u32@+0x6c、双 512B 位图@+0x70）；index_mut=base+idx*0x470；start_new/add_point/last（Option::unwrap panic）/solidify（0.5 间距重采样）/resample_and_smooth=10 遍 0.25/0.5/0.25 平滑 |
| resources/raster_stroke.rs | OK | — | `smooth`（当前为空实现）的真实平滑语义 U；`points: Arc<[Vec3]>` 与 O 一致 |
| resources/brush_sizes.rs | FIX | 字段序错误：现有 `pub water: f32` 位于 +0；原布局 **path@+0（默认 0.8，clamp[0.32,1.2]）/ water@+4（2.0，[1,5]）/ garden@+8（3.0，[2,4]）** | path/garden 的 set/reset 方法与 clamp 常量（0x142ac56c0、0x1429cd128、0x14292e6f0、0x142934694） |
| resources/cursor_terrain_raycast.rs | OK | — | `update` 的输入语义（谁调用）U；布局 {tag u8@0, pos@4} O 正确 |
| resources/mouse_terrain_raycast.rs | FIX | `camera_mouse_ray` unimplemented（0x8ef1c0 有 O 反编译可恢复）；`CursorPositionSS(pub Gap)` 占位 | CameraCursorRay 布局已 O 正确；mouse_terrain_raycast 的“xz 平面 vs intersect_ray 比距取近 + closure$0 实例精化”链；CursorPositionSS 字段 |
| resources/read_mouse_depth_value.rs | STUB | 纯 Gap | MouseDepthValue={w f32@0, xyz@+8, 帧哨兵@+0x40}；read_mouse_depth_value_from_gpu（0xa1a480：SSBO 名 0x47 字符、2×float4 NaN 过滤回读） |
| resources/in_progress_curve.rs | STUB | 全 false/0 占位 | new(0xad3070)/try_add_raw_input_points(0xad3f50，FUN_141466860 验证门)/compute_final_curve(0xad41a0)；本域属 freehand 墙输入曲线（wall 域主消费） |
| resources/drawing_input_smoother.rs | FIX | 全占位 | **DrawingInputSmoother::process（0x145c350）= 弧长均匀重采样 + 最小间距门控 → Vec<(x,z)>**（O，round2 新解）；当前文件只有 RawInputSmoothed 外观 |
| resources/glade_border.rs | STUB | `is_pos_inside` 恒 true 占位 | is_pos_inside=0xad2950（跳板→is_shape_inside 0xad2970 主体）；search_lerped_shape_that_fits_into_glade（0xad2ce0）；CurveU panic "Tried to init CurveU from an invalid u: " |
| resources/rocky_terrain.rs | OK（部分） | `LoadedRockyTerrainVertex._gap: [f32;9]`（0x24B 源记录=9 f32 ✓ 尺寸巧合正确） | 顶点 0x24B→0x30B、三角形 0x10B→0x20B 的字段映射 U；RockyTerrainMesh 布局 U（typed gap 保留合理） |
| resources/terrain_height.rs | FIX | ① `copy_from_gpu(&mut self, _readback: &[u16])` 参数方向待核（O 是 16B 块差分拷贝，非整缓冲）；② sample_at_world_xz 返回 u16（当前注释未提）；③ intersect_ray 的 32 步行进+3 二分未记录 | +0x08=u16 512×512 网格；+0x50/54 min/max（默认 -2.5/24.0，copy_from_gpu 复位逻辑）；视觉实例 0x141246fb0 双线性变体；load_index_unchecked=0x1408b54c0（0x14067dd80 是 ICF 身份函数，勿用） |
| resources/terrain_rock_mask.rs | OK | — | 已 O（0x9ea800、位索引 (z*512|x)>>5、dispatch 0x2000），无需改 |
| resources/terrain_editing.rs | OK | — | HashMap 布局 + new_stroke 0x3fffff 上限 O 正确；serde Compact/Pretty/Value 胶水按项目规则复用 |
| resources/terrain_stroke.rs | OK | — | insert_or_morph_last_point O 正确（0.001/1024）；`falloff: f32` 默认 0.4 为猜测字段（U 标出即可） |
| resources/water.rs | LATER | `rg_buffer_hash() = 0` 占位（真值=name_hash，I） | RippleSourceSsbo/WaterDrawEvent/WaterEraseEvent 载荷 U（typed gap 保留）；`Water` 外观结构本身是恢复层发明（原模块无此 struct），注释已声明可接受 |
| resources/water_mask.rs / water_exclude_clutter_mask.rs | LATER | 同 rg_buffer_hash 占位 | 布局 U（name-only）；阈值 0.5/0.2、映射 1/130 O 正确 |
| resources/garden_mask.rs | LATER | 纯 Gap | GardenMaskSsbo name-only（U） |
| systems/world_raster.rs | FIX | ① `record_raster_history` 地址注释缺失/错（v2=0x957990；旧索引 0x957bf0 是 upload_world_raster_buffers_to_staging）；② draw_raster 泛型面 stub | record：variant·0x470 槽位 + 12B 元素 Vec Arc 克隆 + History::add + reset cap4；upload：脏流门 + 6 光栅 upload_to_staging（copy_to_staging selector=0）；draw_raster 状态机（start_new/add_point/resample/solidify/rasterize_stroke/copy_to_staging 链） |
| systems/terrain/mod.rs | FIX | insert_terrain_resources/add_terrain_events/add_terrain_systems 均 unimplemented（0x9457c0/0x945a20/0x9455f0 有 O 反编译） | 资源/事件注册清单（契约已有）；RtTriangleMetaSsbo Vec 扩展细节 |
| systems/terrain/delayed_terrain_changed_msg.rs | OK | 注释与 O 一致 | 函数体 unimplemented（0xaabdd0 反编译已落盘可恢复）；on_terrain_modified=`LOCK inc`（O，round2 手解） |
| systems/terrain/deferred_terrain_history.rs | STUB | — | history_record_terrain（0xac9770 反编译已落盘） |
| systems/terrain/draw_terrain_stroke.rs | OK | — | 事件转发逻辑 O 正确 |
| systems/terrain/edit_terrain_stroke.rs | STUB | — | 0x94e000 反编译已落盘（35KB）；panic 串 O |
| systems/terrain/terrain_hover_idx.rs | STUB | 注释说“读鼠标回读 SSBO（断言 0x6c）”方向对 | 0xaff720 反编译已落盘；AppMode 0x421 门、水位/距离门控细节 |
| systems/terrain/ui_draw_terrain.rs / ui_edit_terrain.rs / ui_terrain_focus_mode.rs | STUB | — | ui-input 域共享；反编译已落盘（ui_edit 0xaef1f0 33KB、focus_mode 0x9a4fb0 167KB+4 闭包） |
| systems/mesh_follow_terrain.rs | OK | 注释与 O 一致（panic 串、Query 面） | 函数体 stub；Vertex_Position 属性路径（0x142ad5120）可补 |
| systems/garden/* | STUB | draw_garden 半径“+1.5”O 正确；record_garden_history 12B 单元 O 正确 | build（0xb6d290）/draw_garden（0xae4620）/record（0x1695e10）反编译已落盘 |
| render_loop/compute_passes 相关（water/garden/rocky/highlight/mouse_read/lighting/upload/rasterize/read_terrain_data） | 未在 reconstruction/country-core/src 出现 | — | 属 frame 域（country-core-render），DIGEST 已按契约复核，无本文件可改 |

汇总：**OK 7 / STUB 7 / LATER 3 / FIX 9 / 无文件 9**。FIX 最高优先级：brush_sizes.rs（字段序）、active_raster_stroke.rs（0x470B 布局）、world_raster.rs/world_raster_defs.rs（本体布局+6 槽名序）、terrain_height.rs（u16 网格+视觉实例）、drawing_input_smoother.rs（process 语义）、systems/world_raster.rs（record 地址）、systems/terrain/mod.rs（注册三函数）。
