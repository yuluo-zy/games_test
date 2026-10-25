// ==RECONSTRUCTED== shaders/wall/inc/draw_lists.hlsl | nani-cull-a
// Observation: embedded as include in 7 wall bins. Shared wall draw-list
// helpers. The DrawIndexedIndirectCommand record used by the wall family is the
// shared camelCase one provided by inc/draw_indirect_types.hlsl (pulled in via
// inc/draw_indirect.hlsl by the compute passes); the raster consumers read the
// brick draw list directly (see wall_brick_lod*.raster.hlsl).
// Brick draw-list layout: bucket = draw-command index (0..3 pass1, 4..7 LOD
// rings, offset by $Globals.draw_indirect_buffer_offset), capacity per bucket =
// 0x200000 (2097152) bricks.
#ifndef WALL_INC_DRAW_LISTS_HLSL
#define WALL_INC_DRAW_LISTS_HLSL

#include "brick_instance_flags.hlsl"

uint wall_brick_draw_list_slot(uint bucket, uint prev)
{
    return bucket * WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY + prev;
}

#endif // WALL_INC_DRAW_LISTS_HLSL
