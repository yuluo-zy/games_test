
// WARNING: Instruction at (ram,0x0001417a7f77) overlaps instruction at (ram,0x0001417a7f75)
// 
// WARNING: Control flow encountered bad instruction data

void __cdecl country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data(void)

{
  int *piVar1;
  char *pcVar2;
  undefined1 auVar3 [16];
  undefined4 uVar4;
  undefined8 uVar5;
  HANDLE pvVar6;
  undefined8 *puVar7;
  code *pcVar8;
  longlong lVar9;
  undefined1 (*pauVar10) [16];
  undefined4 *puVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  char extraout_AL;
  char cVar16;
  byte bVar17;
  uint uVar19;
  undefined8 extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  longlong extraout_RAX_02;
  byte *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 extraout_RAX_06;
  byte *extraout_RAX_07;
  byte *extraout_RAX_08;
  byte *extraout_RAX_09;
  byte *extraout_RAX_10;
  byte *extraout_RAX_11;
  byte *extraout_RAX_12;
  byte *extraout_RAX_13;
  byte *extraout_RAX_14;
  byte *extraout_RAX_15;
  byte *pbVar20;
  byte *extraout_RAX_16;
  byte *extraout_RAX_17;
  byte *extraout_RAX_18;
  byte *extraout_RAX_19;
  byte *extraout_RAX_20;
  byte *extraout_RAX_21;
  int *piVar21;
  undefined8 extraout_RAX_22;
  byte *extraout_RAX_23;
  byte *extraout_RAX_24;
  char cVar22;
  longlong in_RCX;
  longlong lVar23;
  undefined *puVar24;
  int *piVar25;
  longlong in_RDX;
  undefined8 uVar26;
  undefined *puVar27;
  int *piVar28;
  int iVar29;
  longlong *plVar31;
  undefined8 *puVar32;
  undefined8 *puVar33;
  longlong *plVar34;
  longlong *plVar35;
  undefined1 *puVar36;
  undefined8 *puVar38;
  undefined8 *puVar39;
  undefined1 *puVar40;
  int *in_R9;
  undefined8 in_R11;
  longlong lVar41;
  byte *pbVar42;
  undefined8 extraout_XMM0_Qa;
  undefined8 extraout_XMM0_Qa_00;
  undefined8 extraout_XMM0_Qa_01;
  undefined8 extraout_XMM0_Qa_02;
  undefined8 extraout_XMM0_Qa_03;
  undefined8 extraout_XMM0_Qb;
  undefined8 extraout_XMM0_Qb_00;
  undefined8 uVar43;
  undefined8 extraout_XMM0_Qb_01;
  undefined8 extraout_XMM0_Qb_02;
  undefined8 extraout_XMM0_Qb_03;
  undefined4 uVar44;
  undefined4 uVar45;
  longlong in_stack_00000028;
  undefined1 auStack_658 [128];
  undefined8 local_5d8 [70];
  longlong local_3a8;
  undefined8 local_3a0;
  undefined1 local_f8 [152];
  undefined8 local_60;
  int iVar18;
  char cVar30;
  undefined8 *puVar37;
  
  puVar36 = auStack_658;
  puVar39 = local_5d8;
  local_60 = 0xfffffffffffffffe;
  local_3a0 = *(undefined8 *)(in_stack_00000028 + 0x120);
  local_3a8 = in_RDX + 0x18;
  puVar40 = local_f8;
LAB_1417a6eea:
  *(undefined8 *)(puVar36 + -8) = 0x1417a6efd;
  resources::history::History::closest_resolved_snapshot();
  if ((*(byte *)(puVar39 + 0x1f) & 1) == 0) {
    *(undefined8 *)(in_RCX + 8) = 2;
                    // WARNING: Read-only address (ram,0x000142cdca80) is written
                    // WARNING: Read-only address (ram,0x000142cdcaba) is written
    return;
  }
  uVar4 = *(undefined4 *)(puVar39 + 0x21);
  uVar5 = puVar39[0x20];
  puVar39[0x40] = uVar5;
  puVar39[0x86] = puVar39 + 0x40;
  puVar39[0x87] = core::fmt::num::imp::impl$9::fmt;
  puVar39[0x88] = &anon_7bf6ccd2d38324b0d8e2d9b7c7cb0a3a_66_llvm_11027640352236353829;
  puVar39[0x89] = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  puVar39[0x9c] = &anon_7bf6ccd2d38324b0d8e2d9b7c7cb0a3a_68_llvm_11027640352236353829;
  puVar39[0x9d] = 2;
  puVar39[0x9e] = puVar39 + 0x86;
  puVar39[0x9f] = 2;
  puVar39[0xa0] = 0;
  *(undefined8 *)(puVar36 + -8) = 0x1417a6fa3;
  alloc::fmt::format::format_inner();
  uVar26 = puVar39[0x7c];
  *(int **)(puVar36 + 0x20) = in_R9;
  puVar39[0xad] = uVar26;
  *(undefined8 *)(puVar36 + -8) = 0x1417a6fd0;
  relative_path::RelativePath::to_path<ref_<std::path::Path>_>();
  if (puVar39[0x7b] != 0) {
    *(undefined8 *)(puVar36 + -8) = 0x1417a6fee;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a7001;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  *(undefined4 *)(puVar39 + 0x9c) = 0;
  puVar39[0x9d] = 0;
  puVar39[0x9e] = 0;
  *(undefined4 *)(puVar39 + 0x9f) = 7;
  *(undefined2 *)((longlong)puVar39 + 0x504) = 0;
  *(undefined8 *)((longlong)puVar39 + 0x4fc) = 0;
  *(undefined1 *)(puVar39 + 0xa0) = 1;
  *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a7049;
  uVar26 = extraout_RAX;
  std::fs::OpenOptions::_open();
  if (extraout_RAX_00 != 0) {
    *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a705d;
    anyhow::error::impl$1::from<std::io::error::Error>();
    pbVar42 = extraout_RAX_01;
    goto LAB_1417a70cf;
  }
  puVar39[0xad] = uVar26;
  *(undefined8 *)(puVar36 + -8) = 0x1417a706f;
  memmap2::os::file_len();
  if (extraout_RAX_02 != 0) {
LAB_1417a70b0:
    pvVar6 = (HANDLE)puVar39[0xad];
    *(undefined8 *)(puVar36 + -8) = 0x1417a70bd;
    CloseHandle(pvVar6);
    *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a70cc;
    anyhow::error::impl$1::from<std::io::error::Error>();
    pbVar42 = extraout_RAX_03;
    goto LAB_1417a70cf;
  }
  puVar36[0x20] = 0;
  *(undefined8 *)(puVar36 + -8) = 0x1417a7091;
  memmap2::os::MmapInner::map();
  lVar23 = puVar39[0x9c];
  pbVar42 = (byte *)puVar39[0x9d];
  if (lVar23 == 2) {
    in_R9 = (int *)puVar39[0x99];
    goto LAB_1417a70b0;
  }
  puVar39[0x88] = puVar39[0xa0];
  puVar39[0x86] = puVar39[0x9e];
  puVar39[0x87] = puVar39[0x9f];
  pvVar6 = (HANDLE)puVar39[0xad];
  *(undefined8 *)(puVar36 + -8) = 0x1417a7425;
  CloseHandle(pvVar6);
  puVar39[0x3f] = puVar39[0x88];
  puVar39[0x3d] = puVar39[0x86];
  puVar39[0x3e] = puVar39[0x87];
  puVar39[0x3b] = lVar23;
  puVar39[0x3c] = pbVar42;
  uVar26 = puVar39[0x3e];
  *(undefined8 *)(puVar36 + -8) = 0x1417a7464;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  in_R9 = (int *)puVar39[0x99];
  puVar39[0x39] = extraout_RAX_06;
  puVar39[0x3a] = uVar26;
  *(undefined8 *)(puVar36 + -8) = 0x1417a748c;
  motomoto::Reader::iter();
  *(undefined8 *)(puVar36 + -8) = 0x1417a749f;
  motomoto::impl$9::next();
  puVar39[0xad] = puVar39[0x86];
  if (puVar39[0x86] != 1) {
    puVar39[0x9c] = &anon_2d23a5d940a4c0c0fd6ed925c1c0dfda_51_llvm_9804287465812819959;
    puVar39[0x9d] = 1;
    puVar39[0x9e] = 8;
    puVar39[0x9f] = 0;
    puVar39[0xa0] = 0;
    *(undefined8 *)(puVar36 + -8) = 0x1417a75a7;
    anyhow::__private::format_err();
    pbVar42 = extraout_RAX_08;
    goto LAB_1417a75b4;
  }
  if (puVar39[0x87] == 0) {
    *(undefined8 *)(puVar36 + -8) = 0x1417a75b1;
    anyhow::error::impl$1::from<std::io::error::Error>();
    pbVar42 = extraout_RAX_09;
    goto LAB_1417a75b4;
  }
  *(undefined8 *)(puVar36 + -8) = 0x1417a74da;
  serde_json::de::from_slice<enum2_<serde_json::value::Value>_>();
  if (*(char *)(puVar39 + 0x7b) == '\x06') {
    uVar26 = puVar39[0x7c];
    puVar39[0x72] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x567) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a7506;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "Deserializing history snapshotparsing Versionparsing Wallsparsing Roofsparsing Decoratorsparsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0x1e;
    puVar39[0x9e] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x567) = 0;
    *(undefined8 *)(puVar36 + -8) = 0x1417a7568;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    in_R9 = (int *)puVar39[0x99];
    pbVar42 = extraout_RAX_07;
    goto LAB_1417a75b4;
  }
  *(undefined4 *)((longlong)puVar39 + 0x4a4) = *(undefined4 *)((longlong)puVar39 + 0x3dc);
  *(undefined4 *)((longlong)puVar39 + 0x4a1) = *(undefined4 *)((longlong)puVar39 + 0x3d9);
  puVar39[0x96] = puVar39[0x7d];
  puVar39[0x97] = puVar39[0x7e];
  *(char *)(puVar39 + 0x94) = *(char *)(puVar39 + 0x7b);
  puVar39[0x95] = puVar39[0x7c];
  *(undefined8 *)(puVar36 + -8) = 0x1417a7620;
  serde_json::value::index::impl$1::index_into();
  pbVar20 = extraout_RAX_10;
  if (extraout_RAX_10 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
  puVar27 = &DAT_142cdca80;
  lVar23 = (longlong)*(int *)(&DAT_142cdca80 + (ulonglong)*pbVar20 * 4);
  puVar24 = &DAT_142cdca80 + lVar23;
  cVar16 = (char)pbVar20;
  iVar18 = (int)pbVar20;
  cVar30 = (char)((ulonglong)in_RCX >> 8);
  iVar29 = (int)in_RCX;
  cVar22 = (char)puVar24;
  uVar26 = extraout_XMM0_Qa;
  uVar43 = extraout_XMM0_Qb;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
    *(byte *)((longlong)puVar39 + 0x4e1) = pbVar20[1];
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
    uVar26 = *(undefined8 *)(pbVar20 + 0x10);
    puVar39[0x9d] = *(undefined8 *)(pbVar20 + 8);
    puVar39[0x9e] = uVar26;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
    *(undefined **)(puVar36 + -8) = &UNK_1417a769d;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7665;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    if (*(longlong *)(pbVar20 + 0x18) == 0) {
      puVar39[0x9d] = 0;
      puVar39[0x9f] = 0;
    }
    else {
      if (*(longlong *)(pbVar20 + 8) == 0) {
                    // WARNING: Subroutine does not return
        *(undefined **)(puVar36 + -8) = &UNK_1417a8fd3;
        core::option::unwrap_failed();
      }
      *(undefined **)(puVar36 + -8) = &UNK_1417a76dd;
      alloc::collections::btree::map::impl$2::clone::
      clone_subtree<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    }
    *(undefined1 *)(puVar39 + 0x9c) = 5;
    break;
  case 6:
    pbVar20 = (byte *)(ulonglong)(iVar18 + 0x1e88f6d);
    goto code_r0x0001417a77eb;
  case 7:
    goto code_r0x0001417a795f;
  case 8:
    goto code_r0x0001417a792a;
  case 9:
    goto code_r0x0001417a7947;
  case 10:
    out(0xfe,cVar16);
    goto code_r0x0001417a7901;
  case 0xb:
    *pbVar20 = *pbVar20 + cVar16;
    puVar39 = (undefined8 *)(ulonglong)(uint)((int)puVar39 + iVar29);
    plVar31 = (longlong *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *plVar31 = (longlong)puVar40;
    goto code_r0x0001417a7977;
  case 0xc:
    pbVar20 = (byte *)(ulonglong)(iVar18 + 0xb7003336);
    in_R9 = (int *)(ulonglong)*(uint *)(puVar39 + 0x9b);
    goto code_r0x0001417a7aaa;
  case 0xd:
    goto code_r0x0001417a7afd;
  case 0xe:
    if (puVar24 == (undefined *)0x0) goto code_r0x0001417a7a90;
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7b5a;
  case 0xf:
    goto code_r0x0001417a7ae3;
  case 0x10:
    puVar39 = (undefined8 *)0x4d8;
    goto code_r0x0001417a7aaa;
  case 0x11:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x12:
    (&UNK_142cdca35)[(longlong)puVar24 * 4 + lVar23] =
         (&UNK_142cdca35)[(longlong)puVar24 * 4 + lVar23] + cVar22;
    puVar33 = (undefined8 *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *puVar33 = 0x4c000003;
    puVar40 = (undefined1 *)(ulonglong)*(uint *)(puVar39 + 0x7d);
    goto code_r0x0001417a7c18;
  case 0x13:
    goto code_r0x0001417a7c70;
  case 0x14:
    pbVar20 = (byte *)(ulonglong)
                      (uint)
                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
    goto code_r0x0001417a7c3c;
  case 0x15:
    goto code_r0x0001417a7c4f;
  case 0x16:
    goto code_r0x0001417a7c13;
  case 0x17:
    goto code_r0x0001417a7c7e;
  case 0x18:
    *(int *)(puVar39 + 0x57) = iVar18;
    goto code_r0x0001417a7dc0;
  case 0x19:
    *pbVar20 = *pbVar20 + cVar16;
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7e26;
  case 0x1a:
    goto code_r0x0001417a7de1;
  case 0x1b:
    goto code_r0x0001417a7e02;
  case 0x1c:
    pbVar20[-0x77] = pbVar20[-0x77] + cVar22;
    puVar39 = (undefined8 *)&DAT_000002c0;
    goto code_r0x0001417a7dce;
  case 0x1d:
    goto code_r0x0001417a7e31;
  case 0x1e:
                    // WARNING (jumptable): Read-only address (ram,0x000142cdca80) is written
                    // WARNING: Read-only address (ram,0x000142cdca80) is written
    DAT_142cdca80 = 0x3f;
    *pbVar20 = *pbVar20 + cVar16;
    (&UNK_142cdca35)[(longlong)puVar24 * 4 + lVar23] =
         (&UNK_142cdca35)[(longlong)puVar24 * 4 + lVar23] + cVar22;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1f:
    *in_R9 = *in_R9 + (int)puVar24;
    goto code_r0x0001417a7f94;
  case 0x20:
    puVar37 = (undefined8 *)(puVar36 + -8);
    *(undefined8 **)(puVar36 + -8) = puVar39;
    puVar36 = puVar36 + -0xb;
    puVar39 = puVar37;
    goto code_r0x0001417a7f62;
  case 0x21:
    goto code_r0x0001417a7f77;
  case 0x22:
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7f3d;
  case 0x23:
    goto code_r0x0001417a7fa6;
  }
  *(undefined **)(puVar36 + -8) = &UNK_1417a770b;
  serde_json::value::de::impl$2::
  deserialize_i32<serde::de::impls::impl_54::deserialize::PrimitiveVisitor>();
  if (*(int *)(puVar39 + 0x7b) == 1) {
    uVar26 = puVar39[0x7c];
    puVar39[0x72] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x566) = 1;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7735;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "parsing Versionparsing Wallsparsing Roofsparsing Decoratorsparsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0xf;
    puVar39[0x9e] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x566) = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7797;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    pbVar20 = extraout_RAX_11;
    goto code_r0x0001417a779a;
  }
  *(int *)((longlong)puVar39 + 0x364) = *(int *)((longlong)puVar39 + 0x3dc);
  if (*(int *)((longlong)puVar39 + 0x3dc) != 2) {
    puVar39[0x86] = &UNK_142cdc578;
    puVar39[0x87] = core::fmt::num::imp::impl$7::fmt;
    puVar39[0x88] = (undefined1 *)((longlong)puVar39 + 0x364);
    puVar39[0x89] = core::fmt::num::imp::impl$7::fmt;
    puVar39[0x9c] = &UNK_142cdc840;
    puVar39[0x9d] = 2;
    puVar39[0x9e] = puVar39 + 0x86;
    puVar39[0x9f] = 2;
    puVar39[0xa0] = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7884;
    alloc::fmt::format::format_inner();
    *(undefined **)(puVar36 + -8) = &UNK_1417a788d;
    anyhow::Error::msg<alloc::string::String>();
    bVar17 = *(byte *)(puVar39 + 0x94);
    pbVar42 = extraout_RAX_13;
    goto joined_r0x0001417a789a;
  }
  *(undefined **)(puVar36 + -8) = &UNK_1417a77df;
  serde_json::value::index::impl$1::index_into();
  pbVar20 = extraout_RAX_12;
  uVar26 = extraout_XMM0_Qa_00;
  uVar43 = extraout_XMM0_Qb_00;
  if (extraout_RAX_12 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
code_r0x0001417a77eb:
  puVar27 = &UNK_142cdca98;
  puVar24 = &UNK_142cdca98 + *(int *)(&UNK_142cdca98 + (ulonglong)*pbVar20 * 4);
  cVar16 = (char)pbVar20;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
code_r0x0001417a7977:
    *(byte *)((longlong)puVar39 + 0x4e1) = pbVar20[1];
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
    uVar26 = *(undefined8 *)(pbVar20 + 8);
    uVar43 = *(undefined8 *)(pbVar20 + 0x10);
code_r0x0001417a7947:
    puVar39[0x9d] = uVar26;
    puVar39[0x9e] = uVar43;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
code_r0x0001417a795f:
    *(undefined **)(puVar36 + -8) = &UNK_1417a796e;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
code_r0x0001417a792a:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7936;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    if (*(longlong *)(pbVar20 + 0x18) == 0) {
      puVar39[0x9d] = 0;
      puVar39[0x9f] = 0;
    }
    else {
      if (*(longlong *)(pbVar20 + 8) == 0) {
                    // WARNING: Subroutine does not return
        *(undefined **)(puVar36 + -8) = &UNK_1417a8fe1;
        core::option::unwrap_failed();
      }
      *(undefined **)(puVar36 + -8) = &UNK_1417a79ae;
      alloc::collections::btree::map::impl$2::clone::
      clone_subtree<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    }
    *(undefined1 *)(puVar39 + 0x9c) = 5;
    break;
  case 6:
    pbVar20 = (byte *)(ulonglong)((int)pbVar20 + 0x1e88ca2);
    goto code_r0x0001417a7ab6;
  case 7:
    goto code_r0x0001417a7b15;
  case 8:
    goto code_r0x0001417a7ae0;
  case 9:
    goto code_r0x0001417a7afd;
  case 10:
    puVar32 = (undefined8 *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *puVar32 = in_R11;
    *(int *)(pbVar20 + 99) = *(int *)(pbVar20 + 99) + (int)puVar24;
    pbVar20 = (byte *)((ulonglong)pbVar20 | 0x8a);
    goto code_r0x0001417a7ab6;
  case 0xb:
    *pbVar20 = *pbVar20 + cVar16;
    puVar39 = (undefined8 *)(ulonglong)(uint)((int)puVar39 + iVar29);
    bVar17 = cVar16 - 0xf;
    goto code_r0x0001417a7b31;
  case 0xc:
    goto code_r0x0001417a7c13;
  case 0xd:
    goto code_r0x0001417a7c83;
  case 0xe:
    if (puVar24 == (undefined *)0x0) {
      *pbVar20 = *pbVar20 + cVar16;
      goto code_r0x0001417a7c18;
    }
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7c4f;
  case 0xf:
    goto code_r0x0001417a7c66;
  case 0x10:
    goto code_r0x0001417a7c2b;
  case 0x11:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x12:
    pbVar20[-0x77] = pbVar20[-0x77] + (char)puVar24;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x13:
    goto code_r0x0001417a7e31;
  case 0x14:
    pbVar20 = (byte *)(ulonglong)
                      (uint)
                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
    goto code_r0x0001417a7e07;
  case 0x15:
    goto code_r0x0001417a7e1a;
  case 0x16:
    *pbVar20 = *pbVar20 + cVar16;
    pbVar20[-0x73] = pbVar20[-0x73] + (char)puVar24;
    goto code_r0x0001417a7de1;
  case 0x17:
    goto code_r0x0001417a7e49;
  case 0x18:
    goto code_r0x0001417a7f47;
  case 0x19:
    *pbVar20 = *pbVar20 + cVar16;
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7fb3;
  case 0x1a:
    goto code_r0x0001417a7f83;
  case 0x1b:
    goto code_r0x0001417a7f8f;
  case 0x1c:
    goto code_r0x0001417a7f53;
  case 0x1d:
    goto code_r0x0001417a7fbe;
  }
  *(undefined **)(puVar36 + -8) = &UNK_1417a79dc;
  serde_json::value::de::impl$2::
  deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>_>_>
            ();
  uVar26 = puVar39[0x7c];
  if (SBORROW8(0,puVar39[0x7b])) {
    puVar39[0x72] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x565) = 1;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7a08;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "parsing Wallsparsing Roofsparsing Decoratorsparsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0xd;
    puVar39[0x9e] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x565) = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7a6a;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    pbVar20 = extraout_RAX_14;
    goto code_r0x0001417a779a;
  }
  puVar39[0x5a] = puVar39[0x7b];
  puVar39[0x5b] = uVar26;
  puVar39[0x5c] = puVar39[0x7d];
code_r0x0001417a7a90:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7aa3;
  serde_json::value::index::impl$1::index_into();
  in_R9 = (int *)puVar39[0x9b];
  pbVar20 = extraout_RAX_15;
  uVar26 = extraout_XMM0_Qa_01;
  uVar43 = extraout_XMM0_Qb_01;
code_r0x0001417a7aaa:
  if (pbVar20 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
code_r0x0001417a7ab6:
  puVar24 = &UNK_142cdcab0 + *(int *)(&UNK_142cdcab0 + (ulonglong)*pbVar20 * 4);
  cVar16 = (char)pbVar20;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
    bVar17 = pbVar20[1];
code_r0x0001417a7b31:
    *(byte *)((longlong)puVar39 + 0x4e1) = bVar17;
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
    uVar26 = *(undefined8 *)(pbVar20 + 8);
    uVar43 = *(undefined8 *)(pbVar20 + 0x10);
code_r0x0001417a7afd:
    puVar39[0x9d] = uVar26;
    puVar39[0x9e] = uVar43;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
code_r0x0001417a7b15:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7b24;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
code_r0x0001417a7ae0:
code_r0x0001417a7ae3:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7aef;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7b53;
    alloc::collections::btree::map::impl$2::
    clone<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 5;
    break;
  case 6:
    pbVar20 = (byte *)(ulonglong)((int)pbVar20 + 0x1e88b1c);
    goto code_r0x0001417a7c3c;
  case 7:
    goto code_r0x0001417a7c9b;
  case 8:
    goto code_r0x0001417a7c66;
  case 9:
    goto code_r0x0001417a7c83;
  case 10:
    plVar34 = (longlong *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *plVar34 = in_RCX;
    *(int *)(pbVar20 + 99) = *(int *)(pbVar20 + 99) + (int)puVar24;
    pbVar20 = (byte *)((ulonglong)pbVar20 | 0x8a);
    goto code_r0x0001417a7c3c;
  case 0xb:
    *pbVar20 = *pbVar20 + cVar16;
    puVar39 = (undefined8 *)(ulonglong)(uint)((int)puVar39 + iVar29);
    bVar17 = cVar16 - 0xf;
    goto code_r0x0001417a7cb7;
  case 0xc:
    pcVar8 = (code *)swi(3);
    (*pcVar8)();
    return;
  case 0xd:
    goto code_r0x0001417a7e4e;
  case 0xe:
    if (puVar24 == (undefined *)0x0) goto code_r0x0001417a7de1;
    *pbVar20 = *pbVar20 + cVar16;
    goto code_r0x0001417a7eab;
  case 0xf:
    goto code_r0x0001417a7e31;
  case 0x10:
    puVar39 = (undefined8 *)0x4d8;
    goto code_r0x0001417a7de1;
  case 0x11:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x12:
    uVar19 = (int)pbVar20 + *(int *)pbVar20;
    pcVar2 = &UNK_142cdca65 +
             (longlong)puVar24 * 4 + (longlong)*(int *)(&UNK_142cdcab0 + (ulonglong)*pbVar20 * 4);
    cVar16 = *pcVar2;
    *pcVar2 = *pcVar2 + (char)puVar24;
    if (SCARRY1(cVar16,(char)puVar24)) {
      puVar40 = (undefined1 *)(ulonglong)*(uint *)(puVar39 + 0x7d);
      goto code_r0x0001417a7f83;
    }
    *(char *)(ulonglong)uVar19 = *(char *)(ulonglong)uVar19 + (char)uVar19;
    goto code_r0x0001417a7f62;
  case 0x13:
    goto code_r0x0001417a7fbe;
  case 0x14:
    pbVar20 = (byte *)(ulonglong)
                      (uint)
                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
    goto code_r0x0001417a7f94;
  case 0x15:
    goto code_r0x0001417a7fa7;
  case 0x16:
                    // WARNING: Read-only address (ram,0x000142cdcaba) is written
    UNK_142cdcaba = cVar30 + -0x54;
    goto code_r0x0001417a7f83;
  case 0x17:
    goto code_r0x0001417a7fd6;
  }
code_r0x0001417a7b5a:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7b69;
  serde_json::value::de::impl$2::
  deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<country_core::resources::roofs::roof_shape::Roof>_>
            ();
  uVar26 = puVar39[0x7c];
  if (SBORROW8(0,puVar39[0x7b])) {
    puVar39[0x72] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x564) = 1;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7b99;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "parsing Roofsparsing Decoratorsparsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0xd;
    puVar39[0x9e] = uVar26;
    *(undefined1 *)((longlong)puVar39 + 0x564) = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7bfb;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    pbVar20 = extraout_RAX_16;
    goto code_r0x0001417a8aed;
  }
  puVar39[0x78] = puVar39[0x7b];
  puVar39[0x6d] = uVar26;
  pbVar42 = (byte *)puVar39[0x7d];
code_r0x0001417a7c18:
  puVar27 = (undefined *)0xa;
code_r0x0001417a7c13:
  puVar27[10] = puVar27[10] + cVar30;
code_r0x0001417a7c2b:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7c30;
  serde_json::value::index::impl$1::index_into();
  pbVar20 = extraout_RAX_17;
  uVar26 = extraout_XMM0_Qa_02;
  uVar43 = extraout_XMM0_Qb_02;
  if (extraout_RAX_17 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
code_r0x0001417a7c3c:
  puVar24 = &UNK_142cdcac8 + *(int *)(&UNK_142cdcac8 + (ulonglong)*pbVar20 * 4);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
code_r0x0001417a7c4f:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
    bVar17 = pbVar20[1];
code_r0x0001417a7cb7:
    *(byte *)((longlong)puVar39 + 0x4e1) = bVar17;
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
code_r0x0001417a7c7e:
    uVar26 = *(undefined8 *)(pbVar20 + 8);
    uVar43 = *(undefined8 *)(pbVar20 + 0x10);
code_r0x0001417a7c83:
    puVar39[0x9d] = uVar26;
    puVar39[0x9e] = uVar43;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
code_r0x0001417a7c9b:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7caa;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
code_r0x0001417a7c66:
code_r0x0001417a7c70:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7c75;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7cd9;
    alloc::collections::btree::map::impl$2::
    clone<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 5;
    break;
  case 6:
code_r0x0001417a7e02:
    pbVar20 = (byte *)(ulonglong)((int)pbVar20 + 0x1e88951);
    goto code_r0x0001417a7e07;
  case 7:
    goto code_r0x0001417a7e66;
  case 8:
    goto code_r0x0001417a7e31;
  case 9:
    goto code_r0x0001417a7e4e;
  case 10:
    plVar35 = (longlong *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *plVar35 = in_RCX;
    *(int *)(pbVar20 + 99) = *(int *)(pbVar20 + 99) + (int)puVar24;
    pbVar20 = (byte *)((ulonglong)pbVar20 | 0x8a);
    goto code_r0x0001417a7e07;
  case 0xb:
    *pbVar20 = *pbVar20 + (char)pbVar20;
    puVar39 = (undefined8 *)(ulonglong)(uint)((int)puVar39 + iVar29);
    bVar17 = (char)pbVar20 - 0xf;
    goto code_r0x0001417a7e82;
  case 0xc:
code_r0x0001417a7f77:
    goto code_r0x0001417a7f83;
  case 0xd:
    goto code_r0x0001417a7fdb;
  case 0xe:
code_r0x0001417a7fa6:
    if (puVar24 == (undefined *)0x0) {
      *pbVar20 = *pbVar20 + (char)pbVar20;
      goto code_r0x0001417a7f83;
    }
    *pbVar20 = *pbVar20 + (char)pbVar20;
    goto code_r0x0001417a8038;
  case 0xf:
    goto code_r0x0001417a7fbe;
  case 0x10:
    goto code_r0x0001417a7f83;
  case 0x11:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  *(undefined **)(puVar36 + 0x20) =
       &anon_36890fc008bdb4b97258d46f0ec99221_135_llvm_10723592867021123337;
  *(undefined8 *)(puVar36 + 0x28) = 1;
  *(undefined **)(puVar36 + -8) = &UNK_1417a7d11;
  serde_json::value::de::impl$2::
  deserialize_struct<country_core::resources::walls::decorator_storage::_::impl_0::deserialize::__Visitor>
            ();
  puVar39[0x9b] = in_R9;
  in_R9 = (int *)puVar39[0x7c];
  if (SBORROW8(0,puVar39[0x7b])) {
    puVar39[0x72] = in_R9;
    *(undefined1 *)((longlong)puVar39 + 0x563) = 1;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7d48;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "parsing Decoratorsparsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0x12;
    puVar39[0x9e] = in_R9;
    *(undefined1 *)((longlong)puVar39 + 0x563) = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7daa;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    pbVar20 = extraout_RAX_18;
    goto code_r0x0001417a8abd;
  }
  puVar24 = (undefined *)puVar39[0x7d];
  puVar39[0x57] = puVar39[0x7b];
code_r0x0001417a7dc0:
  puVar39[0xad] = in_R9;
  puVar39[0x58] = in_R9;
code_r0x0001417a7dce:
  puVar39[0x98] = puVar24;
  puVar39[0x59] = puVar24;
code_r0x0001417a7de1:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7df4;
  serde_json::value::index::impl$1::index_into();
  in_R9 = (int *)puVar39[0x9b];
  pbVar20 = extraout_RAX_19;
  uVar26 = extraout_XMM0_Qa_03;
  uVar43 = extraout_XMM0_Qb_03;
  if (extraout_RAX_19 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
code_r0x0001417a7e07:
  iVar18 = *(int *)(&UNK_142cdcae0 + (ulonglong)*pbVar20 * 4);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
code_r0x0001417a7e1a:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
    bVar17 = pbVar20[1];
code_r0x0001417a7e82:
    *(byte *)((longlong)puVar39 + 0x4e1) = bVar17;
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
code_r0x0001417a7e49:
    uVar26 = *(undefined8 *)(pbVar20 + 8);
    uVar43 = *(undefined8 *)(pbVar20 + 0x10);
code_r0x0001417a7e4e:
    puVar39[0x9d] = uVar26;
    puVar39[0x9e] = uVar43;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
code_r0x0001417a7e66:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7e75;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
code_r0x0001417a7e26:
code_r0x0001417a7e31:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7e40;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7ea4;
    alloc::collections::btree::map::impl$2::
    clone<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 5;
    break;
  case 6:
code_r0x0001417a7f8f:
    pbVar20 = (byte *)(ulonglong)((int)pbVar20 + 0x1e887c4);
    goto code_r0x0001417a7f94;
  case 7:
    goto code_r0x0001417a7ff3;
  case 8:
    goto code_r0x0001417a7fbe;
  case 9:
    goto code_r0x0001417a7fdb;
  case 10:
    puVar38 = (undefined8 *)(puVar36 + -8);
    puVar36 = puVar36 + -8;
    *puVar38 = in_R11;
    *(int *)(pbVar20 + 99) = *(int *)(pbVar20 + 99) + iVar18 + 0x42cdcae0;
    pbVar20 = (byte *)((ulonglong)pbVar20 | 0x8a);
    goto code_r0x0001417a7f94;
  case 0xb:
    *pbVar20 = *pbVar20 + (char)pbVar20;
    puVar39 = (undefined8 *)(ulonglong)(uint)((int)puVar39 + iVar29);
    bVar17 = (char)pbVar20 - 0xf;
    goto code_r0x0001417a800f;
  }
code_r0x0001417a7eab:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7eba;
  serde_json::value::de::impl$2::
  deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<tuple_<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>_>_>
            ();
  puVar39[0x4a] = pbVar42;
  pbVar42 = (byte *)puVar39[0x7c];
  if (SBORROW8(0,puVar39[0x7b])) {
    puVar39[0x72] = pbVar42;
    *(undefined1 *)((longlong)puVar39 + 0x562) = 1;
    *(undefined **)(puVar36 + -8) = &UNK_1417a7ef1;
    std::backtrace::Backtrace::capture();
    puVar39[0x8a] = puVar39[0xa0];
    puVar39[0x8b] = puVar39[0xa1];
    puVar39[0x88] = puVar39[0x9e];
    puVar39[0x89] = puVar39[0x9f];
    puVar39[0x86] = puVar39[0x9c];
    puVar39[0x87] = puVar39[0x9d];
    puVar39[0x9c] =
         "parsing WallColorsparsing GateColorsparsing Terrainparsing Treesparsing RasterIgnoring unknown raster data in snapshot: "
    ;
    puVar39[0x9d] = 0x12;
    puVar39[0x9e] = pbVar42;
code_r0x0001417a7f3d:
    *(undefined1 *)((longlong)puVar39 + 0x562) = 0;
code_r0x0001417a7f47:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7f53;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    pbVar20 = extraout_RAX_20;
    goto code_r0x0001417a7f53;
  }
  puVar39[0x79] = puVar39[0x7b];
code_r0x0001417a7f62:
  puVar39[0x6e] = pbVar42;
  pbVar42 = (byte *)puVar39[0x7d];
code_r0x0001417a7f83:
  *(undefined **)(puVar36 + -8) = &UNK_1417a7f88;
  serde_json::value::index::impl$1::index_into();
  pbVar20 = extraout_RAX_21;
  if (extraout_RAX_21 == (byte *)0x0) {
    pbVar20 = (byte *)CONCAT44(__imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                               ._4_4_,(uint)
                                      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
                              );
  }
code_r0x0001417a7f94:
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar20) {
  case 0:
code_r0x0001417a7fa7:
    *(undefined1 *)(puVar39 + 0x9c) = 0;
    break;
  case 1:
    bVar17 = pbVar20[1];
code_r0x0001417a800f:
    *(byte *)((longlong)puVar39 + 0x4e1) = bVar17;
    *(undefined1 *)(puVar39 + 0x9c) = 1;
    break;
  case 2:
code_r0x0001417a7fd6:
    uVar26 = *(undefined8 *)(pbVar20 + 8);
    uVar43 = *(undefined8 *)(pbVar20 + 0x10);
code_r0x0001417a7fdb:
    puVar39[0x9d] = uVar26;
    puVar39[0x9e] = uVar43;
    *(undefined1 *)(puVar39 + 0x9c) = 2;
    break;
  case 3:
code_r0x0001417a7ff3:
    *(undefined **)(puVar36 + -8) = &UNK_1417a8002;
    alloc::string::impl$6::clone();
    *(undefined1 *)(puVar39 + 0x9c) = 3;
    break;
  case 4:
code_r0x0001417a7fb3:
code_r0x0001417a7fbe:
    *(undefined **)(puVar36 + -8) = &UNK_1417a7fcd;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 4;
    break;
  case 5:
    *(undefined **)(puVar36 + -8) = &UNK_1417a8031;
    alloc::collections::btree::map::impl$2::
    clone<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    *(undefined1 *)(puVar39 + 0x9c) = 5;
  }
code_r0x0001417a8038:
  *(undefined **)(puVar36 + -8) = &UNK_1417a8047;
  serde_json::value::de::impl$2::
  deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<tuple_<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>_>_>
            ();
  *(undefined **)(puVar36 + -8) = &UNK_1417a8067;
  anyhow::context::impl$0::
  context<country_core::resources::walls::decorator_storage::DecoratorStorageSnapshot,serde_json::error::Error,ref_<str_>_>
            ();
  puVar39[0x4b] = pbVar42;
  pbVar20 = (byte *)puVar39[0x7c];
  if (!SBORROW8(0,puVar39[0x7b])) {
    puVar39[0x7a] = puVar39[0x7b];
    puVar39[0x6f] = pbVar20;
    uVar26 = puVar39[0x7d];
    *(undefined **)(puVar36 + -8) = &UNK_1417a80b5;
    serde_json::value::index::impl$1::index_into();
    *(undefined **)(puVar36 + -8) = &UNK_1417a80cc;
    serde_json::value::impl$4::clone();
    *(undefined **)(puVar36 + -8) = &UNK_1417a80db;
    serde_json::value::de::impl$2::
    deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<tuple_<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>_>_>
              ();
    puVar39[0x9c] = 0;
    puVar39[0x9d] = 8;
    puVar39[0x9e] = 0;
    *(undefined **)(puVar36 + -8) = &UNK_1417a8112;
    core::result::Result::
    unwrap_or<alloc::vec::Vec<tuple_<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>,alloc::alloc::Global>,serde_json::error::Error>
              ();
    *(undefined **)(puVar36 + -8) = &UNK_1417a812a;
    serde_json::value::index::impl$1::index_into();
    *(undefined **)(puVar36 + -8) = &UNK_1417a8145;
    serde_json::value::impl$4::clone();
    *(undefined **)(puVar36 + 0x20) =
         &anon_12ce4fde5425384d5f438f5122902e1a_92_llvm_3277990281340110158;
    *(undefined8 *)(puVar36 + 0x28) = 3;
    *(undefined **)(puVar36 + -8) = &UNK_1417a8176;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::terrain_editing::_::impl_0::deserialize::__Visitor>
              ();
    *(undefined **)(puVar36 + -8) = &UNK_1417a8192;
    anyhow::context::impl$0::
    context<country_core::resources::terrain_editing::TerrainEditingData,serde_json::error::Error,ref_<str_>_>
              ();
    if (*(int *)(puVar39 + 0x86) == 2) {
      pbVar20 = (byte *)puVar39[0x87];
    }
    else {
      puVar39[0x83] = puVar39[0x8e];
      puVar39[0x81] = puVar39[0x8c];
      puVar39[0x82] = puVar39[0x8d];
      puVar39[0x7f] = puVar39[0x8a];
      puVar39[0x80] = puVar39[0x8b];
      puVar39[0x7d] = puVar39[0x88];
      puVar39[0x7e] = puVar39[0x89];
      *(int *)(puVar39 + 0x7b) = *(int *)(puVar39 + 0x86);
      *(undefined4 *)((longlong)puVar39 + 0x3dc) = *(undefined4 *)((longlong)puVar39 + 0x434);
      puVar39[0x7c] = puVar39[0x87];
      *(undefined **)(puVar36 + -8) = &UNK_1417a8215;
      serde_json::value::index::impl$1::index_into();
      *(undefined **)(puVar36 + -8) = &UNK_1417a822c;
      serde_json::value::impl$4::clone();
      *(undefined **)(puVar36 + -8) = &UNK_1417a823b;
      serde_json::value::de::impl$2::
      deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<tuple_<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>_>_>
                ();
      *(undefined **)(puVar36 + -8) = &UNK_1417a825b;
      anyhow::context::impl$0::
      context<country_core::resources::walls::decorator_storage::DecoratorStorageSnapshot,serde_json::error::Error,ref_<str_>_>
                ();
      puVar39[0x4c] = uVar26;
      pbVar20 = (byte *)puVar39[0x73];
      if (!SBORROW8(0,puVar39[0x72])) {
        puVar39[0x4e] = puVar39[0x72];
        puVar39[0x4f] = pbVar20;
        puVar39[0x50] = puVar39[0x74];
        *(undefined **)(puVar36 + -8) = &UNK_1417a82b0;
        serde_json::value::index::impl$1::index_into();
        *(undefined **)(puVar36 + -8) = &UNK_1417a82cb;
        serde_json::value::impl$4::clone();
        *(undefined **)(puVar36 + -8) = &UNK_1417a82da;
        serde_json::value::de::impl$2::
        deserialize_seq<serde::de::impls::impl_89::deserialize::SeqVisitor<u64,std::hash::random::RandomState>_>
                  ();
        *(undefined **)(puVar36 + -8) = &UNK_1417a82e9;
        core::result::Result::
        unwrap_or_default<std::collections::hash::set::HashSet<u64,std::hash::random::RandomState>,serde_json::error::Error>
                  ();
        *(undefined **)(puVar36 + -8) = &UNK_1417a8301;
        serde_json::value::index::impl$1::index_into();
        *(undefined **)(puVar36 + -8) = &UNK_1417a831c;
        serde_json::value::impl$4::clone();
        *(undefined **)(puVar36 + 0x20) =
             &anon_369a2eef05c89d4ad2f1229306672ae6_55_llvm_6346813648557899910;
        *(undefined8 *)(puVar36 + 0x28) = 2;
        *(undefined **)(puVar36 + -8) = &UNK_1417a834d;
        serde_json::value::de::impl$2::
        deserialize_struct<country_core::resources::stairs::stairs_state::_::impl_0::deserialize::__Visitor>
                  ();
        lVar23 = puVar39[0x9c];
        if (lVar23 == 2) {
          *(undefined **)(puVar36 + -8) = &UNK_1417a8366;
          core::ptr::drop_in_place<serde_json::error::Error>();
          puVar39[0x9a] = 4;
          lVar23 = 0;
          puVar39[0x85] = 0;
          puVar39[0x70] = 0;
        }
        else {
          in_R9 = (int *)puVar39[0x9d];
          puVar39[0x85] = puVar39[0x9e];
          puVar39[0x9a] = puVar39[0x9f];
          puVar39[0x70] = puVar39[0xa0];
        }
        *(undefined **)(puVar36 + -8) = &UNK_1417a83d6;
        serde_json::value::index::impl$1::index_into();
        *(undefined **)(puVar36 + -8) = &UNK_1417a83f1;
        serde_json::value::impl$4::clone();
        *(undefined **)(puVar36 + 0x20) =
             &anon_66a3d72518e7f93316b3f3cb8ace3c6b_95_llvm_1564075658915195154;
        *(undefined8 *)(puVar36 + 0x28) = 1;
        *(undefined **)(puVar36 + -8) = &UNK_1417a8422;
        serde_json::value::de::impl$2::
        deserialize_struct<country_core::resources::stairs::stairs_removed_supports::_::impl_0::deserialize::__Visitor>
                  ();
        *(undefined **)(puVar36 + -8) = &UNK_1417a8431;
        core::result::Result::
        unwrap_or_default<std::collections::hash::set::HashSet<u64,std::hash::random::RandomState>,serde_json::error::Error>
                  ();
        puVar39[0x4d] = lVar23;
        puVar39[0x9b] = in_R9;
        lVar23 = puVar39[0xad];
        puVar39[0x49] = lVar23 + puVar39[0x98] * 0x28;
        lVar41 = puVar39[0x9a];
        lVar9 = puVar39[0x70];
        puVar39[0x71] = in_RCX;
        while (lVar23 != puVar39[0x49]) {
          puVar39[0x48] = lVar23 + 0x28;
          piVar25 = *(int **)(lVar23 + 0x18);
          piVar21 = piVar25 + *(longlong *)(lVar23 + 0x20) * 0x16;
          puVar39[0x98] = piVar21;
          while (piVar28 = piVar25, piVar28 != piVar21) {
            piVar25 = piVar28 + 0x16;
            if (*(char *)((longlong)piVar28 + 0x56) == '\x01') {
              puVar39[0xad] = piVar28 + 0x16;
              piVar25 = (int *)puVar39[0x9a];
              while (piVar25 != (int *)(lVar41 + lVar9 * 8)) {
                piVar21 = piVar25 + 2;
                piVar1 = piVar25 + 1;
                if ((*piVar25 == *piVar28) || (piVar25 = piVar21, *piVar1 == *piVar28)) {
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8509;
                  hashbrown::map::HashMap::
                  insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                            ();
                  piVar25 = piVar21;
                }
              }
              *(undefined1 *)((longlong)piVar28 + 0x56) = 0;
              piVar21 = (int *)puVar39[0x98];
              piVar25 = (int *)puVar39[0xad];
            }
          }
          lVar23 = puVar39[0x48];
        }
        *(undefined **)(puVar36 + -8) = &UNK_1417a8543;
        serde_json::value::index::impl$1::index_into();
        *(undefined **)(puVar36 + -8) = &UNK_1417a855a;
        serde_json::value::impl$4::clone();
        *(undefined **)(puVar36 + -8) = &UNK_1417a8569;
        serde_json::value::de::impl$2::
        deserialize_seq<serde::de::impls::impl_19::deserialize::VecVisitor<alloc::string::String>_>
                  ();
        *(undefined **)(puVar36 + -8) = &UNK_1417a8589;
        anyhow::context::impl$0::
        context<country_core::resources::walls::decorator_storage::DecoratorStorageSnapshot,serde_json::error::Error,ref_<str_>_>
                  ();
        pbVar20 = (byte *)puVar39[0x62];
        if (!SBORROW8(0,puVar39[0x61])) {
          puVar39[0x9c] = pbVar20;
          puVar39[0x9d] = pbVar20;
          puVar39[0x9e] = puVar39[0x61];
          puVar39[0x9f] = pbVar20 + puVar39[99] * 0x18;
          puVar39[0xa0] = puVar39 + 0x1c;
          *(undefined **)(puVar36 + -8) = &UNK_1417a85eb;
          core::iter::adapters::
          try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env_0>,tuple_<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>_>,enum2_<core::result::Result<enum2_<core::convert::Infallible>,anyhow::Error>_>,core::result::impl_25::from_iter::closure_env_0<tuple_<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>_>,anyhow::Error,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env_0>_>,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState>_>
                    ();
          pbVar20 = (byte *)puVar39[0x41];
          if (puVar39[0x40] != 0) {
            puVar39[0x8a] = puVar39[0x44];
            puVar39[0x8b] = puVar39[0x45];
            puVar39[0x88] = puVar39[0x42];
            puVar39[0x89] = puVar39[0x43];
            puVar39[0x86] = puVar39[0x40];
            puVar39[0x87] = pbVar20;
            *(undefined **)(puVar36 + -8) = &UNK_1417a8641;
            resources::world_raster_defs::AllWorldRasterWrite::ordered_names();
            lVar23 = puVar39[0x1a];
            puVar39[0x9c] = lVar23;
            puVar39[0x9d] = lVar23;
            puVar39[0x9e] = puVar39[0x19];
            puVar39[0x9f] = puVar39[0x1b] * 0x10 + lVar23;
            puVar39[0xa0] = puVar39 + 0x86;
            *(undefined **)(puVar36 + -8) = &UNK_1417a869d;
            alloc::vec::in_place_collect::impl$1::
            from_iter<tuple_<ref_<str_>,enum2_<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_<str_>,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env_1>_>
                      ();
            if ((puVar39[0x89] != 0) &&
               (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
              pauVar10 = (undefined1 (*) [16])puVar39[0x86];
              auVar3 = *pauVar10;
              puVar39[0x9c] = pauVar10;
              puVar39[0x9d] = pauVar10 + 1;
              puVar39[0x9e] = *pauVar10 + puVar39[0x87] + 1;
              *(ushort *)(puVar39 + 0x9f) =
                   ~((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
              puVar39[0xa0] = puVar39[0x89];
              *(undefined1 *)((longlong)puVar39 + 0x575) = 1;
              *(undefined **)(puVar36 + -8) = &UNK_1417a8726;
              alloc::vec::spec_from_iter::impl$0::
              from_iter<ref_<alloc::string::String>,std::collections::hash::map::Keys<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>_>_>
                        ();
              puVar39[0x69] = puVar39 + 0x66;
              puVar39[0x6a] =
                   alloc::vec::impl$27::fmt<ref_<alloc::string::String>,alloc::alloc::Global>;
              puVar39[0x9c] = &UNK_142cdc7c0;
              puVar39[0x9d] = 1;
              puVar39[0xa0] = 0;
              puVar39[0x9e] = puVar39 + 0x69;
              puVar39[0x9f] = 1;
              *(undefined **)(puVar36 + -8) = &UNK_1417a878b;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              puVar39[0x61] = &DAT_142cdc6c0;
              puVar39[0x62] = 0x25;
              puVar39[99] = &DAT_142cdc6c0;
              puVar39[100] = 0x25;
              puVar39[0x65] = extraout_RAX_22;
              *(undefined **)(puVar36 + -8) = &UNK_1417a87d1;
              ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
              if (puVar39[0x66] != 0) {
                *(undefined **)(puVar36 + -8) = &UNK_1417a87f3;
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            puVar39[0x54] = puVar39[0x16];
            puVar39[0x55] = puVar39[0x17];
            puVar39[0x56] = puVar39[0x18];
            *(undefined **)(puVar36 + -8) = &UNK_1417a8824;
            resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed();
            pbVar20 = extraout_RAX_23;
            if (extraout_RAX_23 == (byte *)0x0) {
              *(undefined **)(puVar36 + -8) = &UNK_1417a8850;
              alloc::vec::from_elem<country_core::resources::rt::RtTriangleMetaSsbo>();
              *(undefined **)(puVar36 + -8) = &UNK_1417a8863;
              motomoto::impl$9::next();
              *(undefined **)(puVar36 + -8) = &UNK_1417a8876;
              try_chunk();
              lVar23 = puVar39[0x66];
              if (lVar23 == 0) {
                pbVar20 = (byte *)puVar39[0x67];
              }
              else {
                uVar26 = puVar39[0x67];
                *(undefined **)(puVar36 + -8) = &UNK_1417a889b;
                flate2::mem::Decompress::new();
                puVar39[0x9c] = lVar23;
                puVar39[0x9d] = uVar26;
                uVar26 = puVar39[0x6b];
                puVar39[0xad] = puVar39[0x6a];
                *(undefined **)(puVar36 + -8) = &UNK_1417a88ce;
                std::io::
                default_read_exact<flate2::deflate::bufread::DeflateDecoder<ref_<slice2_<u8>_>_>_>()
                ;
                *(undefined **)(puVar36 + -8) = &UNK_1417a88e3;
                anyhow::context::impl$0::context<tuple_<>,std::io::error::Error,ref_<str_>_>();
                if (extraout_RAX_24 == (byte *)0x0) {
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8bcb;
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                  puVar39[0x36] = puVar39[0x54];
                  puVar39[0x37] = puVar39[0x55];
                  puVar39[0x38] = puVar39[0x56];
                  puVar39[0x84] = puVar39[0x69];
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8c16;
                  hashbrown::raw::RawTableInner::
                  drop_inner_table<tuple_<alloc::string::String,alloc::string::String>,alloc::alloc::Global>
                            ();
                  puVar39[0x88] = puVar39[0x38];
                  puVar39[0x86] = puVar39[0x36];
                  puVar39[0x87] = puVar39[0x37];
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8c4c;
                  serde_json::value::index::impl$1::index_into();
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8c67;
                  serde_json::value::impl$4::clone();
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8c7a;
                  serde_json::value::from_value<core::ops::range::Range<f32>_>();
                  if (*(int *)(puVar39 + 0x61) == 1) {
                    *(undefined **)(puVar36 + -8) = &UNK_1417a8c8f;
                    core::ptr::drop_in_place<serde_json::error::Error>();
                    uVar44 = 0xc0200000;
                    uVar45 = 0x41a00000;
                  }
                  else {
                    uVar44 = (undefined4)*(undefined8 *)((longlong)puVar39 + 0x30c);
                    uVar45 = (undefined4)
                             ((ulonglong)*(undefined8 *)((longlong)puVar39 + 0x30c) >> 0x20);
                  }
                  puVar39[0x22] = puVar39[0x5a];
                  puVar39[0x23] = puVar39[0x5b];
                  puVar39[0x24] = puVar39[0x5c];
                  puVar39[0x26] = puVar39[0x57];
                  puVar39[0x27] = puVar39[0x58];
                  puVar39[0x28] = puVar39[0x59];
                  puVar39[0x2a] = puVar39[0x51];
                  puVar39[0x2b] = puVar39[0x52];
                  puVar39[0x2c] = puVar39[0x53];
                  puVar39[6] = puVar39[0x7b];
                  puVar39[7] = puVar39[0x7c];
                  puVar39[8] = puVar39[0x7d];
                  puVar39[9] = puVar39[0x7e];
                  puVar39[10] = puVar39[0x7f];
                  puVar39[0xb] = puVar39[0x80];
                  puVar39[0xc] = puVar39[0x81];
                  puVar39[0xd] = puVar39[0x82];
                  puVar39[0xe] = puVar39[0x83];
                  puVar39[0x30] = puVar39[0x88];
                  puVar39[0x2e] = puVar39[0x86];
                  puVar39[0x2f] = puVar39[0x87];
                  puVar39[0x34] = puVar39[0x50];
                  puVar39[0x32] = puVar39[0x4e];
                  puVar39[0x33] = puVar39[0x4f];
                  puVar39[-2] = puVar39[0x14];
                  puVar39[-1] = puVar39[0x15];
                  puVar39[-4] = puVar39[0x12];
                  puVar39[-3] = puVar39[0x13];
                  puVar39[-6] = puVar39[0x10];
                  puVar39[-5] = puVar39[0x11];
                  puVar39[4] = puVar39[0x76];
                  puVar39[5] = puVar39[0x77];
                  puVar39[2] = puVar39[0x74];
                  puVar39[3] = puVar39[0x75];
                  *puVar39 = puVar39[0x72];
                  puVar39[1] = puVar39[0x73];
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8db4;
                  core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
                  puVar11 = (undefined4 *)puVar39[0x71];
                  *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8dce;
                  core::ptr::drop_in_place<memmap2::os::MmapInner>();
                  *(undefined8 *)(puVar11 + 6) = puVar39[0x85];
                  *(undefined8 *)(puVar11 + 8) = puVar39[0x9a];
                  *(undefined8 *)(puVar11 + 10) = puVar39[0x70];
                  *(undefined8 *)(puVar11 + 0x10) = puVar39[0x24];
                  uVar5 = puVar39[0x23];
                  *(undefined8 *)(puVar11 + 0xc) = puVar39[0x22];
                  *(undefined8 *)(puVar11 + 0xe) = uVar5;
                  *(undefined8 *)(puVar11 + 0x12) = puVar39[0x78];
                  *(undefined8 *)(puVar11 + 0x14) = puVar39[0x6d];
                  *(undefined8 *)(puVar11 + 0x16) = puVar39[0x4a];
                  *(undefined8 *)(puVar11 + 0x1c) = puVar39[0x28];
                  uVar5 = puVar39[0x27];
                  *(undefined8 *)(puVar11 + 0x18) = puVar39[0x26];
                  *(undefined8 *)(puVar11 + 0x1a) = uVar5;
                  *(undefined8 *)(puVar11 + 0x1e) = puVar39[0x79];
                  *(undefined8 *)(puVar11 + 0x20) = puVar39[0x6e];
                  *(undefined8 *)(puVar11 + 0x22) = puVar39[0x4b];
                  *(undefined8 *)(puVar11 + 0x24) = puVar39[0x7a];
                  *(undefined8 *)(puVar11 + 0x26) = puVar39[0x6f];
                  *(undefined8 *)(puVar11 + 0x28) = puVar39[0x4c];
                  *(undefined8 *)(puVar11 + 0x2e) = puVar39[0x2c];
                  uVar5 = puVar39[0x2b];
                  *(undefined8 *)(puVar11 + 0x2a) = puVar39[0x2a];
                  *(undefined8 *)(puVar11 + 0x2c) = uVar5;
                  *(undefined8 *)(puVar11 + 0x34) = puVar39[0x30];
                  uVar5 = puVar39[0x2f];
                  *(undefined8 *)(puVar11 + 0x30) = puVar39[0x2e];
                  *(undefined8 *)(puVar11 + 0x32) = uVar5;
                  *(undefined8 *)(puVar11 + 0x36) = puVar39[0x84];
                  *(undefined8 *)(puVar11 + 0x38) = puVar39[0xad];
                  *(undefined8 *)(puVar11 + 0x3a) = uVar26;
                  *(undefined8 *)(puVar11 + 0x40) = puVar39[0x34];
                  uVar5 = puVar39[0x33];
                  *(undefined8 *)(puVar11 + 0x3c) = puVar39[0x32];
                  *(undefined8 *)(puVar11 + 0x3e) = uVar5;
                  *(undefined8 *)(puVar11 + 0x52) = puVar39[0xe];
                  uVar5 = puVar39[6];
                  uVar26 = puVar39[7];
                  uVar43 = puVar39[8];
                  uVar12 = puVar39[9];
                  uVar13 = puVar39[10];
                  uVar14 = puVar39[0xb];
                  uVar15 = puVar39[0xd];
                  *(undefined8 *)(puVar11 + 0x4e) = puVar39[0xc];
                  *(undefined8 *)(puVar11 + 0x50) = uVar15;
                  *(undefined8 *)(puVar11 + 0x4a) = uVar13;
                  *(undefined8 *)(puVar11 + 0x4c) = uVar14;
                  *(undefined8 *)(puVar11 + 0x46) = uVar43;
                  *(undefined8 *)(puVar11 + 0x48) = uVar12;
                  *(undefined8 *)(puVar11 + 0x42) = uVar5;
                  *(undefined8 *)(puVar11 + 0x44) = uVar26;
                  uVar5 = puVar39[-6];
                  uVar26 = puVar39[-5];
                  uVar43 = puVar39[-4];
                  uVar12 = puVar39[-3];
                  uVar13 = puVar39[-1];
                  *(undefined8 *)(puVar11 + 0x5c) = puVar39[-2];
                  *(undefined8 *)(puVar11 + 0x5e) = uVar13;
                  *(undefined8 *)(puVar11 + 0x58) = uVar43;
                  *(undefined8 *)(puVar11 + 0x5a) = uVar12;
                  *(undefined8 *)(puVar11 + 0x54) = uVar5;
                  *(undefined8 *)(puVar11 + 0x56) = uVar26;
                  uVar5 = *puVar39;
                  uVar26 = puVar39[1];
                  uVar43 = puVar39[2];
                  uVar12 = puVar39[3];
                  uVar13 = puVar39[5];
                  *(undefined8 *)(puVar11 + 0x68) = puVar39[4];
                  *(undefined8 *)(puVar11 + 0x6a) = uVar13;
                  *(undefined8 *)(puVar11 + 100) = uVar43;
                  *(undefined8 *)(puVar11 + 0x66) = uVar12;
                  *(undefined8 *)(puVar11 + 0x60) = uVar5;
                  *(undefined8 *)(puVar11 + 0x62) = uVar26;
                  *puVar11 = uVar4;
                  *(undefined8 *)(puVar11 + 2) = puVar39[0x4d];
                  *(undefined8 *)(puVar11 + 4) = puVar39[0x9b];
                  *(ulonglong *)(puVar11 + 0x6c) = CONCAT44(uVar45,uVar44);
                  if (puVar39[0x5d] == 0) {
                    return;
                  }
                  *(undefined **)(puVar36 + -8) = &UNK_1417a8fbb;
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                  return;
                }
                *(undefined **)(puVar36 + -8) = &UNK_1417a8906;
                std::alloc::__default_lib_allocator::__rust_dealloc();
                pbVar20 = extraout_RAX_24;
              }
              if (puVar39[0x69] != 0) {
                *(undefined **)(puVar36 + -8) = &UNK_1417a8931;
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            *(undefined1 *)((longlong)puVar39 + 0x575) = 0;
            *(undefined **)(puVar36 + -8) = &UNK_1417a8944;
            core::ptr::
            drop_in_place<alloc::vec::Vec<tuple_<ref_<str_>,enum2_<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>,alloc::alloc::Global>_>
                      ();
            *(undefined **)(puVar36 + -8) = &UNK_1417a8963;
            hashbrown::raw::RawTableInner::
            drop_inner_table<tuple_<alloc::string::String,alloc::string::String>,alloc::alloc::Global>
                      ();
          }
        }
        *(undefined **)(puVar36 + -8) = &UNK_1417a896f;
        bevy_ecs::component::ComponentDescriptor::
        drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        in_RCX = puVar39[0x71];
        in_R9 = (int *)puVar39[0x9b];
        if (puVar39[0x85] != 0) {
          *(undefined **)(puVar36 + -8) = &UNK_1417a899f;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        *(undefined **)(puVar36 + -8) = &UNK_1417a89ab;
        bevy_ecs::component::ComponentDescriptor::
        drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
        *(undefined **)(puVar36 + -8) = &UNK_1417a89b7;
        core::ptr::
        drop_in_place<alloc::vec::Vec<tuple_<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,alloc::alloc::Global>_>
                  ();
      }
      *(undefined **)(puVar36 + -8) = &UNK_1417a89d6;
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>
                ();
    }
    if (puVar39[0x51] != 0) {
      *(undefined **)(puVar36 + -8) = &UNK_1417a89f8;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (puVar39[0x7a] != 0) {
      *(undefined **)(puVar36 + -8) = &UNK_1417a8a1a;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  puVar39[0x9b] = in_R9;
  if (puVar39[0x79] != 0) {
    *(undefined **)(puVar36 + -8) = &UNK_1417a8a43;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x0001417a8a43:
  puVar39[0xad] = puVar39[0x58];
  lVar23 = puVar39[0x59];
  if (lVar23 != 0) {
    lVar41 = puVar39[0xad] + 0x18;
    do {
      if (*(longlong *)(lVar41 + -8) != 0) {
        *(undefined **)(puVar36 + -8) = &UNK_1417a8a95;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar41 = lVar41 + 0x28;
      lVar23 = lVar23 + -1;
    } while (lVar23 != 0);
  }
  if (puVar39[0x57] != 0) {
    *(undefined **)(puVar36 + -8) = &UNK_1417a8abd;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x0001417a8abd:
  puVar39[0x9b] = puVar39[0x9b];
  if (puVar39[0x78] != 0) {
    *(undefined **)(puVar36 + -8) = &UNK_1417a8aed;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x0001417a8aed:
  puVar39[0xad] = puVar39[0x5b];
  lVar23 = puVar39[0x5c];
  if (lVar23 != 0) {
    lVar41 = puVar39[0xad] + 0x28;
    do {
      if (*(longlong *)(lVar41 + -0x20) != 0) {
        *(undefined **)(puVar36 + -8) = &UNK_1417a8b45;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (*(longlong *)(lVar41 + -8) != 0) {
        *(undefined **)(puVar36 + -8) = &UNK_1417a8b61;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar41 = lVar41 + 0x78;
      lVar23 = lVar23 + -1;
    } while (lVar23 != 0);
  }
  if (puVar39[0x5a] != 0) {
    *(undefined **)(puVar36 + -8) = &UNK_1417a8b89;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x0001417a779a:
  in_R9 = (int *)puVar39[0x99];
  bVar17 = *(byte *)(puVar39 + 0x94);
  pbVar42 = pbVar20;
joined_r0x0001417a789a:
  if (2 < bVar17) {
    if (bVar17 == 3) {
      lVar23 = puVar39[0x95];
    }
    else {
      if (bVar17 != 4) {
        *(undefined **)(puVar36 + -8) = &UNK_1417a7917;
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
        goto LAB_1417a75b4;
      }
      puVar39[0xad] = puVar39[0x96];
      *(undefined **)(puVar36 + -8) = &UNK_1417a78c1;
      core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
      lVar23 = puVar39[0x95];
    }
    if (lVar23 != 0) {
code_r0x0001417a7901:
      *(undefined **)(puVar36 + -8) = &UNK_1417a7906;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
LAB_1417a75b4:
  *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a75c7;
  core::ptr::drop_in_place<memmap2::os::MmapInner>();
LAB_1417a70cf:
  puVar39[0x72] = pbVar42;
  if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    *(undefined1 *)((longlong)puVar39 + 0x576) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a70fd;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    *(undefined1 *)((longlong)puVar39 + 0x576) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a711c;
    ::utils::file::impl$0::tail();
    puVar39[0x40] = puVar39 + 0x7b;
    puVar39[0x41] = std::path::impl$44::fmt;
    puVar39[0x42] = puVar39 + 0x72;
    puVar39[0x43] = anyhow::error::impl$5::fmt;
    puVar39[0x86] = 2;
    puVar39[0x88] = 2;
    puVar39[0x8a] = 0;
    puVar39[0x8b] = 0x20;
    *(undefined1 *)(puVar39 + 0x8c) = 3;
    puVar39[0x8d] = 2;
    puVar39[0x8f] = 2;
    puVar39[0x91] = 1;
    puVar39[0x92] = 0x400000020;
    *(undefined1 *)(puVar39 + 0x93) = 3;
    *(undefined8 *)(puVar36 + -8) = 0x1417a71cc;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    uVar26 = *extraout_RAX_04;
    uVar43 = extraout_RAX_04[1];
    uVar4 = *(undefined4 *)(extraout_RAX_04 + 2);
    puVar39[0xa2] = 2;
    puVar39[0xa3] = &DAT_142cdc6c0;
    puVar39[0xa4] = 0x25;
    puVar39[0xa6] = &PTR_s_Failed_to_load_history_snapshot___142cdc688;
    puVar39[0xa7] = 2;
    puVar39[0xa8] = puVar39 + 0x40;
    puVar39[0xa9] = 2;
    puVar39[0xaa] = puVar39 + 0x86;
    puVar39[0xab] = 2;
    puVar39[0x9c] = 0;
    puVar39[0x9d] = &DAT_142cdc6c0;
    puVar39[0x9e] = 0x25;
    puVar39[0x9f] = 0;
    puVar39[0xa0] = uVar26;
    puVar39[0xa1] = uVar43;
    *(undefined4 *)(puVar39 + 0xa5) = 1;
    *(undefined4 *)((longlong)puVar39 + 0x52c) = uVar4;
    *(undefined8 *)(puVar36 + -8) = 0x1417a7291;
    ::log::__private_api::impl$2::log();
    if (puVar39[0x7b] != 0) {
      *(undefined8 *)(puVar36 + -8) = 0x1417a72af;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  puVar39[0x9c] = uVar5;
  *(undefined1 *)((longlong)puVar39 + 0x576) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a72cc;
  alloc::vec::Vec::
  retain<country_core::resources::history::HistoryChainSnapshot,alloc::alloc::Global,country_core::resources::history::impl_15::remove_snapshot_record::closure_env_0>
            ();
  *(undefined1 *)((longlong)puVar39 + 0x576) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a72df;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  *(undefined1 *)((longlong)puVar39 + 0x576) = 1;
  *(undefined8 *)(puVar36 + -8) = 0x1417a72fb;
  std::path::Path::_starts_with();
  if (extraout_AL == '\0') {
    puVar39[0x9e] = puVar39[0x5f];
    puVar39[0x9f] = puVar39[0x60];
    puVar39[0x9c] = puVar39[0x5d];
    puVar39[0x9d] = puVar39[0x5e];
    *(undefined8 *)(puVar36 + -8) = 0x1417a7366;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    *(undefined8 *)(puVar36 + -8) = 0x1417a736e;
    std::sys::pal::windows::fs::unlink();
    if (puVar39[0x9c] != 0) {
      *(undefined8 *)(puVar36 + -8) = 0x1417a738f;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (((uint)extraout_RAX_05 & 3) == 1) {
      puVar39[0x9a] = extraout_RAX_05 + -1;
      puVar39[0xad] = *(undefined8 *)(extraout_RAX_05 + -1);
      puVar7 = *(undefined8 **)(extraout_RAX_05 + 7);
      puVar39[0x98] = puVar7;
      pcVar8 = (code *)*puVar7;
      if (pcVar8 != (code *)0x0) {
        uVar5 = puVar39[0xad];
        *(undefined8 *)(puVar36 + -8) = 0x1417a73d2;
        (*pcVar8)(uVar5);
      }
      if (*(longlong *)(puVar39[0x98] + 8) != 0) {
        *(undefined8 *)(puVar36 + -8) = 0x1417a73f6;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined8 *)(puVar36 + -8) = 0x1417a6ed7;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(undefined1 *)((longlong)puVar39 + 0x577) = 0;
    *(undefined8 *)(puVar36 + -8) = 0x1417a6eea;
    anyhow::error::impl$6::drop();
  }
  else {
    *(undefined1 *)((longlong)puVar39 + 0x577) = 1;
    *(undefined8 *)(puVar36 + -8) = 0x1417a7312;
    anyhow::error::impl$6::drop();
    if (puVar39[0x5d] != 0) {
      *(undefined8 *)(puVar36 + -8) = 0x1417a7334;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  goto LAB_1417a6eea;
code_r0x0001417a7f53:
  goto code_r0x0001417a8a43;
}

