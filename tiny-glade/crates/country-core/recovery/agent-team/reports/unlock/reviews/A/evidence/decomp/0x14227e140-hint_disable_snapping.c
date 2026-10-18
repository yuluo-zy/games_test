// 0x14227e140 sig void __cdecl hint_disable_snapping(void)

void __cdecl system_onboarding::hint_disable_snapping::hint_disable_snapping(void)

{
  undefined8 *puVar1;
  byte bVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  char *pcVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 *in_RCX;
  longlong lVar10;
  longlong *in_RDX;
  ulonglong uVar11;
  longlong lVar12;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined2 uStack_ac;
  undefined2 uStack_aa;
  undefined4 uStack_a8;
  undefined6 uStack_66;
  
  puVar4 = (ulonglong *)*in_R9;
  lVar18 = in_R9[1];
  uVar5 = *puVar4;
  uVar11 = 0;
  uVar14 = uVar5 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar14 = uVar11;
  }
  uVar13 = uVar5 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar13 = uVar11;
  }
  uVar5 = *(ulonglong *)(lVar18 + 0x10);
  uVar6 = *(ulonglong *)(lVar18 + 0x30);
  uVar15 = uVar6 - uVar13;
  if (uVar6 < uVar13) {
    uVar15 = uVar11;
  }
  uVar17 = uVar5 - uVar14;
  if (uVar5 < uVar14) {
    uVar17 = uVar11;
  }
  uVar11 = *(ulonglong *)(lVar18 + 0x40);
  lVar16 = *(longlong *)(lVar18 + 8);
  lVar18 = *(longlong *)(lVar18 + 0x28);
  *puVar4 = (uVar11 - uVar15) - uVar17;
  if (uVar5 >= uVar14 && uVar5 - uVar14 != 0 || uVar6 >= uVar13 && uVar6 - uVar13 != 0) {
    lVar18 = lVar18 + uVar13 * 0x10;
    lVar16 = lVar16 + uVar14 * 0x10;
    if (uVar5 < uVar14) {
      lVar16 = 8;
    }
    lVar12 = 0;
    if (uVar14 < uVar5) {
      lVar12 = lVar16 + -0x10 + uVar17 * 0x10;
    }
    if (uVar6 < uVar13) {
      lVar18 = 8;
    }
    lVar18 = lVar18 + -0x10 + uVar15 * 0x10;
    if (uVar6 <= uVar13) {
      lVar18 = lVar12;
    }
    *puVar4 = uVar11;
    if (*(char *)(lVar18 + 8) == '\0') {
      lVar18 = *in_RDX;
      if (((*(char *)(lVar18 + 0x1fc) == '\0') && (*(char *)*in_R8 == '\0')) &&
         (*(int *)*in_RCX == 6)) {
        bVar2 = *(byte *)((int *)*in_RCX + 1);
        country_core::resources::steam::impl$31::from();
        uVar3 = *(undefined4 *)(lVar18 + 0x128);
        uVar8 = *(undefined8 *)(lVar18 + 0x120);
        hint_orchestrator::BindingHintDisplay::
        alternate<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
        *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        lVar18 = *in_stack_00000030;
        uVar9 = *(undefined8 *)(lVar18 + 0x40);
        uStack_ac = (undefined2)uVar8;
        uStack_aa = (undefined2)((ulonglong)uVar8 >> 0x10);
        uStack_a8 = (undefined4)((ulonglong)uVar8 >> 0x20);
        lVar16 = *(longlong *)(lVar18 + 0x30);
        if (lVar16 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                    ();
        }
        lVar12 = *(longlong *)(lVar18 + 0x28);
        lVar10 = lVar16 * 0x58;
        *(undefined8 *)(lVar12 + 0x50 + lVar10) = uVar9;
        puVar1 = (undefined8 *)(lVar12 + 0x40 + lVar10);
        *puVar1 = uStack_100;
        puVar1[1] = CONCAT62(uStack_66,0x300);
        puVar1 = (undefined8 *)(lVar12 + 0x30 + lVar10);
        *puVar1 = uStack_110;
        puVar1[1] = local_108;
        puVar1 = (undefined8 *)(lVar12 + 0x20 + lVar10);
        *puVar1 = uStack_120;
        puVar1[1] = local_118;
        puVar1 = (undefined8 *)(lVar12 + 0x10 + lVar10);
        *puVar1 = CONCAT44(uStack_e8,uStack_ec);
        puVar1[1] = local_128;
        *(undefined8 *)(lVar12 + lVar10) = CONCAT26(uStack_aa,CONCAT24(uStack_ac,bVar2 + 6));
        ((undefined8 *)(lVar12 + lVar10))[1] = CONCAT44(uVar3,uStack_a8);
        *(longlong *)(lVar18 + 0x30) = lVar16 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      }
    }
    else {
      pcVar7 = (char *)*in_R8;
      if (*pcVar7 == '\0') {
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar18 = *in_stack_00000028;
        lVar16 = *(longlong *)(lVar18 + 0x30);
        uVar8 = *(undefined8 *)(lVar18 + 0x40);
        if (lVar16 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar12 = *(longlong *)(lVar18 + 0x28);
        *(undefined8 *)(lVar12 + lVar16 * 0x10) = uVar8;
        *(undefined1 *)(lVar12 + 8 + lVar16 * 0x10) = 3;
        *(longlong *)(lVar18 + 0x30) = lVar16 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      }
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      *pcVar7 = '\x01';
    }
  }
  return;
}

