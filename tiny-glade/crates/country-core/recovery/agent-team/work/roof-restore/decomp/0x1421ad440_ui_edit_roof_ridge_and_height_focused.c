// 0x1421ad440 void __cdecl ui_edit_roof_ridge_and_height_focused(void)

// WARNING: Control flow encountered bad instruction data

void __cdecl
system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused(void)

{
  byte *pbVar1;
  longlong lVar2;
  float fVar3;
  byte bVar4;
  byte bVar5;
  undefined8 uVar6;
  longlong lVar7;
  int *piVar8;
  undefined4 *puVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined *puVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  uint uVar21;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  byte bVar22;
  float *in_RCX;
  longlong *plVar23;
  longlong lVar24;
  longlong lVar25;
  undefined8 *in_RDX;
  undefined8 *puVar26;
  char cVar27;
  ulonglong in_R8;
  undefined *puVar28;
  longlong *in_R9;
  ulonglong uVar29;
  undefined **ppuVar30;
  float extraout_XMM0_Da;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined4 uVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  longlong in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong in_stack_00000060;
  longlong *in_stack_00000070;
  longlong in_stack_00000078;
  float *in_stack_00000080;
  undefined8 local_380;
  float fStack_378;
  undefined **local_308;
  code *pcStack_300;
  undefined8 *puStack_2f8;
  undefined8 local_2f0;
  undefined ***local_2e8;
  undefined8 *local_250;
  undefined8 local_248;
  undefined8 local_238;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  int local_1fc;
  undefined8 local_1f8;
  float local_1f0;
  undefined8 local_1ec;
  float local_1e4;
  undefined8 local_1d0 [2];
  undefined8 *local_1c0;
  undefined8 local_1b8;
  float fStack_1b0;
  int iStack_1ac;
  undefined8 local_1a8;
  float fStack_1a0;
  undefined4 uStack_19c;
  undefined8 uStack_198;
  code *local_190;
  undefined4 *local_188;
  code *local_180;
  undefined8 local_100;
  float *local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  float fStack_e0;
  undefined **local_c8;
  float local_c0;
  undefined4 local_bc;
  undefined8 *local_b8;
  longlong *local_b0;
  undefined8 local_a8;
  longlong *local_a0;
  undefined1 local_92;
  undefined1 local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  uVar6 = *in_stack_00000048;
  local_a0 = (longlong *)in_stack_00000048[4];
  local_100 = in_stack_00000048[8];
  puVar26 = in_RDX;
  local_f8 = in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_238 = 0;
  }
  else {
    in_R8 = 0x5b;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar26 != (undefined8 *)0x0) {
        if (&DAT_0000005a < puVar26) {
          if (puVar26 != (undefined8 *)0x5b) goto LAB_1421ad4fb;
        }
        else if (*(char *)(puVar26 + 0x285fef34) < -0x40) {
LAB_1421ad4fb:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (puVar26 != (undefined8 *)0xfffffffffffffffe) {
          if ((undefined1 *)((longlong)puVar26 + 2) < (undefined1 *)0x5b) {
            if (*(char *)((longlong)puVar26 + 0x142ff79a2) < -0x40) {
LAB_1421ad54f:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((undefined1 *)((longlong)puVar26 + 2) != (undefined1 *)0x5b) goto LAB_1421ad54f;
        }
        in_R8 = 0x59 - (longlong)puVar26;
      }
    }
    puVar28 = &DAT_142ff7bf9;
    do {
      bVar22 = puVar28[-1];
      uVar21 = (uint)(char)bVar22;
      local_a8 = uVar6;
      if ((char)bVar22 < '\0') {
        bVar4 = puVar28[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar28[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar21 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar21 = bVar5 & 0xf;
          }
          uVar21 = bVar4 & 0x3f | uVar21 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar21 = bVar4 & 0x1f;
        }
        uVar21 = (uint)(bVar22 & 0x3f) | uVar21 << 6;
        if (uVar21 != 0x5c) goto LAB_1421ad624;
LAB_1421ad63b:
        if (puVar28 == &UNK_142ff7bb7) break;
        if (puVar28 + -0x142ff7bb7 < (undefined *)0x41) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142ff7bb7 == (undefined *)0x41) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar21 == 0x5c) goto LAB_1421ad63b;
LAB_1421ad624:
      if (uVar21 == 0x2f) goto LAB_1421ad63b;
    } while (puVar28 != &DAT_142ff7bb8);
    local_b8 = in_RDX;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      local_b0 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_1421ae4ba;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      local_b0 = extraout_RAX_08;
      if (extraout_RAX_08 == (longlong *)0x0) goto LAB_1421ae4ba;
    }
    if (*local_b0 != 0) goto LAB_1421ae4c6;
    *local_b0 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_b0 = *local_b0 + 1;
    local_238 = 1;
    in_RDX = local_b8;
  }
  uVar21 = *(uint *)*in_RDX;
  cVar27 = extraout_AL;
  local_1a8 = (undefined ***)CONCAT44(local_1a8._4_4_,(float)local_1a8);
  uStack_198 = (undefined ***)CONCAT44(uStack_198._4_4_,(undefined4)uStack_198);
  if ((uVar21 < 6) &&
     ((uVar21 == 2 ||
      (cVar27 = extraout_AL, local_1a8 = (undefined ***)CONCAT44(local_1a8._4_4_,(float)local_1a8),
      uStack_198 = (undefined ***)CONCAT44(uStack_198._4_4_,(undefined4)uStack_198), uVar21 == 1))))
  {
    local_a8 = CONCAT71(local_a8._1_7_,extraout_AL);
    local_1d0[0] = *(undefined8 *)((uint *)*in_RDX + 2);
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_02 >> 0x20 == 0) {
      if ((char)local_a8 == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_03 == 1) {
        plVar23 = extraout_RAX_03 + 1;
      }
      else {
        if (*extraout_RAX_03 != 0) goto LAB_1421ae4ba;
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        plVar23 = extraout_RAX_11;
        if (extraout_RAX_11 == (longlong *)0x0) goto LAB_1421ae4ba;
      }
      if (*plVar23 != 0) goto LAB_1421ae4c6;
      *plVar23 = -1;
      local_a0 = plVar23;
      puffin::ThreadProfiler::end_scope();
      goto LAB_1421ae450;
    }
    lVar24 = in_R9[1];
    uVar29 = extraout_RAX_02 & 0xffffffff;
    iStack_1ac = (int)(extraout_RAX_02 >> 0x20);
    if ((uVar29 < *(ulonglong *)(lVar24 + 0x10)) &&
       (in_R8 = *(ulonglong *)(lVar24 + 8), *(int *)(in_R8 + uVar29 * 0x14) == iStack_1ac)) {
      lVar2 = in_R8 + uVar29 * 0x14;
      uVar21 = *(uint *)(lVar2 + 4);
      in_R8 = (ulonglong)uVar21;
      if (in_R8 != 0xffffffff) {
        lVar7 = *in_R9;
        uVar33 = 0;
        if ((*(ulonglong *)(lVar7 + 0x38) <= in_R8) ||
           ((*(ulonglong *)(*(longlong *)(lVar7 + 0x28) + (ulonglong)(uVar21 >> 6) * 8) >>
             (in_R8 & 0x3f) & 1) == 0)) goto LAB_1421ae4e2;
        if (*(ulonglong *)(lVar7 + 0x110) <
            *(ulonglong *)
             (*(longlong *)(lVar24 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48)) {
          lVar24 = *(longlong *)(lVar24 + 0x1a8) + (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48;
          uVar29 = *(ulonglong *)(*(longlong *)(lVar24 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
          if (uVar29 == 0) goto LAB_1421ad862;
          lVar24 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar29 * 0x30);
        }
        else {
LAB_1421ad862:
          lVar24 = 0;
        }
        ppuVar30 = (undefined **)((ulonglong)*(uint *)(lVar2 + 0x10) * 0x58 + lVar24);
        country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
        country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space();
        fVar34 = (float)local_1f8;
        fVar35 = (float)((ulonglong)local_1f8 >> 0x20);
        fVar36 = local_1f0;
        if (local_1fc == 1) {
          fVar34 = (fVar34 + (float)local_1ec) * 0.5;
          fVar35 = (fVar35 + (float)((ulonglong)local_1ec >> 0x20)) * 0.5;
          fVar36 = (local_1f0 + local_1e4) * 0.5;
        }
        local_c8 = (undefined **)CONCAT44(fVar35 + 0.75,fVar34 + 0.0);
        local_c0 = fVar36 + 0.0;
        ::utils::camera::CameraMatrices::eye_right();
        ::utils::xz_rotation_bad();
        fStack_1a0 = local_c0 + 0.0;
        local_1a8._0_4_ = SUB84(local_c8,0) + 0.0;
        local_1a8._4_4_ = (float)((ulonglong)local_c8 >> 0x20) + 0.5;
        local_1b8 = (undefined **)local_228;
        fStack_1b0 = (float)uStack_220;
        iStack_1ac = (int)((ulonglong)uStack_220 >> 0x20);
        uStack_19c = 0x3f800000;
        uStack_198._0_4_ = 0x3f800000;
        uStack_198._4_4_ = 0x3f800000;
        uVar33 = 0x3f800000;
        country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
        box_trigger_volume();
        local_91 = 1;
        country_core::input::action_set::DigitalActionState::pressed();
        if (extraout_AL_00 == '\0') {
          local_91 = 1;
          country_core::systems::ui::resources::widget_params::UiSounds::boop();
          pcStack_300 = (code *)0x2;
          local_308 = (undefined **)CONCAT44(local_308._4_4_,1);
          local_91 = 1;
          country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
        }
        else {
          local_91 = 1;
          country_core::systems::ui::resources::widget_params::UiSounds::empty();
        }
        pcStack_300 = country_core::resources::walls::impl$7::fmt;
        local_1b8 = &PTR_s_roof_edit__142ff7c28;
        fStack_1b0 = 1.4013e-45;
        iStack_1ac = 0;
        local_1a8 = &local_308;
        fStack_1a0 = 1.4013e-45;
        uStack_19c = 0;
        uStack_198._0_4_ = 0;
        uStack_198._4_4_ = 0;
        local_91 = 1;
        local_308 = ppuVar30;
        alloc::fmt::format::format_inner();
        local_e8 = local_380;
        fStack_e0 = fStack_378;
        local_2f0 = local_208;
        pcStack_300 = (code *)local_218;
        puStack_2f8 = (undefined8 *)uStack_210;
        local_308 = (undefined **)CONCAT71(local_308._1_7_,2);
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::new();
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
        local_92 = 1;
        country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
        local_92 = 0;
        puVar26 = &local_e8;
        country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        local_91 = 0;
        local_1c0 = puVar26;
        country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered();
        local_1b8 = (undefined **)CONCAT71(local_1b8._1_7_,extraout_AL_01);
        local_91 = 0;
        country_core::resources::walls::inner_wall_state::StructureType::is_fence();
        if (extraout_AL_02 == '\0') {
          local_91 = 0;
          country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
          if (extraout_AL_03 != '\x02') goto LAB_1421adbfe;
        }
        else {
LAB_1421adbfe:
          uVar33 = 0x3f99999a;
        }
        local_1b8 = (undefined **)local_1d0;
        fStack_1b0 = 38.7063;
        iStack_1ac = 1;
        fStack_1a0 = 6.646431;
        uStack_19c = 1;
        uStack_198 = &local_c8;
        local_190 = glam::f32::vec3::impl$93::fmt;
        local_188 = &local_bc;
        local_180 = core::fmt::float::impl$5::fmt;
        local_308 = &PTR_s_height__arrow___142ff7c48;
        pcStack_300 = (code *)0x4;
        local_2f0 = 4;
        local_2e8 = (undefined ***)0x0;
        local_91 = 0;
        puStack_2f8 = &local_1b8;
        local_bc = uVar33;
        local_1a8 = (undefined ***)&local_228;
        alloc::fmt::format::format_inner();
        uVar33 = local_bc;
        local_b8 = local_250;
        local_1b8 = local_c8;
        fStack_1b0 = local_c0;
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_f0 = local_248;
        if (extraout_RAX_04 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_04 = 1;
        extraout_RAX_04[1] = 1;
        extraout_RAX_04[2] = local_228;
        extraout_RAX_04[3] = uStack_220;
        *(undefined4 *)(extraout_RAX_04 + 4) = uVar33;
        *(undefined ***)((longlong)extraout_RAX_04 + 0x24) = local_1b8;
        *(float *)((longlong)extraout_RAX_04 + 0x2c) = fStack_1b0;
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_2e8 = uStack_198;
        pcStack_300 = (code *)CONCAT44(iStack_1ac,fStack_1b0);
        puStack_2f8 = local_1a8;
        local_2f0 = CONCAT44(uStack_19c,fStack_1a0);
        local_308 = local_1b8;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        if (local_b8 != (undefined8 *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        fVar3 = *(float *)((longlong)ppuVar30 + 0x4c);
        local_91 = 0;
        country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
        bVar22 = 3;
        if ((byte)(extraout_AL_04 - 2U) < 3) {
          bVar22 = extraout_AL_04 - 2U;
        }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar22) {
        case 0:
          break;
        case 1:
          local_91 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size();
          local_91 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
          transform_screen_space();
          local_1b8 = (undefined **)CONCAT44(fVar3,fVar34);
          iStack_1ac = 0;
          local_1a8._0_4_ = 1.0;
          local_1a8._4_4_ = 0.0;
          local_91 = 0;
          fStack_1b0 = fVar36;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
          project_onto_ray();
          auVar31._4_4_ = fVar36;
          auVar31._0_4_ = fVar36;
          auVar31._8_8_ = 0;
          auVar32._4_4_ =
               (fVar36 - fVar36) * (fVar36 - fVar36) +
               (fVar35 - fVar3) * (fVar35 - fVar3) + (fVar34 - fVar34) * (fVar34 - fVar34);
          auVar32._0_4_ =
               (fStack_e0 - fVar36) * (fStack_e0 - fVar36) +
               (local_e8._4_4_ - fVar3) * (local_e8._4_4_ - fVar3) +
               ((float)local_e8 - fVar34) * ((float)local_e8 - fVar34);
          auVar32._8_8_ = 0;
          auVar32 = sqrtps(auVar31,auVar32);
          *local_f8 = auVar32._0_4_ - auVar32._4_4_;
          uVar33 = *(undefined4 *)(in_stack_00000060 + 0x1c);
          puVar9 = *(undefined4 **)(in_stack_00000060 + 0x10);
          *puVar9 = uVar33;
          local_91 = 0;
          country_core::resources::rt::impl$5::from();
          local_91 = 0;
          country_core::resources::walls::decorator_archivist::DecoratorArchivist::
          archive_current_decorators_in_storage<ref_<str_>_>();
          *puVar9 = uVar33;
          local_91 = 0;
          country_core::resources::walls::decorator_archivist::DecoratorArchivist::
          decorator_cull_allow_wall();
          local_91 = 0;
          country_core::resources::walls::inner_walls::InnerWalls::get();
          if (extraout_RAX_05 == 0) {
            local_91 = 0;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          *in_stack_00000080 = *(float *)(extraout_RAX_05 + 0x54);
          puVar28 = *ppuVar30;
          puVar18 = ppuVar30[7];
          local_1a8._0_4_ = (float)((ulonglong)ppuVar30[6] >> 0x20);
          local_1a8._4_4_ = SUB84(puVar18,0);
          fStack_1a0 = (float)((ulonglong)puVar18 >> 0x20);
          local_1b8 = *(undefined ***)((longlong)ppuVar30 + 0x24);
          uVar6 = *(undefined8 *)((longlong)ppuVar30 + 0x2c);
          fStack_1b0 = (float)uVar6;
          iStack_1ac = (int)((ulonglong)uVar6 >> 0x20);
          *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
          puVar9 = (undefined4 *)*in_stack_00000040;
          *puVar9 = 0;
          *(undefined **)(puVar9 + 4) = puVar28;
          *(undefined ***)(puVar9 + 6) = local_1b8;
          *(undefined8 *)(puVar9 + 8) = uVar6;
          *(ulonglong *)(puVar9 + 9) = CONCAT44((float)local_1a8,iStack_1ac);
          *(undefined **)(puVar9 + 0xb) = puVar18;
          break;
        case 2:
          local_91 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size();
          local_91 = 0;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::
          transform_screen_space();
          local_1b8 = (undefined **)CONCAT44(fVar3,fVar34);
          iStack_1ac = 0;
          local_1a8._0_4_ = 1.0;
          local_1a8._4_4_ = 0.0;
          local_91 = 0;
          fStack_1b0 = fVar36;
          country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
          project_onto_ray();
          local_91 = 0;
          ::utils::inverse_lerp();
          fVar36 = 0.0;
          if (0.0 <= extraout_XMM0_Da) {
            fVar36 = extraout_XMM0_Da;
          }
          fVar34 = 1.0;
          if (fVar36 <= 1.0) {
            fVar34 = fVar36;
          }
          **(undefined4 **)(in_stack_00000078 + 0x10) = *(undefined4 *)(in_stack_00000078 + 0x1c);
          if (*in_stack_00000080 < 1.6) {
            fVar36 = *(float *)(ppuVar30 + 6);
            fVar35 = (fVar34 - fVar36) + fVar36;
            if (((0.1 < fVar36) && (fVar35 <= 0.1)) || ((fVar36 <= 0.1 && (0.1 < fVar35)))) {
              local_91 = 0;
              country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height();
            }
          }
          puVar28 = *ppuVar30;
          *(undefined4 *)in_stack_00000070[2] = *(undefined4 *)((longlong)in_stack_00000070 + 0x1c);
          lVar24 = *in_stack_00000070;
          lVar2 = *(longlong *)(lVar24 + 0x30);
          uVar6 = *(undefined8 *)(lVar24 + 0x40);
          if (lVar2 == *(longlong *)(lVar24 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
          }
          lVar7 = *(longlong *)(lVar24 + 0x28);
          lVar25 = lVar2 * 0x40;
          *(undefined4 *)(lVar7 + lVar25) = 0;
          *(undefined4 *)(lVar7 + 8 + lVar25) = 1;
          *(float *)(lVar7 + 0xc + lVar25) = fVar34;
          *(undefined4 *)(lVar7 + 0x10 + lVar25) = 0;
          *(undefined4 *)(lVar7 + 0x18 + lVar25) = 0;
          *(undefined **)(lVar7 + 0x20 + lVar25) = puVar28;
          *(undefined4 *)(lVar7 + 0x28 + lVar25) = 0;
          *(undefined1 *)(lVar7 + 0x34 + lVar25) = 0;
          *(undefined8 *)(lVar7 + 0x38 + lVar25) = uVar6;
          *(longlong *)(lVar24 + 0x30) = lVar2 + 1;
          *(longlong *)(lVar24 + 0x40) = *(longlong *)(lVar24 + 0x40) + 1;
          break;
        case 3:
          piVar8 = (int *)*in_stack_00000040;
          if ((*piVar8 != 2) &&
             ((((((((float)piVar8[8] != *(float *)((longlong)ppuVar30 + 0x2c) ||
                   (NAN((float)piVar8[8]) || NAN(*(float *)((longlong)ppuVar30 + 0x2c)))) ||
                  ((float)piVar8[9] != *(float *)(ppuVar30 + 6))) ||
                 ((NAN((float)piVar8[9]) || NAN(*(float *)(ppuVar30 + 6)) ||
                  ((float)piVar8[6] != *(float *)((longlong)ppuVar30 + 0x24))))) ||
                ((NAN((float)piVar8[6]) || NAN(*(float *)((longlong)ppuVar30 + 0x24)) ||
                 (((float)piVar8[7] != *(float *)(ppuVar30 + 5) ||
                  (NAN((float)piVar8[7]) || NAN(*(float *)(ppuVar30 + 5)))))))) ||
               ((float)piVar8[10] != *(float *)((longlong)ppuVar30 + 0x34))) ||
              ((((NAN((float)piVar8[10]) || NAN(*(float *)((longlong)ppuVar30 + 0x34)) ||
                 ((char)piVar8[0xc] != *(char *)((longlong)ppuVar30 + 0x3c))) ||
                ((float)piVar8[0xb] != *(float *)(ppuVar30 + 7))) ||
               ((NAN((float)piVar8[0xb]) || NAN(*(float *)(ppuVar30 + 7)) ||
                (*(char *)((longlong)piVar8 + 0x31) != *(char *)((longlong)ppuVar30 + 0x3d)))))))))
          {
            uVar33 = *(undefined4 *)(in_stack_00000038 + 0x1c);
            puVar9 = *(undefined4 **)(in_stack_00000038 + 0x10);
            *puVar9 = uVar33;
            uVar19 = *(undefined8 *)(piVar8 + 4);
            uVar20 = *(undefined8 *)(piVar8 + 6);
            iVar10 = piVar8[8];
            iVar11 = piVar8[9];
            iVar12 = piVar8[10];
            iVar13 = piVar8[0xb];
            uVar6 = *(undefined8 *)((longlong)ppuVar30 + 0x24);
            uVar14 = *(undefined4 *)((longlong)ppuVar30 + 0x2c);
            uVar15 = *(undefined4 *)(ppuVar30 + 6);
            uVar16 = *(undefined4 *)((longlong)ppuVar30 + 0x34);
            uVar17 = *(undefined4 *)(ppuVar30 + 7);
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_06 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_06 = 1;
            extraout_RAX_06[1] = 1;
            *(undefined1 *)(extraout_RAX_06 + 2) = 2;
            *(int *)((longlong)extraout_RAX_06 + 0x14) = iVar11;
            *(int *)(extraout_RAX_06 + 3) = iVar10;
            *(int *)((longlong)extraout_RAX_06 + 0x1c) = iVar12;
            *(int *)(extraout_RAX_06 + 4) = iVar13;
            *(undefined4 *)((longlong)extraout_RAX_06 + 0x24) = uVar15;
            *(undefined4 *)(extraout_RAX_06 + 5) = uVar14;
            *(undefined4 *)((longlong)extraout_RAX_06 + 0x2c) = uVar16;
            *(undefined4 *)(extraout_RAX_06 + 6) = uVar17;
            extraout_RAX_06[7] = uVar19;
            extraout_RAX_06[8] = uVar20;
            extraout_RAX_06[9] = uVar6;
            fStack_1b0 = SUB84(extraout_RAX_06,0);
            iStack_1ac = (int)((ulonglong)extraout_RAX_06 >> 0x20);
            local_1b8 = (undefined **)0x8;
            country_core::resources::history::History::add();
            country_core::resources::walls::inner_walls::InnerWalls::get();
            if (extraout_RAX_07 == 0) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            if ((*in_stack_00000080 != *(float *)(extraout_RAX_07 + 0x54)) ||
               (NAN(*in_stack_00000080) || NAN(*(float *)(extraout_RAX_07 + 0x54)))) {
              *puVar9 = uVar33;
              country_core::resources::history::wall_history::impl$0::ev_height_changed();
            }
          }
          break;
        default:
                    // WARNING: Bad instruction - Truncating control flow here
          halt_baddata();
        case 7:
        case 9:
                    // WARNING: Could not recover jumptable at 0x000142ff7d37. Too many branches
                    // WARNING: Treating indirect jump as call
          (**(code **)(in_stack_00000078 + 0xff1b64))();
          return;
        }
        cVar27 = (char)local_a8;
        goto LAB_1421ae40f;
      }
    }
    uVar21 = (uint)in_R8;
    uVar33 = 1;
LAB_1421ae4e2:
    local_1b8 = (undefined **)CONCAT44(uVar21,uVar33);
    fStack_1b0 = (float)extraout_RAX_02;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
LAB_1421ae40f:
  if (cVar27 == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_09 == 1) {
    plVar23 = extraout_RAX_09 + 1;
  }
  else {
    if (*extraout_RAX_09 != 0) {
LAB_1421ae4ba:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    std::sys::thread_local::native::lazy::Storage::
    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
              ();
    plVar23 = extraout_RAX_10;
    if (extraout_RAX_10 == (longlong *)0x0) goto LAB_1421ae4ba;
  }
  if (*plVar23 != 0) {
LAB_1421ae4c6:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar23 = -1;
  local_a0 = plVar23;
  puffin::ThreadProfiler::end_scope();
LAB_1421ae450:
  *local_a0 = *local_a0 + 1;
  return;
}


