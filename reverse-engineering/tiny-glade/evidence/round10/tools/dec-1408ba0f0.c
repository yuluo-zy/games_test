
longlong * FUN_1408ba0f0(longlong *param_1,char param_2)

{
  code *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  longlong lStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  lVar3 = 3;
  if (param_2 != '\0') {
    FUN_14091c650(&lStack_50,0,100000000);
    if (lStack_50 == 3) {
      FUN_1428d9760(&UNK_142aa0740,0x1d,&uStack_78,&UNK_142aa0378,&UNK_142aa0798);
      goto code_r0x0001408ba21f;
    }
    lStack_58 = lStack_28;
    uStack_78 = CONCAT44(uStack_44,uStack_48);
    uStack_70 = CONCAT44(uStack_3c,uStack_40);
    uStack_68 = CONCAT44(uStack_34,uStack_38);
    uStack_60 = CONCAT44(uStack_2c,uStack_30);
    lVar3 = lStack_50;
  }
  uStack_48 = (undefined4)uStack_78;
  uStack_44 = uStack_78._4_4_;
  uStack_40 = (undefined4)uStack_70;
  uStack_3c = uStack_70._4_4_;
  uStack_38 = (undefined4)uStack_68;
  uStack_34 = uStack_68._4_4_;
  uStack_30 = (undefined4)uStack_60;
  uStack_2c = uStack_60._4_4_;
  lStack_28 = lStack_58;
  lStack_50 = lVar3;
  puVar2 = (undefined8 *)func_0x000140613c10(0x30,8);
  if (puVar2 != (undefined8 *)0x0) {
    *puVar2 = 1;
    puVar2[1] = 1;
    *(undefined1 *)(puVar2 + 2) = 0;
    *(undefined1 *)((longlong)puVar2 + 0x11) = 0;
    puVar2[3] = 0x8000000000000000;
    param_1[4] = CONCAT44(uStack_2c,uStack_30);
    param_1[5] = lStack_28;
    param_1[2] = CONCAT44(uStack_3c,uStack_40);
    param_1[3] = CONCAT44(uStack_34,uStack_38);
    *param_1 = lStack_50;
    param_1[1] = CONCAT44(uStack_44,uStack_48);
    param_1[6] = (longlong)puVar2;
    return param_1;
  }
  FUN_1428d9000(8,0x30);
code_r0x0001408ba21f:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

