// ==RECONSTRUCTED== shaders/painterly/generate_strokes.cs.hlsl | evidence: _painterly_generate_strokes.cs.hlsl_cs_main_0.bin — one stroke per invocation (gid.x in [0,strokes_this_frame)); golden-ratio rnd2 pos + depth-relative jitter (1024+n*839 mod 1024 seed), 8x4 rotation search minimizing RGB error, srgb encode, pack into PainterlyStrokePacked at (strokes_captured+gid.x)*16; push_constants{strokes_captured,strokes_this_frame,current_t}; LocalSize 32,1,1; caps Shader | status: O (algorithm + pack from inlined SPIR-V); I (helper function names — inlined; stroke color alpha = 0xFF)
//
// Painterly thumbnail chain: procedural stroke generation. Each invocation
// fabricates one stroke whose color matches the target texture and whose
// rotation aligns with the local color gradient (min-error over 8 angles).

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "painterly/stroke.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 target_tex_dims;
    uint2 depth_target_tex_dims;
};

struct PushConstants
{
    uint strokes_captured;
    uint strokes_this_frame;
    float current_t;
};

[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> target_tex : register(t14, space0);
Texture2D<float4> depth_target_tex : register(t15, space0);
RWByteAddressBuffer __alias_strokes_buf : register(u16, space0);

[numthreads(32, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (dispatch_thread_id.x >= push_constants.strokes_this_frame)
    {
        return;
    }

    uint stroke_index = push_constants.strokes_captured + dispatch_thread_id.x;
    float n = float(stroke_index);

    // Golden-ratio low-discrepancy base position.
    float2 rnd = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * n) + 0.5f.xx);

    float aspect = float(target_tex_dims.x) / float(target_tex_dims.y);
    float2 aspect_vec = float2(aspect, 1.0f);
    float base_size = (0.0500000007450580596923828125f * sqrt(aspect)) *
                      lerp(1.2999999523162841796875f, 0.60000002384185791015625f, pow(clamp(n * 0.0009765625f, 0.0f, 1.0f), 1.0f));

    // Jitter the position toward a second golden-ratio point when the random
    // x coordinate is high, keeping the stroke on the same depth side.
    float2 pos;
    if (rnd.x > 0.300000011920928955078125f)
    {
        float2 jitter_seed = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) *
                                   float(1024u + ((stroke_index * 839u) % 1024u))) + 0.5f.xx);
        float2 candidate = frac(rnd + (((2.0f * base_size).xx / aspect_vec) * (jitter_seed - 0.5f.xx)));
        bool2 same_side = (depth_target_tex.SampleLevel(sampler_lnc, candidate, 0.0f).x >
                           depth_target_tex.SampleLevel(sampler_lnc, rnd, 0.0f).x) == (stroke_index < 341u).xx;
        pos = float2(same_side.x ? rnd.x : candidate.x, same_side.y ? rnd.y : candidate.y);
    }
    else
    {
        pos = rnd;
    }

    float4 target = target_tex.SampleLevel(sampler_lnc, pos, 0.0f);
    float2 centered = pos - 0.5f.xx;
    float2 doubled = centered * 2.0f.xx;

    float size = base_size * exp2(min(1.5f, (lerp(0.0f, 3.0f, clamp(dot(doubled, doubled), 0.0f, 1.0f)) +
                lerp(0.800000011920928955078125f, 0.0f, smoothstep(0.0f, 0.5f, pos.x))) +
                lerp(0.0f, 1.0f, clamp(abs((target.w * 2.0f) - 1.0f), 0.0f, 1.0f))));

    // Aspect-corrected centered position (photograph-space UV).
    float2 pos_uv = (((centered * float2(2.0f, -2.0f)) * aspect_vec) * float2(0.5f, -0.5f)) + 0.5f.xx;

    float3 stroke_color = linear_to_srgb(target.xyz);

    // Find the rotation angle whose 4 corner taps best match the target color.
    float angle = float(hash(stroke_index + 1u) % 360u) * 0.0174532942473888397216796875f;
    float best_angle = angle;
    float best_error = 10000000000.0f;
    for (uint k = 0u; k < 8u; k++)
    {
        float a = angle + (float(k) * 0.3926990926265716552734375f);
        float2x2 rot = float2x2(float2(cos(a), sin(a)), float2(-sin(a), cos(a)));
        float half_size = size * 0.5f;
        float2 off_x = mul(rot, float2(half_size, 0.0f)) / aspect_vec;
        float2 off_y = mul(rot, float2(0.0f, half_size)) / aspect_vec;

        float err = 0.0f;
        for (uint v = 0u; v < 2u; v++)
        {
            for (uint u = 0u; u < 2u; u++)
            {
                float2 sample_pos = pos + (off_x * ((float(u) * 2.0f) - 1.0f)) +
                                    (off_y * (((float(v) * 2.0f) - 1.0f) * 0.20000000298023223876953125f));
                float3 diff = target.xyz - target_tex.SampleLevel(sampler_lnc, sample_pos, 0.0f).xyz;
                err += dot(diff, diff);
            }
        }
        err *= 0.25f;
        if (err < best_error)
        {
            best_error = err;
            best_angle = a;
        }
    }

    // Shrink strokes on high-frequency areas of the depth-relative position.
    if (pos.x > 0.300000011920928955078125f)
    {
        if (best_error <= 0.20000000298023223876953125f)
        {
            size *= lerp(1.0f, 0.25f, smoothstep(0.0f, 0.20000000298023223876953125f, best_error));
        }
        else
        {
            size = 0.0f;
        }
    }

    PainterlyStrokePacked stroke;
    stroke.pos = pack_half2(pos_uv);
    stroke.color = pack_rgba8(float4(stroke_color, 1.0f));
    stroke.size_rotation = pack_half2(float2(size, best_angle));
    stroke.appear_t = push_constants.current_t;

    uint byte_offset = stroke_index * 16u;
    __alias_strokes_buf.Store<uint>(byte_offset + 0u, stroke.pos);
    __alias_strokes_buf.Store<uint>(byte_offset + 4u, stroke.color);
    __alias_strokes_buf.Store<uint>(byte_offset + 8u, stroke.size_rotation);
    __alias_strokes_buf.Store<float>(byte_offset + 12u, stroke.appear_t);
}
