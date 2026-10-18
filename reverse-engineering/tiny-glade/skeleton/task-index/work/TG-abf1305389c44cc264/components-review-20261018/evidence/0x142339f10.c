
void __cdecl toolbar::ui_interaction::render_taffy_ui_to_widgets::closure_0(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong *local_78;
  longlong *local_70;
  undefined8 local_60;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = 0x6f72635f6e6f6369;
    *(undefined2 *)(extraout_RAX_00 + 1) = 0x7373;
    bevy_ecs::system::commands::EntityCommands::
    insert<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage>_>();
    if (local_78 == (longlong *)0x0) {
      local_78 = local_70;
    }
    lVar1 = local_78[2];
    if ((ulonglong)(*local_78 - lVar1) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = local_78[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::Ui2DFollowMouse>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar2 + 8 + lVar1) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = local_60;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    local_78[2] = lVar1 + 0x20;
    *extraout_RAX = local_60;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

