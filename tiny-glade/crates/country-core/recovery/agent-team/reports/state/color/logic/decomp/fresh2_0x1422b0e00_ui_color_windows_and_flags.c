// name=ui_color_windows_and_flags
// sig=void __cdecl ui_color_windows_and_flags(void)
// size=2168
// address=0x1422b0e00

void __cdecl system_color::ui_color_decorator::ui_color_windows_and_flags(void)

{
  uint *puVar1;
  float *pfVar2;
  ulonglong *puVar3;
  uint uVar4;
  uint uVar5;
  undefined1 extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  char *pcVar6;
  undefined8 *in_RCX;
  undefined8 *puVar7;
  undefined1 *puVar8;
  char cVar9;
  uint *puVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  uint uVar15;
  ushort uVar16;
  ulonglong uVar17;
  longlong lVar18;
  bool bVar19;
  char cVar20;
  bool bVar21;
  byte bVar22;
  char cVar24;
  char cVar25;
  undefined1 auVar23 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000060;
  longlong *in_stack_00000070;
  undefined8 local_310;
  code *local_308;
  ulonglong local_260;
  undefined1 *local_258;
  byte local_200;
  uint local_1fc;
  ulonglong local_1f8;
  ulonglong local_1f0;
  float local_1e8;
  float local_1e4;
  float local_1e0;
  int local_1d0;
  undefined1 local_1c0 [24];
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 *local_198;
  undefined8 local_190;
  undefined8 local_188;
  byte local_ff;
  ulonglong local_f8;
  ulonglong local_f0;
  undefined8 *local_e0;
  longlong local_d0;
  float *local_c8;
  longlong local_c0;
  longlong local_b8;
  ulonglong local_b0;
  ulonglong local_a0;
  byte local_93;
  undefined1 local_92;
  undefined1 local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  if (*(int *)*in_RCX == 0xc) {
    pfVar2 = (float *)*in_stack_00000060;
    country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params();
    if (local_1d0 != 2) {
      local_a0 = local_1f8;
      local_c8 = pfVar2;
      if (local_200 - 0xd < 2) {
        country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info()
        ;
        if (extraout_RAX == 0) {
          return;
        }
        puVar7 = (undefined8 *)(local_a0 & 0xffffffff);
        uVar11 = (ulonglong)local_1fc;
        country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage();
        lVar13 = *in_stack_00000070;
        local_b0 = uVar11;
        local_a0 = extraout_RAX_00;
        country_core::resources::stairs::stairs_assembly_params::impl$2::from();
        local_1a8 = (undefined **)extraout_RAX_01;
        lVar18 = extraout_RAX;
        if (*(longlong *)(lVar13 + 0x1b0) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::resources::ui_state::WidgetId>_>
                    ();
          lVar14 = *(longlong *)(lVar13 + 0x198);
          puVar7 = *(undefined8 **)(lVar13 + 0x1a0);
          bVar22 = (byte)(extraout_RAX_02 >> 0x39);
          auVar23 = ZEXT216(CONCAT11(bVar22,bVar22));
          auVar23 = pshuflw(auVar23,auVar23,0);
          lVar13 = 0;
          uVar11 = extraout_RAX_02;
          while( true ) {
            uVar11 = (ulonglong)puVar7 & uVar11;
            pcVar6 = (char *)(lVar14 + uVar11);
            cVar20 = auVar23[0];
            auVar29[0] = -(*pcVar6 == cVar20);
            cVar9 = auVar23[1];
            auVar29[1] = -(pcVar6[1] == cVar9);
            cVar24 = auVar23[2];
            auVar29[2] = -(pcVar6[2] == cVar24);
            cVar25 = auVar23[3];
            auVar29[3] = -(pcVar6[3] == cVar25);
            auVar29[4] = -(pcVar6[4] == cVar20);
            auVar29[5] = -(pcVar6[5] == cVar9);
            auVar29[6] = -(pcVar6[6] == cVar24);
            auVar29[7] = -(pcVar6[7] == cVar25);
            auVar29[8] = -(pcVar6[8] == cVar20);
            auVar29[9] = -(pcVar6[9] == cVar9);
            auVar29[10] = -(pcVar6[10] == cVar24);
            auVar29[0xb] = -(pcVar6[0xb] == cVar25);
            auVar29[0xc] = -(pcVar6[0xc] == cVar20);
            auVar29[0xd] = -(pcVar6[0xd] == cVar9);
            auVar29[0xe] = -(pcVar6[0xe] == cVar24);
            auVar29[0xf] = -(pcVar6[0xf] == cVar25);
            uVar16 = (ushort)(SUB161(auVar29 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar29[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar16;
            while (uVar16 != 0) {
              uVar4 = 0;
              for (uVar5 = uVar15; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                uVar4 = uVar4 + 1;
              }
              uVar17 = uVar4 + uVar11 & (ulonglong)puVar7;
              if (extraout_RAX_01 == *(longlong *)(lVar14 + -0x20 + uVar17 * -0x20)) {
                lVar13 = uVar17 * -0x20;
                uVar11 = *(ulonglong *)(lVar14 + -8 + lVar13);
                if (uVar11 < 2) {
                  puVar7 = (undefined8 *)((lVar14 + lVar13) - 0x18);
                }
                else {
                  puVar7 = *(undefined8 **)(lVar14 + -0x18 + lVar13);
                  uVar11 = *(ulonglong *)(lVar14 + -0x10 + lVar13);
                }
                bVar21 = false;
                if ((uVar11 != 0) &&
                   (pcVar6 = (char *)((longlong)puVar7 + uVar11 * 4 + -4), bVar21 = false,
                   pcVar6 != (char *)0x0)) {
                  bVar21 = *pcVar6 == '\x03';
                }
                bVar19 = true;
                if (local_a0 == 3) {
                  return;
                }
                goto LAB_1422b12c6;
              }
              uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
              uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
            }
            auVar27[0] = -(*pcVar6 == -1);
            auVar27[1] = -(pcVar6[1] == -1);
            auVar27[2] = -(pcVar6[2] == -1);
            auVar27[3] = -(pcVar6[3] == -1);
            auVar27[4] = -(pcVar6[4] == -1);
            auVar27[5] = -(pcVar6[5] == -1);
            auVar27[6] = -(pcVar6[6] == -1);
            auVar27[7] = -(pcVar6[7] == -1);
            auVar27[8] = -(pcVar6[8] == -1);
            auVar27[9] = -(pcVar6[9] == -1);
            auVar27[10] = -(pcVar6[10] == -1);
            auVar27[0xb] = -(pcVar6[0xb] == -1);
            auVar27[0xc] = -(pcVar6[0xc] == -1);
            auVar27[0xd] = -(pcVar6[0xd] == -1);
            auVar27[0xe] = -(pcVar6[0xe] == -1);
            auVar27[0xf] = -(pcVar6[0xf] == -1);
            if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar27 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar27 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar27 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar27 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar27 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar27 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar27 >> 0x3f,0) & 1) != 0) || (SUB161(auVar27 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar27 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar27 >> 0x57,0) & 1) != 0) || (SUB161(auVar27 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar27 >> 0x67,0) & 1) != 0) || (SUB161(auVar27 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar27 >> 0x77,0) & 1) != 0) || auVar27[0xf] < '\0') break;
            uVar11 = uVar11 + lVar13 + 0x10;
            lVar13 = lVar13 + 0x10;
          }
        }
        bVar19 = true;
        bVar21 = false;
      }
      else {
        if (local_200 != 4) {
          return;
        }
        local_b0 = local_1f0;
        country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info();
        if (extraout_RAX_03 == 0) {
          return;
        }
        local_93 = *(char *)(extraout_RAX_03 + 0x46) - 0x38;
        country_core::resources::stairs::stairs_state::StairsState::graph();
        local_1a8 = (undefined **)CONCAT44(local_1a8._4_4_,local_1fc);
        puVar7 = &local_1a8;
        indexmap::map::IndexMap::
        get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                  ();
        local_c0 = extraout_RAX_03;
        if ((extraout_RAX_04 == 0) || (*(longlong *)(extraout_RAX_04 + 0x10) == 0)) {
          bVar21 = false;
        }
        else {
          puVar10 = *(uint **)(extraout_RAX_04 + 8);
          puVar1 = puVar10 + *(longlong *)(extraout_RAX_04 + 0x10) * 2;
          lVar18 = *in_stack_00000070;
          local_d0 = lVar18 + 0x1b8;
          bVar21 = false;
          local_b8 = lVar18;
          do {
            puVar7 = (undefined8 *)(ulonglong)*puVar10;
            country_core::resources::stairs::stairs_state::StairSegment::new();
            country_core::resources::stairs::stairs_assembly_params::impl$2::from();
            local_1a8 = (undefined **)extraout_RAX_05;
            if (*(longlong *)(lVar18 + 0x1b0) != 0) {
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::resources::ui_state::WidgetId>_>
                        ();
              lVar13 = *(longlong *)(lVar18 + 0x198);
              puVar7 = *(undefined8 **)(lVar18 + 0x1a0);
              bVar22 = (byte)(extraout_RAX_06 >> 0x39);
              auVar23 = ZEXT216(CONCAT11(bVar22,bVar22));
              auVar23 = pshuflw(auVar23,auVar23,0);
              lVar14 = 0;
              uVar11 = extraout_RAX_06;
              while( true ) {
                uVar11 = (ulonglong)puVar7 & uVar11;
                auVar29 = *(undefined1 (*) [16])(lVar13 + uVar11);
                cVar20 = auVar23[0];
                auVar28[0] = -(auVar29[0] == cVar20);
                cVar9 = auVar23[1];
                auVar28[1] = -(auVar29[1] == cVar9);
                cVar24 = auVar23[2];
                auVar28[2] = -(auVar29[2] == cVar24);
                cVar25 = auVar23[3];
                auVar28[3] = -(auVar29[3] == cVar25);
                auVar28[4] = -(auVar29[4] == cVar20);
                auVar28[5] = -(auVar29[5] == cVar9);
                auVar28[6] = -(auVar29[6] == cVar24);
                auVar28[7] = -(auVar29[7] == cVar25);
                auVar28[8] = -(auVar29[8] == cVar20);
                auVar28[9] = -(auVar29[9] == cVar9);
                auVar28[10] = -(auVar29[10] == cVar24);
                auVar28[0xb] = -(auVar29[0xb] == cVar25);
                auVar28[0xc] = -(auVar29[0xc] == cVar20);
                auVar28[0xd] = -(auVar29[0xd] == cVar9);
                auVar28[0xe] = -(auVar29[0xe] == cVar24);
                auVar28[0xf] = -(auVar29[0xf] == cVar25);
                uVar16 = (ushort)(SUB161(auVar28 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar28[0xf] >> 7) << 0xf;
                uVar15 = (uint)uVar16;
                while (uVar16 != 0) {
                  uVar4 = 0;
                  for (uVar5 = uVar15; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                    uVar4 = uVar4 + 1;
                  }
                  uVar17 = uVar4 + uVar11 & (ulonglong)puVar7;
                  lVar18 = local_b8;
                  if (extraout_RAX_05 == *(longlong *)(lVar13 + -0x20 + uVar17 * -0x20)) {
                    lVar14 = uVar17 * -0x20;
                    puVar7 = *(undefined8 **)(lVar13 + -8 + lVar14);
                    if (puVar7 < (undefined8 *)0x2) {
                      lVar12 = lVar13 + lVar14 + -0x18;
                    }
                    else {
                      lVar12 = *(longlong *)(lVar13 + -0x18 + lVar14);
                      puVar7 = *(undefined8 **)(lVar13 + -0x10 + lVar14);
                    }
                    bVar19 = false;
                    if (puVar7 != (undefined8 *)0x0) {
                      bVar19 = false;
                      pcVar6 = (char *)(lVar12 + (longlong)puVar7 * 4 + -4);
                      if (pcVar6 != (char *)0x0) {
                        bVar19 = *pcVar6 == '\x03';
                      }
                    }
                    bVar21 = (bool)(bVar19 | bVar21);
                    goto LAB_1422b1230;
                  }
                  uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                  uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
                }
                auVar26[0] = -(auVar29[0] == -1);
                auVar26[1] = -(auVar29[1] == -1);
                auVar26[2] = -(auVar29[2] == -1);
                auVar26[3] = -(auVar29[3] == -1);
                auVar26[4] = -(auVar29[4] == -1);
                auVar26[5] = -(auVar29[5] == -1);
                auVar26[6] = -(auVar29[6] == -1);
                auVar26[7] = -(auVar29[7] == -1);
                auVar26[8] = -(auVar29[8] == -1);
                auVar26[9] = -(auVar29[9] == -1);
                auVar26[10] = -(auVar29[10] == -1);
                auVar26[0xb] = -(auVar29[0xb] == -1);
                auVar26[0xc] = -(auVar29[0xc] == -1);
                auVar26[0xd] = -(auVar29[0xd] == -1);
                auVar26[0xe] = -(auVar29[0xe] == -1);
                auVar26[0xf] = -(auVar29[0xf] == -1);
                if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar26 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar26 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') break;
                uVar11 = uVar11 + lVar14 + 0x10;
                lVar14 = lVar14 + 0x10;
              }
            }
LAB_1422b1230:
            puVar10 = puVar10 + 2;
          } while (puVar10 != puVar1);
        }
        bVar19 = local_93 < 6;
        lVar18 = local_c0;
      }
      if (local_a0 != 3) {
LAB_1422b12c6:
        local_f8 = local_a0;
        local_f0 = local_b0;
        country_core::resources::walls::decorator_subtype::impl$8::from();
        local_e0 = puVar7;
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        local_a0 = CONCAT71(local_a0._1_7_,extraout_AL);
        puVar3 = (ulonglong *)*in_stack_00000030;
        uVar11 = *puVar3;
        local_310 = &local_260;
        local_308 = core::fmt::num::impl$89::fmt;
        local_1a8 = &PTR_s_color_decorator__14303cde8;
        local_1a0 = 1;
        local_198 = &local_310;
        local_190 = 1;
        local_188 = 0;
        local_260 = (ulonglong)local_1fc;
        alloc::fmt::format::format_inner();
        local_310 = (ulonglong *)
                    CONCAT44(SQRT((local_c8[2] - local_1e0) * (local_c8[2] - local_1e0) +
                                  (local_c8[1] - local_1e4) * (local_c8[1] - local_1e4) +
                                  (*local_c8 - local_1e8) * (*local_c8 - local_1e8)),
                             (undefined4)local_310);
        local_310 = (ulonglong *)CONCAT71(local_310._1_7_,5);
        local_260 = local_260 & 0xffffffffffffff00;
        local_91 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::new();
        local_ff = local_ff | 2;
        memcpy(&local_310,&local_1a8,0xb0);
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::UiSounds::empty();
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
        local_1a8 = (undefined **)CONCAT44(local_1a8._4_4_,(uint)bVar19 * 4 + 4);
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::UiSounds::on_click();
        local_92 = 0;
        country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
        local_258 = (undefined1 *)CONCAT71(local_258._1_7_,bVar19);
        local_91 = 1;
        local_260 = uVar11;
        country_core::systems::ui::resources::widget_params::WidgetParams::
        signifier<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env_3>();
        local_91 = 0;
        puVar8 = local_1c0;
        country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        if (bVar19 != false) {
          local_258 = puVar8;
          country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
          if (extraout_AL_00 != '\0') {
            if (*(char *)(lVar18 + 0x46) == '?') {
              uVar15 = local_1fc;
              country_core::resources::stairs::stairs_state::StairSegment::new();
              local_1a8 = (undefined **)CONCAT44(uVar15,extraout_EAX);
              local_1a0 = CONCAT26(local_1a0._6_2_,0x10040000000);
              bevy_ecs::event::writer::EventWriter::
              write<country_core::systems::wall::structure_outline::OutlineStairsCmd>();
            }
            else {
              country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms
              ::transform();
              ::utils::components::transform::Transform::compute_affine();
              country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline()
              ;
              if (*(byte *)(lVar18 + 0x22) < 2) {
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
                country_core::resources::walls::public_walls::PublicWalls::get();
              }
              country_core::resources::walls::decorator_outline::DecoratorOutline::display();
            }
          }
          country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
          if (extraout_AL_01 != '\0') {
            bevy_ecs::event::writer::EventWriter::
            write<system_color::show_color_menu::HideColorMenuCmd>();
            cVar20 = *(char *)(lVar18 + 0x46);
            switch(cVar20) {
            case '8':
            case '9':
            case ':':
              cVar20 = (byte)(cVar20 - 0x39U) < 2;
              cVar9 = '\x03';
              break;
            case ';':
              cVar9 = '\n';
              break;
            case '<':
              cVar9 = '\t';
              break;
            case '=':
              cVar9 = '\x06';
              break;
            default:
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            case '?':
              cVar9 = bVar21 + 0xb;
            }
            local_1a8._0_2_ = CONCAT11(cVar20,cVar9);
            local_1a8 = (undefined **)CONCAT44(local_1fc,(undefined4)local_1a8);
            local_198 = (undefined8 *)*puVar3;
            bevy_ecs::event::writer::EventWriter::
            write<system_color::show_color_menu::ShowColorMenuCmd>();
          }
        }
      }
    }
  }
  return;
}


