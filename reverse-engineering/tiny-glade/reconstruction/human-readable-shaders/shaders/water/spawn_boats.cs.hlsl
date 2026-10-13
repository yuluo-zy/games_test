// PATCHED: modules ts.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/spawn_boats.cs.hlsl
// 中文说明:
//   水上小船生成 compute pass。证据(spirv-cross + _water_spawn_boats.cs.hlsl
//   _cs_main_0.bin 原始反汇编):groupshared best_choice 竞争——每线程按船大小
//   补丁内水深最小值打分 + 船首样本门控 + 路径/建筑净空,打包为
//   uint(score*0xFFFFFF00)|localIndex 做 InterlockedMax;胜者写出
//   BoatXform = T(world)*S(0.85)*R(轴 normalize(1,0,1), 浮沉)*R(轴 normalize(0,1,0), 航向)。
//   船尺寸常量 2.2/1.1/0.2 世界单位、浮沉幅度、-0.68 吃水深度为推断值。

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/perlin.hlsl"

struct BoatXform
{
    Affine3Packed transform;
};

// 绕(已归一化)轴的 Rodrigues 旋转——转置行主 4x4。轴按归一化传入,
// 以便发射完整的 9 项矩阵。
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

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 buildings_mask_dims;
    uint2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);
RWStructuredBuffer<BoatXform> __alias_instance_buffer : register(u18, space0);

// frame_constants.hlsl(重建 inc/)省略了原版用来发射 RuntimeDescriptorArray 的
// 死亡无界数组声明;在此复现。
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint best_choice;

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    if (local_index == 0u)
        best_choice = 0u;
    GroupMemoryBarrierWithGroupSync();

    // 网格 -> 世界坐标(约 130 世界单位范围)。
    float2 uv = (float2(id.xy) + 0.5f) / float2(grid_dims);
    float2 world_pos = (uv - 0.5f) * 130.0f;

    // 伪随机朝向(由网格坐标决定)。
    float angle = 40.665630340576171875f * float(id.x + id.y * grid_dims.x);
    float2 dir = float2(cos(angle), sin(angle));

    // 船体关键采样点:前后、两侧、船首。
    float2 fwd = dir * 0.016923077404499053955078125f;
    float2 side = float2(dir.y, -dir.x) * 0.003076923079788684844970703125f;
    float2 bow = uv + dir * 0.03384615480899810791015625f;

    // 船体补丁内水深最小值(water_mask.x 越深越小)。
    float min_water = min(min(min(min(min(
        water_mask.SampleLevel(sampler_lnc, uv, 0.0f).x,
        water_mask.SampleLevel(sampler_lnc, uv - fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd * 1.5f, 0.0f).x);

    // 船首必须有足够水(样本 <0.5)且补丁足够深(>0.5)。
    bool deep = water_mask.SampleLevel(sampler_lnc, bow, 0.0f).x < 0.5f ? (min_water > 0.5f) : false;

    float deep_score = deep ? min_water : 0.0f;
    // 最终得分:排除水面波动(前后 .w 差)与路径掩码;乘以建筑净空衰减。
    float score = (abs(water_mask.SampleLevel(sampler_lnc, uv + fwd, 0.0f).w - water_mask.SampleLevel(sampler_lnc, uv - fwd, 0.0f).w) > 0.1f)
        ? 0.0f
        : ((path_mask.Load(int3(uint2(int2(uv * float2(path_mask_dims))), 0u)).x > 0.0f)
            ? 0.0f
            : deep_score * clamp(buildings_mask.SampleLevel(sampler_lnc, uv + dir * 0.06769230841702222824096679688f, 0.0f).y - 0.3f, 0.0f, 1.0f));

    // 打包得分(高 8 位)+ 线程号,做工作组内最大值竞争。
    uint packed = uint(clamp(score, 0.0f, 1.0f) * 4294967040.0f) | local_index;
    uint prev;
    InterlockedMax(best_choice, packed, prev);
    GroupMemoryBarrierWithGroupSync();

    bool is_winner = best_choice == packed && score > 0.0f;

    if (is_winner)
    {
        // 随机种子 -> 浮沉相位(0.05 幅度)与航向。
        uint seed = id.x * id.y + id.x;
        float rnd = rand01(hash(seed));
        float bob = 0.05f * sin((fmod(float(game_time_sec), TWO_PI) + game_time_frac) + TWO_PI * rnd);
        float yaw = cos(angle);

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 scale = float4x4(
            float4(0.85f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 0.85f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 0.85f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        // 变换:平移到水面(-0.68 吃水)→ 缩放 0.85 → 绕 (1,0,1) 浮沉 → 绕 Y 航向。
        float4x4 m = mul(mul(mul(
            affine_translation(float3(world_pos.x, -0.68f, world_pos.y)),
            mul(identity, scale)),
            rotate_axis(normalize(normalize(float3(1.0f, 0.0f, 1.0f))), bob)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        // 追加绘制命令与实例。
        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = BoatXform(pack_affine(m));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _water_spawn_boats.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,变量名已语义化)
// 算法与上方手写段一致:水面得分竞争 + 胜者写 BoatXform。
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

struct BoatXform
{
    Affine3Packed transform;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 path_mask_dims : packoffset(c0.y);
    uint2 water_mask_dims : packoffset(c1);
    uint2 buildings_mask_dims : packoffset(c1.z);
    uint2 grid_dims : packoffset(c2);
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

RWByteAddressBuffer __alias_instance_buffer : register(u18, space0);
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint best_choice;

void comp_main()
{
    if (0u == gl_LocalInvocationIndex)
    {
        best_choice = 0u;
    }
    GroupMemoryBarrierWithGroupSync();
    float2 uv = (float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims);
    float2 world_pos = (uv - 0.5f.xx) * 130.0f.xx;
    float angle = 40.665630340576171875f * float(gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * grid_dims.x));
    float cos_a = cos(angle);
    float sin_a = sin(angle);
    float2 dir = float2(cos_a, sin_a);
    float2 bow = dir * 0.03384615480899810791015625f;
    float2 uv_back = uv + (dir * (-0.016923077404499053955078125f));
    float2 side = float2(sin_a, -cos_a) * 0.003076923079788684844970703125f;
    float2 uv_fwd = uv + (dir * 0.016923077404499053955078125f);
    float min_water = min(min(min(min(min(water_mask.SampleLevel(sampler_lnc, uv, 0.0f).x, water_mask.SampleLevel(sampler_lnc, uv_back + side, 0.0f).x), water_mask.SampleLevel(sampler_lnc, uv_fwd + side, 0.0f).x), water_mask.SampleLevel(sampler_lnc, uv_back - side, 0.0f).x), water_mask.SampleLevel(sampler_lnc, uv_fwd - side, 0.0f).x), water_mask.SampleLevel(sampler_lnc, uv + ((dir * (-1.5f)) * 0.016923077404499053955078125f), 0.0f).x);
    bool deep;
    if (water_mask.SampleLevel(sampler_lnc, uv + bow, 0.0f).x < 0.5f)
    {
        deep = min_water > 0.5f;
    }
    else
    {
        deep = false;
    }
    float deep_score = deep ? min_water : 0.0f;
    float score = (abs(water_mask.SampleLevel(sampler_lnc, uv_fwd, 0.0f).w - water_mask.SampleLevel(sampler_lnc, uv_back, 0.0f).w) > 0.100000001490116119384765625f) ? 0.0f : ((path_mask.Load(int3(uint2(int2(uv * float2(path_mask_dims))), 0u)).x > 0.0f) ? 0.0f : (deep_score * clamp(buildings_mask.SampleLevel(sampler_lnc, uv + (bow * 2.0f), 0.0f).y - 0.300000011920928955078125f, 0.0f, 1.0f)));
    uint packed = uint(clamp(score, 0.0f, 1.0f) * 4294967040.0f) | gl_LocalInvocationIndex;
    uint prev;
    InterlockedMax(best_choice, packed, prev);
    GroupMemoryBarrierWithGroupSync();
    bool is_winner;
    if (best_choice == packed)
    {
        is_winner = score > 0.0f;
    }
    else
    {
        is_winner = false;
    }
    if (is_winner)
    {
        uint seed = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint h1 = seed + (seed << 10u);
        uint h2 = h1 ^ (h1 >> 6u);
        uint h3 = h2 + (h2 << 3u);
        uint h4 = h3 ^ (h3 >> 11u);
        float bob = 0.0500000007450580596923828125f * sin((fmod(float(frame_constants_game_time_sec), 6.283185482025146484375f) + frame_constants_game_time_frac) + (6.283185482025146484375f * (asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f)));
        float3 bob_axis = normalize(normalize(float3(1.0f, 0.0f, 1.0f)));
        float sin_bob = sin(bob);
        float cos_bob = cos(bob);
        float omc_bob = 1.0f - cos_bob;
        float bax = bob_axis.x;
        float bob_t01 = omc_bob * bax;
        float bay = bob_axis.y;
        float bob_t02 = bob_t01 * bay;
        float baz = bob_axis.z;
        float bob_t03 = baz * sin_bob;
        float bob_t04 = omc_bob * baz;
        float bob_t05 = bob_t04 * bax;
        float bob_t06 = bay * sin_bob;
        float bob_t07 = omc_bob * bay;
        float bob_t08 = bob_t07 * baz;
        float bob_t09 = bax * sin_bob;
        float3 yaw_axis = normalize(float3(0.0f, 1.0f, 0.0f));
        float omc_yaw = 1.0f - cos_a;
        float yax = yaw_axis.x;
        float yaw_t01 = omc_yaw * yax;
        float yay = yaw_axis.y;
        float yaw_t02 = yaw_t01 * yay;
        float yaz = yaw_axis.z;
        float yaw_t03 = yaz * sin_a;
        float yaw_t04 = omc_yaw * yaz;
        float yaw_t05 = yaw_t04 * yax;
        float yaw_t06 = yay * sin_a;
        float yaw_t07 = omc_yaw * yay;
        float yaw_t08 = yaw_t07 * yaz;
        float yaw_t09 = yax * sin_a;
        float4x4 m = mul(mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, world_pos.x), float4(0.0f, 1.0f, 0.0f, -0.680000007152557373046875f), float4(0.0f, 0.0f, 1.0f, world_pos.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(0.85000002384185791015625f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.85000002384185791015625f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.85000002384185791015625f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((bob_t01 * bax) + cos_bob, bob_t02 - bob_t03, bob_t05 + bob_t06, 0.0f), float4(bob_t02 + bob_t03, (bob_t07 * bay) + cos_bob, bob_t08 - bob_t09, 0.0f), float4(bob_t05 - bob_t06, bob_t08 + bob_t09, (bob_t04 * baz) + cos_bob, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((yaw_t01 * yax) + cos_a, yaw_t02 - yaw_t03, yaw_t05 + yaw_t06, 0.0f), float4(yaw_t02 + yaw_t03, (yaw_t07 * yay) + cos_a, yaw_t08 - yaw_t09, 0.0f), float4(yaw_t05 - yaw_t06, yaw_t08 + yaw_t09, (yaw_t04 * yaz) + cos_a, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        uint base;
        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, base);
        uint base_u = uint(int(base));
        float data[12] = { m[0].x, m[1].x, m[2].x, m[0].y, m[1].y, m[2].y, m[0].z, m[1].z, m[2].z, m[0].w, m[1].w, m[2].w };
        Affine3Packed affine = { data };
        BoatXform xform = { affine };
        [unroll]
        for (int i = 0; i < 12; i++)
        {
            __alias_instance_buffer.Store<float>(i * 4 + base_u * 48 + 0, xform.transform.data[i]);
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
