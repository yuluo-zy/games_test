
undefined8 * FUN_140df3e70(undefined8 *param_1,longlong param_2,undefined8 *param_3)

{
  byte *pbVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 *puVar5;
  longlong lVar6;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined2 uStack_34;
  undefined2 uStack_32;
  
                    /* WARNING: Switch is manually overridden */
  switch(*param_3) {
  case 0:
    uStack_60 = param_3[3];
    uStack_70 = *(undefined4 *)(param_3 + 1);
    uStack_6c = *(undefined4 *)((longlong)param_3 + 0xc);
    uStack_68 = *(undefined4 *)(param_3 + 2);
    uStack_64 = *(undefined4 *)((longlong)param_3 + 0x14);
    uStack_78 = 0;
    break;
  case 1:
    uStack_60 = param_3[3];
    uStack_70 = *(undefined4 *)(param_3 + 1);
    uStack_6c = *(undefined4 *)((longlong)param_3 + 0xc);
    uStack_68 = *(undefined4 *)(param_3 + 2);
    uStack_64 = *(undefined4 *)((longlong)param_3 + 0x14);
    uStack_78 = 1;
    break;
  case 2:
    uVar2 = *(undefined4 *)(param_2 + 0x30);
    FUN_141a50530(&uStack_78,param_2 + 0x18,&UNK_142b78de0);
    uStack_60 = CONCAT44(uStack_60._4_4_,uVar2);
    uStack_38 = *(undefined4 *)(param_2 + 0x60);
    uStack_34 = *(undefined2 *)(param_2 + 100);
    uStack_58 = (undefined4)uStack_78;
    uStack_54 = uStack_78._4_4_;
    uStack_50 = uStack_70;
    uStack_4c = uStack_6c;
    uStack_48 = CONCAT44(uStack_64,uStack_68);
    uStack_3c = uStack_60._4_4_;
    lVar3 = *(longlong *)(param_2 + 0x38);
    uStack_40 = uVar2;
    if (-0x7fffffffffffffff < lVar3) {
      lVar4 = *(longlong *)(param_2 + 0x40);
      lVar6 = *(longlong *)(param_2 + 0x48);
      if (lVar6 != 0) {
        puVar5 = (undefined8 *)(lVar4 + 0x10);
        do {
          if (puVar5[-1] != 0) {
            func_0x000140613c20(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 4;
          lVar6 = lVar6 + -1;
        } while (lVar6 != 0);
      }
      if (lVar3 != 0) {
        func_0x000140613c20(lVar4,lVar3 << 5,8);
      }
    }
    *(ulonglong *)(param_2 + 0x58) = CONCAT26(uStack_32,CONCAT24(uStack_34,uStack_38));
    *(undefined8 *)(param_2 + 0x48) = uStack_48;
    *(undefined4 *)(param_2 + 0x50) = uStack_40;
    *(undefined4 *)(param_2 + 0x54) = uStack_3c;
    *(ulonglong *)(param_2 + 0x38) = CONCAT44(uStack_54,uStack_58);
    *(ulonglong *)(param_2 + 0x40) = CONCAT44(uStack_4c,uStack_50);
    goto LAB_140df4001;
  case 3:
    *param_1 = 0x8000000000000001;
    param_1[1] = 0;
    return param_1;
  case 4:
    pbVar1 = (byte *)(param_2 + 0x60 + param_3[1]);
    *pbVar1 = *pbVar1 ^ 1;
    goto LAB_140df4001;
  }
  FUN_140df3d10(param_2,&uStack_78);
LAB_140df4001:
  *param_1 = 0x800000000000001e;
  return param_1;
}

