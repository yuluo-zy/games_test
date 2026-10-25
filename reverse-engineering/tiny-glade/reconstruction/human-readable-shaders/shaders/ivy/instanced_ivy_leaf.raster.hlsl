// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== Instanced ivy leaves (实例常春藤叶片)
// 中文说明:
//   每个 IvyLeaf 实例把叶片四边形沿其前向/枝条方向定向(花 vs 叶变体),
//   把锚点抬高到地形高度,剔除路径掩码拱形淡出内的叶片(通过 NaN 位置),
//   用 3D value-noise 位移,并按每叶种子抖动 UV。片元写出打包的世界法线与
//   sqrt 后的叶片颜色;阴影渲染关闭时做棋盘剔除。
//   两个发行变体 b903e43ffb3da915(全)/f0adff76827e73ff(精简),各自 VS/PS
//   主体为 spirv-cross 转译(行为精确,内部临时变量为 SPIR-V 编号)。

// ==RECONSTRUCTED== Instanced ivy leaves. Each IvyLeaf instance orients a leaf quad along its
// SOURCE PATH : shaders/ivy/instanced_ivy_leaf.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ivy_instanced_ivy_leaf.raster.hlsl_b903e43ffb3da915.bin
//     file_sha256  : da151193d8d424836c573bf743671b7b9907bf8dca37de3feb609e02d9f13b61
//     pass/stage   : _ivy_instanced_ivy_leaf (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : b903e43ffb3da915
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvyLeaf, ivy_leaf_buffer, terrain_heightmap, path_mask, vs_main, sampler_llr, ivy_leaf_color
//   _ivy_instanced_ivy_leaf.raster.hlsl_f0adff76827e73ff.bin
//     file_sha256  : 19f7ffcc2aa8298617a09ccef06d3605284a93d07c0df0e52d5463fe76bc3829
//     pass/stage   : _ivy_instanced_ivy_leaf (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : f0adff76827e73ff
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvyLeaf, ivy_leaf_buffer, terrain_heightmap, path_mask, vs_main, ps_main
// ==RECONSTRUCTED==
// Instanced ivy leaves. Each IvyLeaf instance orients a leaf quad along its
// forward/branch direction (flower vs leaf variants), lifts the anchor to the
// terrain height, culls leaves inside the path-mask arch fade via a NaN
// position, displaces by 3D value noise, and jitters UVs by a per-leaf seed.
// The fragment writes a packed world normal and sqrt'd leaf color, with a
// checkerboard cull when shadow rendering is disabled.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "arch_noise_discard.inc.hlsl"

// 复现原版共享 13 采样器 $Globals 布局(set0/b13)。

// 每着色器 $Globals 成员(隐式 $Globals UBO,set0/b13)。
int is_flower;
float4x4 model_to_world;
uint2 ivy_leaf_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;
int bool_render_shadows;

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

StructuredBuffer<IvyLeaf> ivy_leaf_buffer : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> ivy_leaf_color : register(t15, space0);

struct VSInput
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(1)]] float3 normal: Vertex_Normal;
    [[vk::location(2)]] float2 uv: Vertex_UV;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 world_pos : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 world_pos;
    float3 normal;
    float2 uv;
    switch (0u)
    {
    default:
        float3 pos = ivy_leaf_buffer[input.instance].position;
    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default,
                 pos.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x);
    float arch_height = ivy_leaf_buffer[input.instance].arch_height;
    float size = ivy_leaf_buffer[input.instance].size;
    float base = arch_height - min(h, 0.0f);
    float3 vp = (input.position * 0.75f) * size;
    float3 scaled = vp;
    if (glade_theme_id == 4u)
    {
        scaled = vp * 0.85f;
    }
    bool flower = (is_flower == 1);

    float3 bdir = ivy_leaf_buffer[input.instance].branch_dir;
    float3 vx, vy, vz, voff;
    if (flower)
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 up_cross = cross(float3(0.0f, 1.0f, 0.0f), fwd);
        vz = up_cross;
        vy = fwd;
        vx = cross(up_cross, fwd);
        voff = scaled + (fwd * 0.1f);
    }
    else
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 w = normalize(cross(fwd, normalize(lerp(bdir, float3(0.0f, 1.0f, 0.0f),
                       lerp(0.5f, 1.0f, abs(bdir.y)).xxx))));
        vz = fwd;
        vy = normalize(cross(w, fwd));
        vx = w;
        voff = scaled;
    }
    float4x4 rot = transpose(float4x4(float4(vx, 0.0f), float4(vy, 0.0f), float4(vz, 0.0f),
                                      float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 offset_pos = mul(rot, float4(voff, 1.0f)).xyz;
    float3 anchor = mul(rot, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + pos;
    float path = path_mask.SampleLevel(sampler_llb,
                 anchor.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x;
    bool over_path = path > 0.1f;
    if (over_path)
    {
        float y = anchor.y;
        float n = perlin(anchor.xz * 0.6f + y * 0.6f);
        float t = clamp(base - y, 0.0f, 1.0f);
        float d = (1.0f - exp(-5.0f * path)) * base * (t * t * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
        if (y < d)
        {
            position = asfloat(0x7fbfffffu).xxxx;
            world_pos = 0.0f.xxx;
            normal = 0.0f.xxx;
            uv = 0.0f.xx;
            break;
        }
    }
    float3 lifted = pos;
    lifted.y = pos.y + h;
    bool at_base = flower ? (pos.y < 0.9f) : false;
    float3 world = (at_base ? float3(0.0f, 0.0f, 0.0f) : offset_pos) + lifted;
    uint alt_seed;
    float2 disp = clamp((perlin3(world * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
    world += float3(disp.x, 0.0f, disp.y) * 0.06f;

    uint jh1 = uint(ivy_leaf_buffer[input.instance].seed) ^ 2342304u;
    jh1 += (jh1 << 10u);
    jh1 ^= (jh1 >> 6u);
    jh1 += (jh1 << 3u);
    jh1 ^= (jh1 >> 11u);
    jh1 += (jh1 << 15u);
    uint jh2 = uint(ivy_leaf_buffer[input.instance].seed) ^ 9344565u;
    jh2 += (jh2 << 10u);
    jh2 ^= (jh2 >> 6u);
    jh2 += (jh2 << 3u);
    jh2 ^= (jh2 >> 11u);
    jh2 += (jh2 << 15u);
    float2 jitter = float2(asfloat((jh1 & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                           asfloat((jh2 & 0x7FFFFFu) | 0x3F800000u) - 1.0f);

    position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    world_pos = world;
    normal = mul(rot, float4(input.normal, 0.0f)).xyz;
    uv = input.uv * (flower ? 0.3f : 0.1f) + jitter;
    break;
    }
    output.position = position;
    output.world_pos = world_pos;
    output.normal = normal;
    output.uv = uv;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
    float4 position : SV_Position;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    if (!(bool_render_shadows == 1))
    {
        uint2 checker = uint2(input.position.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (checker.x + checker.y * 2u))
        {
            discard;
        }
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(ivy_leaf_color.Sample(sampler_llr, input.uv).xyz * 1.1f, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 1291845632u;
    return uint2(n, m);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 转译段(b903 变体 VS):spirv-cross,行为精确;与上方手写段算法一致。
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

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

static uint hash_default_u;

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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

ByteAddressBuffer ivy_leaf_buffer : register(t16, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 out_pos;
    float3 out_world;
    float3 out_normal;
    float2 out_uv;
    do
    {
        // 读取叶片锚点;采样地形高度抬高,并按 0.75*size 放大四边形顶点。
        float3 anchor = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float anchor_h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (anchor.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float base = ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(anchor_h, 0.0f);
        float3 leaf_size = (in_var_Vertex_Position * 0.75f) * ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
        float3 leaf_size2;
        if (frame_constants_glade_theme_id == 4u)
        {
            leaf_size2 = leaf_size * 0.85000002384185791015625f;
        }
        else
        {
            leaf_size2 = leaf_size;
        }
        bool is_flower_flag = is_flower == 1;
        // 花 vs 叶两种定向:花用 dir 构造正交系,叶按墙面法线/朝上混合定向。
        float3 basis_z;
        float3 basis_y;
        float3 basis_x;
        float3 corner_local;
        if (is_flower_flag)
        {
            float3 flower_side = cross(float3(0.0f, 1.0f, 0.0f), ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            basis_z = flower_side;
            basis_y = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            basis_x = cross(flower_side, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            corner_local = leaf_size2 + (ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16) * 0.100000001490116119384765625f);
        }
        else
        {
            float3 leaf_up = normalize(cross(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16), normalize(lerp(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32), float3(0.0f, 1.0f, 0.0f), lerp(0.5f, 1.0f, abs(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32).y)).xxx))));
            basis_z = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            basis_y = normalize(cross(leaf_up, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16)));
            basis_x = leaf_up;
            corner_local = leaf_size2;
        }
        float4x4 rot = transpose(float4x4(float4(basis_x, 0.0f), float4(basis_y, 0.0f), float4(basis_z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 corner_off = mul(rot, float4(corner_local, 1.0f)).xyz;
        float3 leaf_pos = mul(rot, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + anchor;
        float4 path_smp = path_mask.SampleLevel(sampler_llb, (leaf_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
        float path = path_smp.x;
        bool over_path = path > 0.100000001490116119384765625f;
        // 路径掩码淡出:路径上且低于 arch-noise 淡出曲面的顶点发射 NaN 位置被剔除。
        if (over_path)
        {
            float leaf_y = leaf_pos.y;
            float2 perlin_pos = (float2(leaf_pos.xz) * 0.60000002384185791015625f) + (leaf_y * 0.60000002384185791015625f).xx;
            int2 perlin_cell = int2(floor(perlin_pos));
            float2 perlin_frac = frac(perlin_pos);
            uint2 cell_u = uint2(perlin_cell);
            uint v275 = cell_u.y;
            uint v277 = v275 + (v275 << 10u);
            uint v279 = v277 ^ (v277 >> 6u);
            uint v281 = v279 + (v279 << 3u);
            uint v283 = v281 ^ (v281 >> 11u);
            uint v289 = (((cell_u.x * 1664525u) + (v283 + (v283 << 15u))) + 1013904223u) * 1664525u;
            uint v291 = v289 ^ (v289 >> 11u);
            uint v294 = v291 ^ ((v291 << 7u) & 2636928640u);
            uint v297 = v294 ^ ((v294 << 15u) & 4022730752u);
            uint2 v305 = uint2(perlin_cell + int2(1, 0));
            uint v307 = v305.y;
            uint v309 = v307 + (v307 << 10u);
            uint v311 = v309 ^ (v309 >> 6u);
            uint v313 = v311 + (v311 << 3u);
            uint v315 = v313 ^ (v313 >> 11u);
            uint v321 = (((v305.x * 1664525u) + (v315 + (v315 << 15u))) + 1013904223u) * 1664525u;
            uint v323 = v321 ^ (v321 >> 11u);
            uint v326 = v323 ^ ((v323 << 7u) & 2636928640u);
            uint v329 = v326 ^ ((v326 << 15u) & 4022730752u);
            uint2 v337 = uint2(perlin_cell + int2(0, 1));
            uint v339 = v337.y;
            uint v341 = v339 + (v339 << 10u);
            uint v343 = v341 ^ (v341 >> 6u);
            uint v345 = v343 + (v343 << 3u);
            uint v347 = v345 ^ (v345 >> 11u);
            uint v353 = (((v337.x * 1664525u) + (v347 + (v347 << 15u))) + 1013904223u) * 1664525u;
            uint v355 = v353 ^ (v353 >> 11u);
            uint v358 = v355 ^ ((v355 << 7u) & 2636928640u);
            uint v361 = v358 ^ ((v358 << 15u) & 4022730752u);
            uint2 v369 = uint2(perlin_cell + int2(1, 1));
            uint v371 = v369.y;
            uint v373 = v371 + (v371 << 10u);
            uint v375 = v373 ^ (v373 >> 6u);
            uint v377 = v375 + (v375 << 3u);
            uint v379 = v377 ^ (v377 >> 11u);
            uint v385 = (((v369.x * 1664525u) + (v379 + (v379 << 15u))) + 1013904223u) * 1664525u;
            uint v387 = v385 ^ (v385 >> 11u);
            uint v390 = v387 ^ ((v387 << 7u) & 2636928640u);
            uint v393 = v390 ^ ((v390 << 15u) & 4022730752u);
            float2 n_smooth = (perlin_frac * perlin_frac) * (3.0f.xx - (perlin_frac * 2.0f));
            float n_smooth_x = n_smooth.x;
            float om_n_smooth_x = 1.0f - n_smooth_x;
            float n_smooth_y = n_smooth.y;
            float climb_t = clamp(base - leaf_y, 0.0f, 1.0f);
            if (leaf_y < (((1.0f - exp((-5.0f) * path)) * base) * ((climb_t * climb_t) * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - n_smooth_y) * (((asfloat(((v297 ^ (v297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) + ((asfloat(((v329 ^ (v329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x))) + (((asfloat(((v361 ^ (v361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) * n_smooth_y)) + (((asfloat(((v393 ^ (v393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x) * n_smooth_y)), 0.800000011920928955078125f, 1.0f))))
            {
                out_pos = asfloat(0x7fbfffffu /* nan */).xxxx;
                out_world = 0.0f.xxx;
                out_normal = 0.0f.xxx;
                out_uv = 0.0f.xx;
                break;
            }
        }
        bool over_path_c = over_path ? true : false;
        bool flower_low;
        if (is_flower_flag)
        {
            flower_low = anchor.y < 0.89999997615814208984375f;
        }
        else
        {
            flower_low = false;
        }
        bool3 flower_mask = flower_low.xxx;
        float3 anchor_up = anchor;
        anchor_up.y = anchor.y + anchor_h;
        float3 local_pos = float3(flower_mask.x ? 0.0f.xxx.x : corner_off.x, flower_mask.y ? 0.0f.xxx.y : corner_off.y, flower_mask.z ? 0.0f.xxx.z : corner_off.z) + anchor_up;
        // 3D value-noise(perlin3)水平位移:8 角 Wang 哈希 -> 平滑插值 -> [-0.5,0.5]。
        float3 local_noise_pos = local_pos * 0.4000000059604644775390625f;
        int3 noise_cell = int3(floor(local_noise_pos));
        float3 noise_frac = frac(local_noise_pos);
        uint3 noise_cell_u = uint3(noise_cell);
        uint v451 = noise_cell_u.z;
        uint v453 = v451 + (v451 << 10u);
        uint v455 = v453 ^ (v453 >> 6u);
        uint v457 = v455 + (v455 << 3u);
        uint v459 = v457 ^ (v457 >> 11u);
        bool v462 = over_path_c ? over_path_c : true;
        uint v463 = over_path_c ? (over_path ? 1664525u : hash_default_u) : 1664525u;
        uint v464 = over_path_c ? (over_path ? 1013904223u : hash_default_u) : 1013904223u;
        uint v468 = (((noise_cell_u.y * v463) + (v459 + (v459 << 15u))) + v464) * v463;
        uint v470 = v468 ^ (v468 >> 11u);
        uint v473 = v470 ^ ((v470 << 7u) & 2636928640u);
        uint v476 = v473 ^ ((v473 << 15u) & 4022730752u);
        bool v479 = v462 ? v462 : true;
        uint v480 = v462 ? v463 : 1664525u;
        uint v481 = v462 ? v464 : 1013904223u;
        uint v485 = (((noise_cell_u.x * v480) + (v476 ^ (v476 >> 18u))) + v481) * v480;
        uint v487 = v485 ^ (v485 >> 11u);
        uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
        uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
        uint v495 = v493 ^ (v493 >> 18u);
        uint v497 = v495 + (v495 << 10u);
        uint v499 = v497 ^ (v497 >> 6u);
        uint v501 = v499 + (v499 << 3u);
        uint v503 = v501 ^ (v501 >> 11u);
        uint v506 = over_path_c ? (over_path ? 8388607u : hash_default_u) : 8388607u;
        uint v507 = over_path_c ? (over_path ? 1065353216u : hash_default_u) : 1065353216u;
        uint v512 = v462 ? v506 : 8388607u;
        uint v513 = v462 ? v507 : 1065353216u;
        uint3 v520 = uint3(noise_cell + int3(1, 0, 0));
        uint v523 = v520.z;
        uint v525 = v523 + (v523 << 10u);
        uint v527 = v525 ^ (v525 >> 6u);
        uint v529 = v527 + (v527 << 3u);
        uint v531 = v529 ^ (v529 >> 11u);
        bool v534 = v479 ? v479 : true;
        uint v535 = v479 ? v480 : 1664525u;
        uint v536 = v479 ? v481 : 1013904223u;
        uint v540 = (((v520.y * v535) + (v531 + (v531 << 15u))) + v536) * v535;
        uint v542 = v540 ^ (v540 >> 11u);
        uint v545 = v542 ^ ((v542 << 7u) & 2636928640u);
        uint v548 = v545 ^ ((v545 << 15u) & 4022730752u);
        bool v551 = v534 ? v534 : true;
        uint v552 = v534 ? v535 : 1664525u;
        uint v553 = v534 ? v536 : 1013904223u;
        uint v557 = (((v520.x * v552) + (v548 ^ (v548 >> 18u))) + v553) * v552;
        uint v559 = v557 ^ (v557 >> 11u);
        uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
        uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
        uint v567 = v565 ^ (v565 >> 18u);
        uint v569 = v567 + (v567 << 10u);
        uint v571 = v569 ^ (v569 >> 6u);
        uint v573 = v571 + (v571 << 3u);
        uint v575 = v573 ^ (v573 >> 11u);
        uint v578 = v479 ? v512 : 8388607u;
        uint v579 = v479 ? v513 : 1065353216u;
        uint v584 = v534 ? v578 : 8388607u;
        uint v585 = v534 ? v579 : 1065353216u;
        uint3 v592 = uint3(noise_cell + int3(0, 1, 0));
        uint v595 = v592.z;
        uint v597 = v595 + (v595 << 10u);
        uint v599 = v597 ^ (v597 >> 6u);
        uint v601 = v599 + (v599 << 3u);
        uint v603 = v601 ^ (v601 >> 11u);
        bool v606 = v551 ? v551 : true;
        uint v607 = v551 ? v552 : 1664525u;
        uint v608 = v551 ? v553 : 1013904223u;
        uint v612 = (((v592.y * v607) + (v603 + (v603 << 15u))) + v608) * v607;
        uint v614 = v612 ^ (v612 >> 11u);
        uint v617 = v614 ^ ((v614 << 7u) & 2636928640u);
        uint v620 = v617 ^ ((v617 << 15u) & 4022730752u);
        bool v623 = v606 ? v606 : true;
        uint v624 = v606 ? v607 : 1664525u;
        uint v625 = v606 ? v608 : 1013904223u;
        uint v629 = (((v592.x * v624) + (v620 ^ (v620 >> 18u))) + v625) * v624;
        uint v631 = v629 ^ (v629 >> 11u);
        uint v634 = v631 ^ ((v631 << 7u) & 2636928640u);
        uint v637 = v634 ^ ((v634 << 15u) & 4022730752u);
        uint v639 = v637 ^ (v637 >> 18u);
        uint v641 = v639 + (v639 << 10u);
        uint v643 = v641 ^ (v641 >> 6u);
        uint v645 = v643 + (v643 << 3u);
        uint v647 = v645 ^ (v645 >> 11u);
        uint v650 = v551 ? v584 : 8388607u;
        uint v651 = v551 ? v585 : 1065353216u;
        uint v656 = v606 ? v650 : 8388607u;
        uint v657 = v606 ? v651 : 1065353216u;
        uint3 v664 = uint3(noise_cell + int3(1, 1, 0));
        uint v667 = v664.z;
        uint v669 = v667 + (v667 << 10u);
        uint v671 = v669 ^ (v669 >> 6u);
        uint v673 = v671 + (v671 << 3u);
        uint v675 = v673 ^ (v673 >> 11u);
        bool v678 = v623 ? v623 : true;
        uint v679 = v623 ? v624 : 1664525u;
        uint v680 = v623 ? v625 : 1013904223u;
        uint v684 = (((v664.y * v679) + (v675 + (v675 << 15u))) + v680) * v679;
        uint v686 = v684 ^ (v684 >> 11u);
        uint v689 = v686 ^ ((v686 << 7u) & 2636928640u);
        uint v692 = v689 ^ ((v689 << 15u) & 4022730752u);
        bool v695 = v678 ? v678 : true;
        uint v696 = v678 ? v679 : 1664525u;
        uint v697 = v678 ? v680 : 1013904223u;
        uint v701 = (((v664.x * v696) + (v692 ^ (v692 >> 18u))) + v697) * v696;
        uint v703 = v701 ^ (v701 >> 11u);
        uint v706 = v703 ^ ((v703 << 7u) & 2636928640u);
        uint v709 = v706 ^ ((v706 << 15u) & 4022730752u);
        uint v711 = v709 ^ (v709 >> 18u);
        uint v713 = v711 + (v711 << 10u);
        uint v715 = v713 ^ (v713 >> 6u);
        uint v717 = v715 + (v715 << 3u);
        uint v719 = v717 ^ (v717 >> 11u);
        uint v722 = v623 ? v656 : 8388607u;
        uint v723 = v623 ? v657 : 1065353216u;
        uint v728 = v678 ? v722 : 8388607u;
        uint v729 = v678 ? v723 : 1065353216u;
        uint3 v736 = uint3(noise_cell + int3(0, 0, 1));
        uint v739 = v736.z;
        uint v741 = v739 + (v739 << 10u);
        uint v743 = v741 ^ (v741 >> 6u);
        uint v745 = v743 + (v743 << 3u);
        uint v747 = v745 ^ (v745 >> 11u);
        bool v750 = v695 ? v695 : true;
        uint v751 = v695 ? v696 : 1664525u;
        uint v752 = v695 ? v697 : 1013904223u;
        uint v756 = (((v736.y * v751) + (v747 + (v747 << 15u))) + v752) * v751;
        uint v758 = v756 ^ (v756 >> 11u);
        uint v761 = v758 ^ ((v758 << 7u) & 2636928640u);
        uint v764 = v761 ^ ((v761 << 15u) & 4022730752u);
        bool v767 = v750 ? v750 : true;
        uint v768 = v750 ? v751 : 1664525u;
        uint v769 = v750 ? v752 : 1013904223u;
        uint v773 = (((v736.x * v768) + (v764 ^ (v764 >> 18u))) + v769) * v768;
        uint v775 = v773 ^ (v773 >> 11u);
        uint v778 = v775 ^ ((v775 << 7u) & 2636928640u);
        uint v781 = v778 ^ ((v778 << 15u) & 4022730752u);
        uint v783 = v781 ^ (v781 >> 18u);
        uint v785 = v783 + (v783 << 10u);
        uint v787 = v785 ^ (v785 >> 6u);
        uint v789 = v787 + (v787 << 3u);
        uint v791 = v789 ^ (v789 >> 11u);
        uint v794 = v695 ? v728 : 8388607u;
        uint v795 = v695 ? v729 : 1065353216u;
        uint v800 = v750 ? v794 : 8388607u;
        uint v801 = v750 ? v795 : 1065353216u;
        uint3 v808 = uint3(noise_cell + int3(1, 0, 1));
        uint v811 = v808.z;
        uint v813 = v811 + (v811 << 10u);
        uint v815 = v813 ^ (v813 >> 6u);
        uint v817 = v815 + (v815 << 3u);
        uint v819 = v817 ^ (v817 >> 11u);
        bool v822 = v767 ? v767 : true;
        uint v823 = v767 ? v768 : 1664525u;
        uint v824 = v767 ? v769 : 1013904223u;
        uint v828 = (((v808.y * v823) + (v819 + (v819 << 15u))) + v824) * v823;
        uint v830 = v828 ^ (v828 >> 11u);
        uint v833 = v830 ^ ((v830 << 7u) & 2636928640u);
        uint v836 = v833 ^ ((v833 << 15u) & 4022730752u);
        bool v839 = v822 ? v822 : true;
        uint v840 = v822 ? v823 : 1664525u;
        uint v841 = v822 ? v824 : 1013904223u;
        uint v845 = (((v808.x * v840) + (v836 ^ (v836 >> 18u))) + v841) * v840;
        uint v847 = v845 ^ (v845 >> 11u);
        uint v850 = v847 ^ ((v847 << 7u) & 2636928640u);
        uint v853 = v850 ^ ((v850 << 15u) & 4022730752u);
        uint v855 = v853 ^ (v853 >> 18u);
        uint v857 = v855 + (v855 << 10u);
        uint v859 = v857 ^ (v857 >> 6u);
        uint v861 = v859 + (v859 << 3u);
        uint v863 = v861 ^ (v861 >> 11u);
        uint v866 = v767 ? v800 : 8388607u;
        uint v867 = v767 ? v801 : 1065353216u;
        uint v872 = v822 ? v866 : 8388607u;
        uint v873 = v822 ? v867 : 1065353216u;
        uint3 v880 = uint3(noise_cell + int3(0, 1, 1));
        uint v883 = v880.z;
        uint v885 = v883 + (v883 << 10u);
        uint v887 = v885 ^ (v885 >> 6u);
        uint v889 = v887 + (v887 << 3u);
        uint v891 = v889 ^ (v889 >> 11u);
        bool v894 = v839 ? v839 : true;
        uint v895 = v839 ? v840 : 1664525u;
        uint v896 = v839 ? v841 : 1013904223u;
        uint v900 = (((v880.y * v895) + (v891 + (v891 << 15u))) + v896) * v895;
        uint v902 = v900 ^ (v900 >> 11u);
        uint v905 = v902 ^ ((v902 << 7u) & 2636928640u);
        uint v908 = v905 ^ ((v905 << 15u) & 4022730752u);
        bool v911 = v894 ? v894 : true;
        uint v912 = v894 ? v895 : 1664525u;
        uint v913 = v894 ? v896 : 1013904223u;
        uint v917 = (((v880.x * v912) + (v908 ^ (v908 >> 18u))) + v913) * v912;
        uint v919 = v917 ^ (v917 >> 11u);
        uint v922 = v919 ^ ((v919 << 7u) & 2636928640u);
        uint v925 = v922 ^ ((v922 << 15u) & 4022730752u);
        uint v927 = v925 ^ (v925 >> 18u);
        uint v929 = v927 + (v927 << 10u);
        uint v931 = v929 ^ (v929 >> 6u);
        uint v933 = v931 + (v931 << 3u);
        uint v935 = v933 ^ (v933 >> 11u);
        uint v938 = v839 ? v872 : 8388607u;
        uint v939 = v839 ? v873 : 1065353216u;
        uint v944 = v894 ? v938 : 8388607u;
        uint v945 = v894 ? v939 : 1065353216u;
        uint3 v952 = uint3(noise_cell + int3(1, 1, 1));
        uint v955 = v952.z;
        uint v957 = v955 + (v955 << 10u);
        uint v959 = v957 ^ (v957 >> 6u);
        uint v961 = v959 + (v959 << 3u);
        uint v963 = v961 ^ (v961 >> 11u);
        bool v966 = v911 ? v911 : true;
        uint v967 = v911 ? v912 : 1664525u;
        uint v968 = v911 ? v913 : 1013904223u;
        uint v972 = (((v952.y * v967) + (v963 + (v963 << 15u))) + v968) * v967;
        uint v974 = v972 ^ (v972 >> 11u);
        uint v977 = v974 ^ ((v974 << 7u) & 2636928640u);
        uint v980 = v977 ^ ((v977 << 15u) & 4022730752u);
        uint v983 = v966 ? v967 : 1664525u;
        uint v988 = (((v952.x * v983) + (v980 ^ (v980 >> 18u))) + (v966 ? v968 : 1013904223u)) * v983;
        uint v990 = v988 ^ (v988 >> 11u);
        uint v993 = v990 ^ ((v990 << 7u) & 2636928640u);
        uint v996 = v993 ^ ((v993 << 15u) & 4022730752u);
        uint v998 = v996 ^ (v996 >> 18u);
        uint v1000 = v998 + (v998 << 10u);
        uint v1002 = v1000 ^ (v1000 >> 6u);
        uint v1004 = v1002 + (v1002 << 3u);
        uint v1006 = v1004 ^ (v1004 >> 11u);
        uint v1009 = v911 ? v944 : 8388607u;
        uint v1010 = v911 ? v945 : 1065353216u;
        bool v1015 = v966 ? v966 : true;
        uint v1016 = v966 ? v1009 : 8388607u;
        uint v1017 = v966 ? v1010 : 1065353216u;
        float3 noise_smooth = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
        float nx = noise_smooth.x;
        float om_nx = 1.0f - nx;
        float ny = noise_smooth.y;
        float om_ny = 1.0f - ny;
        float nz = noise_smooth.z;
        float om_nz = 1.0f - nz;
        float2 noise_sum0 = (((((float2(asfloat((v495 & v506) | v507) - 1.0f, asfloat(((v503 + (v503 << 15u)) & v512) | v513) - 1.0f) * om_nx) * om_ny) * om_nz) + (((float2(asfloat((v567 & v578) | v579) - 1.0f, asfloat(((v575 + (v575 << 15u)) & v584) | v585) - 1.0f) * nx) * om_ny) * om_nz)) + (((float2(asfloat((v639 & v650) | v651) - 1.0f, asfloat(((v647 + (v647 << 15u)) & v656) | v657) - 1.0f) * om_nx) * ny) * om_nz)) + (((float2(asfloat((v711 & v722) | v723) - 1.0f, asfloat(((v719 + (v719 << 15u)) & v728) | v729) - 1.0f) * nx) * ny) * om_nz);
        float2 noise_sum1 = (((noise_sum0 + (((float2(asfloat((v783 & v794) | v795) - 1.0f, asfloat(((v791 + (v791 << 15u)) & v800) | v801) - 1.0f) * om_nx) * om_ny) * nz)) + (((float2(asfloat((v855 & v866) | v867) - 1.0f, asfloat(((v863 + (v863 << 15u)) & v872) | v873) - 1.0f) * nx) * om_ny) * nz)) + (((float2(asfloat((v927 & v938) | v939) - 1.0f, asfloat(((v935 + (v935 << 15u)) & v944) | v945) - 1.0f) * om_nx) * ny) * nz)) + (((float2(asfloat((v998 & v1009) | v1010) - 1.0f, asfloat(((v1006 + (v1006 << 15u)) & v1016) | v1017) - 1.0f) * nx) * ny) * nz);
        float2 disp = clamp((noise_sum1 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float3 world_pos3 = local_pos + (float3(disp.x, 0.0f, disp.y) * 0.0599999986588954925537109375f);
        uint uv_seed_a = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 2342304);
        uint v1094 = uv_seed_a + (uv_seed_a << 10u);
        uint v1096 = v1094 ^ (v1094 >> 6u);
        uint v1098 = v1096 + (v1096 << 3u);
        uint v1100 = v1098 ^ (v1098 >> 11u);
        bool v1103 = v1015 ? v1015 : true;
        uint v1104 = v1015 ? v1016 : 8388607u;
        uint v1105 = v1015 ? v1017 : 1065353216u;
        uint uv_seed_b = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 9344565);
        uint v1114 = uv_seed_b + (uv_seed_b << 10u);
        uint v1116 = v1114 ^ (v1114 >> 6u);
        uint v1118 = v1116 + (v1116 << 3u);
        uint v1120 = v1118 ^ (v1118 >> 11u);
        // 世界->裁剪变换;输出世界位置、旋转后的法线,UV 按每叶种子抖动。
        out_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos3, 1.0f));
        out_world = world_pos3;
        out_normal = mul(rot, float4(in_var_Vertex_Normal, 0.0f)).xyz;
        out_uv = (in_var_Vertex_UV * (is_flower_flag ? 0.300000011920928955078125f : 0.100000001490116119384765625f)) + float2(asfloat(((v1100 + (v1100 << 15u)) & v1104) | v1105) - 1.0f, asfloat(((v1120 + (v1120 << 15u)) & (v1103 ? v1104 : 8388607u)) | (v1103 ? v1105 : 1065353216u)) - 1.0f);
        break;
    } while(false);
    gl_Position = out_pos;
    out_var_C1 = out_world;
    out_var_C2 = out_normal;
    out_var_C4 = out_uv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译段(b903 变体 PS):spirv-cross,行为精确;与上方手写段算法一致。
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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> ivy_leaf_color : register(t15, space0);

static float4 gl_FragCoord;
static float3 in_var_C2;
static float2 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 非阴影渲染时做棋盘剔除(像素奇偶与帧号匹配才保留),减少叶片开销。
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 pixel_parity = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (pixel_parity.x + (pixel_parity.y * 2u)))
        {
            discard;
        }
    }
    float3 packed_normal = (in_var_C2 * 0.5f) + 0.5f.xxx;
    float3 sc = sqrt(clamp(ivy_leaf_color.Sample(sampler_llr, in_var_C4).xyz * 1.10000002384185791015625f, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(sc.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(sc.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(sc.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 转译段(f0ad 变体 VS):spirv-cross,行为精确;与上方手写段算法一致。
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

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

static uint hash_default_u;

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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

ByteAddressBuffer ivy_leaf_buffer : register(t16, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 out_pos;
    float3 out_world;
    float3 out_normal;
    float2 out_uv;
    do
    {
        // 读取叶片锚点;采样地形高度抬高,并按 0.75*size 放大四边形顶点。
        float3 anchor = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float anchor_h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (anchor.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float base = ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(anchor_h, 0.0f);
        float3 leaf_size = (in_var_Vertex_Position * 0.75f) * ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
        float3 leaf_size2;
        if (frame_constants_glade_theme_id == 4u)
        {
            leaf_size2 = leaf_size * 0.85000002384185791015625f;
        }
        else
        {
            leaf_size2 = leaf_size;
        }
        bool is_flower_flag = is_flower == 1;
        // 花 vs 叶两种定向:花用 dir 构造正交系,叶按墙面法线/朝上混合定向。
        float3 basis_z;
        float3 basis_y;
        float3 basis_x;
        float3 corner_local;
        if (is_flower_flag)
        {
            float3 flower_side = cross(float3(0.0f, 1.0f, 0.0f), ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            basis_z = flower_side;
            basis_y = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            basis_x = cross(flower_side, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            corner_local = leaf_size2 + (ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16) * 0.100000001490116119384765625f);
        }
        else
        {
            float3 leaf_up = normalize(cross(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16), normalize(lerp(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32), float3(0.0f, 1.0f, 0.0f), lerp(0.5f, 1.0f, abs(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32).y)).xxx))));
            basis_z = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            basis_y = normalize(cross(leaf_up, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16)));
            basis_x = leaf_up;
            corner_local = leaf_size2;
        }
        float4x4 rot = transpose(float4x4(float4(basis_x, 0.0f), float4(basis_y, 0.0f), float4(basis_z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 corner_off = mul(rot, float4(corner_local, 1.0f)).xyz;
        float3 leaf_pos = mul(rot, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + anchor;
        float4 path_smp = path_mask.SampleLevel(sampler_llb, (leaf_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
        float path = path_smp.x;
        bool over_path = path > 0.100000001490116119384765625f;
        // 路径掩码淡出:路径上且低于 arch-noise 淡出曲面的顶点发射 NaN 位置被剔除。
        if (over_path)
        {
            float leaf_y = leaf_pos.y;
            float2 perlin_pos = (float2(leaf_pos.xz) * 0.60000002384185791015625f) + (leaf_y * 0.60000002384185791015625f).xx;
            int2 perlin_cell = int2(floor(perlin_pos));
            float2 perlin_frac = frac(perlin_pos);
            uint2 cell_u = uint2(perlin_cell);
            uint v275 = cell_u.y;
            uint v277 = v275 + (v275 << 10u);
            uint v279 = v277 ^ (v277 >> 6u);
            uint v281 = v279 + (v279 << 3u);
            uint v283 = v281 ^ (v281 >> 11u);
            uint v289 = (((cell_u.x * 1664525u) + (v283 + (v283 << 15u))) + 1013904223u) * 1664525u;
            uint v291 = v289 ^ (v289 >> 11u);
            uint v294 = v291 ^ ((v291 << 7u) & 2636928640u);
            uint v297 = v294 ^ ((v294 << 15u) & 4022730752u);
            uint2 v305 = uint2(perlin_cell + int2(1, 0));
            uint v307 = v305.y;
            uint v309 = v307 + (v307 << 10u);
            uint v311 = v309 ^ (v309 >> 6u);
            uint v313 = v311 + (v311 << 3u);
            uint v315 = v313 ^ (v313 >> 11u);
            uint v321 = (((v305.x * 1664525u) + (v315 + (v315 << 15u))) + 1013904223u) * 1664525u;
            uint v323 = v321 ^ (v321 >> 11u);
            uint v326 = v323 ^ ((v323 << 7u) & 2636928640u);
            uint v329 = v326 ^ ((v326 << 15u) & 4022730752u);
            uint2 v337 = uint2(perlin_cell + int2(0, 1));
            uint v339 = v337.y;
            uint v341 = v339 + (v339 << 10u);
            uint v343 = v341 ^ (v341 >> 6u);
            uint v345 = v343 + (v343 << 3u);
            uint v347 = v345 ^ (v345 >> 11u);
            uint v353 = (((v337.x * 1664525u) + (v347 + (v347 << 15u))) + 1013904223u) * 1664525u;
            uint v355 = v353 ^ (v353 >> 11u);
            uint v358 = v355 ^ ((v355 << 7u) & 2636928640u);
            uint v361 = v358 ^ ((v358 << 15u) & 4022730752u);
            uint2 v369 = uint2(perlin_cell + int2(1, 1));
            uint v371 = v369.y;
            uint v373 = v371 + (v371 << 10u);
            uint v375 = v373 ^ (v373 >> 6u);
            uint v377 = v375 + (v375 << 3u);
            uint v379 = v377 ^ (v377 >> 11u);
            uint v385 = (((v369.x * 1664525u) + (v379 + (v379 << 15u))) + 1013904223u) * 1664525u;
            uint v387 = v385 ^ (v385 >> 11u);
            uint v390 = v387 ^ ((v387 << 7u) & 2636928640u);
            uint v393 = v390 ^ ((v390 << 15u) & 4022730752u);
            float2 n_smooth = (perlin_frac * perlin_frac) * (3.0f.xx - (perlin_frac * 2.0f));
            float n_smooth_x = n_smooth.x;
            float om_n_smooth_x = 1.0f - n_smooth_x;
            float n_smooth_y = n_smooth.y;
            float climb_t = clamp(base - leaf_y, 0.0f, 1.0f);
            if (leaf_y < (((1.0f - exp((-5.0f) * path)) * base) * ((climb_t * climb_t) * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - n_smooth_y) * (((asfloat(((v297 ^ (v297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) + ((asfloat(((v329 ^ (v329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x))) + (((asfloat(((v361 ^ (v361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) * n_smooth_y)) + (((asfloat(((v393 ^ (v393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x) * n_smooth_y)), 0.800000011920928955078125f, 1.0f))))
            {
                out_pos = asfloat(0x7fbfffffu /* nan */).xxxx;
                out_world = 0.0f.xxx;
                out_normal = 0.0f.xxx;
                out_uv = 0.0f.xx;
                break;
            }
        }
        bool over_path_c = over_path ? true : false;
        bool flower_low;
        if (is_flower_flag)
        {
            flower_low = anchor.y < 0.89999997615814208984375f;
        }
        else
        {
            flower_low = false;
        }
        bool3 flower_mask = flower_low.xxx;
        float3 anchor_up = anchor;
        anchor_up.y = anchor.y + anchor_h;
        float3 local_pos = float3(flower_mask.x ? 0.0f.xxx.x : corner_off.x, flower_mask.y ? 0.0f.xxx.y : corner_off.y, flower_mask.z ? 0.0f.xxx.z : corner_off.z) + anchor_up;
        // 3D value-noise(perlin3)水平位移:8 角 Wang 哈希 -> 平滑插值 -> [-0.5,0.5]。
        float3 local_noise_pos = local_pos * 0.4000000059604644775390625f;
        int3 noise_cell = int3(floor(local_noise_pos));
        float3 noise_frac = frac(local_noise_pos);
        uint3 noise_cell_u = uint3(noise_cell);
        uint v451 = noise_cell_u.z;
        uint v453 = v451 + (v451 << 10u);
        uint v455 = v453 ^ (v453 >> 6u);
        uint v457 = v455 + (v455 << 3u);
        uint v459 = v457 ^ (v457 >> 11u);
        bool v462 = over_path_c ? over_path_c : true;
        uint v463 = over_path_c ? (over_path ? 1664525u : hash_default_u) : 1664525u;
        uint v464 = over_path_c ? (over_path ? 1013904223u : hash_default_u) : 1013904223u;
        uint v468 = (((noise_cell_u.y * v463) + (v459 + (v459 << 15u))) + v464) * v463;
        uint v470 = v468 ^ (v468 >> 11u);
        uint v473 = v470 ^ ((v470 << 7u) & 2636928640u);
        uint v476 = v473 ^ ((v473 << 15u) & 4022730752u);
        bool v479 = v462 ? v462 : true;
        uint v480 = v462 ? v463 : 1664525u;
        uint v481 = v462 ? v464 : 1013904223u;
        uint v485 = (((noise_cell_u.x * v480) + (v476 ^ (v476 >> 18u))) + v481) * v480;
        uint v487 = v485 ^ (v485 >> 11u);
        uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
        uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
        uint v495 = v493 ^ (v493 >> 18u);
        uint v497 = v495 + (v495 << 10u);
        uint v499 = v497 ^ (v497 >> 6u);
        uint v501 = v499 + (v499 << 3u);
        uint v503 = v501 ^ (v501 >> 11u);
        uint v506 = over_path_c ? (over_path ? 8388607u : hash_default_u) : 8388607u;
        uint v507 = over_path_c ? (over_path ? 1065353216u : hash_default_u) : 1065353216u;
        uint v512 = v462 ? v506 : 8388607u;
        uint v513 = v462 ? v507 : 1065353216u;
        uint3 v520 = uint3(noise_cell + int3(1, 0, 0));
        uint v523 = v520.z;
        uint v525 = v523 + (v523 << 10u);
        uint v527 = v525 ^ (v525 >> 6u);
        uint v529 = v527 + (v527 << 3u);
        uint v531 = v529 ^ (v529 >> 11u);
        bool v534 = v479 ? v479 : true;
        uint v535 = v479 ? v480 : 1664525u;
        uint v536 = v479 ? v481 : 1013904223u;
        uint v540 = (((v520.y * v535) + (v531 + (v531 << 15u))) + v536) * v535;
        uint v542 = v540 ^ (v540 >> 11u);
        uint v545 = v542 ^ ((v542 << 7u) & 2636928640u);
        uint v548 = v545 ^ ((v545 << 15u) & 4022730752u);
        bool v551 = v534 ? v534 : true;
        uint v552 = v534 ? v535 : 1664525u;
        uint v553 = v534 ? v536 : 1013904223u;
        uint v557 = (((v520.x * v552) + (v548 ^ (v548 >> 18u))) + v553) * v552;
        uint v559 = v557 ^ (v557 >> 11u);
        uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
        uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
        uint v567 = v565 ^ (v565 >> 18u);
        uint v569 = v567 + (v567 << 10u);
        uint v571 = v569 ^ (v569 >> 6u);
        uint v573 = v571 + (v571 << 3u);
        uint v575 = v573 ^ (v573 >> 11u);
        uint v578 = v479 ? v512 : 8388607u;
        uint v579 = v479 ? v513 : 1065353216u;
        uint v584 = v534 ? v578 : 8388607u;
        uint v585 = v534 ? v579 : 1065353216u;
        uint3 v592 = uint3(noise_cell + int3(0, 1, 0));
        uint v595 = v592.z;
        uint v597 = v595 + (v595 << 10u);
        uint v599 = v597 ^ (v597 >> 6u);
        uint v601 = v599 + (v599 << 3u);
        uint v603 = v601 ^ (v601 >> 11u);
        bool v606 = v551 ? v551 : true;
        uint v607 = v551 ? v552 : 1664525u;
        uint v608 = v551 ? v553 : 1013904223u;
        uint v612 = (((v592.y * v607) + (v603 + (v603 << 15u))) + v608) * v607;
        uint v614 = v612 ^ (v612 >> 11u);
        uint v617 = v614 ^ ((v614 << 7u) & 2636928640u);
        uint v620 = v617 ^ ((v617 << 15u) & 4022730752u);
        bool v623 = v606 ? v606 : true;
        uint v624 = v606 ? v607 : 1664525u;
        uint v625 = v606 ? v608 : 1013904223u;
        uint v629 = (((v592.x * v624) + (v620 ^ (v620 >> 18u))) + v625) * v624;
        uint v631 = v629 ^ (v629 >> 11u);
        uint v634 = v631 ^ ((v631 << 7u) & 2636928640u);
        uint v637 = v634 ^ ((v634 << 15u) & 4022730752u);
        uint v639 = v637 ^ (v637 >> 18u);
        uint v641 = v639 + (v639 << 10u);
        uint v643 = v641 ^ (v641 >> 6u);
        uint v645 = v643 + (v643 << 3u);
        uint v647 = v645 ^ (v645 >> 11u);
        uint v650 = v551 ? v584 : 8388607u;
        uint v651 = v551 ? v585 : 1065353216u;
        uint v656 = v606 ? v650 : 8388607u;
        uint v657 = v606 ? v651 : 1065353216u;
        uint3 v664 = uint3(noise_cell + int3(1, 1, 0));
        uint v667 = v664.z;
        uint v669 = v667 + (v667 << 10u);
        uint v671 = v669 ^ (v669 >> 6u);
        uint v673 = v671 + (v671 << 3u);
        uint v675 = v673 ^ (v673 >> 11u);
        bool v678 = v623 ? v623 : true;
        uint v679 = v623 ? v624 : 1664525u;
        uint v680 = v623 ? v625 : 1013904223u;
        uint v684 = (((v664.y * v679) + (v675 + (v675 << 15u))) + v680) * v679;
        uint v686 = v684 ^ (v684 >> 11u);
        uint v689 = v686 ^ ((v686 << 7u) & 2636928640u);
        uint v692 = v689 ^ ((v689 << 15u) & 4022730752u);
        bool v695 = v678 ? v678 : true;
        uint v696 = v678 ? v679 : 1664525u;
        uint v697 = v678 ? v680 : 1013904223u;
        uint v701 = (((v664.x * v696) + (v692 ^ (v692 >> 18u))) + v697) * v696;
        uint v703 = v701 ^ (v701 >> 11u);
        uint v706 = v703 ^ ((v703 << 7u) & 2636928640u);
        uint v709 = v706 ^ ((v706 << 15u) & 4022730752u);
        uint v711 = v709 ^ (v709 >> 18u);
        uint v713 = v711 + (v711 << 10u);
        uint v715 = v713 ^ (v713 >> 6u);
        uint v717 = v715 + (v715 << 3u);
        uint v719 = v717 ^ (v717 >> 11u);
        uint v722 = v623 ? v656 : 8388607u;
        uint v723 = v623 ? v657 : 1065353216u;
        uint v728 = v678 ? v722 : 8388607u;
        uint v729 = v678 ? v723 : 1065353216u;
        uint3 v736 = uint3(noise_cell + int3(0, 0, 1));
        uint v739 = v736.z;
        uint v741 = v739 + (v739 << 10u);
        uint v743 = v741 ^ (v741 >> 6u);
        uint v745 = v743 + (v743 << 3u);
        uint v747 = v745 ^ (v745 >> 11u);
        bool v750 = v695 ? v695 : true;
        uint v751 = v695 ? v696 : 1664525u;
        uint v752 = v695 ? v697 : 1013904223u;
        uint v756 = (((v736.y * v751) + (v747 + (v747 << 15u))) + v752) * v751;
        uint v758 = v756 ^ (v756 >> 11u);
        uint v761 = v758 ^ ((v758 << 7u) & 2636928640u);
        uint v764 = v761 ^ ((v761 << 15u) & 4022730752u);
        bool v767 = v750 ? v750 : true;
        uint v768 = v750 ? v751 : 1664525u;
        uint v769 = v750 ? v752 : 1013904223u;
        uint v773 = (((v736.x * v768) + (v764 ^ (v764 >> 18u))) + v769) * v768;
        uint v775 = v773 ^ (v773 >> 11u);
        uint v778 = v775 ^ ((v775 << 7u) & 2636928640u);
        uint v781 = v778 ^ ((v778 << 15u) & 4022730752u);
        uint v783 = v781 ^ (v781 >> 18u);
        uint v785 = v783 + (v783 << 10u);
        uint v787 = v785 ^ (v785 >> 6u);
        uint v789 = v787 + (v787 << 3u);
        uint v791 = v789 ^ (v789 >> 11u);
        uint v794 = v695 ? v728 : 8388607u;
        uint v795 = v695 ? v729 : 1065353216u;
        uint v800 = v750 ? v794 : 8388607u;
        uint v801 = v750 ? v795 : 1065353216u;
        uint3 v808 = uint3(noise_cell + int3(1, 0, 1));
        uint v811 = v808.z;
        uint v813 = v811 + (v811 << 10u);
        uint v815 = v813 ^ (v813 >> 6u);
        uint v817 = v815 + (v815 << 3u);
        uint v819 = v817 ^ (v817 >> 11u);
        bool v822 = v767 ? v767 : true;
        uint v823 = v767 ? v768 : 1664525u;
        uint v824 = v767 ? v769 : 1013904223u;
        uint v828 = (((v808.y * v823) + (v819 + (v819 << 15u))) + v824) * v823;
        uint v830 = v828 ^ (v828 >> 11u);
        uint v833 = v830 ^ ((v830 << 7u) & 2636928640u);
        uint v836 = v833 ^ ((v833 << 15u) & 4022730752u);
        bool v839 = v822 ? v822 : true;
        uint v840 = v822 ? v823 : 1664525u;
        uint v841 = v822 ? v824 : 1013904223u;
        uint v845 = (((v808.x * v840) + (v836 ^ (v836 >> 18u))) + v841) * v840;
        uint v847 = v845 ^ (v845 >> 11u);
        uint v850 = v847 ^ ((v847 << 7u) & 2636928640u);
        uint v853 = v850 ^ ((v850 << 15u) & 4022730752u);
        uint v855 = v853 ^ (v853 >> 18u);
        uint v857 = v855 + (v855 << 10u);
        uint v859 = v857 ^ (v857 >> 6u);
        uint v861 = v859 + (v859 << 3u);
        uint v863 = v861 ^ (v861 >> 11u);
        uint v866 = v767 ? v800 : 8388607u;
        uint v867 = v767 ? v801 : 1065353216u;
        uint v872 = v822 ? v866 : 8388607u;
        uint v873 = v822 ? v867 : 1065353216u;
        uint3 v880 = uint3(noise_cell + int3(0, 1, 1));
        uint v883 = v880.z;
        uint v885 = v883 + (v883 << 10u);
        uint v887 = v885 ^ (v885 >> 6u);
        uint v889 = v887 + (v887 << 3u);
        uint v891 = v889 ^ (v889 >> 11u);
        bool v894 = v839 ? v839 : true;
        uint v895 = v839 ? v840 : 1664525u;
        uint v896 = v839 ? v841 : 1013904223u;
        uint v900 = (((v880.y * v895) + (v891 + (v891 << 15u))) + v896) * v895;
        uint v902 = v900 ^ (v900 >> 11u);
        uint v905 = v902 ^ ((v902 << 7u) & 2636928640u);
        uint v908 = v905 ^ ((v905 << 15u) & 4022730752u);
        bool v911 = v894 ? v894 : true;
        uint v912 = v894 ? v895 : 1664525u;
        uint v913 = v894 ? v896 : 1013904223u;
        uint v917 = (((v880.x * v912) + (v908 ^ (v908 >> 18u))) + v913) * v912;
        uint v919 = v917 ^ (v917 >> 11u);
        uint v922 = v919 ^ ((v919 << 7u) & 2636928640u);
        uint v925 = v922 ^ ((v922 << 15u) & 4022730752u);
        uint v927 = v925 ^ (v925 >> 18u);
        uint v929 = v927 + (v927 << 10u);
        uint v931 = v929 ^ (v929 >> 6u);
        uint v933 = v931 + (v931 << 3u);
        uint v935 = v933 ^ (v933 >> 11u);
        uint v938 = v839 ? v872 : 8388607u;
        uint v939 = v839 ? v873 : 1065353216u;
        uint v944 = v894 ? v938 : 8388607u;
        uint v945 = v894 ? v939 : 1065353216u;
        uint3 v952 = uint3(noise_cell + int3(1, 1, 1));
        uint v955 = v952.z;
        uint v957 = v955 + (v955 << 10u);
        uint v959 = v957 ^ (v957 >> 6u);
        uint v961 = v959 + (v959 << 3u);
        uint v963 = v961 ^ (v961 >> 11u);
        bool v966 = v911 ? v911 : true;
        uint v967 = v911 ? v912 : 1664525u;
        uint v968 = v911 ? v913 : 1013904223u;
        uint v972 = (((v952.y * v967) + (v963 + (v963 << 15u))) + v968) * v967;
        uint v974 = v972 ^ (v972 >> 11u);
        uint v977 = v974 ^ ((v974 << 7u) & 2636928640u);
        uint v980 = v977 ^ ((v977 << 15u) & 4022730752u);
        uint v983 = v966 ? v967 : 1664525u;
        uint v988 = (((v952.x * v983) + (v980 ^ (v980 >> 18u))) + (v966 ? v968 : 1013904223u)) * v983;
        uint v990 = v988 ^ (v988 >> 11u);
        uint v993 = v990 ^ ((v990 << 7u) & 2636928640u);
        uint v996 = v993 ^ ((v993 << 15u) & 4022730752u);
        uint v998 = v996 ^ (v996 >> 18u);
        uint v1000 = v998 + (v998 << 10u);
        uint v1002 = v1000 ^ (v1000 >> 6u);
        uint v1004 = v1002 + (v1002 << 3u);
        uint v1006 = v1004 ^ (v1004 >> 11u);
        uint v1009 = v911 ? v944 : 8388607u;
        uint v1010 = v911 ? v945 : 1065353216u;
        bool v1015 = v966 ? v966 : true;
        uint v1016 = v966 ? v1009 : 8388607u;
        uint v1017 = v966 ? v1010 : 1065353216u;
        float3 noise_smooth = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
        float nx = noise_smooth.x;
        float om_nx = 1.0f - nx;
        float ny = noise_smooth.y;
        float om_ny = 1.0f - ny;
        float nz = noise_smooth.z;
        float om_nz = 1.0f - nz;
        float2 noise_sum0 = (((((float2(asfloat((v495 & v506) | v507) - 1.0f, asfloat(((v503 + (v503 << 15u)) & v512) | v513) - 1.0f) * om_nx) * om_ny) * om_nz) + (((float2(asfloat((v567 & v578) | v579) - 1.0f, asfloat(((v575 + (v575 << 15u)) & v584) | v585) - 1.0f) * nx) * om_ny) * om_nz)) + (((float2(asfloat((v639 & v650) | v651) - 1.0f, asfloat(((v647 + (v647 << 15u)) & v656) | v657) - 1.0f) * om_nx) * ny) * om_nz)) + (((float2(asfloat((v711 & v722) | v723) - 1.0f, asfloat(((v719 + (v719 << 15u)) & v728) | v729) - 1.0f) * nx) * ny) * om_nz);
        float2 noise_sum1 = (((noise_sum0 + (((float2(asfloat((v783 & v794) | v795) - 1.0f, asfloat(((v791 + (v791 << 15u)) & v800) | v801) - 1.0f) * om_nx) * om_ny) * nz)) + (((float2(asfloat((v855 & v866) | v867) - 1.0f, asfloat(((v863 + (v863 << 15u)) & v872) | v873) - 1.0f) * nx) * om_ny) * nz)) + (((float2(asfloat((v927 & v938) | v939) - 1.0f, asfloat(((v935 + (v935 << 15u)) & v944) | v945) - 1.0f) * om_nx) * ny) * nz)) + (((float2(asfloat((v998 & v1009) | v1010) - 1.0f, asfloat(((v1006 + (v1006 << 15u)) & v1016) | v1017) - 1.0f) * nx) * ny) * nz);
        float2 disp = clamp((noise_sum1 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float3 world_pos3 = local_pos + (float3(disp.x, 0.0f, disp.y) * 0.0599999986588954925537109375f);
        uint uv_seed_a = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 2342304);
        uint v1094 = uv_seed_a + (uv_seed_a << 10u);
        uint v1096 = v1094 ^ (v1094 >> 6u);
        uint v1098 = v1096 + (v1096 << 3u);
        uint v1100 = v1098 ^ (v1098 >> 11u);
        bool v1103 = v1015 ? v1015 : true;
        uint v1104 = v1015 ? v1016 : 8388607u;
        uint v1105 = v1015 ? v1017 : 1065353216u;
        uint uv_seed_b = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 9344565);
        uint v1114 = uv_seed_b + (uv_seed_b << 10u);
        uint v1116 = v1114 ^ (v1114 >> 6u);
        uint v1118 = v1116 + (v1116 << 3u);
        uint v1120 = v1118 ^ (v1118 >> 11u);
        // 世界->裁剪变换;输出世界位置、旋转后的法线,UV 按每叶种子抖动。
        out_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos3, 1.0f));
        out_world = world_pos3;
        out_normal = mul(rot, float4(in_var_Vertex_Normal, 0.0f)).xyz;
        out_uv = (in_var_Vertex_UV * (is_flower_flag ? 0.300000011920928955078125f : 0.100000001490116119384765625f)) + float2(asfloat(((v1100 + (v1100 << 15u)) & v1104) | v1105) - 1.0f, asfloat(((v1120 + (v1120 << 15u)) & (v1103 ? v1104 : 8388607u)) | (v1103 ? v1105 : 1065353216u)) - 1.0f);
        break;
    } while(false);
    gl_Position = out_pos;
    out_var_C1 = out_world;
    out_var_C2 = out_normal;
    out_var_C4 = out_uv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译段(f0ad 变体 PS):spirv-cross,行为精确;与上方手写段算法一致。
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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};


static float4 gl_FragCoord;
struct SPIRV_Cross_Input
{
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    // 非阴影渲染时做棋盘剔除(像素奇偶与帧号匹配才保留),减少叶片开销。
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 pixel_parity = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (pixel_parity.x + (pixel_parity.y * 2u)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
