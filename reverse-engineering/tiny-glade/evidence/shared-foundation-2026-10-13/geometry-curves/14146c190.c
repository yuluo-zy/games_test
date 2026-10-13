
void FUN_14146c190(longlong *param_1)

{
  longlong lVar1;
  ulonglong uVar2;
  code *pcVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  longlong lStack_78;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  longlong lStack_38;
  
  lVar1 = param_1[1];
  uVar2 = param_1[2];
  if (1 < uVar2) {
    lVar4 = lVar1 + uVar2 * 8;
    if (uVar2 >> 1 == 1) {
      uVar5 = 0;
    }
    else {
      lVar6 = uVar2 - 1;
      uVar5 = 0;
      do {
        uVar7 = *(undefined8 *)(lVar1 + uVar5 * 8);
        *(undefined8 *)(lVar1 + uVar5 * 8) = *(undefined8 *)(lVar1 + lVar6 * 8);
        *(undefined8 *)(lVar1 + lVar6 * 8) = uVar7;
        uVar7 = *(undefined8 *)(lVar1 + 8 + uVar5 * 8);
        *(undefined8 *)(lVar1 + 8 + uVar5 * 8) =
             *(undefined8 *)(lVar4 + (uVar5 ^ 0x1ffffffffffffffe) * 8);
        *(undefined8 *)(lVar4 + (uVar5 ^ 0x1ffffffffffffffe) * 8) = uVar7;
        uVar5 = uVar5 + 2;
        lVar6 = lVar6 + -2;
      } while ((uVar2 >> 1 & 0xfffffffffffffffe) != uVar5);
    }
    if ((uVar2 & 2) != 0) {
      uVar7 = *(undefined8 *)(lVar1 + uVar5 * 8);
      *(undefined8 *)(lVar1 + uVar5 * 8) = *(undefined8 *)(lVar4 + ~uVar5 * 8);
      *(undefined8 *)(lVar4 + ~uVar5 * 8) = uVar7;
    }
  }
  uVar5 = uVar2 * 8;
  uVar7 = 0;
  if ((uVar2 >> 0x3d == 0) && (uVar5 < 0x7ffffffffffffffd)) {
    if (uVar5 == 0) {
      lVar4 = 4;
      uVar8 = 0;
    }
    else {
      uVar7 = 4;
      lVar4 = func_0x000140613c10(uVar5,4);
      uVar8 = uVar2;
      if (lVar4 == 0) goto LAB_14146c374;
    }
    func_0x000142923c00(lVar4,lVar1,uVar5);
    uStack_a8 = uVar8;
    uStack_a0 = lVar4;
    uStack_98 = uVar2;
    FUN_141469430(&uStack_68,&uStack_a8,0);
    if (!SBORROW8(0,CONCAT44(uStack_64,uStack_68))) {
      lStack_78 = lStack_38;
      uStack_88 = uStack_48;
      uStack_84 = uStack_44;
      uStack_80 = uStack_40;
      uStack_7c = uStack_3c;
      uStack_98 = CONCAT44(uStack_54,uStack_58);
      uStack_90 = uStack_50;
      uStack_8c = uStack_4c;
      uStack_a8 = CONCAT44(uStack_64,uStack_68);
      uStack_a0 = CONCAT44(uStack_5c,uStack_60);
      if (*param_1 != 0) {
        func_0x000140613c20(lVar1,*param_1 << 3,4);
      }
      if (param_1[3] != 0) {
        func_0x000140613c20(param_1[4],param_1[3] << 2,4);
      }
      param_1[6] = lStack_78;
      param_1[4] = CONCAT44(uStack_84,uStack_88);
      param_1[5] = CONCAT44(uStack_7c,uStack_80);
      param_1[2] = uStack_98;
      param_1[3] = CONCAT44(uStack_8c,uStack_90);
      *param_1 = uStack_a8;
      param_1[1] = uStack_a0;
      return;
    }
  }
  else {
LAB_14146c374:
    FUN_1428d8fe3(uVar7,uVar5,&UNK_142c38b78);
  }
  FUN_1428d9760(&UNK_142c63340,0x2b,&uStack_a8,&UNK_142c63320,&UNK_142c63b60);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

