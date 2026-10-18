// 0x1409c3a80 pre_taa_transparency_pass size=1260

void __cdecl
country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass(void)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong in_RDX;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 in_R9;
  undefined8 in_stack_00000030;
  longlong in_stack_00000040;
  undefined8 *puVar7;
  undefined1 local_a38 [1000];
  undefined8 local_650;
  undefined1 local_648 [160];
  byte local_5a8;
  undefined2 local_5a7;
  byte local_5a5;
  undefined8 local_5a4;
  undefined8 uStack_59c;
  undefined4 local_594;
  undefined4 uStack_590;
  undefined4 uStack_58c;
  undefined4 uStack_588;
  undefined4 local_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined4 uStack_578;
  undefined4 local_574;
  undefined4 uStack_570;
  undefined4 uStack_56c;
  undefined4 uStack_568;
  undefined4 uStack_564;
  undefined4 uStack_560;
  undefined4 uStack_55c;
  longlong local_268;
  undefined1 local_260 [160];
  byte local_1c0;
  undefined2 local_1bf;
  byte local_1bd;
  undefined8 local_1bc;
  undefined8 uStack_1b4;
  undefined4 local_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 local_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 local_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined *local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  longlong local_e0;
  longlong lStack_d8;
  undefined8 local_d0;
  undefined4 local_c4;
  undefined2 local_c0;
  longlong local_78;
  undefined8 local_70;
  float local_68;
  undefined1 local_64;
  undefined8 local_60;
  longlong local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_650 = (ulonglong)local_650._4_4_ << 0x20;
  local_170 = 1;
  local_160 = 0;
  local_168 = in_R9;
  local_70 = in_R9;
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_49 = 1;
  uVar5 = 0x4bc362195b80f729;
  local_58 = in_RDX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar5 < *(ulonglong *)(local_58 + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(local_58 + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar6 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(local_58 + 0x1e0) + 0x20 + lVar6) != 0) {
      puVar1 = *(ulonglong **)(*(longlong *)(local_58 + 0x1e0) + lVar6 + 0x10);
      local_68 = (float)(int)puVar1[1] / 1e+09 + (float)*puVar1;
      local_64 = 0;
      lVar6 = *(longlong *)(in_stack_00000040 + 0x188);
      local_78 = *(longlong *)(in_stack_00000040 + 400) * 0x80 + lVar6;
      if (lVar6 != local_78) {
        do {
          uVar2 = *(undefined8 *)(lVar6 + 0x50);
          uVar3 = *(undefined8 *)(lVar6 + 0x58);
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
          local_c0 = 0;
          local_c4 = 0x3f800000;
          local_170 = 1;
          local_160 = 0;
          local_158 = 4;
          local_150 = 0;
          local_e0 = *extraout_RAX_00;
          lStack_d8 = extraout_RAX_00[1];
          *extraout_RAX_00 = *extraout_RAX_00 + 1;
          local_f0 = 0;
          uStack_e8 = 0;
          local_100 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
          uStack_f8 = 0;
          local_130 = 0x100000003;
          local_148 = 0;
          local_140 = 1;
          local_138 = 0;
          local_49 = 1;
          local_168 = uVar2;
          local_d0 = uVar3;
          country_core::render::pass_builder::draw_builder::DrawBuilder::new();
          local_49 = 1;
          country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
          local_60 = local_70;
          local_650._0_4_ = 2;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&local_650,&local_170,0xf8);
          local_170 = CONCAT44(local_170._4_4_,2);
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          memcpy(&local_268,&local_650,0xf8);
          local_170 = in_stack_00000030;
          local_650 = CONCAT44(local_650._4_4_,2);
          puVar7 = &local_650;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          lVar4 = local_268;
          memcpy(local_648,local_260,0xa0);
          local_5a4 = local_1bc;
          uStack_59c = uStack_1b4;
          local_594 = local_1ac;
          uStack_590 = uStack_1a8;
          uStack_58c = uStack_1a4;
          uStack_588 = uStack_1a0;
          local_584 = local_19c;
          uStack_580 = uStack_198;
          uStack_57c = uStack_194;
          uStack_578 = uStack_190;
          local_574 = local_18c;
          uStack_570 = uStack_188;
          uStack_56c = uStack_184;
          uStack_568 = uStack_180;
          uStack_564 = uStack_17c;
          uStack_560 = uStack_178;
          uStack_55c = uStack_174;
          local_5a8 = local_1c0 & 0xbf;
          local_5a5 = local_1bd & 0x9f | 0x40;
          local_650 = lVar4;
          local_5a7 = local_1bf;
          local_49 = 1;
          (**(code **)(lVar6 + 0x70))(&local_650,lVar6,local_58,*(code **)(lVar6 + 0x70),puVar7);
          lVar6 = lVar6 + 0x80;
        } while (lVar6 != local_78);
      }
      memcpy(&local_650,local_a38,1000);
      local_49 = 0;
      country_core::render::pass_builder::draw_stream::RasterPass::render();
      return;
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

