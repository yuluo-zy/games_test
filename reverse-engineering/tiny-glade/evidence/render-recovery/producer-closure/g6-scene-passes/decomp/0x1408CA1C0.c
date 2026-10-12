
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1408ca1c0(undefined8 *param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined8 *unaff_R14;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined8 uStack_e8;
  longlong *plStack_e0;
  undefined *puStack_d8;
  longlong lStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 *puStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  longlong lStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  longlong lStack_40;
  undefined8 *puStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  puVar7 = (undefined8 *)func_0x000140613c10(0x13,1);
  uVar5 = _UNK_142aa323e;
  uVar4 = _UNK_142aa323a;
  uVar3 = _UNK_142aa3236;
  if (puVar7 == (undefined8 *)0x0) {
    FUN_1428d8fe3(1,0x13,&UNK_142ac4680);
  }
  else {
    *(undefined4 *)puVar7 = _UNK_142aa3232;
    *(undefined4 *)((longlong)puVar7 + 4) = uVar3;
    *(undefined4 *)(puVar7 + 1) = uVar4;
    *(undefined4 *)((longlong)puVar7 + 0xc) = uVar5;
    *(undefined4 *)((longlong)puVar7 + 0xf) = 0x6c736c68;
    puVar8 = (undefined4 *)func_0x000140613c10(7,1);
    unaff_R14 = puVar7;
    if (puVar8 != (undefined4 *)0x0) {
      *(uint *)((longlong)puVar8 + 3) = CONCAT31(_UNK_142ac4584,UNK_142ac4583);
      *puVar8 = _UNK_142ac4580;
      uStack_80 = (undefined *)0x13;
      uStack_70 = (longlong **)0x13;
      uStack_68 = 7;
      uStack_58 = 7;
      uStack_50 = 0;
      uStack_48 = 8;
      lStack_40 = 0;
      uStack_78 = puVar7;
      uStack_60 = puVar8;
      puVar9 = (undefined4 *)func_0x000140613c10(7,1);
      if (puVar9 == (undefined4 *)0x0) {
        FUN_1428d8fe3(1,7,&UNK_142ac4680);
      }
      else {
        *(undefined4 *)((longlong)puVar9 + 3) = 0x6e69616d;
        *puVar9 = 0x6d5f7363;
        func_0x000140613c20(puVar8,7,1);
        uStack_68 = 7;
        uStack_58 = 7;
        uStack_c8 = (undefined4)uStack_80;
        uStack_c4 = uStack_80._4_4_;
        uStack_c0 = (undefined4)uStack_78;
        uStack_bc = uStack_78._4_4_;
        uStack_98 = (undefined4)uStack_50;
        uStack_94 = uStack_50._4_4_;
        uStack_90 = (undefined4)uStack_48;
        uStack_8c = uStack_48._4_4_;
        lStack_88 = lStack_40;
        uStack_b8 = uStack_70;
        uStack_b0 = 7;
        uStack_a0 = 7;
        puStack_a8 = puVar9;
        uStack_60 = puVar9;
        puVar7 = (undefined8 *)func_0x000140613c10(8,1);
        if (puVar7 == (undefined8 *)0x0) {
          FUN_1428d8fe3(1,8,&UNK_142aa2d40);
        }
        else {
          *puVar7 = 0x746d665f4d524550;
          plStack_e0 = &lStack_d0;
          puStack_d8 = &UNK_140533930;
          uStack_80 = &UNK_142ab5690;
          uStack_78 = (undefined8 *)0x2;
          uStack_70 = &plStack_e0;
          uStack_68 = 1;
          uStack_60 = (undefined4 *)0x0;
          lStack_d0 = param_2;
          puStack_38 = puVar7;
          FUN_140d79f90(&uStack_f8,&uStack_80);
          lVar6 = lStack_88;
          uStack_80 = (undefined *)0x8;
          uStack_78 = puStack_38;
          uStack_70 = (longlong **)0x8;
          uStack_68 = CONCAT44(uStack_f4,uStack_f8);
          uStack_60 = (undefined4 *)CONCAT44(uStack_ec,uStack_f0);
          uStack_58 = uStack_e8;
          if (lStack_88 == CONCAT44(uStack_94,uStack_98)) {
            FUN_1416a7d30(&uStack_98,&UNK_142aa3248);
          }
          lVar2 = CONCAT44(uStack_8c,uStack_90);
          lVar10 = lVar6 * 0x30;
          puVar8 = (undefined4 *)(lVar2 + 0x20 + lVar10);
          *puVar8 = (undefined4)uStack_60;
          puVar8[1] = uStack_60._4_4_;
          puVar8[2] = (undefined4)uStack_58;
          puVar8[3] = uStack_58._4_4_;
          puVar8 = (undefined4 *)(lVar2 + 0x10 + lVar10);
          *puVar8 = (undefined4)uStack_70;
          puVar8[1] = uStack_70._4_4_;
          puVar8[2] = (undefined4)uStack_68;
          puVar8[3] = uStack_68._4_4_;
          puVar8 = (undefined4 *)(lVar2 + lVar10);
          *puVar8 = (undefined4)uStack_80;
          puVar8[1] = uStack_80._4_4_;
          puVar8[2] = (undefined4)uStack_78;
          puVar8[3] = uStack_78._4_4_;
          lStack_88 = lVar6 + 1;
          puVar7 = (undefined8 *)func_0x000140613c10(7,1);
          if (puVar7 != (undefined8 *)0x0) {
            *(undefined4 *)((longlong)puVar7 + 3) = 0x79745f4d;
            *(undefined4 *)puVar7 = 0x4d524550;
            if (*(char *)(param_2 + 0x10) == '\0') {
              uVar11 = 5;
              puVar8 = (undefined4 *)func_0x000140613c10(5,1);
              if (puVar8 == (undefined4 *)0x0) {
                puStack_38 = puVar7;
                FUN_1428d8fe3(1,5,&UNK_142aa2d40);
                goto code_r0x0001408ca5da;
              }
              *(undefined1 *)(puVar8 + 1) = 0x74;
            }
            else {
              uVar11 = 6;
              puVar8 = (undefined4 *)func_0x000140613c10(6,1);
              if (puVar8 == (undefined4 *)0x0) {
                puStack_38 = puVar7;
                FUN_1428d8fe3(1,6,&UNK_142aa2d40);
                goto code_r0x0001408ca5da;
              }
              *(undefined2 *)(puVar8 + 1) = 0x3474;
            }
            lVar6 = lStack_88;
            *puVar8 = 0x616f6c66;
            uStack_80 = (undefined *)0x7;
            uStack_70 = (longlong **)0x7;
            uStack_78 = puVar7;
            uStack_68 = uVar11;
            uStack_60 = puVar8;
            uStack_58 = uVar11;
            if (lStack_88 == CONCAT44(uStack_94,uStack_98)) {
              FUN_1416a7d30(&uStack_98,&UNK_142aa3248);
            }
            lVar2 = CONCAT44(uStack_8c,uStack_90);
            lVar10 = lVar6 * 0x30;
            puVar7 = (undefined8 *)(lVar2 + 0x20 + lVar10);
            *puVar7 = uStack_60;
            puVar7[1] = uStack_58;
            puVar7 = (undefined8 *)(lVar2 + 0x10 + lVar10);
            *puVar7 = uStack_70;
            puVar7[1] = uStack_68;
            *(undefined8 *)(lVar2 + lVar10) = uStack_80;
            ((undefined8 *)(lVar2 + lVar10))[1] = uStack_78;
            param_1[8] = lVar6 + 1;
            param_1[6] = CONCAT44(uStack_94,uStack_98);
            param_1[7] = CONCAT44(uStack_8c,uStack_90);
            param_1[4] = puStack_a8;
            param_1[5] = uStack_a0;
            param_1[2] = uStack_b8;
            param_1[3] = uStack_b0;
            *param_1 = CONCAT44(uStack_c4,uStack_c8);
            param_1[1] = CONCAT44(uStack_bc,uStack_c0);
            return param_1;
          }
          FUN_1428d8fe3(1,7,&UNK_142aa2d40);
        }
      }
      goto code_r0x0001408ca5da;
    }
  }
  puStack_38 = unaff_R14;
  FUN_1428d8fe3(1,7,&UNK_142ac4680);
code_r0x0001408ca5da:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

