
undefined8 * FUN_140bfef70(undefined8 *param_1,longlong param_2,longlong param_3,longlong param_4)

{
  longlong lVar1;
  int *piVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  int iVar5;
  code *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  undefined4 *puVar10;
  int iVar11;
  undefined8 *puVar12;
  ulonglong uVar13;
  undefined4 *puVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined4 auStack_108 [2];
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 *puStack_f0;
  undefined8 uStack_e8;
  longlong lStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  longlong lStack_c8;
  undefined1 auStack_c0 [8];
  int iStack_b8;
  undefined4 uStack_b4;
  undefined4 *puStack_b0;
  longlong lStack_a8;
  undefined4 uStack_a0;
  undefined1 *puStack_98;
  longlong lStack_90;
  undefined8 *puStack_88;
  undefined4 *puStack_80;
  int iStack_78;
  undefined4 uStack_74;
  undefined8 *puStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_88 = param_1;
  puVar12 = (undefined8 *)func_0x000140613c10(0x58,8);
  if (puVar12 == (undefined8 *)0x0) {
    iVar11 = FUN_1428d9000(8,0x58);
  }
  else {
    *puVar12 = 0;
    *(undefined4 *)(puVar12 + 10) = 0;
    lStack_e0 = param_3 + param_4 * 0x28;
    uStack_d8 = 0;
    uStack_e8 = param_3;
    lStack_90 = param_2;
    puStack_70 = puVar12;
    FUN_1418c2150(&iStack_b8,param_2,&uStack_e8,0x7fffffff,&puStack_70);
    lStack_68 = CONCAT44(uStack_b4,iStack_b8);
    lStack_60 = 0;
    lStack_58 = 4;
    lStack_50 = 0;
    puVar14 = puStack_b0 + lStack_a8 * 6;
    puVar10 = puStack_b0;
    lStack_c8 = lStack_58;
    lVar15 = lStack_50;
    puStack_80 = puStack_b0;
    lVar9 = lStack_90;
joined_r0x000140bff058:
    lStack_90 = lVar9;
    lStack_58 = lStack_c8;
    lStack_50 = lVar15;
    if (puVar10 != puVar14) {
      iVar11 = puVar10[1];
      iVar5 = puVar10[2];
      lVar17 = 0;
      do {
        if (lVar15 * 8 - lVar17 == 0) {
          if (lVar15 == lStack_60) {
            FUN_140562e70(&lStack_60);
          }
          *(int *)(lStack_58 + lVar15 * 8) = iVar11;
          *(int *)(lStack_58 + 4 + lVar15 * 8) = iVar5;
          lVar15 = lVar15 + 1;
          puVar10 = puVar10 + 6;
          lStack_c8 = lStack_58;
          lVar9 = lStack_90;
          goto joined_r0x000140bff058;
        }
        lVar1 = lVar17 + 8;
        piVar2 = (int *)(lStack_c8 + lVar17);
        lVar17 = lVar1;
      } while (*piVar2 != iVar11);
      piVar2 = (int *)(lStack_c8 + -4 + lVar1);
      *piVar2 = *piVar2 + iVar5;
      puVar10 = puVar10 + 6;
      goto joined_r0x000140bff058;
    }
    lStack_50._0_4_ = (undefined4)lVar15;
    uVar16 = (ulonglong)uStack_e8 >> 0x20;
    uStack_e8 = CONCAT44((int)uVar16,0x21);
    lStack_e0 = 0;
    uStack_d8 = 0x100000002;
    uStack_d0 = (undefined4)lStack_50;
    iVar11 = (**(code **)(lVar9 + 0x240))(*(undefined8 *)(lVar9 + 0x640),&uStack_e8,0,&iStack_b8);
    if (iVar11 == 0) {
      uVar13 = (ulonglong)*(uint *)(*(longlong *)(lVar9 + 0x658) + 0x680);
      uVar16 = 0;
      if (0x1f < uVar13) {
        uVar16 = uVar13 - 0x20;
      }
      uVar13 = 0x2000;
      if (uVar16 < 0x2000) {
        uVar13 = uVar16;
      }
      uVar8 = CONCAT44(uStack_b4,iStack_b8);
      uStack_74 = (int)uVar13;
      auStack_108[0] = 0x3b9d3eeb;
      uStack_100 = 0;
      uStack_f8 = 1;
      puStack_f0 = &uStack_74;
      puVar14 = auStack_108;
      do {
        puVar14 = *(undefined4 **)(puVar14 + 2);
      } while (puVar14 != (undefined4 *)0x0);
      iStack_b8 = 0x22;
      puStack_b0 = auStack_108;
      uStack_a0 = 1;
      puStack_98 = auStack_c0;
      pcVar6 = *(code **)(lVar9 + 600);
      uVar7 = *(undefined8 *)(lVar9 + 0x640);
      puVar12 = (undefined8 *)func_0x000140613c10(8,8);
      if (puVar12 == (undefined8 *)0x0) {
        FUN_1428d8fe3(8,8,&UNK_142b14f70);
      }
      else {
        iVar11 = (*pcVar6)(uVar7,&iStack_b8,puVar12);
        if (iVar11 == 0) {
          uVar7 = *puVar12;
          func_0x000140613c20(puVar12,8,8);
          puVar12 = *(undefined8 **)(lStack_90 + 0x648);
          lVar15 = FUN_1418aead0(puVar12 + 0xec,uVar8,uVar7);
          uVar4 = *(undefined4 *)(lVar15 + 0xc);
          uVar3 = *(undefined2 *)(puVar12 + 0x14a);
          *puStack_88 = *puVar12;
          puStack_88[1] = lVar15;
          *(undefined4 *)(puStack_88 + 2) = uVar4;
          *(undefined2 *)((longlong)puStack_88 + 0x14) = uVar3;
          *(undefined1 *)((longlong)puStack_88 + 0x16) = 6;
          if (lStack_60 != 0) {
            func_0x000140613c20(lStack_58,lStack_60 << 3,4);
          }
          if (lStack_68 != 0) {
            func_0x000140613c20(puStack_80,lStack_68 * 0x18,8);
          }
          FUN_140bde2a0(&puStack_70);
          return puStack_88;
        }
        func_0x000140613c20(puVar12,8,8);
        iStack_78 = iVar11;
        FUN_1428d9760(&UNK_142b15188,0x2b,&iStack_78,&UNK_142b15128,&UNK_142b17218);
      }
      goto code_r0x000140bff34d;
    }
  }
  iStack_b8 = iVar11;
  FUN_1428d9760(&UNK_142b15188,0x2b,&iStack_b8,&UNK_142b15128,&UNK_142b17200);
code_r0x000140bff34d:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

