// 0x14068eb50 instantiate_ui_template_impl
// signature: void __cdecl instantiate_ui_template_impl(void)

void __cdecl toolbar::mei::config::instantiate_ui_template_impl(void)

{
  longlong *plVar1;
  byte bVar2;
  int *piVar3;
  char **ppcVar4;
  char **ppcVar5;
  uint uVar6;
  undefined1 uVar7;
  byte bVar8;
  char ***extraout_RAX;
  char ***extraout_RAX_00;
  ulonglong extraout_RAX_01;
  char ***extraout_RAX_02;
  char ***pppcVar9;
  longlong extraout_RAX_03;
  void *_Src;
  char ***extraout_RAX_04;
  longlong extraout_RAX_05;
  char ***pppcVar10;
  char ***pppcVar11;
  char ***in_RDX;
  char *pcVar12;
  char *pcVar13;
  undefined1 uVar14;
  ushort uVar15;
  ulonglong uVar16;
  longlong lVar17;
  char *pcVar18;
  char ***_Size;
  longlong *plVar19;
  char *pcVar20;
  undefined1 uVar21;
  char ***pppcVar22;
  uint uVar23;
  char ***in_R8;
  ulonglong uVar24;
  longlong in_R9;
  longlong lVar25;
  char ***pppcVar26;
  int *piVar27;
  uint uVar28;
  undefined4 uVar29;
  char ***pppcVar30;
  char *pcVar31;
  undefined1 uVar32;
  undefined4 uVar33;
  undefined4 uVar34;
  bool bVar35;
  char cVar36;
  char cVar38;
  char cVar39;
  char cVar40;
  undefined1 auVar37 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined4 uVar43;
  undefined4 uVar44;
  undefined4 uVar45;
  undefined4 uVar46;
  char **local_1f8;
  code *pcStack_1f0;
  char *local_1e8;
  char ***pppcStack_1e0;
  char *local_1d8;
  char *pcStack_1d0;
  char *local_1c8;
  char *pcStack_1c0;
  char ***local_1b0;
  longlong *local_1a8;
  char ***local_1a0;
  char *local_198;
  char ***local_190;
  char ***local_188;
  char ***pppcStack_180;
  char ***local_178;
  char ***local_170;
  char ***local_168;
  longlong *local_160;
  char ***local_158;
  char ***pppcStack_150;
  char ***local_148;
  undefined8 local_140;
  ulonglong local_138;
  char ***local_130;
  char ***local_120;
  longlong local_110;
  char ***local_108;
  longlong local_100;
  char ***local_f8;
  char ***local_f0;
  char *local_e8;
  char ***local_e0;
  char ***pppcStack_d8;
  char ***local_d0;
  char ***local_c8;
  char ***pppcStack_c0;
  char ***local_b8;
  char ***local_a8;
  char ***local_a0;
  char ***local_98;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  uVar33 = 0;
  if (!SBORROW8(0,*(longlong *)(in_R9 + 0x30))) {
    local_158 = *(char ****)(in_R9 + 0x38);
    pppcStack_150 = *(char ****)(in_R9 + 0x40);
    local_190 = in_RDX;
    ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
    pppcStack_150 = (char ***)0x41223169ff28813b;
    local_148 = (char ***)0xa79b7268a2a968d9;
    ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
    local_158 = local_190;
    pppcStack_150 = extraout_RAX;
    ::utils::calculate_hash<tuple_<u64,u64>_>();
    in_R8 = extraout_RAX_00;
  }
  local_1b0 = in_R8;
  core::NodeContext::child_with_global_id();
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::FrameContent>();
  local_c8 = local_158;
  pppcStack_c0 = pppcStack_150;
  local_b8 = local_148;
  alloc::vec::impl$11::clone<enum2_<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>();
  widgets::FrameView::draw();
  pppcVar30 = local_e0;
  local_188 = local_e0;
  pppcStack_180 = pppcStack_d8;
  local_178 = local_d0;
  local_1d8 = (char *)0x0;
  pcStack_1d0 = (char *)0x0;
  local_1c8 = (char *)0x0;
  pcStack_1c0 = (char *)0x0;
  if (*(longlong *)(in_R9 + 0x28) != 0) {
    piVar3 = *(int **)(in_R9 + 0x20);
    uVar14 = 4;
    uVar21 = 0;
    uVar43 = 0;
    uVar28 = 7;
    if (*piVar3 - 2U < 7) {
      uVar28 = *piVar3 - 2U;
    }
    uVar29 = 0;
    uVar45 = 0;
    uVar34 = 0;
    uVar33 = 0;
    uVar46 = 0;
    uVar44 = 0;
    lVar25 = 0;
    uVar32 = 0;
    piVar27 = piVar3;
    uVar7 = 0;
    switch(uVar28) {
    case 0:
      goto LAB_14068fab0;
    case 1:
      goto code_r0x00014068fae7;
    case 2:
      goto code_r0x00014068fb16;
    case 3:
      goto code_r0x00014068fb06;
    case 4:
      goto code_r0x00014068fb0e;
    case 5:
      goto code_r0x00014068faec;
    case 6:
      goto code_r0x00014068fb20;
    default:
      goto LAB_14068fb2e;
    }
  }
  local_98 = (char ***)CONCAT71(local_98._1_7_,4);
  uVar44 = 0;
  uVar46 = 0;
  uVar34 = 0;
  uVar45 = 0;
  uVar29 = 0;
  uVar43 = 0;
  uVar21 = 0;
  uVar32 = 0;
  goto LAB_14068ecf5;
LAB_14068eefe:
  while( true ) {
    plVar1 = (longlong *)(pppcVar30 + 0xc);
    if (SBORROW8(0,(longlong)*pppcVar30)) break;
    pppcStack_150 = (char ***)0x41223169ff28813b;
    local_148 = (char ***)0xa79b7268a2a968d9;
    local_158 = pppcVar9;
    ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
    local_158 = local_1b0;
    pppcStack_150 = extraout_RAX_02;
    ::utils::calculate_hash<tuple_<u64,u64>_>();
    instantiate_ui_template_impl();
    pppcVar9 = (char ***)((longlong)pppcVar9 + 1);
    pppcVar30 = (char ***)plVar1;
    if (plVar1 == plVar19) {
      return;
    }
  }
  pcVar20 = (char *)pppcVar30[2];
  _Size = (char ***)pppcVar30[3];
  local_e0 = (char ***)0x0;
  pppcStack_d8 = (char ***)0x1;
  local_f0 = (char ***)0x1;
  local_d0 = (char ***)0x0;
  local_1e8 = pcVar20;
  pppcStack_1e0 = _Size;
  local_160 = (longlong *)pppcVar30;
  local_a8 = (char ***)plVar1;
  local_98 = (char ***)((longlong)pppcVar9 + 1);
  while (local_e8 = pcVar20, local_a0 = local_d0, ::core::str::pattern::StrSearcher::new(),
        pppcVar9 = local_98, pppcVar30 = local_a8, plVar19 = local_160,
        ((ulonglong)local_158 & 1) == 0) {
    if (local_140._2_1_ != '\0') goto LAB_14068f8f4;
    pppcVar26 = pppcStack_150;
    bVar8 = (byte)local_140;
LAB_14068f0f5:
    if (pppcVar26 != (char ***)0x0) {
      if (pppcVar26 < local_108) {
        if (*(char *)((longlong)pppcVar26 + local_110) < -0x40) {
LAB_14068f9e5:
                    // WARNING: Subroutine does not return
          ::core::str::slice_error_fail();
        }
      }
      else if (pppcVar26 != local_108) goto LAB_14068f9e5;
    }
    if (pppcVar26 != local_108) {
      bVar2 = *(byte *)((longlong)pppcVar26 + local_110);
      uVar28 = (uint)bVar2;
      if ((char)bVar2 < '\0') {
        uVar23 = bVar2 & 0x1f;
        uVar28 = *(byte *)((longlong)pppcVar26 + local_110 + 1) & 0x3f;
        if (bVar2 < 0xe0) {
          uVar28 = uVar23 << 6 | uVar28;
        }
        else {
          uVar28 = *(byte *)((longlong)pppcVar26 + local_110 + 2) & 0x3f | uVar28 << 6;
          if (bVar2 < 0xf0) {
            uVar28 = uVar28 | uVar23 << 0xc;
          }
          else {
            uVar28 = *(byte *)((longlong)pppcVar26 + local_110 + 3) & 0x3f | uVar28 << 6 |
                     (bVar2 & 7) << 0x12;
          }
        }
      }
      if ((bVar8 & 1) != 0) goto joined_r0x00014068f366;
      lVar25 = 1;
      if ((0x7f < uVar28) && (lVar25 = 2, 0x7ff < uVar28)) {
        lVar25 = 4 - (ulonglong)(uVar28 < 0x10000);
      }
      bVar8 = bVar8 ^ 1;
      pppcVar26 = (char ***)((longlong)pppcVar26 + lVar25);
      goto LAB_14068f0f5;
    }
    pppcVar26 = local_108;
    if ((bVar8 & 1) == 0) goto LAB_14068f8f4;
joined_r0x00014068f366:
    if (pppcVar26 != (char ***)0x0) {
      if (pppcVar26 < _Size) {
        if (local_e8[(longlong)pppcVar26] < -0x40) {
LAB_14068fa37:
                    // WARNING: Subroutine does not return
          ::core::str::slice_error_fail();
        }
      }
      else if (pppcVar26 != _Size) goto LAB_14068fa37;
    }
    if ((char ***)((longlong)local_e0 - (longlong)local_a0) < pppcVar26) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      local_f0 = pppcStack_d8;
      local_a0 = local_d0;
    }
    pppcVar10 = local_a0;
    pcVar20 = local_e8;
    memcpy((char *)((longlong)local_f0 + (longlong)local_a0),local_e8,(size_t)pppcVar26);
    local_d0 = (char ***)((longlong)pppcVar10 + (longlong)pppcVar26);
    pppcVar26 = (char ***)((longlong)pppcVar26 + 3);
    local_a0 = local_d0;
    if (pppcVar26 != (char ***)0x0) {
      if (pppcVar26 < _Size) {
        if (pcVar20[(longlong)pppcVar26] < -0x40) {
LAB_14068fa18:
                    // WARNING: Subroutine does not return
          ::core::str::slice_error_fail();
        }
      }
      else if (_Size != pppcVar26) goto LAB_14068fa18;
    }
    pcVar18 = (char *)((longlong)_Size - (longlong)pppcVar26);
    pcVar20 = pcVar20 + (longlong)pppcVar26;
    pcVar31 = (char *)0x0;
LAB_14068f419:
    pcVar13 = pcVar20 + (longlong)pcVar31;
    if (&DAT_0000000f < pcVar18 + -(longlong)pcVar31) {
      ::core::slice::memchr::memchr_aligned();
      pcVar12 = pcVar13;
      if (extraout_RAX_03 == 1) goto LAB_14068f477;
      goto LAB_14068f590;
    }
    if (pcVar18 == pcVar31) goto LAB_14068f590;
    pcVar12 = (char *)0x0;
    while (pcVar13[(longlong)pcVar12] != ')') {
      pcVar12 = pcVar12 + 1;
      if (pcVar18 + -(longlong)pcVar31 == pcVar12) goto LAB_14068f590;
    }
LAB_14068f477:
    pcVar13 = pcVar12 + (longlong)pcVar31;
    pcVar31 = pcVar31 + (longlong)pcVar12 + 1;
    if ((pcVar18 <= pcVar13) || (pcVar20[(longlong)pcVar13] != ')')) goto LAB_14068f410;
    if (pcVar18 <= pcVar13) {
      if (pcVar13 != pcVar18) {
                    // WARNING: Subroutine does not return
        ::core::str::slice_error_fail();
      }
LAB_14068fa5f:
                    // WARNING: Subroutine does not return
      ::core::str::slice_error_fail();
    }
    if (pcVar31 < pcVar18) {
      pcVar12 = pcVar31;
      if (pcVar20[(longlong)pcVar31] < -0x40) goto LAB_14068fa5f;
    }
    else {
      pcVar12 = pcVar18;
      if (pcVar31 != pcVar18) goto LAB_14068fa5f;
    }
    lang_lib::lookup();
    if ((char *)((longlong)local_e0 - (longlong)local_a0) < pcVar13) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      local_a0 = local_d0;
    }
    pppcVar30 = local_a0;
    pcVar20 = pcVar20 + (longlong)pcVar12;
    _Size = (char ***)(pcVar18 + -(longlong)pcVar12);
    local_f0 = pppcStack_d8;
    memcpy((char *)((longlong)pppcStack_d8 + (longlong)local_a0),_Src,(size_t)pcVar13);
    local_d0 = (char ***)(pcVar13 + (longlong)pppcVar30);
  }
  local_198 = (char *)((longlong)local_f8 + -1);
  pppcVar26 = local_130;
  if (local_120 == (char ***)0xffffffffffffffff) {
    pppcVar10 = (char ***)((longlong)local_130 + (longlong)local_198);
    if (pppcVar10 < local_108) {
      local_1a0 = local_140;
      local_170 = (char ***)((longlong)pppcStack_150 + -1);
      local_168 = (char ***)((longlong)pppcStack_150 + local_100 + -1);
LAB_14068f269:
      if ((local_138 >> ((ulonglong)*(byte *)((longlong)pppcVar10 + local_110) & 0x3f) & 1) == 0) {
        pppcVar10 = (char ***)((longlong)pppcVar26 + (longlong)local_f8);
      }
      else {
        lVar25 = 0;
        pppcVar22 = pppcVar26;
        do {
          if (local_f8 <= (char ***)((longlong)pppcStack_150 + lVar25)) {
            if (local_f8 <= local_170) {
              if (pppcStack_150 != (char ***)0x0) {
                    // WARNING: Subroutine does not return
                ::core::panicking::panic_bounds_check();
              }
              goto joined_r0x00014068f366;
            }
            pppcVar10 = pppcStack_150;
            pppcVar22 = (char ***)((longlong)local_170 + (longlong)pppcVar26);
            pppcVar11 = local_168;
            goto LAB_14068f2d0;
          }
          if (local_108 <= (char ***)((longlong)pppcVar22 + (longlong)pppcStack_150)) {
                    // WARNING: Subroutine does not return
            ::core::panicking::panic_bounds_check();
          }
          lVar17 = lVar25 + local_100;
          pppcVar10 = (char ***)((longlong)pppcVar22 + 1);
          lVar25 = lVar25 + 1;
          pcVar20 = (char *)((longlong)pppcStack_150 + local_110) + (longlong)pppcVar22;
          pppcVar22 = pppcVar10;
        } while (*(char *)((longlong)pppcStack_150 + lVar17) == *pcVar20);
      }
      goto LAB_14068f256;
    }
  }
  else {
    pppcVar10 = (char ***)(local_198 + (longlong)local_130);
    if (pppcVar10 < local_108) {
      local_170 = local_140;
      local_168 = (char ***)((longlong)local_f8 - (longlong)local_140);
      pppcVar22 = local_120;
LAB_14068eff0:
      do {
        if ((local_138 >> ((ulonglong)*(byte *)((longlong)pppcVar10 + local_110) & 0x3f) & 1) == 0)
        {
          pppcVar26 = (char ***)((longlong)pppcVar26 + (longlong)local_f8);
        }
        else {
          pppcVar10 = pppcStack_150;
          if (pppcStack_150 < pppcVar22) {
            pppcVar10 = pppcVar22;
          }
          do {
            pppcVar11 = pppcStack_150;
            if (local_f8 <= pppcVar10) goto LAB_14068f070;
            if (local_108 <= (char ***)((longlong)pppcVar26 + (longlong)pppcVar10)) {
                    // WARNING: Subroutine does not return
              ::core::panicking::panic_bounds_check();
            }
            pppcVar11 = (char ***)((longlong)pppcVar10 + 1);
            pcVar20 = (char *)((longlong)pppcVar10 + local_100);
            pcVar18 = (char *)((longlong)pppcVar26 + local_110) + (longlong)pppcVar10;
            pppcVar10 = pppcVar11;
          } while (*pcVar20 == *pcVar18);
          pppcVar26 = (char ***)
                      ((longlong)pppcVar11 + ((longlong)pppcVar26 - (longlong)pppcStack_150));
        }
        pppcVar10 = (char ***)(local_198 + (longlong)pppcVar26);
        pppcVar22 = (char ***)0x0;
      } while (pppcVar10 < local_108);
    }
  }
LAB_14068f8f4:
  pppcVar10 = local_a0;
  if ((char ***)((longlong)local_e0 - (longlong)local_a0) < _Size) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    local_f0 = pppcStack_d8;
    pppcVar10 = local_d0;
  }
  pppcVar26 = local_f0;
  memcpy((char *)((longlong)local_f0 + (longlong)pppcVar10),local_e8,(size_t)_Size);
  pppcVar10 = (char ***)((longlong)pppcVar10 + (longlong)_Size);
  local_a8 = local_e0;
LAB_14068f618:
  local_e0 = pppcVar26;
  pppcStack_d8 = pppcVar10;
  local_98 = pppcVar26;
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  pppcStack_150 = (char ***)0xdc603d951b9380b2;
  local_148 = (char ***)0xce69dbf0aee13da2;
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  local_158 = pppcStack_180;
  pppcStack_150 = extraout_RAX_04;
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_text<ref_<str_>_>();
  local_b8 = local_148;
  local_c8 = local_158;
  pppcStack_c0 = pppcStack_150;
  if (local_a8 != (char ***)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((int)plVar19[4] == 1) {
    uVar33 = *(undefined4 *)((longlong)plVar19 + 0x24);
    local_148 = local_b8;
    local_158 = local_c8;
    pppcStack_150 = pppcStack_c0;
    core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
    *(undefined4 *)(extraout_RAX_05 + 600) = uVar33;
    local_c8 = local_158;
    pppcStack_c0 = pppcStack_150;
    local_b8 = local_148;
    if ((int)plVar19[5] != 1) goto LAB_14068f6ee;
LAB_14068f82b:
    local_148 = local_b8;
    local_158 = local_c8;
    pppcStack_150 = pppcStack_c0;
    core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
    tg_text::TextFragment::set_metrics<f32>();
    local_b8 = local_148;
    local_c8 = local_158;
    pppcStack_c0 = pppcStack_150;
    if ((int)plVar19[6] != 1) goto LAB_14068f6f9;
LAB_14068f87d:
    local_148 = local_b8;
    local_158 = local_c8;
    pppcStack_150 = pppcStack_c0;
    core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
    tg_text::TextFragment::set_max_width();
    cVar36 = (char)plVar19[7];
  }
  else {
    if ((int)plVar19[5] == 1) goto LAB_14068f82b;
LAB_14068f6ee:
    if ((int)plVar19[6] == 1) goto LAB_14068f87d;
LAB_14068f6f9:
    cVar36 = (char)plVar19[7];
    local_148 = local_b8;
    local_158 = local_c8;
    pppcStack_150 = pppcStack_c0;
  }
  local_c8 = local_158;
  pppcStack_c0 = pppcStack_150;
  local_b8 = local_148;
  if (cVar36 != '\x04') {
    core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
    tg_text::TextFragment::set_family();
    local_b8 = local_148;
    local_c8 = local_158;
    pppcStack_c0 = pppcStack_150;
  }
  local_148 = local_b8;
  local_158 = local_c8;
  pppcStack_150 = pppcStack_c0;
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_align();
  local_b8 = local_148;
  local_c8 = local_158;
  pppcStack_c0 = pppcStack_150;
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_weight();
  plVar19 = local_1a8;
  if (pppcVar30 == (char ***)local_1a8) {
    return;
  }
  goto LAB_14068eefe;
  while( true ) {
    if (local_108 <= pppcVar22) {
                    // WARNING: Subroutine does not return
      ::core::panicking::panic_bounds_check();
    }
    cVar36 = *(char *)pppcVar11;
    pppcVar11 = (char ***)((longlong)pppcVar11 + -1);
    pcVar20 = (char *)((longlong)pppcVar22 + local_110);
    pppcVar22 = (char ***)((longlong)pppcVar22 + -1);
    if (cVar36 != *pcVar20) break;
LAB_14068f2d0:
    bVar35 = pppcVar10 == (char ***)0x0;
    pppcVar10 = (char ***)((longlong)pppcVar10 + -1);
    if (bVar35) goto joined_r0x00014068f366;
  }
  pppcVar10 = (char ***)((longlong)pppcVar26 + (longlong)local_140);
LAB_14068f256:
  pppcVar26 = pppcVar10;
  pppcVar10 = (char ***)(local_198 + (longlong)pppcVar26);
  if (local_108 <= pppcVar10) goto LAB_14068f8f4;
  goto LAB_14068f269;
  while( true ) {
    pppcVar11 = (char ***)((longlong)pppcVar11 + -1);
    if (local_f8 <= pppcVar11) {
                    // WARNING: Subroutine does not return
      ::core::panicking::panic_bounds_check();
    }
    if (local_108 <= (char ***)((longlong)pppcVar11 + (longlong)pppcVar26)) {
                    // WARNING: Subroutine does not return
      ::core::panicking::panic_bounds_check();
    }
    if (*(char *)((longlong)pppcVar11 + local_100) !=
        *(char *)((longlong)pppcVar11 + (longlong)pppcVar26 + local_110)) break;
LAB_14068f070:
    if (pppcVar11 <= pppcVar22) goto joined_r0x00014068f366;
  }
  pppcVar26 = (char ***)((longlong)pppcVar26 + (longlong)local_140);
  pppcVar10 = (char ***)(local_198 + (longlong)pppcVar26);
  pppcVar22 = local_168;
  if (local_108 <= pppcVar10) goto LAB_14068f8f4;
  goto LAB_14068eff0;
LAB_14068f410:
  if (pcVar18 < pcVar31) {
LAB_14068f590:
    local_1f8 = &local_1e8;
    pcStack_1f0 = ::core::fmt::impl$74::fmt<str_>;
    local_158 = (char ***)&PTR_s_Invalid_t__format__142a185a0;
    pppcStack_150 = (char ***)0x1;
    local_148 = &local_1f8;
    local_140 = (char ***)0x1;
    local_138 = 0;
    alloc::fmt::format::format_inner();
    pppcVar10 = local_b8;
    pppcVar26 = pppcStack_c0;
    local_a8 = local_c8;
    if (local_e0 != (char ***)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    goto LAB_14068f618;
  }
  goto LAB_14068f419;
LAB_14068fab0:
  uVar32 = uVar7;
  lVar25 = lVar25 + 0x20;
  if (*(longlong *)(in_R9 + 0x28) << 5 != lVar25) {
    piVar27 = (int *)((longlong)piVar3 + lVar25);
    uVar23 = *(int *)((longlong)piVar3 + lVar25) - 2;
    uVar28 = 7;
    if (uVar23 < 7) {
      uVar28 = uVar23;
    }
    uVar7 = 0;
    switch(uVar28) {
    case 0:
      break;
    case 1:
code_r0x00014068fae7:
      uVar7 = 1;
      break;
    case 2:
code_r0x00014068fb16:
      uVar44 = 0x3f800000;
      uVar7 = uVar32;
      break;
    case 3:
code_r0x00014068fb06:
      uVar14 = *(undefined1 *)((longlong)piVar3 + lVar25 + 4);
      uVar7 = uVar32;
      break;
    case 4:
code_r0x00014068fb0e:
      uVar21 = *(undefined1 *)((longlong)piVar3 + lVar25 + 4);
      uVar7 = uVar32;
      break;
    case 5:
code_r0x00014068faec:
      uVar34 = *(undefined4 *)((longlong)piVar3 + lVar25 + 4);
      uVar45 = *(undefined4 *)((longlong)piVar3 + lVar25 + 8);
      uVar29 = *(undefined4 *)((longlong)piVar3 + lVar25 + 0xc);
      uVar43 = *(undefined4 *)((longlong)piVar3 + lVar25 + 0x10);
      uVar7 = uVar32;
      break;
    case 6:
code_r0x00014068fb20:
      uVar33 = *(undefined4 *)((longlong)piVar3 + lVar25 + 4);
      uVar46 = *(undefined4 *)((longlong)piVar3 + lVar25 + 8);
      uVar7 = uVar32;
      break;
    default:
LAB_14068fb2e:
      local_1d8 = *(char **)piVar27;
      pcStack_1d0 = *(char **)(piVar27 + 2);
      local_1c8 = *(char **)(piVar27 + 4);
      pcStack_1c0 = *(char **)(piVar27 + 6);
      uVar7 = uVar32;
    }
    goto LAB_14068fab0;
  }
  local_98 = (char ***)CONCAT71(local_98._1_7_,uVar14);
LAB_14068ecf5:
  local_158 = local_e0;
  pppcStack_150 = pppcStack_d8;
  local_148 = local_d0;
  if (local_e0[0x37] != (char **)0x0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    ppcVar4 = pppcVar30[0x34];
    ppcVar5 = pppcVar30[0x35];
    bVar8 = (byte)(extraout_RAX_01 >> 0x39);
    auVar37 = ZEXT216(CONCAT11(bVar8,bVar8));
    auVar37 = pshuflw(auVar37,auVar37,0);
    lVar25 = 0;
    uVar24 = extraout_RAX_01;
    while( true ) {
      uVar24 = (ulonglong)ppcVar5 & uVar24;
      pcVar20 = (char *)((longlong)ppcVar4 + uVar24);
      cVar36 = auVar37[0];
      auVar42[0] = -(*pcVar20 == cVar36);
      cVar38 = auVar37[1];
      auVar42[1] = -(pcVar20[1] == cVar38);
      cVar39 = auVar37[2];
      auVar42[2] = -(pcVar20[2] == cVar39);
      cVar40 = auVar37[3];
      auVar42[3] = -(pcVar20[3] == cVar40);
      auVar42[4] = -(pcVar20[4] == cVar36);
      auVar42[5] = -(pcVar20[5] == cVar38);
      auVar42[6] = -(pcVar20[6] == cVar39);
      auVar42[7] = -(pcVar20[7] == cVar40);
      auVar42[8] = -(pcVar20[8] == cVar36);
      auVar42[9] = -(pcVar20[9] == cVar38);
      auVar42[10] = -(pcVar20[10] == cVar39);
      auVar42[0xb] = -(pcVar20[0xb] == cVar40);
      auVar42[0xc] = -(pcVar20[0xc] == cVar36);
      auVar42[0xd] = -(pcVar20[0xd] == cVar38);
      auVar42[0xe] = -(pcVar20[0xe] == cVar39);
      auVar42[0xf] = -(pcVar20[0xf] == cVar40);
      uVar15 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe | (ushort)(auVar42[0xf] >> 7) << 0xf;
      uVar28 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar23 = 0;
        for (uVar6 = uVar28; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar23 = uVar23 + 1;
        }
        uVar16 = uVar23 + uVar24 & (ulonglong)ppcVar5;
        lVar17 = uVar16 * -0xc0;
        if (pppcStack_150 == (char ***)ppcVar4[uVar16 * -0x18 + -0x18]) {
          ppcVar4[uVar16 * -0x18 + -0x12] = local_1c8;
          (ppcVar4 + uVar16 * -0x18 + -0x12)[1] = pcStack_1c0;
          ppcVar4[uVar16 * -0x18 + -0x14] = local_1d8;
          (ppcVar4 + uVar16 * -0x18 + -0x14)[1] = pcStack_1d0;
          *(undefined4 *)(ppcVar4 + uVar16 * -0x18 + -0x10) = uVar33;
          *(undefined4 *)((longlong)ppcVar4 + lVar17 + -0x7c) = uVar46;
          *(undefined4 *)(ppcVar4 + uVar16 * -0x18 + -0xf) = uVar34;
          *(undefined4 *)((longlong)ppcVar4 + lVar17 + -0x74) = uVar45;
          *(undefined4 *)(ppcVar4 + uVar16 * -0x18 + -0xe) = uVar29;
          *(undefined4 *)((longlong)ppcVar4 + lVar17 + -0x6c) = uVar43;
          *(undefined4 *)(ppcVar4 + uVar16 * -0x18 + -0xd) = uVar44;
          *(undefined8 *)((longlong)ppcVar4 + lVar17 + -100) = 0;
          *(undefined1 *)((longlong)ppcVar4 + lVar17 + -0x5c) = uVar32;
          *(undefined1 *)((longlong)ppcVar4 + lVar17 + -0x5b) = uVar21;
          *(undefined1 *)((longlong)ppcVar4 + lVar17 + -0x5a) = local_98._0_1_;
          local_178 = local_148;
          local_188 = local_158;
          pppcStack_180 = pppcStack_150;
          pppcVar30 = *(char ****)(in_R9 + 0x50);
          local_1a8 = (longlong *)(pppcVar30 + *(longlong *)(in_R9 + 0x58) * 0xc);
          pppcVar9 = (char ***)0;
          plVar19 = local_1a8;
          if (pppcVar30 == (char ***)local_1a8) {
            return;
          }
          goto LAB_14068eefe;
        }
        uVar15 = (ushort)(uVar28 - 1) & (ushort)uVar28;
        uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar15);
      }
      auVar41[0] = -(*pcVar20 == -1);
      auVar41[1] = -(pcVar20[1] == -1);
      auVar41[2] = -(pcVar20[2] == -1);
      auVar41[3] = -(pcVar20[3] == -1);
      auVar41[4] = -(pcVar20[4] == -1);
      auVar41[5] = -(pcVar20[5] == -1);
      auVar41[6] = -(pcVar20[6] == -1);
      auVar41[7] = -(pcVar20[7] == -1);
      auVar41[8] = -(pcVar20[8] == -1);
      auVar41[9] = -(pcVar20[9] == -1);
      auVar41[10] = -(pcVar20[10] == -1);
      auVar41[0xb] = -(pcVar20[0xb] == -1);
      auVar41[0xc] = -(pcVar20[0xc] == -1);
      auVar41[0xd] = -(pcVar20[0xd] == -1);
      auVar41[0xe] = -(pcVar20[0xe] == -1);
      auVar41[0xf] = -(pcVar20[0xf] == -1);
      if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar41 >> 0x17,0) & 1) != 0) || (SUB161(auVar41 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar41 >> 0x27,0) & 1) != 0) || (SUB161(auVar41 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar41 >> 0x4f,0) & 1) != 0) || (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar41 >> 0x5f,0) & 1) != 0) || (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0) ||
          auVar41[0xf] < '\0') break;
      uVar24 = uVar24 + lVar25 + 0x10;
      lVar25 = lVar25 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

