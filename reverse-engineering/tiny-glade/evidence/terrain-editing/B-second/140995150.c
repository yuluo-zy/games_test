
/* WARNING: Removing unreachable block (ram,0x00014099580f) */

void __cdecl country_core::systems::history::replay_terrain::replay_terrain(void)

{
  int *piVar1;
  longlong lVar2;
  float fVar3;
  undefined1 auVar4 [16];
  char cVar5;
  undefined4 uVar6;
  uint uVar7;
  ulonglong *puVar8;
  char *pcVar9;
  undefined4 *puVar10;
  longlong lVar11;
  longlong lVar12;
  void *_Src;
  undefined8 uVar13;
  undefined1 auVar14 [16];
  longlong *plVar15;
  longlong *plVar16;
  float fVar17;
  float fVar18;
  char *pcVar19;
  uint *puVar20;
  ulonglong uVar21;
  undefined1 *extraout_RAX;
  ulonglong uVar22;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong *in_R8;
  longlong lVar25;
  undefined1 (*pauVar26) [16];
  undefined8 *puVar27;
  longlong lVar28;
  char *pcVar29;
  undefined1 *_Dst;
  float fVar30;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  float fVar35;
  longlong local_170;
  ulonglong local_c0;
  ulonglong *local_a8;
  
  puVar8 = (ulonglong *)*in_R8;
  fVar3 = *(float *)(puVar8 + 5);
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  fVar18 = __real_3a83126f;
  fVar17 = __real_3f800000;
  if (puVar8[3] == 0) {
    return;
  }
  uVar24 = puVar8[1];
  uVar23 = *puVar8;
  if (puVar8[2] < *puVar8) {
    uVar23 = 0;
  }
  lVar25 = (puVar8[2] - uVar23) * 0x40;
  lVar2 = uVar24 + lVar25;
  local_c0 = 0;
  pcVar9 = *(char **)(uVar24 + 8 + lVar25);
  lVar25 = *(longlong *)(uVar24 + 0x10 + lVar25);
  uVar6 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar28 = *in_RDX;
  puVar10 = (undefined4 *)in_RDX[2];
  lVar11 = *in_RCX;
  uVar24 = 0;
  pcVar19 = pcVar9;
  while (pcVar29 = pcVar19, uVar23 = uVar24, pcVar29 != pcVar9 + lVar25 * 0x30) {
    uVar24 = uVar23 + 1;
    pcVar19 = pcVar29 + 0x30;
    if (*pcVar29 == '\x03') {
      lVar12 = *(longlong *)(pcVar29 + 8);
      puVar20 = (uint *)(lVar12 + 0x30);
      if (*(longlong *)(lVar12 + 0x10) == -0x7fffffffffffffff) {
        puVar20 = (uint *)(lVar12 + 0x18);
      }
      if ((((!SBORROW8(0,*(longlong *)(lVar12 + 0x10))) || (*(float *)(pcVar29 + 0x10) != 0.0)) ||
          (NAN(*(float *)(pcVar29 + 0x10)))) || (pcVar29[0x1c] != '\0')) {
        fVar30 = *(float *)(pcVar29 + 0x14);
      }
      else {
        uVar7 = *puVar20;
        *puVar10 = uVar6;
        auVar14._8_8_ = 0;
        auVar14._0_8_ = *(ulonglong *)(lVar12 + 0x28);
        uVar21 = SUB168(auVar14 * ZEXT816(0xc),0);
        if ((SUB168(auVar14 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar21)) {
LAB_14099588d:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        _Src = *(void **)(lVar12 + 0x20);
        if (uVar21 == 0) {
          _Dst = &DAT_00000004;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          _Dst = extraout_RAX;
          if (extraout_RAX == (undefined1 *)0x0) goto LAB_14099588d;
        }
        memcpy(_Dst,_Src,uVar21);
        if (0x3ffffe < uVar7) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        uVar22 = *(ulonglong *)(lVar28 + 0x38);
        uVar21 = (ulonglong)uVar7 + 1;
        if (uVar22 < uVar21) {
          uVar22 = uVar21;
        }
        *(ulonglong *)(lVar28 + 0x38) = uVar22;
        hashbrown::map::HashMap::
        insert<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        if ((!SBORROW8(0,local_170)) && (local_170 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        fVar30 = *(float *)(pcVar29 + 0x14);
      }
      fVar35 = fVar17;
      if (fVar18 <= fVar30) {
        fVar35 = 0.0;
        if (0.0 <= *(float *)(pcVar29 + 0x10) / fVar30) {
          fVar35 = *(float *)(pcVar29 + 0x10) / fVar30;
        }
        fVar30 = fVar17;
        if (fVar35 <= fVar17) {
          fVar30 = fVar35;
        }
        fVar30 = fVar17 - fVar30;
        fVar35 = fVar17 - fVar30 * fVar30 * fVar30;
      }
      cVar5 = pcVar29[0x1c];
      *puVar10 = uVar6;
      if (cVar5 == '\0') {
        resources::history::terrain_edit::TerrainEdit::reapply();
      }
      else {
        resources::history::terrain_edit::TerrainEdit::revert();
      }
      if (fVar17 <= fVar35) {
        piVar1 = (int *)(pcVar29 + 0x18);
        *piVar1 = *piVar1 + -1;
        if (*piVar1 < 0) {
          if (local_c0 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(ulonglong *)(local_c0 * 8 + 8) = uVar23;
          local_c0 = local_c0 + 1;
        }
      }
      *(float *)(pcVar29 + 0x10) = *(float *)(lVar11 + 0x28) * fVar3 + *(float *)(pcVar29 + 0x10);
    }
  }
  if (1 < local_c0) {
    uVar24 = local_c0 >> 1;
    if (local_c0 < 8) {
      uVar23 = 0;
    }
    else {
      uVar23 = uVar24 & 0xfffffffffffffffc;
      pauVar26 = (undefined1 (*) [16])(local_c0 * 8 + -8);
      uVar21 = 0;
      do {
        auVar14 = *(undefined1 (*) [16])(uVar21 * 8 + 8);
        auVar4 = *(undefined1 (*) [16])(uVar21 * 8 + 0x18);
        auVar31 = pauVar26[-1];
        auVar32 = *pauVar26;
        auVar34._0_8_ = auVar32._8_8_;
        auVar34._8_4_ = auVar32._0_4_;
        auVar34._12_4_ = auVar32._4_4_;
        auVar33._0_8_ = auVar31._8_8_;
        auVar33._8_4_ = auVar31._0_4_;
        auVar33._12_4_ = auVar31._4_4_;
        *(undefined1 (*) [16])(uVar21 * 8 + 8) = auVar34;
        *(undefined1 (*) [16])(uVar21 * 8 + 0x18) = auVar33;
        auVar31._0_8_ = auVar14._8_8_;
        auVar31._8_4_ = auVar14._0_4_;
        auVar31._12_4_ = auVar14._4_4_;
        *pauVar26 = auVar31;
        auVar32._0_8_ = auVar4._8_8_;
        auVar32._8_4_ = auVar4._0_4_;
        auVar32._12_4_ = auVar4._4_4_;
        pauVar26[-1] = auVar32;
        uVar21 = uVar21 + 4;
        pauVar26 = pauVar26 + -2;
      } while (uVar23 != uVar21);
      if (uVar24 == uVar23) goto joined_r0x000140995634;
    }
    puVar27 = (undefined8 *)(uVar23 * 8);
    lVar25 = -uVar23;
    do {
      puVar27 = puVar27 + 1;
      uVar13 = *puVar27;
      *puVar27 = *(undefined8 *)(local_c0 * 8 + lVar25 * 8);
      *(undefined8 *)(local_c0 * 8 + lVar25 * 8) = uVar13;
      lVar25 = lVar25 + -1;
    } while (-lVar25 != uVar24);
  }
joined_r0x000140995634:
  if (local_c0 != 0) {
    local_a8 = (ulonglong *)&DAT_00000008;
    do {
      uVar24 = *local_a8;
      uVar23 = *(ulonglong *)(lVar2 + 0x10);
      if (uVar23 <= uVar24) {
                    /* WARNING: Subroutine does not return */
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar25 = *(longlong *)(lVar2 + 8);
      lVar28 = uVar24 * 0x30;
      uVar21 = *(ulonglong *)(lVar25 + lVar28);
      plVar15 = (longlong *)((ulonglong *)(lVar25 + lVar28))[1];
      plVar16 = *(longlong **)(lVar25 + 0x10 + lVar28);
      memmove((void *)(lVar25 + lVar28),(void *)(lVar25 + lVar28 + 0x30),(~uVar24 + uVar23) * 0x30);
      *(ulonglong *)(lVar2 + 0x10) = uVar23 - 1;
      switch(uVar21 & 0xff) {
      case 0:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar15 = *plVar15 + -1;
        UNLOCK();
        if (*plVar15 == 0) {
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
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (*plVar16 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_a8 = local_a8 + 1;
    } while (local_a8 != (ulonglong *)(local_c0 * 8 + 8));
  }
  return;
}

