
void FUN_14062e080(ulonglong param_1,ulonglong param_2)

{
  code *pcVar1;
  undefined *puVar2;
  ulonglong *puVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  longlong lStack_148;
  undefined8 uStack_140;
  undefined8 *puStack_130;
  undefined *puStack_128;
  undefined8 uStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined8 *puStack_108;
  ulonglong uStack_100;
  ulonglong uStack_f8;
  ulonglong uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined *puStack_a0;
  undefined8 **ppuStack_98;
  undefined8 uStack_90;
  undefined8 *puStack_88;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined2 uStack_58;
  undefined2 uStack_56;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined2 uStack_3c;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  undefined8 *puStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uVar6 = 0x1b;
  puStack_118 = (undefined *)FUN_140b9d480(&UNK_142a0c328);
  puStack_130 = &puStack_118;
  puStack_128 = &UNK_140109e90;
  puStack_a8 = &UNK_142a0c348;
  puStack_a0 = (undefined *)0x2;
  ppuStack_98 = &puStack_130;
  uStack_90 = 1;
  puStack_88 = (undefined8 *)0x0;
  uStack_110 = uVar6;
  FUN_140d79f90(&lStack_148,&puStack_a8);
  FUN_140d7bd90(&puStack_130,&lStack_148);
  uStack_78 = uStack_120;
  puStack_88 = puStack_130;
  puStack_80 = puStack_128;
  puStack_a8 = (undefined *)0x0;
  puStack_a0 = &UNK_142b70b58;
  ppuStack_98 = (undefined8 **)0x9;
  uStack_90 = CONCAT53(uStack_90._3_5_,0x40300);
  uStack_70 = 0;
  uStack_68 = 0x3fa6666600000000;
  uStack_60 = CONCAT22(uStack_60._2_2_,2);
  uStack_58 = 2;
  uStack_50 = 0;
  uStack_3c = 1;
  uStack_3a = 0;
  if (lStack_148 != 0) {
    func_0x000140613c20(uStack_140,lStack_148,1);
  }
  FUN_140dc1830(&puStack_118,&puStack_a8);
  if (uStack_100 == 0x800000000000000a) goto LAB_14062e220;
  puVar3 = &uStack_f0;
  uVar5 = 1;
  if ((uStack_100 ^ 0x8000000000000000) < 10) {
    uVar5 = uStack_100 ^ 0x8000000000000000;
  }
  if (uVar5 < 9) {
    if ((0x1ddUL >> (uVar5 & 0x3f) & 1) != 0) goto LAB_14062e220;
    if (uVar5 != 1) goto LAB_14062e209;
    if (uStack_100 == 0) goto LAB_14062e220;
    puVar3 = &uStack_f8;
  }
  else {
LAB_14062e209:
    uStack_100 = uStack_f8;
    if (uStack_f8 == 0) goto LAB_14062e220;
  }
  func_0x000140613c20(*puVar3,uStack_100,1);
LAB_14062e220:
  uStack_100 = 0x8000000000000006;
  puStack_a8 = puStack_118;
  puVar2 = puStack_a8;
  puStack_a0 = (undefined *)uStack_110;
  uStack_78 = uStack_e8;
  uStack_70 = (undefined4)uStack_e0;
  uStack_6c = uStack_e0._4_4_;
  uStack_68._0_4_ = uStack_d8;
  uStack_68._4_4_ = uStack_d4;
  uStack_60 = uStack_d0;
  uStack_5c = uStack_cc;
  uStack_58 = (undefined2)uStack_c8;
  uStack_56 = (undefined2)((ulonglong)uStack_c8 >> 0x10);
  uStack_54 = uStack_c8._4_4_;
  uStack_50 = (undefined4)uStack_c0;
  uStack_4c = uStack_c0._4_4_;
  uStack_48 = (undefined4)uStack_b8;
  uStack_44 = uStack_b8._4_4_;
  uStack_40 = (undefined4)uStack_b0;
  uStack_3c = (undefined2)((ulonglong)uStack_b0 >> 0x20);
  uStack_3a = (undefined1)((ulonglong)uStack_b0 >> 0x30);
  uStack_39 = (undefined1)((ulonglong)uStack_b0 >> 0x38);
  ppuStack_98 = (undefined8 **)puStack_108;
  uStack_90 = 0x8000000000000006;
  puStack_a8._0_4_ = (uint)puStack_118;
  puStack_88 = (undefined8 *)param_1;
  puStack_80 = (undefined *)param_2;
  if (4 < (uint)puStack_a8) {
    uStack_38 = uStack_110;
    puStack_30 = puStack_108;
    uStack_f8 = param_1;
    uStack_f0 = param_2;
    puStack_a8 = puVar2;
    if ((code *)*puStack_108 != (code *)0x0) {
      (*(code *)*puStack_108)(uStack_110);
    }
    if (puStack_30[1] != 0) {
      func_0x000140613c20(uStack_38,puStack_30[1],puStack_30[2]);
    }
  }
  puStack_a8 = (undefined *)0x5;
  puStack_a0 = (undefined *)0x1;
  ppuStack_98 = (undefined8 **)&UNK_142a0bd08;
  uStack_e0 = CONCAT44(uStack_6c,uStack_70);
  uStack_f8 = (ulonglong)puStack_88;
  uStack_f0 = (ulonglong)puStack_80;
  uStack_e8 = uStack_78;
  uStack_d8 = (undefined4)uStack_68;
  uStack_d4 = uStack_68._4_4_;
  uStack_d0 = uStack_60;
  uStack_cc = uStack_5c;
  uStack_c8 = CONCAT44(uStack_54,CONCAT22(uStack_56,uStack_58));
  puStack_118 = (undefined *)0x5;
  uStack_110 = 1;
  puStack_108 = (undefined8 *)&UNK_142a0bd08;
  uStack_100 = uStack_90;
  uStack_b0 = CONCAT17(uStack_39,CONCAT16(uStack_3a,CONCAT24(uStack_3c,uStack_40)));
  uStack_c0 = 0;
  uStack_b8 = 0;
  puVar4 = (undefined8 *)func_0x000140613c10(0x70,8);
  if (puVar4 != (undefined8 *)0x0) {
    puVar4[0xc] = uStack_b8;
    puVar4[0xd] = uStack_b0;
    puVar4[10] = uStack_c8;
    puVar4[0xb] = uStack_c0;
    puVar4[8] = CONCAT44(uStack_d4,uStack_d8);
    puVar4[9] = CONCAT44(uStack_cc,uStack_d0);
    puVar4[6] = uStack_e8;
    puVar4[7] = uStack_e0;
    puVar4[4] = uStack_f8;
    puVar4[5] = uStack_f0;
    puVar4[2] = puStack_108;
    puVar4[3] = uStack_100;
    *puVar4 = puStack_118;
    puVar4[1] = uStack_110;
    return;
  }
  FUN_1428d9000(8,0x70);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

