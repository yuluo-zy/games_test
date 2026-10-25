
void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>_>
          (void)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong extraout_RAX;
  byte *in_RCX;
  longlong in_RDX;
  longlong lVar5;
  char *local_80;
  undefined8 local_78;
  byte *local_70;
  code *local_68;
  char **local_60;
  code *local_58;
  undefined *local_50;
  undefined8 local_48;
  byte **local_40;
  undefined8 local_38;
  undefined8 local_30;
  
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if ((*in_RCX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  if (*(ulonglong *)(in_RCX + 8) < *(ulonglong *)(in_RDX + 0x218)) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 8) * 8);
    if ((uVar3 != 0) &&
       (lVar4 = *(longlong *)(in_RDX + 0x1e0), *(longlong *)(lVar4 + 0x20 + ~uVar3 * 0x60) != 0)) {
      if (((*(ulonglong *)(in_RCX + 0x10) < *(ulonglong *)(in_RDX + 0x218)) &&
          (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x10) * 8)
          , uVar3 != 0)) && (lVar5 = ~uVar3 * 0x60, *(longlong *)(lVar4 + 0x20 + lVar5) != 0)) {
        *(int *)(lVar5 + lVar4 + 0x5c) = iVar2;
        country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
        get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
        if (extraout_RAX != 0) {
          country_core::render::impl$0::deref();
          country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
          gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
          *(int *)(in_RCX + 0x150) = iVar2;
          return;
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_70 = in_RCX + 0xb8;
      local_80 = "country_core::asset_libraries::ssbo_library::AssetSsboLibrary";
      local_78 = 0x3d;
      local_68 = alloc::string::impl$23::fmt;
      local_60 = &local_80;
      local_58 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_50 = &DAT_142b76c60;
      local_48 = 2;
      local_30 = 0;
      local_40 = &local_70;
      local_38 = 2;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
  }
  local_70 = in_RCX + 0xb8;
  local_80 = "country_core::render::GraphicsDeviceResourceResource requested by  does not exist: ";
  local_78 = 0x2c;
  local_68 = alloc::string::impl$23::fmt;
  local_60 = &local_80;
  local_58 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_50 = &DAT_142b76c60;
  local_48 = 2;
  local_30 = 0;
  local_40 = &local_70;
  local_38 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

