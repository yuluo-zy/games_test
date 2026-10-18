// 0x1409bf320 refraction_copy_pass size=1079

void __cdecl
country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass(void)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong in_RDX;
  ulonglong uVar4;
  longlong lVar5;
  undefined8 in_R9;
  longlong in_stack_00000030;
  undefined4 *puVar6;
  undefined1 local_9f0 [1000];
  undefined4 local_608;
  undefined4 uStack_604;
  undefined4 uStack_600;
  undefined8 uStack_5fc;
  byte local_560;
  undefined8 local_55f;
  undefined8 uStack_557;
  undefined4 local_54f;
  undefined4 uStack_54b;
  undefined4 uStack_547;
  undefined4 uStack_543;
  undefined4 local_53f;
  undefined4 uStack_53b;
  undefined4 uStack_537;
  undefined4 uStack_533;
  undefined4 local_52f;
  undefined4 uStack_52b;
  undefined4 uStack_527;
  undefined4 uStack_51c;
  undefined4 uStack_518;
  undefined4 uStack_514;
  undefined8 local_220;
  undefined1 local_218 [8];
  undefined8 local_210;
  byte local_178;
  undefined8 local_177;
  undefined8 uStack_16f;
  undefined4 local_167;
  undefined4 uStack_163;
  undefined4 uStack_15f;
  undefined4 uStack_15b;
  undefined4 local_157;
  undefined4 uStack_153;
  undefined4 uStack_14f;
  undefined4 uStack_14b;
  undefined4 local_147;
  undefined4 uStack_143;
  undefined4 uStack_13f;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined *local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined4 local_7c;
  undefined2 local_78;
  longlong local_70;
  undefined8 local_68;
  float local_60;
  undefined1 local_5c;
  longlong local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uStack_604 = 0;
  uStack_600 = 0;
  uStack_5fc = 0;
  local_68 = in_R9;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_608 = 1;
  local_220 = 1;
  local_210 = 0;
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
      local_60 = (float)(int)puVar1[1] / 1e+09 + (float)*puVar1;
      local_5c = 0;
      lVar5 = *(longlong *)(in_stack_00000030 + 0x158);
      local_70 = *(longlong *)(in_stack_00000030 + 0x160) * 0x80 + lVar5;
      if (lVar5 != local_70) {
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
          local_78 = 0;
          local_7c = 0x3f800000;
          local_128 = 1;
          local_118 = 0;
          local_110 = 4;
          local_108 = 0;
          local_98 = *extraout_RAX_00;
          lStack_90 = extraout_RAX_00[1];
          *extraout_RAX_00 = *extraout_RAX_00 + 1;
          local_a8 = 0;
          uStack_a0 = 0;
          local_b8 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
          uStack_b0 = 0;
          local_e8 = 0x100000003;
          local_100 = 0;
          local_f8 = 1;
          local_f0 = 0;
          local_49 = 1;
          local_120 = uVar2;
          local_88 = uVar3;
          country_core::render::pass_builder::draw_builder::DrawBuilder::new();
          local_49 = 1;
          country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
          local_128 = local_68;
          local_608 = 2;
          puVar6 = &local_608;
          country_core::render::pass_builder::draw_builder::impl$3::
          bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                    ();
          uVar2 = local_220;
          memcpy(&uStack_600,local_218,0xa0);
          local_55f = local_177;
          uStack_557 = uStack_16f;
          local_54f = local_167;
          uStack_54b = uStack_163;
          uStack_547 = uStack_15f;
          uStack_543 = uStack_15b;
          local_53f = local_157;
          uStack_53b = uStack_153;
          uStack_537 = uStack_14f;
          uStack_533 = uStack_14b;
          local_52f = local_147;
          uStack_52b = uStack_143;
          uStack_527 = uStack_13f;
          uStack_51c = uStack_134;
          uStack_518 = uStack_130;
          uStack_514 = uStack_12c;
          local_560 = local_178 & 0xbf;
          local_608 = (undefined4)uVar2;
          uStack_604 = (undefined4)((ulonglong)uVar2 >> 0x20);
          local_49 = 1;
          (**(code **)(lVar5 + 0x70))(&local_608,lVar5,local_58,*(code **)(lVar5 + 0x70),puVar6);
          lVar5 = lVar5 + 0x80;
        } while (lVar5 != local_70);
      }
      memcpy(&local_608,local_9f0,1000);
      local_49 = 0;
      country_core::render::pass_builder::draw_stream::RasterPass::render();
      return;
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

