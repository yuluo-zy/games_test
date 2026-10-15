
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1409e8db0(undefined8 *param_1,undefined8 param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  char cVar8;
  undefined8 *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  longlong lVar12;
  undefined8 *unaff_R14;
  undefined8 uStack_118;
  undefined8 *puStack_110;
  undefined8 uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 *puStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong lStack_a8;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 *puStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  longlong lStack_40;
  undefined8 *puStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  puVar9 = (undefined8 *)func_0x000140613c10(0x2a,1);
  uVar2 = _UNK_142ac4be7;
  if (puVar9 == (undefined8 *)0x0) {
    FUN_1428d8fe3(1,0x2a,&UNK_142ac4680);
  }
  else {
    *(ulonglong *)((longlong)puVar9 + 0x1a) = CONCAT26(_UNK_142ac4be5,_UNK_142ac4bdf);
    *(undefined8 *)((longlong)puVar9 + 0x22) = uVar2;
    uVar2 = CONCAT62(_UNK_142ac4bdf,_UNK_142ac4bdd);
    puVar9[2] = _UNK_142ac4bd5;
    puVar9[3] = uVar2;
    uVar6 = _UNK_142ac4bd1;
    uVar5 = _UNK_142ac4bcd;
    uVar4 = _UNK_142ac4bc9;
    *(undefined4 *)puVar9 = _UNK_142ac4bc5;
    *(undefined4 *)((longlong)puVar9 + 4) = uVar4;
    *(undefined4 *)(puVar9 + 1) = uVar5;
    *(undefined4 *)((longlong)puVar9 + 0xc) = uVar6;
    puVar10 = (undefined4 *)func_0x000140613c10(7,1);
    unaff_R14 = puVar9;
    if (puVar10 != (undefined4 *)0x0) {
      *(undefined4 *)((longlong)puVar10 + 3) = 0x6e69616d;
      *puVar10 = 0x6d5f7363;
      uStack_80 = 0x2a;
      uStack_70 = 0x2a;
      uStack_68 = 7;
      uStack_58 = 7;
      uStack_50 = (undefined8 *)0x0;
      uStack_48 = (undefined *)0x8;
      lStack_40 = 0;
      uStack_78 = puVar9;
      puStack_60 = puVar10;
      puVar11 = (undefined4 *)func_0x000140613c10(7,1);
      if (puVar11 == (undefined4 *)0x0) {
        FUN_1428d8fe3(1,7,&UNK_142ac4680);
      }
      else {
        *(undefined4 *)((longlong)puVar11 + 3) = 0x6e69616d;
        *puVar11 = 0x6d5f7363;
        func_0x000140613c20(puVar10,7,1);
        uStack_68 = 7;
        uStack_58 = 7;
        uStack_e8 = (undefined4)uStack_80;
        uStack_e4 = uStack_80._4_4_;
        uStack_e0 = (undefined4)uStack_78;
        uStack_dc = uStack_78._4_4_;
        uStack_b8 = (undefined4)uStack_50;
        uStack_b4 = uStack_50._4_4_;
        uStack_b0 = (undefined4)uStack_48;
        uStack_ac = uStack_48._4_4_;
        lStack_a8 = lStack_40;
        uStack_d8 = uStack_70;
        uStack_d0 = 7;
        uStack_c0 = 7;
        puStack_c8 = puVar11;
        puStack_60 = puVar11;
        puVar9 = (undefined8 *)func_0x000140613c10(10,1);
        if (puVar9 == (undefined8 *)0x0) {
          FUN_1428d8fe3(1,10,&UNK_142ac4680);
        }
        else {
          *puVar9 = 0x6972775f4d524550;
          *(undefined2 *)(puVar9 + 1) = 0x6574;
          uStack_98 = 0;
          uStack_90 = 1;
          uStack_88 = 0;
          uStack_80 = 0;
          uStack_70 = 0;
          puStack_60 = (undefined4 *)0x20;
          uStack_58 = CONCAT71(uStack_58._1_7_,3);
          uStack_50 = &uStack_98;
          uStack_48 = &UNK_142ab53c0;
          puStack_38 = puVar9;
          cVar8 = FUN_140d8d580(param_2,&uStack_80);
          lVar7 = lStack_a8;
          if (cVar8 == '\0') {
            uStack_100 = (undefined4)uStack_98;
            uStack_fc = uStack_98._4_4_;
            uStack_f8 = (undefined4)uStack_90;
            uStack_f4 = uStack_90._4_4_;
            uStack_f0 = uStack_88;
            uStack_118 = 10;
            puStack_110 = puStack_38;
            uStack_108 = 10;
            if (lStack_a8 == CONCAT44(uStack_b4,uStack_b8)) {
              FUN_1416a7d30(&uStack_b8,&UNK_142ac4bf0);
            }
            lVar3 = CONCAT44(uStack_ac,uStack_b0);
            lVar12 = lVar7 * 0x30;
            puVar9 = (undefined8 *)(lVar3 + 0x20 + lVar12);
            *puVar9 = CONCAT44(uStack_f4,uStack_f8);
            puVar9[1] = uStack_f0;
            puVar9 = (undefined8 *)(lVar3 + 0x10 + lVar12);
            *puVar9 = uStack_108;
            puVar9[1] = CONCAT44(uStack_fc,uStack_100);
            *(undefined8 *)(lVar3 + lVar12) = uStack_118;
            ((undefined8 *)(lVar3 + lVar12))[1] = puStack_110;
            param_1[8] = lVar7 + 1;
            param_1[6] = CONCAT44(uStack_b4,uStack_b8);
            param_1[7] = CONCAT44(uStack_ac,uStack_b0);
            param_1[4] = puStack_c8;
            param_1[5] = uStack_c0;
            param_1[2] = uStack_d8;
            param_1[3] = uStack_d0;
            *param_1 = CONCAT44(uStack_e4,uStack_e8);
            param_1[1] = CONCAT44(uStack_dc,uStack_e0);
            return param_1;
          }
          FUN_1428d9760(&UNK_142ab53f0,0x37,&uStack_118,&UNK_142ab5490,&UNK_142ab5478);
        }
      }
      goto code_r0x0001409e90e8;
    }
  }
  puStack_38 = unaff_R14;
  FUN_1428d8fe3(1,7,&UNK_142ac4680);
code_r0x0001409e90e8:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

