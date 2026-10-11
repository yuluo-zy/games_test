
void __cdecl parking_lot_core::parking_lot::deadlock_impl::release_resource(void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  longlong lVar5;
  undefined8 *puVar6;
  longlong unaff_GS_OFFSET;
  undefined8 local_108;
  undefined8 local_98 [15];
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_98[0] = 0;
  lVar3 = *(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)_tls_index * 8);
  lVar4 = *(longlong *)(lVar3 + 0x2a0);
  if (lVar4 == 0) {
    std::sys::thread_local::native::lazy::Storage::
    initialize<parking_lot_core::parking_lot::ThreadData,tuple_<>,parking_lot_core::parking_lot::ThreadData_(*)()>
              ();
    puVar6 = extraout_RAX;
  }
  else if ((int)lVar4 == 1) {
    puVar6 = (undefined8 *)(lVar3 + 0x2a8);
  }
  else {
    ThreadData::new();
    puVar6 = local_98;
    core::ptr::
    drop_in_place<enum2_<core::option::Option<parking_lot_core::parking_lot::ThreadData>_>_>();
    local_98[0] = local_108;
  }
  lVar3 = puVar6[9];
  lVar4 = puVar6[10];
  lVar5 = lVar4 * 8;
  do {
    if (lVar5 == 0) goto LAB_140c81f13;
    lVar1 = lVar5 + -8;
    plVar2 = (longlong *)(lVar3 + -8 + lVar5);
    lVar5 = lVar1;
  } while (*plVar2 != in_RCX);
  *(undefined8 *)(lVar3 + lVar1) = *(undefined8 *)(lVar3 + -8 + lVar4 * 8);
  puVar6[10] = lVar4 + -1;
LAB_140c81f13:
  core::ptr::
  drop_in_place<enum2_<core::option::Option<parking_lot_core::parking_lot::ThreadData>_>_>();
  return;
}

