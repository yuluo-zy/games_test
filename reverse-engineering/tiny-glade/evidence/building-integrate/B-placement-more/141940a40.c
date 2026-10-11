
void __cdecl
utils::curve::Curve::get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>
          (void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  float fVar3;
  ulonglong uVar4;
  longlong in_RCX;
  ulonglong uVar5;
  bool bVar6;
  float fVar7;
  float in_XMM1_Da;
  float in_XMM1_Db;
  float in_XMM2_Da;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  undefined8 *local_f8;
  undefined8 *local_f0;
  undefined8 local_e8;
  float fStack_e0;
  float fStack_dc;
  float fStack_d8;
  ulonglong local_d0;
  undefined8 **local_c8;
  undefined4 local_c0;
  undefined8 local_bc;
  ulonglong local_ac;
  undefined8 uStack_a4;
  float local_9c;
  
  fVar3 = __real_322bcc77;
  fVar7 = __real_3f800000;
  if (*(ulonglong *)(in_RCX + 0x10) < 2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  local_f8 = *(undefined8 **)(in_RCX + 8);
  local_f0 = local_f8 + *(ulonglong *)(in_RCX + 0x10);
  local_e8 = local_e8 & 0xffffffff00000000;
  local_d0 = 0;
  uVar4 = 0;
  uVar5 = 0;
  fVar12 = 0.0;
  fVar13 = __real_7f7fffff;
  fVar10 = in_XMM1_Da;
  do {
    uVar2 = *local_f8;
    if ((uVar4 & 1) == 0) {
      local_c0 = 1;
      local_f8 = local_f8 + 1;
      local_c8 = &local_f8;
      local_bc = uVar2;
      itertools::tuple_impl::impl$31::
      collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec2::Vec2)>_>_>_>
                ();
      fStack_d8 = local_9c;
      local_e8 = local_ac;
      uVar1 = local_e8;
      fStack_e0 = (float)uStack_a4;
      fStack_dc = (float)((ulonglong)uStack_a4 >> 0x20);
      local_e8._0_4_ = (int)local_ac;
      uVar4 = local_ac & 0xffffffff;
      bVar6 = (int)local_e8 != 1;
      local_e8 = uVar1;
      if (bVar6) break;
      local_e8._4_4_ = (float)(local_ac >> 0x20);
      fVar10 = local_e8._4_4_;
    }
    else {
      local_e8 = CONCAT44(fVar10,(int)local_e8);
      fStack_d8 = (float)((ulonglong)uVar2 >> 0x20);
      local_f8 = local_f8 + 1;
      fStack_e0 = in_XMM1_Db;
      fStack_dc = (float)uVar2;
    }
    uVar1 = local_d0 + 1;
    fVar9 = fStack_dc - fVar10;
    fVar11 = fStack_d8 - fStack_e0;
    fVar8 = fVar11 * fVar11 + fVar9 * fVar9;
    if (fVar8 <= fVar3) {
      fVar8 = fVar3;
    }
    fVar8 = ((in_XMM2_Da - fStack_e0) * fVar11 + (in_XMM1_Da - fVar10) * fVar9) / fVar8;
    fVar9 = 0.0;
    if (0.0 <= fVar8) {
      fVar9 = fVar8;
    }
    fVar8 = fVar7;
    if (fVar9 <= fVar7) {
      fVar8 = fVar9;
    }
    fVar10 = in_XMM1_Da - (fVar8 * fStack_dc + (fVar7 - fVar8) * fVar10);
    fVar9 = in_XMM2_Da - (fVar8 * fStack_d8 + (fVar7 - fVar8) * fStack_e0);
    fVar10 = SQRT(fVar9 * fVar9 + fVar10 * fVar10);
    if (fVar10 < fVar13) {
      if ((fVar8 < 0.0) || (fVar7 < fVar8)) {
        local_d0 = uVar1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      uVar5 = local_d0 & 0xffffffff;
      fVar12 = fVar8;
      fVar13 = fVar10;
    }
    fVar10 = fStack_dc;
    in_XMM1_Db = fStack_d8;
    local_d0 = uVar1;
  } while (local_f8 != local_f0);
  if (*(ulonglong *)(in_RCX + 0x28) <= uVar5) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_RCX + 0x28) <= uVar5 + 1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  uVar2 = *(undefined8 *)(*(longlong *)(in_RCX + 0x20) + uVar5 * 4);
  fVar7 = fVar12 * (float)((ulonglong)uVar2 >> 0x20) + (fVar7 - fVar12) * (float)uVar2;
  if (NAN(fVar7)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (ABS(fVar7) != INFINITY) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

