
// WARNING: Instruction at (ram,0x000140fe29d4) overlaps instruction at (ram,0x000140fe29d3)
// 
// WARNING: Removing unreachable block (ram,0x000140fe2a3c)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>(void)

{
  uint *puVar1;
  code *pcVar2;
  float fVar3;
  char cVar4;
  longlong extraout_RAX;
  longlong lVar5;
  longlong lVar6;
  char *pcVar7;
  code *UNRECOVERED_JUMPTABLE;
  longlong *plVar8;
  undefined8 *puVar9;
  undefined2 *puVar10;
  longlong *in_RDX;
  longlong *plVar11;
  float *in_R8;
  uint uVar12;
  undefined8 *in_R9;
  undefined8 *puVar13;
  longlong lVar14;
  longlong *plVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float unaff_XMM6_Da;
  float fVar19;
  float unaff_XMM7_Da;
  float unaff_XMM8_Da;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  longlong *in_stack_00000028;
  float fStack_148;
  float fStack_144;
  float fStack_138;
  float fStack_134;
  char *local_108;
  float fStack_f8;
  
  plVar15 = (longlong *)((longlong)in_stack_00000028 << 4);
  lVar14 = 0;
  if (((ulonglong)in_stack_00000028 >> 0x3c != 0) || ((longlong *)0x7ffffffffffffffc < plVar15)) {
LAB_140fe1f8e:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  puVar13 = in_R9;
  if (plVar15 == (longlong *)0x0) {
    lVar5 = 4;
    plVar8 = (longlong *)0x0;
    plVar11 = in_RDX;
  }
  else {
    lVar14 = 4;
    plVar11 = (longlong *)&DAT_00000004;
    std::alloc::__default_lib_allocator::__rust_alloc();
    lVar5 = extraout_RAX;
    plVar8 = in_stack_00000028;
    if (extraout_RAX == 0) goto LAB_140fe1f8e;
  }
  uVar12 = (uint)puVar13;
  local_108 = (char *)0x0;
  fVar24 = *in_R8;
  fVar22 = *(float *)((longlong)in_RDX + 4);
  if ((char)*in_RDX == '\0') {
    fVar26 = in_R8[1];
    fVar20 = *(float *)((longlong)in_RDX + 0xc);
    if ((char)in_RDX[1] == '\0') goto LAB_140fe1aa4;
LAB_140fe1c34:
    fVar20 = fVar20 * fVar26;
    if ((char)in_RDX[2] != '\0') goto LAB_140fe1c48;
LAB_140fe1ab3:
    fVar16 = *(float *)((longlong)in_RDX + 0x1c);
    cVar4 = (char)in_RDX[3];
  }
  else {
    fVar22 = fVar22 * fVar24;
    fVar26 = in_R8[1];
    fVar20 = *(float *)((longlong)in_RDX + 0xc);
    if ((char)in_RDX[1] != '\0') goto LAB_140fe1c34;
LAB_140fe1aa4:
    if ((char)in_RDX[2] == '\0') goto LAB_140fe1ab3;
LAB_140fe1c48:
    fVar16 = *(float *)((longlong)in_RDX + 0x1c);
    cVar4 = (char)in_RDX[3];
  }
  if (cVar4 != '\0') {
    fVar16 = fVar16 * fVar26;
  }
  fVar21 = fVar26 - (fVar16 + fVar20);
  fVar25 = *(float *)((longlong)in_RDX + 0x24);
  if ((char)in_RDX[4] != '\0') {
    fVar25 = fVar25 * fVar21;
  }
  pcVar7 = (char *)0x0;
  if (in_stack_00000028 != (longlong *)0x0) {
    pcVar7 = (char *)((longlong)in_stack_00000028 + -1);
  }
  fVar21 = fVar21 - (float)(longlong)pcVar7 * fVar25;
  fVar17 = 0.0;
  fVar23 = 0.0;
  if (in_stack_00000028 != (longlong *)0x0) {
    puVar9 = in_R9;
    do {
      fVar18 = *(float *)(puVar9[1] + 0xc);
      if (*(char *)(puVar9[1] + 8) != '\0') {
        fVar18 = fVar18 * fVar21;
      }
      plVar11 = (longlong *)*puVar9;
      fVar19 = *(float *)((longlong)plVar11 + 0x34);
      if ((char)plVar11[6] != '\0') {
        fVar19 = fVar19 * fVar21;
      }
      fVar23 = fVar23 + *(float *)(plVar11 + 7);
      fVar3 = fVar19;
      if (fVar19 <= fVar18) {
        fVar3 = fVar18;
      }
      fVar19 = (float)(~-(uint)NAN(fVar18) & (uint)fVar3 | -(uint)NAN(fVar18) & (uint)fVar19);
      fVar18 = *(float *)((longlong)puVar9 + 0x14);
      unaff_XMM6_Da = fVar18;
      if (fVar18 <= fVar19) {
        unaff_XMM6_Da = fVar19;
      }
      unaff_XMM8_Da = (float)(~-(uint)NAN(fVar19) & (uint)unaff_XMM6_Da);
      unaff_XMM7_Da = (float)(-(uint)NAN(fVar19) & (uint)fVar18 | (uint)unaff_XMM8_Da);
      fVar17 = fVar17 + unaff_XMM7_Da;
      puVar9 = puVar9 + 3;
    } while (puVar9 != in_R9 + (longlong)in_stack_00000028 * 3);
  }
  fVar26 = (fVar26 - ((float)(longlong)pcVar7 * fVar25 + fVar17)) - (fVar16 + fVar20);
  puVar10 = (undefined2 *)&DAT_142bc2260;
  lVar6 = (longlong)*(int *)(&DAT_142bc2260 + (ulonglong)*(byte *)((longlong)in_RDX + 0x46) * 4);
  pcVar7 = &DAT_142bc2260 + lVar6;
  cVar4 = (char)pcVar7;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Read-only address (ram,0x000142bc2262) is written
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)((longlong)in_RDX + 0x46)) {
  case 0:
    goto code_r0x000140fe1c05;
  case 1:
code_r0x000140fe1c61:
    goto joined_r0x000140fe1c77;
  case 2:
    goto code_r0x000140fe1cb1;
  case 3:
    if (in_stack_00000028 < (longlong *)0x2) goto code_r0x000140fe1c61;
    goto code_r0x000140fe1c05;
  case 4:
    goto code_r0x000140fe1cb1;
  case 5:
    goto code_r0x000140fe1e44;
  case 6:
    *pcVar7 = *pcVar7 + cVar4;
    *pcVar7 = *pcVar7 + cVar4;
    unaff_XMM7_Da = (fVar24 - unaff_XMM6_Da) * 0.5;
    fVar24 = *(float *)((longlong)in_R9 + 0x14 + (longlong)in_stack_00000028);
    unaff_XMM8_Da = *(float *)(&UNK_142bc2298 + lVar6);
    if (in_RDX == plVar8) goto code_r0x000140fe1f01;
    goto code_r0x000140fe1d40;
  case 7:
    goto code_r0x000140fe1ebc;
  case 8:
code_r0x000140fe1f01:
    alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
code_r0x000140fe1d40:
    fVar16 = fVar22;
    if (fVar22 <= fVar26) {
      fVar16 = fVar26;
    }
    fVar22 = (float)(~-(uint)NAN(fVar26) & (uint)fVar16 | -(uint)NAN(fVar26) & (uint)fVar22);
    fVar26 = fVar24;
    if (fVar24 <= fVar22) {
      fVar26 = fVar22;
    }
    *(float *)(lVar5 + -8 + lVar14) = unaff_XMM6_Da;
    *(float *)(lVar5 + -4 + lVar14) =
         (unaff_XMM8_Da / fVar23) * fVar25 +
         (float)(-(uint)NAN(fVar22) & (uint)fVar24 | ~-(uint)NAN(fVar22) & (uint)fVar26);
    *(ulonglong *)(lVar5 + lVar14) =
         CONCAT44(fVar20 + fStack_144 + fStack_134,
                  unaff_XMM7_Da + fStack_f8 + fStack_148 + fStack_138);
    local_108 = (char *)((longlong)in_RDX + 1);
    plVar11 = in_stack_00000028 + 3;
    if (plVar15 == plVar11) goto code_r0x000140fe1f31;
    goto code_r0x000140fe1dd7;
  case 9:
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  default:
    if (*(longlong *)((longlong)plVar11 + 3) == 0x796c6e6576456563 && pcVar7 == (char *)*plVar11) {
                    // WARNING (jumptable): Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
      _DAT_142bc2260 = CONCAT22(_UNK_142bc2262,0x432);
      return;
    }
    break;
  case 0xb:
    if ((int)plVar11[1] == 0x6e656577 && *plVar11 == 0x7465426563617053) goto code_r0x000140fe2964;
    break;
  case 0xc:
    if (*(char *)((longlong)plVar11 + 2) == 'd' && (short)pcVar7 == 0x6e45) {
                    // WARNING (jumptable): Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
      _DAT_142bc2260 = CONCAT22(_UNK_142bc2262,0x232);
      return;
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
    }
    break;
  case 0x11:
code_r0x000140fe2964:
                    // WARNING (jumptable): Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
    _DAT_142bc2260 = CONCAT22(_UNK_142bc2262,0x332);
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
    return;
  case 0x12:
    return;
  case 0x13:
    uVar12 = uVar12 ^ 0x74;
    goto code_r0x000140fe2a13;
  case 0x14:
                    // WARNING (jumptable): Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
    _DAT_142bc2260 = 0x39eb0232;
    if (*(int *)((longlong)plVar11 + 3) == 0x68637465 && (int)*plVar11 == 0x65727453) {
      _DAT_142bc2260 = 0x39eb0032;
      return;
    }
    goto LAB_140fe2a80;
  case 0x15:
    break;
  case 0x16:
    UNRECOVERED_JUMPTABLE[-0x3d] = (code)((byte)UNRECOVERED_JUMPTABLE[-0x3d] ^ (byte)in_RDX);
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  case 0x17:
    puVar1 = (uint *)(lVar6 + 0x285784488);
    *puVar1 = *puVar1 | 0x53b8e0ff;
    if (*puVar1 == 0) {
      if ((short)puVar13 == 100 && (short)pcVar7 == 0) {
                    // WARNING (jumptable): Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
                    // WARNING: Read-only address (ram,0x000142bc2262) is written
        _DAT_142bc2260 = CONCAT22(_UNK_142bc2262,0x332);
        return;
      }
      goto LAB_140fe2a80;
    }
    pcVar7 = (char *)CONCAT71((int7)((ulonglong)pcVar7 >> 8),
                              cVar4 + *(char *)((longlong)in_R9 + 0x142bc2216) + 'A');
    puVar10 = (undefined2 *)0x42bc2214;
code_r0x000140fe2a13:
    if (uVar12 == 0 && (int)pcVar7 == 0) {
      *puVar10 = 0x132;
                    // WARNING: Read-only address (ram,0x000142bc2260) is written
      return;
    }
LAB_140fe2a80:
    ron::error::impl$5::unknown_variant();
    return;
  }
  ron::error::impl$5::unknown_variant();
  return;
code_r0x000140fe1cb1:
  goto joined_r0x000140fe1c77;
code_r0x000140fe1c05:
joined_r0x000140fe1c77:
  if (in_stack_00000028 == (longlong *)0x0) {
code_r0x000140fe1f31:
    *(char **)(UNRECOVERED_JUMPTABLE + 0x10) = local_108;
    *(longlong **)UNRECOVERED_JUMPTABLE = plVar8;
    *(longlong *)(UNRECOVERED_JUMPTABLE + 8) = lVar5;
    return;
  }
  plVar11 = (longlong *)0x0;
  UNRECOVERED_JUMPTABLE =
       (code *)(&UNK_142bc2274 +
               *(int *)(&UNK_142bc2274 + (ulonglong)*(byte *)((longlong)in_RDX + 0x45) * 4));
code_r0x000140fe1dd7:
  pcVar7 = *(char **)((longlong)(in_R9 + 1) + (longlong)plVar11);
  if (*pcVar7 == '\0') {
    if (pcVar7[8] == '\0') goto code_r0x000140fe1dfa;
code_r0x000140fe1e95:
    cVar4 = *(char *)(*(longlong *)((longlong)in_R9 + (longlong)plVar11) + 0x28);
  }
  else {
    if (pcVar7[8] != '\0') goto code_r0x000140fe1e95;
code_r0x000140fe1dfa:
    cVar4 = *(char *)(*(longlong *)((longlong)in_R9 + (longlong)plVar11) + 0x28);
  }
  if (cVar4 != '\0') {
code_r0x000140fe1ebc:
  }
code_r0x000140fe1e44:
                    // WARNING: Could not recover jumptable at 0x000140fe1e5c. Too many branches
                    // WARNING: Treating indirect jump as call
  (*UNRECOVERED_JUMPTABLE)();
  return;
}

