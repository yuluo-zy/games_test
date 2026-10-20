
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140b055a0(undefined8 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5)

{
  undefined4 uVar1;
  byte bVar2;
  code *pcVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined8 *puVar12;
  longlong lVar13;
  undefined8 *puVar14;
  undefined *puVar15;
  undefined8 uVar16;
  longlong *plVar17;
  ulonglong unaff_R13;
  ulonglong uVar18;
  longlong *plVar19;
  float fVar20;
  longlong *plStack_1a8;
  longlong lStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong *plStack_108;
  longlong lStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined2 uStack_d8;
  undefined8 uStack_d4;
  undefined4 uStack_c8;
  undefined4 uStack_c0;
  longlong lStack_b8;
  undefined8 *puStack_b0;
  longlong lStack_a8;
  undefined4 uStack_a0;
  undefined5 uStack_9c;
  undefined3 uStack_97;
  undefined5 uStack_94;
  undefined1 uStack_8f;
  byte bStack_8e;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 *puStack_78;
  longlong lStack_70;
  longlong *plStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  plVar17 = *(longlong **)(param_2 + 0x10);
  uVar16 = 0;
  auVar4._8_8_ = 0;
  auVar4._0_8_ = plVar17;
  uVar10 = SUB168(auVar4 * ZEXT816(0xc),0);
  uStack_88 = param_4;
  uStack_80 = param_3;
  if ((SUB168(auVar4 * ZEXT816(0xc),8) == 0) && (uVar10 < 0x7ffffffffffffffd)) {
    unaff_R13 = *(ulonglong *)(param_2 + 8);
    puStack_78 = param_1;
    if (uVar10 == 0) {
      lVar11 = 4;
      plStack_68 = (longlong *)0x0;
      plVar19 = plStack_68;
    }
    else {
      uVar16 = 4;
      lVar11 = func_0x000140613c10(uVar10,4);
      plVar19 = plVar17;
      if (lVar11 == 0) goto LAB_140b05a09;
    }
    plStack_68 = plVar19;
    lStack_70 = lVar11;
    func_0x000142923c00(lVar11,unaff_R13,uVar10);
    uVar10 = *(ulonglong *)(param_2 + 0x28);
    param_1 = (undefined8 *)0x0;
    auVar5._8_8_ = 0;
    auVar5._0_8_ = uVar10;
    unaff_R13 = SUB168(auVar5 * ZEXT816(0xc),0);
    if ((SUB168(auVar5 * ZEXT816(0xc),8) == 0) && (unaff_R13 < 0x7ffffffffffffffd)) {
      uVar16 = *(undefined8 *)(param_2 + 0x20);
      if (unaff_R13 == 0) {
        lVar11 = 4;
        uVar18 = 0;
      }
      else {
        param_1 = (undefined8 *)0x4;
        lVar11 = func_0x000140613c10(unaff_R13,4);
        uVar18 = uVar10;
        if (lVar11 == 0) goto LAB_140b05a1b;
      }
      func_0x000142923c00(lVar11,uVar16,unaff_R13);
      plStack_1a8 = plStack_68;
      lStack_1a0 = lStack_70;
      uStack_198 = plVar17;
      uStack_190 = uVar18;
      uStack_188 = lVar11;
      uStack_180 = uVar10;
      puVar12 = (undefined8 *)func_0x000140613c10(0x13,1);
      if (puVar12 == (undefined8 *)0x0) {
        FUN_1428d8fe3(1,0x13,&UNK_142a98f20);
      }
      else {
        *(undefined4 *)((longlong)puVar12 + 0xf) = _DAT_142a9949b;
        uVar16 = CONCAT17(DAT_142a9949b,_UNK_142a99494);
        *puVar12 = _DAT_142a9948c;
        puVar12[1] = uVar16;
        plStack_108 = plStack_1a8;
        lStack_100 = lStack_1a0;
        uStack_f8 = (undefined4)uStack_198;
        uStack_f4 = uStack_198._4_4_;
        uStack_f0 = (undefined4)uStack_190;
        uStack_ec = uStack_190._4_4_;
        uStack_e8 = (undefined4)uStack_188;
        uStack_e4 = uStack_188._4_4_;
        uStack_e0 = (undefined4)uStack_180;
        uStack_dc = uStack_180._4_4_;
        uStack_9c = 0;
        uStack_97 = 0;
        uStack_94 = 0;
        bStack_8e = *(byte *)(param_2 + 0x38) ^ 1;
        uStack_118 = _DAT_14293e0c0;
        uStack_114 = _UNK_14293e0c4;
        uStack_110 = _UNK_14293e0c8;
        uStack_10c = _UNK_14293e0cc;
        uStack_d8 = 0x100;
        uStack_d4 = 0x13f800000;
        uStack_c8 = 0;
        uStack_c0 = 0;
        lStack_b8 = 0x13;
        lStack_a8 = 0x13;
        uStack_a0 = 0;
        uStack_8f = 0;
        bVar2 = *(byte *)(param_2 + 0x39);
        lVar11 = (ulonglong)(bVar2 ^ 1) * 8 + 0xb;
        puStack_b0 = puVar12;
        lVar13 = func_0x000140613c10(lVar11,1);
        if (lVar13 != 0) {
          puVar15 = &UNK_142ae7020;
          if (bVar2 != 0) {
            puVar15 = &UNK_142ae7033;
          }
          func_0x000142923c00(lVar13,puVar15,lVar11);
          func_0x000140613c20(puVar12,0x13,1);
          lStack_b8 = lVar11;
          puStack_b0 = (undefined8 *)lVar13;
          lStack_a8 = lVar11;
          func_0x000142923c00(&plStack_1a8,&uStack_118,0x90);
          FUN_14089dd50(&uStack_118,&plStack_1a8,uStack_80,uStack_88,param_5);
          uVar9 = uStack_f4;
          uVar8 = uStack_f8;
          plVar17 = plStack_108;
          uVar7 = uStack_10c;
          uVar6 = uStack_110;
          if (bVar2 != 0) {
            fVar20 = *(float *)(param_2 + 0x30) * _DAT_142934694;
            uVar16 = CONCAT44(uStack_f4,uStack_f8);
            plVar19 = (longlong *)CONCAT44(uStack_10c,uStack_110);
            if ((longlong *)CONCAT44(uStack_10c,uStack_110) == (longlong *)0x0) {
              plVar19 = plStack_108;
            }
            lVar11 = plVar19[2];
            if ((ulonglong)(*plVar19 - lVar11) < 0x20) {
              FUN_1428a9a60(plVar19,lVar11,0x20,1,1);
            }
            lVar13 = plVar19[1];
            *(code **)(lVar13 + lVar11) = FUN_1412d8c60;
            *(code **)(lVar13 + 8 + lVar11) = FUN_141564d30;
            *(undefined8 *)(lVar13 + 0x10 + lVar11) = uVar16;
            *(float *)(lVar13 + 0x18 + lVar11) = fVar20;
            *(undefined1 *)(lVar13 + 0x1c + lVar11) = 0;
            plVar19[2] = lVar11 + 0x20;
          }
          puVar12 = puStack_78;
          uVar1 = *(undefined4 *)(param_2 + 0x34);
          if ((longlong *)CONCAT44(uVar7,uVar6) != (longlong *)0x0) {
            plVar17 = (longlong *)CONCAT44(uVar7,uVar6);
          }
          lVar11 = plVar17[2];
          if ((ulonglong)(*plVar17 - lVar11) < 0x28) {
            FUN_1428a9a60(plVar17,lVar11,0x28,1,1);
          }
          lVar13 = plVar17[1];
          *(code **)(lVar13 + lVar11) = FUN_1412dc8f0;
          *(ulonglong *)(lVar13 + 8 + lVar11) = CONCAT44(uVar9,uVar8);
          *(undefined4 *)(lVar13 + 0x10 + lVar11) = 1;
          *(undefined4 *)(lVar13 + 0x14 + lVar11) = uVar1;
          *(undefined1 *)(lVar13 + 0x18 + lVar11) = 0;
          *(code **)(lVar13 + 0x20 + lVar11) = FUN_141564d30;
          plVar17[2] = lVar11 + 0x28;
          puVar14 = (undefined8 *)func_0x000140613c10(8,8);
          if (puVar14 == (undefined8 *)0x0) {
            FUN_1428d9000(8,8);
            pcVar3 = (code *)swi(3);
            puVar12 = (undefined8 *)(*pcVar3)();
            return puVar12;
          }
          *puVar14 = CONCAT44(uVar9,uVar8);
          *puVar12 = 1;
          puVar12[1] = puVar14;
          puVar12[2] = 1;
          return puVar12;
        }
        FUN_1428d8fe3(1,lVar11,&UNK_142a98f20);
      }
      goto LAB_140b05a5b;
    }
  }
  else {
LAB_140b05a09:
    FUN_1428d8fe3(uVar16,uVar10,&UNK_142c38b78);
  }
LAB_140b05a1b:
  FUN_1428d8fe3(param_1,unaff_R13,&UNK_142c38b78);
LAB_140b05a5b:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

