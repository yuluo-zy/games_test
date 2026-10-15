   1	
   2	ulonglong * FUN_1415bed20(ulonglong *param_1,longlong param_2)
   3	
   4	{
   5	  undefined4 *puVar1;
   6	  undefined8 uVar2;
   7	  longlong lVar3;
   8	  longlong lVar4;
   9	  longlong lVar5;
  10	  longlong lVar6;
  11	  ulonglong uVar7;
  12	  byte bVar8;
  13	  undefined4 uVar9;
  14	  undefined4 uVar10;
  15	  ulonglong uVar11;
  16	  char cVar12;
  17	  undefined4 uStack_198;
  18	  undefined4 uStack_194;
  19	  undefined4 uStack_190;
  20	  undefined4 uStack_18c;
  21	  ulonglong uStack_188;
  22	  ulonglong uStack_180;
  23	  undefined8 uStack_168;
  24	  undefined1 auStack_160 [64];
  25	  longlong lStack_120;
  26	  undefined4 uStack_118;
  27	  undefined4 uStack_114;
  28	  undefined4 uStack_110;
  29	  undefined4 uStack_10c;
  30	  undefined4 uStack_108;
  31	  undefined4 uStack_104;
  32	  undefined4 uStack_100;
  33	  undefined4 uStack_fc;
  34	  undefined4 uStack_f8;
  35	  undefined4 uStack_f4;
  36	  undefined4 uStack_f0;
  37	  undefined4 uStack_ec;
  38	  undefined4 uStack_e8;
  39	  undefined4 uStack_e4;
  40	  undefined4 uStack_e0;
  41	  undefined4 uStack_dc;
  42	  ulonglong uStack_d8;
  43	  ulonglong uStack_d0;
  44	  undefined8 uStack_c0;
  45	  byte bStack_b8;
  46	  byte bStack_b7;
  47	  undefined1 uStack_b6;
  48	  undefined1 uStack_b5;
  49	  undefined4 uStack_b4;
  50	  undefined1 uStack_b0;
  51	  undefined4 uStack_af;
  52	  undefined4 uStack_ab;
  53	  undefined3 uStack_a7;
  54	  undefined4 uStack_a4;
  55	  undefined4 uStack_a0;
  56	  undefined4 uStack_9c;
  57	  undefined4 uStack_94;
  58	  longlong lStack_90;
  59	  longlong *plStack_88;
  60	  uint uStack_7c;
  61	  ulonglong uStack_78;
  62	  undefined1 uStack_69;
  63	  ulonglong uStack_68;
  64	  byte bStack_59;
  65	  ulonglong uStack_58;
  66	  undefined1 uStack_49;
  67	  undefined8 uStack_48;
  68	  char cVar13;
  69	  
  70	  uStack_48 = 0xfffffffffffffffe;
  71	  uVar2 = *(undefined8 *)(param_2 + 0x10);
  72	  func_0x000140b880d0(auStack_160);
  73	  uStack_7c = 0;
  74	  bStack_59 = 2;
  75	  uStack_58 = 0x8000000000000000;
  76	code_r0x0001415beda0:
  77	  do {
  78	    uStack_49 = 1;
  79	    uStack_68 = uStack_58;
  80	    FUN_14143c450(&bStack_b8,auStack_160);
  81	    lVar5 = CONCAT44(uStack_b4,CONCAT13(uStack_b5,CONCAT12(uStack_b6,CONCAT11(bStack_b7,bStack_b8)))
  82	                    );
  83	    if (lVar5 == 0) {
  84	code_r0x0001415bf1ee:
  85	      if (SBORROW8(0,uStack_58)) {
  86	        uStack_68 = 0x8000000000000000;
  87	        uStack_49 = 1;
  88	        uVar11 = FUN_1428f9bc0(&UNK_142c7abc0,7);
  89	        goto code_r0x0001415bf2cf;
  90	      }
  91	      if ((uStack_7c & 1) == 0) {
  92	        uVar11 = FUN_1428f9bc0(&UNK_142c7abc7,0xb);
  93	        uVar7 = uStack_58;
  94	        goto joined_r0x0001415bf2c2;
  95	      }
  96	      if (bStack_59 == 2) {
  97	        uVar11 = FUN_1428f9bc0(&UNK_142c7abd2,8);
  98	        uVar7 = uStack_58;
  99	        goto joined_r0x0001415bf2c2;
 100	      }
 101	      if (lStack_120 == 0) {
 102	        *param_1 = uStack_58;
 103	        param_1[1] = uStack_78;
 104	        *(undefined4 *)(param_1 + 2) = uStack_198;
 105	        *(undefined4 *)((longlong)param_1 + 0x14) = uStack_194;
 106	        *(undefined4 *)(param_1 + 3) = uStack_190;
 107	        *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_18c;
 108	        param_1[4] = uStack_188;
 109	        param_1[5] = uStack_180;
 110	        *(undefined4 *)(param_1 + 6) = uStack_94;
 111	        *(undefined8 *)((longlong)param_1 + 0x34) = uStack_168;
 112	        *(byte *)((longlong)param_1 + 0x3c) = bStack_59 & 1;
 113	        *(undefined1 *)((longlong)param_1 + 0x3d) = uStack_69;
 114	      }
 115	      else {
 116	        uVar11 = FUN_1428f9d20(uVar2,&UNK_142c7a098,&UNK_142c7a060);
 117	        param_1[1] = uVar11;
 118	        *param_1 = 0x8000000000000000;
 119	        if (uStack_58 != 0) {
 120	          func_0x000140613c20(uStack_78,uStack_58,1);
 121	        }
 122	      }
 123	      FUN_140887df0(auStack_160);
 124	      goto code_r0x0001415bf2e9;
 125	    }
 126	    lVar6 = CONCAT44(uStack_a4,CONCAT31(uStack_a7,uStack_ab._3_1_));
 127	    lVar3 = *(longlong *)(lVar5 + 0x168 + lVar6 * 0x18);
 128	    plStack_88 = *(longlong **)(lVar5 + 0x170 + lVar6 * 0x18);
 129	    lVar4 = *(longlong *)(lVar5 + 0x178 + lVar6 * 0x18);
 130	    puVar1 = (undefined4 *)(lVar5 + lVar6 * 0x20);
 131	    uStack_f8 = *puVar1;
 132	    uStack_f4 = puVar1[1];
 133	    uStack_f0 = puVar1[2];
 134	    uStack_ec = puVar1[3];
 135	    puVar1 = (undefined4 *)(lVar5 + 0x10 + lVar6 * 0x20);
 136	    uStack_e8 = *puVar1;
 137	    uStack_e4 = puVar1[1];
 138	    uStack_e0 = puVar1[2];
 139	    uStack_dc = puVar1[3];
 140	    if (SBORROW8(0,lVar3)) goto code_r0x0001415bf1ee;
 141	    lStack_90 = lVar3;
 142	    if (false) {
 143	code_r0x0001415befb4:
 144	      FUN_140898f40(&uStack_110);
 145	    }
 146	    else {
 147	      switch((byte)uStack_118) {
 148	      case 0:
 149	      case 1:
 150	      case 2:
 151	      case 6:
 152	        break;
 153	      case 3:
 154	        if (CONCAT44(uStack_10c,uStack_110) != 0) {
 155	          func_0x000140613c20(CONCAT44(uStack_104,uStack_108),CONCAT44(uStack_10c,uStack_110),1);
 156	        }
 157	        break;
 158	      case 4:
 159	        uStack_c0 = CONCAT44(uStack_104,uStack_108);
 160	        FUN_140888d70(uStack_c0,CONCAT44(uStack_fc,uStack_100));
 161	        if (CONCAT44(uStack_10c,uStack_110) != 0) {
 162	          func_0x000140613c20(uStack_c0,CONCAT44(uStack_10c,uStack_110) << 5,8);
 163	        }
 164	        break;
 165	      default:
 166	        goto code_r0x0001415befb4;
 167	      }
 168	    }
 169	    uStack_108 = uStack_e8;
 170	    uStack_104 = uStack_e4;
 171	    uStack_100 = uStack_e0;
 172	    uStack_fc = uStack_dc;
 173	    uStack_118 = uStack_f8;
 174	    uStack_114 = uStack_f4;
 175	    uStack_110 = uStack_f0;
 176	    uStack_10c = uStack_ec;
 177	    if (lVar4 == 0xb) {
 178	      if (*(longlong *)((longlong)plStack_88 + 3) != 0x736e656c5f617265 ||
 179	          *plStack_88 != 0x6c5f6172656d6163) goto code_r0x0001415beedc;
 180	      cVar13 = '\x01';
 181	      cVar12 = '\x01';
 182	    }
 183	    else if (lVar4 == 8) {
 184	      if (*plStack_88 != 0x6369747369747261) goto code_r0x0001415beedc;
 185	      cVar13 = '\x02';
 186	      cVar12 = '\x02';
 187	    }
 188	    else if ((lVar4 == 7) &&
 189	            (*(int *)((longlong)plStack_88 + 3) == 0x676e6964 && (int)*plStack_88 == 0x64617267)) {
 190	      cVar13 = '\0';
 191	      cVar12 = '\0';
 192	    }
 193	    else {
 194	code_r0x0001415beedc:
 195	      cVar13 = '\x03';
 196	      cVar12 = '\x03';
 197	    }
 198	    if (lStack_90 != 0) {
 199	      func_0x000140613c20(plStack_88,uStack_e8,1);
 200	      cVar12 = cVar13;
 201	    }
 202	    uVar10 = uStack_e8;
 203	    uVar9 = uStack_118;
 204	    bVar8 = (byte)uStack_118;
 205	    if (cVar12 != '\x03') {
 206	      uStack_e8._1_3_ = (undefined3)((uint)uStack_108 >> 8);
 207	      uStack_e8 = uVar10;
 208	      if (cVar12 == '\0') {
 209	        if (!SBORROW8(0,uStack_58)) {
 210	          uStack_68 = uStack_58;
 211	          uStack_49 = 1;
 212	          uVar11 = FUN_1428f9db0(&UNK_142c7abc0,7);
 213	          goto code_r0x0001415bf287;
 214	        }
 215	        uStack_118 = CONCAT31(uStack_118._1_3_,6);
 216	        if (bVar8 == 6) {
 217	          uStack_68 = 0x8000000000000000;
 218	          uStack_49 = 1;
 219	          uVar11 = FUN_1428fa000(&UNK_142c7a268,0x10);
 220	          goto code_r0x0001415bf2cf;
 221	        }
 222	        bStack_b8 = bVar8;
 223	        uStack_af = CONCAT13((undefined1)uStack_10c,uStack_110._1_3_);
 224	        uStack_ab = CONCAT13((undefined1)uStack_108,uStack_10c._1_3_);
 225	        uStack_a4 = uStack_104;
 226	        uStack_a0 = uStack_100;
 227	        uStack_9c = uStack_fc;
 228	        bStack_b7 = (byte)((uint)uVar9 >> 8);
 229	        uStack_b6 = (undefined1)((uint)uVar9 >> 0x10);
 230	        uStack_b5 = (undefined1)((uint)uVar9 >> 0x18);
 231	        uStack_b4 = uStack_114;
 232	        uStack_b0 = (undefined1)uStack_110;
 233	        uStack_68 = 0x8000000000000000;
 234	        uStack_49 = 1;
 235	        uStack_a7 = uStack_e8._1_3_;
 236	        FUN_1415a2b40(&uStack_f8,&bStack_b8,&UNK_142ae3d9e,0x11,&UNK_142ae3df8,6);
 237	        uVar11 = CONCAT44(uStack_ec,uStack_f0);
 238	        uStack_78 = uVar11;
 239	        if (SBORROW8(0,CONCAT44(uStack_f4,uStack_f8))) goto code_r0x0001415bf2cf;
 240	        uStack_198 = uStack_e8;
 241	        uStack_194 = uStack_e4;
 242	        uStack_190 = uStack_e0;
 243	        uStack_18c = uStack_dc;
 244	        uStack_188 = uStack_d8;
 245	        uStack_180 = uStack_d0;
 246	        uStack_58 = CONCAT44(uStack_f4,uStack_f8);
 247	        goto code_r0x0001415beda0;
 248	      }
 249	      if (cVar12 == '\x01') {
 250	        if (uStack_7c != 0) {
 251	          uStack_68 = uStack_58;
 252	          uStack_49 = 1;
 253	          uVar11 = FUN_1428f9db0(&UNK_142c7abc7,0xb);
 254	          goto code_r0x0001415bf287;
 255	        }
 256	        uStack_118 = CONCAT31(uStack_118._1_3_,6);
 257	        if (bVar8 == 6) {
 258	          uStack_68 = uStack_58;
 259	          uStack_49 = 1;
 260	          uVar11 = FUN_1428fa000(&UNK_142c7a268,0x10);
 261	          goto code_r0x0001415bf287;
 262	        }
 263	        uStack_e4 = uStack_104;
 264	        uStack_e0 = uStack_100;
 265	        uStack_dc = uStack_fc;
 266	        uStack_f8 = uVar9;
 267	        uStack_f4 = uStack_114;
 268	        uStack_f0 = uStack_110;
 269	        uStack_ec = uStack_10c;
 270	        uStack_e8 = uStack_108;
 271	        uStack_68 = uStack_58;
 272	        uStack_49 = 1;
 273	        FUN_14159e530(&bStack_b8,&uStack_f8,&UNK_142a95918,0x10,&UNK_142a95958,3);
 274	        if ((bStack_b8 & 1) == 0) {
 275	          uStack_94 = uStack_b4;
 276	          uStack_168 = CONCAT35((undefined3)uStack_ab,CONCAT41(uStack_af,uStack_b0));
 277	          uStack_7c = 1;
 278	          goto code_r0x0001415beda0;
 279	        }
 280	      }
 281	      else {
 282	        if (bStack_59 != 2) {
 283	          uStack_68 = uStack_58;
 284	          uStack_49 = 1;
 285	          uVar11 = FUN_1428f9db0(&UNK_142c7abd2,8);
 286	          goto code_r0x0001415bf287;
 287	        }
 288	        uStack_118 = CONCAT31(uStack_118._1_3_,6);
 289	        if (bVar8 == 6) {
 290	          uStack_68 = uStack_58;
 291	          uStack_49 = 1;
 292	          uVar11 = FUN_1428fa000(&UNK_142c7a268,0x10);
 293	          goto code_r0x0001415bf287;
 294	        }
 295	        uStack_e4 = uStack_104;
 296	        uStack_e0 = uStack_100;
 297	        uStack_dc = uStack_fc;
 298	        uStack_f8 = uVar9;
 299	        uStack_f4 = uStack_114;
 300	        uStack_f0 = uStack_110;
 301	        uStack_ec = uStack_10c;
 302	        uStack_e8 = uStack_108;
 303	        uStack_68 = uStack_58;
 304	        uStack_49 = 1;
 305	        FUN_1415a0330(&bStack_b8,&uStack_f8,&UNK_142a95988,0xe,&UNK_142a959b0,2);
 306	        if ((bStack_b8 & 1) == 0) {
 307	          bStack_59 = bStack_b7;
 308	          uStack_69 = uStack_b6;
 309	          goto code_r0x0001415beda0;
 310	        }
 311	      }
 312	      uVar11 = CONCAT35((undefined3)uStack_ab,CONCAT41(uStack_af,uStack_b0));
 313	      goto code_r0x0001415bf287;
 314	    }
 315	    uStack_118 = CONCAT31(uStack_118._1_3_,6);
 316	    if (bVar8 == 6) {
 317	      uStack_68 = uStack_58;
 318	      uStack_49 = 1;
 319	      uVar11 = FUN_1428fa000(&UNK_142c7a268,0x10);
 320	      goto code_r0x0001415bf287;
 321	    }
 322	    uStack_e4 = uStack_104;
 323	    uStack_e0 = uStack_100;
 324	    uStack_dc = uStack_fc;
 325	    uStack_f8 = uVar9;
 326	    uStack_f4 = uStack_114;
 327	    uStack_f0 = uStack_110;
 328	    uStack_ec = uStack_10c;
 329	    uStack_e8 = uStack_108;
 330	    uStack_68 = uStack_58;
 331	    uStack_49 = 1;
 332	    uVar11 = FUN_1415a3940(&uStack_f8);
 333	    if (uVar11 != 0) {
 334	code_r0x0001415bf287:
 335	      uVar7 = uStack_58 & 0x7fffffffffffffff;
 336	joined_r0x0001415bf2c2:
 337	      if (uVar7 != 0) {
 338	        func_0x000140613c20(uStack_78,uStack_58,1);
 339	      }
 340	code_r0x0001415bf2cf:
 341	      param_1[1] = uVar11;
 342	      *param_1 = 0x8000000000000000;
 343	      FUN_140887df0(auStack_160);
 344	code_r0x0001415bf2e9:
 345	      FUN_14088e430(&uStack_118);
 346	      return param_1;
 347	    }
 348	  } while( true );
 349	}
 350	