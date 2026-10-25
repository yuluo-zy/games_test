// 0x1408f6900 thumbnails_startup size=695

void __cdecl country_core::systems::render_loop::session_selection::thumbnails_startup(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong *in_R9;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong local_160;
  longlong local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined1 local_f8 [32];
  longlong local_d8;
  longlong lStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined1 local_70 [8];
  undefined1 *local_68;
  ulonglong local_60;
  ulonglong local_58;
  undefined1 *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  startup::load_shader_into_library<ref_<str_>_>();
  startup::load_shader_into_library<ref_<str_>_>();
  startup::load_shader_into_library<ref_<str_>_>();
  startup::load_shader_into_library<ref_<str_>_>();
  rhapsody::raw::texture::TextureDesc::new_2d();
  rhapsody::raw::texture::TextureDesc::sampled();
  rhapsody::raw::texture::TextureDesc::mip_levels();
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  plVar2 = (longlong *)*in_R9;
  local_60 = 0;
  local_58 = 6;
  local_50 = local_70;
  uVar6 = 6;
  uVar7 = 0;
  local_68 = local_f8;
  do {
    while( true ) {
      core::ops::function::impls::impl$3::
      call_mut<tuple_<usize>,country_core::systems::render_loop::session_selection::thumbnails_startup::closure_env_0>
                ();
      uVar7 = uVar7 + 1;
      if (local_160 != -0x7fffffffffffffff) break;
      if (uVar6 == uVar7) {
        return;
      }
    }
    local_80 = local_108;
    uStack_78 = uStack_100;
    local_90 = local_118;
    uStack_88 = uStack_110;
    local_a0 = local_128;
    uStack_98 = uStack_120;
    local_b0 = local_138;
    uStack_a8 = uStack_130;
    uStack_c0 = local_148;
    uStack_b8 = uStack_140;
    lStack_d0 = local_158;
    uStack_c8 = uStack_150;
    local_d8 = local_160;
    lVar3 = plVar2[2];
    local_60 = uVar7;
    if (lVar3 == *plVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar4 = plVar2[1];
    lVar5 = lVar3 * 0x68;
    *(undefined8 *)(lVar4 + 0x60 + lVar5) = uStack_78;
    puVar1 = (undefined8 *)(lVar4 + 0x50 + lVar5);
    *puVar1 = uStack_88;
    puVar1[1] = local_80;
    puVar1 = (undefined8 *)(lVar4 + 0x40 + lVar5);
    *puVar1 = uStack_98;
    puVar1[1] = local_90;
    puVar1 = (undefined8 *)(lVar4 + 0x30 + lVar5);
    *puVar1 = uStack_a8;
    puVar1[1] = local_a0;
    puVar1 = (undefined8 *)(lVar4 + 0x20 + lVar5);
    *puVar1 = uStack_b8;
    puVar1[1] = local_b0;
    puVar1 = (undefined8 *)(lVar4 + 0x10 + lVar5);
    *puVar1 = uStack_c8;
    puVar1[1] = uStack_c0;
    *(longlong *)(lVar4 + lVar5) = local_d8;
    ((longlong *)(lVar4 + lVar5))[1] = lStack_d0;
    plVar2[2] = lVar3 + 1;
    local_50 = local_70;
    uVar6 = local_58;
    uVar7 = local_60;
  } while (local_60 < local_58);
  return;
}

