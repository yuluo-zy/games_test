// ==RECONSTRUCTED== shaders/instanced_roof_spire.raster.hlsl
// SOURCE PATH : shaders/instanced_roof_spire.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin
//   _instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin
//   pass/stage   : _instanced_roof_spire (roof) / raster
//   entry        : vs:vs_main; ps:ps_main
//   names        : ViewConstants, frame_constants, $Globals, RoofSpire, instanced_data, vs_main, ps_main
// Status: O (structure from disassembly: per-instance 32-byte RoofSpire record,
//   roof-profile-gated xz scaling, back-ease-out spire rise, a 2D value-noise
//   (inc/perlin) bend of world.y, an inline 3D float2 value noise for xz jitter,
//   constant dark-brown albedo; PS blends the vertex normal toward the derived
//   face normal and packs the solid gbuffer). I (variable naming, helper names —
//   the 3D float2 noise is inline in the original, no inc file provides it).
//   $Globals (b13) carries only current_view_index; the full inc/samplers.hlsl
//   set reserves set0 bindings 0..12, so the plain global auto-assigns to b13.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color_library.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Per-instance roof spire record (32 bytes).
struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
};

ByteAddressBuffer instanced_data : register(t15, space0);

// Two-component hash of a 3D lattice cell (the 3D-adapted form of the waves
// 2-component hash: z-avalanche -> y LCG mix -> x LCG mix, then the x/y hash01
// pair). Returns a float2 in [0,1).
float2 spire_cell_hash01_2(uint3 cell)
{
    uint z = cell.z;
    z += (z << 10u);
    z ^= (z >> 6u);
    z += (z << 3u);
    z ^= (z >> 11u);

    uint h1 = ((cell.y * 1664525u) + (z + (z << 15u))) + 1013904223u;
    h1 *= 1664525u;
    h1 ^= (h1 >> 11u);
    h1 ^= (h1 << 7u) & 2636928640u;
    h1 ^= (h1 << 15u) & 4022730752u;

    uint h2 = ((cell.x * 1664525u) + (h1 ^ (h1 >> 18u))) + 1013904223u;
    h2 *= 1664525u;
    h2 ^= (h2 >> 11u);
    h2 ^= (h2 << 7u) & 2636928640u;
    h2 ^= (h2 << 15u) & 4022730752u;

    uint xv = h2 ^ (h2 >> 18u);
    uint yv = xv;
    yv += (yv << 10u);
    yv ^= (yv >> 6u);
    yv += (yv << 3u);
    yv ^= (yv >> 11u);

    return float2(asfloat((xv & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((yv + (yv << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// Inline 3D value noise (8-lattice trilinear blend of the 2-component cell hash)
// with a clamped [-0.5, 0.5] output.
float2 spire_noise3(float3 p)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);

    float2 n = spire_cell_hash01_2(uint3(cell)) * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 0))) * u.x * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 0))) * (1.0f - u.x) * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 0))) * u.x * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 0, 1))) * (1.0f - u.x) * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 1))) * u.x * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 1))) * (1.0f - u.x) * u.y * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 1))) * u.x * u.y * u.z;

    return clamp((n - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float roof_profile_mult : TEXCOORD2;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint base = vin.instance_id * 32u;
    float roof_t = instanced_data.Load<float>(base + 16u);
    float roof_profile = instanced_data.Load<float>(base + 20u);
    float radius = instanced_data.Load<float>(base + 24u);

    // Spire profile gate: only vertices tagged with a profile multiplier bulge
    // horizontally.
    bool is_profile = vin.roof_profile_mult > -0.001f;
    float3 scaled_pos = vin.Vertex_Position;
    if (is_profile)
    {
        float s = max(0.6f, (radius * 2.0f) * lerp(0.6f, 1.0f, vin.roof_profile_mult)
            * lerp(0.4f, 1.5f, 1.0f - roof_t));
        scaled_pos = float3(vin.Vertex_Position.x * s, vin.Vertex_Position.y, vin.Vertex_Position.z * s);
    }

    // Back-ease-out rise: (2^-10t * sin((10t - 0.75) * 2PI/3) + 1), halved.
    float3 world = ((scaled_pos * ((pow(2.0f, (-10.0f) * roof_t)
        * sin(((roof_t * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f)
                 + instanced_data.Load<float4>(base + 0u).xyz;

    // 2D value-noise bend of world y, then the 3D float2 jitter.
    float3 wpos = world;
    wpos.y = world.y + perlin(world.xz * 0.5f) * 0.3f;
    float2 n3 = spire_noise3(wpos * 0.4f);
    float3 disp = wpos + (float3(n3.x, 0.0f, n3.y) * 0.06f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(disp, 1.0f));
    o.C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    o.C1 = vin.Vertex_Normal;
    o.C2 = disp;
    o.C3 = float(is_profile);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 face_normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    float3 normal = normalize(lerp(pin.C1, float3(pin.C1.x, face_normal.y, pin.C1.z), pin.C3));
    float3 albedo = sqrt(clamp(pin.C0 * 0.5f, 0.0f, 1.0f));

    uint normal_word = uint(clamp((normal.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp((normal.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp((normal.z * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3003121664u;
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 117440512u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}
