0x142341fe0 push     r15
0x142341fe2 push     r14
0x142341fe4 push     r13
0x142341fe6 push     r12
0x142341fe8 push     rsi
0x142341fe9 push     rdi
0x142341fea push     rbp
0x142341feb push     rbx
0x142341fec sub      rsp, 0xb8
0x142341ff3 movaps   xmmword ptr [rsp + 0xa0], xmm6
0x142341ffb mov      qword ptr [rsp + 0x50], r9
0x142342000 mov      r13, qword ptr [rdx]
0x142342003 mov      rdx, qword ptr [rdx + 8]
0x142342007 mov      r10, qword ptr [r13]
0x14234200b xor      r9d, r9d
0x14234200e mov      rax, r10
0x142342011 sub      rax, qword ptr [rdx + 0x18]
0x142342015 cmovb    rax, r9
0x142342019 sub      r10, qword ptr [rdx + 0x38]
0x14234201d cmovb    r10, r9
0x142342021 mov      rsi, rax
0x142342024 shl      rsi, 4
0x142342028 add      rsi, qword ptr [rdx + 8]
0x14234202c mov      rdi, qword ptr [rdx + 0x10]
0x142342030 mov      rbp, qword ptr [rdx + 0x30]
0x142342034 sub      rdi, rax
0x142342037 cmovb    rdi, r9
0x14234203b mov      r11d, 8
0x142342041 cmovb    rsi, r11
0x142342045 mov      rax, r10
0x142342048 shl      rax, 4
0x14234204c add      rax, qword ptr [rdx + 0x28]
0x142342050 sub      rbp, r10
0x142342053 cmovb    rbp, r9
0x142342057 cmovb    rax, r11
0x14234205b mov      rdx, qword ptr [rdx + 0x40]
0x14234205f sub      rdx, rbp
0x142342062 sub      rdx, rdi
0x142342065 mov      qword ptr [r13], rdx
0x142342069 shl      rdi, 4
0x14234206d add      rdi, rsi
0x142342070 shl      rbp, 4
0x142342074 add      rbp, rax
0x142342077 mov      r15, qword ptr [rcx]
0x14234207a mov      qword ptr [rsp + 0x68], rcx
0x14234207f mov      r12, qword ptr [rcx + 8]
0x142342083 mov      r10d, dword ptr [r8 + 0x1c]
0x142342087 mov      r11, qword ptr [r8]
0x14234208a mov      rcx, qword ptr [r8 + 0x10]
0x14234208e mov      qword ptr [rsp + 0x30], rcx
0x142342093 lea      rcx, [r11 + 0x20]
0x142342097 mov      qword ptr [rsp + 0x40], rcx
0x14234209c mov      ecx, 0xffffffff
0x1423420a1 mov      qword ptr [rsp + 0x88], r15
0x1423420a9 mov      qword ptr [rsp + 0x80], r12
0x1423420b1 mov      dword ptr [rsp + 0x4c], r10d
0x1423420b6 jmp      0x142342100 ; 
0x1423420b8 mov      rax, qword ptr [r11 + 0x28]
0x1423420bc lea      rcx, [r15 + r15*2]
0x1423420c0 movaps   xmm0, xmmword ptr [rsp + 0x90]
0x1423420c8 movups   xmmword ptr [rax + rcx*8], xmm0
0x1423420cc mov      qword ptr [rax + rcx*8 + 0x10], r12
0x1423420d1 inc      r15
0x1423420d4 mov      qword ptr [r11 + 0x30], r15
0x1423420d8 inc      qword ptr [r11 + 0x40]
0x1423420dc mov      rax, r14
0x1423420df mov      rsi, rbx
0x1423420e2 mov      r15, qword ptr [rsp + 0x88]
0x1423420ea mov      r12, qword ptr [rsp + 0x80]
0x1423420f2 mov      ecx, 0xffffffff
0x1423420f7 nop      word ptr [rax + rax]
0x142342100 test     rsi, rsi
0x142342103 je       0x142342120
0x142342105 cmp      rsi, rdi
0x142342108 je       0x142342120
0x14234210a lea      rbx, [rsi + 0x10]
0x14234210e mov      r14, rax
0x142342111 jmp      0x14234213e ; 
0x142342113 nop      word ptr cs:[rax + rax]
0x142342120 test     rax, rax
0x142342123 je       0x142342234
0x142342129 cmp      rax, rbp
0x14234212c je       0x142342234
0x142342132 lea      r14, [rax + 0x10]
0x142342136 xor      ebx, ebx
0x142342138 mov      rsi, rax
0x14234213b mov      rax, r14
0x14234213e inc      qword ptr [r13]
0x142342142 mov      rdx, qword ptr [rsi + 8]
0x142342146 mov      r8d, edx
0x142342149 mov      rsi, rbx
0x14234214c cmp      r8, qword ptr [r12 + 0x10]
0x142342151 jae      0x142342100
0x142342153 mov      r9, qword ptr [r12 + 8]
0x142342158 shr      rdx, 0x20
0x14234215c lea      r8, [r8 + r8*4]
0x142342160 mov      rax, r14
0x142342163 mov      rsi, rbx
0x142342166 cmp      dword ptr [r9 + r8*4], edx
0x14234216a jne      0x142342100
0x14234216c lea      rdx, [r9 + r8*4]
0x142342170 mov      r8d, dword ptr [rdx + 4]
0x142342174 mov      rax, r14
0x142342177 mov      rsi, rbx
0x14234217a cmp      r8, rcx
0x14234217d je       0x142342100
0x14234217f mov      rax, r14
0x142342182 mov      rsi, rbx
0x142342185 cmp      qword ptr [r15 + 0x38], r8
0x142342189 jbe      0x142342100
0x14234218f mov      rax, qword ptr [r15 + 0x28]
0x142342193 mov      ecx, r8d
0x142342196 shr      ecx, 6
0x142342199 mov      rax, qword ptr [rax + rcx*8]
0x14234219d mov      ecx, 0xffffffff
0x1423421a2 bt       rax, r8
0x1423421a6 mov      rax, r14
0x1423421a9 mov      rsi, rbx
0x1423421ac jae      0x142342100
0x1423421b2 mov      eax, dword ptr [rdx + 0xc]
0x1423421b5 mov      ecx, dword ptr [rdx + 0x10]
0x1423421b8 mov      rdx, qword ptr [r12 + 0x1a8]
0x1423421c0 lea      rax, [rax + rax*8]
0x1423421c4 mov      r8, qword ptr [r15 + 0x110]
0x1423421cb mov      r9, qword ptr [rdx + rax*8 + 0x18]
0x1423421d0 mov      rax, qword ptr [rdx + rax*8 + 0x38]
0x1423421d5 mov      rax, qword ptr [rax + r8*8]
0x1423421d9 not      rax
0x1423421dc lea      rax, [rax + rax*2]
0x1423421e0 shl      rax, 4
0x1423421e4 mov      rax, qword ptr [r9 + rax + 0x10]
0x1423421e9 shl      rcx, 4
0x1423421ed movups   xmm0, xmmword ptr [rax + rcx]
0x1423421f1 movaps   xmmword ptr [rsp + 0x90], xmm0
0x1423421f9 mov      rax, qword ptr [rsp + 0x30]
0x1423421fe mov      dword ptr [rax], r10d
0x142342201 mov      r15, qword ptr [r11 + 0x30]
0x142342205 mov      r12, qword ptr [r11 + 0x40]
0x142342209 cmp      r15, qword ptr [r11 + 0x20]
0x14234220d jne      0x1423420b8
0x142342213 mov      rcx, qword ptr [rsp + 0x40]
0x142342218 lea      rdx, [rip + 0x873051]
0x14234221f mov      rsi, r11
0x142342222 call     0x140562db0 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h015eb00fc5e65d14 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h5a1c5b6b0b0a9bcc
0x142342227 mov      r11, rsi
0x14234222a mov      r10d, dword ptr [rsp + 0x4c]
0x14234222f jmp      0x1423420b8 ; 
0x142342234 mov      rcx, qword ptr [rsp + 0x120]
0x14234223c mov      rax, qword ptr [rcx]
0x14234223f mov      rcx, qword ptr [rcx + 8]
0x142342243 mov      rdx, qword ptr [rax]
0x142342246 xor      r14d, r14d
0x142342249 mov      r8, rdx
0x14234224c sub      r8, qword ptr [rcx + 0x18]
0x142342250 cmovb    r8, r14
0x142342254 sub      rdx, qword ptr [rcx + 0x38]
0x142342258 cmovb    rdx, r14
0x14234225c lea      r9, [r8 + r8*4]
0x142342260 shl      r9, 3
0x142342264 add      r9, qword ptr [rcx + 8]
0x142342268 lea      r10, [rdx + rdx*4]
0x14234226c shl      r10, 3
0x142342270 add      r10, qword ptr [rcx + 0x28]
0x142342274 mov      r11, qword ptr [rcx + 0x10]
0x142342278 sub      r11, r8
0x14234227b cmovb    r11, r14
0x14234227f mov      r13d, 8
0x142342285 cmovb    r9, r13
0x142342289 lea      r8, [r11 + r11*4]
0x14234228d setbe    bl
0x142342290 lea      r8, [r9 + r8*8 - 0x28]
0x142342295 cmovbe   r8, r14
0x142342299 mov      r9, qword ptr [rcx + 0x30]
0x14234229d sub      r9, rdx
0x1423422a0 cmovb    r9, r14
0x1423422a4 cmovb    r10, r13
0x1423422a8 mov      rcx, qword ptr [rcx + 0x40]
0x1423422ac lea      rdx, [r9 + r9*4]
0x1423422b0 lea      rdx, [r10 + rdx*8 - 0x28]
0x1423422b5 cmovbe   rdx, r8
0x1423422b9 mov      qword ptr [rsp + 0x38], rdx
0x1423422be setbe    dl
0x1423422c1 add      r9, r11
0x1423422c4 and      dl, bl
0x1423422c6 mov      byte ptr [rsp + 0x2f], dl
0x1423422ca cmove    r9, r14
0x1423422ce sub      rcx, r9
0x1423422d1 mov      qword ptr [rax], rcx
0x1423422d4 mov      rsi, qword ptr [r15 + 0xf8]
0x1423422db mov      rax, qword ptr [r15 + 0x100]
0x1423422e2 mov      rdx, qword ptr [rsp + 0x128]
0x1423422ea mov      rcx, qword ptr [rdx + 8]
0x1423422ee test     rcx, rcx
0x1423422f1 jne      0x1423422f7
0x1423422f3 mov      rcx, qword ptr [rdx + 0x10]
0x1423422f7 mov      qword ptr [rsp + 0x58], rcx
0x1423422fc mov      rcx, qword ptr [rsp + 0x68]
0x142342301 mov      ecx, dword ptr [rcx + 0x14]
0x142342304 mov      dword ptr [rsp + 0x68], ecx
0x142342308 lea      rbx, [rsi + rax*4]
0x14234230c movzx    eax, byte ptr [r15 + 0x108]
0x142342314 mov      byte ptr [rsp + 0x4c], al
0x142342318 mov      rax, qword ptr [rsp + 0x50]
0x14234231d mov      rax, qword ptr [rax]
0x142342320 mov      qword ptr [rsp + 0x30], rax
0x142342325 movsd    xmm6, qword ptr [rip + 0x74a3c3]
0x14234232d mov      qword ptr [rsp + 0x40], 0
0x142342336 mov      eax, 8
0x14234233b mov      qword ptr [rsp + 0x70], rax
0x142342340 mov      qword ptr [rsp + 0x50], 0
0x142342349 xor      ebp, ebp
0x14234234b cmp      byte ptr [rsp + 0x4c], 0
0x142342350 je       0x142342390
0x142342352 cmp      rbp, r14
0x142342355 jne      0x1423424a1
0x14234235b cmp      rsi, rbx
0x14234235e je       0x1423427cb
0x142342364 mov      rax, qword ptr [r12 + 0x1a8]
0x14234236c nop      dword ptr [rax]
0x142342370 mov      ecx, dword ptr [rsi]
0x142342372 lea      rcx, [rcx + rcx*8]
0x142342376 mov      r14, qword ptr [rax + rcx*8 + 0x10]
0x14234237b test     r14, r14
0x14234237e jne      0x1423423d2
0x142342380 add      rsi, 4
0x142342384 cmp      rsi, rbx
0x142342387 jne      0x142342370
0x142342389 jmp      0x1423427cb ; 
0x14234238e nop      
0x142342390 cmp      rbp, r14
0x142342393 jne      0x1423424ef
0x142342399 cmp      rsi, rbx
0x14234239c je       0x1423427cb
0x1423423a2 mov      rax, qword ptr [r12 + 0x100]
0x1423423aa nop      word ptr [rax + rax]
0x1423423b0 mov      ecx, dword ptr [rsi]
0x1423423b2 lea      rcx, [rcx + rcx*4]
0x1423423b6 shl      rcx, 5
0x1423423ba mov      r14, qword ptr [rax + rcx + 0x58]
0x1423423bf test     r14, r14
0x1423423c2 jne      0x14234240b
0x1423423c4 add      rsi, 4
0x1423423c8 cmp      rsi, rbx
0x1423423cb jne      0x1423423b0
0x1423423cd jmp      0x1423427cb ; 
0x1423423d2 lea      rax, [rax + rcx*8]
0x1423423d6 mov      rdx, qword ptr [r15 + 0x110]
0x1423423dd mov      rcx, qword ptr [rax + 0x38]
0x1423423e1 cmp      qword ptr [rax + 0x40], rdx
0x1423423e5 jbe      0x14234245b
0x1423423e7 mov      r8, qword ptr [rcx + rdx*8]
0x1423423eb mov      rdx, qword ptr [rax + 0x18]
0x1423423ef test     r8, r8
0x1423423f2 je       0x14234245f
0x1423423f4 not      r8
0x1423423f7 lea      r8, [r8 + r8*2]
0x1423423fb shl      r8, 4
0x1423423ff mov      r8, qword ptr [rdx + r8 + 0x10]
0x142342404 mov      qword ptr [rsp + 0x40], r8
0x142342409 jmp      0x142342468 ; 
0x14234240b add      rax, rcx
0x14234240e mov      ecx, dword ptr [rax + 0x94]
0x142342414 mov      r9, qword ptr [r12 + 0x1a8]
0x14234241c lea      r10, [rcx + rcx*8]
0x142342420 lea      rdx, [r9 + r10*8]
0x142342424 mov      r8, qword ptr [r15 + 0x110]
0x14234242b mov      rcx, qword ptr [r9 + r10*8 + 0x38]
0x142342430 cmp      qword ptr [r9 + r10*8 + 0x40], r8
0x142342435 jbe      0x1423424ae
0x142342437 mov      r8, qword ptr [rcx + r8*8]
0x14234243b mov      rdx, qword ptr [rdx + 0x18]
0x14234243f test     r8, r8
0x142342442 je       0x1423424b2
0x142342444 not      r8
0x142342447 lea      r8, [r8 + r8*2]
0x14234244b shl      r8, 4
0x14234244f mov      r8, qword ptr [rdx + r8 + 0x10]
0x142342454 mov      qword ptr [rsp + 0x40], r8
0x142342459 jmp      0x1423424bb ; 
0x14234245b mov      rdx, qword ptr [rax + 0x18]
0x14234245f mov      qword ptr [rsp + 0x40], 0
0x142342468 mov      r8, qword ptr [r15 + 0x118]
0x14234246f add      rsi, 4
0x142342473 mov      rcx, qword ptr [rcx + r8*8]
0x142342477 not      rcx
0x14234247a lea      rcx, [rcx + rcx*2]
0x14234247e shl      rcx, 4
0x142342482 mov      r8, qword ptr [rdx + rcx + 0x10]
0x142342487 mov      qword ptr [rsp + 0x50], r8
0x14234248c mov      rcx, qword ptr [rdx + rcx + 0x28]
0x142342491 mov      qword ptr [rsp + 0x60], rcx
0x142342496 mov      rax, qword ptr [rax + 8]
0x14234249a mov      qword ptr [rsp + 0x70], rax
0x14234249f xor      ebp, ebp
0x1423424a1 mov      rax, qword ptr [rsp + 0x70]
0x1423424a6 lea      r8, [rax + rbp*8]
0x1423424aa mov      eax, ebp
0x1423424ac jmp      0x1423424ff ; 
0x1423424ae mov      rdx, qword ptr [rdx + 0x18]
0x1423424b2 mov      qword ptr [rsp + 0x40], 0
0x1423424bb mov      r8, qword ptr [r15 + 0x118]
0x1423424c2 add      rsi, 4
0x1423424c6 mov      rcx, qword ptr [rcx + r8*8]
0x1423424ca not      rcx
0x1423424cd lea      rcx, [rcx + rcx*2]
0x1423424d1 shl      rcx, 4
0x1423424d5 mov      r8, qword ptr [rdx + rcx + 0x10]
0x1423424da mov      qword ptr [rsp + 0x50], r8
0x1423424df mov      rcx, qword ptr [rdx + rcx + 0x28]
0x1423424e4 mov      qword ptr [rsp + 0x60], rcx
0x1423424e9 mov      r13, qword ptr [rax + 0x50]
0x1423424ed xor      ebp, ebp
0x1423424ef mov      rax, rbp
0x1423424f2 shl      rax, 4
0x1423424f6 lea      r8, [rax + r13]
0x1423424fa mov      eax, dword ptr [r13 + rax + 8]
0x1423424ff lea      rcx, [rax + rax*2]
0x142342503 mov      rdx, qword ptr [rsp + 0x60]
0x142342508 lea      rdx, [rdx + rax*4]
0x14234250c shl      rax, 4
0x142342510 add      rax, qword ptr [rsp + 0x40]
0x142342515 add      rcx, qword ptr [rsp + 0x50]
0x14234251a mov      r8, qword ptr [r8]
0x14234251d mov      qword ptr [rsp + 0x78], r8
0x142342522 mov      r9d, dword ptr [rax]
0x142342525 lea      r10d, [r9 - 6]
0x142342529 mov      r11, r9
0x14234252c add      r11, -5
0x142342530 xor      r8d, r8d
0x142342533 cmp      r10d, 0xf
0x142342537 cmovae   r11, r8
0x14234253b mov      r10, qword ptr [rsp + 0x30]
0x142342540 mov      r10d, dword ptr [r10]
0x142342543 lea      rdi, [r10 - 5]
0x142342547 cmp      r10, 6
0x14234254b cmovb    rdi, r8
0x14234254f cmp      r11, rdi
0x142342552 jne      0x142342630
0x142342558 mov      r8b, 1
0x14234255b cmp      r11, 6
0x14234255f ja       0x142342630
0x142342565 lea      rdi, [rip + 0xd0bd28]
0x14234256c movsxd   r11, dword ptr [rdi + r11*4]
0x142342570 add      r11, rdi
0x142342573 jmp      r11
0x142342576 movzx    r8d, byte ptr [rax + 4]
0x14234257b mov      r9, qword ptr [rsp + 0x30]
0x142342580 cmp      r8b, byte ptr [r9 + 4]
0x142342584 sete     r8b
0x142342588 jmp      0x142342630 ; 
0x14234258d cmp      r10d, 5
0x142342591 ja       0x142342630
0x142342597 cmp      r9d, r10d
0x14234259a jne      0x142342627
0x1423425a0 mov      r9d, r9d
0x1423425a3 lea      r10, [rip + 0xd0bd06]
0x1423425aa movsxd   r9, dword ptr [r10 + r9*4]
0x1423425ae add      r9, r10
0x1423425b1 jmp      r9
0x1423425b4 mov      r8d, dword ptr [rax + 4]
0x1423425b8 mov      r9, qword ptr [rsp + 0x30]
0x1423425bd cmp      r8d, dword ptr [r9 + 4]
0x1423425c1 sete     r8b
0x1423425c5 jmp      0x142342630 ; 
0x1423425c7 mov      r8, qword ptr [rax + 8]
0x1423425cb mov      r9, qword ptr [rsp + 0x30]
0x1423425d0 cmp      r8, qword ptr [r9 + 8]
0x1423425d4 sete     r8b
0x1423425d8 jmp      0x142342630 ; 
0x1423425da movzx    r8d, byte ptr [rax + 4]
0x1423425df lea      r9d, [r8 - 0x38]
0x1423425e3 cmp      r9b, 8
0x1423425e7 movzx    r9d, r9b
0x1423425eb mov      edi, 6
0x1423425f0 cmovae   r9d, edi
0x1423425f4 mov      r10, qword ptr [rsp + 0x30]
0x1423425f9 movzx    r10d, byte ptr [r10 + 4]
0x1423425fe lea      r11d, [r10 - 0x38]
0x142342602 cmp      r11b, 8
0x142342606 movzx    r11d, r11b
0x14234260a cmovae   r11d, edi
0x14234260e cmp      r9b, r11b
0x142342611 jne      0x142342627
0x142342613 cmp      r9b, 6
0x142342617 setne    r9b
0x14234261b cmp      r8b, r10b
0x14234261e sete     r8b
0x142342622 or       r8b, r9b
0x142342625 jmp      0x142342630 ; 
0x142342627 xor      r8d, r8d
0x14234262a nop      word ptr [rax + rax]
0x142342630 inc      rbp
0x142342633 mov      r9d, dword ptr [rsp + 0x68]
0x142342638 mov      dword ptr [rdx], r9d
0x14234263b mov      byte ptr [rcx + 2], r8b
0x14234263f cmp      byte ptr [rsp + 0x2f], 0
0x142342644 jne      0x14234234b
0x14234264a mov      ecx, dword ptr [rax]
0x14234264c lea      edx, [rcx - 6]
0x14234264f mov      r8, rcx
0x142342652 add      r8, -5
0x142342656 cmp      edx, 0xf
0x142342659 mov      r10d, 0
0x14234265f cmovae   r8, r10
0x142342663 mov      rdx, qword ptr [rsp + 0x38]
0x142342668 mov      edx, dword ptr [rdx]
0x14234266a lea      r9, [rdx - 5]
0x14234266e cmp      rdx, 6
0x142342672 cmovb    r9, r10
0x142342676 cmp      r8, r9
0x142342679 jne      0x14234234b
0x14234267f cmp      r8, 6
0x142342683 ja       0x142342748
0x142342689 lea      r9, [rip + 0xd0bc38]
0x142342690 movsxd   r8, dword ptr [r9 + r8*4]
0x142342694 add      r8, r9
0x142342697 jmp      r8
0x14234269a movzx    eax, byte ptr [rax + 4]
0x14234269e mov      rcx, qword ptr [rsp + 0x38]
0x1423426a3 cmp      al, byte ptr [rcx + 4]
0x1423426a6 jne      0x14234234b
0x1423426ac jmp      0x142342748 ; 
0x1423426b1 cmp      edx, 5
0x1423426b4 ja       0x142342748
0x1423426ba cmp      ecx, edx
0x1423426bc jne      0x14234234b
0x1423426c2 mov      ecx, ecx
0x1423426c4 lea      rdx, [rip + 0xd0bc19] ; @0x14304e2e4 'dD/'
0x1423426cb movsxd   rcx, dword ptr [rdx + rcx*4]
0x1423426cf add      rcx, rdx
0x1423426d2 jmp      rcx
0x1423426d4 mov      eax, dword ptr [rax + 4]
0x1423426d7 mov      rcx, qword ptr [rsp + 0x38]
0x1423426dc cmp      eax, dword ptr [rcx + 4]
0x1423426df jne      0x14234234b
0x1423426e5 jmp      0x142342748 ; 
0x1423426e7 mov      rax, qword ptr [rax + 8]
0x1423426eb mov      rcx, qword ptr [rsp + 0x38]
0x1423426f0 cmp      rax, qword ptr [rcx + 8]
0x1423426f4 jne      0x14234234b
0x1423426fa jmp      0x142342748 ; 
0x1423426fc movzx    eax, byte ptr [rax + 4]
0x142342700 lea      ecx, [rax - 0x38]
0x142342703 cmp      cl, 8
0x142342706 movzx    ecx, cl
0x142342709 mov      r9d, 6
0x14234270f cmovae   ecx, r9d
0x142342713 mov      rdx, qword ptr [rsp + 0x38]
0x142342718 movzx    edx, byte ptr [rdx + 4]
0x14234271c lea      r8d, [rdx - 0x38]
0x142342720 cmp      r8b, 8
0x142342724 movzx    r8d, r8b
0x142342728 cmovae   r8d, r9d
0x14234272c cmp      cl, r8b
0x14234272f jne      0x14234234b
0x142342735 cmp      cl, 6
0x142342738 sete     cl
0x14234273b cmp      al, dl
0x14234273d setne    al
0x142342740 test     al, cl
0x142342742 jne      0x14234234b
0x142342748 mov      rcx, qword ptr [rsp + 0x58]
0x14234274d mov      rax, qword ptr [rcx]
0x142342750 mov      rdi, qword ptr [rcx + 0x10]
0x142342754 sub      rax, rdi
0x142342757 cmp      rax, 0x27
0x14234275b jbe      0x1423427a2
0x14234275d mov      rax, qword ptr [rcx + 8]
0x142342761 lea      rdx, [rip - 0x1363568] ; @0x140fdf200 'VWH'
0x142342768 mov      qword ptr [rax + rdi], rdx
0x14234276c lea      rdx, [rip - 0x4f2f13]
0x142342773 mov      qword ptr [rax + rdi + 8], rdx
0x142342778 mov      rdx, qword ptr [rsp + 0x78]
0x14234277d mov      qword ptr [rax + rdi + 0x10], rdx
0x142342782 movsd    qword ptr [rax + rdi + 0x18], xmm6
0x142342788 mov      dword ptr [rax + rdi + 0x20], 0
0x142342790 mov      byte ptr [rax + rdi + 0x24], 0
0x142342795 add      rdi, 0x28
0x142342799 mov      qword ptr [rcx + 0x10], rdi
0x14234279d jmp      0x14234234b ; 
0x1423427a2 mov      qword ptr [rsp + 0x20], 1
0x1423427ab mov      r8d, 0x28
0x1423427b1 mov      r9d, 1
0x1423427b7 mov      rcx, qword ptr [rsp + 0x58]
0x1423427bc mov      rdx, rdi
0x1423427bf call     0x1428a58d0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hbdbbfc746df8f5db
0x1423427c4 mov      rcx, qword ptr [rsp + 0x58]
0x1423427c9 jmp      0x14234275d ; 
0x1423427cb movaps   xmm6, xmmword ptr [rsp + 0xa0]
0x1423427d3 add      rsp, 0xb8
0x1423427da pop      rbx
0x1423427db pop      rbp
0x1423427dc pop      rdi
0x1423427dd pop      rsi
0x1423427de pop      r12
0x1423427e0 pop      r13
0x1423427e2 pop      r14
0x1423427e4 pop      r15
0x1423427e6 ret      