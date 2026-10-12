
ulonglong FUN_140be2710(undefined4 *param_1,undefined8 param_2)

{
  char cVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  lStack_58 = 0;
  lStack_50 = 1;
  lStack_48 = 0;
  uStack_78 = *param_1;
  uStack_74 = param_1[1];
  uStack_70 = param_1[2];
  uStack_6c = param_1[3];
  lVar4 = 0;
  do {
    while( true ) {
      lVar2 = lStack_48;
      cVar1 = *(char *)((longlong)&uStack_78 + lVar4);
      lVar4 = lVar4 + 1;
      if (cVar1 == '\0') goto code_r0x000140be2763;
      if (cVar1 < '\0') break;
      if (lStack_48 == lStack_58) {
        FUN_140d79570(&lStack_58,&UNK_142b12ee8);
      }
      *(char *)(lStack_50 + lVar2) = cVar1;
      lStack_48 = lVar2 + 1;
      if (lVar4 == 0x10) goto code_r0x000140be2763;
    }
    if ((ulonglong)(lStack_58 - lStack_48) < 3) {
      FUN_1428ca940(&lStack_58,lStack_48,3,1,1);
    }
    *(undefined1 *)(lStack_50 + 2 + lStack_48) = 0x93;
    *(undefined2 *)(lStack_50 + lStack_48) = 0x9de2;
    lStack_48 = lStack_48 + 3;
  } while (lVar4 != 0x10);
code_r0x000140be2763:
  uVar3 = FUN_140d88360(lStack_50,lStack_48,param_2);
  if (lStack_58 != 0) {
    uVar3 = uVar3 & 0xffffffff;
    func_0x000140613c20(lStack_50,lStack_58,1);
  }
  return uVar3;
}

