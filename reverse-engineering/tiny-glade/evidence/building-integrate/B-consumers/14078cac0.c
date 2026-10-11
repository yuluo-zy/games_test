
void __cdecl
alloc::raw_vec::RawVec::
grow_one<country_core::geometry::instanced_wall::BrickTransformSsbo,alloc::alloc::Global>(void)

{
  undefined1 auVar1 [16];
  ulonglong *in_RCX;
  ulonglong uVar2;
  ulonglong uVar3;
  int local_48;
  ulonglong local_40;
  
  uVar3 = *in_RCX + 1;
  uVar2 = *in_RCX * 2;
  if (uVar2 < uVar3) {
    uVar2 = uVar3;
  }
  uVar3 = 4;
  if (4 < uVar2) {
    uVar3 = uVar2;
  }
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uVar3;
  if (SUB168(auVar1 * ZEXT816(0x60),8) == 0) {
    if ((SUB168(auVar1 * ZEXT816(0x60),0) < 0x7ffffffffffffffd) &&
       (finish_grow<alloc::alloc::Global>(), local_48 != 1)) {
      in_RCX[1] = local_40;
      *in_RCX = uVar3;
      return;
    }
                    /* WARNING: Subroutine does not return */
    handle_error();
  }
                    /* WARNING: Subroutine does not return */
  handle_error();
}

