   1	
   2	/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
   3	
   4	undefined8 *
   5	FUN_140c930f0(undefined8 *param_1,longlong param_2,undefined8 *param_3,ulonglong param_4)
   6	
   7	{
   8	  ulonglong uVar1;
   9	  undefined4 uVar2;
  10	  undefined4 uVar3;
  11	  undefined4 uVar4;
  12	  undefined4 uVar5;
  13	  undefined4 uVar6;
  14	  undefined4 uVar7;
  15	  undefined4 uVar8;
  16	  ulonglong *puVar9;
  17	  code *pcVar10;
  18	  undefined8 uVar11;
  19	  undefined8 uVar12;
  20	  undefined1 auVar13 [16];
  21	  longlong lVar14;
  22	  undefined4 *puVar15;
  23	  longlong lVar16;
  24	  ulonglong uVar17;
  25	  longlong lVar18;
  26	  undefined4 *puVar19;
  27	  undefined8 *puVar20;
  28	  ulonglong uVar21;
  29	  float *pfVar22;
  30	  ulonglong uVar23;
  31	  longlong lVar24;
  32	  undefined8 *puVar25;
  33	  undefined8 *puVar26;
  34	  undefined4 *puVar27;
  35	  ulonglong uVar28;
  36	  undefined4 *puVar29;
  37	  ulonglong uVar30;
  38	  ulonglong uVar31;
  39	  undefined4 *puVar32;
  40	  ulonglong uVar33;
  41	  float *pfVar34;
  42	  float fVar35;
  43	  float fVar36;
  44	  undefined1 auVar37 [16];
  45	  undefined4 uStack_e8;
  46	  undefined4 uStack_e4;
  47	  undefined4 uStack_e0;
  48	  undefined4 uStack_dc;
  49	  undefined8 *puStack_a8;
  50	  longlong lStack_a0;
  51	  ulonglong uStack_98;
  52	  ulonglong uStack_90;
  53	  ulonglong uStack_80;
  54	  longlong lStack_78;
  55	  undefined8 *puStack_70;
  56	  undefined4 *puStack_68;
  57	  undefined8 uStack_60;
  58	  
  59	  uStack_60 = 0xfffffffffffffffe;
  60	  if (param_4 < 3) {
  61	LAB_140c93851:
  62	    FUN_1428d9430(&UNK_142b2dc50,0x23,&UNK_142b2dc78);
  63	LAB_140c93869:
  64	    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
  65	  }
  66	  else {
  67	    uVar1 = param_4 * 4;
  68	    puVar9 = *(ulonglong **)(param_2 + 0x10);
  69	    uVar33 = puVar9[4] & 0xfffffffffffffffc;
  70	    if ((uVar33 < *puVar9) || (uVar33 - *puVar9 <= uVar1 && uVar1 - (uVar33 - *puVar9) != 0)) {
  71	LAB_140c937ef:
  72	      pfVar34 = (float *)FUN_1428d7780(param_2,4,uVar1);
  73	      if (pfVar34 != (float *)0x0) goto LAB_140c93169;
  74	LAB_140c9384c:
  75	      FUN_1428d79d0();
  76	      goto LAB_140c93851;
  77	    }
  78	    pfVar34 = (float *)(uVar33 + param_4 * -4);
  79	    puVar9[4] = (ulonglong)pfVar34;
  80	    if (pfVar34 == (float *)0x0) goto LAB_140c937ef;
  81	LAB_140c93169:
  82	    puVar25 = param_3 + 2;
  83	    fVar35 = (float)param_3[1] - (float)*param_3;
  84	    fVar36 = (float)((ulonglong)param_3[1] >> 0x20) - (float)((ulonglong)*param_3 >> 0x20);
  85	    *pfVar34 = SQRT(fVar36 * fVar36 + fVar35 * fVar35);
  86	    uVar33 = 1;
  87	    puVar20 = param_3 + 1;
  88	    do {
  89	      if (param_4 == uVar33) goto LAB_140c93869;
  90	      puVar26 = puVar25;
  91	      if (puVar25 == param_3 + param_4) {
  92	        puVar26 = param_3;
  93	      }
  94	      puVar25 = puVar26 + 1;
  95	      fVar35 = (float)*puVar26 - (float)*puVar20;
  96	      fVar36 = (float)((ulonglong)*puVar26 >> 0x20) - (float)((ulonglong)*puVar20 >> 0x20);
  97	      pfVar34[uVar33] = SQRT(fVar36 * fVar36 + fVar35 * fVar35);
  98	      uVar33 = uVar33 + 1;
  99	      puVar20 = puVar26;
 100	    } while (param_4 != uVar33);
 101	    uStack_80 = param_4 + 1;
 102	    if (uStack_80 >> 0x3d != 0) goto LAB_140c9384c;
 103	    uVar33 = param_4 * 4 + 4;
 104	    puVar9 = *(ulonglong **)(param_2 + 0x10);
 105	    uVar21 = puVar9[4] & 0xfffffffffffffffc;
 106	    if ((uVar21 < *puVar9) || (uVar21 - *puVar9 < uVar33)) {
 107	LAB_140c9380d:
 108	      puStack_68 = (undefined4 *)FUN_1428d7780(param_2);
 109	      if (puStack_68 == (undefined4 *)0x0) goto LAB_140c9384c;
 110	    }
 111	    else {
 112	      puStack_68 = (undefined4 *)(uVar21 - uVar33);
 113	      puVar9[4] = (ulonglong)puStack_68;
 114	      if (puStack_68 == (undefined4 *)0x0) goto LAB_140c9380d;
 115	    }
 116	    lStack_78 = param_2;
 117	    if (uStack_80 == 0) goto LAB_140c9384c;
 118	    uVar33 = 0;
 119	    func_0x000142923c20(puStack_68);
 120	    uVar21 = (ulonglong)((uint)param_4 & 3);
 121	    if (param_4 == 3) {
 122	      fVar35 = 0.0;
 123	    }
 124	    else {
 125	      fVar35 = 0.0;
 126	      uVar30 = 0;
 127	      do {
 128	        fVar36 = pfVar34[uVar30];
 129	        puStack_68[uVar30 + 1] = fVar35 + fVar36;
 130	        fVar35 = fVar35 + fVar36 + pfVar34[uVar30 + 1];
 131	        puStack_68[uVar30 + 2] = fVar35;
 132	        fVar35 = fVar35 + pfVar34[uVar30 + 2];
 133	        puStack_68[uVar30 + 3] = fVar35;
 134	        fVar35 = fVar35 + pfVar34[uVar30 + 3];
 135	        uVar33 = uVar30 + 4;
 136	        puStack_68[uVar30 + 4] = fVar35;
 137	        uVar30 = uVar33;
 138	      } while ((param_4 & 0xfffffffffffffffc) != uVar33);
 139	    }
 140	    if (uVar21 != 0) {
 141	      uVar30 = 0;
 142	      do {
 143	        fVar35 = fVar35 + pfVar34[uVar33 + uVar30];
 144	        puStack_68[uVar33 + uVar30 + 1] = fVar35;
 145	        uVar30 = uVar30 + 1;
 146	      } while (uVar21 != uVar30);
 147	    }
 148	    if (param_4 >> 0x3d != 0) goto LAB_140c9384c;
 149	    puVar9 = *(ulonglong **)(lStack_78 + 0x10);
 150	    uVar33 = puVar9[4] & 0xfffffffffffffffc;
 151	    if ((uVar33 < *puVar9) || (uVar33 - *puVar9 <= uVar1 && uVar1 - (uVar33 - *puVar9) != 0)) {
 152	LAB_140c9382c:
 153	      uVar33 = FUN_1428d7780(lStack_78,4,uVar1);
 154	      if (uVar33 == 0) goto LAB_140c9384c;
 155	    }
 156	    else {
 157	      uVar33 = uVar33 + param_4 * -4;
 158	      puVar9[4] = uVar33;
 159	      if (uVar33 == 0) goto LAB_140c9382c;
 160	    }
 161	    puVar15 = puStack_68;
 162	    uVar1 = uStack_80;
 163	    auVar13 = _DAT_142925930;
 164	    uVar21 = param_4 - 1;
 165	    if (param_4 < param_4 - 1) {
 166	      uVar21 = param_4;
 167	    }
 168	    uVar30 = 0;
 169	    pfVar22 = pfVar34;
 170	    if ((3 < uVar21) && (0xf < uVar33 - (longlong)pfVar34)) {
 171	      uVar21 = uVar21 + 1;
 172	      uVar30 = 4;
 173	      if ((uVar21 & 3) != 0) {
 174	        uVar30 = (ulonglong)((uint)uVar21 & 3);
 175	      }
 176	      uVar30 = uVar21 - uVar30;
 177	      pfVar22 = pfVar34 + uVar30;
 178	      uVar21 = 0;
 179	      do {
 180	        auVar37 = divps(auVar13,*(undefined1 (*) [16])(pfVar34 + uVar21));
 181	        *(undefined1 (*) [16])(uVar33 + uVar21 * 4) = auVar37;
 182	        uVar21 = uVar21 + 4;
 183	      } while (uVar30 != uVar21);
 184	    }
 185	    do {
 186	      if (pfVar22 == pfVar34 + param_4) goto LAB_140c93869;
 187	      fVar35 = *pfVar22;
 188	      pfVar22 = pfVar22 + 1;
 189	      *(float *)(uVar33 + uVar30 * 4) = 1.0 / fVar35;
 190	      uVar30 = uVar30 + 1;
 191	    } while (param_4 != uVar30);
 192	    puStack_a8 = (undefined8 *)0x4;
 193	    lStack_a0 = lStack_78;
 194	    uStack_98 = 0;
 195	    uStack_90 = 0;
 196	    FUN_141940de0(&puStack_a8,0,param_4,1);
 197	    puVar25 = param_3;
 198	    do {
 199	      uVar2 = *(undefined4 *)puVar25;
 200	      if (uStack_90 == uStack_98) {
 201	        FUN_141940de0(&puStack_a8,uStack_90,1,1);
 202	      }
 203	      uVar21 = uStack_98;
 204	      lVar14 = lStack_a0;
 205	      puVar25 = puVar25 + 1;
 206	      *(undefined4 *)((longlong)puStack_a8 + uStack_90 * 4) = uVar2;
 207	      uVar30 = uStack_90 + 1;
 208	      uStack_90 = uVar30;
 209	    } while (puVar25 != param_3 + param_4);
 210	    puStack_70 = puStack_a8;
 211	    puVar27 = puStack_68;
 212	    lVar16 = FUN_140c95200(lStack_78,puStack_68,uStack_80,pfVar34,param_4,uVar33,param_4,puStack_a8,
 213	                           uVar30);
 214	    uVar17 = uVar30;
 215	    if (param_4 < uVar30) {
 216	      uVar17 = param_4;
 217	    }
 218	    if (uVar30 != 0) {
 219	      if ((uVar17 < 0x10) ||
 220	         (puStack_70 < param_3 + uVar17 &&
 221	          (undefined4 *)((longlong)param_3 + 4U) < (undefined4 *)((longlong)puStack_70 + uVar17 * 4)
 222	         )) {
 223	        uVar23 = 0;
 224	      }
 225	      else {
 226	        uVar23 = uVar17 & 0x1ffffffffffffff8;
 227	        uVar28 = 0;
 228	        do {
 229	          uVar2 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0xc);
 230	          uVar8 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x14);
 231	          uVar3 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x1c);
 232	          uVar4 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x3c);
 233	          uVar5 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x34);
 234	          uVar6 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x2c);
 235	          uVar7 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x24);
 236	          puVar19 = (undefined4 *)((longlong)puStack_70 + uVar28 * 4);
 237	          *puVar19 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
 238	          puVar19[1] = uVar2;
 239	          puVar19[2] = uVar8;
 240	          puVar19[3] = uVar3;
 241	          puVar19 = (undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 0x10);
 242	          *puVar19 = uVar7;
 243	          puVar19[1] = uVar6;
 244	          puVar19[2] = uVar5;
 245	          puVar19[3] = uVar4;
 246	          uVar28 = uVar28 + 8;
 247	        } while (uVar23 != uVar28);
 248	        if (uVar17 == uVar23) goto LAB_140c93647;
 249	      }
 250	      uVar28 = uVar23;
 251	      for (uVar31 = uVar17 & 3; uVar31 != 0; uVar31 = uVar31 - 1) {
 252	        *(undefined4 *)((longlong)puStack_70 + uVar28 * 4) =
 253	             *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
 254	        uVar28 = uVar28 + 1;
 255	      }
 256	      if (uVar23 - uVar17 < 0xfffffffffffffffd) {
 257	        do {
 258	          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4) =
 259	               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
 260	          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 4) =
 261	               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0xc);
 262	          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 8) =
 263	               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x14);
 264	          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 0xc) =
 265	               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x1c);
 266	          uVar28 = uVar28 + 4;
 267	        } while (uVar17 != uVar28);
 268	      }
 269	    }
 270	LAB_140c93647:
 271	    puVar19 = puStack_68;
 272	    lVar18 = FUN_140c95200(lStack_78,puStack_68,uStack_80,pfVar34,param_4,uVar33,param_4,puStack_70,
 273	                           uVar30);
 274	    if (puVar19 < puVar27) {
 275	      puVar27 = puVar19;
 276	    }
 277	    if (puVar27 < (undefined4 *)0x2e8ba2e8ba2e8bb) {
 278	      puVar9 = *(ulonglong **)(lStack_78 + 0x10);
 279	      uVar33 = puVar9[4] & 0xfffffffffffffffc;
 280	      if ((uVar33 < *puVar9) ||
 281	         (uVar33 - *puVar9 <= (ulonglong)((longlong)puVar27 * 0x2c) &&
 282	          (longlong)puVar27 * 0x2c - (uVar33 - *puVar9) != 0)) {
 283	LAB_140c936c8:
 284	        puVar19 = (undefined4 *)FUN_1428d7780(lStack_78,4);
 285	        if (puVar19 == (undefined4 *)0x0) {
 286	          FUN_1428d79d0();
 287	          goto LAB_140c938ad;
 288	        }
 289	      }
 290	      else {
 291	        puVar19 = (undefined4 *)(uVar33 + (longlong)puVar27 * -0x2c);
 292	        puVar9[4] = (ulonglong)puVar19;
 293	        if (puVar19 == (undefined4 *)0x0) goto LAB_140c936c8;
 294	      }
 295	      if (puVar27 != (undefined4 *)0x0) {
 296	        lVar24 = 0;
 297	        puVar29 = puVar27;
 298	        puVar32 = puVar19;
 299	        do {
 300	          uVar2 = *(undefined4 *)(lVar16 + 0x10 + lVar24);
 301	          uStack_98 = CONCAT44(uStack_98._4_4_,uVar2);
 302	          puStack_a8 = *(undefined8 **)(lVar16 + lVar24);
 303	          lStack_a0 = ((undefined8 *)(lVar16 + lVar24))[1];
 304	          uVar8 = *(undefined4 *)(lVar18 + 0x10 + lVar24);
 305	          uVar11 = *(undefined8 *)(lVar18 + lVar24);
 306	          uVar12 = ((undefined8 *)(lVar18 + lVar24))[1];
 307	          *puVar32 = *(undefined4 *)(lVar16 + 0x10 + lVar24);
 308	          *(undefined8 **)(puVar32 + 1) = puStack_a8;
 309	          *(longlong *)(puVar32 + 3) = lStack_a0;
 310	          puVar32[5] = uVar2;
 311	          uStack_e8 = (undefined4)uVar11;
 312	          uStack_e4 = (undefined4)((ulonglong)uVar11 >> 0x20);
 313	          uStack_e0 = (undefined4)uVar12;
 314	          uStack_dc = (undefined4)((ulonglong)uVar12 >> 0x20);
 315	          puVar32[6] = uStack_e8;
 316	          puVar32[7] = uStack_e4;
 317	          puVar32[8] = uStack_e0;
 318	          puVar32[9] = uStack_dc;
 319	          puVar32[10] = uVar8;
 320	          lVar24 = lVar24 + 0x14;
 321	          puVar32 = puVar32 + 0xb;
 322	          puVar29 = (undefined4 *)((longlong)puVar29 + -1);
 323	        } while (puVar29 != (undefined4 *)0x0);
 324	      }
 325	      if (uStack_80 == 0) {
 326	        FUN_1428d9518(0,0,&UNK_142b2dc90);
 327	      }
 328	      else {
 329	        if (puVar15 + (uVar1 - 1) != (undefined4 *)0x0) {
 330	          uVar2 = *puStack_68;
 331	          uVar8 = puVar15[uVar1 - 1];
 332	          *param_1 = puVar19;
 333	          param_1[1] = puVar27;
 334	          *(undefined4 *)(param_1 + 2) = uVar2;
 335	          *(undefined4 *)((longlong)param_1 + 0x14) = uVar8;
 336	          *(undefined1 *)(param_1 + 3) = 1;
 337	          if (uVar21 != 0) {
 338	            puVar25 = *(undefined8 **)(*(longlong *)(lVar14 + 0x10) + 0x20);
 339	            if (puVar25 == puStack_70) {
 340	              *(undefined4 **)(*(longlong *)(lVar14 + 0x10) + 0x20) =
 341	                   (undefined4 *)((longlong)puVar25 + uVar21 * 4);
 342	            }
 343	          }
 344	          return param_1;
 345	        }
 346	        FUN_1428d9310(&UNK_142b2dca8);
 347	      }
 348	      goto LAB_140c938ad;
 349	    }
 350	  }
 351	  FUN_1428d79d0();
 352	LAB_140c938ad:
 353	                    /* WARNING: Does not return */
 354	  pcVar10 = (code *)invalidInstructionException();
 355	  (*pcVar10)();
 356	}
 357	
