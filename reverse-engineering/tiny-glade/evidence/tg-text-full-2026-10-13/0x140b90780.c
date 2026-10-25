
longlong * FUN_140b90780(undefined8 *param_1,float param_2,float param_3)

{
  longlong lVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined8 auStack_338 [40];
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  longlong lStack_1e8;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined1 uStack_74;
  longlong lStack_70;
  longlong lStack_68;
  undefined8 *puStack_60;
  undefined1 uStack_51;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  uStack_b8 = param_1[7];
  lVar3 = 0;
  if (uStack_b8 < -0x7ffffffffffffffe) {
    lVar3 = uStack_b8 + -0x7fffffffffffffff;
  }
  if (lVar3 == 0) {
    if ((*(float *)(param_1 + 0x2a) == param_2) &&
       (!NAN(*(float *)(param_1 + 0x2a)) && !NAN(param_2))) {
      if ((*(float *)((longlong)param_1 + 0x154) == param_3) &&
         (!NAN(*(float *)((longlong)param_1 + 0x154)) && !NAN(param_3))) {
        return (longlong *)0x0;
      }
    }
    lVar3 = param_1[8];
    lVar1 = param_1[9];
    param_1[7] = 0;
    param_1[8] = 8;
    param_1[9] = 0;
    goto joined_r0x000140b90848;
  }
  if (lVar3 == 1) {
    plVar4 = (longlong *)param_1[8];
    if ((*(float *)(plVar4 + 0x23) == param_2) && (!NAN(*(float *)(plVar4 + 0x23)) && !NAN(param_2))
       ) {
LAB_140b90873:
      if ((*(float *)((longlong)plVar4 + 0x11c) == param_3) &&
         (!NAN(*(float *)((longlong)plVar4 + 0x11c)) && !NAN(param_3))) {
        return plVar4;
      }
    }
  }
  else {
    plVar4 = (longlong *)FUN_141801600(param_1 + 8);
    if ((*(float *)(plVar4 + 0x23) == param_2) && (!NAN(*(float *)(plVar4 + 0x23)) && !NAN(param_2))
       ) goto LAB_140b90873;
  }
  uStack_b8 = *plVar4;
  lVar3 = plVar4[1];
  lVar1 = plVar4[2];
  *plVar4 = 0;
  plVar4[1] = 8;
  plVar4[2] = 0;
joined_r0x000140b90848:
  plVar4 = (longlong *)-uStack_b8;
  if (!SBORROW8(0,uStack_b8)) {
    uStack_98._0_4_ = (undefined4)lVar3;
    uStack_98._4_4_ = (undefined4)((ulonglong)lVar3 >> 0x20);
    uStack_90._0_4_ = (undefined4)lVar1;
    uStack_90._4_4_ = (undefined4)((ulonglong)lVar1 >> 0x20);
    uStack_b0 = (undefined4)uStack_98;
    uStack_ac = uStack_98._4_4_;
    uStack_a8 = (undefined4)uStack_90;
    uStack_a4 = uStack_90._4_4_;
    uStack_51 = 1;
    uStack_98 = lVar3;
    uStack_90 = lVar1;
    FUN_140e15190(&uStack_1f8,param_2);
    auStack_338[0] = CONCAT44(uStack_a4,uStack_a8);
    uStack_348 = (undefined4)uStack_b8;
    uStack_344 = uStack_b8._4_4_;
    uStack_340 = uStack_b0;
    uStack_33c = uStack_ac;
    lStack_68 = CONCAT44(uStack_1ec,uStack_1f0);
    lStack_70 = lStack_1e8;
    puStack_60 = (undefined8 *)0x0;
    lVar3 = lStack_68;
    while (lStack_a0 = (longlong)puStack_60, (undefined8 *)lStack_70 != puStack_60) {
      puStack_60 = (undefined8 *)((longlong)puStack_60 + 1);
      FUN_1417f8c50(lVar3);
      lVar3 = lVar3 + 0xb0;
    }
    if (CONCAT44(uStack_1f4,uStack_1f8) != 0) {
      func_0x000140613c20(lStack_68,CONCAT44(uStack_1f4,uStack_1f8) * 0xb0,8);
    }
    lStack_1e8 = auStack_338[0];
    uStack_1f8 = uStack_348;
    uStack_1f4 = uStack_344;
    uStack_1f0 = uStack_340;
    uStack_1ec = uStack_33c;
    func_0x000142923c00(auStack_338,&uStack_1f8,0x140);
    uStack_84 = 0;
    uStack_80 = 0;
    uStack_7c = 0;
    uStack_78 = 0;
    uStack_74 = 0;
    puStack_60 = param_1;
    FUN_1417f8980(param_1);
    puVar2 = puStack_60;
    *puStack_60 = 0;
    puStack_60[4] = 0x8000000000000000;
    func_0x000142923c00(puStack_60 + 5,&uStack_348,0x150);
    *(undefined4 *)(puVar2 + 0x2f) = 0;
    *(ulonglong *)((longlong)puVar2 + 0x17c) = CONCAT44(uStack_84,uStack_88);
    *(ulonglong *)((longlong)puVar2 + 0x184) = CONCAT44(uStack_7c,uStack_80);
    plVar4 = (longlong *)CONCAT17(uStack_74,CONCAT43(uStack_78,uStack_7c._1_3_));
    *(longlong **)((longlong)puVar2 + 0x189) = plVar4;
    *(undefined4 *)(puVar2 + 0x33) = 4;
  }
  return plVar4;
}

