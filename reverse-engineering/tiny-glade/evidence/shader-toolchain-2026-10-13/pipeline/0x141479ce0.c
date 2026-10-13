
ulonglong FUN_141479ce0(undefined8 *param_1,undefined8 *param_2,int *param_3,undefined1 *param_4)

{
  uint uVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  code *pcVar6;
  undefined1 *puVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  undefined1 *puVar10;
  ulonglong uVar11;
  uint *puVar12;
  undefined1 auStack_120 [8];
  undefined8 uStack_118;
  uint uStack_110;
  uint uStack_10c;
  undefined1 *puStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined1 *puStack_e8;
  undefined1 *puStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  int iStack_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  uint uStack_c0;
  uint uStack_bc;
  undefined1 *puStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_98;
  longlong lStack_90;
  undefined8 *puStack_88;
  uint uStack_7c;
  undefined1 *puStack_78;
  undefined1 *puStack_70;
  int *piStack_68;
  undefined8 *puStack_60;
  undefined1 *puStack_58;
  undefined1 *puStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar11 = param_1[1];
  uVar3 = param_1[2];
  if (uVar11 < uVar3) {
    puStack_60 = (undefined8 *)*param_1;
    uStack_98 = *param_2;
    lStack_90 = -uVar3;
    uVar8 = CONCAT71((int7)((ulonglong)param_2 >> 8),1);
    puStack_88 = param_1;
    piStack_68 = param_3;
    do {
      uVar11 = uVar11 + 1;
      uStack_7c = (uint)uVar8;
      puStack_88[1] = uVar11;
      puVar4 = (undefined8 *)*puStack_60;
      puVar12 = (uint *)*puVar4;
      uVar9 = 7;
      if ((ulonglong)(puVar4[1] - (longlong)puVar12) < 4) {
LAB_141479f03:
        puStack_60 = (undefined8 *)((ulonglong)puStack_70 >> 0x20);
        puStack_50 = (undefined1 *)uVar9;
LAB_141479f3c:
        if (*piStack_68 == 0xf) {
          FUN_1405e7a00(piStack_68 + 2);
        }
        *(undefined1 **)piStack_68 = puStack_50;
        piStack_68[2] = (int)puStack_70;
        piStack_68[3] = (int)puStack_60;
        *(undefined1 **)(piStack_68 + 4) = puStack_78;
        uVar8 = (ulonglong)uStack_7c;
        break;
      }
      uVar1 = *puVar12;
      puVar7 = (undefined1 *)(ulonglong)uVar1;
      puVar12 = puVar12 + 1;
      *puVar4 = puVar12;
      if ((undefined1 *)(puVar4[1] - (longlong)puVar12) < puVar7) goto LAB_141479f03;
      puStack_58 = puVar7;
      if (uVar1 == 0) {
        puVar7 = (undefined1 *)0x1;
      }
      else {
        puVar7 = (undefined1 *)func_0x000140613c10(puVar7,1);
        if (puVar7 == (undefined1 *)0x0) {
          FUN_1428d8fe3(1,puStack_58,&UNK_142c65160);
          pcVar6 = (code *)swi(3);
          uVar11 = (*pcVar6)();
          return uVar11;
        }
      }
      puVar10 = puStack_58;
      func_0x000142923c00(puVar7,puVar12,puStack_58);
      *puVar4 = (undefined1 *)((longlong)puVar12 + (longlong)puVar10);
      puStack_50 = puVar7;
      FUN_140d88f60(&iStack_d0,puVar7);
      if (iStack_d0 == 1) {
        uVar9 = 2;
        if ((int)puStack_58 != 0) {
          func_0x000140613c20(puStack_50,puStack_58,1);
        }
        goto LAB_141479f03;
      }
      puVar5 = (undefined8 *)*puVar4;
      if ((ulonglong)(puVar4[1] - (longlong)puVar5) < 4) {
        uStack_c8 = 7;
        puStack_b8 = param_4;
LAB_141479f19:
        puStack_60 = puVar5;
        puVar7 = puStack_50;
        puStack_78 = puStack_b8;
        puStack_70 = puVar10;
        puStack_50 = (undefined1 *)uStack_c8;
        if ((int)puStack_58 != 0) {
          puStack_50 = (undefined1 *)uStack_c8;
          func_0x000140613c20(puVar7,puStack_58,1);
        }
        goto LAB_141479f3c;
      }
      uVar2 = *(undefined4 *)puVar5;
      *puVar4 = (undefined4 *)((longlong)puVar5 + 4);
      FUN_14147a7e0(&iStack_d0,puVar4);
      puVar7 = puStack_50;
      puVar10 = (undefined1 *)(ulonglong)uStack_c0;
      puVar5 = (undefined8 *)(ulonglong)uStack_bc;
      if (CONCAT44(uStack_cc,iStack_d0) == -0x7ffffffffffffff2) goto LAB_141479f19;
      uStack_100 = uStack_b0;
      uStack_fc = uStack_ac;
      uStack_f8 = uStack_a8;
      uStack_f4 = uStack_a4;
      puStack_e8 = puStack_58;
      puStack_e0 = puStack_50;
      uStack_d8 = SUB84(puStack_58,0);
      uStack_d4 = 0;
      uStack_118 = uStack_c8;
      uStack_110 = uStack_c0;
      uStack_10c = uStack_bc;
      puStack_108 = puStack_b8;
      param_4 = auStack_120;
      uVar9 = uStack_98;
      uStack_f0 = uVar2;
      FUN_14152be10(&iStack_d0,uStack_98,&puStack_e8);
      if (CONCAT44(uStack_cc,iStack_d0) != -0x7ffffffffffffff2) {
        FUN_140889430(&iStack_d0);
      }
      uVar8 = CONCAT71((int7)((ulonglong)uVar9 >> 8),uVar11 < uVar3);
      puStack_78 = puStack_58;
      puStack_70 = puVar7;
    } while (lStack_90 + uVar11 != 0);
  }
  else {
    uVar8 = 0;
  }
  return uVar8 & 0xffffff01;
}

