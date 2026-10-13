// ==RECONSTRUCTED== shaders/nani_plaster.raster.hlsl | B nani raster | bins: _nani_plaster.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani plaster wall: VS forwards the plaster instance; PS shades the plaster
// wall with wall colors + normal map + mortar seams, brush-eraser highlight
// and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "plaster.inc.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/wall_color_customization.hlsl"
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

uint highlighted_wall_id;
int highlight_bricks_to_delete;
float3 brush_position;
float eraser_mult;

ByteAddressBuffer wall_color_ids : register(t15);
Texture2DArray<float4> plaster_colors : register(t17);
StructuredBuffer<uint> instance_indirection_buffer : register(t18);
ByteAddressBuffer nani_instance_data : register(t19);
Texture2D<float4> normal_map : register(t20);
Texture2D<float4> mortar : register(t21);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C2 : TEXCOORD2;
    nointerpolation uint C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    nointerpolation float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float3 C8 : TEXCOORD8;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    float4 bounding_sphere = nani_instance_data.Load<float4>(idx * 48 + 0);
    float3 world = vin.Vertex_Position;
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C2 = world;
    o.C0 = idx;
    o.C1 = vin.Vertex_Position.xy;
    o.C3 = bounding_sphere.xyz;
    o.C4 = float3(0.0f, 1.0f, 0.0f);
    o.C6 = 0.0f;
    o.C7 = vin.Vertex_Position.xy;
    o.C8 = world;
    return o;
}

struct PsIn
{
    nointerpolation uint C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    nointerpolation float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float3 C8 : TEXCOORD8;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    uint wall_id = nani_instance_data.Load<uint>(pin.C0 * 48 + 16);
    int color_id = int(wall_color_ids.Load<uint>(wall_id * 4 + 0));
    float3 albedo = plaster_colors.SampleBias(sampler_llr, float3(pin.C7, float(color_id)), 0.0f).xyz;
    float mortar_seam = mortar.SampleBias(sampler_llr, pin.C7 * 0.5f, 0.0f).x;
    albedo *= lerp(0.9f, 1.0f, mortar_seam);
    float3 nm = normal_map.SampleBias(sampler_llr, pin.C7, 0.0f).xyz * 2.0f - 1.0f.xxx;
    normal = normalize(normal + (nm * 0.3f));
    bool in_brush = false;
    if (highlight_bricks_to_delete == 1)
    {
        in_brush = distance(brush_position.xz, pin.C2.xz) < (1.889f * eraser_mult) - 0.27f;
        if (in_brush)
        {
            albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
        }
    }

    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | (((7u | (uint(in_brush) << 3u)) | 0u) << 24u);
    return o;
}
