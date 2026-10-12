
void __cdecl system_wall_constructor::construct_gates::systems::construct_gates(void)

{
  longlong *plVar1;
  ulonglong *puVar2;
  undefined8 *puVar3;
  undefined4 *puVar4;
  float fVar5;
  float fVar6;
  ulonglong uVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 uVar17;
  longlong *plVar18;
  int iVar19;
  undefined8 *extraout_RAX;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong uVar23;
  undefined8 *extraout_RAX_02;
  char *pcVar24;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong uVar25;
  code *pcVar26;
  longlong lVar27;
  ulonglong uVar28;
  longlong lVar29;
  uint uVar30;
  undefined8 *in_R9;
  longlong lVar31;
  float *pfVar32;
  ulonglong uVar33;
  ushort uVar34;
  char *pcVar35;
  longlong lVar36;
  bool bVar37;
  byte bVar38;
  char cVar39;
  char cVar44;
  float fVar40;
  char cVar42;
  char cVar43;
  undefined1 auVar41 [16];
  float fVar45;
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  float fVar48;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar51;
  undefined1 unaff_XMM6_Bc;
  char cVar52;
  undefined1 unaff_XMM6_Bd;
  char cVar53;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar54;
  undefined1 unaff_XMM6_Bg;
  char cVar55;
  undefined1 unaff_XMM6_Bh;
  char cVar56;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar57;
  undefined1 unaff_XMM6_Bk;
  char cVar58;
  undefined1 unaff_XMM6_Bl;
  char cVar59;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar60;
  undefined1 unaff_XMM6_Bo;
  char cVar61;
  undefined1 unaff_XMM6_Bp;
  char cVar62;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000048;
  longlong **local_268;
  code *local_260;
  code *local_258;
  undefined8 local_248;
  ulonglong *local_240;
  longlong *local_238;
  undefined4 *local_230;
  longlong *local_228;
  longlong local_220;
  longlong local_218;
  longlong *local_210;
  undefined8 local_208;
  undefined4 *local_200;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  longlong local_1e0;
  ulonglong local_1d8;
  code *local_1d0;
  longlong *local_1c8;
  longlong local_1c0;
  longlong local_1b8;
  longlong *local_1b0;
  longlong local_1a0;
  ulonglong local_198;
  char *pcStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined **local_148;
  undefined8 local_140;
  longlong ***local_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined4 local_118;
  undefined4 local_114;
  char *local_110;
  longlong local_108;
  ulonglong local_100;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x3a) ||
       (iVar19 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX[2],0x3a), iVar19 != 0)) {
      local_198 = 0;
                    /* WARNING: Subroutine does not return */
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_220 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (local_220 != 0) {
      local_f1 = '\0';
      local_240 = (ulonglong *)*in_RCX;
      lVar22 = in_RCX[1];
      uVar7 = *local_240;
      uVar25 = 0;
      uVar20 = uVar7 - *(ulonglong *)(lVar22 + 0x18);
      if (uVar7 < *(ulonglong *)(lVar22 + 0x18)) {
        uVar20 = uVar25;
      }
      uVar23 = uVar7 - *(ulonglong *)(lVar22 + 0x38);
      if (uVar7 < *(ulonglong *)(lVar22 + 0x38)) {
        uVar23 = uVar25;
      }
      lVar36 = uVar20 * 0x80 + *(longlong *)(lVar22 + 8);
      uVar7 = *(ulonglong *)(lVar22 + 0x10) - uVar20;
      if (*(ulonglong *)(lVar22 + 0x10) < uVar20) {
        lVar36 = 8;
        uVar7 = uVar25;
      }
      lVar21 = uVar23 * 0x80 + *(longlong *)(lVar22 + 0x28);
      uVar20 = *(ulonglong *)(lVar22 + 0x30) - uVar23;
      if (*(ulonglong *)(lVar22 + 0x30) < uVar23) {
        lVar21 = 8;
        uVar20 = uVar25;
      }
      *local_240 = (*(longlong *)(lVar22 + 0x40) - uVar20) - uVar7;
      fVar16 = __real_be4ccccd;
      fVar15 = __real_3e4ccccd;
      fVar14 = __real_3f800000;
      fVar13 = __real_3f000000;
      local_218 = uVar20 * 0x80 + lVar21;
      local_228 = (longlong *)*in_stack_00000030;
      local_118 = *(undefined4 *)((longlong)in_RDX + 0x1c);
      local_1b0 = (longlong *)*in_RDX;
      local_230 = (undefined4 *)in_RDX[2];
      local_114 = *(undefined4 *)((longlong)in_R9 + 0x1c);
      local_208 = *in_R9;
      local_200 = (undefined4 *)in_R9[2];
      local_1f8 = *in_stack_00000028;
      local_1f0 = *in_stack_00000038;
      local_1e8 = *in_stack_00000048;
      iVar19 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                      CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      cVar51 = (char)((uint)iVar19 >> 8);
      cVar52 = (char)((uint)iVar19 >> 0x10);
      cVar53 = (char)((uint)iVar19 >> 0x18);
      iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                      CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      cVar54 = (char)((uint)iVar10 >> 8);
      cVar55 = (char)((uint)iVar10 >> 0x10);
      cVar56 = (char)((uint)iVar10 >> 0x18);
      iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                      CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      cVar57 = (char)((uint)iVar11 >> 8);
      cVar58 = (char)((uint)iVar11 >> 0x10);
      cVar59 = (char)((uint)iVar11 >> 0x18);
      iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                      CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
      cVar60 = (char)((uint)iVar12 >> 8);
      cVar61 = (char)((uint)iVar12 >> 0x10);
      cVar62 = (char)((uint)iVar12 >> 0x18);
      local_238 = local_228 + 4;
      local_210 = local_1b0 + 4;
      lVar22 = lVar21;
      lVar31 = lVar36;
      if (lVar36 == 0) goto LAB_1407565e0;
LAB_1407565ba:
      lVar22 = lVar21;
      if (lVar31 == uVar7 * 0x80 + lVar36) goto LAB_1407565e0;
      local_1b8 = lVar31 + 0x80;
      lVar22 = lVar31;
      local_1c0 = lVar21;
LAB_140756612:
      lVar21 = local_220;
      plVar18 = local_228;
      *local_240 = *local_240 + 1;
      plVar1 = (longlong *)(lVar22 + 0x68);
      local_1c8 = plVar1;
      if (local_228[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar31 = *plVar18;
        uVar20 = plVar18[1];
        bVar38 = (byte)(extraout_RAX_00 >> 0x39);
        auVar41 = ZEXT216(CONCAT11(bVar38,bVar38));
        auVar41 = pshuflw(auVar41,auVar41,0);
        lVar29 = 0;
        uVar25 = extraout_RAX_00;
        while( true ) {
          uVar25 = uVar20 & uVar25;
          pcVar35 = (char *)(lVar31 + uVar25);
          cVar39 = auVar41[0];
          auVar49[0] = -(*pcVar35 == cVar39);
          cVar42 = auVar41[1];
          auVar49[1] = -(pcVar35[1] == cVar42);
          cVar43 = auVar41[2];
          auVar49[2] = -(pcVar35[2] == cVar43);
          cVar44 = auVar41[3];
          auVar49[3] = -(pcVar35[3] == cVar44);
          auVar49[4] = -(pcVar35[4] == cVar39);
          auVar49[5] = -(pcVar35[5] == cVar42);
          auVar49[6] = -(pcVar35[6] == cVar43);
          auVar49[7] = -(pcVar35[7] == cVar44);
          auVar49[8] = -(pcVar35[8] == cVar39);
          auVar49[9] = -(pcVar35[9] == cVar42);
          auVar49[10] = -(pcVar35[10] == cVar43);
          auVar49[0xb] = -(pcVar35[0xb] == cVar44);
          auVar49[0xc] = -(pcVar35[0xc] == cVar39);
          auVar49[0xd] = -(pcVar35[0xd] == cVar42);
          auVar49[0xe] = -(pcVar35[0xe] == cVar43);
          auVar49[0xf] = -(pcVar35[0xf] == cVar44);
          uVar34 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar49[0xf] >> 7) << 0xf;
          uVar30 = (uint)uVar34;
          if (uVar34 != 0) {
            do {
              uVar8 = 0;
              for (uVar9 = uVar30; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              lVar27 = (uVar8 + uVar25 & uVar20) * -0x2a8;
              if (*plVar1 == *(longlong *)(lVar31 + -0x2a8 + lVar27)) {
                lVar29 = *(longlong *)(lVar31 + -0x180 + lVar27);
                if (lVar29 == 0) {
                    /* WARNING: Subroutine does not return */
                  core::option::unwrap_failed();
                }
                LOCK();
                bVar37 = *(char *)(lVar29 + 0x18) == '\0';
                if (bVar37) {
                  *(char *)(lVar29 + 0x18) = '\x01';
                }
                UNLOCK();
                if (!bVar37) {
                  local_108 = lVar29 + 0x18;
                  parking_lot::raw_mutex::RawMutex::lock_slow();
                }
                parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                plVar1 = local_1c8;
                if ((*(longlong *)(lVar29 + 0x10) * 4 + *(longlong *)(lVar29 + 8) == 4 ||
                     *(longlong *)(lVar29 + 0x10) == 0) && (*(longlong *)(lVar21 + 0x28) == 0)) {
                    /* WARNING: Subroutine does not return */
                  core::option::unwrap_failed();
                }
                *local_230 = local_118;
                if (local_1b0[3] == 0) goto LAB_140756880;
                uVar20 = lVar31 + lVar27;
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                lVar21 = *local_1b0;
                bVar38 = (byte)(extraout_RAX_01 >> 0x39);
                auVar41 = ZEXT216(CONCAT11(bVar38,bVar38));
                auVar41 = pshuflw(auVar41,auVar41,0);
                lVar31 = 0;
                uVar25 = extraout_RAX_01;
                goto LAB_1407567ee;
              }
              uVar34 = (ushort)(uVar30 - 1) & (ushort)uVar30;
              uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar34);
            } while (uVar34 != 0);
          }
          auVar46[0] = -(*pcVar35 == (char)iVar19);
          auVar46[1] = -(pcVar35[1] == cVar51);
          auVar46[2] = -(pcVar35[2] == cVar52);
          auVar46[3] = -(pcVar35[3] == cVar53);
          auVar46[4] = -(pcVar35[4] == (char)iVar10);
          auVar46[5] = -(pcVar35[5] == cVar54);
          auVar46[6] = -(pcVar35[6] == cVar55);
          auVar46[7] = -(pcVar35[7] == cVar56);
          auVar46[8] = -(pcVar35[8] == (char)iVar11);
          auVar46[9] = -(pcVar35[9] == cVar57);
          auVar46[10] = -(pcVar35[10] == cVar58);
          auVar46[0xb] = -(pcVar35[0xb] == cVar59);
          auVar46[0xc] = -(pcVar35[0xc] == (char)iVar12);
          auVar46[0xd] = -(pcVar35[0xd] == cVar60);
          auVar46[0xe] = -(pcVar35[0xe] == cVar61);
          auVar46[0xf] = -(pcVar35[0xf] == cVar62);
          if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar46 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar46 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x2f,0) & 1) != 0) || (SUB161(auVar46 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
          uVar25 = uVar25 + lVar29 + 0x10;
          lVar29 = lVar29 + 0x10;
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
LAB_1407567ee:
  uVar25 = local_1b0[1] & uVar25;
  pcVar35 = (char *)(lVar21 + uVar25);
  cVar39 = auVar41[0];
  auVar50[0] = -(*pcVar35 == cVar39);
  cVar42 = auVar41[1];
  auVar50[1] = -(pcVar35[1] == cVar42);
  cVar43 = auVar41[2];
  auVar50[2] = -(pcVar35[2] == cVar43);
  cVar44 = auVar41[3];
  auVar50[3] = -(pcVar35[3] == cVar44);
  auVar50[4] = -(pcVar35[4] == cVar39);
  auVar50[5] = -(pcVar35[5] == cVar42);
  auVar50[6] = -(pcVar35[6] == cVar43);
  auVar50[7] = -(pcVar35[7] == cVar44);
  auVar50[8] = -(pcVar35[8] == cVar39);
  auVar50[9] = -(pcVar35[9] == cVar42);
  auVar50[10] = -(pcVar35[10] == cVar43);
  auVar50[0xb] = -(pcVar35[0xb] == cVar44);
  auVar50[0xc] = -(pcVar35[0xc] == cVar39);
  auVar50[0xd] = -(pcVar35[0xd] == cVar42);
  auVar50[0xe] = -(pcVar35[0xe] == cVar43);
  auVar50[0xf] = -(pcVar35[0xf] == cVar44);
  uVar34 = (ushort)(SUB161(auVar50 >> 7,0) & 1) | (ushort)(SUB161(auVar50 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar50 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar50 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar50 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar50 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar50 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar50 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar50 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar50 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar50 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar50 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar50 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar50 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar50 >> 0x77,0) & 1) << 0xe | (ushort)(auVar50[0xf] >> 7) << 0xf;
  uVar30 = (uint)uVar34;
  if (uVar34 == 0) {
LAB_140756860:
    auVar47[0] = -(*pcVar35 == (char)iVar19);
    auVar47[1] = -(pcVar35[1] == cVar51);
    auVar47[2] = -(pcVar35[2] == cVar52);
    auVar47[3] = -(pcVar35[3] == cVar53);
    auVar47[4] = -(pcVar35[4] == (char)iVar10);
    auVar47[5] = -(pcVar35[5] == cVar54);
    auVar47[6] = -(pcVar35[6] == cVar55);
    auVar47[7] = -(pcVar35[7] == cVar56);
    auVar47[8] = -(pcVar35[8] == (char)iVar11);
    auVar47[9] = -(pcVar35[9] == cVar57);
    auVar47[10] = -(pcVar35[10] == cVar58);
    auVar47[0xb] = -(pcVar35[0xb] == cVar59);
    auVar47[0xc] = -(pcVar35[0xc] == (char)iVar12);
    auVar47[0xd] = -(pcVar35[0xd] == cVar60);
    auVar47[0xe] = -(pcVar35[0xe] == cVar61);
    auVar47[0xf] = -(pcVar35[0xf] == cVar62);
    if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) == 0 && (SUB161(auVar47 >> 0xf,0) & 1) == 0) &&
                    (SUB161(auVar47 >> 0x17,0) & 1) == 0) && (SUB161(auVar47 >> 0x1f,0) & 1) == 0)
                  && (SUB161(auVar47 >> 0x27,0) & 1) == 0) && (SUB161(auVar47 >> 0x2f,0) & 1) == 0)
                && (SUB161(auVar47 >> 0x37,0) & 1) == 0) && (SUB161(auVar47 >> 0x3f,0) & 1) == 0) &&
              (SUB161(auVar47 >> 0x47,0) & 1) == 0) && (SUB161(auVar47 >> 0x4f,0) & 1) == 0) &&
            (SUB161(auVar47 >> 0x57,0) & 1) == 0) && (SUB161(auVar47 >> 0x5f,0) & 1) == 0) &&
          (SUB161(auVar47 >> 0x67,0) & 1) == 0) && (SUB161(auVar47 >> 0x6f,0) & 1) == 0) &&
        (SUB161(auVar47 >> 0x77,0) & 1) == 0) && -1 < auVar47[0xf]) goto code_r0x00014075686c;
LAB_140756880:
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_268 = &local_1c8;
      local_260 = core::fmt::impl$74::fmt<country_core::resources::walls::WallId>;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_178 = *extraout_RAX_02;
      uStack_170 = extraout_RAX_02[1];
      uStack_14c = *(undefined4 *)(extraout_RAX_02 + 2);
      local_168 = 1;
      uStack_160 = "system_wall_constructor::construct_gates::systems";
      local_158 = 0x31;
      local_148 = &PTR_s_ConstructGateCmd___gates_entitie_142a7c808;
      local_140 = 1;
      local_138 = &local_268;
      local_130 = 1;
      uStack_128 = 0;
      local_198 = 0;
      pcStack_190 = "system_wall_constructor::construct_gates::systems";
      local_188 = 0x31;
      uStack_180 = 0;
      uStack_150 = 1;
      ::log::__private_api::impl$2::log();
    }
  }
  else {
LAB_140756810:
    uVar8 = 0;
    for (uVar9 = uVar30; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    uVar23 = uVar8 + uVar25 & local_1b0[1];
    local_100 = uVar20;
    if (*plVar1 != *(longlong *)(lVar21 + -0x20 + uVar23 * -0x20)) goto code_r0x00014075683a;
    pcVar35 = *(char **)(lVar22 + 0x20);
    uVar20 = *(ulonglong *)(lVar22 + 0x28);
    if (uVar20 == 0) {
      fVar40 = 0.0;
    }
    else {
      if (uVar20 == 1) {
        fVar40 = 0.0;
        uVar25 = 0;
      }
      else {
        pfVar32 = (float *)(pcVar35 + 0x10);
        fVar40 = 0.0;
        uVar25 = 0;
        do {
          fVar40 = (float)(~-(uint)(pfVar32[-3] < 0.0) & (uint)fVar40 |
                          (uint)(fVar40 + fVar14) & -(uint)(pfVar32[-3] < 0.0));
          fVar40 = (float)(-(uint)(*pfVar32 < 0.0) & (uint)(fVar40 + fVar14) |
                          ~-(uint)(*pfVar32 < 0.0) & (uint)fVar40);
          uVar25 = uVar25 + 2;
          pfVar32 = pfVar32 + 6;
        } while ((uVar20 & 0xfffffffffffffffe) != uVar25);
      }
      if ((uVar20 & 1) != 0) {
        fVar40 = (float)(~-(uint)(*(float *)(pcVar35 + uVar25 * 0xc + 4) < 0.0) & (uint)fVar40 |
                        (uint)(fVar40 + fVar14) &
                        -(uint)(*(float *)(pcVar35 + uVar25 * 0xc + 4) < 0.0));
      }
    }
    lVar31 = uVar23 * -0x20;
    local_108 = lVar21 + lVar31;
    local_1a0 = lVar31 + lVar21 + -0x18;
    if (fVar40 / (float)(longlong)uVar20 <= fVar13) {
      *local_200 = local_114;
      construct_wooden_gate_doors();
      lVar21 = local_108;
      uVar20 = local_188;
      lVar22 = *(longlong *)(local_108 + -8);
      uVar25 = local_198;
      pcVar35 = pcStack_190;
      if ((ulonglong)(*(longlong *)(local_108 + -0x18) - lVar22) < local_188) {
        local_110 = pcStack_190;
        local_100 = local_198;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar22 = *(longlong *)(lVar21 + -8);
        uVar25 = local_100;
        pcVar35 = local_110;
      }
      memcpy((void *)(lVar22 * 8 + *(longlong *)(lVar21 + -0x10)),pcVar35,uVar20 * 8);
      *(ulonglong *)(lVar21 + -8) = lVar22 + uVar20;
      if (uVar25 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      uVar25 = *(ulonglong *)(lVar22 + 0x10);
      if (uVar20 <= *(ulonglong *)(lVar22 + 0x10)) {
        uVar25 = uVar20;
      }
      if (uVar25 == 0) goto LAB_140756e20;
      lVar21 = *(longlong *)(lVar22 + 8);
      lVar22 = uVar20 - 1;
      fVar40 = 0.0;
      uVar23 = 0;
      uVar28 = 0;
      do {
        lVar31 = 0;
        pcVar24 = pcVar35;
        while ((uVar23 + lVar31 == 0 || (lVar22 - uVar23 == lVar31))) {
          pcVar24 = pcVar24 + 0xc;
          lVar27 = uVar28 + lVar31;
          lVar31 = lVar31 + 1;
          if (uVar25 <= lVar27 + 1U) goto LAB_140756e20;
        }
        uVar33 = uVar23 + lVar31 + 1;
        if (uVar20 <= uVar33) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar5 = *(float *)(pcVar24 + uVar28 * 0xc);
        fVar6 = *(float *)(pcVar24 + uVar28 * 0xc + 8);
        fVar45 = fVar5 - *(float *)(pcVar24 + uVar23 * 0xc + 0xc);
        fVar48 = fVar6 - *(float *)(pcVar24 + uVar23 * 0xc + 0x14);
        fVar40 = fVar40 + SQRT(fVar48 * fVar48 + fVar45 * fVar45);
        uVar23 = uVar33;
        if (fVar13 < fVar40) {
          fVar40 = *(float *)(pcVar24 + uVar28 * 0xc + 4);
          fVar45 = *(float *)(lVar21 + uVar28 * 4 + lVar31 * 4);
          uStack_160 = (char *)0x7e4dd4fe71bf7165;
          local_168 = local_168 & 0xffffffff00000000;
          local_1e0 = lVar22;
          local_1d8 = uVar33;
          local_110 = pcVar35;
          local_100 = uVar20;
          ::utils::xz_rotation_bad();
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          local_158 = CONCAT71(local_158._1_7_,3);
          local_188 = CONCAT44(fVar40 + fVar16,fVar5);
          uStack_180 = CONCAT44(0x3f800000,fVar6);
          local_178 = CONCAT44(0x3f800000,fVar45 + fVar15);
          bevy_ecs::system::commands::Commands::spawn_empty();
          uVar17 = local_248;
          pcVar26 = local_260;
          if (local_260 == (code *)0x0) {
            pcVar26 = local_258;
          }
          lVar22 = *(longlong *)(pcVar26 + 0x10);
          if ((ulonglong)(*(longlong *)pcVar26 - lVar22) < 0x88) {
            local_1d0 = pcVar26;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            pcVar26 = local_1d0;
          }
          lVar27 = *(longlong *)(pcVar26 + 8);
          *(code **)(lVar27 + lVar22) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          puVar2 = (ulonglong *)(lVar27 + 8 + lVar22);
          *puVar2 = local_198;
          puVar2[1] = (ulonglong)pcStack_190;
          puVar2 = (ulonglong *)(lVar27 + 0x18 + lVar22);
          *puVar2 = local_188;
          puVar2[1] = uStack_180;
          puVar3 = (undefined8 *)(lVar27 + 0x28 + lVar22);
          *puVar3 = local_178;
          puVar3[1] = uStack_170;
          puVar4 = (undefined4 *)(lVar27 + 0x38 + lVar22);
          *puVar4 = (undefined4)local_168;
          puVar4[1] = local_168._4_4_;
          puVar4[2] = (undefined4)uStack_160;
          puVar4[3] = uStack_160._4_4_;
          puVar3 = (undefined8 *)(lVar27 + 0x48 + lVar22);
          *puVar3 = local_158;
          puVar3[1] = CONCAT44(uStack_14c,uStack_150);
          *(undefined1 *)(lVar27 + 0x58 + lVar22) = 0;
          *(undefined8 *)(lVar27 + 0x68 + lVar22) = uVar17;
          *(code **)(lVar27 + 0x78 + lVar22) = bevy_ecs::error::handler::panic;
          *(longlong *)(pcVar26 + 0x10) = lVar22 + 0x88;
          lVar22 = *(longlong *)(local_108 + -8);
          if (lVar22 == *(longlong *)(local_108 + -0x18)) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(undefined8 *)(*(longlong *)(local_108 + -0x10) + lVar22 * 8) = uVar17;
          *(longlong *)(local_108 + -8) = lVar22 + 1;
          fVar40 = 0.0;
          uVar20 = local_100;
          lVar22 = local_1e0;
          uVar23 = local_1d8;
          pcVar35 = local_110;
        }
        uVar28 = uVar28 + lVar31 + 1;
      } while (uVar28 < uVar25);
    }
  }
  goto LAB_140756e20;
code_r0x00014075683a:
  uVar34 = (ushort)(uVar30 - 1) & (ushort)uVar30;
  uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar34);
  if (uVar34 == 0) goto LAB_140756860;
  goto LAB_140756810;
code_r0x00014075686c:
  uVar25 = uVar25 + lVar31 + 0x10;
  lVar31 = lVar31 + 0x10;
  goto LAB_1407567ee;
LAB_140756e20:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar37 = *(char *)(lVar29 + 0x18) == '\x01';
  if (bVar37) {
    *(char *)(lVar29 + 0x18) = '\0';
  }
  UNLOCK();
  if (!bVar37) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  lVar21 = local_1c0;
  lVar22 = local_1c0;
  lVar31 = local_1b8;
  if (local_1b8 != 0) goto LAB_1407565ba;
LAB_1407565e0:
  if ((lVar22 == 0) || (lVar22 == local_218)) {
    if ((local_f1 != '\0') &&
       (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0)) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_178 = *extraout_RAX_03;
      uStack_170 = extraout_RAX_03[1];
      uStack_14c = *(undefined4 *)(extraout_RAX_03 + 2);
      local_168 = 1;
      uStack_160 = "system_wall_constructor::construct_gates::systems";
      local_158 = 0x31;
      local_148 = &PTR_s_Gate_constructor_ran_out_of_bric_142a7c758;
      local_140 = 1;
      local_138 = (longlong ***)0x8;
      local_198 = 0;
      local_130 = 0;
      uStack_128 = 0;
      pcStack_190 = "system_wall_constructor::construct_gates::systems";
      local_188 = 0x31;
      uStack_180 = 0;
      uStack_150 = 1;
      ::log::__private_api::impl$2::log();
    }
    return;
  }
  local_1c0 = lVar22 + 0x80;
  local_1b8 = 0;
  goto LAB_140756612;
}

