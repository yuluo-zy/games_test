
void __cdecl utils::geometry::circle::Circle2d::as_curve2(void)

{
  ulonglong uVar1;
  longlong *in_RCX;
  ulonglong uVar2;
  longlong in_RDX;
  float fVar3;
  float in_XMM2_Da;
  longlong local_50;
  longlong lStack_48;
  longlong local_40;
  
  fVar3 = ceilf((*(float *)(in_RDX + 8) * __real_40c90fdb) / in_XMM2_Da);
  uVar2 = (longlong)(fVar3 - __real_5f000000) & (longlong)fVar3 >> 0x3f | (longlong)fVar3;
  if (fVar3 < 0.0) {
    uVar2 = 0;
  }
  uVar1 = 0xffffffffffffffff;
  if (fVar3 <= __real_5f7fffff) {
    uVar1 = uVar2;
  }
  uVar2 = 3;
  if (3 < uVar1) {
    uVar2 = uVar1;
  }
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>_>
            ();
  curve::Curve::try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (!SBORROW8(0,in_RDX)) {
    in_RCX[6] = local_40;
    in_RCX[4] = local_50;
    in_RCX[5] = lStack_48;
    in_RCX[2] = 0;
    in_RCX[3] = uVar2;
    *in_RCX = in_RDX;
    in_RCX[1] = uVar2;
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
}

