   1	
   2	undefined8 * FUN_1415cc860(undefined8 *param_1,longlong param_2)
   3	
   4	{
   5	  undefined4 *puVar1;
   6	  longlong lVar2;
   7	  longlong lVar3;
   8	  byte bVar4;
   9	  undefined4 uVar5;
  10	  undefined4 uVar6;
  11	  longlong lVar7;
  12	  undefined8 uVar8;
  13	  byte bVar9;
  14	  ulonglong uVar10;
  15	  undefined8 *puVar11;
  16	  longlong lVar12;
  17	  undefined8 uStack_238;
  18	  undefined8 uStack_230;
  19	  undefined8 uStack_228;
  20	  undefined8 uStack_220;
  21	  undefined8 uStack_218;
  22	  undefined8 uStack_210;
  23	  undefined4 uStack_208;
  24	  undefined4 uStack_204;
  25	  undefined4 uStack_200;
  26	  undefined4 uStack_1fc;
  27	  undefined8 uStack_1f8;
  28	  undefined8 uStack_1f0;
  29	  undefined8 uStack_1e8;
  30	  undefined8 uStack_1e0;
  31	  undefined4 uStack_1d8;
  32	  undefined4 uStack_1d4;
  33	  undefined4 uStack_1d0;
  34	  undefined4 uStack_1cc;
  35	  undefined4 uStack_1c8;
  36	  undefined4 uStack_1c4;
  37	  undefined4 uStack_1c0;
  38	  undefined4 uStack_1bc;
  39	  undefined8 uStack_1b8;
  40	  undefined8 uStack_1b0;
  41	  undefined8 uStack_1a8;
  42	  undefined8 uStack_1a0;
  43	  undefined8 uStack_198;
  44	  undefined8 uStack_190;
  45	  ulonglong uStack_188;
  46	  longlong lStack_180;
  47	  longlong lStack_178;
  48	  undefined4 uStack_170;
  49	  undefined4 uStack_16c;
  50	  undefined4 uStack_168;
  51	  undefined4 uStack_164;
  52	  undefined4 uStack_160;
  53	  undefined4 uStack_15c;
  54	  undefined4 uStack_158;
  55	  undefined4 uStack_154;
  56	  undefined4 uStack_150;
  57	  undefined4 uStack_14c;
  58	  undefined4 uStack_148;
  59	  undefined4 uStack_144;
  60	  char cStack_140;
  61	  undefined4 uStack_13f;
  62	  undefined2 uStack_13b;
  63	  undefined1 uStack_139;
  64	  undefined8 uStack_138;
  65	  ulonglong uStack_130;
  66	  undefined1 auStack_128 [64];
  67	  longlong lStack_e8;
  68	  undefined4 uStack_e0;
  69	  undefined4 uStack_dc;
  70	  undefined4 uStack_d8;
  71	  undefined4 uStack_d4;
  72	  undefined4 uStack_d0;
  73	  undefined4 uStack_cc;
  74	  undefined4 uStack_c8;
  75	  undefined4 uStack_c4;
  76	  byte bStack_c0;
  77	  undefined4 uStack_bf;
  78	  undefined4 uStack_bb;
  79	  undefined4 uStack_b7;
  80	  undefined4 uStack_b3;
  81	  undefined3 uStack_af;
  82	  undefined4 uStack_ac;
  83	  undefined4 uStack_a8;
  84	  undefined4 uStack_a4;
  85	  undefined8 uStack_a0;
  86	  longlong *plStack_98;
  87	  longlong lStack_90;
  88	  undefined8 *puStack_88;
  89	  longlong lStack_80;
  90	  ulonglong uStack_78;
  91	  longlong lStack_70;
  92	  ulonglong uStack_68;
  93	  longlong lStack_60;
  94	  longlong lStack_58;
  95	  char cStack_49;
  96	  undefined8 uStack_48;
  97	  
  98	  uStack_48 = 0xfffffffffffffffe;
  99	  uStack_138 = *(undefined8 *)(param_2 + 0x10);
 100	  puStack_88 = param_1;
 101	  func_0x000140b880d0(auStack_128);
 102	  cStack_49 = '\x05';
 103	  uVar10 = 0x8000000000000000;
 104	  lStack_58 = -0x7fffffffffffffff;
 105	  uVar6 = uStack_1c8;
 106	  uStack_78 = uVar10;
 107	code_r0x0001415cc8f0:
 108	  do {
 109	    uStack_1c8 = uVar6;
 110	    lStack_60 = lStack_58;
 111	    uStack_68 = uVar10;
 112	    FUN_14143c450(&bStack_c0,auStack_128);
 113	    puVar11 = puStack_88;
 114	    lVar7 = CONCAT35((undefined3)uStack_bb,CONCAT41(uStack_bf,bStack_c0));
 115	    if (lVar7 == 0) {
 116	code_r0x0001415ccd61:
 117	      if (cStack_49 != '\x05') {
 118	        uStack_190 = lStack_80;
 119	        uStack_188 = uStack_130;
 120	        if (SBORROW8(0,uVar10)) {
 121	          uStack_190 = 1;
 122	          uVar10 = 0;
 123	          uStack_188 = 0;
 124	        }
 125	        if (lStack_58 == -0x7fffffffffffffff) {
 126	          lStack_180 = -0x8000000000000000;
 127	        }
 128	        else {
 129	          uStack_238 = CONCAT44(uStack_204,uStack_208);
 130	          uStack_230 = CONCAT44(uStack_1fc,uStack_200);
 131	          uStack_218 = uStack_1e8;
 132	          uStack_210 = uStack_1e0;
 133	          uStack_228 = uStack_1f8;
 134	          uStack_220 = uStack_1f0;
 135	          lStack_180 = lStack_58;
 136	        }
 137	        lStack_178 = lStack_70;
 138	        uStack_170 = (undefined4)uStack_238;
 139	        uStack_16c = uStack_238._4_4_;
 140	        uStack_168 = (undefined4)uStack_230;
 141	        uStack_164 = uStack_230._4_4_;
 142	        uStack_160 = (undefined4)uStack_228;
 143	        uStack_15c = uStack_228._4_4_;
 144	        uStack_158 = (undefined4)uStack_220;
 145	        uStack_154 = uStack_220._4_4_;
 146	        uStack_150 = (undefined4)uStack_218;
 147	        uStack_14c = uStack_218._4_4_;
 148	        uStack_148 = (undefined4)uStack_210;
 149	        uStack_144 = uStack_210._4_4_;
 150	        cStack_140 = cStack_49;
 151	        uStack_198 = uVar10;
 152	        if (lStack_e8 == 0) {
 153	          puStack_88[10] = uStack_210;
 154	          *(char *)(puStack_88 + 0xb) = cStack_49;
 155	          *(undefined4 *)((longlong)puStack_88 + 0x59) = uStack_13f;
 156	          *(undefined2 *)((longlong)puStack_88 + 0x5d) = uStack_13b;
 157	          *(undefined1 *)((longlong)puStack_88 + 0x5f) = uStack_139;
 158	          puStack_88[8] = uStack_220;
 159	          puStack_88[9] = uStack_218;
 160	          uStack_198._0_4_ = (undefined4)uVar10;
 161	          uStack_198._4_4_ = (undefined4)(uVar10 >> 0x20);
 162	          uStack_190._4_4_ = (undefined4)((ulonglong)uStack_190 >> 0x20);
 163	          puStack_88[6] = uStack_230;
 164	          puStack_88[7] = uStack_228;
 165	          puStack_88[4] = lStack_70;
 166	          puStack_88[5] = uStack_238;
 167	          puStack_88[2] = uStack_188;
 168	          puStack_88[3] = lStack_180;
 169	          *(undefined4 *)puStack_88 = (undefined4)uStack_198;
 170	          *(undefined4 *)((longlong)puStack_88 + 4) = uStack_198._4_4_;
 171	          *(undefined4 *)(puStack_88 + 1) = (undefined4)uStack_190;
 172	          *(undefined4 *)((longlong)puStack_88 + 0xc) = uStack_190._4_4_;
 173	        }
 174	        else {
 175	          uVar8 = FUN_1428f9d20(uStack_138,&UNK_142c7a098,&UNK_142c7a060);
 176	          puVar11[1] = uVar8;
 177	          *puVar11 = 0x8000000000000000;
 178	          FUN_14014b4d0(&uStack_198);
 179	        }
 180	        FUN_140887df0(auStack_128);
 181	        goto code_r0x0001415ccf7e;
 182	      }
 183	      lStack_60 = lStack_58;
 184	      uStack_68 = uVar10;
 185	      lVar7 = FUN_1428f9bc0(&UNK_142c7ace8,5);
 186	      goto code_r0x0001415cce2b;
 187	    }
 188	    lVar3 = CONCAT44(uStack_ac,CONCAT31(uStack_af,uStack_b3._3_1_));
 189	    lVar12 = *(longlong *)(lVar7 + 0x168 + lVar3 * 0x18);
 190	    plStack_98 = *(longlong **)(lVar7 + 0x170 + lVar3 * 0x18);
 191	    lVar2 = *(longlong *)(lVar7 + 0x178 + lVar3 * 0x18);
 192	    puVar1 = (undefined4 *)(lVar7 + lVar3 * 0x20);
 193	    uStack_1d8 = *puVar1;
 194	    uStack_1d4 = puVar1[1];
 195	    uStack_1d0 = puVar1[2];
 196	    uStack_1cc = puVar1[3];
 197	    puVar1 = (undefined4 *)(lVar7 + 0x10 + lVar3 * 0x20);
 198	    uStack_1c8 = *puVar1;
 199	    uStack_1c4 = puVar1[1];
 200	    uStack_1c0 = puVar1[2];
 201	    uStack_1bc = puVar1[3];
 202	    if (SBORROW8(0,lVar12)) goto code_r0x0001415ccd61;
 203	    lStack_90 = lVar12;
 204	    if (false) {
 205	code_r0x0001415ccb17:
 206	      FUN_140898f40(&uStack_d8);
 207	    }
 208	    else {
 209	      switch((byte)uStack_e0) {
 210	      case 0:
 211	      case 1:
 212	      case 2:
 213	      case 6:
 214	        break;
 215	      case 3:
 216	        if (CONCAT44(uStack_d4,uStack_d8) != 0) {
 217	          func_0x000140613c20(CONCAT44(uStack_cc,uStack_d0),CONCAT44(uStack_d4,uStack_d8),1);
 218	        }
 219	        break;
 220	      case 4:
 221	        uStack_a0 = CONCAT44(uStack_cc,uStack_d0);
 222	        FUN_140888d70(uStack_a0,CONCAT44(uStack_c4,uStack_c8));
 223	        if (CONCAT44(uStack_d4,uStack_d8) != 0) {
 224	          func_0x000140613c20(uStack_a0,CONCAT44(uStack_d4,uStack_d8) << 5,8);
 225	        }
 226	        break;
 227	      default:
 228	        goto code_r0x0001415ccb17;
 229	      }
 230	    }
 231	    uStack_d0 = uStack_1c8;
 232	    uStack_cc = uStack_1c4;
 233	    uStack_c8 = uStack_1c0;
 234	    uStack_c4 = uStack_1bc;
 235	    uStack_e0 = uStack_1d8;
 236	    uStack_dc = uStack_1d4;
 237	    uStack_d8 = uStack_1d0;
 238	    uStack_d4 = uStack_1cc;
 239	    if (lVar2 == 8) {
 240	      bVar9 = (*plStack_98 != 0x73726f746174756d) * '\x02' + 1;
 241	    }
 242	    else {
 243	      bVar9 = 3;
 244	      if (lVar2 == 5) {
 245	        if (*(char *)((longlong)plStack_98 + 4) == 'e' && (int)*plStack_98 == 0x64616c67) {
 246	          bVar9 = 0;
 247	        }
 248	        else {
 249	          bVar9 = (*(char *)((longlong)plStack_98 + 4) == 'e' && (int)*plStack_98 == 0x67616d69) ^ 3
 250	          ;
 251	        }
 252	      }
 253	    }
 254	    if (lStack_90 != 0) {
 255	      func_0x000140613c20(uStack_1d8,uStack_1c8,1);
 256	    }
 257	    uVar10 = uStack_78;
 258	    uVar5 = uStack_e0;
 259	    uVar6 = uStack_1c8;
 260	    bVar4 = (byte)uStack_e0;
 261	    if (bVar9 != 3) {
 262	      if (bVar9 == 0) {
 263	        if (cStack_49 != '\x05') {
 264	          lStack_60 = lStack_58;
 265	          uStack_68 = uStack_78;
 266	          lVar7 = FUN_1428f9db0(&UNK_142c7ace8,5);
 267	          goto code_r0x0001415cce2b;
 268	        }
 269	        uStack_e0 = CONCAT31(uStack_e0._1_3_,6);
 270	        if (bVar4 == 6) {
 271	          lStack_60 = lStack_58;
 272	          uStack_68 = uStack_78;
 273	          lVar7 = FUN_1428fa000(&UNK_142c7a268,0x10);
 274	          goto code_r0x0001415cce2b;
 275	        }
 276	        uStack_1c4 = uStack_cc;
 277	        uStack_1c0 = uStack_c8;
 278	        uStack_1bc = uStack_c4;
 279	        uStack_1d8 = uVar5;
 280	        uStack_1d4 = uStack_dc;
 281	        uStack_1d0 = uStack_d8;
 282	        uStack_1cc = uStack_d4;
 283	        uStack_1c8 = uStack_d0;
 284	        lStack_60 = lStack_58;
 285	        uStack_68 = uStack_78;
 286	        FUN_141328d00(&bStack_c0,&uStack_1d8);
 287	        if ((bStack_c0 & 1) != 0) {
 288	          lVar7 = CONCAT35((undefined3)uStack_b3,CONCAT41(uStack_b7,uStack_bb._3_1_));
 289	          goto code_r0x0001415cce2b;
 290	        }
 291	        cStack_49 = (char)uStack_bf;
 292	        uVar6 = uStack_1c8;
 293	        goto code_r0x0001415cc8f0;
 294	      }
 295	      uStack_1c8._1_3_ = (undefined3)((uint)uStack_d0 >> 8);
 296	      uStack_1c8 = uVar6;
 297	      if (bVar9 != 1) {
 298	        if (lStack_58 != -0x7fffffffffffffff) {
 299	          lStack_60 = lStack_58;
 300	          uStack_68 = uStack_78;
 301	          lVar7 = FUN_1428f9db0(&UNK_142c7acf5,5);
 302	          goto code_r0x0001415cce2b;
 303	        }
 304	        uStack_e0 = CONCAT31(uStack_e0._1_3_,6);
 305	        lStack_58 = -0x8000000000000000;
 306	        uStack_78 = uVar10;
 307	        if (bVar4 != 0) {
 308	          if (bVar4 == 6) {
 309	            lStack_60 = -0x7fffffffffffffff;
 310	            uStack_68 = uStack_78;
 311	            lVar7 = FUN_1428fa000(&UNK_142c7a268,0x10);
 312	            puVar11 = puStack_88;
 313	            lVar12 = lStack_80;
 314	            goto code_r0x0001415cce6f;
 315	          }
 316	          bStack_c0 = bVar4;
 317	          uStack_bf = CONCAT13((undefined1)uStack_dc,uStack_e0._1_3_);
 318	          uStack_bb = CONCAT13((undefined1)uStack_d8,uStack_dc._1_3_);
 319	          uStack_b7 = CONCAT13((undefined1)uStack_d4,uStack_d8._1_3_);
 320	          uStack_b3 = CONCAT13((undefined1)uStack_d0,uStack_d4._1_3_);
 321	          uStack_ac = uStack_cc;
 322	          uStack_a8 = uStack_c8;
 323	          uStack_a4 = uStack_c4;
 324	          lStack_60 = -0x7fffffffffffffff;
 325	          uStack_68 = uStack_78;
 326	          uStack_af = uStack_1c8._1_3_;
 327	          FUN_1415a0d30(&uStack_1d8,&bStack_c0,&UNK_142a959d0,0x14,&UNK_142a95a00,3);
 328	          lStack_58 = CONCAT44(uStack_1d4,uStack_1d8);
 329	          lVar7 = CONCAT44(uStack_1cc,uStack_1d0);
 330	          puVar11 = puStack_88;
 331	          lVar12 = lStack_80;
 332	          lStack_70 = lVar7;
 333	          if (SBORROW8(0,lStack_58)) goto code_r0x0001415cce6f;
 334	          uStack_208 = uStack_1c8;
 335	          uStack_204 = uStack_1c4;
 336	          uStack_200 = uStack_1c0;
 337	          uStack_1fc = uStack_1bc;
 338	          uStack_1f8 = uStack_1b8;
 339	          uStack_1f0 = uStack_1b0;
 340	          uStack_1e8 = uStack_1a8;
 341	          uStack_1e0 = uStack_1a0;
 342	          uVar6 = uStack_1c8;
 343	          uStack_78 = uVar10;
 344	        }
 345	        goto code_r0x0001415cc8f0;
 346	      }
 347	      if (!SBORROW8(0,uStack_78)) {
 348	        lStack_60 = lStack_58;
 349	        uStack_68 = uStack_78;
 350	        lVar7 = FUN_1428f9db0(&UNK_142c7aced,8);
 351	        goto code_r0x0001415cce2b;
 352	      }
 353	      uStack_e0 = CONCAT31(uStack_e0._1_3_,6);
 354	      if (bVar4 == 6) {
 355	        lStack_60 = lStack_58;
 356	        uStack_68 = 0x8000000000000000;
 357	        lVar7 = FUN_1428fa000(&UNK_142c7a268,0x10);
 358	      }
 359	      else {
 360	        uStack_1c4 = uStack_cc;
 361	        uStack_1c0 = uStack_c8;
 362	        uStack_1bc = uStack_c4;
 363	        uStack_1d8 = uVar5;
 364	        uStack_1d4 = uStack_dc;
 365	        uStack_1d0 = uStack_d8;
 366	        uStack_1cc = uStack_d4;
 367	        uStack_1c8 = uStack_d0;
 368	        lStack_60 = lStack_58;
 369	        uStack_68 = 0x8000000000000000;
 370	        FUN_14159b6c0(&bStack_c0,&uStack_1d8);
 371	        uVar10 = CONCAT35((undefined3)uStack_bb,CONCAT41(uStack_bf,bStack_c0));
 372	        lVar7 = CONCAT35((undefined3)uStack_b3,CONCAT41(uStack_b7,uStack_bb._3_1_));
 373	        if (!SBORROW8(0,uVar10)) {
 374	          uStack_130 = CONCAT44(uStack_ac,CONCAT31(uStack_af,uStack_b3._3_1_));
 375	          uVar6 = uStack_1c8;
 376	          lStack_80 = lVar7;
 377	          uStack_78 = uVar10;
 378	          goto code_r0x0001415cc8f0;
 379	        }
 380	      }
 381	      uStack_78 = 0x8000000000000000;
 382	      goto code_r0x0001415cce2b;
 383	    }
 384	    uStack_e0 = CONCAT31(uStack_e0._1_3_,6);
 385	    if (bVar4 == 6) {
 386	      lStack_60 = lStack_58;
 387	      uStack_68 = uStack_78;
 388	      lVar7 = FUN_1428fa000(&UNK_142c7a268,0x10);
 389	      goto code_r0x0001415cce2b;
 390	    }
 391	    uStack_1c4 = uStack_cc;
 392	    uStack_1c0 = uStack_c8;
 393	    uStack_1bc = uStack_c4;
 394	    uStack_1d8 = uVar5;
 395	    uStack_1d4 = uStack_dc;
 396	    uStack_1d0 = uStack_d8;
 397	    uStack_1cc = uStack_d4;
 398	    uStack_1c8 = uStack_d0;
 399	    lStack_60 = lStack_58;
 400	    uStack_68 = uStack_78;
 401	    lVar7 = FUN_1415a3940(&uStack_1d8);
 402	    uVar6 = uStack_1c8;
 403	    if (lVar7 != 0) {
 404	code_r0x0001415cce2b:
 405	      lVar12 = lStack_80;
 406	      puVar11 = puStack_88;
 407	      if ((-0x7fffffffffffffff < lStack_58) && (lStack_58 != 0)) {
 408	        func_0x000140613c20(lStack_70,lStack_58,1);
 409	      }
 410	code_r0x0001415cce6f:
 411	      if ((uStack_78 & 0x7fffffffffffffff) != 0) {
 412	        func_0x000140613c20(lVar12,uStack_78,1);
 413	      }
 414	      puVar11[1] = lVar7;
 415	      *puVar11 = 0x8000000000000000;
 416	      FUN_140887df0(auStack_128);
 417	code_r0x0001415ccf7e:
 418	      FUN_14088e430(&uStack_e0);
 419	      return puVar11;
 420	    }
 421	  } while( true );
 422	}
 423	