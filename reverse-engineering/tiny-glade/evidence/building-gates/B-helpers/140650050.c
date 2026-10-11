
void __cdecl alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>(void)

{
  ulonglong *in_RCX;
  ulonglong uVar1;
  ulonglong uVar2;
  int local_48;
  ulonglong local_40;
  
  uVar1 = *in_RCX + 1;
  uVar2 = *in_RCX * 2;
  if (uVar2 < uVar1) {
    uVar2 = uVar1;
  }
  uVar1 = 4;
  if (4 < uVar2) {
    uVar1 = uVar2;
  }
  if (uVar2 >> 0x3e == 0) {
    if ((uVar1 * 4 < 0x7ffffffffffffffd) && (finish_grow<alloc::alloc::Global>(), local_48 != 1)) {
      in_RCX[1] = local_40;
      *in_RCX = uVar1;
      return;
    }
                    /* WARNING: Subroutine does not return */
    handle_error();
  }
                    /* WARNING: Subroutine does not return */
  handle_error();
}

