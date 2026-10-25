// addr=0x140864430 name=startup_sky_and_sun
// signature=void __cdecl startup_sky_and_sun(void)


void __cdecl country_core::startup::startup_sky_and_sun::startup_sky_and_sun(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong *plVar4;
  longlong local_c8;
  longlong *plStack_c0;
  longlong *local_b8;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (!SBORROW8(0,local_c8)) {
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    ::utils::components::transform::impl$1::default();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar4 = plStack_c0;
    if (plStack_c0 == (longlong *)0x0) {
      plVar4 = local_b8;
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
    plVar1 = (longlong *)(lVar3 + 8 + lVar2);
    *plVar1 = local_c8;
    plVar1[1] = (longlong)plStack_c0;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0xc000000080000000;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0x80000000;
    *(ulonglong *)(lVar3 + 0x24 + lVar2) = CONCAT44(uStack_a8,uStack_ac);
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = uStack_a4;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_00;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(ulonglong *)(lVar3 + 0x58 + lVar2) = CONCAT44(uStack_a4,uStack_a8);
    *(code **)(lVar3 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plVar4[2] = lVar2 + 0x78;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


