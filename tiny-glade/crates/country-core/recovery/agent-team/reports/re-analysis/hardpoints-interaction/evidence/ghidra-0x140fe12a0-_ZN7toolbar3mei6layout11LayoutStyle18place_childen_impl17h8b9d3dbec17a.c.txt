
// WARNING: Instruction at (ram,0x000140fe2a00) overlaps instruction at (ram,0x000140fe29fe)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x000140fe299b)
// WARNING: Removing unreachable block (ram,0x000140fe2a09)
// WARNING: Removing unreachable block (ram,0x000140fe2a68)
// WARNING: Removing unreachable block (ram,0x000140fe2a72)
// WARNING: Removing unreachable block (ram,0x000140fe299f)
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>(void)

{
  uint *puVar1;
  int iVar2;
  code *pcVar3;
  float fVar4;
  byte bVar5;
  char cVar6;
  longlong extraout_RAX;
  ulonglong uVar8;
  char *pcVar9;
  undefined7 uVar10;
  code *UNRECOVERED_JUMPTABLE;
  code *UNRECOVERED_JUMPTABLE_01;
  char *pcVar11;
  undefined *puVar12;
  longlong *in_RDX;
  longlong *plVar13;
  byte bVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  ulonglong uVar17;
  undefined8 *puVar18;
  float *in_R8;
  float *pfVar19;
  char *in_R9;
  char *pcVar20;
  longlong lVar21;
  longlong *plVar22;
  bool bVar23;
  bool bVar24;
  float fVar25;
  undefined1 auVar26 [12];
  undefined1 auVar27 [16];
  float fVar28;
  float fVar29;
  float fVar30;
  float unaff_XMM6_Da;
  float fVar31;
  undefined4 unaff_XMM6_Dd;
  ulonglong unaff_XMM7_Qa;
  float unaff_XMM8_Da;
  float fVar32;
  undefined4 unaff_XMM10_Db;
  undefined1 unaff_XMM10_Bi;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  longlong *in_stack_00000028;
  undefined1 auStack_84d8 [33608];
  undefined8 *puStack_190;
  code *apcStack_178 [2];
  undefined1 auStack_168 [32];
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  code *local_120;
  longlong *local_118;
  longlong local_110;
  char *local_108;
  float fStack_fc;
  float fStack_f8;
  float local_f4;
  undefined8 local_f0;
  undefined1 local_e8 [12];
  undefined2 uStack_dc;
  undefined2 uStack_da;
  undefined2 local_d8;
  undefined6 uStack_d6;
  undefined4 uStack_a4;
  undefined1 uStack_a0;
  uint uVar7;
  
  puVar18 = (undefined8 *)auStack_168;
  puVar16 = (undefined8 *)auStack_168;
  UNRECOVERED_JUMPTABLE = (code *)local_e8;
  local_d8 = (undefined2)unaff_XMM7_Qa;
  uStack_d6 = (undefined6)(unaff_XMM7_Qa >> 0x10);
  uStack_dc = (undefined2)unaff_XMM6_Dd;
  uStack_da = (undefined2)((uint)unaff_XMM6_Dd >> 0x10);
  local_f0 = 0xfffffffffffffffe;
  plVar22 = (longlong *)((longlong)in_stack_00000028 << 4);
  lVar21 = 0;
  if (((ulonglong)in_stack_00000028 >> 0x3c != 0) || ((longlong *)0x7ffffffffffffffc < plVar22)) {
LAB_140fe189f:
                    // WARNING: Subroutine does not return
    apcStack_178[1] = (code *)&UNK_140fe18b1;
    alloc::raw_vec::handle_error();
  }
  pfVar19 = in_R8;
  pcVar20 = in_R9;
  uStack_a4 = unaff_XMM10_Db;
  uStack_a0 = unaff_XMM10_Bi;
  if (plVar22 == (longlong *)0x0) {
    local_110 = 4;
    local_118 = (longlong *)0x0;
    plVar13 = in_RDX;
  }
  else {
    lVar21 = 4;
    plVar13 = (longlong *)&DAT_00000004;
    apcStack_178[1] = (code *)0x140fe1355;
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_110 = extraout_RAX;
    local_118 = in_stack_00000028;
    if (extraout_RAX == 0) goto LAB_140fe189f;
  }
  local_108 = (char *)0x0;
  fVar28 = *in_R8;
  fVar32 = *(float *)((longlong)in_RDX + 4);
  if ((char)*in_RDX == '\0') {
    fVar35 = in_R8[1];
    fVar37 = *(float *)((longlong)in_RDX + 0xc);
    if ((char)in_RDX[1] != '\0') goto LAB_140fe152f;
LAB_140fe13a4:
    fVar33 = *(float *)((longlong)in_RDX + 0x14);
    if ((char)in_RDX[2] != '\0') goto LAB_140fe1543;
LAB_140fe13b3:
    fVar25 = *(float *)((longlong)in_RDX + 0x1c);
    cVar6 = (char)in_RDX[3];
  }
  else {
    fVar32 = fVar32 * fVar28;
    fVar35 = in_R8[1];
    fVar37 = *(float *)((longlong)in_RDX + 0xc);
    if ((char)in_RDX[1] == '\0') goto LAB_140fe13a4;
LAB_140fe152f:
    fVar37 = fVar37 * fVar35;
    fVar33 = *(float *)((longlong)in_RDX + 0x14);
    if ((char)in_RDX[2] == '\0') goto LAB_140fe13b3;
LAB_140fe1543:
    fVar33 = fVar33 * fVar28;
    fVar25 = *(float *)((longlong)in_RDX + 0x1c);
    cVar6 = (char)in_RDX[3];
  }
  if (cVar6 != '\0') {
    fVar25 = fVar25 * fVar35;
  }
  local_f4 = fVar28 - (fVar33 + fVar32);
  fVar36 = *(float *)((longlong)in_RDX + 0x24);
  if ((char)in_RDX[4] != '\0') {
    fVar36 = fVar36 * local_f4;
  }
  pcVar9 = (char *)0x0;
  if (in_stack_00000028 != (longlong *)0x0) {
    pcVar9 = (char *)((longlong)in_stack_00000028 + -1);
  }
  local_f4 = local_f4 - (float)(longlong)pcVar9 * fVar36;
  auVar26 = ZEXT812(0);
  fVar29 = 0.0;
  fVar34 = 0.0;
  if (in_stack_00000028 != (longlong *)0x0) {
    pcVar11 = in_R9;
    do {
      fVar30 = *(float *)(*(char **)(pcVar11 + 8) + 4);
      if (**(char **)(pcVar11 + 8) != '\0') {
        fVar30 = fVar30 * local_f4;
      }
      plVar13 = *(longlong **)pcVar11;
      fVar31 = *(float *)((longlong)plVar13 + 0x2c);
      if ((char)plVar13[5] != '\0') {
        fVar31 = fVar31 * local_f4;
      }
      fVar34 = fVar34 + *(float *)(plVar13 + 7);
      fVar4 = fVar31;
      if (fVar31 <= fVar30) {
        fVar4 = fVar30;
      }
      fVar31 = (float)(~-(uint)NAN(fVar30) & (uint)fVar4 | -(uint)NAN(fVar30) & (uint)fVar31);
      fVar30 = *(float *)(pcVar11 + 0x10);
      unaff_XMM6_Da = fVar30;
      if (fVar30 <= fVar31) {
        unaff_XMM6_Da = fVar31;
      }
      unaff_XMM8_Da = (float)(~-(uint)NAN(fVar31) & (uint)unaff_XMM6_Da);
      fVar30 = (float)(-(uint)NAN(fVar31) & (uint)fVar30 | (uint)unaff_XMM8_Da);
      unaff_XMM7_Qa = (ulonglong)(uint)fVar30;
      fVar29 = fVar29 + fVar30;
      pcVar11 = pcVar11 + 0x18;
    } while (pcVar11 != in_R9 + (longlong)in_stack_00000028 * 0x18);
  }
  fVar33 = (fVar28 - ((float)(longlong)pcVar9 * fVar36 + fVar29)) - (fVar33 + fVar32);
  fVar28 = 1.0 - fVar34;
  if (0.0 <= fVar28) {
    auVar26._4_8_ = 0;
    auVar26._0_4_ = fVar28;
  }
  auVar27._12_4_ = 0;
  auVar27._0_12_ = auVar26;
  fVar29 = 1.0;
  if (auVar26._0_4_ <= 1.0) {
    fVar29 = auVar26._0_4_;
  }
  fVar29 = fVar29 * fVar33;
  puVar12 = &DAT_142bc223c;
  uVar8 = (ulonglong)*(int *)(&DAT_142bc223c + (ulonglong)*(byte *)((longlong)in_RDX + 0x46) * 4);
  bVar23 = 0xfffffffebd43ddc3 < uVar8;
  pcVar9 = &DAT_142bc223c + uVar8;
  bVar24 = pcVar9 == (char *)0x0;
  fVar30 = (float)unaff_XMM7_Qa;
  bVar5 = (byte)pcVar9;
  uVar7 = (uint)pcVar9;
  uVar10 = (undefined7)((ulonglong)pcVar9 >> 8);
  bVar14 = (byte)in_RDX;
  local_120 = UNRECOVERED_JUMPTABLE_01;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)((longlong)in_RDX + 0x46)) {
  case 0:
    goto code_r0x000140fe1569;
  case 1:
code_r0x000140fe155c:
  case 2:
    goto code_r0x000140fe1569;
  case 3:
    if (in_stack_00000028 < (longlong *)0x2) goto code_r0x000140fe155c;
    break;
  case 4:
    break;
  case 5:
    goto code_r0x000140fe1713;
  case 6:
    *pcVar9 = *pcVar9 + bVar5;
    unaff_XMM8_Da = (fVar35 - fVar30) * 0.5;
    fVar35 = *(float *)(in_R9 + 0x10 + (longlong)in_stack_00000028);
    unaff_XMM6_Da = *(float *)(&UNK_142bc2274 + uVar8);
    if (in_RDX == local_118) goto code_r0x000140fe17d2;
    goto code_r0x000140fe15f0;
  case 7:
    *in_R9 = *in_R9 + '<';
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 8:
code_r0x000140fe17d2:
    apcStack_178[1] = (code *)&UNK_140fe17d7;
    alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
code_r0x000140fe15f0:
    fVar28 = fVar33;
    if (fVar33 <= fVar37) {
      fVar28 = fVar37;
    }
    fVar28 = (float)(~-(uint)NAN(fVar37) & (uint)fVar28 | -(uint)NAN(fVar37) & (uint)fVar33);
    fVar37 = fVar35;
    if (fVar35 <= fVar28) {
      fVar37 = fVar28;
    }
    *(float *)(local_110 + -8 + lVar21) =
         (unaff_XMM6_Da / fVar34) * fStack_fc +
         (float)(-(uint)NAN(fVar28) & (uint)fVar35 | ~-(uint)NAN(fVar28) & (uint)fVar37);
    *(float *)(local_110 + -4 + lVar21) = fVar30;
    *(ulonglong *)(local_110 + lVar21) =
         CONCAT44(unaff_XMM8_Da + fVar36 + uStack_148._4_4_ + uStack_138._4_4_,
                  fVar32 + (float)uStack_148 + (float)uStack_138);
    local_108 = (char *)((longlong)in_RDX + 1);
    if (plVar22 == in_stack_00000028 + 3) goto code_r0x000140fe1842;
    goto code_r0x000140fe1713;
  case 9:
    goto code_r0x000140fe1be1;
  case 10:
    cVar6 = bVar5 + 0x80 + bVar23;
    if ((POPCOUNT(cVar6) & 1U) == 0) {
      *in_R9 = *in_R9 + '<';
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    if ((POPCOUNT((cVar6 + -0x80) - (0x7f < bVar5 || CARRY1(bVar5 + 0x80,bVar23))) & 1U) == 0) {
      *in_R9 = *in_R9 + '<';
      halt_baddata();
    }
    uVar15 = (ulonglong)(uint)((int)in_RDX + (int)UNRECOVERED_JUMPTABLE_01);
    goto joined_r0x000140fe1cbf;
  case 0xb:
    uVar15 = CONCAT71((int7)((ulonglong)in_RDX >> 8),bVar14 >> 1);
    (&UNK_142bc21c5)[uVar8] = ((&UNK_142bc21c5)[uVar8] + -0x3c) - (bVar14 & 1);
    puVar1 = (uint *)(&UNK_142bc220d + uVar8);
    *puVar1 = *puVar1 >> 9 | *puVar1 << 0x17;
    fVar28 = (float)((ulonglong)(uVar7 & 1) | 0x142bc223c);
    local_f4 = fVar36 + fVar29 / (fVar28 + fVar28);
joined_r0x000140fe1cbf:
    if (in_stack_00000028 == (longlong *)0x0) {
code_r0x000140fe1f31:
      *(char **)(local_120 + 0x10) = local_108;
      *(longlong **)local_120 = local_118;
      *(longlong *)(local_120 + 8) = local_110;
      return;
    }
    fStack_fc = fVar35 - (fVar25 + fVar33);
    uStack_148 = *(undefined8 *)(in_R8 + 2);
    uStack_140 = 0;
    uStack_138 = *(undefined8 *)(uVar15 + 0x3c);
    uStack_130 = 0;
    UNRECOVERED_JUMPTABLE_01 =
         (code *)(&UNK_142bc2274 +
                 *(int *)(&UNK_142bc2274 + (ulonglong)*(byte *)(uVar15 + 0x45) * 4));
    fStack_f8 = fVar33;
code_r0x000140fe1e26:
                    // WARNING: Could not recover jumptable at 0x000140fe1e5c. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_01)();
    return;
  case 0xc:
    auVar27 = maxps(auVar27,ZEXT416((uint)fVar28));
    if (auVar27._0_4_ <= fVar29) {
      fVar29 = auVar27._0_4_;
    }
    fVar29 = fVar29 * fVar37;
    puVar12 = &DAT_142bc2260;
    pcVar9 = (char *)(longlong)
                     *(int *)(&DAT_142bc2260 + (ulonglong)*(byte *)((longlong)in_RDX + 0x46) * 4);
    goto code_r0x000140fe1be1;
  case 0xd:
                    // WARNING: Bad instruction - Truncating control flow here
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    halt_baddata();
  case 0xe:
    goto code_r0x000140fe1e26;
  case 0xf:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x10:
    goto code_r0x000140fe1e26;
  case 0x11:
    fVar28 = *(float *)(in_R9 + 0x14 + (longlong)in_stack_00000028);
    fVar35 = *(float *)(&UNK_142bc2274 + uVar8);
    if (in_RDX == local_118) {
      apcStack_178[1] = (code *)&UNK_140fe1f06;
      alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>();
    }
    fVar25 = fVar33;
    if (fVar33 <= fVar37) {
      fVar25 = fVar37;
    }
    fVar37 = (float)(~-(uint)NAN(fVar37) & (uint)fVar25 | -(uint)NAN(fVar37) & (uint)fVar33);
    fVar33 = fVar28;
    if (fVar28 <= fVar37) {
      fVar33 = fVar37;
    }
    *(float *)(local_110 + -8 + lVar21) = unaff_XMM6_Da;
    *(float *)(local_110 + -4 + lVar21) =
         (fVar35 / fVar34) * fVar36 +
         (float)(-(uint)NAN(fVar37) & (uint)fVar28 | ~-(uint)NAN(fVar37) & (uint)fVar33);
    *(ulonglong *)(local_110 + lVar21) =
         CONCAT44(fVar32 + uStack_148._4_4_ + uStack_138._4_4_,
                  fVar30 * 0.5 + fStack_f8 + (float)uStack_148 + (float)uStack_138);
    local_108 = (char *)((longlong)in_RDX + 1);
    if (plVar22 != in_stack_00000028 + 3) goto code_r0x000140fe1e26;
    goto code_r0x000140fe1f31;
  case 0x12:
    if (!bVar24) {
      return;
    }
    goto code_r0x000140fe28e1;
  default:
    if (bVar24) {
      return;
    }
    if (bVar23) {
      iVar2 = CONCAT22(local_d8,uStack_da) << 6;
      uStack_da = (undefined2)iVar2;
      local_d8 = (undefined2)((uint)iVar2 >> 0x10);
                    // WARNING (jumptable): Read-only address (ram,0x000142bc223c) is written
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
      _DAT_142bc223c = 0x89480432;
      puStack_190 = apcStack_178 + 1;
      puVar16 = apcStack_178 + 1;
      cVar6 = '\x03';
      pcVar3 = UNRECOVERED_JUMPTABLE;
      do {
        pcVar3 = pcVar3 + -8;
        puVar16 = puVar16 + -1;
        *puVar16 = *(undefined8 *)pcVar3;
        cVar6 = cVar6 + -1;
      } while ('\0' < cVar6);
      puVar18 = (undefined8 *)auStack_84d8;
      UNRECOVERED_JUMPTABLE_01[-0x3d] = (code)((byte)UNRECOVERED_JUMPTABLE_01[-0x3d] ^ bVar14);
      apcStack_178[1] = UNRECOVERED_JUMPTABLE;
    }
    else if ((int)pcVar20 == 0 && (int)CONCAT71(uVar10,bVar5 + in_R9[0x142bc21f2] + 'A') == 0) {
      uRam0000000042bc2248 = 0x32;
      return;
    }
    goto LAB_140fe29b5;
  case 0x14:
    return;
  case 0x15:
    puVar16 = apcStack_178 + 1;
    apcStack_178[1] = UNRECOVERED_JUMPTABLE_01;
code_r0x000140fe28e1:
    puVar18 = puVar16 + 0xfffffffffffffffa;
    if ((longlong)pfVar19 - 3U < 10) {
      UNRECOVERED_JUMPTABLE =
           (code *)(&DAT_142bc2284 + *(int *)(&DAT_142bc2284 + ((longlong)pfVar19 - 3U) * 4));
code_r0x000140fe2901:
                    // WARNING: Could not recover jumptable at 0x000140fe2901. Too many branches
                    // WARNING: Treating indirect jump as call
      (*UNRECOVERED_JUMPTABLE)();
      return;
    }
LAB_140fe29b5:
    *(undefined8 *)((longlong)puVar18 + 0x20) = 5;
    *(undefined8 *)((longlong)puVar18 + -8) = 0x140fe29cd;
    ron::error::impl$5::unknown_variant();
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    return;
  case 0x1a:
    if (!bVar24) {
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
                    // WARNING: Ignoring partial resolution of indirect
      _DAT_142bc223c = 0x432;
      _DAT_142bc223c = _DAT_142bc223c;
      return;
    }
                    // WARNING (jumptable): Read-only address (ram,0x000142bc223c) is written
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    _DAT_142bc223c = 0x65eb0132;
    puVar18 = (undefined8 *)auStack_168;
    if ((int)plVar13[1] == 0x6e656577 && *plVar13 == 0x7465426563617053) {
      _DAT_142bc223c = 0x65eb0332;
      return;
    }
    goto LAB_140fe29b5;
  case 0x1b:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar7 + 0x42bc223c);
    goto code_r0x000140fe2901;
  case 0x1c:
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    LOCK();
    UNLOCK();
                    // WARNING: Could not recover jumptable at 0x000140fe2a01. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(pcVar20 + *(int *)(pcVar20 + CONCAT71(uVar10,bVar5 + 0xf) * 4)))(0x42bc22ac);
    return;
  case 0x1d:
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    if ((int)pcVar20 == 0 && uVar7 == 0) {
      uRam0000000042bc2248 = 0x132;
      return;
    }
    apcStack_178[1] = (code *)0x140fe2a98;
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    ron::error::impl$5::unknown_variant();
    return;
  case 0x1e:
    uVar17 = (ulonglong)CONCAT13(uStack_a0,uStack_a4._1_3_);
    puVar16 = (undefined8 *)(uVar17 - 8);
    *(code **)(uVar17 - 8) = UNRECOVERED_JUMPTABLE;
    cVar6 = '\x17';
    do {
      UNRECOVERED_JUMPTABLE = UNRECOVERED_JUMPTABLE + -8;
      puVar16 = puVar16 + -1;
      *puVar16 = *(undefined8 *)UNRECOVERED_JUMPTABLE;
      cVar6 = cVar6 + -1;
    } while ('\0' < cVar6);
    *(ulonglong *)(uVar17 - 200) = uVar17 - 8;
    *(longlong **)(uVar17 - 0xe1cf) = in_RDX;
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    return;
  case 0x1f:
                    // WARNING: Read-only address (ram,0x000142bc223c) is written
    return;
  case 0x20:
    return;
  }
joined_r0x000140fe150d:
  if (in_stack_00000028 == (longlong *)0x0) {
code_r0x000140fe1842:
    *(char **)(local_120 + 0x10) = local_108;
    *(longlong **)local_120 = local_118;
    *(longlong *)(local_120 + 8) = local_110;
    return;
  }
  fStack_f8 = fVar35 - (fVar25 + fVar37);
  fStack_fc = fVar33;
  if (fVar33 <= 0.0) {
    fStack_fc = 0.0;
  }
  uStack_148 = *(undefined8 *)(in_R8 + 2);
  uStack_140 = 0;
  uStack_138 = *(undefined8 *)((longlong)in_RDX + 0x3c);
  uStack_130 = 0;
  UNRECOVERED_JUMPTABLE_01 =
       (code *)(&UNK_142bc2250 +
               *(int *)(&UNK_142bc2250 + (ulonglong)*(byte *)((longlong)in_RDX + 0x45) * 4));
code_r0x000140fe1713:
                    // WARNING: Could not recover jumptable at 0x000140fe1725. Too many branches
                    // WARNING: Treating indirect jump as call
  (*UNRECOVERED_JUMPTABLE_01)();
  return;
code_r0x000140fe1569:
  goto joined_r0x000140fe150d;
code_r0x000140fe1be1:
                    // WARNING: Could not recover jumptable at 0x000140fe1be8. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(pcVar9 + (longlong)puVar12))(puVar12,fVar29);
  return;
}

