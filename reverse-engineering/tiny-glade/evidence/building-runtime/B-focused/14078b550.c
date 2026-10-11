
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_wall_constructor::init_wall_height_and_arch_segment_height::
init_wall_height_and_arch_segment_height(void)

{
  char *pcVar1;
  float *pfVar2;
  float *pfVar3;
  uint uVar4;
  uint uVar5;
  undefined1 auVar6 [16];
  bool bVar7;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  longlong lVar8;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  ulonglong uVar13;
  undefined4 uVar14;
  longlong *in_R8;
  longlong lVar15;
  float *pfVar16;
  uint uVar17;
  ushort uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong *puVar21;
  longlong lVar22;
  longlong *plVar23;
  undefined4 uVar24;
  byte bVar25;
  char cVar26;
  char cVar31;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar27;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  char cVar29;
  char cVar30;
  undefined1 auVar28 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  float fVar38;
  undefined1 auVar39 [16];
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  undefined4 uVar44;
  float fVar45;
  undefined4 uVar46;
  undefined4 uVar47;
  undefined4 uVar48;
  float fVar49;
  float fVar50;
  byte local_234;
  float local_208 [3];
  uint uStack_1fc;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  longlong lStack_1e8;
  longlong local_1e0;
  ulonglong uStack_1d8;
  undefined8 local_1d0;
  longlong local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  longlong lStack_1b0;
  longlong local_1a8;
  ulonglong uStack_1a0;
  undefined8 local_198;
  longlong local_188;
  longlong local_180;
  longlong local_178;
  longlong local_170;
  longlong *local_168;
  ulonglong local_160;
  longlong local_158;
  float local_150;
  float local_14c;
  longlong *local_148;
  ulonglong *local_140;
  float local_134;
  float local_130;
  float local_12c;
  longlong local_128;
  longlong local_120;
  undefined4 *local_118;
  longlong local_110;
  float local_104;
  longlong local_100;
  undefined4 local_f4;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_140 = (ulonglong *)*in_RDX;
  lVar10 = in_RDX[1];
  uVar9 = *local_140;
  uVar20 = 0;
  uVar13 = uVar9 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar13 = uVar20;
  }
  uVar19 = uVar9 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar19 = uVar20;
  }
  lVar8 = uVar13 * 0x10 + *(longlong *)(lVar10 + 8);
  uVar9 = *(ulonglong *)(lVar10 + 0x10) - uVar13;
  if (*(ulonglong *)(lVar10 + 0x10) < uVar13) {
    lVar8 = 8;
    uVar9 = uVar20;
  }
  local_128 = uVar19 * 0x10 + *(longlong *)(lVar10 + 0x28);
  uVar13 = *(ulonglong *)(lVar10 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar19) {
    local_128 = 8;
    uVar13 = uVar20;
  }
  *local_140 = (*(longlong *)(lVar10 + 0x40) - uVar13) - uVar9;
  local_188 = uVar9 * 0x10 + lVar8;
  local_180 = uVar13 * 0x10 + local_128;
  local_f4 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar23 = (longlong *)*in_RCX;
  local_118 = (undefined4 *)in_RCX[2];
  local_168 = plVar23 + 4;
  local_178 = *in_R8;
  local_160 = in_R8[1];
  auVar39._8_4_ = 0xffffffff;
  auVar39._0_8_ = 0xffffffffffffffff;
  auVar39._12_4_ = 0xffffffff;
  puVar12 = local_118;
  puVar21 = local_140;
  fVar41 = __real_c0200000;
  fVar45 = __real_beeb851f;
  fVar49 = __real_3f75c290;
  fVar42 = __real_3f000000;
  uVar44 = ___xmm_3f0000003f0000003f0000003f000000;
  uVar46 = _UNK_14293ac64;
  uVar47 = _UNK_14293ac68;
  uVar48 = _UNK_14293ac6c;
  fVar43 = __real_3fc00000;
  uVar24 = local_f4;
  local_148 = plVar23;
LAB_14078b6c0:
  do {
    if ((lVar8 == 0) || (lVar8 == local_188)) {
      if ((local_128 == 0) || (local_128 == local_180)) {
        return;
      }
      lVar10 = 0;
      lVar8 = local_128;
      local_128 = local_128 + 0x10;
    }
    else {
      lVar10 = lVar8 + 0x10;
    }
    *puVar21 = *puVar21 + 1;
    lVar8 = *(longlong *)(lVar8 + 8);
    *puVar12 = uVar24;
    local_170 = lVar8;
    if (plVar23[3] == 0) {
LAB_14078c5d9:
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar11 = *plVar23;
    bVar25 = (byte)(extraout_RAX >> 0x39);
    auVar28 = ZEXT216(CONCAT11(bVar25,bVar25));
    auVar28 = pshuflw(auVar28,auVar28,0);
    lVar15 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = plVar23[1] & uVar9;
      pcVar1 = (char *)(lVar11 + uVar9);
      cVar26 = auVar28[0];
      auVar37[0] = -(*pcVar1 == cVar26);
      cVar29 = auVar28[1];
      auVar37[1] = -(pcVar1[1] == cVar29);
      cVar30 = auVar28[2];
      auVar37[2] = -(pcVar1[2] == cVar30);
      cVar31 = auVar28[3];
      auVar37[3] = -(pcVar1[3] == cVar31);
      auVar37[4] = -(pcVar1[4] == cVar26);
      auVar37[5] = -(pcVar1[5] == cVar29);
      auVar37[6] = -(pcVar1[6] == cVar30);
      auVar37[7] = -(pcVar1[7] == cVar31);
      auVar37[8] = -(pcVar1[8] == cVar26);
      auVar37[9] = -(pcVar1[9] == cVar29);
      auVar37[10] = -(pcVar1[10] == cVar30);
      auVar37[0xb] = -(pcVar1[0xb] == cVar31);
      auVar37[0xc] = -(pcVar1[0xc] == cVar26);
      auVar37[0xd] = -(pcVar1[0xd] == cVar29);
      auVar37[0xe] = -(pcVar1[0xe] == cVar30);
      auVar37[0xf] = -(pcVar1[0xf] == cVar31);
      uVar18 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf;
      uVar17 = (uint)uVar18;
      if (uVar18 != 0) break;
LAB_14078b7b0:
      auVar32[0] = -(*pcVar1 == auVar39[0]);
      auVar32[1] = -(pcVar1[1] == auVar39[1]);
      auVar32[2] = -(pcVar1[2] == auVar39[2]);
      auVar32[3] = -(pcVar1[3] == auVar39[3]);
      auVar32[4] = -(pcVar1[4] == auVar39[4]);
      auVar32[5] = -(pcVar1[5] == auVar39[5]);
      auVar32[6] = -(pcVar1[6] == auVar39[6]);
      auVar32[7] = -(pcVar1[7] == auVar39[7]);
      auVar32[8] = -(pcVar1[8] == auVar39[8]);
      auVar32[9] = -(pcVar1[9] == auVar39[9]);
      auVar32[10] = -(pcVar1[10] == auVar39[10]);
      auVar32[0xb] = -(pcVar1[0xb] == auVar39[0xb]);
      auVar32[0xc] = -(pcVar1[0xc] == auVar39[0xc]);
      auVar32[0xd] = -(pcVar1[0xd] == auVar39[0xd]);
      auVar32[0xe] = -(pcVar1[0xe] == auVar39[0xe]);
      auVar32[0xf] = -(pcVar1[0xf] == auVar39[0xf]);
      if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 || (SUB161(auVar32 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar32 >> 0x17,0) & 1) != 0) || (SUB161(auVar32 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar32 >> 0x27,0) & 1) != 0) || (SUB161(auVar32 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar32 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar32 >> 0x3f,0) & 1) != 0) || (SUB161(auVar32 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar32 >> 0x4f,0) & 1) != 0) || (SUB161(auVar32 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar32 >> 0x5f,0) & 1) != 0) || (SUB161(auVar32 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar32 >> 0x6f,0) & 1) != 0) || (SUB161(auVar32 >> 0x77,0) & 1) != 0) ||
          auVar32[0xf] < '\0') goto LAB_14078c5d9;
      uVar9 = uVar9 + lVar15 + 0x10;
      lVar15 = lVar15 + 0x10;
    }
    while( true ) {
      uVar4 = 0;
      for (uVar5 = uVar17; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      uVar13 = uVar4 + uVar9 & plVar23[1];
      lVar22 = uVar13 * -0x2a8;
      local_100 = lVar10;
      if (lVar8 == *(longlong *)(lVar11 + -0x2a8 + lVar22)) break;
      uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
      uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
      if (uVar18 == 0) goto LAB_14078b7b0;
    }
    lVar10 = *(longlong *)(lVar11 + -0x1d0 + lVar22);
    local_120 = lVar8;
    if (lVar10 == 0) {
      lVar15 = 4;
      lVar8 = *(longlong *)(lVar11 + lVar22 + -0x1b0);
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc_zeroed();
      if (extraout_RAX_00 == 0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      lVar8 = *(longlong *)(lVar11 + lVar22 + -0x1b0);
      lVar15 = extraout_RAX_00;
    }
    if (lVar8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar24 = local_f4;
    lVar8 = local_100;
    puVar21 = local_140;
    plVar23 = local_148;
    lVar22 = lVar11 + lVar22;
    *(longlong *)(lVar22 + -0x1b0) = lVar10;
    *(longlong *)(lVar22 + -0x1a8) = lVar15;
    *(longlong *)(lVar22 + -0x1a0) = lVar10;
    if (1 < *(uint *)(lVar22 + -0x48)) {
      if (*(char *)(lVar22 + -0xb) != '\0') {
        uVar9 = *(ulonglong *)(lVar22 + -0x208);
        if (uVar9 == 0) {
          fVar50 = *(float *)(lVar22 + -0x1c) + fVar41;
          if (fVar50 <= fVar49) {
            fVar50 = fVar49;
          }
          *(undefined8 *)(lVar22 + -0x1b8) = 0;
          lVar10 = 0;
        }
        else {
          lVar10 = 0;
          fVar50 = fVar41;
          do {
            fVar40 = *(float *)(*(longlong *)(lVar22 + -0x210) + 4 + lVar10);
            fVar27 = fVar40;
            if (fVar40 <= fVar50) {
              fVar27 = fVar50;
            }
            fVar50 = (float)(~-(uint)NAN(fVar50) & (uint)fVar27 | -(uint)NAN(fVar50) & (uint)fVar40)
            ;
            lVar10 = lVar10 + 0xc;
          } while (uVar9 * 0xc != lVar10);
          fVar50 = fVar50 + *(float *)(lVar22 + -0x1c);
          if (fVar50 <= fVar49) {
            fVar50 = fVar49;
          }
          *(undefined8 *)(lVar22 + -0x1b8) = 0;
          if (uVar9 == 0) {
            lVar10 = 0;
          }
          else {
            if (*(ulonglong *)(lVar22 + -0x1c8) < uVar9) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              lVar10 = *(longlong *)(lVar22 + -0x1b8);
              pfVar3 = *(float **)(lVar22 + -0x1c0);
              pfVar16 = pfVar3 + lVar10;
            }
            else {
              lVar10 = 0;
              pfVar3 = *(float **)(lVar22 + -0x1c0);
              pfVar16 = pfVar3;
            }
            if (uVar9 != 1) {
              lVar11 = 1;
              if (uVar9 < 9) {
LAB_14078bab9:
                lVar11 = uVar9 - lVar11;
                do {
                  *pfVar16 = fVar50;
                  pfVar16 = pfVar16 + 1;
                  lVar11 = lVar11 + -1;
                } while (lVar11 != 0);
              }
              else {
                uVar13 = uVar9 - 1 & 0xfffffffffffffff8;
                pfVar16 = pfVar16 + uVar13;
                lVar11 = uVar13 + 1;
                uVar20 = 0;
                do {
                  pfVar2 = pfVar3 + lVar10 + uVar20;
                  *pfVar2 = fVar50;
                  pfVar2[1] = fVar50;
                  pfVar2[2] = fVar50;
                  pfVar2[3] = fVar50;
                  pfVar2 = pfVar3 + lVar10 + uVar20 + 4;
                  *pfVar2 = fVar50;
                  pfVar2[1] = fVar50;
                  pfVar2[2] = fVar50;
                  pfVar2[3] = fVar50;
                  uVar20 = uVar20 + 8;
                } while (uVar13 != uVar20);
                if (uVar9 - 1 != uVar13) goto LAB_14078bab9;
              }
              lVar10 = lVar10 + uVar9 + -1;
            }
            *pfVar16 = fVar50;
            lVar10 = lVar10 + 1;
          }
        }
        *(longlong *)(lVar22 + -0x1b8) = lVar10;
        *(undefined4 *)(lVar22 + -0x198) = 1;
        *(float *)(lVar22 + -0x194) = fVar50;
        *(undefined4 *)(lVar22 + -400) = 1;
        *(float *)(lVar22 + -0x18c) = fVar50;
        puVar12 = local_118;
        goto LAB_14078b6c0;
      }
      *(undefined8 *)(lVar22 + -0x1b8) = 0;
      lVar10 = *(longlong *)(lVar22 + -0x210);
      uVar9 = *(ulonglong *)(lVar22 + -0x208);
      if (*(ulonglong *)(lVar22 + -0x1c8) < uVar9) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar8 = *(longlong *)(lVar22 + -0x1b8);
        lVar15 = *(longlong *)(lVar22 + -0x1c0);
      }
      else {
        if (uVar9 == 0) {
          fVar50 = 0.0;
          goto LAB_14078c55e;
        }
        lVar8 = 0;
        lVar15 = *(longlong *)(lVar22 + -0x1c0);
      }
      if (7 < uVar9) {
        uVar20 = lVar15 + lVar8 * 4;
        pfVar16 = (float *)(lVar15 + (lVar8 + uVar9) * 4);
        if ((lVar10 + uVar9 * 0xc) - 4 <= uVar20 || pfVar16 <= (float *)(lVar10 + 4U)) {
          if (uVar20 < (lVar11 + uVar13 * -0x2a8) - 0x18 && (float *)(lVar22 + -0x1c) < pfVar16) {
            uVar13 = 0;
            goto LAB_14078c4c9;
          }
          uVar13 = uVar9 & 0xfffffffffffffff8;
          fVar50 = *(float *)(lVar22 + -0x1c);
          lVar11 = lVar15 + lVar8 * 4;
          lVar8 = lVar8 + uVar13;
          pfVar16 = (float *)(lVar10 + 0x58);
          uVar20 = 0;
          do {
            auVar33._0_4_ = pfVar16[-0x15] + fVar50;
            auVar33._4_4_ = pfVar16[-0x12] + fVar50;
            auVar33._8_4_ = pfVar16[-0xf] + fVar50;
            auVar33._12_4_ = pfVar16[-0xc] + fVar50;
            auVar36._0_4_ = pfVar16[-9] + fVar50;
            auVar36._4_4_ = pfVar16[-6] + fVar50;
            auVar36._8_4_ = pfVar16[-3] + fVar50;
            auVar36._12_4_ = *pfVar16 + fVar50;
            auVar28._4_4_ = uVar46;
            auVar28._0_4_ = uVar44;
            auVar28._8_4_ = uVar47;
            auVar28._12_4_ = uVar48;
            auVar28 = maxps(auVar33,auVar28);
            auVar6._4_4_ = uVar46;
            auVar6._0_4_ = uVar44;
            auVar6._8_4_ = uVar47;
            auVar6._12_4_ = uVar48;
            auVar37 = maxps(auVar36,auVar6);
            *(undefined1 (*) [16])(lVar11 + uVar20 * 4) = auVar28;
            *(undefined1 (*) [16])(lVar11 + 0x10 + uVar20 * 4) = auVar37;
            uVar20 = uVar20 + 8;
            pfVar16 = pfVar16 + 0x18;
          } while (uVar13 != uVar20);
          if (uVar9 != uVar13) {
LAB_14078c4c9:
            lVar11 = uVar9 - uVar13;
            pfVar16 = (float *)(lVar10 + uVar13 * 0xc + 4);
            do {
              fVar50 = *pfVar16 + *(float *)(lVar22 + -0x1c);
              if (fVar50 <= fVar42) {
                fVar50 = fVar42;
              }
              *(float *)(lVar15 + lVar8 * 4) = fVar50;
              lVar8 = lVar8 + 1;
              pfVar16 = pfVar16 + 3;
              lVar11 = lVar11 + -1;
            } while (lVar11 != 0);
          }
          *(longlong *)(lVar22 + -0x1b8) = lVar8;
          if (lVar8 == 0) {
            fVar50 = 0.0;
          }
          else {
            fVar50 = 0.0;
            lVar10 = 0;
            do {
              fVar40 = *(float *)(*(longlong *)(lVar22 + -0x1c0) + lVar10);
              fVar27 = fVar40;
              if (fVar40 <= fVar50) {
                fVar27 = fVar50;
              }
              fVar50 = (float)(~-(uint)NAN(fVar50) & (uint)fVar27 |
                              -(uint)NAN(fVar50) & (uint)fVar40);
              lVar10 = lVar10 + 4;
            } while (lVar8 * 4 != lVar10);
          }
LAB_14078c55e:
          *(undefined4 *)(lVar22 + -0x198) = 1;
          *(float *)(lVar22 + -0x194) = fVar50;
          *(undefined4 *)(lVar22 + -400) = 1;
          *(float *)(lVar22 + -0x18c) = fVar50;
          lVar8 = local_100;
          puVar12 = local_118;
          uVar24 = local_f4;
          goto LAB_14078b6c0;
        }
      }
      uVar13 = 0;
      goto LAB_14078c4c9;
    }
    uVar9 = *(ulonglong *)(lVar22 + -0x208);
    fVar50 = fVar41;
    if (uVar9 != 0) {
      lVar10 = 0;
      do {
        fVar40 = *(float *)(*(longlong *)(lVar22 + -0x210) + 4 + lVar10);
        fVar27 = fVar40;
        if (fVar40 <= fVar50) {
          fVar27 = fVar50;
        }
        fVar50 = (float)(~-(uint)NAN(fVar50) & (uint)fVar27 | -(uint)NAN(fVar50) & (uint)fVar40);
        lVar10 = lVar10 + 0xc;
      } while (uVar9 * 0xc != lVar10);
    }
    fVar50 = fVar50 + *(float *)(lVar22 + -0x1c) + fVar45;
    if (fVar50 <= fVar49) {
      fVar50 = fVar49;
    }
    *(undefined8 *)(lVar22 + -0x1b8) = 0;
    if (uVar9 == 0) {
      lVar10 = 0;
    }
    else {
      if (*(ulonglong *)(lVar22 + -0x1c8) < uVar9) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar10 = *(longlong *)(lVar22 + -0x1b8);
        pfVar3 = *(float **)(lVar22 + -0x1c0);
        pfVar16 = pfVar3 + lVar10;
      }
      else {
        lVar10 = 0;
        pfVar3 = *(float **)(lVar22 + -0x1c0);
        pfVar16 = pfVar3;
      }
      if (uVar9 != 1) {
        lVar11 = 1;
        if (uVar9 < 9) {
LAB_14078b969:
          lVar11 = uVar9 - lVar11;
          do {
            *pfVar16 = fVar50;
            pfVar16 = pfVar16 + 1;
            lVar11 = lVar11 + -1;
          } while (lVar11 != 0);
        }
        else {
          uVar13 = uVar9 - 1 & 0xfffffffffffffff8;
          pfVar16 = pfVar16 + uVar13;
          lVar11 = uVar13 + 1;
          uVar20 = 0;
          do {
            pfVar2 = pfVar3 + lVar10 + uVar20;
            *pfVar2 = fVar50;
            pfVar2[1] = fVar50;
            pfVar2[2] = fVar50;
            pfVar2[3] = fVar50;
            pfVar2 = pfVar3 + lVar10 + uVar20 + 4;
            *pfVar2 = fVar50;
            pfVar2[1] = fVar50;
            pfVar2[2] = fVar50;
            pfVar2[3] = fVar50;
            uVar20 = uVar20 + 8;
          } while (uVar13 != uVar20);
          if (uVar9 - 1 != uVar13) goto LAB_14078b969;
        }
        lVar10 = lVar10 + uVar9 + -1;
      }
      *pfVar16 = fVar50;
      lVar10 = lVar10 + 1;
    }
    uVar24 = local_f4;
    *(longlong *)(lVar22 + -0x1b8) = lVar10;
    *(undefined4 *)(lVar22 + -0x198) = 1;
    *(float *)(lVar22 + -0x194) = fVar50;
    *(undefined4 *)(lVar22 + -400) = 1;
    *(float *)(lVar22 + -0x18c) = fVar50;
    country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
    puVar12 = local_118;
    puVar21 = local_140;
    if (((local_234 & 1) != 0) &&
       (country_core::resources::walls::public_walls::PublicWalls::roof_entity(),
       puVar12 = local_118, puVar21 = local_140, extraout_RAX_01 >> 0x20 != 0)) {
      uVar13 = extraout_RAX_01 & 0xffffffff;
      uVar9 = local_160;
      if ((*(ulonglong *)(local_160 + 0x10) <= uVar13) ||
         (uVar9 = *(ulonglong *)(local_160 + 8),
         *(int *)(uVar9 + uVar13 * 0x14) != (int)(extraout_RAX_01 >> 0x20))) {
LAB_14078c667:
        uVar17 = (uint)uVar9;
        uVar14 = 1;
LAB_14078c677:
        local_208[2] = (float)uVar14;
        uStack_1fc = uVar17;
                    /* WARNING: Subroutine does not return */
        core::result::unwrap_failed();
      }
      lVar10 = uVar9 + uVar13 * 0x14;
      uVar17 = *(uint *)(lVar10 + 4);
      uVar9 = (ulonglong)uVar17;
      if (uVar9 == 0xffffffff) goto LAB_14078c667;
      uVar14 = 0;
      if ((*(ulonglong *)(local_178 + 0x38) <= uVar9) ||
         ((*(ulonglong *)(*(longlong *)(local_178 + 0x28) + (ulonglong)(uVar17 >> 6) * 8) >>
           (uVar9 & 0x3f) & 1) == 0)) goto LAB_14078c677;
      lVar10 = (ulonglong)*(uint *)(lVar10 + 0x10) * 0x58 +
               *(longlong *)
                (*(longlong *)
                  (*(longlong *)(local_160 + 0x1a8) + 0x18 +
                  (ulonglong)*(uint *)(lVar10 + 0xc) * 0x48) + 0x10 +
                ~*(ulonglong *)
                  (*(longlong *)
                    (*(longlong *)(local_160 + 0x1a8) + 0x38 +
                    (ulonglong)*(uint *)(lVar10 + 0xc) * 0x48) +
                  *(longlong *)(local_178 + 0x110) * 8) * 0x30);
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if (extraout_AL != '\0') {
        cVar26 = *(char *)(lVar10 + 0x3c);
        local_110 = lVar10;
        ::utils::geometry::rectangle::Rectangle2d::get_point_u();
        local_134 = extraout_XMM0_Da;
        ::utils::geometry::rectangle::Rectangle2d::get_point_u();
        ::utils::geometry::rectangle::Rectangle2d::get_point_u();
        local_130 = extraout_XMM0_Da_01;
        ::utils::geometry::rectangle::Rectangle2d::get_point_u();
        fVar41 = local_208[cVar26 == '\0'];
        *local_118 = local_f4;
        if (plVar23[3] == 0) goto LAB_14078c647;
        local_104 = extraout_XMM0_Da_02;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar10 = *plVar23;
        bVar25 = (byte)(extraout_RAX_02 >> 0x39);
        auVar28 = ZEXT216(CONCAT11(bVar25,bVar25));
        auVar28 = pshuflw(auVar28,auVar28,0);
        lVar8 = 0;
        uVar9 = extraout_RAX_02;
LAB_14078be82:
        uVar9 = plVar23[1] & uVar9;
        pcVar1 = (char *)(lVar10 + uVar9);
        cVar26 = auVar28[0];
        auVar35[0] = -(*pcVar1 == cVar26);
        cVar29 = auVar28[1];
        auVar35[1] = -(pcVar1[1] == cVar29);
        cVar30 = auVar28[2];
        auVar35[2] = -(pcVar1[2] == cVar30);
        cVar31 = auVar28[3];
        auVar35[3] = -(pcVar1[3] == cVar31);
        auVar35[4] = -(pcVar1[4] == cVar26);
        auVar35[5] = -(pcVar1[5] == cVar29);
        auVar35[6] = -(pcVar1[6] == cVar30);
        auVar35[7] = -(pcVar1[7] == cVar31);
        auVar35[8] = -(pcVar1[8] == cVar26);
        auVar35[9] = -(pcVar1[9] == cVar29);
        auVar35[10] = -(pcVar1[10] == cVar30);
        auVar35[0xb] = -(pcVar1[0xb] == cVar31);
        auVar35[0xc] = -(pcVar1[0xc] == cVar26);
        auVar35[0xd] = -(pcVar1[0xd] == cVar29);
        auVar35[0xe] = -(pcVar1[0xe] == cVar30);
        auVar35[0xf] = -(pcVar1[0xf] == cVar31);
        uVar18 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf
        ;
        uVar17 = (uint)uVar18;
        while( true ) {
          if (uVar18 == 0) goto LAB_14078becc;
          uVar4 = 0;
          for (uVar5 = uVar17; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          lVar11 = (uVar4 + uVar9 & plVar23[1]) * -0x2a8;
          if (local_120 == *(longlong *)(lVar10 + -0x2a8 + lVar11)) break;
          uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
          uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
        }
        fVar45 = *(float *)(local_110 + 0x2c);
        fVar41 = fVar41 * fVar42;
        local_120 = lVar10 + lVar11 + -7;
        country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
        fVar49 = fVar41;
        if (extraout_AL_00 == '\0') {
          country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws();
          ::utils::inverse_lerp();
          fVar49 = extraout_XMM0_Da_04 * extraout_XMM0_Da_03 + fVar41;
        }
        lVar10 = lVar10 + lVar11;
        fVar49 = fVar49 + __real_3e8f5c29;
        fVar40 = (fVar42 / *(float *)(lVar10 + -0x14)) * __real_3f733333;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
                  ();
        ::utils::curve::Curve::
        try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
        lVar8 = local_1e0;
        if (SBORROW8(0,CONCAT44(uStack_1fc,local_208[2]))) {
          local_1c8 = uStack_1f8;
          uStack_1c0 = local_1f0;
                    /* WARNING: Subroutine does not return */
          core::result::unwrap_failed();
        }
        local_198 = local_1d0;
        local_1c8 = CONCAT44(uStack_1fc,local_208[2]);
        local_1a8 = local_1e0;
        uStack_1a0 = uStack_1d8;
        local_1b8 = local_1f0;
        lStack_1b0 = lStack_1e8;
        uStack_1c0 = uStack_1f8;
        uVar9 = uStack_1d8;
        if (*(ulonglong *)(lVar10 + -0x1b8) < uStack_1d8) {
          uVar9 = *(ulonglong *)(lVar10 + -0x1b8);
        }
        local_158 = lVar10;
        if (uVar9 == 0) {
          bVar7 = false;
        }
        else {
          fVar45 = fVar45 * fVar43 + (__real_3f800000 - fVar45) * fVar42;
          fVar49 = fVar49 / fVar41;
          lVar10 = *(longlong *)(lVar10 + -0x1c0);
          local_150 = fVar40 + extraout_XMM0_Da_00;
          fVar42 = local_134 - fVar40;
          local_14c = extraout_XMM0_Da_00 - local_134;
          fVar43 = local_130 - fVar40;
          fVar40 = fVar40 + local_104;
          local_104 = local_104 - local_130;
          bVar7 = false;
          uVar13 = 0;
          do {
            fVar41 = *(float *)(lVar8 + uVar13 * 4);
            if ((fVar42 <= fVar41) && (fVar41 < local_150)) {
              fVar27 = (fVar41 - local_134) / local_14c;
              fVar41 = 0.0;
              if (0.0 <= fVar27) {
                fVar41 = fVar27;
              }
              fVar27 = __real_3f800000;
              if (fVar41 <= __real_3f800000) {
                fVar27 = fVar41;
              }
              fVar27 = fVar27 + __real_bf000000;
              country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
              country_core::resources::roofs::roof_shape::Roof::height();
              fVar38 = __real_3f800000;
              fVar41 = 0.0;
              if (extraout_AL_01 == '\0') {
                fVar41 = __real_3eeb851f;
              }
              fVar27 = (float)((uint)fVar27 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
              fVar27 = powf(__real_3f800000 - (fVar27 + fVar27) / fVar49,fVar45);
              fVar27 = (extraout_XMM0_Da_05 + fVar41) * fVar27 + (fVar38 - fVar27) * fVar41;
              *(float *)(lVar10 + uVar13 * 4) = *(float *)(lVar10 + uVar13 * 4) + fVar27;
              fVar41 = *(float *)(lVar8 + uVar13 * 4);
              if (extraout_AL_01 == '\0' && fVar41 == fVar38) {
                bVar7 = true;
                local_12c = fVar27;
              }
            }
            if ((fVar43 <= fVar41) && (fVar41 < fVar40)) {
              fVar27 = (fVar41 - local_130) / local_104;
              fVar41 = 0.0;
              if (0.0 <= fVar27) {
                fVar41 = fVar27;
              }
              fVar27 = __real_3f800000;
              if (fVar41 <= __real_3f800000) {
                fVar27 = fVar41;
              }
              fVar27 = fVar27 + __real_bf000000;
              country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
              country_core::resources::roofs::roof_shape::Roof::height();
              fVar41 = __real_3f800000;
              fVar38 = 0.0;
              if (extraout_AL_02 == '\0') {
                fVar38 = __real_3eeb851f;
              }
              fVar27 = (float)((uint)fVar27 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
              fVar27 = powf(__real_3f800000 - (fVar27 + fVar27) / fVar49,fVar45);
              fVar27 = (extraout_XMM0_Da_06 + fVar38) * fVar27 + (fVar41 - fVar27) * fVar38;
              *(float *)(lVar10 + uVar13 * 4) = *(float *)(lVar10 + uVar13 * 4) + fVar27;
              if (extraout_AL_02 == '\0' && *(float *)(lVar8 + uVar13 * 4) == fVar41) {
                local_12c = fVar27;
                bVar7 = true;
              }
            }
            uVar13 = uVar13 + 1;
          } while (uVar9 != uVar13);
        }
        auVar39._8_4_ = 0xffffffff;
        auVar39._0_8_ = 0xffffffffffffffff;
        auVar39._12_4_ = 0xffffffff;
        if (local_1c8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar8 = local_100;
        puVar12 = local_118;
        fVar27 = local_12c;
        plVar23 = local_148;
        fVar49 = __real_3f75c290;
        fVar45 = __real_beeb851f;
        fVar41 = __real_c0200000;
        fVar40 = __real_3eeb851f;
        uVar48 = _UNK_14293ac6c;
        uVar47 = _UNK_14293ac68;
        uVar46 = _UNK_14293ac64;
        uVar44 = ___xmm_3f0000003f0000003f0000003f000000;
        fVar43 = __real_3fc00000;
        fVar42 = __real_3f000000;
        if (lStack_1b0 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar24 = local_f4;
        if (bVar7) {
          if (*(longlong *)(local_158 + -0x1b8) == 0) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          **(float **)(local_158 + -0x1c0) = fVar27 + **(float **)(local_158 + -0x1c0);
        }
        country_core::resources::roofs::roof_shape::Roof::height();
        *(undefined4 *)(local_158 + -400) = 1;
        *(float *)(local_158 + -0x18c) = fVar50 + extraout_XMM0_Da_07 + fVar40;
      }
    }
  } while( true );
LAB_14078becc:
  auVar34[0] = -(*pcVar1 == auVar39[0]);
  auVar34[1] = -(pcVar1[1] == auVar39[1]);
  auVar34[2] = -(pcVar1[2] == auVar39[2]);
  auVar34[3] = -(pcVar1[3] == auVar39[3]);
  auVar34[4] = -(pcVar1[4] == auVar39[4]);
  auVar34[5] = -(pcVar1[5] == auVar39[5]);
  auVar34[6] = -(pcVar1[6] == auVar39[6]);
  auVar34[7] = -(pcVar1[7] == auVar39[7]);
  auVar34[8] = -(pcVar1[8] == auVar39[8]);
  auVar34[9] = -(pcVar1[9] == auVar39[9]);
  auVar34[10] = -(pcVar1[10] == auVar39[10]);
  auVar34[0xb] = -(pcVar1[0xb] == auVar39[0xb]);
  auVar34[0xc] = -(pcVar1[0xc] == auVar39[0xc]);
  auVar34[0xd] = -(pcVar1[0xd] == auVar39[0xd]);
  auVar34[0xe] = -(pcVar1[0xe] == auVar39[0xe]);
  auVar34[0xf] = -(pcVar1[0xf] == auVar39[0xf]);
  if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 || (SUB161(auVar34 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar34 >> 0x17,0) & 1) != 0) || (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar34 >> 0x27,0) & 1) != 0) || (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar34 >> 0x37,0) & 1) != 0) || (SUB161(auVar34 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar34 >> 0x47,0) & 1) != 0) || (SUB161(auVar34 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar34 >> 0x57,0) & 1) != 0) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') {
LAB_14078c647:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar9 = uVar9 + lVar8 + 0x10;
  lVar8 = lVar8 + 0x10;
  goto LAB_14078be82;
}

