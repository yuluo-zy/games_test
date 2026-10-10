
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_1408e3420(longlong *param_1,longlong param_2)

{
  code *pcVar1;
  undefined4 *puVar2;
  longlong *plVar3;
  undefined4 uVar4;
  float fVar5;
  undefined8 uStack_88;
  undefined4 *puStack_80;
  undefined8 uStack_78;
  longlong lStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  
  fVar5 = *(float *)(param_2 + 0x2c);
  puVar2 = (undefined4 *)func_0x000140613c10(0xa0,4);
  if (puVar2 == (undefined4 *)0x0) {
    FUN_1428d8fe3(4,0xa0,&UNK_142aa5200);
  }
  else {
    fVar5 = (_DAT_142925904 - fVar5) * _DAT_142925870 + _DAT_1429258d0 * fVar5;
    uVar4 = func_0x000142923f70(0,fVar5);
    *puVar2 = 0;
    puVar2[1] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa5094,fVar5);
    puVar2[2] = 0x3d579436;
    puVar2[3] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa5098,fVar5);
    puVar2[4] = 0x3dd79436;
    puVar2[5] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa509c,fVar5);
    puVar2[6] = 0x3e21af28;
    puVar2[7] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50a0,fVar5);
    puVar2[8] = 0x3e579436;
    puVar2[9] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50a4,fVar5);
    puVar2[10] = 0x3e86bca2;
    puVar2[0xb] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50a8,fVar5);
    puVar2[0xc] = 0x3ea1af28;
    puVar2[0xd] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50ac,fVar5);
    puVar2[0xe] = 0x3ebca1af;
    puVar2[0xf] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50b0,fVar5);
    puVar2[0x10] = 0x3ed79436;
    puVar2[0x11] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50b4,fVar5);
    puVar2[0x12] = 0x3ef286bd;
    puVar2[0x13] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50b8,fVar5);
    puVar2[0x14] = 0x3f06bca2;
    puVar2[0x15] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50bc,fVar5);
    puVar2[0x16] = 0x3f1435e5;
    puVar2[0x17] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50c0,fVar5);
    puVar2[0x18] = 0x3f21af28;
    puVar2[0x19] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50c4,fVar5);
    puVar2[0x1a] = 0x3f2f286c;
    puVar2[0x1b] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50c8,fVar5);
    puVar2[0x1c] = 0x3f3ca1af;
    puVar2[0x1d] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50cc,fVar5);
    puVar2[0x1e] = 0x3f4a1af3;
    puVar2[0x1f] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50d0,fVar5);
    puVar2[0x20] = 0x3f579436;
    puVar2[0x21] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50d4,fVar5);
    puVar2[0x22] = 0x3f650d79;
    puVar2[0x23] = uVar4;
    uVar4 = func_0x000142923f70(_DAT_142aa50d8,fVar5);
    puVar2[0x24] = 0x3f7286bd;
    puVar2[0x25] = uVar4;
    *(undefined8 *)(puVar2 + 0x26) = _DAT_14292e6b0;
    uStack_88 = 0x14;
    uStack_78 = 0x14;
    puStack_80 = puVar2;
    FUN_141469430(&lStack_70,&uStack_88,0);
    if (!SBORROW8(0,lStack_70)) {
      param_1[6] = lStack_40;
      param_1[4] = lStack_50;
      param_1[5] = lStack_48;
      param_1[2] = CONCAT44(uStack_5c,uStack_60);
      param_1[3] = lStack_58;
      *param_1 = lStack_70;
      param_1[1] = CONCAT44(uStack_64,uStack_68);
      return param_1;
    }
  }
  FUN_1428d9760(&UNK_142c63340,0x2b,&uStack_88,&UNK_142c63320,&UNK_142aa5908);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

