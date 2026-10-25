// ADDR 0x140819c00
// SIG void __cdecl decorator_on_rectangle_edited(void)
// ORIG system::decorator_on_rectangle_edited


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited(void)

{
  char *pcVar1;
  float fVar2;
  undefined8 uVar3;
  int *piVar4;
  int *piVar5;
  undefined8 uVar6;
  uint uVar7;
  uint uVar8;
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  longlong extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined4 *extraout_RAX_03;
  longlong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong lVar11;
  undefined8 *in_RDX;
  longlong lVar12;
  int *piVar13;
  int *piVar14;
  longlong in_R8;
  ulonglong uVar15;
  longlong in_R9;
  uint uVar16;
  int *piVar17;
  int *piVar18;
  ushort uVar19;
  int *piVar20;
  int *piVar21;
  int *piVar22;
  byte bVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  undefined4 extraout_XMM0_Da;
  float fVar28;
  float fVar29;
  undefined4 uVar30;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  undefined8 local_1ac;
  undefined4 local_1a4;
  float fStack_170;
  float fStack_16c;
  int local_138;
  longlong local_130;
  
  lVar11 = in_RDX[1];
  uVar15 = *(ulonglong *)*in_RDX;
  piVar20 = (int *)0x0;
  piVar14 = (int *)(uVar15 - *(ulonglong *)(lVar11 + 0x18));
  if (uVar15 < *(ulonglong *)(lVar11 + 0x18)) {
    piVar14 = piVar20;
  }
  lVar12 = (longlong)piVar14 * 0x58 + *(longlong *)(lVar11 + 8);
  piVar4 = *(int **)(lVar11 + 0x10);
  piVar13 = (int *)((longlong)piVar4 - (longlong)piVar14);
  if (piVar4 < piVar14) {
    piVar13 = piVar20;
  }
  if (piVar4 < piVar14) {
    lVar12 = 8;
  }
  piVar22 = (int *)(lVar12 + -0x58 + (longlong)piVar13 * 0x58);
  if (piVar4 <= piVar14) {
    piVar22 = piVar20;
  }
  piVar5 = *(int **)(lVar11 + 0x30);
  piVar17 = (int *)(uVar15 - *(ulonglong *)(lVar11 + 0x38));
  if (uVar15 < *(ulonglong *)(lVar11 + 0x38)) {
    piVar17 = piVar20;
  }
  lVar12 = (longlong)piVar17 * 0x58 + *(longlong *)(lVar11 + 0x28);
  piVar21 = (int *)((longlong)piVar5 - (longlong)piVar17);
  if (piVar5 < piVar17) {
    piVar21 = piVar20;
  }
  if (piVar5 < piVar17) {
    lVar12 = 8;
  }
  piVar18 = (int *)(lVar12 + -0x58 + (longlong)piVar21 * 0x58);
  if (piVar5 <= piVar17) {
    piVar18 = piVar22;
  }
  piVar13 = (int *)((longlong)piVar21 + (longlong)piVar13);
  if (piVar14 < piVar4 || piVar17 < piVar5) {
    piVar13 = piVar20;
  }
  *(ulonglong *)*in_RDX = *(longlong *)(lVar11 + 0x40) - (longlong)piVar13;
  if (piVar14 < piVar4 || piVar17 < piVar5) {
    **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
    **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
    if (*piVar18 != 0) {
      if (*piVar18 == 1) {
        lVar11 = 0x20;
        lVar12 = 0x10;
      }
      else {
        lVar11 = 0x18;
        lVar12 = 8;
      }
      uVar6 = *(undefined8 *)((longlong)piVar18 + lVar12);
      uVar3 = *(undefined8 *)((longlong)piVar18 + lVar11 + 8);
      fVar36 = (float)uVar3;
      fVar29 = (float)((ulonglong)uVar3 >> 0x20);
      uVar3 = *(undefined8 *)((longlong)piVar18 + lVar11 + 0x10);
      country_core::resources::walls::decorator_archivist::DecoratorArchivist::
      archive_current_decorators_in_storages<ref_<str_>_>();
      country_core::resources::walls::decorator_archivist::DecoratorArchivist::
      decorator_cull_allow_wall();
      country_core::resources::walls::inner_walls::InnerWalls::get();
      if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params();
      if (extraout_RAX_00 == (byte *)0x0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      if ((*extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
      if (extraout_RAX_01 != 0) {
        ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
        fStack_170 = (float)uVar6;
        fStack_16c = (float)((ulonglong)uVar6 >> 0x20);
        ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
        country_core::resources::walls::decorator_storage::impl$3::next();
        if (local_130 != 0) {
          auVar31._0_4_ = fStack_16c * 0.0 - fStack_170 * 0.0;
          auVar31._4_4_ = auVar31._0_4_;
          auVar31._8_4_ = auVar31._0_4_;
          auVar31._12_4_ = auVar31._0_4_;
          auVar31 = divps(___xmm_3f800000bf800000bf8000003f800000,auVar31);
          fStack_16c = fStack_16c * auVar31._0_4_;
          fVar37 = auVar31._4_4_ * 0.0;
          fStack_170 = fStack_170 * auVar31._8_4_;
          fVar38 = auVar31._12_4_ * 0.0;
          fVar28 = fVar29 * fStack_170;
          fVar29 = fVar29 * fVar38;
          do {
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            lVar11 = (ulonglong)*(byte *)(extraout_RAX_02 + 0x22) - 1;
            if (2 < (byte)(*(byte *)(extraout_RAX_02 + 0x22) - 2)) {
              lVar11 = 0;
            }
            if (lVar11 == 0) {
              country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
              country_core::resources::walls::decorator::DecoratorDst::as_wall_mut();
              country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators()
              ;
              if (extraout_RAX_04 == 0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              if (*(longlong *)(extraout_RAX_04 + 0x48) != 0) {
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                          ();
                bVar23 = (byte)(extraout_RAX_05 >> 0x39);
                auVar31 = ZEXT216(CONCAT11(bVar23,bVar23));
                auVar31 = pshuflw(auVar31,auVar31,0);
                lVar11 = 0;
                uVar15 = extraout_RAX_05;
                while( true ) {
                  uVar15 = *(ulonglong *)(extraout_RAX_04 + 0x38) & uVar15;
                  pcVar1 = (char *)(*(longlong *)(extraout_RAX_04 + 0x30) + uVar15);
                  cVar24 = auVar31[0];
                  auVar35[0] = -(*pcVar1 == cVar24);
                  cVar25 = auVar31[1];
                  auVar35[1] = -(pcVar1[1] == cVar25);
                  cVar26 = auVar31[2];
                  auVar35[2] = -(pcVar1[2] == cVar26);
                  cVar27 = auVar31[3];
                  auVar35[3] = -(pcVar1[3] == cVar27);
                  auVar35[4] = -(pcVar1[4] == cVar24);
                  auVar35[5] = -(pcVar1[5] == cVar25);
                  auVar35[6] = -(pcVar1[6] == cVar26);
                  auVar35[7] = -(pcVar1[7] == cVar27);
                  auVar35[8] = -(pcVar1[8] == cVar24);
                  auVar35[9] = -(pcVar1[9] == cVar25);
                  auVar35[10] = -(pcVar1[10] == cVar26);
                  auVar35[0xb] = -(pcVar1[0xb] == cVar27);
                  auVar35[0xc] = -(pcVar1[0xc] == cVar24);
                  auVar35[0xd] = -(pcVar1[0xd] == cVar25);
                  auVar35[0xe] = -(pcVar1[0xe] == cVar26);
                  auVar35[0xf] = -(pcVar1[0xf] == cVar27);
                  uVar19 = (ushort)(SUB161(auVar35 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar35[0xf] >> 7) << 0xf;
                  uVar16 = (uint)uVar19;
                  while (uVar19 != 0) {
                    uVar7 = 0;
                    for (uVar8 = uVar16; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
                      uVar7 = uVar7 + 1;
                    }
                    if (local_138 ==
                        *(int *)(*(longlong *)(extraout_RAX_04 + 0x30) + -0x50 +
                                (uVar7 + uVar15 & *(ulonglong *)(extraout_RAX_04 + 0x38)) * -0x50))
                    {
                      country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
                      fVar2 = *(float *)(extraout_RAX_06 + 0x14);
                      fVar40 = *(float *)(extraout_RAX_06 + 0x1c);
                      country_core::resources::walls::wall_attachment::WallCornerAttachment::
                      to_rectangle_corner_id();
                      if (extraout_RAX_07 == 1) {
                        ::utils::geometry::rectangle::Rectangle2d::get_point();
                        ::utils::swizzlers::impl$1::xvy();
                        extraout_RAX_03[7] = local_1a4;
                        *(undefined8 *)(extraout_RAX_03 + 5) = local_1ac;
                        ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
                        ::utils::geometry::rectangle::Rectangle2d::perimeter();
                        uVar30 = extraout_RAX_03[6];
                      }
                      else {
                        fVar39 = fVar40 * fStack_170;
                        fVar40 = fVar40 * fVar38;
                        auVar33._4_4_ = (fVar40 + fVar2 * fVar37) - (fVar29 + fVar36 * fVar37);
                        auVar33._0_4_ =
                             (fVar39 + fVar2 * fStack_16c) - (fVar28 + fVar36 * fStack_16c);
                        auVar33._8_4_ = (fVar39 + fVar39) - (fVar28 + fVar28);
                        auVar33._12_4_ = (fVar40 + fVar40) - (fVar29 + fVar29);
                        auVar10._4_4_ = (float)((ulonglong)uVar3 >> 0x20) * 0.5;
                        auVar10._0_4_ = (float)uVar3 * 0.5;
                        auVar10._8_8_ = 0;
                        auVar31 = divps(auVar33,auVar10);
                        auVar35 = maxps(___xmm_0000000000000000bf800000bf800000,auVar31);
                        auVar34._0_4_ = ~-(uint)NAN(auVar31._0_4_) & auVar35._0_4_;
                        auVar34._4_4_ = ~-(uint)NAN(auVar31._4_4_) & auVar35._4_4_;
                        auVar34._8_4_ = ~-(uint)NAN(auVar31._8_4_) & auVar35._8_4_;
                        auVar34._12_4_ = ~-(uint)NAN(auVar31._12_4_) & auVar35._12_4_;
                        auVar9._4_4_ = __xmm_0000000000000000bf800000bf800000._4_4_ &
                                       -(uint)NAN(auVar31._4_4_);
                        auVar9._0_4_ = (uint)__xmm_0000000000000000bf800000bf800000 &
                                       -(uint)NAN(auVar31._0_4_);
                        auVar9._8_4_ = __xmm_0000000000000000bf800000bf800000._8_4_ &
                                       -(uint)NAN(auVar31._8_4_);
                        auVar9._12_4_ =
                             __xmm_0000000000000000bf800000bf800000._12_4_ &
                             -(uint)NAN(auVar31._12_4_);
                        minps(___xmm_00000000000000003f8000003f800000,auVar34 | auVar9);
                        ::utils::swizzlers::impl$1::xvy();
                        extraout_RAX_03[7] = local_1a4;
                        *(undefined8 *)(extraout_RAX_03 + 5) = local_1ac;
                        ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
                        ::utils::geometry::rectangle::Rectangle2d::perimeter();
                        uVar30 = extraout_RAX_03[1];
                      }
                      bevy_app::plugin::Plugin::
                      finish<country_core::systems::main_camera::MainCameraPlugin>();
                      *extraout_RAX_03 = extraout_XMM0_Da;
                      extraout_RAX_03[1] = uVar30;
                      goto LAB_140819f9a;
                    }
                    uVar19 = (ushort)(uVar16 - 1) & (ushort)uVar16;
                    uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar19);
                  }
                  auVar32[0] = -(*pcVar1 == -1);
                  auVar32[1] = -(pcVar1[1] == -1);
                  auVar32[2] = -(pcVar1[2] == -1);
                  auVar32[3] = -(pcVar1[3] == -1);
                  auVar32[4] = -(pcVar1[4] == -1);
                  auVar32[5] = -(pcVar1[5] == -1);
                  auVar32[6] = -(pcVar1[6] == -1);
                  auVar32[7] = -(pcVar1[7] == -1);
                  auVar32[8] = -(pcVar1[8] == -1);
                  auVar32[9] = -(pcVar1[9] == -1);
                  auVar32[10] = -(pcVar1[10] == -1);
                  auVar32[0xb] = -(pcVar1[0xb] == -1);
                  auVar32[0xc] = -(pcVar1[0xc] == -1);
                  auVar32[0xd] = -(pcVar1[0xd] == -1);
                  auVar32[0xe] = -(pcVar1[0xe] == -1);
                  auVar32[0xf] = -(pcVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar32 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar32 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar32 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar32 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar32 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar32 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar32 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar32 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar32 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar32 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar32 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x6f,0) & 1) != 0) || (SUB161(auVar32 >> 0x77,0) & 1) != 0
                      ) || auVar32[0xf] < '\0') break;
                  uVar15 = uVar15 + lVar11 + 0x10;
                  lVar11 = lVar11 + 0x10;
                }
              }
                    // WARNING: Subroutine does not return
              core::option::expect_failed();
            }
            if (lVar11 != 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
LAB_140819f9a:
            country_core::resources::walls::decorator_storage::impl$3::next();
          } while (local_130 != 0);
        }
      }
    }
  }
  return;
}


