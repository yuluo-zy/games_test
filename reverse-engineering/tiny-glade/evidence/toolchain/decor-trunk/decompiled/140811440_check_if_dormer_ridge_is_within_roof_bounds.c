// ADDR 0x140811440
// SIG void __cdecl check_if_dormer_ridge_is_within_roof_bounds(void)
// ORIG system::check_if_dormer_ridge_is_within_roof_bounds


void __cdecl
system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds(void)

{
  longlong lVar1;
  ulonglong extraout_RAX;
  ulonglong uVar2;
  undefined1 *in_RCX;
  undefined8 *in_RDX;
  longlong lVar3;
  float extraout_XMM0_Da;
  float fVar4;
  float in_XMM2_Da;
  float fVar5;
  float in_XMM3_Da;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  char in_stack_00000028;
  undefined8 in_stack_00000038;
  longlong in_stack_00000040;
  longlong in_stack_00000048;
  float local_114;
  float local_10c;
  int local_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  
  if (1 < (byte)(in_stack_00000028 - 7U)) {
LAB_140811730:
    *in_RCX = 0xb;
    return;
  }
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (((extraout_RAX >> 0x20 != 0) &&
      (uVar2 = extraout_RAX & 0xffffffff, uVar2 < *(ulonglong *)(in_stack_00000048 + 0x10))) &&
     (*(int *)(*(longlong *)(in_stack_00000048 + 8) + uVar2 * 0x14) == (int)(extraout_RAX >> 0x20)))
  {
    lVar3 = *(longlong *)(in_stack_00000048 + 8) + uVar2 * 0x14;
    uVar2 = (ulonglong)*(uint *)(lVar3 + 4);
    if (((uVar2 != 0xffffffff) && (uVar2 < *(ulonglong *)(in_stack_00000040 + 0x38))) &&
       ((*(ulonglong *)
          (*(longlong *)(in_stack_00000040 + 0x28) + (ulonglong)(*(uint *)(lVar3 + 4) >> 6) * 8) >>
         (uVar2 & 0x3f) & 1) != 0)) {
      lVar1 = *(longlong *)
               (*(longlong *)
                 (*(longlong *)(in_stack_00000048 + 0x1a8) + 0x18 +
                 (ulonglong)*(uint *)(lVar3 + 0xc) * 0x48) + 0x10 +
               ~*(ulonglong *)
                 (*(longlong *)
                   (*(longlong *)(in_stack_00000048 + 0x1a8) + 0x38 +
                   (ulonglong)*(uint *)(lVar3 + 0xc) * 0x48) +
                 *(longlong *)(in_stack_00000040 + 0x110) * 8) * 0x30);
      lVar3 = (ulonglong)*(uint *)(lVar3 + 0x10) * 0x58;
      if (*(int *)(lVar1 + 8 + lVar3) != 0) {
        if (in_stack_00000028 == '\a') {
          country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve();
          if (CONCAT44(uStack_cc,uStack_d0) == 0) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
        }
        else {
          decorator_visual::gothic_dormer::gothic_dormer_profile_curve();
          if (CONCAT44(uStack_cc,uStack_d0) == 0) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
        }
        fVar9 = *(float *)(CONCAT44(uStack_d4,uStack_d8) + 4);
        if (CONCAT44(uStack_dc,local_e0) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        fVar7 = fVar9 * 0.0;
        fVar8 = fVar7 + (float)*in_RDX;
        fVar9 = fVar9 + (float)((ulonglong)*in_RDX >> 0x20);
        fVar7 = fVar7 + *(float *)(in_RDX + 1);
        country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y();
        if (local_e0 != 2) {
          if (local_e0 == 0) {
            ::utils::geometry::circle::Circle2d::closest_point_on_perimeter();
          }
          else {
            ::utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter();
          }
          ::utils::swizzlers::impl$1::xvy();
          fVar4 = *(float *)(lVar1 + lVar3 + 0x4c);
          country_core::resources::roofs::roof_shape::Roof::height();
          if (fVar9 <= extraout_XMM0_Da + fVar4 + -0.1) {
            fVar4 = fVar8 - local_114;
            fVar5 = fVar7 - local_10c;
            fVar6 = 1.0 / SQRT(fVar5 * fVar5 + fVar4 * fVar4);
            if (0.993 <= fVar5 * fVar6 * in_XMM3_Da + fVar6 * fVar4 * in_XMM2_Da)
            goto LAB_140811730;
          }
        }
        ::utils::swizzlers::impl$0::x0y();
        *in_RCX = 1;
        *(ulonglong *)(in_RCX + 4) = CONCAT44(fVar9,fVar8);
        *(float *)(in_RCX + 0xc) = fVar7;
        *(undefined8 *)(in_RCX + 0x20) = in_stack_00000038;
        return;
      }
      goto LAB_140811730;
    }
  }
  *in_RCX = 9;
  return;
}


