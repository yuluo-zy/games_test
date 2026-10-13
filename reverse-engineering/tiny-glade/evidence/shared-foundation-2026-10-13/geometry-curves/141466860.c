
longlong * FUN_141466860(longlong *param_1,longlong *param_2,undefined8 param_3,ulonglong param_4)

{
  ulonglong uVar1;
  longlong lVar2;
  code *pcVar3;
  longlong *plVar4;
  undefined8 uVar5;
  longlong lVar6;
  ulonglong uStack_b0;
  longlong lStack_a8;
  ulonglong uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  
  lVar6 = param_2[2];
  if ((ulonglong)(*param_2 - lVar6) < param_4) {
    FUN_1428a9a60(param_2,lVar6,param_4,4,8);
    lVar6 = param_2[2];
  }
  lVar2 = param_2[1];
  func_0x000142923c00(lVar2 + lVar6 * 8,param_3,param_4 * 8);
  param_4 = lVar6 + param_4;
  param_2[2] = param_4;
  uVar1 = param_4 * 8;
  uVar5 = 0;
  if ((param_4 >> 0x3d == 0) && (uVar1 < 0x7ffffffffffffffd)) {
    if (uVar1 == 0) {
      lStack_a8 = 4;
      uStack_b0 = 0;
    }
    else {
      uVar5 = 4;
      lStack_a8 = func_0x000140613c10(uVar1,4);
      uStack_b0 = param_4;
      if (lStack_a8 == 0) goto LAB_141466a24;
    }
    func_0x000142923c00(lStack_a8,lVar2,uVar1);
    uStack_a0 = param_4;
    FUN_141469430(&lStack_70,&uStack_b0,1);
    if (SBORROW8(0,lStack_70)) {
      *param_1 = lStack_68;
      param_1[1] = lStack_60;
    }
    else {
      uStack_88 = uStack_48;
      uStack_84 = uStack_44;
      uStack_80 = uStack_40;
      uStack_7c = uStack_3c;
      uStack_98 = uStack_58;
      uStack_94 = uStack_54;
      uStack_90 = uStack_50;
      uStack_8c = uStack_4c;
      if (*param_2 != 0) {
        func_0x000140613c20(lVar2,*param_2 << 3,4);
      }
      if (param_2[3] != 0) {
        func_0x000140613c20(param_2[4],param_2[3] << 2,4);
      }
      *param_2 = lStack_70;
      param_2[1] = lStack_68;
      param_2[2] = lStack_60;
      param_2[5] = CONCAT44(uStack_84,uStack_88);
      param_2[6] = CONCAT44(uStack_7c,uStack_80);
      param_2[3] = CONCAT44(uStack_94,uStack_98);
      param_2[4] = CONCAT44(uStack_8c,uStack_90);
      *(undefined1 *)param_1 = 3;
    }
    return param_1;
  }
LAB_141466a24:
  FUN_1428d8fe3(uVar5,uVar1,&UNK_142c38b78);
  pcVar3 = (code *)swi(3);
  plVar4 = (longlong *)(*pcVar3)();
  return plVar4;
}

