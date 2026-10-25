
void __cdecl perchance::PerchanceContext::uniform_f32(void)

{
  undefined1 auVar1 [16];
  ulonglong *in_RCX;
  
  auVar1._8_8_ = 0;
  auVar1._0_8_ = *in_RCX;
  in_RCX[1] = in_RCX[1] * 0x4385df649fccf645 +
              SUB168(auVar1 * ZEXT816(0x4385df649fccf645),8) + *in_RCX * 0x2360ed051fc65da4;
  *in_RCX = SUB168(auVar1 * ZEXT816(0x4385df649fccf645),0);
  return;
}

