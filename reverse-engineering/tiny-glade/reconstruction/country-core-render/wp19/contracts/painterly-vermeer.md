# WP19 契约 1/4 — painterly / vermeer 风格化绘制链

版本 **B**（`tiny-glade.exe` sha256 cf048e27…，PDB `tiny_glade.pdb` 2d9686b8…）。
模块：`systems/render_loop/painterly/mod.rs` + `systems/render_loop/vermeer.rs`。
数据源：本包反编译（`analysis/decomp/*.c`，Ghidra shared worker 只读）+ PDB 骨架
（`painterly/mod.rs`、`vermeer.rs` bundles）+ `evidence/render-passes/INDEX.md §21`
（painterly_vermeer 族）+ `pass-builder/shader-uri-list.json`。

标注：**O**=反编译/PDB 直接支持；**I**=调用链/名称组合；**U**=未恢复。

## 1. 概览（I/O）

Tiny Glade 用两套风格化绘制链做“会话启动揭示动画”（reveal）：**painterly**
（素描/粉彩渐进笔触）与 **vermeer**（维米尔风格），供缩略图/画廊/启动揭示复用
（INDEX.md §21：generate_strokes→raster_strokes→combine）。二者共用同族 shader
（painterly_* 与 vermeer_* 变体），区别在进度模型与笔触总量。

## 2. painterly（`systems/render_loop/painterly/mod.rs`）

### 类型（O 名称；布局 U）
- `PainterlyPlugin` — bevy `Plugin`（size U）。
- `PainterlyState` — 揭示状态资源，布局见 §2.3。

### `PainterlyPlugin::build`（O，0x1408a63e0）
`App::insert_resource<PainterlyState>()`（Default）；`SubApp::add_systems<PreStartup, startup_painterly>`。

### `startup_painterly`（O，0x1408a7fe0；系统签名 `(Res<GraphicsDeviceResource>, Res<AssetShaderLibrary>, Res<ShaderWatch>)`）
调用 `startup::load_shader_into_library<&str>` **2 次**——把 painterly 光栅 shader
注册进 `AssetShaderLibrary`（I：名字为 `painterly_raster_strokes` 与 painterly debug/acquire
之一；计算 shader 走 `ComputePass::new<&str>` 按需加载）。

### `PainterlyState::has_target`（O，0x1408a63d0，7 字节常量函数）
恒等/常量访问器，真实签名返回 bool（`void()` 为 PDB 占位）。语义 I：查询是否已有绘制目标。

### `painterly_pass`（O，0x1408a6410，5339 字节系统）
系统签名 `(Res<AppMode>, ResMut<PainterlyState>, ...)`（I）。业务逻辑：
1. 取资源 `AppMode`（hash `0xd38dae617fb79c7a`，state@+0x00、byte@+0x08）。若 `state==7 && byte@+0x08==1`
   → 直接返回（游戏内模式不跑 painterly）。
2. 取 `ResMut<PainterlyState>`（hash `0x1f18d9c9bdba6c60`）。`state = +0x08`：
   - `state==2` → 重置 progress=0、state=0、timer(+0x14)=0。
3. 取资源 hash `0x48b321ef779dc301`（byte@+0xff）；若 0 再取 hash `0x8fd9dc19c7973bca`（int@+0x80==0）
   → `local_72 = has_painterly_target`（I：目标可用标志）。
4. 若 `in_stack_00000030 == 0`（系统使能字节）→ `state=2`、返回。
5. 计算 temporal extent：`extent_2d()` 得 {w,h}，`uVar7 = ((w<<8)/h) | (1<<32)`（I：横纵比缩放）。
   `get_or_create_temporal<PassResources,&str>` ×2（目标对 + 中间对）。
6. 若 `state==0 || (两个 temporal 都是新建)`（`(local_5b8._4_1_ & bVar4 & 1)==0`）：
   - 日志 “Acquiring painterly target”（O，`PTR_s_Acquiring_painterly_target`）。
   - `create_raw_resource`（名 hash 0x42a9bc70/0xd —— **strokes 缓冲纹理**）。
   - `ComputePass::new<&str>`（**acquire_target**，URI `/painterly/acquire_target.cs.hlsl`，I）：
     绑 3 个 `Handle<Texture>` + 2 个 `&mut Handle<Texture>`，`PassBuilder::render`（dispatch）。
   - 第二个 `ComputePass::new<&str>`（**convolve_target**，URI `/painterly/convolve_target.cs.hlsl`，I）：
     绑 2 纹理，`PassBuilder::render`。
   - `state=1`，`+0x0c = extent handle`。
7. `BufferDesc::new` + `get_or_create_temporal<PassResources,&str>`（**strokes SSBO 对**，O：
   `ComputePass::write_rg_storage_buffer` 之后的 `DispatchIndirectBuffer` 载体）。
8. 取时间资源（hash `0x109ca79d5cccc2d1`；`+0x40 f64` 速率、`+0x58 f32` dt，名称 U）。
   进度模型（O）：`progress = f64 @+0x00`，步进 `step = has_target ? 1024.0 : rate*1638.4`，
   `progress = min(progress + step, 1024.0)`。当 `floor(progress)` 递增（`uVar12-uVar7 != 0`）：
   - `ComputePass::new<&str>`（**generate_strokes**，URI `/painterly/generate_strokes.cs.hlsl`，I）：
     绑 2 纹理 + `write_rg_storage_buffer`（strokes_buf），dispatch 覆盖新笔触区间
     `{start=floor(old), end=floor(new), +0x14 timer}`（12B `[u32;3]`，O）。
   - `*state = progress`。
9. `get_raster_handle_by_name(anon…)`（I：`painterly_raster_strokes`）→
   `RasterPass::new_impl` + `DrawBuilder::new`：
   - `optional_uniform<[u32;2]>`（extent）、uniform f32；
   - `bind_texture_to_draw_builder<TextureFromDisk>`（名 hash `0x76ef6f6cc99f108b`）；
   - `read_rg_storage_buffer`（strokes_buf）；`draw_no_vao`；`RasterPass::render`。
   - `timer(+0x14) += dt(+0x58)`。

### PainterlyState 布局（O 偏移，语义 I）
| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | f64 progress（0..1024，笔触总数） | `*pdVar9 = dVar3` 累加；重置 0.0 |
| +0x08 | u32 state（0=待机/1=运行/2=完成） | `*(pdVar9+1)=2`/`=1`/`=0` |
| +0x0c | u64 temporal extent 句柄 | `*(u64*)(+0x0c)=extent` |
| +0x14 | f32 timer（累计 dt） | `+0x14 += *(f32)(time+0x58)` |

### inline 宿主（PDB O）
`painterly_pass::closure$0`（行 123，dispatch 闭包）、`raster_strokes`（行 270，
`RasterPass`/`DrawBuilder` 组合，与 vermeer::raster_strokes 同构）。

## 3. vermeer（`systems/render_loop/vermeer.rs`）

### 类型（O 名称；布局 U）
- `GenerateStrokes` — 计算 pass 类型，`AsComputeShaderSource` URI
  **`/vermeer/generate_strokes.cs.hlsl:cs_main`**（O，shader_uri.rs §3）。
- `VermeerPlugin`、`VermeerDebugControl`（调试控制资源，O；用途 U）、`VermeerState`。

### `VermeerPlugin::build`（O，0x1408a8280）
`insert_resource<VermeerState>()` + `insert_resource<VermeerDebugControl>()` +
`add_systems<PreStartup, startup_vermeer>` +
`add_systems<OnExit<AppLoadingState>, impl$1::build::closure_env$0>`（= `deactivate` 系统，O）。

### `startup_vermeer`（O，0x1408aa720）
`load_shader_into_library` ×2（I：`vermeer_raster_strokes` + `vermeer_combine`）。

### VermeerState 方法（O）
- `activate(&mut self, param_a: u8, param_b: u8)`（0x1408a8060）：若 state(+0x48)!=2 且
  `+0x3e != 7` → `Plugin::finish<MainCameraPlugin>`（释放主相机）+ drop handle；
  重置 `+0x00=0`、`+0x3e=7`、`+0x40..+0x47=0`、state=0；`+0x54=param_a`、`+0x55=param_b`。
- `deactivate(&mut self)`（0x1408a8140）：若 state!=2 且 `+0x3e!=7` → 释放相机；`state=2`。
- `into_disable_hold(&mut self, other: &VermeerState)`（0x1408a81e0）：state!=2 断言；复制
  `other` 88B 到 self，`+0x54=0`（“解除 hold 标志”）。
- `is_active()`（0x1408a8260）、`should_freeze_camera()`（0x1408a8270）：16B 常量函数，
  真实签名 `-> bool`（`void()` 占位）。`should_freeze_camera` 被 `main_camera_update@97` inline
  （O）。语义 I：`is_active = state != 2`；`should_freeze_camera = state==1`。

### `vermeer_pass`（O，0x1408a82f0，系统）
1. 取资源 hash `0xbe2e7c0aff5cc813`（byte@+0x04、+0x14、+0x28；名称 U，I：揭示控制资源）。
2. 取 `ResMut<VermeerState>`（hash `0xa284155e68bfeb3f`）。`state=+0x48`：
   - `state==2`：若控制资源 byte@+0x04 非 0 → 重置 `+0x00=0,+0x3e=7,+0x40=0,state=0,+0x54=0`
     并返回；否则全局 `DAT_143634c9c = 0`（O：全局活动标志）并返回。
   - 否则 `DAT_143634c9c = 1`。
3. `extent_2d(target+4)` → `new_2d + storage + sampled` →
   `get_or_create_temporal<PassResources,&str>`（**目标 temporal 对**）。
4. `ComputePass::new<&str>`（I：`vermeer_acquire_target` 或 convolve；URI
   `/vermeer/acquire_target.cs.hlsl`）→ 绑 `Handle<Texture>` + `&mut Handle<Texture>` →
   `PassBuilder::render`。`state=1`。
5. 时间资源（hash `0x109ca79d5cccc2d1`）dt：`dt = min(f32@+0x58, 1/30)`。
   - 若控制资源 byte@+0x14==0（未冻结）：`progress(+0x40) += dt/3`（除非 state<2 且 +0x54 置位）。
   - 否则（冻结）：`progress = 控制资源 int@+0x00`。
   - `frame(+0x00) += 1`；`timer(+0x44) += dt`。
   - **`vermeer_render_from_target()`**（主绘制，见 §3.1）。
6. 完成判定（O）：`progress>=1.0 && 控制资源 byte@+0x14!=1 && +0x54==0` →
   若 state!=2 且 `+0x3e!=7` → `Plugin::finish<MainCameraPlugin>` + drop；`state=2`；
   `EventWriter<WriteSettingsFileCmd>` 发送（grow_one）；日志 “Game reveal complete”
   （O，`PTR_s_Game_reveal_complete`）。

### `vermeer_render_from_target`（O，0x1408a8d70，6580 字节）——vermeer 绘制链
参数（I）：`(target: Handle<Texture>, ... , 分辨率 u32, background 标志字节 in_stack_40, 字节 in_stack_48)`。
1. `stroke_count = background? 0x8000(32768) : 0xa00(2560)`（O；`inline max_stroke_count` @254）。
2. 若目标句柄为空（`in_R9==0`，O）：`AssetTextureLibrary::get_by_name`（资源 hash
   `0xa68561b396784eaa`，名称 U）→ `into_sampled_rg_texture`（取背景纹理）。
   **`blur_pyramid()`**（O，compute_passes::blur_pyramid）模糊背景。
3. `extent_2d(target+4)` → `new_2d` → `create_raw_resource`（名 hash `0x42a9bc70`
   = **strokes 缓冲纹理**）→ `BufferDesc::new` → `get_or_create_temporal`（strokes SSBO 对）。
4. `ComputePass::new<GenerateStrokes>`（**URI `/vermeer/generate_strokes.cs.hlsl:cs_main`**，O）：
   - `bind_texture_to_compute_pass<&Handle<Texture>>`（背景纹理，若存在，O：vermeer 两变体差异
     为有无 `background_tex`，INDEX.md §21）；
   - `write_rg_storage_buffer`（strokes_buf）；
   - uniforms（O，`local_542/local_a32 != '\b'` 守卫）：f32、i32、f32、bool、bool；
   - `PassBuilder::render`（dispatch；`local_614 = stroke_count` 相关）。
5. `get_raster_handle_by_name("vermeer_raster_strokes")`（O 字符串）→
   `RasterPass::new_impl` + `DrawBuilder::new`：`optional_uniform<[u32;2]>`（extent）、
   uniforms f32/bool/i32、`bind_texture_to_draw_builder<TextureFromDisk>`
   （名 hash `0x76ef6f6cc99f108b`）、`read_rg_storage_buffer`（strokes_buf）、`draw_no_vao`、
   `RasterPass::render`。
6. `get_raster_handle_by_name("vermeer_combine")`（O 字符串）→
   `RasterPass::new_impl`（target 为输出）+ `DrawBuilder::new`：`optional_uniform<[u32;2]>`、
   uniforms f32/bool/i32、`bind_texture_to_draw_builder<&Handle<Texture>>` ×2
   （strokes 纹理 + 目标）、`draw_no_vao`、`RasterPass::render`。

### VermeerState 布局（O 偏移，语义 I）
| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | u64 帧计数 | vermeer_pass `*puVar4 += 1`；activate 清零 |
| +0x40 | f32 progress（0..1） | `+0x40 += dt/3`；`>=1.0` 完成 |
| +0x44 | f32 timer（累计 dt） | `+0x44 += dt` |
| +0x48 | u32 state（0/1/2） | activate/deactivate/vermeer_pass |
| +0x4c | 句柄/指针（8B） | vermeer_pass `+0x4c = ...` |
| +0x3e | u8（==7 表示相机已释放） | activate/deactivate 守卫 |
| +0x54 | u8 hold 标志 | activate 参数；into_disable_hold 清零 |
| +0x55 | u8 参数 | activate 参数 |
| 其余 0x58B | typed gap | into_disable_hold 复制 88B |

## 4. 全局与 URI（O）
- 全局 `DAT_143634c9c`：vermeer 活动标志（vermeer_pass 置 0/1）。
- shader 族（INDEX.md §21 + shader-uri-list）：painterly `acquire_target/convolve_target/
  generate_strokes/raster_strokes/debug_target`、vermeer `acquire_target/convolve_target/
  generate_strokes(:cs_main)/raster_strokes/combine`。`/vermeer/generate_strokes.cs.hlsl`
  两个变体 `2f7da8dc…/f890205f…` = 有无 `background_tex`（O）。

## 5. typed gaps
- `VermeerState` 0x00..0x40 与 +0x50..0x58 中间字段；`PainterlyState` 其余字段。
- `VermeerDebugControl` 布局/用途；`GenerateStrokes` 结构体字段。
- painterly by-name 计算 pass 的 shader 名（acquire/convolve/generate 为 I）。
- `startup_painterly`/`startup_vermeer` 各 2 个 load_shader 的确切名字。
- 控制资源 hash `0xbe2e7c0aff5cc813`、时间资源 hash `0x109ca79d5cccc2d1` 的名称与全布局。
- `stroke_count` 0x8000/0xa00 与 background 标志的精确映射。
