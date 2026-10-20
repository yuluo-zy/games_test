// name: do_ui_arch_controls
// addr: 0x140767b70
// size: 1406
// signature: void __cdecl do_ui_arch_controls(void)


void __cdecl system_wall_constructor::ui_arch_controls::do_ui_arch_controls(void)

{
  byte *pbVar1;
  float *pfVar2;
  undefined1 auVar3 [16];
  byte bVar4;
  byte bVar5;
  int iVar6;
  undefined4 uVar7;
  ulonglong uVar8;
  undefined4 *puVar9;
  void *_Src;
  undefined1 *puVar10;
  undefined1 *puVar11;
  undefined1 auVar12 [16];
  uint uVar13;
  uint uVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  undefined1 auVar22 [16];
  char extraout_AL;
  char extraout_AL_00;
  uint uVar23;
  int extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar24;
  longlong lVar25;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong extraout_RAX_13;
  undefined8 *extraout_RAX_14;
  float *pfVar26;
  undefined8 *extraout_RAX_15;
  undefined1 *puVar27;
  longlong *extraout_RAX_16;
  longlong *extraout_RAX_17;
  undefined8 *in_RCX;
  ulonglong uVar28;
  ulonglong uVar29;
  float *pfVar30;
  longlong in_RDX;
  ulonglong uVar31;
  longlong *plVar32;
  longlong **pplVar33;
  ulonglong uVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  ulonglong uVar37;
  undefined8 *puVar38;
  undefined *puVar39;
  longlong lVar40;
  undefined8 *in_R9;
  longlong *plVar41;
  ushort uVar42;
  ulonglong uVar43;
  longlong lVar44;
  char cVar45;
  bool bVar46;
  bool bVar47;
  byte bVar48;
  char cVar49;
  char cVar54;
  float fVar50;
  char cVar52;
  char cVar53;
  undefined1 auVar51 [16];
  float fVar55;
  float fVar56;
  float fVar59;
  float fVar60;
  undefined1 auVar57 [16];
  float fVar61;
  undefined1 auVar58 [16];
  undefined1 auVar62 [16];
  float fVar63;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000058;
  longlong in_stack_00000068;
  longlong in_stack_00000070;
  undefined8 *in_stack_00000080;
  longlong lStack_270;
  longlong *plStack_268;
  longlong *plStack_248;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  byte bStack_238;
  byte bStack_237;
  undefined2 uStack_236;
  float fStack_234;
  code *pcStack_230;
  undefined4 *local_228;
  code *pcStack_220;
  undefined1 *puStack_218;
  code *pcStack_210;
  undefined8 uStack_138;
  longlong local_128;
  undefined8 uStack_120;
  undefined4 uStack_104;
  undefined8 *puStack_100;
  undefined **ppuStack_f8;
  undefined8 uStack_f0;
  longlong **pplStack_e8;
  code *pcStack_e0;
  undefined1 auStack_d8 [16];
  undefined1 *puStack_c8;
  ulonglong uStack_c0;
  undefined8 *puStack_b8;
  ulonglong uStack_b0;
  ulonglong local_a8;
  longlong *local_a0;
  longlong *local_98;
  longlong local_90;
  undefined1 uStack_85;
  undefined1 uStack_84;
  undefined1 uStack_83;
  char local_82;
  undefined1 uStack_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  local_a8 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    iVar6 = *(int *)*in_RCX;
  }
  else {
    uVar31 = 0x3e;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar31 != 0) {
        if (uVar31 < 0x3e) {
          if ((char)(&DAT_142a7def6)[uVar31] < -0x40) {
LAB_140767c0b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 != 0x3e) goto LAB_140767c0b;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar31 != 0xfffffffffffffffe)) {
        if (uVar31 + 2 < 0x3e) {
          if ((char)(&UNK_142a7def8)[uVar31] < -0x40) {
LAB_140767c79:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 + 2 != 0x3e) goto LAB_140767c79;
      }
    }
    puVar39 = &DAT_142a7e07d;
    do {
      bVar48 = puVar39[-1];
      uVar23 = (uint)(char)bVar48;
      if ((char)bVar48 < '\0') {
        bVar4 = puVar39[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar39[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar39 + -4;
            puVar39 = puVar39 + -4;
            uVar23 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar39 = puVar39 + -3;
            uVar23 = bVar5 & 0xf;
          }
          uVar23 = bVar4 & 0x3f | uVar23 << 6;
        }
        else {
          puVar39 = puVar39 + -2;
          uVar23 = bVar4 & 0x1f;
        }
        uVar23 = (uint)(bVar48 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_140767d34;
LAB_140767d45:
        if (puVar39 == &UNK_142a7e045) break;
        if (puVar39 + -0x142a7e045 < (undefined *)0x37) {
          if (-0x41 < (char)puVar39[1]) break;
        }
        else if (puVar39 + -0x142a7e045 == (undefined *)0x37) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar39 = puVar39 + -1;
      if (uVar23 == 0x5c) goto LAB_140767d45;
LAB_140767d34:
      if (uVar23 == 0x2f) goto LAB_140767d45;
    } while (puVar39 != &DAT_142a7e046);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar32 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar32 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_140768b91;
    if (*plVar32 != 0) goto LAB_140768b9d;
    *plVar32 = -1;
    local_a0 = plVar32;
    puffin::ThreadProfiler::begin_scope();
    *local_a0 = *local_a0 + 1;
    iVar6 = *(int *)*in_RCX;
  }
  cVar45 = extraout_AL;
  if (iVar6 == 0) {
    lVar44 = in_stack_00000080[1];
    uVar31 = *(ulonglong *)*in_stack_00000080;
    uVar34 = 0;
    uVar43 = uVar31 - *(ulonglong *)(lVar44 + 0x18);
    if (uVar31 < *(ulonglong *)(lVar44 + 0x18)) {
      uVar43 = uVar34;
    }
    uVar24 = uVar31 - *(ulonglong *)(lVar44 + 0x38);
    if (uVar31 < *(ulonglong *)(lVar44 + 0x38)) {
      uVar24 = uVar34;
    }
    uVar31 = *(ulonglong *)(lVar44 + 0x10);
    uVar8 = *(ulonglong *)(lVar44 + 0x30);
    bVar46 = uVar8 < uVar24;
    uVar28 = uVar8 - uVar24;
    uVar29 = uVar28;
    if (bVar46) {
      uVar29 = uVar34;
    }
    bVar47 = uVar31 < uVar43;
    uVar35 = uVar31 - uVar43;
    uVar36 = uVar35;
    if (bVar47) {
      uVar36 = uVar34;
    }
    local_a0 = *(longlong **)(lVar44 + 8);
    uVar37 = uVar29 + uVar36;
    if (!bVar47 && uVar35 != 0 || !bVar46 && uVar28 != 0) {
      uVar37 = uVar34;
    }
    lVar25 = *(longlong *)(lVar44 + 0x28);
    *(ulonglong *)*in_stack_00000080 = *(longlong *)(lVar44 + 0x40) - uVar37;
    if (!bVar47 && uVar35 != 0 || !bVar46 && uVar28 != 0) {
      plVar32 = local_a0 + uVar43 * 3;
      if (uVar31 < uVar43) {
        plVar32 = (longlong *)&DAT_00000008;
      }
      plVar41 = (longlong *)0x0;
      if (uVar43 < uVar31) {
        plVar41 = plVar32 + uVar36 * 3 + -3;
      }
      lVar25 = lVar25 + uVar24 * 0x18;
      if (uVar8 < uVar24) {
        lVar25 = 8;
      }
      plVar32 = (longlong *)(lVar25 + -0x18 + uVar29 * 0x18);
      if (uVar8 <= uVar24) {
        plVar32 = plVar41;
      }
      lVar44 = *plVar32;
      lVar25 = plVar32[1];
      puVar38 = (undefined8 *)*in_R9;
      local_90 = lVar25;
      if (puVar38[3] != 0) {
        local_82 = extraout_AL;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        plVar32 = (longlong *)*puVar38;
        bVar48 = (byte)(extraout_RAX_02 >> 0x39);
        auVar51 = ZEXT216(CONCAT11(bVar48,bVar48));
        auVar51 = pshuflw(auVar51,auVar51,0);
        lVar40 = 0;
        uVar31 = extraout_RAX_02;
        do {
          uVar31 = puVar38[1] & uVar31;
          auVar3 = *(undefined1 (*) [16])((longlong)plVar32 + uVar31);
          cVar49 = auVar51[0];
          auVar62[0] = -(auVar3[0] == cVar49);
          cVar52 = auVar51[1];
          auVar62[1] = -(auVar3[1] == cVar52);
          cVar53 = auVar51[2];
          auVar62[2] = -(auVar3[2] == cVar53);
          cVar54 = auVar51[3];
          auVar62[3] = -(auVar3[3] == cVar54);
          auVar62[4] = -(auVar3[4] == cVar49);
          auVar62[5] = -(auVar3[5] == cVar52);
          auVar62[6] = -(auVar3[6] == cVar53);
          auVar62[7] = -(auVar3[7] == cVar54);
          auVar62[8] = -(auVar3[8] == cVar49);
          auVar62[9] = -(auVar3[9] == cVar52);
          auVar62[10] = -(auVar3[10] == cVar53);
          auVar62[0xb] = -(auVar3[0xb] == cVar54);
          auVar62[0xc] = -(auVar3[0xc] == cVar49);
          auVar62[0xd] = -(auVar3[0xd] == cVar52);
          auVar62[0xe] = -(auVar3[0xe] == cVar53);
          auVar62[0xf] = -(auVar3[0xf] == cVar54);
          uVar42 = (ushort)(SUB161(auVar62 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar62[0xf] >> 7) << 0xf;
          uVar23 = (uint)uVar42;
          while (uVar42 != 0) {
            uVar13 = 0;
            for (uVar14 = uVar23; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
              uVar13 = uVar13 + 1;
            }
            uVar43 = uVar13 + uVar31 & puVar38[1];
            local_a0 = plVar32;
            if (lVar25 == plVar32[uVar43 * -4 + -4]) {
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_128 = lVar44;
              country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
              switch(extraout_EAX - 2U & 0xff) {
              case 0:
                break;
              case 1:
                country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting();
                if ((int)plStack_248 == 2) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                puStack_c8 = puStack_218;
                uStack_f0 = CONCAT44(uStack_23c,uStack_240);
                pplStack_e8 = (longlong **)
                              CONCAT44(fStack_234,
                                       CONCAT22(uStack_236,CONCAT11(bStack_237,bStack_238)));
                auVar51._8_8_ = pcStack_220;
                auVar51._0_8_ = local_228;
                pcStack_e0 = pcStack_230;
                ppuStack_f8 = (undefined **)plStack_248;
                auStack_d8._12_4_ = (undefined4)((ulonglong)pcStack_220 >> 0x20);
                *(undefined4 *)(in_stack_00000030 + 1) = auStack_d8._12_4_;
                auStack_d8._4_8_ = auVar51._4_8_;
                *in_stack_00000030 = auStack_d8._4_8_;
                auStack_d8 = auVar51;
                country_core::resources::walls::public_walls::PublicWalls::get();
                if (extraout_RAX_04 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                *(undefined4 *)((longlong)in_stack_00000030 + 0xc) =
                     *(undefined4 *)(extraout_RAX_04 + 0x288);
                uVar7 = *(undefined4 *)(in_stack_00000068 + 0x1c);
                puVar9 = *(undefined4 **)(in_stack_00000068 + 0x10);
                *puVar9 = uVar7;
                country_core::resources::rt::impl$5::from();
                country_core::resources::walls::decorator_archivist::DecoratorArchivist::
                archive_current_decorators_in_storage<ref_<str_>_>();
                *puVar9 = uVar7;
                country_core::resources::walls::decorator_archivist::DecoratorArchivist::
                decorator_cull_allow_wall();
                break;
              case 2:
                country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
                from_window_size();
                country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
                transform_screen_space();
                uStack_240 = *(undefined4 *)(in_stack_00000030 + 1);
                plStack_248 = (longlong *)*in_stack_00000030;
                uStack_23c = 0;
                bStack_238 = 0;
                bStack_237 = 0;
                uStack_236 = 0x3f80;
                fStack_234 = 0.0;
                uStack_c0 = 0x7f7fffffff7fffff;
                puStack_b8 = (undefined8 *)((ulonglong)puStack_b8 & 0xffffffffffffff00);
                country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
                project_onto_ray();
                fVar63 = uStack_120._4_4_ - *(float *)((longlong)in_stack_00000030 + 4);
                country_core::resources::walls::inner_walls::InnerWalls::get();
                if (extraout_RAX_07 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                fVar55 = *(float *)(extraout_RAX_07 + 0x5c);
                fVar63 = fVar63 + *(float *)((longlong)in_stack_00000030 + 0xc);
                fVar50 = 1.0;
                if (1.0 <= fVar63) {
                  fVar50 = fVar63;
                }
                fVar63 = 14.0;
                if (fVar50 <= 14.0) {
                  fVar63 = fVar50;
                }
                **(undefined4 **)(local_a8 + 0x10) = *(undefined4 *)(local_a8 + 0x1c);
                country_core::resources::walls::inner_walls::InnerWalls::update_arch_height();
                if (fVar55 != fVar63) {
                  uVar7 = *(undefined4 *)(in_stack_00000030 + 1);
                  bStack_238 = (byte)uVar7;
                  bStack_237 = (byte)((uint)uVar7 >> 8);
                  uStack_236 = (undefined2)((uint)uVar7 >> 0x10);
                  uStack_240 = (undefined4)*in_stack_00000030;
                  uStack_23c = (undefined4)((ulonglong)*in_stack_00000030 >> 0x20);
                  plStack_248 = (longlong *)((ulonglong)plStack_248 & 0xffffffffffffff00);
                  pcStack_230 = (code *)((ulonglong)pcStack_230 & 0xffffffffffffff00);
                  fStack_234 = 1.0;
                  bevy_ecs::event::writer::EventWriter::
                  write<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>();
                }
                break;
              default:
                country_core::resources::walls::public_walls::PublicWalls::get();
                if (extraout_RAX_08 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                fVar63 = *(float *)(extraout_RAX_08 + 0x288);
                fVar55 = *(float *)((longlong)in_stack_00000030 + 0xc);
                fVar50 = ABS(fVar63 - fVar55);
                if ((fVar50 < 0.05 & (byte)extraout_EAX) == 0) {
                  if (0.05 <= fVar50) {
                    country_core::resources::history::wall_compound_edit::CompoundEditBuilder::
                    new_edit_sequence();
                    uStack_240 = (undefined4)local_90;
                    uStack_23c = (undefined4)((ulonglong)local_90 >> 0x20);
                    bStack_238 = SUB41(fVar55,0);
                    bStack_237 = (byte)((uint)fVar55 >> 8);
                    uStack_236 = (undefined2)((uint)fVar55 >> 0x10);
                    plStack_248 = (longlong *)0x8000000000000006;
                    uStack_83 = 1;
                    pplVar33 = &plStack_248;
                    fStack_234 = fVar63;
                    country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
                              ();
                    **(undefined4 **)(in_stack_00000070 + 0x10) =
                         *(undefined4 *)(in_stack_00000070 + 0x1c);
                    bStack_238 = (byte)pplStack_e8;
                    bStack_237 = (byte)((ulonglong)pplStack_e8 >> 8);
                    uStack_236 = (undefined2)((ulonglong)pplStack_e8 >> 0x10);
                    fStack_234 = (float)((ulonglong)pplStack_e8 >> 0x20);
                    plStack_248 = (longlong *)ppuStack_f8;
                    uStack_240 = (undefined4)uStack_f0;
                    uStack_23c = (undefined4)((ulonglong)uStack_f0 >> 0x20);
                    uStack_83 = 0;
                    country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
                              ();
                    uStack_240 = (undefined4)extraout_RAX_11;
                    uStack_23c = (undefined4)((ulonglong)extraout_RAX_11 >> 0x20);
                    bStack_238 = (byte)pplVar33;
                    bStack_237 = (byte)((ulonglong)pplVar33 >> 8);
                    uStack_236 = (undefined2)((ulonglong)pplVar33 >> 0x10);
                    fStack_234 = (float)((ulonglong)pplVar33 >> 0x20);
                    plStack_248 = (longlong *)&DAT_00000007;
                    uStack_83 = 0;
                    country_core::resources::history::History::add();
                  }
                }
                else {
                  country_core::resources::walls::public_walls::PublicWalls::get();
                  if (extraout_RAX_09 == 0) {
                    // WARNING: Subroutine does not return
                    core::option::unwrap_failed();
                  }
                  bVar48 = *(byte *)(extraout_RAX_09 + 0x294);
                  **(undefined4 **)(local_a8 + 0x10) = *(undefined4 *)(local_a8 + 0x1c);
                  country_core::resources::walls::inner_walls::InnerWalls::update_arch_type();
                  country_core::resources::history::wall_compound_edit::CompoundEditBuilder::
                  new_edit_sequence();
                  uStack_240 = (undefined4)local_90;
                  uStack_23c = (undefined4)((ulonglong)local_90 >> 0x20);
                  plStack_248 = (longlong *)0x8000000000000007;
                  uStack_84 = 1;
                  pplVar33 = &plStack_248;
                  bStack_238 = bVar48;
                  bStack_237 = bVar48 ^ 1;
                  country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next();
                  **(undefined4 **)(in_stack_00000070 + 0x10) =
                       *(undefined4 *)(in_stack_00000070 + 0x1c);
                  bStack_238 = (byte)pplStack_e8;
                  bStack_237 = (byte)((ulonglong)pplStack_e8 >> 8);
                  uStack_236 = (undefined2)((ulonglong)pplStack_e8 >> 0x10);
                  fStack_234 = (float)((ulonglong)pplStack_e8 >> 0x20);
                  plStack_248 = (longlong *)ppuStack_f8;
                  uStack_240 = (undefined4)uStack_f0;
                  uStack_23c = (undefined4)((ulonglong)uStack_f0 >> 0x20);
                  uStack_84 = 0;
                  country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build()
                  ;
                  uStack_240 = (undefined4)extraout_RAX_10;
                  uStack_23c = (undefined4)((ulonglong)extraout_RAX_10 >> 0x20);
                  bStack_238 = (byte)pplVar33;
                  bStack_237 = (byte)((ulonglong)pplVar33 >> 8);
                  uStack_236 = (undefined2)((ulonglong)pplVar33 >> 0x10);
                  fStack_234 = (float)((ulonglong)pplVar33 >> 0x20);
                  plStack_248 = (longlong *)&DAT_00000007;
                  uStack_84 = 0;
                  country_core::resources::history::History::add();
                }
              }
              country_core::resources::walls::public_walls::PublicWalls::get();
              if (extraout_RAX_12 == 0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              uStack_104 = *(undefined4 *)(extraout_RAX_12 + 0x288);
              country_core::resources::walls::public_walls::PublicWalls::get();
              if (extraout_RAX_13 == 0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              uStack_85 = *(undefined1 *)(extraout_RAX_13 + 0x294);
              country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
              if (extraout_AL_00 == '\0') goto LAB_140768b32;
              lVar44 = local_a0[uVar43 * -4 + -2];
              puStack_100 = (undefined8 *)(local_a0[uVar43 * -4 + -1] * 0x68 + lVar44);
              uStack_138 = *in_stack_00000058;
              lVar25 = 0;
              goto LAB_14076860e;
            }
            uVar42 = (ushort)(uVar23 - 1) & (ushort)uVar23;
            uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar42);
            cVar45 = local_82;
          }
          auVar57[0] = -(auVar3[0] == -1);
          auVar57[1] = -(auVar3[1] == -1);
          auVar57[2] = -(auVar3[2] == -1);
          auVar57[3] = -(auVar3[3] == -1);
          auVar57[4] = -(auVar3[4] == -1);
          auVar57[5] = -(auVar3[5] == -1);
          auVar57[6] = -(auVar3[6] == -1);
          auVar57[7] = -(auVar3[7] == -1);
          auVar57[8] = -(auVar3[8] == -1);
          auVar57[9] = -(auVar3[9] == -1);
          auVar57[10] = -(auVar3[10] == -1);
          auVar57[0xb] = -(auVar3[0xb] == -1);
          auVar57[0xc] = -(auVar3[0xc] == -1);
          auVar57[0xd] = -(auVar3[0xd] == -1);
          auVar57[0xe] = -(auVar3[0xe] == -1);
          auVar57[0xf] = -(auVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar57 >> 7,0) & 1) != 0 || (SUB161(auVar57 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar57 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar57 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar57 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar57 >> 0x2f,0) & 1) != 0) || (SUB161(auVar57 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar57 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar57 >> 0x47,0) & 1) != 0) || (SUB161(auVar57 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar57 >> 0x57,0) & 1) != 0) || (SUB161(auVar57 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar57 >> 0x67,0) & 1) != 0) || (SUB161(auVar57 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar57 >> 0x77,0) & 1) != 0) || auVar57[0xf] < '\0') break;
          uVar31 = uVar31 + lVar40 + 0x10;
          lVar40 = lVar40 + 0x10;
        } while( true );
      }
    }
  }
  if (cVar45 == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_03 == 1) {
    local_98 = extraout_RAX_03 + 1;
  }
  else if ((*extraout_RAX_03 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_98 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140768b91:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*local_98 != 0) {
LAB_140768b9d:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *local_98 = -1;
  puffin::ThreadProfiler::end_scope();
  goto LAB_14076801c;
LAB_14076860e:
  if ((undefined8 *)lVar44 == puStack_100) goto LAB_140768b32;
  uVar31 = *(ulonglong *)(lVar44 + 0x28);
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar31;
  uVar43 = SUB168(auVar3 * ZEXT816(0xc),0);
  uStack_120 = lVar25;
  local_a8 = lVar25;
  if ((SUB168(auVar3 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar43)) {
LAB_140768be6:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  _Src = *(void **)(lVar44 + 0x20);
  if (uVar43 == 0) {
    puVar38 = (undefined8 *)&DAT_00000004;
    uVar34 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar38 = extraout_RAX_14;
    uVar34 = uVar31;
    if (extraout_RAX_14 == (undefined8 *)0x0) goto LAB_140768be6;
  }
  memcpy(puVar38,_Src,uVar43);
  puVar10 = *(undefined1 **)(lVar44 + 8);
  puVar11 = *(undefined1 **)(lVar44 + 0x20);
  uVar43 = *(ulonglong *)(lVar44 + 0x28);
  if (*(ulonglong *)(lVar44 + 0x10) < *(ulonglong *)(lVar44 + 0x28)) {
    uVar43 = *(ulonglong *)(lVar44 + 0x10);
  }
  uStack_c0 = uVar34;
  puStack_b8 = puVar38;
  if (uVar34 - uVar31 < uVar43) {
    uStack_81 = 1;
    uStack_b0 = uVar31;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    uVar31 = uStack_b0;
joined_r0x00014076899a:
    if (0xf < uVar43) {
      auVar12._8_8_ = 0;
      auVar12._0_8_ = uVar43 - 1;
      lVar25 = SUB168(auVar12 * ZEXT816(0xc),0);
      pfVar26 = (float *)((longlong)puStack_b8 + uVar31 * 0xc);
      if (((((undefined1 *)((longlong)puStack_b8 + uVar31 * 0xc + 4) <=
             (undefined1 *)((longlong)pfVar26 + lVar25 + 4)) &&
           (SUB168(auVar12 * ZEXT816(0xc),8) == 0)) &&
          ((undefined1 *)((longlong)puStack_b8 + uVar31 * 0xc + 8) <=
           (undefined1 *)((longlong)pfVar26 + lVar25 + 8))) &&
         (pfVar26 <= (float *)(lVar25 + (longlong)pfVar26))) {
        puVar27 = (undefined1 *)(uVar31 * 0xc + (longlong)puStack_b8);
        if ((puVar10 + uVar43 * 4 <= puVar27 || puVar27 + uVar43 * 0xc <= puVar10) &&
           (puVar11 + uVar43 * 0xc <= puVar27 || puVar27 + uVar43 * 0xc <= puVar11)) {
          uVar34 = uVar43 & 0xfffffffffffffffc;
          uVar31 = uVar31 + uVar34;
          pfVar30 = (float *)(puVar11 + uVar43 * 0xc + -0x30);
          lVar25 = 0;
          do {
            pfVar2 = (float *)(puVar10 + lVar25 * 4 + uVar43 * 4 + -0x10);
            fVar63 = *pfVar2;
            fVar55 = pfVar2[3];
            fVar50 = *pfVar30;
            fVar15 = pfVar30[1];
            fVar16 = pfVar30[2];
            fVar17 = pfVar30[5];
            fVar18 = pfVar30[6];
            fVar19 = pfVar30[9];
            fVar20 = pfVar30[10];
            fVar21 = pfVar30[0xb];
            fVar56 = fVar63 * 0.0;
            fVar59 = pfVar2[1] * 0.0;
            fVar60 = pfVar2[2] * 0.0;
            fVar61 = fVar55 * 0.0;
            auVar58._4_4_ = fVar60 + pfVar30[8];
            auVar58._0_4_ = pfVar30[7] + pfVar2[2];
            auVar58._8_4_ = pfVar30[3] + fVar59;
            auVar58._12_4_ = pfVar30[4] + pfVar2[1];
            *(undefined1 (*) [16])(pfVar26 + 4) = auVar58;
            *pfVar26 = fVar19 + fVar61;
            pfVar26[1] = fVar20 + fVar55;
            pfVar26[2] = fVar61 + fVar21;
            pfVar26[3] = fVar18 + fVar60;
            pfVar26[8] = fVar59 + fVar17;
            pfVar26[9] = fVar50 + fVar56;
            pfVar26[10] = fVar15 + fVar63;
            pfVar26[0xb] = fVar56 + fVar16;
            pfVar30 = pfVar30 + -0xc;
            lVar25 = lVar25 + -4;
            pfVar26 = pfVar26 + 0xc;
          } while (-lVar25 != uVar34);
          bVar46 = uVar43 == uVar34;
          uVar43 = (ulonglong)((uint)uVar43 & 3);
          if (bVar46) goto LAB_14076876b;
        }
      }
    }
    pfVar30 = (float *)((longlong)puStack_b8 + uVar31 * 0xc + 8);
    pfVar26 = (float *)(puVar11 + uVar43 * 0xc + -4);
    lVar25 = 0;
    do {
      fVar55 = *(float *)(puVar10 + lVar25 * 4 + uVar43 * 4 + -4) * 0.0;
      fVar63 = *pfVar26;
      *(ulonglong *)(pfVar30 + -2) =
           CONCAT44(*(float *)(puVar10 + lVar25 * 4 + uVar43 * 4 + -4) +
                    (float)((ulonglong)*(undefined8 *)(pfVar26 + -2) >> 0x20),
                    fVar55 + (float)*(undefined8 *)(pfVar26 + -2));
      *pfVar30 = fVar63 + fVar55;
      lVar25 = lVar25 + -1;
      pfVar30 = pfVar30 + 3;
      pfVar26 = pfVar26 + -3;
    } while (-lVar25 != uVar43);
    uVar31 = uVar31 - lVar25;
  }
  else if (uVar43 != 0) goto joined_r0x00014076899a;
LAB_14076876b:
  uStack_b0 = uVar31;
  if (uVar31 == 0) {
    uStack_81 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  plStack_248 = (longlong *)*puStack_b8;
  uStack_240 = *(undefined4 *)(puStack_b8 + 1);
  if (uVar31 == uStack_c0) {
    uStack_81 = 1;
    alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
  }
  *(undefined4 *)((longlong)puStack_b8 + uVar31 * 0xc + 8) = uStack_240;
  *(longlong **)((longlong)puStack_b8 + uVar31 * 0xc) = plStack_248;
  uStack_b0 = uVar31 + 1;
  plStack_248 = &local_90;
  uStack_240 = 0x40766e00;
  uStack_23c = 1;
  bStack_238 = (byte)&uStack_120;
  bStack_237 = (byte)((ulonglong)&uStack_120 >> 8);
  uStack_236 = (undefined2)((ulonglong)&uStack_120 >> 0x10);
  fStack_234 = (float)((ulonglong)&uStack_120 >> 0x20);
  pcStack_230 = core::fmt::num::impl$89::fmt;
  local_228 = &uStack_104;
  pcStack_220 = core::fmt::float::impl$4::fmt;
  puStack_218 = &uStack_85;
  pcStack_210 = country_core::resources::walls::inner_wall_state::impl$7::fmt;
  ppuStack_f8 = &PTR_s_highlight_arch_142a7e2a0;
  uStack_f0 = 4;
  pcStack_e0 = (code *)0x4;
  auVar22._8_8_ = 0;
  auVar22._0_8_ = auStack_d8._8_8_;
  auStack_d8 = auVar22 << 0x40;
  uStack_81 = 1;
  pplStack_e8 = &plStack_248;
  alloc::fmt::format::format_inner();
  uVar43 = uStack_b0;
  puVar38 = puStack_b8;
  uVar31 = uStack_c0;
  local_98 = (longlong *)lStack_270;
  std::alloc::__default_lib_allocator::__rust_alloc();
  local_a0 = plStack_268;
  if (extraout_RAX_15 == (undefined8 *)0x0) {
    local_a8 = uVar31;
    puStack_100 = puVar38;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_15 = 1;
  extraout_RAX_15[1] = 1;
  extraout_RAX_15[2] = uVar31;
  extraout_RAX_15[3] = puVar38;
  extraout_RAX_15[4] = uVar43;
  country_core::systems::ui::signifiers::SignifierFactory::new();
  auStack_d8._0_8_ = local_228;
  uStack_f0 = CONCAT44(uStack_23c,uStack_240);
  pplStack_e8 = (longlong **)
                CONCAT44(fStack_234,CONCAT22(uStack_236,CONCAT11(bStack_237,bStack_238)));
  pcStack_e0 = pcStack_230;
  ppuStack_f8 = (undefined **)plStack_248;
  country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
  if (local_98 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lVar44 = lVar44 + 0x68;
  lVar25 = local_a8 + 1;
  goto LAB_14076860e;
LAB_140768b32:
  if (local_82 == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_16 == 1) {
    local_98 = extraout_RAX_16 + 1;
  }
  else if ((*extraout_RAX_16 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_98 = extraout_RAX_17, extraout_RAX_17 == (longlong *)0x0))
  goto LAB_140768b91;
  if (*local_98 != 0) goto LAB_140768b9d;
  *local_98 = -1;
  puffin::ThreadProfiler::end_scope();
LAB_14076801c:
  *local_98 = *local_98 + 1;
  return;
}

