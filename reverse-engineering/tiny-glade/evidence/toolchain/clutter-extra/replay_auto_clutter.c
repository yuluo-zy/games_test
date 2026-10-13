
/* WARNING: Removing unreachable block (ram,0x000140aa4f9f) */

void __cdecl country_core::systems::history::replay_auto_clutter::replay_auto_clutter(void)

{
  char *pcVar1;
  longlong lVar2;
  float fVar3;
  float fVar4;
  undefined1 auVar5 [16];
  char cVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  char *pcVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined8 uVar12;
  int iVar13;
  uint uVar14;
  float fVar15;
  longlong *plVar16;
  longlong *plVar17;
  float fVar18;
  longlong *in_RCX;
  longlong lVar19;
  longlong *in_RDX;
  ulonglong uVar20;
  uint uVar21;
  undefined8 *in_R8;
  undefined1 (*pauVar22) [16];
  undefined8 *puVar23;
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  char *pcVar27;
  longlong lVar28;
  undefined1 (*pauVar29) [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  ulonglong local_90;
  ulonglong *local_88;
  
  puVar8 = (ulonglong *)*in_RCX;
  fVar3 = *(float *)(puVar8 + 5);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  fVar18 = __real_3d4ccccd;
  if (puVar8[3] == 0) {
    return;
  }
  uVar26 = puVar8[1];
  uVar20 = *puVar8;
  if (puVar8[2] < *puVar8) {
    uVar20 = 0;
  }
  lVar19 = (puVar8[2] - uVar20) * 0x40;
  lVar2 = uVar26 + lVar19;
  local_90 = 0;
  pcVar9 = *(char **)(uVar26 + 8 + lVar19);
  lVar19 = *(longlong *)(uVar26 + 0x10 + lVar19);
  uVar7 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar23 = (undefined8 *)*in_R8;
  puVar10 = (undefined4 *)in_R8[2];
  lVar24 = *in_RDX;
  uVar26 = 0;
  pcVar1 = pcVar9;
  while (pcVar27 = pcVar1, uVar20 = uVar26, pcVar27 != pcVar9 + lVar19 * 0x30) {
    pcVar1 = pcVar27 + 0x30;
    uVar26 = uVar20 + 1;
    if (*pcVar27 == '\t') {
      if ((*(float *)(pcVar27 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar27 + 0x18)))) {
        lVar11 = *(longlong *)(pcVar27 + 0x10);
        cVar6 = pcVar27[0x1c];
        *puVar10 = uVar7;
        if (cVar6 == '\0') {
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        }
        else {
          lVar28 = puVar23[3];
          if (lVar28 != 0) {
            pauVar29 = (undefined1 (*) [16])*puVar23;
            pauVar22 = pauVar29 + 1;
            auVar31 = *pauVar29;
            uVar21 = ~(uint)(ushort)((ushort)(SUB161(auVar31 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
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
                                     (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar31[0xf] >> 7) << 0xf);
            do {
              if ((short)uVar21 == 0) {
                do {
                  auVar31 = *pauVar22;
                  pauVar29 = pauVar29 + -8;
                  pauVar22 = pauVar22 + 1;
                  uVar21 = (uint)(ushort)((ushort)(SUB161(auVar31 >> 7,0) & 1) |
                                          (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
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
                                          (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                                         (ushort)(byte)(auVar31[0xf] >> 7) << 0xf);
                } while (uVar21 == 0xffff);
                uVar21 = ~uVar21;
              }
              iVar13 = 0;
              for (uVar14 = uVar21; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
                iVar13 = iVar13 + 1;
              }
              if (*(longlong *)(lVar11 + 0x10) ==
                  *(longlong *)((longlong)pauVar29 + (-8 - (ulonglong)(uint)(iVar13 << 3)))) {
                hashbrown::raw::RawTable::
                erase<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
                          ();
              }
              uVar21 = uVar21 - 1 & uVar21;
              lVar28 = lVar28 + -1;
            } while (lVar28 != 0);
          }
        }
      }
      if ((pcVar27[8] & 1U) == 0) {
        if (local_90 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(ulonglong *)(local_90 * 8 + 8) = uVar20;
        local_90 = local_90 + 1;
      }
      else {
        fVar4 = *(float *)(pcVar27 + 0x18);
        fVar15 = *(float *)(pcVar27 + 0xc) / fVar3;
        if (*(float *)(pcVar27 + 0xc) / fVar3 <= fVar18) {
          fVar15 = fVar18;
        }
        if (fVar15 < fVar4) {
          if (local_90 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(ulonglong *)(local_90 * 8 + 8) = uVar20;
          local_90 = local_90 + 1;
          fVar4 = *(float *)(pcVar27 + 0x18);
        }
        *(float *)(pcVar27 + 0x18) = fVar4 + *(float *)(lVar24 + 0x28);
      }
    }
  }
  if (1 < local_90) {
    uVar26 = local_90 >> 1;
    if (local_90 < 8) {
      uVar20 = 0;
    }
    else {
      uVar20 = uVar26 & 0xfffffffffffffffc;
      pauVar22 = (undefined1 (*) [16])(local_90 * 8 + -8);
      uVar25 = 0;
      do {
        auVar31 = *(undefined1 (*) [16])(uVar25 * 8 + 8);
        auVar5 = *(undefined1 (*) [16])(uVar25 * 8 + 0x18);
        auVar30 = pauVar22[-1];
        auVar32 = *pauVar22;
        auVar33._0_8_ = auVar32._8_8_;
        auVar33._8_4_ = auVar32._0_4_;
        auVar33._12_4_ = auVar32._4_4_;
        auVar32._0_8_ = auVar30._8_8_;
        auVar32._8_4_ = auVar30._0_4_;
        auVar32._12_4_ = auVar30._4_4_;
        *(undefined1 (*) [16])(uVar25 * 8 + 8) = auVar33;
        *(undefined1 (*) [16])(uVar25 * 8 + 0x18) = auVar32;
        auVar30._0_8_ = auVar31._8_8_;
        auVar30._8_4_ = auVar31._0_4_;
        auVar30._12_4_ = auVar31._4_4_;
        *pauVar22 = auVar30;
        auVar31._0_8_ = auVar5._8_8_;
        auVar31._8_4_ = auVar5._0_4_;
        auVar31._12_4_ = auVar5._4_4_;
        pauVar22[-1] = auVar31;
        uVar25 = uVar25 + 4;
        pauVar22 = pauVar22 + -2;
      } while (uVar20 != uVar25);
      if (uVar26 == uVar20) goto joined_r0x000140aa4dc1;
    }
    puVar23 = (undefined8 *)(uVar20 * 8);
    lVar19 = -uVar20;
    do {
      puVar23 = puVar23 + 1;
      uVar12 = *puVar23;
      *puVar23 = *(undefined8 *)(local_90 * 8 + lVar19 * 8);
      *(undefined8 *)(local_90 * 8 + lVar19 * 8) = uVar12;
      lVar19 = lVar19 + -1;
    } while (-lVar19 != uVar26);
  }
joined_r0x000140aa4dc1:
  if (local_90 != 0) {
    local_88 = (ulonglong *)&DAT_00000008;
    do {
      uVar26 = *local_88;
      uVar20 = *(ulonglong *)(lVar2 + 0x10);
      if (uVar20 <= uVar26) {
                    /* WARNING: Subroutine does not return */
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar19 = *(longlong *)(lVar2 + 8);
      lVar24 = uVar26 * 0x30;
      uVar25 = *(ulonglong *)(lVar19 + lVar24);
      plVar16 = (longlong *)((ulonglong *)(lVar19 + lVar24))[1];
      plVar17 = *(longlong **)(lVar19 + 0x10 + lVar24);
      memmove((void *)(lVar19 + lVar24),(void *)(lVar19 + lVar24 + 0x30),(~uVar26 + uVar20) * 0x30);
      *(ulonglong *)(lVar2 + 0x10) = uVar20 - 1;
      switch(uVar25 & 0xff) {
      case 0:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 4:
        core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>();
        break;
      case 5:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if (*plVar17 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_88 = local_88 + 1;
    } while (local_88 != (ulonglong *)(local_90 * 8 + 8));
  }
  return;
}

