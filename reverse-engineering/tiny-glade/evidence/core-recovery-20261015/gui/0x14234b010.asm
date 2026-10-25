0x14234b010 push     rbp
0x14234b011 push     r15
0x14234b013 push     r14
0x14234b015 push     r13
0x14234b017 push     r12
0x14234b019 push     rsi
0x14234b01a push     rdi
0x14234b01b push     rbx
0x14234b01c sub      rsp, 0x348
0x14234b023 lea      rbp, [rsp + 0x80]
0x14234b02b movaps   xmmword ptr [rbp + 0x2b0], xmm10
0x14234b033 movaps   xmmword ptr [rbp + 0x2a0], xmm9
0x14234b03b movaps   xmmword ptr [rbp + 0x290], xmm8
0x14234b043 movaps   xmmword ptr [rbp + 0x280], xmm7
0x14234b04a movaps   xmmword ptr [rbp + 0x270], xmm6
0x14234b051 mov      qword ptr [rbp + 0x268], 0xfffffffffffffffe
0x14234b05c mov      qword ptr [rbp + 0x240], r9
0x14234b063 mov      qword ptr [rbp + 0x218], r8
0x14234b06a mov      r15, rcx
0x14234b06d mov      rax, qword ptr [rbp + 0x360]
0x14234b074 mov      r12, qword ptr [rdx]
0x14234b077 mov      r8, qword ptr [rdx + 0x20]
0x14234b07b mov      rcx, qword ptr [rdx + 0x40]
0x14234b07f mov      qword ptr [rbp + 0x190], rcx
0x14234b086 mov      rcx, qword ptr [rdx + 0x60]
0x14234b08a mov      qword ptr [rbp + 0x1a0], rcx
0x14234b091 mov      rbx, qword ptr [rdx + 0x80]
0x14234b098 mov      rcx, qword ptr [rdx + 0xa0]
0x14234b09f mov      qword ptr [rbp + 0x210], rcx
0x14234b0a6 mov      rcx, qword ptr [rdx + 0xc0]
0x14234b0ad mov      qword ptr [rbp + 0x220], rcx
0x14234b0b4 mov      rcx, qword ptr [rdx + 0xe0]
0x14234b0bb mov      qword ptr [rbp + 0x1c8], rcx
0x14234b0c2 mov      rcx, qword ptr [rax]
0x14234b0c5 mov      qword ptr [rbp + 0x170], rcx
0x14234b0cc mov      rax, qword ptr [rax + 0x20]
0x14234b0d0 mov      qword ptr [rbp + 0x178], rax
0x14234b0d7 mov      rcx, qword ptr [rbp + 0x358]
0x14234b0de mov      rax, qword ptr [rbp + 0x338]
0x14234b0e5 mov      rdx, qword ptr [rcx + 0x20]
0x14234b0e9 mov      qword ptr [rbp + 0x1d0], rdx
0x14234b0f0 mov      rdx, qword ptr [rcx + 0x30]
0x14234b0f4 mov      qword ptr [rbp + 0x1d8], rdx
0x14234b0fb mov      edx, dword ptr [rcx + 0x3c]
0x14234b0fe mov      dword ptr [rbp + 0x1f4], edx
0x14234b104 mov      rdx, qword ptr [rcx + 0x40]
0x14234b108 mov      qword ptr [rbp + 0x208], rdx
0x14234b10f mov      rdx, qword ptr [rcx + 0x48]
0x14234b113 mov      qword ptr [rbp + 0x200], rdx
0x14234b11a mov      rsi, qword ptr [rcx + 0x68]
0x14234b11e mov      rdi, qword ptr [rcx + 0x70]
0x14234b122 mov      rdx, qword ptr [rcx + 0x90]
0x14234b129 mov      qword ptr [rbp + 0x1b0], rdx
0x14234b130 mov      rdx, qword ptr [rcx + 0xa0]
0x14234b137 mov      qword ptr [rbp + 0x1b8], rdx
0x14234b13e mov      edx, dword ptr [rcx + 0xac]
0x14234b144 mov      dword ptr [rbp + 0x1f0], edx
0x14234b14a mov      r13, qword ptr [rcx + 0xb0]
0x14234b151 mov      rdx, qword ptr [rcx + 0xc0]
0x14234b158 mov      qword ptr [rbp + 0x1e0], rdx
0x14234b15f mov      edx, dword ptr [rcx + 0xcc]
0x14234b165 mov      dword ptr [rbp + 0x23c], edx
0x14234b16b mov      rdx, qword ptr [rcx + 0xf0]
0x14234b172 mov      qword ptr [rbp + 0x230], rdx
0x14234b179 mov      rcx, qword ptr [rcx + 0xf8]
0x14234b180 mov      qword ptr [rbp + 0x188], rcx
0x14234b187 mov      ecx, dword ptr [rax + 0x1c]
0x14234b18a mov      rdx, qword ptr [rax + 0x10]
0x14234b18e mov      dword ptr [rdx], ecx
0x14234b190 mov      r14, qword ptr [rax]
0x14234b193 mov      qword ptr [rbp + 0x1c0], r8
0x14234b19a mov      rcx, r8
0x14234b19d call     0x14093fbc0
0x14234b1a2 test     al, al
0x14234b1a4 je       0x14234b1b0
0x14234b1a6 mov      byte ptr [r14 + 0xa], 0
0x14234b1ab mov      byte ptr [r14 + 0xd], 1
0x14234b1b0 mov      eax, dword ptr [r12]
0x14234b1b4 cmp      eax, 0xd
0x14234b1b7 je       0x14234b1f7
0x14234b1b9 cmp      eax, 0x13
0x14234b1bc jne      0x14234b1ec
0x14234b1be mov      rax, qword ptr [rbp + 0x210]
0x14234b1c5 mov      ecx, dword ptr [rax + 0x100]
0x14234b1cb movss    xmm1, dword ptr [rax + 0x104]
0x14234b1d3 call     0x14093fa90
0x14234b1d8 test     al, al
0x14234b1da je       0x14234b1f7
0x14234b1dc movzx    eax, byte ptr [r14 + 0xa]
0x14234b1e1 mov      ecx, eax
0x14234b1e3 xor      cl, 1
0x14234b1e6 mov      byte ptr [r14 + 0xa], cl
0x14234b1ea jmp      0x14234b1f3
0x14234b1ec mov      byte ptr [r14 + 0xa], 0
0x14234b1f1 mov      al, 1
0x14234b1f3 mov      byte ptr [r14 + 0xd], al
0x14234b1f7 mov      qword ptr [rbp + 0x198], rbx
0x14234b1fe mov      rcx, qword ptr [rsi]
0x14234b201 xor      eax, eax
0x14234b203 mov      rdx, rcx
0x14234b206 sub      rdx, qword ptr [rdi + 0x18]
0x14234b20a cmovb    rdx, rax
0x14234b20e sub      rcx, qword ptr [rdi + 0x38]
0x14234b212 cmovb    rcx, rax
0x14234b216 lea      r8, [rdx + rdx*4]
0x14234b21a shl      r8, 3
0x14234b21e add      r8, qword ptr [rdi + 8]
0x14234b222 lea      r9, [rcx + rcx*4]
0x14234b226 shl      r9, 3
0x14234b22a add      r9, qword ptr [rdi + 0x28]
0x14234b22e mov      r10, qword ptr [rdi + 0x10]
0x14234b232 sub      r10, rdx
0x14234b235 cmovb    r10, rax
0x14234b239 mov      rdx, qword ptr [rdi + 0x30]
0x14234b23d mov      r11d, 8
0x14234b243 cmovb    r8, r11
0x14234b247 lea      rbx, [r10 + r10*4]
0x14234b24b mov      qword ptr [rbp + 0x248], r14
0x14234b252 setbe    r14b
0x14234b256 lea      r8, [r8 + rbx*8 - 0x28]
0x14234b25b cmovbe   r8, rax
0x14234b25f sub      rdx, rcx
0x14234b262 cmovb    rdx, rax
0x14234b266 cmovb    r9, r11
0x14234b26a lea      rcx, [rdx + rdx*4]
0x14234b26e setbe    r11b
0x14234b272 lea      rbx, [r9 + rcx*8 - 0x28]
0x14234b277 cmovbe   rbx, r8
0x14234b27b add      rdx, r10
0x14234b27e test     r14b, r11b
0x14234b281 cmove    rdx, rax
0x14234b285 mov      rax, qword ptr [rdi + 0x40]
0x14234b289 sub      rax, rdx
0x14234b28c mov      qword ptr [rsi], rax
0x14234b28f test     r14b, r11b
0x14234b292 mov      r14, qword ptr [rbp + 0x248]
0x14234b299 jne      0x14234b386
0x14234b29f mov      rsi, qword ptr [rbp + 0x340]
0x14234b2a6 cmp      dword ptr [rbx], 0x13
0x14234b2a9 jne      0x14234b325
0x14234b2ab cmp      dword ptr [rbx + 0x10], 0xd
0x14234b2af je       0x14234b325
0x14234b2b1 mov      eax, dword ptr [r15 + 0x1c]
0x14234b2b5 mov      rcx, qword ptr [r15 + 0x10]
0x14234b2b9 mov      dword ptr [rcx], eax
0x14234b2bb mov      rax, qword ptr [r15]
0x14234b2be mov      byte ptr [rax + 1], 1
0x14234b2c2 mov      rcx, r14
0x14234b2c5 call     0x140870570
0x14234b2ca movzx    ecx, byte ptr [r14 + 9]
0x14234b2cf call     0x14090e500
0x14234b2d4 mov      r8, qword ptr [rbp + 0x240]
0x14234b2db mov      ecx, dword ptr [r8 + 0x1c]
0x14234b2df mov      rdx, qword ptr [r8 + 0x10]
0x14234b2e3 mov      dword ptr [rdx], ecx
0x14234b2e5 mov      rcx, qword ptr [r8]
0x14234b2e8 mov      byte ptr [rcx + 0x103], al
0x14234b2ee mov      eax, dword ptr [rsi + 0x1c]
0x14234b2f1 mov      rcx, qword ptr [rsi + 0x10]
0x14234b2f5 mov      dword ptr [rcx], eax
0x14234b2f7 mov      rcx, qword ptr [rsi]
0x14234b2fa mov      dl, 2
0x14234b2fc call     0x1408aea90
0x14234b301 cmp      byte ptr [r14 + 0xb], 0
0x14234b306 jne      0x14234b325
0x14234b308 mov      rdx, qword ptr [rbp + 0x218]
0x14234b30f mov      eax, dword ptr [rdx + 0x1c]
0x14234b312 mov      rcx, qword ptr [rdx + 0x10]
0x14234b316 mov      dword ptr [rcx], eax
0x14234b318 mov      rcx, qword ptr [rdx]
0x14234b31b call     0x14090e6f0
0x14234b320 mov      byte ptr [r14 + 0xb], 1
0x14234b325 cmp      dword ptr [rbx + 0x10], 0x13
0x14234b329 jne      0x14234b386
0x14234b32b cmp      dword ptr [rbx], 0xd
0x14234b32e je       0x14234b386
0x14234b330 mov      rdi, qword ptr [rbp + 0x348]
0x14234b337 mov      rdx, qword ptr [rbp + 0x240]
0x14234b33e mov      eax, dword ptr [rdx + 0x1c]
0x14234b341 mov      rcx, qword ptr [rdx + 0x10]
0x14234b345 mov      dword ptr [rcx], eax
0x14234b347 mov      rax, qword ptr [rdx]
0x14234b34a mov      byte ptr [rax + 0x103], 0
0x14234b351 mov      eax, dword ptr [rsi + 0x1c]
0x14234b354 mov      rcx, qword ptr [rsi + 0x10]
0x14234b358 mov      dword ptr [rcx], eax
0x14234b35a mov      rcx, qword ptr [rsi]
0x14234b35d mov      dl, 1
0x14234b35f call     0x1408aea90
0x14234b364 mov      eax, dword ptr [rdi + 0x1c]
0x14234b367 mov      rcx, qword ptr [rdi + 0x10]
0x14234b36b mov      dword ptr [rcx], eax
0x14234b36d mov      rcx, qword ptr [rdi]
0x14234b370 call     0x1408a8140
0x14234b375 mov      eax, dword ptr [r15 + 0x1c]
0x14234b379 mov      rcx, qword ptr [r15 + 0x10]
0x14234b37d mov      dword ptr [rcx], eax
0x14234b37f mov      rax, qword ptr [r15]
0x14234b382 mov      byte ptr [rax + 1], 0
0x14234b386 mov      rax, qword ptr [r15]
0x14234b389 mov      rbx, qword ptr [rbp + 0x240]
0x14234b390 mov      rcx, qword ptr [rbx]
0x14234b393 mov      qword ptr [rbp + 0x1f8], rcx
0x14234b39a pxor     xmm3, xmm3
0x14234b39e cmp      byte ptr [rax + 1], 1
0x14234b3a2 mov      rsi, qword ptr [rbp + 0x208]
0x14234b3a9 jne      0x14234b3c3
0x14234b3ab mov      rcx, qword ptr [rbp + 0x1f8]
0x14234b3b2 cmp      byte ptr [rcx + 0x103], 3
0x14234b3b9 je       0x14234b3c3
0x14234b3bb movd     xmm3, dword ptr [rip + 0x5da541]
0x14234b3c3 mov      qword ptr [rbp + 0x1a8], r12
0x14234b3ca mov      qword ptr [rbp + 0x1e8], rax
0x14234b3d1 mov      qword ptr [rbp + 0x180], r15
0x14234b3d8 lea      rdx, [rip + 0xd03dc4] ; @0x14304f1a3 'PhotoModeMinusFPVcamera_focus_canvas'
0x14234b3df mov      r8d, 0x11
0x14234b3e5 mov      rcx, qword ptr [rbp + 0x220]
0x14234b3ec call     0x140a55f20
0x14234b3f1 mov      rcx, qword ptr [rsi]
0x14234b3f4 xor      eax, eax
0x14234b3f6 mov      rdx, rcx
0x14234b3f9 mov      r8, qword ptr [rbp + 0x200]
0x14234b400 sub      rdx, qword ptr [r8 + 0x18]
0x14234b404 cmovb    rdx, rax
0x14234b408 sub      rcx, qword ptr [r8 + 0x38]
0x14234b40c cmovb    rcx, rax
0x14234b410 mov      r15, rdx
0x14234b413 shl      r15, 4
0x14234b417 add      r15, qword ptr [r8 + 8]
0x14234b41b mov      r12, qword ptr [r8 + 0x10]
0x14234b41f mov      r9, qword ptr [r8 + 0x30]
0x14234b423 sub      r12, rdx
0x14234b426 cmovb    r12, rax
0x14234b42a mov      edx, 8
0x14234b42f cmovb    r15, rdx
0x14234b433 mov      rdi, rcx
0x14234b436 shl      rdi, 4
0x14234b43a add      rdi, qword ptr [r8 + 0x28]
0x14234b43e sub      r9, rcx
0x14234b441 cmovb    r9, rax
0x14234b445 cmovb    rdi, rdx
0x14234b449 mov      rax, qword ptr [r8 + 0x40]
0x14234b44d sub      rax, r9
0x14234b450 sub      rax, r12
0x14234b453 mov      qword ptr [rsi], rax
0x14234b456 shl      r12, 4
0x14234b45a add      r12, r15
0x14234b45d shl      r9, 4
0x14234b461 add      r9, rdi
0x14234b464 mov      qword ptr [rbp + 0x200], r9
0x14234b46b mov      rcx, qword ptr [rbp + 0x218]
0x14234b472 mov      eax, dword ptr [rcx + 0x1c]
0x14234b475 mov      dword ptr [rbp + 0x254], eax
0x14234b47b mov      rax, qword ptr [rcx]
0x14234b47e mov      qword ptr [rbp + 0x258], rax
0x14234b485 mov      rax, qword ptr [rcx + 0x10]
0x14234b489 mov      qword ptr [rbp + 0x228], rax
0x14234b490 mov      eax, dword ptr [rbx + 0x1c]
0x14234b493 mov      dword ptr [rbp + 0x218], eax
0x14234b499 mov      rax, qword ptr [rbx + 0x10]
0x14234b49d mov      qword ptr [rbp + 0x240], rax
0x14234b4a4 lea      rax, [r13 + 0x20]
0x14234b4a8 mov      qword ptr [rbp + 0x220], rax
0x14234b4af jmp      0x14234b51a
0x14234b4b1 nop      word ptr cs:[rax + rax]
0x14234b4c0 mov      rax, qword ptr [r13 + 0x28]
0x14234b4c4 mov      rcx, rsi
0x14234b4c7 shl      rcx, 4
0x14234b4cb mov      qword ptr [rax + rcx], r14
0x14234b4cf mov      byte ptr [rax + rcx + 8], 1
0x14234b4d4 inc      rsi
0x14234b4d7 mov      qword ptr [r13 + 0x30], rsi
0x14234b4db inc      qword ptr [r13 + 0x40]
0x14234b4df mov      r14, qword ptr [rbp + 0x248]
0x14234b4e6 mov      rsi, qword ptr [rbp + 0x208]
0x14234b4ed movzx    ecx, byte ptr [rbx + 8]
0x14234b4f1 call     0x14090e500
0x14234b4f6 mov      ecx, dword ptr [rbp + 0x218]
0x14234b4fc mov      rdx, qword ptr [rbp + 0x240]
0x14234b503 mov      dword ptr [rdx], ecx
0x14234b505 mov      rcx, qword ptr [rbp + 0x1f8]
0x14234b50c mov      byte ptr [rcx + 0x103], al
0x14234b512 movzx    eax, byte ptr [rbx + 8]
0x14234b516 mov      byte ptr [r14 + 9], al
0x14234b51a test     r15, r15
0x14234b51d je       0x14234b530
0x14234b51f cmp      r15, r12
0x14234b522 je       0x14234b530
0x14234b524 lea      rax, [r15 + 0x10]
0x14234b528 mov      rbx, r15
0x14234b52b jmp      0x14234b54f
0x14234b52d nop      dword ptr [rax]
0x14234b530 test     rdi, rdi
0x14234b533 je       0x14234b653
0x14234b539 cmp      rdi, qword ptr [rbp + 0x200]
0x14234b540 je       0x14234b653
0x14234b546 mov      rbx, rdi
0x14234b549 add      rdi, 0x10
0x14234b54d xor      eax, eax
0x14234b54f inc      qword ptr [rsi]
0x14234b552 mov      r15, rax
0x14234b555 movzx    eax, byte ptr [rbx + 8]
0x14234b559 cmp      al, 4
0x14234b55b je       0x14234b51a
0x14234b55d mov      rcx, qword ptr [rbp + 0x1f8]
0x14234b564 cmp      byte ptr [rcx + 0x103], 3
0x14234b56b jne      0x14234b5b0
0x14234b56d mov      rax, qword ptr [rbp + 0x1e0]
0x14234b574 mov      ecx, dword ptr [rbp + 0x23c]
0x14234b57a mov      dword ptr [rax], ecx
0x14234b57c mov      rsi, qword ptr [r13 + 0x30]
0x14234b580 mov      r14, qword ptr [r13 + 0x40]
0x14234b584 cmp      rsi, qword ptr [r13 + 0x20]
0x14234b588 jne      0x14234b4c0
0x14234b58e mov      rcx, qword ptr [rbp + 0x220]
0x14234b595 lea      rdx, [rip + 0x869cd4]
0x14234b59c call     0x140562630
0x14234b5a1 jmp      0x14234b4c0
0x14234b5a6 nop      word ptr cs:[rax + rax]
0x14234b5b0 cmp      al, 2
0x14234b5b2 jne      0x14234b4ed
0x14234b5b8 mov      rax, qword ptr [rbp + 0x1e0]
0x14234b5bf mov      ecx, dword ptr [rbp + 0x23c]
0x14234b5c5 mov      dword ptr [rax], ecx
0x14234b5c7 mov      rsi, qword ptr [r13 + 0x30]
0x14234b5cb mov      r14, qword ptr [r13 + 0x40]
0x14234b5cf cmp      rsi, qword ptr [r13 + 0x20]
0x14234b5d3 jne      0x14234b5e8
0x14234b5d5 mov      rcx, qword ptr [rbp + 0x220]
0x14234b5dc lea      rdx, [rip + 0x869c8d]
0x14234b5e3 call     0x140562630
0x14234b5e8 mov      rax, qword ptr [r13 + 0x28]
0x14234b5ec mov      rcx, rsi
0x14234b5ef shl      rcx, 4
0x14234b5f3 mov      qword ptr [rax + rcx], r14
0x14234b5f7 mov      byte ptr [rax + rcx + 8], 0
0x14234b5fc inc      rsi
0x14234b5ff mov      qword ptr [r13 + 0x30], rsi
0x14234b603 inc      qword ptr [r13 + 0x40]
0x14234b607 mov      r14, qword ptr [rbp + 0x248]
0x14234b60e cmp      byte ptr [r14 + 0xc], 0
0x14234b613 mov      rsi, qword ptr [rbp + 0x208]
0x14234b61a jne      0x14234b4ed
0x14234b620 mov      eax, dword ptr [rbp + 0x254]
0x14234b626 mov      rcx, qword ptr [rbp + 0x228]
0x14234b62d mov      dword ptr [rcx], eax
0x14234b62f mov      rcx, qword ptr [rbp + 0x258]
0x14234b636 mov      rdx, qword ptr [rbp + 0x170]
0x14234b63d mov      r8, qword ptr [rbp + 0x178]
0x14234b644 call     0x14090e510
0x14234b649 mov      byte ptr [r14 + 0xc], 1
0x14234b64e jmp      0x14234b4ed
0x14234b653 mov      rax, qword ptr [rbp + 0x1e8]
0x14234b65a cmp      byte ptr [rax + 1], 0
0x14234b65e je       0x14234b669
0x14234b660 movzx    esi, byte ptr [rax]
0x14234b663 xor      sil, 1
0x14234b667 jmp      0x14234b66b
0x14234b669 xor      esi, esi
0x14234b66b mov      rax, qword ptr [rbp + 0x230]
0x14234b672 mov      rax, qword ptr [rax]
0x14234b675 xor      r10d, r10d
0x14234b678 mov      rcx, rax
0x14234b67b mov      r15, qword ptr [rbp + 0x188]
0x14234b682 sub      rcx, qword ptr [r15 + 0x18]
0x14234b686 cmovb    rcx, r10
0x14234b68a sub      rax, qword ptr [r15 + 0x38]
0x14234b68e cmovb    rax, r10
0x14234b692 mov      r9, qword ptr [r15 + 8]
0x14234b696 mov      r8, qword ptr [r15 + 0x30]
0x14234b69a mov      rdx, r8
0x14234b69d sub      rdx, rax
0x14234b6a0 cmovb    rdx, r10
0x14234b6a4 mov      rbx, qword ptr [r15 + 0x10]
0x14234b6a8 setbe    r14b
0x14234b6ac mov      rdi, rbx
0x14234b6af sub      rdi, rcx
0x14234b6b2 cmovb    rdi, r10
0x14234b6b6 mov      r11, qword ptr [r15 + 0x28]
0x14234b6ba mov      r10, qword ptr [r15 + 0x40]
0x14234b6be setbe    r15b
0x14234b6c2 mov      r13, r10
0x14234b6c5 sub      r13, rdx
0x14234b6c8 sub      r13, rdi
0x14234b6cb mov      r12, qword ptr [rbp + 0x230]
0x14234b6d2 mov      qword ptr [r12], r13
0x14234b6d6 mov      r13, qword ptr [rbp + 0x230]
0x14234b6dd test     r15b, r14b
0x14234b6e0 mov      r15, qword ptr [rbp + 0x180]
0x14234b6e7 mov      r12, qword ptr [rbp + 0x1e8]
0x14234b6ee jne      0x14234b7b0
0x14234b6f4 mov      r14, rax
0x14234b6f7 shl      r14, 4
0x14234b6fb add      r11, r14
0x14234b6fe shl      rdx, 4
0x14234b702 mov      r14, rcx
0x14234b705 shl      r14, 4
0x14234b709 add      r9, r14
0x14234b70c shl      rdi, 4
0x14234b710 xor      r14d, r14d
0x14234b713 cmp      rbx, rcx
0x14234b716 mov      ecx, 8
0x14234b71b cmovb    r9, rcx
0x14234b71f lea      r9, [r9 + rdi - 0x10]
0x14234b724 cmova    r14, r9
0x14234b728 cmp      r8, rax
0x14234b72b cmovb    r11, rcx
0x14234b72f lea      rax, [r11 + rdx - 0x10]
0x14234b734 cmovbe   rax, r14
0x14234b738 mov      qword ptr [r13], r10
0x14234b73c cmp      byte ptr [rax + 8], 1
0x14234b740 jne      0x14234b7b0
0x14234b742 mov      rax, qword ptr [rbp + 0x248]
0x14234b749 mov      byte ptr [rax + 0xa], 0
0x14234b74d mov      eax, dword ptr [r15 + 0x1c]
0x14234b751 mov      rcx, qword ptr [r15 + 0x10]
0x14234b755 mov      dword ptr [rcx], eax
0x14234b757 mov      byte ptr [r12 + 1], 1
0x14234b75d mov      rax, qword ptr [rbp + 0x1b8]
0x14234b764 mov      ecx, dword ptr [rbp + 0x1f0]
0x14234b76a mov      dword ptr [rax], ecx
0x14234b76c mov      r14, qword ptr [rbp + 0x1b0]
0x14234b773 mov      rdi, qword ptr [r14 + 0x30]
0x14234b777 mov      rbx, qword ptr [r14 + 0x40]
0x14234b77b cmp      rdi, qword ptr [r14 + 0x20]
0x14234b77f jne      0x14234b791
0x14234b781 lea      rcx, [r14 + 0x20]
0x14234b785 lea      rdx, [rip + 0x869ae4]
0x14234b78c call     0x140562db0
0x14234b791 mov      rax, qword ptr [r14 + 0x28]
0x14234b795 lea      rcx, [rdi + rdi*2]
0x14234b799 mov      dword ptr [rax + rcx*8], 0x13
0x14234b7a0 mov      qword ptr [rax + rcx*8 + 0x10], rbx
0x14234b7a5 inc      rdi
0x14234b7a8 mov      qword ptr [r14 + 0x30], rdi
0x14234b7ac inc      qword ptr [r14 + 0x40]
0x14234b7b0 movzx    eax, byte ptr [r12 + 1]
0x14234b7b6 cmp      al, 1
0x14234b7b8 jne      0x14234c104
0x14234b7be test     sil, sil
0x14234b7c1 je       0x14234b86d
0x14234b7c7 mov      rax, qword ptr [rbp + 0x1a8]
0x14234b7ce cmp      dword ptr [rax], 0xd
0x14234b7d1 jne      0x14234b829
0x14234b7d3 mov      rax, qword ptr [rbp + 0x1d8]
0x14234b7da mov      ecx, dword ptr [rbp + 0x1f4]
0x14234b7e0 mov      dword ptr [rax], ecx
0x14234b7e2 mov      rbx, qword ptr [rbp + 0x1d0]
0x14234b7e9 mov      rsi, qword ptr [rbx + 0x30]
0x14234b7ed mov      rdi, qword ptr [rbx + 0x40]
0x14234b7f1 cmp      rsi, qword ptr [rbx + 0x20]
0x14234b7f5 jne      0x14234b807
0x14234b7f7 lea      rcx, [rbx + 0x20]
0x14234b7fb lea      rdx, [rip + 0x869a6e]
0x14234b802 call     0x140562630
0x14234b807 mov      rax, qword ptr [rbx + 0x28]
0x14234b80b mov      rcx, rsi
0x14234b80e shl      rcx, 4
0x14234b812 mov      qword ptr [rax + rcx], rdi
0x14234b816 mov      dword ptr [rax + rcx + 8], 0x3000300
0x14234b81e inc      rsi
0x14234b821 mov      qword ptr [rbx + 0x30], rsi
0x14234b825 inc      qword ptr [rbx + 0x40]
0x14234b829 mov      rdx, qword ptr [rbp + 0x358]
0x14234b830 mov      eax, dword ptr [rdx + 0x1c]
0x14234b833 mov      rcx, qword ptr [rdx + 0x10]
0x14234b837 mov      dword ptr [rcx], eax
0x14234b839 mov      rsi, qword ptr [rdx]
0x14234b83c mov      rdi, qword ptr [rsi + 0x30]
0x14234b840 mov      rbx, qword ptr [rsi + 0x40]
0x14234b844 cmp      rdi, qword ptr [rsi + 0x20]
0x14234b848 jne      0x14234b85a
0x14234b84a lea      rcx, [rsi + 0x20]
0x14234b84e lea      rdx, [rip + 0x87be53]
0x14234b855 call     0x140562870
0x14234b85a mov      rax, qword ptr [rsi + 0x28]
0x14234b85e mov      qword ptr [rax + rdi*8], rbx
0x14234b862 inc      rdi
0x14234b865 mov      qword ptr [rsi + 0x30], rdi
0x14234b869 inc      qword ptr [rsi + 0x40]
0x14234b86d mov      rax, qword ptr [rbp + 0x210]
0x14234b874 movss    xmm0, dword ptr [rax + 0x160]
0x14234b87c xorps    xmm1, xmm1
0x14234b87f ucomiss  xmm0, xmm1
0x14234b882 mov      r14, qword ptr [rbp + 0x248]
0x14234b889 jne      0x14234b88d
0x14234b88b jnp      0x14234b90a
0x14234b88d mov      rcx, qword ptr [rbp + 0x258]
0x14234b894 call     0x140911230
0x14234b899 cmp      al, 4
0x14234b89b je       0x14234b90a
0x14234b89d movzx    eax, al
0x14234b8a0 cmp      eax, 1
0x14234b8a3 je       0x14234b90a
0x14234b8a5 mov      rax, qword ptr [rbp + 0x210]
0x14234b8ac movss    xmm8, dword ptr [rax + 0x160]
0x14234b8b5 mov      eax, dword ptr [rbp + 0x254]
0x14234b8bb mov      rcx, qword ptr [rbp + 0x228]
0x14234b8c2 mov      dword ptr [rcx], eax
0x14234b8c4 mov      rcx, qword ptr [rbp + 0x258]
0x14234b8cb call     0x140911260
0x14234b8d0 movss    xmm7, dword ptr [rax + 0xc]
0x14234b8d5 movss    xmm6, dword ptr [rax + 0x10]
0x14234b8da ucomiss  xmm6, xmm7
0x14234b8dd jb       0x14234c17c
0x14234b8e3 mov      rdi, rax
0x14234b8e6 mulss    xmm8, dword ptr [rip + 0xd0381d]
0x14234b8ef movaps   xmm0, xmm8
0x14234b8f3 call     0x142923eb0
0x14234b8f8 mulss    xmm0, dword ptr [rdi + 8]
0x14234b8fd maxss    xmm7, xmm0
0x14234b901 minss    xmm6, xmm7
0x14234b905 movss    dword ptr [rdi + 8], xmm6
0x14234b90a mov      rax, qword ptr [rbp + 0x350]
0x14234b911 mov      rdi, qword ptr [rax]
0x14234b914 mov      dword ptr [rbp + 0x14], 0x7f7fffff
0x14234b91b mov      byte ptr [rbp + 0x10], 5
0x14234b91f mov      byte ptr [rbp - 0x50], 0
0x14234b923 lea      rbx, [rbp + 0xc0]
0x14234b92a lea      rdx, [rbp + 0x10]
0x14234b92e lea      r8, [rbp - 0x50]
0x14234b932 mov      rcx, rbx
0x14234b935 call     0x1409a8e10
0x14234b93a lea      rcx, [rbp + 0x10]
0x14234b93e mov      rdx, rbx
0x14234b941 xor      r8d, r8d
0x14234b944 call     0x1409a8ee0
0x14234b949 mov      byte ptr [rbp + 0x267], 1
0x14234b950 lea      rcx, [rbp - 0x50]
0x14234b954 call     0x1409a8fb0
0x14234b959 mov      byte ptr [rbp + 0x267], 0
0x14234b960 lea      rcx, [rbp + 0xc0]
0x14234b967 lea      rdx, [rbp + 0x10]
0x14234b96b lea      r8, [rbp - 0x50]
0x14234b96f call     0x1409a8e80
0x14234b974 lea      rdx, [rip + 0xd03839] ; @0x14304f1b4 'camera_focus_canvas'
0x14234b97b lea      r9, [rbp + 0xc0]
0x14234b982 mov      r8d, 0x13
0x14234b988 mov      rcx, rdi
0x14234b98b call     0x1409236d0
0x14234b990 mov      qword ptr [rbp - 0x50], rax
0x14234b994 mov      qword ptr [rbp - 0x48], rdx
0x14234b998 mov      rcx, qword ptr [rbp + 0x258]
0x14234b99f call     0x140911230
0x14234b9a4 mov      ebx, eax
0x14234b9a6 mov      dword ptr [r14 + 4], 0xffffffff
0x14234b9ae lea      rdx, [rip + 0xd0a3c3]
0x14234b9b5 mov      rcx, qword ptr [rbp + 0x190]
0x14234b9bc call     0x1423a3280
0x14234b9c1 test     rax, rax
0x14234b9c4 je       0x14234c18e
0x14234b9ca mov      rdi, rax
0x14234b9cd lea      rax, [rip + 0xc4f06c] ; @0x142f9aa40 'country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbocountry_c'
0x14234b9d4 mov      qword ptr [rbp + 0x10], rax
0x14234b9d8 mov      qword ptr [rbp + 0x18], 0x57
0x14234b9e0 cmp      qword ptr [rdi + 0x18], 0x57
0x14234b9e5 jne      0x14234c14c
0x14234b9eb mov      rdx, qword ptr [rdi + 0x10]
0x14234b9ef lea      rcx, [rip + 0xc4f04a] ; @0x142f9aa40 'country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbocountry_c'
0x14234b9f6 mov      r8d, 0x57
0x14234b9fc call     0x142923bf0
0x14234ba01 test     eax, eax
0x14234ba03 jne      0x14234c14c
0x14234ba09 mov      rcx, qword ptr [rdi]
0x14234ba0c mov      rax, qword ptr [rdi + 8]
0x14234ba10 call     qword ptr [rax + 0x18]
0x14234ba13 test     rax, rax
0x14234ba16 je       0x14234c18e
0x14234ba1c cmp      qword ptr [rax + 8], 0
0x14234ba21 je       0x14234c19a
0x14234ba27 movzx    esi, bl
0x14234ba2a mov      rdi, qword ptr [rax]
0x14234ba2d mov      ebx, dword ptr [rdi + 0xc]
0x14234ba30 mov      eax, dword ptr [rbp + 0x254]
0x14234ba36 mov      rcx, qword ptr [rbp + 0x228]
0x14234ba3d mov      dword ptr [rcx], eax
0x14234ba3f mov      rcx, qword ptr [rbp + 0x258]
0x14234ba46 call     0x140911240
0x14234ba4b mov      ecx, dword ptr [rax]
0x14234ba4d cmp      ecx, 2
0x14234ba50 jae      0x14234ba99
0x14234ba52 cmp      ebx, esi
0x14234ba54 setne    dl
0x14234ba57 test     ebx, ebx
0x14234ba59 sete     r8b
0x14234ba5d or       r8b, dl
0x14234ba60 je       0x14234ba6c
0x14234ba62 test     ecx, ecx
0x14234ba64 jne      0x14234ba99
0x14234ba66 mov      dword ptr [r14 + 4], esi
0x14234ba6a jmp      0x14234ba99
0x14234ba6c mov      ecx, dword ptr [rdi + 8]
0x14234ba6f mov      dword ptr [rbp + 0xc8], ecx
0x14234ba75 mov      rcx, qword ptr [rdi]
0x14234ba78 mov      qword ptr [rbp + 0xc0], rcx
0x14234ba7f mov      dword ptr [rax], 1
0x14234ba85 mov      rcx, qword ptr [rbp + 0xc0]
0x14234ba8c mov      qword ptr [rax + 4], rcx
0x14234ba90 mov      ecx, dword ptr [rbp + 0xc8]
0x14234ba96 mov      dword ptr [rax + 0xc], ecx
0x14234ba99 mov      rax, qword ptr [rbp + 0x1a0]
0x14234baa0 movss    xmm10, dword ptr [rax]
0x14234baa5 movss    xmm9, dword ptr [rax + 4]
0x14234baab mov      rbx, qword ptr [rbp + 0x198]
0x14234bab2 mov      rcx, rbx
0x14234bab5 call     0x140949c10
0x14234baba movaps   xmm8, xmm0
0x14234babe movaps   xmm7, xmm1
0x14234bac1 mov      rcx, rbx
0x14234bac4 call     0x140949c20
0x14234bac9 movaps   xmm6, xmm0
0x14234bacc mov      eax, dword ptr [rbp + 0x254]
0x14234bad2 mov      rcx, qword ptr [rbp + 0x228]
0x14234bad9 mov      dword ptr [rcx], eax
0x14234badb mov      rcx, qword ptr [rbp + 0x258]
0x14234bae2 call     0x140911240
0x14234bae7 mov      rdi, rax
0x14234baea mov      ecx, dword ptr [rax]
0x14234baec add      ecx, -2
0x14234baef cmp      ecx, 2
0x14234baf2 mov      eax, 2
0x14234baf7 cmovb    eax, ecx
0x14234bafa test     eax, eax
0x14234bafc je       0x14234bb70
0x14234bafe divss    xmm10, xmm8
0x14234bb03 divss    xmm9, xmm7
0x14234bb08 addss    xmm10, xmm10
0x14234bb0d addss    xmm9, xmm9
0x14234bb12 movss    xmm0, dword ptr [rip + 0x60e5ca]
0x14234bb1a addss    xmm10, xmm0
0x14234bb1f addss    xmm9, xmm0
0x14234bb24 mulss    xmm6, xmm10
0x14234bb29 xorps    xmm9, xmmword ptr [rip + 0x5d9dbf]
0x14234bb31 lea      rcx, [rbp - 0x50]
0x14234bb35 cmp      eax, 1
0x14234bb38 jne      0x14234bb87
0x14234bb3a call     0x14092df10
0x14234bb3f add      al, 0xfe
0x14234bb41 movzx    eax, al
0x14234bb44 cmp      al, 3
0x14234bb46 mov      ecx, 3
0x14234bb4b cmovb    ecx, eax
0x14234bb4e dec      cl
0x14234bb50 cmp      cl, 2
0x14234bb53 jae      0x14234bbc7
0x14234bb55 movss    dword ptr [rdi + 4], xmm6
0x14234bb5a movss    dword ptr [rdi + 8], xmm9
0x14234bb60 cmp      byte ptr [r14 + 0xd], 0
0x14234bb65 jne      0x14234bc93
0x14234bb6b jmp      0x14234bfb2
0x14234bb70 xorps    xmm6, xmm6
0x14234bb73 xorps    xmm9, xmm9
0x14234bb77 cmp      byte ptr [r14 + 0xd], 0
0x14234bb7c jne      0x14234bc93
0x14234bb82 jmp      0x14234bfb2
0x14234bb87 call     0x14092df10
0x14234bb8c add      al, 0xfe
0x14234bb8e movzx    eax, al
0x14234bb91 cmp      al, 3
0x14234bb93 mov      ecx, 3
0x14234bb98 cmovb    ecx, eax
0x14234bb9b dec      cl
0x14234bb9d cmp      cl, 2
0x14234bba0 jae      0x14234bbe2
0x14234bba2 mov      dword ptr [r14 + 4], esi
0x14234bba6 movss    dword ptr [rdi + 0x10], xmm6
0x14234bbab movss    dword ptr [rdi + 0x14], xmm9
0x14234bbb1 mov      dword ptr [rdi], 0
0x14234bbb7 cmp      byte ptr [r14 + 0xd], 0
0x14234bbbc jne      0x14234bc93
0x14234bbc2 jmp      0x14234bfb2
0x14234bbc7 movss    xmm6, dword ptr [rdi + 4]
0x14234bbcc movss    xmm9, dword ptr [rdi + 8]
0x14234bbd2 cmp      byte ptr [r14 + 0xd], 0
0x14234bbd7 jne      0x14234bc93
0x14234bbdd jmp      0x14234bfb2
0x14234bbe2 test     byte ptr [rdi], 1
0x14234bbe5 je       0x14234bc7d
0x14234bbeb movups   xmm0, xmmword ptr [rdi + 4]
0x14234bbef shufps   xmm0, xmm0, 0
0x14234bbf3 mov      rax, qword ptr [rbp + 0x258]
0x14234bbfa mulps    xmm0, xmmword ptr [rax + 0x80]
0x14234bc01 movups   xmm1, xmmword ptr [rdi + 8]
0x14234bc05 shufps   xmm1, xmm1, 0
0x14234bc09 mulps    xmm1, xmmword ptr [rax + 0x90]
0x14234bc10 movups   xmm7, xmmword ptr [rdi + 0xc]
0x14234bc14 shufps   xmm7, xmm7, 0
0x14234bc18 mulps    xmm7, xmmword ptr [rax + 0xa0]
0x14234bc1f addps    xmm1, xmm0
0x14234bc22 addps    xmm7, xmm1
0x14234bc25 addps    xmm7, xmmword ptr [rax + 0xb0]
0x14234bc2c movaps   xmm0, xmm7
0x14234bc2f shufps   xmm0, xmm7, 0
0x14234bc33 mulps    xmm0, xmmword ptr [rax]
0x14234bc36 movaps   xmm1, xmm7
0x14234bc39 shufps   xmm1, xmm7, 0x55
0x14234bc3d mulps    xmm1, xmmword ptr [rax + 0x10]
0x14234bc41 shufps   xmm7, xmm7, 0xaa
0x14234bc45 mulps    xmm7, xmmword ptr [rax + 0x20]
0x14234bc49 addps    xmm1, xmm0
0x14234bc4c addps    xmm7, xmm1
0x14234bc4f addps    xmm7, xmmword ptr [rax + 0x30]
0x14234bc53 movaps   xmm0, xmm7
0x14234bc56 shufps   xmm0, xmm7, 0xff
0x14234bc5a divps    xmm7, xmm0
0x14234bc5d movshdup xmm9, xmm7
0x14234bc62 mov      rcx, rbx
0x14234bc65 call     0x140949c20
0x14234bc6a movaps   xmm6, xmm0
0x14234bc6d mulss    xmm6, xmm7
0x14234bc71 cmp      byte ptr [r14 + 0xd], 0
0x14234bc76 jne      0x14234bc93
0x14234bc78 jmp      0x14234bfb2
0x14234bc7d movss    xmm6, dword ptr [rdi + 0x10]
0x14234bc82 movss    xmm9, dword ptr [rdi + 0x14]
0x14234bc88 cmp      byte ptr [r14 + 0xd], 0
0x14234bc8d je       0x14234bfb2
0x14234bc93 mov      rcx, qword ptr [rbp + 0x1c0]
0x14234bc9a call     0x14093fbe0
0x14234bc9f test     al, al
0x14234bca1 je       0x14234bfb2
0x14234bca7 mov      rcx, qword ptr [rbp + 0x258]
0x14234bcae call     0x140911230
0x14234bcb3 add      al, 0xfd
0x14234bcb5 cmp      al, 2
0x14234bcb7 jb       0x14234bfb2
0x14234bcbd mov      rcx, rbx
0x14234bcc0 call     0x140949c20
0x14234bcc5 movaps   xmm7, xmm0
0x14234bcc8 mov      rcx, qword ptr [rbp + 0x258]
0x14234bccf call     0x140911240
0x14234bcd4 movss    xmm8, dword ptr [rax + 0x18]
0x14234bcda mov      rdx, qword ptr [rbp + 0x330]
0x14234bce1 mov      eax, dword ptr [rdx + 0x1c]
0x14234bce4 mov      rcx, qword ptr [rdx + 0x10]
0x14234bce8 mov      dword ptr [rcx], eax
0x14234bcea mov      rdi, qword ptr [rdx]
0x14234bced lea      rax, [rip + 0xd03504] ; @0x14304f1f8 'camera focusother/camera_focus_pinnedother/camera_focuscamera ghostother/camera_ghost/rustc/05f9'
0x14234bcf4 mov      qword ptr [rbp + 0xc0], rax
0x14234bcfb mov      qword ptr [rbp + 0xc8], 0xc
0x14234bd06 lea      rcx, [rbp + 0xc0]
0x14234bd0d call     0x1405bdda0
0x14234bd12 movabs   rbx, 0xb1695320004fb443
0x14234bd1c xor      rbx, rax
0x14234bd1f mov      rcx, rdi
0x14234bd22 mov      rdx, rbx
0x14234bd25 call     0x140683ec0
0x14234bd2a mov      byte ptr [rax + 0x5d], 0
0x14234bd2e test     dl, 1
0x14234bd31 jne      0x14234bdb1
0x14234bd33 addss    xmm6, xmm7
0x14234bd37 movss    xmm0, dword ptr [rip + 0x5d9bc5]
0x14234bd3f subss    xmm0, xmm9
0x14234bd44 unpcklps xmm6, xmm0
0x14234bd47 mulps    xmm6, xmmword ptr [rip + 0x5d9b92]
0x14234bd4e mulps    xmm6, xmmword ptr [rip + 0xc3c18b]
0x14234bd55 mulss    xmm8, dword ptr [rip + 0x60f8d2]
0x14234bd5e mov      rsi, qword ptr [rdi + 0x10]
0x14234bd62 cmp      rsi, qword ptr [rdi]
0x14234bd65 jne      0x14234bd76
0x14234bd67 lea      rdx, [rip + 0x86325a]
0x14234bd6e mov      rcx, rdi
0x14234bd71 call     0x1406319d0
0x14234bd76 mov      rax, qword ptr [rdi + 8]
0x14234bd7a imul     rcx, rsi, 0x38
0x14234bd7e mov      dword ptr [rax + rcx], 4
0x14234bd85 movq     xmm0, xmm6
0x14234bd89 movdqu   xmmword ptr [rax + rcx + 4], xmm0
0x14234bd8f mov      dword ptr [rax + rcx + 0x14], 1
0x14234bd97 movss    dword ptr [rax + rcx + 0x18], xmm8
0x14234bd9e movss    dword ptr [rax + rcx + 0x1c], xmm8
0x14234bda5 mov      qword ptr [rax + rcx + 0x30], rbx
0x14234bdaa inc      rsi
0x14234bdad mov      qword ptr [rdi + 0x10], rsi
0x14234bdb1 mov      qword ptr [rbp + 0x10], rdi
0x14234bdb5 mov      qword ptr [rbp + 0x18], rbx
0x14234bdb9 mov      qword ptr [rbp + 0x20], 0
0x14234bdc1 mov      rcx, qword ptr [rbp + 0x258]
0x14234bdc8 call     0x140911240
0x14234bdcd cmp      dword ptr [rax], 2
0x14234bdd0 lea      rax, [rip + 0xd0342d] ; @0x14304f204 'other/camera_focus_pinnedother/camera_focuscamera ghostother/camera_ghost/rustc/05f9846f893b09a1'
0x14234bdd7 lea      r8, [rip + 0xd0343f] ; @0x14304f21d 'other/camera_focuscamera ghostother/camera_ghost/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\'
0x14234bdde cmovb    r8, rax
0x14234bde2 mov      eax, 0x19
0x14234bde7 mov      r9d, 0x12
0x14234bded cmovb    r9, rax
0x14234bdf1 mov      qword ptr [rsp + 0x20], 0
0x14234bdfa lea      rcx, [rbp + 0xc0]
0x14234be01 lea      rdx, [rbp + 0x10]
0x14234be05 call     0x140f9e970
0x14234be0a mov      rsi, qword ptr [rbp + 0xd0]
0x14234be11 cmp      qword ptr [rsi + 0x1b8], 0
0x14234be19 je       0x14234bf9e
0x14234be1f lea      rdi, [rbp + 0xd8]
0x14234be26 lea      rcx, [rsi + 0x1c0]
0x14234be2d mov      rdx, rdi
0x14234be30 call     0x1405b0d70
0x14234be35 mov      rcx, qword ptr [rsi + 0x1a0]
0x14234be3c mov      rdx, qword ptr [rsi + 0x1a8]
0x14234be43 mov      r8, rdx
0x14234be46 and      r8, rax
0x14234be49 shr      rax, 0x39
0x14234be4d movd     xmm0, eax
0x14234be51 punpcklbw xmm0, xmm0
0x14234be55 pshuflw  xmm0, xmm0, 0
0x14234be5a pshufd   xmm0, xmm0, 0
0x14234be5f lea      rax, [rcx - 0xc0]
0x14234be66 xor      r9d, r9d
0x14234be69 mov      r10, qword ptr [rbp + 0xd8]
0x14234be70 pcmpeqd  xmm1, xmm1
0x14234be74 movdqu   xmm2, xmmword ptr [rcx + r8]
0x14234be7a movdqa   xmm3, xmm2
0x14234be7e pcmpeqb  xmm3, xmm0
0x14234be82 pmovmskb r11d, xmm3
0x14234be87 test     r11d, r11d
0x14234be8a je       0x14234beb5
0x14234be8c tzcnt    esi, r11d
0x14234be91 add      rsi, r8
0x14234be94 and      rsi, rdx
0x14234be97 neg      rsi
0x14234be9a lea      rsi, [rsi + rsi*2]
0x14234be9e shl      rsi, 6
0x14234bea2 cmp      r10, qword ptr [rax + rsi]
0x14234bea6 je       0x14234bed7
0x14234bea8 lea      esi, [r11 - 1]
0x14234beac and      si, r11w
0x14234beb0 mov      r11d, esi
0x14234beb3 jne      0x14234be8c
0x14234beb5 pcmpeqb  xmm2, xmm1
0x14234beb9 pmovmskb r11d, xmm2
0x14234bebe test     r11d, r11d
0x14234bec1 jne      0x14234bf9e
0x14234bec7 add      r8, r9
0x14234beca add      r8, 0x10
0x14234bece add      r9, 0x10
0x14234bed2 and      r8, rdx
0x14234bed5 jmp      0x14234be74
0x14234bed7 or       byte ptr [rcx + rsi - 1], 2
0x14234bedc mov      rsi, qword ptr [rbp + 0xd0]
0x14234bee3 cmp      qword ptr [rsi + 0x1b8], 0
0x14234beeb je       0x14234bf9e
0x14234bef1 lea      rcx, [rsi + 0x1c0]
0x14234bef8 mov      rdx, rdi
0x14234befb call     0x1405b0d70
0x14234bf00 mov      rcx, qword ptr [rsi + 0x1a0]
0x14234bf07 mov      rdx, qword ptr [rsi + 0x1a8]
0x14234bf0e mov      r8, rdx
0x14234bf11 and      r8, rax
0x14234bf14 shr      rax, 0x39
0x14234bf18 movd     xmm0, eax
0x14234bf1c punpcklbw xmm0, xmm0
0x14234bf20 pshuflw  xmm0, xmm0, 0
0x14234bf25 pshufd   xmm0, xmm0, 0
0x14234bf2a lea      rax, [rcx - 0xc0]
0x14234bf31 xor      r9d, r9d
0x14234bf34 mov      r10, qword ptr [rbp + 0xd8]
0x14234bf3b pcmpeqd  xmm1, xmm1
0x14234bf3f movdqu   xmm2, xmmword ptr [rcx + r8]
0x14234bf45 movdqa   xmm3, xmm2
0x14234bf49 pcmpeqb  xmm3, xmm0
0x14234bf4d pmovmskb r11d, xmm3
0x14234bf52 test     r11d, r11d
0x14234bf55 je       0x14234bf80
0x14234bf57 tzcnt    esi, r11d
0x14234bf5c add      rsi, r8
0x14234bf5f and      rsi, rdx
0x14234bf62 neg      rsi
0x14234bf65 lea      rsi, [rsi + rsi*2]
0x14234bf69 shl      rsi, 6
0x14234bf6d cmp      r10, qword ptr [rax + rsi]
0x14234bf71 je       0x14234bfaa
0x14234bf73 lea      esi, [r11 - 1]
0x14234bf77 and      si, r11w
0x14234bf7b mov      r11d, esi
0x14234bf7e jne      0x14234bf57
0x14234bf80 pcmpeqb  xmm2, xmm1
0x14234bf84 pmovmskb r11d, xmm2
0x14234bf89 test     r11d, r11d
0x14234bf8c jne      0x14234bf9e
0x14234bf8e add      r8, r9
0x14234bf91 add      r8, 0x10
0x14234bf95 add      r9, 0x10
0x14234bf99 and      r8, rdx
0x14234bf9c jmp      0x14234bf3f
0x14234bf9e lea      rcx, [rip + 0x6cc04b]
0x14234bfa5 call     0x1428d9310
0x14234bfaa mov      dword ptr [rcx + rsi - 8], 0x3f800000
0x14234bfb2 mov      rcx, qword ptr [rbp + 0x258]
0x14234bfb9 call     0x140911230
0x14234bfbe cmp      al, 3
0x14234bfc0 jne      0x14234c09e
0x14234bfc6 mov      rcx, qword ptr [rbp + 0x1c8]
0x14234bfcd call     0x1408c5680
0x14234bfd2 test     al, al
0x14234bfd4 je       0x14234c09e
0x14234bfda mov      rdx, qword ptr [rbp + 0x330]
0x14234bfe1 mov      eax, dword ptr [rdx + 0x1c]
0x14234bfe4 mov      rcx, qword ptr [rdx + 0x10]
0x14234bfe8 mov      dword ptr [rcx], eax
0x14234bfea mov      rdi, qword ptr [rdx]
0x14234bfed lea      rax, [rip + 0xd0323b] ; @0x14304f22f 'camera ghostother/camera_ghost/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\std\\src\\sy'
0x14234bff4 mov      qword ptr [rbp + 0xc0], rax
0x14234bffb mov      qword ptr [rbp + 0xc8], 0xc
0x14234c006 lea      rcx, [rbp + 0xc0]
0x14234c00d call     0x1405bdda0
0x14234c012 movabs   rbx, 0xb1695320004fb443
0x14234c01c xor      rbx, rax
0x14234c01f mov      rcx, rdi
0x14234c022 mov      rdx, rbx
0x14234c025 call     0x140683ec0
0x14234c02a mov      byte ptr [rax + 0x5d], 0
0x14234c02e test     dl, 1
0x14234c031 jne      0x14234c066
0x14234c033 mov      rsi, qword ptr [rdi + 0x10]
0x14234c037 cmp      rsi, qword ptr [rdi]
0x14234c03a jne      0x14234c04b
0x14234c03c lea      rdx, [rip + 0x862f85]
0x14234c043 mov      rcx, rdi
0x14234c046 call     0x1406319d0
0x14234c04b mov      rax, qword ptr [rdi + 8]
0x14234c04f imul     rcx, rsi, 0x38
0x14234c053 mov      dword ptr [rax + rcx], 1
0x14234c05a mov      qword ptr [rax + rcx + 0x30], rbx
0x14234c05f inc      rsi
0x14234c062 mov      qword ptr [rdi + 0x10], rsi
0x14234c066 mov      qword ptr [rbp + 0x10], rdi
0x14234c06a mov      qword ptr [rbp + 0x18], rbx
0x14234c06e mov      qword ptr [rbp + 0x20], 0
0x14234c076 mov      qword ptr [rsp + 0x20], 0
0x14234c07f lea      r8, [rip + 0xd031b5] ; @0x14304f23b 'other/camera_ghost/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\std\\src\\sys_common\\wtf'
0x14234c086 lea      rcx, [rbp + 0xc0]
0x14234c08d lea      rdx, [rbp + 0x10]
0x14234c091 mov      r9d, 0x12
0x14234c097 call     0x140f9e970
0x14234c09c jmp      0x14234c0fe
0x14234c09e mov      rcx, qword ptr [rbp + 0x258]
0x14234c0a5 call     0x140911230
0x14234c0aa cmp      al, 3
0x14234c0ac jne      0x14234c0fe
0x14234c0ae mov      rax, qword ptr [rip + 0x12dff73]
0x14234c0b5 cmp      byte ptr [rax], 1
0x14234c0b8 jne      0x14234c0fe
0x14234c0ba mov      rdx, qword ptr [rbp + 0x368]
0x14234c0c1 mov      eax, dword ptr [rdx + 0x1c]
0x14234c0c4 mov      rcx, qword ptr [rdx + 0x10]
0x14234c0c8 mov      dword ptr [rcx], eax
0x14234c0ca mov      rsi, qword ptr [rdx]
0x14234c0cd mov      rdi, qword ptr [rsi + 0x30]
0x14234c0d1 mov      rbx, qword ptr [rsi + 0x40]
0x14234c0d5 cmp      rdi, qword ptr [rsi + 0x20]
0x14234c0d9 jne      0x14234c0eb
0x14234c0db lea      rcx, [rsi + 0x20]
0x14234c0df lea      rdx, [rip + 0x86918a]
0x14234c0e6 call     0x140562870
0x14234c0eb mov      rax, qword ptr [rsi + 0x28]
0x14234c0ef mov      qword ptr [rax + rdi*8], rbx
0x14234c0f3 inc      rdi
0x14234c0f6 mov      qword ptr [rsi + 0x30], rdi
0x14234c0fa inc      qword ptr [rsi + 0x40]
0x14234c0fe movzx    eax, byte ptr [r12 + 1]
0x14234c104 mov      ecx, dword ptr [r15 + 0x1c]
0x14234c108 mov      rdx, qword ptr [r15 + 0x10]
0x14234c10c mov      dword ptr [rdx], ecx
0x14234c10e mov      byte ptr [r12], al
0x14234c112 movaps   xmm6, xmmword ptr [rbp + 0x270]
0x14234c119 movaps   xmm7, xmmword ptr [rbp + 0x280]
0x14234c120 movaps   xmm8, xmmword ptr [rbp + 0x290]
0x14234c128 movaps   xmm9, xmmword ptr [rbp + 0x2a0]
0x14234c130 movaps   xmm10, xmmword ptr [rbp + 0x2b0]
0x14234c138 add      rsp, 0x348
0x14234c13f pop      rbx
0x14234c140 pop      rdi
0x14234c141 pop      rsi
0x14234c142 pop      r12
0x14234c144 pop      r13
0x14234c146 pop      r14
0x14234c148 pop      r15
0x14234c14a pop      rbp
0x14234c14b ret      
0x14234c14c mov      qword ptr [rbp + 0xc0], 0
0x14234c157 add      rdi, 0x10
0x14234c15b lea      rax, [rip + 0xd09c86]
0x14234c162 mov      qword ptr [rsp + 0x20], rax
0x14234c167 lea      rdx, [rbp + 0x10]
0x14234c16b lea      r9, [rbp + 0xc0]
0x14234c172 xor      ecx, ecx
0x14234c174 mov      r8, rdi
0x14234c177 call     0x1428cab41
0x14234c17c lea      r8, [rip + 0xd02fdd]
0x14234c183 movaps   xmm0, xmm7
0x14234c186 movaps   xmm1, xmm6
0x14234c189 call     0x140d8b480