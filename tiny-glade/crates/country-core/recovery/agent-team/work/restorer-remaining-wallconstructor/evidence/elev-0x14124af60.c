
void __cdecl
system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize(void)

{
  longlong lVar1;
  char extraout_AL;
  char extraout_AL_00;
  ulonglong uVar2;
  longlong extraout_RAX;
  longlong in_RCX;
  ulonglong uVar3;
  longlong in_R8;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  float fVar8;
  
  fVar8 = *(float *)(in_RCX + 8);
  if (1.75 <= fVar8) {
    fVar8 = ceilf(fVar8 + fVar8);
    uVar3 = (longlong)(fVar8 - 9.223372e+18) & (longlong)fVar8 >> 0x3f | (longlong)fVar8;
    if (fVar8 < 0.0) {
      uVar3 = 0;
    }
    uVar5 = 0xffffffffffffffff;
    if (fVar8 <= 1.8446743e+19) {
      uVar5 = uVar3;
    }
    lVar1 = *(longlong *)(in_R8 + 0x10);
    uVar3 = 0;
    do {
      uVar2 = uVar3 + (uVar3 < uVar5);
      uVar6 = 0;
      do {
        uVar7 = uVar6 + (uVar6 < uVar5);
        ::utils::geometry::circle::Circle2d::is_point_inside();
        lVar4 = lVar1 * 0x50;
        if (extraout_AL != '\0') {
          do {
            if (lVar4 == 0) break;
            country_core::resources::walls::public_walls::PublicWalls::get();
            if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
            country_core::resources::glade_border::GladeBorder::is_pos_inside();
            lVar4 = lVar4 + -0x50;
          } while (extraout_AL_00 == '\0');
        }
      } while ((uVar6 < uVar5) && (uVar6 = uVar7, uVar7 <= uVar5));
    } while ((uVar3 < uVar5) && (uVar3 = uVar2, uVar2 <= uVar5));
  }
  return;
}

