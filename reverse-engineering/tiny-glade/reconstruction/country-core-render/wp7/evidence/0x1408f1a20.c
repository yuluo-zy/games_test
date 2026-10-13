
void __cdecl country_core::startup::startup_water::startup_water(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  longlong *plVar5;
  longlong *in_stack_00000038;
  longlong local_f0;
  longlong *plStack_e8;
  longlong *local_e0;
  undefined8 local_d0;
  
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    lVar2 = *in_stack_00000038;
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_e8;
    if (plStack_e8 == (longlong *)0x0) {
      plVar5 = local_e0;
    }
    lVar3 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar3) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar5[1];
    *(code **)(lVar4 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
    *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
    *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
    *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX;
    *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX_00;
    *(undefined4 *)(lVar4 + 0x48 + lVar3) = 0xf;
    *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x68 + lVar3) = local_d0;
    *(code **)(lVar4 + 0x78 + lVar3) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar3 + 0x88;
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_e8;
    if (plStack_e8 == (longlong *)0x0) {
      plVar5 = local_e0;
    }
    lVar3 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar3) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar5[1];
    *(code **)(lVar4 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
    *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
    *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
    *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX;
    *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX_01;
    *(undefined4 *)(lVar4 + 0x48 + lVar3) = 0xe;
    *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x68 + lVar3) = local_d0;
    *(code **)(lVar4 + 0x78 + lVar3) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar3 + 0x88;
    if (*(char *)(lVar2 + 0x58) == '\x04') {
      load_shader_into_library<ref_<str_>_>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar5 = plStack_e8;
      if (plStack_e8 == (longlong *)0x0) {
        plVar5 = local_e0;
      }
      lVar3 = plVar5[2];
      if ((ulonglong)(*plVar5 - lVar3) < 0x78) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar4 = plVar5[1];
      *(code **)(lVar4 + lVar3) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
      *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
      *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
      *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
      *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX;
      *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX_02;
      *(undefined1 *)(lVar4 + 0x48 + lVar3) = 0;
      *(undefined8 *)(lVar4 + 0x58 + lVar3) = local_d0;
      *(code **)(lVar4 + 0x68 + lVar3) = bevy_ecs::error::handler::panic;
      plVar5[2] = lVar3 + 0x78;
    }
    load_shader_into_library_replace_previous<ref_<str_>_>();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_e8;
    if (plStack_e8 == (longlong *)0x0) {
      plVar5 = local_e0;
    }
    lVar3 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar3) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar5[1];
    *(code **)(lVar4 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
    *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
    *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
    *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX_04;
    *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX_03;
    *(undefined1 *)(lVar4 + 0x48 + lVar3) = 0;
    *(undefined8 *)(lVar4 + 0x58 + lVar3) = local_d0;
    *(code **)(lVar4 + 0x68 + lVar3) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar3 + 0x78;
    load_shader_into_library<ref_<str_>_>();
    if (*(char *)(lVar2 + 0x58) == '\x04') {
      country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
      if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    else {
      country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
      if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    load_mesh_into_library();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_e8;
    if (plStack_e8 == (longlong *)0x0) {
      plVar5 = local_e0;
    }
    lVar2 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar2) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar5[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_06;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_05;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d0;
    *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar2 + 0x78;
    load_shader_into_library<ref_<str_>_>();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_e8;
    if (plStack_e8 == (longlong *)0x0) {
      plVar5 = local_e0;
    }
    lVar2 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar2) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar5[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_08;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_07;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d0;
    *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar2 + 0x78;
    load_shader_into_library<ref_<str_>_>();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    bevy_ecs::system::commands::Commands::spawn_empty();
    if (plStack_e8 == (longlong *)0x0) {
      plStack_e8 = local_e0;
    }
    lVar2 = plStack_e8[2];
    if ((ulonglong)(*plStack_e8 - lVar2) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plStack_e8[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_10;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_09;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d0;
    *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plStack_e8[2] = lVar2 + 0x78;
  }
  return;
}

