// ==RECONSTRUCTED== shaders/water_x_paths.raster.hlsl
// Evidence: _water_x_paths.raster.hlsl_b903e43ffb3da915.bin (SPIR-V, offsets 23/19694).
//   VS: sampler_llb s4, sampler_default s9, $Globals b13 (current_view_index, water_mask_dims,
//   path_mask_dims, buildings_mask_dims, rocky_terrain_texture_dims, bool_render_shadows),
//   water_mask t14, path_mask t15, buildings_mask t16, frame_constants set1/b0.
//   PS: sampler_llb s4, sampler_mipmap s10, path_mask t15, rocky_terrain_texture t17,
//   frame_constants set1/b0; outputs a packed uint2 (RGB8 color + id/type bits).
//   spirv-cross + disassembly: "x-paths" water-adjacent ground. VS raises vertices inside the water
//   mask (water_mask green > 0.2) onto the path/building-mask terrain with per-prim hash jitter and a
//   building smoothstep carve; outside water the vertex is NaN (culled). PS shades rocky-terrain
//   triplanar samples blended by an albedo-weighted mask + per-cell hash, tints paths near water, and
//   packs sqrt(color) + normal into a uint2.
// Status: O (algorithm + bindings from SPIR-V; hash chains transcribed from disassembly).
// NOTE: the f0ad PS is an empty stub; single source cannot reproduce both — b903 is the target.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 buildings_mask_dims;
    uint2 rocky_terrain_texture_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> buildings_mask : register(t16, space0);
Texture2D<float4> rocky_terrain_texture : register(t17, space0);

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float3 c3 : TEXCOORD3;
    float4 pos : SV_Position;
};

uint hash_base(uint v)
{
    v = v + (v << 10u);
    v ^= v >> 6u;
    v += v << 3u;
    v ^= v >> 11u;
    return v;
}

uint hash_avalanche(uint v)
{
    v ^= v >> 11u;
    v ^= (v << 7u) & 2636928640u;
    v ^= (v << 15u) & 4022730752u;
    return v;
}

float hash_unit_xy(uint h)
{
    return asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
}

VSOut vs_main(float3 Vertex_Position : TEXCOORD0,
              float3 Vertex_Normal : TEXCOORD1,
              float3 prim_center : TEXCOORD2)
{
    VSOut o;
    float2 uv_center = (prim_center.xz * 0.0076923076994717121124267578125f) + 0.5f;
    float2 uv_vert = (Vertex_Position.xz * 0.0076923076994717121124267578125f) + 0.5f;

    float mask = 0.0f;
    if (uv_center.x >= 0.0f && uv_center.x <= 1.0f && uv_center.y >= 0.0f && uv_center.y <= 1.0f)
    {
        mask = water_mask.SampleLevel(sampler_default, uv_center, 0.0f).y;
    }

    float4 pos = float4(Vertex_Position, 1.0f);
    if (mask > 0.2f)
    {
        uint h = uint((prim_center.x + prim_center.z) + 100.0f);
        h = hash_base(h);
        float jitter = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        pos.y = (Vertex_Position.y + (-3.0f))
            + (pow(path_mask.SampleLevel(sampler_llb, uv_center, 0.0f).x, 0.2f) * 1.5f)
            + ((1.5f * pow(path_mask.SampleLevel(sampler_llb, uv_vert, 0.0f).x, 0.2f))
                * (1.2f + lerp(-0.1f, 0.3f, jitter)))
            - smoothstep(0.35f, 0.6f, buildings_mask.SampleLevel(sampler_default, uv_vert, 0.0f).x);
    }
    else
    {
        pos = float4(asfloat(0x7fbfffffu).xxx, pos.w);
    }

    o.pos = mul(view_constants[current_view_index].world_to_sample, pos);
    o.c0 = float3(0.13f, 0.13f, 0.1f);
    o.c1 = Vertex_Normal;
    o.c2 = pos.xyz;
    o.c3 = prim_center;
    return o;
}

uint2 ps_main(float3 c1 : TEXCOORD1, float3 c2 : TEXCOORD2, float3 c3 : TEXCOORD3) : SV_Target0
{
    float3 n = normalize(c1);
    float3 wp = c2 * 0.0333333350718021392822265625f;
    uint seed = uint(1000.0f * (abs(c3.x) + abs(c3.z)));
    float rnd = hash_unit_xy(hash_base(seed));

    float3 rock_xy = rocky_terrain_texture.Sample(sampler_mipmap, (wp.zy * 2.0f) + (rnd * 0.2f)).xyz;
    float3 rock_xz = rocky_terrain_texture.Sample(sampler_mipmap, (wp.xy * 2.0f) + (rnd * 0.2f)).xyz;
    float4 rock_yz = rocky_terrain_texture.Sample(sampler_mipmap, (wp.xz * 2.0f) + (rnd * 0.2f));

    float3 pw = pow(abs(n), 1.0f);
    float l_xy = dot(rock_xz, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float l_zy = dot(rock_xy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + pw.x;
    float l_mix = l_xy + pw.z;
    float l_y = l_xy + pw.y;
    float max_l = max(max(l_zy, l_mix), l_y) - 0.2f;
    float w_zy = max(l_zy - max_l, 0.0f);
    float w_mix = max(l_mix - max_l, 0.0f);
    float w_y = max(l_y - max_l, 0.0f);

    bool theme2or4 = (glade_theme_id == 2u) || (glade_theme_id == 4u);
    float3 tint = theme2or4 ? float3(0.0625f, 0.04f, 0.005625f) : float3(0.042f, 0.07f, 0.021f);

    float2 grid = ((c2.xz + c2.xy) + (c2.yz * 0.5f) + (rnd * 200.0f)) * 0.85f;
    int2 cell = int2(floor(grid));
    float2 f = frac(grid);
    float2 sm = f * f * (3.0f - 2.0f * f);
    uint2 base = uint2(cell);
    uint h00 = hash_avalanche((base.x * 1664525u) + hash_base(base.y));
    uint h10 = hash_avalanche(((base.x + 1u) * 1664525u) + hash_base(base.y));
    uint h01 = hash_avalanche((base.x * 1664525u) + hash_base(base.y + 1u));
    uint h11 = hash_avalanche(((base.x + 1u) * 1664525u) + hash_base(base.y + 1u));
    float nz = smoothstep(0.5f, 1.0f, (1.0f - sm.y) * ((hash_unit_xy(h00) * (1.0f - sm.x)) + (hash_unit_xy(h10) * sm.x))
        + ((hash_unit_xy(h01) * (1.0f - sm.x)) * sm.y) + ((hash_unit_xy(h11) * sm.x) * sm.y));

    float3 blended = ((rock_xy * w_zy) + (rock_xz * w_mix) + (rock_yz.xyz * w_y)) / (w_zy + w_mix + w_y);
    float3 shade = lerp(lerp(blended * 1.1f, tint, ((1.0f - smoothstep(-0.2f, 0.6f - nz, c2.y)) * 0.75f).xxx),
                        tint, (clamp(pw.x + pw.z, 0.0f, 1.0f) * nz).xxx);

    if (glade_theme_id == 4u && c2.y > 0.0f)
    {
        float pm = path_mask.SampleLevel(sampler_llb, (c2.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x;
        shade = lerp(shade, 0.7f.xxx, (smoothstep(0.2f, 1.0f, clamp((n.y * (pm * pm * pm * pm)) * lerp(0.5f, 1.0f, nz), 0.0f, 1.0f)) + 0.1f).xxx);
    }

    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 packed_c = sqrt(clamp(shade, 0.0f, 1.0f));
    uint col = pack_rgb8(packed_c);
    uint2 res = uint2(pack_rgb8(packed_n) + 3422552064u, col);
    res.y = col | 117440512u;
    return res;
}
