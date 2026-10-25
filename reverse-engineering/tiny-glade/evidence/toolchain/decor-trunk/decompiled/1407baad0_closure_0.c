// ADDR 0x1407baad0
// SIG void __cdecl closure$0(void)
// ORIG system::closure$0


void __cdecl system_decorator::load_decorators::load_decorators::closure_0(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  void *_Dst;
  undefined8 *in_RCX;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined1 local_2d;
  undefined1 local_2c;
  undefined1 local_2b;
  undefined1 local_2a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  country_core::render::mesh::Mesh::arrayed_indices();
  local_29 = 1;
  country_core::render::mesh::Mesh::positions_without_indices();
  local_29 = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
            ();
  local_2a = 1;
  country_core::render::mesh::Mesh::normals_without_indices();
  if (extraout_RAX == 0) {
    local_68 = 0x8000000000000000;
  }
  else {
    local_2a = 1;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
              ();
  }
  local_2b = 1;
  country_core::render::mesh::Mesh::uvs_without_indices();
  if (extraout_RAX_00 == 0) {
    local_118 = 0x8000000000000000;
  }
  else {
    local_2b = 1;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,2>_>_>,country_core::render::mesh::impl_0::uvs_without_indices::closure_env_0>_>
              ();
  }
  local_2b = 0;
  country_core::systems::collision::world::RaycastMeshData::new_indexed();
  local_2d = 1;
  country_core::render::mesh::Mesh::as_triangles();
  local_2c = 1;
  country_core::render::mesh::Mesh::collision_bbx();
  local_118 = 1;
  local_110 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,&local_118,0x90);
    in_RCX[2] = local_38;
    *in_RCX = local_48;
    in_RCX[1] = uStack_40;
    in_RCX[6] = local_58;
    in_RCX[7] = uStack_50;
    in_RCX[4] = local_68;
    in_RCX[5] = uStack_60;
    in_RCX[3] = _Dst;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


