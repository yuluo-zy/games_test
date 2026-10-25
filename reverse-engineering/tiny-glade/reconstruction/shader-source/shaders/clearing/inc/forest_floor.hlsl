// ==RECONSTRUCTED== shaders/clearing/inc/forest_floor.hlsl
// Evidence: embedded in the terrain_editing floor raster bins (2 variants). Forest-floor
// shading helpers (ground cover / leaf-litter blend). The terrain_editing floor consumer
// inlines this surface treatment; the exact helper set is not pinned by an inlined use site
// in the reconstructed floor shader (which omits the include), so only a minimal, compiling
// surface is provided here.
// Status: U (helper names/bodies Inference — no inlined use site observed).

#ifndef TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL
#define TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL

// Blend two ground-cover colors by a litter/cover amount.
float3 forest_floor_blend(float3 ground, float3 litter, float cover)
{
    return lerp(ground, litter, cover);
}

#endif // TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL
