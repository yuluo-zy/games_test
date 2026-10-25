// ui_settings_video
// sig: void __cdecl ui_settings_video(void)
// size: 10058

// WARNING: Type propagation algorithm not settling

void __cdecl tiny_glade::gui::settings::ui_settings_video(void)

{
  longlong *plVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  uint uVar4;
  longlong lVar5;
  undefined4 *puVar6;
  void *pvVar7;
  size_t sVar8;
  longlong lVar9;
  code *pcVar10;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  byte extraout_AL_05;
  byte extraout_AL_06;
  byte extraout_AL_07;
  char extraout_AL_08;
  byte extraout_AL_09;
  byte extraout_AL_10;
  byte extraout_AL_11;
  byte extraout_AL_12;
  char extraout_AL_13;
  char extraout_AL_14;
  char extraout_AL_15;
  char extraout_AL_16;
  byte extraout_AL_17;
  char extraout_AL_18;
  int iVar11;
  char *extraout_RAX;
  char *extraout_RAX_00;
  uint *******extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar12;
  uint *******pppppppuVar13;
  char cVar14;
  char cVar15;
  int iVar16;
  longlong *in_RCX;
  uint *puVar17;
  char cVar18;
  undefined4 uVar19;
  char *in_RDX;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined1 (*pauVar22) [16];
  undefined8 *puVar23;
  longlong *plVar24;
  char *pcVar25;
  longlong lVar26;
  longlong *in_R8;
  longlong lVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  uint ******ppppppuVar30;
  uint uVar31;
  byte bVar32;
  uint *******pppppppuVar33;
  uint *******pppppppuVar34;
  uint *******pppppppuVar35;
  bool bVar36;
  float extraout_XMM0_Da;
  float fVar37;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  float fVar42;
  float fVar43;
  byte *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong local_338;
  longlong local_320;
  longlong local_308;
  longlong local_2f0;
  uint *****local_2d8;
  undefined8 local_2d0;
  uint *****local_2c8;
  undefined8 local_2c0;
  uint *******local_2b8 [3];
  uint *****local_2a0 [3];
  uint ******local_288;
  undefined8 uStack_280;
  uint *****local_268;
  undefined8 local_260;
  uint *****local_258;
  undefined8 local_250;
  uint *local_248;
  char local_240;
  undefined7 uStack_23f;
  undefined8 uStack_238;
  undefined8 local_230;
  uint *****local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  uint ******local_208;
  longlong *local_200;
  uint *******local_1f8;
  float local_1ec;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  uint *******local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined4 local_1a8;
  uint *******local_138;
  code *local_130;
  uint *******local_128;
  undefined4 local_11c;
  uint *******local_118;
  uint *******pppppppuStack_110;
  uint *******local_108;
  char *local_e0;
  uint *******local_d8;
  uint ******local_d0;
  uint *******local_c8;
  uint *******pppppppuStack_c0;
  uint *******local_b8;
  code *local_b0;
  uint ******local_98;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  if (*in_RDX == '\0') {
    uVar20 = 0x19;
    lang_lib::lookup();
    local_130 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_1e8._0_4_ = 0x429a0d00;
    local_1e8._4_4_ = 1;
    uStack_1e0._0_4_ = 1;
    uStack_1e0._4_4_ = 0.0;
    uStack_1d0 = 1;
    local_1c8 = 0;
    local_1d8 = (uint *******)&local_138;
    local_138 = (uint *******)&local_118;
    pppppppuStack_110 = (uint *******)uVar20;
    alloc::fmt::format::format_inner();
    local_78 = (char *)pppppppuStack_c0;
    toolbar::mei::widgets::impl$26::settings_button();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (local_c8 != (uint *******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (extraout_AL != '\0') {
      uStack_1e0._0_4_ = CONCAT31(uStack_1e0._1_3_,1);
      local_1e8._0_4_ = 7;
      local_1e8._4_4_ = 0;
      country_core::resources::gui::NextEscMenuState::set();
    }
    uVar20 = 0x1b;
    lang_lib::lookup();
    local_130 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_1e8._0_4_ = 0x429a0d00;
    local_1e8._4_4_ = 1;
    uStack_1e0._0_4_ = 1;
    uStack_1e0._4_4_ = 0.0;
    uStack_1d0 = 1;
    local_1c8 = 0;
    local_1d8 = (uint *******)&local_138;
    local_138 = (uint *******)&local_118;
    pppppppuStack_110 = (uint *******)uVar20;
    alloc::fmt::format::format_inner();
    local_80 = pppppppuStack_c0;
    toolbar::mei::widgets::impl$26::settings_button();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (local_c8 != (uint *******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (extraout_AL_00 == '\0') {
      return;
    }
    uStack_1e0._0_4_ = CONCAT31(uStack_1e0._1_3_,2);
    local_1e8._0_4_ = 7;
    local_1e8._4_4_ = 0;
    country_core::resources::gui::NextEscMenuState::set();
    return;
  }
  if (*in_RDX != '\x01') {
    plVar24 = (longlong *)*in_R8;
    local_78 = (char *)*in_stack_00000038;
    local_98 = (uint ******)*in_stack_00000040;
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::label();
    toolbar::mei::widgets::LabelView::content_mut();
    tg_text::TextFragment::set_metrics<f32>();
    local_b8 = local_1d8;
    local_c8 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
    pppppppuStack_c0 = (uint *******)CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
    toolbar::mei::widgets::LabelView::weight();
    local_e0 = (char *)in_RCX[2];
    local_88 = CONCAT44(local_88._4_4_,*(undefined4 *)((longlong)in_RCX + 0x1c));
    *(undefined4 *)local_e0 = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar5 = *in_RCX;
    if ((*(char *)(lVar5 + 0xac) == '\x01') &&
       (pppppppuVar33 = (uint *******)plVar24[1], pppppppuVar33 != (uint *******)0x0)) {
      lVar27 = 0;
      ppppppuVar30 = (uint ******)0x0;
LAB_140247c10:
      if (*(longlong *)(*plVar24 + 0x80 + lVar27) != *(longlong *)(lVar5 + 0xb5) ||
          *(longlong *)(*plVar24 + 0x78 + lVar27) != *(longlong *)(lVar5 + 0xad))
      goto code_r0x000140247c2c;
      local_138 = (uint *******)ppppppuVar30;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (uint *******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      local_1e8 = (uint *******)&local_b8;
      local_b8 = (uint *******)0x0;
      uStack_1e0._0_4_ = 0;
      uStack_1e0._4_4_ = 0.0;
      local_c8 = pppppppuVar33;
      pppppppuStack_c0 = extraout_RAX_01;
      core::iter::adapters::map::impl$2::
      fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env_1,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<alloc::string::String,alloc::vec::impl_20::extend_trusted::closure_env_0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::ui_settings_video_display::closure_env_1>_>_>_>
                ();
      local_118 = local_c8;
      pppppppuStack_110 = pppppppuStack_c0;
      local_108 = local_b8;
      lang_lib::lookup();
      lang_lib::lookup();
      toolbar::mei::widgets::tweaker::HintTracker::text();
      toolbar::mei::widgets::tweaker::impl$11::default();
      pppppppuVar33 = local_108;
      local_d8 = pppppppuStack_110;
      local_1e8._0_4_ = (uint)pppppppuStack_110;
      local_1e8._4_4_ = (undefined4)((ulonglong)pppppppuStack_110 >> 0x20);
      uStack_1e0._0_4_ = SUB84(local_108,0);
      uStack_1e0._4_4_ = (float)((ulonglong)local_108 >> 0x20);
      local_1d8 = (uint *******)&local_138;
      toolbar::mei::widgets::tweaker::impl$13::
      tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>_>();
      if (extraout_AL_03 != '\0') {
        *in_stack_00000030 = 1;
        if ((uint ******)plVar24[1] <= local_138) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        plVar1 = (longlong *)(*plVar24 + 0x78 + (longlong)local_138 * 0xa0);
        lVar27 = *plVar1;
        lVar26 = plVar1[1];
        local_1e8._0_4_ = (uint)lVar27;
        local_1e8._4_4_ = (undefined4)((ulonglong)lVar27 >> 0x20);
        uStack_1e0._0_4_ = (undefined4)lVar26;
        uStack_1e0._4_4_ = (float)((ulonglong)lVar26 >> 0x20);
        *(undefined4 *)local_e0 = (undefined4)local_88;
        *(undefined1 *)(lVar5 + 0xac) = 1;
        *(longlong *)(lVar5 + 0xad) = lVar27;
        *(longlong *)(lVar5 + 0xb5) = lVar26;
      }
      country_core::render::impl$0::deref();
      rhapsody::raw::device::GraphicsDevice::device_uuid();
      if ((*(char *)(lVar5 + 0xac) != '\x01') ||
         (CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0) != *(longlong *)(lVar5 + 0xb5) ||
          CONCAT44(local_1e8._4_4_,(uint)local_1e8) != *(longlong *)(lVar5 + 0xad))) {
        if ((uint ******)plVar24[1] <= local_138) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        country_core::systems::save::load_completion_checker::SessionLoadReadiness::deny_load();
        if (extraout_AL_04 == '\0') {
          lang_lib::lookup();
          toolbar::mei::widgets::impl$26::label();
          toolbar::mei::widgets::LabelView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          local_b8 = local_1d8;
          local_c8 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
          pppppppuStack_c0 = (uint *******)CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
          toolbar::mei::widgets::LabelView::color();
          toolbar::mei::widgets::ActionHintView::into_inner();
        }
        else {
          lang_lib::lookup();
          toolbar::mei::widgets::impl$26::label();
          toolbar::mei::widgets::LabelView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          local_b8 = local_1d8;
          local_c8 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
          pppppppuStack_c0 = (uint *******)CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
          toolbar::mei::widgets::LabelView::color();
          toolbar::mei::widgets::ActionHintView::into_inner();
        }
      }
      if (pppppppuVar33 != (uint *******)0x0) {
        pppppppuVar13 = local_d8 + 1;
        do {
          if (pppppppuVar13[-1] != (uint ******)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          pppppppuVar13 = pppppppuVar13 + 3;
          pppppppuVar33 = (uint *******)((longlong)pppppppuVar33 + -1);
        } while (pppppppuVar33 != (uint *******)0x0);
      }
      if (local_118 != (uint *******)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
LAB_140249304:
    uVar19 = (undefined4)local_88;
    *(undefined4 *)local_e0 = (undefined4)local_88;
    local_240 = '\0';
    if (*(char *)(lVar5 + 0x7c) != '\0') {
      local_240 = (*(byte *)(lVar5 + 0x7d) & 1) + 1;
    }
    lang_lib::lookup();
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>_>
              ();
    ppppppuVar30 = local_98;
    if (extraout_AL_13 == '\0') {
      if (local_240 != '\0') goto LAB_1402493ed;
    }
    else {
      *in_stack_00000030 = 1;
      if (local_240 != '\0') {
        *(undefined1 *)(lVar5 + 0x7c) = 1;
        *(bool *)(lVar5 + 0x7d) = local_240 != '\x01';
LAB_1402493ed:
        alloc::vec::spec_from_iter::impl$0::
        from_iter<ref_<alloc::string::String>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::window::AvailableMonitor>,tiny_glade::gui::settings::ui_settings_video_display::closure_env_2>_>
                  ();
        pppppppuVar33 = local_1d8;
        local_78 = (char *)CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
        if (local_1d8 < (uint *******)0x2) {
          pppppppuVar13 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
          if (local_1d8 == (uint *******)0x0) goto LAB_14024964d;
          lVar27 = *(longlong *)(lVar5 + 0x38);
        }
        else {
          if (&DAT_00000014 < local_1d8) {
            core::slice::sort::stable::
            driftsort_main<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>),alloc::vec::Vec<ref_<alloc::string::String>,alloc::alloc::Global>_>
                      ();
          }
          else {
            core::slice::sort::shared::smallsort::
            insertion_sort_shift_left<ref_<alloc::string::String>,bool_(*)(ref_<ref_<alloc::string::String>_>,ref_<ref_<alloc::string::String>_>)>
                      ();
          }
          pppppppuVar13 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
          lVar27 = *(longlong *)(lVar5 + 0x38);
          uVar19 = (undefined4)local_88;
          ppppppuVar30 = local_98;
        }
        if (-0x7fffffffffffffff < lVar27) {
          local_200 = (longlong *)(lVar5 + 0x38);
          pvVar7 = *(void **)(lVar5 + 0x40);
          sVar8 = *(size_t *)(lVar5 + 0x48);
          lVar27 = (longlong)pppppppuVar33 * 8;
          pppppppuVar34 = (uint *******)0x0;
          local_d8 = pppppppuVar13;
          do {
            lVar26 = *(longlong *)((longlong)local_78 + (longlong)pppppppuVar34 * 8);
            if ((*(size_t *)(lVar26 + 0x10) == sVar8) &&
               (iVar11 = memcmp(*(void **)(lVar26 + 8),pvVar7,sVar8), iVar11 == 0)) {
              local_118 = pppppppuVar34;
              lang_lib::lookup();
              ppppppuVar30 = local_98;
              toolbar::mei::widgets::tweaker::HintTracker::text();
              toolbar::mei::widgets::tweaker::impl$11::default();
              local_1e8._0_4_ = (uint)local_78;
              local_1e8._4_4_ = (undefined4)((ulonglong)local_78 >> 0x20);
              uStack_1e0._0_4_ = SUB84(pppppppuVar33,0);
              uStack_1e0._4_4_ = (float)((ulonglong)pppppppuVar33 >> 0x20);
              local_1d8 = (uint *******)&local_118;
              toolbar::mei::widgets::tweaker::impl$13::
              tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref_<alloc::string::String>_>_>
                        ();
              pppppppuVar13 = local_118;
              if (extraout_AL_14 == '\0') goto LAB_140249631;
              *in_stack_00000030 = 1;
              if (*(longlong *)((longlong)ppppppuVar30 + 0x10) == 0) goto LAB_140249631;
              if (pppppppuVar33 <= local_118) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              plVar24 = (longlong *)(*(longlong *)((longlong)ppppppuVar30 + 8) + 0x18);
              lVar27 = *(longlong *)((longlong)ppppppuVar30 + 0x10) << 6;
              goto LAB_14024958c;
            }
            pppppppuVar34 = (uint *******)((longlong)pppppppuVar34 + 1);
            lVar27 = lVar27 + -8;
          } while (lVar27 != 0);
          pppppppuVar13 = local_d8;
          ppppppuVar30 = local_98;
          uVar19 = (undefined4)local_88;
        }
        goto LAB_14024964d;
      }
      *(undefined1 *)(lVar5 + 0x7c) = 0;
      uStack_1e0 = CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
    }
    goto LAB_140249d65;
  }
  local_98 = (uint ******)*in_stack_00000048;
  country_core::render::impl$0::deref();
  lVar5 = *in_RCX;
  vram::calculate_resolution_scale_mem_estimate_text();
  local_d8 = (uint *******)*in_stack_00000050;
  if ((char)local_d8[5] == '\0') {
    fVar42 = 256.0;
    fVar43 = 256.0;
  }
  else {
    country_core::render::impl$0::deref();
    vram::calculate_max_resolution_scale();
    fVar42 = extraout_XMM0_Da;
    fVar43 = extraout_XMM0_Da;
    if (1.5 <= extraout_XMM0_Da) {
      fVar43 = 1.5;
    }
  }
  lang_lib::lookup();
  toolbar::mei::widgets::impl$26::label();
  toolbar::mei::widgets::LabelView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  local_b8 = local_1d8;
  toolbar::mei::widgets::LabelView::weight();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (char *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = '\0';
  extraout_RAX[4] = '\0';
  extraout_RAX[5] = '\0';
  extraout_RAX[6] = -0x80;
  extraout_RAX[7] = '?';
  extraout_RAX[8] = '\x01';
  extraout_RAX[9] = '\x01';
  extraout_RAX[10] = '\0';
  extraout_RAX[0xc] = '\x01';
  extraout_RAX[0x10] = '\0';
  extraout_RAX[0x11] = '\0';
  extraout_RAX[0x12] = -0x80;
  extraout_RAX[0x13] = '?';
  extraout_RAX[0x14] = '\x03';
  extraout_RAX[0x15] = '\x01';
  extraout_RAX[0x16] = '\x01';
  extraout_RAX[0x18] = '\x02';
  extraout_RAX[0x1c] = '\0';
  extraout_RAX[0x1d] = '\0';
  extraout_RAX[0x1e] = -0x80;
  extraout_RAX[0x1f] = '?';
  extraout_RAX[0x20] = '\x03';
  extraout_RAX[0x21] = '\0';
  extraout_RAX[0x22] = '\x01';
  extraout_RAX[0x24] = '\x03';
  extraout_RAX[0x28] = '\0';
  extraout_RAX[0x29] = '\0';
  extraout_RAX[0x2a] = -0x40;
  extraout_RAX[0x2b] = '?';
  extraout_RAX[0x2c] = '\x04';
  extraout_RAX[0x2d] = '\x01';
  extraout_RAX[0x2e] = '\x01';
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (char *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = *extraout_RAX;
  extraout_RAX_00[1] = extraout_RAX[0xc];
  extraout_RAX_00[2] = extraout_RAX[0x18];
  extraout_RAX_00[3] = extraout_RAX[0x24];
  cVar15 = *(char *)(lVar5 + 0xab);
  local_e0 = extraout_RAX;
  local_78 = extraout_RAX_00;
  if (*(char *)(lVar5 + 0x88) == '\0') {
    fVar37 = 1.0;
    cVar18 = *(char *)(lVar5 + 0xc3);
    cVar14 = *(char *)(lVar5 + 0xa9);
    if (cVar15 != extraout_RAX[9]) goto LAB_140247e37;
LAB_140247e20:
    if (((cVar18 != extraout_RAX[8]) || (cVar14 != extraout_RAX[10])) ||
       ((fVar37 != *(float *)(extraout_RAX + 4) ||
        (NAN(fVar37) || NAN(*(float *)(extraout_RAX + 4)))))) goto LAB_140247e37;
    local_118 = (uint *******)0x0;
LAB_140247ea5:
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    local_1e8._0_4_ = (uint)local_78;
    local_1e8._4_4_ = (undefined4)((ulonglong)local_78 >> 0x20);
    uStack_1e0._0_4_ = 4;
    uStack_1e0._4_4_ = 0.0;
    local_1d8 = (uint *******)&local_118;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset>_>
              ();
    if ((uint ******)0x3 < local_118) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    cVar15 = local_e0[(longlong)local_118 * 0xc];
    cVar18 = extraout_AL_01;
  }
  else {
    fVar37 = *(float *)(lVar5 + 0x8c);
    cVar18 = *(char *)(lVar5 + 0xc3);
    cVar14 = *(char *)(lVar5 + 0xa9);
    if (cVar15 == extraout_RAX[9]) goto LAB_140247e20;
LAB_140247e37:
    if (((cVar15 == extraout_RAX[0x15]) && (cVar18 == extraout_RAX[0x14])) &&
       (cVar14 == extraout_RAX[0x16])) {
      local_118 = (uint *******)0x1;
      if ((fVar37 == *(float *)(extraout_RAX + 0x10)) &&
         (!NAN(fVar37) && !NAN(*(float *)(extraout_RAX + 0x10)))) goto LAB_140247ea5;
    }
    if ((cVar15 == extraout_RAX[0x21]) &&
       ((cVar18 == extraout_RAX[0x20] && (cVar14 == extraout_RAX[0x22])))) {
      local_118 = (uint *******)0x2;
      if ((fVar37 == *(float *)(extraout_RAX + 0x1c)) &&
         (!NAN(fVar37) && !NAN(*(float *)(extraout_RAX + 0x1c)))) goto LAB_140247ea5;
    }
    if (((cVar15 == extraout_RAX[0x2d]) && (cVar18 == extraout_RAX[0x2c])) &&
       (cVar14 == extraout_RAX[0x2e])) {
      local_118 = (uint *******)0x3;
      if ((fVar37 == *(float *)(extraout_RAX + 0x28)) &&
         (!NAN(fVar37) && !NAN(*(float *)(extraout_RAX + 0x28)))) goto LAB_140247ea5;
    }
    local_118 = (uint *******)0x1;
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    local_1e8._0_4_ = 0x429a0fc0;
    local_1e8._4_4_ = 1;
    uStack_1e0._0_4_ = 3;
    uStack_1e0._4_4_ = 0.0;
    local_1d8 = (uint *******)&local_118;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset>_>
              ();
    cVar15 = (local_118 == (uint *******)0x1) * '\x03' + '\x02';
    cVar18 = extraout_AL_02;
  }
  if ((cVar15 != '\x05') && (cVar18 != '\0')) {
    pcVar25 = local_e0;
    if ((*local_e0 != cVar15) &&
       (((pcVar25 = local_e0 + 0xc, *pcVar25 != cVar15 &&
         (pcVar25 = local_e0 + 0x18, *pcVar25 != cVar15)) &&
        (pcVar25 = local_e0 + 0x24, *pcVar25 != cVar15)))) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (*(float *)(pcVar25 + 4) <= fVar42) {
      *in_stack_00000030 = 1;
      cVar15 = pcVar25[9];
      uVar3 = *(undefined4 *)((longlong)in_RCX + 0x1c);
      puVar6 = (undefined4 *)in_RCX[2];
      *puVar6 = uVar3;
      *(char *)(lVar5 + 0xab) = cVar15;
      cVar15 = pcVar25[8];
      *puVar6 = uVar3;
      *(char *)(lVar5 + 0xc3) = cVar15;
      cVar15 = pcVar25[10];
      *puVar6 = uVar3;
      *(char *)(lVar5 + 0xa9) = cVar15;
      uVar19 = *(undefined4 *)(pcVar25 + 4);
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      *(undefined4 *)((longlong)local_98 + 0x18) = uVar19;
      fVar37 = *(float *)(pcVar25 + 4);
      if ((fVar37 != 1.0) || (NAN(fVar37))) {
        uVar19 = 1;
        if ((*(uint *)(lVar5 + 0x88) & 1) != 0) {
          if ((fVar37 == *(float *)(lVar5 + 0x8c)) &&
             (!NAN(fVar37) && !NAN(*(float *)(lVar5 + 0x8c)))) goto LAB_1402483c4;
        }
      }
      else {
        uVar19 = 0;
        if ((*(uint *)(lVar5 + 0x88) & 1) == 0) goto LAB_1402483c4;
      }
      *puVar6 = uVar3;
      *(undefined4 *)(lVar5 + 0x88) = uVar19;
      *(float *)(lVar5 + 0x8c) = fVar37;
      *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      *local_d8 = (uint ******)0x0;
    }
  }
LAB_1402483c4:
  pcVar25 = (char *)0x1;
  std::alloc::__default_lib_allocator::__rust_dealloc();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  uVar20 = 0x21;
  lang_lib::lookup();
  bVar36 = *(char *)(lVar5 + 0xab) != '\0';
  if (bVar36) {
    pcVar25 = "Tiny Glade Super Resolution\n\n";
  }
  local_2d0 = 0x1d;
  if (!bVar36) {
    local_2d0 = 0;
  }
  local_c8 = (uint *******)&local_2d8;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)&local_2c8;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1e8._0_4_ = 0x429a1020;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 2;
  local_1c8 = 0;
  local_2d8 = (uint *****)pcVar25;
  local_2c0 = uVar20;
  alloc::fmt::format::format_inner();
  local_200 = (longlong *)local_2f0;
  local_258 = (uint *****)&DAT_1429a1040;
  local_250 = 4;
  uVar20 = 0x1d;
  lang_lib::lookup();
  local_c8 = (uint *******)&local_258;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)&local_118;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1e8._0_4_ = 0x429a1020;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 2;
  local_1c8 = 0;
  pppppppuStack_110 = (uint *******)uVar20;
  alloc::fmt::format::format_inner();
  local_d0 = (uint ******)local_308;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  puVar6 = (undefined4 *)in_RCX[2];
  local_78 = (char *)CONCAT44(local_78._4_4_,*(undefined4 *)((longlong)in_RCX + 0x1c));
  *puVar6 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode>_>
            ();
  bVar32 = extraout_AL_05 | *in_stack_00000030;
  *in_stack_00000030 = bVar32;
  if (local_d0 != (uint ******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (*(char *)(lVar5 + 0xab) == '\x01') {
    uVar20 = 0x23;
    lang_lib::lookup();
    local_c8 = (uint *******)&local_258;
    pppppppuStack_c0 =
         (uint *******)
         anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_b8 = (uint *******)&local_118;
    local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_1e8._0_4_ = 0x429a1020;
    local_1e8._4_4_ = 1;
    uStack_1e0._0_4_ = 2;
    uStack_1e0._4_4_ = 0.0;
    local_1d8 = (uint *******)&local_c8;
    uStack_1d0 = 2;
    local_1c8 = 0;
    pppppppuStack_110 = (uint *******)uVar20;
    alloc::fmt::format::format_inner();
    local_d0 = (uint ******)local_320;
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *puVar6 = (undefined4)local_78;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset>_>
              ();
    bVar32 = bVar32 | extraout_AL_06;
    *in_stack_00000030 = bVar32;
    if (local_d0 != (uint ******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  uVar20 = 0x17;
  lang_lib::lookup();
  local_c8 = (uint *******)&local_258;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)&local_118;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1e8._0_4_ = 0x429a1020;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 2;
  local_1c8 = 0;
  pppppppuStack_110 = (uint *******)uVar20;
  alloc::fmt::format::format_inner();
  local_d0 = (uint ******)local_338;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar6 = (undefined4)local_78;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar32 = bVar32 | extraout_AL_07;
  *in_stack_00000030 = bVar32;
  if (local_d0 != (uint ******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (*(char *)(lVar5 + 0x88) == '\0') {
    local_1ec = 1.0;
  }
  else {
    local_1ec = *(float *)(lVar5 + 0x8c);
  }
  uVar20 = 0x23;
  lang_lib::lookup();
  local_c8 = (uint *******)&local_118;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)local_2a0;
  local_b0 = alloc::string::impl$23::fmt;
  local_1e8._0_4_ = 0x429a10c0;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 2;
  local_1c8 = 0;
  pppppppuStack_110 = (uint *******)uVar20;
  alloc::fmt::format::format_inner();
  local_228 = (uint *****)CONCAT71(uStack_23f,local_240);
  uStack_220 = uStack_238;
  local_218 = local_230;
  if (*(char *)(lVar5 + 0x88) == '\0') {
    fVar37 = 1.0;
  }
  else {
    fVar37 = *(float *)(lVar5 + 0x8c);
  }
  local_248 = (uint *)((longlong)local_98 + 0x18);
  if (*(float *)((longlong)local_98 + 0x18) <= fVar37 &&
      fVar37 != *(float *)((longlong)local_98 + 0x18)) {
                    // WARNING: Could not recover jumptable at 0x0001402489b3. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429a1744 + *(int *)(&DAT_1429a1744 + (longlong)*local_d8 * 4)))();
    return;
  }
  local_108 = (uint *******)local_230;
  local_118 = (uint *******)CONCAT71(uStack_23f,local_240);
  pppppppuStack_110 = (uint *******)uStack_238;
  local_11c = 0;
  local_c8 = (uint *******)&local_118;
  pppppppuStack_c0 = (uint *******)alloc::string::impl$23::fmt;
  local_1e8._0_4_ = 0x429a11a0;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 1;
  local_1c8 = 0;
  alloc::fmt::format::format_inner();
  local_d0 = local_288;
  local_88 = uStack_280;
  uVar20 = 0x1f;
  lang_lib::lookup();
  local_c8 = (uint *******)&local_258;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)&local_268;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1e8._0_4_ = 0x429a1020;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 2;
  uStack_1e0._4_4_ = 0.0;
  local_1d8 = (uint *******)&local_c8;
  uStack_1d0 = 2;
  local_1c8 = 0;
  local_260 = uVar20;
  alloc::fmt::format::format_inner();
  local_208 = (uint ******)local_138;
  local_98 = (uint ******)local_130;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  uStack_1e0._4_4_ = fVar42;
  if (0.1 <= fVar42) {
    uStack_1e0._4_4_ = 0.1;
  }
  local_1e8 = (uint *******)&local_1ec;
  local_1d8._0_5_ = (uint5)(uint)fVar43;
  uStack_1e0._0_4_ = 0x3dcccccd;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  if (extraout_AL_08 == '\0') {
    if (local_208 != (uint ******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    if (local_208 != (uint ******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *puVar6 = (undefined4)local_78;
    *(undefined4 *)(lVar5 + 0x88) = 1;
    *(float *)(lVar5 + 0x8c) = local_1ec;
    fVar43 = fVar42;
    if (local_1ec <= fVar42) {
      fVar43 = local_1ec;
    }
    *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
    *local_248 = ~-(uint)NAN(local_1ec) & (uint)fVar43 | -(uint)NAN(local_1ec) & (uint)fVar42;
    *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
    *local_d8 = (uint ******)0x0;
    *in_stack_00000030 = 1;
    bVar32 = 1;
  }
  if (local_d0 != (uint ******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_118 != (uint *******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (((char)local_11c != '\0') && (local_228 != (uint *****)0x0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  toolbar::mei::widgets::impl$26::space();
  uVar20 = 0x17;
  lang_lib::lookup();
  uVar21 = 9;
  local_260 = uVar20;
  lang_lib::lookup();
  local_c8 = (uint *******)&local_268;
  pppppppuStack_c0 =
       (uint *******)anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
  ;
  local_b8 = (uint *******)&local_228;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1e8._0_4_ = 0x429a11f0;
  local_1e8._4_4_ = 1;
  uStack_1e0._0_4_ = 3;
  uStack_1e0._4_4_ = 0.0;
  uStack_1d0 = 2;
  local_1c8 = 0;
  uStack_220 = uVar21;
  local_1d8 = (uint *******)&local_c8;
  alloc::fmt::format::format_inner();
  local_98 = (uint ******)local_118;
  local_88 = pppppppuStack_110;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar6 = (undefined4)local_78;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  *in_stack_00000030 = bVar32 | extraout_AL_09;
  if (local_98 != (uint ******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar6 = (undefined4)local_78;
  local_1e8 = (uint *******)(lVar5 + 0xa0);
  local_1d8._0_5_ = 0x40000000;
  uStack_1e0._0_4_ = 0x3e4ccccd;
  uStack_1e0._4_4_ = -2.0;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar32 = bVar32 | extraout_AL_09 | extraout_AL_10;
  *in_stack_00000030 = bVar32;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar6 = (undefined4)local_78;
  local_1e8 = (uint *******)(lVar5 + 0xa4);
  local_1d8._0_5_ = 0x3f000000;
  uStack_1e0._0_4_ = 0x3d4ccccd;
  uStack_1e0._4_4_ = -0.5;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar32 = bVar32 | extraout_AL_11;
  *in_stack_00000030 = bVar32;
  lang_lib::lookup();
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar6 = (undefined4)local_78;
  local_1e8 = (uint *******)(lVar5 + 0x9c);
  local_1d8 = (uint *******)CONCAT35(local_1d8._5_3_,0x3f800000);
  uStack_1e0._0_4_ = 0x3dcccccd;
  uStack_1e0._4_4_ = 0.0;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  *in_stack_00000030 = bVar32 | extraout_AL_12;
  if (local_200 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_2a0[0] == (uint *****)0x0) {
    return;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
code_r0x000140247c2c:
  ppppppuVar30 = (uint ******)((longlong)ppppppuVar30 + 1);
  lVar27 = lVar27 + 0xa0;
  if ((longlong)pppppppuVar33 * 0xa0 == lVar27) goto LAB_140249304;
  goto LAB_140247c10;
LAB_14024958c:
  do {
    if (!SBORROW8(0,*plVar24)) {
      lVar26 = *(longlong *)((longlong)local_78 + (longlong)pppppppuVar13 * 8);
      if ((plVar24[2] == *(size_t *)(lVar26 + 0x10)) &&
         (iVar11 = memcmp((void *)plVar24[1],*(void **)(lVar26 + 8),plVar24[2]), iVar11 == 0)) {
        alloc::string::impl$6::clone();
        plVar1 = local_200;
        lVar26 = plVar24[3];
        lVar9 = plVar24[4];
        local_b8 = local_1d8;
        local_c8 = (uint *******)CONCAT44(local_1e8._4_4_,(uint)local_1e8);
        pppppppuStack_c0 = (uint *******)CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
        lVar27 = *local_200;
        if ((lVar27 != -0x7fffffffffffffff) && ((!SBORROW8(0,lVar27) && (lVar27 != 0)))) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar1[2] = (longlong)local_b8;
        *plVar1 = (longlong)local_c8;
        plVar1[1] = (longlong)pppppppuStack_c0;
        *(longlong *)(lVar5 + 0x50) = lVar26;
        *(longlong *)(lVar5 + 0x58) = lVar9;
        break;
      }
    }
    plVar24 = plVar24 + 8;
    lVar27 = lVar27 + -0x40;
  } while (lVar27 != 0);
LAB_140249631:
  pppppppuVar13 = local_d8;
  uVar19 = (undefined4)local_88;
LAB_14024964d:
  if (pppppppuVar13 != (uint *******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uStack_1e0 = CONCAT44(uStack_1e0._4_4_,(undefined4)uStack_1e0);
  if (local_240 == '\x02') {
    lVar27 = *(longlong *)((longlong)ppppppuVar30 + 0x10);
    if (lVar27 != 0) {
      lVar26 = *(longlong *)((longlong)ppppppuVar30 + 8);
      if (*(longlong *)(lVar5 + 0x38) == -0x7fffffffffffffff) {
        lVar27 = lVar27 << 6;
        do {
          if (SBORROW8(0,*(longlong *)(lVar26 + 0x18))) goto LAB_1402496e2;
          lVar26 = lVar26 + 0x40;
          lVar27 = lVar27 + -0x40;
        } while (lVar27 != 0);
      }
      else if (SBORROW8(0,*(longlong *)(lVar5 + 0x38))) {
        lVar27 = lVar27 << 6;
        do {
          if (SBORROW8(0,*(longlong *)(lVar26 + 0x18))) goto LAB_1402496e2;
          lVar26 = lVar26 + 0x40;
          lVar27 = lVar27 + -0x40;
        } while (lVar27 != 0);
      }
      else {
        pvVar7 = *(void **)(lVar5 + 0x40);
        sVar8 = *(size_t *)(lVar5 + 0x48);
        lVar27 = lVar27 << 6;
        do {
          if (((!SBORROW8(0,*(longlong *)(lVar26 + 0x18))) && (*(size_t *)(lVar26 + 0x28) == sVar8))
             && (iVar11 = memcmp(*(void **)(lVar26 + 0x20),pvVar7,sVar8), iVar11 == 0))
          goto LAB_1402496e2;
          lVar26 = lVar26 + 0x40;
          lVar27 = lVar27 + -0x40;
        } while (lVar27 != 0);
      }
    }
    lVar26 = 0;
LAB_1402496e2:
    local_1e8._0_4_ = 1;
    local_1e8._4_4_ = 0;
    uStack_1e0._0_4_ = (undefined4)lVar26;
    uStack_1e0._4_4_ = (float)((ulonglong)lVar26 >> 0x20);
    local_1d8 = (uint *******)0x0;
    local_1c8 = 0;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::flatten::FlatMap<core::option::IntoIter<ref_<country_core::resources::window::AvailableMonitor>_>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::render::render_config::VideoMode>_>,tiny_glade::gui::settings::ui_settings_video_display::closure_env_6>_>
              ();
    pppppppuVar33 = local_128;
    pcVar10 = local_130;
    if (local_128 != (uint *******)0x0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (pppppppuVar33 < (uint *******)0x4) {
        uVar12 = 0;
      }
      else {
        puVar23 = (undefined8 *)((longlong)pcVar10 + 0x30);
        uVar12 = 0;
        do {
          *(undefined8 *)(extraout_RAX_02 + uVar12 * 8) = puVar23[-6];
          *(undefined8 *)(extraout_RAX_02 + 8 + uVar12 * 8) = puVar23[-4];
          *(undefined8 *)(extraout_RAX_02 + 0x10 + uVar12 * 8) = puVar23[-2];
          *(undefined8 *)(extraout_RAX_02 + 0x18 + uVar12 * 8) = *puVar23;
          uVar12 = uVar12 + 4;
          puVar23 = puVar23 + 8;
        } while (((ulonglong)pppppppuVar33 & 0xfffffffffffffffc) != uVar12);
      }
      if (((ulonglong)pppppppuVar33 & 3) != 0) {
        uVar28 = 0;
        do {
          *(undefined8 *)(uVar12 * 8 + extraout_RAX_02 + uVar28) =
               *(undefined8 *)(uVar12 * 0x10 + (longlong)pcVar10 + uVar28 * 2);
          uVar28 = uVar28 + 8;
        } while (((uint)pppppppuVar33 & 3) << 3 != uVar28);
      }
      pppppppuVar34 = (uint *******)0x1;
      pppppppuVar13 = (uint *******)0x1;
      pppppppuVar35 = pppppppuVar34;
      if (pppppppuVar33 != (uint *******)0x1) {
        do {
          pppppppuVar13 = pppppppuVar35;
          if (*(longlong *)(extraout_RAX_02 + (longlong)pppppppuVar35 * 8) ==
              *(longlong *)(extraout_RAX_02 + -8 + (longlong)pppppppuVar35 * 8)) goto LAB_140249893;
          pppppppuVar35 = (uint *******)((longlong)pppppppuVar35 + 1);
          pppppppuVar34 = pppppppuVar33;
        } while (pppppppuVar33 != pppppppuVar35);
        goto LAB_140249817;
      }
      goto LAB_140249966;
    }
    goto LAB_140249d3d;
  }
LAB_140249d65:
  lang_lib::lookup();
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *(undefined4 *)local_e0 = uVar19;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  *in_stack_00000030 = *in_stack_00000030 | extraout_AL_17;
  local_1e8._0_4_ = 0;
  local_1e8._4_4_ = 0xf;
  uStack_1e0._0_4_ = 0;
  uStack_1e0._4_4_ = 2.8026e-44;
  local_1d8 = (uint *******)0x1e00000000;
  uStack_1d0 = 0x3c00000000;
  local_1c8 = 0x4800000000;
  uStack_1c0 = 0x5000000000;
  local_1b8 = 0x5a00000000;
  uStack_1b0 = 0x7800000000;
  local_1a8 = 1;
  fVar42 = *(float *)(lVar5 + 0x98);
  local_138 = (uint *******)0x0;
  iVar11 = (int)(longlong)fVar42;
  if (fVar42 < 0.0) {
    iVar11 = 0;
  }
  iVar16 = -1;
  if (fVar42 <= 4.294967e+09) {
    iVar16 = iVar11;
  }
  if (0x3b < iVar16) {
    switch(iVar16) {
    case 0x3c:
      local_138 = (uint *******)0x3;
      break;
    case 0x48:
      local_138 = (uint *******)0x4;
      break;
    case 0x50:
      local_138 = (uint *******)0x5;
      break;
    case 0x5a:
      local_138 = (uint *******)0x6;
      break;
    default:
      if (iVar16 == 0x78) {
        local_138 = (uint *******)0x7;
        break;
      }
    case 0x3d:
    case 0x3e:
    case 0x3f:
    case 0x40:
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
LAB_140249ea5:
      local_138 = (uint *******)0x8;
    }
LAB_140249ec6:
    lang_lib::lookup();
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    pppppppuStack_c0 = (uint *******)0x9;
    local_b8 = (uint *******)&local_138;
    local_c8 = (uint *******)&local_1e8;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2_<tiny_glade::gui::settings::FpsCap>_>_>
              ();
    if (extraout_AL_18 != '\0') {
      *in_stack_00000030 = 1;
      if ((uint ******)0x8 < local_138) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar42 = 0.0;
      if ((*(byte *)(&local_1e8 + (longlong)local_138) & 1) == 0) {
        fVar42 = (float)*(uint *)((longlong)&local_1e8 + (longlong)local_138 * 8 + 4);
      }
      *(undefined4 *)local_e0 = uVar19;
      *(float *)(lVar5 + 0x98) = fVar42;
    }
    return;
  }
  if (iVar16 == 0xf) goto LAB_140249ec6;
  if (iVar16 == 0x14) {
    local_138 = (uint *******)0x1;
    goto LAB_140249ec6;
  }
  if (iVar16 == 0x1e) {
    local_138 = (uint *******)0x2;
    goto LAB_140249ec6;
  }
  goto LAB_140249ea5;
LAB_140249893:
  while (pppppppuVar13 = (uint *******)((longlong)pppppppuVar13 + 1), pppppppuVar34 = pppppppuVar35,
        pppppppuVar13 < pppppppuVar33) {
    lVar27 = *(longlong *)(extraout_RAX_02 + (longlong)pppppppuVar13 * 8);
    if (lVar27 != *(longlong *)(extraout_RAX_02 + -8 + (longlong)pppppppuVar35 * 8)) {
      *(longlong *)(extraout_RAX_02 + (longlong)pppppppuVar35 * 8) = lVar27;
      pppppppuVar35 = (uint *******)((longlong)pppppppuVar35 + 1);
    }
  }
LAB_140249817:
  if ((uint *******)0x1 < pppppppuVar34) {
    uVar12 = (ulonglong)pppppppuVar34 >> 1;
    if (&DAT_00000007 < pppppppuVar34) {
      uVar28 = uVar12 & 0xfffffffffffffffc;
      pauVar22 = (undefined1 (*) [16])((longlong)pppppppuVar34 * 8 + -0x10 + extraout_RAX_02);
      uVar29 = 0;
      do {
        auVar39 = *(undefined1 (*) [16])(extraout_RAX_02 + uVar29 * 8);
        auVar2 = *(undefined1 (*) [16])(extraout_RAX_02 + 0x10 + uVar29 * 8);
        auVar38 = pauVar22[-1];
        auVar40 = *pauVar22;
        auVar41._0_8_ = auVar40._8_8_;
        auVar41._8_4_ = auVar40._0_4_;
        auVar41._12_4_ = auVar40._4_4_;
        auVar40._0_8_ = auVar38._8_8_;
        auVar40._8_4_ = auVar38._0_4_;
        auVar40._12_4_ = auVar38._4_4_;
        *(undefined1 (*) [16])(extraout_RAX_02 + uVar29 * 8) = auVar41;
        *(undefined1 (*) [16])(extraout_RAX_02 + 0x10 + uVar29 * 8) = auVar40;
        auVar38._0_8_ = auVar39._8_8_;
        auVar38._8_4_ = auVar39._0_4_;
        auVar38._12_4_ = auVar39._4_4_;
        *pauVar22 = auVar38;
        auVar39._0_8_ = auVar2._8_8_;
        auVar39._8_4_ = auVar2._0_4_;
        auVar39._12_4_ = auVar2._4_4_;
        pauVar22[-1] = auVar39;
        uVar29 = uVar29 + 4;
        pauVar22 = pauVar22 + -2;
      } while (uVar28 != uVar29);
      if (uVar12 == uVar28) goto LAB_14024995a;
    }
    else {
      uVar28 = 0;
    }
    puVar23 = (undefined8 *)(uVar28 * 8 + extraout_RAX_02);
    lVar27 = -uVar28;
    lVar26 = (longlong)pppppppuVar34 * 8 + -8 + extraout_RAX_02;
    do {
      uVar20 = *puVar23;
      *puVar23 = *(undefined8 *)(lVar26 + lVar27 * 8);
      *(undefined8 *)(lVar26 + lVar27 * 8) = uVar20;
      lVar27 = lVar27 + -1;
      puVar23 = puVar23 + 1;
    } while (-lVar27 != uVar12);
  }
LAB_14024995a:
  pppppppuVar13 = pppppppuVar33;
  if (pppppppuVar34 != (uint *******)0x0) {
LAB_140249966:
    local_98 = (uint ******)(lVar5 + 0x68);
    local_d8 = pppppppuVar13;
    local_78 = (char *)extraout_RAX_02;
    country_core::render::render_config::FullscreenConfig::resolution();
    if (((uint)local_1e8 & 1) != 0) {
      lVar27 = (longlong)pppppppuVar34 * 8;
      local_118 = (uint *******)0x0;
      do {
        if (*(longlong *)((longlong)local_78 + (longlong)local_118 * 8) ==
            CONCAT44((undefined4)uStack_1e0,local_1e8._4_4_)) {
          lang_lib::lookup();
          toolbar::mei::widgets::tweaker::HintTracker::text();
          toolbar::mei::widgets::tweaker::impl$11::default();
          local_1e8._0_4_ = (uint)local_78;
          local_1e8._4_4_ = (undefined4)((ulonglong)local_78 >> 0x20);
          uStack_1e0._0_4_ = SUB84(pppppppuVar34,0);
          uStack_1e0._4_4_ = (float)((ulonglong)pppppppuVar34 >> 0x20);
          local_1d8 = (uint *******)&local_118;
          toolbar::mei::widgets::tweaker::impl$13::
          tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>_>
                    ();
          if (extraout_AL_15 != '\0') goto joined_r0x000140249ae1;
          goto LAB_140249afb;
        }
        local_118 = (uint *******)((longlong)local_118 + 1);
        lVar27 = lVar27 + -8;
      } while (lVar27 != 0);
    }
    local_118 = (uint *******)((longlong)pppppppuVar34 + -1);
joined_r0x000140249ae1:
    if (pppppppuVar34 <= local_118) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    lVar27 = 0;
    do {
      if ((longlong)pppppppuVar33 * 0x10 == lVar27) goto LAB_140249afb;
      lVar26 = lVar27 + 0x10;
      plVar24 = (longlong *)((longlong)pcVar10 + lVar27);
      lVar27 = lVar26;
    } while (*plVar24 != *(longlong *)((longlong)local_78 + (longlong)local_118 * 8));
    puVar23 = (undefined8 *)((longlong)pcVar10 + -0x10 + lVar26);
    uVar20 = *puVar23;
    uVar21 = puVar23[1];
    local_1e8._0_4_ = (uint)uVar20;
    local_1e8._4_4_ = (undefined4)((ulonglong)uVar20 >> 0x20);
    uStack_1e0._0_4_ = (undefined4)uVar21;
    uStack_1e0._4_4_ = (float)((ulonglong)uVar21 >> 0x20);
    *(undefined4 *)(lVar5 + 0x68) = 1;
    *(undefined8 *)(lVar5 + 0x6c) = uVar20;
    *(undefined8 *)(lVar5 + 0x74) = uVar21;
    *in_stack_00000030 = 1;
LAB_140249afb:
    if (((ulonglong)*local_98 & 1) != 0) {
      uStack_1e0 = (longlong)pppppppuVar33 * 0x10 + (longlong)pcVar10;
      local_288 = *(uint *******)(lVar5 + 0x6c);
      uStack_280 = *(undefined8 *)(lVar5 + 0x74);
      local_1e8._0_4_ = (uint)pcVar10;
      local_1e8._4_4_ = (undefined4)((ulonglong)pcVar10 >> 0x20);
      local_1d8 = &local_288;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<tiny_glade::gui::settings::RefreshRateMillihertz,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::render::render_config::VideoMode>,tiny_glade::gui::settings::ui_settings_video_display::closure_env_11>_>
                ();
      pppppppuVar13 = local_108;
      pppppppuVar33 = pppppppuStack_110;
      if (local_108 < (uint *******)0x2) {
LAB_140249b66:
        pppppppuVar13 = local_108;
        if (local_108 != (uint *******)0x0) goto LAB_140249b6b;
        pppppppuVar13 = (uint *******)0x0;
      }
      else {
        if (&DAT_00000014 < local_108) {
          local_1f8 = pppppppuStack_110;
          core::slice::sort::stable::
          driftsort_main<tiny_glade::gui::settings::RefreshRateMillihertz,bool_(*)(ref_<tiny_glade::gui::settings::RefreshRateMillihertz>,ref_<tiny_glade::gui::settings::RefreshRateMillihertz>),alloc::vec::Vec<tiny_glade::gui::settings::RefreshRateMillihertz,alloc::alloc::Global>_>
                    ();
        }
        else {
          puVar17 = (uint *)((longlong)pppppppuStack_110 + 4);
          lVar27 = 4;
          do {
            uVar31 = puVar17[-1];
            uVar4 = *puVar17;
            lVar26 = lVar27;
            if (uVar4 < uVar31) {
              do {
                *(uint *)((longlong)pppppppuStack_110 + lVar26) = uVar31;
                pppppppuVar34 = pppppppuStack_110;
                if (lVar26 == 4) goto LAB_14024a033;
                uVar31 = *(uint *)((longlong)pppppppuStack_110 + lVar26 + -8);
                lVar26 = lVar26 + -4;
              } while (uVar4 < uVar31);
              pppppppuVar34 = (uint *******)(lVar26 + (longlong)pppppppuStack_110);
LAB_14024a033:
              *(uint *)pppppppuVar34 = uVar4;
            }
            puVar17 = puVar17 + 1;
            lVar27 = lVar27 + 4;
          } while (puVar17 != (uint *)((longlong)pppppppuStack_110 + (longlong)local_108 * 4));
          if (local_108 < (uint *******)0x2) goto LAB_140249b66;
        }
        pppppppuVar34 = (uint *******)0x1;
        do {
          pppppppuVar35 = pppppppuVar34;
          if (*(uint *)((longlong)pppppppuVar33 + (longlong)pppppppuVar34 * 4) ==
              *(uint *)((longlong)pppppppuVar33 + (longlong)((longlong)pppppppuVar34 + -1) * 4))
          goto LAB_14024a0ad;
          pppppppuVar34 = (uint *******)((longlong)pppppppuVar34 + 1);
        } while (pppppppuVar13 != pppppppuVar34);
LAB_140249b6b:
        local_2b8[0] = (uint *******)0x0;
        pppppppuVar34 = pppppppuVar33;
        do {
          if ((uint)uStack_280 == *(uint *)pppppppuVar34) {
            local_1f8 = pppppppuVar33;
            lang_lib::lookup();
            local_1f8 = pppppppuVar33;
            toolbar::mei::widgets::tweaker::HintTracker::text();
            local_1f8 = pppppppuVar33;
            toolbar::mei::widgets::tweaker::impl$11::default();
            local_1e8._0_4_ = (uint)pppppppuVar33;
            local_1e8._4_4_ = (undefined4)((ulonglong)pppppppuVar33 >> 0x20);
            uStack_1e0._0_4_ = SUB84(pppppppuVar13,0);
            uStack_1e0._4_4_ = (float)((ulonglong)pppppppuVar13 >> 0x20);
            local_1d8 = (uint *******)local_2b8;
            local_1f8 = pppppppuVar33;
            toolbar::mei::widgets::tweaker::impl$13::
            tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>_>
                      ();
            if (extraout_AL_16 == '\0') goto LAB_140249d06;
            if (pppppppuVar13 <= local_2b8[0]) goto LAB_140249ce4;
            goto LAB_140249bb3;
          }
          pppppppuVar34 = (uint *******)((longlong)pppppppuVar34 + 4);
          local_2b8[0] = (uint *******)((longlong)local_2b8[0] + 1);
        } while (pppppppuVar34 !=
                 (uint *******)((longlong)pppppppuVar33 + (longlong)pppppppuVar13 * 4));
      }
      local_2b8[0] = (uint *******)((longlong)pppppppuVar13 + -1);
      if (pppppppuVar13 <= local_2b8[0]) {
LAB_140249ce4:
        local_1f8 = pppppppuVar33;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
LAB_140249bb3:
      if (local_128 != (uint *******)0x0) {
        lVar27 = 0;
        do {
          if ((*(uint *******)((longlong)local_130 + lVar27) == local_288) &&
             (*(uint *)((longlong)local_130 + 8 + lVar27) ==
              *(uint *)((longlong)pppppppuVar33 + (longlong)local_2b8[0] * 4))) {
            uVar20 = *(undefined8 *)((longlong)local_130 + lVar27);
            uStack_1e0 = ((undefined8 *)((longlong)local_130 + lVar27))[1];
            local_1e8._0_4_ = (uint)uVar20;
            local_1e8._4_4_ = (undefined4)((ulonglong)uVar20 >> 0x20);
            *(undefined4 *)local_98 = 1;
            *(undefined8 *)(lVar5 + 0x6c) = uVar20;
            *(longlong *)(lVar5 + 0x74) = uStack_1e0;
            *in_stack_00000030 = 1;
            break;
          }
          lVar27 = lVar27 + 0x10;
        } while ((longlong)local_128 << 4 != lVar27);
      }
LAB_140249d06:
      if (local_118 != (uint *******)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
LAB_140249d3d:
  if (local_138 != (uint *******)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar19 = (undefined4)local_88;
  goto LAB_140249d65;
LAB_14024a0ad:
  while (pppppppuVar35 = (uint *******)((longlong)pppppppuVar35 + 1), local_108 = pppppppuVar34,
        pppppppuVar35 < pppppppuVar13) {
    uVar4 = *(uint *)((longlong)pppppppuVar33 + (longlong)pppppppuVar35 * 4);
    if (uVar4 != *(uint *)((longlong)pppppppuVar33 + (longlong)((longlong)pppppppuVar34 + -1) * 4))
    {
      *(uint *)((longlong)pppppppuVar33 + (longlong)pppppppuVar34 * 4) = uVar4;
      pppppppuVar34 = (uint *******)((longlong)pppppppuVar34 + 1);
    }
  }
  goto LAB_140249b66;
}

