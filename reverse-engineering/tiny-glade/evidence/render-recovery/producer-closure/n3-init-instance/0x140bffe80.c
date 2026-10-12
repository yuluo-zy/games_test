
undefined4 FUN_140bffe80(longlong *param_1,longlong param_2)

{
  longlong *plVar1;
  code *pcVar2;
  char cVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 *puVar6;
  undefined4 auStack_108 [6];
  undefined4 auStack_f0 [6];
  longlong *plStack_d8;
  longlong *plStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined8 *puStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined1 uStack_a0;
  undefined8 *puStack_98;
  undefined *puStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 *puStack_78;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 *puStack_40;
  undefined *puStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  uStack_50 = (longlong **)0x0;
  uStack_48 = (undefined *)0x1;
  puStack_40 = (undefined8 *)0x0;
  puStack_c8 = (undefined *)0x0;
  puStack_b8 = (undefined8 *)0x0;
  uStack_a8 = 0x20;
  uStack_a0 = 3;
  puStack_90 = &UNK_142b17590;
  puStack_98 = &uStack_50;
  cVar3 = FUN_140c00200(param_1 + 6,&puStack_c8);
  if (cVar3 != '\0') {
    FUN_1428d9760(&UNK_142b175c0,0x37,&uStack_68,&UNK_142b176e0,&UNK_142b17648);
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  uStack_88 = (undefined4)uStack_50;
  uStack_84 = uStack_50._4_4_;
  uStack_80 = (undefined4)uStack_48;
  uStack_7c = uStack_48._4_4_;
  puStack_78 = puStack_40;
  if (SBORROW8(0,param_1[3])) {
    uStack_50 = (longlong **)&uStack_88;
    uStack_48 = &UNK_14003bad0;
    puStack_c8 = &UNK_142b17938;
    uStack_c0 = 2;
    uStack_b0 = 1;
    uStack_a8 = 0;
    puVar6 = auStack_108;
    puStack_b8 = &uStack_50;
    FUN_140d79f90(puVar6,&puStack_c8);
  }
  else {
    plVar1 = param_1 + 3;
    plStack_d0 = plVar1;
    if (puStack_40 <= (undefined8 *)param_1[5]) {
      iVar4 = func_0x000142923bf0(uStack_48,param_1[5] + (param_1[4] - (longlong)puStack_40));
      if (iVar4 == 0) {
        FUN_140d7bd90(&uStack_68,plVar1);
        goto code_r0x000140c00002;
      }
    }
    uStack_50 = &plStack_d0;
    uStack_48 = &UNK_140053920;
    puStack_40 = (undefined8 *)&uStack_88;
    puStack_38 = &UNK_14003bad0;
    puStack_c8 = &UNK_142b178f8;
    uStack_c0 = 3;
    uStack_b0 = 2;
    uStack_a8 = 0;
    puVar6 = auStack_f0;
    puStack_b8 = &uStack_50;
    FUN_140d79f90(puVar6,&puStack_c8);
  }
  uStack_58 = *(undefined8 *)(puVar6 + 4);
  uStack_68._0_4_ = *puVar6;
  uStack_68._4_4_ = puVar6[1];
  uStack_60 = puVar6[2];
  uStack_5c = puVar6[3];
code_r0x000140c00002:
  if (SBORROW8(0,*param_1)) {
    uVar5 = FUN_140d86b90(param_2,CONCAT44(uStack_5c,uStack_60),uStack_58);
  }
  else {
    uStack_50 = &plStack_d8;
    uStack_48 = &UNK_140053920;
    puStack_40 = &uStack_68;
    puStack_38 = &UNK_14003bad0;
    puStack_c8 = &UNK_142b17960;
    uStack_c0 = 2;
    uStack_b0 = 2;
    uStack_a8 = 0;
    plStack_d8 = param_1;
    puStack_b8 = &uStack_50;
    uVar5 = FUN_140d85ad0(*(undefined8 *)(param_2 + 0x30),*(undefined8 *)(param_2 + 0x38),
                          &puStack_c8);
  }
  if (CONCAT44(uStack_68._4_4_,(undefined4)uStack_68) != 0) {
    func_0x000140613c20(CONCAT44(uStack_5c,uStack_60),
                        CONCAT44(uStack_68._4_4_,(undefined4)uStack_68),1);
  }
  if (CONCAT44(uStack_84,uStack_88) != 0) {
    func_0x000140613c20(CONCAT44(uStack_7c,uStack_80),CONCAT44(uStack_84,uStack_88),1);
  }
  return uVar5;
}

