// get_cli_args
// sig: void __cdecl get_cli_args(void)
// size: 538

// WARNING: Control flow encountered bad instruction data

void __cdecl tiny_glade::app_config::get_cli_args(void)

{
  byte *pbVar1;
  char cVar2;
  byte bVar3;
  byte *pbVar4;
  longlong lVar5;
  uint uVar6;
  void *in_RCX;
  byte *pbVar7;
  longlong lVar8;
  char *pcVar9;
  longlong lVar10;
  char *pcVar11;
  int local_260 [2];
  longlong local_258;
  longlong local_110;
  undefined8 local_108;
  undefined1 *local_f8;
  code *local_f0;
  byte local_e8;
  byte bStack_e7;
  undefined8 uStack_e0;
  undefined **local_a8;
  longlong local_a0;
  undefined1 **local_98;
  longlong local_90;
  undefined8 local_88;
  longlong local_68;
  undefined8 local_60;
  longlong local_58;
  undefined1 local_50 [8];
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  clap_builder::derive::Parser::try_parse<tiny_glade::app_config::CliArgs>();
  if (local_260[0] != 2) {
    memcpy(in_RCX,local_260,0x150);
    return;
  }
  local_68 = local_258;
  clap_builder::error::Error::new<clap_builder::error::format::RichFormatter>();
  pcVar9 = *(char **)(local_258 + 0x28);
  pbVar4 = *(byte **)(local_258 + 0x40);
  pcVar11 = pcVar9 + *(longlong *)(local_258 + 0x30);
  lVar5 = *(longlong *)(local_258 + 0x48);
  pbVar1 = pbVar4;
LAB_140353560:
  do {
    pbVar7 = pbVar1;
    if (pcVar9 == pcVar11) {
      tg_iced::dialog::MessageDialog::new();
      tg_iced::dialog::MessageDialog::set_level();
      tg_iced::dialog::MessageDialog::set_title();
      local_f0 = clap_builder::error::impl$5::fmt<clap_builder::error::format::RichFormatter>;
      local_a8 = &PTR_DAT_1429c3850;
      local_a0 = 1;
      local_88 = 0;
      local_98 = &local_f8;
      local_90 = 1;
      local_f8 = local_50;
      alloc::fmt::format();
      local_58 = local_110;
      local_60 = local_108;
      tg_iced::dialog::MessageDialog::set_description();
      tg_iced::dialog::MessageDialog::show();
      if (local_58 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::
      drop_in_place<clap_builder::error::Error<clap_builder::error::format::RichFormatter>_>();
                    // WARNING: Subroutine does not return
      clap_builder::error::Error::exit<clap_builder::error::format::RichFormatter>();
    }
    if (pbVar7 == pbVar4 + lVar5 * 0x20) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    pbVar1 = pbVar7 + 0x20;
    cVar2 = *pcVar9;
    pcVar9 = pcVar9 + 1;
  } while (cVar2 == '\x0f');
  bVar3 = *pbVar7;
  uVar6 = *(int *)(&DAT_1429c4b78 + (ulonglong)bVar3 * 4) + 0x429c4b78;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bVar3) {
  case 0:
    break;
  case 1:
    bStack_e7 = pbVar7[1];
    break;
  case 2:
    alloc::string::impl$6::clone();
    break;
  case 3:
    alloc::vec::impl$11::clone<clap_builder::builder::styled_str::StyledStr,alloc::alloc::Global>();
    break;
  case 4:
    alloc::string::impl$6::clone();
    break;
  case 5:
    alloc::vec::impl$11::clone<clap_builder::builder::styled_str::StyledStr,alloc::alloc::Global>();
    break;
  case 6:
    uStack_e0 = *(undefined8 *)(pbVar7 + 8);
    break;
  default:
    goto code_r0x000140353633;
  case 8:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 10:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  local_e8 = bVar3;
  clap_builder::error::Error::insert<clap_builder::error::format::RichFormatter>();
  uVar6 = (uint)local_a8 & 0xff;
code_r0x000140353633:
  switch(uVar6) {
  case 2:
  case 4:
    if (local_a0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    goto LAB_140353560;
  case 3:
    if (local_90 != 0) {
      lVar8 = (longlong)local_98 + 8;
      lVar10 = local_90;
      do {
        if (*(longlong *)(lVar8 + -8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar8 = lVar8 + 0x18;
        lVar10 = lVar10 + -1;
      } while (lVar10 != 0);
    }
    break;
  case 5:
    if (local_90 != 0) {
      lVar8 = (longlong)local_98 + 8;
      lVar10 = local_90;
      do {
        if (*(longlong *)(lVar8 + -8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar8 = lVar8 + 0x18;
        lVar10 = lVar10 + -1;
      } while (lVar10 != 0);
    }
    break;
  default:
    goto LAB_140353560;
  }
  if (local_a0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_140353560;
}

