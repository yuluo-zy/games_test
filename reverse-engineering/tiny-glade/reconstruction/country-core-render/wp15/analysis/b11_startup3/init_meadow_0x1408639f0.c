// addr=0x1408639f0 name=init_meadow
// signature=void __cdecl init_meadow(void)


void __cdecl country_core::startup::startup_meadow::init_meadow(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong lVar10;
  longlong *plVar11;
  longlong in_stack_00000040;
  longlong local_228;
  undefined8 local_210;
  longlong local_1e8;
  longlong *plStack_1e0;
  longlong *local_1d8;
  undefined8 local_1c8;
  longlong local_138;
  longlong *plStack_130;
  longlong local_128;
  longlong local_c0;
  
  load_shader_into_library<ref_<str_>_>();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (local_138 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (SBORROW8(0,local_c0)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  serde_json::de::
  from_str<alloc::vec::Vec<country_core::startup::startup_meadow::PlacementConfigEntry,alloc::alloc::Global>_>
            ();
  if (SBORROW8(0,local_138)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (local_c0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_1e8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar3 = *(undefined4 *)(in_stack_00000040 + 0x1c);
  puVar4 = *(undefined4 **)(in_stack_00000040 + 0x10);
  plVar11 = plStack_130;
  while (plVar11 != plStack_130 + local_128 * 9) {
    plVar1 = plVar11 + 9;
    lVar5 = *plVar11;
    lVar8 = plVar11[6];
    lVar9 = plVar11[3];
    lVar10 = plVar11[1];
    if (SBORROW8(0,lVar5)) break;
    *puVar4 = uVar3;
    std::path::Path::_join();
    relative_path::RelativePath::to_path<std::path::PathBuf>();
    asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>();
    relative_path::RelativePath::to_path<ref_<std::path::Path>_>();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::path::Path::_with_extension();
    if (local_1e8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>();
    if (SBORROW8(0,local_1e8)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar11 = plStack_1e0;
    if (plStack_1e0 == (longlong *)0x0) {
      plVar11 = local_1d8;
    }
    lVar6 = plVar11[2];
    if ((ulonglong)(*plVar11 - lVar6) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar7 = plVar11[1];
    *(code **)(lVar7 + lVar6) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::asset_libraries::placement_data_library::InstancedDrawFromPlacementData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar2 = (undefined8 *)(lVar7 + 8 + lVar6);
    *puVar2 = 0;
    puVar2[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar7 + 0x18 + lVar6) = 0;
    *(undefined4 *)(lVar7 + 0x20 + lVar6) = 0;
    *(undefined8 *)(lVar7 + 0x24 + lVar6) = 0x3f8000003f800000;
    *(undefined4 *)(lVar7 + 0x2c + lVar6) = 0x3f800000;
    *(undefined8 *)(lVar7 + 0x38 + lVar6) = extraout_RAX;
    *(longlong *)(lVar7 + 0x40 + lVar6) = lVar10;
    *(undefined8 *)(lVar7 + 0x48 + lVar6) = local_210;
    *(undefined1 *)(lVar7 + 0x58 + lVar6) = 0;
    *(undefined8 *)(lVar7 + 0x68 + lVar6) = local_1c8;
    *(code **)(lVar7 + 0x78 + lVar6) = bevy_ecs::error::handler::panic;
    plVar11[2] = lVar6 + 0x88;
    if (local_228 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lVar5 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lVar9 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar11 = plVar1;
    if (lVar8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  core::ptr::
  drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::startup_meadow::PlacementConfigEntry,alloc::alloc::Global>_>
            ();
  return;
}


