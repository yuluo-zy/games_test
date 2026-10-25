// addr=0x140a38610 name=startup_nani
// signature=void __cdecl startup_nani(void)


// WARNING: Instruction at (ram,0x000140a393ed) overlaps instruction at (ram,0x000140a393ea)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Type propagation algorithm not settling

void __cdecl country_core::startup::startup_nani::startup_nani(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined1 *puVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  undefined1 *puVar7;
  undefined1 *puVar8;
  byte bVar9;
  uint uVar10;
  code *UNRECOVERED_JUMPTABLE;
  char *pcVar11;
  longlong *extraout_RAX;
  ulonglong extraout_RAX_00;
  char *pcVar12;
  longlong lVar13;
  code *extraout_RAX_01;
  char cVar14;
  code *UNRECOVERED_JUMPTABLE_00;
  uint *in_RDX;
  undefined8 *unaff_RBX;
  undefined1 *puVar15;
  undefined1 *puVar16;
  undefined1 *puVar17;
  undefined1 *puVar18;
  undefined1 *puVar19;
  undefined1 *puVar20;
  undefined8 *puVar21;
  undefined8 *puVar22;
  undefined8 *in_R8;
  longlong lVar23;
  ulonglong uVar24;
  longlong *unaff_R12;
  ulonglong unaff_R13;
  undefined8 unaff_R14;
  longlong unaff_R15;
  undefined2 in_ES;
  bool bVar25;
  bool bVar26;
  int *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined1 auStack_be71 [47169];
  undefined8 *puStack_630;
  undefined8 *puStack_598;
  undefined8 *apuStack_578 [2];
  undefined1 auStack_568 [8];
  undefined1 auStack_560 [8];
  undefined4 uStack_558;
  undefined8 uStack_548;
  undefined8 uStack_540;
  undefined8 uStack_538;
  byte abStack_520 [48];
  undefined8 auStack_4f0 [27];
  undefined *puStack_418;
  undefined8 uStack_410;
  undefined *puStack_408;
  undefined8 uStack_400;
  undefined *puStack_3f8;
  undefined8 uStack_3f0;
  undefined *puStack_3e8;
  undefined8 uStack_3e0;
  undefined *puStack_3d8;
  undefined8 uStack_3d0;
  undefined *puStack_3c8;
  undefined8 uStack_3c0;
  undefined *puStack_3b8;
  undefined8 uStack_3b0;
  undefined *puStack_3a8;
  undefined8 uStack_3a0;
  undefined *puStack_398;
  undefined8 uStack_390;
  undefined *puStack_388;
  undefined8 uStack_380;
  undefined *puStack_378;
  undefined8 uStack_370;
  undefined *puStack_368;
  undefined8 uStack_360;
  undefined *puStack_358;
  undefined8 uStack_350;
  undefined *puStack_348;
  undefined8 uStack_340;
  undefined *puStack_338;
  undefined8 uStack_330;
  undefined *puStack_328;
  undefined8 uStack_320;
  undefined *puStack_318;
  undefined8 uStack_310;
  undefined *puStack_308;
  undefined8 uStack_300;
  undefined *puStack_2f8;
  undefined8 uStack_2f0;
  undefined *puStack_2e8;
  undefined8 uStack_2e0;
  undefined *puStack_2d8;
  undefined8 uStack_2d0;
  undefined *puStack_2c8;
  undefined8 uStack_2c0;
  undefined *puStack_2b8;
  undefined8 uStack_2b0;
  undefined *puStack_2a8;
  undefined8 uStack_2a0;
  undefined *puStack_298;
  undefined8 uStack_290;
  undefined *puStack_288;
  undefined8 uStack_280;
  undefined *puStack_278;
  undefined8 uStack_270;
  undefined *puStack_268;
  undefined8 uStack_260;
  undefined *puStack_258;
  undefined8 uStack_250;
  undefined *puStack_248;
  undefined8 uStack_240;
  undefined *puStack_238;
  undefined8 uStack_230;
  undefined *puStack_228;
  undefined8 uStack_220;
  undefined *puStack_218;
  undefined8 uStack_210;
  undefined *puStack_208;
  undefined8 uStack_200;
  undefined *puStack_1f8;
  undefined8 uStack_1f0;
  undefined *puStack_1e8;
  undefined8 uStack_1e0;
  undefined *puStack_1d8;
  undefined8 uStack_1d0;
  undefined *puStack_1c8;
  undefined8 uStack_1c0;
  undefined *puStack_1b8;
  undefined8 uStack_1b0;
  undefined *puStack_1a8;
  undefined8 uStack_1a0;
  undefined *puStack_198;
  undefined8 uStack_190;
  undefined *puStack_188;
  undefined8 uStack_180;
  undefined *puStack_178;
  undefined8 uStack_170;
  undefined *puStack_168;
  undefined8 uStack_160;
  undefined *puStack_158;
  undefined8 uStack_150;
  undefined *puStack_148;
  undefined8 uStack_140;
  undefined *puStack_138;
  undefined8 uStack_130;
  undefined *puStack_128;
  undefined8 uStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined *puStack_108;
  undefined8 uStack_100;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  longlong lStack_b8;
  undefined8 uStack_b0;
  longlong lStack_98;
  undefined8 uStack_90;
  uint uStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  int iStack_50;
  undefined8 local_48;
  
  puVar20 = auStack_568;
  puVar19 = auStack_568;
  puVar18 = auStack_568;
  puVar21 = auStack_4f0 + 1;
  local_48 = 0xfffffffffffffffe;
  lVar23 = 0;
  UNRECOVERED_JUMPTABLE = (code *)&DAT_142acd998;
LAB_140a38660:
  uVar24 = (ulonglong)(int)(&DAT_142acd998)[lVar23];
  bVar25 = 0xfffffffebd532667 < uVar24;
  lVar23 = 2;
  bVar9 = (byte)UNRECOVERED_JUMPTABLE_00;
  cVar14 = (char)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8);
  uVar10 = (uint)UNRECOVERED_JUMPTABLE_00;
  puVar4 = auStack_568;
  puVar5 = auStack_568;
  puVar17 = auStack_568;
  puVar16 = auStack_568;
  puVar15 = auStack_568;
  puVar6 = auStack_568;
  puVar7 = auStack_568;
  puVar8 = auStack_568;
  puVar22 = puVar21;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Read-only address (ram,0x000142acd999) is written
  switch((longlong)&DAT_142acd998 + uVar24) {
  case 0x4401eea7:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a38657:
    lVar23 = 0x12;
  case 0x140a38660:
    goto LAB_140a38660;
  case 0x140a38671:
    lVar23 = 1;
    goto LAB_140a38660;
  case 0x140a38676:
    lVar23 = 0x10;
    goto LAB_140a38660;
  case 0x140a3867b:
    lVar23 = 0xd;
    goto LAB_140a38660;
  case 0x140a38680:
    lVar23 = 6;
    goto LAB_140a38660;
  case 0x140a38685:
    lVar23 = 0xe;
    goto LAB_140a38660;
  case 0x140a3868a:
    lVar23 = 0xb;
    goto LAB_140a38660;
  case 0x140a3868f:
    goto code_r0x000140a3868f;
  case 0x140a38694:
    lVar23 = 0x1a;
    goto LAB_140a38660;
  case 0x140a38699:
    lVar23 = 4;
    goto LAB_140a38660;
  case 0x140a3869e:
    lVar23 = 0x11;
    goto LAB_140a38660;
  case 0x140a386a3:
    lVar23 = 5;
    goto LAB_140a38660;
  case 0x140a386a8:
    lVar23 = 9;
    goto LAB_140a38660;
  case 0x140a386ad:
    lVar23 = 3;
    goto LAB_140a38660;
  case 0x140a386b2:
    lVar23 = 0x13;
    goto LAB_140a38660;
  case 0x140a386b7:
    lVar23 = 0x16;
    goto LAB_140a38660;
  case 0x140a386bc:
    lVar23 = 7;
    goto LAB_140a38660;
  case 0x140a386c1:
    lVar23 = 0x17;
    goto LAB_140a38660;
  case 0x140a386c6:
    lVar23 = 0xf;
    goto LAB_140a38660;
  case 0x140a386cb:
    lVar23 = 8;
    goto LAB_140a38660;
  case 0x140a386d0:
    lVar23 = 0x14;
    goto LAB_140a38660;
  case 0x140a386d5:
    lVar23 = 0xc;
    goto LAB_140a38660;
  case 0x140a386da:
    lVar23 = 10;
    goto LAB_140a38660;
  case 0x140a386df:
    lVar23 = 0x18;
    goto LAB_140a38660;
  case 0x140a386e7:
    lVar23 = 0x15;
    goto LAB_140a38660;
  case 0x140a386ef:
    lVar23 = 0x19;
    goto LAB_140a38660;
  case 0x140a386f7:
    goto code_r0x000140a386f7;
  case 0x140a38cad:
    uStack_548 = CONCAT44(uStack_548._4_4_,0x42acd998);
    goto code_r0x000140a38cb1;
  case 0x140a38ccd:
    goto code_r0x000140a38ccd;
  case 0x140a38ced:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfdf6ac71;
    goto code_r0x000140a38cf2;
  case 0x140a38d0d:
    goto code_r0x000140a38d0d;
  case 0x140a38d2d:
    cRam00000000fffefdd9 = cRam00000000fffefdd9 + -0x27;
    puVar18 = auStack_568;
code_r0x000140a38d31:
    *(undefined1 *)((longlong)puVar21 + 0x49f) = 1;
code_r0x000140a3906c:
    *(undefined **)(puVar18 + -8) = &UNK_140a39077;
    resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset();
    *(undefined **)(puVar18 + -8) = &UNK_140a3908c;
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_00 & 1) == 0) {
code_r0x000140a39620:
      puVar21[-7] = (undefined *)((longlong)puVar21 + 0x49e);
      puVar21[-6] = country_core::render::nani::gpu_world::impl$17::fmt;
      puVar21[0x1a] = &UNK_142acd1e0;
      puVar21[0x1b] = 1;
      puVar21[0x1e] = 0;
      puVar21[0x1c] = puVar21 + -7;
      puVar21[0x1d] = 1;
                    // WARNING: Subroutine does not return
      *(undefined **)(puVar18 + -8) = &UNK_140a39683;
      core::panicking::panic_fmt();
    }
    in_stack_00000030 = extraout_RAX;
    if ((int)*extraout_RAX != 1) {
      puVar21[-7] = 0x8000000000000001;
      puVar17 = puVar18;
      goto code_r0x000140a391be;
    }
code_r0x000140a390a0:
    unaff_R15 = in_stack_00000030[1];
    *(undefined **)(puVar18 + -8) = &UNK_140a390b7;
    bevy_ecs::system::commands::Commands::spawn_empty();
    unaff_R12 = (longlong *)puVar21[0x1b];
    puVar8 = puVar18;
code_r0x000140a390be:
    puVar18 = puVar8;
    unaff_R14 = puVar21[0x1e];
    if (unaff_R12 == (longlong *)0x0) {
      unaff_R12 = (longlong *)puVar21[0x1c];
    }
    unaff_RBX = (undefined8 *)unaff_R12[2];
    if ((ulonglong)(*unaff_R12 - (longlong)unaff_RBX) < 0x78) {
      *(undefined8 *)(puVar18 + 0x20) = 1;
code_r0x000140a39604:
      lVar23 = 1;
      *(undefined **)(puVar18 + -8) = &UNK_140a3961b;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
code_r0x000140a390e7:
    UNRECOVERED_JUMPTABLE = (code *)unaff_R12[1];
    *(code **)(UNRECOVERED_JUMPTABLE + (longlong)unaff_RBX) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::render::nani::GpuDrivenDrawable,country_core::components::KeepOnLevelUnload,country_core::components::drawable::OcclusionCulled>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined8 *)(UNRECOVERED_JUMPTABLE + 8 + (longlong)unaff_RBX) = 0;
    *(undefined8 *)(UNRECOVERED_JUMPTABLE + 8 + (longlong)unaff_RBX + 8) = 0x3f80000000000000;
    *(undefined8 *)(UNRECOVERED_JUMPTABLE + 0x18 + (longlong)unaff_RBX) = 0;
    *(undefined4 *)(UNRECOVERED_JUMPTABLE + 0x20 + (longlong)unaff_RBX) = 0;
    *(undefined8 *)(UNRECOVERED_JUMPTABLE + 0x24 + (longlong)unaff_RBX) = 0x3f8000003f800000;
    *(undefined4 *)(UNRECOVERED_JUMPTABLE + 0x2c + (longlong)unaff_RBX) = 0x3f800000;
    *(longlong *)(UNRECOVERED_JUMPTABLE + 0x38 + (longlong)unaff_RBX) = unaff_R15;
    *(int **)(UNRECOVERED_JUMPTABLE + 0x40 + (longlong)unaff_RBX) = in_stack_00000028;
    puVar4 = puVar18;
code_r0x000140a39135:
    puVar18 = puVar4;
    (UNRECOVERED_JUMPTABLE + 0x48)[(longlong)unaff_RBX] = (code)0x0;
    *(undefined8 *)(UNRECOVERED_JUMPTABLE + 0x58 + (longlong)unaff_RBX) = unaff_R14;
    *(code **)(UNRECOVERED_JUMPTABLE + 0x68 + (longlong)unaff_RBX) = bevy_ecs::error::handler::panic
    ;
    unaff_R12[2] = (longlong)(unaff_RBX + 0xf);
    UNRECOVERED_JUMPTABLE =
         core::ops::function::FnOnce::
         call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
    ;
    UNRECOVERED_JUMPTABLE_00 = (code *)(unaff_R13 & 0xff);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(UNRECOVERED_JUMPTABLE_00) {
    case (code *)0x0:
    case (code *)0x1:
    case (code *)0x2:
code_r0x000140a391ca:
      goto code_r0x000140a391de;
    case (code *)0x3:
      lVar23 = 0;
code_r0x000140a3949a:
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
code_r0x000140a392e7:
      unaff_RBX = (undefined8 *)puVar21[0x91];
      unaff_R14 = puVar21[0x8f];
      unaff_R15 = puVar21[0x90];
      uVar10 = *(uint *)(puVar21 + 0x93);
      *(undefined **)(puVar18 + -8) = &UNK_140a3930c;
      (*UNRECOVERED_JUMPTABLE)(puVar21 + -7,unaff_RBX);
code_r0x000140a3930c:
      unaff_R12 = (longlong *)(ulonglong)uVar10;
      *(uint *)puVar21[0x8e] = uVar10;
      in_stack_00000028 = (int *)(unaff_R13 & 0xff);
      puVar19 = puVar18;
      if (in_stack_00000030[0x13] != 0) {
        uVar24 = (ulonglong)(uint)((int)in_stack_00000028 << 10);
        lVar13 = in_stack_00000030[0x13] * 0x10;
        if (*(ulonglong *)(unaff_R15 + 0x1470) < lVar13 + uVar24) {
          UNRECOVERED_JUMPTABLE = (code *)&DAT_0000000b;
          goto code_r0x000140a396d0;
        }
        lVar23 = in_stack_00000030[0x12];
        puVar21[0x88] = 0;
        puVar21[0x89] = lVar13;
        *(undefined1 *)((longlong)puVar21 + 0x497) = 1;
        *(ulonglong *)(puVar18 + 0x38) = uVar24;
        *(undefined8 **)(puVar18 + 0x28) = puVar21 + 0x88;
        *(longlong *)(puVar18 + 0x20) = lVar13;
        *(undefined8 *)(puVar18 + 0x30) = 1;
        puVar6 = puVar18;
code_r0x000140a39394:
        puVar18 = puVar6;
        *(undefined **)(puVar18 + -8) = &UNK_140a39399;
        rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
        UNRECOVERED_JUMPTABLE = (code *)puVar21[0x1a];
        puVar19 = puVar18;
        if (UNRECOVERED_JUMPTABLE != (code *)&DAT_0000000b) {
          puVar21[0x18] = puVar21[0x23];
          puVar21[0x16] = puVar21[0x21];
          puVar21[0x17] = puVar21[0x22];
          puVar21[0x14] = puVar21[0x1f];
          puVar21[0x15] = puVar21[0x20];
          puVar21[0x12] = puVar21[0x1d];
          puVar21[0x13] = puVar21[0x1e];
          puVar21[0x10] = puVar21[0x1b];
          puVar21[0x11] = puVar21[0x1c];
          goto code_r0x000140a396d0;
        }
      }
code_r0x000140a393aa:
      *(int *)puVar21[0x8e] = (int)unaff_R12;
      *(undefined **)(puVar19 + -8) = &UNK_140a393ca;
      memcpy(puVar21 + 0x1a,puVar21 + -7,0xb8);
      UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uint)((int)in_stack_00000028 * 0xc0);
      puVar21[0x8d] = UNRECOVERED_JUMPTABLE + unaff_R15 + 8;
      puVar18 = puVar19;
code_r0x000140a393e0:
      bVar26 = SBORROW8(*(longlong *)(UNRECOVERED_JUMPTABLE + unaff_R15 + 8),-0x7ffffffffffffffe);
      bVar25 = *(longlong *)(UNRECOVERED_JUMPTABLE + unaff_R15 + 8) + 0x7ffffffffffffffe < 0;
code_r0x000140a393ef:
      puVar7 = puVar18;
      if (bVar26 == bVar25) {
code_r0x000140a393f8:
        puVar18 = puVar7;
        *(undefined **)(puVar18 + -8) = &UNK_140a393fd;
        core::ptr::drop_in_place<country_core::render::nani::gpu_world::OwnedGpuWorldSubset>();
      }
      in_RDX = (uint *)(puVar21 + 0x1a);
code_r0x000140a39404:
      in_R8 = (undefined8 *)&DAT_000000b8;
      UNRECOVERED_JUMPTABLE_00 = (code *)puVar21[0x8d];
      puVar5 = puVar18;
code_r0x000140a39411:
      puVar18 = puVar5;
      *(undefined **)(puVar18 + -8) = &UNK_140a39416;
      memcpy(UNRECOVERED_JUMPTABLE_00,in_RDX,(size_t)in_R8);
      bVar9 = *(byte *)((longlong)puVar21 + 0x49f);
      unaff_R13 = (ulonglong)bVar9;
      UNRECOVERED_JUMPTABLE_00 = (code *)&UNK_142acda08;
      uVar24 = (ulonglong)*(int *)(&UNK_142acda08 + unaff_R13 * 4);
      UNRECOVERED_JUMPTABLE = (code *)(&UNK_142acda08 + uVar24);
      cVar14 = (char)UNRECOVERED_JUMPTABLE;
      uVar10 = (uint)UNRECOVERED_JUMPTABLE;
      puVar17 = puVar18;
      puVar16 = puVar18;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(bVar9) {
      case 0:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_12_llvm_1710316161241481988;
        goto code_r0x000140a38d31;
      case 1:
        goto code_r0x000140a38d3d;
      case 2:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_14_llvm_1710316161241481988;
code_r0x000140a38d71:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 3;
        goto code_r0x000140a3906c;
      case 3:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_15_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 4;
        goto code_r0x000140a3906c;
      case 4:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_16_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 5;
        goto code_r0x000140a3906c;
      case 5:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_17_llvm_1710316161241481988;
code_r0x000140a38dd1:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 6;
        goto code_r0x000140a3906c;
      case 6:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_18_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 7;
        goto code_r0x000140a3906c;
      case 7:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_19_llvm_1710316161241481988;
code_r0x000140a38e11:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 8;
        goto code_r0x000140a3906c;
      case 8:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_20_llvm_1710316161241481988;
code_r0x000140a38e31:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 9;
        goto code_r0x000140a3906c;
      case 9:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_21_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 10;
        goto code_r0x000140a3906c;
      case 10:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_22_llvm_1710316161241481988;
code_r0x000140a38e71:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0xb;
        goto code_r0x000140a3906c;
      case 0xb:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_23_llvm_1710316161241481988;
code_r0x000140a38e91:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0xc;
        goto code_r0x000140a3906c;
      case 0xc:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_24_llvm_1710316161241481988;
code_r0x000140a38eb1:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0xd;
        goto code_r0x000140a3906c;
      case 0xd:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_25_llvm_1710316161241481988;
code_r0x000140a38ed1:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0xe;
        goto code_r0x000140a3906c;
      case 0xe:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_26_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0xf;
        goto code_r0x000140a3906c;
      case 0xf:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_27_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x10;
        goto code_r0x000140a3906c;
      case 0x10:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_28_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x11;
        goto code_r0x000140a3906c;
      case 0x11:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_29_llvm_1710316161241481988;
code_r0x000140a38f51:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x12;
        goto code_r0x000140a3906c;
      case 0x12:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_30_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x13;
        goto code_r0x000140a3906c;
      case 0x13:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_31_llvm_1710316161241481988;
code_r0x000140a38f91:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x14;
        goto code_r0x000140a3906c;
      case 0x14:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_32_llvm_1710316161241481988;
code_r0x000140a38fab:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x15;
        goto code_r0x000140a3906c;
      case 0x15:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        puVar15 = puVar18;
code_r0x000140a38fc4:
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_33_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x16;
        puVar18 = puVar15;
        goto code_r0x000140a3906c;
      case 0x16:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_34_llvm_1710316161241481988;
code_r0x000140a38feb:
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x17;
        goto code_r0x000140a3906c;
      case 0x17:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_35_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x18;
        goto code_r0x000140a3906c;
      case 0x18:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_36_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x19;
        goto code_r0x000140a3906c;
      case 0x19:
        *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
        in_stack_00000028 =
             (int *)&anon_3703a396df78e7356336212e87fdc18b_37_llvm_1710316161241481988;
        *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x1a;
        goto code_r0x000140a3906c;
      case 0x1a:
        goto code_r0x000140a39051;
      case 0x1b:
        uVar10 = *(uint *)(*(longlong *)(unaff_R15 + 0x1450) + 0x60);
        if (0x1f < uVar10) {
          *(undefined **)(puVar18 + -8) = &UNK_140a3957e;
          std::alloc::__default_lib_allocator::__rust_alloc_zeroed();
          if (extraout_RAX_01 == (code *)0x0) {
                    // WARNING: Subroutine does not return
            *(undefined **)(puVar18 + -8) = &UNK_140a3975e;
            alloc::raw_vec::handle_error();
          }
          puVar21[0x8e] = (ulonglong)(uVar10 >> 5);
          UNRECOVERED_JUMPTABLE = extraout_RAX_01;
          goto code_r0x000140a3958e;
        }
        UNRECOVERED_JUMPTABLE = (code *)&DAT_00000004;
        break;
      case 0x1c:
      case 0x1d:
      case 0x1e:
        goto code_r0x000140a3915a;
      case 0x1f:
        goto code_r0x000140a3941d;
      case 0x20:
        goto code_r0x000140a3926a;
      case 0x21:
        *(uint *)UNRECOVERED_JUMPTABLE = *(int *)UNRECOVERED_JUMPTABLE + uVar10;
code_r0x000140a39247:
        UNRECOVERED_JUMPTABLE =
             core::ops::function::FnOnce::
             call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned_custom::closure_env_0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
        ;
        goto code_r0x000140a392e7;
      case 0x22:
        goto code_r0x000140a39263;
      case 0x23:
        *(char *)((longlong)unaff_RBX + 0x50ab367dd) =
             *(char *)((longlong)unaff_RBX + 0x50ab367dd) + '\b';
        *(undefined1 *)((longlong)UNRECOVERED_JUMPTABLE * 2) =
             *(undefined1 *)((longlong)UNRECOVERED_JUMPTABLE * 2);
        goto code_r0x000140a391fa;
      case 0x24:
      case 0x27:
      case 0x28:
      case 0x29:
        goto code_r0x000140a39100;
      case 0x25:
        goto code_r0x000140a39481;
      case 0x26:
        *in_RDX = *in_RDX | uVar10;
        goto code_r0x000140a391de;
      case 0x2a:
        goto code_r0x000140a39404;
      case 0x2b:
        goto code_r0x000140a391be;
      case 0x2c:
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        uVar10 = uVar10 ^ 0xb81ffdf6;
        *(char *)(ulonglong)uVar10 = *(char *)(ulonglong)uVar10 + (char)uVar10;
code_r0x000140a3945b:
        lVar23 = 0;
        goto code_r0x000140a39468;
      case 0x2d:
        goto code_r0x000140a391fa;
      case 0x2e:
        pcVar11 = (char *)((ulonglong)puVar18 & 0xffffffff);
        puVar18 = (undefined1 *)((ulonglong)UNRECOVERED_JUMPTABLE & 0xffffffff);
        cVar14 = (char)pcVar11;
        *pcVar11 = *pcVar11 + cVar14;
        *(char *)((longlong)unaff_RBX + 0x50ab36b0e) =
             *(char *)((longlong)unaff_RBX + 0x50ab36b0e) + -0x1e;
        pcVar12 = (char *)(ulonglong)(uint)(int)(short)pcVar11;
        *pcVar12 = *pcVar12 + cVar14;
                    // WARNING: Read-only address (ram,0x000142acda9b) is written
        UNK_142acda9b = cVar14 + -3;
        *(char *)((longlong)pcVar12 * 2) = *(char *)((longlong)pcVar12 * 2) + cVar14;
        pcVar12[-0x73] = pcVar12[-0x73] + -0x1e;
        UNRECOVERED_JUMPTABLE = (code *)(ulonglong)((int)(short)pcVar11 + 0xb31259);
        goto code_r0x000140a392e7;
      case 0x2f:
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      case 0x30:
        pcVar11 = (char *)(ulonglong)(uint)(int)(short)UNRECOVERED_JUMPTABLE;
        *pcVar11 = *pcVar11 + cVar14;
                    // WARNING: Read-only address (ram,0x000142acd9c1) is written
        UNK_142acd9c1 = cVar14 + -0x54;
                    // WARNING: Read-only address (ram,0x000142acda08) is written
        UNK_142acda08 = cVar14 + '\x15';
        *pcVar11 = *pcVar11 + cVar14;
code_r0x000140a39222:
        UNRECOVERED_JUMPTABLE =
             core::ops::function::FnOnce::
             call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::VelocityOnlyInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
        ;
        goto code_r0x000140a392e7;
      case 0x31:
        goto code_r0x000140a3922e;
      case 0x32:
        goto code_r0x000140a393eb;
      case 0x33:
      case 0x44:
        goto code_r0x000140a39468;
      case 0x34:
        goto code_r0x000140a391ca;
      case 0x35:
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        goto code_r0x000140a391ae;
      case 0x36:
        goto code_r0x000140a3949a;
      case 0x37:
      case 0x38:
        goto code_r0x000140a39227;
      case 0x39:
        goto code_r0x000140a3912e;
      default:
        *(uint *)UNRECOVERED_JUMPTABLE = *(int *)UNRECOVERED_JUMPTABLE + uVar10;
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        goto code_r0x000140a39620;
      case 0x3f:
        pcVar11 = (char *)(CONCAT71((int7)((ulonglong)UNRECOVERED_JUMPTABLE >> 8),
                                    cVar14 + (char)*UNRECOVERED_JUMPTABLE) + -0x73);
        *pcVar11 = *pcVar11 + '\b';
        goto code_r0x000140a39481;
      case 0x40:
        *(int *)((longlong)unaff_RBX + 0x50ab367a6) =
             *(int *)((longlong)unaff_RBX + 0x50ab367a6) + -1;
        pcVar11 = (char *)(ulonglong)(uint)(int)(short)UNRECOVERED_JUMPTABLE;
        *pcVar11 = *pcVar11 + cVar14;
                    // WARNING: Read-only address (ram,0x000142acd9c1) is written
        UNK_142acd9c1 = cVar14 + -0x54;
        *(byte *)in_RDX = (byte)*in_RDX + cVar14;
        *pcVar11 = *pcVar11 + cVar14;
        goto code_r0x000140a39481;
      case 0x41:
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
        break;
      case 0x42:
        LOCK();
        *(char *)(uVar24 + 0x183acda08) = (char)unaff_RBX;
        UNLOCK();
        goto code_r0x000140a39468;
      case 0x43:
        *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar14);
      }
      puVar21[0x8e] = 0;
code_r0x000140a3958e:
      puVar21[0x8d] = UNRECOVERED_JUMPTABLE;
      *(undefined **)(puVar18 + -8) = &UNK_140a395b0;
      asset_libraries::ssbo_library::AssetSsboLibrary::
      add_new_with_data<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>();
      *(undefined **)(puVar18 + -8) = &UNK_140a395c5;
      asset_libraries::ssbo_library::AssetSsboLibrary::
      add_new_with_data<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>();
      if (puVar21[0x8e] != 0) {
        *(undefined **)(puVar18 + -8) = &UNK_140a395e6;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    case (code *)0x4:
      goto code_r0x000140a392da;
    case (code *)0x5:
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392da;
    case (code *)0x6:
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392da;
    case (code *)0x7:
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392da;
    case (code *)0x8:
    case (code *)0xb:
    case (code *)0xc:
    case (code *)0xd:
      goto code_r0x000140a39184;
    case (code *)0x9:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::StoneFloorInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0xa:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::PlantInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0xe:
      lVar23 = 0;
      goto code_r0x000140a39481;
    case (code *)0xf:
      goto code_r0x000140a3922e;
    case (code *)0x10:
      lVar23 = 0;
      break;
    case (code *)0x11:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::WrenInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x12:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::LanternEmissiveInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x13:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned_custom::closure_env_0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x14:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::ClothInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x15:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned_custom::closure_env_0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x16:
      goto code_r0x000140a3945b;
    case (code *)0x18:
      lVar23 = 0;
      goto code_r0x000140a39247;
    case (code *)0x19:
      lVar23 = 0;
      goto code_r0x000140a39222;
    case (code *)0x1a:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned_custom::closure_env_0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x1b:
    case (code *)0x1c:
      UNRECOVERED_JUMPTABLE = (code *)0x42098534;
      goto code_r0x000140a392e7;
    case (code *)0x1d:
      core::ops::function::FnOnce::
      call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
           = (code)((char)core::ops::function::FnOnce::
                          call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
                   + ' ');
      goto code_r0x000140a391ae;
    default:
      core::ops::function::FnOnce::
      call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
           = (code)((char)core::ops::function::FnOnce::
                          call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
                   + ' ');
code_r0x000140a396d0:
      puVar21[0x1a] = UNRECOVERED_JUMPTABLE;
      puVar21[0x1b] = puVar21[0x10];
      puVar21[0x1c] = puVar21[0x11];
      puVar21[0x1d] = puVar21[0x12];
      puVar21[0x1e] = puVar21[0x13];
      puVar21[0x1f] = puVar21[0x14];
      puVar21[0x20] = puVar21[0x15];
      puVar21[0x21] = puVar21[0x16];
      puVar21[0x22] = puVar21[0x17];
      puVar21[0x23] = puVar21[0x18];
      *(undefined ***)(puVar18 + 0x20) =
           &anon_3703a396df78e7356336212e87fdc18b_49_llvm_1710316161241481988;
                    // WARNING: Subroutine does not return
      *(undefined **)(puVar18 + -8) = &UNK_140a39748;
      core::result::unwrap_failed();
    case (code *)0x23:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case (code *)0x24:
    case (code *)0x17:
    case (code *)0x28:
      lVar23 = 0;
      UNRECOVERED_JUMPTABLE =
           core::ops::function::FnOnce::
           call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::UberInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
      ;
      goto code_r0x000140a392e7;
    case (code *)0x25:
      cRam00000000ffff8920 = cRam00000000ffff8920 + ' ';
      UNRECOVERED_JUMPTABLE_00[-0x47] = (code)((char)UNRECOVERED_JUMPTABLE_00[-0x47] + ' ');
      cRam00000000ffff8920 = cRam00000000ffff8920 + -0x68;
      break;
    case (code *)0x26:
      UNRECOVERED_JUMPTABLE = (code *)0x4156af40;
      goto code_r0x000140a392e7;
    case (code *)0x27:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    UNRECOVERED_JUMPTABLE =
         core::ops::function::FnOnce::
         call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
    ;
    goto code_r0x000140a392e7;
  case 0x140a38d4d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd9d9) is written
                    // WARNING: Read-only address (ram,0x000142acd9d9) is written
    UNK_142acd9d9 = 0x86;
    puVar18 = auStack_568;
    goto code_r0x000140a38d51;
  case 0x140a38d6d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfdf6ac71;
    puVar18 = auStack_568;
    goto code_r0x000140a38d71;
  case 0x140a38d8d:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a38dad:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a38dcd:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd900) is written
                    // WARNING: Read-only address (ram,0x000142acd900) is written
    UNK_142acd900 = 0xdd;
    puVar18 = auStack_568;
    goto code_r0x000140a38dd1;
  case 0x140a38ded:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a38e0d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd999) is written
    DAT_142acd998._1_3_ = (undefined3)(-0x44b62cbf - (uint)bVar25 >> 8);
    DAT_142acd998 = CONCAT31(DAT_142acd998._1_3_,0x71);
    puVar18 = auStack_568;
    goto code_r0x000140a38e11;
  case 0x140a38e2d:
    uVar10 = bVar25 + 0x40a38671;
    *(char *)(ulonglong)uVar10 = *(char *)(ulonglong)uVar10 + (char)uVar10;
    puVar18 = auStack_568;
    goto code_r0x000140a38e31;
  case 0x140a38e4d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = LocalDescriptorTableRegister();
    puVar18 = auStack_568;
    goto code_r0x000140a3906c;
  case 0x140a38e6d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd9d9) is written
                    // WARNING: Read-only address (ram,0x000142acd9d9) is written
    UNK_142acd9d9 = 0x86;
    puVar18 = auStack_568;
    goto code_r0x000140a38e71;
  case 0x140a38e8d:
    cRam00000000fffefdd9 = cRam00000000fffefdd9 + -0x27;
    puVar18 = auStack_568;
    goto code_r0x000140a38e91;
  case 0x140a38ead:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfffefd71;
    puVar18 = auStack_568;
    goto code_r0x000140a38eb1;
  case 0x140a38ecd:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd999) is written
    DAT_142acd998._1_3_ = (undefined3)(bVar25 + 0x40a38671 >> 8);
    DAT_142acd998 = CONCAT31(DAT_142acd998._1_3_,0x71);
    puVar18 = auStack_568;
    goto code_r0x000140a38ed1;
  case 0x140a38eed:
    puVar18 = auStack_568;
    goto code_r0x000140a3906c;
  case 0x140a38f0d:
    puVar18 = auStack_568;
    goto code_r0x000140a3906c;
  case 0x140a38f2d:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a38f4d:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfffefd71;
    puVar18 = auStack_568;
    goto code_r0x000140a38f51;
  case 0x140a38f6d:
    puVar18 = auStack_568;
    goto code_r0x000140a3906c;
  case 0x140a38f8a:
    pcVar11 = (char *)CONCAT71(0x142acd9,(char)UNRECOVERED_JUMPTABLE_00[-0x44] + -0x68);
    *pcVar11 = *pcVar11 + (char)UNRECOVERED_JUMPTABLE_00[-0x44] + -0x68;
    puVar18 = auStack_568;
    goto code_r0x000140a38f91;
  case 0x140a38fa7:
    UNRECOVERED_JUMPTABLE = (code *)swi(0x13);
    bVar9 = (*UNRECOVERED_JUMPTABLE)();
    puVar18 = auStack_560;
    *(byte *)in_RDX = (byte)*in_RDX | bVar9;
    goto code_r0x000140a38fab;
  case 0x140a38fc4:
    goto code_r0x000140a38fc4;
  case 0x140a38fe1:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd925) is written
                    // WARNING: Read-only address (ram,0x000142acd925) is written
    UNK_142acd925 = bVar9 + 0x7a;
    puVar18 = auStack_568;
    goto code_r0x000140a38feb;
  case 0x140a39090:
    uRam000000017f269096 = in_ES;
    *in_stack_00000028 = *in_stack_00000028 + uVar10;
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0x40a38671;
                    // WARNING (jumptable): Read-only address (ram,0x000142acd921) is written
                    // WARNING: Read-only address (ram,0x000142acd921) is written
    UNK_142acd921 = bVar9 + 0x76;
    puVar18 = auStack_568;
    goto code_r0x000140a390a0;
  case 0x140a390be:
    goto code_r0x000140a390be;
  case 0x140a390ea:
    goto code_r0x000140a390e7;
  case 0x140a39135:
    goto code_r0x000140a39135;
  case 0x140a3914e:
    if (-1 < (longlong)((longlong)&DAT_142acd998 + uVar24)) {
      uStack_558 = SUB84(unaff_RBX,0);
      puVar4 = auStack_568;
      goto code_r0x000140a39135;
    }
    goto code_r0x000140a391ae;
  case 0x140a3915a:
    goto code_r0x000140a3915a;
  case 0x140a39173:
    *(int *)(UNRECOVERED_JUMPTABLE_00 + -0x74b3000b) =
         *(int *)(UNRECOVERED_JUMPTABLE_00 + -0x74b3000b) + -0x42acd998;
    LOCK();
    uRam0000000183acd998 = SUB81(unaff_RBX,0);
    UNLOCK();
    UNRECOVERED_JUMPTABLE_00 = (code *)0x2000;
code_r0x000140a39184:
    UNRECOVERED_JUMPTABLE_00 =
         (code *)(&UNK_142acdae4 + *(int *)(&UNK_142acdae4 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4)
                 );
code_r0x000140a39192:
code_r0x000140a391ae:
                    // WARNING: Could not recover jumptable at 0x000140a391ae. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)();
    return;
  case 0x140a39189:
    *in_RDX = *in_RDX | 0x42acd998;
    goto code_r0x000140a39184;
  case 0x140a39192:
    goto code_r0x000140a39192;
  case 0x140a391a8:
    goto code_r0x000140a391ae;
  case 0x140a391b7:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfdf6ac71;
                    // WARNING (jumptable): Read-only address (ram,0x000142acd921) is written
                    // WARNING: Read-only address (ram,0x000142acd921) is written
    UNK_142acd921 = 'w';
    puStack_598 = apuStack_578 + 1;
    puVar22 = apuStack_578 + 1;
    cVar3 = '\x04';
    puVar1 = puVar21;
    do {
      puVar1 = puVar1 + -1;
      puVar22 = puVar22 + -1;
      *puVar22 = *puVar1;
      cVar3 = cVar3 + -1;
    } while ('\0' < cVar3);
    cVar14 = bVar9 + cVar14;
    UNRECOVERED_JUMPTABLE_00 =
         (code *)CONCAT71((int7)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8),cVar14);
    iRam00000000ffffd998 = iRam00000000ffffd998 + -0x2668;
    cRam00000000ffffd925 = cRam00000000ffffd925 + cVar14;
    apuStack_578[1] = puVar21;
code_r0x000140a391de:
    UNRECOVERED_JUMPTABLE_00 =
         (code *)(&UNK_142acdae4 + *(int *)(&UNK_142acdae4 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4)
                 );
code_r0x000140a391f3:
code_r0x000140a391fa:
                    // WARNING: Could not recover jumptable at 0x000140a39208. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)();
    return;
  case 0x140a391be:
    goto code_r0x000140a391be;
  case 0x140a391d4:
    iRam00000000ffffd998._0_1_ = (char)iRam00000000ffffd998 + -0x68;
    UNRECOVERED_JUMPTABLE_00[-0x47] = (code)((char)UNRECOVERED_JUMPTABLE_00[-0x47] + -0x68);
    iRam00000000ffffd998 = CONCAT31(iRam00000000ffffd998._1_3_,(char)iRam00000000ffffd998 + 'q');
    goto code_r0x000140a391de;
  case 0x140a391dd:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd925) is written
                    // WARNING: Read-only address (ram,0x000142acd925) is written
    UNK_142acd925 = bVar9 + 0x7a;
    goto code_r0x000140a391de;
  case 0x140a391f3:
    goto code_r0x000140a391f3;
  case 0x140a391fa:
    goto code_r0x000140a391fa;
  case 0x140a3937b:
    uStack_538 = CONCAT44(uStack_538._4_4_,1);
    puVar6 = auStack_568;
  case 0x140a39394:
    goto code_r0x000140a39394;
  case 0x140a393ad:
    if (SCARRY8(uVar24,0x142acd998)) {
                    // WARNING (jumptable): Read-only address (ram,0x000142acd925) is written
                    // WARNING: Read-only address (ram,0x000142acd925) is written
      UNK_142acd925 = bVar9 & 0x7a;
      UNK_142acd900 = 0xdd;
      UNK_142acd921 = 0x76;
      UNK_142acd951 = 0x7f;
      DAT_142acd998 = 0xfdf6acd9;
      UNK_142acd9c1 = 0xac;
      UNK_142acda08 = 0x15;
      UNK_142acda9b = 0xfd;
      UNK_142acd971 = 0x84;
      UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfdf6ac71;
    goto code_r0x000140a393aa;
  case 0x140a393be:
    puStack_630 = apuStack_578 + 1;
    puVar1 = apuStack_578 + 1;
    puVar22 = apuStack_578 + 1;
    cVar3 = '\x17';
    puVar2 = puVar21;
    do {
      puVar2 = puVar2 + -1;
      puVar1 = puVar1 + -1;
      *puVar1 = *puVar2;
      cVar3 = cVar3 + -1;
    } while ('\0' < cVar3);
    puVar20 = auStack_be71;
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xfdf6ac71;
    UNRECOVERED_JUMPTABLE = (code *)CONCAT71(0x142acd9,cVar14 + -0x68);
    apuStack_578[1] = puVar21;
code_r0x000140a393c6:
    *(uint *)(UNRECOVERED_JUMPTABLE + -0x73) = *(int *)(UNRECOVERED_JUMPTABLE + -0x73) + uVar10;
    puVar19 = puVar20;
    puVar21 = puVar22;
    goto code_r0x000140a393aa;
  case 0x140a393c6:
    goto code_r0x000140a393c6;
  case 0x140a393d7:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd998) is written
                    // WARNING: Read-only address (ram,0x000142acd998) is written
    DAT_142acd998 = 0xd9fdf6ac;
    uStack_80 = uVar10;
    puVar18 = auStack_568;
    goto code_r0x000140a393e0;
  case 0x140a393df:
                    // WARNING (jumptable): Read-only address (ram,0x000142acd951) is written
                    // WARNING: Read-only address (ram,0x000142acd951) is written
    UNK_142acd951 = bVar9 + 0x7f;
    break;
  case 0x140a393ed:
    UNK_142acd900 = 0xdd;
    UNK_142acd921 = 0x76;
    UNK_142acd925 = 0x7a;
    UNK_142acd951 = 0x7f;
    DAT_142acd998 = 0xfdf6acd9;
    UNK_142acd9c1 = 0xac;
    UNK_142acda08 = 0x15;
    UNK_142acda9b = 0xfd;
    UNK_142acd971 = 0x84;
    UNK_142acd9d9 = 0xad;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x140a393f8:
    goto code_r0x000140a393f8;
  case 0x140a39403:
    UNRECOVERED_JUMPTABLE_00[-0x48] = (code)((char)UNRECOVERED_JUMPTABLE_00[-0x48] + -0x68);
    UNRECOVERED_JUMPTABLE_00 = (code *)(ulonglong)uStack_80;
    puVar5 = auStack_568;
  case 0x140a39411:
    goto code_r0x000140a39411;
  case 0x140a3942a:
    puVar5 = auStack_568;
    goto code_r0x000140a39411;
  case 0x140a395fc:
    uStack_548 = CONCAT44(uStack_548._4_4_,1);
    puVar18 = auStack_568;
    goto code_r0x000140a39604;
  }
                    // WARNING (jumptable): Read-only address (ram,0x000142acd971) is written
                    // WARNING: Read-only address (ram,0x000142acd971) is written
                    // WARNING (jumptable): Read-only address (ram,0x000142acd971) is written
                    // WARNING: Read-only address (ram,0x000142acd971) is written
  UNK_142acd971 = 0xf5;
  cRam0000000149f912ba = cRam0000000149f912ba + 'q';
  abStack_520[(longlong)UNRECOVERED_JUMPTABLE_00] =
       abStack_520[(longlong)UNRECOVERED_JUMPTABLE_00] | (byte)((ulonglong)unaff_RBX >> 8);
  puVar7 = auStack_568;
  goto code_r0x000140a393f8;
code_r0x000140a3912e:
  *UNRECOVERED_JUMPTABLE =
       (code)(((char)*UNRECOVERED_JUMPTABLE - (char)((ulonglong)unaff_RBX >> 8)) -
             (0xfffffffebd5325f7 < uVar24));
  goto code_r0x000140a390e7;
code_r0x000140a39227:
  goto code_r0x000140a392e7;
code_r0x000140a393eb:
  bVar26 = SBORROW4(*(int *)((longlong)(in_stack_00000028 + 2) + (longlong)UNRECOVERED_JUMPTABLE),
                    0x42acda08);
  bVar25 = *(int *)((longlong)(in_stack_00000028 + 2) + (longlong)UNRECOVERED_JUMPTABLE) +
           -0x42acda08 < 0;
  goto code_r0x000140a393ef;
code_r0x000140a3922e:
  UNRECOVERED_JUMPTABLE =
       core::ops::function::FnOnce::
       call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
  ;
code_r0x000140a392da:
  lVar23 = 0;
  goto code_r0x000140a392e7;
code_r0x000140a39468:
  UNRECOVERED_JUMPTABLE =
       core::ops::function::FnOnce::
       call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::BalconyDoorInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
  ;
  goto code_r0x000140a392e7;
code_r0x000140a391be:
  uVar10 = *(uint *)(puVar21 + 0x93);
  puVar18 = puVar17;
  goto code_r0x000140a3930c;
code_r0x000140a39481:
  UNRECOVERED_JUMPTABLE =
       core::ops::function::FnOnce::
       call_once<country_core::render::nani::gpu_world::impl_4::gpu_world_subset_desc::owned::closure_env_0<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>,tuple_<ref_<rhapsody::raw::device::GraphicsDevice>,usize,usize>_>
  ;
  goto code_r0x000140a392e7;
code_r0x000140a39100:
  *UNRECOVERED_JUMPTABLE =
       (code)(((char)*UNRECOVERED_JUMPTABLE - (char)lVar23) - (0xfffffffebd5325f7 < uVar24));
  goto code_r0x000140a390e7;
code_r0x000140a39263:
  puVar18 = puVar18 + 8;
code_r0x000140a3926a:
  UNRECOVERED_JUMPTABLE = (code *)0x415698e0;
  goto code_r0x000140a392da;
code_r0x000140a3941d:
  (&UNK_142acd995)[uVar24] = (&UNK_142acd995)[uVar24] + '\b';
  puVar5 = puVar18;
  goto code_r0x000140a39411;
code_r0x000140a3915a:
  UNRECOVERED_JUMPTABLE_00[0xf] =
       (code)((char)UNRECOVERED_JUMPTABLE_00[0xf] + (char)UNRECOVERED_JUMPTABLE);
  puVar4 = puVar16;
  goto code_r0x000140a39135;
code_r0x000140a39051:
  *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
  in_stack_00000028 = (int *)&anon_3703a396df78e7356336212e87fdc18b_38_llvm_1710316161241481988;
  *(undefined1 *)((longlong)puVar21 + 0x49f) = 0x1b;
  goto code_r0x000140a3906c;
code_r0x000140a38d3d:
  *(byte *)((longlong)puVar21 + 0x49e) = bVar9;
  in_stack_00000028 = (int *)&anon_3703a396df78e7356336212e87fdc18b_13_llvm_1710316161241481988;
code_r0x000140a38d51:
  *(undefined1 *)((longlong)puVar21 + 0x49f) = 2;
  goto code_r0x000140a3906c;
code_r0x000140a3868f:
  lVar23 = 0x1b;
  goto LAB_140a38660;
code_r0x000140a386f7:
  lStack_60 = **(longlong **)UNRECOVERED_JUMPTABLE_00 + 0x10;
  puStack_418 = &UNK_142accc10;
  uStack_410 = 0x20;
  puStack_408 = &UNK_142accc30;
  unaff_RBX = &uStack_400;
  uStack_400 = 0x2f;
  puStack_3f8 = &UNK_142accc5f;
  uStack_3f0 = 10;
  puStack_3e8 = &UNK_142accc69;
  uStack_3e0 = 0x20;
  puStack_3d8 = &UNK_142accc89;
  uStack_3d0 = 0xb;
  puStack_3c8 = &UNK_142accc94;
  uStack_3c0 = 0x21;
  puStack_3b8 = &UNK_142acccb5;
  uStack_3b0 = 0x17;
  puStack_3a8 = &UNK_142accccc;
  uStack_3a0 = 0x24;
  puStack_398 = &UNK_142acccf0;
  uStack_390 = 0x1e;
  puStack_388 = &UNK_142accd0e;
  uStack_380 = 0x2b;
  puStack_378 = &UNK_142accd39;
  uStack_370 = 0x24;
  puStack_368 = &UNK_142accd5d;
  uStack_360 = 0x31;
  puStack_358 = &UNK_142accd8e;
  uStack_350 = 0x1e;
  puStack_348 = &UNK_142accdac;
  uStack_340 = 0x2b;
  puStack_338 = &UNK_142accdd7;
  uStack_330 = 0x19;
  puStack_328 = &UNK_142accdf0;
  uStack_320 = 0x26;
  puStack_318 = &UNK_142acce16;
  uStack_310 = 0x13;
  puStack_308 = &UNK_142acce29;
  uStack_300 = 0x20;
  puStack_2f8 = &UNK_142acce49;
  uStack_2f0 = 0xf;
  puStack_2e8 = &UNK_142acce58;
  uStack_2e0 = 0x1c;
  puStack_2d8 = &UNK_142acce74;
  uStack_2d0 = 0x16;
  puStack_2c8 = &UNK_142acce8a;
  uStack_2c0 = 0x23;
  puStack_2b8 = &UNK_142accead;
  uStack_2b0 = 10;
  puStack_2a8 = &UNK_142acceb7;
  uStack_2a0 = 0x17;
  puStack_298 = &UNK_142accece;
  uStack_290 = 0xb;
  puStack_288 = &UNK_142acced9;
  uStack_280 = 0x18;
  puStack_278 = &UNK_142accef1;
  uStack_270 = 9;
  puStack_268 = &UNK_142accefa;
  uStack_260 = 0x16;
  puStack_258 = &UNK_142accf10;
  uStack_250 = 0x11;
  puStack_248 = &UNK_142accf21;
  uStack_240 = 0x1e;
  puStack_238 = &UNK_142accf3f;
  uStack_230 = 0x15;
  puStack_228 = &UNK_142accf54;
  uStack_220 = 0x22;
  puStack_218 = &UNK_142accf76;
  uStack_210 = 0x15;
  puStack_208 = &UNK_142accf8b;
  uStack_200 = 0x22;
  puStack_1f8 = &UNK_142accfad;
  uStack_1f0 = 0x1e;
  puStack_1e8 = &UNK_142accfcb;
  uStack_1e0 = 0x2b;
  puStack_1d8 = &UNK_142accff6;
  uStack_1d0 = 9;
  puStack_1c8 = &UNK_142accfff;
  uStack_1c0 = 0x1f;
  puStack_1b8 = &UNK_142acd01e;
  uStack_1b0 = 0x15;
  puStack_1a8 = &UNK_142acd033;
  uStack_1a0 = 0x22;
  puStack_198 = &UNK_142acd055;
  uStack_190 = 0xc;
  puStack_188 = &UNK_142acd061;
  uStack_180 = 0x19;
  puStack_178 = &UNK_142acd07a;
  uStack_170 = 10;
  puStack_168 = &UNK_142acd084;
  uStack_160 = 0x17;
  puStack_158 = &UNK_142acd09b;
  uStack_150 = 10;
  puStack_148 = &UNK_142acd0a5;
  uStack_140 = 0x17;
  puStack_138 = &UNK_142acd0bc;
  uStack_130 = 0x11;
  puStack_128 = &UNK_142acd0cd;
  uStack_120 = 0x1e;
  puStack_118 = &UNK_142acd0eb;
  uStack_110 = 9;
  puStack_108 = &UNK_142acd0f4;
  uStack_100 = 0x16;
  puStack_f8 = &UNK_142acd10a;
  uStack_f0 = 0x12;
  puStack_e8 = &UNK_142acd11c;
  uStack_e0 = 0x1f;
  uStack_d8 = &UNK_142acd13b;
  uStack_d0 = 0x11;
  puStack_c8 = &UNK_142acd14c;
  uStack_c0 = 0x1e;
  uStack_b0 = 0x1b;
  unaff_R14 = *in_R8;
  uStack_90 = *(undefined8 *)in_RDX;
  unaff_R15 = 1;
  do {
    lStack_b8 = unaff_R15;
    if (unaff_RBX[-3] == 0) break;
    uStack_548 = unaff_RBX[-2];
    uStack_538 = uStack_90;
    uStack_540 = unaff_R14;
code_r0x000140a38cb1:
    apuStack_578[1] = (undefined8 *)&UNK_140a38cbd;
    load_shader_into_library<ref_<str_>_>();
    unaff_RBX = unaff_RBX + 4;
    unaff_R15 = unaff_R15 + 1;
  } while (unaff_R15 != 0x1c);
  unaff_R14 = *(undefined8 *)in_stack_00000028;
code_r0x000140a38ccd:
  iStack_50 = *(int *)((longlong)in_stack_00000030 + 0x1c);
  unaff_R15 = *in_stack_00000030;
  lStack_78 = in_stack_00000030[2];
  lStack_98 = unaff_R15 + 0x1458;
code_r0x000140a38cf2:
  UNRECOVERED_JUMPTABLE = (code *)&UNK_140a38d1d;
code_r0x000140a38d0d:
  uStack_70 = unaff_R14;
  lStack_68 = unaff_R15;
                    // WARNING: Could not recover jumptable at 0x000140a38d1b. Too many branches
                    // WARNING: Treating indirect jump as call
  (*UNRECOVERED_JUMPTABLE)();
  return;
}


