141242530 push r15
141242532 push r14
141242534 push r13
141242536 push r12
141242538 push rsi
141242539 push rdi
14124253a push rbp
14124253b push rbx
14124253c sub rsp, 0x248
141242543 movaps xmmword ptr [rsp + 0x230], xmm11
14124254c movaps xmmword ptr [rsp + 0x220], xmm10
141242555 movaps xmmword ptr [rsp + 0x210], xmm9
14124255e movaps xmmword ptr [rsp + 0x200], xmm8
141242567 movaps xmmword ptr [rsp + 0x1f0], xmm7
14124256f movaps xmmword ptr [rsp + 0x1e0], xmm6
141242577 mov rsi, r9
14124257a mov rbx, r8
14124257d mov qword ptr [rsp + 0xf8], rdx
141242585 mov qword ptr [rsp + 0xd0], rcx
14124258d mov rbp, qword ptr [rsp + 0x2e0]
141242595 mov r15, qword ptr [rsp + 0x2d8]
14124259d mov r14, qword ptr [rsp + 0x2c8]
1412425a5 mov r13, qword ptr [rsp + 0x2c0]
1412425ad mov rdi, qword ptr [rsp + 0x2b0]
1412425b5 mov rcx, r13
1412425b8 mov rdx, r14
1412425bb call 0x14067dd80
1412425c0 mov r12, rdx
1412425c3 mov qword ptr [rsp + 0xe8], rbx
1412425cb mov qword ptr [rsp + 0x1b0], rbx
1412425d3 mov qword ptr [rsp + 0xf0], rsi
1412425db mov qword ptr [rsp + 0x1b8], rsi
1412425e3 mov qword ptr [rsp + 0x1c0], rdi
1412425eb mov rdi, r13
1412425ee mov rax, qword ptr [rsp + 0x2b8]
1412425f6 mov qword ptr [rsp + 0x1c8], rax
1412425fe mov qword ptr [rsp + 0x1d0], r13
141242606 mov qword ptr [rsp + 0x1d8], r14
14124260e mov r8, r15
141242611 mov rbx, 0xffffffffffffffff
141242618 mov al, 1
14124261a mov r11d, 2
141242620 xor r9d, r9d
141242623 mov r10d, 1
141242629 movss xmm6, dword ptr [rip + 0x19dd1ab]
141242631 movss xmm7, dword ptr [rip + 0x16e32cb]
141242639 movss xmm8, dword ptr [rip + 0x19dd19e]
141242642 movss xmm9, dword ptr [rip + 0x16e3225]
14124264b movss xmm10, dword ptr [rip + 0x184af6c]
141242654 xor esi, esi
141242656 xor edx, edx
141242658 jmp 0x14124270d
14124265d mov rax, qword ptr [rsp + 0x2e8]
141242665 mov edx, dword ptr [rax + rbx*8]
141242668 movss xmm2, dword ptr [rax + rbx*8 + 4]
14124266e movss xmm0, dword ptr [rsp + 0x154]
141242677 mulss xmm0, xmm10
14124267c mov rax, qword ptr [rsp + 0x2d0]
141242684 movups xmm1, xmmword ptr [rax]
141242687 mov dword ptr [rsp + 0x60], ebx
14124268b mov eax, dword ptr [rsp + 0x2f8]
141242692 mov dword ptr [rsp + 0x58], eax
141242696 movups xmmword ptr [rsp + 0x48], xmm1
14124269b mov qword ptr [rsp + 0x40], r14
1412426a0 mov rax, qword ptr [rsp + 0x2b0]
1412426a8 mov qword ptr [rsp + 0x38], rax
1412426ad movss dword ptr [rsp + 0x28], xmm0
1412426b3 mov rax, qword ptr [rsp + 0xf0]
1412426bb mov qword ptr [rsp + 0x20], rax
1412426c0 mov byte ptr [rsp + 0x30], 0
1412426c5 mov rcx, qword ptr [rsp + 0xd0]
1412426cd mov r9, qword ptr [rsp + 0xe8]
1412426d5 call 0x141243740
1412426da xor eax, eax
1412426dc mov r8, qword ptr [rsp + 0x2d8]
1412426e4 mov rdx, qword ptr [rsp + 0x168]
1412426ec mov rcx, qword ptr [rsp + 0x160]
1412426f4 mov r9, qword ptr [rsp + 0x1a8]
1412426fc mov r10d, 1
141242702 mov r11, qword ptr [rsp + 0x1a0]
14124270a mov sil, 1
14124270d mov r14, rbx
141242710 cmp r11, 1
141242714 je 0x141242750
141242716 cmp r11, 2
14124271a jne 0x141243131
141242720 mov rbx, r9
141242723 cmp r9, r12
141242726 jae 0x141243131
14124272c nop dword ptr [rax]
141242730 cmp rbx, rbp
141242733 jae 0x14124365b
141242739 test byte ptr [r8 + rbx], 0xf
14124273e jne 0x141242760
141242740 inc rbx
141242743 cmp r12, rbx
141242746 jne 0x141242730
141242748 jmp 0x141243131
14124274d nop dword ptr [rax]
141242750 mov rbx, r15
141242753 jmp 0x141242764
141242755 nop word ptr cs:[rax + rax]
141242760 lea r9, [rbx + 1]
141242764 test al, 1
141242766 cmovne rcx, rbx
14124276a mov r15, r9
14124276d cmp r9, r12
141242770 jae 0x1412427b0
141242772 nop word ptr cs:[rax + rax]
141242780 cmp r15, rbp
141242783 jae 0x14124365b
141242789 test byte ptr [r8 + r15], 0xf
14124278e jne 0x1412427b8
141242790 inc r15
141242793 cmp r12, r15
141242796 jne 0x141242780
141242798 mov r9, r12
14124279b mov r15, r12
14124279e xor r11d, r11d
1412427a1 jmp 0x1412427c2
1412427a3 nop word ptr cs:[rax + rax]
1412427b0 mov r15, r9
1412427b3 xor r11d, r11d
1412427b6 jmp 0x1412427c2
1412427b8 lea r9, [r15 + 1]
1412427bc mov r11d, 1
1412427c2 test r11, r11
1412427c5 cmove r13, rbx
1412427c9 cmove rdx, r10
1412427cd mov sil, 1
1412427d0 xor eax, eax
1412427d2 cmp r14, rbx
1412427d5 jae 0x14124270d
1412427db mov qword ptr [rsp + 0xd8], r13
1412427e3 mov r13, qword ptr [rsp + 0x2c8]
1412427eb cmp r14, r13
1412427ee jae 0x141243674
1412427f4 mov qword ptr [rsp + 0x1a0], r11
1412427fc mov qword ptr [rsp + 0x1a8], r9
141242804 mov qword ptr [rsp + 0x160], rcx
14124280c mov qword ptr [rsp + 0x168], rdx
141242814 lea rax, [r14 + r14*2]
141242818 mov ecx, dword ptr [rdi + rax*4 + 8]
14124281c lea rdx, [rsp + 0x118]
141242824 mov dword ptr [rdx + 8], ecx
141242827 mov rax, qword ptr [rdi + rax*4]
14124282b mov qword ptr [rdx], rax
14124282e mov qword ptr [rsp + 0x110], r14
141242836 mov dword ptr [rsp + 0x100], 0
141242841 mov dword ptr [rsp + 0x124], 0
14124284c mov byte ptr [rsp + 0x128], 0
141242854 cmp rbx, r13
141242857 jae 0x141243686
14124285d lea rax, [rbx + rbx*2]
141242861 mov ecx, dword ptr [rdi + rax*4 + 8]
141242865 lea rdx, [rsp + 0x148]
14124286d mov dword ptr [rdx + 8], ecx
141242870 mov rax, qword ptr [rdi + rax*4]
141242874 mov qword ptr [rdx], rax
141242877 mov qword ptr [rsp + 0x140], rbx
14124287f mov dword ptr [rsp + 0x130], 0
14124288a mov dword ptr [rsp + 0x154], 0
141242895 mov byte ptr [rsp + 0x158], 0
14124289d lea rbp, [r14 + 1]
1412428a1 mov rax, qword ptr [rsp + 0x2e0]
1412428a9 mov rdi, r14
1412428ac mov r14, rax
1412428af mov qword ptr [rsp + 0x30], rax
1412428b4 mov rax, qword ptr [rsp + 0x2d8]
1412428bc mov qword ptr [rsp + 0x28], rax
1412428c1 mov qword ptr [rsp + 0x20], r13
1412428c6 lea rcx, [rsp + 0x170]
1412428ce lea rdx, [rsp + 0x100]
1412428d6 mov r8, rbp
1412428d9 mov r9, qword ptr [rsp + 0x2c0]
1412428e1 call 0x141fe09c0
1412428e6 lea rsi, [rbx - 1]
1412428ea mov qword ptr [rsp + 0x30], r14
1412428ef mov rax, qword ptr [rsp + 0x2d8]
1412428f7 mov qword ptr [rsp + 0x28], rax
1412428fc mov r9, qword ptr [rsp + 0x2c0]
141242904 mov qword ptr [rsp + 0x20], r13
141242909 lea rcx, [rsp + 0xa0]
141242911 lea rdx, [rsp + 0x130]
141242919 mov r8, rsi
14124291c call 0x141fe09c0
141242921 mov r8, qword ptr [rsp + 0x110]
141242929 mov rdx, qword ptr [rsp + 0x140]
141242931 cmp r8, rdx
141242934 mov qword ptr [rsp + 0xe0], rdi
14124293c jae 0x141242c8e
141242942 lea r13, [rdi + 2]
141242946 cmp rbp, rbx
141242949 setae r14b
14124294d cmovae r13, rbp
141242951 mov rbp, qword ptr [rsp + 0x2e0]
141242959 lea rdi, [rsp + 0x18c]
141242961 jmp 0x141242a31
141242966 nop word ptr cs:[rax + rax]
141242970 dec rsi
141242973 mov qword ptr [rsp + 0x30], rbp
141242978 mov rax, qword ptr [rsp + 0x2d8]
141242980 mov qword ptr [rsp + 0x28], rax
141242985 mov rax, qword ptr [rsp + 0x2c8]
14124298d mov qword ptr [rsp + 0x20], rax
141242992 lea rcx, [rsp + 0x70]
141242997 lea rdx, [rsp + 0x130]
14124299f mov r8, rsi
1412429a2 mov r9, qword ptr [rsp + 0x2c0]
1412429aa call 0x141fe09c0
1412429af movups xmm0, xmmword ptr [rsp + 0x70]
1412429b4 movups xmm1, xmmword ptr [rsp + 0x80]
1412429bc mov rax, qword ptr [rsp + 0x90]
1412429c4 mov qword ptr [rsp + 0xc0], rax
1412429cc movzx eax, byte ptr [rsp + 0x98]
1412429d4 mov byte ptr [rsp + 0xc8], al
1412429db mov eax, dword ptr [rsp + 0x99]
1412429e2 mov dword ptr [rsp + 0xc9], eax
1412429e9 movzx eax, word ptr [rsp + 0x9d]
1412429f1 mov word ptr [rsp + 0xcd], ax
1412429f9 movzx eax, byte ptr [rsp + 0x9f]
141242a01 mov byte ptr [rsp + 0xcf], al
141242a08 movaps xmmword ptr [rsp + 0xb0], xmm1
141242a10 movaps xmmword ptr [rsp + 0xa0], xmm0
141242a18 mov r8, qword ptr [rsp + 0x110]
141242a20 mov rdx, qword ptr [rsp + 0x140]
141242a28 cmp r8, rdx
141242a2b jae 0x141242c96
141242a31 movzx ecx, byte ptr [rsp + 0x198]
141242a39 movzx eax, byte ptr [rsp + 0xc8]
141242a41 cmp cl, 2
141242a44 jne 0x141242a60
141242a46 cmp al, 2
141242a48 jne 0x141242b70
141242a4e jmp 0x141242c96
141242a53 nop word ptr cs:[rax + rax]
141242a60 cmp al, 2
141242a62 je 0x141242a7b
141242a64 movss xmm0, dword ptr [rsp + 0xb8]
141242a6d ucomiss xmm0, dword ptr [rsp + 0x188]
141242a75 jbe 0x141242b70
141242a7b cmp byte ptr [rsp + 0x128], 1
141242a83 je 0x141243643
141242a89 mov rax, qword ptr [rsp + 0x170]
141242a91 movss xmm0, dword ptr [rsp + 0x178]
141242a9a mov qword ptr [rsp + 0x110], rax
141242aa2 mov dword ptr [rsp + 0x100], 1
141242aad mov eax, dword ptr [rdi - 8]
141242ab0 lea rdx, [rsp + 0x118]
141242ab8 mov dword ptr [rdx - 0xc], eax
141242abb mov rax, qword ptr [rdi - 0x10]
141242abf mov qword ptr [rdx - 0x14], rax
141242ac3 mov eax, dword ptr [rdi + 8]
141242ac6 mov dword ptr [rdx + 8], eax
141242ac9 mov rax, qword ptr [rdi]
141242acc mov qword ptr [rdx], rax
141242acf addss xmm0, dword ptr [rsp + 0x124]
141242ad8 movss dword ptr [rsp + 0x124], xmm0
141242ae1 mov byte ptr [rsp + 0x128], cl
141242ae8 test r14b, 1
141242aec jne 0x141242c02
141242af2 cmp r13, rbx
141242af5 ja 0x141242c02
141242afb mov rbp, r13
141242afe adc rbp, 0
141242b02 cmp r13, rbx
141242b05 movzx r14d, r14b
141242b09 mov eax, 1
141242b0e cmovae r14d, eax
141242b12 mov rax, qword ptr [rsp + 0x2e0]
141242b1a mov qword ptr [rsp + 0x30], rax
141242b1f mov rax, qword ptr [rsp + 0x2d8]
141242b27 mov qword ptr [rsp + 0x28], rax
141242b2c mov rax, qword ptr [rsp + 0x2c8]
141242b34 mov qword ptr [rsp + 0x20], rax
141242b39 lea rcx, [rsp + 0x70]
141242b3e lea rdx, [rsp + 0x100]
141242b46 mov r8, r13
141242b49 mov r9, qword ptr [rsp + 0x2c0]
141242b51 call 0x141fe09c0
141242b56 mov r13, rbp
141242b59 mov rbp, qword ptr [rsp + 0x2e0]
141242b61 jmp 0x141242c0a
141242b66 nop word ptr cs:[rax + rax]
141242b70 cmp byte ptr [rsp + 0x158], 1
141242b78 je 0x141243643
141242b7e mov rcx, qword ptr [rsp + 0xa0]
141242b86 movss xmm0, dword ptr [rsp + 0xa8]
141242b8f mov qword ptr [rsp + 0x140], rcx
141242b97 mov dword ptr [rsp + 0x130], 1
141242ba2 lea rdx, [rsp + 0xbc]
141242baa mov ecx, dword ptr [rdx - 8]
141242bad lea r8, [rsp + 0x148]
141242bb5 mov dword ptr [r8 - 0xc], ecx
141242bb9 mov rcx, qword ptr [rdx - 0x10]
141242bbd mov qword ptr [r8 - 0x14], rcx
141242bc1 mov ecx, dword ptr [rdx + 8]
141242bc4 mov dword ptr [r8 + 8], ecx
141242bc8 mov rcx, qword ptr [rdx]
141242bcb mov qword ptr [r8], rcx
141242bce addss xmm0, dword ptr [rsp + 0x154]
141242bd7 movss dword ptr [rsp + 0x154], xmm0
141242be0 mov byte ptr [rsp + 0x158], al
141242be7 cmp qword ptr [rsp + 0xe0], rsi
141242bef jb 0x141242970
141242bf5 mov byte ptr [rsp + 0x98], 2
141242bfd jmp 0x1412429af
141242c02 mov byte ptr [rsp + 0x98], 2
141242c0a movups xmm0, xmmword ptr [rsp + 0x70]
141242c0f movups xmm1, xmmword ptr [rsp + 0x80]
141242c17 mov rax, qword ptr [rsp + 0x90]
141242c1f mov qword ptr [rsp + 0x190], rax
141242c27 movzx eax, byte ptr [rsp + 0x98]
141242c2f mov byte ptr [rsp + 0x198], al
141242c36 mov eax, dword ptr [rsp + 0x99]
141242c3d mov dword ptr [rsp + 0x199], eax
141242c44 movzx eax, word ptr [rsp + 0x9d]
141242c4c mov word ptr [rsp + 0x19d], ax
141242c54 movzx eax, byte ptr [rsp + 0x9f]
141242c5c mov byte ptr [rsp + 0x19f], al
141242c63 movaps xmmword ptr [rsp + 0x180], xmm1
141242c6b movaps xmmword ptr [rsp + 0x170], xmm0
141242c73 mov r8, qword ptr [rsp + 0x110]
141242c7b mov rdx, qword ptr [rsp + 0x140]
141242c83 cmp r8, rdx
141242c86 jb 0x141242a31
141242c8c jmp 0x141242c96
141242c8e mov rbp, qword ptr [rsp + 0x2e0]
141242c96 cmp r8, rdx
141242c99 mov r14, qword ptr [rsp + 0x2b8]
141242ca1 mov r13, qword ptr [rsp + 0xd8]
141242ca9 mov rdi, qword ptr [rsp + 0x2c0]
141242cb1 jne 0x141242d80
141242cb7 movss xmm0, dword ptr [rsp + 0x124]
141242cc0 addss xmm0, dword ptr [rsp + 0x154]
141242cc9 ucomiss xmm6, xmm0
141242ccc jb 0x141242d80
141242cd2 ucomiss xmm0, xmm7
141242cd5 jb 0x141242d80
141242cdb mov rcx, qword ptr [rsp + 0xe0]
141242ce3 mov rax, rcx
141242ce6 cmp r8, rcx
141242ce9 je 0x141242d35
141242ceb mov qword ptr [rsp + 0x30], rbp
141242cf0 mov rax, qword ptr [rsp + 0x2d8]
141242cf8 mov qword ptr [rsp + 0x28], rax
141242cfd mov rax, qword ptr [rsp + 0x2c8]
141242d05 mov qword ptr [rsp + 0x20], rax
141242d0a lea rcx, [rsp + 0x70]
141242d0f lea rdx, [rsp + 0x100]
141242d17 mov r9, rdi
141242d1a call 0x141fe09c0
141242d1f cmp byte ptr [rsp + 0x98], 2
141242d27 jne 0x1412430bf
141242d2d mov rax, qword ptr [rsp + 0x110]
141242d35 cmp rax, rbx
141242d38 je 0x141242d80
141242d3a mov qword ptr [rsp + 0x30], rbp
141242d3f mov rcx, qword ptr [rsp + 0x2d8]
141242d47 mov qword ptr [rsp + 0x28], rcx
141242d4c mov rcx, qword ptr [rsp + 0x2c8]
141242d54 mov qword ptr [rsp + 0x20], rcx
141242d59 lea rcx, [rsp + 0x70]
141242d5e lea rdx, [rsp + 0x130]
141242d66 mov r8, rax
141242d69 mov r9, rdi
141242d6c call 0x141fe09c0
141242d71 cmp byte ptr [rsp + 0x98], 2
141242d79 jne 0x1412430bf
141242d7f nop 
141242d80 movss xmm0, dword ptr [rsp + 0x124]
141242d89 ucomiss xmm0, xmm7
141242d8c jae 0x141242ebf
141242d92 xor ebp, ebp
141242d94 movss xmm0, dword ptr [rsp + 0x154]
141242d9d ucomiss xmm0, xmm7
141242da0 mov rax, qword ptr [rsp + 0x140]
141242da8 jb 0x141242f38
141242dae mov qword ptr [rsp + 0x70], rax
141242db3 mov qword ptr [rsp + 0x78], rbx
141242db8 mov byte ptr [rsp + 0x80], 0
141242dc0 mov eax, dword ptr [rsp + 0x2f8]
141242dc7 mov dword ptr [rsp + 0x28], eax
141242dcb mov rax, qword ptr [rsp + 0x2d0]
141242dd3 mov qword ptr [rsp + 0x20], rax
141242dd8 lea rcx, [rsp + 0x1b0]
141242de0 mov rdx, qword ptr [rsp + 0xd0]
141242de8 mov r8, qword ptr [rsp + 0xf8]
141242df0 lea r9, [rsp + 0x70]
141242df5 call 0x141243c20
141242dfa mov sil, 1
141242dfd mov rax, qword ptr [rsp + 0x140]
141242e05 cmp qword ptr [rsp + 0x110], rax
141242e0d je 0x141242f48
141242e13 test bpl, bpl
141242e16 mov rbp, qword ptr [rsp + 0x2e0]
141242e1e je 0x1412430a0
141242e24 mov r9, qword ptr [rsp + 0xe0]
141242e2c cmp r9, qword ptr [rsp + 0x2f0]
141242e34 mov rax, qword ptr [rsp + 0xf0]
141242e3c jae 0x1412436d2
141242e42 mov rcx, qword ptr [rsp + 0x2e8]
141242e4a mov edx, dword ptr [rcx + r9*8]
141242e4e movss xmm2, dword ptr [rcx + r9*8 + 4]
141242e55 movss xmm0, dword ptr [rsp + 0x124]
141242e5e mulss xmm0, xmm10
141242e63 mov rcx, qword ptr [rsp + 0x2d0]
141242e6b movups xmm1, xmmword ptr [rcx]
141242e6e mov dword ptr [rsp + 0x60], r9d
141242e73 mov ecx, dword ptr [rsp + 0x2f8]
141242e7a mov dword ptr [rsp + 0x58], ecx
141242e7e movups xmmword ptr [rsp + 0x48], xmm1
141242e83 mov qword ptr [rsp + 0x40], r14
141242e88 mov rcx, qword ptr [rsp + 0x2b0]
141242e90 mov qword ptr [rsp + 0x38], rcx
141242e95 movss dword ptr [rsp + 0x28], xmm0
141242e9b mov qword ptr [rsp + 0x20], rax
141242ea0 mov byte ptr [rsp + 0x30], 1
141242ea5 mov rcx, qword ptr [rsp + 0xd0]
141242ead mov r9, qword ptr [rsp + 0xe8]
141242eb5 call 0x141243740
141242eba jmp 0x1412430a0
141242ebf mov rax, qword ptr [rsp + 0x110]
141242ec7 mov rcx, qword ptr [rsp + 0xe0]
141242ecf mov qword ptr [rsp + 0x70], rcx
141242ed4 mov qword ptr [rsp + 0x78], rax
141242ed9 mov byte ptr [rsp + 0x80], 0
141242ee1 mov eax, dword ptr [rsp + 0x2f8]
141242ee8 mov dword ptr [rsp + 0x28], eax
141242eec mov rax, qword ptr [rsp + 0x2d0]
141242ef4 mov qword ptr [rsp + 0x20], rax
141242ef9 lea rcx, [rsp + 0x1b0]
141242f01 mov rdx, qword ptr [rsp + 0xd0]
141242f09 mov r8, qword ptr [rsp + 0xf8]
141242f11 lea r9, [rsp + 0x70]
141242f16 call 0x141243c20
141242f1b mov bpl, 1
141242f1e movss xmm0, dword ptr [rsp + 0x154]
141242f27 ucomiss xmm0, xmm7
141242f2a mov rax, qword ptr [rsp + 0x140]
141242f32 jae 0x141242dae
141242f38 xor esi, esi
141242f3a cmp qword ptr [rsp + 0x110], rax
141242f42 jne 0x141242e13
141242f48 or sil, bpl
141242f4b mov rbp, qword ptr [rsp + 0x2e0]
141242f53 je 0x1412430a0
141242f59 movss xmm11, dword ptr [rsp + 0x124]
141242f63 addss xmm11, dword ptr [rsp + 0x154]
141242f6d ucomiss xmm11, xmm8
141242f71 jb 0x1412426da
141242f77 mulss xmm11, xmm9
141242f7c mov rcx, qword ptr [rsp + 0xe0]
141242f84 mov rax, qword ptr [rsp + 0x2f0]
141242f8c cmp rcx, rax
141242f8f jae 0x1412436e9
141242f95 mov rax, qword ptr [rsp + 0x2e8]
141242f9d mov edx, dword ptr [rax + rcx*8]
141242fa0 movss xmm2, dword ptr [rax + rcx*8 + 4]
141242fa6 mov r13, qword ptr [rsp + 0x2d0]
141242fae movups xmm0, xmmword ptr [r13]
141242fb3 mov dword ptr [rsp + 0x60], ecx
141242fb7 mov ebp, dword ptr [rsp + 0x2f8]
141242fbe mov dword ptr [rsp + 0x58], ebp
141242fc2 movups xmmword ptr [rsp + 0x48], xmm0
141242fc7 mov qword ptr [rsp + 0x40], r14
141242fcc mov r14, qword ptr [rsp + 0x2b0]
141242fd4 mov qword ptr [rsp + 0x38], r14
141242fd9 movss dword ptr [rsp + 0x28], xmm11
141242fe0 mov rsi, qword ptr [rsp + 0xf0]
141242fe8 mov qword ptr [rsp + 0x20], rsi
141242fed mov byte ptr [rsp + 0x30], 1
141242ff2 mov rdi, qword ptr [rsp + 0xd0]
141242ffa mov rcx, rdi
141242ffd mov r14, qword ptr [rsp + 0xe8]
141243005 mov r9, r14
141243008 call 0x141243740
14124300d mov rdx, qword ptr [rsp + 0x2f0]
141243015 cmp rbx, rdx
141243018 jae 0x1412436fd
14124301e mov rax, qword ptr [rsp + 0x2e8]
141243026 mov edx, dword ptr [rax + rbx*8]
141243029 movss xmm2, dword ptr [rax + rbx*8 + 4]
14124302f movups xmm0, xmmword ptr [r13]
141243034 mov dword ptr [rsp + 0x60], ebx
141243038 mov dword ptr [rsp + 0x58], ebp
14124303c movups xmmword ptr [rsp + 0x48], xmm0
141243041 mov rax, qword ptr [rsp + 0x2b8]
141243049 mov qword ptr [rsp + 0x40], rax
14124304e mov rax, qword ptr [rsp + 0x2b0]
141243056 mov qword ptr [rsp + 0x38], rax
14124305b movss dword ptr [rsp + 0x28], xmm11
141243062 mov qword ptr [rsp + 0x20], rsi
141243067 mov byte ptr [rsp + 0x30], 0
14124306c mov rcx, rdi
14124306f mov r9, r14
141243072 call 0x141243740
141243077 mov rbp, qword ptr [rsp + 0x2e0]
14124307f mov rdi, qword ptr [rsp + 0x2c0]
141243087 mov r13, qword ptr [rsp + 0xd8]
14124308f jmp 0x1412426da
141243094 nop word ptr cs:[rax + rax]
1412430a0 test sil, sil
1412430a3 je 0x1412426da
1412430a9 mov rdx, qword ptr [rsp + 0x2f0]
1412430b1 cmp rbx, rdx
1412430b4 jb 0x14124265d
1412430ba jmp 0x1412436c3
1412430bf mov rax, qword ptr [rsp + 0xe0]
1412430c7 mov qword ptr [rsp + 0x70], rax
1412430cc mov qword ptr [rsp + 0x78], rbx
1412430d1 mov byte ptr [rsp + 0x80], 0
1412430d9 mov eax, dword ptr [rsp + 0x2f8]
1412430e0 mov dword ptr [rsp + 0x28], eax
1412430e4 mov rax, qword ptr [rsp + 0x2d0]
1412430ec mov qword ptr [rsp + 0x20], rax
1412430f1 lea rcx, [rsp + 0x1b0]
1412430f9 mov rdx, qword ptr [rsp + 0xd0]
141243101 mov r8, qword ptr [rsp + 0xf8]
141243109 lea r9, [rsp + 0x70]
14124310e call 0x141243c20
141243113 mov rax, qword ptr [rsp + 0x110]
14124311b mov sil, 1
14124311e cmp rax, qword ptr [rsp + 0x140]
141243126 jne 0x141242e24
14124312c jmp 0x141242f59
141243131 test sil, 1
141243135 je 0x14124339a
14124313b mov qword ptr [rsp + 0x168], rdx
141243143 cmp rcx, qword ptr [rsp + 0x2c8]
14124314b jae 0x141243698
141243151 lea rax, [rcx + rcx*2]
141243155 mov edx, dword ptr [rdi + rax*4 + 8]
141243159 mov dword ptr [rsp + 0xc0], edx
141243160 mov rax, qword ptr [rdi + rax*4]
141243164 mov qword ptr [rsp + 0xb8], rax
14124316c mov qword ptr [rsp + 0xb0], rcx
141243174 mov dword ptr [rsp + 0xa0], 0
14124317f mov dword ptr [rsp + 0xc4], 0
14124318a mov byte ptr [rsp + 0xc8], 0
141243192 test rcx, rcx
141243195 mov rax, rdi
141243198 je 0x14124338f
14124319e mov qword ptr [rsp + 0xd8], r13
1412431a6 mov rdi, rax
1412431a9 lea r14, [rsp + 0xb8]
1412431b1 lea r15, [rsp + 0x7c]
1412431b6 mov qword ptr [rsp + 0x160], rcx
1412431be lea rsi, [rcx - 1]
1412431c2 lea r13, [rsp + 0x70]
1412431c7 lea rbx, [rsp + 0xa0]
1412431cf nop 
1412431d0 mov qword ptr [rsp + 0x30], rbp
1412431d5 mov rax, qword ptr [rsp + 0x2d8]
1412431dd mov qword ptr [rsp + 0x28], rax
1412431e2 mov rax, qword ptr [rsp + 0x2c8]
1412431ea mov qword ptr [rsp + 0x20], rax
1412431ef mov rcx, r13
1412431f2 mov rdx, rbx
1412431f5 mov r8, rsi
1412431f8 mov r9, rdi
1412431fb call 0x141fe09c0
141243200 movzx eax, byte ptr [rsp + 0x98]
141243208 cmp al, 2
14124320a je 0x14124327b
14124320c cmp byte ptr [rsp + 0xc8], 1
141243214 je 0x141243643
14124321a mov rcx, qword ptr [rsp + 0x70]
14124321f movss xmm0, dword ptr [rsp + 0x78]
141243225 mov qword ptr [rsp + 0xb0], rcx
14124322d mov dword ptr [rsp + 0xa0], 1
141243238 mov ecx, dword ptr [r15 + 8]
14124323c mov dword ptr [r14 - 0xc], ecx
141243240 mov rcx, qword ptr [r15]
141243243 mov qword ptr [r14 - 0x14], rcx
141243247 mov ecx, dword ptr [r15 + 0x18]
14124324b mov dword ptr [r14 + 8], ecx
14124324f mov rcx, qword ptr [r15 + 0x10]
141243253 mov qword ptr [r14], rcx
141243256 addss xmm0, dword ptr [rsp + 0xc4]
14124325f movss dword ptr [rsp + 0xc4], xmm0
141243268 mov byte ptr [rsp + 0xc8], al
14124326f add rsi, -1
141243273 jb 0x1412431d0
141243279 jmp 0x141243284
14124327b movss xmm0, dword ptr [rsp + 0xc4]
141243284 ucomiss xmm0, dword ptr [rip + 0x16e2679]
14124328b mov rsi, qword ptr [rsp + 0x2f0]
141243293 mov rax, rdi
141243296 mov r13, qword ptr [rsp + 0xd8]
14124329e mov r14, qword ptr [rsp + 0x160]
1412432a6 jb 0x14124338f
1412432ac mov rax, qword ptr [rsp + 0xb0]
1412432b4 mov qword ptr [rsp + 0x70], rax
1412432b9 mov qword ptr [rsp + 0x78], r14
1412432be mov byte ptr [rsp + 0x80], 0
1412432c6 mov ebx, dword ptr [rsp + 0x2f8]
1412432cd mov dword ptr [rsp + 0x28], ebx
1412432d1 mov r15, qword ptr [rsp + 0x2d0]
1412432d9 mov qword ptr [rsp + 0x20], r15
1412432de lea rcx, [rsp + 0x1b0]
1412432e6 lea r9, [rsp + 0x70]
1412432eb mov r13, qword ptr [rsp + 0xd0]
1412432f3 mov rdx, r13
1412432f6 mov r8, qword ptr [rsp + 0xf8]
1412432fe call 0x141243c20
141243303 cmp r14, rsi
141243306 jae 0x14124370c
14124330c mov rax, qword ptr [rsp + 0x2e8]
141243314 mov edx, dword ptr [rax + r14*8]
141243318 movss xmm2, dword ptr [rax + r14*8 + 4]
14124331f movss xmm0, dword ptr [rsp + 0xc4]
141243328 mulss xmm0, dword ptr [rip + 0x184a290]
141243330 movups xmm1, xmmword ptr [r15]
141243334 mov dword ptr [rsp + 0x60], r14d
141243339 mov dword ptr [rsp + 0x58], ebx
14124333d movups xmmword ptr [rsp + 0x48], xmm1
141243342 mov rax, qword ptr [rsp + 0x2b8]
14124334a mov qword ptr [rsp + 0x40], rax
14124334f mov rax, qword ptr [rsp + 0x2b0]
141243357 mov qword ptr [rsp + 0x38], rax
14124335c movss dword ptr [rsp + 0x28], xmm0
141243362 mov rax, qword ptr [rsp + 0xf0]
14124336a mov qword ptr [rsp + 0x20], rax
14124336f mov byte ptr [rsp + 0x30], 0
141243374 mov rcx, r13
141243377 mov r9, qword ptr [rsp + 0xe8]
14124337f call 0x141243740
141243384 mov rax, rdi
141243387 mov r13, qword ptr [rsp + 0xd8]
14124338f mov rdx, qword ptr [rsp + 0x168]
141243397 mov rdi, rax
14124339a cmp rdx, 1
14124339e jne 0x1412435fb
1412433a4 cmp r13, qword ptr [rsp + 0x2c8]
1412433ac jae 0x1412436ac
1412433b2 lea rax, [r13*2]
1412433ba add rax, r13
1412433bd mov ecx, dword ptr [rdi + rax*4 + 8]
1412433c1 mov dword ptr [rsp + 0xc0], ecx
1412433c8 mov rax, qword ptr [rdi + rax*4]
1412433cc mov qword ptr [rsp + 0xb8], rax
1412433d4 mov qword ptr [rsp + 0xb0], r13
1412433dc mov dword ptr [rsp + 0xa0], 0
1412433e7 mov dword ptr [rsp + 0xc4], 0
1412433f2 mov byte ptr [rsp + 0xc8], 0
1412433fa lea rsi, [r13 + 1]
1412433fe cmp rsi, r12
141243401 jae 0x1412435fb
141243407 mov qword ptr [rsp + 0xd8], r13
14124340f mov r13, rdi
141243412 lea r14, [rsp + 0xb8]
14124341a lea r15, [rsp + 0x7c]
14124341f lea rdi, [rsp + 0x70]
141243424 lea rbx, [rsp + 0xa0]
14124342c nop dword ptr [rax]
141243430 mov qword ptr [rsp + 0x30], rbp
141243435 mov rax, qword ptr [rsp + 0x2d8]
14124343d mov qword ptr [rsp + 0x28], rax
141243442 mov rax, qword ptr [rsp + 0x2c8]
14124344a mov qword ptr [rsp + 0x20], rax
14124344f mov rcx, rdi
141243452 mov rdx, rbx
141243455 mov r8, rsi
141243458 mov r9, r13
14124345b call 0x141fe09c0
141243460 movzx eax, byte ptr [rsp + 0x98]
141243468 cmp al, 2
14124346a je 0x1412434fd
141243470 cmp byte ptr [rsp + 0xc8], 1
141243478 je 0x141243643
14124347e mov rcx, qword ptr [rsp + 0x70]
141243483 movss xmm0, dword ptr [rsp + 0x78]
141243489 mov qword ptr [rsp + 0xb0], rcx
141243491 mov dword ptr [rsp + 0xa0], 1
14124349c mov ecx, dword ptr [r15 + 8]
1412434a0 mov dword ptr [r14 - 0xc], ecx
1412434a4 mov rcx, qword ptr [r15]
1412434a7 mov qword ptr [r14 - 0x14], rcx
1412434ab mov ecx, dword ptr [r15 + 0x18]
1412434af mov dword ptr [r14 + 8], ecx
1412434b3 mov rcx, qword ptr [r15 + 0x10]
1412434b7 mov qword ptr [r14], rcx
1412434ba addss xmm0, dword ptr [rsp + 0xc4]
1412434c3 movss dword ptr [rsp + 0xc4], xmm0
1412434cc mov byte ptr [rsp + 0xc8], al
1412434d3 inc rsi
1412434d6 cmp r12, rsi
1412434d9 jne 0x141243430
1412434df ucomiss xmm0, dword ptr [rip + 0x16e241e]
1412434e6 mov rsi, qword ptr [rsp + 0x2f0]
1412434ee mov r15, qword ptr [rsp + 0xd8]
1412434f6 jae 0x141243523
1412434f8 jmp 0x1412435fb
1412434fd movss xmm0, dword ptr [rsp + 0xc4]
141243506 ucomiss xmm0, dword ptr [rip + 0x16e23f7]
14124350d mov rsi, qword ptr [rsp + 0x2f0]
141243515 mov r15, qword ptr [rsp + 0xd8]
14124351d jb 0x1412435fb
141243523 mov rax, qword ptr [rsp + 0xb0]
14124352b mov qword ptr [rsp + 0x70], r15
141243530 mov qword ptr [rsp + 0x78], rax
141243535 mov byte ptr [rsp + 0x80], 0
14124353d mov ebx, dword ptr [rsp + 0x2f8]
141243544 mov dword ptr [rsp + 0x28], ebx
141243548 mov r14, qword ptr [rsp + 0x2d0]
141243550 mov qword ptr [rsp + 0x20], r14
141243555 lea rcx, [rsp + 0x1b0]
14124355d lea r9, [rsp + 0x70]
141243562 mov rdi, qword ptr [rsp + 0xd0]
14124356a mov rdx, rdi
14124356d mov r8, qword ptr [rsp + 0xf8]
141243575 call 0x141243c20
14124357a cmp r15, rsi
14124357d jae 0x14124371e
141243583 mov rax, qword ptr [rsp + 0x2e8]
14124358b mov edx, dword ptr [rax + r15*8]
14124358f movss xmm2, dword ptr [rax + r15*8 + 4]
141243596 movss xmm0, dword ptr [rsp + 0xc4]
14124359f mulss xmm0, dword ptr [rip + 0x184a019]
1412435a7 movups xmm1, xmmword ptr [r14]
1412435ab mov dword ptr [rsp + 0x60], r15d
1412435b0 mov dword ptr [rsp + 0x58], ebx
1412435b4 movups xmmword ptr [rsp + 0x48], xmm1
1412435b9 mov rax, qword ptr [rsp + 0x2b8]
1412435c1 mov qword ptr [rsp + 0x40], rax
1412435c6 mov rax, qword ptr [rsp + 0x2b0]
1412435ce mov qword ptr [rsp + 0x38], rax
1412435d3 movss dword ptr [rsp + 0x28], xmm0
1412435d9 mov rax, qword ptr [rsp + 0xf0]
1412435e1 mov qword ptr [rsp + 0x20], rax
1412435e6 mov byte ptr [rsp + 0x30], 1
1412435eb mov rcx, rdi
1412435ee mov r9, qword ptr [rsp + 0xe8]
1412435f6 call 0x141243740
1412435fb movaps xmm6, xmmword ptr [rsp + 0x1e0]
141243603 movaps xmm7, xmmword ptr [rsp + 0x1f0]
14124360b movaps xmm8, xmmword ptr [rsp + 0x200]
141243614 movaps xmm9, xmmword ptr [rsp + 0x210]
14124361d movaps xmm10, xmmword ptr [rsp + 0x220]
141243626 movaps xmm11, xmmword ptr [rsp + 0x230]
14124362f add rsp, 0x248
141243636 pop rbx
141243637 pop rbp
141243638 pop rdi
141243639 pop rsi
14124363a pop r12
14124363c pop r13
14124363e pop r14
141243640 pop r15
141243642 ret 
141243643 lea rcx, [rip + 0x1d87d26]
14124364a lea r8, [rip + 0x1d87d47]
141243651 mov edx, 0x25
141243656 call 0x1428d9430
14124365b cmp r9, rbp
14124365e cmovbe r9, rbp
141243662 lea r8, [rip + 0x19dc327]
141243669 mov rcx, r9
14124366c mov rdx, rbp
14124366f call 0x1428d9518
141243674 lea r8, [rip + 0x1d87cad]
14124367b mov rcx, r14
14124367e mov rdx, r13
141243681 call 0x1428d9518
141243686 lea r8, [rip + 0x1d87c9b]
14124368d mov rcx, rbx
141243690 mov rdx, r13
141243693 call 0x1428d9518
141243698 lea r8, [rip + 0x1d87c89]
14124369f mov rdx, qword ptr [rsp + 0x2c8]
1412436a7 call 0x1428d9518
1412436ac lea r8, [rip + 0x1d87c75]
1412436b3 mov rcx, r13
1412436b6 mov rdx, qword ptr [rsp + 0x2c8]
1412436be call 0x1428d9518
1412436c3 lea r8, [rip + 0x19dc24e]
1412436ca mov rcx, rbx
1412436cd call 0x1428d9518
1412436d2 lea r8, [rip + 0x19dc227]
1412436d9 mov rcx, r9
1412436dc mov rdx, qword ptr [rsp + 0x2f0]
1412436e4 call 0x1428d9518
1412436e9 lea r8, [rip + 0x19dc240]
1412436f0 mov rdx, qword ptr [rsp + 0x2f0]
1412436f8 call 0x1428d9518
1412436fd lea r8, [rip + 0x19dc244]
141243704 mov rcx, rbx
141243707 call 0x1428d9518
14124370c lea r8, [rip + 0x19dc24d]
141243713 mov rcx, r14
141243716 mov rdx, rsi
141243719 call 0x1428d9518
14124371e lea r8, [rip + 0x19dc253]
141243725 mov rcx, r15
141243728 mov rdx, rsi
14124372b call 0x1428d9518
141243730 int3 
