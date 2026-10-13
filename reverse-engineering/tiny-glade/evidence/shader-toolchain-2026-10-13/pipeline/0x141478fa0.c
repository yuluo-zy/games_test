
ulonglong FUN_141478fa0(undefined8 *param_1,undefined8 *param_2,int *param_3)

{
  uint uVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  longlong *plVar4;
  undefined4 *puVar5;
  code *pcVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong *puVar9;
  int iVar10;
  int *piVar11;
  ulonglong uVar12;
  uint *puVar13;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  int iStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  longlong lStack_90;
  undefined8 *puStack_88;
  uint uStack_7c;
  ulonglong uStack_78;
  ulonglong uStack_70;
  int *piStack_68;
  ulonglong uStack_60;
  undefined8 *puStack_58;
  int iStack_4c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar12 = param_1[1];
  uVar3 = param_1[2];
  if (uVar12 < uVar3) {
    puStack_58 = (undefined8 *)*param_1;
    uStack_98 = *param_2;
    lStack_90 = -uVar3;
    uVar8 = CONCAT71((int7)((ulonglong)param_2 >> 8),1);
    puStack_88 = param_1;
    piStack_68 = param_3;
    do {
      uVar12 = uVar12 + 1;
      uStack_7c = (uint)uVar8;
      puStack_88[1] = uVar12;
      plVar4 = (longlong *)*puStack_58;
      puVar13 = (uint *)*plVar4;
      if ((ulonglong)(plVar4[1] - (longlong)puVar13) < 4) {
LAB_1414791a2:
        puStack_58 = (undefined8 *)0x7;
        piVar11 = piStack_68;
LAB_141479142:
        uStack_60 = uStack_70 >> 0x20;
        if (*piVar11 == 0xf) {
          FUN_1405e7a00(piStack_68 + 2);
        }
        *(undefined8 **)piStack_68 = puStack_58;
        piStack_68[2] = (int)uStack_70;
        piStack_68[3] = (int)uStack_60;
        piStack_68[4] = iStack_4c;
        piStack_68[5] = 0;
        uVar8 = (ulonglong)uStack_7c;
        break;
      }
      uVar1 = *puVar13;
      uVar8 = (ulonglong)uVar1;
      puVar13 = puVar13 + 1;
      *plVar4 = (longlong)puVar13;
      if ((ulonglong)(plVar4[1] - (longlong)puVar13) < uVar8) goto LAB_1414791a2;
      if (uVar1 == 0) {
        uVar7 = 1;
      }
      else {
        uVar7 = func_0x000140613c10(uVar8,1);
        if (uVar7 == 0) {
          FUN_1428d8fe3(1,uVar8,&UNK_142c65160);
          pcVar6 = (code *)swi(3);
          uVar12 = (*pcVar6)();
          return uVar12;
        }
      }
      func_0x000142923c00(uVar7,puVar13);
      *plVar4 = (longlong)puVar13 + uVar8;
      uStack_78 = uVar7;
      uStack_60 = uVar8;
      FUN_140d88f60(&uStack_b0,uVar7,uVar8);
      piVar11 = piStack_68;
      iVar10 = (int)uStack_60;
      if ((int)uStack_b0 == 1) {
        puStack_58 = (undefined8 *)0x2;
        uVar12 = uStack_70;
        if (iVar10 != 0) {
LAB_141479133:
          func_0x000140613c20(uStack_78,uStack_60,1);
          uStack_70 = uVar12;
        }
        goto LAB_141479142;
      }
      puVar5 = (undefined4 *)*plVar4;
      if ((ulonglong)(plVar4[1] - (longlong)puVar5) < 4) {
        if (iVar10 != 0) {
          puStack_58 = (undefined8 *)0x7;
          uVar12 = uStack_78;
          iStack_4c = iVar10;
          goto LAB_141479133;
        }
        puStack_58 = (undefined8 *)0x7;
        iStack_4c = 0;
        uStack_70 = uStack_78;
        goto LAB_141479142;
      }
      uVar2 = *puVar5;
      *plVar4 = (longlong)(puVar5 + 1);
      uVar7 = uStack_78;
      uStack_a8 = uStack_78;
      uStack_9c = 0;
      uStack_b0 = uStack_60;
      puVar9 = &uStack_b0;
      iStack_a0 = iVar10;
      FUN_14152a5d0(uStack_98,&uStack_b0,uVar2);
      uVar8 = CONCAT71((int7)((ulonglong)puVar9 >> 8),uVar12 < uVar3);
      uStack_70 = uVar7;
      iStack_4c = iVar10;
    } while (lStack_90 + uVar12 != 0);
  }
  else {
    uVar8 = 0;
  }
  return uVar8 & 0xffffff01;
}

