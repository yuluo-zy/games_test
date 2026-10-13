// ==RECONSTRUCTED== shaders/read_mouse_depth_value.cs.hlsl | F-D
// Evidence: _read_mouse_depth_value.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (depth_map_dims/mouse_position_screenspace/window_size/
//   filter_box_y_frac), out_buffer u15, depth_map t14, frame_constants set1/b0,
//   LocalSize [16,16,1], entry cs_main.
//   spirv-cross: reads depth at mouse position and screen center, stores world-space
//   positions (offset 0/16); samples a box around the mouse into groupshared
//   group_depths[256] (float2), ascending bitonic-sorts (min/max per component, inc
//   bitonic_sort_pair index math), then stores depth-clamped world positions
//   (offset 32/48).
// Status: O (structure from disassembly; bitonic index math matches inc/bitonic_sort.hlsl).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/bitonic_sort.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_map_dims;
    float2 mouse_position_screenspace;
    float2 window_size;
    float filter_box_y_frac;
};

Texture2D<float4> depth_map : register(t14, space0);
RWByteAddressBuffer out_buffer : register(u15, space0);

groupshared float2 group_depths[256];

[numthreads(16, 16, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID, uint groupIndex : SV_GroupIndex)
{
    float2 mouse_uv = mouse_position_screenspace / window_size;
    float2 dims = float2(depth_map_dims);
    uint2 max_px = depth_map_dims - uint2(1u, 1u);

    uint2 mouse_px = uint2(int2(clamp(uint2(int2(mouse_uv * dims)), uint2(0u, 0u), max_px)));
    float mouse_depth = depth_map.Load(int3(mouse_px, 0)).x;
    float4 mouse_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(mouse_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), mouse_depth, 1.0f)));

    uint2 center_px = uint2(int2(clamp(uint2(int2(0.5f.xx * dims)), uint2(0u, 0u), max_px)));
    float center_depth = depth_map.Load(int3(center_px, 0)).x;
    float4 center_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(center_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), center_depth, 1.0f)));

    if (groupIndex == 0u)
    {
        out_buffer.Store<float4>(0, float4(mouse_world.xyz / mouse_world.w, mouse_depth));
        out_buffer.Store<float4>(16, float4(center_world.xyz / center_world.w, center_depth));
    }

    float2 sample_offset = ((float(depth_map_dims.y) * filter_box_y_frac).xx / dims) * (((float2(groupThreadID.xy) + 0.5f.xx) * 0.0625f.xx) - 0.5f.xx);
    group_depths[groupIndex] = float2(depth_map.Load(int3(uint2(int2(clamp(uint2(int2((mouse_uv + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x, depth_map.Load(int3(uint2(int2(clamp(uint2(int2((0.5f + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x);

    GroupMemoryBarrierWithGroupSync();

    uint sort_size = 2u << (firstbithigh(255u) & 31u);
    for (uint stride = 2u; stride <= sort_size; stride *= 2u)
    {
        uint half = stride >> 1u;
        for (uint step = half; step > 0u; step = step >> 1u)
        {
            uint lane = groupIndex & (step - 1u);
            uint a = 2u * (groupIndex - lane);
            uint b = a + lane;
            uint c;
            if (step == half)
            {
                c = (a + ((2u * step) - 1u)) - lane;
            }
            else
            {
                c = (a + step) + lane;
            }
            if (c < 256u && b < 256u)
            {
                float2 va = group_depths[b];
                float2 vb = group_depths[c];
                bool2 cmp = bool2(va.x <= vb.x, va.y <= vb.y);
                group_depths[b] = float2(cmp.x ? va.x : vb.x, cmp.y ? va.y : vb.y);
                group_depths[c] = float2(cmp.x ? vb.x : va.x, cmp.y ? vb.y : va.y);
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }

    if (groupIndex == 0u)
    {
        float clamped_mouse = clamp(mouse_depth, group_depths[179u].x, group_depths[230u].x);
        float4 mouse_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(mouse_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_mouse, 1.0f)));
        out_buffer.Store<float4>(32, float4(mouse_world_clamped.xyz / mouse_world_clamped.w, clamped_mouse));

        float clamped_center = clamp(center_depth, group_depths[179u].y, group_depths[230u].y);
        float4 center_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(center_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_center, 1.0f)));
        out_buffer.Store<float4>(48, float4(center_world_clamped.xyz / center_world_clamped.w, clamped_center));
    }
}
