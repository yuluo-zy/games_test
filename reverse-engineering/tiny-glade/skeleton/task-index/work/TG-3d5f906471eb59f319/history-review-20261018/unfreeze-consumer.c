
void __cdecl country_core::systems::debug::sheep_controls::sheep_controls(void)

{
  float fVar1;
  char cVar2;
  uint uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  undefined *puVar6;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar7;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined4 *puVar8;
  uint *puVar9;
  longlong *in_RDX;
  ulonglong uVar10;
  longlong lVar11;
  undefined ***pppuVar12;
  longlong lVar13;
  undefined8 *puVar14;
  ulonglong uVar15;
  undefined **ppuVar16;
  longlong lVar17;
  longlong *plVar18;
  longlong lVar19;
  undefined4 *puVar20;
  longlong lVar21;
  undefined4 *puVar22;
  longlong *in_R8;
  undefined8 *in_R9;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong lVar25;
  ulonglong uVar26;
  longlong lVar27;
  undefined **ppuVar28;
  longlong lVar29;
  uint *puVar30;
  undefined4 *puVar31;
  bool bVar32;
  undefined4 uVar33;
  float fVar34;
  undefined4 uVar35;
  float fVar36;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined ***in_stack_00000038;
  longlong in_stack_00000040;
  undefined8 *in_stack_00000050;
  undefined **local_448;
  undefined8 uStack_440;
  undefined **local_438;
  longlong local_430 [5];
  int local_408;
  longlong local_320;
  undefined *local_2c0;
  code *local_2b8;
  longlong local_2b0;
  uint *local_2a8;
  undefined **local_2a0;
  undefined8 local_298;
  undefined **local_290;
  longlong local_288;
  longlong local_280;
  longlong lStack_278;
  undefined8 local_270;
  undefined *local_268;
  undefined8 local_260;
  undefined8 local_258;
  undefined **ppuStack_250;
  undefined8 local_248;
  undefined ***local_240;
  longlong local_238;
  undefined8 local_230;
  undefined2 local_228;
  uint local_218;
  undefined4 local_214;
  undefined **local_210;
  float local_208;
  uint local_1f8;
  undefined4 local_1f4;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined1 local_1e0;
  undefined4 *local_118;
  longlong local_110;
  undefined **local_108;
  undefined8 local_100;
  longlong local_f8;
  byte *local_f0;
  longlong local_e8;
  undefined **local_e0;
  longlong local_d8;
  undefined **local_d0;
  byte *local_c8;
  longlong *local_c0;
  undefined4 local_b4;
  longlong local_b0;
  undefined8 *local_a8;
  longlong local_a0;
  undefined1 local_91 [81];
  
  local_91._1_8_ = 0xfffffffffffffffe;
  lVar29 = *in_R8;
  lVar11 = in_R8[1];
  lVar17 = in_R8[2];
  puVar30 = *(uint **)(lVar29 + 0xf8);
  lVar13 = *(longlong *)(lVar29 + 0x100);
  cVar2 = *(char *)(lVar29 + 0x108);
  puVar9 = puVar30;
  if (cVar2 == '\0') {
    puVar14 = &local_298;
    pppuVar12 = &local_290;
    plVar18 = &local_288;
    lVar25 = lVar13 * 4;
    do {
      if (lVar25 == 0) goto LAB_1416f13b9;
      uVar3 = *puVar9;
      puVar9 = puVar9 + 1;
      lVar19 = *(longlong *)(*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0);
      lVar25 = lVar25 + -4;
    } while (lVar19 == 0);
    lVar21 = *(longlong *)(lVar11 + 0x100) + (ulonglong)uVar3 * 0xa0;
    uVar15 = (ulonglong)*(uint *)(lVar21 + 0x94);
    lVar25 = *(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x18 + uVar15 * 0x48);
    lVar27 = ~*(ulonglong *)
               (*(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x38 + uVar15 * 0x48) +
               *(longlong *)(lVar29 + 0x110) * 8) * 0x30;
    uVar15 = (ulonglong)*(uint *)(*(longlong *)(lVar21 + 0x50) + 8);
    ppuVar28 = (undefined **)(uVar15 * 0x50 + *(longlong *)(lVar25 + 0x10 + lVar27));
    lVar29 = uVar15 * 4;
    ppuVar16 = (undefined **)(lVar29 + *(longlong *)(lVar25 + 0x28 + lVar27));
    local_2a0 = ppuVar28;
    local_298 = (char *)(*(longlong *)(lVar25 + 0x20 + lVar27) + lVar29);
  }
  else {
    puVar14 = &uStack_440;
    pppuVar12 = &local_438;
    plVar18 = local_430;
    lVar25 = lVar13 * 4;
    do {
      if (lVar25 == 0) goto LAB_1416f13b9;
      uVar3 = *puVar9;
      puVar9 = puVar9 + 1;
      lVar19 = *(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48);
      lVar25 = lVar25 + -4;
    } while (lVar19 == 0);
    lVar25 = *(longlong *)(lVar11 + 0x1a8) + (ulonglong)uVar3 * 0x48;
    lVar21 = *(longlong *)(lVar25 + 0x18);
    lVar29 = ~*(ulonglong *)(*(longlong *)(lVar25 + 0x38) + *(longlong *)(lVar29 + 0x110) * 8) *
             0x30;
    ppuVar16 = *(undefined ***)(lVar21 + 0x28 + lVar29);
    ppuVar28 = *(undefined ***)(lVar21 + 0x10 + lVar29);
    uStack_440 = *(code **)(lVar21 + 0x20 + lVar29);
    local_448 = ppuVar28;
  }
  *pppuVar12 = ppuVar16;
  *plVar18 = lVar17;
  pppuVar12 = (undefined ***)puVar14[1];
  uVar35 = (undefined4)((ulonglong)puVar14[2] >> 0x20);
  lVar17 = 1;
  if (cVar2 != '\0') goto LAB_1416f13c7;
LAB_1416f1381:
  if (lVar17 == lVar19) {
    do {
      if (puVar9 == puVar30 + lVar13) goto LAB_1416f1445;
      uVar3 = *puVar9;
      puVar9 = puVar9 + 1;
    } while (*(longlong *)(*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0) == 0);
  }
LAB_1416f13ef:
  bVar32 = ppuVar28 == (undefined **)0x0;
LAB_1416f13f5:
  local_2a0 = (undefined **)(ulonglong)(bVar32 ^ 1);
  local_298 = 
  "bevy_ecs::system::query::Query<&mut country_core::systems::debug::sheep_controls::SheepControl>/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs"
  ;
  local_290 = (undefined **)0x5f;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
LAB_1416f13b9:
  uVar35 = (undefined4)((ulonglong)puVar14 >> 0x20);
  lVar17 = 0;
  lVar19 = 0;
  ppuVar28 = (undefined **)0x0;
  if (cVar2 == '\0') goto LAB_1416f1381;
LAB_1416f13c7:
  if (lVar17 == lVar19) {
    do {
      if (puVar9 == puVar30 + lVar13) goto LAB_1416f1445;
      uVar3 = *puVar9;
      puVar9 = puVar9 + 1;
    } while (*(longlong *)(*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48) == 0);
  }
  goto LAB_1416f13ef;
LAB_1416f1445:
  bVar32 = ppuVar28 == (undefined **)0x0;
  if (!bVar32) {
    local_c0 = in_RDX;
    local_a8 = in_R9;
    *(undefined4 *)pppuVar12 = uVar35;
    if (SBORROW8(0,(longlong)*ppuVar28)) {
      puVar6 = (undefined *)0x8000000000000000;
    }
    else {
      apply_sheep_snapshot();
      puVar6 = *ppuVar28;
    }
    bVar32 = SBORROW8(0,(longlong)puVar6);
    local_91[0] = !bVar32;
    if (game_config::DEVELOPER_HOTKEYS_ENABLED != '\0') {
      local_2a0 = (undefined **)CONCAT44(local_2a0._4_4_,0x28);
      hashbrown::map::HashMap::
      get_inner<enum2_<bevy_input::keyboard::KeyCode>,tuple_<>,bevy_platform::hash::FixedHasher,allocator_api2::stable::alloc::global::Global,enum2_<bevy_input::keyboard::KeyCode>_>
                ();
      if ((extraout_RAX != 0) &&
         (local_91[0] = bVar32,
         2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
        local_448 = (undefined **)local_91;
        uStack_440 = core::fmt::impl$18::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_280 = *extraout_RAX_00;
        lStack_278 = extraout_RAX_00[1];
        local_270 = 3;
        local_268 = &DAT_142c90420;
        local_260 = 0x2c;
        ppuStack_250 = &PTR_s_Sheep_freeze_142c903f8;
        local_248 = 1;
        local_240 = &local_448;
        local_238 = 1;
        local_230 = 0;
        local_2a0 = (undefined **)0x0;
        local_298 = &DAT_142c90420;
        local_290 = (undefined **)&DAT_0000002c;
        local_288 = 0;
        local_258 = CONCAT44((int)extraout_RAX_00[2],1);
        ::log::__private_api::impl$2::log();
      }
    }
    ppuVar16 = *in_stack_00000038;
    if ((*(char *)((longlong)ppuVar16 + 0xf5) == '\0') || (*(char *)(ppuVar16 + 0x1e) == '\0')) {
      plVar18 = *(longlong **)(in_stack_00000040 + 8);
      if (plVar18 == (longlong *)0x0) {
        plVar18 = *(longlong **)(in_stack_00000040 + 0x10);
      }
      puVar6 = ppuVar28[6];
      lVar17 = plVar18[2];
      if ((ulonglong)(*plVar18 - lVar17) < 0x18) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar29 = plVar18[1];
      *(code **)(lVar29 + lVar17) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<utils::components::transform::Transform>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar29 + 8 + lVar17) = bevy_ecs::error::handler::warn;
      *(undefined **)(lVar29 + 0x10 + lVar17) = puVar6;
      plVar18[2] = lVar17 + 0x18;
      plVar18 = *(longlong **)(in_stack_00000040 + 8);
      if (plVar18 == (longlong *)0x0) {
        plVar18 = *(longlong **)(in_stack_00000040 + 0x10);
      }
      puVar6 = ppuVar28[7];
      lVar17 = plVar18[2];
      if ((ulonglong)(*plVar18 - lVar17) < 0x18) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar29 = plVar18[1];
      *(code **)(lVar29 + lVar17) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<utils::components::transform::Transform>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar29 + 8 + lVar17) = bevy_ecs::error::handler::warn;
      *(undefined **)(lVar29 + 0x10 + lVar17) = puVar6;
      plVar18[2] = lVar17 + 0x18;
    }
    else if (*in_stack_00000030 != 0) {
      egui::containers::window::Window::new<ref_<str_>_>();
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      local_320 = (longlong)ppuVar16 + 0xf5;
      memcpy(&local_2a0,&local_448,0x188);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      local_e0 = ppuVar28 + 8;
      local_108 = ppuVar28 + 9;
      *extraout_RAX_01 = (longlong)local_91;
      extraout_RAX_01[1] = (longlong)local_e0;
      extraout_RAX_01[2] = (longlong)local_108;
      extraout_RAX_01[3] = (longlong)(ppuVar28 + 6);
      extraout_RAX_01[4] = (longlong)(ppuVar28 + 7);
      extraout_RAX_01[5] = (longlong)ppuVar28;
      extraout_RAX_01[6] = (longlong)(ppuVar28 + 3);
      extraout_RAX_01[7] = (longlong)local_c0;
      extraout_RAX_01[8] = in_stack_00000040;
      egui::containers::window::Window::show_dyn<tuple_<>_>();
      local_d0 = ppuVar28;
      if (local_408 != 2) {
        LOCK();
        *local_438 = *local_438 + -1;
        UNLOCK();
        if (*local_438 == (undefined *)0x0) {
          alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
      }
      local_d8 = *in_stack_00000028;
      local_c8 = (byte *)*local_a8;
      local_f8 = *local_c0;
      local_2b0 = local_c0[1];
      local_b4 = *(undefined4 *)((longlong)local_c0 + 0x14);
      puVar30 = *(uint **)(local_f8 + 0xf8);
      local_2a8 = puVar30 + *(longlong *)(local_f8 + 0x100);
      local_a8 = (undefined8 *)CONCAT71(local_a8._1_7_,*(undefined1 *)(local_f8 + 0x108));
      fVar1 = *(float *)(local_c8 + 0x10);
      fVar34 = (float)*(undefined8 *)(local_c8 + 0x14);
      fVar36 = (float)((ulonglong)*(undefined8 *)(local_c8 + 0x14) >> 0x20);
      local_f0 = local_c8 + 4;
      local_110 = 8;
      local_a0 = 0;
      local_b0 = 0;
      uVar15 = 0;
      uVar26 = 0;
      local_100 = 0;
LAB_1416f1c68:
      do {
        ppuVar28 = local_d0;
        if ((char)local_a8 == '\0') {
          if (uVar26 == uVar15) {
            do {
              if (puVar30 == local_2a8) goto LAB_1416f1e18;
              uVar3 = *puVar30;
              puVar30 = puVar30 + 1;
              uVar15 = *(ulonglong *)
                        (*(longlong *)(local_2b0 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0);
            } while (uVar15 == 0);
            lVar11 = *(longlong *)(local_2b0 + 0x100) + (ulonglong)uVar3 * 0xa0;
            uVar26 = (ulonglong)*(uint *)(lVar11 + 0x94);
            lVar17 = *(longlong *)(*(longlong *)(local_2b0 + 0x1a8) + 0x18 + uVar26 * 0x48);
            lVar29 = *(longlong *)(*(longlong *)(local_2b0 + 0x1a8) + 0x38 + uVar26 * 0x48);
            lVar13 = ~*(ulonglong *)(lVar29 + *(longlong *)(local_f8 + 0x110) * 8) * 0x30;
            local_b0 = *(longlong *)(lVar17 + 0x10 + lVar13);
            local_e8 = *(longlong *)(lVar17 + 0x28 + lVar13);
            local_a0 = *(longlong *)
                        (lVar17 + 0x10 +
                        ~*(ulonglong *)(lVar29 + *(longlong *)(local_f8 + 0x118) * 8) * 0x30);
            local_110 = *(longlong *)(lVar11 + 0x50);
            uVar26 = 0;
          }
          uVar7 = (ulonglong)*(uint *)(local_110 + 8 + uVar26 * 0x10);
        }
        else {
          if (uVar26 == uVar15) {
            do {
              if (puVar30 == local_2a8) goto LAB_1416f1e18;
              uVar3 = *puVar30;
              puVar30 = puVar30 + 1;
              uVar15 = *(ulonglong *)
                        (*(longlong *)(local_2b0 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48);
            } while (uVar15 == 0);
            lVar17 = *(longlong *)(local_2b0 + 0x1a8) + (ulonglong)uVar3 * 0x48;
            lVar29 = *(longlong *)(lVar17 + 0x18);
            lVar17 = *(longlong *)(lVar17 + 0x38);
            lVar11 = ~*(ulonglong *)(lVar17 + *(longlong *)(local_f8 + 0x110) * 8) * 0x30;
            local_b0 = *(longlong *)(lVar29 + 0x10 + lVar11);
            local_e8 = *(longlong *)(lVar29 + 0x28 + lVar11);
            local_a0 = *(longlong *)
                        (lVar29 + 0x10 +
                        ~*(ulonglong *)(lVar17 + *(longlong *)(local_f8 + 0x118) * 8) * 0x30);
            uVar26 = 0;
          }
          uVar7 = uVar26 & 0xffffffff;
        }
        lVar17 = uVar7 * 0x30 + local_b0;
        local_118 = (undefined4 *)(local_e8 + uVar7 * 4);
        lVar29 = uVar7 * 0x68 + local_a0;
        uVar26 = uVar26 + 1;
        local_2c0 = (undefined *)(lVar29 + 0x20);
        local_2b8 = core::fmt::num::imp::impl$9::fmt;
        local_2a0 = &PTR_DAT_142c905a8;
        local_298 = (char *)0x1;
        local_290 = &local_2c0;
        local_288 = 1;
        local_280 = 0;
        in_stack_00000038 = &local_448;
        alloc::fmt::format::format_inner();
        local_208 = *(float *)(lVar17 + 0x18) + 0.0;
        local_210 = (undefined **)
                    CONCAT44((float)((ulonglong)*(undefined8 *)(lVar17 + 0x10) >> 0x20) + 0.8,
                             (float)*(undefined8 *)(lVar17 + 0x10) + 0.0);
        local_218 = local_218 & 0xffffff00;
        local_214 = 0x3fa00000;
        local_228 = 0xe01;
        local_288 = CONCAT44(local_288._4_4_,1);
        local_280 = 0;
        local_270 = CONCAT44(local_270._4_4_,2);
        local_258 = CONCAT44(local_258._4_4_,3);
        local_240 = (undefined ***)CONCAT44(local_240._4_4_,3);
        local_2a0 = (undefined **)0x0;
        local_298 = &DAT_00000008;
        local_290 = (undefined **)0x0;
        local_1f8 = CONCAT22(local_1f8._2_2_,0x101);
        pppuVar12 = in_stack_00000038;
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      } while (*(byte *)(extraout_RAX_02 + 0x18) < 2);
      if (((*(undefined ****)(extraout_RAX_02 + 0x38) == pppuVar12) &&
          (*(char *)(extraout_RAX_02 + 0x19) == '\x04')) &&
         (0.01 < SQRT(fVar36 * fVar36 + fVar1 * fVar1 + fVar34 * fVar34)))
      goto code_r0x0001416f1d95;
      goto LAB_1416f1de5;
    }
    goto LAB_1416f186d;
  }
  goto LAB_1416f13f5;
LAB_1416f1e18:
  cVar2 = *(char *)local_108;
  if (cVar2 == '\0') goto LAB_1416f2183;
  if ((*local_c8 & 1) == 0) {
    uStack_440 = (code *)((ulonglong)uStack_440._4_4_ << 0x20);
    local_448 = (undefined **)0x0;
  }
  else {
    uStack_440 = (code *)CONCAT44(uStack_440._4_4_,*(undefined4 *)(local_f0 + 8));
    local_448 = *(undefined ***)local_f0;
  }
  local_208 = (float)uStack_440;
  local_210 = local_448;
  local_2a0 = (undefined **)0x0;
  local_298 = &DAT_00000008;
  local_290 = (undefined **)0x0;
  local_288 = CONCAT44(local_288._4_4_,1);
  local_280 = 0;
  local_270 = CONCAT44(local_270._4_4_,2);
  local_258 = CONCAT44(local_258._4_4_,3);
  local_240 = (undefined ***)CONCAT44(local_240._4_4_,3);
  local_228 = 0x101;
  local_218 = local_218 & 0xffffff00;
  local_214 = 0x3e800000;
  local_1f8 = CONCAT22(local_1f8._2_2_,0x100);
  puVar6 = &DAT_142c90588;
  systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
  ppuVar16 = local_d0;
  if ((((*(byte *)(extraout_RAX_03 + 0x18) < 2) || (1 < *(byte *)(extraout_RAX_03 + 0x19))) ||
      ((*(byte *)(extraout_RAX_03 + 0x19) & 1) == 0)) ||
     (*(undefined **)(extraout_RAX_03 + 0x38) != puVar6)) goto LAB_1416f2183;
  lVar17 = *local_c0;
  lVar29 = local_c0[1];
  puVar9 = *(uint **)(lVar17 + 0xf8);
  puVar30 = puVar9 + *(longlong *)(lVar17 + 0x100);
  if (*(char *)(lVar17 + 0x108) == '\0') {
    lVar25 = 8;
    lVar21 = 0;
    lVar13 = 0;
    lVar11 = 0;
    local_a8 = (undefined8 *)0x0;
    lVar19 = 0;
    do {
      if (lVar21 == lVar19) {
        do {
          if (puVar9 == puVar30) goto LAB_1416f2179;
          uVar3 = *puVar9;
          puVar9 = puVar9 + 1;
          lVar19 = *(longlong *)(*(longlong *)(lVar29 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0);
        } while (lVar19 == 0);
        lVar27 = *(longlong *)(lVar29 + 0x100) + (ulonglong)uVar3 * 0xa0;
        uVar15 = (ulonglong)*(uint *)(lVar27 + 0x94);
        lVar25 = *(longlong *)(*(longlong *)(lVar29 + 0x1a8) + 0x18 + uVar15 * 0x48);
        lVar13 = *(longlong *)(*(longlong *)(lVar29 + 0x1a8) + 0x38 + uVar15 * 0x48);
        lVar11 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x110) * 8) * 0x30;
        local_a8 = *(undefined8 **)(lVar25 + 0x10 + lVar11);
        local_a0 = *(longlong *)(lVar25 + 0x28 + lVar11);
        lVar21 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x118) * 8) * 0x30;
        lVar11 = *(longlong *)(lVar25 + 0x10 + lVar21);
        local_b0 = *(longlong *)(lVar25 + 0x28 + lVar21);
        lVar21 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x120) * 8) * 0x30;
        lVar13 = *(longlong *)(lVar25 + 0x10 + lVar21);
        local_d8 = *(longlong *)(lVar25 + 0x28 + lVar21);
        lVar25 = *(longlong *)(lVar27 + 0x50);
        lVar21 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(lVar25 + 8 + lVar21 * 0x10);
      lVar21 = lVar21 + 1;
    } while (*(undefined **)(lVar11 + 0x20 + uVar15 * 0x68) != *local_e0);
    lVar11 = lVar11 + uVar15 * 0x68;
    puVar14 = local_a8 + uVar15 * 6;
    puVar8 = (undefined4 *)(local_a0 + uVar15 * 4);
    puVar20 = (undefined4 *)(local_b0 + uVar15 * 4);
    puVar22 = (undefined4 *)(local_d8 + uVar15 * 4);
    puVar31 = (undefined4 *)(lVar13 + uVar15 * 0x10);
  }
  else {
    uVar26 = 0;
    lVar13 = 0;
    lVar11 = 0;
    local_a8 = (undefined8 *)0x0;
    uVar15 = 0;
    do {
      if (uVar26 == uVar15) {
        do {
          if (puVar9 == puVar30) goto LAB_1416f2179;
          uVar3 = *puVar9;
          puVar9 = puVar9 + 1;
          uVar15 = *(ulonglong *)(*(longlong *)(lVar29 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48);
        } while (uVar15 == 0);
        lVar11 = *(longlong *)(lVar29 + 0x1a8) + (ulonglong)uVar3 * 0x48;
        lVar25 = *(longlong *)(lVar11 + 0x18);
        lVar13 = *(longlong *)(lVar11 + 0x38);
        lVar11 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x110) * 8) * 0x30;
        local_a8 = *(undefined8 **)(lVar25 + 0x10 + lVar11);
        local_a0 = *(longlong *)(lVar25 + 0x28 + lVar11);
        lVar19 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x118) * 8) * 0x30;
        lVar11 = *(longlong *)(lVar25 + 0x10 + lVar19);
        local_b0 = *(longlong *)(lVar25 + 0x28 + lVar19);
        lVar19 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar17 + 0x120) * 8) * 0x30;
        lVar13 = *(longlong *)(lVar25 + 0x10 + lVar19);
        in_stack_00000038 = *(undefined ****)(lVar25 + 0x28 + lVar19);
        uVar26 = 0;
      }
      uVar7 = uVar26 & 0xffffffff;
      uVar26 = uVar26 + 1;
    } while (*(undefined **)(lVar11 + 0x20 + uVar7 * 0x68) != *local_e0);
    lVar11 = lVar11 + uVar7 * 0x68;
    puVar14 = local_a8 + uVar7 * 6;
    puVar8 = (undefined4 *)(local_a0 + uVar7 * 4);
    puVar20 = (undefined4 *)(local_b0 + uVar7 * 4);
    puVar22 = (undefined4 *)((longlong)in_stack_00000038 + uVar7 * 4);
    puVar31 = (undefined4 *)(lVar13 + uVar7 * 0x10);
  }
  if (cVar2 == '\x02') {
    uVar33 = 0;
    uVar35 = 0;
    if ((*local_c8 & 1) != 0) {
      uVar35 = *(undefined4 *)(local_c8 + 4);
      uVar33 = *(undefined4 *)(local_c8 + 0xc);
    }
    *(undefined4 *)(puVar14 + 2) = uVar35;
    *puVar8 = local_b4;
    *(undefined4 *)(puVar14 + 3) = uVar33;
  }
  else {
    if ((*local_c8 & 1) == 0) {
      local_298 = (char *)((ulonglong)local_298._4_4_ << 0x20);
      local_2a0 = (undefined **)0x0;
    }
    else {
      local_298 = (char *)CONCAT44(local_298._4_4_,*(undefined4 *)(local_f0 + 8));
      local_2a0 = *(undefined ***)local_f0;
    }
    ::utils::swizzlers::impl$3::x0z();
    uVar35 = local_b4;
    *puVar22 = local_b4;
    *puVar31 = 1;
    *(undefined ***)(puVar31 + 1) = local_448;
    puVar31[3] = (float)uStack_440;
    *puVar20 = uVar35;
    *(undefined2 *)(lVar11 + 0x30) = 0x101;
    *(undefined8 *)(lVar11 + 0x34) = 0x7f7fffff00000000;
  }
  *(undefined1 *)(ppuVar16 + 9) = 0;
  ppuVar28 = ppuVar16;
  if (SBORROW8(0,(longlong)*ppuVar16)) goto LAB_1416f186d;
  goto LAB_1416f2198;
code_r0x0001416f1d95:
  uVar33 = 0;
  uVar35 = 0;
  if ((*local_c8 & 1) != 0) {
    uVar35 = *(undefined4 *)(local_c8 + 4);
    uVar33 = *(undefined4 *)(local_c8 + 0xc);
  }
  *(undefined4 *)(lVar17 + 0x10) = uVar35;
  *local_118 = local_b4;
  *(undefined4 *)(lVar17 + 0x18) = uVar33;
  local_100 = CONCAT71((uint7)(uint3)((uint)local_b4 >> 8),1);
  if (1 < *(byte *)(extraout_RAX_02 + 0x18)) {
LAB_1416f1de5:
    if (((*(byte *)(extraout_RAX_02 + 0x19) < 2) && ((*(byte *)(extraout_RAX_02 + 0x19) & 1) != 0))
       && (*(undefined ****)(extraout_RAX_02 + 0x38) == pppuVar12)) {
      *local_e0 = *(undefined **)(lVar29 + 0x20);
    }
  }
  goto LAB_1416f1c68;
LAB_1416f2179:
  *(char *)local_108 = '\0';
  ppuVar28 = local_d0;
LAB_1416f2183:
  if (((byte)local_100 & !SBORROW8(0,(longlong)*ppuVar28)) == 0) goto LAB_1416f186d;
LAB_1416f2198:
  local_288 = *local_c0;
  local_2a0 = (undefined **)local_c0[1];
  local_218 = *(uint *)(local_c0 + 2);
  local_214 = *(undefined4 *)((longlong)local_c0 + 0x14);
  local_238 = local_c0[2];
  local_298 = (char *)(local_2a0 + 0x34);
  local_290 = local_2a0 + 0x1f;
  local_280 = *(longlong *)(local_288 + 0xf8);
  lStack_278 = local_280 + *(longlong *)(local_288 + 0x100) * 4;
  local_1e0 = *(undefined1 *)(local_288 + 0x108);
  local_270 = 8;
  local_268 = (undefined *)0x0;
  local_260 = 8;
  local_258 = 0;
  ppuStack_250 = (undefined **)0x0;
  local_230 = 0;
  local_210 = (undefined **)0x0;
  local_1f0 = 0;
  uStack_1e8 = 0;
  local_1f8 = local_218;
  local_1f4 = local_214;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::critters::sheep::control::SheepSnapshot,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::sheep::Sheep>,ref_mut_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env_0>_>
            ();
  if (*ppuVar28 != (undefined *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  ppuVar28[2] = (undefined *)local_438;
  *ppuVar28 = (undefined *)local_448;
  ppuVar28[1] = uStack_440;
LAB_1416f186d:
  puVar6 = *ppuVar28;
  if (local_91[0] == '\0') {
    if ((!SBORROW8(0,(longlong)puVar6)) && (puVar6 != (undefined *)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *ppuVar28 = (undefined *)0x8000000000000000;
  }
  else if (SBORROW8(0,(longlong)puVar6)) {
    local_288 = *local_c0;
    local_2a0 = (undefined **)local_c0[1];
    local_218 = *(uint *)(local_c0 + 2);
    local_214 = *(undefined4 *)((longlong)local_c0 + 0x14);
    local_238 = local_c0[2];
    local_298 = (char *)(local_2a0 + 0x34);
    local_290 = local_2a0 + 0x1f;
    local_280 = *(longlong *)(local_288 + 0xf8);
    lStack_278 = local_280 + *(longlong *)(local_288 + 0x100) * 4;
    local_1e0 = *(undefined1 *)(local_288 + 0x108);
    local_270 = 8;
    local_268 = (undefined *)0x0;
    local_260 = 8;
    local_258 = 0;
    ppuStack_250 = (undefined **)0x0;
    local_230 = 0;
    local_210 = (undefined **)0x0;
    local_1f0 = 0;
    uStack_1e8 = 0;
    local_1f8 = local_218;
    local_1f4 = local_214;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::resources::critters::sheep::control::SheepSnapshot,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_mut_<utils::components::transform::Transform>,ref_mut_<country_core::resources::critters::sheep::Sheep>,ref_mut_<country_core::resources::critters::CritterGoal>_>,tuple_<>_>,country_core::resources::critters::sheep::control::capture_sheep_snapshot::closure_env_0>_>
              ();
    if ((!SBORROW8(0,(longlong)*ppuVar28)) && (*ppuVar28 != (undefined *)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ppuVar28[2] = (undefined *)local_438;
    *ppuVar28 = (undefined *)local_448;
    ppuVar28[1] = uStack_440;
  }
  puVar4 = (ulonglong *)*in_stack_00000050;
  lVar17 = in_stack_00000050[1];
  uVar15 = *puVar4;
  uVar7 = 0;
  uVar26 = uVar15 - *(ulonglong *)(lVar17 + 0x18);
  if (uVar15 < *(ulonglong *)(lVar17 + 0x18)) {
    uVar26 = uVar7;
  }
  uVar10 = uVar15 - *(ulonglong *)(lVar17 + 0x38);
  if (uVar15 < *(ulonglong *)(lVar17 + 0x38)) {
    uVar10 = uVar7;
  }
  uVar15 = *(ulonglong *)(lVar17 + 0x30);
  uVar23 = uVar15 - uVar10;
  if (uVar15 < uVar10) {
    uVar23 = uVar7;
  }
  uVar5 = *(ulonglong *)(lVar17 + 0x10);
  uVar24 = uVar5 - uVar26;
  if (uVar5 < uVar26) {
    uVar24 = uVar7;
  }
  uVar7 = *(ulonglong *)(lVar17 + 0x40);
  *puVar4 = uVar7 - (uVar24 + uVar23);
  if (uVar5 >= uVar26 && uVar5 - uVar26 != 0 || uVar15 >= uVar10 && uVar15 - uVar10 != 0) {
    *puVar4 = uVar7;
    if ((!SBORROW8(0,(longlong)*ppuVar28)) && (*ppuVar28 != (undefined *)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *ppuVar28 = (undefined *)0x8000000000000000;
  }
  return;
}


