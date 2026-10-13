# 合同：misc resources 杂项（26 模块）+ render/nani.rs + systems/save/save_backup_metronome + systems/garden/{draw_garden,record_garden_history}

版本 B。方法同 WP21 其它族。每模块：O 证据 → I → U。

## clutter_meshes.rs（O：ClutterMeshTy 55 变体 + 名称映射 + 数据表）

- **`ClutterMeshTy`**（0x38=56 变体）：`From<&str>`（0x1408b83d0，2231B）按资产路径前缀
  匹配到变体（`clutter/plants/...`、`clutter/watering/...`、`clutter/windows_...`、
  `clutter/firewood/...`、`clutter/crates_...`、`clutter/barrel_w_...`、`clutter/stall_veggies/...`、
  `clutter/cart_w_pumpkins/...`、`clutter/basket_w_food/...` 等，未识别 panic）。
- **`clutter_mesh_ty_data`**（0x1408b8e90）：变体→(名指针, 名长, kind_id) 数据表（switch 0..0x37）。
- **`ClutterMesh`** / **`ClutterMeshes`**：资产库/集合（HashMap 或 vec；U）。
- **`ClutterMarkedForDeletion`** / **`DecoratorAutoClutterChildren`** / **`DisplayChildAutoClutterInfo`**：
  UI/删除标记组件。`impl$1::register_required_components`（0x1408b9170）= Bevy ECS 组件注册胶水。
- **`impl$0::{first,next}`**：变体迭代（0x2370 折叠 stub）。

## color_icon.rs（O：brightness + 图标组件注册）

- **`ColorMenuBrightnessValue`**（枚举）：`to_f32`（0xb06770）、`default_f32`（0xb06780）。
- 15+ 图标组件：`ColorIcon` / `ColorWheelSeparator` / `MagicColorIcon` / `WallStyleIcon` /
  `FlatRoofStyleIcon` / `FlatRoofRimTypeToggle` / `PlasterPeelIcon` / `CustomizationIcon` /
  `ChimneySmokeToggle` / `FreehandHeightTypeToggleIcon` / `StairRailingIcon` /
  `ColorMenuBrightnessIcon` / `FlagConnectionIcon` / `CustomizationIconMarkedForDeletion`；
  各自 `register_required_components`（0xb06b20..0xb0c550，多数折叠到共同 stub）。
  语义：调色菜单图标组件的 Bevy 注册。

## daily.rs（O：DailyThemeInfo 时间戳）

- **`DailyThemeInfo`**：`default`（0x83c480）置两个 u64 = 0x8000000000000000
  （SystemTime 前 epoch 哨兵）；`is_weekly`（0x83c440）用 `std::time::SystemTime::
  duration_since` 判周限；`get_name`（0x83c240）返回主题名（String，U）。
- **`CurrentDaily`** / **`DailyThemeCredits`**：当前主题/致谢（U）。

## feedback_report_resources.rs（O：反馈报告资源 + 截图）

- **`FeedbackReportResources`**：`+0x0 Arc<Mutex<Option<Vec<u8>>>>`（截图字节）、
  `+0x8 Arc<Mutex<Option<Texture>>>`（rhapsody 纹理）、`+0x3e` 状态字节（7=空）、
  `+0x40..0x46` 选项/选择字节。`reset_choices`（0x909910）：清 3 个选项字节
  （+0x40=0、+0x44=0x0101、+0x46=0）。`clear_screenshot`（0x909930）：drop 截图/纹理、
  状态置 7。`feedback_report_texture_receive`（0x909ab0）：接收 GPU 截图纹理并存。
- **`FeedbackReportTags`**：标签表；`to_list`（0x909870）→ `Vec<String>`（filter_map 静态串）。
- **`FeedbackReportBgTexture::is_available`**（0x909860）：背景纹理就绪。
- **`FeedbackReportBackgroundPlugin`**（impl$3::build 0x909a70）：Bevy 插件
  （insert_resource + EscapeMenuJustClosed 系统）。rhapsody Texture = 复用边界。

## garden_mask.rs / preplaced_autoclutter.rs / shape_intersections.rs / weather.rs / sparkle.rs（O：标记）

- **`GardenMaskSsbo`**（garden_mask.rs）：花园掩码 SSBO 载体（空 bundle；世界光栅 512² 族）。
- **`PreplacedAutoClutter`**：预放置自动杂饰（`impl$1::default` 内联；U）。
- **`ShapeIntersections`**：形状求交缓存（`impl$0::default` 内联；U）。
- **`Weather`**：天气资源（`impl$0::default` 内联；U；audio weather 消费）。
- **`SparkleSsbo`**：闪烁粒子 SSBO（空 bundle；U）。

## glade_color_customization.rs（O：默认布局字节）

- **`GladeColorCustomization`**：`impl$0::default`（0x8c5600，82B）：+0x0=0、+0x4=0、
  +0x8=0x186a3(99939?)、+0x18=0x800000000、+0x20=0、+0x40=0(u16)、+0x42=1、
  +0x48/+0x50/+0x58=0、+0x60=1.0。字段语义 U（glade 自定义颜色/阶梯配色，serde 存档）。
- **`StairCustomization`**：阶梯配色（U）。

## glade_settings.rs（O：load + 默认）

- **`GladeSettings`**：`load`（0x8bbbc0）从 `path.join("glade_settings.json")`
  `serde_json::from_str<Option<GladeSettings>>`；失败回退 `Default`。
  `Default`（0x8c2240）：`{ +0x0: u32=0, +0x8: f32=0.01, +0xc: f32=1.0, +0x10: u32=8,
    +0x18: f32=0.0, +0x1c: f32=1.0, +0x20: u8=1 }`（字段语义 U）。
- **`GladeSettings::try_load`**（O 名，内联）。

## gothic_window_profile.rs（O：拱形轮廓曲线）

- **`gothic_half_dormer_profile_curve(size)`**（0x9a4cc0）：size 1/2/3 → 6/8/10 个 Vec2
  曲线点（f32 对；末点 y=−0.95）；size≠1..3 panic。输出 `Vec<Vec2>`（ptr/len/cap）。
- **`gothic_door_profile_curve(size)`**（0x9a4e70）：size 1/2/3 → 6/5/6 个 Vec2 点
  （size2 直接 5；含负 y 尾点）。
- 语义：Gothic 窗/门拱剖面的 CPU 常量曲线（供 wall/decorator 轮廓用）。

## image_formation.rs（O：ImageFormationConfigs 三档）

- **`ImageFormationConfigs`**：`{ configs: [ImageFormationConfig; 3]（0xc0B 步长）,
  kind: u8 @+0xc0 }`。`switch_kind`（0x8aea90）写 kind；`iter_non_default_mut`（0x8aea60）
  迭代三档；`current/current_mut/get/initial`（0x8aea50/80，+ `initial` 折叠）。
- **`ImageFormationConfig`** = 0xc0B：`{ name: "default"(8B), Vec<7 项>（@+0x8，ptr@+0x8、
  cap@+0x10、len@+0x0 布局 O），项=各种成像参数（+0x18 起，1.0/1.0 等默认）}`。
- **`ArtisticConfig`** / **`CameraLensConfig`**：艺术/镜头配置子类型（serde；U 详布局）。
- 语义：画面形成（锐化/暖光/相机镜头）三档预设（默认/暖/…，I）。

## mouse_capture.rs（O：捕获命令 + 系统面）

- **`CursorCapturedPosition`**：`new` + `is_currently_captured`（O 名；`impl$0::default`
  0xa86b30 = `{ u32=0 }` 折叠自 mode_manager）。**`CursorCaptureCmd`** /
  **`OsCursorCaptureCmd`** / **`OsCursorVisibilityCmd`** / **`OsCursorVisibilityCmdOrigin`** /
  **`CustomCursorSsboCreatedCmd`** / **`MOUSE_CAPTURE`**：窗口光标捕获命令族。
- **`mouse_capture`**（0xb17830，系统）：读窗口/事件状态，未捕获时发 SettingsMenuState /
  WindowFocusEvent 等（事件队列为 Bevy/winit 边界）。

## photo_mode.rs（O：相机模式循环 + 插件）

- **`PhotoModeCameraMode`**（判别枚举）：`next`（0xb59720）、`next_demo`（0xb596b0，
  mode 3 panic）。**`PhotoModeCmd`**：命令事件。**`PhotoModePlugin`**（impl$0::build
  0xb59640）：`register_event<PhotoModeCmd>`；`impl$1::from`（0x90e500）极短 stub。
- 消费：`startup/init_photomode_state.rs`（共享 crate）。

## placement_data.rs（O：SSBO 记录 → Affine3A）

- **`PlacementDataSsbo`**：放置记录：`{ position: Vec3 @+0x0, scale: f32 @+0xc, angle: f32
  @+0x10 }`。`to_affine3a`（0xad5020）：绕 Y 轴旋转矩阵×scale（sin/cos(angle/2)，
  轴 y=0 的经典构造），输出 3×4 矩阵；`position`（0xad51a0）取 position。
  `to_transform`、`default_anim_t`（O 名，内联）。
- 消费：放置系统/GPU 实例 transform（`PlacementDataSsbo` 经 AssetSsboLibrary）。

## prefab.rs（O：PrefabLibrary HashMap）

- **`PrefabLibrary`** = `HashMap<PrefabName, Prefab>`；`add`（0x8e2540）插 `Prefab`
  （含 `Vec<PrefabPart>`，drop 逐 part）；`get<PrefabName>`/`has<PrefabNameRef>`/`iter`。
  `PrefabPart` / `PrefabInstance`（0x58B，impl$7::register_required_components 0x8e5d10 =
  Bevy 组件注册）。**`PrefabsModifiedCmd`**、**`PrefabWatchPlugin`**、**`AudioTriggerShape`**。
- 扩展共享 crate `resources/prefab/mod.rs`（wp13 骨架对齐）。

## raster_stroke.rs（O：RasterStroke 点集）

- **`RasterStroke`**：`{ points: Arc<[Vec3]>, radius: f32 }`。`add_point`、`smooth`（11 内联，
  RDP/平滑）、`radius`、`length`、`iter_points_xz`（0xa97860，50 内联）、`impl$4::from`
  （0x9f1c90 fmt=Debug）。地形笔刷类（与 `terrain_stroke` 同域不同源）。

## render/spatial_resolve_offsets.rs（O：默认偏移表）

- **`SpatialResolveOffsetsSsbo::default_value`**（0xb31d10，7179B）：写 2KiB 常量表 =
  128 个 (dx,dy) i32 偏移对（0x10 步长交织 0 填充；首项 {-1,-1}、{2,-1}、{-2,-2}…），
  `+0x0=1`（计数）。TAA 空间解析默认偏移。Rust：结构体 + 表（O 结构/部分首项；全表转录
  留证据，U 语义）。

## shader_refactoring_state.rs（O：disabled）

- **`ShaderRefactoringState`**：`disabled()`（0xb1f210）构造停用态（bool 标志）。

## shutdown.rs（O：initiate_game_shutdown 0xb2ba40）

- **`ShutdownGame`**：命令资源。`initiate_game_shutdown`：读事件，有触发则
  `Commands::insert_resource(ShutdownGame)`（Bevy command queue push 8B）。

## steam.rs（O：Steamworks 输入行动集）

- **`SteamInput`**（0x9e6280 `get_action_origin_glyphs`）：steamworks API 取行动原点字形
  （`SteamAPI_ISteamInput_GetGlyphForActionOrigin_Legacy`）→ `Vec<(String, …)>`。
- **`SteamMainActionSet`** / **`SteamMenuActionSet`**：`read_state`（0x9e86b0/0x9e8bf0）
  调 `GetAnalogActionData`×N + `GetDigitalActionData`×N → `SteamMainActionSetState`
  （0x30B：4×f32 模拟 + 24 数字字节位）/ `SteamMenuActionSetState`（0x10B：1×f32 + 9 位）。
  `is_any_active`（0x9e8aa0）：任一位有效。`try_load<Input>`（0x17c3720/0x17c5770）。
  `SteamMainActionSetDigitalAction::name_c_str` 等静态名表（内联）。
- **`SteamActionSetAction`** / **`SteamActionSetDigitalAction`** / **`SteamMainActionSet{Action,
  DigitalAction, AnalogAction}`** / **`SteamMenuActionSetAction`** 等：行动句柄类型。
- 复用边界：`steamworks` crate（缓存无 → typed gap）；全部为 steamworks 调用面。

## texture_presets.rs（O：From<u8> 建 TextureDesc）

- **`TexturePreset`**（枚举）：`impl$1::from`（0xa59890）按变体建 rhapsody
  `TextureDesc::new_2d/sampled/storage/color_attachment/depth_stencil_attachment` +
  `mip_levels`（变体 8），带 20 个 hash 常量（资源 id）。**`TextureFromDisk`**：磁盘纹理
  事件（U）。rhapsody = 复用边界。

## time_of_day.rs（O：TodSequence 插值）

- **`TimeOfDay`**：`hour`（内联）。**`TodSequence`**：`{ entries: ptr @+0x8, len @+0x10 }`
  条目（0x14 f32=0x50B 步长：hour 起止 + TodLightingSnapshot 参数）。
  `snapshot_at_tod`（0xb067d0）：`fmodf(tod,1)` 定位段，`inverse_lerp` 两段
  `TodLightingSnapshot::lerp` + `SunSkyColors::lerp` → 插值快照。
- **`TodLightingSnapshot`**（0x44B）：`{ 4×f32 @+0x0, 4×f32 @+0x10, 2×u64 @+0x20,
  4×f32 @+0x30, f32 @+0x40 }`（天空色/太阳参数）。`lerp`、`snapshot_lerp`（内联）。
- **`TodEasing`**（默认 0xb06790 用 fastrand 选段）、**`TodSequenceEntry`**、**`TempUiId`**。

## toolbar.rs（O：默认工具栏 11 类）

- **`Toolbar`**：`new`（0xaf8c50）：11 个类别（Vec<CategoryModes>，stride 0x20 表），
  各含模式表（stride 0x10：`{cat: u32, mode: u8, arg: u64}`）：类 6×[1,2,0]、7、
  8×[1,2,3]、0xb×[0x38..0x3d]、0xc、9×[1,2,3,5,6,4]、0x10、10、0xf、0x11、0x13；
  demo 模式（DEV_OVERRIDE_IS_DEMO_MODE）精简。`switch_mode`（0xaf9850）：查含类 → 设
  `+0x18 当前索引 + 负深度`；未找到 panic。`switch_category`（内联）。
- **`ToolbarCategoryModes::contains`**（0xaf89d0）：`(cat,mode)` 是否在类模式集。
  `next_mode`/`last_selected_mode`（内联）。
- **`ToolbarCategory`**（枚举）、**`ToolbarUiIsInFocus`**、**`ResetToolbar`**。

## ui_settings.rs（O：工具栏对齐 + 默认）

- **`ToolbarAlignment`**（枚举）：`is_horizontal`（0xb0e070）、`fmt`（0xb0e080，
  lang_lib 本地化名 0x28/0x26/0x27/0x25）。
- **`UiSettings`**：`Default`（0xb12f10）：`{ +0x0: f32=1.0, +0x4: f32=24.0,
  +0x8: u32=0x00000101, +0xc: u8=0 }`（工具栏缩放/间距，I）。RON/JSON 序列化。

## render/nani.rs（O：mod 胶水）

- 骨架 `render/nani.rs`；PDB 实体 `GpuDrivenDrawable` / `InstanceHandle`。函数
  `InstanceHandle::subset`、`impl$3::component_ids`、`impl$5::get_components`、
  `impl$2::clone`。共享 crate `render/nani/`（wp1-6）已有；本文件为同面骨架
  （nani ECS 域 gpu_world/mesh_atlas 的 handle/组件 id 表面）。

## systems/save/save_backup_metronome.rs（O：节拍器，内联于 save_game/load_game）

- **`SaveBackupMetronome`**：备份节流节拍器。函数 `add_active_game_time`、
  `consider_backing_up`（closure$0）、`new_from_backup_list` —— 全部**内联**于
  `save_game`（0x140a3f470，行 18/23/55）与 `load_game`（0x140146d030，行 77）
  （PDB inline_sites 证据）。语义：累积活动游戏时间，达到阈值时触发备份
  （`SaveBackupList::backup_metronome_counter` 关联，wp13）。内部字段/阈值 U
  （typed gap；业务逻辑在 save_game 主体内，wp13 已恢复 save_game）。

## systems/garden/draw_garden.rs（O：花园画笔系统 0xae4620）

- **`draw_garden`**：读工具状态（类 6 花丛/4 橡皮），橡皮分支（UI state clearing 层）；
  花丛分支：`Circle2d::new` 半径（`stored + 1.5`）、`GladeBorder::is_shape_inside`，
  不越界则推事件、置画笔状态字节（0/2/3），最后
  `world_raster::draw_raster<GardenRasterStagingSsbo, WorldRaster<512>, c0, c1, c2>`
  绘制到花园掩码光栅（世界光栅 512² 域）。事件/UI 队列为 Bevy/共享 crate 边界。

## systems/garden/record_garden_history.rs（O：0x1695e10）

- **`record_garden_history`**：鼠标按下时把当前花园编辑记录（`lVar8*0xc` 字节 = 12B 编辑
  单元 Arc）封装成历史项，`resources::history::History::add`（共享 crate wp13）。
- **`RecordGardenEditToHistoryCmd`**（PDB 实体）。

## 输入 / 输出 / 未知汇总

- 大量模块为 UI/资产/存档/输入资源（O 名 + 部分布局），业务逻辑面已列；深层字段名
  多数 U → typed gap。steamworks/rhapsody/lang-lib/winit/parry3d 为复用边界。
