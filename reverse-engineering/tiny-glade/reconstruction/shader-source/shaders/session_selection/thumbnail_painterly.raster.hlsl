// ==RECONSTRUCTED== shaders/session_selection/thumbnail_painterly.raster.hlsl | F-E | bins: _session_selection_thumbnail_painterly.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Painterly session thumbnails: instanced (6 verts/instance) tiles scattered
// around first_thumbnail_center, rotated and sized by hashed randomness; the VS
// samples the thumbnail and passes the tinted sprite through C1; the ps applies
// a brush texture mask.

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_llc s3)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
    float thumbnail_size;
    float hover;
    uint is_black_and_white;
    uint thumbnail_index;
};

Texture2D<float4> thumbnail_tex : register(t13, space0);
Texture2D<float4> brush_tex : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint instance = vertex_id / 6u;
    uint vid = vertex_id % 6u;

    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float fi = float(instance);
    float ang = fi * 0.5698403f;
    float s = sin(ang);
    float c = cos(ang);
    float2 rotated = mul(float2x2(float2(c, s), float2(-s, c)), corner.xy);
    float4 local = float4(rotated, corner.z, 1.0f - corner.w);

    uint seed = thumbnail_index + (thumbnail_index << 10u);
    seed = seed ^ (seed >> 6u);
    seed = seed + (seed << 3u);
    seed = seed ^ (seed >> 11u);
    uint rnd = ((instance * 1664525u) + (seed + (seed << 15u))) * 1664525u;
    rnd = rnd ^ (rnd >> 11u);
    rnd = rnd ^ ((rnd << 7u) & 2636928640u);
    rnd = rnd ^ ((rnd << 15u) & 4022730752u);
    uint rand_bits = rnd ^ (rnd >> 18u);

    float2 dir = float2(cos(fi * 4.7430363f), sin(fi * 4.7430363f)) * 0.5f;
    float p = fi * 0.0078125f;
    float size = lerp(0.5f, 0.75f, sqrt(p));

    uint h2 = rand_bits + (rand_bits << 10u);
    h2 = h2 ^ (h2 >> 6u);
    h2 = h2 + (h2 << 3u);
    h2 = h2 ^ (h2 >> 11u);
    bool grid = ((h2 + (h2 << 15u)) % 6u) == 0u;

    float2 offset;
    if (grid)
        offset = frac((float2(0.7548777f, 0.5698403f) * fi) + 0.5f) - 0.5f;
    else
        offset = dir * ((lerp(0.8f, 1.2f, rand01(rand_bits)) * 0.5f) * size / max(abs(dir.x), abs(dir.y)));

    float2 pos = (local.xy * thumbnail_size) * (lerp(0.25f, 0.12f, p) * 2.0f) + (offset * thumbnail_size) + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float4 clip = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    if ((instance % 2u) != 0u)
        clip = float4(0.0f, 0.0f, 0.0f, 0.0f);

    float4 sprite = thumbnail_tex.SampleLevel(sampler_lnc, offset + 0.5f, 0.0f);
    float4 sp = float4(sprite.rgb * sprite.a, sprite.a);

    float rim = size * 0.08f;
    float4 sprite_col;
    if (!grid)
    {
        float radius = 0.2f * size;
        float2 d = (abs(offset) - (0.5f * size).xx) + rim.xx;
        float dist = abs((length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f)) - rim);
        float m = (dist > radius) ? 0.0f : 1.0f - (dist / radius) * (dist / radius) * (3.0f - 2.0f * (dist / radius));
        sprite_col = sp * m;
    }
    else
        sprite_col = sp * 0.5f;

    sprite_col *= 0.75f;
    if (sprite_col.w < 1e-4f)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = local.zw;
    o.sprite = sprite_col;
    return o;
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
};

float4 ps_main(PsIn pin) : SV_Target0
{
    float4 brush = brush_tex.Sample(sampler_llc, pin.uv);
    float4 col = brush.xxxx * pin.sprite;
    float3 tinted = col.rgb * lerp(1.0f, 1.2f, hover);

    float4 out_col;
    if (is_black_and_white == 1u)
        out_col = float4(dot(tinted, float3(0.2126f, 0.7152f, 0.0722f)).xxx, col.w);
    else
        out_col = float4(tinted, col.w);
    return out_col;
}
