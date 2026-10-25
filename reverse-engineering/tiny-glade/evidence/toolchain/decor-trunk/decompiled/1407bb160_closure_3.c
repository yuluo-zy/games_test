// ADDR 0x1407bb160
// SIG void __cdecl closure$3(void)
// ORIG system::closure$3


void __cdecl system_decorator::load_decorators::load_decorators::closure_3(void)

{
  longlong extraout_RAX;
  void *_Dst;
  undefined8 *in_RCX;
  longlong local_108;
  undefined8 uStack_100;
  longlong local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  longlong local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined1 local_2a;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  country_core::render::mesh::Mesh::positions_without_indices();
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
            ();
  local_38 = uStack_50;
  country_core::systems::collision::world::RaycastMeshData::new_convex();
  if (local_108 != 0) {
    if (local_58 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_2a = 1;
    country_core::render::mesh::Mesh::as_triangles();
    local_29 = 1;
    country_core::render::mesh::Mesh::collision_bbx();
    local_88 = local_98;
    uStack_80 = uStack_90;
    local_98 = local_a8;
    uStack_90 = uStack_a0;
    local_a8 = local_b8;
    uStack_a0 = uStack_b0;
    local_b8 = local_c8;
    uStack_b0 = uStack_c0;
    local_c8 = local_d8;
    uStack_c0 = uStack_d0;
    local_d8 = local_e8;
    uStack_d0 = uStack_e0;
    local_e8 = local_f8;
    uStack_e0 = uStack_f0;
    local_f8 = local_108;
    uStack_f0 = uStack_100;
    local_108 = 1;
    uStack_100 = 1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst != (void *)0x0) {
      memcpy(_Dst,&local_108,0x90);
      in_RCX[2] = local_60;
      *in_RCX = local_70;
      in_RCX[1] = uStack_68;
      in_RCX[6] = local_48;
      in_RCX[7] = uStack_40;
      in_RCX[4] = local_58;
      in_RCX[5] = uStack_50;
      in_RCX[3] = _Dst;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  core::option::expect_failed();
}


