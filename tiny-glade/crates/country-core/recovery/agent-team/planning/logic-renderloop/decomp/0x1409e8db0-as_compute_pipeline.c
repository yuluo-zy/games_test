// 0x1409e8db0 as_compute_pipeline size=818

void __cdecl
country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined **ppuVar8;
  longlong lVar9;
  char extraout_AL;
  undefined8 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  undefined4 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong lVar10;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 *puStack_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 *local_50;
  undefined **ppuStack_48;
  longlong local_40;
  undefined8 *local_38;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  builtin_strncpy((char *)((longlong)extraout_RAX + 0x1a),"to_cells",8);
  builtin_strncpy((char *)((longlong)extraout_RAX + 0x22),".cs.hlsl",8);
  extraout_RAX[2] = 0x746867696c5f6e69;
  extraout_RAX[3] = 0x6c65635f6f745f73;
  *extraout_RAX = 0x625f746867696c2f;
  extraout_RAX[1] = 0x622f676e696e6e69;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_00 + 3) = 0x6e69616d;
  *extraout_RAX_00 = 0x6d5f7363;
  local_80 = 0x2a;
  local_70 = 0x2a;
  local_68 = 7;
  local_58 = 7;
  local_50 = (undefined8 *)0x0;
  ppuStack_48 = (undefined **)0x8;
  local_40 = 0;
  puStack_78 = extraout_RAX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_01 + 3) = 0x6e69616d;
  *extraout_RAX_01 = 0x6d5f7363;
  std::alloc::__default_lib_allocator::__rust_dealloc();
  lVar9 = local_40;
  ppuVar8 = ppuStack_48;
  puVar7 = local_50;
  uVar6 = local_70;
  uVar5 = local_80;
  local_68 = 7;
  local_58 = 7;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    *extraout_RAX_02 = 0x6972775f4d524550;
    *(undefined2 *)(extraout_RAX_02 + 1) = 0x6574;
    local_98 = 0;
    uStack_90 = 1;
    local_88 = 0;
    local_80 = 0;
    local_70 = 0;
    local_60 = 0x20;
    local_58 = CONCAT71(local_58._1_7_,3);
    local_50 = &local_98;
    ppuStack_48 = &anon_f9f771d881507ddb72d8ac01f7aacbf3_6_llvm_8833080502119418031;
    local_38 = extraout_RAX_02;
    core::fmt::num::imp::impl$7::fmt();
    uVar4 = local_88;
    uVar3 = uStack_90;
    uVar2 = local_98;
    if (extraout_AL == '\0') {
      if ((undefined8 *)lVar9 == puVar7) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<alloc::string::String,alloc::string::String>,alloc::alloc::Global>();
      }
      lVar10 = lVar9 * 0x30;
      puVar1 = (undefined8 *)((longlong)ppuVar8 + 0x20 + lVar10);
      *puVar1 = uVar3;
      puVar1[1] = uVar4;
      puVar1 = (undefined8 *)((longlong)ppuVar8 + 0x10 + lVar10);
      *puVar1 = 10;
      puVar1[1] = uVar2;
      *(undefined8 *)((longlong)ppuVar8 + lVar10) = 10;
      ((undefined8 *)((longlong)ppuVar8 + lVar10))[1] = local_38;
      in_RCX[8] = lVar9 + 1;
      in_RCX[6] = puVar7;
      in_RCX[7] = ppuVar8;
      in_RCX[4] = extraout_RAX_01;
      in_RCX[5] = 7;
      in_RCX[2] = uVar6;
      in_RCX[3] = 7;
      *in_RCX = uVar5;
      in_RCX[1] = puStack_78;
      return;
    }
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

