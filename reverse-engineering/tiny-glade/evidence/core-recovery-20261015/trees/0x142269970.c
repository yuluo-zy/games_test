   1	
   2	void FUN_142269970(undefined8 *param_1,undefined8 *param_2,longlong *param_3,undefined8 *param_4,
   3	                  longlong *param_5)
   4	
   5	{
   6	  longlong *plVar1;
   7	  longlong *plVar2;
   8	  ulonglong *puVar3;
   9	  longlong lVar4;
  10	  undefined8 uVar5;
  11	  undefined4 *puVar6;
  12	  undefined8 uVar7;
  13	  undefined4 *puVar8;
  14	  undefined4 *puVar9;
  15	  longlong lVar10;
  16	  undefined1 auVar11 [16];
  17	  undefined1 auVar12 [16];
  18	  undefined1 auVar13 [16];
  19	  undefined1 auVar14 [16];
  20	  code *pcVar15;
  21	  longlong lVar16;
  22	  undefined4 uVar17;
  23	  undefined4 uVar18;
  24	  undefined4 uVar19;
  25	  undefined4 uVar20;
  26	  longlong *plVar21;
  27	  undefined8 uVar22;
  28	  longlong *plVar23;
  29	  undefined8 *puVar24;
  30	  longlong *plVar25;
  31	  longlong *plVar26;
  32	  ulonglong uVar27;
  33	  ulonglong uVar28;
  34	  undefined8 uVar29;
  35	  ulonglong uVar30;
  36	  ulonglong uVar31;
  37	  float fVar32;
  38	  float fVar33;
  39	  undefined8 uStack_198;
  40	  longlong *plStack_168;
  41	  longlong *plStack_160;
  42	  longlong *plStack_158;
  43	  longlong *plStack_150;
  44	  undefined4 uStack_148;
  45	  undefined8 uStack_144;
  46	  undefined4 uStack_13c;
  47	  undefined1 uStack_138;
  48	  undefined7 uStack_137;
  49	  longlong *plStack_130;
  50	  longlong *plStack_128;
  51	  longlong *plStack_120;
  52	  undefined8 uStack_118;
  53	  undefined8 uStack_110;
  54	  undefined4 uStack_108;
  55	  undefined4 uStack_104;
  56	  undefined8 uStack_100;
  57	  undefined4 uStack_f8;
  58	  undefined4 uStack_f4;
  59	  undefined4 uStack_f0;
  60	  undefined4 uStack_ec;
  61	  undefined1 uStack_e8;
  62	  undefined7 uStack_e7;
  63	  undefined4 uStack_d4;
  64	  undefined4 uStack_d0;
  65	  undefined4 uStack_cc;
  66	  longlong *plStack_c8;
  67	  longlong *plStack_c0;
  68	  longlong *plStack_b8;
  69	  longlong *plStack_b0;
  70	  undefined1 uStack_a1;
  71	  undefined8 uStack_a0;
  72	  
  73	  uStack_a0 = 0xfffffffffffffffe;
  74	  puVar3 = (ulonglong *)*param_2;
  75	  lVar4 = param_2[1];
  76	  uVar28 = *puVar3;
  77	  uVar31 = 0;
  78	  uVar30 = uVar28 - *(ulonglong *)(lVar4 + 0x18);
  79	  if (uVar28 < *(ulonglong *)(lVar4 + 0x18)) {
  80	    uVar30 = uVar31;
  81	  }
  82	  uVar27 = uVar28 - *(ulonglong *)(lVar4 + 0x38);
  83	  if (uVar28 < *(ulonglong *)(lVar4 + 0x38)) {
  84	    uVar27 = uVar31;
  85	  }
  86	  plStack_130 = (longlong *)(uVar30 * 0x18 + *(longlong *)(lVar4 + 8));
  87	  uVar28 = *(ulonglong *)(lVar4 + 0x10) - uVar30;
  88	  if (*(ulonglong *)(lVar4 + 0x10) < uVar30) {
  89	    plStack_130 = (longlong *)0x8;
  90	    uVar28 = uVar31;
  91	  }
  92	  plStack_c0 = (longlong *)(uVar27 * 0x18 + *(longlong *)(lVar4 + 0x28));
  93	  uVar30 = *(ulonglong *)(lVar4 + 0x30) - uVar27;
  94	  if (*(ulonglong *)(lVar4 + 0x30) < uVar27) {
  95	    plStack_c0 = (longlong *)0x8;
  96	    uVar30 = uVar31;
  97	  }
  98	  *puVar3 = (*(longlong *)(lVar4 + 0x40) - uVar30) - uVar28;
  99	  plVar1 = plStack_130 + uVar28 * 3;
 100	  plVar2 = plStack_c0 + uVar30 * 3;
 101	  uStack_d4 = *(undefined4 *)((longlong)param_1 + 0x1c);
 102	  uVar5 = *param_1;
 103	  puVar6 = (undefined4 *)param_1[2];
 104	  param_5 = (longlong *)*param_5;
 105	  uStack_cc = *(undefined4 *)((longlong)param_4 + 0x1c);
 106	  uVar7 = *param_4;
 107	  puVar8 = (undefined4 *)param_4[2];
 108	  uStack_d0 = *(undefined4 *)((longlong)param_3 + 0x1c);
 109	  lVar4 = *param_3;
 110	  puVar9 = (undefined4 *)param_3[2];
 111	  if (plStack_130 == (longlong *)0x0) goto code_r0x000142269b10;
 112	code_r0x000142269af0:
 113	  if (plStack_130 == plVar1) goto code_r0x000142269b10;
 114	  plVar26 = plStack_130;
 115	  plStack_130 = plStack_130 + 3;
 116	  do {
 117	    *puVar3 = *puVar3 + 1;
 118	    *puVar6 = uStack_d4;
 119	    plStack_b8 = (longlong *)func_0x000140a38600(uVar5);
 120	    plStack_b0 = plStack_b8;
 121	    fVar32 = (float)func_0x000140caf380(&plStack_b8);
 122	    plVar25 = (longlong *)((longlong)plStack_b8 + 0xa0761d6478bd642f);
 123	    auVar11._8_8_ = 0;
 124	    auVar11._0_8_ = (ulonglong)plVar25 ^ 0xe7037ed1a0b428db;
 125	    auVar13._8_8_ = 0;
 126	    auVar13._0_8_ = plVar25;
 127	    uVar28 = SUB168(auVar11 * auVar13,8) ^ SUB168(auVar11 * auVar13,0);
 128	    if ((uint)((int)uVar28 * 10) < 6) {
 129	      uVar30 = (longlong)plStack_b8 + 0x40ec3ac8f17ac85e;
 130	      do {
 131	        auVar12._8_8_ = 0;
 132	        auVar12._0_8_ = uVar30 ^ 0xe7037ed1a0b428db;
 133	        auVar14._8_8_ = 0;
 134	        auVar14._0_8_ = uVar30;
 135	        plVar25 = (longlong *)((longlong)plVar25 + 0xa0761d6478bd642f);
 136	        uVar28 = SUB168(auVar12 * auVar14,8) ^ SUB168(auVar12 * auVar14,0);
 137	        uVar30 = uVar30 + 0xa0761d6478bd642f;
 138	      } while ((uint)((int)uVar28 * 10) < 6);
 139	    }
 140	    plStack_b8 = plVar25;
 141	    lVar16 = plVar26[2];
 142	    plVar26 = (longlong *)plVar26[1];
 143	    fVar33 = (float)func_0x000140caf380(&plStack_b8);
 144	    plVar25 = param_5;
 145	    uStack_118 = (longlong *)FUN_14226a170(plStack_b0);
 146	    uStack_110 = plVar25;
 147	    uVar22 = func_0x000141556130(&uStack_118);
 148	    if ((longlong)plVar25 < 0) {
 149	      uVar29 = 0;
 150	      plVar25 = plStack_120;
 151	code_r0x000142269f24:
 152	      plStack_120 = plVar25;
 153	      FUN_1428d8fe3(uVar29,plStack_120,&UNK_1430350f0);
 154	code_r0x000142269f37:
 155	                    /* WARNING: Does not return */
 156	      pcVar15 = (code *)invalidInstructionException();
 157	      (*pcVar15)();
 158	    }
 159	    if (plVar25 == (longlong *)0x0) {
 160	      plVar23 = (longlong *)0x1;
 161	    }
 162	    else {
 163	      uVar29 = 1;
 164	      plVar23 = (longlong *)func_0x000140613c10(plVar25,1);
 165	      if (plVar23 == (longlong *)0x0) goto code_r0x000142269f24;
 166	    }
 167	    func_0x000142923c00(plVar23,uVar22,plVar25);
 168	    uStack_144 = CONCAT44(fVar33 * 360.0 + (1.0 - fVar33) * 0.0,fVar32 * 0.9 + (1.0 - fVar32) * 0.5)
 169	    ;
 170	    uStack_13c = (undefined4)((uVar28 & 0xffffffff) * 10 >> 0x20);
 171	    uStack_138 = 1;
 172	    LOCK();
 173	    *uStack_118 = *uStack_118 + -1;
 174	    UNLOCK();
 175	    plStack_168 = plVar25;
 176	    plStack_160 = plVar23;
 177	    plStack_158 = plVar25;
 178	    plStack_150 = plVar26;
 179	    uStack_148 = (int)lVar16;
 180	    plStack_120 = plVar23;
 181	    if (*uStack_118 == 0) {
 182	      uStack_a1 = 1;
 183	      func_0x00014064eb30(&uStack_118);
 184	    }
 185	    uVar17 = uStack_13c;
 186	    uVar22 = uStack_144;
 187	    uStack_f8 = uStack_148;
 188	    uStack_100 = plStack_150;
 189	    uStack_a1 = 1;
 190	    FUN_140d7bd90(&uStack_118,&plStack_168);
 191	    uVar18 = uStack_f8;
 192	    plVar26 = uStack_100;
 193	    uStack_f4 = (undefined4)uVar22;
 194	    uVar19 = uStack_f4;
 195	    uStack_f0 = (undefined4)((ulonglong)uVar22 >> 0x20);
 196	    uVar20 = uStack_f0;
 197	    uStack_ec = uVar17;
 198	    uStack_e8 = uStack_138;
 199	    plStack_c8 = uStack_118;
 200	    plStack_128 = uStack_110;
 201	    uStack_198 = CONCAT44(uStack_104,uStack_108);
 202	    uVar29 = CONCAT71(uStack_e7,uStack_138);
 203	    *puVar9 = uStack_d0;
 204	    lVar16 = *(longlong *)(lVar4 + 0x30);
 205	    uVar22 = *(undefined8 *)(lVar4 + 0x40);
 206	    if (lVar16 == *(longlong *)(lVar4 + 0x20)) {
 207	      FUN_140b5f9f0(lVar4 + 0x20,&UNK_143034d10);
 208	    }
 209	    plVar21 = plStack_b0;
 210	    plVar23 = plStack_160;
 211	    plVar25 = plStack_168;
 212	    lVar10 = *(longlong *)(lVar4 + 0x28);
 213	    *(longlong **)(lVar10 + lVar16 * 0x48) = plStack_c8;
 214	    *(longlong **)(lVar10 + 8 + lVar16 * 0x48) = plStack_128;
 215	    puVar24 = (undefined8 *)(lVar10 + 0x10 + lVar16 * 0x48);
 216	    *puVar24 = uStack_198;
 217	    puVar24[1] = plVar26;
 218	    puVar24 = (undefined8 *)(lVar10 + 0x20 + lVar16 * 0x48);
 219	    *puVar24 = CONCAT44(uVar19,uVar18);
 220	    puVar24[1] = CONCAT44(uVar17,uVar20);
 221	    *(undefined8 *)(lVar10 + 0x30 + lVar16 * 0x48) = uVar29;
 222	    *(longlong **)(lVar10 + 0x38 + lVar16 * 0x48) = plStack_b0;
 223	    *(undefined8 *)(lVar10 + 0x40 + lVar16 * 0x48) = uVar22;
 224	    *(longlong *)(lVar4 + 0x30) = lVar16 + 1;
 225	    *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
 226	    *puVar8 = uStack_cc;
 227	    uStack_118 = plStack_158;
 228	    uStack_110 = plStack_150;
 229	    uStack_108 = uStack_148;
 230	    uStack_104 = (undefined4)uStack_144;
 231	    uStack_100 = (longlong *)CONCAT44(uStack_13c,uStack_144._4_4_);
 232	    uStack_f8 = (undefined4)CONCAT71(uStack_137,uStack_138);
 233	    uStack_f4 = (undefined4)((uint7)uStack_137 >> 0x18);
 234	    puVar24 = (undefined8 *)func_0x000140613c10(0x58,8);
 235	    if (puVar24 == (undefined8 *)0x0) {
 236	      plStack_c0 = plVar23;
 237	      plStack_b0 = plVar25;
 238	      FUN_1428d9000(8,0x58);
 239	      goto code_r0x000142269f37;
 240	    }
 241	    *puVar24 = 1;
 242	    puVar24[1] = 1;
 243	    *(undefined4 *)(puVar24 + 2) = 0;
 244	    puVar24[3] = plVar21;
 245	    puVar24[4] = plVar25;
 246	    puVar24[5] = plVar23;
 247	    puVar24[10] = CONCAT44(uStack_f4,uStack_f8);
 248	    *(undefined4 *)(puVar24 + 8) = uStack_108;
 249	    *(undefined4 *)((longlong)puVar24 + 0x44) = uStack_104;
 250	    *(undefined4 *)(puVar24 + 9) = (undefined4)uStack_100;
 251	    *(undefined4 *)((longlong)puVar24 + 0x4c) = uStack_100._4_4_;
 252	    *(undefined4 *)(puVar24 + 6) = (undefined4)uStack_118;
 253	    *(undefined4 *)((longlong)puVar24 + 0x34) = uStack_118._4_4_;
 254	    *(undefined4 *)(puVar24 + 7) = (undefined4)uStack_110;
 255	    *(undefined4 *)((longlong)puVar24 + 0x3c) = uStack_110._4_4_;
 256	    uStack_118 = (longlong *)0xc;
 257	    uStack_a1 = 0;
 258	    uStack_110 = puVar24;
 259	    FUN_140a84ec0(uVar7,&uStack_118,&UNK_1430351b8);
 260	    if (plStack_130 != (longlong *)0x0) goto code_r0x000142269af0;
 261	code_r0x000142269b10:
 262	    if ((plStack_c0 == (longlong *)0x0) || (plStack_c0 == plVar2)) {
 263	      return;
 264	    }
 265	    plStack_130 = (longlong *)0x0;
 266	    plVar26 = plStack_c0;
 267	    plStack_c0 = plStack_c0 + 3;
 268	  } while( true );
 269	}
 270	