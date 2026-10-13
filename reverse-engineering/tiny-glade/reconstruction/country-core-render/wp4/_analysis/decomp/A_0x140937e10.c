// ADDRESS: 0x140937e10
// SIZE: 738
// SIGNATURE: void __cdecl new(void)

void __cdecl country_core::render::shader::ComputeShaderProgram::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 *extraout_RAX;
  void *_Dst;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 in_R8;
  undefined8 *in_R9;
  char *local_330 [40];
  longlong local_1f0;
  char *local_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  char *local_1b8;
  undefined8 local_1b0;
  undefined4 local_1a8;
  undefined4 local_1a4;
  undefined **local_1a0;
  undefined8 local_198;
  undefined8 **local_190;
  undefined8 local_188;
  undefined8 local_180;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 *local_58;
  code *local_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 *local_38;
  undefined1 local_22;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_48 = in_RDX;
  uStack_40 = in_R8;
  local_38 = in_R9;
  shader_pipeline::compiled::ShaderCompilerArtifacts::
  load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>();
  if (SBORROW8(0,local_1f0)) {
    local_330[0] = local_1e8;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  memcpy(local_330,&local_1f0,0x140);
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_58 = &local_48;
    local_50 = core::fmt::impl$74::fmt<str_>;
    local_21 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_1d0 = *extraout_RAX;
    uStack_1c8 = extraout_RAX[1];
    local_1a4 = *(undefined4 *)(extraout_RAX + 2);
    local_1c0 = 5;
    local_1b8 = "country_core::render::shader";
    local_1b0 = 0x1c;
    local_1a0 = &PTR_s_Creating_pipeline_142ab1248;
    local_198 = 1;
    local_190 = &local_58;
    local_188 = 1;
    local_180 = 0;
    local_1f0 = 0;
    local_1e8 = "country_core::render::shader";
    local_1e0 = 0x1c;
    local_1d8 = 0;
    local_1a8 = 1;
    local_21 = 1;
    ::log::__private_api::impl$2::log();
  }
  memcpy(&local_1e0,local_330,0x140);
  local_1f0 = 1;
  local_1e8 = (char *)0x1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,&local_1f0,0x150);
    local_22 = 1;
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    local_22 = 0;
    rhapsody::raw::device::GraphicsDevice::
    create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>_>
              ();
    in_RCX[8] = local_38[8];
    uVar1 = *local_38;
    uVar2 = local_38[1];
    uVar3 = local_38[2];
    uVar4 = local_38[3];
    uVar5 = local_38[4];
    uVar6 = local_38[5];
    uVar7 = local_38[7];
    in_RCX[6] = local_38[6];
    in_RCX[7] = uVar7;
    in_RCX[4] = uVar5;
    in_RCX[5] = uVar6;
    in_RCX[2] = uVar3;
    in_RCX[3] = uVar4;
    *in_RCX = uVar1;
    in_RCX[1] = uVar2;
    in_RCX[0x12] = local_48;
    in_RCX[0x13] = uStack_40;
    in_RCX[0x11] = local_60;
    in_RCX[9] = local_a0;
    in_RCX[10] = uStack_98;
    in_RCX[0xb] = local_90;
    in_RCX[0xc] = uStack_88;
    in_RCX[0xd] = local_80;
    in_RCX[0xe] = uStack_78;
    in_RCX[0xf] = local_70;
    in_RCX[0x10] = uStack_68;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

