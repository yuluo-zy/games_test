
void __cdecl
country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_1
          (void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong *plStack_68;
  longlong *local_60;
  undefined8 local_50;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *(undefined4 *)((longlong)extraout_RAX_00 + 0xf) = 0x72656461;
    *extraout_RAX_00 = 0x635f786574726576;
    extraout_RAX_00[1] = 0x6168735f726f6c6f;
    systems::ui::signifiers::SignifierEntityBuilder::build();
    if (plStack_68 == (longlong *)0x0) {
      plStack_68 = local_60;
    }
    lVar1 = plStack_68[2];
    if ((ulonglong)(*plStack_68 - lVar1) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = plStack_68[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar2 + 8 + lVar1) = local_50;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = 0x4040000000000001;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    *(code **)(lVar2 + 0x20 + lVar1) = bevy_ecs::error::handler::panic;
    plStack_68[2] = lVar1 + 0x28;
    bevy_ecs::system::commands::EntityCommands::
    insert<country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier>
              ();
    *extraout_RAX = local_50;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

