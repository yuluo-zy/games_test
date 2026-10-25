
void __cdecl system_onboarding::hint_terrain_focus::hint_terrain_focus(void)

{
  longlong lVar1;
  char *pcVar2;
  undefined8 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  char *pcVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  longlong lVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  longlong lVar15;
  undefined8 *in_RCX;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  longlong *in_RDX;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 local_134;
  undefined4 local_12c;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined4 local_ee;
  undefined2 local_ea;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  
  puVar6 = (ulonglong *)*in_R9;
  lVar7 = in_R9[1];
  uVar8 = *puVar6;
  uVar19 = 0;
  uVar14 = uVar8 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar14 = uVar19;
  }
  uVar20 = uVar8 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar20 = uVar19;
  }
  lVar16 = uVar14 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar8 = *(ulonglong *)(lVar7 + 0x10) - uVar14;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar14) {
    lVar16 = 8;
    uVar8 = uVar19;
  }
  lVar15 = uVar20 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar14 = *(ulonglong *)(lVar7 + 0x30) - uVar20;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar20) {
    lVar15 = 8;
    uVar14 = uVar19;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar14) - uVar8;
  uVar4 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  pcVar9 = (char *)*in_R8;
  puVar10 = (undefined4 *)in_R8[2];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar7 = *in_stack_00000028;
  puVar11 = (undefined4 *)in_stack_00000028[2];
  lVar18 = lVar15;
  lVar17 = lVar16;
LAB_142283a95:
  do {
    if ((lVar17 == 0) || (lVar17 == uVar8 * 0x10 + lVar16)) goto LAB_142283ac0;
    lVar1 = lVar17 + 0x10;
    *puVar6 = *puVar6 + 1;
    pcVar2 = (char *)(lVar17 + 8);
    lVar17 = lVar1;
  } while (*pcVar2 != '\x02');
  goto LAB_142283ae2;
LAB_142283ac0:
  if ((lVar18 == 0) || (lVar18 == uVar14 * 0x10 + lVar15)) {
    if ((((*(int *)*in_RCX == 10) && (*(char *)(*in_RDX + 0x1fb) == '\0')) &&
        (*(longlong *)*in_stack_00000038 != 0)) && (*pcVar9 == '\0')) {
      hint_clutter_focus_edit::make_focus_or_camera_binding_hint();
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar7 = *in_stack_00000030;
      uVar13 = *(undefined8 *)(lVar7 + 0x40);
      uStack_e4 = (undefined4)local_134;
      uStack_e0 = (undefined4)((ulonglong)local_134 >> 0x20);
      lVar16 = *(longlong *)(lVar7 + 0x30);
      if (lVar16 == *(longlong *)(lVar7 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                  ();
      }
      lVar15 = *(longlong *)(lVar7 + 0x28);
      lVar18 = lVar16 * 0x58;
      *(undefined8 *)(lVar15 + 0x50 + lVar18) = uVar13;
      puVar3 = (undefined8 *)(lVar15 + 0x40 + lVar18);
      *puVar3 = local_f8;
      puVar3[1] = CONCAT26(local_ea,CONCAT42(local_ee,0x100));
      puVar3 = (undefined8 *)(lVar15 + 0x30 + lVar18);
      *puVar3 = local_108;
      puVar3[1] = uStack_100;
      puVar3 = (undefined8 *)(lVar15 + 0x20 + lVar18);
      *puVar3 = local_118;
      puVar3[1] = uStack_110;
      puVar3 = (undefined8 *)(lVar15 + 0x10 + lVar18);
      *puVar3 = local_128;
      puVar3[1] = uStack_120;
      *(undefined8 *)(lVar15 + lVar18) = CONCAT44(uStack_e4,6);
      ((undefined8 *)(lVar15 + lVar18))[1] = CONCAT44(local_12c,uStack_e0);
      *(longlong *)(lVar7 + 0x30) = lVar16 + 1;
      *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
    }
    return;
  }
  lVar1 = lVar18 + 0x10;
  *puVar6 = *puVar6 + 1;
  pcVar2 = (char *)(lVar18 + 8);
  lVar17 = 0;
  lVar18 = lVar1;
  lVar1 = 0;
  if (*pcVar2 == '\x02') {
LAB_142283ae2:
    lVar17 = lVar1;
    *puVar10 = uVar4;
    *pcVar9 = '\x01';
    *puVar11 = uVar5;
    lVar1 = *(longlong *)(lVar7 + 0x30);
    uVar13 = *(undefined8 *)(lVar7 + 0x40);
    if (lVar1 == *(longlong *)(lVar7 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar12 = *(longlong *)(lVar7 + 0x28);
    *(undefined8 *)(lVar12 + lVar1 * 0x10) = uVar13;
    *(undefined1 *)(lVar12 + 8 + lVar1 * 0x10) = 5;
    *(longlong *)(lVar7 + 0x30) = lVar1 + 1;
    *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
  }
  goto LAB_142283a95;
}

