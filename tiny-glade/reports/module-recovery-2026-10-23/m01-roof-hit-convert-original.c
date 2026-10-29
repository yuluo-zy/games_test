
void __cdecl country_core::systems::collision::world::rapier_hit_to_raycast_world_result(void)

{
  float fVar1;
  float fVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined4 uVar10;
  ulonglong uVar11;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar12;
  int *in_R8;
  float *in_R9;
  ulonglong uVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  undefined8 *in_stack_00000028;
  float local_f4;
  undefined8 local_f0;
  
  fVar1 = (float)in_R8[2];
  fVar23 = *(float *)((longlong)in_RDX + 0x14) * fVar1 + *(float *)(in_RDX + 1);
  fVar21 = fVar1 * (float)*(undefined8 *)((longlong)in_RDX + 0xc) + (float)*in_RDX;
  fVar22 = fVar1 * (float)((ulonglong)*(undefined8 *)((longlong)in_RDX + 0xc) >> 0x20) +
           (float)((ulonglong)*in_RDX >> 0x20);
  lVar7 = in_stack_00000028[3];
  uVar8 = *(ulonglong *)(lVar7 + 0x28);
  if (uVar8 == 0) {
    iVar3 = in_R8[5];
    *(undefined8 *)((longlong)in_RCX + 0x24) = *(undefined8 *)(in_R8 + 3);
    *(int *)((longlong)in_RCX + 0x2c) = iVar3;
    uVar4 = in_stack_00000028[1];
    *in_RCX = *in_stack_00000028;
    in_RCX[1] = uVar4;
    in_RCX[2] = in_stack_00000028[2];
    in_RCX[3] = CONCAT44(fVar22,fVar21);
    *(float *)(in_RCX + 4) = fVar23;
    *(float *)(in_RCX + 6) = fVar1;
    *(undefined4 *)((longlong)in_RCX + 0x34) = 0;
    return;
  }
  if (*in_R8 != 2) {
    *(undefined4 *)((longlong)in_RCX + 0x34) = 2;
    return;
  }
  uVar11 = 0;
  if (uVar8 <= (uint)in_R8[1]) {
    uVar11 = uVar8;
  }
  uVar11 = (uint)in_R8[1] - uVar11;
  if (uVar8 <= uVar11) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar9 = *(longlong *)(lVar7 + 0x20);
  uVar12 = (ulonglong)*(uint *)(lVar9 + uVar11 * 0xc);
  uVar13 = (ulonglong)*(uint *)(lVar9 + 4 + uVar11 * 0xc);
  uVar11 = (ulonglong)*(uint *)(lVar9 + 8 + uVar11 * 0xc);
  fVar15 = in_R9[3];
  fVar2 = *in_R9;
  fVar24 = (float)*(undefined8 *)(in_R9 + 1);
  fVar25 = (float)((ulonglong)*(undefined8 *)(in_R9 + 1) >> 0x20);
  uVar8 = *(ulonglong *)(lVar7 + 0x18);
  if (uVar8 <= uVar12) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (uVar13 < uVar8) {
    if (uVar8 <= uVar11) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    ::utils::geometry::barycentric::barycentric();
    lVar9 = *(longlong *)(lVar7 + 0x40);
    fVar16 = (float)local_f0;
    fVar14 = (float)((ulonglong)local_f0 >> 0x20);
    if (lVar9 == 0) {
      fVar19 = (float)*(undefined8 *)(in_R8 + 3);
      fVar20 = (float)((ulonglong)*(undefined8 *)(in_R8 + 3) >> 0x20);
      fVar18 = (float)in_R8[5];
      lVar9 = *(longlong *)(lVar7 + 0x50);
    }
    else {
      uVar8 = *(ulonglong *)(lVar7 + 0x48);
      if (uVar8 <= uVar12) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar8 <= uVar13) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar8 <= uVar11) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar4 = *(undefined8 *)(lVar9 + uVar12 * 0xc);
      fVar18 = fVar14 * *(float *)(lVar9 + 8 + uVar11 * 0xc) +
               *(float *)(lVar9 + 8 + uVar12 * 0xc) * local_f4 +
               fVar16 * *(float *)(lVar9 + 8 + uVar13 * 0xc);
      uVar5 = *(undefined8 *)(lVar9 + uVar13 * 0xc);
      uVar6 = *(undefined8 *)(lVar9 + uVar11 * 0xc);
      fVar19 = (float)uVar6 * fVar14 + (float)uVar5 * fVar16 + local_f4 * (float)uVar4;
      fVar20 = (float)((ulonglong)uVar6 >> 0x20) * fVar14 +
               (float)((ulonglong)uVar5 >> 0x20) * fVar16 +
               local_f4 * (float)((ulonglong)uVar4 >> 0x20);
      fVar26 = fVar18 * fVar24 - fVar20 * fVar25;
      fVar27 = fVar19 * fVar25 - fVar18 * fVar2;
      fVar17 = fVar20 * fVar2 - fVar24 * fVar19;
      fVar26 = fVar26 + fVar26;
      fVar27 = fVar27 + fVar27;
      fVar17 = fVar17 + fVar17;
      fVar19 = fVar15 * fVar26 + (fVar17 * fVar24 - fVar27 * fVar25) + fVar19;
      fVar20 = fVar15 * fVar27 + (fVar26 * fVar25 - fVar17 * fVar2) + fVar20;
      fVar18 = (fVar27 * fVar2 - fVar24 * fVar26) + fVar15 * fVar17 + fVar18;
      lVar9 = *(longlong *)(lVar7 + 0x50);
    }
    if (lVar9 == 0) {
      uVar10 = 0;
      fVar15 = fVar14;
    }
    else {
      uVar8 = *(ulonglong *)(lVar7 + 0x58);
      if (uVar8 <= uVar12) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar8 <= uVar13) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar8 <= uVar11) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar4 = *(undefined8 *)(lVar9 + uVar12 * 8);
      uVar5 = *(undefined8 *)(lVar9 + uVar13 * 8);
      uVar6 = *(undefined8 *)(lVar9 + uVar11 * 8);
      fVar15 = fVar14 * (float)((ulonglong)uVar6 >> 0x20);
      fVar14 = fVar14 * (float)uVar6 + fVar16 * (float)uVar5 + local_f4 * (float)uVar4;
      uVar10 = 1;
      fVar15 = fVar15 + fVar16 * (float)((ulonglong)uVar5 >> 0x20) +
                        local_f4 * (float)((ulonglong)uVar4 >> 0x20);
    }
    *(ulonglong *)((longlong)in_RCX + 0x24) = CONCAT44(fVar20,fVar19);
    *(float *)((longlong)in_RCX + 0x2c) = fVar18;
    uVar4 = in_stack_00000028[1];
    *in_RCX = *in_stack_00000028;
    in_RCX[1] = uVar4;
    in_RCX[2] = in_stack_00000028[2];
    in_RCX[3] = CONCAT44(fVar22,fVar21);
    *(float *)(in_RCX + 4) = fVar23;
    *(float *)(in_RCX + 6) = fVar1;
    *(undefined4 *)((longlong)in_RCX + 0x34) = uVar10;
    in_RCX[7] = CONCAT44(fVar15,fVar14);
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}


