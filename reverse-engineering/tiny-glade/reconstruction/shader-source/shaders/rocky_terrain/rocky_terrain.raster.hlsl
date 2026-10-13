// ==RECONSTRUCTED== shaders/rocky_terrain/rocky_terrain.raster.hlsl | F-R | bins: _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin, _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin | entry vs:vs_main (vs_6_4) ps:ps_main (ps_6_4)
// Rocky-terrain mesh raster pass. The vertex shader indexes the displaced mesh
// SSBO by gl_VertexIndex (3 vertices per triangle): emits the displaced
// position, the triangle id (vertexIndex/3), the barycentric corner weight
// (one-hot of vertexIndex%3) and the corner's cell_id. A top-down
// (force_top_down_render) projection remaps xz by 1/65 and height by
// clamp((y+2.5)/26.5) for the game's top-down tool views. The pixel shader
// applies per-triangle crack fixing (triangle_adjacency.inc.hlsl): near the
// triangle's nearest edge the shading normal is blended toward the
// neighbouring triangle's face normal, with a hash-driven simplex noise
// perturbing the blend band; then triplanar rocky-terrain_texture detail
// sampling (3 SampleBias taps on world-uv/30 with a cell_id jitter, blended by
// luminance+normal weights) and a GBuffer pack of RGB8 albedo + RGB8 normal.
// bindings: $Globals b13, vertices t15 (StructuredBuffer<Vertex>),
//   triangles t16 (StructuredBuffer<Triangle>), frame_constants b0/space1 (vs)
//   | sampler_mipmap s10, vertices t15, triangles t16, rocky_terrain_texture
//   t18, frame_constants b0/space1 (ps). Capabilities: RuntimeDescriptorArray
//   (forced, SPV_EXT_descriptor_indexing on the SSBOs) + Shader on both modules.
// evidence: spirv-cross decompilation of _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin
//   (workspace/f-r-scratch/modules/_rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915_{0,1}.hlsl.cross)
//   transcribed math/order verbatim (Vertex position loaded at byte 16, cell_id
//   at byte 28; Triangle neighbours/is_top/normal at 0/12/16). The f0ad variant
//   differs only in the PS (empty ps_main, no bindings) — this file targets the
//   b903 variant.
// status: O (algorithm/order/literals from .cross); inc-layer OpSource/OpString
//   + dxc-version opcode diffs are expected (recorded, not fixed).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/color_library.hlsl"
#include "rocky_terrain/inc_mesh.inc.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/texture_blend.hlsl"
#include "rocky_terrain/triangle_adjacency.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBOs; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<Vertex> vertices : register(t15, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<Triangle> triangles : register(t16, space0);

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    uint2 rocky_terrain_texture_dims;
    uint2 highlight_active_hill_dims;
    uint force_top_down_render;
};

Texture2D<float4> rocky_terrain_texture : register(t18, space0);

struct vs_main_output
{
    float3 c0 : TEXCOORD0;
    nointerpolation int c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    nointerpolation int c3 : TEXCOORD3;
    float4 position : SV_Position;
};

vs_main_output vs_main(uint vertex_id : SV_VertexID)
{
    float3 position = vertices[vertex_id].position;

    float4 clip_pos;
    if (force_top_down_render != 0u)
    {
        clip_pos = float4(position.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f),
                           clamp((position.y - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f),
                           1.0f);
    }
    else
    {
        clip_pos = mul(view_constants[current_view_index].world_to_sample, float4(position, 1.0f));
    }

    vs_main_output output;
    output.position = clip_pos;
    output.c0 = position;
    output.c1 = int(vertex_id / 3u);
    float3 corner_weight = 0.0f;
    corner_weight[vertex_id % 3u] = 1.0f;
    output.c2 = corner_weight;
    output.c3 = vertices[vertex_id].cell_id;
    return output;
}

struct ps_main_input
{
    float3 c0 : TEXCOORD0;
    nointerpolation int c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    nointerpolation int c3 : TEXCOORD3;
};

uint2 ps_main(ps_main_input input) : SV_Target0
{
    uint tri = uint(input.c1);

    // Crack-fix adjacency: nearest triangle edge from the interpolated
    // barycentric weights, and the neighbour across it (-1 on a boundary).
    uint edge = nearest_edge_index(input.c2);
    int neighbour = triangles[tri].neighbours[edge];
    uint neighbour_u = uint(neighbour);

    // Distance from the fragment to the nearest edge (only when the edge has a
    // neighbour across it; the blend below is still applied to the OOB read,
    // exactly as the original inlines it).
    float edge_distance;
    if (neighbour != -1)
    {
        float3 edge_start = vertices[tri * 3u + edge].position;
        float3 edge_end = vertices[tri * 3u + (edge + 1u) % 3u].position;
        edge_distance = distance_to_edge(input.c0, edge_start, edge_end);
    }
    else
    {
        edge_distance = -1.0f;
    }

    bool both_top = (triangles[tri].is_top == 1) && (triangles[neighbour_u].is_top == 1);
    float blend_width = both_top ? 0.20000000298023223876953125f : 0.07999999821186065673828125f;

    // Hash-driven 2D value/gradient noise (Ashima permute + normalize
    // constants, m^4 falloff) on a cell-randomized world position; it perturbs
    // the edge distance so the crack-fix blend band is not a straight line.
    float2 noise_p = float2(input.c0.x, input.c0.z)
                   + (float(input.c3) * 0.100000001490116119384765625f + input.c0.y * 0.5f).xx;
    float2 i = floor(noise_p + dot(noise_p, 0.3660254180431365966796875f.xx));
    float2 x0 = (noise_p - i) + dot(i, 0.211324870586395263671875f.xx);
    float2 i1 = (x0.x > x0.y) ? float2(1.0f, 0.0f) : float2(0.0f, 1.0f);
    float4 c = x0.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f,
                                -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 x1 = c.xy - i1;
    float4 x12 = float4(x1.x, x1.y, c.z, c.w);
    float2 i_mod = fmod(i, 289.0f.xx);
    float3 p0a = i_mod.y.xxx + float3(0.0f, i1.y, 1.0f);
    float3 p0 = fmod((p0a * 34.0f + 1.0f) * p0a, 289.0f) + i_mod.x.xxx + float3(0.0f, i1.x, 1.0f);
    float3 m_fall = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x12.zw, x12.zw)), 0.0f);
    float3 m = m_fall * m_fall;
    float3 g_raw = frac(fmod((p0 * 34.0f + 1.0f) * p0, 289.0f) * 0.024390242993831634521484375f) * 2.0f;
    float3 g = g_raw - 1.0f;
    float3 s = abs(g) - 0.5f;
    float3 a0 = g - floor(g_raw - 0.5f);
    float3 grad;
    grad.x = a0.x * x0.x + s.x * x0.y;
    grad.y = a0.y * x1.x + s.y * x1.y;
    grad.z = a0.z * x12.z + s.z * x12.w;
    float noise = 130.0f * dot((m * m) * (1.792842864990234375f - ((a0 * a0 + s * s) * 0.8537347316741943359375f)), grad);

    // Blend the shading normal toward the neighbour's face normal near the
    // shared edge.
    float edge_noise = edge_distance + noise * (both_top ? 0.5f : 0.25f);
    float t0 = smoothstep(0.0f, blend_width, edge_noise);
    float t1 = smoothstep(0.0f, blend_width * 0.5f, edge_distance);
    float blend = (1.0f - min(t0, t1)) * 0.5f;
    float3 n_raw = lerp(triangles[tri].normal, triangles[neighbour_u].normal, blend);

    // Triplanar detail sampling of the rocky-terrain texture: world position
    // scaled by 1/30, three axis taps, blended by luminance + normal weights.
    float3 world_uv = input.c0 * 0.0333333350718021392822265625f;
    float jitter_v = rand01(hash(uint(input.c3)));
    float2 jitter = float2(jitter_v, jitter_v);
    float3 sx = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.zy + jitter, global_mip_bias).xyz;
    float3 sy = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.xy + jitter, global_mip_bias).xyz;
    float3 sz = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.xz + jitter, global_mip_bias).xyz;

    float3 abs_normal = pow(abs(n_raw), 1.0f);
    const float3 luminance = float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f);
    float lum_sy = dot(sy, luminance);
    float w0 = dot(sx, luminance) + abs_normal.x;
    float w1 = lum_sy + abs_normal.z;
    float w2 = lum_sy + abs_normal.y;
    float max_w = max(max(w0, w1), w2) - 0.20000000298023223876953125f;
    float c0 = max(w0 - max_w, 0.0f);
    float c1 = max(w1 - max_w, 0.0f);
    float c2 = max(w2 - max_w, 0.0f);
    float3 rock = ((sx * c0) + (sy * c1) + (sz * c2)) / (c0 + c1 + c2) * 1.10000002384185791015625f;

    float3 n = normalize(n_raw);
    float3 rock_color;
    if (glade_theme_id == 4u)
    {
        rock_color = lerp(rock, 0.699999988079071044921875f,
                          smoothstep(0.5f, 1.0f, clamp(n.y, 0.0f, 1.0f))
                          * smoothstep(-0.100000001490116119384765625f, 0.0f, input.c0.y));
    }
    else
    {
        rock_color = rock;
    }

    float3 packed_normal = n * 0.5f + 0.5f;
    float3 sqrt_color = sqrt(clamp(rock_color, 0.0f, 1.0f));

    uint color_packed = (uint((clamp(sqrt_color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                       + (uint((clamp(sqrt_color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                       + (uint((clamp(sqrt_color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);

    uint normal_packed = (uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)
                        + 3422552064u;

    uint2 result = uint2(normal_packed, color_packed);
    result.y = color_packed | 369098752u;
    return result;
}
