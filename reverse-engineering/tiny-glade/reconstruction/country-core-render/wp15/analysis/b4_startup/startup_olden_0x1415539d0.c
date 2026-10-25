// addr=0x1415539d0 name=startup_olden
// signature=void __cdecl startup_olden(void)


// WARNING: Removing unreachable block (ram,0x0001415542fd)
// WARNING: Removing unreachable block (ram,0x0001415542ba)
// WARNING: Removing unreachable block (ram,0x000141553fad)
// WARNING: Removing unreachable block (ram,0x000141554168)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::startup::glade_themes::olden::startup_olden(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  longlong lVar3;
  uint uVar4;
  undefined *puVar5;
  longlong lVar6;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  undefined *puVar7;
  longlong lVar8;
  float _X;
  undefined8 local_2e0;
  undefined8 uStack_2d8;
  undefined8 local_2d0;
  longlong local_2c8;
  longlong local_2c0;
  longlong local_2b8;
  longlong local_288;
  uint local_1d0 [4];
  undefined *local_1c0;
  undefined8 uStack_1b8;
  uint local_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined *local_1a0;
  undefined8 uStack_198;
  undefined1 local_190;
  undefined3 uStack_18f;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined *local_180;
  undefined8 uStack_178;
  undefined1 local_170;
  undefined3 uStack_16f;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined *local_160;
  undefined8 local_158;
  undefined1 local_150;
  undefined8 local_14c;
  undefined *local_140;
  undefined8 local_138;
  undefined1 local_130;
  undefined8 local_12c;
  longlong local_120;
  undefined8 local_118;
  longlong local_110;
  longlong lStack_108;
  longlong local_100;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_110 = 0;
  lStack_108 = 4;
  local_100 = 0;
  local_1d0[0] = local_1d0[0] & 0xffffff00;
  local_1d0[1] = 0x3fa66666;
  local_1d0[2] = 0x3fa66666;
  local_1c0 = &DAT_142c6c936;
  uStack_1b8 = 0x1e;
  local_1b0 = local_1b0 & 0xffffff00;
  uStack_1ac = 0x3fa66666;
  uStack_1a8 = 0x3fa66666;
  local_1a0 = &DAT_142c6c954;
  uStack_198 = 0x11;
  _local_190 = CONCAT31(uStack_18f,2);
  uStack_18c = 0x3fe66666;
  uStack_188 = 0x3ff33333;
  local_180 = &DAT_142c6c965;
  uStack_178 = 0x24;
  _local_170 = CONCAT31(uStack_16f,2);
  uStack_16c = 0x3fe66666;
  uStack_168 = 0x3ff33333;
  local_160 = &DAT_142c6c989;
  local_158 = 0xf;
  local_150 = 1;
  local_14c = 0x408ccccd3fe66666;
  local_140 = &DAT_142c6c998;
  local_138 = 0x22;
  local_130 = 1;
  local_12c = 0x408ccccd3fe66666;
  local_118 = 6;
  lVar8 = 0;
  while (lVar8 != 6) {
    lVar1 = lVar8 + 1;
    local_120 = lVar1;
    uVar4 = local_1d0[lVar8 * 8];
    if ((char)uVar4 == '\x03') break;
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::path::Path::_join();
    asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>();
    if (local_2b8 != 0) {
      lVar8 = local_2c0;
      do {
        _X = *(float *)(lVar8 + 0x10) * 0.5;
        sinf(_X);
        cosf(_X);
        ::utils::geometry::rectangle::Rectangle2d::new();
        lVar6 = local_100;
        if (local_100 == local_110) {
          alloc::raw_vec::RawVec::
          grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>
                    ();
        }
        lVar3 = lVar6 * 0x1c;
        *(undefined8 *)(lStack_108 + 0x10 + lVar3) = local_2d0;
        *(undefined8 *)(lStack_108 + lVar3) = local_2e0;
        ((undefined8 *)(lStack_108 + lVar3))[1] = uStack_2d8;
        *(char *)(lStack_108 + 0x18 + lVar3) = (char)uVar4;
        local_100 = lVar6 + 1;
        lVar8 = lVar8 + 0x30;
      } while (lVar8 != local_2b8 * 0x30 + local_2c0);
    }
    if (local_2c8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar8 = lVar1;
    if (local_288 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  puVar7 = &DAT_142c6c8f3;
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  bevy_ecs::system::commands::Commands::spawn_empty();
  puVar5 = local_1c0;
  lVar8 = _DAT_00000020;
  if ((ulonglong)(_DAT_00000010 - _DAT_00000020) < 0x98) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar1 = _DAT_00000018;
  *(code **)(_DAT_00000018 + lVar8) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::AlbedoMap,country_core::components::drawable::AlphaMap,country_core::components::NormalMap>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar2 = (undefined8 *)(lVar1 + 8 + lVar8);
  *puVar2 = 0;
  puVar2[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar1 + 0x18 + lVar8) = 0;
  *(undefined4 *)(lVar1 + 0x20 + lVar8) = 0;
  *(undefined8 *)(lVar1 + 0x24 + lVar8) = 0x3f8000003f800000;
  *(undefined4 *)(lVar1 + 0x2c + lVar8) = 0x3f800000;
  *(undefined8 *)(lVar1 + 0x38 + lVar8) = extraout_RAX;
  *(undefined **)(lVar1 + 0x40 + lVar8) = puVar7;
  *(undefined8 *)(lVar1 + 0x48 + lVar8) = 0x23ae4b5c6ce0d14d;
  *(undefined8 *)(lVar1 + 0x50 + lVar8) = 0x691e5a80737217f2;
  *(undefined8 *)(lVar1 + 0x58 + lVar8) = 0xfbf3a82f352c1401;
  *(undefined1 *)(lVar1 + 0x68 + lVar8) = 0;
  *(undefined **)(lVar1 + 0x78 + lVar8) = puVar5;
  *(code **)(lVar1 + 0x88 + lVar8) = bevy_ecs::error::handler::panic;
  _DAT_00000020 = lVar8 + 0x98;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  puVar7 = &DAT_142c6c8f3;
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  bevy_ecs::system::commands::Commands::spawn_empty();
  puVar5 = local_1c0;
  lVar8 = _DAT_00000020;
  if ((ulonglong)(_DAT_00000010 - _DAT_00000020) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar1 = _DAT_00000018;
  *(code **)(_DAT_00000018 + lVar8) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::AlbedoMap>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar2 = (undefined8 *)(lVar1 + 8 + lVar8);
  *puVar2 = 0;
  puVar2[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar1 + 0x18 + lVar8) = 0;
  *(undefined4 *)(lVar1 + 0x20 + lVar8) = 0;
  *(undefined8 *)(lVar1 + 0x24 + lVar8) = 0x3f8000003f800000;
  *(undefined4 *)(lVar1 + 0x2c + lVar8) = 0x3f800000;
  *(undefined8 *)(lVar1 + 0x38 + lVar8) = extraout_RAX_01;
  *(undefined **)(lVar1 + 0x40 + lVar8) = puVar7;
  *(undefined8 *)(lVar1 + 0x48 + lVar8) = 0x2bb813728bcb5dd4;
  *(undefined1 *)(lVar1 + 0x58 + lVar8) = 0;
  *(undefined **)(lVar1 + 0x68 + lVar8) = puVar5;
  *(code **)(lVar1 + 0x78 + lVar8) = bevy_ecs::error::handler::panic;
  _DAT_00000020 = lVar8 + 0x88;
  in_RCX[2] = local_100;
  *in_RCX = local_110;
  in_RCX[1] = lStack_108;
  return;
}


