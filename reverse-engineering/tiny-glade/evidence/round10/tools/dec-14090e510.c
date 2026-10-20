
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14090e510(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined8 uStack_80;
  float fStack_78;
  undefined8 uStack_74;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  float fStack_60;
  float fStack_50;
  float fStack_4c;
  undefined8 auStack_40 [5];
  
  uStack_80 = _UNK_142aab390;
  uVar1 = FUN_1409e9640(param_3,_UNK_142aab3a0,_UNK_142aab3a4);
  func_0x000140c9cde0(&uStack_68,&uStack_80,uVar1);
  fVar4 = (float)uStack_68;
  fVar2 = uStack_68._4_4_;
  fStack_78 = fStack_60 + 0.0;
  uStack_80 = CONCAT44(uStack_68._4_4_ + _UNK_142aab3b4,(float)uStack_68 + _UNK_142aab3b0);
  uStack_74 = _UNK_1429ddd00;
  uStack_6c = 0x80000000;
  FUN_1408ce180(&uStack_68,param_2,&uStack_80,0);
  if ((char)uStack_68 != '\x11') {
    fVar2 = fStack_4c;
    fVar4 = fStack_50;
  }
  fVar3 = fVar2 + _UNK_142a8e4f8;
  if (fVar2 + _UNK_142a8e4f8 <= _UNK_142925970) {
    fVar3 = _UNK_142925970;
  }
  uStack_80 = CONCAT44(fVar3,fVar4);
  auStack_40[0] = _UNK_142aab3c0;
  func_0x000140c9cdc0(&uStack_68,auStack_40);
  fVar2 = (float)uStack_68;
  fVar3 = (float)((ulonglong)uStack_68 >> 0x20);
  fVar5 = _UNK_142925904 / SQRT(fStack_60 * fStack_60 + fVar3 * fVar3 + fVar2 * fVar2);
  uStack_68 = _UNK_1429cd330;
  fVar4 = _UNK_14295a0e4;
  if ((int)fVar5 - 1U < 0x7fffff || (int)ABS(fVar5) - 0x800000U < 0x7f000000 && -1 < (int)fVar5) {
    uStack_68 = CONCAT44(fVar3 * fVar5,fVar2 * fVar5);
    fVar4 = fStack_60 * fVar5;
  }
  fStack_60 = fVar4;
  FUN_140994130(param_1 + 0x2f0,&uStack_80,&uStack_68);
  return;
}

