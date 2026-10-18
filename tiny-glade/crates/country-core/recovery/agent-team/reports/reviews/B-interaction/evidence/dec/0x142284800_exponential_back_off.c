
void __cdecl system_onboarding::exponential_back_off::exponential_back_off(void)

{
  char cVar1;
  char cVar2;
  longlong lVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  int *piVar11;
  longlong *in_RCX;
  ulonglong uVar12;
  ulonglong uVar13;
  float *in_RDX;
  ulonglong uVar14;
  longlong lVar15;
  undefined4 *puVar16;
  undefined4 uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  longlong *in_R9;
  int *piVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  longlong lVar24;
  bool bVar25;
  bool bVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  
  lVar3 = *in_R9;
  fVar30 = *(float *)(lVar3 + 0x28) + *in_RDX;
  *in_RDX = fVar30;
  puVar4 = (ulonglong *)*in_R8;
  lVar24 = in_R8[1];
  uVar5 = *puVar4;
  uVar20 = 0;
  uVar14 = uVar5 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar14 = uVar20;
  }
  uVar9 = uVar5 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar9 = uVar20;
  }
  uVar5 = *(ulonglong *)(lVar24 + 0x30);
  uVar18 = uVar5 - uVar9;
  if (uVar9 > uVar5) {
    uVar18 = uVar20;
  }
  uVar13 = *(ulonglong *)(lVar24 + 0x10);
  uVar12 = uVar13 - uVar14;
  if (uVar14 > uVar13) {
    uVar12 = uVar20;
  }
  uVar20 = *(ulonglong *)(lVar24 + 0x40);
  *puVar4 = uVar20 - (uVar12 + uVar18);
  if (uVar14 <= uVar13 && uVar13 - uVar14 != 0 || uVar9 <= uVar5 && uVar5 - uVar9 != 0) {
    *puVar4 = uVar20;
    *in_RDX = 0.0;
    fVar30 = 0.0;
LAB_1422848a7:
    fVar29 = *(float *)(lVar3 + 0x28);
    uVar17 = *(undefined4 *)((longlong)in_RCX + 0x1c);
    puVar16 = (undefined4 *)in_RCX[2];
    *puVar16 = uVar17;
    lVar24 = *in_RCX;
    *(float *)(lVar24 + 0x1f0) = fVar29 + *(float *)(lVar24 + 0x1f0);
  }
  else {
    if (fVar30 < 30.0) goto LAB_1422848a7;
    lVar24 = *in_RCX;
    puVar16 = (undefined4 *)in_RCX[2];
    uVar17 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  }
  cVar1 = *(char *)(lVar24 + 500);
  cVar2 = *(char *)(lVar24 + 0x1f5);
  *puVar16 = uVar17;
  fVar29 = *(float *)(lVar24 + 0x1e4);
  if ((fVar29 == 0.0) && (!NAN(fVar29))) {
    *(undefined4 *)(lVar24 + 0x1e4) = 0x44340000;
    fVar29 = 720.0;
  }
  puVar4 = (ulonglong *)*in_stack_00000040;
  lVar6 = in_stack_00000040[1];
  uVar5 = *puVar4;
  uVar20 = 0;
  uVar14 = uVar5 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar14 = uVar20;
  }
  uVar9 = uVar5 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar9 = uVar20;
  }
  uVar5 = *(ulonglong *)(lVar6 + 0x30);
  uVar18 = uVar5 - uVar9;
  if (uVar5 < uVar9) {
    uVar18 = uVar20;
  }
  uVar13 = *(ulonglong *)(lVar6 + 0x10);
  uVar12 = uVar13 - uVar14;
  if (uVar13 < uVar14) {
    uVar12 = uVar20;
  }
  uVar20 = *(ulonglong *)(lVar6 + 0x40);
  *puVar4 = uVar20 - (uVar12 + uVar18);
  if (uVar13 >= uVar14 && uVar13 - uVar14 != 0 || uVar5 >= uVar9 && uVar5 - uVar9 != 0) {
    *puVar4 = uVar20;
    *(undefined4 *)(lVar24 + 0x1e0) = 0;
    fVar27 = 720.0;
    if (720.0 <= fVar29 * 1.5) {
      fVar27 = fVar29 * 1.5;
    }
    fVar29 = 10800.0;
    if (fVar27 <= 10800.0) {
      fVar29 = fVar27;
    }
    *(float *)(lVar24 + 0x1e4) = fVar29;
    *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    lVar6 = *in_stack_00000038;
    lVar15 = *(longlong *)(lVar6 + 0x30);
    uVar7 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar15 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar6 + 0x28) + lVar15 * 8) = uVar7;
    *(longlong *)(lVar6 + 0x30) = lVar15 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  }
  if (cVar1 != '\0') {
    fVar27 = *(float *)(lVar24 + 0x1e0);
    if (fVar30 < 10.0) {
      fVar27 = fVar27 + *(float *)(lVar3 + 0x28);
      *(float *)(lVar24 + 0x1e0) = fVar27;
      if (fVar29 < 0.0) goto LAB_142284cfc;
      fVar28 = 0.0;
      if (0.0 <= fVar27) {
        fVar28 = fVar27;
      }
      fVar27 = fVar29;
      if (fVar28 <= fVar29) {
        fVar27 = fVar28;
      }
      *(float *)(lVar24 + 0x1e0) = fVar27;
    }
    if (fVar29 <= fVar27) {
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar6 = *in_stack_00000038;
      lVar15 = *(longlong *)(lVar6 + 0x30);
      uVar7 = *(undefined8 *)(lVar6 + 0x40);
      if (lVar15 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar6 + 0x28) + lVar15 * 8) = uVar7;
      *(longlong *)(lVar6 + 0x30) = lVar15 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      puVar8 = (undefined8 *)*in_stack_00000030;
      *puVar8 = 0;
      *(undefined2 *)(puVar8 + 1) = 0;
      *(undefined1 *)(lVar24 + 500) = 0;
    }
  }
  *puVar16 = uVar17;
  fVar29 = *(float *)(lVar24 + 0x1ec);
  if ((fVar29 == 0.0) && (!NAN(fVar29))) {
    *(undefined4 *)(lVar24 + 0x1ec) = 0x44340000;
    fVar29 = 720.0;
  }
  lVar6 = in_stack_00000048[1];
  uVar5 = *(ulonglong *)*in_stack_00000048;
  uVar20 = 0;
  uVar14 = uVar5 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar14 = uVar20;
  }
  uVar9 = uVar5 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar9 = uVar20;
  }
  uVar5 = *(ulonglong *)(lVar6 + 0x10);
  uVar18 = *(ulonglong *)(lVar6 + 0x30);
  bVar25 = uVar18 < uVar9;
  uVar12 = uVar18 - uVar9;
  uVar13 = uVar12;
  if (bVar25) {
    uVar13 = uVar20;
  }
  bVar26 = uVar5 < uVar14;
  uVar22 = uVar5 - uVar14;
  uVar23 = uVar22;
  if (bVar26) {
    uVar23 = uVar20;
  }
  lVar15 = *(longlong *)(lVar6 + 8);
  uVar19 = uVar13 + uVar23;
  if (!bVar26 && uVar22 != 0 || !bVar25 && uVar12 != 0) {
    uVar19 = uVar20;
  }
  lVar10 = *(longlong *)(lVar6 + 0x28);
  *(ulonglong *)*in_stack_00000048 = *(longlong *)(lVar6 + 0x40) - uVar19;
  if (!bVar26 && uVar22 != 0 || !bVar25 && uVar12 != 0) {
    lVar15 = lVar15 + uVar14 * 0x28;
    if (uVar5 < uVar14) {
      lVar15 = 8;
    }
    piVar21 = (int *)0x0;
    if (uVar14 < uVar5) {
      piVar21 = (int *)(lVar15 + -0x28 + uVar23 * 0x28);
    }
    lVar10 = lVar10 + uVar9 * 0x28;
    if (uVar18 < uVar9) {
      lVar10 = 8;
    }
    piVar11 = (int *)(lVar10 + -0x28 + uVar13 * 0x28);
    if (uVar18 <= uVar9) {
      piVar11 = piVar21;
    }
    if (*piVar11 - 1U < 2) {
      *(undefined4 *)(lVar24 + 0x1e8) = 0;
      fVar27 = 720.0;
      if (720.0 <= fVar29 * 1.5) {
        fVar27 = fVar29 * 1.5;
      }
      fVar29 = 10800.0;
      if (fVar27 <= 10800.0) {
        fVar29 = fVar27;
      }
      *(float *)(lVar24 + 0x1ec) = fVar29;
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar6 = *in_stack_00000038;
      lVar15 = *(longlong *)(lVar6 + 0x30);
      uVar7 = *(undefined8 *)(lVar6 + 0x40);
      if (lVar15 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar6 + 0x28) + lVar15 * 8) = uVar7;
      *(longlong *)(lVar6 + 0x30) = lVar15 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
    }
  }
  if (cVar2 != '\0') {
    fVar27 = *(float *)(lVar24 + 0x1e8);
    if (fVar30 < 10.0) {
      fVar27 = fVar27 + *(float *)(lVar3 + 0x28);
      *(float *)(lVar24 + 0x1e8) = fVar27;
      if (fVar29 < 0.0) {
LAB_142284cfc:
                    // WARNING: Subroutine does not return
        core::f32::impl$0::clamp::do_panic::runtime();
      }
      fVar30 = 0.0;
      if (0.0 <= fVar27) {
        fVar30 = fVar27;
      }
      fVar27 = fVar29;
      if (fVar30 <= fVar29) {
        fVar27 = fVar30;
      }
      *(float *)(lVar24 + 0x1e8) = fVar27;
    }
    if (fVar29 <= fVar27) {
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar3 = *in_stack_00000038;
      lVar6 = *(longlong *)(lVar3 + 0x30);
      uVar7 = *(undefined8 *)(lVar3 + 0x40);
      if (lVar6 == *(longlong *)(lVar3 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar3 + 0x28) + lVar6 * 8) = uVar7;
      *(longlong *)(lVar3 + 0x30) = lVar6 + 1;
      *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      puVar8 = (undefined8 *)*in_stack_00000028;
      *puVar8 = 0;
      *(undefined2 *)(puVar8 + 1) = 0;
      *puVar16 = uVar17;
      *(undefined1 *)(lVar24 + 0x1f5) = 0;
    }
  }
  return;
}

