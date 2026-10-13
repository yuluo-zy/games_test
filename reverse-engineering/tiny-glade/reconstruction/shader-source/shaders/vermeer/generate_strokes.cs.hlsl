// ==RECONSTRUCTED== shaders/vermeer/generate_strokes.cs.hlsl | evidence: _vermeer_generate_strokes.cs.hlsl_cs_main_2f7da8dc0793466b.bin (target variant; f890205ffc078ce8 is the background_tex feature variant, not targeted) — one stroke per invocation; golden-ratio base pos + trilinear hash3 jitter, mip-selected target color (edge = 20*length(abs(c-l)/(c+l))), 8x6 rotation search minimizing error, temporal blend against previous stroke when history_valid && prev_size>0, pack into VermeerStrokePacked at gid.x*16; push via $Globals b13 {max_strokes,wall_time_seconds,progress_t,history_valid,target_tex_dims,background_tex_dims}; LocalSize 32,1,1; caps Shader+RuntimeDescriptorArray | status: O (algorithm + pack from inlined SPIR-V); I (helper names — hash3/mip_sample inlined)
//
// Vermeer thumbnail chain: procedural stroke generation with temporal
// stability. Per stroke: pick a golden-ratio position, jitter it with a
// per-pixel trilinear hash, sample the target at a size-dependent mip, and
// search 8 angles for the orientation whose corner taps best match.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/perlin.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "vermeer/stroke.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint max_strokes;
    float wall_time_seconds;
    float progress_t;
    uint history_valid;
    uint2 target_tex_dims;
    uint2 background_tex_dims;
};

Texture2D<float4> target_tex : register(t14, space0);
RWByteAddressBuffer __alias_strokes_buf : register(u16, space0);

// Sample a texture with linear mip-level interpolation (floor/ceil blend).
float4 mip_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip)
{
    return lerp(tex.SampleLevel(samp, uv, floor(mip)), tex.SampleLevel(samp, uv, ceil(mip)), frac(mip).xxxx);
}

// Two-channel unit-hash of a uint3 corner (Wang-style 2D hash + 5-step).
float2 corner_noise(uint3 c)
{
    uint z = c.z;
    z += (z << 10u); z ^= (z >> 6u); z += (z << 3u); z ^= (z >> 11u);
    uint h = ((c.y * 1664525u) + (z + (z << 15u)) + 1013904223u) * 1664525u;
    h ^= (h >> 11u); h ^= ((h << 7u) & 2636928640u); h ^= ((h << 15u) & 4022730752u);
    h = ((c.x * 1664525u) + (h ^ (h >> 18u)) + 1013904223u) * 1664525u;
    h ^= (h >> 11u); h ^= ((h << 7u) & 2636928640u); h ^= ((h << 15u) & 4022730752u);
    uint r = h ^ (h >> 18u);
    uint hf = r + (r << 10u); hf ^= (hf >> 6u); hf += (hf << 3u); hf ^= (hf >> 11u); hf += (hf << 15u);
    return float2(asfloat((r & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat((hf & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// Smoothstep-trilinear blend of the 8 corner noises.
float2 hash3_noise(float3 p)
{
    uint3 c = uint3(floor(p));
    float3 f = frac(p);
    float3 w = f * f * (3.0f - (2.0f * f));

    float2 acc = 0.0f.xx;
    acc += corner_noise(c + uint3(0u, 0u, 0u)) * ((1.0f - w.x) * (1.0f - w.y) * (1.0f - w.z));
    acc += corner_noise(c + uint3(1u, 0u, 0u)) * (w.x * (1.0f - w.y) * (1.0f - w.z));
    acc += corner_noise(c + uint3(0u, 1u, 0u)) * ((1.0f - w.x) * w.y * (1.0f - w.z));
    acc += corner_noise(c + uint3(1u, 1u, 0u)) * (w.x * w.y * (1.0f - w.z));
    acc += corner_noise(c + uint3(0u, 0u, 1u)) * ((1.0f - w.x) * (1.0f - w.y) * w.z);
    acc += corner_noise(c + uint3(1u, 0u, 1u)) * (w.x * (1.0f - w.y) * w.z);
    acc += corner_noise(c + uint3(0u, 1u, 1u)) * ((1.0f - w.x) * w.y * w.z);
    acc += corner_noise(c + uint3(1u, 1u, 1u)) * (w.x * w.y * w.z);
    return acc;
}

[numthreads(32, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint stroke_index = dispatch_thread_id.x;
    uint n = stroke_index + 1u;
    float nf = float(stroke_index);

    // Previous stroke (for the temporal blend) stored in the same slot.
    float4 prev_color = unpack_rgba8(__alias_strokes_buf.Load<uint>(stroke_index * 16u + 4u));
    float prev_size = unpack_half_x(__alias_strokes_buf.Load<uint>(stroke_index * 16u + 8u) & 0xffffu);
    float prev_rotation = unpack_half_x((__alias_strokes_buf.Load<uint>(stroke_index * 16u + 8u) >> 16u) & 0xffffu);

    float2 rnd = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * nf) + 0.5f.xx);
    float2 aspect_vec = float2(float(target_tex_dims.x) / float(target_tex_dims.y), 1.0f);

    float rnd_x = abs((((rnd - 0.5f.xx) * 2.0f.xx).x));
    float rnd_x3 = (rnd_x * rnd_x) * rnd_x;
    float base_size = lerp(0.0599999986588954925537109375f, 0.13300000131130218505859375f,
                           1.0f - pow(clamp(float(n) / float(max_strokes), 0.0f, 1.0f), 0.014999999664723873138427734375f)) *
                      lerp(1.0f, 1.5f, rnd_x3);

    float size = base_size * exp2(min(1.5f, 0.0f));

    float tex_mip = max(0.0f, log2((0.0199999995529651641845703125f * float(target_tex_dims.y)) * base_size));
    float4 base = mip_sample(target_tex, sampler_lnc, rnd, tex_mip);
    float3 color = base.xyz;

    float luma_mip = tex_mip + 1.0f;
    float3 luma = mip_sample(target_tex, sampler_lnc, rnd, luma_mip).xyz;

    float edge = clamp(20.0f * length(abs(color - luma) / (color + luma)), 0.0f, 1.0f) * lerp(1.0f, 0.0f, rnd_x3);
    float scale = lerp(1.0f, 2.0f, rnd_x3);

    // Trilinear-hash jitter of the base position (time-animated z channel).
    float3 hash_pos = float3(rnd * 128.0f, (nf * 1.61803400516510009765625f) + (wall_time_seconds * 0.100000001490116119384765625f));
    float2 jitter = hash3_noise(hash_pos);
    float2 pos = rnd + (((jitter - 0.5f.xx) * size) * 0.1500000059604644775390625f);

    float neighbor_mip = max(0.0f, log2((0.0199999995529651641845703125f * float(target_tex_dims.y)) * size));

    bool use_history = (history_valid != 0u) && (prev_size > 0.0f);

    float angle = float(hash(n) % 360u) * 0.0174532942473888397216796875f;
    float best_angle = angle;
    float best_error = 10000000000.0f;
    for (uint k = 0u; k < 8u; k++)
    {
        float a = angle + (float(k) * 0.3926990926265716552734375f);
        float2x2 rot = float2x2(float2(cos(a), sin(a)), float2(-sin(a), cos(a)));
        float half_size = size * 0.5f;
        float2 off_x = mul(rot, float2(half_size, 0.0f)) / aspect_vec;
        float2 off_y = mul(rot, float2(0.0f, half_size)) / aspect_vec;
        float half_width = 0.5f * lerp(0.300000011920928955078125f, 1.0f, clamp(size, 0.0f, 1.0f));

        float err = 0.0f;
        for (uint v = 0u; v < 3u; v++)
        {
            for (uint u = 0u; u < 2u; u++)
            {
                float2 sample_pos = rnd + (off_x * (float(v) - 1.0f)) +
                                    (off_y * (((float(u) * 2.0f) - 1.0f) * half_width));
                float3 diff = color - mip_sample(target_tex, sampler_lnc, sample_pos, neighbor_mip).xyz;
                err += dot(diff, diff);
            }
        }
        err *= 0.16666667163372039794921875f;
        if (err < best_error)
        {
            best_error = err;
            best_angle = a;
        }
    }

    float stroke_alpha = clamp(2.0f * edge, 0.5f, 1.0f) *
                         lerp(1.0f, 0.0f, smoothstep(scale * 0.014999999664723873138427734375f, 0.02999999932944774627685546875f * scale, best_error));
    float3 stroke_color = linear_to_srgb(color);
    float4 current = float4(stroke_color, stroke_alpha);

    float display_size = size * exp2((0.89999997615814208984375f - edge) * 1.5f);

    float4 final_color;
    float final_size;
    float final_rotation;
    if (use_history)
    {
        float blend_t = lerp(0.100000001490116119384765625f, 0.5f, smoothstep(0.100000001490116119384765625f, 0.5f, length(prev_color - current)));
        final_color = lerp(prev_color, current, blend_t.xxxx);
        final_size = prev_size + (clamp(display_size - prev_size, -0.004999999888241291046142578125f, 0.004999999888241291046142578125f) * blend_t);
        final_rotation = prev_rotation + (clamp(best_angle - prev_rotation, -0.20000000298023223876953125f, 0.20000000298023223876953125f) * blend_t);
    }
    else
    {
        final_color = current;
        final_size = display_size;
        final_rotation = best_angle;
    }

    VermeerStrokePacked stroke;
    stroke.pos = pack_half2(pos);
    stroke.color = pack_rgba8(final_color);
    stroke.size_rotation = pack_half2(float2(final_size, final_rotation));
    stroke.appear_t = 1.0f;

    uint byte_offset = stroke_index * 16u;
    __alias_strokes_buf.Store<uint>(byte_offset + 0u, stroke.pos);
    __alias_strokes_buf.Store<uint>(byte_offset + 4u, stroke.color);
    __alias_strokes_buf.Store<uint>(byte_offset + 8u, stroke.size_rotation);
    __alias_strokes_buf.Store<float>(byte_offset + 12u, stroke.appear_t);
}
