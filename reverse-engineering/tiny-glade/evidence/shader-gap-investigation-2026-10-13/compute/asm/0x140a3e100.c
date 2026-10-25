
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140a3e100(undefined8 *param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char cVar6;
  undefined8 *puVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  undefined8 *puVar10;
  longlong lVar11;
  undefined8 *unaff_R14;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  longlong lStack_e8;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 *puStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 *puStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  puVar7 = (undefined8 *)func_0x000140613c10(0xd,1);
  if (puVar7 == (undefined8 *)0x0) {
    FUN_1428d8fe3(1,0xd,&UNK_142ac4680);
  }
  else {
    *(undefined8 *)((longlong)puVar7 + 5) = 0x6c736c682e73632e;
    *puVar7 = 0x73632e727367742f;
    puVar8 = (undefined4 *)func_0x000140613c10(7,1);
    unaff_R14 = puVar7;
    if (puVar8 != (undefined4 *)0x0) {
      *(uint *)((longlong)puVar8 + 3) = CONCAT31(_UNK_142ac4584,DAT_142ac4583);
      *puVar8 = _DAT_142ac4580;
      uStack_a8 = 0xd;
      uStack_98 = 0xd;
      uStack_90 = 7;
      uStack_80 = 7;
      uStack_78 = (undefined8 *)0x0;
      uStack_70 = (undefined *)0x8;
      lStack_68 = 0;
      puStack_a0 = puVar7;
      puStack_88 = puVar8;
      puVar9 = (undefined4 *)func_0x000140613c10(7,1);
      if (puVar9 == (undefined4 *)0x0) {
        FUN_1428d8fe3(1,7,&UNK_142ac4680);
      }
      else {
        *(undefined4 *)((longlong)puVar9 + 3) = 0x6e69616d;
        *puVar9 = 0x6d5f7363;
        func_0x000140613c20(puVar8,7,1);
        uVar5 = uStack_98;
        puVar7 = puStack_a0;
        uVar4 = uStack_a8;
        uStack_90 = 7;
        uStack_80 = 7;
        uStack_f8 = (undefined4)uStack_78;
        uStack_f4 = uStack_78._4_4_;
        uStack_f0 = (undefined4)uStack_70;
        uStack_ec = uStack_70._4_4_;
        lStack_e8 = lStack_68;
        puStack_88 = puVar9;
        puVar10 = (undefined8 *)func_0x000140613c10(0xb,1);
        if (puVar10 == (undefined8 *)0x0) {
          FUN_1428d8fe3(1,0xb,&UNK_142acc780);
        }
        else {
          *puVar10 = 0x7661775f4d524550;
          *(undefined4 *)((longlong)puVar10 + 7) = 0x32336576;
          uStack_60 = 0;
          uStack_58 = 1;
          uStack_50 = 0;
          uStack_a8 = 0;
          uStack_98 = 0;
          puStack_88 = (undefined4 *)0x20;
          uStack_80 = CONCAT71(uStack_80._1_7_,3);
          uStack_70 = &UNK_142ab53c0;
          uStack_78 = &uStack_60;
          puStack_48 = puVar10;
          cVar6 = FUN_140d8d580(param_2,&uStack_a8);
          lVar3 = lStack_e8;
          if (cVar6 == '\0') {
            uStack_c0 = (undefined4)uStack_60;
            uStack_bc = uStack_60._4_4_;
            uStack_b8 = (undefined4)uStack_58;
            uStack_b4 = uStack_58._4_4_;
            uStack_b0 = uStack_50;
            uStack_d8 = 0xb;
            uStack_d0 = puStack_48;
            uStack_c8 = 0xb;
            if (lStack_e8 == CONCAT44(uStack_f4,uStack_f8)) {
              FUN_1416a7d30(&uStack_f8,&UNK_142acd758);
            }
            lVar2 = CONCAT44(uStack_ec,uStack_f0);
            lVar11 = lVar3 * 0x30;
            puVar8 = (undefined4 *)(lVar2 + 0x20 + lVar11);
            *puVar8 = uStack_b8;
            puVar8[1] = uStack_b4;
            puVar8[2] = (undefined4)uStack_b0;
            puVar8[3] = uStack_b0._4_4_;
            puVar8 = (undefined4 *)(lVar2 + 0x10 + lVar11);
            *puVar8 = (undefined4)uStack_c8;
            puVar8[1] = uStack_c8._4_4_;
            puVar8[2] = uStack_c0;
            puVar8[3] = uStack_bc;
            puVar8 = (undefined4 *)(lVar2 + lVar11);
            *puVar8 = (undefined4)uStack_d8;
            puVar8[1] = uStack_d8._4_4_;
            puVar8[2] = (undefined4)uStack_d0;
            puVar8[3] = uStack_d0._4_4_;
            lStack_e8 = lVar3 + 1;
            puVar10 = (undefined8 *)func_0x000140613c10(0xc,1);
            if (puVar10 == (undefined8 *)0x0) {
              FUN_1428d8fe3(1,0xc,&UNK_142acc780);
            }
            else {
              *puVar10 = 0x6168735f4d524550;
              *(undefined4 *)(puVar10 + 1) = 0x6e657072;
              uStack_60 = 0;
              uStack_58 = 1;
              uStack_50 = 0;
              uStack_a8 = 0;
              uStack_98 = 0;
              puStack_88 = (undefined4 *)0x20;
              uStack_80 = CONCAT71(uStack_80._1_7_,3);
              uStack_70 = &UNK_142ab53c0;
              uStack_78 = &uStack_60;
              puStack_48 = puVar10;
              cVar6 = FUN_140d8d580(param_2 + 4,&uStack_a8);
              lVar3 = lStack_e8;
              if (cVar6 == '\0') {
                uStack_c0 = (undefined4)uStack_60;
                uStack_bc = uStack_60._4_4_;
                uStack_b8 = (undefined4)uStack_58;
                uStack_b4 = uStack_58._4_4_;
                uStack_b0 = uStack_50;
                uStack_d8 = 0xc;
                uStack_d0 = puStack_48;
                uStack_c8 = 0xc;
                if (lStack_e8 == CONCAT44(uStack_f4,uStack_f8)) {
                  FUN_1416a7d30(&uStack_f8,&UNK_142acd758);
                }
                lVar2 = CONCAT44(uStack_ec,uStack_f0);
                lVar11 = lVar3 * 0x30;
                puVar10 = (undefined8 *)(lVar2 + 0x20 + lVar11);
                *puVar10 = CONCAT44(uStack_b4,uStack_b8);
                puVar10[1] = uStack_b0;
                puVar10 = (undefined8 *)(lVar2 + 0x10 + lVar11);
                *puVar10 = uStack_c8;
                puVar10[1] = CONCAT44(uStack_bc,uStack_c0);
                *(undefined8 *)(lVar2 + lVar11) = uStack_d8;
                ((undefined8 *)(lVar2 + lVar11))[1] = uStack_d0;
                param_1[8] = lVar3 + 1;
                param_1[6] = CONCAT44(uStack_f4,uStack_f8);
                param_1[7] = CONCAT44(uStack_ec,uStack_f0);
                param_1[4] = puVar9;
                param_1[5] = 7;
                param_1[2] = uVar5;
                param_1[3] = 7;
                *param_1 = uVar4;
                param_1[1] = puVar7;
                return param_1;
              }
              FUN_1428d9760(&UNK_142ab53f0,0x37,&uStack_d8,&UNK_142ab5490,&UNK_142ab5478);
            }
          }
          else {
            FUN_1428d9760(&UNK_142ab53f0,0x37,&uStack_d8,&UNK_142ab5490,&UNK_142ab5478);
          }
        }
      }
      goto LAB_140a3e58e;
    }
  }
  puStack_48 = unaff_R14;
  FUN_1428d8fe3(1,7,&UNK_142ac4680);
LAB_140a3e58e:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

