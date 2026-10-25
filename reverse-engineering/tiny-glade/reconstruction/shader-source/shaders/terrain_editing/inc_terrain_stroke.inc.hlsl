// ==RECONSTRUCTED== shaders/terrain_editing/inc_terrain_stroke.inc.hlsl | evidence: include-graph direct edge -> sdf_utils.inc.hlsl; consumed only by rasterize_terrain_stroke (stroke list rasterized into img_output / rocky_terrain / terrain_sdg); $Globals of that pass carries stroke_width / stroke_infos_device_address / stroke_count | status: O (include edge, stroke role); I (struct/body Inference — inlined)

#ifndef TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL

#include "sdf_utils.inc.hlsl"

// A single terrain-editing stroke, written by the CPU into the stroke info
// buffer (device address in the consuming shader's $Globals).
struct TerrainStroke
{
    float2 start_xz;
    float2 end_xz;
    float width;
    float falloff;
    int kind;
    int pad;
};

// Signed distance of a world-space point to a stroke centerline, in world
// units. Strokes are flattened to the terrain plane (xz).
float stroke_distance(float2 world_xz, TerrainStroke stroke)
{
    return sd_segment(world_xz, stroke.start_xz, stroke.end_xz) - stroke.width * 0.5f;
}

// Normalized stroke influence in [0,1] using the stroke falloff.
float stroke_influence(float2 world_xz, TerrainStroke stroke)
{
    float d = stroke_distance(world_xz, stroke);
    return 1.0f - clamp(d / max(stroke.falloff, 1e-4f), 0.0f, 1.0f);
}

#endif // TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL
