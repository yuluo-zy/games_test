// ==RECONSTRUCTED== shaders/skydome.raster.hlsl | F-E | bins: _skydome.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Skydome: a dome mesh is transformed by model_to_world; the ps blends a
// sun-disc + glow, a moon sprite (projected onto the dome), and a day/night
// sky gradient, then applies a gradient-noise dither.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 moon_tex_dims;
    float4x4 model_to_world;
    uint2 framebuffer_dims;
};

Texture2D<float4> moon_tex : register(t14, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD2;
    float bbx_y : TEXCOORD3;
};

struct VsOut
{
    float bbx_y : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float3 scaled = vin.Vertex_Position * 5000.0f;
    float4 world = mul(model_to_world, float4(scaled.x, scaled.y * 0.5f, scaled.z, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.bbx_y = vin.bbx_y;
    o.normal = vin.Vertex_Normal;
    o.world_pos = world.xyz;
    return o;
}

float4 ps_main(float3 world_pos : TEXCOORD2, float4 position : SV_Position) : SV_Target0
{
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - world_pos);

    float sun_angle = acos(-dot(dir_to_sun, view_dir));
    float sun_dist = max(0.0f, sun_angle - 0.02617994f);
    float glow_inner = 0.5f + 25.0f * sun_dist;
    float glow_outer = 1.0f + 5.0f * sun_dist;

    // Moon direction: contracted horizontally and faded toward day.
    float3 moon_dir = normalize(dir_to_moon * float2(lerp(2.5f, 1.0f, smoothstep(0.53f, 0.84f, time_of_day)), 1.0f).xyx);

    // Project the moon sprite onto the dome around moon_dir.
    float3 moon_light = 0.0f;
    float3 off = view_dir - moon_dir;
    float3 axis = normalize(cross(off, float3(0.0f, 1.0f, 0.0f)));
    float2 moon_uv = (float2(dot(moon_dir, axis), dot(moon_dir, normalize(cross(axis, off)))) * 35.0f) * 0.5f + 0.5f;
    float2 clamped = clamp(moon_uv, 0.0f, 1.0f);
    if (all(moon_uv == clamped))
    {
        float4 m = moon_tex.SampleLevel(sampler_lnc, moon_uv, 0.0f);
        moon_light = m.rgb * m.a * 2.0f;
    }

    float day_night = smoothstep(0.35f, 0.65f, time_of_day) - smoothstep(0.8f, 0.95f, time_of_day);

    float3 col = (
        (float3(1.0f, 0.7f, 0.2f) * (smoothstep(0.02617994f, 0.02443461f, sun_angle) * 30.0f))
        + (float3(1.0f, 0.65f, 0.2f) * (5.0f / (glow_inner * glow_inner)))
        + (float3(1.0f, 0.75f, 0.6f) * (0.8f / (glow_outer * glow_outer)))
        + (moon_light * (day_night * day_night * day_night))
    ) * ev_mult
    + (lerp(sky_low_color.xyz * 1.5f, sky_high_color.xyz,
            pow(clamp(1.0f - pow(1.0f - clamp(-view_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f)))
      * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f));

    // Slight saturation boost via luma-weighted lerp.
    float3 boosted = lerp(dot(col, float3(0.2126f, 0.7152f, 0.0722f)).xxx, col, 1.25f);

    // Gradient-noise dither (interleaved gradient noise).
    float noise = (frac(52.9829177f * frac(0.06711056f * position.x + 0.0058371499f * position.y)) * 2.0f) - 1.0f;
    float dither = (max(-1.0f, noise * rsqrt(abs(noise))) - float(int(sign(noise)))) * 0.0039215688f;

    return float4(boosted + clamp(boosted * 512.0f, 0.0f, 1.0f) * dither, 1.0f);
}
