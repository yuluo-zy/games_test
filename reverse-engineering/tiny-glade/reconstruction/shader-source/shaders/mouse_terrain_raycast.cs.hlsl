// ==RECONSTRUCTED== shaders/mouse_terrain_raycast.cs.hlsl | evidence: spirv-cross of _mouse_terrain_raycast.cs.hlsl_cs_main_0.bin (ray-march against terrain heightmap from the mouse ray; stores mouse_raycast_position + terrain_stroke_index) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float3 mouse_ray_origin;
    float3 mouse_ray_dir;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 stroke_indices_dims;
};

struct CsOut
{
    float4 mouse_raycast_position;
    int terrain_stroke_index;
};

RWStructuredBuffer<CsOut> my_buffer : register(u17, space0);
Texture2D<int4> stroke_indices : register(t16, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    float3 origin = mouse_ray_origin;
    float3 dir = mouse_ray_dir;

    // Intersection of the mouse ray with the top of the terrain height range.
    float3 top_hit = origin + dir * ((TERRAIN_HEIGHT_MAX - origin.y) / dir.y);
    float3 ground_hit = origin + dir * ((-origin.y) / dir.y);
    float step = length(ground_hit - top_hit) * 0.001f;

    float3 pos = top_hit;
    float3 next;
    for (int i = 0; i < 1000; pos = next, i++)
    {
        next = pos + dir * step;
        float2 uv = next.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f;
        if (next.y < lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x))
        {
            break;
        }
    }

    int2 dims = int2(stroke_indices_dims);
    float3 query = pos * TERRAIN_SAMPLE_SCALE.xxx + 0.5f;
    int2 pixel = int2(int(query.x * float(dims.x)), int(query.z * float(dims.y)));
    my_buffer[0].mouse_raycast_position = pos.xyzz;
    my_buffer[0].terrain_stroke_index = stroke_indices.Load(int3(uint2(pixel), 0)).x;
}
