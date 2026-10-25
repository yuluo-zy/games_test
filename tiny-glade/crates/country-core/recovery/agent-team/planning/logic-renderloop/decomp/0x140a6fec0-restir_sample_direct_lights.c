// 0x140a6fec0 restir_sample_direct_lights size=1922

void __cdecl
country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights(void)

{
  int iVar1;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *puVar2;
  undefined8 extraout_RAX_01;
  undefined8 *in_RCX;
  ulonglong uVar3;
  longlong in_R8;
  undefined1 local_428 [32];
  undefined4 local_408;
  int local_3f8;
  undefined4 uStack_3f4;
  undefined4 uStack_3f0;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined4 local_3d4;
  undefined4 uStack_3c8;
  undefined4 local_3c4;
  undefined4 uStack_3b0;
  undefined4 local_3ac;
  undefined8 uStack_3a8;
  undefined8 local_3a0;
  undefined4 local_398;
  undefined4 local_394;
  undefined4 local_390;
  undefined4 local_38c;
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
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 local_318;
  char local_2c2;
  undefined *local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
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
  char local_132;
  undefined8 local_130;
  undefined8 local_e8;
  undefined *local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined4 local_b8;
  undefined8 local_b0;
  undefined8 *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uVar3 = 0x8226cc786e679355;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_R8 + 0x218) <= uVar3)) ||
      (uVar3 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar3 * 8), uVar3 == 0)) ||
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar3 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_268 = &anon_107183261458dea85466985370debbb6_16_llvm_3641509820080252961;
    uStack_260 = 0x35;
    if (extraout_RAX_00[3] == 0x35) {
      iVar1 = memcmp(&anon_107183261458dea85466985370debbb6_16_llvm_3641509820080252961,
                     (void *)extraout_RAX_00[2],0x35);
      if (iVar1 == 0) {
        puVar2 = (undefined8 *)(**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
        if (puVar2 != (undefined8 *)0x0) {
          if ((int)*puVar2 == 0) {
            rhapsody::raw::texture::TextureDesc::new_2d();
            local_258 = local_98;
            uStack_250 = uStack_90;
            local_268 = (undefined *)local_a8;
            uStack_260 = uStack_a0;
            rhapsody::rg::resource::impl$4::from();
            local_3f8 = 0x42ad5870;
            uStack_3f4 = 1;
            uStack_3f0 = 0x1a;
            uStack_3ec = 0;
            rhapsody::rg::graph::RenderGraph::create_raw_resource();
            *(undefined8 *)((longlong)in_RCX + 0x14) = local_98;
            *(undefined8 *)((longlong)in_RCX + 0x1c) = uStack_90;
            *(undefined8 **)((longlong)in_RCX + 4) = local_a8;
            *(undefined8 *)((longlong)in_RCX + 0xc) = uStack_a0;
            *(undefined4 *)in_RCX = extraout_EAX;
          }
          else {
            rhapsody::raw::texture::TextureDesc::extent_2d();
            local_b0 = extraout_RAX_01;
            rhapsody::raw::texture::TextureDesc::new_2d();
            rhapsody::raw::texture::TextureDesc::sampled();
            rhapsody::raw::texture::TextureDesc::storage();
            rhapsody::rg::temporal::impl$20::
            get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
                      ();
            if (local_3f8 == 1) {
                    // WARNING: Subroutine does not return
              core::result::unwrap_failed();
            }
            local_268 = (undefined *)CONCAT44(uStack_3f0,uStack_3f4);
            uStack_260 = CONCAT44(uStack_3e8,uStack_3ec);
            local_258 = CONCAT44(uStack_3e0,uStack_3e4);
            uStack_250 = CONCAT44(uStack_3d8,uStack_3dc);
            local_49 = (undefined1)local_3ac;
            local_b8 = local_3d4;
            local_408 = uStack_3b0;
            local_d8 = local_268;
            uStack_d0 = uStack_260;
            local_c8 = local_258;
            uStack_c0 = uStack_250;
            country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>
                      ();
            local_a8 = (undefined8 *)0x579f39ab91a01865;
            local_268._0_4_ = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
                      ();
            memcpy(&local_268,&local_3f8,400);
            local_3f8 = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_3f8,&local_268,400);
            local_268._0_4_ = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_268,&local_3f8,400);
            local_3f8 = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_3f8,&local_268,400);
            local_a8 = (undefined8 *)local_428;
            local_268._0_4_ = 2;
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_268,&local_3f8,400);
            country_core::render::pass_builder::compute_pass_builder::impl$3::
            read_storage_buffer<country_core::resources::render::lantern::LanternSsbo>();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            read_rg_storage_buffer();
            country_core::render::pass_builder::compute_pass_builder::ComputePass::
            read_rg_storage_buffer();
            local_a8 = &local_d8;
            local_268 = (undefined *)CONCAT44(local_268._4_4_,2);
            country_core::render::pass_builder::compute_pass_builder::impl$4::
            bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                      ();
            memcpy(&local_268,&local_3f8,400);
            if (local_132 != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                        ();
            }
            memcpy(&local_3f8,&local_268,400);
            if (local_2c2 != '\b') {
              country_core::render::pass_builder::uniform_binder::UniformBinder::
              write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                        ();
            }
            memcpy(&local_268,&local_3f8,400);
            local_68 = local_1c8;
            uStack_60 = uStack_1c0;
            local_78 = local_1d8;
            uStack_70 = uStack_1d0;
            local_88 = local_1e8;
            uStack_80 = uStack_1e0;
            local_98 = local_1f8;
            uStack_90 = uStack_1f0;
            local_a8 = (undefined8 *)local_208;
            uStack_a0 = uStack_200;
            uStack_3e8 = (undefined4)local_258;
            uStack_3e4 = (undefined4)((ulonglong)local_258 >> 0x20);
            local_3f8 = (int)local_268;
            uStack_3f4 = (undefined4)((ulonglong)local_268 >> 0x20);
            uStack_3f0 = (undefined4)uStack_260;
            uStack_3ec = (undefined4)((ulonglong)uStack_260 >> 0x20);
            uStack_3c8 = (undefined4)uStack_250;
            local_3c4 = (undefined4)((ulonglong)uStack_250 >> 0x20);
            local_380 = local_1b8;
            uStack_378 = uStack_1b0;
            local_370 = local_1a8;
            uStack_368 = uStack_1a0;
            local_360 = local_198;
            uStack_358 = uStack_190;
            local_350 = local_188;
            uStack_348 = uStack_180;
            local_340 = local_178;
            uStack_338 = uStack_170;
            local_330 = local_168;
            uStack_328 = uStack_160;
            local_3a0 = local_210;
            uStack_3b0 = (undefined4)local_220;
            local_3ac = (undefined4)((ulonglong)local_220 >> 0x20);
            uStack_3a8 = uStack_218;
            local_318 = local_e8;
            local_320 = local_130;
            local_398 = 1;
            local_394 = (undefined4)local_b0;
            local_390 = (undefined4)((ulonglong)local_b0 >> 0x20);
            local_38c = 1;
            rhapsody::rg::pass_builder::PassBuilder::
            render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                      ();
            if (local_132 != '\b') {
              rhapsody::raw::managed::handle::impl$6::drop();
            }
            *(undefined4 *)(in_RCX + 4) = local_b8;
            in_RCX[2] = local_c8;
            in_RCX[3] = uStack_c0;
            *in_RCX = local_d8;
            in_RCX[1] = uStack_d0;
          }
          return;
        }
        goto LAB_140a70600;
      }
    }
    local_3f8 = 0;
    uStack_3f4 = 0;
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
  }
LAB_140a70600:
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

