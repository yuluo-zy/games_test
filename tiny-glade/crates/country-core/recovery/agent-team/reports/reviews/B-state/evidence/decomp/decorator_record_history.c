
void __cdecl system_decorator::deferred_decorator_history::decorator_record_history(void)

{
  longlong *plVar1;
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  undefined4 *puVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong *plVar10;
  undefined8 *extraout_RAX;
  ulonglong uVar11;
  longlong in_RCX;
  longlong *in_RDX;
  longlong *plVar12;
  longlong local_b8 [3];
  longlong *local_a0;
  longlong *local_98;
  longlong local_90;
  longlong local_88;
  undefined8 local_80;
  longlong local_78;
  longlong lStack_70;
  longlong local_68;
  undefined4 *local_58;
  undefined4 local_4c;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar5 = (undefined4 *)in_RDX[2];
  *puVar5 = uVar3;
  lVar6 = *in_RDX;
  cVar2 = *(char *)(lVar6 + 0x48);
  *(undefined1 *)(lVar6 + 0x48) = 0;
  *puVar5 = uVar3;
  if (cVar2 != '\0') {
    local_88 = *(longlong *)(lVar6 + 0x10);
    *(undefined8 *)(lVar6 + 0x10) = 0;
    plVar12 = *(longlong **)(lVar6 + 8) + local_88 * 4;
    local_80 = 0;
    uVar4 = *(undefined4 *)(in_RCX + 0x1c);
    puVar7 = *(undefined4 **)(in_RCX + 0x10);
    local_4c = uVar3;
    local_58 = puVar5;
    local_90 = lVar6;
    plVar10 = *(longlong **)(lVar6 + 8);
    local_98 = plVar12;
    while (plVar10 != plVar12) {
      plVar1 = plVar10 + 4;
      lVar8 = *plVar10;
      local_a0 = plVar1;
      if (lVar8 == 6) break;
      local_68 = plVar10[3];
      local_78 = plVar10[1];
      lStack_70 = plVar10[2];
      *puVar7 = uVar4;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX = 1;
      extraout_RAX[1] = 1;
      extraout_RAX[2] = lVar8;
      extraout_RAX[3] = local_78;
      extraout_RAX[4] = lStack_70;
      extraout_RAX[5] = local_68;
      local_b8[0] = 10;
      in_RDX = local_b8;
      country_core::resources::history::History::add();
      plVar10 = plVar1;
    }
    *local_58 = local_4c;
  }
  *(undefined8 *)(lVar6 + 0x10) = 0;
  if (*(longlong *)(lVar6 + 0x30) != 0) {
    uVar9 = *(ulonglong *)(lVar6 + 0x20);
    if (uVar9 != 0) {
      memset(*(void **)(lVar6 + 0x18),(int)CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff),
             uVar9 + 0x11);
    }
    *(undefined8 *)(lVar6 + 0x30) = 0;
    uVar11 = (uVar9 + 1 & 0xfffffffffffffff8) - (uVar9 + 1 >> 3);
    if (uVar9 < 8) {
      uVar11 = uVar9;
    }
    *(ulonglong *)(lVar6 + 0x28) = uVar11;
  }
  return;
}

