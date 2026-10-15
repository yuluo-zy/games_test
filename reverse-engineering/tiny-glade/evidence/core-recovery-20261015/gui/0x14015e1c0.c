
// WARNING: Removing unreachable block (ram,0x00014015e229)

void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env_1,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  size_t _Size;
  void *_Src;
  longlong lVar5;
  void *extraout_RAX;
  longlong *in_RCX;
  longlong lVar6;
  void *_Dst;
  float fVar7;
  longlong *local_1a8;
  
  plVar1 = (longlong *)*in_RCX;
  plVar2 = (longlong *)in_RCX[1];
  *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
  lVar3 = *plVar1;
  lVar4 = *plVar2;
  lVar6 = 0;
  do {
    if (!SBORROW8(0,*(longlong *)(lVar3 + 0x10 + lVar6))) {
      _Size = *(size_t *)(lVar3 + 0x20 + lVar6);
      if ((longlong)_Size < 0) {
LAB_14015e456:
                    // WARNING: Subroutine does not return
        ::alloc::raw_vec::handle_error();
      }
      _Src = *(void **)(lVar3 + 0x18 + lVar6);
      if (_Size == 0) {
        _Dst = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX;
        if (extraout_RAX == (void *)0x0) goto LAB_14015e456;
      }
      memcpy(_Dst,_Src,_Size);
      if (SBORROW8(0,*(longlong *)(lVar3 + 0x10 + lVar6))) {
                    // WARNING: Subroutine does not return
        option::unwrap_failed();
      }
      egui::widgets::label::impl$2::ui();
      LOCK();
      *local_1a8 = *local_1a8 + -1;
      UNLOCK();
      if (*local_1a8 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      lVar5 = *(longlong *)(lVar3 + 0x10 + lVar6);
      if (SBORROW8(0,lVar5)) {
                    // WARNING: Subroutine does not return
        option::unwrap_failed();
      }
      fVar7 = *(float *)(lVar3 + 0x2c + lVar6) - *(float *)(lVar4 + 0x28);
      *(float *)(lVar3 + 0x2c + lVar6) = fVar7;
      if (fVar7 < 0.0) {
        if (lVar5 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        *(undefined8 *)(lVar3 + 0x10 + lVar6) = 0x8000000000000000;
      }
    }
    lVar6 = lVar6 + 0x20;
    if (lVar6 == 0xa0) {
      return;
    }
  } while( true );
}


