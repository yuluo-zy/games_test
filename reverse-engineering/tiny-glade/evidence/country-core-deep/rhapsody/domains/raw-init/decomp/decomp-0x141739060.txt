
undefined4 * FUN_141739060(undefined4 *param_1,uint param_2)

{
  undefined1 uVar1;
  undefined4 uVar2;
  code *pcVar3;
  ulonglong uVar4;
  undefined4 *puVar5;
  undefined1 auStack_38 [8];
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined1 *puStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  if (param_2 < 0x93) {
    uVar4 = (ulonglong)param_2;
    uVar2 = *(undefined4 *)(&UNK_142ccddbc + uVar4 * 4);
    uVar1 = (&UNK_142cce008)[uVar4];
    *param_1 = *(undefined4 *)(&UNK_142ccdb70 + uVar4 * 4);
    param_1[1] = uVar2;
    *(undefined1 *)(param_1 + 2) = uVar1;
    *(undefined1 *)((longlong)param_1 + 9) = uVar1;
    return param_1;
  }
  puStack_30 = &UNK_142ccd828;
  uStack_28 = 1;
  puStack_20 = auStack_38;
  uStack_18 = 0;
  uStack_10 = 0;
  FUN_1428d9390(&puStack_30,&UNK_142ccd838);
  pcVar3 = (code *)swi(3);
  puVar5 = (undefined4 *)(*pcVar3)();
  return puVar5;
}

