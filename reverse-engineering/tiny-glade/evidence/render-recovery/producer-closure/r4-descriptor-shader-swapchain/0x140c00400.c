
longlong * FUN_140c00400(longlong *param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 uVar3;
  ulonglong uStack_58;
  undefined8 uStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar2 = func_0x000142924010(param_2 + 0xdc);
  FUN_140d79f70(&uStack_58,param_2 + 0xdc,lVar2 + 1);
  if (lStack_48 < 0) {
    uVar3 = 0;
code_r0x000140c004c1:
    uStack_40 = uStack_50;
    FUN_1428d8fe3(uVar3,lStack_48,&UNK_142b17750);
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  if (lStack_48 == 0) {
    lVar2 = 1;
  }
  else {
    uVar3 = 1;
    lVar2 = func_0x000140613c10(lStack_48,1);
    if (lVar2 == 0) goto code_r0x000140c004c1;
  }
  func_0x000142923c00(lVar2,uStack_50,lStack_48);
  *param_1 = lStack_48;
  param_1[1] = lVar2;
  param_1[2] = lStack_48;
  if ((uStack_58 & 0x7fffffffffffffff) != 0) {
    func_0x000140613c20(uStack_50,uStack_58,1);
  }
  return param_1;
}

