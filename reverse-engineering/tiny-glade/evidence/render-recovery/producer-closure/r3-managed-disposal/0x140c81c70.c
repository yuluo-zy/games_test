
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c81c70(undefined8 param_1)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong unaff_GS_OFFSET;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uStack_98 = 0;
  lVar1 = *(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8);
  lVar2 = *(longlong *)(lVar1 + 0x2a0);
  if (lVar2 == 0) {
    puVar3 = (undefined8 *)
             FUN_14290be60(*(longlong *)
                            (*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) +
                           0x2a0,0);
  }
  else if ((int)lVar2 == 1) {
    puVar3 = (undefined8 *)(lVar1 + 0x2a8);
  }
  else {
    FUN_14192e020(&uStack_108);
    puVar3 = &uStack_98;
    FUN_140c81800(puVar3);
    uStack_38 = uStack_a8;
    uStack_30 = uStack_a0;
    uStack_48 = uStack_b8;
    uStack_40 = uStack_b0;
    uStack_58 = uStack_c8;
    uStack_50 = uStack_c0;
    uStack_68 = uStack_d8;
    uStack_60 = uStack_d0;
    uStack_78 = uStack_e8;
    uStack_70 = uStack_e0;
    uStack_88 = uStack_f8;
    uStack_80 = uStack_f0;
    uStack_98 = CONCAT44(uStack_104,uStack_108);
    uStack_90 = uStack_100;
    uStack_8c = uStack_fc;
  }
  lVar1 = puVar3[10];
  if (lVar1 == puVar3[8]) {
    FUN_140b81df0(puVar3 + 8,&UNK_142b1fc78);
  }
  *(undefined8 *)(puVar3[9] + lVar1 * 8) = param_1;
  puVar3[10] = lVar1 + 1;
  FUN_140c81800(&uStack_98);
  return;
}

