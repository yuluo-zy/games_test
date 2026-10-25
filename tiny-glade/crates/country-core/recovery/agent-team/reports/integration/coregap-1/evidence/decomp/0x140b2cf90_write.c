// 0x140b2cf90 write :: void __cdecl write(void)

void __cdecl country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write(void)

{
  longlong lVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  uint extraout_EAX;
  longlong lVar10;
  longlong extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  undefined8 *puVar11;
  bool bVar12;
  
  LOCK();
  plVar2 = (longlong *)(in_RCX + 0x168);
  lVar4 = *plVar2;
  *plVar2 = *plVar2 + 1;
  UNLOCK();
  puVar11 = in_RDX;
  append_only_vec::indices();
  lVar1 = lVar4 + 1;
  if (*(ulonglong *)(in_RCX + 0x160) < (ulonglong)(lVar1 - (longlong)puVar11)) {
    if (puVar11 == (undefined8 *)0x0) {
      if (0x333333333333333 < (ulonglong)(8L << ((byte)extraout_EAX & 0x3f))) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (0x2b < extraout_EAX) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      *(longlong *)(in_RCX + (ulonglong)extraout_EAX * 8) = extraout_RAX;
      lVar10 = extraout_RAX;
      goto LAB_14146c926;
    }
    uVar5 = *(ulonglong *)(in_RCX + 0x160);
    while (uVar5 < (ulonglong)(lVar1 - (longlong)puVar11)) {
      uVar5 = *(ulonglong *)(in_RCX + 0x160);
    }
    if (0x2b < extraout_EAX) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
  }
  else if (0x2b < extraout_EAX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar10 = *(longlong *)(in_RCX + (ulonglong)extraout_EAX * 8);
LAB_14146c926:
  uVar6 = *in_RDX;
  uVar7 = in_RDX[1];
  uVar8 = in_RDX[2];
  uVar9 = in_RDX[3];
  *(undefined8 *)(lVar10 + 0x20 + (longlong)puVar11 * 0x28) = in_RDX[4];
  puVar3 = (undefined8 *)(lVar10 + 0x10 + (longlong)puVar11 * 0x28);
  *puVar3 = uVar8;
  puVar3[1] = uVar9;
  puVar11 = (undefined8 *)(lVar10 + (longlong)puVar11 * 0x28);
  *puVar11 = uVar6;
  puVar11[1] = uVar7;
  LOCK();
  bVar12 = lVar4 == *(longlong *)(in_RCX + 0x160);
  if (bVar12) {
    *(longlong *)(in_RCX + 0x160) = lVar1;
  }
  UNLOCK();
  while (!bVar12) {
    LOCK();
    bVar12 = lVar4 == *(longlong *)(in_RCX + 0x160);
    if (bVar12) {
      *(longlong *)(in_RCX + 0x160) = lVar1;
    }
    UNLOCK();
  }
  return;
}


