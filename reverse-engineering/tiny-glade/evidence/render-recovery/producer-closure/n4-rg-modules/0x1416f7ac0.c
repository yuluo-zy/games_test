
void FUN_1416f7ac0(longlong *param_1,longlong param_2,undefined *param_3,undefined8 param_4,
                  undefined4 param_5)

{
  code *pcVar1;
  longlong lVar2;
  undefined *puVar3;
  undefined8 auStack_40 [6];
  
  lVar2 = *(longlong *)(param_1[1] + 0x648);
  if (*(short *)(param_2 + 0x14) == *(short *)(lVar2 + 0xa50)) {
    if (*(char *)(param_2 + 0x16) != '\x01') goto code_r0x0001416f7b7c;
    if (*(int *)(*(longlong *)(param_2 + 8) + 0xc) == *(int *)(param_2 + 0x10)) {
      if (*(short *)(param_3 + 0x14) != *(short *)(param_2 + 0x14)) goto code_r0x0001416f7ba8;
      if (param_3[0x16] == '\x01') {
        if (*(int *)(*(longlong *)(param_3 + 8) + 0xc) == *(int *)(param_3 + 0x10)) {
          (**(code **)(param_1[1] + 0x370))
                    (*(undefined8 *)(*param_1 + 0x48),
                     *(undefined8 *)(*(longlong *)(param_2 + 8) + 0x30),
                     *(undefined8 *)(*(longlong *)(param_3 + 8) + 0x30),param_5,param_4);
          return;
        }
        goto code_r0x0001416f7c17;
      }
      goto code_r0x0001416f7bd9;
    }
  }
  else {
    auStack_40[0] = 0;
    param_2 = param_2 + 0x14;
    FUN_1428ca816(0,param_2,lVar2 + 0xa50,auStack_40,&UNK_142d022a0);
code_r0x0001416f7b7c:
    auStack_40[0] = 0;
    param_3 = &UNK_142d022d9;
    lVar2 = FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022d9,auStack_40,&UNK_142d022c0);
code_r0x0001416f7ba8:
    auStack_40[0] = 0;
    puVar3 = param_3 + 0x14;
    param_3 = (undefined *)(lVar2 + 0xa50);
    FUN_1428ca816(0,puVar3,param_3,auStack_40,&UNK_142d022a0);
code_r0x0001416f7bd9:
    auStack_40[0] = 0;
    FUN_1428ca902(0,param_3 + 0x16,&UNK_142d022d9,auStack_40,&UNK_142d022c0);
  }
  FUN_1428d9310(&UNK_142c91790);
code_r0x0001416f7c17:
  FUN_1428d9310(&UNK_142c917a8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

