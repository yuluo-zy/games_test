// address 0x1409ce780  name raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1>

void __cdecl
country_core::render::raster_single::
raster_single_object<1,array_<enum2_<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env_1>
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
  undefined8 *in_stack_00000030;
  void *in_stack_00000038;
  undefined1 local_e08 [1000];
  undefined1 local_a20 [248];
  undefined1 local_928 [248];
  undefined **local_830;
  code *local_828;
  undefined **local_738;
  code *local_730;
  undefined8 local_640;
  undefined8 local_638;
  undefined8 uStack_630;
  undefined8 local_628;
  undefined8 local_620;
  undefined8 local_618;
  undefined8 local_610;
  undefined8 local_608;
  undefined8 local_600;
  undefined *local_5d0;
  undefined8 uStack_5c8;
  undefined8 local_5c0;
  undefined8 uStack_5b8;
  longlong local_5b0;
  longlong lStack_5a8;
  undefined4 local_594;
  undefined2 local_590;
  char local_562;
  longlong local_258;
  undefined *local_250;
  code *local_248;
  undefined *local_230;
  undefined *local_228 [4];
  undefined *local_204 [8];
  undefined1 local_1c2;
  undefined8 local_1c0;
  undefined **local_1b0;
  undefined8 local_1a8;
  undefined ***local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  char local_d2;
  undefined *local_a8;
  undefined8 local_a0;
  longlong *local_98;
  undefined *local_90;
  code *local_88;
  longlong *local_78;
  code *local_70;
  undefined1 local_67;
  undefined1 local_66;
  undefined1 local_65;
  undefined1 local_64;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_61 = 1;
  puffin::are_scopes_on();
  local_78 = (longlong *)CONCAT71(local_78._1_7_,extraout_AL);
  if (extraout_AL == '\0') {
    local_1c0 = 0;
  }
  else {
    local_61 = 1;
    local_70 = (code *)0x39;
    uVar12 = 0x39;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x39) {
          if ((char)(&DAT_142ac13cb)[uVar12] < -0x40) {
LAB_1409ce821:
            local_61 = 1;
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 != 0x39) goto LAB_1409ce821;
      }
      local_61 = 1;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar12 != 0xfffffffffffffffe) {
          if (uVar12 + 2 < 0x39) {
            if ((char)(&UNK_142ac13cd)[uVar12] < -0x40) {
LAB_1409ce892:
              local_61 = 1;
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar12 + 2 != 0x39) goto LAB_1409ce892;
        }
        local_70 = (code *)(0x37 - uVar12);
      }
    }
    puVar9 = &DAT_142ac15fa;
    do {
      bVar2 = puVar9[-1];
      uVar10 = (uint)(char)bVar2;
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
        if (uVar10 != 0x5c) goto LAB_1409ce969;
LAB_1409ce97e:
        if (puVar9 == &UNK_142ac15ca) break;
        if (puVar9 + -0x142ac15ca < (undefined *)0x2f) {
          if (-0x41 < (char)puVar9[1]) break;
        }
        else if (puVar9 + -0x142ac15ca == (undefined *)0x2f) break;
        local_61 = 1;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar9 = puVar9 + -1;
      if (uVar10 == 0x5c) goto LAB_1409ce97e;
LAB_1409ce969:
      if (uVar10 == 0x2f) goto LAB_1409ce97e;
    } while (puVar9 != &DAT_142ac15cb);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 0) {
      local_61 = 1;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      local_98 = extraout_RAX_02;
      if (extraout_RAX_02 == (longlong *)0x0) goto LAB_1409cf561;
    }
    else {
      if ((int)*extraout_RAX_01 != 1) {
LAB_1409cf561:
        local_61 = 1;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_98 = extraout_RAX_01 + 1;
    }
    if (*local_98 != 0) {
      local_61 = 1;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_98 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_98 = *local_98 + 1;
    local_1c0 = 1;
  }
  uVar5 = *in_stack_00000030;
  uVar6 = in_stack_00000030[3];
  uVar7 = in_stack_00000030[1];
  uVar8 = in_stack_00000030[2];
  local_63 = 1;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_1b0 = (undefined **)CONCAT44(local_1b0._4_4_,2);
  local_63 = 1;
  local_640 = uVar5;
  local_638 = uVar7;
  uStack_630 = uVar8;
  local_628 = uVar6;
  pass_builder::draw_stream::RasterPass::new_impl();
  memcpy(&local_258,in_stack_00000038,0x98);
  local_62 = 1;
  rhapsody::raw::shader::impl$7::default();
  local_62 = 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_03 == (longlong *)0x0) {
    local_62 = 1;
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_590 = 0;
  local_594 = 0x3f800000;
  local_640 = 0;
  uStack_630 = 0;
  local_628 = 4;
  local_620 = 0;
  local_5b0 = *extraout_RAX_03;
  lStack_5a8 = extraout_RAX_03[1];
  *extraout_RAX_03 = *extraout_RAX_03 + 1;
  local_5c0 = 0;
  uStack_5b8 = 0;
  local_5d0 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
  uStack_5c8 = 0;
  local_600 = 0x100000003;
  local_618 = 0;
  local_610 = 1;
  local_608 = 0;
  local_62 = 1;
  pass_builder::draw_builder::DrawBuilder::new();
  memcpy(local_928,local_a20,0xf8);
  local_1b0 = local_228;
  local_640._0_4_ = 2;
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
            ();
  memcpy(&local_640,local_928,0xf8);
  local_738 = local_204;
  local_1b0 = (undefined **)CONCAT44(local_1b0._4_4_,2);
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
            ();
  memcpy(&local_1b0,&local_640,0xf8);
  local_738 = (undefined **)0x579f39ab91a01865;
  local_640 = CONCAT44(local_640._4_4_,2);
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  if (local_d2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  if (local_d2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  if (local_d2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  if (local_d2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  if (local_d2 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_640,&local_1b0,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>();
  if (SBORROW8(0,local_258)) {
    local_250 = &DAT_142ac1908;
  }
  local_88 = (code *)0x7;
  if (!SBORROW8(0,local_258)) {
    local_88 = local_248;
  }
  local_830 = &local_90;
  local_828 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1b0 = &anon_ff3ecbf17b154c6586c2d7358fc9984a_75_llvm_1075954330245063046;
  local_1a8 = 1;
  local_1a0 = &local_830;
  local_198 = 1;
  local_190 = 0;
  local_90 = local_250;
  alloc::fmt::format::format_inner();
  local_64 = 1;
  local_70 = local_730;
  ::utils::name_hash::NameHash::compute();
  local_1b0 = (undefined **)CONCAT44(local_1b0._4_4_,2);
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(&local_1b0,&local_640,0xf8);
  local_830 = (undefined **)0xbbc2dfc4c6824251;
  local_640 = CONCAT44(local_640._4_4_,2);
  pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(&local_640,&local_1b0,0xf8);
  pass_builder::draw_builder::impl$2::
  read_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>
            ();
  if (local_738 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_230 == (undefined *)0x0) {
    local_1c2 = 0;
    memcpy(&local_640,&local_830,0xf8);
    local_a8 = &DAT_142ac1908;
    local_a0 = 7;
    local_738 = &local_a8;
    local_730 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_1b0 = &anon_ff3ecbf17b154c6586c2d7358fc9984a_75_llvm_1075954330245063046;
    local_1a8 = 1;
    local_198 = 1;
    local_190 = 0;
    local_67 = 1;
    local_1a0 = &local_738;
    alloc::fmt::format::format_inner();
    local_65 = 1;
    local_70 = local_88;
    ::utils::name_hash::NameHash::compute();
    local_1b0 = (undefined **)CONCAT44(local_1b0._4_4_,2);
    pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(&local_738,&local_640,0xf8);
    if (local_90 != (undefined *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    memcpy(&local_640,&local_830,0xf8);
    local_90 = local_230;
    local_1b0 = (undefined **)CONCAT44(local_1b0._4_4_,2);
    pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<ref_<rhapsody::raw::texture::Texture>_>();
    memcpy(&local_738,&local_640,0xf8);
  }
  memcpy(&local_640,&local_738,0xf8);
  if (local_562 != '\b') {
    pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_1b0,&local_640,0xf8);
  pass_builder::draw_builder::DrawBuilder::draw_no_vao();
  if ((!SBORROW8(0,local_258)) && (local_258 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  memcpy(&local_640,local_e08,1000);
  local_66 = 0;
  pass_builder::draw_stream::RasterPass::render();
  if ((char)local_78 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 0) {
      local_61 = 0;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar11 = extraout_RAX_05;
      if (extraout_RAX_05 == (longlong *)0x0) goto LAB_1409cf576;
    }
    else {
      if ((int)*extraout_RAX_04 != 1) {
LAB_1409cf576:
        local_61 = 0;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar11 = extraout_RAX_04 + 1;
    }
    if (*plVar11 != 0) {
      local_61 = 0;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar11 = -1;
    local_78 = plVar11;
    puffin::ThreadProfiler::end_scope();
    *local_78 = *local_78 + 1;
  }
  return;
}


