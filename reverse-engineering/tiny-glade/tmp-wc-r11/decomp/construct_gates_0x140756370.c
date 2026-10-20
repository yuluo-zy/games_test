// name: construct_gates
// addr: 0x140756370
// size: 3315
// signature: void __cdecl construct_gates(void)


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
  undefined8 uVar13;
  longlong *plVar14;
  int iVar15;
  undefined8 *extraout_RAX;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong uVar19;
  undefined8 *extraout_RAX_02;
  char *pcVar20;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong uVar21;
  code *pcVar22;
  longlong lVar23;
  ulonglong uVar24;
  longlong lVar25;
  uint uVar26;
  undefined8 *in_R9;
  longlong lVar27;
  float *pfVar28;
  ulonglong uVar29;
  ushort uVar30;
  char *pcVar31;
  longlong lVar32;
  bool bVar33;
  byte bVar34;
  char cVar35;
  char cVar40;
  float fVar36;
  char cVar38;
  char cVar39;
  undefined1 auVar37 [16];
  float fVar41;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float fVar44;
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar47;
  undefined1 unaff_XMM6_Bc;
  char cVar48;
  undefined1 unaff_XMM6_Bd;
  char cVar49;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar50;
  undefined1 unaff_XMM6_Bg;
  char cVar51;
  undefined1 unaff_XMM6_Bh;
  char cVar52;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar53;
  undefined1 unaff_XMM6_Bk;
  char cVar54;
  undefined1 unaff_XMM6_Bl;
  char cVar55;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar56;
  undefined1 unaff_XMM6_Bo;
  char cVar57;
  undefined1 unaff_XMM6_Bp;
  char cVar58;
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
       (iVar15 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX[2],0x3a), iVar15 != 0)) {
      local_198 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_220 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (local_220 != 0) {
      local_f1 = '\0';
      local_240 = (ulonglong *)*in_RCX;
      lVar18 = in_RCX[1];
      uVar7 = *local_240;
      uVar21 = 0;
      uVar16 = uVar7 - *(ulonglong *)(lVar18 + 0x18);
      if (uVar7 < *(ulonglong *)(lVar18 + 0x18)) {
        uVar16 = uVar21;
      }
      uVar19 = uVar7 - *(ulonglong *)(lVar18 + 0x38);
      if (uVar7 < *(ulonglong *)(lVar18 + 0x38)) {
        uVar19 = uVar21;
      }
      lVar32 = uVar16 * 0x80 + *(longlong *)(lVar18 + 8);
      uVar7 = *(ulonglong *)(lVar18 + 0x10) - uVar16;
      if (*(ulonglong *)(lVar18 + 0x10) < uVar16) {
        lVar32 = 8;
        uVar7 = uVar21;
      }
      lVar17 = uVar19 * 0x80 + *(longlong *)(lVar18 + 0x28);
      uVar16 = *(ulonglong *)(lVar18 + 0x30) - uVar19;
      if (*(ulonglong *)(lVar18 + 0x30) < uVar19) {
        lVar17 = 8;
        uVar16 = uVar21;
      }
      *local_240 = (*(longlong *)(lVar18 + 0x40) - uVar16) - uVar7;
      local_218 = uVar16 * 0x80 + lVar17;
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
      iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                      CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      cVar47 = (char)((uint)iVar15 >> 8);
      cVar48 = (char)((uint)iVar15 >> 0x10);
      cVar49 = (char)((uint)iVar15 >> 0x18);
      iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                      CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      cVar50 = (char)((uint)iVar10 >> 8);
      cVar51 = (char)((uint)iVar10 >> 0x10);
      cVar52 = (char)((uint)iVar10 >> 0x18);
      iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                      CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      cVar53 = (char)((uint)iVar11 >> 8);
      cVar54 = (char)((uint)iVar11 >> 0x10);
      cVar55 = (char)((uint)iVar11 >> 0x18);
      iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                      CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
      cVar56 = (char)((uint)iVar12 >> 8);
      cVar57 = (char)((uint)iVar12 >> 0x10);
      cVar58 = (char)((uint)iVar12 >> 0x18);
      local_238 = local_228 + 4;
      local_210 = local_1b0 + 4;
      lVar18 = lVar17;
      lVar27 = lVar32;
      if (lVar32 == 0) goto LAB_1407565e0;
LAB_1407565ba:
      lVar18 = lVar17;
      if (lVar27 == uVar7 * 0x80 + lVar32) goto LAB_1407565e0;
      local_1b8 = lVar27 + 0x80;
      lVar18 = lVar27;
      local_1c0 = lVar17;
LAB_140756612:
      lVar17 = local_220;
      plVar14 = local_228;
      *local_240 = *local_240 + 1;
      plVar1 = (longlong *)(lVar18 + 0x68);
      local_1c8 = plVar1;
      if (local_228[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar27 = *plVar14;
        uVar16 = plVar14[1];
        bVar34 = (byte)(extraout_RAX_00 >> 0x39);
        auVar37 = ZEXT216(CONCAT11(bVar34,bVar34));
        auVar37 = pshuflw(auVar37,auVar37,0);
        lVar25 = 0;
        uVar21 = extraout_RAX_00;
        while( true ) {
          uVar21 = uVar16 & uVar21;
          pcVar31 = (char *)(lVar27 + uVar21);
          cVar35 = auVar37[0];
          auVar45[0] = -(*pcVar31 == cVar35);
          cVar38 = auVar37[1];
          auVar45[1] = -(pcVar31[1] == cVar38);
          cVar39 = auVar37[2];
          auVar45[2] = -(pcVar31[2] == cVar39);
          cVar40 = auVar37[3];
          auVar45[3] = -(pcVar31[3] == cVar40);
          auVar45[4] = -(pcVar31[4] == cVar35);
          auVar45[5] = -(pcVar31[5] == cVar38);
          auVar45[6] = -(pcVar31[6] == cVar39);
          auVar45[7] = -(pcVar31[7] == cVar40);
          auVar45[8] = -(pcVar31[8] == cVar35);
          auVar45[9] = -(pcVar31[9] == cVar38);
          auVar45[10] = -(pcVar31[10] == cVar39);
          auVar45[0xb] = -(pcVar31[0xb] == cVar40);
          auVar45[0xc] = -(pcVar31[0xc] == cVar35);
          auVar45[0xd] = -(pcVar31[0xd] == cVar38);
          auVar45[0xe] = -(pcVar31[0xe] == cVar39);
          auVar45[0xf] = -(pcVar31[0xf] == cVar40);
          uVar30 = (ushort)(SUB161(auVar45 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar45[0xf] >> 7) << 0xf;
          uVar26 = (uint)uVar30;
          if (uVar30 != 0) {
            do {
              uVar8 = 0;
              for (uVar9 = uVar26; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              lVar23 = (uVar8 + uVar21 & uVar16) * -0x2a8;
              if (*plVar1 == *(longlong *)(lVar27 + -0x2a8 + lVar23)) {
                lVar25 = *(longlong *)(lVar27 + -0x180 + lVar23);
                if (lVar25 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                LOCK();
                bVar33 = *(char *)(lVar25 + 0x18) == '\0';
                if (bVar33) {
                  *(char *)(lVar25 + 0x18) = '\x01';
                }
                UNLOCK();
                if (!bVar33) {
                  local_108 = lVar25 + 0x18;
                  parking_lot::raw_mutex::RawMutex::lock_slow();
                }
                parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                plVar1 = local_1c8;
                if ((*(longlong *)(lVar25 + 0x10) * 4 + *(longlong *)(lVar25 + 8) == 4 ||
                     *(longlong *)(lVar25 + 0x10) == 0) && (*(longlong *)(lVar17 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                *local_230 = local_118;
                if (local_1b0[3] == 0) goto LAB_140756880;
                uVar16 = lVar27 + lVar23;
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                lVar17 = *local_1b0;
                bVar34 = (byte)(extraout_RAX_01 >> 0x39);
                auVar37 = ZEXT216(CONCAT11(bVar34,bVar34));
                auVar37 = pshuflw(auVar37,auVar37,0);
                lVar27 = 0;
                uVar21 = extraout_RAX_01;
                goto LAB_1407567ee;
              }
              uVar30 = (ushort)(uVar26 - 1) & (ushort)uVar26;
              uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar30);
            } while (uVar30 != 0);
          }
          auVar42[0] = -(*pcVar31 == (char)iVar15);
          auVar42[1] = -(pcVar31[1] == cVar47);
          auVar42[2] = -(pcVar31[2] == cVar48);
          auVar42[3] = -(pcVar31[3] == cVar49);
          auVar42[4] = -(pcVar31[4] == (char)iVar10);
          auVar42[5] = -(pcVar31[5] == cVar50);
          auVar42[6] = -(pcVar31[6] == cVar51);
          auVar42[7] = -(pcVar31[7] == cVar52);
          auVar42[8] = -(pcVar31[8] == (char)iVar11);
          auVar42[9] = -(pcVar31[9] == cVar53);
          auVar42[10] = -(pcVar31[10] == cVar54);
          auVar42[0xb] = -(pcVar31[0xb] == cVar55);
          auVar42[0xc] = -(pcVar31[0xc] == (char)iVar12);
          auVar42[0xd] = -(pcVar31[0xd] == cVar56);
          auVar42[0xe] = -(pcVar31[0xe] == cVar57);
          auVar42[0xf] = -(pcVar31[0xf] == cVar58);
          if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar42 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar42 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar42 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar42 >> 0x2f,0) & 1) != 0) || (SUB161(auVar42 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar42 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar42 >> 0x47,0) & 1) != 0) || (SUB161(auVar42 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar42 >> 0x57,0) & 1) != 0) || (SUB161(auVar42 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar42 >> 0x67,0) & 1) != 0) || (SUB161(auVar42 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar42 >> 0x77,0) & 1) != 0) || auVar42[0xf] < '\0') break;
          uVar21 = uVar21 + lVar25 + 0x10;
          lVar25 = lVar25 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_1407567ee:
  uVar21 = local_1b0[1] & uVar21;
  pcVar31 = (char *)(lVar17 + uVar21);
  cVar35 = auVar37[0];
  auVar46[0] = -(*pcVar31 == cVar35);
  cVar38 = auVar37[1];
  auVar46[1] = -(pcVar31[1] == cVar38);
  cVar39 = auVar37[2];
  auVar46[2] = -(pcVar31[2] == cVar39);
  cVar40 = auVar37[3];
  auVar46[3] = -(pcVar31[3] == cVar40);
  auVar46[4] = -(pcVar31[4] == cVar35);
  auVar46[5] = -(pcVar31[5] == cVar38);
  auVar46[6] = -(pcVar31[6] == cVar39);
  auVar46[7] = -(pcVar31[7] == cVar40);
  auVar46[8] = -(pcVar31[8] == cVar35);
  auVar46[9] = -(pcVar31[9] == cVar38);
  auVar46[10] = -(pcVar31[10] == cVar39);
  auVar46[0xb] = -(pcVar31[0xb] == cVar40);
  auVar46[0xc] = -(pcVar31[0xc] == cVar35);
  auVar46[0xd] = -(pcVar31[0xd] == cVar38);
  auVar46[0xe] = -(pcVar31[0xe] == cVar39);
  auVar46[0xf] = -(pcVar31[0xf] == cVar40);
  uVar30 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe | (ushort)(auVar46[0xf] >> 7) << 0xf;
  uVar26 = (uint)uVar30;
  if (uVar30 == 0) {
LAB_140756860:
    auVar43[0] = -(*pcVar31 == (char)iVar15);
    auVar43[1] = -(pcVar31[1] == cVar47);
    auVar43[2] = -(pcVar31[2] == cVar48);
    auVar43[3] = -(pcVar31[3] == cVar49);
    auVar43[4] = -(pcVar31[4] == (char)iVar10);
    auVar43[5] = -(pcVar31[5] == cVar50);
    auVar43[6] = -(pcVar31[6] == cVar51);
    auVar43[7] = -(pcVar31[7] == cVar52);
    auVar43[8] = -(pcVar31[8] == (char)iVar11);
    auVar43[9] = -(pcVar31[9] == cVar53);
    auVar43[10] = -(pcVar31[10] == cVar54);
    auVar43[0xb] = -(pcVar31[0xb] == cVar55);
    auVar43[0xc] = -(pcVar31[0xc] == (char)iVar12);
    auVar43[0xd] = -(pcVar31[0xd] == cVar56);
    auVar43[0xe] = -(pcVar31[0xe] == cVar57);
    auVar43[0xf] = -(pcVar31[0xf] == cVar58);
    if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) == 0 && (SUB161(auVar43 >> 0xf,0) & 1) == 0) &&
                    (SUB161(auVar43 >> 0x17,0) & 1) == 0) && (SUB161(auVar43 >> 0x1f,0) & 1) == 0)
                  && (SUB161(auVar43 >> 0x27,0) & 1) == 0) && (SUB161(auVar43 >> 0x2f,0) & 1) == 0)
                && (SUB161(auVar43 >> 0x37,0) & 1) == 0) && (SUB161(auVar43 >> 0x3f,0) & 1) == 0) &&
              (SUB161(auVar43 >> 0x47,0) & 1) == 0) && (SUB161(auVar43 >> 0x4f,0) & 1) == 0) &&
            (SUB161(auVar43 >> 0x57,0) & 1) == 0) && (SUB161(auVar43 >> 0x5f,0) & 1) == 0) &&
          (SUB161(auVar43 >> 0x67,0) & 1) == 0) && (SUB161(auVar43 >> 0x6f,0) & 1) == 0) &&
        (SUB161(auVar43 >> 0x77,0) & 1) == 0) && -1 < auVar43[0xf]) goto code_r0x00014075686c;
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
    for (uVar9 = uVar26; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    uVar19 = uVar8 + uVar21 & local_1b0[1];
    local_100 = uVar16;
    if (*plVar1 != *(longlong *)(lVar17 + -0x20 + uVar19 * -0x20)) goto code_r0x00014075683a;
    pcVar31 = *(char **)(lVar18 + 0x20);
    uVar16 = *(ulonglong *)(lVar18 + 0x28);
    if (uVar16 == 0) {
      fVar36 = 0.0;
    }
    else {
      if (uVar16 == 1) {
        fVar36 = 0.0;
        uVar21 = 0;
      }
      else {
        pfVar28 = (float *)(pcVar31 + 0x10);
        fVar36 = 0.0;
        uVar21 = 0;
        do {
          fVar36 = (float)(~-(uint)(pfVar28[-3] < 0.0) & (uint)fVar36 |
                          (uint)(fVar36 + 1.0) & -(uint)(pfVar28[-3] < 0.0));
          fVar36 = (float)(-(uint)(*pfVar28 < 0.0) & (uint)(fVar36 + 1.0) |
                          ~-(uint)(*pfVar28 < 0.0) & (uint)fVar36);
          uVar21 = uVar21 + 2;
          pfVar28 = pfVar28 + 6;
        } while ((uVar16 & 0xfffffffffffffffe) != uVar21);
      }
      if ((uVar16 & 1) != 0) {
        fVar36 = (float)(~-(uint)(*(float *)(pcVar31 + uVar21 * 0xc + 4) < 0.0) & (uint)fVar36 |
                        (uint)(fVar36 + 1.0) & -(uint)(*(float *)(pcVar31 + uVar21 * 0xc + 4) < 0.0)
                        );
      }
    }
    lVar27 = uVar19 * -0x20;
    local_108 = lVar17 + lVar27;
    local_1a0 = lVar27 + lVar17 + -0x18;
    if (fVar36 / (float)(longlong)uVar16 <= 0.5) {
      *local_200 = local_114;
      construct_wooden_gate_doors();
      lVar17 = local_108;
      uVar16 = local_188;
      lVar18 = *(longlong *)(local_108 + -8);
      uVar21 = local_198;
      pcVar31 = pcStack_190;
      if ((ulonglong)(*(longlong *)(local_108 + -0x18) - lVar18) < local_188) {
        local_110 = pcStack_190;
        local_100 = local_198;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar18 = *(longlong *)(lVar17 + -8);
        uVar21 = local_100;
        pcVar31 = local_110;
      }
      memcpy((void *)(lVar18 * 8 + *(longlong *)(lVar17 + -0x10)),pcVar31,uVar16 * 8);
      *(ulonglong *)(lVar17 + -8) = lVar18 + uVar16;
      if (uVar21 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      uVar21 = *(ulonglong *)(lVar18 + 0x10);
      if (uVar16 <= *(ulonglong *)(lVar18 + 0x10)) {
        uVar21 = uVar16;
      }
      if (uVar21 == 0) goto LAB_140756e20;
      lVar17 = *(longlong *)(lVar18 + 8);
      lVar18 = uVar16 - 1;
      fVar36 = 0.0;
      uVar19 = 0;
      uVar24 = 0;
      do {
        lVar27 = 0;
        pcVar20 = pcVar31;
        while ((uVar19 + lVar27 == 0 || (lVar18 - uVar19 == lVar27))) {
          pcVar20 = pcVar20 + 0xc;
          lVar23 = uVar24 + lVar27;
          lVar27 = lVar27 + 1;
          if (uVar21 <= lVar23 + 1U) goto LAB_140756e20;
        }
        uVar29 = uVar19 + lVar27 + 1;
        if (uVar16 <= uVar29) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar5 = *(float *)(pcVar20 + uVar24 * 0xc);
        fVar6 = *(float *)(pcVar20 + uVar24 * 0xc + 8);
        fVar41 = fVar5 - *(float *)(pcVar20 + uVar19 * 0xc + 0xc);
        fVar44 = fVar6 - *(float *)(pcVar20 + uVar19 * 0xc + 0x14);
        fVar36 = fVar36 + SQRT(fVar44 * fVar44 + fVar41 * fVar41);
        uVar19 = uVar29;
        if (0.5 < fVar36) {
          fVar36 = *(float *)(pcVar20 + uVar24 * 0xc + 4);
          fVar41 = *(float *)(lVar17 + uVar24 * 4 + lVar27 * 4);
          uStack_160 = (char *)0x7e4dd4fe71bf7165;
          local_168 = local_168 & 0xffffffff00000000;
          local_1e0 = lVar18;
          local_1d8 = uVar29;
          local_110 = pcVar31;
          local_100 = uVar16;
          ::utils::xz_rotation_bad();
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          local_158 = CONCAT71(local_158._1_7_,3);
          local_188 = CONCAT44(fVar36 + -0.2,fVar5);
          uStack_180 = CONCAT44(0x3f800000,fVar6);
          local_178 = CONCAT44(0x3f800000,fVar41 + 0.2);
          bevy_ecs::system::commands::Commands::spawn_empty();
          uVar13 = local_248;
          pcVar22 = local_260;
          if (local_260 == (code *)0x0) {
            pcVar22 = local_258;
          }
          lVar18 = *(longlong *)(pcVar22 + 0x10);
          if ((ulonglong)(*(longlong *)pcVar22 - lVar18) < 0x88) {
            local_1d0 = pcVar22;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            pcVar22 = local_1d0;
          }
          lVar23 = *(longlong *)(pcVar22 + 8);
          *(code **)(lVar23 + lVar18) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          puVar2 = (ulonglong *)(lVar23 + 8 + lVar18);
          *puVar2 = local_198;
          puVar2[1] = (ulonglong)pcStack_190;
          puVar2 = (ulonglong *)(lVar23 + 0x18 + lVar18);
          *puVar2 = local_188;
          puVar2[1] = uStack_180;
          puVar3 = (undefined8 *)(lVar23 + 0x28 + lVar18);
          *puVar3 = local_178;
          puVar3[1] = uStack_170;
          puVar4 = (undefined4 *)(lVar23 + 0x38 + lVar18);
          *puVar4 = (undefined4)local_168;
          puVar4[1] = local_168._4_4_;
          puVar4[2] = (undefined4)uStack_160;
          puVar4[3] = uStack_160._4_4_;
          puVar3 = (undefined8 *)(lVar23 + 0x48 + lVar18);
          *puVar3 = local_158;
          puVar3[1] = CONCAT44(uStack_14c,uStack_150);
          *(undefined1 *)(lVar23 + 0x58 + lVar18) = 0;
          *(undefined8 *)(lVar23 + 0x68 + lVar18) = uVar13;
          *(code **)(lVar23 + 0x78 + lVar18) = bevy_ecs::error::handler::panic;
          *(longlong *)(pcVar22 + 0x10) = lVar18 + 0x88;
          lVar18 = *(longlong *)(local_108 + -8);
          if (lVar18 == *(longlong *)(local_108 + -0x18)) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(undefined8 *)(*(longlong *)(local_108 + -0x10) + lVar18 * 8) = uVar13;
          *(longlong *)(local_108 + -8) = lVar18 + 1;
          fVar36 = 0.0;
          uVar16 = local_100;
          lVar18 = local_1e0;
          uVar19 = local_1d8;
          pcVar31 = local_110;
        }
        uVar24 = uVar24 + lVar27 + 1;
      } while (uVar24 < uVar21);
    }
  }
  goto LAB_140756e20;
code_r0x00014075683a:
  uVar30 = (ushort)(uVar26 - 1) & (ushort)uVar26;
  uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar30);
  if (uVar30 == 0) goto LAB_140756860;
  goto LAB_140756810;
code_r0x00014075686c:
  uVar21 = uVar21 + lVar27 + 0x10;
  lVar27 = lVar27 + 0x10;
  goto LAB_1407567ee;
LAB_140756e20:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar33 = *(char *)(lVar25 + 0x18) == '\x01';
  if (bVar33) {
    *(char *)(lVar25 + 0x18) = '\0';
  }
  UNLOCK();
  if (!bVar33) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  lVar17 = local_1c0;
  lVar18 = local_1c0;
  lVar27 = local_1b8;
  if (local_1b8 != 0) goto LAB_1407565ba;
LAB_1407565e0:
  if ((lVar18 == 0) || (lVar18 == local_218)) {
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
  local_1c0 = lVar18 + 0x80;
  local_1b8 = 0;
  goto LAB_140756612;
}

