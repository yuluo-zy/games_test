
void __cdecl country_core::systems::wall::deferred_wall_history::history_record_wall_changed(void)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *_Dst;
  longlong in_RCX;
  longlong *in_RDX;
  undefined8 *_Src;
  bool bVar3;
  undefined8 local_330;
  undefined8 local_328;
  undefined8 *local_320;
  undefined8 *local_318;
  undefined8 local_240;
  undefined8 local_238;
  undefined4 local_230;
  longlong local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 local_58;
  undefined4 local_50;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar1 = *in_RDX;
  local_d0 = *(longlong *)(lVar1 + 0x120);
  uVar2 = *(undefined8 *)(lVar1 + 0x200);
  *(undefined8 *)(lVar1 + 0x120) = 0x8000000000000000;
  if (!SBORROW8(0,local_d0)) {
    local_50 = *(undefined4 *)(lVar1 + 0x210);
    local_58 = *(undefined8 *)(lVar1 + 0x208);
    uStack_c8 = *(undefined8 *)(lVar1 + 0x128);
    uStack_c0 = *(undefined8 *)(lVar1 + 0x130);
    uStack_b8 = *(undefined8 *)(lVar1 + 0x138);
    uStack_b0 = *(undefined8 *)(lVar1 + 0x140);
    local_a8 = *(undefined8 *)(lVar1 + 0x148);
    uStack_a0 = *(undefined8 *)(lVar1 + 0x150);
    local_98 = *(undefined8 *)(lVar1 + 0x158);
    uStack_90 = *(undefined8 *)(lVar1 + 0x160);
    local_88 = *(undefined8 *)(lVar1 + 0x168);
    uStack_80 = *(undefined8 *)(lVar1 + 0x170);
    local_78 = *(undefined8 *)(lVar1 + 0x178);
    uStack_70 = *(undefined8 *)(lVar1 + 0x180);
    local_68 = *(undefined8 *)(lVar1 + 0x188);
    local_e8 = *(undefined8 *)(lVar1 + 0x1f0);
    uStack_e0 = *(undefined8 *)(lVar1 + 0x1f8);
    local_f8 = *(undefined8 *)(lVar1 + 0x1e0);
    uStack_f0 = *(undefined8 *)(lVar1 + 0x1e8);
    local_108 = *(undefined8 *)(lVar1 + 0x1d0);
    uStack_100 = *(undefined8 *)(lVar1 + 0x1d8);
    local_148 = *(undefined8 *)(lVar1 + 400);
    uStack_140 = *(undefined8 *)(lVar1 + 0x198);
    local_138 = *(undefined8 *)(lVar1 + 0x1a0);
    uStack_130 = *(undefined8 *)(lVar1 + 0x1a8);
    local_128 = *(undefined8 *)(lVar1 + 0x1b0);
    uStack_120 = *(undefined8 *)(lVar1 + 0x1b8);
    local_118 = *(undefined8 *)(lVar1 + 0x1c0);
    uStack_110 = *(undefined8 *)(lVar1 + 0x1c8);
    **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
      local_39 = 1;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    bVar3 = extraout_RAX_00 == (undefined8 *)0x0;
    local_3a = bVar3;
    if (bVar3) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    local_1c8 = uStack_70;
    uStack_1c0 = local_68;
    local_1d8 = uStack_80;
    uStack_1d0 = local_78;
    local_1e8 = uStack_90;
    uStack_1e0 = local_88;
    local_1f8 = uStack_a0;
    uStack_1f0 = local_98;
    local_208 = uStack_b0;
    uStack_200 = local_a8;
    local_218 = uStack_c0;
    uStack_210 = uStack_b8;
    local_228 = local_d0;
    uStack_220 = uStack_c8;
    local_1b8 = local_148;
    uStack_1b0 = uStack_140;
    local_1a8 = local_138;
    uStack_1a0 = uStack_130;
    local_198 = local_128;
    uStack_190 = uStack_120;
    local_188 = local_118;
    uStack_180 = uStack_110;
    local_178 = local_108;
    uStack_170 = uStack_100;
    local_168 = local_f8;
    uStack_160 = uStack_f0;
    local_158 = local_e8;
    uStack_150 = uStack_e0;
    local_230 = local_50;
    local_238 = local_58;
    local_330 = 1;
    local_328 = 1;
    memcpy(&local_320,&local_228,0xe0);
    local_240 = uVar2;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    _Src = &local_330;
    memcpy(_Dst,_Src,0x108);
    *extraout_RAX_00 = _Dst;
    *extraout_RAX = 1;
    extraout_RAX[1] = extraout_RAX_00;
    extraout_RAX[2] = 1;
    local_318 = extraout_RAX + 3;
    local_320 = (undefined8 *)0x1;
    local_39 = 0;
    alloc::boxed::iter::impl$13::
    from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
              ();
    local_330 = 7;
    local_39 = 0;
    local_320 = _Src;
    resources::history::History::add();
  }
  return;
}

