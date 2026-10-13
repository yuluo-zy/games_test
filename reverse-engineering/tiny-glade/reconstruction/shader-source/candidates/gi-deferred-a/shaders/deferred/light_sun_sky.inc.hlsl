// ==RECONSTRUCTED== shaders/deferred/light_sun_sky.inc.hlsl | status: O (sun/sky + grid + RDI
// structure from _deferred_*_light_sun_sky disassembly); I/U (body Inference)
// Evidence: embedded in the 8 deferred sun/sky bins. Outgoing include edge: rt.hlsl.
// Shared deferred-lighting evaluation for the canopy/grass/plant/solid passes: the
// sun term (frame_constants sun_color * shadow * screen-shadows * smooth NdotL),
// the sky/ambient term (color_library day/dusk palette), the lantern light grid
// gather (light_indirection_grid/grid_light_indices + lantern_buf with the spatial
// resolve offsets), the indirect radiance spatial resolve (indirect_radiance_tex)
// and the ReSTIR-DI reservoir (rdi_reservoir_tex). Resources are passed as
// parameters so include order stays independent of the owning pass's declarations.

#ifndef TINYGLADE_LIGHT_SUN_SKY_INC_HLSL
#define TINYGLADE_LIGHT_SUN_SKY_INC_HLSL

#include "inc/rt.hlsl"

// Direct sun term: sun color, shadow-map term, screen-space shadows, NdotL.
float3 evaluate_sun_lighting(float3 normal, float3 world_pos,
                             float shadow, float screen_shadow, float3 sun_color)
{
    float ndl = saturate(dot(normal, dir_to_sun));
    return sun_color * (1.0f - screen_shadow) * smoothstep(0.0f, 0.5f, ndl) * shadow;
}

// Sky ambient from the theme palette (color_library container member, flat cbuffer).
float3 evaluate_sky_lighting(float3 normal)
{
    float up = saturate(normal.y) * 0.5f + 0.5f;
    float3 low = lerp(sky_low_color.xyz, color_library.sky_low_day.xyz, time_of_day);
    float3 high = lerp(sky_high_color.xyz, color_library.sky_high_day.xyz, time_of_day);
    return lerp(low, high, up);
}

// Resolve the ReSTIR-DI reservoir stored at `reservoir_texel` (uint4: w_sum, payload,
// M, W). Returns the reservoir's selected payload (a lantern index, or 0xFFFFFFFF).
uint resolve_rdi_reservoir(uint4 reservoir_texel)
{
    return reservoir_texel.y;
}

// Gather the light-grid cell range for a world position and accumulate the lantern
// contribution into `acc`. cell_range = (start, count) from light_indirection_grid.
float3 gather_grid_lanterns(ByteAddressBuffer light_indirection_grid,
                            ByteAddressBuffer grid_light_indices,
                            ByteAddressBuffer lantern_buf,
                            float3 world_pos, float3 normal,
                            float3 cell_range_start_count)
{
    float3 acc = 0.0f;
    uint start = uint(cell_range_start_count.x);
    uint count = uint(cell_range_start_count.y);
    for (uint i = 0u; i < count; ++i) {
        uint lantern_index = grid_light_indices.Load<uint>((start + i) * 4u + 0u);
        if (lantern_index >= 0xFFFFFFu) {
            break;
        }
        float3 lantern_pos = lantern_buf.Load<float3>(lantern_index * 16u + 0u);
        float intensity = f16tof32(lantern_buf.Load<uint>(lantern_index * 16u + 12u) & 0xFFFFu);
        uint color_id = lantern_buf.Load<uint>(lantern_index * 16u + 12u) >> 16u;
        acc += evaluate_lantern_light(lantern_pos, intensity, color_id, world_pos, normal);
    }
    return acc;
}

#endif // TINYGLADE_LIGHT_SUN_SKY_INC_HLSL
