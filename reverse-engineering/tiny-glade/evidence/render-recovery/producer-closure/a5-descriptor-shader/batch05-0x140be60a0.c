
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong * FUN_140be60a0(ulonglong *param_1,longlong param_2,longlong param_3,undefined4 param_4)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong *plVar3;
  undefined4 *puVar4;
  undefined1 auVar5 [16];
  ulonglong uVar6;
  code *pcVar7;
  undefined8 uVar8;
  int iVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  undefined8 *puVar12;
  byte *pbVar13;
  longlong lVar14;
  undefined1 (**ppauVar15) [16];
  undefined4 uVar16;
  longlong unaff_R14;
  int iStack_108;
  undefined4 uStack_104;
  undefined8 uStack_100;
  undefined8 uStack_f0;
  undefined1 (*pauStack_e8) [16];
  undefined1 (*pauStack_e0) [16];
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_ac;
  undefined8 *puStack_a8;
  undefined8 uStack_a0;
  longlong lStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  ulonglong uStack_80;
  ulonglong uStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uStack_ac = param_4;
  puVar10 = (undefined8 *)func_0x000140613c10(0x58,8);
  if (puVar10 == (undefined8 *)0x0) {
    FUN_1428d9000(8,0x58);
code_r0x000140be64e3:
    FUN_1428d8fe3(8,unaff_R14,&UNK_142b13890);
  }
  else {
    *puVar10 = 0;
    *(undefined4 *)(puVar10 + 10) = 0;
    uVar6 = *(ulonglong *)(param_3 + 0x10);
    puStack_a8 = puVar10;
    if (4 < uVar6) {
      FUN_1428d9430(&UNK_142b13840,0x32,&UNK_142b13878);
      goto code_r0x000140be64f7;
    }
    if (uVar6 == 0) {
      uStack_78 = 0;
      uStack_70 = 8;
      uStack_68 = 0;
      uVar11 = 8;
    }
    else {
      unaff_R14 = uVar6 * 8;
      uVar11 = func_0x000140613c10(unaff_R14,8);
      if (uVar11 == 0) goto code_r0x000140be64e3;
      uStack_68 = 0;
      uStack_78 = uVar6;
      uStack_70 = uVar11;
      uVar11 = func_0x000140613c10(uVar6 * 0x30,8);
      if (uVar11 == 0) {
        FUN_1428d8fe3(8,uVar6 * 0x30,&UNK_142b138a8);
        goto code_r0x000140be64f7;
      }
    }
    uStack_80 = 0;
    puVar10 = *(undefined8 **)(param_3 + 8);
    puVar12 = puVar10;
    lVar14 = 0;
    uStack_90 = uVar6;
    uStack_88 = uVar11;
    while( true ) {
      while( true ) {
        if (puVar12 == puVar10 + uVar6 * 6) {
          param_1[2] = uStack_68;
          *param_1 = uStack_78;
          param_1[1] = uStack_70;
          param_1[3] = uStack_90;
          param_1[4] = uStack_88;
          param_1[5] = uStack_80;
          FUN_140bde2a0(&puStack_a8);
          return param_1;
        }
        puVar1 = puVar12 + 6;
        lVar2 = lVar14 + 1;
        if (puVar12[3] == 0) break;
        uVar16 = 0x7fffffff;
        if (lVar14 == 0) {
          uVar16 = uStack_ac;
        }
        pauStack_e8 = (undefined1 (*) [16])*puVar12;
        uStack_d8 = *pauStack_e8 + puVar12[1] + 1;
        auVar5 = *pauStack_e8;
        pauStack_e0 = pauStack_e8 + 1;
        uStack_d0 = CONCAT62(uStack_d0._2_6_,
                             ~((ushort)(SUB161(auVar5 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar5[0xf] >> 7) << 0xf));
        uStack_c8 = puVar12[3];
        FUN_1418c1a10(&iStack_108,param_2,&pauStack_e8,uVar16,&puStack_a8);
        uVar11 = uStack_68;
        uVar8 = uStack_f0;
        lStack_98 = CONCAT44(uStack_104,iStack_108);
        uStack_a0 = uStack_100;
        if (uStack_68 == uStack_78) {
          FUN_140562870(&uStack_78,&UNK_142b138c0);
        }
        *(undefined8 *)(uStack_70 + uVar11 * 8) = uVar8;
        uStack_68 = uVar11 + 1;
        FUN_1418c2800(&pauStack_e8);
        uVar11 = uStack_80;
        if (uStack_80 == uStack_90) {
          FUN_1405629f0(&uStack_90,&UNK_142b138d8);
        }
        lVar14 = uVar11 * 0x30;
        puVar4 = (undefined4 *)(uStack_88 + 0x20 + lVar14);
        *puVar4 = (undefined4)uStack_c8;
        puVar4[1] = uStack_c8._4_4_;
        puVar4[2] = (undefined4)uStack_c0;
        puVar4[3] = uStack_c0._4_4_;
        puVar4 = (undefined4 *)(uStack_88 + 0x10 + lVar14);
        *puVar4 = (undefined4)uStack_d8;
        puVar4[1] = uStack_d8._4_4_;
        puVar4[2] = (undefined4)uStack_d0;
        puVar4[3] = uStack_d0._4_4_;
        *(undefined8 *)(uStack_88 + lVar14) = pauStack_e8;
        ((undefined8 *)(uStack_88 + lVar14))[1] = pauStack_e0;
        uStack_80 = uVar11 + 1;
        puVar12 = puVar1;
        lVar14 = lVar2;
        if (lStack_98 != 0) {
          func_0x000140613c20(uStack_a0,lStack_98 * 0x18);
        }
      }
      pauStack_e8 = (undefined1 (*) [16])CONCAT44(pauStack_e8._4_4_,0x20);
      pauStack_e0 = (undefined1 (*) [16])0x0;
      uStack_d8 = (undefined1 *)0x0;
      uStack_d0 = 0;
      ppauVar15 = &pauStack_e8;
      iVar9 = (**(code **)(param_2 + 0x230))
                        (*(undefined8 *)(param_2 + 0x640),ppauVar15,0,&iStack_108);
      uVar11 = uStack_68;
      if (iVar9 != 0) break;
      uVar8 = CONCAT44(uStack_104,iStack_108);
      if (uStack_68 == uStack_78) {
        ppauVar15 = (undefined1 (**) [16])&UNK_142b13908;
        FUN_140562870(&uStack_78);
      }
      *(undefined8 *)(uStack_70 + uVar11 * 8) = uVar8;
      uStack_68 = uVar11 + 1;
      pbVar13 = (byte *)FUN_140d4d740();
      if ((*pbVar13 & 1) == 0) {
        uStack_c8 = FUN_140d64e00();
        pbVar13[0] = 1;
        pbVar13[1] = 0;
        pbVar13[2] = 0;
        pbVar13[3] = 0;
        pbVar13[4] = 0;
        pbVar13[5] = 0;
        pbVar13[6] = 0;
        pbVar13[7] = 0;
        *(longlong *)(pbVar13 + 8) = uStack_c8;
        *(undefined1 (***) [16])(pbVar13 + 0x10) = ppauVar15;
      }
      else {
        uStack_c8 = *(longlong *)(pbVar13 + 8);
        ppauVar15 = *(undefined1 (***) [16])(pbVar13 + 0x10);
      }
      uVar11 = uStack_80;
      *(longlong *)(pbVar13 + 8) = uStack_c8 + 1;
      uStack_d8 = _UNK_142b13690;
      uStack_d0 = _UNK_142b13698;
      pauStack_e8 = _UNK_142b13680;
      pauStack_e0 = _UNK_142b13688;
      uStack_c0 = ppauVar15;
      if (uStack_80 == uStack_90) {
        FUN_1405629f0(&uStack_90,&UNK_142b13920);
      }
      lVar14 = uVar11 * 0x30;
      plVar3 = (longlong *)(uStack_88 + 0x20 + lVar14);
      *plVar3 = uStack_c8;
      plVar3[1] = (longlong)uStack_c0;
      plVar3 = (longlong *)(uStack_88 + 0x10 + lVar14);
      *plVar3 = (longlong)uStack_d8;
      plVar3[1] = uStack_d0;
      *(undefined8 *)(uStack_88 + lVar14) = pauStack_e8;
      ((undefined8 *)(uStack_88 + lVar14))[1] = pauStack_e0;
      uStack_80 = uVar11 + 1;
      puVar12 = puVar1;
      lVar14 = lVar2;
    }
    iStack_108 = iVar9;
    FUN_1428d9760(&UNK_142b13640,0x2b,&iStack_108,&UNK_142b13620,&UNK_142b138f0);
  }
code_r0x000140be64f7:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

