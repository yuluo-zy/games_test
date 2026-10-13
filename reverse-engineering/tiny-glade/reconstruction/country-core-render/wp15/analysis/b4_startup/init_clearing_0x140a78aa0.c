// addr=0x140a78aa0 name=init_clearing
// signature=void __cdecl init_clearing(void)


void __cdecl country_core::startup::clearing::init_clearing(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  undefined4 *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  char cVar10;
  undefined8 *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong uVar11;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 *in_RCX;
  longlong **pplVar12;
  longlong lVar13;
  longlong ***ppplVar14;
  longlong *plVar15;
  longlong **pplVar16;
  longlong **pplVar17;
  ulonglong uVar18;
  longlong *in_R9;
  longlong **pplVar19;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  longlong in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000058;
  longlong local_290;
  longlong **local_288;
  longlong **local_280;
  longlong *local_238;
  longlong local_230;
  undefined4 local_228;
  undefined4 local_224;
  longlong *local_220;
  longlong *plStack_218;
  longlong *local_210;
  longlong *local_208;
  longlong local_200;
  longlong local_1f8;
  longlong *local_1f0;
  int local_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 local_1d8;
  undefined4 uStack_1d4;
  undefined8 *puStack_1d0;
  undefined8 *local_1c8;
  undefined8 *puStack_1c0;
  undefined8 *local_1b8;
  undefined8 *puStack_1b0;
  undefined8 *local_1a8;
  undefined8 *puStack_1a0;
  undefined *local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  ulonglong local_170;
  longlong local_168;
  ulonglong local_160;
  longlong local_158;
  longlong **local_148;
  longlong **local_140;
  longlong local_138;
  ulonglong local_128;
  longlong local_120;
  longlong **local_118;
  longlong ***ppplStack_110;
  longlong ***local_108;
  undefined8 *puStack_100;
  undefined8 *local_f8;
  undefined8 *puStack_f0;
  undefined8 *local_e8;
  undefined8 *puStack_e0;
  undefined8 *local_d8;
  undefined8 uStack_d0;
  undefined *local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined1 auStack_b0 [16];
  longlong local_98;
  longlong **local_90;
  longlong **local_88;
  longlong **local_80;
  undefined1 local_74;
  char local_73;
  undefined1 local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_f8 = (undefined8 *)*extraout_RAX;
    puStack_f0 = (undefined8 *)extraout_RAX[1];
    local_e8 = (undefined8 *)&DAT_00000004;
    puStack_e0 = &DAT_142ad6d58;
    local_d8 = (undefined8 *)&DAT_0000001f;
    local_c8 = &DAT_142ad6d00;
    uStack_c0 = 1;
    local_b8 = 8;
    local_118 = (longlong **)0x0;
    auStack_b0 = (undefined1  [16])0x0;
    ppplStack_110 = (longlong ***)&DAT_142ad6d58;
    local_108 = (longlong ***)&DAT_0000001f;
    puStack_100 = (undefined8 *)0x0;
    uStack_d0 = (undefined8 *)CONCAT44(*(undefined4 *)(extraout_RAX + 2),1);
    ::log::__private_api::impl$2::log();
  }
  local_98 = *(longlong *)*in_RCX + 0x10;
  cVar10 = *(char *)(*in_R9 + 0x58);
  local_1f0 = in_R9;
  if (cVar10 == '\x04') {
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    uStack_1e0 = SUB84(ppplStack_110,0);
    uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
    if (SBORROW8(0,(longlong)local_118)) {
      local_1e8 = uStack_1e0;
      uStack_1e4 = uStack_1dc;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_178 = auStack_b0._8_8_;
    local_188 = local_b8;
    uStack_180 = auStack_b0._0_8_;
    local_198 = local_c8;
    uStack_190 = uStack_c0;
    local_1a8 = local_d8;
    puStack_1a0 = uStack_d0;
    local_1b8 = local_e8;
    puStack_1b0 = puStack_e0;
    local_1c8 = local_f8;
    puStack_1c0 = puStack_f0;
    local_1d8 = SUB84(local_108,0);
    uStack_1d4 = (undefined4)((ulonglong)local_108 >> 0x20);
    puStack_1d0 = puStack_100;
    local_1e8 = (int)local_118;
    uStack_1e4 = (undefined4)((ulonglong)local_118 >> 0x20);
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    puVar1 = local_f8;
    ppplVar14 = ppplStack_110;
    if (ppplStack_110 == (longlong ***)0x0) {
      ppplVar14 = local_108;
    }
    plVar15 = (longlong *)ppplVar14[2];
    if ((ulonglong)((longlong)*ppplVar14 - (longlong)plVar15) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    plVar2 = (longlong *)ppplVar14[1];
    *(code **)((longlong)plVar2 + (longlong)plVar15) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)((longlong)(plVar2 + 1) + (longlong)plVar15) = 0;
    ((undefined8 *)((longlong)(plVar2 + 1) + (longlong)plVar15))[1] = 0x3f80000000000000;
    *(undefined8 *)((longlong)(plVar2 + 3) + (longlong)plVar15) = 0;
    *(undefined4 *)((longlong)(plVar2 + 4) + (longlong)plVar15) = 0;
    *(undefined8 *)((longlong)plVar2 + 0x24 + (longlong)plVar15) = 0x3f8000003f800000;
    *(undefined4 *)((longlong)plVar2 + 0x2c + (longlong)plVar15) = 0x3f800000;
    *(undefined8 *)((longlong)(plVar2 + 7) + (longlong)plVar15) = extraout_RAX_00;
    *(undefined8 *)((longlong)(plVar2 + 8) + (longlong)plVar15) = extraout_RAX_01;
    *(undefined1 *)((longlong)(plVar2 + 9) + (longlong)plVar15) = 0;
    *(undefined8 **)((longlong)(plVar2 + 0xb) + (longlong)plVar15) = puVar1;
    *(code **)((longlong)(plVar2 + 0xd) + (longlong)plVar15) = bevy_ecs::error::handler::panic;
    ppplVar14[2] = (longlong **)(plVar15 + 0xf);
    cVar10 = *(char *)(*local_1f0 + 0x58);
  }
  if (cVar10 == '\x01') {
    **(undefined4 **)(in_stack_00000040 + 0x10) = *(undefined4 *)(in_stack_00000040 + 0x1c);
    glade_themes::olden::startup_olden();
    bevy_ecs::system::commands::Commands::
    insert_resource<country_core::resources::preplaced_autoclutter::PreplacedAutoClutter>();
    cVar10 = *(char *)(*local_1f0 + 0x58);
  }
  else {
    if (cVar10 == '\x02') {
      glade_themes::flowery::startup_flowery();
    }
    cVar10 = *(char *)(*local_1f0 + 0x58);
  }
  if (cVar10 != '\x01') {
    plVar15 = *(longlong **)(in_stack_00000038 + 8);
    if (plVar15 == (longlong *)0x0) {
      plVar15 = *(longlong **)(in_stack_00000038 + 0x10);
    }
    lVar4 = plVar15[2];
    if ((ulonglong)(*plVar15 - lVar4) < 8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    *(code **)(plVar15[1] + lVar4) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::remove_resource::closure_env_0<country_core::resources::preplaced_autoclutter::PreplacedAutoClutter>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar15[2] = lVar4 + 8;
  }
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (CONCAT44(uStack_1e4,local_1e8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_80 = local_148;
  if (SBORROW8(0,(longlong)local_148)) {
    local_1e8 = (int)local_140;
    uStack_1e4 = (undefined4)((ulonglong)local_140 >> 0x20);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_90 = local_140;
  serde_json::de::
  from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>_>
            ();
  plVar15 = local_220;
  if (SBORROW8(0,(longlong)local_220)) {
    local_1e8 = (int)plStack_218;
    uStack_1e4 = (undefined4)((ulonglong)plStack_218 >> 0x20);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (local_80 != (longlong **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_118 != (longlong **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_170 = 0;
  local_168 = 4;
  local_160 = 0;
  local_224 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  puVar3 = (undefined4 *)in_stack_00000058[2];
  *puVar3 = local_224;
  lVar4 = *in_stack_00000058;
  pplVar16 = *(longlong ***)(lVar4 + 8);
  pplVar19 = *(longlong ***)(lVar4 + 0x10);
  *(undefined8 *)(lVar4 + 0x10) = 0;
  local_90 = pplVar16;
  local_88 = pplVar19;
  if (pplVar19 != (longlong **)0x0) {
    pplVar12 = (longlong **)0x1;
    do {
      plVar2 = *pplVar16;
      LOCK();
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      local_80 = pplVar12;
      if (*plVar2 == 0) {
        alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
      pplVar12 = (longlong **)((longlong)local_80 + 1);
      pplVar16 = pplVar16 + 10;
    } while (local_80 != pplVar19);
  }
  *(undefined1 *)(lVar4 + 0x18) = 0;
  local_208 = plStack_218 + (longlong)local_210 * 10;
  local_220 = plStack_218;
  local_210 = plVar15;
  while (plVar15 = plStack_218, plStack_218 != local_208) {
    plVar15 = plStack_218 + 10;
    lVar4 = *plStack_218;
    if (SBORROW8(0,lVar4)) break;
    local_88 = (longlong **)plStack_218[1];
    lVar13 = plStack_218[2];
    local_200 = plStack_218[3];
    local_158 = plStack_218[4];
    uVar18 = plStack_218[6];
    local_120 = plStack_218[7];
    lVar7 = plStack_218[9];
    plStack_218 = plVar15;
    local_1f8 = lVar4;
    if ((lVar13 == 0x12) &&
       (auVar20[0] = -(*(char *)local_88 == 'c'),
       auVar20[1] = -(*(char *)((longlong)local_88 + 1) == 'l'),
       auVar20[2] = -(*(char *)((longlong)local_88 + 2) == 'e'),
       auVar20[3] = -(*(char *)((longlong)local_88 + 3) == 'a'),
       auVar20[4] = -(*(char *)((longlong)local_88 + 4) == 'r'),
       auVar20[5] = -(*(char *)((longlong)local_88 + 5) == 'i'),
       auVar20[6] = -(*(char *)((longlong)local_88 + 6) == 'n'),
       auVar20[7] = -(*(char *)((longlong)local_88 + 7) == 'g'),
       auVar20[8] = -(*(char *)(local_88 + 1) == '/'),
       auVar20[9] = -(*(char *)((longlong)local_88 + 9) == 't'),
       auVar20[10] = -(*(char *)((longlong)local_88 + 10) == 'r'),
       auVar20[0xb] = -(*(char *)((longlong)local_88 + 0xb) == 'e'),
       auVar20[0xc] = -(*(char *)((longlong)local_88 + 0xc) == 'e'),
       auVar20[0xd] = -(*(char *)((longlong)local_88 + 0xd) == '/'),
       auVar20[0xe] = -(*(char *)((longlong)local_88 + 0xe) == 'b'),
       auVar20[0xf] = -(*(char *)((longlong)local_88 + 0xf) == 'u'),
       auVar21[0] = -((char)*(undefined2 *)(local_88 + 2) == 's'),
       auVar21[1] = -((char)((ushort)*(undefined2 *)(local_88 + 2) >> 8) == 'h'), auVar21[2] = 0xff,
       auVar21[3] = 0xff, auVar21[4] = 0xff, auVar21[5] = 0xff, auVar21[6] = 0xff, auVar21[7] = 0xff
       , auVar21[8] = 0xff, auVar21[9] = 0xff, auVar21[10] = 0xff, auVar21[0xb] = 0xff,
       auVar21[0xc] = 0xff, auVar21[0xd] = 0xff, auVar21[0xe] = 0xff, auVar21[0xf] = 0xff,
       auVar21 = auVar21 & auVar20,
       (ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar21[0xf] >> 7) << 0xf) == 0xffff)) {
      if (lVar4 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_200 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_140a79630;
    }
    **(undefined4 **)(in_stack_00000040 + 0x10) = *(undefined4 *)(in_stack_00000040 + 0x1c);
    local_72 = 1;
    local_128 = uVar18;
    std::path::Path::_join();
    local_72 = 1;
    relative_path::RelativePath::to_path<std::path::PathBuf>();
    local_72 = 1;
    asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>();
    local_73 = '\x01';
    asset::prefab::PrefabName::new();
    *puVar3 = local_224;
    ppplStack_110 = (longlong ***)(local_140 + local_138 * 6);
    local_118 = local_140;
    local_108 = (longlong ***)&local_238;
    puStack_100 = (undefined8 *)&local_228;
    local_74 = 1;
    local_238 = extraout_RAX_02;
    local_230 = lVar13;
    local_228 = (int)lVar7;
    alloc::vec::spec_extend::impl$0::
    spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env_0>,alloc::alloc::Global>
              ();
    cVar10 = '\x01';
    if (SBORROW8(0,local_128)) {
LAB_140a79557:
      LOCK();
      *local_238 = *local_238 + -1;
      lVar4 = *local_238;
      UNLOCK();
    }
    else {
      relative_path::RelativePath::to_path<ref_<std::path::Path>_>();
      country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>();
      uVar9 = uStack_1dc;
      uVar8 = uStack_1e0;
      uStack_1e0 = SUB84(ppplStack_110,0);
      uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
      if (SBORROW8(0,(longlong)local_118)) {
        local_1e8 = uStack_1e0;
        uStack_1e4 = uStack_1dc;
        uStack_1e0 = uVar8;
        uStack_1dc = uVar9;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_178 = auStack_b0._8_8_;
      local_188 = local_b8;
      uStack_180 = auStack_b0._0_8_;
      local_198 = local_c8;
      uStack_190 = uStack_c0;
      local_1a8 = local_d8;
      puStack_1a0 = uStack_d0;
      local_1b8 = local_e8;
      puStack_1b0 = puStack_e0;
      local_1c8 = local_f8;
      puStack_1c0 = puStack_f0;
      local_1d8 = SUB84(local_108,0);
      uStack_1d4 = (undefined4)((ulonglong)local_108 >> 0x20);
      puStack_1d0 = puStack_100;
      local_1e8 = (int)local_118;
      uStack_1e4 = (undefined4)((ulonglong)local_118 >> 0x20);
      country_core::render::mesh::Mesh::positions_without_indices();
      alloc::vec::spec_from_iter::impl$0::
      from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
                ();
      core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
      pplVar12 = local_140 + local_138 * 6;
      local_80 = local_280;
      local_90 = local_288;
      pplVar16 = (longlong **)((longlong)local_288 + (longlong)local_280 * 0xc);
      for (pplVar19 = local_140; pplVar19 != pplVar12; pplVar19 = pplVar19 + 6) {
        pplVar17 = local_90;
        ppplVar14 = (longlong ***)pplVar16;
        ppplStack_110 = (longlong ***)pplVar16;
        local_108 = (longlong ***)pplVar19;
        if (local_80 != (longlong **)0x0) {
          do {
            while( true ) {
              pplVar17 = (longlong **)((longlong)pplVar17 + 0xc);
              local_118 = pplVar17;
              core::ops::function::impls::impl$3::
              call_mut<tuple_<ref_<glam::f32::vec3::Vec3>_>,country_core::startup::clearing::init_clearing::closure_env_1>
                        ();
              uVar18 = local_160;
              if (local_1e8 == 1) break;
              if ((longlong ***)pplVar17 == ppplVar14) goto LAB_140a79410;
            }
            uVar5 = CONCAT44(uStack_1e0,uStack_1e4);
            uVar6 = CONCAT44(local_1d8,uStack_1dc);
            if (local_160 == local_170) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            }
            puVar1 = (undefined8 *)(local_168 + uVar18 * 0x10);
            *puVar1 = uVar5;
            puVar1[1] = uVar6;
            local_160 = uVar18 + 1;
            pplVar17 = local_118;
            ppplVar14 = ppplStack_110;
          } while ((longlong ***)local_118 != ppplStack_110);
        }
LAB_140a79410:
      }
      if (local_290 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_128 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        cVar10 = '\0';
        goto LAB_140a79557;
      }
      cVar10 = '\0';
      LOCK();
      *local_238 = *local_238 + -1;
      lVar4 = *local_238;
      UNLOCK();
    }
    if (lVar4 == 0) {
      local_73 = cVar10;
      alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
    }
    if (local_148 != (longlong **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_1f8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_200 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar18 = local_128;
    if (cVar10 != '\0') {
LAB_140a79630:
      if ((uVar18 & 0x7fffffffffffffff) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  plStack_218 = plVar15;
  local_71 = 0;
  core::ptr::
  drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>_>
            ();
  **(undefined4 **)(in_stack_00000048 + 0x10) = *(undefined4 *)(in_stack_00000048 + 0x1c);
  local_71 = 0;
  asset_libraries::ssbo_library::AssetSsboLibrary::
  get_mut<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>()
  ;
  uVar18 = local_160;
  if ((extraout_RAX_03 != 0) && (local_160 != 0)) {
    if (*(longlong ***)(extraout_RAX_03 + 0x18) < (longlong **)(local_160 << 4)) {
      local_118 = (longlong **)&DAT_0000000b;
    }
    else {
      local_148 = (longlong **)0x0;
      local_71 = 0;
      local_140 = (longlong **)(local_160 << 4);
      rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
      if (local_118 == (longlong **)&DAT_0000000b) goto LAB_140a79739;
      local_1a8 = uStack_d0;
      local_1b8 = puStack_e0;
      puStack_1b0 = local_d8;
      local_1c8 = puStack_f0;
      puStack_1c0 = local_e8;
      local_1d8 = SUB84(puStack_100,0);
      uStack_1d4 = (undefined4)((ulonglong)puStack_100 >> 0x20);
      puStack_1d0 = local_f8;
      local_1e8 = (int)ppplStack_110;
      uStack_1e4 = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
      uStack_1e0 = SUB84(local_108,0);
      uStack_1dc = (undefined4)((ulonglong)local_108 >> 0x20);
    }
    ppplStack_110 = (longlong ***)CONCAT44(uStack_1e4,local_1e8);
    local_108 = (longlong ***)CONCAT44(uStack_1dc,uStack_1e0);
    puStack_100 = (undefined8 *)CONCAT44(uStack_1d4,local_1d8);
    local_f8 = puStack_1d0;
    puStack_f0 = local_1c8;
    local_e8 = puStack_1c0;
    puStack_e0 = local_1b8;
    local_d8 = puStack_1b0;
    uStack_d0 = local_1a8;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
LAB_140a79739:
  uVar11 = 0x44c;
  if (uVar18 < 0x44c) {
    uVar11 = uVar18;
  }
  *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
  *(ulonglong *)*in_stack_00000050 = uVar11;
  local_220 = (longlong *)&DAT_142ad6e60;
  plStack_218 = (longlong *)0x11;
  local_140 = (longlong **)
              anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_118 = (longlong **)&PTR_s__clearing__raster_hlsl_142ad6ca0;
  ppplStack_110 = (longlong ***)0x2;
  local_108 = &local_148;
  puStack_100 = (undefined8 *)0x1;
  local_f8 = (undefined8 *)0x0;
  local_71 = 0;
  local_148 = &local_220;
  alloc::fmt::format::format_inner();
  local_80 = (longlong **)CONCAT44(uStack_1dc,uStack_1e0);
  local_140 = (longlong **)
              anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_118 = (longlong **)&DAT_142ad6cc8;
  ppplStack_110 = (longlong ***)0x2;
  local_108 = &local_148;
  puStack_100 = (undefined8 *)0x1;
  local_f8 = (undefined8 *)0x0;
  local_148 = &local_220;
  alloc::fmt::format::format_inner();
  load_shader_into_library<alloc::string::String>();
  if (CONCAT44(uStack_1e4,local_1e8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_71 = 0;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  uVar9 = uStack_1dc;
  uVar8 = uStack_1e0;
  uStack_1e0 = SUB84(ppplStack_110,0);
  uStack_1dc = (undefined4)((ulonglong)ppplStack_110 >> 0x20);
  if (SBORROW8(0,(longlong)local_118)) {
    local_1e8 = uStack_1e0;
    uStack_1e4 = uStack_1dc;
    uStack_1e0 = uVar8;
    uStack_1dc = uVar9;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_178 = auStack_b0._8_8_;
  local_188 = local_b8;
  uStack_180 = auStack_b0._0_8_;
  local_198 = local_c8;
  uStack_190 = uStack_c0;
  local_1a8 = local_d8;
  puStack_1a0 = uStack_d0;
  local_1b8 = local_e8;
  puStack_1b0 = puStack_e0;
  local_1c8 = local_f8;
  puStack_1c0 = puStack_f0;
  local_1d8 = SUB84(local_108,0);
  uStack_1d4 = (undefined4)((ulonglong)local_108 >> 0x20);
  puStack_1d0 = puStack_100;
  local_1e8 = (int)local_118;
  uStack_1e4 = (undefined4)((ulonglong)local_118 >> 0x20);
  local_71 = 0;
  load_mesh_into_library();
  local_71 = 0;
  bevy_ecs::system::commands::Commands::spawn_empty();
  puVar1 = local_f8;
  ppplVar14 = ppplStack_110;
  if (ppplStack_110 == (longlong ***)0x0) {
    ppplVar14 = local_108;
  }
  pplVar16 = ppplVar14[2];
  if ((ulonglong)((longlong)*ppplVar14 - (longlong)pplVar16) < 0x78) {
    local_71 = 0;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  pplVar19 = ppplVar14[1];
  *(code **)((longlong)pplVar19 + (longlong)pplVar16) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)((longlong)(pplVar19 + 1) + (longlong)pplVar16) = 0;
  ((undefined8 *)((longlong)(pplVar19 + 1) + (longlong)pplVar16))[1] = 0x3f80000000000000;
  *(undefined8 *)((longlong)(pplVar19 + 3) + (longlong)pplVar16) = 0;
  *(undefined4 *)((longlong)(pplVar19 + 4) + (longlong)pplVar16) = 0;
  *(undefined8 *)((longlong)pplVar19 + 0x24 + (longlong)pplVar16) = 0x3f8000003f800000;
  *(undefined4 *)((longlong)pplVar19 + 0x2c + (longlong)pplVar16) = 0x3f800000;
  *(undefined8 *)((longlong)(pplVar19 + 7) + (longlong)pplVar16) = extraout_RAX_05;
  *(undefined8 *)((longlong)(pplVar19 + 8) + (longlong)pplVar16) = extraout_RAX_04;
  *(undefined1 *)((longlong)(pplVar19 + 9) + (longlong)pplVar16) = 0;
  *(undefined8 **)((longlong)(pplVar19 + 0xb) + (longlong)pplVar16) = puVar1;
  *(code **)((longlong)(pplVar19 + 0xd) + (longlong)pplVar16) = bevy_ecs::error::handler::panic;
  ppplVar14[2] = pplVar16 + 0xf;
  if (local_170 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


