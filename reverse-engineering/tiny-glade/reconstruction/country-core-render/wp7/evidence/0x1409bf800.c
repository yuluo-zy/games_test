
void __cdecl
country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass(void)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong in_RDX;
  ulonglong uVar4;
  undefined *in_R8;
  longlong lVar5;
  undefined8 in_R9;
  byte in_stack_00000038;
  undefined *in_stack_00000040;
  undefined8 in_stack_00000048;
  longlong in_stack_00000050;
  undefined **ppuVar6;
  undefined *local_c18;
  undefined1 local_c10 [163];
  undefined1 local_b6d;
  undefined8 local_b6c;
  undefined8 uStack_b64;
  undefined4 local_b5c;
  undefined4 uStack_b30;
  undefined4 uStack_b2c;
  undefined4 uStack_b28;
  undefined4 uStack_b24;
  undefined8 local_b20 [27];
  char local_a42;
  undefined *local_a28;
  undefined8 uStack_a20;
  undefined8 local_a18;
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 local_9f8;
  undefined8 uStack_9f0;
  undefined8 local_9e8;
  undefined8 local_9e0;
  undefined8 uStack_9d8;
  undefined8 local_9d0;
  undefined *local_9c8;
  undefined8 uStack_9c0;
  undefined8 local_9b8;
  undefined8 uStack_9b0;
  undefined8 local_9a8;
  undefined8 uStack_9a0;
  undefined8 local_998;
  undefined8 uStack_990;
  undefined8 local_988;
  undefined8 uStack_980;
  undefined8 local_978;
  undefined8 uStack_970;
  undefined8 local_968;
  undefined8 uStack_960;
  undefined8 local_958;
  undefined8 uStack_950;
  undefined8 local_948;
  undefined8 uStack_940;
  undefined8 local_938;
  undefined8 uStack_930;
  undefined8 local_928;
  undefined8 uStack_920;
  char local_8f2;
  undefined8 local_8f0;
  undefined8 local_8a8;
  undefined *local_638;
  undefined8 uStack_630;
  undefined8 local_628;
  undefined8 local_620;
  undefined8 uStack_618;
  undefined8 local_610;
  undefined8 local_608;
  undefined8 uStack_600;
  undefined8 local_5f8;
  undefined8 local_5f0;
  undefined8 uStack_5e8;
  undefined8 local_5e0;
  undefined8 local_5d8;
  undefined8 uStack_5d0;
  undefined8 local_5c0;
  undefined8 uStack_5b8;
  undefined8 local_5b0;
  undefined8 uStack_5a8;
  undefined8 local_5a0;
  undefined8 uStack_598;
  uint local_590;
  undefined4 uStack_58c;
  undefined4 uStack_588;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined8 uStack_578;
  undefined8 uStack_570;
  undefined8 uStack_568;
  undefined8 uStack_560;
  undefined8 uStack_558;
  undefined4 uStack_550;
  undefined4 uStack_54c;
  undefined4 uStack_548;
  undefined4 uStack_544;
  char local_502;
  undefined *local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined *local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  longlong local_1b8;
  longlong lStack_1b0;
  undefined8 local_1a8;
  undefined4 local_19c;
  undefined2 local_198;
  undefined *local_b8;
  undefined8 uStack_b0;
  longlong local_a0;
  undefined *local_90;
  undefined *local_88;
  undefined8 uStack_80;
  float local_68;
  undefined1 local_64;
  longlong local_58;
  uint local_50;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  rhapsody::raw::buffer::BufferDesc::new_gpu_only();
  local_a28 = local_248;
  uStack_a20 = uStack_240;
  rhapsody::rg::resource::impl$5::from();
  local_638 = &DAT_142abfbf1;
  uStack_630 = 0x15;
  rhapsody::rg::graph::RenderGraph::create_raw_resource();
  local_88 = local_248;
  uStack_80 = uStack_240;
  clear_rg_buffer();
  rhapsody::raw::texture::TextureDesc::extent_2d();
  rhapsody::raw::buffer::BufferDesc::new_gpu_only();
  local_a28 = local_248;
  uStack_a20 = uStack_240;
  rhapsody::rg::resource::impl$5::from();
  local_638 = &DAT_142abfc06;
  uStack_630 = 0x13;
  rhapsody::rg::graph::RenderGraph::create_raw_resource();
  local_b8 = local_248;
  uStack_b0 = uStack_240;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_638 = (undefined *)((ulonglong)local_638 & 0xffffffff00000000);
  local_248 = (undefined *)0x0;
  local_238 = 0;
  uStack_240 = in_R9;
  local_90 = in_R8;
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_49 = 1;
  uVar4 = 0x4bc362195b80f729;
  local_58 = in_RDX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar4 < *(ulonglong *)(local_58 + 0x218))) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(local_58 + 0x210) + uVar4 * 8), uVar4 != 0)) {
    lVar5 = ~uVar4 * 0x60;
    if (*(longlong *)(*(longlong *)(local_58 + 0x1e0) + 0x20 + lVar5) != 0) {
      puVar1 = *(ulonglong **)(*(longlong *)(local_58 + 0x1e0) + lVar5 + 0x10);
      local_68 = (float)(int)puVar1[1] / 1e+09 + (float)*puVar1;
      local_64 = 0;
      lVar5 = *(longlong *)(in_stack_00000050 + 0x170);
      local_a0 = *(longlong *)(in_stack_00000050 + 0x178) * 0x80 + lVar5;
      local_50 = (uint)in_stack_00000038;
      if (lVar5 != local_a0) {
        do {
          uVar2 = *(undefined8 *)(lVar5 + 0x50);
          uVar3 = *(undefined8 *)(lVar5 + 0x58);
          local_49 = 1;
          rhapsody::raw::shader::impl$7::default();
          local_49 = 1;
          core::ops::function::FnOnce::
          call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                    ();
          if (extraout_RAX_00 == (longlong *)0x0) {
            local_49 = 1;
                    // WARNING: Subroutine does not return
            std::thread::local::panic_access_error();
          }
          local_198 = 0;
          local_19c = 0x3f800000;
          local_248 = (undefined *)0x1;
          local_238 = 0;
          uStack_230 = 4;
          local_228 = 0;
          local_1b8 = *extraout_RAX_00;
          lStack_1b0 = extraout_RAX_00[1];
          *extraout_RAX_00 = *extraout_RAX_00 + 1;
          local_1c8 = 0;
          uStack_1c0 = 0;
          local_1d8 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
          uStack_1d0 = 0;
          local_208 = 0x100000003;
          uStack_220 = 0;
          local_218 = 1;
          uStack_210 = 0;
          local_49 = 1;
          uStack_240 = uVar2;
          local_1a8 = uVar3;
          country_core::render::pass_builder::draw_builder::DrawBuilder::new();
          local_49 = 1;
          country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
          local_638 = (undefined *)CONCAT44(local_638._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&local_638,&local_248,0xf8);
          local_248 = (undefined *)CONCAT44(local_248._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(local_b20,&local_638,0xf8);
          if (local_a42 != '\b') {
            country_core::render::pass_builder::uniform_binder::UniformBinder::
            write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                      ();
          }
          memcpy(&local_638,local_b20,0xf8);
          local_b20[0] = in_stack_00000048;
          local_248 = (undefined *)CONCAT44(local_248._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&local_248,&local_638,0xf8);
          local_49 = 1;
          country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer();
          local_49 = 1;
          country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer();
          local_248 = in_stack_00000040;
          local_638 = (undefined *)CONCAT44(local_638._4_4_,2);
          ppuVar6 = &local_638;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&uStack_630,local_c10,0xa3);
          uStack_58c = (undefined4)local_b6c;
          uStack_588 = (undefined4)((ulonglong)local_b6c >> 0x20);
          uStack_584 = (undefined4)uStack_b64;
          uStack_580 = (undefined4)((ulonglong)uStack_b64 >> 0x20);
          uStack_57c = local_b5c;
          uStack_550 = uStack_b30;
          uStack_54c = uStack_b2c;
          uStack_548 = uStack_b28;
          uStack_544 = uStack_b24;
          local_638 = local_c18;
          local_590 = CONCAT13(local_b6d,(undefined3)local_590) & 0x9fffffff | 0x40000000;
          local_49 = 1;
          (**(code **)(lVar5 + 0x70))(&local_638,lVar5,local_58,*(code **)(lVar5 + 0x70),ppuVar6);
          lVar5 = lVar5 + 0x80;
        } while (lVar5 != local_a0);
      }
      memcpy(&local_638,&local_a28,1000);
      local_49 = 0;
      country_core::render::pass_builder::draw_stream::RasterPass::render();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
                ();
      local_208 = local_988;
      uStack_200 = uStack_980;
      local_218 = local_998;
      uStack_210 = uStack_990;
      local_228 = local_9a8;
      uStack_220 = uStack_9a0;
      local_238 = local_9b8;
      uStack_230 = uStack_9b0;
      local_248 = local_9c8;
      uStack_240 = uStack_9c0;
      local_628 = local_a18;
      local_638 = local_a28;
      uStack_630 = uStack_a20;
      local_608 = local_a10;
      uStack_600 = uStack_a08;
      local_5f8 = local_a00;
      local_620 = local_9f8;
      uStack_618 = uStack_9f0;
      local_610 = local_9e8;
      local_5c0 = local_978;
      uStack_5b8 = uStack_970;
      local_5b0 = local_968;
      uStack_5a8 = uStack_960;
      local_5a0 = local_958;
      uStack_598 = uStack_950;
      local_590 = (uint)local_948;
      uStack_58c = (undefined4)((ulonglong)local_948 >> 0x20);
      uStack_588 = (undefined4)uStack_940;
      uStack_584 = (undefined4)((ulonglong)uStack_940 >> 0x20);
      uStack_580 = (undefined4)local_938;
      uStack_57c = (undefined4)((ulonglong)local_938 >> 0x20);
      uStack_578 = uStack_930;
      uStack_570 = local_928;
      uStack_568 = uStack_920;
      local_5e0 = local_9d0;
      local_5f0 = local_9e0;
      uStack_5e8 = uStack_9d8;
      uStack_558 = local_8a8;
      uStack_560 = local_8f0;
      local_5d8 = 0x100000001;
      uStack_5d0 = 0x100000001;
      rhapsody::rg::pass_builder::PassBuilder::
      render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                ();
      if (local_8f2 != '\b') {
        rhapsody::raw::managed::handle::impl$6::drop();
      }
      country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
                ();
      country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
                ();
      local_a28 = (undefined *)CONCAT44(local_a28._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_a28,&local_638,400);
      local_638 = (undefined *)CONCAT44(local_638._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_638,&local_a28,400);
      if (local_502 != '\b') {
        country_core::render::pass_builder::uniform_binder::UniformBinder::
        write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                  ();
      }
      memcpy(&local_a28,&local_638,400);
      local_638._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_638,&local_a28,400);
      local_248 = (undefined *)0x579f39ab91a01865;
      local_a28 = (undefined *)CONCAT44(local_a28._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>();
      memcpy(&local_a28,&local_638,400);
      local_248 = (undefined *)CONCAT71(local_248._1_7_,9);
      local_638._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_638,&local_a28,400);
      local_b20[0] = CONCAT71(local_b20[0]._1_7_,10);
      local_248 = (undefined *)CONCAT44(local_248._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_248,&local_638,400);
      local_a28 = (undefined *)CONCAT71(local_a28._1_7_,8);
      local_638 = (undefined *)CONCAT44(local_638._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>();
      memcpy(&local_638,&local_248,400);
      local_b20[0] = 0x579f39ab91a01865;
      local_a28._0_4_ = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>();
      memcpy(&local_a28,&local_638,400);
      country_core::render::pass_builder::bind_resources::impl$0::
      bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>();
      local_248 = local_90;
      local_a28 = (undefined *)CONCAT44(local_a28._4_4_,2);
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
      memcpy(&local_a28,&local_638,400);
      country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw()
      ;
      return;
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

