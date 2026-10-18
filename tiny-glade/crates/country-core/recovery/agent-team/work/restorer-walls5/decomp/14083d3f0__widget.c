// 0x14083d3f0 widget size=1560 sig=void __cdecl widget(void)
void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::
UiRectangleOrientationEdit::widget(void)

{
  ulonglong uVar1;
  byte bVar2;
  uint extraout_EAX;
  longlong in_RCX;
  longlong in_R8;
  longlong in_R9;
  ulonglong uVar3;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  ulonglong in_stack_00000028;
  byte in_stack_00000030;
  float local_1e4;
  undefined8 local_1e0;
  float local_1d8;
  float fStack_1d4;
  float local_1d0;
  float fStack_1cc;
  float local_1c8;
  float local_1c4;
  float local_1c0 [2];
  undefined1 local_1b8 [8];
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 local_170;
  undefined4 local_168;
  undefined4 uStack_164;
  int local_158;
  undefined8 local_148;
  int local_140;
  int local_128;
  undefined1 local_110;
  undefined1 local_100;
  undefined8 local_ff;
  undefined7 uStack_f7;
  undefined1 uStack_f0;
  undefined7 uStack_ef;
  undefined8 uStack_e8;
  byte local_e0;
  undefined1 local_df;
  undefined7 uStack_d7;
  undefined1 local_d0;
  undefined7 uStack_cf;
  undefined1 uStack_c8;
  undefined7 uStack_c7;
  undefined8 local_c0;
  
  ::utils::geometry::rectangle::Rectangle2d::get_point();
  fVar5 = extraout_XMM0_Da;
  ::utils::curve::Curve::
  get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>();
  ::utils::geometry::rectangle::Rectangle2d::get_point();
  uVar3 = (ulonglong)extraout_EAX;
  if (*(ulonglong *)(in_R8 + 0x98) <= uVar3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  uVar1 = uVar3 + 1;
  if (*(ulonglong *)(in_R8 + 0x98) <= uVar1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  fVar12 = *(float *)(*(longlong *)(in_R8 + 0x90) + 4 + uVar1 * 0xc) * extraout_XMM0_Da_00 +
           (1.0 - extraout_XMM0_Da_00) * *(float *)(*(longlong *)(in_R8 + 0x90) + 4 + uVar3 * 0xc) +
           1.0;
  *(float *)(in_RCX + 0x28) = fVar12;
  local_1e4 = fVar12;
  if (*(char *)(in_RCX + 0x2c) == '\0') {
    if (in_stack_00000028 <= uVar3) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (in_stack_00000028 <= uVar1) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    fVar8 = *(float *)(in_R9 + uVar3 * 4);
    fVar8 = (((*(float *)(in_R9 + 4 + uVar3 * 4) - fVar8) * extraout_XMM0_Da_00 + fVar8) - fVar12) *
            0.5;
  }
  else {
    fVar8 = 0.25;
  }
  fVar6 = fVar5;
  ::utils::geometry::rectangle::Rectangle2d::get_corner_normal();
  if (*(char *)(in_RCX + 0x2c) != '\0') {
    ::utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id();
    local_1d8 = (float)((ulonglong)local_1b0 >> 0x20);
    local_1e0 = local_1b0;
    fStack_1d4 = -(float)local_1b0;
    fStack_1cc = (float)((ulonglong)local_1a8 >> 0x20);
    local_1c8 = -fStack_1cc;
    local_1d0 = (float)local_1a8;
    uStack_180 = local_1c0;
    local_188 = &local_1e0;
    local_178 = local_1b8;
    local_170 = &local_1e4;
    local_168 = 0;
    uStack_164 = 0;
    local_148 = 0;
    local_1c4 = local_1d0;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl_1::widget::closure_env_0>_>
              ();
    goto LAB_14083d879;
  }
  if (in_stack_00000028 <= uVar3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (in_stack_00000028 <= uVar1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  ::utils::geometry::rectangle::Rectangle2d::orientation_mat3();
  if (local_1c0[0] <= 0.0) {
    fVar7 = local_1d0 - (float)local_1e0;
    if (0.0 < fVar7) {
      fVar9 = fStack_1d4 + local_1e0._4_4_;
      fVar10 = (1.0 - local_1c0[0]) + fVar7;
      fVar11 = local_1c4 + fStack_1cc;
      fVar7 = local_1c8 - local_1d8;
      fVar4 = fVar10;
    }
    else {
      fVar10 = fStack_1d4 + local_1e0._4_4_;
      fVar11 = local_1c8 + local_1d8;
      fVar9 = (1.0 - local_1c0[0]) - fVar7;
      fVar7 = fStack_1cc - local_1c4;
      fVar4 = fVar9;
    }
LAB_14083d80a:
    fVar4 = 0.5 / SQRT(fVar4);
    fVar9 = fVar4 * fVar9;
    fVar10 = fVar4 * fVar10;
    fVar11 = fVar4 * fVar11;
    fVar4 = fVar4 * fVar7;
  }
  else {
    fVar7 = (float)local_1e0 + local_1d0;
    if (0.0 < fVar7) {
      fVar9 = fStack_1cc - local_1c4;
      fVar10 = local_1c8 - local_1d8;
      fVar11 = local_1e0._4_4_ - fStack_1d4;
      fVar7 = local_1c0[0] + 1.0 + fVar7;
      fVar4 = fVar7;
      goto LAB_14083d80a;
    }
    fVar7 = (local_1c0[0] + 1.0) - fVar7;
    fVar4 = 0.5 / SQRT(fVar7);
    fVar9 = fVar4 * (local_1d8 + local_1c8);
    fVar10 = fVar4 * (local_1c4 + fStack_1cc);
    fVar11 = fVar4 * fVar7;
    fVar4 = fVar4 * (local_1e0._4_4_ - fStack_1d4);
  }
  local_178 = (undefined1 *)CONCAT44(fVar8 + fVar12,extraout_XMM0_Da_01 - extraout_XMM0_Da_02 * 0.3)
  ;
  local_188 = (undefined8 *)CONCAT44(fVar10,fVar9);
  uStack_180 = (float *)CONCAT44(fVar4,fVar11);
  local_170._4_4_ = 0x3f800000;
  local_168 = 0x3f800000;
  uStack_164 = 0x3f800000;
  local_170._0_4_ = fVar5 - fVar6 * 0.3;
  ui_edit_rectangle_orientation::box_trigger_volume();
LAB_14083d879:
  local_c0 = local_190;
  local_d0 = (undefined1)local_1a0;
  uStack_cf = (undefined7)((ulonglong)local_1a0 >> 8);
  uStack_c8 = (undefined1)uStack_198;
  uStack_c7 = (undefined7)((ulonglong)uStack_198 >> 8);
  bVar2 = in_stack_00000030 ^ 1;
  local_158 = (uint)bVar2 + (uint)bVar2;
  local_140 = (uint)bVar2 + (uint)bVar2 * 2;
  local_e0 = *(byte *)(in_RCX + 0x2c) ^ 1;
  local_188 = (undefined8 *)0x0;
  uStack_180 = (float *)0x8;
  local_178 = (undefined1 *)0x0;
  local_170._0_4_ = (float)(uint)bVar2;
  local_168 = 0;
  uStack_164 = 0;
  local_110 = 0;
  local_100 = 2;
  local_ff = CONCAT17(local_d0,uStack_d7);
  uStack_f7 = uStack_cf;
  uStack_e8 = local_190;
  local_df = 1;
  local_128 = local_140;
  uStack_f0 = uStack_c8;
  uStack_ef = uStack_c7;
  systems::ui::resources::ui_state::UiState::
  add<ref_<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit>_>
            ();
  return;
}
