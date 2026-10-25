0000000141339440: 53                       push       rbx
0000000141339441: 4883ec40                 sub        rsp, 0x40
0000000141339445: 498d40ff                 lea        rax, [r8 - 1]
0000000141339449: 4839d0                   cmp        rax, rdx
000000014133944c: 0f83e1000000             jae        0x141339533
0000000141339452: 4939d0                   cmp        r8, rdx
0000000141339455: 7506                     jne        0x14133945d
0000000141339457: 4883c440                 add        rsp, 0x40
000000014133945b: 5b                       pop        rbx
000000014133945c: c3                       ret        
000000014133945d: 48c1e206                 shl        rdx, 6
0000000141339461: 4801ca                   add        rdx, rcx
0000000141339464: 49c1e006                 shl        r8, 6
0000000141339468: 4a8d0401                 lea        rax, [rcx + r8]
000000014133946c: eb3b                     jmp        0x1413394a9
000000014133946e: 6690                     nop        
0000000141339470: 4989ca                   mov        r10, rcx
0000000141339473: 4d890a                   mov        qword ptr [r10], r9
0000000141339476: 0f280424                 movaps     xmm0, xmmword ptr [rsp]
000000014133947a: 0f284c2410               movaps     xmm1, xmmword ptr [rsp + 0x10]
000000014133947f: 0f28542420               movaps     xmm2, xmmword ptr [rsp + 0x20]
0000000141339484: 410f114208               movups     xmmword ptr [r10 + 8], xmm0
0000000141339489: 410f114a18               movups     xmmword ptr [r10 + 0x18], xmm1
000000014133948e: 410f115228               movups     xmmword ptr [r10 + 0x28], xmm2
0000000141339493: 4c8b4c2430               mov        r9, qword ptr [rsp + 0x30]
0000000141339498: 4d894a38                 mov        qword ptr [r10 + 0x38], r9
000000014133949c: 4883c040                 add        rax, 0x40
00000001413394a0: 4983c040                 add        r8, 0x40
00000001413394a4: 4839d0                   cmp        rax, rdx
00000001413394a7: 74ae                     je         0x141339457
00000001413394a9: 4c8b50c0                 mov        r10, qword ptr [rax - 0x40]
00000001413394ad: 4c8b08                   mov        r9, qword ptr [rax]
00000001413394b0: 450fb619                 movzx      r11d, byte ptr [r9]
00000001413394b4: 453a1a                   cmp        r11b, byte ptr [r10]
00000001413394b7: 73e3                     jae        0x14133949c
00000001413394b9: 4c8b5038                 mov        r10, qword ptr [rax + 0x38]
00000001413394bd: 4c89542430               mov        qword ptr [rsp + 0x30], r10
00000001413394c2: 0f104008                 movups     xmm0, xmmword ptr [rax + 8]
00000001413394c6: 0f104818                 movups     xmm1, xmmword ptr [rax + 0x18]
00000001413394ca: 0f105028                 movups     xmm2, xmmword ptr [rax + 0x28]
00000001413394ce: 0f29542420               movaps     xmmword ptr [rsp + 0x20], xmm2
00000001413394d3: 0f294c2410               movaps     xmmword ptr [rsp + 0x10], xmm1
00000001413394d8: 0f290424                 movaps     xmmword ptr [rsp], xmm0
00000001413394dc: 4d89c2                   mov        r10, r8
00000001413394df: 90                       nop        
00000001413394e0: 420f104411c0             movups     xmm0, xmmword ptr [rcx + r10 - 0x40]
00000001413394e6: 420f104c11d0             movups     xmm1, xmmword ptr [rcx + r10 - 0x30]
00000001413394ec: 420f105411e0             movups     xmm2, xmmword ptr [rcx + r10 - 0x20]
00000001413394f2: 420f105c11f0             movups     xmm3, xmmword ptr [rcx + r10 - 0x10]
00000001413394f8: 420f115c1130             movups     xmmword ptr [rcx + r10 + 0x30], xmm3
00000001413394fe: 420f11541120             movups     xmmword ptr [rcx + r10 + 0x20], xmm2
0000000141339504: 420f114c1110             movups     xmmword ptr [rcx + r10 + 0x10], xmm1
000000014133950a: 420f110411               movups     xmmword ptr [rcx + r10], xmm0
000000014133950f: 4983fa40                 cmp        r10, 0x40
0000000141339513: 0f8457ffffff             je         0x141339470
0000000141339519: 4e8b5c1180               mov        r11, qword ptr [rcx + r10 - 0x80]
000000014133951e: 410fb619                 movzx      ebx, byte ptr [r9]
0000000141339522: 4983c2c0                 add        r10, -0x40
0000000141339526: 413a1b                   cmp        bl, byte ptr [r11]
0000000141339529: 72b5                     jb         0x1413394e0
000000014133952b: 4901ca                   add        r10, rcx
000000014133952e: e940ffffff               jmp        0x141339473
0000000141339533: 0f0b                     ud2        