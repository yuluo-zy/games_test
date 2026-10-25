
undefined8 * FUN_1421bb740(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3)

{
  undefined8 uVar1;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined *puStack_80;
  undefined8 uStack_78;
  longlong **pplStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_50;
  code *pcStack_48;
  longlong lStack_40;
  longlong lStack_38;
  undefined8 uStack_30;
  undefined8 *puStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
                    /* WARNING: Switch is manually overridden */
  switch(*param_3) {
  case 0:
    uVar1 = param_3[1];
    *param_2 = 1;
    param_2[1] = uVar1;
    break;
  case 1:
    *(undefined1 *)(param_2 + 0x12) = 1;
    break;
  case 2:
    uStack_90 = *(undefined4 *)(param_3 + 1);
    uStack_8c = *(undefined4 *)((longlong)param_3 + 0xc);
    uStack_88 = *(undefined4 *)(param_3 + 2);
    uStack_84 = *(undefined4 *)((longlong)param_3 + 0x14);
    plStack_50 = (longlong *)&uStack_90;
    pcStack_48 = (code *)&UNK_1405338a0;
    puStack_80 = &UNK_142ffa058;
    uStack_78 = 2;
    uStack_60 = 0;
    uStack_68 = 1;
    pplStack_70 = &plStack_50;
    FUN_140d5c990(&puStack_80);
    lStack_38 = func_0x000140b7f2d0(CONCAT44(uStack_8c,uStack_90),CONCAT44(uStack_84,uStack_88));
    if (lStack_38 == 0) goto override_jmp_1421bb76e_case_3;
    pcStack_48 = FUN_140d5ace0;
    puStack_80 = &UNK_142ffa098;
    uStack_78 = 2;
    uStack_60 = 0;
    uStack_68 = 1;
    pplStack_70 = &plStack_50;
    plStack_50 = &lStack_38;
    FUN_140d5caa0(&puStack_80);
    if (((uint)lStack_38 & 3) != 1) break;
    lStack_40 = lStack_38 + -1;
    uStack_30 = *(undefined8 *)(lStack_38 + -1);
    puStack_28 = *(undefined8 **)(lStack_38 + 7);
    if ((code *)*puStack_28 != (code *)0x0) {
      (*(code *)*puStack_28)(uStack_30);
    }
    goto LAB_1421bb92b;
  case 3:
override_jmp_1421bb76e_case_3:
    *param_1 = 0x8000000000000001;
    param_1[1] = 0;
    return param_1;
  case 4:
    uVar1 = FUN_140b9dbc0();
    lStack_38 = FUN_14258f760(uVar1);
    if (lStack_38 == 0) break;
    pcStack_48 = FUN_140d5ace0;
    puStack_80 = &UNK_142ffa0d0;
    uStack_78 = 2;
    uStack_60 = 0;
    pplStack_70 = &plStack_50;
    uStack_68 = 1;
    plStack_50 = &lStack_38;
    FUN_140d5caa0(&puStack_80);
    if (((uint)lStack_38 & 3) != 1) break;
    lStack_40 = lStack_38 + -1;
    uStack_30 = *(undefined8 *)(lStack_38 + -1);
    puStack_28 = *(undefined8 **)(lStack_38 + 7);
    if ((code *)*puStack_28 != (code *)0x0) {
      (*(code *)*puStack_28)(uStack_30);
    }
LAB_1421bb92b:
    if (puStack_28[1] != 0) {
      func_0x000140613c20(uStack_30,puStack_28[1],puStack_28[2]);
    }
    func_0x000140613c20(lStack_40,0x18,8);
  }
  *param_1 = 0x800000000000001e;
  return param_1;
}

