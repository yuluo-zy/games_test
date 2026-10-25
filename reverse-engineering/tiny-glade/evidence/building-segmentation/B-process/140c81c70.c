
void __cdecl parking_lot_core::parking_lot::deadlock_impl::acquire_resource(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *extraout_RAX;
  undefined8 in_RCX;
  undefined8 *puVar3;
  longlong unaff_GS_OFFSET;
  undefined8 local_108;
  undefined8 local_98 [14];
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  local_98[0] = 0;
  lVar1 = *(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)_tls_index * 8);
  lVar2 = *(longlong *)(lVar1 + 0x2a0);
  if (lVar2 == 0) {
    std::sys::thread_local::native::lazy::Storage::
    initialize<parking_lot_core::parking_lot::ThreadData,tuple_<>,parking_lot_core::parking_lot::ThreadData_(*)()>
              ();
    puVar3 = extraout_RAX;
  }
  else if ((int)lVar2 == 1) {
    puVar3 = (undefined8 *)(lVar1 + 0x2a8);
  }
  else {
    ThreadData::new();
    puVar3 = local_98;
    core::ptr::
    drop_in_place<enum2_<core::option::Option<parking_lot_core::parking_lot::ThreadData>_>_>();
    local_98[0] = local_108;
  }
  lVar1 = puVar3[10];
  if (lVar1 == puVar3[8]) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
  *(undefined8 *)(puVar3[9] + lVar1 * 8) = in_RCX;
  puVar3[10] = lVar1 + 1;
  core::ptr::
  drop_in_place<enum2_<core::option::Option<parking_lot_core::parking_lot::ThreadData>_>_>();
  return;
}

