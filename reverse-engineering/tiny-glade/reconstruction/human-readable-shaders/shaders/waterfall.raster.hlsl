// ==RECONSTRUCTED== shaders/waterfall.raster.hlsl
// 整文件提升:函数体由 tools/render-passes/batch_lift.py 从 oracle bin 生成
// (手写版本低于保真度门槛)。变体通过 TGV_<binhash> 选择。
// 瀑布渲染:一张实例化的瀑布网格(每实例 64 字节,位置/切线/两层法线)。
//   vs:逐顶点做"下落动画" —— 读实例网格,按顶点 id 展开网格uv(u 沿宽度、
//       v 沿高度,高度再 *0.1),两层法线插值后沿切向/法向推挤顶点,再叠加
//       3D perlin 位移(随时间沿 +Y 平移,让瀑布像在流动);最终世界->采样。
//   ps (b903e,完整版):与 water_plane 相同的水面算法 —— 多层水噪声+涟漪源
//       法线、half-depth 折射 marching、阴影、景深、镜面反射 marching、TAA
//       历史、GGX 高光、主题4岸线泡沫;折射色与反射按菲涅尔混合输出。
//   ps (f0adff,精简版):只做水法线/反射 marching,并往水 RT ray 缓冲写入光线,
//       不写颜色输出。
// 绑定:b903e ps —— ripple_source_buffer(t26)、autofocus_buf(t33)、
//   water_rt_count/water_rt_ray(u34/u35)、shadow_map(t18)、water_noise(t27)、
//   water_mask(t28)、half_depth_tex(t29)、refraction_tex(t30)、
//   taa_history_tex(t31)、reprojection_tex(t32);f0adff ps 只用其中部分。

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _waterfall.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
};

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer instance_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 读入当前瀑布实例的网格数据(位置/切线/两层法线),再按顶点 id 展开网格 uv。
    WaterfallMesh mesh;
    mesh.position = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 0);
    mesh.tangent = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 16);
    mesh.normal0 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 32);
    mesh.normal1 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 48);
    float2 uv = float2(float(uint(gl_VertexIndex) % 2u), float(uint(gl_VertexIndex) / 2u)) * float2(1.0f, 0.100000001490116119384765625f);
    float uv_x = uv.x;
    float3 n = lerp(mesh.normal0.xyz, mesh.normal1.xyz, uv_x.xxx);
    float uv_y = uv.y;
    float3 pos = (mesh.position.xyz + (float3(mesh.tangent.xy, 0.0f).xzy * uv_x)) + (n * (uv_y * 0.5f));
    pos.y = lerp(mesh.position.w, mesh.position.y, 1.0f - pow(uv_y, 3.0f));
    // 把顶点坐标折半并随时间沿 +Y 平移,构造 3D 值噪声的采样网格(让瀑布看起来在流)。
    float3 scaled = (pos * float3(0.5f, 1.0f, 0.5f)) + (float3(0.0f, 3.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 cell = int3(floor(scaled));
    float3 f = frac(scaled);
    uint3 cell_u = uint3(cell);
    uint h000_z0 = cell_u.z;
    uint h000_z1 = h000_z0 + (h000_z0 << 10u);
    uint h000_z2 = h000_z1 ^ (h000_z1 >> 6u);
    uint h000_z3 = h000_z2 + (h000_z2 << 3u);
    uint h000_z4 = h000_z3 ^ (h000_z3 >> 11u);
    uint h000_y0 = (((cell_u.y * 1664525u) + (h000_z4 + (h000_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h000_y1 = h000_y0 ^ (h000_y0 >> 11u);
    uint h000_y2 = h000_y1 ^ ((h000_y1 << 7u) & 2636928640u);
    uint h000_y3 = h000_y2 ^ ((h000_y2 << 15u) & 4022730752u);
    uint h000_x0 = (((cell_u.x * 1664525u) + (h000_y3 ^ (h000_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h000_x1 = h000_x0 ^ (h000_x0 >> 11u);
    uint h000_x2 = h000_x1 ^ ((h000_x1 << 7u) & 2636928640u);
    uint h000_x3 = h000_x2 ^ ((h000_x2 << 15u) & 4022730752u);
    uint seed000_x = h000_x3 ^ (h000_x3 >> 18u);
    uint h000_sx1 = seed000_x + (seed000_x << 10u);
    uint h000_sx2 = h000_sx1 ^ (h000_sx1 >> 6u);
    uint h000_sx3 = h000_sx2 + (h000_sx2 << 3u);
    uint h000_sx4 = h000_sx3 ^ (h000_sx3 >> 11u);
    uint seed000_y = h000_sx4 + (h000_sx4 << 15u);
    uint h000_sy1 = seed000_y + (seed000_y << 10u);
    uint h000_sy2 = h000_sy1 ^ (h000_sy1 >> 6u);
    uint h000_sy3 = h000_sy2 + (h000_sy2 << 3u);
    uint seed000_z = h000_sy3 ^ (h000_sy3 >> 11u);
    uint3 cell_x1 = uint3(cell + int3(1, 0, 0));
    uint h100_z0 = cell_x1.z;
    uint h100_z1 = h100_z0 + (h100_z0 << 10u);
    uint h100_z2 = h100_z1 ^ (h100_z1 >> 6u);
    uint h100_z3 = h100_z2 + (h100_z2 << 3u);
    uint h100_z4 = h100_z3 ^ (h100_z3 >> 11u);
    uint h100_y0 = (((cell_x1.y * 1664525u) + (h100_z4 + (h100_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h100_y1 = h100_y0 ^ (h100_y0 >> 11u);
    uint h100_y2 = h100_y1 ^ ((h100_y1 << 7u) & 2636928640u);
    uint h100_y3 = h100_y2 ^ ((h100_y2 << 15u) & 4022730752u);
    uint h100_x0 = (((cell_x1.x * 1664525u) + (h100_y3 ^ (h100_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h100_x1 = h100_x0 ^ (h100_x0 >> 11u);
    uint h100_x2 = h100_x1 ^ ((h100_x1 << 7u) & 2636928640u);
    uint h100_x3 = h100_x2 ^ ((h100_x2 << 15u) & 4022730752u);
    uint seed100_x = h100_x3 ^ (h100_x3 >> 18u);
    uint h100_sx1 = seed100_x + (seed100_x << 10u);
    uint h100_sx2 = h100_sx1 ^ (h100_sx1 >> 6u);
    uint h100_sx3 = h100_sx2 + (h100_sx2 << 3u);
    uint h100_sx4 = h100_sx3 ^ (h100_sx3 >> 11u);
    uint seed100_y = h100_sx4 + (h100_sx4 << 15u);
    uint h100_sy1 = seed100_y + (seed100_y << 10u);
    uint h100_sy2 = h100_sy1 ^ (h100_sy1 >> 6u);
    uint h100_sy3 = h100_sy2 + (h100_sy2 << 3u);
    uint seed100_z = h100_sy3 ^ (h100_sy3 >> 11u);
    uint3 cell_y1 = uint3(cell + int3(0, 1, 0));
    uint h010_z0 = cell_y1.z;
    uint h010_z1 = h010_z0 + (h010_z0 << 10u);
    uint h010_z2 = h010_z1 ^ (h010_z1 >> 6u);
    uint h010_z3 = h010_z2 + (h010_z2 << 3u);
    uint h010_z4 = h010_z3 ^ (h010_z3 >> 11u);
    uint h010_y0 = (((cell_y1.y * 1664525u) + (h010_z4 + (h010_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h010_y1 = h010_y0 ^ (h010_y0 >> 11u);
    uint h010_y2 = h010_y1 ^ ((h010_y1 << 7u) & 2636928640u);
    uint h010_y3 = h010_y2 ^ ((h010_y2 << 15u) & 4022730752u);
    uint h010_x0 = (((cell_y1.x * 1664525u) + (h010_y3 ^ (h010_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h010_x1 = h010_x0 ^ (h010_x0 >> 11u);
    uint h010_x2 = h010_x1 ^ ((h010_x1 << 7u) & 2636928640u);
    uint h010_x3 = h010_x2 ^ ((h010_x2 << 15u) & 4022730752u);
    uint seed010_x = h010_x3 ^ (h010_x3 >> 18u);
    uint h010_sx1 = seed010_x + (seed010_x << 10u);
    uint h010_sx2 = h010_sx1 ^ (h010_sx1 >> 6u);
    uint h010_sx3 = h010_sx2 + (h010_sx2 << 3u);
    uint h010_sx4 = h010_sx3 ^ (h010_sx3 >> 11u);
    uint seed010_y = h010_sx4 + (h010_sx4 << 15u);
    uint h010_sy1 = seed010_y + (seed010_y << 10u);
    uint h010_sy2 = h010_sy1 ^ (h010_sy1 >> 6u);
    uint h010_sy3 = h010_sy2 + (h010_sy2 << 3u);
    uint seed010_z = h010_sy3 ^ (h010_sy3 >> 11u);
    uint3 cell_xy1 = uint3(cell + int3(1, 1, 0));
    uint h110_z0 = cell_xy1.z;
    uint h110_z1 = h110_z0 + (h110_z0 << 10u);
    uint h110_z2 = h110_z1 ^ (h110_z1 >> 6u);
    uint h110_z3 = h110_z2 + (h110_z2 << 3u);
    uint h110_z4 = h110_z3 ^ (h110_z3 >> 11u);
    uint h110_y0 = (((cell_xy1.y * 1664525u) + (h110_z4 + (h110_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h110_y1 = h110_y0 ^ (h110_y0 >> 11u);
    uint h110_y2 = h110_y1 ^ ((h110_y1 << 7u) & 2636928640u);
    uint h110_y3 = h110_y2 ^ ((h110_y2 << 15u) & 4022730752u);
    uint h110_x0 = (((cell_xy1.x * 1664525u) + (h110_y3 ^ (h110_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h110_x1 = h110_x0 ^ (h110_x0 >> 11u);
    uint h110_x2 = h110_x1 ^ ((h110_x1 << 7u) & 2636928640u);
    uint h110_x3 = h110_x2 ^ ((h110_x2 << 15u) & 4022730752u);
    uint seed110_x = h110_x3 ^ (h110_x3 >> 18u);
    uint h110_sx1 = seed110_x + (seed110_x << 10u);
    uint h110_sx2 = h110_sx1 ^ (h110_sx1 >> 6u);
    uint h110_sx3 = h110_sx2 + (h110_sx2 << 3u);
    uint h110_sx4 = h110_sx3 ^ (h110_sx3 >> 11u);
    uint seed110_y = h110_sx4 + (h110_sx4 << 15u);
    uint h110_sy1 = seed110_y + (seed110_y << 10u);
    uint h110_sy2 = h110_sy1 ^ (h110_sy1 >> 6u);
    uint h110_sy3 = h110_sy2 + (h110_sy2 << 3u);
    uint seed110_z = h110_sy3 ^ (h110_sy3 >> 11u);
    uint3 cell_z1 = uint3(cell + int3(0, 0, 1));
    uint h001_z0 = cell_z1.z;
    uint h001_z1 = h001_z0 + (h001_z0 << 10u);
    uint h001_z2 = h001_z1 ^ (h001_z1 >> 6u);
    uint h001_z3 = h001_z2 + (h001_z2 << 3u);
    uint h001_z4 = h001_z3 ^ (h001_z3 >> 11u);
    uint h001_y0 = (((cell_z1.y * 1664525u) + (h001_z4 + (h001_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h001_y1 = h001_y0 ^ (h001_y0 >> 11u);
    uint h001_y2 = h001_y1 ^ ((h001_y1 << 7u) & 2636928640u);
    uint h001_y3 = h001_y2 ^ ((h001_y2 << 15u) & 4022730752u);
    uint h001_x0 = (((cell_z1.x * 1664525u) + (h001_y3 ^ (h001_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h001_x1 = h001_x0 ^ (h001_x0 >> 11u);
    uint h001_x2 = h001_x1 ^ ((h001_x1 << 7u) & 2636928640u);
    uint h001_x3 = h001_x2 ^ ((h001_x2 << 15u) & 4022730752u);
    uint seed001_x = h001_x3 ^ (h001_x3 >> 18u);
    uint h001_sx1 = seed001_x + (seed001_x << 10u);
    uint h001_sx2 = h001_sx1 ^ (h001_sx1 >> 6u);
    uint h001_sx3 = h001_sx2 + (h001_sx2 << 3u);
    uint h001_sx4 = h001_sx3 ^ (h001_sx3 >> 11u);
    uint seed001_y = h001_sx4 + (h001_sx4 << 15u);
    uint h001_sy1 = seed001_y + (seed001_y << 10u);
    uint h001_sy2 = h001_sy1 ^ (h001_sy1 >> 6u);
    uint h001_sy3 = h001_sy2 + (h001_sy2 << 3u);
    uint seed001_z = h001_sy3 ^ (h001_sy3 >> 11u);
    uint3 cell_xz1 = uint3(cell + int3(1, 0, 1));
    uint h101_z0 = cell_xz1.z;
    uint h101_z1 = h101_z0 + (h101_z0 << 10u);
    uint h101_z2 = h101_z1 ^ (h101_z1 >> 6u);
    uint h101_z3 = h101_z2 + (h101_z2 << 3u);
    uint h101_z4 = h101_z3 ^ (h101_z3 >> 11u);
    uint h101_y0 = (((cell_xz1.y * 1664525u) + (h101_z4 + (h101_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h101_y1 = h101_y0 ^ (h101_y0 >> 11u);
    uint h101_y2 = h101_y1 ^ ((h101_y1 << 7u) & 2636928640u);
    uint h101_y3 = h101_y2 ^ ((h101_y2 << 15u) & 4022730752u);
    uint h101_x0 = (((cell_xz1.x * 1664525u) + (h101_y3 ^ (h101_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h101_x1 = h101_x0 ^ (h101_x0 >> 11u);
    uint h101_x2 = h101_x1 ^ ((h101_x1 << 7u) & 2636928640u);
    uint h101_x3 = h101_x2 ^ ((h101_x2 << 15u) & 4022730752u);
    uint seed101_x = h101_x3 ^ (h101_x3 >> 18u);
    uint h101_sx1 = seed101_x + (seed101_x << 10u);
    uint h101_sx2 = h101_sx1 ^ (h101_sx1 >> 6u);
    uint h101_sx3 = h101_sx2 + (h101_sx2 << 3u);
    uint h101_sx4 = h101_sx3 ^ (h101_sx3 >> 11u);
    uint seed101_y = h101_sx4 + (h101_sx4 << 15u);
    uint h101_sy1 = seed101_y + (seed101_y << 10u);
    uint h101_sy2 = h101_sy1 ^ (h101_sy1 >> 6u);
    uint h101_sy3 = h101_sy2 + (h101_sy2 << 3u);
    uint seed101_z = h101_sy3 ^ (h101_sy3 >> 11u);
    uint3 cell_yz1 = uint3(cell + int3(0, 1, 1));
    uint h011_z0 = cell_yz1.z;
    uint h011_z1 = h011_z0 + (h011_z0 << 10u);
    uint h011_z2 = h011_z1 ^ (h011_z1 >> 6u);
    uint h011_z3 = h011_z2 + (h011_z2 << 3u);
    uint h011_z4 = h011_z3 ^ (h011_z3 >> 11u);
    uint h011_y0 = (((cell_yz1.y * 1664525u) + (h011_z4 + (h011_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h011_y1 = h011_y0 ^ (h011_y0 >> 11u);
    uint h011_y2 = h011_y1 ^ ((h011_y1 << 7u) & 2636928640u);
    uint h011_y3 = h011_y2 ^ ((h011_y2 << 15u) & 4022730752u);
    uint h011_x0 = (((cell_yz1.x * 1664525u) + (h011_y3 ^ (h011_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h011_x1 = h011_x0 ^ (h011_x0 >> 11u);
    uint h011_x2 = h011_x1 ^ ((h011_x1 << 7u) & 2636928640u);
    uint h011_x3 = h011_x2 ^ ((h011_x2 << 15u) & 4022730752u);
    uint seed011_x = h011_x3 ^ (h011_x3 >> 18u);
    uint h011_sx1 = seed011_x + (seed011_x << 10u);
    uint h011_sx2 = h011_sx1 ^ (h011_sx1 >> 6u);
    uint h011_sx3 = h011_sx2 + (h011_sx2 << 3u);
    uint h011_sx4 = h011_sx3 ^ (h011_sx3 >> 11u);
    uint seed011_y = h011_sx4 + (h011_sx4 << 15u);
    uint h011_sy1 = seed011_y + (seed011_y << 10u);
    uint h011_sy2 = h011_sy1 ^ (h011_sy1 >> 6u);
    uint h011_sy3 = h011_sy2 + (h011_sy2 << 3u);
    uint seed011_z = h011_sy3 ^ (h011_sy3 >> 11u);
    uint3 cell_xyz1 = uint3(cell + int3(1, 1, 1));
    uint h111_z0 = cell_xyz1.z;
    uint h111_z1 = h111_z0 + (h111_z0 << 10u);
    uint h111_z2 = h111_z1 ^ (h111_z1 >> 6u);
    uint h111_z3 = h111_z2 + (h111_z2 << 3u);
    uint h111_z4 = h111_z3 ^ (h111_z3 >> 11u);
    uint h111_y0 = (((cell_xyz1.y * 1664525u) + (h111_z4 + (h111_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h111_y1 = h111_y0 ^ (h111_y0 >> 11u);
    uint h111_y2 = h111_y1 ^ ((h111_y1 << 7u) & 2636928640u);
    uint h111_y3 = h111_y2 ^ ((h111_y2 << 15u) & 4022730752u);
    uint h111_x0 = (((cell_xyz1.x * 1664525u) + (h111_y3 ^ (h111_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h111_x1 = h111_x0 ^ (h111_x0 >> 11u);
    uint h111_x2 = h111_x1 ^ ((h111_x1 << 7u) & 2636928640u);
    uint h111_x3 = h111_x2 ^ ((h111_x2 << 15u) & 4022730752u);
    uint seed111_x = h111_x3 ^ (h111_x3 >> 18u);
    uint h111_sx1 = seed111_x + (seed111_x << 10u);
    uint h111_sx2 = h111_sx1 ^ (h111_sx1 >> 6u);
    uint h111_sx3 = h111_sx2 + (h111_sx2 << 3u);
    uint h111_sx4 = h111_sx3 ^ (h111_sx3 >> 11u);
    uint seed111_y = h111_sx4 + (h111_sx4 << 15u);
    uint h111_sy1 = seed111_y + (seed111_y << 10u);
    uint h111_sy2 = h111_sy1 ^ (h111_sy1 >> 6u);
    uint h111_sy3 = h111_sy2 + (h111_sy2 << 3u);
    uint seed111_z = h111_sy3 ^ (h111_sy3 >> 11u);
        // ---- 3D perlin 位移 ----
        // 对 8 个格角的整型哈希做三线性插值;每角哈希链为:格坐标乘 1664525 混入
        // z/y/x,再走两级雪崩(>>11、<<7&2636928640、<<15&4022730752),最后取低 23 位
        // 映射到 [-1,1] 作为噪声值。
    float3 smooth = (f * f) * (3.0f.xxx - (f * 2.0f));
    float smooth_x = smooth.x;
    float smooth_1mx = 1.0f - smooth_x;
    float smooth_y = smooth.y;
    float smooth_1my = 1.0f - smooth_y;
    float smooth_z = smooth.z;
    float smooth_1mz = 1.0f - smooth_z;
    float3 noise_a = (((((float3(asfloat((seed000_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed000_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed000_z + (seed000_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_1my) * smooth_1mz) + (((float3(asfloat((seed100_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed100_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed100_z + (seed100_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_1my) * smooth_1mz)) + (((float3(asfloat((seed010_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed010_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed010_z + (seed010_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_y) * smooth_1mz)) + (((float3(asfloat((seed110_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed110_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed110_z + (seed110_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y) * smooth_1mz);
    float3 noise_b = (((noise_a + (((float3(asfloat((seed001_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed001_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed001_z + (seed001_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_1my) * smooth_z)) + (((float3(asfloat((seed101_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed101_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed101_z + (seed101_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_1my) * smooth_z)) + (((float3(asfloat((seed011_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed011_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed011_z + (seed011_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_y) * smooth_z)) + (((float3(asfloat((seed111_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed111_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed111_z + (seed111_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y) * smooth_z);
    // 把噪声位移(只取 xz)按高度 v 缩放加到顶点位置,完成瀑布的流动形变。
    float3 final_pos = pos + ((((noise_b - 0.5f.xxx) * float3(1.0f, 0.0f, 1.0f)) * uv_y) * 0.20000000298023223876953125f);
    // 世界 -> 采样空间;C0 为合成法线(网格法线 * 切向斜率项 + 少量 y 分量)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(final_pos, 1.0f));
    out_var_C0 = normalize((n * (-((mesh.position.y - mesh.position.w) * ((-3.0f) * pow(uv_y, 2.0f))))) + float3(0.0f, 0.5f, 0.0f));
    out_var_C1 = final_pos;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _waterfall.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct RippleSourceSsbo
{
    uint4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
};

struct DofFocus
{
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};

static uint zero_u;
static float zero_f;
static float zero_f2;
static float zero_f3;
static float3 zero_f3v;
static float2 zero_f2v;

static const uint2 quarter_offsets[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
static const float noise_freqs[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float noise_weights[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint noise_seeds[4] = { 17u, 18u, 20u, 21u };

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer ripple_source_buffer : register(t26, space0);
ByteAddressBuffer autofocus_buf : register(t33, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t18, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> water_mask : register(t28, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);
Texture2D<float4> refraction_tex : register(t30, space0);
Texture2D<float4> taa_history_tex : register(t31, space0);
Texture2D<float4> reprojection_tex : register(t32, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

// 打包两个 half 成 uint(half2x16)。
uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

// 解包 uint 成两个 half。
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void frag_main()
{
    bool is_theme4;
    bool is_helper = IsHelperLane();
    float3 world_pos = in_var_C1;
    uint2 pixel_uv = uint2(gl_FragCoord.xy);
    uint carry_m1;
    uint carry_m2;
    bool carry_m3;
    bool carry_m4;
    uint carry_m5;
    uint carry_m6;
    bool carry_m7;
    bool carry_m8;
    float3 water_normal;
    do
    {
        is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            carry_m1 = zero_u;
            carry_m2 = zero_u;
            carry_m3 = false;
            carry_m4 = false;
            carry_m5 = zero_u;
            carry_m6 = zero_u;
            carry_m7 = false;
            carry_m8 = false;
            water_normal = float3(0.0f, 1.0f, 0.0f);
            break;
        }
        uint carry_n1;
        uint carry_n2;
        bool carry_n3;
        bool carry_n4;
        uint carry_n5;
        uint carry_n6;
        bool carry_n7;
        bool carry_n8;
        float3 normal_off;
        do
        {
            if (is_theme4)
            {
                carry_n1 = zero_u;
                carry_n2 = zero_u;
                carry_n3 = false;
                carry_n4 = false;
                carry_n5 = zero_u;
                carry_n6 = zero_u;
                carry_n7 = false;
                carry_n8 = false;
                normal_off = 0.0f.xxx;
                break;
            }
            float game_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
            uint carry_a;
            uint carry_b;
            bool carry_c;
            bool carry_d;
            uint carry_e;
            uint carry_f;
            bool carry_g;
            bool carry_h;
            carry_a = zero_u;
            carry_b = zero_u;
            carry_c = false;
            carry_d = false;
            carry_e = zero_u;
            carry_f = zero_u;
            carry_g = false;
            carry_h = false;
            float2 noise_offsets[4];
            for (uint i_noise = 0u; i_noise < 4u; )
            {
                float seed_f = float(noise_seeds[i_noise]);
                int seed_int = int(floor(seed_f));
                float seed_frac = frac(seed_f);
                uint seed_u = uint(seed_int);
                uint seed_u1 = seed_u + (seed_u << 10u);
                uint seed_u2 = seed_u1 ^ (seed_u1 >> 6u);
                uint seed_u3 = seed_u2 + (seed_u2 << 3u);
                uint seed_u4 = seed_u3 ^ (seed_u3 >> 11u);
                bool sel_a = carry_h ? carry_h : true;
                uint sel_b = carry_h ? carry_f : 1664525u;
                bool sel_c = carry_g ? carry_g : true;
                uint sel_d = carry_g ? carry_e : 1013904223u;
                uint hash0 = (((seed_u * sel_b) + (seed_u4 + (seed_u4 << 15u))) + sel_d) * sel_b;
                uint hash0b = hash0 ^ (hash0 >> 11u);
                uint hash0c = hash0b ^ ((hash0b << 7u) & 2636928640u);
                uint hash0d = hash0c ^ ((hash0c << 15u) & 4022730752u);
                uint hash0e = hash0d ^ (hash0d >> 18u);
                uint hash0f = hash0e + (hash0e << 10u);
                uint hash0g = hash0f ^ (hash0f >> 6u);
                uint hash0h = hash0g + (hash0g << 3u);
                uint hash0i = hash0h ^ (hash0h >> 11u);
                bool sel_e = carry_d ? carry_d : true;
                uint sel_f = carry_d ? carry_b : 8388607u;
                bool sel_g = carry_c ? carry_c : true;
                uint sel_h = carry_c ? carry_a : 1065353216u;
                bool sel_e2 = sel_e ? sel_e : true;
                uint sel_f2 = sel_e ? sel_f : 8388607u;
                bool sel_g2 = sel_g ? sel_g : true;
                uint sel_h2 = sel_g ? sel_h : 1065353216u;
                uint seed_x1 = uint(seed_int + 1);
                uint seed_x1a = seed_x1 + (seed_x1 << 10u);
                uint seed_x1b = seed_x1a ^ (seed_x1a >> 6u);
                uint seed_x1c = seed_x1b + (seed_x1b << 3u);
                uint seed_x1d = seed_x1c ^ (seed_x1c >> 11u);
                uint carry_f2 = sel_a ? sel_b : 1664525u;
                uint carry_e2 = sel_c ? sel_d : 1013904223u;
                uint hash1 = (((seed_x1 * carry_f2) + (seed_x1d + (seed_x1d << 15u))) + carry_e2) * carry_f2;
                uint hash1b = hash1 ^ (hash1 >> 11u);
                uint hash1c = hash1b ^ ((hash1b << 7u) & 2636928640u);
                uint hash1d = hash1c ^ ((hash1c << 15u) & 4022730752u);
                uint hash1e = hash1d ^ (hash1d >> 18u);
                uint hash1f = hash1e + (hash1e << 10u);
                uint hash1g = hash1f ^ (hash1f >> 6u);
                uint hash1h = hash1g + (hash1g << 3u);
                uint hash1i = hash1h ^ (hash1h >> 11u);
                bool sel_i = sel_e2 ? sel_e2 : true;
                uint sel_j = sel_e2 ? sel_f2 : 8388607u;
                bool sel_k = sel_g2 ? sel_g2 : true;
                uint sel_l = sel_g2 ? sel_h2 : 1065353216u;
                uint carry_b2 = sel_i ? sel_j : 8388607u;
                uint carry_a2 = sel_k ? sel_l : 1065353216u;
                float smooth_f = (seed_frac * seed_frac) * (3.0f - (2.0f * seed_frac));
                noise_offsets[i_noise] = (normalize(((float2(asfloat((hash0e & sel_f) | sel_h) - 1.0f, asfloat(((hash0i + (hash0i << 15u)) & sel_f2) | sel_h2) - 1.0f) * (1.0f - smooth_f)) + (float2(asfloat((hash1e & sel_j) | sel_l) - 1.0f, asfloat(((hash1i + (hash1i << 15u)) & carry_b2) | carry_a2) - 1.0f) * smooth_f)) - 0.5f.xx) * game_time) * 3.0f;
                i_noise++;
                carry_a = carry_a2;
                carry_b = carry_b2;
                carry_c = sel_k ? sel_k : true;
                carry_d = sel_i ? sel_i : true;
                carry_e = carry_e2;
                carry_f = carry_f2;
                carry_g = sel_c ? sel_c : true;
                carry_h = sel_a ? sel_a : true;
                continue;
            }
            float3 normal_acc;
            normal_acc = 0.0f.xxx;
            for (uint i_oct = 0u; i_oct < 4u; )
            {
                uint oct_next = i_oct + 1u;
                float oct_angle = float(oct_next) * 10.16640758514404296875f;
                float oct_cos = cos(oct_angle);
                float oct_sin = sin(oct_angle);
                float2x2 rot_mat = float2x2(float2(oct_cos, oct_sin), float2(-oct_sin, oct_cos));
                float2 oct_delta = normal_acc.yz + mul(((((water_noise_texture.Sample(sampler_llr, mul(rot_mat, (world_pos.xz * noise_freqs[i_oct]) + noise_offsets[i_oct]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * noise_weights[i_oct]) * float2(1.0f, noise_freqs[i_oct] * 0.01200000010430812835693359375f).xyy).yz, rot_mat);
                normal_acc = float3(normal_acc.x, oct_delta.x, oct_delta.y);
                i_oct = oct_next;
                continue;
            }
            carry_n1 = carry_a;
            carry_n2 = carry_b;
            carry_n3 = carry_c;
            carry_n4 = carry_d;
            carry_n5 = carry_e;
            carry_n6 = carry_f;
            carry_n7 = carry_g;
            carry_n8 = carry_h;
            normal_off = normal_acc;
            break;
        } while(false);
        float3 ripple_total;
        do
        {
            if (is_theme4)
            {
                ripple_total = 0.0f.xxx;
                break;
            }
            uint ripple_count = min(32u, ripple_source_buffer.Load<uint>(512));
            float3 ripple_acc;
            ripple_acc = 0.0f.xxx;
            for (uint i_ripple = 0u; i_ripple < ripple_count; )
            {
                float2 r_pos_a = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).x >> 16u) & 65535u).x);
                float2 r_pos_b = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).y >> 16u) & 65535u).x);
                float2 r_center = lerp(r_pos_a, r_pos_b, 0.5f.xx);
                float2 r_off = r_center - world_pos.xz;
                float r_dist = length(r_off);
                float2 r_vel = (r_pos_b - r_pos_a) * 3.0f;
                float r_dist2 = r_dist + dot(r_off, r_vel);
                float r_time = (float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f;
                float r_max1 = max(r_dist, r_dist2);
                float r_max1sq = r_max1 * r_max1;
                float r_wave = sin((r_dist2 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max1sq * r_max1sq));
                float2 r_off2 = r_center - (world_pos.xz - float2(0.00999999977648258209228515625f, 0.0f));
                float r_dist3 = length(r_off2);
                float r_dist4 = r_dist3 + dot(r_off2, r_vel);
                float r_max2 = max(r_dist3, r_dist4);
                float r_max2sq = r_max2 * r_max2;
                float2 r_off3 = r_center - (world_pos.xz - float2(0.0f, 0.00999999977648258209228515625f));
                float r_dist5 = length(r_off3);
                float r_dist6 = r_dist5 + dot(r_off3, r_vel);
                float r_max3 = max(r_dist5, r_dist6);
                float r_max3sq = r_max3 * r_max3;
                float3 r_waves = float3(r_wave, (sin((r_dist4 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max2sq * r_max2sq))) - r_wave, (sin((r_dist6 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max3sq * r_max3sq))) - r_wave);
                float2 r_waves_half = r_waves.yz * 0.5f;
                ripple_acc += (float3(r_waves.x, r_waves_half.x, r_waves_half.y) * asfloat(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).z));
                i_ripple++;
                continue;
            }
            ripple_total = ripple_acc;
            break;
        } while(false);
        // 水面法线构造:把两层法线偏移(横波+涟漪)合并,再归一化。
        float3 normal_ts = float3(normal_off.y, 1.0f, normal_off.z);
        float2 normal_ts2 = normal_ts.xz + (float3(ripple_total.y, 1.0f, ripple_total.z).xz / abs(1.0f).xx);
        carry_m1 = carry_n1;
        carry_m2 = carry_n2;
        carry_m3 = carry_n3;
        carry_m4 = carry_n4;
        carry_m5 = carry_n5;
        carry_m6 = carry_n6;
        carry_m7 = carry_n7;
        carry_m8 = carry_n8;
        water_normal = normalize(float3(normal_ts2.x, normal_ts.y, normal_ts2.y));
        break;
    } while(false);
    float far_z;
    // ---- 主光线构造:片元坐标 -> NDC -> 视图/世界光线 ----
    // 含 TAA 抖动:按帧索引 4 周期取 quarter_offsets 做子像素偏移。
    float2 ndc_uv = (((float2(int2((pixel_uv + uint2(1u, 1u)) - quarter_offsets[frame_constants_frame_index & 3u])) + 0.5f.xx) / float2(framebuffer_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc_uv.x;
    float4 clip_ray = float4(ndc_x, ndc_uv.y, gl_FragCoord.z, 1.0f);
    float4 view_ray = mul(frame_constants_view_constants[0u].sample_to_view, clip_ray);
    float4 world_ray = mul(frame_constants_view_constants[0u].view_to_world, view_ray);
    float3 view_dir = normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc_uv.y, 0.0f, 1.0f))).xyz);
    float3 to_camera = -view_dir;
    float4 up4 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    float3 up_vec = up4.xyz;
    float ndotl = 1.0f - dot(to_camera, water_normal);
    float ndotl2 = ndotl * ndotl;
    // Schlick 菲涅尔近似(水面掠射角反射增强)。
    float fresnel = 0.959999978542327880859375f * ((ndotl2 * ndotl2) * ndotl);
    float frame_angle = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float g_noise = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_uv.x) + frame_angle)) + (0.005837149918079376220703125f * (float(pixel_uv.y) + frame_angle))));
    uint2 half_px = pixel_uv / uint2(2u, 2u);
    float4 repro_world = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc_uv.y, half_depth_tex.Load(int3(half_px, 0u)).x, 1.0f)));
    float2 half_dims = float2(half_depth_tex_dims);
    float3 ray_org = clip_ray.xyz;
    float4 march_start = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(world_pos + (normalize(view_dir - ((float3(water_normal.x, 0.0f, water_normal.z) * 0.5f) * 2.0f)) * min(4.0f, 4.0f * length(world_pos - (repro_world.xyz / repro_world.w.xxx)))), 1.0f)));
    float3 march_clip = march_start.xyz / march_start.w.xxx;
    // ---- 折射光线 marching 区间:从水面沿视线方向射向水下场景 ----
    // 先对从水面世界位置沿视线前推一个最小距离得到的点做裁剪空间 slab 求交,
    // 得到 marching 的起止点(march_p0/march_p1)。
    float4 march_ray = clip_ray + (float4(march_clip - ray_org, 0.0f) * float(int(sign(march_clip.z))));
    float march_w = march_ray.w;
    float3 march_end = (march_ray / (float((march_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(march_w))).xxxx).xyz;
    float3 march_dir = march_end - ray_org;
    bool3 march_neg = bool3(march_dir.x < 0.0f.xxx.x, march_dir.y < 0.0f.xxx.y, march_dir.z < 0.0f.xxx.z);
    float3 march_t = (float3(march_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - ray_org) / march_dir;
    float3 march_p0 = ray_org + (march_dir * max(0.0f, max(march_t.x, march_t.y)));
    float3 march_delta = march_end - march_p0;
    bool3 march_pos = bool3(march_delta.x >= 0.0f.xxx.x, march_delta.y >= 0.0f.xxx.y, march_delta.z >= 0.0f.xxx.z);
    float3 march_t2 = (float3(march_pos.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_pos.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_pos.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - march_p0) / march_delta;
    float3 march_p1 = march_p0 + (march_delta * min(1.0f, min(min(march_t2.x, march_t2.y), march_t2.z)));
    float water_hit_t;
    do
    {
        // ---- 在 half-depth 缓冲中步进求折射遮挡点 ----
        // 沿光线等距采样 half_depth_tex,比较场景深度与光线深度,找首交点;
        // 命中后用二分法精化命中距离。
        far_z = frame_constants_view_constants[0u].clip_to_view[3].z;
        float thickness = 0.100000001490116119384765625f * far_z;
        uint steps = uint(int(max(2u, min(3u, uint(int(floor(length((((march_p1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((march_p0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float mres_tprev;
        float res_t;
        float res_diff;
        float res_thick;
        float res_hitdiff;
        bool res_hit;
        do
        {
            float3 step_delta = march_p1 - march_p0;
            float step_cnt;
            if (march_p1.z > 9.5367431640625e-07f)
            {
                step_cnt = float(steps);
            }
            else
            {
                step_cnt = max(1.0f, float(steps) - 1.0f);
            }
            float step_inv = 1.0f / step_cnt;
            float mres_t0;
            float mres_diff;
            float mres_hitdiff;
            float mres_thick;
            float mres_t1;
            bool mres_hit;
            if (steps > 0u)
            {
                float step_t = lerp(0.0f, 1.0f, pow(g_noise * step_inv, 1.0f));
                float3 step_pos = march_p0 + (step_delta * step_t);
                float step_invz = 1.0f / step_pos.z;
                float4 step_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (step_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float step_depth = step_depth4.x;
                float step_diff = (1.00000202655792236328125f / step_depth) - step_invz;
                bool step_hit = (step_diff < 0.0f) ? true : false;
                float bis_t0;
                float bis_diff;
                float bis_hitdiff;
                float bis_thick;
                float bis_t1;
                bool bis_hit;
                float bis_tprev;
                if (step_hit)
                {
                    bis_t0 = 0.0f;
                    bis_diff = step_diff;
                    bis_hitdiff = step_invz - (1.0f / step_depth);
                    bis_thick = thickness;
                    bis_t1 = step_t;
                    bis_hit = step_hit;
                    bis_tprev = 0.0f;
                }
                else
                {
                    float prev_diff;
                    float prev_t;
                    prev_diff = step_diff;
                    prev_t = step_t;
                    float prev_hitdiff;
                    bool cur_hit;
                    float cur_t;
                    float out_diff;
                    float out_hitdiff;
                    float out_thick;
                    float out_t;
                    bool out_hit;
                    uint step_i = 1u;
                    bool prev_hit = step_hit;
                    for (;;)
                    {
                        if (step_i < steps)
                        {
                            cur_t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + g_noise) * step_inv, 1.0f)));
                            float3 cur_pos = march_p0 + (step_delta * cur_t);
                            float cur_invz = 1.0f / cur_pos.z;
                            float4 cur_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (cur_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float cur_depth = cur_depth4.x;
                            prev_hitdiff = (1.00000202655792236328125f / cur_depth) - cur_invz;
                            cur_hit = (prev_hitdiff < 0.0f) ? true : false;
                            if (cur_hit)
                            {
                                out_diff = prev_hitdiff;
                                out_hitdiff = cur_invz - (1.0f / cur_depth);
                                out_thick = thickness;
                                out_t = cur_t;
                                out_hit = cur_hit;
                                break;
                            }
                            step_i++;
                            prev_diff = prev_hitdiff;
                            prev_hit = cur_hit;
                            prev_t = cur_t;
                            continue;
                        }
                        else
                        {
                            out_diff = 0.0f;
                            out_hitdiff = 0.0f;
                            out_thick = 0.0f;
                            out_t = 1.0f;
                            out_hit = prev_hit;
                            break;
                        }
                    }
                    bis_t0 = prev_diff;
                    bis_diff = out_diff;
                    bis_hitdiff = out_hitdiff;
                    bis_thick = out_thick;
                    bis_t1 = out_t;
                    bis_hit = out_hit;
                    bis_tprev = prev_t;
                }
                mres_t0 = bis_t0;
                mres_diff = bis_diff;
                mres_hitdiff = bis_hitdiff;
                mres_thick = bis_thick;
                mres_t1 = bis_t1;
                mres_hit = bis_hit;
                mres_tprev = bis_tprev;
            }
            else
            {
                mres_t0 = 0.0f;
                mres_diff = 0.0f;
                mres_hitdiff = 0.0f;
                mres_thick = 0.0f;
                mres_t1 = 1.0f;
                mres_hit = false;
                mres_tprev = 0.0f;
            }
            if (mres_hit)
            {
                float bis_t0b;
                float bis_diffb;
                float bis_hitdiffb;
                float bis_thickb;
                float bis_t1b;
                float bis_tprevb;
                bis_t0b = mres_tprev;
                bis_diffb = mres_t0;
                bis_hitdiffb = mres_diff;
                bis_thickb = mres_hitdiff;
                bis_t1b = mres_thick;
                bis_tprevb = mres_t1;
                for (uint bis_loop = 0u; bis_loop < 1u; )
                {
                    float bis_mid = bis_t0b + bis_tprevb;
                    float bis_mid2 = bis_mid * 0.5f;
                    float3 bis_pos = march_p0 + (step_delta * bis_mid2);
                    float bis_invz = 1.0f / bis_pos.z;
                    float4 bis_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (bis_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float bis_depth = bis_depth4.x;
                    float bis_diff2 = (1.00000202655792236328125f / bis_depth) - bis_invz;
                    bool bis_hit2 = (bis_diff2 < 0.0f) ? true : false;
                    bis_t0b = bis_hit2 ? bis_t0b : bis_mid2;
                    bis_diffb = bis_hit2 ? bis_diffb : bis_diff2;
                    bis_hitdiffb = bis_hit2 ? bis_diff2 : bis_hitdiffb;
                    bis_thickb = bis_hit2 ? (bis_invz - (1.0f / bis_depth)) : bis_thickb;
                    bis_t1b = bis_hit2 ? thickness : bis_t1b;
                    bis_tprevb = bis_hit2 ? bis_mid2 : bis_tprevb;
                    bis_loop++;
                    continue;
                }
                float final_t = lerp(bis_t0b, bis_tprevb, bis_diffb / (bis_diffb + (-bis_hitdiffb)));
                float3 final_pos = march_p0 + (step_delta * final_t);
                float final_invz = 1.0f / final_pos.z;
                float4 final_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (final_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float final_depth = final_depth4.x;
                float final_diff = (1.00000202655792236328125f / final_depth) - final_invz;
                bool final_hit = (abs(final_diff) < (bis_diffb * 0.89999997615814208984375f)) ? true : false;
                res_t = final_hit ? final_t : bis_tprevb;
                res_diff = final_hit ? final_diff : bis_hitdiffb;
                res_thick = final_hit ? thickness : bis_t1b;
                res_hitdiff = final_hit ? (final_invz - (1.0f / final_depth)) : bis_thickb;
                res_hit = true;
                break;
            }
            else
            {
                res_t = mres_tprev;
                res_diff = zero_f;
                res_thick = zero_f;
                res_hitdiff = zero_f;
                res_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool cond_hit;
        if (res_hit)
        {
            bool cond_t;
            if (res_hitdiff < res_thick)
            {
                cond_t = res_diff < res_thick;
            }
            else
            {
                cond_t = false;
            }
            cond_hit = cond_t;
        }
        else
        {
            cond_hit = false;
        }
        if (cond_hit)
        {
            water_hit_t = res_t;
            break;
        }
        water_hit_t = mres_tprev;
        break;
    } while(false);
    // 折射光线最终命中点(水面下遮挡位置)。
    float3 water_hit = lerp(march_p0, march_p1, water_hit_t.xxx);
    float water_hit_x = water_hit.x;
    float2 water_uv = (float4(water_hit_x, water_hit.y, zero_f2, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float4 water_world4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(water_hit_x, water_hit.y, half_depth_tex.SampleLevel(sampler_nnc, water_uv, 0.0f).x, 1.0f)));
    float3 water_world = water_world4.xyz / water_world4.w.xxx;
    float water_y = water_world.y;
    float water_dy = abs(world_pos.y - water_y);
    int2 px_i = int2(pixel_uv);
    // ---- 水面阴影:把水面片元投影到阴影贴图,PCF 软阴影 ----
    // 在 shadow_map 中取 5 个深度采样求平均遮挡,再叠加 10 次 PCF 软阴影。
    float2 shadow_total;
    do
    {
        float4 sample_pos4 = mul(frame_constants_view_constants[2u].world_to_sample, float4(world_pos, 1.0f));
        if (sample_pos4.z > 1.0f)
        {
            shadow_total = 0.0f.xx;
            break;
        }
        float2 sample_xy = sample_pos4.xy;
        float2 sample_abs = abs(sample_xy);
        bool sample_edge = any(bool2(sample_abs.x > 0.9833333492279052734375f.xx.x, sample_abs.y > 0.9833333492279052734375f.xx.y));
        float2 sample_xy2;
        if (sample_edge)
        {
            sample_xy2 = sample_xy * 0.5f;
        }
        else
        {
            sample_xy2 = sample_xy;
        }
        float texel_scale = sample_edge ? 5.0f : 10.0f;
        float sample_scale = sample_edge ? 0.5f : 1.0f;
        float2 sample_abs2 = abs(sample_xy2);
        float2 edge_limit = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * texel_scale));
        bool sample_out = any(bool2(sample_abs2.x > edge_limit.x, sample_abs2.y > edge_limit.y));
        float shadow_tile;
        float2 shadow_uv;
        float shadow_scale;
        if (sample_out)
        {
            shadow_tile = texel_scale * 0.2857142984867095947265625f;
            shadow_uv = sample_xy2 * 0.2857142984867095947265625f;
            shadow_scale = sample_scale * 0.2857142984867095947265625f;
        }
        else
        {
            shadow_tile = texel_scale;
            shadow_uv = sample_xy2;
            shadow_scale = sample_scale;
        }
        float shadow_layer = float(sample_out ? 2u : uint(sample_edge));
        uint shadow_view = 2u + uint(shadow_layer);
        float normal_dot = dot(water_normal, -up_vec);
        float4 shadow_pos4 = float4(shadow_uv.x, shadow_uv.y, sample_pos4.z, sample_pos4.w) + mul(frame_constants_view_constants[shadow_view].world_to_sample, float4((water_normal * (0.066666670143604278564453125f * clamp(1.0f - normal_dot, 0.0f, 1.0f))) / shadow_scale.xxx, 0.0f));
        float2 shadow_uv2 = (shadow_pos4.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float3 shadow_uv3 = float3(shadow_uv2.x, shadow_uv2.y, shadow_pos4.z);
        float shadow_bias = max((0.300000011920928955078125f / shadow_scale) * (1.0f - abs(normal_dot)), 0.02500000037252902984619140625f);
        float px_angle = float((uint((px_i.x * 11) + (px_i.y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        uint2 px_u2 = uint2(px_i);
        float px_noise = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_u2.x) + frame_angle)) + (0.005837149918079376220703125f * (float(px_u2.y) + frame_angle))));
        float4 normal_shadow4 = mul(frame_constants_view_constants[shadow_view].world_to_sample, float4(water_normal, 0.0f));
        float shadow_sum;
        int shadow_cnt;
        shadow_sum = 0.0f;
        shadow_cnt = 0;
        float shadow_sum2;
        int shadow_cnt2;
        for (int shadow_i = 0; shadow_i < 5; shadow_sum = shadow_sum2, shadow_cnt = shadow_cnt2, shadow_i++)
        {
            float shadow_i_f = float(shadow_i);
            float shadow_ang = (shadow_i_f + px_angle) * 2.3999631404876708984375f;
            float3 shadow_ray = shadow_uv3 + float3((((float2(cos(shadow_ang), sin(shadow_ang)) * (shadow_i_f + px_noise)) * 0.20000000298023223876953125f.xx) * shadow_tile) * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
            float4 shadow_depth4 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(shadow_ray.xy, shadow_layer), 0.0f);
            float shadow_depth = shadow_depth4.x;
            if (shadow_ray.z > shadow_depth)
            {
                shadow_sum2 = shadow_sum + shadow_depth;
                shadow_cnt2 = shadow_cnt + 1;
            }
            else
            {
                shadow_sum2 = shadow_sum;
                shadow_cnt2 = shadow_cnt;
            }
        }
        float shadow_avg = shadow_sum / float(shadow_cnt);
        if (shadow_cnt == 0)
        {
            shadow_total = 0.0f.xx;
            break;
        }
        float shadow_frac = max(((shadow_pos4.z - shadow_avg) * ((17.0666675567626953125f * shadow_tile) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / shadow_avg, 0.0f);
        float shadow_soft = (shadow_frac / pow(pow(shadow_frac / (shadow_tile - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
        float normal_zs = normal_shadow4.z;
        float2 shadow_slope = (((normal_shadow4.xy * float(int(sign(normal_zs)))) / max(0.1500000059604644775390625f, abs(normal_zs)).xx) * 0.0009765625f.xx) * 2.0f;
        float2 shadow_acc;
        shadow_acc = 0.0f.xx;
        for (int pcf_i = 0; pcf_i < 10; )
        {
            float pcf_i_f = float(pcf_i);
            float pcf_ang = (pcf_i_f + px_angle) * 2.3999631404876708984375f;
            float2 pcf_off = ((float2(cos(pcf_ang), sin(pcf_ang)) * (pcf_i_f + px_noise)) * 0.100000001490116119384765625f.xx) * shadow_soft;
            float3 pcf_pos = shadow_uv3 + float3(pcf_off * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
            shadow_acc += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(pcf_pos.xy, shadow_layer), pcf_pos.z + min(0.0f, dot(shadow_slope, pcf_off))), zero_f3);
            pcf_i++;
            continue;
        }
        shadow_total = shadow_acc * 0.100000001490116119384765625f;
        break;
    } while(false);
    // ---- 景深散焦(依赖自动对焦缓冲) ----
    // 用相机对焦距离与锁焦状态算出片元的景深模糊量 dof_blur。
    float focus_lock = autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 56);
    float focus_inv0 = 1.0f / autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 0);
    float focus_inv1 = 1.0f / autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 4);
    float focus_hi = max(0.001000000047497451305389404296875f, max(focus_inv0, focus_inv1)) + 9.9999997473787516355514526367188e-06f;
    float focus_lo = max(0.001000000047497451305389404296875f, min(focus_inv0, focus_inv1)) - 9.9999997473787516355514526367188e-06f;
    bool focus_locked = autofocus_buf.Load<uint>(frame_constants_camera_idx * 64 + 44) != 0u;
    float3 focus_pos = world_pos;
    focus_pos.y = world_pos.y * float(focus_locked ? 0 : 1);
    float4 cam_pos4 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_dir = normalize(mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float cam_depth = 1.0f / max(1.0000000133514319600180897396058e-10f, dot(focus_pos - (cam_pos4.xyz / cam_pos4.w.xxx), cam_dir));
    float focus_range = focus_hi - focus_lo;
    float focus_eps = min(0.00999999977648258209228515625f, 0.0500000007450580596923828125f * focus_range);
    float focus_dof;
    if (cam_depth > focus_hi)
    {
        focus_dof = (cam_depth - focus_hi) + focus_eps;
    }
    else
    {
        float focus_dof2;
        if (cam_depth < focus_lo)
        {
            focus_dof2 = (cam_depth - focus_lo) - focus_eps;
        }
        else
        {
            focus_dof2 = (-focus_eps) + (((2.0f * focus_eps) * (cam_depth - focus_lo)) / focus_range);
        }
        focus_dof = focus_dof2;
    }
    float focus_dof3;
    if (focus_locked)
    {
        focus_dof3 = focus_dof / max(0.00999999977648258209228515625f, length(cam_dir.xz));
    }
    else
    {
        focus_dof3 = focus_dof;
    }
    float dof_blur = sqrt(lerp(0.16000001132488250732421875f, 9.9999997473787516355514526367188e-05f, exp2((-0.014999999664723873138427734375f) * sqrt(max(abs(((-focus_dof3) * focus_lock) * frame_constants_view_constants[0u].view_to_clip[1].y), (view_ray.xyz / view_ray.w.xxx).z * (-0.0030000000260770320892333984375f))))));
    // ---- 镜面反射光线:反射视线方向,做 y 翻转(避免水下自反射) ----
    // 反射光线同样在 half-depth 中 marching,得到反射命中点与距离。
    float3 refl_dir = -reflect(to_camera, water_normal);
    float refl_y = refl_dir.y;
    float3 refl_dir2;
    if (refl_y < 0.0f)
    {
        float3 refl_tmp = refl_dir;
        refl_tmp.y = -refl_y;
        refl_dir2 = refl_tmp;
    }
    else
    {
        refl_dir2 = refl_dir;
    }
    float4 refl_clip = clip_ray + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(refl_dir2, 0.0f)));
    float refl_w = refl_clip.w;
    float3 refl_end = (refl_clip / (float((refl_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(refl_w))).xxxx).xyz;
    float3 refl_dir3 = refl_end - ray_org;
    bool3 refl_neg = bool3(refl_dir3.x < 0.0f.xxx.x, refl_dir3.y < 0.0f.xxx.y, refl_dir3.z < 0.0f.xxx.z);
    float3 refl_slab_t = (float3(refl_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, refl_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, refl_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - ray_org) / refl_dir3;
    float3 refl_p0 = ray_org + (refl_dir3 * max(0.0f, max(refl_slab_t.x, refl_slab_t.y)));
    float3 refl_delta = refl_end - refl_p0;
    bool3 refl_pos = bool3(refl_delta.x >= 0.0f.xxx.x, refl_delta.y >= 0.0f.xxx.y, refl_delta.z >= 0.0f.xxx.z);
    float3 refl_t2 = (float3(refl_pos.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, refl_pos.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, refl_pos.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - refl_p0) / refl_delta;
    float3 refl_p1 = refl_p0 + (refl_delta * min(min(refl_t2.x, refl_t2.y), refl_t2.z));
    bool refl_hit;
    float refl_t;
    do
    {
        // 反射光线步进:采样数 2..8 由屏幕距离决定,二分精化命中点。
        uint refl_steps = uint(int(max(2u, min(8u, uint(int(floor(length((((refl_p1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((refl_p0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));
        float refl_mres_tprev;
        float refl_res_t;
        float refl_res_diff;
        float refl_res_thick;
        float refl_res_hitdiff;
        bool refl_res_hit;
        do
        {
            float3 refl_step = refl_p1 - refl_p0;
            float refl_cnt;
            if (refl_p1.z > 9.5367431640625e-07f)
            {
                refl_cnt = float(refl_steps);
            }
            else
            {
                refl_cnt = max(1.0f, float(refl_steps) - 1.0f);
            }
            float refl_inv = 1.0f / refl_cnt;
            float refl_mres_t0;
            float refl_mres_diff;
            float refl_mres_hitdiff;
            float refl_mres_thick;
            float refl_mres_t1;
            bool refl_mres_hit;
            if (refl_steps > 0u)
            {
                float refl_first_t = lerp(0.0f, 1.0f, pow(g_noise * refl_inv, 1.0f));
                float3 refl_pos2 = refl_p0 + (refl_step * refl_first_t);
                float refl_invz = 1.0f / refl_pos2.z;
                float4 refl_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_pos2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float refl_depth = refl_depth4.x;
                float refl_diff = (1.00000202655792236328125f / refl_depth) - refl_invz;
                bool refl_first_hit = (refl_diff < 0.0f) ? true : false;
                float refl_bis_t0;
                float refl_bis_diff;
                float refl_bis_hitdiff;
                float refl_bis_thick;
                float refl_bis_t1;
                bool refl_bis_hit;
                float refl_bis_tprev;
                if (refl_first_hit)
                {
                    refl_bis_t0 = 0.0f;
                    refl_bis_diff = refl_diff;
                    refl_bis_hitdiff = refl_invz - (1.0f / refl_depth);
                    refl_bis_thick = far_z;
                    refl_bis_t1 = refl_first_t;
                    refl_bis_hit = refl_first_hit;
                    refl_bis_tprev = 0.0f;
                }
                else
                {
                    float refl_prev_diff;
                    float refl_prev_t;
                    refl_prev_diff = refl_diff;
                    refl_prev_t = refl_first_t;
                    float refl_cur_diff;
                    bool refl_cur_hit;
                    float refl_cur_t;
                    float refl_out_diff;
                    float refl_out_hitdiff;
                    float refl_out_thick;
                    float refl_out_t;
                    bool refl_out_hit;
                    uint refl_i = 1u;
                    bool refl_prev_hit = refl_first_hit;
                    for (;;)
                    {
                        if (refl_i < refl_steps)
                        {
                            refl_cur_t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(refl_i) + g_noise) * refl_inv, 1.0f)));
                            float3 refl_cur_pos = refl_p0 + (refl_step * refl_cur_t);
                            float refl_cur_invz = 1.0f / refl_cur_pos.z;
                            float4 refl_cur_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_cur_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float refl_cur_depth = refl_cur_depth4.x;
                            refl_cur_diff = (1.00000202655792236328125f / refl_cur_depth) - refl_cur_invz;
                            refl_cur_hit = (refl_cur_diff < 0.0f) ? true : false;
                            if (refl_cur_hit)
                            {
                                refl_out_diff = refl_cur_diff;
                                refl_out_hitdiff = refl_cur_invz - (1.0f / refl_cur_depth);
                                refl_out_thick = far_z;
                                refl_out_t = refl_cur_t;
                                refl_out_hit = refl_cur_hit;
                                break;
                            }
                            refl_i++;
                            refl_prev_diff = refl_cur_diff;
                            refl_prev_hit = refl_cur_hit;
                            refl_prev_t = refl_cur_t;
                            continue;
                        }
                        else
                        {
                            refl_out_diff = 0.0f;
                            refl_out_hitdiff = 0.0f;
                            refl_out_thick = 0.0f;
                            refl_out_t = 1.0f;
                            refl_out_hit = refl_prev_hit;
                            break;
                        }
                    }
                    refl_bis_t0 = refl_prev_diff;
                    refl_bis_diff = refl_out_diff;
                    refl_bis_hitdiff = refl_out_hitdiff;
                    refl_bis_thick = refl_out_thick;
                    refl_bis_t1 = refl_out_t;
                    refl_bis_hit = refl_out_hit;
                    refl_bis_tprev = refl_prev_t;
                }
                refl_mres_t0 = refl_bis_t0;
                refl_mres_diff = refl_bis_diff;
                refl_mres_hitdiff = refl_bis_hitdiff;
                refl_mres_thick = refl_bis_thick;
                refl_mres_t1 = refl_bis_t1;
                refl_mres_hit = refl_bis_hit;
                refl_mres_tprev = refl_bis_tprev;
            }
            else
            {
                refl_mres_t0 = 0.0f;
                refl_mres_diff = 0.0f;
                refl_mres_hitdiff = 0.0f;
                refl_mres_thick = 0.0f;
                refl_mres_t1 = 1.0f;
                refl_mres_hit = false;
                refl_mres_tprev = 0.0f;
            }
            if (refl_mres_hit)
            {
                float refl_bis_t0b;
                float refl_bis_diffb;
                float refl_bis_hitdiffb;
                float refl_bis_thickb;
                float refl_bis_t1b;
                float refl_bis_tprevb;
                refl_bis_t0b = refl_mres_tprev;
                refl_bis_diffb = refl_mres_t0;
                refl_bis_hitdiffb = refl_mres_diff;
                refl_bis_thickb = refl_mres_hitdiff;
                refl_bis_t1b = refl_mres_thick;
                refl_bis_tprevb = refl_mres_t1;
                for (uint refl_bis_loop = 0u; refl_bis_loop < 3u; )
                {
                    float refl_bis_mid = refl_bis_t0b + refl_bis_tprevb;
                    float refl_bis_mid2 = refl_bis_mid * 0.5f;
                    float3 refl_bis_pos = refl_p0 + (refl_step * refl_bis_mid2);
                    float refl_bis_invz = 1.0f / refl_bis_pos.z;
                    float4 refl_bis_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_bis_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float refl_bis_depth = refl_bis_depth4.x;
                    float refl_bis_diff2 = (1.00000202655792236328125f / refl_bis_depth) - refl_bis_invz;
                    bool refl_bis_hit2 = (refl_bis_diff2 < 0.0f) ? true : false;
                    refl_bis_t0b = refl_bis_hit2 ? refl_bis_t0b : refl_bis_mid2;
                    refl_bis_diffb = refl_bis_hit2 ? refl_bis_diffb : refl_bis_diff2;
                    refl_bis_hitdiffb = refl_bis_hit2 ? refl_bis_diff2 : refl_bis_hitdiffb;
                    refl_bis_thickb = refl_bis_hit2 ? (refl_bis_invz - (1.0f / refl_bis_depth)) : refl_bis_thickb;
                    refl_bis_t1b = refl_bis_hit2 ? far_z : refl_bis_t1b;
                    refl_bis_tprevb = refl_bis_hit2 ? refl_bis_mid2 : refl_bis_tprevb;
                    refl_bis_loop++;
                    continue;
                }
                float refl_final_t = lerp(refl_bis_t0b, refl_bis_tprevb, refl_bis_diffb / (refl_bis_diffb + (-refl_bis_hitdiffb)));
                float3 refl_final_pos = refl_p0 + (refl_step * refl_final_t);
                float refl_final_invz = 1.0f / refl_final_pos.z;
                float4 refl_final_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_final_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float refl_final_depth = refl_final_depth4.x;
                float refl_final_diff = (1.00000202655792236328125f / refl_final_depth) - refl_final_invz;
                bool refl_final_hit = (abs(refl_final_diff) < (refl_bis_diffb * 0.89999997615814208984375f)) ? true : false;
                refl_res_t = refl_final_hit ? refl_final_t : refl_bis_tprevb;
                refl_res_diff = refl_final_hit ? refl_final_diff : refl_bis_hitdiffb;
                refl_res_thick = refl_final_hit ? far_z : refl_bis_t1b;
                refl_res_hitdiff = refl_final_hit ? (refl_final_invz - (1.0f / refl_final_depth)) : refl_bis_thickb;
                refl_res_hit = true;
                break;
            }
            else
            {
                refl_res_t = refl_mres_tprev;
                refl_res_diff = zero_f;
                refl_res_thick = zero_f;
                refl_res_hitdiff = zero_f;
                refl_res_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool refl_cond_hit;
        if (refl_res_hit)
        {
            bool refl_cond_t;
            if (refl_res_hitdiff < refl_res_thick)
            {
                refl_cond_t = refl_res_diff < refl_res_thick;
            }
            else
            {
                refl_cond_t = false;
            }
            refl_cond_hit = refl_cond_t;
        }
        else
        {
            refl_cond_hit = false;
        }
        if (refl_cond_hit)
        {
            refl_hit = true;
            refl_t = refl_res_t;
            break;
        }
        refl_hit = false;
        refl_t = refl_mres_tprev;
        break;
    } while(false);
    float quad_t = QuadReadAcrossX(refl_t);
    float quad_hit = QuadReadAcrossX(float(refl_hit));
    bool quad_flag = (int(quad_hit != 0.0f) & int(!refl_hit)) != 0;
    bool quad_hit2 = quad_flag ? true : refl_hit;
    float quad_t2 = quad_flag ? quad_t : refl_t;
    float quad_t3 = QuadReadAcrossY(quad_t2);
    float quad_hit3 = QuadReadAcrossY(float(quad_hit2));
    bool quad_flag2 = (int(quad_hit3 != 0.0f) & int(!quad_hit2)) != 0;
    float quad_t4 = quad_flag2 ? quad_t3 : quad_t2;
    // ---- TAA 历史颜色:用重投影纹理在 taa_history_tex 中取上一帧颜色 ----
    // 命中处(反射被水面遮挡)复用历史,否则清 0;非命中处写入水 RT ray。
    float3 history_col;
    if (quad_flag2 ? true : quad_hit2)
    {
        float2 history_uv = (lerp(refl_p0, refl_p1, quad_t4.xxx).xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        bool history_ok;
        if (history_valid != 0u)
        {
            history_ok = !(0u == frame_constants_frame_index);
        }
        else
        {
            history_ok = false;
        }
        float3 history_sample;
        if (history_ok)
        {
            history_sample = taa_history_tex.SampleLevel(sampler_lnc, reprojection_tex.SampleLevel(sampler_nnc, history_uv, 0.0f).xy + history_uv, 0.0f).xyz;
        }
        else
        {
            history_sample = 0.0f.xxx;
        }
        history_col = history_sample;
    }
    else
    {
        float3 rt_pos = lerp(refl_p0, refl_p1, quad_t4.xxx);
        float4 rt_world4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(rt_pos, 1.0f)));
        uint lane_mask;
        if (is_helper)
        {
            lane_mask = 0u;
        }
        else
        {
            uint2 lane_parity = pixel_uv & uint2(1u, 1u);
            lane_mask = 1u << ((lane_parity.x + (lane_parity.y * 2u)) & 31u);
        }
        uint lane_x = QuadReadAcrossX(lane_mask);
        uint lane_sum = lane_mask + lane_x;
        uint lane_y = QuadReadAcrossY(lane_sum);
        uint lane_total = lane_sum + lane_y;
        uint2 lane_sel = pixel_uv & uint2(1u, 1u);
        if ((lane_sel.x + (lane_sel.y * 2u)) == firstbithigh(lane_total))
        {
            uint rt_index;
            __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, rt_index);
            float3 cube_dir = water_normal / ((abs(water_normal.x) + abs(water_normal.y)) + abs(water_normal.z)).xxx;
            float3 cube_dir2;
            if (cube_dir.z < 0.0f)
            {
                float2 cube_xy = (1.0f.xx - abs(cube_dir.yx)) * ((step(0.0f.xx, cube_dir.xy) * 2.0f) - 1.0f.xx);
                cube_dir2 = float3(cube_xy.x, cube_xy.y, cube_dir.z);
            }
            else
            {
                cube_dir2 = cube_dir;
            }
            float2 cube_uv = (cube_dir2.xy * 0.5f) + 0.5f.xx;
            __alias_water_rt_ray_buffer.Store<uint4>(rt_index * 16 + 0, uint4(half_px.x + (half_px.y * 65536u), spvPackHalf2x16(float2(cube_uv.x, 0.0f)) | (spvPackHalf2x16(float2(cube_uv.y, 0.0f)) << 16u), asuint(dot(refl_dir2, (rt_world4.xyz / rt_world4.w.xxx) - (world_ray.xyz / world_ray.w.xxx))), (asuint(gl_FragCoord.z) & 4294967280u) + lane_total));
        }
        history_col = 0.0f.xxx;
    }
    // ---- GGX 镜面高光(面向判定 + 微面反射) ----
    float ndotv;
    float blur_sq = dof_blur * dof_blur;
    float3 specular;
    do
    {
        ndotv = dot(water_normal, up_vec);
        bool face_ok;
        if (!(ndotv <= 0.0f))
        {
            face_ok = dot(water_normal, to_camera) <= 0.0f;
        }
        else
        {
            face_ok = true;
        }
        if (face_ok)
        {
            specular = 0.0f.xxx;
            break;
        }
        float rough_sq = blur_sq * blur_sq;
        float3 h_vec = normalize(to_camera + up_vec);
        float ndoth = dot(water_normal, h_vec);
        float vdotn1 = max(0.0f, 1.0f - dot(h_vec, up_vec));
        float vdotn2 = vdotn1 * vdotn1;
        float ndotl2 = dot(water_normal, to_camera);
        float spec_den = ((ndoth * ndoth) * (rough_sq - 1.0f)) + 1.0f;
        specular = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((vdotn2 * vdotn2) * vdotn1).xxx) * (((2.0f * ndotv) * ndotl2) / ((ndotv * sqrt(((((-ndotl2) * rough_sq) + ndotl2) * ndotl2) + rough_sq)) + (ndotl2 * sqrt(((((-ndotv) * rough_sq) + ndotv) * ndotv) + rough_sq))))) * (rough_sq / ((3.1415927410125732421875f * spec_den) * spec_den))) / ((4.0f * ndotl2) * ndotv).xxx;
        break;
    } while(false);
    // ---- 第二层水面法线:在高程坐标系下重新叠加噪声八度与涟漪源 ----
    float2 water_ts = (water_world - ((up_vec * water_y) / up4.y.xxx)).xz;
    uint carry_f1;
    uint carry_f2;
    bool carry_f3;
    bool carry_f4;
    uint carry_f5;
    uint carry_f6;
    bool carry_f7;
    bool carry_f8;
    float3 normal_off2;
    do
    {
        if (is_theme4)
        {
            carry_f1 = carry_m1;
            carry_f2 = carry_m2;
            carry_f3 = carry_m3;
            carry_f4 = carry_m4;
            carry_f5 = carry_m5;
            carry_f6 = carry_m6;
            carry_f7 = carry_m7;
            carry_f8 = carry_m8;
            normal_off2 = 0.0f.xxx;
            break;
        }
        float game_time2 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint carry2_a;
        uint carry2_b;
        bool carry2_c;
        bool carry2_d;
        uint carry2_e;
        uint carry2_f;
        bool carry2_g;
        bool carry2_h;
        carry2_a = carry_m1;
        carry2_b = carry_m2;
        carry2_c = carry_m3;
        carry2_d = carry_m4;
        carry2_e = carry_m5;
        carry2_f = carry_m6;
        carry2_g = carry_m7;
        carry2_h = carry_m8;
        float2 noise_offsets2[4];
        for (uint i_noise2 = 0u; i_noise2 < 4u; )
        {
            float seed_f2 = float(noise_seeds[i_noise2]);
            int seed_int2 = int(floor(seed_f2));
            float seed_frac2 = frac(seed_f2);
            uint seed_u2 = uint(seed_int2);
            uint seed_u2a = seed_u2 + (seed_u2 << 10u);
            uint seed_u2b = seed_u2a ^ (seed_u2a >> 6u);
            uint seed_u2c = seed_u2b + (seed_u2b << 3u);
            uint seed_u2d = seed_u2c ^ (seed_u2c >> 11u);
            bool sel_a2 = carry2_h ? carry2_h : true;
            uint sel_b2 = carry2_h ? carry2_f : 1664525u;
            bool sel_c2 = carry2_g ? carry2_g : true;
            uint sel_d2 = carry2_g ? carry2_e : 1013904223u;
            uint hash0_2 = (((seed_u2 * sel_b2) + (seed_u2d + (seed_u2d << 15u))) + sel_d2) * sel_b2;
            uint hash0_2b = hash0_2 ^ (hash0_2 >> 11u);
            uint hash0_2c = hash0_2b ^ ((hash0_2b << 7u) & 2636928640u);
            uint hash0_2d = hash0_2c ^ ((hash0_2c << 15u) & 4022730752u);
            uint hash0_2e = hash0_2d ^ (hash0_2d >> 18u);
            uint hash0_2f = hash0_2e + (hash0_2e << 10u);
            uint hash0_2g = hash0_2f ^ (hash0_2f >> 6u);
            uint hash0_2h = hash0_2g + (hash0_2g << 3u);
            uint hash0_2i = hash0_2h ^ (hash0_2h >> 11u);
            bool sel_e2 = carry2_d ? carry2_d : true;
            uint sel_f2 = carry2_d ? carry2_b : 8388607u;
            bool sel_g2 = carry2_c ? carry2_c : true;
            uint sel_h2 = carry2_c ? carry2_a : 1065353216u;
            bool sel_e22 = sel_e2 ? sel_e2 : true;
            uint sel_f22 = sel_e2 ? sel_f2 : 8388607u;
            bool sel_g22 = sel_g2 ? sel_g2 : true;
            uint sel_h22 = sel_g2 ? sel_h2 : 1065353216u;
            uint seed_x12 = uint(seed_int2 + 1);
            uint seed_x1a2 = seed_x12 + (seed_x12 << 10u);
            uint seed_x1b2 = seed_x1a2 ^ (seed_x1a2 >> 6u);
            uint seed_x1c2 = seed_x1b2 + (seed_x1b2 << 3u);
            uint seed_x1d2 = seed_x1c2 ^ (seed_x1c2 >> 11u);
            uint carry2_f2 = sel_a2 ? sel_b2 : 1664525u;
            uint carry2_e2 = sel_c2 ? sel_d2 : 1013904223u;
            uint hash1_2 = (((seed_x12 * carry2_f2) + (seed_x1d2 + (seed_x1d2 << 15u))) + carry2_e2) * carry2_f2;
            uint hash1_2b = hash1_2 ^ (hash1_2 >> 11u);
            uint hash1_2c = hash1_2b ^ ((hash1_2b << 7u) & 2636928640u);
            uint hash1_2d = hash1_2c ^ ((hash1_2c << 15u) & 4022730752u);
            uint hash1_2e = hash1_2d ^ (hash1_2d >> 18u);
            uint hash1_2f = hash1_2e + (hash1_2e << 10u);
            uint hash1_2g = hash1_2f ^ (hash1_2f >> 6u);
            uint hash1_2h = hash1_2g + (hash1_2g << 3u);
            uint hash1_2i = hash1_2h ^ (hash1_2h >> 11u);
            bool sel_i2 = sel_e22 ? sel_e22 : true;
            uint sel_j2 = sel_e22 ? sel_f22 : 8388607u;
            bool sel_k2 = sel_g22 ? sel_g22 : true;
            uint sel_l2 = sel_g22 ? sel_h22 : 1065353216u;
            uint carry2_b2 = sel_i2 ? sel_j2 : 8388607u;
            uint carry2_a2 = sel_k2 ? sel_l2 : 1065353216u;
            float smooth_f2 = (seed_frac2 * seed_frac2) * (3.0f - (2.0f * seed_frac2));
            noise_offsets2[i_noise2] = (normalize(((float2(asfloat((hash0_2e & sel_f2) | sel_h2) - 1.0f, asfloat(((hash0_2i + (hash0_2i << 15u)) & sel_f22) | sel_h22) - 1.0f) * (1.0f - smooth_f2)) + (float2(asfloat((hash1_2e & sel_j2) | sel_l2) - 1.0f, asfloat(((hash1_2i + (hash1_2i << 15u)) & carry2_b2) | carry2_a2) - 1.0f) * smooth_f2)) - 0.5f.xx) * game_time2) * 3.0f;
            i_noise2++;
            carry2_a = carry2_a2;
            carry2_b = carry2_b2;
            carry2_c = sel_k2 ? sel_k2 : true;
            carry2_d = sel_i2 ? sel_i2 : true;
            carry2_e = carry2_e2;
            carry2_f = carry2_f2;
            carry2_g = sel_c2 ? sel_c2 : true;
            carry2_h = sel_a2 ? sel_a2 : true;
            continue;
        }
        float3 normal_acc2;
        normal_acc2 = 0.0f.xxx;
        for (uint i_oct2 = 0u; i_oct2 < 4u; )
        {
            uint oct_next2 = i_oct2 + 1u;
            float oct_angle2 = float(oct_next2) * 10.16640758514404296875f;
            float oct_cos2 = cos(oct_angle2);
            float oct_sin2 = sin(oct_angle2);
            float3 oct_delta2;
            oct_delta2.x = normal_acc2.x + ((((water_noise_texture.Sample(sampler_llr, mul(float2x2(float2(oct_cos2, oct_sin2), float2(-oct_sin2, oct_cos2)), (water_ts * noise_freqs[i_oct2]) + noise_offsets2[i_oct2]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * noise_weights[i_oct2]) * float2(1.0f, noise_freqs[i_oct2] * 0.01200000010430812835693359375f).xyy).x;
            normal_acc2 = float3(oct_delta2.x, zero_f2v.x, zero_f2v.y);
            i_oct2 = oct_next2;
            continue;
        }
        carry_f1 = carry2_a;
        carry_f2 = carry2_b;
        carry_f3 = carry2_c;
        carry_f4 = carry2_d;
        carry_f5 = carry2_e;
        carry_f6 = carry2_f;
        carry_f7 = carry2_g;
        carry_f8 = carry2_h;
        normal_off2 = normal_acc2;
        break;
    } while(false);
    float3 ripple_total2;
    do
    {
        if (is_theme4)
        {
            ripple_total2 = 0.0f.xxx;
            break;
        }
        uint ripple_count2 = min(32u, ripple_source_buffer.Load<uint>(512));
        float3 ripple_acc2;
        ripple_acc2 = 0.0f.xxx;
        for (uint i_ripple2 = 0u; i_ripple2 < ripple_count2; )
        {
            float2 r_pos_a2 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple2 * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple2 * 16 + 0).x >> 16u) & 65535u).x);
            float2 r_pos_b2 = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple2 * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple2 * 16 + 0).y >> 16u) & 65535u).x);
            float2 r_off2 = lerp(r_pos_a2, r_pos_b2, 0.5f.xx) - water_ts;
            float r_dist2 = length(r_off2);
            float r_dist22 = r_dist2 + dot(r_off2, (r_pos_b2 - r_pos_a2) * 3.0f);
            float r_max2 = max(r_dist2, r_dist22);
            float r_max2sq = r_max2 * r_max2;
            ripple_acc2 += (float3(float3(sin((r_dist22 * 16.3649997711181640625f) - ((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f)) / (1.0f + (r_max2sq * r_max2sq)), zero_f3, zero_f3).x, zero_f2v.x, zero_f2v.y) * asfloat(ripple_source_buffer.Load<uint4>(i_ripple2 * 16 + 0).z));
            i_ripple2++;
            continue;
        }
        ripple_total2 = ripple_acc2;
        break;
    } while(false);
    // 水面亮斑强度:随到水面的距离衰减,叠加第二层法线扰动。
    float water_glint = 0.60000002384185791015625f + ((0.4000000059604644775390625f * (exp2((-0.4000000059604644775390625f) * water_dy) * smoothstep(0.0f, 1.0f, water_dy))) * (normal_off2 + ripple_total2).x);
    // ---- 岸线泡沫:主题 4 变体在水面上叠加 perlin 噪声泡沫 ----
    float3 foam;
    if (is_theme4)
    {
        float2 foam_p = world_pos.xz * 1.0f;
        int2 foam_cell = int2(floor(foam_p));
        float2 foam_f = frac(foam_p);
        uint2 foam_cell_u = uint2(foam_cell);
        uint foam_cell_y = foam_cell_u.y;
        uint foam_cy1 = foam_cell_y + (foam_cell_y << 10u);
        uint foam_cy2 = foam_cy1 ^ (foam_cy1 >> 6u);
        uint foam_cy3 = foam_cy2 + (foam_cy2 << 3u);
        uint foam_cy4 = foam_cy3 ^ (foam_cy3 >> 11u);
        bool sel_a3 = carry_f8 ? carry_f8 : true;
        uint sel_b3 = carry_f8 ? carry_f6 : 1664525u;
        bool sel_c3 = carry_f7 ? carry_f7 : true;
        uint sel_d3 = carry_f7 ? carry_f5 : 1013904223u;
        uint foam_hash0 = (((foam_cell_u.x * sel_b3) + (foam_cy4 + (foam_cy4 << 15u))) + sel_d3) * sel_b3;
        uint foam_hash0b = foam_hash0 ^ (foam_hash0 >> 11u);
        uint foam_hash0c = foam_hash0b ^ ((foam_hash0b << 7u) & 2636928640u);
        uint foam_hash0d = foam_hash0c ^ ((foam_hash0c << 15u) & 4022730752u);
        bool sel_e3 = carry_f4 ? carry_f4 : true;
        uint sel_f3 = carry_f4 ? carry_f2 : 8388607u;
        bool sel_g3 = carry_f3 ? carry_f3 : true;
        uint sel_h3 = carry_f3 ? carry_f1 : 1065353216u;
        uint2 foam_cell_x1 = uint2(foam_cell + int2(1, 0));
        uint foam_cell_x1y = foam_cell_x1.y;
        uint foam_cell_x1a = foam_cell_x1y + (foam_cell_x1y << 10u);
        uint foam_cell_x1b = foam_cell_x1a ^ (foam_cell_x1a >> 6u);
        uint foam_cell_x1c = foam_cell_x1b + (foam_cell_x1b << 3u);
        uint foam_cell_x1d = foam_cell_x1c ^ (foam_cell_x1c >> 11u);
        bool sel_a32 = sel_a3 ? sel_a3 : true;
        uint sel_b32 = sel_a3 ? sel_b3 : 1664525u;
        bool sel_c32 = sel_c3 ? sel_c3 : true;
        uint sel_d32 = sel_c3 ? sel_d3 : 1013904223u;
        uint foam_hash1 = (((foam_cell_x1.x * sel_b32) + (foam_cell_x1d + (foam_cell_x1d << 15u))) + sel_d32) * sel_b32;
        uint foam_hash1b = foam_hash1 ^ (foam_hash1 >> 11u);
        uint foam_hash1c = foam_hash1b ^ ((foam_hash1b << 7u) & 2636928640u);
        uint foam_hash1d = foam_hash1c ^ ((foam_hash1c << 15u) & 4022730752u);
        bool sel_e32 = sel_e3 ? sel_e3 : true;
        uint sel_f32 = sel_e3 ? sel_f3 : 8388607u;
        bool sel_g32 = sel_g3 ? sel_g3 : true;
        uint sel_h32 = sel_g3 ? sel_h3 : 1065353216u;
        uint2 foam_cell_y1 = uint2(foam_cell + int2(0, 1));
        uint foam_cell_y1y = foam_cell_y1.y;
        uint foam_cell_y1a = foam_cell_y1y + (foam_cell_y1y << 10u);
        uint foam_cell_y1b = foam_cell_y1a ^ (foam_cell_y1a >> 6u);
        uint foam_cell_y1c = foam_cell_y1b + (foam_cell_y1b << 3u);
        uint foam_cell_y1d = foam_cell_y1c ^ (foam_cell_y1c >> 11u);
        uint carry2_f3 = sel_a32 ? sel_b32 : 1664525u;
        uint carry2_e3 = sel_c32 ? sel_d32 : 1013904223u;
        uint foam_hash2 = (((foam_cell_y1.x * carry2_f3) + (foam_cell_y1d + (foam_cell_y1d << 15u))) + carry2_e3) * carry2_f3;
        uint foam_hash2b = foam_hash2 ^ (foam_hash2 >> 11u);
        uint foam_hash2c = foam_hash2b ^ ((foam_hash2b << 7u) & 2636928640u);
        uint foam_hash2d = foam_hash2c ^ ((foam_hash2c << 15u) & 4022730752u);
        uint sel_f33 = sel_e32 ? sel_f32 : 8388607u;
        uint sel_h33 = sel_g32 ? sel_h32 : 1065353216u;
        uint2 foam_cell_xy1 = uint2(foam_cell + int2(1, 1));
        uint foam_cell_xy1y = foam_cell_xy1.y;
        uint foam_cell_xy1a = foam_cell_xy1y + (foam_cell_xy1y << 10u);
        uint foam_cell_xy1b = foam_cell_xy1a ^ (foam_cell_xy1a >> 6u);
        uint foam_cell_xy1c = foam_cell_xy1b + (foam_cell_xy1b << 3u);
        uint foam_cell_xy1d = foam_cell_xy1c ^ (foam_cell_xy1c >> 11u);
        uint carry2_f33 = (sel_a32 ? sel_a32 : true) ? carry2_f3 : 1664525u;
        uint foam_hash3 = (((foam_cell_xy1.x * carry2_f33) + (foam_cell_xy1d + (foam_cell_xy1d << 15u))) + ((sel_c32 ? sel_c32 : true) ? carry2_e3 : 1013904223u)) * carry2_f33;
        uint foam_hash3b = foam_hash3 ^ (foam_hash3 >> 11u);
        uint foam_hash3c = foam_hash3b ^ ((foam_hash3b << 7u) & 2636928640u);
        uint foam_hash3d = foam_hash3c ^ ((foam_hash3c << 15u) & 4022730752u);
        float2 foam_u = (foam_f * foam_f) * (3.0f.xx - (foam_f * 2.0f));
        float foam_ux = foam_u.x;
        float foam_1mux = 1.0f - foam_ux;
        float foam_uy = foam_u.y;
        float foam_noise = (((1.0f - foam_uy) * (((asfloat(((foam_hash0d ^ (foam_hash0d >> 18u)) & sel_f3) | sel_h3) - 1.0f) * foam_1mux) + ((asfloat(((foam_hash1d ^ (foam_hash1d >> 18u)) & sel_f32) | sel_h32) - 1.0f) * foam_ux))) + (((asfloat(((foam_hash2d ^ (foam_hash2d >> 18u)) & sel_f33) | sel_h33) - 1.0f) * foam_1mux) * foam_uy)) + (((asfloat(((foam_hash3d ^ (foam_hash3d >> 18u)) & ((sel_e32 ? sel_e32 : true) ? sel_f33 : 8388607u)) | ((sel_g32 ? sel_g32 : true) ? sel_h33 : 1065353216u)) - 1.0f) * foam_ux) * foam_uy);
        foam = lerp(0.0f.xxx, 2.0f.xxx, (clamp((1.0f - smoothstep(0.699999988079071044921875f, 0.85000002384185791015625f, water_mask.SampleLevel(sampler_llc, (world_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y + ((foam_noise * foam_noise) * 0.20000000298023223876953125f))) * 0.20000000298023223876953125f, 0.0f, 1.0f) + 0.100000001490116119384765625f).xxx);
    }
    else
    {
        foam = 0.0f.xxx;
    }
    // ---- 最终合成 ----
    // 折射色(随水深衰减) + 反射(TAA 历史/镜面高光,按菲涅尔混合) + 岸线泡沫。
    out_var_SV_TARGET = float4(((((exp((-water_color) * ((water_dy + length(water_world - world_pos)) + 0.699999988079071044921875f)) * (0.959999978542327880859375f - fresnel)) * refraction_tex.SampleLevel(sampler_lnc, water_uv, 0.0f).xyz) * (1.0f + ((water_glint * water_glint) * water_glint))) + ((history_col + ((frame_constants_sun_color.xyz * min(30.0f, specular.x)) * ((1.0f - shadow_total.x) * clamp(ndotv, 0.0f, 1.0f)))) * (0.039999999105930328369140625f + fresnel))) + foam, 0.0f);
}

[earlydepthstencil]
SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _waterfall.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
};

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer instance_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 读入当前瀑布实例的网格数据(位置/切线/两层法线),再按顶点 id 展开网格 uv。
    WaterfallMesh mesh;
    mesh.position = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 0);
    mesh.tangent = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 16);
    mesh.normal0 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 32);
    mesh.normal1 = instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 64 + 48);
    float2 uv = float2(float(uint(gl_VertexIndex) % 2u), float(uint(gl_VertexIndex) / 2u)) * float2(1.0f, 0.100000001490116119384765625f);
    float uv_x = uv.x;
    float3 n = lerp(mesh.normal0.xyz, mesh.normal1.xyz, uv_x.xxx);
    float uv_y = uv.y;
    float3 pos = (mesh.position.xyz + (float3(mesh.tangent.xy, 0.0f).xzy * uv_x)) + (n * (uv_y * 0.5f));
    pos.y = lerp(mesh.position.w, mesh.position.y, 1.0f - pow(uv_y, 3.0f));
    // 把顶点坐标折半并随时间沿 +Y 平移,构造 3D 值噪声的采样网格(让瀑布看起来在流)。
    float3 scaled = (pos * float3(0.5f, 1.0f, 0.5f)) + (float3(0.0f, 3.0f, 0.0f) * (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int3 cell = int3(floor(scaled));
    float3 f = frac(scaled);
    uint3 cell_u = uint3(cell);
    uint h000_z0 = cell_u.z;
    uint h000_z1 = h000_z0 + (h000_z0 << 10u);
    uint h000_z2 = h000_z1 ^ (h000_z1 >> 6u);
    uint h000_z3 = h000_z2 + (h000_z2 << 3u);
    uint h000_z4 = h000_z3 ^ (h000_z3 >> 11u);
    uint h000_y0 = (((cell_u.y * 1664525u) + (h000_z4 + (h000_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h000_y1 = h000_y0 ^ (h000_y0 >> 11u);
    uint h000_y2 = h000_y1 ^ ((h000_y1 << 7u) & 2636928640u);
    uint h000_y3 = h000_y2 ^ ((h000_y2 << 15u) & 4022730752u);
    uint h000_x0 = (((cell_u.x * 1664525u) + (h000_y3 ^ (h000_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h000_x1 = h000_x0 ^ (h000_x0 >> 11u);
    uint h000_x2 = h000_x1 ^ ((h000_x1 << 7u) & 2636928640u);
    uint h000_x3 = h000_x2 ^ ((h000_x2 << 15u) & 4022730752u);
    uint seed000_x = h000_x3 ^ (h000_x3 >> 18u);
    uint h000_sx1 = seed000_x + (seed000_x << 10u);
    uint h000_sx2 = h000_sx1 ^ (h000_sx1 >> 6u);
    uint h000_sx3 = h000_sx2 + (h000_sx2 << 3u);
    uint h000_sx4 = h000_sx3 ^ (h000_sx3 >> 11u);
    uint seed000_y = h000_sx4 + (h000_sx4 << 15u);
    uint h000_sy1 = seed000_y + (seed000_y << 10u);
    uint h000_sy2 = h000_sy1 ^ (h000_sy1 >> 6u);
    uint h000_sy3 = h000_sy2 + (h000_sy2 << 3u);
    uint seed000_z = h000_sy3 ^ (h000_sy3 >> 11u);
    uint3 cell_x1 = uint3(cell + int3(1, 0, 0));
    uint h100_z0 = cell_x1.z;
    uint h100_z1 = h100_z0 + (h100_z0 << 10u);
    uint h100_z2 = h100_z1 ^ (h100_z1 >> 6u);
    uint h100_z3 = h100_z2 + (h100_z2 << 3u);
    uint h100_z4 = h100_z3 ^ (h100_z3 >> 11u);
    uint h100_y0 = (((cell_x1.y * 1664525u) + (h100_z4 + (h100_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h100_y1 = h100_y0 ^ (h100_y0 >> 11u);
    uint h100_y2 = h100_y1 ^ ((h100_y1 << 7u) & 2636928640u);
    uint h100_y3 = h100_y2 ^ ((h100_y2 << 15u) & 4022730752u);
    uint h100_x0 = (((cell_x1.x * 1664525u) + (h100_y3 ^ (h100_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h100_x1 = h100_x0 ^ (h100_x0 >> 11u);
    uint h100_x2 = h100_x1 ^ ((h100_x1 << 7u) & 2636928640u);
    uint h100_x3 = h100_x2 ^ ((h100_x2 << 15u) & 4022730752u);
    uint seed100_x = h100_x3 ^ (h100_x3 >> 18u);
    uint h100_sx1 = seed100_x + (seed100_x << 10u);
    uint h100_sx2 = h100_sx1 ^ (h100_sx1 >> 6u);
    uint h100_sx3 = h100_sx2 + (h100_sx2 << 3u);
    uint h100_sx4 = h100_sx3 ^ (h100_sx3 >> 11u);
    uint seed100_y = h100_sx4 + (h100_sx4 << 15u);
    uint h100_sy1 = seed100_y + (seed100_y << 10u);
    uint h100_sy2 = h100_sy1 ^ (h100_sy1 >> 6u);
    uint h100_sy3 = h100_sy2 + (h100_sy2 << 3u);
    uint seed100_z = h100_sy3 ^ (h100_sy3 >> 11u);
    uint3 cell_y1 = uint3(cell + int3(0, 1, 0));
    uint h010_z0 = cell_y1.z;
    uint h010_z1 = h010_z0 + (h010_z0 << 10u);
    uint h010_z2 = h010_z1 ^ (h010_z1 >> 6u);
    uint h010_z3 = h010_z2 + (h010_z2 << 3u);
    uint h010_z4 = h010_z3 ^ (h010_z3 >> 11u);
    uint h010_y0 = (((cell_y1.y * 1664525u) + (h010_z4 + (h010_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h010_y1 = h010_y0 ^ (h010_y0 >> 11u);
    uint h010_y2 = h010_y1 ^ ((h010_y1 << 7u) & 2636928640u);
    uint h010_y3 = h010_y2 ^ ((h010_y2 << 15u) & 4022730752u);
    uint h010_x0 = (((cell_y1.x * 1664525u) + (h010_y3 ^ (h010_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h010_x1 = h010_x0 ^ (h010_x0 >> 11u);
    uint h010_x2 = h010_x1 ^ ((h010_x1 << 7u) & 2636928640u);
    uint h010_x3 = h010_x2 ^ ((h010_x2 << 15u) & 4022730752u);
    uint seed010_x = h010_x3 ^ (h010_x3 >> 18u);
    uint h010_sx1 = seed010_x + (seed010_x << 10u);
    uint h010_sx2 = h010_sx1 ^ (h010_sx1 >> 6u);
    uint h010_sx3 = h010_sx2 + (h010_sx2 << 3u);
    uint h010_sx4 = h010_sx3 ^ (h010_sx3 >> 11u);
    uint seed010_y = h010_sx4 + (h010_sx4 << 15u);
    uint h010_sy1 = seed010_y + (seed010_y << 10u);
    uint h010_sy2 = h010_sy1 ^ (h010_sy1 >> 6u);
    uint h010_sy3 = h010_sy2 + (h010_sy2 << 3u);
    uint seed010_z = h010_sy3 ^ (h010_sy3 >> 11u);
    uint3 cell_xy1 = uint3(cell + int3(1, 1, 0));
    uint h110_z0 = cell_xy1.z;
    uint h110_z1 = h110_z0 + (h110_z0 << 10u);
    uint h110_z2 = h110_z1 ^ (h110_z1 >> 6u);
    uint h110_z3 = h110_z2 + (h110_z2 << 3u);
    uint h110_z4 = h110_z3 ^ (h110_z3 >> 11u);
    uint h110_y0 = (((cell_xy1.y * 1664525u) + (h110_z4 + (h110_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h110_y1 = h110_y0 ^ (h110_y0 >> 11u);
    uint h110_y2 = h110_y1 ^ ((h110_y1 << 7u) & 2636928640u);
    uint h110_y3 = h110_y2 ^ ((h110_y2 << 15u) & 4022730752u);
    uint h110_x0 = (((cell_xy1.x * 1664525u) + (h110_y3 ^ (h110_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h110_x1 = h110_x0 ^ (h110_x0 >> 11u);
    uint h110_x2 = h110_x1 ^ ((h110_x1 << 7u) & 2636928640u);
    uint h110_x3 = h110_x2 ^ ((h110_x2 << 15u) & 4022730752u);
    uint seed110_x = h110_x3 ^ (h110_x3 >> 18u);
    uint h110_sx1 = seed110_x + (seed110_x << 10u);
    uint h110_sx2 = h110_sx1 ^ (h110_sx1 >> 6u);
    uint h110_sx3 = h110_sx2 + (h110_sx2 << 3u);
    uint h110_sx4 = h110_sx3 ^ (h110_sx3 >> 11u);
    uint seed110_y = h110_sx4 + (h110_sx4 << 15u);
    uint h110_sy1 = seed110_y + (seed110_y << 10u);
    uint h110_sy2 = h110_sy1 ^ (h110_sy1 >> 6u);
    uint h110_sy3 = h110_sy2 + (h110_sy2 << 3u);
    uint seed110_z = h110_sy3 ^ (h110_sy3 >> 11u);
    uint3 cell_z1 = uint3(cell + int3(0, 0, 1));
    uint h001_z0 = cell_z1.z;
    uint h001_z1 = h001_z0 + (h001_z0 << 10u);
    uint h001_z2 = h001_z1 ^ (h001_z1 >> 6u);
    uint h001_z3 = h001_z2 + (h001_z2 << 3u);
    uint h001_z4 = h001_z3 ^ (h001_z3 >> 11u);
    uint h001_y0 = (((cell_z1.y * 1664525u) + (h001_z4 + (h001_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h001_y1 = h001_y0 ^ (h001_y0 >> 11u);
    uint h001_y2 = h001_y1 ^ ((h001_y1 << 7u) & 2636928640u);
    uint h001_y3 = h001_y2 ^ ((h001_y2 << 15u) & 4022730752u);
    uint h001_x0 = (((cell_z1.x * 1664525u) + (h001_y3 ^ (h001_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h001_x1 = h001_x0 ^ (h001_x0 >> 11u);
    uint h001_x2 = h001_x1 ^ ((h001_x1 << 7u) & 2636928640u);
    uint h001_x3 = h001_x2 ^ ((h001_x2 << 15u) & 4022730752u);
    uint seed001_x = h001_x3 ^ (h001_x3 >> 18u);
    uint h001_sx1 = seed001_x + (seed001_x << 10u);
    uint h001_sx2 = h001_sx1 ^ (h001_sx1 >> 6u);
    uint h001_sx3 = h001_sx2 + (h001_sx2 << 3u);
    uint h001_sx4 = h001_sx3 ^ (h001_sx3 >> 11u);
    uint seed001_y = h001_sx4 + (h001_sx4 << 15u);
    uint h001_sy1 = seed001_y + (seed001_y << 10u);
    uint h001_sy2 = h001_sy1 ^ (h001_sy1 >> 6u);
    uint h001_sy3 = h001_sy2 + (h001_sy2 << 3u);
    uint seed001_z = h001_sy3 ^ (h001_sy3 >> 11u);
    uint3 cell_xz1 = uint3(cell + int3(1, 0, 1));
    uint h101_z0 = cell_xz1.z;
    uint h101_z1 = h101_z0 + (h101_z0 << 10u);
    uint h101_z2 = h101_z1 ^ (h101_z1 >> 6u);
    uint h101_z3 = h101_z2 + (h101_z2 << 3u);
    uint h101_z4 = h101_z3 ^ (h101_z3 >> 11u);
    uint h101_y0 = (((cell_xz1.y * 1664525u) + (h101_z4 + (h101_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h101_y1 = h101_y0 ^ (h101_y0 >> 11u);
    uint h101_y2 = h101_y1 ^ ((h101_y1 << 7u) & 2636928640u);
    uint h101_y3 = h101_y2 ^ ((h101_y2 << 15u) & 4022730752u);
    uint h101_x0 = (((cell_xz1.x * 1664525u) + (h101_y3 ^ (h101_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h101_x1 = h101_x0 ^ (h101_x0 >> 11u);
    uint h101_x2 = h101_x1 ^ ((h101_x1 << 7u) & 2636928640u);
    uint h101_x3 = h101_x2 ^ ((h101_x2 << 15u) & 4022730752u);
    uint seed101_x = h101_x3 ^ (h101_x3 >> 18u);
    uint h101_sx1 = seed101_x + (seed101_x << 10u);
    uint h101_sx2 = h101_sx1 ^ (h101_sx1 >> 6u);
    uint h101_sx3 = h101_sx2 + (h101_sx2 << 3u);
    uint h101_sx4 = h101_sx3 ^ (h101_sx3 >> 11u);
    uint seed101_y = h101_sx4 + (h101_sx4 << 15u);
    uint h101_sy1 = seed101_y + (seed101_y << 10u);
    uint h101_sy2 = h101_sy1 ^ (h101_sy1 >> 6u);
    uint h101_sy3 = h101_sy2 + (h101_sy2 << 3u);
    uint seed101_z = h101_sy3 ^ (h101_sy3 >> 11u);
    uint3 cell_yz1 = uint3(cell + int3(0, 1, 1));
    uint h011_z0 = cell_yz1.z;
    uint h011_z1 = h011_z0 + (h011_z0 << 10u);
    uint h011_z2 = h011_z1 ^ (h011_z1 >> 6u);
    uint h011_z3 = h011_z2 + (h011_z2 << 3u);
    uint h011_z4 = h011_z3 ^ (h011_z3 >> 11u);
    uint h011_y0 = (((cell_yz1.y * 1664525u) + (h011_z4 + (h011_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h011_y1 = h011_y0 ^ (h011_y0 >> 11u);
    uint h011_y2 = h011_y1 ^ ((h011_y1 << 7u) & 2636928640u);
    uint h011_y3 = h011_y2 ^ ((h011_y2 << 15u) & 4022730752u);
    uint h011_x0 = (((cell_yz1.x * 1664525u) + (h011_y3 ^ (h011_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h011_x1 = h011_x0 ^ (h011_x0 >> 11u);
    uint h011_x2 = h011_x1 ^ ((h011_x1 << 7u) & 2636928640u);
    uint h011_x3 = h011_x2 ^ ((h011_x2 << 15u) & 4022730752u);
    uint seed011_x = h011_x3 ^ (h011_x3 >> 18u);
    uint h011_sx1 = seed011_x + (seed011_x << 10u);
    uint h011_sx2 = h011_sx1 ^ (h011_sx1 >> 6u);
    uint h011_sx3 = h011_sx2 + (h011_sx2 << 3u);
    uint h011_sx4 = h011_sx3 ^ (h011_sx3 >> 11u);
    uint seed011_y = h011_sx4 + (h011_sx4 << 15u);
    uint h011_sy1 = seed011_y + (seed011_y << 10u);
    uint h011_sy2 = h011_sy1 ^ (h011_sy1 >> 6u);
    uint h011_sy3 = h011_sy2 + (h011_sy2 << 3u);
    uint seed011_z = h011_sy3 ^ (h011_sy3 >> 11u);
    uint3 cell_xyz1 = uint3(cell + int3(1, 1, 1));
    uint h111_z0 = cell_xyz1.z;
    uint h111_z1 = h111_z0 + (h111_z0 << 10u);
    uint h111_z2 = h111_z1 ^ (h111_z1 >> 6u);
    uint h111_z3 = h111_z2 + (h111_z2 << 3u);
    uint h111_z4 = h111_z3 ^ (h111_z3 >> 11u);
    uint h111_y0 = (((cell_xyz1.y * 1664525u) + (h111_z4 + (h111_z4 << 15u))) + 1013904223u) * 1664525u;
    uint h111_y1 = h111_y0 ^ (h111_y0 >> 11u);
    uint h111_y2 = h111_y1 ^ ((h111_y1 << 7u) & 2636928640u);
    uint h111_y3 = h111_y2 ^ ((h111_y2 << 15u) & 4022730752u);
    uint h111_x0 = (((cell_xyz1.x * 1664525u) + (h111_y3 ^ (h111_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint h111_x1 = h111_x0 ^ (h111_x0 >> 11u);
    uint h111_x2 = h111_x1 ^ ((h111_x1 << 7u) & 2636928640u);
    uint h111_x3 = h111_x2 ^ ((h111_x2 << 15u) & 4022730752u);
    uint seed111_x = h111_x3 ^ (h111_x3 >> 18u);
    uint h111_sx1 = seed111_x + (seed111_x << 10u);
    uint h111_sx2 = h111_sx1 ^ (h111_sx1 >> 6u);
    uint h111_sx3 = h111_sx2 + (h111_sx2 << 3u);
    uint h111_sx4 = h111_sx3 ^ (h111_sx3 >> 11u);
    uint seed111_y = h111_sx4 + (h111_sx4 << 15u);
    uint h111_sy1 = seed111_y + (seed111_y << 10u);
    uint h111_sy2 = h111_sy1 ^ (h111_sy1 >> 6u);
    uint h111_sy3 = h111_sy2 + (h111_sy2 << 3u);
    uint seed111_z = h111_sy3 ^ (h111_sy3 >> 11u);
        // ---- 3D perlin 位移 ----
        // 对 8 个格角的整型哈希做三线性插值;每角哈希链为:格坐标乘 1664525 混入
        // z/y/x,再走两级雪崩(>>11、<<7&2636928640、<<15&4022730752),最后取低 23 位
        // 映射到 [-1,1] 作为噪声值。
    float3 smooth = (f * f) * (3.0f.xxx - (f * 2.0f));
    float smooth_x = smooth.x;
    float smooth_1mx = 1.0f - smooth_x;
    float smooth_y = smooth.y;
    float smooth_1my = 1.0f - smooth_y;
    float smooth_z = smooth.z;
    float smooth_1mz = 1.0f - smooth_z;
    float3 noise_a = (((((float3(asfloat((seed000_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed000_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed000_z + (seed000_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_1my) * smooth_1mz) + (((float3(asfloat((seed100_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed100_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed100_z + (seed100_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_1my) * smooth_1mz)) + (((float3(asfloat((seed010_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed010_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed010_z + (seed010_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_y) * smooth_1mz)) + (((float3(asfloat((seed110_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed110_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed110_z + (seed110_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y) * smooth_1mz);
    float3 noise_b = (((noise_a + (((float3(asfloat((seed001_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed001_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed001_z + (seed001_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_1my) * smooth_z)) + (((float3(asfloat((seed101_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed101_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed101_z + (seed101_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_1my) * smooth_z)) + (((float3(asfloat((seed011_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed011_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed011_z + (seed011_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_1mx) * smooth_y) * smooth_z)) + (((float3(asfloat((seed111_x & 8388607u) | 1065353216u) - 1.0f, asfloat((seed111_y & 8388607u) | 1065353216u) - 1.0f, asfloat(((seed111_z + (seed111_z << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y) * smooth_z);
    // 把噪声位移(只取 xz)按高度 v 缩放加到顶点位置,完成瀑布的流动形变。
    float3 final_pos = pos + ((((noise_b - 0.5f.xxx) * float3(1.0f, 0.0f, 1.0f)) * uv_y) * 0.20000000298023223876953125f);
    // 世界 -> 采样空间;C0 为合成法线(网格法线 * 切向斜率项 + 少量 y 分量)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(final_pos, 1.0f));
    out_var_C0 = normalize((n * (-((mesh.position.y - mesh.position.w) * ((-3.0f) * pow(uv_y, 2.0f))))) + float3(0.0f, 0.5f, 0.0f));
    out_var_C1 = final_pos;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _waterfall.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct RippleSourceSsbo
{
    uint4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
};

static uint zero_u;
static float zero_f;

static const uint2 quarter_offsets[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
static const float noise_freqs[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float noise_weights[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint noise_seeds[4] = { 17u, 18u, 20u, 21u };

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
    uint history_valid : packoffset(c0.y);
    uint2 terrain_lighting_texture_dims : packoffset(c0.z);
    uint2 blue_noise_texture_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 brdf_fg_lut_dims : packoffset(c2.z);
    uint rt_bvh_node_count : packoffset(c3);
    uint rt_triangle_count : packoffset(c3.y);
    uint2 water_noise_texture_dims : packoffset(c3.z);
    uint2 framebuffer_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 half_depth_tex_dims : packoffset(c5);
    uint2 refraction_tex_dims : packoffset(c5.z);
    uint2 taa_history_tex_dims : packoffset(c6);
    uint2 reprojection_tex_dims : packoffset(c6.z);
    float3 water_color : packoffset(c7);
};

ByteAddressBuffer ripple_source_buffer : register(t26, space0);
RWByteAddressBuffer __alias_water_rt_count_buffer : register(u34, space0);
RWByteAddressBuffer __alias_water_rt_ray_buffer : register(u35, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> water_noise_texture : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

// 打包两个 half 成 uint(half2x16)。
uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

// 解包 uint 成两个 half。
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void frag_main()
{
    // 精简变体(f0adff):不写颜色输出,只做水面法线/反射 marching/RT ray 写入。
    bool is_helper = IsHelperLane();
    uint2 pixel_uv = uint2(gl_FragCoord.xy);
    float3 water_normal;
    do
    {
        bool is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            water_normal = float3(0.0f, 1.0f, 0.0f);
            break;
        }
        float3 normal_off;
        do
        {
            if (is_theme4)
            {
                normal_off = 0.0f.xxx;
                break;
            }
            float game_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
            float2 noise_offsets[4];
            uint i_noise = 0u;
            uint carry_a = 0u;
            uint carry_b = 0u;
            bool carry_c = false;
            bool carry_d = false;
            uint carry_e = 0u;
            uint carry_f = 0u;
            bool carry_g = false;
            bool carry_h = false;
            for (; i_noise < 4u; )
            {
                float seed_f = float(noise_seeds[i_noise]);
                int seed_int = int(floor(seed_f));
                float seed_frac = frac(seed_f);
                uint seed_u = uint(seed_int);
                uint seed_u1 = seed_u + (seed_u << 10u);
                uint seed_u2 = seed_u1 ^ (seed_u1 >> 6u);
                uint seed_u3 = seed_u2 + (seed_u2 << 3u);
                uint seed_u4 = seed_u3 ^ (seed_u3 >> 11u);
                bool sel_a = carry_h ? carry_h : true;
                uint sel_b = carry_h ? carry_f : 1664525u;
                bool sel_c = carry_g ? carry_g : true;
                uint sel_d = carry_g ? carry_e : 1013904223u;
                uint hash0 = (((seed_u * sel_b) + (seed_u4 + (seed_u4 << 15u))) + sel_d) * sel_b;
                uint hash0b = hash0 ^ (hash0 >> 11u);
                uint hash0c = hash0b ^ ((hash0b << 7u) & 2636928640u);
                uint hash0d = hash0c ^ ((hash0c << 15u) & 4022730752u);
                uint hash0e = hash0d ^ (hash0d >> 18u);
                uint hash0f = hash0e + (hash0e << 10u);
                uint hash0g = hash0f ^ (hash0f >> 6u);
                uint hash0h = hash0g + (hash0g << 3u);
                uint hash0i = hash0h ^ (hash0h >> 11u);
                bool sel_e = carry_d ? carry_d : true;
                uint sel_f = carry_d ? carry_b : 8388607u;
                bool sel_g = carry_c ? carry_c : true;
                uint sel_h = carry_c ? carry_a : 1065353216u;
                bool sel_e2 = sel_e ? sel_e : true;
                uint sel_f2 = sel_e ? sel_f : 8388607u;
                bool sel_g2 = sel_g ? sel_g : true;
                uint sel_h2 = sel_g ? sel_h : 1065353216u;
                uint seed_x1 = uint(seed_int + 1);
                uint seed_x1a = seed_x1 + (seed_x1 << 10u);
                uint seed_x1b = seed_x1a ^ (seed_x1a >> 6u);
                uint seed_x1c = seed_x1b + (seed_x1b << 3u);
                uint seed_x1d = seed_x1c ^ (seed_x1c >> 11u);
                uint carry_f2 = sel_a ? sel_b : 1664525u;
                uint carry_e2 = sel_c ? sel_d : 1013904223u;
                uint hash1 = (((seed_x1 * carry_f2) + (seed_x1d + (seed_x1d << 15u))) + carry_e2) * carry_f2;
                uint hash1b = hash1 ^ (hash1 >> 11u);
                uint hash1c = hash1b ^ ((hash1b << 7u) & 2636928640u);
                uint hash1d = hash1c ^ ((hash1c << 15u) & 4022730752u);
                uint hash1e = hash1d ^ (hash1d >> 18u);
                uint hash1f = hash1e + (hash1e << 10u);
                uint hash1g = hash1f ^ (hash1f >> 6u);
                uint hash1h = hash1g + (hash1g << 3u);
                uint hash1i = hash1h ^ (hash1h >> 11u);
                bool sel_i = sel_e2 ? sel_e2 : true;
                uint sel_j = sel_e2 ? sel_f2 : 8388607u;
                bool sel_k = sel_g2 ? sel_g2 : true;
                uint sel_l = sel_g2 ? sel_h2 : 1065353216u;
                uint carry_g2 = sel_i ? sel_j : 8388607u;
                uint carry_h2 = sel_k ? sel_l : 1065353216u;
                float smooth_f = (seed_frac * seed_frac) * (3.0f - (2.0f * seed_frac));
                noise_offsets[i_noise] = (normalize(((float2(asfloat((hash0e & sel_f) | sel_h) - 1.0f, asfloat(((hash0i + (hash0i << 15u)) & sel_f2) | sel_h2) - 1.0f) * (1.0f - smooth_f)) + (float2(asfloat((hash1e & sel_j) | sel_l) - 1.0f, asfloat(((hash1i + (hash1i << 15u)) & carry_g2) | carry_h2) - 1.0f) * smooth_f)) - 0.5f.xx) * game_time) * 3.0f;
                i_noise++;
                carry_a = carry_h2;
                carry_b = carry_g2;
                carry_c = sel_k ? sel_k : true;
                carry_d = sel_i ? sel_i : true;
                carry_e = carry_e2;
                carry_f = carry_f2;
                carry_g = sel_c ? sel_c : true;
                carry_h = sel_a ? sel_a : true;
                continue;
            }
            float3 normal_acc;
            normal_acc = 0.0f.xxx;
            for (uint i_oct = 0u; i_oct < 4u; )
            {
                uint oct_next = i_oct + 1u;
                float oct_angle = float(oct_next) * 10.16640758514404296875f;
                float oct_cos = cos(oct_angle);
                float oct_sin = sin(oct_angle);
                float2x2 rot_mat = float2x2(float2(oct_cos, oct_sin), float2(-oct_sin, oct_cos));
                float2 oct_delta = normal_acc.yz + mul(((((water_noise_texture.Sample(sampler_llr, mul(rot_mat, (in_var_C1.xz * noise_freqs[i_oct]) + noise_offsets[i_oct]) * 0.01200000010430812835693359375f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * noise_weights[i_oct]) * float2(1.0f, noise_freqs[i_oct] * 0.01200000010430812835693359375f).xyy).yz, rot_mat);
                normal_acc = float3(normal_acc.x, oct_delta.x, oct_delta.y);
                i_oct = oct_next;
                continue;
            }
            normal_off = normal_acc;
            break;
        } while(false);
        float3 ripple_total;
        do
        {
            if (is_theme4)
            {
                ripple_total = 0.0f.xxx;
                break;
            }
            uint ripple_count = min(32u, ripple_source_buffer.Load<uint>(512));
            float3 ripple_acc;
            ripple_acc = 0.0f.xxx;
            for (uint i_ripple = 0u; i_ripple < ripple_count; )
            {
                float2 r_pos_a = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).x & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).x >> 16u) & 65535u).x);
                float2 r_pos_b = float2(spvUnpackHalf2x16(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).y & 65535u).x, spvUnpackHalf2x16((ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).y >> 16u) & 65535u).x);
                float2 r_center = lerp(r_pos_a, r_pos_b, 0.5f.xx);
                float2 r_off = r_center - in_var_C1.xz;
                float r_dist = length(r_off);
                float2 r_vel = (r_pos_b - r_pos_a) * 3.0f;
                float r_dist2 = r_dist + dot(r_off, r_vel);
                float r_time = (float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 6.0f;
                float r_max1 = max(r_dist, r_dist2);
                float r_max1sq = r_max1 * r_max1;
                float r_wave = sin((r_dist2 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max1sq * r_max1sq));
                float2 r_off2 = r_center - (in_var_C1.xz - float2(0.00999999977648258209228515625f, 0.0f));
                float r_dist3 = length(r_off2);
                float r_dist4 = r_dist3 + dot(r_off2, r_vel);
                float r_max2 = max(r_dist3, r_dist4);
                float r_max2sq = r_max2 * r_max2;
                float2 r_off3 = r_center - (in_var_C1.xz - float2(0.0f, 0.00999999977648258209228515625f));
                float r_dist5 = length(r_off3);
                float r_dist6 = r_dist5 + dot(r_off3, r_vel);
                float r_max3 = max(r_dist5, r_dist6);
                float r_max3sq = r_max3 * r_max3;
                float3 r_waves = float3(r_wave, (sin((r_dist4 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max2sq * r_max2sq))) - r_wave, (sin((r_dist6 * 16.3649997711181640625f) - r_time) / (1.0f + (r_max3sq * r_max3sq))) - r_wave);
                float2 r_waves_half = r_waves.yz * 0.5f;
                ripple_acc += (float3(r_waves.x, r_waves_half.x, r_waves_half.y) * asfloat(ripple_source_buffer.Load<uint4>(i_ripple * 16 + 0).z));
                i_ripple++;
                continue;
            }
            ripple_total = ripple_acc;
            break;
        } while(false);
        float3 normal_ts = float3(normal_off.y, 1.0f, normal_off.z);
        float2 normal_ts2 = normal_ts.xz + (float3(ripple_total.y, 1.0f, ripple_total.z).xz / abs(1.0f).xx);
        water_normal = normalize(float3(normal_ts2.x, normal_ts.y, normal_ts2.y));
        break;
    } while(false);
    float2 ndc_uv = (((float2(int2((pixel_uv + uint2(1u, 1u)) - quarter_offsets[frame_constants_frame_index & 3u])) + 0.5f.xx) / float2(framebuffer_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc_uv.x;
    float4 clip_ray = float4(ndc_x, ndc_uv.y, gl_FragCoord.z, 1.0f);
    float4 world_ray = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, clip_ray));
    float frame_angle = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float g_noise = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_uv.x) + frame_angle)) + (0.005837149918079376220703125f * (float(pixel_uv.y) + frame_angle))));
    float3 refl_dir = -reflect(-normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc_uv.y, 0.0f, 1.0f))).xyz), water_normal);
    float refl_y = refl_dir.y;
    float3 refl_dir2;
    if (refl_y < 0.0f)
    {
        float3 refl_tmp = refl_dir;
        refl_tmp.y = -refl_y;
        refl_dir2 = refl_tmp;
    }
    else
    {
        refl_dir2 = refl_dir;
    }
    float3 ray_org = clip_ray.xyz;
    float4 refl_clip = clip_ray + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(refl_dir2, 0.0f)));
    float refl_w = refl_clip.w;
    float3 refl_end = (refl_clip / (float((refl_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(refl_w))).xxxx).xyz;
    float3 refl_dir3 = refl_end - ray_org;
    bool3 refl_neg = bool3(refl_dir3.x < 0.0f.xxx.x, refl_dir3.y < 0.0f.xxx.y, refl_dir3.z < 0.0f.xxx.z);
    float3 refl_slab_t = (float3(refl_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, refl_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, refl_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - ray_org) / refl_dir3;
    float3 refl_p0 = ray_org + (refl_dir3 * max(0.0f, max(refl_slab_t.x, refl_slab_t.y)));
    float3 refl_delta = refl_end - refl_p0;
    bool3 refl_pos = bool3(refl_delta.x >= 0.0f.xxx.x, refl_delta.y >= 0.0f.xxx.y, refl_delta.z >= 0.0f.xxx.z);
    float3 refl_t2 = (float3(refl_pos.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, refl_pos.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, refl_pos.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - refl_p0) / refl_delta;
    float3 refl_p1 = refl_p0 + (refl_delta * min(min(refl_t2.x, refl_t2.y), refl_t2.z));
        // ---- 反射光线在 half-depth 中 marching,求命中点与距离 ----
    bool refl_hit;
    float refl_t;
    do
    {
        uint refl_steps = uint(int(max(2u, min(8u, uint(int(floor(length((((refl_p1.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((refl_p0.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * float2(half_depth_tex_dims)))))))));
        float refl_done_t;
        float refl_res_t;
        float refl_res_diff;
        float refl_res_thick;
        float refl_res_hitdiff;
        bool refl_res_hit;
        do
        {
            float3 refl_step = refl_p1 - refl_p0;
            float refl_cnt;
            if (refl_p1.z > 9.5367431640625e-07f)
            {
                refl_cnt = float(refl_steps);
            }
            else
            {
                refl_cnt = max(1.0f, float(refl_steps) - 1.0f);
            }
            float refl_inv = 1.0f / refl_cnt;
            float refl_bis_t0;
            float refl_bis_diff;
            float refl_bis_hitdiff;
            float refl_bis_thick;
            float refl_bis_t1;
            bool refl_bis_hit;
            if (refl_steps > 0u)
            {
                float refl_first_t = lerp(0.0f, 1.0f, pow(g_noise * refl_inv, 1.0f));
                float3 refl_pos2 = refl_p0 + (refl_step * refl_first_t);
                float refl_invz = 1.0f / refl_pos2.z;
                float4 refl_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_pos2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float refl_depth = refl_depth4.x;
                float refl_diff = (1.00000202655792236328125f / refl_depth) - refl_invz;
                bool refl_first_hit = (refl_diff < 0.0f) ? true : false;
                float refl_ref_t0;
                float refl_ref_diff;
                float refl_ref_hitdiff;
                float refl_ref_thick;
                float refl_ref_t1;
                bool refl_ref_hit;
                float refl_ref_tprev;
                if (refl_first_hit)
                {
                    refl_ref_t0 = 0.0f;
                    refl_ref_diff = refl_diff;
                    refl_ref_hitdiff = refl_invz - (1.0f / refl_depth);
                    refl_ref_thick = frame_constants_view_constants[0u].clip_to_view[3].z;
                    refl_ref_t1 = refl_first_t;
                    refl_ref_hit = refl_first_hit;
                    refl_ref_tprev = 0.0f;
                }
                else
                {
                    float refl_prev_diff;
                    float refl_prev_t;
                    refl_prev_diff = refl_diff;
                    refl_prev_t = refl_first_t;
                    float refl_cur_diff;
                    bool refl_cur_hit;
                    float refl_cur_t;
                    float refl_out_diff;
                    float refl_out_hitdiff;
                    float refl_out_thick;
                    float refl_out_t;
                    bool refl_out_hit;
                    uint refl_i = 1u;
                    bool refl_prev_hit = refl_first_hit;
                    for (;;)
                    {
                        if (refl_i < refl_steps)
                        {
                            refl_cur_t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(refl_i) + g_noise) * refl_inv, 1.0f)));
                            float3 refl_cur_pos = refl_p0 + (refl_step * refl_cur_t);
                            float refl_cur_invz = 1.0f / refl_cur_pos.z;
                            float4 refl_cur_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_cur_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                            float refl_cur_depth = refl_cur_depth4.x;
                            refl_cur_diff = (1.00000202655792236328125f / refl_cur_depth) - refl_cur_invz;
                            refl_cur_hit = (refl_cur_diff < 0.0f) ? true : false;
                            if (refl_cur_hit)
                            {
                                refl_out_diff = refl_cur_diff;
                                refl_out_hitdiff = refl_cur_invz - (1.0f / refl_cur_depth);
                                refl_out_thick = frame_constants_view_constants[0u].clip_to_view[3].z;
                                refl_out_t = refl_cur_t;
                                refl_out_hit = refl_cur_hit;
                                break;
                            }
                            refl_i++;
                            refl_prev_diff = refl_cur_diff;
                            refl_prev_hit = refl_cur_hit;
                            refl_prev_t = refl_cur_t;
                            continue;
                        }
                        else
                        {
                            refl_out_diff = 0.0f;
                            refl_out_hitdiff = 0.0f;
                            refl_out_thick = 0.0f;
                            refl_out_t = 1.0f;
                            refl_out_hit = refl_prev_hit;
                            break;
                        }
                    }
                    refl_ref_t0 = refl_prev_diff;
                    refl_ref_diff = refl_out_diff;
                    refl_ref_hitdiff = refl_out_hitdiff;
                    refl_ref_thick = refl_out_thick;
                    refl_ref_t1 = refl_out_t;
                    refl_ref_hit = refl_out_hit;
                    refl_ref_tprev = refl_prev_t;
                }
                refl_bis_t0 = refl_ref_t0;
                refl_bis_diff = refl_ref_diff;
                refl_bis_hitdiff = refl_ref_hitdiff;
                refl_bis_thick = refl_ref_thick;
                refl_bis_t1 = refl_ref_t1;
                refl_bis_hit = refl_ref_hit;
                refl_done_t = refl_ref_tprev;
            }
            else
            {
                refl_bis_t0 = 0.0f;
                refl_bis_diff = 0.0f;
                refl_bis_hitdiff = 0.0f;
                refl_bis_thick = 0.0f;
                refl_bis_t1 = 1.0f;
                refl_bis_hit = false;
                refl_done_t = 0.0f;
            }
            if (refl_bis_hit)
            {
                float refl_bis_lo;
                float refl_bis_diff2;
                float refl_bis_hitdiff2;
                float refl_bis_thick2;
                float refl_bis_hi;
                float refl_bis_t2;
                refl_bis_lo = refl_done_t;
                refl_bis_diff2 = refl_bis_t0;
                refl_bis_hitdiff2 = refl_bis_diff;
                refl_bis_thick2 = refl_bis_hitdiff;
                refl_bis_hi = refl_bis_thick;
                refl_bis_t2 = refl_bis_t1;
                for (uint refl_bis_loop = 0u; refl_bis_loop < 3u; )
                {
                    float refl_bis_mid = refl_bis_lo + refl_bis_t2;
                    float refl_bis_mid2 = refl_bis_mid * 0.5f;
                    float3 refl_bis_pos = refl_p0 + (refl_step * refl_bis_mid2);
                    float refl_bis_invz = 1.0f / refl_bis_pos.z;
                    float4 refl_bis_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_bis_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float refl_bis_depth = refl_bis_depth4.x;
                    float refl_bis_diff3 = (1.00000202655792236328125f / refl_bis_depth) - refl_bis_invz;
                    bool refl_bis_hit2 = (refl_bis_diff3 < 0.0f) ? true : false;
                    refl_bis_lo = refl_bis_hit2 ? refl_bis_lo : refl_bis_mid2;
                    refl_bis_diff2 = refl_bis_hit2 ? refl_bis_diff2 : refl_bis_diff3;
                    refl_bis_hitdiff2 = refl_bis_hit2 ? refl_bis_diff3 : refl_bis_hitdiff2;
                    refl_bis_thick2 = refl_bis_hit2 ? (refl_bis_invz - (1.0f / refl_bis_depth)) : refl_bis_thick2;
                    refl_bis_hi = refl_bis_hit2 ? frame_constants_view_constants[0u].clip_to_view[3].z : refl_bis_hi;
                    refl_bis_t2 = refl_bis_hit2 ? refl_bis_mid2 : refl_bis_t2;
                    refl_bis_loop++;
                    continue;
                }
                float refl_final_t = lerp(refl_bis_lo, refl_bis_t2, refl_bis_diff2 / (refl_bis_diff2 + (-refl_bis_hitdiff2)));
                float3 refl_final_pos = refl_p0 + (refl_step * refl_final_t);
                float refl_final_invz = 1.0f / refl_final_pos.z;
                float4 refl_final_depth4 = half_depth_tex.SampleLevel(sampler_nnc, (refl_final_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                float refl_final_depth = refl_final_depth4.x;
                float refl_final_diff = (1.00000202655792236328125f / refl_final_depth) - refl_final_invz;
                bool refl_final_hit = (abs(refl_final_diff) < (refl_bis_diff2 * 0.89999997615814208984375f)) ? true : false;
                refl_res_t = refl_final_hit ? refl_final_t : refl_bis_t2;
                refl_res_diff = refl_final_hit ? refl_final_diff : refl_bis_hitdiff2;
                refl_res_thick = refl_final_hit ? frame_constants_view_constants[0u].clip_to_view[3].z : refl_bis_hi;
                refl_res_hitdiff = refl_final_hit ? (refl_final_invz - (1.0f / refl_final_depth)) : refl_bis_thick2;
                refl_res_hit = true;
                break;
            }
            else
            {
                refl_res_t = refl_done_t;
                refl_res_diff = zero_f;
                refl_res_thick = zero_f;
                refl_res_hitdiff = zero_f;
                refl_res_hit = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool refl_cond;
        if (refl_res_hit)
        {
            bool refl_cond2;
            if (refl_res_hitdiff < refl_res_thick)
            {
                refl_cond2 = refl_res_diff < refl_res_thick;
            }
            else
            {
                refl_cond2 = false;
            }
            refl_cond = refl_cond2;
        }
        else
        {
            refl_cond = false;
        }
        if (refl_cond)
        {
            refl_hit = true;
            refl_t = refl_res_t;
            break;
        }
        refl_hit = false;
        refl_t = refl_done_t;
        break;
    } while(false);
    // ---- 反射结果:跨象限一致化 + 写入水 RT ray(每 2x2 块选一个代表) ----
    float quad_t = QuadReadAcrossX(refl_t);
    float quad_hit = QuadReadAcrossX(float(refl_hit));
    bool quad_flag = (int(quad_hit != 0.0f) & int(!refl_hit)) != 0;
    bool quad_hit2 = quad_flag ? true : refl_hit;
    float quad_t2 = quad_flag ? quad_t : refl_t;
    float quad_t3 = QuadReadAcrossY(quad_t2);
    float quad_hit3 = QuadReadAcrossY(float(quad_hit2));
    bool quad_flag2 = (int(quad_hit3 != 0.0f) & int(!quad_hit2)) != 0;
    if (quad_flag2 ? true : quad_hit2)
    {
    }
    else
    {
        float3 rt_pos = lerp(refl_p0, refl_p1, (quad_flag2 ? quad_t3 : quad_t2).xxx);
        float4 rt_world4 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(rt_pos, 1.0f)));
        uint lane_mask;
        if (is_helper)
        {
            lane_mask = 0u;
        }
        else
        {
            uint2 lane_parity = pixel_uv & uint2(1u, 1u);
            lane_mask = 1u << ((lane_parity.x + (lane_parity.y * 2u)) & 31u);
        }
        uint lane_x = QuadReadAcrossX(lane_mask);
        uint lane_sum = lane_mask + lane_x;
        uint lane_y = QuadReadAcrossY(lane_sum);
        uint lane_total = lane_sum + lane_y;
        uint2 lane_sel = pixel_uv & uint2(1u, 1u);
        if ((lane_sel.x + (lane_sel.y * 2u)) == firstbithigh(lane_total))
        {
            uint rt_index;
            __alias_water_rt_count_buffer.InterlockedAdd(0, 1u, rt_index);
            uint2 rt_pixel = pixel_uv / uint2(2u, 2u);
            float3 cube_dir = water_normal / ((abs(water_normal.x) + abs(water_normal.y)) + abs(water_normal.z)).xxx;
            float3 cube_dir2;
            if (cube_dir.z < 0.0f)
            {
                float2 cube_xy = (1.0f.xx - abs(cube_dir.yx)) * ((step(0.0f.xx, cube_dir.xy) * 2.0f) - 1.0f.xx);
                cube_dir2 = float3(cube_xy.x, cube_xy.y, cube_dir.z);
            }
            else
            {
                cube_dir2 = cube_dir;
            }
            float2 cube_uv = (cube_dir2.xy * 0.5f) + 0.5f.xx;
            __alias_water_rt_ray_buffer.Store<uint4>(rt_index * 16 + 0, uint4(rt_pixel.x + (rt_pixel.y * 65536u), spvPackHalf2x16(float2(cube_uv.x, 0.0f)) | (spvPackHalf2x16(float2(cube_uv.y, 0.0f)) << 16u), asuint(dot(refl_dir2, (rt_world4.xyz / rt_world4.w.xxx) - (world_ray.xyz / world_ray.w.xxx))), (asuint(gl_FragCoord.z) & 4294967280u) + lane_total));
        }
    }
}

[earlydepthstencil]
void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
