
void __cdecl bevy_ecs::system::commands::EntityCommands::despawn(void)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong in_RCX;
  longlong *plVar4;
  
  plVar4 = *(longlong **)(in_RCX + 8);
  if (plVar4 == (longlong *)0x0) {
    plVar4 = *(longlong **)(in_RCX + 0x10);
  }
  uVar1 = *(undefined8 *)(in_RCX + 0x20);
  lVar2 = plVar4[2];
  if ((ulonglong)(*plVar4 - lVar2) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar3 = plVar4[1];
  *(code **)(lVar3 + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::despawn::closure_env_0>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar3 + 8 + lVar2) = bevy_ecs::error::handler::warn;
  *(undefined8 *)(lVar3 + 0x10 + lVar2) = uVar1;
  plVar4[2] = lVar2 + 0x18;
  return;
}

