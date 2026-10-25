// ADDR 0x14080d3d0
// SIG void __cdecl auto_allow_decorator_culling(void)
// ORIG system::auto_allow_decorator_culling


void __cdecl system_decorator::cull_oob_decorators::auto_allow_decorator_culling(void)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  ulonglong uVar3;
  undefined4 *puVar4;
  char extraout_AL;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong in_R8;
  ulonglong uVar9;
  undefined8 *in_R9;
  ulonglong uVar10;
  longlong lVar11;
  
  country_core::resources::history::History::is_recording();
  if (extraout_AL == '\0') {
    return;
  }
  puVar2 = (ulonglong *)*in_R9;
  lVar6 = in_R9[1];
  uVar3 = *puVar2;
  uVar8 = 0;
  uVar10 = uVar3 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar3 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar10 = uVar8;
  }
  uVar9 = uVar3 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar3 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar9 = uVar8;
  }
  lVar5 = uVar10 * 0x30 + *(longlong *)(lVar6 + 8);
  uVar3 = *(ulonglong *)(lVar6 + 0x10) - uVar10;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar10) {
    lVar5 = 8;
    uVar3 = uVar8;
  }
  lVar11 = uVar9 * 0x30 + *(longlong *)(lVar6 + 0x28);
  uVar10 = *(ulonglong *)(lVar6 + 0x30) - uVar9;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar9) {
    lVar11 = 8;
    uVar10 = uVar8;
  }
  *puVar2 = (*(longlong *)(lVar6 + 0x40) - uVar10) - uVar3;
  lVar6 = uVar3 * 0x30 + lVar5;
  lVar7 = uVar10 * 0x30 + lVar11;
  uVar1 = *(undefined4 *)(in_R8 + 0x1c);
  puVar4 = *(undefined4 **)(in_R8 + 0x10);
  do {
    if ((lVar5 == 0) || (lVar5 == lVar6)) {
      if ((lVar11 == 0) || (lVar11 == lVar7)) {
        *puVar4 = uVar1;
        country_core::resources::walls::decorator_archivist::DecoratorArchivist::
        decorator_cull_allow_stairs();
        return;
      }
      lVar11 = lVar11 + 0x30;
      lVar5 = 0;
    }
    else {
      lVar5 = lVar5 + 0x30;
    }
    *puVar2 = *puVar2 + 1;
    *puVar4 = uVar1;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorator_ids();
    _ZN121__LT_hashbrown__map__HashMap_LT_K_C_V_C_S_C_A_GT__u20_as_u20_core__iter__traits__collect__Extend_LT__LP_K_C_V_RP__GT__GT_6extend17hbc9c89e6eace21ecE
              ();
  } while( true );
}


