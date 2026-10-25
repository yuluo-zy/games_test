
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140b00b20(undefined8 *param_1,longlong param_2)

{
  code *pcVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  
  if (*(char *)(param_2 + 0x28) == '\0') {
    FUN_140d7bd90(param_1);
    uVar4 = *(undefined4 *)(param_2 + 0x18);
    uVar5 = *(undefined4 *)(param_2 + 0x1c);
    uVar6 = *(undefined4 *)(param_2 + 0x20);
    uVar7 = *(undefined4 *)(param_2 + 0x24);
    *(undefined1 *)(param_1 + 5) = 0;
  }
  else {
    puVar2 = (undefined4 *)func_0x000140613c10(7,1);
    if (puVar2 == (undefined4 *)0x0) {
      FUN_1428d8fe3(1,7,&UNK_142ae67e0);
      pcVar1 = (code *)swi(3);
      puVar3 = (undefined8 *)(*pcVar1)();
      return puVar3;
    }
    *(undefined4 *)((longlong)puVar2 + 3) = 0x746c7561;
    *puVar2 = 0x61666564;
    *(undefined1 *)(param_1 + 5) = 0;
    *param_1 = 7;
    param_1[1] = puVar2;
    param_1[2] = 7;
    uVar4 = _UNK_142a9be30;
    uVar5 = _UNK_142a9be34;
    uVar6 = _UNK_142a9be38;
    uVar7 = _UNK_142a9be3c;
  }
  *(undefined4 *)(param_1 + 3) = uVar4;
  *(undefined4 *)((longlong)param_1 + 0x1c) = uVar5;
  *(undefined4 *)(param_1 + 4) = uVar6;
  *(undefined4 *)((longlong)param_1 + 0x24) = uVar7;
  return param_1;
}

