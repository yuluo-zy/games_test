# 游戏领域 shader 来源边界审计

本范围核查了原版 source dependencies 中 202 个 shader main/include：wall、roof、buildings、nani、terrain_editing、rocky_terrain、garden、grass、ivy、critters、paths、clearing、相关实例化道具，以及 water 物体放置/水mask/waves、painterly/vermeer、旗帜、粒子、鼠标/地形查询与补充材质声明。读取原始发行 `.bin`，逐个验证 SHA-256，然后用原 OpString/OpLine 关联指令所属源文件，读取原 OpName/OpMemberName、结构 member offset 与数组 stride。恢复 HLSL、恢复 README 和恢复注释未用作来源判断。

逐文件结果在 `classification.json`；完整原始证据在 `raw_game_evidence.json`，扫描与分类入口为 `scan_game.py`、`classify_game.py`。202 条中，134 条归为 game_specific，60 条为 game_adapter，8 条为 generic_unresolved。数量是本次分类覆盖，不是原创行数/开源占比，也不是完整 shader 目录的总分类。187 个源文件有独立可归属 OpLine 指令区域。

这里 game_specific 的准确含义是“原版有游戏专属接口与实现策略的正面证据”。它不等于“每行由开发者原创、所有公式自行发明”。game_adapter 指通用操作与游戏数据的接入、包装或声明；generic_unresolved 表示通用算法辅助代码的精确源码血统没有得到确认。

## 原始证据对应的领域规则

| 范围 | 原版可直接观察的契约/指令区域 | 可作出的判断与边界 |
|---|---|---|
| wall 砖墙 | `wall/inc/brick_vert.hlsl` 原源行 27–278 有274条归属指令；读取 `instanced_wall_data`、`instance_list`、`wall_color_ids`、`roof_color_ids`。原 `InstancedWallData` 的 member 为 `transform`(0)、`sin_deform_y`(48)、`global_arch_height_vals`(64)、`seed`(76)、`source_id`(80)、`wallspace_x_range`(84)、`flags`(92)。Globals 有 `highlight_bricks_to_delete`、`brush_position`、`eraser_mult`。 | 砖实例变形、源墙身份、拱高、删除刷高亮是具体游戏接入与规则。矩阵运算、PBR、普通贴图公式不由此证明原创。两个 wall main 仅作入口包装，实现在 brick vert/frag include 中。 |
| roof 与楼板 | roof main 读取 `roof_color_ids`/`roof_colors`；`roof/dormer_hole.inc.hlsl` 原行37–97有104条指令读取 `dormer_hole_count`/`dormer_holes`；spire 顶点输入有 `roof_profile_mult`。stone/wooden floor 接入 `brick_colors`/`halftimber_colors`。 | 屋顶形状、天窗洞与游戏材质库是游戏专属实现契约；通用材质数学另保留来源边界。 |
| buildings/garden/path | `rasterize_buildings` 读取 `buildings_buffer` 并写两张mask；garden marching squares 原行33–286有1625条指令，联用 `garden_mask`、`foliage_exclusion_mask`、`path_mask`、water/rocky terrain维度与自己的 contouring grid。path visual 联用三种 cobblestone mask 与 dirt mask。 | 多mask相互排除、花园采样/摆放与路径视觉输出是游戏策略。marching squares 是通用算法名称；未据此断言算法核心原创。高斯blur是adapter。 |
| nani | `generate_draw_lists` 原行38–111有106条指令，操作 `mesh_histogram`、`thread_group_draw_cmd_histogram`、`sorted_instances_buffer`、`atlas_part_info`。PushConstants 为 `world_subset`/`draw_list_index`/`list_draw_cmd_offset`/两级 merge window offset；MeshAtlasPartInfo 为 first_index/index_count/bounding_sphere_radius。各自的 cull、sorted/unsorted instance与渲染入口接到游戏实例类型。 | 自己的mesh atlas、过滤实例、draw list联接有正面证据。通用prefix scan、clear和indirect-command转换归adapter，不能把GPU驱动几何的公共理念说成游戏原创。 |
| terrain_editing/rocky terrain | terrain stroke原行32–460有506条指令；Globals含 `stroke_width`、`stroke_infos_device_address`、`stroke_count`、`stroke_indices_dims`、`terrain_sdg_dims`、`water_mask_dims`。stairs placement接自己draw commands；highlight读取 active hill与color library。 | 地形编辑笔划、选择与台阶接入策略游戏专属。SDF、isolines、通用marching-squares include、triangle adjacency保留generic_unresolved。 |
| vegetation | grass分块/计数/生成读取 tile、blade prefix、grasslet ranges；ivy branch/leaf读取自己的 `ivy_buffer`/`ivy_leaf_buffer`；树冠顶点有 `prim_center`/`prim_normals`/`appear_pos`/`age`，花朵有 `flower_pivot`/`stem`/`wind_bend`/季节色输入。 | 实例化布局、显现/季节/风输入与物种契约游戏专属。一般噪声、法线、随机采样、alpha discard来源未逐行确认。 |
| critters | duck原行28–112有154条指令，其Globals有 `anim_t`(144)、`landing_t`(148)、`is_landing`(152)、`is_taking_off`(156)、`pet_t`(160)、`pet_t_is_playing_forward`(164)。顶点包含 rest_pos/rest_n/is_wing/wing_t/is_mouth_bottom/pet_pos/pet_n。firefly读取可见性buffer，butterfly读取自己的buffer/is_wing。 | 起降与抚摸形变输入是游戏专属；向量变换/普通动画数学可仍复用通用知识。 |
| water摆放与waves | boat原行22–140有149条指令，结合 `water_mask`/`buildings_mask`/`path_mask`输出间接draw；lilypad结合garden/动画，reed结合building exclusion。waves原行36–191有335条指令，读 `water_noise_texture` 与 `ripple_source_buffer`。 | 摆放策略、水面扰动源接入游戏专属；不能据waves名将所有波形/噪声判为原创，水渲染/软件BVH8部分由另外调查分类。 |
| painterly/vermeer | painterly generate原行27–157有184条指令，其push constants为 strokes_captured/strokes_this_frame/current_t；vermeer generate原行53–391有531条指令，Globals有 max_strokes/wall_time_seconds/progress_t/history_valid及目标/背景尺寸。 | 逐帧笔触生成与捕获时序有具体集成证据。convolution、acquire、combine归adapter；没有断言整个painterly算法没有外部来源。 |

## 混合边界

原始 source attribution 可将领域代码和 include 内发射区域区分。比如 wall fragment 的来源是 wall/brick_frag，而共用 frame_constants 的布局是另一个来源层。已确认的 SPD、XeGTAO、Kajiya reservoir、GPU-Raytracer BVH8 不因某个consumer包含它们，就成为该consumer全部代码的作者。

原GPU-Raytracer BVH8只在另报的四个 GI/debug/water/ice tracing模块得到独特指令证据。ray_cone 仍未确认精确源码来源。根调查会给每条文件合并 confirmed helper 的 emitted dependency 与依赖闭包；本范围的 classification 仅评价各自领域代码/接入层，不把共同include计数当实际开源贡献。

15 个源文件没有独立可归属的 OpLine发射区，包括一些 defs/flags/layout声明、building curved helper与wall入口包装。对此归game_adapter低置信，并显式标记源依赖证明与实现正文缺失；没有从“目录名看起来属于游戏”推断完整原创。通用SDF等helper及补充 mei_shadow/star 共8项保留generic_unresolved。本轮没有执行“所有公共仓库都没找到”的负面证明，也没有估算每行作者比例。

## 27 项补充审计

已保留最初175条分类原样，追加27条，新增16个game_specific、9个game_adapter、2个generic_unresolved。

- 旗帜三项有独立源行归属：roof flag包含动画贴图/法线与flag palette；wall flag具有 `local_start_y`(84)、`body_end_y`(88)、`tip_end_y`(92)、`is_corner`(96)，支持贴墙旗帜形变/接角的专属契约，不能外推普通布料数学原创。
- grass生成联用 `grasslet_bundle_ranges`、`bundle_buffer`、`tile_buffer`、`garden_mask`、`buildings_mask`、`grass_object_push`。grass main Globals包含terrain高度图尺寸/grass_buffer_size。grass_lod1没有独立源行正文区，作为入口adapter；grass_object_push只有通用位置半径契约，保留adapter。
- mouse_terrain_raycast包含 `mouse_ray_origin`/`mouse_ray_dir`与stroke_indices，输出 CsOut{mouse_raycast_position:0,terrain_stroke_index:16}；read_terrain_stroke、timed terrain gizmo有对应的笔划/箭头输入。read_mouse_depth_value采用depth_map、mouse_position_screenspace与filter_box_y_frac，分类接入adapter，精确深度滤波来源未知。
- snow粒子有 `snow_strength`(80)、`is_photomode`(84)、`is_topdown_mode`(88)与DofFocus，支持游戏照片/俯视模式接入。sparkles有SparkleInstanceData{position:0,scale:12,time_offset:16,speed_mult:20,color_id:24}；leaf实例布局/发射计数有自己的实现契约，但不声称标准粒子/动画运算原创。rain和splash仅能证实通用参数渲染接入，归adapter。
- plaster、smoke、stairs_floor三个include和grass_lod1缺少独立OpLine正文区，标低置信adapter。wooden_plank原区域读取nani_instance_data和wall_id，支持游戏木板实例契约。mei_shadow与star只有通用屏幕/噪声/帧输入与发射区域，本轮没有精确上游指纹，也没有足够独特游戏规则，归generic_unresolved。

补充来源逐条在同一 classification.json 内，原始资料身份和新增source归属区写入同一 raw_game_evidence.json；未修改共享remaining目录或恢复代码。
