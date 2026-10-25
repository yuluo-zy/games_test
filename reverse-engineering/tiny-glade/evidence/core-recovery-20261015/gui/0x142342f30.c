   1	
   2	void FUN_142342f30(longlong *param_1,undefined8 *param_2,longlong *param_3,longlong *param_4,
   3	                  longlong *param_5,longlong param_6)
   4	
   5	{
   6	  uint *puVar1;
   7	  byte bVar2;
   8	  undefined1 uVar3;
   9	  int iVar4;
  10	  undefined4 uVar5;
  11	  undefined4 uVar6;
  12	  uint uVar7;
  13	  longlong lVar8;
  14	  longlong lVar9;
  15	  longlong lVar10;
  16	  undefined4 *puVar11;
  17	  undefined4 *puVar12;
  18	  undefined8 uVar13;
  19	  longlong lVar14;
  20	  ulonglong uVar15;
  21	  longlong lVar16;
  22	  ulonglong uVar17;
  23	  ulonglong *puVar18;
  24	  uint *puVar19;
  25	  longlong lVar20;
  26	  ulonglong uVar21;
  27	  ulonglong uVar22;
  28	  longlong lVar23;
  29	  ulonglong uVar24;
  30	  ulonglong uVar25;
  31	  ulonglong uVar26;
  32	  ulonglong uVar27;
  33	  longlong lVar28;
  34	  ulonglong *puVar29;
  35	  ulonglong uVar30;
  36	  longlong lVar31;
  37	  longlong lVar32;
  38	  ulonglong uVar33;
  39	  bool bVar34;
  40	  bool bVar35;
  41	  longlong *plStack_98;
  42	  
  43	  puVar29 = (ulonglong *)*param_2;
  44	  lVar8 = param_2[1];
  45	  uVar24 = *puVar29;
  46	  uVar25 = 0;
  47	  uVar30 = uVar24 - *(ulonglong *)(lVar8 + 0x18);
  48	  if (uVar24 < *(ulonglong *)(lVar8 + 0x18)) {
  49	    uVar30 = uVar25;
  50	  }
  51	  uVar17 = uVar24 - *(ulonglong *)(lVar8 + 0x38);
  52	  if (uVar24 < *(ulonglong *)(lVar8 + 0x38)) {
  53	    uVar17 = uVar25;
  54	  }
  55	  lVar23 = uVar30 * 0x10 + *(longlong *)(lVar8 + 8);
  56	  uVar24 = *(ulonglong *)(lVar8 + 0x10) - uVar30;
  57	  if (*(ulonglong *)(lVar8 + 0x10) < uVar30) {
  58	    lVar23 = 8;
  59	    uVar24 = uVar25;
  60	  }
  61	  lVar16 = uVar17 * 0x10 + *(longlong *)(lVar8 + 0x28);
  62	  uVar30 = *(ulonglong *)(lVar8 + 0x30) - uVar17;
  63	  if (*(ulonglong *)(lVar8 + 0x30) < uVar17) {
  64	    lVar16 = 8;
  65	    uVar30 = uVar25;
  66	  }
  67	  *puVar29 = (*(longlong *)(lVar8 + 0x40) - uVar30) - uVar24;
  68	  plStack_98 = *(longlong **)(param_6 + 8);
  69	  if (plStack_98 == (longlong *)0x0) {
  70	    plStack_98 = *(longlong **)(param_6 + 0x10);
  71	  }
  72	  lVar31 = uVar24 * 0x10 + lVar23;
  73	  lVar32 = uVar30 * 0x10 + lVar16;
  74	  lVar8 = *param_1;
  75	  lVar9 = param_1[1];
  76	  iVar4 = *(int *)((longlong)param_1 + 0x14);
  77	  lVar10 = *param_3;
  78	  puVar11 = (undefined4 *)param_3[2];
  79	  uVar5 = *(undefined4 *)((longlong)param_3 + 0x1c);
  80	  uVar6 = *(undefined4 *)((longlong)param_4 + 0x1c);
  81	  lVar28 = *param_4;
  82	  puVar12 = (undefined4 *)param_4[2];
  83	  do {
  84	    if ((lVar23 == 0) || (lVar23 == lVar31)) {
  85	      if ((lVar16 == 0) || (lVar16 == lVar32)) {
  86	        puVar29 = (ulonglong *)*param_5;
  87	        lVar23 = param_5[1];
  88	        uVar24 = *puVar29;
  89	        uVar25 = 0;
  90	        uVar30 = uVar24 - *(ulonglong *)(lVar23 + 0x18);
  91	        if (uVar24 < *(ulonglong *)(lVar23 + 0x18)) {
  92	          uVar30 = uVar25;
  93	        }
  94	        uVar17 = uVar24 - *(ulonglong *)(lVar23 + 0x38);
  95	        if (uVar24 < *(ulonglong *)(lVar23 + 0x38)) {
  96	          uVar17 = uVar25;
  97	        }
  98	        uVar24 = *(ulonglong *)(lVar23 + 0x10);
  99	        uVar15 = *(ulonglong *)(lVar23 + 0x30);
 100	        bVar34 = uVar15 < uVar17;
 101	        uVar21 = uVar15 - uVar17;
 102	        uVar22 = uVar21;
 103	        if (bVar34) {
 104	          uVar22 = uVar25;
 105	        }
 106	        bVar35 = uVar24 < uVar30;
 107	        uVar26 = uVar24 - uVar30;
 108	        uVar27 = uVar26;
 109	        if (bVar35) {
 110	          uVar27 = uVar25;
 111	        }
 112	        lVar16 = *(longlong *)(lVar23 + 8);
 113	        uVar33 = uVar22 + uVar27;
 114	        if (!bVar35 && uVar26 != 0 || !bVar34 && uVar21 != 0) {
 115	          uVar33 = uVar25;
 116	        }
 117	        lVar28 = *(longlong *)(lVar23 + 0x28);
 118	        *puVar29 = *(longlong *)(lVar23 + 0x40) - uVar33;
 119	        if (!bVar35 && uVar26 != 0 || !bVar34 && uVar21 != 0) {
 120	          lVar16 = lVar16 + uVar30 * 0x28;
 121	          if (uVar24 < uVar30) {
 122	            lVar16 = 8;
 123	          }
 124	          puVar29 = (ulonglong *)0x0;
 125	          if (uVar30 < uVar24) {
 126	            puVar29 = (ulonglong *)(lVar16 + -0x28 + uVar27 * 0x28);
 127	          }
 128	          lVar23 = lVar28 + uVar17 * 0x28;
 129	          if (uVar15 < uVar17) {
 130	            lVar23 = 8;
 131	          }
 132	          puVar18 = (ulonglong *)(lVar23 + -0x28 + uVar22 * 0x28);
 133	          if (uVar15 <= uVar17) {
 134	            puVar18 = puVar29;
 135	          }
 136	          if (((int)*puVar18 != 0xd) && (*(char *)(lVar10 + 10) == '\x01')) {
 137	            *puVar11 = uVar5;
 138	            *(undefined1 *)(lVar10 + 10) = 0;
 139	          }
 140	        }
 141	        puVar19 = *(uint **)(lVar8 + 0xf8);
 142	        puVar1 = puVar19 + *(longlong *)(lVar8 + 0x100);
 143	        if (*(char *)(lVar8 + 0x108) == '\0') {
 144	          lVar16 = 8;
 145	          lVar23 = 0;
 146	          lVar31 = 0;
 147	          lVar32 = 0;
 148	          do {
 149	            if (lVar32 == lVar31) {
 150	              if (puVar19 == puVar1) {
 151	                return;
 152	              }
 153	              while( true ) {
 154	                lVar31 = *(longlong *)
 155	                          (*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
 156	                if (lVar31 != 0) break;
 157	                puVar19 = puVar19 + 1;
 158	                if (puVar19 == puVar1) {
 159	                  return;
 160	                }
 161	              }
 162	              lVar32 = *(longlong *)(lVar9 + 0x100) + (ulonglong)*puVar19 * 0xa0;
 163	              uVar24 = (ulonglong)*(uint *)(lVar32 + 0x94);
 164	              lVar16 = *(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x18 + uVar24 * 0x48);
 165	              lVar28 = ~*(ulonglong *)
 166	                         (*(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x38 + uVar24 * 0x48) +
 167	                         *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
 168	              lVar23 = *(longlong *)(lVar16 + 0x10 + lVar28);
 169	              lVar28 = *(longlong *)(lVar16 + 0x28 + lVar28);
 170	              lVar16 = *(longlong *)(lVar32 + 0x50);
 171	              puVar19 = puVar19 + 1;
 172	              lVar32 = 0;
 173	            }
 174	            uVar24 = (ulonglong)*(uint *)(lVar16 + 8 + lVar32 * 0x10);
 175	            lVar32 = lVar32 + 1;
 176	            uVar3 = *(undefined1 *)(lVar10 + 10);
 177	            *(int *)(lVar28 + uVar24 * 4) = iVar4;
 178	            *(undefined1 *)(lVar23 + 2 + uVar24 * 3) = uVar3;
 179	          } while( true );
 180	        }
 181	        lVar23 = 0;
 182	        uVar24 = 0;
 183	        uVar30 = 0;
 184	        do {
 185	          if (uVar30 == uVar24) {
 186	            if (puVar19 == puVar1) {
 187	              return;
 188	            }
 189	            while( true ) {
 190	              uVar24 = *(ulonglong *)
 191	                        (*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48);
 192	              if (uVar24 != 0) break;
 193	              puVar19 = puVar19 + 1;
 194	              if (puVar19 == puVar1) {
 195	                return;
 196	              }
 197	            }
 198	            lVar23 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
 199	            lVar16 = *(longlong *)(lVar23 + 0x18);
 200	            lVar28 = ~*(ulonglong *)
 201	                       (*(longlong *)(lVar23 + 0x38) + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
 202	            lVar23 = *(longlong *)(lVar16 + 0x10 + lVar28);
 203	            puVar29 = *(ulonglong **)(lVar16 + 0x28 + lVar28);
 204	            puVar19 = puVar19 + 1;
 205	            uVar30 = 0;
 206	          }
 207	          uVar25 = uVar30 & 0xffffffff;
 208	          uVar30 = uVar30 + 1;
 209	          uVar3 = *(undefined1 *)(lVar10 + 10);
 210	          *(int *)((longlong)puVar29 + uVar25 * 4) = iVar4;
 211	          *(undefined1 *)(lVar23 + 2 + uVar25 * 3) = uVar3;
 212	        } while( true );
 213	      }
 214	      lVar23 = 0;
 215	      lVar14 = lVar16 + 0x10;
 216	      lVar20 = lVar16;
 217	    }
 218	    else {
 219	      lVar14 = lVar16;
 220	      lVar20 = lVar23;
 221	      lVar23 = lVar23 + 0x10;
 222	    }
 223	    lVar16 = lVar14;
 224	    *puVar29 = *puVar29 + 1;
 225	    uVar24 = *(ulonglong *)(lVar20 + 8) & 0xffffffff;
 226	    if ((uVar24 < *(ulonglong *)(lVar9 + 0x10)) &&
 227	       (*(int *)(*(longlong *)(lVar9 + 8) + uVar24 * 0x14) ==
 228	        (int)(*(ulonglong *)(lVar20 + 8) >> 0x20))) {
 229	      uVar7 = *(uint *)(*(longlong *)(lVar9 + 8) + uVar24 * 0x14 + 4);
 230	      uVar24 = (ulonglong)uVar7;
 231	      if ((uVar24 != 0xffffffff) &&
 232	         ((uVar24 < *(ulonglong *)(lVar8 + 0x38) &&
 233	          ((*(ulonglong *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)(uVar7 >> 6) * 8) >>
 234	            (uVar24 & 0x3f) & 1) != 0)))) {
 235	        bVar2 = *(byte *)(lVar10 + 10);
 236	        *puVar11 = uVar5;
 237	        *(byte *)(lVar10 + 10) = bVar2 ^ 1;
 238	        uVar13 = *(undefined8 *)(lVar20 + 8);
 239	        lVar14 = plStack_98[2];
 240	        if ((ulonglong)(*plStack_98 - lVar14) < 0x28) {
 241	          FUN_1428a58d0(plStack_98,lVar14,0x28,1,1);
 242	        }
 243	        lVar20 = plStack_98[1];
 244	        *(code **)(lVar20 + lVar14) = FUN_140fdf200;
 245	        *(code **)(lVar20 + 8 + lVar14) = FUN_141e4f860;
 246	        *(undefined8 *)(lVar20 + 0x10 + lVar14) = uVar13;
 247	        *(undefined8 *)(lVar20 + 0x18 + lVar14) = 0x3e4ccccd3dcccccd;
 248	        *(undefined4 *)(lVar20 + 0x20 + lVar14) = 0;
 249	        *(undefined1 *)(lVar20 + 0x24 + lVar14) = 0;
 250	        plStack_98[2] = lVar14 + 0x28;
 251	        if (*(char *)(lVar10 + 10) == '\x01') {
 252	          *puVar12 = uVar6;
 253	          *(undefined8 *)(lVar28 + 0x10) = 0;
 254	        }
 255	      }
 256	    }
 257	  } while( true );
 258	}
 259	