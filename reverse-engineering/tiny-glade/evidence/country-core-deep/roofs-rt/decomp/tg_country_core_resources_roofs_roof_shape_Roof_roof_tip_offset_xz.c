
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_1408e3780(longlong param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 uStack_28;
  float fStack_18;
  float fStack_14;
  float fStack_10;
  float fStack_c;
  
  if ((*(byte *)(param_1 + 8) & 1) == 0) {
    fVar2 = *(float *)(param_1 + 0x14) + *(float *)(param_1 + 0x14);
    fVar1 = fVar2 * (float)*(undefined8 *)(param_1 + 0x24) * _DAT_1429258e0;
    fVar2 = fVar2 * (float)((ulonglong)*(undefined8 *)(param_1 + 0x24) >> 0x20) * _UNK_1429258e4;
  }
  else {
    uStack_28 = *(undefined8 *)(param_1 + 0x1c);
    uStack_38 = *(undefined4 *)(param_1 + 0xc);
    uStack_34 = *(undefined4 *)(param_1 + 0x10);
    uStack_30 = *(undefined4 *)(param_1 + 0x14);
    uStack_2c = *(undefined4 *)(param_1 + 0x18);
    func_0x000140c99540(&fStack_18,&uStack_38);
    fVar2 = (float)*(undefined8 *)(param_1 + 0x24) * (float)*(undefined8 *)(param_1 + 0x1c);
    fVar3 = (float)((ulonglong)*(undefined8 *)(param_1 + 0x24) >> 0x20) *
            (float)((ulonglong)*(undefined8 *)(param_1 + 0x1c) >> 0x20);
    fVar1 = fVar3 * _UNK_14293ac68 * fStack_10 + fVar2 * _DAT_14293ac60 * fStack_18;
    fVar2 = fVar3 * _UNK_14293ac6c * fStack_c + fVar2 * _UNK_14293ac64 * fStack_14;
  }
  return CONCAT44(fVar2,fVar1);
}

