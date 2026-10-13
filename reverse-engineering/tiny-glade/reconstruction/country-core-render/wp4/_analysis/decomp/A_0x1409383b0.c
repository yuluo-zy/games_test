// ADDRESS: 0x1409383b0
// SIZE: 1054
// SIGNATURE: void __cdecl compile_shader_variants(void)

void __cdecl country_core::render::shader::compile_shader_variants(void)

{
  longlong *_Dst;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  void *_Dst_00;
  byte *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong lVar1;
  undefined8 *in_RCX;
  undefined8 *puVar2;
  longlong local_3d8;
  undefined8 local_3d0;
  undefined1 local_3c8 [304];
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 local_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  longlong *local_a8;
  undefined8 uStack_a0;
  undefined *local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  longlong local_78;
  undefined8 *puStack_70;
  longlong *local_60;
  undefined1 local_51;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  shader_variants::get_raster_shader_variants();
  local_108 = local_258;
  local_118 = local_268;
  uStack_110 = uStack_260;
  local_128 = local_278;
  uStack_120 = uStack_270;
  local_138 = local_288;
  uStack_130 = uStack_280;
  local_148 = local_298;
  uStack_140 = uStack_290;
  local_b8 = local_210;
  local_c8 = local_220;
  uStack_c0 = uStack_218;
  local_d8 = local_230;
  uStack_d0 = uStack_228;
  local_e8 = local_240;
  uStack_e0 = uStack_238;
  local_f8 = local_250;
  uStack_f0 = uStack_248;
  shader_pipeline::compiled::ShaderCompilerArtifacts::
  load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>();
  local_51 = 1;
  core::ptr::drop_in_place<shader_pipeline::pipeline_desc::RasterPipelineDesc>();
  if (SBORROW8(0,local_3d8)) {
    in_RCX[1] = local_3d0;
    *in_RCX = 0;
    core::ptr::drop_in_place<shader_pipeline::pipeline_desc::RasterPipelineDesc>();
  }
  else {
    memcpy(&local_278,local_3c8,0x130);
    local_298 = 1;
    uStack_290 = 1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    puVar2 = &local_298;
    memcpy(_Dst,puVar2,0x150);
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    local_60 = _Dst;
    if ((*extraout_RAX & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX[0] = 1;
      extraout_RAX[1] = 0;
      extraout_RAX[2] = 0;
      extraout_RAX[3] = 0;
      extraout_RAX[4] = 0;
      extraout_RAX[5] = 0;
      extraout_RAX[6] = 0;
      extraout_RAX[7] = 0;
      *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
      *(undefined8 **)(extraout_RAX + 0x10) = puVar2;
      local_78 = extraout_RAX_00;
    }
    else {
      local_78 = *(longlong *)(extraout_RAX + 8);
      puVar2 = *(undefined8 **)(extraout_RAX + 0x10);
    }
    *(longlong *)(extraout_RAX + 8) = local_78 + 1;
    uStack_a0 = 0;
    local_98 = &anon_2d944acd27646b19de27e02456f85def_36_llvm_220559307296278160;
    uStack_90 = 0;
    local_88 = 0;
    uStack_80 = 0;
    local_258 = local_b8;
    local_268 = local_c8;
    uStack_260 = uStack_c0;
    local_278 = local_d8;
    uStack_270 = uStack_d0;
    local_288 = local_e8;
    uStack_280 = uStack_e0;
    local_298 = local_f8;
    uStack_290 = uStack_f0;
    local_a8 = local_60;
    puStack_70 = puVar2;
    shader_pipeline::compiled::ShaderCompilerArtifacts::
    load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>();
    core::ptr::drop_in_place<shader_pipeline::pipeline_desc::RasterPipelineDesc>();
    if (SBORROW8(0,local_3d8)) {
      in_RCX[1] = local_3d0;
      *in_RCX = 0;
      LOCK();
      *local_a8 = *local_a8 + -1;
      UNLOCK();
      if (*local_a8 == 0) {
        alloc::sync::Arc::
        drop_slow<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>();
      }
      local_51 = 0;
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    else {
      memcpy(&local_278,local_3c8,0x130);
      local_298 = 1;
      uStack_290 = 1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst_00 == (void *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      puVar2 = &local_298;
      memcpy(_Dst_00,puVar2,0x150);
      std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
      local_60 = _Dst_00;
      if ((*extraout_RAX_01 & 1) == 0) {
        std::sys::random::hashmap_random_keys();
        extraout_RAX_01[0] = 1;
        extraout_RAX_01[1] = 0;
        extraout_RAX_01[2] = 0;
        extraout_RAX_01[3] = 0;
        extraout_RAX_01[4] = 0;
        extraout_RAX_01[5] = 0;
        extraout_RAX_01[6] = 0;
        extraout_RAX_01[7] = 0;
        *(longlong *)(extraout_RAX_01 + 8) = extraout_RAX_02;
        *(undefined8 **)(extraout_RAX_01 + 0x10) = puVar2;
        lVar1 = extraout_RAX_02;
      }
      else {
        lVar1 = *(longlong *)(extraout_RAX_01 + 8);
        puVar2 = *(undefined8 **)(extraout_RAX_01 + 0x10);
      }
      *(longlong *)(extraout_RAX_01 + 8) = lVar1 + 1;
      in_RCX[6] = local_78;
      in_RCX[7] = puStack_70;
      in_RCX[4] = local_88;
      in_RCX[5] = uStack_80;
      in_RCX[2] = local_98;
      in_RCX[3] = uStack_90;
      *in_RCX = local_a8;
      in_RCX[1] = uStack_a0;
      in_RCX[8] = local_60;
      in_RCX[9] = 0;
      in_RCX[10] = &anon_2d944acd27646b19de27e02456f85def_36_llvm_220559307296278160;
      in_RCX[0xb] = 0;
      in_RCX[0xc] = 0;
      in_RCX[0xd] = 0;
      in_RCX[0xe] = lVar1;
      in_RCX[0xf] = puVar2;
    }
  }
  return;
}

