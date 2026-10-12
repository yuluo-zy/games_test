1412411c0 push r15
1412411c2 push r14
1412411c4 push r13
1412411c6 push r12
1412411c8 push rsi
1412411c9 push rdi
1412411ca push rbp
1412411cb push rbx
1412411cc sub rsp, 0x208
1412411d3 movaps xmmword ptr [rsp + 0x1f0], xmm15
1412411dc movaps xmmword ptr [rsp + 0x1e0], xmm14
1412411e5 movaps xmmword ptr [rsp + 0x1d0], xmm13
1412411ee movaps xmmword ptr [rsp + 0x1c0], xmm12
1412411f7 movaps xmmword ptr [rsp + 0x1b0], xmm11
141241200 movaps xmmword ptr [rsp + 0x1a0], xmm10
141241209 movaps xmmword ptr [rsp + 0x190], xmm9
141241212 movaps xmmword ptr [rsp + 0x180], xmm8
14124121b movaps xmmword ptr [rsp + 0x170], xmm7
141241223 movaps xmmword ptr [rsp + 0x160], xmm6
14124122b mov rsi, rdx
14124122e mov qword ptr [rsp + 0xd0], rcx
141241236 call 0x1412402e0
14124123b xorps xmm6, xmm6
14124123e ucomiss xmm6, xmm0
141241241 jae 0x141242414
141241247 mov rcx, qword ptr [rsp + 0xd0]
14124124f call 0x1412402f0
141241254 ucomiss xmm6, xmm0
141241257 jae 0x141242414
14124125d cmp qword ptr [rsi + 0x18], 0
141241262 je 0x141242414
141241268 mov rax, qword ptr [rsp + 0xd0]
141241270 movss xmm0, dword ptr [rax + 8]
141241275 movss xmm1, dword ptr [rax + 0xc]
14124127a movss xmm2, dword ptr [rax]
14124127e movss xmm3, dword ptr [rax + 4]
141241283 movss dword ptr [rsp + 0xdc], xmm0
14124128c movss dword ptr [rsp + 0xd8], xmm2
141241295 subss xmm0, xmm2
141241299 movss dword ptr [rsp + 0x80], xmm1
1412412a2 movss dword ptr [rsp + 0x7c], xmm3
1412412a8 subss xmm1, xmm3
1412412ac movss xmm9, dword ptr [rip + 0x16e45bb]
1412412b5 mulss xmm1, xmm9
1412412ba movss dword ptr [rsp + 0x30], xmm1
1412412c0 mulss xmm0, xmm9
1412412c5 movss dword ptr [rsp + 0x34], xmm0
1412412cb lea r12, [rsp + 0x90]
1412412d3 xor r13d, r13d
1412412d6 mov qword ptr [rsp + 0x40], rsi
1412412db jmp 0x1412412f1
1412412dd dec rcx
1412412e0 mov rax, rcx
1412412e3 mov qword ptr [rsi + 0x18], rax
1412412e7 cmp qword ptr [rsi + 0x18], r13
1412412eb jbe 0x141242414
1412412f1 mov rax, qword ptr [rsi + 0x10]
1412412f5 mov qword ptr [rsp + 0x48], r13
1412412fa add rax, r13
1412412fd mov rcx, qword ptr [rsi]
141241300 mov rdx, qword ptr [rsi + 8]
141241304 cmp rax, rcx
141241307 mov r8d, 0
14124130d cmovae r8, rcx
141241311 sub rax, r8
141241314 lea rax, [rax + rax*4]
141241318 movups xmm0, xmmword ptr [rdx + rax*8]
14124131c movups xmm1, xmmword ptr [rdx + rax*8 + 0x10]
141241321 movaps xmmword ptr [rsp + 0xa0], xmm1
141241329 mov rax, qword ptr [rdx + rax*8 + 0x20]
14124132e mov qword ptr [rsp + 0xb0], rax
141241336 movaps xmmword ptr [rsp + 0x90], xmm0
14124133e movsd xmm0, qword ptr [rsp + 0xa0]
141241347 movaps xmmword ptr [rsp + 0x100], xmm0
14124134f movsd xmm0, qword ptr [rsp + 0xa8]
141241358 movaps xmmword ptr [rsp + 0xc0], xmm0
141241360 movzx esi, byte ptr [rsp + 0xb4]
141241368 movzx eax, byte ptr [rsp + 0xb5]
141241370 mov byte ptr [rsp + 0x2f], al
141241374 movaps xmm0, xmmword ptr [rsp + 0xa0]
14124137c movaps xmmword ptr [rsp + 0x110], xmm0
141241384 mov r14, r12
141241387 mov rcx, r12
14124138a call 0x1412400c0
14124138f movaps xmm14, xmm0
141241393 movaps xmm13, xmm1
141241397 movss xmm8, dword ptr [rsp + 0x98]
1412413a1 movss xmm15, dword ptr [rsp + 0x9c]
1412413ab movss xmm7, dword ptr [rsp + 0x90]
1412413b4 movss xmm0, dword ptr [rsp + 0x94]
1412413bd movaps xmm10, xmm8
1412413c1 subss xmm10, xmm7
1412413c6 movaps xmm11, xmm15
1412413ca movss dword ptr [rsp + 0x84], xmm0
1412413d3 subss xmm11, xmm0
1412413d8 mov rdi, qword ptr [rsp + 0xd0]
1412413e0 mov rcx, rdi
1412413e3 call 0x1412400c0
1412413e8 movss dword ptr [rsp + 0x38], xmm0
1412413ee movaps xmmword ptr [rsp + 0xf0], xmm1
1412413f6 movaps xmm12, xmm10
1412413fa mulss xmm12, xmm9
1412413ff movss dword ptr [rsp + 0x8c], xmm11
141241409 mulss xmm11, xmm9
14124140e movaps xmm6, xmm14
141241412 subss xmm6, xmm12
141241417 movaps xmm9, xmm13
14124141b subss xmm9, xmm11
141241420 movss dword ptr [rsp + 0x88], xmm14
14124142a addss xmm12, xmm14
14124142f movss dword ptr [rsp + 0x3c], xmm13
141241436 addss xmm11, xmm13
14124143b mov rcx, rdi
14124143e movaps xmm1, xmm6
141241441 movaps xmm2, xmm9
141241445 call 0x141240140
14124144a mov r15d, eax
14124144d movzx r12d, al
141241451 mov rcx, rdi
141241454 movaps xmm1, xmm12
141241458 movaps xmm2, xmm9
14124145c call 0x141240140
141241461 mov ebp, eax
141241463 lea ebx, [r12 + 1]
141241468 test al, al
14124146a cmove ebx, r12d
14124146e mov rcx, rdi
141241471 movaps xmm1, xmm6
141241474 movaps xmm2, xmm11
141241478 call 0x141240140
14124147d mov r13d, eax
141241480 movzx r12d, al
141241484 mov rcx, rdi
141241487 movaps xmm1, xmm12
14124148b movaps xmm2, xmm11
14124148f call 0x141240140
141241494 movzx ecx, al
141241497 add ebx, r12d
14124149a add ebx, ecx
14124149c cmp ebx, 4
14124149f ja 0x1412424b0
1412414a5 lea r8, [rip + 0x19de31c]
1412414ac movsxd rdx, dword ptr [r8 + rbx*4]
1412414b0 add rdx, r8
1412414b3 jmp rdx
1412414b5 movzx edi, byte ptr [rsp + 0x2f]
1412414ba test dil, dil
1412414bd je 0x14124185d
1412414c3 test sil, sil
1412414c6 movss xmm9, dword ptr [rip + 0x16e43a1]
1412414cf mov r12, r14
1412414d2 mov r13, qword ptr [rsp + 0x48]
1412414d7 je 0x141241d33
1412414dd inc r13
1412414e0 mov rsi, qword ptr [rsp + 0x40]
1412414e5 jmp 0x1412412e7
1412414ea nop word ptr [rax + rax]
1412414f0 test r15b, r15b
1412414f3 je 0x14124171b
1412414f9 test bpl, bpl
1412414fc mov ebx, esi
1412414fe je 0x1412419ff
141241504 or ecx, r12d
141241507 mov r13, qword ptr [rsp + 0x48]
14124150c mov rsi, qword ptr [rsp + 0x40]
141241511 movss xmm8, dword ptr [rsp + 0x3c]
141241518 jne 0x141242498
14124151e mov r12, r14
141241521 movaps xmm11, xmmword ptr [rsp + 0xf0]
14124152a addss xmm11, dword ptr [rsp + 0x30]
141241531 subss xmm11, xmm9
141241536 movaps xmm0, xmm11
14124153a movss xmm13, dword ptr [rip + 0x16e432d]
141241543 movaps xmm12, xmm13
141241547 mulss xmm0, xmm13
14124154c addss xmm8, xmm0
141241551 movaps xmm0, xmmword ptr [rsp + 0xc0]
141241559 movshdup xmm7, xmm0
14124155d movd ebp, xmm0
141241561 xorps xmm10, xmm10
141241565 xor r15d, r15d
141241568 jmp 0x141241b88
14124156d nop dword ptr [rax]
141241570 mov rsi, qword ptr [rsp + 0x40]
141241575 mov rcx, qword ptr [rsi + 0x18]
141241579 test rcx, rcx
14124157c mov r13, qword ptr [rsp + 0x48]
141241581 je 0x141241701
141241587 lea rax, [rcx - 1]
14124158b cmp rax, r13
14124158e jbe 0x141241701
141241594 mov rdx, qword ptr [rsi + 0x10]
141241598 lea r8, [rdx + r13]
14124159c mov r9, qword ptr [rsi]
14124159f mov rcx, qword ptr [rsi + 8]
1412415a3 cmp r8, r9
1412415a6 mov r10, r9
1412415a9 mov r11d, 0
1412415af cmovb r10, r11
1412415b3 sub r8, r10
1412415b6 add rdx, rax
1412415b9 cmp rdx, r9
1412415bc cmovb r9, r11
1412415c0 sub rdx, r9
1412415c3 lea r8, [r8 + r8*4]
1412415c7 lea rdx, [rdx + rdx*4]
1412415cb mov r9, qword ptr [rcx + r8*8 + 0x20]
1412415d0 mov qword ptr [rsp + 0x70], r9
1412415d5 movups xmm0, xmmword ptr [rcx + r8*8]
1412415da movups xmm1, xmmword ptr [rcx + r8*8 + 0x10]
1412415e0 movaps xmmword ptr [rsp + 0x60], xmm1
1412415e5 movaps xmmword ptr [rsp + 0x50], xmm0
1412415ea mov r9, qword ptr [rcx + rdx*8 + 0x20]
1412415ef movups xmm0, xmmword ptr [rcx + rdx*8]
1412415f3 movups xmm1, xmmword ptr [rcx + rdx*8 + 0x10]
1412415f8 movups xmmword ptr [rcx + r8*8], xmm0
1412415fd movups xmmword ptr [rcx + r8*8 + 0x10], xmm1
141241603 mov qword ptr [rcx + r8*8 + 0x20], r9
141241608 mov r8, qword ptr [rsp + 0x70]
14124160d mov qword ptr [rcx + rdx*8 + 0x20], r8
141241612 movaps xmm0, xmmword ptr [rsp + 0x50]
141241617 movaps xmm1, xmmword ptr [rsp + 0x60]
14124161c movups xmmword ptr [rcx + rdx*8 + 0x10], xmm1
141241621 movups xmmword ptr [rcx + rdx*8], xmm0
141241625 movss xmm9, dword ptr [rip + 0x16e4242]
14124162e mov r12, r14
141241631 jmp 0x1412412e3
141241636 nop word ptr cs:[rax + rax]
141241640 test r15b, r15b
141241643 je 0x14124179d
141241649 or bpl, r13b
14124164c or bpl, al
14124164f mov edi, esi
141241651 mov rsi, qword ptr [rsp + 0x40]
141241656 jne 0x141242480
14124165c mov r12, r14
14124165f movss xmm2, dword ptr [rsp + 0x38]
141241665 addss xmm2, dword ptr [rsp + 0x34]
14124166b movaps xmm0, xmm6
14124166e movaps xmm1, xmm12
141241672 call 0x140c904a0
141241677 xorps xmm1, xmm1
14124167a maxss xmm1, xmm0
14124167e movss xmm8, dword ptr [rip + 0x16e427d]
141241687 minss xmm8, xmm1
14124168c movaps xmm1, xmmword ptr [rsp + 0xf0]
141241694 addss xmm1, dword ptr [rsp + 0x30]
14124169a movsldup xmm0, xmm8
14124169f movaps xmm2, xmmword ptr [rsp + 0xc0]
1412416a7 mulps xmm2, xmm0
1412416aa movaps xmm10, xmmword ptr [rip + 0x18bafae]
1412416b2 unpcklps xmm10, xmm1
1412416b6 movaps xmm0, xmm8
1412416ba unpcklps xmm0, xmm9
1412416be subps xmm10, xmm0
1412416c2 movshdup xmm0, xmm10
1412416c7 movss xmm11, dword ptr [rip + 0x16e41a0]
1412416d0 mulss xmm0, xmm11
1412416d5 movss xmm6, dword ptr [rsp + 0x3c]
1412416db addss xmm6, xmm0
1412416df movaps xmm1, xmm10
1412416e3 shufps xmm1, xmm10, 0
1412416e8 mulps xmm1, xmmword ptr [rsp + 0x110]
1412416f0 xorps xmm0, xmm0
1412416f3 movlhps xmm0, xmm2
1412416f6 xor r15d, r15d
1412416f9 mov bpl, 1
1412416fc jmp 0x141241f68
141241701 cmp rcx, r13
141241704 movss xmm9, dword ptr [rip + 0x16e4163]
14124170d mov r12, r14
141241710 ja 0x1412412dd
141241716 jmp 0x1412412e7
14124171b test bpl, bpl
14124171e mov ebx, esi
141241720 movss xmm8, dword ptr [rsp + 0x3c]
141241727 movss xmm9, dword ptr [rsp + 0x88]
141241731 je 0x141241b2b
141241737 xor r13b, 1
14124173b test al, r13b
14124173e mov r13, qword ptr [rsp + 0x48]
141241743 mov rsi, qword ptr [rsp + 0x40]
141241748 je 0x141242498
14124174e mov r12, r14
141241751 movss xmm0, dword ptr [rsp + 0x38]
141241757 subss xmm0, dword ptr [rsp + 0x34]
14124175d subss xmm12, xmm0
141241762 movaps xmm0, xmm12
141241766 mulss xmm0, dword ptr [rip + 0x171897a]
14124176e addss xmm9, xmm0
141241773 movdqa xmm0, xmmword ptr [rsp + 0xc0]
14124177c movd r15d, xmm0
141241781 xor ebp, ebp
141241783 xorps xmm7, xmm7
141241786 movss xmm11, dword ptr [rip + 0x16e40e1]
14124178f movaps xmm8, xmmword ptr [rsp + 0x100]
141241798 jmp 0x141241a6d
14124179d test bpl, bpl
1412417a0 mov edi, esi
1412417a2 je 0x141241c84
1412417a8 or ecx, r12d
1412417ab mov rsi, qword ptr [rsp + 0x40]
1412417b0 jne 0x141242480
1412417b6 mov r12, r14
1412417b9 movss xmm2, dword ptr [rsp + 0x38]
1412417bf subss xmm2, dword ptr [rsp + 0x34]
1412417c5 movaps xmm0, xmm6
1412417c8 movaps xmm1, xmm12
1412417cc call 0x140c904a0
1412417d1 xorps xmm1, xmm1
1412417d4 maxss xmm1, xmm0
1412417d8 movss xmm8, dword ptr [rip + 0x16e4123]
1412417e1 minss xmm8, xmm1
1412417e6 movaps xmm0, xmmword ptr [rsp + 0xf0]
1412417ee addss xmm0, dword ptr [rsp + 0x30]
1412417f4 movaps xmm10, xmmword ptr [rip + 0x18bae64]
1412417fc unpcklps xmm10, xmm0
141241800 movaps xmm0, xmm8
141241804 unpcklps xmm0, xmm9
141241808 subps xmm10, xmm0
14124180c movshdup xmm0, xmm10
141241811 movss xmm11, dword ptr [rip + 0x16e4056]
14124181a mulss xmm0, xmm11
14124181f movss xmm6, dword ptr [rsp + 0x3c]
141241825 addss xmm6, xmm0
141241829 movaps xmm0, xmm10
14124182d shufps xmm0, xmm10, 0
141241832 mulps xmm0, xmmword ptr [rsp + 0x110]
14124183a movsldup xmm1, xmm8
14124183f movaps xmm2, xmmword ptr [rsp + 0xc0]
141241847 mulps xmm2, xmm1
14124184a xorps xmm1, xmm1
14124184d movlhps xmm1, xmm2
141241850 mov r15d, 1
141241856 xor ebp, ebp
141241858 jmp 0x141241f68
14124185d ucomiss xmm8, dword ptr [rsp + 0xdc]
141241866 movss xmm9, dword ptr [rip + 0x16e4001]
14124186f mov r12, r14
141241872 mov r13, qword ptr [rsp + 0x48]
141241877 jbe 0x141241c53
14124187d movss xmm0, dword ptr [rsp + 0xd8]
141241886 ucomiss xmm0, xmm7
141241889 jbe 0x141241c53
14124188f mov rcx, r12
141241892 call 0x1412402e0
141241897 movaps xmm6, xmm0
14124189a mulss xmm6, xmm9
14124189f movaps xmm0, xmm6
1412418a2 movaps xmm10, xmm9
1412418a6 mulss xmm0, xmm9
1412418ab movss xmm8, dword ptr [rsp + 0x90]
1412418b5 addss xmm8, xmm0
1412418ba movss xmm7, dword ptr [rsp + 0x98]
1412418c3 subss xmm7, xmm0
1412418c7 mov rcx, r12
1412418ca call 0x1412400c0
1412418cf movaps xmm9, xmm1
1412418d3 mov rcx, r12
1412418d6 call 0x1412402f0
1412418db movss dword ptr [rsp + 0x20], xmm0
1412418e1 lea rcx, [rsp + 0x50]
1412418e6 movaps xmm1, xmm8
1412418ea movaps xmm2, xmm9
1412418ee movaps xmm3, xmm6
1412418f1 call 0x14123fff0
1412418f6 mov rcx, r12
1412418f9 call 0x1412400c0
1412418fe movaps xmm8, xmm1
141241902 mov rcx, r12
141241905 call 0x1412402f0
14124190a movss dword ptr [rsp + 0x20], xmm0
141241910 lea rcx, [rsp + 0x140]
141241918 movaps xmm1, xmm7
14124191b movaps xmm2, xmm8
14124191f movaps xmm3, xmm6
141241922 call 0x14123fff0
141241927 mov rdi, qword ptr [rsp + 0x40]
14124192c cmp qword ptr [rdi + 0x18], r13
141241930 jbe 0x1412424f8
141241936 mov ebp, dword ptr [rsp + 0xb0]
14124193d movaps xmm1, xmmword ptr [rsp + 0xc0]
141241945 mulps xmm1, xmmword ptr [rip + 0x16e3f94]
14124194c mov rax, qword ptr [rdi + 0x10]
141241950 add rax, r13
141241953 mov rcx, qword ptr [rdi]
141241956 mov rdx, qword ptr [rdi + 8]
14124195a cmp rax, rcx
14124195d mov r8d, 0
141241963 cmovae r8, rcx
141241967 sub rax, r8
14124196a lea rax, [rax + rax*4]
14124196e movups xmm0, xmmword ptr [rsp + 0x50]
141241973 movups xmmword ptr [rdx + rax*8], xmm0
141241977 xorps xmm6, xmm6
14124197a movlhps xmm6, xmm1
14124197d movups xmmword ptr [rdx + rax*8 + 0x10], xmm6
141241982 mov dword ptr [rdx + rax*8 + 0x20], ebp
141241986 mov byte ptr [rdx + rax*8 + 0x24], sil
14124198b mov byte ptr [rdx + rax*8 + 0x25], 1
141241990 mov rcx, qword ptr [rdi]
141241993 mov rax, qword ptr [rdi + 0x18]
141241997 cmp rax, rcx
14124199a jne 0x1412419b2
14124199c mov rcx, rdi
14124199f lea rdx, [rip + 0x19ddd3a]
1412419a6 call 0x141241120
1412419ab mov rcx, qword ptr [rdi]
1412419ae mov rax, qword ptr [rdi + 0x18]
1412419b2 add ebp, 0x64
1412419b5 add rax, qword ptr [rdi + 0x10]
1412419b9 cmp rax, rcx
1412419bc mov edx, 0
1412419c1 cmovae rdx, rcx
1412419c5 sub rax, rdx
1412419c8 mov rcx, qword ptr [rdi + 8]
1412419cc lea rax, [rax + rax*4]
1412419d0 movups xmm0, xmmword ptr [rsp + 0x140]
1412419d8 movups xmmword ptr [rcx + rax*8], xmm0
1412419dc movups xmmword ptr [rcx + rax*8 + 0x10], xmm6
1412419e1 mov dword ptr [rcx + rax*8 + 0x20], ebp
1412419e5 mov byte ptr [rcx + rax*8 + 0x24], sil
1412419ea mov byte ptr [rcx + rax*8 + 0x25], 1
1412419ef inc qword ptr [rdi + 0x18]
1412419f3 mov rsi, rdi
1412419f6 movaps xmm9, xmm10
1412419fa jmp 0x1412412e7
1412419ff xor al, 1
141241a01 test r13b, al
141241a04 mov r13, qword ptr [rsp + 0x48]
141241a09 mov rsi, qword ptr [rsp + 0x40]
141241a0e movss xmm9, dword ptr [rsp + 0x88]
141241a18 je 0x141242498
141241a1e mov r12, r14
141241a21 movss xmm12, dword ptr [rsp + 0x38]
141241a28 addss xmm12, dword ptr [rsp + 0x34]
141241a2f subss xmm12, xmm6
141241a34 movaps xmm0, xmm12
141241a38 movss xmm13, dword ptr [rip + 0x16e3e2f]
141241a41 movaps xmm11, xmm13
141241a45 mulss xmm0, xmm13
141241a4a addss xmm9, xmm0
141241a4f pshufd xmm0, xmmword ptr [rsp + 0x100], 0x55
141241a59 movd ebp, xmm0
141241a5d movshdup xmm7, xmmword ptr [rsp + 0xc0]
141241a66 xorps xmm8, xmm8
141241a6a xor r15d, r15d
141241a6d subss xmm10, xmm12
141241a72 mov rcx, r12
141241a75 call 0x1412400c0
141241a7a movaps xmm6, xmm1
141241a7d mov rcx, r12
141241a80 call 0x1412402f0
141241a85 movd dword ptr [rsp + 0x20], xmm0
141241a8b lea rcx, [rsp + 0x50]
141241a90 movaps xmm1, xmm9
141241a94 movaps xmm2, xmm6
141241a97 movaps xmm3, xmm10
141241a9b call 0x14123fff0
141241aa0 cmp qword ptr [rsi + 0x18], r13
141241aa4 jbe 0x1412424e0
141241aaa mov eax, dword ptr [rsp + 0xb0]
141241ab1 mov ecx, eax
141241ab3 mov edx, 0xcccccccd
141241ab8 imul rcx, rdx
141241abc shr rcx, 0x23
141241ac0 add ecx, ecx
141241ac2 lea ecx, [rcx + rcx*4]
141241ac5 sub eax, ecx
141241ac7 mov rcx, qword ptr [rsi + 0x10]
141241acb add rcx, r13
141241ace mov rdx, qword ptr [rsi]
141241ad1 mov r8, qword ptr [rsi + 8]
141241ad5 cmp rcx, rdx
141241ad8 mov r9d, 0
141241ade cmovae r9, rdx
141241ae2 sub rcx, r9
141241ae5 lea rcx, [rcx + rcx*4]
141241ae9 movups xmm0, xmmword ptr [rsp + 0x50]
141241aee movups xmmword ptr [r8 + rcx*8], xmm0
141241af3 movss dword ptr [r8 + rcx*8 + 0x10], xmm8
141241afa mov dword ptr [r8 + rcx*8 + 0x14], ebp
141241aff mov dword ptr [r8 + rcx*8 + 0x18], r15d
141241b04 movss dword ptr [r8 + rcx*8 + 0x1c], xmm7
141241b0b mov dword ptr [r8 + rcx*8 + 0x20], eax
141241b10 mov byte ptr [r8 + rcx*8 + 0x24], bl
141241b15 movzx eax, byte ptr [rsp + 0x2f]
141241b1a mov byte ptr [r8 + rcx*8 + 0x25], al
141241b1f inc r13
141241b22 movaps xmm9, xmm11
141241b26 jmp 0x1412412e7
141241b2b test r13b, al
141241b2e mov r13, qword ptr [rsp + 0x48]
141241b33 mov rsi, qword ptr [rsp + 0x40]
141241b38 je 0x141242498
141241b3e mov r12, r14
141241b41 movaps xmm0, xmmword ptr [rsp + 0xf0]
141241b49 subss xmm0, dword ptr [rsp + 0x30]
141241b4f subss xmm11, xmm0
141241b54 movaps xmm0, xmm11
141241b58 mulss xmm0, dword ptr [rip + 0x1718588]
141241b60 addss xmm8, xmm0
141241b65 movdqa xmm10, xmmword ptr [rsp + 0x100]
141241b6f pshufd xmm0, xmm10, 0x55
141241b75 movd r15d, xmm0
141241b7a xor ebp, ebp
141241b7c xorps xmm7, xmm7
141241b7f movss xmm12, dword ptr [rip + 0x16e3ce8]
141241b88 movss xmm9, dword ptr [rsp + 0x8c]
141241b92 subss xmm9, xmm11
141241b97 mov rcx, r12
141241b9a call 0x1412400c0
141241b9f movdqa xmm6, xmm0
141241ba3 mov rcx, r12
141241ba6 call 0x1412402e0
141241bab movss dword ptr [rsp + 0x20], xmm9
141241bb2 lea rcx, [rsp + 0x50]
141241bb7 movdqa xmm1, xmm6
141241bbb movaps xmm2, xmm8
141241bbf movdqa xmm3, xmm0
141241bc3 call 0x14123fff0
141241bc8 cmp qword ptr [rsi + 0x18], r13
141241bcc jbe 0x1412424c8
141241bd2 mov eax, dword ptr [rsp + 0xb0]
141241bd9 mov ecx, eax
141241bdb mov edx, 0xcccccccd
141241be0 imul rcx, rdx
141241be4 shr rcx, 0x23
141241be8 add ecx, ecx
141241bea lea ecx, [rcx + rcx*4]
141241bed sub eax, ecx
141241bef mov rcx, qword ptr [rsi + 0x10]
141241bf3 add rcx, r13
141241bf6 mov rdx, qword ptr [rsi]
141241bf9 mov r8, qword ptr [rsi + 8]
141241bfd cmp rcx, rdx
141241c00 mov r9d, 0
141241c06 cmovae r9, rdx
141241c0a sub rcx, r9
141241c0d lea rcx, [rcx + rcx*4]
141241c11 movups xmm0, xmmword ptr [rsp + 0x50]
141241c16 movups xmmword ptr [r8 + rcx*8], xmm0
141241c1b movd dword ptr [r8 + rcx*8 + 0x10], xmm10
141241c22 mov dword ptr [r8 + rcx*8 + 0x14], r15d
141241c27 mov dword ptr [r8 + rcx*8 + 0x18], ebp
141241c2c movss dword ptr [r8 + rcx*8 + 0x1c], xmm7
141241c33 mov dword ptr [r8 + rcx*8 + 0x20], eax
141241c38 mov byte ptr [r8 + rcx*8 + 0x24], bl
141241c3d movzx eax, byte ptr [rsp + 0x2f]
141241c42 mov byte ptr [r8 + rcx*8 + 0x25], al
141241c47 inc r13
141241c4a movaps xmm9, xmm12
141241c4e jmp 0x1412412e7
141241c53 ucomiss xmm15, dword ptr [rsp + 0x80]
141241c5c jbe 0x1412414dd
141241c62 movss xmm0, dword ptr [rsp + 0x7c]
141241c68 ucomiss xmm0, dword ptr [rsp + 0x84]
141241c70 jbe 0x1412414dd
141241c76 test sil, sil
141241c79 je 0x141241d56
141241c7f jmp 0x1412414dd
141241c84 test r13b, r13b
141241c87 mov rsi, qword ptr [rsp + 0x40]
141241c8c je 0x141241ec3
141241c92 test al, al
141241c94 jne 0x141242480
141241c9a mov r12, r14
141241c9d movss xmm2, dword ptr [rsp + 0x38]
141241ca3 addss xmm2, dword ptr [rsp + 0x34]
141241ca9 movaps xmm0, xmm6
141241cac movaps xmm1, xmm12
141241cb0 call 0x140c904a0
141241cb5 xorps xmm1, xmm1
141241cb8 maxss xmm1, xmm0
141241cbc movss xmm8, dword ptr [rip + 0x16e3c3f]
141241cc5 minss xmm8, xmm1
141241cca movaps xmm2, xmmword ptr [rsp + 0xf0]
141241cd2 subss xmm2, dword ptr [rsp + 0x30]
141241cd8 movsldup xmm0, xmm8
141241cdd mulps xmm0, xmmword ptr [rsp + 0x100]
141241ce5 movaps xmm10, xmmword ptr [rip + 0x18ba973]
141241ced unpcklps xmm10, xmm11
141241cf1 movaps xmm1, xmm8
141241cf5 unpcklps xmm1, xmm2
141241cf8 subps xmm10, xmm1
141241cfc movshdup xmm1, xmm10
141241d01 mulss xmm1, dword ptr [rip + 0x17183df]
141241d09 movss xmm6, dword ptr [rsp + 0x3c]
141241d0f addss xmm6, xmm1
141241d13 movaps xmm1, xmm10
141241d17 shufps xmm1, xmm10, 0
141241d1c mulps xmm1, xmmword ptr [rsp + 0x110]
141241d24 movq xmm0, xmm0
141241d28 xor r15d, r15d
141241d2b mov bpl, 1
141241d2e jmp 0x141241f5f
141241d33 ucomiss xmm15, dword ptr [rsp + 0x80]
141241d3c jbe 0x1412414dd
141241d42 movss xmm0, dword ptr [rsp + 0x7c]
141241d48 ucomiss xmm0, dword ptr [rsp + 0x84]
141241d50 jbe 0x1412414dd
141241d56 mov rcx, r12
141241d59 call 0x1412402f0
141241d5e movaps xmm6, xmm0
141241d61 mulss xmm6, xmm9
141241d66 movaps xmm0, xmm6
141241d69 movaps xmm10, xmm9
141241d6d mulss xmm0, xmm9
141241d72 movss xmm8, dword ptr [rsp + 0x94]
141241d7c addss xmm8, xmm0
141241d81 movss xmm7, dword ptr [rsp + 0x9c]
141241d8a subss xmm7, xmm0
141241d8e mov rcx, r12
141241d91 call 0x1412400c0
141241d96 movaps xmm9, xmm0
141241d9a mov rcx, r12
141241d9d call 0x1412402e0
141241da2 movss dword ptr [rsp + 0x20], xmm6
141241da8 lea rcx, [rsp + 0x50]
141241dad movaps xmm1, xmm9
141241db1 movaps xmm2, xmm8
141241db5 movaps xmm3, xmm0
141241db8 call 0x14123fff0
141241dbd mov rcx, r12
141241dc0 call 0x1412400c0
141241dc5 movaps xmm8, xmm0
141241dc9 mov rcx, r12
141241dcc call 0x1412402e0
141241dd1 movss dword ptr [rsp + 0x20], xmm6
141241dd7 lea rcx, [rsp + 0x150]
141241ddf movaps xmm1, xmm8
141241de3 movaps xmm2, xmm7
141241de6 movaps xmm3, xmm0
141241de9 call 0x14123fff0
141241dee mov rsi, qword ptr [rsp + 0x40]
141241df3 cmp qword ptr [rsi + 0x18], r13
141241df7 jbe 0x141242510
141241dfd mov ebx, dword ptr [rsp + 0xb0]
141241e04 movaps xmm1, xmmword ptr [rsp + 0xc0]
141241e0c mulps xmm1, xmmword ptr [rip + 0x16e3acd]
141241e13 mov rax, qword ptr [rsi + 0x10]
141241e17 add rax, r13
141241e1a mov rcx, qword ptr [rsi]
141241e1d mov rdx, qword ptr [rsi + 8]
141241e21 cmp rax, rcx
141241e24 mov r8d, 0
141241e2a cmovae r8, rcx
141241e2e sub rax, r8
141241e31 lea rax, [rax + rax*4]
141241e35 movups xmm0, xmmword ptr [rsp + 0x50]
141241e3a movups xmmword ptr [rdx + rax*8], xmm0
141241e3e xorps xmm6, xmm6
141241e41 movlhps xmm6, xmm1
141241e44 movups xmmword ptr [rdx + rax*8 + 0x10], xmm6
141241e49 mov dword ptr [rdx + rax*8 + 0x20], ebx
141241e4d mov byte ptr [rdx + rax*8 + 0x24], 1
141241e52 mov byte ptr [rdx + rax*8 + 0x25], dil
141241e57 mov rcx, qword ptr [rsi]
141241e5a mov rax, qword ptr [rsi + 0x18]
141241e5e cmp rax, rcx
141241e61 jne 0x141241e79
141241e63 mov rcx, rsi
141241e66 lea rdx, [rip + 0x19dd843]
141241e6d call 0x141241120
141241e72 mov rcx, qword ptr [rsi]
141241e75 mov rax, qword ptr [rsi + 0x18]
141241e79 add ebx, 0x64
141241e7c add rax, qword ptr [rsi + 0x10]
141241e80 cmp rax, rcx
141241e83 mov edx, 0
141241e88 cmovae rdx, rcx
141241e8c sub rax, rdx
141241e8f mov rcx, qword ptr [rsi + 8]
141241e93 lea rax, [rax + rax*4]
141241e97 movups xmm0, xmmword ptr [rsp + 0x150]
141241e9f movups xmmword ptr [rcx + rax*8], xmm0
141241ea3 movups xmmword ptr [rcx + rax*8 + 0x10], xmm6
141241ea8 mov dword ptr [rcx + rax*8 + 0x20], ebx
141241eac mov byte ptr [rcx + rax*8 + 0x24], 1
141241eb1 mov byte ptr [rcx + rax*8 + 0x25], dil
141241eb6 inc qword ptr [rsi + 0x18]
141241eba movaps xmm9, xmm10
141241ebe jmp 0x1412412e7
141241ec3 test al, al
141241ec5 je 0x141242480
141241ecb mov r12, r14
141241ece movss xmm2, dword ptr [rsp + 0x38]
141241ed4 subss xmm2, dword ptr [rsp + 0x34]
141241eda movaps xmm0, xmm6
141241edd movaps xmm1, xmm12
141241ee1 call 0x140c904a0
141241ee6 xorps xmm1, xmm1
141241ee9 maxss xmm1, xmm0
141241eed movss xmm8, dword ptr [rip + 0x16e3a0e]
141241ef6 minss xmm8, xmm1
141241efb movaps xmm1, xmmword ptr [rsp + 0xf0]
141241f03 subss xmm1, dword ptr [rsp + 0x30]
141241f09 movaps xmm10, xmmword ptr [rip + 0x18ba74f]
141241f11 unpcklps xmm10, xmm11
141241f15 movaps xmm0, xmm8
141241f19 unpcklps xmm0, xmm1
141241f1c subps xmm10, xmm0
141241f20 movshdup xmm0, xmm10
141241f25 mulss xmm0, dword ptr [rip + 0x17181bb]
141241f2d movss xmm6, dword ptr [rsp + 0x3c]
141241f33 addss xmm6, xmm0
141241f37 movaps xmm0, xmm10
141241f3b shufps xmm0, xmm10, 0
141241f40 movaps xmm2, xmmword ptr [rsp + 0x110]
141241f48 mulps xmm0, xmm2
141241f4b movsldup xmm1, xmm8
141241f50 mulps xmm1, xmm2
141241f53 movq xmm1, xmm1
141241f57 mov r15d, 1
141241f5d xor ebp, ebp
141241f5f movss xmm11, dword ptr [rip + 0x16e3908]
141241f68 movshdup xmm2, xmm10
141241f6d movss xmm7, dword ptr [rsp + 0x8c]
141241f76 subss xmm7, xmm2
141241f7a movaps xmmword ptr [rsp + 0x120], xmm0
141241f82 movaps xmmword ptr [rsp + 0x130], xmm1
141241f8a mov rcx, r12
141241f8d call 0x1412402e0
141241f92 movaps xmm9, xmm0
141241f96 mulss xmm9, xmm8
141241f9b mov rcx, r12
141241f9e call 0x1412402e0
141241fa3 movaps xmm12, xmm0
141241fa7 mulss xmm12, xmm10
141241fac movaps xmm10, xmm9
141241fb0 mulss xmm10, xmm11
141241fb5 addss xmm10, dword ptr [rsp + 0x90]
141241fbf movss xmm8, dword ptr [rsp + 0x98]
141241fc9 movaps xmm0, xmm12
141241fcd movaps xmm13, xmm11
141241fd1 mulss xmm0, xmm11
141241fd6 subss xmm8, xmm0
141241fdb mov eax, r15d
141241fde shl eax, 4
141241fe1 lea r13, [rsp + rax]
141241fe5 add r13, 0x120
141241fec test bpl, bpl
141241fef je 0x1412421b5
141241ff5 movss dword ptr [rsp + 0x20], xmm7
141241ffb lea rcx, [rsp + 0xe0]
141242003 movaps xmm1, xmm10
141242007 movaps xmm2, xmm6
14124200a movaps xmm3, xmm9
14124200e call 0x14123fff0
141242013 movups xmm0, xmmword ptr [r13]
141242018 lea rax, [rsp + 0x60]
14124201d movups xmmword ptr [rax], xmm0
141242020 add r15d, dword ptr [rsp + 0xb0]
141242028 movups xmm0, xmmword ptr [rsp + 0xe0]
141242030 movaps xmmword ptr [rsp + 0x50], xmm0
141242035 mov rax, qword ptr [rsi]
141242038 mov rcx, qword ptr [rsi + 0x18]
14124203c cmp rcx, rax
14124203f jne 0x141242057
141242041 mov rcx, rsi
141242044 lea rdx, [rip + 0x19dd6f5]
14124204b call 0x141241120
141242050 mov rax, qword ptr [rsi]
141242053 mov rcx, qword ptr [rsi + 0x18]
141242057 mov rdx, r15
14124205a mov r8d, 0xcccccccd
141242060 imul rdx, r8
141242064 shr rdx, 0x23
141242068 add edx, edx
14124206a lea edx, [rdx + rdx*4]
14124206d sub r15d, edx
141242070 mov rdx, qword ptr [rsi + 0x10]
141242074 dec rdx
141242077 add rax, rdx
14124207a cmovae rax, rdx
14124207e mov rdx, qword ptr [rsi + 8]
141242082 mov qword ptr [rsi + 0x10], rax
141242086 inc rcx
141242089 mov qword ptr [rsi + 0x18], rcx
14124208d lea rax, [rax + rax*4]
141242091 movaps xmm0, xmmword ptr [rsp + 0x50]
141242096 movaps xmm1, xmmword ptr [rsp + 0x60]
14124209b movups xmmword ptr [rdx + rax*8], xmm0
14124209f movups xmmword ptr [rdx + rax*8 + 0x10], xmm1
1412420a4 mov dword ptr [rdx + rax*8 + 0x20], r15d
1412420a9 mov byte ptr [rdx + rax*8 + 0x24], dil
1412420ae movzx ebx, byte ptr [rsp + 0x2f]
1412420b3 mov byte ptr [rdx + rax*8 + 0x25], bl
1412420b7 mov rcx, r12
1412420ba call 0x1412400c0
1412420bf movaps xmm6, xmm1
1412420c2 mov rcx, r12
1412420c5 call 0x1412402f0
1412420ca movss dword ptr [rsp + 0x20], xmm0
1412420d0 lea rcx, [rsp + 0xe0]
1412420d8 movaps xmm1, xmm8
1412420dc movaps xmm2, xmm6
1412420df movaps xmm3, xmm12
1412420e3 call 0x14123fff0
1412420e8 lea rax, [rsp + 0x130]
1412420f0 movups xmm0, xmmword ptr [rax]
1412420f3 lea rax, [rsp + 0x60]
1412420f8 movups xmmword ptr [rax], xmm0
1412420fb mov r15d, dword ptr [rsp + 0xb0]
141242103 movups xmm0, xmmword ptr [rsp + 0xe0]
14124210b movaps xmmword ptr [rsp + 0x50], xmm0
141242110 mov rax, qword ptr [rsi]
141242113 mov rcx, qword ptr [rsi + 0x18]
141242117 cmp rcx, rax
14124211a mov r13, qword ptr [rsp + 0x48]
14124211f movaps xmm9, xmm13
141242123 jne 0x14124213b
141242125 mov rcx, rsi
141242128 lea rdx, [rip + 0x19dd5f9]
14124212f call 0x141241120
141242134 mov rax, qword ptr [rsi]
141242137 mov rcx, qword ptr [rsi + 0x18]
14124213b mov edx, r15d
14124213e inc edx
141242140 mov r8d, 0xcccccccd
141242146 imul rdx, r8
14124214a shr rdx, 0x23
14124214e add edx, edx
141242150 lea edx, [rdx + rdx*4]
141242153 neg edx
141242155 add edx, r15d
141242158 inc edx
14124215a mov r8, qword ptr [rsi + 0x10]
14124215e dec r8
141242161 add rax, r8
141242164 cmovae rax, r8
141242168 mov r8, qword ptr [rsi + 8]
14124216c mov qword ptr [rsi + 0x10], rax
141242170 inc rcx
141242173 mov qword ptr [rsi + 0x18], rcx
141242177 lea rax, [rax + rax*4]
14124217b movaps xmm0, xmmword ptr [rsp + 0x50]
141242180 movaps xmm1, xmmword ptr [rsp + 0x60]
141242185 movups xmmword ptr [r8 + rax*8], xmm0
14124218a movups xmmword ptr [r8 + rax*8 + 0x10], xmm1
141242190 mov dword ptr [r8 + rax*8 + 0x20], edx
141242195 mov byte ptr [r8 + rax*8 + 0x24], dil
14124219a mov byte ptr [r8 + rax*8 + 0x25], bl
14124219f add r13, 2
1412421a3 mov rcx, qword ptr [rsi + 0x18]
1412421a7 test rcx, rcx
1412421aa jne 0x141242363
1412421b0 jmp 0x141242406
1412421b5 mov rcx, r12
1412421b8 call 0x1412400c0
1412421bd movaps xmm11, xmm1
1412421c1 mov rcx, r12
1412421c4 call 0x1412402f0
1412421c9 movss dword ptr [rsp + 0x20], xmm0
1412421cf lea rcx, [rsp + 0xe0]
1412421d7 movaps xmm1, xmm10
1412421db movaps xmm2, xmm11
1412421df movaps xmm3, xmm9
1412421e3 call 0x14123fff0
1412421e8 movaps xmm0, xmmword ptr [rsp + 0x120]
1412421f0 lea rax, [rsp + 0x60]
1412421f5 movups xmmword ptr [rax], xmm0
1412421f8 mov ebp, dword ptr [rsp + 0xb0]
1412421ff movups xmm0, xmmword ptr [rsp + 0xe0]
141242207 movaps xmmword ptr [rsp + 0x50], xmm0
14124220c mov rax, qword ptr [rsi]
14124220f mov rcx, qword ptr [rsi + 0x18]
141242213 cmp rcx, rax
141242216 jne 0x14124222e
141242218 mov rcx, rsi
14124221b lea rdx, [rip + 0x19dd506]
141242222 call 0x141241120
141242227 mov rax, qword ptr [rsi]
14124222a mov rcx, qword ptr [rsi + 0x18]
14124222e mov rdx, rbp
141242231 mov r8d, 0xcccccccd
141242237 imul rdx, r8
14124223b shr rdx, 0x23
14124223f add edx, edx
141242241 lea edx, [rdx + rdx*4]
141242244 sub ebp, edx
141242246 mov rdx, qword ptr [rsi + 0x10]
14124224a dec rdx
14124224d add rax, rdx
141242250 cmovae rax, rdx
141242254 mov rdx, qword ptr [rsi + 8]
141242258 mov qword ptr [rsi + 0x10], rax
14124225c inc rcx
14124225f mov qword ptr [rsi + 0x18], rcx
141242263 lea rax, [rax + rax*4]
141242267 movaps xmm0, xmmword ptr [rsp + 0x50]
14124226c movaps xmm1, xmmword ptr [rsp + 0x60]
141242271 movups xmmword ptr [rdx + rax*8], xmm0
141242275 movups xmmword ptr [rdx + rax*8 + 0x10], xmm1
14124227a mov dword ptr [rdx + rax*8 + 0x20], ebp
14124227e mov byte ptr [rdx + rax*8 + 0x24], dil
141242283 movzx ebx, byte ptr [rsp + 0x2f]
141242288 mov byte ptr [rdx + rax*8 + 0x25], bl
14124228c movss dword ptr [rsp + 0x20], xmm7
141242292 lea rcx, [rsp + 0xe0]
14124229a movaps xmm1, xmm8
14124229e movaps xmm2, xmm6
1412422a1 movaps xmm3, xmm12
1412422a5 call 0x14123fff0
1412422aa movups xmm0, xmmword ptr [r13]
1412422af lea rax, [rsp + 0x60]
1412422b4 movups xmmword ptr [rax], xmm0
1412422b7 add r15d, dword ptr [rsp + 0xb0]
1412422bf movups xmm0, xmmword ptr [rsp + 0xe0]
1412422c7 movaps xmmword ptr [rsp + 0x50], xmm0
1412422cc mov rax, qword ptr [rsi]
1412422cf mov rcx, qword ptr [rsi + 0x18]
1412422d3 cmp rcx, rax
1412422d6 movaps xmm9, xmm13
1412422da jne 0x1412422f2
1412422dc mov rcx, rsi
1412422df lea rdx, [rip + 0x19dd45a]
1412422e6 call 0x141241120
1412422eb mov rax, qword ptr [rsi]
1412422ee mov rcx, qword ptr [rsi + 0x18]
1412422f2 mov rdx, r15
1412422f5 mov r8d, 0xcccccccd
1412422fb imul rdx, r8
1412422ff shr rdx, 0x23
141242303 add edx, edx
141242305 lea edx, [rdx + rdx*4]
141242308 sub r15d, edx
14124230b mov rdx, qword ptr [rsi + 0x10]
14124230f dec rdx
141242312 add rax, rdx
141242315 cmovae rax, rdx
141242319 mov rdx, qword ptr [rsi + 8]
14124231d mov qword ptr [rsi + 0x10], rax
141242321 inc rcx
141242324 mov qword ptr [rsi + 0x18], rcx
141242328 lea rax, [rax + rax*4]
14124232c movaps xmm0, xmmword ptr [rsp + 0x50]
141242331 movaps xmm1, xmmword ptr [rsp + 0x60]
141242336 movups xmmword ptr [rdx + rax*8], xmm0
14124233a movups xmmword ptr [rdx + rax*8 + 0x10], xmm1
14124233f mov dword ptr [rdx + rax*8 + 0x20], r15d
141242344 mov byte ptr [rdx + rax*8 + 0x24], dil
141242349 mov byte ptr [rdx + rax*8 + 0x25], bl
14124234d mov r13, qword ptr [rsp + 0x48]
141242352 add r13, 2
141242356 mov rcx, qword ptr [rsi + 0x18]
14124235a test rcx, rcx
14124235d je 0x141242406
141242363 lea rax, [rcx - 1]
141242367 cmp rax, r13
14124236a jbe 0x141242406
141242370 mov rdx, qword ptr [rsi + 0x10]
141242374 lea r8, [rdx + r13]
141242378 mov r9, qword ptr [rsi]
14124237b mov rcx, qword ptr [rsi + 8]
14124237f cmp r8, r9
141242382 mov r10, r9
141242385 mov r11d, 0
14124238b cmovb r10, r11
14124238f sub r8, r10
141242392 add rdx, rax
141242395 cmp rdx, r9
141242398 cmovb r9, r11
14124239c sub rdx, r9
14124239f lea r8, [r8 + r8*4]
1412423a3 lea rdx, [rdx + rdx*4]
1412423a7 mov r9, qword ptr [rcx + r8*8 + 0x20]
1412423ac mov qword ptr [rsp + 0x70], r9
1412423b1 movups xmm0, xmmword ptr [rcx + r8*8]
1412423b6 movups xmm1, xmmword ptr [rcx + r8*8 + 0x10]
1412423bc movaps xmmword ptr [rsp + 0x60], xmm1
1412423c1 movaps xmmword ptr [rsp + 0x50], xmm0
1412423c6 mov r9, qword ptr [rcx + rdx*8 + 0x20]
1412423cb movups xmm0, xmmword ptr [rcx + rdx*8]
1412423cf movups xmm1, xmmword ptr [rcx + rdx*8 + 0x10]
1412423d4 movups xmmword ptr [rcx + r8*8], xmm0
1412423d9 movups xmmword ptr [rcx + r8*8 + 0x10], xmm1
1412423df mov qword ptr [rcx + r8*8 + 0x20], r9
1412423e4 mov r8, qword ptr [rsp + 0x70]
1412423e9 mov qword ptr [rcx + rdx*8 + 0x20], r8
1412423ee movaps xmm0, xmmword ptr [rsp + 0x50]
1412423f3 movaps xmm1, xmmword ptr [rsp + 0x60]
1412423f8 movups xmmword ptr [rcx + rdx*8 + 0x10], xmm1
1412423fd movups xmmword ptr [rcx + rdx*8], xmm0
141242401 jmp 0x1412412e3
141242406 cmp rcx, r13
141242409 ja 0x1412412dd
14124240f jmp 0x1412412e7
141242414 movaps xmm6, xmmword ptr [rsp + 0x160]
14124241c movaps xmm7, xmmword ptr [rsp + 0x170]
141242424 movaps xmm8, xmmword ptr [rsp + 0x180]
14124242d movaps xmm9, xmmword ptr [rsp + 0x190]
141242436 movaps xmm10, xmmword ptr [rsp + 0x1a0]
14124243f movaps xmm11, xmmword ptr [rsp + 0x1b0]
141242448 movaps xmm12, xmmword ptr [rsp + 0x1c0]
141242451 movaps xmm13, xmmword ptr [rsp + 0x1d0]
14124245a movaps xmm14, xmmword ptr [rsp + 0x1e0]
141242463 movaps xmm15, xmmword ptr [rsp + 0x1f0]
14124246c add rsp, 0x208
141242473 pop rbx
141242474 pop rbp
141242475 pop rdi
141242476 pop rsi
141242477 pop r12
141242479 pop r13
14124247b pop r14
14124247d pop r15
14124247f ret 
141242480 lea rcx, [rip + 0x19dd2d1]
141242487 lea r8, [rip + 0x19dd2f2]
14124248e mov edx, 0x28
141242493 call 0x1428d9430
141242498 lea rcx, [rip + 0x19dd2b9]
14124249f lea r8, [rip + 0x19dd2f2]
1412424a6 mov edx, 0x28
1412424ab call 0x1428d9430
1412424b0 lea rcx, [rip + 0x19dd2a1]
1412424b7 lea r8, [rip + 0x19dd2f2]
1412424be mov edx, 0x28
1412424c3 call 0x1428d9430
1412424c8 lea rcx, [rip + 0x19dd171]