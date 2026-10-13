// PATCHED: modules ds.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/spawn_reeds.cs.hlsl
// 中文说明:
//   芦苇生成 compute pass。证据(spirv-cross + _water_spawn_reeds.cs.hlsl_cs_main_0.bin
//   原始反汇编):抖动世界位置;water-mask 高度越界检查采样;水位带在 0.5 附近
//   衰减 + 路径带在 0.2 附近衰减;perlin(world+20)>0.6 && water>0.55 &&
//   buildings<0.1 门控;Reed = T*S*R(yaw)。

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

struct Reed
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

// 以 `center` 为中心的带状衰减:中心为 1,距离 >= half_width 时为 0。
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
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t17, space0);
RWStructuredBuffer<Reed> __alias_instance_buffer : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint seed = id.x * id.y + id.x;
    float rnd = rand01(hash(seed));

    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-0.4f, 0.4f, rnd).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;

    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;
    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;

    float water_band = band_falloff(abs(water - 0.5f), 3.3333332538604736328125f);
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float score = (water_band + path_band * smoothstep(0.6f, 0.8f, water)) - smoothstep(0.2f, 0.25f, path);

    if (score > 0.0f)
    {
        if (!(perlin(world_pos + 20.0f) > 0.6f && water > 0.55f
              && buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x < 0.1f))
            return;

        float scale = lerp(1.0f, 1.5f, rand01(hash(id.x + id.y)));
        float yaw = 3.14f * rnd;

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 s = float4x4(
            float4(scale, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale * lerp(0.5f, 1.0f, score * score), 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 m = mul(mul(
            affine_translation(float3(world_pos.x, -1.0f, world_pos.y)),
            mul(identity, s)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = Reed(pack_affine(m));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _water_spawn_reeds.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,变量名已语义化)
// 算法与上方手写段一致:抖动位置 -> 水位带/路径带/噪声门控 -> Reed = T*S*R(yaw)。
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 water_mask_dims : packoffset(c0.y);
    uint2 path_mask_dims : packoffset(c1);
    uint2 buildings_mask_non_blurred_dims : packoffset(c1.z);
    int2 grid_dims : packoffset(c2);
};

RWByteAddressBuffer __alias_instance_buffer : register(u18, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t17, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 uv;
    uint seed0 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
    uint h1 = seed0 + (seed0 << 10u);
    uint h2 = h1 ^ (h1 >> 6u);
    uint h3 = h2 + (h2 << 3u);
    uint h4 = h3 ^ (h3 >> 11u);
    float rnd = asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 world_pos = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-0.4000000059604644775390625f, 0.4000000059604644775390625f, rnd).xx;
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
    float4 path_sample = path_mask.SampleLevel(sampler_llb, uv, 0.0f);
    float path_val = path_sample.x;
    float water_band;
    do
    {
        float w_dist = abs(water - 0.5f);
        if (w_dist > 0.300000011920928955078125f)
        {
            water_band = 0.0f;
            break;
        }
        float w_norm = w_dist * 3.3333332538604736328125f;
        water_band = 1.0f - ((w_norm * w_norm) * (3.0f - (w_dist * 6.666666507720947265625f)));
        break;
    } while(false);
    float path_band;
    do
    {
        float p_dist = abs(path_val - 0.20000000298023223876953125f);
        if (p_dist > 0.20000000298023223876953125f)
        {
            path_band = 0.0f;
            break;
        }
        float p_norm = p_dist * 5.0f;
        path_band = 1.0f - ((p_norm * p_norm) * (3.0f - (p_dist * 10.0f)));
        break;
    } while(false);
    float gate_score = (water_band + (path_band * smoothstep(0.60000002384185791015625f, 0.800000011920928955078125f, water))) - smoothstep(0.20000000298023223876953125f, 0.25f, path_val);
    bool perlin_gate;
    if (gate_score > 0.0f)
    {
        float2 perlin_pos = world_pos + 20.0f.xx;
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
        perlin_gate = ((int(((((1.0f - uy) * (((asfloat(((hh00c ^ (hh00c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux) + ((asfloat(((hh10c ^ (hh10c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux))) + (((asfloat(((hh01c ^ (hh01c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * omux) * uy)) + (((asfloat(((hh11c ^ (hh11c >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux) * uy)) > 0.60000002384185791015625f) & int(water > 0.550000011920928955078125f)) & int(buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x < 0.100000001490116119384765625f)) != 0;
    }
    else
    {
        perlin_gate = false;
    }
    if (perlin_gate)
    {
        uint seed1 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
        uint sh1 = seed1 + (seed1 << 10u);
        uint sh2 = sh1 ^ (sh1 >> 6u);
        uint sh3 = sh2 + (sh2 << 3u);
        uint sh4 = sh3 ^ (sh3 >> 11u);
        float scale = lerp(1.0f, 1.5f, asfloat(((sh4 + (sh4 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
        float yaw_angle = 3.1400001049041748046875f * rnd;
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
        float4x4 m = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, world_pos.x), float4(0.0f, 1.0f, 0.0f, -1.0f), float4(0.0f, 0.0f, 1.0f, world_pos.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale * lerp(0.5f, 1.0f, gate_score * gate_score), 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((rt01 * yax) + cos_yaw, rt02 - rt03, rt05 + rt06, 0.0f), float4(rt02 + rt03, (rt07 * yay) + cos_yaw, rt08 - rt09, 0.0f), float4(rt05 - rt06, rt08 + rt09, (rt04 * yaz) + cos_yaw, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        uint base;
        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, base);
        uint base_u = uint(int(base));
        float data[12] = { m[0].x, m[1].x, m[2].x, m[0].y, m[1].y, m[2].y, m[0].z, m[1].z, m[2].z, m[0].w, m[1].w, m[2].w };
        Affine3Packed affine = { data };
        Reed reed = { affine };
        [unroll]
        for (int i = 0; i < 12; i++)
        {
            __alias_instance_buffer.Store<float>(i * 4 + base_u * 48 + 0, reed.xform.data[i]);
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
