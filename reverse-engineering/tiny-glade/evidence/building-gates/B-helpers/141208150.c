
void __cdecl
core::iter::adapters::map::impl$2::
fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl_20::extend_trusted::closure_env_0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env_0>_>_>_>
          (void)

{
  float fVar1;
  float fVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  float *pfVar6;
  float *pfVar7;
  longlong lVar8;
  float _Y;
  ulonglong uVar9;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong lVar10;
  float fVar11;
  
  _Y = __real_3f800000;
  lVar3 = *in_RCX;
  lVar4 = in_RCX[1];
  plVar5 = (longlong *)*in_RDX;
  lVar10 = in_RDX[1];
  if (lVar3 != lVar4) {
    pfVar6 = (float *)in_RCX[2];
    pfVar7 = (float *)in_RCX[3];
    lVar8 = in_RDX[2];
    uVar9 = 0;
    do {
      fVar11 = *(float *)(lVar3 + uVar9 * 4);
      fVar1 = *pfVar7;
      fVar2 = *pfVar6;
      if (fVar2 <= fVar1) {
        fVar11 = fVar11 * fVar1 + (_Y - fVar11) * fVar2;
        if (NAN(fVar11)) {
                    /* WARNING: Subroutine does not return */
          panicking::panic();
        }
      }
      else {
        fVar11 = fVar11 * (fVar1 + _Y) + (_Y - fVar11) * fVar2;
        if (NAN(fVar11)) {
                    /* WARNING: Subroutine does not return */
          panicking::panic();
        }
      }
      fVar11 = fmodf(fVar11,_Y);
      fVar11 = (float)(~-(uint)(fVar11 < 0.0) & (uint)fVar11 |
                      (uint)(fVar11 + _Y) & -(uint)(fVar11 < 0.0));
      if ((fVar11 < 0.0) || (_Y < fVar11)) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_fmt();
      }
      if (NAN(fVar11)) {
                    /* WARNING: Subroutine does not return */
        panicking::panic();
      }
      *(float *)(lVar8 + lVar10 * 4 + uVar9 * 4) = fVar11;
      uVar9 = uVar9 + 1;
    } while ((ulonglong)(lVar4 - lVar3) >> 2 != uVar9);
    lVar10 = lVar10 + uVar9;
  }
  *plVar5 = lVar10;
  return;
}

