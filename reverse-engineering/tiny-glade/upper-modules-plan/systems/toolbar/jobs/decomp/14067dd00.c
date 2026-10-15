// 0x14067dd00 confirm
// signature: void __cdecl confirm(void)

void __cdecl toolbar::mei::widgets::tweaker::MaybeHint::confirm(void)

{
  ulonglong *puVar1;
  void *_Src;
  size_t _Size;
  longlong *in_RCX;
  ulonglong uVar2;
  
  puVar1 = (ulonglong *)*in_RCX;
  _Src = (void *)in_RCX[1];
  _Size = in_RCX[2];
  puVar1[2] = 0;
  if (*puVar1 < _Size) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    uVar2 = puVar1[2];
  }
  else {
    uVar2 = 0;
  }
  memcpy((void *)(puVar1[1] + uVar2),_Src,_Size);
  puVar1[2] = uVar2 + _Size;
  return;
}

