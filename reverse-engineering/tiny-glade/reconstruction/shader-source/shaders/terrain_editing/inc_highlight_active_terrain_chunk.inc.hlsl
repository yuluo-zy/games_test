// ==RECONSTRUCTED== shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl | evidence: no direct include edges; consumed by floor.raster.hlsl + deferred_*_light_sun_sky + rocky_terrain/stairs; highlights the actively-edited terrain chunk (editing_highlight_color from frame_constants) | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL

// Blend a surface color toward the editing highlight when the pixel lies in
// the active terrain chunk. `chunk_uv` is the heightmap-space uv; the active
// chunk is assumed to be the full editing area.
float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif // TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
