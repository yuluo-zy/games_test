// open_newsletter @ 0x0x1403d0af0
// sig: void __cdecl open_newsletter(void)
// size: 370


void __cdecl tiny_glade::gui::gui_escape_menu::open_newsletter(void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong *extraout_RAX;
  ulonglong *extraout_RAX_00;
  
  obfstr::xref::inner<9294081805341155440>();
  uVar1 = *extraout_RAX;
  uVar2 = extraout_RAX[1];
  uVar3 = extraout_RAX[2];
  uVar4 = extraout_RAX[3];
  uVar5 = extraout_RAX[4];
  uVar6 = extraout_RAX[5];
  uVar7 = extraout_RAX[6];
  uVar8 = extraout_RAX[7];
  uVar9 = extraout_RAX[8];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (ulonglong *)0x0) {
    *extraout_RAX_00 = uVar1 ^ 0x22a4a09645caa29a;
    extraout_RAX_00[1] = uVar2 ^ 0x4fa408136071426d;
    extraout_RAX_00[2] = uVar3 ^ 0x377d14e71a67e740;
    extraout_RAX_00[3] = uVar4 ^ 0x2f5515b329d67cf7;
    extraout_RAX_00[4] = uVar5 ^ 0xc64f987031855b02;
    extraout_RAX_00[5] = uVar6 ^ 0x6b1af7459d71d8d0;
    extraout_RAX_00[6] = uVar7 ^ 0x6b7c08028bbbfa3c;
    extraout_RAX_00[7] = uVar8 ^ 0x3224caaa5fb9abc;
    *(uint *)(extraout_RAX_00 + 8) = (uint)uVar9 ^ 0x1c5217d1;
    country_core::systems::steam::SteamOverlayOpenResources::
    open_overlay_or_browser_to_webpage<alloc::string::String>();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

