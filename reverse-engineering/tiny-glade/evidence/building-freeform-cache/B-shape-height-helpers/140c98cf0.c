
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::geometry::rectangle::Rectangle2d::as_curve2(void)

{
  undefined8 *extraout_RAX;
  longlong *in_RCX;
  undefined8 *in_RDX;
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  longlong local_e0;
  longlong lStack_d8;
  longlong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  longlong lStack_b8;
  longlong local_b0;
  
  fVar4 = *(float *)(in_RDX + 2);
  fVar7 = *(float *)((longlong)in_RDX + 0x14);
  if (fVar4 * fVar7 <= 0.0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar8 = (float)*in_RDX;
  fVar9 = (float)((ulonglong)*in_RDX >> 0x20);
  fVar5 = (float)in_RDX[1];
  fVar6 = (float)((ulonglong)in_RDX[1] >> 0x20);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    fVar1 = (float)((uint)fVar9 ^ ___xmm_80000000800000008000000080000000);
    fVar3 = fVar4 * __real_bf000000;
    fVar2 = __real_bf000000 * fVar7;
    fVar10 = fVar2 * fVar1 + fVar3 * fVar8 + fVar5;
    fVar11 = fVar2 * fVar8 + fVar3 * fVar9 + fVar6;
    fVar4 = fVar4 * __real_3f000000;
    fVar7 = fVar7 * __real_3f000000;
    *extraout_RAX = CONCAT44(fVar11,fVar10);
    *(float *)(extraout_RAX + 1) = fVar3 * fVar8 + fVar7 * fVar1 + fVar5;
    *(float *)((longlong)extraout_RAX + 0xc) = fVar3 * fVar9 + fVar7 * fVar8 + fVar6;
    *(float *)(extraout_RAX + 2) = fVar4 * fVar8 + fVar7 * fVar1 + fVar5;
    *(float *)((longlong)extraout_RAX + 0x14) = fVar4 * fVar9 + fVar7 * fVar8 + fVar6;
    extraout_RAX[3] =
         CONCAT44(fVar2 * fVar8 + fVar4 * fVar9 + fVar6,fVar2 * fVar1 + fVar4 * fVar8 + fVar5);
    alloc::raw_vec::RawVec::
    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
              ();
    extraout_RAX[4] = CONCAT44(fVar11,fVar10);
    curve::Curve::try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    if (!SBORROW8(0,local_e0)) {
      in_RCX[6] = local_b0;
      in_RCX[4] = local_c0;
      in_RCX[5] = lStack_b8;
      in_RCX[2] = local_d0;
      in_RCX[3] = lStack_c8;
      *in_RCX = local_e0;
      in_RCX[1] = lStack_d8;
      return;
    }
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

