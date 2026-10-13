// vermeer_pass  0x1408a82f0
// signature: void __cdecl vermeer_pass(void)
// callees: [{"name": "as_inner_mut", "address": "14067dd80"}, {"name": "bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408ee240"}, {"name": "bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408edd00"}, {"name": "drop", "address": "140be24b0"}, {"name": "extent_2d", "address": "140bd5790"}, {"name": "finish<country_core::systems::main_camera::MainCameraPlugin>", "address": "14003bb80"}, {"name": "get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$>_>", "address": "14096c890"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>", "address": "140562870"}, {"name": "log", "address": "140d4a330"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new<ref$<str$>_>", "address": "1416032a0"}, {"name": "new_2d", "address": "140bd52a0"}, {"name": "render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>", "address": "1409dd4d0"}, {"name": "sampled", "address": "140bd5370"}, {"name": "storage", "address": "140bd5390"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "unwrap_failed", "address": "1428d9310"}, {"name": "vermeer_render_from_target", "address": "1408a8d70"}]


void __cdecl country_core::systems::render_loop::vermeer::vermeer_pass(void)

{
  undefined4 *puVar1;
  longlong lVar2;
  undefined8 uVar3;
  ulonglong *puVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  longlong lVar5;
  longlong in_RDX;
  ulonglong uVar6;
  ulonglong *in_R8;
  float fVar7;
  undefined8 local_408;
  undefined *puStack_400;
  undefined8 local_3f8;
  undefined8 local_3f0;
  undefined *puStack_3e8;
  undefined8 local_3e0;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  undefined8 local_3c0;
  undefined **ppuStack_3b8;
  undefined8 local_3b0;
  undefined4 local_3a8;
  undefined4 local_3a4;
  undefined4 local_3a0;
  undefined4 local_39c;
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
  ulonglong *local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined *puStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined4 local_208;
  undefined8 local_200;
  undefined *puStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 local_1d0;
  undefined *puStack_1c8;
  undefined8 local_1c0;
  undefined4 local_1b8;
  undefined4 uStack_1b4;
  undefined **ppuStack_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  char local_ca;
  undefined8 local_c8;
  undefined8 local_80;
  undefined4 *local_70;
  int local_64;
  ulonglong *local_60;
  ulonglong *local_58;
  char local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uVar6 = 0xbe2e7c0aff5cc813;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 != 0)) {
    lVar5 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0) {
      lVar5 = *(longlong *)(in_RDX + 0x1e0) + lVar5;
      puVar1 = *(undefined4 **)(lVar5 + 0x10);
      *(undefined4 *)(lVar5 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
      uVar6 = 0xa284155e68bfeb3f;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_00 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
         (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 != 0)) {
        lVar5 = ~uVar6 * 0x60;
        if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0) {
          lVar5 = *(longlong *)(in_RDX + 0x1e0) + lVar5;
          local_60 = *(ulonglong **)(lVar5 + 0x10);
          *(undefined4 *)(lVar5 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
          local_64 = (int)local_60[9];
          if (local_64 == 2) {
            if (*(char *)(puVar1 + 1) != '\0') {
              *local_60 = 0;
              *(undefined1 *)((longlong)local_60 + 0x3e) = 7;
              local_60[8] = 0;
              *(undefined4 *)(local_60 + 9) = 0;
              *(undefined2 *)((longlong)local_60 + 0x54) = 0;
              return;
            }
            DAT_143634c9c = 0;
            return;
          }
          local_70 = puVar1;
          local_58 = in_R8;
          rhapsody::raw::texture::TextureDesc::extent_2d();
          DAT_143634c9c = 1;
          rhapsody::raw::texture::TextureDesc::new_2d();
          rhapsody::raw::texture::TextureDesc::storage();
          rhapsody::raw::texture::TextureDesc::sampled();
          rhapsody::rg::temporal::impl$11::
          get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
                    ();
          local_49 = local_1e0._4_1_;
          if (local_1e0._4_1_ != '\x02') {
            local_208 = (undefined4)local_1e0;
            local_218 = local_1f0;
            uStack_210 = uStack_1e8;
            local_228 = local_200;
            puStack_220 = puStack_1f8;
            if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_1e0 = *extraout_RAX_02;
              uStack_1d8 = extraout_RAX_02[1];
              uStack_1b4 = *(undefined4 *)(extraout_RAX_02 + 2);
              local_1d0 = 5;
              puStack_1c8 = &DAT_142a9bb70;
              local_1c0 = 0x2b;
              ppuStack_1b0 = &PTR_DAT_142a9bb48;
              local_1a8 = 1;
              local_1a0 = 8;
              local_200 = 0;
              uStack_198 = 0;
              uStack_190 = 0;
              puStack_1f8 = &DAT_142a9bb70;
              local_1f0 = 0x2b;
              uStack_1e8 = 0;
              local_1b8 = 1;
              ::log::__private_api::impl$2::log();
            }
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            local_278 = local_58;
            local_408 = CONCAT44(local_408._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_408,&local_200,400);
            local_278 = &local_228;
            local_200 = CONCAT44(local_200._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_200,&local_408,400);
            local_238 = local_160;
            uStack_230 = uStack_158;
            local_248 = local_170;
            uStack_240 = uStack_168;
            local_258 = local_180;
            uStack_250 = uStack_178;
            local_268 = uStack_190;
            uStack_260 = uStack_188;
            local_278 = (ulonglong *)local_1a0;
            uStack_270 = uStack_198;
            local_3f8 = local_1f0;
            local_408 = local_200;
            puStack_400 = puStack_1f8;
            local_3d8 = uStack_1e8;
            uStack_3d0 = local_1e0;
            local_3c8 = uStack_1d8;
            local_3f0 = local_1d0;
            puStack_3e8 = puStack_1c8;
            local_3e0 = local_1c0;
            local_390 = local_150;
            uStack_388 = uStack_148;
            local_380 = local_140;
            uStack_378 = uStack_138;
            local_370 = local_130;
            uStack_368 = uStack_128;
            local_360 = local_120;
            uStack_358 = uStack_118;
            local_350 = local_110;
            uStack_348 = uStack_108;
            local_340 = local_100;
            uStack_338 = uStack_f8;
            local_3b0 = local_1a8;
            local_3c0 = CONCAT44(uStack_1b4,local_1b8);
            ppuStack_3b8 = ppuStack_1b0;
            local_328 = local_80;
            local_330 = local_c8;
            local_3a8 = 1;
            local_3a4 = (undefined4)extraout_RAX_01;
            local_3a0 = (undefined4)((ulonglong)extraout_RAX_01 >> 0x20);
            local_39c = 1;
            rhapsody::rg::pass_builder::PassBuilder::
            render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                      ();
            puVar4 = local_60;
            puVar1 = local_70;
            if (local_ca != '\b') {
              rhapsody::raw::managed::handle::impl$6::drop();
            }
            rhapsody::raw::texture::TextureDesc::extent_2d();
            *(undefined4 *)(puVar4 + 9) = 1;
            *(undefined8 *)((longlong)puVar4 + 0x4c) = extraout_RAX_03;
            uVar6 = 0x109ca79d5cccc2d1;
            bevy_ecs::component::Components::get_resource_id();
            if (((extraout_RAX_04 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
               (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 != 0)) {
              lVar5 = ~uVar6 * 0x60;
              if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0) {
                fVar7 = *(float *)(*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar5 + 0x10) +
                                  0x58);
                if (0.033333335 <= fVar7) {
                  fVar7 = 0.033333335;
                }
                if (*(char *)((longlong)puVar1 + 5) == '\0') {
                  uVar6 = *puVar4;
                  if ((uVar6 < 2 | *(byte *)((longlong)puVar4 + 0x54)) != 1) {
                    *(float *)(puVar4 + 8) = fVar7 / 3.0 + *(float *)(puVar4 + 8);
                  }
                }
                else {
                  *(undefined4 *)(puVar4 + 8) = *puVar1;
                  uVar6 = *puVar4;
                }
                *puVar4 = uVar6 + 1;
                *(float *)((longlong)puVar4 + 0x44) = fVar7 + *(float *)((longlong)puVar4 + 0x44);
                vermeer_render_from_target();
                if (((1.0 <= *(float *)(puVar4 + 8)) && (*(char *)((longlong)puVar1 + 5) != '\x01'))
                   && (*(char *)((longlong)puVar4 + 0x54) == '\0')) {
                  if (((int)puVar4[9] != 2) && (*(char *)((longlong)local_60 + 0x3e) != '\a')) {
                    local_58 = puVar4 + 1;
                    bevy_app::plugin::Plugin::
                    finish<country_core::systems::main_camera::MainCameraPlugin>();
                    rhapsody::raw::managed::handle::impl$6::drop();
                  }
                  *(undefined4 *)(local_60 + 9) = 2;
                  uVar6 = 0xd8f93d064ea9d61c;
                  bevy_ecs::component::Components::get_resource_id();
                  if (((extraout_RAX_05 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
                     (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 != 0)
                     ) {
                    lVar5 = ~uVar6 * 0x60;
                    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0) {
                      lVar5 = *(longlong *)(in_RDX + 0x1e0) + lVar5;
                      lVar2 = *(longlong *)(lVar5 + 0x10);
                      *(undefined4 *)(lVar5 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
                      lVar5 = *(longlong *)(lVar2 + 0x30);
                      uVar3 = *(undefined8 *)(lVar2 + 0x40);
                      if (lVar5 == *(longlong *)(lVar2 + 0x20)) {
                        alloc::raw_vec::RawVec::
                        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                                  ();
                      }
                      *(undefined8 *)(*(longlong *)(lVar2 + 0x28) + lVar5 * 8) = uVar3;
                      *(longlong *)(lVar2 + 0x30) = lVar5 + 1;
                      *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
                      if (*(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E < 3)
                      {
                        return;
                      }
                      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                      local_1e0 = *extraout_RAX_06;
                      uStack_1d8 = extraout_RAX_06[1];
                      uStack_1b4 = *(undefined4 *)(extraout_RAX_06 + 2);
                      local_1d0 = 3;
                      puStack_1c8 = &DAT_142a9bb70;
                      local_1c0 = 0x2b;
                      ppuStack_1b0 = &PTR_s_Game_reveal_complete__142a9bc18;
                      local_1a8 = 1;
                      local_1a0 = 8;
                      local_200 = 0;
                      uStack_198 = 0;
                      uStack_190 = 0;
                      puStack_1f8 = &DAT_142a9bb70;
                      local_1f0 = 0x2b;
                      uStack_1e8 = 0;
                      local_1b8 = 1;
                      ::log::__private_api::impl$2::log();
                      return;
                    }
                  }
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                return;
              }
            }
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          local_408 = local_200;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

