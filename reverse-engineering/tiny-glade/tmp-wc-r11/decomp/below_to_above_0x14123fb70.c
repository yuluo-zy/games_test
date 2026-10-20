// name: below_to_above
// addr: 0x14123fb70
// size: 247
// signature: void __cdecl below_to_above(void)


void __cdecl system_wall_constructor::utils::trim_rows::impl$0::below_to_above(void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *in_RCX;
  longlong lVar5;
  ulonglong in_RDX;
  undefined4 extraout_XMM0_Da;
  float in_XMM2_Da;
  
  if ((in_XMM2_Da < 0.0) || (1.0 < in_XMM2_Da)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (*(ulonglong *)(*in_RCX + 0x28) <= (in_RDX & 0xffffffff)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if ((in_RDX & 0xffffffff) + 1 < *(ulonglong *)(*in_RCX + 0x28)) {
    ::utils::lerp();
    plVar2 = (longlong *)in_RCX[1];
    lVar1 = in_RCX[3];
    lVar3 = plVar2[2];
    if (lVar3 == *plVar2) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar4 = plVar2[1];
    lVar5 = lVar3 * 0x20;
    *(undefined8 *)(lVar4 + lVar5) = 0;
    ((undefined8 *)(lVar4 + lVar5))[1] = 0;
    *(longlong *)(lVar4 + 0x10 + lVar5) = lVar1;
    *(undefined4 *)(lVar4 + 0x18 + lVar5) = extraout_XMM0_Da;
    *(undefined4 *)(lVar4 + 0x1c + lVar5) = 0x3f800000;
    plVar2[2] = lVar3 + 1;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

