// addr=0x14090dfe0 name=is_within_glade_shape
// signature=void __cdecl is_within_glade_shape(void)


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::game_config::is_within_glade_shape(void)

{
  undefined1 auVar1 [16];
  float in_XMM0_Da;
  float in_XMM0_Db;
  float in_XMM1_Da;
  float in_XMM1_Db;
  undefined4 in_XMM1_Dc;
  undefined1 auVar3 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar2 [16];
  undefined1 auVar4 [16];
  
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
    auVar5._4_4_ = in_XMM0_Db;
    auVar5._0_4_ = in_XMM0_Da;
    auVar5._8_4_ = in_XMM0_Db;
    auVar5._12_4_ = in_XMM1_Db;
    auVar4._8_8_ = auVar5._8_8_;
    auVar4._4_4_ = in_XMM1_Da;
    auVar4._0_4_ = in_XMM0_Da;
    auVar8 = maxps(___xmm_0000000000000000c2820000c2820000,auVar4);
    auVar6._0_4_ = ~-(uint)NAN(in_XMM0_Da) & auVar8._0_4_;
    auVar6._4_4_ = ~-(uint)NAN(in_XMM1_Da) & auVar8._4_4_;
    auVar6._8_4_ = ~-(uint)NAN(in_XMM0_Db) & auVar8._8_4_;
    auVar6._12_4_ = ~-(uint)NAN(in_XMM1_Db) & auVar8._12_4_;
    auVar8._4_4_ = __xmm_0000000000000000c2820000c2820000._4_4_ & -(uint)NAN(in_XMM1_Da);
    auVar8._0_4_ = (uint)__xmm_0000000000000000c2820000c2820000 & -(uint)NAN(in_XMM0_Da);
    auVar8._8_4_ = __xmm_0000000000000000c2820000c2820000._8_4_ & -(uint)NAN(in_XMM0_Db);
    auVar8._12_4_ = __xmm_0000000000000000c2820000c2820000._12_4_ & -(uint)NAN(in_XMM1_Db);
    minps(___xmm_00000000000000004282000042820000,auVar6 | auVar8);
  }
  else {
    fVar7 = in_XMM1_Da * in_XMM1_Da + in_XMM0_Da * in_XMM0_Da;
    auVar3._4_4_ = in_XMM0_Db;
    auVar3._0_4_ = in_XMM0_Da;
    auVar3._8_4_ = in_XMM0_Db;
    auVar3._12_4_ = in_XMM1_Db;
    auVar2._8_8_ = auVar3._8_8_;
    auVar2._4_4_ = in_XMM1_Da;
    auVar2._0_4_ = in_XMM0_Da;
    if (4225.0 < fVar7) {
      fVar7 = SQRT(fVar7);
      auVar1._4_4_ = fVar7;
      auVar1._0_4_ = fVar7;
      auVar1._8_4_ = in_XMM1_Dc;
      auVar1._12_4_ = in_XMM1_Dc;
      divps(auVar2,auVar1);
    }
  }
  return;
}


