// ==RECONSTRUCTED== shaders/wall/inc/brick_instance_flags.hlsl | nani-cull-a
// Observation: embedded as include in 10 wall bins. Wall brick instance flag
// bits decoded from the InstancedWallData.flags / instance_count_and_flags
// consumers. Kept as #defines (not file-scope static consts) so they never
// spill into the implicit $Globals cbuffer, whose wall-family layout is exactly
// { int draw_indirect_buffer_offset; uint2 hzb_tex_inner_dims; }.
//   0x80 (128)            = brick uses bevel geometry (pass1 LOD skip)
//   0x80000000 (2^31)     = pass2 (HZB) flag in push_constants.instance_count_and_flags
//   0xFFFFFF  (16777215)  = instance-count mask within the same push constant
//   0x200000  (2097152)   = brick draw-list bucket capacity (bricks per bucket)
#ifndef WALL_INC_BRICK_INSTANCE_FLAGS_HLSL
#define WALL_INC_BRICK_INSTANCE_FLAGS_HLSL

#define WALL_BRICK_FLAG_BEVEL 0x80u
#define WALL_BRICK_FLAG_PASS2 0x80000000u
#define WALL_BRICK_INSTANCE_COUNT_MASK 0xFFFFFFu
#define WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY 0x200000u

#endif // WALL_INC_BRICK_INSTANCE_FLAGS_HLSL
