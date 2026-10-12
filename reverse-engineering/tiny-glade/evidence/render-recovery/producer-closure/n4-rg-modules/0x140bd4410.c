
void FUN_140bd4410(longlong param_1)

{
  ulonglong uVar1;
  ulonglong uVar2;
  
  *(undefined8 *)(param_1 + 0x10) = 0;
  *(undefined8 *)(param_1 + 0x28) = 0;
  *(undefined8 *)(param_1 + 0x40) = 0;
  *(undefined8 *)(param_1 + 0x58) = 0;
  *(undefined8 *)(param_1 + 0x70) = 0;
  if (*(longlong *)(param_1 + 0xa8) != 0) {
    uVar1 = *(ulonglong *)(param_1 + 0x98);
    if (uVar1 != 0) {
      func_0x000142923c20(*(undefined8 *)(param_1 + 0x90),0xff,uVar1 + 0x11);
    }
    *(undefined8 *)(param_1 + 0xa8) = 0;
    uVar2 = (uVar1 + 1 & 0xfffffffffffffff8) - (uVar1 + 1 >> 3);
    if (uVar1 < 8) {
      uVar2 = uVar1;
    }
    *(ulonglong *)(param_1 + 0xa0) = uVar2;
  }
  *(undefined8 *)(param_1 + 0x88) = 0;
  return;
}

