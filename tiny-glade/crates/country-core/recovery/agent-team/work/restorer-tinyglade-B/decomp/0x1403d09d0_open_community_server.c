// open_community_server @ 0x1403d09d0
// sig: void __cdecl open_community_server(void)


void __cdecl tiny_glade::gui::gui_escape_menu::open_community_server(void)

{
  byte bVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong *extraout_RAX;
  ulonglong *extraout_RAX_00;
  uint uVar6;
  
  obfstr::xref::inner<16115868519102756398>();
  uVar2 = *extraout_RAX;
  uVar3 = extraout_RAX[1];
  uVar4 = extraout_RAX[2];
  uVar5 = extraout_RAX[3];
  bVar1 = *(byte *)((longlong)extraout_RAX + 0x1c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (ulonglong *)0x0) {
    uVar6 = (uint)uVar5 ^ 0xa76feff9;
    *extraout_RAX_00 = uVar2 ^ 0x9a50bb85b50c41b8;
    extraout_RAX_00[1] = uVar3 ^ 0x8d73efbb292bffb5;
    extraout_RAX_00[2] = uVar4 ^ 0x812e7379e01a08d9;
    *(char *)(extraout_RAX_00 + 3) = (char)uVar6;
    *(char *)((longlong)extraout_RAX_00 + 0x1b) = (char)(uVar6 >> 0x18);
    *(short *)((longlong)extraout_RAX_00 + 0x19) = (short)(uVar6 >> 8);
    *(byte *)((longlong)extraout_RAX_00 + 0x1c) = bVar1 ^ 0x91;
    country_core::systems::steam::SteamOverlayOpenResources::
    open_overlay_or_browser_to_webpage<alloc::string::String>();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

