// to_input_recording_opts
// sig: void __cdecl to_input_recording_opts(void)
// size: 289

void __cdecl tiny_glade::app_config::CliArgs::to_input_recording_opts(void)

{
  undefined1 uVar1;
  void *pvVar2;
  void *extraout_RAX;
  void *extraout_RAX_00;
  longlong lVar3;
  undefined8 *in_RCX;
  longlong in_RDX;
  size_t _Size;
  undefined8 uVar4;
  void *_Dst;
  
  if (SBORROW8(0,*(longlong *)(in_RDX + 0x68))) {
    uVar4 = 0;
    if (SBORROW8(0,*(longlong *)(in_RDX + 0x88))) goto LAB_140353486;
    _Size = *(size_t *)(in_RDX + 0x98);
    if ((longlong)_Size < 0) goto LAB_14035349f;
    pvVar2 = *(void **)(in_RDX + 0x90);
    if (_Size == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (void *)0x0) goto LAB_14035349f;
    }
    memcpy(_Dst,pvVar2,_Size);
    uVar4 = 2;
    lVar3 = 0xa0;
  }
  else {
    _Size = *(size_t *)(in_RDX + 0x78);
    if ((longlong)_Size < 0) {
LAB_14035349f:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    pvVar2 = *(void **)(in_RDX + 0x70);
    uVar4 = 1;
    _Dst = (void *)0x1;
    if ((_Size != 0) &&
       (std::alloc::__default_lib_allocator::__rust_alloc(), _Dst = extraout_RAX_00,
       extraout_RAX_00 == (void *)0x0)) goto LAB_14035349f;
    memcpy(_Dst,pvVar2,_Size);
    lVar3 = 0x80;
  }
  uVar1 = *(undefined1 *)(in_RDX + lVar3);
  in_RCX[1] = _Size;
  in_RCX[2] = _Dst;
  in_RCX[3] = _Size;
  *(undefined1 *)(in_RCX + 4) = uVar1;
LAB_140353486:
  *in_RCX = uVar4;
  return;
}

