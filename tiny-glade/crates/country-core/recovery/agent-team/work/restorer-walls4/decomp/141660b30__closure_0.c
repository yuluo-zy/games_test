// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure_0
          (void)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *extraout_RAX;
  longlong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong *plVar4;
  undefined8 local_68;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  lVar3 = in_RDX[2];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    lVar2 = in_RDX[1];
    *extraout_RAX_00 = *in_RDX;
    extraout_RAX_00[1] = lVar2;
    *(int *)(extraout_RAX_00 + 2) = (int)lVar3;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (undefined8 *)0x0) {
      *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
      *extraout_RAX_01 = 0x635f786574726576;
      extraout_RAX_01[1] = 0x6168735f726f6c6f;
      systems::ui::signifiers::SignifierEntityBuilder::build();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = &DAT_142c82528;
      }
      uVar1 = *(undefined4 *)((longlong)in_RDX + 0x14);
      lVar3 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar3) < 0x28) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar2 = plVar4[1];
      *(code **)(lVar2 + lVar3) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(undefined8 *)(lVar2 + 8 + lVar3) = local_68;
      *(undefined4 *)(lVar2 + 0x10 + lVar3) = 1;
      *(undefined4 *)(lVar2 + 0x14 + lVar3) = uVar1;
      *(undefined1 *)(lVar2 + 0x18 + lVar3) = 0;
      *(code **)(lVar2 + 0x20 + lVar3) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar3 + 0x28;
      *extraout_RAX = local_68;
      *in_RCX = 1;
      in_RCX[1] = extraout_RAX;
      in_RCX[2] = 1;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

