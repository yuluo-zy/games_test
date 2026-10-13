   1	
   2	float ******
   3	FUN_140c94a10(float ******param_1,float ******param_2,ulonglong param_3,float ******param_4,
   4	             float ******param_5,float ******param_6,float ******param_7,float ******param_8,
   5	             float ******param_9,int param_10,undefined4 param_11,int param_12,undefined4 param_13)
   6	
   7	{
   8	  float fVar1;
   9	  float *****pppppfVar2;
  10	  code *pcVar3;
  11	  float ******ppppppfVar4;
  12	  float ******ppppppfVar5;
  13	  float ******ppppppfVar6;
  14	  float ****ppppfVar7;
  15	  float ******unaff_RSI;
  16	  float ******ppppppfVar8;
  17	  float ******unaff_RDI;
  18	  ulonglong uVar9;
  19	  float ******ppppppfVar10;
  20	  float ******in_R10;
  21	  float ****ppppfVar11;
  22	  float ****ppppfVar12;
  23	  float fVar13;
  24	  undefined4 uVar14;
  25	  float *****pppppfStack_118;
  26	  int iStack_108;
  27	  undefined4 uStack_104;
  28	  ulonglong uStack_100;
  29	  int iStack_f8;
  30	  undefined4 uStack_f4;
  31	  float *****pppppfStack_f0;
  32	  float *****pppppfStack_e8;
  33	  float *****pppppfStack_e0;
  34	  float *****pppppfStack_d8;
  35	  float *****pppppfStack_d0;
  36	  float *****pppppfStack_c8;
  37	  float *****pppppfStack_c0;
  38	  float *****pppppfStack_b8;
  39	  float ***pppfStack_b0;
  40	  float *****pppppfStack_a8;
  41	  float *****pppppfStack_a0;
  42	  ulonglong uStack_98;
  43	  float *****pppppfStack_90;
  44	  float *****pppppfStack_88;
  45	  longlong lStack_80;
  46	  float fStack_54;
  47	  float fStack_50;
  48	  float fStack_4c;
  49	  float fStack_48;
  50	  float fStack_44;
  51	  
  52	  ppppppfVar5 = param_9;
  53	  iStack_108 = param_10;
  54	  uStack_104 = param_11;
  55	  iStack_f8 = param_12;
  56	  uStack_f4 = param_13;
  57	  pppppfStack_e0 = (float *****)&iStack_108;
  58	  pppppfStack_d8 = (float *****)param_8;
  59	  pppppfStack_d0 = (float *****)param_9;
  60	  pppppfStack_c8 = (float *****)param_6;
  61	  pppppfStack_c0 = (float *****)param_7;
  62	  pppppfStack_b8 = (float *****)&param_9;
  63	  pppfStack_b0 = (float ***)&iStack_f8;
  64	  pppppfStack_a8 = (float *****)0x0;
  65	  pppppfStack_a0 = (float *****)param_9;
  66	  ppppppfVar10 = param_8;
  67	  uStack_100 = param_3;
  68	  pppppfStack_e8 = (float *****)param_2;
  69	  if ((ulonglong)param_9 >> 0x3d == 0) {
  70	    pppppfVar2 = param_1[2];
  71	    ppppfVar11 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
  72	    if ((ppppfVar11 < *pppppfVar2) ||
  73	       ((ulonglong)((longlong)ppppfVar11 - (longlong)*pppppfVar2) <=
  74	        (ulonglong)((longlong)param_9 * 4) &&
  75	        (longlong)param_9 * 4 - ((longlong)ppppfVar11 - (longlong)*pppppfVar2) != 0)) {
  76	LAB_140c95023:
  77	      ppppfVar11 = (float ****)FUN_1428d7780(param_1);
  78	      if (ppppfVar11 == (float ****)0x0) goto LAB_140c95099;
  79	    }
  80	    else {
  81	      ppppfVar11 = (float ****)((longlong)ppppfVar11 + (longlong)param_9 * -4);
  82	      pppppfVar2[4] = ppppfVar11;
  83	      if (ppppfVar11 == (float ****)0x0) goto LAB_140c95023;
  84	    }
  85	    ppppppfVar8 = unaff_RSI;
  86	    if (ppppppfVar5 != (float ******)0x0) {
  87	      ppppppfVar8 = &pppppfStack_e0;
  88	      unaff_RDI = (float ******)0x0;
  89	      do {
  90	        unaff_RSI = ppppppfVar8;
  91	        if (pppppfStack_a0 <= pppppfStack_a8) goto LAB_140c9509e;
  92	        ppppppfVar6 = (float ******)((longlong)unaff_RDI + 1);
  93	        pppppfStack_a8 = (float *****)((longlong)pppppfStack_a8 + 1);
  94	        fVar13 = (float)FUN_141941180(ppppppfVar8);
  95	        *(float *)((longlong)ppppfVar11 + (longlong)unaff_RDI * 4) = fVar13;
  96	        unaff_RDI = ppppppfVar6;
  97	      } while (ppppppfVar5 != ppppppfVar6);
  98	    }
  99	    ppppppfVar6 = param_9;
 100	    unaff_RSI = ppppppfVar8;
 101	    if ((ulonglong)param_9 >> 0x3d != 0) goto LAB_140c95099;
 102	    unaff_RSI = (float ******)((longlong)param_9 * 4);
 103	    pppppfVar2 = param_1[2];
 104	    ppppfVar7 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
 105	    if ((ppppfVar7 < *pppppfVar2) ||
 106	       ((float ******)((longlong)ppppfVar7 - (longlong)*pppppfVar2) <= unaff_RSI &&
 107	        (longlong)unaff_RSI - ((longlong)ppppfVar7 - (longlong)*pppppfVar2) != 0)) {
 108	LAB_140c9503e:
 109	      ppppfVar7 = (float ****)FUN_1428d7780(param_1);
 110	      ppppppfVar8 = unaff_RSI;
 111	      if (ppppfVar7 == (float ****)0x0) goto LAB_140c95099;
 112	    }
 113	    else {
 114	      ppppfVar7 = (float ****)((longlong)ppppfVar7 + (longlong)param_9 * -4);
 115	      pppppfVar2[4] = ppppfVar7;
 116	      if (ppppfVar7 == (float ****)0x0) goto LAB_140c9503e;
 117	    }
 118	    if (ppppppfVar6 != (float ******)0x0) {
 119	      func_0x000142923c20(ppppfVar7,0,unaff_RSI);
 120	    }
 121	    unaff_RDI = param_9;
 122	    unaff_RSI = ppppppfVar8;
 123	    if ((ulonglong)param_9 >> 0x3d != 0) goto LAB_140c95099;
 124	    unaff_RSI = (float ******)((longlong)param_9 * 4);
 125	    pppppfVar2 = param_1[2];
 126	    ppppfVar12 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
 127	    if ((ppppfVar12 < *pppppfVar2) ||
 128	       ((float ******)((longlong)ppppfVar12 - (longlong)*pppppfVar2) <= unaff_RSI &&
 129	        (longlong)unaff_RSI - ((longlong)ppppfVar12 - (longlong)*pppppfVar2) != 0)) {
 130	LAB_140c9505f:
 131	      ppppfVar12 = (float ****)FUN_1428d7780();
 132	      ppppppfVar8 = unaff_RSI;
 133	      if (ppppfVar12 == (float ****)0x0) goto LAB_140c95099;
 134	    }
 135	    else {
 136	      ppppfVar12 = (float ****)((longlong)ppppfVar12 + (longlong)param_9 * -4);
 137	      pppppfVar2[4] = ppppfVar12;
 138	      if (ppppfVar12 == (float ****)0x0) goto LAB_140c9505f;
 139	    }
 140	    if (unaff_RDI != (float ******)0x0) {
 141	      func_0x000142923c20(ppppfVar12,0,unaff_RSI);
 142	    }
 143	    if (ppppppfVar6 == (float ******)0x0) goto LAB_140c9514a;
 144	    uVar14 = 0;
 145	    if (iStack_108 != 0) {
 146	      uVar14 = 0x3f000000;
 147	    }
 148	    *(undefined4 *)ppppfVar7 = uVar14;
 149	    if (ppppppfVar5 == (float ******)0x0) goto LAB_140c9515a;
 150	    if (iStack_108 == 0) {
 151	      fVar13 = 1.0;
 152	      if (unaff_RDI != (float ******)0x0) goto LAB_140c94c8b;
 153	LAB_140c94e63:
 154	      ppppppfVar10 = (float ******)0x0;
 155	      ppppppfVar6 = (float ******)0x0;
 156	      pppppfStack_f0 = (float *****)param_1;
 157	      fVar13 = (float)FUN_1428d9518(0,0,&UNK_142b2dd50);
 158	LAB_140c94e73:
 159	      if (unaff_RDI <= ppppppfVar8) goto LAB_140c951e2;
 160	      *(int *)((longlong)ppppfVar12 + ((longlong)ppppppfVar10 + -1) * 4) = 0;
 161	      if (ppppppfVar8 == (float ******)0x0) {
 162	LAB_140c94edc:
 163	        pppppfStack_118 = (float *****)ppppppfVar5;
 164	        pppppfStack_e0 = (float *****)param_8;
 165	        pppppfStack_d0 = (float *****)param_6;
 166	        pppppfStack_c8 = (float *****)param_7;
 167	        pppppfStack_b8 = (float *****)param_5;
 168	        pppppfStack_a8 = (float *****)unaff_RDI;
 169	        pppppfStack_a0 = pppppfStack_e8;
 170	        uStack_98 = uStack_100;
 171	        unaff_RSI = ppppppfVar8;
 172	        ppppppfVar10 = (float ******)pppppfStack_f0;
 173	        in_R10 = ppppppfVar5;
 174	        pppppfStack_d8 = pppppfStack_118;
 175	        pppppfStack_c0 = (float *****)param_4;
 176	        pppfStack_b0 = (float ***)ppppfVar12;
 177	        pppppfStack_88 = (float *****)ppppppfVar8;
 178	        if (ppppppfVar8 < (float ******)0x666666666666667) {
 179	          pppppfVar2 = (float *****)pppppfStack_f0[2];
 180	          ppppfVar11 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
 181	          if ((*pppppfVar2 <= ppppfVar11) &&
 182	             ((ulonglong)((longlong)ppppppfVar8 * 0x14) <=
 183	              (ulonglong)((longlong)ppppfVar11 - (longlong)*pppppfVar2))) {
 184	            unaff_RDI = (float ******)((longlong)ppppfVar11 + (longlong)ppppppfVar8 * -0x14);
 185	            pppppfVar2[4] = (float ****)unaff_RDI;
 186	            if (unaff_RDI != (float ******)0x0) goto LAB_140c94fa1;
 187	          }
 188	          unaff_RDI = (float ******)FUN_1428d7780(pppppfStack_f0,4);
 189	          in_R10 = ppppppfVar5;
 190	          if (unaff_RDI != (float ******)0x0) {
 191	LAB_140c94fa1:
 192	            if (ppppppfVar8 != (float ******)0x0) {
 193	              ppppppfVar5 = (float ******)0x0;
 194	              ppppppfVar6 = unaff_RDI;
 195	              do {
 196	                if (ppppppfVar8 <= ppppppfVar5) goto LAB_140c9509e;
 197	                ppppppfVar5 = (float ******)((longlong)ppppppfVar5 + 1);
 198	                pppppfStack_90 = (float *****)ppppppfVar5;
 199	                FUN_141941510(&fStack_54,&pppppfStack_e0);
 200	                *(float *)(ppppppfVar6 + 2) = fStack_44;
 201	                *(float *)ppppppfVar6 = fStack_54;
 202	                *(float *)((longlong)ppppppfVar6 + 4) = fStack_50;
 203	                *(float *)(ppppppfVar6 + 1) = fStack_4c;
 204	                *(float *)((longlong)ppppppfVar6 + 0xc) = fStack_48;
 205	                ppppppfVar6 = (float ******)((longlong)ppppppfVar6 + 0x14);
 206	              } while (ppppppfVar5 != ppppppfVar8);
 207	            }
 208	            return unaff_RDI;
 209	          }
 210	        }
 211	        goto LAB_140c95099;
 212	      }
 213	LAB_140c94e94:
 214	      pppppfStack_118 = (float *****)ppppppfVar5;
 215	      ppppppfVar4 = ppppppfVar8;
 216	      if ((float ******)((longlong)ppppppfVar10 + -2) < ppppppfVar6) {
 217	        do {
 218	          ppppppfVar10 = (float ******)pppppfStack_f0;
 219	          in_R10 = ppppppfVar5;
 220	          if (unaff_RDI <= ppppppfVar4) goto LAB_140c950b6;
 221	          fVar13 = *(float *)((longlong)ppppfVar7 + ((longlong)ppppppfVar4 + -1) * 4) *
 222	                   *(float *)((longlong)ppppfVar12 + (longlong)ppppppfVar4 * 4);
 223	          *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar4 + -1) * 4) =
 224	               *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar4 + -1) * 4) - fVar13;
 225	          ppppppfVar4 = (float ******)((longlong)ppppppfVar4 + -1);
 226	        } while (ppppppfVar4 != (float ******)0x0);
 227	        goto LAB_140c94edc;
 228	      }
 229	      goto LAB_140c951c4;
 230	    }
 231	    in_R10 = param_5;
 232	    if (param_5 == (float ******)0x0) goto LAB_140c9516a;
 233	    fVar13 = *(float *)param_4 + *(float *)param_4;
 234	    if (unaff_RDI == (float ******)0x0) goto LAB_140c94e63;
 235	LAB_140c94c8b:
 236	    pppppfStack_118 = (float *****)ppppppfVar5;
 237	    fVar13 = *(float *)ppppfVar11 / fVar13;
 238	    *(float *)ppppfVar12 = fVar13;
 239	    ppppppfVar8 = (float ******)((longlong)param_9 - 1);
 240	    if ((float ******)0x1 < ppppppfVar8) {
 241	      uVar9 = 2;
 242	      if (2 < uStack_100) {
 243	        uVar9 = uStack_100;
 244	      }
 245	      lStack_80 = (longlong)param_5 + (ulonglong)(param_5 == (float ******)0x0);
 246	      ppppppfVar10 = (float ******)0x0;
 247	      do {
 248	        in_R10 = param_5;
 249	        pppppfStack_f0 = (float *****)param_1;
 250	        if ((float ******)(uVar9 - 2) == ppppppfVar10) goto LAB_140c950fa;
 251	        if (param_5 == ppppppfVar10) goto LAB_140c95112;
 252	        if (ppppppfVar6 == ppppppfVar10) goto LAB_140c950d4;
 253	        if ((float ******)(lStack_80 + -1) == ppppppfVar10) goto LAB_140c950e3;
 254	        if ((float ******)((longlong)ppppppfVar6 + -1) == ppppppfVar10) goto LAB_140c950c5;
 255	        fVar13 = *(float *)((longlong)pppppfStack_e8 + ((longlong)ppppppfVar10 + 2) * 4) -
 256	                 *(float *)((longlong)pppppfStack_e8 + (longlong)ppppppfVar10 * 4);
 257	        fVar1 = *(float *)((longlong)param_4 + (longlong)ppppppfVar10 * 4);
 258	        fVar13 = (fVar13 + fVar13) -
 259	                 *(float *)((longlong)ppppfVar7 + (longlong)ppppppfVar10 * 4) * fVar1;
 260	        *(float *)((longlong)ppppfVar7 + ((longlong)ppppppfVar10 + 1) * 4) =
 261	             *(float *)((longlong)param_4 + ((longlong)ppppppfVar10 + 1) * 4) / fVar13;
 262	        in_R10 = param_4;
 263	        if ((float ******)((longlong)ppppppfVar5 + -1) == ppppppfVar10) goto LAB_140c95124;
 264	        in_R10 = (float ******)0x0;
 265	        if ((1 - (longlong)unaff_RDI) + (longlong)ppppppfVar10 == 0) goto LAB_140c95138;
 266	        *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar10 + 1) * 4) =
 267	             (*(float *)((longlong)ppppfVar11 + ((longlong)ppppppfVar10 + 1) * 4) -
 268	             fVar1 * *(float *)((longlong)ppppfVar12 + (longlong)ppppppfVar10 * 4)) / fVar13;
 269	        ppppppfVar10 = (float ******)((longlong)ppppppfVar10 + 1);
 270	      } while ((float ******)((longlong)param_9 + -2) != ppppppfVar10);
 271	    }
 272	    ppppppfVar10 = param_9;
 273	    pppppfStack_f0 = (float *****)param_1;
 274	    if (iStack_f8 == 0) goto LAB_140c94e73;
 275	    ppppppfVar4 = (float ******)((longlong)param_9 - 2);
 276	    if (param_5 <= ppppppfVar4) goto LAB_140c9517a;
 277	    if (ppppppfVar6 <= ppppppfVar4) goto LAB_140c95191;
 278	    if (ppppppfVar5 <= ppppppfVar8) goto LAB_140c951a0;
 279	    if (unaff_RDI <= ppppppfVar4) goto LAB_140c951b2;
 280	    if (ppppppfVar8 < unaff_RDI) {
 281	      fVar1 = *(float *)((longlong)param_4 + ((longlong)param_9 + -2) * 4);
 282	      fVar13 = (2.0 - *(float *)((longlong)ppppfVar7 + ((longlong)param_9 + -2) * 4)) * fVar1;
 283	      *(float *)((longlong)ppppfVar12 + ((longlong)param_9 + -1) * 4) =
 284	           (*(float *)((longlong)ppppfVar11 + ((longlong)param_9 + -1) * 4) -
 285	           fVar1 * *(float *)((longlong)ppppfVar12 + ((longlong)param_9 + -2) * 4)) / fVar13;
 286	      if (ppppppfVar8 != (float ******)0x0) goto LAB_140c94e94;
 287	      goto LAB_140c94edc;
 288	    }
 289	  }
 290	  else {
 291	LAB_140c95099:
 292	    FUN_1428d79d0();
 293	LAB_140c9509e:
 294	    fVar13 = (float)FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
 295	    ppppppfVar8 = unaff_RSI;
 296	LAB_140c950b6:
 297	    ppppppfVar6 = unaff_RDI;
 298	    FUN_1428d9518(fVar13,unaff_RDI,&UNK_142b2de10);
 299	LAB_140c950c5:
 300	    FUN_1428d9518(ppppppfVar6);
 301	LAB_140c950d4:
 302	    FUN_1428d9518(ppppppfVar10);
 303	LAB_140c950e3:
 304	    FUN_1428d9518(lStack_80,in_R10,&UNK_142b2de70);
 305	LAB_140c950fa:
 306	    ppppppfVar10 = (float ******)((longlong)ppppppfVar10 + 2);
 307	    FUN_1428d9518(ppppppfVar10,uStack_100,&UNK_142b2de28);
 308	LAB_140c95112:
 309	    FUN_1428d9518(ppppppfVar10,in_R10,&UNK_142b2de40);
 310	LAB_140c95124:
 311	    FUN_1428d9518(pppppfStack_118,pppppfStack_118,&UNK_142b2dea0);
 312	LAB_140c95138:
 313	    FUN_1428d9518(unaff_RDI,unaff_RDI,&UNK_142b2deb8);
 314	LAB_140c9514a:
 315	    FUN_1428d9518(0,0,&UNK_142b2dd08);
 316	LAB_140c9515a:
 317	    FUN_1428d9518(0,0,&UNK_142b2dd20);
 318	LAB_140c9516a:
 319	    ppppppfVar5 = in_R10;
 320	    ppppppfVar4 = (float ******)FUN_1428d9518(0,0,&UNK_142b2dd38);
 321	LAB_140c9517a:
 322	    ppppppfVar4 = (float ******)FUN_1428d9518(ppppppfVar4,param_5,&UNK_142b2dd80);
 323	LAB_140c95191:
 324	    FUN_1428d9518(ppppppfVar4);
 325	LAB_140c951a0:
 326	    ppppppfVar4 = (float ******)FUN_1428d9518(ppppppfVar8,ppppppfVar5,&UNK_142b2ddb0);
 327	LAB_140c951b2:
 328	    FUN_1428d9518(ppppppfVar4,unaff_RDI,&UNK_142b2ddc8);
 329	LAB_140c951c4:
 330	    FUN_1428d9518();
 331	  }
 332	  FUN_1428d9518(ppppppfVar8,unaff_RDI,&UNK_142b2dde0);
 333	LAB_140c951e2:
 334	  FUN_1428d9518(ppppppfVar8,unaff_RDI,&UNK_142b2dd68);
 335	  pcVar3 = (code *)swi(3);
 336	  ppppppfVar5 = (float ******)(*pcVar3)();
 337	  return ppppppfVar5;
 338	}
 339	
