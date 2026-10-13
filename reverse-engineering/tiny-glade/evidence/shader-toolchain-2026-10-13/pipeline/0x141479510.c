
ulonglong FUN_141479510(undefined8 *param_1,undefined8 *param_2,int *param_3)

{
  uint uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  ulonglong uVar4;
  longlong *plVar5;
  undefined4 *puVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  code *pcVar9;
  ulonglong uVar10;
  int iVar11;
  undefined8 uVar12;
  uint *puVar13;
  ulonglong uVar14;
  undefined8 unaff_R14;
  undefined1 auStack_bc [12];
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  int iStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  longlong lStack_90;
  undefined8 *puStack_88;
  ulonglong uStack_80;
  ulonglong uStack_78;
  undefined8 *puStack_70;
  int *piStack_68;
  uint uStack_5c;
  ulonglong uStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar14 = param_1[1];
  uVar4 = param_1[2];
  if (uVar14 < uVar4) {
    puStack_70 = (undefined8 *)*param_1;
    uStack_98 = *param_2;
    lStack_90 = -uVar4;
    uVar10 = CONCAT71((int7)((ulonglong)unaff_R14 >> 8),1);
    puStack_88 = param_1;
    piStack_68 = param_3;
    do {
      uVar14 = uVar14 + 1;
      uStack_5c = (uint)uVar10;
      puStack_88[1] = uVar14;
      plVar5 = (longlong *)*puStack_70;
      puVar13 = (uint *)*plVar5;
      uVar12 = 7;
      if ((ulonglong)(plVar5[1] - (longlong)puVar13) < 4) {
LAB_1414796cc:
        uVar14 = uStack_78 >> 0x20;
        puStack_70 = (undefined8 *)uVar12;
LAB_1414796d8:
        uVar10 = (ulonglong)uStack_5c;
        uStack_58 = uStack_80 >> 0x20;
        uStack_50 = uVar14;
        if (*piStack_68 == 0xf) {
          FUN_1405e7a00(piStack_68 + 2);
        }
        *(undefined8 **)piStack_68 = puStack_70;
        piStack_68[2] = (int)uStack_78;
        piStack_68[3] = (int)uStack_50;
        piStack_68[4] = (int)uStack_80;
        piStack_68[5] = (int)uStack_58;
        break;
      }
      uVar1 = *puVar13;
      uStack_50 = (ulonglong)uVar1;
      puVar13 = puVar13 + 1;
      *plVar5 = (longlong)puVar13;
      if ((ulonglong)(plVar5[1] - (longlong)puVar13) < uStack_50) goto LAB_1414796cc;
      if (uVar1 == 0) {
        uVar10 = 1;
      }
      else {
        uVar10 = func_0x000140613c10(uStack_50,1);
        if (uVar10 == 0) {
          FUN_1428d8fe3(1,uStack_50,&UNK_142c65160);
          pcVar9 = (code *)swi(3);
          uVar14 = (*pcVar9)();
          return uVar14;
        }
      }
      uVar7 = uStack_50;
      func_0x000142923c00(uVar10,puVar13,uStack_50);
      *plVar5 = (longlong)puVar13 + uVar7;
      uStack_58 = uVar10;
      FUN_140d88f60(&uStack_b0,uVar10,uVar7);
      iVar11 = (int)uStack_50;
      if ((int)uStack_b0 == 1) {
        uVar12 = 2;
        if (iVar11 != 0) {
          func_0x000140613c20(uStack_58,uStack_50,1);
        }
        goto LAB_1414796cc;
      }
      puVar6 = (undefined4 *)*plVar5;
      if ((ulonglong)(plVar5[1] - (longlong)puVar6) < 4) {
LAB_141479744:
        uVar14 = 7;
        if (iVar11 == 0) {
          puStack_70 = (undefined8 *)0x7;
          uStack_78 = uVar14;
        }
        else {
          puStack_70 = (undefined8 *)0x7;
          uVar14 = func_0x000140613c20(uStack_58,uStack_50,1);
          uStack_78 = uVar14;
        }
        goto LAB_1414796d8;
      }
      uVar2 = *puVar6;
      *plVar5 = (longlong)(puVar6 + 1);
      if ((ulonglong)(plVar5[1] - (longlong)(puVar6 + 1)) < 4) goto LAB_141479744;
      uVar3 = puVar6[1];
      *plVar5 = (longlong)(puVar6 + 2);
      uVar7 = uStack_58;
      uStack_a8 = uStack_58;
      uVar8 = uStack_50;
      uStack_9c = 0;
      uStack_b0 = uStack_50;
      iStack_a0 = iVar11;
      FUN_141525a20(auStack_bc,uStack_98,&uStack_b0,uVar2,uVar3);
      uVar10 = CONCAT71((int7)((ulonglong)plVar5 >> 8),uVar14 < uVar4);
      uStack_80 = uVar8;
      uStack_78 = uVar7;
    } while (lStack_90 + uVar14 != 0);
  }
  else {
    uVar10 = 0;
  }
  return uVar10 & 0xffffff01;
}

