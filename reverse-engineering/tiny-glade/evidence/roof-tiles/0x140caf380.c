
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float tiles(ulonglong *param_1)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  ulonglong uVar3;
  
  uVar3 = *param_1 + 0xa0761d6478bd642f;
  *param_1 = uVar3;
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uVar3 ^ 0xe7037ed1a0b428db;
  auVar2._8_8_ = 0;
  auVar2._0_8_ = uVar3;
  return (float)((SUB164(auVar1 * auVar2,8) ^ SUB164(auVar1 * auVar2,0)) >> 9 | 0x3f800000) +
         _DAT_14295a0e4;
}

