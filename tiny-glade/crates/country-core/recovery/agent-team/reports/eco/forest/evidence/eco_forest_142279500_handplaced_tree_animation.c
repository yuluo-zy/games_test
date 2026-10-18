// signature: void __cdecl handplaced_tree_animation(void)
// size: 821
// locals: [{"name": "pbVar1", "type": "byte *", "is_phantom": false, "in_decompiled_code": true}, {"name": "uVar7", "type": "ulonglong", "storage": "RDX:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "in_R8", "type": "undefined8 *", "storage": "R8:8", "is_phantom": true, "in_decompiled_code": true}, {"name": "extraout_RAX_02", "type": "longlong *", "storage": "RAX:8", "is_phantom": true, "in_decompiled_code": true}, {"name": "plVar6", "type": "longlong *", "storage": "RCX:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "uVar10", "type": "undefined8", "storage": "Stack[-0x1d0]:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "uVar11", "type": "undefined8", "storage": "Stack[-0x1c8]:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "puVar8", "type": "undefined *", "storage": "R8:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "lVar9", "type": "longlong", "storage": "R9:8", "is_phantom": false, "in_decompiled_code": true}, {"name": "bVar4", "type": "byte", "is_phantom": false, "in_decompiled_code": true}, {"name": "extraout_AL", "type": "char", "storage": "AL:1", "is_phantom": true, "in_decompiled_code": true}, {"name": "bVar2", "type": "byte", "is_phantom": false, "in_decompiled_code": true}, {"name": "bVar3", "type": "byte", "is_phantom": false, "in_decompiled_code": true}, {"name": "extraout_RAX_00", "type": "longlong", "storage": "RAX:8", "is_phantom": true, "in_decompiled_code": true}, {"name": "extraout_RAX_01", "type": "longlong *", "storage": "RAX:8", "is_phantom": true, "in_decompiled_code": true}, {"name": "uVar5", "type": "uint", "storage": "EAX:4", "is_phantom": false, "in_decompiled_code": true}, {"name": "extraout_RAX", "type": "longlong", "storage": "RAX:8", "is_phantom": true, "in_decompiled_code": true}]
// callees: [{"name": "VAL", "address": "140c18130"}, {"name": "are_scopes_on", "address": "140c16a00"}, {"name": "begin_scope", "address": "140c16d40"}, {"name": "initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>", "address": "14291a560"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "panic_already_borrowed", "address": "1428d9250"}, {"name": "rfind<ref$<str$>_>", "address": "1422790e0"}, {"name": "slice_error_fail", "address": "1428d9940"}]

void __cdecl forest::tree_animation::handplaced_tree_animation(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char extraout_AL;
  uint uVar5;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *plVar6;
  ulonglong uVar7;
  undefined8 *in_R8;
  undefined *puVar8;
  longlong lVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    uVar5 = *(uint *)*in_R8;
    lVar9 = 0;
    if (uVar5 - 6 < 0xf) {
      lVar9 = (ulonglong)uVar5 - 5;
    }
                    // WARNING: Could not emulate address calculation at 0x0001422795fa
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)&DAT_143038050 + (longlong)(int)(&DAT_143038050)[lVar9]))
              ((uint *)*in_R8,&DAT_143038050,
               (code *)((longlong)&DAT_143038050 + (longlong)(int)(&DAT_143038050)[lVar9]));
    return;
  }
  uVar7 = 0x31;
  core::str::impl$0::rfind<ref_<str_>_>();
  if (extraout_RAX == 1) {
    if (uVar7 != 0) {
      if (uVar7 < 0x31) {
        if ((char)(&DAT_143037f00)[uVar7] < -0x40) {
LAB_1422795bb:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else if (uVar7 != 0x31) goto LAB_1422795bb;
    }
    core::str::impl$0::rfind<ref_<str_>_>();
    if ((extraout_RAX_00 == 1) && (uVar7 != 0xfffffffffffffffe)) {
      if (uVar7 + 2 < 0x31) {
        if ((char)(&UNK_143037f02)[uVar7] < -0x40) {
LAB_142279642:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else if (uVar7 + 2 != 0x31) goto LAB_142279642;
    }
  }
  puVar8 = &DAT_14303802b;
  do {
    bVar2 = puVar8[-1];
    uVar5 = (uint)(char)bVar2;
    if ((char)bVar2 < '\0') {
      bVar3 = puVar8[-2];
      if ((char)bVar3 < -0x40) {
        bVar4 = puVar8[-3];
        if ((char)bVar4 < -0x40) {
          pbVar1 = puVar8 + -4;
          puVar8 = puVar8 + -4;
          uVar5 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar8 = puVar8 + -3;
          uVar5 = bVar4 & 0xf;
        }
        uVar5 = bVar3 & 0x3f | uVar5 << 6;
      }
      else {
        puVar8 = puVar8 + -2;
        uVar5 = bVar3 & 0x1f;
      }
      uVar5 = (uint)(bVar2 & 0x3f) | uVar5 << 6;
      if (uVar5 != 0x5c) goto LAB_142279704;
LAB_14227971b:
      if (puVar8 == &UNK_143037fff) break;
      if (puVar8 + -0x143037fff < (undefined *)0x2b) {
        if (-0x41 < (char)puVar8[1]) break;
      }
      else if (puVar8 + -0x143037fff == (undefined *)0x2b) break;
                    // WARNING: Subroutine does not return
      core::str::slice_error_fail();
    }
    puVar8 = puVar8 + -1;
    if (uVar5 == 0x5c) goto LAB_14227971b;
LAB_142279704:
    if (uVar5 == 0x2f) goto LAB_14227971b;
  } while (puVar8 != &DAT_143038000);
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_01 == 1) {
    plVar6 = extraout_RAX_01 + 1;
  }
  else if ((*extraout_RAX_01 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar6 = extraout_RAX_02, extraout_RAX_02 == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar6 != 0) {
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar6 = -1;
  uVar11 = 0;
  uVar10 = 1;
  puffin::ThreadProfiler::begin_scope();
  *plVar6 = *plVar6 + 1;
  uVar5 = *(uint *)*in_R8;
  lVar9 = 0;
  if (uVar5 - 6 < 0xf) {
    lVar9 = (ulonglong)uVar5 - 5;
  }
                    // WARNING: Could not emulate address calculation at 0x0001422797f7
                    // WARNING: Treating indirect jump as call
  (*(code *)((longlong)&DAT_143038050 + (longlong)(int)(&DAT_143038050)[lVar9]))
            ((uint *)*in_R8,&DAT_143038050,
             (code *)((longlong)&DAT_143038050 + (longlong)(int)(&DAT_143038050)[lVar9]),lVar9,
             uVar10,uVar11);
  return;
}

