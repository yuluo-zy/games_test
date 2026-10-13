// addr=0x140b0eeb0 name=load_json_as_mesh<ref$<str$>_>
// signature=void __cdecl load_json_as_mesh<ref$<str$>_>(void)


// WARNING: Removing unreachable block (ram,0x000140b0f336)
// WARNING: Removing unreachable block (ram,0x000140b0f338)
// WARNING: Removing unreachable block (ram,0x000140b0f53e)

void __cdecl country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  uint *puVar3;
  uint *puVar4;
  undefined8 *puVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  undefined8 uVar13;
  undefined1 *puVar14;
  undefined1 *puVar15;
  code *pcVar16;
  longlong lVar17;
  char extraout_AL;
  bool bVar18;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar19;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  char *extraout_RAX_03;
  uint *extraout_RAX_04;
  longlong lVar20;
  char *extraout_RAX_05;
  char *pcVar21;
  longlong extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  uint uVar22;
  longlong *in_RCX;
  longlong lVar23;
  longlong *plVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  longlong lVar27;
  uint *puVar28;
  longlong lVar29;
  longlong lVar30;
  ulonglong uVar31;
  uint in_R9D;
  ulonglong uVar32;
  uint uVar33;
  uint uVar34;
  uint uVar35;
  uint uVar36;
  uint uVar37;
  uint uVar38;
  uint uVar39;
  uint uVar40;
  uint uVar41;
  uint uVar42;
  int local_170;
  int iStack_c0;
  undefined1 local_b8 [16];
  code *local_a8;
  code *pcStack_a0;
  longlong *local_98;
  longlong lStack_90;
  longlong local_88;
  longlong local_80;
  undefined1 *local_78;
  code *pcStack_70;
  longlong *local_68;
  longlong lStack_60;
  longlong *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    lVar27 = 0;
  }
  else {
    uVar25 = 0x31;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar25 != 0) {
        if (uVar25 < 0x31) {
          if ((char)(&DAT_142ae7ae5)[uVar25] < -0x40) {
LAB_140b0ef36:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 != 0x31) goto LAB_140b0ef36;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar25 != 0xfffffffffffffffe)) {
        if (uVar25 + 2 < 0x31) {
          if ((char)(&UNK_142ae7ae7)[uVar25] < -0x40) {
LAB_140b0ef94:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar25 + 2 != 0x31) goto LAB_140b0ef94;
      }
    }
    puVar19 = &DAT_142ae7f8a;
    in_R9D = 0x42ae7f60;
    do {
      bVar6 = puVar19[-1];
      uVar22 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = puVar19[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar19[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar19 + -4;
            puVar19 = puVar19 + -4;
            uVar22 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar19 = puVar19 + -3;
            uVar22 = bVar8 & 0xf;
          }
          uVar22 = bVar7 & 0x3f | uVar22 << 6;
        }
        else {
          puVar19 = puVar19 + -2;
          uVar22 = bVar7 & 0x1f;
        }
        uVar22 = (uint)(bVar6 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_140b0f059;
LAB_140b0f06f:
        if (puVar19 != &UNK_142ae7f5f) {
          if (puVar19 + -0x142ae7f5f < (undefined *)0x2a) {
            if ((char)puVar19[1] < -0x40) {
LAB_140b0f08e:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (puVar19 + -0x142ae7f5f != (undefined *)0x2a) goto LAB_140b0f08e;
        }
        in_R9D = (int)puVar19 + 1;
        break;
      }
      puVar19 = puVar19 + -1;
      if (uVar22 == 0x5c) goto LAB_140b0f06f;
LAB_140b0f059:
      if (uVar22 == 0x2f) goto LAB_140b0f06f;
    } while (puVar19 != &DAT_142ae7f60);
    std::ffi::os_str::impl$41::try_from();
    if (local_170 == 1) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar24 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
    goto LAB_140b0f847;
    if (*plVar24 != 0) goto LAB_140b0f870;
    *plVar24 = -1;
    local_50 = plVar24;
    puffin::ThreadProfiler::begin_scope();
    *local_50 = *local_50 + 1;
    lVar27 = 1;
  }
  local_88 = lVar27;
  std::fs::read_to_string::inner();
  puVar14 = local_78;
  if (SBORROW8(0,(longlong)local_78)) {
    local_a8 = pcStack_70;
    local_50 = (longlong *)lVar27;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_78 = local_b8;
    pcStack_70 = core::fmt::impl$74::fmt<std::ffi::os_str::OsStr>;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    ::log::__private_api::impl$2::log();
  }
  serde_json::de::from_str<enum2_<serde_json::value::Value>_>();
  if ((char)local_78 == '\x06') {
    local_a8 = pcStack_70;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  local_98 = local_68;
  lStack_90 = lStack_60;
  local_a8 = (code *)local_78;
  pcStack_a0 = pcStack_70;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar11 = *extraout_RAX_02;
  lVar12 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  std::path::Path::to_path_buf();
  lVar17 = lStack_60;
  pcVar16 = pcStack_70;
  puVar15 = local_78;
  serde_json::value::index::impl$1::index_into();
  serde_json::value::index::impl$1::index_into();
  pcVar21 = extraout_RAX_03;
  if (extraout_RAX_03 == (char *)0x0) {
    pcVar21 = 
    __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
  }
  if (*pcVar21 != '\x04') {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar9 = *(longlong *)(pcVar21 + 0x18);
  if (lVar9 == 0) {
    bVar18 = false;
    puVar28 = (uint *)&DAT_00000004;
  }
  else {
    lVar20 = *(longlong *)(pcVar21 + 0x10);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_04 == (uint *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    puVar28 = (uint *)(lVar20 + 0x10);
    lVar20 = 0;
    do {
      if (((char)puVar28[-4] != '\x02') || (*(longlong *)(puVar28 + -2) != 0)) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      extraout_RAX_04[lVar20] = *puVar28;
      lVar20 = lVar20 + 1;
      puVar28 = puVar28 + 8;
    } while (lVar9 != lVar20);
    in_R9D = *extraout_RAX_04;
    bVar18 = true;
    puVar28 = extraout_RAX_04;
    if (lVar9 != 1) {
      uVar25 = lVar9 + 0x3fffffffffffffffU & 0x3fffffffffffffff;
      if (uVar25 < 8) {
        uVar26 = 0;
      }
      else {
        uVar31 = (ulonglong)((uint)(lVar9 + 0x3fffffffffffffffU) & 7);
        uVar26 = uVar25 - uVar31;
        uVar32 = 0;
        uVar22 = in_R9D;
        uVar33 = in_R9D;
        uVar36 = in_R9D;
        uVar35 = in_R9D;
        uVar37 = in_R9D;
        uVar38 = in_R9D;
        uVar34 = in_R9D;
        do {
          puVar3 = extraout_RAX_04 + uVar32 + 1;
          puVar4 = extraout_RAX_04 + uVar32 + 5;
          uVar39 = -(uint)((int)in_R9D < (int)*puVar3);
          uVar40 = -(uint)((int)uVar22 < (int)puVar3[1]);
          uVar41 = -(uint)((int)uVar33 < (int)puVar3[2]);
          uVar42 = -(uint)((int)uVar36 < (int)puVar3[3]);
          in_R9D = ~uVar39 & in_R9D | *puVar3 & uVar39;
          uVar22 = ~uVar40 & uVar22 | puVar3[1] & uVar40;
          uVar33 = ~uVar41 & uVar33 | puVar3[2] & uVar41;
          uVar36 = ~uVar42 & uVar36 | puVar3[3] & uVar42;
          uVar39 = -(uint)((int)uVar35 < (int)*puVar4);
          uVar40 = -(uint)((int)uVar37 < (int)puVar4[1]);
          uVar41 = -(uint)((int)uVar38 < (int)puVar4[2]);
          uVar42 = -(uint)((int)uVar34 < (int)puVar4[3]);
          uVar35 = ~uVar39 & uVar35 | *puVar4 & uVar39;
          uVar37 = ~uVar40 & uVar37 | puVar4[1] & uVar40;
          uVar38 = ~uVar41 & uVar38 | puVar4[2] & uVar41;
          uVar34 = ~uVar42 & uVar34 | puVar4[3] & uVar42;
          uVar32 = uVar32 + 8;
        } while (uVar26 != uVar32);
        uVar35 = ~-(uint)((int)uVar35 < (int)in_R9D) & uVar35 |
                 in_R9D & -(uint)((int)uVar35 < (int)in_R9D);
        uVar37 = ~-(uint)((int)uVar37 < (int)uVar22) & uVar37 |
                 uVar22 & -(uint)((int)uVar37 < (int)uVar22);
        uVar38 = ~-(uint)((int)uVar38 < (int)uVar33) & uVar38 |
                 uVar33 & -(uint)((int)uVar38 < (int)uVar33);
        uVar36 = ~-(uint)((int)uVar34 < (int)uVar36) & uVar34 |
                 uVar36 & -(uint)((int)uVar34 < (int)uVar36);
        uVar22 = -(uint)((int)uVar38 < (int)uVar35);
        uVar33 = -(uint)((int)uVar36 < (int)uVar37);
        uVar22 = ~uVar22 & uVar38 | uVar35 & uVar22;
        uVar33 = ~uVar33 & uVar36 | uVar37 & uVar33;
        uVar36 = -(uint)((int)uVar33 < (int)uVar22);
        in_R9D = ~uVar36 & uVar33 | uVar22 & uVar36;
        if (uVar31 == 0) goto LAB_140b0f524;
      }
      do {
        if ((int)in_R9D < (int)extraout_RAX_04[uVar26 + 1]) {
          in_R9D = extraout_RAX_04[uVar26 + 1];
        }
        uVar26 = uVar26 + 1;
      } while (uVar25 != uVar26);
    }
  }
LAB_140b0f524:
  uVar22 = 0xffffffff;
  if (bVar18) {
    uVar22 = in_R9D;
  }
  local_50 = (longlong *)CONCAT44(local_50._4_4_,uVar22);
  serde_json::value::index::impl$1::index_into();
  pcVar21 = extraout_RAX_05;
  if (extraout_RAX_05 == (char *)0x0) {
    pcVar21 = 
    __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
  }
  if (*pcVar21 != '\x04') {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar20 = *(longlong *)(pcVar21 + 0x18);
  if (lVar20 == 0) {
    lVar29 = 8;
  }
  else {
    lVar10 = *(longlong *)(pcVar21 + 0x10);
    lVar30 = 8;
    std::alloc::__default_lib_allocator::__rust_alloc();
    lVar23 = lVar20;
    if (extraout_RAX_06 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    do {
      if (*(char *)(lVar10 + -0x10 + lVar30 * 2) != '\x03') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      puVar2 = (undefined8 *)(lVar10 + lVar30 * 2);
      uVar13 = puVar2[1];
      puVar5 = (undefined8 *)(extraout_RAX_06 + -8 + lVar30);
      *puVar5 = *puVar2;
      puVar5[1] = uVar13;
      lVar30 = lVar30 + 0x10;
      lVar23 = lVar23 + -1;
      lVar29 = extraout_RAX_06;
    } while (lVar23 != 0);
  }
  lVar10 = lVar29 + lVar20 * 0x10;
  local_80 = lVar29;
  for (; lVar29 != lVar10; lVar29 = lVar29 + 0x10) {
    read_attribute_from_json();
    if ((int)local_78 == 5) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    iStack_c0 = (int)lStack_60;
    if (iStack_c0 <= (int)local_50) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    country_core::render::mesh::Mesh::
    set_attribute<enum2_<country_core::render::mesh::VertexAttributeValues>_>();
  }
  if (lVar20 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  in_RCX[0xe] = 0x11;
  in_RCX[0xc] = lVar12;
  in_RCX[0xd] = (longlong)&DAT_142ae7ffe;
  in_RCX[10] = 0;
  in_RCX[0xb] = lVar11;
  in_RCX[8] = 0;
  in_RCX[9] = 0;
  in_RCX[6] = lVar17;
  in_RCX[7] = (longlong)&anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
  in_RCX[4] = (longlong)pcVar16;
  in_RCX[5] = (longlong)local_68;
  in_RCX[2] = lVar9;
  in_RCX[3] = (longlong)puVar15;
  *in_RCX = lVar9;
  in_RCX[1] = (longlong)puVar28;
  if (2 < (byte)local_a8) {
    if ((byte)local_a8 != 3) {
      if ((byte)local_a8 != 4) {
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
        goto LAB_140b0f79b;
      }
      local_50 = local_98;
      core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
    }
    if (pcStack_a0 != (code *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
LAB_140b0f79b:
  if (puVar14 != (undefined1 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (lVar27 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_07 == 1) {
      plVar24 = extraout_RAX_07 + 1;
    }
    else if ((*extraout_RAX_07 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_140b0f847:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar24 != 0) {
LAB_140b0f870:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar24 = -1;
    local_50 = plVar24;
    puffin::ThreadProfiler::end_scope();
    *local_50 = *local_50 + 1;
  }
  return;
}


