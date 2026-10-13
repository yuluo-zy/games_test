
void __cdecl system_tree::ui_move_tree::ui_move_tree(void)

{
  undefined1 auVar1 [16];
  ulonglong uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  ushort uVar5;
  ulonglong uVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined8 uVar11;
  char extraout_AL;
  byte bVar12;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar13;
  int iVar14;
  longlong lVar15;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong lVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  size_t _Size;
  undefined1 *puVar18;
  longlong lVar19;
  longlong *in_R9;
  undefined1 (*pauVar20) [16];
  longlong lVar21;
  uint uVar22;
  bool bVar23;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 local_428;
  code *pcStack_420;
  float fStack_418;
  float local_378;
  undefined8 local_374;
  undefined4 local_36c;
  undefined8 local_368;
  undefined *local_310;
  undefined8 *local_308;
  undefined8 *local_300;
  undefined8 local_2f8;
  undefined8 local_2f0;
  undefined8 local_260;
  float local_258;
  longlong local_200;
  undefined1 (*local_1f8) [16];
  ushort local_1e8;
  longlong local_1e0;
  undefined1 local_1d8 [24];
  undefined4 local_1c0;
  undefined8 local_1bc;
  undefined4 local_1b4;
  undefined8 local_1b0;
  longlong local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  longlong local_190;
  longlong local_188;
  undefined8 *local_180;
  undefined4 *local_178;
  undefined4 *local_170;
  undefined4 *local_168;
  undefined8 local_160;
  longlong local_158;
  longlong local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined4 *local_138;
  longlong local_130;
  undefined4 local_124;
  undefined4 local_120;
  undefined4 local_11c;
  longlong local_118;
  longlong local_110;
  undefined8 local_108;
  longlong local_100;
  undefined4 local_f4;
  undefined8 *local_f0;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  local_148 = *in_stack_00000050;
  local_118 = in_stack_00000050[4];
  local_178 = (undefined4 *)in_stack_00000050[6];
  local_124 = *(undefined4 *)((longlong)in_stack_00000050 + 0x3c);
  uVar13 = *(uint *)*in_RCX;
  lVar19 = 0;
  if (uVar13 - 6 < 0xf) {
    lVar19 = (ulonglong)uVar13 - 5;
  }
  local_110 = in_stack_00000050[8];
  local_170 = (undefined4 *)in_stack_00000050[10];
  local_120 = *(undefined4 *)((longlong)in_stack_00000050 + 0x5c);
  if (((lVar19 == 0xb) || ((lVar19 == 0 && (uVar13 == 0)))) &&
     (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0')) {
    country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
    fVar10 = __real_bf99999a;
    fVar9 = __real_c0200000;
    fVar8 = __real_40200000;
    fVar7 = __real_3f800000;
    local_190 = *in_R9;
    local_188 = in_R9[1];
    local_1a0 = *in_RDX;
    local_198 = *in_stack_00000028;
    local_11c = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    local_160 = *in_stack_00000040;
    local_168 = (undefined4 *)in_stack_00000040[2];
    local_140 = *in_stack_00000030;
    local_f4 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    local_100 = *in_stack_00000038;
    local_138 = (undefined4 *)in_stack_00000038[2];
    fVar28 = (float)__xmm_00000000000000003f00000000000000;
    uVar6 = (ulonglong)__xmm_00000000000000003f00000000000000 >> 0x20;
    uVar13 = (uint)local_1e8;
    local_180 = (undefined8 *)((longlong)in_stack_00000048 + 0xc);
    local_130 = local_100 + 0x20;
    local_158 = local_118 + 0x20;
    local_150 = local_110 + 0x20;
    lVar19 = local_200;
    pauVar20 = local_1f8;
    while (local_1a8 = local_1e0 + -1, local_1e0 != 0) {
      if ((short)uVar13 == 0) {
        do {
          auVar1 = *pauVar20;
          uVar5 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          lVar19 = lVar19 + -0x100;
          pauVar20 = pauVar20 + 1;
        } while (uVar5 == 0xffff);
        uVar22 = ~(uint)uVar5;
        uVar13 = uVar22 & -(uint)uVar5 - 2;
      }
      else {
        if (lVar19 == 0) {
          return;
        }
        uVar22 = uVar13;
        uVar13 = uVar13 - 1 & uVar13;
      }
      iVar14 = 0;
      for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
        iVar14 = iVar14 + 1;
      }
      lVar15 = lVar19 - (ulonglong)(uint)(iVar14 << 4);
      local_108 = *(undefined8 *)(lVar15 + -0x10);
      uVar2 = *(ulonglong *)(lVar15 + -8);
      uVar17 = uVar2 & 0xffffffff;
      if ((*(ulonglong *)(local_188 + 0x10) <= uVar17) ||
         (*(int *)(*(longlong *)(local_188 + 8) + uVar17 * 0x14) != (int)(uVar2 >> 0x20))) {
LAB_14226bbf2:
        local_310 = (undefined *)(((ulonglong)local_f0 & 0xffffffff00000000) + 1);
LAB_14226bc13:
        local_308 = (undefined8 *)uVar2;
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      lVar15 = *(longlong *)(local_188 + 8) + uVar17 * 0x14;
      uVar17 = (ulonglong)*(uint *)(lVar15 + 4);
      if (uVar17 == 0xffffffff) goto LAB_14226bbf2;
      if ((*(ulonglong *)(local_190 + 0x38) <= uVar17) ||
         ((*(ulonglong *)
            (*(longlong *)(local_190 + 0x28) + (ulonglong)(*(uint *)(lVar15 + 4) >> 6) * 8) >>
           (uVar17 & 0x3f) & 1) == 0)) {
        local_310 = (undefined *)(uVar17 << 0x20);
        goto LAB_14226bc13;
      }
      lVar21 = *(longlong *)(local_188 + 0x1a8) + (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48;
      uVar2 = *(ulonglong *)
               (*(longlong *)(local_188 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48)
      ;
      if ((*(ulonglong *)(local_190 + 0x110) < uVar2) &&
         (uVar17 = *(ulonglong *)
                    (*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(local_190 + 0x110) * 8),
         uVar17 != 0)) {
        local_f0 = *(undefined8 **)(*(longlong *)(lVar21 + 0x18) + 0x10 + ~uVar17 * 0x30);
        uVar17 = *(ulonglong *)(local_190 + 0x118);
        if (uVar2 <= uVar17) goto LAB_14226b630;
LAB_14226b607:
        uVar2 = *(ulonglong *)(*(longlong *)(lVar21 + 0x38) + uVar17 * 8);
        if (uVar2 == 0) goto LAB_14226b630;
        lVar21 = *(longlong *)(*(longlong *)(lVar21 + 0x18) + 0x10 + ~uVar2 * 0x30);
      }
      else {
        local_f0 = (undefined8 *)0x0;
        uVar17 = *(ulonglong *)(local_190 + 0x118);
        if (uVar17 < uVar2) goto LAB_14226b607;
LAB_14226b630:
        lVar21 = 0;
      }
      uVar22 = *(uint *)(lVar15 + 0x10);
      lVar15 = (ulonglong)uVar22 * 0x58;
      _Size = 0x19;
      asset::prefab::PrefabName::new();
      local_308 = (undefined8 *)_Size;
      if (*(size_t *)(lVar21 + 0x50 + lVar15) == _Size) {
        iVar14 = memcmp((void *)(*(longlong *)(lVar21 + lVar15 + 0x48) + 0x10),extraout_RAX + 2,
                        _Size);
        bVar23 = iVar14 == 0;
        LOCK();
        *extraout_RAX = *extraout_RAX + -1;
        lVar15 = *extraout_RAX;
        UNLOCK();
      }
      else {
        bVar23 = false;
        LOCK();
        *extraout_RAX = *extraout_RAX + -1;
        lVar15 = *extraout_RAX;
        UNLOCK();
      }
      if (lVar15 == 0) {
        alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
      fVar25 = fVar8;
      if (!bVar23) {
        fVar25 = fVar7;
      }
      local_f0 = (undefined8 *)((longlong)local_f0 + (ulonglong)uVar22 * 0x30);
      fVar24 = (float)*local_f0 + fVar28;
      fVar26 = (float)((ulonglong)*local_f0 >> 0x20) + (float)uVar6;
      fVar27 = *(float *)(local_f0 + 1);
      local_428 = &local_108;
      pcStack_420 = country_core::components::impl$13::fmt;
      local_310 = &DAT_143035510;
      local_308 = (undefined8 *)0x1;
      local_300 = &local_428;
      local_2f8 = 1;
      local_2f0 = 0;
      alloc::fmt::format::format_inner();
      local_428 = (undefined8 *)CONCAT44(fVar25,(undefined4)local_428);
      pcStack_420 = (code *)CONCAT44(fVar26,fVar24);
      local_428 = (undefined8 *)((ulonglong)local_428 & 0xffffffffffffff00);
      local_260 = local_260 & 0xffffffffffffff00;
      local_e2 = 1;
      fStack_418 = fVar27 + 0.0;
      country_core::systems::ui::resources::widget_params::WidgetParams::new();
      local_e1 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::empty();
      local_378 = 3.22299e-44;
      local_e1 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
      local_378 = 3.36312e-44;
      local_e1 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_drag_start();
      local_1c0 = 0x19;
      local_368 = local_1b0;
      local_378 = 3.50325e-44;
      local_374 = local_1bc;
      local_36c = local_1b4;
      local_e1 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_drag_release();
      local_e1 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_click();
      local_e1 = 0;
      country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
      local_260 = CONCAT44(fVar26,fVar24);
      local_e2 = 1;
      local_258 = fVar27 + 0.0;
      country_core::systems::ui::resources::widget_params::WidgetParams::
      signifier<system_tree::ui_move_tree::ui_move_tree::closure_env_0>();
      local_e2 = 0;
      puVar18 = local_1d8;
      country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      pcStack_420 = (code *)puVar18;
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      if (local_260._4_4_ < 0.0) {
        local_260 = CONCAT44((local_260._4_4_ / fVar9) * fVar10 +
                             (fVar7 - local_260._4_4_ / fVar9) * 0.0,(float)local_260);
      }
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      bVar12 = extraout_AL - 2;
      if (2 < bVar12) {
        bVar12 = 3;
      }
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(bVar12) {
      case 1:
        fVar25 = *(float *)(local_f0 + 1);
        uVar4 = *local_f0;
        *in_stack_00000048 = uVar4;
        *(float *)(in_stack_00000048 + 1) = fVar25;
        local_378 = (float)uVar4 - (float)local_260;
        local_374 = CONCAT44(local_374._4_4_,fVar25 - local_258);
        ::utils::swizzlers::impl$0::x0y();
        *(undefined4 *)(local_180 + 1) = local_308._0_4_;
        *local_180 = local_310;
        break;
      case 2:
        fVar27 = (float)local_260 + *(float *)((longlong)in_stack_00000048 + 0xc);
        fVar25 = local_258 + *(float *)((longlong)in_stack_00000048 + 0x14);
        country_core::resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        country_core::resources::glade_border::GladeBorder::is_pos_inside();
        uVar4 = local_108;
        if (extraout_AL_00 == '\0') {
          *local_178 = local_124;
          lVar15 = *(longlong *)(local_118 + 0x30);
          uVar4 = *(undefined8 *)(local_118 + 0x40);
          if (lVar15 == *(longlong *)(local_118 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(local_118 + 0x28) + lVar15 * 8) = uVar4;
          *(longlong *)(local_118 + 0x30) = lVar15 + 1;
          *(longlong *)(local_118 + 0x40) = *(longlong *)(local_118 + 0x40) + 1;
          *local_170 = local_120;
          lVar15 = *(longlong *)(local_110 + 0x30);
          uVar4 = *(undefined8 *)(local_110 + 0x40);
          if (lVar15 == *(longlong *)(local_110 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(local_110 + 0x28) + lVar15 * 8) = uVar4;
          *(longlong *)(local_110 + 0x30) = lVar15 + 1;
          *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
        }
        else {
          *local_138 = local_f4;
          lVar15 = *(longlong *)(local_100 + 0x30);
          uVar3 = *(undefined8 *)(local_100 + 0x40);
          if (lVar15 == *(longlong *)(local_100 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          fVar24 = extraout_XMM0_Da;
          if (extraout_XMM0_Da <= 0.0) {
            fVar24 = 0.0;
          }
          lVar21 = *(longlong *)(local_100 + 0x28);
          lVar16 = lVar15 * 0x20;
          *(undefined8 *)(lVar21 + lVar16) = uVar3;
          *(undefined8 *)(lVar21 + 8 + lVar16) = uVar4;
          *(float *)(lVar21 + 0x10 + lVar16) = fVar27;
          *(float *)(lVar21 + 0x14 + lVar16) = fVar24;
          *(float *)(lVar21 + 0x18 + lVar16) = fVar25;
          *(longlong *)(local_100 + 0x30) = lVar15 + 1;
          *(longlong *)(local_100 + 0x40) = *(longlong *)(local_100 + 0x40) + 1;
        }
        break;
      case 3:
        fVar25 = (float)local_260 + *(float *)((longlong)in_stack_00000048 + 0xc);
        fVar27 = local_258 + *(float *)((longlong)in_stack_00000048 + 0x14);
        country_core::resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        country_core::resources::glade_border::GladeBorder::is_pos_inside();
        uVar4 = local_108;
        if (extraout_AL_01 != '\0') {
          *local_138 = local_f4;
          lVar15 = *(longlong *)(local_100 + 0x30);
          uVar3 = *(undefined8 *)(local_100 + 0x40);
          if (lVar15 == *(longlong *)(local_100 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          uVar11 = local_108;
          fVar24 = extraout_XMM0_Da_00;
          if (extraout_XMM0_Da_00 <= 0.0) {
            fVar24 = 0.0;
          }
          lVar21 = *(longlong *)(local_100 + 0x28);
          lVar16 = lVar15 * 0x20;
          *(undefined8 *)(lVar21 + lVar16) = uVar3;
          *(undefined8 *)(lVar21 + 8 + lVar16) = uVar4;
          *(float *)(lVar21 + 0x10 + lVar16) = fVar25;
          *(float *)(lVar21 + 0x14 + lVar16) = fVar24;
          *(float *)(lVar21 + 0x18 + lVar16) = fVar27;
          *(longlong *)(local_100 + 0x30) = lVar15 + 1;
          *(longlong *)(local_100 + 0x40) = *(longlong *)(local_100 + 0x40) + 1;
          *local_168 = local_11c;
          local_310 = (undefined *)CONCAT44(local_310._4_4_,fVar27);
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_00 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_00 = 1;
          extraout_RAX_00[1] = 1;
          *(undefined4 *)(extraout_RAX_00 + 2) = 1;
          *(undefined8 *)((longlong)extraout_RAX_00 + 0x14) = *in_stack_00000048;
          *(undefined4 *)((longlong)extraout_RAX_00 + 0x1c) = *(undefined4 *)(in_stack_00000048 + 1)
          ;
          *(float *)(extraout_RAX_00 + 4) = fVar25;
          *(float *)((longlong)extraout_RAX_00 + 0x24) = fVar24;
          extraout_RAX_00[5] = local_310;
          extraout_RAX_00[6] = uVar11;
          local_310 = (undefined *)0xc;
          local_308 = extraout_RAX_00;
          country_core::resources::history::History::add();
        }
      }
      local_1e0 = local_1a8;
    }
  }
  return;
}

