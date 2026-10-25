// 0x142283cc0 sig void __cdecl hint_terrain_layering(void)

void __cdecl system_onboarding::hint_terrain::hint_terrain_layering(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined8 uVar12;
  longlong lVar13;
  undefined8 *in_RCX;
  ulonglong uVar14;
  longlong lVar15;
  char cVar16;
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
  undefined8 local_144;
  undefined4 local_13c;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined4 local_fe;
  undefined2 local_fa;
  undefined2 uStack_f4;
  undefined2 uStack_f2;
  undefined4 uStack_f0;
  
  puVar5 = (ulonglong *)*in_R9;
  lVar6 = in_R9[1];
  uVar7 = *puVar5;
  uVar17 = 0;
  uVar14 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar14 = uVar17;
  }
  uVar19 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar19 = uVar17;
  }
  lVar13 = uVar14 * 0x10 + *(longlong *)(lVar6 + 8);
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar14;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar14) {
    lVar13 = 8;
    uVar7 = uVar17;
  }
  lVar15 = uVar19 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar14 = *(ulonglong *)(lVar6 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar19) {
    lVar15 = 8;
    uVar14 = uVar17;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar14) - uVar7;
  lVar18 = uVar7 * 0x10 + lVar13;
  lVar20 = uVar14 * 0x10 + lVar15;
  puVar8 = (ulonglong *)*in_R8;
  puVar9 = (undefined4 *)in_R8[2];
  uVar3 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  uVar4 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar6 = *in_stack_00000028;
  puVar10 = (undefined4 *)in_stack_00000028[2];
  do {
    while ((lVar13 == 0 || (lVar13 == lVar18))) {
      if ((lVar15 == 0) || (lVar15 == lVar20)) {
        if ((((*(int *)*in_RCX == 10) && (*(char *)(*in_RDX + 0x1f9) == '\0')) && (1 < *puVar8)) &&
           ((*(longlong *)*in_stack_00000038 != 0 && ((char)puVar8[1] == '\0')))) {
          country_core::resources::steam::impl$31::from();
          hint_orchestrator::BindingHintDisplay::
          alternate<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
          *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
          lVar6 = *in_stack_00000030;
          uVar12 = *(undefined8 *)(lVar6 + 0x40);
          uStack_f4 = (undefined2)local_144;
          uStack_f2 = (undefined2)((ulonglong)local_144 >> 0x10);
          uStack_f0 = (undefined4)((ulonglong)local_144 >> 0x20);
          lVar13 = *(longlong *)(lVar6 + 0x30);
          if (lVar13 == *(longlong *)(lVar6 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                      ();
          }
          lVar15 = *(longlong *)(lVar6 + 0x28);
          lVar18 = lVar13 * 0x58;
          *(undefined8 *)(lVar15 + 0x50 + lVar18) = uVar12;
          puVar2 = (undefined8 *)(lVar15 + 0x40 + lVar18);
          *puVar2 = local_108;
          puVar2[1] = CONCAT26(local_fa,CONCAT42(local_fe,0x600));
          puVar2 = (undefined8 *)(lVar15 + 0x30 + lVar18);
          *puVar2 = local_118;
          puVar2[1] = uStack_110;
          puVar2 = (undefined8 *)(lVar15 + 0x20 + lVar18);
          *puVar2 = local_128;
          puVar2[1] = uStack_120;
          puVar2 = (undefined8 *)(lVar15 + 0x10 + lVar18);
          *puVar2 = local_138;
          puVar2[1] = uStack_130;
          *(undefined8 *)(lVar15 + lVar18) = CONCAT26(uStack_f2,CONCAT24(uStack_f4,6));
          ((undefined8 *)(lVar15 + lVar18))[1] = CONCAT44(local_13c,uStack_f0);
          *(longlong *)(lVar6 + 0x30) = lVar13 + 1;
          *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
        }
        return;
      }
      lVar1 = lVar15 + 0x10;
      lVar13 = 0;
      *puVar5 = *puVar5 + 1;
      cVar16 = *(char *)(lVar15 + 8);
      lVar15 = lVar1;
      if (cVar16 != '\0') goto LAB_142283e0c;
LAB_142283e92:
      *puVar9 = uVar3;
      *puVar8 = *puVar8 + 1;
    }
    lVar1 = lVar13 + 0x10;
    *puVar5 = *puVar5 + 1;
    cVar16 = *(char *)(lVar13 + 8);
    lVar13 = lVar1;
    if (cVar16 == '\0') goto LAB_142283e92;
LAB_142283e0c:
    if (cVar16 == '\x01') {
      if ((char)puVar8[1] == '\0') {
        *puVar10 = uVar4;
        lVar1 = *(longlong *)(lVar6 + 0x30);
        uVar12 = *(undefined8 *)(lVar6 + 0x40);
        if (lVar1 == *(longlong *)(lVar6 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar11 = *(longlong *)(lVar6 + 0x28);
        *(undefined8 *)(lVar11 + lVar1 * 0x10) = uVar12;
        *(undefined1 *)(lVar11 + 8 + lVar1 * 0x10) = 6;
        *(longlong *)(lVar6 + 0x30) = lVar1 + 1;
        *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
      }
      *puVar9 = uVar3;
      *(undefined1 *)(puVar8 + 1) = 1;
    }
  } while( true );
}

