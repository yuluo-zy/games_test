
void __cdecl
country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::
highlight_active_terrain_stroke(void)

{
  uint uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar4;
  longlong in_RDX;
  ulonglong uVar5;
  int iVar6;
  longlong lVar7;
  undefined1 local_3ec [4];
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined *local_3c8;
  undefined8 uStack_3c0;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 local_3a0;
  undefined8 local_398;
  undefined8 uStack_390;
  undefined8 local_388;
  undefined8 local_380;
  undefined8 uStack_378;
  undefined8 local_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  char local_292;
  undefined8 local_290;
  undefined8 local_248;
  undefined *local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  char local_102;
  undefined8 local_100;
  undefined8 local_b8;
  undefined1 *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  uVar5 = 0xc2b8e941c91a6428;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar5 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar4 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar4) != 0) {
      uVar1 = **(uint **)(*(longlong *)(in_RDX + 0x1e0) + lVar4 + 0x10);
      lVar4 = 0;
      if (uVar1 - 6 < 0xf) {
        lVar4 = (ulonglong)uVar1 - 5;
      }
      iVar6 = -1;
      if ((lVar4 != 5) && ((lVar4 != 0 || (uVar1 != 5)))) {
LAB_140a585cc:
        uVar5 = 0x48b321ef779dc301;
        bevy_ecs::component::Components::get_resource_id();
        if (((extraout_RAX_01 != 0) && (uVar5 < *(ulonglong *)(in_RDX + 0x218))) &&
           (uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar5 * 8), uVar5 != 0)) {
          lVar4 = ~uVar5 * 0x60;
          if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar4) != 0) {
            if (*(char *)(*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar4 + 0x10) + 0xf9) !=
                '\0') {
              iVar6 = -1;
            }
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            if (local_102 != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                        ();
            }
            memcpy(&local_3c8,&local_238,400);
            local_a8._0_1_ = 0xb;
            local_238 = (undefined *)CONCAT44(local_238._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
            memcpy(&local_238,&local_3c8,400);
            local_a8 = (undefined1 *)CONCAT71(local_a8._1_7_,0xc);
            local_3c8 = (undefined *)CONCAT44(local_3c8._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
            memcpy(&local_3c8,&local_238,400);
            local_68 = local_328;
            uStack_60 = uStack_320;
            local_78 = local_338;
            uStack_70 = uStack_330;
            local_88 = local_348;
            uStack_80 = uStack_340;
            local_98 = local_358;
            uStack_90 = uStack_350;
            local_a8 = local_368;
            uStack_a0 = uStack_360;
            local_228 = local_3b8;
            local_238 = local_3c8;
            uStack_230 = uStack_3c0;
            local_208 = uStack_3b0;
            uStack_200 = uStack_3a8;
            local_1f8 = local_3a0;
            local_220 = local_398;
            uStack_218 = uStack_390;
            local_210 = local_388;
            local_1c0 = local_318;
            uStack_1b8 = uStack_310;
            local_1b0 = local_308;
            uStack_1a8 = uStack_300;
            local_1a0 = local_2f8;
            uStack_198 = uStack_2f0;
            local_190 = local_2e8;
            uStack_188 = uStack_2e0;
            local_180 = local_2d8;
            uStack_178 = uStack_2d0;
            local_170 = local_2c8;
            uStack_168 = uStack_2c0;
            local_1e0 = local_370;
            local_1f0 = local_380;
            uStack_1e8 = uStack_378;
            local_158 = local_248;
            local_160 = local_290;
            local_1d8 = 0x10000000001;
            uStack_1d0 = 0x100000100;
            rhapsody::rg::pass_builder::PassBuilder::
            render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                      ();
            if (local_292 != '\b') {
              rhapsody::raw::managed::handle::impl$6::drop();
            }
            if (iVar6 != -1) {
              rhapsody::raw::texture::TextureDesc::new_2d();
              local_3b8 = local_98;
              uStack_3b0 = uStack_90;
              local_3c8 = local_a8;
              uStack_3c0 = uStack_a0;
              rhapsody::rg::resource::impl$4::from();
              local_238 = &DAT_142ad2e99;
              uStack_230 = 9;
              rhapsody::rg::graph::RenderGraph::create_raw_resource();
              local_3e8 = local_a8;
              uStack_3e0 = uStack_a0;
              local_3d8 = local_98;
              uStack_3d0 = uStack_90;
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<ref_<str_>_>();
              if (local_102 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                          ();
              }
              memcpy(&local_3c8,&local_238,400);
              if (local_292 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                          ();
              }
              memcpy(&local_238,&local_3c8,400);
              local_a8 = (undefined1 *)CONCAT71(local_a8._1_7_,0xc);
              local_3c8 = (undefined *)CONCAT44(local_3c8._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                        ();
              memcpy(&local_3c8,&local_238,400);
              local_238 = (undefined *)CONCAT44(local_238._4_4_,2);
              local_a8 = local_3ec;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_238,&local_3c8,400);
              local_68 = uStack_198;
              uStack_60 = local_190;
              local_78 = uStack_1a8;
              uStack_70 = local_1a0;
              local_88 = uStack_1b8;
              uStack_80 = local_1b0;
              local_98 = uStack_1c8;
              uStack_90 = local_1c0;
              local_a8 = (undefined1 *)local_1d8;
              uStack_a0 = uStack_1d0;
              local_3b8 = local_228;
              local_3c8 = local_238;
              uStack_3c0 = uStack_230;
              local_398 = local_220;
              uStack_390 = uStack_218;
              local_388 = local_210;
              uStack_3b0 = local_208;
              uStack_3a8 = uStack_200;
              local_3a0 = local_1f8;
              uStack_350 = uStack_188;
              local_348 = local_180;
              uStack_340 = uStack_178;
              local_338 = local_170;
              uStack_330 = uStack_168;
              local_328 = local_160;
              uStack_320 = local_158;
              local_318 = uStack_150;
              uStack_310 = local_148;
              local_308 = uStack_140;
              uStack_300 = local_138;
              local_2f8 = uStack_130;
              local_370 = local_1e0;
              local_380 = local_1f0;
              uStack_378 = uStack_1e8;
              local_2e8 = local_b8;
              uStack_2f0 = local_100;
              local_368 = (undefined *)0x10000000001;
              uStack_360 = 0x100000100;
              rhapsody::rg::pass_builder::PassBuilder::
              render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                        ();
              if (local_102 != '\b') {
                rhapsody::raw::managed::handle::impl$6::drop();
              }
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<ref_<str_>_>();
              if (local_102 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                          ();
              }
              memcpy(&local_3c8,&local_238,400);
              if (local_292 != '\b') {
                country_core::render::pass_builder::uniform_binder::UniformBinder::
                write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                          ();
              }
              memcpy(&local_238,&local_3c8,400);
              local_3c8 = (undefined *)CONCAT44(local_3c8._4_4_,2);
              local_a8 = local_3ec;
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                        ();
              memcpy(&local_3c8,&local_238,400);
              local_a8 = (undefined1 *)CONCAT71(local_a8._1_7_,0xc);
              local_238 = (undefined *)CONCAT44(local_238._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                        ();
              memcpy(&local_238,&local_3c8,400);
              local_68 = uStack_198;
              uStack_60 = local_190;
              local_78 = uStack_1a8;
              uStack_70 = local_1a0;
              local_88 = uStack_1b8;
              uStack_80 = local_1b0;
              local_98 = uStack_1c8;
              uStack_90 = local_1c0;
              local_a8 = (undefined1 *)local_1d8;
              uStack_a0 = uStack_1d0;
              local_3b8 = local_228;
              local_3c8 = local_238;
              uStack_3c0 = uStack_230;
              local_398 = local_220;
              uStack_390 = uStack_218;
              local_388 = local_210;
              uStack_3b0 = local_208;
              uStack_3a8 = uStack_200;
              local_3a0 = local_1f8;
              uStack_350 = uStack_188;
              local_348 = local_180;
              uStack_340 = uStack_178;
              local_338 = local_170;
              uStack_330 = uStack_168;
              local_328 = local_160;
              uStack_320 = local_158;
              local_318 = uStack_150;
              uStack_310 = local_148;
              local_308 = uStack_140;
              uStack_300 = local_138;
              local_2f8 = uStack_130;
              local_370 = local_1e0;
              local_380 = local_1f0;
              uStack_378 = uStack_1e8;
              local_2e8 = local_b8;
              uStack_2f0 = local_100;
              local_368 = (undefined *)0x10000000001;
              uStack_360 = 0x100000100;
              rhapsody::rg::pass_builder::PassBuilder::
              render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                        ();
              if (local_102 != '\b') {
                rhapsody::raw::managed::handle::impl$6::drop();
              }
            }
            return;
          }
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      uVar5 = 0xc26ebbeefbfc3261;
      bevy_ecs::component::Components::get_resource_id();
      if ((extraout_RAX_00 != 0) &&
         ((uVar5 < *(ulonglong *)(in_RDX + 0x218) &&
          (uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar5 * 8), uVar5 != 0)))) {
        lVar4 = ~uVar5 * 0x60;
        if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar4) != 0) {
          lVar4 = *(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar4 + 0x10);
          lVar2 = *(longlong *)(lVar4 + 0x10);
          lVar3 = *(longlong *)(lVar4 + 0x30);
          if (lVar3 != 0 || lVar2 != 0) {
            lVar7 = *(longlong *)(lVar4 + 8) + -0x10 + lVar2 * 0x10;
            if (lVar2 == 0) {
              lVar7 = 0;
            }
            lVar4 = *(longlong *)(lVar4 + 0x28) + -0x10 + lVar3 * 0x10;
            if (lVar3 == 0) {
              lVar4 = lVar7;
            }
            iVar6 = *(int *)(lVar4 + 8);
          }
          goto LAB_140a585cc;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


