// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::
closure_0(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong *plStack_80;
  longlong *local_78;
  undefined8 local_68;
  
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
    if (plStack_80 == (longlong *)0x0) {
      plStack_80 = local_78;
    }
    lVar2 = plStack_80[2];
    if ((ulonglong)(*plStack_80 - lVar2) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plStack_80[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(lVar3 + 8 + lVar2) = local_68;
    *(undefined8 *)(lVar3 + 0x10 + lVar2) = 0x4040000000000001;
    *(undefined1 *)(lVar3 + 0x18 + lVar2) = 0;
    *(code **)(lVar3 + 0x20 + lVar2) = bevy_ecs::error::handler::panic;
    lVar3 = lVar2 + 0x28;
    plStack_80[2] = lVar3;
    uVar4 = in_RDX[2];
    if ((ulonglong)(*plStack_80 - lVar3) < 0x48) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plStack_80[1];
    *(code **)(lVar5 + 0x28 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 0x30 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar5 + 0x38 + lVar2) = local_68;
    uVar6 = in_RDX[1];
    puVar1 = (undefined8 *)(lVar5 + 0x18 + lVar3);
    *puVar1 = *in_RDX;
    puVar1[1] = uVar6;
    *(undefined8 *)(lVar5 + 0x50 + lVar2) = uVar4;
    *(undefined4 *)(lVar5 + 0x38 + lVar3) = 0;
    *(undefined8 *)(lVar5 + 0x30 + lVar3) = 0;
    *(undefined1 *)(lVar5 + 0x68 + lVar2) = 0;
    plStack_80[2] = lVar2 + 0x70;
    if ((ulonglong)(*plStack_80 - (lVar2 + 0x70)) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plStack_80[1];
    *(code **)(lVar3 + 0x70 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar3 + 0x80 + lVar2) = local_68;
    *(undefined1 *)(lVar3 + 0x88 + lVar2) = 0;
    plStack_80[2] = lVar2 + 0x90;
    *extraout_RAX = local_68;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

