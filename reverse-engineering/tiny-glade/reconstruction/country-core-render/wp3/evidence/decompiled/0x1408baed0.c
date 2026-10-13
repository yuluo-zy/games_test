// address 0x1408baed0
// name nani_calculate_draw_list_params
// signature void __cdecl nani_calculate_draw_list_params(void)

// WARNING: Instruction at (ram,0x0001408c05d5) overlaps instruction at (ram,0x0001408c05d2)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x0001408c0605)
// WARNING: Removing unreachable block (ram,0x0001408c05ba)
// WARNING: Heritage AFTER dead removal. Example location: s0x00000010 : 0x0001408c0600
// WARNING: Restarted to delay deadcode elimination for space: stack

void __cdecl country_core::render::nani::buffers::nani_calculate_draw_list_params(void)

{
  char *pcVar1;
  code *pcVar2;
  byte bVar3;
  longlong *plVar4;
  char cVar5;
  byte bVar6;
  byte *pbVar7;
  undefined7 uVar9;
  ulonglong extraout_RAX;
  byte *pbVar8;
  void *_Dst;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong *in_RCX;
  undefined4 *puVar10;
  undefined8 uVar11;
  longlong *plVar12;
  undefined1 *puVar13;
  undefined **ppuVar14;
  ulonglong uVar15;
  undefined4 *puVar16;
  int iVar17;
  undefined8 *puVar18;
  undefined1 *puVar19;
  undefined1 *puVar20;
  ulonglong uVar21;
  undefined1 *puVar22;
  undefined1 *puVar23;
  ulonglong *puVar24;
  undefined1 *puVar25;
  undefined1 *puVar26;
  undefined1 *puVar27;
  ulonglong *puVar28;
  uint uVar29;
  uint uVar30;
  longlong *plVar31;
  longlong *plVar32;
  undefined4 uVar33;
  undefined *in_R8;
  undefined4 *puVar34;
  undefined4 *in_R9;
  undefined4 *in_R10;
  longlong *in_R11;
  int iVar35;
  uint uVar36;
  ulonglong uVar37;
  int iVar38;
  uint uVar39;
  undefined *puVar40;
  longlong lVar41;
  bool bVar42;
  char cStackX_f;
  longlong *plStackX_10;
  undefined1 *puStack_f8;
  ulonglong uStack_f0;
  undefined1 auStack_e8 [32];
  undefined *puStack_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  ulonglong local_b0;
  longlong *local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  ulonglong local_90;
  longlong *local_88;
  ulonglong local_80;
  longlong *local_78;
  ulonglong local_70;
  ulonglong local_68;
  undefined7 local_60;
  uint uStack_59;
  undefined4 uStack_55;
  byte bStack_51;
  uint local_50;
  int local_4c;
  undefined4 *local_48;
  
  puVar19 = auStack_e8;
  puVar20 = auStack_e8;
  puVar23 = auStack_e8;
  puVar22 = auStack_e8;
  puVar26 = auStack_e8;
  local_48 = (undefined4 *)0xfffffffffffffffe;
  local_70 = 0;
  local_68 = 4;
  local_60 = 0;
  uStack_59 = uStack_59 & 0xffffff00;
  local_88 = (longlong *)0x0;
  local_80 = 4;
  local_78 = (longlong *)0x0;
  local_a0 = 0;
  local_98 = 4;
  local_90 = 0;
  local_b8 = (longlong *)0x0;
  local_b0 = 4;
  local_a8 = (longlong *)0x0;
  puVar18 = (undefined8 *)0x0;
  plVar12 = (longlong *)0x0;
  local_50 = 0;
  local_4c = 0;
  puVar28 = (ulonglong *)0x0;
  local_c0 = in_RCX;
LAB_1408bb01e:
  puVar13 = (undefined1 *)0x1;
LAB_1408bb090:
  bStack_51 = (byte)puVar13;
  bVar6 = (byte)plVar12;
  lVar41 = (longlong)(char)bVar6;
  iVar35 = *(int *)(&DAT_142aa14c0 + lVar41 * 4);
  uVar15 = CONCAT17((undefined1)uStack_59,local_60);
  if (uVar15 == local_70) {
    uStack_f0 = (ulonglong)&LAB_1408bb0bc;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_78;
  *(int *)(local_68 + uVar15 * 4) = (int)puVar18;
  local_60 = (undefined7)(uVar15 + 1);
  uStack_59._0_1_ = (undefined1)(uVar15 + 1 >> 0x38);
  iVar35 = iVar35 + (int)puVar18;
  if ((NANI_FORCE_FINE_GRAINED_SORTING != 0) ||
     (uVar39 = 0x40, bVar6 < 2 && (NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) == 0)) {
    uVar39 = *(uint *)(&DAT_142aa14c0 + lVar41 * 4);
  }
  if (local_78 == local_88) {
    uStack_f0 = 0x1408bb125;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  uVar15 = local_90;
  bVar3 = NANI_FORCE_FINE_GRAINED_SORTING;
  *(uint *)(local_80 + (longlong)plVar12 * 4) = local_50;
  local_78 = (longlong *)((longlong)plVar12 + 1);
  local_50 = local_50 + uVar39;
  uVar36 = uVar39 >> 5;
  if ((NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) != 0) {
    uVar36 = 0;
  }
  if (1 < bVar6) {
    uVar36 = 0;
  }
  if ((bVar3 & 1) != 0) {
    uVar36 = uVar39 >> 5;
  }
  if (local_90 == local_a0) {
    uStack_f0 = (ulonglong)&LAB_1408bb17a;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_a8;
  *(int *)(local_98 + uVar15 * 4) = local_4c;
  local_90 = uVar15 + 1;
  local_4c = local_4c + uVar36;
  if (local_a8 == local_b8) {
    uStack_f0 = (ulonglong)&LAB_1408bb1bd;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  *(int *)(local_b0 + (longlong)plVar12 * 4) = (int)puVar28;
  local_a8 = (longlong *)((longlong)plVar12 + 1);
  iVar38 = (uVar36 >> 10) + (int)puVar28 + (uint)((uVar36 & 0x3fe) != 0);
  iVar17 = *(int *)(&DAT_142aa14c0 + lVar41 * 4);
  uVar15 = CONCAT17((undefined1)uStack_59,local_60);
  if (uVar15 == local_70) {
    uStack_f0 = (ulonglong)&LAB_1408bb1f6;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_78;
  *(int *)(local_68 + uVar15 * 4) = iVar35;
  local_60 = (undefined7)(uVar15 + 1);
  uStack_59._0_1_ = (undefined1)(uVar15 + 1 >> 0x38);
  iVar17 = iVar17 + iVar35;
  if ((NANI_FORCE_FINE_GRAINED_SORTING != 0) ||
     (uVar39 = 0x40, bVar6 < 2 && (NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) == 0)) {
    uVar39 = *(uint *)(&DAT_142aa14c0 + lVar41 * 4);
  }
  if (local_78 == local_88) {
    uStack_f0 = 0x1408bb265;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  uVar15 = local_90;
  bVar3 = NANI_FORCE_FINE_GRAINED_SORTING;
  *(uint *)(local_80 + (longlong)plVar12 * 4) = local_50;
  local_78 = (longlong *)((longlong)plVar12 + 1);
  local_50 = local_50 + uVar39;
  uVar36 = uVar39 >> 5;
  if ((NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) != 0) {
    uVar36 = 0;
  }
  if (1 < bVar6) {
    uVar36 = 0;
  }
  if ((bVar3 & 1) != 0) {
    uVar36 = uVar39 >> 5;
  }
  if (local_90 == local_a0) {
    uStack_f0 = (ulonglong)&LAB_1408bb2ba;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_a8;
  *(int *)(local_98 + uVar15 * 4) = local_4c;
  local_90 = uVar15 + 1;
  local_4c = local_4c + uVar36;
  if (local_a8 == local_b8) {
    uStack_f0 = (ulonglong)&LAB_1408bb2fc;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  *(int *)(local_b0 + (longlong)plVar12 * 4) = iVar38;
  local_a8 = (longlong *)((longlong)plVar12 + 1);
  iVar38 = (uVar36 >> 10) + iVar38 + (uint)((uVar36 & 0x3fe) != 0);
  iVar35 = *(int *)(&DAT_142aa14c0 + lVar41 * 4);
  uVar15 = CONCAT17((undefined1)uStack_59,local_60);
  if (uVar15 == local_70) {
    uStack_f0 = (ulonglong)&LAB_1408bb336;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_78;
  *(int *)(local_68 + uVar15 * 4) = iVar17;
  local_60 = (undefined7)(uVar15 + 1);
  uStack_59._0_1_ = (undefined1)(uVar15 + 1 >> 0x38);
  uVar39 = iVar35 + iVar17;
  puVar40 = (undefined *)(ulonglong)uVar39;
  if ((NANI_FORCE_FINE_GRAINED_SORTING != 0) ||
     (uVar36 = 0x40, bVar6 < 2 && (NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) == 0)) {
    uVar36 = *(uint *)(&DAT_142aa14c0 + lVar41 * 4);
  }
  if (local_78 == local_88) {
    uStack_f0 = 0x1408bb3a5;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  uVar15 = local_90;
  bVar3 = NANI_FORCE_FINE_GRAINED_SORTING;
  *(uint *)(local_80 + (longlong)plVar12 * 4) = local_50;
  local_78 = (longlong *)((longlong)plVar12 + 1);
  local_50 = local_50 + uVar36;
  uVar29 = uVar36 >> 5;
  if ((NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) != 0) {
    uVar29 = 0;
  }
  if (1 < bVar6) {
    uVar29 = 0;
  }
  if ((bVar3 & 1) != 0) {
    uVar29 = uVar36 >> 5;
  }
  if (local_90 == local_a0) {
    uStack_f0 = (ulonglong)&LAB_1408bb3fa;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_a8;
  *(int *)(local_98 + uVar15 * 4) = local_4c;
  local_90 = uVar15 + 1;
  local_4c = local_4c + uVar29;
  if (local_a8 == local_b8) {
    uStack_f0 = (ulonglong)&LAB_1408bb43c;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  *(int *)(local_b0 + (longlong)plVar12 * 4) = iVar38;
  local_a8 = (longlong *)((longlong)plVar12 + 1);
  uVar36 = (uVar29 >> 10) + iVar38 + (uint)((uVar29 & 0x3fe) != 0);
  uVar37 = (ulonglong)uVar36;
  iVar35 = *(int *)(&DAT_142aa14c0 + lVar41 * 4);
  uVar15 = CONCAT17((undefined1)uStack_59,local_60);
  if (uVar15 == local_70) {
    uStack_f0 = (ulonglong)&LAB_1408bb475;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar12 = local_78;
  *(uint *)(local_68 + uVar15 * 4) = uVar39;
  local_60 = (undefined7)(uVar15 + 1);
  uStack_59._0_1_ = (undefined1)(uVar15 + 1 >> 0x38);
  uVar39 = iVar35 + uVar39;
  puVar18 = (undefined8 *)(ulonglong)uVar39;
  if ((NANI_FORCE_FINE_GRAINED_SORTING != 0) ||
     (uVar29 = 0x40, bVar6 < 2 && (NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) == 0)) {
    uVar29 = *(uint *)(&DAT_142aa14c0 + lVar41 * 4);
  }
  if (local_78 == local_88) {
    uStack_f0 = 0x1408bb4e5;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  uVar15 = local_90;
  bVar3 = NANI_FORCE_FINE_GRAINED_SORTING;
  *(uint *)(local_80 + (longlong)plVar12 * 4) = local_50;
  local_78 = (longlong *)((longlong)plVar12 + 1);
  ppuVar14 = (undefined **)(ulonglong)(local_50 + uVar29);
  local_50 = local_50 + uVar29;
  uVar30 = uVar29 >> 5;
  if ((NANI_FORCE_NO_FINE_GRAINED_SORTING & 1) != 0) {
    uVar30 = 0;
  }
  if (1 < bVar6) {
    uVar30 = 0;
  }
  if ((bVar3 & 1) != 0) {
    uVar30 = uVar29 >> 5;
  }
  if (local_90 == local_a0) {
    ppuVar14 = &PTR_anon_3c3ea25760756e0ff5020ae6f10abbb6_55_llvm_15866041425536365678_142aa0958;
    uStack_f0 = (ulonglong)&LAB_1408bb53a;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  plVar4 = local_a8;
  *(int *)(local_98 + uVar15 * 4) = local_4c;
  local_90 = uVar15 + 1;
  local_4c = local_4c + uVar30;
  if (local_a8 == local_b8) {
    ppuVar14 = &PTR_anon_3c3ea25760756e0ff5020ae6f10abbb6_55_llvm_15866041425536365678_142aa0970;
    uStack_f0 = (ulonglong)&LAB_1408bb57c;
    alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
  }
  *(uint *)(local_b0 + (longlong)plVar4 * 4) = uVar36;
  plVar31 = (longlong *)((longlong)plVar4 + 1);
  uVar36 = (uVar30 >> 10) + uVar36 + (uint)((uVar30 & 0x3fe) != 0);
  puVar28 = (ulonglong *)(ulonglong)uVar36;
  plVar12 = (longlong *)(ulonglong)bStack_51;
  puVar10 = &DAT_142aa15a8;
  lVar41 = (longlong)(int)(&DAT_142aa15a8)[(longlong)plVar12];
  pbVar7 = (byte *)((longlong)&DAT_142aa15a8 + lVar41);
  bVar42 = pbVar7 == (byte *)0x0;
  puVar13 = (undefined1 *)CONCAT71((int7)((ulonglong)ppuVar14 >> 8),2);
  bVar6 = (byte)pbVar7;
  iVar35 = (int)pbVar7;
  uVar9 = (undefined7)((ulonglong)pbVar7 >> 8);
  uVar33 = SUB84(in_R8,0);
  puVar27 = auStack_e8;
  puVar24 = (ulonglong *)auStack_e8;
  puVar25 = auStack_e8;
  local_a8 = plVar31;
  plVar32 = plVar31;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bStack_51) {
  case 0:
    goto LAB_1408bb01e;
  case 1:
    goto LAB_1408bb090;
  case 2:
    puVar13 = (undefined1 *)0x3;
    goto LAB_1408bb090;
  case 3:
    puVar13 = (undefined1 *)0x4;
    goto LAB_1408bb090;
  case 4:
    puVar13 = (undefined1 *)0x5;
    goto LAB_1408bb090;
  case 5:
    puVar13 = (undefined1 *)0x6;
    goto LAB_1408bb090;
  case 6:
    puVar13 = (undefined1 *)0x7;
    goto LAB_1408bb090;
  case 7:
    puVar13 = (undefined1 *)0x8;
    goto LAB_1408bb090;
  case 8:
    puVar13 = (undefined1 *)0x9;
    goto LAB_1408bb090;
  case 9:
    puVar13 = (undefined1 *)0xa;
    goto LAB_1408bb090;
  case 10:
    puVar13 = (undefined1 *)0xb;
    goto LAB_1408bb090;
  case 0xb:
    puVar13 = (undefined1 *)0xc;
    goto LAB_1408bb090;
  case 0xc:
    puVar13 = (undefined1 *)0xd;
    goto LAB_1408bb090;
  case 0xd:
    puVar13 = (undefined1 *)0xe;
    goto LAB_1408bb090;
  case 0xe:
    puVar13 = (undefined1 *)0xf;
    goto LAB_1408bb090;
  case 0xf:
    puVar13 = (undefined1 *)0x10;
    goto LAB_1408bb090;
  case 0x10:
    puVar13 = (undefined1 *)0x11;
    goto LAB_1408bb090;
  case 0x11:
    puVar13 = (undefined1 *)0x12;
    goto LAB_1408bb090;
  case 0x12:
    puVar13 = (undefined1 *)0x13;
    goto LAB_1408bb090;
  case 0x13:
    puVar13 = (undefined1 *)0x14;
    goto LAB_1408bb090;
  case 0x14:
    puVar13 = (undefined1 *)0x15;
    goto LAB_1408bb090;
  case 0x15:
    puVar13 = (undefined1 *)0x16;
    goto LAB_1408bb090;
  case 0x16:
    puVar13 = (undefined1 *)0x17;
    goto LAB_1408bb090;
  case 0x17:
    puVar13 = (undefined1 *)0x18;
    goto LAB_1408bb090;
  case 0x18:
    puVar13 = (undefined1 *)0x19;
    goto LAB_1408bb090;
  case 0x19:
    puVar13 = (undefined1 *)0x1a;
    goto LAB_1408bb090;
  case 0x1a:
    puVar13 = (undefined1 *)0x1b;
    goto LAB_1408bb090;
  case 0x1b:
    local_c0[2] = CONCAT17((undefined1)uStack_59,local_60);
    *local_c0 = local_70;
    local_c0[1] = local_68;
    local_c0[5] = (ulonglong)local_78;
    local_c0[3] = (ulonglong)local_88;
    local_c0[4] = local_80;
    local_c0[6] = local_a0;
    local_c0[7] = local_98;
    local_c0[8] = local_90;
    local_c0[9] = (ulonglong)local_b8;
    local_c0[10] = local_b0;
    local_c0[0xb] = (ulonglong)plVar31;
    *(uint *)(local_c0 + 0xc) = uVar39;
    *(uint *)((longlong)local_c0 + 100) = local_50;
    *(int *)(local_c0 + 0xd) = local_4c;
    *(uint *)((longlong)local_c0 + 0x6c) = uVar36;
    return;
  case 0x1c:
    *pbVar7 = *pbVar7 + bVar6;
  case 0x2c:
    uVar15 = 0;
    uStack_f0 = 0x1408bb670;
    memset(&local_c0,0,0x1b0);
    pbVar7 = (byte *)&DAT_142aa1618;
    uVar11 = 0;
    goto LAB_1408bb6b8;
  case 0x1d:
    puVar20 = (undefined1 *)((ulonglong)pbVar7 & 0xffffffff);
    pbVar7 = (byte *)0x42aa35a8;
    goto code_r0x0001408bb889;
  case 0x1e:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa16a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa17a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa18a8;
    uVar15 = 0x1a;
    uVar11 = 0x42aa19a8;
    goto LAB_1408bb6b8;
  case 0x1f:
    pcVar1 = (char *)((longlong)&DAT_142aa15a8 + lVar41 + 1);
    *pcVar1 = *pcVar1 + bVar6;
    cRam000000064d526c8c = cRam000000064d526c8c + bVar6;
    puVar19 = (undefined1 *)((ulonglong)pbVar7 & 0xffffffff);
    pbVar7 = (byte *)0x42ac95a8;
  case 0x2b:
    *(undefined4 *)(puVar19 + (longlong)puVar13 * 4 + 0x28) = 0x42aa15a8;
    *(undefined4 *)(puVar19 + (longlong)puVar13 * 4 + 0x2c) = 0x42ab55a8;
    *(undefined4 *)(puVar19 + (longlong)puVar13 * 4 + 0x30) = 0x42ac95a8;
    *(undefined4 *)(puVar19 + (longlong)puVar13 * 4 + 0x34) = 0x42add5a8;
    uVar15 = 0x19;
    uVar11 = 0x42af15a8;
    goto LAB_1408bb6b8;
  case 0x20:
    if (!bVar42) {
      uStack_f0 = 0x1408bb620;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_70 != 0) {
      uStack_f0 = 0x1408bb63c;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  case 0x21:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa19a8;
    in_R8 = (undefined *)0x42aa1da8;
    goto code_r0x0001408bb916;
  case 0x22:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa19a8;
    in_R8 = (undefined *)0x42aa1da8;
    goto code_r0x0001408bb995;
  case 0x23:
    *pbVar7 = *pbVar7 + 2;
    *pbVar7 = *pbVar7 + bVar6;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = uVar33;
    in_R8 = (undefined *)0x42aa35a8;
    goto code_r0x0001408bb889;
  case 0x24:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa95a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42ab15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42ab95a8;
    uVar15 = 0x13;
    uVar11 = 0x42ac15a8;
    goto LAB_1408bb6b8;
  case 0x25:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aae528;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42abb4a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42ac8428;
    uVar15 = 0xc;
    uVar11 = 0x42ad53a8;
    goto LAB_1408bb6b8;
  case 0x26:
    *pbVar7 = *pbVar7 + bVar6;
    pbVar7 = (byte *)CONCAT71(uVar9,bVar6 + *pbVar7);
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = uVar33;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42ae15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42b015a8;
    uVar15 = 0xe;
    uVar11 = 0x42b215a8;
    goto LAB_1408bb6b8;
  case 0x27:
    uVar15 = (ulonglong)(byte)(uVar39 >> 8);
    *(undefined4 *)((longlong)&local_c0 + uVar15 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + uVar15 * 4 + 4) = 0x42aa25a8;
    *(undefined4 *)((longlong)&local_b8 + uVar15 * 4) = 0x42aa35a8;
    *(undefined4 *)((longlong)&local_b8 + uVar15 * 4 + 4) = 0x42aa45a8;
    uVar15 = 0x11;
    uVar11 = 0x42aa55a8;
    goto LAB_1408bb6b8;
  case 0x28:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa17a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa19a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa1ba8;
    uVar15 = 0xf;
    uVar11 = 0x42aa1da8;
    goto LAB_1408bb6b8;
  case 0x29:
    *pbVar7 = *pbVar7 + 0xa8;
    *pbVar7 = *pbVar7 + bVar6;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = uVar33;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa25a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa2da8;
    uVar15 = 0x15;
    uVar11 = 0x42aa35a8;
    goto LAB_1408bb6b8;
  case 0x2a:
    *pbVar7 = *pbVar7 + bVar6;
    *(int *)((longlong)&local_c0 + (longlong)in_R8 * 4 + 4) = (int)in_R9;
    *(undefined4 *)((longlong)&local_b8 + (longlong)in_R8 * 4) = 0x42aa55a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)in_R8 * 4 + 4) = 0x42aa75a8;
    uVar11 = 0x42aa95a8;
    uVar15 = (ulonglong)puVar13 & 0xffffffff;
    goto LAB_1408bb6b8;
  case 0x2d:
    *pbVar7 = *pbVar7 + bVar6;
    puVar20 = auStack_e8;
code_r0x0001408bb889:
    *(int *)(puVar20 + (longlong)puVar13 * 4 + 0x30) = (int)in_R8;
    *(undefined4 *)(puVar20 + (longlong)puVar13 * 4 + 0x34) = 0x42aa45a8;
    uVar15 = 3;
    uVar11 = 0x42aa55a8;
    goto LAB_1408bb6b8;
  case 0x2e:
    *(char *)((longlong)pbVar7 * 2) = *(char *)((longlong)pbVar7 * 2) + bVar6;
    cRam000000064d526c8c = cRam000000064d526c8c + bVar6;
    puVar23 = (undefined1 *)((ulonglong)pbVar7 & 0xffffffff);
    pbVar7 = (byte *)0x42aa1da8;
code_r0x0001408bb995:
    *(int *)(puVar23 + (longlong)puVar13 * 4 + 0x30) = (int)in_R8;
    *(undefined4 *)(puVar23 + (longlong)puVar13 * 4 + 0x34) = 0x42aa21a8;
    uVar15 = 0x16;
    uVar11 = 0x42aa25a8;
    goto LAB_1408bb6b8;
  case 0x2f:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42ae15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42b215a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42b615a8;
    uVar15 = 9;
    uVar11 = 0x42ba15a8;
    goto LAB_1408bb6b8;
  case 0x30:
    uVar21 = (ulonglong)pbVar7 & 0xffffffff;
    pbVar7 = (byte *)0x42ab15a8;
    *(undefined4 *)(uVar21 + 0x30 + (longlong)puVar13 * 4) = uVar33;
    *(undefined4 *)(uVar21 + 0x34 + (longlong)puVar13 * 4) = 0x42ab95a8;
    uVar15 = 4;
    uVar11 = 0x42ac15a8;
    goto LAB_1408bb6b8;
  case 0x31:
    *(char *)((longlong)pbVar7 * 2) = *(char *)((longlong)pbVar7 * 2) + bVar6;
    cRam000000064d526c8c = cRam000000064d526c8c + bVar6;
    puVar22 = (undefined1 *)((ulonglong)pbVar7 & 0xffffffff);
    pbVar7 = (byte *)0x42aa1da8;
code_r0x0001408bb916:
    *(int *)(puVar22 + (longlong)puVar13 * 4 + 0x30) = (int)in_R8;
    *(undefined4 *)(puVar22 + (longlong)puVar13 * 4 + 0x34) = 0x42aa21a8;
    uVar15 = 0x12;
    uVar11 = 0x42aa25a8;
    goto LAB_1408bb6b8;
  case 0x32:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa19a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa1da8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa21a8;
    uVar15 = 0xb;
    uVar11 = 0x42aa25a8;
    goto LAB_1408bb6b8;
  case 0x33:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa1da8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa25a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa2da8;
    uVar15 = 0x17;
    uVar11 = 0x42aa35a8;
    goto LAB_1408bb6b8;
  case 0x34:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aab5a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42ab55a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42abf5a8;
    uVar15 = 10;
    uVar11 = 0x42ac95a8;
    goto LAB_1408bb6b8;
  case 0x35:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42aa3b28;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42aa60a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42aa8628;
    uVar15 = 0xd;
    uVar11 = 0x42aaaba8;
    goto LAB_1408bb6b8;
  case 0x36:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42ab15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42ac15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42ad15a8;
    uVar15 = 0x14;
    uVar11 = 0x42ae15a8;
    goto LAB_1408bb6b8;
  case 0x37:
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4) = 0x42aa15a8;
    *(undefined4 *)((longlong)&local_c0 + (longlong)puVar13 * 4 + 4) = 0x42ac15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4) = 0x42ae15a8;
    *(undefined4 *)((longlong)&local_b8 + (longlong)puVar13 * 4 + 4) = 0x42b015a8;
    uVar15 = 0x18;
    uVar11 = 0x42b215a8;
LAB_1408bb6b8:
                    // WARNING: Could not recover jumptable at 0x0001408bb6c3. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(pbVar7 + *(int *)(pbVar7 + (uVar15 & 0xff) * 4)))
              (uVar11,pbVar7 + *(int *)(pbVar7 + (uVar15 & 0xff) * 4));
    return;
  case 0x38:
    in_R8 = &UNK_142aa0ef2;
    break;
  case 0x39:
    *(char *)(puVar18 + 0xa1550ade) = *(char *)(puVar18 + 0xa1550ade) + 'X';
                    // WARNING (jumptable): Read-only address (ram,0x000142aa15d1) is written
                    // WARNING: Read-only address (ram,0x000142aa15d1) is written
    UNK_142aa15d1 = 0x32;
    *(undefined1 *)((longlong)plVar4 + 0x142aa15eb) =
         *(undefined1 *)((longlong)plVar4 + 0x142aa15eb);
    uStack_59 = uStack_59 >> 1 | (uint)((uStack_59 & 1) != 0) << 0x1f;
    (&UNK_142aa15a7)[lVar41] = (&UNK_142aa15a7)[lVar41] + -0x59;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3a:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0a75);
    break;
  case 0x3b:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e08f5);
    break;
  case 0x3c:
    goto code_r0x0001408c0653;
  case 0x3d:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x41000002);
    in_R8 = &UNK_142aa0f47;
    break;
  case 0x3e:
    return;
  case 0x3f:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e089e);
    break;
  case 0x40:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa15a8) is written
                    // WARNING: Read-only address (ram,0x000142aa15a8) is written
    DAT_142aa15a8 = 0xb7229a76;
    pbVar7 = (byte *)CONCAT71(uVar9,bVar6 - 1);
    *pbVar7 = *pbVar7 + (bVar6 - 1);
    in_R8 = &UNK_142aa0f0d;
    break;
  case 0x41:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0be9);
    break;
  case 0x42:
    pbVar8 = (byte *)((ulonglong)plVar31 & 0xffffffff);
    plVar32 = (longlong *)((ulonglong)pbVar7 & 0xffffffff);
    *(int *)pbVar8 = *(int *)pbVar8 + (int)plVar31;
    bVar6 = (byte)pbVar8;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1561) is written
                    // WARNING: Read-only address (ram,0x000142aa1561) is written
    PTR_anon_d73a4b5ce1ce08d9079e364c894f8a17_53_llvm_3895383109152965105_142aa1560._1_1_ =
         bVar6 - 0xc;
    *pbVar8 = *pbVar8 & bVar6;
    *pbVar8 = *pbVar8 + bVar6;
    in_R8 = &UNK_142aa0e9e;
    pbVar7 = pbVar8;
    break;
  case 0x43:
    goto code_r0x0001408c0653;
  case 0x44:
    pcVar2 = (code *)swi(1);
    (*pcVar2)();
    return;
  case 0x45:
    *pbVar7 = *pbVar7 + bVar6;
    in_R9 = (undefined4 *)0x1b;
    in_R8 = &UNK_142aa110c;
    break;
  case 0x46:
    goto code_r0x0001408c0643;
  case 0x47:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0b36);
    break;
  case 0x48:
    goto code_r0x0001408c060e;
  case 0x49:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e081c);
    break;
  case 0x4a:
    bRam000000064d526cac = bRam000000064d526cac | 0xa8;
    local_b8 = (longlong *)0x0;
    local_c0 = (ulonglong *)0x1;
    uStack_f0 = 0x1408c0546;
    puffin::ThreadProfiler::begin_scope();
    *plStackX_10 = *plStackX_10 + 1;
    local_60 = (undefined7)extraout_RAX;
    uStack_59._0_1_ = (undefined1)(extraout_RAX >> 0x38);
    local_a8 = (longlong *)0x1;
    puVar28 = (ulonglong *)*puVar18;
    lVar41 = puVar18[1];
    uVar15 = *puVar28;
    puVar10 = (undefined4 *)0x0;
    puVar34 = (undefined4 *)(uVar15 - *(ulonglong *)(lVar41 + 0x18));
    if (uVar15 < *(ulonglong *)(lVar41 + 0x18)) {
      puVar34 = puVar10;
    }
    puVar16 = (undefined4 *)(uVar15 - *(ulonglong *)(lVar41 + 0x38));
    if (uVar15 < *(ulonglong *)(lVar41 + 0x38)) {
      puVar16 = puVar10;
    }
    in_R11 = (longlong *)((longlong)puVar34 * 0x18 + *(longlong *)(lVar41 + 8));
    in_R9 = (undefined4 *)((longlong)*(undefined4 **)(lVar41 + 0x10) - (longlong)puVar34);
    if (*(undefined4 **)(lVar41 + 0x10) < puVar34) {
      in_R11 = (longlong *)&DAT_00000008;
      in_R9 = puVar10;
    }
    uVar37 = (longlong)puVar16 * 0x18 + *(longlong *)(lVar41 + 0x28);
    in_R10 = (undefined4 *)((longlong)*(undefined4 **)(lVar41 + 0x30) - (longlong)puVar16);
    if (*(undefined4 **)(lVar41 + 0x30) < puVar16) {
      uVar37 = 8;
      in_R10 = puVar10;
    }
    pbVar7 = (byte *)((*(longlong *)(lVar41 + 0x40) - (longlong)in_R10) - (longlong)in_R9);
    puVar24 = (ulonglong *)auStack_e8;
    local_a0 = extraout_RAX;
  case 0x5e:
code_r0x0001408c05bf:
    pbVar8 = (byte *)((longlong)&local_a0 + (longlong)puVar10);
    *pbVar8 = *pbVar8 >> 1 | *pbVar8 << 7;
    *puVar28 = (ulonglong)pbVar7;
    plVar31 = in_R11 + (longlong)in_R9 * 3;
    local_68 = uVar37 + (longlong)in_R10 * 0x18;
    puVar18 = (undefined8 *)&DAT_142aa0e4c;
    puVar25 = (undefined1 *)puVar24;
code_r0x0001408c05eb:
    puVar40 = &DAT_142aa1688;
    puVar26 = puVar25;
    goto code_r0x0001408c0600;
  case 0x4b:
    uVar39 = (uint)*puVar28;
    *(int *)(ulonglong)uVar39 = *(int *)(ulonglong)uVar39 + uVar39;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1561) is written
                    // WARNING: Read-only address (ram,0x000142aa1561) is written
    PTR_anon_d73a4b5ce1ce08d9079e364c894f8a17_53_llvm_3895383109152965105_142aa1560._1_1_ =
         (char)uVar39 + -0xc;
    pbVar7 = (byte *)((ulonglong)(uint3)(uVar39 >> 8) << 8);
    *pbVar7 = *pbVar7;
    in_R8 = &UNK_142aa0fef;
    puVar28 = (ulonglong *)((longlong)puVar28 + 4);
    break;
  case 0x4c:
    in_R8 = &UNK_142aa1098;
    break;
  case 0x4d:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0748);
    break;
  case 0x4e:
    out((short)puVar13,iVar35);
    *(int *)pbVar7 = *(int *)pbVar7 + iVar35;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1561) is written
                    // WARNING: Read-only address (ram,0x000142aa1561) is written
    PTR_anon_d73a4b5ce1ce08d9079e364c894f8a17_53_llvm_3895383109152965105_142aa1560._1_1_ =
         bVar6 - 0xc;
    cVar5 = bVar6 - (0xb < bVar6);
    pbVar7 = (byte *)CONCAT71(uVar9,cVar5);
    *pbVar7 = *pbVar7 + cVar5;
    in_R8 = &UNK_142aa1127;
    break;
  case 0x4f:
    if ((longlong)pbVar7 < 0) {
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1561) is written
                    // WARNING: Read-only address (ram,0x000142aa1561) is written
      PTR_anon_d73a4b5ce1ce08d9079e364c894f8a17_53_llvm_3895383109152965105_142aa1560._1_1_ =
           bVar6 - 0xc;
      pbVar7 = (byte *)(ulonglong)((iVar35 - *(int *)pbVar7) - (uint)(0xb < bVar6));
      goto code_r0x0001408c0897;
    }
    pbVar7 = (byte *)0x42aa1041;
    break;
  case 0x50:
    *(int *)pbVar7 = *(int *)pbVar7 + iVar35;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1561) is written
                    // WARNING: Read-only address (ram,0x000142aa1561) is written
    PTR_anon_d73a4b5ce1ce08d9079e364c894f8a17_53_llvm_3895383109152965105_142aa1560._1_1_ =
         bVar6 - 0xc;
    *(uint *)pbVar7 = (*(int *)pbVar7 - iVar35) - (uint)(0xb < bVar6);
    *pbVar7 = *pbVar7 + bVar6;
    in_R8 = &UNK_142aa0ed9;
    break;
  case 0x51:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0b9e);
    break;
  case 0x52:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0b98);
    break;
  case 0x53:
code_r0x0001408c0897:
    *pbVar7 = *pbVar7 + (char)pbVar7;
    in_R8 = &UNK_142aa0fb7;
    break;
  case 0x54:
    puVar18 = (undefined8 *)((ulonglong)plStackX_10 & 0xffffffff);
    goto code_r0x0001408c0600;
  case 0x55:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0xfde681e7U);
    *pbVar7 = *pbVar7 + (char)(iVar35 + 0xfde681e7U);
    in_R9 = (undefined4 *)0x1d;
    in_R8 = &UNK_142aa10b7;
    break;
  case 0x56:
    *(int *)pbVar7 = *(int *)pbVar7 + -0x42aa15a8;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x57:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e06ed);
    break;
  case 0x58:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e0ad5);
    break;
  case 0x59:
    pbVar7 = (byte *)(ulonglong)(iVar35 + 0x21e09af);
    break;
  case 0x5a:
    goto code_r0x0001408c0653;
  case 0x5b:
    *(byte *)((longlong)in_R9 + 0xf) = *(byte *)((longlong)in_R9 + 0xf) | bVar6;
    goto code_r0x0001408c060e;
  case 0x5c:
    puVar24 = &uStack_f0;
    uStack_f0 = (ulonglong)puVar13;
    goto code_r0x0001408c05bf;
  case 0x5d:
    goto code_r0x0001408c05eb;
  }
  local_80 = CONCAT44(local_80._4_4_,*(undefined4 *)(pbVar7 + 8));
  local_88 = *(longlong **)pbVar7;
  local_78 = in_R11;
  if (*plVar12 == 0) {
    uStack_f0 = (ulonglong)&UNK_1408c09fa;
    local_70 = uVar37;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
      uStack_f0 = (ulonglong)&UNK_1408c0bd9;
      alloc::raw_vec::handle_error();
    }
    uStack_f0 = (ulonglong)&UNK_1408c0a14;
    memcpy(_Dst,in_R8,(size_t)in_R9);
    uStack_59 = uStack_59 & 0xff;
    uStack_55 = 0;
    bStack_51 = 0;
    puVar40 = &DAT_142aa1688;
    uVar37 = local_70;
    local_48 = in_R9;
  }
  else {
    uStack_f0 = (ulonglong)&UNK_1408c09b6;
    resources::audio_resources::AudioCoreInner::instantiate_audio();
    if (CONCAT17(bStack_51,CONCAT43(uStack_55,uStack_59._1_3_)) == 2) {
      puStack_c8 = &UNK_142aa12b8;
                    // WARNING: Subroutine does not return
      uStack_f0 = (ulonglong)&UNK_1408c0bc3;
      core::result::unwrap_failed();
    }
  }
  puVar10 = local_48;
  uStack_f0 = (ulonglong)&UNK_1408c0a9f;
  resources::audio_resources::impl$10::set_position();
  uStack_f0 = (ulonglong)&UNK_1408c0ac8;
  resources::audio_resources::impl$10::set_parameter();
  uStack_f0 = (ulonglong)&UNK_1408c0ad0;
  resources::audio_resources::AudioInstance::start_if_not_playing();
  uStack_f0 = (ulonglong)&UNK_1408c0ad8;
  resources::audio_resources::impl$13::drop();
  puVar26 = auStack_e8;
  plVar31 = plVar32;
  if (puVar10 != (undefined4 *)0x0) {
    uStack_f0 = (ulonglong)&UNK_1408c0af4;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    puVar26 = auStack_e8;
  }
code_r0x0001408c0600:
  if ((plStackX_10 != (longlong *)0x0) && (in_R11 = plStackX_10, plStackX_10 != plVar31)) {
code_r0x0001408c060e:
    *puVar28 = *puVar28 + 1;
                    // WARNING: Could not recover jumptable at 0x0001408c0636. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar40 + *(int *)(puVar40 + (ulonglong)*(byte *)((longlong)in_R11 + 0x14) * 4)))
              (in_R11 + 3,
               puVar40 + *(int *)(puVar40 + (ulonglong)*(byte *)((longlong)in_R11 + 0x14) * 4),
               puVar18,0x1e);
    return;
  }
  bVar42 = uVar37 == 0;
  puVar27 = puVar26;
code_r0x0001408c0643:
  if ((!bVar42) && (uVar37 != local_68)) {
code_r0x0001408c0653:
    *puVar28 = *puVar28 + 1;
                    // WARNING: Could not recover jumptable at 0x0001408c0687. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar40 + *(int *)(puVar40 + (ulonglong)*(byte *)(uVar37 + 0x14) * 4)))
              (0,puVar40 + *(int *)(puVar40 + (ulonglong)*(byte *)(uVar37 + 0x14) * 4),puVar18,0x1e)
    ;
    return;
  }
  if (cStackX_f != '\0') {
    *(undefined8 *)(puVar27 + -8) = 0x1408c0b04;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_00 == 1) {
      plVar12 = extraout_RAX_00 + 1;
    }
    else {
      if (*extraout_RAX_00 != 0) {
LAB_1408c0b7b:
                    // WARNING: Subroutine does not return
        *(undefined8 *)(puVar27 + -8) = 0x1408c0b87;
        std::thread::local::panic_access_error();
      }
      *(undefined8 *)(puVar27 + -8) = 0x1408c0b73;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar12 = extraout_RAX_01;
      if (extraout_RAX_01 == (longlong *)0x0) goto LAB_1408c0b7b;
    }
    if (*plVar12 != 0) {
                    // WARNING: Subroutine does not return
      *(undefined **)(puVar27 + -8) = &UNK_1408c0b93;
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    *(undefined8 *)(puVar27 + -8) = 0x1408c0b32;
    puffin::ThreadProfiler::end_scope();
    *plVar12 = *plVar12 + 1;
  }
  return;
}

