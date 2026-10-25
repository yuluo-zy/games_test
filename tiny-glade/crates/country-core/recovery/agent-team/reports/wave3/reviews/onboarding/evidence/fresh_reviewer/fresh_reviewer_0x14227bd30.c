
void __cdecl system_onboarding::hint_camera_progress::hint_camera_progress_update(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  float *pfVar3;
  undefined4 *puVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined4 *puVar9;
  longlong lVar10;
  undefined8 uVar11;
  longlong lVar12;
  int *piVar13;
  float *pfVar14;
  undefined8 *in_RCX;
  ulonglong uVar15;
  int *piVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  longlong *in_R8;
  longlong *in_R9;
  ulonglong uVar21;
  int *piVar22;
  float fVar23;
  
  pfVar3 = (float *)*in_RCX;
  puVar4 = (undefined4 *)in_RCX[2];
  fVar23 = *pfVar3;
  uVar1 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  *puVar4 = uVar1;
  pfVar3[3] = fVar23;
  fVar23 = pfVar3[1];
  *puVar4 = uVar1;
  pfVar3[4] = fVar23;
  fVar23 = pfVar3[2];
  *puVar4 = uVar1;
  pfVar3[5] = fVar23;
  puVar5 = (ulonglong *)*in_RDX;
  lVar6 = in_RDX[1];
  uVar7 = *puVar5;
  uVar15 = 0;
  uVar21 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar21 = uVar15;
  }
  uVar17 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar17 = uVar15;
  }
  piVar22 = (int *)(uVar21 * 0x10 + *(longlong *)(lVar6 + 8));
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar21;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar21) {
    piVar22 = (int *)&DAT_00000008;
    uVar7 = uVar15;
  }
  piVar18 = (int *)(uVar17 * 0x10 + *(longlong *)(lVar6 + 0x28));
  uVar21 = *(ulonglong *)(lVar6 + 0x30) - uVar17;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar17) {
    piVar18 = (int *)&DAT_00000008;
    uVar21 = uVar15;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar21) - uVar7;
  lVar6 = *in_R9;
  uVar2 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar8 = *in_R8;
  puVar9 = (undefined4 *)in_R8[2];
  piVar19 = piVar18;
  piVar13 = piVar22;
  do {
    if ((piVar13 == (int *)0x0) || (piVar13 == piVar22 + uVar7 * 4)) {
      if ((piVar19 == (int *)0x0) || (piVar19 == piVar18 + uVar21 * 4)) {
        return;
      }
      piVar20 = piVar19 + 4;
      piVar16 = (int *)0x0;
      piVar13 = piVar19;
    }
    else {
      piVar16 = piVar13 + 4;
      piVar20 = piVar19;
    }
    *puVar5 = *puVar5 + 1;
    if (*piVar13 == 2) {
      fVar23 = *(float *)(lVar6 + 0x28) / pfVar3[7];
      pfVar14 = pfVar3 + 1;
    }
    else if (*piVar13 == 1) {
      fVar23 = (float)piVar13[1] / pfVar3[8];
      pfVar14 = pfVar3 + 2;
    }
    else {
      fVar23 = *(float *)(lVar6 + 0x28) / pfVar3[6];
      pfVar14 = pfVar3;
    }
    *puVar4 = uVar1;
    *pfVar14 = fVar23 + *pfVar14;
    if ((pfVar3[3] <= 1.0 && pfVar3[3] != 1.0) && (1.0 <= *pfVar3)) {
      *puVar9 = uVar2;
      lVar10 = *(longlong *)(lVar8 + 0x30);
      uVar11 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar10 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar12 = *(longlong *)(lVar8 + 0x28);
      *(undefined8 *)(lVar12 + lVar10 * 0x10) = uVar11;
      *(undefined1 *)(lVar12 + 8 + lVar10 * 0x10) = 10;
      *(longlong *)(lVar8 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
    if ((pfVar3[4] <= 1.0 && pfVar3[4] != 1.0) && (1.0 <= pfVar3[1])) {
      *puVar9 = uVar2;
      lVar10 = *(longlong *)(lVar8 + 0x30);
      uVar11 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar10 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar12 = *(longlong *)(lVar8 + 0x28);
      *(undefined8 *)(lVar12 + lVar10 * 0x10) = uVar11;
      *(undefined1 *)(lVar12 + 8 + lVar10 * 0x10) = 0xb;
      *(longlong *)(lVar8 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
    piVar19 = piVar20;
    piVar13 = piVar16;
    if ((pfVar3[5] <= 1.0 && pfVar3[5] != 1.0) && (1.0 <= pfVar3[2])) {
      *puVar9 = uVar2;
      lVar10 = *(longlong *)(lVar8 + 0x30);
      uVar11 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar10 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar12 = *(longlong *)(lVar8 + 0x28);
      *(undefined8 *)(lVar12 + lVar10 * 0x10) = uVar11;
      *(undefined1 *)(lVar12 + 8 + lVar10 * 0x10) = 0xc;
      *(longlong *)(lVar8 + 0x30) = lVar10 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
  } while( true );
}

