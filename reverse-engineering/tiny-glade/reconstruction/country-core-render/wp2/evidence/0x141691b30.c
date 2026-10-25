// address 0x141691b30  name gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
// signature: void __cdecl gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>(void)

void __cdecl
country_core::render::shadowed_buffer::ShadowedBuffer::
gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong uVar5;
  char extraout_AL;
  uint uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  ulonglong *in_RCX;
  longlong *plVar7;
  ulonglong in_RDX;
  undefined *puVar8;
  ulonglong uVar9;
  longlong local_118;
  undefined *local_b8;
  
  if (*in_RCX <= in_RCX[1]) {
    uVar5 = *in_RCX;
    uVar9 = in_RCX[1];
    puffin::are_scopes_on();
    if (extraout_AL != '\0') {
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX == 1) {
        if (in_RDX != 0) {
          if (in_RDX < 0x43) {
            if ((char)(&DAT_142c86673)[in_RDX] < -0x40) {
LAB_141691bbf:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (in_RDX != 0x43) goto LAB_141691bbf;
        }
        core::str::impl$0::rfind<ref_<str_>_>();
        if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
          if (in_RDX + 2 < 0x43) {
            if ("untry_core::render::shadowed_buffer::ShadowedBuffer<_>::gpu_flush::f"[in_RDX] <
                -0x40) {
LAB_141691c13:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (in_RDX + 2 != 0x43) goto LAB_141691c13;
        }
      }
      puVar8 = &DAT_142c867e9;
      do {
        bVar2 = puVar8[-1];
        uVar6 = (uint)(char)bVar2;
        if ((char)bVar2 < '\0') {
          bVar3 = puVar8[-2];
          if ((char)bVar3 < -0x40) {
            bVar4 = puVar8[-3];
            if ((char)bVar4 < -0x40) {
              pbVar1 = puVar8 + -4;
              puVar8 = puVar8 + -4;
              uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar8 = puVar8 + -3;
              uVar6 = bVar4 & 0xf;
            }
            uVar6 = bVar3 & 0x3f | uVar6 << 6;
          }
          else {
            puVar8 = puVar8 + -2;
            uVar6 = bVar3 & 0x1f;
          }
          uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
          if (uVar6 != 0x5c) goto LAB_141691cd4;
LAB_141691ce6:
          if (puVar8 == &UNK_142c867b7) break;
          if (puVar8 + -0x142c867b7 < (undefined *)0x31) {
            if (-0x41 < (char)puVar8[1]) break;
          }
          else if (puVar8 + -0x142c867b7 == (undefined *)0x31) break;
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
        puVar8 = puVar8 + -1;
        if (uVar6 == 0x5c) goto LAB_141691ce6;
LAB_141691cd4:
        if (uVar6 == 0x2f) goto LAB_141691ce6;
      } while (puVar8 != &anon_e33fdb04fc97586641c5ddb6e9131f54_22_llvm_13407665901565942354);
      local_b8 = &DAT_142c86868;
      alloc::fmt::format::format_inner();
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_01 == 0) {
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        plVar7 = extraout_RAX_02;
        if (extraout_RAX_02 == (longlong *)0x0) goto LAB_141691f95;
      }
      else {
        if ((int)*extraout_RAX_01 != 1) {
LAB_141691f95:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        plVar7 = extraout_RAX_01 + 1;
      }
      if (*plVar7 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar7 = -1;
      puffin::ThreadProfiler::begin_scope();
      *plVar7 = *plVar7 + 1;
      if (local_118 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    uVar9 = uVar9 + 1;
    if (uVar9 < uVar5) {
                    // WARNING: Subroutine does not return
      core::slice::index::slice_index_order_fail();
    }
    if (in_RCX[3] < uVar9) {
                    // WARNING: Subroutine does not return
      core::slice::index::slice_end_index_len_fail();
    }
    if ((uVar9 != uVar5) &&
       ((in_RCX[7] < uVar9 * 0x30 ||
        (rhapsody::raw::device::GraphicsDevice::update_buffer_bytes(), local_b8 != (undefined *)0xb)
        ))) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    *in_RCX = 0xffffffffffffffff;
    in_RCX[1] = 0;
    if (extraout_AL != '\0') {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_03 == 1) {
        plVar7 = extraout_RAX_03 + 1;
      }
      else if ((*extraout_RAX_03 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar7 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      if (*plVar7 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar7 = -1;
      puffin::ThreadProfiler::end_scope();
      *plVar7 = *plVar7 + 1;
    }
  }
  return;
}


