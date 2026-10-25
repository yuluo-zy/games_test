// addr=0x140926bf0 name=startup_terrain
// signature=void __cdecl startup_terrain(void)


// WARNING: Removing unreachable block (ram,0x000140927d6e)
// WARNING: Removing unreachable block (ram,0x00014092783b)
// WARNING: Removing unreachable block (ram,0x0001409276d8)
// WARNING: Removing unreachable block (ram,0x000140927578)
// WARNING: Removing unreachable block (ram,0x00014092743e)
// WARNING: Removing unreachable block (ram,0x0001409272c0)
// WARNING: Removing unreachable block (ram,0x000140926fb3)
// WARNING: Removing unreachable block (ram,0x000140926dbe)
// WARNING: Removing unreachable block (ram,0x000140927109)
// WARNING: Removing unreachable block (ram,0x0001409279b3)
// WARNING: Removing unreachable block (ram,0x000140927d30)
// WARNING: Removing unreachable block (ram,0x000140927cf2)
// WARNING: Removing unreachable block (ram,0x000140927cbd)
// WARNING: Removing unreachable block (ram,0x000140927c88)
// WARNING: Removing unreachable block (ram,0x000140927c53)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::startup::startup_terrain::startup_terrain(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong *extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  ulonglong extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  undefined8 extraout_RAX_12;
  ulonglong extraout_RAX_13;
  undefined8 extraout_RAX_14;
  ulonglong extraout_RAX_15;
  undefined8 extraout_RAX_16;
  undefined8 extraout_RAX_17;
  undefined8 uVar4;
  char *pcVar5;
  longlong *plVar6;
  longlong in_stack_00000038;
  undefined8 local_d8;
  
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (longlong *)0x0) {
LAB_140927c16:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  *extraout_RAX = *extraout_RAX + 1;
  grid_gen_cw();
  load_mesh_into_library();
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  lVar2 = _DAT_00000014;
  if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = _DAT_0000000c;
  *(code **)(_DAT_0000000c + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
  *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
  *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_00;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_01;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d8;
  *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
  _DAT_00000014 = lVar2 + 0x78;
  if (_DEV_OVERRIDE_IS_DEMO_MODE != '\0') {
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_02 == (longlong *)0x0) goto LAB_140927c16;
    *extraout_RAX_02 = *extraout_RAX_02 + 1;
    grid_gen_cw();
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    lVar2 = _DAT_00000014;
    if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = _DAT_0000000c;
    *(code **)(_DAT_0000000c + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_03;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
    *(undefined4 *)(lVar3 + 0x48 + lVar2) = 0x10;
    *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x68 + lVar2) = local_d8;
    *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
    _DAT_00000014 = lVar2 + 0x88;
  }
  uVar4 = 0x1e;
  resources::rocky_terrain::RockyTerrainMesh::load();
  if ((extraout_RAX_05 & 1) != 0) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  plVar6 = *(longlong **)(in_stack_00000038 + 8);
  if (plVar6 == (longlong *)0x0) {
    plVar6 = *(longlong **)(in_stack_00000038 + 0x10);
  }
  lVar2 = plVar6[2];
  if ((ulonglong)(*plVar6 - lVar2) < 0x10) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = plVar6[1];
  *(code **)(lVar3 + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar3 + 8 + lVar2) = uVar4;
  plVar6[2] = lVar2 + 0x10;
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  lVar2 = _DAT_00000014;
  if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = _DAT_0000000c;
  *(code **)(_DAT_0000000c + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
  *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
  *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_00;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_06;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d8;
  *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
  _DAT_00000014 = lVar2 + 0x78;
  plVar6 = *(longlong **)(in_stack_00000038 + 8);
  if (plVar6 == (longlong *)0x0) {
    plVar6 = *(longlong **)(in_stack_00000038 + 0x10);
  }
  lVar2 = plVar6[2];
  if ((ulonglong)(*plVar6 - lVar2) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = plVar6[1];
  *(code **)(lVar3 + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<country_core::resources::rocky_terrain::TerrainEntities>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar3 + 8 + lVar2) = local_d8;
  *(undefined8 *)(lVar3 + 0x10 + lVar2) = local_d8;
  plVar6[2] = lVar2 + 0x18;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  lVar2 = _DAT_00000014;
  if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = _DAT_0000000c;
  *(code **)(_DAT_0000000c + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::StairsStep>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
  *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
  *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_07;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_08;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
  *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x68 + lVar2) = local_d8;
  *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
  _DAT_00000014 = lVar2 + 0x88;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  lVar2 = _DAT_00000014;
  if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = _DAT_0000000c;
  *(code **)(_DAT_0000000c + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
  *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
  *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_09;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_10;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d8;
  *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
  _DAT_00000014 = lVar2 + 0x78;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  bevy_ecs::system::commands::Commands::spawn_empty();
  lVar2 = _DAT_00000014;
  if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = _DAT_0000000c;
  *(code **)(_DAT_0000000c + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::StairsStep>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
  *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
  *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_11;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_08;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 1;
  *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x68 + lVar2) = local_d8;
  *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
  _DAT_00000014 = lVar2 + 0x88;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  load_mesh_into_library();
  pcVar5 = "solid_vertex_color_albedo_map";
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_13 & 1) != 0) {
    bevy_ecs::system::commands::Commands::spawn_empty();
    lVar2 = _DAT_00000014;
    if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = _DAT_0000000c;
    *(code **)(_DAT_0000000c + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::BackgroundTerrain,country_core::components::DontReceiveContactShadows,country_core::components::OverwriteRoughness>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_12;
    *(char **)(lVar3 + 0x40 + lVar2) = pcVar5;
    *(undefined4 *)(lVar3 + 0x48 + lVar2) = 0x3f666666;
    *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x68 + lVar2) = local_d8;
    *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
    _DAT_00000014 = lVar2 + 0x88;
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    load_mesh_into_library();
    pcVar5 = "solid_vertex_color";
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_15 & 1) != 0) {
      bevy_ecs::system::commands::Commands::spawn_empty();
      lVar2 = _DAT_00000014;
      if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x78) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = _DAT_0000000c;
      *(code **)(_DAT_0000000c + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::FlatLighting>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_14;
      *(char **)(lVar3 + 0x40 + lVar2) = pcVar5;
      *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
      _DAT_00000014 = lVar2 + 0x78;
      country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
      load_mesh_into_library();
      load_shader_into_library<ref_<str_>_>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      lVar2 = _DAT_00000014;
      if ((ulonglong)(_DAT_00000004 - _DAT_00000014) < 0x78) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = _DAT_0000000c;
      *(code **)(_DAT_0000000c + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_16;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_17;
      *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
      _DAT_00000014 = lVar2 + 0x78;
      return;
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


