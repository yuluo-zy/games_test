
uint FUN_140bd4b00(uint param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar2 = (param_1 & 6) << 5 | (param_1 & 1) << 8;
  uVar1 = uVar2 + 0x20 + (param_1 & 8) * 2;
  if ((param_1 & 0x18060) == 0) {
    uVar1 = uVar2 + (param_1 & 8) * 2;
  }
  return uVar1 | param_1 >> 0xb & 3;
}

