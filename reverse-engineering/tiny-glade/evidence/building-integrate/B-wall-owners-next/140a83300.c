
void __cdecl country_core::resources::history::wall_history::impl$0::ev_new_wall(void)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  void *_Src;
  ulonglong uVar4;
  undefined1 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  void *_Dst;
  undefined8 *extraout_RAX_03;
  void *_Dst_00;
  undefined8 in_RDX;
  undefined ***_Src_00;
  undefined8 *in_R8;
  uint in_R9D;
  float fVar5;
  float fVar6;
  undefined **local_448;
  code *pcStack_440;
  undefined ***local_438;
  undefined8 uStack_430;
  void *local_428;
  ulonglong uStack_420;
  undefined8 local_418;
  undefined8 uStack_410;
  undefined8 local_408;
  undefined8 uStack_400;
  undefined8 local_3f8;
  undefined8 uStack_3f0;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  undefined8 uStack_3c0;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  undefined8 local_3a8;
  undefined8 uStack_3a0;
  undefined8 local_398;
  undefined8 uStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 local_360;
  float local_358;
  undefined *local_348;
  undefined ***local_340;
  undefined1 *local_338;
  undefined ***local_330;
  ulonglong local_328;
  undefined1 *local_320;
  ulonglong local_318;
  undefined4 local_310;
  undefined4 local_308;
  undefined8 local_2ec;
  undefined8 uStack_2e4;
  undefined1 local_2dc;
  undefined1 local_2db;
  undefined1 local_2da;
  undefined1 local_2d9;
  undefined1 local_2d8;
  undefined1 local_2d7;
  undefined8 local_2d0;
  undefined8 local_2c8;
  uint local_2c0;
  undefined **local_250;
  void *pvStack_248;
  undefined ***local_240;
  undefined8 *puStack_238;
  void *local_230;
  ulonglong uStack_228;
  undefined8 local_220;
  undefined **local_148;
  void *pvStack_140;
  undefined ***local_138;
  undefined8 uStack_130;
  void *local_128;
  ulonglong uStack_120;
  undefined8 local_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 local_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined1 local_e4;
  undefined1 local_e3;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined1 local_e0;
  undefined1 local_df;
  undefined6 uStack_de;
  undefined8 local_d0;
  undefined8 *puStack_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 *local_a8;
  undefined1 *local_a0;
  undefined1 local_98;
  undefined7 uStack_97;
  ulonglong local_90;
  undefined ***local_88;
  uint local_80;
  undefined1 local_79;
  undefined1 local_78;
  undefined1 local_77;
  undefined1 local_76;
  undefined1 local_75;
  undefined1 local_74;
  undefined1 local_73;
  undefined1 local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  fVar6 = *(float *)(in_R8 + 6);
  if ((fVar6 == 0.0) && (!NAN(fVar6))) {
    local_71 = 1;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar5 = 0.0;
  if (0.0 <= __real_3e4ccccd / fVar6) {
    fVar5 = __real_3e4ccccd / fVar6;
  }
  fVar6 = __real_3f800000;
  if (fVar5 <= __real_3f800000) {
    fVar6 = fVar5;
  }
  local_348 = (undefined *)CONCAT44(local_348._4_4_,fVar6);
  if ((fVar6 < 0.0) || (__real_3f800000 < fVar6)) {
    local_448 = &local_348;
    pcStack_440 = core::fmt::float::impl$5::fmt;
    local_250 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
    pvStack_248 = (void *)0x1;
    local_230 = (void *)0x0;
    local_240 = &local_448;
    puStack_238 = (undefined8 *)0x1;
    local_71 = 1;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  if (NAN(fVar6)) {
    local_71 = 1;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  local_71 = 1;
  local_a8 = in_R8;
  ::utils::curve::Curve::crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  local_e4 = *(undefined1 *)((longlong)in_R8 + 100);
  local_e3 = *(undefined1 *)((longlong)in_R8 + 0x65);
  local_e0 = *(undefined1 *)(in_R8 + 0xd);
  local_df = *(undefined1 *)((longlong)in_R8 + 0x69);
  local_e2 = *(undefined1 *)((longlong)in_R8 + 0x66);
  uVar2 = *(undefined8 *)((longlong)in_R8 + 0x54);
  uVar3 = *(undefined8 *)((longlong)in_R8 + 0x5c);
  local_e1 = *(undefined1 *)((longlong)in_R8 + 0x67);
  local_118 = local_220;
  local_128 = local_230;
  uStack_120 = uStack_228;
  local_138 = local_240;
  uStack_130 = puStack_238;
  local_148 = local_250;
  pvStack_140 = pvStack_248;
  uStack_110 = 2;
  local_90 = CONCAT71(local_90._1_7_,local_e0);
  _local_98 = CONCAT71(uStack_97,local_df);
  local_f4 = (undefined4)uVar2;
  uStack_f0 = (undefined4)((ulonglong)uVar2 >> 0x20);
  uStack_ec = (undefined4)uVar3;
  uStack_e8 = (undefined4)((ulonglong)uVar3 >> 0x20);
  sVar1 = (longlong)local_240 * 8;
  local_79 = local_e4;
  local_78 = local_e3;
  local_77 = local_e2;
  if (((ulonglong)local_240 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar1)) {
LAB_140a839f3:
    local_72 = 1;
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  local_b8 = in_RDX;
  local_80 = in_R9D;
  local_76 = local_e1;
  if (sVar1 == 0) {
    local_a0 = &DAT_00000004;
    local_88 = (undefined ***)0x0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_140a839f3;
    local_88 = local_240;
    local_a0 = extraout_RAX;
  }
  memcpy(local_a0,pvStack_248,sVar1);
  uVar4 = uStack_120;
  _Src = local_128;
  sVar1 = uStack_120 * 4;
  if ((uStack_120 >> 0x3e == 0) && (sVar1 < 0x7ffffffffffffffd)) {
    if (sVar1 == 0) {
      local_320 = &DAT_00000004;
      local_328 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_328 = uVar4;
      local_320 = extraout_RAX_00;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140a83a11;
    }
    memcpy(local_320,_Src,sVar1);
    local_340 = local_88;
    local_338 = local_a0;
    local_330 = local_240;
    local_318 = uVar4;
    local_2d0 = local_b8;
    local_310 = (undefined4)local_118;
    local_308 = 2;
    local_2dc = local_79;
    local_2db = local_78;
    local_2da = local_77;
    local_2d9 = local_76;
    local_2d8 = (undefined1)local_90;
    local_2d7 = local_98;
    local_2c0 = local_80;
    local_348 = (undefined *)0x8000000000000000;
    local_3e8 = CONCAT17(local_e1,CONCAT16(local_e2,CONCAT15(local_e3,CONCAT14(local_e4,uStack_e8)))
                        );
    uStack_3e0 = CONCAT62(uStack_de,CONCAT11(local_df,local_e0));
    local_3f8 = CONCAT44(local_f4,uStack_f8);
    uStack_3f0 = CONCAT44(uStack_ec,uStack_f0);
    local_408 = local_108;
    uStack_400 = uStack_100;
    uStack_410 = CONCAT44(uStack_10c,uStack_110);
    local_418 = local_118;
    local_428 = local_128;
    uStack_420 = uStack_120;
    local_438 = local_138;
    uStack_430 = uStack_130;
    local_448 = local_148;
    pcStack_440 = pvStack_140;
    local_3d8 = *local_a8;
    uStack_3d0 = local_a8[1];
    local_3c8 = local_a8[2];
    uStack_3c0 = local_a8[3];
    local_3b8 = local_a8[4];
    uStack_3b0 = local_a8[5];
    local_3a8 = local_a8[6];
    uStack_3a0 = local_a8[7];
    local_398 = local_a8[8];
    uStack_390 = local_a8[9];
    local_388 = local_a8[10];
    uStack_380 = local_a8[0xb];
    local_378 = local_a8[0xc];
    uStack_370 = local_a8[0xd];
    local_368 = local_b8;
    local_360 = 0;
    local_358 = fVar6;
    local_2ec = uVar2;
    uStack_2e4 = uVar3;
    local_2c8 = uVar2;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
      local_73 = 1;
      local_74 = 1;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
      local_80 = CONCAT31(local_80._1_3_,1);
      local_75 = 1;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    memcpy(&local_240,&local_348,0xf8);
    local_250 = (undefined **)0x1;
    pvStack_248 = (void *)0x1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
      local_80 = local_80 & 0xffffff00;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    memcpy(_Dst,&local_250,0x108);
    *extraout_RAX_02 = _Dst;
    local_d0 = 1;
    local_c0 = 1;
    puStack_c8 = extraout_RAX_02;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 == (undefined8 *)0x0) {
      local_90 = CONCAT71(local_90._1_7_,1);
      local_80 = local_80 & 0xffffff00;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    memcpy(&local_240,&local_448,0xf8);
    local_250 = (undefined **)0x1;
    pvStack_248 = (void *)0x1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst_00 == (void *)0x0) {
      local_90 = local_90 & 0xffffffffffffff00;
      local_80 = local_80 & 0xffffff00;
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    _Src_00 = &local_250;
    memcpy(_Dst_00,_Src_00,0x108);
    *extraout_RAX_03 = _Dst_00;
    extraout_RAX_01[2] = local_c0;
    *extraout_RAX_01 = local_d0;
    extraout_RAX_01[1] = puStack_c8;
    extraout_RAX_01[3] = 1;
    extraout_RAX_01[4] = extraout_RAX_03;
    extraout_RAX_01[5] = 1;
    puStack_238 = extraout_RAX_01 + 6;
    local_240 = (undefined ***)0x2;
    local_73 = 0;
    local_74 = 0;
    alloc::boxed::iter::impl$13::
    from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
              ();
    local_250 = (undefined **)0x7;
    local_73 = 0;
    local_74 = 0;
    local_240 = _Src_00;
    History::add();
    return;
  }
LAB_140a83a11:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

