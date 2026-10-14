// ==RECONSTRUCTED== shaders/apply_screen_shadows.cs.hlsl
// 证据: _apply_screen_shadows.cs.hlsl_cs_main_0.bin (SPIR-V, 偏移 23)。
//   绑定: sampler_lnc s0, sampler_shadow_nearest s11, sampler_shadow_linear s12,
//   $Globals b14 (gbuffer_tex_dims/depth_tex_dims/history_tex_dims/reprojection_tex_dims/
//   history_valid/output_tex_dims), gbuffer_tex t15, depth_tex t16, history_tex t17,
//   reprojection_tex t18, shadow_map t19 (2DArray), output_tex u20 (unorm float),
//   __alias_shadow_mask_histogram u21 (RWStructuredBuffer<uint>), frame_constants set1/b0。
//   工作组尺寸 [16,16,1],入口 cs_main。
//   spirv-cross + 反汇编: PCSS 阴影重建。解码 gbuffer RGB8 法线,由深度重建世界坐标,
//   按斜率缩放法线偏移选取阴影级联,做 5 采样 blocker 搜索和 10 采样 PCF,
//   结合历史缓冲做基于方差的时域钳制,再按屏幕中央区域的分组存活归约
//   通过原子写入阴影掩码直方图 (RWStructuredBuffer)。
// 状态: O(结构来自反汇编;PCSS 内联以对齐操作码流)。
// 注: 内嵌 set 列出了 inc/history_valid.hlsl 与 inc/pack_unpack.hlsl,但重建的
//   inc 图中 pack_unpack -> bilinear -> history_valid 的 `history_valid(...)` 函数
//   与已验证的 $Globals 成员 `history_valid` 冲突(游戏也未内嵌 bilinear.hlsl)。
//   两个 include 均省略以保留已验证的成员名;unpack 已内联如下。

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/history_valid.hlsl"
#include "inc/shadow.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/cascades.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 gbuffer_tex_dims;
    uint2 depth_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint history_valid;
    uint2 output_tex_dims;
};

Texture2D<uint4> gbuffer_tex : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
Texture2D<float4> history_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t18, space0);
Texture2DArray<float4> shadow_map : register(t19, space0);
RWTexture2D<unorm float> output_tex : register(u20, space0);
RWStructuredBuffer<uint> __alias_shadow_mask_histogram : register(u21, space0);

groupshared uint thread_group_shadow_mask_sum;
groupshared uint thread_group_alive;

// PCSS 百分比接近软阴影。返回 (受光量, 受光量平方) 以及半影滤波尺度。
// 提前返回(背向太阳 / 无 blocker)形成反汇编中见到的 switch 形退出结构。
float2 sample_shadow_pcss(float3 world_pos,
                          float3 world_normal,
                          float3 sun_dir,
                          int2 pixel_pos,
                          uint frame_index,
                          out float penumbra)
{
    penumbra = 0.0f;

    // 级联 0 投影,带斜率缩放的法线偏移。
    float4 c0 = mul(view_constants[2].world_to_sample,
                    float4(world_pos + world_normal * (0.066666670143604278564453125f
                        * clamp(1.0f - dot(world_normal, sun_dir), 0.0f, 1.0f)), 1.0f));
    if (c0.z > 1.0f)
    {
        return float2(0.0f, 0.0f);
    }

    // 级联选择:对外层级联重新映射 UV / 纹素尺度 / 采样尺度。
    bool out0 = any(abs(c0.xy) > 0.983333349f);
    float2 uv1;
    if (out0)
    {
        uv1 = c0.xy * 0.5f;
    }
    else
    {
        uv1 = c0.xy;
    }
    float texel_scale = out0 ? 5.0f : 10.0f;
    float sample_scale = out0 ? 0.5f : 1.0f;
    uint slice0 = out0 ? 1u : 0u;
    bool out1 = any(abs(uv1) > (1.0f - (0.000833333353511989116668701171875f * (2.0f * texel_scale))));
    if (out1)
    {
        texel_scale = texel_scale * 0.2857142984867095947265625f;
        uv1 = uv1 * 0.2857142984867095947265625f;
        sample_scale = sample_scale * 0.2857142984867095947265625f;
    }
    uint slice = out1 ? 2u : slice0;

    // 最终阴影贴图投影(级联 2 + slice),带法线偏移。
    float4 c1 = float4(uv1.x, uv1.y, c0.z, c0.w)
        + mul(view_constants[2u + slice].world_to_sample,
              float4(world_normal * (0.066666670143604278564453125f
                  * clamp(1.0f - dot(world_normal, -sun_dir), 0.0f, 1.0f)) / sample_scale, 0.0f));
    float2 shadow_uv = c1.xy * float2(0.5f, -0.5f) + 0.5f;
    float bias = max((0.300000011920928955078125f / sample_scale) * (1.0f - abs(dot(world_normal, -sun_dir))),
                     0.02500000037252902984619140625f);

    // 逐像素螺旋种子 + 哈希,用于去相关 PCSS 采样点。
    float spiral_seed = float((uint(pixel_pos.x * 11 + pixel_pos.y * 7) + frame_index * 5u) % 64u) * 1.61803400516510009765625f;
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float hash_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_pos.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel_pos.y) + frame_rand))));

    // 用于斜率偏移的投影法线。
    float4 normal_proj = mul(view_constants[2u + slice].world_to_sample, float4(world_normal, 0.0f));
    float2 slope_bias = ((normal_proj.xy * float(int(sign(normal_proj.z)))) / max(0.1500000059604644775390625f, abs(normal_proj.z)).xx)
        * 0.0009765625f * 2.0f;

    // Blocker 搜索(5 个采样)。
    float blocker_depth = 0.0f;
    int blocker_count = 0;
    for (int i = 0; i < 5; ++i)
    {
        float angle = (float(i) + spiral_seed) * 2.3999631404876708984375f;
        float3 sample_pos = float3(shadow_uv.x, shadow_uv.y, c1.z) + float3(
            ((float2(cos(angle), sin(angle)) * (float(i) + hash_seed)) * 0.20000000298023223876953125f) * texel_scale * 0.000833333353511989116668701171875f,
            bias * (-0.0009765625f));
        float d = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sample_pos.xy, float(slice)), 0.0f).x;
        if (sample_pos.z > d)
        {
            blocker_depth += d;
            blocker_count++;
        }
    }

    float avg_blocker = blocker_depth / float(blocker_count);
    if (blocker_count == 0)
    {
        return float2(0.0f, 0.0f);
    }

    // 半影宽度(按雾尺度调制)。
    float penumbra_uv = max((c1.z - avg_blocker) * ((17.0666675567626953125f * texel_scale)
        * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w))) / avg_blocker, 0.0f);
    float filter_scale = (penumbra_uv / pow(pow(penumbra_uv / (texel_scale - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;

    // PCF(10 个采样),带斜率偏移。
    float2 lit = 0.0f;
    for (int j = 0; j < 10; ++j)
    {
        float angle = (float(j) + spiral_seed) * 2.3999631404876708984375f;
        float2 off = ((float2(cos(angle), sin(angle)) * (float(j) + hash_seed)) * 0.100000001490116119384765625f) * filter_scale;
        float3 sample_pos = float3(shadow_uv.x, shadow_uv.y, c1.z) + float3(off * 0.000833333353511989116668701171875f, bias * (-0.0009765625f));
        float s = 1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(sample_pos.xy, float(slice)), sample_pos.z + min(0.0f, dot(slope_bias, off)));
        lit += float2(s, s * s);
    }

    penumbra = filter_scale;
    return lit * 0.100000001490116119384765625f;
}

[numthreads(16, 16, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    bool is_first = (gl_LocalInvocationIndex == 0u);
    if (is_first)
    {
        thread_group_alive = 0u;
        thread_group_shadow_mask_sum = 0u;
    }
    GroupMemoryBarrierWithGroupSync();

    // 解码 gbuffer RGB8 法线并重建世界坐标。
    uint normal_packed = gbuffer_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x;
    float3 world_normal = (float4(float(normal_packed & 255u) * 0.0039215688593685626983642578125f,
                                  float((normal_packed >> 8u) & 255u) * 0.0039215688593685626983642578125f,
                                  float((normal_packed >> 16u) & 255u) * 0.0039215688593685626983642578125f,
                                  0.0f).xyz * 2.0f) - 1.0f;
    float3 sun_dir = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    int2 pixel_pos = int2(gl_GlobalInvocationID.xy);
    float2 depth_uv = (float2(pixel_pos) + 0.5f) / float2(depth_tex_dims);
    float2 clip_uv = depth_uv - 0.5f;
    float depth = depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x;
    float4 world_pos = mul(view_constants[0].view_to_world,
                           mul(view_constants[0].sample_to_view, float4(clip_uv * float2(2.0f, -2.0f), depth, 1.0f)));

    float penumbra;
    float2 shadow_result = sample_shadow_pcss(world_pos.xyz / world_pos.w, world_normal, sun_dir, pixel_pos, frame_index, penumbra);

    // 基于方差的钳制范围(削弱软阴影上的半影噪点)。
    float shadow_mask = shadow_result.x;
    float shadow_mask2 = shadow_result.y;
    float shadow_filtered = clamp((shadow_mask - 0.100000001490116119384765625f) * 1.111111164093017578125f, 0.0f, 1.0f);
    float penumbra_norm = penumbra * 0.20000000298023223876953125f;
    float2 shadow_range = shadow_filtered.xx + (float2(-1.0f, 1.0f) * sqrt(max(0.0f, shadow_mask2 - shadow_mask * shadow_mask)))
        * (3.0f * max(0.0f, 1.0f - penumbra_norm));

    // 对历史缓冲做时域重投影。
    float4 reproj = reprojection_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
    bool use_history;
    if (reproj.z > 0.0f)
    {
        if (history_valid != 0u)
        {
            use_history = !(0u == frame_index);
        }
        else
        {
            use_history = false;
        }
    }
    else
    {
        use_history = false;
    }

    float history_sample;
    if (use_history)
    {
        history_sample = history_tex.SampleLevel(sampler_lnc, depth_uv + reproj.xy, 0.0f).x;
    }
    else
    {
        history_sample = shadow_filtered;
    }

    float history_clamped = clamp(history_sample, shadow_range.x, shadow_range.y);
    float final_shadow = lerp(history_clamped, shadow_filtered, 0.125f);

    // 屏幕中央区域的存活归约,用于阴影掩码直方图。
    if (length(clip_uv * float2(float(depth_tex_dims.x) / float(depth_tex_dims.y), 1.0f)) < 0.449999988079071044921875f)
    {
        uint alive_prev;
        InterlockedAdd(thread_group_alive, 1u, alive_prev);
        uint sum_prev;
        InterlockedAdd(thread_group_shadow_mask_sum, clamp(uint(clamp(1.0f - final_shadow, 0.0f, 1.0f) * 256.0f), 0u, 255u), sum_prev);
    }
    GroupMemoryBarrierWithGroupSync();

    if (is_first)
    {
        if (thread_group_alive > 16u)
        {
            uint histogram_prev;
            InterlockedAdd(__alias_shadow_mask_histogram[clamp(uint(float(thread_group_shadow_mask_sum) / float(thread_group_alive)), 0u, 255u)],
                           1u, histogram_prev);
        }
    }

    output_tex[gl_GlobalInvocationID.xy] = final_shadow + 0.00196078442968428134918212890625f;
}
