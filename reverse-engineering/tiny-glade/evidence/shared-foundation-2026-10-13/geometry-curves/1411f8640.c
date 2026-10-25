
longlong *
FUN_1411f8640(longlong *param_1,undefined8 param_2,undefined4 param_3,undefined4 param_4,
             undefined8 param_5)

{
  code *pcVar1;
  longlong lStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined1 uStack_19;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  uStack_38 = 0;
  uStack_30 = 4;
  uStack_28 = 0;
  uStack_19 = 1;
  FUN_1411f8130(param_2,param_3,param_4,&uStack_38);
  uStack_19 = 0;
  FUN_1411f9920(&lStack_80,&uStack_38,0);
  if (!SBORROW8(0,lStack_80)) {
    param_1[6] = lStack_50;
    param_1[4] = lStack_60;
    param_1[5] = lStack_58;
    param_1[2] = CONCAT44(uStack_6c,uStack_70);
    param_1[3] = lStack_68;
    *param_1 = lStack_80;
    param_1[1] = CONCAT44(uStack_74,uStack_78);
    return param_1;
  }
  uStack_48 = uStack_78;
  uStack_44 = uStack_74;
  uStack_40 = uStack_70;
  uStack_3c = uStack_6c;
  uStack_19 = 0;
  FUN_1428d9760(&UNK_142c18bc8,0x2b,&uStack_48,&UNK_142c18ba8,param_5);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

