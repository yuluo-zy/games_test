
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::free::impl$1::update(void)

{
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 local_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  
  uStack_78 = _UNK_142aece50;
  uStack_74 = (undefined4)_UNK_142aece54;
  uStack_70 = (undefined4)((ulonglong)_UNK_142aece54 >> 0x20);
  local_88 = _anon_f854cc98b5e1efe54f6ab98d53b9f85b_17_llvm_5637972437638668905;
  uStack_84 = _UNK_142aece44;
  uStack_80 = _UNK_142aece48;
  uStack_7c = _DAT_142aece4c;
  if (*(longlong *)(in_RDX + 0x10) == 0) {
    uStack_54 = 0;
    local_64 = (undefined4)__xmm_3f800000000000000000000000000000;
    uStack_60 = __xmm_3f800000000000000000000000000000._4_4_;
    uStack_5c = (undefined4)DAT_14293e0c8;
    uStack_58 = DAT_14293e0c8._4_4_;
    uVar3 = 0;
    uVar4 = 0;
  }
  else {
    puVar1 = *(undefined8 **)(in_RDX + 8);
    puVar2 = puVar1 + *(longlong *)(in_RDX + 0x10) * 2;
    do {
      (**(code **)(puVar1[1] + 0x30))(&local_64,*puVar1,&local_88);
      local_88 = local_64;
      uStack_84 = uStack_60;
      uStack_80 = uStack_5c;
      uStack_7c = uStack_58;
      uStack_78 = uStack_54;
      uStack_74 = (undefined4)uStack_50;
      uStack_70 = (undefined4)((ulonglong)uStack_50 >> 0x20);
      puVar1 = puVar1 + 2;
      uVar3 = uStack_74;
      uVar4 = uStack_70;
    } while (puVar1 != puVar2);
  }
  *(ulonglong *)(in_RDX + 0x24) = CONCAT44(uStack_78,uStack_7c);
  *(ulonglong *)(in_RDX + 0x2c) = CONCAT44(uStack_70,uStack_74);
  *(ulonglong *)(in_RDX + 0x18) = CONCAT44(uStack_84,local_88);
  *(ulonglong *)(in_RDX + 0x20) = CONCAT44(uStack_7c,uStack_80);
  *in_RCX = CONCAT44(uVar3,uStack_54);
  *(undefined4 *)(in_RCX + 1) = uVar4;
  *(undefined4 *)(in_RCX + 2) = local_64;
  *(undefined4 *)((longlong)in_RCX + 0x14) = uStack_60;
  *(undefined4 *)(in_RCX + 3) = uStack_5c;
  *(undefined4 *)((longlong)in_RCX + 0x1c) = uStack_58;
  return;
}

