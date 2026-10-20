
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408bf9c0(undefined8 *param_1,longlong *param_2)

{
  longlong lVar1;
  char *pcVar2;
  undefined1 uVar3;
  code *pcVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined *puVar9;
  longlong *plVar10;
  undefined4 uVar11;
  undefined1 uVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  longlong lStack_128;
  undefined8 *puStack_120;
  undefined8 uStack_118;
  longlong lStack_110;
  undefined8 uStack_108;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  uint uStack_c0;
  longlong lStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  uint uStack_88;
  undefined1 uStack_84;
  undefined2 uStack_83;
  undefined1 uStack_81;
  undefined8 *puStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lStack_b8 = *param_2;
  lVar15 = 0;
  do {
    if (*(longlong *)(lStack_b8 + 0x10) == lVar15) {
      lVar15 = (ulonglong)*(byte *)(lStack_b8 + 0x58) * 8;
      puVar9 = &UNK_142aef51a;
      FUN_140d62810(&lStack_110,&UNK_142aef51a,0xc,*(undefined8 *)(&UNK_142aa1540 + lVar15),
                    *(undefined8 *)(&UNK_142aa1568 + lVar15));
      uVar7 = FUN_140d76ed0(&lStack_110);
      FUN_140d62810(&uStack_b0,uVar7,puVar9,&UNK_142aa0da7,0xc);
      uVar8 = FUN_140d76ed0(&uStack_b0);
      FUN_140d59140(&lStack_128,uVar8,uVar7);
      if (CONCAT44(uStack_ac,uStack_b0) != 0) {
        func_0x000140613c20(CONCAT44(uStack_a4,uStack_a8),CONCAT44(uStack_ac,uStack_b0),1);
      }
      if (SBORROW8(0,lStack_128)) {
        if (((uint)puStack_120 & 3) == 1) {
          lStack_f0 = (longlong)puStack_120 + -1;
          lStack_78 = *(longlong *)((longlong)puStack_120 + -1);
          puStack_80 = *(undefined8 **)((longlong)puStack_120 + 7);
          if ((code *)*puStack_80 != (code *)0x0) {
            (*(code *)*puStack_80)(lStack_78);
          }
          if (puStack_80[1] != 0) {
            func_0x000140613c20(lStack_78,puStack_80[1],puStack_80[2]);
          }
          func_0x000140613c20(lStack_f0,0x18,8);
        }
        if (lStack_110 != 0) {
          func_0x000140613c20(uStack_108,lStack_110,1);
        }
        uStack_e0 = (undefined4 *)func_0x000140613c10(7,1);
        if (uStack_e0 == (undefined4 *)0x0) goto code_r0x0001408c004f;
        *(uint *)((longlong)uStack_e0 + 3) = CONCAT31(_UNK_142ae6bf3,UNK_142ae6bf2);
        *uStack_e0 = _UNK_142ae6bef;
        uStack_c0 = uStack_c0 & 0xffffff00;
        uStack_e8 = 7;
        uStack_d8 = 7;
        uStack_d0 = _UNK_142a9be30;
        uStack_cc = _UNK_142a9be34;
        uStack_c8 = _UNK_142a9be38;
        uStack_c4 = _UNK_142a9be3c;
        uVar12 = 0;
      }
      else {
        lStack_78 = lStack_128;
        puStack_80 = puStack_120;
        FUN_141405ec0(&uStack_b0,puStack_120,uStack_118);
        if (SBORROW8(0,CONCAT44(uStack_ac,uStack_b0))) goto code_r0x0001408c0065;
        uStack_c8 = uStack_90;
        uStack_c4 = uStack_8c;
        uStack_c0 = uStack_88;
        uStack_d8 = CONCAT44(uStack_9c,uStack_a0);
        uStack_d0 = uStack_98;
        uStack_cc = uStack_94;
        uStack_e8 = CONCAT44(uStack_ac,uStack_b0);
        uStack_e0 = (undefined4 *)CONCAT44(uStack_a4,uStack_a8);
        if (lStack_78 != 0) {
          func_0x000140613c20(puStack_80,lStack_78,1);
        }
        if (lStack_110 != 0) {
          func_0x000140613c20(uStack_108,lStack_110,1);
        }
        uVar12 = (undefined1)uStack_c0;
      }
      uVar11 = *(undefined4 *)((longlong)param_1 + 0x1c);
      puVar14 = (undefined4 *)param_1[2];
      *puVar14 = uVar11;
      plVar10 = (longlong *)*param_1;
      FUN_140d7bd90(&uStack_b0,&uStack_e8);
      uStack_88 = CONCAT31(uStack_88._1_3_,uVar12);
      uStack_98 = uStack_d0;
      uStack_94 = uStack_cc;
      uStack_90 = uStack_c8;
      uStack_8c = uStack_c4;
      if (*plVar10 != 0) {
        func_0x000140613c20(plVar10[1],*plVar10,1);
        uVar12 = (undefined1)uStack_c0;
      }
      *(undefined4 *)(plVar10 + 4) = uStack_90;
      *(undefined4 *)((longlong)plVar10 + 0x24) = uStack_8c;
      *(uint *)(plVar10 + 5) = uStack_88;
      *(uint *)((longlong)plVar10 + 0x2c) = CONCAT13(uStack_81,CONCAT21(uStack_83,uStack_84));
      *(undefined4 *)(plVar10 + 2) = uStack_a0;
      *(undefined4 *)((longlong)plVar10 + 0x14) = uStack_9c;
      *(undefined4 *)(plVar10 + 3) = uStack_98;
      *(undefined4 *)((longlong)plVar10 + 0x1c) = uStack_94;
      *(undefined4 *)plVar10 = uStack_b0;
      *(undefined4 *)((longlong)plVar10 + 4) = uStack_ac;
      *(undefined4 *)(plVar10 + 1) = uStack_a8;
      *(undefined4 *)((longlong)plVar10 + 0xc) = uStack_a4;
      *(undefined8 *)((longlong)plVar10 + 0x36) = 0;
      plVar10[6] = 0;
      FUN_140d7bd90(&uStack_b0,&uStack_e8);
      uStack_98 = uStack_d0;
      uStack_94 = uStack_cc;
      uStack_90 = uStack_c8;
      uStack_8c = uStack_c4;
      lVar15 = plVar10[8];
      uStack_88._0_1_ = uVar12;
      if (lVar15 != 0) {
        func_0x000140613c20(plVar10[9],lVar15,1);
      }
      plVar10[0xc] = CONCAT44(uStack_8c,uStack_90);
      *(undefined1 *)(plVar10 + 0xd) = (undefined1)uStack_88;
      *(uint *)((longlong)plVar10 + 0x69) = CONCAT13(uStack_84,uStack_88._1_3_);
      *(undefined2 *)((longlong)plVar10 + 0x6d) = uStack_83;
      *(undefined1 *)((longlong)plVar10 + 0x6f) = uStack_81;
      *(undefined4 *)(plVar10 + 10) = uStack_a0;
      *(undefined4 *)((longlong)plVar10 + 0x54) = uStack_9c;
      *(undefined4 *)(plVar10 + 0xb) = uStack_98;
      *(undefined4 *)((longlong)plVar10 + 0x5c) = uStack_94;
      *(undefined4 *)(plVar10 + 8) = uStack_b0;
      *(undefined4 *)((longlong)plVar10 + 0x44) = uStack_ac;
      *(undefined4 *)(plVar10 + 9) = uStack_a8;
      *(undefined4 *)((longlong)plVar10 + 0x4c) = uStack_a4;
      *(undefined8 *)((longlong)plVar10 + 0x76) = 0;
      plVar10[0xe] = 0;
      uVar12 = (undefined1)uStack_c0;
      FUN_140d7bd90(&uStack_b0,&uStack_e8);
      uStack_88 = CONCAT31(uStack_88._1_3_,uVar12);
      uStack_98 = uStack_d0;
      uStack_94 = uStack_cc;
      uStack_90 = uStack_c8;
      uStack_8c = uStack_c4;
      lVar15 = plVar10[0x10];
      if (lVar15 != 0) {
        func_0x000140613c20(plVar10[0x11],lVar15,1);
      }
      plVar10[0x14] = CONCAT44(uStack_8c,uStack_90);
      *(undefined1 *)(plVar10 + 0x15) = (undefined1)uStack_88;
      *(uint *)((longlong)plVar10 + 0xa9) = CONCAT13(uStack_84,uStack_88._1_3_);
      *(undefined2 *)((longlong)plVar10 + 0xad) = uStack_83;
      *(undefined1 *)((longlong)plVar10 + 0xaf) = uStack_81;
      *(undefined4 *)(plVar10 + 0x12) = uStack_a0;
      *(undefined4 *)((longlong)plVar10 + 0x94) = uStack_9c;
      *(undefined4 *)(plVar10 + 0x13) = uStack_98;
      *(undefined4 *)((longlong)plVar10 + 0x9c) = uStack_94;
      *(undefined4 *)(plVar10 + 0x10) = uStack_b0;
      *(undefined4 *)((longlong)plVar10 + 0x84) = uStack_ac;
      *(undefined4 *)(plVar10 + 0x11) = uStack_a8;
      *(undefined4 *)((longlong)plVar10 + 0x8c) = uStack_a4;
      *(undefined8 *)((longlong)plVar10 + 0xb6) = 0;
      plVar10[0x16] = 0;
      if (uStack_e8 != 0) {
        func_0x000140613c20(uStack_e0,uStack_e8,1);
      }
      goto code_r0x0001408bff1d;
    }
    pcVar2 = (char *)(*(longlong *)(lStack_b8 + 8) + lVar15);
    lVar15 = lVar15 + 1;
  } while (*pcVar2 != '\0');
  uVar11 = *(undefined4 *)((longlong)param_1 + 0x1c);
  puVar14 = (undefined4 *)param_1[2];
  *puVar14 = uVar11;
  plVar10 = (longlong *)*param_1;
  puVar6 = (undefined8 *)func_0x000140613c10(0x16,1);
  uVar8 = _UNK_142aa0e08;
  uVar7 = _UNK_142aa0e00;
  if (puVar6 != (undefined8 *)0x0) {
    *puVar6 = _UNK_142aa0e00;
    puVar6[1] = uVar8;
    *(undefined8 *)((longlong)puVar6 + 0xe) = 0x315f3930375f616d;
    if (*plVar10 != 0) {
      func_0x000140613c20(plVar10[1],*plVar10,1);
    }
    *plVar10 = 0x16;
    plVar10[1] = (longlong)puVar6;
    plVar10[2] = 0x16;
    lVar1 = _UNK_142aa0198;
    lVar15 = _UNK_142aa0190;
    plVar10[3] = _UNK_142aa0190;
    plVar10[4] = lVar1;
    *(undefined1 *)(plVar10 + 5) = 0;
    plVar10[6] = 0;
    *(undefined8 *)((longlong)plVar10 + 0x36) = 0;
    puVar6 = (undefined8 *)func_0x000140613c10(0x16,1);
    if (puVar6 != (undefined8 *)0x0) {
      *puVar6 = uVar7;
      puVar6[1] = uVar8;
      *(undefined8 *)((longlong)puVar6 + 0xe) = 0x315f3930375f616d;
      if (plVar10[8] != 0) {
        func_0x000140613c20(plVar10[9],plVar10[8],1);
      }
      plVar10[8] = 0x16;
      plVar10[9] = (longlong)puVar6;
      plVar10[10] = 0x16;
      plVar10[0xb] = lVar15;
      plVar10[0xc] = lVar1;
      *(undefined1 *)(plVar10 + 0xd) = 0;
      plVar10[0xe] = 0;
      *(undefined8 *)((longlong)plVar10 + 0x76) = 0;
      puVar6 = (undefined8 *)func_0x000140613c10(0x16,1);
      if (puVar6 != (undefined8 *)0x0) {
        *puVar6 = uVar7;
        puVar6[1] = uVar8;
        *(undefined8 *)((longlong)puVar6 + 0xe) = 0x315f3930375f616d;
        if (plVar10[0x10] != 0) {
          func_0x000140613c20(plVar10[0x11],plVar10[0x10],1);
        }
        plVar10[0x10] = 0x16;
        plVar10[0x11] = (longlong)puVar6;
        plVar10[0x12] = 0x16;
        plVar10[0x13] = lVar15;
        plVar10[0x14] = lVar1;
        *(undefined1 *)(plVar10 + 0x15) = 0;
        plVar10[0x16] = 0;
        *(undefined8 *)((longlong)plVar10 + 0xb6) = 0;
code_r0x0001408bff1d:
        if (!SBORROW8(0,*(longlong *)(lStack_b8 + 0x18))) {
          puStack_80 = (undefined8 *)(lStack_b8 + 0x18);
          *puVar14 = uVar11;
          lStack_78 = 0;
          lVar15 = 0;
          do {
            lVar5 = lStack_b8;
            lVar1 = lStack_78 + 0x40;
            lVar13 = lStack_78;
            if (lVar15 == 0) {
              if (lVar1 == 0xc0) {
                return;
              }
              lVar15 = 1;
              lVar13 = lVar1;
              lVar1 = lStack_78 + 0x80;
            }
            lStack_78 = lVar1;
            puVar14 = (undefined4 *)((longlong)plVar10 + lVar13);
            uVar12 = *(undefined1 *)(lStack_b8 + 0x40);
            FUN_140d7bd90(&uStack_b0,puStack_80);
            uStack_88 = CONCAT31(uStack_88._1_3_,uVar12);
            uStack_98 = *(undefined4 *)(lVar5 + 0x30);
            uStack_94 = *(undefined4 *)(lVar5 + 0x34);
            uStack_90 = *(undefined4 *)(lVar5 + 0x38);
            uStack_8c = *(undefined4 *)(lVar5 + 0x3c);
            uVar7 = *(undefined8 *)(lVar5 + 0x48);
            uVar11 = *(undefined4 *)(lVar5 + 0x50);
            uVar12 = *(undefined1 *)(lVar5 + 0x54);
            uVar3 = *(undefined1 *)(lVar5 + 0x55);
            if (*(longlong *)((longlong)plVar10 + lVar13) != 0) {
              func_0x000140613c20(*(undefined8 *)(puVar14 + 2),
                                  *(longlong *)((longlong)plVar10 + lVar13),1);
            }
            lVar15 = lVar15 + 1;
            *(ulonglong *)(puVar14 + 8) = CONCAT44(uStack_8c,uStack_90);
            *(ulonglong *)(puVar14 + 10) =
                 CONCAT17(uStack_81,CONCAT25(uStack_83,CONCAT14(uStack_84,uStack_88)));
            *(ulonglong *)(puVar14 + 4) = CONCAT44(uStack_9c,uStack_a0);
            *(ulonglong *)(puVar14 + 6) = CONCAT44(uStack_94,uStack_98);
            *puVar14 = uStack_b0;
            puVar14[1] = uStack_ac;
            puVar14[2] = uStack_a8;
            puVar14[3] = uStack_a4;
            *(undefined8 *)(puVar14 + 0xc) = uVar7;
            puVar14[0xe] = uVar11;
            *(undefined1 *)(puVar14 + 0xf) = uVar12;
            *(undefined1 *)((longlong)puVar14 + 0x3d) = uVar3;
          } while (lStack_78 != 0xc0);
        }
        return;
      }
    }
  }
  FUN_1428d8fe3(1,0x16,&UNK_142aa0520);
code_r0x0001408c004f:
  FUN_1428d8fe3(1,7,&UNK_142ae67e0);
code_r0x0001408c0065:
  FUN_1428d9760(&UNK_142aa03b8,0x2b,&uStack_e8,&UNK_142aa0468,&UNK_142aa0de8);
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

