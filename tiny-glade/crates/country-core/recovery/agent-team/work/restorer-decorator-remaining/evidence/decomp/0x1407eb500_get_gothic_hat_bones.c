// 0x1407eb500 get_gothic_hat_bones
// sig: void __cdecl get_gothic_hat_bones(void)
// callees: [{"name": "get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>", "address": "14126a040"}, {"name": "get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>", "address": "141269710"}, {"name": "inverse_lerp", "address": "140c904a0"}, {"name": "panic", "address": "1428d9430"}, {"name": "panic_bounds_check", "address": "1428d9518"}, {"name": "project_clamp", "address": "140c91eb0"}, {"name": "to_world_space", "address": "140c92090"}, {"name": "wall_space", "address": "140b2a740"}, {"name": "xvy", "address": "140c9cde0"}]

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char cVar3;
  longlong lVar4;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  ulonglong uVar5;
  float *in_RCX;
  longlong in_RDX;
  longlong in_R8;
  undefined1 auVar6 [16];
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar14;
  undefined1 auVar13 [16];
  undefined8 local_c4;
  float local_bc;
  
  if (*(int *)(in_RDX + 600) == 0) {
    ::utils::inverse_lerp();
    cVar3 = *(char *)(in_R8 + 0x44);
    if (((cVar3 != '\x01') && (cVar3 != '\x02')) && (cVar3 != '\x03')) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
  }
  else {
    cVar3 = *(char *)(in_R8 + 0x44);
    if (((cVar3 != '\x01') && (cVar3 != '\x02')) && (cVar3 != '\x03')) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
  }
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  ::utils::wall_space::impl$1::project_clamp();
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  ::utils::wall_space::impl$1::project_clamp();
  ::utils::wall_space::WallSpaceCoord::to_world_space();
  ::utils::wall_space::WallSpaceCoord::to_world_space();
  ::utils::curve::Curve::
  get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  ::utils::curve::Curve::
  get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
            ();
  uVar5 = (ulonglong)extraout_EAX;
  if (uVar5 + 1 < *(ulonglong *)(in_RDX + 0x10)) {
    lVar4 = *(longlong *)(in_RDX + 8);
    fVar10 = *(float *)(lVar4 + 8 + uVar5 * 8);
    fVar12 = *(float *)(lVar4 + 0xc + uVar5 * 8);
    fVar11 = *(float *)(lVar4 + uVar5 * 8);
    fVar14 = *(float *)(lVar4 + 4 + uVar5 * 8);
    ::utils::curve::Curve::
    get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
              ();
    uVar5 = (ulonglong)extraout_EAX_00;
    if (uVar5 + 1 < *(ulonglong *)(in_RDX + 0x10)) {
      uVar1 = *(undefined8 *)(*(longlong *)(in_RDX + 8) + uVar5 * 8);
      uVar2 = *(undefined8 *)(*(longlong *)(in_RDX + 8) + 8 + uVar5 * 8);
      ::utils::wall_space::WallSpaceCoord::to_world_space();
      ::utils::curve::Curve::
      get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                ();
      uVar5 = (ulonglong)extraout_EAX_01;
      if (uVar5 + 1 < *(ulonglong *)(in_RDX + 0x10)) {
        fVar7 = (float)uVar2 - (float)uVar1;
        fVar8 = (float)((ulonglong)uVar2 >> 0x20) - (float)((ulonglong)uVar1 >> 0x20);
        fVar9 = 1.0 / SQRT(fVar8 * fVar8 + fVar7 * fVar7);
        lVar4 = *(longlong *)(in_RDX + 8);
        fVar10 = fVar10 - fVar11;
        fVar11 = *(float *)(lVar4 + 8 + uVar5 * 8) - *(float *)(lVar4 + uVar5 * 8);
        fVar12 = fVar12 - fVar14;
        fVar14 = *(float *)(lVar4 + 0xc + uVar5 * 8) - *(float *)(lVar4 + 4 + uVar5 * 8);
        auVar6._0_4_ = fVar10 * fVar10;
        auVar6._4_4_ = fVar11 * fVar11;
        auVar6._8_8_ = 0;
        auVar13._0_4_ = fVar12 * fVar12 + auVar6._0_4_;
        auVar13._4_4_ = fVar14 * fVar14 + auVar6._4_4_;
        auVar13._8_8_ = 0;
        auVar6 = sqrtps(auVar6,auVar13);
        auVar6 = divps(___xmm_00000000000000003f8000003f800000,auVar6);
        ::utils::swizzlers::impl$1::xvy();
        ::utils::swizzlers::impl$1::xvy();
        in_RCX[9] = local_bc;
        *(undefined8 *)(in_RCX + 7) = local_c4;
        *in_RCX = -fVar12 * auVar6._0_4_;
        in_RCX[1] = fVar10 * auVar6._0_4_;
        in_RCX[2] = -fVar14 * auVar6._4_4_;
        in_RCX[3] = auVar6._4_4_ * fVar11;
        *(ulonglong *)(in_RCX + 0xd) = CONCAT44(fVar7 * fVar9,-fVar8 * fVar9);
        return;
      }
    }
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

