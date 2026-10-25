// 0x1409c40b0 post_taa_transparency_pass size=869

void __cdecl
country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass(void)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  code *pcVar4;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong lVar5;
  longlong in_RDX;
  ulonglong uVar6;
  longlong in_R9;
  longlong lVar7;
  undefined1 local_928 [1000];
  undefined4 local_540 [250];
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined *local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  uint local_b0;
  undefined4 local_ac;
  undefined2 local_a8;
  float local_60;
  undefined1 local_5c;
  longlong local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar7 = *(longlong *)(in_R9 + 0x1a8);
  if (lVar7 == 0) {
    return;
  }
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_540[0] = 0;
  local_158 = CONCAT44(local_158._4_4_,2);
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_49 = 1;
  uVar6 = 0x4bc362195b80f729;
  local_58 = in_RDX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar6 < *(ulonglong *)(local_58 + 0x218))) &&
     (uVar6 = *(ulonglong *)(*(longlong *)(local_58 + 0x210) + uVar6 * 8), uVar6 != 0)) {
    lVar5 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(local_58 + 0x1e0) + 0x20 + lVar5) != 0) {
      puVar1 = *(ulonglong **)(*(longlong *)(local_58 + 0x1e0) + lVar5 + 0x10);
      local_60 = (float)(int)puVar1[1] / 1e+09 + (float)*puVar1;
      local_5c = 1;
      lVar5 = *(longlong *)(in_R9 + 0x1a0);
      lVar7 = lVar7 * 0x80 + lVar5;
      while( true ) {
        if (lVar5 == lVar7) {
          memcpy(local_540,local_928,1000);
          local_49 = 0;
          country_core::render::pass_builder::draw_stream::RasterPass::render();
          return;
        }
        uVar2 = *(undefined8 *)(lVar5 + 0x50);
        uVar3 = *(undefined8 *)(lVar5 + 0x58);
        local_49 = 1;
        rhapsody::raw::shader::impl$7::default();
        local_49 = 1;
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_00 == (longlong *)0x0) break;
        local_a8 = 0;
        local_ac = 0x3f800000;
        local_158 = 1;
        local_148 = 0;
        local_140 = 4;
        local_138 = 0;
        local_c8 = *extraout_RAX_00;
        lStack_c0 = extraout_RAX_00[1];
        *extraout_RAX_00 = *extraout_RAX_00 + 1;
        local_d8 = 0;
        uStack_d0 = 0;
        local_e8 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
        uStack_e0 = 0;
        local_118 = 0x100000003;
        local_130 = 0;
        local_128 = 1;
        local_120 = 0;
        local_49 = 1;
        local_150 = uVar2;
        local_b8 = uVar3;
        local_b0 = extraout_EAX;
        country_core::render::pass_builder::draw_builder::DrawBuilder::new();
        local_49 = 1;
        country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
        local_b0 = local_b0 & 0x9fffffbf | 0x40000000;
        pcVar4 = *(code **)(lVar5 + 0x70);
        memcpy(local_540,&local_158,0xf8);
        local_49 = 1;
        (*pcVar4)(local_540,lVar5,local_58);
        lVar5 = lVar5 + 0x80;
      }
      local_49 = 1;
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

