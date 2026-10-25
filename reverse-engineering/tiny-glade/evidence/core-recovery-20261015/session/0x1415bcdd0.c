   1	
   2	undefined1 * FUN_1415bcdd0(undefined1 *param_1,longlong param_2)
   3	
   4	{
   5	  undefined4 *puVar1;
   6	  longlong lVar2;
   7	  longlong lVar3;
   8	  char cVar4;
   9	  undefined4 uVar5;
  10	  longlong lVar6;
  11	  undefined8 uVar7;
  12	  char cVar8;
  13	  undefined1 auStack_110 [64];
  14	  longlong lStack_d0;
  15	  undefined4 uStack_c8;
  16	  undefined4 uStack_c4;
  17	  undefined4 uStack_c0;
  18	  undefined4 uStack_bc;
  19	  undefined4 uStack_b8;
  20	  undefined4 uStack_b4;
  21	  undefined4 uStack_b0;
  22	  undefined4 uStack_ac;
  23	  undefined8 uStack_a8;
  24	  byte bStack_a0;
  25	  byte bStack_9f;
  26	  undefined6 uStack_9e;
  27	  longlong lStack_98;
  28	  longlong lStack_90;
  29	  longlong *plStack_88;
  30	  undefined8 uStack_80;
  31	  undefined4 uStack_78;
  32	  undefined4 uStack_74;
  33	  undefined4 uStack_70;
  34	  undefined4 uStack_6c;
  35	  undefined4 uStack_68;
  36	  undefined4 uStack_64;
  37	  undefined4 uStack_60;
  38	  undefined4 uStack_5c;
  39	  longlong lStack_58;
  40	  byte bStack_4a;
  41	  byte bStack_49;
  42	  undefined8 uStack_48;
  43	  
  44	  uStack_48 = 0xfffffffffffffffe;
  45	  uStack_a8 = *(undefined8 *)(param_2 + 0x10);
  46	  func_0x000140b880d0(auStack_110);
  47	  bStack_49 = 2;
  48	  bStack_4a = 2;
  49	code_r0x0001415bce30:
  50	  do {
  51	    FUN_14143c450(&bStack_a0,auStack_110);
  52	    lVar6 = CONCAT62(uStack_9e,CONCAT11(bStack_9f,bStack_a0));
  53	    if (lVar6 == 0) {
  54	code_r0x0001415bd077:
  55	      if (bStack_4a == 2) {
  56	        lVar6 = FUN_1428f9bc0(&UNK_142c7ab68,0xd);
  57	      }
  58	      else {
  59	        if (bStack_49 != 2) {
  60	          if (lStack_d0 != 0) {
  61	            uVar7 = FUN_1428f9d20(uStack_a8,&UNK_142c7a098,&UNK_142c7a060);
  62	            *(undefined8 *)(param_1 + 8) = uVar7;
  63	          }
  64	          else {
  65	            param_1[1] = bStack_4a & 1;
  66	            param_1[2] = bStack_49 & 1;
  67	          }
  68	          *param_1 = lStack_d0 != 0;
  69	          FUN_140887df0(auStack_110);
  70	          goto code_r0x0001415bd0d2;
  71	        }
  72	        lVar6 = FUN_1428f9bc0(&UNK_142c7ab75,0xd);
  73	      }
  74	      goto code_r0x0001415bd0bf;
  75	    }
  76	    lVar2 = *(longlong *)(lVar6 + 0x168 + lStack_90 * 0x18);
  77	    plStack_88 = *(longlong **)(lVar6 + 0x170 + lStack_90 * 0x18);
  78	    lVar3 = *(longlong *)(lVar6 + 0x178 + lStack_90 * 0x18);
  79	    puVar1 = (undefined4 *)(lVar6 + lStack_90 * 0x20);
  80	    uStack_78 = *puVar1;
  81	    uStack_74 = puVar1[1];
  82	    uStack_70 = puVar1[2];
  83	    uStack_6c = puVar1[3];
  84	    puVar1 = (undefined4 *)(lVar6 + 0x10 + lStack_90 * 0x20);
  85	    uStack_68 = *puVar1;
  86	    uStack_64 = puVar1[1];
  87	    uStack_60 = puVar1[2];
  88	    uStack_5c = puVar1[3];
  89	    if (SBORROW8(0,lVar2)) goto code_r0x0001415bd077;
  90	    lStack_58 = lVar2;
  91	    if (false) {
  92	code_r0x0001415bcfbe:
  93	      FUN_140898f40(&uStack_c0);
  94	    }
  95	    else {
  96	      switch((char)uStack_c8) {
  97	      case '\0':
  98	      case '\x01':
  99	      case '\x02':
 100	      case '\x06':
 101	        break;
 102	      case '\x03':
 103	        if (CONCAT44(uStack_bc,uStack_c0) != 0) {
 104	          func_0x000140613c20(CONCAT44(uStack_b4,uStack_b8),CONCAT44(uStack_bc,uStack_c0),1);
 105	        }
 106	        break;
 107	      case '\x04':
 108	        uStack_80 = CONCAT44(uStack_b4,uStack_b8);
 109	        FUN_140888d70(uStack_80,CONCAT44(uStack_ac,uStack_b0));
 110	        if (CONCAT44(uStack_bc,uStack_c0) != 0) {
 111	          func_0x000140613c20(uStack_80,CONCAT44(uStack_bc,uStack_c0) << 5,8);
 112	        }
 113	        break;
 114	      default:
 115	        goto code_r0x0001415bcfbe;
 116	      }
 117	    }
 118	    uStack_b8 = uStack_68;
 119	    uStack_b4 = uStack_64;
 120	    uStack_b0 = uStack_60;
 121	    uStack_ac = uStack_5c;
 122	    uStack_c8 = uStack_78;
 123	    uStack_c4 = uStack_74;
 124	    uStack_c0 = uStack_70;
 125	    uStack_bc = uStack_6c;
 126	    cVar8 = '\x02';
 127	    if (lVar3 == 0xd) {
 128	      if (*(longlong *)((longlong)plStack_88 + 5) == 0x796c6e6f5f676e69 &&
 129	          *plStack_88 == 0x676e69746867696c) {
 130	        cVar8 = '\0';
 131	      }
 132	      else {
 133	        cVar8 = '\x02' - (*(longlong *)((longlong)plStack_88 + 5) == 0x757268745f656573 &&
 134	                         *plStack_88 == 0x6565735f65657274);
 135	      }
 136	    }
 137	    if (lStack_58 != 0) {
 138	      func_0x000140613c20(plStack_88,lStack_58,1);
 139	    }
 140	    uVar5 = uStack_c8;
 141	    cVar4 = (char)uStack_c8;
 142	    if (cVar8 == '\x02') break;
 143	    if (cVar8 == '\x01') {
 144	      if (bStack_49 != 2) {
 145	        lVar6 = FUN_1428f9db0(&UNK_142c7ab75,0xd);
 146	        goto code_r0x0001415bd0bf;
 147	      }
 148	      uStack_c8 = CONCAT31(uStack_c8._1_3_,6);
 149	      if (cVar4 == '\x06') {
 150	        lVar6 = FUN_1428fa000(&UNK_142c7a268,0x10);
 151	        goto code_r0x0001415bd0bf;
 152	      }
 153	      uStack_64 = uStack_b4;
 154	      uStack_60 = uStack_b0;
 155	      uStack_5c = uStack_ac;
 156	      uStack_78 = uVar5;
 157	      uStack_74 = uStack_c4;
 158	      uStack_70 = uStack_c0;
 159	      uStack_6c = uStack_bc;
 160	      uStack_68 = uStack_b8;
 161	      FUN_14159d4f0(&bStack_a0,&uStack_78);
 162	      lVar6 = lStack_98;
 163	      if ((bStack_a0 & 1) != 0) goto code_r0x0001415bd0bf;
 164	      bStack_49 = bStack_9f;
 165	    }
 166	    else {
 167	      if (bStack_4a != 2) {
 168	        lVar6 = FUN_1428f9db0(&UNK_142c7ab68,0xd);
 169	        goto code_r0x0001415bd0bf;
 170	      }
 171	      uStack_c8 = CONCAT31(uStack_c8._1_3_,6);
 172	      if (cVar4 == '\x06') {
 173	        lVar6 = FUN_1428fa000(&UNK_142c7a268,0x10);
 174	        goto code_r0x0001415bd0bf;
 175	      }
 176	      uStack_64 = uStack_b4;
 177	      uStack_60 = uStack_b0;
 178	      uStack_5c = uStack_ac;
 179	      uStack_78 = uVar5;
 180	      uStack_74 = uStack_c4;