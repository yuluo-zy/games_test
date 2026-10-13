// ==RECONSTRUCTED== shaders/star.raster.hlsl | F-E | bins: _star.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Stars: instanced procedural billboards scattered on a large shell around the
// camera; the ps samples the star texture, gated by view direction and the
// day/night time window.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 star_texture_dims;
};

Texture2D<float4> star_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));

    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 1e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.0001666667f;
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + 10000.0f * (rand01(h1) * rand01(h1))) - 1000.0f;

    float2 size = (float2(2.0f, 1.0f) * (lerp(1.0f, 0.2f, smoothstep(0.7f, 1.0f, rand01(h1))) * 10.0f)) * 3.0f
                * rand01(hash(h3));
    float3 right = float3(facing.z, 0.0f, -facing.x);

    float3 world = center + (right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y));

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    if (instance_id > 6000u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - vin.world_pos);

    // Night / dusk intensity windows around specific times of day.
    float d_night = abs(time_of_day - 0.9f);
    float night = (d_night > 0.1f) ? 0.0f : 1.0f - (d_night * 10.0f) * (d_night * 10.0f) * (3.0f - d_night * 20.0f);
    float d_dusk = abs(time_of_day - 0.45f);
    float dusk = (d_dusk > 0.15f) ? 0.0f : 1.0f - (d_dusk * 6.6667f) * (d_dusk * 6.6667f) * (3.0f - d_dusk * 13.333f);

    float occlusion = smoothstep(-1.0f, 1.0f, dot(view_dir, -mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz));
    float visible = smoothstep(0.4f, 0.6f, time_of_day) - smoothstep(0.8f, 0.92f, time_of_day);

    float star = star_texture.Sample(sampler_lnc, vin.uv).x * (1.0f - occlusion * (night + dusk)) * visible;
    return float4(star.xxx * (10.0f * ev_mult), star);
}
