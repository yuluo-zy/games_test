140756370 push rbp
140756371 push r15
140756373 push r14
140756375 push r13
140756377 push r12
140756379 push rsi
14075637a push rdi
14075637b push rbx
14075637c sub rsp, 0x2e8
140756383 lea rbp, [rsp + 0x80]
14075638b movaps xmmword ptr [rbp + 0x250], xmm15
140756393 movaps xmmword ptr [rbp + 0x240], xmm14
14075639b movaps xmmword ptr [rbp + 0x230], xmm13
1407563a3 movaps xmmword ptr [rbp + 0x220], xmm12
1407563ab movaps xmmword ptr [rbp + 0x210], xmm11
1407563b3 movaps xmmword ptr [rbp + 0x200], xmm10
1407563bb movaps xmmword ptr [rbp + 0x1f0], xmm9
1407563c3 movaps xmmword ptr [rbp + 0x1e0], xmm8
1407563cb movaps xmmword ptr [rbp + 0x1d0], xmm7
1407563d2 movdqa xmmword ptr [rbp + 0x1c0], xmm6
1407563da mov qword ptr [rbp + 0x1b8], 0xfffffffffffffffe
1407563e5 mov rsi, r9
1407563e8 mov qword ptr [rbp + 0x100], r8
1407563ef mov rbx, rdx
1407563f2 mov r14, rcx
1407563f5 mov rax, qword ptr [rbp + 0x2e8]
1407563fc mov rcx, qword ptr [rax]
1407563ff lea rdx, [rip + 0x24c4472]
140756406 call 0x141209c40
14075640b test rax, rax
14075640e je 0x140757050
140756414 mov rdi, rax
140756417 lea rax, [rip + 0x24c4587]
14075641e mov qword ptr [rbp - 0x18], rax
140756422 mov qword ptr [rbp - 0x10], 0x3a
14075642a cmp qword ptr [rdi + 0x18], 0x3a
14075642f jne 0x140757020
140756435 mov rdx, qword ptr [rdi + 0x10]
140756439 lea rcx, [rip + 0x24c4565]
140756440 mov r8d, 0x3a
140756446 call 0x142923bf0
14075644b test eax, eax
14075644d jne 0x140757020
140756453 mov rcx, qword ptr [rdi]
140756456 mov rax, qword ptr [rdi + 8]
14075645a call qword ptr [rax + 0x18]
14075645d test rax, rax
140756460 je 0x140757050
140756466 mov rdi, rax
140756469 mov byte ptr [rbp + 0x1b7], 0
140756470 mov r9, qword ptr [r14]
140756473 mov rcx, qword ptr [r14 + 8]
140756477 mov rdx, qword ptr [r9]
14075647a xor r10d, r10d
14075647d mov rax, rdx
140756480 sub rax, qword ptr [rcx + 0x18]
140756484 cmovb rax, r10
140756488 sub rdx, qword ptr [rcx + 0x38]
14075648c cmovb rdx, r10
140756490 mov r13, rax
140756493 shl r13, 7
140756497 add r13, qword ptr [rcx + 8]
14075649b mov r14, qword ptr [rcx + 0x10]
14075649f sub r14, rax
1407564a2 cmovb r14, r10
1407564a6 mov r8d, 8
1407564ac cmovb r13, r8
1407564b0 mov rax, rdx
1407564b3 shl rax, 7
1407564b7 add rax, qword ptr [rcx + 0x28]
1407564bb mov r11, qword ptr [rcx + 0x30]
1407564bf sub r11, rdx
1407564c2 cmovb r11, r10
1407564c6 mov rdx, qword ptr [rbp + 0x2d8]
1407564cd cmovb rax, r8
1407564d1 mov rcx, qword ptr [rcx + 0x40]
1407564d5 sub rcx, r11
1407564d8 sub rcx, r14
1407564db mov qword ptr [rbp + 0x68], r9
1407564df mov qword ptr [r9], rcx
1407564e2 shl r14, 7
1407564e6 add r14, r13
1407564e9 shl r11, 7
1407564ed add r11, rax
1407564f0 mov qword ptr [rbp + 0x90], r11
1407564f7 mov rcx, qword ptr [rdx]
1407564fa mov edx, dword ptr [rbx + 0x1c]
1407564fd mov dword ptr [rbp + 0x190], edx
140756503 mov rdx, qword ptr [rbx]
140756506 mov r8, qword ptr [rbx + 0x10]
14075650a mov qword ptr [rbp + 0x78], r8
14075650e mov rbx, rcx
140756511 mov ecx, dword ptr [rsi + 0x1c]
140756514 mov dword ptr [rbp + 0x194], ecx
14075651a mov rcx, qword ptr [rsi]
14075651d mov qword ptr [rbp + 0xa0], rcx
140756524 mov rcx, qword ptr [rsi + 0x10]
140756528 mov qword ptr [rbp + 0xa8], rcx
14075652f mov rcx, qword ptr [rbp + 0x2d0]
140756536 mov rcx, qword ptr [rcx]
140756539 mov qword ptr [rbp + 0xb0], rcx
140756540 mov rcx, qword ptr [rbp + 0x2e0]
140756547 mov rcx, qword ptr [rcx]
14075654a mov qword ptr [rbp + 0xb8], rcx
140756551 mov rcx, qword ptr [rbp + 0x2f0]
140756558 mov rcx, qword ptr [rcx]
14075655b mov qword ptr [rbp + 0xc0], rcx
140756562 pcmpeqd xmm6, xmm6
140756566 movss xmm7, dword ptr [rip + 0x21cf302]
14075656e movss xmm8, dword ptr [rip + 0x2288bf5]
140756577 movss xmm9, dword ptr [rip + 0x21cf384]
140756580 movss xmm10, dword ptr [rip + 0x2276adf]
140756589 xorps xmm11, xmm11
14075658d lea rcx, [rbx + 0x20]
140756591 mov qword ptr [rbp + 0x70], rcx
140756595 mov qword ptr [rbp + 0xf8], rdx
14075659c lea rcx, [rdx + 0x20]
1407565a0 mov qword ptr [rbp + 0x98], rcx
1407565a7 mov qword ptr [rbp + 0x80], rbx
1407565ae mov qword ptr [rbp + 0x88], rdi
1407565b5 test r13, r13
1407565b8 je 0x1407565e0
1407565ba cmp r13, r14
1407565bd je 0x1407565e0
1407565bf lea rcx, [r13 + 0x80]
1407565c6 mov qword ptr [rbp + 0xf0], rcx
1407565cd mov qword ptr [rbp + 0xe8], rax
1407565d4 jmp 0x140756612
1407565d6 nop word ptr cs:[rax + rax]
1407565e0 test rax, rax
1407565e3 je 0x140756ee8
1407565e9 cmp rax, qword ptr [rbp + 0x90]
1407565f0 je 0x140756ee8
1407565f6 lea rcx, [rax + 0x80]
1407565fd mov qword ptr [rbp + 0xe8], rcx
140756604 mov qword ptr [rbp + 0xf0], 0
14075660f mov r13, rax
140756612 mov rax, qword ptr [rbp + 0x68]
140756616 inc qword ptr [rax]
140756619 lea rsi, [r13 + 0x68]
14075661d mov qword ptr [rbp + 0xe0], rsi
140756624 cmp qword ptr [rbx + 0x18], 0
140756629 je 0x140756edc
14075662f mov rcx, qword ptr [rbp + 0x70]
140756633 mov rdx, rsi
140756636 call 0x1405b0d70
14075663b mov r12, qword ptr [rbx]
14075663e mov rcx, qword ptr [rbx + 8]
140756642 mov rdx, rcx
140756645 and rdx, rax
140756648 shr rax, 0x39
14075664c movd xmm0, eax
140756650 punpcklbw xmm0, xmm0
140756654 pshuflw xmm0, xmm0, 0
140756659 pshufd xmm0, xmm0, 0
14075665e lea rax, [r12 - 0x2a8]
140756666 xor r8d, r8d
140756669 movdqu xmm1, xmmword ptr [r12 + rdx]
14075666f movdqa xmm2, xmm1
140756673 pcmpeqb xmm2, xmm0
140756677 pmovmskb r9d, xmm2
14075667c test r9d, r9d
14075667f je 0x1407566b0
140756681 mov r10, qword ptr [rsi]
140756684 tzcnt r11d, r9d
140756689 add r11, rdx
14075668c and r11, rcx
14075668f neg r11
140756692 imul rbx, r11, 0x2a8
140756699 cmp r10, qword ptr [rax + rbx]
14075669d je 0x1407566e0
14075669f lea r11d, [r9 - 1]
1407566a3 and r11w, r9w
1407566a7 mov r9d, r11d
1407566aa jne 0x140756684
1407566ac nop dword ptr [rax]
1407566b0 pcmpeqb xmm1, xmm6
1407566b4 pmovmskb r9d, xmm1
1407566b9 test r9d, r9d
1407566bc jne 0x140756edc
1407566c2 add rdx, r8
1407566c5 add rdx, 0x10
1407566c9 add r8, 0x10
1407566cd and rdx, rcx
1407566d0 jmp 0x140756669
1407566d2 nop word ptr cs:[rax + rax]
1407566e0 mov rsi, qword ptr [r12 + rbx - 0x180]
1407566e8 test rsi, rsi
1407566eb je 0x14075705c
1407566f1 lea rcx, [rsi + 0x18]
1407566f5 xor eax, eax
1407566f7 mov dl, 1
1407566f9 lock cmpxchg byte ptr [rsi + 0x18], dl
1407566fe jne 0x140756e65
140756704 call 0x140c81c70
140756709 mov rax, qword ptr [rsi + 0x10]
14075670d test rax, rax
140756710 sete cl
140756713 shl rax, 2
140756717 add rax, qword ptr [rsi + 8]
14075671b add rax, -4
14075671f sete dl
140756722 or dl, cl
140756724 je 0x140756740
140756726 mov rax, qword ptr [rdi + 0x28]
14075672a test rax, rax
14075672d je 0x140757068
140756733 lea ecx, [rax - 1]
140756736 jmp 0x140756746
140756738 nop dword ptr [rax + rax]
140756740 mov ecx, dword ptr [rax]
140756742 mov rax, qword ptr [rdi + 0x28]
140756746 mov rdx, qword ptr [rdi + 0x20]
14075674a movdqu xmm0, xmmword ptr [rdi + 0x30]
14075674f movdqu xmm1, xmmword ptr [rdi + 0x40]
140756754 movdqu xmm2, xmmword ptr [rdi + 0x50]
140756759 mov dword ptr [rbp + 0x38], ecx
14075675c mov qword ptr [rbp - 0x18], rdx
140756760 mov qword ptr [rbp - 0x10], rax
140756764 movdqu xmmword ptr [rbp - 8], xmm0
140756769 mov qword ptr [rbp + 8], rsi
14075676d movdqu xmmword ptr [rbp + 0x10], xmm1
140756772 mov qword ptr [rbp + 0x20], rdi
140756776 movdqu xmmword ptr [rbp + 0x28], xmm2
14075677b mov eax, dword ptr [rbp + 0x190]
140756781 mov rcx, qword ptr [rbp + 0x78]
140756785 mov dword ptr [rcx], eax
140756787 mov rax, qword ptr [rbp + 0xf8]
14075678e cmp qword ptr [rax + 0x18], 0
140756793 je 0x140756880
140756799 lea rsi, [r12 + rbx]
14075679d add r12, rbx
1407567a0 add r12, -0x2a0
1407567a7 mov rdi, qword ptr [rbp + 0xe0]
1407567ae mov rcx, qword ptr [rbp + 0x98]
1407567b5 mov rdx, rdi
1407567b8 call 0x1405b0d70
1407567bd mov rdx, qword ptr [rbp + 0xf8]
1407567c4 mov r15, qword ptr [rdx]
1407567c7 mov rdx, qword ptr [rdx + 8]
1407567cb mov r8, rdx
1407567ce and r8, rax
1407567d1 shr rax, 0x39
1407567d5 movd xmm0, eax
1407567d9 punpcklbw xmm0, xmm0
1407567dd pshuflw xmm0, xmm0, 0
1407567e2 pshufd xmm0, xmm0, 0
1407567e7 lea r10, [r15 - 0x20]
1407567eb xor r9d, r9d
1407567ee movdqu xmm1, xmmword ptr [r15 + r8]
1407567f4 movdqa xmm2, xmm1
1407567f8 pcmpeqb xmm2, xmm0
1407567fc pmovmskb r11d, xmm2
140756801 test r11d, r11d
140756804 je 0x140756860
140756806 mov rbx, qword ptr [rdi]
140756809 mov qword ptr [rbp + 0x1a8], rsi
140756810 mov rcx, r14
140756813 mov r14, r9
140756816 mov rsi, r12
140756819 tzcnt eax, r11d
14075681e add rax, r8
140756821 and rax, rdx
140756824 mov r12, rax
140756827 shl r12, 5
14075682b mov r9, r10
14075682e sub r9, r12
140756831 cmp rbx, qword ptr [r9]
140756834 je 0x140756970
14075683a lea eax, [r11 - 1]
14075683e and ax, r11w
140756842 mov r11d, eax
140756845 mov r12, rsi
140756848 mov rsi, qword ptr [rbp + 0x1a8]
14075684f mov r9, r14
140756852 mov r14, rcx
140756855 jne 0x140756810
140756857 nop word ptr [rax + rax]
140756860 pcmpeqb xmm1, xmm6
140756864 pmovmskb eax, xmm1
140756868 test eax, eax
14075686a jne 0x140756880
14075686c add r8, r9
14075686f add r8, 0x10
140756873 add r9, 0x10
140756877 and r8, rdx
14075687a jmp 0x1407567ee
14075687f nop 
140756880 mov rax, qword ptr [rip + 0x2ed5b21]
140756887 mov rax, qword ptr [rax]
14075688a test rax, rax
14075688d je 0x140756e20
140756893 lea rax, [rbp + 0xe0]
14075689a mov qword ptr [rbp + 0x40], rax
14075689e lea rax, [rip + 0xab76cb]
1407568a5 mov qword ptr [rbp + 0x48], rax
1407568a9 lea rcx, [rip + 0x2325f68]
1407568b0 call 0x14067dd80
1407568b5 movups xmm0, xmmword ptr [rax]
1407568b8 mov eax, dword ptr [rax + 0x10]
1407568bb mov qword ptr [rbp + 0x140], 1
1407568c6 lea rcx, [rip + 0x2325eb3]
1407568cd mov qword ptr [rbp + 0x148], rcx
1407568d4 mov qword ptr [rbp + 0x150], 0x31
1407568df lea rdx, [rip + 0x2325f22]
1407568e6 mov qword ptr [rbp + 0x160], rdx
1407568ed mov qword ptr [rbp + 0x168], 1
1407568f8 lea rdx, [rbp + 0x40]
1407568fc mov qword ptr [rbp + 0x170], rdx
140756903 mov qword ptr [rbp + 0x178], 1
14075690e mov qword ptr [rbp + 0x180], 0
140756919 mov qword ptr [rbp + 0x110], 0
140756924 mov qword ptr [rbp + 0x118], rcx
14075692b mov qword ptr [rbp + 0x120], 0x31
140756936 mov qword ptr [rbp + 0x128], 0
140756941 movups xmmword ptr [rbp + 0x130], xmm0
140756948 mov dword ptr [rbp + 0x158], 1
140756952 mov dword ptr [rbp + 0x15c], eax
140756958 lea rcx, [rbp + 0x1b6]
14075695f lea rdx, [rbp + 0x110]
140756966 call 0x140d4a330
14075696b jmp 0x140756e20
140756970 mov r11, qword ptr [r13 + 0x20]
140756974 mov rdx, qword ptr [r13 + 0x28]
140756978 test rdx, rdx
14075697b je 0x14075698f
14075697d cmp rdx, 1
140756981 jne 0x14075699b
140756983 pxor xmm0, xmm0
140756987 xor r10d, r10d
14075698a mov r14, rcx
14075698d jmp 0x1407569fa
14075698f pxor xmm0, xmm0
140756993 mov r14, rcx
140756996 jmp 0x140756a24
14075699b mov r8, rdx
14075699e and r8, 0xfffffffffffffffe
1407569a2 lea r9, [r11 + 0x10]
1407569a6 pxor xmm1, xmm1
1407569aa xor r10d, r10d
1407569ad mov r14, rcx
1407569b0 movdqa xmm2, xmm1
1407569b4 addss xmm2, xmm9
1407569b9 movss xmm3, dword ptr [r9 - 0xc]
1407569bf movss xmm0, dword ptr [r9]
1407569c4 cmpltss xmm3, xmm11
1407569ca andps xmm2, xmm3
1407569cd andnps xmm3, xmm1
1407569d0 orps xmm3, xmm2
1407569d3 cmpltss xmm0, xmm11
1407569d9 movaps xmm1, xmm0
1407569dc andnps xmm1, xmm3
1407569df addss xmm3, xmm9
1407569e4 andps xmm0, xmm3
1407569e7 orps xmm0, xmm1
1407569ea add r10, 2
1407569ee add r9, 0x18
1407569f2 movaps xmm1, xmm0
1407569f5 cmp r8, r10
1407569f8 jne 0x1407569b0
1407569fa test dl, 1
1407569fd je 0x140756a24
1407569ff lea r8, [r10 + r10*2]
140756a03 movss xmm1, dword ptr [r11 + r8*4 + 4]
140756a0a movaps xmm2, xmm0
140756a0d addss xmm2, xmm9
140756a12 cmpltss xmm1, xmm11
140756a18 andps xmm2, xmm1
140756a1b andnps xmm1, xmm0
140756a1e orps xmm1, xmm2
140756a21 movaps xmm0, xmm1
140756a24 neg rax
140756a27 shl rax, 5
140756a2b lea r8, [r15 + rax]
140756a2f mov qword ptr [rbp + 0x1a0], r8
140756a36 xorps xmm1, xmm1
140756a39 cvtsi2ss xmm1, rdx
140756a3e add rax, r15
140756a41 add rax, -0x18
140756a45 mov qword ptr [rbp + 0x108], rax
140756a4c divss xmm0, xmm1
140756a50 ucomiss xmm0, xmm7
140756a53 jbe 0x140756d14
140756a59 mov rsi, qword ptr [r13 + 0x10]
140756a5d cmp rsi, rdx
140756a60 cmovae rsi, rdx
140756a64 test rsi, rsi
140756a67 je 0x140756e20
140756a6d mov r10, qword ptr [r13 + 8]
140756a71 lea rdi, [rdx - 1]
140756a75 xorps xmm0, xmm0
140756a78 xor r9d, r9d
140756a7b xor ebx, ebx
140756a7d mov rcx, r9
140756a80 mov r9, rdi
140756a83 sub r9, rcx
140756a86 mov rax, r11
140756a89 xor r12d, r12d
140756a8c jmp 0x140756aa7
140756a8e nop 
140756a90 add rax, 0xc
140756a94 lea r8, [rbx + r12]
140756a98 inc r8
140756a9b inc r12
140756a9e cmp r8, rsi
140756aa1 jae 0x140756e20
140756aa7 mov r8, rcx
140756aaa add r8, r12
140756aad je 0x140756a90
140756aaf cmp r9, r12
140756ab2 je 0x140756a90
140756ab4 lea r9, [rcx + r12]
140756ab8 inc r9
140756abb cmp r9, rdx
140756abe jae 0x140757074
140756ac4 lea r8, [rbx + rbx*2]
140756ac8 movss xmm13, dword ptr [rax + r8*4]
140756ace movss xmm12, dword ptr [rax + r8*4 + 8]
140756ad5 lea rcx, [rcx + rcx*2]
140756ad9 movaps xmm1, xmm13
140756add subss xmm1, dword ptr [rax + rcx*4 + 0xc]
140756ae3 movaps xmm2, xmm12
140756ae7 subss xmm2, dword ptr [rax + rcx*4 + 0x14]
140756aed movaps xmm3, xmm1
140756af0 mulss xmm3, xmm1
140756af4 movaps xmm4, xmm2
140756af7 mulss xmm4, xmm2
140756afb addss xmm4, xmm3
140756aff xorps xmm3, xmm3
140756b02 sqrtss xmm3, xmm4
140756b06 addss xmm0, xmm3
140756b0a ucomiss xmm0, xmm7
140756b0d jbe 0x140756ccd
140756b13 mov qword ptr [rbp + 0xc8], rdi
140756b1a mov qword ptr [rbp + 0xd0], r9
140756b21 mov qword ptr [rbp + 0x198], r11
140756b28 mov qword ptr [rbp + 0x1a8], rdx
140756b2f mov r15, r10
140756b32 lea rcx, [r10 + rbx*4]
140756b36 movss xmm15, dword ptr [rax + r8*4 + 4]
140756b3d movss xmm14, dword ptr [rcx + r12*4]
140756b43 movabs rax, 0x7e4dd4fe71bf7165
140756b4d mov qword ptr [rbp + 0x148], rax
140756b54 mov dword ptr [rbp + 0x140], 0
140756b5e addss xmm15, xmm8
140756b63 movaps xmm0, xmm9
140756b67 divss xmm0, xmm3
140756b6b mulss xmm1, xmm0
140756b6f mulss xmm2, xmm0
140756b73 lea rcx, [rbp + 0x110]
140756b7a call 0x140c90fd0
140756b7f call 0x14003bb80
140756b84 addss xmm14, xmm10
140756b89 mov byte ptr [rbp + 0x150], 3
140756b90 movss dword ptr [rbp + 0x120], xmm13
140756b99 movss dword ptr [rbp + 0x124], xmm15
140756ba2 movss dword ptr [rbp + 0x128], xmm12
140756bab mov dword ptr [rbp + 0x12c], 0x3f800000
140756bb5 movss dword ptr [rbp + 0x130], xmm14
140756bbe mov dword ptr [rbp + 0x134], 0x3f800000
140756bc8 lea rcx, [rbp + 0x40]
140756bcc mov rdx, qword ptr [rbp + 0x100]
140756bd3 call 0x140d146a0
140756bd8 mov rdx, qword ptr [rbp + 0x48]
140756bdc test rdx, rdx
140756bdf jne 0x140756be5
140756be1 mov rdx, qword ptr [rbp + 0x50]
140756be5 mov rdi, qword ptr [rbp + 0x60]
140756be9 mov rax, qword ptr [rdx]
140756bec mov r13, qword ptr [rdx + 0x10]
140756bf0 sub rax, r13
140756bf3 cmp rax, 0x87
140756bf9 jbe 0x140756ce1
140756bff mov rax, qword ptr [rdx + 8]
140756c03 lea rcx, [rip + 0xab81d6]
140756c0a mov qword ptr [rax + r13], rcx
140756c0e movaps xmm0, xmmword ptr [rbp + 0x110]
140756c15 movaps xmm1, xmmword ptr [rbp + 0x120]
140756c1c movaps xmm2, xmmword ptr [rbp + 0x130]
140756c23 movaps xmm3, xmmword ptr [rbp + 0x140]
140756c2a movups xmmword ptr [rax + r13 + 8], xmm0
140756c30 movups xmmword ptr [rax + r13 + 0x18], xmm1
140756c36 movups xmmword ptr [rax + r13 + 0x28], xmm2
140756c3c movups xmmword ptr [rax + r13 + 0x38], xmm3
140756c42 movaps xmm0, xmmword ptr [rbp + 0x150]
140756c49 movups xmmword ptr [rax + r13 + 0x48], xmm0
140756c4f mov byte ptr [rax + r13 + 0x58], 0
140756c55 mov qword ptr [rax + r13 + 0x68], rdi
140756c5a lea rcx, [rip + 0xab935f]
140756c61 mov qword ptr [rax + r13 + 0x78], rcx
140756c66 add r13, 0x88
140756c6d mov qword ptr [rdx + 0x10], r13
140756c71 mov rax, qword ptr [rbp + 0x1a0]
140756c78 mov r13, qword ptr [rax - 8]
140756c7c cmp r13, qword ptr [rax - 0x18]
140756c80 jne 0x140756c95
140756c82 mov rcx, qword ptr [rbp + 0x108]
140756c89 lea rdx, [rip + 0x2325bb8]
140756c90 call 0x140b81df0
140756c95 mov rcx, qword ptr [rbp + 0x1a0]
140756c9c mov rax, qword ptr [rcx - 0x10]
140756ca0 mov qword ptr [rax + r13*8], rdi
140756ca4 inc r13
140756ca7 mov qword ptr [rcx - 8], r13
140756cab xorps xmm0, xmm0
140756cae mov rdx, qword ptr [rbp + 0x1a8]
140756cb5 mov r11, qword ptr [rbp + 0x198]
140756cbc mov r9, qword ptr [rbp + 0xd0]
140756cc3 mov r10, r15
140756cc6 mov rdi, qword ptr [rbp + 0xc8]
140756ccd add rbx, r12
140756cd0 inc rbx
140756cd3 cmp rbx, rsi
140756cd6 jb 0x140756a7d
140756cdc jmp 0x140756e20
140756ce1 mov qword ptr [rsp + 0x20], 1
140756cea mov r8d, 0x88
140756cf0 mov r9d, 1
140756cf6 mov rcx, rdx
140756cf9 mov qword ptr [rbp + 0xd8], rdx
140756d00 mov rdx, r13
140756d03 call 0x1428a7de0
140756d08 mov rdx, qword ptr [rbp + 0xd8]
140756d0f jmp 0x140756bff
140756d14 mov eax, dword ptr [rbp + 0x194]
140756d1a mov rcx, qword ptr [rbp + 0xa8]
140756d21 mov dword ptr [rcx], eax
140756d23 mov rax, qword ptr [rbp + 0x1a8]
140756d2a movss xmm0, dword ptr [rax - 0x18]
140756d2f mov rax, qword ptr [r13 + 0x68]
140756d33 mov rcx, qword ptr [r13 + 0x70]
140756d37 mov rdx, qword ptr [rbp + 0xc0]
140756d3e mov qword ptr [rsp + 0x60], rdx
140756d43 mov qword ptr [rsp + 0x58], rcx
140756d48 movss dword ptr [rsp + 0x50], xmm0
140756d4e mov qword ptr [rsp + 0x48], rax
140756d53 lea rax, [rbp + 0x1b7]
140756d5a mov qword ptr [rsp + 0x40], rax
140756d5f lea rax, [rbp - 0x18]
140756d63 mov qword ptr [rsp + 0x38], rax
140756d68 mov rax, qword ptr [rbp + 0x100]
140756d6f mov qword ptr [rsp + 0x30], rax
140756d74 mov rax, qword ptr [rbp + 0xb8]
140756d7b mov qword ptr [rsp + 0x28], rax
140756d80 mov rax, qword ptr [rbp + 0xb0]
140756d87 mov qword ptr [rsp + 0x20], rax
140756d8c lea rcx, [rbp + 0x110]
140756d93 mov rdx, rsi
140756d96 mov r8, r13
140756d99 mov r9, qword ptr [rbp + 0xa0]
140756da0 call 0x141210110
140756da5 mov rdi, qword ptr [rbp + 0x110]
140756dac mov r15, qword ptr [rbp + 0x118]
140756db3 mov rbx, qword ptr [rbp + 0x120]
140756dba mov rsi, qword ptr [rbp + 0x1a0]
140756dc1 mov rax, qword ptr [rsi - 0x18]
140756dc5 mov r12, qword ptr [rsi - 8]
140756dc9 sub rax, r12
140756dcc cmp rax, rbx
140756dcf jb 0x140756e96
140756dd5 lea r8, [rbx*8]
140756ddd lea rcx, [r12*8]
140756de5 add rcx, qword ptr [rsi - 0x10]
140756de9 mov rdx, r15
140756dec call 0x142923c00
140756df1 add r12, rbx
140756df4 mov qword ptr [rsi - 8], r12
140756df8 test rdi, rdi
140756dfb je 0x140756e20
140756dfd shl rdi, 3
140756e01 mov r8d, 8
140756e07 mov rcx, r15
140756e0a mov rdx, rdi
140756e0d call 0x140613c20
140756e12 nop word ptr cs:[rax + rax]
140756e20 mov rbx, qword ptr [rbp + 8]
140756e24 lea rsi, [rbx + 0x18]
140756e28 mov rcx, rsi
140756e2b call 0x140c81e10
140756e30 mov al, 1
140756e32 xor ecx, ecx
140756e34 lock cmpxchg byte ptr [rbx + 0x18], cl
140756e39 jne 0x140756e8a
140756e3b mov r13, qword ptr [rbp + 0xf0]
140756e42 mov rax, qword ptr [rbp + 0xe8]
140756e49 mov rbx, qword ptr [rbp + 0x80]
140756e50 mov rdi, qword ptr [rbp + 0x88]
140756e57 test r13, r13
140756e5a jne 0x1407565ba
140756e60 jmp 0x1407565e0
140756e65 mov qword ptr [rbp + 0x1a0], rcx
140756e6c mov rcx, qword ptr [rbp + 0x1a0]
140756e73 mov r8d, 0x3b9aca00
140756e79 call 0x1428cd210
140756e7e mov rcx, qword ptr [rbp + 0x1a0]
140756e85 jmp 0x140756704
140756e8a mov rcx, rsi
140756e8d xor edx, edx
140756e8f call 0x1428cdb90
140756e94 jmp 0x140756e3b
140756e96 mov qword ptr [rbp + 0x198], r15
140756e9d mov qword ptr [rbp + 0x1a8], rdi
140756ea4 mov qword ptr [rsp + 0x20], 8
140756ead mov r9d, 8
140756eb3 mov rcx, qword ptr [rbp + 0x108]
140756eba mov rdx, r12
140756ebd mov r8, rbx
140756ec0 call 0x1428a7de0
140756ec5 mov r12, qword ptr [rsi - 8]
140756ec9 mov rdi, qword ptr [rbp + 0x1a8]
140756ed0 mov r15, qword ptr [rbp + 0x198]
140756ed7 jmp 0x140756dd5
140756edc lea rcx, [rip + 0x23258d5]
140756ee3 call 0x1428d9310
140756ee8 cmp byte ptr [rbp + 0x1b7], 0
140756eef je 0x140756fbe
140756ef5 mov rax, qword ptr [rip + 0x2ed54ac]
140756efc mov rax, qword ptr [rax]
140756eff test rax, rax
140756f02 je 0x140756fbe
140756f08 lea rcx, [rip + 0x2325859]
140756f0f call 0x14067dd80
140756f14 movups xmm0, xmmword ptr [rax]
140756f17 mov eax, dword ptr [rax + 0x10]
140756f1a mov qword ptr [rbp + 0x140], 1
140756f25 lea rdx, [rip + 0x2325854]
140756f2c mov qword ptr [rbp + 0x148], rdx
140756f33 mov qword ptr [rbp + 0x150], 0x31
140756f3e lea rcx, [rip + 0x2325813]
140756f45 mov qword ptr [rbp + 0x160], rcx
140756f4c mov qword ptr [rbp + 0x168], 1
140756f57 mov qword ptr [rbp + 0x170], 8
140756f62 mov qword ptr [rbp + 0x110], 0
140756f6d xorps xmm1, xmm1
140756f70 movups xmmword ptr [rbp + 0x178], xmm1
140756f77 mov qword ptr [rbp + 0x118], rdx
140756f7e mov qword ptr [rbp + 0x120], 0x31
140756f89 mov qword ptr [rbp + 0x128], 0
140756f94 movups xmmword ptr [rbp + 0x130], xmm0
140756f9b mov dword ptr [rbp + 0x158], 1
140756fa5 mov dword ptr [rbp + 0x15c], eax
140756fab lea rcx, [rbp + 0x1b6]
140756fb2 lea rdx, [rbp + 0x110]
140756fb9 call 0x140d4a330
140756fbe movaps xmm6, xmmword ptr [rbp + 0x1c0]
140756fc5 movaps xmm7, xmmword ptr [rbp + 0x1d0]
140756fcc movaps xmm8, xmmword ptr [rbp + 0x1e0]
140756fd4 movaps xmm9, xmmword ptr [rbp + 0x1f0]
140756fdc movaps xmm10, xmmword ptr [rbp + 0x200]
140756fe4 movaps xmm11, xmmword ptr [rbp + 0x210]
140756fec movaps xmm12, xmmword ptr [rbp + 0x220]
140756ff4 movaps xmm13, xmmword ptr [rbp + 0x230]
140756ffc movaps xmm14, xmmword ptr [rbp + 0x240]
140757004 movaps xmm15, xmmword ptr [rbp + 0x250]
14075700c add rsp, 0x2e8
140757013 pop rbx
140757014 pop rdi
140757015 pop rsi
140757016 pop r12
140757018 pop r13
14075701a pop r14
14075701c pop r15
14075701e pop rbp
14075701f ret 
140757020 mov qword ptr [rbp + 0x110], 0
14075702b add rdi, 0x10
14075702f lea rax, [rip + 0x24c382a]
140757036 mov qword ptr [rsp + 0x20], rax
14075703b lea rdx, [rbp - 0x18]
14075703f lea r9, [rbp + 0x110]
140757046 xor ecx, ecx
140757048 mov r8, rdi
14075704b call 0x1428cab41
140757050 lea rcx, [rip + 0x23256c1]
140757057 call 0x1428d9310
14075705c lea rcx, [rip + 0x24c3a45]
140757063 call 0x1428d9310
140757068 lea rcx, [rip + 0x24c3a51]
14075706f call 0x1428d9310
140757074 add rcx, r12
140757077 inc rcx
14075707a lea r8, [rip + 0x23257af]
140757081 call 0x1428d9518
140757086 ud2 
