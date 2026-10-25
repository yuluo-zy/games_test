// ==RECONSTRUCTED== shaders/terrain_derivative.cs.hlsl | evidence: spirv-cross of _terrain_derivative.cs.hlsl_cs_main_0.bin (rock steepness from central differences of the terrain heightmap, smoothstep(0.75,1.25) into rocky_terrain.x) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 rocky_terrain_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
};

RWTexture2D<float4> rocky_terrain : register(u14, space0);

// Out-of-bounds neighbor default height (flat terrain).
static const float FLAT_HEIGHT = 0.09433962404727935791015625f;

float sample_terrain_height(int2 pixel)
{
    if (any(pixel < 0) || any(uint2(pixel) >= rocky_terrain_dims))
    {
        return FLAT_HEIGHT;
    }
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, terrain_heightmap.Load(int3(pixel, 0)).x);
}

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    float2 world_per_pixel = (float2(130.0f, 130.0f) / float2(terrain_heightmap_dims)) * 2.0f;
    int2 right = int2(int(min(uint(int3(dispatch_thread_id).x + 1), terrain_heightmap_dims.x)), int3(dispatch_thread_id).y);
    int2 left = int2(max(int3(dispatch_thread_id).x - 1, 0), int3(dispatch_thread_id).y);
    int2 up = int2(int3(dispatch_thread_id).x, int(min(uint(int3(dispatch_thread_id).y + 1), terrain_heightmap_dims.y)));
    int2 down = int2(int3(dispatch_thread_id).x, max(int3(dispatch_thread_id).y - 1, 0));

    float h_right = sample_terrain_height(right);
    float h_left = sample_terrain_height(left);
    float h_up = sample_terrain_height(up);
    float h_down = sample_terrain_height(down);

    float steepness = length(float2((h_right - h_left) / world_per_pixel.x, (h_up - h_down) / world_per_pixel.y));
    float4 rocky = rocky_terrain[uint2(int3(dispatch_thread_id).xy)];
    rocky.x = smoothstep(0.75f, 1.25f, steepness);
    rocky_terrain[uint2(int3(dispatch_thread_id).xy)] = rocky;
}
