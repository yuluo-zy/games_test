// address: 0x1409dca90
// pdb name: add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>
// signature: void __cdecl add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>(void)

void __cdecl
rhapsody::rg::pass_builder::PassBuilder::
add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>(void)

{
  int *piVar1;
  char cVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  char extraout_AL;
  char extraout_AL_00;
  undefined4 extraout_EAX;
  ulonglong *in_RCX;
  longlong lVar6;
  undefined4 *puVar7;
  uint in_EDX;
  char in_R8B;
  undefined1 uVar8;
  undefined4 in_R9D;
  ulonglong uVar9;
  char *pcVar10;
  ulonglong uVar11;
  undefined1 local_d5;
  undefined4 local_d4;
  uint local_d0;
  undefined1 local_cc [4];
  ulonglong local_c8;
  undefined1 *local_c0;
  code *local_b8;
  char *local_b0;
  code *local_a8;
  undefined4 *local_a0;
  code *local_98;
  undefined1 *local_90;
  code *local_88;
  undefined4 *local_80;
  code *local_78;
  undefined **local_70;
  undefined8 local_68;
  undefined1 **local_60;
  undefined8 local_58;
  undefined8 local_50;
  
  uVar11 = *in_RCX;
  if (SBORROW8(0,uVar11)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (*(ulonglong *)(in_RCX[8] + 0x40) <= (ulonglong)in_EDX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar3 = *(longlong *)(in_RCX[8] + 0x38);
  lVar6 = (ulonglong)in_EDX * 0x10;
  piVar1 = (int *)(lVar3 + lVar6);
  uVar5 = in_RCX[9];
  if (*(int *)(lVar3 + lVar6) != (int)uVar5) {
    uVar9 = in_RCX[2];
    local_d4 = in_R9D;
    goto LAB_1409dcbbe;
  }
  uVar4 = *(ulonglong *)(piVar1 + 2);
  uVar9 = in_RCX[2];
  local_c8 = uVar11;
  if (uVar9 <= uVar4) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  uVar11 = in_RCX[1];
  puVar7 = (undefined4 *)(uVar11 + uVar4 * 0xc + 4);
  local_d0 = in_EDX;
  if ((*(char *)(uVar11 + 8 + uVar4 * 0xc) == in_R8B) &&
     (recorder::ResourceSyncGroup::is_equivalent(), extraout_AL != '\0')) {
    return;
  }
  pcVar10 = (char *)(uVar11 + uVar4 * 0xc + 8);
  recorder::ResourceSyncGroup::is_equivalent();
  if (extraout_AL_00 == '\0') {
LAB_1409dcb90:
    if ((in_R8B != '7') || (1 < (byte)(*pcVar10 - 0x19U))) {
LAB_1409dcc37:
      local_c0 = local_cc;
      local_b8 = resource::impl$39::fmt;
      local_a8 = vk_sync_fork::impl$1::fmt;
      local_98 = recorder::impl$12::fmt;
      local_90 = &local_d5;
      local_88 = vk_sync_fork::impl$1::fmt;
      local_78 = recorder::impl$12::fmt;
      local_70 = &PTR_s_Previous_access_of_resource_alre_142ac3a00;
      local_68 = 6;
      local_50 = 0;
      local_60 = &local_c0;
      local_58 = 5;
      local_b0 = pcVar10;
      local_a0 = puVar7;
      local_80 = &local_d4;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
  }
  else {
    cVar2 = *pcVar10;
    if (cVar2 == '\x17') {
      if (in_R8B == '\x1d') {
LAB_1409dcc26:
        *pcVar10 = '\x1e';
        return;
      }
      goto LAB_1409dcc37;
    }
    if (cVar2 != '\x1a') {
      if (cVar2 == '\x1d') {
        if (in_R8B == '\x17') goto LAB_1409dcc26;
        goto LAB_1409dcc37;
      }
      goto LAB_1409dcb90;
    }
    if (in_R8B == '\x02') {
      *pcVar10 = '\x02';
      return;
    }
    if (in_R8B != '7') goto LAB_1409dcc37;
  }
  *pcVar10 = '7';
  local_d4 = 0;
  *puVar7 = 0;
  uVar11 = local_c8;
  in_EDX = local_d0;
LAB_1409dcbbe:
  uVar8 = (undefined1)local_d4;
  *piVar1 = (int)uVar5;
  *(ulonglong *)(piVar1 + 2) = uVar9;
  country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
  if (uVar9 == uVar11) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
              ();
  }
  uVar11 = in_RCX[1];
  *(uint *)(uVar11 + uVar9 * 0xc) = in_EDX;
  *(undefined4 *)(uVar11 + 4 + uVar9 * 0xc) = extraout_EAX;
  *(undefined1 *)(uVar11 + 8 + uVar9 * 0xc) = uVar8;
  in_RCX[2] = uVar9 + 1;
  return;
}


