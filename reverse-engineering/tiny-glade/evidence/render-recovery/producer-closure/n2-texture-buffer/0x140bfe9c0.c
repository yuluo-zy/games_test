
undefined8
FUN_140bfe9c0(undefined8 param_1,longlong param_2,longlong param_3,undefined8 param_4,
             undefined8 param_5,undefined1 param_6)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 auStack_38 [6];
  
  if (*(short *)(param_3 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_3 + 0x16) == '\0') {
      lVar3 = *(longlong *)(param_3 + 8);
      if (*(int *)(lVar3 + 0xc) == *(int *)(param_3 + 0x10)) {
        FUN_140bfeac0(param_1,param_2,param_3,lVar3 + 0x10,*(undefined4 *)(lVar3 + 0x18),param_4,
                      param_5,param_6,param_6);
        return param_1;
      }
      goto code_r0x000140bfeaa5;
    }
  }
  else {
    auStack_38[0] = 0;
    lVar3 = param_3 + 0x14;
    param_3 = *(longlong *)(param_2 + 0x648) + 0xa50;
    FUN_1428ca816(0,lVar3,param_3,auStack_38,&UNK_142d022a0);
  }
  auStack_38[0] = 0;
  FUN_1428ca902(0,param_3 + 0x16,&UNK_142d022b8,auStack_38,&UNK_142d022c0);
code_r0x000140bfeaa5:
  FUN_1428d9310(&UNK_142b17150);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

