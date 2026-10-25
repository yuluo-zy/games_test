
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_center(void)

{
  int iVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  int iVar4;
  int iVar5;
  longlong in_RCX;
  int in_EDX;
  float fVar6;
  undefined1 auVar7 [16];
  undefined4 in_XMM3_Dc;
  undefined4 in_XMM4_Db;
  float fVar9;
  undefined4 in_XMM5_Dc;
  undefined1 auVar10 [16];
  undefined1 auVar8 [16];
  
  iVar1 = *(int *)(in_RCX + 0x48);
  if (iVar1 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_by_zero();
  }
  if (in_EDX != -0x80000000 || iVar1 != -1) {
    fVar6 = floorf((float)in_EDX / (float)iVar1);
    iVar5 = 0x7fffffff;
    if (fVar6 <= 2.1474835e+09) {
      iVar5 = (int)fVar6;
    }
    iVar4 = 0;
    if (!NAN(fVar6)) {
      iVar4 = iVar5;
    }
    fVar6 = (float)(int)*(undefined8 *)(in_RCX + 0x48);
    fVar9 = (float)(int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
    auVar10._4_4_ = (float)(in_EDX % iVar1);
    auVar10._0_4_ = *(undefined4 *)(in_RCX + 0x50);
    auVar10._8_8_ = 0;
    auVar2._4_4_ = fVar6;
    auVar2._0_4_ = fVar6;
    auVar2._8_4_ = in_XMM3_Dc;
    auVar2._12_4_ = in_XMM3_Dc;
    divps(auVar10,auVar2);
    auVar8._4_8_ = 0;
    auVar8._0_4_ = *(uint *)(in_RCX + 0x54);
    auVar8._12_4_ = in_XMM4_Db;
    auVar7._8_8_ = auVar8._8_8_;
    auVar7._4_4_ = (float)iVar4;
    auVar7._0_4_ = *(uint *)(in_RCX + 0x54);
    auVar3._4_4_ = fVar9;
    auVar3._0_4_ = fVar9;
    auVar3._8_4_ = in_XMM5_Dc;
    auVar3._12_4_ = in_XMM5_Dc;
    divps(auVar7,auVar3);
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_const::panic_const_rem_overflow();
}

