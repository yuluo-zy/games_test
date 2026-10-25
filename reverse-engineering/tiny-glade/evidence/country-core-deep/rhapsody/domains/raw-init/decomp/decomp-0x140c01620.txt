
void FUN_140c01620(longlong param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  code *pcVar18;
  undefined4 *puVar19;
  longlong *plVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  undefined8 *puVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  longlong *plVar26;
  bool bVar27;
  longlong lStack_4a8;
  longlong *plStack_4a0;
  longlong lStack_498;
  longlong *plStack_490;
  undefined4 auStack_488 [2];
  undefined8 uStack_480;
  ulonglong uStack_478;
  ulonglong uStack_470;
  ulonglong uStack_468;
  ulonglong uStack_460;
  ulonglong uStack_458;
  ulonglong uStack_450;
  ulonglong uStack_448;
  ulonglong uStack_440;
  ulonglong uStack_438;
  ulonglong uStack_430;
  ulonglong uStack_428;
  ulonglong uStack_420;
  ulonglong uStack_418;
  ulonglong uStack_410;
  ulonglong uStack_408;
  ulonglong uStack_400;
  undefined8 uStack_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  longlong *plStack_378;
  longlong *plStack_370;
  longlong *plStack_368;
  longlong *plStack_360;
  undefined8 **ppuStack_358;
  undefined4 auStack_258 [2];
  undefined4 *puStack_250;
  undefined1 auStack_248 [520];
  
  uStack_388 = 0;
  uStack_380 = 0;
  uStack_398 = 0;
  uStack_390 = 0;
  uStack_3a8 = 0;
  uStack_3a0 = 0;
  uStack_3b8 = 0;
  uStack_3b0 = 0;
  uStack_3c8 = 0;
  uStack_3c0 = 0;
  uStack_3d8 = 0;
  uStack_3d0 = 0;
  uStack_3e8 = 0;
  uStack_3e0 = 0;
  uStack_3f8 = 0;
  uStack_3f0 = 0;
  uStack_408 = 0;
  uStack_400 = 0;
  uStack_418 = 0;
  uStack_410 = 0;
  uStack_428 = 0;
  uStack_420 = 0;
  uStack_438 = 0;
  uStack_430 = 0;
  uStack_448 = 0;
  uStack_440 = 0;
  uStack_458 = 0;
  uStack_450 = 0;
  uStack_468 = 0;
  uStack_460 = 0;
  uStack_478 = 0;
  uStack_470 = 0;
  auStack_488[0] = 0x3b9e67c8;
  uStack_480 = 0;
  puVar19 = auStack_488;
  do {
    puVar19 = *(undefined4 **)(puVar19 + 2);
  } while (puVar19 != (undefined4 *)0x0);
  auStack_258[0] = 0x3b9bb07e;
  puStack_250 = auStack_488;
  func_0x000142923c20(auStack_248,0,0x208);
  FUN_140d635e0();
  plVar26 = (longlong *)auStack_258;
  (**(code **)(*(longlong *)(param_1 + 0x88) + 0xd8))(*(undefined8 *)(param_1 + 0xc0));
  plVar1 = (longlong *)(param_1 + 0x608);
  plStack_368 = plVar1;
  plStack_368 = (longlong *)FUN_1418bc8e0(&plStack_368);
  uVar22 = plStack_368[4];
  uVar25 = plStack_368[5];
  uVar4 = plStack_368[6];
  uVar5 = plStack_368[7];
  uVar6 = plStack_368[8];
  uVar7 = plStack_368[9];
  uVar8 = plStack_368[10];
  uVar9 = plStack_368[0xb];
  uVar10 = plStack_368[0xc];
  uVar11 = plStack_368[0xd];
  uVar12 = plStack_368[0xe];
  uVar13 = plStack_368[0xf];
  uVar14 = plStack_368[0x10];
  uVar15 = plStack_368[0x11];
  uVar16 = plStack_368[0x12];
  uVar17 = plStack_368[0x13];
  plStack_360 = (longlong *)0x0;
  if (plVar26 == (longlong *)0x0) {
code_r0x000140c017f1:
    LOCK();
    *plStack_368 = *plStack_368 + -1;
    UNLOCK();
    if (*plStack_368 == 0) {
      func_0x000140bdd240(&plStack_368);
    }
  }
  else {
    plVar20 = plStack_368 + 2;
    LOCK();
    plVar2 = (longlong *)*plVar26;
    if (plVar20 == plVar2) {
      *plVar26 = 3;
    }
    UNLOCK();
    if (plVar20 != plVar2) goto code_r0x000140c017f1;
  }
  uVar24 = (uVar22 >> 3) + (ulonglong)(uVar22 >> 3 == 0);
  uVar21 = 0;
  if (uVar24 <= uVar22) {
    uVar21 = uVar22 - uVar24;
  }
  uVar24 = uVar24 + uVar22;
  if (uVar21 <= uVar24) {
    if (uStack_478 < uVar24) {
      uVar24 = uStack_478;
    }
    bVar27 = uStack_478 < uVar21;
    uStack_478 = uVar24;
    if (bVar27) {
      uStack_478 = uVar21;
    }
    uVar22 = (uVar25 >> 3) + (ulonglong)(uVar25 >> 3 == 0);
    uVar21 = 0;
    if (uVar22 <= uVar25) {
      uVar21 = uVar25 - uVar22;
    }
    uVar22 = uVar22 + uVar25;
    if (uVar21 <= uVar22) {
      if (uStack_470 < uVar22) {
        uVar22 = uStack_470;
      }
      bVar27 = uStack_470 < uVar21;
      uStack_470 = uVar22;
      if (bVar27) {
        uStack_470 = uVar21;
      }
      uVar25 = (uVar4 >> 3) + (ulonglong)(uVar4 >> 3 == 0);
      uVar22 = 0;
      if (uVar25 <= uVar4) {
        uVar22 = uVar4 - uVar25;
      }
      uVar25 = uVar25 + uVar4;
      if (uVar22 <= uVar25) {
        if (uStack_468 < uVar25) {
          uVar25 = uStack_468;
        }
        bVar27 = uStack_468 < uVar22;
        uStack_468 = uVar25;
        if (bVar27) {
          uStack_468 = uVar22;
        }
        uVar22 = (uVar5 >> 3) + (ulonglong)(uVar5 >> 3 == 0);
        uVar25 = 0;
        if (uVar22 <= uVar5) {
          uVar25 = uVar5 - uVar22;
        }
        uVar22 = uVar22 + uVar5;
        if (uVar25 <= uVar22) {
          if (uStack_460 < uVar22) {
            uVar22 = uStack_460;
          }
          bVar27 = uStack_460 < uVar25;
          uStack_460 = uVar22;
          if (bVar27) {
            uStack_460 = uVar25;
          }
          uVar25 = (uVar6 >> 3) + (ulonglong)(uVar6 >> 3 == 0);
          uVar22 = 0;
          if (uVar25 <= uVar6) {
            uVar22 = uVar6 - uVar25;
          }
          uVar25 = uVar25 + uVar6;
          if (uVar22 <= uVar25) {
            if (uStack_458 < uVar25) {
              uVar25 = uStack_458;
            }
            bVar27 = uStack_458 < uVar22;
            uStack_458 = uVar25;
            if (bVar27) {
              uStack_458 = uVar22;
            }
            uVar22 = (uVar7 >> 3) + (ulonglong)(uVar7 >> 3 == 0);
            uVar25 = 0;
            if (uVar22 <= uVar7) {
              uVar25 = uVar7 - uVar22;
            }
            uVar22 = uVar22 + uVar7;
            if (uVar25 <= uVar22) {
              if (uStack_450 < uVar22) {
                uVar22 = uStack_450;
              }
              bVar27 = uStack_450 < uVar25;
              uStack_450 = uVar22;
              if (bVar27) {
                uStack_450 = uVar25;
              }
              uVar25 = (uVar8 >> 3) + (ulonglong)(uVar8 >> 3 == 0);
              uVar22 = 0;
              if (uVar25 <= uVar8) {
                uVar22 = uVar8 - uVar25;
              }
              uVar25 = uVar25 + uVar8;
              if (uVar22 <= uVar25) {
                if (uStack_448 < uVar25) {
                  uVar25 = uStack_448;
                }
                bVar27 = uStack_448 < uVar22;
                uStack_448 = uVar25;
                if (bVar27) {
                  uStack_448 = uVar22;
                }
                uVar22 = (uVar9 >> 3) + (ulonglong)(uVar9 >> 3 == 0);
                uVar25 = 0;
                if (uVar22 <= uVar9) {
                  uVar25 = uVar9 - uVar22;
                }
                uVar22 = uVar22 + uVar9;
                if (uVar25 <= uVar22) {
                  if (uStack_440 < uVar22) {
                    uVar22 = uStack_440;
                  }
                  bVar27 = uStack_440 < uVar25;
                  uStack_440 = uVar22;
                  if (bVar27) {
                    uStack_440 = uVar25;
                  }
                  uVar25 = (uVar10 >> 3) + (ulonglong)(uVar10 >> 3 == 0);
                  uVar22 = 0;
                  if (uVar25 <= uVar10) {
                    uVar22 = uVar10 - uVar25;
                  }
                  uVar25 = uVar25 + uVar10;
                  if (uVar22 <= uVar25) {
                    if (uStack_438 < uVar25) {
                      uVar25 = uStack_438;
                    }
                    bVar27 = uStack_438 < uVar22;
                    uStack_438 = uVar25;
                    if (bVar27) {
                      uStack_438 = uVar22;
                    }
                    uVar22 = (uVar11 >> 3) + (ulonglong)(uVar11 >> 3 == 0);
                    uVar25 = 0;
                    if (uVar22 <= uVar11) {
                      uVar25 = uVar11 - uVar22;
                    }
                    uVar22 = uVar22 + uVar11;
                    if (uVar25 <= uVar22) {
                      if (uStack_430 < uVar22) {
                        uVar22 = uStack_430;
                      }
                      bVar27 = uStack_430 < uVar25;
                      uStack_430 = uVar22;
                      if (bVar27) {
                        uStack_430 = uVar25;
                      }
                      uVar25 = (uVar12 >> 3) + (ulonglong)(uVar12 >> 3 == 0);
                      uVar22 = 0;
                      if (uVar25 <= uVar12) {
                        uVar22 = uVar12 - uVar25;
                      }
                      uVar25 = uVar25 + uVar12;
                      if (uVar22 <= uVar25) {
                        if (uStack_428 < uVar25) {
                          uVar25 = uStack_428;
                        }
                        bVar27 = uStack_428 < uVar22;
                        uStack_428 = uVar25;
                        if (bVar27) {
                          uStack_428 = uVar22;
                        }
                        uVar22 = (uVar13 >> 3) + (ulonglong)(uVar13 >> 3 == 0);
                        uVar25 = 0;
                        if (uVar22 <= uVar13) {
                          uVar25 = uVar13 - uVar22;
                        }
                        uVar22 = uVar22 + uVar13;
                        if (uVar25 <= uVar22) {
                          if (uStack_420 < uVar22) {
                            uVar22 = uStack_420;
                          }
                          bVar27 = uStack_420 < uVar25;
                          uStack_420 = uVar22;
                          if (bVar27) {
                            uStack_420 = uVar25;
                          }
                          uVar25 = (uVar14 >> 3) + (ulonglong)(uVar14 >> 3 == 0);
                          uVar22 = 0;
                          if (uVar25 <= uVar14) {
                            uVar22 = uVar14 - uVar25;
                          }
                          uVar25 = uVar25 + uVar14;
                          if (uVar22 <= uVar25) {
                            if (uStack_418 < uVar25) {
                              uVar25 = uStack_418;
                            }
                            bVar27 = uStack_418 < uVar22;
                            uStack_418 = uVar25;
                            if (bVar27) {
                              uStack_418 = uVar22;
                            }
                            uVar22 = (uVar15 >> 3) + (ulonglong)(uVar15 >> 3 == 0);
                            uVar25 = 0;
                            if (uVar22 <= uVar15) {
                              uVar25 = uVar15 - uVar22;
                            }
                            uVar22 = uVar22 + uVar15;
                            if (uVar25 <= uVar22) {
                              if (uStack_410 < uVar22) {
                                uVar22 = uStack_410;
                              }
                              bVar27 = uStack_410 < uVar25;
                              uStack_410 = uVar22;
                              if (bVar27) {
                                uStack_410 = uVar25;
                              }
                              uVar25 = (uVar16 >> 3) + (ulonglong)(uVar16 >> 3 == 0);
                              uVar22 = 0;
                              if (uVar25 <= uVar16) {
                                uVar22 = uVar16 - uVar25;
                              }
                              uVar25 = uVar25 + uVar16;
                              if (uVar22 <= uVar25) {
                                if (uStack_408 < uVar25) {
                                  uVar25 = uStack_408;
                                }
                                bVar27 = uStack_408 < uVar22;
                                uStack_408 = uVar25;
                                if (bVar27) {
                                  uStack_408 = uVar22;
                                }
                                uVar22 = (uVar17 >> 3) + (ulonglong)(uVar17 >> 3 == 0);
                                uVar25 = 0;
                                if (uVar22 <= uVar17) {
                                  uVar25 = uVar17 - uVar22;
                                }
                                uVar22 = uVar22 + uVar17;
                                if (uVar25 <= uVar22) {
                                  if (uStack_400 < uVar22) {
                                    uVar22 = uStack_400;
                                  }
                                  bVar27 = uStack_400 < uVar25;
                                  uStack_400 = uVar22;
                                  if (bVar27) {
                                    uStack_400 = uVar25;
                                  }
                                  func_0x000142923c00(&plStack_368,auStack_488,0x110);
                                  puVar23 = (undefined8 *)func_0x000140613c10(0x120,8);
                                  if (puVar23 != (undefined8 *)0x0) {
                                    *puVar23 = 1;
                                    puVar23[1] = 1;
                                    func_0x000142923c00(puVar23 + 2,&plStack_368,0x110);
                                    LOCK();
                                    lVar3 = *(longlong *)(param_1 + 0x608);
                                    *(longlong *)(param_1 + 0x608) = (longlong)(puVar23 + 2);
                                    UNLOCK();
                                    lStack_4a8 = param_1 + 0x610;
                                    plStack_378 = &lStack_4a8;
                                    plStack_370 = (longlong *)&plStack_4a0;
                                    plStack_368 = &lStack_498;
                                    plStack_360 = (longlong *)&plStack_490;
                                    ppuStack_358 = &plStack_378;
                                    plStack_4a0 = plVar1;
                                    lStack_498 = lVar3;
                                    plStack_490 = plVar1;
                                    FUN_1418bc6c0(&plStack_368);
                                    plStack_368 = (longlong *)(lVar3 + -0x10);
                                    LOCK();
                                    plVar1 = (longlong *)(lVar3 + -0x10);
                                    *plVar1 = *plVar1 + -1;
                                    UNLOCK();
                                    if (*plVar1 == 0) {
                                      func_0x000140bdd240(&plStack_368);
                                    }
                                    return;
                                  }
                                  goto code_r0x000140c01de9;
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  FUN_1428d9430(&UNK_142b17660,0x1c,&UNK_142b176c8);
code_r0x000140c01de9:
  FUN_1428d9000(8,0x120);
  pcVar18 = (code *)swi(3);
  (*pcVar18)();
  return;
}

