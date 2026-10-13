longlong * FUN_141286730(longlong *param_1,undefined8 *param_2)
{
  longlong lStack_a0;
  longlong lStack_98;
  longlong lStack_90;
  longlong lStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
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
  undefined1 uStack_28;
  undefined1 uStack_20;
  undefined8 uStack_18;
  uStack_18 = 0xfffffffffffffffe;
  uStack_70 = *param_2;
  lStack_68 = param_2[1];
  uStack_60 = *(undefined4 *)(param_2 + 2);
  uStack_5c = *(undefined4 *)((longlong)param_2 + 0x14);
  uStack_58 = *(undefined4 *)(param_2 + 3);
  uStack_54 = *(undefined4 *)((longlong)param_2 + 0x1c);
  uStack_50 = *(undefined4 *)(param_2 + 4);
  uStack_4c = *(undefined4 *)((longlong)param_2 + 0x24);
  uStack_48 = *(undefined4 *)(param_2 + 5);
  uStack_44 = *(undefined4 *)((longlong)param_2 + 0x2c);
  uStack_40 = 1;
  uStack_38 = 0;
  uStack_30 = 0;
  uStack_28 = 0;
  lStack_88 = 0;
  uStack_80 = 1;
  uStack_78 = 0;
  uStack_20 = 0x80;
  FUN_1413e4830(&lStack_a0,&lStack_88);
  if (SBORROW8(0,lStack_a0)) {
    param_1[1] = lStack_98;
    *param_1 = -0x8000000000000000;
  }
  else {
    *param_1 = lStack_a0;
    param_1[1] = lStack_98;
    param_1[2] = lStack_90;
  }
  if (lStack_68 != 0) {
    func_0x000140613c20(uStack_70,lStack_68,1);
  }
  CloseHandle(CONCAT44(uStack_44,uStack_48));
  if (lStack_88 != 0) {
    func_0x000140613c20(uStack_80,lStack_88,1);
  }
  return param_1;
}