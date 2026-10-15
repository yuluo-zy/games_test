
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1404e8010(undefined8 param_1,ulonglong param_2,undefined8 param_3,undefined8 param_4)

{
  undefined1 *puVar1;
  undefined4 uVar2;
  byte bVar3;
  byte bVar4;
  undefined1 uVar5;
  undefined1 uVar6;
  undefined1 uVar7;
  undefined4 uVar8;
  longlong lVar9;
  undefined8 uVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  code *pcVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  char cVar21;
  undefined1 uVar22;
  char cVar23;
  undefined4 uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong lVar27;
  undefined8 uVar28;
  undefined8 uVar29;
  byte *pbVar30;
  undefined8 uVar31;
  ulonglong uVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  longlong *plVar35;
  byte bVar36;
  undefined8 *puVar37;
  longlong lVar38;
  int iVar39;
  ulonglong uVar40;
  undefined1 *puVar41;
  longlong lVar42;
  undefined1 *puVar43;
  undefined *puVar44;
  undefined4 uVar45;
  float *pfVar46;
  uint uVar47;
  undefined *puVar48;
  undefined *puVar49;
  bool bVar50;
  float fVar51;
  double dVar52;
  double dVar53;
  double dVar54;
  undefined1 auVar55 [16];
  double dVar57;
  undefined1 auVar56 [16];
  double dVar58;
  double dVar59;
  undefined1 auVar60 [16];
  undefined4 unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined4 unaff_XMM7_Da;
  undefined4 unaff_XMM7_Db;
  undefined4 unaff_XMM7_Dc;
  undefined4 unaff_XMM7_Dd;
  undefined4 unaff_XMM8_Da;
  undefined4 unaff_XMM8_Db;
  undefined4 unaff_XMM8_Dc;
  undefined4 unaff_XMM8_Dd;
  undefined4 unaff_XMM9_Da;
  undefined4 unaff_XMM9_Db;
  undefined4 unaff_XMM9_Dc;
  undefined4 unaff_XMM9_Dd;
  undefined4 unaff_XMM10_Da;
  undefined4 unaff_XMM10_Db;
  undefined4 unaff_XMM10_Dc;
  undefined4 unaff_XMM10_Dd;
  undefined4 unaff_XMM11_Da;
  undefined4 unaff_XMM11_Db;
  undefined4 unaff_XMM11_Dc;
  undefined4 unaff_XMM11_Dd;
  undefined *puStack_48;
  
  puStack_48 = &UNK_1404e8026;
  lVar25 = FUN_14289aa40();
  lVar25 = -lVar25;
  *(undefined4 *)(&stack0x000011b0 + lVar25) = unaff_XMM11_Da;
  *(undefined4 *)(&stack0x000011b4 + lVar25) = unaff_XMM11_Db;
  *(undefined4 *)(&stack0x000011b8 + lVar25) = unaff_XMM11_Dc;
  *(undefined4 *)(&stack0x000011bc + lVar25) = unaff_XMM11_Dd;
  *(undefined4 *)(&stack0x000011a0 + lVar25) = unaff_XMM10_Da;
  *(undefined4 *)(&stack0x000011a4 + lVar25) = unaff_XMM10_Db;
  *(undefined4 *)(&stack0x000011a8 + lVar25) = unaff_XMM10_Dc;
  *(undefined4 *)(&stack0x000011ac + lVar25) = unaff_XMM10_Dd;
  *(undefined4 *)(&stack0x00001190 + lVar25) = unaff_XMM9_Da;
  *(undefined4 *)(&stack0x00001194 + lVar25) = unaff_XMM9_Db;
  *(undefined4 *)(&stack0x00001198 + lVar25) = unaff_XMM9_Dc;
  *(undefined4 *)(&stack0x0000119c + lVar25) = unaff_XMM9_Dd;
  *(undefined4 *)(&stack0x00001180 + lVar25) = unaff_XMM8_Da;
  *(undefined4 *)(&stack0x00001184 + lVar25) = unaff_XMM8_Db;
  *(undefined4 *)(&stack0x00001188 + lVar25) = unaff_XMM8_Dc;
  *(undefined4 *)(&stack0x0000118c + lVar25) = unaff_XMM8_Dd;
  *(undefined4 *)(&stack0x00001170 + lVar25) = unaff_XMM7_Da;
  *(undefined4 *)(&stack0x00001174 + lVar25) = unaff_XMM7_Db;
  *(undefined4 *)(&stack0x00001178 + lVar25) = unaff_XMM7_Dc;
  *(undefined4 *)(&stack0x0000117c + lVar25) = unaff_XMM7_Dd;
  *(undefined4 *)(&stack0x00001160 + lVar25) = unaff_XMM6_Da;
  *(undefined4 *)(&stack0x00001164 + lVar25) = unaff_XMM6_Db;
  *(undefined4 *)(&stack0x00001168 + lVar25) = unaff_XMM6_Dc;
  *(undefined4 *)(&stack0x0000116c + lVar25) = unaff_XMM6_Dd;
  *(undefined8 *)(&stack0x00001158 + lVar25) = 0xfffffffffffffffe;
  *(undefined8 *)(&stack0x00001120 + lVar25) = param_4;
  *(undefined8 *)(&stack0x000010e8 + lVar25) = param_3;
  *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8083;
  uVar40 = param_2;
  cVar21 = func_0x000140c16a00();
  *(ulonglong *)(&stack0x00001118 + lVar25) = param_2;
  if (cVar21 == '\0') {
code_r0x0001404e80ea:
    uVar29 = 0;
code_r0x0001404e82b8:
    *(undefined8 *)(&stack0x000010c0 + lVar25) = uVar29;
    lVar26 = param_2 + 0x38;
    uVar40 = 0x2ed189581b92decf;
    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e82df;
    lVar27 = FUN_1400dd6f0(lVar26,0x2ed189581b92decf,0x15d3d99396065f38);
    if (((lVar27 != 0) && (uVar40 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar40 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar40 * 8), uVar40 != 0)) {
      lVar27 = ~uVar40 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar27) != 0) {
        plVar35 = (longlong *)**(undefined8 **)(*(longlong *)(param_2 + 0x1e0) + lVar27 + 0x10);
        LOCK();
        lVar27 = *plVar35;
        *plVar35 = *plVar35 + 1;
        UNLOCK();
        if (*plVar35 == 0 || SCARRY8(lVar27,1) != *plVar35 < 0) goto code_r0x0001404ea7cd;
        *(longlong **)(&stack0x000010e0 + lVar25) = plVar35;
        uVar40 = 0x3d7b56e787d47c40;
        *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e835e;
        lVar27 = FUN_1400dd6f0(lVar26,0x3d7b56e787d47c40,0x67b0b10a8d43bfc7);
        if (((lVar27 != 0) && (uVar40 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar40 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar40 * 8), uVar40 != 0)) {
          lVar27 = ~uVar40 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar27) != 0) {
            lVar27 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar27 + 0x10);
            bVar50 = *(int *)(lVar27 + 0x68) == 1;
            if (bVar50) {
              uVar29 = *(undefined8 *)(lVar27 + 0x74);
              *(undefined8 *)(&stack0x00000b40 + lVar25) = *(undefined8 *)(lVar27 + 0x6c);
              *(undefined8 *)(&stack0x00000b48 + lVar25) = uVar29;
            }
            *(undefined4 *)(&stack0x0000113c + lVar25) = *(undefined4 *)(lVar27 + 0x94);
            uVar22 = *(undefined1 *)(lVar27 + 0x7c);
            uVar5 = *(undefined1 *)(lVar27 + 0x7d);
            *(undefined8 *)(&stack0x000010f8 + lVar25) = *(undefined8 *)(lVar27 + 0x60);
            lVar38 = *(longlong *)(lVar27 + 0x38);
            if (SBORROW8(0,lVar38)) {
code_r0x0001404e8425:
              *(longlong *)(&stack0x00001140 + lVar25) = lVar38;
              unaff_XMM6_Da = *(undefined4 *)(lVar27 + 0x50);
              unaff_XMM6_Db = *(undefined4 *)(lVar27 + 0x54);
              unaff_XMM6_Dc = *(undefined4 *)(lVar27 + 0x58);
              unaff_XMM6_Dd = *(undefined4 *)(lVar27 + 0x5c);
              *(undefined8 *)(&stack0x00001130 + lVar25) =
                   *(undefined8 *)(&stack0x00000d68 + lVar25);
              *(undefined8 *)(&stack0x00000b30 + lVar25) =
                   *(undefined8 *)(&stack0x00000d70 + lVar25);
            }
            else {
              if (lVar38 != -0x7fffffffffffffff) {
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e841e;
                FUN_140d7bd90(&stack0x00000d60 + lVar25,lVar27 + 0x38);
                lVar38 = *(longlong *)(&stack0x00000d60 + lVar25);
                goto code_r0x0001404e8425;
              }
              *(undefined8 *)(&stack0x00001140 + lVar25) = 0x8000000000000001;
            }
            *(undefined8 *)(&stack0x000000d0 + lVar25) = *(undefined8 *)(&stack0x00000b40 + lVar25);
            *(undefined8 *)(&stack0x000000d8 + lVar25) = *(undefined8 *)(&stack0x00000b48 + lVar25);
            *(undefined4 *)(&stack0x00001108 + lVar25) = *(undefined4 *)(lVar27 + 0x80);
            uVar24 = *(undefined4 *)(lVar27 + 0x84);
            *(undefined4 *)(&stack0x00001110 + lVar25) = *(undefined4 *)(lVar27 + 0x88);
            uVar45 = *(undefined4 *)(lVar27 + 0x8c);
            uVar29 = *(undefined8 *)(lVar27 + 0xa8);
            (&stack0x00001100)[lVar25] = *(undefined1 *)(lVar27 + 0xc3);
            uVar31 = *(undefined8 *)(lVar27 + 0x98);
            uVar10 = *(undefined8 *)(lVar27 + 0xa0);
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e84b9;
            uVar28 = FUN_1402b8400(&stack0x00000350 + lVar25,lVar27,&UNK_1429eba40);
            *(undefined8 *)(&stack0x00000b38 + lVar25) = param_1;
            cVar21 = *(char *)(lVar27 + 0xac);
            *(longlong **)(&stack0x000010d0 + lVar25) = plVar35;
            *(longlong *)(&stack0x000010d8 + lVar25) = lVar26;
            if (cVar21 == '\x01') {
              uVar14 = *(undefined8 *)(lVar27 + 0xb5);
              *(undefined8 *)(&stack0x00000570 + lVar25) = *(undefined8 *)(lVar27 + 0xad);
              *(undefined8 *)(&stack0x00000578 + lVar25) = uVar14;
              *(int *)(&stack0x00001128 + lVar25) = (int)CONCAT71((int7)((ulonglong)uVar28 >> 8),1);
            }
            else {
              *(undefined4 *)(&stack0x00001128 + lVar25) = 0;
            }
            uVar2 = *(undefined4 *)(lVar27 + 0xbd);
            uVar6 = *(undefined1 *)(lVar27 + 0xc1);
            uVar7 = *(undefined1 *)(lVar27 + 0xc2);
            uVar8 = *(undefined4 *)(lVar27 + 0x30);
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e852c;
            FUN_1402b7890(&stack0x00000b40 + lVar25,lVar27 + 0x18,&UNK_1429ebaa8);
            *(undefined4 *)(&stack0x00000b58 + lVar25) = uVar8;
            uVar8 = *(undefined4 *)(lVar27 + 0x90);
            *(undefined4 *)(&stack0x00000df4 + lVar25) = *(undefined4 *)(&stack0x0000113c + lVar25);
            *(undefined8 *)(&stack0x00000d98 + lVar25) = *(undefined8 *)(&stack0x00001140 + lVar25);
            *(undefined8 *)(&stack0x00000da0 + lVar25) = *(undefined8 *)(&stack0x00001130 + lVar25);
            *(undefined8 *)(&stack0x00000da8 + lVar25) = *(undefined8 *)(&stack0x00000b30 + lVar25);
            *(undefined4 *)(&stack0x00000db0 + lVar25) = unaff_XMM6_Da;
            *(undefined4 *)(&stack0x00000db4 + lVar25) = unaff_XMM6_Db;
            *(undefined4 *)(&stack0x00000db8 + lVar25) = unaff_XMM6_Dc;
            *(undefined4 *)(&stack0x00000dbc + lVar25) = unaff_XMM6_Dd;
            *(undefined8 *)(&stack0x00000dc0 + lVar25) = *(undefined8 *)(&stack0x000010f8 + lVar25);
            *(uint *)(&stack0x00000dc8 + lVar25) = (uint)bVar50;
            *(undefined8 *)(&stack0x00000dcc + lVar25) = *(undefined8 *)(&stack0x000000d0 + lVar25);
            *(undefined8 *)(&stack0x00000dd4 + lVar25) = *(undefined8 *)(&stack0x000000d8 + lVar25);
            (&stack0x00000ddc)[lVar25] = uVar22;
            (&stack0x00000ddd)[lVar25] = uVar5;
            *(undefined4 *)(&stack0x00000de0 + lVar25) = *(undefined4 *)(&stack0x00001108 + lVar25);
            *(undefined4 *)(&stack0x00000de4 + lVar25) = uVar24;
            *(undefined4 *)(&stack0x00000de8 + lVar25) = *(undefined4 *)(&stack0x00001110 + lVar25);
            *(undefined4 *)(&stack0x00000dec + lVar25) = uVar45;
            *(int *)(&stack0x00000e08 + lVar25) = (int)uVar29;
            (&stack0x00000e23)[lVar25] = (&stack0x00001100)[lVar25];
            *(undefined8 *)(&stack0x00000df8 + lVar25) = uVar31;
            *(undefined8 *)(&stack0x00000e00 + lVar25) = uVar10;
            *(undefined8 *)(&stack0x00000d70 + lVar25) = *(undefined8 *)(&stack0x00000360 + lVar25);
            *(undefined8 *)(&stack0x00000d60 + lVar25) = *(undefined8 *)(&stack0x00000350 + lVar25);
            *(undefined8 *)(&stack0x00000d68 + lVar25) = *(undefined8 *)(&stack0x00000358 + lVar25);
            (&stack0x00000e0c)[lVar25] = (char)*(undefined4 *)(&stack0x00001128 + lVar25);
            *(undefined8 *)(&stack0x00000e0d + lVar25) = *(undefined8 *)(&stack0x00000570 + lVar25);
            *(undefined8 *)(&stack0x00000e15 + lVar25) = *(undefined8 *)(&stack0x00000578 + lVar25);
            *(undefined4 *)(&stack0x00000e1d + lVar25) = uVar2;
            (&stack0x00000e21)[lVar25] = uVar6;
            (&stack0x00000e22)[lVar25] = uVar7;
            *(undefined8 *)(&stack0x00000d78 + lVar25) = *(undefined8 *)(&stack0x00000b40 + lVar25);
            *(undefined8 *)(&stack0x00000d80 + lVar25) = *(undefined8 *)(&stack0x00000b48 + lVar25);
            *(undefined8 *)(&stack0x00000d88 + lVar25) = *(undefined8 *)(&stack0x00000b50 + lVar25);
            *(undefined8 *)(&stack0x00000d90 + lVar25) = *(undefined8 *)(&stack0x00000b58 + lVar25);
            *(undefined4 *)(&stack0x00000df0 + lVar25) = uVar8;
            uVar10 = *(undefined8 *)(lVar27 + 0xf8);
            uVar28 = *(undefined8 *)(lVar27 + 0x100);
            uVar24 = *(undefined4 *)(lVar27 + 0xf0);
            uVar22 = *(undefined1 *)(lVar27 + 0xf4);
            uVar29 = *(undefined8 *)(lVar27 + 0xe8);
            uVar31 = *(undefined8 *)(lVar27 + 0xe0);
            uVar45 = *(undefined4 *)(lVar27 + 0x11c);
            uVar14 = *(undefined8 *)(lVar27 + 0x108);
            uVar15 = *(undefined8 *)(lVar27 + 0x110);
            uVar2 = *(undefined4 *)(lVar27 + 0x118);
            uVar5 = *(undefined1 *)(lVar27 + 0x120);
            uVar6 = *(undefined1 *)(lVar27 + 0x121);
            if (SBORROW8(0,*(longlong *)(lVar27 + 200))) {
              *(undefined8 *)(&stack0x00000b40 + lVar25) = 0x8000000000000000;
            }
            else {
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e86e2;
              FUN_140d7bd90(&stack0x00000b40 + lVar25,lVar27 + 200);
            }
            puVar41 = &stack0x00000d60 + lVar25;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e86fe;
            func_0x000142923c00(&stack0x000007c8 + lVar25,puVar41,200);
            *(undefined8 *)(&stack0x000008c0 + lVar25) = uVar10;
            *(undefined8 *)(&stack0x000008c8 + lVar25) = uVar28;
            *(undefined8 *)(&stack0x000008b0 + lVar25) = uVar29;
            *(undefined4 *)(&stack0x000008b8 + lVar25) = uVar24;
            (&stack0x000008bc)[lVar25] = uVar22;
            *(undefined8 *)(&stack0x000008a8 + lVar25) = uVar31;
            *(undefined8 *)(&stack0x000008d0 + lVar25) = uVar14;
            *(undefined8 *)(&stack0x000008d8 + lVar25) = uVar15;
            *(undefined4 *)(&stack0x000008e0 + lVar25) = uVar2;
            *(undefined4 *)(&stack0x000008e4 + lVar25) = uVar45;
            (&stack0x000008e8)[lVar25] = uVar5;
            (&stack0x000008e9)[lVar25] = uVar6;
            *(undefined8 *)(&stack0x00000890 + lVar25) = *(undefined8 *)(&stack0x00000b40 + lVar25);
            *(undefined8 *)(&stack0x00000898 + lVar25) = *(undefined8 *)(&stack0x00000b48 + lVar25);
            *(undefined8 *)(&stack0x000008a0 + lVar25) = *(undefined8 *)(&stack0x00000b50 + lVar25);
            *puRam000000014362bfe0 = (&stack0x00000888)[lVar25];
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e877d;
            uVar22 = func_0x000140c16a00();
            (&stack0x000010f8)[lVar25] = uVar22;
            cVar21 = (&stack0x00001248)[lVar25];
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e879e;
            func_0x0001409ba340(&stack0x000000d0 + lVar25);
            (&stack0x00001157)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e87b8;
            FUN_1409ba540(*(undefined8 *)(&stack0x00001118 + lVar25),&stack0x000000d0 + lVar25);
            lVar26 = *(longlong *)(&stack0x00001120 + lVar25);
            (&stack0x00001157)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e87d2;
            FUN_1409ba4b0(&stack0x000000d0 + lVar25);
            lVar27 = *(longlong *)(&stack0x00001238 + lVar25);
            *(longlong *)(&stack0x00001140 + lVar25) =
                 *(longlong *)(&stack0x000010d0 + lVar25) + 0x10;
            if (lVar27 != 0) {
              (&stack0x00001157)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e87fc;
              uVar24 = func_0x000140c0e7e0();
              (&stack0x00001157)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8818;
              uVar29 = FUN_140c0ea90(*(undefined8 *)(&stack0x00001238 + lVar25),
                                     *(undefined8 *)(&stack0x00001140 + lVar25));
              *(undefined4 *)(&stack0x000010b4 + lVar25) = uVar24;
              *(undefined8 *)(&stack0x000010b8 + lVar25) = uVar29;
            }
            uVar47 = (uint)(lVar27 != 0);
            plVar35 = *(longlong **)(&stack0x00001230 + lVar25);
            *(uint *)(&stack0x000010b0 + lVar25) = uVar47;
            uVar29 = *(undefined8 *)(&stack0x00001118 + lVar25);
            if (cVar21 == '\0') {
              if (uVar47 == 0) {
                (&stack0x00001157)[lVar25] = 1;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea636;
                FUN_1428d9330(&UNK_1429ebbc8,0x28,&UNK_1429ebbf0);
                goto code_r0x0001404ea7cd;
              }
              uVar31 = *(undefined8 *)(&stack0x000010b8 + lVar25);
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8994;
              func_0x000142923c00(&stack0x00000b40 + lVar25,&stack0x000000d0 + lVar25,0x1f8);
              *(undefined8 *)(&stack0x00000570 + lVar25) = uVar31;
              *(undefined8 *)(&stack0x00000350 + lVar25) = uVar29;
              *(undefined8 *)(&stack0x00000358 + lVar25) =
                   *(undefined8 *)(&stack0x00001140 + lVar25);
              *(longlong **)(&stack0x00000360 + lVar25) = plVar35 + 0xf;
              *(undefined1 **)(&stack0x00000368 + lVar25) = &stack0x00000570 + lVar25;
              *(undefined1 **)(&stack0x00000370 + lVar25) = &stack0x00000b40 + lVar25;
              *(undefined1 **)(&stack0xffffffffffffffe0 + lVar25) = &stack0x00000350 + lVar25;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e89ef;
              FUN_1401647b0(&stack0x00000d60 + lVar25,lVar26 + 0xd0,
                            *(undefined8 *)(&stack0x00001140 + lVar25),plVar35 + 3);
              (&stack0x00001157)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8a02;
              FUN_140609f60(&stack0x00000b40 + lVar25);
              if (SBORROW8(0,*(longlong *)(&stack0x00000d60 + lVar25))) {
                (&stack0x00001157)[lVar25] = 0;
                *(undefined **)(&stack0xffffffffffffffe0 + lVar25) = &UNK_1429ebc28;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea66d;
                FUN_1428d9760(&UNK_1429ebc08,0x1b,&stack0x00001040 + lVar25,&UNK_1429eb860);
                goto code_r0x0001404ea7cd;
              }
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8a2a;
              func_0x000142923c00(&stack0x00000570 + lVar25,&stack0x00000d60 + lVar25,0x218);
              uVar24 = *(undefined4 *)(&stack0x00000f78 + lVar25);
              (&stack0x0000113c)[lVar25] = 1;
            }
            else {
              uVar31 = *(undefined8 *)(&stack0x00001240 + lVar25);
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e886e;
              func_0x000142923c00(puVar41,&stack0x000000d0 + lVar25,0x1f8);
              *(undefined8 *)(&stack0x00000570 + lVar25) =
                   *(undefined8 *)(&stack0x000010b0 + lVar25);
              *(undefined8 *)(&stack0x00000578 + lVar25) =
                   *(undefined8 *)(&stack0x000010b8 + lVar25);
              *(undefined8 *)(&stack0x00001040 + lVar25) =
                   *(undefined8 *)(&stack0x00000b38 + lVar25);
              *(undefined8 *)(&stack0x00000b40 + lVar25) = uVar29;
              *(undefined8 *)(&stack0x00000b48 + lVar25) =
                   *(undefined8 *)(&stack0x00001140 + lVar25);
              *(longlong **)(&stack0x00001108 + lVar25) = plVar35 + 0xf;
              *(longlong **)(&stack0x00000b50 + lVar25) = plVar35 + 0xf;
              *(undefined1 **)(&stack0x00000b58 + lVar25) = &stack0x00001040 + lVar25;
              *(undefined1 **)(&stack0x00000b60 + lVar25) = puVar41;
              *(undefined1 **)(&stack0x00000b68 + lVar25) = &stack0x000007c8 + lVar25;
              *(undefined1 **)(&stack0x00000b70 + lVar25) = &stack0x00000570 + lVar25;
              *(undefined8 *)(&stack0x00000b78 + lVar25) = uVar31;
              *(undefined1 **)(&stack0xffffffffffffffe0 + lVar25) = &stack0x00000b40 + lVar25;
              *(longlong *)(&stack0x00001100 + lVar25) = lVar26 + 0xd0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8914;
              FUN_140161cd0(&stack0x00000350 + lVar25,lVar26 + 0xd0);
              (&stack0x00001157)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8927;
              FUN_140609f60(&stack0x00000d60 + lVar25);
              (&stack0x0000113c)[lVar25] = !SBORROW8(0,*(longlong *)(&stack0x00000350 + lVar25));
              if (SBORROW8(0,*(longlong *)(&stack0x00000350 + lVar25))) {
                (&stack0x00001157)[lVar25] = 0;
                puVar41 = (undefined1 *)0xa4c02661e67b048b;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8a64;
                lVar38 = FUN_1400dd6f0(*(undefined8 *)(&stack0x000010d8 + lVar25),0xa4c02661e67b048b
                                       ,0xeae761e6aa2a67ec);
                lVar27 = *(longlong *)(&stack0x00001118 + lVar25);
                if (((lVar38 != 0) && (puVar41 < *(undefined1 **)(lVar27 + 0x218))) &&
                   (uVar40 = *(ulonglong *)(*(longlong *)(lVar27 + 0x210) + (longlong)puVar41 * 8),
                   uVar40 != 0)) {
                  lVar38 = ~uVar40 * 0x60;
                  if (*(longlong *)(*(longlong *)(lVar27 + 0x1e0) + 0x20 + lVar38) != 0) {
                    lVar38 = *(longlong *)(lVar27 + 0x1e0) + lVar38;
                    lVar42 = *(longlong *)(lVar38 + 0x10);
                    uVar24 = *(undefined4 *)(lVar27 + 0x5b0);
                    pfVar46 = (float *)(lVar42 + 0x18);
                    fVar51 = *(float *)(lVar42 + 0x18) + _UNK_1429eb6f8;
                    *(float *)(&stack0x000010f4 + lVar25) = fVar51;
                    if (*puRam000000014362c3a8 != 0) {
                      *(float **)(&stack0x00000570 + lVar25) = pfVar46;
                      *(code **)(&stack0x00000578 + lVar25) = FUN_140d8c910;
                      *(undefined1 **)(&stack0x00000580 + lVar25) = &stack0x000010f4 + lVar25;
                      *(code **)(&stack0x00000588 + lVar25) = FUN_140d8c910;
                      *(undefined8 *)(&stack0x00000b40 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000b48 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000b50 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000b58 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000b60 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000b68 + lVar25) = 0x20;
                      (&stack0x00000b70)[lVar25] = 3;
                      *(undefined8 *)(&stack0x00000b78 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000b80 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000b88 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000b90 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000b98 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000ba0 + lVar25) = 0x20;
                      (&stack0x00000ba8)[lVar25] = 3;
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8bbb;
                      puVar37 = (undefined8 *)func_0x00014067dd80(&UNK_1429ebcd0);
                      uVar29 = *puVar37;
                      uVar31 = puVar37[1];
                      uVar45 = *(undefined4 *)(puVar37 + 2);
                      *(undefined8 *)(&stack0x00000d90 + lVar25) = 1;
                      *(undefined **)(&stack0x00000d98 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000da0 + lVar25) = 0x17;
                      *(undefined **)(&stack0x00000db0 + lVar25) = &UNK_1429ebcb0;
                      *(undefined8 *)(&stack0x00000db8 + lVar25) = 2;
                      *(undefined1 **)(&stack0x00000dc0 + lVar25) = &stack0x00000570 + lVar25;
                      *(undefined8 *)(&stack0x00000dc8 + lVar25) = 2;
                      *(undefined1 **)(&stack0x00000dd0 + lVar25) = &stack0x00000b40 + lVar25;
                      *(undefined8 *)(&stack0x00000dd8 + lVar25) = 2;
                      *(undefined8 *)(&stack0x00000d60 + lVar25) = 0;
                      *(undefined **)(&stack0x00000d68 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000d70 + lVar25) = 0x17;
                      *(undefined8 *)(&stack0x00000d78 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000d80 + lVar25) = uVar29;
                      *(undefined8 *)(&stack0x00000d88 + lVar25) = uVar31;
                      *(undefined4 *)(&stack0x00000da8 + lVar25) = 1;
                      *(undefined4 *)(&stack0x00000dac + lVar25) = uVar45;
                      (&stack0x00001157)[lVar25] = 0;
                      puVar41 = &stack0x00000d60 + lVar25;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8c8b;
                      func_0x000140d4a330(&stack0x00001040 + lVar25);
                      fVar51 = *(float *)(&stack0x000010f4 + lVar25);
                    }
                    if (fVar51 <= 0.0) {
                      *(undefined1 **)(&stack0x00000b40 + lVar25) = &stack0x000010b0 + lVar25;
                      *(code **)(&stack0x00000b48 + lVar25) = FUN_1404e7f90;
                      *(undefined **)(&stack0x00000d60 + lVar25) = &UNK_1429ebea8;
                      *(undefined8 *)(&stack0x00000d68 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000d80 + lVar25) = 0;
                      *(undefined1 **)(&stack0x00000d70 + lVar25) = &stack0x00000b40 + lVar25;
                      *(undefined8 *)(&stack0x00000d78 + lVar25) = 1;
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea6e5;
                      FUN_1428d9390(&stack0x00000d60 + lVar25,&UNK_1429ebeb8);
                      goto code_r0x0001404ea7cd;
                    }
                    *(undefined4 *)(lVar38 + 0x5c) = uVar24;
                    *pfVar46 = fVar51;
                    (&stack0x00001157)[lVar25] = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8cb5;
                    FUN_140be76f0(plVar35 + 3);
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8cba;
                    pbVar30 = (byte *)FUN_140d4d740();
                    if ((*pbVar30 & 1) == 0) {
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8cdf;
                      lVar27 = FUN_140d64e00();
                      pbVar30[0] = 1;
                      pbVar30[1] = 0;
                      pbVar30[2] = 0;
                      pbVar30[3] = 0;
                      pbVar30[4] = 0;
                      pbVar30[5] = 0;
                      pbVar30[6] = 0;
                      pbVar30[7] = 0;
                      *(longlong *)(pbVar30 + 8) = lVar27;
                      *(undefined1 **)(pbVar30 + 0x10) = puVar41;
                    }
                    else {
                      lVar27 = *(longlong *)(pbVar30 + 8);
                      puVar41 = *(undefined1 **)(pbVar30 + 0x10);
                    }
                    *(longlong *)(pbVar30 + 8) = lVar27 + 1;
                    uVar28 = _UNK_1429ebaf8;
                    uVar10 = _UNK_1429ebaf0;
                    uVar31 = _UNK_1429ebae8;
                    uVar29 = _UNK_1429ebae0;
                    *(undefined8 *)(&stack0x00000b50 + lVar25) = _UNK_1429ebaf0;
                    *(undefined8 *)(&stack0x00000b58 + lVar25) = _UNK_1429ebaf8;
                    *(undefined8 *)(&stack0x00000b40 + lVar25) = _UNK_1429ebae0;
                    *(undefined8 *)(&stack0x00000b48 + lVar25) = _UNK_1429ebae8;
                    *(longlong *)(&stack0x00000b60 + lVar25) = lVar27;
                    *(undefined1 **)(&stack0x00000b68 + lVar25) = puVar41;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8d2b;
                    pbVar30 = (byte *)FUN_140d4d740();
                    if ((*pbVar30 & 1) == 0) {
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8d49;
                      lVar27 = FUN_140d64e00();
                      pbVar30[0] = 1;
                      pbVar30[1] = 0;
                      pbVar30[2] = 0;
                      pbVar30[3] = 0;
                      pbVar30[4] = 0;
                      pbVar30[5] = 0;
                      pbVar30[6] = 0;
                      pbVar30[7] = 0;
                      *(longlong *)(pbVar30 + 8) = lVar27;
                      *(undefined1 **)(pbVar30 + 0x10) = puVar41;
                      puVar43 = puVar41;
                    }
                    else {
                      lVar27 = *(longlong *)(pbVar30 + 8);
                      puVar43 = *(undefined1 **)(pbVar30 + 0x10);
                    }
                    *(longlong *)(pbVar30 + 8) = lVar27 + 1;
                    *(undefined8 *)(&stack0x00000d80 + lVar25) =
                         *(undefined8 *)(&stack0x00000b60 + lVar25);
                    *(undefined8 *)(&stack0x00000d88 + lVar25) =
                         *(undefined8 *)(&stack0x00000b68 + lVar25);
                    *(undefined8 *)(&stack0x00000b70 + lVar25) = uVar29;
                    *(undefined8 *)(&stack0x00000b78 + lVar25) = uVar31;
                    *(undefined8 *)(&stack0x00000b80 + lVar25) = uVar10;
                    *(undefined8 *)(&stack0x00000b88 + lVar25) = uVar28;
                    *(undefined8 *)(&stack0x00000b40 + lVar25) =
                         *(undefined8 *)(&stack0x00000b40 + lVar25);
                    *(undefined8 *)(&stack0x00000b48 + lVar25) =
                         *(undefined8 *)(&stack0x00000b48 + lVar25);
                    *(undefined8 *)(&stack0x00000b50 + lVar25) =
                         *(undefined8 *)(&stack0x00000b50 + lVar25);
                    *(undefined8 *)(&stack0x00000b58 + lVar25) =
                         *(undefined8 *)(&stack0x00000b58 + lVar25);
                    *(undefined8 *)(&stack0x00000b60 + lVar25) =
                         *(undefined8 *)(&stack0x00000b60 + lVar25);
                    *(undefined8 *)(&stack0x00000b68 + lVar25) =
                         *(undefined8 *)(&stack0x00000b68 + lVar25);
                    puVar37 = (undefined8 *)(lVar26 + 0x160);
                    if (*(longlong *)(lVar26 + 0x160) != 0) {
                      *(longlong *)(&stack0x00001128 + lVar25) = lVar27;
                      *(undefined1 **)(&stack0x00001110 + lVar25) = puVar43;
                      *(undefined8 **)(&stack0x00001130 + lVar25) = puVar37;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8df0;
                      FUN_14004ceb0(puVar37,lVar26 + 0x180,0x68,0x10);
                      lVar26 = *(longlong *)(&stack0x00001120 + lVar25);
                      lVar38 = *(longlong *)(lVar26 + 0x198);
                      puVar43 = *(undefined1 **)(&stack0x00001110 + lVar25);
                      lVar27 = *(longlong *)(&stack0x00001128 + lVar25);
                      puVar37 = *(undefined8 **)(&stack0x00001130 + lVar25);
                      puVar41 = (undefined1 *)0x0;
                      if ((lVar38 != 0) &&
                         (puVar41 = (undefined1 *)(lVar38 * 0x11 + 0x21),
                         puVar41 != (undefined1 *)0x0)) {
                        lVar42 = *(longlong *)(lVar26 + 400);
                        *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8e44;
                        func_0x000140613c20(lVar42 + lVar38 * -0x10 + -0x10,puVar41,0x10);
                      }
                    }
                    uVar14 = *(undefined8 *)(&stack0x00000b88 + lVar25);
                    puVar37[8] = *(undefined8 *)(&stack0x00000b80 + lVar25);
                    puVar37[9] = uVar14;
                    uVar14 = *(undefined8 *)(&stack0x00000b40 + lVar25);
                    uVar15 = *(undefined8 *)(&stack0x00000b48 + lVar25);
                    uVar19 = *(undefined8 *)(&stack0x00000b50 + lVar25);
                    uVar20 = *(undefined8 *)(&stack0x00000b58 + lVar25);
                    uVar16 = *(undefined8 *)(&stack0x00000b60 + lVar25);
                    uVar17 = *(undefined8 *)(&stack0x00000b68 + lVar25);
                    uVar18 = *(undefined8 *)(&stack0x00000b78 + lVar25);
                    puVar37[6] = *(undefined8 *)(&stack0x00000b70 + lVar25);
                    puVar37[7] = uVar18;
                    puVar37[4] = uVar16;
                    puVar37[5] = uVar17;
                    puVar37[2] = uVar19;
                    puVar37[3] = uVar20;
                    *puVar37 = uVar14;
                    puVar37[1] = uVar15;
                    *(longlong *)(lVar26 + 0x1b0) = lVar27;
                    *(undefined1 **)(lVar26 + 0x1b8) = puVar43;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8e93;
                    pbVar30 = (byte *)FUN_140d4d740();
                    if ((*pbVar30 & 1) == 0) {
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8eb8;
                      lVar27 = FUN_140d64e00();
                      pbVar30[0] = 1;
                      pbVar30[1] = 0;
                      pbVar30[2] = 0;
                      pbVar30[3] = 0;
                      pbVar30[4] = 0;
                      pbVar30[5] = 0;
                      pbVar30[6] = 0;
                      pbVar30[7] = 0;
                      *(longlong *)(pbVar30 + 8) = lVar27;
                      *(undefined1 **)(pbVar30 + 0x10) = puVar41;
                    }
                    else {
                      lVar27 = *(longlong *)(pbVar30 + 8);
                      puVar41 = *(undefined1 **)(pbVar30 + 0x10);
                    }
                    *(longlong *)(pbVar30 + 8) = lVar27 + 1;
                    *(undefined8 *)(&stack0x00000b50 + lVar25) = uVar10;
                    *(undefined8 *)(&stack0x00000b58 + lVar25) = uVar28;
                    *(undefined8 *)(&stack0x00000b40 + lVar25) = uVar29;
                    *(undefined8 *)(&stack0x00000b48 + lVar25) = uVar31;
                    *(longlong *)(&stack0x00000b60 + lVar25) = lVar27;
                    *(undefined1 **)(&stack0x00000b68 + lVar25) = puVar41;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8ef6;
                    pbVar30 = (byte *)FUN_140d4d740();
                    if ((*pbVar30 & 1) == 0) {
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8f14;
                      lVar27 = FUN_140d64e00();
                      pbVar30[0] = 1;
                      pbVar30[1] = 0;
                      pbVar30[2] = 0;
                      pbVar30[3] = 0;
                      pbVar30[4] = 0;
                      pbVar30[5] = 0;
                      pbVar30[6] = 0;
                      pbVar30[7] = 0;
                      *(longlong *)(pbVar30 + 8) = lVar27;
                      *(undefined1 **)(pbVar30 + 0x10) = puVar41;
                    }
                    else {
                      lVar27 = *(longlong *)(pbVar30 + 8);
                      puVar41 = *(undefined1 **)(pbVar30 + 0x10);
                    }
                    *(longlong *)(pbVar30 + 8) = lVar27 + 1;
                    *(undefined8 *)(&stack0x00000d80 + lVar25) =
                         *(undefined8 *)(&stack0x00000b60 + lVar25);
                    *(undefined8 *)(&stack0x00000d88 + lVar25) =
                         *(undefined8 *)(&stack0x00000b68 + lVar25);
                    *(undefined8 *)(&stack0x00000b70 + lVar25) = uVar29;
                    *(undefined8 *)(&stack0x00000b78 + lVar25) = uVar31;
                    *(undefined8 *)(&stack0x00000b80 + lVar25) = uVar10;
                    *(undefined8 *)(&stack0x00000b88 + lVar25) = uVar28;
                    *(undefined8 *)(&stack0x00000b40 + lVar25) =
                         *(undefined8 *)(&stack0x00000b40 + lVar25);
                    *(undefined8 *)(&stack0x00000b48 + lVar25) =
                         *(undefined8 *)(&stack0x00000b48 + lVar25);
                    *(undefined8 *)(&stack0x00000b50 + lVar25) =
                         *(undefined8 *)(&stack0x00000b50 + lVar25);
                    *(undefined8 *)(&stack0x00000b58 + lVar25) =
                         *(undefined8 *)(&stack0x00000b58 + lVar25);
                    *(undefined8 *)(&stack0x00000b60 + lVar25) =
                         *(undefined8 *)(&stack0x00000b60 + lVar25);
                    *(undefined8 *)(&stack0x00000b68 + lVar25) =
                         *(undefined8 *)(&stack0x00000b68 + lVar25);
                    puVar37 = (undefined8 *)(lVar26 + 0x250);
                    if (*(longlong *)(lVar26 + 0x250) != 0) {
                      *(longlong *)(&stack0x00001128 + lVar25) = lVar27;
                      *(undefined1 **)(&stack0x00001110 + lVar25) = puVar41;
                      *(undefined8 **)(&stack0x00001130 + lVar25) = puVar37;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8fbb;
                      FUN_14004ceb0(puVar37,lVar26 + 0x270,0x68,0x10);
                      lVar26 = *(longlong *)(&stack0x00001120 + lVar25);
                      lVar38 = *(longlong *)(lVar26 + 0x288);
                      puVar41 = *(undefined1 **)(&stack0x00001110 + lVar25);
                      lVar27 = *(longlong *)(&stack0x00001128 + lVar25);
                      puVar37 = *(undefined8 **)(&stack0x00001130 + lVar25);
                      if ((lVar38 != 0) && (lVar42 = lVar38 * 0x11 + 0x21, lVar42 != 0)) {
                        lVar9 = *(longlong *)(lVar26 + 0x280);
                        *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e900f;
                        func_0x000140613c20(lVar9 + lVar38 * -0x10 + -0x10,lVar42,0x10);
                      }
                    }
                    uVar29 = *(undefined8 *)(&stack0x00000b88 + lVar25);
                    puVar37[8] = *(undefined8 *)(&stack0x00000b80 + lVar25);
                    puVar37[9] = uVar29;
                    uVar29 = *(undefined8 *)(&stack0x00000b40 + lVar25);
                    uVar31 = *(undefined8 *)(&stack0x00000b48 + lVar25);
                    uVar15 = *(undefined8 *)(&stack0x00000b50 + lVar25);
                    uVar16 = *(undefined8 *)(&stack0x00000b58 + lVar25);
                    uVar10 = *(undefined8 *)(&stack0x00000b60 + lVar25);
                    uVar28 = *(undefined8 *)(&stack0x00000b68 + lVar25);
                    uVar14 = *(undefined8 *)(&stack0x00000b78 + lVar25);
                    puVar37[6] = *(undefined8 *)(&stack0x00000b70 + lVar25);
                    puVar37[7] = uVar14;
                    puVar37[4] = uVar10;
                    puVar37[5] = uVar28;
                    puVar37[2] = uVar15;
                    puVar37[3] = uVar16;
                    *puVar37 = uVar29;
                    puVar37[1] = uVar31;
                    *(longlong *)(lVar26 + 0x2a0) = lVar27;
                    *(undefined1 **)(lVar26 + 0x2a8) = puVar41;
                    (&stack0x00001157)[lVar25] = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e906c;
                    FUN_140bf8e60(*(undefined8 *)(&stack0x00001140 + lVar25));
                    (&stack0x00001157)[lVar25] = 0;
                    uVar40 = 0x3335386a5fbcee80;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9093;
                    lVar38 = FUN_1400dd6f0(*(undefined8 *)(&stack0x000010d8 + lVar25),
                                           0x3335386a5fbcee80,0x8f2a59f31a65ec60);
                    lVar27 = *(longlong *)(&stack0x00001118 + lVar25);
                    if (((lVar38 != 0) && (uVar40 < *(ulonglong *)(lVar27 + 0x218))) &&
                       (uVar40 = *(ulonglong *)(*(longlong *)(lVar27 + 0x210) + uVar40 * 8),
                       uVar40 != 0)) {
                      lVar38 = ~uVar40 * 0x60;
                      if (*(longlong *)(*(longlong *)(lVar27 + 0x1e0) + 0x20 + lVar38) != 0) {
                        lVar38 = *(longlong *)(lVar27 + 0x1e0) + lVar38;
                        *(undefined4 *)(lVar38 + 0x5c) = *(undefined4 *)(lVar27 + 0x5b0);
                        **(undefined8 **)(lVar38 + 0x10) = 2;
                      }
                    }
                    if (2 < *puRam000000014362c3a8) {
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9111;
                      puVar37 = (undefined8 *)func_0x00014067dd80(&UNK_1429ebd38);
                      uVar29 = *puVar37;
                      uVar31 = puVar37[1];
                      uVar24 = *(undefined4 *)(puVar37 + 2);
                      *(undefined8 *)(&stack0x00000d90 + lVar25) = 3;
                      *(undefined **)(&stack0x00000d98 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000da0 + lVar25) = 0x17;
                      *(undefined **)(&stack0x00000db0 + lVar25) = &UNK_1429ebd28;
                      *(undefined8 *)(&stack0x00000db8 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000dc0 + lVar25) = 8;
                      *(undefined8 *)(&stack0x00000d60 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000dc8 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000dd0 + lVar25) = 0;
                      *(undefined **)(&stack0x00000d68 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000d70 + lVar25) = 0x17;
                      *(undefined8 *)(&stack0x00000d78 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000d80 + lVar25) = uVar29;
                      *(undefined8 *)(&stack0x00000d88 + lVar25) = uVar31;
                      *(undefined4 *)(&stack0x00000da8 + lVar25) = 1;
                      *(undefined4 *)(&stack0x00000dac + lVar25) = uVar24;
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e91c4;
                      func_0x000140d4a330(&stack0x00001040 + lVar25,&stack0x00000d60 + lVar25);
                      lVar27 = *(longlong *)(&stack0x00001118 + lVar25);
                    }
                    *(undefined4 *)(&stack0x00001040 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b40 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b48 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000b50 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b58 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b60 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000b68 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b70 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b78 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000b80 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b88 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000b90 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000b98 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000ba0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000ba8 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000bb0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000bb8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000bc0 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000bc8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000bd0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000bd8 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000be0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000be8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000bf0 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000bf8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c00 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c08 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c10 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c18 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c20 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c28 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c30 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c38 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c40 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c48 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c50 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c58 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c60 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c68 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c70 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c78 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c80 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000c88 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c90 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000c98 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000ca0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000ca8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cb0 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000cb8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cc0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cc8 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000cd0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cd8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000ce0 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000ce8 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cf0 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000cf8 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000d00 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000d08 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000d10 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000d18 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000d20 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000d28 + lVar25) = 0x10;
                    *(undefined8 *)(&stack0x00000d30 + lVar25) = 0;
                    *(undefined8 *)(&stack0x00000a00 + lVar25) = 0x2000000020;
                    *(longlong *)(&stack0x00000570 + lVar25) = lVar27;
                    *(undefined8 *)(&stack0x00000578 + lVar25) =
                         *(undefined8 *)(&stack0x00001140 + lVar25);
                    *(undefined8 *)(&stack0x00000580 + lVar25) =
                         *(undefined8 *)(&stack0x00001108 + lVar25);
                    *(undefined1 **)(&stack0x00000588 + lVar25) = &stack0x00000a00 + lVar25;
                    *(undefined1 **)(&stack0x00000590 + lVar25) = &stack0x00000b40 + lVar25;
                    *(undefined1 **)(&stack0x00000598 + lVar25) = &stack0x000007c8 + lVar25;
                    *(undefined1 **)(&stack0x000005a0 + lVar25) = &stack0x00001040 + lVar25;
                    *(undefined8 *)(&stack0x000005a8 + lVar25) = 0;
                    *(undefined1 **)(&stack0xffffffffffffffe0 + lVar25) = &stack0x00000570 + lVar25;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e940f;
                    FUN_140161cd0(&stack0x00000d60 + lVar25,
                                  *(undefined8 *)(&stack0x00001100 + lVar25),
                                  *(undefined8 *)(&stack0x00001140 + lVar25),plVar35 + 3);
                    (&stack0x00001157)[lVar25] = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9422;
                    FUN_140609f60(&stack0x00000b40 + lVar25);
                    if (SBORROW8(0,*(longlong *)(&stack0x00000d60 + lVar25))) {
                      (&stack0x00001157)[lVar25] = 0;
                      *(undefined **)(&stack0xffffffffffffffe0 + lVar25) = &UNK_1429ebd88;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea71c;
                      FUN_1428d9760(&UNK_1429ebd50,0x37,&stack0x00001040 + lVar25,&UNK_1429eb860);
                      goto code_r0x0001404ea7cd;
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e944a;
                    func_0x000142923c00(&stack0x00000b40 + lVar25,&stack0x00000d60 + lVar25,0x220);
                    if (2 < *puRam000000014362c3a8) {
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e946a;
                      puVar37 = (undefined8 *)func_0x00014067dd80(&UNK_1429ebde0);
                      uVar29 = *puVar37;
                      uVar31 = puVar37[1];
                      uVar24 = *(undefined4 *)(puVar37 + 2);
                      *(undefined8 *)(&stack0x00000d90 + lVar25) = 3;
                      *(undefined **)(&stack0x00000d98 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000da0 + lVar25) = 0x17;
                      *(undefined **)(&stack0x00000db0 + lVar25) = &UNK_1429ebdd0;
                      *(undefined8 *)(&stack0x00000db8 + lVar25) = 1;
                      *(undefined8 *)(&stack0x00000dc0 + lVar25) = 8;
                      *(undefined8 *)(&stack0x00000d60 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000dc8 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000dd0 + lVar25) = 0;
                      *(undefined **)(&stack0x00000d68 + lVar25) = &UNK_1429ebb70;
                      *(undefined8 *)(&stack0x00000d70 + lVar25) = 0x17;
                      *(undefined8 *)(&stack0x00000d78 + lVar25) = 0;
                      *(undefined8 *)(&stack0x00000d80 + lVar25) = uVar29;
                      *(undefined8 *)(&stack0x00000d88 + lVar25) = uVar31;
                      *(undefined4 *)(&stack0x00000da8 + lVar25) = 1;
                      *(undefined4 *)(&stack0x00000dac + lVar25) = uVar24;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9516;
                      func_0x000140d4a330(&stack0x00001040 + lVar25,&stack0x00000d60 + lVar25);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e952f;
                    func_0x000142923c00(&stack0x00000570 + lVar25,&stack0x00000b40 + lVar25,0x218);
                    uVar24 = *(undefined4 *)(&stack0x00000d58 + lVar25);
                    goto code_r0x0001404e9536;
                  }
                }
                (&stack0x00001157)[lVar25] = 0;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea5f3;
                FUN_1428d9310(&UNK_1429ebc40);
                goto code_r0x0001404ea7cd;
              }
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8956;
              func_0x000142923c00(&stack0x00000570 + lVar25,&stack0x00000350 + lVar25,0x218);
              uVar24 = *(undefined4 *)(&stack0x00000568 + lVar25);
            }
code_r0x0001404e9536:
            puVar41 = &stack0x00000570 + lVar25;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e954f;
            func_0x000142923c00(&stack0x00000b40 + lVar25,puVar41,0x218);
            uVar45 = SUB84(puVar41,0);
            if (iRam000000014362b0e8 == 1000000000) {
              uVar29 = 0;
              uVar45 = 0;
              bVar36 = (&stack0x000010f8)[lVar25];
            }
            else {
              (&stack0x00001150)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9584;
              uVar29 = FUN_140d635e0();
              (&stack0x00001150)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9599;
              uVar29 = FUN_140d63b00(uVar29);
              bVar36 = (&stack0x000010f8)[lVar25];
            }
            cVar23 = '\x02';
            if ((cVar21 == '\x02' & bVar36) != 0) {
              if (plVar35[2] == 0) {
code_r0x0001404e95ce:
                (&stack0x00001150)[lVar25] = 1;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e95e8;
                FUN_140bff4a0(&stack0x00000d60 + lVar25,*(undefined8 *)(&stack0x00001140 + lVar25));
                cVar23 = (&stack0x00000dac)[lVar25];
              }
              else {
                lVar27 = plVar35[2] + -1;
                plVar35[2] = lVar27;
                lVar27 = lVar27 * 0x58;
                cVar23 = *(char *)(plVar35[1] + 0x4c + lVar27);
                if (cVar23 == '\x02') goto code_r0x0001404e95ce;
                puVar37 = (undefined8 *)(plVar35[1] + lVar27);
                uVar31 = *(undefined8 *)((longlong)puVar37 + 0x44);
                *(undefined8 *)(&stack0x00000d9c + lVar25) =
                     *(undefined8 *)((longlong)puVar37 + 0x3c);
                *(undefined8 *)(&stack0x00000da4 + lVar25) = uVar31;
                uVar31 = *puVar37;
                uVar10 = puVar37[1];
                uVar28 = puVar37[2];
                uVar14 = puVar37[3];
                uVar15 = puVar37[4];
                uVar16 = puVar37[5];
                uVar17 = puVar37[7];
                *(undefined8 *)(&stack0x00000d90 + lVar25) = puVar37[6];
                *(undefined8 *)(&stack0x00000d98 + lVar25) = uVar17;
                *(undefined8 *)(&stack0x00000d80 + lVar25) = uVar15;
                *(undefined8 *)(&stack0x00000d88 + lVar25) = uVar16;
                *(undefined8 *)(&stack0x00000d70 + lVar25) = uVar28;
                *(undefined8 *)(&stack0x00000d78 + lVar25) = uVar14;
                *(undefined8 *)(&stack0x00000d60 + lVar25) = uVar31;
                *(undefined8 *)(&stack0x00000d68 + lVar25) = uVar10;
                (&stack0x00000dac)[lVar25] = cVar23;
                *(undefined8 *)(&stack0x00000dad + lVar25) =
                     *(undefined8 *)((longlong)puVar37 + 0x4d);
                *(undefined4 *)(&stack0x00000db4 + lVar25) =
                     *(undefined4 *)((longlong)puVar37 + 0x54);
              }
              *(undefined8 *)(&stack0x0000095c + lVar25) =
                   *(undefined8 *)(&stack0x00000d9c + lVar25);
              *(undefined8 *)(&stack0x00000964 + lVar25) =
                   *(undefined8 *)(&stack0x00000da4 + lVar25);
              *(undefined8 *)(&stack0x00000950 + lVar25) =
                   *(undefined8 *)(&stack0x00000d90 + lVar25);
              *(undefined8 *)(&stack0x00000958 + lVar25) =
                   *(undefined8 *)(&stack0x00000d98 + lVar25);
              *(undefined8 *)(&stack0x00000940 + lVar25) =
                   *(undefined8 *)(&stack0x00000d80 + lVar25);
              *(undefined8 *)(&stack0x00000948 + lVar25) =
                   *(undefined8 *)(&stack0x00000d88 + lVar25);
              *(undefined8 *)(&stack0x00000930 + lVar25) =
                   *(undefined8 *)(&stack0x00000d70 + lVar25);
              *(undefined8 *)(&stack0x00000938 + lVar25) =
                   *(undefined8 *)(&stack0x00000d78 + lVar25);
              *(undefined8 *)(&stack0x00000920 + lVar25) =
                   *(undefined8 *)(&stack0x00000d60 + lVar25);
              *(undefined8 *)(&stack0x00000928 + lVar25) =
                   *(undefined8 *)(&stack0x00000d68 + lVar25);
              *(undefined8 *)(&stack0x00001030 + lVar25) =
                   *(undefined8 *)(&stack0x00000dad + lVar25);
              *(undefined4 *)(&stack0x00001037 + lVar25) =
                   *(undefined4 *)(&stack0x00000db4 + lVar25);
            }
            *(undefined8 *)(&stack0x0000038c + lVar25) = *(undefined8 *)(&stack0x0000095c + lVar25);
            *(undefined8 *)(&stack0x00000394 + lVar25) = *(undefined8 *)(&stack0x00000964 + lVar25);
            *(undefined8 *)(&stack0x00000380 + lVar25) = *(undefined8 *)(&stack0x00000950 + lVar25);
            *(undefined8 *)(&stack0x00000388 + lVar25) = *(undefined8 *)(&stack0x00000958 + lVar25);
            *(undefined8 *)(&stack0x00000370 + lVar25) = *(undefined8 *)(&stack0x00000940 + lVar25);
            *(undefined8 *)(&stack0x00000378 + lVar25) = *(undefined8 *)(&stack0x00000948 + lVar25);
            *(undefined8 *)(&stack0x00000360 + lVar25) = *(undefined8 *)(&stack0x00000930 + lVar25);
            *(undefined8 *)(&stack0x00000368 + lVar25) = *(undefined8 *)(&stack0x00000938 + lVar25);
            *(undefined8 *)(&stack0x00000350 + lVar25) = *(undefined8 *)(&stack0x00000920 + lVar25);
            *(undefined8 *)(&stack0x00000358 + lVar25) = *(undefined8 *)(&stack0x00000928 + lVar25);
            (&stack0x0000039c)[lVar25] = cVar23;
            *(undefined8 *)(&stack0x0000039d + lVar25) = *(undefined8 *)(&stack0x00001030 + lVar25);
            *(undefined4 *)(&stack0x000003a4 + lVar25) = *(undefined4 *)(&stack0x00001037 + lVar25);
            (&stack0x00001150)[lVar25] = 1;
            *(undefined1 **)(&stack0xfffffffffffffff0 + lVar25) = &stack0x00000350 + lVar25;
            (&stack0xffffffffffffffe8)[lVar25] = (&stack0x00000888)[lVar25];
            (&stack0xffffffffffffffe0)[lVar25] = (&stack0x00000888)[lVar25];
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e975a;
            FUN_140beabd0(&stack0x00000d60 + lVar25,lVar26,
                          *(undefined8 *)(&stack0x00001140 + lVar25),plVar35 + 3);
            *(undefined4 *)(&stack0x000010f8 + lVar25) = uVar24;
            *(undefined8 *)(&stack0x00001108 + lVar25) = uVar29;
            *(undefined4 *)(&stack0x00001100 + lVar25) = uVar45;
            puVar41 = &stack0x00000d60 + lVar25;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e978a;
            func_0x000142923c00(&stack0x00000350 + lVar25,puVar41,400);
            puVar43 = &stack0x00000ef0 + lVar25;
            puVar1 = &stack0x00000578 + lVar25;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e97a6;
            func_0x000142923c00(puVar1,puVar43,0x108);
            *(undefined8 *)(&stack0x00000570 + lVar25) = 0;
            uVar40 = 0;
            while (uVar40 != *(uint *)(&stack0x00000678 + lVar25)) {
              *(ulonglong *)(&stack0x00000570 + lVar25) = uVar40 + 1;
              lVar26 = uVar40 * 0x80;
              puVar43 = *(undefined1 **)(puVar1 + lVar26);
              uVar40 = uVar40 + 1;
              if (!SBORROW8(0,(longlong)puVar43)) {
                if (puVar43 == (undefined1 *)0x8000000000000001) break;
                *(undefined1 **)(&stack0x000002d0 + lVar25) = puVar43;
                *(undefined8 *)(&stack0x00000348 + lVar25) = *(undefined8 *)(puVar1 + lVar26 + 0x78)
                ;
                uVar29 = *(undefined8 *)(puVar1 + lVar26 + 0x70);
                *(undefined8 *)(&stack0x00000338 + lVar25) = *(undefined8 *)(puVar1 + lVar26 + 0x68)
                ;
                *(undefined8 *)(&stack0x00000340 + lVar25) = uVar29;
                uVar29 = *(undefined8 *)(puVar1 + lVar26 + 0x60);
                *(undefined8 *)(&stack0x00000328 + lVar25) = *(undefined8 *)(puVar1 + lVar26 + 0x58)
                ;
                *(undefined8 *)(&stack0x00000330 + lVar25) = uVar29;
                uVar29 = *(undefined8 *)(puVar1 + lVar26 + 0x50);
                *(undefined8 *)(&stack0x00000318 + lVar25) = *(undefined8 *)(puVar1 + lVar26 + 0x48)
                ;
                *(undefined8 *)(&stack0x00000320 + lVar25) = uVar29;
                uVar29 = *(undefined8 *)(puVar1 + lVar26 + 8);
                uVar31 = *(undefined8 *)(puVar1 + lVar26 + 0x10);
                uVar10 = *(undefined8 *)(puVar1 + lVar26 + 0x18);
                uVar28 = *(undefined8 *)(puVar1 + lVar26 + 0x20);
                uVar14 = *(undefined8 *)(puVar1 + lVar26 + 0x28);
                uVar15 = *(undefined8 *)(puVar1 + lVar26 + 0x30);
                uVar16 = *(undefined8 *)(puVar1 + lVar26 + 0x40);
                *(undefined8 *)(&stack0x00000308 + lVar25) = *(undefined8 *)(puVar1 + lVar26 + 0x38)
                ;
                *(undefined8 *)(&stack0x00000310 + lVar25) = uVar16;
                *(undefined8 *)(&stack0x000002f8 + lVar25) = uVar14;
                *(undefined8 *)(&stack0x00000300 + lVar25) = uVar15;
                *(undefined8 *)(&stack0x000002e8 + lVar25) = uVar10;
                *(undefined8 *)(&stack0x000002f0 + lVar25) = uVar28;
                *(undefined8 *)(&stack0x000002d8 + lVar25) = uVar29;
                *(undefined8 *)(&stack0x000002e0 + lVar25) = uVar31;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9872;
                FUN_140294cf0(puVar41,&stack0x000002d0 + lVar25,-(longlong)puVar43);
                *(undefined8 *)(&stack0x00001070 + lVar25) =
                     *(undefined8 *)(&stack0x00000d90 + lVar25);
                *(undefined8 *)(&stack0x00001060 + lVar25) =
                     *(undefined8 *)(&stack0x00000d80 + lVar25);
                *(undefined8 *)(&stack0x00001068 + lVar25) =
                     *(undefined8 *)(&stack0x00000d88 + lVar25);
                *(undefined8 *)(&stack0x00001050 + lVar25) =
                     *(undefined8 *)(&stack0x00000d70 + lVar25);
                *(undefined8 *)(&stack0x00001058 + lVar25) =
                     *(undefined8 *)(&stack0x00000d78 + lVar25);
                *(undefined8 *)(&stack0x00001040 + lVar25) =
                     *(undefined8 *)(&stack0x00000d60 + lVar25);
                *(undefined8 *)(&stack0x00001048 + lVar25) =
                     *(undefined8 *)(&stack0x00000d68 + lVar25);
                *(undefined8 *)(&stack0x000009a0 + lVar25) =
                     *(undefined8 *)(&stack0x00000d98 + lVar25);
                *(undefined8 *)(&stack0x000009a8 + lVar25) =
                     *(undefined8 *)(&stack0x00000da0 + lVar25);
                *(undefined8 *)(&stack0x000009b0 + lVar25) =
                     *(undefined8 *)(&stack0x00000da8 + lVar25);
                *(undefined8 *)(&stack0x000009b8 + lVar25) =
                     *(undefined8 *)(&stack0x00000db0 + lVar25);
                *(undefined8 *)(&stack0x000009c0 + lVar25) =
                     *(undefined8 *)(&stack0x00000db8 + lVar25);
                *(undefined8 *)(&stack0x000009c8 + lVar25) =
                     *(undefined8 *)(&stack0x00000dc0 + lVar25);
                *(undefined8 *)(&stack0x000009d0 + lVar25) =
                     *(undefined8 *)(&stack0x00000dc8 + lVar25);
                *(undefined8 *)(&stack0x000009d8 + lVar25) =
                     *(undefined8 *)(&stack0x00000dd0 + lVar25);
                *(undefined8 *)(&stack0x000009e0 + lVar25) =
                     *(undefined8 *)(&stack0x00000dd8 + lVar25);
                *(undefined8 *)(&stack0x000009e8 + lVar25) =
                     *(undefined8 *)(&stack0x00000de0 + lVar25);
                *(undefined8 *)(&stack0x000009f0 + lVar25) =
                     *(undefined8 *)(&stack0x00000de8 + lVar25);
                lVar26 = plVar35[2];
                if (lVar26 == *plVar35) {
                  *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e991b;
                  FUN_140562f30(plVar35,&UNK_1429ebe68);
                }
                lVar27 = plVar35[1];
                lVar38 = lVar26 * 0x58;
                uVar10 = *(undefined8 *)(&stack0x000009a0 + lVar25);
                uVar28 = *(undefined8 *)(&stack0x000009a8 + lVar25);
                uVar14 = *(undefined8 *)(&stack0x000009b0 + lVar25);
                uVar15 = *(undefined8 *)(&stack0x000009b8 + lVar25);
                uVar16 = *(undefined8 *)(&stack0x000009c0 + lVar25);
                uVar17 = *(undefined8 *)(&stack0x000009c8 + lVar25);
                uVar29 = *(undefined8 *)(&stack0x000009d0 + lVar25);
                uVar31 = *(undefined8 *)(&stack0x000009d8 + lVar25);
                uVar18 = *(undefined8 *)(&stack0x000009e0 + lVar25);
                uVar19 = *(undefined8 *)(&stack0x000009e8 + lVar25);
                *(undefined8 *)(lVar27 + 0x50 + lVar38) = *(undefined8 *)(&stack0x000009f0 + lVar25)
                ;
                puVar37 = (undefined8 *)(lVar27 + 0x40 + lVar38);
                *puVar37 = uVar18;
                puVar37[1] = uVar19;
                puVar37 = (undefined8 *)(lVar27 + 0x30 + lVar38);
                *puVar37 = uVar29;
                puVar37[1] = uVar31;
                puVar37 = (undefined8 *)(lVar27 + 0x20 + lVar38);
                *puVar37 = uVar16;
                puVar37[1] = uVar17;
                puVar37 = (undefined8 *)(lVar27 + 0x10 + lVar38);
                *puVar37 = uVar14;
                puVar37[1] = uVar15;
                *(undefined8 *)(lVar27 + lVar38) = uVar10;
                ((undefined8 *)(lVar27 + lVar38))[1] = uVar28;
                plVar35[2] = lVar26 + 1;
                *(undefined8 *)(&stack0x00000d90 + lVar25) =
                     *(undefined8 *)(&stack0x00001070 + lVar25);
                *(undefined8 *)(&stack0x00000d80 + lVar25) =
                     *(undefined8 *)(&stack0x00001060 + lVar25);
                *(undefined8 *)(&stack0x00000d88 + lVar25) =
                     *(undefined8 *)(&stack0x00001068 + lVar25);
                *(undefined8 *)(&stack0x00000d70 + lVar25) =
                     *(undefined8 *)(&stack0x00001050 + lVar25);
                *(undefined8 *)(&stack0x00000d78 + lVar25) =
                     *(undefined8 *)(&stack0x00001058 + lVar25);
                *(undefined8 *)(&stack0x00000d60 + lVar25) =
                     *(undefined8 *)(&stack0x00001040 + lVar25);
                *(undefined8 *)(&stack0x00000d68 + lVar25) =
                     *(undefined8 *)(&stack0x00001048 + lVar25);
                (&stack0x0000114d)[lVar25] = 0;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e99c5;
                FUN_140bd3cb0(&stack0x00000a00 + lVar25,puVar41);
                puVar43 = *(undefined1 **)(&stack0x000010e8 + lVar25);
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e99d4;
                FUN_140bd4030(&stack0x00000a00 + lVar25);
                if (*(longlong *)(&stack0x00000a00 + lVar25) != 0) {
                  puVar43 = (undefined1 *)(*(longlong *)(&stack0x00000a00 + lVar25) * 0x18);
                  *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e99fe;
                  func_0x000140613c20(*(undefined8 *)(&stack0x00000a08 + lVar25),puVar43,8);
                }
                uVar40 = *(ulonglong *)(&stack0x00000570 + lVar25);
              }
            }
            uVar24 = SUB84(puVar43,0);
            (&stack0x00001156)[lVar25] = 1;
            (&stack0x00001155)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9a1d;
            FUN_1405d1490(&stack0x00000570 + lVar25);
            lVar26 = *(longlong *)(&stack0x00001120 + lVar25);
            uVar29 = *(undefined8 *)(&stack0x00001118 + lVar25);
            lVar27 = *(longlong *)(&stack0x00001238 + lVar25);
            (&stack0x00001156)[lVar25] = 1;
            (&stack0x00001155)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9a45;
            uVar31 = FUN_140d635e0();
            *(undefined8 *)(&stack0x00000b10 + lVar25) = uVar31;
            *(undefined4 *)(&stack0x00000b18 + lVar25) = uVar24;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9a6b;
            func_0x000142923c00(&stack0x00000d60 + lVar25,&stack0x00000b40 + lVar25,0x218);
            (&stack0x00001068)[lVar25] = 2;
            (&stack0x0000114c)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9a8b;
            FUN_140909310(&stack0x00000570 + lVar25,uVar29);
            (&stack0x0000114c)[lVar25] = 0;
            *(undefined1 **)(&stack0xffffffffffffffe0 + lVar25) = &stack0x00000570 + lVar25;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9ab5;
            FUN_1400fa190(&stack0xfffffffffffffff8 + lVar25,&stack0x00000350 + lVar25,
                          &stack0x00000d60 + lVar25,&stack0x00001040 + lVar25);
            (&stack0x00001156)[lVar25] = 1;
            (&stack0x00001155)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9ada;
            FUN_140be9d00(&stack0x00000d60 + lVar25,&stack0xfffffffffffffff8 + lVar25,
                          *(undefined8 *)(&stack0x000004d0 + lVar25));
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9af3;
            func_0x000142923c00(&stack0x00000570 + lVar25,&stack0x00000d60 + lVar25,0x90);
            *(undefined8 *)(&stack0x00000b00 + lVar25) = *(undefined8 *)(&stack0x00000e10 + lVar25);
            *(undefined8 *)(&stack0x00000b08 + lVar25) = *(undefined8 *)(&stack0x00000e18 + lVar25);
            *(undefined8 *)(&stack0x00000af0 + lVar25) = *(undefined8 *)(&stack0x00000e00 + lVar25);
            *(undefined8 *)(&stack0x00000af8 + lVar25) = *(undefined8 *)(&stack0x00000e08 + lVar25);
            *(undefined8 *)(&stack0x00000ae0 + lVar25) = *(undefined8 *)(&stack0x00000df0 + lVar25);
            *(undefined8 *)(&stack0x00000ae8 + lVar25) = *(undefined8 *)(&stack0x00000df8 + lVar25);
            (&stack0x00001152)[lVar25] = 1;
            (&stack0x00001151)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9b4b;
            FUN_140be9c80(&stack0x00000d60 + lVar25,&stack0x00000ae0 + lVar25,
                          *(undefined4 *)(&stack0x000010f8 + lVar25));
            (&stack0x00001152)[lVar25] = 1;
            (&stack0x00001151)[lVar25] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9b6c;
            FUN_140bd63f0(&stack0x00000a68 + lVar25,&stack0x00000d60 + lVar25);
            uVar40 = lVar26 + 0xd0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9b88;
            func_0x000142923c00(&stack0x00000d60 + lVar25,uVar40,0x90);
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9b9d;
            func_0x000142923c00(uVar40,&stack0x00000570 + lVar25,0x90);
            if (!SBORROW8(0,*(longlong *)(&stack0x00000d60 + lVar25))) {
              (&stack0x00001154)[lVar25] = 1;
              (&stack0x00001153)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9bc2;
              FUN_1405ef210(&stack0x00000d60 + lVar25);
            }
            cVar21 = '\x01';
            if ((&stack0x0000113c)[lVar25] == '\x01' && lVar27 != 0) {
              (&stack0x00001154)[lVar25] = 1;
              (&stack0x00001153)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9bfd;
              FUN_140bfd9d0(&stack0x00000a00 + lVar25,*(undefined8 *)(&stack0x00001140 + lVar25),
                            lVar27);
              if ((&stack0x00000a5e)[lVar25] == '\a') goto code_r0x0001404e9c06;
              *(undefined8 *)(&stack0x000010a0 + lVar25) =
                   *(undefined8 *)(&stack0x00000a60 + lVar25);
              *(undefined8 *)(&stack0x00001090 + lVar25) =
                   *(undefined8 *)(&stack0x00000a50 + lVar25);
              *(undefined8 *)(&stack0x00001098 + lVar25) =
                   *(undefined8 *)(&stack0x00000a58 + lVar25);
              *(undefined8 *)(&stack0x00001080 + lVar25) =
                   *(undefined8 *)(&stack0x00000a40 + lVar25);
              *(undefined8 *)(&stack0x00001088 + lVar25) =
                   *(undefined8 *)(&stack0x00000a48 + lVar25);
              *(undefined8 *)(&stack0x00001070 + lVar25) =
                   *(undefined8 *)(&stack0x00000a30 + lVar25);
              *(undefined8 *)(&stack0x00001078 + lVar25) =
                   *(undefined8 *)(&stack0x00000a38 + lVar25);
              *(undefined8 *)(&stack0x00001060 + lVar25) =
                   *(undefined8 *)(&stack0x00000a20 + lVar25);
              *(undefined8 *)(&stack0x00001068 + lVar25) =
                   *(undefined8 *)(&stack0x00000a28 + lVar25);
              *(undefined8 *)(&stack0x00001050 + lVar25) =
                   *(undefined8 *)(&stack0x00000a10 + lVar25);
              *(undefined8 *)(&stack0x00001058 + lVar25) =
                   *(undefined8 *)(&stack0x00000a18 + lVar25);
              *(undefined8 *)(&stack0x00001040 + lVar25) =
                   *(undefined8 *)(&stack0x00000a00 + lVar25);
              *(undefined8 *)(&stack0x00001048 + lVar25) =
                   *(undefined8 *)(&stack0x00000a08 + lVar25);
              (&stack0x0000114f)[lVar25] = 1;
              (&stack0x0000114e)[lVar25] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9c9f;
              func_0x000140c0f120(&stack0x00000d60 + lVar25,&stack0x00001040 + lVar25);
              uVar29 = *(undefined8 *)(&stack0x00000d60 + lVar25);
              *(undefined8 *)(&stack0x00000990 + lVar25) =
                   *(undefined8 *)(&stack0x00000d88 + lVar25);
              *(undefined8 *)(&stack0x00000998 + lVar25) =
                   *(undefined8 *)(&stack0x00000d90 + lVar25);
              *(undefined8 *)(&stack0x00000980 + lVar25) =
                   *(undefined8 *)(&stack0x00000d78 + lVar25);
              *(undefined8 *)(&stack0x00000988 + lVar25) =
                   *(undefined8 *)(&stack0x00000d80 + lVar25);
              *(undefined8 *)(&stack0x00000970 + lVar25) =
                   *(undefined8 *)(&stack0x00000d68 + lVar25);
              *(undefined8 *)(&stack0x00000978 + lVar25) =
                   *(undefined8 *)(&stack0x00000d70 + lVar25);
              *(undefined8 *)(&stack0x00000ad0 + lVar25) =
                   *(undefined8 *)(&stack0x00000a98 + lVar25);
              *(undefined8 *)(&stack0x00000ac0 + lVar25) =
                   *(undefined8 *)(&stack0x00000a88 + lVar25);
              *(undefined8 *)(&stack0x00000ac8 + lVar25) =
                   *(undefined8 *)(&stack0x00000a90 + lVar25);
              *(undefined8 *)(&stack0x00000ab0 + lVar25) =
                   *(undefined8 *)(&stack0x00000a78 + lVar25);
              *(undefined8 *)(&stack0x00000ab8 + lVar25) =
                   *(undefined8 *)(&stack0x00000a80 + lVar25);
              *(undefined8 *)(&stack0x00000aa0 + lVar25) =
                   *(undefined8 *)(&stack0x00000a68 + lVar25);
              *(undefined8 *)(&stack0x00000aa8 + lVar25) =
                   *(undefined8 *)(&stack0x00000a70 + lVar25);
              uVar31 = *(undefined8 *)(&stack0x000004c8 + lVar25);
              (&stack0x0000114b)[lVar25] = 1;
              uVar10 = *(undefined8 *)(&stack0x00001118 + lVar25);
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9d32;
              FUN_140909310(&stack0x00000790 + lVar25,uVar10);
              *(undefined8 *)(&stack0x00001020 + lVar25) =
                   *(undefined8 *)(&stack0x00000990 + lVar25);
              *(undefined8 *)(&stack0x00001028 + lVar25) =
                   *(undefined8 *)(&stack0x00000998 + lVar25);
              *(undefined8 *)(&stack0x00001010 + lVar25) =
                   *(undefined8 *)(&stack0x00000980 + lVar25);
              *(undefined8 *)(&stack0x00001018 + lVar25) =
                   *(undefined8 *)(&stack0x00000988 + lVar25);
              *(undefined8 *)(&stack0x00001000 + lVar25) =
                   *(undefined8 *)(&stack0x00000970 + lVar25);
              *(undefined8 *)(&stack0x00001008 + lVar25) =
                   *(undefined8 *)(&stack0x00000978 + lVar25);
              *(undefined8 *)(&stack0x00000d60 + lVar25) = uVar10;
              *(undefined8 *)(&stack0x00000d68 + lVar25) = uVar31;
              *(longlong **)(&stack0x00000d70 + lVar25) = plVar35 + 0xf;
              *(undefined8 *)(&stack0x00000d88 + lVar25) =
                   *(undefined8 *)(&stack0x00000aa0 + lVar25);
              *(undefined8 *)(&stack0x00000d90 + lVar25) =
                   *(undefined8 *)(&stack0x00000aa8 + lVar25);
              *(undefined8 *)(&stack0x00000d98 + lVar25) =
                   *(undefined8 *)(&stack0x00000ab0 + lVar25);
              *(undefined8 *)(&stack0x00000da0 + lVar25) =
                   *(undefined8 *)(&stack0x00000ab8 + lVar25);
              *(undefined8 *)(&stack0x00000da8 + lVar25) =
                   *(undefined8 *)(&stack0x00000ac0 + lVar25);
              *(undefined8 *)(&stack0x00000db0 + lVar25) =
                   *(undefined8 *)(&stack0x00000ac8 + lVar25);
              *(undefined8 *)(&stack0x00000db8 + lVar25) =
                   *(undefined8 *)(&stack0x00000ad0 + lVar25);
              *(undefined8 *)(&stack0x00000d78 + lVar25) = uVar29;
              *(undefined1 **)(&stack0x00000d80 + lVar25) = &stack0x000007c8 + lVar25;
              (&stack0x0000114b)[lVar25] = 0;
              *(undefined1 **)(&stack0xffffffffffffffe8 + lVar25) = &stack0x00000d60 + lVar25;
              *(undefined1 **)(&stack0xffffffffffffffe0 + lVar25) = &stack0x00001000 + lVar25;
              puVar41 = &stack0x00000350 + lVar25;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9e0b;
              FUN_1400fb5b0(&stack0x000008f0 + lVar25,puVar41,&stack0x00000790 + lVar25,
                            *(longlong *)(&stack0x00001120 + lVar25) + 0x1c0);
              uVar47 = (uint)puVar41;
              if (*(longlong *)(&stack0x000008f0 + lVar25) == 0) {
                *(undefined8 *)(&stack0x00001000 + lVar25) = 0;
code_r0x0001404ea72c:
                *(undefined1 **)(&stack0x00000790 + lVar25) = &stack0x00001040 + lVar25;
                *(undefined **)(&stack0x00000798 + lVar25) = &UNK_1404e7ff0;
                *(undefined **)(&stack0x00000d60 + lVar25) = &UNK_1429ebe28;
                *(undefined8 *)(&stack0x00000d68 + lVar25) = 1;
                *(undefined8 *)(&stack0x00000d80 + lVar25) = 0;
                *(undefined1 **)(&stack0x00000d70 + lVar25) = &stack0x00000790 + lVar25;
                *(undefined8 *)(&stack0x00000d78 + lVar25) = 1;
                (&stack0x0000114a)[lVar25] = 1;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea791;
                FUN_1428d9390(&stack0x00000d60 + lVar25,&UNK_1429ebe38);
                goto code_r0x0001404ea7cd;
              }
              *(undefined8 *)(&stack0x00001000 + lVar25) =
                   *(undefined8 *)(&stack0x000008f0 + lVar25);
              *(undefined8 *)(&stack0x00001008 + lVar25) =
                   *(undefined8 *)(&stack0x000008f8 + lVar25);
              *(undefined8 *)(&stack0x00001020 + lVar25) =
                   *(undefined8 *)(&stack0x00000910 + lVar25);
              *(undefined8 *)(&stack0x00001028 + lVar25) =
                   *(undefined8 *)(&stack0x00000918 + lVar25);
              *(undefined8 *)(&stack0x00001010 + lVar25) =
                   *(undefined8 *)(&stack0x00000900 + lVar25);
              *(undefined8 *)(&stack0x00001018 + lVar25) =
                   *(undefined8 *)(&stack0x00000908 + lVar25);
              if (*(longlong *)(&stack0x00001000 + lVar25) == 0) goto code_r0x0001404ea72c;
              *(undefined8 *)(&stack0x00000dc0 + lVar25) =
                   *(undefined8 *)(&stack0x000010a0 + lVar25);
              *(undefined8 *)(&stack0x00000db0 + lVar25) =
                   *(undefined8 *)(&stack0x00001090 + lVar25);
              *(undefined8 *)(&stack0x00000db8 + lVar25) =
                   *(undefined8 *)(&stack0x00001098 + lVar25);
              *(undefined8 *)(&stack0x00000da0 + lVar25) =
                   *(undefined8 *)(&stack0x00001080 + lVar25);
              *(undefined8 *)(&stack0x00000da8 + lVar25) =
                   *(undefined8 *)(&stack0x00001088 + lVar25);
              *(undefined8 *)(&stack0x00000d90 + lVar25) =
                   *(undefined8 *)(&stack0x00001070 + lVar25);
              *(undefined8 *)(&stack0x00000d98 + lVar25) =
                   *(undefined8 *)(&stack0x00001078 + lVar25);
              *(undefined8 *)(&stack0x00000d80 + lVar25) =
                   *(undefined8 *)(&stack0x00001060 + lVar25);
              *(undefined8 *)(&stack0x00000d88 + lVar25) =
                   *(undefined8 *)(&stack0x00001068 + lVar25);
              *(undefined8 *)(&stack0x00000d70 + lVar25) =
                   *(undefined8 *)(&stack0x00001050 + lVar25);
              *(undefined8 *)(&stack0x00000d78 + lVar25) =
                   *(undefined8 *)(&stack0x00001058 + lVar25);
              *(undefined8 *)(&stack0x00000d60 + lVar25) =
                   *(undefined8 *)(&stack0x00001040 + lVar25);
              *(undefined8 *)(&stack0x00000d68 + lVar25) =
                   *(undefined8 *)(&stack0x00001048 + lVar25);
              (&stack0x0000114a)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9ed4;
              lVar26 = FUN_140c0f170(&stack0x00000d60 + lVar25);
              uVar40 = (ulonglong)uVar47;
              if (*(longlong *)(&stack0x00001000 + lVar25) != 0) {
                (&stack0x0000114f)[lVar25] = 0;
                (&stack0x0000114e)[lVar25] = 0;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9f10;
                FUN_14004f690(&stack0x00001000 + lVar25,&stack0x00001020 + lVar25,0x48,0x10);
              }
              bVar50 = true;
              cVar21 = '\0';
            }
            else {
code_r0x0001404e9c06:
              bVar50 = false;
            }
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9f32;
            func_0x000142923c00(&stack0x00000d60 + lVar25,&stack0x00000350 + lVar25,400);
            (&stack0x00001154)[lVar25] = cVar21;
            (&stack0x00001153)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9f48;
            FUN_140bec400(&stack0x00000d60 + lVar25);
            if (bVar50) {
              if (lVar27 == 0) {
                (&stack0x00001154)[lVar25] = cVar21;
                (&stack0x00001153)[lVar25] = 0;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea612;
                FUN_1428d9310(&UNK_1429ebe50);
                goto code_r0x0001404ea7cd;
              }
              (&stack0x00001154)[lVar25] = cVar21;
              (&stack0x00001153)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9f79;
              FUN_140c0ef80(lVar27,*(undefined8 *)(&stack0x00001140 + lVar25),lVar26,
                            uVar40 & 0xffffffff);
            }
            (&stack0x00001154)[lVar25] = cVar21;
            (&stack0x00001153)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9f93;
            FUN_14031ea20(*(undefined8 *)(&stack0x00001140 + lVar25));
            (&stack0x00001154)[lVar25] = cVar21;
            (&stack0x00001153)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9fbb;
            func_0x000140c02da0(&stack0x00000d60 + lVar25,
                                *(longlong *)(&stack0x000010d0 + lVar25) + 0xab0);
            uRam0000000143634ba8 = *(undefined8 *)(&stack0x00000d70 + lVar25);
            uRam0000000143634b98 = *(undefined8 *)(&stack0x00000d60 + lVar25);
            uRam0000000143634ba0 = *(undefined8 *)(&stack0x00000d68 + lVar25);
            uVar29 = *(undefined8 *)(&stack0x00001120 + lVar25);
            if (cVar21 != '\0') {
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e9ff1;
              func_0x00014003bb80(&stack0x00000a68 + lVar25);
              (&stack0x00001152)[lVar25] = 0;
              (&stack0x00001151)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea00b;
              FUN_140be24b0(&stack0x00000a68 + lVar25);
            }
            puVar41 = &stack0x00000b00 + lVar25;
            (&stack0x00001149)[lVar25] = 0;
            (&stack0x00001148)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea038;
            FUN_14004f690(&stack0x00000ae0 + lVar25,puVar41,0x48,0x10);
            (&stack0x00001156)[lVar25] = 0;
            (&stack0x00001155)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea052;
            func_0x000140bf8f80(*(undefined8 *)(&stack0x00001140 + lVar25));
            iVar39 = (int)puVar41;
            if ((&stack0x000008ba)[lVar25] != '\0') {
              (&stack0x00001156)[lVar25] = 0;
              (&stack0x00001155)[lVar25] = 0;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea079;
              uVar31 = FUN_140d63910(&stack0x00000b10 + lVar25);
              (&stack0x00001156)[lVar25] = 0;
              (&stack0x00001155)[lVar25] = 0;
              puVar41 = *(undefined1 **)(&stack0x00001140 + lVar25);
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea09b;
              lVar26 = FUN_140beaae0(uVar29);
              if (lVar26 == 1) {
                auVar55._8_4_ = (int)((ulonglong)uVar31 >> 0x20);
                auVar55._0_8_ = uVar31;
                auVar55._12_4_ = _UNK_14292e934;
                dVar52 = (double)CONCAT44(_UNK_14292e930,(int)uVar31) - _UNK_14292e940;
                dVar57 = auVar55._8_8_ - _UNK_14292e948;
                dVar59 = (double)iVar39 / _UNK_1429a25a8;
                uVar31 = *(undefined8 *)(&stack0x00001108 + lVar25);
                auVar56._8_4_ = (int)((ulonglong)uVar31 >> 0x20);
                auVar56._0_8_ = uVar31;
                auVar56._12_4_ = _UNK_14292e934;
                dVar53 = (double)CONCAT44(_UNK_14292e930,(int)uVar31) - _UNK_14292e940;
                dVar58 = auVar56._8_8_ - _UNK_14292e948;
                dVar54 = (double)*(int *)(&stack0x00001100 + lVar25) / _UNK_1429a25a8;
                auVar60._8_4_ = (int)((ulonglong)puVar41 >> 0x20);
                auVar60._0_8_ = puVar41;
                auVar60._12_4_ = _UNK_14292e934;
                *(double *)((ulonglong)((uint)lRam0000000143634bd8 & 3) * 8 + 0x143634bb8) =
                     ((auVar60._8_8_ - _UNK_14292e948) +
                     ((double)CONCAT44(_UNK_14292e930,(int)puVar41) - _UNK_14292e940)) /
                     _UNK_1429a25a8;
                lVar26 = lRam0000000143634c00 + 1;
                *(double *)((ulonglong)((uint)lRam0000000143634c00 & 3) * 8 + 0x143634be0) =
                     dVar54 + dVar58 + dVar53 + dVar59 + dVar57 + dVar52;
                lVar27 = lRam0000000143634bd8 + 1;
                dVar52 = _UNK_14292fee8;
                if (lVar27 != 0) {
                  dVar52 = dRam0000000143634bb8;
                  if (_UNK_14292fee8 <= dRam0000000143634bb8) {
                    dVar52 = _UNK_14292fee8;
                  }
                  if (lRam0000000143634bd8 != 0) {
                    dVar53 = dRam0000000143634bc0;
                    if (dVar52 <= dRam0000000143634bc0) {
                      dVar53 = dVar52;
                    }
                    dVar52 = (double)(~-(ulonglong)NAN(dVar52) & (ulonglong)dVar53 |
                                     -(ulonglong)NAN(dVar52) & (ulonglong)dRam0000000143634bc0);
                    if (lVar27 != 2) {
                      dVar53 = dRam0000000143634bc8;
                      if (dVar52 <= dRam0000000143634bc8) {
                        dVar53 = dVar52;
                      }
                      dVar52 = (double)(~-(ulonglong)NAN(dVar52) & (ulonglong)dVar53 |
                                       -(ulonglong)NAN(dVar52) & (ulonglong)dRam0000000143634bc8);
                      if (lVar27 != 3) {
                        dVar53 = dRam0000000143634bd0;
                        if (dVar52 <= dRam0000000143634bd0) {
                          dVar53 = dVar52;
                        }
                        dVar52 = (double)(~-(ulonglong)NAN(dVar52) & (ulonglong)dVar53 |
                                         -(ulonglong)NAN(dVar52) & (ulonglong)dRam0000000143634bd0);
                      }
                    }
                  }
                }
                dVar53 = _UNK_1429eb700;
                if (lVar26 != 0) {
                  dVar53 = dRam0000000143634be0;
                  if (dRam0000000143634be0 <= _UNK_1429eb700) {
                    dVar53 = _UNK_1429eb700;
                  }
                  if (lRam0000000143634c00 != 0) {
                    dVar54 = dRam0000000143634be8;
                    if (dRam0000000143634be8 <= dVar53) {
                      dVar54 = dVar53;
                    }
                    dVar53 = (double)(~-(ulonglong)NAN(dVar53) & (ulonglong)dVar54 |
                                     -(ulonglong)NAN(dVar53) & (ulonglong)dRam0000000143634be8);
                    if (lVar26 != 2) {
                      dVar54 = dRam0000000143634bf0;
                      if (dRam0000000143634bf0 <= dVar53) {
                        dVar54 = dVar53;
                      }
                      dVar53 = (double)(~-(ulonglong)NAN(dVar53) & (ulonglong)dVar54 |
                                       -(ulonglong)NAN(dVar53) & (ulonglong)dRam0000000143634bf0);
                      if (lVar26 != 3) {
                        dVar54 = dRam0000000143634bf8;
                        if (dRam0000000143634bf8 <= dVar53) {
                          dVar54 = dVar53;
                        }
                        dVar53 = (double)(~-(ulonglong)NAN(dVar53) & (ulonglong)dVar54 |
                                         -(ulonglong)NAN(dVar53) & (ulonglong)dRam0000000143634bf8);
                      }
                    }
                  }
                }
                dVar52 = dVar52 - dVar53;
                if (dVar52 <= 0.0) {
                  dVar52 = 0.0;
                }
                uVar47 = (uint)((ulonglong)dVar52 >> 0x34) & 0x7ff;
                uVar40 = 0;
                lRam0000000143634bd8 = lVar27;
                lRam0000000143634c00 = lVar26;
                if (uVar47 < 0x3e0) {
                  uVar32 = 0;
                }
                else {
                  uVar32 = (ulonglong)dVar52 & 0xfffffffffffff | 0x10000000000000;
                  cVar21 = (char)((ulonglong)dVar52 >> 0x34);
                  if (uVar47 < 0x3ff) {
                    bVar36 = cVar21 + 0x2d;
                    uVar33 = uVar32 << (bVar36 & 0x3f);
                    uVar34 = uVar33;
                    uVar40 = 0L << (bVar36 & 0x3f) | uVar32 >> 0x40 - (bVar36 & 0x3f);
                    if ((bVar36 & 0x40) != 0) {
                      uVar34 = 0;
                      uVar40 = uVar33;
                    }
                    auVar11._8_8_ = 0;
                    auVar11._0_8_ = uVar34;
                    uVar40 = SUB168(auVar11 * ZEXT816(1000000000),8) + uVar40 * 1000000000;
                    uVar47 = (uint)(byte)(((byte)(uVar40 >> 0x20) |
                                          ((uVar40 & 0xffffffff) != 0x80000000 ||
                                          SUB168(auVar11 * ZEXT816(1000000000),0) != 0)) &
                                         (byte)(uVar40 >> 0x18) >> 7) + (int)(uVar40 >> 0x20);
                    uVar32 = (ulonglong)(uVar47 == 1000000000);
                    uVar40 = (ulonglong)uVar47;
                    if (uVar47 == 1000000000) {
                      uVar40 = 0;
                    }
                  }
                  else if (uVar47 < 0x433) {
                    auVar12._8_8_ = 0;
                    auVar12._0_8_ = (longlong)dVar52 << (cVar21 + 1U & 0x3f) & 0xfffffffffffff;
                    auVar12 = auVar12 * ZEXT816(1000000000);
                    uVar47 = (uint)(((byte)(auVar12._0_8_ >> 0x34) |
                                    (auVar12._0_8_ & 0xffffffffffe00) != 0x8000000000000) &
                                   (byte)(auVar12._4_4_ >> 0x13) & 1) +
                             (auVar12._8_4_ << 0xc | auVar12._4_4_ >> 0x14);
                    uVar40 = 0;
                    if (uVar47 != 1000000000) {
                      uVar40 = (ulonglong)uVar47;
                    }
                    uVar32 = (ulonglong)(uVar47 == 1000000000) + (uVar32 >> (0x33U - cVar21 & 0x3f))
                    ;
                  }
                  else {
                    if (0x43e < uVar47) {
                      *(undefined **)(&stack0x00000d60 + lVar25) = &UNK_1429eb7b8;
                      *(undefined8 *)(&stack0x00000d68 + lVar25) = 0x48;
                      (&stack0x00001156)[lVar25] = 0;
                      (&stack0x00001155)[lVar25] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea7cd;
                      FUN_1428ce090(&stack0x00000d60 + lVar25,&UNK_1429eb848,uVar47,0);
                      goto code_r0x0001404ea7cd;
                    }
                    uVar32 = uVar32 << (cVar21 + 0xdU & 0x3f);
                  }
                }
                (&stack0x00001156)[lVar25] = 0;
                (&stack0x00001155)[lVar25] = 0;
                puVar41 = *(undefined1 **)(&stack0x00001140 + lVar25);
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea46d;
                FUN_140bea680(uVar29,puVar41,uVar32,uVar40);
              }
            }
            iVar39 = (int)puVar41;
            (&stack0x00001156)[lVar25] = 0;
            (&stack0x00001155)[lVar25] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea480;
            uRam0000000143634bb0 = FUN_140d635e0();
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea499;
            iRam000000014362b0e8 = iVar39;
            FUN_1405fdb80(&stack0x000007c8 + lVar25);
            lVar26 = *(longlong *)(&stack0x00000890 + lVar25);
            if ((!SBORROW8(0,lVar26)) && (lVar26 != 0)) {
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea4bc;
              func_0x000140613c20(*(undefined8 *)(&stack0x00000898 + lVar25),lVar26,1);
            }
            plVar35 = *(longlong **)(&stack0x000010e0 + lVar25);
            LOCK();
            *plVar35 = *plVar35 + -1;
            UNLOCK();
            if (*plVar35 == 0) {
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea4d5;
              FUN_140b5be90(&stack0x000010e0 + lVar25);
            }
            if (*(longlong *)(&stack0x000010c0 + lVar25) == 0) {
              return;
            }
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea4e4;
            plVar35 = (longlong *)func_0x000140c18130();
            if (*plVar35 == 1) {
              plVar35 = plVar35 + 1;
code_r0x0001404ea4f4:
              if (*plVar35 == 0) {
                *plVar35 = -1;
                *(longlong **)(&stack0x000010e8 + lVar25) = plVar35;
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea51c;
                FUN_140c170f0(plVar35 + 1,*(undefined8 *)(&stack0x000010c8 + lVar25));
                **(longlong **)(&stack0x000010e8 + lVar25) =
                     **(longlong **)(&stack0x000010e8 + lVar25) + 1;
                return;
              }
            }
            else {
              if (*plVar35 == 0) {
                *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea574;
                plVar35 = (longlong *)FUN_1428a1640(plVar35,0);
                if (plVar35 != (longlong *)0x0) goto code_r0x0001404ea4f4;
              }
code_r0x0001404ea59a:
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea5a6;
              FUN_1428d80d0(&UNK_142932a10);
            }
            goto code_r0x0001404ea5a6;
          }
        }
        *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea5db;
        FUN_1428d9330(&UNK_1429ebba0,10,&UNK_1429ebbb0);
        goto code_r0x0001404ea7cd;
      }
    }
  }
  else {
    param_2 = 0x1f;
    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e809d;
    lVar26 = FUN_1404e7b60(0x1f);
    puVar44 = &UNK_1429eb8f4;
    if (lVar26 == 1) {
      if (uVar40 != 0) {
        if (uVar40 < 0x1f) {
          if ((char)(&UNK_1429eb8f4)[uVar40] < -0x40) {
code_r0x0001404e80ca:
            *(undefined **)(&stack0xffffffffffffffe0 + lVar25) = &UNK_1429eb998;
            *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e80ea;
            FUN_1428d9940(&UNK_1429eb8f4,0x1f,0);
            goto code_r0x0001404e80ea;
          }
        }
        else if (uVar40 != 0x1f) goto code_r0x0001404e80ca;
      }
      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e80fb;
      lVar26 = FUN_1404e7b60(uVar40);
      if (lVar26 == 1) {
        uVar32 = uVar40 + 2;
        if (uVar32 != 0) {
          uVar22 = uVar32 == 0x1f;
          if (0x1e < uVar32) goto code_r0x0001404e8141;
          uVar22 = (&UNK_1429eb8f6)[uVar40] == -0x41;
          if ((char)(&UNK_1429eb8f6)[uVar40] < -0x40) {
            do {
              *(undefined **)(&stack0xffffffffffffffe0 + lVar25) = &UNK_1429eb9b0;
              uVar40 = 0x1f;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8141;
              FUN_1428d9940(&UNK_1429eb8f4,0x1f);
code_r0x0001404e8141:
            } while (!(bool)uVar22);
          }
        }
        param_2 = 0x1d - uVar40;
        puVar44 = &UNK_1429eb8f4 + uVar32;
      }
    }
    puVar48 = &UNK_1429ebb54;
    puVar49 = &UNK_1429ebb30;
    do {
      bVar36 = puVar48[-1];
      uVar47 = (uint)(char)bVar36;
      if ((char)bVar36 < '\0') {
        bVar3 = puVar48[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar48[-3];
          if ((char)bVar4 < -0x40) {
            pbVar30 = puVar48 + -4;
            puVar48 = puVar48 + -4;
            uVar47 = bVar4 & 0x3f | (*pbVar30 & 7) << 6;
          }
          else {
            puVar48 = puVar48 + -3;
            uVar47 = bVar4 & 0xf;
          }
          uVar47 = bVar3 & 0x3f | uVar47 << 6;
        }
        else {
          puVar48 = puVar48 + -2;
          uVar47 = bVar3 & 0x1f;
        }
        uVar47 = (uint)(bVar36 & 0x3f) | uVar47 << 6;
        if (uVar47 != 0x5c) goto code_r0x0001404e81e4;
code_r0x0001404e81f5:
        puVar49 = puVar48 + -0x1429ebb2f;
        if (puVar49 != (undefined *)0x0) {
          uVar22 = puVar49 == (undefined *)0x24;
          if ((undefined *)0x23 < puVar49) goto code_r0x0001404e8234;
          uVar22 = puVar48[1] == -0x41;
          if ((char)puVar48[1] < -0x40) {
            do {
              *(undefined **)(&stack0xffffffffffffffe0 + lVar25) = &UNK_1429eb978;
              *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8234;
              FUN_1428d9940(&UNK_1429ebb30,0x24);
code_r0x0001404e8234:
            } while (!(bool)uVar22);
          }
        }
        lVar26 = 0x24 - (longlong)puVar49;
        puVar49 = &UNK_1429ebb30 + (longlong)puVar49;
        goto code_r0x0001404e8241;
      }
      puVar48 = puVar48 + -1;
      if (uVar47 == 0x5c) goto code_r0x0001404e81f5;
code_r0x0001404e81e4:
      if (uVar47 == 0x2f) goto code_r0x0001404e81f5;
    } while (puVar48 != &UNK_1429ebb30);
    lVar26 = 0x24;
code_r0x0001404e8241:
    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e8246;
    plVar35 = (longlong *)func_0x000140c18130();
    if (*plVar35 != 1) {
      if (*plVar35 == 0) {
        *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea58e;
        plVar35 = (longlong *)FUN_1428a1640(plVar35,0);
        if (plVar35 != (longlong *)0x0) goto code_r0x0001404e825a;
      }
      goto code_r0x0001404ea59a;
    }
    plVar35 = plVar35 + 1;
code_r0x0001404e825a:
    if (*plVar35 == 0) {
      *plVar35 = -1;
      *(longlong **)(&stack0x00001140 + lVar25) = plVar35;
      *(longlong *)(&stack0xffffffffffffffe0 + lVar25) = lVar26;
      *(undefined8 *)(&stack0xfffffffffffffff0 + lVar25) = 0;
      *(undefined8 *)(&stack0xffffffffffffffe8 + lVar25) = 1;
      *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404e829b;
      uVar29 = FUN_140c16d40(plVar35 + 1,puVar44,param_2,puVar49);
      **(longlong **)(&stack0x00001140 + lVar25) = **(longlong **)(&stack0x00001140 + lVar25) + 1;
      *(undefined8 *)(&stack0x000010c8 + lVar25) = uVar29;
      uVar29 = 1;
      param_2 = *(ulonglong *)(&stack0x00001118 + lVar25);
      goto code_r0x0001404e82b8;
    }
code_r0x0001404ea5a6:
    *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea5b2;
    FUN_1428d9250(&UNK_142932ab8);
  }
  *(undefined **)((longlong)&puStack_48 + lVar25) = &UNK_1404ea5be;
  FUN_1428d9310(&UNK_1429ebb88);
code_r0x0001404ea7cd:
                    /* WARNING: Does not return */
  pcVar13 = (code *)invalidInstructionException();
  (*pcVar13)();
}

