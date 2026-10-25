
void __cdecl
utils::curve::Curve::crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>(void)

{
  longlong *in_RCX;
  longlong local_80;
  longlong lStack_78;
  longlong local_70;
  longlong lStack_68;
  longlong local_60;
  longlong lStack_58;
  longlong local_50;
  
  slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
            ();
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (!SBORROW8(0,local_80)) {
    in_RCX[6] = local_50;
    in_RCX[4] = local_60;
    in_RCX[5] = lStack_58;
    in_RCX[2] = local_70;
    in_RCX[3] = lStack_68;
    *in_RCX = local_80;
    in_RCX[1] = lStack_78;
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
}

