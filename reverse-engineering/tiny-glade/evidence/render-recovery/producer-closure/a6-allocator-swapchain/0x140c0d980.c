
longlong * FUN_140c0d980(longlong *param_1,undefined8 param_2,undefined1 param_3)

{
  longlong lStack_640;
  longlong lStack_638;
  longlong lStack_630;
  undefined4 uStack_628;
  undefined4 uStack_624;
  undefined4 uStack_620;
  undefined4 uStack_61c;
  undefined1 auStack_618 [1528];
  
  FUN_140c120d0(&lStack_640);
  if (SBORROW8(0,lStack_640)) {
    param_1[3] = CONCAT44(uStack_624,uStack_628);
    param_1[4] = CONCAT44(uStack_61c,uStack_620);
    param_1[1] = lStack_638;
    param_1[2] = lStack_630;
  }
  else {
    func_0x000142923c00(param_1 + 5,auStack_618,0x5f8);
    param_1[3] = CONCAT44(uStack_624,uStack_628);
    param_1[4] = CONCAT44(uStack_61c,uStack_620);
    param_1[1] = lStack_638;
    param_1[2] = lStack_630;
    param_1[0xc4] = -0x8000000000000000;
    param_1[0xce] = 0;
    param_1[0xcf] = 0;
    param_1[0xd0] = 0;
    *(undefined1 *)(param_1 + 0xd1) = param_3;
  }
  *param_1 = lStack_640;
  return param_1;
}

