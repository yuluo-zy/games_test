// winit_to_bevy_keycode @ 0x1400fb850
// sig: void __cdecl winit_to_bevy_keycode(void)


void __cdecl tiny_glade::winit_bevy_glue::winit_to_bevy_keycode(void)

{
  undefined2 uVar1;
  undefined4 uVar2;
  undefined4 *in_RCX;
  longlong in_RDX;
  
  if (*(short *)(in_RDX + 0x58) == 5) {
    *in_RCX = *(undefined4 *)(&DAT_14294c58c + (ulonglong)(*(byte *)(in_RDX + 0x5a) ^ 0x80) * 4);
    return;
  }
  uVar1 = *(undefined2 *)(in_RDX + 0x5a);
  uVar2 = *(undefined4 *)(in_RDX + 0x5c);
  *(short *)(in_RCX + 1) = *(short *)(in_RDX + 0x58);
  *(undefined2 *)((longlong)in_RCX + 6) = uVar1;
  in_RCX[2] = uVar2;
  *in_RCX = 0;
  return;
}

