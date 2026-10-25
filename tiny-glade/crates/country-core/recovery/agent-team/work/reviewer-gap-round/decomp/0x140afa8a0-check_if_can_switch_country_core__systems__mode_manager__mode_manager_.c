
void __cdecl
country_core::systems::mode_manager::
check_if_can_switch<country_core::systems::mode_manager::mode_manager::closure_env_0>(void)

{
  byte bVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  char in_CL;
  longlong *in_RDX;
  longlong lVar10;
  undefined8 *in_R8;
  
  if (in_CL == '\0') {
    puVar6 = (undefined8 *)*in_R8;
    plVar7 = (longlong *)in_R8[1];
    puVar8 = (undefined4 *)in_R8[2];
    plVar9 = (longlong *)in_R8[3];
    *(undefined4 *)puVar6[2] = *(undefined4 *)((longlong)puVar6 + 0x1c);
    *(undefined4 *)*puVar6 = 0xd;
    uVar2 = *puVar8;
    *(undefined4 *)plVar7[2] = *(undefined4 *)((longlong)plVar7 + 0x1c);
    lVar10 = *plVar7;
    lVar3 = *(longlong *)(lVar10 + 0x30);
    uVar4 = *(undefined8 *)(lVar10 + 0x40);
    if (lVar3 == *(longlong *)(lVar10 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar5 = *(longlong *)(lVar10 + 0x28);
    *(undefined8 *)(lVar5 + lVar3 * 0x10) = uVar4;
    *(undefined4 *)(lVar5 + 8 + lVar3 * 0x10) = uVar2;
    *(longlong *)(lVar10 + 0x30) = lVar3 + 1;
    *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
    bVar1 = *(byte *)((longlong)puVar8 + 1);
    if (1 < bVar1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    *(undefined4 *)plVar9[2] = *(undefined4 *)((longlong)plVar9 + 0x1c);
    lVar3 = *plVar9;
    lVar10 = *(longlong *)(lVar3 + 0x30);
    uVar4 = *(undefined8 *)(lVar3 + 0x40);
    if (lVar10 == *(longlong *)(lVar3 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar5 = *(longlong *)(lVar3 + 0x28);
    *(undefined8 *)(lVar5 + lVar10 * 0x10) = uVar4;
    *(byte *)(lVar5 + 8 + lVar10 * 0x10) = bVar1;
  }
  else {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar3 = *in_RDX;
    lVar10 = *(longlong *)(lVar3 + 0x30);
    uVar4 = *(undefined8 *)(lVar3 + 0x40);
    if (lVar10 == *(longlong *)(lVar3 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar5 = *(longlong *)(lVar3 + 0x28);
    *(undefined4 *)(lVar5 + lVar10 * 0x20) = 4;
    *(undefined8 *)(lVar5 + 0x18 + lVar10 * 0x20) = uVar4;
  }
  *(longlong *)(lVar3 + 0x30) = lVar10 + 1;
  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  return;
}

