// 0x142283640 sig void __cdecl hint_terrain_steepness(void)

void __cdecl system_onboarding::hint_terrain_steepness::hint_terrain_steepness(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  char *pcVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined8 uVar12;
  longlong lVar13;
  longlong lVar14;
  undefined8 *in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  longlong *in_RDX;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong uVar17;
  longlong lVar18;
  ulonglong uVar19;
  longlong lVar20;
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
  uVar17 = 0;
  uVar15 = uVar8 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar15 = uVar17;
  }
  uVar19 = uVar8 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar19 = uVar17;
  }
  lVar14 = uVar15 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar8 = *(ulonglong *)(lVar7 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar15) {
    lVar14 = 8;
    uVar8 = uVar17;
  }
  lVar16 = uVar19 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar15 = *(ulonglong *)(lVar7 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar19) {
    lVar16 = 8;
    uVar15 = uVar17;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar15) - uVar8;
  lVar18 = uVar8 * 0x10 + lVar14;
  lVar20 = uVar15 * 0x10 + lVar16;
  uVar4 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  pcVar9 = (char *)*in_R8;
  puVar10 = (undefined4 *)in_R8[2];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar7 = *in_stack_00000028;
  puVar11 = (undefined4 *)in_stack_00000028[2];
  do {
    while ((lVar14 == 0 || (lVar14 == lVar18))) {
      if ((lVar16 == 0) || (lVar16 == lVar20)) {
        if ((((*(int *)*in_RCX == 10) && (*(char *)(*in_RDX + 0x1fa) == '\0')) &&
            (*pcVar9 == '\x01')) && ((*(longlong *)*in_stack_00000038 != 0 && (pcVar9[1] == '\0'))))
        {
          hint_roof_shape::make_lbm_drag_binding_hint();
          *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
          lVar7 = *in_stack_00000030;
          uVar12 = *(undefined8 *)(lVar7 + 0x40);
          uStack_e4 = (undefined4)local_134;
          uStack_e0 = (undefined4)((ulonglong)local_134 >> 0x20);
          lVar14 = *(longlong *)(lVar7 + 0x30);
          if (lVar14 == *(longlong *)(lVar7 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                      ();
          }
          lVar16 = *(longlong *)(lVar7 + 0x28);
          lVar18 = lVar14 * 0x58;
          *(undefined8 *)(lVar16 + 0x50 + lVar18) = uVar12;
          puVar2 = (undefined8 *)(lVar16 + 0x40 + lVar18);
          *puVar2 = local_f8;
          puVar2[1] = CONCAT26(local_ea,CONCAT42(local_ee,0x400));
          puVar2 = (undefined8 *)(lVar16 + 0x30 + lVar18);
          *puVar2 = local_108;
          puVar2[1] = uStack_100;
          puVar2 = (undefined8 *)(lVar16 + 0x20 + lVar18);
          *puVar2 = local_118;
          puVar2[1] = uStack_110;
          puVar2 = (undefined8 *)(lVar16 + 0x10 + lVar18);
          *puVar2 = local_128;
          puVar2[1] = uStack_120;
          *(undefined8 *)(lVar16 + lVar18) = CONCAT44(uStack_e4,6);
          ((undefined8 *)(lVar16 + lVar18))[1] = CONCAT44(local_12c,uStack_e0);
          *(longlong *)(lVar7 + 0x30) = lVar14 + 1;
          *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
        }
        return;
      }
      lVar1 = lVar16 + 0x10;
      lVar14 = 0;
      *puVar6 = *puVar6 + 1;
      cVar3 = *(char *)(lVar16 + 8);
      *puVar10 = uVar4;
      lVar16 = lVar1;
      if (cVar3 != '\0') goto LAB_142283788;
LAB_142283730:
      *pcVar9 = '\x01';
    }
    lVar1 = lVar14 + 0x10;
    *puVar6 = *puVar6 + 1;
    cVar3 = *(char *)(lVar14 + 8);
    *puVar10 = uVar4;
    lVar14 = lVar1;
    if (cVar3 == '\0') goto LAB_142283730;
LAB_142283788:
    pcVar9[1] = '\x01';
    *puVar11 = uVar5;
    lVar1 = *(longlong *)(lVar7 + 0x30);
    uVar12 = *(undefined8 *)(lVar7 + 0x40);
    if (lVar1 == *(longlong *)(lVar7 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar13 = *(longlong *)(lVar7 + 0x28);
    *(undefined8 *)(lVar13 + lVar1 * 0x10) = uVar12;
    *(undefined1 *)(lVar13 + 8 + lVar1 * 0x10) = 4;
    *(longlong *)(lVar7 + 0x30) = lVar1 + 1;
    *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
  } while( true );
}

