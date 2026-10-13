// ==RECONSTRUCTED== shaders/splash_particle.raster.hlsl | F-V | bins: _splash_particle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Splash particle: per-vertex position/color/normal/uv transformed by a global
// model_to_world; ps applies a 2-octave value-noise ring pattern (drop + ripple)
// animated by anim_t and time, discards outside the pattern, packs solid gbuffer
// (normal RGB8 + 0xFF000000, albedo RGB8 | material flags 0x46000000).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

float4x4 model_to_world;
int bool_render_shadows;
float anim_t;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world_pos = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(GetCurrentViewConstants().world_to_sample, world_pos);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world_pos.xyz;
    o.C3 = vin.Vertex_UV;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// Ring falloff: returns 1 at the ring center, fading to 0 at the radius edge.
float drop_ring(float d, float radius)
{
    if (d > radius)
    {
        return 0.0f;
    }
    float t = d / radius;
    return 1.0f - (t * t) * (3.0f - (2.0f * t));
}

PsOut ps_main(PsIn pin)
{
    PsOut o;
    static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
    uint2 frag = uint2(pin.position.xy) & uint2(1u, 1u);
    uint2 sub = hi_px_subpixels[frame_index & 3u];
    if (all(frag != sub))
    {
        discard;
    }

    float t = 1.0f - anim_t;
    float2 uv = pin.C3 * 25.6f;

    // Drop ring: 2D value-noise (LCG hash lattice, explicit bilinear blend).
    float2 npos1 = float2((uv.x * 2.0f) + pin.C2.x + pin.C2.z,
                          (uv.y * 0.5f) + (float(game_time_sec) + game_time_frac));
    int2 cell1 = int2(floor(npos1));
    float2 f1 = frac(npos1);
    float2 u1 = f1 * f1 * (3.0f - 2.0f * f1);
    float n00 = incb_perlin_hash01(hash(uint2(cell1)));
    float n10 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell1 + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 1))));
    float drop_blend = (1.0f - u1.y) * ((n00 * (1.0f - u1.x)) + (n10 * u1.x))
                     + (n01 * (1.0f - u1.x)) * u1.y
                     + (n11 * u1.x) * u1.y;
    float drop_radius = lerp(0.1f, 0.4f, drop_blend);

    float d = abs((pin.C3.y + lerp(-1.0f, 1.0f, (t * t * t))) - 0.5f);
    float drop = drop_ring(d, drop_radius);

    // Ripple ring: second value-noise field.
    float2 npos2 = float2((uv.x * 2.0f) + 2.0f + pin.C2.x + pin.C2.z,
                          uv.y * 0.2f);
    int2 cell2 = int2(floor(npos2));
    float2 f2 = frac(npos2);
    float2 u2 = f2 * f2 * (3.0f - 2.0f * f2);
    float m00 = incb_perlin_hash01(hash(uint2(cell2)));
    float m10 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 0))));
    float m01 = incb_perlin_hash01(hash(uint2(cell2 + int2(0, 1))));
    float m11 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 1))));
    float ripple_blend = (1.0f - u2.y) * ((m00 * (1.0f - u2.x)) + (m10 * u2.x))
                       + (m01 * (1.0f - u2.x)) * u2.y
                       + (m11 * u2.x) * u2.y;
    float ripple_w = lerp(0.0f, 0.3f, ripple_blend);
    if ((drop * (1.0f - smoothstep(0.9f, 1.0f, pin.C3.y + ripple_w))) < 0.5f)
    {
        discard;
    }

    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    float3 albedo = sqrt(clamp(pin.C0 * 0.7f, 0.0f, 1.0f));
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 1174405120u;
    return o;
}
