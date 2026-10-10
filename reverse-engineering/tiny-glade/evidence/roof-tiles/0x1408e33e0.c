
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tiles(undefined8 *param_1,longlong param_2)

{
  byte bVar1;
  undefined8 uVar2;
  
  uVar2 = 0;
  if (((*(float *)(param_2 + 0x34) == _DAT_142925904) &&
      (!NAN(*(float *)(param_2 + 0x34)) && !NAN(_DAT_142925904))) &&
     (_DAT_142925984 < *(float *)(param_2 + 0x30))) {
    bVar1 = *(byte *)(param_2 + 0x3c);
    param_1[1] = (ulonglong)bVar1;
    param_1[2] = (ulonglong)bVar1 | 2;
    uVar2 = 1;
  }
  *param_1 = uVar2;
  return;
}

