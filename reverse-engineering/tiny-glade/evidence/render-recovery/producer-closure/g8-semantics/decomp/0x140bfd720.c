
undefined8 *
FUN_140bfd720(undefined8 *param_1,longlong param_2,undefined4 *param_3,ulonglong *param_4)

{
  undefined2 uVar1;
  undefined4 uVar2;
  undefined8 *puVar3;
  code *pcVar4;
  uint uVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  byte bVar9;
  ulonglong *puVar10;
  undefined8 *puVar11;
  longlong lVar12;
  bool bVar13;
  undefined4 extraout_XMM0_Da;
  undefined4 uVar14;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined4 uStack_2a0;
  undefined4 uStack_29c;
  undefined4 uStack_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined8 uStack_288;
  ulonglong uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined1 auStack_1a8 [192];
  undefined1 auStack_e8 [192];
  
  if (param_4 == (ulonglong *)0x0) {
    bVar9 = 2;
    uVar6 = 0;
code_r0x000140bfd787:
    uStack_288 = *(undefined8 *)(param_3 + 8);
    uStack_2a8 = *param_3;
    uStack_2a4 = param_3[1];
    uStack_2a0 = param_3[2];
    uStack_29c = param_3[3];
    uStack_298 = param_3[4];
    uStack_294 = param_3[5];
    uStack_290 = param_3[6];
    uStack_28c = param_3[7];
    FUN_1418ca030(&uStack_278,param_2,&uStack_2a8,uVar6,bVar9);
    if ((uStack_278 & 1) == 0) {
      func_0x000142923c00(auStack_1a8,&uStack_270,0xc0);
      func_0x000142923c00(auStack_e8,auStack_1a8,0xc0);
      uVar2 = param_3[4];
      puVar3 = *(undefined8 **)(param_2 + 0x648);
      puVar8 = (undefined8 *)puVar3[0x8d];
      do {
        puVar11 = puVar8;
        if (puVar11 == (undefined8 *)0x0) {
          func_0x000142923c00(&uStack_268,auStack_e8,0xc0);
          uStack_278 = 0;
          uStack_270 = 1;
          puVar10 = &uStack_278;
          uVar7 = FUN_1418bbd30(puVar3 + 0x5f);
          if ((ulonglong)puVar3[0x8b] <= uVar7) goto code_r0x000140bfd997;
          uVar5 = func_0x000140c111b0(uVar7);
          uVar14 = extraout_XMM0_Da;
          if (0x2b < uVar5) goto code_r0x000140bfd9bb;
          puVar11 = (undefined8 *)((longlong)puVar10 * 0xd0 + (puVar3 + 0x5f)[uVar5]);
          goto code_r0x000140bfd8f3;
        }
        LOCK();
        puVar8 = (undefined8 *)puVar3[0x8d];
        bVar13 = puVar11 == puVar8;
        if (bVar13) {
          puVar3[0x8d] = puVar11[2];
          puVar8 = puVar11;
        }
        UNLOCK();
      } while (!bVar13);
      func_0x000142923c10(puVar11 + 2,auStack_e8,0xc0);
      *(undefined4 *)(puVar11 + 1) = 1;
      *puVar11 = 0;
code_r0x000140bfd8f3:
      uVar14 = *(undefined4 *)((longlong)puVar11 + 0xc);
      uVar1 = *(undefined2 *)(puVar3 + 0x14a);
      param_1[1] = *puVar3;
      param_1[2] = puVar11;
      *(undefined4 *)(param_1 + 3) = uVar14;
      *(undefined2 *)((longlong)param_1 + 0x1c) = uVar1;
      *(undefined1 *)((longlong)param_1 + 0x1e) = 2;
      *(undefined4 *)(param_1 + 4) = uVar2;
      *(undefined1 *)((longlong)param_1 + 0x24) = 0;
      *param_1 = 6;
    }
    else {
      param_1[6] = uStack_240;
      param_1[7] = uStack_238;
      param_1[4] = uStack_250;
      param_1[5] = uStack_248;
      param_1[2] = uStack_260;
      param_1[3] = uStack_258;
      *param_1 = uStack_270;
      param_1[1] = uStack_268;
    }
    return param_1;
  }
  if (*(short *)((longlong)param_4 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)((longlong)param_4 + 0x16) == '\x02') {
      if (*(int *)(param_4[1] + 0xc) == (int)param_4[2]) {
        lVar12 = param_2;
        uVar6 = FUN_1418ccb50(param_4[1] + 0x10);
        bVar9 = (byte)lVar12 & 1;
        goto code_r0x000140bfd787;
      }
      goto code_r0x000140bfd9af;
    }
  }
  else {
    uStack_278 = 0;
    lVar12 = (longlong)param_4 + 0x14;
    param_4 = &uStack_278;
    FUN_1428ca816(0,lVar12,*(longlong *)(param_2 + 0x648) + 0xa50,param_4,&UNK_142d022a0);
  }
  uStack_278 = 0;
  FUN_1428ca902(0,(longlong)param_4 + 0x16,&UNK_142d022d8,&uStack_278,&UNK_142d022c0);
code_r0x000140bfd997:
  FUN_1428d9430(&UNK_142d0528e,0x22,&UNK_142d02268);
code_r0x000140bfd9af:
  uVar14 = FUN_1428d9310(&UNK_142b16cf8);
code_r0x000140bfd9bb:
  FUN_1428d9518(uVar14,0x2c,&UNK_142d02268);
  pcVar4 = (code *)swi(3);
  puVar8 = (undefined8 *)(*pcVar4)();
  return puVar8;
}

