
ulonglong FUN_14147a290(undefined8 *param_1,undefined8 *param_2,int *param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  longlong *plVar6;
  undefined8 *puVar7;
  longlong lVar8;
  undefined8 unaff_RSI;
  ulonglong uVar9;
  undefined8 uVar11;
  ulonglong uVar12;
  undefined1 auStack_5c [12];
  int *piStack_50;
  undefined8 uStack_48;
  undefined7 uVar10;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar3 = param_1[2];
  if ((ulonglong)param_1[1] < uVar3) {
    puVar4 = (undefined8 *)*param_1;
    uVar5 = *param_2;
    uVar12 = param_1[1] + 1;
    param_1[1] = uVar12;
    plVar6 = (longlong *)*puVar4;
    puVar7 = (undefined8 *)*plVar6;
    lVar8 = plVar6[1];
    uVar10 = (undefined7)((ulonglong)unaff_RSI >> 8);
    uVar9 = CONCAT71(uVar10,1);
    piStack_50 = param_3;
    if (7 < (ulonglong)(lVar8 - (longlong)puVar7)) {
      uVar11 = *puVar7;
      *plVar6 = (longlong)(puVar7 + 1);
      if (3 < (ulonglong)(lVar8 - (longlong)(puVar7 + 1))) {
        uVar9 = CONCAT71(uVar10,1);
        do {
          uVar1 = *(undefined4 *)(puVar7 + 1);
          *plVar6 = (longlong)puVar7 + 0xc;
          if ((ulonglong)(lVar8 - ((longlong)puVar7 + 0xc)) < 4) break;
          uVar2 = *(undefined4 *)((longlong)puVar7 + 0xc);
          *plVar6 = (longlong)(puVar7 + 2);
          FUN_1415271e0(auStack_5c,uVar5,uVar11,uVar1,uVar2);
          uVar9 = CONCAT71((int7)(uVar9 >> 8),uVar12 < uVar3);
          if (uVar12 == uVar3) goto LAB_14147a3ae;
          uVar12 = uVar12 + 1;
          param_1[1] = uVar12;
          plVar6 = (longlong *)*puVar4;
          puVar7 = (undefined8 *)*plVar6;
          lVar8 = plVar6[1];
          if ((ulonglong)(lVar8 - (longlong)puVar7) < 8) break;
          uVar11 = *puVar7;
          *plVar6 = (longlong)(puVar7 + 1);
        } while (3 < (ulonglong)(lVar8 - (longlong)(puVar7 + 1)));
      }
    }
    if (*piStack_50 == 0xf) {
      FUN_1405e7a00(piStack_50 + 2);
    }
    piStack_50[0] = 7;
    piStack_50[1] = 0;
    piStack_50[2] = 0;
    piStack_50[3] = 0;
    piStack_50[5] = 0;
  }
  else {
    uVar9 = 0;
  }
LAB_14147a3ae:
  return uVar9 & 0xffffff01;
}

