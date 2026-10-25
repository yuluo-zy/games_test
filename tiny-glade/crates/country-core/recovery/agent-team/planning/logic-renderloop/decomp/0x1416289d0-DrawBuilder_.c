// 0x1416289d0 bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder> size=3639

void __cdecl
country_core::render::pass_builder::bind_resources::impl$0::
bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>(void)

{
  uint *puVar1;
  char *pcVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  undefined1 (*pauVar5) [16];
  uint uVar6;
  undefined8 *puVar7;
  undefined *puVar8;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  int iVar9;
  undefined8 *extraout_RAX;
  undefined8 *puVar10;
  undefined8 *extraout_RAX_00;
  undefined8 *puVar11;
  void *in_RCX;
  ulonglong uVar12;
  void *in_RDX;
  ulonglong uVar13;
  longlong lVar14;
  uint uVar15;
  byte bVar16;
  byte bVar17;
  uint uVar18;
  longlong in_R8;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  ushort uVar22;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  longlong local_250;
  longlong local_248;
  undefined8 *local_240;
  ulonglong local_238;
  undefined8 *local_230;
  longlong local_198;
  ulonglong local_190;
  char local_172;
  longlong local_168;
  undefined *local_158;
  longlong local_150;
  undefined8 *local_148;
  ulonglong local_140;
  undefined8 *local_138;
  longlong local_a0;
  ulonglong local_98;
  char local_7a;
  longlong local_70;
  void *local_60;
  void *local_58;
  int local_50;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_60 = in_RCX;
  local_58 = in_RDX;
  if (*(longlong *)(in_R8 + 0x88) != 0) {
    lVar21 = *(longlong *)(in_R8 + 0x70);
    uVar12 = *(ulonglong *)(in_R8 + 0x78);
    uVar13 = uVar12 & 0x65cfda6a55a841ac;
    lVar20 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar21 + uVar13);
      auVar30[0] = -(*pcVar2 == '2');
      auVar30[1] = -(pcVar2[1] == '2');
      auVar30[2] = -(pcVar2[2] == '2');
      auVar30[3] = -(pcVar2[3] == '2');
      auVar30[4] = -(pcVar2[4] == '2');
      auVar30[5] = -(pcVar2[5] == '2');
      auVar30[6] = -(pcVar2[6] == '2');
      auVar30[7] = -(pcVar2[7] == '2');
      auVar30[8] = -(pcVar2[8] == '2');
      auVar30[9] = -(pcVar2[9] == '2');
      auVar30[10] = -(pcVar2[10] == '2');
      auVar30[0xb] = -(pcVar2[0xb] == '2');
      auVar30[0xc] = -(pcVar2[0xc] == '2');
      auVar30[0xd] = -(pcVar2[0xd] == '2');
      auVar30[0xe] = -(pcVar2[0xe] == '2');
      auVar30[0xf] = -(pcVar2[0xf] == '2');
      uVar22 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar22;
      while (uVar22 != 0) {
        uVar18 = 0;
        for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar18 = uVar18 + 1;
        }
        uVar19 = uVar18 + uVar13 & uVar12;
        if ((*(longlong *)(lVar21 + -0x18 + uVar19 * -0x18) == -0x7dd9338791986cab) &&
           (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x18) == 0x65cfda6a55a841ac)) {
          uVar12 = *(ulonglong *)(lVar21 + uVar19 * -0x18 + -8);
          goto LAB_141628bbb;
        }
        uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
      }
      auVar23[0] = -(*pcVar2 == -1);
      auVar23[1] = -(pcVar2[1] == -1);
      auVar23[2] = -(pcVar2[2] == -1);
      auVar23[3] = -(pcVar2[3] == -1);
      auVar23[4] = -(pcVar2[4] == -1);
      auVar23[5] = -(pcVar2[5] == -1);
      auVar23[6] = -(pcVar2[6] == -1);
      auVar23[7] = -(pcVar2[7] == -1);
      auVar23[8] = -(pcVar2[8] == -1);
      auVar23[9] = -(pcVar2[9] == -1);
      auVar23[10] = -(pcVar2[10] == -1);
      auVar23[0xb] = -(pcVar2[0xb] == -1);
      auVar23[0xc] = -(pcVar2[0xc] == -1);
      auVar23[0xd] = -(pcVar2[0xd] == -1);
      auVar23[0xe] = -(pcVar2[0xe] == -1);
      auVar23[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar23 >> 0x3f,0) & 1) != 0) || (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar23 >> 0x4f,0) & 1) != 0) || (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar23 >> 0x5f,0) & 1) != 0) || (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0) ||
          auVar23[0xf] < '\0') break;
      lVar14 = uVar13 + lVar20;
      lVar20 = lVar20 + 0x10;
      uVar13 = lVar14 + 0x10U & uVar12;
    }
  }
  puVar1 = (uint *)(in_R8 + 0x90);
  uVar15 = *(uint *)(in_R8 + 0x90);
  if (uVar15 < 0x3ffffffe) {
    LOCK();
    uVar18 = *puVar1;
    if (uVar15 == uVar18) {
      *puVar1 = uVar15 + 1;
    }
    UNLOCK();
    if (uVar15 == uVar18) goto LAB_141628ae0;
  }
  local_49 = 1;
  std::sys::sync::rwlock::futex::RwLock::read_contended();
LAB_141628ae0:
  if (*(longlong *)(in_R8 + 0xf0) == 0) {
LAB_141629804:
    LOCK();
    uVar15 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (SBORROW4(0,uVar15 - 1 & 0xbfffffff)) {
      local_49 = 1;
      std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
    }
LAB_14162981a:
    local_49 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar21 = *(longlong *)(in_R8 + 0xd8);
  uVar12 = *(ulonglong *)(in_R8 + 0xe0);
  uVar13 = uVar12 & 0x65cfda6a55a841ac;
  lVar20 = 0;
LAB_141628b19:
  pcVar2 = (char *)(lVar21 + uVar13);
  auVar31[0] = -(*pcVar2 == '2');
  auVar31[1] = -(pcVar2[1] == '2');
  auVar31[2] = -(pcVar2[2] == '2');
  auVar31[3] = -(pcVar2[3] == '2');
  auVar31[4] = -(pcVar2[4] == '2');
  auVar31[5] = -(pcVar2[5] == '2');
  auVar31[6] = -(pcVar2[6] == '2');
  auVar31[7] = -(pcVar2[7] == '2');
  auVar31[8] = -(pcVar2[8] == '2');
  auVar31[9] = -(pcVar2[9] == '2');
  auVar31[10] = -(pcVar2[10] == '2');
  auVar31[0xb] = -(pcVar2[0xb] == '2');
  auVar31[0xc] = -(pcVar2[0xc] == '2');
  auVar31[0xd] = -(pcVar2[0xd] == '2');
  auVar31[0xe] = -(pcVar2[0xe] == '2');
  auVar31[0xf] = -(pcVar2[0xf] == '2');
  uVar22 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf;
  uVar15 = (uint)uVar22;
  while( true ) {
    if (uVar22 == 0) goto LAB_141628b30;
    uVar18 = 0;
    for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar18 = uVar18 + 1;
    }
    lVar14 = (uVar18 + uVar13 & uVar12) * -0x88;
    if ((*(longlong *)(lVar21 + -0x88 + lVar14) == -0x7dd9338791986cab) &&
       (*(longlong *)(lVar21 + -0x80 + lVar14) == 0x65cfda6a55a841ac)) break;
    uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
    uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
  }
  uVar12 = *(ulonglong *)(lVar21 + lVar14 + -8);
  LOCK();
  uVar15 = *puVar1;
  *puVar1 = *puVar1 - 1;
  UNLOCK();
  if (SBORROW4(0,uVar15 - 1 & 0xbfffffff)) {
    local_49 = 1;
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
  }
LAB_141628bbb:
  if (((*(ulonglong *)(in_R8 + 0x218) <= uVar12) ||
      (uVar12 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar12 * 8), uVar12 == 0)) ||
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar12 * 0x60) == 0)) goto LAB_14162981a;
  local_49 = 1;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    local_158 = &anon_107183261458dea85466985370debbb6_52_llvm_3641509820080252961;
    local_150 = 0x2c;
    if ((extraout_RAX[3] != 0x2c) ||
       (iVar9 = memcmp(&anon_107183261458dea85466985370debbb6_52_llvm_3641509820080252961,
                       (void *)extraout_RAX[2],0x2c), iVar9 != 0)) {
      local_250 = 0;
      local_49 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_49 = 1;
    puVar10 = (undefined8 *)(**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (puVar10 != (undefined8 *)0x0) {
      local_49 = 1;
      frozen::FrozenMap::
      get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                ();
      if (extraout_RAX_00 != (undefined8 *)0x0) {
        local_158 = &anon_107183261458dea85466985370debbb6_93_llvm_3641509820080252961;
        local_150 = 0x26;
        if ((extraout_RAX_00[3] != 0x26) ||
           (iVar9 = memcmp(&anon_107183261458dea85466985370debbb6_93_llvm_3641509820080252961,
                           (void *)extraout_RAX_00[2],0x26), iVar9 != 0)) {
          local_250 = 0;
          local_49 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        local_49 = 1;
        puVar11 = (undefined8 *)(**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
        if (puVar11 != (undefined8 *)0x0) {
          memcpy(&local_250,local_58,0xf8);
          hashbrown::map::HashMap::
          rustc_entry<ref_<str_>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          uVar12 = local_140;
          puVar7 = local_148;
          lVar21 = local_150;
          puVar8 = local_158;
          if (local_158 == (undefined *)0x0) {
            lVar21 = *(longlong *)(local_168 + 0xb0);
            puVar7 = local_230;
            lVar20 = local_150;
          }
          else {
            pass_resource_access::read_access_buffer<country_core::resources::rt::RtTrianglesSsbo>()
            ;
            pauVar5 = (undefined1 (*) [16])*puVar7;
            uVar13 = puVar7[1];
            uVar19 = uVar13 & uVar12;
            auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar19);
            uVar22 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar30[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar22;
            if (uVar22 == 0) {
              lVar20 = 0x10;
              do {
                uVar19 = uVar19 + lVar20 & uVar13;
                auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar19);
                lVar20 = lVar20 + 0x10;
                uVar15 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                        (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                        (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                        (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                        (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                        (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                        (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                        (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                        (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                        (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                        (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                        (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                        (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                        (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                        (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                       (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
              } while (uVar15 == 0);
            }
            uVar18 = 0;
            for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
              uVar18 = uVar18 + 1;
            }
            uVar19 = uVar18 + uVar19 & uVar13;
            bVar17 = (*pauVar5)[uVar19];
            if (-1 < (char)bVar17) {
              auVar30 = *pauVar5;
              uVar15 = 0;
              for (uVar18 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                           (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                           (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                           (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                           (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                           (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                           (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                           (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                           (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                           (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                           (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                           (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                           (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                           (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                           (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                          (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
                  (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
                uVar15 = uVar15 + 1;
              }
              uVar19 = (ulonglong)uVar15;
              bVar17 = (*pauVar5)[uVar19];
            }
            bVar16 = (byte)(uVar12 >> 0x39);
            (*pauVar5)[uVar19] = bVar16;
            pauVar5[1][uVar19 - 0x10 & uVar13] = bVar16;
            puVar7[2] = puVar7[2] - (ulonglong)(bVar17 & 1);
            lVar20 = (longlong)pauVar5 + uVar19 * -0x18;
            *(undefined **)((longlong)pauVar5 + uVar19 * -0x18 + -0x18) = puVar8;
            *(longlong *)((longlong)pauVar5 + uVar19 * -0x18 + -0x10) = lVar21;
            *(undefined1 *)((longlong)pauVar5 + uVar19 * -0x18 + -8) = extraout_AL;
            puVar7[3] = puVar7[3] + 1;
            lVar21 = *(longlong *)(local_168 + 0xb0);
            puVar7 = local_230;
          }
          local_230 = puVar7;
          if (lVar21 != 0) {
            uVar3 = *(undefined1 *)(lVar20 + -8);
            lVar21 = *(longlong *)(local_168 + 0x98);
            uVar12 = *(ulonglong *)(local_168 + 0xa0);
            uVar13 = uVar12 & 0x168ddb2c3b5f38a9;
            lVar20 = 0;
            while( true ) {
              pcVar2 = (char *)(lVar21 + uVar13);
              auVar32[0] = -(*pcVar2 == '\v');
              auVar32[1] = -(pcVar2[1] == '\v');
              auVar32[2] = -(pcVar2[2] == '\v');
              auVar32[3] = -(pcVar2[3] == '\v');
              auVar32[4] = -(pcVar2[4] == '\v');
              auVar32[5] = -(pcVar2[5] == '\v');
              auVar32[6] = -(pcVar2[6] == '\v');
              auVar32[7] = -(pcVar2[7] == '\v');
              auVar32[8] = -(pcVar2[8] == '\v');
              auVar32[9] = -(pcVar2[9] == '\v');
              auVar32[10] = -(pcVar2[10] == '\v');
              auVar32[0xb] = -(pcVar2[0xb] == '\v');
              auVar32[0xc] = -(pcVar2[0xc] == '\v');
              auVar32[0xd] = -(pcVar2[0xd] == '\v');
              auVar32[0xe] = -(pcVar2[0xe] == '\v');
              auVar32[0xf] = -(pcVar2[0xf] == '\v');
              uVar22 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar32[0xf] >> 7) << 0xf;
              uVar15 = (uint)uVar22;
              while (uVar22 != 0) {
                uVar18 = 0;
                for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar18 = uVar18 + 1;
                }
                uVar19 = uVar18 + uVar13 & uVar12;
                if (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x10) == 0x168ddb2c3b5f38a9) {
                  lVar20 = uVar19 * -0x10;
                  local_50 = *(int *)(lVar21 + -8 + lVar20);
                  uVar4 = *(undefined4 *)(lVar21 + -4 + lVar20);
                  if (local_50 != 0) {
                    local_158 = (undefined *)0x0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (puVar7 == local_240) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_238 + (longlong)puVar7 * 8) = uVar4;
                  *(undefined1 *)(local_238 + 4 + (longlong)puVar7 * 8) = uVar3;
                  local_230 = (undefined8 *)((longlong)puVar7 + 1);
                  goto LAB_141628f39;
                }
                uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
              }
              auVar25[0] = -(*pcVar2 == -1);
              auVar25[1] = -(pcVar2[1] == -1);
              auVar25[2] = -(pcVar2[2] == -1);
              auVar25[3] = -(pcVar2[3] == -1);
              auVar25[4] = -(pcVar2[4] == -1);
              auVar25[5] = -(pcVar2[5] == -1);
              auVar25[6] = -(pcVar2[6] == -1);
              auVar25[7] = -(pcVar2[7] == -1);
              auVar25[8] = -(pcVar2[8] == -1);
              auVar25[9] = -(pcVar2[9] == -1);
              auVar25[10] = -(pcVar2[10] == -1);
              auVar25[0xb] = -(pcVar2[0xb] == -1);
              auVar25[0xc] = -(pcVar2[0xc] == -1);
              auVar25[0xd] = -(pcVar2[0xd] == -1);
              auVar25[0xe] = -(pcVar2[0xe] == -1);
              auVar25[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar25 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar25 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar25 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') break;
              lVar14 = uVar13 + lVar20;
              lVar20 = lVar20 + 0x10;
              uVar13 = lVar14 + 0x10U & uVar12;
            }
          }
LAB_141628f39:
          memcpy(&local_158,&local_250,0xf8);
          hashbrown::map::HashMap::
          rustc_entry<ref_<str_>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          uVar12 = local_238;
          puVar7 = local_240;
          if (local_250 == 0) {
            lVar20 = *(longlong *)(local_70 + 0xb0);
            lVar21 = local_248;
            puVar7 = local_138;
          }
          else {
            pass_resource_access::
            read_access_buffer<country_core::resources::rt::RtTriangleMetaSsbo>();
            pauVar5 = (undefined1 (*) [16])*puVar7;
            uVar13 = puVar7[1];
            uVar19 = uVar13 & uVar12;
            auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar19);
            uVar22 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar30[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar22;
            if (uVar22 == 0) {
              lVar21 = 0x10;
              do {
                uVar19 = uVar19 + lVar21 & uVar13;
                auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar19);
                lVar21 = lVar21 + 0x10;
                uVar15 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                        (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                        (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                        (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                        (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                        (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                        (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                        (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                        (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                        (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                        (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                        (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                        (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                        (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                        (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                       (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
              } while (uVar15 == 0);
            }
            uVar18 = 0;
            for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
              uVar18 = uVar18 + 1;
            }
            uVar19 = uVar18 + uVar19 & uVar13;
            bVar17 = (*pauVar5)[uVar19];
            if (-1 < (char)bVar17) {
              auVar30 = *pauVar5;
              uVar15 = 0;
              for (uVar18 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                           (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                           (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                           (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                           (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                           (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                           (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                           (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                           (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                           (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                           (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                           (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                           (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                           (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                           (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                          (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
                  (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
                uVar15 = uVar15 + 1;
              }
              uVar19 = (ulonglong)uVar15;
              bVar17 = (*pauVar5)[uVar19];
            }
            bVar16 = (byte)(uVar12 >> 0x39);
            (*pauVar5)[uVar19] = bVar16;
            pauVar5[1][uVar19 - 0x10 & uVar13] = bVar16;
            puVar7[2] = puVar7[2] - (ulonglong)(bVar17 & 1);
            lVar21 = (longlong)pauVar5 + uVar19 * -0x18;
            *(longlong *)((longlong)pauVar5 + uVar19 * -0x18 + -0x18) = local_250;
            *(longlong *)((longlong)pauVar5 + uVar19 * -0x18 + -0x10) = local_248;
            *(undefined1 *)((longlong)pauVar5 + uVar19 * -0x18 + -8) = extraout_AL_00;
            puVar7[3] = puVar7[3] + 1;
            lVar20 = *(longlong *)(local_70 + 0xb0);
            puVar7 = local_138;
          }
          local_138 = puVar7;
          if (lVar20 != 0) {
            uVar3 = *(undefined1 *)(lVar21 + -8);
            lVar21 = *(longlong *)(local_70 + 0x98);
            uVar12 = *(ulonglong *)(local_70 + 0xa0);
            uVar13 = uVar12 & 0x47ddebfb264383d0;
            lVar20 = 0;
            while( true ) {
              pcVar2 = (char *)(lVar21 + uVar13);
              auVar33[0] = -(*pcVar2 == '#');
              auVar33[1] = -(pcVar2[1] == '#');
              auVar33[2] = -(pcVar2[2] == '#');
              auVar33[3] = -(pcVar2[3] == '#');
              auVar33[4] = -(pcVar2[4] == '#');
              auVar33[5] = -(pcVar2[5] == '#');
              auVar33[6] = -(pcVar2[6] == '#');
              auVar33[7] = -(pcVar2[7] == '#');
              auVar33[8] = -(pcVar2[8] == '#');
              auVar33[9] = -(pcVar2[9] == '#');
              auVar33[10] = -(pcVar2[10] == '#');
              auVar33[0xb] = -(pcVar2[0xb] == '#');
              auVar33[0xc] = -(pcVar2[0xc] == '#');
              auVar33[0xd] = -(pcVar2[0xd] == '#');
              auVar33[0xe] = -(pcVar2[0xe] == '#');
              auVar33[0xf] = -(pcVar2[0xf] == '#');
              uVar22 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar33[0xf] >> 7) << 0xf;
              uVar15 = (uint)uVar22;
              while (uVar22 != 0) {
                uVar18 = 0;
                for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar18 = uVar18 + 1;
                }
                uVar19 = uVar18 + uVar13 & uVar12;
                if (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x10) == 0x47ddebfb264383d0) {
                  lVar20 = uVar19 * -0x10;
                  local_50 = *(int *)(lVar21 + -8 + lVar20);
                  uVar4 = *(undefined4 *)(lVar21 + -4 + lVar20);
                  if (local_50 != 0) {
                    local_250 = 0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (puVar7 == local_148) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_140 + (longlong)puVar7 * 8) = uVar4;
                  *(undefined1 *)(local_140 + 4 + (longlong)puVar7 * 8) = uVar3;
                  local_138 = (undefined8 *)((longlong)puVar7 + 1);
                  goto LAB_141629158;
                }
                uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
              }
              auVar26[0] = -(*pcVar2 == -1);
              auVar26[1] = -(pcVar2[1] == -1);
              auVar26[2] = -(pcVar2[2] == -1);
              auVar26[3] = -(pcVar2[3] == -1);
              auVar26[4] = -(pcVar2[4] == -1);
              auVar26[5] = -(pcVar2[5] == -1);
              auVar26[6] = -(pcVar2[6] == -1);
              auVar26[7] = -(pcVar2[7] == -1);
              auVar26[8] = -(pcVar2[8] == -1);
              auVar26[9] = -(pcVar2[9] == -1);
              auVar26[10] = -(pcVar2[10] == -1);
              auVar26[0xb] = -(pcVar2[0xb] == -1);
              auVar26[0xc] = -(pcVar2[0xc] == -1);
              auVar26[0xd] = -(pcVar2[0xd] == -1);
              auVar26[0xe] = -(pcVar2[0xe] == -1);
              auVar26[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar26 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar26 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') break;
              lVar14 = uVar13 + lVar20;
              lVar20 = lVar20 + 0x10;
              uVar13 = lVar14 + 0x10U & uVar12;
            }
          }
LAB_141629158:
          memcpy(&local_250,&local_158,0xf8);
          hashbrown::map::HashMap::
          rustc_entry<ref_<str_>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          lVar21 = local_150;
          puVar8 = local_158;
          if (local_158 == (undefined *)0x0) {
            lVar21 = *(longlong *)(local_168 + 0xb0);
            puVar7 = local_230;
            lVar20 = local_150;
          }
          else {
            pass_resource_access::read_access_buffer<country_core::resources::rt::RtBvhSsbo>();
            pauVar5 = (undefined1 (*) [16])*local_148;
            uVar12 = local_148[1];
            uVar13 = uVar12 & local_140;
            auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar13);
            uVar22 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar30[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar22;
            if (uVar22 == 0) {
              lVar20 = 0x10;
              do {
                uVar13 = uVar13 + lVar20 & uVar12;
                auVar30 = *(undefined1 (*) [16])(*pauVar5 + uVar13);
                lVar20 = lVar20 + 0x10;
                uVar15 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                        (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                        (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                        (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                        (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                        (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                        (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                        (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                        (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                        (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                        (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                        (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                        (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                        (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                        (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                       (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
              } while (uVar15 == 0);
            }
            uVar18 = 0;
            for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
              uVar18 = uVar18 + 1;
            }
            uVar13 = uVar18 + uVar13 & uVar12;
            bVar17 = (*pauVar5)[uVar13];
            if (-1 < (char)bVar17) {
              auVar30 = *pauVar5;
              uVar15 = 0;
              for (uVar18 = (uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                                           (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                                           (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                                           (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                                           (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                                           (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                                           (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                                           (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                                           (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                                           (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                                           (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                                           (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                                           (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                                           (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                                           (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                                          (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
                  (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
                uVar15 = uVar15 + 1;
              }
              uVar13 = (ulonglong)uVar15;
              bVar17 = (*pauVar5)[uVar13];
            }
            bVar16 = (byte)(local_140 >> 0x39);
            (*pauVar5)[uVar13] = bVar16;
            pauVar5[1][uVar13 - 0x10 & uVar12] = bVar16;
            local_148[2] = local_148[2] - (ulonglong)(bVar17 & 1);
            lVar20 = (longlong)pauVar5 + uVar13 * -0x18;
            *(undefined **)((longlong)pauVar5 + uVar13 * -0x18 + -0x18) = puVar8;
            *(longlong *)((longlong)pauVar5 + uVar13 * -0x18 + -0x10) = lVar21;
            *(undefined1 *)((longlong)pauVar5 + uVar13 * -0x18 + -8) = extraout_AL_01;
            local_148[3] = local_148[3] + 1;
            lVar21 = *(longlong *)(local_168 + 0xb0);
            puVar7 = local_230;
          }
          local_230 = puVar7;
          if (lVar21 != 0) {
            uVar3 = *(undefined1 *)(lVar20 + -8);
            lVar21 = *(longlong *)(local_168 + 0x98);
            uVar12 = *(ulonglong *)(local_168 + 0xa0);
            uVar13 = uVar12 & 0x339e76846042a271;
            lVar20 = 0;
            while( true ) {
              pcVar2 = (char *)(lVar21 + uVar13);
              auVar34[0] = -(*pcVar2 == '\x19');
              auVar34[1] = -(pcVar2[1] == '\x19');
              auVar34[2] = -(pcVar2[2] == '\x19');
              auVar34[3] = -(pcVar2[3] == '\x19');
              auVar34[4] = -(pcVar2[4] == '\x19');
              auVar34[5] = -(pcVar2[5] == '\x19');
              auVar34[6] = -(pcVar2[6] == '\x19');
              auVar34[7] = -(pcVar2[7] == '\x19');
              auVar34[8] = -(pcVar2[8] == '\x19');
              auVar34[9] = -(pcVar2[9] == '\x19');
              auVar34[10] = -(pcVar2[10] == '\x19');
              auVar34[0xb] = -(pcVar2[0xb] == '\x19');
              auVar34[0xc] = -(pcVar2[0xc] == '\x19');
              auVar34[0xd] = -(pcVar2[0xd] == '\x19');
              auVar34[0xe] = -(pcVar2[0xe] == '\x19');
              auVar34[0xf] = -(pcVar2[0xf] == '\x19');
              uVar22 = (ushort)(SUB161(auVar34 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar34[0xf] >> 7) << 0xf;
              uVar15 = (uint)uVar22;
              while (uVar22 != 0) {
                uVar18 = 0;
                for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar18 = uVar18 + 1;
                }
                uVar19 = uVar18 + uVar13 & uVar12;
                if (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x10) == 0x339e76846042a271) {
                  lVar20 = uVar19 * -0x10;
                  local_50 = *(int *)(lVar21 + -8 + lVar20);
                  uVar4 = *(undefined4 *)(lVar21 + -4 + lVar20);
                  if (local_50 != 0) {
                    local_158 = (undefined *)0x0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (puVar7 == local_240) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_238 + (longlong)puVar7 * 8) = uVar4;
                  *(undefined1 *)(local_238 + 4 + (longlong)puVar7 * 8) = uVar3;
                  local_230 = (undefined8 *)((longlong)puVar7 + 1);
                  goto LAB_141629379;
                }
                uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
              }
              auVar27[0] = -(*pcVar2 == -1);
              auVar27[1] = -(pcVar2[1] == -1);
              auVar27[2] = -(pcVar2[2] == -1);
              auVar27[3] = -(pcVar2[3] == -1);
              auVar27[4] = -(pcVar2[4] == -1);
              auVar27[5] = -(pcVar2[5] == -1);
              auVar27[6] = -(pcVar2[6] == -1);
              auVar27[7] = -(pcVar2[7] == -1);
              auVar27[8] = -(pcVar2[8] == -1);
              auVar27[9] = -(pcVar2[9] == -1);
              auVar27[10] = -(pcVar2[10] == -1);
              auVar27[0xb] = -(pcVar2[0xb] == -1);
              auVar27[0xc] = -(pcVar2[0xc] == -1);
              auVar27[0xd] = -(pcVar2[0xd] == -1);
              auVar27[0xe] = -(pcVar2[0xe] == -1);
              auVar27[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar27 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar27 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar27 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar27 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar27 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar27 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar27 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar27 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar27 >> 0x4f,0) & 1) != 0) || (SUB161(auVar27 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar27 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar27 >> 0x67,0) & 1) != 0) || (SUB161(auVar27 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar27 >> 0x77,0) & 1) != 0) || auVar27[0xf] < '\0') break;
              lVar14 = uVar13 + lVar20;
              lVar20 = lVar20 + 0x10;
              uVar13 = lVar14 + 0x10U & uVar12;
            }
          }
LAB_141629379:
          memcpy(&local_158,&local_250,0xf8);
          local_49 = 0;
          draw_builder::impl$2::
          read_storage_buffer<country_core::resources::color_id::WallColorIdSsbo>();
          if (((local_172 != '\b') && (*(longlong *)(local_168 + 0xe8) != 0)) &&
             (*(longlong *)(local_168 + 0x130) != 0)) {
            lVar21 = *(longlong *)(local_168 + 0x118);
            uVar12 = *(ulonglong *)(local_168 + 0x120);
            uVar13 = uVar12 & 0x1c9f2eef0ac739ac;
            lVar20 = 0;
            while( true ) {
              pcVar2 = (char *)(lVar21 + uVar13);
              auVar35[0] = -(*pcVar2 == '\x0e');
              auVar35[1] = -(pcVar2[1] == '\x0e');
              auVar35[2] = -(pcVar2[2] == '\x0e');
              auVar35[3] = -(pcVar2[3] == '\x0e');
              auVar35[4] = -(pcVar2[4] == '\x0e');
              auVar35[5] = -(pcVar2[5] == '\x0e');
              auVar35[6] = -(pcVar2[6] == '\x0e');
              auVar35[7] = -(pcVar2[7] == '\x0e');
              auVar35[8] = -(pcVar2[8] == '\x0e');
              auVar35[9] = -(pcVar2[9] == '\x0e');
              auVar35[10] = -(pcVar2[10] == '\x0e');
              auVar35[0xb] = -(pcVar2[0xb] == '\x0e');
              auVar35[0xc] = -(pcVar2[0xc] == '\x0e');
              auVar35[0xd] = -(pcVar2[0xd] == '\x0e');
              auVar35[0xe] = -(pcVar2[0xe] == '\x0e');
              auVar35[0xf] = -(pcVar2[0xf] == '\x0e');
              uVar22 = (ushort)(SUB161(auVar35 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar35[0xf] >> 7) << 0xf;
              uVar15 = (uint)uVar22;
              while (uVar22 != 0) {
                uVar18 = 0;
                for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar18 = uVar18 + 1;
                }
                uVar19 = uVar18 + uVar13 & uVar12;
                if (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x10) == 0x1c9f2eef0ac739ac) {
                  uVar12 = (ulonglong)*(uint *)(lVar21 + -8 + uVar19 * -0x10);
                  if (local_190 < uVar12) {
                    // WARNING: Subroutine does not return
                    core::slice::index::slice_start_index_len_fail();
                  }
                  *(int *)(local_198 + uVar12) = (int)*puVar10;
                  goto LAB_141629494;
                }
                uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
              }
              auVar28[0] = -(*pcVar2 == -1);
              auVar28[1] = -(pcVar2[1] == -1);
              auVar28[2] = -(pcVar2[2] == -1);
              auVar28[3] = -(pcVar2[3] == -1);
              auVar28[4] = -(pcVar2[4] == -1);
              auVar28[5] = -(pcVar2[5] == -1);
              auVar28[6] = -(pcVar2[6] == -1);
              auVar28[7] = -(pcVar2[7] == -1);
              auVar28[8] = -(pcVar2[8] == -1);
              auVar28[9] = -(pcVar2[9] == -1);
              auVar28[10] = -(pcVar2[10] == -1);
              auVar28[0xb] = -(pcVar2[0xb] == -1);
              auVar28[0xc] = -(pcVar2[0xc] == -1);
              auVar28[0xd] = -(pcVar2[0xd] == -1);
              auVar28[0xe] = -(pcVar2[0xe] == -1);
              auVar28[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar28 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar28 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar28 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar28 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar28 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar28 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar28 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar28 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar28 >> 0x4f,0) & 1) != 0) || (SUB161(auVar28 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar28 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar28 >> 0x67,0) & 1) != 0) || (SUB161(auVar28 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar28 >> 0x77,0) & 1) != 0) || auVar28[0xf] < '\0') break;
              lVar14 = uVar13 + lVar20;
              lVar20 = lVar20 + 0x10;
              uVar13 = lVar14 + 0x10U & uVar12;
            }
          }
LAB_141629494:
          memcpy(&local_158,&local_250,0xf8);
          if (((local_7a != '\b') && (*(longlong *)(local_70 + 0xe8) != 0)) &&
             (*(longlong *)(local_70 + 0x130) != 0)) {
            lVar21 = *(longlong *)(local_70 + 0x118);
            uVar12 = *(ulonglong *)(local_70 + 0x120);
            uVar13 = uVar12 & 0xc2b7f2fdf5e21521;
            lVar20 = 0;
            while( true ) {
              pcVar2 = (char *)(lVar21 + uVar13);
              auVar36[0] = -(*pcVar2 == 'a');
              auVar36[1] = -(pcVar2[1] == 'a');
              auVar36[2] = -(pcVar2[2] == 'a');
              auVar36[3] = -(pcVar2[3] == 'a');
              auVar36[4] = -(pcVar2[4] == 'a');
              auVar36[5] = -(pcVar2[5] == 'a');
              auVar36[6] = -(pcVar2[6] == 'a');
              auVar36[7] = -(pcVar2[7] == 'a');
              auVar36[8] = -(pcVar2[8] == 'a');
              auVar36[9] = -(pcVar2[9] == 'a');
              auVar36[10] = -(pcVar2[10] == 'a');
              auVar36[0xb] = -(pcVar2[0xb] == 'a');
              auVar36[0xc] = -(pcVar2[0xc] == 'a');
              auVar36[0xd] = -(pcVar2[0xd] == 'a');
              auVar36[0xe] = -(pcVar2[0xe] == 'a');
              auVar36[0xf] = -(pcVar2[0xf] == 'a');
              uVar22 = (ushort)(SUB161(auVar36 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar36[0xf] >> 7) << 0xf;
              uVar15 = (uint)uVar22;
              while (uVar22 != 0) {
                uVar18 = 0;
                for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                  uVar18 = uVar18 + 1;
                }
                uVar19 = uVar18 + uVar13 & uVar12;
                if (*(longlong *)(lVar21 + -0x10 + uVar19 * -0x10) == -0x3d480d020a1deadf) {
                  uVar12 = (ulonglong)*(uint *)(lVar21 + -8 + uVar19 * -0x10);
                  if (local_98 < uVar12) {
                    // WARNING: Subroutine does not return
                    core::slice::index::slice_start_index_len_fail();
                  }
                  *(int *)(local_a0 + uVar12) = (int)*puVar11;
                  goto LAB_141629591;
                }
                uVar22 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar22);
              }
              auVar29[0] = -(*pcVar2 == -1);
              auVar29[1] = -(pcVar2[1] == -1);
              auVar29[2] = -(pcVar2[2] == -1);
              auVar29[3] = -(pcVar2[3] == -1);
              auVar29[4] = -(pcVar2[4] == -1);
              auVar29[5] = -(pcVar2[5] == -1);
              auVar29[6] = -(pcVar2[6] == -1);
              auVar29[7] = -(pcVar2[7] == -1);
              auVar29[8] = -(pcVar2[8] == -1);
              auVar29[9] = -(pcVar2[9] == -1);
              auVar29[10] = -(pcVar2[10] == -1);
              auVar29[0xb] = -(pcVar2[0xb] == -1);
              auVar29[0xc] = -(pcVar2[0xc] == -1);
              auVar29[0xd] = -(pcVar2[0xd] == -1);
              auVar29[0xe] = -(pcVar2[0xe] == -1);
              auVar29[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar29 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar29 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar29 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x4f,0) & 1) != 0) || (SUB161(auVar29 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
              lVar14 = uVar13 + lVar20;
              lVar20 = lVar20 + 0x10;
              uVar13 = lVar14 + 0x10U & uVar12;
            }
          }
LAB_141629591:
          memcpy(local_60,&local_158,0xf8);
          return;
        }
      }
      local_49 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_141628b30:
  auVar24[0] = -(*pcVar2 == -1);
  auVar24[1] = -(pcVar2[1] == -1);
  auVar24[2] = -(pcVar2[2] == -1);
  auVar24[3] = -(pcVar2[3] == -1);
  auVar24[4] = -(pcVar2[4] == -1);
  auVar24[5] = -(pcVar2[5] == -1);
  auVar24[6] = -(pcVar2[6] == -1);
  auVar24[7] = -(pcVar2[7] == -1);
  auVar24[8] = -(pcVar2[8] == -1);
  auVar24[9] = -(pcVar2[9] == -1);
  auVar24[10] = -(pcVar2[10] == -1);
  auVar24[0xb] = -(pcVar2[0xb] == -1);
  auVar24[0xc] = -(pcVar2[0xc] == -1);
  auVar24[0xd] = -(pcVar2[0xd] == -1);
  auVar24[0xe] = -(pcVar2[0xe] == -1);
  auVar24[0xf] = -(pcVar2[0xf] == -1);
  if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') goto LAB_141629804;
  lVar14 = uVar13 + lVar20;
  lVar20 = lVar20 + 0x10;
  uVar13 = lVar14 + 0x10U & uVar12;
  goto LAB_141628b19;
}

