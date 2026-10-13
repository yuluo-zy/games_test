   1	
   2	/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
   3	
   4	undefined8 *
   5	FUN_140c926b0(undefined8 *param_1,longlong param_2,float *param_3,ulonglong param_4,
   6	             undefined4 *param_5,undefined4 *param_6)
   7	
   8	{
   9	  ulonglong uVar1;
  10	  float *pfVar2;
  11	  float *pfVar3;
  12	  float fVar4;
  13	  float fVar5;
  14	  undefined4 uVar6;
  15	  undefined4 uVar7;
  16	  ulonglong *puVar8;
  17	  code *pcVar9;
  18	  undefined8 uVar10;
  19	  undefined8 uVar11;
  20	  float *pfVar12;
  21	  float *pfVar13;
  22	  ulonglong uVar14;
  23	  undefined4 *puVar15;
  24	  ulonglong uVar16;
  25	  longlong lVar17;
  26	  ulonglong uVar18;
  27	  longlong lVar19;
  28	  undefined4 *puVar20;
  29	  float *pfVar21;
  30	  ulonglong uVar22;
  31	  longlong lVar23;
  32	  ulonglong uVar24;
  33	  undefined4 *puVar25;
  34	  ulonglong uVar26;
  35	  undefined4 *puVar27;
  36	  ulonglong uVar28;
  37	  float *pfVar29;
  38	  longlong lVar30;
  39	  undefined4 *puVar31;
  40	  undefined1 auVar32 [16];
  41	  float fVar33;
  42	  float fVar34;
  43	  float fVar36;
  44	  float fVar37;
  45	  undefined1 auVar35 [16];
  46	  float *pfStack_b8;
  47	  longlong lStack_b0;
  48	  ulonglong uStack_a8;
  49	  ulonglong uStack_a0;
  50	  longlong lStack_90;
  51	  ulonglong uStack_88;
  52	  float *pfStack_80;
  53	  undefined4 *puStack_78;
  54	  ulonglong uStack_70;
  55	  ulonglong uStack_68;
  56	  undefined8 uStack_60;
  57	  
  58	  uStack_60 = 0xfffffffffffffffe;
  59	  if (param_4 < 2) {
  60	LAB_140c92fcb:
  61	    FUN_1428d9430(&UNK_142b2dbd8,0x23,&UNK_142b2dc20);
  62	LAB_140c92fe3:
  63	    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
  64	  }
  65	  else {
  66	    uVar1 = param_4 * 4 - 4;
  67	    puVar8 = *(ulonglong **)(param_2 + 0x10);
  68	    uVar28 = puVar8[4] & 0xfffffffffffffffc;
  69	    uStack_70 = param_4;
  70	    if ((uVar28 < *puVar8) || (uVar28 - *puVar8 < uVar1)) {
  71	LAB_140c92f62:
  72	      pfVar29 = (float *)FUN_1428d7780(param_2,4,uVar1);
  73	      if (pfVar29 != (float *)0x0) goto LAB_140c9272d;
  74	LAB_140c92fc6:
  75	      FUN_1428d79d0();
  76	      goto LAB_140c92fcb;
  77	    }
  78	    pfVar29 = (float *)(uVar28 - uVar1);
  79	    puVar8[4] = (ulonglong)pfVar29;
  80	    if (pfVar29 == (float *)0x0) goto LAB_140c92f62;
  81	LAB_140c9272d:
  82	    lVar17 = uStack_70 * 2;
  83	    uStack_68 = uStack_70 - 1;
  84	    fVar33 = (float)*(undefined8 *)(param_3 + 2) - (float)*(undefined8 *)param_3;
  85	    fVar34 = (float)((ulonglong)*(undefined8 *)(param_3 + 2) >> 0x20) -
  86	             (float)((ulonglong)*(undefined8 *)param_3 >> 0x20);
  87	    *pfVar29 = SQRT(fVar34 * fVar34 + fVar33 * fVar33);
  88	    if (uStack_68 == 1) {
  89	      uStack_88 = 2;
  90	      uVar28 = 8;
  91	    }
  92	    else {
  93	      pfVar21 = param_3 + 2;
  94	      pfVar13 = param_3 + 4;
  95	      uVar24 = uStack_70 * 8 - 0x10 >> 3;
  96	      uVar28 = uStack_70 - 3;
  97	      uVar14 = uVar28;
  98	      if (uVar24 < uVar28) {
  99	        uVar14 = uVar24;
 100	      }
 101	      uVar16 = 1;
 102	      pfVar12 = pfVar13;
 103	      if (3 < uVar14) {
 104	        if (uVar24 < uVar28) {
 105	          uVar28 = uVar24;
 106	        }
 107	        if ((param_3 + uVar28 * 2 + 6 <= pfVar29 + 1) || (pfVar29 + uVar28 + 2 <= pfVar21)) {
 108	          uVar14 = uVar14 + 1;
 109	          uVar28 = 4;
 110	          if ((uVar14 & 3) != 0) {
 111	            uVar28 = (ulonglong)((uint)uVar14 & 3);
 112	          }
 113	          lVar19 = uVar14 - uVar28;
 114	          pfVar21 = pfVar21 + lVar19 * 2;
 115	          uVar16 = (uVar14 - uVar28) + 1;
 116	          lVar30 = 0;
 117	          do {
 118	            pfVar2 = param_3 + lVar30 * 2 + 2;
 119	            pfVar12 = pfVar13 + lVar30 * 2;
 120	            auVar32 = *(undefined1 (*) [16])(param_3 + lVar30 * 2 + 6);
 121	            pfVar3 = param_3 + lVar30 * 2 + 8;
 122	            fVar33 = *pfVar3 - auVar32._0_4_;
 123	            fVar34 = pfVar3[1] - auVar32._4_4_;
 124	            fVar36 = pfVar3[2] - auVar32._8_4_;
 125	            fVar37 = pfVar3[3] - auVar32._12_4_;
 126	            auVar35._4_4_ =
 127	                 (pfVar12[2] - pfVar2[2]) * (pfVar12[2] - pfVar2[2]) +
 128	                 (pfVar12[3] - pfVar2[3]) * (pfVar12[3] - pfVar2[3]);
 129	            auVar35._0_4_ =
 130	                 (*pfVar12 - *pfVar2) * (*pfVar12 - *pfVar2) +
 131	                 (pfVar12[1] - pfVar2[1]) * (pfVar12[1] - pfVar2[1]);
 132	            auVar35._8_4_ = fVar33 * fVar33 + fVar34 * fVar34;
 133	            auVar35._12_4_ = fVar36 * fVar36 + fVar37 * fVar37;
 134	            auVar32 = sqrtps(auVar32,auVar35);
 135	            *(undefined1 (*) [16])(pfVar29 + lVar30 + 1) = auVar32;
 136	            lVar30 = lVar30 + 4;
 137	            pfVar12 = pfVar13 + lVar19 * 2;
 138	          } while (lVar19 != lVar30);
 139	        }
 140	      }
 141	      do {
 142	        pfVar13 = pfVar12;
 143	        if (pfVar13 == param_3 + lVar17) goto LAB_140c92fe3;
 144	        fVar33 = (float)*(undefined8 *)pfVar13 - (float)*(undefined8 *)pfVar21;
 145	        fVar34 = (float)((ulonglong)*(undefined8 *)pfVar13 >> 0x20) -
 146	                 (float)((ulonglong)*(undefined8 *)pfVar21 >> 0x20);
 147	        pfVar29[uVar16] = SQRT(fVar34 * fVar34 + fVar33 * fVar33);
 148	        uVar16 = uVar16 + 1;
 149	        pfVar12 = pfVar13 + 2;
 150	        pfVar21 = pfVar13;
 151	      } while (uStack_68 != uVar16);
 152	      if (uStack_68 == 0x1fffffffffffffff) goto LAB_140c92fc6;
 153	      uVar28 = uStack_70 * 4;
 154	      uStack_88 = uStack_70;
 155	    }
 156	    puVar8 = *(ulonglong **)(param_2 + 0x10);
 157	    uVar14 = puVar8[4] & 0xfffffffffffffffc;
 158	    if ((uVar14 < *puVar8) || (uVar14 - *puVar8 < uVar28)) {
 159	LAB_140c92f87:
 160	      puVar15 = (undefined4 *)FUN_1428d7780(param_2);
 161	      if (puVar15 == (undefined4 *)0x0) goto LAB_140c92fc6;
 162	    }
 163	    else {
 164	      puVar15 = (undefined4 *)(uVar14 - uVar28);
 165	      puVar8[4] = (ulonglong)puVar15;
 166	      if (puVar15 == (undefined4 *)0x0) goto LAB_140c92f87;
 167	    }
 168	    puStack_78 = puVar15;
 169	    func_0x000142923c20(puVar15,0,param_4 * 4);
 170	    uVar28 = uStack_68;
 171	    uVar14 = 0;
 172	    if ((uStack_88 & 0x3fffffffffffffff) != 0) {
 173	      uVar14 = (uStack_88 & 0x3fffffffffffffff) - 1;
 174	    }
 175	    if (uStack_68 < uVar14) {
 176	      uVar14 = uStack_68;
 177	    }
 178	    if (uVar14 != 0) {
 179	      uVar24 = (ulonglong)((uint)uVar14 & 3);
 180	      if (uVar14 < 4) {
 181	        fVar33 = 0.0;
 182	        uVar18 = 0;
 183	      }
 184	      else {
 185	        fVar33 = 0.0;
 186	        uVar16 = 0;
 187	        do {
 188	          fVar34 = pfVar29[uVar16];
 189	          puStack_78[uVar16 + 1] = fVar33 + fVar34;
 190	          fVar33 = fVar33 + fVar34 + pfVar29[uVar16 + 1];
 191	          puStack_78[uVar16 + 2] = fVar33;
 192	          fVar33 = fVar33 + pfVar29[uVar16 + 2];
 193	          puStack_78[uVar16 + 3] = fVar33;
 194	          fVar33 = fVar33 + pfVar29[uVar16 + 3];
 195	          uVar18 = uVar16 + 4;
 196	          puStack_78[uVar16 + 4] = fVar33;
 197	          uVar16 = uVar18;
 198	        } while ((uVar14 & 0x3ffffffffffffffc) != uVar18);
 199	      }
 200	      if (uVar24 != 0) {
 201	        uVar14 = 0;
 202	        do {
 203	          fVar33 = fVar33 + pfVar29[uVar18 + uVar14];
 204	          puStack_78[uVar18 + uVar14 + 1] = fVar33;
 205	          uVar14 = uVar14 + 1;
 206	        } while (uVar24 != uVar14);
 207	      }
 208	    }
 209	    if (uStack_68 >> 0x3d != 0) goto LAB_140c92fc6;
 210	    puVar8 = *(ulonglong **)(param_2 + 0x10);
 211	    uVar14 = puVar8[4] & 0xfffffffffffffffc;
 212	    lStack_90 = param_2;
 213	    if ((uVar14 < *puVar8) || (uVar14 - *puVar8 < uVar1)) {
 214	LAB_140c92f9f:
 215	      uVar14 = FUN_1428d7780(param_2,4,uVar1);
 216	      if (uVar14 == 0) goto LAB_140c92fc6;
 217	    }
 218	    else {
 219	      uVar14 = uVar14 - uVar1;
 220	      puVar8[4] = uVar14;
 221	      if (uVar14 == 0) goto LAB_140c92f9f;
 222	    }
 223	    puVar15 = puStack_78;
 224	    uVar1 = uStack_88;
 225	    auVar32 = _DAT_142925930;
 226	    uVar24 = uStack_70 - 2;
 227	    if (uVar28 < uStack_70 - 2) {
 228	      uVar24 = uVar28;
 229	    }
 230	    uVar16 = 0;
 231	    pfVar21 = pfVar29;
 232	    if ((3 < uVar24) && (0xf < uVar14 - (longlong)pfVar29)) {
 233	      uVar24 = uVar24 + 1;
 234	      uVar16 = 4;
 235	      if ((uVar24 & 3) != 0) {
 236	        uVar16 = (ulonglong)((uint)uVar24 & 3);
 237	      }
 238	      uVar16 = uVar24 - uVar16;
 239	      pfVar21 = pfVar29 + uVar16;
 240	      uVar24 = 0;
 241	      do {
 242	        auVar35 = divps(auVar32,*(undefined1 (*) [16])(pfVar29 + uVar24));
 243	        *(undefined1 (*) [16])(uVar14 + uVar24 * 4) = auVar35;
 244	        uVar24 = uVar24 + 4;
 245	      } while (uVar16 != uVar24);
 246	    }
 247	    do {
 248	      if (pfVar21 == pfVar29 + uVar28) goto LAB_140c92fe3;
 249	      fVar33 = *pfVar21;
 250	      pfVar21 = pfVar21 + 1;
 251	      *(float *)(uVar14 + uVar16 * 4) = 1.0 / fVar33;
 252	      uVar16 = uVar16 + 1;
 253	    } while (uVar28 != uVar16);
 254	    pfStack_b8 = (float *)0x4;
 255	    lStack_b0 = lStack_90;
 256	    uStack_a8 = 0;
 257	    uStack_a0 = 0;
 258	    FUN_141940de0(&pfStack_b8,0,uStack_70,1);
 259	    pfVar21 = param_3;
 260	    do {
 261	      fVar33 = *pfVar21;
 262	      if (uStack_a0 == uStack_a8) {
 263	        FUN_141940de0(&pfStack_b8,uStack_a0,1,1);
 264	      }
 265	      uVar28 = uStack_a8;
 266	      lVar30 = lStack_b0;
 267	      pfVar21 = pfVar21 + 2;
 268	      pfStack_b8[uStack_a0] = fVar33;
 269	      uVar24 = uStack_a0 + 1;
 270	      uStack_a0 = uVar24;
 271	    } while (pfVar21 != param_3 + lVar17);
 272	    uVar6 = *param_5;
 273	    uVar7 = *param_6;
 274	    pfStack_80 = pfStack_b8;
 275	    puVar25 = puStack_78;
 276	    lVar17 = FUN_140c94a10(lStack_90,puStack_78,uStack_88,pfVar29,uStack_68,uVar14,uStack_68,
 277	                           pfStack_b8,uVar24,uVar6,param_5[1],uVar7,param_6[1]);
 278	    uVar16 = uStack_70;
 279	    if (uVar24 <= uStack_70) {
 280	      uVar16 = uVar24;
 281	    }
 282	    if (uVar24 != 0) {
 283	      if (uVar16 < 0x10) {
 284	        uVar18 = 0;
 285	      }
 286	      else if (pfStack_80 < param_3 + uVar16 * 2 && param_3 + 1 < pfStack_80 + uVar16) {
 287	        uVar18 = 0;
 288	      }
 289	      else {
 290	        uVar18 = uVar16 & 0x3ffffffffffffff8;
 291	        uVar22 = 0;
 292	        do {
 293	          fVar33 = param_3[uVar22 * 2 + 5];
 294	          fVar34 = param_3[uVar22 * 2 + 7];
 295	          fVar36 = param_3[uVar22 * 2 + 0xf];
 296	          fVar37 = param_3[uVar22 * 2 + 0xd];
 297	          fVar4 = param_3[uVar22 * 2 + 0xb];
 298	          fVar5 = param_3[uVar22 * 2 + 9];
 299	          *(ulonglong *)(pfStack_80 + uVar22) =
 300	               CONCAT44(param_3[uVar22 * 2 + 3],param_3[uVar22 * 2 + 1]);
 301	          *(ulonglong *)(pfStack_80 + uVar22 + 2) = CONCAT44(fVar34,fVar33);
 302	          pfVar21 = pfStack_80 + uVar22 + 4;
 303	          *pfVar21 = fVar5;
 304	          pfVar21[1] = fVar4;
 305	          pfVar21[2] = fVar37;
 306	          pfVar21[3] = fVar36;
 307	          uVar22 = uVar22 + 8;
 308	        } while (uVar18 != uVar22);
 309	        if (uVar16 == uVar18) goto LAB_140c92d8f;
 310	      }
 311	      uVar22 = uVar18;
 312	      for (uVar26 = uVar16 & 3; uVar26 != 0; uVar26 = uVar26 - 1) {
 313	        pfStack_80[uVar22] = param_3[uVar22 * 2 + 1];
 314	        uVar22 = uVar22 + 1;
 315	      }
 316	      if (uVar18 - uVar16 < 0xfffffffffffffffd) {
 317	        do {
 318	          pfStack_80[uVar22] = param_3[uVar22 * 2 + 1];
 319	          pfStack_80[uVar22 + 1] = param_3[uVar22 * 2 + 3];
 320	          pfStack_80[uVar22 + 2] = param_3[uVar22 * 2 + 5];
 321	          pfStack_80[uVar22 + 3] = param_3[uVar22 * 2 + 7];
 322	          uVar22 = uVar22 + 4;
 323	        } while (uVar16 != uVar22);
 324	      }
 325	    }
 326	LAB_140c92d8f:
 327	    puVar20 = puStack_78;
 328	    lVar19 = FUN_140c94a10(lStack_90,puStack_78,uStack_88,pfVar29,uStack_68,uVar14,uStack_68,
 329	                           pfStack_80,uVar24,uVar6,param_5[2],uVar7,param_6[2]);
 330	    if (puVar20 < puVar25) {
 331	      puVar25 = puVar20;
 332	    }
 333	    if (puVar25 < (undefined4 *)0x2e8ba2e8ba2e8bb) {
 334	      puVar8 = *(ulonglong **)(lStack_90 + 0x10);
 335	      uVar14 = puVar8[4] & 0xfffffffffffffffc;
 336	      if ((uVar14 < *puVar8) ||
 337	         (uVar14 - *puVar8 <= (ulonglong)((longlong)puVar25 * 0x2c) &&
 338	          (longlong)puVar25 * 0x2c - (uVar14 - *puVar8) != 0)) {
 339	LAB_140c92e4a:
 340	        puVar20 = (undefined4 *)FUN_1428d7780(lStack_90,4);
 341	        if (puVar20 == (undefined4 *)0x0) {
 342	          FUN_1428d79d0();
 343	          goto LAB_140c93015;
 344	        }
 345	      }
 346	      else {
 347	        puVar20 = (undefined4 *)(uVar14 + (longlong)puVar25 * -0x2c);
 348	        puVar8[4] = (ulonglong)puVar20;
 349	        if (puVar20 == (undefined4 *)0x0) goto LAB_140c92e4a;
 350	      }
 351	      if (puVar25 != (undefined4 *)0x0) {
 352	        lVar23 = 0;
 353	        puVar27 = puVar25;
 354	        puVar31 = puVar20;
 355	        do {
 356	          uVar6 = *(undefined4 *)(lVar17 + 0x10 + lVar23);
 357	          uStack_a8 = CONCAT44(uStack_a8._4_4_,uVar6);
 358	          pfStack_b8 = *(float **)(lVar17 + lVar23);
 359	          lStack_b0 = ((undefined8 *)(lVar17 + lVar23))[1];
 360	          uVar7 = *(undefined4 *)(lVar19 + 0x10 + lVar23);
 361	          uVar10 = *(undefined8 *)(lVar19 + lVar23);
 362	          uVar11 = ((undefined8 *)(lVar19 + lVar23))[1];
 363	          *puVar31 = *(undefined4 *)(lVar17 + 0x10 + lVar23);
 364	          *(float **)(puVar31 + 1) = pfStack_b8;
 365	          *(longlong *)(puVar31 + 3) = lStack_b0;
 366	          puVar31[5] = uVar6;
 367	          *(undefined8 *)(puVar31 + 6) = uVar10;
 368	          *(undefined8 *)(puVar31 + 8) = uVar11;
 369	          puVar31[10] = uVar7;
 370	          lVar23 = lVar23 + 0x14;
 371	          puVar31 = puVar31 + 0xb;
 372	          puVar27 = (undefined4 *)((longlong)puVar27 + -1);
 373	        } while (puVar27 != (undefined4 *)0x0);
 374	      }
 375	      if (puVar15 + (uVar1 - 1) != (undefined4 *)0x0) {
 376	        uVar6 = *puStack_78;
 377	        uVar7 = puVar15[uVar1 - 1];
 378	        *param_1 = puVar20;
 379	        param_1[1] = puVar25;
 380	        *(undefined4 *)(param_1 + 2) = uVar6;
 381	        *(undefined4 *)((longlong)param_1 + 0x14) = uVar7;
 382	        *(undefined1 *)(param_1 + 3) = 0;
 383	        if (uVar28 != 0) {
 384	          pfVar29 = *(float **)(*(longlong *)(lVar30 + 0x10) + 0x20);
 385	          if (pfVar29 == pfStack_80) {
 386	            *(float **)(*(longlong *)(lVar30 + 0x10) + 0x20) = pfVar29 + uVar28;
 387	          }
 388	        }
 389	        return param_1;
 390	      }
 391	      FUN_1428d9310(&UNK_142b2dc38);
 392	      goto LAB_140c93015;
 393	    }
 394	  }
 395	  FUN_1428d79d0();
 396	LAB_140c93015:
 397	                    /* WARNING: Does not return */
 398	  pcVar9 = (code *)invalidInstructionException();
 399	  (*pcVar9)();
 400	}
 401	
