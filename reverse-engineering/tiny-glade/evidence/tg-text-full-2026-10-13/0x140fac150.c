
void FUN_140fac150(longlong param_1,undefined8 param_2)

{
  code *pcVar1;
  longlong lVar2;
  float fVar3;
  float fStack_28;
  float fStack_24;
  undefined1 auStack_20 [16];
  undefined4 uStack_10;
  undefined4 uStack_c;
  
  fVar3 = (float)param_2;
  fStack_28 = (float)FUN_140b906c0(param_2);
  if ((((fStack_28 == *(float *)(param_1 + 0x240)) &&
       (!NAN(fStack_28) && !NAN(*(float *)(param_1 + 0x240)))) &&
      (fVar3 == *(float *)(param_1 + 0x244))) && (!NAN(fVar3) && !NAN(*(float *)(param_1 + 0x244))))
  {
    return;
  }
  fStack_24 = fVar3;
  lVar2 = FUN_140b91fa0(*(longlong *)(param_1 + 0x230) + 0x10,*(undefined1 *)(param_1 + 0x255));
  if (lVar2 != 0) {
    func_0x000140b92090(auStack_20,lVar2,&fStack_28);
    FUN_140b90780(param_1,uStack_10,uStack_c);
    *(ulonglong *)(param_1 + 0x240) = CONCAT44(fStack_24,fStack_28);
    return;
  }
  FUN_1428d9310(&UNK_142bb0eb8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

