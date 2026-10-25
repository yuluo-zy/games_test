// 0x140a5a570 calc_dof size=1635

// WARNING: Control flow encountered bad instruction data

void __cdecl country_core::systems::render_loop::compute_passes::dof::calc_dof(void)

{
  char cVar1;
  char *pcVar2;
  int extraout_EAX;
  int extraout_EAX_00;
  ulonglong extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong extraout_RAX_01;
  int *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined1 *extraout_RAX_04;
  int *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *in_RCX;
  ulonglong uVar3;
  longlong lVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  ulonglong uVar8;
  undefined1 *puVar9;
  size_t _Size;
  longlong in_R8;
  int *piVar10;
  int iVar11;
  int *in_R9;
  int iVar12;
  bool bVar13;
  longlong in_stack_00000030;
  byte in_stack_00000038;
  int local_74c;
  int *local_748;
  undefined8 uStack_740;
  undefined8 local_738;
  undefined8 uStack_730;
  int iStack_728;
  undefined4 uStack_724;
  undefined4 uStack_720;
  undefined4 uStack_71c;
  undefined4 uStack_718;
  undefined4 uStack_714;
  uint uStack_710;
  undefined4 uStack_70c;
  undefined4 local_708;
  undefined8 uStack_700;
  undefined8 uStack_6f8;
  undefined8 uStack_6f0;
  ulonglong local_6e8;
  ulonglong local_6e0;
  int *local_6c8;
  undefined8 uStack_6c0;
  undefined8 local_6b8;
  undefined8 uStack_6b0;
  undefined8 local_6a8;
  undefined8 uStack_6a0;
  undefined8 local_698;
  undefined8 uStack_690;
  undefined8 local_688;
  undefined8 uStack_680;
  char cStack_592;
  longlong lStack_530;
  void *pvStack_528;
  ulonglong uStack_520;
  int iStack_518;
  int *piStack_514;
  undefined8 uStack_50c;
  undefined8 uStack_504;
  undefined8 uStack_4fc;
  int local_4e8;
  undefined4 uStack_4e4;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined8 uStack_4d8;
  undefined4 uStack_4d0;
  undefined4 uStack_4cc;
  undefined4 uStack_4c8;
  undefined4 uStack_4c4;
  undefined *puStack_4c0;
  undefined8 uStack_4b8;
  uint *puStack_4b0;
  undefined *puStack_4a8;
  undefined1 uStack_4a0;
  undefined *puStack_498;
  undefined8 uStack_490;
  uint *puStack_488;
  undefined *puStack_480;
  undefined1 uStack_478;
  int *piStack_468;
  undefined8 uStack_460;
  int *piStack_448;
  undefined8 uStack_440;
  int *piStack_428;
  undefined8 uStack_420;
  int *local_408;
  undefined8 uStack_400;
  undefined8 local_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 local_3e0;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  longlong local_3c0;
  undefined8 uStack_3b8;
  undefined8 local_3b0;
  undefined4 local_3a8;
  int local_3a4;
  undefined8 local_3a0;
  undefined8 uStack_398;
  undefined8 local_390;
  undefined8 uStack_388;
  undefined8 local_380;
  undefined8 uStack_378;
  undefined8 local_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 uStack_358;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  char local_2d2;
  undefined8 local_2d0;
  undefined8 uStack_288;
  uint uStack_26c;
  uint uStack_268;
  uint uStack_264;
  int local_260;
  int local_25c;
  int *piStack_258;
  ulonglong uStack_250;
  int *local_248;
  int *piStack_240;
  ulonglong local_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined4 local_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  char local_112;
  undefined8 local_110;
  undefined8 local_c8;
  undefined8 *local_b0;
  int *piStack_a8;
  undefined1 *puStack_a0;
  ulonglong uStack_98;
  ulonglong uStack_90;
  int *piStack_88;
  char *pcStack_80;
  longlong local_78;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_b0 = in_RCX;
  if (USE_DOF_MULTIRES == '\0') {
    rhapsody::raw::texture::TextureDesc::extent_2d();
    rhapsody::raw::texture::TextureDesc::extent_2d();
    rhapsody::raw::texture::TextureDesc::new_2d();
    local_3f8 = local_6b8;
    uStack_3f0 = uStack_6b0;
    local_408 = local_6c8;
    uStack_400 = uStack_6c0;
    rhapsody::rg::resource::impl$4::from();
    local_248 = (int *)0x142ad3193;
    piStack_240 = (int *)0xa;
    rhapsody::rg::graph::RenderGraph::create_raw_resource();
    uStack_4e4 = SUB84(local_6c8,0);
    uStack_4e0 = (undefined4)((ulonglong)local_6c8 >> 0x20);
    uStack_4dc = (undefined4)uStack_6c0;
    uStack_4d8._0_4_ = (undefined4)((ulonglong)uStack_6c0 >> 0x20);
    uStack_4d8._4_4_ = (undefined4)local_6b8;
    uStack_4d0 = (undefined4)((ulonglong)local_6b8 >> 0x20);
    uStack_4cc = (undefined4)uStack_6b0;
    uStack_4c8 = (undefined4)((ulonglong)uStack_6b0 >> 0x20);
    uVar8 = 0x4a142e09642375b9;
    local_4e8 = extraout_EAX;
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX_01 != 0) && (uVar8 < *(ulonglong *)(in_R8 + 0x218))) &&
        (uVar8 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar8 * 8), uVar8 != 0)) &&
       (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar8 * 0x60) != 0)) {
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      local_408._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_408,&local_248,400);
      local_248._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_248,&local_408,400);
      local_408._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_408,&local_248,400);
      local_6c8 = (int *)0x579f39ab91a01865;
      local_248 = (int *)CONCAT44(local_248._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>();
      memcpy(&local_248,&local_408,400);
      local_6c8 = &local_4e8;
      local_408 = (int *)CONCAT44(local_408._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_408,&local_248,400);
      if (local_2d2 != '\b') {
        country_core::render::pass_builder::uniform_binder::UniformBinder::
        write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                  ();
      }
      memcpy(&local_248,&local_408,400);
      local_688 = local_1a8;
      uStack_680 = uStack_1a0;
      local_698 = local_1b8;
      uStack_690 = uStack_1b0;
      local_6a8 = local_1c8;
      uStack_6a0 = uStack_1c0;
      local_6b8 = local_1d8;
      uStack_6b0 = uStack_1d0;
      local_3f8 = local_238;
      local_408 = local_248;
      uStack_400 = piStack_240;
      local_3d8 = uStack_230;
      uStack_3d0 = uStack_228;
      local_3c8 = local_220;
      uStack_3f0 = local_218;
      uStack_3e8 = uStack_210;
      local_3e0 = local_208;
      local_390 = local_198;
      uStack_388 = uStack_190;
      local_380 = local_188;
      uStack_378 = uStack_180;
      local_370 = local_178;
      uStack_368 = uStack_170;
      local_360 = local_168;
      uStack_358 = uStack_160;
      local_350 = local_158;
      uStack_348 = uStack_150;
      local_340 = local_148;
      uStack_338 = uStack_140;
      local_3b0 = local_1f0;
      local_3c0 = local_200;
      uStack_3b8 = uStack_1f8;
      local_328 = local_c8;
      local_330 = local_110;
      local_3a8 = 1;
      local_3a4 = (int)extraout_RAX_00;
      local_3a0 = CONCAT44(1,(int)((ulonglong)extraout_RAX_00 >> 0x20));
      rhapsody::rg::pass_builder::PassBuilder::
      render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                ();
      if (local_112 != '\b') {
        rhapsody::raw::managed::handle::impl$6::drop();
      }
      *(undefined4 *)(local_b0 + 4) = uStack_4c8;
      local_b0[2] = CONCAT44(uStack_4d8._4_4_,(undefined4)uStack_4d8);
      local_b0[3] = CONCAT44(uStack_4cc,uStack_4d0);
      *local_b0 = CONCAT44(uStack_4e4,local_4e8);
      local_b0[1] = CONCAT44(uStack_4dc,uStack_4e0);
      return;
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_78 = in_R8;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_248 = *(int **)(in_R9 + 1);
  piStack_240 = *(int **)(in_R9 + 3);
  local_238 = *(ulonglong *)(in_R9 + 5);
  uStack_230 = *(undefined8 *)(in_R9 + 7);
  rhapsody::raw::texture::TextureDesc::half_res();
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_248 = *(int **)(in_stack_00000030 + 0x28);
  piStack_240 = *(int **)(in_stack_00000030 + 0x30);
  local_238 = *(ulonglong *)(in_stack_00000030 + 0x38);
  uStack_230 = *(undefined8 *)(in_stack_00000030 + 0x40);
  rhapsody::raw::texture::TextureDesc::format();
  local_3f8 = local_6b8;
  uStack_3f0 = uStack_6b0;
  local_408 = local_6c8;
  uStack_400 = uStack_6c0;
  rhapsody::rg::resource::impl$4::from();
  local_248 = (int *)&DAT_142ad3218;
  piStack_240 = (int *)0xb;
  rhapsody::rg::graph::RenderGraph::create_raw_resource();
  local_748 = local_6c8;
  uStack_740 = uStack_6c0;
  local_738 = local_6b8;
  uStack_730 = uStack_6b0;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_6e0 = extraout_RAX >> 0x20;
  uVar5 = (uint)(extraout_RAX >> 0x20);
  local_260 = (uVar5 - (uVar5 >> 1)) *
              ((int)extraout_RAX - ((uint)(extraout_RAX >> 1) & 0x7fffffff));
  local_25c = uVar5 * (int)extraout_RAX;
  iVar12 = 0;
LAB_140a5abe7:
  lVar4 = 1;
  do {
    iVar12 = iVar12 + 1;
    piVar10 = &DAT_142ad3614 + lVar4;
    bVar13 = (longlong)&DAT_142ad3614 + (longlong)*piVar10 == 0;
    lVar4 = 2;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
    switch((longlong)&DAT_142ad3614 + (longlong)*piVar10) {
    case 0x140a5abd3:
      lVar4 = 3;
      break;
    case 0x140a5abd7:
      lVar4 = 4;
      break;
    case 0x140a5abdb:
      lVar4 = 5;
      break;
    case 0x140a5abdf:
      lVar4 = 6;
      break;
    case 0x140a5abe3:
      lVar4 = 7;
      break;
    case 0x140a5abe7:
      goto LAB_140a5abe7;
    case 0x140a5ac09:
      local_248 = &local_25c;
      piStack_240 = &local_260;
      local_238 = local_238 & 0xffffffffffffff00;
      local_6e8 = extraout_RAX;
      piStack_258 = in_R9;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env_0>_>
                ();
      uVar8 = uStack_520 * 4;
      if ((uStack_520 >> 0x3e == 0) && (uVar8 < 0x7ffffffffffffffd)) {
        uStack_90 = uVar8;
        if (uVar8 == 0) {
          piStack_88 = (int *)&DAT_00000004;
          uStack_250 = 0;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_02 == (int *)0x0) goto code_r0x000140a5c571;
          uStack_250 = uStack_520;
          piStack_88 = extraout_RAX_02;
        }
        memcpy(piStack_88,pvStack_528,uStack_90);
        if (uStack_520 != 0) {
          uVar3 = (uStack_90 - 4 >> 2) + 1;
          uVar8 = (ulonglong)((uint)uVar3 & 7);
          piVar10 = piStack_88;
          if (uStack_90 - 4 < 0x1c) {
            iVar7 = 0;
          }
          else {
            uVar3 = uVar3 & 0xfffffffffffffff8;
            iVar7 = 0;
            do {
              iVar11 = *piVar10;
              *piVar10 = iVar7;
              iVar6 = piVar10[1] + iVar11 + iVar7;
              piVar10[1] = iVar11 + iVar7;
              iVar11 = piVar10[2] + iVar6;
              piVar10[2] = iVar6;
              iVar7 = piVar10[3] + iVar11;
              piVar10[3] = iVar11;
              iVar11 = piVar10[4] + iVar7;
              piVar10[4] = iVar7;
              iVar7 = piVar10[5] + iVar11;
              piVar10[5] = iVar11;
              iVar11 = piVar10[6] + iVar7;
              piVar10[6] = iVar7;
              iVar7 = piVar10[7];
              piVar10[7] = iVar11;
              piVar10 = piVar10 + 8;
              iVar7 = iVar7 + iVar11;
              uVar3 = uVar3 - 8;
            } while (uVar3 != 0);
          }
          if (uVar8 != 0) {
            uVar3 = 0;
            do {
              iVar11 = piVar10[uVar3];
              piVar10[uVar3] = iVar7;
              uVar3 = uVar3 + 1;
              iVar7 = iVar11 + iVar7;
            } while (uVar8 != uVar3);
          }
        }
        rhapsody::raw::buffer::BufferDesc::new_gpu_only();
        local_408 = local_6c8;
        uStack_400 = uStack_6c0;
        rhapsody::rg::resource::impl$5::from();
        local_248 = (int *)&UNK_142ad3223;
        piStack_240 = (int *)0x10;
        rhapsody::rg::graph::RenderGraph::create_raw_resource();
        piStack_428 = local_6c8;
        uStack_420 = uStack_6c0;
        rhapsody::raw::buffer::BufferDesc::new_gpu_only();
        local_408 = local_6c8;
        uStack_400 = uStack_6c0;
        rhapsody::rg::resource::impl$5::from();
        local_248 = (int *)0x142ad3233;
        piStack_240 = (int *)0xf;
        rhapsody::rg::graph::RenderGraph::create_raw_resource();
        piStack_468 = local_6c8;
        uStack_460 = uStack_6c0;
        rhapsody::raw::buffer::BufferDesc::new_gpu_only();
        local_408 = local_6c8;
        uStack_400 = uStack_6c0;
        rhapsody::rg::resource::impl$5::from();
        local_248 = (int *)0x142ad3242;
        piStack_240 = (int *)0x11;
        rhapsody::rg::graph::RenderGraph::create_raw_resource();
        piStack_448 = local_6c8;
        uStack_440 = uStack_6c0;
        uVar8 = 0x47c94cb78283a72e;
        bevy_ecs::component::Components::get_resource_id();
        if (((extraout_RAX_03 != 0) && (uVar8 < *(ulonglong *)(local_78 + 0x218))) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(local_78 + 0x210) + uVar8 * 8), uVar8 != 0)) {
          lVar4 = ~uVar8 * 0x60;
          if (*(longlong *)(*(longlong *)(local_78 + 0x1e0) + 0x20 + lVar4) != 0) {
            pcStack_80 = *(char **)(*(longlong *)(local_78 + 0x1e0) + lVar4 + 0x10);
            if (uStack_90 == 0) {
              puVar9 = &DAT_00000004;
              uVar8 = 0;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc();
              puVar9 = extraout_RAX_04;
              uVar8 = uStack_520;
              if (extraout_RAX_04 == (undefined1 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
            }
            memcpy(puVar9,piStack_88,uStack_90);
            piStack_a8 = (int *)uVar8;
            puStack_a0 = puVar9;
            for (; uStack_98 = uStack_520, (uStack_520 & 3) != 0; uStack_520 = uStack_520 + 1) {
              if ((int *)uStack_520 == piStack_a8) {
                alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
              }
              *(undefined4 *)(puStack_a0 + uStack_520 * 4) = 0;
            }
            LOCK();
            cVar1 = *pcStack_80;
            if (cVar1 == '\0') {
              *pcStack_80 = '\x01';
            }
            UNLOCK();
            if (cVar1 != '\0') {
              parking_lot::raw_mutex::RawMutex::lock_slow();
            }
            parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
            puVar9 = puStack_a0;
            _Size = uStack_98 << 2;
            rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
            if (piStack_240 != (int *)_Size) {
                    // WARNING: Subroutine does not return
              core::slice::impl$0::copy_from_slice::len_mismatch_fail();
            }
            memcpy(local_248,puVar9,_Size);
            pcVar2 = pcStack_80;
            local_4e8 = (int)local_248;
            uStack_4e4 = (undefined4)((ulonglong)local_248 >> 0x20);
            uStack_4e0 = SUB84(piStack_240,0);
            uStack_4dc = (undefined4)((ulonglong)piStack_240 >> 0x20);
            uStack_4d8._0_4_ = (undefined4)local_238;
            uStack_4d8._4_4_ = (undefined4)(local_238 >> 0x20);
            uStack_4d0 = (undefined4)uStack_230;
            uStack_4cc = (undefined4)((ulonglong)uStack_230 >> 0x20);
            uStack_4c8 = (undefined4)uStack_228;
            uStack_4c4 = (undefined4)((ulonglong)uStack_228 >> 0x20);
            puStack_4c0 = (undefined *)local_220;
            uStack_4b8 = local_218;
            puStack_4b0 = (uint *)uStack_210;
            puStack_4a8 = (undefined *)local_208;
            uStack_62 = 1;
            parking_lot_core::parking_lot::deadlock_impl::release_resource();
            LOCK();
            bVar13 = *pcVar2 == '\x01';
            if (bVar13) {
              *pcVar2 = '\0';
            }
            UNLOCK();
            if (!bVar13) {
              uStack_62 = 1;
              parking_lot::raw_mutex::RawMutex::unlock_slow();
            }
            local_208 = puStack_4a8;
            local_248 = (int *)CONCAT44(uStack_4e4,local_4e8);
            piStack_240 = (int *)CONCAT44(uStack_4dc,uStack_4e0);
            local_238 = CONCAT44(uStack_4d8._4_4_,(undefined4)uStack_4d8);
            uStack_230 = CONCAT44(uStack_4cc,uStack_4d0);
            uStack_228 = CONCAT44(uStack_4c4,uStack_4c8);
            local_218 = uStack_4b8;
            uStack_210 = puStack_4b0;
            local_220 = puStack_4c0;
            uStack_62 = 0;
            rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding();
            uStack_61 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            uStack_61 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            write_rg_storage_buffer();
            uStack_61 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            write_rg_storage_buffer();
            local_698 = uStack_6f8;
            uStack_690 = uStack_6f0;
            uStack_6a0 = uStack_700;
            uStack_61 = 0;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_raw
                      ();
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_05 == (int *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::raw_vec::handle_error();
            }
            *extraout_RAX_05 = iVar12;
            if (local_3c0 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            local_3c0 = 4;
            local_3b0 = 4;
            memcpy(&local_248,&local_408,400);
            local_688 = local_1a8;
            uStack_680 = uStack_1a0;
            local_698 = local_1b8;
            uStack_690 = uStack_1b0;
            local_6a8 = local_1c8;
            uStack_6a0 = uStack_1c0;
            local_6b8 = local_1d8;
            uStack_6b0 = uStack_1d0;
            local_3f8 = local_238;
            local_408 = local_248;
            uStack_400 = piStack_240;
            local_3d8 = uStack_230;
            uStack_3d0 = uStack_228;
            local_3c8 = local_220;
            uStack_3f0 = local_218;
            uStack_3e8 = uStack_210;
            local_3e0 = local_208;
            local_390 = local_198;
            uStack_388 = uStack_190;
            local_380 = local_188;
            uStack_378 = uStack_180;
            local_370 = local_178;
            uStack_368 = uStack_170;
            local_360 = local_168;
            uStack_358 = uStack_160;
            local_350 = local_158;
            uStack_348 = uStack_150;
            local_340 = local_148;
            uStack_338 = uStack_140;
            local_3b0 = local_1f0;
            local_3c0 = local_200;
            uStack_3b8 = uStack_1f8;
            local_328 = local_c8;
            local_330 = local_110;
            local_3a8 = 1;
            local_3a0 = 0x100000001;
            local_3a4 = iVar12;
            rhapsody::rg::pass_builder::PassBuilder::
            render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                      ();
            if (local_112 != '\b') {
              uStack_61 = 0;
              rhapsody::raw::managed::handle::impl$6::drop();
            }
            if (piStack_a8 != (int *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            local_408 = (int *)CONCAT44(local_408._4_4_,2);
            local_6c8 = (int *)(in_stack_00000030 + 0x24);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_408,&local_248,400);
            local_6c8 = &local_74c;
            local_248 = (int *)CONCAT44(local_248._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_248,&local_408,400);
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            read_rg_storage_buffer();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            write_rg_storage_buffer();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            write_rg_storage_buffer();
            local_688 = uStack_368;
            uStack_680 = local_360;
            local_698 = uStack_378;
            uStack_690 = local_370;
            local_6a8 = uStack_388;
            uStack_6a0 = local_380;
            local_6b8 = uStack_398;
            uStack_6b0 = local_390;
            local_6c8 = (int *)CONCAT44(local_3a4,local_3a8);
            uStack_6c0 = local_3a0;
            local_238 = local_3f8;
            local_248 = local_408;
            piStack_240 = (int *)uStack_400;
            local_218 = uStack_3f0;
            uStack_210 = uStack_3e8;
            local_208 = local_3e0;
            uStack_230 = local_3d8;
            uStack_228 = uStack_3d0;
            local_220 = local_3c8;
            uStack_1d0 = uStack_358;
            local_1c8 = local_350;
            uStack_1c0 = uStack_348;
            local_1b8 = local_340;
            uStack_1b0 = uStack_338;
            local_1a8 = local_330;
            uStack_1a0 = local_328;
            local_198 = uStack_320;
            uStack_190 = uStack_318;
            local_188 = uStack_310;
            uStack_180 = uStack_308;
            local_178 = uStack_300;
            local_1f0 = local_3b0;
            local_200 = local_3c0;
            uStack_1f8 = uStack_3b8;
            local_168 = uStack_288;
            uStack_170 = local_2d0;
            local_1e8 = 1;
            uStack_1e4 = (undefined4)local_6e8;
            uStack_1e0 = (undefined4)local_6e0;
            uStack_1dc = 1;
            rhapsody::rg::pass_builder::PassBuilder::
            render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                      ();
            if (local_2d2 != '\b') {
              rhapsody::raw::managed::handle::impl$6::drop();
            }
            rhapsody::raw::texture::TextureDesc::new_2d();
            local_3f8 = local_6b8;
            uStack_3f0 = uStack_6b0;
            local_408 = local_6c8;
            uStack_400 = uStack_6c0;
            rhapsody::rg::resource::impl$4::from();
            local_248 = (int *)&UNK_142ad32b8;
            piStack_240 = (int *)0xb;
            rhapsody::rg::graph::RenderGraph::create_raw_resource();
            piStack_514 = local_6c8;
            uStack_50c = uStack_6c0;
            uStack_504 = local_6b8;
            uStack_4fc = uStack_6b0;
            rhapsody::raw::texture::TextureDesc::new_2d();
            local_3f8 = local_6b8;
            uStack_3f0 = uStack_6b0;
            local_408 = local_6c8;
            uStack_400 = uStack_6c0;
            rhapsody::rg::resource::impl$4::from();
            local_248 = (int *)&UNK_142ad32c3;
            piStack_240 = (int *)0xa;
            rhapsody::rg::graph::RenderGraph::create_raw_resource();
            uStack_724 = SUB84(local_6c8,0);
            uStack_720 = (undefined4)((ulonglong)local_6c8 >> 0x20);
            uStack_71c = (undefined4)uStack_6c0;
            uStack_718 = (undefined4)((ulonglong)uStack_6c0 >> 0x20);
            uStack_714 = (undefined4)local_6b8;
            uStack_710 = (uint)((ulonglong)local_6b8 >> 0x20);
            uStack_70c = (undefined4)uStack_6b0;
            local_708 = (undefined4)((ulonglong)uStack_6b0 >> 0x20);
            iStack_728 = extraout_EAX_00;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            read_rg_storage_buffer();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            read_rg_storage_buffer();
            local_6c8 = &iStack_518;
            local_408._0_4_ = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_408,&local_248,400);
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            dispatch_indirect_raw();
            uVar8 = 0x4a142e09642375b9;
            bevy_ecs::component::Components::get_resource_id();
            if ((((extraout_RAX_06 != 0) && (uVar8 < *(ulonglong *)(local_78 + 0x218))) &&
                (uVar8 = *(ulonglong *)(*(longlong *)(local_78 + 0x210) + uVar8 * 8), uVar8 != 0))
               && (*(longlong *)(*(longlong *)(local_78 + 0x1e0) + 0x20 + ~uVar8 * 0x60) != 0)) {
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<country_core::systems::render_loop::compute_passes::dof::HalfResDof>();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              local_6c8 = piStack_258;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              local_248._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = &local_74c;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              local_248 = (int *)CONCAT44(local_248._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = (int *)0x579f39ab91a01865;
              local_408 = (int *)CONCAT44(local_408._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
                        ();
              memcpy(&local_408,&local_248,400);
              if (local_2d2 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                          ();
              }
              memcpy(&local_248,&local_408,400);
              local_6c8 = &iStack_518;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              dispatch_indirect_raw();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              local_6c8 = piStack_258;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              local_248._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = &local_74c;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              local_248._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = (int *)0x579f39ab91a01865;
              local_408 = (int *)CONCAT44(local_408._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
                        ();
              memcpy(&local_408,&local_248,400);
              if (local_2d2 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                          ();
              }
              memcpy(&local_248,&local_408,400);
              local_6c8 = &iStack_518;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              dispatch_indirect_raw();
              uStack_264 = (uint)in_stack_00000038;
              uStack_268 = (uint)in_stack_00000038;
              uStack_26c = (uint)in_stack_00000038;
              local_4e8 = 0x42ad3348;
              uStack_4e4 = 1;
              uStack_4e0 = 0x14;
              uStack_4dc = 0;
              uStack_4d8 = &uStack_264;
              uStack_4d0 = 0x42ad3320;
              uStack_4cc = 1;
              uStack_4c8 = CONCAT31(uStack_4c8._1_3_,1);
              puStack_4c0 = &UNK_142ad3388;
              uStack_4b8 = 0x14;
              puStack_4b0 = &uStack_268;
              puStack_4a8 = &UNK_142ad3360;
              uStack_4a0 = 2;
              puStack_498 = &UNK_142ad33c8;
              uStack_490 = 0xb;
              puStack_488 = &uStack_26c;
              puStack_480 = &UNK_142ad33a0;
              uStack_478 = 0;
              lVar4 = 0;
              do {
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<ref_<dyn_<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>_>_>
                          ();
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                read_rg_storage_buffer();
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                read_rg_storage_buffer();
                piStack_a8 = piStack_258;
                local_408._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_408,&local_248,400);
                local_248._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_248,&local_408,400);
                piStack_a8 = &local_74c;
                local_408._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_408,&local_248,400);
                local_248 = (int *)CONCAT44(local_248._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_248,&local_408,400);
                piStack_a8 = (int *)0x579f39ab91a01865;
                local_408 = (int *)CONCAT44(local_408._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
                          ();
                memcpy(&local_6c8,&local_248,400);
                if (cStack_592 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                            ();
                }
                memcpy(&local_248,&local_6c8,400);
                country_core::render::pass_builder::compute_pass_builder::impl$3::
                read_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo>
                          ();
                local_6c8 = &iStack_518;
                local_248._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_248,&local_408,400);
                local_6c8 = &iStack_728;
                local_408._0_4_ = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                          ();
                memcpy(&local_408,&local_248,400);
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                dispatch_indirect_raw();
                lVar4 = lVar4 + 0x28;
              } while (lVar4 != 0x78);
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<ref_<str_>_>();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              read_rg_storage_buffer();
              country_core::render::pass_builder::compute_pass_builder::impl$3::
              read_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo>
                        ();
              local_6c8 = (int *)0x579f39ab91a01865;
              local_248._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = &iStack_518;
              local_408._0_4_ = 2;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              local_248 = (int *)CONCAT44(local_248._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_248,&local_408,400);
              local_6c8 = &iStack_728;
              local_408 = (int *)CONCAT44(local_408._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_408,&local_248,400);
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              dispatch_indirect_raw();
              *(undefined4 *)(local_b0 + 4) = local_708;
              local_b0[2] = CONCAT44(uStack_714,uStack_718);
              local_b0[3] = CONCAT44(uStack_70c,uStack_710);
              *local_b0 = CONCAT44(uStack_724,iStack_728);
              local_b0[1] = CONCAT44(uStack_71c,uStack_720);
              if (uStack_250 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (lStack_530 == 0) {
                return;
              }
              std::alloc::__default_lib_allocator::__rust_dealloc();
              return;
            }
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
code_r0x000140a5c571:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    case 0x142ad3614:
      bVar13 = (uStack_710 & 0x3fffffff) == 0;
      goto code_r0x000142ad361b;
    case 0x142ad3615:
      if (!bVar13) {
                    // WARNING: Could not recover jumptable at 0x000142ad3610. Too many branches
                    // WARNING: Treating indirect jump as call
        (**(code **)(&UNK_142ad3611 + in_stack_00000030 * 8))();
        return;
      }
code_r0x000142ad361b:
      do {
      } while (!bVar13);
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
  } while( true );
}

