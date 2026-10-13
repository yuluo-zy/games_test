// ==RECONSTRUCTED== shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl | F-R | bin: _rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0.bin | entry cs:cs_main | LocalSize [64,1,1] | profile cs_6_4
// Rocky-terrain vertex/triangle displacement. One thread owns one triangle
// (gid.x: vertices [3x, 3x+1, 3x+2], triangle x). When none of the 3 vertices
// sits inside a rocky/water region the first vertex is written back with a NaN
// position; otherwise every vertex is displaced: terrain height sample with a
// -2.0 drop outside the 130-unit extent, rocky_terrain/path_mask/water_mask
// erosion, per-vertex value noise (perlin) at 0.2 world scale, a
// gathered-gradient ribbon displacement for non-corner, non-path vertices
// modulated by a 1-octave fbm, then the triangle normal is recomputed from the
// displaced positions.
// bindings: push_constants (PushConstant), sampler_llc s3, sampler_llb s4,
//   sampler_default s9, __alias_vertices u13 (RWStructuredBuffer<Vertex>),
//   __alias_triangles u14, $Globals b15, terrain_heightmap t16,
//   rocky_terrain t18, path_mask t19, water_mask t20
// evidence: spirv-cross decompilation of _rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0.bin
//   (workspace/f-r-scratch/modules/_rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0_0.hlsl.cross)
//   transcribed math/order verbatim. The spirv-cross phi-carried per-vertex
//   noise "rotation" state reduces to the standard perlin constants: the select
//   chains (1664525/1013904223 seeds, 0x7FFFFF mantissa mask, 0x3F800000 base)
//   propagate the initial defaults invariantly through all 3 vertices and the
//   1-octave fbm, so the lattice matches inc/perlin.hlsl hash(uint2).
// status: O (algorithm/order/literals from .cross); residual do-while/phi +
//   opcode-count differences are dxc-version artifacts (recorded, not fixed).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "rocky_terrain/inc_mesh.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
Texture2D<float4> terrain_heightmap : register(t16, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

// Vertex/Triangle record layouts come from rocky_terrain/inc_mesh.inc.hlsl
// (reconstructed; previously inlined here while that inc was still a skeleton).
//
RWStructuredBuffer<Vertex> __alias_vertices : register(u13, space0);
RWStructuredBuffer<Triangle> __alias_triangles : register(u14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
};

struct PushConstants
{
    uint triangle_count;
};
[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> water_mask : register(t20, space0);

// World-xz -> mask uv (1/130 world scale, centered at .5).
static const float TERRAIN_UV_SCALE = 0.0076923076994717121124267578125f;

// Bounds-checked rocky-terrain channel sample: the target returns 0.0 for a
// channel when the uv escapes [0,1]^2 (spirv-cross do/while(false) guard in the
// .cross; z/x/y sampled separately there, folded into one guarded sample here).
float sample_rocky_terrain_channel(float2 uv, int channel)
{
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
    {
        return 0.0f;
    }
    float4 t = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
    return (channel == 0) ? t.x : ((channel == 1) ? t.y : t.z);
}

// 2D value-noise lattice transcribed from the target's inlined lattice:
// per-corner hash(uint2) (perlin rotation constants), smoothstep curve and the
// ((1-w)*blend0) + ((n01*(1-u))*w + (n11*u)*w) grouping, verbatim.
float value_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    return (((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
          + (((n01 * (1.0f - u.x)) * u.y) + ((n11 * u.x) * u.y)));
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= push_constants.triangle_count)
    {
        return;
    }

    uint triangle_base = gl_GlobalInvocationID.x * 3u;

    // Early-out: if none of the triangle's 3 vertices sits inside a rocky or
    // water region, write the first vertex back with a NaN position.
    bool displace = false;
    for (uint v = 0; v < 3; ++v)
    {
        float2 uv = (__alias_vertices[triangle_base + v].rest_position * 65.0f).xz
                    * TERRAIN_UV_SCALE + 0.5f;
        float r_z = sample_rocky_terrain_channel(uv, 2);
        float r_x = sample_rocky_terrain_channel(uv, 0);
        float r_y = sample_rocky_terrain_channel(uv, 1);
        float region = clamp(smoothstep(0.1f, 0.2f, r_x + r_y)
                           + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv, 0.0f).y), 0.0f, 1.0f);
        if (region > 0.0f || r_z > 0.0f)
        {
            displace = true;
            break;
        }
    }

    if (!displace)
    {
        Vertex v0 = __alias_vertices[triangle_base];
        v0.position = asfloat(0x7fbfffffu).xxx;
        __alias_vertices[triangle_base] = v0;
        return;
    }

    for (uint vi = 0; vi < 3; ++vi)
    {
        uint vertex_id = triangle_base + vi;
        Vertex vert = __alias_vertices[vertex_id];

        // 5-step wang hash of the vertex cell id -> per-vertex float in [0,1).
        uint cell_hash = hash(uint(vert.cell_id));
        float cell_rand = asfloat((cell_hash & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

        // Jitter the rest position toward/away from the cell centroid, then
        // scale into the 130-unit world space.
        float3 jittered = vert.rest_position
                        + (vert.dir_to_centroid * 0.001f * lerp(-3.0f, 8.0f, cell_rand));
        float3 pos = jittered * 65.0f;
        float2 uv0 = (pos.xz * TERRAIN_UV_SCALE) + 0.5f;

        // Base terrain height, dropped 2.0 beyond the 130-unit extent.
        float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv0, 0.0f).x);
        float3 moved = pos;
        moved.y = h;
        if (abs(pos.x) > 65.0f || abs(pos.z) > 65.0f)
        {
            moved.y = h - 2.0f;
        }

        float2 uv1 = (moved.xz * TERRAIN_UV_SCALE) + 0.5f;
        float r_x1 = sample_rocky_terrain_channel(uv1, 0);
        float r_y1 = sample_rocky_terrain_channel(uv1, 1);
        float rock_region = clamp(smoothstep(0.1f, 0.2f, r_x1 + r_y1)
                                + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y), 0.0f, 1.0f);
        float r_y2 = sample_rocky_terrain_channel(uv1, 1);
        float path_t = smoothstep(0.1f, 0.15f, path_mask.SampleLevel(sampler_llb, uv1, 0.0f).x);
        float r_z0 = sample_rocky_terrain_channel(uv0, 2);
        float water_y = water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y;
        float wd = abs(water_y - 0.5f);
        float water_smooth;
        if (wd > 1.0f)
        {
            water_smooth = 0.0f;
        }
        else
        {
            water_smooth = 1.0f - ((wd * wd) * (3.0f - (2.0f * wd)));
        }

        // Per-vertex value noise at 0.2 world scale.
        float noise = value_noise(moved.xz * 0.2f);

        // Combine erosion, path flattening and water smoothing with the noise,
        // modulated by the terrain slope (flat regions get more noise).
        float3 result = moved;
        result.y = moved.y
            + lerp(-0.4f, 0.2f, clamp(rock_region + r_z0 - 10.0f * path_t, 0.0f, 1.0f))
            - smoothstep(0.5f, 0.6f, water_y) * 0.2f
            - (water_smooth * noise * (1.0f - smoothstep(0.3f, 0.5f,
                lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv1, 0.0f).x))));

        float3 displaced;
        if (rock_region > 0.0f)
        {
            // Terrain gradient + world-space normal from a gather central
            // difference at the base-mapped uv.
            float4 gather = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
            float2 scale = 130.0f / float2(terrain_heightmap_dims);
            float2 grad = (float2((gather.z + gather.y) - (gather.w + gather.x),
                                  (gather.x + gather.y) - (gather.w + gather.z)) * 0.5f) * 26.5f / scale;
            float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)),
                                  normalize(float3(1.0f, grad.x, 0.0f)));
            float4 rt = rocky_terrain.SampleLevel(sampler_default, uv0, 0.0f);
            float rock_t = rt.x;

            // Slide the vertex along the terrain normal inside the rocky region
            // and pull it down along the path.
            float3 tilted = result
                + (normal * lerp(-0.3f, 0.1f * lerp(0.0f, 3.0f, cell_rand), vert.cell_bby)
                          * rock_region * (1.0f - path_t))
                - (normal * path_t);

            // Non-corner vertices away from paths get an extra ribbon-shaped
            // normal offset scaled by a 1-octave fbm of the jittered position.
            if ((vert.is_corner == 0) && (path_t == 0.0f))
            {
                float4 g1 = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
                float4 g2 = terrain_heightmap.GatherRed(sampler_default,
                    (tilted.xz * TERRAIN_UV_SCALE) + 0.5f, int2(0, 0));
                float grad1_len = length(((float2((g1.z + g1.y) - (g1.w + g1.x),
                                                  (g1.x + g1.y) - (g1.w + g1.z)) * 0.5f) * 26.5f) / scale);
                float grad2_len = length(((float2((g2.z + g2.y) - (g2.w + g2.x),
                                                  (g2.x + g2.y) - (g2.w + g2.z)) * 0.5f) * 26.5f) / scale);

                float2 fbm_pos = (jittered.xz * 0.5f) + (rock_t * 20.0f).xx;
                float fbm_sum = 0.0f;
                float amp = 0.5f;
                for (int oct = 0; oct < 1; ++oct)
                {
                    fbm_sum += amp * (-1.0f + 2.0f * value_noise(fbm_pos));
                    amp *= 0.55f;
                    fbm_pos = mul(float2x2(float2(0.8f, -0.6f), float2(0.6f, 0.8f)), fbm_pos * 1.9f);
                }

                displaced = tilted
                    + (normal * lerp(-0.2f, 1.0f - smoothstep(4.0f, 7.0f, grad2_len),
                                     clamp((fbm_sum + 1.0f) * 0.5f, 0.0f, 1.0f))
                       * smoothstep(0.0f, 0.3f, rock_t) * grad1_len) * 0.3f
                    + (normal * r_y2);
            }
            else
            {
                displaced = tilted;
            }
        }
        else
        {
            displaced = result;
        }

        vert.position = displaced;
        __alias_vertices[vertex_id] = vert;
    }

    // Recompute the triangle normal from the displaced positions.
    Triangle tri = __alias_triangles[gl_GlobalInvocationID.x];
    float3 p0 = __alias_vertices[triangle_base].position;
    float3 p1 = __alias_vertices[triangle_base + 1u].position;
    float3 p2 = __alias_vertices[triangle_base + 2u].position;
    tri.normal = normalize(cross(p2 - p0, p1 - p0));
    __alias_triangles[gl_GlobalInvocationID.x] = tri;
}
