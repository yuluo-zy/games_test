   1	
   2	longlong FUN_1417522e0(longlong *param_1,undefined8 param_2)
   3	
   4	{
   5	  code *pcVar1;
   6	  undefined8 uVar2;
   7	  char cVar3;
   8	  undefined4 *puVar4;
   9	  undefined8 uVar5;
  10	  longlong lVar6;
  11	  longlong lVar7;
  12	  longlong lVar8;
  13	  longlong *plVar9;
  14	  longlong *plVar10;
  15	  longlong lStack_140;
  16	  undefined8 uStack_138;
  17	  undefined8 uStack_130;
  18	  longlong *plStack_128;
  19	  byte abStack_120 [8];
  20	  longlong lStack_118;
  21	  longlong lStack_110;
  22	  undefined8 uStack_108;
  23	  longlong *plStack_100;
  24	  longlong lStack_f8;
  25	  undefined4 *puStack_f0;
  26	  longlong lStack_e8;
  27	  undefined4 uStack_e0;
  28	  undefined4 uStack_dc;
  29	  longlong *plStack_d0;
  30	  undefined8 uStack_c8;
  31	  undefined8 uStack_c0;
  32	  undefined4 uStack_b8;
  33	  undefined4 uStack_b4;
  34	  undefined1 uStack_b0;
  35	  undefined1 uStack_af;
  36	  undefined1 uStack_ae;
  37	  undefined1 uStack_ad;
  38	  undefined1 uStack_ac;
  39	  undefined1 uStack_ab;
  40	  undefined2 uStack_aa;
  41	  undefined1 uStack_a8;
  42	  longlong *plStack_a0;
  43	  undefined *puStack_98;
  44	  longlong lStack_90;
  45	  longlong *plStack_88;
  46	  undefined4 uStack_80;
  47	  undefined4 uStack_7c;
  48	  undefined4 uStack_78;
  49	  undefined4 uStack_74;
  50	  undefined4 uStack_70;
  51	  undefined4 uStack_6c;
  52	  longlong lStack_60;
  53	  longlong lStack_58;
  54	  undefined8 uStack_50;
  55	  undefined1 uStack_41;
  56	  longlong lStack_40;
  57	  undefined1 uStack_31;
  58	  undefined8 uStack_30;
  59	  
  60	  uStack_30 = 0xfffffffffffffffe;
  61	  lStack_60 = 0;
  62	  uStack_50 = 0;
  63	  plStack_100 = param_1;
  64	  puVar4 = (undefined4 *)func_0x000140613c10(7,1);
  65	  if (puVar4 == (undefined4 *)0x0) {
  66	    FUN_1428d8fe3(1,7,&UNK_142cd3c28);
  67	    goto code_r0x0001417528ce;
  68	  }
  69	  *(undefined4 *)((longlong)puVar4 + 3) = 0x6e6f6973;
  70	  *puVar4 = 0x73726576;
  71	  lStack_f8 = 7;
  72	  lStack_e8 = 7;
  73	  plStack_d0 = (longlong *)CONCAT71(plStack_d0._1_7_,2);
  74	  uStack_c8 = (longlong *)0x0;
  75	  uStack_c0 = 1;
  76	  puStack_f0 = puVar4;
  77	  FUN_14143ba30(&plStack_88,&lStack_60,&lStack_f8,&plStack_d0);
  78	  if (false) {
  79	code_r0x0001417523ed:
  80	    FUN_140898f40(&uStack_80);
  81	  }
  82	  else {
  83	    switch((ulonglong)plStack_88 & 0xff) {
  84	    case 0:
  85	    case 1:
  86	    case 2:
  87	    case 6:
  88	      break;
  89	    case 3:
  90	      lVar6 = CONCAT44(uStack_7c,uStack_80);
  91	      if (lVar6 != 0) {
  92	        lVar8 = CONCAT44(uStack_74,uStack_78);
  93	        uVar5 = 1;
  94	code_r0x0001417523e6:
  95	        func_0x000140613c20(lVar8,lVar6,uVar5);
  96	      }
  97	      break;
  98	    case 4:
  99	      lStack_40 = CONCAT44(uStack_74,uStack_78);
 100	      FUN_140888d70(lStack_40,CONCAT44(uStack_6c,uStack_70));
 101	      if (CONCAT44(uStack_7c,uStack_80) != 0) {
 102	        lVar6 = CONCAT44(uStack_7c,uStack_80) << 5;
 103	        uVar5 = 8;
 104	        lVar8 = lStack_40;
 105	        goto code_r0x0001417523e6;
 106	      }
 107	      break;
 108	    default:
 109	      goto code_r0x0001417523ed;
 110	    }
 111	  }
 112	  puVar4 = (undefined4 *)func_0x000140613c10(5,1);
 113	  if (puVar4 == (undefined4 *)0x0) {
 114	    FUN_1428d8fe3(1,5,&UNK_142cd3c28);
 115	    goto code_r0x0001417528ce;
 116	  }
 117	  *(undefined1 *)(puVar4 + 1) = 0x65;
 118	  *puVar4 = 0x74617473;
 119	  lStack_f8 = 5;
 120	  lStack_e8 = 5;
 121	  uStack_31 = 1;
 122	  puStack_f0 = puVar4;
 123	  FUN_141329ca0(&plStack_d0,param_2);
 124	  if ((char)plStack_d0 == '\x06') {
 125	    plStack_88 = uStack_c8;
 126	    FUN_1428d9760(&UNK_142cd3bb0,0x2b,&plStack_88,&UNK_142cd3b90,&UNK_142cd3df0);
 127	    goto code_r0x0001417528ce;
 128	  }
 129	  uStack_78 = (undefined4)uStack_c0;
 130	  uStack_74 = uStack_c0._4_4_;
 131	  uStack_70 = uStack_b8;
 132	  uStack_6c = uStack_b4;
 133	  plStack_88 = plStack_d0;
 134	  uStack_80 = (undefined4)uStack_c8;
 135	  uStack_7c = uStack_c8._4_4_;
 136	  uStack_31 = 0;
 137	  plVar9 = &lStack_60;
 138	  FUN_14143ba30(&plStack_d0,&lStack_60,&lStack_f8,&plStack_88);
 139	  if (false) {
 140	code_r0x0001417524e6:
 141	    FUN_140898f40(&uStack_c8);
 142	  }
 143	  else {
 144	                    /* WARNING: Could not find normalized switch variable to match jumptable */
 145	    switch((ulonglong)plStack_d0 & 0xff) {
 146	    case 0:
 147	    case 1:
 148	    case 2:
 149	      break;
 150	    case 3:
 151	      plVar9 = uStack_c8;
 152	      if (uStack_c8 != (longlong *)0x0) {
 153	        uVar5 = 1;
 154	        lVar6 = uStack_c0;
 155	code_r0x0001417524df:
 156	        func_0x000140613c20(lVar6,plVar9,uVar5);
 157	      }
 158	      break;
 159	    case 4:
 160	      lStack_40 = uStack_c0;
 161	      FUN_140888d70(uStack_c0,CONCAT44(uStack_b4,uStack_b8));
 162	      plVar9 = (longlong *)0x0;
 163	      if (uStack_c8 != (longlong *)0x0) {
 164	        uVar5 = 8;
 165	        lVar6 = lStack_40;
 166	        plVar9 = (longlong *)((longlong)uStack_c8 << 5);
 167	        goto code_r0x0001417524df;
 168	      }
 169	      break;
 170	    default:
 171	      goto code_r0x0001417524e6;
 172	    }
 173	  }
 174	  lStack_118 = lStack_60;
 175	  lStack_110 = lStack_58;
 176	  uStack_108 = uStack_50;
 177	  abStack_120[0] = 5;
 178	  lStack_f8 = *plStack_100;
 179	  puStack_f0 = (undefined4 *)plStack_100[1];
 180	  lStack_e8 = plStack_100[2];
 181	  uStack_e0 = (undefined4)plStack_100[3];
 182	  uStack_dc = *(undefined4 *)((longlong)plStack_100 + 0x1c);
 183	  uVar5 = FUN_140d76ed0(&lStack_f8);
 184	  plVar10 = plVar9;
 185	  uStack_130 = uVar5;
 186	  plStack_128 = plVar9;
 187	  lVar6 = FUN_140d61cd0(uVar5);
 188	  if (lVar6 == 0) {
 189	code_r0x000141752568:
 190	    FUN_140d62a10(&plStack_88,uVar5,plVar9,&UNK_142cd3e08,3);
 191	    lVar6 = FUN_140d76ed0(&plStack_88);
 192	    lStack_60 = 0;
 193	    lStack_58 = 1;
 194	    uStack_50 = 0;
 195	    plStack_d0 = (longlong *)0x0;
 196	    uStack_c0 = 0;
 197	    uStack_b0 = 0x20;
 198	    uStack_af = 0;
 199	    uStack_ae = 0;
 200	    uStack_ad = 0;
 201	    uStack_ac = 0;
 202	    uStack_ab = 0;
 203	    uStack_aa = 0;
 204	    uStack_a8 = 3;
 205	    puStack_98 = &UNK_142cd3aa0;
 206	    lStack_140 = lVar6;
 207	    uStack_138 = uVar5;
 208	    plStack_a0 = &lStack_60;
 209	    cVar3 = FUN_140b883f0(abStack_120,&plStack_d0);
 210	    uVar2 = uStack_50;
 211	    if (cVar3 != '\0') {
 212	      FUN_1428d9760(&UNK_142cd3ad0,0x37,&uStack_41,&UNK_142cd3b70,&UNK_142cd3b58);
 213	code_r0x0001417528ce:
 214	                    /* WARNING: Does not return */
 215	      pcVar1 = (code *)invalidInstructionException();
 216	      (*pcVar1)();
 217	    }
 218	    lStack_40 = lStack_60;
 219	    lStack_90 = lStack_58;
 220	    plStack_d0 = (longlong *)((ulonglong)plStack_d0 & 0xffffffff00000000);
 221	    uStack_c8 = (longlong *)0x0;
 222	    uStack_c0 = 0;
 223	    uStack_b8 = 7;
 224	    uStack_b4 = 0;
 225	    uStack_b0 = 0;
 226	    uStack_ae = 0;
 227	    uStack_ab = 0;
 228	    uStack_af = 1;
 229	    uStack_ac = 1;
 230	    uStack_ad = 1;
 231	    uVar5 = FUN_140d59dc0(&plStack_d0,lVar6,uVar5);
 232	    lVar7 = FUN_1415a55d0(uVar5);
 233	    lVar8 = lStack_40;
 234	    if (lVar7 == 0) {
 235	      lStack_60 = lVar6;
 236	      uVar5 = FUN_140914c90(&lStack_60,lStack_90,uVar2);
 237	      lVar6 = FUN_1415a50c0(uVar5,&lStack_140);
 238	      if (lVar6 != 0) {
 239	code_r0x0001417526d1:
 240	        lVar8 = lStack_40;
 241	        CloseHandle(lStack_60);
 242	        goto joined_r0x00014175267b;
 243	      }
 244	      uVar5 = FUN_140d597e0(&lStack_60);
 245	      lVar6 = FUN_1415a5a10(uVar5,&lStack_140);
 246	      if (lVar6 != 0) goto code_r0x0001417526d1;
 247	      CloseHandle(lStack_60);
 248	      if (lStack_40 != 0) {
 249	        func_0x000140613c20(lStack_90,lStack_40,1);
 250	      }
 251	      uVar5 = FUN_140d76ed0(&plStack_88);
 252	      uVar5 = FUN_140d69870(uVar5);
 253	      lVar6 = FUN_1415a58b0(uVar5,&plStack_88,&uStack_130);
 254	      if (lVar6 == 0) {
 255	        if (plStack_88 != (longlong *)0x0) {
 256	          func_0x000140613c20(CONCAT44(uStack_7c,uStack_80),plStack_88,1);
 257	        }
 258	        FUN_14085da90(abStack_120);
 259	        lVar6 = 0;
 260	        goto joined_r0x000141752836;
 261	      }
 262	    }
 263	    else {
 264	joined_r0x00014175267b:
 265	      if (lVar8 != 0) {
 266	        func_0x000140613c20(lStack_90,lVar8,1);
 267	      }
 268	    }
 269	    if (plStack_88 != (longlong *)0x0) {
 270	      func_0x000140613c20(CONCAT44(uStack_7c,uStack_80),plStack_88,1);
 271	    }
 272	  }
 273	  else {
 274	    plStack_d0 = (longlong *)CONCAT71(plStack_d0._1_7_,1);
 275	    lVar6 = FUN_140d5a150(&plStack_d0,lVar6,plVar10);
 276	    if (lVar6 == 0) goto code_r0x000141752568;
 277	    lVar6 = FUN_1428a9590(lVar6);
 278	  }
 279	  if (abStack_120[0] < 3) goto joined_r0x000141752836;
 280	  if (abStack_120[0] == 3) {
 281	    if (lStack_118 == 0) goto joined_r0x000141752836;
 282	    uVar5 = 1;
 283	    lVar8 = lStack_110;
 284	    lVar7 = lStack_118;
 285	  }
 286	  else {
 287	    if (abStack_120[0] != 4) {
 288	      FUN_140898f40(&lStack_118);
 289	      goto joined_r0x000141752836;
 290	    }
 291	    lStack_40 = lStack_110;
 292	    FUN_140888d70(lStack_110,uStack_108);
 293	    if (lStack_118 == 0) goto joined_r0x000141752836;
 294	    uVar5 = 8;
 295	    lVar8 = lStack_40;
 296	    lVar7 = lStack_118 << 5;
 297	  }
 298	  func_0x000140613c20(lVar8,lVar7,uVar5);
 299	joined_r0x000141752836:
 300	  if (lStack_f8 != 0) {
 301	    func_0x000140613c20(puStack_f0,lStack_f8,1);
 302	  }
 303	  return lVar6;
 304	}
 305	