141210110 push rbp
141210111 push r15
141210113 push r14
141210115 push r13
141210117 push r12
141210119 push rsi
14121011a push rdi
14121011b push rbx
14121011c sub rsp, 0x518
141210123 lea rbp, [rsp + 0x80]
14121012b movaps xmmword ptr [rbp + 0x480], xmm15
141210133 movaps xmmword ptr [rbp + 0x470], xmm14
14121013b movaps xmmword ptr [rbp + 0x460], xmm13
141210143 movaps xmmword ptr [rbp + 0x450], xmm12
14121014b movaps xmmword ptr [rbp + 0x440], xmm11
141210153 movaps xmmword ptr [rbp + 0x430], xmm10
14121015b movaps xmmword ptr [rbp + 0x420], xmm9
141210163 movaps xmmword ptr [rbp + 0x410], xmm8
14121016b movaps xmmword ptr [rbp + 0x400], xmm7
141210172 movaps xmmword ptr [rbp + 0x3f0], xmm6
141210179 mov qword ptr [rbp + 0x3e8], 0xfffffffffffffffe
141210184 mov qword ptr [rbp + 0xb0], r9
14121018b mov r14, r8
14121018e mov rbx, rdx
141210191 mov rdi, rcx
141210194 mov qword ptr [rbp + 0x290], 0
14121019f mov qword ptr [rbp + 0x298], 8
1412101aa mov qword ptr [rbp + 0x2a0], 0
1412101b5 mov r12, qword ptr [r8 + 0x20]
1412101b9 mov r15, qword ptr [r8 + 0x28]
1412101bd lea rax, [r15 + r15*2]
1412101c1 lea r13, [r12 + rax*4]
1412101c5 mov qword ptr [rbp + 0x2d0], r12
1412101cc mov qword ptr [rbp + 0x2d8], r13
1412101d3 mov qword ptr [rbp + 0x2e0], 0
1412101de lea rcx, [rbp + 0x2d0]
1412101e5 call 0x141237ea0
1412101ea movss xmm7, dword ptr [rip + 0x17244a6]
1412101f2 test rax, rax
1412101f5 je 0x14121025d
1412101f7 lea rsi, [rbp + 0x2d0]
1412101fe nop 
141210200 movss xmm8, dword ptr [rax]
141210205 movss xmm6, dword ptr [rax + 4]
14121020a movss xmm11, dword ptr [rax + 8]
141210210 movss xmm10, dword ptr [rdx]
141210215 movss xmm9, dword ptr [rdx + 4]
14121021b movss xmm12, dword ptr [rdx + 8]
141210221 mov rcx, rsi
141210224 call 0x141237ea0
141210229 unpcklps xmm8, xmm11
14121022d unpcklps xmm10, xmm12
141210231 subps xmm8, xmm10
141210235 subss xmm6, xmm9
14121023a mulss xmm6, xmm6
14121023e mulps xmm8, xmm8
141210242 addss xmm6, xmm8
141210247 movshdup xmm0, xmm8
14121024c addss xmm0, xmm6
141210250 sqrtss xmm0, xmm0
141210254 addss xmm7, xmm0
141210258 test rax, rax
14121025b jne 0x141210200
14121025d test r15, r15
141210260 je 0x14121396d
141210266 lea rax, [r15*4]
14121026e lea rsi, [rax + rax*2]
141210272 movzx eax, byte ptr [rip + 0x2424990]
141210279 mov edx, 4
14121027e mov rcx, rsi
141210281 call 0x140613c10
141210286 test rax, rax
141210289 je 0x1412139a3
14121028f mov qword ptr [rbp + 0xd8], rdi
141210296 mov qword ptr [rbp + 0x360], r15
14121029d mov qword ptr [rbp + 0x368], rax
1412102a4 mov qword ptr [rbp + 0x370], 0
1412102af cmp r13, 0xc
1412102b3 je 0x1412139bc
1412102b9 movss xmm0, dword ptr [rbp + 0x530]
1412102c1 addss xmm0, dword ptr [rip + 0x17155a7]
1412102c9 movss dword ptr [rbp + 0x3d0], xmm0
1412102d1 movss xmm8, dword ptr [r12]
1412102d7 mov qword ptr [rbp + 0x3d8], r13
1412102de movss xmm10, dword ptr [r13 - 0xc]
1412102e4 mov r13, qword ptr [r14 + 8]
1412102e8 mov rax, qword ptr [r14 + 0x10]
1412102ec cmp rax, r15
1412102ef mov rdi, r15
1412102f2 cmovb rdi, rax
1412102f6 xorps xmm13, xmm13
1412102fa test rax, rax
1412102fd mov qword ptr [rbp + 0x198], rbx
141210304 mov qword ptr [rbp + 0xe8], r14
14121030b je 0x141210375
14121030d lea rax, [r12 + 4]
141210312 movss xmm1, dword ptr [rip + 0x1715666]
14121031a xorps xmm0, xmm0
14121031d xor ecx, ecx
14121031f nop 
141210320 movss xmm6, dword ptr [rax]
141210324 movaps xmm11, xmm6
141210328 cmpunordss xmm11, xmm6
14121032e movaps xmm2, xmm11
141210332 andps xmm2, xmm1
141210335 minss xmm1, xmm6
141210339 andnps xmm11, xmm1
14121033d orps xmm11, xmm2
141210341 addss xmm6, dword ptr [r13 + rcx*4]
141210348 inc rcx
14121034b movaps xmm1, xmm0
14121034e maxss xmm1, xmm6
141210352 cmpunordss xmm6, xmm6
141210357 movaps xmm2, xmm6
14121035a andnps xmm2, xmm1
14121035d andps xmm6, xmm0
141210360 orps xmm6, xmm2
141210363 add rax, 0xc
141210367 movaps xmm0, xmm6
14121036a movaps xmm1, xmm11
14121036e cmp rdi, rcx
141210371 jne 0x141210320
141210373 jmp 0x141210381
141210375 movss xmm11, dword ptr [rip + 0x1715602]
14121037e xorps xmm6, xmm6
141210381 movss xmm9, dword ptr [r12 + 8]
141210388 mov rax, qword ptr [rbp + 0x3d8]
14121038f movss xmm14, dword ptr [rax - 4]
141210395 movaps xmm0, xmm6
141210398 subss xmm0, dword ptr [rbp + 0x3d0]
1412103a0 movaps xmm1, xmm11
1412103a4 maxss xmm1, xmm0
1412103a8 cmpunordss xmm0, xmm0
1412103ad movaps xmm2, xmm0
1412103b0 andnps xmm2, xmm1
1412103b3 andps xmm0, xmm11
1412103b7 orps xmm0, xmm2
1412103ba unpcklps xmm8, xmm0
1412103be unpcklps xmm10, xmm0
1412103c2 xor esi, esi
1412103c4 movss xmm0, dword ptr [rip + 0x1715538]
1412103cc lea r14, [rbp + 0x360]
1412103d3 mov qword ptr [rbp + 0x3c0], r12
1412103da mov rax, r12
1412103dd xor r12d, r12d
1412103e0 movaps xmm15, xmm13
1412103e4 divss xmm15, xmm7
1412103e9 subss xmm0, xmm15
1412103ee movaps xmm1, xmm9
1412103f2 mulss xmm1, xmm0
1412103f6 movsldup xmm0, xmm0
1412103fa mulps xmm0, xmm8
1412103fe movsldup xmm12, xmm15
141210403 mulps xmm12, xmm10
141210407 addps xmm12, xmm0
14121040b mulss xmm15, xmm14
141210410 addss xmm15, xmm1
141210415 jmp 0x14121044d
141210417 nop word ptr [rax + rax]
141210420 lea rax, [rbx + 0xc]
141210424 inc r12
141210427 mov rcx, qword ptr [rbp + 0x368]
14121042e lea rdx, [rsi + rsi*2]
141210432 movlps qword ptr [rcx + rdx*4], xmm12
141210437 movss dword ptr [rcx + rdx*4 + 8], xmm15
14121043e inc rsi
141210441 mov qword ptr [rbp + 0x370], rsi
141210448 cmp r12, r15
14121044b jb 0x141210480
14121044d cmp rax, qword ptr [rbp + 0x3d8]
141210454 je 0x1412104df
14121045a mov rbx, rax
14121045d cmp rsi, qword ptr [rbp + 0x360]
141210464 jne 0x141210420
141210466 mov rcx, r14
141210469 lea rdx, [rip + 0x1a0be98]
141210470 call 0x140b9f180
141210475 jmp 0x141210420
141210477 nop word ptr [rax + rax]
141210480 lea rcx, [r12 + r12*2]
141210484 mov rdx, qword ptr [rbp + 0x3c0]
14121048b movss xmm0, dword ptr [rdx + rcx*4 + 4]
141210491 subss xmm0, dword ptr [rbx + 4]
141210496 mulss xmm0, xmm0
14121049a movss xmm1, dword ptr [rdx + rcx*4 + 8]
1412104a0 movss xmm2, dword ptr [rdx + rcx*4]
1412104a5 unpcklps xmm2, xmm1
1412104a8 movss xmm1, dword ptr [rbx]
1412104ac movss xmm3, dword ptr [rbx + 8]
1412104b1 unpcklps xmm1, xmm3
1412104b4 subps xmm2, xmm1
1412104b7 mulps xmm2, xmm2
1412104ba addss xmm0, xmm2
1412104be movshdup xmm1, xmm2
1412104c2 addss xmm1, xmm0
1412104c6 xorps xmm0, xmm0
1412104c9 sqrtss xmm0, xmm1
1412104cd addss xmm13, xmm0
1412104d2 movss xmm0, dword ptr [rip + 0x171542a]
1412104da jmp 0x1412103e0
1412104df xor r15d, r15d
1412104e2 mov ecx, 0xc
1412104e7 mov rax, rsi
1412104ea mul rcx
1412104ed mov rbx, rax
1412104f0 jo 0x141213922
1412104f6 movabs rax, 0x7ffffffffffffffc
141210500 cmp rbx, rax
141210503 ja 0x141213922
141210509 mov r14, qword ptr [rbp + 0x368]
141210510 test rbx, rbx
141210513 je 0x141210540
141210515 movzx eax, byte ptr [rip + 0x24246ed]
14121051c mov r15d, 4
141210522 mov edx, 4
141210527 mov rcx, rbx
14121052a call 0x140613c10
14121052f test rax, rax
141210532 je 0x141213922
141210538 mov r12, rax
14121053b mov r15, rsi
14121053e jmp 0x141210549
141210540 mov r12d, 4
141210546 xor r15d, r15d
141210549 mov rcx, r12
14121054c mov rdx, r14
14121054f mov r8, rbx
141210552 call 0x142923c00
141210557 mov qword ptr [rbp + 0x120], r15
14121055e mov qword ptr [rbp + 0x128], r12
141210565 mov qword ptr [rbp + 0x130], rsi
14121056c lea rcx, [rbp + 0x2d0]
141210573 lea rdx, [rbp + 0x120]
14121057a xor r8d, r8d
14121057d call 0x1411f9be0
141210582 mov rsi, qword ptr [rbp + 0x2d0]
141210589 mov rax, rsi
14121058c neg rax
14121058f jno 0x1412105fb
141210591 mov rax, qword ptr [rbp + 0x360]
141210598 test rax, rax
14121059b je 0x1412105b7
14121059d mov rcx, qword ptr [rbp + 0x368]
1412105a4 shl rax, 2
1412105a8 lea rdx, [rax + rax*2]
1412105ac mov r8d, 4
1412105b2 call 0x140613c20
1412105b7 mov rsi, qword ptr [rbp + 0xd8]
1412105be mov qword ptr [rsi], 0
1412105c5 mov qword ptr [rsi + 8], 8
1412105cd mov qword ptr [rsi + 0x10], 0
1412105d5 mov rdx, qword ptr [rbp + 0x290]
1412105dc test rdx, rdx
1412105df je 0x141213299
1412105e5 mov rcx, qword ptr [rbp + 0x298]
1412105ec shl rdx, 3
1412105f0 mov r8d, 8
1412105f6 jmp 0x141213294
1412105fb movups xmm0, xmmword ptr [rbp + 0x2d8]
141210602 movups xmm1, xmmword ptr [rbp + 0x2e8]
141210609 movups xmm2, xmmword ptr [rbp + 0x2f8]
141210610 movaps xmmword ptr [rbp + 0x220], xmm2
141210617 movaps xmmword ptr [rbp + 0x210], xmm1
14121061e movaps xmmword ptr [rbp + 0x200], xmm0
141210625 mov rax, qword ptr [rbp + 0x360]
14121062c test rax, rax
14121062f je 0x14121064b
141210631 mov rcx, qword ptr [rbp + 0x368]
141210638 shl rax, 2
14121063c lea rdx, [rax + rax*2]
141210640 mov r8d, 4
141210646 call 0x140613c20
14121064b mov qword ptr [rbp + 0x1b0], rsi
141210652 movaps xmm0, xmmword ptr [rbp + 0x200]
141210659 movaps xmm1, xmmword ptr [rbp + 0x210]
141210660 movaps xmm2, xmmword ptr [rbp + 0x220]
141210667 movups xmmword ptr [rbp + 0x1b8], xmm0
14121066e movups xmmword ptr [rbp + 0x1c8], xmm1
141210675 movups xmmword ptr [rbp + 0x1d8], xmm2
14121067c mov rbx, qword ptr [rbp + 0x1c0]
141210683 test rbx, rbx
141210686 je 0x1412106b6
141210688 lea rsi, [rbx*4]
141210690 movzx eax, byte ptr [rip + 0x2424572]
141210697 mov edx, 4
14121069c mov rcx, rsi
14121069f call 0x140613c10
1412106a4 test rax, rax
1412106a7 je 0x1412139f3
1412106ad mov rsi, qword ptr [rbp + 0x1c0]
1412106b4 jmp 0x1412106bd
1412106b6 mov eax, 4
1412106bb xor esi, esi
1412106bd mov qword ptr [rbp + 0x2d0], rbx
1412106c4 mov qword ptr [rbp + 0x2d8], rax
1412106cb mov qword ptr [rbp + 0x2e0], 0
1412106d6 mov r12, qword ptr [rbp + 0x1b8]
1412106dd mov qword ptr [rbp + 0xa8], 0
1412106e8 lea r15, [rbp + 0x2d0]
1412106ef mov qword ptr [rbp + 0x3c8], 0
1412106fa xor ebx, ebx
1412106fc xor r14d, r14d
1412106ff jmp 0x141210748
141210701 nop word ptr cs:[rax + rax]
141210710 inc rbx
141210713 inc r14
141210716 subss xmm8, xmm7
14121071b mov rax, qword ptr [rbp + 0x2d8]
141210722 mov rcx, qword ptr [rbp + 0x3c8]
141210729 movss dword ptr [rax + rcx*4], xmm8
14121072f inc rcx
141210732 mov qword ptr [rbp + 0x3c8], rcx
141210739 mov qword ptr [rbp + 0x2e0], rcx
141210740 add qword ptr [rbp + 0xa8], 4
141210748 cmp rbx, rdi
14121074b jae 0x14121079d
14121074d cmp r14, rsi
141210750 jae 0x141213877
141210756 lea rax, [r14 + r14*2]
14121075a movss xmm7, dword ptr [r12 + rax*4 + 4]
141210761 lea rax, [rbx + rbx*2]
141210765 mov rcx, qword ptr [rbp + 0x3c0]
14121076c subss xmm7, dword ptr [rcx + rax*4 + 4]
141210772 movss xmm8, dword ptr [r13 + rbx*4]
141210779 mov rax, qword ptr [rbp + 0x3c8]
141210780 cmp rax, qword ptr [rbp + 0x2d0]
141210787 jne 0x141210710
141210789 mov rcx, r15
14121078c lea rdx, [rip + 0x1a0bbed]
141210793 call 0x140650050
141210798 jmp 0x141210710
14121079d mov r14, qword ptr [rbp + 0x2d0]
1412107a4 mov rax, qword ptr [rbp + 0x2d8]
1412107ab mov qword ptr [rbp + 0x2a8], rax
1412107b2 movss xmm0, dword ptr [rbp + 0x1e0]
1412107ba movss dword ptr [rbp + 0x39c], xmm0
1412107c2 subss xmm6, xmm11
1412107c7 ucomiss xmm6, dword ptr [rbp + 0x3d0]
1412107ce mov qword ptr [rbp + 0x2b0], r14
1412107d5 mov r12, qword ptr [rbp + 0x198]
1412107dc mov r13, qword ptr [rbp + 0x3c8]
1412107e3 jbe 0x141211878
1412107e9 mov rax, qword ptr [rbp + 0x1c0]
1412107f0 test rax, rax
1412107f3 je 0x141213a0c
1412107f9 mov rcx, qword ptr [rbp + 0x1b8]
141210800 lea rax, [rax + rax*2]
141210804 lea rax, [rcx + rax*4]
141210808 cmp rax, 0xc
14121080c movss xmm4, dword ptr [rip + 0x17150f0]
141210814 je 0x141213a21
14121081a movsd xmm1, qword ptr [rcx]
14121081e movss xmm15, dword ptr [rcx + 8]
141210824 movsd xmm10, qword ptr [rax - 0xc]
14121082a movss xmm14, dword ptr [rax - 4]
141210830 movaps xmm0, xmm10
141210834 movaps xmmword ptr [rbp + 0x1a0], xmm1
14121083b subss xmm0, xmm1
14121083f movaps xmm1, xmm14
141210843 subss xmm1, xmm15
141210848 movaps xmm2, xmm0
14121084b mulss xmm2, xmm0
14121084f movaps xmm3, xmm1
141210852 mulss xmm3, xmm1
141210856 addss xmm3, xmm2
14121085a xorps xmm2, xmm2
14121085d sqrtss xmm2, xmm3
141210861 movaps xmm3, xmm4
141210864 divss xmm3, xmm2
141210868 mulss xmm0, xmm3
14121086c mulss xmm3, xmm1
141210870 movss dword ptr [rbp + 0x2d0], xmm0
141210878 movss dword ptr [rbp + 0x2d4], xmm3
141210880 lea rcx, [rbp + 0x84]
141210887 lea rdx, [rbp + 0x2d0]
14121088e call 0x140c9cdc0
141210893 movss xmm8, dword ptr [rbp + 0x84]
14121089c movss xmm12, dword ptr [rbp + 0x8c]
1412108a5 movaps xmm9, xmmword ptr [rip + 0x1715043]
1412108ad xorps xmm9, xmm12
1412108b1 subss xmm6, dword ptr [rbp + 0x3d0]
1412108b9 movss xmm0, dword ptr [rip + 0x199d2f3]
1412108c1 ucomiss xmm0, xmm6
1412108c4 jbe 0x1412109df
1412108ca divss xmm6, dword ptr [rip + 0x178fe92]
1412108d2 movaps xmm0, xmm6
1412108d5 call 0x142923fa0
1412108da maxss xmm0, dword ptr [rip + 0x1715022]
1412108e2 cvttss2si rax, xmm0
1412108e7 mov rcx, rax
1412108ea sar rcx, 0x3f
1412108ee movaps xmm1, xmm0
1412108f1 subss xmm1, dword ptr [rip + 0x17497e3]
1412108f9 cvttss2si rdx, xmm1
1412108fe and rdx, rcx
141210901 or rdx, rax
141210904 xor eax, eax
141210906 xorps xmm1, xmm1
141210909 ucomiss xmm0, xmm1
14121090c cmovae rax, rdx
141210910 ucomiss xmm0, dword ptr [rip + 0x17497c9]
141210917 mov rcx, 0xffffffffffffffff
14121091e cmovbe rcx, rax
141210922 cmp rcx, 4
141210926 mov esi, 4
14121092b cmovb rsi, rcx
14121092f xorps xmm6, xmm6
141210932 cvtsi2ss xmm6, rsi
141210937 mulss xmm6, dword ptr [rip + 0x17cd0ad]
14121093f mulss xmm6, dword ptr [rip + 0x1806371]
141210947 movaps xmm7, xmm6
14121094a mulss xmm7, xmm9
14121094f movaps xmm13, xmm8
141210953 mulss xmm6, xmm8
141210958 movaps xmm1, xmmword ptr [rbp + 0x1a0]
14121095f addss xmm1, xmm7
141210963 movaps xmm2, xmm15
141210967 addss xmm2, xmm6
14121096b mov rcx, qword ptr [rbp + 0x508]
141210972 call 0x141246fb0
141210977 movaps xmm8, xmm0
14121097b addss xmm7, xmm10
141210980 addss xmm6, xmm14
141210985 mov rcx, qword ptr [rbp + 0x508]
14121098c movaps xmm1, xmm7
14121098f movaps xmm2, xmm6
141210992 call 0x141246fb0
141210997 subss xmm8, xmm0
14121099c andps xmm8, xmmword ptr [rip + 0x171dfdc]
1412109a4 subss xmm8, dword ptr [rbp + 0x3d0]
1412109ad movss xmm0, dword ptr [rip + 0x199d1ff]
1412109b5 ucomiss xmm0, xmm8
1412109b9 movshdup xmm0, xmmword ptr [rbp + 0x1a0]
1412109c1 xorps xmm1, xmm1
1412109c4 jbe 0x1412109fb
1412109c6 ucomiss xmm0, xmm1
1412109c9 mov r14, qword ptr [rbp + 0x2b0]
1412109d0 movaps xmm9, xmm13
1412109d4 ja 0x141210c45
1412109da jmp 0x141211878
1412109df movshdup xmm0, xmmword ptr [rbp + 0x1a0]
1412109e7 xorps xmm1, xmm1
1412109ea ucomiss xmm0, xmm1
1412109ed mov r14, qword ptr [rbp + 0x2b0]
1412109f4 ja 0x141210a0f
1412109f6 jmp 0x141211878
1412109fb ucomiss xmm0, xmm1
1412109fe mov r14, qword ptr [rbp + 0x2b0]
141210a05 movaps xmm8, xmm13
141210a09 jbe 0x141211878
141210a0f movss dword ptr [rbp + 0x120], xmm9
141210a18 movss dword ptr [rbp + 0x124], xmm8
141210a21 mov rax, qword ptr [rbp + 0x3d8]
141210a28 movss xmm0, dword ptr [rax - 8]
141210a2d mov rax, qword ptr [rbp + 0x3c0]
141210a34 ucomiss xmm0, dword ptr [rax + 4]
141210a38 movss xmm0, dword ptr [rbp + 0x88]
141210a40 movaps xmm9, xmm8
141210a44 unpcklps xmm8, xmm0
141210a48 mulps xmm8, xmmword ptr [rip + 0x1a0b3b0]
141210a50 movss xmm6, dword ptr [rip + 0x1a0b3b8]
141210a58 mulss xmm6, xmm12
141210a5d jbe 0x141210a99
141210a5f lea rcx, [rbp + 0x2d0]
141210a66 lea rdx, [rbp + 0x120]
141210a6d call 0x140c9cdc0
141210a72 addss xmm15, xmm6
141210a77 addps xmm8, xmmword ptr [rbp + 0x1a0]
141210a7f movsd xmm6, qword ptr [rbp + 0x2d0]
141210a87 movss xmm7, dword ptr [rbp + 0x2d8]
141210a8f movaps xmm14, xmm15
141210a93 movaps xmm10, xmm8
141210a97 jmp 0x141210ac5
141210a99 lea rcx, [rbp + 0x2d0]
141210aa0 lea rdx, [rbp + 0x120]
141210aa7 call 0x140c9cdc0
141210aac subss xmm14, xmm6
141210ab1 subps xmm10, xmm8
141210ab5 movsd xmm6, qword ptr [rbp + 0x2d0]
141210abd movss xmm7, dword ptr [rbp + 0x2d8]
141210ac5 movabs rax, 0x6cd896c8b7173d6e
141210acf mov qword ptr [rbp + 0x308], rax
141210ad6 mov dword ptr [rbp + 0x300], 0
141210ae0 movss xmm1, dword ptr [rbp + 0x120]
141210ae8 movss xmm2, dword ptr [rbp + 0x124]
141210af0 lea rcx, [rbp + 0x2d0]
141210af7 call 0x140c90fd0
141210afc call 0x14003bb80
141210b01 mulss xmm7, dword ptr [rip + 0x17ccee3]
141210b09 mulps xmm6, xmmword ptr [rip + 0x17ce6c0]
141210b10 mulps xmm6, xmmword ptr [rip + 0x1714dc9]
141210b17 mulss xmm7, dword ptr [rip + 0x1714d51]
141210b1f addps xmm6, xmm10
141210b23 addss xmm7, xmm14
141210b28 mov byte ptr [rbp + 0x310], 3
141210b2f movlps qword ptr [rbp + 0x2e0], xmm6
141210b36 movss dword ptr [rbp + 0x2e8], xmm7
141210b3e movabs rax, 0x3f8000003f800000
141210b48 mov qword ptr [rbp + 0x2ec], rax
141210b4f mov dword ptr [rbp + 0x2f4], 0x3f800000
141210b59 lea rcx, [rbp + 0x200]
141210b60 mov rdx, qword ptr [rbp + 0x510]
141210b67 call 0x140d146a0
141210b6c mov rsi, qword ptr [rbp + 0x208]
141210b73 test rsi, rsi
141210b76 jne 0x141210b7f
141210b78 mov rsi, qword ptr [rbp + 0x210]
141210b7f mov rbx, qword ptr [rbp + 0x220]
141210b86 mov rax, qword ptr [rsi]
141210b89 mov rdi, qword ptr [rsi + 0x10]
141210b8d sub rax, rdi
141210b90 cmp rax, 0x87
141210b96 jbe 0x1412137e2
141210b9c mov rax, qword ptr [rsi + 8]
141210ba0 lea rcx, [rip - 0x1dc7]
141210ba7 mov qword ptr [rax + rdi], rcx
141210bab movaps xmm0, xmmword ptr [rbp + 0x2d0]
141210bb2 movaps xmm1, xmmword ptr [rbp + 0x2e0]
141210bb9 movaps xmm2, xmmword ptr [rbp + 0x2f0]
141210bc0 movaps xmm3, xmmword ptr [rbp + 0x300]
141210bc7 movups xmmword ptr [rax + rdi + 8], xmm0
141210bcc movups xmmword ptr [rax + rdi + 0x18], xmm1
141210bd1 movups xmmword ptr [rax + rdi + 0x28], xmm2
141210bd6 movups xmmword ptr [rax + rdi + 0x38], xmm3
141210bdb movaps xmm0, xmmword ptr [rbp + 0x310]
141210be2 movups xmmword ptr [rax + rdi + 0x48], xmm0
141210be7 mov byte ptr [rax + rdi + 0x58], 0
141210bec mov qword ptr [rax + rdi + 0x68], rbx
141210bf1 lea rcx, [rip - 0xc38]
141210bf8 mov qword ptr [rax + rdi + 0x78], rcx
141210bfd add rdi, 0x88
141210c04 mov qword ptr [rsi + 0x10], rdi
141210c08 mov rsi, qword ptr [rbp + 0x2a0]
141210c0f cmp rsi, qword ptr [rbp + 0x290]
141210c16 jne 0x141210c2b
141210c18 lea rdx, [rip + 0x1a0b341]
141210c1f lea rcx, [rbp + 0x290]
141210c26 call 0x140b81df0
141210c2b mov rax, qword ptr [rbp + 0x298]
141210c32 mov qword ptr [rax + rsi*8], rbx
141210c36 inc rsi
141210c39 mov qword ptr [rbp + 0x2a0], rsi
141210c40 mov esi, 1
141210c45 mov rax, qword ptr [rbp + 0x528]
141210c4c mov qword ptr [rbp + 0x3c0], rax
141210c53 mov qword ptr [rbp + 0x108], 0
141210c5e mov qword ptr [rbp + 0x110], 4
141210c69 mov qword ptr [rbp + 0x118], 0
141210c74 mov qword ptr [rbp + 0x180], 0
141210c7f mov qword ptr [rbp + 0x188], 4
141210c8a mov qword ptr [rbp + 0x190], 0
141210c95 mov qword ptr [rbp + 0x278], 0
141210ca0 mov qword ptr [rbp + 0x280], 4
141210cab mov qword ptr [rbp + 0x288], 0
141210cb6 mov qword ptr [rbp + 0x1e8], 0
141210cc1 mov qword ptr [rbp + 0x1f0], 4
141210ccc mov qword ptr [rbp + 0x1f8], 0
141210cd7 addss xmm11, dword ptr [rip + 0x1a0b134]
141210ce0 mov qword ptr [rbp + 0x2b8], 0
141210ceb mov qword ptr [rbp + 0x2c0], 4
141210cf6 mov qword ptr [rbp + 0x2c8], 0
141210d01 unpcklps xmm12, xmm9
141210d05 mulps xmm12, xmmword ptr [rip + 0x17ce4c3]
141210d0d mulps xmm12, xmmword ptr [rip + 0x186ff7b]
141210d15 movaps xmmword ptr [rbp + 0x70], xmm12
141210d1a lea r13, [rbp + 0x308]
141210d21 lea r14, [rbp + 0x230]
141210d28 movshdup xmm7, xmmword ptr [rbp + 0x1a0]
141210d30 movaps xmm8, xmm11
141210d34 shufps xmm8, xmm11, 0
141210d39 mov edi, 4
141210d3e xor r15d, r15d
141210d41 movss xmm0, dword ptr [rip + 0x186e507]
141210d49 xorps xmm13, xmm13
141210d4d movss xmm6, dword ptr [rip + 0x18788ab]
141210d55 movss xmm1, dword ptr [rip + 0x1714c27]
141210d5d lea r9, [rip + 0x1a07e2c]
141210d64 lea rbx, [rbp + 0x360]
141210d6b mov qword ptr [rbp + 0x3d8], 0
141210d76 xor r8d, r8d
141210d79 mov qword ptr [rbp + 0x1a0], rsi
141210d80 cmp r8, rsi
141210d83 jae 0x141213436
141210d89 lea rax, [r8 + 1]
141210d8d cvtsi2ss xmm10, r8
141210d92 movaps xmm14, xmm10
141210d96 mulss xmm14, xmm0
141210d9b addss xmm14, xmm7
141210da0 ucomiss xmm13, xmm14
141210da4 jae 0x141210db2
141210da6 mov qword ptr [rbp + 0x3d0], rax
141210dad jmp 0x141210e80
141210db2 cmp rax, rsi
141210db5 jae 0x141213436
141210dbb lea rcx, [r8 + 2]
141210dbf xorps xmm10, xmm10
141210dc3 cvtsi2ss xmm10, rax
141210dc8 movaps xmm14, xmm10
141210dcc mulss xmm14, xmm0
141210dd1 addss xmm14, xmm7
141210dd6 ucomiss xmm13, xmm14
141210dda jae 0x141210de8
141210ddc mov qword ptr [rbp + 0x3d0], rcx
141210de3 jmp 0x141210e80
141210de8 cmp rcx, rsi
141210deb jae 0x141213436
141210df1 lea rdx, [r8 + 3]
141210df5 xorps xmm10, xmm10
141210df9 cvtsi2ss xmm10, rcx
141210dfe movaps xmm14, xmm10
141210e02 mulss xmm14, xmm0
141210e07 addss xmm14, xmm7
141210e0c ucomiss xmm13, xmm14
141210e10 jae 0x141210e1b
141210e12 mov qword ptr [rbp + 0x3d0], rdx
141210e19 jmp 0x141210e80
141210e1b cmp rdx, rsi
141210e1e jae 0x141213436
141210e24 lea rax, [r8 + 4]
141210e28 xorps xmm10, xmm10
141210e2c cvtsi2ss xmm10, rdx
141210e31 movaps xmm14, xmm10
141210e35 mulss xmm14, xmm0
141210e3a addss xmm14, xmm7
141210e3f ucomiss xmm13, xmm14
141210e43 jb 0x141210da6
141210e49 cmp rax, rsi
141210e4c jae 0x141213436
141210e52 xorps xmm10, xmm10
141210e56 cvtsi2ss xmm10, rax
141210e5b add r8, 5
141210e5f mov qword ptr [rbp + 0x3d0], r8
141210e66 movaps xmm14, xmm10
141210e6a mulss xmm14, xmm0
141210e6f addss xmm14, xmm7
141210e74 nop word ptr cs:[rax + rax]
141210e80 movaps xmm12, xmm6
141210e84 divss xmm6, dword ptr [rbp + 0x1e0]
141210e8c movaps xmm15, xmm1
141210e90 minss xmm6, xmm1
141210e94 mov rdx, qword ptr [rbp + 0x1b8]
141210e9b mov rax, qword ptr [rbp + 0x1c0]
141210ea2 lea rax, [rax + rax*2]
141210ea6 lea r8, [rdx + rax*4]
141210eaa lea rsi, [rbp + 0x200]
141210eb1 mov rcx, rsi
141210eb4 call 0x1402ba9e0
141210eb9 movss xmm9, dword ptr [rip + 0x1714a42]
141210ec2 lea rcx, [rbp + 0x2d0]
141210ec9 mov rdx, rsi
141210ecc xor r8d, r8d
141210ecf call 0x1411f9920
141210ed4 cmp r15, qword ptr [rbp + 0x2d0]
141210edb jo 0x141213a32
141210ee1 mov rax, qword ptr [rbp + 0x300]
141210ee8 mov qword ptr [rbp + 0x230], rax
141210eef movups xmm0, xmmword ptr [rbp + 0x2d0]
141210ef6 movups xmm1, xmmword ptr [rbp + 0x2e0]
141210efd movups xmm2, xmmword ptr [rbp + 0x2f0]
141210f04 movaps xmmword ptr [rbp + 0x220], xmm2
141210f0b movaps xmmword ptr [rbp + 0x210], xmm1
141210f12 movaps xmmword ptr [rbp + 0x200], xmm0
141210f19 movss dword ptr [rbp + 0x3a0], xmm6
141210f21 ucomiss xmm6, xmm13
141210f25 jb 0x141213604
141210f2b movaps xmm2, xmm9
141210f2f subss xmm2, xmm6
141210f33 movss dword ptr [rbp + 0x3a0], xmm2
141210f3b ucomiss xmm2, xmm13
141210f3f jb 0x14121366e
141210f45 ucomiss xmm9, xmm2
141210f49 jb 0x14121366e
141210f4f mov qword ptr [rbp + 0x3a0], 0
141210f5a mov qword ptr [rbp + 0x3a8], 4
141210f65 mov qword ptr [rbp + 0x3b0], 0
141210f70 mov byte ptr [rbp + 0x3e6], 1
141210f77 mov rcx, rsi
141210f7a movaps xmm1, xmm6
141210f7d lea r9, [rbp + 0x3a0]
141210f84 call 0x1411f8130
141210f89 mov rax, qword ptr [rbp + 0x3b0]
141210f90 mov qword ptr [rbp + 0x370], rax
141210f97 movups xmm0, xmmword ptr [rbp + 0x3a0]
141210f9e movaps xmmword ptr [rbp + 0x360], xmm0
141210fa5 mov byte ptr [rbp + 0x3e6], 0
141210fac lea rcx, [rbp + 0x2d0]
141210fb3 mov rdx, rbx
141210fb6 xor r8d, r8d
141210fb9 call 0x1411f9920
141210fbe cmp r15, qword ptr [rbp + 0x2d0]
141210fc5 jo 0x141213a70
141210fcb mov rax, qword ptr [rbp + 0x300]
141210fd2 mov qword ptr [rbp + 0x150], rax
141210fd9 movups xmm0, xmmword ptr [rbp + 0x2d0]
141210fe0 movups xmm1, xmmword ptr [rbp + 0x2e0]
141210fe7 movups xmm2, xmmword ptr [rbp + 0x2f0]
141210fee movaps xmmword ptr [rbp + 0x140], xmm2
141210ff5 movaps xmmword ptr [rbp + 0x130], xmm1
141210ffc movaps xmmword ptr [rbp + 0x120], xmm0
141211003 mov rdx, qword ptr [rbp + 0x200]
14121100a test rdx, rdx
14121100d mov rbx, qword ptr [rbp + 0x3d8]
141211014 je 0x14121102c
141211016 mov rcx, qword ptr [rbp + 0x208]
14121101d shl rdx, 3
141211021 mov r8d, 4
141211027 call 0x140613c20
14121102c mov rdx, qword ptr [rbp + 0x218]
141211033 test rdx, rdx
141211036 je 0x14121104e
141211038 mov rcx, qword ptr [rbp + 0x220]
14121103f shl rdx, 2
141211043 mov r8d, 4
141211049 call 0x140613c20
14121104e mov rax, qword ptr [rbp + 0x130]
141211055 test rax, rax
141211058 je 0x141211120
14121105e mov rdx, qword ptr [rbp + 0x128]
141211065 movsldup xmm0, xmm10
14121106a mulps xmm0, xmmword ptr [rbp + 0x70]
14121106e lea r8, [rax - 1]
141211072 movabs rcx, 0x1fffffffffffffff
14121107c and r8, rcx
14121107f mov rcx, rdx
141211082 cmp r8, 2
141211086 jbe 0x1412110fc
141211088 inc r8
14121108b mov r9, r8
14121108e and r9, 0xfffffffffffffffc
141211092 lea rcx, [rdx + r9*8]
141211096 movsldup xmm1, xmm0
14121109a movshdup xmm2, xmm0
14121109e xor r10d, r10d
1412110a1 nop word ptr cs:[rax + rax]
1412110b0 movups xmm3, xmmword ptr [rdx + r10*8]
1412110b5 movups xmm4, xmmword ptr [rdx + r10*8 + 0x10]
1412110bb movaps xmm5, xmm3
1412110be shufps xmm5, xmm3, 0xe8
1412110c2 movaps xmm6, xmm4
1412110c5 shufps xmm6, xmm4, 0xe8
1412110c9 shufps xmm3, xmm3, 0xed
1412110cd shufps xmm4, xmm4, 0xed
1412110d1 subps xmm5, xmm1
1412110d4 subps xmm6, xmm1
1412110d7 addps xmm3, xmm2
1412110da unpcklps xmm5, xmm3
1412110dd addps xmm4, xmm2
1412110e0 unpcklps xmm6, xmm4
1412110e3 movups xmmword ptr [rdx + r10*8], xmm5
1412110e8 movups xmmword ptr [rdx + r10*8 + 0x10], xmm6
1412110ee add r10, 4
1412110f2 cmp r9, r10
1412110f5 jne 0x1412110b0
1412110f7 cmp r8, r9
1412110fa je 0x141211117
1412110fc lea rax, [rdx + rax*8]
141211100 movsd xmm1, qword ptr [rcx]
141211104 addsubps xmm1, xmm0
141211108 movlps qword ptr [rcx], xmm1
14121110b lea rdx, [rcx + 8]
14121110f mov rcx, rdx
141211112 cmp rdx, rax
141211115 jne 0x141211100
141211117 mov r15, qword ptr [rbp + 0x130]
14121111e jmp 0x141211123
141211120 xor r15d, r15d
141211123 cmp rbx, r15
141211126 cmovae rbx, r15
14121112a mov qword ptr [rbp + 0x1f8], rbx
141211131 test rbx, rbx
141211134 je 0x1412111a5
141211136 lea rcx, [rbx - 1]
14121113a movabs rax, 0x3fffffffffffffff
141211144 and rcx, rax
141211147 mov rax, rdi
14121114a cmp rcx, 7
14121114e jb 0x141211189
141211150 inc rcx
141211153 mov rdx, rcx
141211156 and rdx, 0xfffffffffffffff8
14121115a lea rax, [rdi + rdx*4]
14121115e movaps xmm0, xmm14
141211162 shufps xmm0, xmm14, 0
141211167 xor r8d, r8d
14121116a nop word ptr [rax + rax]
141211170 movups xmmword ptr [rdi + r8*4], xmm0
141211175 movups xmmword ptr [rdi + r8*4 + 0x10], xmm0
14121117b add r8, 8
14121117f cmp rdx, r8
141211182 jne 0x141211170
141211184 cmp rcx, rdx
141211187 je 0x14121119e
141211189 lea rcx, [rdi + rbx*4]
14121118d nop dword ptr [rax]
141211190 movss dword ptr [rax], xmm14
141211195 add rax, 4
141211199 cmp rax, rcx
14121119c jne 0x141211190
14121119e mov r15, qword ptr [rbp + 0x130]
1412111a5 mov rsi, r15
1412111a8 sub rsi, rbx
1412111ab jbe 0x141211255
1412111b1 mov rcx, qword ptr [rbp + 0x1e8]
1412111b8 sub rcx, rbx
1412111bb mov rax, rbx
1412111be cmp rcx, rsi
1412111c1 jb 0x1412116a7
1412111c7 mov r10, qword ptr [rbp + 0x1f0]
1412111ce lea rcx, [r10 + rax*4]
1412111d2 cmp rsi, 2
1412111d6 jb 0x141211244
1412111d8 mov r8, rbx
1412111db not r8
1412111de add r8, r15
1412111e1 mov edx, 1
1412111e6 cmp r8, 7
1412111ea jbe 0x141211229
1412111ec mov r9, r8
1412111ef and r9, 0xfffffffffffffff8
1412111f3 lea rcx, [rcx + r9*4]
1412111f7 lea rdx, [r9 + 1]
1412111fb movaps xmm0, xmm14
1412111ff shufps xmm0, xmm14, 0
141211204 lea r10, [r10 + rax*4]
141211208 add r10, 0x10
14121120c xor r11d, r11d
14121120f nop 
141211210 movups xmmword ptr [r10 + r11*4 - 0x10], xmm0
141211216 movups xmmword ptr [r10 + r11*4], xmm0
14121121b add r11, 8
14121121f cmp r9, r11
141211222 jne 0x141211210
141211224 cmp r8, r9
141211227 je 0x14121123e
141211229 add rdx, rbx
14121122c sub rdx, r15
14121122f nop 
141211230 movss dword ptr [rcx], xmm14
141211235 add rcx, 4
141211239 inc rdx
14121123c jne 0x141211230
14121123e add rax, rsi
141211241 dec rax
141211244 movss dword ptr [rcx], xmm14
141211249 inc rax
14121124c mov r15, qword ptr [rbp + 0x130]
141211253 jmp 0x141211258
141211255 mov rax, r15
141211258 mov qword ptr [rbp + 0x3d8], rax
14121125f mov qword ptr [rbp + 0x1f8], rax
141211266 mov rsi, qword ptr [rbp + 0x190]
14121126d mov rdi, r15
141211270 sub rdi, rsi
141211273 jbe 0x141211396
141211279 mov rax, qword ptr [rbp + 0x180]
141211280 sub rax, rsi
141211283 mov rbx, rsi
141211286 cmp rax, rdi
141211289 jb 0x1412116ef
14121128f mov r12, qword ptr [rbp + 0x188]
141211296 lea rcx, [r12 + rbx*4]
14121129a cmp rdi, 2
14121129e jb 0x1412112c8
1412112a0 mov r8, rsi
1412112a3 not r8
1412112a6 add r8, r15
1412112a9 shl r8, 2
1412112ad xor edx, edx
1412112af call 0x142923c20
1412112b4 add r15, rbx
1412112b7 sub r15, rsi
1412112ba lea rcx, [r12 + r15*4]
1412112be add rcx, -4
1412112c2 add rbx, rdi
1412112c5 dec rbx
1412112c8 mov dword ptr [rcx], 0
1412112ce mov rsi, rbx
1412112d1 inc rsi
1412112d4 mov qword ptr [rbp + 0x190], rsi
1412112db mov r12, qword ptr [rbp + 0x130]
1412112e2 mov r15, qword ptr [rbp + 0x288]
1412112e9 mov rdi, r12
1412112ec sub rdi, r15
1412112ef jbe 0x14121138f
1412112f5 mov rcx, qword ptr [rbp + 0x278]
1412112fc sub rcx, r15
1412112ff mov rax, r15
141211302 cmp rcx, rdi
141211305 jb 0x141211731
14121130b mov r10, qword ptr [rbp + 0x280]
141211312 lea rcx, [r10 + rax*4]
141211316 cmp rdi, 2
14121131a jb 0x141211384
14121131c mov r8, r15
14121131f not r8
141211322 add r8, r12
141211325 mov edx, 1
14121132a cmp r8, 7
14121132e jbe 0x141211369
141211330 mov r9, r8
141211333 and r9, 0xfffffffffffffff8
141211337 lea rcx, [rcx + r9*4]
14121133b lea rdx, [r9 + 1]
14121133f lea r10, [r10 + rax*4]
141211343 add r10, 0x10
141211347 xor r11d, r11d
14121134a nop word ptr [rax + rax]
141211350 movups xmmword ptr [r10 + r11*4 - 0x10], xmm8
141211356 movups xmmword ptr [r10 + r11*4], xmm8
14121135b add r11, 8
14121135f cmp r9, r11
141211362 jne 0x141211350
141211364 cmp r8, r9
141211367 je 0x14121137e
141211369 add rdx, r15
14121136c sub rdx, r12
14121136f nop 
141211370 movss dword ptr [rcx], xmm11
141211375 add rcx, 4
141211379 inc rdx
14121137c jne 0x141211370
14121137e add rax, rdi
141211381 dec rax
141211384 movss dword ptr [rcx], xmm11
141211389 inc rax
14121138c mov r12, rax
14121138f mov qword ptr [rbp + 0x288], r12
141211396 mov qword ptr [rbp + 0x118], 0
1412113a1 mov rcx, qword ptr [rbp + 0x3c0]
1412113a8 call 0x140858790
1412113ad mov rcx, qword ptr [rbp + 0x130]
1412113b4 mov rdx, qword ptr [rbp + 0x288]
1412113bb cmp rcx, rdx
1412113be ja 0x1412136d8
1412113c4 cmp rcx, rsi
1412113c7 mov r8, qword ptr [rbp + 0x3d8]
1412113ce ja 0x1412136e9
1412113d4 mov rdi, qword ptr [rbp + 0x1f0]
1412113db mov rsi, qword ptr [rbp + 0x280]
1412113e2 mov r15, qword ptr [rbp + 0x188]
1412113e9 mov qword ptr [rbp + 0x2d0], 0
1412113f4 mov qword ptr [rbp + 0x2d8], 4
1412113ff mov qword ptr [rbp + 0x2e0], 0
14121140a mov dword ptr [rbp + 0x2e8], 0
141211414 mov byte ptr [rbp + 0x2ec], 0
14121141b mov qword ptr [rbp + 0x2f0], 0
141211426 xorps xmm0, xmm0
141211429 movups xmmword ptr [r13 + 0x10], xmm0
14121142e movups xmmword ptr [r13], xmm0
141211433 mov qword ptr [rbp + 0x2f8], 4
14121143e mov qword ptr [rbp + 0x300], 0
141211449 mov byte ptr [rbp + 0x214], 2
141211450 movss dword ptr [rsp + 0xa0], xmm14
14121145a lea rdx, [rbp + 0x200]
141211461 mov qword ptr [rsp + 0x78], rdx
141211466 lea rdx, [rbp + 0x108]
14121146d mov qword ptr [rsp + 0x68], rdx
141211472 lea rdx, [rbp + 0x2d0]
141211479 mov qword ptr [rsp + 0x58], rdx
14121147e mov qword ptr [rsp + 0x40], rcx
141211483 mov qword ptr [rsp + 0x38], r15
141211488 mov qword ptr [rsp + 0x30], rcx
14121148d mov qword ptr [rsp + 0x28], rsi
141211492 mov qword ptr [rsp + 0x20], r8
141211497 mov byte ptr [rsp + 0xa8], 0
14121149f mov dword ptr [rsp + 0x90], 0
1412114aa mov dword ptr [rsp + 0x80], 0
1412114b5 mov dword ptr [rsp + 0x70], 0
1412114bd mov dword ptr [rsp + 0x60], 0x3e4ccccd
1412114c5 mov qword ptr [rsp + 0x50], 0
1412114ce mov byte ptr [rsp + 0x48], 0
1412114d3 lea rcx, [rbp + 0x2b8]
1412114da lea rdx, [rbp + 0x120]
1412114e1 mov r8d, eax
1412114e4 mov r9, rdi
1412114e7 call 0x140769760
1412114ec mov rax, qword ptr [rbp + 0x2d0]
1412114f3 test rax, rax
1412114f6 je 0x141211512
1412114f8 mov rcx, qword ptr [rbp + 0x2d8]
1412114ff shl rax, 2
141211503 lea rdx, [rax + rax*4]
141211507 mov r8d, 4
14121150d call 0x140613c20
141211512 mov rax, qword ptr [rbp + 0x2f0]
141211519 test rax, rax
14121151c je 0x141211538
14121151e mov rcx, qword ptr [rbp + 0x2f8]
141211525 shl rax, 2
141211529 lea rdx, [rax + rax*4]
14121152d mov r8d, 4
141211533 call 0x140613c20
141211538 mov r12, qword ptr [rbp + 0x110]
14121153f mov rax, qword ptr [rbp + 0x118]
141211546 lea rbx, [rax + rax*2]
14121154a shl rbx, 5
14121154e add rbx, r12
141211551 cmp r12, rbx
141211554 je 0x14121161c
14121155a nop word ptr [rax + rax]
141211560 movups xmm0, xmmword ptr [r12]
141211565 movups xmm1, xmmword ptr [r12 + 0x10]
14121156b movups xmm2, xmmword ptr [r12 + 0x20]
141211571 movaps xmmword ptr [rbp + 0x380], xmm2
141211578 movaps xmmword ptr [rbp + 0x370], xmm1
14121157f movaps xmmword ptr [rbp + 0x360], xmm0
141211586 movq xmm6, qword ptr [r12 + 0x4c]
14121158d lea rcx, [rbp + 0x2d0]
141211594 call 0x1408587c0
141211599 movaps xmm0, xmmword ptr [rbp + 0x360]
1412115a0 movaps xmm1, xmmword ptr [rbp + 0x370]
1412115a7 movaps xmm2, xmmword ptr [rbp + 0x380]
1412115ae movaps xmmword ptr [rbp + 0x220], xmm2
1412115b5 movaps xmmword ptr [rbp + 0x210], xmm1
1412115bc movaps xmmword ptr [rbp + 0x200], xmm0
1412115c3 movups xmm0, xmmword ptr [r13 - 8]
1412115c8 movups xmmword ptr [r14], xmm0
1412115cc mov eax, dword ptr [r13 + 0x10]
1412115d0 mov dword ptr [r14 + 0x18], eax
1412115d4 mov rax, qword ptr [r13 + 8]
1412115d8 mov qword ptr [r14 + 0x10], rax
1412115dc movq xmm0, xmm6
1412115e0 movdqu xmmword ptr [rbp + 0x24c], xmm0
1412115e8 mov eax, dword ptr [rbp + 0x32c]
1412115ee mov dword ptr [rbp + 0x25c], eax
1412115f4 mov rcx, qword ptr [rbp + 0x518]
1412115fb lea rdx, [rbp + 0x200]
141211602 call 0x14120c120
141211607 test al, al
141211609 jne 0x141211773
14121160f add r12, 0x60
141211613 cmp r12, rbx
141211616 jne 0x141211560
14121161c mov rdx, qword ptr [rbp + 0x120]
141211623 test rdx, rdx
141211626 je 0x14121163e
141211628 mov rcx, qword ptr [rbp + 0x128]
14121162f shl rdx, 3
141211633 mov r8d, 4
141211639 call 0x140613c20
14121163e mov rdx, qword ptr [rbp + 0x138]
141211645 test rdx, rdx
141211648 mov r12, qword ptr [rbp + 0x198]
14121164f mov rsi, qword ptr [rbp + 0x1a0]
141211656 lea rbx, [rbp + 0x360]
14121165d mov r15d, 0
141211663 je 0x14121167b
141211665 mov rcx, qword ptr [rbp + 0x140]
14121166c shl rdx, 2
141211670 mov r8d, 4
141211676 call 0x140613c20
14121167b movaps xmm6, xmm12
14121167f movss xmm0, dword ptr [rip + 0x186dbc9]
141211687 movaps xmm1, xmm15
14121168b lea r9, [rip + 0x1a074fe]
141211692 mov r8, qword ptr [rbp + 0x3d0]
141211699 cmp r8, rsi
14121169c jb 0x141210d89
1412116a2 jmp 0x141213436
1412116a7 mov qword ptr [rsp + 0x20], 4
1412116b0 mov r9d, 4
1412116b6 lea rcx, [rbp + 0x1e8]
1412116bd mov rdi, rbx
1412116c0 mov rdx, rbx
1412116c3 mov r8, rsi
1412116c6 call 0x1428a7de0
1412116cb mov rax, qword ptr [rbp + 0x1f8]
1412116d2 mov rbx, rdi
1412116d5 mov r10, qword ptr [rbp + 0x1f0]
1412116dc lea rcx, [r10 + rax*4]
1412116e0 cmp rsi, 2
1412116e4 jae 0x1412111d8
1412116ea jmp 0x141211244
1412116ef mov qword ptr [rsp + 0x20], 4
1412116f8 mov r9d, 4
1412116fe lea rcx, [rbp + 0x180]
141211705 mov rdx, rsi
141211708 mov r8, rdi
14121170b call 0x1428a7de0
141211710 mov rbx, qword ptr [rbp + 0x190]
141211717 mov r12, qword ptr [rbp + 0x188]
14121171e lea rcx, [r12 + rbx*4]
141211722 cmp rdi, 2
141211726 jae 0x1412112a0
14121172c jmp 0x1412112c8
141211731 mov qword ptr [rsp + 0x20], 4
14121173a mov r9d, 4
141211740 lea rcx, [rbp + 0x278]
141211747 mov rdx, r15
14121174a mov r8, rdi
14121174d call 0x1428a7de0
141211752 mov rax, qword ptr [rbp + 0x288]
141211759 mov r10, qword ptr [rbp + 0x280]
141211760 lea rcx, [r10 + rax*4]
141211764 cmp rdi, 2
141211768 jae 0x14121131c
14121176e jmp 0x141211384
141211773 mov rax, qword ptr [rbp + 0x520]
14121177a mov byte ptr [rax], 1
14121177d mov rdx, qword ptr [rbp + 0x120]
141211784 test rdx, rdx
141211787 je 0x14121179f
141211789 mov rcx, qword ptr [rbp + 0x128]
141211790 shl rdx, 3
141211794 mov r8d, 4
14121179a call 0x140613c20
14121179f mov rdx, qword ptr [rbp + 0x138]
1412117a6 test rdx, rdx
1412117a9 mov r14, qword ptr [rbp + 0x2b0]
1412117b0 mov r12, qword ptr [rbp + 0x198]
1412117b7 mov r13, qword ptr [rbp + 0x3c8]
1412117be je 0x1412117d6
1412117c0 mov rcx, qword ptr [rbp + 0x140]
1412117c7 shl rdx, 2
1412117cb mov r8d, 4
1412117d1 call 0x140613c20
1412117d6 mov rdx, qword ptr [rbp + 0x2b8]
1412117dd test rdx, rdx
1412117e0 je 0x1412117f8
1412117e2 mov rcx, qword ptr [rbp + 0x2c0]
1412117e9 shl rdx, 4
1412117ed mov r8d, 4
1412117f3 call 0x140613c20
1412117f8 mov rdx, qword ptr [rbp + 0x1e8]
1412117ff test rdx, rdx
141211802 je 0x141211816
141211804 shl rdx, 2
141211808 mov r8d, 4
14121180e mov rcx, rdi
141211811 call 0x140613c20
141211816 mov rdx, qword ptr [rbp + 0x278]
14121181d test rdx, rdx
141211820 je 0x141211834
141211822 shl rdx, 2
141211826 mov r8d, 4
14121182c mov rcx, rsi
14121182f call 0x140613c20
141211834 mov rdx, qword ptr [rbp + 0x180]
14121183b test rdx, rdx
14121183e je 0x141211852
141211840 shl rdx, 2
141211844 mov r8d, 4
14121184a mov rcx, r15
14121184d call 0x140613c20
141211852 mov rax, qword ptr [rbp + 0x108]
141211859 test rax, rax
14121185c je 0x141211878
14121185e mov rcx, qword ptr [rbp + 0x110]
141211865 shl rax, 5
141211869 lea rdx, [rax + rax*2]
14121186d mov r8d, 4
141211873 call 0x140613c20
141211878 mov rbx, qword ptr [rbp + 0x1c0]
14121187f test rbx, rbx
141211882 je 0x1412139cd
141211888 mov rdi, qword ptr [rbp + 0x1b8]
14121188f lea rax, [rbx + rbx*2]
141211893 lea rsi, [rdi + rax*4]
141211897 cmp rsi, 0xc
14121189b movss xmm3, dword ptr [rip + 0x1714061]
1412118a3 je 0x1412139e2
1412118a9 movss xmm1, dword ptr [rsi - 0xc]
1412118ae movss xmm0, dword ptr [rsi - 4]
1412118b3 unpcklps xmm0, xmm1
1412118b6 movss xmm1, dword ptr [rdi]
1412118ba movss xmm2, dword ptr [rdi + 8]
1412118bf unpcklps xmm2, xmm1
1412118c2 subps xmm0, xmm2
1412118c5 movaps xmm1, xmm0
1412118c8 mulps xmm1, xmm0
1412118cb movshdup xmm2, xmm1
1412118cf addss xmm2, xmm1
1412118d3 sqrtss xmm2, xmm2
1412118d7 movaps xmm1, xmm3
1412118da divss xmm1, xmm2
1412118de movd eax, xmm1
1412118e2 test eax, eax
1412118e4 sets cl
1412118e7 mov edx, eax
1412118e9 and edx, 0x7fffffff
1412118ef add edx, 0xff800000
1412118f5 cmp edx, 0x7f000000
1412118fb setae dl
1412118fe or dl, cl
141211900 dec eax
141211902 cmp eax, 0x7fffff
141211907 setae al
14121190a test al, dl
14121190c jne 0x141213217
141211912 movaps xmm2, xmmword ptr [rip + 0x1713fd7]
141211919 xorps xmm2, xmm1
14121191c unpcklps xmm2, xmm1
14121191f mulps xmm0, xmm2
141211922 movlps qword ptr [rbp + 0x200], xmm0
141211929 lea rcx, [rbp + 0x2d0]
141211930 lea rdx, [rbp + 0x200]
141211937 call 0x140c9cdc0
14121193c shl rbx, 2
141211940 lea rcx, [rbx + rbx*2]
141211944 test rcx, rcx
141211947 je 0x141211a3f
14121194d movsd xmm0, qword ptr [rbp + 0x2d0]
141211955 mulps xmm0, xmmword ptr [rip + 0x1a0a4c4]
14121195c movss xmm1, dword ptr [rbp + 0x2d8]
141211964 mulss xmm1, dword ptr [rip + 0x1a0a4c4]
14121196c add rcx, -0xc
141211970 movabs rdx, 0xaaaaaaaaaaaaaaab
14121197a mov rax, rcx
14121197d mul rdx
141211980 shr edx, 3
141211983 inc edx
141211985 and edx, 3
141211988 je 0x1412119c2
14121198a shl edx, 2
14121198d lea rax, [rdx + rdx*2]
141211991 nop word ptr cs:[rax + rax]
1412119a0 movsd xmm2, qword ptr [rdi]
1412119a4 addps xmm2, xmm0
1412119a7 movlps qword ptr [rdi], xmm2
1412119aa movss xmm2, dword ptr [rdi + 8]
1412119af addss xmm2, xmm1
1412119b3 movss dword ptr [rdi + 8], xmm2
1412119b8 add rdi, 0xc
1412119bc add rax, -0xc
1412119c0 jne 0x1412119a0
1412119c2 cmp rcx, 0x24
1412119c6 jb 0x141211a3f
1412119c8 nop dword ptr [rax + rax]
1412119d0 movsd xmm2, qword ptr [rdi]
1412119d4 movsd xmm3, qword ptr [rdi + 0xc]
1412119d9 movsd xmm4, qword ptr [rdi + 0x18]
1412119de movsd xmm5, qword ptr [rdi + 0x24]
1412119e3 addps xmm2, xmm0
1412119e6 movlps qword ptr [rdi], xmm2
1412119e9 movss xmm2, dword ptr [rdi + 8]
1412119ee addss xmm2, xmm1
1412119f2 movss dword ptr [rdi + 8], xmm2
1412119f7 addps xmm3, xmm0
1412119fa movlps qword ptr [rdi + 0xc], xmm3
1412119fe movss xmm2, dword ptr [rdi + 0x14]
141211a03 addss xmm2, xmm1
141211a07 movss dword ptr [rdi + 0x14], xmm2
141211a0c addps xmm4, xmm0
141211a0f movlps qword ptr [rdi + 0x18], xmm4
141211a13 movss xmm2, dword ptr [rdi + 0x20]
141211a18 addss xmm2, xmm1
141211a1c movss dword ptr [rdi + 0x20], xmm2
141211a21 addps xmm5, xmm0
141211a24 movlps qword ptr [rdi + 0x24], xmm5
141211a28 movss xmm2, dword ptr [rdi + 0x2c]
141211a2d addss xmm2, xmm1
141211a31 movss dword ptr [rdi + 0x2c], xmm2
141211a36 add rdi, 0x30
141211a3a cmp rdi, rsi
141211a3d jne 0x1412119d0
141211a3f movss xmm0, dword ptr [rbp + 0x39c]
141211a47 ucomiss xmm0, dword ptr [rip + 0x1a0a3e6]
141211a4e jbe 0x141211ae1
141211a54 mov rsi, qword ptr [rbp + 0x1c0]
141211a5b movzx eax, byte ptr [rip + 0x24231a7]
141211a62 mov ecx, 0x30
141211a67 mov edx, 8
141211a6c call 0x140613c10
141211a71 test rax, rax
141211a74 je 0x141213ab5
141211a7a mov rcx, rax
141211a7d movss xmm0, dword ptr [rip + 0x171f2eb]
141211a85 divss xmm0, dword ptr [rbp + 0x39c]
141211a8d minss xmm0, dword ptr [rip + 0x1713eef]
141211a95 shr rsi, 1
141211a98 cmp rsi, 3
141211a9c mov eax, 2
141211aa1 cmovb rsi, rax
141211aa5 movss xmm1, dword ptr [rip + 0x1713dc3]
141211aad movaps xmm2, xmm1
141211ab0 subss xmm2, xmm0
141211ab4 addss xmm0, xmm1
141211ab8 mov dword ptr [rcx], 0
141211abe movss dword ptr [rcx + 4], xmm2
141211ac3 mov qword ptr [rcx + 8], rsi
141211ac7 mov byte ptr [rcx + 0x10], 1
141211acb movss dword ptr [rcx + 0x18], xmm0
141211ad0 mov dword ptr [rcx + 0x1c], 0x3f800000
141211ad7 mov qword ptr [rcx + 0x20], rsi
141211adb mov byte ptr [rcx + 0x28], 1
141211adf jmp 0x141211b23
141211ae1 movzx eax, byte ptr [rip + 0x2423121]
141211ae8 mov ecx, 0x18
141211aed mov edx, 8
141211af2 call 0x140613c10
141211af7 test rax, rax
141211afa je 0x141213ac9
141211b00 mov rcx, rax
141211b03 mov rax, qword ptr [rbp + 0x1c0]
141211b0a movsd xmm0, qword ptr [rip + 0x1722a9e]
141211b12 movsd qword ptr [rcx], xmm0
141211b16 mov qword ptr [rcx + 8], rax
141211b1a mov byte ptr [rcx + 0x10], 0
141211b1e mov eax, 1
141211b23 mov qword ptr [rbp + 0xe0], rax
141211b2a lea rax, [rax + rax*2]
141211b2e lea rax, [rcx + rax*8]
141211b32 mov qword ptr [rbp + 0x48], rax
141211b36 lea rdi, [rbp + 0x2dc]
141211b3d lea rax, [r12 + 0x88]
141211b45 mov qword ptr [rbp + 0x1a0], rax
141211b4c mov rax, qword ptr [rbp + 0xe8]
141211b53 mov rdx, qword ptr [rax + 0x38]
141211b57 mov rax, qword ptr [rax + 0x40]
141211b5b mov qword ptr [rbp + 0x60], rdx
141211b5f mov qword ptr [rbp + 0x58], rax
141211b63 lea rdx, [rdx + rax*4 - 4]
141211b68 xorps xmm14, xmm14
141211b6c mov rax, qword ptr [rbp + 0x2a8]
141211b73 lea rax, [rax + r13*4]
141211b77 mov qword ptr [rbp + 0x50], rax
141211b7b mov qword ptr [rbp + 0x3d0], 0
141211b86 mov qword ptr [rbp + 0x100], rcx
141211b8d mov rbx, rcx
141211b90 mov qword ptr [rbp + 0x68], rdx
141211b94 jmp 0x141211bce
141211b96 nop word ptr cs:[rax + rax]
141211ba0 mov rax, qword ptr [rbp + 0x298]
141211ba7 mov qword ptr [rax + rsi*8], rbx
141211bab inc rsi
141211bae mov qword ptr [rbp + 0x2a0], rsi
141211bb5 mov rbx, qword ptr [rbp + 0xe8]
141211bbc mov rax, qword ptr [rbp + 0x70]
141211bc0 mov qword ptr [rbp + 0x3d0], rax
141211bc7 mov r13, qword ptr [rbp + 0x3c8]
141211bce cmp rbx, qword ptr [rbp + 0x48]
141211bd2 je 0x141212cd8
141211bd8 mov qword ptr [rbp + 0x120], 0
141211be3 mov qword ptr [rbp + 0x128], 4
141211bee lea rax, [rbp + 0x130]
141211bf5 xorps xmm0, xmm0
141211bf8 movups xmmword ptr [rax], xmm0
141211bfb mov qword ptr [rbp + 0x140], 4
141211c06 movups xmmword ptr [rax + 0x18], xmm0
141211c0a mov qword ptr [rbp + 0x158], 4
141211c15 movups xmmword ptr [rax + 0x30], xmm0
141211c19 mov qword ptr [rbp + 0x170], 4
141211c24 mov qword ptr [rbp + 0x178], 0
141211c2f mov r15, qword ptr [rbp + 0x1c0]
141211c36 test r15, r15
141211c39 movss xmm3, dword ptr [rip + 0x1713cc3]
141211c41 je 0x14121388e
141211c47 mov r14, qword ptr [rbp + 0x1b8]
141211c4e lea rax, [r15 + r15*2]
141211c52 lea rax, [r14 + rax*4]
141211c56 cmp rax, 0xc
141211c5a lea rdx, [rbp + 0x2d0]
141211c61 je 0x1412138aa
141211c67 movss xmm11, dword ptr [rbp + 0x1e0]
141211c70 movss xmm0, dword ptr [rax - 0xc]
141211c75 movss xmm1, dword ptr [rax - 4]
141211c7a unpcklps xmm1, xmm0
141211c7d movss xmm0, dword ptr [r14]
141211c82 movss xmm2, dword ptr [r14 + 8]
141211c88 unpcklps xmm2, xmm0
141211c8b subps xmm1, xmm2
141211c8e movaps xmm0, xmm1
141211c91 mulps xmm0, xmm1
141211c94 movshdup xmm2, xmm0
141211c98 addss xmm2, xmm0
141211c9c xorps xmm0, xmm0
141211c9f sqrtss xmm0, xmm2
141211ca3 movaps xmm2, xmm3
141211ca6 divss xmm2, xmm0
141211caa movaps xmm0, xmm2
141211cad xorps xmm0, xmmword ptr [rip + 0x1713c3c]
141211cb4 unpcklps xmm0, xmm2
141211cb7 mulps xmm0, xmm1
141211cba movlps qword ptr [rbp + 0x2d0], xmm0
141211cc1 mov byte ptr [rbp + 0x3e7], 1
141211cc8 lea rcx, [rbp + 0x2b8]
141211ccf call 0x140c9cdc0
141211cd4 lea rax, [rbx + 0x18]
141211cd8 mov qword ptr [rbp + 0xe8], rax
141211cdf mov rax, qword ptr [rbp + 0x3d0]
141211ce6 inc rax
141211ce9 mov qword ptr [rbp + 0x70], rax
141211ced mov r13, qword ptr [rbx + 8]
141211cf1 movzx eax, byte ptr [rbx + 0x10]
141211cf5 mov byte ptr [rbp + 0x3d8], al
141211cfb movss xmm10, dword ptr [rbx]
141211d00 movss xmm0, dword ptr [rbx + 4]
141211d05 movss dword ptr [rbp + 0x3c0], xmm0
141211d0d xor r12d, r12d
141211d10 xor eax, eax
141211d12 movaps xmm8, xmm12
141211d16 cmp r12, r13
141211d19 movss xmm3, dword ptr [rip + 0x1713be3]
141211d21 jae 0x14121218b
141211d27 nop word ptr [rax + rax]
141211d30 test r12, r12
141211d33 js 0x141211d60
141211d35 xorps xmm6, xmm6
141211d38 cvtsi2ss xmm6, r12
141211d3d mov rcx, qword ptr [rbx + 8]
141211d41 test rcx, rcx
141211d44 jns 0x141211d84
141211d46 mov rdx, rcx
141211d49 shr rdx, 1
141211d4c and ecx, 1
141211d4f or rcx, rdx
141211d52 xorps xmm0, xmm0
141211d55 cvtsi2ss xmm0, rcx
141211d5a addss xmm0, xmm0
141211d5e jmp 0x141211d8c
141211d60 mov rcx, r12
141211d63 shr rcx, 1
141211d66 mov edx, r12d
141211d69 and edx, 1
141211d6c or rdx, rcx
141211d6f xorps xmm6, xmm6
141211d72 cvtsi2ss xmm6, rdx
141211d77 addss xmm6, xmm6
141211d7b mov rcx, qword ptr [rbx + 8]
141211d7f test rcx, rcx
141211d82 js 0x141211d46
141211d84 xorps xmm0, xmm0
141211d87 cvtsi2ss xmm0, rcx
141211d8c lea rcx, [r12 + 1]
141211d91 addss xmm0, dword ptr [rip + 0x174834b]
141211d99 divss xmm6, xmm0
141211d9d movss xmm1, dword ptr [rbx]
141211da1 movss xmm7, dword ptr [rbx + 4]
141211da6 movaps xmm2, xmm3
141211da9 subss xmm2, xmm6
141211dad mulss xmm2, xmm1
141211db1 mulss xmm6, xmm7
141211db5 addss xmm6, xmm2
141211db9 test al, 1
141211dbb je 0x141211de0
141211dbd mov r12, rcx
141211dc0 movaps xmm7, xmm6
141211dc3 movaps xmm6, xmm8
141211dc7 movss dword ptr [rbp + 0x3a0], xmm6
141211dcf ucomiss xmm6, xmm14
141211dd3 jae 0x141211e45
141211dd5 jmp 0x1412132fe
141211dda nop word ptr [rax + rax]
141211de0 cmp rcx, r13
141211de3 jae 0x14121218b
141211de9 test rcx, rcx
141211dec js 0x141211e00
141211dee xorps xmm2, xmm2
141211df1 cvtsi2ss xmm2, rcx
141211df6 jmp 0x141211e18
141211df8 nop dword ptr [rax + rax]
141211e00 mov rax, rcx
141211e03 shr rax, 1
141211e06 and ecx, 1
141211e09 or rcx, rax
141211e0c xorps xmm2, xmm2
141211e0f cvtsi2ss xmm2, rcx
141211e14 addss xmm2, xmm2
141211e18 add r12, 2
141211e1c divss xmm2, xmm0
141211e20 movaps xmm0, xmm3
141211e23 subss xmm0, xmm2
141211e27 mulss xmm1, xmm0
141211e2b mulss xmm7, xmm2
141211e2f addss xmm7, xmm1
141211e33 movss dword ptr [rbp + 0x3a0], xmm6
141211e3b ucomiss xmm6, xmm14
141211e3f jb 0x1412132fe
141211e45 ucomiss xmm3, xmm6
141211e48 jb 0x1412132fe
141211e4e ucomiss xmm6, xmm6
141211e51 jp 0x141213376
141211e57 mov byte ptr [rbp + 0x3e7], 1
141211e5e lea rcx, [rbp + 0x1b0]
141211e65 movaps xmm1, xmm6
141211e68 call 0x1411f8c10
141211e6d movaps xmm8, xmm0
141211e71 movss dword ptr [rbp + 0x3a0], xmm7
141211e79 ucomiss xmm7, xmm14
141211e7d movss xmm0, dword ptr [rip + 0x1713a7f]
141211e85 jb 0x14121339a
141211e8b ucomiss xmm0, xmm7
141211e8e jb 0x14121339a
141211e94 ucomiss xmm7, xmm7
141211e97 jp 0x141213412
141211e9d mov esi, eax
141211e9f mov byte ptr [rbp + 0x3e7], 1
141211ea6 lea rcx, [rbp + 0x1b0]
141211ead movaps xmm1, xmm7
141211eb0 call 0x1411f8c10
141211eb5 mov r10d, esi
141211eb8 mov rdx, qword ptr [rbp + 0x3c8]
141211ebf cmp rdx, r10
141211ec2 jbe 0x1412136fd
141211ec8 lea r9, [r10 + 1]
141211ecc cmp r9, rdx
141211ecf jae 0x141213718
141211ed5 mov ecx, eax
141211ed7 cmp rdx, rcx
141211eda jbe 0x141213733
141211ee0 lea rax, [rcx + 1]
141211ee4 cmp rax, rdx
141211ee7 jae 0x1412137c0
141211eed cmp r15, r10
141211ef0 jbe 0x141213784
141211ef6 cmp r9, r15
141211ef9 jae 0x14121374b
141211eff cmp r15, rcx
141211f02 jbe 0x141213769
141211f08 cmp rax, r15
141211f0b jae 0x1412137a2
141211f11 mov r8, qword ptr [rbp + 0x2a8]
141211f18 movss xmm2, dword ptr [r8 + r10*4]
141211f1e movss xmm1, dword ptr [r8 + r10*4 + 4]
141211f25 subss xmm1, xmm2
141211f29 mulss xmm1, xmm8
141211f2e addss xmm1, xmm2
141211f32 movss xmm3, dword ptr [r8 + rcx*4]
141211f38 movss xmm2, dword ptr [r8 + rcx*4 + 4]
141211f3f subss xmm2, xmm3
141211f43 mulss xmm2, xmm0
141211f47 addss xmm2, xmm3
141211f4b lea rdx, [r10 + r10*2]
141211f4f movss xmm5, dword ptr [r14 + rdx*4 + 8]
141211f56 lea r8, [r9 + r9*2]
141211f5a movss xmm3, dword ptr [r14 + r8*4 + 8]
141211f61 subss xmm3, xmm5
141211f65 movsd xmm9, qword ptr [r14 + rdx*4]
141211f6b movsd xmm12, qword ptr [r14 + r8*4]
141211f71 subps xmm12, xmm9
141211f75 movsldup xmm4, xmm8
141211f7a mulps xmm4, xmm12
141211f7e mulss xmm3, xmm8
141211f83 addps xmm4, xmm9
141211f87 addss xmm3, xmm5
141211f8b lea rcx, [rcx + rcx*2]
141211f8f movss xmm9, dword ptr [r14 + rcx*4 + 8]
141211f96 lea rax, [rax + rax*2]
141211f9a movss xmm8, dword ptr [r14 + rax*4 + 8]
141211fa1 subss xmm8, xmm9
141211fa6 movaps xmm5, xmm1
141211fa9 mulss xmm5, xmm14
141211fae movlps qword ptr [rbp + 0x200], xmm4
141211fb5 movss dword ptr [rbp + 0x208], xmm3
141211fbd movsd xmm12, qword ptr [r14 + rcx*4]
141211fc3 movsd xmm13, qword ptr [r14 + rax*4]
141211fc9 subps xmm13, xmm12
141211fcd movsldup xmm15, xmm0
141211fd2 mulps xmm15, xmm13
141211fd6 addps xmm15, xmm12
141211fda movshdup xmm12, xmm15
141211fdf addss xmm12, xmm2
141211fe4 xorps xmm13, xmm13
141211fe8 movss xmm13, xmm0
141211fed unpcklps xmm8, xmm2
141211ff1 mulps xmm8, xmm13
141211ff5 movsldup xmm0, xmm15
141211ffa movss xmm0, xmm9
141211fff addps xmm0, xmm8
141212003 movlhps xmm15, xmm0
141212007 movups xmmword ptr [rbp + 0x20c], xmm15
14121200f movss dword ptr [rbp + 0x21c], xmm12
141212018 shufps xmm5, xmm8, 0xd4
14121201d movaps xmm9, xmm1
141212021 shufps xmm9, xmm8, 0x30
141212026 shufps xmm5, xmm9, 0x82
14121202b shufps xmm5, xmm5, 0x64
14121202f movlhps xmm0, xmm4
141212032 shufps xmm3, xmm4, 0xd4
141212036 shufps xmm0, xmm3, 0x28
14121203a addps xmm0, xmm5
14121203d movups xmmword ptr [rbp + 0x220], xmm0
141212044 test byte ptr [rbp + 0x3d8], 1
14121204b je 0x141212070
14121204d ucomiss xmm10, xmm14
141212051 jne 0x141212075
141212053 jp 0x141212075
141212055 movss xmm0, dword ptr [rbp + 0x3c0]
14121205d movaps xmm3, xmm0
141212060 subss xmm3, xmm6
141212064 subss xmm0, xmm7
141212068 movaps xmm6, xmm3
14121206b jmp 0x141212090
14121206d nop dword ptr [rax]
141212070 movaps xmm0, xmm7
141212073 jmp 0x141212090
141212075 subss xmm6, xmm10
14121207a movaps xmm0, xmm7
14121207d subss xmm0, xmm10
141212082 nop word ptr cs:[rax + rax]
141212090 mov eax, dword ptr [rbp + 0x2c0]
141212096 mov dword ptr [rbp + 0x2d8], eax
14121209c mov rcx, qword ptr [rbp + 0x2b8]
1412120a3 mov qword ptr [rbp + 0x2d0], rcx
1412120aa mov dword ptr [rdi + 8], eax
1412120ad mov qword ptr [rdi], rcx
1412120b0 mov eax, dword ptr [rbp + 0x2c0]
1412120b6 mov dword ptr [rdi + 0x14], eax
1412120b9 mov rax, qword ptr [rbp + 0x2b8]
1412120c0 mov qword ptr [rdi + 0xc], rax
1412120c4 mov eax, dword ptr [rbp + 0x2c0]
1412120ca mov dword ptr [rdi + 0x20], eax
1412120cd mov rax, qword ptr [rbp + 0x2b8]
1412120d4 mov qword ptr [rdi + 0x18], rax
1412120d8 mulss xmm6, xmm11
1412120dd mulss xmm0, xmm11
1412120e2 movss dword ptr [rbp + 0x360], xmm6
1412120ea mov dword ptr [rbp + 0x364], 0
1412120f4 movss dword ptr [rbp + 0x368], xmm0
1412120fc mov dword ptr [rbp + 0x36c], 0
141212106 movss dword ptr [rbp + 0x370], xmm0
14121210e movss dword ptr [rbp + 0x374], xmm2
141212116 movss dword ptr [rbp + 0x378], xmm6
14121211e movss dword ptr [rbp + 0x37c], xmm1
141212126 movsd xmm0, qword ptr [rip + 0x1a09d22]
14121212e movsd qword ptr [rbp + 0x3a0], xmm0
141212136 mov dword ptr [rbp + 0x3a8], 0x3d0e8a72
141212140 mov byte ptr [rbp + 0x3e7], 1
141212147 lea rax, [rbp + 0x3a0]
14121214e mov qword ptr [rsp + 0x20], rax
141212153 lea rcx, [rbp + 0x120]
14121215a lea rdx, [rbp + 0x200]
141212161 lea r8, [rbp + 0x2d0]
141212168 lea r9, [rbp + 0x360]
14121216f call 0x141214ed0
141212174 mov al, 1
141212176 movaps xmm8, xmm7
14121217a cmp r12, r13
14121217d movss xmm3, dword ptr [rip + 0x171377f]
141212185 jb 0x141211d30
14121218b movss xmm0, dword ptr [rbp + 0x39c]
141212193 ucomiss xmm0, dword ptr [rip + 0x1a09c9a]
14121219a seta al
14121219d cmp qword ptr [rbp + 0x3d0], 1
1412121a5 sete cl
1412121a8 test al, cl
1412121aa je 0x141212668
1412121b0 movss dword ptr [rbp + 0x3d8], xmm8
1412121b9 mov rcx, qword ptr [rbp + 0x1c0]
1412121c0 test rcx, rcx
1412121c3 lea rdx, [rbp + 0x2d0]
1412121ca je 0x141213951
1412121d0 mov rax, qword ptr [rbp + 0x1b8]
1412121d7 lea rcx, [rcx + rcx*2]
1412121db lea rcx, [rax + rcx*4]
1412121df cmp rcx, 0xc
1412121e3 mov r8, qword ptr [rbp + 0x100]
1412121ea je 0x141213939
1412121f0 movss xmm7, dword ptr [r8 + 4]
1412121f6 movss xmm6, dword ptr [r8 + 0x18]
1412121fc movss xmm10, dword ptr [rbp + 0x1e0]
141212205 movss xmm0, dword ptr [rcx - 0xc]
14121220a movss xmm1, dword ptr [rcx - 4]
14121220f unpcklps xmm1, xmm0
141212212 movss xmm0, dword ptr [rax]
141212216 movss xmm2, dword ptr [rax + 8]
14121221b unpcklps xmm2, xmm0
14121221e subps xmm1, xmm2
141212221 movaps xmm0, xmm1
141212224 mulps xmm0, xmm1
141212227 movshdup xmm2, xmm0
14121222b addss xmm2, xmm0
14121222f xorps xmm0, xmm0
141212232 sqrtss xmm0, xmm2
141212236 movaps xmm2, xmm3
141212239 divss xmm2, xmm0
14121223d movaps xmm0, xmm2
141212240 xorps xmm0, xmmword ptr [rip + 0x17136a9]
141212247 unpcklps xmm0, xmm2
14121224a mulps xmm0, xmm1
14121224d movlps qword ptr [rbp + 0x2d0], xmm0
141212254 mov byte ptr [rbp + 0x3e7], 1
14121225b lea rcx, [rbp + 0x278]
141212262 call 0x140c9cdc0
141212267 movss dword ptr [rbp + 0x1e8], xmm7
14121226f mov dword ptr [rbp + 0x1ec], 0x3f000000
141212279 movss dword ptr [rbp + 0x1f0], xmm6
141212281 lea rax, [rbp + 0x1f4]
141212288 mov qword ptr [rbp + 0x2d8], rax
14121228f mov qword ptr [rbp + 0x2e0], 0
14121229a lea rcx, [rbp + 0x1b0]
1412122a1 mov qword ptr [rbp + 0x2e8], rcx
1412122a8 lea rcx, [rbp + 0x278]
1412122af mov qword ptr [rbp + 0x2f0], rcx
1412122b6 mov dword ptr [rbp + 0x2f8], 0
1412122c0 lea r9, [rbp + 0x1e8]
1412122c7 cmp r9, rax
1412122ca je 0x14121265f
1412122d0 lea rax, [r9 + 4]
1412122d4 mov qword ptr [rbp + 0x2d0], rax
1412122db mov r8, qword ptr [rbp + 0x2e0]
1412122e2 lea rax, [r8 + 1]
1412122e6 mov qword ptr [rbp + 0x2e0], rax
1412122ed mov byte ptr [rbp + 0x3e7], 1
1412122f4 lea rcx, [rbp + 0x2bc]
1412122fb lea rdx, [rbp + 0x2e8]
141212302 call 0x141fdb840
141212307 cmp dword ptr [rbp + 0x2f8], 1
14121230e jne 0x141212340
141212310 lea rcx, [rbp + 0x30c]
141212317 movups xmm0, xmmword ptr [rcx]
14121231a movups xmmword ptr [rdi + 0x20], xmm0
14121231e lea rax, [rbp + 0x2bc]
141212325 movups xmm0, xmmword ptr [rax]
141212328 movups xmmword ptr [rcx], xmm0
14121232b lea rcx, [rbp + 0x2c0]
141212332 lea rdx, [rbp + 0x2c4]
141212339 jmp 0x1412123c7
14121233e nop 
141212340 lea rax, [rbp + 0x2bc]
141212347 movups xmm0, xmmword ptr [rax]
14121234a lea rax, [rbp + 0x3ac]
141212351 movups xmmword ptr [rax], xmm0
141212354 mov dword ptr [rbp + 0x3a8], 1
14121235e lea rax, [rbp + 0x2d0]
141212365 mov qword ptr [rbp + 0x3a0], rax
14121236c mov byte ptr [rbp + 0x3e7], 1
141212373 lea rcx, [rbp + 0x200]
14121237a lea rdx, [rbp + 0x3a0]
141212381 call 0x14123a430
141212386 mov eax, dword ptr [rbp + 0x220]
14121238c mov dword ptr [rdi + 0x3c], eax
14121238f movups xmm0, xmmword ptr [rbp + 0x200]
141212396 movups xmm1, xmmword ptr [rbp + 0x210]
14121239d movups xmmword ptr [rdi + 0x2c], xmm1
1412123a1 movups xmmword ptr [rdi + 0x1c], xmm0
1412123a5 cmp dword ptr [rbp + 0x2f8], 1
1412123ac lea rax, [rbp + 0x30c]
1412123b3 lea rcx, [rbp + 0x310]
1412123ba lea rdx, [rbp + 0x314]
1412123c1 jne 0x14121265f
1412123c7 movss xmm13, dword ptr [rbp + 0x2fc]
1412123d0 movsd xmm11, qword ptr [rbp + 0x300]
1412123d9 movss xmm6, dword ptr [rbp + 0x308]
1412123e1 movss xmm15, dword ptr [rax]
1412123e6 movss xmm9, dword ptr [rcx]
1412123eb movss xmm12, dword ptr [rdx]
1412123f0 movss xmm7, dword ptr [rbp + 0x318]
1412123f8 movss dword ptr [rbp + 0x2b8], xmm6
141212400 ucomiss xmm6, xmm14
141212404 jb 0x1412134cc
14121240a movss xmm0, dword ptr [rip + 0x17134f2]
141212412 ucomiss xmm0, xmm6
141212415 jb 0x1412134cc
14121241b ucomiss xmm6, xmm6
14121241e jp 0x141213544
141212424 mov byte ptr [rbp + 0x3e7], 1
14121242b lea rcx, [rbp + 0x1b0]
141212432 movaps xmm1, xmm6
141212435 call 0x1411f8c10
14121243a movss dword ptr [rbp + 0x2b8], xmm7
141212442 ucomiss xmm7, xmm14
141212446 jb 0x141213568
14121244c movaps xmm8, xmm0
141212450 movss xmm0, dword ptr [rip + 0x17134ac]
141212458 ucomiss xmm0, xmm7
14121245b jb 0x141213568
141212461 ucomiss xmm7, xmm7
141212464 jp 0x1412135e0
14121246a mov esi, eax
14121246c mov byte ptr [rbp + 0x3e7], 1
141212473 lea rcx, [rbp + 0x1b0]
14121247a movaps xmm1, xmm7
14121247d call 0x1411f8c10
141212482 mov ecx, esi
141212484 mov rdx, qword ptr [rbp + 0x3c8]
14121248b cmp rdx, rcx
14121248e jbe 0x141213822
141212494 lea r9, [rcx + 1]
141212498 cmp r9, rdx
14121249b lea r8, [rbp + 0x200]
1412124a2 jae 0x141213807
1412124a8 mov eax, eax
1412124aa cmp rdx, rax
1412124ad jbe 0x14121385c
1412124b3 lea r9, [rax + 1]
1412124b7 cmp r9, rdx
1412124ba jae 0x14121383a
1412124c0 mov rdx, qword ptr [rbp + 0x2a8]
1412124c7 movss xmm1, dword ptr [rdx + rcx*4]
1412124cc movss xmm2, dword ptr [rdx + rcx*4 + 4]
1412124d2 subss xmm2, xmm1
1412124d6 mulss xmm2, xmm8
1412124db addss xmm2, xmm1
1412124df movss xmm3, dword ptr [rdx + rax*4]
1412124e4 movss xmm1, dword ptr [rdx + rax*4 + 4]
1412124ea subss xmm1, xmm3
1412124ee mulss xmm1, xmm0
1412124f2 addss xmm1, xmm3
1412124f6 movaps xmm0, xmm1
1412124f9 mulss xmm0, xmm14
1412124fe movaps xmm3, xmm2
141212501 mulss xmm6, xmm10
141212506 mulss xmm7, xmm10
14121250b movss dword ptr [rbp + 0x3a0], xmm6
141212513 mov dword ptr [rbp + 0x3a4], 0
14121251d movss dword ptr [rbp + 0x3a8], xmm7
141212525 mov dword ptr [rbp + 0x3ac], 0
14121252f movss dword ptr [rbp + 0x3b0], xmm7
141212537 movss dword ptr [rbp + 0x3b4], xmm1
14121253f movss dword ptr [rbp + 0x3b8], xmm6
141212547 movss dword ptr [rbp + 0x3bc], xmm2
14121254f mulss xmm2, xmm14
141212554 movaps xmm4, xmm13
141212558 movlhps xmm4, xmm11
14121255c movaps xmm5, xmm15
141212560 shufps xmm5, xmm11, 0xd4
141212565 shufps xmm4, xmm5, 0x28
141212569 movaps xmmword ptr [rbp + 0x360], xmm4
141212570 movss dword ptr [rbp + 0x370], xmm9
141212579 movss dword ptr [rbp + 0x374], xmm12
141212582 unpcklps xmm12, xmm13
141212586 unpcklps xmm15, xmm9
14121258a movlhps xmm15, xmm12
14121258e unpcklps xmm0, xmm1
141212591 shufps xmm0, xmm2, 4
141212595 shufps xmm0, xmm0, 0xc4
141212599 addps xmm0, xmm15
14121259d movups xmmword ptr [rbp + 0x378], xmm0
1412125a4 unpcklps xmm3, xmm2
1412125a7 addps xmm3, xmm11
1412125ab movlps qword ptr [rbp + 0x388], xmm3
1412125b2 mov eax, dword ptr [rbp + 0x280]
1412125b8 mov dword ptr [rbp + 0x208], eax
1412125be mov rcx, qword ptr [rbp + 0x278]
1412125c5 mov qword ptr [rbp + 0x200], rcx
1412125cc lea rdx, [rbp + 0x20c]
1412125d3 mov dword ptr [rdx + 8], eax
1412125d6 mov qword ptr [rdx], rcx
1412125d9 mov eax, dword ptr [rbp + 0x280]
1412125df mov dword ptr [rdx + 0x14], eax
1412125e2 mov rax, qword ptr [rbp + 0x278]
1412125e9 mov qword ptr [rdx + 0xc], rax
1412125ed mov eax, dword ptr [rbp + 0x280]
1412125f3 mov dword ptr [rdx + 0x20], eax
1412125f6 mov rax, qword ptr [rbp + 0x278]
1412125fd mov qword ptr [rdx + 0x18], rax
141212601 movsd xmm0, qword ptr [rip + 0x1a09837]
141212609 movsd qword ptr [rbp + 0x2b8], xmm0
141212611 mov dword ptr [rbp + 0x2c0], 0x3ca64c2f
14121261b mov byte ptr [rbp + 0x3e7], 1
141212622 lea rax, [rbp + 0x2b8]
141212629 mov qword ptr [rsp + 0x20], rax
14121262e lea rcx, [rbp + 0x120]
141212635 lea rdx, [rbp + 0x360]
14121263c lea r9, [rbp + 0x3a0]
141212643 call 0x141214ed0
141212648 mov r9, qword ptr [rbp + 0x2d0]
14121264f mov rax, qword ptr [rbp + 0x2d8]
141212656 cmp r9, rax
141212659 jne 0x1412122d0
14121265f movss xmm8, dword ptr [rbp + 0x3d8]
141212668 mov rax, qword ptr [rbp + 0x120]
14121266f mov qword ptr [rbp + 0xf8], rax
141212676 mov r9, qword ptr [rbp + 0x128]
14121267d mov rax, qword ptr [rbp + 0x130]
141212684 mov rcx, qword ptr [rbp + 0x138]
14121268b mov qword ptr [rbp + 0x3d8], rcx
141212692 mov rcx, qword ptr [rbp + 0x140]
141212699 mov qword ptr [rbp + 0xc0], rcx
1412126a0 mov rbx, qword ptr [rbp + 0x148]
1412126a7 mov rcx, qword ptr [rbp + 0x150]
1412126ae mov qword ptr [rbp + 0xf0], rcx
1412126b5 mov rcx, qword ptr [rbp + 0x158]
1412126bc mov qword ptr [rbp + 0xc8], rcx
1412126c3 mov rsi, qword ptr [rbp + 0x160]
1412126ca mov rcx, qword ptr [rbp + 0x168]
1412126d1 mov qword ptr [rbp + 0xb8], rcx
1412126d8 mov rcx, qword ptr [rbp + 0x170]
1412126df mov qword ptr [rbp + 0x3c0], rcx
1412126e6 mov r14, qword ptr [rbp + 0x178]
1412126ed lea rax, [rax + rax*2]
1412126f1 lea rax, [r9 + rax*4]
1412126f5 mov byte ptr [rbp + 0x3e3], 0
1412126fc mov byte ptr [rbp + 0x3e2], 1
141212703 mov byte ptr [rbp + 0x3e1], 1
14121270a mov qword ptr [rsp + 0x20], rax
14121270f mov r8d, 4
141212715 lea r15, [rbp + 0x2d0]
14121271c mov rcx, r15
14121271f lea rdx, [rip + 0x1a09962]
141212726 mov qword ptr [rbp + 0xd0], r9
14121272d call 0x140a6c120
141212732 lea r12, [rbp + 0x200]
141212739 movaps xmm12, xmm8
14121273d mov rax, qword ptr [rbp + 0xb8]
141212744 mov qword ptr [rbp + 0x200], rax
14121274b mov rax, qword ptr [rbp + 0x3c0]
141212752 mov qword ptr [rbp + 0x208], rax
141212759 mov qword ptr [rbp + 0x210], r14
141212760 mov byte ptr [rbp + 0x3e5], 1
141212767 mov byte ptr [rbp + 0x3e4], 1
14121276e mov r8d, 0xc
141212774 mov rcx, r15
141212777 lea rdx, [rip + 0x1a0990e]
14121277e mov r9, r12
141212781 call 0x141fdb9d0
141212786 mov rax, qword ptr [rbp + 0x3d8]
14121278d mov qword ptr [rbp + 0x200], rax
141212794 mov rax, qword ptr [rbp + 0xc0]
14121279b mov qword ptr [rbp + 0x208], rax
1412127a2 mov qword ptr [rbp + 0x210], rbx
1412127a9 mov byte ptr [rbp + 0x3e5], 0
1412127b0 mov byte ptr [rbp + 0x3e4], 1
1412127b7 mov r8d, 0xd
1412127bd mov rcx, r15
1412127c0 lea rdx, [rip + 0x1a098d1]
1412127c7 mov r9, r12
1412127ca call 0x141fdb9d0
1412127cf mov rax, qword ptr [rbp + 0xf0]
1412127d6 mov qword ptr [rbp + 0x200], rax
1412127dd mov rax, qword ptr [rbp + 0xc8]
1412127e4 mov qword ptr [rbp + 0x208], rax
1412127eb mov qword ptr [rbp + 0x210], rsi
1412127f2 mov byte ptr [rbp + 0x3e5], 0
1412127f9 mov byte ptr [rbp + 0x3e4], 0
141212800 mov r8d, 9
141212806 mov rcx, r15
141212809 lea rdx, [rip + 0x1a09895]
141212810 mov r9, r12
141212813 call 0x141fdbe50
141212818 movups xmm0, xmmword ptr [rbp + 0x2d0]
14121281f movups xmm1, xmmword ptr [rbp + 0x2e0]
141212826 movups xmm2, xmmword ptr [rbp + 0x2f0]
14121282d movups xmm3, xmmword ptr [rbp + 0x300]
141212834 movaps xmmword ptr [rbp + 0x200], xmm0
14121283b movaps xmmword ptr [rbp + 0x210], xmm1
141212842 movaps xmmword ptr [rbp + 0x220], xmm2
141212849 movaps xmmword ptr [rbp + 0x230], xmm3
141212850 movups xmm0, xmmword ptr [rbp + 0x310]
141212857 movaps xmmword ptr [rbp + 0x240], xmm0
14121285e movups xmm0, xmmword ptr [rbp + 0x320]
141212865 movaps xmmword ptr [rbp + 0x250], xmm0
14121286c movups xmm0, xmmword ptr [rbp + 0x330]
141212873 movaps xmmword ptr [rbp + 0x260], xmm0
14121287a mov rax, qword ptr [rbp + 0x340]
141212881 mov qword ptr [rbp + 0x270], rax
141212888 mov rax, qword ptr [rbp + 0xf8]
14121288f test rax, rax
141212892 je 0x1412128ae
141212894 shl rax, 2
141212898 lea rdx, [rax + rax*2]
14121289c mov r8d, 4
1412128a2 mov rcx, qword ptr [rbp + 0xd0]
1412128a9 call 0x140613c20
1412128ae mov rax, qword ptr [rbp + 0x270]
1412128b5 mov qword ptr [rbp + 0x340], rax
1412128bc movaps xmm0, xmmword ptr [rbp + 0x260]
1412128c3 movaps xmmword ptr [rbp + 0x330], xmm0
1412128ca movaps xmm0, xmmword ptr [rbp + 0x250]
1412128d1 movaps xmmword ptr [rbp + 0x320], xmm0
1412128d8 movaps xmm0, xmmword ptr [rbp + 0x240]
1412128df movaps xmmword ptr [rbp + 0x310], xmm0
1412128e6 movaps xmm0, xmmword ptr [rbp + 0x200]
1412128ed movaps xmm1, xmmword ptr [rbp + 0x210]
1412128f4 movaps xmm2, xmmword ptr [rbp + 0x220]
1412128fb movaps xmm3, xmmword ptr [rbp + 0x230]
141212902 movaps xmmword ptr [rbp + 0x300], xmm3
141212909 movaps xmmword ptr [rbp + 0x2f0], xmm2
141212910 movaps xmmword ptr [rbp + 0x2e0], xmm1
141212917 movaps xmmword ptr [rbp + 0x2d0], xmm0
14121291e movabs rax, 0x8000000000000000
141212928 mov qword ptr [rbp + 0x348], rax
14121292f mov byte ptr [rbp + 0x3e7], 0
141212936 mov rcx, qword ptr [rbp + 0xb0]
14121293d mov rdx, r15
141212940 call 0x14092cc60
141212945 mov r15, rax
141212948 mov byte ptr [rbp + 0x3e7], 0
14121294f mov r8d, 0x1e
141212955 mov rcx, qword ptr [rbp + 0x500]
14121295c lea rdx, [rip + 0x1a0968d]
141212963 call 0x140937aa0
141212968 test al, 1
14121296a je 0x1412138d3
141212970 mov rsi, rdx
141212973 mov byte ptr [rbp + 0x3e7], 0
14121297a mov rcx, qword ptr [rbp + 0xb0]
141212981 call 0x140002450
141212986 mov byte ptr [rbp + 0x3e7], 0
14121298d mov rcx, r15
141212990 mov rdx, rax
141212993 call 0x140aa8200
141212998 mov r14, rax
14121299b mov rbx, rdx
14121299e mov qword ptr [rbp + 0x3a0], rax
1412129a5 mov qword ptr [rbp + 0x3a8], rdx
1412129ac lea rcx, [rbp + 0x528]
1412129b3 call 0x1400885d0
1412129b8 mov r13, rax
1412129bb mov rax, qword ptr [rbp + 0x1b8]
1412129c2 mov rcx, qword ptr [rbp + 0x1c0]
1412129c9 lea rdx, [rcx + rcx*2]
1412129cd mov r9, qword ptr [rbp + 0x3c8]
1412129d4 cmp r9, rcx
1412129d7 mov r8, rcx
1412129da cmovb r8, r9
1412129de lea rdx, [rax + rdx*4]
1412129e2 mov qword ptr [rbp + 0x200], rax
1412129e9 mov qword ptr [rbp + 0x208], rdx
1412129f0 mov rax, qword ptr [rbp + 0x2a8]
1412129f7 mov qword ptr [rbp + 0x210], rax
1412129fe mov rax, qword ptr [rbp + 0x50]
141212a02 mov qword ptr [rbp + 0x218], rax
141212a09 mov qword ptr [rbp + 0x220], 0
141212a14 mov qword ptr [rbp + 0x228], r8
141212a1b mov qword ptr [rbp + 0x230], rcx
141212a22 lea rcx, [rbp + 0x360]
141212a29 lea rdx, [rbp + 0x200]
141212a30 lea r8, [rip + 0x1a094a1]
141212a37 call 0x141204df0
141212a3c cmp qword ptr [rbp + 0x58], 0
141212a41 je 0x14121390d
141212a47 mov rax, qword ptr [rbp + 0x60]
141212a4b movss xmm1, dword ptr [rax]
141212a4f mov rcx, qword ptr [rbp + 0x1a0]
141212a56 call 0x1411f8c10
141212a5b mov r12, qword ptr [rbp + 0x198]
141212a62 mov eax, eax
141212a64 lea rcx, [rax + 1]
141212a68 mov rdx, qword ptr [r12 + 0x98]
141212a70 cmp rcx, rdx
141212a73 mov r8, qword ptr [rbp + 0x68]
141212a77 jae 0x1412138fc
141212a7d test r8, r8
141212a80 je 0x1412138eb
141212a86 mov rdx, qword ptr [r12 + 0x90]
141212a8e lea rcx, [rcx + rcx*2]
141212a92 movss xmm6, dword ptr [rdx + rcx*4 + 4]
141212a98 lea rax, [rax + rax*2]
141212a9c subss xmm6, dword ptr [rdx + rax*4 + 4]
141212aa2 movss xmm9, dword ptr [rdx + rcx*4 + 8]
141212aa9 movss xmm7, dword ptr [rdx + rcx*4]
141212aae movss xmm8, dword ptr [rdx + rax*4]
141212ab4 movss xmm10, dword ptr [rdx + rax*4 + 8]
141212abb movss xmm1, dword ptr [r8]
141212ac0 mov rcx, qword ptr [rbp + 0x1a0]
141212ac7 call 0x1411f8c10
141212acc mov eax, eax
141212ace lea rcx, [rax + 1]
141212ad2 mov rdx, qword ptr [r12 + 0x98]
141212ada cmp rcx, rdx
141212add jae 0x1412138c2
141212ae3 mov rdx, qword ptr [r12 + 0x90]
141212aeb lea rcx, [rcx + rcx*2]
141212aef movss xmm0, dword ptr [rdx + rcx*4 + 4]
141212af5 lea rax, [rax + rax*2]
141212af9 subss xmm0, dword ptr [rdx + rax*4 + 4]
141212aff movss xmm2, dword ptr [rdx + rcx*4 + 8]
141212b05 movss xmm1, dword ptr [rdx + rcx*4]
141212b0a movss xmm3, dword ptr [rdx + rax*4]
141212b0f movss xmm4, dword ptr [rdx + rax*4 + 8]
141212b15 mov rax, qword ptr [rbp + 0x538]
141212b1c imul r13d, eax
141212b20 mov rdx, qword ptr [rbp + 0x3d0]
141212b27 add edx, r13d
141212b2a movzx eax, byte ptr [r12 + 0x29a]
141212b33 mov rcx, qword ptr [rbp + 0x370]
141212b3a mov qword ptr [rdi + 0x54], rcx
141212b3e movups xmm5, xmmword ptr [rbp + 0x360]
141212b45 movups xmmword ptr [rdi + 0x44], xmm5
141212b49 movaps xmm5, xmmword ptr [rip + 0x172b570]
141212b50 movaps xmmword ptr [rbp + 0x2e0], xmm5
141212b57 mov dword ptr [rdi + 0x1c], 0
141212b5e mov qword ptr [rdi + 0x14], 0
141212b66 movabs rcx, 0x3f8000003f800000
141212b70 mov qword ptr [rdi + 0x20], rcx
141212b74 mov dword ptr [rdi + 0x28], 0x3f800000
141212b7b mov qword ptr [rbp + 0x310], r15
141212b82 mov qword ptr [rbp + 0x318], rsi
141212b89 mov qword ptr [rbp + 0x2d0], r14
141212b90 mov qword ptr [rbp + 0x2d8], rbx
141212b97 mov rcx, qword ptr [rbp + 0x528]
141212b9e mov qword ptr [rbp + 0x338], rcx
141212ba5 unpcklps xmm7, xmm9
141212ba9 unpcklps xmm8, xmm10
141212bad mulss xmm6, xmm6
141212bb1 subps xmm7, xmm8
141212bb5 movaps xmm5, xmm7
141212bb8 mulps xmm5, xmm7
141212bbb addss xmm6, xmm5
141212bbf movshdup xmm5, xmm5
141212bc3 addss xmm5, xmm6
141212bc7 sqrtss xmm5, xmm5
141212bcb movss xmm8, dword ptr [rip + 0x1712d30]
141212bd4 movaps xmm6, xmm8
141212bd8 divss xmm6, xmm5
141212bdc movsldup xmm5, xmm6
141212be0 mulps xmm5, xmm7
141212be3 movaps xmm6, xmm5
141212be6 mulps xmm6, xmm5
141212be9 movshdup xmm7, xmm6
141212bed addss xmm7, xmm6
141212bf1 xorps xmm6, xmm6
141212bf4 sqrtss xmm6, xmm7
141212bf8 movaps xmm7, xmm8
141212bfc divss xmm7, xmm6
141212c00 movsldup xmm6, xmm7
141212c04 mulps xmm6, xmm5
141212c07 unpcklps xmm1, xmm2
141212c0a mulss xmm0, xmm0
141212c0e unpcklps xmm3, xmm4
141212c11 subps xmm1, xmm3
141212c14 movaps xmm2, xmm1
141212c17 mulps xmm2, xmm1
141212c1a addss xmm0, xmm2
141212c1e movshdup xmm2, xmm2
141212c22 addss xmm2, xmm0
141212c26 xorps xmm0, xmm0
141212c29 sqrtss xmm0, xmm2
141212c2d movaps xmm2, xmm8
141212c31 divss xmm2, xmm0
141212c35 movsldup xmm0, xmm2
141212c39 mulps xmm0, xmm1
141212c3c movaps xmm1, xmm0
141212c3f mulps xmm1, xmm0
141212c42 movshdup xmm2, xmm1
141212c46 addss xmm2, xmm1
141212c4a xorps xmm1, xmm1
141212c4d sqrtss xmm1, xmm2
141212c51 movaps xmm2, xmm8
141212c55 divss xmm2, xmm1
141212c59 movsldup xmm1, xmm2
141212c5d mulps xmm1, xmm0
141212c60 movlhps xmm6, xmm1
141212c63 movaps xmmword ptr [rbp + 0x340], xmm6
141212c6a mov dword ptr [rbp + 0x350], edx
141212c70 mov byte ptr [rbp + 0x354], al
141212c76 mov byte ptr [rbp + 0x3e7], 0
141212c7d lea rcx, [rbp + 0x200]
141212c84 mov rdx, qword ptr [rbp + 0x510]
141212c8b lea r8, [rbp + 0x2d0]
141212c92 lea r9, [rip + 0x1a093bf]
141212c99 call 0x14120f7c0
141212c9e mov rbx, qword ptr [rbp + 0x220]
141212ca5 mov rsi, qword ptr [rbp + 0x2a0]
141212cac cmp rsi, qword ptr [rbp + 0x290]
141212cb3 jne 0x141211ba0
141212cb9 mov byte ptr [rbp + 0x3e7], 0
141212cc0 lea rcx, [rbp + 0x290]
141212cc7 lea rdx, [rip + 0x1a093a2]
141212cce call 0x140b81df0
141212cd3 jmp 0x141211ba0
141212cd8 mov rax, qword ptr [rbp + 0xe0]
141212cdf shl eax, 3
141212ce2 lea rdx, [rax + rax*2]
141212ce6 mov r8d, 8
141212cec mov rcx, qword ptr [rbp + 0x100]
141212cf3 call 0x140613c20
141212cf8 mov rcx, qword ptr [rbp + 0x1c0]
141212cff test rcx, rcx
141212d02 movss xmm3, dword ptr [rip + 0x1712bfa]
141212d0a je 0x141213ada
141212d10 mov rax, qword ptr [rbp + 0x1b8]
141212d17 lea rcx, [rcx + rcx*2]
141212d1b lea rcx, [rax + rcx*4]
141212d1f cmp rcx, 0xc
141212d23 je 0x141213aec
141212d29 movss xmm8, dword ptr [rcx - 0xc]
141212d2f movss xmm0, dword ptr [rcx - 8]
141212d34 movss xmm1, dword ptr [rcx - 4]
141212d39 subss xmm8, dword ptr [rax]
141212d3e subss xmm0, dword ptr [rax + 4]
141212d43 subss xmm1, dword ptr [rax + 8]
141212d48 movaps xmm2, xmm8
141212d4c mulss xmm2, xmm8
141212d51 mulss xmm0, xmm0
141212d55 addss xmm0, xmm2
141212d59 movaps xmm2, xmm1
141212d5c mulss xmm2, xmm1
141212d60 addss xmm2, xmm0
141212d64 xorps xmm0, xmm0
141212d67 sqrtss xmm0, xmm2
141212d6b divss xmm3, xmm0
141212d6f mulss xmm8, xmm3
141212d74 mulss xmm3, xmm1
141212d78 movaps xmm0, xmmword ptr [rip + 0x1712b71]
141212d7f movaps xmm12, xmm3
141212d83 xorps xmm0, xmm3
141212d86 movss dword ptr [rbp + 0x2d0], xmm0
141212d8e movss dword ptr [rbp + 0x2d4], xmm8
141212d97 lea rcx, [rbp + 0x3c]
141212d9b lea rdx, [rbp + 0x2d0]
141212da2 call 0x140c9cdc0
141212da7 test r13, r13
141212daa je 0x141212de8
141212dac xorps xmm0, xmm0
141212daf mov rax, qword ptr [rbp + 0x2a8]
141212db6 movaps xmm7, xmm0
141212db9 mov rcx, qword ptr [rbp + 0xa8]
141212dc0 movss xmm1, dword ptr [rax]
141212dc4 add rax, 4
141212dc8 cmpunordss xmm7, xmm7
141212dcd movaps xmm2, xmm7
141212dd0 andps xmm2, xmm1
141212dd3 maxss xmm1, xmm0
141212dd7 andnps xmm7, xmm1
141212dda orps xmm7, xmm2
141212ddd movaps xmm0, xmm7
141212de0 add rcx, -4
141212de4 jne 0x141212dc0
141212de6 jmp 0x141212deb
141212de8 xorps xmm7, xmm7
141212deb movss xmm0, dword ptr [rbp + 0x39c]
141212df3 ucomiss xmm0, dword ptr [rip + 0x1a0903a]
141212dfa mulss xmm7, dword ptr [rip + 0x1712b6e]
141212e02 addss xmm7, dword ptr [rip + 0x1a09056]
141212e0a jbe 0x141212f8d
141212e10 lea rcx, [rbp + 0x1b0]
141212e17 movss xmm1, dword ptr [rip + 0x1712a51]
141212e1f call 0x1411f8c10
141212e24 mov rdx, qword ptr [rbp + 0x1c0]
141212e2b mov ecx, eax
141212e2d cmp rdx, rcx
141212e30 jbe 0x141213afa
141212e36 lea rax, [rcx + 1]
141212e3a cmp rax, rdx
141212e3d jae 0x141213b16
141212e43 movaps xmm6, xmm0
141212e46 mov rdx, qword ptr [rbp + 0x1b8]
141212e4d lea rcx, [rcx + rcx*2]
141212e51 movss xmm13, dword ptr [rdx + rcx*4 + 8]
141212e58 lea rax, [rax + rax*2]
141212e5c movss xmm9, dword ptr [rdx + rax*4 + 8]
141212e63 movsd xmm14, qword ptr [rdx + rcx*4]
141212e69 movsd xmm15, qword ptr [rdx + rax*4]
141212e6f movss dword ptr [rbp + 0x200], xmm8
141212e78 movss dword ptr [rbp + 0x204], xmm12
141212e81 lea rcx, [rbp + 0x2d0]
141212e88 lea rdx, [rbp + 0x200]
141212e8f call 0x140c9cdc0
141212e94 movss xmm10, dword ptr [rbp + 0x2d8]
141212e9d movsd xmm11, qword ptr [rbp + 0x2d0]
141212ea6 movss dword ptr [rbp + 0x200], xmm8
141212eaf movss dword ptr [rbp + 0x204], xmm12
141212eb8 lea rcx, [rbp + 0x2d0]
141212ebf lea rdx, [rbp + 0x200]
141212ec6 call 0x140c9cdc0
141212ecb movsd xmm12, qword ptr [rbp + 0x2d0]
141212ed4 movss xmm8, dword ptr [rbp + 0x2d8]
141212edd movzx eax, byte ptr [rip + 0x2421d25]
141212ee4 mov ecx, 0x18
141212ee9 mov edx, 4
141212eee call 0x140613c10
141212ef3 test rax, rax
141212ef6 je 0x141213b49
141212efc mov rcx, rax
141212eff subss xmm9, xmm13
141212f04 subps xmm15, xmm14
141212f08 movsldup xmm0, xmm6
141212f0c mulps xmm0, xmm15
141212f10 mulss xmm9, xmm6
141212f15 addps xmm0, xmm14
141212f19 addss xmm9, xmm13
141212f1e xorps xmm1, xmm1
141212f21 mulss xmm10, dword ptr [rip + 0x1712a26]
141212f2a mulps xmm11, xmmword ptr [rip + 0x1712a0e]
141212f32 mulss xmm1, xmm7
141212f36 addss xmm10, xmm1
141212f3b mulss xmm8, dword ptr [rip + 0x186fccc]
141212f44 addss xmm8, xmm1
141212f49 unpcklps xmm1, xmm7
141212f4c addps xmm11, xmm1
141212f50 addps xmm11, xmm0
141212f54 addss xmm10, xmm9
141212f59 mulps xmm12, xmmword ptr [rip + 0x1a08f0f]
141212f61 addss xmm8, xmm9
141212f66 movlps qword ptr [rax], xmm11
141212f6a movss dword ptr [rax + 8], xmm10
141212f70 addps xmm12, xmm1
141212f74 addps xmm12, xmm0
141212f78 movlps qword ptr [rax + 0xc], xmm12
141212f7d movss dword ptr [rax + 0x14], xmm8
141212f83 mov eax, 2
141212f88 jmp 0x14121304d
141212f8d lea rcx, [rbp + 0x1b0]
141212f94 movss xmm1, dword ptr [rip + 0x17129b4]
141212f9c call 0x1411f8c10
141212fa1 mov rdx, qword ptr [rbp + 0x1c0]
141212fa8 mov ecx, eax
141212faa cmp rdx, rcx
141212fad jbe 0x141213b08
141212fb3 lea rax, [rcx + 1]
141212fb7 cmp rax, rdx
141212fba jae 0x141213b27
141212fc0 movaps xmm6, xmm0
141212fc3 mov rdx, qword ptr [rbp + 0x1b8]
141212fca lea rcx, [rcx + rcx*2]
141212fce movss xmm11, dword ptr [rdx + rcx*4 + 8]
141212fd5 lea rax, [rax + rax*2]
141212fd9 movsd xmm9, qword ptr [rdx + rcx*4]
141212fdf movsd xmm10, qword ptr [rdx + rax*4]
141212fe5 movss xmm8, dword ptr [rdx + rax*4 + 8]
141212fec movzx eax, byte ptr [rip + 0x2421c16]
141212ff3 mov ecx, 0xc
141212ff8 mov edx, 4
141212ffd call 0x140613c10
141213002 test rax, rax
141213005 je 0x141213b38
14121300b mov rcx, rax
14121300e subss xmm8, xmm11
141213013 mulss xmm8, xmm6
141213018 addss xmm8, xmm11
14121301d subps xmm10, xmm9
141213021 xorps xmm0, xmm0
141213024 mulss xmm0, xmm7
141213028 addss xmm8, xmm0
14121302d movsldup xmm1, xmm6
141213031 mulps xmm1, xmm10
141213035 addps xmm1, xmm9
141213039 unpcklps xmm0, xmm7
14121303c addps xmm0, xmm1
14121303f movlps qword ptr [rax], xmm0
141213042 movss dword ptr [rax + 8], xmm8
141213048 mov eax, 1
14121304d movss xmm6, dword ptr [rbp + 0x3c]
141213052 movss xmm7, dword ptr [rbp + 0x44]
141213057 mov qword ptr [rbp + 0x3d8], rax
14121305e lea eax, [rax*4]
141213065 lea r12, [rax + rax*2]
141213069 xor r13d, r13d
14121306c lea rsi, [rbp + 0x2d0]
141213073 lea rdi, [rip - 0x30ba]
14121307a mov qword ptr [rbp + 0x3c8], rcx
141213081 jmp 0x1412130b9
141213083 nop word ptr cs:[rax + rax]
141213090 mov rax, qword ptr [rbp + 0x298]
141213097 mov qword ptr [rax + r14*8], rbx
14121309b inc r14
14121309e mov qword ptr [rbp + 0x2a0], r14
1412130a5 add r13, 0xc
1412130a9 cmp r12, r13
1412130ac mov rcx, qword ptr [rbp + 0x3c8]
1412130b3 je 0x1412131f7
1412130b9 mov eax, dword ptr [rcx + r13 + 8]
1412130be mov dword ptr [rbp + 0x98], eax
1412130c4 mov rax, qword ptr [rcx + r13]
1412130c8 mov qword ptr [rbp + 0x90], rax
1412130cf mov rcx, rsi
1412130d2 movaps xmm1, xmm6
1412130d5 movaps xmm2, xmm7
1412130d8 call 0x140c90fd0
1412130dd movaps xmm8, xmmword ptr [rbp + 0x2d0]
1412130e5 call 0x14003bb80
1412130ea mov rcx, rsi
1412130ed mov rdx, qword ptr [rbp + 0x510]
1412130f4 call 0x140d146a0
1412130f9 mov r14, qword ptr [rbp + 0x2d8]
141213100 test r14, r14
141213103 jne 0x14121310c
141213105 mov r14, qword ptr [rbp + 0x2e0]
14121310c mov rbx, qword ptr [rbp + 0x2f0]
141213113 mov rax, qword ptr [r14]
141213116 mov r15, qword ptr [r14 + 0x10]
14121311a sub rax, r15
14121311d cmp rax, 0x87
141213123 jbe 0x1412131d2
141213129 mov rax, qword ptr [r14 + 8]
14121312d lea rcx, [rip - 0x4354]
141213134 mov qword ptr [rax + r15], rcx
141213138 movups xmmword ptr [rax + r15 + 8], xmm8
14121313e mov rcx, qword ptr [rbp + 0x90]
141213145 mov qword ptr [rax + r15 + 0x18], rcx
14121314a mov ecx, dword ptr [rbp + 0x98]
141213150 mov dword ptr [rax + r15 + 0x20], ecx
141213155 movabs rcx, 0x3f8000003f800000
14121315f mov qword ptr [rax + r15 + 0x24], rcx
141213164 mov dword ptr [rax + r15 + 0x2c], 0x3f800000
14121316d mov dword ptr [rax + r15 + 0x38], 0
141213176 movabs rcx, 0xb75231fda9a1fa83
141213180 mov qword ptr [rax + r15 + 0x40], rcx
141213185 mov byte ptr [rax + r15 + 0x48], 3
14121318b mov byte ptr [rax + r15 + 0x58], 0
141213191 mov qword ptr [rax + r15 + 0x68], rbx
141213196 mov qword ptr [rax + r15 + 0x78], rdi
14121319b add r15, 0x88
1412131a2 mov qword ptr [r14 + 0x10], r15
1412131a6 mov r14, qword ptr [rbp + 0x2a0]
1412131ad cmp r14, qword ptr [rbp + 0x290]
1412131b4 jne 0x141213090
1412131ba lea rcx, [rbp + 0x290]
1412131c1 lea rdx, [rip + 0x1a08e10]
1412131c8 call 0x140b81df0
1412131cd jmp 0x141213090
1412131d2 mov qword ptr [rsp + 0x20], 1
1412131db mov r8d, 0x88
1412131e1 mov r9d, 1
1412131e7 mov rcx, r14
1412131ea mov rdx, r15
1412131ed call 0x1428a7de0
1412131f2 jmp 0x141213129
1412131f7 mov rax, qword ptr [rbp + 0x3d8]
1412131fe shl eax, 2
141213201 lea rdx, [rax + rax*2]
141213205 mov r8d, 4
14121320b call 0x140613c20
141213210 mov r14, qword ptr [rbp + 0x2b0]
141213217 mov rax, qword ptr [rbp + 0x2a0]
14121321e mov rsi, qword ptr [rbp + 0xd8]
141213225 mov qword ptr [rsi + 0x10], rax
141213229 movups xmm0, xmmword ptr [rbp + 0x290]
141213230 movups xmmword ptr [rsi], xmm0
141213233 test r14, r14
141213236 je 0x141213251
141213238 shl r14, 2
14121323c mov r8d, 4
141213242 mov rcx, qword ptr [rbp + 0x2a8]
141213249 mov rdx, r14
14121324c call 0x140613c20
141213251 mov rax, qword ptr [rbp + 0x1b0]
141213258 test rax, rax
14121325b je 0x141213277
14121325d mov rcx, qword ptr [rbp + 0x1b8]
141213264 shl rax, 2
141213268 lea rdx, [rax + rax*2]
14121326c mov r8d, 4
141213272 call 0x140613c20
141213277 mov rdx, qword ptr [rbp + 0x1c8]
14121327e test rdx, rdx
141213281 je 0x141213299
141213283 mov rcx, qword ptr [rbp + 0x1d0]
14121328a shl rdx, 2
14121328e mov r8d, 4
141213294 call 0x140613c20
141213299 mov rax, rsi
14121329c movaps xmm6, xmmword ptr [rbp + 0x3f0]
1412132a3 movaps xmm7, xmmword ptr [rbp + 0x400]
1412132aa movaps xmm8, xmmword ptr [rbp + 0x410]
1412132b2 movaps xmm9, xmmword ptr [rbp + 0x420]
1412132ba movaps xmm10, xmmword ptr [rbp + 0x430]
1412132c2 movaps xmm11, xmmword ptr [rbp + 0x440]
1412132ca movaps xmm12, xmmword ptr [rbp + 0x450]
1412132d2 movaps xmm13, xmmword ptr [rbp + 0x460]
1412132da movaps xmm14, xmmword ptr [rbp + 0x470]
1412132e2 movaps xmm15, xmmword ptr [rbp + 0x480]
1412132ea add rsp, 0x518
1412132f1 pop rbx
1412132f2 pop rdi
1412132f3 pop rsi
1412132f4 pop r12
1412132f6 pop r13
1412132f8 pop r14
1412132fa pop r15
1412132fc pop rbp
1412132fd ret 
1412132fe lea rax, [rbp + 0x3a0]
141213305 mov qword ptr [rbp + 0x360], rax
14121330c lea rax, [rip - 0x486a03]
141213313 mov qword ptr [rbp + 0x368], rax
14121331a lea rax, [rip + 0x1a072b7]
141213321 mov qword ptr [rbp + 0x2d0], rax
141213328 mov qword ptr [rbp + 0x2d8], 1
141213333 mov qword ptr [rbp + 0x2f0], 0
14121333e lea rax, [rbp + 0x360]
141213345 mov qword ptr [rbp + 0x2e0], rax
14121334c mov qword ptr [rbp + 0x2e8], 1
141213357 mov byte ptr [rbp + 0x3e7], 1
14121335e lea rdx, [rip + 0x1a08e3b]
141213365 lea rcx, [rbp + 0x2d0]
14121336c call 0x1428d9390
141213371 jmp 0x141213b58
141213376 mov byte ptr [rbp + 0x3e7], 1
14121337d lea rcx, [rip + 0x1a07264]
141213384 lea r8, [rip + 0x1a08e15]
14121338b mov edx, 0x1d
141213390 call 0x1428d9430
141213395 jmp 0x141213b58
14121339a lea rax, [rbp + 0x3a0]
1412133a1 mov qword ptr [rbp + 0x360], rax
1412133a8 lea rax, [rip - 0x486a9f]
1412133af mov qword ptr [rbp + 0x368], rax
1412133b6 lea rax, [rip + 0x1a0721b]
1412133bd mov qword ptr [rbp + 0x2d0], rax
1412133c4 mov qword ptr [rbp + 0x2d8], 1
1412133cf mov qword ptr [rbp + 0x2f0], 0
1412133da lea rax, [rbp + 0x360]
1412133e1 mov qword ptr [rbp + 0x2e0], rax
1412133e8 mov qword ptr [rbp + 0x2e8], 1
1412133f3 mov byte ptr [rbp + 0x3e7], 1
1412133fa lea rdx, [rip + 0x1a08db7]
141213401 lea rcx, [rbp + 0x2d0]
141213408 call 0x1428d9390
14121340d jmp 0x141213b58
141213412 mov byte ptr [rbp + 0x3e7], 1
141213419 lea rcx, [rip + 0x1a071c8]
141213420 lea r8, [rip + 0x1a08d91]
141213427 mov edx, 0x1d
14121342c call 0x1428d9430
141213431 jmp 0x141213b58
141213436 mov rdx, qword ptr [rbp + 0x2b8]
14121343d test rdx, rdx
141213440 je 0x141213458
141213442 mov rcx, qword ptr [rbp + 0x2c0]
141213449 shl rdx, 4
14121344d mov r8d, 4
141213453 call 0x140613c20
141213458 mov rdx, qword ptr [rbp + 0x1e8]
14121345f test rdx, rdx
141213462 mov r14, qword ptr [rbp + 0x2b0]
141213469 mov r13, qword ptr [rbp + 0x3c8]
141213470 je 0x141213484
141213472 shl rdx, 2
141213476 mov r8d, 4
14121347c mov rcx, rdi
14121347f call 0x140613c20
141213484 mov rdx, qword ptr [rbp + 0x278]
14121348b test rdx, rdx
14121348e je 0x1412134a6
141213490 mov rcx, qword ptr [rbp + 0x280]
141213497 shl rdx, 2
14121349b mov r8d, 4
1412134a1 call 0x140613c20
1412134a6 mov rdx, qword ptr [rbp + 0x180]
1412134ad test rdx, rdx
1412134b0 je 0x141211852
1412134b6 mov rcx, qword ptr [rbp + 0x188]
1412134bd shl rdx, 2
1412134c1 mov r8d, 4
1412134c7 jmp 0x14121184d
1412134cc lea rax, [rbp + 0x2b8]
1412134d3 mov qword ptr [rbp + 0x3a0], rax
1412134da lea rax, [rip - 0x486bd1]
1412134e1 mov qword ptr [rbp + 0x3a8], rax
1412134e8 lea rax, [rip + 0x1a070e9]
1412134ef mov qword ptr [rbp + 0x200], rax
1412134f6 mov qword ptr [rbp + 0x208], 1
141213501 mov qword ptr [rbp + 0x220], 0
14121350c lea rax, [rbp + 0x3a0]
141213513 mov qword ptr [rbp + 0x210], rax
14121351a mov qword ptr [rbp + 0x218], 1
141213525 mov byte ptr [rbp + 0x3e7], 1
14121352c lea rdx, [rip + 0x1a08d2d]
141213533 lea rcx, [rbp + 0x200]
14121353a call 0x1428d9390
14121353f jmp 0x141213b58
141213544 mov byte ptr [rbp + 0x3e7], 1
14121354b lea rcx, [rip + 0x1a07096]
141213552 lea r8, [rip + 0x1a08d07]
141213559 mov edx, 0x1d
14121355e call 0x1428d9430
141213563 jmp 0x141213b58
141213568 lea rax, [rbp + 0x2b8]
14121356f mov qword ptr [rbp + 0x3a0], rax
141213576 lea rax, [rip - 0x486c6d]
14121357d mov qword ptr [rbp + 0x3a8], rax
141213584 lea rax, [rip + 0x1a0704d]
14121358b mov qword ptr [rbp + 0x200], rax
141213592 mov qword ptr [rbp + 0x208], 1
14121359d mov qword ptr [rbp + 0x220], 0
1412135a8 lea rax, [rbp + 0x3a0]
1412135af mov qword ptr [rbp + 0x210], rax
1412135b6 mov qword ptr [rbp + 0x218], 1
1412135c1 mov byte ptr [rbp + 0x3e7], 1
1412135c8 lea rdx, [rip + 0x1a08ca9]
1412135cf lea rcx, [rbp + 0x200]
1412135d6 call 0x1428d9390
1412135db jmp 0x141213b58
1412135e0 mov byte ptr [rbp + 0x3e7], 1
1412135e7 lea rcx, [rip + 0x1a06ffa]
1412135ee lea r8, [rip + 0x1a08c83]
1412135f5 mov edx, 0x1d
1412135fa call 0x1428d9430
1412135ff jmp 0x141213b58
141213604 lea rax, [rbp + 0x3a0]
14121360b mov qword ptr [rbp + 0x360], rax
141213612 lea rax, [rip - 0x486d09]
141213619 mov qword ptr [rbp + 0x368], rax
141213620 lea rax, [rip + 0x1a06fb1]
141213627 mov qword ptr [rbp + 0x2d0], rax
14121362e mov qword ptr [rbp + 0x2d8], 1
141213639 mov qword ptr [rbp + 0x2f0], 0
141213644 mov qword ptr [rbp + 0x2e0], rbx
14121364b mov qword ptr [rbp + 0x2e8], 1
141213656 lea rdx, [rip + 0x1a08d3b]
14121365d lea rcx, [rbp + 0x2d0]
141213664 call 0x1428d9390
141213669 jmp 0x141213b58
14121366e lea rax, [rbp + 0x3a0]
141213675 mov qword ptr [rbp + 0x360], rax
14121367c lea rax, [rip - 0x486d73]
141213683 mov qword ptr [rbp + 0x368], rax
14121368a lea rax, [rip + 0x1a06f47]
141213691 mov qword ptr [rbp + 0x2d0], rax
141213698 mov qword ptr [rbp + 0x2d8], 1
1412136a3 mov qword ptr [rbp + 0x2f0], 0
1412136ae mov qword ptr [rbp + 0x2e0], rbx
1412136b5 mov qword ptr [rbp + 0x2e8], 1
1412136c0 lea rdx, [rip + 0x1a08ce9]
1412136c7 lea rcx, [rbp + 0x2d0]
1412136ce call 0x1428d9390
1412136d3 jmp 0x141213b58
1412136d8 lea r8, [rip + 0x1a08d01]
1412136df call 0x1428d9840
1412136e4 jmp 0x141213b58
1412136e9 lea r8, [rip + 0x1a08d08]
1412136f0 mov rdx, rsi
1412136f3 call 0x1428d9840
1412136f8 jmp 0x141213b58
1412136fd mov byte ptr [rbp + 0x3e7], 1
141213704 lea r8, [rip + 0x1a08ac5]
14121370b mov rcx, r10
14121370e call 0x1428d9518
141213713 jmp 0x141213b58
141213718 mov byte ptr [rbp + 0x3e7], 1
14121371f lea r8, [rip + 0x1a08aaa]
141213726 mov rcx, r9
141213729 call 0x1428d9518
14121372e jmp 0x141213b58
141213733 mov byte ptr [rbp + 0x3e7], 1
14121373a lea r8, [rip + 0x1a08aa7]
141213741 call 0x1428d9518
141213746 jmp 0x141213b58
14121374b mov byte ptr [rbp + 0x3e7], 1
141213752 lea r8, [rip + 0x1a08aa7]
141213759 mov rcx, r9
14121375c mov rdx, r15
14121375f call 0x1428d9518
141213764 jmp 0x141213b58
141213769 mov byte ptr [rbp + 0x3e7], 1
141213770 lea r8, [rip + 0x1a08aa1]
141213777 mov rdx, r15
14121377a call 0x1428d9518
14121377f jmp 0x141213b58
141213784 mov byte ptr [rbp + 0x3e7], 1
14121378b lea r8, [rip + 0x1a08a6e]
141213792 mov rcx, r10
141213795 mov rdx, r15
141213798 call 0x1428d9518
14121379d jmp 0x141213b58
1412137a2 mov byte ptr [rbp + 0x3e7], 1
1412137a9 lea r8, [rip + 0x1a08a68]
1412137b0 mov rcx, rax
1412137b3 mov rdx, r15
1412137b6 call 0x1428d9518
1412137bb jmp 0x141213b58
1412137c0 mov byte ptr [rbp + 0x3e7], 1
1412137c7 lea r8, [rip + 0x1a08a1a]
1412137ce mov rcx, rax
1412137d1 mov rdx, qword ptr [rbp + 0x3c8]
1412137d8 call 0x1428d9518
1412137dd jmp 0x141213b58
1412137e2 mov qword ptr [rsp + 0x20], 1
1412137eb mov r8d, 0x88
1412137f1 mov r9d, 1
1412137f7 mov rcx, rsi
1412137fa mov rdx, rdi
1412137fd call 0x1428a7de0
141213802 jmp 0x141210b9c
141213807 mov byte ptr [rbp + 0x3e7], 1
14121380e lea r8, [rip + 0x1a08a7b]
141213815 mov rcx, r9
141213818 call 0x1428d9518
14121381d jmp 0x141213b58
141213822 mov byte ptr [rbp + 0x3e7], 1
141213829 lea r8, [rip + 0x1a08a60]
141213830 call 0x1428d9518
141213835 jmp 0x141213b58
14121383a mov byte ptr [rbp + 0x3e7], 1
141213841 lea r8, [rip + 0x1a08a60]
141213848 mov rcx, r9
14121384b mov rdx, qword ptr [rbp + 0x3c8]
141213852 call 0x1428d9518
141213857 jmp 0x141213b58
14121385c mov byte ptr [rbp + 0x3e7], 1
141213863 lea r8, [rip + 0x1a08a3e]
14121386a mov rcx, rax
14121386d call 0x1428d9518
141213872 jmp 0x141213b58
141213877 lea r8, [rip + 0x1a08aea]
14121387e mov rcx, r14
141213881 mov rdx, rsi
141213884 call 0x1428d9518
141213889 jmp 0x141213b58
14121388e mov byte ptr [rbp + 0x3e7], 1
141213895 lea r8, [rip + 0x1a088d4]
14121389c xor ecx, ecx
14121389e xor edx, edx
1412138a0 call 0x1428d9518
1412138a5 jmp 0x141213b58
1412138aa mov byte ptr [rbp + 0x3e7], 1
1412138b1 lea rcx, [rip + 0x1a088d0]
1412138b8 call 0x1428d9310
1412138bd jmp 0x141213b58
1412138c2 lea r8, [rip + 0x1a055af]
1412138c9 call 0x1428d9518
1412138ce jmp 0x141213b58
1412138d3 mov byte ptr [rbp + 0x3e7], 0
1412138da lea rcx, [rip + 0x1a0872f]
1412138e1 call 0x1428d9310
1412138e6 jmp 0x141213b58
1412138eb lea rcx, [rip + 0x1a0874e]
1412138f2 call 0x1428d9310
1412138f7 jmp 0x141213b58
1412138fc lea r8, [rip + 0x1a05575]
141213903 call 0x1428d9518
141213908 jmp 0x141213b58
14121390d lea r8, [rip + 0x1a08714]
141213914 xor ecx, ecx
141213916 xor edx, edx
141213918 call 0x1428d9518
14121391d jmp 0x141213b58
141213922 lea r8, [rip + 0x1a06647]
141213929 mov rcx, r15
14121392c mov rdx, rbx
14121392f call 0x1428d8fe3
141213934 jmp 0x141213b58
141213939 mov byte ptr [rbp + 0x3e7], 1
141213940 lea rcx, [rip + 0x1a08901]
141213947 call 0x1428d9310
14121394c jmp 0x141213b58
141213951 mov byte ptr [rbp + 0x3e7], 1
141213958 lea r8, [rip + 0x1a088d1]
14121395f xor ecx, ecx
141213961 xor edx, edx
141213963 call 0x1428d9518
141213968 jmp 0x141213b58
14121396d mov qword ptr [rbp + 0x360], 0
141213978 mov qword ptr [rbp + 0x368], 4
141213983 mov qword ptr [rbp + 0x370], 0
14121398e lea r8, [rip + 0x1a08943]
141213995 xor ecx, ecx
141213997 xor edx, edx
141213999 call 0x1428d9518
14121399e jmp 0x141213b58
1412139a3 lea r8, [rip + 0x1a08916]
1412139aa mov ecx, 4
1412139af mov rdx, rsi
1412139b2 call 0x1428d8fe3
1412139b7 jmp 0x141213b58
1412139bc lea rcx, [rip + 0x1a0892d]
1412139c3 call 0x1428d9310
1412139c8 jmp 0x141213b58
1412139cd lea r8, [rip + 0x1a0894c]
1412139d4 xor ecx, ecx
1412139d6 xor edx, edx
1412139d8 call 0x1428d9518
1412139dd jmp 0x141213b58
1412139e2 lea rcx, [rip + 0x1a0894f]
1412139e9 call 0x1428d9310
1412139ee jmp 0x141213b58
1412139f3 lea r8, [rip + 0x1a08956]
1412139fa mov ecx, 4
1412139ff mov rdx, rsi
141213a02 call 0x1428d8fe3
141213a07 jmp 0x141213b58
141213a0c lea r8, [rip + 0x1a0851d]
141213a13 xor ecx, ecx
141213a15 xor edx, edx
141213a17 call 0x1428d9518
141213a1c jmp 0x141213b58
141213a21 lea rcx, [rip + 0x1a08520]
141213a28 call 0x1428d9310
141213a2d jmp 0x141213b58
141213a32 movups xmm0, xmmword ptr [rbp + 0x2d8]
141213a39 movaps xmmword ptr [rbp + 0x200], xmm0
141213a40 lea rax, [rip + 0x1a05931]
141213a47 mov qword ptr [rsp + 0x20], rax
141213a4c lea rcx, [rip + 0x1a05175]
141213a53 lea r9, [rip + 0x1a0514e]
141213a5a lea r8, [rbp + 0x200]
141213a61 mov edx, 0x2b
141213a66 call 0x1428d9760
141213a6b jmp 0x141213b58
141213a70 movups xmm0, xmmword ptr [rbp + 0x2d8]
141213a77 movaps xmmword ptr [rbp + 0x360], xmm0
141213a7e mov byte ptr [rbp + 0x3e6], 0
141213a85 lea rax, [rip + 0x1a0893c]
141213a8c mov qword ptr [rsp + 0x20], rax
141213a91 lea rcx, [rip + 0x1a05130]
141213a98 lea r9, [rip + 0x1a05109]
141213a9f lea r8, [rbp + 0x360]
141213aa6 mov edx, 0x2b
141213aab call 0x1428d9760
141213ab0 jmp 0x141213b58
141213ab5 mov ecx, 8
141213aba mov edx, 0x30
141213abf call 0x1428d9000
141213ac4 jmp 0x141213b58
141213ac9 mov ecx, 8
141213ace mov edx, 0x18
141213ad3 call 0x1428d9000
141213ad8 jmp 0x141213b58
141213ada lea r8, [rip + 0x1a08497]
141213ae1 xor ecx, ecx
141213ae3 xor edx, edx
141213ae5 call 0x1428d9518
141213aea jmp 0x141213b58
141213aec lea rcx, [rip + 0x1a0849d]
141213af3 call 0x1428d9310
141213af8 jmp 0x141213b58
141213afa lea r8, [rip + 0x1a084bf]
141213b01 call 0x1428d9518
141213b06 jmp 0x141213b58
141213b08 lea r8, [rip + 0x1a08499]
141213b0f call 0x1428d9518
141213b14 jmp 0x141213b58
141213b16 lea r8, [rip + 0x1a084a3]
141213b1d mov rcx, rax
141213b20 call 0x1428d9518
141213b25 jmp 0x141213b58
141213b27 lea r8, [rip + 0x1a0847a]
141213b2e mov rcx, rax
141213b31 call 0x1428d9518
141213b36 jmp 0x141213b58
141213b38 mov ecx, 4
141213b3d mov edx, 0xc
141213b42 call 0x1428d9000
141213b47 jmp 0x141213b58
141213b49 mov ecx, 4
141213b4e mov edx, 0x18
141213b53 call 0x1428d9000
141213b58 ud2 
