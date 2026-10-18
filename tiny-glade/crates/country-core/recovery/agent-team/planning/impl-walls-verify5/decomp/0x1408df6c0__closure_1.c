// closure$1  size=671
// void __cdecl closure$1(void)

void __cdecl
country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::
closure_1(void)

{
  void *_Src;
  longlong lVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined8 *extraout_RAX;
  ulonglong _Size;
  longlong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong *_Dst;
  longlong *plVar4;
  undefined8 local_68;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  plVar4 = *(longlong **)(in_RDX + 0x10);
  auVar3._8_8_ = 0;
  auVar3._0_8_ = plVar4;
  _Size = SUB168(auVar3 * ZEXT816(0xc),0);
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (_Size < 0x7ffffffffffffffd)) {
    _Src = *(void **)(in_RDX + 8);
    if (_Size == 0) {
      _Dst = (longlong *)&DAT_00000004;
      plVar4 = (longlong *)0x0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) goto LAB_1408df928;
    }
    memcpy(_Dst,_Src,_Size);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
    *extraout_RAX_01 = 0x635f786574726576;
    extraout_RAX_01[1] = 0x6168735f726f6c6f;
    systems::ui::signifiers::SignifierEntityBuilder::build();
    if (plVar4 == (longlong *)0x0) {
      plVar4 = _Dst;
    }
    lVar1 = plVar4[2];
    if ((ulonglong)(*plVar4 - lVar1) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = plVar4[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar2 + 8 + lVar1) = local_68;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = 0x400ccccd00000001;
    *(undefined1 *)(lVar2 + 0x18 + lVar1) = 0;
    *(code **)(lVar2 + 0x20 + lVar1) = bevy_ecs::error::handler::panic;
    plVar4[2] = lVar1 + 0x28;
    *extraout_RAX = local_68;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
LAB_1408df928:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': 'build', 'address': '14089dd50'}, {'name': 'do_reserve_and_handle<alloc::alloc::Global>', 'address': '1428a9a60'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}]