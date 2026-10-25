
undefined4 * FUN_140c0f040(undefined4 *param_1,undefined8 *param_2)

{
  int *piVar1;
  undefined1 uVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  code *pcVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  
  if (*(char *)(param_2 + 5) != '\x01') {
    *(undefined1 *)(param_2 + 5) = 1;
    lVar5 = param_2[1];
    LOCK();
    piVar1 = (int *)(lVar5 + 8);
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    uVar4 = *(undefined4 *)(param_2 + 2);
    uVar2 = *(undefined1 *)((longlong)param_2 + 0x16);
    uVar3 = *(undefined2 *)((longlong)param_2 + 0x14);
    uVar6 = *param_2;
    uVar8 = *(undefined4 *)((longlong)param_2 + 0x1c);
    uVar9 = *(undefined4 *)(param_2 + 4);
    uVar10 = *(undefined4 *)((longlong)param_2 + 0x24);
    *param_1 = *(undefined4 *)(param_2 + 3);
    param_1[1] = uVar8;
    param_1[2] = uVar9;
    param_1[3] = uVar10;
    *(undefined8 *)(param_1 + 4) = uVar6;
    *(longlong *)(param_1 + 6) = lVar5;
    param_1[8] = uVar4;
    *(undefined2 *)(param_1 + 9) = uVar3;
    *(undefined1 *)((longlong)param_1 + 0x26) = uVar2;
    if ((*puRam000000014362c420 & 0x7fffffffffffffff) != 0) {
      FUN_1428d86b0();
    }
    FUN_140be24b0(param_2);
    return param_1;
  }
  FUN_1428d9430(&UNK_142b19ca0,0x1f,&UNK_142b19cc0);
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

