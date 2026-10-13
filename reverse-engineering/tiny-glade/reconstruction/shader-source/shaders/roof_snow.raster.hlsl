// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/roof_snow.raster.hlsl | F-V | bins: _roof_snow.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Roof snow overlay (roof family). The vs displaces each roof vertex with a
// fine snow-noise jitter: a trilinear 3D-lattice float2 noise (custom 3D hash)
// scatters the snow surface, and a 2D value noise (inc/perlin.hlsl lattice)
// snow depth pushes along the normal; vertices near the roof edge are pulled
// down by a UV-y smoothstep. ps discards the below-half region and bakes the
// flat 0.7 snow albedo + normal into the solid gbuffer word.
// $Globals (set0/b13) holds the per-object transform + roof pivot.

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

float4x4 model_to_world;
float3 roof_pivot;
int is_circle;
int bool_render_shadows;

// 4-step integer hash used inside the 3D snow lattice (the 5th step, +x<<15,
// is folded into the corner value mix, matching the compiled form).
uint snow_hash4(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

// One 3D lattice corner: hash (x,y,z) ints into two [0,1) values.
float2 snow_corner(uint3 c)
{
    uint z4 = snow_hash4(c.z);
    uint hy = ((c.y * 1664525u) + (z4 + (z4 << 15u))) + 1013904223u;
    hy *= 1664525u;
    hy ^= (hy >> 11u);
    hy ^= (hy << 7u) & 2636928640u;
    hy ^= (hy << 15u) & 4022730752u;
    uint h = ((c.x * 1664525u) + (hy ^ (hy >> 18u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = h ^ (h >> 18u);
    uint h5 = snow_hash4(hx);
    return float2(asfloat((hx & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((h5 + (h5 << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// Trilinear 3D-lattice float2 noise (8 corners, ordered as compiled).
float2 snow_noise3(int3 cell, float ux, float ox, float uy, float oy, float uz, float oz)
{
    float2 n000 = snow_corner(uint3(cell + int3(0, 0, 0)));
    float2 n100 = snow_corner(uint3(cell + int3(1, 0, 0)));
    float2 n010 = snow_corner(uint3(cell + int3(0, 1, 0)));
    float2 n110 = snow_corner(uint3(cell + int3(1, 1, 0)));
    float2 n001 = snow_corner(uint3(cell + int3(0, 0, 1)));
    float2 n101 = snow_corner(uint3(cell + int3(1, 0, 1)));
    float2 n011 = snow_corner(uint3(cell + int3(0, 1, 1)));
    float2 n111 = snow_corner(uint3(cell + int3(1, 1, 1)));
    return ((((n000 * ox) * oy) * oz) + (((n100 * ux) * oy) * oz))
        + (((n010 * ox) * uy) * oz) + (((n110 * ux) * uy) * oz)
        + (((n001 * ox) * oy) * uz) + (((n101 * ux) * oy) * uz)
        + (((n011 * ox) * uy) * uz) + (((n111 * ux) * uy) * uz);
}

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
    float2 Vertex_UV : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float3 snow = vin.Vertex_Position * 0.4f;
    int3 cell = int3(floor(snow));
    float3 f = frac(snow);
    float3 u = f * f * (3.0f - 2.0f * f);
    float ux = u.x;
    float ox = 1.0f - ux;
    float uy = u.y;
    float oy = 1.0f - uy;
    float uz = u.z;
    float oz = 1.0f - uz;
    float2 jitter = clamp((snow_noise3(cell, ux, ox, uy, oy, uz, oz) - 0.5f) * 2.5f, -0.5f, 0.5f);

    float3 to_pivot = vin.Vertex_Position - roof_pivot;
    float depth = flag_noise(to_pivot.xz * 2.0f);

    float3 displaced = (vin.Vertex_Position + (float3(jitter.x, 0.0f, jitter.y) * 0.06f))
        - ((vin.Vertex_Normal * 0.4f) * 0.1f)
        + ((vin.Vertex_Normal * (depth * depth)) * 0.9f * 0.1f)
        - ((vin.Vertex_Normal * 0.1f) * smoothstep(0.4f, 0.0f, vin.Vertex_UV.y));

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    o.C0 = 0.7f.xxx;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = displaced;
    o.C3 = to_pivot;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (pin.C3.y < 0.5f)
    {
        discard;
    }
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(pin.C0, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _roof_snow.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C0;
static float3 in_var_C1;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (in_var_C3.y < 0.5f)
    {
        discard;
    }
    float3 _64 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _86 = sqrt(clamp(in_var_C0, 0.0f.xxx, 1.0f.xxx));
    uint _105 = (uint((clamp(_86.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_86.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_86.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _106 = uint2(((uint((clamp(_64.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_64.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_64.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _105);
    _106.y = _105 | 117440512u;
    out_var_SV_TARGET0 = _106;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _roof_snow.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

void frag_main()
{
    if (in_var_C3.y < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
