141233380 push r15
141233382 push r14
141233384 push r13
141233386 push r12
141233388 push rsi
141233389 push rdi
14123338a push rbp
14123338b push rbx
14123338c sub rsp, 0x208
141233393 movaps xmmword ptr [rsp + 0x1f0], xmm15
14123339c movaps xmmword ptr [rsp + 0x1e0], xmm14
1412333a5 movaps xmmword ptr [rsp + 0x1d0], xmm13
1412333ae movaps xmmword ptr [rsp + 0x1c0], xmm12
1412333b7 movaps xmmword ptr [rsp + 0x1b0], xmm11
1412333c0 movaps xmmword ptr [rsp + 0x1a0], xmm10
1412333c9 movaps xmmword ptr [rsp + 0x190], xmm9
1412333d2 movaps xmmword ptr [rsp + 0x180], xmm8
1412333db movaps xmmword ptr [rsp + 0x170], xmm7
1412333e3 movaps xmmword ptr [rsp + 0x160], xmm6
1412333eb mov rsi, qword ptr [rsp + 0x270]
1412333f3 test rsi, rsi
1412333f6 je 0x14123364c
1412333fc test r8, r8
1412333ff jne 0x141233411
141233401 lea r8, [rip + 0x19eb0b8]
141233408 xor ecx, ecx
14123340a xor edx, edx
14123340c call 0x1428d9518
141233411 mov r13, rcx
141233414 movss xmm9, dword ptr [r9]
141233419 movss xmm10, dword ptr [r9 + 8]
14123341f mov r12, rdx
141233422 movss xmm7, dword ptr [rdx]
141233426 mov rbp, r9
141233429 addss xmm7, dword ptr [r9 + 4]
14123342f movss dword ptr [rsp + 0xe0], xmm9
141233439 movss dword ptr [rsp + 0xe4], xmm10
141233443 lea rcx, [rsp + 0x50]
141233448 lea rbx, [rsp + 0xe0]
141233450 mov rdx, rbx
141233453 movaps xmm2, xmm7
141233456 mov qword ptr [rsp + 0xc8], r8
14123345e call 0x140c9cde0
141233463 mov rdi, qword ptr [rsp + 0xc8]
14123346b cmp rsi, rdi
14123346e cmovb rdi, rsi
141233472 mov rax, rdi
141233475 neg rax
141233478 mov qword ptr [rsp + 0x28], rax
14123347d xorps xmm8, xmm8
141233481 mov r9d, 1
141233487 xor eax, eax
141233489 lea r14, [rsp + 0x40]
14123348e xor ecx, ecx
141233490 test cl, 1
141233493 mov rdx, r13
141233496 je 0x14123355b
14123349c cmp rdi, rax
14123349f jbe 0x1412335aa
1412334a5 mov r8, rbp
1412334a8 mov rcx, r12
1412334ab lea r15, [rax + 1]
1412334af lea rdx, [rax + rax*2]
1412334b3 movss xmm0, dword ptr [r8 + rdx*4 + 8]
1412334ba movss xmm1, dword ptr [r8 + rdx*4]
1412334c0 movss xmm2, dword ptr [r8 + rdx*4 + 4]
1412334c7 addss xmm2, dword ptr [rcx + rax*4]
1412334cc movss dword ptr [rsp + 0xe0], xmm1
1412334d5 movss dword ptr [rsp + 0xe4], xmm0
1412334de mov rcx, r14
1412334e1 mov rdx, rbx
1412334e4 call 0x140c9cde0
1412334e9 movss xmm0, dword ptr [rsp + 0x44]
1412334ef subss xmm0, dword ptr [rsp + 0x54]
1412334f5 mulss xmm0, xmm0
1412334f9 movss xmm1, dword ptr [rsp + 0x48]
1412334ff movss xmm2, dword ptr [rsp + 0x40]
141233505 unpcklps xmm2, xmm1
141233508 movss xmm1, dword ptr [rsp + 0x50]
14123350e movss xmm3, dword ptr [rsp + 0x58]
141233514 unpcklps xmm1, xmm3
141233517 subps xmm2, xmm1
14123351a mulps xmm2, xmm2
14123351d addss xmm0, xmm2
141233521 movshdup xmm1, xmm2
141233525 addss xmm1, xmm0
141233529 xorps xmm0, xmm0
14123352c sqrtss xmm0, xmm1
141233530 addss xmm8, xmm0
141233535 mov eax, dword ptr [rsp + 0x48]
141233539 mov dword ptr [rsp + 0x58], eax
14123353d mov rax, qword ptr [rsp + 0x40]
141233542 mov qword ptr [rsp + 0x50], rax
141233547 mov cl, 1
141233549 mov rax, r15
14123354c xor r9d, r9d
14123354f test cl, 1
141233552 mov rdx, r13
141233555 jne 0x14123349c
14123355b mov r11, rdi
14123355e sub r11, rax
141233561 cmp r11, r9
141233564 cmovae r11, r9
141233568 lea r10, [r11 + rax]
14123356c cmp rax, r10
14123356f cmova r10, rax
141233573 cmp r10, rdi
141233576 jae 0x1412335aa
141233578 mov r8, rbp
14123357b mov rcx, r12
14123357e sub r11, r9
141233581 mov rax, qword ptr [rsp + 0x28]
141233586 lea r9, [rax + r10]
14123358a xor eax, eax
14123358c cmp r11, rax
14123358f je 0x14123359b
141233591 dec rax
141233594 cmp r9, rax
141233597 jne 0x14123358c
141233599 jmp 0x1412335aa
14123359b sub r10, rax
14123359e lea r15, [r10 + 1]
1412335a2 mov rax, r10
1412335a5 jmp 0x1412334af
1412335aa movaps xmm0, xmm8
1412335ae divss xmm0, dword ptr [rdx + 8]
1412335b3 addss xmm0, dword ptr [rip + 0x16f22b5]
1412335bb call 0x142923ee0
1412335c0 maxss xmm0, dword ptr [rip + 0x16f233c]
1412335c8 cvttss2si rax, xmm0
1412335cd mov rcx, rax
1412335d0 sar rcx, 0x3f
1412335d4 movaps xmm1, xmm0
1412335d7 subss xmm1, dword ptr [rip + 0x1726afd]
1412335df cvttss2si rdx, xmm1
1412335e4 and rdx, rcx
1412335e7 or rdx, rax
1412335ea xor ecx, ecx
1412335ec xorps xmm1, xmm1
1412335ef ucomiss xmm0, xmm1
1412335f2 cmovae rcx, rdx
1412335f6 ucomiss xmm0, dword ptr [rip + 0x1726ae3]
1412335fd mov rax, 0xffffffffffffffff
141233604 cmovbe rax, rcx
141233608 test rax, rax
14123360b js 0x141233617
14123360d xorps xmm0, xmm0
141233610 cvtsi2ss xmm0, rax
141233615 jmp 0x14123362f
141233617 mov rcx, rax
14123361a shr rcx, 1
14123361d and eax, 1
141233620 or rax, rcx
141233623 xorps xmm0, xmm0
141233626 cvtsi2ss xmm0, rax
14123362b addss xmm0, xmm0
14123362f mov rbx, qword ptr [rsp + 0x280]
141233637 test rbx, rbx
14123363a jne 0x14123365c
14123363c lea r8, [rip + 0x19eae95]
141233643 xor ecx, ecx
141233645 xor edx, edx
141233647 call 0x1428d9518
14123364c lea r8, [rip + 0x19eae55]
141233653 xor ecx, ecx
141233655 xor edx, edx
141233657 call 0x1428d9518
14123365c divss xmm8, xmm0
141233661 mov rax, qword ptr [rsp + 0x278]
141233669 movss xmm0, dword ptr [rax]
14123366d movss dword ptr [rsp + 0x24], xmm0
141233673 movss dword ptr [rsp + 0x50], xmm9
14123367a movss dword ptr [rsp + 0x54], xmm10
141233681 lea rcx, [rsp + 0x30]
141233686 lea rdx, [rsp + 0x50]
14123368b movaps xmm2, xmm7
14123368e call 0x140c9cde0
141233693 movss xmm7, dword ptr [rsp + 0x30]
141233699 movsd xmm13, qword ptr [rsp + 0x34]
1412336a0 cmp rbx, rdi
1412336a3 cmovb rdi, rbx
1412336a7 mov rcx, qword ptr [rsp + 0x288]
1412336af mov rax, qword ptr [rcx]
1412336b2 mov qword ptr [rsp + 0xa8], rax
1412336ba mov rax, qword ptr [rcx + 8]
1412336be mov qword ptr [rsp + 0xb0], rax
1412336c6 mov rax, qword ptr [rcx + 0x10]
1412336ca mov qword ptr [rsp + 0xb8], rax
1412336d2 mov rax, qword ptr [rcx + 0x18]
1412336d6 mov qword ptr [rsp + 0xc0], rax
1412336de mov ecx, 1
1412336e3 movss xmm14, dword ptr [rip + 0x16f2218]
1412336ec xorps xmm15, xmm15
1412336f0 xor eax, eax
1412336f2 xor edx, edx
1412336f4 test dl, 1
1412336f7 je 0x141233af5
1412336fd cmp rax, rdi
141233700 jae 0x141233b07
141233706 lea rcx, [rax + 1]
14123370a mov qword ptr [rsp + 0xa0], rcx
141233712 lea rcx, [rax + rax*2]
141233716 mov rdx, qword ptr [rsp + 0x278]
14123371e movss xmm0, dword ptr [rdx + rax*4]
141233723 movss dword ptr [rsp + 0x28], xmm0
141233729 movss xmm0, dword ptr [rbp + rcx*4 + 8]
14123372f movss xmm1, dword ptr [rbp + rcx*4]
141233735 movss xmm2, dword ptr [rbp + rcx*4 + 4]
14123373b addss xmm2, dword ptr [r12 + rax*4]
141233741 movss dword ptr [rsp + 0x50], xmm1
141233747 movss dword ptr [rsp + 0x54], xmm0
14123374d lea rcx, [rsp + 0x94]
141233755 lea rdx, [rsp + 0x50]
14123375a call 0x140c9cde0
14123375f movss xmm2, dword ptr [rsp + 0x30]
141233765 movss xmm1, dword ptr [rsp + 0x94]
14123376e movsd xmm3, qword ptr [rsp + 0x34]
141233774 movsd xmm4, qword ptr [rsp + 0x98]
14123377d movaps xmm0, xmm4
141233780 movaps xmmword ptr [rsp + 0x140], xmm1
141233788 movss xmm0, xmm1
14123378c movaps xmm1, xmm3
14123378f movaps xmmword ptr [rsp + 0x150], xmm2
141233797 movss xmm1, xmm2
14123379b subps xmm0, xmm1
14123379e mulps xmm0, xmm0
1412337a1 movaps xmmword ptr [rsp + 0x120], xmm4
1412337a9 movaps xmm1, xmm4
1412337ac movaps xmmword ptr [rsp + 0x130], xmm3
1412337b4 subss xmm1, xmm3
1412337b8 mulss xmm1, xmm1
1412337bc addss xmm1, xmm0
1412337c0 movshdup xmm0, xmm0
1412337c4 addss xmm0, xmm1
1412337c8 xorps xmm10, xmm10
1412337cc sqrtss xmm10, xmm0
1412337d1 movaps xmm0, xmm15
1412337d5 addss xmm0, xmm10
1412337da ucomiss xmm0, xmm8
1412337de jbe 0x141233a9c
1412337e4 xorps xmm11, xmm11
1412337e8 xorps xmm6, xmm6
1412337eb jmp 0x14123381a
1412337ed subss xmm15, xmm8
1412337f2 xorps xmm6, xmm6
1412337f5 maxss xmm15, xmm6
1412337fa movaps xmm0, xmm10
1412337fe subss xmm0, xmm11
141233803 addss xmm0, xmm15
141233808 ucomiss xmm0, xmm8
14123380c movaps xmm7, xmm12
141233810 movaps xmm13, xmm9
141233814 jbe 0x141233aa7
14123381a movaps xmm0, xmm11
14123381e movaps xmm11, xmm8
141233822 subss xmm11, xmm15
141233827 maxss xmm11, xmm6
14123382c addss xmm11, xmm0
141233831 movaps xmm9, xmm11
141233835 divss xmm9, xmm10
14123383a mov rcx, r13
14123383d movss xmm1, dword ptr [rsp + 0x28]
141233843 movss xmm12, dword ptr [rsp + 0x24]
14123384a movaps xmm2, xmm12
14123384e call 0x140c91f60
141233853 mulss xmm0, xmm9
141233858 addss xmm0, xmm12
14123385d mov rcx, r13
141233860 movaps xmm1, xmm0
141233863 call 0x140c91eb0
141233868 movss dword ptr [rsp + 0x4c], xmm0
14123386e ucomiss xmm0, xmm6
141233871 jb 0x141233da5
141233877 ucomiss xmm14, xmm0
14123387b jb 0x141233da5
141233881 ucomiss xmm0, xmm0
141233884 jp 0x141233e06
14123388a movaps xmm0, xmm14
14123388e subss xmm0, xmm9
141233893 movaps xmm1, xmmword ptr [rsp + 0x150]
14123389b mulss xmm1, xmm0
14123389f movaps xmm12, xmmword ptr [rsp + 0x140]
1412338a8 mulss xmm12, xmm9
1412338ad addss xmm12, xmm1
1412338b2 movsldup xmm0, xmm0
1412338b6 mulps xmm0, xmmword ptr [rsp + 0x130]
1412338be movsldup xmm9, xmm9
1412338c3 mulps xmm9, xmmword ptr [rsp + 0x120]
1412338cc addps xmm9, xmm0
1412338d0 movaps xmm2, xmm13
1412338d4 movss xmm5, dword ptr [rip + 0x184ae94]
1412338dc addss xmm2, xmm5
1412338e0 movaps xmm0, xmm12
1412338e4 subss xmm0, xmm7
1412338e8 movaps xmm1, xmm0
1412338eb mulss xmm1, xmm0
1412338ef movaps xmm3, xmm9
1412338f3 addss xmm3, xmm5
1412338f7 movaps xmm4, xmm13
1412338fb addss xmm4, xmm5
1412338ff subps xmm3, xmm4
141233902 movaps xmm4, xmm3
141233905 mulps xmm4, xmm3
141233908 addss xmm1, xmm4
14123390c movshdup xmm4, xmm4
141233910 addss xmm4, xmm1
141233914 sqrtss xmm4, xmm4
141233918 movaps xmm1, xmm14
14123391c divss xmm1, xmm4
141233920 mulss xmm0, xmm1
141233924 movsldup xmm5, xmm1
141233928 mulps xmm5, xmm3
14123392b movlhps xmm0, xmm5
14123392e movaps xmm1, xmm0
141233931 shufps xmm1, xmm5, 0xd8
141233935 shufps xmm0, xmm5, 0x58
141233939 movaps xmm3, xmm1
14123393c movss xmm5, dword ptr [rip + 0x16f1fc0]
141233944 mulps xmm3, xmm5
141233947 shufps xmm1, xmm1, 0x52
14123394b mulps xmm1, xmm5
14123394e shufps xmm3, xmm3, 9
141233952 subps xmm1, xmm3
141233955 movaps xmm3, xmm7
141233958 unpcklps xmm3, xmm2
14123395b movq xmm2, xmm4
14123395f movaps xmm4, xmm0
141233962 movaps xmm6, xmmword ptr [rip + 0x19eaad7]
141233969 mulps xmm4, xmm6
14123396c addps xmm4, xmmword ptr [rip + 0x19eaadd]
141233973 movaps xmm5, xmm1
141233976 mulps xmm5, xmm6
141233979 addps xmm5, xmm4
14123397c shufps xmm2, xmm2, 0x80
141233980 mulps xmm2, xmm0
141233983 xorps xmm6, xmm6
141233986 addps xmm2, xmm6
141233989 movaps xmm4, xmm1
14123398c mulps xmm4, xmm6
14123398f addps xmm2, xmm4
141233992 mulps xmm0, xmm6
141233995 addps xmm0, xmm6
141233998 mulps xmm1, xmmword ptr [rip + 0x19eaac1]
14123399f addps xmm1, xmm0
1412339a2 addps xmm0, xmm4
1412339a5 shufps xmm3, xmm13, 0x54
1412339aa addps xmm0, xmm3
1412339ad movaps xmmword ptr [rsp + 0xe0], xmm5
1412339b5 movaps xmmword ptr [rsp + 0xf0], xmm2
1412339bd movaps xmmword ptr [rsp + 0x100], xmm1
1412339c5 movaps xmmword ptr [rsp + 0x110], xmm0
1412339cd lea rbx, [rsp + 0x50]
1412339d2 mov rcx, rbx
1412339d5 lea rdx, [rsp + 0xe0]
1412339dd call 0x14089d560
1412339e2 movss xmm2, dword ptr [rip + 0x1727c46]
1412339ea mulss xmm7, xmm2
1412339ee cvttss2si eax, xmm7
1412339f2 movss xmm1, dword ptr [rip + 0x1770fc2]
1412339fa ucomiss xmm7, xmm1
1412339fd mov edx, 0x7fffffff
141233a02 cmova eax, edx
141233a05 ucomiss xmm7, xmm7
141233a08 mov ecx, 0
141233a0d cmovp eax, ecx
141233a10 movshdup xmm0, xmm13
141233a15 mulss xmm0, xmm2
141233a19 cvttss2si r15d, xmm0
141233a1e ucomiss xmm0, xmm1
141233a21 cmova r15d, edx
141233a25 ucomiss xmm0, xmm0
141233a28 cmovp r15d, ecx
141233a2c mov rcx, qword ptr [rsp + 0xb0]
141233a34 mov r14d, dword ptr [rcx]
141233a37 add r15d, eax
141233a3a mov rax, qword ptr [rsp + 0xb8]
141233a42 add r15d, dword ptr [rax]
141233a45 mov rcx, qword ptr [rsp + 0xc0]
141233a4d call 0x1400885d0
141233a52 mov dword ptr [rsp + 0x80], r14d
141233a5a mov dword ptr [rsp + 0x84], r15d
141233a62 mov dword ptr [rsp + 0x88], eax
141233a69 mov dword ptr [rsp + 0x8c], 0
141233a74 mov rax, qword ptr [rsp + 0xa8]
141233a7c mov rcx, qword ptr [rax]
141233a7f mov r15, qword ptr [rax + 8]
141233a83 mov rdx, rbx
141233a86 call 0x14120c430
141233a8b test al, al
141233a8d je 0x1412337ed
141233a93 mov byte ptr [r15], 1
141233a97 jmp 0x1412337ed
141233a9c movaps xmm12, xmm7
141233aa0 movaps xmm9, xmm13
141233aa4 xorps xmm6, xmm6
141233aa7 ucomiss xmm0, xmm6
141233aaa jb 0x141233e1e
141233ab0 mov eax, dword ptr [rsp + 0x9c]
141233ab7 mov dword ptr [rsp + 0x38], eax
141233abb mov rax, qword ptr [rsp + 0x94]
141233ac3 mov qword ptr [rsp + 0x30], rax
141233ac8 mov dl, 1
141233aca movaps xmm15, xmm0
141233ace movaps xmm7, xmm12
141233ad2 mov rax, qword ptr [rsp + 0xa0]
141233ada xor ecx, ecx
141233adc movss xmm0, dword ptr [rsp + 0x28]
141233ae2 movss dword ptr [rsp + 0x24], xmm0
141233ae8 movaps xmm13, xmm9
141233aec test dl, 1
141233aef jne 0x1412336fd
141233af5 cmp rax, rdi
141233af8 jae 0x141233b07
141233afa inc rax
141233afd dec rcx
141233b00 jne 0x141233af5
141233b02 jmp 0x1412336fd
141233b07 mulss xmm8, dword ptr [r13 + 0xc]
141233b0d ucomiss xmm15, xmm8
141233b11 jbe 0x141233d39
141233b17 dec rsi
141233b1a cmp rsi, qword ptr [rsp + 0x280]
141233b22 jae 0x141233e36
141233b28 mov rax, qword ptr [rsp + 0xc8]
141233b30 movss xmm2, dword ptr [r12 + rax*4 - 4]
141233b37 lea rax, [rsi + rsi*2]
141233b3b movss xmm0, dword ptr [rbp + rax*4]
141233b41 movss xmm1, dword ptr [rbp + rax*4 + 8]
141233b47 addss xmm2, dword ptr [rbp + rax*4 + 4]
141233b4d movss dword ptr [rsp + 0x50], xmm0
141233b53 movss dword ptr [rsp + 0x54], xmm1
141233b59 lea rcx, [rsp + 0xd4]
141233b61 lea rdx, [rsp + 0x50]
141233b66 call 0x140c9cde0
141233b6b movss xmm0, dword ptr [rsp + 0xd4]
141233b74 movss xmm1, dword ptr [rsp + 0xd8]
141233b7d movss xmm5, dword ptr [rip + 0x184abeb]
141233b85 addss xmm1, xmm5
141233b89 movss xmm2, dword ptr [rsp + 0xdc]
141233b92 unpcklps xmm1, xmm2
141233b95 movaps xmm2, xmm13
141233b99 addss xmm2, xmm5
141233b9d subss xmm0, xmm7
141233ba1 movshdup xmm6, xmm13
141233ba6 movaps xmm3, xmm0
141233ba9 mulss xmm3, xmm0
141233bad movaps xmm4, xmm13
141233bb1 addss xmm4, xmm5
141233bb5 subps xmm1, xmm4
141233bb8 movaps xmm4, xmm1
141233bbb mulps xmm4, xmm1
141233bbe addss xmm3, xmm4
141233bc2 movshdup xmm4, xmm4
141233bc6 addss xmm4, xmm3
141233bca sqrtss xmm4, xmm4
141233bce divss xmm14, xmm4
141233bd3 mulss xmm0, xmm14
141233bd8 movsldup xmm3, xmm14
141233bdd mulps xmm3, xmm1
141233be0 movlhps xmm0, xmm3
141233be3 movaps xmm1, xmm0
141233be6 shufps xmm1, xmm3, 0xd8
141233bea shufps xmm0, xmm3, 0x58
141233bee movaps xmm3, xmm1
141233bf1 movss xmm5, dword ptr [rip + 0x16f1d0b]
141233bf9 mulps xmm3, xmm5
141233bfc shufps xmm1, xmm1, 0x52
141233c00 mulps xmm1, xmm5
141233c03 shufps xmm3, xmm3, 9
141233c07 subps xmm1, xmm3
141233c0a movaps xmm3, xmm7
141233c0d unpcklps xmm3, xmm2
141233c10 shufps xmm3, xmm13, 0x54
141233c15 movq xmm2, xmm4
141233c19 movaps xmm4, xmm0
141233c1c movaps xmm8, xmmword ptr [rip + 0x19ea81c]
141233c24 mulps xmm4, xmm8
141233c28 addps xmm4, xmmword ptr [rip + 0x19ea821]
141233c2f mulps xmm8, xmm1
141233c33 addps xmm8, xmm4
141233c37 shufps xmm2, xmm2, 0x80
141233c3b mulps xmm2, xmm0
141233c3e xorps xmm4, xmm4
141233c41 addps xmm2, xmm4
141233c44 movaps xmm5, xmm1
141233c47 mulps xmm5, xmm4
141233c4a addps xmm2, xmm5
141233c4d mulps xmm0, xmm4
141233c50 mulps xmm1, xmmword ptr [rip + 0x19ea809]
141233c57 addps xmm0, xmm4
141233c5a addps xmm1, xmm0
141233c5d addps xmm0, xmm5
141233c60 addps xmm0, xmm3
141233c63 movaps xmmword ptr [rsp + 0xe0], xmm8
141233c6c movaps xmmword ptr [rsp + 0xf0], xmm2
141233c74 movaps xmmword ptr [rsp + 0x100], xmm1
141233c7c movaps xmmword ptr [rsp + 0x110], xmm0
141233c84 lea rsi, [rsp + 0x50]
141233c89 lea rdx, [rsp + 0xe0]
141233c91 mov rcx, rsi
141233c94 call 0x14089d560
141233c99 movss xmm1, dword ptr [rip + 0x172798f]
141233ca1 mulss xmm7, xmm1
141233ca5 cvttss2si eax, xmm7
141233ca9 movss xmm0, dword ptr [rip + 0x1770d0b]
141233cb1 ucomiss xmm7, xmm0
141233cb4 mov ecx, 0x7fffffff
141233cb9 cmova eax, ecx
141233cbc xor edx, edx
141233cbe ucomiss xmm7, xmm7
141233cc1 cmovp eax, edx
141233cc4 mulss xmm6, xmm1
141233cc8 cvttss2si edi, xmm6
141233ccc ucomiss xmm6, xmm0
141233ccf cmova edi, ecx
141233cd2 ucomiss xmm6, xmm6
141233cd5 cmovp edi, edx
141233cd8 mov rcx, qword ptr [rsp + 0xb0]
141233ce0 mov ebp, dword ptr [rcx]
141233ce2 add edi, eax
141233ce4 mov rax, qword ptr [rsp + 0xb8]
141233cec add edi, dword ptr [rax]
141233cee mov rcx, qword ptr [rsp + 0xc0]
141233cf6 call 0x1400885d0
141233cfb mov dword ptr [rsp + 0x80], ebp
141233d02 mov dword ptr [rsp + 0x84], edi
141233d09 mov dword ptr [rsp + 0x88], eax
141233d10 mov dword ptr [rsp + 0x8c], 0
141233d1b mov rax, qword ptr [rsp + 0xa8]
141233d23 mov rcx, qword ptr [rax]
141233d26 mov rdi, qword ptr [rax + 8]
141233d2a mov rdx, rsi
141233d2d call 0x14120c430
141233d32 test al, al
141233d34 je 0x141233d39
141233d36 mov byte ptr [rdi], 1
141233d39 movaps xmm6, xmmword ptr [rsp + 0x160]
141233d41 movaps xmm7, xmmword ptr [rsp + 0x170]
141233d49 movaps xmm8, xmmword ptr [rsp + 0x180]
141233d52 movaps xmm9, xmmword ptr [rsp + 0x190]
141233d5b movaps xmm10, xmmword ptr [rsp + 0x1a0]
141233d64 movaps xmm11, xmmword ptr [rsp + 0x1b0]
141233d6d movaps xmm12, xmmword ptr [rsp + 0x1c0]
141233d76 movaps xmm13, xmmword ptr [rsp + 0x1d0]
141233d7f movaps xmm14, xmmword ptr [rsp + 0x1e0]
141233d88 movaps xmm15, xmmword ptr [rsp + 0x1f0]
141233d91 add rsp, 0x208
141233d98 pop rbx
141233d99 pop rbp
141233d9a pop rdi
141233d9b pop rsi
141233d9c pop r12
141233d9e pop r13
141233da0 pop r14
141233da2 pop r15
141233da4 ret 
141233da5 lea rax, [rsp + 0x4c]
141233daa mov qword ptr [rsp + 0xe0], rax
141233db2 lea rax, [rip - 0x4a74a9]
141233db9 mov qword ptr [rsp + 0xe8], rax
141233dc1 lea rax, [rip + 0x19e6810]
141233dc8 mov qword ptr [rsp + 0x50], rax
141233dcd mov qword ptr [rsp + 0x58], 1
141233dd6 mov qword ptr [rsp + 0x70], 0
141233ddf lea rax, [rsp + 0xe0]
141233de7 mov qword ptr [rsp + 0x60], rax
141233dec mov qword ptr [rsp + 0x68], 1
141233df5 lea rdx, [rip + 0x19ea74c]
141233dfc lea rcx, [rsp + 0x50]
141233e01 call 0x1428d9390
141233e06 lea rcx, [rip + 0x19e67db]
141233e0d lea r8, [rip + 0x19ea734]
141233e14 mov edx, 0x1d
141233e19 call 0x1428d9430
141233e1e lea rcx, [rip + 0x19ea6e3]
141233e25 lea r8, [rip + 0x19ea704]
141233e2c mov edx, 0x26
141233e31 call 0x1428d9430
141233e36 lea r8, [rip + 0x19ea6b3]
141233e3d mov rcx, rsi
141233e40 mov rdx, qword ptr [rsp + 0x280]
141233e48 call 0x1428d9518
141233e4d int3 
