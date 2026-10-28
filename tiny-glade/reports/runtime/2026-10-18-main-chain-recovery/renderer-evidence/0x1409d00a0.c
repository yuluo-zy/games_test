
void __cdecl
country_core::render::raster_single::
raster_single_object<1,array_<enum2_<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env_0>
          (void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar9;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  uint uVar10;
  longlong *plVar11;
  ulonglong uVar12;
  longlong lVar13;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined1 local_b60 [1000];
  undefined1 local_778 [248];
  undefined1 local_680 [248];
  undefined8 local_588;
  undefined8 local_580;
  undefined8 uStack_578;
  undefined8 local_570;
  undefined8 local_568;
  undefined8 local_560;
  undefined8 local_558;
  undefined8 local_550;
  undefined8 local_548;
  undefined *local_518;
  undefined8 uStack_510;
  undefined8 local_508;
  undefined8 uStack_500;
  longlong local_4f8;
  longlong lStack_4f0;
  undefined4 local_4dc;
  undefined2 local_4d8;
  undefined4 local_1a0 [55];
  char local_c2;
  undefined8 local_a8;
  undefined8 local_98;
  longlong local_80;
  longlong *local_78;
  longlong *local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_70 = (longlong *)CONCAT71(local_70._1_7_,extraout_AL);
  if (extraout_AL == '\0') {
    local_a8 = 0;
  }
  else {
    lVar13 = 0x39;
    uVar12 = 0x39;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x39) {
          if ((char)(&DAT_142ac13cb)[uVar12] < -0x40) {
LAB_1409d0128:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 != 0x39) goto LAB_1409d0128;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar12 != 0xfffffffffffffffe) {
          if (uVar12 + 2 < 0x39) {
            if ((char)(&UNK_142ac13cd)[uVar12] < -0x40) {
LAB_1409d0186:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar12 + 2 != 0x39) goto LAB_1409d0186;
        }
        lVar13 = 0x37 - uVar12;
      }
    }
    puVar9 = &DAT_142ac15fa;
    do {
      bVar2 = puVar9[-1];
      uVar10 = (uint)(char)bVar2;
      local_80 = lVar13;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar9[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar9[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar9 + -4;
            puVar9 = puVar9 + -4;
            uVar10 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar9 = puVar9 + -3;
            uVar10 = bVar4 & 0xf;
          }
          uVar10 = bVar3 & 0x3f | uVar10 << 6;
        }
        else {
          puVar9 = puVar9 + -2;
          uVar10 = bVar3 & 0x1f;
        }
        uVar10 = (uint)(bVar2 & 0x3f) | uVar10 << 6;
        if (uVar10 != 0x5c) goto LAB_1409d0259;
LAB_1409d026f:
        if (puVar9 == &UNK_142ac15ca) break;
        if (puVar9 + -0x142ac15ca < (undefined *)0x2f) {
          if (-0x41 < (char)puVar9[1]) break;
        }
        else if (puVar9 + -0x142ac15ca == (undefined *)0x2f) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar9 = puVar9 + -1;
      if (uVar10 == 0x5c) goto LAB_1409d026f;
LAB_1409d0259:
      if (uVar10 == 0x2f) goto LAB_1409d026f;
    } while (puVar9 != &DAT_142ac15cb);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar11 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1409d066a;
    if (*plVar11 != 0) goto LAB_1409d0676;
    *plVar11 = -1;
    local_78 = plVar11;
    puffin::ThreadProfiler::begin_scope();
    *local_78 = *local_78 + 1;
    local_a8 = 1;
  }
  uVar5 = *in_stack_00000030;
  uVar6 = in_stack_00000030[3];
  uVar7 = in_stack_00000030[1];
  uVar8 = in_stack_00000030[2];
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_1a0[0] = 2;
  local_588 = uVar5;
  local_580 = uVar7;
  uStack_578 = uVar8;
  local_570 = uVar6;
  pass_builder::draw_stream::RasterPass::new_impl();
  uVar5 = *in_stack_00000038;
  local_61 = 1;
  rhapsody::raw::shader::impl$7::default();
  local_61 = 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
    local_61 = 1;
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_4d8 = 0;
  local_4dc = 0x3f800000;
  local_588 = 0;
  uStack_578 = 0;
  local_570 = 4;
  local_568 = 0;
  local_4f8 = *extraout_RAX_02;
  lStack_4f0 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  local_508 = 0;
  uStack_500 = 0;
  local_518 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
  uStack_510 = 0;
  local_548 = 0x100000003;
  local_560 = 0;
  local_558 = 1;
  local_550 = 0;
  local_61 = 1;
  pass_builder::draw_builder::DrawBuilder::new();
  memcpy(local_778,local_680,0xf8);
  local_61 = 1;
  pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>();
  local_1a0[0] = 2;
  local_98 = uVar5;
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
            ();
  memcpy(local_1a0,&local_588,0xf8);
  if (local_c2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_588,local_1a0,0xf8);
  local_61 = 1;
  pass_builder::draw_builder::DrawBuilder::draw_no_vao();
  memcpy(&local_588,local_b60,1000);
  local_61 = 0;
  pass_builder::draw_stream::RasterPass::render();
  if ((char)local_70 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar11 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1409d066a:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar11 != 0) {
LAB_1409d0676:
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

