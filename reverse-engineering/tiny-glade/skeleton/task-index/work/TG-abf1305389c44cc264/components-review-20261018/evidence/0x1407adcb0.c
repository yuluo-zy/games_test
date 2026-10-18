
void __cdecl system_decorator::ui_systems::ui_place_decorator::decorator_cursor_icon(void)

{
  int iVar1;
  longlong lVar2;
  size_t _Size;
  longlong lVar3;
  void *_Dst;
  byte bVar4;
  char in_R8B;
  longlong *local_198;
  longlong *local_190;
  undefined8 local_180;
  
  bVar4 = 6;
  if ((byte)(in_R8B - 0x38U) < 8) {
    bVar4 = in_R8B - 0x38U;
  }
  lVar2 = *(longlong *)(&DAT_142a830b0 + (ulonglong)bVar4 * 8);
  country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
  iVar1 = *(int *)(&DAT_142a830f0 + lVar2 * 4);
  _Size = *(size_t *)(&DAT_142a83108 + lVar2 * 8);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,&DAT_142a830f0 + iVar1,_Size);
    bevy_ecs::system::commands::EntityCommands::
    insert<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiInactive>_>
              ();
    if (local_198 == (longlong *)0x0) {
      local_198 = local_190;
    }
    lVar2 = local_198[2];
    if ((ulonglong)(*local_198 - lVar2) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = local_198[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::Ui2DFollowMouse>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar3 + 8 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar3 + 0x10 + lVar2) = local_180;
    *(undefined1 *)(lVar3 + 0x18 + lVar2) = 0;
    local_198[2] = lVar2 + 0x20;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

