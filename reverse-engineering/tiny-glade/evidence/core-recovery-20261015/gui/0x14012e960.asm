0x14012e960 push     rbp
0x14012e961 push     r15
0x14012e963 push     r14
0x14012e965 push     r12
0x14012e967 push     rsi
0x14012e968 push     rdi
0x14012e969 push     rbx
0x14012e96a sub      rsp, 0x250
0x14012e971 lea      rbp, [rsp + 0x80]
0x14012e979 mov      qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
0x14012e984 mov      rbx, rdx
0x14012e987 mov      rsi, rcx
0x14012e98a lea      rdi, [rcx + 0x38]
0x14012e98e lea      rax, [rcx + 0x5a8]
0x14012e995 mov      qword ptr [rbp + 0x1a8], rdi
0x14012e99c mov      qword ptr [rbp + 0x1b0], rax
0x14012e9a3 cmp      qword ptr [rcx + 0x88], 0
0x14012e9ab movabs   rax, 0x9fea1ff84878cb90
0x14012e9b5 movabs   rcx, 0xb03058abfcaada2c
0x14012e9bf je       0x14012ea5a
0x14012e9c5 mov      rdx, qword ptr [rsi + 0x70]
0x14012e9c9 mov      r8, qword ptr [rsi + 0x78]
0x14012e9cd mov      r9, r8
0x14012e9d0 and      r9, rax
0x14012e9d3 xor      r10d, r10d
0x14012e9d6 movdqa   xmm0, xmmword ptr [rip + 0x2821c92] ; @0x142950670 'OOOOOOOOOOOOOOOO'
0x14012e9de pcmpeqd  xmm1, xmm1
0x14012e9e2 movdqu   xmm2, xmmword ptr [rdx + r9]
0x14012e9e8 movdqa   xmm3, xmm2
0x14012e9ec pcmpeqb  xmm3, xmm0
0x14012e9f0 pmovmskb r11d, xmm3
0x14012e9f5 test     r11d, r11d
0x14012e9f8 jne      0x14012ea2d
0x14012e9fa pcmpeqb  xmm2, xmm1
0x14012e9fe pmovmskb r11d, xmm2
0x14012ea03 test     r11d, r11d
0x14012ea06 jne      0x14012ea5a
0x14012ea08 add      r9, r10
0x14012ea0b add      r9, 0x10
0x14012ea0f add      r10, 0x10
0x14012ea13 and      r9, r8
0x14012ea16 jmp      0x14012e9e2
0x14012ea18 nop      dword ptr [rax + rax]
0x14012ea20 lea      r14d, [r11 - 1]
0x14012ea24 and      r14w, r11w
0x14012ea28 mov      r11d, r14d
0x14012ea2b je       0x14012e9fa
0x14012ea2d tzcnt    r14d, r11d
0x14012ea32 add      r14, r9
0x14012ea35 and      r14, r8
0x14012ea38 neg      r14
0x14012ea3b lea      r14, [r14 + r14*2]
0x14012ea3f cmp      qword ptr [rdx + r14*8 - 0x18], rcx
0x14012ea44 jne      0x14012ea20
0x14012ea46 cmp      qword ptr [rdx + r14*8 - 0x10], rax
0x14012ea4b jne      0x14012ea20
0x14012ea4d lea      rax, [rdx + r14*8]
0x14012ea51 mov      r14, qword ptr [rax - 8]
0x14012ea55 jmp      0x14012ed47
0x14012ea5a movzx    edx, byte ptr [rsi + 0x98]
0x14012ea61 mov      rdx, qword ptr [rsi + 0xd8]
0x14012ea68 mov      r9, qword ptr [rsi + 0xe0]
0x14012ea6f mov      r10, r9
0x14012ea72 and      r10, rax
0x14012ea75 xor      r11d, r11d
0x14012ea78 movdqa   xmm0, xmmword ptr [rip + 0x2821bf0] ; @0x142950670 'OOOOOOOOOOOOOOOO'
0x14012ea80 pcmpeqd  xmm1, xmm1
0x14012ea84 movdqu   xmm2, xmmword ptr [rdx + r10]
0x14012ea8a movdqa   xmm3, xmm2
0x14012ea8e pcmpeqb  xmm3, xmm0
0x14012ea92 pmovmskb r14d, xmm3
0x14012ea97 test     r14d, r14d
0x14012ea9a jne      0x14012eacd
0x14012ea9c pcmpeqb  xmm2, xmm1
0x14012eaa0 pmovmskb r8d, xmm2
0x14012eaa5 test     r8d, r8d
0x14012eaa8 jne      0x14012eb9d
0x14012eaae add      r10, r11
0x14012eab1 add      r10, 0x10
0x14012eab5 add      r11, 0x10
0x14012eab9 and      r10, r9
0x14012eabc jmp      0x14012ea84
0x14012eabe nop      
0x14012eac0 lea      r8d, [r14 - 1]
0x14012eac4 and      r8w, r14w
0x14012eac8 mov      r14d, r8d
0x14012eacb je       0x14012ea9c
0x14012eacd tzcnt    r15d, r14d
0x14012ead2 add      r15, r10
0x14012ead5 and      r15, r9
0x14012ead8 mov      r8, r15
0x14012eadb neg      r8
0x14012eade lea      r12, [r15*8]
0x14012eae6 shl      r8, 7
0x14012eaea sub      r8, r12
0x14012eaed cmp      qword ptr [rdx + r8 - 0x88], rcx
0x14012eaf5 jne      0x14012eac0
0x14012eaf7 cmp      qword ptr [rdx + r8 - 0x80], rax
0x14012eafc jne      0x14012eac0
0x14012eafe mov      r10, r15
0x14012eb01 shl      r10, 7
0x14012eb05 lea      r11, [r10 + r15*8]
0x14012eb09 sar      r11, 3
0x14012eb0d movabs   r10, 0xf0f0f0f0f0f0f0f1
0x14012eb17 imul     r10, r11
0x14012eb1b lea      r11, [r10 - 0x10]
0x14012eb1f and      r11, r9
0x14012eb22 movdqu   xmm0, xmmword ptr [rdx + r11]
0x14012eb28 pcmpeqd  xmm1, xmm1
0x14012eb2c pcmpeqb  xmm0, xmm1
0x14012eb30 pmovmskb r14d, xmm0
0x14012eb35 movdqu   xmm0, xmmword ptr [rdx + r10]
0x14012eb3b pcmpeqb  xmm0, xmm1
0x14012eb3f pmovmskb r9d, xmm0
0x14012eb44 test     r14d, r14d
0x14012eb47 je       0x14012eb54
0x14012eb49 bsr      r14w, r14w
0x14012eb4e xor      r14d, 0xf
0x14012eb52 jmp      0x14012eb59
0x14012eb54 mov      r14w, 0x10
0x14012eb59 add      r8, rdx
0x14012eb5c or       r9d, 0x10000
0x14012eb63 tzcnt    r9d, r9d
0x14012eb68 add      r14d, r9d
0x14012eb6b mov      r9b, 0x80
0x14012eb6e cmp      r14w, 0xf
0x14012eb73 ja       0x14012eb7f
0x14012eb75 inc      qword ptr [rsi + 0xe8]
0x14012eb7c mov      r9b, 0xff
0x14012eb7f mov      byte ptr [rdx + r10], r9b
0x14012eb83 mov      byte ptr [rdx + r11 + 0x10], r9b
0x14012eb88 dec      qword ptr [rsi + 0xf0]
0x14012eb8f mov      rdx, qword ptr [r8 - 0x78]
0x14012eb93 cmp      rdx, 2
0x14012eb97 jne      0x14012ecd4
0x14012eb9d mov      r14, qword ptr [rsi + 0x5a8]
0x14012eba4 lea      r15, [r14 + 1]
0x14012eba8 mov      qword ptr [rsi + 0x5a8], r15
0x14012ebaf movabs   rdx, 0x8000000000000000
0x14012ebb9 mov      qword ptr [rbp + 0x180], rdx
0x14012ebc0 lea      rdx, [rip + 0x282856e] ; @0x142957135 'country_core::resources::gui::TimeOfDayScreencountry_core::resources::color_id::WallColorIdscoun'
0x14012ebc7 mov      qword ptr [rbp + 0x188], rdx
0x14012ebce mov      qword ptr [rbp + 0x190], 0x2d
0x14012ebd9 mov      word ptr [rbp + 0x1a0], 0x100
0x14012ebe2 mov      qword ptr [rbp + 0x148], 1
0x14012ebed mov      qword ptr [rbp + 0x150], rcx
0x14012ebf4 mov      qword ptr [rbp + 0x158], rax
0x14012ebfb mov      qword ptr [rbp + 0x170], 4
0x14012ec06 mov      qword ptr [rbp + 0x178], 0x10
0x14012ec11 mov      qword ptr [rbp + 0x198], 0
0x14012ec1c mov      byte ptr [rbp + 0x1a2], 1
0x14012ec23 mov      qword ptr [rbp + 0x160], 0
0x14012ec2e lea      rcx, [rbp - 0x60]
0x14012ec32 lea      r8, [rbp + 0x148]
0x14012ec39 mov      rdx, r14
0x14012ec3c call     0x140d3f870
0x14012ec41 cmp      qword ptr [rsi + 0x48], r15
0x14012ec45 jae      0x14012ec60
0x14012ec47 mov      byte ptr [rbp + 0x1c7], 1
0x14012ec4e lea      r8, [rip + 0x2821c2b]
0x14012ec55 mov      rcx, rdi
0x14012ec58 mov      rdx, r15
0x14012ec5b call     0x1402b5690
0x14012ec60 mov      r15, qword ptr [rsi + 0x40]
0x14012ec64 imul     r12, r14, 0xd0
0x14012ec6b lea      rax, [r15 + r12]
0x14012ec6f mov      qword ptr [rbp + 0x1b8], rax
0x14012ec76 lea      rcx, [rbp + 0x78]
0x14012ec7a lea      rdx, [rbp - 0x60]
0x14012ec7e mov      r8d, 0xd0
0x14012ec84 call     0x142923c00
0x14012ec89 cmp      dword ptr [r15 + r12], 2
0x14012ec8e je       0x14012ec9c
0x14012ec90 mov      rcx, qword ptr [rbp + 0x1b8]
0x14012ec97 call     0x1405f0b50
0x14012ec9c lea      rdx, [rbp + 0x78]
0x14012eca0 mov      r8d, 0xd0
0x14012eca6 mov      rcx, qword ptr [rbp + 0x1b8]
0x14012ecad call     0x142923c00
0x14012ecb2 lea      rcx, [rsi + 0x70]
0x14012ecb6 movabs   rdx, 0xb03058abfcaada2c
0x14012ecc0 movabs   r8, 0x9fea1ff84878cb90
0x14012ecca mov      r9, r14
0x14012eccd call     0x140082c20
0x14012ecd2 jmp      0x14012ed47
0x14012ecd4 movups   xmm0, xmmword ptr [r8 - 0x10]
0x14012ecd9 movups   xmmword ptr [rbp + 0xe0], xmm0
0x14012ece0 movups   xmm0, xmmword ptr [r8 - 0x20]
0x14012ece5 movups   xmmword ptr [rbp + 0xd0], xmm0
0x14012ecec movups   xmm0, xmmword ptr [r8 - 0x30]
0x14012ecf1 movups   xmmword ptr [rbp + 0xc0], xmm0
0x14012ecf8 movdqu   xmm0, xmmword ptr [r8 - 0x70]
0x14012ecfe movdqu   xmm1, xmmword ptr [r8 - 0x60]
0x14012ed04 movdqu   xmm2, xmmword ptr [r8 - 0x50]
0x14012ed0a movdqu   xmm3, xmmword ptr [r8 - 0x40]
0x14012ed10 movdqu   xmmword ptr [rbp + 0xb0], xmm3
0x14012ed18 movdqu   xmmword ptr [rbp + 0xa0], xmm2
0x14012ed20 movdqu   xmmword ptr [rbp + 0x90], xmm1
0x14012ed28 movdqu   xmmword ptr [rbp + 0x80], xmm0
0x14012ed30 mov      qword ptr [rbp + 0x78], rdx
0x14012ed34 lea      rcx, [rbp + 0x78]
0x14012ed38 lea      rdx, [rbp + 0x1a8]
0x14012ed3f call     0x140d3fa40
0x14012ed44 mov      r14, rax
0x14012ed47 movups   xmm0, xmmword ptr [rbx]
0x14012ed4a movaps   xmmword ptr [rbp - 0x60], xmm0
0x14012ed4e mov      ebx, dword ptr [rsi + 0x5b0]
0x14012ed54 mov      rcx, rsi
0x14012ed57 call     0x140cf8250
0x14012ed5c lea      rax, [rsi + 0xf8]
0x14012ed63 mov      qword ptr [rbp + 0x148], r14
0x14012ed6a lea      rcx, [rsi + 0x1d8]
0x14012ed71 mov      qword ptr [rbp + 0x78], rdi
0x14012ed75 lea      rdx, [rbp + 0x148]
0x14012ed7c mov      qword ptr [rbp + 0x80], rdx
0x14012ed83 mov      qword ptr [rbp + 0x88], rax
0x14012ed8a lea      r8, [rbp + 0x78]
0x14012ed8e mov      rdx, r14
0x14012ed91 call     0x140509180
0x14012ed96 mov      rdi, rax
0x14012ed99 cmp      qword ptr [rax + 0x20], 0
0x14012ed9e je       0x14012edb0
0x14012eda0 lea      r8, [rbp - 0x60]
0x14012eda4 mov      rcx, rdi
0x14012eda7 xor      edx, edx
0x14012eda9 call     0x140cff670
0x14012edae jmp      0x14012edd9
0x14012edb0 cmp      qword ptr [rdi + 0x18], 0
0x14012edb5 je       0x14012edf1
0x14012edb7 xor      ecx, ecx
0x14012edb9 lea      rax, [rcx + 1]
0x14012edbd mov      qword ptr [rdi + 0x20], rax
0x14012edc1 mov      r8, qword ptr [rdi + 8]
0x14012edc5 imul     rcx, r8
0x14012edc9 add      rcx, qword ptr [rdi + 0x10]
0x14012edcd lea      rdx, [rbp - 0x60]
0x14012edd1 call     0x142923c00
0x14012edd6 mov      dword ptr [rdi + 0x58], ebx
0x14012edd9 mov      dword ptr [rdi + 0x5c], ebx
0x14012eddc mov      rax, rsi
0x14012eddf add      rsp, 0x250
0x14012ede6 pop      rbx
0x14012ede7 pop      rdi
0x14012ede8 pop      rsi
0x14012ede9 pop      r12
0x14012edeb pop      r14
0x14012eded pop      r15
0x14012edef pop      rbp
0x14012edf0 ret      
0x14012edf1 mov      edx, 1
0x14012edf6 mov      rcx, rdi