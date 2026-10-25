// needs_apply
// sig: void __cdecl needs_apply(void)
// size: 350

void __cdecl tiny_glade::gui::settings::LastAppliedVideoSettings::needs_apply(void)

{
  longlong *in_RCX;
  longlong in_RDX;
  
  if (SBORROW8(0,*in_RCX)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (((*(longlong *)(in_RDX + 0x38) != -0x7fffffffffffffff && in_RCX[7] != -0x7fffffffffffffff) &&
      (*(longlong *)(in_RDX + 0x38) != -0x8000000000000000 && in_RCX[7] != -0x8000000000000000)) &&
     (in_RCX[9] == *(size_t *)(in_RDX + 0x48))) {
    memcmp((void *)in_RCX[8],*(void **)(in_RDX + 0x40),in_RCX[9]);
  }
  return;
}

