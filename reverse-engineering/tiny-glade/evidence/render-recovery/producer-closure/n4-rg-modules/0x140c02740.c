
longlong *
FUN_140c02740(longlong *param_1,longlong *param_2,ulonglong param_3,ulonglong param_4,
             ulonglong param_5,ulonglong param_6)

{
  int *piVar1;
  undefined1 uVar2;
  undefined2 uVar3;
  longlong lVar4;
  code *pcVar5;
  longlong lVar6;
  longlong lVar7;
  longlong *plVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *unaff_RSI;
  undefined *puVar11;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  longlong lStack_50;
  longlong *plStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  if (param_4 < 0x40001) {
    if (param_6 < 0x11) {
      param_6 = 0x10;
    }
    if ((param_5 & 1) == 0) {
      param_6 = 0x10;
    }
    if (param_6 < (ulonglong)param_2[4]) {
      param_6 = param_2[4];
    }
    uVar9 = -param_6 & (param_2[2] + param_6) - 1;
    param_2[2] = uVar9;
    uVar10 = uVar9 + param_4;
    unaff_RSI = param_2;
    if (uVar10 < 0x40001) {
      lVar7 = *param_2;
      param_3 = param_2[1];
code_r0x000140c0288a:
      if (uVar10 < uVar9) goto code_r0x000140c02917;
      if (uVar10 <= param_3) {
        param_2[2] = uVar10;
        lVar4 = param_2[6];
        LOCK();
        piVar1 = (int *)(lVar4 + 8);
        *piVar1 = *piVar1 + 1;
        UNLOCK();
        lVar6 = param_2[7];
        uVar2 = *(undefined1 *)((longlong)param_2 + 0x3e);
        uVar3 = *(undefined2 *)((longlong)param_2 + 0x3c);
        param_1[4] = param_2[5];
        param_1[5] = lVar4;
        *(int *)(param_1 + 6) = (int)lVar6;
        *(undefined2 *)((longlong)param_1 + 0x34) = uVar3;
        *(undefined1 *)((longlong)param_1 + 0x36) = uVar2;
        *(undefined1 *)(param_1 + 7) = 0;
        lVar4 = param_2[3];
        *param_1 = lVar7 + uVar9;
        param_1[1] = param_4;
        param_1[2] = lVar4 + uVar9;
        *(int *)(param_1 + 8) = (int)uVar9;
        param_1[3] = param_4;
        return param_1;
      }
      goto code_r0x000140c02923;
    }
    FUN_140c029f0(&uStack_70,param_3);
    plStack_40 = param_2 + 5;
    FUN_140be24b0();
    plVar8 = plStack_40;
    plStack_40[4] = lStack_50;
    *(undefined4 *)(plStack_40 + 2) = (undefined4)uStack_60;
    *(undefined4 *)((longlong)plStack_40 + 0x14) = uStack_60._4_4_;
    *(undefined4 *)(plStack_40 + 3) = uStack_58;
    *(undefined4 *)((longlong)plStack_40 + 0x1c) = uStack_54;
    *(undefined4 *)plStack_40 = (undefined4)uStack_70;
    *(undefined4 *)((longlong)plStack_40 + 4) = uStack_70._4_4_;
    *(undefined4 *)(plStack_40 + 1) = (undefined4)uStack_68;
    *(undefined4 *)((longlong)plStack_40 + 0xc) = uStack_68._4_4_;
    param_2[2] = 0;
    puVar11 = *(undefined **)(param_3 + 0x648);
    if (*(short *)((longlong)param_2 + 0x3c) != *(short *)(puVar11 + 0xa50))
    goto code_r0x000140c02935;
    if (*(char *)((longlong)param_2 + 0x3e) != '\x01') goto code_r0x000140c02962;
    if (*(int *)(param_2[6] + 0xc) == (int)param_2[7]) {
      uStack_60 = *(undefined8 *)(param_2[6] + 0x30);
      uStack_70 = CONCAT44(uStack_70._4_4_,0x3b9e8321);
      uStack_68 = 0;
      lVar7 = (**(code **)(param_3 + 0x508))(*(undefined8 *)(param_3 + 0x640),&uStack_70);
      param_2[3] = lVar7;
      lVar7 = FUN_140bd4cd0(plVar8);
      if (lVar7 == 0) goto code_r0x000140c0299b;
      *param_2 = lVar7;
      param_2[1] = param_3;
      uVar9 = param_2[2];
      uVar10 = uVar9 + param_4;
      goto code_r0x000140c0288a;
    }
  }
  else {
    uVar10 = 0x3d;
    FUN_1428d9430(&UNK_142b17f60,0x3d,&UNK_142b17fa0);
    param_3 = param_4;
code_r0x000140c02917:
    FUN_1428d9850();
code_r0x000140c02923:
    puVar11 = &UNK_142b17fd0;
    FUN_1428d9840(uVar10,param_3,&UNK_142b17fd0);
code_r0x000140c02935:
    uStack_70 = 0;
    param_2 = (longlong *)((longlong)unaff_RSI + 0x3c);
    FUN_1428ca816(0,param_2,puVar11 + 0xa50,&uStack_70,&UNK_142d022a0);
code_r0x000140c02962:
    uStack_70 = 0;
    FUN_1428ca902(0,(longlong)param_2 + 0x3e,&UNK_142d022d9,&uStack_70,&UNK_142d022c0);
  }
  FUN_1428d9310(&UNK_142b0f7d8);
code_r0x000140c0299b:
  FUN_1428d9310(&UNK_142b17fb8);
  pcVar5 = (code *)swi(3);
  plVar8 = (longlong *)(*pcVar5)();
  return plVar8;
}

