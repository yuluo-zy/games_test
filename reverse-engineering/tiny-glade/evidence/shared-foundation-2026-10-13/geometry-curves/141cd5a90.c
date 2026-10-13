
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_141cd5a90(undefined8 param_1,undefined8 param_2,float param_3,float param_4)

{
  float *pfVar1;
  longlong unaff_RBX;
  undefined8 unaff_RSI;
  float **unaff_RDI;
  longlong unaff_R12;
  longlong lVar2;
  undefined8 unaff_R13;
  undefined8 uVar3;
  undefined *unaff_R14;
  float *unaff_R15;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined *puStack_e8;
  float *pfStack_e0;
  float **ppfStack_d8;
  longlong lStack_d0;
  undefined8 uStack_c8;
  float *pfStack_b0;
  code *pcStack_a8;
  float *pfStack_a0;
  undefined8 uStack_98;
  longlong lStack_90;
  undefined *puStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  float fStack_68;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  fStack_68 = param_3;
  if ((param_3 < 0.0) || (_DAT_142925904 < param_3)) {
LAB_141cd5d11:
    pfStack_b0 = &fStack_68;
    pcStack_a8 = FUN_140d8c910;
    puStack_e8 = &UNK_143043930;
    pfStack_e0 = (float *)0x1;
    uStack_c8 = 0;
    ppfStack_d8 = &pfStack_b0;
    lStack_d0 = 1;
    FUN_1428d9390(&puStack_e8,&UNK_1430438f0);
LAB_141cd5d5f:
    FUN_1428d9430(&UNK_143043940,0x1d,&UNK_1430438f0);
    lStack_90 = unaff_R12;
    uStack_98 = unaff_R13;
    puStack_88 = unaff_R14;
    pfStack_a0 = unaff_R15;
LAB_141cd5d77:
    FUN_1429108f0(&lStack_80,unaff_RBX,unaff_RDI,4,0xc);
    unaff_RBX = lStack_70;
    lVar2 = lStack_90;
    uVar3 = uStack_98;
  }
  else {
    if (NAN(param_3)) goto LAB_141cd5d5f;
    fStack_68 = param_4;
    if ((param_4 < 0.0) || (_DAT_142925904 < param_4)) goto LAB_141cd5d11;
    if (NAN(param_4)) goto LAB_141cd5d5f;
    if (param_3 < param_4) {
      uStack_100 = 0;
      uStack_f8 = 4;
      uStack_f0 = 0;
      uStack_62 = 1;
      FUN_141cd5530(param_2,param_3,param_4,&uStack_100);
      uStack_62 = 0;
      FUN_141cd6240(param_1,&uStack_100);
      return param_1;
    }
    lStack_80 = 0;
    lStack_78 = 4;
    lStack_70 = 0;
    if ((param_3 != _DAT_142925904) || (NAN(param_3) || NAN(_DAT_142925904))) {
      uStack_61 = 1;
      FUN_141cd5530(param_2,param_3,_DAT_142925904,&lStack_80);
    }
    unaff_RSI = param_1;
    if ((param_4 == 0.0) && (!NAN(param_4))) goto LAB_141cd5ce6;
    uStack_118 = 0;
    uStack_110 = 4;
    uStack_108 = 0;
    uStack_63 = 1;
    FUN_141cd5530(param_2,0,param_4,&uStack_118);
    uStack_63 = 0;
    FUN_141cd6240(&puStack_e8,&uStack_118);
    if (SBORROW8(0,(longlong)puStack_e8)) goto LAB_141cd5ce6;
    if (lStack_70 == 0) {
      unaff_RBX = 0;
    }
    else {
      unaff_RBX = lStack_70 + -1;
      lStack_70 = unaff_RBX;
    }
    pfVar1 = (float *)(lStack_78 + -0xc + unaff_RBX * 0xc);
    if (unaff_RBX == 0) {
      pfVar1 = (float *)0x0;
    }
    if ((ppfStack_d8 == (float **)0x0) || (pfVar1 == (float *)0x0)) {
      if ((unaff_RBX != 0) && ((ppfStack_d8 == (float **)0x0 && (pfVar1 == (float *)0x0)))) {
LAB_141cd5c6f:
        unaff_RBX = unaff_RBX + -1;
        lStack_70 = unaff_RBX;
      }
    }
    else if ((*pfVar1 == *pfStack_e0) && (!NAN(*pfVar1) && !NAN(*pfStack_e0))) {
      if (((pfVar1[1] == pfStack_e0[1]) && (!NAN(pfVar1[1]) && !NAN(pfStack_e0[1]))) &&
         (unaff_RBX != 0)) {
        if ((pfVar1[2] == pfStack_e0[2]) && (!NAN(pfVar1[2]) && !NAN(pfStack_e0[2])))
        goto LAB_141cd5c6f;
      }
    }
    unaff_RDI = ppfStack_d8;
    lStack_90 = lStack_d0;
    lVar2 = lStack_d0;
    uStack_98 = uStack_c8;
    uVar3 = uStack_c8;
    puStack_88 = puStack_e8;
    pfStack_a0 = pfStack_e0;
    if ((float **)(lStack_80 - unaff_RBX) < ppfStack_d8) goto LAB_141cd5d77;
  }
  func_0x000142923c00(lStack_78 + unaff_RBX * 0xc,pfStack_a0,(longlong)unaff_RDI * 0xc);
  lStack_70 = unaff_RBX + (longlong)unaff_RDI;
  if (puStack_88 != (undefined *)0x0) {
    func_0x000140613c20(pfStack_a0,(longlong)puStack_88 * 0xc,4);
  }
  if (lVar2 != 0) {
    func_0x000140613c20(uVar3,lVar2 << 2,4);
  }
LAB_141cd5ce6:
  uStack_61 = 0;
  FUN_141cd6240(unaff_RSI,&lStack_80);
  return unaff_RSI;
}

