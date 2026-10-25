0x140aa4a60  push     rbp
0x140aa4a61  push     r15
0x140aa4a63  push     r14
0x140aa4a65  push     r13
0x140aa4a67  push     r12
0x140aa4a69  push     rsi
0x140aa4a6a  push     rdi
0x140aa4a6b  push     rbx
0x140aa4a6c  sub      rsp, 0xf8
0x140aa4a73  lea      rbp, [rsp + 0x80]
0x140aa4a7b  movaps   xmmword ptr [rbp + 0x60], xmm8
0x140aa4a80  movaps   xmmword ptr [rbp + 0x50], xmm7
0x140aa4a84  movaps   xmmword ptr [rbp + 0x40], xmm6
0x140aa4a88  mov      qword ptr [rbp + 0x38], 0xfffffffffffffffe
0x140aa4a90  mov      rax, qword ptr [rcx]
0x140aa4a93  mov      r9, qword ptr [rcx + 0x10]
0x140aa4a97  movss    xmm6, dword ptr [rax + 0x28]
0x140aa4a9c  mov      ecx, dword ptr [rcx + 0x1c]
0x140aa4a9f  mov      dword ptr [r9], ecx
0x140aa4aa2  cmp      qword ptr [rax + 0x18], 0
0x140aa4aa7  je       0x140aa4fb2
0x140aa4aad  mov      rcx, qword ptr [rax + 0x10]
0x140aa4ab1  mov      r9, qword ptr [rax]
0x140aa4ab4  mov      rax, qword ptr [rax + 8]
0x140aa4ab8  xor      r10d, r10d
0x140aa4abb  cmp      rcx, r9
0x140aa4abe  cmovb    r9, r10
0x140aa4ac2  sub      rcx, r9
0x140aa4ac5  shl      rcx, 6
0x140aa4ac9  lea      r9, [rax + rcx]
0x140aa4acd  mov      qword ptr [rbp - 0x58], r9
0x140aa4ad1  mov      qword ptr [rbp + 0x18], 0
0x140aa4ad9  mov      qword ptr [rbp + 0x20], 8
0x140aa4ae1  mov      qword ptr [rbp + 0x28], 0
0x140aa4ae9  mov      r13, qword ptr [rax + rcx + 8]
0x140aa4aee  mov      rax, qword ptr [rax + rcx + 0x10]
0x140aa4af3  lea      r11, [rax + rax*2]
0x140aa4af7  shl      r11, 4
0x140aa4afb  add      r11, r13
0x140aa4afe  mov      eax, dword ptr [r8 + 0x1c]
0x140aa4b02  mov      dword ptr [rbp - 4], eax
0x140aa4b05  mov      rax, qword ptr [r8]
0x140aa4b08  mov      qword ptr [rbp + 0x30], rax
0x140aa4b0c  mov      rax, qword ptr [r8 + 0x10]
0x140aa4b10  mov      qword ptr [rbp - 0x50], rax
0x140aa4b14  mov      rax, qword ptr [rdx]
0x140aa4b17  mov      qword ptr [rbp - 0x48], rax
0x140aa4b1b  xorps    xmm7, xmm7
0x140aa4b1e  movss    xmm8, dword ptr [rip + 0x1e8c249]
0x140aa4b27  mov      qword ptr [rbp], r11
0x140aa4b2b  jmp      0x140aa4b5b
0x140aa4b2d  mov      rax, qword ptr [rbp + 0x20]
0x140aa4b31  mov      qword ptr [rax + rsi*8], r12
0x140aa4b35  inc      rsi
0x140aa4b38  mov      qword ptr [rbp + 0x28], rsi
0x140aa4b3c  mov      rax, qword ptr [rbp + 8]
0x140aa4b40  movss    xmm0, dword ptr [rax + 0x18]
0x140aa4b45  mov      rcx, qword ptr [rbp - 0x48]
0x140aa4b49  addss    xmm0, dword ptr [rcx + 0x28]
0x140aa4b4e  movss    dword ptr [rax + 0x18], xmm0
0x140aa4b53  mov      r11, qword ptr [rbp]
0x140aa4b57  mov      r10, qword ptr [rbp + 0x10]
0x140aa4b5b  mov      rsi, r13
0x140aa4b5e  cmp      r13, r11
0x140aa4b61  je       0x140aa4cce
0x140aa4b67  mov      r12, r10
0x140aa4b6a  lea      r13, [rsi + 0x30]
0x140aa4b6e  inc      r10
0x140aa4b71  cmp      byte ptr [rsi], 9
0x140aa4b74  jne      0x140aa4b5b
0x140aa4b76  mov      qword ptr [rbp + 0x10], r10
0x140aa4b7a  mov      qword ptr [rbp + 8], rsi
0x140aa4b7e  movss    xmm0, dword ptr [rsi + 0x18]
0x140aa4b83  ucomiss  xmm0, xmm7
0x140aa4b86  jne      0x140aa4c49
0x140aa4b8c  jp       0x140aa4c49
0x140aa4b92  mov      rax, qword ptr [rbp + 8]
0x140aa4b96  mov      rdi, qword ptr [rax + 0x10]
0x140aa4b9a  cmp      byte ptr [rax + 0x1c], 0
0x140aa4b9e  mov      eax, dword ptr [rbp - 4]
0x140aa4ba1  mov      rcx, qword ptr [rbp - 0x50]
0x140aa4ba5  mov      dword ptr [rcx], eax
0x140aa4ba7  je       0x140aa4c3c
0x140aa4bad  mov      rax, qword ptr [rbp + 0x30]
0x140aa4bb1  mov      r14, qword ptr [rax + 0x18]
0x140aa4bb5  test     r14, r14
0x140aa4bb8  je       0x140aa4c49
0x140aa4bbe  mov      rax, qword ptr [rbp + 0x30]
0x140aa4bc2  mov      r15, qword ptr [rax]
0x140aa4bc5  lea      rbx, [r15 + 0x10]
0x140aa4bc9  movdqa   xmm0, xmmword ptr [r15]
0x140aa4bce  pmovmskb esi, xmm0
0x140aa4bd2  not      esi
0x140aa4bd4  jmp      0x140aa4bec
0x140aa4bd6  nop      word ptr cs:[rax + rax]
0x140aa4be0  lea      eax, [rsi - 1]
0x140aa4be3  and      eax, esi
0x140aa4be5  mov      esi, eax
0x140aa4be7  dec      r14
0x140aa4bea  je       0x140aa4c49
0x140aa4bec  test     si, si
0x140aa4bef  jne      0x140aa4c1a
0x140aa4bf1  nop      word ptr cs:[rax + rax]
0x140aa4c00  movdqa   xmm0, xmmword ptr [rbx]
0x140aa4c04  pmovmskb esi, xmm0
0x140aa4c08  add      r15, -0x80
0x140aa4c0c  add      rbx, 0x10
0x140aa4c10  cmp      esi, 0xffff
0x140aa4c16  je       0x140aa4c00
0x140aa4c18  not      esi
0x140aa4c1a  tzcnt    eax, esi
0x140aa4c1e  shl      eax, 3
0x140aa4c21  mov      rdx, r15
0x140aa4c24  sub      rdx, rax
0x140aa4c27  mov      rax, qword ptr [rdi + 0x10]
0x140aa4c2b  cmp      rax, qword ptr [rdx - 8]
0x140aa4c2f  jne      0x140aa4be0
0x140aa4c31  mov      rcx, qword ptr [rbp + 0x30]
0x140aa4c35  call     0x140b7a160
0x140aa4c3a  jmp      0x140aa4be0
0x140aa4c3c  mov      rdx, qword ptr [rdi + 0x10]
0x140aa4c40  mov      rcx, qword ptr [rbp + 0x30]
0x140aa4c44  call     0x1402ca2f0
0x140aa4c49  mov      rax, qword ptr [rbp + 8]
0x140aa4c4d  test     byte ptr [rax + 8], 1
0x140aa4c51  je       0x140aa4ca0
0x140aa4c53  movss    xmm1, dword ptr [rax + 0xc]
0x140aa4c58  divss    xmm1, xmm6
0x140aa4c5c  movss    xmm0, dword ptr [rax + 0x18]
0x140aa4c61  maxss    xmm1, xmm8
0x140aa4c66  ucomiss  xmm0, xmm1
0x140aa4c69  jbe      0x140aa4b45
0x140aa4c6f  mov      rsi, qword ptr [rbp + 0x28]
0x140aa4c73  cmp      rsi, qword ptr [rbp + 0x18]
0x140aa4c77  jne      0x140aa4b2d
0x140aa4c7d  lea      rcx, [rbp + 0x18]
0x140aa4c81  lea      rdx, [rip + 0x2038138]
0x140aa4c88  call     0x140b81df0
0x140aa4c8d  jmp      0x140aa4b2d
0x140aa4c92  nop      word ptr cs:[rax + rax]
0x140aa4ca0  mov      rsi, qword ptr [rbp + 0x28]
0x140aa4ca4  cmp      rsi, qword ptr [rbp + 0x18]
0x140aa4ca8  jne      0x140aa4cba
0x140aa4caa  lea      rcx, [rbp + 0x18]
0x140aa4cae  lea      rdx, [rip + 0x20380f3]
0x140aa4cb5  call     0x140b81df0
0x140aa4cba  mov      rax, qword ptr [rbp + 0x20]
0x140aa4cbe  mov      qword ptr [rax + rsi*8], r12
0x140aa4cc2  inc      rsi
0x140aa4cc5  mov      qword ptr [rbp + 0x28], rsi
0x140aa4cc9  jmp      0x140aa4b53
0x140aa4cce  mov      r10, qword ptr [rbp + 0x20]
0x140aa4cd2  mov      rax, qword ptr [rbp + 0x28]
