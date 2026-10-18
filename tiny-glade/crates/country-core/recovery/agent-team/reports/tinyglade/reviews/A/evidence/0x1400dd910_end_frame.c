// end_frame @ 0x1400dd910 size=565
// sig: void __cdecl end_frame(void)


void __cdecl tiny_glade::egui_backend::EguiBackend::end_frame(void)

{
  longlong lVar1;
  undefined8 *in_RCX;
  longlong *in_RDX;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  longlong local_160;
  longlong lStack_158;
  longlong local_150;
  longlong local_48;
  
  egui::context::Context::end_frame();
  hashbrown::raw::RawTableInner::
  drop_inner_table<tuple_<egui::viewport::ViewportId,egui::viewport::ViewportOutput>,alloc::alloc::Global>
            ();
  lVar1 = in_RDX[2];
  local_48 = 0;
  while (lVar1 != local_48) {
    local_48 = local_48 + 1;
    core::ptr::drop_in_place<enum2_<epaint::shape::Shape>_>();
  }
  if (*in_RDX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  in_RDX[2] = local_150;
  *in_RDX = local_160;
  in_RDX[1] = lStack_158;
  epaint::textures::TexturesDelta::append();
  in_RCX[0xe] = local_198;
  in_RCX[0xc] = local_1a8;
  in_RCX[0xd] = uStack_1a0;
  in_RCX[10] = local_1b8;
  in_RCX[0xb] = uStack_1b0;
  in_RCX[8] = local_1c8;
  in_RCX[9] = uStack_1c0;
  in_RCX[6] = local_1d8;
  in_RCX[7] = uStack_1d0;
  in_RCX[4] = local_1e8;
  in_RCX[5] = uStack_1e0;
  in_RCX[2] = local_1f8;
  in_RCX[3] = uStack_1f0;
  *in_RCX = local_208;
  in_RCX[1] = uStack_200;
  return;
}


