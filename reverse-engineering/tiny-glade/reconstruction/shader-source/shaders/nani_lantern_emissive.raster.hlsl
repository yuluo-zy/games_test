// ==RECONSTRUCTED== shaders/nani_lantern_emissive.raster.hlsl | B nani raster | bins: _nani_lantern_emissive.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani lantern: VS forwards a half-packed lantern position/intensity + flags;
// PS dithers semitransparent lanterns, colors the glass by palette id, packs
// the gbuffer and emits a point-light glow into the emissive target.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint2 light_position_ls_intensity;
    uint flags_color_id;
    uint pad0;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    uint2 half_packed = nani_instance_data[idx].light_position_ls_intensity;
    float3 light_local = float3(f16tof32(half_packed.x & 0xFFFFu),
                                f16tof32((half_packed.x >> 16u) & 0xFFFFu),
                                f16tof32(half_packed.y & 0xFFFFu));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C3 = mul(xform, float4(light_local, 1.0f));
    o.C4 = nani_instance_data[idx].flags_color_id;
    o.C5 = f16tof32((half_packed.y >> 16u) & 0xFFFFu);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 emissive : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = (pin.C4 & 2147483648u) != 0u && bool_render_shadows != 0;
    if (dither)
    {
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    uint color_id = pin.C4 & 65535u;
    float3 color;
    if (color_id == 0u)
    {
        color = float3(0.79198f, 0.633584f, 0.079198f) / max(1e-10f, 0.79198f);
    }
    else if (color_id == 1u)
    {
        color = float3(0.0f, 0.158396f, 0.79198f) / max(1e-10f, 0.21524f);
    }
    else if (color_id == 2u)
    {
        color = float3(0.158396f, 0.79198f, 0.158396f) / max(1e-10f, 0.77216f);
    }
    else if (color_id == 3u)
    {
        color = float3(0.237594f, 0.158396f, 0.79198f) / max(1e-10f, 0.27902f);
    }
    else
    {
        color = float3(0.79198f, 0.237594f, 0.079198f) / max(1e-10f, 0.43438f);
    }

    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(pin.C0, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;

    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 to_light = normalize(pin.C2 - cam_pos);
    float3 perp = pin.C3 - (cam_pos + (to_light * dot(pin.C3 - cam_pos, to_light)));
    float dist2 = dot(perp, perp);
    float3 glow = (color * (1.4f * pin.C5)) * (3.0f * ((0.8f * exp2((-50.0f) * dist2)) + (2.0f * exp2((-200.0f) * dist2))));
    o.emissive = (pin.C4 & 1073741824u) != 0u ? glow : 0.0f.xxx;
    return o;
}
