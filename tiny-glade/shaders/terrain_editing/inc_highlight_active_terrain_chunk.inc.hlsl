// ============================================================
// terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl
// 功能:当像素位于"正在编辑的地形块"内时,把表面颜色向编辑高亮色混合。
// 消费方:floor.raster.hlsl、deferred_*_light_sun_sky、rocky_terrain/stairs。
// 说明:active 地形块假定为整个编辑区域,高亮强度由 strength 控制。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl | evidence: no direct include edges; consumed by floor.raster.hlsl + deferred_*_light_sun_sky + rocky_terrain/stairs; highlights the actively-edited terrain chunk (editing_highlight_color from frame_constants) | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL

// 当像素位于 active 地形块内时,把表面颜色向编辑高亮色混合。
// chunk_uv 为高度图空间的 uv;active 块假定为整个编辑区域。
float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif // TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
