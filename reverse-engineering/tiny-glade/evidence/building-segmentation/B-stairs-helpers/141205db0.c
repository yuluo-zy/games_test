
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32>_>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env_6>_>
          (void)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  longlong lVar5;
  longlong lVar6;
  undefined1 auVar7 [16];
  float fVar8;
  float fVar9;
  ulonglong uVar10;
  ulonglong extraout_RAX;
  ulonglong *in_RCX;
  longlong *in_RDX;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  float fVar15;
  float extraout_XMM0_Da;
  float fVar16;
  float fVar17;
  float fVar18;
  ulonglong local_d0;
  
  lVar13 = in_RDX[4];
  lVar5 = in_RDX[5];
  uVar12 = lVar5 - lVar13;
  auVar7._8_8_ = 0;
  auVar7._0_8_ = uVar12;
  uVar10 = SUB168(auVar7 * ZEXT816(0xc),0);
  if ((SUB168(auVar7 * ZEXT816(0xc),8) == 0) && (uVar10 < 0x7ffffffffffffffd)) {
    if (uVar10 == 0) {
      local_d0 = 4;
      uVar10 = 0;
      fVar8 = __real_3f0ccccd;
      fVar9 = __real_40400000;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar10 = uVar12;
      local_d0 = extraout_RAX;
      fVar8 = __real_3f0ccccd;
      fVar9 = __real_40400000;
      if (extraout_RAX == 0) goto LAB_141205fa4;
    }
    __real_40400000 = fVar9;
    __real_3f0ccccd = fVar8;
    if (lVar5 == lVar13) {
      uVar12 = 0;
    }
    else {
      lVar6 = in_RDX[2];
      lVar1 = *in_RDX + lVar13 * 0xc;
      lVar11 = 2;
      do {
        fVar15 = *(float *)(lVar6 + lVar13 * 4);
        fVar2 = *(float *)(lVar1 + -8 + lVar11 * 4);
        fVar3 = *(float *)(lVar1 + -4 + lVar11 * 4);
        fVar4 = *(float *)(lVar1 + lVar11 * 4);
        country_core::resources::walls::public_wall_state::PublicWallState::max_y();
        fVar16 = fVar15 * 0.0;
        fVar18 = fVar3;
        if (fVar3 <= fVar8) {
          fVar18 = fVar8;
        }
        fVar17 = fVar16;
        if (fVar9 <= extraout_XMM0_Da - fVar18) {
          fVar17 = fVar15;
        }
        uVar14 = -(uint)(extraout_XMM0_Da - fVar18 < fVar9);
        fVar15 = (float)(~uVar14 & (uint)fVar16 | (uint)(fVar16 * 0.0) & uVar14);
        *(float *)((local_d0 - 8) + lVar11 * 4) = fVar2 + fVar15;
        *(float *)((local_d0 - 4) + lVar11 * 4) = fVar3 + fVar17;
        *(float *)(local_d0 + lVar11 * 4) = fVar4 + fVar15;
        lVar13 = lVar13 + 1;
        lVar11 = lVar11 + 3;
      } while (lVar5 != lVar13);
    }
    *in_RCX = uVar10;
    in_RCX[1] = local_d0;
    in_RCX[2] = uVar12;
    return;
  }
LAB_141205fa4:
                    /* WARNING: Subroutine does not return */
  raw_vec::handle_error();
}

