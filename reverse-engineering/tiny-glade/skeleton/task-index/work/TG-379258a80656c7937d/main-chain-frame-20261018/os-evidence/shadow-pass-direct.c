
void __cdecl country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  longlong lVar6;
  code *pcVar7;
  char extraout_AL;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar8;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  uint uVar9;
  longlong lVar10;
  longlong *plVar11;
  longlong in_RDX;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong in_R8;
  longlong local_9a8;
  undefined1 *local_9a0;
  undefined8 local_998;
  undefined1 local_5c0 [1000];
  undefined8 local_1d8;
  longlong local_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined *local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  uint local_130;
  undefined4 local_12c;
  undefined2 local_128;
  undefined1 local_dc [52];
  ulonglong local_a8;
  longlong local_98;
  longlong *local_90;
  float local_88;
  char local_84;
  longlong local_80;
  longlong *local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_80 = 0;
  }
  else {
    uVar12 = 0x49;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x49) {
          if ((char)(&DAT_142ac0758)[uVar12] < -0x40) {
LAB_1409cc2f2:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 != 0x49) goto LAB_1409cc2f2;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar12 != 0xfffffffffffffffe)) {
        if (uVar12 + 2 < 0x49) {
          if ((char)(&UNK_142ac075a)[uVar12] < -0x40) {
LAB_1409cc350:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 + 2 != 0x49) goto LAB_1409cc350;
      }
    }
    puVar8 = &DAT_142ac0cc6;
    do {
      bVar2 = puVar8[-1];
      uVar9 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar8[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar8[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar8 + -4;
            puVar8 = puVar8 + -4;
            uVar9 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar8 = puVar8 + -3;
            uVar9 = bVar4 & 0xf;
          }
          uVar9 = bVar3 & 0x3f | uVar9 << 6;
        }
        else {
          puVar8 = puVar8 + -2;
          uVar9 = bVar3 & 0x1f;
        }
        uVar9 = (uint)(bVar2 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_1409cc419;
LAB_1409cc42e:
        if (puVar8 == &UNK_142ac0c7f) break;
        if (puVar8 + -0x142ac0c7f < (undefined *)0x46) {
          if (-0x41 < (char)puVar8[1]) break;
        }
        else if (puVar8 + -0x142ac0c7f == (undefined *)0x46) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar8 = puVar8 + -1;
      if (uVar9 == 0x5c) goto LAB_1409cc42e;
LAB_1409cc419:
      if (uVar9 == 0x2f) goto LAB_1409cc42e;
    } while (puVar8 != &DAT_142ac0c80);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar11 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_1409cc946;
    if (*plVar11 != 0) goto LAB_1409cc967;
    *plVar11 = -1;
    local_70 = plVar11;
    puffin::ThreadProfiler::begin_scope();
    *local_70 = *local_70 + 1;
    local_80 = 1;
  }
  local_98 = in_RDX + 0x38;
  uVar12 = 0xa68561b396784eaa;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar12)) ||
      (uVar12 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar12 * 8), uVar12 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar12 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture();
  local_90 = *(longlong **)(in_R8 + 8);
  local_70 = local_90 + *(longlong *)(in_R8 + 0x10) * 0x10;
  uVar12 = 0;
  do {
    rhapsody::raw::texture::TextureDesc::extent_2d();
    local_9a8 = uVar12 << 0x20;
    local_9a0 = local_dc;
    local_998 = 0x3f80000000000001;
    country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
    local_61 = 1;
    uVar13 = 0x4bc362195b80f729;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar13)) ||
       (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) {
LAB_1409cc973:
      local_61 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar10 = ~uVar13 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar10) == 0) goto LAB_1409cc973;
    puVar5 = *(ulonglong **)(*(longlong *)(in_RDX + 0x1e0) + lVar10 + 0x10);
    local_a8 = uVar12 + 1;
    local_84 = (char)uVar12 + '\x02';
    local_88 = (float)(int)puVar5[1] / 1e+09 + (float)*puVar5;
    plVar11 = local_90;
    if (local_90 != local_70) {
      do {
        lVar10 = plVar11[10];
        lVar6 = plVar11[0xb];
        local_61 = 1;
        rhapsody::raw::shader::impl$7::default();
        local_61 = 1;
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_04 == (longlong *)0x0) {
          local_61 = 1;
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        local_128 = 0;
        local_12c = 0x3f800000;
        local_1d8 = 1;
        local_1c8 = 0;
        local_1c0 = 4;
        local_1b8 = 0;
        local_148 = *extraout_RAX_04;
        lStack_140 = extraout_RAX_04[1];
        *extraout_RAX_04 = *extraout_RAX_04 + 1;
        local_158 = 0;
        uStack_150 = 0;
        local_168 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
        uStack_160 = 0;
        local_198 = 0x100000003;
        local_1b0 = 0;
        local_1a8 = 1;
        local_1a0 = 0;
        local_61 = 1;
        local_1d0 = lVar10;
        local_138 = lVar6;
        local_130 = extraout_EAX;
        country_core::render::pass_builder::draw_builder::DrawBuilder::new();
        local_61 = 1;
        country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
        local_130 = local_130 & 0xffffffcf;
        pcVar7 = (code *)plVar11[0xe];
        memcpy(&local_9a8,&local_1d8,0xf8);
        local_61 = 1;
        (*pcVar7)(&local_9a8,plVar11,in_RDX);
        plVar11 = plVar11 + 0x10;
      } while (plVar11 != local_70);
    }
    memcpy(&local_9a8,local_5c0,1000);
    local_61 = 0;
    country_core::render::pass_builder::draw_stream::RasterPass::render();
    uVar12 = local_a8;
  } while (local_a8 < 3);
  if (local_80 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      plVar11 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_1409cc946:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar11 != 0) {
LAB_1409cc967:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar11 = -1;
    local_70 = plVar11;
    puffin::ThreadProfiler::end_scope();
    *local_70 = *local_70 + 1;
  }
  return;
}


