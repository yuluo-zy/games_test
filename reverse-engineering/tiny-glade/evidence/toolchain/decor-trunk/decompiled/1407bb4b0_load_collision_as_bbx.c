// ADDR 0x1407bb4b0
// SIG void __cdecl load_collision_as_bbx(void)
// ORIG system::load_collision_as_bbx


void __cdecl system_decorator::load_decorators::load_collision_as_bbx(void)

{
  longlong extraout_RAX;
  void *_Dst;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined8 local_1e8;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  void *local_d0;
  undefined8 local_c8;
  float local_c0;
  float local_bc;
  undefined4 local_b0;
  undefined4 local_ac;
  float local_a8;
  float local_a4;
  undefined4 local_a0;
  float fStack_9c;
  undefined4 local_98;
  undefined8 local_90;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  country_core::render::mesh::Mesh::collision_bbx();
  if ((((local_c0 != 1.0) || (NAN(local_c0))) || (local_bc != 0.0)) || (NAN(local_bc))) {
    local_178 = 0;
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>();
  }
  ::utils::swizzlers::impl$1::xvy();
  ::utils::xz_rotation_bad();
  uStack_15c = 0x3f800000;
  uStack_158 = 0x3f800000;
  uStack_154 = 0x3f800000;
  country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
            ();
  _local_a0 = CONCAT44(local_a4 - local_a8,local_b0);
  local_98 = local_ac;
  local_81 = 1;
  ::utils::swizzlers::impl$1::xvy();
  local_81 = 1;
  country_core::systems::collision::world::RaycastMeshData::new_cuboid();
  local_178 = 1;
  uStack_170 = 1;
  local_168 = local_1f8;
  uStack_160 = uStack_1f0;
  uStack_15c = uStack_1ec;
  uStack_158 = (undefined4)local_1e8;
  uStack_154 = (undefined4)((ulonglong)local_1e8 >> 0x20);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_178,0x90);
  local_90 = local_e8;
  local_c8 = local_e0;
  local_d0 = _Dst;
  country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name();
  if (extraout_RAX_00 != 0) {
    country_core::render::mesh::Mesh::collision_bbx();
    *in_RCX = local_90;
    in_RCX[1] = local_c8;
    in_RCX[2] = local_d8;
    in_RCX[4] = local_178;
    in_RCX[5] = uStack_170;
    in_RCX[6] = local_168;
    in_RCX[7] = CONCAT44(uStack_15c,uStack_160);
    in_RCX[3] = local_d0;
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


