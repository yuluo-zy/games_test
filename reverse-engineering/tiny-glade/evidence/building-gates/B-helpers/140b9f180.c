
void __cdecl alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>(void)

{
  ulonglong uVar1;
  undefined1 auVar2 [16];
  ulonglong *in_RCX;
  ulonglong uVar3;
  longlong *plVar4;
  longlong lVar5;
  ulonglong uVar6;
  int local_48;
  ulonglong local_40;
  longlong local_28;
  longlong local_20;
  
  uVar1 = *in_RCX;
  uVar3 = uVar1 * 2;
  if (uVar1 * 2 < uVar1 + 1) {
    uVar3 = uVar1 + 1;
  }
  uVar6 = 4;
  if (4 < uVar3) {
    uVar6 = uVar3;
  }
  auVar2._8_8_ = 0;
  auVar2._0_8_ = uVar6;
  if (SUB168(auVar2 * ZEXT816(0xc),8) != 0) {
                    /* WARNING: Subroutine does not return */
    handle_error();
  }
  if (SUB168(auVar2 * ZEXT816(0xc),0) < 0x7ffffffffffffffd) {
    if (uVar1 == 0) {
      plVar4 = &local_28;
      lVar5 = 0;
    }
    else {
      plVar4 = &local_20;
      lVar5 = uVar1 * 0xc;
      local_28 = 4;
    }
    *plVar4 = lVar5;
    finish_grow<alloc::alloc::Global>();
    if (local_48 != 1) {
      in_RCX[1] = local_40;
      *in_RCX = uVar6;
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  handle_error();
}

