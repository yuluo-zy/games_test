
undefined4 * FUN_140c022b0(undefined4 *param_1,longlong param_2,longlong param_3)

{
  undefined1 uVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  code *pcVar5;
  longlong lVar6;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 uStack_38;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uStack_38 = *(undefined8 *)(param_2 + 0x30);
  uStack_48 = *(undefined4 *)(param_2 + 0x20);
  uStack_44 = *(undefined4 *)(param_2 + 0x24);
  uStack_40 = *(undefined4 *)(param_2 + 0x28);
  uStack_3c = *(undefined4 *)(param_2 + 0x2c);
  uVar1 = *(undefined1 *)(param_2 + 0x38);
  lVar6 = FUN_1418aeba0(*(undefined8 *)(param_3 + 0x648),&uStack_48,0);
  if (lVar6 != 0) {
    uVar3 = *(undefined8 *)(lVar6 + 0x20);
    uVar2 = *(uint *)(param_2 + 0x40);
    uVar4 = *(undefined8 *)(param_2 + 0x18);
    *(undefined8 *)(param_1 + 0xc) = uStack_38;
    *(ulonglong *)(param_1 + 8) = CONCAT44(uStack_44,uStack_48);
    *(ulonglong *)(param_1 + 10) = CONCAT44(uStack_3c,uStack_40);
    *param_1 = 6;
    *(undefined8 *)(param_1 + 2) = uVar3;
    *(ulonglong *)(param_1 + 4) = (ulonglong)uVar2;
    *(undefined8 *)(param_1 + 6) = uVar4;
    *(undefined1 *)(param_1 + 0xe) = uVar1;
    return param_1;
  }
  FUN_1428d9310(&UNK_142d057c0);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

