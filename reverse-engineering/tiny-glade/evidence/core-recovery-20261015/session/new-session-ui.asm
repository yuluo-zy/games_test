0x141eb9690  push     rbp
0x141eb9691  push     r15
0x141eb9693  push     r14
0x141eb9695  push     r13
0x141eb9697  push     r12
0x141eb9699  push     rsi
0x141eb969a  push     rdi
0x141eb969b  push     rbx
0x141eb969c  sub      rsp, 0x408
0x141eb96a3  lea      rbp, [rsp + 0x80]
0x141eb96ab  movaps   xmmword ptr [rbp + 0x370], xmm10
0x141eb96b3  movaps   xmmword ptr [rbp + 0x360], xmm9
0x141eb96bb  movdqa   xmmword ptr [rbp + 0x350], xmm8
0x141eb96c4  movdqa   xmmword ptr [rbp + 0x340], xmm7
0x141eb96cc  movdqa   xmmword ptr [rbp + 0x330], xmm6
0x141eb96d4  mov      qword ptr [rbp + 0x328], 0xfffffffffffffffe
0x141eb96df  mov      r14, r9
0x141eb96e2  mov      r13, r8
0x141eb96e5  mov      rsi, rdx
0x141eb96e8  call     0x140c16a00
0x141eb96ed  mov      ebx, eax
0x141eb96ef  test     al, al
0x141eb96f1  je       0x141eb9750
0x141eb96f3  mov      r15d, 0x1e
0x141eb96f9  mov      ecx, 0x1e
0x141eb96fe  call     0x141eb9100
0x141eb9703  lea      r12, [rip + 0x10e0db6]
0x141eb970a  cmp      rax, 1
0x141eb970e  jne      0x141eb97bc
0x141eb9714  mov      r9, rdx
0x141eb9717  test     rdx, rdx
0x141eb971a  je       0x141eb9759
0x141eb971c  cmp      r9, 0x1e
0x141eb9720  jae      0x141eb9757
0x141eb9722  lea      rax, [rip + 0x10e0d97]
0x141eb9729  cmp      byte ptr [r9 + rax], 0xbf
0x141eb972e  jg       0x141eb9759
0x141eb9730  lea      rax, [rip + 0x10e0e31]
0x141eb9737  mov      qword ptr [rsp + 0x20], rax
0x141eb973c  lea      rcx, [rip + 0x10e0d7d]
0x141eb9743  mov      edx, 0x1e
0x141eb9748  xor      r8d, r8d
0x141eb974b  call     0x1428d9940
0x141eb9750  xor      ecx, ecx
0x141eb9752  jmp      0x141eb992b
0x141eb9757  jne      0x141eb9730
0x141eb9759  mov      rcx, r9
0x141eb975c  call     0x141eb9100
0x141eb9761  cmp      rax, 1
0x141eb9765  jne      0x141eb97bc
0x141eb9767  mov      r8, rdx
0x141eb976a  add      r8, 2
0x141eb976e  je       0x141eb97a9
0x141eb9770  cmp      r8, 0x1e
0x141eb9774  jae      0x141eb97a7
0x141eb9776  lea      rax, [rip + 0x10e0d43]
0x141eb977d  cmp      byte ptr [r8 + rax], 0xbf
0x141eb9782  jg       0x141eb97a9
0x141eb9784  lea      rax, [rip + 0x10e0df5]
0x141eb978b  mov      qword ptr [rsp + 0x20], rax
0x141eb9790  lea      rcx, [rip + 0x10e0d29]
0x141eb9797  mov      edx, 0x1e
0x141eb979c  mov      r9d, 0x1e
0x141eb97a2  call     0x1428d9940
0x141eb97a7  jne      0x141eb9784
0x141eb97a9  mov      r15d, 0x1c
0x141eb97af  sub      r15, rdx
0x141eb97b2  lea      r12, [rip + 0x10e0d07]
0x141eb97b9  add      r12, r8
0x141eb97bc  mov      qword ptr [rbp + 0x2f8], r13
0x141eb97c3  lea      r8, [rip + 0x10e0e14]
0x141eb97ca  lea      r13, [rip + 0x10e0ddf]
0x141eb97d1  nop      word ptr cs:[rax + rax]
0x141eb97e0  movsx    eax, byte ptr [r8 - 1]
0x141eb97e5  test     eax, eax
0x141eb97e7  js       0x141eb9800
0x141eb97e9  dec      r8
0x141eb97ec  cmp      eax, 0x5c
0x141eb97ef  jne      0x141eb9854
0x141eb97f1  jmp      0x141eb9865
0x141eb97f3  nop      word ptr cs:[rax + rax]
0x141eb9800  movzx    ecx, byte ptr [r8 - 2]
0x141eb9805  cmp      cl, 0xc0
0x141eb9808  jge      0x141eb982d
0x141eb980a  movzx    edx, byte ptr [r8 - 3]
0x141eb980f  cmp      dl, 0xc0
0x141eb9812  jge      0x141eb9836
0x141eb9814  movzx    r9d, byte ptr [r8 - 4]
0x141eb9819  add      r8, -4
0x141eb981d  and      r9d, 7
0x141eb9821  shl      r9d, 6
0x141eb9825  and      edx, 0x3f
0x141eb9828  or       edx, r9d
0x141eb982b  jmp      0x141eb983d
0x141eb982d  add      r8, -2
0x141eb9831  and      ecx, 0x1f
0x141eb9834  jmp      0x141eb9845
0x141eb9836  add      r8, -3
0x141eb983a  and      edx, 0xf
0x141eb983d  shl      edx, 6
0x141eb9840  and      ecx, 0x3f
0x141eb9843  or       ecx, edx
0x141eb9845  shl      ecx, 6
0x141eb9848  and      al, 0x3f
0x141eb984a  movzx    eax, al
0x141eb984d  or       eax, ecx
0x141eb984f  cmp      eax, 0x5c
0x141eb9852  je       0x141eb9865
0x141eb9854  cmp      eax, 0x2f
0x141eb9857  je       0x141eb9865
0x141eb9859  cmp      r8, r13
0x141eb985c  jne      0x141eb97e0
0x141eb985e  mov      edi, 0x2e
0x141eb9863  jmp      0x141eb98b1
0x141eb9865  lea      r13, [rip + 0x10e0d44]
0x141eb986c  sub      r8, r13
0x141eb986f  inc      r8
0x141eb9872  je       0x141eb98a6
0x141eb9874  cmp      r8, 0x2e
0x141eb9878  jae      0x141eb98a4
0x141eb987a  cmp      byte ptr [r8 + r13], 0xbf
0x141eb987f  jg       0x141eb98a6
0x141eb9881  lea      rax, [rip + 0x10e0cc0]
0x141eb9888  mov      qword ptr [rsp + 0x20], rax
0x141eb988d  lea      rcx, [rip + 0x10e0d1c]
0x141eb9894  mov      edx, 0x2e
0x141eb9899  mov      r9d, 0x2e
0x141eb989f  call     0x1428d9940
0x141eb98a4  jne      0x141eb9881
0x141eb98a6  mov      edi, 0x2e
0x141eb98ab  sub      rdi, r8
0x141eb98ae  add      r13, r8
0x141eb98b1  call     0x140c18130
0x141eb98b6  mov      rcx, rax
0x141eb98b9  mov      rax, qword ptr [rax]
0x141eb98bc  cmp      rax, 1
0x141eb98c0  jne      0x141ebb078
0x141eb98c6  add      rcx, 8
0x141eb98ca  cmp      qword ptr [rcx], 0
0x141eb98ce  jne      0x141ebc241
0x141eb98d4  mov      qword ptr [rcx], 0xffffffffffffffff
0x141eb98db  mov      qword ptr [rbp + 0x258], rcx
0x141eb98e2  add      rcx, 8
0x141eb98e6  mov      qword ptr [rsp + 0x20], rdi
0x141eb98eb  mov      qword ptr [rsp + 0x30], 0
0x141eb98f4  mov      qword ptr [rsp + 0x28], 1
0x141eb98fd  mov      rdx, r12
0x141eb9900  mov      r8, r15
0x141eb9903  mov      r9, r13
0x141eb9906  call     0x140c16d40
0x141eb990b  mov      r15, rax
0x141eb990e  mov      rax, qword ptr [rbp + 0x258]
0x141eb9915  inc      qword ptr [rax]
0x141eb9918  mov      qword ptr [rbp + 0xb0], r15
0x141eb991f  mov      ecx, 1
0x141eb9924  mov      r13, qword ptr [rbp + 0x2f8]
0x141eb992b  mov      rax, qword ptr [rbp + 0x428]
0x141eb9932  mov      qword ptr [rbp + 0xa8], rcx
0x141eb9939  mov      rcx, qword ptr [r14]
0x141eb993c  cmp      dword ptr [rcx], 5
0x141eb993f  jne      0x141eb9d4e
0x141eb9945  mov      rcx, qword ptr [rbp + 0x408]
0x141eb994c  mov      edx, dword ptr [rax + 0x1c]
0x141eb994f  mov      r8, qword ptr [rax + 0x10]
0x141eb9953  mov      dword ptr [rbp + 0x2a8], edx
0x141eb9959  mov      qword ptr [rbp + 0x1e8], r8
0x141eb9960  mov      dword ptr [r8], edx
0x141eb9963  mov      rax, qword ptr [rax]
0x141eb9966  mov      qword ptr [rbp + 0x2a0], rax
0x141eb996d  mov      byte ptr [rax + 0x1cc], 0
0x141eb9974  mov      rax, qword ptr [rsi]
0x141eb9977  mov      rax, qword ptr [rax + 0x10]
0x141eb997b  mov      qword ptr [rbp + 0x2b0], rax
0x141eb9982  mov      byte ptr [rbp + 0x2f7], 0
0x141eb9989  mov      eax, dword ptr [rcx + 0x1c]
0x141eb998c  mov      rdx, qword ptr [rcx + 0x10]
0x141eb9990  mov      dword ptr [rbp + 0x2f8], eax
0x141eb9996  mov      qword ptr [rbp + 0x258], rdx
0x141eb999d  mov      dword ptr [rdx], eax
0x141eb999f  mov      rsi, qword ptr [rcx]
0x141eb99a2  lea      rax, [rip + 0x10e0c35]
0x141eb99a9  mov      qword ptr [rbp + 0x110], rax
0x141eb99b0  mov      qword ptr [rbp + 0x118], 4
0x141eb99bb  lea      rcx, [rbp + 0x110]
0x141eb99c2  call     0x1405bdda0
0x141eb99c7  mov      rbx, rax
0x141eb99ca  movabs   rax, 0xb1695320004fb443
0x141eb99d4  xor      rbx, rax
0x141eb99d7  mov      rcx, rsi
0x141eb99da  mov      rdx, rbx
0x141eb99dd  call     0x140683ec0
0x141eb99e2  mov      byte ptr [rax + 0x5d], 0
0x141eb99e6  test     dl, 1
0x141eb99e9  jne      0x141eb9a1e
0x141eb99eb  mov      rdi, qword ptr [rsi + 0x10]
0x141eb99ef  cmp      rdi, qword ptr [rsi]
0x141eb99f2  jne      0x141eb9a03
0x141eb99f4  lea      rdx, [rip + 0xcf55cd]
0x141eb99fb  mov      rcx, rsi
0x141eb99fe  call     0x1406319d0
0x141eb9a03  mov      rax, qword ptr [rsi + 8]
0x141eb9a07  imul     rcx, rdi, 0x38
0x141eb9a0b  mov      dword ptr [rax + rcx], 2
0x141eb9a12  mov      qword ptr [rax + rcx + 0x30], rbx
0x141eb9a17  inc      rdi
0x141eb9a1a  mov      qword ptr [rsi + 0x10], rdi
0x141eb9a1e  mov      qword ptr [rbp + 0x1f0], rsi
0x141eb9a25  mov      qword ptr [rbp + 0x1f8], rbx
0x141eb9a2c  mov      qword ptr [rbp + 0x200], 0
0x141eb9a37  mov      qword ptr [rsp + 0x20], 0
0x141eb9a40  lea      r8, [rip + 0x10e0b9b]
0x141eb9a47  lea      rcx, [rbp + 0x110]
0x141eb9a4e  lea      rdx, [rbp + 0x1f0]
0x141eb9a55  mov      r9d, 9
0x141eb9a5b  call     0x140f9e970
0x141eb9a60  mov      rax, qword ptr [rbp + 0x400]
0x141eb9a67  mov      qword ptr [rsp + 0x20], rax
0x141eb9a6c  lea      rdx, [rip + 0x10e0b78]
0x141eb9a73  lea      rcx, [rbp + 0x110]
0x141eb9a7a  lea      r9, [rbp + 0x2f7]
0x141eb9a81  mov      r8d, 6
0x141eb9a87  call     0x140fc20f0
0x141eb9a8c  mov      rax, qword ptr [rbp + 0x420]
0x141eb9a93  mov      r12, qword ptr [rbp + 0x3f0]
0x141eb9a9a  cmp      byte ptr [rax], 0
0x141eb9a9d  je       0x141eb9de6
0x141eb9aa3  mov      eax, dword ptr [rbp + 0x2f8]
0x141eb9aa9  mov      rcx, qword ptr [rbp + 0x258]
0x141eb9ab0  mov      dword ptr [rcx], eax
0x141eb9ab2  lea      rax, [rip + 0x10e0df1]
0x141eb9ab9  mov      qword ptr [rbp + 0x110], rax
0x141eb9ac0  mov      qword ptr [rbp + 0x118], 0x11
0x141eb9acb  lea      rcx, [rbp + 0x110]
0x141eb9ad2  call     0x1405bdda0
0x141eb9ad7  mov      rdi, rax
0x141eb9ada  movabs   rax, 0xb1695320004fb443
0x141eb9ae4  xor      rdi, rax
0x141eb9ae7  mov      rcx, rsi
0x141eb9aea  mov      rdx, rdi
0x141eb9aed  call     0x140683ec0
0x141eb9af2  mov      byte ptr [rax + 0x5d], 0
0x141eb9af6  mov      byte ptr [rax + 0xb7], 1
0x141eb9afd  test     dl, 1
0x141eb9b00  jne      0x141eb9b35
0x141eb9b02  mov      rbx, qword ptr [rsi + 0x10]
0x141eb9b06  cmp      rbx, qword ptr [rsi]
0x141eb9b09  jne      0x141eb9b1a
0x141eb9b0b  lea      rdx, [rip + 0xcf54b6]
0x141eb9b12  mov      rcx, rsi
0x141eb9b15  call     0x1406319d0
0x141eb9b1a  mov      rax, qword ptr [rsi + 8]
0x141eb9b1e  imul     rcx, rbx, 0x38
0x141eb9b22  mov      dword ptr [rax + rcx], 1
0x141eb9b29  mov      qword ptr [rax + rcx + 0x30], rdi
0x141eb9b2e  inc      rbx
0x141eb9b31  mov      qword ptr [rsi + 0x10], rbx
0x141eb9b35  mov      qword ptr [rbp + 0x2d0], rsi
0x141eb9b3c  mov      qword ptr [rbp + 0x2d8], rdi
0x141eb9b43  mov      qword ptr [rbp + 0x2e0], 0
0x141eb9b4e  mov      qword ptr [rbp + 0x110], 0
0x141eb9b59  movabs   rbx, 0x41223169ff28813b
0x141eb9b63  mov      qword ptr [rbp + 0x118], rbx
0x141eb9b6a  movabs   r14, 0xa79b7268a2a968d9
0x141eb9b74  mov      qword ptr [rbp + 0x120], r14
0x141eb9b7b  lea      rcx, [rbp + 0x110]
0x141eb9b82  call     0x1405bdf20
0x141eb9b87  mov      qword ptr [rbp + 0x110], rdi
0x141eb9b8e  mov      qword ptr [rbp + 0x118], rax
0x141eb9b95  lea      rcx, [rbp + 0x110]
0x141eb9b9c  call     0x1405bd910
0x141eb9ba1  lea      rcx, [rbp + 0x270]
0x141eb9ba8  lea      rdx, [rbp + 0x2d0]
0x141eb9baf  mov      r8, rax
0x141eb9bb2  call     0x1406861b0
0x141eb9bb7  mov      rdi, qword ptr [rbp + 0x278]
0x141eb9bbe  mov      qword ptr [rbp + 0x110], 0
0x141eb9bc9  mov      qword ptr [rbp + 0x118], rbx
0x141eb9bd0  mov      qword ptr [rbp + 0x120], r14
0x141eb9bd7  lea      rcx, [rbp + 0x110]
0x141eb9bde  call     0x1405bdf20
0x141eb9be3  mov      qword ptr [rbp + 0x110], rdi
0x141eb9bea  mov      qword ptr [rbp + 0x118], rax
0x141eb9bf1  lea      rcx, [rbp + 0x110]
0x141eb9bf8  call     0x1405bd910
0x141eb9bfd  lea      rcx, [rbp + 0x110]
0x141eb9c04  lea      rdx, [rbp + 0x270]
0x141eb9c0b  mov      r8, rax
0x141eb9c0e  call     0x1406861b0
0x141eb9c13  mov      qword ptr [rsp + 0x20], 0
0x141eb9c1c  lea      r8, [rip + 0x10e0c98]
0x141eb9c23  lea      rcx, [rbp - 0x40]
0x141eb9c27  lea      rdx, [rbp + 0x110]
0x141eb9c2e  mov      r9d, 0x16
0x141eb9c34  call     0x140f9e970
0x141eb9c39  lea      r8, [rip + 0x10e0c91]
0x141eb9c40  lea      rcx, [rbp + 0x1f0]
0x141eb9c47  lea      rdx, [rbp - 0x40]
0x141eb9c4b  mov      r9d, 6
0x141eb9c51  call     0x140fb8070
0x141eb9c56  lea      rcx, [rip + 0x10e0c7a]
0x141eb9c5d  mov      edx, 0xd
0x141eb9c62  call     0x140b9d480
0x141eb9c67  mov      r9, rdx
0x141eb9c6a  lea      rcx, [rbp + 0x110]
0x141eb9c71  lea      rdx, [rbp + 0x1f0]
0x141eb9c78  mov      r8, rax
0x141eb9c7b  call     0x14068bfc0
0x141eb9c80  mov      rdi, qword ptr [rbp + 0x110]
0x141eb9c87  cmp      qword ptr [rdi + 0x1b8], 0
0x141eb9c8f  je       0x141eba1e3
0x141eb9c95  lea      rdx, [rbp + 0x118]
0x141eb9c9c  lea      rcx, [rdi + 0x1c0]
0x141eb9ca3  call     0x1405b0d70
0x141eb9ca8  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eb9caf  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eb9cb6  mov      r8, rdx
0x141eb9cb9  and      r8, rax
0x141eb9cbc  shr      rax, 0x39
0x141eb9cc0  movd     xmm0, eax
0x141eb9cc4  punpcklbw xmm0, xmm0
0x141eb9cc8  pshuflw  xmm0, xmm0, 0
0x141eb9ccd  pshufd   xmm0, xmm0, 0
0x141eb9cd2  lea      rax, [rcx - 0xc0]
0x141eb9cd9  mov      r9, qword ptr [rbp + 0x118]
0x141eb9ce0  xor      r10d, r10d
0x141eb9ce3  pcmpeqd  xmm1, xmm1
0x141eb9ce7  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141eb9ced  movdqa   xmm3, xmm2
0x141eb9cf1  pcmpeqb  xmm3, xmm0
0x141eb9cf5  pmovmskb r11d, xmm3
0x141eb9cfa  test     r11d, r11d
0x141eb9cfd  je       0x141eb9d2c
0x141eb9cff  tzcnt    edi, r11d
0x141eb9d04  add      rdi, r8
0x141eb9d07  and      rdi, rdx
0x141eb9d0a  neg      rdi
0x141eb9d0d  lea      rdi, [rdi + rdi*2]
0x141eb9d11  shl      rdi, 6
0x141eb9d15  cmp      r9, qword ptr [rax + rdi]
0x141eb9d19  je       0x141eba0b0
0x141eb9d1f  lea      edi, [r11 - 1]
0x141eb9d23  and      di, r11w
0x141eb9d27  mov      r11d, edi
0x141eb9d2a  jne      0x141eb9cff
0x141eb9d2c  pcmpeqb  xmm2, xmm1
0x141eb9d30  pmovmskb r11d, xmm2
0x141eb9d35  test     r11d, r11d
0x141eb9d38  jne      0x141eba1e3
0x141eb9d3e  add      r8, r10
0x141eb9d41  add      r8, 0x10
0x141eb9d45  add      r10, 0x10
0x141eb9d49  and      r8, rdx
0x141eb9d4c  jmp      0x141eb9ce7
0x141eb9d4e  mov      ecx, dword ptr [rax + 0x1c]
0x141eb9d51  mov      rdx, qword ptr [rax + 0x10]
0x141eb9d55  mov      dword ptr [rdx], ecx
0x141eb9d57  mov      rax, qword ptr [rax]
0x141eb9d5a  mov      byte ptr [rax + 0x1cc], 5
0x141eb9d61  test     bl, bl
0x141eb9d63  je       0x141eb9dac
0x141eb9d65  call     0x140c18130
0x141eb9d6a  mov      rcx, rax
0x141eb9d6d  mov      rax, qword ptr [rax]
0x141eb9d70  cmp      rax, 1
0x141eb9d74  jne      0x141ebb0b6
0x141eb9d7a  add      rcx, 8
0x141eb9d7e  cmp      qword ptr [rcx], 0
0x141eb9d82  jne      0x141ebc241
0x141eb9d88  mov      qword ptr [rcx], 0xffffffffffffffff
0x141eb9d8f  mov      qword ptr [rbp + 0x2f8], rcx
0x141eb9d96  add      rcx, 8
0x141eb9d9a  mov      rdx, r15
0x141eb9d9d  call     0x140c170f0
0x141eb9da2  mov      rax, qword ptr [rbp + 0x2f8]
0x141eb9da9  inc      qword ptr [rax]
0x141eb9dac  movaps   xmm6, xmmword ptr [rbp + 0x330]
0x141eb9db3  movaps   xmm7, xmmword ptr [rbp + 0x340]
0x141eb9dba  movaps   xmm8, xmmword ptr [rbp + 0x350]
0x141eb9dc2  movaps   xmm9, xmmword ptr [rbp + 0x360]
0x141eb9dca  movaps   xmm10, xmmword ptr [rbp + 0x370]
0x141eb9dd2  add      rsp, 0x408
0x141eb9dd9  pop      rbx
0x141eb9dda  pop      rdi
0x141eb9ddb  pop      rsi
0x141eb9ddc  pop      r12
0x141eb9dde  pop      r13
0x141eb9de0  pop      r14
0x141eb9de2  pop      r15
0x141eb9de4  pop      rbp
0x141eb9de5  ret      
0x141eb9de6  mov      eax, dword ptr [rbp + 0x2f8]
0x141eb9dec  mov      rcx, qword ptr [rbp + 0x258]
0x141eb9df3  mov      dword ptr [rcx], eax
0x141eb9df5  lea      rax, [rip + 0x10e07f5]
0x141eb9dfc  mov      qword ptr [rbp + 0x1f0], rax
0x141eb9e03  mov      qword ptr [rbp + 0x1f8], 0xb
0x141eb9e0e  lea      rcx, [rbp + 0x1f0]
0x141eb9e15  call     0x1405bdda0
0x141eb9e1a  mov      r14, rax
0x141eb9e1d  movabs   rax, 0xb1695320004fb443
0x141eb9e27  xor      r14, rax
0x141eb9e2a  mov      rcx, rsi
0x141eb9e2d  mov      rdx, r14
0x141eb9e30  call     0x140683ec0
0x141eb9e35  mov      byte ptr [rax + 0x5d], 0
0x141eb9e39  mov      byte ptr [rax + 0xb7], 1
0x141eb9e40  test     dl, 1
0x141eb9e43  jne      0x141eb9e78
0x141eb9e45  mov      rdi, qword ptr [rsi + 0x10]
0x141eb9e49  cmp      rdi, qword ptr [rsi]
0x141eb9e4c  jne      0x141eb9e5d
0x141eb9e4e  lea      rdx, [rip + 0xcf5173]
0x141eb9e55  mov      rcx, rsi
0x141eb9e58  call     0x1406319d0
0x141eb9e5d  mov      rax, qword ptr [rsi + 8]
0x141eb9e61  imul     rcx, rdi, 0x38
0x141eb9e65  mov      dword ptr [rax + rcx], 1
0x141eb9e6c  mov      qword ptr [rax + rcx + 0x30], r14
0x141eb9e71  inc      rdi
0x141eb9e74  mov      qword ptr [rsi + 0x10], rdi
0x141eb9e78  mov      qword ptr [rbp + 0x110], rsi
0x141eb9e7f  mov      qword ptr [rbp + 0x118], r14
0x141eb9e86  mov      qword ptr [rbp + 0x120], 0
0x141eb9e91  mov      qword ptr [rsp + 0x20], 0
0x141eb9e9a  lea      r8, [rip + 0x10e075b]
0x141eb9ea1  lea      rcx, [rbp + 0x270]
0x141eb9ea8  lea      rdx, [rbp + 0x110]
0x141eb9eaf  mov      r9d, 0x10
0x141eb9eb5  call     0x140f9e970
0x141eb9eba  lea      rdx, [rip + 0x10e074b]
0x141eb9ec1  lea      rcx, [rbp + 0x270]
0x141eb9ec8  mov      r8d, 6
0x141eb9ece  call     0x140fc2db0
0x141eb9ed3  lea      rdx, [rip + 0x10e0704]
0x141eb9eda  lea      rcx, [rbp + 0x270]
0x141eb9ee1  mov      r8d, 4
0x141eb9ee7  mov      r9, qword ptr [rbp + 0x400]
0x141eb9eee  call     0x140fc0d00
0x141eb9ef3  mov      rax, qword ptr [r13]
0x141eb9ef7  movss    xmm0, dword ptr [rax + 0x58]
0x141eb9efc  mov      rax, qword ptr [rbp + 0x3f8]
0x141eb9f03  addss    xmm0, dword ptr [rax]
0x141eb9f07  movss    dword ptr [rax], xmm0
0x141eb9f0b  lea      rdx, [rip + 0x10e0700]
0x141eb9f12  lea      rcx, [rbp + 0x270]
0x141eb9f19  mov      r8d, 9
0x141eb9f1f  call     0x140fb9310
0x141eb9f24  lea      r8, [rip + 0x10e06f0]
0x141eb9f2b  lea      rcx, [rbp + 0x80]
0x141eb9f32  lea      rdx, [rbp + 0x270]
0x141eb9f39  mov      r9d, 4
0x141eb9f3f  call     0x140fb8340
0x141eb9f44  mov      rdi, qword ptr [rbp + 0x80]
0x141eb9f4b  cmp      qword ptr [rdi + 0x1b8], 0
0x141eb9f53  je       0x141eba1d2
0x141eb9f59  lea      rdx, [rbp + 0x88]
0x141eb9f60  lea      rcx, [rdi + 0x1c0]
0x141eb9f67  call     0x1405b0d70
0x141eb9f6c  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eb9f73  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eb9f7a  mov      r8, rdx
0x141eb9f7d  and      r8, rax
0x141eb9f80  shr      rax, 0x39
0x141eb9f84  movd     xmm0, eax
0x141eb9f88  punpcklbw xmm0, xmm0
0x141eb9f8c  pshuflw  xmm0, xmm0, 0
0x141eb9f91  pshufd   xmm0, xmm0, 0
0x141eb9f96  lea      rax, [rcx - 0xc0]
0x141eb9f9d  mov      r9, qword ptr [rbp + 0x88]
0x141eb9fa4  xor      r10d, r10d
0x141eb9fa7  pcmpeqd  xmm1, xmm1
0x141eb9fab  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141eb9fb1  movdqa   xmm3, xmm2
0x141eb9fb5  pcmpeqb  xmm3, xmm0
0x141eb9fb9  pmovmskb r11d, xmm3
0x141eb9fbe  test     r11d, r11d
0x141eb9fc1  je       0x141eb9fec
0x141eb9fc3  tzcnt    edi, r11d
0x141eb9fc8  add      rdi, r8
0x141eb9fcb  and      rdi, rdx
0x141eb9fce  neg      rdi
0x141eb9fd1  lea      rdi, [rdi + rdi*2]
0x141eb9fd5  shl      rdi, 6
0x141eb9fd9  cmp      r9, qword ptr [rax + rdi]
0x141eb9fdd  je       0x141eba00e
0x141eb9fdf  lea      edi, [r11 - 1]
0x141eb9fe3  and      di, r11w
0x141eb9fe7  mov      r11d, edi
0x141eb9fea  jne      0x141eb9fc3
0x141eb9fec  pcmpeqb  xmm2, xmm1
0x141eb9ff0  pmovmskb r11d, xmm2
0x141eb9ff5  test     r11d, r11d
0x141eb9ff8  jne      0x141eba1d2
0x141eb9ffe  add      r8, r10
0x141eba001  add      r8, 0x10
0x141eba005  add      r10, 0x10
0x141eba009  and      r8, rdx
0x141eba00c  jmp      0x141eb9fab
0x141eba00e  movzx    eax, byte ptr [rcx + rdi - 2]
0x141eba013  mov      dword ptr [rbp + 0x2cc], eax
0x141eba019  lea      rcx, [rip + 0x10e05ff]
0x141eba020  mov      edx, 0xf
0x141eba025  call     0x140b9d480
0x141eba02a  mov      r15, rax
0x141eba02d  mov      rdi, rdx
0x141eba030  mov      rcx, qword ptr [r12]
0x141eba034  xor      eax, eax
0x141eba036  cmp      rax, qword ptr [rcx]
0x141eba039  mov      qword ptr [rbp + 0x78], rcx
0x141eba03d  jo       0x141eba05f
0x141eba03f  call     0x14083c440
0x141eba044  test     al, al
0x141eba046  je       0x141eba05f
0x141eba048  lea      rcx, [rip + 0x10e05df]
0x141eba04f  mov      edx, 0x10
0x141eba054  call     0x140b9d480
0x141eba059  mov      r15, rax
0x141eba05c  mov      rdi, rdx
0x141eba05f  lea      rcx, [rip + 0x10e05d8]
0x141eba066  mov      edx, 0x10
0x141eba06b  call     0x140b9d480
0x141eba070  mov      rbx, rdx
0x141eba073  test     rdx, rdx
0x141eba076  js       0x141ebc7ef
0x141eba07c  mov      r12, rax
0x141eba07f  test     rbx, rbx
0x141eba082  je       0x141eba200
0x141eba088  movzx    eax, byte ptr [rip + 0x177ab7a]
0x141eba08f  mov      r13d, 1
0x141eba095  mov      edx, 1
0x141eba09a  mov      rcx, rbx
0x141eba09d  call     0x140613c10
0x141eba0a2  test     rax, rax
0x141eba0a5  jne      0x141eba205
0x141eba0ab  jmp      0x141ebc7f2
0x141eba0b0  test     byte ptr [rcx + rdi - 2], 0x30
0x141eba0b5  je       0x141eba0c1
0x141eba0b7  mov      rax, qword ptr [rbp + 0x420]
0x141eba0be  mov      byte ptr [rax], 0
0x141eba0c1  lea      rcx, [rip + 0x10e081c]
0x141eba0c8  mov      edx, 0xc
0x141eba0cd  call     0x140b9d480
0x141eba0d2  mov      r9, rdx
0x141eba0d5  lea      rcx, [rbp + 0x110]
0x141eba0dc  lea      rdx, [rbp + 0x1f0]
0x141eba0e3  mov      r8, rax
0x141eba0e6  call     0x14068bfc0
0x141eba0eb  mov      rdi, qword ptr [rbp + 0x110]
0x141eba0f2  cmp      qword ptr [rdi + 0x1b8], 0
0x141eba0fa  je       0x141ebb001
0x141eba100  lea      rdx, [rbp + 0x118]
0x141eba107  lea      rcx, [rdi + 0x1c0]
0x141eba10e  call     0x1405b0d70
0x141eba113  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eba11a  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eba121  mov      r8, rdx
0x141eba124  and      r8, rax
0x141eba127  shr      rax, 0x39
0x141eba12b  movd     xmm0, eax
0x141eba12f  punpcklbw xmm0, xmm0
0x141eba133  pshuflw  xmm0, xmm0, 0
0x141eba138  pshufd   xmm0, xmm0, 0
0x141eba13d  lea      rax, [rcx - 0xc0]
0x141eba144  mov      r9, qword ptr [rbp + 0x118]
0x141eba14b  xor      r10d, r10d
0x141eba14e  pcmpeqd  xmm1, xmm1
0x141eba152  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141eba158  movdqa   xmm3, xmm2
0x141eba15c  pcmpeqb  xmm3, xmm0
0x141eba160  pmovmskb r11d, xmm3
0x141eba165  test     r11d, r11d
0x141eba168  je       0x141eba193
0x141eba16a  tzcnt    edi, r11d
0x141eba16f  add      rdi, r8
0x141eba172  and      rdi, rdx
0x141eba175  neg      rdi
0x141eba178  lea      rdi, [rdi + rdi*2]
0x141eba17c  shl      rdi, 6
0x141eba180  cmp      r9, qword ptr [rax + rdi]
0x141eba184  je       0x141eba1b5
0x141eba186  lea      edi, [r11 - 1]
0x141eba18a  and      di, r11w
0x141eba18e  mov      r11d, edi
0x141eba191  jne      0x141eba16a
0x141eba193  pcmpeqb  xmm2, xmm1
0x141eba197  pmovmskb r11d, xmm2
0x141eba19c  test     r11d, r11d
0x141eba19f  jne      0x141ebb001
0x141eba1a5  add      r8, r10
0x141eba1a8  add      r8, 0x10
0x141eba1ac  add      r10, 0x10
0x141eba1b0  and      r8, rdx
0x141eba1b3  jmp      0x141eba152
0x141eba1b5  test     byte ptr [rcx + rdi - 2], 0x30
0x141eba1ba  je       0x141ebb01e
0x141eba1c0  mov      rax, qword ptr [rbp + 0x420]
0x141eba1c7  mov      byte ptr [rax], 0
0x141eba1ca  xor      r14d, r14d
0x141eba1cd  jmp      0x141ebb021
0x141eba1d2  lea      rcx, [rip + 0xb5da47]
0x141eba1d9  call     0x1428d9310
0x141eba1de  jmp      0x141ebc902
0x141eba1e3  lea      rcx, [rip + 0xb5d01e]
0x141eba1ea  lea      r8, [rip + 0xb5dde7]
0x141eba1f1  mov      edx, 0x16
0x141eba1f6  call     0x1428d9330
0x141eba1fb  jmp      0x141ebc902
0x141eba200  mov      eax, 1
0x141eba205  mov      qword ptr [rbp + 0x260], rax
0x141eba20c  mov      rcx, rax
0x141eba20f  mov      rdx, r12
0x141eba212  mov      qword ptr [rbp + 0x2b8], rbx
0x141eba219  mov      r8, rbx
0x141eba21c  call     0x142923c00
0x141eba221  lea      rcx, [rip + 0x10e0426]
0x141eba228  mov      edx, 0x10
0x141eba22d  call     0x140b9d480
0x141eba232  mov      rbx, rdx
0x141eba235  test     rdx, rdx
0x141eba238  js       0x141ebc809
0x141eba23e  mov      r12, rax
0x141eba241  test     rbx, rbx
0x141eba244  je       0x141eba26a
0x141eba246  movzx    eax, byte ptr [rip + 0x177a9bc]
0x141eba24d  mov      r13d, 1
0x141eba253  mov      edx, 1
0x141eba258  mov      rcx, rbx
0x141eba25b  call     0x140613c10
0x141eba260  test     rax, rax
0x141eba263  jne      0x141eba26f
0x141eba265  jmp      0x141ebc80c
0x141eba26a  mov      eax, 1
0x141eba26f  mov      qword ptr [rbp + 0x268], rax
0x141eba276  mov      rcx, rax
0x141eba279  mov      rdx, r12
0x141eba27c  mov      qword ptr [rbp + 0x2c0], rbx
0x141eba283  mov      r8, rbx
0x141eba286  call     0x142923c00
0x141eba28b  lea      rcx, [rip + 0x10e03cc]
0x141eba292  mov      edx, 0xf
0x141eba297  call     0x140b9d480
0x141eba29c  mov      rbx, rdx
0x141eba29f  test     rdx, rdx
0x141eba2a2  js       0x141ebc823
0x141eba2a8  mov      r12, rax
0x141eba2ab  test     rbx, rbx
0x141eba2ae  je       0x141eba2d4
0x141eba2b0  movzx    eax, byte ptr [rip + 0x177a952]
0x141eba2b7  mov      r13d, 1
0x141eba2bd  mov      edx, 1
0x141eba2c2  mov      rcx, rbx
0x141eba2c5  call     0x140613c10
0x141eba2ca  test     rax, rax
0x141eba2cd  jne      0x141eba2d9
0x141eba2cf  jmp      0x141ebc826
0x141eba2d4  mov      eax, 1
0x141eba2d9  mov      qword ptr [rbp + 0xb8], rax
0x141eba2e0  mov      rcx, rax
0x141eba2e3  mov      rdx, r12
0x141eba2e6  mov      qword ptr [rbp + 0x298], rbx
0x141eba2ed  mov      r8, rbx
0x141eba2f0  call     0x142923c00
0x141eba2f5  lea      rcx, [rip + 0x10e0371]
0x141eba2fc  mov      edx, 0x11
0x141eba301  call     0x140b9d480
0x141eba306  mov      rbx, rdx
0x141eba309  test     rdx, rdx
0x141eba30c  js       0x141ebc83d
0x141eba312  mov      r12, rax
0x141eba315  test     rbx, rbx
0x141eba318  je       0x141eba33e
0x141eba31a  movzx    eax, byte ptr [rip + 0x177a8e8]
0x141eba321  mov      r13d, 1
0x141eba327  mov      edx, 1
0x141eba32c  mov      rcx, rbx
0x141eba32f  call     0x140613c10
0x141eba334  test     rax, rax
0x141eba337  jne      0x141eba343
0x141eba339  jmp      0x141ebc840
0x141eba33e  mov      eax, 1
0x141eba343  mov      qword ptr [rbp + 0xc0], rax
0x141eba34a  mov      rcx, rax
0x141eba34d  mov      rdx, r12
0x141eba350  mov      qword ptr [rbp + 0xd0], rbx
0x141eba357  mov      r8, rbx
0x141eba35a  call     0x142923c00
0x141eba35f  lea      rcx, [rip + 0x10e0318]
0x141eba366  mov      edx, 0x10
0x141eba36b  call     0x140b9d480
0x141eba370  mov      r14, rdx
0x141eba373  test     rdx, rdx
0x141eba376  js       0x141ebc857
0x141eba37c  mov      r12, rax
0x141eba37f  test     r14, r14
0x141eba382  mov      rbx, qword ptr [rbp + 0x2b8]
0x141eba389  je       0x141eba3af
0x141eba38b  movzx    eax, byte ptr [rip + 0x177a877]
0x141eba392  mov      r13d, 1
0x141eba398  mov      edx, 1
0x141eba39d  mov      rcx, r14
0x141eba3a0  call     0x140613c10
0x141eba3a5  test     rax, rax
0x141eba3a8  jne      0x141eba3b4
0x141eba3aa  jmp      0x141ebc85a
0x141eba3af  mov      eax, 1
0x141eba3b4  mov      qword ptr [rbp + 0xc8], rax
0x141eba3bb  mov      rcx, rax
0x141eba3be  mov      rdx, r12
0x141eba3c1  mov      r8, r14
0x141eba3c4  call     0x142923c00
0x141eba3c9  test     rdi, rdi
0x141eba3cc  js       0x141ebc871
0x141eba3d2  je       0x141eba3fc
0x141eba3d4  movzx    eax, byte ptr [rip + 0x177a82e]
0x141eba3db  mov      r13d, 1
0x141eba3e1  mov      edx, 1
0x141eba3e6  mov      rcx, rdi
0x141eba3e9  call     0x140613c10
0x141eba3ee  test     rax, rax
0x141eba3f1  je       0x141ebc874
0x141eba3f7  mov      r12, rax
0x141eba3fa  jmp      0x141eba402
0x141eba3fc  mov      r12d, 1
0x141eba402  mov      rcx, r12
0x141eba405  mov      rdx, r15
0x141eba408  mov      r8, rdi
0x141eba40b  call     0x142923c00
0x141eba410  mov      word ptr [rbp + 0x110], 1
0x141eba419  mov      qword ptr [rbp + 0x118], rbx
0x141eba420  mov      rax, qword ptr [rbp + 0x260]
0x141eba427  mov      qword ptr [rbp + 0x120], rax
0x141eba42e  mov      qword ptr [rbp + 0x128], rbx
0x141eba435  mov      word ptr [rbp + 0x130], 0x100
0x141eba43e  mov      rax, qword ptr [rbp + 0x2c0]
0x141eba445  mov      qword ptr [rbp + 0x138], rax
0x141eba44c  mov      rcx, qword ptr [rbp + 0x268]
0x141eba453  mov      qword ptr [rbp + 0x140], rcx
0x141eba45a  mov      qword ptr [rbp + 0x148], rax
0x141eba461  mov      word ptr [rbp + 0x150], 0x300
0x141eba46a  mov      rax, qword ptr [rbp + 0x298]
0x141eba471  mov      qword ptr [rbp + 0x158], rax
0x141eba478  mov      rcx, qword ptr [rbp + 0xb8]
0x141eba47f  mov      qword ptr [rbp + 0x160], rcx
0x141eba486  mov      qword ptr [rbp + 0x168], rax
0x141eba48d  mov      word ptr [rbp + 0x170], 0x200
0x141eba496  mov      rax, qword ptr [rbp + 0xd0]
0x141eba49d  mov      qword ptr [rbp + 0x178], rax
0x141eba4a4  mov      rcx, qword ptr [rbp + 0xc0]
0x141eba4ab  mov      qword ptr [rbp + 0x180], rcx
0x141eba4b2  mov      qword ptr [rbp + 0x188], rax
0x141eba4b9  mov      word ptr [rbp + 0x190], 0x400
0x141eba4c2  mov      qword ptr [rbp + 0x198], r14
0x141eba4c9  mov      rax, qword ptr [rbp + 0xc8]
0x141eba4d0  mov      qword ptr [rbp + 0x1a0], rax
0x141eba4d7  mov      qword ptr [rbp + 0x1a8], r14
0x141eba4de  mov      word ptr [rbp + 0x1b0], 0x500
0x141eba4e7  mov      qword ptr [rbp + 0x1b8], rdi
0x141eba4ee  mov      qword ptr [rbp + 0x1c0], r12
0x141eba4f5  mov      qword ptr [rbp + 0x1c8], rdi
0x141eba4fc  mov      qword ptr [rbp + 0x1d8], 6
0x141eba507  mov      qword ptr [rbp + 0x1e0], 0
0x141eba512  mov      r14b, 6
0x141eba515  mov      edi, 6
0x141eba51a  xor      eax, eax
0x141eba51c  lea      r13, [rbp + 0x1f0]
0x141eba523  movd     xmm6, dword ptr [rip + 0xb173a1]
0x141eba52b  movd     xmm7, dword ptr [rip + 0xcf8c61]
0x141eba533  pcmpeqd  xmm8, xmm8
0x141eba538  movaps   xmm9, xmmword ptr [rip + 0xb12b80]
0x141eba540  movsd    xmm10, qword ptr [rip + 0xb5c8e7]
0x141eba549  cmp      rdi, rax
0x141eba54c  je       0x141ebae87
0x141eba552  lea      rcx, [rax + 1]
0x141eba556  mov      qword ptr [rbp + 0x1d0], rcx
0x141eba55d  shl      rax, 5
0x141eba561  mov      r8, qword ptr [rbp + rax + 0x118]
0x141eba569  mov      rdx, r8
0x141eba56c  neg      rdx
0x141eba56f  jo       0x141ebae8a
0x141eba575  mov      qword ptr [rbp + 0x2c0], r8
0x141eba57c  mov      dword ptr [rbp + 0x260], r14d
0x141eba583  movzx    r12d, byte ptr [rbp + rax + 0x111]
0x141eba58c  mov      rcx, qword ptr [rbp + rax + 0x120]
0x141eba594  mov      qword ptr [rbp + 0x2b8], rcx
0x141eba59b  mov      rdi, qword ptr [rbp + rax + 0x128]
0x141eba5a3  mov      rbx, qword ptr [rbp + 0x1e0]
0x141eba5aa  lea      rax, [rbx + 1]
0x141eba5ae  mov      qword ptr [rbp + 0x1e0], rax
0x141eba5b5  cmp      r12b, 5
0x141eba5b9  jne      0x141eba682
0x141eba5bf  mov      r14, qword ptr [rbp + 0x88]
0x141eba5c6  movabs   rax, 0xc1ea527c35d
0x141eba5d0  mov      qword ptr [rbp + 0x1f0], rax
0x141eba5d7  movabs   rax, 0x41223169ff28813b
0x141eba5e1  mov      qword ptr [rbp + 0x1f8], rax
0x141eba5e8  movabs   rax, 0xa79b7268a2a968d9
0x141eba5f2  mov      qword ptr [rbp + 0x200], rax
0x141eba5f9  mov      byte ptr [rbp + 0x325], 1
0x141eba600  mov      rcx, r13
0x141eba603  call     0x1405bdf20
0x141eba608  mov      qword ptr [rbp + 0x1f0], r14
0x141eba60f  mov      qword ptr [rbp + 0x1f8], rax
0x141eba616  mov      byte ptr [rbp + 0x325], 1
0x141eba61d  mov      rcx, r13
0x141eba620  call     0x1405bd910
0x141eba625  mov      byte ptr [rbp + 0x325], 1
0x141eba62c  lea      rcx, [rbp + 0x2d0]
0x141eba633  lea      rdx, [rbp + 0x80]
0x141eba63a  mov      r8, rax
0x141eba63d  call     0x1406861b0
0x141eba642  mov      byte ptr [rbp + 0x325], 1
0x141eba649  mov      qword ptr [rsp + 0x20], 0
0x141eba652  mov      r9d, 0x1a
0x141eba658  mov      rcx, r13
0x141eba65b  lea      rdx, [rbp + 0x2d0]
0x141eba662  lea      r8, [rip + 0x10e020b]
0x141eba669  call     0x140f9e970
0x141eba66e  mov      rax, qword ptr [rip + 0x1771a23]
0x141eba675  cmp      byte ptr [rax], 0
0x141eba678  jne      0x141eba699
0x141eba67a  mov      r15b, 2
0x141eba67d  mov      r14b, 1
0x141eba680  jmp      0x141eba69f
0x141eba682  mov      r15b, 2
0x141eba685  mov      r14b, 1
0x141eba688  mov      rax, qword ptr [rip + 0x1771a09]
0x141eba68f  cmp      byte ptr [rax], 1
0x141eba692  jne      0x141eba69f
0x141eba694  test     r12b, r12b
0x141eba697  je       0x141eba69f
0x141eba699  mov      r15b, 0xc
0x141eba69c  xor      r14d, r14d
0x141eba69f  mov      rcx, r13
0x141eba6a2  lea      rdx, [rbp + 0x80]
0x141eba6a9  mov      r8, qword ptr [rbp + 0x2b8]
0x141eba6b0  mov      r9, rdi
0x141eba6b3  call     0x14068bfc0
0x141eba6b8  mov      rcx, r13
0x141eba6bb  mov      dl, 1
0x141eba6bd  call     0x140f9dba0
0x141eba6c2  mov      rcx, rax
0x141eba6c5  movdqa   xmm1, xmm6
0x141eba6c9  call     0x140fac150
0x141eba6ce  mov      rax, qword ptr [rbp + 0x200]
0x141eba6d5  mov      qword ptr [rbp + 0x2e0], rax
0x141eba6dc  movdqu   xmm0, xmmword ptr [rbp + 0x1f0]
0x141eba6e4  movdqa   xmmword ptr [rbp + 0x2d0], xmm0
0x141eba6ec  lea      rcx, [rbp + 0x2d0]
0x141eba6f3  mov      dl, 1
0x141eba6f5  call     0x140f9dba0
0x141eba6fa  mov      dword ptr [rax + 0x258], 0xffffffa0
0x141eba704  mov      rax, qword ptr [rbp + 0x2e0]
0x141eba70b  mov      qword ptr [rbp + 0x200], rax
0x141eba712  movdqa   xmm0, xmmword ptr [rbp + 0x2d0]
0x141eba71a  movdqa   xmmword ptr [rbp + 0x1f0], xmm0
0x141eba722  mov      rcx, r13
0x141eba725  mov      dl, 1
0x141eba727  call     0x140f9dba0
0x141eba72c  mov      rcx, rax
0x141eba72f  movdqa   xmm1, xmm7
0x141eba733  call     0x140b90ec0
0x141eba738  mov      rax, qword ptr [rbp + 0x200]
0x141eba73f  mov      qword ptr [rbp + 0x2e0], rax
0x141eba746  movdqa   xmm0, xmmword ptr [rbp + 0x1f0]
0x141eba74e  movdqa   xmmword ptr [rbp + 0x2d0], xmm0
0x141eba756  lea      rcx, [rbp + 0x2d0]
0x141eba75d  mov      dl, 1
0x141eba75f  call     0x140f9dba0
0x141eba764  mov      rcx, rax
0x141eba767  mov      dl, 2
0x141eba769  call     0x140b90f40
0x141eba76e  mov      rax, qword ptr [rbp + 0x2e0]
0x141eba775  mov      qword ptr [rbp + 0x200], rax
0x141eba77c  movdqa   xmm0, xmmword ptr [rbp + 0x2d0]
0x141eba784  movdqa   xmmword ptr [rbp + 0x1f0], xmm0
0x141eba78c  mov      rdi, qword ptr [rbp + 0x1f0]
0x141eba793  cmp      qword ptr [rdi + 0x1b8], 0
0x141eba79b  je       0x141ebaff0
0x141eba7a1  lea      rcx, [rdi + 0x1c0]
0x141eba7a8  lea      rdx, [rbp + 0x1f8]
0x141eba7af  call     0x1405b0d70
0x141eba7b4  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eba7bb  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eba7c2  mov      r8, rdx
0x141eba7c5  and      r8, rax
0x141eba7c8  shr      rax, 0x39
0x141eba7cc  movd     xmm0, eax
0x141eba7d0  punpcklbw xmm0, xmm0
0x141eba7d4  pshuflw  xmm0, xmm0, 0
0x141eba7d9  pshufd   xmm0, xmm0, 0
0x141eba7de  lea      rax, [rcx - 0xc0]
0x141eba7e5  mov      r9, qword ptr [rbp + 0x1f8]
0x141eba7ec  xor      r10d, r10d
0x141eba7ef  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141eba7f5  movdqa   xmm2, xmm1
0x141eba7f9  pcmpeqb  xmm2, xmm0
0x141eba7fd  pmovmskb r11d, xmm2
0x141eba802  test     r11d, r11d
0x141eba805  je       0x141eba830
0x141eba807  tzcnt    edi, r11d
0x141eba80c  add      rdi, r8
0x141eba80f  and      rdi, rdx
0x141eba812  neg      rdi
0x141eba815  lea      rdi, [rdi + rdi*2]
0x141eba819  shl      rdi, 6
0x141eba81d  cmp      r9, qword ptr [rax + rdi]
0x141eba821  je       0x141eba853
0x141eba823  lea      edi, [r11 - 1]
0x141eba827  and      di, r11w
0x141eba82b  mov      r11d, edi
0x141eba82e  jne      0x141eba807
0x141eba830  pcmpeqb  xmm1, xmm8
0x141eba835  pmovmskb r11d, xmm1
0x141eba83a  test     r11d, r11d
0x141eba83d  jne      0x141ebaff0
0x141eba843  add      r8, r10
0x141eba846  add      r8, 0x10
0x141eba84a  add      r10, 0x10
0x141eba84e  and      r8, rdx
0x141eba851  jmp      0x141eba7ef
0x141eba853  mov      byte ptr [rcx + rdi - 4], 0
0x141eba858  mov      byte ptr [rcx + rdi - 3], r15b
0x141eba85d  mov      rax, qword ptr [rbp + 0x200]
0x141eba864  mov      qword ptr [rbp + 0x2e0], rax
0x141eba86b  movdqa   xmm0, xmmword ptr [rbp + 0x1f0]
0x141eba873  movdqa   xmmword ptr [rbp + 0x2d0], xmm0
0x141eba87b  lea      rcx, [rbp + 0xd8]
0x141eba882  lea      rdx, [rbp + 0x2d0]
0x141eba889  call     0x14067df20
0x141eba88e  test     rbx, rbx
0x141eba891  jne      0x141eba963
0x141eba897  mov      rdi, qword ptr [rbp + 0xd8]
0x141eba89e  cmp      qword ptr [rdi + 0x1b8], 0
0x141eba8a6  je       0x141ebb0d7
0x141eba8ac  lea      rcx, [rdi + 0x1c0]
0x141eba8b3  lea      rdx, [rbp + 0xe0]
0x141eba8ba  call     0x1405b0d70
0x141eba8bf  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eba8c6  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eba8cd  mov      r8, rdx
0x141eba8d0  and      r8, rax
0x141eba8d3  shr      rax, 0x39
0x141eba8d7  movd     xmm0, eax
0x141eba8db  punpcklbw xmm0, xmm0
0x141eba8df  pshuflw  xmm0, xmm0, 0
0x141eba8e4  pshufd   xmm0, xmm0, 0
0x141eba8e9  lea      rax, [rcx - 0xc0]
0x141eba8f0  mov      r9, qword ptr [rbp + 0xe0]
0x141eba8f7  xor      r10d, r10d
0x141eba8fa  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141eba900  movdqa   xmm2, xmm1
0x141eba904  pcmpeqb  xmm2, xmm0
0x141eba908  pmovmskb r11d, xmm2
0x141eba90d  test     r11d, r11d
0x141eba910  je       0x141eba93b
0x141eba912  tzcnt    edi, r11d
0x141eba917  add      rdi, r8
0x141eba91a  and      rdi, rdx
0x141eba91d  neg      rdi
0x141eba920  lea      rdi, [rdi + rdi*2]
0x141eba924  shl      rdi, 6
0x141eba928  cmp      r9, qword ptr [rax + rdi]
0x141eba92c  je       0x141eba95e
0x141eba92e  lea      edi, [r11 - 1]
0x141eba932  and      di, r11w
0x141eba936  mov      r11d, edi
0x141eba939  jne      0x141eba912
0x141eba93b  pcmpeqb  xmm1, xmm8
0x141eba940  pmovmskb r11d, xmm1
0x141eba945  test     r11d, r11d
0x141eba948  jne      0x141ebb0d7
0x141eba94e  add      r8, r10
0x141eba951  add      r8, 0x10
0x141eba955  add      r10, 0x10
0x141eba959  and      r8, rdx
0x141eba95c  jmp      0x141eba8fa
0x141eba95e  or       byte ptr [rcx + rdi - 1], 4
0x141eba963  mov      rdi, qword ptr [rbp + 0xd8]
0x141eba96a  cmp      qword ptr [rdi + 0x1b8], 0
0x141eba972  je       0x141ebb05b
0x141eba978  lea      rcx, [rdi + 0x1c0]
0x141eba97f  lea      rdx, [rbp + 0xe0]
0x141eba986  call     0x1405b0d70
0x141eba98b  mov      rcx, qword ptr [rdi + 0x1a0]
0x141eba992  mov      rdx, qword ptr [rdi + 0x1a8]
0x141eba999  mov      r8, rdx
0x141eba99c  and      r8, rax
0x141eba99f  shr      rax, 0x39
0x141eba9a3  movd     xmm0, eax
0x141eba9a7  punpcklbw xmm0, xmm0
0x141eba9ab  pshuflw  xmm0, xmm0, 0
0x141eba9b0  pshufd   xmm0, xmm0, 0
0x141eba9b5  lea      rax, [rcx - 0xc0]
0x141eba9bc  mov      r9, qword ptr [rbp + 0xe0]
0x141eba9c3  xor      r10d, r10d
0x141eba9c6  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141eba9cc  movdqa   xmm2, xmm1
0x141eba9d0  pcmpeqb  xmm2, xmm0
0x141eba9d4  pmovmskb r11d, xmm2
0x141eba9d9  test     r11d, r11d
0x141eba9dc  je       0x141ebaa07
0x141eba9de  tzcnt    edi, r11d
0x141eba9e3  add      rdi, r8
0x141eba9e6  and      rdi, rdx
0x141eba9e9  neg      rdi
0x141eba9ec  lea      rdi, [rdi + rdi*2]
0x141eba9f0  shl      rdi, 6
0x141eba9f4  cmp      r9, qword ptr [rax + rdi]
0x141eba9f8  je       0x141ebaa2a
0x141eba9fa  lea      edi, [r11 - 1]
0x141eba9fe  and      di, r11w
0x141ebaa02  mov      r11d, edi
0x141ebaa05  jne      0x141eba9de
0x141ebaa07  pcmpeqb  xmm1, xmm8
0x141ebaa0c  pmovmskb r11d, xmm1
0x141ebaa11  test     r11d, r11d
0x141ebaa14  jne      0x141ebb05b
0x141ebaa1a  add      r8, r10
0x141ebaa1d  add      r8, 0x10
0x141ebaa21  add      r10, 0x10
0x141ebaa25  and      r8, rdx
0x141ebaa28  jmp      0x141eba9c6
0x141ebaa2a  test     byte ptr [rcx + rdi - 2], 0x30
0x141ebaa2f  setne    al
0x141ebaa32  test     r14b, al
0x141ebaa35  je       0x141ebaa5d
0x141ebaa37  mov      r15d, r12d
0x141ebaa3a  cmp      qword ptr [rbp + 0x2b0], 3
0x141ebaa42  jb       0x141ebaa66
0x141ebaa44  mov      rax, qword ptr [rip + 0x177164d]
0x141ebaa4b  cmp      byte ptr [rax], 0
0x141ebaa4e  mov      r15d, r12d
0x141ebaa51  je       0x141ebaa66
0x141ebaa53  mov      rax, qword ptr [rbp + 0x420]
0x141ebaa5a  mov      byte ptr [rax], 1
0x141ebaa5d  mov      eax, dword ptr [rbp + 0x260]
0x141ebaa63  mov      r15d, eax
0x141ebaa66  mov      rdi, qword ptr [rbp + 0xd8]
0x141ebaa6d  cmp      qword ptr [rdi + 0x1b8], 0
0x141ebaa75  je       0x141ebb099
0x141ebaa7b  lea      rcx, [rdi + 0x1c0]
0x141ebaa82  lea      rdx, [rbp + 0xe0]
0x141ebaa89  call     0x1405b0d70
0x141ebaa8e  mov      rcx, qword ptr [rdi + 0x1a0]
0x141ebaa95  mov      rdx, qword ptr [rdi + 0x1a8]
0x141ebaa9c  mov      r8, rdx
0x141ebaa9f  and      r8, rax
0x141ebaaa2  shr      rax, 0x39
0x141ebaaa6  movd     xmm0, eax
0x141ebaaaa  punpcklbw xmm0, xmm0
0x141ebaaae  pshuflw  xmm0, xmm0, 0
0x141ebaab3  pshufd   xmm0, xmm0, 0
0x141ebaab8  lea      rax, [rcx - 0xc0]
0x141ebaabf  mov      r9, qword ptr [rbp + 0xe0]
0x141ebaac6  xor      r10d, r10d
0x141ebaac9  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141ebaacf  movdqa   xmm2, xmm1
0x141ebaad3  pcmpeqb  xmm2, xmm0
0x141ebaad7  pmovmskb r11d, xmm2
0x141ebaadc  test     r11d, r11d
0x141ebaadf  je       0x141ebab0a
0x141ebaae1  tzcnt    edi, r11d
0x141ebaae6  add      rdi, r8
0x141ebaae9  and      rdi, rdx
0x141ebaaec  neg      rdi
0x141ebaaef  lea      rdi, [rdi + rdi*2]
0x141ebaaf3  shl      rdi, 6
0x141ebaaf7  cmp      r9, qword ptr [rax + rdi]
0x141ebaafb  je       0x141ebab2d
0x141ebaafd  lea      edi, [r11 - 1]
0x141ebab01  and      di, r11w
0x141ebab05  mov      r11d, edi
0x141ebab08  jne      0x141ebaae1
0x141ebab0a  pcmpeqb  xmm1, xmm8
0x141ebab0f  pmovmskb r11d, xmm1
0x141ebab14  test     r11d, r11d
0x141ebab17  jne      0x141ebb099
0x141ebab1d  add      r8, r10
0x141ebab20  add      r8, 0x10
0x141ebab24  add      r10, 0x10
0x141ebab28  and      r8, rdx
0x141ebab2b  jmp      0x141ebaac9
0x141ebab2d  test     byte ptr [rcx + rdi - 2], 0x40
0x141ebab32  mov      eax, dword ptr [rbp + 0x2cc]
0x141ebab38  movzx    eax, al
0x141ebab3b  mov      ecx, 1
0x141ebab40  cmovne   eax, ecx
0x141ebab43  mov      dword ptr [rbp + 0x2cc], eax
0x141ebab49  mov      rdi, qword ptr [rbp + 0xd8]
0x141ebab50  cmp      qword ptr [rdi + 0x1b8], 0
0x141ebab58  je       0x141ebb0e8
0x141ebab5e  lea      rcx, [rdi + 0x1c0]
0x141ebab65  lea      rdx, [rbp + 0xe0]
0x141ebab6c  call     0x1405b0d70
0x141ebab71  mov      rcx, qword ptr [rdi + 0x1a0]
0x141ebab78  mov      rdx, qword ptr [rdi + 0x1a8]
0x141ebab7f  mov      r8, rdx
0x141ebab82  and      r8, rax
0x141ebab85  shr      rax, 0x39
0x141ebab89  movd     xmm0, eax
0x141ebab8d  punpcklbw xmm0, xmm0
0x141ebab91  pshuflw  xmm0, xmm0, 0
0x141ebab96  pshufd   xmm0, xmm0, 0
0x141ebab9b  lea      rax, [rcx - 0xc0]
0x141ebaba2  mov      rbx, qword ptr [rbp + 0xe0]
0x141ebaba9  xor      r9d, r9d
0x141ebabac  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141ebabb2  movdqa   xmm2, xmm1
0x141ebabb6  pcmpeqb  xmm2, xmm0
0x141ebabba  pmovmskb r10d, xmm2
0x141ebabbf  test     r10d, r10d
0x141ebabc2  je       0x141ebabed
0x141ebabc4  tzcnt    r11d, r10d
0x141ebabc9  add      r11, r8
0x141ebabcc  and      r11, rdx
0x141ebabcf  neg      r11
0x141ebabd2  lea      r11, [r11 + r11*2]
0x141ebabd6  shl      r11, 6
0x141ebabda  cmp      rbx, qword ptr [rax + r11]
0x141ebabde  je       0x141ebac10
0x141ebabe0  lea      r11d, [r10 - 1]
0x141ebabe4  and      r11w, r10w
0x141ebabe8  mov      r10d, r11d
0x141ebabeb  jne      0x141ebabc4
0x141ebabed  pcmpeqb  xmm1, xmm8
0x141ebabf2  pmovmskb r10d, xmm1
0x141ebabf7  test     r10d, r10d
0x141ebabfa  jne      0x141ebb0e8
0x141ebac00  add      r8, r9
0x141ebac03  add      r8, 0x10
0x141ebac07  add      r9, 0x10
0x141ebac0b  and      r8, rdx
0x141ebac0e  jmp      0x141ebabac
0x141ebac10  test     byte ptr [rcx + r11 - 2], 1
0x141ebac16  je       0x141ebae4f
0x141ebac1c  cmp      r12b, 5
0x141ebac20  sete     al
0x141ebac23  mov      rcx, qword ptr [rbp + 0x3f8]
0x141ebac2a  mov      byte ptr [rcx + 4], r12b
0x141ebac2e  mov      dword ptr [rcx], 0
0x141ebac34  or       al, r14b
0x141ebac37  jne      0x141ebae4f
0x141ebac3d  mov      rdi, qword ptr [rbp + 0x80]
0x141ebac44  mov      rcx, rdi
0x141ebac47  movabs   rdx, 0xb16953209e4336ba
0x141ebac51  call     0x140683ec0
0x141ebac56  mov      byte ptr [rax + 0x5d], 0
0x141ebac5a  test     dl, 1
0x141ebac5d  jne      0x141ebacaf
0x141ebac5f  mov      r14, qword ptr [rdi + 0x10]
0x141ebac63  cmp      r14, qword ptr [rdi]
0x141ebac66  jne      0x141ebac77
0x141ebac68  mov      rcx, rdi
0x141ebac6b  lea      rdx, [rip + 0xcf4356]
0x141ebac72  call     0x1406319d0
0x141ebac77  mov      rax, qword ptr [rdi + 8]
0x141ebac7b  imul     rcx, r14, 0x38
0x141ebac7f  mov      qword ptr [rax + rcx], 3
0x141ebac87  mov      qword ptr [rax + rcx + 0x10], rbx
0x141ebac8c  movups   xmmword ptr [rax + rcx + 0x18], xmm9
0x141ebac92  movsd    qword ptr [rax + rcx + 0x28], xmm10
0x141ebac99  movabs   rdx, 0xb16953209e4336ba
0x141ebaca3  mov      qword ptr [rax + rcx + 0x30], rdx
0x141ebaca8  inc      r14
0x141ebacab  mov      qword ptr [rdi + 0x10], r14
0x141ebacaf  mov      qword ptr [rbp + 0xf0], rdi
0x141ebacb6  movabs   rax, 0xb16953209e4336ba
0x141ebacc0  mov      qword ptr [rbp + 0xf8], rax
0x141ebacc7  mov      qword ptr [rbp + 0x100], 0
0x141ebacd2  mov      qword ptr [rbp + 0x1f0], 0
0x141ebacdd  movabs   rax, 0x41223169ff28813b
0x141ebace7  mov      qword ptr [rbp + 0x1f8], rax
0x141ebacee  movabs   rax, 0xa79b7268a2a968d9
0x141ebacf8  mov      qword ptr [rbp + 0x200], rax
0x141ebacff  mov      rcx, r13
0x141ebad02  call     0x1405bdf20
0x141ebad07  movabs   rcx, 0xb16953209e4336ba
0x141ebad11  mov      qword ptr [rbp + 0x1f0], rcx
0x141ebad18  mov      qword ptr [rbp + 0x1f8], rax
0x141ebad1f  mov      rcx, r13
0x141ebad22  call     0x1405bd910
0x141ebad27  lea      rcx, [rbp + 0x300]
0x141ebad2e  lea      rdx, [rbp + 0xf0]
0x141ebad35  mov      r8, rax
0x141ebad38  call     0x1406861b0
0x141ebad3d  mov      rdi, qword ptr [rbp + 0x300]
0x141ebad44  cmp      qword ptr [rdi + 0x1b8], 0
0x141ebad4c  je       0x141ebb1b4
0x141ebad52  lea      rcx, [rdi + 0x1c0]
0x141ebad59  lea      rdx, [rbp + 0x308]
0x141ebad60  call     0x1405b0d70
0x141ebad65  mov      rcx, qword ptr [rdi + 0x1a0]
0x141ebad6c  mov      rdx, qword ptr [rdi + 0x1a8]
0x141ebad73  mov      r8, rdx
0x141ebad76  and      r8, rax
0x141ebad79  shr      rax, 0x39
0x141ebad7d  movd     xmm0, eax
0x141ebad81  punpcklbw xmm0, xmm0
0x141ebad85  pshuflw  xmm0, xmm0, 0
0x141ebad8a  pshufd   xmm0, xmm0, 0
0x141ebad8f  lea      rax, [rcx - 0xc0]
0x141ebad96  mov      r9, qword ptr [rbp + 0x308]
0x141ebad9d  xor      r10d, r10d
0x141ebada0  movdqu   xmm1, xmmword ptr [rcx + r8]
0x141ebada6  movdqa   xmm2, xmm1
0x141ebadaa  pcmpeqb  xmm2, xmm0
0x141ebadae  pmovmskb r11d, xmm2
0x141ebadb3  test     r11d, r11d
0x141ebadb6  je       0x141ebade1
0x141ebadb8  tzcnt    edi, r11d
0x141ebadbd  add      rdi, r8
0x141ebadc0  and      rdi, rdx
0x141ebadc3  neg      rdi
0x141ebadc6  lea      rdi, [rdi + rdi*2]
0x141ebadca  shl      rdi, 6
0x141ebadce  cmp      r9, qword ptr [rax + rdi]
0x141ebadd2  je       0x141ebae04
0x141ebadd4  lea      edi, [r11 - 1]
0x141ebadd8  and      di, r11w
0x141ebaddc  mov      r11d, edi
0x141ebaddf  jne      0x141ebadb8
0x141ebade1  pcmpeqb  xmm1, xmm8
0x141ebade6  pmovmskb r11d, xmm1
0x141ebadeb  test     r11d, r11d
0x141ebadee  jne      0x141ebb1b4
0x141ebadf4  add      r8, r10
0x141ebadf7  add      r8, 0x10
0x141ebadfb  add      r10, 0x10
0x141ebadff  and      r8, rdx
0x141ebae02  jmp      0x141ebada0
0x141ebae04  mov      dword ptr [rcx + rdi - 0x10], 0x43480000
0x141ebae0c  mov      rax, qword ptr [rbp + 0x310]
0x141ebae13  mov      qword ptr [rbp + 0x2e0], rax
0x141ebae1a  movdqu   xmm0, xmmword ptr [rbp + 0x300]
0x141ebae22  movdqa   xmmword ptr [rbp + 0x2d0], xmm0
0x141ebae2a  mov      qword ptr [rsp + 0x20], 0
0x141ebae33  mov      r9d, 0x1c
0x141ebae39  mov      rcx, r13
0x141ebae3c  lea      rdx, [rbp + 0x2d0]
0x141ebae43  lea      r8, [rip + 0x10dfa44]
0x141ebae4a  call     0x140f9e970
0x141ebae4f  mov      rdx, qword ptr [rbp + 0x2c0]
0x141ebae56  test     rdx, rdx
0x141ebae59  je       0x141ebae6d
0x141ebae5b  mov      r8d, 1
0x141ebae61  mov      rcx, qword ptr [rbp + 0x2b8]
0x141ebae68  call     0x140613c20
0x141ebae6d  mov      rax, qword ptr [rbp + 0x1d0]
0x141ebae74  mov      rdi, qword ptr [rbp + 0x1d8]
0x141ebae7b  mov      r14d, r15d
0x141ebae7e  cmp      rdi, rax
0x141ebae81  jne      0x141eba552
0x141ebae87  mov      rcx, rax
0x141ebae8a  sub      rdi, rcx
0x141ebae8d  mov      r12d, dword ptr [rbp + 0x2cc]
0x141ebae94  jne      0x141ebb179
0x141ebae9a  mov      rax, qword ptr [rbp + 0x3f8]
0x141ebaea1  cmp      byte ptr [rax + 4], 5
0x141ebaea5  mov      rdi, qword ptr [rbp + 0x78]
0x141ebaea9  jne      0x141ebb028
0x141ebaeaf  test     r12b, 1
0x141ebaeb3  je       0x141ebb028
0x141ebaeb9  mov      rax, qword ptr [rip + 0x17711d8]
0x141ebaec0  cmp      byte ptr [rax], 1
0x141ebaec3  jne      0x141ebbfff
0x141ebaec9  mov      r15d, r14d
0x141ebaecc  mov      rbx, rdi
0x141ebaecf  lea      r8, [rip + 0x10df7f3]
0x141ebaed6  lea      rcx, [rbp + 0x1f0]
0x141ebaedd  lea      rdx, [rbp + 0x270]
0x141ebaee4  mov      r9d, 0xa
0x141ebaeea  call     0x140fb8340
0x141ebaeef  mov      qword ptr [rsp + 0x20], 0
0x141ebaef8  lea      r8, [rip + 0x10df953]
0x141ebaeff  lea      rcx, [rbp + 0x110]
0x141ebaf06  lea      rdx, [rbp + 0x1f0]
0x141ebaf0d  mov      r9d, 0x22
0x141ebaf13  call     0x140f9e970
0x141ebaf18  mov      rdi, qword ptr [rbp + 0x120]
0x141ebaf1f  cmp      qword ptr [rdi + 0x1b8], 0
0x141ebaf27  je       0x141ebc230
0x141ebaf2d  lea      rdx, [rbp + 0x128]
0x141ebaf34  lea      rcx, [rdi + 0x1c0]
0x141ebaf3b  call     0x1405b0d70
0x141ebaf40  mov      rcx, qword ptr [rdi + 0x1a0]
0x141ebaf47  mov      rdx, qword ptr [rdi + 0x1a8]
0x141ebaf4e  mov      r8, rdx
0x141ebaf51  and      r8, rax
0x141ebaf54  shr      rax, 0x39
0x141ebaf58  movd     xmm0, eax
0x141ebaf5c  punpcklbw xmm0, xmm0
0x141ebaf60  pshuflw  xmm0, xmm0, 0
0x141ebaf65  pshufd   xmm0, xmm0, 0
0x141ebaf6a  lea      rax, [rcx - 0xc0]
0x141ebaf71  mov      r9, qword ptr [rbp + 0x128]
0x141ebaf78  xor      r10d, r10d
0x141ebaf7b  pcmpeqd  xmm1, xmm1
0x141ebaf7f  mov      r12d, dword ptr [rbp + 0x2cc]
0x141ebaf86  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141ebaf8c  movdqa   xmm3, xmm2
0x141ebaf90  pcmpeqb  xmm3, xmm0
0x141ebaf94  pmovmskb r11d, xmm3
0x141ebaf99  test     r11d, r11d
0x141ebaf9c  je       0x141ebafce
0x141ebaf9e  mov      r14d, r15d
0x141ebafa1  tzcnt    edi, r11d
0x141ebafa6  add      rdi, r8
0x141ebafa9  and      rdi, rdx
0x141ebafac  neg      rdi
0x141ebafaf  lea      rdi, [rdi + rdi*2]
0x141ebafb3  shl      rdi, 6
0x141ebafb7  cmp      r9, qword ptr [rax + rdi]
0x141ebafbb  je       0x141ebc185
0x141ebafc1  lea      edi, [r11 - 1]
0x141ebafc5  and      di, r11w
0x141ebafc9  mov      r11d, edi
0x141ebafcc  jne      0x141ebafa1
0x141ebafce  pcmpeqb  xmm2, xmm1
0x141ebafd2  pmovmskb r11d, xmm2
0x141ebafd7  test     r11d, r11d
0x141ebafda  jne      0x141ebc230
0x141ebafe0  add      r8, r10
0x141ebafe3  add      r8, 0x10
0x141ebafe7  add      r10, 0x10
0x141ebafeb  and      r8, rdx
0x141ebafee  jmp      0x141ebaf86
0x141ebaff0  lea      rcx, [rip + 0xb5cff9]
0x141ebaff7  call     0x1428d9310
0x141ebaffc  jmp      0x141ebc902
0x141ebb001  lea      rcx, [rip + 0xb5c200]
0x141ebb008  lea      r8, [rip + 0xb5cfc9]
0x141ebb00f  mov      edx, 0x16
0x141ebb014  call     0x1428d9330
0x141ebb019  jmp      0x141ebc902
0x141ebb01e  mov      r14b, 6
0x141ebb021  mov      rdi, qword ptr [r12]
0x141ebb025  xor      r12d, r12d
0x141ebb028  cmp      r14b, 6
0x141ebb02c  je       0x141ebb6a9
0x141ebb032  movabs   r9, 0x8000000000000000
0x141ebb03c  movzx    eax, r14b
0x141ebb040  lea      rcx, [rip + 0x10df9c9]
0x141ebb047  movsxd   rax, dword ptr [rcx + rax*4]
0x141ebb04b  add      rax, rcx
0x141ebb04e  mov      r13, rdi
0x141ebb051  jmp      rax
0x141ebb053  mov      r15d, r14d
0x141ebb056  jmp      0x141ebb1ed
0x141ebb05b  lea      rcx, [rip + 0xb5c1a6]
0x141ebb062  lea      r8, [rip + 0xb5cf6f]
0x141ebb069  mov      edx, 0x16
0x141ebb06e  call     0x1428d9330
0x141ebb073  jmp      0x141ebc902
0x141ebb078  test     rax, rax
0x141ebb07b  jne      0x141ebbff3
0x141ebb081  xor      edx, edx
0x141ebb083  call     0x1428e6250
0x141ebb088  mov      rcx, rax
0x141ebb08b  test     rax, rax
0x141ebb08e  jne      0x141eb98ca
0x141ebb094  jmp      0x141ebbff3
0x141ebb099  lea      rcx, [rip + 0xb5c168]
0x141ebb0a0  lea      r8, [rip + 0xb5cf31]
0x141ebb0a7  mov      edx, 0x16
0x141ebb0ac  call     0x1428d9330
0x141ebb0b1  jmp      0x141ebc902
0x141ebb0b6  test     rax, rax
0x141ebb0b9  jne      0x141ebbff3
0x141ebb0bf  xor      edx, edx
0x141ebb0c1  call     0x1428e6250
0x141ebb0c6  mov      rcx, rax
0x141ebb0c9  test     rax, rax
0x141ebb0cc  jne      0x141eb9d7e
0x141ebb0d2  jmp      0x141ebbff3
0x141ebb0d7  lea      rcx, [rip + 0xb5cf12]
0x141ebb0de  call     0x1428d9310
0x141ebb0e3  jmp      0x141ebc902
0x141ebb0e8  lea      rcx, [rip + 0xb5c119]
0x141ebb0ef  lea      r8, [rip + 0xb5cee2]
0x141ebb0f6  mov      edx, 0x16
0x141ebb0fb  call     0x1428d9330
0x141ebb100  jmp      0x141ebc902
0x141ebb105  mov      r15d, r14d
0x141ebb108  mov      r14b, 3
0x141ebb10b  jmp      0x141ebb1ed
0x141ebb110  xor      eax, eax
0x141ebb112  cmp      rax, qword ptr [rdi]
0x141ebb115  jo       0x141ebb6a9
0x141ebb11b  mov      r8, qword ptr [rdi + 0x88]
0x141ebb122  test     r8, r8
0x141ebb125  js       0x141ebc8ce
0x141ebb12b  mov      r15d, r14d
0x141ebb12e  movzx    r14d, byte ptr [rdi + 0x70]
0x141ebb133  mov      rdi, qword ptr [rdi + 0x80]
0x141ebb13a  je       0x141ebb1c5
0x141ebb140  movzx    eax, byte ptr [rip + 0x1779ac2]
0x141ebb147  mov      eax, 1
0x141ebb14c  mov      qword ptr [rbp + 0x2b0], rax
0x141ebb153  mov      edx, 1
0x141ebb158  mov      rbx, r8
0x141ebb15b  mov      rcx, r8
0x141ebb15e  call     0x140613c10
0x141ebb163  test     rax, rax
0x141ebb166  je       0x141ebc8dc
0x141ebb16c  mov      r8, rbx
0x141ebb16f  jmp      0x141ebb1ca
0x141ebb171  mov      r15d, r14d
0x141ebb174  mov      r14b, 1
0x141ebb177  jmp      0x141ebb1ed
0x141ebb179  shl      rcx, 5
0x141ebb17d  lea      rax, [rbp + 0x110]
0x141ebb184  lea      rbx, [rcx + rax]
0x141ebb188  add      rbx, 0x10
0x141ebb18c  jmp      0x141ebb19b
0x141ebb18e  add      rbx, 0x20
0x141ebb192  dec      rdi
0x141ebb195  je       0x141ebae9a
0x141ebb19b  mov      rdx, qword ptr [rbx - 8]
0x141ebb19f  test     rdx, rdx
0x141ebb1a2  je       0x141ebb18e
0x141ebb1a4  mov      rcx, qword ptr [rbx]
0x141ebb1a7  mov      r8d, 1
0x141ebb1ad  call     0x140613c20
0x141ebb1b2  jmp      0x141ebb18e
0x141ebb1b4  lea      rcx, [rip + 0xb5ce35]
0x141ebb1bb  call     0x1428d9310
0x141ebb1c0  jmp      0x141ebc902
0x141ebb1c5  mov      eax, 1
0x141ebb1ca  mov      qword ptr [rbp + 0x2b8], rax
0x141ebb1d1  mov      rcx, rax
0x141ebb1d4  mov      rdx, rdi
0x141ebb1d7  mov      rdi, r8
0x141ebb1da  call     0x142923c00
0x141ebb1df  mov      r9, rdi
0x141ebb1e2  mov      rdi, r13
0x141ebb1e5  movzx    r10d, byte ptr [r13 + 0x90]
0x141ebb1ed  mov      rax, qword ptr [rbp + 0x430]
0x141ebb1f4  mov      ecx, dword ptr [rax + 0x1c]
0x141ebb1f7  mov      rdx, qword ptr [rax + 0x10]
0x141ebb1fb  mov      dword ptr [rdx], ecx
0x141ebb1fd  mov      rax, qword ptr [rax]
0x141ebb200  inc      qword ptr [rax + 0x1d8]
0x141ebb207  mov      rax, qword ptr [rbp + 0x410]
0x141ebb20e  mov      ecx, dword ptr [rax + 0x1c]
0x141ebb211  mov      rdx, qword ptr [rax + 0x10]
0x141ebb215  mov      dword ptr [rdx], ecx
0x141ebb217  mov      rax, qword ptr [rax]
0x141ebb21a  mov      byte ptr [rax], 2
0x141ebb21d  mov      dword ptr [rbp + 0x260], r15d
0x141ebb224  cmp      r15b, 5
0x141ebb228  mov      qword ptr [rbp + 0x2b0], r9
0x141ebb22f  jne      0x141ebb259
0x141ebb231  xor      eax, eax
0x141ebb233  cmp      rax, qword ptr [rdi]
0x141ebb236  jno      0x141ebb270
0x141ebb238  add      rdi, 8
0x141ebb23c  movabs   rax, 0x8000000000000000
0x141ebb246  mov      ecx, 1
0x141ebb24b  mov      qword ptr [rbp + 0x2c0], rcx
0x141ebb252  xor      ecx, ecx
0x141ebb254  jmp      0x141ebb501
0x141ebb259  movabs   rax, 0x8000000000000000
0x141ebb263  mov      r8d, 1
0x141ebb269  xor      ecx, ecx
0x141ebb26b  jmp      0x141ebb508
0x141ebb270  mov      rcx, qword ptr [rdi + 0x28]
0x141ebb274  test     rcx, rcx
0x141ebb277  js       0x141ebc8ae
0x141ebb27d  mov      byte ptr [rbp + 0x298], r10b
0x141ebb284  je       0x141ebb2e0
0x141ebb286  mov      r15, qword ptr [rdi + 0x20]
0x141ebb28a  movzx    eax, byte ptr [rip + 0x1779978]
0x141ebb291  mov      edi, 1
0x141ebb296  mov      edx, 1
0x141ebb29b  mov      rbx, rcx
0x141ebb29e  call     0x140613c10
0x141ebb2a3  test     rax, rax
0x141ebb2a6  je       0x141ebc8b3
0x141ebb2ac  lea      r9, [rbx - 1]
0x141ebb2b0  cmp      rbx, r9
0x141ebb2b3  cmovb    r9, rbx
0x141ebb2b7  cmp      r9, 0x20
0x141ebb2bb  setb     cl
0x141ebb2be  mov      rdx, rax
0x141ebb2c1  sub      rdx, r15
0x141ebb2c4  cmp      rdx, 0x20
0x141ebb2c8  setb     dl
0x141ebb2cb  or       dl, cl
0x141ebb2cd  mov      rcx, rbx
0x141ebb2d0  je       0x141ebb2f1
0x141ebb2d2  xor      r9d, r9d
0x141ebb2d5  mov      r11, rcx
0x141ebb2d8  mov      rdx, r15
0x141ebb2db  mov      r10, r13
0x141ebb2de  jmp      0x141ebb33a
0x141ebb2e0  mov      eax, 1
0x141ebb2e5  mov      qword ptr [rbp + 0x2c0], rax
0x141ebb2ec  jmp      0x141ebb3a2
0x141ebb2f1  inc      r9
0x141ebb2f4  mov      r8d, r9d
0x141ebb2f7  and      r8d, 0x1f
0x141ebb2fb  mov      edx, 0x20
0x141ebb300  cmovne   rdx, r8
0x141ebb304  sub      r9, rdx
0x141ebb307  mov      r11, rcx
0x141ebb30a  sub      r11, r9
0x141ebb30d  lea      rdx, [r15 + r9]
0x141ebb311  xor      r8d, r8d
0x141ebb314  mov      r10, r13
0x141ebb317  movdqu   xmm0, xmmword ptr [r15 + r8]
0x141ebb31d  movdqu   xmm1, xmmword ptr [r15 + r8 + 0x10]
0x141ebb324  movdqu   xmmword ptr [rax + r8], xmm0
0x141ebb32a  movdqu   xmmword ptr [rax + r8 + 0x10], xmm1
0x141ebb331  add      r8, 0x20
0x141ebb335  cmp      r9, r8
0x141ebb338  jne      0x141ebb317
0x141ebb33a  add      r15, rcx
0x141ebb33d  mov      qword ptr [rbp + 0x2c0], rax
0x141ebb344  mov      r8, rax
0x141ebb347  add      r8, r9
0x141ebb34a  xor      eax, eax
0x141ebb34c  lea      r9, [rdx + rax]
0x141ebb350  cmp      r9, r15
0x141ebb353  je       0x141ebb365
0x141ebb355  movzx    r9d, byte ptr [r9]
0x141ebb359  mov      byte ptr [r8 + rax], r9b
0x141ebb35d  inc      rax
0x141ebb360  cmp      r11, rax
0x141ebb363  jne      0x141ebb34c
0x141ebb365  xor      eax, eax
0x141ebb367  xor      edx, edx
0x141ebb369  cmp      rax, qword ptr [r10]
0x141ebb36c  seto     dl
0x141ebb36f  lea      rdi, [r10 + rdx*8]
0x141ebb373  jno      0x141ebb393
0x141ebb375  movabs   rax, 0x8000000000000000
0x141ebb37f  mov      r9, qword ptr [rbp + 0x2b0]
0x141ebb386  movzx    r10d, byte ptr [rbp + 0x298]
0x141ebb38e  jmp      0x141ebb501
0x141ebb393  mov      r9, qword ptr [rbp + 0x2b0]
0x141ebb39a  movzx    r10d, byte ptr [rbp + 0x298]
0x141ebb3a2  xor      eax, eax
0x141ebb3a4  cmp      rax, qword ptr [rdi + 0x30]
0x141ebb3a8  jno      0x141ebb3b9
0x141ebb3aa  movabs   rax, 0x8000000000000000
0x141ebb3b4  jmp      0x141ebb4a7
0x141ebb3b9  mov      qword ptr [rbp + 0x268], rcx
0x141ebb3c0  lea      rdx, [rdi + 0x30]
0x141ebb3c4  mov      r15b, byte ptr [rdi + 0x58]
0x141ebb3c8  lea      rcx, [rbp + 0x110]
0x141ebb3cf  call     0x140d7bd90
0x141ebb3d4  mov      byte ptr [rbp + 0x138], r15b
0x141ebb3db  movups   xmm0, xmmword ptr [rdi + 0x48]
0x141ebb3df  movups   xmmword ptr [rbp + 0x128], xmm0
0x141ebb3e6  movq     xmm1, qword ptr [rdi + 0x60]
0x141ebb3eb  movd     xmm0, dword ptr [rdi + 0x68]
0x141ebb3f0  movzx    edx, byte ptr [rdi + 0x6c]
0x141ebb3f4  movzx    ebx, byte ptr [rdi + 0x6d]
0x141ebb3f8  mov      rax, qword ptr [rbp + 0x110]
0x141ebb3ff  mov      rdi, qword ptr [rbp + 0x118]
0x141ebb406  movdqu   xmm2, xmmword ptr [rbp + 0x120]
0x141ebb40e  mov      r8, qword ptr [rbp + 0x130]
0x141ebb415  movzx    r9d, byte ptr [rbp + 0x138]
0x141ebb41d  mov      r10d, dword ptr [rbp + 0x139]
0x141ebb424  movzx    r11d, word ptr [rbp + 0x13d]
0x141ebb42c  movzx    r15d, byte ptr [rbp + 0x13f]
0x141ebb434  movdqa   xmmword ptr [rbp + 0x1f0], xmm2
0x141ebb43c  mov      qword ptr [rbp + 0x200], r8
0x141ebb443  mov      byte ptr [rbp + 0x208], r9b
0x141ebb44a  mov      dword ptr [rbp + 0x209], r10d
0x141ebb451  mov      word ptr [rbp + 0x20d], r11w
0x141ebb459  mov      byte ptr [rbp + 0x20f], r15b
0x141ebb460  movabs   r8, 0x8000000000000001
0x141ebb46a  cmp      rax, r8
0x141ebb46d  jne      0x141ebb491
0x141ebb46f  movabs   rax, 0x8000000000000000
0x141ebb479  mov      r9, qword ptr [rbp + 0x2b0]
0x141ebb480  mov      rcx, qword ptr [rbp + 0x268]
0x141ebb487  movzx    r10d, byte ptr [rbp + 0x298]
0x141ebb48f  jmp      0x141ebb501
0x141ebb491  mov      r9, qword ptr [rbp + 0x2b0]
0x141ebb498  mov      rcx, qword ptr [rbp + 0x268]
0x141ebb49f  movzx    r10d, byte ptr [rbp + 0x298]
0x141ebb4a7  movdqa   xmm2, xmmword ptr [rbp + 0x1f0]
0x141ebb4af  mov      r8, qword ptr [rbp + 0x200]
0x141ebb4b6  mov      qword ptr [rbp + 0x280], r8
0x141ebb4bd  movzx    r8d, byte ptr [rbp + 0x208]
0x141ebb4c5  mov      byte ptr [rbp + 0x288], r8b
0x141ebb4cc  mov      r8d, dword ptr [rbp + 0x209]
0x141ebb4d3  mov      dword ptr [rbp + 0x289], r8d
0x141ebb4da  movzx    r8d, word ptr [rbp + 0x20d]
0x141ebb4e2  mov      word ptr [rbp + 0x28d], r8w
0x141ebb4ea  movzx    r8d, byte ptr [rbp + 0x20f]
0x141ebb4f2  mov      byte ptr [rbp + 0x28f], r8b
0x141ebb4f9  movdqa   xmmword ptr [rbp + 0x270], xmm2
0x141ebb501  mov      r8, qword ptr [rbp + 0x2c0]
0x141ebb508  mov      byte ptr [rbp + 0x248], r14b
0x141ebb50f  mov      qword ptr [rbp + 0x1f0], rcx
0x141ebb516  mov      qword ptr [rbp + 0x1f8], r8
0x141ebb51d  mov      qword ptr [rbp + 0x200], rcx
0x141ebb524  mov      qword ptr [rbp + 0x208], rax
0x141ebb52b  mov      qword ptr [rbp + 0x210], rdi
0x141ebb532  movdqa   xmm2, xmmword ptr [rbp + 0x270]
0x141ebb53a  movdqa   xmm3, xmmword ptr [rbp + 0x280]
0x141ebb542  movdqu   xmmword ptr [rbp + 0x218], xmm2
0x141ebb54a  movdqu   xmmword ptr [rbp + 0x228], xmm3
0x141ebb552  movq     qword ptr [rbp + 0x238], xmm1
0x141ebb55a  movd     dword ptr [rbp + 0x240], xmm0
0x141ebb562  mov      byte ptr [rbp + 0x244], dl
0x141ebb568  mov      byte ptr [rbp + 0x245], bl
0x141ebb56e  mov      qword ptr [rbp + 0x270], r9
0x141ebb575  mov      rax, qword ptr [rbp + 0x2b8]
0x141ebb57c  mov      qword ptr [rbp + 0x278], rax
0x141ebb583  mov      qword ptr [rbp + 0x280], r9
0x141ebb58a  mov      byte ptr [rbp + 0x288], r10b
0x141ebb591  mov      byte ptr [rbp + 0x326], 0
0x141ebb598  lea      rcx, [rbp + 0x110]
0x141ebb59f  lea      rdx, [rbp + 0x1f0]
0x141ebb5a6  lea      r8, [rbp + 0x270]
0x141ebb5ad  call     0x140b46960
0x141ebb5b2  mov      rcx, qword ptr [rbp + 0x418]
0x141ebb5b9  mov      byte ptr [rbp + 0x326], 0
0x141ebb5c0  lea      r8, [rip + 0x10df329]
0x141ebb5c7  lea      rdx, [rbp + 0x110]
0x141ebb5ce  call     0x140fe0190
0x141ebb5d3  mov      rdx, qword ptr [rbp + 0x400]
0x141ebb5da  mov      eax, dword ptr [rdx + 0x1c]
0x141ebb5dd  mov      rcx, qword ptr [rdx + 0x10]
0x141ebb5e1  mov      dword ptr [rcx], eax
0x141ebb5e3  mov      rcx, qword ptr [rdx]
0x141ebb5e6  mov      qword ptr [rbp + 0x110], 2
0x141ebb5f1  mov      byte ptr [rbp + 0x326], 0
0x141ebb5f8  lea      rdx, [rbp + 0x110]
0x141ebb5ff  call     0x14093fad0
0x141ebb604  mov      rax, qword ptr [rbp + 0x438]
0x141ebb60b  mov      ecx, dword ptr [rax + 0x1c]
0x141ebb60e  mov      rdx, qword ptr [rax + 0x10]
0x141ebb612  mov      dword ptr [rdx], ecx
0x141ebb614  mov      rdi, qword ptr [rax]
0x141ebb617  mov      rbx, qword ptr [rdi + 0x30]
0x141ebb61b  mov      r14, qword ptr [rdi + 0x40]
0x141ebb61f  cmp      rbx, qword ptr [rdi + 0x20]
0x141ebb623  jne      0x141ebb63c
0x141ebb625  lea      rcx, [rdi + 0x20]
0x141ebb629  mov      byte ptr [rbp + 0x326], 0
0x141ebb630  lea      rdx, [rip + 0xcf9c39]
0x141ebb637  call     0x140562870
0x141ebb63c  mov      rax, qword ptr [rdi + 0x28]
0x141ebb640  mov      qword ptr [rax + rbx*8], r14
0x141ebb644  inc      qword ptr [rdi + 0x40]
0x141ebb648  inc      rbx
0x141ebb64b  mov      qword ptr [rdi + 0x30], rbx
0x141ebb64f  cmp      byte ptr [rbp + 0x260], 5
0x141ebb656  mov      rdi, r13
0x141ebb659  jne      0x141ebb6a9
0x141ebb65b  mov      rax, qword ptr [rbp + 0x440]
0x141ebb662  mov      ecx, dword ptr [rax + 0x1c]
0x141ebb665  mov      rdx, qword ptr [rax + 0x10]
0x141ebb669  mov      dword ptr [rdx], ecx
0x141ebb66b  mov      rdi, qword ptr [rax]
0x141ebb66e  mov      rbx, qword ptr [rdi + 0x30]
0x141ebb672  mov      r14, qword ptr [rdi + 0x40]
0x141ebb676  cmp      rbx, qword ptr [rdi + 0x20]
0x141ebb67a  jne      0x141ebb693
0x141ebb67c  lea      rcx, [rdi + 0x20]
0x141ebb680  mov      byte ptr [rbp + 0x326], 0
0x141ebb687  lea      rdx, [rip + 0xcf9be2]
0x141ebb68e  call     0x140562870
0x141ebb693  mov      rax, qword ptr [rdi + 0x28]
0x141ebb697  mov      qword ptr [rax + rbx*8], r14
0x141ebb69b  inc      rbx
0x141ebb69e  mov      qword ptr [rdi + 0x30], rbx
0x141ebb6a2  inc      qword ptr [rdi + 0x40]
0x141ebb6a6  mov      rdi, r13
0x141ebb6a9  mov      rax, qword ptr [rbp + 0x3f8]
0x141ebb6b0  movzx    eax, byte ptr [rax + 4]
0x141ebb6b4  cmp      eax, 6
0x141ebb6b7  jne      0x141ebb6cf
0x141ebb6b9  movabs   rax, 0x8000000000000000
0x141ebb6c3  mov      qword ptr [rbp + 0x300], rax
0x141ebb6ca  jmp      0x141ebb8b4
0x141ebb6cf  mov      eax, eax
0x141ebb6d1  lea      rcx, [rip + 0x10df350]
0x141ebb6d8  movsxd   rax, dword ptr [rcx + rax*4]
0x141ebb6dc  add      rax, rcx
0x141ebb6df  jmp      rax
0x141ebb6e1  mov      eax, dword ptr [rbp + 0x2a8]
0x141ebb6e7  mov      rcx, qword ptr [rbp + 0x1e8]
0x141ebb6ee  mov      dword ptr [rcx], eax
0x141ebb6f0  mov      rax, qword ptr [rbp + 0x2a0]
0x141ebb6f7  mov      byte ptr [rax + 0x1cc], 0
0x141ebb6fe  lea      rax, [rip + 0x10df245]
0x141ebb705  jmp      0x141ebb7c1
0x141ebb70a  mov      eax, dword ptr [rbp + 0x2a8]
0x141ebb710  mov      rcx, qword ptr [rbp + 0x1e8]
0x141ebb717  mov      dword ptr [rcx], eax
0x141ebb719  mov      rax, qword ptr [rbp + 0x2a0]
0x141ebb720  mov      byte ptr [rax + 0x1cc], 4
0x141ebb727  lea      rax, [rip + 0x10df234]
0x141ebb72e  jmp      0x141ebb7c1
0x141ebb733  mov      eax, dword ptr [rbp + 0x2a8]
0x141ebb739  mov      rcx, qword ptr [rbp + 0x1e8]
0x141ebb740  mov      dword ptr [rcx], eax
0x141ebb742  mov      rax, qword ptr [rbp + 0x2a0]
0x141ebb749  mov      byte ptr [rax + 0x1cc], 2
0x141ebb750  lea      rax, [rip + 0x10df1ff]
0x141ebb757  mov      qword ptr [rbp + 0x270], rax
0x141ebb75e  mov      eax, 7
0x141ebb763  jmp      0x141ebb841
0x141ebb768  mov      eax, dword ptr [rbp + 0x2a8]
0x141ebb76e  mov      rcx, qword ptr [rbp + 0x1e8]
0x141ebb775  mov      dword ptr [rcx], eax
0x141ebb777  mov      rax, qword ptr [rbp + 0x2a0]
0x141ebb77e  mov      byte ptr [rax + 0x1cc], 1
0x141ebb785  lea      rax, [rip + 0x10df1d1]
0x141ebb78c  mov      qword ptr [rbp + 0x270], rax
0x141ebb793  mov      eax, 5
0x141ebb798  jmp      0x141ebb841
0x141ebb79d  mov      eax, dword ptr [rbp + 0x2a8]
0x141ebb7a3  mov      rcx, qword ptr [rbp + 0x1e8]
0x141ebb7aa  mov      dword ptr [rcx], eax
0x141ebb7ac  mov      rax, qword ptr [rbp + 0x2a0]
0x141ebb7b3  mov      byte ptr [rax + 0x1cc], 3
0x141ebb7ba  lea      rax, [rip + 0x10df18f]
0x141ebb7c1  mov      qword ptr [rbp + 0x270], rax
0x141ebb7c8  mov      eax, 6
0x141ebb7cd  jmp      0x141ebb841
0x141ebb7cf  xor      eax, eax
0x141ebb7d1  cmp      rax, qword ptr [rdi]
0x141ebb7d4  jno      0x141ebb7e4
0x141ebb7d6  mov      eax, 0x11
0x141ebb7db  lea      rcx, [rip + 0x10df186]
0x141ebb7e2  jmp      0x141ebb83a
0x141ebb7e4  mov      rdx, qword ptr [rdi + 0x88]
0x141ebb7eb  test     rdx, rdx
0x141ebb7ee  js       0x141ebc8f4
0x141ebb7f4  movzx    ecx, byte ptr [rdi + 0x70]
0x141ebb7f8  movzx    eax, cl
0x141ebb7fb  je       0x141ebb804
0x141ebb7fd  movzx    edx, byte ptr [rip + 0x1779405]
0x141ebb804  mov      edx, dword ptr [rbp + 0x2a8]
0x141ebb80a  mov      r8, qword ptr [rbp + 0x1e8]
0x141ebb811  mov      dword ptr [r8], edx
0x141ebb814  mov      rdx, qword ptr [rbp + 0x2a0]
0x141ebb81b  mov      byte ptr [rdx + 0x1cc], cl
0x141ebb821  lea      rdx, [rip + 0x10df1a8]
0x141ebb828  movsxd   rcx, dword ptr [rdx + rax*4]
0x141ebb82c  add      rcx, rdx
0x141ebb82f  lea      rdx, [rip + 0x10df1b2]
0x141ebb836  mov      rax, qword ptr [rdx + rax*8]
0x141ebb83a  mov      qword ptr [rbp + 0x270], rcx
0x141ebb841  mov      qword ptr [rbp + 0x278], rax
0x141ebb848  lea      rax, [rbp + 0x270]
0x141ebb84f  mov      qword ptr [rbp + 0x1f0], rax
0x141ebb856  lea      rax, [rip - 0x1db19cd]
0x141ebb85d  mov      qword ptr [rbp + 0x1f8], rax
0x141ebb864  lea      rax, [rip + 0x10df155]
0x141ebb86b  mov      qword ptr [rbp + 0x110], rax
0x141ebb872  mov      qword ptr [rbp + 0x118], 1
0x141ebb87d  lea      rax, [rbp + 0x1f0]
0x141ebb884  mov      qword ptr [rbp + 0x120], rax
0x141ebb88b  mov      qword ptr [rbp + 0x128], 1
0x141ebb896  mov      qword ptr [rbp + 0x130], 0
0x141ebb8a1  lea      rcx, [rbp + 0x300]
0x141ebb8a8  lea      rdx, [rbp + 0x110]
0x141ebb8af  call     0x140d79f90
0x141ebb8b4  test     r12b, 1
0x141ebb8b8  je       0x141ebb906
0x141ebb8ba  xor      eax, eax
0x141ebb8bc  cmp      rax, qword ptr [rbp + 0x300]
0x141ebb8c3  jno      0x141ebb950
0x141ebb8c9  movzx    eax, byte ptr [rip + 0x1779339]
0x141ebb8d0  mov      ecx, 0x10
0x141ebb8d5  mov      edx, 1
0x141ebb8da  call     0x140613c10
0x141ebb8df  test     rax, rax
0x141ebb8e2  jne      0x141ebb925
0x141ebb8e4  mov      byte ptr [rbp + 0x324], 1
0x141ebb8eb  lea      r8, [rip + 0x10debb6]
0x141ebb8f2  mov      ecx, 1
0x141ebb8f7  mov      edx, 0x10
0x141ebb8fc  call     0x1428d8fe3
0x141ebb901  jmp      0x141ebc902
0x141ebb906  movzx    eax, byte ptr [rip + 0x17792fc]
0x141ebb90d  mov      ecx, 0x10
0x141ebb912  mov      edx, 1
0x141ebb917  call     0x140613c10
0x141ebb91c  test     rax, rax
0x141ebb91f  je       0x141ebc88f
0x141ebb925  movdqu   xmm0, xmmword ptr [rip + 0x10defdb]
0x141ebb92d  movdqu   xmmword ptr [rax], xmm0
0x141ebb931  mov      qword ptr [rbp + 0x2d0], 0x10
0x141ebb93c  mov      qword ptr [rbp + 0x2d8], rax
0x141ebb943  mov      qword ptr [rbp + 0x2e0], 0x10
0x141ebb94e  jmp      0x141ebb96a
0x141ebb950  mov      byte ptr [rbp + 0x324], 1
0x141ebb957  lea      rcx, [rbp + 0x2d0]
0x141ebb95e  lea      rdx, [rbp + 0x300]
0x141ebb965  call     0x140d7bd90
0x141ebb96a  mov      eax, dword ptr [rbp + 0x2f8]
0x141ebb970  mov      rcx, qword ptr [rbp + 0x258]
0x141ebb977  mov      dword ptr [rcx], eax
0x141ebb979  lea      rax, [rip + 0x10def98]
0x141ebb980  mov      qword ptr [rbp + 0x1f0], rax
0x141ebb987  mov      qword ptr [rbp + 0x1f8], 0x18
0x141ebb992  mov      byte ptr [rbp + 0x327], 1
0x141ebb999  lea      rcx, [rbp + 0x1f0]
0x141ebb9a0  call     0x1405bdda0
0x141ebb9a5  mov      rdi, rax
0x141ebb9a8  movabs   rax, 0xb1695320004fb443
0x141ebb9b2  xor      rdi, rax
0x141ebb9b5  mov      byte ptr [rbp + 0x327], 1
0x141ebb9bc  mov      rcx, rsi
0x141ebb9bf  mov      rdx, rdi
0x141ebb9c2  call     0x140683ec0
0x141ebb9c7  mov      byte ptr [rax + 0x5d], 0
0x141ebb9cb  mov      byte ptr [rax + 0xb7], 1
0x141ebb9d2  test     dl, 1
0x141ebb9d5  jne      0x141ebba11
0x141ebb9d7  mov      rbx, qword ptr [rsi + 0x10]
0x141ebb9db  cmp      rbx, qword ptr [rsi]
0x141ebb9de  jne      0x141ebb9f6
0x141ebb9e0  mov      byte ptr [rbp + 0x327], 1
0x141ebb9e7  lea      rdx, [rip + 0xcf35da]
0x141ebb9ee  mov      rcx, rsi
0x141ebb9f1  call     0x1406319d0
0x141ebb9f6  mov      rax, qword ptr [rsi + 8]
0x141ebb9fa  imul     rcx, rbx, 0x38
0x141ebb9fe  mov      dword ptr [rax + rcx], 0
0x141ebba05  mov      qword ptr [rax + rcx + 0x30], rdi
0x141ebba0a  inc      rbx
0x141ebba0d  mov      qword ptr [rsi + 0x10], rbx
0x141ebba11  mov      qword ptr [rbp + 0x110], rsi
0x141ebba18  mov      qword ptr [rbp + 0x118], rdi
0x141ebba1f  mov      qword ptr [rbp + 0x120], 0
0x141ebba2a  mov      rdi, qword ptr [rbp + 0x2d8]
0x141ebba31  mov      rax, qword ptr [rbp + 0x2e0]
0x141ebba38  mov      byte ptr [rbp + 0x327], 1
0x141ebba3f  mov      qword ptr [rsp + 0x28], rax
0x141ebba44  mov      qword ptr [rsp + 0x20], rdi
0x141ebba49  mov      dword ptr [rsp + 0x30], 0x447a0000
0x141ebba51  lea      r8, [rip + 0x10deed8]
0x141ebba58  lea      rcx, [rbp + 0x1f0]
0x141ebba5f  lea      rdx, [rbp + 0x110]
0x141ebba66  mov      r9d, 2
0x141ebba6c  call     0x140f9fca0
0x141ebba71  mov      byte ptr [rbp + 0x327], 1
0x141ebba78  lea      rcx, [rbp + 0x1f0]
0x141ebba7f  mov      dl, 1
0x141ebba81  call     0x140f9e130
0x141ebba86  mov      byte ptr [rax + 0x44], 1
0x141ebba8a  mov      rax, qword ptr [rbp + 0x200]
0x141ebba91  mov      qword ptr [rbp + 0x280], rax
0x141ebba98  movdqu   xmm0, xmmword ptr [rbp + 0x1f0]
0x141ebbaa0  movdqa   xmmword ptr [rbp + 0x270], xmm0
0x141ebbaa8  mov      rbx, qword ptr [rbp + 0x270]
0x141ebbaaf  cmp      qword ptr [rbx + 0x1b8], 0
0x141ebbab7  je       0x141ebbd1e
0x141ebbabd  lea      rdx, [rbp + 0x278]
0x141ebbac4  lea      rcx, [rbx + 0x1c0]
0x141ebbacb  mov      byte ptr [rbp + 0x327], 1
0x141ebbad2  call     0x1405b0d70
0x141ebbad7  mov      rcx, qword ptr [rbx + 0x1a0]
0x141ebbade  mov      rdx, qword ptr [rbx + 0x1a8]
0x141ebbae5  mov      r8, rdx
0x141ebbae8  and      r8, rax
0x141ebbaeb  shr      rax, 0x39
0x141ebbaef  movd     xmm0, eax
0x141ebbaf3  punpcklbw xmm0, xmm0
0x141ebbaf7  pshuflw  xmm0, xmm0, 0
0x141ebbafc  pshufd   xmm0, xmm0, 0
0x141ebbb01  lea      rax, [rcx - 0xc0]
0x141ebbb08  mov      r9, qword ptr [rbp + 0x278]
0x141ebbb0f  xor      r10d, r10d
0x141ebbb12  pcmpeqd  xmm1, xmm1
0x141ebbb16  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141ebbb1c  movdqa   xmm3, xmm2
0x141ebbb20  pcmpeqb  xmm3, xmm0
0x141ebbb24  pmovmskb r11d, xmm3
0x141ebbb29  test     r11d, r11d
0x141ebbb2c  je       0x141ebbb57
0x141ebbb2e  tzcnt    ebx, r11d
0x141ebbb33  add      rbx, r8
0x141ebbb36  and      rbx, rdx
0x141ebbb39  neg      rbx
0x141ebbb3c  lea      rbx, [rbx + rbx*2]
0x141ebbb40  shl      rbx, 6
0x141ebbb44  cmp      r9, qword ptr [rax + rbx]
0x141ebbb48  je       0x141ebbb79
0x141ebbb4a  lea      ebx, [r11 - 1]
0x141ebbb4e  and      bx, r11w
0x141ebbb52  mov      r11d, ebx
0x141ebbb55  jne      0x141ebbb2e
0x141ebbb57  pcmpeqb  xmm2, xmm1
0x141ebbb5b  pmovmskb r11d, xmm2
0x141ebbb60  test     r11d, r11d
0x141ebbb63  jne      0x141ebbd1e
0x141ebbb69  add      r8, r10
0x141ebbb6c  add      r8, 0x10
0x141ebbb70  add      r10, 0x10
0x141ebbb74  and      r8, rdx
0x141ebbb77  jmp      0x141ebbb16
0x141ebbb79  mov      dword ptr [rcx + rbx - 0x68], 0x3f800000
0x141ebbb81  mov      rbx, qword ptr [rbp + 0x300]
0x141ebbb88  mov      rax, rbx
0x141ebbb8b  neg      rax
0x141ebbb8e  jo       0x141ebbf22
0x141ebbb94  mov      qword ptr [rbp + 0x2a0], rbx
0x141ebbb9b  mov      rax, qword ptr [rbp + 0x308]
0x141ebbba2  mov      qword ptr [rbp + 0x2a8], rax
0x141ebbba9  mov      r14, qword ptr [rbp + 0x310]
0x141ebbbb0  mov      eax, dword ptr [rbp + 0x2f8]
0x141ebbbb6  mov      rcx, qword ptr [rbp + 0x258]
0x141ebbbbd  mov      dword ptr [rcx], eax
0x141ebbbbf  lea      rax, [rip + 0x10ded6c]
0x141ebbbc6  mov      qword ptr [rbp + 0x110], rax
0x141ebbbcd  mov      qword ptr [rbp + 0x118], 0x18
0x141ebbbd8  lea      rcx, [rbp + 0x110]
0x141ebbbdf  call     0x1405bdda0
0x141ebbbe4  mov      rbx, rax
0x141ebbbe7  movabs   rax, 0xb1695320004fb443
0x141ebbbf1  xor      rbx, rax
0x141ebbbf4  mov      rcx, rsi
0x141ebbbf7  mov      rdx, rbx
0x141ebbbfa  call     0x140683ec0
0x141ebbbff  mov      byte ptr [rax + 0x5d], 0
0x141ebbc03  mov      byte ptr [rax + 0xb7], 1
0x141ebbc0a  test     dl, 1
0x141ebbc0d  jne      0x141ebbc42
0x141ebbc0f  mov      r15, qword ptr [rsi + 0x10]
0x141ebbc13  cmp      r15, qword ptr [rsi]
0x141ebbc16  jne      0x141ebbc27
0x141ebbc18  lea      rdx, [rip + 0xcf33a9]
0x141ebbc1f  mov      rcx, rsi
0x141ebbc22  call     0x1406319d0
0x141ebbc27  mov      rax, qword ptr [rsi + 8]
0x141ebbc2b  imul     rcx, r15, 0x38
0x141ebbc2f  mov      dword ptr [rax + rcx], 0
0x141ebbc36  mov      qword ptr [rax + rcx + 0x30], rbx
0x141ebbc3b  inc      r15
0x141ebbc3e  mov      qword ptr [rsi + 0x10], r15
0x141ebbc42  mov      qword ptr [rbp + 0x270], rsi
0x141ebbc49  mov      qword ptr [rbp + 0x278], rbx
0x141ebbc50  mov      qword ptr [rbp + 0x280], 0
0x141ebbc5b  cmp      qword ptr [rsi + 0x1b8], 0
0x141ebbc63  je       0x141ebbfbd
0x141ebbc69  lea      rdx, [rbp + 0x278]
0x141ebbc70  lea      rcx, [rsi + 0x1c0]
0x141ebbc77  call     0x1405b0d70
0x141ebbc7c  mov      rcx, qword ptr [rsi + 0x1a0]
0x141ebbc83  mov      rdx, qword ptr [rsi + 0x1a8]
0x141ebbc8a  mov      r8, rdx
0x141ebbc8d  and      r8, rax
0x141ebbc90  shr      rax, 0x39
0x141ebbc94  movd     xmm0, eax
0x141ebbc98  punpcklbw xmm0, xmm0
0x141ebbc9c  pshuflw  xmm0, xmm0, 0
0x141ebbca1  pshufd   xmm0, xmm0, 0
0x141ebbca6  lea      rax, [rcx - 0xc0]
0x141ebbcad  mov      r9, qword ptr [rbp + 0x278]
0x141ebbcb4  xor      r10d, r10d
0x141ebbcb7  pcmpeqd  xmm1, xmm1
0x141ebbcbb  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141ebbcc1  movdqa   xmm3, xmm2
0x141ebbcc5  pcmpeqb  xmm3, xmm0
0x141ebbcc9  pmovmskb r11d, xmm3
0x141ebbcce  test     r11d, r11d
0x141ebbcd1  je       0x141ebbcfc
0x141ebbcd3  tzcnt    esi, r11d
0x141ebbcd8  add      rsi, r8
0x141ebbcdb  and      rsi, rdx
0x141ebbcde  neg      rsi
0x141ebbce1  lea      rsi, [rsi + rsi*2]
0x141ebbce5  shl      rsi, 6
0x141ebbce9  cmp      r9, qword ptr [rax + rsi]
0x141ebbced  je       0x141ebbd36
0x141ebbcef  lea      esi, [r11 - 1]
0x141ebbcf3  and      si, r11w
0x141ebbcf7  mov      r11d, esi
0x141ebbcfa  jne      0x141ebbcd3
0x141ebbcfc  pcmpeqb  xmm2, xmm1
0x141ebbd00  pmovmskb r11d, xmm2
0x141ebbd05  test     r11d, r11d
0x141ebbd08  jne      0x141ebbfbd
0x141ebbd0e  add      r8, r10
0x141ebbd11  add      r8, 0x10
0x141ebbd15  add      r10, 0x10
0x141ebbd19  and      r8, rdx
0x141ebbd1c  jmp      0x141ebbcbb
0x141ebbd1e  mov      byte ptr [rbp + 0x327], 1
0x141ebbd25  lea      rcx, [rip + 0xcf32fc]
0x141ebbd2c  call     0x1428d9310
0x141ebbd31  jmp      0x141ebc902
0x141ebbd36  mov      dword ptr [rcx + rsi - 0x10], 0x3f800000
0x141ebbd3e  mov      qword ptr [rsp + 0x28], r14
0x141ebbd43  mov      rax, qword ptr [rbp + 0x2a8]
0x141ebbd4a  mov      qword ptr [rsp + 0x20], rax
0x141ebbd4f  mov      dword ptr [rsp + 0x30], 0x447a0000
0x141ebbd57  lea      r8, [rip + 0x10debd2]
0x141ebbd5e  lea      rcx, [rbp + 0x110]
0x141ebbd65  lea      rdx, [rbp + 0x270]
0x141ebbd6c  mov      r9d, 2
0x141ebbd72  call     0x140f9fca0
0x141ebbd77  mov      rax, qword ptr [rbp + 0x3f8]
0x141ebbd7e  movss    xmm6, dword ptr [rax]
0x141ebbd82  lea      rcx, [rbp + 0x110]
0x141ebbd89  mov      dl, 1
0x141ebbd8b  call     0x140f9e130
0x141ebbd90  mulss    xmm6, dword ptr [rip + 0xbd0ed0]
0x141ebbd98  addss    xmm6, dword ptr [rip + 0xa69b64]
0x141ebbda0  movss    xmm0, dword ptr [rip + 0xa7eec8]
0x141ebbda8  mulss    xmm6, xmm0
0x141ebbdac  pxor     xmm1, xmm1
0x141ebbdb0  pxor     xmm2, xmm2
0x141ebbdb4  maxss    xmm2, xmm6
0x141ebbdb8  minss    xmm0, xmm2
0x141ebbdbc  cvttss2si rcx, xmm0
0x141ebbdc1  xor      edx, edx
0x141ebbdc3  ucomiss  xmm0, xmm1
0x141ebbdc6  cmovae   edx, ecx
0x141ebbdc9  ucomiss  xmm0, dword ptr [rip + 0xae4a00]
0x141ebbdd0  mov      ecx, 0xffffffff
0x141ebbdd5  cmovbe   ecx, edx
0x141ebbdd8  or       ecx, 0xffffff00
0x141ebbdde  mov      dword ptr [rax + 0x40], ecx
0x141ebbde1  mov      rax, qword ptr [rbp + 0x120]
0x141ebbde8  mov      qword ptr [rbp + 0x200], rax
0x141ebbdef  movdqu   xmm0, xmmword ptr [rbp + 0x110]
0x141ebbdf7  movdqa   xmmword ptr [rbp + 0x1f0], xmm0
0x141ebbdff  lea      rcx, [rbp + 0x1f0]
0x141ebbe06  mov      dl, 1
0x141ebbe08  call     0x140f9e130
0x141ebbe0d  mov      byte ptr [rax + 0x44], 1
0x141ebbe11  mov      rax, qword ptr [rbp + 0x200]
0x141ebbe18  mov      qword ptr [rbp + 0x120], rax
0x141ebbe1f  movdqa   xmm0, xmmword ptr [rbp + 0x1f0]
0x141ebbe27  movdqa   xmmword ptr [rbp + 0x110], xmm0
0x141ebbe2f  mov      rsi, qword ptr [rbp + 0x110]
0x141ebbe36  cmp      qword ptr [rsi + 0x1b8], 0
0x141ebbe3e  je       0x141ebbfce
0x141ebbe44  lea      rdx, [rbp + 0x118]
0x141ebbe4b  lea      rcx, [rsi + 0x1c0]
0x141ebbe52  call     0x1405b0d70
0x141ebbe57  mov      rcx, qword ptr [rsi + 0x1a0]
0x141ebbe5e  mov      rdx, qword ptr [rsi + 0x1a8]
0x141ebbe65  mov      r8, rdx
0x141ebbe68  and      r8, rax
0x141ebbe6b  shr      rax, 0x39
0x141ebbe6f  movd     xmm0, eax
0x141ebbe73  punpcklbw xmm0, xmm0
0x141ebbe77  pshuflw  xmm0, xmm0, 0
0x141ebbe7c  pshufd   xmm0, xmm0, 0
0x141ebbe81  lea      rax, [rcx - 0xc0]
0x141ebbe88  mov      r9, qword ptr [rbp + 0x118]
0x141ebbe8f  xor      r10d, r10d
0x141ebbe92  pcmpeqd  xmm1, xmm1
0x141ebbe96  mov      rbx, qword ptr [rbp + 0x2a0]
0x141ebbe9d  movdqu   xmm2, xmmword ptr [rcx + r8]
0x141ebbea3  movdqa   xmm3, xmm2
0x141ebbea7  pcmpeqb  xmm3, xmm0
0x141ebbeab  pmovmskb r11d, xmm3
0x141ebbeb0  test     r11d, r11d
0x141ebbeb3  je       0x141ebbede
0x141ebbeb5  tzcnt    esi, r11d
0x141ebbeba  add      rsi, r8
0x141ebbebd  and      rsi, rdx
0x141ebbec0  neg      rsi
0x141ebbec3  lea      rsi, [rsi + rsi*2]
0x141ebbec7  shl      rsi, 6
0x141ebbecb  cmp      r9, qword ptr [rax + rsi]
0x141ebbecf  je       0x141ebbf00
0x141ebbed1  lea      esi, [r11 - 1]
0x141ebbed5  and      si, r11w
0x141ebbed9  mov      r11d, esi
0x141ebbedc  jne      0x141ebbeb5
0x141ebbede  pcmpeqb  xmm2, xmm1
0x141ebbee2  pmovmskb r11d, xmm2
0x141ebbee7  test     r11d, r11d
0x141ebbeea  jne      0x141ebbfce
0x141ebbef0  add      r8, r10
0x141ebbef3  add      r8, 0x10
0x141ebbef7  add      r10, 0x10
0x141ebbefb  and      r8, rdx
0x141ebbefe  jmp      0x141ebbe9d
0x141ebbf00  mov      dword ptr [rcx + rsi - 0x68], 0x3f800000
0x141ebbf08  test     rbx, rbx
0x141ebbf0b  je       0x141ebbf22
0x141ebbf0d  mov      r8d, 1
0x141ebbf13  mov      rcx, qword ptr [rbp + 0x2a8]
0x141ebbf1a  mov      rdx, rbx
0x141ebbf1d  call     0x140613c20
0x141ebbf22  mov      rdx, qword ptr [rbp + 0x2d0]
0x141ebbf29  test     rdx, rdx
0x141ebbf2c  je       0x141ebbf3c
0x141ebbf2e  mov      r8d, 1
0x141ebbf34  mov      rcx, rdi
0x141ebbf37  call     0x140613c20
0x141ebbf3c  neg      rbx
0x141ebbf3f  seto     al
0x141ebbf42  mov      rdx, qword ptr [rbp + 0x300]
0x141ebbf49  mov      rcx, rdx
0x141ebbf4c  neg      rcx
0x141ebbf4f  setno    cl
0x141ebbf52  test     al, cl
0x141ebbf54  je       0x141ebbf6d
0x141ebbf56  test     rdx, rdx
0x141ebbf59  je       0x141ebbf6d
0x141ebbf5b  mov      rcx, qword ptr [rbp + 0x308]
0x141ebbf62  mov      r8d, 1
0x141ebbf68  call     0x140613c20
0x141ebbf6d  cmp      qword ptr [rbp + 0xa8], 0
0x141ebbf75  je       0x141eb9dac
0x141ebbf7b  call     0x140c18130
0x141ebbf80  mov      rcx, rax
0x141ebbf83  mov      rax, qword ptr [rax]
0x141ebbf86  cmp      rax, 1
0x141ebbf8a  jne      0x141ebbfdf
0x141ebbf8c  add      rcx, 8
