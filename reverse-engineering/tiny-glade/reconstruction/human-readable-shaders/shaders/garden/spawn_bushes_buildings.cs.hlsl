// PATCHED: modules gs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/garden/spawn_bushes_buildings.cs.hlsl
// 灌木生成(建筑区):对网格格点做路径/岩石/水面/植被排除,garden_mask.y>0.3 跳过,
// 建筑遮罩 gate + 世界坐标 perlin>0.5 时放置 BushXform(命令 10,48B,按地形高度,
// theme4 时 -0.5);30% 概率附加 FlowerXform(命令 11,64B,color -1,type 2)。
// 依据:_garden_spawn_bushes_buildings.cs.hlsl_cs_main_0.bin 的 spirv-cross。
// 状态:O (算法取自 SPIR-V);I (阈值/缩放字面量)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "buildings/building_mask.inc.hlsl"
#include "garden/defs.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "garden/placement_utils.inc.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);
RWStructuredBuffer<BushXform> __alias_bush_buffer : register(u20, space0);
RWStructuredBuffer<FlowerXform> __alias_bush_flowers : register(u21, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float rnd = rand01(hash(id.x * id.y + id.x));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-1.0f, 1.0f, rnd).xx;

    float2 uv = placement_uv(world_pos);
    if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                           rocky_terrain, water_mask, foliage_exclusion_mask,
                           sampler_llb, sampler_default, uv))
        return;
    if (garden_mask.Load(int3(uint2(uv * float2(path_mask_dims)), 0u)).y > 0.3f)
        return;

    float bmx = buildings_mask.SampleLevel(sampler_default, uv, 0.0f).x;
    if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
        return;

    if (bmx > 0.0f && perlin(world_pos) > 0.5f)
    {
        float height = get_terrain_height(terrain_heightmap, sampler_default, world_pos);
        float3 pos = float3(world_pos.x, glade_theme_id == 4u ? height - 0.5f : height, world_pos.y);

        float band1 = band_falloff_garden(abs(buildings_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.4f), 3.3333332538604736328125f);
        float fs = lerp(0.8f, 1.2f, rand01(hash(id.x + id.y)));
        float3 s0 = fs.xxx * lerp(0.2f, 1.0f, sqrt(band1));
        float4x4 s = float4x4(
            float4(min(s0.x, 0.75f), 0.0f, 0.0f, 0.0f),
            float4(0.0f, min(s0.y * fs, 1.2f), 0.0f, 0.0f),
            float4(0.0f, 0.0f, min(s0.z, 0.75f), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 s3 = float3(min(s0.x, 0.75f), min(s0.y * fs, 1.2f), min(s0.z, 0.75f)) * 0.75f;

        float band2 = band_falloff_garden(abs(bmx - 0.4f), 3.3333332538604736328125f);
        if (min((fs.xxx * lerp(0.2f, 1.0f, sqrt(band2))).x, 0.75f) * 0.75f < 0.3f)
            return;

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 m = mul(mul(
            affine_translation(pos),
            mul(identity, s)),
            rotate_y(3.14f * rnd));

        uint bush_base;
        InterlockedAdd(__alias_draw_commands[10u].instanceCount, 1u, bush_base);
        __alias_bush_buffer[bush_base] = BushXform(pack_affine(m));

        if (rand01(hash((id.x * 2u) + (id.y / 5u) + 123u)) > 0.7f)
        {
            uint flower_base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, flower_base);
            FlowerXform f;
            f.transform = pack_affine(m);
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 2u;
            __alias_bush_flowers[flower_base] = f;
        }
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _garden_spawn_bushes_buildings.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,变量名已语义化)
// 计算着色器:建筑区灌木/花朵生成(逻辑同重建段,算术为 oracle 逐字)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

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

struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

static float scratch;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 buildings_mask_dims : packoffset(c2);
    uint2 buildings_mask_non_blurred_dims : packoffset(c2.z);
    int2 grid_dims : packoffset(c3);
    uint2 path_mask_dims : packoffset(c3.z);
    uint2 rocky_terrain_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c5);
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

RWByteAddressBuffer __alias_bush_buffer : register(u20, space0);
RWByteAddressBuffer __alias_bush_flowers : register(u21, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        uint2 px;
        uint seed0 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint h1a = seed0 + (seed0 << 10u);
        uint h1b = h1a ^ (h1a >> 6u);
        uint h1c = h1b + (h1b << 3u);
        uint h1d = h1c ^ (h1c >> 11u);
        float rand01 = asfloat(((h1d + (h1d << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 world_pos = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, rand01).xx;
        float2 uv = (world_pos * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        bool excluded;
        do
        {
            px = uint2(uv * float2(path_mask_dims));
            float4 g = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
            float gz = g.z;
            float gy = g.y;
            float gw = g.w;
            float gx = g.x;
            bool excl_path;
            if (!(path_mask.Load(int3(px, 0u)).x > 0.20000000298023223876953125f))
            {
                excl_path = (int(rocky_terrain.Load(int3(px, 0u)).x > 0.0f) & int(length(((float2((gz + gy) - (gw + gx), (gx + gy) - (gw + gz)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                excl_path = true;
            }
            bool excl_water;
            if (!excl_path)
            {
                excl_water = water_mask.Load(int3(px, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                excl_water = true;
            }
            bool excl_foliage;
            if (!excl_water)
            {
                excl_foliage = foliage_exclusion_mask.SampleLevel(sampler_llb, uv, 0.0f).x > 0.0f;
            }
            else
            {
                excl_foliage = true;
            }
            if (excl_foliage)
            {
                excluded = true;
                break;
            }
            excluded = false;
            break;
        } while(false);
        if (excluded)
        {
            break;
        }
        if (garden_mask.Load(int3(px, 0u)).y > 0.300000011920928955078125f)
        {
            break;
        }
        float4 bmask_s = buildings_mask.SampleLevel(sampler_default, uv, 0.0f);
        float bmask_x = bmask_s.x;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
        {
            break;
        }
        bool perlin_gt;
        if (bmask_x > 0.0f)
        {
            int2 cell = int2(floor(world_pos));
            float2 frac_pos = frac(world_pos);
            uint2 cell_u = uint2(cell);
            uint cell_y = cell_u.y;
            uint ha0 = cell_y + (cell_y << 10u);
            uint ha1 = ha0 ^ (ha0 >> 6u);
            uint ha2 = ha1 + (ha1 << 3u);
            uint ha3 = ha2 ^ (ha2 >> 11u);
            uint ra0 = (((cell_u.x * 1664525u) + (ha3 + (ha3 << 15u))) + 1013904223u) * 1664525u;
            uint ra1 = ra0 ^ (ra0 >> 11u);
            uint ra2 = ra1 ^ ((ra1 << 7u) & 2636928640u);
            uint ra3 = ra2 ^ ((ra2 << 15u) & 4022730752u);
            uint2 cell_x1 = uint2(cell + int2(1, 0));
            uint c1y = cell_x1.y;
            uint hb0 = c1y + (c1y << 10u);
            uint hb1 = hb0 ^ (hb0 >> 6u);
            uint hb2 = hb1 + (hb1 << 3u);
            uint hb3 = hb2 ^ (hb2 >> 11u);
            uint rb0 = (((cell_x1.x * 1664525u) + (hb3 + (hb3 << 15u))) + 1013904223u) * 1664525u;
            uint rb1 = rb0 ^ (rb0 >> 11u);
            uint rb2 = rb1 ^ ((rb1 << 7u) & 2636928640u);
            uint rb3 = rb2 ^ ((rb2 << 15u) & 4022730752u);
            uint2 cell_y1 = uint2(cell + int2(0, 1));
            uint c2y = cell_y1.y;
            uint hc0 = c2y + (c2y << 10u);
            uint hc1 = hc0 ^ (hc0 >> 6u);
            uint hc2 = hc1 + (hc1 << 3u);
            uint hc3 = hc2 ^ (hc2 >> 11u);
            uint rc0 = (((cell_y1.x * 1664525u) + (hc3 + (hc3 << 15u))) + 1013904223u) * 1664525u;
            uint rc1 = rc0 ^ (rc0 >> 11u);
            uint rc2 = rc1 ^ ((rc1 << 7u) & 2636928640u);
            uint rc3 = rc2 ^ ((rc2 << 15u) & 4022730752u);
            uint2 cell_xy1 = uint2(cell + int2(1, 1));
            uint c3y = cell_xy1.y;
            uint hd0 = c3y + (c3y << 10u);
            uint hd1 = hd0 ^ (hd0 >> 6u);
            uint hd2 = hd1 + (hd1 << 3u);
            uint hd3 = hd2 ^ (hd2 >> 11u);
            uint rd0 = (((cell_xy1.x * 1664525u) + (hd3 + (hd3 << 15u))) + 1013904223u) * 1664525u;
            uint rd1 = rd0 ^ (rd0 >> 11u);
            uint rd2 = rd1 ^ ((rd1 << 7u) & 2636928640u);
            uint rd3 = rd2 ^ ((rd2 << 15u) & 4022730752u);
            float2 u_s = (frac_pos * frac_pos) * (3.0f.xx - (frac_pos * 2.0f));
            float u_x = u_s.x;
            float u_x_inv = 1.0f - u_x;
            float u_y = u_s.y;
            perlin_gt = ((((1.0f - u_y) * (((asfloat(((ra3 ^ (ra3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * u_x_inv) + ((asfloat(((rb3 ^ (rb3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * u_x))) + (((asfloat(((rc3 ^ (rc3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * u_x_inv) * u_y)) + (((asfloat(((rd3 ^ (rd3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * u_x) * u_y)) > 0.5f;
        }
        else
        {
            perlin_gt = false;
        }
        if (perlin_gt)
        {
            float3 bush_pos = world_pos.xxy;
            float terrain_h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (world_pos.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
            bush_pos.y = terrain_h;
            float3 bush_pos_final;
            if (frame_constants_glade_theme_id == 4u)
            {
                float3 theme4_pos = bush_pos;
                theme4_pos.y = terrain_h - 0.5f;
                bush_pos_final = theme4_pos;
            }
            else
            {
                bush_pos_final = bush_pos;
            }
            float bush_scale_f;
            do
            {
                float dist_bm = abs(bmask_s.y - 0.4000000059604644775390625f);
                if (dist_bm > 0.300000011920928955078125f)
                {
                    bush_scale_f = 0.0f;
                    break;
                }
                float t_bm = dist_bm * 3.3333332538604736328125f;
                bush_scale_f = 1.0f - ((t_bm * t_bm) * (3.0f - (dist_bm * 6.666666507720947265625f)));
                break;
            } while(false);
            uint seed2 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint he0 = seed2 + (seed2 << 10u);
            uint he1 = he0 ^ (he0 >> 6u);
            uint he2 = he1 + (he1 << 3u);
            uint he3 = he2 ^ (he2 >> 11u);
            float rand_scale = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((he3 + (he3 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 rand_scale3 = rand_scale.xxx;
            float3 scaled = rand_scale3 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(bush_scale_f));
            float scaled_y = scaled.y * rand_scale;
            float3 min_scale = float3(min(scaled.x, 0.75f), scaled_y, min(scaled.z, 0.75f));
            min_scale.y = min(scaled_y, 1.2000000476837158203125f);
            float3 bush_scale = min_scale * 0.75f;
            float flower_scale_f;
            do
            {
                float dist_bm2 = abs(bmask_x - 0.4000000059604644775390625f);
                if (dist_bm2 > 0.300000011920928955078125f)
                {
                    flower_scale_f = 0.0f;
                    break;
                }
                float t_bm2 = dist_bm2 * 3.3333332538604736328125f;
                flower_scale_f = 1.0f - ((t_bm2 * t_bm2) * (3.0f - (dist_bm2 * 6.666666507720947265625f)));
                break;
            } while(false);
            if ((float3(min((rand_scale3 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(flower_scale_f))).x, 0.75f), scratch, scratch) * 0.75f).x < 0.300000011920928955078125f)
            {
                break;
            }
            float yaw_angle = 3.1400001049041748046875f * rand01;
            float3 axis_y = normalize(float3(0.0f, 1.0f, 0.0f));
            float sin_a = sin(yaw_angle);
            float cos_a = cos(yaw_angle);
            float omc = 1.0f - cos_a;
            float axis_x = axis_y.x;
            float tx_y = omc * axis_x;
            float axis_y2 = axis_y.y;
            float tx_y2 = tx_y * axis_y2;
            float axis_z = axis_y.z;
            float t_z = axis_z * sin_a;
            float t_zy = omc * axis_z;
            float t_zx = t_zy * axis_x;
            float t_y = axis_y2 * sin_a;
            float t_yy = omc * axis_y2;
            float t_zz = t_yy * axis_z;
            float t_x = axis_x * sin_a;
            float4x4 transform = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, bush_pos_final.x), float4(0.0f, 1.0f, 0.0f, bush_pos_final.y), float4(0.0f, 0.0f, 1.0f, bush_pos_final.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(bush_scale.x, 0.0f, 0.0f, 0.0f), float4(0.0f, bush_scale.y, 0.0f, 0.0f), float4(0.0f, 0.0f, bush_scale.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((tx_y * axis_x) + cos_a, tx_y2 - t_z, t_zx + t_y, 0.0f), float4(tx_y2 + t_z, (t_yy * axis_y2) + cos_a, t_zz - t_x, 0.0f), float4(t_zx - t_y, t_zz + t_x, (t_zy * axis_z) + cos_a, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint slot;
            __alias_draw_commands.InterlockedAdd(204, 1u, slot);
            uint bush_slot = uint(int(slot));
            float data[12] = { transform[0].x, transform[1].x, transform[2].x, transform[0].y, transform[1].y, transform[2].y, transform[0].z, transform[1].z, transform[2].z, transform[0].w, transform[1].w, transform[2].w };
            Affine3Packed affine = { data };
            BushXform bush_xform = { affine };
            [unroll]
            for (int k = 0; k < 12; k++)
            {
                __alias_bush_buffer.Store<float>(k * 4 + bush_slot * 48 + 0, bush_xform.transform.data[k]);
            }
            uint flower_seed = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 123u;
            uint ha0f = flower_seed + (flower_seed << 10u);
            uint ha1f = ha0f ^ (ha0f >> 6u);
            uint ha2f = ha1f + (ha1f << 3u);
            uint ha3f = ha2f ^ (ha2f >> 11u);
            if ((asfloat(((ha3f + (ha3f << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.699999988079071044921875f)
            {
                uint flower_slot;
                __alias_draw_commands.InterlockedAdd(224, 1u, flower_slot);
                uint flower_slot_i = uint(int(flower_slot));
                FlowerXform flower_xform = { affine, (-1.0f).xxx, 2u };
                [unroll]
                for (int k2 = 0; k2 < 12; k2++)
                {
                    __alias_bush_flowers.Store<float>(k2 * 4 + flower_slot_i * 64 + 0, flower_xform.transform.data[k2]);
                }
                __alias_bush_flowers.Store<float3>(flower_slot_i * 64 + 48, flower_xform.color);
                __alias_bush_flowers.Store<uint>(flower_slot_i * 64 + 60, flower_xform.flower_type);
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
