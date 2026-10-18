
void __cdecl
country_core::render::raster_single::
raster_single_object<1,array_<enum2_<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env_0>
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
  undefined1 local_b90 [1000];
  undefined1 local_7a8 [248];
  undefined1 local_6b0 [248];
  undefined8 local_5b8;
  undefined8 local_5b0;
  undefined8 uStack_5a8;
  undefined8 local_5a0;
  undefined8 local_598;
  undefined8 local_590;
  undefined8 local_588;
  undefined8 local_580;
  undefined8 local_578;
  undefined *local_548;
  undefined8 uStack_540;
  undefined8 local_538;
  undefined8 uStack_530;
  longlong local_528;
  longlong lStack_520;
  uint local_510;
  undefined8 local_50c;
  undefined8 uStack_504;
  undefined8 local_4fc;
  undefined8 uStack_4f4;
  undefined8 local_4ec;
  undefined8 uStack_4e4;
  undefined8 local_4dc;
  undefined8 uStack_4c8;
  undefined4 local_1d0;
  undefined4 uStack_1cc;
  undefined1 local_1c8 [160];
  undefined1 local_128;
  undefined2 local_127;
  undefined1 local_125;
  undefined8 local_124;
  undefined8 uStack_11c;
  undefined8 local_114;
  undefined8 uStack_10c;
  undefined8 local_104;
  undefined8 uStack_fc;
  undefined8 local_f4;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined4 local_b8;
  undefined8 local_a8;
  longlong local_88;
  longlong *local_80;
  undefined8 *local_78;
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
LAB_1409cdf48:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 != 0x39) goto LAB_1409cdf48;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar12 != 0xfffffffffffffffe) {
          if (uVar12 + 2 < 0x39) {
            if ((char)(&UNK_142ac13cd)[uVar12] < -0x40) {
LAB_1409cdfa6:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar12 + 2 != 0x39) goto LAB_1409cdfa6;
        }
        lVar13 = 0x37 - uVar12;
      }
    }
    puVar9 = &DAT_142ac15fa;
    do {
      bVar2 = puVar9[-1];
      uVar10 = (uint)(char)bVar2;
      local_88 = lVar13;
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
        if (uVar10 != 0x5c) goto LAB_1409ce079;
LAB_1409ce08f:
        if (puVar9 == &UNK_142ac15ca) break;
        if (puVar9 + -0x142ac15ca < (undefined *)0x2f) {
          if (-0x41 < (char)puVar9[1]) break;
        }
        else if (puVar9 + -0x142ac15ca == (undefined *)0x2f) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar9 = puVar9 + -1;
      if (uVar10 == 0x5c) goto LAB_1409ce08f;
LAB_1409ce079:
      if (uVar10 == 0x2f) goto LAB_1409ce08f;
    } while (puVar9 != &DAT_142ac15cb);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar11 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1409ce521;
    if (*plVar11 != 0) goto LAB_1409ce52d;
    *plVar11 = -1;
    local_80 = plVar11;
    puffin::ThreadProfiler::begin_scope();
    *local_80 = *local_80 + 1;
    local_a8 = 1;
  }
  uVar5 = *in_stack_00000030;
  uVar6 = in_stack_00000030[3];
  uVar7 = in_stack_00000030[1];
  uVar8 = in_stack_00000030[2];
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_1d0 = 2;
  local_5b8 = uVar5;
  local_5b0 = uVar7;
  uStack_5a8 = uVar8;
  local_5a0 = uVar6;
  pass_builder::draw_stream::RasterPass::new_impl();
  local_b8 = *(undefined4 *)(in_stack_00000038 + 4);
  local_d8 = *in_stack_00000038;
  uStack_d0 = in_stack_00000038[1];
  local_c8 = in_stack_00000038[2];
  uStack_c0 = in_stack_00000038[3];
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
  local_50c = (ulonglong)local_50c._6_2_ << 0x30;
  local_50c = CONCAT44(local_50c._4_4_,0x3f800000);
  local_5b8 = 0;
  uStack_5a8 = 0;
  local_5a0 = 4;
  local_598 = 0;
  local_528 = *extraout_RAX_02;
  lStack_520 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  local_538 = 0;
  uStack_530 = 0;
  local_548 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
  uStack_540 = 0;
  local_578 = 0x100000003;
  local_590 = 0;
  local_588 = 1;
  local_580 = 0;
  local_61 = 1;
  pass_builder::draw_builder::DrawBuilder::new();
  memcpy(local_6b0,local_7a8,0xf8);
  local_78 = (undefined8 *)0x579f39ab91a01865;
  local_5b8._0_4_ = 2;
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(&local_1d0,local_6b0,0xf8);
  local_78 = &local_d8;
  local_5b8 = CONCAT44(local_5b8._4_4_,2);
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
            ();
  uVar5 = CONCAT44(uStack_1cc,local_1d0);
  memcpy(&local_5b0,local_1c8,0xa0);
  local_50c = local_124;
  uStack_504 = uStack_11c;
  local_4fc = local_114;
  uStack_4f4 = uStack_10c;
  local_4ec = local_104;
  uStack_4e4 = uStack_fc;
  local_4dc = local_f4;
  uStack_4c8 = uStack_e0;
  local_510 = CONCAT13(local_125,CONCAT21(local_127,local_128)) & 0x9fffff7f | 0x40000000;
  local_61 = 1;
  local_5b8 = uVar5;
  pass_builder::draw_builder::DrawBuilder::draw_no_vao();
  memcpy(&local_5b8,local_b90,1000);
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
LAB_1409ce521:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar11 != 0) {
LAB_1409ce52d:
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

