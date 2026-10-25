
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::asset_libraries::shader_library::AssetShaderLibrary::new(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined *puVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined *puVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  longlong *extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar13;
  longlong *extraout_RAX_02;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined7 uStack_ff;
  uint7 local_98;
  undefined1 uStack_91;
  undefined7 uStack_90;
  undefined1 uStack_89;
  undefined7 uStack_88;
  undefined1 uStack_81;
  undefined7 uStack_80;
  undefined1 uStack_79;
  
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    std::thread::local::panic_access_error();
  }
  lVar1 = *extraout_RAX;
  lVar2 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  uVar12 = _UNK_142ab5b78;
  uVar11 = _DAT_142ab5b70;
  uVar10 = _UNK_142ab5b68;
  puVar9 = anon_a3e9af60763f14b22f2dc37588e49665_33_llvm_1322674437799810495;
  uStack_91 = SUB81(anon_a3e9af60763f14b22f2dc37588e49665_33_llvm_1322674437799810495,0);
  uStack_90 = (undefined7)
              ((ulonglong)anon_a3e9af60763f14b22f2dc37588e49665_33_llvm_1322674437799810495 >> 8);
  uStack_89 = (undefined1)_UNK_142ab5b68;
  uStack_88 = (undefined7)((ulonglong)_UNK_142ab5b68 >> 8);
  uStack_81 = (undefined1)_DAT_142ab5b70;
  uStack_80 = (undefined7)((ulonglong)_DAT_142ab5b70 >> 8);
  uStack_79 = (undefined1)_UNK_142ab5b78;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_00 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_00[0] = 1;
    extraout_RAX_00[1] = 0;
    extraout_RAX_00[2] = 0;
    extraout_RAX_00[3] = 0;
    extraout_RAX_00[4] = 0;
    extraout_RAX_00[5] = 0;
    extraout_RAX_00[6] = 0;
    extraout_RAX_00[7] = 0;
    *(longlong *)(extraout_RAX_00 + 8) = extraout_RAX_01;
    *(undefined8 *)(extraout_RAX_00 + 0x10) = in_RDX;
    lVar13 = extraout_RAX_01;
  }
  else {
    lVar13 = *(longlong *)(extraout_RAX_00 + 8);
    in_RDX = *(undefined8 *)(extraout_RAX_00 + 0x10);
  }
  *(longlong *)(extraout_RAX_00 + 8) = lVar13 + 1;
  uVar8 = _UNK_142ab0ca8;
  uVar7 = _DAT_142ab0ca0;
  uVar6 = _UNK_142ab0c98;
  puVar5 = anon_2d944acd27646b19de27e02456f85def_37_llvm_220559307296278160;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 != (longlong *)0x0) {
    lVar3 = *extraout_RAX_02;
    lVar4 = extraout_RAX_02[1];
    *extraout_RAX_02 = *extraout_RAX_02 + 1;
    *in_RCX = 0;
    in_RCX[1] = (ulonglong)local_98 << 8;
    in_RCX[2] = puVar9;
    in_RCX[3] = uVar10;
    in_RCX[4] = uVar11;
    in_RCX[5] = uVar12;
    in_RCX[6] = lVar1;
    in_RCX[7] = lVar2;
    in_RCX[0x10] = 0;
    in_RCX[0x11] = puVar5;
    in_RCX[0x12] = uVar6;
    in_RCX[0x13] = uVar7;
    in_RCX[0x14] = uVar8;
    in_RCX[0x15] = lVar13;
    in_RCX[0x16] = in_RDX;
    in_RCX[8] = 0;
    *(undefined1 *)(in_RCX + 9) = 0;
    in_RCX[0xd] = uVar12;
    *(ulonglong *)((longlong)in_RCX + 0x49) = CONCAT17(uStack_91,uStack_ff);
    *(ulonglong *)((longlong)in_RCX + 0x51) = CONCAT17(uStack_89,uStack_90);
    *(ulonglong *)((longlong)in_RCX + 0x59) = CONCAT17(uStack_81,uStack_88);
    *(ulonglong *)((longlong)in_RCX + 0x61) = CONCAT17(uStack_79,uStack_80);
    in_RCX[0xe] = lVar3;
    in_RCX[0xf] = lVar4;
    in_RCX[0x17] = 0;
    in_RCX[0x18] = puVar5;
    in_RCX[0x19] = uVar6;
    in_RCX[0x1a] = uVar7;
    in_RCX[0x1b] = uVar8;
    return;
  }
                    /* WARNING: Subroutine does not return */
  std::thread::local::panic_access_error();
}

