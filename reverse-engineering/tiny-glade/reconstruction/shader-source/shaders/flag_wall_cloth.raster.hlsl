// ==RECONSTRUCTED== shaders/flag_wall_cloth.raster.hlsl | F-V | bins: _flag_wall_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Wall banner cloth (roof family). The vs bends the flag with a gust envelope
// (origin-anchored 2D value noise) plus a faster flutter term; vertices are
// split between body/tip by is_tip, and corner flags are flattened (abs trick)
// with a smaller bend amplitude. ps samples the flag pattern + palette, using
// border_mask to pick solid-vs-pattern, and bakes normal+albedo+alpha into the
// solid gbuffer word. $Globals (set0/b13) holds the per-object transform/color.

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

// Textures used by the fragment stage (samplers come from inc/samplers.hlsl).
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

float4x4 model_to_world;
int seed;
float local_start_y;
float body_end_y;
float tip_end_y;
int is_corner;
int color_id;
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
    int is_tip : TEXCOORD4;
    float bby : TEXCOORD5;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float3 vpos = vin.Vertex_Position;
    float3 pos;
    if (vin.is_tip == 0)
    {
        pos = vpos;
        pos.y = -lerp(local_start_y, body_end_y, 1.0f - vin.bby);
    }
    else
    {
        pos = vpos;
        pos.y = -lerp(body_end_y, tip_end_y, 1.0f - vin.bby);
    }

    float t_sec = float(game_time_sec);
    float t = t_sec + game_time_frac;

    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float r0 = rand01(hash(uint(seed)));
    float bend = lerp(0.3f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) * 0.3f);

    float amp;
    if (is_corner == 0)
    {
        amp = bend * 0.3f;
    }
    else
    {
        amp = bend * 0.5f;
    }

    float flutter = flag_noise(float2(t + pos.x, pos.y));
    float z = pos.z + (sin((fmod(t_sec * 1.8f, TWO_PI) + (game_time_frac * 1.8f)) + (pos.y * 0.5f)
        + (flutter * smoothstep(0.0f, -3.0f, pos.y)))) * amp;
    float3 result = pos;
    result.z = z;
    if (is_corner == 0)
    {
        result.z = abs(z + 0.05f) - 0.05f;
    }

    float4 wp = mul(model_to_world, float4(result, 1.0f));
    float body_h = body_end_y - local_start_y;
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = float2(vpos.x + 0.5f, (result.y * 1.5f) + 0.5f);
    o.C4 = vin.Vertex_UV;
    o.C5 = (float2(vpos.x, result.y + (body_h * 0.5f) + 0.2f) * (0.8f / min(max(0.7f, body_h - 0.5f), 1.0f))) + 0.5f;
    o.C6 = (float2(vpos.x, (tip_end_y - local_start_y) + result.y) * 0.75f) + float2(0.5f, 0.0f);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    float layer1 = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float layer2 = float(int(round(lerp(0.0f, 2.0f, rand01(hash(uint(seed ^ 794)))))));
    float4 pat1 = flag_patterns.SampleBias(sampler_llr, float3(pin.C3.x, 1.0f - pin.C3.y, layer1), global_mip_bias);
    float4 pat2 = flag_patterns.SampleBias(sampler_llc, float3(pin.C6.x, 1.0f - pin.C6.y, layer2), global_mip_bias);
    float border = border_mask.SampleBias(sampler_llc, pin.C4, global_mip_bias).x;
    float pattern_sum = pat1.x + pat2.x;
    float pat3 = flag_patterns.SampleLevel(sampler_llc, float3(pin.C5.x, 1.0f - pin.C5.y, float((seed % 6) + 3)), 0.0f).x;

    uint cid = uint(color_id);
    float3 albedo;
    if (border > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern_sum).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, (pat3).xxx);
    }

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

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    float alpha = (((border + pattern_sum) + pat3) > 0.89999997615814208984375f) ? 0.6f : 1.0f;
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(alpha, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
    return o;
}
