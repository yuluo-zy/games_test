// addr=0x141553120 name=startup_flowery
// signature=void __cdecl startup_flowery(void)


void __cdecl country_core::startup::glade_themes::flowery::startup_flowery(void)

{
  undefined4 *puVar1;
  undefined8 *puVar2;
  float *pfVar3;
  longlong lVar4;
  longlong lVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined *puVar10;
  longlong lVar11;
  longlong *plVar12;
  float fVar13;
  float fVar14;
  longlong in_stack_00000028;
  longlong local_1e8;
  longlong local_1e0;
  longlong local_1d8;
  longlong local_138;
  longlong *plStack_130;
  longlong *local_128;
  undefined8 local_118;
  
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_138)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  puVar10 = &DAT_142c6c862;
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar12 = plStack_130;
  if (plStack_130 == (longlong *)0x0) {
    plVar12 = local_128;
  }
  lVar11 = plVar12[2];
  if ((ulonglong)(*plVar12 - lVar11) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plVar12[1];
  *(code **)(lVar4 + lVar11) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar2 = (undefined8 *)(lVar4 + 8 + lVar11);
  *puVar2 = 0;
  puVar2[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar4 + 0x18 + lVar11) = 0;
  *(undefined4 *)(lVar4 + 0x20 + lVar11) = 0;
  *(undefined8 *)(lVar4 + 0x24 + lVar11) = 0x3f8000003f800000;
  *(undefined4 *)(lVar4 + 0x2c + lVar11) = 0x3f800000;
  *(undefined8 *)(lVar4 + 0x38 + lVar11) = extraout_RAX;
  *(undefined **)(lVar4 + 0x40 + lVar11) = puVar10;
  *(undefined1 *)(lVar4 + 0x48 + lVar11) = 0;
  *(undefined8 *)(lVar4 + 0x58 + lVar11) = local_118;
  *(code **)(lVar4 + 0x68 + lVar11) = bevy_ecs::error::handler::panic;
  plVar12[2] = lVar11 + 0x78;
  **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>();
  if (local_1d8 != 0) {
    lVar11 = 0;
    do {
      fVar14 = *(float *)(local_1e0 + 0x10 + lVar11) * 0.5;
      fVar13 = sinf(fVar14);
      fVar14 = cosf(fVar14);
      puVar1 = (undefined4 *)(local_1e0 + lVar11);
      uVar6 = *puVar1;
      uVar7 = puVar1[1];
      uVar8 = puVar1[2];
      uVar9 = puVar1[3];
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar12 = plStack_130;
      if (plStack_130 == (longlong *)0x0) {
        plVar12 = local_128;
      }
      lVar4 = plVar12[2];
      if ((ulonglong)(*plVar12 - lVar4) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar5 = plVar12[1];
      *(code **)(lVar5 + lVar4) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar3 = (float *)(lVar5 + 8 + lVar4);
      *pfVar3 = fVar13 * 0.0;
      pfVar3[1] = fVar13;
      pfVar3[2] = fVar13 * 0.0;
      pfVar3[3] = fVar14;
      puVar1 = (undefined4 *)(lVar5 + 0x18 + lVar4);
      *puVar1 = uVar6;
      puVar1[1] = uVar7;
      puVar1[2] = uVar8;
      puVar1[3] = uVar9;
      *(ulonglong *)(lVar5 + 0x28 + lVar4) = CONCAT44(uVar9,uVar9);
      *(undefined4 *)(lVar5 + 0x38 + lVar4) = 0;
      *(undefined8 *)(lVar5 + 0x40 + lVar4) = 0x13fa53c58fdddee8;
      *(undefined1 *)(lVar5 + 0x48 + lVar4) = 0xe;
      *(undefined1 *)(lVar5 + 0x58 + lVar4) = 0;
      *(undefined8 *)(lVar5 + 0x68 + lVar4) = local_118;
      *(code **)(lVar5 + 0x78 + lVar4) = bevy_ecs::error::handler::panic;
      plVar12[2] = lVar4 + 0x88;
      lVar11 = lVar11 + 0x30;
    } while (local_1d8 * 0x30 != lVar11);
  }
  if (local_1e8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_138 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_138)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  puVar10 = &DAT_142c6c8f3;
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  ::utils::name_hash::NameHash::compute();
  bevy_ecs::system::commands::Commands::spawn_empty();
  if (plStack_130 == (longlong *)0x0) {
    plStack_130 = local_128;
  }
  lVar11 = plStack_130[2];
  if ((ulonglong)(*plStack_130 - lVar11) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plStack_130[1];
  *(code **)(lVar4 + lVar11) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::AlbedoMap>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar2 = (undefined8 *)(lVar4 + 8 + lVar11);
  *puVar2 = 0;
  puVar2[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar4 + 0x18 + lVar11) = 0;
  *(undefined4 *)(lVar4 + 0x20 + lVar11) = 0;
  *(undefined8 *)(lVar4 + 0x24 + lVar11) = 0x3f8000003f800000;
  *(undefined4 *)(lVar4 + 0x2c + lVar11) = 0x3f800000;
  *(undefined8 *)(lVar4 + 0x38 + lVar11) = extraout_RAX_01;
  *(undefined **)(lVar4 + 0x40 + lVar11) = puVar10;
  *(undefined8 *)(lVar4 + 0x48 + lVar11) = extraout_RAX_03;
  *(undefined1 *)(lVar4 + 0x58 + lVar11) = 0;
  *(undefined8 *)(lVar4 + 0x68 + lVar11) = local_118;
  *(code **)(lVar4 + 0x78 + lVar11) = bevy_ecs::error::handler::panic;
  plStack_130[2] = lVar11 + 0x88;
  return;
}


