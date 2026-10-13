
void FUN_1421be370(longlong param_1)

{
  code *pcVar1;
  undefined4 *puVar2;
  undefined8 uVar3;
  uint auStack_2e0 [40];
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 *puStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined8 uStack_194;
  undefined4 uStack_18c;
  longlong lStack_188;
  undefined8 uStack_180;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined2 uStack_b8;
  undefined2 uStack_b6;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined2 uStack_9c;
  undefined1 uStack_9a;
  undefined1 uStack_99;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uVar3 = 0x14;
  uStack_240 = FUN_140b9d480(&UNK_142ffa2e4);
  uStack_1e8 = param_1 + 0x28;
  puStack_1f8 = &uStack_240;
  uStack_1f0 = &UNK_140109e90;
  uStack_1e0 = FUN_140bffe80;
  uStack_108 = (undefined8 *)&UNK_142ffa300;
  uStack_100 = (undefined *)0x2;
  uStack_f8 = &puStack_1f8;
  uStack_f0 = 2;
  uStack_e8 = (undefined8 *)0x0;
  uStack_238 = uVar3;
  FUN_140d79f90(&lStack_188,&uStack_108);
  FUN_140d7bd90(&puStack_1f8,&lStack_188);
  uStack_d8 = uStack_1e8;
  uStack_e8 = puStack_1f8;
  uStack_e0 = (undefined4)uStack_1f0;
  uStack_dc = uStack_1f0._4_4_;
  uStack_108 = (undefined8 *)0x0;
  uStack_100 = &UNK_142ec09c8;
  uStack_f8 = (undefined8 **)0x9;
  uStack_f0 = CONCAT53(uStack_f0._3_5_,0x40300);
  uStack_d0 = 0;
  uStack_c8 = 0x3fa6666600000000;
  uStack_c0 = CONCAT62(uStack_c0._2_6_,2);
  uStack_b8 = 2;
  uStack_b0 = 0;
  uStack_9c = 1;
  uStack_9a = 0;
  if (lStack_188 != 0) {
    func_0x000140613c20(uStack_180,lStack_188,1);
  }
  FUN_1421cc2d0(&puStack_1f8,&uStack_108);
  uStack_198 = 0x44160000;
  uStack_108 = puStack_1f8;
  uStack_100 = uStack_1f0;
  uStack_f8._0_4_ = (undefined4)uStack_1e8;
  uStack_f8._4_4_ = uStack_1e8._4_4_;
  uStack_f0._0_4_ = SUB84(uStack_1e0,0);
  uStack_f0._4_4_ = uStack_1e0._4_4_;
  uStack_e8._0_4_ = uStack_1d8;
  uStack_e8._4_4_ = uStack_1d4;
  uStack_e0 = uStack_1d0;
  uStack_dc = uStack_1cc;
  uStack_d8._0_4_ = uStack_1c8;
  uStack_d8._4_4_ = uStack_1c4;
  uStack_d0 = uStack_1c0;
  uStack_cc = uStack_1bc;
  uStack_c8 = uStack_1b8;
  uStack_c0 = uStack_1b0;
  uStack_b8 = (undefined2)uStack_1a8;
  uStack_b6 = (undefined2)((uint)uStack_1a8 >> 0x10);
  uStack_b4 = uStack_1a4;
  uStack_b0 = uStack_1a0;
  uStack_ac = uStack_19c;
  uStack_a8 = 0x44160000;
  uStack_a4 = (undefined4)uStack_194;
  uStack_a0 = (undefined4)((ulonglong)uStack_194 >> 0x20);
  uStack_9c = (undefined2)uStack_18c;
  uStack_9a = (undefined1)((uint)uStack_18c >> 0x10);
  uStack_99 = (undefined1)((uint)uStack_18c >> 0x18);
  puVar2 = (undefined4 *)func_0x000140613c10(0x70,8);
  if (puVar2 != (undefined4 *)0x0) {
    *(ulonglong *)(puVar2 + 0x18) = CONCAT44(uStack_a4,uStack_a8);
    *(ulonglong *)(puVar2 + 0x1a) =
         CONCAT17(uStack_99,CONCAT16(uStack_9a,CONCAT24(uStack_9c,uStack_a0)));
    *(ulonglong *)(puVar2 + 0x14) = CONCAT44(uStack_b4,CONCAT22(uStack_b6,uStack_b8));
    *(ulonglong *)(puVar2 + 0x16) = CONCAT44(uStack_ac,uStack_b0);
    *(undefined8 *)(puVar2 + 0x10) = uStack_c8;
    *(undefined8 *)(puVar2 + 0x12) = uStack_c0;
    puVar2[0xc] = (undefined4)uStack_d8;
    puVar2[0xd] = uStack_d8._4_4_;
    puVar2[0xe] = uStack_d0;
    puVar2[0xf] = uStack_cc;
    puVar2[8] = (undefined4)uStack_e8;
    puVar2[9] = uStack_e8._4_4_;
    puVar2[10] = uStack_e0;
    puVar2[0xb] = uStack_dc;
    puVar2[4] = (undefined4)uStack_f8;
    puVar2[5] = uStack_f8._4_4_;
    puVar2[6] = (undefined4)uStack_f0;
    puVar2[7] = uStack_f0._4_4_;
    *puVar2 = (undefined4)uStack_108;
    puVar2[1] = uStack_108._4_4_;
    puVar2[2] = (undefined4)uStack_100;
    puVar2[3] = uStack_100._4_4_;
    uStack_61 = 1;
    FUN_140bda990(auStack_2e0,param_1);
                    /* WARNING: Could not recover jumptable at 0x0001421be644. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142ffa940 + *(int *)(&UNK_142ffa940 + (ulonglong)auStack_2e0[0] * 4)))
              (&UNK_142ffa940 + *(int *)(&UNK_142ffa940 + (ulonglong)auStack_2e0[0] * 4));
    return;
  }
  FUN_1428d9000(8,0x70);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

