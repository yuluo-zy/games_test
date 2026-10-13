
/* WARNING: Removing unreachable block (ram,0x000140b9da57) */
/* WARNING: Removing unreachable block (ram,0x000140b9da5d) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined * FUN_140b9d9b0(undefined8 param_1,longlong *param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  undefined4 *puVar3;
  undefined *puVar4;
  undefined *puStack_98;
  undefined8 uStack_90;
  longlong *plStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined *puStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  
  puStack_98 = &DAT_143634da8;
  plVar2 = (longlong *)FUN_141849000(&puStack_98);
  if (param_2 == (longlong *)0x0) {
    uStack_90 = 0;
    plStack_88 = (longlong *)0x0;
    if (plVar2 != (longlong *)0x0) {
      LOCK();
      if (*plVar2 == 0) {
        *plVar2 = 3;
      }
      UNLOCK();
    }
    puVar4 = &UNK_142afd69a;
    if (*_DAT_14362c3a8 != 0) {
      puVar3 = (undefined4 *)func_0x00014067dd80(&UNK_142afd8c0);
      uStack_70 = *puVar3;
      uStack_6c = puVar3[1];
      uStack_68 = puVar3[2];
      uStack_64 = puVar3[3];
      uStack_44 = puVar3[4];
      uStack_60 = 1;
      puStack_58 = &UNK_142afd730;
      uStack_50 = 8;
      puStack_40 = &UNK_142afd8b0;
      uStack_38 = 1;
      uStack_30 = 8;
      uStack_90 = 0;
      uStack_28 = 0;
      uStack_20 = 0;
      plStack_88 = (longlong *)&UNK_142afd730;
      uStack_80 = 8;
      uStack_78 = 0;
      uStack_48 = 1;
      func_0x000140d4a330(&puStack_98,&uStack_90);
    }
  }
  else {
    puVar4 = (undefined *)param_2[2];
    uStack_90 = 0;
    if (plVar2 != (longlong *)0x0) {
      LOCK();
      plVar1 = (longlong *)*plVar2;
      if (param_2 + 2 == plVar1) {
        *plVar2 = 3;
      }
      UNLOCK();
      if (param_2 + 2 == plVar1) {
        return puVar4;
      }
      if (param_2 == (longlong *)0x0) {
        return puVar4;
      }
    }
    LOCK();
    *param_2 = *param_2 + -1;
    UNLOCK();
    if (*param_2 == 0) {
      plStack_88 = param_2;
      FUN_1418491f0(&plStack_88);
    }
  }
  return puVar4;
}

