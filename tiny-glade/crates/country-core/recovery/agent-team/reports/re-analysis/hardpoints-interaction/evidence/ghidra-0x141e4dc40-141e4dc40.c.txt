
void __cdecl toolbar::resouces_and_components::toolbar_layout::add_toolbar_item(void)

{
  longlong lVar1;
  byte bVar2;
  longlong lVar3;
  void *_Src;
  size_t _Size;
  longlong lVar4;
  longlong lVar5;
  uint uVar6;
  void *extraout_RAX;
  byte *in_R9;
  longlong lVar7;
  longlong lVar8;
  void *_Dst;
  undefined8 in_stack_00000030;
  int local_e8;
  int local_c8;
  longlong *plStack_c0;
  longlong *local_b8;
  undefined8 local_a8;
  longlong local_98;
  ulonglong local_70;
  
  bVar2 = *in_R9;
  uVar6 = (uint)bVar2;
  if (uVar6 == 4) {
    lVar3 = *(longlong *)(in_R9 + 0x18);
    local_98 = *(longlong *)(in_R9 + 8);
    lVar4 = *(longlong *)(in_R9 + 0x10);
    taffy::node::Taffy::new_leaf();
    if (local_e8 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    taffy::node::Taffy::add_child();
    if (local_c8 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    lVar8 = lVar3 * 0x40;
    lVar1 = lVar4 + lVar8;
    lVar5 = lVar4;
    while ((lVar7 = lVar4, lVar3 != 0 && (lVar7 = lVar5 + 0x40, *(char *)(lVar5 + 0x38) != '\x02')))
    {
      add_toolbar_item();
      lVar8 = lVar8 + -0x40;
      lVar4 = lVar1;
      lVar5 = lVar7;
      lVar3 = lVar8;
    }
    local_70 = 0;
    while ((ulonglong)(lVar1 - lVar7) >> 6 != local_70) {
      local_70 = local_70 + 1;
      core::ptr::
      drop_in_place<enum2_<toolbar::resouces_and_components::toolbar_layout::ToolbarItemKind>_>();
    }
  }
  else {
    if (bVar2 != 5) {
      taffy::node::Taffy::new_leaf();
      if (local_e8 != 4) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      taffy::node::Taffy::add_child();
      if (local_c8 != 4) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      _Src = *(void **)(in_R9 + 0x20);
      _Size = *(size_t *)(in_R9 + 0x28);
      if (-1 < (longlong)_Size) {
        if (_Size == 0) {
          _Dst = (void *)0x1;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          _Dst = extraout_RAX;
          if (extraout_RAX == (void *)0x0) goto LAB_141e4ef08;
        }
        memcpy(_Dst,_Src,_Size);
        bevy_ecs::system::commands::Commands::
        spawn<tuple_<toolbar::resouces_and_components::TaffyNode,toolbar::resouces_and_components::UiScaleWithUserSetting,country_core::components::UiInteractionState,toolbar::resouces_and_components::UiAnimationState,country_core::components::UiMouseInteractable,country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiDrawOrder>_>
                  ();
        if (plStack_c0 == (longlong *)0x0) {
          plStack_c0 = local_b8;
        }
        lVar3 = plStack_c0[2];
        if ((ulonglong)(*plStack_c0 - lVar3) < 0x28) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar4 = plStack_c0[1];
        *(code **)(lVar4 + lVar3) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<toolbar::resouces_and_components::UiShowIfParentIsInFocus>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar4 + 8 + lVar3) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar4 + 0x10 + lVar3) = local_a8;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = in_stack_00000030;
        *(undefined1 *)(lVar4 + 0x20 + lVar3) = 0;
        plStack_c0[2] = lVar3 + 0x28;
        if (in_R9[0x38] != 0) {
          if ((ulonglong)(*plStack_c0 - (lVar3 + 0x28)) < 0x20) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar4 = plStack_c0[1];
          *(code **)(lVar4 + 0x28 + lVar3) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<toolbar::resouces_and_components::ShowOnlyInFpvAndFree>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar4 + 0x30 + lVar3) = bevy_ecs::error::handler::panic;
          *(undefined8 *)(lVar4 + 0x38 + lVar3) = local_a8;
          *(undefined1 *)(lVar4 + 0x40 + lVar3) = 0;
          plStack_c0[2] = lVar3 + 0x48;
                    // WARNING: Could not recover jumptable at 0x000141e4e5db. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)(&DAT_142f8f774 + *(int *)(&DAT_142f8f774 + (ulonglong)uVar6 * 4)))();
          return;
        }
                    // WARNING: Could not recover jumptable at 0x000141e4e5f1. Too many branches
                    // WARNING: Treating indirect jump as call
        (*(code *)(&DAT_142f8f774 + *(int *)(&DAT_142f8f774 + (ulonglong)uVar6 * 4)))();
        return;
      }
LAB_141e4ef08:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    lVar3 = *(longlong *)(in_R9 + 0x18);
    local_98 = *(longlong *)(in_R9 + 8);
    lVar4 = *(longlong *)(in_R9 + 0x10);
    taffy::node::Taffy::new_leaf();
    if (local_e8 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    taffy::node::Taffy::add_child();
    if (local_c8 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    lVar8 = lVar3 * 0x40;
    lVar1 = lVar4 + lVar8;
    lVar5 = lVar4;
    while ((lVar7 = lVar4, lVar3 != 0 && (lVar7 = lVar5 + 0x40, *(char *)(lVar5 + 0x38) != '\x02')))
    {
      add_toolbar_item();
      lVar8 = lVar8 + -0x40;
      lVar4 = lVar1;
      lVar5 = lVar7;
      lVar3 = lVar8;
    }
    local_70 = 0;
    while ((ulonglong)(lVar1 - lVar7) >> 6 != local_70) {
      local_70 = local_70 + 1;
      core::ptr::
      drop_in_place<enum2_<toolbar::resouces_and_components::toolbar_layout::ToolbarItemKind>_>();
    }
  }
  if (local_98 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((bVar2 & 6) != 4) {
    core::ptr::
    drop_in_place<enum2_<toolbar::resouces_and_components::toolbar_layout::ToolbarItemKind>_>();
    return;
  }
  return;
}

