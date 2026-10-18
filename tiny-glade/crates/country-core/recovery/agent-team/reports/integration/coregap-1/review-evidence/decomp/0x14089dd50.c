
// WARNING: Type propagation algorithm not settling

void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::build(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong *plVar8;
  code *pcVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined ****ppppuVar12;
  ulonglong extraout_RAX;
  undefined8 ***extraout_RAX_00;
  undefined8 ***pppuVar13;
  undefined8 ***extraout_RAX_01;
  undefined8 ***extraout_RAX_02;
  longlong *extraout_RAX_03;
  undefined8 ***extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong *in_RCX;
  undefined8 *in_RDX;
  undefined8 uVar14;
  longlong lVar15;
  longlong in_R8;
  longlong lVar16;
  longlong lVar17;
  undefined *****pppppuVar18;
  undefined4 uVar19;
  undefined ****ppppuStack_270;
  longlong *plStack_268;
  longlong *plStack_260;
  undefined *puStack_258;
  undefined ****ppppuStack_250;
  longlong *plStack_248;
  longlong *plStack_240;
  undefined *puStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  longlong local_218;
  longlong lStack_210;
  undefined *puStack_208;
  undefined8 uStack_200;
  undefined8 ***pppuStack_1f8;
  undefined8 uStack_1f0;
  undefined ****ppppuStack_1e8;
  code *pcStack_1e0;
  longlong *plStack_1d8;
  undefined *puStack_1d0;
  undefined ****ppppuStack_1c8;
  longlong *plStack_1c0;
  longlong *plStack_1b8;
  undefined *puStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  longlong lStack_190;
  longlong lStack_188;
  undefined *puStack_180;
  undefined8 uStack_178;
  longlong *local_170;
  longlong local_168;
  undefined8 uStack_160;
  undefined ****ppppuStack_158;
  longlong *plStack_150;
  undefined *****pppppuStack_148;
  undefined *puStack_140;
  undefined ****ppppuStack_138;
  longlong *plStack_130;
  longlong *plStack_128;
  undefined *puStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 *local_c0;
  undefined8 ***pppuStack_b8;
  ulonglong uStack_b0;
  undefined8 ***pppuStack_a8;
  ulonglong uStack_a0;
  undefined1 uStack_92;
  undefined1 uStack_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  local_170 = in_RCX;
  local_168 = in_R8;
  local_c0 = in_RDX;
  switch(in_RDX[2] ^ 0x8000000000000000) {
  case 0:
    uStack_a0 = in_RDX[3];
    pppuVar13 = (undefined8 ***)in_RDX[4];
    uStack_1f0 = in_RDX[5];
    pppuStack_1f8 = pppuVar13;
    pppuStack_a8 = pppuVar13;
    asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name();
    pppuStack_b8 = pppuVar13;
    if ((extraout_RAX & 1) == 0) {
      ppppuStack_1e8 = (undefined ****)&pppuStack_1f8;
      pcStack_1e0 = core::fmt::impl$74::fmt<str_>;
      ppppuStack_158 =
           (undefined ****)&anon_03061c64e98a3a4ea7442d952ee9ce0d_45_llvm_2292516692454265093;
      plStack_150 = (longlong *)0x2;
      ppppuStack_138 = (undefined ****)0x0;
      pppppuStack_148 = &ppppuStack_1e8;
      puStack_140 = (undefined *)0x1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (uStack_a0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uStack_a0 = uStack_a0 & 0xffffffff00000000;
    goto LAB_14089e296;
  case 1:
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    lStack_100 = *extraout_RAX_03;
    lStack_f8 = extraout_RAX_03[1];
    *extraout_RAX_03 = *extraout_RAX_03 + 1;
    puStack_1d0 = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
    ppppuStack_1c8 = (undefined ****)0x0;
    plStack_1c0 = (longlong *)0x0;
    plStack_1b8 = (longlong *)0x0;
    ppppuStack_158 = (undefined ****)0x0;
    plStack_150 = (longlong *)&DAT_00000004;
    pppppuStack_148 = (undefined *****)0x0;
    puStack_140 = &DAT_8000000000000000;
    ppppuStack_138 = ppppuStack_1e8;
    plStack_130 = (longlong *)pcStack_1e0;
    plStack_128 = plStack_1d8;
    puStack_120 = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
    uStack_118 = 0;
    uStack_110 = 0;
    uStack_108 = 0;
    puStack_f0 = &UNK_142a99570;
    uStack_e8 = 0xf;
    uStack_e0 = 0x8000000000000000;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    pppuVar13 = extraout_RAX_04;
    pppuStack_b8 = extraout_RAX_04;
    break;
  case 2:
    pppuStack_a8 = (undefined8 ***)in_RDX[3];
    uStack_b0 = in_RDX[4];
    country_core::utils::geo::mesh_from_pts();
    uStack_e8 = uStack_200;
    lStack_f8 = lStack_210;
    puStack_f0 = puStack_208;
    uStack_108 = uStack_220;
    lStack_100 = local_218;
    uStack_118 = uStack_230;
    uStack_110 = uStack_228;
    plStack_128 = plStack_240;
    puStack_120 = puStack_238;
    ppppuStack_138 = ppppuStack_250;
    plStack_130 = plStack_248;
    pppppuStack_148 = (undefined *****)plStack_260;
    puStack_140 = puStack_258;
    ppppuStack_158 = ppppuStack_270;
    plStack_150 = plStack_268;
    uStack_e0 = 0x8000000000000000;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    pppuVar13 = extraout_RAX_01;
    pppuStack_b8 = extraout_RAX_01;
    if (pppuStack_a8 != (undefined8 ***)0x0) {
      uStack_a0 = CONCAT44(uStack_a0._4_4_,(int)CONCAT71((int7)((ulonglong)extraout_RAX_01 >> 8),1))
      ;
code_r0x00014089e05f:
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
    break;
  default:
    uStack_160 = in_RDX[3];
    pppuStack_a8 = (undefined8 ***)in_RDX[5];
    uStack_a0 = in_RDX[6];
    uVar14 = in_RDX[7];
    uStack_91 = 0;
    uStack_b0 = in_RDX[2];
    country_core::utils::geo::mesh_from_pts();
    ppppuStack_158 = (undefined ****)pppuStack_a8;
    plStack_150 = (longlong *)uStack_a0;
    uStack_92 = 1;
    pppppuStack_148 = (undefined *****)uVar14;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    uStack_e8 = uStack_178;
    lStack_f8 = lStack_188;
    puStack_f0 = puStack_180;
    uStack_108 = uStack_198;
    lStack_100 = lStack_190;
    uStack_118 = uStack_1a8;
    uStack_110 = uStack_1a0;
    plStack_128 = plStack_1b8;
    puStack_120 = puStack_1b0;
    ppppuStack_138 = ppppuStack_1c8;
    plStack_130 = plStack_1c0;
    pppppuStack_148 = (undefined *****)plStack_1d8;
    puStack_140 = puStack_1d0;
    ppppuStack_158 = ppppuStack_1e8;
    plStack_150 = (longlong *)pcStack_1e0;
    uStack_e0 = 0x8000000000000000;
    uStack_92 = 0;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    pppuVar13 = extraout_RAX_02;
    pppuStack_b8 = extraout_RAX_02;
    if (uStack_b0 != 0) {
      uStack_a0 = CONCAT44(uStack_a0._4_4_,(int)CONCAT71((int7)((ulonglong)extraout_RAX_02 >> 8),1))
      ;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
    break;
  case 4:
    uStack_b0 = in_RDX[3];
    pppuStack_a8 = (undefined8 ***)in_RDX[4];
    (*(code *)pppuStack_a8[5])(&ppppuStack_158,uStack_b0);
    pppuStack_b8 = (undefined8 ***)plStack_150;
    country_core::utils::geo::mesh_from_pts();
    if (ppppuStack_158 != (undefined ****)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uStack_e8 = uStack_178;
    lStack_f8 = lStack_188;
    puStack_f0 = puStack_180;
    uStack_108 = uStack_198;
    lStack_100 = lStack_190;
    uStack_118 = uStack_1a8;
    uStack_110 = uStack_1a0;
    plStack_128 = plStack_1b8;
    puStack_120 = puStack_1b0;
    ppppuStack_138 = ppppuStack_1c8;
    plStack_130 = plStack_1c0;
    pppppuStack_148 = (undefined *****)plStack_1d8;
    puStack_140 = puStack_1d0;
    ppppuStack_158 = ppppuStack_1e8;
    plStack_150 = (longlong *)pcStack_1e0;
    uStack_e0 = 0x8000000000000000;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    pppuVar13 = (undefined8 ***)0x0;
    pppuStack_b8 = extraout_RAX_00;
    if (*pppuStack_a8 != (undefined8 **)0x0) {
      pppuVar13 = (undefined8 ***)(*(code *)*pppuStack_a8)(uStack_b0);
    }
    if (pppuStack_a8[1] != (undefined8 **)0x0) {
      uStack_a0 = CONCAT44(uStack_a0._4_4_,(int)CONCAT71((int7)((ulonglong)pppuVar13 >> 8),1));
      goto code_r0x00014089e05f;
    }
  }
  uStack_a0 = CONCAT44(uStack_a0._4_4_,(int)CONCAT71((int7)((ulonglong)pppuVar13 >> 8),1));
LAB_14089e296:
  bevy_ecs::system::commands::Commands::spawn_empty();
  if (*(char *)(local_c0 + 10) == '\0') {
    uVar19 = 0x3f800000;
    cVar4 = *(char *)(local_c0 + 8);
  }
  else {
    uVar19 = *(undefined4 *)((longlong)local_c0 + 0x54);
    cVar4 = *(char *)(local_c0 + 8);
  }
  if (cVar4 == '\0') {
    *(undefined1 *)(local_c0 + 8) = 0;
    *(undefined4 *)((longlong)local_c0 + 0x44) = uVar19;
    uVar19 = 0;
  }
  uVar14 = local_c0[0xd];
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  ppppuVar12 = ppppuStack_138;
  if ((extraout_RAX_05 & 1) != 0) {
    pppppuVar18 = (undefined *****)plStack_150;
    if (plStack_150 == (longlong *)0x0) {
      pppppuVar18 = pppppuStack_148;
    }
    uVar10 = *local_c0;
    uVar11 = local_c0[1];
    uVar2 = *(undefined4 *)(local_c0 + 0xf);
    uVar5 = *(undefined4 *)(local_c0 + 9);
    uVar3 = *(undefined4 *)((longlong)local_c0 + 0x4c);
    uVar6 = local_c0[8];
    lVar16 = (longlong)pppppuVar18[2];
    if ((ulonglong)((longlong)*pppppuVar18 - lVar16) < 0x98) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar17 = (longlong)pppppuVar18[1];
    *(code **)(lVar17 + lVar16) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::systems::ui::signifiers::SignifierAnimation>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar17 + 8 + lVar16);
    *puVar1 = uVar10;
    puVar1[1] = uVar11;
    *(undefined8 *)(lVar17 + 0x18 + lVar16) = *(undefined8 *)((longlong)local_c0 + 0x7c);
    *(undefined4 *)(lVar17 + 0x20 + lVar16) = *(undefined4 *)((longlong)local_c0 + 0x84);
    *(undefined4 *)(lVar17 + 0x24 + lVar16) = uVar19;
    *(undefined4 *)(lVar17 + 0x28 + lVar16) = uVar19;
    *(undefined4 *)(lVar17 + 0x2c + lVar16) = uVar19;
    *(undefined8 ****)(lVar17 + 0x38 + lVar16) = pppuStack_b8;
    *(undefined8 *)(lVar17 + 0x40 + lVar16) = uVar14;
    *(undefined4 *)(lVar17 + 0x48 + lVar16) = uVar5;
    *(undefined4 *)(lVar17 + 0x4c + lVar16) = uVar3;
    *(undefined8 *)(lVar17 + 0x50 + lVar16) = uVar6;
    *(undefined4 *)(lVar17 + 0x58 + lVar16) = uVar2;
    *(undefined8 *)(lVar17 + 0x5c + lVar16) = 0;
    *(undefined1 *)(lVar17 + 0x68 + lVar16) = 0;
    *(undefined *****)(lVar17 + 0x78 + lVar16) = ppppuVar12;
    *(code **)(lVar17 + 0x88 + lVar16) = bevy_ecs::error::handler::panic;
    lVar17 = lVar16 + 0x98;
    pppppuVar18[2] = (undefined ****)lVar17;
    if (*(char *)((longlong)local_c0 + 0x89) == '\0') {
      if (*(char *)((longlong)local_c0 + 0x8a) == '\0') {
        if ((ulonglong)((longlong)*pppppuVar18 - lVar17) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar17 = (longlong)pppppuVar18[1];
        *(code **)(lVar17 + 0x98 + lVar16) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::UiPass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar17 + 0xa0 + lVar16) = bevy_ecs::error::handler::panic;
        *(undefined *****)(lVar17 + 0xa8 + lVar16) = ppppuVar12;
        *(undefined1 *)(lVar17 + 0xb0 + lVar16) = 0;
        lVar17 = lVar16 + 0xb8;
        pppppuVar18[2] = (undefined ****)lVar17;
        lVar15 = 0xd8;
        if ((ulonglong)((longlong)*pppppuVar18 - lVar17) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
      else {
        lVar15 = 0xb8;
        if ((ulonglong)((longlong)*pppppuVar18 - lVar17) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
    }
    else {
      if ((ulonglong)((longlong)*pppppuVar18 - lVar17) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar17 = (longlong)pppppuVar18[1];
      *(code **)(lVar17 + 0x98 + lVar16) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::WireframePass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar17 + 0xa0 + lVar16) = bevy_ecs::error::handler::panic;
      *(undefined *****)(lVar17 + 0xa8 + lVar16) = ppppuVar12;
      *(undefined1 *)(lVar17 + 0xb0 + lVar16) = 0;
      lVar17 = lVar16 + 0xb8;
      pppppuVar18[2] = (undefined ****)lVar17;
      lVar15 = 0xd8;
      if ((ulonglong)((longlong)*pppppuVar18 - lVar17) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    lVar7 = (longlong)pppppuVar18[1];
    *(code **)(lVar7 + lVar17) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DontCastShadow>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar7 + 8 + lVar17) = bevy_ecs::error::handler::panic;
    *(undefined *****)(lVar7 + 0x10 + lVar17) = ppppuVar12;
    *(undefined1 *)(lVar7 + 0x18 + lVar17) = 0;
    lVar16 = lVar16 + lVar15;
    pppppuVar18[2] = (undefined ****)lVar16;
    if ((*(byte *)(local_c0 + 0xb) & 1) != 0) {
      uVar19 = *(undefined4 *)((longlong)local_c0 + 0x5c);
      if ((ulonglong)((longlong)*pppppuVar18 - lVar16) < 0x28) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar17 = (longlong)pppppuVar18[1];
      *(code **)(lVar17 + lVar16) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierMinimumLifespan>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar17 + 8 + lVar16) = bevy_ecs::error::handler::panic;
      *(undefined *****)(lVar17 + 0x10 + lVar16) = ppppuVar12;
      *(undefined4 *)(lVar17 + 0x18 + lVar16) = 0;
      *(undefined4 *)(lVar17 + 0x1c + lVar16) = uVar19;
      *(undefined1 *)(lVar17 + 0x20 + lVar16) = 0;
      lVar16 = lVar16 + 0x28;
      pppppuVar18[2] = (undefined ****)lVar16;
    }
    if (*(char *)(local_c0 + 0x11) != '\0') {
      if ((ulonglong)((longlong)*pppppuVar18 - lVar16) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar17 = (longlong)pppppuVar18[1];
      *(code **)(lVar17 + lVar16) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar17 + 8 + lVar16) = bevy_ecs::error::handler::panic;
      *(undefined *****)(lVar17 + 0x10 + lVar16) = ppppuVar12;
      *(undefined1 *)(lVar17 + 0x18 + lVar16) = 0;
      pppppuVar18[2] = (undefined ****)(lVar16 + 0x20);
    }
    if ((char)uStack_a0 != '\0') {
      plVar8 = *(longlong **)(local_168 + 0x18);
      LOCK();
      lVar16 = *plVar8;
      *plVar8 = *plVar8 + 1;
      UNLOCK();
      if (*plVar8 == 0 || SCARRY8(lVar16,1) != *plVar8 < 0) {
                    // WARNING: Does not return
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      bevy_ecs::system::commands::EntityCommands::
      insert<country_core::components::transient_mesh::TransientMesh>();
    }
    local_170[4] = (longlong)ppppuStack_138;
    local_170[2] = (longlong)pppppuStack_148;
    local_170[3] = (longlong)puStack_140;
    *local_170 = (longlong)ppppuStack_158;
    local_170[1] = (longlong)plStack_150;
    if (local_c0[0xc] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

