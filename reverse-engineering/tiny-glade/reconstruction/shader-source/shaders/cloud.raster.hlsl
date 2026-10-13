// ==RECONSTRUCTED== shaders/cloud.raster.hlsl | F-E | bins: _cloud.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Clouds: instanced billboards on a shell around the camera. The ps samples one
// of 4 cloud textures (by instance quarter) at 7 texel offsets, resolves light
// against the sun/sky, and gates with the day/night window.

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
    uint2 cloud1_dims;
    uint2 cloud2_dims;
    uint2 cloud3_dims;
    uint2 cloud4_dims;
};

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

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
    uint h4 = hash(h3 + (h3 << 10u));

    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 5e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.01666667f;
    float3 center = (-facing) * (10.0f * lerp(400.0f, 800.0f, t * t));

    float height;
    if (glade_theme_id == 1u || glade_theme_id == 2u) height = -600.0f;
    else if (glade_theme_id == 3u) height = -500.0f;
    else height = -200.0f;
    if (glade_theme_id == 4u) height = -300.0f;
    center.y += height + 2000.0f * (rand01(h1) * rand01(h1));

    float2 quad_dims = (glade_theme_id == 4u) ? float2(1050.0f, 525.0f) : float2(1400.0f, 700.0f);
    uint type = int(frac(fi * 0.25f) * 4.0f);
    if (type == 1u) quad_dims *= 1.2f;
    else if (type == 2u) quad_dims *= 0.6f;
    else if (type == 3u) quad_dims *= 1.2f;

    float3 right = float3(facing.z, 0.0f, -facing.x);
    float3 world = center + (right * (quad_dims.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (quad_dims.y * quad.y));

    float2 uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    if (rand01(h4) > 0.5f)
    {
        uv.x = 1.0f - float(vertex_id & 1u);
        right = -right;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    if (instance_id > 60u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = uv;
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    uint type = int(frac(float(vin.instance) * 0.25f) * 4.0f);

    float x0 = vin.uv.x * (1.0f / 3.0f);
    float y0 = vin.uv.y * (1.0f / 3.0f);

    float s0, s1, s2, s3, s4, s5, s6;
    if (type == 1u)
    {
        s0 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud2.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud2.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 2u)
    {
        s0 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud3.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud3.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 3u)
    {
        s0 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud4.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud4.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else
    {
        s0 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud1.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud1.Sample(sampler_lnc, float2(x0, y0)).x;
    }

    if (s4 < 0.001f)
        discard;

    float3 forward = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 camera_world = cam4.xyz / cam4.w;

    float day_night = 1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f);
    float3 sun_light = sun_color.rgb * 0.1f * day_night;
    float3 sky = sky_high_color.rgb * 2.0f * day_night;

    float3 col = sky;
    col += sun_light * s6 * pow(max(dot(forward, vin.right), 0.0f), 2.0f);
    col += sun_light * s5 * pow(abs(min(dot(forward, vin.right), 0.0f)), 2.0f);
    col += sun_light * s3 * pow(max(forward.y, 0.0f), 2.0f);
    col += sun_light * s2 * pow(abs(min(forward.y, 0.0f)), 2.0f);
    col += sun_light * s0 * pow(max(dot(forward, vin.facing), 0.0f), 2.0f);
    col += sun_color.rgb * (smoothstep(0.9f, 1.0f, dot(forward, -normalize((camera_world / camera_world.xxx) - vin.world_pos))) * s1 * 0.5f);

    float alpha = (glade_theme_id == 3u) ? s4 * 0.5f : s4;
    float3 shaded = (glade_theme_id == 3u) ? col * 0.6f : col;
    return float4(shaded * alpha, alpha);
}
