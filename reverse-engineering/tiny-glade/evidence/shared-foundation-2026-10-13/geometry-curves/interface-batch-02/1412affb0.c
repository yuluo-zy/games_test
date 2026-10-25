
void FUN_1412affb0(undefined8 *param_1,ulonglong *param_2)

{
  char cVar1;
  undefined8 *puVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 in_XMM1_Da;
  
  puVar2 = (undefined8 *)param_2[3];
  if ((*param_2 & 0xfffffffffffffffd) == 0) {
    if ((puVar2 != (undefined8 *)0x0) && (*(char *)(puVar2 + 9) == '\0')) {
      uVar3 = puVar2[3];
      uVar4 = puVar2[4];
      puVar2[4] = uVar4 + 1;
      if (uVar3 == 0) {
        lVar8 = puVar2[1];
        if (lVar8 != puVar2[2]) {
LAB_1412b0009:
          puVar2[1] = lVar8 + 0xc;
          uVar10 = (**(code **)*puVar2)();
          cVar1 = *(char *)(puVar2 + 9);
          uVar6 = in_XMM1_Da;
          goto joined_r0x0001412b0052;
        }
      }
      else {
        puVar2[3] = 0;
        lVar8 = puVar2[2];
        uVar9 = ((ulonglong)(lVar8 - puVar2[1]) >> 2) * -0x5555555555555555;
        if (uVar3 <= uVar9 && uVar9 - uVar3 != 0) {
          lVar8 = puVar2[1] + uVar3 * 0xc;
          goto LAB_1412b0009;
        }
LAB_1412b0123:
        puVar2[1] = lVar8;
      }
    }
  }
  else if ((puVar2 != (undefined8 *)0x0 & (byte)*param_2) == 1) {
    uVar4 = param_2[1];
    uVar10 = (undefined4)param_2[2];
    cVar1 = *(char *)(puVar2 + 9);
    uVar6 = (int)(param_2[2] >> 0x20);
joined_r0x0001412b0052:
    if (cVar1 == '\0') {
      uVar3 = puVar2[3];
      lVar5 = puVar2[4];
      puVar2[4] = lVar5 + 1;
      if (uVar3 == 0) {
        lVar8 = puVar2[1];
        if (lVar8 == puVar2[2]) goto LAB_1412b0127;
      }
      else {
        puVar2[3] = 0;
        lVar8 = puVar2[2];
        uVar9 = ((ulonglong)(lVar8 - puVar2[1]) >> 2) * -0x5555555555555555;
        if (uVar9 < uVar3 || uVar9 - uVar3 == 0) goto LAB_1412b0123;
        lVar8 = puVar2[1] + uVar3 * 0xc;
      }
      puVar2[1] = lVar8 + 0xc;
      uVar11 = (**(code **)*puVar2)(lVar5);
      param_1[1] = uVar4;
      param_1[2] = CONCAT44(uVar6,uVar10);
      param_1[3] = lVar5;
      *(undefined4 *)(param_1 + 4) = uVar11;
      *(undefined4 *)((longlong)param_1 + 0x24) = in_XMM1_Da;
      uVar7 = 1;
      goto LAB_1412b0129;
    }
  }
LAB_1412b0127:
  uVar7 = 0;
LAB_1412b0129:
  *param_1 = uVar7;
  return;
}

