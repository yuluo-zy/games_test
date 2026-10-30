
void __cdecl
bevy_ecs::system::commands::EntityCommands::
insert<country_core::components::AutoClutterConvertableToDecoClutter>(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  longlong in_RCX;
  undefined8 *in_RDX;
  longlong lVar5;
  longlong *plVar6;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined4 local_47;
  undefined2 local_43;
  undefined1 local_41;
  
  lVar3 = *(longlong *)(in_RCX + 0x20);
  plVar6 = *(longlong **)(in_RCX + 8);
  if (plVar6 == (longlong *)0x0) {
    plVar6 = *(longlong **)(in_RCX + 0x10);
    local_58 = in_RDX[10];
    uStack_50 = in_RDX[0xb];
    local_68 = in_RDX[8];
    uStack_60 = in_RDX[9];
    local_a8 = *in_RDX;
    uStack_a0 = in_RDX[1];
    local_98 = in_RDX[2];
    uStack_90 = in_RDX[3];
    local_88 = in_RDX[4];
    uStack_80 = in_RDX[5];
    local_78 = in_RDX[6];
    uStack_70 = in_RDX[7];
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 0x80) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  else {
    local_58 = in_RDX[10];
    uStack_50 = in_RDX[0xb];
    local_68 = in_RDX[8];
    uStack_60 = in_RDX[9];
    local_a8 = *in_RDX;
    uStack_a0 = in_RDX[1];
    local_98 = in_RDX[2];
    uStack_90 = in_RDX[3];
    local_88 = in_RDX[4];
    uStack_80 = in_RDX[5];
    local_78 = in_RDX[6];
    uStack_70 = in_RDX[7];
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 0x80) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  lVar4 = plVar6[1];
  *(code **)(lVar4 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::AutoClutterConvertableToDecoClutter>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar4 + 8 + lVar5);
  *puVar1 = local_a8;
  puVar1[1] = uStack_a0;
  puVar1 = (undefined8 *)(lVar4 + 0x18 + lVar5);
  *puVar1 = local_98;
  puVar1[1] = uStack_90;
  puVar1 = (undefined8 *)(lVar4 + 0x28 + lVar5);
  *puVar1 = local_88;
  puVar1[1] = uStack_80;
  puVar1 = (undefined8 *)(lVar4 + 0x38 + lVar5);
  *puVar1 = local_78;
  puVar1[1] = uStack_70;
  puVar1 = (undefined8 *)(lVar4 + 0x48 + lVar5);
  *puVar1 = local_68;
  puVar1[1] = uStack_60;
  puVar1 = (undefined8 *)(lVar4 + 0x58 + lVar5);
  *puVar1 = local_58;
  puVar1[1] = uStack_50;
  plVar2 = (longlong *)(lVar4 + 0x68 + lVar5);
  *plVar2 = (ulonglong)CONCAT16(local_41,CONCAT24(local_43,local_47)) << 8;
  plVar2[1] = lVar3;
  *(code **)(lVar4 + 0x78 + lVar5) = bevy_ecs::error::handler::panic;
  plVar6[2] = lVar5 + 0x80;
  return;
}


