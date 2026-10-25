
// WARNING: Removing unreachable block (ram,0x000141250526)

void __cdecl
system_wall_constructor::construct_elevation_supports::rectangle_pillars::
construct_rectangle_pillars(void)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  longlong lVar4;
  char extraout_AL;
  char extraout_AL_00;
  int extraout_EAX;
  ulonglong uVar5;
  longlong extraout_RAX;
  longlong lVar6;
  longlong in_RDX;
  undefined8 *in_R8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar7;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  uint uVar8;
  uint uVar9;
  float in_XMM1_Da;
  float fVar10;
  float fVar11;
  float fVar12;
  uint uVar13;
  uint uVar14;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong *in_stack_00000060;
  float local_1f8;
  undefined4 uStack_1f4;
  uint local_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 local_1b8;
  undefined4 uStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  float fStack_1a8;
  int local_198;
  longlong local_178;
  undefined8 uStack_170;
  float local_144;
  float fStack_140;
  float fStack_13c;
  longlong lStack_128;
  longlong local_120;
  
  if (*(char *)(in_RDX + 0x298) == '\0') {
    lStack_128 = 8;
    local_120 = 0;
    uVar3 = *in_R8;
    uVar13 = (uint)in_R8[2];
    uVar14 = (uint)((ulonglong)in_R8[2] >> 0x20);
    uVar1 = 0;
LAB_14124fdd0:
    uVar5 = uVar1;
    if (uVar5 < 4) {
      fVar10 = in_XMM1_Da;
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      fVar12 = *(float *)(in_RDX + 0x280);
      in_XMM1_Da = extraout_XMM0_Da;
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      uVar1 = uVar5 + 1;
      lVar6 = in_stack_00000028;
      if (extraout_XMM0_Da_00 + 0.05 <= fVar12) {
        while (lVar6 != in_stack_00000030 * 0x50 + in_stack_00000028) {
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
          in_XMM1_Da = extraout_XMM0_Da;
          country_core::resources::glade_border::GladeBorder::is_pos_inside();
          lVar6 = lVar6 + 0x50;
          if (extraout_AL != '\0') goto LAB_14124fdd0;
        }
        ::utils::swizzlers::impl$1::xvy();
        if (in_stack_00000030 == 0) goto LAB_14125010c;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::geometry::rectangle::Rectangle2d::get_corner_normal();
        uStack_170 = 3;
        lVar6 = 0;
        fVar12 = 10000.0;
        while (lVar6 != 3) {
          lVar6 = lVar6 + 1;
          ::utils::swizzlers::impl$0::x0y();
          country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params();
          local_178 = lVar6;
          if ((local_198 != 2) && ((0x6d84U >> (local_1c8 & 0x1f) & 1) == 0)) {
            if ((local_1c8 & 0xff) != 0) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            fVar7 = SQRT((fStack_13c - fStack_1a8) * (fStack_13c - fStack_1a8) +
                         (local_144 - fStack_1b0) * (local_144 - fStack_1b0) +
                         (fStack_140 - fStack_1ac) * (fStack_140 - fStack_1ac));
            fVar11 = fVar7;
            if (fVar12 <= fVar7) {
              fVar11 = fVar12;
            }
            fVar12 = (float)(~-(uint)NAN(fVar12) & (uint)fVar11 | -(uint)NAN(fVar12) & (uint)fVar7);
            in_XMM1_Da = fVar12;
          }
        }
        if (2.3 <= fVar12) {
LAB_14125010c:
          uStack_1b4 = (undefined4)in_R8[2];
          fStack_1b0 = (float)((ulonglong)in_R8[2] >> 0x20);
          uStack_1c4 = (undefined4)*in_R8;
          uStack_1c0 = (undefined4)((ulonglong)*in_R8 >> 0x20);
          uStack_1bc = (undefined4)in_R8[1];
          local_1b8 = (undefined4)((ulonglong)in_R8[1] >> 0x20);
          local_1c8 = 1;
          find_stone_column_start_y();
          if (extraout_EAX == 1) {
            fVar12 = *(float *)(in_RDX + 0x280);
            fVar7 = ((fVar12 - extraout_XMM0_Da_01) + -5.0) / 5.0;
            fVar11 = 0.0;
            if (0.0 <= fVar7) {
              fVar11 = fVar7;
            }
            fVar7 = 1.0;
            if (fVar11 <= 1.0) {
              fVar7 = fVar11;
            }
            fVar11 = fVar7 * fVar7;
            fVar7 = (3.0 - (fVar7 + fVar7)) * fVar11;
            ::utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id();
            ::utils::geometry::rectangle::Rectangle2d::get_corner_normal();
            fVar7 = fVar7 * 1.5 + (1.0 - fVar7);
            if (local_120 == 0) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
            }
            uVar9 = (uint)((uVar5 & 1) == 0);
            uVar8 = (int)(uVar9 << 0x1f) >> 0x1f;
            uVar9 = (int)(uVar9 << 0x1f) >> 0x1f;
            lVar6 = local_120 * 0x40;
            *(ulonglong *)(lVar6 + 8) =
                 CONCAT44(fVar10 + fVar11 * fVar7 * -0.4,
                          extraout_XMM0_Da + extraout_XMM0_Da_02 * fVar7 * -0.4);
            *(ulonglong *)(lVar6 + 0x10) = uVar5;
            local_1f8 = (float)uVar3;
            uStack_1f4 = (undefined4)((ulonglong)uVar3 >> 0x20);
            *(float *)(lVar6 + 0x18) = local_1f8;
            *(undefined4 *)(lVar6 + 0x1c) = uStack_1f4;
            *(longlong *)(lVar6 + 0x20) = local_178;
            *(undefined8 *)(lVar6 + 0x28) = uStack_170;
            *(float *)(lVar6 + 0x30) = extraout_XMM0_Da_01;
            *(float *)(lVar6 + 0x34) = fVar12;
            *(float *)(lVar6 + 0x38) = fVar7;
            *(ulonglong *)(lVar6 + 0x3c) =
                 CONCAT44(~uVar9 & uVar13,~uVar8 & uVar14) | CONCAT44(uVar14 & uVar9,uVar13 & uVar8)
            ;
            local_120 = local_120 + 1;
            in_XMM1_Da = local_1f8;
          }
        }
      }
      goto LAB_14124fdd0;
    }
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    ::utils::rng::init_rng();
    for (; lStack_128 != local_120 * 0x40 + 8; lStack_128 = lStack_128 + 0x40) {
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::rectangle::Rectangle3d::new();
      lVar6 = in_stack_00000060[2];
      if (lVar6 == *in_stack_00000060) {
        alloc::raw_vec::RawVec::
        grow_one<enum2_<toolbar::mei::config::NodeLayoutConfigAttrib>,alloc::alloc::Global>();
      }
      lVar4 = in_stack_00000060[1];
      puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar6 * 0x20);
      *puVar2 = CONCAT44(uStack_1b4,local_1b8);
      puVar2[1] = CONCAT44(fStack_1ac,fStack_1b0);
      puVar2 = (undefined8 *)(lVar4 + lVar6 * 0x20);
      *puVar2 = CONCAT44(uStack_1c4,local_1c8);
      puVar2[1] = CONCAT44(uStack_1bc,uStack_1c0);
      in_stack_00000060[2] = lVar6 + 1;
    }
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    if (extraout_AL_00 == '\0') {
      util_pillar_construction::construct_brick_columns();
    }
    else {
      util_pillar_construction::construct_wooden_pillar_w_stone_base();
    }
  }
  return;
}

