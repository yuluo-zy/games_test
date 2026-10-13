// ==RECONSTRUCTED== shaders/flag_roof_cloth.raster.hlsl | F-V | bins: _flag_roof_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Roof-top flag cloth (roof family). The vs blends between a rest pose and an
// animated pose read from anim textures (positions + normals), keyed by a sway
// envelope driven by an origin-anchored gust noise; rest_pos/rest_normal come
// from the CPU side. ps samples the flag pattern + palette (border_mask picks
// solid-vs-pattern) and bakes normal+albedo into the solid gbuffer word.
// $Globals (set0/b13) holds the per-object transform/color/seed + texture dims.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl carries only s0..s4,s9..s12. Declaring s5..s8
// here restores the 13 consecutive sampler slots that position $Globals at
// set0/b13.

// Sampler and animation textures used by the vertex stage.
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

// Textures used by the fragment stage.
Texture2D<float4> border_mask : register(t17, space0);
Texture2D<float4> flag_palettes : register(t18, space0);
Texture2DArray<float4> flag_patterns : register(t19, space0);

int seed;
int color_id;
float4x4 model_to_world;
uint2 flag_color_dims;
uint2 anim_texture_dims;
uint2 anim_normal_texture_dims;
uint2 border_mask_dims;
uint2 flag_palettes_dims;
int bool_render_shadows;

// 2D value noise with factored bilinear interpolation (matches the original
// compiled form; inc/perlin.hlsl's perlin() uses the lerp form instead).
float flag_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    return ((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
        + ((n01 * (1.0f - u.x)) * u.y)
        + ((n11 * u.x) * u.y);
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    float3 rest_pos : TEXCOORD4;
    float3 rest_normal : TEXCOORD5;
    float3 uv2 : TEXCOORD6;
    uint vertex_id : SV_VertexID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float t = float(game_time_sec) + game_time_frac;
    float r0 = rand01(hash(uint(seed)));

    // Animation uv: per-vertex row, time/rand column.
    float2 anim_uv = float2((float(vin.vertex_id) + 0.5f) * 0.0125f, (t * 0.15f) + (r0 * 0.2f));

    // Gust sway envelope.
    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float sway = clamp(smoothstep(0.0f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) + 0.7f), 0.0f, 1.0f);

    uint2 vidx = uint2(uint(vin.vertex_id), 0u);
    float3 anim_pos = (anim_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_texture.Load(int3(vidx, 0u)).xyz) * 2.0f * 0.8f;
    float3 anim_nrm = anim_normal_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_normal_texture.Load(int3(vidx, 0u)).xyz;

    float4 wp = mul(model_to_world, float4(lerp(vin.rest_pos * 0.8f, (vin.Vertex_Position * 0.8f) + anim_pos, sway), 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = normalize(mul(model_to_world, float4(lerp(vin.rest_normal, vin.Vertex_Normal + anim_nrm, sway), 0.0f)).xyz);
    o.C2 = wp.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = float2(((vin.uv2.x - 0.5f) * 1.2f) + 0.5f, vin.uv2.y * 7.0f);
    o.C5 = ((float2((vin.uv2.y * 4.0f) - 2.75f, vin.uv2.x) - 0.5f) * 1.3f) + 0.5f;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    // View position and normal facing.
    float3 nrm;
    float4 origin = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (dot(normalize((origin.xyz / origin.w) - pin.C2), pin.C1) <= 0.0f)
    {
        nrm = -pin.C1;
    }
    else
    {
        nrm = pin.C1;
    }

    float layer = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float4 pattern = flag_patterns.SampleBias(sampler_llr, float3(pin.C4.x, 1.0f - pin.C4.y, layer), global_mip_bias);

    uint cid = uint(color_id);
    float3 albedo;
    if (border_mask.SampleBias(sampler_llc, pin.C3, global_mip_bias).x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern.x).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz,
                      (flag_patterns.SampleLevel(sampler_llc, float3(pin.C5, float((seed % 6) + 3)), 0.0f).x).xxx);
    }

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 100663296u;
    return o;
}
