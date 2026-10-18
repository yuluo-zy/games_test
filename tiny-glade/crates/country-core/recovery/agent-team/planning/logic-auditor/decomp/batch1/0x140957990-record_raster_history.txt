
void __cdecl country_core::systems::world_raster::record_raster_history(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined1 uVar3;
  longlong lVar4;
  longlong lVar5;
  void *_Src;
  longlong lVar6;
  size_t _Size;
  longlong lVar7;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined4 *extraout_RAX_01;
  longlong *in_RCX;
  byte in_DL;
  size_t sVar8;
  code *in_R8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined4 local_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 *local_80;
  undefined4 local_74;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  lVar7 = (ulonglong)in_DL * 0x470;
  lVar4 = *in_RCX;
  local_74 = *(undefined4 *)(lVar4 + 0x68 + lVar7);
  uVar1 = *(undefined4 *)(lVar4 + 0x6c + lVar7);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  lVar5 = *(longlong *)(lVar4 + 0x48 + lVar7);
  _Src = *(void **)(lVar4 + 0x50 + lVar7);
  lVar6 = *(longlong *)(lVar4 + 0x58 + lVar7);
  uVar2 = *(undefined4 *)(lVar4 + 0x60 + lVar7);
  uVar3 = *(undefined1 *)(lVar4 + 100 + lVar7);
  *(undefined8 *)(lVar4 + 0x48 + lVar7) = 0;
  *(undefined8 *)(lVar4 + 0x50 + lVar7) = 4;
  *(undefined8 *)(lVar4 + 0x58 + lVar7) = 0;
  *(undefined4 *)(lVar4 + 0x60 + lVar7) = 0;
  *(undefined1 *)(lVar4 + 100 + lVar7) = 0;
  *(undefined4 *)in_RCX[6] = *(undefined4 *)((longlong)in_RCX + 0x3c);
  _Size = lVar6 * 0xc;
  sVar8 = _Size;
  alloc::sync::arcinner_layout_for_value_layout();
  local_80 = extraout_RAX;
  if (sVar8 != 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_80 = extraout_RAX_00;
  }
  if (local_80 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *local_80 = 1;
  local_80[1] = 1;
  memcpy(local_80 + 2,_Src,_Size);
  if (lVar5 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uStack_98 = local_80;
  uStack_84 = CONCAT31(uStack_84._1_3_,uVar3);
  local_b0 = 1;
  uStack_a8 = 1;
  local_a0 = local_74;
  uStack_9c = uVar1;
  local_90 = lVar6;
  uStack_88 = uVar2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 != (undefined4 *)0x0) {
    extraout_RAX_01[8] = (undefined4)local_90;
    extraout_RAX_01[9] = local_90._4_4_;
    extraout_RAX_01[10] = uStack_88;
    extraout_RAX_01[0xb] = uStack_84;
    extraout_RAX_01[4] = local_a0;
    extraout_RAX_01[5] = uStack_9c;
    extraout_RAX_01[6] = (undefined4)uStack_98;
    extraout_RAX_01[7] = uStack_98._4_4_;
    *extraout_RAX_01 = (undefined4)local_b0;
    extraout_RAX_01[1] = local_b0._4_4_;
    extraout_RAX_01[2] = (undefined4)uStack_a8;
    extraout_RAX_01[3] = uStack_a8._4_4_;
    (*in_R8)(&local_b0,extraout_RAX_01);
    resources::history::History::add();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

