// name=color_cursor_icon
// sig=void __cdecl color_cursor_icon(void)
// size=580
// address=0x141c9b510

void __cdecl system_color::ui_color_wall_and_roof::color_cursor_icon(void)

{
  longlong lVar1;
  longlong lVar2;
  char *extraout_RAX;
  longlong *plVar3;
  char in_R8B;
  longlong *local_198;
  longlong *local_190;
  undefined8 local_180;
  
  country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
  if (in_R8B == '\0') {
    plVar3 = local_198;
    if (local_198 == (longlong *)0x0) {
      plVar3 = local_190;
    }
    lVar1 = plVar3[2];
    if ((ulonglong)(*plVar3 - lVar1) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = plVar3[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::UiInactive>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar2 + 8 + lVar1) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = local_180;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    plVar3[2] = lVar1 + 0x20;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (char *)0x0) {
    builtin_strncpy(extraout_RAX,"color_icon/color_cursor",0x17);
    bevy_ecs::system::commands::EntityCommands::
    insert<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage>_>();
    if (local_198 == (longlong *)0x0) {
      local_198 = local_190;
    }
    lVar1 = local_198[2];
    if ((ulonglong)(*local_198 - lVar1) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = local_198[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::Ui2DFollowMouse>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar2 + 8 + lVar1) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = local_180;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    local_198[2] = lVar1 + 0x20;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


