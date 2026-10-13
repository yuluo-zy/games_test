# WP19 契约 2/4 — menu_background / session_selection / loading_screen / logo_animation

版本 **B**。模块：`systems/render_loop/menu_background.rs`、
`systems/render_loop/session_selection/mod.rs`、`resources/loading_screen.rs`、
`resources/logo_animation.rs`。数据源：本包反编译 + PDB 骨架 + `INDEX.md §22`
（session_ui 族：`_loading_screen`/`_logo_animation`/`_logo_branches_animation`/
`_logo_overlay`/`_session_selection_*`）。

标注：**O**=反编译/PDB；**I**=推断；**U**=未恢复。

## 1. menu_background（`systems/render_loop/menu_background.rs`，module 495）

### 类型（O；布局 U）
- `MenuBackgroundPlugin`（bevy Plugin）、`MenuBackgroundResources`。

### `MenuBackgroundPlugin::build`（O，0x1408c99c0）
分配 `MenuBackgroundResources`：默认 `{+0x00=1, +0x08=1, +0x10=0, +0x4e=7}` →
`insert_resource<MenuBackgroundResources>()`。`impl$1::default`（inline @29）同值。

### `render_menu_bg`（O，0x1408c9a30，系统）
1. 取 `Res<AppMode>`（hash `0xd38dae617fb79c7a`，state@+0x00、byte@+0x08）。
   跳过条件：`state==2`；`state==10 && byte@+0x08==1`；模式位掩码 `0xaf>>k & 1`（k=state-2<9）
   为 0 且 `(0x50>>k & 1)==0 && byte@+0x08==0`；`state==7 && byte@+0x08==1`（I：菜单/加载
   相关 AppMode 集合）。
2. 取 `ResMut<MenuBackgroundResources>`（hash `0xfb32e4dcc8287bca`）。`lVar2=*data` 为内部
   Mutex 结构：lock byte@+0x10；句柄组 @+0x18..+0x48（7 qword）；`+0x4e=7`。
   锁内读取 7 句柄 → 若 `+0x48 第6字节 != 7` 且 `data+0x3e != 7` →
   `Plugin::finish<MainCameraPlugin>`（释放主相机）+ drop；把 7 句柄复制到 `data+0x08..0x40`。
   若 `data+0x3e==7` → 返回（已完成）。
3. `get_raster_handle_by_name("blit_stretch_decode_srgb")`（**O 字符串**，data 0x142aa31e0；
   与 INDEX.md 最终图 `_blit_blit_stretch_decode_srgb` 同 shader）→
   `RasterPass::new_impl` + `DrawBuilder::new`：`bind_texture_to_draw_builder<&Texture>`
   （背景纹理）→ `draw_no_vao` → `RasterPass::render`。

语义（I）：菜单背景 = 把背景纹理 stretch-decode-sRGB 全屏 blit；首次执行释放相机。

### typed gaps
`MenuBackgroundResources` 全布局（7 句柄语义、+0x00/+0x08/+0x10 默认值含义）；
AppMode 模式集合含义；背景纹理来源。

## 2. session_selection（`systems/render_loop/session_selection/mod.rs`，modules 487/501/1585/1610）

### 类型（O；布局 U）
- `SessionThumbnailsPlugin`、`ThumbnailTextures`、`PooledThumbnail`。

### `SessionThumbnailsPlugin::build`（O，0x1408f68c0）
`insert_resource<ThumbnailTextures>()` +
`add_systems<PreStartup, thumbnails_startup>`（签名 `(Res<GraphicsDeviceResource>,
Res<AssetShaderLibrary>, Res<ShaderWatch>, ResMut<ThumbnailTextures>)`）。

### `thumbnails_startup`（O，0x1408f6900）
- `load_shader_into_library<&str>` **×4**（I：`session_selection_background`/
  `session_selection_thumbnail`/`session_selection_thumbnail_painterly`/
  `session_selection_new_glade_thumbnail`——与 `render_session_selection_previews`
  按名取 4 个 shader 一致，O）。
- `TextureDesc::new_2d + sampled + mip_levels` → 循环 6 次（O，池大小=6）调用
  `thumbnails_startup::closure_env$0`（inline @319/322），每个产出 **0x68 字节**
  记录（7×16B 纹理句柄）→ push 进 `ThumbnailTextures` 内部 `Vec<PooledThumbnail>`
  （stride 0x68）。

### `render_session_selection_previews`（O，0x1408f6c00，11245 字节；Ghidra/ReVa 反编译均超时，
本包以 callee 清单 + PDB inline 宿主 + g7-post-ui 遗留 `0x1408F6C00.c` 复述）
1. 取 `Res<AppMode>`（hash `0xd38dae617fb79c7a`）：`state-2 < 9 && state-2 != 4` → 返回
   （只在会话选择相关 AppMode 渲染）。
2. `get_raster_handle_by_name` ×4（**O 字符串**，data 0x142aa8ec1/f52/f05/fa9）：
   `session_selection_background`(28)、`session_selection_new_glade_thumbnail`(37)、
   `session_selection_thumbnail`(27)、`session_selection_thumbnail_painterly`(37)。
3. 锁 `ThumbnailTextures`（或会话列表，parking_lot；+0x10 lock、+0x14..+0x40 字段）→
   读 SavedSession 列表（`from_iter<(SavedSession, Option<&Texture>), closure>`，O）。
4. inline 宿主（PDB O，宿主 `render_session_selection_previews`）：
   `ThumbnailTextures::extract_thumbnails_to_display` @72（`from_iter<Option<&Texture>,
   Map<Iter<usize>, extract_closure>>` @115）、`find_backed_thumbnail_range` @127、
   `assign_pool_indices` @149（`assign_pool_indices::closure$0` @175/@179、`closure$1` @205）、
   `refresh_screenshot` @235。
5. 截图刷新（O callees）：`Reader::open_impl` + `image::decode<BufReader<File>>` +
   `DynamicImage::to_rgba8`（PNG 缩略图磁盘读取）；`eq_by<Rev<Path::Components>>` 路径比较
   （screenshots 目录判断）；`insert<String, SetValZST>`（BTreeSet 已见名）。
6. 绘制：`RasterPass::new_impl`（extent 含窗口尺寸 `lock<PhysicalSize<u32>>`，O）+ 每缩略图
   `DrawBuilder::new(shader)` + `optional_uniform<[u32;2]>` + `bind_texture_to_draw_builder`
   `<&Texture>`/`<TextureFromDisk>` + `draw_no_vao` → `RasterPass::render`。

语义（I）：会话选择界面把磁盘上的会话截图（PNG）加载进 6 槽纹理池，按池索引分配，用
`_session_selection_thumbnail`/`_thumbnail_painterly`/`_background`/`_new_glade_thumbnail`
绘制背景与新存档缩略图。

### typed gaps
`ThumbnailTextures`/`PooledThumbnail` 布局（0x68 stride 字段映射）、截图像素尺寸、
`extract/find/assign/refresh` 精确算法、`render_session_selection_previews` 完整反编译。

## 3. loading_screen（`resources/loading_screen.rs`，modules 540/1596/1603/1611）

### 类型（O；布局 U）
- `LoadingScreen`、`LoadingScreenProgress`。

### `LoadingScreenPlugin::build`（O，0x140a1a190）
`insert_resource<LoadingScreenProgress>()` +
`add_systems<Startup, startup_loading_screen>`（签名 `(Res<GraphicsDeviceResource>,
Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`，O）+ 
`add_systems<Update, update_loading_screen>.run_if(谓词 (ResMut<LoadingScreenProgress>,
Res<LogoAnimation>, FilteredWallTime))`——谓词签名 `bool(*)(Option<Res<LoadingScreenProgress>>)`
（I：资源存在才更新）。

### `startup_loading_screen`（O，0x140a1a1e0）
- `from_iter<u32, FlatMap<TupleWindows<RangeInclusive<u32>, (u32,u32)>, [u32;6],
  draw_shaders::clouds::startup_clouds::closure_env_0>>`（O：复用 clouds 起点闭包生成
  [u32;6] 网格索引——加载屏“云朵/网格”造型，I）。
- `load_mesh_into_library`（mesh 名 hash `0x42ac8c61`、len 14，字符串 U）+ 
  `load_shader_into_library<&str>`（加载屏 shader）。
- `Commands::spawn_empty` + insert `(DrawableMeshBundle, DesignatedRenderPass, DontCastShadow)`
  （O：组件名完整），transform 单位缩放。

### `update_loading_screen`（inline @29，宿主 `FunctionSystem<void(*)(ResMut<LoadingScreenProgress>,
Res<LogoAnimation>, FilteredWallTime)>`，O）
- 每帧推进 `LoadingScreenProgress`（+0x10 字段读改写；`LogoAnimation` 状态查询）：
  加载条进度动画。`startup_loading_screen::closure$0`（无独立实例）。

### typed gaps
`LoadingScreen`/`LoadingScreenProgress` 字段布局；加载屏 mesh 名字符串；`update_loading_screen` 体。

## 4. logo_animation（`resources/logo_animation.rs`，module 579）

### 类型（O；布局 U）
- `LogoAnimation`（资源）、`AnimatedLogo`（实体组件）。

### `LogoAnimationPlugin::build`（O，0x140b19d40，impl$2::build）
`insert_resource<LogoAnimation>()`（默认）+ `add_systems<Startup, startup_animated_logo>`
（签名 6 参同 startup_loading_screen）+ `add_systems<Update, animate_logo>`（签名
`(ResMut<LogoAnimation>, Res<bevy_time::time::Time<()>>, Res<AudioCore>)`，O）。

### `LogoAnimation` 方法（O）
- `play(&mut self)`（0x140b19d00）：清零 `+0x00(u64)`、`+0x08(u64)`、`+0x10(u32)`。
- `finish_wait(&mut self)`（0x140b19d30）：`+0x0c = 1.0f32`。
- `is_waiting()`（0x140b19d10）、`is_finished()`（0x140b19d20）：16B 常量函数
  （真实签名 `-> bool`）。语义 I：`is_finished = +0x0c >= 1.0`。

### `startup_animated_logo`（O，0x140b19d90）
- `load_json_as_mesh<&str>`（logo 网格 JSON）+ `MeshWatch::watch()`；
  `AssetMeshLibrary::add("tiny_glade_logo")`（**O 字符串**，15 字符）；`load_shader_into_library`；
  生成实体 insert `(KeepOnLevelUnload, DrawableMeshBundle, DesignatedRenderPass, DontCastShadow)`
  （**O 组件名**），transform scale≈{6,5,5}。
- 第二网格 `AssetMeshLibrary::add("logo_branches")`（**O 字符串**，13 字符）+ 第二个实体
  （枝干 logo 层）。

### `animate_logo`（O，0x140b1a6b0，系统 `(ResMut<LogoAnimation>, Res<Time<()>>, Res<AudioCore>)`）
- `dt = min(Time::delta_seconds@+0x58, 1)`（O：Time 数据 +0x58）。
- **5 阶段 logo 动画**（O）：`stage[0..5] f32`（0..1 进度）+ `duration[0..5] f32`；
  找第一个 `stage[i] < 1.0`，`stage[i] = clamp(stage[i] + dt/duration[i], eps, 1.0)`。
- 阶段开始（`stage[i]==0.0`）播放音频（O 字符串）：stage0 = `event:/UI/Logo/Scribble`
  （`builtin_strncpy` 0x17=23），stage2 = `event:/UI/Logo/Branches`（栈内 3×8B 拼接）；
  经 `AudioCore::instantiate_audio` + `AudioInstance::start_if_not_playing`。
- 每阶段推进写 `*(LogoAnimation + 0x10 目标指针) = 当前 stage 值`（O：+0x10 为 GPU uniform
  写目标，+0x1c 当前值）；全部完成返回。

### typed gaps
`LogoAnimation`（0x20+ B）/`AnimatedLogo` 字段全布局；5 阶段各自语义；`Time`/`FilteredWallTime` 布局。

## 5. shader 族映射（O）
`_loading_screen`/`_logo_animation`/`_logo_branches_animation`/`_logo_overlay`/`_session_selection_*`
均有 `b903…/f0ad…` 两变体（INDEX.md §22）。`blit_stretch_decode_srgb` 同最终图 blit 族。
