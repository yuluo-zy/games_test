# backend-misc 域 DIGEST（systems/backend 非 input + systems 顶层 misc）

域 agent：backend-misc（tg_bm1）。日期：2026-10-14。
范围：`country-core/src/systems/backend/**`（**排除** backend/input/**，属 ui-input 域）+ `systems/{mod,utils,utils_intersect,brush_preview}.rs`，共 16 文件。

---

## 1. 模块清单

| 文件 | 业务函数数 | 结论概览 |
|---|---|---|
| backend/configwatch.rs | 1（+助手 FUN_14093b530） | O |
| backend/flush_screen_log.rs | 3（+助手 FUN_14093fe50） | O |
| backend/game_frame_index.rs | 0 业务（全内联） | I |
| backend/grow_grass.rs | 1（+内联 is_grown） | O |
| backend/icc.rs | 2（+内联 generate_icc_lut） | O |
| backend/meshwatch.rs | 1（+助手 FUN_14092d8d0） | O |
| backend/mod.rs | 3 | O |
| backend/panic_reporter_fetch_active_session.rs | 0 业务（全内联） | I |
| backend/prepare_frame_constants.rs | 1（+助手 FUN_140b18f60） | O |
| backend/texture_watch.rs | 1（+助手 FUN_140b73a50） | O |
| backend/transient_mesh.rs | 0 业务（全内联） | I |
| backend/vao_update.rs | 2 | O |
| systems/mod.rs | 4 | O |
| systems/utils.rs | 1 | O |
| systems/utils_intersect.rs | 1 | O |
| systems/brush_preview.rs | 3（由 terrain-water 已证） | O |

共反编译落盘 `backend-misc/decomp/`：21 个业务地址 + 5 个助手地址（含 0x140a58440 跳板的目标 0x140b65a80）。

## 2. 布局表

| 结构 | 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|---|
| `GameFrameIndex`（资源） | +0x00 | 4B | 首 u32（门槛 `>1` 才写帧号） | prepare_frame_constants：`uVar44=*puVar14; if(1<uVar44)` |
| | +0x04 | 4B | 次 u32（当前帧号，写 `& 3`） | `*(frame_constants+0x108) = puVar14[1] & 3` |
| `GrassGrowthTimer`（资源） | +0x00 | 4B | elapsed f32（+dt 累加） | grow_grass `fVar67 = *param_2; *param_2 = fVar67` |
| | +0x04 | 1B | grown 标志（1=已生长） | `*(char*)(param_2+1)` 检查/置位 |
| 屏幕日志全局队列 | 0x14362bfe8 | 1B | Mutex 状态（LOCK/UNLOCK/WAKE） | flush/log_* 反编译 |
| | 0x14362bfe9 | 1B | is_processing | 同上 |
| | 0x14362bff0 | 8B | Vec cap（cap-first） | 同上 |
| | 0x14362bff8 | 8B | Vec ptr | 同上 |
| | 0x14362c000 | 8B | Vec len | 同上 |
| 队列条目 | 0x00/0x08/0x10/0x18 | 0x20B | {len, ptr, len, kind}（kind 0=Error/1=Warning） | log_error/log_warning 写入位 |
| `ScreenLog`（资源） | +0xb0 | 8B | 槽计数（5 槽环 `(n+1)%5`） | FUN_14093fe50 |
| 每槽 | +0x28 | 4B | 颜色 u32（红 0xff0000ff / 黄 0xffff00ff） | 颜色表 0x142aa159c |
| `FrameConstants` | +0x108 | 4B | 帧号 `idx & 3` | prepare_frame_constants |
| TAA jitter 表 | 0x142ae4fc0 | 4B×n | {0,3,1,2,4,...} | rne-read --hex |
| `GameSystemSet` | — | 1B | 枚举判别（dyn_clone 分配 1 字节） | 0x140a90e20 |

## 3. 方法/系统行为（O / I / U）

全部系统的 ECS 参数面按 ResMut tick 回写（`*(param+0x1c)`）与 Res 解引用判别。

- **configwatch** 0x14085b1c0 **O**：锁 ConfigWatch → 日志 `'Configwatch: detected changes: N'`（门槛 `全局>2`）→ `FUN_14093b530` 重载 ColorLibrary/SSBO → 清理。panic `Result::unwrap`（行 43）。
- **flush_screen_log**（3 函数）**O**：全局队列 `Mutex<Vec<(String,LogType)>>`；`log_error_to_screen`/`log_warning_to_screen` 压入（kind 0/1）；`flush_error_log_to_screen(mut screen_log: ResMut<ScreenLog>)` 排空并 `FUN_14093fe50` 逐条着色写入 5 槽环。
- **game_frame_index** **I**（全内联）：帧推进并入调用点；资源被读为 2×u32（门槛+帧号）。
- **grow_grass** 0x14172fe80 **O**：Timer{f32,u8}；`elapsed+=dt`；>300.0s 后解码 15B 静态（XOR）→ 64B 哈希 memcmp → EventWriter 发 8B 事件 → 置 grown；含 CloseHandle（后台线程等待）。
- **icc**：build 0x140a58440（跳板→0x140b65a80，App 调度注册，Schedules 缺失 panic）**O**；`update_window_icc_profile` 0x14093d830 **O**（lcms2-5.6.0 open/transform，生成 LUT 写 ActiveIccProfile/SSBO，colorspace≠0xb panic）；`generate_icc_lut` 内联 **I**。
- **meshwatch** 0x14085b4b0 **O**：锁 MeshWatch → 日志变更数 → 位图迭代 → `AssetMeshLibrary::reload_mesh`（FUN_14092d8d0）。
- **backend/mod.rs**：`add_backend_systems` 0x140942c80 **O**（注册 ~7 系统，GameSystemSet 变体 1/2/3/5/10/0x2c，条件系统 config+0xe7）；`insert_backend_resources` 0x140942f80 **O**（3 分支状态机）；`add_backend_events` 0x140943660 **O**（6 类型注册）。
- **panic_reporter_fetch_active_session** **I**（全内联）。
- **prepare_frame_constants** 0x140af40b0 **O**：`frame_constants+0x108 = idx & 3`；TAA jitter 全局门控 + `%table_len` 采样；4×ViewConstants（FUN_140b18f60）+ 800B 帧常量块。
- **texture_watch** 0x140aab370 **O**：锁 TextureWatch → 日志 `'Texture watch: detected changes: N'` / `'Error processing textures: '` → 位图迭代 → FUN_140b73a50 重载。
- **transient_mesh** **I**（全内联）：`TransientMeshRegistry::drain_meshes_to_delete` 消费闭包。
- **vao_update**：`build_missing_vaos` 0x1408646b0 **O**；`rebuild_vaos` 0x140864bf0 **O**。
- **systems/mod.rs**：`is_gameplay_active` 0x140a90e00 **O**（两 Res 判 `[rcx]+0x30==1 && [rdx]+0==1`，资源身份 U）；`dyn_clone`/`as_dyn_eq`/`dyn_hash` **O**（1 字节判别）。
- **utils::add_stages** 0x140945ed0 **O**：`enum_iterator::All<GameSystemSet>` → `App::configure_sets(Update)`。
- **utils_intersect::line_x_shape_overlap** 0x14175ada0 **O**：圆/矩形射线求交 + t∈[0,1] 夹取 + fmod 回绕；返回 `{hit, t1, t2}`。
- **brush_preview** 3 函数由 terrain-water 已证 **O**（本章节确认保留）。

## 4. panic/日志串清单

| 地址 | 文本 | 使用函数 |
|---|---|---|
| 0x142a949f0 | `called `Result::unwrap()` on an `Err` value` | configwatch/meshwatch（+0x2b 行） |
| 0x142a95080 | `Configwatch: detected changes: ` | configwatch |
| 0x142a95048/0x142a95090 | `crates/country-core/src/systems/backend/configwatch.rs` | configwatch |
| 0x142a95140 | `Meshwatch: detected changes: ` | meshwatch |
| 0x142a95168 | `...meshwatch` + `cloud/cloud.raster.hlsl` `cloud_shader` | meshwatch |
| 0x142aa03b8 | `called `Result::unwrap()` on an `Err` value` | flush_screen_log（+0x2b 行） |
| 0x142aa1320/1338/1368 | `crates/country-core/src/systems/backend/flush_screen_log.rs` | flush_screen_log |
| 0x142a96490 | `Couldn't build VAO: ` | build_missing_vaos |
| 0x142a96508 | `Mesh is empty. Skip building VAO` | build_missing_vaos |
| 0x142a96568 | `WARNING: Tried rebuilding a mesh that has no VAOs!` | rebuild_vaos |
| 0x142a965c0 | `Couldn't rebuild VAO: ` | rebuild_vaos |
| 0x142aafd88 | `Asset mesh library couldnt reload a mesh: ` | mesh_library（FUN_14092d8d0） |
| 0x142aafe20 | `Asset Mesh library was requested to reload a non-existing mesh path ` | mesh_library |
| 0x142add810 区段 | `Texture watch: detected ch...` | texture_watch |
| 0x142add878 | `Error processing textures: ` | texture_watch |
| 0x142aa53ed | `assertion failed: (0.0..=1.0).contains(&t)`（源 utils/curve/curve_u.rs:42） | line_x_shape_overlap |
| 0x142af2cc0 | bevy `SchedulesRequested resource does not exist in the World...` | icc build |
| 0x142ab2130 | `Failed creating an ICC LUT: ` | update_window_icc_profile |
| 0x142ab2158 | `terrain_1 terrain_2 wall_moss wall_bricks terrain_highlight night_tint` + `... entry is missing from ColorLibrarySSBO` | generate_icc_lut |
| 0x142ab1708/17b0 | lcms2-5.6.0 `profile.rs`/`transform.rs` | icc |

## 5. 与现有重构源码的差异（代码写 X，证据为 Y）

1. `backend/flush_screen_log.rs`：重构 `log_error_to_screen` 用 `log::error!("{}", _msg)` —— **证据为压入全局屏幕日志队列**（无 log 框架调用）。`flush_error_log_to_screen(_world: &mut World)` —— **证据为 `ResMut<ScreenLog>` 单参系统**。且函数名虽为 flush_error 实际排空全部 error+warning。
2. `backend/grow_grass.rs`：重构 `GrassGrowthTimer(pub f64)` + `is_grown = self.0 > 0.0` —— **证据为 `{f32 elapsed@0, u8 grown@4}`**；`is_grown` 检查 flag@4。
3. `systems/mod.rs is_gameplay_active(_world: &mut World)` —— **证据为两个 Res 参数**（`[rcx]→+0x30` 与 `[rdx]→0` 双状态判断）。
4. `systems/utils_intersect.rs line_x_shape_overlap(...) -> bool` —— **证据返回 `{hit: bool, t1: f32, t2: f32}`**（Option<RayBoxIntersection> 类），且签名含额外 `param_3`（射线源偏移）与 `param_6`（f32）。
5. `backend/game_frame_index.rs GameFrameIndex(pub u32)` —— 证据读 2×u32（门槛+帧号），单字段版本不完整（**I**，待精确布局确认）。
6. 重构 `prepare_frame_constants` 帧号 `& 3` 与 TAA jitter `%table_len` 描述 —— **完全证实**（frame_constants+0x108 写位、全局门控、jitter 表）。

## 6. 跨领域线索

- `configwatch` → `FUN_14093b530` 重载 **asset_libraries::{color_library(colors.json), ssbo_library}**（audio-misc 域资源）。
- `meshwatch` → `FUN_14092d8d0` = **asset_libraries::mesh_library::reload_mesh**（load JSON）。
- `texture_watch` → **startup/load_textures** 装载路径；`FUN_140b73a50` 纹理位图树。
- `icc::generate_icc_lut` → **ColorLibrarySSBO** + lcms2 5.6.0（registry 路径 `/home/h3/.cargo/registry/.../lcms2-5.6.0`，与 utils crate fork 同源）。
- `flush_screen_log` → `ScreenLog` 资源供 **UI 文本渲染**（ui-input 域消费者待查）。
- `vao_update` → **asset_libraries::vao_library**（AssetVAOLibrary::add/rebuild_vao）。
- `panic_reporter_fetch_active_session` → **feedback_report_resources**（audio-misc 域）。
- `grow_grass` 8B 事件 → 消费者未定位（可能 collision-eco / 视觉域，**U**）。
- `systems/mod.rs GameSystemSet` / `utils::add_stages configure_sets(Update)` → **全局调度接线**（跨全部系统域）。
- `prepare_frame_constants` → **render::frame_constants/view_constants**（render 域类型）。

## 7. 错位/纠错

- **sysres-procedures-v3.json 索引错位**：`backend/mod.rs::insert_backend_resources::closure$0` 被映射到 0x140943660，但该地址反编译行为 = `add_backend_events`（6 类型注册），与 SOURCE_MAP（add_backend_events=0x140943660）一致。closure$0 实际地址未定位。
- **icc build 0x140a58440** 为 2 指令尾调用跳板（→0x140b65a80），不是独立注册体；RE-MCP `decompile_function` 对其返回 NotFound（Ghidra 未建函数），机器码手解确认。
- **flush_error_log_to_screen 命名误导**：处理错误+警告两类。
- grow_grass 的 `GrassGrowthTimer` 重构（f64）与 `is_grown` 语义（time>0）均为重构阶段推断，**与机器码不符**，需按 `{f32, u8}` 修正。

## 交付物

- 章节：`chapters/systems/backend/*.md`（12）+ `chapters/systems/{mod,utils,utils_intersect,brush_preview}.md`
- 反编译：`backend-misc/decomp/decomp-*.txt`（26 个）
- 本 DIGEST + `backend-misc/HIGHLIGHTS.md`
