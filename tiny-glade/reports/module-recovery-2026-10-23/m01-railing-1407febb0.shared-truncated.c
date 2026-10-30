
void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece(void)

{
  ulonglong uVar1;
  ulonglong in_stack_00000030;
  undefined8 *in_stack_00000038;
  ulonglong in_stack_00000040;
  ulonglong in_stack_00000050;
  char in_stack_00000058;
  char in_stack_00000060;
  float in_stack_00000068;
  float in_stack_00000070;
  
  if (in_stack_00000040 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  uVar1 = 0;
  if (0.0 < (float)((ulonglong)*in_stack_00000038 >> 0x20) * in_stack_00000070 +
            (float)*in_stack_00000038 * in_stack_00000068) {
    uVar1 = in_stack_00000030 - 1;
  }
  if (in_stack_00000030 <= uVar1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (in_stack_00000040 <= uVar1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  ::utils::swizzlers::impl$0::x0y();
  if (uVar1 < in_stack_00000050) {
    ::utils::swizzlers::impl$0::x0y();
    if (in_stack_00000058 == '\0') {
      country_core::resources::render::Affine3Packed::from_cols();
      country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
      country_core::geometry::instanced_wall::impl$3::default();
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
    }
    if (in_stack_00000060 == '\0') {
      country_core::resources::render::Affine3Packed::from_cols();
      country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
      country_core::geometry::instanced_wall::impl$3::default();
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}


