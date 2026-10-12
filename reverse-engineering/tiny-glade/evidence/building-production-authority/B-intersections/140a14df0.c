
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
utils::float_grid::impl$0::
bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>(void)

{
  undefined1 auVar1 [16];
  undefined4 in_XMM1_Da;
  undefined4 in_XMM1_Db;
  undefined4 in_XMM2_Da;
  undefined4 in_XMM2_Db;
  undefined4 in_XMM3_Da;
  undefined4 in_XMM3_Db;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  uint in_stack_00000028;
  undefined1 auVar2 [16];
  
  auVar3._4_4_ = in_XMM2_Db;
  auVar3._0_4_ = in_XMM2_Da;
  auVar3._8_4_ = in_XMM2_Db;
  auVar3._12_4_ = in_XMM1_Db;
  auVar2._8_8_ = auVar3._8_8_;
  auVar2._4_4_ = in_XMM1_Da;
  auVar2._0_4_ = in_XMM2_Da;
  auVar1._4_8_ = 0;
  auVar1._0_4_ = in_stack_00000028;
  auVar1._12_4_ = in_XMM3_Db;
  auVar4._8_8_ = auVar1._8_8_;
  auVar4._4_4_ = in_XMM3_Da;
  auVar4._0_4_ = in_stack_00000028;
  auVar4 = divps(auVar2,auVar4);
  auVar5._0_4_ = auVar4._0_4_ + (float)__xmm_00000000000000003f0000003f000000;
  auVar5._4_4_ = auVar4._4_4_ + __xmm_00000000000000003f0000003f000000._4_4_;
  auVar5._8_4_ = auVar4._8_4_ + _UNK_1429258e8;
  auVar5._12_4_ = auVar4._12_4_ + _UNK_1429258ec;
  if (*__imp__ZN5utils10float_grid27USE_FLOAT_GRID_SAMPLING_FIX17h1da07c56e07a6074E == '\0') {
    auVar4 = maxps(ZEXT816(0),auVar5);
    auVar6._0_4_ = -(uint)!NAN(auVar5._0_4_) & auVar4._0_4_;
    auVar6._4_4_ = -(uint)!NAN(auVar5._4_4_) & auVar4._4_4_;
    auVar6._8_4_ = -(uint)!NAN(auVar5._8_4_) & auVar4._8_4_;
    auVar6._12_4_ = -(uint)!NAN(auVar5._12_4_) & auVar4._12_4_;
    minps(___xmm_00000000000000003f8000003f800000,auVar6);
  }
  lerp();
  lerp();
  lerp();
  return;
}

