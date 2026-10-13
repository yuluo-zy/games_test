// ==RECONSTRUCTED== shaders/instanced_roof_beams.raster.hlsl
// SOURCE PATH : shaders/instanced_roof_beams.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _instanced_roof_beams.raster.hlsl_b903e43ffb3da915.bin
//   _instanced_roof_beams.raster.hlsl_f0adff76827e73ff.bin
//   pass/stage   : _instanced_roof_beams (roof) / raster
//   entry        : vs:vs_main; ps:ps_main
//   names        : ViewConstants, frame_constants, $Globals, color_library, RoofSupportBeam, Affine3Packed, instanced_data, vs_main, ps_main
// Status: O (structure from disassembly: per-instance 80-byte RoofSupportBeam
//   transform + circle_normal + roof_profile; 2D value-noise (inc/perlin) bend of
//   world.y; inverse-transpose normal; beam tint from color_library.fence_color;
//   PS packs the solid gbuffer with material-id bits). I (variable naming, the
//   trailing 8 bytes of RoofSupportBeam, pass intent).
//   instanced_data is a ByteAddressBuffer (the embedded module accesses it with
//   raw .Load at byte offsets); the RoofSupportBeam struct documents the record
//   layout (48-byte affine + 16-byte circle_normal + float profile + padding).
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
#include "inc/math.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Per-instance roof support-beam record (80 bytes).
struct RoofSupportBeam
{
    Affine3Packed transform;
    float4 circle_normal;
    float roof_profile;
    int pad0;
    float pad1;
    float pad2;
};

ByteAddressBuffer instanced_data : register(t15, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    Affine3Packed packed;
    [unroll]
    for (uint i = 0u; i < 12u; i++)
    {
        packed.data[i] = instanced_data.Load<float>(vin.instance_id * 80u + i * 4u);
    }

    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));

    // Roof beams lean the vertex toward the circle plane by the roof profile.
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f))
                 + (instanced_data.Load<float4>(vin.instance_id * 80u + 48u).xyz
                    * lerp(0.2f, -0.15f, instanced_data.Load<float>(vin.instance_id * 80u + 64u)));

    // 2D value-noise bend of the world y (same lattice as inc/perlin).
    float3 wpos = world;
    wpos.y = world.y + perlin(world.xz * 0.5f) * 0.3f;

    // Inverse-transpose normal transform (upper 3x3 of the affine).
    float3 c0 = cross(xform[1].xyz, xform[2].xyz);
    float3 c1 = cross(xform[2].xyz, xform[0].xyz);
    float3 c2 = cross(xform[0].xyz, xform[1].xyz);
    float3 inv_det = (1.0f / dot(xform[2].xyz, c2)).xxx;
    o.C1 = mul(vin.Vertex_Normal, float3x3(c0 * inv_det, c1 * inv_det, c2 * inv_det));

    o.position = mul(view_constants[current_view_index].world_to_sample,
                     float4(world.x, wpos.y, world.z, 1.0f));
    o.C0 = lerp(color_library.fence_color_2.xyz, color_library.fence_color_1.xyz, 0.2f)
         * float3(0.72f, 0.648f, 0.576f);
    o.C2 = wpos;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(pin.C1);
    float3 albedo = sqrt(clamp(pin.C0, 0.0f, 1.0f));

    uint normal_word = uint(clamp((normal.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp((normal.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp((normal.z * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3858759680u;
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 117440512u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}
