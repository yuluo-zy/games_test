
void __cdecl system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure_3(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  undefined8 uVar8;
  undefined4 uVar9;
  char extraout_AL;
  longlong in_RCX;
  undefined8 *in_RDX;
  undefined8 local_f8;
  undefined3 uStack_f0;
  undefined4 uStack_ed;
  undefined4 uStack_e9;
  undefined4 uStack_e5;
  
  local_f8 = in_RDX[2];
  uVar3 = in_RDX[3];
  move_ops::calculate_move_dst();
  uStack_f0 = (undefined3)uVar3;
  if (*(int *)(in_RDX + 8) == 1) {
    lVar4 = *(longlong *)(in_RCX + 0x10);
    **(undefined4 **)(lVar4 + 0x10) = *(undefined4 *)(lVar4 + 0x1c);
    lVar5 = *(longlong *)(in_RCX + 0x18);
    **(undefined4 **)(lVar5 + 0x10) = *(undefined4 *)(lVar5 + 0x1c);
    lVar6 = *(longlong *)(in_RCX + 0x20);
    **(undefined4 **)(lVar6 + 0x10) = *(undefined4 *)(lVar6 + 0x1c);
    apply_decorator_op::apply_decorator_op();
    uStack_ed = (undefined4)((ulonglong)uVar3 >> 0x18);
    **(undefined4 **)(lVar4 + 0x10) = *(undefined4 *)(lVar4 + 0x1c);
    **(undefined4 **)(lVar5 + 0x10) = *(undefined4 *)(lVar5 + 0x1c);
    **(undefined4 **)(lVar6 + 0x10) = *(undefined4 *)(lVar6 + 0x1c);
  }
  else {
    local_f8 = *in_RDX;
    uStack_f0 = (undefined3)in_RDX[1];
    uStack_ed = (undefined4)((ulonglong)in_RDX[1] >> 0x18);
    **(undefined4 **)(*(longlong *)(in_RCX + 0x10) + 0x10) =
         *(undefined4 *)(*(longlong *)(in_RCX + 0x10) + 0x1c);
    **(undefined4 **)(*(longlong *)(in_RCX + 0x18) + 0x10) =
         *(undefined4 *)(*(longlong *)(in_RCX + 0x18) + 0x1c);
    **(undefined4 **)(*(longlong *)(in_RCX + 0x20) + 0x10) =
         *(undefined4 *)(*(longlong *)(in_RCX + 0x20) + 0x1c);
  }
  apply_decorator_op::apply_decorator_op();
  country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
  if (extraout_AL == '\x03') {
    plVar7 = *(longlong **)(in_RCX + 0x78);
    uVar2 = *(undefined4 *)(in_RDX + 0xb);
    uVar3 = in_RDX[10];
    uStack_e9 = (undefined4)uVar3;
    uVar9 = uStack_e9;
    uStack_e5 = (undefined4)((ulonglong)uVar3 >> 0x20);
    *(undefined4 *)plVar7[2] = *(undefined4 *)((longlong)plVar7 + 0x1c);
    lVar4 = *plVar7;
    lVar5 = *(longlong *)(lVar4 + 0x30);
    uVar8 = *(undefined8 *)(lVar4 + 0x40);
    if (lVar5 == *(longlong *)(lVar4 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar6 = *(longlong *)(lVar4 + 0x28);
    *(undefined8 *)(lVar6 + lVar5 * 0x28) = uVar8;
    *(undefined1 *)(lVar6 + 8 + lVar5 * 0x28) = 9;
    uStack_e9._0_1_ = (undefined1)uVar3;
    puVar1 = (undefined8 *)(lVar6 + 9 + lVar5 * 0x28);
    *puVar1 = local_f8;
    puVar1[1] = CONCAT17((undefined1)uStack_e9,CONCAT43(uStack_ed,uStack_f0));
    puVar1 = (undefined8 *)(lVar6 + 0x14 + lVar5 * 0x28);
    *puVar1 = CONCAT44(uVar9,uStack_ed);
    puVar1[1] = CONCAT44(uVar2,uStack_e5);
    *(undefined1 *)(lVar6 + 0x24 + lVar5 * 0x28) = 0;
    *(longlong *)(lVar4 + 0x30) = lVar5 + 1;
    *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  }
  return;
}

