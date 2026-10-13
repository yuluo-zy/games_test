// PATCHED: modules ds.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/spawn_lilypads.cs.hlsl
// 中文说明:
//   荷叶生成 compute pass。证据(spirv-cross of _water_spawn_lilypads.cs.hlsl
//   _cs_main_0.bin):animation 存储图像向"花园已清除"标志缓动;水位带
//   (主题 2 -> perlin 扭曲带);路径带;perlin 门控;LilypadXform(48B)
//   随机基 * yaw * 生长缩放;20% 花 -> 命令槽 15。

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/texture_utils.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

struct LilypadXform
{
    Affine3Packed xform;
};

// 绕(已归一化)轴的 Rodrigues 旋转——转置行主。
float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    float cxx = omc * x;
    float cyy = omc * y;
    float czz = omc * z;
    return transpose(float4x4(
        float4(cxx * x + c, cxx * y - z * s, czz * x + y * s, 0.0f),
        float4(cxx * y + z * s, cyy * y + c, cyy * z - x * s, 0.0f),
        float4(czz * x - y * s, cyy * z + x * s, czz * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

float band_falloff(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 garden_mask_dims;
    uint2 animation_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> garden_mask : register(t17, space0);
RWTexture2D<half> animation : register(u18, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_buffer : register(u19, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_flower_buffer : register(u20, space0);

// frame_constants.hlsl(重建 inc/)省略了原版用来发射 RuntimeDescriptorArray 的
// 死亡无界数组声明;在此复现。
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    if (id.x >= animation_dims.x || id.y >= animation_dims.y)
        return;

    float rx = rand01(hash(id.x));
    float ry = rand01(hash(id.y));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f
        + (lerp(-0.25f, 0.25f, (rx + ry) * 0.5f) * 2.0f).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;
    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;

    float garden = smoothstep(0.4f, 0.5f, smoothstep(0.0f, 0.5f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y));
    animation[id.xy] = lerp(animation[id.xy].x, float(garden > 0.0f), 1.5f * delta_time_seconds);

    bool theme2 = glade_theme_id == 2u;
    float water_band;
    if (theme2)
    {
        water_band = band_falloff(abs((water + perlin(world_pos * 1.83f) * 0.3f) - 0.662f), 2.5445291996002197265625f);
    }
    else
    {
        water_band = band_falloff(abs(water - 0.45f), 3.3333332538604736328125f);
    }

    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float pad_score = water_band + path_band * smoothstep(0.6f, 0.8f, water);
    float bloom_score = garden * smoothstep(0.4f, 0.5f, water);

    if (pad_score <= 0.0f)
        return;

    bool perlin_gate = theme2
        ? perlin(world_pos) > 0.5f
        : (bloom_score > 0.0f && perlin(world_pos * 2.0f) > 0.5f);

    if (!perlin_gate)
        return;

    float3 pos = float3(world_pos.x, 0.0f, world_pos.y);
    if (glade_theme_id == 4u)
        pos.y = -0.1f;

    float rand = rand01(hash(id.x + id.y));
    float3 scale = (lerp(0.8f, theme2 ? 1.3f : 2.0f, rand)
                     * lerp(0.5f, 1.0f, clamp(pad_score + bloom_score, 0.0f, 1.0f))
                     * lerp(1.0f, 0.75f, bloom_score)).xxx;

    float2 dir2 = float2(rand01(hash(id.x ^ 234u)) * 2.0f - 1.0f, rand01(hash(id.x ^ 8634u)) * 2.0f - 1.0f);
    float3 dir = normalize(float3(dir2.x, 0.0f, dir2.y));
    float3 up = dir.y < 0.0f ? float3(0.0f, -1.0f, 0.0f) : float3(0.0f, 1.0f, 0.0f);
    float4x4 basis = transpose(float4x4(
        float4(dir, 0.0f),
        float4(up, 0.0f),
        float4(cross(dir, up), 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4x4 m = mul(mul(
        affine_translation(pos),
        mul(basis, float4x4(
            float4(scale.x, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale.y, 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale.z, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f)))),
        rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), 3.14f * rand));

    uint base;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
    __alias_lilypad_buffer[base] = LilypadXform(pack_affine(m));

    if (bloom_score > 0.7f && rand01(hash(id.x + id.y + 345u)) > 0.2f)
    {
        float grow = clamp(1.0f - smoothstep(rand01(hash((10u * id.x) + 45u + id.y * 10u)) * 0.2f - 0.2f, 1.0f, animation[id.xy].x), 0.0f, 1.0f);
        float pop = (grow * grow * grow) - (grow * sin(grow * 3.14f) * 1.5f);
        float3 flower_scale = scale * (bloom_score * 0.8f) * (1.0f - pop);
        if (theme2)
            flower_scale *= 2.0f;

        float jx = rand01(hash((id.x + id.y * 2u) + 345u));
        float jy = rand01(hash(id.x * 10u + id.y * 345u));
        float3 fpos = float3(
            pos.x + lerp(-0.3f, 0.3f, jx),
            (pos.y - pop * 0.3f) + jy * 0.1f,
            pos.z + lerp(-0.3f, 0.3f, rand01(hash((id.x + id.y * 3u) - 345u))));

        float4x4 fm = mul(
            affine_translation(fpos),
            mul(basis, float4x4(
                float4(flower_scale.x, 0.0f, 0.0f, 0.0f),
                float4(0.0f, flower_scale.y, 0.0f, 0.0f),
                float4(0.0f, 0.0f, flower_scale.z, 0.0f),
                float4(0.0f, 0.0f, 0.0f, 1.0f))));

        uint fbase;
        InterlockedAdd(__alias_draw_commands[15u].instanceCount, 1u, fbase);
        __alias_lilypad_flower_buffer[fbase] = LilypadXform(pack_affine(fm));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _water_spawn_lilypads.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,变量名已语义化)
// 算法与上方手写段一致:animation 缓动 -> 水位带/路径带/噪声门控 -> 荷叶 = T*R*S(yaw)
// + 20% 概率写入花朵实例。
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

struct LilypadXform
{
    Affine3Packed xform;
};

static uint seed_sentinel;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 water_mask_dims : packoffset(c0.y);
    uint2 path_mask_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 animation_dims : packoffset(c2);
    int2 grid_dims : packoffset(c2.z);
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

RWByteAddressBuffer __alias_lilypad_buffer : register(u19, space0);
RWByteAddressBuffer __alias_lilypad_flower_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> garden_mask : register(t17, space0);
RWTexture2D<float> animation : register(u18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (any(bool2(gl_GlobalInvocationID.xy.x >= animation_dims.x, gl_GlobalInvocationID.xy.y >= animation_dims.y)))
        {
            break;
        }
        float2 uv;
        uint sx0 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
        uint sx1 = sx0 ^ (sx0 >> 6u);
        uint sx2 = sx1 + (sx1 << 3u);
        uint sx3 = sx2 ^ (sx2 >> 11u);
        uint sy0 = gl_GlobalInvocationID.y + (gl_GlobalInvocationID.y << 10u);
        uint sy1 = sy0 ^ (sy0 >> 6u);
        uint sy2 = sy1 + (sy1 << 3u);
        uint sy3 = sy2 ^ (sy2 >> 11u);
        float2 world_pos = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + (lerp(-0.25f, 0.25f, ((asfloat(((sx3 + (sx3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) + (asfloat(((sy3 + (sy3 << 15u)) & 8388607u) | 1065353216u) - 1.0f)) * 0.5f) * 2.0f).xx;
        float water;
        do
        {
            uv = (world_pos * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            bool oob_x0;
            if (!(uv.x < 0.0f))
            {
                oob_x0 = uv.x > 1.0f;
            }
            else
            {
                oob_x0 = true;
            }
            bool oob_y0;
            if (!oob_x0)
            {
                oob_y0 = uv.y < 0.0f;
            }
            else
            {
                oob_y0 = true;
            }
            bool oob_y1;
            if (!oob_y0)
            {
                oob_y1 = uv.y > 1.0f;
            }
            else
            {
                oob_y1 = true;
            }
            if (oob_y1)
            {
                water = 0.0f;
                break;
            }
            else
            {
                water = water_mask.SampleLevel(sampler_default, uv, 0.0f).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float garden = smoothstep(0.4000000059604644775390625f, 0.5f, smoothstep(0.0f, 0.5f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y));
        animation[gl_GlobalInvocationID.xy] = lerp(animation[gl_GlobalInvocationID.xy].xxxx.x, ((garden > 0.0f) ? 1.0f : 0.0f), 1.5f * frame_constants_delta_time_seconds);
        bool theme2 = frame_constants_glade_theme_id == 2u;
        float water_band;
        if (theme2)
        {
            float2 perlin_pos = world_pos * 1.83000004291534423828125f;
            int2 cell = int2(floor(perlin_pos));
            float2 f = frac(perlin_pos);
            uint2 cell_u = uint2(cell);
            uint cy00 = cell_u.y;
            uint ch00a = cy00 + (cy00 << 10u);
            uint ch00b = ch00a ^ (ch00a >> 6u);
            uint ch00c = ch00b + (ch00b << 3u);
            uint ch00d = ch00c ^ (ch00c >> 11u);
            uint hh00 = (((cell_u.x * 1664525u) + (ch00d + (ch00d << 15u))) + 1013904223u) * 1664525u;
            uint hh00a = hh00 ^ (hh00 >> 11u);
            uint hh00b = hh00a ^ ((hh00a << 7u) & 2636928640u);
            uint hh00c = hh00b ^ ((hh00b << 15u) & 4022730752u);
            uint2 cell10 = uint2(cell + int2(1, 0));
            uint cy10 = cell10.y;
            uint ch10a = cy10 + (cy10 << 10u);
            uint ch10b = ch10a ^ (ch10a >> 6u);
            uint ch10c = ch10b + (ch10b << 3u);
            uint ch10d = ch10c ^ (ch10c >> 11u);
            uint hh10 = (((cell10.x * 1664525u) + (ch10d + (ch10d << 15u))) + 1013904223u) * 1664525u;
            uint hh10a = hh10 ^ (hh10 >> 11u);
            uint hh10b = hh10a ^ ((hh10a << 7u) & 2636928640u);
            uint hh10c = hh10b ^ ((hh10b << 15u) & 4022730752u);
            uint2 cell01 = uint2(cell + int2(0, 1));
            uint cy01 = cell01.y;
            uint ch01a = cy01 + (cy01 << 10u);
            uint ch01b = ch01a ^ (ch01a >> 6u);
            uint ch01c = ch01b + (ch01b << 3u);
            uint ch01d = ch01c ^ (ch01c >> 11u);
            uint hh01 = (((cell01.x * 1664525u) + (ch01d + (ch01d << 15u))) + 1013904223u) * 1664525u;
            uint hh01a = hh01 ^ (hh01 >> 11u);
            uint hh01b = hh01a ^ ((hh01a << 7u) & 2636928640u);
            uint hh01c = hh01b ^ ((hh01b << 15u) & 4022730752u);
            uint2 cell11 = uint2(cell + int2(1, 1));
            uint cy11 = cell11.y;
            uint ch11a = cy11 + (cy11 << 10u);
            uint ch11b = ch11a ^ (ch11a >> 6u);
            uint ch11c = ch11b + (ch11b << 3u);
            uint ch11d = ch11c ^ (ch11c >> 11u);
            uint hh11 = (((cell11.x * 1664525u) + (ch11d + (ch11d << 15u))) + 1013904223u) * 1664525u;
            uint hh11a = hh11 ^ (hh11 >> 11u);
            uint hh11b = hh11a ^ ((hh11a << 7u) & 2636928640u);
            uint hh11c = hh11b ^ ((hh11b << 15u) & 4022730752u);
            float2 u = (f * f) * (3.0f.xx - (f * 2.0f));
            float ux = u.x;
            float omux = 1.0f - ux;
            float uy = u.y;
            float wband_val;
            do
            {
                float wband_dist = abs((water + (((((1.0f - uy) * (((asfloat(((hh00c ^ (hh00c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux) + ((asfloat(((hh10c ^ (hh10c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux))) + (((asfloat(((hh01c ^ (hh01c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux) * uy)) + (((asfloat(((hh11c ^ (hh11c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux) * uy)) * 0.300000011920928955078125f)) - 0.662000000476837158203125f);
                if (wband_dist > 0.39300000667572021484375f)
                {
                    wband_val = 0.0f;
                    break;
                }
                float wband_norm = wband_dist * 2.5445291996002197265625f;
                wband_val = 1.0f - ((wband_norm * wband_norm) * (3.0f - (wband_dist * 5.089058399200439453125f)));
                break;
            } while(false);
            water_band = wband_val;
        }
        else
        {
            float wband2_val;
            do
            {
                float wband2_dist = abs(water - 0.449999988079071044921875f);
                if (wband2_dist > 0.300000011920928955078125f)
                {
                    wband2_val = 0.0f;
                    break;
                }
                float wband2_norm = wband2_dist * 3.3333332538604736328125f;
                wband2_val = 1.0f - ((wband2_norm * wband2_norm) * (3.0f - (wband2_dist * 6.666666507720947265625f)));
                break;
            } while(false);
            water_band = wband2_val;
        }
        uint seed2 = theme2 ? 1013904223u : seed_sentinel;
        uint seed1 = theme2 ? 1664525u : seed_sentinel;
        bool theme2_flag = theme2 ? true : false;
        float path_band;
        do
        {
            float pband_dist = abs(path_mask.SampleLevel(sampler_llb, uv, 0.0f).x - 0.20000000298023223876953125f);
            if (pband_dist > 0.20000000298023223876953125f)
            {
                path_band = 0.0f;
                break;
            }
            float pband_norm = pband_dist * 5.0f;
            path_band = 1.0f - ((pband_norm * pband_norm) * (3.0f - (pband_dist * 10.0f)));
            break;
        } while(false);
        float pad_score = water_band + (path_band * smoothstep(0.60000002384185791015625f, 0.800000011920928955078125f, water));
        float bloom_score = garden * smoothstep(0.4000000059604644775390625f, 0.5f, water);
        uint seed_s2;
        uint seed_s1;
        bool flag_s;
        bool perlin_gate;
        if (pad_score > 0.0f)
        {
            int2 cell2 = int2(floor(world_pos));
            float2 f2 = frac(world_pos);
            uint2 cell2_u = uint2(cell2);
            uint cy20 = cell2_u.y;
            uint ch20a = cy20 + (cy20 << 10u);
            uint ch20b = ch20a ^ (ch20a >> 6u);
            uint ch20c = ch20b + (ch20b << 3u);
            uint ch20d = ch20c ^ (ch20c >> 11u);
            bool flag20 = theme2_flag ? theme2_flag : true;
            uint s20a = theme2_flag ? seed1 : 1664525u;
            uint s20b = theme2_flag ? seed2 : 1013904223u;
            uint hh20 = (((cell2_u.x * s20a) + (ch20d + (ch20d << 15u))) + s20b) * s20a;
            uint hh20a = hh20 ^ (hh20 >> 11u);
            uint hh20b = hh20a ^ ((hh20a << 7u) & 2636928640u);
            uint hh20c = hh20b ^ ((hh20b << 15u) & 4022730752u);
            uint2 cell21 = uint2(cell2 + int2(1, 0));
            uint cy21 = cell21.y;
            uint ch21a = cy21 + (cy21 << 10u);
            uint ch21b = ch21a ^ (ch21a >> 6u);
            uint ch21c = ch21b + (ch21b << 3u);
            uint ch21d = ch21c ^ (ch21c >> 11u);
            bool flag21 = flag20 ? flag20 : true;
            uint s21a = flag20 ? s20a : 1664525u;
            uint s21b = flag20 ? s20b : 1013904223u;
            uint hh21 = (((cell21.x * s21a) + (ch21d + (ch21d << 15u))) + s21b) * s21a;
            uint hh21a = hh21 ^ (hh21 >> 11u);
            uint hh21b = hh21a ^ ((hh21a << 7u) & 2636928640u);
            uint hh21c = hh21b ^ ((hh21b << 15u) & 4022730752u);
            uint2 cell22 = uint2(cell2 + int2(0, 1));
            uint cy22 = cell22.y;
            uint ch22a = cy22 + (cy22 << 10u);
            uint ch22b = ch22a ^ (ch22a >> 6u);
            uint ch22c = ch22b + (ch22b << 3u);
            uint ch22d = ch22c ^ (ch22c >> 11u);
            bool flag22 = flag21 ? flag21 : true;
            uint s22a = flag21 ? s21a : 1664525u;
            uint s22b = flag21 ? s21b : 1013904223u;
            uint hh22 = (((cell22.x * s22a) + (ch22d + (ch22d << 15u))) + s22b) * s22a;
            uint hh22a = hh22 ^ (hh22 >> 11u);
            uint hh22b = hh22a ^ ((hh22a << 7u) & 2636928640u);
            uint hh22c = hh22b ^ ((hh22b << 15u) & 4022730752u);
            uint2 cell23 = uint2(cell2 + int2(1, 1));
            uint cy23 = cell23.y;
            uint ch23a = cy23 + (cy23 << 10u);
            uint ch23b = ch23a ^ (ch23a >> 6u);
            uint ch23c = ch23b + (ch23b << 3u);
            uint ch23d = ch23c ^ (ch23c >> 11u);
            uint s23a = flag22 ? s22a : 1664525u;
            uint s23b = flag22 ? s22b : 1013904223u;
            uint hh23 = (((cell23.x * s23a) + (ch23d + (ch23d << 15u))) + s23b) * s23a;
            uint hh23a = hh23 ^ (hh23 >> 11u);
            uint hh23b = hh23a ^ ((hh23a << 7u) & 2636928640u);
            uint hh23c = hh23b ^ ((hh23b << 15u) & 4022730752u);
            float2 u2 = (f2 * f2) * (3.0f.xx - (f2 * 2.0f));
            float ux2 = u2.x;
            float omux2 = 1.0f - ux2;
            float uy2 = u2.y;
            seed_s2 = s23b;
            seed_s1 = s23a;
            flag_s = flag22 ? flag22 : true;
            perlin_gate = ((((1.0f - uy2) * (((asfloat(((hh20c ^ (hh20c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux2) + ((asfloat(((hh21c ^ (hh21c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux2))) + (((asfloat(((hh22c ^ (hh22c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux2) * uy2)) + (((asfloat(((hh23c ^ (hh23c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux2) * uy2)) > 0.5f;
        }
        else
        {
            seed_s2 = seed2;
            seed_s1 = seed1;
            flag_s = theme2_flag;
            perlin_gate = false;
        }
        bool bloom_gate;
        if (!perlin_gate)
        {
            bool bloom_gate_inner;
            if (bloom_score > 0.0f)
            {
                float2 perlin2_pos = world_pos * 2.0f;
                int2 cell3 = int2(floor(perlin2_pos));
                float2 f3 = frac(perlin2_pos);
                uint2 cell3_u = uint2(cell3);
                uint cy30 = cell3_u.y;
                uint ch30a = cy30 + (cy30 << 10u);
                uint ch30b = ch30a ^ (ch30a >> 6u);
                uint ch30c = ch30b + (ch30b << 3u);
                uint ch30d = ch30c ^ (ch30c >> 11u);
                bool flag30 = flag_s ? flag_s : true;
                uint s30a = flag_s ? seed_s1 : 1664525u;
                uint s30b = flag_s ? seed_s2 : 1013904223u;
                uint hh30 = (((cell3_u.x * s30a) + (ch30d + (ch30d << 15u))) + s30b) * s30a;
                uint hh30a = hh30 ^ (hh30 >> 11u);
                uint hh30b = hh30a ^ ((hh30a << 7u) & 2636928640u);
                uint hh30c = hh30b ^ ((hh30b << 15u) & 4022730752u);
                uint2 cell31 = uint2(cell3 + int2(1, 0));
                uint cy31 = cell31.y;
                uint ch31a = cy31 + (cy31 << 10u);
                uint ch31b = ch31a ^ (ch31a >> 6u);
                uint ch31c = ch31b + (ch31b << 3u);
                uint ch31d = ch31c ^ (ch31c >> 11u);
                bool flag31 = flag30 ? flag30 : true;
                uint s31a = flag30 ? s30a : 1664525u;
                uint s31b = flag30 ? s30b : 1013904223u;
                uint hh31 = (((cell31.x * s31a) + (ch31d + (ch31d << 15u))) + s31b) * s31a;
                uint hh31a = hh31 ^ (hh31 >> 11u);
                uint hh31b = hh31a ^ ((hh31a << 7u) & 2636928640u);
                uint hh31c = hh31b ^ ((hh31b << 15u) & 4022730752u);
                uint2 cell32 = uint2(cell3 + int2(0, 1));
                uint cy32 = cell32.y;
                uint ch32a = cy32 + (cy32 << 10u);
                uint ch32b = ch32a ^ (ch32a >> 6u);
                uint ch32c = ch32b + (ch32b << 3u);
                uint ch32d = ch32c ^ (ch32c >> 11u);
                bool flag32 = flag31 ? flag31 : true;
                uint s32a = flag31 ? s31a : 1664525u;
                uint s32b = flag31 ? s31b : 1013904223u;
                uint hh32 = (((cell32.x * s32a) + (ch32d + (ch32d << 15u))) + s32b) * s32a;
                uint hh32a = hh32 ^ (hh32 >> 11u);
                uint hh32b = hh32a ^ ((hh32a << 7u) & 2636928640u);
                uint hh32c = hh32b ^ ((hh32b << 15u) & 4022730752u);
                uint2 cell33 = uint2(cell3 + int2(1, 1));
                uint cy33 = cell33.y;
                uint ch33a = cy33 + (cy33 << 10u);
                uint ch33b = ch33a ^ (ch33a >> 6u);
                uint ch33c = ch33b + (ch33b << 3u);
                uint ch33d = ch33c ^ (ch33c >> 11u);
                uint s33a = flag32 ? s32a : 1664525u;
                uint hh33 = (((cell33.x * s33a) + (ch33d + (ch33d << 15u))) + (flag32 ? s32b : 1013904223u)) * s33a;
                uint hh33a = hh33 ^ (hh33 >> 11u);
                uint hh33b = hh33a ^ ((hh33a << 7u) & 2636928640u);
                uint hh33c = hh33b ^ ((hh33b << 15u) & 4022730752u);
                float2 u3 = (f3 * f3) * (3.0f.xx - (f3 * 2.0f));
                float ux3 = u3.x;
                float omux3 = 1.0f - ux3;
                float uy3 = u3.y;
                bloom_gate_inner = ((((1.0f - uy3) * (((asfloat(((hh30c ^ (hh30c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux3) + ((asfloat(((hh31c ^ (hh31c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux3))) + (((asfloat(((hh32c ^ (hh32c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux3) * uy3)) + (((asfloat(((hh33c ^ (hh33c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux3) * uy3)) > 0.5f;
            }
            else
            {
                bloom_gate_inner = false;
            }
            bloom_gate = bloom_gate_inner;
        }
        else
        {
            bloom_gate = true;
        }
        if (bloom_gate)
        {
            int seed = int((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x);
            float3 pos = world_pos.xxy;
            pos.y = 0.0f;
            float3 pos_final;
            if (frame_constants_glade_theme_id == 4u)
            {
                float3 pos_theme4 = pos;
                pos_theme4.y = -0.100000001490116119384765625f;
                pos_final = pos_theme4;
            }
            else
            {
                pos_final = pos;
            }
            uint seed1b = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint sh1b = seed1b + (seed1b << 10u);
            uint sh2b = sh1b ^ (sh1b >> 6u);
            uint sh3b = sh2b + (sh2b << 3u);
            uint sh4b = sh3b ^ (sh3b >> 11u);
            float rand = asfloat(((sh4b + (sh4b << 15u)) & 8388607u) | 1065353216u) - 1.0f;
            float3 scale = (lerp(0.800000011920928955078125f, theme2 ? 1.2999999523162841796875f : 2.0f, rand) * lerp(0.5f, 1.0f, clamp(pad_score + bloom_score, 0.0f, 1.0f))).xxx * lerp(1.0f, 0.75f, bloom_score);
            uint seedx = uint(seed ^ 234);
            uint shx1 = seedx + (seedx << 10u);
            uint shx2 = shx1 ^ (shx1 >> 6u);
            uint shx3 = shx2 + (shx2 << 3u);
            uint shx4 = shx3 ^ (shx3 >> 11u);
            uint seedz = uint(seed ^ 8634);
            uint shz1 = seedz + (seedz << 10u);
            uint shz2 = shz1 ^ (shz1 >> 6u);
            uint shz3 = shz2 + (shz2 << 3u);
            uint shz4 = shz3 ^ (shz3 >> 11u);
            float3 dir = normalize(float3(asfloat(((shx4 + (shx4 << 15u)) & 8388607u) | 1065353216u) - 1.0f, 0.0f, asfloat(((shz4 + (shz4 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
            float4x4 basis = transpose(float4x4(float4(dir, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(dir, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
            float yaw_angle = 3.1400001049041748046875f * rand;
            float3 yaw_axis = normalize(float3(0.0f, 1.0f, 0.0f));
            float sin_yaw = sin(yaw_angle);
            float cos_yaw = cos(yaw_angle);
            float omc_yaw = 1.0f - cos_yaw;
            float yax = yaw_axis.x;
            float rt01 = omc_yaw * yax;
            float yay = yaw_axis.y;
            float rt02 = rt01 * yay;
            float yaz = yaw_axis.z;
            float rt03 = yaz * sin_yaw;
            float rt04 = omc_yaw * yaz;
            float rt05 = rt04 * yax;
            float rt06 = yay * sin_yaw;
            float rt07 = omc_yaw * yay;
            float rt08 = rt07 * yaz;
            float rt09 = yax * sin_yaw;
            float4x4 m = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, pos_final.x), float4(0.0f, 1.0f, 0.0f, pos_final.y), float4(0.0f, 0.0f, 1.0f, pos_final.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(basis, float4x4(float4(scale.x, 0.0f, 0.0f, 0.0f), float4(0.0f, scale.y, 0.0f, 0.0f), float4(0.0f, 0.0f, scale.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((rt01 * yax) + cos_yaw, rt02 - rt03, rt05 + rt06, 0.0f), float4(rt02 + rt03, (rt07 * yay) + cos_yaw, rt08 - rt09, 0.0f), float4(rt05 - rt06, rt08 + rt09, (rt04 * yaz) + cos_yaw, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint base;
            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, base);
            uint base_u = uint(int(base));
            float data[12] = { m[0].x, m[1].x, m[2].x, m[0].y, m[1].y, m[2].y, m[0].z, m[1].z, m[2].z, m[0].w, m[1].w, m[2].w };
            Affine3Packed affine = { data };
            LilypadXform lilypad = { affine };
            [unroll]
            for (int i = 0; i < 12; i++)
            {
                __alias_lilypad_buffer.Store<float>(i * 4 + base_u * 48 + 0, lilypad.xform.data[i]);
            }
            uint seedf = seed1b + 345u;
            uint shf1 = seedf + (seedf << 10u);
            uint shf2 = shf1 ^ (shf1 >> 6u);
            uint shf3 = shf2 + (shf2 << 3u);
            uint shf4 = shf3 ^ (shf3 >> 11u);
            if ((int(bloom_score > 0.699999988079071044921875f) & int((asfloat(((shf4 + (shf4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.20000000298023223876953125f)) != 0)
            {
                uint fbase;
                __alias_draw_commands.InterlockedAdd(304, 1u, fbase);
                uint seedg = (10u * seed1b) + 45u;
                uint shg1 = seedg + (seedg << 10u);
                uint shg2 = shg1 ^ (shg1 >> 6u);
                uint shg3 = shg2 + (shg2 << 3u);
                uint shg4 = shg3 ^ (shg3 >> 11u);
                float grow = clamp(1.0f - smoothstep(((asfloat(((shg4 + (shg4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.20000000298023223876953125f) - 0.20000000298023223876953125f, 1.0f, animation[gl_GlobalInvocationID.xy].xxxx.x), 0.0f, 1.0f);
                float pop = ((grow * grow) * grow) - ((grow * sin(grow * 3.1400001049041748046875f)) * 1.5f);
                float3 flower_scale = (scale * (bloom_score * 0.800000011920928955078125f)) * (1.0f - pop);
                float3 flower_scale_theme;
                if (theme2)
                {
                    flower_scale_theme = flower_scale * 2.0f;
                }
                else
                {
                    flower_scale_theme = flower_scale;
                }
                uint seedjx = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 2u)) + 345u;
                uint shjx1 = seedjx + (seedjx << 10u);
                uint shjx2 = shjx1 ^ (shjx1 >> 6u);
                uint shjx3 = shjx2 + (shjx2 << 3u);
                uint shjx4 = shjx3 ^ (shjx3 >> 11u);
                uint seedjz = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 3u)) - 345u;
                uint shjz1 = seedjz + (seedjz << 10u);
                uint shjz2 = shjz1 ^ (shjz1 >> 6u);
                uint shjz3 = shjz2 + (shjz2 << 3u);
                uint shjz4 = shjz3 ^ (shjz3 >> 11u);
                uint seedjy = (gl_GlobalInvocationID.x * 10u) + (gl_GlobalInvocationID.y * 345u);
                uint shjy1 = seedjy + (seedjy << 10u);
                uint shjy2 = shjy1 ^ (shjy1 >> 6u);
                uint shjy3 = shjy2 + (shjy2 << 3u);
                uint shjy4 = shjy3 ^ (shjy3 >> 11u);
                float4x4 fm = mul(float4x4(float4(1.0f, 0.0f, 0.0f, pos_final.x + lerp(-0.300000011920928955078125f, 0.300000011920928955078125f, asfloat(((shjx4 + (shjx4 << 15u)) & 8388607u) | 1065353216u) - 1.0f)), float4(0.0f, 1.0f, 0.0f, (pos_final.y - (pop * 0.300000011920928955078125f)) + ((asfloat(((shjy4 + (shjy4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.100000001490116119384765625f)), float4(0.0f, 0.0f, 1.0f, pos_final.z + lerp(-0.300000011920928955078125f, 0.300000011920928955078125f, asfloat(((shjz4 + (shjz4 << 15u)) & 8388607u) | 1065353216u) - 1.0f)), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(basis, float4x4(float4(flower_scale_theme.x, 0.0f, 0.0f, 0.0f), float4(0.0f, flower_scale_theme.y, 0.0f, 0.0f), float4(0.0f, 0.0f, flower_scale_theme.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                uint fbase_u = uint(int(fbase));
                float fdata[12] = { fm[0].x, fm[1].x, fm[2].x, fm[0].y, fm[1].y, fm[2].y, fm[0].z, fm[1].z, fm[2].z, fm[0].w, fm[1].w, fm[2].w };
                Affine3Packed faffine = { fdata };
                LilypadXform flower = { faffine };
                [unroll]
                for (int i = 0; i < 12; i++)
                {
                    __alias_lilypad_flower_buffer.Store<float>(i * 4 + fbase_u * 48 + 0, flower.xform.data[i]);
                }
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
