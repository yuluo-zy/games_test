0x140a468b0  push     rsi
0x140a468b1  sub      rsp, 0x20
0x140a468b5  mov      rsi, rcx
0x140a468b8  lea      rax, [r8 - 4]
0x140a468bc  cmp      rax, 0x1a
0x140a468c0  ja       0x140a46a4f
0x140a468c6  lea      rcx, [rip + 0x2088d53]
0x140a468cd  movsxd   rax, dword ptr [rcx + rax*4]
0x140a468d1  add      rax, rcx
0x140a468d4  jmp      rax
0x140a468d6  cmp      dword ptr [rdx], 0x6e696172
0x140a468dc  jne      0x140a46a4f
0x140a468e2  mov      byte ptr [rsi + 1], 4
0x140a468e6  jmp      0x140a46a56
0x140a468eb  movabs   rax, 0x776f6c665f797669
0x140a468f5  xor      rax, qword ptr [rdx]
0x140a468f8  movabs   rcx, 0x737265776f6c665f
0x140a46902  xor      rcx, qword ptr [rdx + 3]
0x140a46906  or       rcx, rax
0x140a46909  je       0x140a46a62
0x140a4690f  movabs   rax, 0x5f72656874616577
0x140a46919  xor      rax, qword ptr [rdx]
0x140a4691c  movabs   rcx, 0x676f665f72656874
0x140a46926  xor      rcx, qword ptr [rdx + 3]
0x140a4692a  or       rcx, rax
0x140a4692d  je       0x140a46a6e
0x140a46933  movabs   rax, 0x6c66726574747562
0x140a4693d  xor      rax, qword ptr [rdx]
0x140a46940  movabs   rcx, 0x7365696c66726574
0x140a4694a  xor      rcx, qword ptr [rdx + 3]
0x140a4694e  or       rcx, rax
0x140a46951  jne      0x140a46a4f
0x140a46957  mov      byte ptr [rsi + 1], 5
0x140a4695b  jmp      0x140a46a56
0x140a46960  lea      rax, [rip + 0x2088205]
0x140a46967  mov      rcx, rdx
0x140a4696a  mov      rdx, rax
0x140a4696d  call     0x142923bf0
0x140a46972  lea      rcx, [rsi + 1]
0x140a46976  test     eax, eax
0x140a46978  jne      0x140a46a53
0x140a4697e  mov      byte ptr [rcx], 0xa
0x140a46981  jmp      0x140a46a56
0x140a46986  movabs   rax, 0x7661656c5f797669
0x140a46990  xor      rax, qword ptr [rdx]
0x140a46993  movzx    ecx, word ptr [rdx + 8]
0x140a46997  xor      rcx, 0x7365
0x140a4699e  or       rcx, rax
0x140a469a1  jne      0x140a46a4f
0x140a469a7  mov      byte ptr [rsi + 1], 0
0x140a469ab  jmp      0x140a46a56
0x140a469b0  lea      rax, [rip + 0x208817e]
0x140a469b7  mov      rcx, rdx
0x140a469ba  mov      rdx, rax
0x140a469bd  call     0x142923bf0
0x140a469c2  test     eax, eax
0x140a469c4  jne      0x140a46a4f
0x140a469ca  mov      byte ptr [rsi + 1], 8
0x140a469ce  jmp      0x140a46a56
0x140a469d3  lea      rax, [rip + 0x2088140]
0x140a469da  mov      rcx, rdx
0x140a469dd  mov      rdx, rax
0x140a469e0  call     0x142923bf0
0x140a469e5  test     eax, eax
0x140a469e7  jne      0x140a46a4f
0x140a469e9  mov      byte ptr [rsi + 1], 7
0x140a469ed  jmp      0x140a46a56
0x140a469ef  lea      rax, [rip + 0x2088117]
0x140a469f6  mov      rcx, rdx
0x140a469f9  mov      rdx, rax
0x140a469fc  call     0x142923bf0
0x140a46a01  test     eax, eax
0x140a46a03  jne      0x140a46a4f
0x140a46a05  mov      byte ptr [rsi + 1], 6
0x140a46a09  jmp      0x140a46a56
0x140a46a0b  movdqu   xmm0, xmmword ptr [rdx]
0x140a46a0f  movdqu   xmm1, xmmword ptr [rdx + 3]
0x140a46a14  pcmpeqb  xmm1, xmmword ptr [rip + 0x2087a04]
0x140a46a1c  pcmpeqb  xmm0, xmmword ptr [rip + 0x208431c]
0x140a46a24  pand     xmm0, xmm1
0x140a46a28  pmovmskb eax, xmm0
0x140a46a2c  cmp      eax, 0xffff
0x140a46a31  jne      0x140a46a4f
0x140a46a33  mov      byte ptr [rsi + 1], 2
0x140a46a37  jmp      0x140a46a56
0x140a46a39  lea      rax, [rip + 0x208810e]
0x140a46a40  mov      rcx, rdx
0x140a46a43  mov      rdx, rax
0x140a46a46  call     0x142923bf0
0x140a46a4b  test     eax, eax
0x140a46a4d  je       0x140a46a68
0x140a46a4f  lea      rcx, [rsi + 1]
0x140a46a53  mov      byte ptr [rcx], 0xb
0x140a46a56  mov      byte ptr [rsi], 0
0x140a46a59  mov      rax, rsi
0x140a46a5c  add      rsp, 0x20
0x140a46a60  pop      rsi
0x140a46a61  ret      
0x140a46a62  mov      byte ptr [rsi + 1], 1
0x140a46a66  jmp      0x140a46a56
0x140a46a68  mov      byte ptr [rsi + 1], 9
0x140a46a6c  jmp      0x140a46a56
0x140a46a6e  mov      byte ptr [rsi + 1], 3
0x140a46a72  jmp      0x140a46a56
