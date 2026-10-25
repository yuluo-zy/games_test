// 0x1421a8c50
// void __cdecl do_ui_edit_roof_ridge_length(void)
// do_ui_edit_roof_ridge_length


void __cdecl system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  void *_Src;
  longlong lVar6;
  undefined8 uVar7;
  undefined4 *puVar8;
  int *piVar9;
  undefined1 auVar10 [16];
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  longlong *plVar21;
  char extraout_AL;
  char extraout_AL_00;
  byte bVar22;
  uint uVar23;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  undefined8 *extraout_RAX_08;
  longlong *plVar24;
  longlong *in_RDX;
  ulonglong uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong lVar28;
  ulonglong uVar29;
  undefined8 *in_R8;
  undefined *puVar30;
  ulonglong uVar31;
  undefined8 *in_R9;
  ulonglong uVar32;
  ulonglong uVar33;
  longlong lVar34;
  undefined1 *puVar35;
  float fVar36;
  float fVar37;
  undefined4 extraout_XMM0_Da;
  float fVar38;
  float fVar39;
  undefined4 unaff_XMM6_Da;
  float fVar40;
  float fVar41;
  float fVar42;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  float *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 *in_stack_00000068;
  longlong *in_stack_00000070;
  undefined8 *in_stack_00000078;
  longlong local_290;
  ulonglong *local_268;
  code *pcStack_260;
  ulonglong **ppuStack_258;
  code *pcStack_250;
  undefined4 *local_248;
  code *local_240;
  undefined1 *local_220;
  undefined8 local_218;
  longlong local_210;
  undefined8 local_208;
  undefined8 local_200;
  undefined4 *local_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  ulonglong local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  ulonglong local_1a8;
  ulonglong local_1a0;
  undefined8 local_198;
  undefined4 local_18c;
  undefined8 uStack_188;
  undefined1 uStack_180;
  longlong local_178;
  ulonglong local_170;
  undefined8 *local_168;
  undefined1 *local_160;
  undefined8 local_158;
  float fStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 local_138;
  undefined4 local_130;
  undefined1 local_124;
  float fStack_114;
  float fStack_110;
  float fStack_10c;
  undefined8 local_108;
  undefined4 local_100;
  undefined4 local_fc;
  longlong *local_f8;
  longlong *local_f0;
  undefined8 *local_e8;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  local_f0 = in_RDX;
  puffin::are_scopes_on();
  local_168 = in_R8;
  if (extraout_AL == '\0') {
    local_178 = 0;
  }
  else {
    uVar25 = 0x44;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar25 != 0) {
        if (uVar25 < 0x44) {
          if ((char)(&DAT_142ff6f30)[uVar25] < -0x40) {
LAB_1421a8d1c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 != 0x44) goto LAB_1421a8d1c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar25 != 0xfffffffffffffffe)) {
        if (uVar25 + 2 < 0x44) {
          if ((char)(&UNK_142ff6f32)[uVar25] < -0x40) {
LAB_1421a8d7d:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 + 2 != 0x44) goto LAB_1421a8d7d;
      }
    }
    puVar30 = &DAT_142ff71ac;
    do {
      bVar22 = puVar30[-1];
      uVar23 = (uint)(char)bVar22;
      if ((char)bVar22 < '\0') {
        bVar2 = puVar30[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar30[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar30 + -4;
            puVar30 = puVar30 + -4;
            uVar23 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar30 = puVar30 + -3;
            uVar23 = bVar3 & 0xf;
          }
          uVar23 = bVar2 & 0x3f | uVar23 << 6;
        }
        else {
          puVar30 = puVar30 + -2;
          uVar23 = bVar2 & 0x1f;
        }
        uVar23 = (uint)(bVar22 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_1421a8e44;
LAB_1421a8e55:
        if (puVar30 == &UNK_142ff7177) break;
        if (puVar30 + -0x142ff7177 < (undefined *)0x34) {
          if (-0x41 < (char)puVar30[1]) break;
        }
        else if (puVar30 + -0x142ff7177 == (undefined *)0x34) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar30 = puVar30 + -1;
      if (uVar23 == 0x5c) goto LAB_1421a8e55;
LAB_1421a8e44:
      if (uVar23 == 0x2f) goto LAB_1421a8e55;
    } while (puVar30 != &DAT_142ff7178);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar24 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_1421a9271;
    if (*plVar24 != 0) goto LAB_1421a9a78;
    *plVar24 = -1;
    local_f8 = plVar24;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
    local_178 = 1;
  }
  lVar34 = in_stack_00000078[1];
  uVar25 = *(ulonglong *)*in_stack_00000078;
  uVar26 = 0;
  uVar33 = uVar25 - *(ulonglong *)(lVar34 + 0x18);
  if (uVar25 < *(ulonglong *)(lVar34 + 0x18)) {
    uVar33 = uVar26;
  }
  lVar28 = uVar33 * 0x40 + *(longlong *)(lVar34 + 8);
  uVar4 = *(ulonglong *)(lVar34 + 0x10);
  uVar5 = *(ulonglong *)(lVar34 + 0x30);
  uVar32 = uVar4 - uVar33;
  if (uVar4 < uVar33) {
    uVar32 = uVar26;
  }
  if (uVar4 < uVar33) {
    lVar28 = 8;
  }
  uVar29 = lVar28 + -0x40 + uVar32 * 0x40;
  if (uVar4 <= uVar33) {
    uVar29 = uVar26;
  }
  uVar31 = uVar25 - *(ulonglong *)(lVar34 + 0x38);
  if (uVar25 < *(ulonglong *)(lVar34 + 0x38)) {
    uVar31 = uVar26;
  }
  lVar28 = uVar31 * 0x40 + *(longlong *)(lVar34 + 0x28);
  uVar25 = uVar5 - uVar31;
  if (uVar5 < uVar31) {
    uVar25 = uVar26;
  }
  if (uVar5 < uVar31) {
    lVar28 = 8;
  }
  uVar27 = lVar28 + -0x40 + uVar25 * 0x40;
  if (uVar5 <= uVar31) {
    uVar27 = uVar29;
  }
  uVar25 = uVar25 + uVar32;
  if (uVar33 < uVar4 || uVar31 < uVar5) {
    uVar25 = uVar26;
  }
  *(ulonglong *)*in_stack_00000078 = *(longlong *)(lVar34 + 0x40) - uVar25;
  if (uVar33 < uVar4 || uVar31 < uVar5) {
    plVar24 = *(longlong **)(uVar27 + 0x10);
    auVar10._8_8_ = 0;
    auVar10._0_8_ = plVar24;
    uVar25 = SUB168(auVar10 * ZEXT816(0x18),0);
    if ((SUB168(auVar10 * ZEXT816(0x18),8) != 0) || (0x7ffffffffffffff8 < uVar25)) {
LAB_1421a9a84:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    _Src = *(void **)(uVar27 + 8);
    if (uVar25 == 0) {
      local_160 = &DAT_00000008;
      local_f8 = (longlong *)0x0;
      plVar21 = local_f8;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_160 = extraout_RAX_02;
      plVar21 = plVar24;
      if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_1421a9a84;
    }
    local_f8 = plVar21;
    memcpy(local_160,_Src,uVar25);
    local_1e8 = *(undefined8 *)(uVar27 + 0x18);
    uStack_1e0 = *(undefined8 *)(uVar27 + 0x20);
    local_1d8 = *(undefined8 *)(uVar27 + 0x28);
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_03 >> 0x20 != 0) {
      lVar34 = local_f0[1];
      uVar25 = extraout_RAX_03 & 0xffffffff;
      if ((uVar25 < *(ulonglong *)(lVar34 + 0x10)) &&
         (*(int *)(*(longlong *)(lVar34 + 8) + uVar25 * 0x14) == (int)(extraout_RAX_03 >> 0x20))) {
        lVar28 = *(longlong *)(lVar34 + 8) + uVar25 * 0x14;
        uVar25 = (ulonglong)*(uint *)(lVar28 + 4);
        if ((uVar25 != 0xffffffff) &&
           ((lVar6 = *local_f0, uVar25 < *(ulonglong *)(lVar6 + 0x38) &&
            ((*(ulonglong *)
               (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar28 + 4) >> 6) * 8) >>
              (uVar25 & 0x3f) & 1) != 0)))) {
          local_198 = 0;
          if (*(ulonglong *)(lVar6 + 0x110) <
              *(ulonglong *)
               (*(longlong *)(lVar34 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48)) {
            lVar34 = *(longlong *)(lVar34 + 0x1a8) + (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48;
            uVar25 = *(ulonglong *)
                      (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8);
            if (uVar25 != 0) {
              lVar34 = *(longlong *)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar25 * 0x30);
              goto LAB_1421a9280;
            }
          }
          lVar34 = 0;
LAB_1421a9280:
          local_e8 = (undefined8 *)((ulonglong)*(uint *)(lVar28 + 0x10) * 0x58 + lVar34);
          fVar38 = (float)((ulonglong)uStack_1e0 >> 0x20);
          fVar40 = (local_1e8._4_4_ + (float)local_1d8) * 0.5;
          fVar41 = (local_1d8._4_4_ + (float)uStack_1e0) * 0.5;
          fVar42 = ((float)local_1e8 + fVar38) * 0.5;
          local_220 = local_160 + (longlong)plVar24 * 0x18;
          local_218 = *local_168;
          local_208 = *in_R9;
          local_210 = *in_stack_00000070 + 0x2c;
          fVar38 = (float)local_1e8 - fVar38;
          fVar36 = local_1e8._4_4_ - (float)local_1d8;
          fVar39 = (float)uStack_1e0 - local_1d8._4_4_;
          local_18c = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
          local_200 = *in_stack_00000060;
          local_1f8 = (undefined4 *)in_stack_00000060[2];
          local_1f0 = *in_stack_00000068;
          local_1c0 = *in_stack_00000050;
          local_1b8 = *in_stack_00000048;
          local_1b0 = *in_stack_00000028;
          uVar25 = 0;
          local_1a0 = 0;
          local_1a8 = 0;
          for (puVar35 = local_160; puVar35 != local_220; puVar35 = puVar35 + 0x18) {
            local_fc = *(undefined4 *)(puVar35 + 8);
            local_108 = *(undefined8 *)(puVar35 + 0xc);
            local_100 = *(undefined4 *)(puVar35 + 0x14);
            local_1c8 = uVar25;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            uVar33 = local_1c8;
            if (1 < local_1c8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            local_268 = &local_1c8;
            pcStack_260 = core::fmt::num::impl$89::fmt;
            ppuStack_258 = (ulonglong **)local_210;
            pcStack_250 = core::fmt::num::impl$86::fmt;
            local_248 = &local_fc;
            local_240 = core::fmt::float::impl$4::fmt;
            local_158 = (ulonglong **)&DAT_142ff7260;
            fStack_150 = 4.2039e-45;
            uStack_14c = 0;
            uStack_140 = 3;
            uStack_13c = 0;
            local_138 = 0;
            local_148 = &local_268;
            alloc::fmt::format::format_inner();
            uVar15 = local_fc;
            local_f0 = (longlong *)local_290;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_07 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_07 = 1;
            extraout_RAX_07[1] = 1;
            *(undefined4 *)(extraout_RAX_07 + 2) = uVar15;
            *(undefined8 *)((longlong)extraout_RAX_07 + 0x14) =
                 *(undefined8 *)((longlong)&local_1e8 + uVar33 * 0xc);
            *(undefined4 *)((longlong)extraout_RAX_07 + 0x1c) =
                 *(undefined4 *)((longlong)&uStack_1e0 + uVar33 * 0xc);
            extraout_RAX_07[4] = local_108;
            *(undefined4 *)(extraout_RAX_07 + 5) = local_100;
            country_core::systems::ui::signifiers::SignifierFactory::new();
            local_248 = (undefined4 *)local_138;
            pcStack_260 = (code *)CONCAT44(uStack_14c,fStack_150);
            ppuStack_258 = local_148;
            pcStack_250 = (code *)CONCAT44(uStack_13c,uStack_140);
            local_268 = (ulonglong *)local_158;
            country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
            if (local_f0 != (longlong *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
            bVar22 = extraout_AL_00 - 2;
            if (2 < bVar22) {
              bVar22 = 3;
            }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
            switch(bVar22) {
            case 1:
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
              from_window_size();
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
              transform_screen_space();
              local_158 = (ulonglong **)CONCAT44(fVar40,fVar42);
              local_148._4_4_ = local_100;
              uStack_14c = (undefined4)local_108;
              local_148._0_4_ = (undefined4)((ulonglong)local_108 >> 0x20);
              uStack_188 = 0x7f7fffff00000000;
              uStack_180 = 0;
              fStack_150 = fVar41;
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
              project_onto_ray();
              fVar37 = SQRT((fStack_114 - fVar42) * (fStack_114 - fVar42) +
                            (fStack_110 - fVar40) * (fStack_110 - fVar40) +
                            (fStack_10c - fVar41) * (fStack_10c - fVar41));
              *in_stack_00000058 =
                   (fVar37 + fVar37) - SQRT(fVar39 * fVar39 + fVar36 * fVar36 + fVar38 * fVar38);
              *local_1f8 = local_18c;
              local_1a0 = CONCAT71((int7)((ulonglong)local_e8 >> 8),1);
              country_core::resources::walls::decorator_archivist::DecoratorArchivist::
              decorator_cull_allow_wall();
              break;
            case 2:
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
              from_window_size();
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
              transform_screen_space();
              local_158 = (ulonglong **)CONCAT44(fVar40,fVar42);
              local_148._4_4_ = local_100;
              uStack_14c = (undefined4)local_108;
              local_148._0_4_ = (undefined4)((ulonglong)local_108 >> 0x20);
              uStack_188 = 0x7f7fffff00000000;
              uStack_180 = 0;
              fStack_150 = fVar41;
              country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
              project_onto_ray();
              country_core::resources::roofs::roof_shape::Roof::shape_dims();
              country_core::resources::roofs::roof_ridge::inverse_ridge_01_param();
              local_198 = 1;
              unaff_XMM6_Da = extraout_XMM0_Da;
              break;
            case 3:
              local_1a8 = CONCAT71((int7)((ulonglong)
                                          (&DAT_142ff7370 +
                                          *(int *)(&DAT_142ff7370 + (ulonglong)bVar22 * 4)) >> 8),1)
              ;
            }
            uVar25 = uVar25 + 1;
          }
          if ((int)local_198 != 0) {
            local_138 = *local_e8;
            local_124 = 0;
            local_158 = (ulonglong **)((ulonglong)local_158 & 0xffffffff00000000);
            fStack_150 = 0.0;
            local_130 = 0;
            local_148._0_4_ = 1;
            uStack_140 = 0;
            local_148._4_4_ = unaff_XMM6_Da;
            bevy_ecs::event::writer::EventWriter::
            write<country_core::resources::roofs::commands::EditRoofCmd>();
          }
          if ((local_1a0 & 1) != 0) {
            uVar7 = *local_e8;
            uVar19 = local_e8[7];
            local_148._0_4_ = (undefined4)((ulonglong)local_e8[6] >> 0x20);
            local_148._4_4_ = (undefined4)uVar19;
            uStack_140 = (undefined4)((ulonglong)uVar19 >> 0x20);
            local_158 = *(ulonglong ***)((longlong)local_e8 + 0x24);
            uVar20 = *(undefined8 *)((longlong)local_e8 + 0x2c);
            fStack_150 = (float)uVar20;
            uStack_14c = (undefined4)((ulonglong)uVar20 >> 0x20);
            *(undefined4 *)in_stack_00000038[2] =
                 *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
            puVar8 = (undefined4 *)*in_stack_00000038;
            *puVar8 = 0;
            *(undefined8 *)(puVar8 + 4) = uVar7;
            *(ulonglong ***)(puVar8 + 6) = local_158;
            *(undefined8 *)(puVar8 + 8) = uVar20;
            *(ulonglong *)(puVar8 + 9) = CONCAT44((undefined4)local_148,uStack_14c);
            *(undefined8 *)(puVar8 + 0xb) = uVar19;
          }
          if ((((local_1a8 & 1) != 0) && (piVar9 = (int *)*in_stack_00000038, *piVar9 != 2)) &&
             ((((((float)piVar9[8] != *(float *)((longlong)local_e8 + 0x2c) ||
                 (((NAN((float)piVar9[8]) || NAN(*(float *)((longlong)local_e8 + 0x2c)) ||
                   ((float)piVar9[9] != *(float *)(local_e8 + 6))) ||
                  (NAN((float)piVar9[9]) || NAN(*(float *)(local_e8 + 6)))))) ||
                ((((float)piVar9[6] != *(float *)((longlong)local_e8 + 0x24) ||
                  (NAN((float)piVar9[6]) || NAN(*(float *)((longlong)local_e8 + 0x24)))) ||
                 ((float)piVar9[7] != *(float *)(local_e8 + 5))))) ||
               ((NAN((float)piVar9[7]) || NAN(*(float *)(local_e8 + 5)) ||
                ((float)piVar9[10] != *(float *)((longlong)local_e8 + 0x34))))) ||
              ((NAN((float)piVar9[10]) || NAN(*(float *)((longlong)local_e8 + 0x34)) ||
               (((((char)piVar9[0xc] != *(char *)((longlong)local_e8 + 0x3c) ||
                  ((float)piVar9[0xb] != *(float *)(local_e8 + 7))) ||
                 (NAN((float)piVar9[0xb]) || NAN(*(float *)(local_e8 + 7)))) ||
                (*(char *)((longlong)piVar9 + 0x31) != *(char *)((longlong)local_e8 + 0x3d)))))))))
          {
            **(undefined4 **)(in_stack_00000030 + 0x10) = *(undefined4 *)(in_stack_00000030 + 0x1c);
            uVar19 = *(undefined8 *)(piVar9 + 4);
            uVar20 = *(undefined8 *)(piVar9 + 6);
            iVar11 = piVar9[8];
            iVar12 = piVar9[9];
            iVar13 = piVar9[10];
            iVar14 = piVar9[0xb];
            uVar7 = *(undefined8 *)((longlong)local_e8 + 0x24);
            uVar15 = *(undefined4 *)((longlong)local_e8 + 0x2c);
            uVar16 = *(undefined4 *)(local_e8 + 6);
            uVar17 = *(undefined4 *)((longlong)local_e8 + 0x34);
            uVar18 = *(undefined4 *)(local_e8 + 7);
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_08 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_08 = 1;
            extraout_RAX_08[1] = 1;
            *(undefined1 *)(extraout_RAX_08 + 2) = 2;
            *(int *)((longlong)extraout_RAX_08 + 0x14) = iVar12;
            *(int *)(extraout_RAX_08 + 3) = iVar11;
            *(int *)((longlong)extraout_RAX_08 + 0x1c) = iVar13;
            *(int *)(extraout_RAX_08 + 4) = iVar14;
            *(undefined4 *)((longlong)extraout_RAX_08 + 0x24) = uVar16;
            *(undefined4 *)(extraout_RAX_08 + 5) = uVar15;
            *(undefined4 *)((longlong)extraout_RAX_08 + 0x2c) = uVar17;
            *(undefined4 *)(extraout_RAX_08 + 6) = uVar18;
            extraout_RAX_08[7] = uVar19;
            extraout_RAX_08[8] = uVar20;
            extraout_RAX_08[9] = uVar7;
            fStack_150 = SUB84(extraout_RAX_08,0);
            uStack_14c = (undefined4)((ulonglong)extraout_RAX_08 >> 0x20);
            local_158 = (ulonglong **)0x8;
            country_core::resources::history::History::add();
          }
          if (local_f8 != (longlong *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_178 == 0) {
            return;
          }
          local_268 = &local_170;
          local_158 = &local_268;
          std::thread::local::LocalKey::
          with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl_6::call::closure_env_0<tuple_<>,puffin::impl_10::drop::closure_env_0>,tuple_<>_>
                    ();
          return;
        }
      }
    }
    if (local_f8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (local_178 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      local_f0 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_f0 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1421a9271:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*local_f0 != 0) {
LAB_1421a9a78:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_f0 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_f0 = *local_f0 + 1;
  }
  return;
}


