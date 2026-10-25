
char * asset__inheritable__deserialize_inheritable_option(char *param_1,undefined8 *param_2)
{
  char cVar1;
  int iVar2;
  undefined8 unaff_R14;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_e8;
  int iStack_e4;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined1 auStack_a0 [8];
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_80;
  int iStack_7c;
  uint uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined8 uStack_40;
  undefined2 uStack_38;
  undefined1 uStack_36;
  undefined2 uStack_34;
  char cStack_32;
  undefined8 uStack_30;
  uStack_30 = 0xfffffffffffffffe;
  ron__de__impl$2__deserialize_any(&uStack_80,*param_2);
  cVar1 = (char)uStack_80;
  if ((char)uStack_80 != '2') {
    unaff_R14 = CONCAT44(uStack_74,uStack_78);
    uStack_118 = CONCAT44(uStack_5c,uStack_60);
    uStack_128 = CONCAT44(uStack_6c,uStack_70);
    uStack_120 = CONCAT44(uStack_64,uStack_68);
    uStack_110 = CONCAT44(uStack_54,uStack_58);
    uStack_108 = CONCAT44(uStack_4c,uStack_50);
    uStack_100 = CONCAT44(uStack_44,uStack_48);
    iVar2 = iStack_7c;
    goto joined_r0x00014268afd3;
  }
  uStack_98 = uStack_70;
  uStack_94 = uStack_6c;
  uStack_90 = uStack_68;
  uStack_8c = uStack_64;
  iVar2 = 0;
  if ((char)uStack_78 != '\x10') {
    if ((uStack_78 & 0xff) == 0x11) {
      serde____private__de__content__impl$21__deserialize_tuple(&uStack_80,CONCAT44(uStack_6c,uStack_70),2);
    }
    else {
      iVar2 = 0;
      if ((uStack_78 & 0xff) == 0x12) goto LAB_14268b1c8;
      serde____private__de__content__impl$21__deserialize_tuple(&uStack_80,auStack_a0,2);
    }
    if ((char)uStack_80 == '2') {
      unaff_R14 = CONCAT44(uStack_78,iStack_7c);
      iVar2 = 1;
    }
    else {
      uStack_36 = uStack_80._3_1_;
      uStack_38 = uStack_80._1_2_;
      unaff_R14 = CONCAT44(uStack_74,uStack_78);
      iVar2 = iStack_7c;
      if ((char)uStack_80 != '2') {
        iStack_e4 = iStack_7c;
        uStack_e8 = uStack_80;
        uStack_d8 = uStack_70;
        uStack_d4 = uStack_6c;
        uStack_d0 = uStack_68;
        uStack_cc = uStack_64;
        uStack_c8 = uStack_60;
        uStack_c4 = uStack_5c;
        uStack_c0 = uStack_58;
        uStack_bc = uStack_54;
        uStack_b8 = uStack_50;
        uStack_b4 = uStack_4c;
        uStack_b0 = uStack_48;
        uStack_ac = uStack_44;
        uStack_a8 = uStack_40;
        core__ptr__drop_in_place(&uStack_e8);
        serde____private__de__content__impl$21__deserialize_tuple(&uStack_80,auStack_a0,2);
        if ((char)uStack_80 == '2') {
          unaff_R14 = CONCAT44(uStack_78,iStack_7c);
          iVar2 = 2;
        }
        else {
          uStack_e8 = uStack_80;
          iStack_e4 = iStack_7c;
          uStack_e0 = CONCAT44(uStack_74,uStack_78);
          uStack_a8 = uStack_40;
          uStack_b8 = uStack_50;
          uStack_b4 = uStack_4c;
          uStack_b0 = uStack_48;
          uStack_ac = uStack_44;
          uStack_c8 = uStack_60;
          uStack_c4 = uStack_5c;
          uStack_c0 = uStack_58;
          uStack_bc = uStack_54;
          uStack_d8 = uStack_70;
          uStack_d4 = uStack_6c;
          uStack_d0 = uStack_68;
          uStack_cc = uStack_64;
          unaff_R14 = uStack_e0;
          iVar2 = iStack_7c;
          if ((char)uStack_80 != '2') {
            core__ptr__drop_in_place(&uStack_e8);
            ron__error__impl$5__custom(&uStack_80,&UNK_143150438,0x3b);
            uStack_34 = uStack_80._1_2_;
            cStack_32 = uStack_80._3_1_;
            unaff_R14 = CONCAT44(uStack_74,uStack_78);
            uStack_128 = CONCAT44(uStack_6c,uStack_70);
            uStack_120 = CONCAT44(uStack_64,uStack_68);
            uStack_118 = CONCAT44(uStack_5c,uStack_60);
            uStack_110 = CONCAT44(uStack_54,uStack_58);
            uStack_108 = CONCAT44(uStack_4c,uStack_50);
            uStack_100 = CONCAT44(uStack_44,uStack_48);
            uStack_f8 = uStack_40;
            cVar1 = (char)uStack_80;
            iVar2 = iStack_7c;
          }
        }
      }
    }
  }
LAB_14268b1c8:
  core__ptr__drop_in_place(auStack_a0);
  uStack_80._3_1_ = cStack_32;
  uStack_80._1_2_ = uStack_34;
  uStack_40 = uStack_f8;
joined_r0x00014268afd3:
  if (cVar1 == '2') {
    if (iVar2 == 2) {
      param_1[4] = '\x01';
      param_1[5] = '\0';
      param_1[6] = '\0';
      param_1[7] = '\0';
    }
    else {
      *(int *)(param_1 + 4) = iVar2;
    }
    *(undefined8 *)(param_1 + 8) = unaff_R14;
    *param_1 = '2';
  }
  else {
    param_1[3] = uStack_80._3_1_;
    *(undefined2 *)(param_1 + 1) = uStack_80._1_2_;
    *(undefined8 *)(param_1 + 0x10) = uStack_128;
    *(undefined8 *)(param_1 + 0x18) = uStack_120;
    *(undefined8 *)(param_1 + 0x20) = uStack_118;
    *(undefined8 *)(param_1 + 0x28) = uStack_110;
    *(undefined8 *)(param_1 + 0x30) = uStack_108;
    *(undefined8 *)(param_1 + 0x38) = uStack_100;
    *(undefined8 *)(param_1 + 0x40) = uStack_40;
    *param_1 = cVar1;
    *(int *)(param_1 + 4) = iVar2;
    *(undefined8 *)(param_1 + 8) = unaff_R14;
  }
  return param_1;
}
