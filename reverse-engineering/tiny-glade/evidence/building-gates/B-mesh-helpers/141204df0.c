
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32>_>,system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env_0>_>
          (void)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 auVar5 [16];
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong extraout_RAX;
  ulonglong *in_RCX;
  longlong *in_RDX;
  longlong lVar8;
  longlong lVar9;
  ulonglong uVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  
  lVar9 = in_RDX[4];
  lVar3 = in_RDX[5];
  uVar10 = lVar3 - lVar9;
  auVar5._8_8_ = 0;
  auVar5._0_8_ = uVar10;
  uVar7 = SUB168(auVar5 * ZEXT816(0xc),0);
  if ((SUB168(auVar5 * ZEXT816(0xc),8) == 0) && (uVar7 < 0x7ffffffffffffffd)) {
    if (uVar7 == 0) {
      uVar7 = 4;
      uVar6 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar7 = extraout_RAX;
      uVar6 = uVar10;
      if (extraout_RAX == 0) goto LAB_141204f90;
    }
    if (lVar3 == lVar9) {
      uVar10 = 0;
    }
    else {
      lVar4 = in_RDX[2];
      lVar1 = *in_RDX + lVar9 * 0xc;
      lVar8 = 2;
      do {
        fVar13 = *(float *)(lVar4 + lVar9 * 4);
        fVar11 = fVar13 * 0.0;
        uVar2 = *(undefined8 *)(lVar1 + -8 + lVar8 * 4);
        fVar12 = fVar11 + (float)uVar2;
        fVar13 = fVar13 + (float)((ulonglong)uVar2 >> 0x20);
        fVar11 = fVar11 + *(float *)(lVar1 + lVar8 * 4);
        if (((0x7f7fffff < (uint)ABS(fVar12)) || (0x7f7fffff < (uint)ABS(fVar13))) ||
           (0x7f7fffff < (uint)ABS(fVar11))) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        *(ulonglong *)((uVar7 - 8) + lVar8 * 4) = CONCAT44(fVar13,fVar12);
        *(float *)(uVar7 + lVar8 * 4) = fVar11;
        lVar9 = lVar9 + 1;
        lVar8 = lVar8 + 3;
      } while (lVar3 != lVar9);
    }
    *in_RCX = uVar6;
    in_RCX[1] = uVar7;
    in_RCX[2] = uVar10;
    return;
  }
LAB_141204f90:
                    /* WARNING: Subroutine does not return */
  raw_vec::handle_error();
}

