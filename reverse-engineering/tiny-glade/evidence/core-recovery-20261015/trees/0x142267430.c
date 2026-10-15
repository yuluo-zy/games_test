   1	
   2	void FUN_142267430(undefined8 *param_1,longlong *param_2,undefined8 *param_3,undefined8 *param_4,
   3	                  undefined8 *param_5,longlong *param_6,undefined8 *param_7,longlong *param_8,
   4	                  ulonglong *param_9,undefined8 *param_10,longlong *param_11)
   5	
   6	{
   7	  undefined4 *puVar1;
   8	  undefined8 uVar2;
   9	  byte bVar3;
  10	  char cVar4;
  11	  uint uVar5;
  12	  longlong lVar6;
  13	  undefined8 *puVar7;
  14	  ulonglong *puVar8;
  15	  undefined8 *puVar9;
  16	  longlong lVar10;
  17	  ulonglong *puVar11;
  18	  undefined8 uVar12;
  19	  ulonglong uVar13;
  20	  ulonglong uVar14;
  21	  ulonglong uVar15;
  22	  undefined *puVar16;
  23	  ulonglong uVar17;
  24	  longlong lVar18;
  25	  longlong lVar19;
  26	  byte bVar20;
  27	  undefined4 uVar21;
  28	  longlong lVar22;
  29	  char *pcVar23;
  30	  undefined1 uVar24;
  31	  longlong lVar25;
  32	  ulonglong *puVar26;
  33	  int iVar27;
  34	  ulonglong *puVar28;
  35	  longlong *plVar29;
  36	  longlong lVar30;
  37	  longlong lVar31;
  38	  longlong *plVar32;
  39	  bool bVar33;
  40	  char cVar40;
  41	  char cVar41;
  42	  undefined4 uVar34;
  43	  float fVar35;
  44	  float fVar36;
  45	  char cVar43;
  46	  char cVar44;
  47	  char cVar45;
  48	  char cVar46;
  49	  char cVar47;
  50	  char cVar48;
  51	  char cVar49;
  52	  char cVar50;
  53	  char cVar51;
  54	  char cVar52;
  55	  char cVar53;
  56	  undefined1 auVar37 [16];
  57	  char cVar42;
  58	  char cVar54;
  59	  undefined1 auVar38 [16];
  60	  undefined1 auVar39 [16];
  61	  undefined1 auVar55 [16];
  62	  undefined1 auVar56 [16];
  63	  undefined1 auVar57 [16];
  64	  float fVar58;
  65	  ulonglong uStack_268;
  66	  undefined8 uStack_260;
  67	  undefined8 uStack_258;
  68	  ulonglong *puStack_250;
  69	  undefined4 *puStack_248;
  70	  longlong lStack_240;
  71	  ulonglong *puStack_238;
  72	  ulonglong *puStack_230;
  73	  longlong lStack_228;
  74	  undefined4 *puStack_220;
  75	  longlong lStack_218;
  76	  ulonglong uStack_210;
  77	  undefined8 *puStack_208;
  78	  ulonglong *puStack_200;
  79	  code *pcStack_1f8;
  80	  longlong lStack_1f0;
  81	  longlong lStack_1e8;
  82	  undefined4 uStack_1dc;
  83	  undefined4 uStack_1d8;
  84	  undefined4 uStack_1d4;
  85	  undefined8 uStack_1d0;
  86	  undefined4 *puStack_1c8;
  87	  ulonglong *puStack_1c0;
  88	  undefined4 *puStack_1b8;
  89	  longlong lStack_1b0;
  90	  longlong lStack_1a8;
  91	  undefined8 uStack_1a0;
  92	  longlong *plStack_198;
  93	  longlong *plStack_190;
  94	  ulonglong uStack_188;
  95	  undefined8 uStack_180;
  96	  undefined8 uStack_178;
  97	  undefined4 uStack_170;
  98	  undefined4 uStack_16c;
  99	  undefined4 uStack_168;
 100	  undefined4 uStack_164;
 101	  undefined4 uStack_160;
 102	  int iStack_15c;
 103	  undefined8 uStack_158;
 104	  undefined *puStack_150;
 105	  undefined8 uStack_148;
 106	  ulonglong **ppuStack_140;
 107	  undefined8 uStack_138;
 108	  undefined8 uStack_130;
 109	  undefined8 uStack_120;
 110	  ulonglong *puStack_118;
 111	  longlong lStack_110;
 112	  longlong lStack_108;
 113	  undefined4 uStack_fc;
 114	  undefined4 uStack_f8;
 115	  undefined1 uStack_f3;
 116	  byte bStack_f2;
 117	  byte bStack_f1;
 118	  undefined8 uStack_f0;
 119	  
 120	  uStack_f0 = 0xfffffffffffffffe;
 121	  puStack_208 = param_4;
 122	  lVar6 = func_0x00014067dd80(*param_10);
 123	  puVar9 = (undefined8 *)(lVar6 + 0x20);
 124	  puVar7 = (undefined8 *)(lVar6 + 0x28);
 125	  if (*(char *)(lVar6 + 0x298) == '\0') {
 126	    puVar7 = (undefined8 *)(lVar6 + 0x10);
 127	    puVar9 = (undefined8 *)(lVar6 + 8);
 128	  }
 129	  uStack_258 = *puVar7;
 130	  uStack_260 = *puVar9;
 131	  puVar26 = (ulonglong *)*param_3;
 132	  lVar6 = param_3[1];
 133	  uVar13 = *puVar26;
 134	  uVar15 = 0;
 135	  uVar14 = uVar13 - *(ulonglong *)(lVar6 + 0x18);
 136	  if (uVar13 < *(ulonglong *)(lVar6 + 0x18)) {
 137	    uVar14 = uVar15;
 138	  }
 139	  uVar17 = uVar13 - *(ulonglong *)(lVar6 + 0x38);
 140	  if (uVar13 < *(ulonglong *)(lVar6 + 0x38)) {
 141	    uVar17 = uVar15;
 142	  }
 143	  lVar30 = uVar14 * 0x20 + *(longlong *)(lVar6 + 8);
 144	  uVar13 = *(ulonglong *)(lVar6 + 0x10) - uVar14;
 145	  if (*(ulonglong *)(lVar6 + 0x10) < uVar14) {
 146	    lVar30 = 8;
 147	    uVar13 = uVar15;
 148	  }
 149	  lVar22 = uVar17 * 0x20 + *(longlong *)(lVar6 + 0x28);
 150	  uVar14 = *(ulonglong *)(lVar6 + 0x30) - uVar17;
 151	  if (*(ulonglong *)(lVar6 + 0x30) < uVar17) {
 152	    lVar22 = 8;
 153	    uVar14 = uVar15;
 154	  }
 155	  *puVar26 = (*(longlong *)(lVar6 + 0x40) - uVar14) - uVar13;
 156	  uStack_fc = *(undefined4 *)((longlong)param_1 + 0x1c);
 157	  uStack_1d0 = *param_1;
 158	  puStack_1c8 = (undefined4 *)param_1[2];
 159	  lStack_1e8 = *param_2;
 160	  lStack_1a8 = param_2[1];
 161	  lStack_108 = CONCAT44(lStack_108._4_4_,*(undefined4 *)((longlong)param_2 + 0x14));
 162	  uStack_f8 = *(undefined4 *)((longlong)param_8 + 0x1c);
 163	  lStack_110 = *param_8;
 164	  puStack_1b8 = (undefined4 *)param_8[2];
 165	  lStack_1b0 = lStack_110 + 0x20;
 166	  lVar19 = lVar22;
 167	  lVar6 = lVar30;
 168	  puVar11 = puRam000000014362c3a8;
 169	  do {
 170	    if ((lVar6 == 0) || (lVar6 == uVar13 * 0x20 + lVar30)) {
 171	      if ((lVar19 == 0) || (lVar19 == uVar14 * 0x20 + lVar22)) break;
 172	      lVar18 = lVar19 + 0x20;
 173	      lVar31 = 0;
 174	      lVar6 = lVar19;
 175	    }
 176	    else {
 177	      lVar31 = lVar6 + 0x20;
 178	      lVar18 = lVar19;
 179	    }
 180	    *puVar26 = *puVar26 + 1;
 181	    uVar12 = *(undefined8 *)(lVar6 + 0x10);
 182	    uVar21 = *(undefined4 *)(lVar6 + 0x18);
 183	    uStack_268 = *(ulonglong *)(lVar6 + 8);
 184	    *puStack_1c8 = uStack_fc;
 185	    puVar8 = (ulonglong *)FUN_140a38460(uStack_1d0);
 186	    lVar6 = lVar31;
 187	    lVar19 = lVar18;
 188	    if (puVar8 == (ulonglong *)0x0) {
 189	      if (1 < *puVar11) {
 190	        puStack_200 = &uStack_268;
 191	        pcStack_1f8 = FUN_1422673f0;
 192	        puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_143035058);
 193	        uStack_180 = (longlong *)*puVar9;
 194	        uStack_178 = puVar9[1];
 195	        uStack_170 = 2;
 196	        uStack_16c = 0;
 197	        uStack_168 = 0x43034fe0;
 198	        uStack_164 = 1;
 199	        uStack_160 = 0x1e;
 200	        iStack_15c = 0;
 201	        puStack_150 = &UNK_143035048;
 202	        uStack_148 = 1;
 203	        ppuStack_140 = &puStack_200;
 204	        uStack_138 = 1;
 205	        uStack_130 = 0;
 206	        uStack_1a0 = 0;
 207	        plStack_198 = (longlong *)&UNK_143034fe0;
 208	        plStack_190 = (longlong *)0x1e;
 209	        uStack_188 = 0;
 210	        uStack_158 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
 211	        func_0x000140d4a330(&uStack_f3,&uStack_1a0);
 212	      }
 213	    }
 214	    else {
 215	      uVar15 = *puVar8;
 216	      uVar17 = uVar15 & 0xffffffff;
 217	      if ((*(ulonglong *)(lStack_1a8 + 0x10) <= uVar17) ||
 218	         (*(int *)(*(longlong *)(lStack_1a8 + 8) + uVar17 * 0x14) != (int)(uVar15 >> 0x20))) {
 219	code_r0x0001422685b1:
 220	        uStack_1a0 = ((ulonglong)puStack_118 & 0xffffffff00000000) + 1;
 221	        do {
 222	          puVar16 = &UNK_143035000;
 223	          plStack_198 = (longlong *)uVar15;
 224	code_r0x000142268583:
 225	          puVar9 = &uStack_1a0;
 226	          uVar15 = FUN_1428d9760(&UNK_143034db0,0x2b,puVar9,&UNK_143034d90,puVar16);
 227	code_r0x0001422685a4:
 228	          uStack_1a0 = (longlong)puVar9 << 0x20;
 229	        } while( true );
 230	      }
 231	      lVar31 = *(longlong *)(lStack_1a8 + 8) + uVar17 * 0x14;
 232	      puVar9 = (undefined8 *)(ulonglong)*(uint *)(lVar31 + 4);
 233	      if (puVar9 == (undefined8 *)0xffffffff) goto code_r0x0001422685b1;
 234	      if (*(undefined8 **)(lStack_1e8 + 0x38) <= puVar9) goto code_r0x0001422685a4;
 235	      if ((*(ulonglong *)
 236	            (*(longlong *)(lStack_1e8 + 0x28) + (ulonglong)(*(uint *)(lVar31 + 4) >> 6) * 8) >>
 237	           ((ulonglong)puVar9 & 0x3f) & 1) == 0) goto code_r0x0001422685a4;
 238	      uVar15 = (ulonglong)*(uint *)(lVar31 + 0x10);
 239	      lVar18 = *(longlong *)
 240	                (*(longlong *)(lStack_1a8 + 0x1a8) + 0x18 +
 241	                (ulonglong)*(uint *)(lVar31 + 0xc) * 0x48);
 242	      lVar10 = ~*(ulonglong *)
 243	                 (*(longlong *)
 244	                   (*(longlong *)(lStack_1a8 + 0x1a8) + 0x38 +
 245	                   (ulonglong)*(uint *)(lVar31 + 0xc) * 0x48) +
 246	                 *(longlong *)(lStack_1e8 + 0x110) * 8) * 0x30;
 247	      lVar31 = *(longlong *)(lVar18 + 0x10 + lVar10);
 248	      lVar25 = uVar15 * 0x30;
 249	      puStack_118 = (ulonglong *)(uVar15 * 4 + *(longlong *)(lVar18 + 0x20 + lVar10));
 250	      *(undefined4 *)(*(longlong *)(lVar18 + 0x28 + lVar10) + uVar15 * 4) = (undefined4)lStack_108;
 251	      *(undefined8 *)(lVar31 + lVar25) = uVar12;
 252	      *(undefined4 *)(lVar31 + 8 + lVar25) = uVar21;
 253	      uVar34 = func_0x000140c904a0(uVar15,lVar31,*(undefined4 *)(lVar31 + 0xc + lVar25));
 254	      fVar35 = (float)FUN_140a14df0(&uStack_260);
 255	      *puStack_1b8 = uStack_f8;
 256	      lVar31 = *(longlong *)(lStack_110 + 0x30);
 257	      uVar2 = *(undefined8 *)(lStack_110 + 0x40);
 258	      uVar24 = 0.0 < fVar35;
 259	      if (lVar31 == *(longlong *)(lStack_110 + 0x20)) {
 260	        puStack_1c0 = (ulonglong *)CONCAT71(puStack_1c0._1_7_,0.0 < fVar35);
 261	        FUN_140562c30(lStack_1b0,&UNK_143034d10);
 262	        uVar24 = puStack_1c0._0_1_;
 263	      }
 264	      lVar18 = *(longlong *)(lStack_110 + 0x28);
 265	      lVar10 = lVar31 * 0x20;
 266	      *(undefined8 *)(lVar18 + lVar10) = uVar2;
 267	      *(undefined1 *)(lVar18 + 8 + lVar10) = 1;
 268	      *(undefined1 *)(lVar18 + 9 + lVar10) = uVar24;
 269	      *(undefined4 *)(lVar18 + 0xc + lVar10) = uVar34;
 270	      *(undefined8 *)(lVar18 + 0x10 + lVar10) = uVar12;
 271	      *(undefined4 *)(lVar18 + 0x18 + lVar10) = uVar21;
 272	      *(longlong *)(lStack_110 + 0x30) = lVar31 + 1;
 273	      *(longlong *)(lStack_110 + 0x40) = *(longlong *)(lStack_110 + 0x40) + 1;
 274	      puVar11 = puRam000000014362c3a8;
 275	    }
 276	  } while( true );
 277	  puStack_250 = (ulonglong *)*param_5;
 278	  lVar6 = param_5[1];
 279	  uVar13 = *puStack_250;
 280	  uVar15 = 0;
 281	  uVar14 = uVar13 - *(ulonglong *)(lVar6 + 0x18);
 282	  if (uVar13 < *(ulonglong *)(lVar6 + 0x18)) {
 283	    uVar14 = uVar15;
 284	  }
 285	  uVar17 = uVar13 - *(ulonglong *)(lVar6 + 0x38);
 286	  if (uVar13 < *(ulonglong *)(lVar6 + 0x38)) {
 287	    uVar17 = uVar15;
 288	  }
 289	  puVar26 = (ulonglong *)(uVar14 * 0x48 + *(longlong *)(lVar6 + 8));
 290	  uVar13 = *(ulonglong *)(lVar6 + 0x10) - uVar14;
 291	  if (*(ulonglong *)(lVar6 + 0x10) < uVar14) {
 292	    puVar26 = (ulonglong *)0x8;
 293	    uVar13 = uVar15;
 294	  }
 295	  puVar11 = (ulonglong *)(uVar17 * 0x48 + *(longlong *)(lVar6 + 0x28));
 296	  uVar14 = *(ulonglong *)(lVar6 + 0x30) - uVar17;
 297	  if (*(ulonglong *)(lVar6 + 0x30) < uVar17) {
 298	    puVar11 = (ulonglong *)0x8;
 299	    uVar14 = uVar15;
 300	  }
 301	  *puStack_250 = (*(longlong *)(lVar6 + 0x40) - uVar14) - uVar13;
 302	  puStack_238 = puVar26 + uVar13 * 9;
 303	  puStack_230 = puVar11 + uVar14 * 9;
 304	  uStack_210 = *param_9;
 305	  uStack_1d8 = *(undefined4 *)((longlong)param_6 + 0x1c);
 306	  lStack_108 = *param_6;
 307	  puStack_248 = (undefined4 *)param_6[2];
 308	  uStack_1d4 = *(undefined4 *)((longlong)param_11 + 0x1c);
 309	  lStack_1f0 = *param_11;
 310	  puStack_220 = (undefined4 *)param_11[2];
 311	  lVar6 = uStack_210 + 0x58;
 312	  lStack_228 = lStack_108 + 0x20;
 313	  lVar30 = lStack_1f0 + 0x20;
 314	  lStack_240 = lVar6;
 315	  lStack_218 = lVar30;
 316	  do {
 317	    if ((puVar26 == (ulonglong *)0x0) || (puVar26 == puStack_238)) {
 318	      if ((puVar11 == (ulonglong *)0x0) || (puVar11 == puStack_230)) break;
 319	      puVar8 = puVar11 + 9;
 320	      puVar28 = (ulonglong *)0x0;
 321	    }
 322	    else {
 323	      puVar28 = puVar26 + 9;
 324	      puVar8 = puVar11;
 325	      puVar11 = puVar26;
 326	    }
 327	    *puStack_250 = *puStack_250 + 1;
 328	    fVar35 = (float)func_0x000140c904a0(lVar30,0x3f666666,*(undefined4 *)((longlong)puVar11 + 0x24))
 329	    ;
 330	    uVar12 = func_0x00014092e6a0(lVar6);
 331	    if ((char)uVar12 == '\0') {
 332	      uStack_120 = (ulonglong)uStack_120._4_4_ << 0x20;
 333	      pcVar23 = (char *)puVar11[1];
 334	      uVar13 = puVar11[2];
 335	      if (uVar13 == 0x19) goto code_r0x000142267b4a;
 336	code_r0x000142267ad2:
 337	      if (uVar13 == 0x27) {
 338	        iVar27 = func_0x000142923bf0(pcVar23,&UNK_143034f28,0x27);
 339	        if (iVar27 != 0) {
 340	          iVar27 = func_0x000142923bf0(pcVar23,&UNK_143034f4f,0x27);
 341	          bVar33 = iVar27 == 0;
 342	          goto code_r0x000142267ba0;
 343	        }
 344	code_r0x000142267bb4:
 345	        uVar21 = (undefined4)CONCAT71((int7)((ulonglong)pcVar23 >> 8),1);
 346	      }
 347	      else {
 348	        if (uVar13 == 0x1a) {
 349	          auVar55[0] = -(pcVar23[10] == 'a');
 350	          auVar55[1] = -(pcVar23[0xb] == 'k');
 351	          auVar55[2] = -(pcVar23[0xc] == 'e');
 352	          auVar55[3] = -(pcVar23[0xd] == 'd');
 353	          auVar55[4] = -(pcVar23[0xe] == '_');
 354	          auVar55[5] = -(pcVar23[0xf] == 't');
 355	          auVar55[6] = -(pcVar23[0x10] == 'r');
 356	          auVar55[7] = -(pcVar23[0x11] == 'e');
 357	          auVar55[8] = -(pcVar23[0x12] == 'e');
 358	          auVar55[9] = -(pcVar23[0x13] == '/');
 359	          auVar55[10] = -(pcVar23[0x14] == 'n');
 360	          auVar55[0xb] = -(pcVar23[0x15] == 'a');
 361	          auVar55[0xc] = -(pcVar23[0x16] == 'k');
 362	          auVar55[0xd] = -(pcVar23[0x17] == 'e');
 363	          auVar55[0xe] = -(pcVar23[0x18] == 'd');
 364	          auVar55[0xf] = -(pcVar23[0x19] == '1');
 365	          auVar37[0] = -(*pcVar23 == 'c');
 366	          auVar37[1] = -(pcVar23[1] == 'l');
 367	          auVar37[2] = -(pcVar23[2] == 'e');
 368	          auVar37[3] = -(pcVar23[3] == 'a');
 369	          auVar37[4] = -(pcVar23[4] == 'r');
 370	          auVar37[5] = -(pcVar23[5] == 'i');
 371	          auVar37[6] = -(pcVar23[6] == 'n');
 372	          auVar37[7] = -(pcVar23[7] == 'g');
 373	          auVar37[8] = -(pcVar23[8] == '/');
 374	          auVar37[9] = -(pcVar23[9] == 'n');
 375	          auVar37[10] = -(pcVar23[10] == 'a');
 376	          auVar37[0xb] = -(pcVar23[0xb] == 'k');
 377	          auVar37[0xc] = -(pcVar23[0xc] == 'e');
 378	          auVar37[0xd] = -(pcVar23[0xd] == 'd');
 379	          auVar37[0xe] = -(pcVar23[0xe] == '_');
 380	          auVar37[0xf] = -(pcVar23[0xf] == 't');
 381	          auVar37 = auVar37 & auVar55;
 382	          if ((ushort)((ushort)(SUB161(auVar37 >> 7,0) & 1) |
 383	                       (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
 384	                       (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
 385	                       (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
 386	                       (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
 387	                       (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
 388	                       (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
 389	                       (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
 390	                       (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
 391	                       (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
 392	                       (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
 393	                       (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
 394	                       (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
 395	                       (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
 396	                       (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
 397	                      (ushort)(byte)(auVar37[0xf] >> 7) << 0xf) == 0xffff)
 398	          goto code_r0x000142267bb4;
 399	          cVar4 = *pcVar23;
 400	          cVar40 = pcVar23[1];
 401	          cVar41 = pcVar23[2];
 402	          cVar42 = pcVar23[3];
 403	          cVar43 = pcVar23[4];
 404	          cVar44 = pcVar23[5];
 405	          cVar45 = pcVar23[6];
 406	          cVar46 = pcVar23[7];
 407	          cVar47 = pcVar23[8];
 408	          cVar48 = pcVar23[9];
 409	          cVar49 = pcVar23[10];
 410	          cVar50 = pcVar23[0xb];
 411	          cVar51 = pcVar23[0xc];
 412	          cVar52 = pcVar23[0xd];
 413	          cVar53 = pcVar23[0xe];
 414	          cVar54 = pcVar23[0xf];
 415	          auVar56[0] = -(pcVar23[10] == 'a');
 416	          auVar56[1] = -(pcVar23[0xb] == 'k');
 417	          auVar56[2] = -(pcVar23[0xc] == 'e');
 418	          auVar56[3] = -(pcVar23[0xd] == 'd');
 419	          auVar56[4] = -(pcVar23[0xe] == '_');
 420	          auVar56[5] = -(pcVar23[0xf] == 't');
 421	          auVar56[6] = -(pcVar23[0x10] == 'r');
 422	          auVar56[7] = -(pcVar23[0x11] == 'e');
 423	          auVar56[8] = -(pcVar23[0x12] == 'e');
 424	          auVar56[9] = -(pcVar23[0x13] == '/');
 425	          auVar56[10] = -(pcVar23[0x14] == 'n');
 426	          auVar56[0xb] = -(pcVar23[0x15] == 'a');
 427	          auVar56[0xc] = -(pcVar23[0x16] == 'k');
 428	          auVar56[0xd] = -(pcVar23[0x17] == 'e');
 429	          auVar56[0xe] = -(pcVar23[0x18] == 'd');
 430	          auVar56[0xf] = -(pcVar23[0x19] == '2');
 431	          goto code_r0x000142267b5b;
 432	        }
 433	        uVar21 = 0;
 434	      }
 435	    }
 436	    else {
 437	      uStack_120 = CONCAT44(uStack_120._4_4_,
 438	                            (int)CONCAT71((int7)((ulonglong)uVar12 >> 8),
 439	                                          (*(byte *)((longlong)puVar11 + 0x2c) & 3) != 0));
 440	      pcVar23 = (char *)puVar11[1];
 441	      uVar13 = puVar11[2];
 442	      if (uVar13 != 0x19) goto code_r0x000142267ad2;
 443	code_r0x000142267b4a:
 444	      cVar4 = *pcVar23;
 445	      cVar40 = pcVar23[1];
 446	      cVar41 = pcVar23[2];
 447	      cVar42 = pcVar23[3];
 448	      cVar43 = pcVar23[4];
 449	      cVar44 = pcVar23[5];
 450	      cVar45 = pcVar23[6];
 451	      cVar46 = pcVar23[7];
 452	      cVar47 = pcVar23[8];
 453	      cVar48 = pcVar23[9];
 454	      cVar49 = pcVar23[10];
 455	      cVar50 = pcVar23[0xb];
 456	      cVar51 = pcVar23[0xc];
 457	      cVar52 = pcVar23[0xd];
 458	      cVar53 = pcVar23[0xe];
 459	      cVar54 = pcVar23[0xf];
 460	      auVar56[0] = -(pcVar23[9] == 'n');
 461	      auVar56[1] = -(pcVar23[10] == 'a');
 462	      auVar56[2] = -(pcVar23[0xb] == 'k');
 463	      auVar56[3] = -(pcVar23[0xc] == 'e');
 464	      auVar56[4] = -(pcVar23[0xd] == 'd');
 465	      auVar56[5] = -(pcVar23[0xe] == '_');
 466	      auVar56[6] = -(pcVar23[0xf] == 't');
 467	      auVar56[7] = -(pcVar23[0x10] == 'r');
 468	      auVar56[8] = -(pcVar23[0x11] == 'e');
 469	      auVar56[9] = -(pcVar23[0x12] == 'e');
 470	      auVar56[10] = -(pcVar23[0x13] == '/');
 471	      auVar56[0xb] = -(pcVar23[0x14] == 'l');
 472	      auVar56[0xc] = -(pcVar23[0x15] == 'a');
 473	      auVar56[0xd] = -(pcVar23[0x16] == 'r');
 474	      auVar56[0xe] = -(pcVar23[0x17] == 'g');
 475	      auVar56[0xf] = -(pcVar23[0x18] == 'e');
 476	code_r0x000142267b5b:
 477	      auVar38[0] = -(cVar4 == 'c');
 478	      auVar38[1] = -(cVar40 == 'l');
 479	      auVar38[2] = -(cVar41 == 'e');
 480	      auVar38[3] = -(cVar42 == 'a');
 481	      auVar38[4] = -(cVar43 == 'r');
 482	      auVar38[5] = -(cVar44 == 'i');
 483	      auVar38[6] = -(cVar45 == 'n');
 484	      auVar38[7] = -(cVar46 == 'g');
 485	      auVar38[8] = -(cVar47 == '/');
 486	      auVar38[9] = -(cVar48 == 'n');
 487	      auVar38[10] = -(cVar49 == 'a');
 488	      auVar38[0xb] = -(cVar50 == 'k');
 489	      auVar38[0xc] = -(cVar51 == 'e');
 490	      auVar38[0xd] = -(cVar52 == 'd');
 491	      auVar38[0xe] = -(cVar53 == '_');
 492	      auVar38[0xf] = -(cVar54 == 't');
 493	      auVar38 = auVar38 & auVar56;
 494	      bVar33 = (ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
 495	                        (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
 496	                        (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
 497	                        (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
 498	                        (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
 499	                        (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
 500	                        (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
 501	                        (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
 502	                        (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
 503	                        (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
 504	                        (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
 505	                        (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
 506	                        (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
 507	                        (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
 508	                        (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
 509	                       (ushort)(byte)(auVar38[0xf] >> 7) << 0xf) == 0xffff;
 510	code_r0x000142267ba0:
 511	      uVar21 = (undefined4)CONCAT71((int7)((ulonglong)pcVar23 >> 8),bVar33);
 512	    }
 513	    puStack_1c0 = puVar28;
 514	    puStack_118 = puVar8;
 515	    bVar3 = func_0x000140a9fa70(lVar6);
 516	    bStack_f1 = func_0x000140ae8320(lVar6);
 517	    fVar36 = *(float *)((longlong)puVar11 + 0x24);
 518	    fVar58 = fVar36;
 519	    if (fVar36 <= 0.7) {
 520	      fVar58 = 0.7;
 521	    }
 522	    bVar20 = (byte)uVar21 & bVar3;
 523	    if (bVar20 == 0) {
 524	      fVar58 = fVar36;
 525	    }
 526	    uVar13 = puVar11[7];
 527	    uVar14 = puVar11[6];
 528	    uStack_1dc = uVar21;
 529	    bStack_f2 = bVar3;
 530	    FUN_140d146a0(&uStack_1a0,param_7);
 531	    plVar29 = uStack_180;
 532	    plVar32 = plStack_198;
 533	    if (plStack_198 == (longlong *)0x0) {
 534	      plVar32 = plStack_190;
 535	    }
 536	    lVar6 = plVar32[2];
 537	    if ((ulonglong)(*plVar32 - lVar6) < 0x30) {
 538	      FUN_1429105f0(plVar32,lVar6,0x30,1,1);
 539	    }
 540	    lVar30 = plVar32[1];
 541	    *(code **)(lVar30 + lVar6) = FUN_1425de680;
 542	    *(code **)(lVar30 + 8 + lVar6) = FUN_1425de2b0;
 543	    *(longlong **)(lVar30 + 0x10 + lVar6) = plVar29;
 544	    *(ulonglong *)(lVar30 + 0x18 + lVar6) = uVar13;
 545	    *(char *)(lVar30 + 0x20 + lVar6) = (char)uVar14;
 546	    *(undefined1 *)(lVar30 + 0x28 + lVar6) = 0;
 547	    lVar30 = lVar6 + 0x30;
 548	    plVar32[2] = lVar30;
 549	    if ((ulonglong)(*plVar32 - lVar30) < 0x30) {
 550	      FUN_1429105f0(plVar32,lVar30,0x30,1,1);
 551	    }
 552	    lVar22 = plVar32[1];
 553	    *(code **)(lVar22 + 0x30 + lVar6) = FUN_1425de580;
 554	    *(code **)(lVar22 + 0x38 + lVar6) = FUN_1425de2b0;
 555	    *(longlong **)(lVar22 + 0x40 + lVar6) = plVar29;
 556	    *(float *)(lVar22 + 0x48 + lVar6) = fVar35 * 0.75 + (1.0 - fVar35) * 0.55;
 557	    *(undefined1 *)(lVar22 + 0x24 + lVar30) = 0;
 558	    *(undefined8 *)(lVar22 + 0x1c + lVar30) = 0;
 559	    *(undefined1 *)(lVar22 + 0x58 + lVar6) = 0;
 560	    lVar30 = lVar6 + 0x60;
 561	    plVar32[2] = lVar30;
 562	    uVar13 = puVar11[3];
 563	    uVar14 = puVar11[4];
 564	    if ((ulonglong)(*plVar32 - lVar30) < 0x28) {
 565	      FUN_1429105f0(plVar32,lVar30,0x28,1,1);
 566	    }
 567	    lVar30 = plVar32[1];
 568	    *(code **)(lVar30 + 0x60 + lVar6) = FUN_1425de880;
 569	    *(code **)(lVar30 + 0x68 + lVar6) = FUN_1425de2b0;
 570	    *(longlong **)(lVar30 + 0x70 + lVar6) = plVar29;
 571	    *(int *)(lVar30 + 0x78 + lVar6) = (int)uVar13;
 572	    *(int *)(lVar30 + 0x7c + lVar6) = (int)uVar14;
 573	    *(undefined1 *)(lVar30 + 0x80 + lVar6) = 0;
 574	    lVar30 = lVar6 + 0x88;
 575	    plVar32[2] = lVar30;
 576	    uVar13 = puVar11[3];
 577	    fVar36 = *(float *)(puVar11 + 4);
 578	    if ((ulonglong)(*plVar32 - lVar30) < 0x28) {
 579	      FUN_1429105f0(plVar32,lVar30,0x28,1,1);
 580	    }
 581	    lVar30 = plVar32[1];
 582	    *(code **)(lVar30 + 0x88 + lVar6) = FUN_1425de490;
 583	    *(code **)(lVar30 + 0x90 + lVar6) = FUN_1425de2b0;
 584	    *(longlong **)(lVar30 + 0x98 + lVar6) = plVar29;
 585	    *(ulonglong *)(lVar30 + 0xa0 + lVar6) =
 586	         CONCAT44(fVar58 * 10.0 + (float)(uVar13 >> 0x20),fVar58 * 0.0 + (float)uVar13);
 587	    *(float *)(lVar30 + 0xa8 + lVar6) = fVar36 + fVar58 * 0.0;
 588	    *(undefined1 *)(lVar30 + 0xac + lVar6) = 0;
 589	    plVar32[2] = lVar6 + 0xb0;
 590	    uVar13 = puVar11[3];
 591	    uVar15 = puVar11[4];
 592	    uVar14 = puVar11[5];
 593	    iVar27 = 3;
 594	    if (bVar20 == 0) {
 595	      iVar27 = (byte)uStack_120 + 1;
 596	      if ((char)puVar11[6] != '\0') {
 597	        iVar27 = 0;
 598	      }
 599	    }
 600	    puVar8 = puVar11 + 7;
 601	    pcVar23 = (char *)puVar11[1];
 602	    uVar17 = puVar11[2];
 603	    if (uVar17 == 0x18) {
 604	      auVar39[0] = -(*pcVar23 == 'M');
 605	      auVar39[1] = -(pcVar23[1] == 'I');
 606	      auVar39[2] = -(pcVar23[2] == 'S');
 607	      auVar39[3] = -(pcVar23[3] == 'S');
 608	      auVar39[4] = -(pcVar23[4] == 'I');
 609	      auVar39[5] = -(pcVar23[5] == 'N');
 610	      auVar39[6] = -(pcVar23[6] == 'G');
 611	      auVar39[7] = -(pcVar23[7] == '_');
 612	      auVar39[8] = -(pcVar23[8] == 'T');
 613	      auVar39[9] = -(pcVar23[9] == 'R');
 614	      auVar39[10] = -(pcVar23[10] == 'E');
 615	      auVar39[0xb] = -(pcVar23[0xb] == 'E');
 616	      auVar39[0xc] = -(pcVar23[0xc] == '_');
 617	      auVar39[0xd] = -(pcVar23[0xd] == 'P');
 618	      auVar39[0xe] = -(pcVar23[0xe] == 'R');
 619	      auVar39[0xf] = -(pcVar23[0xf] == 'E');
 620	      uVar12 = *(undefined8 *)(pcVar23 + 0x10);
 621	      auVar57[0] = -((char)uVar12 == 'F');
 622	      auVar57[1] = -((char)((ulonglong)uVar12 >> 8) == 'A');
 623	      auVar57[2] = -((char)((ulonglong)uVar12 >> 0x10) == 'B');
 624	      auVar57[3] = -((char)((ulonglong)uVar12 >> 0x18) == '_');
 625	      auVar57[4] = -((char)((ulonglong)uVar12 >> 0x20) == 'N');
 626	      auVar57[5] = -((char)((ulonglong)uVar12 >> 0x28) == 'A');
 627	      auVar57[6] = -((char)((ulonglong)uVar12 >> 0x30) == 'M');
 628	      auVar57[7] = -((char)((ulonglong)uVar12 >> 0x38) == 'E');
 629	      auVar57[8] = 0xff;
 630	      auVar57[9] = 0xff;
 631	      auVar57[10] = 0xff;
 632	      auVar57[0xb] = 0xff;
 633	      auVar57[0xc] = 0xff;
 634	      auVar57[0xd] = 0xff;
 635	      auVar57[0xe] = 0xff;
 636	      auVar57[0xf] = 0xff;
 637	      auVar57 = auVar57 & auVar39;
 638	      if ((ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
 639	                   (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
 640	                   (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
 641	                   (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
 642	                   (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
 643	                   (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
 644	                   (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
 645	                   (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
 646	                   (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
 647	                   (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
 648	                   (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
 649	                   (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
 650	                   (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
 651	                   (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
 652	                   (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
 653	                  (ushort)(byte)(auVar57[0xf] >> 7) << 0xf) != 0xffff) goto code_r0x000142267e32;
 654	      uVar17 = uStack_210;
 655	      uStack_1a0 = FUN_14226a170(*puVar8);
 656	    }
 657	    else {
 658	code_r0x000142267e32:
 659	      uStack_1a0 = FUN_141556080();
 660	    }
 661	    *puStack_248 = uStack_1d8;
 662	    uStack_158 = *(undefined8 *)(lStack_108 + 0x40);
 663	    plStack_190 = plVar29;
 664	    uStack_180 = (longlong *)CONCAT44(fVar58,(int)uVar15);
 665	    uStack_170 = 0x3f800000;
 666	    uStack_16c = 0;
 667	    uStack_168 = 0;
 668	    uStack_164 = 0;
 669	    uStack_160 = 0;
 670	    lVar6 = *(longlong *)(lStack_108 + 0x30);
 671	    plStack_198 = (longlong *)uVar17;
 672	    uStack_188 = uVar13;
 673	    uStack_178 = uVar14;
 674	    iStack_15c = iVar27;
 675	    uStack_120 = uStack_1a0;
 676	    if (lVar6 == *(longlong *)(lStack_108 + 0x20)) {
 677	      FUN_140563540(lStack_228,&UNK_143034d10);
 678	    }
 679	    lVar30 = *(longlong *)(lStack_108 + 0x28);
 680	    lVar22 = lVar6 * 0x50;
 681	    puVar9 = (undefined8 *)(lVar30 + 0x40 + lVar22);
 682	    *puVar9 = CONCAT44(iStack_15c,uStack_160);
 683	    puVar9[1] = uStack_158;
 684	    puVar1 = (undefined4 *)(lVar30 + 0x30 + lVar22);
 685	    *puVar1 = uStack_170;
 686	    puVar1[1] = uStack_16c;
 687	    puVar1[2] = uStack_168;
 688	    puVar1[3] = uStack_164;
 689	    puVar1 = (undefined4 *)(lVar30 + 0x20 + lVar22);
 690	    *puVar1 = (undefined4)uStack_180;
 691	    puVar1[1] = uStack_180._4_4_;
 692	    puVar1[2] = (undefined4)uStack_178;
 693	    puVar1[3] = uStack_178._4_4_;
 694	    puVar9 = (undefined8 *)(lVar30 + 0x10 + lVar22);
 695	    *puVar9 = plStack_190;
 696	    puVar9[1] = uStack_188;
 697	    *(undefined8 *)(lVar30 + lVar22) = uStack_1a0;
 698	    ((undefined8 *)(lVar30 + lVar22))[1] = plStack_198;
 699	    *(longlong *)(lStack_108 + 0x30) = lVar6 + 1;
 700	    *(longlong *)(lStack_108 + 0x40) = *(longlong *)(lStack_108 + 0x40) + 1;
 701	    *puStack_1c8 = uStack_fc;
 702	    uVar13 = FUN_140a38560(uStack_1d0,puVar11[7],plVar29);
 703	    fVar36 = (float)FUN_140a14df0(&uStack_260);
 704	    uStack_1a0 = puVar11[3];
 705	    plStack_198 = (longlong *)CONCAT44(plStack_198._4_4_,(int)puVar11[4]);
 706	    *puStack_1b8 = uStack_f8;
 707	    lVar22 = *(longlong *)(lStack_110 + 0x30);
 708	    uVar12 = *(undefined8 *)(lStack_110 + 0x40);
 709	    if (lVar22 == *(longlong *)(lStack_110 + 0x20)) {
 710	      FUN_140562c30(lStack_1b0,&UNK_143034d10);
 711	    }
 712	    puVar11 = puStack_118;
 713	    puVar26 = puStack_1c0;
 714	    lVar6 = lStack_240;
 715	    lVar19 = *(longlong *)(lStack_110 + 0x28);
 716	    lVar30 = lVar22 * 0x20;
 717	    *(undefined8 *)(lVar19 + lVar30) = uVar12;
 718	    *(undefined1 *)(lVar19 + 8 + lVar30) = 0;
 719	    *(bool *)(lVar19 + 9 + lVar30) = 0.0 < fVar36;
 720	    *(byte *)(lVar19 + 10 + lVar30) = (byte)uStack_1dc & (bStack_f2 | bStack_f1);
 721	    *(float *)(lVar19 + 0xc + lVar30) = fVar35;
 722	    *(undefined4 *)(lVar19 + 0x18 + lVar30) = plStack_198._0_4_;
 723	    *(ulonglong *)(lVar19 + 0x10 + lVar30) = uStack_1a0;
 724	    *(longlong *)(lStack_110 + 0x30) = lVar22 + 1;
 725	    *(longlong *)(lStack_110 + 0x40) = *(longlong *)(lStack_110 + 0x40) + 1;
 726	    if (uVar13 >> 0x20 != 0) {
 727	      if (1 < *puRam000000014362c3a8) {
 728	        pcStack_1f8 = FUN_140d8d6c0;
 729	        puStack_200 = puVar8;
 730	        puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_143034fc8);
 731	        uStack_180 = (longlong *)*puVar9;
 732	        uStack_178 = puVar9[1];
 733	        uStack_170 = 2;
 734	        uStack_16c = 0;
 735	        uStack_168 = 0x43034fe0;
 736	        uStack_164 = 1;
 737	        uStack_160 = 0x1e;
 738	        iStack_15c = 0;
 739	        puStack_150 = &UNK_143034fa8;
 740	        uStack_148 = 2;
 741	        ppuStack_140 = &puStack_200;
 742	        uStack_138 = 1;
 743	        uStack_130 = 0;
 744	        uStack_1a0 = 0;
 745	        plStack_198 = (longlong *)&UNK_143034fe0;
 746	        plStack_190 = (longlong *)0x1e;
 747	        uStack_188 = 0;
 748	        uStack_158 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
 749	        func_0x000140d4a330(&uStack_f3,&uStack_1a0);
 750	      }
 751	      *puStack_220 = uStack_1d4;
 752	      lVar22 = *(longlong *)(lStack_1f0 + 0x30);
 753	      uVar12 = *(undefined8 *)(lStack_1f0 + 0x40);
 754	      if (lVar22 == *(longlong *)(lStack_1f0 + 0x20)) {
 755	        FUN_140562630(lStack_218,&UNK_143034d10);
 756	      }
 757	      lVar19 = *(longlong *)(lStack_1f0 + 0x28);
 758	      lVar30 = lVar22 * 0x10;
 759	      *(undefined8 *)(lVar19 + lVar30) = uVar12;
 760	      *(ulonglong *)(lVar19 + 8 + lVar30) = uVar13;
 761	      *(longlong *)(lStack_1f0 + 0x30) = lVar22 + 1;
 762	      *(longlong *)(lStack_1f0 + 0x40) = *(longlong *)(lStack_1f0 + 0x40) + 1;
 763	    }
 764	  } while( true );
 765	  puVar26 = (ulonglong *)*puStack_208;
 766	  lVar6 = puStack_208[1];
 767	  uVar13 = *puVar26;
 768	  uVar15 = 0;
 769	  uVar14 = uVar13 - *(ulonglong *)(lVar6 + 0x18);
 770	  if (uVar13 < *(ulonglong *)(lVar6 + 0x18)) {
 771	    uVar14 = uVar15;
 772	  }
 773	  uVar17 = uVar13 - *(ulonglong *)(lVar6 + 0x38);
 774	  if (uVar13 < *(ulonglong *)(lVar6 + 0x38)) {
 775	    uVar17 = uVar15;
 776	  }
 777	  lVar30 = uVar14 * 0x10 + *(longlong *)(lVar6 + 8);
 778	  uVar13 = *(ulonglong *)(lVar6 + 0x10) - uVar14;
 779	  if (*(ulonglong *)(lVar6 + 0x10) < uVar14) {
 780	    lVar30 = 8;
 781	    uVar13 = uVar15;
 782	  }
 783	  lVar22 = uVar17 * 0x10 + *(longlong *)(lVar6 + 0x28);
 784	  uVar14 = *(ulonglong *)(lVar6 + 0x30) - uVar17;
 785	  if (*(ulonglong *)(lVar6 + 0x30) < uVar17) {
 786	    lVar22 = 8;
 787	    uVar14 = uVar15;
 788	  }
 789	  *puVar26 = (*(longlong *)(lVar6 + 0x40) - uVar14) - uVar13;
 790	  lVar6 = uVar13 * 0x10 + lVar30;
 791	  lStack_108 = uVar14 * 0x10 + lVar22;
 792	  puStack_118 = puVar26;
 793	  while( true ) {
 794	    do {
 795	      if ((lVar30 == 0) || (lVar30 == lVar6)) {
 796	        if ((lVar22 == 0) || (lVar22 == lStack_108)) {
 797	          return;
 798	        }
 799	        lVar30 = 0;
 800	        lVar19 = lVar22;
 801	        lVar22 = lVar22 + 0x10;
 802	      }
 803	      else {
 804	        lVar19 = lVar30;
 805	        lVar30 = lVar30 + 0x10;
 806	      }
 807	      *puVar26 = *puVar26 + 1;
 808	      *puStack_1c8 = uStack_fc;
 809	      uVar13 = FUN_140a385a0(uStack_1d0,*(undefined8 *)(lVar19 + 8));
 810	    } while ((uVar13 >> 0x20 == 0) || (cVar4 = func_0x000140ceafa0(*param_7,uVar13), cVar4 == '\0'))
 811	    ;
 812	    plVar29 = (longlong *)param_7[1];
 813	    if (plVar29 == (longlong *)0x0) {
 814	      plVar29 = (longlong *)param_7[2];
 815	    }
 816	    lVar19 = plVar29[2];
 817	    if ((ulonglong)(*plVar29 - lVar19) < 0x20) {
 818	      FUN_1429105f0(plVar29,lVar19,0x20,1,1);
 819	    }
 820	    puVar26 = puStack_118;
 821	    uVar14 = plVar29[1];
 822	    *(code **)(uVar14 + lVar19) = FUN_1425de790;
 823	    *(code **)(uVar14 + 8 + lVar19) = FUN_1425de2b0;
 824	    *(ulonglong *)(uVar14 + 0x10 + lVar19) = uVar13;
 825	    *(undefined1 *)(uVar14 + 0x18 + lVar19) = 0;
 826	    plVar29[2] = lVar19 + 0x20;
 827	    uVar15 = uVar13 & 0xffffffff;
 828	    if ((*(ulonglong *)(lStack_1a8 + 0x10) <= uVar15) ||
 829	       (uVar14 = *(ulonglong *)(lStack_1a8 + 8),
 830	       *(int *)(uVar14 + uVar15 * 0x14) != (int)(uVar13 >> 0x20))) break;
 831	    lVar19 = uVar14 + uVar15 * 0x14;
 832	    uVar5 = *(uint *)(lVar19 + 4);
 833	    uVar14 = (ulonglong)uVar5;
 834	    if (uVar14 == 0xffffffff) break;
 835	    uVar21 = 0;
 836	    if ((*(ulonglong *)(lStack_1e8 + 0x38) <= uVar14) ||
 837	       ((*(ulonglong *)(*(longlong *)(lStack_1e8 + 0x28) + (ulonglong)(uVar5 >> 6) * 8) >>
 838	         (uVar14 & 0x3f) & 1) == 0)) goto code_r0x00014226856f;
 839	    lVar31 = *(longlong *)(lStack_1a8 + 0x1a8);
 840	    lVar18 = *(longlong *)
 841	              (*(longlong *)(lVar31 + 0x18 + (ulonglong)*(uint *)(lVar19 + 0xc) * 0x48) + 0x10 +
 842	              ~*(ulonglong *)
 843	                (*(longlong *)(lVar31 + 0x38 + (ulonglong)*(uint *)(lVar19 + 0xc) * 0x48) +
 844	                *(longlong *)(lStack_1e8 + 0x110) * 8) * 0x30);
 845	    lVar10 = (ulonglong)*(uint *)(lVar19 + 0x10) * 0x30;
 846	    uVar34 = func_0x000140c904a0((ulonglong)*(uint *)(lVar19 + 0x10),lVar31,
 847	                                 *(undefined4 *)(lVar18 + 0xc + lVar10));
 848	    uVar12 = *(undefined8 *)(lVar18 + lVar10);
 849	    uVar21 = *(undefined4 *)(lVar18 + 8 + lVar10);
 850	    *puStack_1b8 = uStack_f8;