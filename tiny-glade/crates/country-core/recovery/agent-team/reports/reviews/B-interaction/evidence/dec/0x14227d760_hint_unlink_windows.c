
void __cdecl system_onboarding::hint_unlink_windows::hint_unlink_windows(void)

{
  longlong lVar1;
  char *pcVar2;
  undefined8 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  undefined4 *puVar9;
  longlong lVar10;
  undefined4 *puVar11;
  longlong lVar12;
  undefined8 uVar13;
  longlong lVar14;
  undefined8 *in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong *in_RDX;
  longlong lVar18;
  longlong *in_R8;
  longlong *in_R9;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong lVar21;
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
  
  if (*(char *)(*in_R8 + 0x1fd) != '\0') {
    return;
  }
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar8 = *puVar6;
  uVar19 = 0;
  uVar15 = uVar8 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar15 = uVar19;
  }
  uVar20 = uVar8 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar8 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar20 = uVar19;
  }
  lVar14 = uVar15 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar8 = *(ulonglong *)(lVar7 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar15) {
    lVar14 = 8;
    uVar8 = uVar19;
  }
  lVar16 = uVar20 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar15 = *(ulonglong *)(lVar7 + 0x30) - uVar20;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar20) {
    lVar16 = 8;
    uVar15 = uVar19;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar15) - uVar8;
  lVar18 = uVar8 * 0x10 + lVar14;
  lVar21 = uVar15 * 0x10 + lVar16;
  uVar4 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar7 = *in_RDX;
  puVar9 = (undefined4 *)in_RDX[2];
  uVar5 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar10 = *in_R9;
  puVar11 = (undefined4 *)in_R9[2];
  do {
    while ((lVar14 != 0 && (lVar14 != lVar18))) {
      lVar1 = lVar14 + 0x10;
      *puVar6 = *puVar6 + 1;
      pcVar2 = (char *)(lVar14 + 8);
      lVar14 = lVar1;
      if (*pcVar2 == '\0') goto LAB_14227d916;
LAB_14227d8c1:
      *puVar11 = uVar5;
      lVar1 = *(longlong *)(lVar10 + 0x30);
      uVar13 = *(undefined8 *)(lVar10 + 0x40);
      if (lVar1 == *(longlong *)(lVar10 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar12 = *(longlong *)(lVar10 + 0x28);
      *(undefined8 *)(lVar12 + lVar1 * 0x10) = uVar13;
      *(undefined1 *)(lVar12 + 8 + lVar1 * 0x10) = 8;
      *(longlong *)(lVar10 + 0x30) = lVar1 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
    }
    if (lVar16 == 0) {
      return;
    }
    if (lVar16 == lVar21) {
      return;
    }
    lVar1 = lVar16 + 0x10;
    lVar14 = 0;
    *puVar6 = *puVar6 + 1;
    pcVar2 = (char *)(lVar16 + 8);
    lVar16 = lVar1;
    if (*pcVar2 != '\0') goto LAB_14227d8c1;
LAB_14227d916:
    country_core::resources::steam::impl$31::from();
    hint_orchestrator::BindingHintDisplay::
    alternate<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
    *puVar9 = uVar4;
    uVar13 = *(undefined8 *)(lVar7 + 0x40);
    uStack_f4 = (undefined2)local_144;
    uStack_f2 = (undefined2)((ulonglong)local_144 >> 0x10);
    uStack_f0 = (undefined4)((ulonglong)local_144 >> 0x20);
    lVar1 = *(longlong *)(lVar7 + 0x30);
    if (lVar1 == *(longlong *)(lVar7 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar12 = *(longlong *)(lVar7 + 0x28);
    lVar17 = lVar1 * 0x58;
    *(undefined8 *)(lVar12 + 0x50 + lVar17) = uVar13;
    puVar3 = (undefined8 *)(lVar12 + 0x40 + lVar17);
    *puVar3 = local_108;
    puVar3[1] = CONCAT26(local_fa,CONCAT42(local_fe,0x800));
    puVar3 = (undefined8 *)(lVar12 + 0x30 + lVar17);
    *puVar3 = local_118;
    puVar3[1] = uStack_110;
    puVar3 = (undefined8 *)(lVar12 + 0x20 + lVar17);
    *puVar3 = local_128;
    puVar3[1] = uStack_120;
    puVar3 = (undefined8 *)(lVar12 + 0x10 + lVar17);
    *puVar3 = local_138;
    puVar3[1] = uStack_130;
    *(undefined8 *)(lVar12 + lVar17) = CONCAT26(uStack_f2,CONCAT24(uStack_f4,6));
    ((undefined8 *)(lVar12 + lVar17))[1] = CONCAT44(local_13c,uStack_f0);
    *(longlong *)(lVar7 + 0x30) = lVar1 + 1;
    *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
  } while( true );
}

