   1	
   2	longlong FUN_1403b5920(longlong param_1,longlong *param_2)
   3	
   4	{
   5	  longlong *plVar1;
   6	  longlong lVar2;
   7	  longlong *plStack_28;
   8	  char cStack_20;
   9	  
  10	  plVar1 = (longlong *)*param_2;
  11	  lVar2 = plVar1[2];
  12	  if (*plVar1 == lVar2) {
  13	    FUN_1428a29e0(plVar1,lVar2,1,1,1);
  14	    lVar2 = plVar1[2];
  15	  }
  16	  *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
  17	  plVar1[2] = lVar2 + 1;
  18	  cStack_20 = '\x01';
  19	  plStack_28 = param_2;
  20	  lVar2 = FUN_1404c19e0(&plStack_28,&UNK_1429ccac0,2,param_1 + 0x60);
  21	  if (lVar2 == 0) {
  22	    lVar2 = FUN_1404bd4d0(&plStack_28,&UNK_1429ccac2,4,param_1);
  23	    if (lVar2 == 0) {
  24	      if (cStack_20 != '\0') {
  25	        plVar1 = (longlong *)*plStack_28;
  26	        lVar2 = plVar1[2];
  27	        if (*plVar1 == lVar2) {
  28	          FUN_1428a29e0(plVar1,lVar2,1,1,1);
  29	          lVar2 = plVar1[2];
  30	        }
  31	        *(undefined1 *)(plVar1[1] + lVar2) = 0x7d;
  32	        plVar1[2] = lVar2 + 1;
  33	      }
  34	      lVar2 = 0;
  35	    }
  36	  }
  37	  return lVar2;
  38	}
  39	