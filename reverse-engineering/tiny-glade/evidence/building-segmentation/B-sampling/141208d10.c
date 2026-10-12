
void __cdecl
utils::curve::impl$12::
sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>
          (void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  longlong lVar6;
  uint uVar7;
  ulonglong uVar8;
  undefined8 *in_RCX;
  longlong *in_RDX;
  int in_R8D;
  int iVar9;
  ulonglong uVar10;
  float fVar11;
  float fVar12;
  float in_XMM3_Da;
  
  uVar5 = *(uint *)(in_RDX + 3);
  if (uVar5 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_const::panic_const_rem_by_zero();
  }
  iVar9 = in_R8D + (int)in_RDX[2];
  if (iVar9 != -0x80000000 || uVar5 != 0xffffffff) {
    uVar7 = -uVar5;
    if ((int)-uVar5 < 0) {
      uVar7 = uVar5;
    }
    uVar10 = (ulonglong)(int)((iVar9 % (int)uVar5 >> 0x1f & uVar7) + iVar9 % (int)uVar5);
    if ((ulonglong)in_RDX[1] <= uVar10) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (uVar5 != 0xffffffff || iVar9 != 0x7fffffff) {
      iVar9 = (iVar9 + 1) % (int)uVar5;
      uVar8 = (ulonglong)(int)((iVar9 >> 0x1f & uVar7) + iVar9);
      if (uVar8 < (ulonglong)in_RDX[1]) {
        lVar6 = *in_RDX;
        uVar3 = *(undefined8 *)(lVar6 + uVar10 * 0xc);
        fVar1 = *(float *)(lVar6 + 8 + uVar10 * 0xc);
        fVar2 = *(float *)(lVar6 + 8 + uVar8 * 0xc);
        uVar4 = *(undefined8 *)(lVar6 + uVar8 * 0xc);
        fVar11 = (float)uVar3;
        fVar12 = (float)((ulonglong)uVar3 >> 0x20);
        *in_RCX = CONCAT44(in_XMM3_Da * ((float)((ulonglong)uVar4 >> 0x20) - fVar12) + fVar12,
                           in_XMM3_Da * ((float)uVar4 - fVar11) + fVar11);
        *(float *)(in_RCX + 1) = (fVar2 - fVar1) * in_XMM3_Da + fVar1;
        return;
      }
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_const::panic_const_rem_overflow();
}

