// ADDR 0x140814470
// SIG void __cdecl is_half_dormer(void)
// ORIG system::is_half_dormer


void __cdecl system_decorator::derive_decorator_subtype::is_half_dormer(void)

{
  longlong lVar1;
  char *pcVar2;
  undefined8 *puVar3;
  char cVar4;
  longlong lVar5;
  ulonglong uVar6;
  uint uVar7;
  uint uVar8;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint extraout_EAX;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  char in_DL;
  char cVar9;
  ulonglong uVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong in_R9;
  longlong lVar14;
  ushort uVar15;
  byte bVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  char local_e6;
  char local_e5;
  undefined *local_d8;
  longlong lStack_d0;
  char **local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  char *local_a8;
  code *local_a0;
  
  if (1 < *(uint *)(in_R9 + 600)) {
    return;
  }
  if (in_stack_00000028 != 0) {
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (extraout_AL != '\0') {
      return;
    }
    country_core::resources::roofs::roof_shape::Roof::is_gable();
    if (extraout_AL_00 != '\0') {
      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
      country_core::resources::roofs::roof_shape::Roof::gable_sides();
      if (((ulonglong)local_d8 & 1) == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      ::utils::wall_space::WallSpaceCoord::to_world_space();
      ::utils::geometry::rectangle::Rectangle2d::get_closest_edge_id();
      if (lStack_d0 == extraout_RAX) {
        return;
      }
      if (local_c8 == (char **)extraout_RAX) {
        return;
      }
    }
  }
  ::utils::wall_space::WallSpaceCoord::to_world_space();
  ::utils::curve::Curve::
  get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>();
  if (*(ulonglong *)(in_R9 + 0xe8) <= (ulonglong)extraout_EAX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_R9 + 0xe8) <= (ulonglong)extraout_EAX + 1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (2 < (byte)(in_DL - 0x38U)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  cVar9 = (char)(0x160600 >> ((in_DL - 0x38U & 3) << 3));
  country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
  local_e5 = cVar9;
  if (in_stack_00000030[3] != 0) {
    local_e6 = extraout_AL_01;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>_>
              ();
    lVar5 = *in_stack_00000030;
    uVar6 = in_stack_00000030[1];
    uVar12 = uVar6 & extraout_RAX_00;
    bVar16 = (byte)(extraout_RAX_00 >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar16,bVar16));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar1 = lVar5 + -0x48;
    cVar9 = auVar17[0];
    cVar18 = auVar17[1];
    cVar19 = auVar17[2];
    cVar20 = auVar17[3];
    if (local_e6 == '\x1a') {
      lVar14 = 0;
      do {
        pcVar2 = (char *)(lVar5 + uVar12);
        auVar23[0] = -(*pcVar2 == cVar9);
        auVar23[1] = -(pcVar2[1] == cVar18);
        auVar23[2] = -(pcVar2[2] == cVar19);
        auVar23[3] = -(pcVar2[3] == cVar20);
        auVar23[4] = -(pcVar2[4] == cVar9);
        auVar23[5] = -(pcVar2[5] == cVar18);
        auVar23[6] = -(pcVar2[6] == cVar19);
        auVar23[7] = -(pcVar2[7] == cVar20);
        auVar23[8] = -(pcVar2[8] == cVar9);
        auVar23[9] = -(pcVar2[9] == cVar18);
        auVar23[10] = -(pcVar2[10] == cVar19);
        auVar23[0xb] = -(pcVar2[0xb] == cVar20);
        auVar23[0xc] = -(pcVar2[0xc] == cVar9);
        auVar23[0xd] = -(pcVar2[0xd] == cVar18);
        auVar23[0xe] = -(pcVar2[0xe] == cVar19);
        auVar23[0xf] = -(pcVar2[0xf] == cVar20);
        uVar15 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf
        ;
        uVar11 = (uint)uVar15;
        while (uVar15 != 0) {
          uVar7 = 0;
          for (uVar8 = uVar11; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          uVar10 = uVar7 + uVar12 & uVar6;
          if (*(char *)(lVar1 + uVar10 * -0x48) == '\x1a') {
LAB_14081470f:
            puVar3 = (undefined8 *)(lVar5 + -0x20 + uVar10 * -0x48);
            local_d8 = (undefined *)*puVar3;
            lStack_d0 = puVar3[1];
            puVar3 = (undefined8 *)(lVar5 + -0x10 + uVar10 * -0x48);
            local_c8 = (char **)*puVar3;
            uStack_c0 = puVar3[1];
            ::utils::swizzlers::impl$1::xvy();
            return;
          }
          uVar15 = (ushort)(uVar11 - 1) & (ushort)uVar11;
          uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar15);
        }
        auVar21[0] = -(*pcVar2 == -1);
        auVar21[1] = -(pcVar2[1] == -1);
        auVar21[2] = -(pcVar2[2] == -1);
        auVar21[3] = -(pcVar2[3] == -1);
        auVar21[4] = -(pcVar2[4] == -1);
        auVar21[5] = -(pcVar2[5] == -1);
        auVar21[6] = -(pcVar2[6] == -1);
        auVar21[7] = -(pcVar2[7] == -1);
        auVar21[8] = -(pcVar2[8] == -1);
        auVar21[9] = -(pcVar2[9] == -1);
        auVar21[10] = -(pcVar2[10] == -1);
        auVar21[0xb] = -(pcVar2[0xb] == -1);
        auVar21[0xc] = -(pcVar2[0xc] == -1);
        auVar21[0xd] = -(pcVar2[0xd] == -1);
        auVar21[0xe] = -(pcVar2[0xe] == -1);
        auVar21[0xf] = -(pcVar2[0xf] == -1);
        if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x1f,0) & 1) != 0) || (SUB161(auVar21 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') goto LAB_1408147be;
        lVar13 = uVar12 + lVar14;
        lVar14 = lVar14 + 0x10;
        uVar12 = lVar13 + 0x10U & uVar6;
      } while( true );
    }
    lVar14 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar5 + uVar12);
      auVar17[0] = -(*pcVar2 == cVar9);
      auVar17[1] = -(pcVar2[1] == cVar18);
      auVar17[2] = -(pcVar2[2] == cVar19);
      auVar17[3] = -(pcVar2[3] == cVar20);
      auVar17[4] = -(pcVar2[4] == cVar9);
      auVar17[5] = -(pcVar2[5] == cVar18);
      auVar17[6] = -(pcVar2[6] == cVar19);
      auVar17[7] = -(pcVar2[7] == cVar20);
      auVar17[8] = -(pcVar2[8] == cVar9);
      auVar17[9] = -(pcVar2[9] == cVar18);
      auVar17[10] = -(pcVar2[10] == cVar19);
      auVar17[0xb] = -(pcVar2[0xb] == cVar20);
      auVar17[0xc] = -(pcVar2[0xc] == cVar9);
      auVar17[0xd] = -(pcVar2[0xd] == cVar18);
      auVar17[0xe] = -(pcVar2[0xe] == cVar19);
      auVar17[0xf] = -(pcVar2[0xf] == cVar20);
      uVar15 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar7 = 0;
        for (uVar8 = uVar11; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        uVar10 = uVar7 + uVar12 & uVar6;
        cVar4 = *(char *)(lVar1 + uVar10 * -0x48);
        if (((cVar4 != '\x1a') && (local_e6 == cVar4)) &&
           (local_e5 == *(char *)(lVar1 + uVar10 * -0x48 + 1))) goto LAB_14081470f;
        uVar15 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar15);
      }
      auVar22[0] = -(*pcVar2 == -1);
      auVar22[1] = -(pcVar2[1] == -1);
      auVar22[2] = -(pcVar2[2] == -1);
      auVar22[3] = -(pcVar2[3] == -1);
      auVar22[4] = -(pcVar2[4] == -1);
      auVar22[5] = -(pcVar2[5] == -1);
      auVar22[6] = -(pcVar2[6] == -1);
      auVar22[7] = -(pcVar2[7] == -1);
      auVar22[8] = -(pcVar2[8] == -1);
      auVar22[9] = -(pcVar2[9] == -1);
      auVar22[10] = -(pcVar2[10] == -1);
      auVar22[0xb] = -(pcVar2[0xb] == -1);
      auVar22[0xc] = -(pcVar2[0xc] == -1);
      auVar22[0xd] = -(pcVar2[0xd] == -1);
      auVar22[0xe] = -(pcVar2[0xe] == -1);
      auVar22[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      lVar13 = uVar12 + lVar14;
      lVar14 = lVar14 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar6;
    }
  }
LAB_1408147be:
  local_a8 = &local_e6;
  local_a0 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
  local_d8 = &DAT_142a8f090;
  lStack_d0 = 1;
  local_b8 = 0;
  local_c8 = &local_a8;
  uStack_c0 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


