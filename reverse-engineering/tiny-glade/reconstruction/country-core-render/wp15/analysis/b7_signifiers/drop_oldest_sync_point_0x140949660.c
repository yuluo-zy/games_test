// addr=0x140949660 name=drop_oldest_sync_point
// signature=void __cdecl drop_oldest_sync_point(void)


void __cdecl country_core::input::input_stream::InputStream::drop_oldest_sync_point(void)

{
  bool bVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong *in_RCX;
  ulonglong *puVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong local_120;
  longlong local_110 [3];
  longlong local_f8;
  
  bVar1 = true;
  if (in_RCX[3] != 0) {
    uVar3 = in_RCX[2];
    uVar10 = *in_RCX;
    uVar5 = in_RCX[1];
    uVar12 = 0;
    if (uVar10 <= uVar3 + 1) {
      uVar12 = uVar10;
    }
    uVar12 = (uVar3 - uVar12) + 1;
    in_RCX[2] = uVar12;
    local_120 = in_RCX[3] - 1;
    in_RCX[3] = local_120;
    lVar2 = uVar3 * 0x60;
    lVar9 = *(longlong *)(uVar5 + lVar2);
    if (!SBORROW8(0,lVar9)) {
      lVar11 = *(longlong *)(uVar5 + 0x10 + lVar2);
      if (lVar11 != 0) {
        lVar2 = *(longlong *)(uVar5 + 8 + lVar2) + 8;
        do {
          if (*(longlong *)(lVar2 + -8) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          lVar2 = lVar2 + 0x20;
          lVar11 = lVar11 + -1;
        } while (lVar11 != 0);
      }
      if (lVar9 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    if (local_120 != 0) {
      bVar1 = false;
      uVar3 = 0;
      if (uVar10 <= uVar12) {
        uVar3 = uVar10;
      }
      lVar9 = uVar5 + (uVar12 - uVar3) * 0x60;
      goto LAB_140949775;
    }
  }
  local_120 = 0;
  lVar9 = 0;
LAB_140949775:
  puVar6 = (ulonglong *)(lVar9 + 0x58);
  if (lVar9 == 0) {
    puVar6 = in_RCX + 7;
  }
  uVar3 = *puVar6;
  if (uVar3 != 0) {
    uVar10 = in_RCX[7];
    uVar5 = in_RCX[4];
    uVar12 = in_RCX[5];
    uVar4 = in_RCX[6];
    uVar8 = uVar3;
    do {
      uVar10 = uVar10 - 1;
      if (uVar10 == 0xffffffffffffffff) {
        local_110[0] = -0x8000000000000000;
LAB_140949940:
        bevy_ecs::component::ComponentDescriptor::
        drop_ptr<country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier>
                  ();
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      uVar7 = uVar5;
      if (uVar4 + 1 < uVar5) {
        uVar7 = 0;
      }
      uVar7 = (uVar4 - uVar7) + 1;
      in_RCX[6] = uVar7;
      in_RCX[7] = uVar10;
      memcpy(local_110,(void *)(uVar4 * 0xd0 + uVar12),0xd0);
      if (SBORROW8(0,local_110[0])) goto LAB_140949940;
      if (local_110[0] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_f8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      uVar8 = uVar8 - 1;
      uVar4 = uVar7;
    } while (uVar8 != 0);
  }
  if (bVar1) {
    lVar9 = 0;
    uVar10 = 0;
    lVar2 = 0;
  }
  else {
    uVar10 = *in_RCX;
    uVar5 = 0;
    if (uVar10 <= in_RCX[2]) {
      uVar5 = uVar10;
    }
    lVar9 = in_RCX[2] - uVar5;
    lVar2 = local_120 - (uVar10 - lVar9);
    if (local_120 < uVar10 - lVar9 || lVar2 == 0) {
      uVar10 = local_120 + lVar9;
      lVar2 = 0;
    }
  }
  uVar5 = in_RCX[1];
  uVar7 = lVar9 * 0x60 + uVar5;
  uVar8 = uVar10 * 0x60 + uVar5;
  uVar12 = lVar2 * 0x60 + uVar5;
  uVar10 = uVar5;
  uVar4 = uVar12;
  if (uVar7 == uVar8) goto LAB_1409498e0;
  do {
    do {
      if (*(ulonglong *)(uVar7 + 0x58) < uVar3) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      *(ulonglong *)(uVar7 + 0x58) = *(ulonglong *)(uVar7 + 0x58) - uVar3;
      uVar7 = uVar7 + 0x60;
      uVar10 = uVar5;
      uVar4 = uVar12;
    } while (uVar7 != uVar8);
LAB_1409498e0:
    uVar5 = uVar7;
  } while ((uVar10 != uVar4) && (uVar12 = uVar5, uVar8 = uVar4, uVar7 = uVar10, uVar10 != 0));
  return;
}


