
void FUN_140530430(longlong *param_1,ulonglong param_2,longlong param_3,longlong param_4)

{
  byte *pbVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  int iVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong lVar7;
  code *pcVar8;
  char cVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  uint uVar12;
  uint uVar13;
  undefined4 uVar14;
  undefined4 *puVar15;
  ulonglong uVar16;
  uint *puVar17;
  longlong lVar18;
  undefined4 *puVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lStack_c0;
  uint uStack_98;
  int iStack_94;
  longlong *plStack_90;
  longlong lStack_88;
  longlong lStack_80;
  ulonglong uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  uint *puStack_48;
  
  FUN_140bd4410();
  lVar5 = *(longlong *)(param_2 + 8);
  uVar6 = *(ulonglong *)(param_2 + 0x10);
  iStack_94 = (int)param_4 + -1;
  uStack_98 = (uint)((int)param_4 != 0);
  lStack_50 = lVar5 + uVar6 * 0x48;
  lStack_68 = param_1[8];
  if ((ulonglong)(param_1[6] - lStack_68) < uVar6) {
    FUN_1428a29e0(param_1 + 6,lStack_68,uVar6,4,0x18);
    lStack_68 = param_1[8];
  }
  lStack_60 = param_1[7];
  puStack_48 = &uStack_98;
  uStack_70 = (ulonglong *)(param_1 + 8);
  lStack_58 = lVar5;
  FUN_14048da00(&lStack_58,&uStack_70);
  uVar21 = param_1[0xb];
  puVar19 = (undefined4 *)(uVar6 - uVar21);
  uVar16 = uVar6;
  lStack_c0 = param_3;
  if (uVar6 < uVar21 || puVar19 == (undefined4 *)0x0) goto code_r0x000140530572;
  if ((undefined4 *)(param_1[9] - uVar21) < puVar19) goto code_r0x000140530919;
  lVar18 = param_1[10];
  puVar15 = (undefined4 *)(uVar21 * 4 + lVar18);
  uVar16 = uVar21;
  while( true ) {
    if ((undefined4 *)0x1 < puVar19) {
      func_0x000142923c20(puVar15,0,(~uVar21 + uVar6) * 4);
      puVar15 = (undefined4 *)((uVar16 + uVar6) * 4 + uVar21 * -4 + lVar18 + -4);
      uVar16 = (longlong)puVar19 + (uVar16 - 1);
    }
    *puVar15 = 0;
    uVar16 = uVar16 + 1;
code_r0x000140530572:
    param_1[0xb] = uVar16;
    uVar21 = param_1[0xe];
    uVar20 = uVar6 - uVar21;
    uVar16 = uVar6;
    if (uVar21 <= uVar6 && uVar20 != 0) {
      if (param_1[0xc] - uVar21 < uVar20) {
        FUN_1428a29e0(param_1 + 0xc,uVar21,uVar20,4,4);
        lVar18 = param_1[0xd];
        puVar19 = (undefined4 *)(param_1[0xe] * 4 + lVar18);
        uVar16 = param_1[0xe];
      }
      else {
        lVar18 = param_1[0xd];
        puVar19 = (undefined4 *)(uVar21 * 4 + lVar18);
        uVar16 = uVar21;
      }
      if (1 < uVar20) {
        func_0x000142923c20(puVar19,0,(~uVar21 + uVar6) * 4);
        puVar19 = (undefined4 *)((uVar16 + uVar6) * 4 + uVar21 * -4 + lVar18 + -4);
        uVar16 = (uVar16 + uVar20) - 1;
      }
      *puVar19 = 0;
      uVar16 = uVar16 + 1;
    }
    param_1[0xe] = uVar16;
    FUN_140bd44b0(param_1,param_2);
    FUN_140bd4560(param_1,param_2);
    if (param_4 == 0) break;
    param_4 = param_4 * 0x40 + lStack_c0;
    plStack_90 = param_1 + 3;
    uVar21 = 0;
    lStack_88 = param_4;
    while( true ) {
      lStack_80 = param_1[2];
      if (*(longlong *)(lStack_c0 + 0x10) != 0) break;
code_r0x000140530870:
      uVar11 = FUN_140bd4b80(lStack_80);
      uVar14 = FUN_140bd4b80(param_1[2]);
      lVar18 = param_1[5];
      if (lVar18 == param_1[3]) {
        FUN_140562e70(plStack_90,&UNK_1429f34c8);
      }
      lStack_c0 = lStack_c0 + 0x40;
      uVar21 = uVar21 + 1;
      lVar7 = param_1[4];
      *(undefined4 *)(lVar7 + lVar18 * 8) = uVar11;
      *(undefined4 *)(lVar7 + 4 + lVar18 * 8) = uVar14;
      param_2 = lVar18 + 1;
      param_1[5] = param_2;
      if (lStack_c0 == lStack_88) goto code_r0x0001405308ab;
    }
    puVar19 = *(undefined4 **)(lStack_c0 + 8);
    lVar18 = *(longlong *)(lStack_c0 + 0x10) * 0xc;
    uStack_78 = uVar21;
    while (uVar16 = func_0x000140bece40(puVar19), uVar16 < (ulonglong)param_1[8]) {
      lVar7 = param_1[7];
      uVar10 = FUN_140bd4b80(param_1[2]);
      uVar11 = *puVar19;
      uVar14 = puVar19[1];
      uVar3 = *(undefined4 *)(lVar7 + 0x14 + uVar16 * 0x18);
      uVar2 = *(undefined1 *)(puVar19 + 2);
      param_4 = param_1[2];
      if (param_4 == *param_1) {
        FUN_140563600(param_1,&UNK_1429f34f8);
      }
      uVar21 = uStack_78;
      pbVar1 = (byte *)(lVar7 + uVar16 * 0x18);
      lVar7 = param_1[1];
      *(undefined4 *)(lVar7 + param_4 * 0x14) = uVar11;
      *(undefined4 *)(lVar7 + 4 + param_4 * 0x14) = uVar3;
      *(undefined4 *)(lVar7 + 8 + param_4 * 0x14) = 0xffffffff;
      *(undefined4 *)(lVar7 + 0xc + param_4 * 0x14) = uVar14;
      *(undefined1 *)(lVar7 + 0x10 + param_4 * 0x14) = uVar2;
      param_4 = param_4 + 1;
      param_1[2] = param_4;
      uVar11 = FUN_140bd4b40(uStack_78);
      if ((*pbVar1 & 1) == 0) {
        pbVar1[0] = 1;
        pbVar1[1] = 0;
        pbVar1[2] = 0;
        pbVar1[3] = 0;
        *(undefined4 *)(pbVar1 + 4) = uVar11;
      }
      if (*(int *)(pbVar1 + 8) == 1) {
        uVar13 = *(uint *)(pbVar1 + 0xc);
        uVar12 = FUN_140bd4b40(uVar21);
        if (uVar13 < uVar12) {
          uVar13 = uVar12;
        }
        *(uint *)(pbVar1 + 0xc) = uVar13;
        iVar4 = *(int *)(pbVar1 + 0x10);
      }
      else {
        uVar11 = FUN_140bd4b40(uVar21);
        pbVar1[8] = 1;
        pbVar1[9] = 0;
        pbVar1[10] = 0;
        pbVar1[0xb] = 0;
        *(undefined4 *)(pbVar1 + 0xc) = uVar11;
        iVar4 = *(int *)(pbVar1 + 0x10);
      }
      if (iVar4 == -1) {
        *(undefined4 *)(pbVar1 + 0x10) = uVar10;
      }
      *(undefined4 *)(pbVar1 + 0x14) = uVar10;
      func_0x000140c10dd0(&uStack_70,*(undefined1 *)(puVar19 + 2));
      uVar13 = uStack_70._4_4_;
      param_2 = (ulonglong)uStack_70._4_4_;
      uVar16 = func_0x000140bece40(puVar19);
      if (uVar6 <= uVar16) {
        uVar16 = FUN_1428d9518(uVar16,uVar6,&UNK_1429f3510);
        break;
      }
      cVar9 = FUN_140bd6d10(lVar5 + uVar16 * 0x48);
      if (cVar9 != '\x02') {
        if (cVar9 == '\x01') {
          uVar16 = func_0x000140bece40(puVar19);
          if ((ulonglong)param_1[0xe] <= uVar16) {
code_r0x0001405309a8:
            FUN_1428d9518(uVar16);
            pcVar8 = (code *)swi(3);
            (*pcVar8)();
            return;
          }
          puVar17 = (uint *)(uVar16 * 4 + param_1[0xd]);
          uVar13 = FUN_140bd4b00(uVar13);
        }
        else {
          uVar16 = func_0x000140bece40(puVar19);
          if ((ulonglong)param_1[0xb] <= uVar16) {
            FUN_1428d9518(uVar16,param_1[0xb],&UNK_1429f3528);
            goto code_r0x0001405309a8;
          }
          puVar17 = (uint *)(uVar16 * 4 + param_1[10]);
          uVar13 = FUN_140bd49d0(param_2);
        }
        *puVar17 = *puVar17 | uVar13;
      }
      puVar19 = puVar19 + 3;
      lVar18 = lVar18 + -0xc;
      if (lVar18 == 0) goto code_r0x000140530870;
    }
    FUN_1428d9518(uVar16);
code_r0x000140530919:
    FUN_1428a29e0(param_1 + 9,uVar21,puVar19,4,4);
    uVar16 = param_1[0xb];
    lVar18 = param_1[10];
    puVar15 = (undefined4 *)(uVar16 * 4 + lVar18);
  }
code_r0x0001405308ab:
  FUN_140bd47d0(param_1);
  FUN_140bd4860(param_1);
  return;
}

