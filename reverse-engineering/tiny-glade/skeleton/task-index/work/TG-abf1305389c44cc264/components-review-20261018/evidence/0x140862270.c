
void __cdecl country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees(void)

{
  float fVar1;
  float fVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  code *pcVar9;
  longlong *plVar10;
  float *pfVar11;
  undefined8 *extraout_RAX;
  undefined8 **extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined8 *in_RCX;
  longlong lVar12;
  float *pfVar13;
  longlong **pplVar14;
  float *pfVar15;
  float *pfVar16;
  longlong *plVar17;
  ulonglong uVar18;
  float *pfVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  longlong *in_R9;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  longlong lVar24;
  ulonglong uVar25;
  longlong *plVar26;
  float *pfVar27;
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  longlong *in_stack_00000028;
  longlong local_338 [4];
  undefined8 local_318;
  float *pfStack_310;
  undefined8 local_308;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  float local_2e8;
  float *local_2e0;
  longlong *local_2d8;
  undefined8 local_2d0;
  undefined4 *local_2c0;
  undefined4 *local_2b8;
  longlong local_2b0;
  longlong local_2a8;
  longlong *local_2a0;
  longlong *local_298;
  float *pfStack_290;
  longlong local_288;
  longlong lStack_280;
  longlong local_278;
  longlong lStack_270;
  longlong local_268;
  longlong lStack_260;
  longlong local_258;
  longlong *local_248;
  longlong *local_240;
  longlong local_238;
  longlong *local_230;
  longlong *local_228;
  longlong *local_220;
  float *local_218;
  undefined **local_210;
  float *pfStack_208;
  longlong **local_200;
  undefined8 uStack_1f8;
  float *local_1f0;
  undefined8 uStack_1e8;
  undefined4 local_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  float fStack_1cc;
  undefined8 uStack_1c8;
  undefined **local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined1 local_1a8 [16];
  float *local_190;
  longlong *local_188;
  float *pfStack_180;
  longlong local_178;
  longlong lStack_170;
  longlong local_168;
  longlong lStack_160;
  longlong local_158;
  longlong lStack_150;
  longlong local_148;
  undefined4 local_140;
  undefined4 local_13c;
  float *local_138;
  longlong *local_130;
  float *local_128;
  float *pfStack_120;
  float *local_118;
  undefined8 uStack_110;
  float *local_108;
  float *local_100;
  undefined8 **local_f8;
  float *local_f0;
  longlong *local_e8;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puVar5 = (ulonglong *)*in_R8;
  lVar24 = in_R8[1];
  uVar25 = *puVar5;
  uVar21 = 0;
  uVar22 = uVar25 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar25 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar22 = uVar21;
  }
  uVar20 = uVar25 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar25 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar20 = uVar21;
  }
  uVar25 = *(ulonglong *)(lVar24 + 0x30);
  uVar23 = uVar25 - uVar20;
  if (uVar25 < uVar20) {
    uVar23 = uVar21;
  }
  uVar6 = *(ulonglong *)(lVar24 + 0x10);
  uVar18 = uVar6 - uVar22;
  if (uVar6 < uVar22) {
    uVar18 = uVar21;
  }
  uVar21 = *(ulonglong *)(lVar24 + 0x40);
  *puVar5 = uVar21 - (uVar18 + uVar23);
  if (uVar6 >= uVar22 && uVar6 - uVar22 != 0 || uVar25 >= uVar20 && uVar25 - uVar20 != 0) {
    *puVar5 = uVar21;
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_1f0 = (float *)*extraout_RAX;
      uStack_1e8 = extraout_RAX[1];
      local_1e0 = 3;
      uStack_1dc = 0;
      uStack_1d8 = 0x42a96158;
      uStack_1d4 = 1;
      uStack_1d0 = 0x2c;
      fStack_1cc = 0.0;
      local_1c0 = &PTR_s_Converting_pre_placed_trees_142a960f8;
      local_1b8 = 1;
      local_1b0 = 8;
      local_210 = (undefined **)0x0;
      local_1a8 = (undefined1  [16])0x0;
      pfStack_208 = (float *)&DAT_142a96158;
      local_200 = (longlong **)&DAT_0000002c;
      uStack_1f8 = 0;
      uStack_1c8 = CONCAT44(*(undefined4 *)(extraout_RAX + 2),1);
      ::log::__private_api::impl$2::log();
    }
    local_2d0 = *in_RCX;
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::fs::read_to_string::inner();
    if (local_190 != (float *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_e8 = local_298;
    if (SBORROW8(0,(longlong)local_298)) {
      local_190 = pfStack_290;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_100 = pfStack_290;
    serde_json::de::
    from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>_>
              ();
    pfVar27 = pfStack_180;
    plVar17 = local_188;
    pfVar13 = local_190;
    if (SBORROW8(0,(longlong)local_190)) {
      local_298 = local_188;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    if (local_e8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((undefined8 **)local_210 != (undefined8 **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_230 = plVar17 + (longlong)pfVar27 * 10;
    local_248 = plVar17;
    local_238 = (longlong)pfVar13;
    local_13c = *(undefined4 *)((longlong)in_R9 + 0x1c);
    local_100 = (float *)*in_R9;
    local_2b8 = (undefined4 *)in_R9[2];
    local_140 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    local_e8 = (longlong *)*in_stack_00000028;
    local_2c0 = (undefined4 *)in_stack_00000028[2];
    local_2a8 = (longlong)local_e8 + 0x20;
    lVar24 = 0;
    plVar10 = local_230;
    if (plVar17 != local_230) {
      do {
        while( true ) {
          plVar26 = plVar17 + 10;
          lVar7 = *plVar17;
          local_258 = plVar17[9];
          local_268 = plVar17[7];
          lStack_260 = plVar17[8];
          local_278 = plVar17[5];
          lStack_270 = plVar17[6];
          local_288 = plVar17[3];
          lStack_280 = plVar17[4];
          local_298 = (longlong *)plVar17[1];
          pfVar13 = (float *)plVar17[2];
          plVar17 = plVar26;
          pfStack_290 = pfVar13;
          if (SBORROW8(0,lVar7)) goto LAB_140862dd5;
          local_190 = (float *)lVar7;
          local_188 = local_298;
          pfStack_180 = pfVar13;
          local_178 = local_288;
          lStack_170 = lStack_280;
          local_168 = local_278;
          lStack_160 = lStack_270;
          local_158 = local_268;
          lStack_150 = lStack_260;
          local_148 = local_258;
          if ((pfVar13 != (float *)&DAT_00000012) ||
             (auVar28[0] = -((char)*local_298 == 'c'),
             auVar28[1] = -(*(char *)((longlong)local_298 + 1) == 'l'),
             auVar28[2] = -(*(char *)((longlong)local_298 + 2) == 'e'),
             auVar28[3] = -(*(char *)((longlong)local_298 + 3) == 'a'),
             auVar28[4] = -(*(char *)((longlong)local_298 + 4) == 'r'),
             auVar28[5] = -(*(char *)((longlong)local_298 + 5) == 'i'),
             auVar28[6] = -(*(char *)((longlong)local_298 + 6) == 'n'),
             auVar28[7] = -(*(char *)((longlong)local_298 + 7) == 'g'),
             auVar28[8] = -((char)local_298[1] == '/'),
             auVar28[9] = -(*(char *)((longlong)local_298 + 9) == 't'),
             auVar28[10] = -(*(char *)((longlong)local_298 + 10) == 'r'),
             auVar28[0xb] = -(*(char *)((longlong)local_298 + 0xb) == 'e'),
             auVar28[0xc] = -(*(char *)((longlong)local_298 + 0xc) == 'e'),
             auVar28[0xd] = -(*(char *)((longlong)local_298 + 0xd) == '/'),
             auVar28[0xe] = -(*(char *)((longlong)local_298 + 0xe) == 'b'),
             auVar28[0xf] = -(*(char *)((longlong)local_298 + 0xf) == 'u'),
             auVar29[0] = -((char)(short)local_298[2] == 's'),
             auVar29[1] = -((char)((ushort)(short)local_298[2] >> 8) == 'h'), auVar29[2] = 0xff,
             auVar29[3] = 0xff, auVar29[4] = 0xff, auVar29[5] = 0xff, auVar29[6] = 0xff,
             auVar29[7] = 0xff, auVar29[8] = 0xff, auVar29[9] = 0xff, auVar29[10] = 0xff,
             auVar29[0xb] = 0xff, auVar29[0xc] = 0xff, auVar29[0xd] = 0xff, auVar29[0xe] = 0xff,
             auVar29[0xf] = 0xff, auVar29 = auVar29 & auVar28,
             (ushort)((ushort)(SUB161(auVar29 >> 7,0) & 1) |
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
                     (ushort)(byte)(auVar29[0xf] >> 7) << 0xf) != 0xffff)) break;
          if (lVar7 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_178 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if ((!SBORROW8(0,lStack_160)) && (lStack_160 != 0)) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (plVar26 == plVar10) goto LAB_140862dd5;
        }
        local_240 = plVar26;
        local_130 = local_298;
        std::path::Path::_join();
        relative_path::RelativePath::to_path<std::path::PathBuf>();
        local_2d8 = local_338;
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::fs::read_to_string::inner();
        local_228 = (longlong *)local_128;
        if (SBORROW8(0,(longlong)local_128)) {
          local_2e0 = pfStack_120;
          local_f8 = &local_2d8;
          local_f0 = (float *)core::fmt::impl$74::fmt<std::path::PathBuf>;
          local_210 = &PTR_s_Couldn_t_find_142a96250;
          pfStack_208 = (float *)0x1;
          local_1f0 = (float *)0x0;
          local_200 = (longlong **)&local_f8;
          uStack_1f8 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        local_218 = pfStack_120;
        serde_json::de::
        from_str<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>_>
                  ();
        pplVar14 = local_200;
        local_220 = (longlong *)local_210;
        if (SBORROW8(0,(longlong)local_210)) {
          local_128 = pfStack_208;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        local_138 = pfStack_208;
        asset::prefab::PrefabName::new();
        local_130 = (longlong *)CONCAT44(local_130._4_4_,(undefined4)local_148);
        pfVar19 = local_138 + (longlong)pplVar14 * 0xc;
        pfVar27 = local_138;
        local_f8 = extraout_RAX_00;
        local_f0 = pfVar13;
LAB_140862870:
        pfVar13 = pfVar27;
        if (pfVar13 != pfVar19) {
          pfVar27 = pfVar13 + 0xc;
          if ((ABS(*pfVar13) < 65.0) && (ABS(pfVar13[2]) < 65.0)) {
            bevy_ecs::system::commands::Commands::spawn_empty();
            pfVar16 = local_1f0;
            pplVar14 = (longlong **)pfStack_208;
            if (pfStack_208 == (float *)0x0) {
              pplVar14 = local_200;
            }
            lVar7 = *(longlong *)((longlong)pplVar14 + 0x10);
            if ((ulonglong)((longlong)*pplVar14 - lVar7) < 0x30) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            }
            local_2b0 = lVar24 + 0x186a1;
            lVar8 = *(longlong *)((longlong)pplVar14 + 8);
            *(code **)(lVar8 + lVar7) =
                 core::ops::function::FnOnce::
                 call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeComponent>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
            ;
            *(code **)(lVar8 + 8 + lVar7) = bevy_ecs::error::handler::panic;
            *(float **)(lVar8 + 0x10 + lVar7) = pfVar16;
            *(longlong *)(lVar8 + 0x18 + lVar7) = local_2b0;
            *(undefined1 *)(lVar8 + 0x20 + lVar7) = 1;
            *(undefined1 *)(lVar8 + 0x28 + lVar7) = 0;
            *(longlong *)((longlong)pplVar14 + 0x10) = lVar7 + 0x30;
            local_108 = local_1f0;
            local_118 = (float *)local_200;
            uStack_110 = uStack_1f8;
            local_128 = (float *)local_210;
            pfStack_120 = pfStack_208;
            LOCK();
            puVar3 = *local_f8;
            *local_f8 = (undefined8 *)((longlong)*local_f8 + 1);
            UNLOCK();
            if (*local_f8 != (undefined8 *)0x0 &&
                SCARRY8((longlong)puVar3,1) == (longlong)*local_f8 < 0) {
              bevy_ecs::system::commands::EntityCommands::
              insert<country_core::components::ConvertedPreplacedTree>();
              pfVar16 = local_108;
              pfVar15 = pfStack_120;
              if (pfStack_120 == (float *)0x0) {
                pfVar15 = local_118;
              }
              lVar7 = *(longlong *)(pfVar15 + 4);
              if ((ulonglong)(*(longlong *)pfVar15 - lVar7) < 0x30) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              pfVar11 = local_108;
              lVar8 = *(longlong *)(pfVar15 + 2);
              *(code **)(lVar8 + lVar7) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar8 + 8 + lVar7) = bevy_ecs::error::handler::panic;
              *(float **)(lVar8 + 0x10 + lVar7) = pfVar16;
              *(undefined8 *)(lVar8 + 0x18 + lVar7) = 0x3f8000003f400000;
              *(undefined4 *)(lVar8 + 0x20 + lVar7) = 0x3f800000;
              *(undefined1 *)(lVar8 + 0x24 + lVar7) = 0;
              *(undefined1 *)(lVar8 + 0x28 + lVar7) = 0;
              *(longlong *)(pfVar15 + 4) = lVar7 + 0x30;
              pfVar16 = pfStack_120;
              if (pfStack_120 == (float *)0x0) {
                pfVar16 = local_118;
              }
              fVar1 = *pfVar13;
              fVar2 = pfVar13[2];
              lVar7 = *(longlong *)(pfVar16 + 4);
              if ((ulonglong)(*(longlong *)pfVar16 - lVar7) < 0x28) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              pfVar15 = local_108;
              lVar8 = *(longlong *)(pfVar16 + 2);
              *(code **)(lVar8 + lVar7) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMoveBend>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar8 + 8 + lVar7) = bevy_ecs::error::handler::panic;
              *(float **)(lVar8 + 0x10 + lVar7) = pfVar11;
              *(float *)(lVar8 + 0x18 + lVar7) = fVar1;
              *(float *)(lVar8 + 0x1c + lVar7) = fVar2;
              *(undefined1 *)(lVar8 + 0x20 + lVar7) = 0;
              *(longlong *)(pfVar16 + 4) = lVar7 + 0x28;
              uVar4 = *(undefined8 *)pfVar13;
              pfVar16 = pfStack_120;
              if (pfStack_120 == (float *)0x0) {
                pfVar16 = local_118;
              }
              fVar1 = pfVar13[2];
              lVar7 = *(longlong *)(pfVar16 + 4);
              if ((ulonglong)(*(longlong *)pfVar16 - lVar7) < 0x28) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              lVar8 = *(longlong *)(pfVar16 + 2);
              *(code **)(lVar8 + lVar7) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAmbience>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar8 + 8 + lVar7) = bevy_ecs::error::handler::panic;
              *(float **)(lVar8 + 0x10 + lVar7) = pfVar15;
              *(ulonglong *)(lVar8 + 0x18 + lVar7) =
                   CONCAT44((float)((ulonglong)uVar4 >> 0x20) + 10.0,(float)uVar4 + 0.0);
              *(float *)(lVar8 + 0x20 + lVar7) = fVar1 + 0.0;
              *(undefined1 *)(lVar8 + 0x24 + lVar7) = 0;
              *(longlong *)(pfVar16 + 4) = lVar7 + 0x28;
              local_318 = *(undefined8 *)pfVar13;
              pfStack_310 = *(float **)(pfVar13 + 2);
              local_308 = *(undefined8 *)(pfVar13 + 4);
              local_2f8 = *(undefined8 *)(pfVar13 + 7);
              uStack_2f0 = *(undefined8 *)(pfVar13 + 9);
              local_2e8 = pfVar13[0xb];
              LOCK();
              puVar3 = *local_f8;
              *local_f8 = (undefined8 *)((longlong)*local_f8 + 1);
              UNLOCK();
              if (*local_f8 != (undefined8 *)0x0 &&
                  SCARRY8((longlong)puVar3,1) == (longlong)*local_f8 < 0) {
                *local_2c0 = local_140;
                uStack_1c8 = *(undefined8 *)((longlong)local_e8 + 0x40);
                local_2a0 = (longlong *)local_f8;
                local_210 = (undefined **)local_f8;
                pfStack_208 = local_f0;
                local_200 = (longlong **)local_108;
                local_1e0 = local_130._0_4_;
                uStack_1dc = (undefined4)local_2f8;
                uStack_1d8 = (undefined4)((ulonglong)local_2f8 >> 0x20);
                uStack_1d4 = (undefined4)uStack_2f0;
                uStack_1d0 = (undefined4)((ulonglong)uStack_2f0 >> 0x20);
                lVar7 = *(longlong *)((longlong)local_e8 + 0x30);
                uStack_1f8 = local_318;
                local_1f0 = pfStack_310;
                uStack_1e8 = local_308;
                fStack_1cc = local_2e8;
                if (lVar7 == *(longlong *)((longlong)local_e8 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
                }
                lVar8 = *(longlong *)((longlong)local_e8 + 0x28);
                lVar12 = lVar7 * 0x50;
                puVar3 = (undefined8 *)(lVar8 + 0x40 + lVar12);
                *puVar3 = CONCAT44(fStack_1cc,uStack_1d0);
                puVar3[1] = uStack_1c8;
                puVar3 = (undefined8 *)(lVar8 + 0x30 + lVar12);
                *puVar3 = CONCAT44(uStack_1dc,local_1e0);
                puVar3[1] = CONCAT44(uStack_1d4,uStack_1d8);
                puVar3 = (undefined8 *)(lVar8 + 0x20 + lVar12);
                *puVar3 = local_1f0;
                puVar3[1] = uStack_1e8;
                puVar3 = (undefined8 *)(lVar8 + 0x10 + lVar12);
                *puVar3 = local_200;
                puVar3[1] = uStack_1f8;
                *(undefined8 *)(lVar8 + lVar12) = local_210;
                ((undefined8 *)(lVar8 + lVar12))[1] = pfStack_208;
                *(longlong *)((longlong)local_e8 + 0x30) = lVar7 + 1;
                *(longlong *)((longlong)local_e8 + 0x40) =
                     *(longlong *)((longlong)local_e8 + 0x40) + 1;
                *local_2b8 = local_13c;
                hashbrown::map::HashMap::
                insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                          ();
                uVar25 = lVar24 + 0x186a2U;
                if (lVar24 + 0x186a2U <= *(ulonglong *)((longlong)local_100 + 0x30)) {
                  uVar25 = *(ulonglong *)((longlong)local_100 + 0x30);
                }
                *(ulonglong *)((longlong)local_100 + 0x30) = uVar25;
                lVar24 = lVar24 + 1;
                if (extraout_RAX_01 >> 0x20 != 0) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic();
                }
                goto LAB_140862870;
              }
            }
                    // WARNING: Does not return
            pcVar9 = (code *)invalidInstructionException();
            (*pcVar9)();
          }
          goto LAB_140862870;
        }
        LOCK();
        *local_f8 = (undefined8 *)((longlong)*local_f8 + -1);
        UNLOCK();
        if (*local_f8 == (undefined8 *)0x0) {
          alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
        }
        if ((undefined8 **)local_220 != (undefined8 **)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if ((undefined8 **)local_228 != (undefined8 **)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_338[0] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_190 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_178 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if ((!SBORROW8(0,lStack_160)) && (lStack_160 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar17 = local_240;
        plVar10 = local_230;
      } while (local_240 != local_230);
    }
LAB_140862dd5:
    local_240 = plVar17;
    core::ptr::
    drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>_>
              ();
  }
  return;
}

