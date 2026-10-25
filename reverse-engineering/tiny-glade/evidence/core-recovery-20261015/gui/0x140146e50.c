   1	
   2	void FUN_140146e50(undefined8 *param_1,undefined8 param_2)
   3	
   4	{
   5	  float *pfVar1;
   6	  undefined8 *puVar2;
   7	  undefined8 uVar3;
   8	  code *pcVar4;
   9	  longlong *plVar5;
  10	  char cVar6;
  11	  undefined8 *puVar7;
  12	  undefined2 *puVar8;
  13	  uint uVar9;
  14	  longlong lVar10;
  15	  float fVar11;
  16	  float fVar12;
  17	  undefined8 uStack_410;
  18	  longlong lStack_408;
  19	  undefined8 uStack_3c8;
  20	  ulonglong uStack_3c0;
  21	  ulonglong uStack_3b8;
  22	  undefined8 uStack_3b0;
  23	  undefined8 uStack_3a8;
  24	  undefined8 uStack_3a0;
  25	  undefined4 uStack_2c8;
  26	  undefined4 uStack_2c4;
  27	  undefined4 uStack_2c0;
  28	  undefined4 uStack_2bc;
  29	  undefined4 uStack_2b8;
  30	  undefined4 uStack_2b4;
  31	  undefined4 uStack_2b0;
  32	  undefined4 uStack_2ac;
  33	  undefined4 uStack_2a8;
  34	  undefined4 uStack_2a4;
  35	  undefined4 uStack_2a0;
  36	  undefined4 uStack_29c;
  37	  undefined4 uStack_298;
  38	  undefined4 uStack_294;
  39	  undefined4 uStack_290;
  40	  undefined4 uStack_28c;
  41	  longlong lStack_288;
  42	  ulonglong uStack_280;
  43	  ulonglong uStack_278;
  44	  undefined8 uStack_270;
  45	  undefined8 uStack_268;
  46	  undefined2 *puStack_260;
  47	  undefined4 uStack_258;
  48	  undefined4 uStack_254;
  49	  undefined3 uStack_250;
  50	  longlong *aplStack_248 [8];
  51	  char cStack_205;
  52	  char cStack_204;
  53	  undefined8 uStack_1f8;
  54	  undefined4 uStack_1f0;
  55	  undefined4 uStack_1ec;
  56	  undefined8 uStack_1e8;
  57	  undefined4 uStack_1e0;
  58	  undefined4 uStack_1dc;
  59	  undefined8 uStack_1d8;
  60	  undefined8 uStack_1d0;
  61	  undefined8 uStack_1c8;
  62	  undefined8 uStack_1c0;
  63	  undefined8 uStack_1b8;
  64	  undefined8 uStack_1b0;
  65	  undefined4 uStack_1a8;
  66	  undefined4 uStack_1a4;
  67	  undefined4 uStack_1a0;
  68	  undefined4 uStack_19c;
  69	  undefined8 uStack_198;
  70	  undefined2 *puStack_190;
  71	  undefined8 uStack_188;
  72	  ulonglong uStack_180;
  73	  ulonglong uStack_178;
  74	  undefined8 uStack_170;
  75	  undefined1 uStack_168;
  76	  undefined7 uStack_167;
  77	  undefined4 uStack_160;
  78	  undefined4 uStack_15c;
  79	  undefined8 uStack_158;
  80	  undefined8 uStack_150;
  81	  undefined8 uStack_148;
  82	  undefined8 uStack_140;
  83	  undefined8 uStack_138;
  84	  undefined8 uStack_130;
  85	  undefined4 uStack_128;
  86	  undefined8 uStack_120;
  87	  undefined8 uStack_118;
  88	  undefined1 uStack_110;
  89	  undefined4 uStack_108;
  90	  undefined4 uStack_104;
  91	  undefined4 uStack_100;
  92	  undefined4 uStack_fc;
  93	  undefined1 uStack_f8;
  94	  undefined8 *puStack_f0;
  95	  undefined *puStack_e8;
  96	  undefined8 uStack_e0;
  97	  undefined8 uStack_d8;
  98	  undefined8 uStack_c8;
  99	  undefined4 uStack_b8;
 100	  undefined1 uStack_b4;
 101	  undefined8 *puStack_50;
 102	  undefined1 uStack_41;
 103	  undefined8 uStack_40;
 104	  
 105	  uStack_40 = 0xfffffffffffffffe;
 106	  pfVar1 = (float *)*param_1;
 107	  puVar2 = (undefined8 *)param_1[1];
 108	  puVar7 = (undefined8 *)func_0x000140613c10(8,8);
 109	  if (puVar7 == (undefined8 *)0x0) {
 110	    FUN_1428d9000(8,8);
 111	code_r0x00014014739e:
 112	    FUN_1428d8fe3(1,3,&UNK_142951128);
 113	                    /* WARNING: Does not return */
 114	    pcVar4 = (code *)invalidInstructionException();
 115	    (*pcVar4)();
 116	  }
 117	  *puVar7 = pfVar1;
 118	  puStack_50 = puVar7;
 119	  FUN_140c34750(&uStack_2c8);
 120	  uStack_158 = 0;
 121	  uStack_150 = 1;
 122	  uStack_148 = 0;
 123	  uStack_140 = 0;
 124	  uStack_138 = 1;
 125	  uStack_130 = 0;
 126	  lStack_408 = CONCAT44(uStack_2c4,uStack_2c8);
 127	  puStack_190 = (undefined2 *)CONCAT44(uStack_28c,uStack_290);
 128	  uStack_3c8 = lStack_288;
 129	  uStack_3c0 = uStack_280;
 130	  uStack_3b8 = uStack_278;
 131	  uStack_3b0 = uStack_270;
 132	  uStack_3a8 = uStack_268;
 133	  uStack_3a0 = puStack_260;
 134	  uStack_e0 = 0;
 135	  uStack_d8 = 0;
 136	  uStack_1f8 = 0;
 137	  uStack_1e8 = (longlong *)0x0;
 138	  uStack_1d8 = 0;
 139	  uStack_1d0 = uStack_410;
 140	  uStack_1c0 = (longlong *)CONCAT44(uStack_2bc,uStack_2c0);
 141	  uStack_1b8 = CONCAT44(uStack_2b4,uStack_2b8);
 142	  uStack_1b0 = CONCAT44(uStack_2ac,uStack_2b0);
 143	  uStack_1a8 = uStack_2a8;
 144	  uStack_1a4 = uStack_2a4;
 145	  uStack_1a0 = uStack_2a0;
 146	  uStack_19c = uStack_29c;
 147	  uStack_198 = CONCAT44(uStack_294,uStack_298);
 148	  uStack_188 = lStack_288;
 149	  uStack_180 = uStack_280;
 150	  uStack_178 = uStack_278;
 151	  uStack_170 = uStack_270;
 152	  uStack_168 = (undefined1)uStack_268;
 153	  uStack_167 = (undefined7)((ulonglong)uStack_268 >> 8);
 154	  uStack_160 = SUB84(puStack_260,0);
 155	  uStack_15c = (undefined4)((ulonglong)puStack_260 >> 0x20);
 156	  uStack_128 = 2;
 157	  uStack_120 = 0x4024000000000000;
 158	  uStack_118 = 0x4054000000000000;
 159	  uStack_110 = 0;
 160	  uStack_108 = 0xa0b5ed8d;
 161	  uStack_104 = 0x3eb0c6f7;
 162	  uStack_100 = 0;
 163	  uStack_fc = 0x7ff00000;
 164	  uStack_f8 = 0;
 165	  puStack_f0 = puStack_50;
 166	  puStack_e8 = &UNK_142a02098;
 167	  uStack_c8 = 0;
 168	  uStack_b8 = 0x10101;
 169	  uStack_b4 = 2;
 170	  uStack_1c8 = lStack_408;
 171	  puVar8 = (undefined2 *)func_0x000140613c10(3,1);
 172	  if (puVar8 == (undefined2 *)0x0) goto code_r0x00014014739e;
 173	  *(undefined1 *)(puVar8 + 1) = 0x56;
 174	  *puVar8 = 0x6f46;
 175	  uStack_250 = 0;
 176	  uStack_254 = 0;
 177	  uVar9 = (uint)uStack_1c8;
 178	  lVar10 = 0;
 179	  if ((uVar9 & 0xfffffffe) == 4) {
 180	    lVar10 = uStack_1c8 + -3;
 181	  }
 182	  puStack_50 = (undefined8 *)puVar8;
 183	  if (lVar10 == 0) {
 184	    if (uStack_198 != 0) {
 185	      func_0x000140613c20(puStack_190,uStack_198,1);
 186	    }
 187	    if ((uVar9 != 3) && (1 < uVar9)) {
 188	      LOCK();
 189	      *uStack_1c0 = *uStack_1c0 + -1;
 190	      UNLOCK();
 191	      if (*uStack_1c0 == 0) {
 192	        func_0x00014064eb30(&uStack_1c0);
 193	      }
 194	    }
 195	    if ((uStack_1b0 != 6) && (4 < (uint)uStack_1b0)) {
 196	      plVar5 = (longlong *)CONCAT44(uStack_1a4,uStack_1a8);
 197	      LOCK();
 198	      *plVar5 = *plVar5 + -1;
 199	      UNLOCK();
 200	      if (*plVar5 == 0) {
 201	        func_0x00014064eb30(&uStack_1a8);
 202	      }
 203	    }
 204	  }
 205	  else if (lVar10 == 1) {
 206	    if (uStack_1c0 != (longlong *)0x0) {
 207	      func_0x000140613c20(uStack_1b8,uStack_1c0,1);
 208	    }
 209	    FUN_140608f80(&uStack_1a8);
 210	  }
 211	  else {
 212	    LOCK();
 213	    *uStack_1c0 = *uStack_1c0 + -1;
 214	    UNLOCK();
 215	    if (*uStack_1c0 == 0) {
 216	      FUN_140c7de90(&uStack_1c0);
 217	    }
 218	  }
 219	  uStack_1c8 = 3;
 220	  uStack_1c0 = (longlong *)uStack_410;
 221	  uStack_1b8 = lStack_408;
 222	  uStack_1b0 = 6;
 223	  uStack_1a8 = uStack_2c8;
 224	  uStack_1a4 = uStack_2c4;
 225	  uStack_1a0 = uStack_2c0;
 226	  uStack_19c = uStack_2bc;
 227	  uStack_198 = 3;
 228	  puStack_190 = (undefined2 *)puStack_50;
 229	  uStack_188 = 3;
 230	  uStack_180 = uStack_180 & 0xffffffff00000000;
 231	  uStack_178 = uStack_178 & 0xffffffff00000000;
 232	  uStack_170 = 0;
 233	  uStack_168 = 0;
 234	  uStack_167 = (undefined7)CONCAT44(uStack_254,uStack_258);
 235	  uStack_160 = CONCAT31(uStack_250,uStack_254._3_1_);
 236	  func_0x000142923c00(&uStack_410,&uStack_1f8,0x148);
 237	  FUN_140c77160(&uStack_1f8,&uStack_410,param_2);
 238	  LOCK();
 239	  *uStack_1e8 = *uStack_1e8 + -1;
 240	  UNLOCK();
 241	  if (*uStack_1e8 == 0) {
 242	    FUN_140c41170(&uStack_1e8);
 243	  }
 244	  uStack_410 = 4;
 245	  uStack_41 = 0;
 246	  FUN_1400568d0(&uStack_1f8,&UNK_1429511a5,5);
 247	  uStack_268 = uStack_198;
 248	  puStack_260 = puStack_190;
 249	  uStack_278 = CONCAT44(uStack_1a4,uStack_1a8);
 250	  uStack_270 = CONCAT44(uStack_19c,uStack_1a0);
 251	  lStack_288 = uStack_1b8;
 252	  uStack_280 = uStack_1b0;
 253	  uStack_298 = (undefined4)uStack_1c8;
 254	  uStack_294 = uStack_1c8._4_4_;
 255	  uStack_290 = (undefined4)uStack_1c0;
 256	  uStack_28c = uStack_1c0._4_4_;
 257	  uStack_2a8 = (undefined4)uStack_1d8;
 258	  uStack_2a4 = uStack_1d8._4_4_;
 259	  uStack_2a0 = (undefined4)uStack_1d0;
 260	  uStack_29c = uStack_1d0._4_4_;
 261	  uStack_2b8 = (undefined4)uStack_1e8;
 262	  uStack_2b4 = uStack_1e8._4_4_;
 263	  uStack_2b0 = uStack_1e0;
 264	  uStack_2ac = uStack_1dc;
 265	  uStack_2c8 = (undefined4)uStack_1f8;
 266	  uStack_2c4 = uStack_1f8._4_4_;
 267	  uStack_2c0 = uStack_1f0;
 268	  uStack_2bc = uStack_1ec;
 269	  uStack_41 = 1;
 270	  FUN_140c46bd0(&uStack_1f8,&uStack_410,&uStack_2c8);
 271	  FUN_140c46d20(&uStack_258,&uStack_1f8,param_2);
 272	  if (cStack_204 == '\0') {
 273	    uStack_1f8 = uStack_1f8 & 0xffffffffffffff00;
 274	    if (cStack_205 != '\0') {
 275	      cVar6 = FUN_140042870(aplStack_248,&uStack_1f8);
 276	      if (cVar6 != '\0') goto code_r0x0001401472e5;
 277	    }
 278	    LOCK();
 279	    *aplStack_248[0] = *aplStack_248[0] + -1;
 280	    UNLOCK();
 281	    if (*aplStack_248[0] == 0) {
 282	      FUN_140c41170(aplStack_248);
 283	    }
 284	    fVar12 = *pfVar1;
 285	  }
 286	  else {
 287	code_r0x0001401472e5:
 288	    LOCK();
 289	    *aplStack_248[0] = *aplStack_248[0] + -1;
 290	    UNLOCK();
 291	    if (*aplStack_248[0] == 0) {
 292	      FUN_140c41170(aplStack_248);
 293	    }
 294	    *pfVar1 = 45.0;
 295	    fVar12 = 45.0;
 296	  }
 297	  uVar3 = *puVar2;
 298	  fVar11 = (float)func_0x0001409114d0(uVar3);
 299	  if ((fVar12 != fVar11) || (NAN(fVar12) || NAN(fVar11))) {
 300	    *(undefined4 *)puVar2[2] = *(undefined4 *)((longlong)puVar2 + 0x1c);
 301	    func_0x0001409114f0(uVar3,*pfVar1);
 302	  }
 303	  return;
 304	}
 305	
