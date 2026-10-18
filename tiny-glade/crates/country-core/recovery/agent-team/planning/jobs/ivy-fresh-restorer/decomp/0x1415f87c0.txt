
void __cdecl country_core::startup::startup_garden_brush::startup_garden_brush(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  longlong *plVar4;
  longlong local_f0;
  longlong *plStack_e8;
  longlong *local_e0;
  undefined8 local_d0;
  
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_f0)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar4 = plStack_e8;
  if (plStack_e8 == (longlong *)0x0) {
    plVar4 = local_e0;
  }
  lVar2 = plVar4[2];
  if ((ulonglong)(*plVar4 - lVar2) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = plVar4[1];
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
  *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX;
  *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_00;
  *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
  *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d0;
  *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
  plVar4[2] = lVar2 + 0x78;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (!SBORROW8(0,local_f0)) {
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
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
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_01;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_02;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x58 + lVar2) = local_d0;
    *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plStack_e8[2] = lVar2 + 0x78;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

