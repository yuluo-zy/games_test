
undefined4 * FUN_140a96740(undefined4 *param_1,longlong *param_2,undefined *param_3)

{
  undefined4 uVar1;
  longlong lVar2;
  ulonglong *puVar3;
  code *pcVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  undefined4 *puVar7;
  int iStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_90;
  int iStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
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
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  if (param_3 < (undefined *)param_2[1]) {
    if ((*(byte *)(*param_2 + (longlong)param_3 * 8) & 1) == 0) goto code_r0x000140a96846;
    if (param_3 < *(undefined **)(param_2[2] + 0x10)) {
      uVar1 = *(undefined4 *)(*param_2 + 4 + (longlong)param_3 * 8);
      FUN_140bfeea0(&iStack_80,param_2[3],(longlong)param_3 * 0x38 + *(longlong *)(param_2[2] + 8),0
                   );
      if (iStack_80 != 0xb) goto code_r0x000140a96870;
      uVar5 = CONCAT44(uStack_74,uStack_78);
      lVar2 = param_2[4];
      puVar3 = *(ulonglong **)(lVar2 + 0x10);
      uVar6 = puVar3[4] & 0xfffffffffffffff8;
      if ((*puVar3 <= uVar6) && (0x1f < uVar6 - *puVar3)) {
        puVar7 = (undefined4 *)(uVar6 - 0x20);
        puVar3[4] = (ulonglong)puVar7;
        if (puVar7 != (undefined4 *)0x0) goto code_r0x000140a967e2;
      }
      param_3 = (undefined *)0x20;
      puVar7 = (undefined4 *)FUN_1428d7780(lVar2,8,0x20);
      if (puVar7 != (undefined4 *)0x0) {
code_r0x000140a967e2:
        *puVar7 = 0;
        *(undefined8 *)(puVar7 + 2) = 0;
        *(undefined8 *)(puVar7 + 4) = uVar5;
        *(undefined8 *)(puVar7 + 6) = 5;
        *param_1 = 0;
        param_1[1] = uVar1;
        *(undefined4 **)(param_1 + 2) = puVar7;
        return param_1;
      }
      FUN_1428d79d0();
      goto code_r0x000140a96834;
    }
  }
  else {
code_r0x000140a96834:
    FUN_1428d9518(param_3);
code_r0x000140a96846:
    param_3 = &UNK_142adb040;
    FUN_1428d9330(&UNK_142adb028,0x16,&UNK_142adb040);
  }
  FUN_1428d9518(param_3);
code_r0x000140a96870:
  uStack_98 = uStack_40;
  uStack_90 = uStack_38;
  uStack_a8 = uStack_50;
  uStack_a4 = uStack_4c;
  uStack_a0 = uStack_48;
  uStack_9c = uStack_44;
  uStack_b8 = uStack_60;
  uStack_b4 = uStack_5c;
  uStack_b0 = uStack_58;
  uStack_ac = uStack_54;
  uStack_c8 = uStack_70;
  uStack_c4 = uStack_6c;
  uStack_c0 = uStack_68;
  uStack_bc = uStack_64;
  iStack_d8 = iStack_80;
  uStack_d4 = uStack_7c;
  uStack_d0 = uStack_78;
  uStack_cc = uStack_74;
  FUN_1428d9760(&UNK_142adab48,0x2b,&iStack_d8,&UNK_142adab28,&UNK_142adb070);
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

