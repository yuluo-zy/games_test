// 0x141a37ad0 void __cdecl replay_roof(void)

// WARNING: Removing unreachable block (ram,0x000141a37ed1)

void __cdecl system_roof::replay_roof::replay_roof(void)

{
  char *pcVar1;
  undefined1 auVar2 [16];
  char *pcVar3;
  undefined8 uVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong extraout_RAX;
  longlong in_RCX;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined1 (*pauVar10) [16];
  undefined8 *puVar11;
  ulonglong uVar12;
  char *pcVar13;
  longlong lVar14;
  longlong lVar15;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  ulonglong local_70;
  ulonglong *local_68;
  
  country_core::input::input_stream::InputStream::sync_point_count();
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket();
  if (extraout_RAX == 0) {
    return;
  }
  local_70 = 0;
  pcVar3 = *(char **)(extraout_RAX + 8);
  lVar9 = *(longlong *)(extraout_RAX + 0x10);
  pcVar1 = pcVar3;
  lVar15 = 0;
  while (lVar14 = lVar15, pcVar13 = pcVar1, pcVar13 != pcVar3 + lVar9 * 0x30) {
    pcVar1 = pcVar13 + 0x30;
    lVar15 = lVar14 + 1;
    if ((*pcVar13 == '\x05') && (impl$0::process(), pcVar13[0x1d] == '\x01')) {
      if (local_70 == 0) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      *(longlong *)(local_70 * 8 + 8) = lVar14;
      local_70 = local_70 + 1;
    }
  }
  if (1 < local_70) {
    uVar7 = local_70 >> 1;
    if (local_70 < 8) {
      uVar8 = 0;
    }
    else {
      uVar8 = uVar7 & 0xfffffffffffffffc;
      pauVar10 = (undefined1 (*) [16])(local_70 * 8 + -8);
      uVar12 = 0;
      do {
        auVar17 = *(undefined1 (*) [16])(uVar12 * 8 + 8);
        auVar2 = *(undefined1 (*) [16])(uVar12 * 8 + 0x18);
        auVar16 = pauVar10[-1];
        auVar18 = *pauVar10;
        auVar19._0_8_ = auVar18._8_8_;
        auVar19._8_4_ = auVar18._0_4_;
        auVar19._12_4_ = auVar18._4_4_;
        auVar18._0_8_ = auVar16._8_8_;
        auVar18._8_4_ = auVar16._0_4_;
        auVar18._12_4_ = auVar16._4_4_;
        *(undefined1 (*) [16])(uVar12 * 8 + 8) = auVar19;
        *(undefined1 (*) [16])(uVar12 * 8 + 0x18) = auVar18;
        auVar16._0_8_ = auVar17._8_8_;
        auVar16._8_4_ = auVar17._0_4_;
        auVar16._12_4_ = auVar17._4_4_;
        *pauVar10 = auVar16;
        auVar17._0_8_ = auVar2._8_8_;
        auVar17._8_4_ = auVar2._0_4_;
        auVar17._12_4_ = auVar2._4_4_;
        pauVar10[-1] = auVar17;
        uVar12 = uVar12 + 4;
        pauVar10 = pauVar10 + -2;
      } while (uVar8 != uVar12);
      if (uVar7 == uVar8) goto joined_r0x000141a37d01;
    }
    puVar11 = (undefined8 *)(uVar8 * 8);
    lVar9 = -uVar8;
    do {
      puVar11 = puVar11 + 1;
      uVar4 = *puVar11;
      *puVar11 = *(undefined8 *)(local_70 * 8 + lVar9 * 8);
      *(undefined8 *)(local_70 * 8 + lVar9 * 8) = uVar4;
      lVar9 = lVar9 + -1;
    } while (-lVar9 != uVar7);
  }
joined_r0x000141a37d01:
  if (local_70 != 0) {
    local_68 = (ulonglong *)&DAT_00000008;
    do {
      uVar7 = *local_68;
      uVar8 = *(ulonglong *)(extraout_RAX + 0x10);
      if (uVar8 <= uVar7) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar9 = *(longlong *)(extraout_RAX + 8);
      lVar15 = uVar7 * 0x30;
      uVar12 = *(ulonglong *)(lVar9 + lVar15);
      plVar5 = (longlong *)((ulonglong *)(lVar9 + lVar15))[1];
      plVar6 = *(longlong **)(lVar9 + 0x10 + lVar15);
      memmove((void *)(lVar9 + lVar15),(void *)(lVar9 + lVar15 + 0x30),(~uVar7 + uVar8) * 0x30);
      *(ulonglong *)(extraout_RAX + 0x10) = uVar8 - 1;
      switch(uVar12 & 0xff) {
      case 0:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar5 = *plVar5 + -1;
        UNLOCK();
        if (*plVar5 == 0) {
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
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_68 = local_68 + 1;
    } while (local_68 != (ulonglong *)(local_70 * 8 + 8));
  }
  return;
}


