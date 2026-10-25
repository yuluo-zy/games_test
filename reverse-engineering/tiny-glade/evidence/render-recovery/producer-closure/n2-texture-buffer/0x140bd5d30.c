
longlong * FUN_140bd5d30(longlong *param_1,longlong *param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  code *pcVar3;
  ushort uVar4;
  longlong *plVar5;
  ulonglong uStack_78;
  ulonglong uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
  plVar5 = (longlong *)((longlong)param_2 + 0x1c);
  FUN_140bd5bf0(&uStack_78,plVar5);
  if ((uStack_78 & 1) == 0) {
    do {
      uVar4 = (short)(int)param_2[5] + 1;
      if (*(ushort *)((longlong)param_2 + 0x2a) <= uVar4) {
        uStack_78 = param_2[1];
        if (uStack_78 == 0) {
          *param_1 = 0;
          return param_1;
        }
        goto code_r0x000140bd5e76;
      }
      *(ushort *)(param_2 + 5) = uVar4;
      *(int *)((longlong)param_2 + 0x24) = (int)param_2[3];
      *plVar5 = param_2[2];
      FUN_140bd5bf0(&uStack_78,plVar5);
    } while ((uStack_78 & 1) == 0);
  }
  uVar1 = param_2[1];
  if (uVar1 == 0) {
    puStack_58 = &UNK_142b0fd40;
    uStack_50 = 1;
    uStack_48 = 8;
    uStack_40 = 0;
    uStack_38 = 0;
    FUN_1428d9390(&puStack_58,&UNK_142b0fd50);
  }
  else if (uStack_70 <= uVar1) {
    lVar2 = *param_2;
    *(undefined4 *)(param_1 + 3) = 0;
    param_1[2] = 0;
    uVar4 = *(ushort *)(param_2 + 5);
    *(undefined4 *)((longlong)param_1 + 0x24) = uStack_60;
    *(undefined8 *)((longlong)param_1 + 0x1c) = uStack_68;
    *param_2 = lVar2 + uStack_70;
    param_2[1] = uVar1 - uStack_70;
    *param_1 = lVar2;
    param_1[1] = uStack_70;
    *(undefined4 *)(param_1 + 5) = uStack_5c;
    *(uint *)((longlong)param_1 + 0x2c) = (uint)uVar4;
    return param_1;
  }
  FUN_1428d9840();
code_r0x000140bd5e76:
  puStack_58 = &UNK_142b0fdc0;
  uStack_50 = 1;
  uStack_48 = 8;
  uStack_40 = 0;
  uStack_38 = 0;
  FUN_1428d9596(0,&uStack_78,&UNK_142b0fd68,&puStack_58,&UNK_142b0fdd0);
  pcVar3 = (code *)swi(3);
  plVar5 = (longlong *)(*pcVar3)();
  return plVar5;
}

