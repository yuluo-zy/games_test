   1	
   2	void FUN_14083b480(undefined8 *param_1,undefined8 param_2)
   3	
   4	{
   5	  longlong *plVar1;
   6	  longlong *plVar2;
   7	  longlong lVar3;
   8	  ulonglong uVar4;
   9	  code *pcVar5;
  10	  longlong ***ppplVar6;
  11	  char cVar7;
  12	  undefined2 *puVar8;
  13	  int *piVar9;
  14	  ulonglong uVar10;
  15	  undefined4 uVar11;
  16	  longlong ***ppplStack_2c8;
  17	  undefined8 uStack_2c0;
  18	  undefined8 uStack_2b8;
  19	  ulonglong uStack_2b0;
  20	  ulonglong *puStack_2a8;
  21	  undefined8 uStack_2a0;
  22	  ulonglong **ppuStack_298;
  23	  undefined8 uStack_290;
  24	  undefined8 uStack_288;
  25	  undefined4 uStack_280;
  26	  undefined4 uStack_27c;
  27	  longlong ***ppplStack_278;
  28	  undefined4 uStack_270;
  29	  undefined4 uStack_26c;
  30	  undefined8 uStack_268;
  31	  undefined4 uStack_260;
  32	  undefined4 uStack_258;
  33	  undefined8 uStack_250;
  34	  undefined1 uStack_248;
  35	  undefined4 uStack_243;
  36	  undefined3 uStack_23f;
  37	  undefined2 uStack_238;
  38	  undefined1 uStack_236;
  39	  undefined1 uStack_233;
  40	  longlong **pplStack_f8;
  41	  longlong *plStack_f0;
  42	  code *pcStack_e8;
  43	  longlong ***ppplStack_e0;
  44	  code *pcStack_d8;
  45	  ulonglong **ppuStack_d0;
  46	  undefined8 uStack_c8;
  47	  ulonglong *puStack_c0;
  48	  code *pcStack_b8;
  49	  longlong ***ppplStack_b0;
  50	  undefined *puStack_a8;
  51	  undefined8 uStack_a0;
  52	  undefined8 uStack_98;
  53	  undefined1 uStack_90;
  54	  undefined7 uStack_8f;
  55	  char cStack_6d;
  56	  char cStack_6c;
  57	  undefined8 uStack_60;
  58	  
  59	  uStack_60 = 0xfffffffffffffffe;
  60	  if (*(char *)*param_1 == '\0') {
  61	    uVar11 = *(undefined4 *)param_1[2];
  62	  }
  63	  else {
  64	    uVar11 = 0x3f800000;
  65	  }
  66	  pplStack_f8 = (longlong **)CONCAT44(pplStack_f8._4_4_,uVar11);
  67	  ppplStack_2c8 = &pplStack_f8;
  68	  uStack_2c0 = FUN_140d8c910;
  69	  puStack_c0 = (ulonglong *)0x0;
  70	  pcStack_b8 = (code *)0x2;
  71	  ppplStack_b0 = (longlong ***)0x0;
  72	  puStack_a8 = (undefined *)0x2;
  73	  uStack_a0 = 0;
  74	  uStack_98 = 0x20;
  75	  uStack_90 = 3;
  76	  plStack_f0 = (longlong *)&UNK_142a93528;
  77	  pcStack_e8 = (code *)0x1;
  78	  ppplStack_e0 = (longlong ***)&ppplStack_2c8;
  79	  pcStack_d8 = (code *)0x1;
  80	  ppuStack_d0 = &puStack_c0;
  81	  uStack_c8 = 1;
  82	  FUN_140d79f90(&ppplStack_278,&plStack_f0);
  83	  uStack_248 = 0;
  84	  uStack_243 = 0;
  85	  uStack_23f = 0;
  86	  puStack_2a8 = (ulonglong *)0x3;
  87	  uStack_290 = 6;
  88	  uStack_260 = 0;
  89	  uStack_258 = 0;
  90	  uStack_250 = 0;
  91	  uStack_238 = 0x200;
  92	  uStack_236 = 2;
  93	  uStack_233 = 2;
  94	  FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
  95	  LOCK();
  96	  *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
  97	  UNLOCK();
  98	  if (*ppplStack_b0 == (longlong **)0x0) {
  99	    FUN_140c41170(&ppplStack_b0);
 100	  }
 101	  plVar2 = (longlong *)param_1[1];
 102	  if ((char)*plVar2 == '\0') {
 103	    uVar10 = 0;
 104	  }
 105	  else {
 106	    uVar10 = plVar2[1];
 107	  }
 108	  lVar3 = plVar2[0x1b];
 109	  uVar4 = plVar2[0x1c];
 110	  uStack_2b0 = uVar10;
 111	  puVar8 = (undefined2 *)func_0x000140613c10(2,1);
 112	  if (puVar8 != (undefined2 *)0x0) {
 113	    *puVar8 = 0x3f3f;
 114	    plStack_f0 = (longlong *)0x2;
 115	    ppplStack_e0 = (longlong ***)0x2;
 116	    pplStack_f8 = (longlong **)(lVar3 + uVar10 * 0x18);
 117	    if (uVar4 <= uVar10) {
 118	      pplStack_f8 = &plStack_f0;
 119	    }
 120	    puStack_c0 = &uStack_2b0;
 121	    pcStack_b8 = FUN_140d8d6c0;
 122	    ppplStack_b0 = &pplStack_f8;
 123	    puStack_a8 = &UNK_140053920;
 124	    puStack_2a8 = (ulonglong *)&UNK_142a93548;
 125	    uStack_2a0 = 2;
 126	    uStack_290 = 2;
 127	    uStack_288 = 0;
 128	    ppuStack_298 = &puStack_c0;
 129	    pcStack_e8 = (code *)puVar8;
 130	    FUN_140d79f90(&ppplStack_2c8,&puStack_2a8);
 131	    if (plStack_f0 != (longlong *)0x0) {
 132	      func_0x000140613c20(pcStack_e8,plStack_f0,1);
 133	    }
 134	    uStack_268 = uStack_2b8;
 135	    ppplStack_278 = ppplStack_2c8;
 136	    uStack_270 = (undefined4)uStack_2c0;
 137	    uStack_26c = uStack_2c0._4_4_;
 138	    uStack_248 = 0;
 139	    uStack_243 = 0;
 140	    uStack_23f = 0;
 141	    puStack_2a8 = (ulonglong *)0x3;
 142	    uStack_290 = 6;
 143	    uStack_260 = 0;
 144	    uStack_258 = 0;
 145	    uStack_250 = 0;
 146	    uStack_238 = 0x200;
 147	    uStack_236 = 2;
 148	    uStack_233 = 2;
 149	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 150	    ppplVar6 = ppplStack_b0;
 151	    LOCK();
 152	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 153	    UNLOCK();
 154	    if (*ppplVar6 == (longlong **)0x0) {
 155	      FUN_140c41170(&ppplStack_b0);
 156	    }
 157	    plVar1 = plVar2 + 0x23;
 158	    pcStack_e8 = FUN_14083fec0;
 159	    puStack_c0 = (ulonglong *)&UNK_142a93580;
 160	    pcStack_b8 = (code *)0x1;
 161	    puStack_a8 = (undefined *)0x1;
 162	    uStack_a0 = 0;
 163	    plStack_f0 = plVar1;
 164	    ppplStack_b0 = (longlong ***)&plStack_f0;
 165	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 166	    uStack_248 = 0;
 167	    uStack_243 = 0;
 168	    uStack_23f = 0;
 169	    puStack_2a8 = (ulonglong *)0x3;
 170	    uStack_290 = 6;
 171	    uStack_260 = 0;
 172	    uStack_258 = 0;
 173	    uStack_250 = 0;
 174	    uStack_238 = 0x200;
 175	    uStack_236 = 2;
 176	    uStack_233 = 2;
 177	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 178	    ppplVar6 = ppplStack_b0;
 179	    LOCK();
 180	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 181	    UNLOCK();
 182	    if (*ppplVar6 == (longlong **)0x0) {
 183	      FUN_140c41170(&ppplStack_b0);
 184	    }
 185	    FUN_1409dc370(&puStack_2a8,&UNK_142a93590,0x12);
 186	    FUN_140c46d20(&puStack_c0,&puStack_2a8,param_2);
 187	    if (cStack_6c == '\0') {
 188	      puStack_2a8 = (ulonglong *)((ulonglong)puStack_2a8 & 0xffffffffffffff00);
 189	      if (cStack_6d != '\0') {
 190	        cVar7 = FUN_140042870(&ppplStack_b0,&puStack_2a8);
 191	        if (cVar7 != '\0') goto code_r0x00014083b891;
 192	      }
 193	      ppplVar6 = ppplStack_b0;
 194	      LOCK();
 195	      *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 196	      UNLOCK();
 197	      if (*ppplVar6 == (longlong **)0x0) {
 198	        FUN_140c41170(&ppplStack_b0);
 199	      }
 200	    }
 201	    else {
 202	code_r0x00014083b891:
 203	      ppplVar6 = ppplStack_b0;
 204	      LOCK();
 205	      *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 206	      UNLOCK();
 207	      if (*ppplVar6 == (longlong **)0x0) {
 208	        FUN_140c41170(&ppplStack_b0);
 209	      }
 210	      *(undefined4 *)plVar1 = 3;
 211	    }
 212	    FUN_1409dc370(&puStack_2a8,&UNK_142a935a2,0x11);
 213	    FUN_140c46d20(&puStack_c0,&puStack_2a8,param_2);
 214	    if (cStack_6c == '\0') {
 215	      puStack_2a8 = (ulonglong *)((ulonglong)puStack_2a8 & 0xffffffffffffff00);
 216	      if (cStack_6d != '\0') {
 217	        cVar7 = FUN_140042870(&ppplStack_b0,&puStack_2a8);
 218	        if (cVar7 != '\0') goto code_r0x00014083b921;
 219	      }
 220	      ppplVar6 = ppplStack_b0;
 221	      LOCK();
 222	      *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 223	      UNLOCK();
 224	      if (*ppplVar6 == (longlong **)0x0) {
 225	        FUN_140c41170(&ppplStack_b0);
 226	      }
 227	      goto code_r0x00014083ba04;
 228	    }
 229	code_r0x00014083b921:
 230	    ppplVar6 = ppplStack_b0;
 231	    LOCK();
 232	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 233	    UNLOCK();
 234	    if (*ppplVar6 == (longlong **)0x0) {
 235	      FUN_140c41170(&ppplStack_b0);
 236	    }
 237	    if ((int)plVar2[2] == 1) {
 238	      FUN_141437440(&puStack_c0,plVar2 + 3,1);
 239	      if (puStack_c0 != (ulonglong *)0x8000000000000004) goto code_r0x00014083bec2;
 240	    }
 241	    if (((*plVar2 != 0) && ((ulonglong)plVar2[1] < (ulonglong)plVar2[0x1f])) &&
 242	       (piVar9 = (int *)(plVar2[1] * 0x30 + plVar2[0x1e]), *piVar9 == 1)) {
 243	      FUN_141437440(&puStack_c0,piVar9 + 2,1);
 244	      if (puStack_c0 != (ulonglong *)0x8000000000000004) {
 245	        ppplStack_278 = (longlong ***)CONCAT71(uStack_8f,uStack_90);
 246	        uStack_288 = uStack_a0;
 247	        uStack_280 = (undefined4)uStack_98;
 248	        uStack_27c = uStack_98._4_4_;
 249	        ppuStack_298 = (ulonglong **)ppplStack_b0;
 250	        uStack_290 = puStack_a8;
 251	        puStack_2a8 = puStack_c0;
 252	        uStack_2a0 = pcStack_b8;
 253	        FUN_1428d9760(&UNK_142ad0600,0x2b,&puStack_2a8,&UNK_142ad05c0,&UNK_142ad2c20);
 254	        goto code_r0x00014083bf6c;
 255	      }
 256	    }
 257	    *(undefined4 *)plVar1 = 0;
 258	code_r0x00014083ba04:
 259	    puStack_c0 = (ulonglong *)0x40c00000;
 260	    pcStack_b8 = (code *)CONCAT71(pcStack_b8._1_7_,2);
 261	    FUN_140c29310(&puStack_2a8,&puStack_c0,param_2);
 262	    LOCK();
 263	    *ppuStack_298 = (ulonglong *)((longlong)*ppuStack_298 + -1);
 264	    UNLOCK();
 265	    if (*ppuStack_298 == (ulonglong *)0x0) {
 266	      FUN_140c41170(&ppuStack_298);
 267	    }
 268	    plStack_f0 = (longlong *)0x143634c68;
 269	    pcStack_e8 = FUN_140d8d6c0;
 270	    puStack_c0 = (ulonglong *)&UNK_142a935c0;
 271	    pcStack_b8 = (code *)0x1;
 272	    puStack_a8 = (undefined *)0x1;
 273	    uStack_a0 = 0;
 274	    ppplStack_b0 = (longlong ***)&plStack_f0;
 275	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 276	    uStack_248 = 0;
 277	    uStack_243 = 0;
 278	    uStack_23f = 0;
 279	    puStack_2a8 = (ulonglong *)0x3;
 280	    uStack_290 = 6;
 281	    uStack_260 = 0;
 282	    uStack_258 = 0;
 283	    uStack_250 = 0;
 284	    uStack_238 = 0x200;
 285	    uStack_236 = 2;
 286	    uStack_233 = 2;
 287	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 288	    ppplVar6 = ppplStack_b0;
 289	    LOCK();
 290	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 291	    UNLOCK();
 292	    if (*ppplVar6 == (longlong **)0x0) {
 293	      FUN_140c41170(&ppplStack_b0);
 294	    }
 295	    plStack_f0 = (longlong *)0x143634c88;
 296	    pcStack_e8 = FUN_140d8d6c0;
 297	    puStack_c0 = (ulonglong *)&UNK_142a935e0;
 298	    pcStack_b8 = (code *)0x1;
 299	    puStack_a8 = (undefined *)0x1;
 300	    uStack_a0 = 0;
 301	    ppplStack_b0 = (longlong ***)&plStack_f0;
 302	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 303	    uStack_248 = 0;
 304	    uStack_243 = 0;
 305	    uStack_23f = 0;
 306	    puStack_2a8 = (ulonglong *)0x3;
 307	    uStack_290 = 6;
 308	    uStack_260 = 0;
 309	    uStack_258 = 0;
 310	    uStack_250 = 0;
 311	    uStack_238 = 0x200;
 312	    uStack_236 = 2;
 313	    uStack_233 = 2;
 314	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 315	    ppplVar6 = ppplStack_b0;
 316	    LOCK();
 317	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 318	    UNLOCK();
 319	    if (*ppplVar6 == (longlong **)0x0) {
 320	      FUN_140c41170(&ppplStack_b0);
 321	    }
 322	    plStack_f0 = (longlong *)0x143634c90;
 323	    pcStack_e8 = FUN_140d8d6c0;
 324	    puStack_c0 = (ulonglong *)&UNK_142a93600;
 325	    pcStack_b8 = (code *)0x1;
 326	    puStack_a8 = (undefined *)0x1;
 327	    uStack_a0 = 0;
 328	    ppplStack_b0 = (longlong ***)&plStack_f0;
 329	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 330	    uStack_248 = 0;
 331	    uStack_243 = 0;
 332	    uStack_23f = 0;
 333	    puStack_2a8 = (ulonglong *)0x3;
 334	    uStack_290 = 6;
 335	    uStack_260 = 0;
 336	    uStack_258 = 0;
 337	    uStack_250 = 0;
 338	    uStack_238 = 0x200;
 339	    uStack_236 = 2;
 340	    uStack_233 = 2;
 341	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 342	    ppplVar6 = ppplStack_b0;
 343	    LOCK();
 344	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 345	    UNLOCK();
 346	    if (*ppplVar6 == (longlong **)0x0) {
 347	      FUN_140c41170(&ppplStack_b0);
 348	    }
 349	    plStack_f0 = (longlong *)0x143634c70;
 350	    pcStack_e8 = FUN_140d8c910;
 351	    puStack_c0 = (ulonglong *)&UNK_142a93618;
 352	    pcStack_b8 = (code *)0x1;
 353	    puStack_a8 = (undefined *)0x1;
 354	    uStack_a0 = 0;
 355	    ppplStack_b0 = (longlong ***)&plStack_f0;
 356	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 357	    uStack_248 = 0;
 358	    uStack_243 = 0;
 359	    uStack_23f = 0;
 360	    puStack_2a8 = (ulonglong *)0x3;
 361	    uStack_290 = 6;
 362	    uStack_260 = 0;
 363	    uStack_258 = 0;
 364	    uStack_250 = 0;
 365	    uStack_238 = 0x200;
 366	    uStack_236 = 2;
 367	    uStack_233 = 2;
 368	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 369	    ppplVar6 = ppplStack_b0;
 370	    LOCK();
 371	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 372	    UNLOCK();
 373	    if (*ppplVar6 == (longlong **)0x0) {
 374	      FUN_140c41170(&ppplStack_b0);
 375	    }
 376	    plStack_f0 = (longlong *)0x143634c78;
 377	    pcStack_e8 = FUN_140d8d6c0;
 378	    ppplStack_e0 = (longlong ***)0x143634c80;
 379	    pcStack_d8 = FUN_140d8d6c0;
 380	    puStack_c0 = (ulonglong *)&UNK_142a93640;
 381	    pcStack_b8 = (code *)0x2;
 382	    puStack_a8 = (undefined *)0x2;
 383	    uStack_a0 = 0;
 384	    ppplStack_b0 = (longlong ***)&plStack_f0;
 385	    FUN_140d79f90(&ppplStack_278,&puStack_c0);
 386	    uStack_248 = 0;
 387	    uStack_243 = 0;
 388	    uStack_23f = 0;
 389	    puStack_2a8 = (ulonglong *)0x3;
 390	    uStack_290 = 6;
 391	    uStack_260 = 0;
 392	    uStack_258 = 0;
 393	    uStack_250 = 0;
 394	    uStack_238 = 0x200;
 395	    uStack_236 = 2;
 396	    uStack_233 = 2;
 397	    FUN_140c28d40(&puStack_c0,&puStack_2a8,param_2);
 398	    ppplVar6 = ppplStack_b0;
 399	    LOCK();
 400	    *ppplStack_b0 = (longlong **)((longlong)*ppplStack_b0 + -1);
 401	    UNLOCK();
 402	    if (*ppplVar6 == (longlong **)0x0) {
 403	      FUN_140c41170(&ppplStack_b0);
 404	    }
 405	    puStack_c0 = (ulonglong *)0x40c00000;
 406	    pcStack_b8 = (code *)CONCAT71(pcStack_b8._1_7_,2);
 407	    FUN_140c29310(&puStack_2a8,&puStack_c0,param_2);
 408	    LOCK();
 409	    *ppuStack_298 = (ulonglong *)((longlong)*ppuStack_298 + -1);
 410	    UNLOCK();
 411	    if (*ppuStack_298 == (ulonglong *)0x0) {
 412	      FUN_140c41170(&ppuStack_298);
 413	    }
 414	    return;
 415	  }
 416	  FUN_1428d8fe3(1,2,&UNK_142a93490);
 417	code_r0x00014083bec2:
 418	  ppplStack_278 = (longlong ***)CONCAT71(uStack_8f,uStack_90);
 419	  uStack_288 = uStack_a0;
 420	  uStack_280 = (undefined4)uStack_98;
 421	  uStack_27c = uStack_98._4_4_;
 422	  ppuStack_298 = (ulonglong **)ppplStack_b0;
 423	  uStack_290 = puStack_a8;
 424	  puStack_2a8 = puStack_c0;
 425	  uStack_2a0 = pcStack_b8;
 426	  FUN_1428d9760(&UNK_142ad0600,0x2b,&puStack_2a8,&UNK_142ad05c0,&UNK_142ad2c20);
 427	code_r0x00014083bf6c:
 428	                    /* WARNING: Does not return */
 429	  pcVar5 = (code *)invalidInstructionException();
 430	  (*pcVar5)();
 431	}
 432	
