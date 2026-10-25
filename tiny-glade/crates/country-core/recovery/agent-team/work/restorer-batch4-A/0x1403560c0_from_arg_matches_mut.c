// from_arg_matches_mut
// sig: void __cdecl from_arg_matches_mut(void)
// size: 1542

void __cdecl tiny_glade::app_config::impl$5::from_arg_matches_mut(void)

{
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  char extraout_AL_09;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  undefined8 extraout_RAX_12;
  undefined8 uVar1;
  undefined8 *in_RCX;
  undefined *puVar2;
  undefined *puVar3;
  undefined *puVar4;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  longlong local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  longlong local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  longlong local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  longlong local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  
  clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<alloc::string::String>();
  clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
  if (extraout_AL == '\x02') {
    clap_builder::error::Error::raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
    uVar1 = extraout_RAX;
  }
  else {
    clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
    if (extraout_AL_00 == '\x02') {
      clap_builder::error::Error::raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
      uVar1 = extraout_RAX_00;
    }
    else {
      clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
      if (extraout_AL_01 == '\x02') {
        clap_builder::error::Error::raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
        uVar1 = extraout_RAX_01;
      }
      else {
        clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
        if (extraout_AL_02 == '\x02') {
          clap_builder::error::Error::raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
          uVar1 = extraout_RAX_02;
        }
        else {
          clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
          if (extraout_AL_03 == '\x02') {
            clap_builder::error::Error::raw<clap_builder::error::format::RichFormatter,ref_<str_>_>
                      ();
            uVar1 = extraout_RAX_03;
          }
          else {
            clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
            if (extraout_AL_04 == '\x02') {
              clap_builder::error::Error::
              raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
              uVar1 = extraout_RAX_04;
            }
            else {
              clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
              if (extraout_AL_05 == '\x02') {
                clap_builder::error::Error::
                raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
                uVar1 = extraout_RAX_05;
              }
              else {
                puVar2 = &DAT_1429c3c20;
                clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<usize>();
                clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
                if (extraout_AL_06 != '\x02') {
                  clap_builder::parser::matches::arg_matches::ArgMatches::
                  remove_one<std::path::PathBuf>();
                  puVar3 = &DAT_1429c3c5a;
                  clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<u64>();
                  clap_builder::parser::matches::arg_matches::ArgMatches::
                  remove_one<std::path::PathBuf>();
                  clap_builder::parser::matches::arg_matches::ArgMatches::
                  remove_one<std::path::PathBuf>();
                  clap_builder::parser::matches::arg_matches::ArgMatches::
                  remove_one<std::path::PathBuf>();
                  puVar4 = &DAT_1429c3c97;
                  clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<usize>();
                  clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
                  if (extraout_AL_07 == '\x02') {
                    clap_builder::error::Error::
                    raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
                    uVar1 = extraout_RAX_10;
                  }
                  else {
                    clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
                    if (extraout_AL_08 == '\x02') {
                      clap_builder::error::Error::
                      raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
                      uVar1 = extraout_RAX_11;
                    }
                    else {
                      clap_builder::parser::matches::arg_matches::ArgMatches::remove_one<bool>();
                      if (extraout_AL_09 != '\x02') {
                        clap_builder::parser::matches::arg_matches::ArgMatches::
                        remove_one<std::path::PathBuf>();
                        clap_builder::parser::matches::arg_matches::ArgMatches::
                        remove_one<std::path::PathBuf>();
                        clap_builder::parser::matches::arg_matches::ArgMatches::
                        remove_one<std::path::PathBuf>();
                        clap_builder::parser::matches::arg_matches::ArgMatches::
                        remove_one<alloc::string::String>();
                        in_RCX[8] = local_50;
                        in_RCX[6] = local_60;
                        in_RCX[7] = uStack_58;
                        in_RCX[0xb] = local_78;
                        in_RCX[0xc] = uStack_70;
                        in_RCX[9] = local_88;
                        in_RCX[10] = uStack_80;
                        in_RCX[0xf] = local_98;
                        in_RCX[0x10] = uStack_90;
                        in_RCX[0xd] = local_a8;
                        in_RCX[0xe] = uStack_a0;
                        in_RCX[0x13] = local_b8;
                        in_RCX[0x14] = uStack_b0;
                        in_RCX[0x11] = local_c8;
                        in_RCX[0x12] = uStack_c0;
                        in_RCX[0x17] = local_d8;
                        in_RCX[0x18] = uStack_d0;
                        in_RCX[0x15] = local_e8;
                        in_RCX[0x16] = uStack_e0;
                        in_RCX[0x19] = local_138;
                        in_RCX[0x1a] = uStack_130;
                        in_RCX[0x1b] = local_128;
                        in_RCX[0x1c] = uStack_120;
                        in_RCX[0x1d] = local_158;
                        in_RCX[0x1e] = uStack_150;
                        in_RCX[0x1f] = local_148;
                        in_RCX[0x20] = uStack_140;
                        in_RCX[0x23] = local_168;
                        in_RCX[0x24] = uStack_160;
                        in_RCX[0x21] = local_178;
                        in_RCX[0x22] = uStack_170;
                        in_RCX[0x27] = local_180;
                        in_RCX[0x25] = local_190;
                        in_RCX[0x26] = uStack_188;
                        *in_RCX = extraout_RAX_06;
                        in_RCX[1] = puVar2;
                        in_RCX[2] = extraout_RAX_08;
                        in_RCX[3] = puVar3;
                        in_RCX[4] = extraout_RAX_09;
                        in_RCX[5] = puVar4;
                        *(char *)(in_RCX + 0x28) = extraout_AL;
                        *(char *)((longlong)in_RCX + 0x141) = extraout_AL_00;
                        *(char *)((longlong)in_RCX + 0x142) = extraout_AL_01;
                        *(char *)((longlong)in_RCX + 0x143) = extraout_AL_02;
                        *(char *)((longlong)in_RCX + 0x144) = extraout_AL_03;
                        *(char *)((longlong)in_RCX + 0x145) = extraout_AL_04;
                        *(char *)((longlong)in_RCX + 0x146) = extraout_AL_05;
                        *(char *)((longlong)in_RCX + 0x147) = extraout_AL_06;
                        *(char *)(in_RCX + 0x29) = extraout_AL_07;
                        *(char *)((longlong)in_RCX + 0x149) = extraout_AL_08;
                        *(char *)((longlong)in_RCX + 0x14a) = extraout_AL_09;
                        return;
                      }
                      clap_builder::error::Error::
                      raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
                      uVar1 = extraout_RAX_12;
                    }
                  }
                  in_RCX[1] = uVar1;
                  *in_RCX = 2;
                  if ((!SBORROW8(0,local_e8)) && (local_e8 != 0)) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  if ((!SBORROW8(0,local_c8)) && (local_c8 != 0)) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  if ((!SBORROW8(0,local_a8)) && (local_a8 != 0)) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  if ((!SBORROW8(0,local_88)) && (local_88 != 0)) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  goto joined_r0x0001403564eb;
                }
                clap_builder::error::Error::
                raw<clap_builder::error::format::RichFormatter,ref_<str_>_>();
                uVar1 = extraout_RAX_07;
              }
            }
          }
        }
      }
    }
  }
  in_RCX[1] = uVar1;
  *in_RCX = 2;
joined_r0x0001403564eb:
  if ((!SBORROW8(0,local_60)) && (local_60 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

