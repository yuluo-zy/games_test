
// WARNING: Removing unreachable block (ram,0x000140fae18f)

void __cdecl toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup(void)

{
  undefined8 *puVar1;
  byte bVar2;
  longlong lVar3;
  longlong lVar4;
  char *_Size;
  void *extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong *plVar5;
  ulonglong uVar6;
  void *_Dst;
  char **ppcVar7;
  int local_328;
  undefined4 uStack_324;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  longlong *local_318;
  undefined8 local_308;
  char *local_198;
  undefined8 uStack_190;
  ulonglong local_188;
  char *pcStack_180;
  undefined8 local_178;
  undefined1 local_170;
  undefined8 *local_150;
  ulonglong local_138;
  char *local_130;
  int local_128;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  
  taffy::node::Taffy::new_leaf();
  if (local_328 != 4) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  lVar3 = *(longlong *)(in_RDX + 0xa8);
  if (lVar3 == *(longlong *)(in_RDX + 0x98)) {
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
              ();
  }
  lVar4 = *(longlong *)(in_RDX + 0xa0);
  puVar1 = (undefined8 *)(lVar4 + lVar3 * 0x28);
  *puVar1 = 0;
  puVar1[1] = 0;
  *(ulonglong *)(lVar4 + 0x10 + lVar3 * 0x28) = CONCAT44(uStack_320,uStack_324);
  *(undefined1 *)(lVar4 + 0x24 + lVar3 * 0x28) = 4;
  *(longlong *)(in_RDX + 0xa8) = lVar3 + 1;
  taffy::node::Taffy::new_leaf();
  local_108 = CONCAT44(uStack_320,uStack_324);
  uStack_100 = 0;
  uStack_f0 = 0;
  ppcVar7 = &local_198;
  local_150 = in_RCX;
  local_f8 = CONCAT44(uStack_320,uStack_324);
  taffy::node::Taffy::add_child();
  if ((int)local_198 != 4) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_198 = "icon/undoicon/redo";
  uStack_190 = 9;
  local_188 = local_188 & 0xffffffffffffff00;
  pcStack_180 = "icon/redo";
  local_178 = 9;
  local_170 = 1;
  uVar6 = 0;
  do {
    bVar2 = *(byte *)(ppcVar7 + 2);
    if (bVar2 == 2) break;
    local_130 = *ppcVar7;
    _Size = ppcVar7[1];
    taffy::node::Taffy::new_leaf();
    if (local_128 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    taffy::node::Taffy::add_child();
    if ((longlong)_Size < 0) {
LAB_140fae1d3:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_138 = uVar6;
    if (_Size == (char *)0x0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (void *)0x0) goto LAB_140fae1d3;
    }
    memcpy(_Dst,local_130,(size_t)_Size);
    bevy_ecs::system::commands::Commands::
    spawn<tuple_<toolbar::resouces_and_components::TaffyNode,toolbar::resouces_and_components::UiScaleWithUserSetting,country_core::components::UiInteractionState,toolbar::resouces_and_components::UiAnimationState,country_core::components::UiMouseInteractable,country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiDrawOrder>_>
              ();
    plVar5 = (longlong *)CONCAT44(uStack_31c,uStack_320);
    if ((longlong *)CONCAT44(uStack_31c,uStack_320) == (longlong *)0x0) {
      plVar5 = local_318;
    }
    lVar3 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar3) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar5[1];
    *(code **)(lVar4 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::HistoryButton>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar4 + 8 + lVar3) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar4 + 0x10 + lVar3) = local_308;
    *(byte *)(lVar4 + 0x18 + lVar3) = bVar2 & 1;
    *(undefined1 *)(lVar4 + 0x19 + lVar3) = 0;
    plVar5[2] = lVar3 + 0x20;
    if ((ulonglong)(*plVar5 - (lVar3 + 0x20)) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar5[1];
    *(code **)(lVar4 + 0x20 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<toolbar::resouces_and_components::HideIfMenuIsOpen>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar4 + 0x28 + lVar3) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar4 + 0x30 + lVar3) = local_308;
    *(undefined1 *)(lVar4 + 0x38 + lVar3) = 0;
    plVar5[2] = lVar3 + 0x40;
    ppcVar7 = &pcStack_180;
    uVar6 = CONCAT71((int7)((ulonglong)plVar5 >> 8),1);
  } while ((local_138 & 1) == 0);
  *local_150 = local_f8;
  local_150[1] = local_108;
  *(undefined4 *)(local_150 + 2) = 0;
  return;
}

