
undefined8 FUN_140be6b50(undefined8 param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  
  if (lRam0000000143634e40 != 0) {
    FUN_1418c1430(param_1,0x143634e40);
    return param_1;
  }
  FUN_1428d9310(&UNK_142b12f70);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

