   1	
   2	uint * FUN_1415cc100(uint *param_1,longlong param_2)
   3	
   4	{
   5	  undefined8 *puVar1;
   6	  undefined4 uVar2;
   7	  longlong lVar3;
   8	  longlong lVar4;
   9	  undefined8 uVar5;
  10	  undefined8 uVar6;
  11	  undefined8 uVar7;
  12	  longlong lVar8;
  13	  undefined8 uVar9;
  14	  char cVar10;
  15	  undefined1 auVar12 [16];
  16	  undefined1 auVar13 [16];
  17	  undefined1 auVar14 [16];
  18	  undefined1 auVar16 [11];
  19	  undefined1 auVar15 [16];
  20	  uint unaff_XMM6_Da;
  21	  uint unaff_XMM7_Da;
  22	  uint unaff_XMM8_Da;
  23	  undefined1 auStack_150 [64];
  24	  longlong lStack_110;
  25	  char cStack_108;
  26	  undefined7 uStack_107;
  27	  undefined1 uStack_100;
  28	  undefined7 uStack_ff;
  29	  undefined1 uStack_f8;
  30	  undefined7 uStack_f7;
  31	  undefined8 uStack_f0;
  32	  undefined8 uStack_e0;
  33	  char cStack_d8;
  34	  undefined7 uStack_d7;
  35	  undefined1 uStack_d0;
  36	  undefined7 uStack_cf;
  37	  undefined1 uStack_c8;
  38	  undefined7 uStack_c7;
  39	  undefined8 uStack_c0;
  40	  uint uStack_b8;
  41	  uint uStack_b4;
  42	  longlong lStack_b0;
  43	  longlong lStack_a8;
  44	  longlong lStack_a0;
  45	  uint uStack_98;
  46	  uint uStack_94;
  47	  longlong *plStack_90;
  48	  uint uStack_88;
  49	  uint uStack_84;
  50	  undefined8 uStack_80;
  51	  char cVar11;
  52	  
  53	  uStack_80 = 0xfffffffffffffffe;
  54	  uVar9 = *(undefined8 *)(param_2 + 0x10);
  55	  uVar7 = func_0x000140b880d0(auStack_150);
  56	  uStack_94 = 0;
  57	  uStack_98 = (uint)CONCAT71((int7)((ulonglong)uVar7 >> 8),1);
  58	  uStack_84 = 0;
  59	  uStack_88 = 0;
  60	code_r0x0001415cc180:
  61	  do {
  62	    FUN_14143c450(&uStack_b8,auStack_150);
  63	    lVar8 = CONCAT44(uStack_b4,uStack_b8);
  64	    if (lVar8 == 0) {
  65	code_r0x0001415cc4d5:
  66	      if ((uStack_88 & 1) == 0) {
  67	        lVar8 = FUN_1428f9bc0(&UNK_142c7ab00,8);
  68	      }
  69	      else if ((uStack_84 & 1) == 0) {
  70	        lVar8 = FUN_1428f9bc0(&UNK_142c7ab08,0x14);
  71	      }
  72	      else {
  73	        if ((uStack_94 & 1) != 0) {
  74	          if (lStack_110 != 0) {
  75	            uVar9 = FUN_1428f9d20(uVar9,&UNK_142c7a098,&UNK_142c7a060);
  76	          }
  77	          else {
  78	            uVar9 = CONCAT44(unaff_XMM6_Da,unaff_XMM8_Da);
  79	            param_1[1] = unaff_XMM7_Da;
  80	          }
  81	          *(undefined8 *)(param_1 + 2) = uVar9;
  82	          *param_1 = (uint)(lStack_110 != 0);
  83	          FUN_140887df0(auStack_150);
  84	          goto code_r0x0001415cc5b8;
  85	        }
  86	        lVar8 = FUN_1428f9bc0(&UNK_142c7ab1c,0x11);
  87	      }
  88	      goto code_r0x0001415cc5a5;
  89	    }
  90	    lVar3 = *(longlong *)(lVar8 + 0x168 + lStack_a8 * 0x18);
  91	    plStack_90 = *(longlong **)(lVar8 + 0x170 + lStack_a8 * 0x18);
  92	    lVar4 = *(longlong *)(lVar8 + 0x178 + lStack_a8 * 0x18);
  93	    puVar1 = (undefined8 *)(lVar8 + lStack_a8 * 0x20);
  94	    uVar7 = *puVar1;
  95	    uVar5 = puVar1[1];
  96	    puVar1 = (undefined8 *)(lVar8 + 0x10 + lStack_a8 * 0x20);
  97	    uVar6 = *puVar1;
  98	    uStack_c0 = puVar1[1];
  99	    cStack_d8 = (char)uVar7;
 100	    uStack_d7 = (undefined7)((ulonglong)uVar7 >> 8);
 101	    uStack_d0 = (undefined1)uVar5;
 102	    uStack_cf = (undefined7)((ulonglong)uVar5 >> 8);
 103	    uStack_c8 = (undefined1)uVar6;
 104	    uStack_c7 = (undefined7)((ulonglong)uVar6 >> 8);
 105	    if (SBORROW8(0,lVar3)) goto code_r0x0001415cc4d5;
 106	    lStack_a0 = lVar3;
 107	    if (false) {
 108	code_r0x0001415cc39f:
 109	      FUN_140898f40(&uStack_100);
 110	    }
 111	    else {
 112	      switch(cStack_108) {
 113	      case '\0':
 114	      case '\x01':
 115	      case '\x02':
 116	      case '\x06':
 117	        break;
 118	      case '\x03':
 119	        if (CONCAT71(uStack_ff,uStack_100) != 0) {
 120	          func_0x000140613c20(CONCAT71(uStack_f7,uStack_f8),CONCAT71(uStack_ff,uStack_100),1);
 121	        }
 122	        break;
 123	      case '\x04':
 124	        uStack_e0 = CONCAT71(uStack_f7,uStack_f8);
 125	        FUN_140888d70(uStack_e0,uStack_f0);
 126	        if (CONCAT71(uStack_ff,uStack_100) != 0) {
 127	          func_0x000140613c20(uStack_e0,CONCAT71(uStack_ff,uStack_100) << 5,8);
 128	        }
 129	        break;
 130	      default:
 131	        goto code_r0x0001415cc39f;
 132	      }
 133	    }
 134	    uStack_f8 = uStack_c8;
 135	    uStack_f7 = uStack_c7;
 136	    uStack_f0 = uStack_c0;
 137	    cStack_108 = cStack_d8;
 138	    uStack_107 = uStack_d7;
 139	    uStack_100 = uStack_d0;
 140	    uStack_ff = uStack_cf;
 141	    if (lVar4 == 0x14) {
 142	      auVar13[0] = -((char)*plStack_90 == 'c');
 143	      auVar13[1] = -(*(char *)((longlong)plStack_90 + 1) == 'h');
 144	      auVar13[2] = -(*(char *)((longlong)plStack_90 + 2) == 'r');
 145	      auVar13[3] = -(*(char *)((longlong)plStack_90 + 3) == 'o');
 146	      auVar13[4] = -(*(char *)((longlong)plStack_90 + 4) == 'm');
 147	      auVar13[5] = -(*(char *)((longlong)plStack_90 + 5) == 'a');
 148	      auVar13[6] = -(*(char *)((longlong)plStack_90 + 6) == 't');
 149	      auVar13[7] = -(*(char *)((longlong)plStack_90 + 7) == 'i');
 150	      auVar13[8] = -((char)plStack_90[1] == 'c');
 151	      auVar13[9] = -(*(char *)((longlong)plStack_90 + 9) == '_');
 152	      auVar13[10] = -(*(char *)((longlong)plStack_90 + 10) == 'a');
 153	      auVar13[0xb] = -(*(char *)((longlong)plStack_90 + 0xb) == 'b');
 154	      auVar13[0xc] = -(*(char *)((longlong)plStack_90 + 0xc) == 'e');
 155	      auVar13[0xd] = -(*(char *)((longlong)plStack_90 + 0xd) == 'r');
 156	      auVar13[0xe] = -(*(char *)((longlong)plStack_90 + 0xe) == 'r');
 157	      auVar13[0xf] = -(*(char *)((longlong)plStack_90 + 0xf) == 'a');
 158	      uVar2 = (undefined4)plStack_90[2];
 159	      auVar15[0] = -((char)uVar2 == 't');
 160	      auVar15[1] = -((char)((uint)uVar2 >> 8) == 'i');
 161	      auVar15[2] = -((char)((uint)uVar2 >> 0x10) == 'o');
 162	      auVar15[3] = -((char)((uint)uVar2 >> 0x18) == 'n');
 163	      auVar16 = SUB1211((undefined1  [12])0x0,1);
 164	      auVar15[4] = 0xff;
 165	      auVar15[5] = 0xff;
 166	      auVar15[6] = 0xff;
 167	      auVar15[7] = 0xff;
 168	      auVar15[8] = 0xff;
 169	      auVar15[9] = 0xff;
 170	      auVar15[10] = 0xff;
 171	      auVar15[0xb] = 0xff;
 172	      auVar15[0xc] = 0xff;
 173	      auVar15[0xd] = -(auVar16[8] == '\0');
 174	      auVar15[0xe] = -(auVar16[9] == '\0');
 175	      auVar15[0xf] = -(auVar16[10] == '\0');
 176	      auVar15 = auVar15 & auVar13;
 177	      if ((ushort)((ushort)(SUB161(auVar15 >> 7,0) & 1) |
 178	                   (ushort)(SUB161(auVar15 >> 0xf,0) & 1) << 1 |
 179	                   (ushort)(SUB161(auVar15 >> 0x17,0) & 1) << 2 |
 180	                   (ushort)(SUB161(auVar15 >> 0x1f,0) & 1) << 3 |