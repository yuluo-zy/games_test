
void __cdecl
system_wall_constructor::construct_elevation_supports::circle_bottom_layer::
add_bottom_brick<ref_mut_<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env_0>_>
          (void)

{
  undefined1 *puVar1;
  char extraout_AL;
  longlong in_stack_00000038;
  
  fastrand::Rng::f32();
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$1::xvy();
  country_core::resources::render::impl$1::from();
  country_core::geometry::instanced_wall::impl$1::from();
  country_core::geometry::instanced_wall::impl$3::default();
  puVar1 = *(undefined1 **)(in_stack_00000038 + 8);
  country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
  push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  if (extraout_AL != '\0') {
    *puVar1 = 1;
  }
  return;
}

