// name=closure$2
// sig=void __cdecl closure$2(void)
// size=522

void __cdecl system_color::ui_color_gates::ui_color_gates::closure_2(void)

{
  void *_Src;
  longlong lVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined8 *extraout_RAX;
  ulonglong _Size;
  undefined1 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined1 *_Dst;
  longlong *local_170;
  longlong *local_168;
  undefined8 local_158;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  auVar3._8_8_ = 0;
  auVar3._0_8_ = *(ulonglong *)(in_RDX + 0x10);
  _Size = SUB168(auVar3 * ZEXT816(0xc),0);
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (_Size < 0x7ffffffffffffffd)) {
    _Src = *(void **)(in_RDX + 8);
    if (_Size == 0) {
      _Dst = &DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_1422b0d89;
    }
    memcpy(_Dst,_Src,_Size);
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    if (local_170 == (longlong *)0x0) {
      local_170 = local_168;
    }
    lVar1 = local_170[2];
    if ((ulonglong)(*local_170 - lVar1) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = local_170[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar2 + 8 + lVar1) = local_158;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = 0x4040000000000001;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    *(code **)(lVar2 + 0x20 + lVar1) = bevy_ecs::error::handler::panic;
    local_170[2] = lVar1 + 0x28;
    *extraout_RAX = local_158;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
LAB_1422b0d89:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

