// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== Instanced ivy branches (实例常春藤枝条)
// 中文说明:
//   每个 IvySegment 以四边形条带实例化;顶点沿段定位(起/终点抬高到地形高度),
//   按段厚度缩放宽度,并用 3D value-noise(arch noise)按路径掩码淡出做位移。
//   落在路径掩码淡出形状内的顶点通过发射 NaN 位置剔除。片元把打包的世界
//   法线与 sqrt 后的枝条颜色写到 uint2 目标。
//   两个发行变体 b903e43ffb3da915(全)/f0adff76827e73ff(精简),各自的
//   VS/PS 主体为 spirv-cross 转译(行为精确,内部临时变量为 SPIR-V 编号)。

// ==RECONSTRUCTED== Instanced ivy branches. Each IvySegment is instanced as a quad strip; the
// SOURCE PATH : shaders/ivy/instanced_ivy_branch.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _ivy_instanced_ivy_branch.raster.hlsl_b903e43ffb3da915.bin
//     file_sha256  : a5b102c2f95b0270b88f4731273a29b1fc35f053d1114819e1ad5573ab8444d3
//     pass/stage   : _ivy_instanced_ivy_branch (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : b903e43ffb3da915
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, path_mask, vs_main, sampler_llr, ivy_branch_color
//   _ivy_instanced_ivy_branch.raster.hlsl_f0adff76827e73ff.bin
//     file_sha256  : 1648414082036e0da543cb7944b2a10f2fb90a18a6b0965f9f95e074400bfc61
//     pass/stage   : _ivy_instanced_ivy_branch (vegetation_grass) / raster
//     entry        : vs:vs_main; ps:ps_main
//     variant      : f0adff76827e73ff
//     local_size   : -
//     names        : sampler_llb, sampler_default, ViewConstants, frame_constants, $Globals, IvySegment, ivy_buffer, terrain_heightmap, path_mask, vs_main, ps_main
// ==RECONSTRUCTED==
// Instanced ivy branches. Each IvySegment is instanced as a quad strip; the
// vertex is positioned along the segment (start/end lifted to terrain height),
// width-scaled by the segment thickness, and displaced by 3D value noise
// (arch noise) scaled by a path-mask fade. Vertices inside the path-mask fade
// shape are culled by emitting a NaN position. The fragment writes the packed
// world normal and a sqrt'd branch color to a uint2 target.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "arch_noise_discard.inc.hlsl"

// 复现原版共享 13 采样器 $Globals 布局(set0/b13)。

// 每着色器 $Globals 成员(隐式 $Globals UBO,set0/b13)。
float4x4 model_to_world;
uint2 ivy_branch_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
    float arch_height;
};

StructuredBuffer<IvySegment> ivy_buffer : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> ivy_branch_color : register(t15, space0);

struct VSInput
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 normal;
    float2 uv;
    float3 world_pos;
    switch (0u)
    {
    default:
        float3 start = ivy_buffer[input.instance].start_pos;
        float3 end = ivy_buffer[input.instance].end_pos;
        float2 start_uv = start.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hstart = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, start_uv, 0.0f).x);
        float2 end_uv = end.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hend = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, end_uv, 0.0f).x);
        float start_thick = ivy_buffer[input.instance].start_thickness;
        float end_thick = ivy_buffer[input.instance].end_thickness;
        float3 wall_normal = ivy_buffer[input.instance].wall_normal;
        float arch_height = ivy_buffer[input.instance].arch_height;
        float base = arch_height - min(hstart, 0.0f);
        float path = path_mask.SampleLevel(sampler_llb, start_uv, 0.0f).x;
        bool over_path = path > 0.1f;

        float t0, t1;
        if (over_path)
        {
            float fade = (1.0f - exp(-5.0f * path)) * base;
            float n = perlin(start.xz * 0.6f + start.y * 0.6f);
            float t = clamp(base - start.y, 0.0f, 1.0f);
            float tt = t * t;
            float d = fade * (tt * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
            if (start.y < d)
            {
                position = asfloat(0x7fbfffffu).xxxx;
                normal = 0.0f.xxx;
                uv = 0.0f.xx;
                world_pos = 0.0f.xxx;
                break;
            }
            else if (start.y < fade)
            {
                float k = 1.0f - tt;
                t0 = min(start_thick * k, 0.2f);
                t1 = min(end_thick * k, 0.2f);
            }
            else
            {
                t0 = start_thick;
                t1 = end_thick;
            }
        }
        else
        {
            t0 = start_thick;
            t1 = end_thick;
        }

        float3 s = start;
        s.y = start.y + hstart;
        float3 e = end;
        e.y = end.y + hend;
        float3 dir = normalize(e - s);
        float3 side = normalize(cross(dir, wall_normal));
        float3 vpos;
        if (input.position.y < 0.1f)
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t0);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        else
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t1);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        float4x4 rot = transpose(float4x4(float4(side, 0.0f), float4(dir, 0.0f),
                                          float4(normalize(cross(side, dir)), 0.0f),
                                          float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, start.x),
                                  float4(0.0f, 1.0f, 0.0f, s.y),
                                  float4(0.0f, 0.0f, 1.0f, start.z),
                                  float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 local = mul(trans, mul(rot, float4(vpos.x, vpos.y * (distance(s, e) * 1.1f), vpos.z, 1.0f))).xyz;
        uint alt_seed;
        float2 disp = clamp((perlin3(local * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
        float3 world = mul(model_to_world, float4(local + (float3(disp.x, 0.0f, disp.y) * 0.06f), 1.0f)).xyz;
        position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
        normal = mul(rot, float4(normalize(float3(input.position.x, 0.0f, input.position.z)), 0.0f)).xyz;
        uv = float2(end.x + e.y * 0.5f, end.z + e.y * 0.5f) * 0.2f;
        world_pos = world;
        break;
    }
    output.position = position;
    output.normal = normal;
    output.uv = uv;
    output.world_pos = world_pos;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    float3 color = ivy_branch_color.Sample(sampler_llr, input.uv).xyz;
    float3 c;
    if (glade_theme_id == 4u)
    {
        float fade = smoothstep(-0.1f, 0.0f, input.world_pos.y) * smoothstep(0.0f, 0.7f, clamp(input.normal.y, 0.0f, 1.0f));
        c = lerp(lerp(color, 0.7f.xxx, 0.2f.xxx), 0.7f.xxx, fade.xxx);
    }
    else
    {
        c = color;
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(c, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 436207616u;
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

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
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
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_branch_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
};

ByteAddressBuffer ivy_buffer : register(t14, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float2 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 out_pos;
    float3 out_normal;
    float2 out_uv;
    float3 out_world;
    do
    {
        // 读取当前 IvySegment 的起/终点;起点采样地形高度抬高,并采样路径掩码。
        float3 start = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float3 end = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
        float2 start_uv = (start.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float hstart = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, start_uv, 0.0f).x);
        float base = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(hstart, 0.0f);
        float4 path_smp = path_mask.SampleLevel(sampler_llb, start_uv, 0.0f);
        float path = path_smp.x;
        bool over_path = path > 0.100000001490116119384765625f;
        float t0;
        float t1;
        // 路径掩码淡出:路径上枝条的宽度按爬升高度收敛;完全落入拱形噪声淡出内的
        // 顶点发射 NaN 位置(被 GPU 剔除)。threshold 由 perlin 2D + 爬升曲线决定。
        if (over_path)
        {
            float fade = (1.0f - exp((-5.0f) * path)) * base;
            float2 perlin_pos = (float2(start.xz) * 0.60000002384185791015625f) + (start.y * 0.60000002384185791015625f).xx;
            int2 perlin_cell = int2(floor(perlin_pos));
            float2 perlin_frac = frac(perlin_pos);
            uint2 cell_u = uint2(perlin_cell);
            uint v211 = cell_u.y;
            uint v213 = v211 + (v211 << 10u);
            uint v215 = v213 ^ (v213 >> 6u);
            uint v217 = v215 + (v215 << 3u);
            uint v219 = v217 ^ (v217 >> 11u);
            uint v225 = (((cell_u.x * 1664525u) + (v219 + (v219 << 15u))) + 1013904223u) * 1664525u;
            uint v227 = v225 ^ (v225 >> 11u);
            uint v230 = v227 ^ ((v227 << 7u) & 2636928640u);
            uint v233 = v230 ^ ((v230 << 15u) & 4022730752u);
            uint2 v241 = uint2(perlin_cell + int2(1, 0));
            uint v243 = v241.y;
            uint v245 = v243 + (v243 << 10u);
            uint v247 = v245 ^ (v245 >> 6u);
            uint v249 = v247 + (v247 << 3u);
            uint v251 = v249 ^ (v249 >> 11u);
            uint v257 = (((v241.x * 1664525u) + (v251 + (v251 << 15u))) + 1013904223u) * 1664525u;
            uint v259 = v257 ^ (v257 >> 11u);
            uint v262 = v259 ^ ((v259 << 7u) & 2636928640u);
            uint v265 = v262 ^ ((v262 << 15u) & 4022730752u);
            uint2 v273 = uint2(perlin_cell + int2(0, 1));
            uint v275 = v273.y;
            uint v277 = v275 + (v275 << 10u);
            uint v279 = v277 ^ (v277 >> 6u);
            uint v281 = v279 + (v279 << 3u);
            uint v283 = v281 ^ (v281 >> 11u);
            uint v289 = (((v273.x * 1664525u) + (v283 + (v283 << 15u))) + 1013904223u) * 1664525u;
            uint v291 = v289 ^ (v289 >> 11u);
            uint v294 = v291 ^ ((v291 << 7u) & 2636928640u);
            uint v297 = v294 ^ ((v294 << 15u) & 4022730752u);
            uint2 v305 = uint2(perlin_cell + int2(1, 1));
            uint v307 = v305.y;
            uint v309 = v307 + (v307 << 10u);
            uint v311 = v309 ^ (v309 >> 6u);
            uint v313 = v311 + (v311 << 3u);
            uint v315 = v313 ^ (v313 >> 11u);
            uint v321 = (((v305.x * 1664525u) + (v315 + (v315 << 15u))) + 1013904223u) * 1664525u;
            uint v323 = v321 ^ (v321 >> 11u);
            uint v326 = v323 ^ ((v323 << 7u) & 2636928640u);
            uint v329 = v326 ^ ((v326 << 15u) & 4022730752u);
            float2 n_smooth = (perlin_frac * perlin_frac) * (3.0f.xx - (perlin_frac * 2.0f));
            float n_smooth_x = n_smooth.x;
            float om_n_smooth_x = 1.0f - n_smooth_x;
            float n_smooth_y = n_smooth.y;
            float climb_t = clamp(base - start.y, 0.0f, 1.0f);
            float climb_tt = climb_t * climb_t;
            float t0_path;
            float t1_path;
            if (start.y < (fade * (climb_tt * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - n_smooth_y) * (((asfloat(((v233 ^ (v233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) + ((asfloat(((v265 ^ (v265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x))) + (((asfloat(((v297 ^ (v297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) * n_smooth_y)) + (((asfloat(((v329 ^ (v329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x) * n_smooth_y)), 0.800000011920928955078125f, 1.0f))))
            {
                out_pos = asfloat(0x7fbfffffu /* nan */).xxxx;
                out_normal = 0.0f.xxx;
                out_uv = 0.0f.xx;
                out_world = 0.0f.xxx;
                break;
            }
            else
            {
                if (start.y < fade)
                {
                    float climb_k = 1.0f - climb_tt;
                    t0_path = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12) * climb_k, 0.20000000298023223876953125f);
                    t1_path = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28) * climb_k, 0.20000000298023223876953125f);
                }
                else
                {
                    t0_path = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
                    t1_path = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
                }
            }
            t0 = t0_path;
            t1 = t1_path;
        }
        else
        {
            t0 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
            t1 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
        }
        bool over_path_c = over_path ? true : false;
        float start_y_up = start.y + hstart;
        float3 start_up = start;
        start_up.y = start_y_up;
        float end_y_up = end.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (end.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float3 end_up = end;
        end_up.y = end_y_up;
        float3 dir = normalize(end_up - start_up);
        float3 side = normalize(cross(dir, ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32)));
        float3 vpos;
        if (in_var_Vertex_Position.y < 0.100000001490116119384765625f)
        {
            float2 sc_s = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t0);
            vpos = float3(sc_s.x, in_var_Vertex_Position.y, sc_s.y);
        }
        else
        {
            float2 sc_e = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t1);
            vpos = float3(sc_e.x, in_var_Vertex_Position.y, sc_e.y);
        }
        float4x4 rot = transpose(float4x4(float4(side, 0.0f), float4(dir, 0.0f), float4(normalize(cross(side, dir)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 local = mul(float4x4(float4(1.0f, 0.0f, 0.0f, start.x), float4(0.0f, 1.0f, 0.0f, start_y_up), float4(0.0f, 0.0f, 1.0f, start.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(rot, float4(vpos.x, vpos.y * (distance(start_up, end_up) * 1.10000002384185791015625f), vpos.z, 1.0f))).xyz;
        // 3D value-noise(perlin3)位移:8 角 Wang 哈希 -> 平滑插值 -> 映射到 [-0.5,0.5],
        // 只在水平面 (x,z) 方向施加 0.06 倍位移,让枝条沿墙面起伏。
        float3 local_noise_pos = local * 0.4000000059604644775390625f;
        int3 noise_cell = int3(floor(local_noise_pos));
        float3 noise_frac = frac(local_noise_pos);
        uint3 noise_cell_u = uint3(noise_cell);
        uint v449 = noise_cell_u.z;
        uint v451 = v449 + (v449 << 10u);
        uint v453 = v451 ^ (v451 >> 6u);
        uint v455 = v453 + (v453 << 3u);
        uint v457 = v455 ^ (v455 >> 11u);
        bool v460 = over_path_c ? over_path_c : true;
        uint v461 = over_path_c ? (over_path ? 1664525u : hash_default_u) : 1664525u;
        uint v462 = over_path_c ? (over_path ? 1013904223u : hash_default_u) : 1013904223u;
        uint v466 = (((noise_cell_u.y * v461) + (v457 + (v457 << 15u))) + v462) * v461;
        uint v468 = v466 ^ (v466 >> 11u);
        uint v471 = v468 ^ ((v468 << 7u) & 2636928640u);
        uint v474 = v471 ^ ((v471 << 15u) & 4022730752u);
        bool v477 = v460 ? v460 : true;
        uint v478 = v460 ? v461 : 1664525u;
        uint v479 = v460 ? v462 : 1013904223u;
        uint v483 = (((noise_cell_u.x * v478) + (v474 ^ (v474 >> 18u))) + v479) * v478;
        uint v485 = v483 ^ (v483 >> 11u);
        uint v488 = v485 ^ ((v485 << 7u) & 2636928640u);
        uint v491 = v488 ^ ((v488 << 15u) & 4022730752u);
        uint v493 = v491 ^ (v491 >> 18u);
        uint v495 = v493 + (v493 << 10u);
        uint v497 = v495 ^ (v495 >> 6u);
        uint v499 = v497 + (v497 << 3u);
        uint v501 = v499 ^ (v499 >> 11u);
        uint v504 = over_path_c ? (over_path ? 8388607u : hash_default_u) : 8388607u;
        uint v505 = over_path_c ? (over_path ? 1065353216u : hash_default_u) : 1065353216u;
        uint v510 = v460 ? v504 : 8388607u;
        uint v511 = v460 ? v505 : 1065353216u;
        uint3 v518 = uint3(noise_cell + int3(1, 0, 0));
        uint v521 = v518.z;
        uint v523 = v521 + (v521 << 10u);
        uint v525 = v523 ^ (v523 >> 6u);
        uint v527 = v525 + (v525 << 3u);
        uint v529 = v527 ^ (v527 >> 11u);
        bool v532 = v477 ? v477 : true;
        uint v533 = v477 ? v478 : 1664525u;
        uint v534 = v477 ? v479 : 1013904223u;
        uint v538 = (((v518.y * v533) + (v529 + (v529 << 15u))) + v534) * v533;
        uint v540 = v538 ^ (v538 >> 11u);
        uint v543 = v540 ^ ((v540 << 7u) & 2636928640u);
        uint v546 = v543 ^ ((v543 << 15u) & 4022730752u);
        bool v549 = v532 ? v532 : true;
        uint v550 = v532 ? v533 : 1664525u;
        uint v551 = v532 ? v534 : 1013904223u;
        uint v555 = (((v518.x * v550) + (v546 ^ (v546 >> 18u))) + v551) * v550;
        uint v557 = v555 ^ (v555 >> 11u);
        uint v560 = v557 ^ ((v557 << 7u) & 2636928640u);
        uint v563 = v560 ^ ((v560 << 15u) & 4022730752u);
        uint v565 = v563 ^ (v563 >> 18u);
        uint v567 = v565 + (v565 << 10u);
        uint v569 = v567 ^ (v567 >> 6u);
        uint v571 = v569 + (v569 << 3u);
        uint v573 = v571 ^ (v571 >> 11u);
        uint v576 = v477 ? v510 : 8388607u;
        uint v577 = v477 ? v511 : 1065353216u;
        uint v582 = v532 ? v576 : 8388607u;
        uint v583 = v532 ? v577 : 1065353216u;
        uint3 v590 = uint3(noise_cell + int3(0, 1, 0));
        uint v593 = v590.z;
        uint v595 = v593 + (v593 << 10u);
        uint v597 = v595 ^ (v595 >> 6u);
        uint v599 = v597 + (v597 << 3u);
        uint v601 = v599 ^ (v599 >> 11u);
        bool v604 = v549 ? v549 : true;
        uint v605 = v549 ? v550 : 1664525u;
        uint v606 = v549 ? v551 : 1013904223u;
        uint v610 = (((v590.y * v605) + (v601 + (v601 << 15u))) + v606) * v605;
        uint v612 = v610 ^ (v610 >> 11u);
        uint v615 = v612 ^ ((v612 << 7u) & 2636928640u);
        uint v618 = v615 ^ ((v615 << 15u) & 4022730752u);
        bool v621 = v604 ? v604 : true;
        uint v622 = v604 ? v605 : 1664525u;
        uint v623 = v604 ? v606 : 1013904223u;
        uint v627 = (((v590.x * v622) + (v618 ^ (v618 >> 18u))) + v623) * v622;
        uint v629 = v627 ^ (v627 >> 11u);
        uint v632 = v629 ^ ((v629 << 7u) & 2636928640u);
        uint v635 = v632 ^ ((v632 << 15u) & 4022730752u);
        uint v637 = v635 ^ (v635 >> 18u);
        uint v639 = v637 + (v637 << 10u);
        uint v641 = v639 ^ (v639 >> 6u);
        uint v643 = v641 + (v641 << 3u);
        uint v645 = v643 ^ (v643 >> 11u);
        uint v648 = v549 ? v582 : 8388607u;
        uint v649 = v549 ? v583 : 1065353216u;
        uint v654 = v604 ? v648 : 8388607u;
        uint v655 = v604 ? v649 : 1065353216u;
        uint3 v662 = uint3(noise_cell + int3(1, 1, 0));
        uint v665 = v662.z;
        uint v667 = v665 + (v665 << 10u);
        uint v669 = v667 ^ (v667 >> 6u);
        uint v671 = v669 + (v669 << 3u);
        uint v673 = v671 ^ (v671 >> 11u);
        bool v676 = v621 ? v621 : true;
        uint v677 = v621 ? v622 : 1664525u;
        uint v678 = v621 ? v623 : 1013904223u;
        uint v682 = (((v662.y * v677) + (v673 + (v673 << 15u))) + v678) * v677;
        uint v684 = v682 ^ (v682 >> 11u);
        uint v687 = v684 ^ ((v684 << 7u) & 2636928640u);
        uint v690 = v687 ^ ((v687 << 15u) & 4022730752u);
        bool v693 = v676 ? v676 : true;
        uint v694 = v676 ? v677 : 1664525u;
        uint v695 = v676 ? v678 : 1013904223u;
        uint v699 = (((v662.x * v694) + (v690 ^ (v690 >> 18u))) + v695) * v694;
        uint v701 = v699 ^ (v699 >> 11u);
        uint v704 = v701 ^ ((v701 << 7u) & 2636928640u);
        uint v707 = v704 ^ ((v704 << 15u) & 4022730752u);
        uint v709 = v707 ^ (v707 >> 18u);
        uint v711 = v709 + (v709 << 10u);
        uint v713 = v711 ^ (v711 >> 6u);
        uint v715 = v713 + (v713 << 3u);
        uint v717 = v715 ^ (v715 >> 11u);
        uint v720 = v621 ? v654 : 8388607u;
        uint v721 = v621 ? v655 : 1065353216u;
        uint v726 = v676 ? v720 : 8388607u;
        uint v727 = v676 ? v721 : 1065353216u;
        uint3 v734 = uint3(noise_cell + int3(0, 0, 1));
        uint v737 = v734.z;
        uint v739 = v737 + (v737 << 10u);
        uint v741 = v739 ^ (v739 >> 6u);
        uint v743 = v741 + (v741 << 3u);
        uint v745 = v743 ^ (v743 >> 11u);
        bool v748 = v693 ? v693 : true;
        uint v749 = v693 ? v694 : 1664525u;
        uint v750 = v693 ? v695 : 1013904223u;
        uint v754 = (((v734.y * v749) + (v745 + (v745 << 15u))) + v750) * v749;
        uint v756 = v754 ^ (v754 >> 11u);
        uint v759 = v756 ^ ((v756 << 7u) & 2636928640u);
        uint v762 = v759 ^ ((v759 << 15u) & 4022730752u);
        bool v765 = v748 ? v748 : true;
        uint v766 = v748 ? v749 : 1664525u;
        uint v767 = v748 ? v750 : 1013904223u;
        uint v771 = (((v734.x * v766) + (v762 ^ (v762 >> 18u))) + v767) * v766;
        uint v773 = v771 ^ (v771 >> 11u);
        uint v776 = v773 ^ ((v773 << 7u) & 2636928640u);
        uint v779 = v776 ^ ((v776 << 15u) & 4022730752u);
        uint v781 = v779 ^ (v779 >> 18u);
        uint v783 = v781 + (v781 << 10u);
        uint v785 = v783 ^ (v783 >> 6u);
        uint v787 = v785 + (v785 << 3u);
        uint v789 = v787 ^ (v787 >> 11u);
        uint v792 = v693 ? v726 : 8388607u;
        uint v793 = v693 ? v727 : 1065353216u;
        uint v798 = v748 ? v792 : 8388607u;
        uint v799 = v748 ? v793 : 1065353216u;
        uint3 v806 = uint3(noise_cell + int3(1, 0, 1));
        uint v809 = v806.z;
        uint v811 = v809 + (v809 << 10u);
        uint v813 = v811 ^ (v811 >> 6u);
        uint v815 = v813 + (v813 << 3u);
        uint v817 = v815 ^ (v815 >> 11u);
        bool v820 = v765 ? v765 : true;
        uint v821 = v765 ? v766 : 1664525u;
        uint v822 = v765 ? v767 : 1013904223u;
        uint v826 = (((v806.y * v821) + (v817 + (v817 << 15u))) + v822) * v821;
        uint v828 = v826 ^ (v826 >> 11u);
        uint v831 = v828 ^ ((v828 << 7u) & 2636928640u);
        uint v834 = v831 ^ ((v831 << 15u) & 4022730752u);
        bool v837 = v820 ? v820 : true;
        uint v838 = v820 ? v821 : 1664525u;
        uint v839 = v820 ? v822 : 1013904223u;
        uint v843 = (((v806.x * v838) + (v834 ^ (v834 >> 18u))) + v839) * v838;
        uint v845 = v843 ^ (v843 >> 11u);
        uint v848 = v845 ^ ((v845 << 7u) & 2636928640u);
        uint v851 = v848 ^ ((v848 << 15u) & 4022730752u);
        uint v853 = v851 ^ (v851 >> 18u);
        uint v855 = v853 + (v853 << 10u);
        uint v857 = v855 ^ (v855 >> 6u);
        uint v859 = v857 + (v857 << 3u);
        uint v861 = v859 ^ (v859 >> 11u);
        uint v864 = v765 ? v798 : 8388607u;
        uint v865 = v765 ? v799 : 1065353216u;
        uint v870 = v820 ? v864 : 8388607u;
        uint v871 = v820 ? v865 : 1065353216u;
        uint3 v878 = uint3(noise_cell + int3(0, 1, 1));
        uint v881 = v878.z;
        uint v883 = v881 + (v881 << 10u);
        uint v885 = v883 ^ (v883 >> 6u);
        uint v887 = v885 + (v885 << 3u);
        uint v889 = v887 ^ (v887 >> 11u);
        bool v892 = v837 ? v837 : true;
        uint v893 = v837 ? v838 : 1664525u;
        uint v894 = v837 ? v839 : 1013904223u;
        uint v898 = (((v878.y * v893) + (v889 + (v889 << 15u))) + v894) * v893;
        uint v900 = v898 ^ (v898 >> 11u);
        uint v903 = v900 ^ ((v900 << 7u) & 2636928640u);
        uint v906 = v903 ^ ((v903 << 15u) & 4022730752u);
        bool v909 = v892 ? v892 : true;
        uint v910 = v892 ? v893 : 1664525u;
        uint v911 = v892 ? v894 : 1013904223u;
        uint v915 = (((v878.x * v910) + (v906 ^ (v906 >> 18u))) + v911) * v910;
        uint v917 = v915 ^ (v915 >> 11u);
        uint v920 = v917 ^ ((v917 << 7u) & 2636928640u);
        uint v923 = v920 ^ ((v920 << 15u) & 4022730752u);
        uint v925 = v923 ^ (v923 >> 18u);
        uint v927 = v925 + (v925 << 10u);
        uint v929 = v927 ^ (v927 >> 6u);
        uint v931 = v929 + (v929 << 3u);
        uint v933 = v931 ^ (v931 >> 11u);
        uint v936 = v837 ? v870 : 8388607u;
        uint v937 = v837 ? v871 : 1065353216u;
        uint v942 = v892 ? v936 : 8388607u;
        uint v943 = v892 ? v937 : 1065353216u;
        uint3 v950 = uint3(noise_cell + int3(1, 1, 1));
        uint v953 = v950.z;
        uint v955 = v953 + (v953 << 10u);
        uint v957 = v955 ^ (v955 >> 6u);
        uint v959 = v957 + (v957 << 3u);
        uint v961 = v959 ^ (v959 >> 11u);
        bool v964 = v909 ? v909 : true;
        uint v965 = v909 ? v910 : 1664525u;
        uint v966 = v909 ? v911 : 1013904223u;
        uint v970 = (((v950.y * v965) + (v961 + (v961 << 15u))) + v966) * v965;
        uint v972 = v970 ^ (v970 >> 11u);
        uint v975 = v972 ^ ((v972 << 7u) & 2636928640u);
        uint v978 = v975 ^ ((v975 << 15u) & 4022730752u);
        uint v981 = v964 ? v965 : 1664525u;
        uint v986 = (((v950.x * v981) + (v978 ^ (v978 >> 18u))) + (v964 ? v966 : 1013904223u)) * v981;
        uint v988 = v986 ^ (v986 >> 11u);
        uint v991 = v988 ^ ((v988 << 7u) & 2636928640u);
        uint v994 = v991 ^ ((v991 << 15u) & 4022730752u);
        uint v996 = v994 ^ (v994 >> 18u);
        uint v998 = v996 + (v996 << 10u);
        uint v1000 = v998 ^ (v998 >> 6u);
        uint v1002 = v1000 + (v1000 << 3u);
        uint v1004 = v1002 ^ (v1002 >> 11u);
        uint v1007 = v909 ? v942 : 8388607u;
        uint v1008 = v909 ? v943 : 1065353216u;
        float3 noise_smooth = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
        float nx = noise_smooth.x;
        float om_nx = 1.0f - nx;
        float ny = noise_smooth.y;
        float om_ny = 1.0f - ny;
        float nz = noise_smooth.z;
        float om_nz = 1.0f - nz;
        float2 noise_sum0 = (((((float2(asfloat((v493 & v504) | v505) - 1.0f, asfloat(((v501 + (v501 << 15u)) & v510) | v511) - 1.0f) * om_nx) * om_ny) * om_nz) + (((float2(asfloat((v565 & v576) | v577) - 1.0f, asfloat(((v573 + (v573 << 15u)) & v582) | v583) - 1.0f) * nx) * om_ny) * om_nz)) + (((float2(asfloat((v637 & v648) | v649) - 1.0f, asfloat(((v645 + (v645 << 15u)) & v654) | v655) - 1.0f) * om_nx) * ny) * om_nz)) + (((float2(asfloat((v709 & v720) | v721) - 1.0f, asfloat(((v717 + (v717 << 15u)) & v726) | v727) - 1.0f) * nx) * ny) * om_nz);
        float2 noise_sum1 = (((noise_sum0 + (((float2(asfloat((v781 & v792) | v793) - 1.0f, asfloat(((v789 + (v789 << 15u)) & v798) | v799) - 1.0f) * om_nx) * om_ny) * nz)) + (((float2(asfloat((v853 & v864) | v865) - 1.0f, asfloat(((v861 + (v861 << 15u)) & v870) | v871) - 1.0f) * nx) * om_ny) * nz)) + (((float2(asfloat((v925 & v936) | v937) - 1.0f, asfloat(((v933 + (v933 << 15u)) & v942) | v943) - 1.0f) * om_nx) * ny) * nz)) + (((float2(asfloat((v996 & v1007) | v1008) - 1.0f, asfloat(((v1004 + (v1004 << 15u)) & (v964 ? v1007 : 8388607u)) | (v964 ? v1008 : 1065353216u)) - 1.0f) * nx) * ny) * nz);
        float2 disp = clamp((noise_sum1 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        // 模型->世界->裁剪空间变换;输出旋转后的法线、按段长缩放的 UV 与世界位置。
        float4 world = mul(model_to_world, float4(local + (float3(disp.x, 0.0f, disp.y) * 0.0599999986588954925537109375f), 1.0f));
        float end_y_half = end_y_up * 0.5f;
        out_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, world);
        out_normal = mul(rot, float4(normalize(float3(in_var_Vertex_Position.x, 0.0f, in_var_Vertex_Position.z)), 0.0f)).xyz;
        out_uv = float2(end.x + end_y_half, end.z + end_y_half) * 0.20000000298023223876953125f;
        out_world = world.xyz;
        break;
    } while(false);
    gl_Position = out_pos;
    out_var_C1 = out_normal;
    out_var_C2 = out_uv;
    out_var_C3 = out_world;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译段(b903 变体 PS):spirv-cross,行为精确。
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> ivy_branch_color : register(t15, space0);

static float3 in_var_C1;
static float2 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 采样枝条颜色;glade 主题 4(幽灵?)按世界高度/朝上程度向 0.7 淡出。
    float4 color_smp = ivy_branch_color.Sample(sampler_llr, in_var_C2);
    float3 color = color_smp.xyz;
    float3 c;
    if (frame_constants_glade_theme_id == 4u)
    {
        c = lerp(lerp(color, 0.699999988079071044921875f.xxx, 0.20000000298023223876953125f.xxx), 0.699999988079071044921875f.xxx, (smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y) * smoothstep(0.0f, 0.699999988079071044921875f, clamp(in_var_C1.y, 0.0f, 1.0f))).xxx);
    }
    else
    {
        c = color;
    }
    float3 packed_normal = (in_var_C1 * 0.5f) + 0.5f.xxx;
    // 打包到 uint2:法线 RGB8(alpha 0xff)+ 颜色 sqrt 后 RGB8(alpha 0x1a)。
    float3 sc = sqrt(clamp(c, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(sc.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(sc.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(sc.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 436207616u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 转译段(f0ad 变体 VS):spirv-cross,行为精确。
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

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
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
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_branch_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
};

ByteAddressBuffer ivy_buffer : register(t14, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float2 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 out_pos;
    float3 out_normal;
    float2 out_uv;
    float3 out_world;
    do
    {
        // 读取当前 IvySegment 的起/终点;起点采样地形高度抬高,并采样路径掩码。
        float3 start = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float3 end = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
        float2 start_uv = (start.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float hstart = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, start_uv, 0.0f).x);
        float base = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(hstart, 0.0f);
        float4 path_smp = path_mask.SampleLevel(sampler_llb, start_uv, 0.0f);
        float path = path_smp.x;
        bool over_path = path > 0.100000001490116119384765625f;
        float t0;
        float t1;
        // 路径掩码淡出:路径上枝条的宽度按爬升高度收敛;完全落入拱形噪声淡出内的
        // 顶点发射 NaN 位置(被 GPU 剔除)。threshold 由 perlin 2D + 爬升曲线决定。
        if (over_path)
        {
            float fade = (1.0f - exp((-5.0f) * path)) * base;
            float2 perlin_pos = (float2(start.xz) * 0.60000002384185791015625f) + (start.y * 0.60000002384185791015625f).xx;
            int2 perlin_cell = int2(floor(perlin_pos));
            float2 perlin_frac = frac(perlin_pos);
            uint2 cell_u = uint2(perlin_cell);
            uint v211 = cell_u.y;
            uint v213 = v211 + (v211 << 10u);
            uint v215 = v213 ^ (v213 >> 6u);
            uint v217 = v215 + (v215 << 3u);
            uint v219 = v217 ^ (v217 >> 11u);
            uint v225 = (((cell_u.x * 1664525u) + (v219 + (v219 << 15u))) + 1013904223u) * 1664525u;
            uint v227 = v225 ^ (v225 >> 11u);
            uint v230 = v227 ^ ((v227 << 7u) & 2636928640u);
            uint v233 = v230 ^ ((v230 << 15u) & 4022730752u);
            uint2 v241 = uint2(perlin_cell + int2(1, 0));
            uint v243 = v241.y;
            uint v245 = v243 + (v243 << 10u);
            uint v247 = v245 ^ (v245 >> 6u);
            uint v249 = v247 + (v247 << 3u);
            uint v251 = v249 ^ (v249 >> 11u);
            uint v257 = (((v241.x * 1664525u) + (v251 + (v251 << 15u))) + 1013904223u) * 1664525u;
            uint v259 = v257 ^ (v257 >> 11u);
            uint v262 = v259 ^ ((v259 << 7u) & 2636928640u);
            uint v265 = v262 ^ ((v262 << 15u) & 4022730752u);
            uint2 v273 = uint2(perlin_cell + int2(0, 1));
            uint v275 = v273.y;
            uint v277 = v275 + (v275 << 10u);
            uint v279 = v277 ^ (v277 >> 6u);
            uint v281 = v279 + (v279 << 3u);
            uint v283 = v281 ^ (v281 >> 11u);
            uint v289 = (((v273.x * 1664525u) + (v283 + (v283 << 15u))) + 1013904223u) * 1664525u;
            uint v291 = v289 ^ (v289 >> 11u);
            uint v294 = v291 ^ ((v291 << 7u) & 2636928640u);
            uint v297 = v294 ^ ((v294 << 15u) & 4022730752u);
            uint2 v305 = uint2(perlin_cell + int2(1, 1));
            uint v307 = v305.y;
            uint v309 = v307 + (v307 << 10u);
            uint v311 = v309 ^ (v309 >> 6u);
            uint v313 = v311 + (v311 << 3u);
            uint v315 = v313 ^ (v313 >> 11u);
            uint v321 = (((v305.x * 1664525u) + (v315 + (v315 << 15u))) + 1013904223u) * 1664525u;
            uint v323 = v321 ^ (v321 >> 11u);
            uint v326 = v323 ^ ((v323 << 7u) & 2636928640u);
            uint v329 = v326 ^ ((v326 << 15u) & 4022730752u);
            float2 n_smooth = (perlin_frac * perlin_frac) * (3.0f.xx - (perlin_frac * 2.0f));
            float n_smooth_x = n_smooth.x;
            float om_n_smooth_x = 1.0f - n_smooth_x;
            float n_smooth_y = n_smooth.y;
            float climb_t = clamp(base - start.y, 0.0f, 1.0f);
            float climb_tt = climb_t * climb_t;
            float t0_path;
            float t1_path;
            if (start.y < (fade * (climb_tt * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - n_smooth_y) * (((asfloat(((v233 ^ (v233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) + ((asfloat(((v265 ^ (v265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x))) + (((asfloat(((v297 ^ (v297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * om_n_smooth_x) * n_smooth_y)) + (((asfloat(((v329 ^ (v329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n_smooth_x) * n_smooth_y)), 0.800000011920928955078125f, 1.0f))))
            {
                out_pos = asfloat(0x7fbfffffu /* nan */).xxxx;
                out_normal = 0.0f.xxx;
                out_uv = 0.0f.xx;
                out_world = 0.0f.xxx;
                break;
            }
            else
            {
                if (start.y < fade)
                {
                    float climb_k = 1.0f - climb_tt;
                    t0_path = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12) * climb_k, 0.20000000298023223876953125f);
                    t1_path = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28) * climb_k, 0.20000000298023223876953125f);
                }
                else
                {
                    t0_path = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
                    t1_path = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
                }
            }
            t0 = t0_path;
            t1 = t1_path;
        }
        else
        {
            t0 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
            t1 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
        }
        bool over_path_c = over_path ? true : false;
        float start_y_up = start.y + hstart;
        float3 start_up = start;
        start_up.y = start_y_up;
        float end_y_up = end.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (end.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float3 end_up = end;
        end_up.y = end_y_up;
        float3 dir = normalize(end_up - start_up);
        float3 side = normalize(cross(dir, ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32)));
        float3 vpos;
        if (in_var_Vertex_Position.y < 0.100000001490116119384765625f)
        {
            float2 sc_s = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t0);
            vpos = float3(sc_s.x, in_var_Vertex_Position.y, sc_s.y);
        }
        else
        {
            float2 sc_e = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t1);
            vpos = float3(sc_e.x, in_var_Vertex_Position.y, sc_e.y);
        }
        float4x4 rot = transpose(float4x4(float4(side, 0.0f), float4(dir, 0.0f), float4(normalize(cross(side, dir)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 local = mul(float4x4(float4(1.0f, 0.0f, 0.0f, start.x), float4(0.0f, 1.0f, 0.0f, start_y_up), float4(0.0f, 0.0f, 1.0f, start.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(rot, float4(vpos.x, vpos.y * (distance(start_up, end_up) * 1.10000002384185791015625f), vpos.z, 1.0f))).xyz;
        // 3D value-noise(perlin3)位移:8 角 Wang 哈希 -> 平滑插值 -> 映射到 [-0.5,0.5],
        // 只在水平面 (x,z) 方向施加 0.06 倍位移,让枝条沿墙面起伏。
        float3 local_noise_pos = local * 0.4000000059604644775390625f;
        int3 noise_cell = int3(floor(local_noise_pos));
        float3 noise_frac = frac(local_noise_pos);
        uint3 noise_cell_u = uint3(noise_cell);
        uint v449 = noise_cell_u.z;
        uint v451 = v449 + (v449 << 10u);
        uint v453 = v451 ^ (v451 >> 6u);
        uint v455 = v453 + (v453 << 3u);
        uint v457 = v455 ^ (v455 >> 11u);
        bool v460 = over_path_c ? over_path_c : true;
        uint v461 = over_path_c ? (over_path ? 1664525u : hash_default_u) : 1664525u;
        uint v462 = over_path_c ? (over_path ? 1013904223u : hash_default_u) : 1013904223u;
        uint v466 = (((noise_cell_u.y * v461) + (v457 + (v457 << 15u))) + v462) * v461;
        uint v468 = v466 ^ (v466 >> 11u);
        uint v471 = v468 ^ ((v468 << 7u) & 2636928640u);
        uint v474 = v471 ^ ((v471 << 15u) & 4022730752u);
        bool v477 = v460 ? v460 : true;
        uint v478 = v460 ? v461 : 1664525u;
        uint v479 = v460 ? v462 : 1013904223u;
        uint v483 = (((noise_cell_u.x * v478) + (v474 ^ (v474 >> 18u))) + v479) * v478;
        uint v485 = v483 ^ (v483 >> 11u);
        uint v488 = v485 ^ ((v485 << 7u) & 2636928640u);
        uint v491 = v488 ^ ((v488 << 15u) & 4022730752u);
        uint v493 = v491 ^ (v491 >> 18u);
        uint v495 = v493 + (v493 << 10u);
        uint v497 = v495 ^ (v495 >> 6u);
        uint v499 = v497 + (v497 << 3u);
        uint v501 = v499 ^ (v499 >> 11u);
        uint v504 = over_path_c ? (over_path ? 8388607u : hash_default_u) : 8388607u;
        uint v505 = over_path_c ? (over_path ? 1065353216u : hash_default_u) : 1065353216u;
        uint v510 = v460 ? v504 : 8388607u;
        uint v511 = v460 ? v505 : 1065353216u;
        uint3 v518 = uint3(noise_cell + int3(1, 0, 0));
        uint v521 = v518.z;
        uint v523 = v521 + (v521 << 10u);
        uint v525 = v523 ^ (v523 >> 6u);
        uint v527 = v525 + (v525 << 3u);
        uint v529 = v527 ^ (v527 >> 11u);
        bool v532 = v477 ? v477 : true;
        uint v533 = v477 ? v478 : 1664525u;
        uint v534 = v477 ? v479 : 1013904223u;
        uint v538 = (((v518.y * v533) + (v529 + (v529 << 15u))) + v534) * v533;
        uint v540 = v538 ^ (v538 >> 11u);
        uint v543 = v540 ^ ((v540 << 7u) & 2636928640u);
        uint v546 = v543 ^ ((v543 << 15u) & 4022730752u);
        bool v549 = v532 ? v532 : true;
        uint v550 = v532 ? v533 : 1664525u;
        uint v551 = v532 ? v534 : 1013904223u;
        uint v555 = (((v518.x * v550) + (v546 ^ (v546 >> 18u))) + v551) * v550;
        uint v557 = v555 ^ (v555 >> 11u);
        uint v560 = v557 ^ ((v557 << 7u) & 2636928640u);
        uint v563 = v560 ^ ((v560 << 15u) & 4022730752u);
        uint v565 = v563 ^ (v563 >> 18u);
        uint v567 = v565 + (v565 << 10u);
        uint v569 = v567 ^ (v567 >> 6u);
        uint v571 = v569 + (v569 << 3u);
        uint v573 = v571 ^ (v571 >> 11u);
        uint v576 = v477 ? v510 : 8388607u;
        uint v577 = v477 ? v511 : 1065353216u;
        uint v582 = v532 ? v576 : 8388607u;
        uint v583 = v532 ? v577 : 1065353216u;
        uint3 v590 = uint3(noise_cell + int3(0, 1, 0));
        uint v593 = v590.z;
        uint v595 = v593 + (v593 << 10u);
        uint v597 = v595 ^ (v595 >> 6u);
        uint v599 = v597 + (v597 << 3u);
        uint v601 = v599 ^ (v599 >> 11u);
        bool v604 = v549 ? v549 : true;
        uint v605 = v549 ? v550 : 1664525u;
        uint v606 = v549 ? v551 : 1013904223u;
        uint v610 = (((v590.y * v605) + (v601 + (v601 << 15u))) + v606) * v605;
        uint v612 = v610 ^ (v610 >> 11u);
        uint v615 = v612 ^ ((v612 << 7u) & 2636928640u);
        uint v618 = v615 ^ ((v615 << 15u) & 4022730752u);
        bool v621 = v604 ? v604 : true;
        uint v622 = v604 ? v605 : 1664525u;
        uint v623 = v604 ? v606 : 1013904223u;
        uint v627 = (((v590.x * v622) + (v618 ^ (v618 >> 18u))) + v623) * v622;
        uint v629 = v627 ^ (v627 >> 11u);
        uint v632 = v629 ^ ((v629 << 7u) & 2636928640u);
        uint v635 = v632 ^ ((v632 << 15u) & 4022730752u);
        uint v637 = v635 ^ (v635 >> 18u);
        uint v639 = v637 + (v637 << 10u);
        uint v641 = v639 ^ (v639 >> 6u);
        uint v643 = v641 + (v641 << 3u);
        uint v645 = v643 ^ (v643 >> 11u);
        uint v648 = v549 ? v582 : 8388607u;
        uint v649 = v549 ? v583 : 1065353216u;
        uint v654 = v604 ? v648 : 8388607u;
        uint v655 = v604 ? v649 : 1065353216u;
        uint3 v662 = uint3(noise_cell + int3(1, 1, 0));
        uint v665 = v662.z;
        uint v667 = v665 + (v665 << 10u);
        uint v669 = v667 ^ (v667 >> 6u);
        uint v671 = v669 + (v669 << 3u);
        uint v673 = v671 ^ (v671 >> 11u);
        bool v676 = v621 ? v621 : true;
        uint v677 = v621 ? v622 : 1664525u;
        uint v678 = v621 ? v623 : 1013904223u;
        uint v682 = (((v662.y * v677) + (v673 + (v673 << 15u))) + v678) * v677;
        uint v684 = v682 ^ (v682 >> 11u);
        uint v687 = v684 ^ ((v684 << 7u) & 2636928640u);
        uint v690 = v687 ^ ((v687 << 15u) & 4022730752u);
        bool v693 = v676 ? v676 : true;
        uint v694 = v676 ? v677 : 1664525u;
        uint v695 = v676 ? v678 : 1013904223u;
        uint v699 = (((v662.x * v694) + (v690 ^ (v690 >> 18u))) + v695) * v694;
        uint v701 = v699 ^ (v699 >> 11u);
        uint v704 = v701 ^ ((v701 << 7u) & 2636928640u);
        uint v707 = v704 ^ ((v704 << 15u) & 4022730752u);
        uint v709 = v707 ^ (v707 >> 18u);
        uint v711 = v709 + (v709 << 10u);
        uint v713 = v711 ^ (v711 >> 6u);
        uint v715 = v713 + (v713 << 3u);
        uint v717 = v715 ^ (v715 >> 11u);
        uint v720 = v621 ? v654 : 8388607u;
        uint v721 = v621 ? v655 : 1065353216u;
        uint v726 = v676 ? v720 : 8388607u;
        uint v727 = v676 ? v721 : 1065353216u;
        uint3 v734 = uint3(noise_cell + int3(0, 0, 1));
        uint v737 = v734.z;
        uint v739 = v737 + (v737 << 10u);
        uint v741 = v739 ^ (v739 >> 6u);
        uint v743 = v741 + (v741 << 3u);
        uint v745 = v743 ^ (v743 >> 11u);
        bool v748 = v693 ? v693 : true;
        uint v749 = v693 ? v694 : 1664525u;
        uint v750 = v693 ? v695 : 1013904223u;
        uint v754 = (((v734.y * v749) + (v745 + (v745 << 15u))) + v750) * v749;
        uint v756 = v754 ^ (v754 >> 11u);
        uint v759 = v756 ^ ((v756 << 7u) & 2636928640u);
        uint v762 = v759 ^ ((v759 << 15u) & 4022730752u);
        bool v765 = v748 ? v748 : true;
        uint v766 = v748 ? v749 : 1664525u;
        uint v767 = v748 ? v750 : 1013904223u;
        uint v771 = (((v734.x * v766) + (v762 ^ (v762 >> 18u))) + v767) * v766;
        uint v773 = v771 ^ (v771 >> 11u);
        uint v776 = v773 ^ ((v773 << 7u) & 2636928640u);
        uint v779 = v776 ^ ((v776 << 15u) & 4022730752u);
        uint v781 = v779 ^ (v779 >> 18u);
        uint v783 = v781 + (v781 << 10u);
        uint v785 = v783 ^ (v783 >> 6u);
        uint v787 = v785 + (v785 << 3u);
        uint v789 = v787 ^ (v787 >> 11u);
        uint v792 = v693 ? v726 : 8388607u;
        uint v793 = v693 ? v727 : 1065353216u;
        uint v798 = v748 ? v792 : 8388607u;
        uint v799 = v748 ? v793 : 1065353216u;
        uint3 v806 = uint3(noise_cell + int3(1, 0, 1));
        uint v809 = v806.z;
        uint v811 = v809 + (v809 << 10u);
        uint v813 = v811 ^ (v811 >> 6u);
        uint v815 = v813 + (v813 << 3u);
        uint v817 = v815 ^ (v815 >> 11u);
        bool v820 = v765 ? v765 : true;
        uint v821 = v765 ? v766 : 1664525u;
        uint v822 = v765 ? v767 : 1013904223u;
        uint v826 = (((v806.y * v821) + (v817 + (v817 << 15u))) + v822) * v821;
        uint v828 = v826 ^ (v826 >> 11u);
        uint v831 = v828 ^ ((v828 << 7u) & 2636928640u);
        uint v834 = v831 ^ ((v831 << 15u) & 4022730752u);
        bool v837 = v820 ? v820 : true;
        uint v838 = v820 ? v821 : 1664525u;
        uint v839 = v820 ? v822 : 1013904223u;
        uint v843 = (((v806.x * v838) + (v834 ^ (v834 >> 18u))) + v839) * v838;
        uint v845 = v843 ^ (v843 >> 11u);
        uint v848 = v845 ^ ((v845 << 7u) & 2636928640u);
        uint v851 = v848 ^ ((v848 << 15u) & 4022730752u);
        uint v853 = v851 ^ (v851 >> 18u);
        uint v855 = v853 + (v853 << 10u);
        uint v857 = v855 ^ (v855 >> 6u);
        uint v859 = v857 + (v857 << 3u);
        uint v861 = v859 ^ (v859 >> 11u);
        uint v864 = v765 ? v798 : 8388607u;
        uint v865 = v765 ? v799 : 1065353216u;
        uint v870 = v820 ? v864 : 8388607u;
        uint v871 = v820 ? v865 : 1065353216u;
        uint3 v878 = uint3(noise_cell + int3(0, 1, 1));
        uint v881 = v878.z;
        uint v883 = v881 + (v881 << 10u);
        uint v885 = v883 ^ (v883 >> 6u);
        uint v887 = v885 + (v885 << 3u);
        uint v889 = v887 ^ (v887 >> 11u);
        bool v892 = v837 ? v837 : true;
        uint v893 = v837 ? v838 : 1664525u;
        uint v894 = v837 ? v839 : 1013904223u;
        uint v898 = (((v878.y * v893) + (v889 + (v889 << 15u))) + v894) * v893;
        uint v900 = v898 ^ (v898 >> 11u);
        uint v903 = v900 ^ ((v900 << 7u) & 2636928640u);
        uint v906 = v903 ^ ((v903 << 15u) & 4022730752u);
        bool v909 = v892 ? v892 : true;
        uint v910 = v892 ? v893 : 1664525u;
        uint v911 = v892 ? v894 : 1013904223u;
        uint v915 = (((v878.x * v910) + (v906 ^ (v906 >> 18u))) + v911) * v910;
        uint v917 = v915 ^ (v915 >> 11u);
        uint v920 = v917 ^ ((v917 << 7u) & 2636928640u);
        uint v923 = v920 ^ ((v920 << 15u) & 4022730752u);
        uint v925 = v923 ^ (v923 >> 18u);
        uint v927 = v925 + (v925 << 10u);
        uint v929 = v927 ^ (v927 >> 6u);
        uint v931 = v929 + (v929 << 3u);
        uint v933 = v931 ^ (v931 >> 11u);
        uint v936 = v837 ? v870 : 8388607u;
        uint v937 = v837 ? v871 : 1065353216u;
        uint v942 = v892 ? v936 : 8388607u;
        uint v943 = v892 ? v937 : 1065353216u;
        uint3 v950 = uint3(noise_cell + int3(1, 1, 1));
        uint v953 = v950.z;
        uint v955 = v953 + (v953 << 10u);
        uint v957 = v955 ^ (v955 >> 6u);
        uint v959 = v957 + (v957 << 3u);
        uint v961 = v959 ^ (v959 >> 11u);
        bool v964 = v909 ? v909 : true;
        uint v965 = v909 ? v910 : 1664525u;
        uint v966 = v909 ? v911 : 1013904223u;
        uint v970 = (((v950.y * v965) + (v961 + (v961 << 15u))) + v966) * v965;
        uint v972 = v970 ^ (v970 >> 11u);
        uint v975 = v972 ^ ((v972 << 7u) & 2636928640u);
        uint v978 = v975 ^ ((v975 << 15u) & 4022730752u);
        uint v981 = v964 ? v965 : 1664525u;
        uint v986 = (((v950.x * v981) + (v978 ^ (v978 >> 18u))) + (v964 ? v966 : 1013904223u)) * v981;
        uint v988 = v986 ^ (v986 >> 11u);
        uint v991 = v988 ^ ((v988 << 7u) & 2636928640u);
        uint v994 = v991 ^ ((v991 << 15u) & 4022730752u);
        uint v996 = v994 ^ (v994 >> 18u);
        uint v998 = v996 + (v996 << 10u);
        uint v1000 = v998 ^ (v998 >> 6u);
        uint v1002 = v1000 + (v1000 << 3u);
        uint v1004 = v1002 ^ (v1002 >> 11u);
        uint v1007 = v909 ? v942 : 8388607u;
        uint v1008 = v909 ? v943 : 1065353216u;
        float3 noise_smooth = (noise_frac * noise_frac) * (3.0f.xxx - (noise_frac * 2.0f));
        float nx = noise_smooth.x;
        float om_nx = 1.0f - nx;
        float ny = noise_smooth.y;
        float om_ny = 1.0f - ny;
        float nz = noise_smooth.z;
        float om_nz = 1.0f - nz;
        float2 noise_sum0 = (((((float2(asfloat((v493 & v504) | v505) - 1.0f, asfloat(((v501 + (v501 << 15u)) & v510) | v511) - 1.0f) * om_nx) * om_ny) * om_nz) + (((float2(asfloat((v565 & v576) | v577) - 1.0f, asfloat(((v573 + (v573 << 15u)) & v582) | v583) - 1.0f) * nx) * om_ny) * om_nz)) + (((float2(asfloat((v637 & v648) | v649) - 1.0f, asfloat(((v645 + (v645 << 15u)) & v654) | v655) - 1.0f) * om_nx) * ny) * om_nz)) + (((float2(asfloat((v709 & v720) | v721) - 1.0f, asfloat(((v717 + (v717 << 15u)) & v726) | v727) - 1.0f) * nx) * ny) * om_nz);
        float2 noise_sum1 = (((noise_sum0 + (((float2(asfloat((v781 & v792) | v793) - 1.0f, asfloat(((v789 + (v789 << 15u)) & v798) | v799) - 1.0f) * om_nx) * om_ny) * nz)) + (((float2(asfloat((v853 & v864) | v865) - 1.0f, asfloat(((v861 + (v861 << 15u)) & v870) | v871) - 1.0f) * nx) * om_ny) * nz)) + (((float2(asfloat((v925 & v936) | v937) - 1.0f, asfloat(((v933 + (v933 << 15u)) & v942) | v943) - 1.0f) * om_nx) * ny) * nz)) + (((float2(asfloat((v996 & v1007) | v1008) - 1.0f, asfloat(((v1004 + (v1004 << 15u)) & (v964 ? v1007 : 8388607u)) | (v964 ? v1008 : 1065353216u)) - 1.0f) * nx) * ny) * nz);
        float2 disp = clamp((noise_sum1 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        // 模型->世界->裁剪空间变换;输出旋转后的法线、按段长缩放的 UV 与世界位置。
        float4 world = mul(model_to_world, float4(local + (float3(disp.x, 0.0f, disp.y) * 0.0599999986588954925537109375f), 1.0f));
        float end_y_half = end_y_up * 0.5f;
        out_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, world);
        out_normal = mul(rot, float4(normalize(float3(in_var_Vertex_Position.x, 0.0f, in_var_Vertex_Position.z)), 0.0f)).xyz;
        out_uv = float2(end.x + end_y_half, end.z + end_y_half) * 0.20000000298023223876953125f;
        out_world = world.xyz;
        break;
    } while(false);
    gl_Position = out_pos;
    out_var_C1 = out_normal;
    out_var_C2 = out_uv;
    out_var_C3 = out_world;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译段(f0ad 变体 PS):spirv-cross,行为精确。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
