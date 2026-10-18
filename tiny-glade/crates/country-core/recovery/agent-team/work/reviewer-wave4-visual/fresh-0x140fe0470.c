
void __cdecl
bevy_ecs::system::commands::Commands::
spawn<tuple_<toolbar::resouces_and_components::TaffyNode,toolbar::resouces_and_components::UiScaleWithUserSetting,country_core::components::UiInteractionState,toolbar::resouces_and_components::UiAnimationState,country_core::components::UiMouseInteractable,country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiDrawOrder>_>
          (void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 *in_RCX;
  longlong lVar10;
  longlong *plVar11;
  undefined8 *in_R8;
  undefined8 local_78;
  longlong *plStack_70;
  longlong *local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  
  spawn_empty();
  uVar2 = *in_R8;
  uVar3 = in_R8[1];
  if (plStack_70 == (longlong *)0x0) {
    lVar10 = local_68[2];
    plVar11 = local_68;
    if ((ulonglong)(*local_68 - lVar10) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  else {
    lVar10 = plStack_70[2];
    plVar11 = plStack_70;
    if ((ulonglong)(*plStack_70 - lVar10) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  lVar4 = plVar11[1];
  *(code **)(lVar4 + lVar10) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<toolbar::resouces_and_components::TaffyNode,toolbar::resouces_and_components::UiScaleWithUserSetting,country_core::components::UiInteractionState,toolbar::resouces_and_components::UiAnimationState,country_core::components::UiMouseInteractable,country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::components::UiDrawOrder>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar4 + 8 + lVar10) = uVar2;
  *(undefined8 *)(lVar4 + 0x10 + lVar10) = uVar3;
  uVar2 = in_R8[3];
  uVar3 = in_R8[4];
  uVar5 = in_R8[5];
  uVar6 = in_R8[6];
  uVar7 = in_R8[7];
  uVar8 = in_R8[8];
  uVar9 = in_R8[9];
  puVar1 = (undefined8 *)(lVar4 + 0x18 + lVar10);
  *puVar1 = in_R8[2];
  puVar1[1] = uVar2;
  puVar1 = (undefined8 *)(lVar4 + 0x28 + lVar10);
  *puVar1 = uVar3;
  puVar1[1] = uVar5;
  puVar1 = (undefined8 *)(lVar4 + 0x38 + lVar10);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  puVar1 = (undefined8 *)(lVar4 + 0x48 + lVar10);
  *puVar1 = uVar8;
  puVar1[1] = uVar9;
  uVar2 = in_R8[0xb];
  puVar1 = (undefined8 *)(lVar4 + 0x58 + lVar10);
  *puVar1 = in_R8[10];
  puVar1[1] = uVar2;
  *(undefined8 *)(lVar4 + 0x68 + lVar10) = in_R8[0xc];
  *(undefined1 *)(lVar4 + 0x70 + lVar10) = 0;
  *(undefined8 *)(lVar4 + 0x78 + lVar10) = local_58;
  *(code **)(lVar4 + 0x80 + lVar10) = bevy_ecs::error::handler::panic;
  plVar11[2] = lVar10 + 0x88;
  in_RCX[4] = local_58;
  in_RCX[2] = local_68;
  in_RCX[3] = uStack_60;
  *in_RCX = local_78;
  in_RCX[1] = plStack_70;
  return;
}

