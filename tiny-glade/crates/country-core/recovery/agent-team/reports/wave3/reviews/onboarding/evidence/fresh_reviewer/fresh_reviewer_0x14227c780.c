
void __cdecl system_onboarding::hint_camera_controls::hint_camera_enable_condition(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  int *piVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined8 *in_R8;
  longlong *in_R9;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  bool bVar14;
  bool bVar15;
  float fVar16;
  float fVar17;
  
  uVar1 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  puVar2 = (undefined4 *)in_RCX[2];
  *puVar2 = uVar1;
  piVar3 = (int *)*in_RCX;
  if (*piVar3 != 0) {
    return;
  }
  puVar4 = (ulonglong *)*in_RDX;
  lVar11 = in_RDX[1];
  uVar9 = *puVar4;
  uVar10 = 0;
  uVar7 = uVar9 - *(ulonglong *)(lVar11 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar11 + 0x18)) {
    uVar7 = uVar10;
  }
  uVar12 = uVar9 - *(ulonglong *)(lVar11 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar11 + 0x38)) {
    uVar12 = uVar10;
  }
  uVar9 = *(ulonglong *)(lVar11 + 0x30);
  uVar8 = uVar9 - uVar12;
  if (uVar12 > uVar9) {
    uVar8 = uVar10;
  }
  uVar5 = *(ulonglong *)(lVar11 + 0x10);
  uVar13 = uVar5 - uVar7;
  if (uVar7 > uVar5) {
    uVar13 = uVar10;
  }
  uVar10 = *(ulonglong *)(lVar11 + 0x40);
  *puVar4 = uVar10 - (uVar13 + uVar8);
  if (uVar7 <= uVar5 && uVar5 - uVar7 != 0 || uVar12 <= uVar9 && uVar9 - uVar12 != 0) {
    *puVar4 = uVar10;
    piVar3[2] = 0;
    lVar11 = *in_R9;
  }
  else {
    lVar11 = *in_R9;
    fVar16 = *(float *)(lVar11 + 0x28) + (float)piVar3[2];
    piVar3[2] = (int)fVar16;
    if (10.0 <= fVar16) goto LAB_14227c834;
  }
  piVar3[1] = (int)(*(float *)(lVar11 + 0x28) + (float)piVar3[1]);
LAB_14227c834:
  lVar6 = in_R8[1];
  uVar9 = *(ulonglong *)*in_R8;
  uVar10 = 0;
  uVar7 = uVar9 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar7 = uVar10;
  }
  uVar12 = uVar9 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar12 = uVar10;
  }
  bVar14 = *(ulonglong *)(lVar6 + 0x30) < uVar12;
  uVar12 = *(ulonglong *)(lVar6 + 0x30) - uVar12;
  uVar9 = uVar12;
  if (bVar14) {
    uVar9 = uVar10;
  }
  bVar15 = *(ulonglong *)(lVar6 + 0x10) < uVar7;
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar7;
  uVar8 = uVar7;
  if (bVar15) {
    uVar8 = uVar10;
  }
  uVar9 = uVar8 + uVar9;
  if (!bVar15 && uVar7 != 0 || !bVar14 && uVar12 != 0) {
    uVar9 = uVar10;
  }
  *(ulonglong *)*in_R8 = *(longlong *)(lVar6 + 0x40) - uVar9;
  if (!bVar15 && uVar7 != 0 || !bVar14 && uVar12 != 0) {
    piVar3[4] = 0;
    fVar16 = *(float *)(lVar11 + 0x28) + (float)piVar3[3];
    piVar3[3] = (int)fVar16;
    bVar14 = false;
    fVar17 = (float)piVar3[1];
  }
  else {
    fVar17 = *(float *)(lVar11 + 0x28) + (float)piVar3[4];
    piVar3[4] = (int)fVar17;
    fVar16 = (float)piVar3[3];
    bVar14 = 1.5 < fVar17;
    fVar17 = (float)piVar3[1];
  }
  if (0.1 < fVar16) {
    if ((bool)(~bVar14 & fVar17 <= 100.0)) {
      return;
    }
  }
  else if (fVar17 <= 100.0) {
    return;
  }
  *puVar2 = uVar1;
  *piVar3 = 1;
  return;
}

