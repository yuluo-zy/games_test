// closure$0  size=616
// void __cdecl closure$0(void)

// WARNING: Removing unreachable block (ram,0x000140b357e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow::
closure_0(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  undefined8 local_60;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = 0x7272615f74616c66;
  *(undefined2 *)(extraout_RAX_00 + 1) = 0x776f;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
  *extraout_RAX_01 = 0x635f786574726576;
  extraout_RAX_01[1] = 0x6168735f726f6c6f;
  systems::ui::signifiers::SignifierEntityBuilder::build();
  lVar2 = _DAT_0000001a;
  if ((ulonglong)(_DAT_0000000a - _DAT_0000001a) < 0x20) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar1 = _DAT_00000012;
  *(code **)(_DAT_00000012 + lVar2) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar1 + 8 + lVar2) = bevy_ecs::error::handler::panic;
  *(undefined8 *)(lVar1 + 0x10 + lVar2) = local_60;
  *(undefined1 *)(lVar1 + 0x18 + lVar2) = 0;
  _DAT_0000001a = lVar2 + 0x20;
  *extraout_RAX = local_60;
  *in_RCX = 1;
  in_RCX[1] = extraout_RAX;
  in_RCX[2] = 1;
  return;
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': 'build', 'address': '14089dd50'}, {'name': 'do_reserve_and_handle<alloc::alloc::Global>', 'address': '1428a9a60'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}]