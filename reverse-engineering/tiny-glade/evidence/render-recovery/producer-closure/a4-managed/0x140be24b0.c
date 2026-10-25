
ulonglong FUN_140be24b0(undefined8 *param_1)

{
  int *piVar1;
  byte bVar2;
  undefined1 uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 *unaff_RSI;
  byte *unaff_R13;
  
  uVar9 = param_1[1];
  if (uVar9 == 0) {
    FUN_1428d9430(&UNK_142b12c50,0x2a,&UNK_142b12c80);
code_r0x000140be25a3:
    FUN_1428cd210(unaff_R13);
    param_1 = unaff_RSI;
  }
  else {
    LOCK();
    piVar1 = (int *)(uVar9 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 != 0) goto code_r0x000140be2572;
    unaff_R13 = (byte *)*param_1;
    LOCK();
    bVar2 = *unaff_R13;
    if (bVar2 == 0) {
      *unaff_R13 = 1;
    }
    UNLOCK();
    unaff_RSI = param_1;
    if (bVar2 != 0) goto code_r0x000140be25a3;
  }
  FUN_140c81c70(unaff_R13);
  uVar6 = param_1[1];
  uVar5 = *(undefined4 *)(param_1 + 2);
  uVar3 = *(undefined1 *)((longlong)param_1 + 0x16);
  uVar4 = *(undefined2 *)((longlong)param_1 + 0x14);
  lVar7 = *(longlong *)(unaff_R13 + 0x18);
  if (lVar7 == *(longlong *)(unaff_R13 + 8)) {
    FUN_140562630(unaff_R13 + 8,&UNK_142b12c38);
  }
  lVar8 = *(longlong *)(unaff_R13 + 0x10);
  lVar10 = lVar7 * 0x10;
  *(undefined8 *)(lVar8 + lVar10) = uVar6;
  *(undefined4 *)(lVar8 + 8 + lVar10) = uVar5;
  *(undefined2 *)(lVar8 + 0xc + lVar10) = uVar4;
  *(undefined1 *)(lVar8 + 0xe + lVar10) = uVar3;
  *(longlong *)(unaff_R13 + 0x18) = lVar7 + 1;
  FUN_140c81e10();
  uVar9 = 1;
  LOCK();
  bVar2 = *unaff_R13;
  if (bVar2 == 1) {
    *unaff_R13 = 0;
  }
  else {
    uVar9 = (ulonglong)bVar2;
  }
  UNLOCK();
  if (bVar2 != 1) {
    uVar9 = FUN_1428cdb90(unaff_R13,0);
  }
code_r0x000140be2572:
  param_1[1] = 0;
  return uVar9;
}

