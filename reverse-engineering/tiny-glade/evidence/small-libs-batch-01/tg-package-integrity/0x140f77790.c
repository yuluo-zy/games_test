
void FUN_140f77790(undefined8 param_1,ulonglong param_2)

{
  byte bVar1;
  byte bVar2;
  
  param_2 = param_2 & 0xffffffffffffff80;
  bVar1 = *(byte *)(param_2 + 0x4a);
  bVar2 = *(byte *)(param_2 + 0x49);
  if (*(char *)(param_2 + 0x48) != '\0') {
    FUN_140f75d40(param_2 + 0xd8);
  }
  if ((bVar1 & 1) != 0) {
    FUN_140f76340(param_2 + 0xc0);
  }
  if ((bVar2 & 1) != 0) {
    FUN_141d41ec0(param_2 + 0x180);
  }
  if ((bVar1 & 1) != 0) {
    FUN_14002e900(*(undefined8 *)(param_2 + 0xa0),*(undefined8 *)(param_2 + 0x88));
  }
  return;
}

