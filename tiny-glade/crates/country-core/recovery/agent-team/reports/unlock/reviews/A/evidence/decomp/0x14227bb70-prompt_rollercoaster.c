// 0x14227bb70 sig void __cdecl prompt_rollercoaster(void)

void __cdecl system_onboarding::prompt_rollercoaster(void)

{
  undefined8 *puVar1;
  ulonglong *puVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  undefined8 *in_RCX;
  longlong lVar9;
  longlong *in_RDX;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined8 local_d4;
  undefined4 local_cc;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined4 local_8e;
  undefined2 local_8a;
  undefined2 uStack_84;
  undefined2 uStack_82;
  undefined4 uStack_80;
  
  puVar2 = (ulonglong *)*in_RCX;
  lVar3 = in_RCX[1];
  uVar4 = *puVar2;
  uVar11 = 0;
  uVar12 = uVar4 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar4 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar12 = uVar11;
  }
  uVar10 = uVar4 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar4 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar10 = uVar11;
  }
  uVar4 = *(ulonglong *)(lVar3 + 0x30);
  uVar13 = uVar4 - uVar10;
  if (uVar4 < uVar10) {
    uVar13 = uVar11;
  }
  uVar5 = *(ulonglong *)(lVar3 + 0x10);
  uVar14 = uVar5 - uVar12;
  if (uVar5 < uVar12) {
    uVar14 = uVar11;
  }
  uVar11 = *(ulonglong *)(lVar3 + 0x40);
  *puVar2 = uVar11 - (uVar14 + uVar13);
  if (uVar5 >= uVar12 && uVar5 - uVar12 != 0 || uVar4 >= uVar10 && uVar4 - uVar10 != 0) {
    *puVar2 = uVar11;
    country_core::resources::steam::impl$31::from();
    hint_orchestrator::BindingHintDisplay::
    alternate<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar3 = *in_RDX;
    uVar6 = *(undefined8 *)(lVar3 + 0x40);
    uStack_84 = (undefined2)local_d4;
    uStack_82 = (undefined2)((ulonglong)local_d4 >> 0x10);
    uStack_80 = (undefined4)((ulonglong)local_d4 >> 0x20);
    lVar7 = *(longlong *)(lVar3 + 0x30);
    if (lVar7 == *(longlong *)(lVar3 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar8 = *(longlong *)(lVar3 + 0x28);
    lVar9 = lVar7 * 0x58;
    *(undefined8 *)(lVar8 + 0x50 + lVar9) = uVar6;
    puVar1 = (undefined8 *)(lVar8 + 0x40 + lVar9);
    *puVar1 = local_98;
    puVar1[1] = CONCAT26(local_8a,CONCAT42(local_8e,0x1200));
    puVar1 = (undefined8 *)(lVar8 + 0x30 + lVar9);
    *puVar1 = local_a8;
    puVar1[1] = uStack_a0;
    puVar1 = (undefined8 *)(lVar8 + 0x20 + lVar9);
    *puVar1 = local_b8;
    puVar1[1] = uStack_b0;
    puVar1 = (undefined8 *)(lVar8 + 0x10 + lVar9);
    *puVar1 = local_c8;
    puVar1[1] = uStack_c0;
    *(undefined8 *)(lVar8 + lVar9) = CONCAT26(uStack_82,CONCAT24(uStack_84,10));
    ((undefined8 *)(lVar8 + lVar9))[1] = CONCAT44(local_cc,uStack_80);
    *(longlong *)(lVar3 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  }
  return;
}

