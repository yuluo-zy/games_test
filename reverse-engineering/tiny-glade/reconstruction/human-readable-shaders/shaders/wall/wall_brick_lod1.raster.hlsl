// ============================================================================
// wall_brick_lod1.raster.hlsl —— 人类可读重写版
// 原版由 tools/render-passes/batch_lift.py 从 oracle bin 提升(spirv-cross 转译,
// 变量名为 SPIR-V 编号),本文件在"逐字保留算术/控制流/绑定/守卫"的前提下,
// 把编号变量改成语义化英文名,并加中文注释。
//
// 用途:
//   墙砖 LOD1 栅格化着色器。
//   顶点阶段(vs_main):从 instance_list 取墙实例数据,对墙体平移做 8 角点哈希噪声
//   抖动(三线性插值),把相机相对砖原点的偏移按符号翻转让砖角朝向相机(透视内缩),
//   应用拱形高度 / 倒角凹凸变形,最后把砖颜色 id、UV 抖动、高度等打包进 TEXCOORD。
//   片元阶段(ps_main):在砖局部空间做光线求交(先求进入 t,再沿 4 个平面滑窗求真正
//   退出的砖面),用二维 simplex 值噪声区分砖缝,采样 brick_colors / roof_colors 图集
//   并按墙色库、釉面、主题、编辑高亮做混合,最后把反照率(开平方 + 8bit 量化)与
//   法线打包进 uint2 SV_Target0,同时写 SV_DepthLessEqual 做深度提前写入。
//
// 入口:
//   vs_main(SPIRV_Cross_Input) -> SPIRV_Cross_Output   (顶点阶段)
//   ps_main(SPIRV_Cross_Input) -> SPIRV_Cross_Output   (片元阶段)
//
// 变体(TGV_ 哈希,由构建管线用 -D TGV_<hash> 选择,阶段由 __SHADER_TARGET_STAGE 守卫):
//   TGV_b903e43ffb3da915 : vs_main + ps_main —— 完整版(带色板颜色 id / 釉面混合 / 主题)
//   TGV_f0adff76827e73ff : vs_main + ps_main —— 简化版(无颜色 id;片元只做两层高度裁剪)
//
// 关键绑定:
//   cbuffer frame_constants   : register(b0,  space1) —— 每帧常量(含 5 组视图矩阵)
//   cbuffer $Globals          : register(b13, space0) —— 当前视图索引 / 高亮模式 / 擦除半径
//   cbuffer color_library     : register(b21, space0) —— 仅 b903e 片元:墙体/苔藓基色等
//   ByteAddressBuffer wall_color_ids    : t14(仅 b903e 顶点)    instanced_wall_data : t19
//   ByteAddressBuffer roof_color_ids    : t18(仅 b903e 顶点)    instance_list       : t20
//   Texture2DArray brick_colors : t15(仅 b903e 片元)   roof_colors : t17(仅 b903e 片元)
//   SamplerState sampler_llr   : s1(仅 b903e 片元)
//
// 结构体:SPIRV_Cross_Input / SPIRV_Cross_Output / ViewConstants / Affine3Packed /
//   InstancedWallData / PlaneEq —— 成员名原样保留。
// 重写约束:仅"改名 + 中文注释 + 排版";算术、常量、控制流、入口签名逐字保留。
// ============================================================================

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// [变体 b903e43ffb3da915 | 顶点阶段] 完整版顶点:8 角点哈希噪声抖动 + 砖角朝向相机 + 拱形/倒角变形 + 颜色 id。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

static float zero;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

ByteAddressBuffer wall_color_ids : register(t14, space0);
ByteAddressBuffer roof_color_ids : register(t18, space0);
ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
    float4 gl_Position : SV_Position;
};

// 顶点主函数:计算墙体平移抖动、砖角定位与变形,输出各 TEXCOORD。
void vert_main()
{
    // 实例槽位:由 highlight_mode(0/1)与实例索引拼出 instance_list 偏移。
    uint instance_slot = ((1u + (2u * uint(highlight_mode != 0u))) * 2097152u) + uint(gl_InstanceIndex);
    float transform_data[12];
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        transform_data[i] = instanced_wall_data.Load<float>(i * 4 + instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 0);
    }
    float3x4 wall_xform = transpose(float4x3(float3(transform_data[0], transform_data[1], transform_data[2]), float3(transform_data[3], transform_data[4], transform_data[5]), float3(transform_data[6], transform_data[7], transform_data[8]), float3(transform_data[9], transform_data[10], transform_data[11])));
    float3 wall_pos = float3(wall_xform[0].w, wall_xform[1].w, wall_xform[2].w);
    // 把墙体平移位置缩放到噪声格尺度(0.4 格/单位),下面 8 个角点的哈希在此格上进行。
    float3 noise_world = wall_pos * 0.4000000059604644775390625f;
    // 噪声格坐标 = floor(缩放后位置);frac 作为三线性插值权重。
    int3 noise_cell = int3(floor(noise_world));
    float3 noise_frac = frac(noise_world);
    uint3 noise_cell_u = uint3(noise_cell);
    uint n0_z = noise_cell_u.z;
    uint n0_z1 = n0_z + (n0_z << 10u);
    uint n0_z2 = n0_z1 ^ (n0_z1 >> 6u);
    uint n0_z3 = n0_z2 + (n0_z2 << 3u);
    uint n0_z4 = n0_z3 ^ (n0_z3 >> 11u);
    uint n0_y0 = (((noise_cell_u.y * 1664525u) + (n0_z4 + (n0_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n0_y1 = n0_y0 ^ (n0_y0 >> 11u);
    uint n0_y2 = n0_y1 ^ ((n0_y1 << 7u) & 2636928640u);
    uint n0_y3 = n0_y2 ^ ((n0_y2 << 15u) & 4022730752u);
    uint n0_x0 = (((noise_cell_u.x * 1664525u) + (n0_y3 ^ (n0_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n0_x1 = n0_x0 ^ (n0_x0 >> 11u);
    uint n0_x2 = n0_x1 ^ ((n0_x1 << 7u) & 2636928640u);
    uint n0_x3 = n0_x2 ^ ((n0_x2 << 15u) & 4022730752u);
    uint n0_hashA = n0_x3 ^ (n0_x3 >> 18u);
    uint n0_s0 = n0_hashA + (n0_hashA << 10u);
    uint n0_s1 = n0_s0 ^ (n0_s0 >> 6u);
    uint n0_s2 = n0_s1 + (n0_s1 << 3u);
    uint n0_hashB = n0_s2 ^ (n0_s2 >> 11u);
    // 噪声角点 (1,0,0):每个角点用整数哈希链生成两个 [0,1) 随机数。
    uint3 n1_cell = uint3(noise_cell + int3(1, 0, 0));
    uint n1_z = n1_cell.z;
    uint n1_z1 = n1_z + (n1_z << 10u);
    uint n1_z2 = n1_z1 ^ (n1_z1 >> 6u);
    uint n1_z3 = n1_z2 + (n1_z2 << 3u);
    uint n1_z4 = n1_z3 ^ (n1_z3 >> 11u);
    uint n1_y0 = (((n1_cell.y * 1664525u) + (n1_z4 + (n1_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n1_y1 = n1_y0 ^ (n1_y0 >> 11u);
    uint n1_y2 = n1_y1 ^ ((n1_y1 << 7u) & 2636928640u);
    uint n1_y3 = n1_y2 ^ ((n1_y2 << 15u) & 4022730752u);
    uint n1_x0 = (((n1_cell.x * 1664525u) + (n1_y3 ^ (n1_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n1_x1 = n1_x0 ^ (n1_x0 >> 11u);
    uint n1_x2 = n1_x1 ^ ((n1_x1 << 7u) & 2636928640u);
    uint n1_x3 = n1_x2 ^ ((n1_x2 << 15u) & 4022730752u);
    uint n1_hashA = n1_x3 ^ (n1_x3 >> 18u);
    uint n1_s0 = n1_hashA + (n1_hashA << 10u);
    uint n1_s1 = n1_s0 ^ (n1_s0 >> 6u);
    uint n1_s2 = n1_s1 + (n1_s1 << 3u);
    uint n1_hashB = n1_s2 ^ (n1_s2 >> 11u);
    // 噪声角点 (0,1,0) 的哈希链。
    uint3 n2_cell = uint3(noise_cell + int3(0, 1, 0));
    uint n2_z = n2_cell.z;
    uint n2_z1 = n2_z + (n2_z << 10u);
    uint n2_z2 = n2_z1 ^ (n2_z1 >> 6u);
    uint n2_z3 = n2_z2 + (n2_z2 << 3u);
    uint n2_z4 = n2_z3 ^ (n2_z3 >> 11u);
    uint n2_y0 = (((n2_cell.y * 1664525u) + (n2_z4 + (n2_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n2_y1 = n2_y0 ^ (n2_y0 >> 11u);
    uint n2_y2 = n2_y1 ^ ((n2_y1 << 7u) & 2636928640u);
    uint n2_y3 = n2_y2 ^ ((n2_y2 << 15u) & 4022730752u);
    uint n2_x0 = (((n2_cell.x * 1664525u) + (n2_y3 ^ (n2_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n2_x1 = n2_x0 ^ (n2_x0 >> 11u);
    uint n2_x2 = n2_x1 ^ ((n2_x1 << 7u) & 2636928640u);
    uint n2_x3 = n2_x2 ^ ((n2_x2 << 15u) & 4022730752u);
    uint n2_hashA = n2_x3 ^ (n2_x3 >> 18u);
    uint n2_s0 = n2_hashA + (n2_hashA << 10u);
    uint n2_s1 = n2_s0 ^ (n2_s0 >> 6u);
    uint n2_s2 = n2_s1 + (n2_s1 << 3u);
    uint n2_hashB = n2_s2 ^ (n2_s2 >> 11u);
    // 噪声角点 (1,1,0) 的哈希链。
    uint3 n3_cell = uint3(noise_cell + int3(1, 1, 0));
    uint n3_z = n3_cell.z;
    uint n3_z1 = n3_z + (n3_z << 10u);
    uint n3_z2 = n3_z1 ^ (n3_z1 >> 6u);
    uint n3_z3 = n3_z2 + (n3_z2 << 3u);
    uint n3_z4 = n3_z3 ^ (n3_z3 >> 11u);
    uint n3_y0 = (((n3_cell.y * 1664525u) + (n3_z4 + (n3_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n3_y1 = n3_y0 ^ (n3_y0 >> 11u);
    uint n3_y2 = n3_y1 ^ ((n3_y1 << 7u) & 2636928640u);
    uint n3_y3 = n3_y2 ^ ((n3_y2 << 15u) & 4022730752u);
    uint n3_x0 = (((n3_cell.x * 1664525u) + (n3_y3 ^ (n3_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n3_x1 = n3_x0 ^ (n3_x0 >> 11u);
    uint n3_x2 = n3_x1 ^ ((n3_x1 << 7u) & 2636928640u);
    uint n3_x3 = n3_x2 ^ ((n3_x2 << 15u) & 4022730752u);
    uint n3_hashA = n3_x3 ^ (n3_x3 >> 18u);
    uint n3_s0 = n3_hashA + (n3_hashA << 10u);
    uint n3_s1 = n3_s0 ^ (n3_s0 >> 6u);
    uint n3_s2 = n3_s1 + (n3_s1 << 3u);
    uint n3_hashB = n3_s2 ^ (n3_s2 >> 11u);
    // 噪声角点 (0,0,1) 的哈希链。
    uint3 n4_cell = uint3(noise_cell + int3(0, 0, 1));
    uint n4_z = n4_cell.z;
    uint n4_z1 = n4_z + (n4_z << 10u);
    uint n4_z2 = n4_z1 ^ (n4_z1 >> 6u);
    uint n4_z3 = n4_z2 + (n4_z2 << 3u);
    uint n4_z4 = n4_z3 ^ (n4_z3 >> 11u);
    uint n4_y0 = (((n4_cell.y * 1664525u) + (n4_z4 + (n4_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n4_y1 = n4_y0 ^ (n4_y0 >> 11u);
    uint n4_y2 = n4_y1 ^ ((n4_y1 << 7u) & 2636928640u);
    uint n4_y3 = n4_y2 ^ ((n4_y2 << 15u) & 4022730752u);
    uint n4_x0 = (((n4_cell.x * 1664525u) + (n4_y3 ^ (n4_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n4_x1 = n4_x0 ^ (n4_x0 >> 11u);
    uint n4_x2 = n4_x1 ^ ((n4_x1 << 7u) & 2636928640u);
    uint n4_x3 = n4_x2 ^ ((n4_x2 << 15u) & 4022730752u);
    uint n4_hashA = n4_x3 ^ (n4_x3 >> 18u);
    uint n4_s0 = n4_hashA + (n4_hashA << 10u);
    uint n4_s1 = n4_s0 ^ (n4_s0 >> 6u);
    uint n4_s2 = n4_s1 + (n4_s1 << 3u);
    uint n4_hashB = n4_s2 ^ (n4_s2 >> 11u);
    // 噪声角点 (1,0,1) 的哈希链。
    uint3 n5_cell = uint3(noise_cell + int3(1, 0, 1));
    uint n5_z = n5_cell.z;
    uint n5_z1 = n5_z + (n5_z << 10u);
    uint n5_z2 = n5_z1 ^ (n5_z1 >> 6u);
    uint n5_z3 = n5_z2 + (n5_z2 << 3u);
    uint n5_z4 = n5_z3 ^ (n5_z3 >> 11u);
    uint n5_y0 = (((n5_cell.y * 1664525u) + (n5_z4 + (n5_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n5_y1 = n5_y0 ^ (n5_y0 >> 11u);
    uint n5_y2 = n5_y1 ^ ((n5_y1 << 7u) & 2636928640u);
    uint n5_y3 = n5_y2 ^ ((n5_y2 << 15u) & 4022730752u);
    uint n5_x0 = (((n5_cell.x * 1664525u) + (n5_y3 ^ (n5_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n5_x1 = n5_x0 ^ (n5_x0 >> 11u);
    uint n5_x2 = n5_x1 ^ ((n5_x1 << 7u) & 2636928640u);
    uint n5_x3 = n5_x2 ^ ((n5_x2 << 15u) & 4022730752u);
    uint n5_hashA = n5_x3 ^ (n5_x3 >> 18u);
    uint n5_s0 = n5_hashA + (n5_hashA << 10u);
    uint n5_s1 = n5_s0 ^ (n5_s0 >> 6u);
    uint n5_s2 = n5_s1 + (n5_s1 << 3u);
    uint n5_hashB = n5_s2 ^ (n5_s2 >> 11u);
    // 噪声角点 (0,1,1) 的哈希链。
    uint3 n6_cell = uint3(noise_cell + int3(0, 1, 1));
    uint n6_z = n6_cell.z;
    uint n6_z1 = n6_z + (n6_z << 10u);
    uint n6_z2 = n6_z1 ^ (n6_z1 >> 6u);
    uint n6_z3 = n6_z2 + (n6_z2 << 3u);
    uint n6_z4 = n6_z3 ^ (n6_z3 >> 11u);
    uint n6_y0 = (((n6_cell.y * 1664525u) + (n6_z4 + (n6_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n6_y1 = n6_y0 ^ (n6_y0 >> 11u);
    uint n6_y2 = n6_y1 ^ ((n6_y1 << 7u) & 2636928640u);
    uint n6_y3 = n6_y2 ^ ((n6_y2 << 15u) & 4022730752u);
    uint n6_x0 = (((n6_cell.x * 1664525u) + (n6_y3 ^ (n6_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n6_x1 = n6_x0 ^ (n6_x0 >> 11u);
    uint n6_x2 = n6_x1 ^ ((n6_x1 << 7u) & 2636928640u);
    uint n6_x3 = n6_x2 ^ ((n6_x2 << 15u) & 4022730752u);
    uint n6_hashA = n6_x3 ^ (n6_x3 >> 18u);
    uint n6_s0 = n6_hashA + (n6_hashA << 10u);
    uint n6_s1 = n6_s0 ^ (n6_s0 >> 6u);
    uint n6_s2 = n6_s1 + (n6_s1 << 3u);
    uint n6_hashB = n6_s2 ^ (n6_s2 >> 11u);
    // 噪声角点 (1,1,1) 的哈希链。
    uint3 n7_cell = uint3(noise_cell + int3(1, 1, 1));
    uint n7_z = n7_cell.z;
    uint n7_z1 = n7_z + (n7_z << 10u);
    uint n7_z2 = n7_z1 ^ (n7_z1 >> 6u);
    uint n7_z3 = n7_z2 + (n7_z2 << 3u);
    uint n7_z4 = n7_z3 ^ (n7_z3 >> 11u);
    uint n7_y0 = (((n7_cell.y * 1664525u) + (n7_z4 + (n7_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n7_y1 = n7_y0 ^ (n7_y0 >> 11u);
    uint n7_y2 = n7_y1 ^ ((n7_y1 << 7u) & 2636928640u);
    uint n7_y3 = n7_y2 ^ ((n7_y2 << 15u) & 4022730752u);
    uint n7_x0 = (((n7_cell.x * 1664525u) + (n7_y3 ^ (n7_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n7_x1 = n7_x0 ^ (n7_x0 >> 11u);
    uint n7_x2 = n7_x1 ^ ((n7_x1 << 7u) & 2636928640u);
    uint n7_x3 = n7_x2 ^ ((n7_x2 << 15u) & 4022730752u);
    uint n7_hashA = n7_x3 ^ (n7_x3 >> 18u);
    uint n7_s0 = n7_hashA + (n7_hashA << 10u);
    uint n7_s1 = n7_s0 ^ (n7_s0 >> 6u);
    uint n7_s2 = n7_s1 + (n7_s1 << 3u);
    uint n7_hashB = n7_s2 ^ (n7_s2 >> 11u);
    // smoothstep 权重:frac^2 * (3 - 2*frac),用于把角点随机值三线性插值。
    float3 smooth_weight = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
    float wx = smooth_weight.x;
    float wx1 = 1.0f - wx;
    float wy = smooth_weight.y;
    float wy1 = 1.0f - wy;
    float wz = smooth_weight.z;
    float wz1 = 1.0f - wz;
    // 8 个角点 2D 随机向量的三线性插值:asfloat((hash & 0x7fffff)|0x3f800000)-1 把哈希高 23 位重解释为 [-1,1) 随机数。
    float2 noise_sum = ((((((float2(asfloat((n0_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n0_hashB + (n0_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy1) * wz1) + (((float2(asfloat((n1_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n1_hashB + (n1_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy1) * wz1)) + (((float2(asfloat((n2_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n2_hashB + (n2_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy) * wz1)) + (((float2(asfloat((n3_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n3_hashB + (n3_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz1)) + (((float2(asfloat((n4_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n4_hashB + (n4_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy1) * wz);
    // 抖动量:中心化到 [-0.5,0.5] 再放大 2.5 并夹到 ±0.5,得到墙面水平 UV 抖动。
    float2 noise_uv = clamp(((((noise_sum + (((float2(asfloat((n5_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n5_hashB + (n5_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy1) * wz)) + (((float2(asfloat((n6_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n6_hashB + (n6_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy) * wz)) + (((float2(asfloat((n7_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n7_hashB + (n7_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    // 把抖动转成 3D 位置偏移:水平面缩放 0.06,垂直方向叠加 sin_deform_y 四分量均值(点积 0.25)。
    float3 pos_offset = (float3(noise_uv.x, 0.0f, noise_uv.y) * 0.0599999986588954925537109375f) + float3(0.0f, dot(instanced_wall_data.Load<float4>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 48), 0.25f.xxxx), 0.0f);
    // 新的墙体平移 = 原平移 + 抖动偏移(写入仿射矩阵的 w 分量)。
    float3 new_pos = wall_pos + pos_offset;
    float3x4 xfm1 = wall_xform;
    xfm1[0].w = new_pos.x;
    float3x4 xfm2 = xfm1;
    xfm2[1].w = new_pos.y;
    float3x4 xfm3 = xfm2;
    xfm3[2].w = new_pos.z;
    // 从顶点索引取 3 位,确定砖块 8 个角中的哪一个(每块砖由 8 个顶点组成)。
    uint3 corner_mask = uint3((uint(gl_VertexIndex) & 4u) >> 2u, (uint(gl_VertexIndex) & 2u) >> 1u, uint(gl_VertexIndex) & 1u);
    float3 brick_origin = mul(xfm3, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 cam_pos4 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 corner_local = mul((cam_pos4.xyz / cam_pos4.w.xxx) - brick_origin, float3x3(xfm1[0].xyz, xfm2[1].xyz, xfm3[2].xyz));
    bool3 corner_flip = bool3(corner_local.x > 0.0f.xxx.x, corner_local.y > 0.0f.xxx.y, corner_local.z > 0.0f.xxx.z);
    uint3 corner_mask_inv = uint3(1u, 1u, 1u) - corner_mask;
    // 砖角方向(±0.5):把相机相对砖原点的局部偏移按符号翻转,让砖角朝向相机,形成透视内缩。
    float3 corner_dir = float3(uint3(corner_flip.x ? corner_mask.x : corner_mask_inv.x, corner_flip.y ? corner_mask.y : corner_mask_inv.y, corner_flip.z ? corner_mask.z : corner_mask_inv.z)) - 0.5f.xxx;
    float4 corner_dir4 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(corner_dir, 1.0f));
    float3 corner_dir3 = corner_dir4.xyz;
    // 墙标志(数据偏移 92):位 5=拱形,位 2=倒角,位 0/1/4/6 等控制砖形与颜色。
    int wall_flags = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 92));
    // 砖角局部坐标(未缩放);若为拱形砖则 z 按高度插值修正。
    float3 corner_pos;
    if ((wall_flags & 32) != 0)
    {
        float3 corner_pos_new = corner_dir3;
        corner_pos_new.z = corner_dir4.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 68), 0.5f - corner_dir4.x);
        corner_pos = corner_pos_new;
    }
    else
    {
        corner_pos = corner_dir3;
    }
    // 砖角最终缩放(倒角凹凸或恒 1)。
    float3 brick_scale;
    // 倒角砖:用随机数选 1.0/1.1 做轴向缩放,若位 0 置位则用 1.05。
    if ((wall_flags & 4) != 0)
    {
        uint bump_h0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) << 10u);
        uint bump_h1 = bump_h0 ^ (bump_h0 >> 6u);
        uint bump_h2 = bump_h1 + (bump_h1 << 3u);
        uint bump_h3 = bump_h2 ^ (bump_h2 >> 11u);
        float bump_rand = ((asfloat(((bump_h3 + (bump_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 bump_scale_vec = float3(bump_rand, bump_rand, 1.0f);
        bool3 bevel_mask = ((wall_flags & 1) != 0).xxx;
        brick_scale = float3(bevel_mask.x ? 1.0499999523162841796875f.xxx.x : bump_scale_vec.x, bevel_mask.y ? 1.0499999523162841796875f.xxx.y : bump_scale_vec.y, bevel_mask.z ? 1.0499999523162841796875f.xxx.z : bump_scale_vec.z);
    }
    else
    {
        brick_scale = 1.0f.xxx;
    }
    float3 brick_pos = mul(xfm3, float4(corner_pos * brick_scale, 1.0f));
    float4 brick_pos4 = float4(brick_pos, 1.0f);
    // 砖在墙面上沿 x 的归一化位置 [0,1](corner_dir.x + 0.5)。
    float brick_u = mul(float3x3(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1.0f, 0.0f), float3(0.0f, 0.0f, 1.0f)), corner_dir).x - (-0.5f);
    // 砖颜色 id = 数据偏移 76 的低 16 位。
    uint color_id = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) & 65535u;
    // 砖标志:若颜色 id < 10 则视为新建/低龄砖,追加 512 标志。
    int brick_flags;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) < 10u)
    {
        brick_flags = wall_flags | 512;
    }
    else
    {
        brick_flags = wall_flags;
    }
    // 拱形高度:把 global_arch_height_vals 三个值按 brick_u 两段线性插值。
    float arch_height;
    if (brick_u < 0.5f)
    {
        arch_height = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).y, brick_u * 2.0f);
    }
    else
    {
        arch_height = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).z, (brick_u * 2.0f) - 1.0f);
    }
    int color_id_out;
    int flags_out;
    // 高位标志:楼梯瞬态边高亮则直接写楼梯边颜色 id;否则从屋顶/墙体色板查颜色 id。
    if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) & 2147483648u) != 0u)
    {
        bool is_stair_hl;
        if ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) & 1073741824u) != 0u)
        {
            is_stair_hl = ((instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) >> 4u) & 65535u) == frame_constants_highlighted_transient_stair_edge_id;
        }
        else
        {
            is_stair_hl = false;
        }
        int brick_flags_hl;
        if (is_stair_hl)
        {
            brick_flags_hl = brick_flags | 2048;
        }
        else
        {
            brick_flags_hl = brick_flags;
        }
        color_id_out = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) & 15u);
        flags_out = brick_flags_hl;
    }
    else
    {
        uint palette_color_id;
        if ((brick_flags & 2) != 0)
        {
            palette_color_id = roof_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) * 4 + 0);
        }
        else
        {
            palette_color_id = wall_color_ids.Load<uint>(instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) * 4 + 0);
        }
        color_id_out = int(palette_color_id);
        flags_out = brick_flags;
    }
    // 实例种子(数据偏移 80 的低 16 位),用于生成 UV 抖动。
    uint instance_seed = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 80) & 65535u;
    uint seed_h0 = instance_seed ^ 7993234u;
    uint seed_h1 = seed_h0 + (seed_h0 << 10u);
    uint seed_h2 = seed_h1 ^ (seed_h1 >> 6u);
    uint seed_h3 = seed_h2 + (seed_h2 << 3u);
    uint seed_hashB = seed_h3 ^ (seed_h3 >> 11u);
    uint seed2_h0 = instance_seed ^ 3450234u;
    uint seed2_h1 = seed2_h0 + (seed2_h0 << 10u);
    uint seed2_h2 = seed2_h1 ^ (seed2_h1 >> 6u);
    uint seed2_h3 = seed2_h2 + (seed2_h2 << 3u);
    uint seed2_hashB = seed2_h3 ^ (seed2_h3 >> 11u);
    // UV 随机抖动:沿墙 x 进度 lerp 墙宽,叠加 y 高度与两个种子哈希随机数。
    float2 uv_offset = (((float2(lerp(instanced_wall_data.Load<float2>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 84).x, instanced_wall_data.Load<float2>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 84).y, brick_u), brick_pos.y) * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(instance_seed) * 0.20000000298023223876953125f).xx) + float2(asfloat(((seed_hashB + (seed_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed2_hashB + (seed2_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint uvj_h0 = color_id ^ 292478890u;
    uint uvj_h1 = uvj_h0 + (uvj_h0 << 10u);
    uint uvj_h2 = uvj_h1 ^ (uvj_h1 >> 6u);
    uint uvj_h3 = uvj_h2 + (uvj_h2 << 3u);
    uint uvj_hashB = uvj_h3 ^ (uvj_h3 >> 11u);
    // 种子哈希随机数:>0.9 的砖(极少数)抖动放大 3 倍。
    float uvj_rand = asfloat(((uvj_hashB + (uvj_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 uv_offset2;
    if (uvj_rand > 0.89999997615814208984375f)
    {
        uv_offset2 = uv_offset * 3.0f;
    }
    else
    {
        uv_offset2 = uv_offset + (0.61000001430511474609375f * uvj_rand).xx;
    }
    // 最终 UV 抖动:带 512 标志(新建砖)再放大 2.2 倍。
    float2 uv_offset_final;
    if (int((flags_out & 512) != 0) == 1)
    {
        uv_offset_final = uv_offset2 * 2.2000000476837158203125f;
    }
    else
    {
        uv_offset_final = uv_offset2;
    }
    // 拱形标志:(flags & 32):拱形砖改为按墙面 xz 抖动。
    bool is_arch = (brick_flags & 32) != 0;
    // 拱形砖的 UV 抖动(或普通砖的 uv_offset_final)。
    float2 arch_uv;
    if (is_arch)
    {
        uint arch_h0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) << 10u);
        uint arch_h1 = arch_h0 ^ (arch_h0 >> 6u);
        uint arch_h2 = arch_h1 + (arch_h1 << 3u);
        uint arch_hashB = arch_h2 ^ (arch_h2 >> 11u);
        arch_uv = ((brick_pos4.xz * 0.699999988079071044921875f) * 0.300000011920928955078125f) + (asfloat(((arch_hashB + (arch_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    }
    else
    {
        arch_uv = uv_offset_final;
    }
    // 输出:世界→采样空间的裁剪位置。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, brick_pos4);
    // 输出 TEXCOORD:颜色 id、位置、抖动、高度、色板 id、标志、实例 id。
    out_var_C0 = color_id;
    out_var_C1 = 0.0f.xxx;
    out_var_C2 = brick_pos4.xyz;
    out_var_C5 = pos_offset;
    out_var_C7 = is_arch ? (-100.0f) : arch_height;
    out_var_C10 = arch_uv;
    out_var_C11 = color_id_out;
    out_var_C12 = float4(brick_origin.x, zero, brick_origin.z, 1.0f).xz;
    out_var_C13 = flags_out;
    out_var_C14 = instance_list.Load<uint>(instance_slot * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// [变体 b903e43ffb3da915 | 片元阶段] 完整版片元:砖块光线求交、砖缝噪声、砖色/釉面/主题/高亮混合,打包输出。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

struct PlaneEq
{
    float4 abcd;
};

static uint zero_u;
static float3 unused_zero3;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

ByteAddressBuffer instanced_wall_data : register(t19, space0);
cbuffer color_library : register(b21, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> brick_colors : register(t15, space0);
Texture2DArray<float4> roof_colors : register(t17, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static uint in_var_C0;
static float3 in_var_C2;
static float3 in_var_C5;
static float in_var_C7;
static float2 in_var_C10;
static int in_var_C11;
static float2 in_var_C12;
static int in_var_C13;
static uint in_var_C14;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 in_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int in_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 in_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint in_var_C14 : TEXCOORD14;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

// 片元主函数:砖块光线求交、砖缝噪声、砖色采样与高亮混合,打包输出。
void frag_main()
{
    // 砖标志:C13 位 8=楼梯,位 16=暗面(主题),位 2048=楼梯高亮,位 4=倒角,位 2=屋顶面,位 512=新建砖。
    bool is_stair = (in_var_C13 & 8) != 0;
    bool flag_dark = (in_var_C13 & 16) != 0;
    bool is_stair_hl = (in_var_C13 & 2048) != 0;
    float3 ddx_c2 = ddx(in_var_C2);
    float3 ddy_c2 = ddy(in_var_C2);
    // 片元面法线:由屏幕空间位置导数叉积近似(未经归一化)。
    float3 face_normal = cross(ddx_c2, -ddy_c2);
    bool is_bevel = (in_var_C13 & 4) != 0;
    float3 brick_scale;
    // 倒角砖:与顶点阶段一致,给砖一个轴向凹凸缩放(1.0/1.1 或 1.05)。
    if (is_bevel)
    {
        uint bev_h0 = instanced_wall_data.Load<uint>(in_var_C14 * 96 + 76) + (instanced_wall_data.Load<uint>(in_var_C14 * 96 + 76) << 10u);
        uint bev_h1 = bev_h0 ^ (bev_h0 >> 6u);
        uint bev_h2 = bev_h1 + (bev_h1 << 3u);
        uint bev_h3 = bev_h2 ^ (bev_h2 >> 11u);
        float bev_rand = ((asfloat(((bev_h3 + (bev_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 bev_scale_vec = float3(bev_rand, bev_rand, 1.0f);
        bool3 bev_mask = ((in_var_C13 & 1) != 0).xxx;
        brick_scale = float3(bev_mask.x ? 1.0499999523162841796875f.xxx.x : bev_scale_vec.x, bev_mask.y ? 1.0499999523162841796875f.xxx.y : bev_scale_vec.y, bev_mask.z ? 1.0499999523162841796875f.xxx.z : bev_scale_vec.z);
    }
    else
    {
        brick_scale = 1.0f.xxx;
    }
    // 随机数重解释用的位模式:1065353216u=0x3f800000(浮点 1.0 的指数位),8388607u=0x7fffff(尾数掩码);非倒角时为 0。
    uint rand_or_bits = is_bevel ? 1065353216u : zero_u;
    uint rand_mask_bits = is_bevel ? 8388607u : zero_u;
    bool use_rand_bits = is_bevel ? true : false;
    // 对角缩放矩阵:把墙仿射矩阵按砖轴缩放(倒角凹凸)。
    float4x4 scale_xform = float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    scale_xform[0].x = brick_scale.x;
    scale_xform[1].y = brick_scale.y;
    scale_xform[2].z = brick_scale.z;
    scale_xform[3].w = 1.0f;
    float transform_data[12];
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        transform_data[i] = instanced_wall_data.Load<float>(i * 4 + in_var_C14 * 96 + 0);
    }
    float3x4 wall_xform = mul(transpose(float4x3(float3(transform_data[0], transform_data[1], transform_data[2]), float3(transform_data[3], transform_data[4], transform_data[5]), float3(transform_data[6], transform_data[7], transform_data[8]), float3(transform_data[9], transform_data[10], transform_data[11]))), scale_xform);
    // 新的墙体平移 = 原平移 + 顶点阶段写入的抖动偏移(in_var_C5)。
    float3 new_pos = float3(wall_xform[0].w, wall_xform[1].w, wall_xform[2].w) + in_var_C5;
    float3x4 xfm1 = wall_xform;
    xfm1[0].w = new_pos.x;
    float3x4 xfm2 = xfm1;
    xfm2[1].w = new_pos.y;
    float3x4 xfm3 = xfm2;
    xfm3[2].w = new_pos.z;
    // 墙基向量(缩放后的 3x3 子矩阵)。
    float3x3 basis = float3x3(xfm1[0].xyz, xfm2[1].xyz, xfm3[2].xyz);
    float3 basis_col0 = mul(basis, float3(1.0f, 0.0f, 0.0f));
    float3 basis_col1 = mul(basis, float3(0.0f, 1.0f, 0.0f));
    float3 basis_col2 = mul(basis, float3(0.0f, 0.0f, 1.0f));
    float3 basis_cross = cross(basis_col0, basis_col1);
    float3 basis_inv_scale = (1.0f / dot(basis_col2, basis_cross)).xxx;
    // 基向量逆矩阵:把世界空间方向/位置变换到砖局部空间。
    float3x3 basis_inv = float3x3(cross(basis_col1, basis_col2) * basis_inv_scale, cross(basis_col2, basis_col0) * basis_inv_scale, basis_cross * basis_inv_scale);
    // 相机世界位置(视角矩阵原点)。
    float4 cam_pos4 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_pos4.xyz / cam_pos4.w.xxx;
    // 光线起点:相机位置在砖局部空间中的坐标(相对砖原点)。
    float3 ray_origin = mul(basis_inv, cam_pos - mul(xfm3, float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 视线方向(砖局部空间,归一化)。
    float3 ray_dir = mul(basis_inv, normalize(in_var_C2 - cam_pos));
    float3 t_near = (0.5f.xxx - ray_origin) / ray_dir;
    float3 t_far = ((-0.5f).xxx - ray_origin) / ray_dir;
    float3 t_min = min(t_near, t_far);
    float3 t_max = max(t_near, t_far);
    // 光线进入砖半体所需的 t(取三条轴 t_min 的最大值)。
    float t_enter = max(max(t_min.x, t_min.y), t_min.z);
    float3 hit_pos = ray_origin + (ray_dir * t_enter);
    float3 abs_hit = abs(hit_pos);
    float3 frag_pos_final;
    float frag_depth_final;
    float3 frag_normal_final;
    // 若命中点在 >0.425 的半体外:沿 4 个候选平面滑窗,求光线真正退出的砖面。
    if (dot(1.0f.xxx, select(bool3(abs_hit.x > 0.425000011920928955078125f.xxx.x, abs_hit.y > 0.425000011920928955078125f.xxx.y, abs_hit.z > 0.425000011920928955078125f.xxx.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f))) >= 2.0f)
    {
        int3 hit_sign = int3(sign(hit_pos));
        float3 hit_sign_f = float3(hit_sign);
        float sx = hit_sign_f.x;
        // 4 个候选退出平面:三个坐标轴面 + 一个 -1.35 的远面。
        PlaneEq plane0 = { float4(0.0f, hit_sign_f.yz, -0.925000011920928955078125f) };
        PlaneEq plane1 = { float4(sx, 0.0f, hit_sign_f.z, -0.925000011920928955078125f) };
        PlaneEq plane2 = { float4(sx, hit_sign_f.y, 0.0f, -0.925000011920928955078125f) };
        PlaneEq plane3 = { float4(sx, hit_sign_f.yz, -1.35000002384185791015625f) };
        PlaneEq planes_src[4] = { plane0, plane1, plane2, plane3 };
        PlaneEq planes[4] = planes_src;
        float3 max_abs = max(abs_hit.x, max(abs_hit.y, abs_hit.z)).xxx;
        float3 hit_face;
        bool face_found;
        float t_exit;
        float t_cur;
        hit_face = float3(int3(bool3(abs_hit.x == max_abs.x, abs_hit.y == max_abs.y, abs_hit.z == max_abs.z)) * hit_sign);
        face_found = false;
        t_exit = min(min(t_max.x, t_max.y), t_max.z);
        t_cur = t_enter;
        bool face_found_new;
        float t_exit_new;
        float t_cur_new;
        float3 hit_face_new;
        // 滑窗循环:对每个候选面求光线交点参数 t,取最近且 > 当前进入 t 的退出面。
        for (uint plane_idx = 0u; plane_idx < 4u; hit_face = hit_face_new, face_found = face_found_new, t_exit = t_exit_new, t_cur = t_cur_new, plane_idx++)
        {
            float t_plane0 = (planes[plane_idx].abcd.w + dot(ray_origin, planes[plane_idx].abcd.xyz)) / (-dot(ray_dir, planes[plane_idx].abcd.xyz));
            float3 plane_normal = (planes[plane_idx].abcd.xyz * (-1.0f)).xyz;
            float t_plane1 = (planes[plane_idx].abcd.w + dot(ray_origin, plane_normal)) / (-dot(ray_dir, plane_normal));
            float t_plane_min = min(t_plane0, t_plane1);
            bool plane_closer = t_plane_min > t_cur;
            if (plane_closer)
            {
                hit_face_new = plane_normal * float(-int(sign(dot(plane_normal, ray_dir))));
            }
            else
            {
                hit_face_new = hit_face;
            }
            face_found_new = plane_closer ? true : face_found;
            t_cur_new = plane_closer ? t_plane_min : t_cur;
            t_exit_new = min(t_exit, max(t_plane0, t_plane1));
        }
        float3 frag_pos;
        float frag_depth;
        float3 frag_normal;
        // 滑窗后有有效退出面:用 t_cur 求世界命中点与深度(供 SV_DepthLessEqual);否则 discard 该片元。
        if (t_cur < t_exit)
        {
            if (face_found)
            {
                float3 hit_world = mul(xfm3, float4(ray_origin + (ray_dir * t_cur), 1.0f));
                float4 hit_clip = mul(frame_constants_view_constants[current_view_index].view_to_sample, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(hit_world, 1.0f)));
                float3 hit_clip_xyz = hit_clip.xyz / hit_clip.w.xxx;
                float hit_depth = hit_clip_xyz.z;
                frag_pos = hit_world;
                frag_depth = (hit_depth < gl_FragCoord.z) ? hit_depth : gl_FragCoord.z;
                frag_normal = mul(hit_face, basis_inv);
            }
            else
            {
                frag_pos = in_var_C2;
                frag_depth = gl_FragCoord.z;
                frag_normal = face_normal;
            }
        }
        else
        {
            discard;
        }
        frag_pos_final = frag_pos;
        frag_depth_final = frag_depth;
        frag_normal_final = frag_normal;
    }
    else
    {
        frag_pos_final = in_var_C2;
        frag_depth_final = gl_FragCoord.z;
        frag_normal_final = face_normal;
    }
    // 取命中位置的最大轴向分量,用于判定砖缝噪声的坐标轴。
    float3 max_abs2 = max(abs_hit.x, max(abs_hit.y, abs_hit.z)).xxx;
    // 非最大轴向的边位置(把命中点沿最大轴向分量归零)。
    float3 edge_pos = select(bool3(abs_hit.x != max_abs2.x, abs_hit.y != max_abs2.y, abs_hit.z != max_abs2.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * hit_pos;
    float3 abs_edge = abs(edge_pos);
    // 二维 simplex 值噪声:以命中位置 xy 细分砖面,用于区分砖缝。
    float2 scaled_xy = frag_pos_final.xy * 2.5f;
    float2 grid = floor(scaled_xy + dot(scaled_xy, 0.3660254180431365966796875f.xx).xx);
    float2 frac_pos = (scaled_xy - grid) + dot(grid, 0.211324870586395263671875f.xx).xx;
    float fx = frac_pos.x;
    float fy = frac_pos.y;
    bool2 fx_gt_fy = (fx > fy).xx;
    float2 diag = float2(fx_gt_fy.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, fx_gt_fy.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 sf_v = frac_pos.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 sf_v0 = sf_v.xy - diag;
    float4 sf_v01 = float4(sf_v0.x, sf_v0.y, sf_v.z, sf_v.w);
    float2 grid_hash = fmod(grid, 289.0f.xx);
    float3 hash_y = grid_hash.y.xxx + float3(0.0f, diag.y, 1.0f);
    float3 hash_x = (fmod(((hash_y * 34.0f) + 1.0f.xxx) * hash_y, 289.0f.xxx) + grid_hash.x.xxx) + float3(0.0f, diag.x, 1.0f);
    float2 sf_v0_xy = sf_v0.xy;
    float2 sf_v1_zw = sf_v.zw;
    float3 sf_w = max(0.5f.xxx - float3(dot(frac_pos, frac_pos), dot(sf_v0_xy, sf_v0_xy), dot(sf_v1_zw, sf_v1_zw)), 0.0f.xxx);
    float3 sf_w2 = sf_w * sf_w;
    float3 sf_grad = frac(fmod(((hash_x * 34.0f) + 1.0f.xxx) * hash_x, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 sf_grad_c = sf_grad - 1.0f.xxx;
    float3 sf_grad_abs = abs(sf_grad_c) - 0.5f.xxx;
    float3 sf_grad_fix = sf_grad_c - floor(sf_grad + (-0.5f).xxx);
    float3 sf_dot0;
    sf_dot0.x = (sf_grad_fix.x * fx) + (sf_grad_abs.x * fy);
    float2 sf_dot1 = (sf_grad_fix.yz * sf_v01.xz) + (sf_grad_abs.yz * sf_v01.yw);
    float3 sf_noise = edge_pos + (dot((sf_w2 * sf_w2) * (1.792842864990234375f.xxx - (((sf_grad_fix * sf_grad_fix) + (sf_grad_abs * sf_grad_abs)) * 0.8537347316741943359375f)), float3(sf_dot0.x, sf_dot1.x, sf_dot1.y)) * 26.0f).xxx;
    // 砖缝判定:噪声强度超过 0.36 视为砂浆缝(后续对 UV 做偏移)。
    bool is_mortar = lerp(max(abs_edge.x, max(abs_edge.y, abs_edge.z)), 1.2999999523162841796875f * dot(sf_noise, sf_noise), 0.4000000059604644775390625f) > 0.36000001430511474609375f;
    // 是否需要按墙面高度裁剪:若 y < 墙顶(in_var_C7)则丢弃(墙顶以下)。
    bool below_top;
    if (!is_stair)
    {
        below_top = in_var_C7 > (-2.5f);
    }
    else
    {
        below_top = false;
    }
    bool discard_below;
    if (below_top)
    {
        discard_below = frag_pos_final.y < in_var_C7;
    }
    else
    {
        discard_below = false;
    }
    // 低于墙顶的部分 discard。
    if (discard_below)
    {
        discard;
    }
    else
    {
        bool above_top;
        if (is_stair)
        {
            above_top = frag_pos_final.y > in_var_C7;
        }
        else
        {
            above_top = false;
        }
        if (above_top)
        {
            discard;
        }
    }
    // 颜色库 id(in_var_C11):决定采样 roof_colors 层还是 brick_colors 层。
    uint color_lib_id = uint(in_var_C11);
    // 砖反照率:屋顶面采样 roof_colors;墙面按 id 选 brick_colors 层或颜色库基色。
    float3 albedo;
    // 位 2=屋顶面:采样 roof_colors 图集。
    if ((in_var_C13 & 2) != 0)
    {
        albedo = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(color_lib_id))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 uv;
        if (is_mortar)
        {
            uv = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            uv = in_var_C10;
        }
        bool is_plain_brick;
        if (!(color_lib_id == 1u))
        {
            is_plain_brick = color_lib_id == 0u;
        }
        else
        {
            is_plain_brick = true;
        }
        // 基色:颜色库 wall_bricks 或 wall_moss(取 0.8 强度)。
        float3 base_color;
        if (is_plain_brick)
        {
            base_color = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            base_color = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        // 砖形 id:按 0..12 选择 brick_colors 图集的层与采样偏移。
        int brick_id = int(color_lib_id);
        bool is_flat_brick;
        if (!(brick_id == 0))
        {
            is_flat_brick = brick_id == 4;
        }
        else
        {
            is_flat_brick = true;
        }
        bool is_double;
        if (!is_flat_brick)
        {
            is_double = brick_id == 10;
        }
        else
        {
            is_double = true;
        }
        bool is_dark_row;
        if (!is_double)
        {
            is_dark_row = brick_id == 12;
        }
        else
        {
            is_dark_row = true;
        }
        // brick_colors 图集采样结果(带不同层偏移与 gamma 调整)。
        float3 brick_sample;
        if (is_dark_row)
        {
            brick_sample = brick_colors.Sample(sampler_llr, float3(uv + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool is_beveled_row;
            if (!(brick_id == 1))
            {
                is_beveled_row = brick_id == 5;
            }
            else
            {
                is_beveled_row = true;
            }
            float3 brick_sample2;
            if (is_beveled_row)
            {
                brick_sample2 = pow(brick_colors.Sample(sampler_llr, float3(uv + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool is_row2;
                if (!(brick_id == 2))
                {
                    is_row2 = brick_id == 7;
                }
                else
                {
                    is_row2 = true;
                }
                float3 brick_sample3;
                if (is_row2)
                {
                    brick_sample3 = brick_colors.Sample(sampler_llr, float3(uv, 2.0f)).xyz;
                }
                else
                {
                    bool is_row3;
                    if (!(brick_id == 3))
                    {
                        is_row3 = brick_id == 6;
                    }
                    else
                    {
                        is_row3 = true;
                    }
                    bool is_row4;
                    if (!is_row3)
                    {
                        is_row4 = brick_id == 8;
                    }
                    else
                    {
                        is_row4 = true;
                    }
                    bool is_row5;
                    if (!is_row4)
                    {
                        is_row5 = brick_id == 9;
                    }
                    else
                    {
                        is_row5 = true;
                    }
                    float3 brick_sample4;
                    if (is_row5)
                    {
                        brick_sample4 = brick_colors.Sample(sampler_llr, float3(uv, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 brick_sample5;
                        if (brick_id == 11)
                        {
                            brick_sample5 = brick_colors.Sample(sampler_llr, float3(uv, 4.0f)).xyz;
                        }
                        else
                        {
                            brick_sample5 = 0.0f.xxx;
                        }
                        brick_sample4 = brick_sample5;
                    }
                    brick_sample3 = brick_sample4;
                }
                brick_sample2 = brick_sample3;
            }
            brick_sample = brick_sample2;
        }
        bool is_glaze_cand = !(color_lib_id == 7u);
        bool is_glaze;
        if (is_glaze_cand)
        {
            is_glaze = color_lib_id == 2u;
        }
        else
        {
            is_glaze = true;
        }
        bool is_glaze2;
        if (!is_glaze)
        {
            is_glaze2 = color_lib_id == 11u;
        }
        else
        {
            is_glaze2 = true;
        }
        bool3 is_glaze_mask = is_glaze2.xxx;
        // 釉面混合色:随机数经 smoothstep 决定釉面强度,与砖色 lerp。
        float3 glaze_color;
        uint glaze_or_bits;
        uint glaze_mask_bits;
        bool glaze_use_bits;
        if ((in_var_C13 & 512) != 0)
        {
            uint glz_h0 = in_var_C0 ^ 8902390u;
            uint glz_h1 = glz_h0 + (glz_h0 << 10u);
            uint glz_h2 = glz_h1 ^ (glz_h1 >> 6u);
            uint glz_h3 = glz_h2 + (glz_h2 << 3u);
            uint glz_hashB = glz_h3 ^ (glz_h3 >> 11u);
            uint rand_mask_sel = use_rand_bits ? rand_mask_bits : 8388607u;
            uint rand_or_sel = use_rand_bits ? rand_or_bits : 1065353216u;
            glaze_color = lerp(brick_sample, float3(is_glaze_mask.x ? brick_sample.x : base_color.x, is_glaze_mask.y ? brick_sample.y : base_color.y, is_glaze_mask.z ? brick_sample.z : base_color.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((glz_hashB + (glz_hashB << 15u)) & rand_mask_sel) | rand_or_sel) - 1.0f)).xxx);
            glaze_or_bits = rand_or_sel;
            glaze_mask_bits = rand_mask_sel;
            glaze_use_bits = use_rand_bits ? use_rand_bits : true;
        }
        else
        {
            glaze_color = brick_sample;
            glaze_or_bits = rand_or_bits;
            glaze_mask_bits = rand_mask_bits;
            glaze_use_bits = use_rand_bits;
        }
        // 第二层釉面:再按随机数向颜色库 wall_1 混合。
        float3 albedo2;
        if (is_mortar)
        {
            bool is_glaze3;
            if (is_glaze_cand)
            {
                is_glaze3 = color_lib_id == 2u;
            }
            else
            {
                is_glaze3 = true;
            }
            uint glz2_h0 = in_var_C0 ^ 239085u;
            uint glz2_h1 = glz2_h0 + (glz2_h0 << 10u);
            uint glz2_h2 = glz2_h1 ^ (glz2_h1 >> 6u);
            uint glz2_h3 = glz2_h2 + (glz2_h2 << 3u);
            uint glz2_hashB = glz2_h3 ^ (glz2_h3 >> 11u);
            albedo2 = lerp(glaze_color, color_library_wall_1.xyz, lerp(0.0f, is_glaze3 ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((glz2_hashB + (glz2_hashB << 15u)) & (glaze_use_bits ? glaze_mask_bits : 8388607u)) | (glaze_use_bits ? glaze_or_bits : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            albedo2 = glaze_color;
        }
        albedo = albedo2;
    }
    // 最终法线(normalize)。
    float3 frag_n = normalize(frag_normal_final);
    // 高亮判定:highlight_mode==1/2 或擦除模式命中时,把砖色向高亮色混合。
    bool is_any_hl;
    // 高亮混合色(橙色高亮或压暗)。
    float3 hl_color;
    if (!((in_var_C13 & 64) != 0))
    {
        bool is_hl1;
        float3 hl_blend;
        if (highlight_mode == 1u)
        {
            is_hl1 = true;
            hl_blend = lerp(albedo, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool is_hl2 = highlight_mode == 2u;
            float3 dim_color;
            if (is_hl2)
            {
                dim_color = lerp(albedo, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                dim_color = albedo;
            }
            is_hl1 = is_hl2 ? true : false;
            hl_blend = dim_color;
        }
        bool is_any_eraser;
        float3 final_hl_color;
        if (highlight_bricks_to_delete == 1)
        {
            bool near_eraser = distance(brush_position.xz, in_var_C12) < (1.88900005817413330078125f * eraser_mult);
            float3 eraser_color;
            if (near_eraser)
            {
                eraser_color = lerp(albedo, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                eraser_color = hl_blend;
            }
            is_any_eraser = near_eraser ? true : is_hl1;
            final_hl_color = eraser_color;
        }
        else
        {
            is_any_eraser = is_hl1;
            final_hl_color = hl_blend;
        }
        is_any_hl = is_any_eraser;
        hl_color = final_hl_color;
    }
    else
    {
        is_any_hl = false;
        hl_color = albedo;
    }
    // 最终反照率(楼梯高亮时再向橙色混合)。
    float3 final_albedo;
    if (is_stair_hl)
    {
        final_albedo = lerp(albedo, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        final_albedo = hl_color;
    }
    // 主题色:glade_theme_id==4 时用 simplex 噪声在墙面上做粉刷渐变。
    float3 theme_albedo;
    if (frame_constants_glade_theme_id == 4u)
    {
        // 主题粉刷的二维 simplex 噪声(网格为 0.4 尺度)。
        float2 theme_xy = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 theme_grid = floor(theme_xy + dot(theme_xy, 0.3660254180431365966796875f.xx).xx);
        float2 theme_frac = (theme_xy - theme_grid) + dot(theme_grid, 0.211324870586395263671875f.xx).xx;
        float tfx = theme_frac.x;
        float tfy = theme_frac.y;
        bool2 tfx_gt_tfy = (tfx > tfy).xx;
        float2 tdiag = float2(tfx_gt_tfy.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, tfx_gt_tfy.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 tsf_v = theme_frac.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 tsf_v0 = tsf_v.xy - tdiag;
        float4 tsf_v01 = float4(tsf_v0.x, tsf_v0.y, tsf_v.z, tsf_v.w);
        float2 tgrid_hash = fmod(theme_grid, 289.0f.xx);
        float3 thash_y = tgrid_hash.y.xxx + float3(0.0f, tdiag.y, 1.0f);
        float3 thash_x = (fmod(((thash_y * 34.0f) + 1.0f.xxx) * thash_y, 289.0f.xxx) + tgrid_hash.x.xxx) + float3(0.0f, tdiag.x, 1.0f);
        float2 tsf_v0_xy = tsf_v0.xy;
        float2 tsf_v1_zw = tsf_v.zw;
        float3 tsf_w = max(0.5f.xxx - float3(dot(theme_frac, theme_frac), dot(tsf_v0_xy, tsf_v0_xy), dot(tsf_v1_zw, tsf_v1_zw)), 0.0f.xxx);
        float3 tsf_w2 = tsf_w * tsf_w;
        float3 tsf_grad = frac(fmod(((thash_x * 34.0f) + 1.0f.xxx) * thash_x, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 tsf_grad_c = tsf_grad - 1.0f.xxx;
        float3 tsf_grad_abs = abs(tsf_grad_c) - 0.5f.xxx;
        float3 tsf_grad_fix = tsf_grad_c - floor(tsf_grad + (-0.5f).xxx);
        float3 tsf_dot0;
        tsf_dot0.x = (tsf_grad_fix.x * tfx) + (tsf_grad_abs.x * tfy);
        float2 tsf_dot1 = (tsf_grad_fix.yz * tsf_v01.xz) + (tsf_grad_abs.yz * tsf_v01.yw);
        theme_albedo = lerp(final_albedo, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(frag_n.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((tsf_w2 * tsf_w2) * (1.792842864990234375f.xxx - (((tsf_grad_fix * tsf_grad_fix) + (tsf_grad_abs * tsf_grad_abs)) * 0.8537347316741943359375f)), float3(tsf_dot0.x, tsf_dot1.x, tsf_dot1.y))) + 0.75f, 0.0f, 1.0f), flag_dark ? 0.0f : 0.4000000059604644775390625f, flag_dark ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        theme_albedo = final_albedo;
    }
    // 法线打包到 [0,1]:n * 0.5 + 0.5。
    float3 packed_n = (frag_n * 0.5f) + 0.5f.xxx;
    // 反照率开平方(近似 sRGB 线性化),并量化到 8bit。
    float3 gamma_albedo = sqrt(clamp(theme_albedo, 0.0f.xxx, 1.0f.xxx));
    // 把 RGB 各 8bit 打包进一个 uint。
    uint packed_albedo = (uint((clamp(gamma_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(gamma_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(gamma_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    // 输出像素:高 8bit 打包法线与标志,低 24bit 打包反照率。
    uint2 out_pixel = uint2(((uint((clamp(packed_n.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_n.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_n.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, packed_albedo);
    out_pixel.y = packed_albedo | (((((7u | (uint(is_stair_hl ? true : is_any_hl) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = out_pixel;
    gl_FragDepth = frag_depth_final;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C10 = stage_input.in_var_C10;
    in_var_C11 = stage_input.in_var_C11;
    in_var_C12 = stage_input.in_var_C12;
    in_var_C13 = stage_input.in_var_C13;
    in_var_C14 = stage_input.in_var_C14;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// [变体 f0adff76827e73ff | 顶点阶段] 简化版顶点:无颜色 id / 釉面,砖体整体再乘 0.95,其余同 b903e 顶点。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

static float zero;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint highlight_mode : packoffset(c0.y);
    int highlight_bricks_to_delete : packoffset(c0.z);
    float3 brush_position : packoffset(c1);
    uint bool_render_shadows : packoffset(c1.w);
    float eraser_mult : packoffset(c2);
};

ByteAddressBuffer instanced_wall_data : register(t19, space0);
ByteAddressBuffer instance_list : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
    float4 gl_Position : SV_Position;
};

// 顶点主函数(简化变体):与 b903e 类似但更简——无颜色 id / 釉面,砖体整体再乘 0.95。
void vert_main()
{
    // 实例槽位:由 current_view_index + 2 与实例索引拼出 instance_list 偏移。
    uint instance_slot = ((current_view_index + 2u) * 2097152u) + uint(gl_InstanceIndex);
    float transform_data[12];
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        transform_data[i] = instanced_wall_data.Load<float>(i * 4 + instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 0);
    }
    float3x4 wall_xform = transpose(float4x3(float3(transform_data[0], transform_data[1], transform_data[2]), float3(transform_data[3], transform_data[4], transform_data[5]), float3(transform_data[6], transform_data[7], transform_data[8]), float3(transform_data[9], transform_data[10], transform_data[11])));
    float3 wall_pos = float3(wall_xform[0].w, wall_xform[1].w, wall_xform[2].w);
    // 把墙体平移缩放到噪声格尺度。
    float3 noise_world = wall_pos * 0.4000000059604644775390625f;
    // 噪声格坐标 = floor(缩放后位置)。
    int3 noise_cell = int3(floor(noise_world));
    float3 noise_frac = frac(noise_world);
    uint3 noise_cell_u = uint3(noise_cell);
    uint n0_z = noise_cell_u.z;
    uint n0_z1 = n0_z + (n0_z << 10u);
    uint n0_z2 = n0_z1 ^ (n0_z1 >> 6u);
    uint n0_z3 = n0_z2 + (n0_z2 << 3u);
    uint n0_z4 = n0_z3 ^ (n0_z3 >> 11u);
    uint n0_y0 = (((noise_cell_u.y * 1664525u) + (n0_z4 + (n0_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n0_y1 = n0_y0 ^ (n0_y0 >> 11u);
    uint n0_y2 = n0_y1 ^ ((n0_y1 << 7u) & 2636928640u);
    uint n0_y3 = n0_y2 ^ ((n0_y2 << 15u) & 4022730752u);
    uint n0_x0 = (((noise_cell_u.x * 1664525u) + (n0_y3 ^ (n0_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n0_x1 = n0_x0 ^ (n0_x0 >> 11u);
    uint n0_x2 = n0_x1 ^ ((n0_x1 << 7u) & 2636928640u);
    uint n0_x3 = n0_x2 ^ ((n0_x2 << 15u) & 4022730752u);
    uint n0_hashA = n0_x3 ^ (n0_x3 >> 18u);
    uint n0_s0 = n0_hashA + (n0_hashA << 10u);
    uint n0_s1 = n0_s0 ^ (n0_s0 >> 6u);
    uint n0_s2 = n0_s1 + (n0_s1 << 3u);
    uint n0_hashB = n0_s2 ^ (n0_s2 >> 11u);
    // 噪声角点 (1,0,0) 的哈希链。
    uint3 n1_cell = uint3(noise_cell + int3(1, 0, 0));
    uint n1_z = n1_cell.z;
    uint n1_z1 = n1_z + (n1_z << 10u);
    uint n1_z2 = n1_z1 ^ (n1_z1 >> 6u);
    uint n1_z3 = n1_z2 + (n1_z2 << 3u);
    uint n1_z4 = n1_z3 ^ (n1_z3 >> 11u);
    uint n1_y0 = (((n1_cell.y * 1664525u) + (n1_z4 + (n1_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n1_y1 = n1_y0 ^ (n1_y0 >> 11u);
    uint n1_y2 = n1_y1 ^ ((n1_y1 << 7u) & 2636928640u);
    uint n1_y3 = n1_y2 ^ ((n1_y2 << 15u) & 4022730752u);
    uint n1_x0 = (((n1_cell.x * 1664525u) + (n1_y3 ^ (n1_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n1_x1 = n1_x0 ^ (n1_x0 >> 11u);
    uint n1_x2 = n1_x1 ^ ((n1_x1 << 7u) & 2636928640u);
    uint n1_x3 = n1_x2 ^ ((n1_x2 << 15u) & 4022730752u);
    uint n1_hashA = n1_x3 ^ (n1_x3 >> 18u);
    uint n1_s0 = n1_hashA + (n1_hashA << 10u);
    uint n1_s1 = n1_s0 ^ (n1_s0 >> 6u);
    uint n1_s2 = n1_s1 + (n1_s1 << 3u);
    uint n1_hashB = n1_s2 ^ (n1_s2 >> 11u);
    // 噪声角点 (0,1,0) 的哈希链。
    uint3 n2_cell = uint3(noise_cell + int3(0, 1, 0));
    uint n2_z = n2_cell.z;
    uint n2_z1 = n2_z + (n2_z << 10u);
    uint n2_z2 = n2_z1 ^ (n2_z1 >> 6u);
    uint n2_z3 = n2_z2 + (n2_z2 << 3u);
    uint n2_z4 = n2_z3 ^ (n2_z3 >> 11u);
    uint n2_y0 = (((n2_cell.y * 1664525u) + (n2_z4 + (n2_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n2_y1 = n2_y0 ^ (n2_y0 >> 11u);
    uint n2_y2 = n2_y1 ^ ((n2_y1 << 7u) & 2636928640u);
    uint n2_y3 = n2_y2 ^ ((n2_y2 << 15u) & 4022730752u);
    uint n2_x0 = (((n2_cell.x * 1664525u) + (n2_y3 ^ (n2_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n2_x1 = n2_x0 ^ (n2_x0 >> 11u);
    uint n2_x2 = n2_x1 ^ ((n2_x1 << 7u) & 2636928640u);
    uint n2_x3 = n2_x2 ^ ((n2_x2 << 15u) & 4022730752u);
    uint n2_hashA = n2_x3 ^ (n2_x3 >> 18u);
    uint n2_s0 = n2_hashA + (n2_hashA << 10u);
    uint n2_s1 = n2_s0 ^ (n2_s0 >> 6u);
    uint n2_s2 = n2_s1 + (n2_s1 << 3u);
    uint n2_hashB = n2_s2 ^ (n2_s2 >> 11u);
    // 噪声角点 (1,1,0) 的哈希链。
    uint3 n3_cell = uint3(noise_cell + int3(1, 1, 0));
    uint n3_z = n3_cell.z;
    uint n3_z1 = n3_z + (n3_z << 10u);
    uint n3_z2 = n3_z1 ^ (n3_z1 >> 6u);
    uint n3_z3 = n3_z2 + (n3_z2 << 3u);
    uint n3_z4 = n3_z3 ^ (n3_z3 >> 11u);
    uint n3_y0 = (((n3_cell.y * 1664525u) + (n3_z4 + (n3_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n3_y1 = n3_y0 ^ (n3_y0 >> 11u);
    uint n3_y2 = n3_y1 ^ ((n3_y1 << 7u) & 2636928640u);
    uint n3_y3 = n3_y2 ^ ((n3_y2 << 15u) & 4022730752u);
    uint n3_x0 = (((n3_cell.x * 1664525u) + (n3_y3 ^ (n3_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n3_x1 = n3_x0 ^ (n3_x0 >> 11u);
    uint n3_x2 = n3_x1 ^ ((n3_x1 << 7u) & 2636928640u);
    uint n3_x3 = n3_x2 ^ ((n3_x2 << 15u) & 4022730752u);
    uint n3_hashA = n3_x3 ^ (n3_x3 >> 18u);
    uint n3_s0 = n3_hashA + (n3_hashA << 10u);
    uint n3_s1 = n3_s0 ^ (n3_s0 >> 6u);
    uint n3_s2 = n3_s1 + (n3_s1 << 3u);
    uint n3_hashB = n3_s2 ^ (n3_s2 >> 11u);
    // 噪声角点 (0,0,1) 的哈希链。
    uint3 n4_cell = uint3(noise_cell + int3(0, 0, 1));
    uint n4_z = n4_cell.z;
    uint n4_z1 = n4_z + (n4_z << 10u);
    uint n4_z2 = n4_z1 ^ (n4_z1 >> 6u);
    uint n4_z3 = n4_z2 + (n4_z2 << 3u);
    uint n4_z4 = n4_z3 ^ (n4_z3 >> 11u);
    uint n4_y0 = (((n4_cell.y * 1664525u) + (n4_z4 + (n4_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n4_y1 = n4_y0 ^ (n4_y0 >> 11u);
    uint n4_y2 = n4_y1 ^ ((n4_y1 << 7u) & 2636928640u);
    uint n4_y3 = n4_y2 ^ ((n4_y2 << 15u) & 4022730752u);
    uint n4_x0 = (((n4_cell.x * 1664525u) + (n4_y3 ^ (n4_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n4_x1 = n4_x0 ^ (n4_x0 >> 11u);
    uint n4_x2 = n4_x1 ^ ((n4_x1 << 7u) & 2636928640u);
    uint n4_x3 = n4_x2 ^ ((n4_x2 << 15u) & 4022730752u);
    uint n4_hashA = n4_x3 ^ (n4_x3 >> 18u);
    uint n4_s0 = n4_hashA + (n4_hashA << 10u);
    uint n4_s1 = n4_s0 ^ (n4_s0 >> 6u);
    uint n4_s2 = n4_s1 + (n4_s1 << 3u);
    uint n4_hashB = n4_s2 ^ (n4_s2 >> 11u);
    // 噪声角点 (1,0,1) 的哈希链。
    uint3 n5_cell = uint3(noise_cell + int3(1, 0, 1));
    uint n5_z = n5_cell.z;
    uint n5_z1 = n5_z + (n5_z << 10u);
    uint n5_z2 = n5_z1 ^ (n5_z1 >> 6u);
    uint n5_z3 = n5_z2 + (n5_z2 << 3u);
    uint n5_z4 = n5_z3 ^ (n5_z3 >> 11u);
    uint n5_y0 = (((n5_cell.y * 1664525u) + (n5_z4 + (n5_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n5_y1 = n5_y0 ^ (n5_y0 >> 11u);
    uint n5_y2 = n5_y1 ^ ((n5_y1 << 7u) & 2636928640u);
    uint n5_y3 = n5_y2 ^ ((n5_y2 << 15u) & 4022730752u);
    uint n5_x0 = (((n5_cell.x * 1664525u) + (n5_y3 ^ (n5_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n5_x1 = n5_x0 ^ (n5_x0 >> 11u);
    uint n5_x2 = n5_x1 ^ ((n5_x1 << 7u) & 2636928640u);
    uint n5_x3 = n5_x2 ^ ((n5_x2 << 15u) & 4022730752u);
    uint n5_hashA = n5_x3 ^ (n5_x3 >> 18u);
    uint n5_s0 = n5_hashA + (n5_hashA << 10u);
    uint n5_s1 = n5_s0 ^ (n5_s0 >> 6u);
    uint n5_s2 = n5_s1 + (n5_s1 << 3u);
    uint n5_hashB = n5_s2 ^ (n5_s2 >> 11u);
    // 噪声角点 (0,1,1) 的哈希链。
    uint3 n6_cell = uint3(noise_cell + int3(0, 1, 1));
    uint n6_z = n6_cell.z;
    uint n6_z1 = n6_z + (n6_z << 10u);
    uint n6_z2 = n6_z1 ^ (n6_z1 >> 6u);
    uint n6_z3 = n6_z2 + (n6_z2 << 3u);
    uint n6_z4 = n6_z3 ^ (n6_z3 >> 11u);
    uint n6_y0 = (((n6_cell.y * 1664525u) + (n6_z4 + (n6_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n6_y1 = n6_y0 ^ (n6_y0 >> 11u);
    uint n6_y2 = n6_y1 ^ ((n6_y1 << 7u) & 2636928640u);
    uint n6_y3 = n6_y2 ^ ((n6_y2 << 15u) & 4022730752u);
    uint n6_x0 = (((n6_cell.x * 1664525u) + (n6_y3 ^ (n6_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n6_x1 = n6_x0 ^ (n6_x0 >> 11u);
    uint n6_x2 = n6_x1 ^ ((n6_x1 << 7u) & 2636928640u);
    uint n6_x3 = n6_x2 ^ ((n6_x2 << 15u) & 4022730752u);
    uint n6_hashA = n6_x3 ^ (n6_x3 >> 18u);
    uint n6_s0 = n6_hashA + (n6_hashA << 10u);
    uint n6_s1 = n6_s0 ^ (n6_s0 >> 6u);
    uint n6_s2 = n6_s1 + (n6_s1 << 3u);
    uint n6_hashB = n6_s2 ^ (n6_s2 >> 11u);
    // 噪声角点 (1,1,1) 的哈希链。
    uint3 n7_cell = uint3(noise_cell + int3(1, 1, 1));
    uint n7_z = n7_cell.z;
    uint n7_z1 = n7_z + (n7_z << 10u);
    uint n7_z2 = n7_z1 ^ (n7_z1 >> 6u);
    uint n7_z3 = n7_z2 + (n7_z2 << 3u);
    uint n7_z4 = n7_z3 ^ (n7_z3 >> 11u);
    uint n7_y0 = (((n7_cell.y * 1664525u) + (n7_z4 + (n7_z4 << 15u))) + 1013904223u) * 1664525u;
    uint n7_y1 = n7_y0 ^ (n7_y0 >> 11u);
    uint n7_y2 = n7_y1 ^ ((n7_y1 << 7u) & 2636928640u);
    uint n7_y3 = n7_y2 ^ ((n7_y2 << 15u) & 4022730752u);
    uint n7_x0 = (((n7_cell.x * 1664525u) + (n7_y3 ^ (n7_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint n7_x1 = n7_x0 ^ (n7_x0 >> 11u);
    uint n7_x2 = n7_x1 ^ ((n7_x1 << 7u) & 2636928640u);
    uint n7_x3 = n7_x2 ^ ((n7_x2 << 15u) & 4022730752u);
    uint n7_hashA = n7_x3 ^ (n7_x3 >> 18u);
    uint n7_s0 = n7_hashA + (n7_hashA << 10u);
    uint n7_s1 = n7_s0 ^ (n7_s0 >> 6u);
    uint n7_s2 = n7_s1 + (n7_s1 << 3u);
    uint n7_hashB = n7_s2 ^ (n7_s2 >> 11u);
    // smoothstep 权重。
    float3 smooth_weight = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
    float wx = smooth_weight.x;
    float wx1 = 1.0f - wx;
    float wy = smooth_weight.y;
    float wy1 = 1.0f - wy;
    float wz = smooth_weight.z;
    float wz1 = 1.0f - wz;
    // 8 角点 2D 随机向量的三线性插值。
    float2 noise_sum = ((((((float2(asfloat((n0_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n0_hashB + (n0_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy1) * wz1) + (((float2(asfloat((n1_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n1_hashB + (n1_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy1) * wz1)) + (((float2(asfloat((n2_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n2_hashB + (n2_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy) * wz1)) + (((float2(asfloat((n3_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n3_hashB + (n3_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz1)) + (((float2(asfloat((n4_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n4_hashB + (n4_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy1) * wz);
    // 抖动量:中心化、放大 2.5 并夹到 ±0.5。
    float2 noise_uv = clamp(((((noise_sum + (((float2(asfloat((n5_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n5_hashB + (n5_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy1) * wz)) + (((float2(asfloat((n6_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n6_hashB + (n6_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx1) * wy) * wz)) + (((float2(asfloat((n7_hashA & 8388607u) | 1065353216u) - 1.0f, asfloat(((n7_hashB + (n7_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    // 把抖动转成 3D 位置偏移(叠加 sin_deform_y 均值)。
    float3 pos_offset = (float3(noise_uv.x, 0.0f, noise_uv.y) * 0.0599999986588954925537109375f) + float3(0.0f, dot(instanced_wall_data.Load<float4>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 48), 0.25f.xxxx), 0.0f);
    // 新的墙体平移 = 原平移 + 抖动偏移。
    float3 new_pos = wall_pos + pos_offset;
    float3x4 xfm1 = wall_xform;
    xfm1[0].w = new_pos.x;
    float3x4 xfm2 = xfm1;
    xfm2[1].w = new_pos.y;
    float3x4 xfm3 = xfm2;
    xfm3[2].w = new_pos.z;
    // 从顶点索引取 3 位确定砖角。
    uint3 corner_mask = uint3((uint(gl_VertexIndex) & 4u) >> 2u, (uint(gl_VertexIndex) & 2u) >> 1u, uint(gl_VertexIndex) & 1u);
    float3 brick_origin = mul(xfm3, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 cam_pos4 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 corner_local = mul((cam_pos4.xyz / cam_pos4.w.xxx) - brick_origin, float3x3(xfm1[0].xyz, xfm2[1].xyz, xfm3[2].xyz));
    bool3 corner_flip = bool3(corner_local.x > 0.0f.xxx.x, corner_local.y > 0.0f.xxx.y, corner_local.z > 0.0f.xxx.z);
    uint3 corner_mask_inv = uint3(1u, 1u, 1u) - corner_mask;
    // 砖角方向(±0.5),让砖角朝向相机。
    float3 corner_dir = float3(uint3(corner_flip.x ? corner_mask.x : corner_mask_inv.x, corner_flip.y ? corner_mask.y : corner_mask_inv.y, corner_flip.z ? corner_mask.z : corner_mask_inv.z)) - 0.5f.xxx;
    float4 corner_dir4 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(corner_dir, 1.0f));
    float3 corner_dir3 = corner_dir4.xyz;
    // 墙标志(偏移 92)。
    int wall_flags = int(instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 92));
    float3 corner_pos;
    if ((wall_flags & 32) != 0)
    {
        float3 corner_pos_new = corner_dir3;
        corner_pos_new.z = corner_dir4.z * lerp(instanced_wall_data.Load<float>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64), instanced_wall_data.Load<float>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 68), 0.5f - corner_dir4.x);
        corner_pos = corner_pos_new;
    }
    else
    {
        corner_pos = corner_dir3;
    }
    // 砖角最终缩放。
    float3 brick_scale;
    // 倒角砖缩放(1.0/1.1 或 1.05)。
    if ((wall_flags & 4) != 0)
    {
        uint bump_h0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) << 10u);
        uint bump_h1 = bump_h0 ^ (bump_h0 >> 6u);
        uint bump_h2 = bump_h1 + (bump_h1 << 3u);
        uint bump_h3 = bump_h2 ^ (bump_h2 >> 11u);
        float bump_rand = ((asfloat(((bump_h3 + (bump_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 1.0f : 1.099999904632568359375f;
        float3 bump_scale_vec = float3(bump_rand, bump_rand, 1.0f);
        bool3 bevel_mask = ((wall_flags & 1) != 0).xxx;
        brick_scale = float3(bevel_mask.x ? 1.0499999523162841796875f.xxx.x : bump_scale_vec.x, bevel_mask.y ? 1.0499999523162841796875f.xxx.y : bump_scale_vec.y, bevel_mask.z ? 1.0499999523162841796875f.xxx.z : bump_scale_vec.z);
    }
    else
    {
        brick_scale = 1.0f.xxx;
    }
    // 砖角世界位置(带缩放,整体再乘 0.95)。
    float4 brick_pos4 = float4(mul(xfm3, float4(corner_pos * (brick_scale * 0.949999988079071044921875f), 1.0f)), 1.0f);
    // 砖在墙面上沿 x 的归一化位置。
    float brick_u = mul(float3x3(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1.0f, 0.0f), float3(0.0f, 0.0f, 1.0f)), corner_dir).x - (-0.5f);
    // 砖标志:颜色 id < 10 时追加 512。
    int brick_flags;
    if (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) < 10u)
    {
        brick_flags = wall_flags | 512;
    }
    else
    {
        brick_flags = wall_flags;
    }
    // 拱形高度(三值插值)。
    float arch_height;
    if (brick_u < 0.5f)
    {
        arch_height = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).x, instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).y, brick_u * 2.0f);
    }
    else
    {
        arch_height = lerp(instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).y, instanced_wall_data.Load<float3>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 64).z, (brick_u * 2.0f) - 1.0f);
    }
    // 拱形标志。
    bool is_arch = (brick_flags & 32) != 0;
    // 拱形砖的 UV 抖动(或 0)。
    float2 arch_uv;
    if (is_arch)
    {
        uint arch_h0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) + (instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) << 10u);
        uint arch_h1 = arch_h0 ^ (arch_h0 >> 6u);
        uint arch_h2 = arch_h1 + (arch_h1 << 3u);
        uint arch_hashB = arch_h2 ^ (arch_h2 >> 11u);
        arch_uv = ((brick_pos4.xz * 0.699999988079071044921875f) * 0.300000011920928955078125f) + (asfloat(((arch_hashB + (arch_hashB << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    }
    else
    {
        arch_uv = 0.0f.xx;
    }
    // 输出:世界→采样空间裁剪位置。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, brick_pos4);
    // 输出 TEXCOORD(颜色 id、位置、抖动、高度、标志、实例 id)。
    out_var_C0 = instanced_wall_data.Load<uint>(instance_list.Load<uint>(instance_slot * 4 + 0) * 96 + 76) & 65535u;
    out_var_C1 = 0.0f.xxx;
    out_var_C2 = brick_pos4.xyz;
    out_var_C5 = pos_offset;
    out_var_C7 = is_arch ? (-100.0f) : arch_height;
    out_var_C10 = arch_uv;
    out_var_C11 = 0;
    out_var_C12 = float4(brick_origin.x, zero, brick_origin.z, 1.0f).xz;
    out_var_C13 = brick_flags;
    out_var_C14 = instance_list.Load<uint>(instance_slot * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// [变体 f0adff76827e73ff | 片元阶段] 简化版片元:只做两层高度裁剪(墙顶以上/以下)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C2;
static float in_var_C7;
static int in_var_C13;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
};

// 片元主函数(简化变体):只做两层高度裁剪(墙顶以下 / 楼梯墙顶以上)。
void frag_main()
{
    // 若 in_var_C7 > 0:y < 墙顶则丢弃(墙顶以下)。
    bool below_wall;
    if (in_var_C7 > 0.0f)
    {
        below_wall = in_var_C2.y < in_var_C7;
    }
    else
    {
        below_wall = false;
    }
    if (below_wall)
    {
        discard;
    }
    // 若 C13 位 8 置位(楼梯):y > 墙顶则丢弃。
    bool above_wall;
    if ((in_var_C13 & 8) != 0)
    {
        above_wall = in_var_C2.y > in_var_C7;
    }
    else
    {
        above_wall = false;
    }
    if (above_wall)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C13 = stage_input.in_var_C13;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
