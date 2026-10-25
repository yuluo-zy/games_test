
undefined8 FUN_1403206a0(longlong *param_1,ulonglong param_2,undefined8 param_3,ulonglong param_4)

{
  char *pcVar1;
  char cVar2;
  byte bVar3;
  undefined1 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  code *pcVar8;
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  longlong lVar17;
  undefined8 uVar18;
  undefined4 *puVar19;
  longlong lVar20;
  longlong lVar21;
  short *psVar22;
  ulonglong uVar23;
  longlong lVar24;
  ulonglong uVar25;
  byte *pbVar26;
  bool bVar27;
  longlong lStackX_8;
  undefined1 auStackX_10 [8];
  undefined1 auStackX_18 [8];
  char cStackX_20;
  undefined1 auStackX_21 [7];
  undefined8 uStack_48;
  
  uStack_48 = 0x1403206b6;
  lVar17 = FUN_14289aa40();
  lVar17 = -lVar17;
  *(undefined8 *)(&stack0x000021e0 + lVar17) = 0xfffffffffffffffe;
  auVar9._8_8_ = 0;
  auVar9._0_8_ = (param_2 & 0xffffffff) << 2;
  auVar11._8_8_ = 0;
  auVar11._0_8_ = param_2 >> 0x20;
  if (param_4 < SUB168(auVar9 * auVar11,0) || SUB168(auVar9 * auVar11,8) != 0) {
    *(undefined **)(&stack0x00001070 + lVar17) = &UNK_1429bcb88;
    *(undefined8 *)(&stack0x00001078 + lVar17) = 1;
    *(undefined8 *)(&stack0x00001080 + lVar17) = 8;
    *(undefined8 *)(&stack0x00001088 + lVar17) = 0;
    *(undefined8 *)(&stack0x00001090 + lVar17) = 0;
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320734;
    uVar18 = FUN_1428a2020(&stack0x00001070 + lVar17);
LAB_140320737:
    pbVar26 = *(byte **)(&stack0x00002250 + lVar17);
LAB_14032073e:
    *(undefined8 *)(&stack0x000021c0 + lVar17) = uVar18;
    *(undefined1 **)(&stack0x000021a0 + lVar17) = &stack0x000021c0 + lVar17;
    bVar3 = *pbVar26;
  }
  else {
    *(undefined8 *)(&stack0x000020d8 + lVar17) = param_3;
    *(ulonglong *)(&stack0x000020e0 + lVar17) = param_4;
    *(ulonglong *)(&stack0x000020e8 + lVar17) = param_2;
    lVar24 = *param_1 + -0x7fffffffffffffff;
    lVar20 = 0;
    if (*param_1 < -0x7ffffffffffffffe) {
      lVar20 = lVar24;
    }
    if (lVar20 != 0) {
      if (lVar20 != 1) {
        (&stack0x000021df)[lVar17] = 0;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032125b;
        FUN_1428d9430(&UNK_1429bcb28,0x28,&UNK_1429bcb50);
        goto LAB_14032127e;
      }
      (&stack0x000021df)[lVar17] = 0;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320958;
      FUN_14014fff0(&stack0x00002158 + lVar17,&stack0x000020d8 + lVar17);
      *(undefined8 *)(&stack0x000021a0 + lVar17) = 0;
      *(undefined8 *)(&stack0x000021a8 + lVar17) = 1;
      *(undefined8 *)(&stack0x000021b0 + lVar17) = 0;
      lVar20 = param_1[2];
      (&stack0x000021dd)[lVar17] = 1;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320995;
      FUN_1402d8cd0((longlong)&lStackX_8 + lVar17,&stack0x000021a0 + lVar17,(char)lVar20);
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403209ae;
      func_0x000142923c00(&stack0x00001070 + lVar17,(longlong)&lStackX_8 + lVar17,0x1068);
      uVar7 = *(undefined8 *)(&stack0x00002160 + lVar17);
      uVar18 = *(undefined8 *)(&stack0x00002168 + lVar17);
      uVar5 = *(undefined4 *)(&stack0x00002170 + lVar17);
      uVar6 = *(undefined4 *)(&stack0x00002174 + lVar17);
      (&stack0x000021dd)[lVar17] = 1;
      *(undefined **)(auStackX_10 + lVar17 + -0x10) = &UNK_1429bcb10;
      *(undefined4 *)(&stack0xffffffffffffffe8 + lVar17) = uVar6;
      *(undefined4 *)(&stack0xffffffffffffffe0 + lVar17) = uVar5;
      (&stack0xfffffffffffffff0)[lVar17] = 0xf;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403209fa;
      FUN_1402d93e0(&stack0x00002108 + lVar17,&stack0x00001070 + lVar17,uVar7,uVar18);
      if (*(longlong *)(&stack0x00002108 + lVar17) == -0x7ffffffffffffff6) {
        *(undefined8 *)(&stack0x000021d0 + lVar17) = *(undefined8 *)(&stack0x000021a0 + lVar17);
        *(undefined8 *)(&stack0x000021c8 + lVar17) = *(undefined8 *)(&stack0x000021a8 + lVar17);
        uVar18 = *(undefined8 *)(&stack0x000021b0 + lVar17);
        lVar20 = param_1[1];
        pcVar1 = (char *)(lVar20 + 0x10);
        LOCK();
        bVar27 = *(char *)(lVar20 + 0x10) == '\0';
        if (bVar27) {
          *(char *)(lVar20 + 0x10) = '\x01';
        }
        UNLOCK();
        if (!bVar27) {
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403211d9;
          FUN_1428cd210(pcVar1);
        }
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320a57;
        FUN_140c81c70(pcVar1);
        lVar24 = *(longlong *)(lVar20 + 0x18);
        if ((!SBORROW8(0,lVar24)) && (lVar24 != 0)) {
          uVar7 = *(undefined8 *)(lVar20 + 0x20);
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320a74;
          func_0x000140613c20(uVar7,lVar24,1);
        }
        *(undefined8 *)(lVar20 + 0x18) = *(undefined8 *)(&stack0x000021d0 + lVar17);
        *(undefined8 *)(lVar20 + 0x20) = *(undefined8 *)(&stack0x000021c8 + lVar17);
        *(undefined8 *)(lVar20 + 0x28) = uVar18;
        (&stack0x000021dd)[lVar17] = 0;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320a9d;
        FUN_140c81e10();
        LOCK();
        cVar2 = *pcVar1;
        if (cVar2 == '\x01') {
          *pcVar1 = '\0';
        }
        UNLOCK();
        if (cVar2 != '\x01') {
          (&stack0x000021dd)[lVar17] = 0;
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403211ef;
          FUN_1428cdb90(pcVar1,0);
        }
        if (*(longlong *)(&stack0x00002158 + lVar17) != 0) {
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320ac9;
          func_0x000140613c20(*(undefined8 *)(&stack0x00002160 + lVar17),
                              *(longlong *)(&stack0x00002158 + lVar17),1);
        }
        pbVar26 = *(byte **)(&stack0x00002250 + lVar17);
        *(undefined8 *)(&stack0x000021c0 + lVar17) = 0;
        bVar3 = *pbVar26;
joined_r0x000140320f0f:
        if ((bVar3 & 1) == 0) {
          return 0;
        }
        lVar20 = *(longlong *)(pbVar26 + 8);
        lVar24 = *(longlong *)(pbVar26 + 0x10);
        *(longlong *)(auStackX_10 + lVar17) = lVar24;
        *(undefined8 *)((longlong)&lStackX_8 + lVar17) = 0;
        if (lVar20 == 0) {
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140321269;
          FUN_1428d9310(&UNK_1429bc780);
          goto LAB_14032127e;
        }
        *(undefined8 *)(&stack0x00001070 + lVar17) = 0;
        *(ulonglong *)(&stack0x00001078 + lVar17) = param_2;
        *(longlong *)(&stack0x000021c8 + lVar17) = lVar20;
        *(longlong *)(&stack0x000021d0 + lVar17) = lVar24;
        pcVar8 = *(code **)(lVar24 + 0x18);
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320b2b;
        (*pcVar8)(lVar20,&stack0x00001070 + lVar17);
        lVar20 = *(longlong *)(*(longlong *)(&stack0x000021d0 + lVar17) + 8);
        if (lVar20 != 0) {
          uVar18 = *(undefined8 *)(*(longlong *)(&stack0x000021d0 + lVar17) + 0x10);
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320b4b;
          func_0x000140613c20(*(undefined8 *)(&stack0x000021c8 + lVar17),lVar20,uVar18);
        }
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320b54;
        FUN_1405fdd60((longlong)&lStackX_8 + lVar17);
        goto LAB_1403208d5;
      }
      *(undefined8 *)(&stack0x000010a8 + lVar17) = *(undefined8 *)(&stack0x00002140 + lVar17);
      *(undefined4 *)(&stack0x00001098 + lVar17) = *(undefined4 *)(&stack0x00002130 + lVar17);
      *(undefined4 *)(&stack0x0000109c + lVar17) = *(undefined4 *)(&stack0x00002134 + lVar17);
      *(undefined4 *)(&stack0x000010a0 + lVar17) = *(undefined4 *)(&stack0x00002138 + lVar17);
      *(undefined4 *)(&stack0x000010a4 + lVar17) = *(undefined4 *)(&stack0x0000213c + lVar17);
      *(undefined4 *)(&stack0x00001088 + lVar17) = *(undefined4 *)(&stack0x00002120 + lVar17);
      *(undefined4 *)(&stack0x0000108c + lVar17) = *(undefined4 *)(&stack0x00002124 + lVar17);
      *(undefined4 *)(&stack0x00001090 + lVar17) = *(undefined4 *)(&stack0x00002128 + lVar17);
      *(undefined4 *)(&stack0x00001094 + lVar17) = *(undefined4 *)(&stack0x0000212c + lVar17);
      *(undefined4 *)(&stack0x00001078 + lVar17) = *(undefined4 *)(&stack0x00002110 + lVar17);
      *(undefined4 *)(&stack0x0000107c + lVar17) = *(undefined4 *)(&stack0x00002114 + lVar17);
      *(undefined4 *)(&stack0x00001080 + lVar17) = *(undefined4 *)(&stack0x00002118 + lVar17);
      *(undefined4 *)(&stack0x00001084 + lVar17) = *(undefined4 *)(&stack0x0000211c + lVar17);
      *(longlong *)(&stack0x00001070 + lVar17) = *(longlong *)(&stack0x00002108 + lVar17);
      (&stack0x000021dd)[lVar17] = 1;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320ce8;
      uVar18 = FUN_1428a3f50(&stack0x00001070 + lVar17);
      if (*(longlong *)(&stack0x000021a0 + lVar17) != 0) {
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320d09;
        func_0x000140613c20(*(undefined8 *)(&stack0x000021a8 + lVar17),
                            *(longlong *)(&stack0x000021a0 + lVar17),1);
      }
      pbVar26 = *(byte **)(&stack0x00002250 + lVar17);
      if (*(longlong *)(&stack0x00002158 + lVar17) != 0) {
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320d2e;
        func_0x000140613c20(uVar7,*(longlong *)(&stack0x00002158 + lVar17),1);
      }
      goto LAB_14032073e;
    }
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320b6e;
    uVar18 = FUN_140d76ed0(param_1);
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320b7d;
    lVar20 = FUN_140d61cd0(uVar18);
    if (lVar20 != 0) {
      (&stack0x00001070)[lVar17] = 1;
      (&stack0x000021df)[lVar17] = 0;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320ba2;
      lVar21 = FUN_140d5a150(&stack0x00001070 + lVar17,lVar20,lVar24);
      lVar24 = lVar20;
      if (lVar21 != 0) {
        (&stack0x000021df)[lVar17] = 0;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320bb6;
        uVar18 = FUN_1428a4000(lVar21);
        goto LAB_140320737;
      }
    }
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320bca;
    uVar18 = FUN_140d76ed0(param_1);
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320bd9;
    lVar20 = FUN_140d61cd0(uVar18);
    if (lVar20 != 0) {
      *(undefined8 *)(&stack0x00001098 + lVar17) = 6;
      *(undefined **)(&stack0x00001078 + lVar17) = &UNK_1429345a3;
      *(undefined8 *)(&stack0x00001080 + lVar17) = 4;
      *(undefined8 *)(&stack0x00001088 + lVar17) = 1;
      *(undefined8 *)(&stack0x00001090 + lVar17) = 0;
      *(undefined2 *)(&stack0x000010a0 + lVar17) = 0;
      *(undefined4 *)(&stack0x00001070 + lVar17) = 0;
      (&stack0x000021df)[lVar17] = 0;
      *(undefined1 **)(auStackX_10 + lVar17 + -0x10) = &stack0x00001070 + lVar17;
      *(undefined8 *)(&stack0xfffffffffffffff8 + lVar17) = 6;
      *(undefined8 *)(&stack0xfffffffffffffff0 + lVar17) = 0;
      *(undefined8 *)(&stack0xffffffffffffffe8 + lVar17) = 1;
      *(undefined8 *)(&stack0xffffffffffffffe0 + lVar17) = 4;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320c75;
      FUN_140312690((longlong)&lStackX_8 + lVar17,lVar20,lVar24);
      cVar2 = auStackX_10[lVar17 + 0x10];
      if (cVar2 == '\x02') {
        (&stack0x000021df)[lVar17] = 0;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320c91;
        uVar18 = FUN_1428a4000();
      }
      else {
        uVar5 = *(undefined4 *)(auStackX_10 + lVar17 + 4);
        uVar6 = *(undefined4 *)(auStackX_10 + lVar17 + 8);
        uVar13 = *(undefined4 *)(auStackX_10 + lVar17 + 0xc);
        *(undefined4 *)(&stack0x00002188 + lVar17) = *(undefined4 *)(auStackX_10 + lVar17);
        *(undefined4 *)(&stack0x0000218c + lVar17) = uVar5;
        *(undefined4 *)(&stack0x00002190 + lVar17) = uVar6;
        *(undefined4 *)(&stack0x00002194 + lVar17) = uVar13;
        *(undefined4 *)(&stack0x00002199 + lVar17) = *(undefined4 *)(auStackX_10 + lVar17 + 0x11);
        uVar23 = (ulonglong)*(uint *)(auStackX_10 + lVar17 + 0x14);
        *(uint *)(&stack0x0000219c + lVar17) = *(uint *)(auStackX_10 + lVar17 + 0x14);
        *(undefined8 *)(&stack0x00002180 + lVar17) = *(undefined8 *)((longlong)&lStackX_8 + lVar17);
        (&stack0x00002198)[lVar17] = cVar2;
        (&stack0x000021de)[lVar17] = 1;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320dcb;
        uVar18 = FUN_140d76ed0(param_1);
        (&stack0x000021de)[lVar17] = 1;
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320dda;
        psVar22 = (short *)FUN_140d627a0(uVar18);
        uVar25 = (ulonglong)(uVar23 == 3 && psVar22 != (short *)0x0);
        if ((uVar23 == 3 && psVar22 != (short *)0x0) &&
           ((char)psVar22[1] == 'g' && *psVar22 == 0x706a)) {
          (&stack0x000021de)[lVar17] = 1;
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032100a;
          FUN_14014fff0(&stack0x000021a0 + lVar17,&stack0x000020d8 + lVar17);
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140321020;
          FUN_1400f62c0(&stack0x00002158 + lVar17,&stack0x00002180 + lVar17);
          *(undefined1 **)(&stack0x00002150 + lVar17) = &stack0x00002158 + lVar17;
          lVar20 = param_1[4];
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032103c;
          FUN_1402d8cd0((longlong)&lStackX_8 + lVar17,&stack0x00002150 + lVar17,(char)lVar20);
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140321055;
          func_0x000142923c00(&stack0x00001070 + lVar17,(longlong)&lStackX_8 + lVar17,0x1068);
          uVar7 = *(undefined8 *)(&stack0x000021a8 + lVar17);
          uVar18 = *(undefined8 *)(&stack0x000021b0 + lVar17);
          uVar5 = *(undefined4 *)(&stack0x000021b8 + lVar17);
          uVar6 = *(undefined4 *)(&stack0x000021bc + lVar17);
          *(undefined **)(auStackX_10 + lVar17 + -0x10) = &UNK_1429bcaf8;
          *(undefined4 *)(&stack0xffffffffffffffe8 + lVar17) = uVar6;
          *(undefined4 *)(&stack0xffffffffffffffe0 + lVar17) = uVar5;
          (&stack0xfffffffffffffff0)[lVar17] = 0xf;
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032109a;
          FUN_1402dbe30(&stack0x00002108 + lVar17,&stack0x00001070 + lVar17,uVar7,uVar18);
          if (*(longlong *)(&stack0x00002108 + lVar17) == -0x7ffffffffffffff6) {
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403210bc;
            lVar20 = FUN_1400f6330(*(undefined8 *)(&stack0x00002150 + lVar17));
            if (lVar20 == 0) {
              *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403211a8;
              FUN_1405d3020(&stack0x00002158 + lVar17);
              if (*(longlong *)(&stack0x000021a0 + lVar17) != 0) {
                *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403211c6;
                func_0x000140613c20(uVar7,*(longlong *)(&stack0x000021a0 + lVar17),1);
              }
LAB_140320eae:
              *(undefined4 *)(&stack0x00001080 + lVar17) =
                   *(undefined4 *)(&stack0x00002190 + lVar17);
              *(undefined4 *)(&stack0x00001084 + lVar17) =
                   *(undefined4 *)(&stack0x00002194 + lVar17);
              *(undefined4 *)(&stack0x00001088 + lVar17) =
                   *(undefined4 *)(&stack0x00002198 + lVar17);
              *(undefined4 *)(&stack0x0000108c + lVar17) =
                   *(undefined4 *)(&stack0x0000219c + lVar17);
              *(undefined4 *)(&stack0x00001070 + lVar17) =
                   *(undefined4 *)(&stack0x00002180 + lVar17);
              *(undefined4 *)(&stack0x00001074 + lVar17) =
                   *(undefined4 *)(&stack0x00002184 + lVar17);
              *(undefined4 *)(&stack0x00001078 + lVar17) =
                   *(undefined4 *)(&stack0x00002188 + lVar17);
              *(undefined4 *)(&stack0x0000107c + lVar17) =
                   *(undefined4 *)(&stack0x0000218c + lVar17);
              (&stack0x000021de)[lVar17] = 0;
              *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320ee4;
              FUN_1400b2280((longlong)&lStackX_8 + lVar17,&stack0x00001070 + lVar17,param_1);
              if ((&stack0x00000028)[lVar17] == '\x02') {
                *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320efa;
                CloseHandle();
                pbVar26 = *(byte **)(&stack0x00002250 + lVar17);
                *(undefined8 *)(&stack0x000021c0 + lVar17) = 0;
                bVar3 = *pbVar26;
                goto joined_r0x000140320f0f;
              }
              *(undefined8 *)(&stack0x00001088 + lVar17) =
                   *(undefined8 *)(auStackX_10 + lVar17 + 0x10);
              uVar5 = *(undefined4 *)(auStackX_10 + lVar17 + 4);
              uVar6 = *(undefined4 *)(auStackX_10 + lVar17 + 8);
              uVar13 = *(undefined4 *)(auStackX_10 + lVar17 + 0xc);
              *(undefined4 *)(&stack0x00001078 + lVar17) = *(undefined4 *)(auStackX_10 + lVar17);
              *(undefined4 *)(&stack0x0000107c + lVar17) = uVar5;
              *(undefined4 *)(&stack0x00001080 + lVar17) = uVar6;
              *(undefined4 *)(&stack0x00001084 + lVar17) = uVar13;
              *(undefined4 *)(&stack0x00001091 + lVar17) =
                   *(undefined4 *)(&stack0x00000029 + lVar17);
              *(undefined4 *)(&stack0x00001094 + lVar17) =
                   *(undefined4 *)(&stack0x0000002c + lVar17);
              *(undefined8 *)(&stack0x00001070 + lVar17) =
                   *(undefined8 *)((longlong)&lStackX_8 + lVar17);
              (&stack0x00001090)[lVar17] = (&stack0x00000028)[lVar17];
              (&stack0x000021de)[lVar17] = 0;
              *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032111a;
              uVar18 = FUN_1428a4220(&stack0x00001070 + lVar17);
              goto LAB_140320737;
            }
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403210cd;
            uVar18 = FUN_1428a4000(lVar20);
          }
          else {
            *(undefined8 *)(&stack0x000010a8 + lVar17) = *(undefined8 *)(&stack0x00002140 + lVar17);
            *(undefined4 *)(&stack0x00001098 + lVar17) = *(undefined4 *)(&stack0x00002130 + lVar17);
            *(undefined4 *)(&stack0x0000109c + lVar17) = *(undefined4 *)(&stack0x00002134 + lVar17);
            *(undefined4 *)(&stack0x000010a0 + lVar17) = *(undefined4 *)(&stack0x00002138 + lVar17);
            *(undefined4 *)(&stack0x000010a4 + lVar17) = *(undefined4 *)(&stack0x0000213c + lVar17);
            *(undefined4 *)(&stack0x00001088 + lVar17) = *(undefined4 *)(&stack0x00002120 + lVar17);
            *(undefined4 *)(&stack0x0000108c + lVar17) = *(undefined4 *)(&stack0x00002124 + lVar17);
            *(undefined4 *)(&stack0x00001090 + lVar17) = *(undefined4 *)(&stack0x00002128 + lVar17);
            *(undefined4 *)(&stack0x00001094 + lVar17) = *(undefined4 *)(&stack0x0000212c + lVar17);
            *(undefined4 *)(&stack0x00001078 + lVar17) = *(undefined4 *)(&stack0x00002110 + lVar17);
            *(undefined4 *)(&stack0x0000107c + lVar17) = *(undefined4 *)(&stack0x00002114 + lVar17);
            *(undefined4 *)(&stack0x00001080 + lVar17) = *(undefined4 *)(&stack0x00002118 + lVar17);
            *(undefined4 *)(&stack0x00001084 + lVar17) = *(undefined4 *)(&stack0x0000211c + lVar17);
            *(longlong *)(&stack0x00001070 + lVar17) = *(longlong *)(&stack0x00002108 + lVar17);
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032116a;
            uVar18 = FUN_1428a3f50(&stack0x00001070 + lVar17);
          }
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140321179;
          FUN_1405d3020(&stack0x00002158 + lVar17);
          if (*(longlong *)(&stack0x000021a0 + lVar17) != 0) {
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140321197;
            func_0x000140613c20(uVar7,*(longlong *)(&stack0x000021a0 + lVar17),1);
          }
        }
        else {
          (&stack0x000021de)[lVar17] = 1;
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320e16;
          uVar18 = FUN_140d76ed0(param_1);
          (&stack0x000021de)[lVar17] = 1;
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320e2c;
          FUN_140ba48d0((longlong)&lStackX_8 + lVar17,uVar18,uVar25);
          uVar4 = auStackX_10[lVar17];
          if (*(longlong *)((longlong)&lStackX_8 + lVar17) == -0x7ffffffffffffff6) {
            auVar10._8_8_ = 0;
            auVar10._0_8_ = (param_2 & 0xffffffff) << 2;
            auVar12._8_8_ = 0;
            auVar12._0_8_ = param_2 >> 0x20;
            uVar23 = SUB168(auVar10 * auVar12,0);
            if (SUB168(auVar10 * auVar12,8) != 0) {
              (&stack0x000021de)[lVar17] = 1;
              *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032127e;
              FUN_1428d9310(&UNK_1429595f8);
              goto LAB_14032127e;
            }
            if (param_4 < uVar23) {
              (&stack0x000021de)[lVar17] = 1;
              *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032120d;
              FUN_1428d9840(uVar23,param_4,&UNK_142959610);
              goto LAB_14032127e;
            }
            (&stack0x000021de)[lVar17] = 1;
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320e72;
            uVar18 = func_0x00014067dd80(param_3,uVar23);
            (&stack0x000021de)[lVar17] = 1;
            auStackX_10[lVar17 + -0x10] = uVar4;
            *(int *)(&stack0xffffffffffffffe8 + lVar17) = (int)(param_2 >> 0x20);
            *(int *)(&stack0xffffffffffffffe0 + lVar17) = (int)param_2;
            (&stack0xfffffffffffffff0)[lVar17] = 0x10;
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320ea1;
            FUN_1405ceca0((longlong)&lStackX_8 + lVar17,&stack0x00002180 + lVar17,uVar18,uVar23);
            if (*(longlong *)((longlong)&lStackX_8 + lVar17) == -0x7ffffffffffffff6)
            goto LAB_140320eae;
            *(undefined8 *)(&stack0x000010a8 + lVar17) = *(undefined8 *)(&stack0x00000040 + lVar17);
            uVar5 = *(undefined4 *)(auStackX_10 + lVar17 + 4);
            uVar6 = *(undefined4 *)(auStackX_10 + lVar17 + 8);
            uVar13 = *(undefined4 *)(auStackX_10 + lVar17 + 0xc);
            uVar14 = *(undefined4 *)(auStackX_10 + lVar17 + 0x10);
            uVar15 = *(undefined4 *)(auStackX_10 + lVar17 + 0x14);
            *(undefined4 *)(&stack0x00001098 + lVar17) = *(undefined4 *)(&stack0x00000030 + lVar17);
            *(undefined4 *)(&stack0x0000109c + lVar17) = *(undefined4 *)(&stack0x00000034 + lVar17);
            *(undefined4 *)(&stack0x000010a0 + lVar17) = *(undefined4 *)(&stack0x00000038 + lVar17);
            *(undefined4 *)(&stack0x000010a4 + lVar17) = *(undefined4 *)(&stack0x0000003c + lVar17);
            *(undefined4 *)(&stack0x00001088 + lVar17) = uVar14;
            *(undefined4 *)(&stack0x0000108c + lVar17) = uVar15;
            *(undefined4 *)(&stack0x00001090 + lVar17) = *(undefined4 *)(&stack0x00000028 + lVar17);
            *(undefined4 *)(&stack0x00001094 + lVar17) = *(undefined4 *)(&stack0x0000002c + lVar17);
            *(undefined4 *)(&stack0x00001078 + lVar17) = *(undefined4 *)(auStackX_10 + lVar17);
            *(undefined4 *)(&stack0x0000107c + lVar17) = uVar5;
            *(undefined4 *)(&stack0x00001080 + lVar17) = uVar6;
            *(undefined4 *)(&stack0x00001084 + lVar17) = uVar13;
            *(longlong *)(&stack0x00001070 + lVar17) = *(longlong *)((longlong)&lStackX_8 + lVar17);
            (&stack0x000021de)[lVar17] = 1;
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320fb1;
            uVar18 = FUN_1428a3f50(&stack0x00001070 + lVar17);
          }
          else {
            *(undefined8 *)(&stack0x000010a8 + lVar17) = *(undefined8 *)(&stack0x00000040 + lVar17);
            uVar5 = *(undefined4 *)(auStackX_10 + lVar17 + 1);
            uVar6 = *(undefined4 *)(auStackX_10 + lVar17 + 5);
            uVar13 = *(undefined4 *)(auStackX_10 + lVar17 + 9);
            uVar14 = *(undefined4 *)(auStackX_10 + lVar17 + 0xd);
            uVar15 = *(undefined4 *)(auStackX_10 + lVar17 + 0x11);
            uVar16 = *(undefined4 *)(auStackX_10 + lVar17 + 0x15);
            *(undefined4 *)(&stack0x00001099 + lVar17) = *(undefined4 *)(&stack0x00000031 + lVar17);
            *(undefined4 *)(&stack0x0000109d + lVar17) = *(undefined4 *)(&stack0x00000035 + lVar17);
            *(undefined4 *)(&stack0x000010a1 + lVar17) = *(undefined4 *)(&stack0x00000039 + lVar17);
            *(undefined4 *)(&stack0x000010a5 + lVar17) = *(undefined4 *)(&stack0x0000003d + lVar17);
            *(undefined4 *)(&stack0x00001089 + lVar17) = uVar15;
            *(undefined4 *)(&stack0x0000108d + lVar17) = uVar16;
            *(undefined4 *)(&stack0x00001091 + lVar17) = *(undefined4 *)(&stack0x00000029 + lVar17);
            *(undefined4 *)(&stack0x00001095 + lVar17) = *(undefined4 *)(&stack0x0000002d + lVar17);
            *(undefined4 *)(&stack0x00001079 + lVar17) = uVar5;
            *(undefined4 *)(&stack0x0000107d + lVar17) = uVar6;
            *(undefined4 *)(&stack0x00001081 + lVar17) = uVar13;
            *(undefined4 *)(&stack0x00001085 + lVar17) = uVar14;
            *(longlong *)(&stack0x00001070 + lVar17) = *(longlong *)((longlong)&lStackX_8 + lVar17);
            (&stack0x00001078)[lVar17] = uVar4;
            (&stack0x000021de)[lVar17] = 1;
            *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320f69;
            uVar18 = FUN_1428a3f50(&stack0x00001070 + lVar17);
          }
        }
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320fc0;
        FUN_140b83ed0(&stack0x00002188 + lVar17);
        if (*(longlong *)(&stack0x00002190 + lVar17) != 0) {
          *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320fde;
          func_0x000140613c20(*(undefined8 *)(&stack0x00002188 + lVar17),
                              *(longlong *)(&stack0x00002190 + lVar17),1);
        }
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320feb;
        CloseHandle(*(undefined8 *)(&stack0x00002180 + lVar17));
      }
      goto LAB_140320737;
    }
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320d46;
    FUN_140d56700(&stack0x00001070 + lVar17);
    (&stack0x000021df)[lVar17] = 0;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320d65;
    uVar18 = FUN_1428a3890(&UNK_1429bcad8,0x1e,&stack0x00001070 + lVar17);
    *(undefined8 *)(&stack0x000021c0 + lVar17) = uVar18;
    pbVar26 = *(byte **)(&stack0x00002250 + lVar17);
    *(undefined1 **)(&stack0x000021a0 + lVar17) = &stack0x000021c0 + lVar17;
    bVar3 = *pbVar26;
  }
  if ((bVar3 & 1) == 0) {
    return uVar18;
  }
  lVar20 = *(longlong *)(pbVar26 + 8);
  lVar24 = *(longlong *)(pbVar26 + 0x10);
  *(longlong *)(&stack0x00002158 + lVar17) = lVar20;
  *(longlong *)(&stack0x00002160 + lVar17) = lVar24;
  *(undefined1 **)(&stack0x00002108 + lVar17) = &stack0x000021a0 + lVar17;
  *(undefined **)(&stack0x00002110 + lVar17) = &UNK_1405b7a50;
  *(undefined8 *)(&stack0x00001070 + lVar17) = 2;
  *(undefined8 *)(&stack0x00001080 + lVar17) = 2;
  *(undefined8 *)(&stack0x00001090 + lVar17) = 0;
  *(undefined8 *)(&stack0x00001098 + lVar17) = 0x400000020;
  (&stack0x000010a0)[lVar17] = 3;
  *(undefined **)((longlong)&lStackX_8 + lVar17) = &UNK_1429bca30;
  *(undefined8 *)(auStackX_10 + lVar17) = 1;
  *(undefined1 **)(auStackX_10 + lVar17 + 8) = &stack0x00002108 + lVar17;
  *(undefined8 *)(auStackX_10 + lVar17 + 0x10) = 1;
  *(undefined1 **)(&stack0x00000028 + lVar17) = &stack0x00001070 + lVar17;
  *(undefined8 *)(&stack0x00000030 + lVar17) = 1;
  (&stack0x000021dc)[lVar17] = 1;
  *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x140320817;
  FUN_140d79f90(&stack0x000020f0 + lVar17,(longlong)&lStackX_8 + lVar17);
  uVar18 = *(undefined8 *)(&stack0x000020f0 + lVar17);
  uVar7 = *(undefined8 *)(&stack0x000020f8 + lVar17);
  *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032083b;
  puVar19 = (undefined4 *)func_0x000140613c10(0x18,8);
  if (puVar19 == (undefined4 *)0x0) {
    *(undefined8 *)(&stack0x000021c8 + lVar17) = uVar7;
    *(undefined8 *)(&stack0x000021d0 + lVar17) = uVar18;
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032122c;
    FUN_1428d9000(8,0x18);
  }
  else {
    *(undefined8 *)(puVar19 + 4) = *(undefined8 *)(&stack0x00002100 + lVar17);
    uVar5 = *(undefined4 *)(&stack0x000020f4 + lVar17);
    uVar6 = *(undefined4 *)(&stack0x000020f8 + lVar17);
    uVar13 = *(undefined4 *)(&stack0x000020fc + lVar17);
    *puVar19 = *(undefined4 *)(&stack0x000020f0 + lVar17);
    puVar19[1] = uVar5;
    puVar19[2] = uVar6;
    puVar19[3] = uVar13;
    *(longlong *)(auStackX_10 + lVar17) = lVar24;
    *(undefined8 *)((longlong)&lStackX_8 + lVar17) = 0;
    *(longlong *)(&stack0x000021c8 + lVar17) = lVar20;
    *(undefined4 **)(&stack0x00002148 + lVar17) = puVar19;
    if (lVar20 != 0) {
      *(undefined4 **)(&stack0x00001070 + lVar17) = puVar19;
      *(undefined **)(&stack0x00001078 + lVar17) = &UNK_1429bc590;
      *(longlong *)(&stack0x000021d0 + lVar17) = lVar24;
      pcVar8 = *(code **)(lVar24 + 0x18);
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403208a5;
      (*pcVar8)(lVar20,&stack0x00001070 + lVar17);
      lVar20 = *(longlong *)(*(longlong *)(&stack0x000021d0 + lVar17) + 8);
      if (lVar20 != 0) {
        uVar18 = *(undefined8 *)(*(longlong *)(&stack0x000021d0 + lVar17) + 0x10);
        *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403208c5;
        func_0x000140613c20(*(undefined8 *)(&stack0x000021c8 + lVar17),lVar20,uVar18);
      }
      (&stack0x000021dc)[lVar17] = 0;
      *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x1403208d5;
      FUN_1405fdd60((longlong)&lStackX_8 + lVar17);
LAB_1403208d5:
      return *(undefined8 *)(&stack0x000021c0 + lVar17);
    }
    *(undefined8 *)((longlong)&uStack_48 + lVar17) = 0x14032123a;
    FUN_1428d9310(&UNK_1429bc780);
  }
LAB_14032127e:
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

