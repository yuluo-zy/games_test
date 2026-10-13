// signature: void __cdecl create_ui_widget(void)

void __cdecl system_roof::ui_edit_roof_ridge_length::create_ui_widget(void)

{
  float fVar1;
  undefined8 uVar2;
  char cVar3;
  undefined4 uVar4;
  ulonglong uVar5;
  char extraout_AL;
  undefined1 extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong lVar6;
  undefined8 *extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *in_RCX;
  longlong lVar7;
  undefined8 *in_RDX;
  undefined8 *puVar8;
  undefined8 uVar9;
  undefined8 *in_R9;
  longlong lVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  float fVar13;
  undefined4 uVar14;
  ulonglong *in_stack_00000028;
  byte in_stack_00000030;
  undefined8 *local_3d8;
  code *pcStack_3d0;
  longlong *local_3c8;
  code *local_3c0;
  byte local_324;
  undefined8 local_320;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined8 local_310;
  undefined4 local_308;
  undefined8 local_300;
  undefined8 local_2f0;
  undefined8 uStack_2e8;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 local_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  longlong local_2b0;
  undefined8 local_2a8;
  ulonglong local_2a0;
  float local_298;
  undefined8 local_294;
  undefined4 local_28c;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  longlong local_270;
  undefined **local_268;
  undefined8 *puStack_260;
  undefined8 **local_258;
  float local_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  longlong local_158;
  undefined8 *local_150;
  undefined8 *local_148;
  undefined8 local_13c;
  undefined4 local_134;
  undefined8 *local_130;
  undefined8 local_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  ulonglong local_100;
  undefined8 *puStack_f8;
  ulonglong local_f0;
  byte local_e4;
  undefined1 local_e3;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
  local_268 = (undefined **)extraout_RAX;
  country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space();
  country_core::resources::roofs::roof_shape::Roof::ty();
  if (extraout_AL == '\0') {
    return;
  }
  if (*(int *)(in_RCX + 1) != 1) {
    return;
  }
  local_168 = *(undefined8 *)((longlong)in_RCX + 0x1c);
  local_178 = *(undefined8 *)((longlong)in_RCX + 0xc);
  uStack_170 = *(undefined8 *)((longlong)in_RCX + 0x14);
  if ((local_324 & 1) == 0) {
    uStack_11c = (undefined4)local_320;
    uStack_118 = (undefined4)((ulonglong)local_320 >> 0x20);
    cVar3 = *(char *)((longlong)in_RCX + 0x3c);
    uStack_114 = uStack_318;
  }
  else {
    uStack_118 = (undefined4)local_310;
    uStack_114 = (undefined4)((ulonglong)local_310 >> 0x20);
    cVar3 = *(char *)((longlong)in_RCX + 0x3c);
    uStack_11c = uStack_314;
  }
  if (cVar3 == '\0') {
    uStack_120 = uStack_318;
    local_128 = local_320;
    ::utils::swizzlers::impl$1::xvy();
    uVar11 = SUB84(local_268,0);
    uVar12 = (undefined4)((ulonglong)local_268 >> 0x20);
  }
  else {
    uStack_120 = uStack_318;
    local_128 = local_320;
    uVar14 = (undefined4)local_178;
    local_178._4_4_ = (undefined4)((ulonglong)local_178 >> 0x20);
    uVar4 = local_178._4_4_;
    local_3d8 = (undefined8 *)(CONCAT44((undefined4)local_178,local_178._4_4_) ^ 0x80000000);
    ::utils::swizzlers::impl$1::xvy();
    uVar11 = SUB84(local_268,0);
    uVar12 = (undefined4)((ulonglong)local_268 >> 0x20);
    local_268 = (undefined **)(CONCAT44(uVar14,uVar4) ^ 0x80000000);
  }
  fVar13 = -puStack_260._0_4_;
  ::utils::swizzlers::impl$1::xvy();
  local_150 = in_RDX;
  local_148 = in_R9;
  local_130 = in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_100 = 2;
  local_f0 = 0;
  local_2b8 = CONCAT44(uStack_114,uStack_118);
  local_2c8 = local_128;
  uStack_2c0 = CONCAT44(uStack_11c,uStack_120);
  local_294 = local_13c;
  local_28c = local_134;
  local_288 = 0;
  uStack_280 = 0;
  local_278 = 0;
  local_2d8 = 0;
  local_2d0 = 2;
  local_2a8 = 2;
  local_2a0 = CONCAT44(uVar12,uVar11) ^ 0x8000000080000000;
  local_298 = fVar13;
  local_e4 = in_stack_00000030 ^ 1;
  lVar10 = 0;
  lVar6 = 0;
  lVar7 = 0;
  puStack_f8 = extraout_RAX_00;
  do {
    if ((lVar10 == 2) || (lVar7 == 2)) {
      if (in_stack_00000030 == 0) {
        if (local_f0 == 0) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar9 = *puStack_f8;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        puStack_260 = (undefined8 *)uVar9;
        country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
        if (extraout_AL_03 == '\0') {
          if (local_f0 < 2) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar9 = puStack_f8[3];
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          puStack_260 = (undefined8 *)uVar9;
          country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
          if (extraout_AL_04 == '\0') {
            if (local_100 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
              return;
            }
            return;
          }
        }
      }
      uVar9 = *local_130;
      *local_148 = 1;
      local_148[1] = uVar9;
      local_258 = (undefined8 **)local_f0;
      local_268 = (undefined **)local_100;
      puStack_260 = puStack_f8;
      if (*in_stack_00000028 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      in_stack_00000028[2] = (ulonglong)local_258;
      *in_stack_00000028 = (ulonglong)local_268;
      in_stack_00000028[1] = (ulonglong)puStack_260;
      local_150[2] = CONCAT44(uStack_114,uStack_118);
      *local_150 = local_128;
      local_150[1] = CONCAT44(uStack_11c,uStack_120);
      return;
    }
    uVar9 = *(undefined8 *)((longlong)&local_2c8 + lVar10 * 0xc);
    fVar13 = *(float *)((longlong)&uStack_2c0 + lVar10 * 0xc);
    uVar2 = *(undefined8 *)((longlong)&local_2a0 + lVar7 * 0xc);
    fVar1 = (&local_298)[lVar7 * 3];
    local_2b0 = lVar7 + 1;
    local_270 = lVar6 + 1;
    local_158 = lVar6;
    ::utils::rotation_from_normal();
    local_250 = fVar13 + fVar1 * 0.75;
    local_258 = (undefined8 **)
                CONCAT44((float)((ulonglong)uVar9 >> 0x20) +
                         (float)((ulonglong)uVar2 >> 0x20) * 0.75,(float)uVar9 + (float)uVar2 * 0.75
                        );
    local_268 = (undefined **)local_3d8;
    puStack_260 = (undefined8 *)pcStack_3d0;
    uStack_24c = 0x3f800000;
    uStack_248 = 0x3f800000;
    uStack_244 = 0x3f800000;
    country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
    box_trigger_volume();
    local_3d8 = local_130;
    pcStack_3d0 = country_core::resources::walls::impl$7::fmt;
    local_3c8 = &local_158;
    local_3c0 = core::fmt::num::impl$89::fmt;
    local_268 = &PTR_s_roof_ridge_edit_142ff7338;
    puStack_260 = (undefined8 *)0x2;
    local_250 = 2.8026e-45;
    uStack_24c = 0;
    uStack_248 = 0;
    uStack_244 = 0;
    local_e1 = 1;
    local_258 = &local_3d8;
    alloc::fmt::format::format_inner();
    local_198 = local_2f0;
    uStack_190 = uStack_2e8;
    local_188 = local_2e0;
    local_3c0 = (code *)local_1a0;
    pcStack_3d0 = (code *)local_1b0;
    local_3c8 = (longlong *)uStack_1a8;
    local_3d8 = (undefined8 *)CONCAT71(local_3d8._1_7_,2);
    local_e2 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::new();
    local_e3 = 1;
    country_core::systems::ui::resources::widget_params::UiSounds::boop();
    local_300 = 2;
    local_308 = 1;
    local_e3 = 1;
    country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
    local_e3 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
    local_e2 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
    local_e2 = 0;
    puVar8 = &local_198;
    country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
    local_e1 = 0;
    local_3d8 = extraout_RAX_01;
    pcStack_3d0 = (code *)puVar8;
    country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered();
    local_268 = (undefined **)CONCAT71(local_268._1_7_,extraout_AL_00);
    local_e1 = 0;
    country_core::resources::walls::inner_wall_state::StructureType::is_fence();
    if (extraout_AL_01 == '\0') {
      local_e1 = 0;
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      uVar14 = 0x3f800000;
      if (extraout_AL_02 != '\x02') goto LAB_1421aa745;
    }
    else {
LAB_1421aa745:
      uVar14 = 0x3f99999a;
    }
    local_e1 = 0;
    toolbar::mei::core::NodeContext::id();
    uVar5 = local_f0;
    if (local_f0 == local_100) {
      local_e1 = 0;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                ();
    }
    lVar10 = lVar10 + 1;
    puStack_f8[uVar5 * 3] = extraout_RAX_02;
    *(undefined4 *)(puStack_f8 + uVar5 * 3 + 1) = uVar14;
    *(undefined8 *)((longlong)puStack_f8 + uVar5 * 0x18 + 0xc) = uVar2;
    *(float *)((longlong)puStack_f8 + uVar5 * 0x18 + 0x14) = fVar1;
    local_f0 = uVar5 + 1;
    lVar6 = lVar6 + 1;
    lVar7 = lVar7 + 1;
  } while( true );
}

