
// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000140861678) overlaps instruction at (ram,0x000140861677)
// 
// WARNING: Removing unreachable block (ram,0x000140861688)
// WARNING: Removing unreachable block (ram,0x000142a95cc2)

void __cdecl
country_core::resources::walls::inner_wall_state::
deserialize_wall_style<enum2_<serde_json::value::Value>_>(void)

{
  uint *puVar1;
  byte *pbVar2;
  byte bVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong lVar6;
  uint uVar7;
  char cVar8;
  int iVar9;
  ulonglong uVar10;
  char *pcVar11;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong extraout_RAX_13;
  longlong extraout_RAX_14;
  byte bVar12;
  int *in_RCX;
  code *UNRECOVERED_JUMPTABLE_00;
  undefined2 uVar13;
  byte *in_RDX;
  char *pcVar14;
  int *unaff_RBX;
  char *unaff_RSI;
  byte *unaff_RDI;
  longlong in_R9;
  char unaff_R12B;
  char unaff_R14B;
  char unaff_R15B;
  longlong unaff_GS_OFFSET;
  bool bVar15;
  ulonglong in_MM0;
  ulonglong in_MM1;
  float unaff_XMM6_Da;
  float unaff_XMM7_Da;
  char in_stack_fffffffff7840f48;
  char cStack_c8;
  
  uVar10 = (ulonglong)*(int *)(&DAT_142a95cb8 + (ulonglong)*in_RDX * 4);
  pcVar11 = &DAT_142a95cb8 + uVar10;
  bVar15 = pcVar11 == (char *)0x0;
  cVar8 = (char)pcVar11;
  bVar12 = (byte)in_RCX;
  iVar9 = (int)in_RCX;
  uVar13 = SUB82(in_RDX,0);
  uVar7 = (uint)pcVar11;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*in_RDX) {
  case 0:
    goto code_r0x00014085ed89;
  case 1:
    *(byte *)((longlong)in_RCX + 1) = in_RDX[1];
    *(undefined1 *)in_RCX = 0;
    return;
  case 2:
code_r0x00014085ed89:
    serde_json::error::impl$8::invalid_type();
    *(undefined8 *)(in_RCX + 2) = extraout_RAX_00;
    *(undefined1 *)in_RCX = 1;
    return;
  case 3:
    lVar6 = *(longlong *)(in_RDX + 8);
    plVar4 = *(longlong **)(in_RDX + 0x10);
    lVar5 = *(longlong *)(in_RDX + 0x18);
    if (lVar5 == 5) {
      if (*(char *)((longlong)plVar4 + 4) == 'e' && (int)*plVar4 == 0x6e6f7453) {
        *(undefined1 *)((longlong)in_RCX + 1) = 0;
        goto code_r0x00014085ed08;
      }
    }
    else if (lVar5 == 0xf) {
      if (*(longlong *)((longlong)plVar4 + 7) == 0x737472696b536874 && *plVar4 == 0x746957656e6f7453
         ) {
        *(undefined1 *)((longlong)in_RCX + 1) = 1;
code_r0x00014085ed08:
        *(undefined1 *)in_RCX = 0;
        goto joined_r0x00014085edd8;
      }
    }
    else if ((lVar5 == 0xc) && ((int)plVar4[1] == 0x64657265 && *plVar4 == 0x626d6954666c6148)) {
      *(undefined1 *)((longlong)in_RCX + 1) = 2;
      goto code_r0x00014085ed08;
    }
    serde::de::Error::invalid_value<serde_json::error::Error>();
    *(undefined8 *)(in_RCX + 2) = extraout_RAX_01;
    *(undefined1 *)in_RCX = 1;
joined_r0x00014085edd8:
    if (lVar6 == 0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  case 4:
    serde_json::value::de::
    visit_array<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>
              ();
    return;
  case 5:
    serde_json::value::de::MapDeserializer::new();
    serde_json::error::impl$8::invalid_type();
    *(undefined8 *)(in_RCX + 2) = extraout_RAX;
    *(undefined1 *)in_RCX = 1;
    core::ptr::drop_in_place<serde_json::map::IntoIter>();
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
    return;
  case 6:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 7:
    if ((byte)((byte)(in_MM0 >> 7) & 1 | ((byte)(in_MM0 >> 0xf) & 1) << 1 |
               ((byte)(in_MM0 >> 0x17) & 1) << 2 | ((byte)(in_MM0 >> 0x1f) & 1) << 3 |
               ((byte)(in_MM0 >> 0x27) & 1) << 4 | ((byte)(in_MM0 >> 0x2f) & 1) << 5 |
               ((byte)(in_MM0 >> 0x37) & 1) << 6 | (byte)(in_MM0 >> 0x38) & 0x80) == 0xffff) {
      (&UNK_142a95cb9)[uVar10] = 3;
      goto code_r0x00014085f7a9;
    }
    break;
  case 8:
    if ((byte)((byte)(in_MM1 >> 7) & 1 | ((byte)(in_MM1 >> 0xf) & 1) << 1 |
               ((byte)(in_MM1 >> 0x17) & 1) << 2 | ((byte)(in_MM1 >> 0x1f) & 1) << 3 |
               ((byte)(in_MM1 >> 0x27) & 1) << 4 | ((byte)(in_MM1 >> 0x2f) & 1) << 5 |
               ((byte)(in_MM1 >> 0x37) & 1) << 6 | (byte)(in_MM1 >> 0x38) & 0x80) == 0xffff) {
      (&UNK_142a95cb9)[uVar10] = 0xd;
      goto code_r0x00014085f7a9;
    }
    break;
  case 9:
    if ((int)in_RDX == 0x74686769 && in_RCX == (int *)0x0) {
      (&UNK_142a95cb9)[uVar10] = 8;
      goto code_r0x00014085f7a9;
    }
    break;
  case 10:
    if (uVar10 < 0xfffffffebd56a348 && !bVar15) {
      bVar12 = in(uVar13);
      *unaff_RDI = bVar12;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    if (*(int *)(in_RDX + 3) == 0x42a95cb8 && iVar9 == 0) {
      (&UNK_142a95cb9)[uVar10] = 10;
      goto code_r0x00014085f7a9;
    }
    break;
  case 0xb:
  case 0x11:
  case 0x12:
  case 0x13:
    out(*unaff_RSI,uVar13);
    if (*(longlong *)(in_RDX + 5) == 0x746867696c5f6e72 && in_RCX == *(int **)in_RDX) {
      (&UNK_142a95cb9)[uVar10] = 6;
      goto code_r0x00014085f7a9;
    }
    break;
  case 0xc:
    puVar1 = (uint *)(&UNK_142a95c41 + uVar10);
    *puVar1 = *puVar1 >> 0x10 | *puVar1 << 0x10;
    pcVar14 = unaff_RSI + 1;
    if (iVar9 == 0) {
      *pcVar14 = '\x0e';
      goto code_r0x00014085f7a9;
    }
    goto code_r0x00014085f7a6;
  case 0xd:
    if (in_RDX[4] == 0x73 && iVar9 == 0) {
      (&UNK_142a95cb9)[uVar10] = 0xb;
      goto code_r0x00014085f7a9;
    }
    break;
  case 0xe:
    pcVar11 = unaff_RSI;
    if ((uVar7 | 2) == 0) {
      unaff_RSI[1] = '\x04';
      goto code_r0x00014085f7a9;
    }
    break;
  case 0xf:
    if (bVar15) {
      UNRECOVERED_JUMPTABLE_00 = (code *)swi(3);
      (*UNRECOVERED_JUMPTABLE_00)();
      return;
    }
    bVar12 = in(uVar13);
    *unaff_RDI = bVar12;
    pcVar11 = (char *)0x735f6d69;
    if (*(undefined **)(in_RDX + unaff_GS_OFFSET + 6) == &DAT_142a95cb8 && in_RCX == (int *)0x0) {
      uRam00000000735f6d6a = 0xc;
      goto code_r0x00014085f7a9;
    }
    break;
  case 0x10:
    *pcVar11 = *pcVar11 + cVar8;
    (&UNK_142a95c71)[uVar10] = (&UNK_142a95c71)[uVar10] + bVar12;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x14:
    pcVar11 = unaff_RSI;
    if ((uVar7 | 2) == 0) {
      unaff_RSI[1] = '\t';
      goto code_r0x00014085f7a9;
    }
    break;
  case 0x15:
    pcVar11 = unaff_RSI;
    if ((uVar7 | 2) == 0) {
      unaff_RSI[1] = '\x05';
      goto code_r0x00014085f7a9;
    }
    break;
  case 0x16:
    core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
    return;
  default:
    if (uVar10 >= 0xfffffffebd56a348) {
      if (in_R9 != 0 || iVar9 != *(int *)(in_RDX + 7)) goto LAB_140860dff;
      (&UNK_142a95cb9)[uVar10] = 1;
    }
    goto code_r0x000140860df5;
  case 0x1d:
    LOCK();
    *(int *)(CONCAT71((int7)((ulonglong)in_RCX >> 8),bVar12 | *unaff_RDI) + 0x4c000000) =
         (int)unaff_RBX;
    UNLOCK();
    UNRECOVERED_JUMPTABLE_00 = (code *)(*(int *)(in_R9 + 0x10aa57440) + in_R9);
                    // WARNING: Could not recover jumptable at 0x000140860d74. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)(UNRECOVERED_JUMPTABLE_00);
    return;
  case 0x1e:
    out(*(undefined4 *)unaff_RSI,uVar13);
    out(unaff_RSI[4],uVar13);
    if (in_RDX[4] != 0x65 || iVar9 != *(int *)in_RDX) {
LAB_140860dff:
      serde::de::Error::unknown_variant<serde_json::error::Error>();
      *(undefined8 *)(&UNK_142a95cc0 + uVar10) = extraout_RAX_02;
      cVar8 = '\x01';
      goto LAB_140860df7;
    }
    (&UNK_142a95cb9)[uVar10] = 0;
code_r0x000140860df5:
    cVar8 = '\0';
LAB_140860df7:
    *pcVar11 = cVar8;
    return;
  case 0x20:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x21:
    goto LAB_1408614ba;
  case 0x22:
    UNRECOVERED_JUMPTABLE_00 = (code *)swi(3);
    (*UNRECOVERED_JUMPTABLE_00)();
    return;
  case 0x23:
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar6 = *(longlong *)(unaff_RDI + 0x10);
    *(undefined1 *)(*(longlong *)(unaff_RDI + 8) + lVar6) = 0x7b;
    *(longlong *)(unaff_RDI + 0x10) = lVar6 + 1;
    cStack_c8 = '\x01';
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::history::wall_compound_edit::_::impl_0::serialize::__SerializeWith>
              ();
    if (extraout_RAX_03 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_04 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<country_core::systems::wall::valid_enclosure::IsEnclosed>_>
              ();
    if (extraout_RAX_05 != 0) {
      return;
    }
    if (((unaff_XMM7_Da != 0.0) || (NAN(unaff_XMM7_Da))) &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                  (), extraout_RAX_12 != 0)) {
      return;
    }
    if (((*(float *)(unaff_RSI + 0x5c) != 2.8) || (NAN(*(float *)(unaff_RSI + 0x5c)))) &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                  (), extraout_RAX_13 != 0)) {
      return;
    }
    if ((unaff_R12B != '\0') &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::inner_wall_state::ArchType>
                  (), extraout_RAX_06 != 0)) {
      return;
    }
    if ((unaff_R15B != '\0') &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::inner_wall_state::WallStyle>
                  (), extraout_RAX_07 != 0)) {
      return;
    }
    if ((unaff_R14B != '\0') &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::inner_wall_state::StructureType>
                  (), extraout_RAX_08 != 0)) {
      return;
    }
    if ((unaff_RSI[0x69] != '\0') &&
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::decorator::ClutterVersion>
                  (), extraout_RAX_09 != 0)) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::inner_wall_state::ArchType>
              ();
    if (extraout_RAX_10 != 0) {
      return;
    }
    if ((unaff_XMM6_Da == 0.0) && (!NAN(unaff_XMM6_Da))) goto LAB_140861412;
LAB_1408614ba:
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>
              ();
    if (extraout_RAX_14 != 0) {
      return;
    }
LAB_140861412:
    if ((((char)unaff_RBX == '\0') ||
        (serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,bool>
                   (), extraout_RAX_11 == 0)) && (cStack_c8 != '\0')) {
      alloc::vec::Vec::extend_from_slice<u8,alloc::alloc::Global>();
    }
    return;
  case 0x24:
  case 0x29:
  case 0x2a:
  case 0x2d:
    if (in_RDX == (byte *)0x6e && in_RCX == (int *)0x0) {
      (&UNK_142a95cb9)[uVar10] = 3;
      goto code_r0x00014086174c;
    }
    goto LAB_140861745;
  case 0x25:
    *in_RCX = *in_RCX + (int)in_R9;
    pcVar11 = unaff_RSI;
code_r0x000140861628:
    pcVar11[1] = '\t';
    goto code_r0x00014086174c;
  case 0x26:
    if (in_R9 == 0 && in_RCX == (int *)0x0) {
      (&UNK_142a95cb9)[uVar10] = 7;
      goto code_r0x00014086174c;
    }
    iVar9 = memcmp(in_RDX,&anon_fd26506bd31d5e4909a9375e3935cf05_80_llvm_1874562925060143905,
                   0x142a95cb8);
    if (iVar9 == 0) {
      (&UNK_142a95cb9)[uVar10] = 10;
      goto code_r0x00014086174c;
    }
    goto LAB_140861745;
  case 0x27:
    UNRECOVERED_JUMPTABLE_00 = (code *)(*(int *)(in_R9 + (longlong)in_RCX * 4) + in_R9);
                    // WARNING: Could not recover jumptable at 0x000140861546. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)(UNRECOVERED_JUMPTABLE_00);
    return;
  case 0x28:
    if ((int)in_RDX == 0 && iVar9 == 0) {
      (&UNK_142a95cb9)[uVar10] = 1;
      goto code_r0x00014086174c;
    }
    goto LAB_140861745;
  case 0x2b:
    *unaff_RDI = *unaff_RDI + bVar12;
    *unaff_RBX = *unaff_RBX + iVar9;
    *(undefined1 *)in_RCX = 0xb;
    goto code_r0x00014086174c;
  case 0x2c:
    *pcVar11 = *pcVar11 + cVar8;
    iVar9 = memcmp(in_RDX,&anon_fd26506bd31d5e4909a9375e3935cf05_33_llvm_1874562925060143905,
                   0x142a95cb8);
    if (iVar9 == 0) goto code_r0x000140861628;
LAB_140861745:
    (&UNK_142a95cb9)[uVar10] = 0xc;
code_r0x00014086174c:
    *pcVar11 = '\0';
    return;
  case 0x2e:
    pcVar11 = (char *)CONCAT71((int7)((ulonglong)pcVar11 >> 8),cVar8 + in_stack_fffffffff7840f48);
    *pcVar11 = *pcVar11 + cVar8 + in_stack_fffffffff7840f48;
    pbVar2 = (byte *)(pcVar11 + -0x7d);
    bVar3 = *pbVar2;
    *pbVar2 = *pbVar2 + bVar12;
    pcVar11[-0x77] = pcVar11[-0x77] + bVar12 + CARRY1(bVar3,bVar12);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2f:
  case 0x30:
  case 0x31:
    if (!bVar15) {
      bVar12 = in(uVar13);
      *unaff_RDI = bVar12;
      unaff_RDI = unaff_RDI + 1;
    }
    bVar12 = in(uVar13);
    *unaff_RDI = bVar12;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  pcVar14 = pcVar11 + 1;
code_r0x00014085f7a6:
  *pcVar14 = '\x0f';
code_r0x00014085f7a9:
  *pcVar11 = '\0';
  return;
}

