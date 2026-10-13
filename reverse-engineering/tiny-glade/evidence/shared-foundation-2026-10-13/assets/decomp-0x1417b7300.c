
longlong * FUN_1417b7300(longlong *param_1,undefined1 param_2,char param_3,undefined1 param_4)

{
  longlong lVar1;
  undefined8 unaff_RBX;
  undefined8 unaff_RDI;
  undefined8 *puVar2;
  undefined8 uVar3;
  longlong lStack_158;
  undefined8 *puStack_150;
  undefined8 uStack_148;
  longlong lStack_140;
  undefined8 *puStack_138;
  longlong lStack_130;
  longlong lStack_128;
  undefined8 *puStack_120;
  longlong lStack_118;
  longlong lStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  longlong lStack_c8;
  longlong lStack_c0;
  undefined8 uStack_b8;
  longlong lStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined8 *puStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_140d61b00(&lStack_98,&UNK_142cdf570,0x13);
  FUN_140d59140(&lStack_c8,&UNK_142cdf5a3,0x21);
  FUN_1420d7300(&lStack_158,&lStack_c8);
  if (SBORROW8(0,lStack_158)) {
    param_1[1] = (longlong)puStack_150;
    lVar1 = -0x7fffffffffffffff;
  }
  else {
    lStack_50 = lStack_158;
    uStack_b8 = 0;
    lStack_c8 = 1;
    lStack_c0 = 0x80;
    puStack_58 = puStack_150;
    FUN_1420d7f60(&lStack_128,&lStack_c8,puStack_150,uStack_148);
    FUN_1420d7100(&lStack_140,&lStack_128);
    if (SBORROW8(0,lStack_140)) {
      param_1[1] = (longlong)puStack_138;
      *param_1 = -0x7fffffffffffffff;
      if (lStack_50 == 0) goto LAB_1417b7516;
      uVar3 = 1;
      puStack_138 = puStack_58;
      lStack_140 = lStack_50;
LAB_1417b7410:
      func_0x000140613c20(puStack_138,lStack_140,uVar3);
      goto LAB_1417b7516;
    }
    uStack_78 = lStack_140;
    uStack_70 = puStack_138;
    lStack_68 = lStack_130;
    lVar1 = lStack_50;
    if (lStack_50 != 0) {
      func_0x000140613c20(puStack_58,lStack_50,1);
    }
    if (param_3 != '\0') {
      uVar3 = FUN_140d76ed0(&lStack_98);
      FUN_140d6a710(&lStack_128,uVar3,lVar1);
      if ((int)lStack_128 != 2) {
        FUN_1417b7790(&lStack_c8,CONCAT71((int7)((ulonglong)unaff_RDI >> 8),param_2) & 0xffffffff,
                      CONCAT71((int7)((ulonglong)unaff_RBX >> 8),param_4) & 0xffffffff);
        if (lStack_c8 != 0) {
          lStack_118 = lStack_68;
          lStack_128 = uStack_78;
          puStack_120 = uStack_70;
          lStack_110 = lStack_c8;
          *param_1 = uStack_78;
          param_1[1] = (longlong)uStack_70;
          param_1[8] = lStack_a0;
          param_1[6] = lStack_b0;
          param_1[7] = lStack_a8;
          uStack_108._0_4_ = (undefined4)lStack_c0;
          uStack_108._4_4_ = (undefined4)((ulonglong)lStack_c0 >> 0x20);
          uStack_100._0_4_ = (undefined4)uStack_b8;
          uStack_100._4_4_ = (undefined4)((ulonglong)uStack_b8 >> 0x20);
          *(undefined4 *)(param_1 + 4) = (undefined4)uStack_108;
          *(undefined4 *)((longlong)param_1 + 0x24) = uStack_108._4_4_;
          *(undefined4 *)(param_1 + 5) = (undefined4)uStack_100;
          *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_100._4_4_;
          param_1[2] = lStack_68;
          param_1[3] = lStack_c8;
          uStack_108 = lStack_c0;
          uStack_100 = uStack_b8;
          goto LAB_1417b7516;
        }
        param_1[1] = lStack_c0;
        *param_1 = -0x7fffffffffffffff;
        if (lStack_130 != 0) {
          puVar2 = puStack_138 + 1;
          lVar1 = lStack_130;
          do {
            if (puVar2[-1] != 0) {
              func_0x000140613c20(*puVar2,puVar2[-1],1);
            }
            puVar2 = puVar2 + 3;
            lVar1 = lVar1 + -1;
          } while (lVar1 != 0);
        }
        if (lStack_140 == 0) goto LAB_1417b7516;
        lStack_140 = lStack_140 * 0x18;
        uVar3 = 8;
        goto LAB_1417b7410;
      }
      if (((uint)puStack_120 & 3) == 1) {
        lStack_60 = (longlong)puStack_120 + -1;
        lStack_50 = *(longlong *)((longlong)puStack_120 + -1);
        puStack_58 = *(undefined8 **)((longlong)puStack_120 + 7);
        if ((code *)*puStack_58 != (code *)0x0) {
          (*(code *)*puStack_58)(lStack_50);
        }
        if (puStack_58[1] != 0) {
          func_0x000140613c20(lStack_50,puStack_58[1],puStack_58[2]);
        }
        func_0x000140613c20(lStack_60,0x18,8);
      }
    }
    param_1[3] = lStack_68;
    *(undefined4 *)(param_1 + 1) = (undefined4)uStack_78;
    *(undefined4 *)((longlong)param_1 + 0xc) = uStack_78._4_4_;
    *(undefined4 *)(param_1 + 2) = (undefined4)uStack_70;
    *(undefined4 *)((longlong)param_1 + 0x14) = uStack_70._4_4_;
    lVar1 = -0x8000000000000000;
  }
  *param_1 = lVar1;
LAB_1417b7516:
  if (lStack_98 != 0) {
    func_0x000140613c20(uStack_90,lStack_98,1);
  }
  return param_1;
}

