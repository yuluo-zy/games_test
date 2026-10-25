// handle_screenshot_opportunity @ 0x14031d140
// sig: void __cdecl handle_screenshot_opportunity(void)


void __cdecl tiny_glade::screenshot::handle_screenshot_opportunity(void)

{
  longlong lVar1;
  char cVar2;
  char ***pppcVar3;
  undefined4 uVar4;
  char *pcVar5;
  byte bVar6;
  uint uVar7;
  undefined4 extraout_EAX;
  undefined4 extraout_EAX_00;
  ulonglong extraout_RAX;
  undefined8 *puVar8;
  char ***pppcVar9;
  ulonglong uVar10;
  undefined8 in_RCX;
  undefined8 in_R8;
  char ***in_R9;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  bool bVar14;
  float fVar15;
  undefined8 ***local_6e8;
  undefined8 ***pppuStack_6e0;
  undefined8 ***pppuStack_6d8;
  undefined8 ***pppuStack_6d0;
  undefined8 ***pppuStack_6c8;
  undefined8 ***local_6c0;
  ulonglong uStack_6b8;
  undefined4 local_6b0;
  undefined4 uStack_6ac;
  undefined8 local_6a8;
  undefined8 ***local_538;
  undefined8 ***pppuStack_530;
  undefined8 ***local_528;
  undefined8 ***pppuStack_520;
  undefined8 ***local_518;
  ulonglong uStack_510;
  char local_402;
  undefined8 ***local_3a8;
  undefined8 ***pppuStack_3a0;
  undefined8 ***local_398;
  undefined8 ***pppuStack_390;
  undefined8 ***local_388;
  undefined8 ***pppuStack_380;
  undefined8 ***local_378;
  undefined8 ***pppuStack_370;
  undefined8 ***local_368;
  undefined6 uStack_360;
  undefined2 uStack_35a;
  undefined6 uStack_358;
  undefined2 uStack_352;
  undefined8 local_348;
  char **local_1ec;
  undefined8 uStack_1e4;
  undefined8 local_1dc;
  undefined8 uStack_1d4;
  undefined4 local_1cc;
  undefined8 ***local_1c8;
  undefined8 ***pppuStack_1c0;
  undefined8 ***local_1b8;
  undefined8 ***pppuStack_1b0;
  undefined8 ***local_1a8;
  ulonglong uStack_1a0;
  undefined8 local_198;
  undefined8 local_190;
  longlong local_188;
  undefined8 local_180;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined1 local_158 [8];
  undefined8 ***pppuStack_150;
  undefined8 ***local_148;
  undefined8 ***pppuStack_140;
  undefined4 local_138;
  undefined8 ***local_128;
  undefined8 *local_120;
  longlong local_118;
  undefined8 local_110;
  undefined8 ***local_108;
  undefined8 ***pppuStack_100;
  undefined8 ***local_f8;
  undefined8 ***pppuStack_f0;
  undefined8 ***local_e8;
  char *local_c8;
  uint local_bc;
  undefined8 ***local_b8;
  ulonglong local_a8;
  longlong local_a0;
  ulonglong local_98;
  undefined8 *local_90;
  undefined1 local_84;
  undefined1 local_83;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  LOCK();
  bVar14 = ACTIVE_SCREENSHOT_REQUESTS == '\0';
  if (bVar14) {
    ACTIVE_SCREENSHOT_REQUESTS = '\x01';
  }
  UNLOCK();
  local_110 = in_R8;
  local_b8 = (undefined8 ***)in_R9;
  if (!bVar14) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (DAT_14362b068 == 0) {
LAB_14031e00b:
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar14 = ACTIVE_SCREENSHOT_REQUESTS == '\x01';
    if (bVar14) {
      ACTIVE_SCREENSHOT_REQUESTS = '\0';
    }
    UNLOCK();
  }
  else {
    local_118 = DAT_14362b060;
    local_188 = DAT_14362b068 * 0x98 + DAT_14362b060;
    uVar7 = (int)local_110 - 2;
    local_bc = uVar7 & 0xff;
    if (4 < (byte)uVar7) {
      local_bc = 3;
    }
    local_120 = (undefined8 *)((longlong)local_b8 + 4);
    lVar11 = local_188;
    local_190 = in_RCX;
    lVar1 = local_118;
    while (local_118 = lVar1, local_118 != lVar11) {
      bVar6 = *(char *)(local_118 + 0x4c) - 2;
      if (4 < bVar6) {
        bVar6 = 3;
      }
      lVar1 = local_118 + 0x98;
      if ((((byte)local_bc != 3 || *(char *)(local_118 + 0x4c) == (char)local_110) &&
          (bVar6 == (byte)local_bc)) && (*(char *)(local_118 + 0x96) == '\x06')) {
        local_a0 = local_118;
        local_118 = lVar1;
        rhapsody::raw::texture::TextureDesc::extent_2d();
        local_98 = extraout_RAX >> 0x20;
        uVar13 = extraout_RAX;
        if ((*(byte *)(local_a0 + 0x40) & 1) != 0) {
          uVar13 = (ulonglong)*(uint *)(local_a0 + 0x44);
          local_98 = (ulonglong)*(uint *)(local_a0 + 0x48);
        }
        if (*(longlong *)(local_a0 + 0x18) == -0x7fffffffffffffff) {
          local_90 = (undefined8 *)(uVar13 & 0xffffffff | local_98 << 0x20);
          local_a8 = uVar13;
          rhapsody::raw::texture::TextureDesc::new_2d();
          rhapsody::raw::texture::TextureDesc::storage();
          rhapsody::raw::texture::TextureDesc::sampled();
          local_108 = (undefined8 ***)0x0;
          rhapsody::raw::device::GraphicsDevice::
          create_texture<enum2_<rhapsody::raw::texture::TextureInitialData>_>();
          local_538 = pppuStack_6e0;
          pppuStack_530 = pppuStack_6d8;
          local_528 = pppuStack_6d0;
          pppuStack_520 = pppuStack_6c8;
          local_518 = local_6c0;
          uStack_510 = uStack_6b8;
          local_198 = CONCAT44(uStack_6ac,local_6b0);
          if (local_6e8 != (undefined8 ***)&DAT_0000000b) {
            local_348 = local_6a8;
            pppuStack_380 = pppuStack_6e0;
            local_378 = pppuStack_6d8;
            pppuStack_370 = pppuStack_6d0;
            local_368 = pppuStack_6c8;
            uStack_360 = SUB86(local_6c0,0);
            uStack_35a = (undefined2)((ulonglong)local_6c0 >> 0x30);
            uStack_358 = (undefined6)uStack_6b8;
            uStack_352 = (undefined2)(uStack_6b8 >> 0x30);
            local_388 = local_6e8;
            anyhow::error::impl$1::from<enum2_<rhapsody::raw::error::BackendError>_>();
            goto LAB_14031e00b;
          }
          local_1c8 = pppuStack_6e0;
          pppuStack_1c0 = pppuStack_6d8;
          local_1b8 = pppuStack_6d0;
          pppuStack_1b0 = pppuStack_6c8;
          local_1a8 = local_6c0;
          uStack_1a0 = uStack_6b8;
          local_81 = 1;
          rhapsody::rg::import_export::impl$0::import_unowned();
          log2f((float)(extraout_RAX >> 0x20) / (float)(local_98 & 0xffffffff));
          fVar15 = (float)roundf();
          uVar13 = 0;
          if (0.0 <= fVar15) {
            uVar13 = (longlong)(fVar15 - 9.223372e+18) & (longlong)fVar15 >> 0x3f | (longlong)fVar15
            ;
          }
          if (1.8446743e+19 < fVar15) {
            uVar13 = 0xffffffffffffffff;
          }
          if (uVar13 == 0) {
            local_81 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            local_108 = local_b8;
            local_6e8 = (undefined8 ***)CONCAT44(local_6e8._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_6e8,&local_388,400);
            local_108 = (undefined8 ***)&local_1ec;
            local_388 = (undefined8 ***)CONCAT44(local_388._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_538,&local_6e8,400);
            if (local_402 != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                        ();
            }
            memcpy(&local_388,&local_538,400);
            local_81 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d();
          }
          else {
            local_138._0_3_ = CONCAT12(7,(undefined2)local_138);
            uVar12 = 0;
            do {
              cVar2 = local_138._2_1_;
              puVar8 = (undefined8 *)(local_158 + 4);
              pppcVar3 = (char ***)local_158;
              if (local_138._2_1_ == '\a') {
                puVar8 = local_120;
                pppcVar3 = (char ***)local_b8;
              }
              local_388 = (undefined8 ***)*puVar8;
              pppuStack_380 = (undefined8 ***)puVar8[1];
              local_378 = (undefined8 ***)puVar8[2];
              pppuStack_370 = (undefined8 ***)puVar8[3];
              local_81 = 1;
              rhapsody::raw::texture::TextureDesc::half_res();
              local_81 = 1;
              rhapsody::raw::texture::TextureDesc::extent_2d();
              local_81 = 1;
              rhapsody::raw::texture::TextureDesc::new_2d();
              pppcVar9 = (char ***)local_158;
              if (cVar2 == '\a') {
                pppcVar9 = (char ***)local_b8;
              }
              cVar2 = *(char *)((longlong)pppcVar9 + 0x21);
              pppuStack_6d8 = local_398;
              pppuStack_6d0 = pppuStack_390;
              local_6e8 = local_3a8;
              pppuStack_6e0 = pppuStack_3a0;
              local_81 = 1;
              rhapsody::rg::resource::impl$4::from();
              local_388 = (undefined8 ***)&DAT_1429bcbc8;
              pppuStack_380 = (undefined8 ***)&DAT_00000013;
              local_81 = 1;
              rhapsody::rg::graph::RenderGraph::create_raw_resource();
              uStack_16c = SUB84(local_398,0);
              uStack_168 = (undefined4)((ulonglong)local_398 >> 0x20);
              uStack_164 = SUB84(pppuStack_390,0);
              uStack_160 = (undefined4)((ulonglong)pppuStack_390 >> 0x20);
              local_180._4_4_ = SUB84(local_3a8,0);
              uStack_178 = (undefined4)((ulonglong)local_3a8 >> 0x20);
              uStack_174 = SUB84(pppuStack_3a0,0);
              uStack_170 = (undefined4)((ulonglong)pppuStack_3a0 >> 0x20);
              local_180._0_4_ = extraout_EAX;
              if (cVar2 == '\x01') {
                local_81 = 1;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<ref_<str_>_>();
              }
              else {
                local_81 = 1;
                country_core::render::pass_builder::compute_pass_builder::ComputePass::
                new<ref_<str_>_>();
              }
              local_108._0_4_ = 2;
              local_128 = (undefined8 ***)pppcVar3;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_388,&local_538,400);
              local_128 = (undefined8 ***)&local_180;
              local_108 = (undefined8 ***)CONCAT44(local_108._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_6e8,&local_388,400);
              local_84 = 1;
              rhapsody::raw::texture::TextureDesc::extent_2d();
              in_RCX = local_190;
              local_84 = 0;
              country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d();
              uVar12 = uVar12 + 1;
              local_158 = (undefined1  [8])CONCAT44(local_180._4_4_,(undefined4)local_180);
              pppuStack_150 = (undefined8 ***)CONCAT44(uStack_174,uStack_178);
              local_148 = (undefined8 ***)CONCAT44(uStack_16c,uStack_170);
              pppuStack_140 = (undefined8 ***)CONCAT44(uStack_164,uStack_168);
              local_138 = uStack_160;
              uVar4 = local_138;
              local_180 = (undefined8 ****)local_158;
            } while (uVar12 != uVar13);
            local_138._2_1_ = (char)((uint)uStack_160 >> 0x10);
            local_138 = uVar4;
            if (local_138._2_1_ == '\a') {
              local_81 = 1;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            local_e8 = (undefined8 ***)CONCAT44(local_e8._4_4_,uStack_160);
            local_81 = 1;
            local_108 = (undefined8 ***)local_158;
            pppuStack_100 = (undefined8 ***)CONCAT44(uStack_174,uStack_178);
            local_f8 = (undefined8 ***)CONCAT44(uStack_16c,uStack_170);
            pppuStack_f0 = (undefined8 ***)CONCAT44(uStack_164,uStack_168);
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            local_180 = &local_108;
            local_6e8 = (undefined8 ***)CONCAT44(local_6e8._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_6e8,&local_388,400);
            local_180 = (undefined8 ****)&local_1ec;
            local_388 = (undefined8 ***)CONCAT44(local_388._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_538,&local_6e8,400);
            if (local_402 != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                        ();
            }
            memcpy(&local_388,&local_538,400);
            local_81 = 1;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d();
          }
          local_81 = 1;
          rhapsody::rg::resource_info::impl$13::from();
          local_6e8 = (undefined8 ***)0x0;
          pppuStack_6e0 = (undefined8 ***)&DAT_00000004;
          pppuStack_6d8 = (undefined8 ***)0x0;
          pppuStack_6d0 = (undefined8 ***)0x1429bc9e5;
          pppuStack_6c8 = (undefined8 ***)&DAT_0000000d;
          local_6c0 = (undefined8 ***)0x0;
          local_83 = 1;
          local_6b0 = extraout_EAX_00;
          local_6a8 = in_RCX;
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          local_83 = 1;
          rhapsody::rg::pass_builder::PassBuilder::
          add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
          local_348 = local_6a8;
          uStack_358 = (undefined6)uStack_6b8;
          uStack_352 = (undefined2)(uStack_6b8 >> 0x30);
          local_368 = pppuStack_6c8;
          uStack_360 = SUB86(local_6c0,0);
          uStack_35a = (undefined2)((ulonglong)local_6c0 >> 0x30);
          local_378 = pppuStack_6d8;
          pppuStack_370 = pppuStack_6d0;
          local_388 = local_6e8;
          pppuStack_380 = pppuStack_6e0;
          local_83 = 0;
          rhapsody::rg::pass_builder::PassBuilder::
          render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env_0>
                    ();
          local_368 = (undefined8 ***)CONCAT44(local_368._4_4_,local_1cc);
          local_378 = (undefined8 ***)local_1dc;
          pppuStack_370 = (undefined8 ***)uStack_1d4;
          local_388 = (undefined8 ***)local_1ec;
          pppuStack_380 = (undefined8 ***)uStack_1e4;
          local_81 = 1;
          rhapsody::rg::import_export::impl$0::export();
          uStack_358 = (undefined6)local_198;
          uStack_352 = (undefined2)((ulonglong)local_198 >> 0x30);
          local_368 = local_1a8;
          uStack_360 = (undefined6)uStack_1a0;
          uStack_35a = (undefined2)(uStack_1a0 >> 0x30);
          local_378 = local_1b8;
          pppuStack_370 = pppuStack_1b0;
          local_388 = local_1c8;
          pppuStack_380 = pppuStack_1c0;
          lVar1 = *(longlong *)(local_a0 + 0x20);
          LOCK();
          bVar14 = *(char *)(lVar1 + 0x10) == '\0';
          if (bVar14) {
            *(char *)(lVar1 + 0x10) = '\x01';
          }
          UNLOCK();
          if (!bVar14) {
            parking_lot::raw_mutex::RawMutex::lock_slow();
          }
          parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
          local_90 = (undefined8 *)(lVar1 + 0x18);
          local_c8 = (char *)(lVar1 + 0x10);
          if (*(char *)(lVar1 + 0x4e) != '\a') {
            bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>()
            ;
            rhapsody::raw::managed::handle::impl$6::drop();
          }
          pcVar5 = local_c8;
          local_90[6] = CONCAT26(uStack_352,uStack_358);
          local_90[4] = local_368;
          local_90[5] = CONCAT26(uStack_35a,uStack_360);
          local_90[2] = local_378;
          local_90[3] = pppuStack_370;
          *local_90 = local_388;
          local_90[1] = pppuStack_380;
          local_81 = 0;
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          bVar14 = *pcVar5 == '\x01';
          if (bVar14) {
            *pcVar5 = '\0';
          }
          UNLOCK();
          if (!bVar14) {
            local_81 = 0;
            parking_lot::raw_mutex::RawMutex::unlock_slow();
          }
          if (1 < (byte)(*(char *)(local_a0 + 0x96) - 5U)) {
            rhapsody::raw::managed::handle::impl$6::drop();
          }
          *(int *)(local_a0 + 0x58) = (int)local_a8;
          *(int *)(local_a0 + 0x5c) = (int)local_98;
          *(undefined8 ****)(local_a0 + 0x60) = local_388;
          *(undefined8 ****)(local_a0 + 0x68) = pppuStack_380;
          *(undefined8 ****)(local_a0 + 0x70) = local_378;
          *(undefined8 ****)(local_a0 + 0x78) = pppuStack_370;
          *(undefined8 ****)(local_a0 + 0x80) = local_368;
          *(ulonglong *)(local_a0 + 0x88) = CONCAT26(uStack_35a,uStack_360);
          *(ulonglong *)(local_a0 + 0x8e) = CONCAT62(uStack_358,uStack_35a);
          *(undefined1 *)(local_a0 + 0x96) = 5;
          lVar11 = local_188;
          lVar1 = local_118;
        }
        else {
          rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu();
          rhapsody::raw::device::GraphicsDevice::create_mapped_buffer();
          uVar12 = uStack_6b8;
          local_538 = pppuStack_6e0;
          pppuStack_530 = pppuStack_6d8;
          local_528 = pppuStack_6d0;
          pppuStack_520 = pppuStack_6c8;
          local_518 = local_6c0;
          if (local_6e8 != (undefined8 ***)&DAT_0000000b) {
            local_348 = local_6a8;
            pppuStack_380 = pppuStack_6e0;
            local_378 = pppuStack_6d8;
            pppuStack_370 = pppuStack_6d0;
            local_368 = pppuStack_6c8;
            uStack_360 = SUB86(local_6c0,0);
            uStack_35a = (undefined2)((ulonglong)local_6c0 >> 0x30);
            local_388 = local_6e8;
            uStack_358 = (undefined6)uStack_6b8;
            uStack_352 = (undefined2)(uStack_6b8 >> 0x30);
            anyhow::error::impl$1::from<enum2_<rhapsody::raw::error::BackendError>_>();
            goto LAB_14031e00b;
          }
          local_e8 = local_6c0;
          local_f8 = pppuStack_6d0;
          pppuStack_f0 = pppuStack_6c8;
          local_108 = pppuStack_6e0;
          pppuStack_100 = pppuStack_6d8;
          local_82 = 1;
          local_a8 = CONCAT44(uStack_6ac,local_6b0);
          rhapsody::rg::import_export::impl$1::import_unowned();
          local_82 = 1;
          country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
          local_1c8 = local_b8;
          local_6e8 = (undefined8 ***)CONCAT44(local_6e8._4_4_,2);
          country_core::render::pass_builder::compute_pass_builder::impl$4::
          bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&local_6e8,&local_388,400);
          local_82 = 1;
          country_core::render::pass_builder::compute_pass_builder::ComputePass::
          bind_external_storage_buffer();
          uVar10 = local_98 << 0x20;
          if (local_402 != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                      ();
          }
          memcpy(&local_388,&local_538,400);
          local_82 = 1;
          local_90 = (undefined8 *)(uVar13 & 0xffffffff | uVar10);
          country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d();
          local_82 = 1;
          rhapsody::rg::import_export::impl$1::export();
          local_98 = uVar12;
          local_368 = local_e8;
          local_388 = local_108;
          pppuStack_380 = pppuStack_100;
          local_378 = local_f8;
          pppuStack_370 = pppuStack_f0;
          if (1 < (byte)(*(char *)(local_a0 + 0x96) - 5U)) {
            local_c8 = (char *)(local_a0 + 0x70);
            rhapsody::raw::managed::handle::impl$6::drop();
          }
          *(undefined8 **)(local_a0 + 0x58) = local_90;
          *(ulonglong *)(local_a0 + 0x60) = local_98;
          *(ulonglong *)(local_a0 + 0x68) = local_a8;
          *(undefined8 ****)(local_a0 + 0x70) = local_388;
          *(undefined8 ****)(local_a0 + 0x78) = pppuStack_380;
          *(undefined8 ****)(local_a0 + 0x80) = local_378;
          *(undefined8 ****)(local_a0 + 0x88) = pppuStack_370;
          *(undefined8 ****)(local_a0 + 0x90) = local_368;
          lVar1 = local_118;
        }
      }
    }
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar14 = ACTIVE_SCREENSHOT_REQUESTS == '\x01';
    if (bVar14) {
      ACTIVE_SCREENSHOT_REQUESTS = '\0';
    }
    UNLOCK();
  }
  if (!bVar14) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  return;
}

