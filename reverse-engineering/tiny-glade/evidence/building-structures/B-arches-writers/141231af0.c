
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_wall_constructor::construct_arches::construct_arches::closure_3(void)

{
  longlong *plVar1;
  char extraout_AL;
  longlong in_RCX;
  
  country_core::resources::render::Affine3Packed::from_mat3_translation();
  plVar1 = *(longlong **)(in_RCX + 8);
  country_core::geometry::instanced_wall::impl$1::from();
  country_core::geometry::instanced_wall::impl$3::default();
  country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
  push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  if (extraout_AL != '\0') {
    **(undefined1 **)(in_RCX + 0x28) = 1;
  }
  *plVar1 = *plVar1 + 1;
  return;
}

