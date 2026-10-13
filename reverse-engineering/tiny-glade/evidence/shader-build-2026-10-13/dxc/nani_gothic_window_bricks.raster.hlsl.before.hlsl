// ==RECONSTRUCTED== shaders/nani_gothic_window_bricks.raster.hlsl | B nani raster | bins: _nani_gothic_window_bricks.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani gothic window brick wall: brick material + window frame colors, snow
// tint and semitransparent dither; packs the gbuffer.

// Registerless terrain heightmap global for the inc/terrain_heightmap.inc.hlsl
// terrain_height() helper (INC fix #3); unused here, dxc DCEs it.
Texture2D terrain_heightmap : register(t63);

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "wall/inc/brick_frag.hlsl"
#include "wall/inc/brick_instance_flags.hlsl"
#include "wall/inc/brick_vs_out.hlsl"
#include "wall/inc/highlight_mode.hlsl"
#include "inc/hash.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/wall_color_customization.hlsl"
#include "inc/roof_colors_include.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint wallspace_x_range_packed;
    uint seed;
    uint wall_id_is_wooden_packed;
    float mesh_size_x;
};

ByteAddressBuffer wall_color_ids : register(t18);
StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
ByteAddressBuffer halftimber_color_ids : register(t19);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2DArray<float4> brick_colors : register(t22);
Texture2DArray<float4> roof_colors : register(t24);

uint hash3_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    nointerpolation uint C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C4 : TEXCOORD4;
    float C7 : TEXCOORD7;
    float2 C10 : TEXCOORD10;
    nointerpolation int C11 : TEXCOORD11;
    nointerpolation float2 C12 : TEXCOORD12;
    nointerpolation int C13 : TEXCOORD13;
    nointerpolation uint C14 : TEXCOORD14;
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
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = idx;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C4 = mul(xform, float4(vin.Vertex_Position, 1.0f)).xyz;
    o.C7 = world.y;
    o.C10 = vin.Vertex_UV;
    uint wall_seed = hash(nani_instance_data[idx].seed ^ 934565u);
    o.C11 = int(wall_color_ids.Load<uint>(wall_seed * 4 + 0));
    o.C12 = float2(float(nani_instance_data[idx].wallspace_x_range_packed), nani_instance_data[idx].mesh_size_x);
    o.C13 = int(nani_instance_data[idx].seed);
    o.C14 = idx;
    return o;
}

struct PsIn
{
    nointerpolation uint C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C4 : TEXCOORD4;
    float C7 : TEXCOORD7;
    float2 C10 : TEXCOORD10;
    nointerpolation int C11 : TEXCOORD11;
    nointerpolation float2 C12 : TEXCOORD12;
    nointerpolation int C13 : TEXCOORD13;
    nointerpolation uint C14 : TEXCOORD14;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool is_wooden = (pin.C13 & 8) != 0;
    bool has_roof = (pin.C13 & 2) != 0;
    bool semitransparent = (pin.C13 & 512) != 0;
    bool dither = semitransparent && bool_render_shadows != 0;
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

    uint color_id = uint(pin.C11);
    float3 albedo;
    if (has_roof)
    {
        albedo = roof_colors.Sample(sampler_llr, float3((pin.C0.xx * 0.2f) + (pin.C2.xz * 0.2f), float(color_id))).xyz * 0.7f;
    }
    else
    {
        float2 brick_uv = pin.C10;
        uint timber_color_id = uint(halftimber_color_ids.Load<uint>(uint(pin.C11) * 4 + 0));
        float3 timber = halftimber_colors.SampleBias(sampler_llr, float3(brick_uv, float(timber_color_id)), global_mip_bias).xyz;
        bool brick_like = color_id == 0u || color_id == 4u || color_id == 10u || color_id == 12u;
        albedo = brick_like
            ? brick_colors.Sample(sampler_llr, float3(brick_uv + 0.85f.xx, 0.0f)).xyz
            : timber;
    }

    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, (smoothstep(0.0f, 0.2f, clamp(normal.y, 0.0f, 1.0f)) * smoothstep(-0.1f, 0.0f, pin.C2.y)).xxx);
    }
    float instance_seed = float(nani_instance_data[pin.C0].seed);
    float seed_jitter = hash3_rand(hash(uint(instance_seed)));
    albedo *= (0.9f + (seed_jitter * 0.2f)).xxx;
    albedo = lerp(albedo, color_library.wall_1.xyz, 0.02f.xxx);

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
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}
