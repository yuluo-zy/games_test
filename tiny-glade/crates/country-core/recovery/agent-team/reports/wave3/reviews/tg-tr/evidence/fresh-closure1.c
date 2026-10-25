
void __cdecl system_glade_border::display_glade_border::display_glade_border::closure_1(void)

{
  float fVar1;
  void *_Src;
  longlong lVar2;
  longlong lVar3;
  undefined1 auVar4 [16];
  undefined8 *extraout_RAX;
  ulonglong _Size;
  undefined1 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined1 *_Dst;
  longlong *local_190;
  longlong *local_188;
  undefined8 local_178;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  auVar4._8_8_ = 0;
  auVar4._0_8_ = *(ulonglong *)(in_RDX + 0x10);
  _Size = SUB168(auVar4 * ZEXT816(0xc),0);
  if ((SUB168(auVar4 * ZEXT816(0xc),8) == 0) && (_Size < 0x7ffffffffffffffd)) {
    _Src = *(void **)(in_RDX + 8);
    if (_Size == 0) {
      _Dst = &DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_141be131b;
    }
    memcpy(_Dst,_Src,_Size);
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    if (local_190 == (longlong *)0x0) {
      local_190 = local_188;
    }
    lVar2 = local_190[2];
    if ((ulonglong)(*local_190 - lVar2) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = local_190[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar3 + 8 + lVar2) = local_178;
    *(undefined8 *)(lVar3 + 0x10 + lVar2) = 0x4000000000000001;
    *(undefined1 *)(lVar3 + 0x18 + lVar2) = 0;
    *(code **)(lVar3 + 0x20 + lVar2) = bevy_ecs::error::handler::panic;
    local_190[2] = lVar2 + 0x28;
    fVar1 = *(float *)(in_RDX + 0x18);
    if ((ulonglong)(*local_190 - (lVar2 + 0x28)) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = local_190[1];
    *(code **)(lVar3 + 0x28 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::DashedLineLength>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar3 + 0x30 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = local_178;
    *(float *)(lVar3 + 0x40 + lVar2) = fVar1 * 0.7;
    *(undefined1 *)(lVar3 + 0x44 + lVar2) = 0;
    local_190[2] = lVar2 + 0x48;
    *extraout_RAX = local_178;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
LAB_141be131b:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

