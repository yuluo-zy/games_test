   1	
   2	void FUN_142341fe0(longlong *param_1,undefined8 *param_2,longlong *param_3,undefined8 *param_4,
   3	                  undefined8 *param_5,longlong param_6)
   4	
   5	{
   6	  undefined4 *puVar1;
   7	  char cVar2;
   8	  undefined4 uVar3;
   9	  uint uVar4;
  10	  uint uVar5;
  11	  ulonglong *puVar6;
  12	  longlong lVar7;
  13	  longlong lVar8;
  14	  undefined4 *puVar9;
  15	  longlong lVar10;
  16	  undefined8 uVar11;
  17	  undefined4 uVar12;
  18	  undefined4 uVar13;
  19	  undefined4 uVar14;
  20	  undefined4 uVar15;
  21	  ulonglong uVar16;
  22	  longlong lVar17;
  23	  ulonglong uVar18;
  24	  uint *puVar19;
  25	  longlong *plVar20;
  26	  uint *puVar21;
  27	  longlong lVar22;
  28	  longlong lVar23;
  29	  uint *puVar24;
  30	  longlong lVar25;
  31	  longlong lVar26;
  32	  ulonglong uVar27;
  33	  uint *puVar28;
  34	  undefined8 *puVar29;
  35	  ulonglong uVar30;
  36	  byte bVar31;
  37	  uint *puVar32;
  38	  longlong lVar33;
  39	  byte bVar34;
  40	  uint *puVar35;
  41	  longlong lVar36;
  42	  uint *puVar37;
  43	  bool bVar38;
  44	  bool bVar39;
  45	  bool bVar40;
  46	  bool bVar41;
  47	  longlong lStack_b8;
  48	  longlong lStack_a8;
  49	  longlong lStack_98;
  50	  longlong lStack_88;
  51	  
  52	  puVar6 = (ulonglong *)*param_2;
  53	  lVar7 = param_2[1];
  54	  uVar18 = *puVar6;
  55	  uVar30 = 0;
  56	  uVar16 = uVar18 - *(ulonglong *)(lVar7 + 0x18);
  57	  if (uVar18 < *(ulonglong *)(lVar7 + 0x18)) {
  58	    uVar16 = uVar30;
  59	  }
  60	  uVar27 = uVar18 - *(ulonglong *)(lVar7 + 0x38);
  61	  if (uVar18 < *(ulonglong *)(lVar7 + 0x38)) {
  62	    uVar27 = uVar30;
  63	  }
  64	  lVar25 = uVar16 * 0x10 + *(longlong *)(lVar7 + 8);
  65	  uVar18 = *(ulonglong *)(lVar7 + 0x10) - uVar16;
  66	  if (*(ulonglong *)(lVar7 + 0x10) < uVar16) {
  67	    lVar25 = 8;
  68	    uVar18 = uVar30;
  69	  }
  70	  lVar17 = uVar27 * 0x10 + *(longlong *)(lVar7 + 0x28);
  71	  uVar16 = *(ulonglong *)(lVar7 + 0x30) - uVar27;
  72	  if (*(ulonglong *)(lVar7 + 0x30) < uVar27) {
  73	    lVar17 = 8;
  74	    uVar16 = uVar30;
  75	  }
  76	  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar16) - uVar18;
  77	  lVar23 = uVar16 * 0x10 + lVar17;
  78	  lVar7 = *param_1;
  79	  lVar8 = param_1[1];
  80	  uVar3 = *(undefined4 *)((longlong)param_3 + 0x1c);
  81	  lVar33 = *param_3;
  82	  puVar9 = (undefined4 *)param_3[2];
  83	  lVar36 = lVar25;
  84	  do {
  85	    if ((lVar36 == 0) || (lVar36 == uVar18 * 0x10 + lVar25)) {
  86	      if ((lVar17 == 0) || (lVar17 == lVar23)) break;
  87	      lVar22 = 0;
  88	      lVar10 = lVar17 + 0x10;
  89	      lVar26 = lVar17;
  90	    }
  91	    else {
  92	      lVar22 = lVar36 + 0x10;
  93	      lVar10 = lVar17;
  94	      lVar26 = lVar36;
  95	    }
  96	    lVar17 = lVar10;
  97	    *puVar6 = *puVar6 + 1;
  98	    uVar16 = *(ulonglong *)(lVar26 + 8) & 0xffffffff;
  99	    lVar36 = lVar22;
 100	    if ((uVar16 < *(ulonglong *)(lVar8 + 0x10)) &&
 101	       (*(int *)(*(longlong *)(lVar8 + 8) + uVar16 * 0x14) ==
 102	        (int)(*(ulonglong *)(lVar26 + 8) >> 0x20))) {
 103	      lVar22 = *(longlong *)(lVar8 + 8) + uVar16 * 0x14;
 104	      uVar16 = (ulonglong)*(uint *)(lVar22 + 4);
 105	      if ((uVar16 != 0xffffffff) &&
 106	         ((uVar16 < *(ulonglong *)(lVar7 + 0x38) &&
 107	          ((*(ulonglong *)
 108	             (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar22 + 4) >> 6) * 8) >>
 109	            (uVar16 & 0x3f) & 1) != 0)))) {
 110	        puVar1 = (undefined4 *)
 111	                 (*(longlong *)
 112	                   (*(longlong *)
 113	                     (*(longlong *)(lVar8 + 0x1a8) + 0x18 +
 114	                     (ulonglong)*(uint *)(lVar22 + 0xc) * 0x48) + 0x10 +
 115	                   ~*(ulonglong *)
 116	                     (*(longlong *)
 117	                       (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
 118	                       (ulonglong)*(uint *)(lVar22 + 0xc) * 0x48) + *(longlong *)(lVar7 + 0x110) * 8
 119	                     ) * 0x30) + (ulonglong)*(uint *)(lVar22 + 0x10) * 0x10);
 120	        uVar12 = *puVar1;
 121	        uVar13 = puVar1[1];
 122	        uVar14 = puVar1[2];
 123	        uVar15 = puVar1[3];
 124	        *puVar9 = uVar3;
 125	        lVar22 = *(longlong *)(lVar33 + 0x30);
 126	        uVar11 = *(undefined8 *)(lVar33 + 0x40);
 127	        if (lVar22 == *(longlong *)(lVar33 + 0x20)) {
 128	          FUN_140562db0(lVar33 + 0x20,&UNK_142bb5270);
 129	        }
 130	        lVar10 = *(longlong *)(lVar33 + 0x28);
 131	        puVar1 = (undefined4 *)(lVar10 + lVar22 * 0x18);
 132	        *puVar1 = uVar12;
 133	        puVar1[1] = uVar13;
 134	        puVar1[2] = uVar14;
 135	        puVar1[3] = uVar15;
 136	        *(undefined8 *)(lVar10 + 0x10 + lVar22 * 0x18) = uVar11;
 137	        *(longlong *)(lVar33 + 0x30) = lVar22 + 1;
 138	        *(longlong *)(lVar33 + 0x40) = *(longlong *)(lVar33 + 0x40) + 1;
 139	      }
 140	    }
 141	  } while( true );
 142	  lVar25 = param_5[1];
 143	  uVar18 = *(ulonglong *)*param_5;
 144	  puVar37 = (uint *)0x0;
 145	  puVar35 = (uint *)(uVar18 - *(ulonglong *)(lVar25 + 0x18));
 146	  if (uVar18 < *(ulonglong *)(lVar25 + 0x18)) {
 147	    puVar35 = puVar37;
 148	  }
 149	  puVar32 = (uint *)(uVar18 - *(ulonglong *)(lVar25 + 0x38));
 150	  if (uVar18 < *(ulonglong *)(lVar25 + 0x38)) {
 151	    puVar32 = puVar37;
 152	  }
 153	  lVar17 = (longlong)puVar35 * 0x28 + *(longlong *)(lVar25 + 8);
 154	  lVar33 = (longlong)puVar32 * 0x28 + *(longlong *)(lVar25 + 0x28);
 155	  bVar38 = *(uint **)(lVar25 + 0x10) < puVar35;
 156	  puVar35 = (uint *)((longlong)*(uint **)(lVar25 + 0x10) - (longlong)puVar35);
 157	  bVar40 = puVar35 == (uint *)0x0;
 158	  if (bVar38) {
 159	    puVar35 = puVar37;
 160	  }
 161	  lVar36 = 8;
 162	  if (bVar38) {
 163	    lVar17 = 8;
 164	  }
 165	  puVar28 = (uint *)(lVar17 + -0x28 + (longlong)puVar35 * 0x28);
 166	  if (bVar38 || bVar40) {
 167	    puVar28 = puVar37;
 168	  }
 169	  bVar39 = *(uint **)(lVar25 + 0x30) < puVar32;
 170	  puVar32 = (uint *)((longlong)*(uint **)(lVar25 + 0x30) - (longlong)puVar32);
 171	  bVar41 = puVar32 == (uint *)0x0;
 172	  if (bVar39) {
 173	    lVar33 = 8;
 174	    puVar32 = puVar37;
 175	  }
 176	  puVar21 = (uint *)(lVar33 + -0x28 + (longlong)puVar32 * 0x28);
 177	  if (bVar39 || bVar41) {
 178	    puVar21 = puVar28;
 179	  }
 180	  puVar35 = (uint *)((longlong)puVar32 + (longlong)puVar35);
 181	  if (!bVar39 && !bVar41 || !bVar38 && !bVar40) {
 182	    puVar35 = puVar37;
 183	  }
 184	  *(ulonglong *)*param_5 = *(longlong *)(lVar25 + 0x40) - (longlong)puVar35;
 185	  puVar35 = *(uint **)(lVar7 + 0xf8);
 186	  plVar20 = *(longlong **)(param_6 + 8);
 187	  if (plVar20 == (longlong *)0x0) {
 188	    plVar20 = *(longlong **)(param_6 + 0x10);
 189	  }
 190	  uVar3 = *(undefined4 *)((longlong)param_1 + 0x14);
 191	  puVar32 = puVar35 + *(longlong *)(lVar7 + 0x100);
 192	  cVar2 = *(char *)(lVar7 + 0x108);
 193	  puVar28 = (uint *)*param_4;
 194	  lStack_b8 = 0;
 195	  lStack_88 = 8;
 196	  lStack_a8 = 0;
 197	  puVar24 = (uint *)0x0;
 198	code_r0x00014234234b:
 199	  do {
 200	    do {
 201	      if (cVar2 == '\0') {
 202	        if (puVar24 == puVar37) {
 203	          if (puVar35 == puVar32) {
 204	            return;
 205	          }
 206	          while( true ) {
 207	            puVar37 = *(uint **)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar35 * 0xa0);
 208	            if (puVar37 != (uint *)0x0) break;
 209	            puVar35 = puVar35 + 1;
 210	            if (puVar35 == puVar32) {
 211	              return;
 212	            }
 213	          }
 214	          lVar36 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar35 * 0xa0;
 215	          uVar18 = (ulonglong)*(uint *)(lVar36 + 0x94);
 216	          lVar17 = *(longlong *)(lVar8 + 0x1a8);
 217	          lVar25 = lVar17 + uVar18 * 0x48;
 218	          lVar33 = *(longlong *)(lVar17 + 0x38 + uVar18 * 0x48);
 219	          if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar17 + 0x40 + uVar18 * 0x48)) {
 220	            uVar18 = *(ulonglong *)(lVar33 + *(ulonglong *)(lVar7 + 0x110) * 8);
 221	            lVar25 = *(longlong *)(lVar25 + 0x18);
 222	            if (uVar18 == 0) goto code_r0x0001423424b2;
 223	            lStack_b8 = *(longlong *)(lVar25 + 0x10 + ~uVar18 * 0x30);
 224	          }
 225	          else {
 226	            lVar25 = *(longlong *)(lVar25 + 0x18);
 227	code_r0x0001423424b2:
 228	            lStack_b8 = 0;
 229	          }
 230	          puVar35 = puVar35 + 1;
 231	          lVar17 = ~*(ulonglong *)(lVar33 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
 232	          lStack_a8 = *(longlong *)(lVar25 + 0x10 + lVar17);
 233	          lStack_98 = *(longlong *)(lVar25 + 0x28 + lVar17);
 234	          lVar36 = *(longlong *)(lVar36 + 0x50);
 235	          puVar24 = (uint *)0x0;
 236	        }
 237	        puVar29 = (undefined8 *)((longlong)puVar24 * 0x10 + lVar36);
 238	        uVar18 = (ulonglong)*(uint *)(lVar36 + 8 + (longlong)puVar24 * 0x10);
 239	      }
 240	      else {
 241	        if (puVar24 == puVar37) {
 242	          if (puVar35 == puVar32) {
 243	            return;
 244	          }
 245	          while( true ) {
 246	            puVar37 = *(uint **)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar35 * 0x48);
 247	            if (puVar37 != (uint *)0x0) break;
 248	            puVar35 = puVar35 + 1;
 249	            if (puVar35 == puVar32) {
 250	              return;
 251	            }
 252	          }
 253	          lVar25 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar35 * 0x48;
 254	          if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar25 + 0x40)) {
 255	            uVar18 = *(ulonglong *)
 256	                      (*(longlong *)(lVar25 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
 257	            lVar17 = *(longlong *)(lVar25 + 0x18);
 258	            if (uVar18 == 0) goto code_r0x00014234245f;
 259	            lStack_b8 = *(longlong *)(lVar17 + 0x10 + ~uVar18 * 0x30);
 260	          }
 261	          else {
 262	            lVar17 = *(longlong *)(lVar25 + 0x18);
 263	code_r0x00014234245f:
 264	            lStack_b8 = 0;
 265	          }
 266	          puVar35 = puVar35 + 1;
 267	          lVar33 = ~*(ulonglong *)(*(longlong *)(lVar25 + 0x38) + *(longlong *)(lVar7 + 0x118) * 8)
 268	                   * 0x30;
 269	          lStack_a8 = *(longlong *)(lVar17 + 0x10 + lVar33);
 270	          lStack_98 = *(longlong *)(lVar17 + 0x28 + lVar33);
 271	          lStack_88 = *(longlong *)(lVar25 + 8);
 272	          puVar24 = (uint *)0x0;
 273	        }
 274	        puVar29 = (undefined8 *)(lStack_88 + (longlong)puVar24 * 8);
 275	        uVar18 = (ulonglong)puVar24 & 0xffffffff;
 276	      }
 277	      puVar19 = (uint *)(uVar18 * 0x10 + lStack_b8);
 278	      uVar11 = *puVar29;
 279	      uVar4 = *puVar19;
 280	      uVar30 = 0;
 281	      uVar16 = (ulonglong)uVar4 - 5;
 282	      if (0xe < uVar4 - 6) {
 283	        uVar16 = uVar30;
 284	      }
 285	      uVar5 = *puVar28;
 286	      uVar27 = (ulonglong)uVar5 - 5;
 287	      if ((ulonglong)uVar5 < 6) {
 288	        uVar27 = uVar30;
 289	      }
 290	      if ((uVar16 == uVar27) && (uVar30 = 1, true)) {
 291	        switch(uVar16) {
 292	        case 0:
 293	          if (uVar5 < 6) {
 294	            if (uVar4 == uVar5) {
 295	                    /* WARNING: Could not recover jumptable at 0x0001423425b1. Too many branches */
 296	                    /* WARNING: Treating indirect jump as call */
 297	              (*(code *)(&UNK_14304e2b0 + *(int *)(&UNK_14304e2b0 + (ulonglong)uVar4 * 4)))();
 298	              return;
 299	            }
 300	code_r0x000142342627:
 301	            uVar30 = 0;
 302	          }
 303	          break;
 304	        case 1:
 305	        case 4:
 306	          uVar30 = (ulonglong)((char)puVar19[1] == (char)puVar28[1]);
 307	          break;
 308	        case 3:
 309	          uVar30 = (ulonglong)(*(longlong *)(puVar19 + 2) == *(longlong *)(puVar28 + 2));
 310	          break;
 311	        case 6:
 312	          bVar31 = (char)puVar19[1] - 0x38;
 313	          if (7 < bVar31) {
 314	            bVar31 = 6;
 315	          }
 316	          bVar34 = (char)puVar28[1] - 0x38;
 317	          if (7 < bVar34) {
 318	            bVar34 = 6;
 319	          }
 320	          if (bVar31 != bVar34) goto code_r0x000142342627;
 321	          uVar30 = (ulonglong)((char)puVar19[1] == (char)puVar28[1] || bVar31 != 6);
 322	        }
 323	      }
 324	      puVar24 = (uint *)((longlong)puVar24 + 1);
 325	      *(undefined4 *)(lStack_98 + uVar18 * 4) = uVar3;
 326	      *(char *)(uVar18 * 3 + lStack_a8 + 2) = (char)uVar30;
 327	    } while ((bVar39 || bVar41) && (bVar38 || bVar40));
 328	    uVar4 = *puVar19;
 329	    lVar25 = (ulonglong)uVar4 - 5;
 330	    if (0xe < uVar4 - 6) {
 331	      lVar25 = 0;
 332	    }
 333	    uVar5 = *puVar21;
 334	    lVar17 = (ulonglong)uVar5 - 5;
 335	    if ((ulonglong)uVar5 < 6) {
 336	      lVar17 = 0;
 337	    }
 338	  } while (lVar25 != lVar17);
 339	  if (false) goto code_r0x000142342748;
 340	  switch(lVar25) {
 341	  case 0:
 342	    if (5 < uVar5) break;
 343	    if (uVar4 == uVar5) {
 344	                    /* WARNING: Could not recover jumptable at 0x0001423426d2. Too many branches */
 345	                    /* WARNING: Treating indirect jump as call */
 346	      (*(code *)(&UNK_14304e2e4 + *(int *)(&UNK_14304e2e4 + (ulonglong)uVar4 * 4)))
 347	                (&UNK_14304e2e4 + *(int *)(&UNK_14304e2e4 + (ulonglong)uVar4 * 4));
 348	      return;
 349	    }
 350	    goto code_r0x00014234234b;
 351	  case 1:
 352	  case 4:
 353	    if ((char)puVar19[1] != (char)puVar21[1]) goto code_r0x00014234234b;
 354	    break;
 355	  case 3:
 356	    if (*(longlong *)(puVar19 + 2) != *(longlong *)(puVar21 + 2)) goto code_r0x00014234234b;
 357	    break;
 358	  case 6:
 359	    bVar31 = (char)puVar19[1] - 0x38;
 360	    if (7 < bVar31) {
 361	      bVar31 = 6;
 362	    }
 363	    bVar34 = (char)puVar21[1] - 0x38;
 364	    if (7 < bVar34) {
 365	      bVar34 = 6;
 366	    }
 367	    if ((bVar31 != bVar34) || ((char)puVar19[1] != (char)puVar21[1] && bVar31 == 6))
 368	    goto code_r0x00014234234b;
 369	  }
 370	code_r0x000142342748:
 371	  lVar25 = plVar20[2];
 372	  if ((ulonglong)(*plVar20 - lVar25) < 0x28) {
 373	    FUN_1428a58d0(plVar20,lVar25,0x28,1,1);
 374	  }
 375	  lVar17 = plVar20[1];
 376	  *(code **)(lVar17 + lVar25) = FUN_140fdf200;
 377	  *(code **)(lVar17 + 8 + lVar25) = FUN_141e4f860;
 378	  *(undefined8 *)(lVar17 + 0x10 + lVar25) = uVar11;
 379	  *(undefined8 *)(lVar17 + 0x18 + lVar25) = 0x3e4ccccd3e4ccccd;
 380	  *(undefined4 *)(lVar17 + 0x20 + lVar25) = 0;
 381	  *(undefined1 *)(lVar17 + 0x24 + lVar25) = 0;
 382	  plVar20[2] = lVar25 + 0x28;
 383	  goto code_r0x00014234234b;
 384	}
 385	