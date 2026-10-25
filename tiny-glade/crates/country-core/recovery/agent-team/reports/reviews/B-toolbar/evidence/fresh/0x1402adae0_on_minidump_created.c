
void __cdecl tiny_glade::crash_pickup::server::impl$1::on_minidump_created(void)

{
  longlong lVar1;
  char *pcVar2;
  HANDLE extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong in_RCX;
  longlong *plVar3;
  longlong *in_RDX;
  HANDLE pvVar4;
  longlong lVar5;
  bool bVar6;
  longlong lStack_4f0;
  longlong *local_4e8;
  longlong lStack_4e0;
  longlong *local_308;
  longlong lStack_300;
  longlong lStack_2f8;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined4 uStack_2e8;
  undefined2 local_2e4;
  undefined2 uStack_2e2;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined4 local_2d8;
  undefined4 uStack_2d4;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  HANDLE local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  HANDLE local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong *local_98;
  longlong lStack_90;
  longlong local_88;
  undefined8 uStack_80;
  longlong local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  longlong local_50;
  char *local_48;
  longlong *local_40;
  undefined1 local_35;
  undefined1 local_33;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  if (SBORROW8(0,*in_RDX)) {
    core::ptr::
    drop_in_place<enum2_<core::result::Result<minidumper::MinidumpBinary,enum2_<minidumper::errors::Error>_>_>_>
              ();
    return;
  }
  local_98 = (longlong *)*in_RDX;
  lStack_90 = in_RDX[1];
  local_88 = in_RDX[2];
  uStack_80 = in_RDX[3];
  lVar5 = in_RDX[4];
  uStack_70 = (longlong *)in_RDX[5];
  local_68 = in_RDX[6];
  uStack_60 = in_RDX[7];
  if (!SBORROW8(0,lVar5)) {
    local_308 = (longlong *)in_RDX[5];
    lStack_300 = in_RDX[6];
    goto LAB_1402adcda;
  }
  lStack_4f0 = 0;
  local_4e8 = (longlong *)0x1;
  lStack_4e0 = 0;
  local_308 = (longlong *)((ulonglong)local_308 & 0xffffffff00000000);
  lStack_300 = 0;
  lStack_2f8 = 0;
  uStack_2f0 = 7;
  uStack_2ec = 0;
  local_2e4 = 0;
  uStack_2e8 = 1;
  local_33 = 1;
  local_78 = lVar5;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_33 = 1;
  pvVar4 = extraout_RAX;
  std::fs::OpenOptions::_open();
  if ((extraout_RAX_00 & 1) != 0) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_35 = 1;
  plVar3 = &lStack_4f0;
  local_d8 = pvVar4;
  std::fs::impl$8::read_to_end();
  if ((extraout_RAX_01 & 1) == 0) {
    plVar3 = local_4e8;
    if (SBORROW8(0,lStack_4f0)) goto LAB_1402adc4d;
    local_308 = local_4e8;
    lStack_300 = lStack_4e0;
    lVar5 = lStack_4f0;
  }
  else {
    if (lStack_4f0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
LAB_1402adc4d:
    lVar5 = -0x8000000000000000;
    if (((uint)plVar3 & 3) == 1) {
      local_50 = (longlong)plVar3 + -1;
      local_48 = (char *)*(undefined8 *)((longlong)plVar3 + -1);
      local_40 = *(longlong **)((longlong)plVar3 + 7);
      if ((code *)*local_40 != (code *)0x0) {
        (*(code *)*local_40)(*(undefined8 *)((longlong)plVar3 + -1));
      }
      if (local_40[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  CloseHandle(local_d8);
LAB_1402adcda:
  uStack_70 = local_308;
  local_68 = lStack_300;
  lVar1 = *(longlong *)(in_RCX + 0x20);
  LOCK();
  bVar6 = *(char *)(lVar1 + 0x10) == '\0';
  if (bVar6) {
    *(char *)(lVar1 + 0x10) = '\x01';
  }
  UNLOCK();
  local_78 = lVar5;
  if (!bVar6) {
    local_31 = 1;
    local_32 = 1;
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  local_31 = 1;
  local_32 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_48 = (char *)(lVar1 + 0x10);
  if (SBORROW8(0,*(longlong *)(lVar1 + 0x18))) {
    local_d8 = (HANDLE)0x8000000000000001;
    crash_payload::impl$0::default();
    local_f0 = local_a8;
    uStack_e8 = uStack_a0;
    local_100 = local_b8;
    uStack_f8 = uStack_b0;
    local_110 = local_c8;
    uStack_108 = uStack_c0;
    local_120 = local_d8;
    uStack_118 = uStack_d0;
    memcpy(&local_308,&lStack_4f0,0x1e8);
    memcpy((void *)(lVar1 + 0x18),&local_308,0x228);
  }
  local_2d8 = (undefined4)local_68;
  uStack_2d4 = local_68._4_4_;
  uStack_2d0 = (undefined4)uStack_60;
  uStack_2cc = uStack_60._4_4_;
  uStack_2e8 = (undefined4)local_78;
  local_2e4 = (undefined2)((ulonglong)local_78 >> 0x20);
  uStack_2e2 = (undefined2)((ulonglong)local_78 >> 0x30);
  uStack_2e0 = (undefined4)uStack_70;
  uStack_2dc = uStack_70._4_4_;
  lStack_2f8 = local_88;
  uStack_2f0 = (undefined4)uStack_80;
  uStack_2ec = uStack_80._4_4_;
  local_308 = local_98;
  lStack_300 = lStack_90;
  local_40 = (longlong *)(lVar1 + 0x200);
  lVar5 = *(longlong *)(lVar1 + 0x200);
  if (SBORROW8(0,lVar5)) {
    core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::BacktracePayload>();
  }
  else if (lVar5 != -0x7fffffffffffffff) {
    CloseHandle(*(HANDLE *)(lVar1 + 0x238));
    if (lVar5 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((!SBORROW8(0,*(longlong *)(lVar1 + 0x220))) && (*(longlong *)(lVar1 + 0x220) != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  pcVar2 = local_48;
  local_40[6] = CONCAT44(uStack_2d4,local_2d8);
  local_40[7] = CONCAT44(uStack_2cc,uStack_2d0);
  local_40[4] = CONCAT26(uStack_2e2,CONCAT24(local_2e4,uStack_2e8));
  local_40[5] = CONCAT44(uStack_2dc,uStack_2e0);
  local_40[2] = lStack_2f8;
  local_40[3] = CONCAT44(uStack_2ec,uStack_2f0);
  *local_40 = (longlong)local_308;
  local_40[1] = lStack_300;
  local_31 = 0;
  local_32 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar6 = *pcVar2 == '\x01';
  if (bVar6) {
    *pcVar2 = '\0';
  }
  UNLOCK();
  if (!bVar6) {
    local_31 = 0;
    local_32 = 0;
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  return;
}

