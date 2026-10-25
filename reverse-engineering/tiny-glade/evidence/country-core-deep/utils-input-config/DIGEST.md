# utils / input / key-bindings / profile / meta-commander 域 DIGEST（utils-input-config）

> 章节化挖掘交付（dbid `tg_ui2`，2026-10-14）。
> 域：`utils/{audio_utils,child_wall,debug_viz,dirty_tiles,geo,load_json,mod,project_cursor_onto_ray}`、
> `input/{action_set,input_stream,mod,panic_repro_state}`、`game_key_bindings.rs`、
> `game_profile.rs`、`meta_commander.rs`。
> 规则：文档与证据 only，不改 reconstruction 源码。机器码为准，PDB 名只作线索。

---

## 1. 模块清单（15 章文件 + 本 DIGEST）

| 文件 | 业务过程数 | 判定覆盖（表格行） |
|---|---|---|
| `utils/dirty_tiles.rs` | 27 | O 15（rasterize/iter 多泛型实例同地址合并行） |
| `utils/debug_viz.rs` | 17 | O 17 |
| `utils/load_json.rs` | 5 | O 5 |
| `utils/geo.rs` | 6 | O 6 |
| `utils/audio_utils.rs` | 4 | O 4 |
| `utils/project_cursor_onto_ray.rs` | 3 | O 2 / I 1 |
| `utils/mod.rs` | 5 | O 5 |
| `utils/child_wall.rs` | 1 | O 1 |
| `input/action_set.rs` | 13 | O 12 / I 1（reconstruct） |
| `input/input_stream.rs` | 10 | O 9 / I 1（try_load） |
| `input/mod.rs` | 4 | O 3 / I 1（mode） |
| `input/panic_repro_state.rs` | 1 | O 1 |
| `game_key_bindings.rs` | 10 | O 9 / I 1（assign_key 交换体） |
| `game_profile.rs` | 6 | O 6 |
| `meta_commander.rs` | 9 | O 9 |
| **合计** | **121 业务（SOURCE_MAP）** | **表格行 O 104 / I 5 / U 0** |

反编译落盘：`utils-input-config/decomp/decomp-<va>.txt`（本域新取 16 个，其中 6 个 NotFound
改由 `rne-read.py --hex` 原字节手解）；既有证据复用 `expansion-b1/b2/decomp/`。

**与共享 `utils` 重复判定**：本域 8 个 utils 文件**全部为 country-core 特有**，不与
`reconstruction/shared-foundation/crates/utils` 重复（共享 utils 含 `Aabb3/Rectangle2d/NameHash/
float_grid/IndexCondenser/Transform/geometry/rdp/swizzlers/comparative_space/curve/camera`，均不在本域清单）。
其中 `load_json_as_mesh` 共享 utils **没有**（RENDER_GAP_ANALYSIS §4.1 已确认），68 调用方均依赖 country-core 自有实现 → 重点挖（已证实自定义 JSON 网格解析器，见 §3.3）。
共享 utils 里同名的 `geometry`/`float_grid` 等由其他域/worker 处理，不在本域。

---

## 2. 布局表（资源/结构 = 偏移、大小、含义、证据）

### 2.1 `DirtyTiles`（utils/dirty_tiles.rs）—— 64×64 位图
- **大小 0x200 = 512 B = 64×u64**。`clear` @0x140960c20 / `new_clear` @0x140960ac0 均 `memset(0x200)`（原字节：`mov r8d,0x200; xor edx,edx; jmp memset`）。
- **布局**：`bits[y]` = 第 y 行字，第 x 位 = 位 x（`iter_world_space_rect_cells` 循环写
  `word[y] |= 1 << (x & 0x3f)`，@0x140ade8e0）。
- **单元大小 = 32 世界单位**：`rasterize_world_space_stroke` @0x140ade620 用常量
  `{0.49230769f32 (=32/65), 0.0003846154f32 (=1/2600), 32.0f32}` @0x142925874 做世界→cell
  缩放；64 格 × 32 单位 = 2048×2048 地图范围。
- `cell_from_world_space` @0x140960b20：两路 `fmaf`（`e8 8f 33 fc 01` / `e8 7c 33 fc 01`）+
  `cvttss2si` 截断转换，`0x7fffffff` 为越界哨兵，`< 0x40` 判定有效（与重建注释一致）。
- `extend` @0x140960c30：32×`movups + orps` 向量 OR 合并（即 tabloid `apply_to` 的位面语义）。
- `impl$2::apply_to` @0x140960ef0：`movups [rcx] 与 [rdx] 逐 16B orps` —— 与 tabloid
  `StateDelta::apply_to(state) = state.bits[i] |= delta.bits[i]` 完全一致（对照
  `shared-foundation/crates/tabloid/tests/dirty_tiles.rs`）。**→ DirtyTiles 同时是 tabloid 的
  `State` 与 `StateDelta`，`apply_to` 即 OR 累积**。
- `impl$3::iter_dirty_tiles` @0x141458b20/0x141458a50：遍历脏格（wall 域
  `publish_public_wall_state::gather_lazy_update_sources` 订阅者用）。

### 2.2 `InputFrame` / `InputStreamSyncPoint` / `InputStream`（input/input_stream.rs）
- `InputStream` = 64 B：`{ sync_points: VecDeque<InputStreamSyncPoint> (cap@0x00, ptr@0x08,
  head@0x10, len@0x18), frames: VecDeque<InputFrame> (cap@0x20, ptr@0x28, head@0x30,
  len@0x38) }`（cap-first）。证据：`sync_point_count` @0x140949170 = `[rcx+0x18]`；
  `frame_count` @0x140949180 = `[rcx+0x38]`；`last_frame_mut` @0x140949480 = `((count+head-1) -
  ((count+head-1)>=cap?cap:0))*0xd0 + ptr`。
- `InputFrame` stride **0xd0 (208 B)**；内含 `Vec<[u8;12]>`（`add_frame` 释放路径
  `dealloc(slot+8, n*0xc, 4)` = Vec<[u8;12]>）+ `Vec<u32>`（`dealloc(slot+0x20, m<<2, 4)`）。
  **与 `MainActionSet::deconstruct` 的 0xc 字节动作记录完全一致** → 每帧输入=打包动作集。
- `InputStreamSyncPoint` stride **0x60 (96 B)**：`{ InputTranslationState (88 B = 11 qwords),
  frame_index @+0x58 }`（`on_history_sync` @0x1409494c0 写 11 qwords + 帧数）。
- `InputTranslationState` = 88 B（11 qwords），为回放同步快照。
- `InputRecordReplayState` = 80 B：`{ stream: InputStream (64 B), mode @0x40 (u64), index @0x48
  (i64, 载入时 -1) }`（`try_load` @0x1409499c0）。`InputRecordReplayMode` 5 变体（Display fmt
  @0x14094f2d0）：case0 `AutoRecordPanicRepro`、case1 `Records…`(带 dst_path)、case2
  `Replay…`、case3 `DisableInput`、case4 `None`。
- 快照 JSON 序列化字段（`0x1413fe6f0` Pretty）：`translation_state`(+0x00)、
  `prev_normalized_cursor_position`(+0xc0)、`time_delta_seconds`(+0xc8)、
  `main_action_set_value`(+0x68)；文件形式 `{replay_initial_state, first_frame}`（0x1402e0060）。

### 2.3 `KeyBindings`（game_key_bindings.rs）
- `KeyBindings::default` @0x14089b110（原字节）：**40 个动作槽，12 B 步距**，从偏移 0 起写
  u32 默认键码；随后 `[0x1b0]=0xc3`(u32)，`[0x1bc..0x1c8]` 三个 u16 鼠标键 `{1,2,1}`。
- 默认 36 键序列（原字节）：
  `0x2a,0x26,0x14,0x17,0x18,0x24, 0xc3,0xc3,0xc3,0xc3, 0x07,0x08,0x09,0x0a,0x0b,0x0c,
  0x0d,0x0e,0x0f, 0x23,0x27,0x2d,0x2b, 0xc3,0x38,0xab,0xa5,0x1b,0x49,0x3a,0x51,0x52,0x50,
  0x53,0x4f,0x4e` —— 后 26 与重建一致，前 10（`0x2a..0x24` 及三个 0xc3）为重建缺失。
- **字段名表（ron serialize @0x140831740，stride 0xc）**：move_forward@0, move_back@0xc,
  move_left@0x18, move_right@0x24, move_up@0x30, move_down@0x3c, rotate_left@0x48,
  rotate_right@0x54, zoom_in@0x60, zoom_out@0x6c, building@0x78, path@0x84, decorators@0x90,
  customize@0x9c, eraser@0xa8, terrain@0xb4, garden@0xc0, tree@0xcc, water@0xd8, photomode@0xe4,
  time_of_day@0xf0, undo@0xfc, redo@0x108, focus_edit@0x114, disable_snapping@0x120,
  screenshot@0x12c, feedback@0x138, hide_ui_photomode@0x144, ui_cancel@0x150, ui_confirm@0x15c,
  ui_left@0x168, ui_right@0x174, ui_down@0x180, ui_up@0x18c, ui_tab_prev@0x198, ui_tab_next@0x1a4,
  experimental_camera@0x1b0, mouse_rotate@0x1bc, mouse_pan@0x1c0, mouse_focus_edit@0x1c4。
- `assign_key` @0x14089b2a0（73KB）：26 个展开块逐槽扫描；命中则清为 0xc3（unset），
  冲突交换；鼠标冲突按按钮跳表 `UNK_142a9a730 + btn*4`；派发 0x98B 项变更事件。
- `action_desc` @0x14089cb60：44 分支 switch，标签
  `MoveUp/MoveDown/Move/Rotate/ZoomIn/ZoomOut/Undo/Redo/FocusOrDeselect/DisableSnapping/
  TakeScreenshot/SendFeedback/HideUiInPhotoMode` + toolbar 图标 + `UI Cancel/Confirm/Left/
  Right/Down/Up/TabPrev/TabNext/Snap…`。
- `mouse_button_to_string` @0x14089c970：12 项跳表 → `LMB/MMB/RMB` 等。

### 2.4 `GameProfile`（game_profile.rs）
- **大小 0x208 B**（`load_game_profile` 命令 payload 0x210 = 8 头 + 0x208）。
- **ron 字段（deserialize/serialize @0x1404b0ed0/@0x140a658a0）**：`player_id`、
  `game_installation_id`、`tutorial`、`tutorial_cooldowns`、`key_bindings`、
  `new_glade_count`；struct 名 `GameProfile`。
- `PlayerId` = `uuid::Uuid::new_v4()`（`load` 默认回退经 `FUN_140887a50`）。

### 2.5 `MetaCommander` / `MetaCommand` / `MetaCommandFence`（meta_commander.rs）
- `MetaCommander` = **80 B = 10 qwords**（`new` @0x14091eff0）：`commands: VecDeque<MetaCommand>`
  (4 qwords @0..0x1f, cap-first) + profiler_sinks HashMap 区（4 qwords @0x20..0x3f，来自
  静态 `_UNK_142aadc40..58`）+ RandomState（2 qwords @0x40..0x4f，种子计数 `FUN_140d4d740`）。
- `MetaCommand` element = **0x38 B**：3×16B + 1×8B（`consume` @0x14091f100 用
  `movups xmm0/xmm1/xmm2` + `mov qword` 拷贝）。空哨兵 `0x8000000000000000`。
- `commands` VecDeque 布局 = cap-first `{cap@0x00, ptr@0x08, head@0x10, len@0x18}`：
  `peek` @0x14091f0c0 = `front()` = `ptr + (head - (head>=cap?cap:0))*0x38`（len==0 返 NULL）；
  `consume` = `pop_front()`（head 环绕进位 + len-1）。
- `MetaCommandFence` = `Arc<Mutex<(bool,bool)>>` 两份（`new_pair_blocked` @0x14091ef90 分配 0x18 B
  = Arc header 16B + `(bool,bool)`，strong=2）；两标志均 true = blocked。
- `is_profiling` @0x14091f0f0 = `[self+0x38] != 0`。

### 2.6 `AudioLoop` / `AudioLoopDesc`（utils/audio_utils.rs + resources/audio_resources.rs）
- `AudioLoop` ≈ 0x38 B 自包含游标结构（**非 Entity 非 Resource**，挂在 `AudioCore` resource 下）：
  `{ entries: Vec<AudioLoopEntry (48 B)> (ptr@+0x08, len@+0x10), start: Instant @+0x18,
  unset 哨兵 1e9 @+0x20, cursor @+0x28, elapsed 阈值 @+0x30, flags @+0x34/+0x35/+0x36 }`。
- 驱动 **kira** `set_parameter`（参数名 `"loop"` @0x142ad2f20）；失败 `warn!("Failed to set
  parameter named {name}")`（@0x140a57670 编译助手，串 @0x142ad2cc8）。
- `AudioLoopDesc`：`{ path: String }`。

### 2.7 `ProportionalScreenSpace`（utils/project_cursor_onto_ray.rs）
- `from_window_size` @0x140ac8420（原字节）= 16 B：`{ size: Vec2 (w,h), aspect: f32 (w/h) @+0x08,
  pad 1.0f @+0x0c }`。重建只记 `{size}` → 修正。
- `ProportionalScreenSpaceCoord::project_onto_ray` @0x140ac84a0：近/远点经两个 4×4 矩阵变换 +
  透视除 → NDC → 屏幕线，`t = ((cursor-near)·dir)/|dir|²`，clamp 到 PSS 范围后 lerp 回世界点。

### 2.8 其它
- `ActiveInputModeTracker` = 0x20 B `{u64 @0, u32 @8, u64 @0x10, u32 @0x18}`；
  `mode()` @0x140958f30 比较两半返回判别；`ActiveInputMode::is_controller` @0x1408c5680 =
  直接返回判别字节（2 变体，Controller=1）。
- `PanicReproState::is_inactive` @0x140833230 = `byte[0]==0`。
- `DebugSphere` 布局（`new` @0x140b02490 原字节）：`[0x00]=0, [0x10]=radius=0.1f(0x3dcccccd),
  [0x14..0x20]=center (x,y,z), [0x20]=1.0f`（重建字段顺序 `{center,radius,color,solid}` 与实际
  布局不一致，见 §7）。
- 事件注册表 `EventRegistry`（utils/mod.rs）：自定义 **TypeId→Events<T>** HashMap
  （hasher@+0x38, buckets@+0x1e0, len@+0x210, cap@+0x218, tick@+0x5b0），**非 Bevy Events**。

---

## 3. 方法/系统行为（逐函数 O/I/U）

### 3.1 utils/dirty_tiles.rs（27 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `new_clear` | 0x140960ac0 | **O** | memset(self,0,0x200)（原字节 tail-jmp memset） |
| `rasterize_world_space_stroke<…×9>` | 0x140ade620 | **O** | 逐点对世界→cell 缩放（0x142925874 常量），`FUN_140b1e460` 保守线光栅（复用 `conservative_line_raster`） |
| `iter_world_space_rect_cells<rasterize>` | 0x140ade8e0 | **O** | 遍历 [min,max] cell 范围，写 `word[y]|=1<<x`；越界 panic（UNK_142ae2060） |
| `iter_world_space_rect_cells<intersects>` | 0x140adec50 | **O** | 同上，闭包置 bool |
| `iter_world_space_rect_cells<autoclutter/bush ×4>` | 0x141cf3540/3900/3c00/3ee0 | **O** | 同迭代器，system_clutter 订阅 |
| `rasterize_world_space_rect` | 0x140960ae0 | **O** | tail-jmp 到 rasterize 闭包变体 |
| `intersects_world_space_rect` | 0x140960af0 | **O** | 栈 bool + 闭包变体，返回 byte |
| `cell_from_world_space` | 0x140960b20 | **O** | fmaf 缩放 + cvttss2si + `<0x40` 有效 + `0x7fffffff` 哨兵 |
| `clear` | 0x140960c20 | **O** | memset 0x200 |
| `iter_tiles<wall ×2>` | 0x140adeea0 / 0x140adf4b0 | **O** | 逐格闭包（wall_state gather_lazy_update_sources） |
| `iter_tiles<stairs>` | 0x1412945c0 | **O** | 同上（decorator_visual stairs） |
| `retain<copy_to_staging>` | 0x140adfcb0 | **O** | 逐格谓词保留（world_raster copy_to_staging 订阅） |
| `extend` | 0x140960c30 | **O** | 向量 OR 合并 32×16B |
| `impl$2::apply_to` | 0x140960ef0 | **O** | tabloid `StateDelta::apply_to`（OR 累积） |
| `impl$3::iter_dirty_tiles<wall ×2>` | 0x141458b20 / 0x141458a50 | **O** | 脏 delta 遍历（gather_lazy_update_sources） |
| `iter_world_space_stroke_cells` | inline | **O** | 逐位展开（重建实现合理） |
| `iter_tiles` / `set` / `is_set` / `bitmap_set` / `bitmap_intersects` / `dilate` | inline | **I** | 名称+内联宿主；位图语义与 `set/is_set` 机器码一致，`dilate` 无独立符号 |

### 3.2 utils/debug_viz.rs（17 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `DebugSphere::new` | 0x140b02490 | **O** | 构造，radius=0.1f@0x10，center@0x14，alpha@0x20=1.0（原字节） |
| `DebugSphere::{color,radius,solid}` | 0x140b024c0/e0/00 | **O** | builder（原字节短函数） |
| `DebugSphere::show` | 0x140b02520 | **O** | 构 box + Commands 队列写（FUN_14146c880） |
| `DebugRectangle3d::{new,color,line_width,dashed_line}` | 0x140b02e10/40/70/a0 | **O** | builder 族（默认线宽 1.0） |
| `DebugRectangle3d::show` | 0x140b02ed0 | **O** | 4 角点（FUN_140c99600）+ 点变换（FUN_140c9cde0）+ 边长@box+0x3c + fmt（FUN_140d79f90）+ Commands 写 |
| `DebugRectangle2d::show` | 0x140b04640 | **O** | 4 f32 角点（FUN_140c991f0）+ box 0x48 + fmt + Commands 写 |
| `DebugLine::{new,color,set_ui_pass,dashed,line_width}` | 0x140b04e70/b0/ee0/f10/f40 | **O** | builder 族 |
| `DebugLine::show` | 0x140b04f70 | **O** | 折线总长（逐段 SQRT 累加）+ box 0x50 + 判别跳表 `UNK_142ae731c` + Commands 写 |
| `impl$6/$7::fmt` | 0x140b05b60 / 0x140b05c70 | **O** | Debug fmt 胶水 |

### 3.3 utils/load_json.rs（5 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `load_json_as_mesh<&str>` | 0x140b0eeb0 | **O** | 读串 → 自定义 JSON 解析（FUN_141405930，非 serde_json，断言 `values.array_length() as i32 > max_index`）→ 取 `indicesbuffer/buffer`(u32 索引) + `attributes` → Mesh。panic `Couldn't find  ` / `Couldn't parse  `；日志 `Reading mesh ` |
| `load_json_as_mesh<PathBuf>` | 0x140b0fe60 | **O** | 同（`fs::read_to_string` FUN_140d59140；缺失 panic） |
| `load_json_as_mesh<String>` | 0x140b10ed0 | **O** | 同（消费并释放 String） |
| `load_json_as_mesh<&PathBuf>` | 0x140b11f50 | **O** | 同（借用） |
| `read_attribute_from_json` | 0x14083e210 | **O** | 按名取 map，读 `type` 键，格式化描述符匹配 `["int",1]`/`["float",1..4]` → tag 0..4；数值数组转 f32；panic `load_json_as_mesh: unrecognized attrib '` |

### 3.4 utils/geo.rs（6 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `update_curve_mesh<IntoIter/Copied>` | 0x14157eac0 / 0x14157f130 | **O** | 曲线点 → 曲线网格（推 `Vertex_Position`/`Vertex_Color` 属性 + FUN_1412f7260 ribbon 生成 + u32 索引）；null 输出 panic `MEOW####` |
| `update_curve_mesh<circle/rectangle 闭包>` | 0x14157eda0 / 0x14157ef70 | **O** | 同上（wall 域 signifier 用 Vec2→Vec3 闭包） |
| `rewire_enclosed_curve` | 0x140a6ba50 | **O** | 阈值==0/哨兵 → 空；`FUN_141466bb0` 曲线切片；端点 ε 去重（`_UNK_142925984`）；`FUN_141469430` 重建；用私有 `utils::curve` crate（panic 源 `…/crates/utils/src/curve/mod.rs`） |
| `mesh_from_pts` | 0x140a6c120 | **O** | `pts_len>0` 断言；拷贝 Vec3；`Vertex_Position`/`Vertex_Color`；u32 顺序索引（SIMD 常量 0,1,2,3/4,4,4,4/8,8,8,8）；输出 `{count, indices_ptr, count, 哨兵 0x8000000000000000, attributes…}` |

### 3.5 utils/audio_utils.rs（4 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `AudioLoop::stop` | 0x140a59430 | **O** | 计时门控；首调记录 `Instant::now()`（哨兵 1e9），否则 elapsed≤阈值 早退；游标遍历 48B entries 检查 playing flag；kira `set_parameter("loop", …)` 失败 warn |
| `AudioLoop::start_if_not_playing` | 0x140a596b0 | **O** | 幂等启动；重置哨兵；flag@+0x34 早退；遍历 entries + kira 增益 `_UNK_142925904` |
| `AudioCore::instantiate_loop` | 0x140a59220 | **O** | 由 AudioLoopDesc 构造（name fat-ptr @+16, u32 @+24）；entries Vec 初始化；失败返 None 哨兵；内部 `.unwrap()` panic |
| `AudioLoop::current` | 0x140a593f0 | **O** | `&entries[cursor]`（cursor*0x30 + base@+8），越界 panic（audio_utils.rs） |

### 3.6 utils/project_cursor_onto_ray.rs（3 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `from_window_size` | 0x140ac8420 | **O** | 16B 结构 {size, aspect, 1.0}（原字节） |
| `transform_screen_space` | 0x140ac8450 | **I** | PSS 3 f32 {min,max,scale} 重映射；helper `func_0x000140c906d0` 语义未吃透 |
| `project_onto_ray` | 0x140ac84a0 | **O** | 矩阵变换 → NDC → t = 点积投影 → clamp → lerp 世界点；f32::clamp panic（`/rustc/…/core/src/num/f32.rs`） |

### 3.7 utils/mod.rs（5 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `get_last_event<T>×3` | 0x1402b4910/4aa0/4c40 | **O** | 自定义 EventRegistry（TypeId→Events）取最后一条（hash `0x6ecb4684a4c9e1f9` 等）；未注册 panic `Event not registered: {TypeName}`（UNK_1429ae260） |
| `drain_events<OsWindowCmd>` | 0x1402b47f0 | **O** | 同注册表，排空入 Vec |
| `world_space_to_screen_space` | 0x140ac8780 | **O** | 4×4 矩阵变换（world 乘 VP 列）+ 透视除 + 屏幕缩放（原字节） |
| `get_res`/`get_res_mut`/`is_inside_map_bounds`/`convert_function_expr_to_title` | inline | **I** | 名称推断 |

### 3.8 utils/child_wall.rs（1 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `ChildWall::slice` | 0x140a24000 | **O** | 读 a.y@+8 / a.z@+0xc，两值分别 `f32::clamp` 入 [min,max]，越界/NaN panic（UNK_142aa53c0/UNK_142ac9aa0/ab8，f32::clamp）；返回 `{param_3, t1, t2}`。**重建签名 `(ChildWall,ChildWall)` 需复核** |

### 3.9 input/action_set.rs（13 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `deserialize::visit_str` | 0x1416f92d0 | **O** | 46 动作名 → 变体索引（0..0x2c，0x2e=unknown）；SIMD 16B 常量比较 `UNK_142c91850..`；`clic`+`k`→click、`camera_p`+`an`→camera_pan 等；fallback memcmp |
| `serialize (Compact)` | 0x1400ed690 | **O** | serde_json 对象，逐非默认字段写 46 键（`input_space_cursor_move`…`camera_*`、`tool_*`、`ui_*` 等） |
| `serialize (Pretty)` | 0x1416f8680 | **O** | 同上 + 缩进处理 |
| `MainActionSet::just_pressed` | 0x140946620 | **O** | per-action just_pressed 汇聚（原字节短函数） |
| `MainActionSet::deconstruct` | 0x140946b80 | **O** | 非默认动作打包为 `Vec<[u8;12]>`：u16 tag（`slot<<8 | kind`：kind 0 数字区 tags 0x0..0x500，kind 1 键槽区 tags 0x1..0x2701）+ 8B 载荷；6 数字态 + 40 键槽；`Vec{ptr,len,cap}` 扩容 FUN_140562cf0 |
| `MainActionSet::reconstruct` | 0x140947c80 | **I** | 逆向；0xc 步距记录 → switch tag → 写 92×i32（0x170B 结构）；损坏控制流（case 0x19/0x20 halt）；空路径置 92 槽 = 4(None) |
| `ActionSetAction::just_pressed/pressed/as_analog` | 0x1409487b0/e0/800 | **O** | tag byte + 内态 byte + 子判别；Analog 分支 `movsd [rdx+4]` 取 Vec2，Option disc 0/1（原字节） |
| `DigitalActionState::{pressed,released,just_pressed,just_released}` | 0x14093fa70/80/90/c0 | **O** | `pressed=idx<3`、`released=(idx-3)<2`、`just_released=idx==3`、`just_pressed`=跳表（原字节；**变体序/数目与重建 4 态不一致**，见 §7） |

### 3.10 input/input_stream.rs（10 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `deserialize::visit_str` | 0x1413fe800 | **O** | serde_json 变体名匹配（输入流 JSON） |
| `serialize (Compact)` | 0x1402e0060 | **O** | `{replay_initial_state, first_frame}` 两键 |
| `serialize (Pretty)` | 0x1413fe6f0 | **O** | `{translation_state, prev_normalized_cursor_position, time_delta_seconds, main_action_set_value}` |
| `sync_point_count` | 0x140949170 | **O** | `[+0x18]` |
| `frame_count` | 0x140949180 | **O** | `[+0x38]` |
| `add_frame` | 0x140949190 | **O** | 录帧（0xd0 B）入环；recording 且 ≥9999：sync<2 且 ≥50000 直接删最老帧（释放内层 Vec），否则 `drop_oldest_sync_point`；`((head+len)%cap)*0xd0+ptr` 尾插；hash 限频日志 `Input stream frame count: ` |
| `last_frame_mut` | 0x140949480 | **O** | 末尾帧指针/None |
| `on_history_sync` | 0x1409494c0 | **O** | 非 recording 截断两队列；sync≥9 或 frames≥10000 删最老同步点；push 0x60 B 同步点（11 qwords 翻译态 + 帧数） |
| `drop_oldest_sync_point` | 0x140949660 | **O** | pop 最老同步点，删到其帧索引，重基剩余同步点 frame_index；panic `assertion failed: self.frames.pop_front().is_some()` |
| `try_load` | 0x1409499c0 | **I** | 反序列化进 80 B 状态；成功写 8 qwords + 哨兵；失败 Err(-0x8000000000000000) |

### 3.11 input/mod.rs + panic_repro_state.rs（5 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `new_default_controller` | 0x140958eb0 | **O** | 构造 0x20 B tracker `{u64,u32,u64,u32}`；常量 `0x0ee6b280`(2.5e8) |
| `new_default_mouse_keyboard` | 0x140958ef0 | **O** | 同构（默认鼠标键盘） |
| `mode` | 0x140958f30 | **I** | 比较两半 + `0xff` 哨兵 → 判别（原字节） |
| `ActiveInputMode::is_controller` | 0x1408c5680 | **O** | 返回判别字节（Controller=1）（原字节） |
| `PanicReproState::is_inactive` | 0x140833230 | **O** | `byte[0]==0`（原字节） |

### 3.12 game_key_bindings.rs（10 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `KeyBinding::action_desc` | 0x14089cb60 | **O** | 44 分支 switch → 动作标签/图标（MoveUp 等） |
| `MouseBinding::action_desc` | 0x14089c8a0 | **O** | btn0→Rotate、1→Move、else→FocusOrDeselect |
| `mouse_button_to_string` | 0x14089c970 | **O** | 12 项跳表 → LMB/MMB/RMB 等 |
| `deserialize::visit_str<ron>` | 0x140366a40 | **O** | ron 变体匹配（键名表） |
| `serialize<ron>` | 0x140831740 | **O** | 40 字段名表（见 §2.3） |
| `assign_mouse_button` | 0x14089b280 | **O** | 写 `{key,mouse} u16` 于 `0x1bc+action*4`（重建已证） |
| `assign_key` | 0x14089b2a0 | **O**（交换体 I） | 26 块逐槽扫描、清 0xc3、冲突交换、变更事件 |
| `MouseBinding::get_mouse_button` | 0x14089c880 | **O** | 取值 |
| `KeyBinding::get_keycode` | 0x14089d300 | **O** | 取值 |
| `keycode_to_str` | 0x14089d490 | **O** | 键码 → 标签（跳表） |

### 3.13 game_profile.rs（6 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `deserialize/serialize<ron>` | 0x1404b0ed0 / 0x140a658a0 | **O** | 6 字段表（player_id…new_glade_count） |
| `profile_file_path` | 0x140916440 | **O** | `config_dir() + "profile.ron"`（UNK_142aaca1c, len 0xb） |
| `load_game_profile` | 0x140916480 | **O** | read_to_string；缺失 log `No profile.ron file found. Creating a new one.`；ron::from_str 失败 log `Failed deserializing profile.ron. Falling back to defaults. The error was: `；默认回退（Uuid::new_v4 + 默认键位）+ write_profile_via_tmp + insert_resource；GameProfile 0x208 B memcpy |
| `save_game_profile` | 0x140917230 | **O** | → write_profile_via_tmp；错误 log 后 drop |
| `write_profile_via_tmp` | 0x140917360 | **O** | NamedTempFile（后缀 `.tmp` UNK_142ab5b48）→ ron to_string_pretty → persist profile.ron；`.unwrap()` panic |
| `impl$0::fmt` | 0x1400da0c0 | **O** | Debug fmt（大函数，原字节 0x538 帧） |
| `impl$1::default` | 0x140916420 | **O** | 委托默认构造（FUN_140c102e0） |

### 3.14 meta_commander.rs（9 业务）
| 函数 | 地址 | 结论 | 作用（证据） |
|---|---|---|---|
| `commander_update` | 0x14091f180 | **O** | `switch((tag)^0x8000000000000000)` niche 判别，case 0..0x13（0xd 合并/共享）逐变体副作用（会话加载/复队/TimeUpdateStrategy/InputRecordReplayState/手动时间/设备列表/建目录/开 profiling/停 profiling 写文件/GPU 报告/内存日志/despawn/写设置/ShutdownGame/exit）。串：`No such profiling ID: `、`writing profiler output`、`GPU peak memory report not available.\nRun the game with --track-gpu-allocations`、panic 源 `crates/country-core/src/meta_commander.rs`、`called Result::unwrap() on an Err value` |
| `deserialize::visit_str` | 0x140168e00 | **O** | 全局动作/设置名表匹配（`input_space_cursor_move`、`camera_pan`、`tool_*`、`ui_*`、`ToolbarAlignment…`、`ImageFormationConfig`、`grading`、`InnerOuter` 等，0x14293d960+ 巨型串表） |
| `deserialize::visit_enum<ron>` | 0x140168b60 | **O** | ron enum 变体匹配（多分支） |
| `MetaCommandFence::new_pair_blocked` | 0x14091ef90 | **O** | 分配 0x18 B（Arc 头 + Mutex<(bool,bool)>），strong=2，两 flag=true |
| `MetaCommander::new` | 0x14091eff0 | **O** | 80 B：空 commands + 空 sinks + RandomState 种子 |
| `peek` | 0x14091f0c0 | **O** | VecDeque front()（0x38 B 元素） |
| `is_profiling` | 0x14091f0f0 | **O** | `[+0x38]!=0`（字段身份存疑，见 §7） |
| `consume` | 0x14091f100 | **O** | pop_front（head 环绕 + len-1），拷贝 0x38 B |
| `serialize (MemoryLog)` | 0x1413fc1b0 | **O** | MemoryLog JSON 序列化（PrettyFormatter） |

---

## 4. panic / 日志串清单（原文 + 地址）

| 串 | 地址 | 归属 |
|---|---|---|
| `crates/country-core/src/meta_commander.rs` | 0x142aade08 等 | meta_commander panic 源 |
| `country_core::meta_commander` | 0x142aade88 | 日志目标 |
| `called `Result::unwrap()` on an `Err` value` | 0x142aad758 | meta/其它 `.unwrap()` |
| `No such profiling ID: ` | 0x142aae178 | commander_update profiling |
| `writing profiler output` | 0x142aae028 | tag 0xd |
| `GPU peak memory report not available.\nRun the game with --track-gpu-allocations` | 0x142aae098 | tag 0xe |
| `crates/country-core/src/input/input_stream.rs` | 0x142ab34e8/3530 | input_stream panic/日志源 |
| `Input stream frame count: ` | 0x142ab3520 | add_frame 限频日志 |
| `assertion failed: self.frames.pop_front().is_some()` | 0x142ab3588 | drop_oldest_sync_point |
| `Out of bounds access` | 0x142ab3224 | InputStream::index |
| `Event not registered: {TypeName}` | 0x1429ae260 | get_last_event |
| `Couldn't find  ` / `Couldn't parse  ` | 0x142ae80e0 / 0x142ae8118 | load_json_as_mesh |
| `Reading mesh ` | 0x142ae7fb8 | load_json 日志 |
| `values.array_length() as i32 > max_index` | 0x142ae8078 | load_json 自定义解析断言 |
| `incorrect attribute type` | 0x142ac6fdd | load_json |
| `load_json_as_mesh: unrecognized attrib '` | 0x142a93a50 | read_attribute_from_json |
| `crates/country-core/src/utils/geo.rs` | 0x142ad5458/5470 | geo panic 源 |
| `MEOW####` | 0x142c77ab9 | update_curve_mesh null 输出 |
| `Failed to set parameter named ` | 0x142ad2cc8 | AudioLoop kira warn |
| `profile.ron` | 0x142aaca1c | game_profile |
| `No profile.ron file found. Creating a new one.` | 0x142ab5b48 区 | load_game_profile |
| `Failed deserializing profile.ron. Falling back to defaults. The error was: ` | 0x142ab… | load_game_profile |
| `.tmp` | 0x142ab5b48 | write_profile_via_tmp |
| `crates/country-core/src/game_profile.rs` / `country_core::game_profile` | — | game_profile panic/日志源 |
| 动作名大表（`input_space_cursor_move`、`tool_building`、`ui_cancel`、`ToolbarAlignmentBottomLeftRightTop`、`ImageFormationConfig`、`InnerOuter`、`WallTerrainStairs`…） | 0x14293d960+ | meta/action_set/input 全局串表 |

---

## 5. 与现有合同/代码的差异（reconstruction `country-core/src/**` 对照）

1. **`game_profile.rs` 字段**：重建 `GameProfile { player_id, key_bindings, tutorial_completion,
   tutorial_backoff, _gap }` → 证据为 `{player_id, game_installation_id, tutorial,
   tutorial_cooldowns, key_bindings, new_glade_count}`。字段名错配；`TutorialCompletionState`/
   `TutorialExponentialBackOff` 实为 `tutorial`/`tutorial_cooldowns`。
2. **`InputRecordReplayMode`**：重建 2 变体（Idle/Replaying）→ 证据 5 变体（AutoRecordPanicRepro/
   Records/Replay/DisableInput/None）。tag `0x8000000000000002` = Replay（载入文件）。
3. **`DigitalActionState`**：重建 4 态 Released/Pressed/JustPressed/JustReleased，`pressed =
   Pressed|JustPressed` → 机器码 `pressed = idx<3`、`released = (idx-3)<2`。变体顺序/数目可能
   为 5 态或不同排序；语义兼容但顺序 U。
4. **`get_last_event`/`drain_events`**：重建假设 Bevy Events → 证据为自定义 **EventRegistry**
   （TypeId→Events HashMap）。调用方 `mouse_capture`/`window` 命令类型正确。
5. **`load_json`**：重建假设 `serde_json::Value` → 证据为**自定义 JSON 解析器/Value**（断言
   `values.array_length()`）。`read_attribute_from_json` 签名 `(Value, &str) -> Vec<f32>` 与证据
   `(out, &map, name, len)` 略异（输出 tag 0..4 多态数值）。
6. **`ProportionalScreenSpace`**：重建仅 `{size: Vec2}` → 证据 16 B `{size, aspect, 1.0}`。
7. **`DebugSphere`/`DebugLine` 布局**：重建 `{center, radius, color, solid}` → 证据
   `radius@0x10, center@0x14, alpha@0x20`（radius 在 center 前）。字段顺序需按证据修正。
8. **`ChildWall::slice`**：重建 `(ChildWall, ChildWall)` → 证据返回 `{param_3, t1, t2}`
   （两 clamp 标量）。签名待复核。
9. **`InputTranslationState`**：重建 88 B 空 `Gap` → 证据 11 qwords，序列化含
   `prev_normalized_cursor_position`/`time_delta_seconds`/`main_action_set_value`。
10. **`MainActionSet` 结构**：重建 `<A,D>` 泛型两字段 → 证据 action_set 为 46 动作、0x58 B 紧凑
    视图 + 0x170 B deconstruct 视图；`deconstruct` tag 方案 `slot<<8|kind`（0x100/0x101 等）。
11. **`game_key_bindings.rs`**：重建默认 26 键序列 → 证据 36 键（多前 10 键 `0x2a..0x24` + 三个
    `0xc3`）+ `0x1b0` 处 0xc3 + u16 `{1,2,1}`；动作槽 stride 12 B（非简单 `{key,mouse} u16`）。
12. **`KeyBindings.slots` Vec 模型**：重建 `slots: Vec<KeyBinding>` → 证据为固定 40 槽数组
    （12 B 步距）而非 Vec。
13. **`meta_commander::MetaCommander`**：重建 `{commands, profiler_sinks, profiling}` → 证据 80 B
    `{commands VecDeque, HashMap, RandomState}`；`is_profiling` 读 `+0x38`（字段身份存疑）。
14. **`utils/mod.rs::world_space_to_screen_space`**：重建 `(world, view, proj, size)` → 证据单个
    VP 矩阵（r8+0x80 起）+ 透视除 + 屏幕缩放；参数面待核。

---

## 6. 跨领域线索（供中央索引关联）

- **dirty_tiles → tabloid**：`shared-foundation/crates/tabloid` 的 `State`/`StateDelta`（`apply_to`
  = OR）；`shared-foundation/crates/tabloid/tests/dirty_tiles.rs` 即该协议镜像测试。
- **dirty_tiles 订阅者**：`systems/wall/wall_state/publish_public_wall_state/gather_lazy_update_sources`
  （@0x140adeea0/0x140adf4b0/0x141458b20/0x141458a50）、`resources/world_raster/impl$2/copy_to_staging`
  （@0x140adfcb0）、`system_decorator/decorator_visual/stairs_assemble`（@0x1412945c0）、
  `system_clutter/autoclutter`（@0x141cf3540..3ee0）。
- **input_stream ↔ action_set**：`InputFrame` 内 `Vec<[u8;12]>` = `MainActionSet::deconstruct`
  记录；回放 = 每帧打包动作集。`InputRecordReplayState` 由 meta_commander tag 3/7 插入。
- **game_key_bindings ↔ action_set**：ron 字段名（`move_forward`…）与 serde_json 46 键名同源
  （`camera_*`、`tool_*`、`ui_*`、`screenshot*`、`toggle_snapping` 等）。
- **meta_commander ↔ 会话/资源**：`resources/session/NextSession::append_on_load`（fence 阻塞）、
  `TimeUpdateStrategy`、`ShutdownGame`、`WriteSettingsFileCmd`、`LastKnownGraphicsDevice`。
- **utils/mod EventRegistry ↔ mouse_capture/window**：`CustomCursorSsboCreatedCmd`、
  `OsCursorCaptureCmd`、`OsCursorVisibilityCmd`、`OsWindowCmd`。
- **load_json_as_mesh**：68 调用方（startup/prefab、mesh 入库等）。
- **geo ↔ wall 域**：`update_curve_mesh` 闭包来自 `shapes::circle::signifier_edit_circle_line`、
  `shapes::rectangle::signifier_edit_dims_line`；`rewire_enclosed_curve` 用私有 `utils::curve` crate。
- **debug_viz**：show → `FUN_14146c880`（`Commands` 族写入，见 wall/DIGEST）、`FUN_140d79f90`
  （format dispatch，见 history/DIGEST）。

---

## 7. 错位 / 纠错

1. **DigitalActionState 判别序**：`pressed = idx<3` 与重建 4 态枚举不一致 → 变体序 U（可能 5 态
   `{Pressed,JustPressed,?,JustReleased,Released}`）。不影响 O 判定。
2. **MetaCommander::is_profiling 字段**：读 `+0x38` 位于 HashMap 区（qword 7）内，非重建的独立
   `profiling` 字段 → 字段位置待核。
3. **debug_viz "字符串常量"** 实为 fmt/`Arguments` 数据表（hex 含代码指针 0x14014b850 等），
   非 ASCII 文本。
4. **load_json 用自定义 JSON** 而非 serde_json（与 terrain-water DIGEST 中
   `fs::read_to_string`/serde 描述衔接需复核）。
5. `mode()`（0x140958f30）/`from_window_size`（0x140ac8420）等在 tg_b1/b2 为 NotFound（<40B
   小函数），本次按原字节手解；其余 NotFound 小函数（Debug* builders、KeyBindings 取值器、
   InputStream 计数 getter、InputRecordReplayMode Display、GameProfile fmt）同。

---

*证据路径：`evidence/country-core-deep/utils-input-config/decomp/*`（本域新取 16）、
`expansion-b1/decomp/*`、`expansion-b2/decomp/*`；原字节经 `scripts/rne-read.py --hex` 手解。*
