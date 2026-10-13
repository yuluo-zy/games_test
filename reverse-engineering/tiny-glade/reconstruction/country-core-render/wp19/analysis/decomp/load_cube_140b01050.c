// load_cube  0x140b01050
// signature: void __cdecl load_cube(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "capture", "address": "140d56700"}, {"name": "construct<anyhow::error::ContextError<alloc::string::String,anyhow::Error>_>", "address": "1428a94b0"}, {"name": "construct<anyhow::error::ContextError<ref$<str$>,anyhow::Error>_>", "address": "1428a8a20"}, {"name": "construct<anyhow::error::ContextError<ref$<str$>,core::num::error::ParseIntError>_>", "address": "1428a93d0"}, {"name": "create_texture<ref$<slice2$<u8>_>_>", "address": "1416a6700"}, {"name": "do_reserve_and_handle<alloc::alloc::Global>", "address": "1428a9a60"}, {"name": "format_err", "address": "1428a8020"}, {"name": "format_inner", "address": "140d79f90"}, {"name": "from<std::io::error::Error>", "address": "1428a9590"}, {"name": "inner", "address": "140d59140"}, {"name": "memchr_aligned", "address": "140d88a40"}, {"name": "memset", "address": "142923c20"}, {"name": "new_3d", "address": "140bd5310"}, {"name": "pack_rgb9e5", "address": "1417c66b0"}, {"name": "panic_bounds_check", "address": "1428d9518"}, {"name": "parse_rgb", "address": "140b01cc0"}, {"name": "sampled", "address": "140bd5370"}, {"name": "trim_matches<core::str::impl$0::trim::closure_env$0>", "address": "1400f9c30"}, {"name": "unwrap_failed", "address": "1428d9760"}]


void __cdecl country_core::resources::grading_lut::GradingLut::load_cube(void)

{
  undefined1 auVar1 [16];
  undefined **ppuVar2;
  longlong **pplVar3;
  int extraout_EAX;
  undefined4 uVar4;
  undefined8 extraout_RAX;
  longlong **extraout_RAX_00;
  longlong **pplVar5;
  uint *extraout_RAX_01;
  char *extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 uVar6;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 *in_RCX;
  char *pcVar7;
  uint uVar8;
  ulonglong uVar9;
  longlong **pplVar10;
  longlong **pplVar11;
  longlong **pplVar12;
  longlong **pplVar13;
  undefined1 uVar14;
  longlong **pplVar15;
  longlong **_Dst;
  ulonglong uVar16;
  longlong **pplVar17;
  longlong **pplVar18;
  longlong **pplVar19;
  bool bVar20;
  undefined8 *in_stack_00000028;
  undefined1 local_1d8 [8];
  ulonglong local_1d0;
  undefined4 local_1c4;
  undefined8 local_1c0;
  longlong local_1b8;
  undefined8 local_1b0;
  longlong **local_1a0;
  int local_198;
  undefined4 local_194;
  undefined8 local_190;
  longlong **local_188;
  longlong local_180;
  longlong *local_178;
  code *pcStack_170;
  undefined1 *local_168;
  code *pcStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  int local_120;
  float fStack_11c;
  float fStack_118;
  float fStack_114;
  undefined8 local_110;
  longlong local_100;
  longlong **local_f8;
  longlong **local_f0;
  undefined **local_e8;
  longlong **pplStack_e0;
  longlong **local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined **local_98;
  longlong **local_90;
  undefined8 *local_88;
  longlong **local_80;
  longlong **local_78;
  ulonglong local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_88 = in_RCX;
  std::fs::read_to_string::inner();
  pplVar3 = local_d8;
  local_98 = local_e8;
  if (!SBORROW8(0,(longlong)local_e8)) {
    local_80 = pplStack_e0;
    local_100 = 0;
    local_f8 = (longlong **)&DAT_00000004;
    local_f0 = (longlong **)0x0;
    local_70 = 0x1f;
    local_90 = (longlong **)&DAT_00000004;
    pplVar10 = (longlong **)0x0;
    local_188 = (longlong **)0x0;
    local_180 = 0;
    pplVar5 = (longlong **)0x0;
    pplVar17 = (longlong **)0x0;
    local_78 = (longlong **)0x0;
    _Dst = pplStack_e0;
    pplVar12 = (longlong **)0x0;
    pplVar19 = local_f0;
    pplVar13 = local_78;
LAB_140b0114d:
    local_78 = pplVar13;
    local_f0 = pplVar19;
    pplVar19 = pplVar17;
    pplVar13 = pplVar17;
    pplVar15 = pplVar5;
    if (((ulonglong)pplVar17 & 1) == 0) {
joined_r0x000140b0115e:
      do {
        pplVar5 = local_80;
        pplVar11 = pplVar10;
        local_80 = pplVar5;
        if (pplVar3 < pplVar10) {
LAB_140b01213:
          local_188 = (longlong **)CONCAT71((int7)((ulonglong)pplVar19 >> 8),1);
          pplVar17 = (longlong **)CONCAT71((int7)((ulonglong)pplVar13 >> 8),1);
          pplVar18 = pplVar3;
          pplVar10 = pplVar11;
          pplVar5 = pplVar15;
          if (pplVar3 != pplVar15) break;
          goto LAB_140b016ab;
        }
        uVar9 = (longlong)pplVar5 + (longlong)pplVar10;
        pplVar13 = pplVar17;
        pplVar11 = pplVar3;
        if ((ulonglong)((longlong)pplVar3 - (longlong)pplVar10) < 0x10) {
          pplVar19 = pplVar5;
          if (pplVar3 != pplVar10) {
            uVar16 = 0;
            do {
              if (*(char *)(uVar9 + uVar16) == '\n') goto LAB_140b011e4;
              uVar16 = uVar16 + 1;
            } while ((longlong)pplVar3 - (longlong)pplVar10 != uVar16);
          }
          goto LAB_140b01213;
        }
        pplVar18 = (longlong **)CONCAT71((int7)((ulonglong)_Dst >> 8),10);
        uVar16 = uVar9;
        core::slice::memchr::memchr_aligned();
        pplVar5 = extraout_RAX_00;
        _Dst = pplVar18;
        if (extraout_RAX_00 != (longlong **)0x1) {
          pplVar19 = extraout_RAX_00;
          pplVar10 = pplVar3;
          pplVar5 = pplVar3;
          if (((ulonglong)extraout_RAX_00 & 1) == 0) goto LAB_140b01213;
          break;
        }
LAB_140b011e4:
        pplVar18 = (longlong **)((longlong)pplVar10 + uVar16 + 1);
        pplVar11 = (longlong **)((longlong)pplVar10 + uVar16);
        pplVar19 = pplVar5;
        pplVar10 = pplVar18;
      } while ((pplVar3 <= pplVar11) || (pplVar5 = pplVar18, *(char *)(uVar16 + uVar9) != '\n'));
      uVar9 = (longlong)pplVar18 - (longlong)pplVar15;
      if (uVar9 != 0) {
        uVar16 = uVar9;
        if (*(char *)((longlong)local_80 + -1 + (longlong)pplVar18) != '\n') goto LAB_140b01272;
        if (uVar9 - 1 != 0) {
          uVar16 = uVar9 - 2;
          if (*(char *)((longlong)local_80 + -2 + (longlong)pplVar18) != '\r') {
            uVar16 = uVar9 - 1;
          }
          goto LAB_140b01272;
        }
      }
      uVar16 = 0;
LAB_140b01272:
      _Dst = (longlong **)((longlong)pplVar15 + (longlong)local_80);
      local_180 = local_180 + 1;
      core::str::impl$0::trim_matches<core::str::impl_0::trim::closure_env_0>();
      local_1d0 = uVar16;
      pplVar19 = local_f0;
      pplVar13 = local_78;
      if ((uVar16 == 0) || ((char)*extraout_RAX_01 == '#')) goto LAB_140b0114d;
      if (4 < uVar16) {
        _Dst = (longlong **)(ulonglong)(*extraout_RAX_01 ^ 0x4c544954);
        if ((char)extraout_RAX_01[1] == 'E' && (*extraout_RAX_01 ^ 0x4c544954) == 0)
        goto LAB_140b0114d;
        if (9 < uVar16) {
          if ((short)extraout_RAX_01[2] == 0x4e49 &&
              *(longlong *)extraout_RAX_01 == 0x4d5f4e49414d4f44) {
            grading_lut::impl$2::load_cube::parse_rgb();
            if (local_120 == 1) {
              local_168 = (undefined1 *)CONCAT44(fStack_114,fStack_118);
              local_178 = (longlong *)0x142ae6cd0;
              pcStack_170 = (code *)0x12;
              local_e8 = (undefined **)0x3;
              anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,anyhow::Error>_>();
              uVar6 = extraout_RAX_07;
              goto LAB_140b018b1;
            }
            if ((ABS(fStack_11c) == 0.0) &&
               (_Dst = (longlong **)0x7fffffff7fffffff, pplVar19 = local_f0, pplVar13 = local_78,
               (CONCAT44(fStack_114,fStack_118) & 0x7fffffff7fffffff) == 0)) goto LAB_140b0114d;
            local_e8 = (undefined **)&DAT_142ae6d00;
            pplStack_e0 = (longlong **)0x1;
            local_d8 = (longlong **)&DAT_00000008;
            uStack_d0 = 0;
            local_c8 = 0;
            anyhow::__private::format_err();
            uVar6 = extraout_RAX_08;
            goto LAB_140b018b1;
          }
          if ((short)extraout_RAX_01[2] == 0x5841 &&
              *(longlong *)extraout_RAX_01 == 0x4d5f4e49414d4f44) {
            _Dst = (longlong **)&local_120;
            grading_lut::impl$2::load_cube::parse_rgb();
            if (local_120 == 1) {
              local_168 = (undefined1 *)CONCAT44(fStack_114,fStack_118);
              local_178 = (longlong *)0x142ae6d10;
              pcStack_170 = (code *)0x12;
              local_e8 = (undefined **)0x3;
              anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,anyhow::Error>_>();
              uVar6 = extraout_RAX_09;
              goto LAB_140b018b1;
            }
            if (((((fStack_11c == 1.0) && (!NAN(fStack_11c))) && (fStack_118 == 1.0)) &&
                ((!NAN(fStack_118) && (fStack_114 == 1.0)))) &&
               (pplVar19 = local_f0, pplVar13 = local_78, !NAN(fStack_114))) goto LAB_140b0114d;
            local_e8 = (undefined **)&DAT_142ae6d40;
            pplStack_e0 = (longlong **)0x1;
            local_d8 = (longlong **)&DAT_00000008;
            uStack_d0 = 0;
            local_c8 = 0;
            anyhow::__private::format_err();
            uVar6 = extraout_RAX_10;
            goto LAB_140b018b1;
          }
          if (uVar16 != 10) {
            if (*(longlong *)((longlong)extraout_RAX_01 + 3) == 0x455a49535f44315f &&
                *(longlong *)extraout_RAX_01 == 0x535f44315f54554c) {
              local_e8 = &PTR_s_1D_LUTs_are_not_supported_142ae6de8;
              pplStack_e0 = (longlong **)0x1;
              local_d8 = (longlong **)&DAT_00000008;
              uStack_d0 = 0;
              local_c8 = 0;
              anyhow::__private::format_err();
              uVar6 = extraout_RAX_05;
              goto LAB_140b018b1;
            }
            if (*(longlong *)((longlong)extraout_RAX_01 + 3) == 0x455a49535f44335f &&
                *(longlong *)extraout_RAX_01 == 0x535f44335f54554c) goto LAB_140b0149d;
          }
        }
      }
      grading_lut::impl$2::load_cube::parse_rgb();
      if (local_198 == 1) {
        local_1b0 = local_190;
        local_1b8 = local_180;
        local_178 = &local_1b8;
        pcStack_170 = core::fmt::num::imp::impl$9::fmt;
        local_168 = local_1d8;
        pcStack_160 = core::fmt::impl$74::fmt<str_>;
        local_e8 = &anon_de0eed98b15ac5150e4d56a6eb006e4c_143_llvm_7941911296138889869;
        pplStack_e0 = (longlong **)0x2;
        local_d8 = &local_178;
        uStack_d0 = 2;
        local_c8 = 0;
        local_61 = 1;
        alloc::fmt::format::format_inner();
        local_168 = (undefined1 *)local_110;
        local_178 = (longlong *)CONCAT44(fStack_11c,local_120);
        pcStack_170 = (code *)CONCAT44(fStack_114,fStack_118);
        pcStack_160 = (code *)local_190;
        local_e8 = (undefined **)0x3;
        local_61 = 0;
        anyhow::Error::construct<anyhow::error::ContextError<alloc::string::String,anyhow::Error>_>
                  ();
        uVar6 = extraout_RAX_03;
        goto LAB_140b018b1;
      }
      local_1c4 = local_194;
      local_1c0 = local_190;
      if (local_78 < (longlong **)((longlong)pplVar12 + 1)) {
        local_e8 = &PTR_s_Too_many_entries_in_the_LUT__Bai_142ae6db8;
        pplStack_e0 = (longlong **)0x1;
        local_d8 = (longlong **)&DAT_00000008;
        uStack_d0 = 0;
        local_c8 = 0;
        anyhow::__private::format_err();
        uVar6 = extraout_RAX_04;
        goto LAB_140b018b1;
      }
      ::utils::rgb9e5::pack_rgb9e5();
      if (local_78 <= pplVar12) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      *(int *)((longlong)local_90 + (longlong)pplVar12 * 4) = extraout_EAX;
      pplVar19 = local_188;
      _Dst = local_90;
      pplVar13 = pplVar17;
      pplVar15 = pplVar5;
      pplVar12 = (longlong **)((longlong)pplVar12 + 1);
      pplVar17 = local_188;
      if (((ulonglong)local_188 & 1) != 0) goto LAB_140b016ab;
      goto joined_r0x000140b0115e;
    }
LAB_140b016ab:
    uVar4 = (undefined4)local_70;
    local_178 = (longlong *)CONCAT44(uVar4,uVar4);
    pcStack_170 = (code *)CONCAT44(pcStack_170._4_4_,uVar4);
    rhapsody::raw::texture::TextureDesc::new_3d();
    rhapsody::raw::texture::TextureDesc::sampled();
    rhapsody::raw::device::GraphicsDevice::create_texture<ref_<slice2_<u8>_>_>();
    if ((int)local_178 != 0xb) {
      local_a8 = local_138;
      uStack_a0 = uStack_130;
      local_b8 = local_148;
      uStack_b0 = uStack_140;
      local_c8 = local_158;
      uStack_c0 = uStack_150;
      local_d8 = (longlong **)local_168;
      uStack_d0 = pcStack_160;
      local_e8 = (undefined **)local_178;
      pplStack_e0 = (longlong **)pcStack_170;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_88[0xb] = uStack_140;
    local_88[9] = uStack_150;
    local_88[10] = local_148;
    local_88[7] = pcStack_160;
    local_88[8] = local_158;
    local_88[5] = pcStack_170;
    local_88[6] = local_168;
    local_88[2] = in_stack_00000028[2];
    uVar6 = in_stack_00000028[1];
    *local_88 = *in_stack_00000028;
    local_88[1] = uVar6;
    uVar4 = (undefined4)local_70;
    *(undefined4 *)(local_88 + 3) = uVar4;
    *(undefined4 *)((longlong)local_88 + 0x1c) = uVar4;
    *(undefined4 *)(local_88 + 4) = uVar4;
    ppuVar2 = local_98;
    if (local_100 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      ppuVar2 = local_98;
    }
    goto joined_r0x000140b0179d;
  }
  anyhow::error::impl$1::from<std::io::error::Error>();
  local_88[1] = extraout_RAX;
  *local_88 = 0x8000000000000000;
  goto LAB_140b01909;
LAB_140b0149d:
  uVar16 = uVar16 - 0xb;
  core::str::impl$0::trim_matches<core::str::impl_0::trim::closure_env_0>();
  pcVar7 = extraout_RAX_02;
  if (uVar16 == 1) {
    uVar14 = 1;
    if ((*extraout_RAX_02 != '+') && (uVar16 = 1, *extraout_RAX_02 != '-')) goto LAB_140b01505;
  }
  else {
    if (uVar16 != 0) {
      if (*extraout_RAX_02 == '+') {
        pcVar7 = extraout_RAX_02 + 1;
        uVar9 = uVar16 - 1;
        bVar20 = uVar16 < 0x12;
        uVar16 = uVar9;
        if (bVar20) {
LAB_140b01505:
          uVar9 = 0;
          local_70 = 0;
          do {
            if (9 < (byte)pcVar7[uVar9] - 0x30) goto LAB_140b01988;
            local_70 = (ulonglong)((byte)pcVar7[uVar9] - 0x30) + local_70 * 10;
            uVar9 = uVar9 + 1;
          } while (uVar16 != uVar9);
          goto LAB_140b01542;
        }
      }
      else {
        uVar9 = uVar16;
        if (uVar16 < 0x11) goto LAB_140b01505;
      }
      uVar16 = 0;
      local_70 = 0;
      while (uVar9 != uVar16) {
        auVar1._8_8_ = 0;
        auVar1._0_8_ = local_70;
        local_70 = SUB168(auVar1 * ZEXT816(10),0);
        uVar8 = (byte)pcVar7[uVar16] - 0x30;
        if (9 < uVar8) goto LAB_140b01988;
        uVar14 = 2;
        if (SUB168(auVar1 * ZEXT816(10),8) != 0) goto LAB_140b0198b;
        uVar16 = uVar16 + 1;
        bVar20 = CARRY8(local_70,(ulonglong)uVar8);
        local_70 = local_70 + uVar8;
        if (bVar20) goto LAB_140b0198b;
      }
LAB_140b01542:
      pplVar19 = (longlong **)(local_70 * local_70 * local_70);
      uVar9 = (longlong)pplVar19 - (longlong)local_78;
      _Dst = local_78;
      local_1a0 = pplVar12;
      pplVar13 = pplVar19;
      if (local_78 <= pplVar19 && uVar9 != 0) {
        if ((ulonglong)(local_100 - (longlong)local_78) < uVar9) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          _Dst = (longlong **)((longlong)local_f8 + (longlong)local_f0 * 4);
          local_90 = local_f8;
          pplVar13 = local_f0;
        }
        else {
          _Dst = (longlong **)((longlong)local_90 + (longlong)local_78 * 4);
          pplVar13 = local_78;
        }
        if (1 < uVar9) {
          memset(_Dst,0,(~(ulonglong)local_78 + (longlong)pplVar19) * 4);
          _Dst = (longlong **)
                 ((longlong)local_90 +
                 ((longlong)((longlong)pplVar19 + (longlong)pplVar13) - (longlong)local_78) * 4 + -4
                 );
          pplVar13 = (longlong **)((longlong)pplVar13 + (uVar9 - 1));
        }
        *(int *)_Dst = 0;
        pplVar12 = local_1a0;
        pplVar19 = (longlong **)((longlong)pplVar13 + 1);
        pplVar13 = (longlong **)((longlong)pplVar13 + 1);
      }
      goto LAB_140b0114d;
    }
    uVar14 = 0;
  }
  goto LAB_140b0198b;
LAB_140b01988:
  uVar14 = 1;
LAB_140b0198b:
  std::backtrace::Backtrace::capture();
  local_178 = (longlong *)0x142ae6d50;
  pcStack_170 = (code *)0x13;
  local_168 = (undefined1 *)CONCAT71(local_168._1_7_,uVar14);
  anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,core::num::error::ParseIntError>_>
            ();
  uVar6 = extraout_RAX_06;
LAB_140b018b1:
  local_88[1] = uVar6;
  *local_88 = 0x8000000000000000;
  if (local_100 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_98 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_140b01909:
  ppuVar2 = (undefined **)*in_stack_00000028;
joined_r0x000140b0179d:
  if (ppuVar2 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

