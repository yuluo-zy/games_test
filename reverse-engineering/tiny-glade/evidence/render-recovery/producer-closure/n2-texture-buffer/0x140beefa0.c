
undefined8 *
FUN_140beefa0(undefined8 *param_1,longlong param_2,longlong param_3,longlong param_4,
             ulonglong param_5,ulonglong *param_6,longlong param_7,longlong param_8)

{
  code *pcVar1;
  undefined8 *puVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong *puVar5;
  undefined *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined **ppuVar11;
  undefined *puVar12;
  undefined8 uVar13;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  longlong lStack_108;
  undefined8 *puStack_100;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  longlong lStack_c0;
  ulonglong uStack_b8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined4 uStack_a0;
  undefined2 uStack_9c;
  undefined1 uStack_9a;
  undefined8 uStack_98;
  undefined8 uStack_90;
  longlong lStack_88;
  ulonglong *puStack_80;
  longlong lStack_78;
  undefined8 *puStack_70;
  longlong lStack_68;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar8 = *(ulonglong *)(param_2 + 0x648);
  lStack_58 = param_2;
  if (*(short *)(param_3 + 0x14) == *(short *)(uVar8 + 0xa50)) {
    if (*(char *)(param_3 + 0x16) != '\x01') goto code_r0x000140bef372;
    lVar7 = *(longlong *)(param_3 + 8);
    lStack_50 = lVar7 + 0x10;
    lVar9 = 0;
    if (*(int *)(lVar7 + 0xc) != *(int *)(param_3 + 0x10)) goto code_r0x000140bef406;
    lVar9 = param_7 * 0x10;
    puVar6 = (undefined *)0x0;
    puVar5 = param_6;
    do {
      if (lVar9 == 0) goto code_r0x000140bef09e;
      uVar3 = *puVar5;
      uVar4 = puVar5[1];
      if (uVar4 < uVar3) {
        FUN_1428d9850(uVar3,uVar4,&UNK_142b15ae8);
code_r0x000140bef2ec:
        uVar3 = FUN_1428d9840(uVar4,param_5,&UNK_142b15ae8);
        goto code_r0x000140bef2fe;
      }
      if (param_5 < uVar4) goto code_r0x000140bef2ec;
      puVar5 = puVar5 + 2;
      puVar6 = puVar6 + (uVar4 - uVar3);
      lVar9 = lVar9 + -0x10;
    } while (puVar6 + param_8 <= *(undefined **)(lVar7 + 0x20));
    puVar6 = &UNK_142b15b60;
    FUN_1428d9430(&UNK_142b15b00,0x5c);
code_r0x000140bef09e:
    if ((*(byte *)(lStack_50 + 0x18) & 2) == 0) goto code_r0x000140bef39e;
    if (puVar6 == (undefined *)0x0) goto code_r0x000140bef3ee;
    uStack_a0 = 1;
    uStack_9a = 3;
    uStack_9c = 0;
    lVar7 = lStack_58 + 0x870;
    if (*(longlong *)(lStack_58 + 0x870) == 0) {
      lVar7 = 0;
    }
    lVar9 = *(longlong *)(lStack_58 + 0x678) + 0x10;
    puVar12 = &UNK_142b15a78;
    ppuVar11 = &puStack_a8;
    uVar13 = 0x12;
    lStack_108 = param_3;
    puStack_a8 = puVar6;
    FUN_140bee200(&puStack_f8,lStack_58 + 0x78,lVar7);
    uStack_98 = uStack_f0;
    lVar7 = uStack_98;
    uStack_90 = uStack_e8;
    puVar2 = uStack_90;
    lStack_88 = uStack_e0;
    puStack_80 = (ulonglong *)uStack_d8;
    lStack_78 = uStack_d0;
    puStack_70 = (undefined8 *)uStack_c8;
    if (puStack_f8 != (undefined *)0xb) {
      param_1[5] = uStack_d0;
      param_1[6] = uStack_c8;
      param_1[3] = uStack_e0;
      param_1[4] = uStack_d8;
      param_1[1] = uStack_f0;
      param_1[2] = uStack_e8;
      *param_1 = puStack_f8;
      param_1[7] = lStack_c0;
      param_1[8] = uStack_b8;
      param_1[9] = uStack_b0;
      return param_1;
    }
    uStack_98._0_4_ = (undefined4)uStack_f0;
    uStack_98._4_4_ = (undefined4)((ulonglong)uStack_f0 >> 0x20);
    uStack_90._0_4_ = SUB84(uStack_e8,0);
    uStack_90._4_4_ = (undefined4)((ulonglong)uStack_e8 >> 0x20);
    uStack_118 = uStack_d0;
    uStack_110 = uStack_c8;
    uStack_128 = uStack_e0;
    uStack_120 = uStack_d8;
    uStack_138 = (undefined4)uStack_98;
    uStack_134 = uStack_98._4_4_;
    uStack_130 = (undefined4)uStack_90;
    uStack_12c = uStack_90._4_4_;
    puStack_100 = param_1;
    uStack_98 = lVar7;
    uStack_90 = puVar2;
    if (lStack_c0 != 0) {
      uVar4 = uStack_b8;
      puVar5 = param_6;
      lVar7 = lStack_c0;
      if (param_7 != 0) {
        do {
          uVar3 = *puVar5;
          uVar8 = puVar5[1];
          uVar10 = uVar8 - uVar3;
          if (uVar8 < uVar3) goto code_r0x000140bef2fe;
          if (param_5 < uVar8) {
            FUN_1428d9840(uVar8,param_5,&UNK_142b15ab8,uVar8,lVar9,ppuVar11,puVar12,uVar13);
            goto code_r0x000140bef42a;
          }
          if (uVar4 < uVar10) {
            FUN_1428d9840(uVar10,uVar4,&UNK_142b15ad0,uVar8,lVar9,ppuVar11,puVar12,uVar13);
            goto code_r0x000140bef42a;
          }
          func_0x000142923c00(lVar7,uVar3 + param_4,uVar10);
          puVar5 = puVar5 + 2;
          uVar4 = uVar4 - uVar10;
          lVar7 = lVar7 + uVar10;
        } while (puVar5 != param_6 + param_7 * 2);
      }
      uStack_98 = lStack_108;
      uStack_90 = (undefined8 *)&uStack_138;
      lStack_88 = lStack_58;
      puStack_80 = param_6;
      lStack_78 = param_7;
      puStack_70 = &param_8;
      lStack_68 = lStack_50;
      FUN_140c04280(&puStack_f8,lStack_58,&uStack_98);
      puVar2 = puStack_100;
      if (puStack_f8 == (undefined *)0xb) {
        *puStack_100 = 0xb;
      }
      else {
        puStack_100[9] = uStack_b0;
        puStack_100[7] = lStack_c0;
        puStack_100[8] = uStack_b8;
        puStack_100[5] = uStack_d0;
        puStack_100[6] = uStack_c8;
        puStack_100[3] = uStack_e0;
        puStack_100[4] = uStack_d8;
        *(undefined4 *)(puStack_100 + 1) = (undefined4)uStack_f0;
        *(undefined4 *)((longlong)puStack_100 + 0xc) = uStack_f0._4_4_;
        *(undefined4 *)(puStack_100 + 2) = (undefined4)uStack_e8;
        *(undefined4 *)((longlong)puStack_100 + 0x14) = uStack_e8._4_4_;
        *puStack_100 = puStack_f8;
      }
      FUN_140be24b0(&uStack_130);
      return puVar2;
    }
  }
  else {
    puStack_f8 = (undefined *)0x0;
    param_3 = param_3 + 0x14;
    FUN_1428ca816(0,param_3,uVar8 + 0xa50,&puStack_f8,&UNK_142d022a0);
code_r0x000140bef372:
    puStack_f8 = (undefined *)0x0;
    FUN_1428ca902(0,param_3 + 0x16,&UNK_142d022d9,&puStack_f8,&UNK_142d022c0);
code_r0x000140bef39e:
    uStack_98 = lStack_50;
    uStack_90 = (undefined8 *)&UNK_1405338a0;
    puStack_f8 = &UNK_142b15a40;
    uStack_f0 = 2;
    uStack_d8 = 0;
    uStack_e8 = &uStack_98;
    uStack_e0 = 1;
    FUN_1428d9390(&puStack_f8,&UNK_142b15a60);
code_r0x000140bef3ee:
    FUN_1428d9430(&UNK_142b0f830,0x1a,&UNK_142b0f898);
    lVar9 = lStack_50;
code_r0x000140bef406:
    lStack_50 = lVar9;
    FUN_1428d9310(&UNK_142b159c8);
  }
  FUN_1428d9330(&UNK_142b15a8a,0x14,&UNK_142b15aa0);
code_r0x000140bef42a:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
code_r0x000140bef2fe:
  FUN_1428d9850(uVar3,uVar8,&UNK_142b15ab8);
  goto code_r0x000140bef42a;
}

