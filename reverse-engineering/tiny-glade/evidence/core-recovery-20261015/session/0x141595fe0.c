   1	
   2	undefined8 FUN_141595fe0(longlong *param_1,undefined8 param_2,longlong param_3,undefined8 param_4)
   3	
   4	{
   5	  longlong lVar1;
   6	  code *pcVar2;
   7	  longlong lVar3;
   8	  undefined8 uVar4;
   9	  undefined4 uStack_c8;
  10	  undefined4 uStack_c4;
  11	  undefined4 uStack_c0;
  12	  undefined4 uStack_bc;
  13	  undefined4 uStack_b8;
  14	  undefined4 uStack_b4;
  15	  undefined4 uStack_b0;
  16	  undefined4 uStack_ac;
  17	  undefined4 uStack_a0;
  18	  undefined4 uStack_9c;
  19	  undefined4 uStack_98;
  20	  undefined4 uStack_94;
  21	  undefined4 uStack_90;
  22	  undefined4 uStack_8c;
  23	  undefined4 uStack_88;
  24	  undefined4 uStack_84;
  25	  longlong lStack_80;
  26	  longlong lStack_78;
  27	  longlong lStack_70;
  28	  undefined1 auStack_68 [8];
  29	  longlong lStack_60;
  30	  longlong lStack_58;
  31	  undefined8 uStack_50;
  32	  longlong lStack_48;
  33	  undefined8 uStack_40;
  34	  
  35	  uStack_40 = 0xfffffffffffffffe;
  36	  if (param_3 < 0) {
  37	    uVar4 = 0;
  38	code_r0x000141596189:
  39	    FUN_1428d8fe3(uVar4,param_3,&UNK_142c781d8);
  40	    pcVar2 = (code *)swi(3);
  41	    uVar4 = (*pcVar2)();
  42	    return uVar4;
  43	  }
  44	  if (param_3 == 0) {
  45	    lVar3 = 1;
  46	    lVar1 = *param_1;
  47	  }
  48	  else {
  49	    uVar4 = 1;
  50	    lVar3 = func_0x000140613c10(param_3,1);
  51	    if (lVar3 == 0) goto code_r0x000141596189;
  52	    func_0x000142923c00(lVar3,param_2,param_3);
  53	    lVar1 = *param_1;
  54	  }
  55	  if ((!SBORROW8(0,lVar1)) && (lVar1 != 0)) {
  56	    func_0x000140613c20(param_1[1],lVar1,1);
  57	  }
  58	  param_1[1] = lVar3;
  59	  param_1[2] = param_3;
  60	  *param_1 = -0x8000000000000000;
  61	  lStack_78 = param_1[1];
  62	  lStack_70 = param_1[2];
  63	  lStack_80 = param_3;
  64	  lStack_48 = param_3;
  65	  FUN_14085faa0(&uStack_a0,param_4);
  66	  if ((char)uStack_a0 == '\x06') {
  67	    if (lStack_48 != 0) {
  68	      func_0x000140613c20(lStack_78,lStack_48,1);
  69	      return CONCAT44(uStack_94,uStack_98);
  70	    }
  71	    return CONCAT44(uStack_94,uStack_98);
  72	  }
  73	  uStack_b8 = uStack_90;
  74	  uStack_b4 = uStack_8c;
  75	  uStack_b0 = uStack_88;
  76	  uStack_ac = uStack_84;
  77	  uStack_c8 = uStack_a0;
  78	  uStack_c4 = uStack_9c;
  79	  uStack_c0 = uStack_98;
  80	  uStack_bc = uStack_94;
  81	  FUN_14143ba30(auStack_68,param_1 + 3,&lStack_80,&uStack_c8);
  82	  if (false) {
  83	code_r0x000141596169:
  84	    FUN_140898f40(&lStack_60);
  85	  }
  86	  else {
  87	    switch(auStack_68[0]) {
  88	    case 0:
  89	    case 1:
  90	    case 2:
  91	    case 6:
  92	      goto code_r0x000141596174;
  93	    case 3:
  94	      if (lStack_60 == 0) {
  95	        return 0;
  96	      }
  97	      uVar4 = 1;
  98	      break;
  99	    case 4:
 100	      lStack_48 = lStack_58;
 101	      FUN_140888d70(lStack_58,uStack_50);
 102	      if (lStack_60 == 0) {
 103	        return 0;
 104	      }
 105	      lStack_60 = lStack_60 << 5;
 106	      uVar4 = 8;
 107	      lStack_58 = lStack_48;
 108	      break;
 109	    default:
 110	      goto code_r0x000141596169;
 111	    }
 112	    func_0x000140613c20(lStack_58,lStack_60,uVar4);
 113	  }
 114	code_r0x000141596174:
 115	  return 0;
 116	}
 117	