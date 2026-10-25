// new @ 0x1403b6880 size=575
// sig: void __cdecl new(void)


void __cdecl tiny_glade::rollercoaster::CoasterAudioInstance::new(void)

{
  longlong lVar1;
  longlong *in_RCX;
  float in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  int local_e0;
  undefined4 uStack_dc;
  longlong lStack_d8;
  longlong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  longlong lStack_b8;
  undefined1 *local_b0;
  code *local_a8;
  undefined8 **local_a0;
  code *local_98;
  undefined1 local_90 [16];
  longlong local_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong local_58;
  longlong *local_50 [2];
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if (0.03 <= in_stack_00000028) {
    country_core::resources::audio_resources::AudioCore::instantiate_audio();
    if ((local_e0 == 2) &&
       (local_50[0] = &lStack_d8,
       *(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0)) {
      local_b0 = local_90;
      local_a8 = core::fmt::impl$74::fmt<str_>;
      local_a0 = local_50;
      local_98 = core::fmt::impl$74::fmt<anyhow::Error>;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    local_80 = CONCAT44(uStack_dc,local_e0);
    if (local_80 != 2) {
      local_58 = lStack_b8;
      lStack_68 = lStack_c8;
      lStack_60 = local_c0;
      lStack_78 = lStack_d8;
      lStack_70 = local_d0;
      country_core::resources::audio_resources::impl$10::set_volume();
      lVar1 = in_stack_00000030 + in_stack_00000038 * 0x18;
      for (; in_stack_00000030 != lVar1; in_stack_00000030 = in_stack_00000030 + 0x18) {
        country_core::resources::audio_resources::impl$10::set_parameter();
      }
      in_RCX[4] = lStack_60;
      in_RCX[5] = local_58;
      in_RCX[2] = lStack_70;
      in_RCX[3] = lStack_68;
      *in_RCX = local_80;
      in_RCX[1] = lStack_78;
      *(float *)(in_RCX + 6) = in_stack_00000028;
      return;
    }
    anyhow::error::impl$6::drop();
  }
  *in_RCX = 2;
  return;
}


