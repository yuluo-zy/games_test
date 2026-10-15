# systems/ui/resources/ui_state.rs

> 文件作用：UiState 权威注册表：RwLock<HashMap<WidgetId, Arc<WidgetParams>>>（锁字@+0x60、表@+0x68）；UiWidgetContext（hover@0x18/click@0x19/id@0x38/交点载荷@0x28..0x58）；clearing_canvas 专用 widget（130×130 体积，tag 4）。

函数总计 39：业务 26 / 内联无独立符号 10 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::resources::ui_state::WidgetId::from_name` | 0x14092df30 | 18 | WidgetId::from_name | O：= utils::calculate_hash(name)（u64 name-hash 身份） | ui-input\decomp\decomp-0x14092df30.txt（已补；16B@+8 载荷 → FUN_1405bdda0=calculate_hash） |
| `country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound` | 0x14092df50 | 63 | 注册 clearing_canvas widget | O：体积 0x43020000²=130×130（tag 4）、哈希名、alloc 0xc0 Arc、RwLock 插表（1e9ns 写锁超时） | ui-input\decomp\decomp-0x14092df50.txt |
| `country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer` | 0x14092e340 | 86 | 注册 clearing_canvas widget（带 layer） | O：转发 _w_layer_and_sound（layer 进 WidgetVolumeShape.layer） | ui-input\decomp\decomp-0x14092e340.txt（已补；5B 尾调转发器） |
| `country_core::systems::ui::resources::ui_state::UiState::clearing_canvas` | 0x14092e390 | 95 | 注册 clearing_canvas widget（默认 layer） | O：转发 _w_layer_and_sound | wp15/analysis/b5_ui_state/clearing_canvas_0x14092e390.c |
| `country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>` | 0x1409231f0 | 103 | add<String> 单态 | O：String 名字单态（name → hash → insert） |  |
| `country_core::systems::ui::resources::ui_state::UiState::add<ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit> >` | 0x140923490 | 103 | add<UiRectangleDimsEdit> 单态 | O：类型单态（widget 名来自类型名 hash；wall 域矩形尺寸编辑注册） | ui-input\decomp\decomp-0x140923490.txt |
| `country_core::systems::ui::resources::ui_state::UiState::add<ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit> >` | 0x140922fb0 | 103 | UiState::add(name, params) | O：54 callers——WidgetId::from_name + add_with_id | ui-input\decomp\decomp-0x140922fb0.txt |
| `country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> >` | 0x1409236d0 | 103 | add<&str> 主单态 | O：add 的实际实现体（ui_edit_focus 0x1409771f0 经此注册 'edit_focus_canvas'；0x11=17 为名字长度参数） |  |
| `country_core::systems::ui::resources::ui_state::UiState::add<system_decorator::decorator_resources::DecoratorHandleName>` | 0x14125d180 | 103 | add<DecoratorHandleName> 单态 | O：system_decorator 域单态（decorator 手柄名注册） | ui-input\decomp\decomp-0x14125d180.txt |
| `country_core::systems::ui::resources::ui_state::UiState::add_with_id` | 0x14092e3e0 | 111 | UiState::add_with_id | O：LOCK [p+0x60]==0→写 8 取写锁；拷贝 0xb0B WidgetParams；alloc 0xc0B Arc；HashMap@+0x68 insert | ui-input\decomp\decomp-0x14092e3e0.txt |
| `bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >` | 0x140002450 | 204 | active_widget_state | O：折叠单行（NotFound），语义=取当前 active widget 状态对（U 细节） | ui-input\decomp\decomp-0x140002450.txt |
| `country_core::asset_libraries::mesh_library::AssetMeshLibrary::transient_meshes` | 0x140002450 | 204 | active_widget_state | O：折叠单行（NotFound），语义=取当前 active widget 状态对（U 细节） | ui-input\decomp\decomp-0x140002450.txt |
| `country_core::systems::ui::resources::ui_state::UiState::active_widget_state` | 0x140002450 | 204 | active_widget_state | O：折叠单行（NotFound），语义=取当前 active widget 状态对（U 细节） | ui-input\decomp\decomp-0x140002450.txt |
| `country_core::systems::ui::resources::ui_state::UiState::widget` | 0x14067dd80 | 234 | UiState::widget 查询 | O：1177 callers（ICF 折叠——与 AssetMeshLibrary::transient_meshes/PendingEguiContextResource::into_egui_context_resource 同码；按 WidgetId 查 Arc<WidgetParams>） | ui-input\decomp\decomp-0x14067dd80.txt |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked` | 0x14092e5f0 | 246 | UiWidgetContext::clicked | O：hover>1 && click<2 && click&1（wp15 合同；尾含 id 匹配 sete） | hex 手解（NotFound；`cmp byte[+0x18],2/jb`、`cmp dl,1/ja`、`test dl,1/jz`、id@0x38 匹配） |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered` | 0x14092e620 | 250 | UiWidgetContext::hovered | O：hover **==1** 且 id@+0x38 == ctx id，返回值=byte@+0x19（click 字节）（review-audit 纠错：hover==1 非 !=0） | hex 手解（NotFound；`cmp byte[+0x18],1/jne`、`cmp [+0x38],[self+8]/jne`、`movzx eax,byte[+0x19]`） |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged` | 0x14092df10 | 254 | UiWidgetContext::dragged（与 into_drag_state 同址折叠） | O（round3 round2 复查修正：原“折叠单行（NotFound）”证伪）：真实 26B 条件函数（默认返 2；[+0x18]>=2 且 [+0x38]==[self+8] 时返 byte@+0x19）；S_PUB32 该址含 dragged 与 IntoHandleDragState 两符号 | rne-read 0x14092df10；S_PUB32 |
| `country_core::systems::ui::resources::slider3d::impl$1::into_drag_state` | 0x14092df10 | 255 | into_drag_state（与 dragged 同址折叠） | O（round3 round2 复查修正：原“折叠（NotFound）”证伪）：真实 26B 条件函数（默认返 2；状态 tag@+0x18>=2 且 @+0x38==self+8 时返 byte@+0x19）；“语义=DragState::is_started”未证实（判别值映射需枚举证据） | rne-read 0x14092df10；PDB size 26 |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting` | 0x14092e640 | 259 | UiWidgetContext::intersecting | O：交点载荷 7 qword 拷贝或 tag-2 None | hex 手解（NotFound；hover==0/id 不匹配 → `mov dword[rax],2` None；否则 7 qword 拷贝 @+0x28..0x58 → +0..0x30） |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active` | 0x14092e680 | 264 | UiWidgetContext::is_active | O：hover!=0 && id 匹配 && **dword@+0x28 != 2**（review-audit 纠错：缺 [+0x28]!=2 门控） | hex 手解（NotFound；`cmp byte[+0x18],0/jz`、`cmp [+0x38],[self+8]/jne`、`cmp dword[+0x28],2/setne`） |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::id` | 0x1406868b0 | 274 | UiWidgetContext::id | O：返回 ctx 的 WidgetId（NotFound 折叠，语义由调用点证实） | ui-input\decomp\decomp-0x1406868b0.txt |
| `country_core::systems::ui::resources::ui_state::ActiveWidget::is_dragging` | 0x14092e6a0 | 299 | ActiveWidget::is_dragging | O（round3 复查修正）：机器码 `cmp byte[rcx],2; sete al` = 判别==2（is_dragging；与 session-save 章同址 tag==2 记录一致） | rne-read 0x14092e6a0 机器码 |
| `country_core::systems::ui::resources::ui_state::HoverState::is_some` | 0x1408c5680 | 339 | HoverState::is_some | O：折叠单行（NotFound）= 判别非 None（hover 字节非 0） | wall\decomp\decomp-0x1408c5680.txt |
| `country_core::systems::ui::resources::ui_state::DragState::is_some` | 0x14092e6b0 | 355 | DragState::is_some | O：折叠 = 判别非 None | camera-time\decomp\decomp-0x14092e6b0.txt |
| `country_core::systems::ui::resources::ui_state::DragState::is_started` | 0x14092e6c0 | 364 | DragState::is_started | O：**单一判别 byte[rcx]==3**（sete；与 GladeType::is_autumn ICF 折叠同址；review-audit 纠错："∈{Started,Dragging}"不被机器码支持） | wp15/analysis/b5_ui_state/is_started_0x14092e6c0.c |
| `country_core::systems::ui::resources::ui_state::DragState::is_finished` | 0x14092e6d0 | 372 | DragState::is_finished | O：== Finished（wp15 b5 既有） | wp15/analysis/b5_ui_state/is_finished_0x14092e6d0.c |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::resources::ui_state::impl$1::to_widget_name_hash`（行 43）— 待推断
- `country_core::systems::ui::resources::ui_state::UiState::new`（行 55）— 构造
- `country_core::systems::ui::resources::ui_state::UiState::clicked`（行 116）— 待推断
- `country_core::systems::ui::resources::ui_state::UiState::hovered`（行 133）— 待推断
- `country_core::systems::ui::resources::ui_state::UiState::dragged`（行 150）— 待推断
- `country_core::systems::ui::resources::ui_state::UiState::intersecting`（行 168）— 几何/碰撞
- `country_core::systems::ui::resources::ui_state::UiState::is_active`（行 183）— 谓词/校验
- `country_core::systems::ui::resources::ui_state::UiState::is_widget_name_active`（行 188）— 谓词/校验
- `country_core::systems::ui::resources::ui_state::UiState::current_intersection`（行 195）— 查询/计算
- `country_core::systems::ui::resources::ui_state::UiState::origin`（行 212）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`fmt`、`hash` 等 3 项

## 实现方式（算法级）

- `UiState` = RwLock `HashMap<WidgetId, Arc<WidgetParams>>`（锁字@+0x60、表@+0x68）：
  - `add<&str>` @0x1409236d0 / `add<String>` @0x1409231f0（O）：名字 → `WidgetId::from_name`（hash）→ `add_with_id`。`add<ref$UiRectangleDimsEdit>` @0x140923490、`add<DecoratorHandleName>` @0x14125d180 为类型名 hash 单态。
  - `add_with_id` @0x14092e3e0（O）：`LOCK [p+0x60]==0→写 8` 取写锁（未取到走 `FUN_1428cb5f0` 超时 1e9ns）；拷贝 0xb0B WidgetParams；alloc 0xc0B ArcInner（strong/weak=1）；`FUN_140fb17e0` HashMap insert；旧 Arc 递减；`[p+0x60]==8→写 0` 释放写锁。
  - `clearing_canvas_w_layer_and_sound` @0x14092df50（O）：体积 130×130（`0x43020000` 双 f32）、hash 名、alloc 0xc0 Arc、插表（tag 4）；`_w_layer` @0x14092e340 / `clearing_canvas` @0x14092e390 为其转发。
  - `widget` @0x14067dd80（O）：1177 callers，ICF 折叠——按 WidgetId 查 Arc。
- UiWidgetContext 谓词（O，review-audit 纠错版）：`clicked` @0x14092e5f0 = `hover>1 && click<2 && click&1`；`hovered` @0x14092e620 = `hover==1 && id@+0x38==ctx id`，返回 click 字节@+0x19；`dragged` @0x14092df10（见 slider3d 章）；`intersecting` @0x14092e640 = 交点载荷 7 qword 拷贝或 tag-2 None；`is_active` @0x14092e680 = `hover!=0 && id 匹配 && dword@+0x28 != 2`。
- HoverState/DragState 判别（O）：`HoverState::is_some` @0x1408c5680（非 None）；`DragState::is_dragging` @0x14092e6a0（`byte==2`）、`is_started` @0x14092e6c0（`byte==3`）、`is_finished` @0x14092e6d0（Finished）、`is_some` @0x14092e6b0。
- `active_widget_state` @0x140002450（O/U）：ICF 折叠单行，语义=取当前 active widget 状态对，细节 U。
