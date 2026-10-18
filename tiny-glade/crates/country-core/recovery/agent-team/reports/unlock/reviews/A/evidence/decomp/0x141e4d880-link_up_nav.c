// addr 0x141e4d880 sig void __cdecl link_up_nav(void) size 857

void __cdecl toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::link_up_nav(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  char in_DL;
  ulonglong uVar6;
  longlong lVar7;
  longlong in_R8;
  undefined8 *puVar8;
  ulonglong uVar9;
  longlong *plVar10;
  
  uVar1 = *(ulonglong *)(in_RCX + 0x10);
  if (uVar1 == 0) {
    return;
  }
  lVar7 = *(longlong *)(in_RCX + 8);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (uVar1 < 4) {
    uVar6 = 0;
  }
  else {
    puVar8 = (undefined8 *)(lVar7 + 0x68);
    uVar6 = 0;
    do {
      extraout_RAX[uVar6] = puVar8[-0xc];
      extraout_RAX[uVar6 + 1] = puVar8[-8];
      extraout_RAX[uVar6 + 2] = puVar8[-4];
      extraout_RAX[uVar6 + 3] = *puVar8;
      uVar6 = uVar6 + 4;
      puVar8 = puVar8 + 0x10;
    } while ((uVar1 & 0xfffffffffffffffc) != uVar6);
  }
  if ((uVar1 & 3) != 0) {
    uVar9 = 0;
    do {
      *(undefined8 *)((longlong)extraout_RAX + uVar9 + uVar6 * 8) =
           *(undefined8 *)(uVar6 * 0x20 + lVar7 + 8 + uVar9 * 4);
      uVar9 = uVar9 + 8;
    } while (((uint)uVar1 & 3) << 3 != uVar9);
  }
  plVar10 = *(longlong **)(in_R8 + 8);
  if (plVar10 == (longlong *)0x0) {
    plVar10 = *(longlong **)(in_R8 + 0x10);
  }
  lVar7 = plVar10[2];
  if (in_DL == '\0') {
    uVar6 = 0;
    puVar8 = extraout_RAX;
    do {
      uVar2 = *puVar8;
      uVar9 = (uVar1 - 1) + uVar6;
      if ((uVar9 | uVar1) >> 0x20 == 0) {
        uVar9 = (uVar9 & 0xffffffff) % (uVar1 & 0xffffffff);
      }
      else {
        uVar9 = uVar9 % uVar1;
      }
      uVar6 = uVar6 + 1;
      uVar3 = extraout_RAX[uVar9];
      if ((uVar6 | uVar1) >> 0x20 == 0) {
        uVar9 = (uVar6 & 0xffffffff) % (uVar1 & 0xffffffff);
      }
      else {
        uVar9 = uVar6 % uVar1;
      }
      uVar4 = extraout_RAX[uVar9];
      if ((ulonglong)(*plVar10 - lVar7) < 0x60) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar5 = plVar10[1];
      *(code **)(lVar5 + lVar7) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::UiNav>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(undefined8 *)(lVar5 + 8 + lVar7) = 0;
      *(undefined8 *)(lVar5 + 0x18 + lVar7) = 0;
      *(undefined8 *)(lVar5 + 0x28 + lVar7) = 3;
      *(undefined8 *)(lVar5 + 0x30 + lVar7) = uVar4;
      *(undefined8 *)(lVar5 + 0x38 + lVar7) = 3;
      *(undefined8 *)(lVar5 + 0x40 + lVar7) = uVar3;
      *(undefined1 *)(lVar5 + 0x48 + lVar7) = 0;
      *(undefined8 *)(lVar5 + 0x50 + lVar7) = uVar2;
      *(code **)(lVar5 + 0x58 + lVar7) = bevy_ecs::error::handler::panic;
      lVar7 = lVar7 + 0x60;
      plVar10[2] = lVar7;
      puVar8 = puVar8 + 1;
    } while (puVar8 != extraout_RAX + uVar1);
  }
  else {
    uVar6 = 0;
    puVar8 = extraout_RAX;
    do {
      uVar2 = *puVar8;
      uVar9 = (uVar1 - 1) + uVar6;
      if ((uVar9 | uVar1) >> 0x20 == 0) {
        uVar9 = (uVar9 & 0xffffffff) % (uVar1 & 0xffffffff);
      }
      else {
        uVar9 = uVar9 % uVar1;
      }
      uVar6 = uVar6 + 1;
      uVar3 = extraout_RAX[uVar9];
      if ((uVar6 | uVar1) >> 0x20 == 0) {
        uVar9 = (uVar6 & 0xffffffff) % (uVar1 & 0xffffffff);
      }
      else {
        uVar9 = uVar6 % uVar1;
      }
      uVar4 = extraout_RAX[uVar9];
      if ((ulonglong)(*plVar10 - lVar7) < 0x60) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar5 = plVar10[1];
      *(code **)(lVar5 + lVar7) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::UiNav>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(undefined8 *)(lVar5 + 8 + lVar7) = 3;
      *(undefined8 *)(lVar5 + 0x10 + lVar7) = uVar3;
      *(undefined8 *)(lVar5 + 0x18 + lVar7) = 3;
      *(undefined8 *)(lVar5 + 0x20 + lVar7) = uVar4;
      *(undefined8 *)(lVar5 + 0x28 + lVar7) = 0;
      *(undefined8 *)(lVar5 + 0x38 + lVar7) = 0;
      *(undefined1 *)(lVar5 + 0x48 + lVar7) = 0;
      *(undefined8 *)(lVar5 + 0x50 + lVar7) = uVar2;
      *(code **)(lVar5 + 0x58 + lVar7) = bevy_ecs::error::handler::panic;
      lVar7 = lVar7 + 0x60;
      plVar10[2] = lVar7;
      puVar8 = puVar8 + 1;
    } while (puVar8 != extraout_RAX + uVar1);
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}

