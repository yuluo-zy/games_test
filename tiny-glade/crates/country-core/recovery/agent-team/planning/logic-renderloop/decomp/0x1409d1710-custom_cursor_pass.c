// 0x1409d1710 custom_cursor_pass size=1936

void __cdecl
country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char extraout_AL;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar5;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  uint uVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong in_RDX;
  ulonglong uVar9;
  char *pcVar10;
  undefined1 local_950 [1000];
  undefined4 local_568 [55];
  char local_48a;
  undefined8 local_180 [2];
  undefined8 local_170;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined *local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  longlong local_f0;
  longlong lStack_e8;
  char *local_e0;
  uint local_d8;
  undefined4 local_d4;
  undefined2 local_d0;
  undefined8 local_88;
  undefined8 local_78;
  longlong *local_60;
  longlong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_58 = (longlong *)CONCAT71(local_58._1_7_,extraout_AL);
  if (extraout_AL == '\0') {
    local_88 = 0;
  }
  else {
    uVar9 = 0x57;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar9 != 0) {
        if (uVar9 < 0x57) {
          if ((char)(&DAT_142ac1445)[uVar9] < -0x40) {
LAB_1409d178d:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar9 != 0x57) goto LAB_1409d178d;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar9 != 0xfffffffffffffffe)) {
        if (uVar9 + 2 < 0x57) {
          if ((char)(&UNK_142ac1447)[uVar9] < -0x40) {
LAB_1409d17eb:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar9 + 2 != 0x57) goto LAB_1409d17eb;
      }
    }
    puVar5 = &DAT_142ac1795;
    do {
      bVar2 = puVar5[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar5[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar5[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar5 + -4;
            puVar5 = puVar5 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar5 = puVar5 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar5 = puVar5 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_1409d18a9;
LAB_1409d18bf:
        if (puVar5 == &UNK_142ac1747) break;
        if (puVar5 + -0x142ac1747 < (undefined *)0x4d) {
          if (-0x41 < (char)puVar5[1]) break;
        }
        else if (puVar5 + -0x142ac1747 == (undefined *)0x4d) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar5 = puVar5 + -1;
      if (uVar6 == 0x5c) goto LAB_1409d18bf;
LAB_1409d18a9:
      if (uVar6 == 0x2f) goto LAB_1409d18bf;
    } while (puVar5 != &DAT_142ac1748);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      local_60 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_60 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
    goto LAB_1409d1e53;
    if (*local_60 != 0) goto LAB_1409d1e5f;
    *local_60 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_60 = *local_60 + 1;
    local_88 = 1;
  }
  uVar9 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar9)) ||
      (uVar9 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar9 * 8), uVar9 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  pcVar10 = "custom_cursor";
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_03 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar9 = 0x864ddca98c2312a6;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar9)) ||
     ((uVar9 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar9 * 8), uVar9 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar9 = 0x3d7b56e787d47c40;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_05 != 0) && (uVar9 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar9 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar9 * 8), uVar9 != 0)) {
    lVar7 = ~uVar9 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
      if (*(char *)(*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar7 + 0x10) + 0xf4) != '\0') {
        rhapsody::raw::texture::TextureDesc::extent_2d();
        local_568[0] = 0;
        local_180[0] = CONCAT44(local_180[0]._4_4_,2);
        country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
        local_49 = 1;
        rhapsody::raw::shader::impl$7::default();
        local_49 = 1;
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_06 == (longlong *)0x0) {
          local_49 = 1;
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        local_d0 = 0;
        local_d4 = 0x3f800000;
        local_180[0] = 0;
        local_170 = 0;
        local_168 = 4;
        local_160 = 0;
        local_f0 = *extraout_RAX_06;
        lStack_e8 = extraout_RAX_06[1];
        *extraout_RAX_06 = *extraout_RAX_06 + 1;
        local_100 = 0;
        uStack_f8 = 0;
        local_110 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
        uStack_108 = 0;
        local_140 = 0x100000003;
        local_158 = 0;
        local_150 = 1;
        local_148 = 0;
        local_49 = 1;
        local_e0 = pcVar10;
        local_d8 = extraout_EAX;
        country_core::render::pass_builder::draw_builder::DrawBuilder::new();
        if (local_48a != '\b') {
          country_core::render::pass_builder::uniform_binder::UniformBinder::
          write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<array_<u32,2>_>,array_<u32,2>_>
                    ();
        }
        memcpy(local_180,local_568,0xf8);
        local_78 = 0xf9f14d14bcc4e780;
        local_568[0] = 2;
        country_core::render::pass_builder::draw_builder::impl$3::
        bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
        memcpy(local_568,local_180,0xf8);
        local_49 = 1;
        country_core::render::pass_builder::draw_builder::impl$2::
        read_storage_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>
                  ();
        local_d8 = local_d8 & 0x9fffff7f | 0x40000000;
        memcpy(local_568,local_180,0xf8);
        local_49 = 1;
        country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
        memcpy(local_568,local_950,1000);
        local_49 = 0;
        country_core::render::pass_builder::draw_stream::RasterPass::render();
      }
      if ((char)local_58 != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_07 == 1) {
          plVar8 = extraout_RAX_07 + 1;
        }
        else if ((*extraout_RAX_07 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar8 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_1409d1e53:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        if (*plVar8 != 0) {
LAB_1409d1e5f:
                    // WARNING: Subroutine does not return
          core::cell::panic_already_borrowed();
        }
        *plVar8 = -1;
        local_58 = plVar8;
        puffin::ThreadProfiler::end_scope();
        *local_58 = *local_58 + 1;
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

