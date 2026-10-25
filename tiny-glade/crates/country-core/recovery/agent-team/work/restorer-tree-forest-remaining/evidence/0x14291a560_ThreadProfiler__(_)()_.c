// initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()> 0x14291a560 size=287
// signature: void __cdecl initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>(void)


void __cdecl
std::sys::thread_local::native::lazy::Storage::
initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  longlong *in_RCX;
  byte *in_RDX;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  code *pcVar6;
  code *pcVar7;
  longlong lVar8;
  longlong lVar9;
  longlong local_48;
  longlong lStack_40;
  longlong local_38;
  longlong local_28;
  longlong lStack_20;
  longlong local_18;
  
  if (in_RDX != (byte *)0x0) {
    lVar5 = *(longlong *)(in_RDX + 0x20);
    lVar4 = *(longlong *)(in_RDX + 0x28);
    lVar8 = *(longlong *)(in_RDX + 0x48);
    lVar9 = *(longlong *)(in_RDX + 0x50);
    pcVar6 = *(code **)(in_RDX + 0x58);
    pcVar7 = *(code **)(in_RDX + 0x60);
    lVar3 = *(longlong *)(in_RDX + 0x68);
    in_RDX[0] = 0;
    in_RDX[1] = 0;
    in_RDX[2] = 0;
    in_RDX[3] = 0;
    in_RDX[4] = 0;
    in_RDX[5] = 0;
    in_RDX[6] = 0;
    in_RDX[7] = 0;
    if ((*in_RDX & 1) != 0) {
      local_18 = *(longlong *)(in_RDX + 0x18);
      local_28 = *(longlong *)(in_RDX + 8);
      lStack_20 = *(longlong *)(in_RDX + 0x10);
      local_48 = *(longlong *)(in_RDX + 0x30);
      lStack_40 = *(longlong *)(in_RDX + 0x38);
      local_38 = *(longlong *)(in_RDX + 0x40);
      goto LAB_14291a5fa;
    }
  }
  local_28 = 0;
  lStack_20 = 0;
  local_48 = 0;
  lStack_40 = 0;
  pcVar7 = puffin::global_reporter;
  pcVar6 = puffin::now_ns;
  local_38 = 0;
  lVar8 = 0x7fffffffffffffff;
  lVar9 = -0x8000000000000000;
  lVar4 = 1;
  lVar3 = 0;
  lVar5 = 0;
LAB_14291a5fa:
  lVar1 = *in_RCX;
  lVar2 = in_RCX[4];
  *in_RCX = 1;
  in_RCX[1] = local_28;
  in_RCX[2] = lStack_20;
  in_RCX[3] = local_18;
  in_RCX[4] = lVar5;
  in_RCX[5] = lVar4;
  in_RCX[6] = local_48;
  in_RCX[7] = lStack_40;
  in_RCX[8] = local_38;
  in_RCX[9] = lVar8;
  in_RCX[10] = lVar9;
  in_RCX[0xb] = (longlong)pcVar6;
  in_RCX[0xc] = (longlong)pcVar7;
  in_RCX[0xd] = lVar3;
  if (lVar1 == 1) {
    if (lVar2 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else if (lVar1 == 0) {
    destructors::list::register();
  }
  return;
}

