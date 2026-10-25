
void FUN_140fac4c0(longlong param_1,undefined8 param_2,longlong param_3)

{
  code *pcVar1;
  int iVar2;
  longlong lVar3;
  undefined1 *puVar4;
  char cVar5;
  undefined8 uVar6;
  longlong lStack_b8;
  undefined8 uStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined1 *puStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined1 uStack_52;
  char cStack_51;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  if (param_3 < 0) {
    uVar6 = 0;
LAB_140fac7a9:
    FUN_1428d8fe3(uVar6,param_3,&UNK_142bb0e30);
    pcVar1 = (code *)swi(3);
    (*pcVar1)();
    return;
  }
  if (param_3 == 0) {
    lVar3 = 1;
  }
  else {
    uVar6 = 1;
    lVar3 = func_0x000140613c10(param_3,1);
    if (lVar3 == 0) goto LAB_140fac7a9;
  }
  func_0x000142923c00(lVar3,param_2);
  lStack_70 = param_3;
  lStack_68 = lVar3;
  lStack_60 = param_3;
  if (((SBORROW8(0,*(longlong *)(param_1 + 0x1e8))) || (*(longlong *)(param_1 + 0x1f8) != param_3))
     || (iVar2 = func_0x000142923bf0(*(undefined8 *)(param_1 + 0x1f0),lVar3), iVar2 != 0)) {
    lStack_b8 = *(longlong *)(param_1 + 0x200) + -0x7fffffffffffffff;
    if (-0x7ffffffffffffffc < *(longlong *)(param_1 + 0x200)) {
      lStack_b8 = 0;
    }
    uStack_b0 = *(undefined8 *)(param_1 + 0x208);
    uStack_a8 = *(ulonglong *)(param_1 + 0x210);
    uStack_98 = *(undefined4 *)(param_1 + 0x220);
    uStack_94 = *(undefined4 *)(param_1 + 0x224);
    uStack_90 = *(undefined4 *)(param_1 + 0x228);
    uStack_8c = *(undefined4 *)(param_1 + 0x22c);
    uStack_a0 = *(ulonglong *)(param_1 + 0x218);
    uStack_52 = 1;
    uStack_88 = &lStack_70;
    uStack_80 = &lStack_b8;
    puStack_78 = &uStack_52;
    lVar3 = *(longlong *)(param_1 + 0x38) + -0x7fffffffffffffff;
    if (-0x7fffffffffffffff < *(longlong *)(param_1 + 0x38)) {
      lVar3 = 0;
    }
    if (lVar3 == 0) {
      lVar3 = param_1 + 0x38;
    }
    else if (lVar3 == 1) {
      lVar3 = *(longlong *)(param_1 + 0x40);
    }
    else {
      cStack_51 = 1;
      lVar3 = FUN_141e385f0(param_1 + 0x40);
    }
    cStack_51 = 1;
    FUN_141e388e0(&uStack_88,lVar3);
    puStack_78 = (undefined1 *)lStack_60;
    uStack_88 = (longlong *)lStack_70;
    uStack_80 = (longlong *)lStack_68;
    lVar3 = *(longlong *)(param_1 + 0x1e8);
    if ((!SBORROW8(0,lVar3)) && (lVar3 != 0)) {
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x1f0),lVar3,1);
    }
    *(undefined1 **)(param_1 + 0x1f8) = puStack_78;
    *(undefined4 *)(param_1 + 0x1e8) = (undefined4)uStack_88;
    *(undefined4 *)(param_1 + 0x1ec) = uStack_88._4_4_;
    *(undefined4 *)(param_1 + 0x1f0) = (undefined4)uStack_80;
    *(undefined4 *)(param_1 + 500) = uStack_80._4_4_;
    cVar5 = '\0';
  }
  else {
    lVar3 = 0;
    if (*(longlong *)(param_1 + 0x38) < -0x7ffffffffffffffe) {
      lVar3 = *(longlong *)(param_1 + 0x38) + -0x7fffffffffffffff;
    }
    if (lVar3 == 0) {
      if ((*(longlong *)(param_1 + 0x158) == 0) && (cVar5 = '\x01', *(int *)(param_1 + 0x160) == 0))
      goto LAB_140fac669;
      *(undefined8 *)(param_1 + 0x158) = 0;
      *(undefined4 *)(param_1 + 0x160) = 0;
      puVar4 = (undefined1 *)(param_1 + 0x170);
    }
    else {
      if (lVar3 == 1) {
        lVar3 = *(longlong *)(param_1 + 0x40);
      }
      else {
        cStack_51 = 1;
        lVar3 = FUN_141e385f0(param_1 + 0x40);
      }
      if ((*(longlong *)(lVar3 + 0x120) == 0) && (cVar5 = '\x01', *(int *)(lVar3 + 0x128) == 0))
      goto LAB_140fac669;
      *(undefined8 *)(lVar3 + 0x120) = 0;
      *(undefined4 *)(lVar3 + 0x128) = 0;
      puVar4 = (undefined1 *)(lVar3 + 0x138);
    }
    *puVar4 = 1;
    cVar5 = '\x01';
  }
LAB_140fac669:
  lStack_b8 = 0;
  uStack_b0 = 0;
  uStack_a8 = uStack_a8 & 0xffffffffffffff00;
  cStack_51 = cVar5;
  FUN_140e832b0(param_1,&lStack_b8);
  uStack_b0 = 0;
  uStack_a8 = 0;
  uStack_a0 = uStack_a0 & 0xffffffffffffff00;
  lStack_b8 = 1;
  cStack_51 = cVar5;
  FUN_140e83360(param_1,&lStack_b8);
  lStack_b8 = CONCAT44(lStack_b8._4_4_,0x14);
  cStack_51 = cVar5;
  FUN_140b91530(param_1,&lStack_b8);
  if ((cVar5 != '\0') && (lStack_70 != 0)) {
    func_0x000140613c20(lStack_68,lStack_70,1);
  }
  return;
}

