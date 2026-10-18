0x1404d9580 push rbp
0x1404d9581 push r15
0x1404d9583 push r14
0x1404d9585 push r13
0x1404d9587 push r12
0x1404d9589 push rsi
0x1404d958a push rdi
0x1404d958b push rbx
0x1404d958c mov eax, 0x15d8
0x1404d9591 call 0x14289aa40
0x1404d9596 sub rsp, rax
0x1404d9599 lea rbp, [rsp + 0x80]
0x1404d95a1 movdqa xmmword ptr [rbp + 0x1540], xmm15
0x1404d95aa movaps xmmword ptr [rbp + 0x1530], xmm14
0x1404d95b2 movdqa xmmword ptr [rbp + 0x1520], xmm13
0x1404d95bb movaps xmmword ptr [rbp + 0x1510], xmm12
0x1404d95c3 movaps xmmword ptr [rbp + 0x1500], xmm11
0x1404d95cb movapd xmmword ptr [rbp + 0x14f0], xmm10
0x1404d95d4 movdqa xmmword ptr [rbp + 0x14e0], xmm9
0x1404d95dd movaps xmmword ptr [rbp + 0x14d0], xmm8
0x1404d95e5 movdqa xmmword ptr [rbp + 0x14c0], xmm7
0x1404d95ed movapd xmmword ptr [rbp + 0x14b0], xmm6
0x1404d95f5 mov qword ptr [rbp + 0x14a8], 0xfffffffffffffffe
0x1404d9600 mov qword ptr [rbp + 0x13b0], 0
0x1404d960b mov qword ptr [rbp + 0x13b8], 8
0x1404d9616 mov qword ptr [rbp + 0x13c0], 0
0x1404d9621 mov qword ptr [rbp + 0x1478], rcx
0x1404d9628 call 0x1404e2470
0x1404d962d mov rax, qword ptr [rbp + 0x1478]
0x1404d9634 lea rcx, [rax + 0x30]
0x1404d9638 mov qword ptr [rbp + 0x1450], rcx
0x1404d963f lea rcx, [rax + 0x68]
0x1404d9643 mov qword ptr [rbp + 0x1488], rcx
0x1404d964a lea rcx, [rax + 0xa88]
0x1404d9651 mov qword ptr [rbp + 0x1410], rcx
0x1404d9658 lea rcx, [rax + 0x20]
0x1404d965c mov qword ptr [rbp + 0x1418], rcx
0x1404d9663 lea rcx, [rax + 0x1115]
0x1404d966a mov qword ptr [rbp + 0x13a8], rcx
0x1404d9671 movapd xmm10, xmmword ptr [rip + 0x250ff36]
0x1404d967a xorps xmm11, xmm11
0x1404d967e movss xmm12, dword ptr [rip + 0x24c7149]
0x1404d9687 movdqa xmm13, xmmword ptr [rip + 0x244c260]
0x1404d9690 movdqa xmm9, xmmword ptr [rip + 0x2487ed7]
0x1404d9699 lea rcx, [rax + 0x10c8]
0x1404d96a0 mov qword ptr [rbp + 0x1420], rcx
0x1404d96a7 lea rcx, [rax + 0xc0]
0x1404d96ae mov qword ptr [rbp + 0x13d8], rcx
0x1404d96b5 lea rcx, [rax + 0x6a0]
0x1404d96bc mov qword ptr [rbp + 0x1350], rcx
0x1404d96c3 lea rcx, [rax + 0x890]
0x1404d96ca mov qword ptr [rbp + 0x13c8], rcx
0x1404d96d1 lea rcx, [rax + 0x8e0]
0x1404d96d8 mov qword ptr [rbp + 0x1370], rcx
0x1404d96df lea rcx, [rax + 0x980]
0x1404d96e6 mov qword ptr [rbp + 0x13d0], rcx
0x1404d96ed lea rcx, [rax + 0x9d0]
0x1404d96f4 mov qword ptr [rbp + 0x1378], rcx
0x1404d96fb lea rcx, [rax + 0x9e0]
0x1404d9702 mov qword ptr [rbp + 0x1390], rcx
0x1404d9709 lea rcx, [rax + 0x10f0]
0x1404d9710 mov qword ptr [rbp + 0x1388], rcx
0x1404d9717 lea rcx, [rax + 0x730]
0x1404d971e mov qword ptr [rbp + 0x1330], rcx
0x1404d9725 lea rcx, [rax + 0x688]
0x1404d972c mov qword ptr [rbp + 0x1338], rcx
0x1404d9733 lea rcx, [rax + 0x10b0]
0x1404d973a mov qword ptr [rbp + 0x1340], rcx
0x1404d9741 lea rcx, [rax + 0xa90]
0x1404d9748 mov qword ptr [rbp + 0x1348], rcx
0x1404d974f lea rcx, [rax + 0x8b0]
0x1404d9756 mov qword ptr [rbp + 0x1358], rcx
0x1404d975d lea rcx, [rax + 0x8c0]
0x1404d9764 mov qword ptr [rbp + 0x1398], rcx
0x1404d976b lea rcx, [rax + 0x9a0]
0x1404d9772 mov qword ptr [rbp + 0x1360], rcx
0x1404d9779 lea rcx, [rax + 0x9b0]
0x1404d9780 mov qword ptr [rbp + 0x13a0], rcx
0x1404d9787 add rax, 0xa40
0x1404d978d mov qword ptr [rbp + 0x1368], rax
0x1404d9794 mov qword ptr [rbp + 0x1380], 0
0x1404d979f pcmpeqd xmm7, xmm7
0x1404d97a3 jmp 0x1404d97cc
0x1404d97a5 nop word ptr cs:[rax + rax]
0x1404d97b0 mov r8d, 0xc8
0x1404d97b6 mov rcx, qword ptr [rbp + 0x1498]
0x1404d97bd mov rdx, rsi
0x1404d97c0 call 0x142923c00
0x1404d97c5 inc qword ptr [rbp + 0x1380]
0x1404d97cc mov rcx, qword ptr [rbp + 0x1488]
0x1404d97d3 movabs rdx, 0x98df08dcfa7ea5ba
0x1404d97dd movabs r8, 0x67b117a60e632bc3
0x1404d97e7 call 0x1400dd6f0
0x1404d97ec test rax, rax
0x1404d97ef mov r8, qword ptr [rbp + 0x1478]
0x1404d97f6 je 0x1404d9850
0x1404d97f8 cmp qword ptr [r8 + 0x248], rdx
0x1404d97ff jbe 0x1404d9850
0x1404d9801 mov rax, qword ptr [r8 + 0x240]
0x1404d9808 mov rcx, qword ptr [rax + rdx*8]
0x1404d980c test rcx, rcx
0x1404d980f je 0x1404d9850
0x1404d9811 mov rax, qword ptr [r8 + 0x210]
0x1404d9818 not rcx
0x1404d981b lea rcx, [rcx + rcx*2]
0x1404d981f shl rcx, 5
0x1404d9823 cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9829 je 0x1404d9850
0x1404d982b add rax, rcx
0x1404d982e mov rcx, qword ptr [rax + 0x10]
0x1404d9832 call 0x140b19d10
0x1404d9837 test al, al
0x1404d9839 mov r8, qword ptr [rbp + 0x1478]
0x1404d9840 je 0x1404d9a63
0x1404d9846 nop word ptr cs:[rax + rax]
0x1404d9850 mov rax, qword ptr [r8]
0x1404d9853 mov rdx, qword ptr [r8 + 8]
0x1404d9857 test rax, rax
0x1404d985a je 0x1404d9870
0x1404d985c cmp eax, 1
0x1404d985f jne 0x1404d9880
0x1404d9861 lea rcx, [rbp - 0x40]
0x1404d9865 call 0x140267660
0x1404d986a jmp 0x1404d9889
0x1404d986c nop dword ptr [rax]
0x1404d9870 lea rcx, [rbp - 0x40]
0x1404d9874 call 0x1404f2090
0x1404d9879 jmp 0x1404d9889
0x1404d987b nop dword ptr [rax + rax]
0x1404d9880 lea rcx, [rbp - 0x40]
0x1404d9884 call 0x1405205b0
0x1404d9889 xor eax, eax
0x1404d988b cmp rax, qword ptr [rbp - 0x40]
0x1404d988f jo 0x1404d9a63
0x1404d9895 mov r8d, 0x628
0x1404d989b lea rcx, [rbp + 0x600]
0x1404d98a2 lea rdx, [rbp - 0x40]
0x1404d98a6 call 0x142923c00
0x1404d98ab mov rax, qword ptr [rip + 0x3152af6]
0x1404d98b2 mov rax, qword ptr [rax]
0x1404d98b5 cmp rax, 3
0x1404d98b9 mov rsi, qword ptr [rbp + 0x1478]
0x1404d98c0 mov rdi, qword ptr [rbp + 0x1410]
0x1404d98c7 lea rbx, [rbp + 0xc30]
0x1404d98ce jb 0x1404d999b
0x1404d98d4 mov byte ptr [rbp + 0x1497], 1
0x1404d98db lea rcx, [rip + 0x251045e]
0x1404d98e2 call 0x14067dd80
0x1404d98e7 movupd xmm0, xmmword ptr [rax]
0x1404d98eb mov eax, dword ptr [rax + 0x10]
0x1404d98ee mov qword ptr [rbp + 0xc60], 3
0x1404d98f9 lea rdx, [rip + 0x25103e2]
0x1404d9900 mov qword ptr [rbp + 0xc68], rdx
0x1404d9907 mov qword ptr [rbp + 0xc70], 0x15
0x1404d9912 lea rcx, [rip + 0x2510417]
0x1404d9919 mov qword ptr [rbp + 0xc80], rcx
0x1404d9920 mov qword ptr [rbp + 0xc88], 1
0x1404d992b mov qword ptr [rbp + 0xc90], 8
0x1404d9936 mov qword ptr [rbp + 0xc30], 0
0x1404d9941 lea rcx, [rbp + 0xc98]
0x1404d9948 pxor xmm1, xmm1
0x1404d994c movdqu xmmword ptr [rcx], xmm1
0x1404d9950 mov qword ptr [rbp + 0xc38], rdx
0x1404d9957 mov qword ptr [rbp + 0xc40], 0x15
0x1404d9962 mov qword ptr [rbp + 0xc48], 0
0x1404d996d movupd xmmword ptr [rbp + 0xc50], xmm0
0x1404d9975 mov dword ptr [rbp + 0xc78], 1
0x1404d997f mov dword ptr [rbp + 0xc7c], eax
0x1404d9985 mov byte ptr [rbp + 0x1497], 1
0x1404d998c lea rcx, [rbp + 0x1477]
0x1404d9993 mov rdx, rbx
0x1404d9996 call 0x140d4a330
0x1404d999b mov r8d, 0x628
0x1404d99a1 mov rcx, rbx
0x1404d99a4 lea rdx, [rbp + 0x600]
0x1404d99ab call 0x142923c00
0x1404d99b0 xor eax, eax
0x1404d99b2 cmp rax, qword ptr [rdi]
0x1404d99b5 jo 0x1404d99c3
0x1404d99b7 mov rcx, qword ptr [rbp + 0x1410]
0x1404d99be call 0x1405e8b60
0x1404d99c3 mov r8d, 0x628
0x1404d99c9 mov rcx, qword ptr [rbp + 0x1410]
0x1404d99d0 mov rdx, rbx
0x1404d99d3 call 0x142923c00
0x1404d99d8 mov byte ptr [rbp + 0x1497], 0
0x1404d99df mov rcx, qword ptr [rbp + 0x1488]
0x1404d99e6 movabs rdx, 0x98df08dcfa7ea5ba
0x1404d99f0 movabs r8, 0x67b117a60e632bc3
0x1404d99fa call 0x1400dd6f0
0x1404d99ff test rax, rax
0x1404d9a02 je 0x1404df3c0
0x1404d9a08 cmp qword ptr [rsi + 0x248], rdx
0x1404d9a0f jbe 0x1404df3c0
0x1404d9a15 mov rax, qword ptr [rsi + 0x240]
0x1404d9a1c mov rcx, qword ptr [rax + rdx*8]
0x1404d9a20 test rcx, rcx
0x1404d9a23 je 0x1404df3c0
0x1404d9a29 mov rax, qword ptr [rsi + 0x210]
0x1404d9a30 not rcx
0x1404d9a33 lea rcx, [rcx + rcx*2]
0x1404d9a37 shl rcx, 5
0x1404d9a3b cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9a41 je 0x1404df3c0
0x1404d9a47 add rax, rcx
0x1404d9a4a mov rcx, qword ptr [rax + 0x10]
0x1404d9a4e mov edx, dword ptr [rsi + 0x5e0]
0x1404d9a54 mov dword ptr [rax + 0x5c], edx
0x1404d9a57 mov byte ptr [rbp + 0x1497], 0
0x1404d9a5e call 0x140b19d30
0x1404d9a63 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9a6a movabs rdx, 0x98df08dcfa7ea5ba
0x1404d9a74 movabs r8, 0x67b117a60e632bc3
0x1404d9a7e call 0x1400dd6f0
0x1404d9a83 test rax, rax
0x1404d9a86 mov r14, qword ptr [rbp + 0x1478]
0x1404d9a8d mov r8, qword ptr [rbp + 0x1410]
0x1404d9a94 lea r12, [rbp + 0xc30]
0x1404d9a9b je 0x1404d9b00
0x1404d9a9d cmp qword ptr [r14 + 0x248], rdx
0x1404d9aa4 jbe 0x1404d9b00
0x1404d9aa6 mov rax, qword ptr [r14 + 0x240]
0x1404d9aad mov rcx, qword ptr [rax + rdx*8]
0x1404d9ab1 test rcx, rcx
0x1404d9ab4 je 0x1404d9b00
0x1404d9ab6 mov rax, qword ptr [r14 + 0x210]
0x1404d9abd not rcx
0x1404d9ac0 lea rcx, [rcx + rcx*2]
0x1404d9ac4 shl rcx, 5
0x1404d9ac8 cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9ace je 0x1404d9b00
0x1404d9ad0 add rax, rcx
0x1404d9ad3 mov rcx, qword ptr [rax + 0x10]
0x1404d9ad7 call 0x140b19d20
0x1404d9adc test al, al
0x1404d9ade mov r14, qword ptr [rbp + 0x1478]
0x1404d9ae5 mov r8, qword ptr [rbp + 0x1410]
0x1404d9aec je 0x1404da381
0x1404d9af2 nop word ptr cs:[rax + rax]
0x1404d9b00 mov rax, qword ptr [r8]
0x1404d9b03 movabs rcx, 0x8000000000000000
0x1404d9b0d mov qword ptr [r8], rcx
0x1404d9b10 mov rcx, rax
0x1404d9b13 neg rcx
0x1404d9b16 jo 0x1404da381
0x1404d9b1c mov qword ptr [rbp + 0x600], rax
0x1404d9b23 mov r8d, 0x620
0x1404d9b29 lea rcx, [rbp + 0x608]
0x1404d9b30 mov rdx, qword ptr [rbp + 0x1348]
0x1404d9b37 call 0x142923c00
0x1404d9b3c mov rax, qword ptr [rip + 0x3152865]
0x1404d9b43 mov rax, qword ptr [rax]
0x1404d9b46 cmp rax, 3
0x1404d9b4a mov r15, qword ptr [rbp + 0x1450]
0x1404d9b51 jb 0x1404d9c1e
0x1404d9b57 mov byte ptr [rbp + 0x14a4], 1
0x1404d9b5e lea rcx, [rip + 0x251024b]
0x1404d9b65 call 0x14067dd80
0x1404d9b6a movupd xmm0, xmmword ptr [rax]
0x1404d9b6e mov eax, dword ptr [rax + 0x10]
0x1404d9b71 mov qword ptr [rbp + 0xc60], 3
0x1404d9b7c lea rdx, [rip + 0x251015f]
0x1404d9b83 mov qword ptr [rbp + 0xc68], rdx
0x1404d9b8a mov qword ptr [rbp + 0xc70], 0x15
0x1404d9b95 lea rcx, [rip + 0x2510204]
0x1404d9b9c mov qword ptr [rbp + 0xc80], rcx
0x1404d9ba3 mov qword ptr [rbp + 0xc88], 1
0x1404d9bae mov qword ptr [rbp + 0xc90], 8
0x1404d9bb9 mov qword ptr [rbp + 0xc30], 0
0x1404d9bc4 lea rcx, [rbp + 0xc98]
0x1404d9bcb pxor xmm1, xmm1
0x1404d9bcf movdqu xmmword ptr [rcx], xmm1
0x1404d9bd3 mov qword ptr [rbp + 0xc38], rdx
0x1404d9bda mov qword ptr [rbp + 0xc40], 0x15
0x1404d9be5 mov qword ptr [rbp + 0xc48], 0
0x1404d9bf0 movupd xmmword ptr [rbp + 0xc50], xmm0
0x1404d9bf8 mov dword ptr [rbp + 0xc78], 1
0x1404d9c02 mov dword ptr [rbp + 0xc7c], eax
0x1404d9c08 mov byte ptr [rbp + 0x14a4], 1
0x1404d9c0f lea rcx, [rbp + 0x1477]
0x1404d9c16 mov rdx, r12
0x1404d9c19 call 0x140d4a330
0x1404d9c1e mov byte ptr [rbp + 0x14a4], 1
0x1404d9c25 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9c2c movabs rdx, 0x3d7b56e787d47c40
0x1404d9c36 movabs r8, 0x67b0b10a8d43bfc7
0x1404d9c40 call 0x1400dd6f0
0x1404d9c45 test rax, rax
0x1404d9c48 je 0x1404df206
0x1404d9c4e cmp qword ptr [r14 + 0x248], rdx
0x1404d9c55 jbe 0x1404df206
0x1404d9c5b mov rax, qword ptr [r14 + 0x240]
0x1404d9c62 mov rax, qword ptr [rax + rdx*8]
0x1404d9c66 test rax, rax
0x1404d9c69 je 0x1404df206
0x1404d9c6f mov rcx, qword ptr [r14 + 0x210]
0x1404d9c76 not rax
0x1404d9c79 lea rax, [rax + rax*2]
0x1404d9c7d shl rax, 5
0x1404d9c81 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9c87 je 0x1404df206
0x1404d9c8d add rcx, rax
0x1404d9c90 mov byte ptr [rbp + 0x14a4], 1
0x1404d9c97 xor edx, edx
0x1404d9c99 call 0x140cff710
0x1404d9c9e mov rcx, qword ptr [rax]
0x1404d9ca1 mov rdx, rcx
0x1404d9ca4 neg rdx
0x1404d9ca7 jo 0x1404df206
0x1404d9cad add rax, 8
0x1404d9cb1 mov qword ptr [rbp + 0xc30], rcx
0x1404d9cb8 mov r8d, 0x120
0x1404d9cbe lea rcx, [rbp + 0xc38]
0x1404d9cc5 mov rdx, rax
0x1404d9cc8 call 0x142923c00
0x1404d9ccd mov byte ptr [rbp + 0x14a4], 1
0x1404d9cd4 lea rcx, [rbp + 0x600]
0x1404d9cdb mov rdx, r12
0x1404d9cde call 0x140328d80
0x1404d9ce3 mov byte ptr [rbp + 0x14a4], 1
0x1404d9cea mov rcx, qword ptr [rbp + 0x1488]
0x1404d9cf1 movabs rdx, 0x2b39980e7de6bf8a
0x1404d9cfb movabs r8, 0x52d9aabfd20d56c5
0x1404d9d05 call 0x1400dd6f0
0x1404d9d0a test rax, rax
0x1404d9d0d je 0x1404df1ee
0x1404d9d13 cmp qword ptr [r14 + 0x248], rdx
0x1404d9d1a jbe 0x1404df1ee
0x1404d9d20 mov rax, qword ptr [r14 + 0x240]
0x1404d9d27 mov rax, qword ptr [rax + rdx*8]
0x1404d9d2b test rax, rax
0x1404d9d2e je 0x1404df1ee
0x1404d9d34 mov rcx, qword ptr [r14 + 0x210]
0x1404d9d3b not rax
0x1404d9d3e lea rax, [rax + rax*2]
0x1404d9d42 shl rax, 5
0x1404d9d46 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9d4c je 0x1404df1ee
0x1404d9d52 add rcx, rax
0x1404d9d55 mov byte ptr [rbp + 0x14a4], 1
0x1404d9d5c xor edx, edx
0x1404d9d5e call 0x140cff710
0x1404d9d63 mov ecx, dword ptr [rax + 0x18]
0x1404d9d66 cmp ecx, 0x3b9aca00
0x1404d9d6c je 0x1404df1ee
0x1404d9d72 mov edx, dword ptr [rax + 0x1c]
0x1404d9d75 mov r8, qword ptr [rax + 0x10]
0x1404d9d79 mov qword ptr [rbp + 0xc40], r8
0x1404d9d80 movupd xmm0, xmmword ptr [rax]
0x1404d9d84 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404d9d8c mov dword ptr [rbp + 0xc48], ecx
0x1404d9d92 mov dword ptr [rbp + 0xc4c], edx
0x1404d9d98 mov byte ptr [rbp + 0x14a4], 1
0x1404d9d9f lea rcx, [rbp + 0x600]
0x1404d9da6 mov rdx, r12
0x1404d9da9 call 0x140329350
0x1404d9dae mov byte ptr [rbp + 0x14a4], 1
0x1404d9db5 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9dbc movabs rdx, 0x864ddca98c2312a6
0x1404d9dc6 movabs r8, 0x27b0e9369e9a484e
0x1404d9dd0 call 0x1400dd6f0
0x1404d9dd5 test rax, rax
0x1404d9dd8 je 0x1404df451
0x1404d9dde cmp qword ptr [r14 + 0x248], rdx
0x1404d9de5 jbe 0x1404df451
0x1404d9deb mov rax, qword ptr [r14 + 0x240]
0x1404d9df2 mov rax, qword ptr [rax + rdx*8]
0x1404d9df6 test rax, rax
0x1404d9df9 je 0x1404df451
0x1404d9dff mov rcx, qword ptr [r14 + 0x210]
0x1404d9e06 not rax
0x1404d9e09 lea rax, [rax + rax*2]
0x1404d9e0d shl rax, 5
0x1404d9e11 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9e17 je 0x1404df451
0x1404d9e1d add rcx, rax
0x1404d9e20 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e27 xor edx, edx
0x1404d9e29 call 0x140cff710
0x1404d9e2e mov edx, dword ptr [rax]
0x1404d9e30 mov r8d, dword ptr [rax + 4]
0x1404d9e34 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e3b lea rcx, [rbp + 0x600]
0x1404d9e42 call 0x140329e70
0x1404d9e47 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e4e mov rcx, qword ptr [rbp + 0x1488]
0x1404d9e55 movabs rdx, 0xe5e047cb9d6d98d7
0x1404d9e5f movabs r8, 0x9c5fd35c0a788a5a
0x1404d9e69 call 0x1400dd6f0
0x1404d9e6e test rax, rax
0x1404d9e71 je 0x1404df21e
0x1404d9e77 cmp qword ptr [r14 + 0x248], rdx
0x1404d9e7e jbe 0x1404df21e
0x1404d9e84 mov rax, qword ptr [r14 + 0x240]
0x1404d9e8b mov rax, qword ptr [rax + rdx*8]
0x1404d9e8f test rax, rax
0x1404d9e92 je 0x1404df21e
0x1404d9e98 mov rcx, qword ptr [r14 + 0x210]
0x1404d9e9f not rax
0x1404d9ea2 lea rax, [rax + rax*2]
0x1404d9ea6 shl rax, 5
0x1404d9eaa cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9eb0 je 0x1404df21e
0x1404d9eb6 add rcx, rax
0x1404d9eb9 mov byte ptr [rbp + 0x14a4], 1
0x1404d9ec0 xor edx, edx
0x1404d9ec2 call 0x140cff710
0x1404d9ec7 mov rcx, qword ptr [rax]
0x1404d9eca mov rdx, rcx
0x1404d9ecd neg rdx
0x1404d9ed0 jo 0x1404df21e
0x1404d9ed6 mov qword ptr [rbp + 0xc30], rcx
0x1404d9edd movupd xmm0, xmmword ptr [rax + 8]
0x1404d9ee2 lea rax, [rbp + 0xc38]
0x1404d9ee9 movupd xmmword ptr [rax], xmm0
0x1404d9eed mov byte ptr [rbp + 0x14a4], 1
0x1404d9ef4 lea rcx, [rbp + 0x600]
0x1404d9efb mov rdx, r12
0x1404d9efe call 0x14032a9f0
0x1404d9f03 mov byte ptr [rbp + 0x14a4], 1
0x1404d9f0a mov rcx, qword ptr [rbp + 0x1488]
0x1404d9f11 movabs rdx, 0xa4c02661e67b048b
0x1404d9f1b movabs r8, 0xeae761e6aa2a67ec
0x1404d9f25 call 0x1400dd6f0
0x1404d9f2a test rax, rax
0x1404d9f2d je 0x1404df236
0x1404d9f33 cmp qword ptr [r14 + 0x248], rdx
0x1404d9f3a jbe 0x1404df236
0x1404d9f40 mov rax, qword ptr [r14 + 0x240]
0x1404d9f47 mov rax, qword ptr [rax + rdx*8]
0x1404d9f4b test rax, rax
0x1404d9f4e je 0x1404df236
0x1404d9f54 mov rcx, qword ptr [r14 + 0x210]
0x1404d9f5b not rax
0x1404d9f5e lea rax, [rax + rax*2]
0x1404d9f62 shl rax, 5
0x1404d9f66 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9f6c je 0x1404df236
0x1404d9f72 add rcx, rax
0x1404d9f75 mov byte ptr [rbp + 0x14a4], 1
0x1404d9f7c xor edx, edx
0x1404d9f7e call 0x140cff710
0x1404d9f83 movzx ecx, byte ptr [rax + 0x20]
0x1404d9f87 cmp cl, 2
0x1404d9f8a je 0x1404df236
0x1404d9f90 movupd xmm0, xmmword ptr [rax]
0x1404d9f94 movdqu xmm1, xmmword ptr [rax + 0x10]
0x1404d9f99 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404d9fa1 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404d9fa9 mov byte ptr [rbp + 0xc50], cl
0x1404d9faf movzx ecx, byte ptr [rax + 0x23]
0x1404d9fb3 lea rdx, [rbp + 0xc38]
0x1404d9fba mov byte ptr [rdx + 0x1b], cl
0x1404d9fbd movzx eax, word ptr [rax + 0x21]
0x1404d9fc1 mov word ptr [rdx + 0x19], ax
0x1404d9fc5 mov byte ptr [rbp + 0x14a4], 1
0x1404d9fcc lea rcx, [rbp + 0x600]
0x1404d9fd3 mov rdx, r12
0x1404d9fd6 call 0x140328840
0x1404d9fdb mov byte ptr [rbp + 0x14a4], 1
0x1404d9fe2 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9fe9 movabs rdx, 0x3335386a5fbcee80
0x1404d9ff3 movabs r8, 0x8f2a59f31a65ec60
0x1404d9ffd call 0x1400dd6f0
0x1404da002 test rax, rax
0x1404da005 je 0x1404df24e
0x1404da00b cmp qword ptr [r14 + 0x248], rdx
0x1404da012 jbe 0x1404df24e
0x1404da018 mov rax, qword ptr [r14 + 0x240]
0x1404da01f mov rax, qword ptr [rax + rdx*8]
0x1404da023 test rax, rax
0x1404da026 je 0x1404df24e
0x1404da02c mov rcx, qword ptr [r14 + 0x210]
0x1404da033 not rax
0x1404da036 lea rax, [rax + rax*2]
0x1404da03a shl rax, 5
0x1404da03e cmp qword ptr [rcx + rax + 0x20], 0
0x1404da044 je 0x1404df24e
0x1404da04a add rcx, rax
0x1404da04d mov byte ptr [rbp + 0x14a4], 1
0x1404da054 xor edx, edx
0x1404da056 call 0x140cff710
0x1404da05b mov rcx, qword ptr [rax]
0x1404da05e cmp rcx, 4
0x1404da062 je 0x1404df24e
0x1404da068 mov qword ptr [rbp + 0xc30], rcx
0x1404da06f mov rcx, qword ptr [rax + 0x28]
0x1404da073 lea rdx, [rbp + 0xc38]
0x1404da07a mov qword ptr [rdx + 0x20], rcx
0x1404da07e movupd xmm0, xmmword ptr [rax + 8]
0x1404da083 movdqu xmm1, xmmword ptr [rax + 0x18]
0x1404da088 movdqu xmmword ptr [rdx + 0x10], xmm1
0x1404da08d movupd xmmword ptr [rdx], xmm0
0x1404da091 mov byte ptr [rbp + 0x14a4], 1
0x1404da098 lea rcx, [rbp + 0x600]
0x1404da09f mov rdx, r12
0x1404da0a2 call 0x1403282f0
0x1404da0a7 mov byte ptr [rbp + 0x14a4], 1
0x1404da0ae lea rcx, [rbp + 0x638]
0x1404da0b5 movabs rdx, 0x76eef710ca5ee797
0x1404da0bf movabs r8, 0x804a5169b0f39613
0x1404da0c9 call 0x1400dd6f0
0x1404da0ce test rax, rax
0x1404da0d1 je 0x1404df469
0x1404da0d7 cmp qword ptr [rbp + 0x818], rdx
0x1404da0de jbe 0x1404df469
0x1404da0e4 mov rax, qword ptr [rbp + 0x810]
0x1404da0eb mov rcx, qword ptr [rax + rdx*8]
0x1404da0ef test rcx, rcx
0x1404da0f2 je 0x1404df469
0x1404da0f8 mov rax, qword ptr [rbp + 0x7e0]
0x1404da0ff not rcx
0x1404da102 lea rcx, [rcx + rcx*2]
0x1404da106 shl rcx, 5
0x1404da10a cmp qword ptr [rax + rcx + 0x20], 0
0x1404da110 je 0x1404df469
0x1404da116 add rax, rcx
0x1404da119 mov rsi, qword ptr [rax + 0x10]
0x1404da11d mov ecx, dword ptr [rbp + 0xbb0]
0x1404da123 mov dword ptr [rax + 0x5c], ecx
0x1404da126 mov rdi, qword ptr [rsi + 0x30]
0x1404da12a mov rbx, qword ptr [rsi + 0x40]
0x1404da12e cmp rdi, qword ptr [rsi + 0x20]
0x1404da132 jne 0x1404da14b
0x1404da134 lea rcx, [rsi + 0x20]
0x1404da138 mov byte ptr [rbp + 0x14a4], 1
0x1404da13f lea rdx, [rip + 0x2518fe2]
0x1404da146 call 0x140562870
0x1404da14b mov rax, qword ptr [rsi + 0x28]
0x1404da14f mov qword ptr [rax + rdi*8], rbx
0x1404da153 inc rdi
0x1404da156 mov qword ptr [rsi + 0x30], rdi
0x1404da15a inc qword ptr [rsi + 0x40]
0x1404da15e mov byte ptr [rbp + 0x14a4], 1
0x1404da165 mov rcx, qword ptr [rbp + 0x1488]
0x1404da16c movabs rdx, 0xa284155e68bfeb3f
0x1404da176 movabs r8, 0x84a2b3441bc64a3a
0x1404da180 call 0x1400dd6f0
0x1404da185 test rax, rax
0x1404da188 je 0x1404df266
0x1404da18e cmp qword ptr [r14 + 0x248], rdx
0x1404da195 jbe 0x1404df266
0x1404da19b mov rax, qword ptr [r14 + 0x240]
0x1404da1a2 mov rax, qword ptr [rax + rdx*8]
0x1404da1a6 test rax, rax
0x1404da1a9 je 0x1404df266
0x1404da1af mov rcx, qword ptr [r14 + 0x210]
0x1404da1b6 not rax
0x1404da1b9 lea rax, [rax + rax*2]
0x1404da1bd shl rax, 5
0x1404da1c1 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da1c7 je 0x1404df266
0x1404da1cd add rcx, rax
0x1404da1d0 mov byte ptr [rbp + 0x14a4], 1
0x1404da1d7 xor edx, edx
0x1404da1d9 call 0x140cff710
0x1404da1de mov ecx, dword ptr [rax + 0x48]
0x1404da1e1 cmp ecx, 3
0x1404da1e4 je 0x1404df266
0x1404da1ea mov rdx, qword ptr [rax + 0x40]
0x1404da1ee mov qword ptr [rbp + 0xc70], rdx
0x1404da1f5 movupd xmm0, xmmword ptr [rax]
0x1404da1f9 movdqu xmm1, xmmword ptr [rax + 0x10]
0x1404da1fe movdqu xmm2, xmmword ptr [rax + 0x20]
0x1404da203 movdqu xmm3, xmmword ptr [rax + 0x30]
0x1404da208 movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404da210 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404da218 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404da220 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404da228 mov dword ptr [rbp + 0xc78], ecx
0x1404da22e mov ecx, dword ptr [rax + 0x54]
0x1404da231 lea rdx, [rbp + 0xc38]
0x1404da238 mov dword ptr [rdx + 0x4c], ecx
0x1404da23b mov rax, qword ptr [rax + 0x4c]
0x1404da23f mov qword ptr [rdx + 0x44], rax
0x1404da243 mov byte ptr [rbp + 0x14a4], 1
0x1404da24a lea rcx, [rbp - 0x40]
0x1404da24e mov rdx, r12
0x1404da251 call 0x1408a81e0
0x1404da256 mov byte ptr [rbp + 0x14a4], 1
0x1404da25d lea rcx, [rbp + 0x600]
0x1404da264 lea rdx, [rbp - 0x40]
0x1404da268 call 0x14032a3c0
0x1404da26d mov byte ptr [rbp + 0x14a4], 1
0x1404da274 lea rcx, [rbp + 0x638]
0x1404da27b movabs rdx, 0xaed7b5777ef68ba3
0x1404da285 movabs r8, 0x5834a199b0692b72
0x1404da28f call 0x1400dd6f0
0x1404da294 test rax, rax
0x1404da297 je 0x1404df481
0x1404da29d cmp qword ptr [rbp + 0x818], rdx
0x1404da2a4 jbe 0x1404df481
0x1404da2aa mov rax, qword ptr [rbp + 0x810]
0x1404da2b1 mov rax, qword ptr [rax + rdx*8]
0x1404da2b5 test rax, rax
0x1404da2b8 je 0x1404df481
0x1404da2be mov rcx, qword ptr [rbp + 0x7e0]
0x1404da2c5 not rax
0x1404da2c8 lea rax, [rax + rax*2]
0x1404da2cc shl rax, 5
0x1404da2d0 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da2d6 je 0x1404df481
0x1404da2dc add rcx, rax
0x1404da2df mov byte ptr [rbp + 0x14a4], 1
0x1404da2e6 xor edx, edx
0x1404da2e8 call 0x140cff710
0x1404da2ed mov rcx, qword ptr [rax]
0x1404da2f0 mov byte ptr [rbp + 0x14a4], 1
0x1404da2f7 call 0x14067dd80
0x1404da2fc mov byte ptr [rbp + 0x14a4], 1
0x1404da303 lea rcx, [rbp + 0x600]
0x1404da30a mov rdx, rax
0x1404da30d call 0x140329890
0x1404da312 mov r8d, 0x628
0x1404da318 mov rcx, r12
0x1404da31b lea rdx, [rbp + 0x600]
0x1404da322 call 0x142923c00
0x1404da327 mov rcx, r15
0x1404da32a call 0x1405e8b60
0x1404da32f mov r8d, 0x628
0x1404da335 mov rcx, qword ptr [rbp + 0x1450]
0x1404da33c mov rdx, r12
0x1404da33f call 0x142923c00
0x1404da344 jmp 0x1404da381
0x1404da346 nop word ptr cs:[rax + rax]
0x1404da350 mov rax, rsi
0x1404da353 shl rax, 7
0x1404da357 lea rcx, [rax + rsi*8]
0x1404da35b add rcx, qword ptr [rbp + 0x13b8]
0x1404da362 mov r8d, 0x88
0x1404da368 mov rdx, rdi
0x1404da36b call 0x142923c10
0x1404da370 inc rsi
0x1404da373 mov qword ptr [rbp + 0x13c0], rsi
0x1404da37a mov r14, qword ptr [rbp + 0x1478]
0x1404da381 mov rax, qword ptr [r14 + 0x10]
0x1404da385 mov rdx, qword ptr [r14 + 0x18]
0x1404da389 cmp rax, 2
0x1404da38d je 0x1404da3b0
0x1404da38f cmp eax, 1
0x1404da392 jne 0x1404da3c0
0x1404da394 lea rcx, [rbp + 0x600]
0x1404da39b call 0x140267ac0
0x1404da3a0 jmp 0x1404da3cc
0x1404da3a2 nop word ptr cs:[rax + rax]
0x1404da3b0 lea rcx, [rbp + 0x600]
0x1404da3b7 call 0x14051f480
0x1404da3bc jmp 0x1404da3cc
0x1404da3be nop 
0x1404da3c0 lea rcx, [rbp + 0x600]
0x1404da3c7 call 0x1404f29a0
0x1404da3cc cmp word ptr [rbp + 0x600], 0x12
0x1404da3d4 je 0x1404da420
0x1404da3d6 mov r8d, 0x88
0x1404da3dc lea rdi, [rbp + 0xc30]
0x1404da3e3 mov rcx, rdi
0x1404da3e6 lea rdx, [rbp + 0x600]
0x1404da3ed call 0x142923c00
0x1404da3f2 mov rsi, qword ptr [rbp + 0x13c0]
0x1404da3f9 cmp rsi, qword ptr [rbp + 0x13b0]
0x1404da400 jne 0x1404da350
0x1404da406 lea rcx, [rbp + 0x13b0]
0x1404da40d lea rdx, [rip + 0x250f8e4]
0x1404da414 call 0x140563170
0x1404da419 jmp 0x1404da350
0x1404da41e nop 
0x1404da420 cmp byte ptr [rbp + 0x602], 0
0x1404da427 jne 0x1404dedae
0x1404da42d mov rcx, qword ptr [rbp + 0x1488]
0x1404da434 movabs rdx, 0xfb3868c4bf7b3b51
0x1404da43e movabs r8, 0xe49e3d2090dcf639
0x1404da448 call 0x1400dd6f0
0x1404da44d test rax, rax
0x1404da450 mov rcx, qword ptr [rbp + 0x1478]
0x1404da457 je 0x1404da490
0x1404da459 cmp qword ptr [rcx + 0x248], rdx
0x1404da460 jbe 0x1404da490
0x1404da462 mov rax, qword ptr [rcx + 0x240]
0x1404da469 mov rax, qword ptr [rax + rdx*8]
0x1404da46d test rax, rax
0x1404da470 je 0x1404da490
0x1404da472 mov rcx, qword ptr [rcx + 0x210]
0x1404da479 not rax
0x1404da47c lea rax, [rax + rax*2]
0x1404da480 shl rax, 5
0x1404da484 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da48a jne 0x1404dee24
0x1404da490 lea rcx, [rbp - 0x40]
0x1404da494 mov rdx, qword ptr [rbp + 0x1450]
0x1404da49b call 0x1402b4aa0
0x1404da4a0 mov esi, dword ptr [rbp - 0x40]
0x1404da4a3 cmp esi, 3
0x1404da4a6 je 0x1404da650
0x1404da4ac movsd xmm6, qword ptr [rbp - 0x3c]
0x1404da4b1 cmp esi, 2
0x1404da4b4 jb 0x1404da57d
0x1404da4ba mov rcx, qword ptr [rbp + 0x1488]
0x1404da4c1 movabs rdx, 0x923f1ac331d52636
0x1404da4cb movabs r8, 0x4114c9eaff932cf6
0x1404da4d5 call 0x1400dd6f0
0x1404da4da test rax, rax
0x1404da4dd mov r8, qword ptr [rbp + 0x1478]
0x1404da4e4 je 0x1404df4ff
0x1404da4ea cmp qword ptr [r8 + 0x248], rdx
0x1404da4f1 jbe 0x1404df4ff
0x1404da4f7 mov rax, qword ptr [r8 + 0x240]
0x1404da4fe mov rcx, qword ptr [rax + rdx*8]
0x1404da502 test rcx, rcx
0x1404da505 je 0x1404df4ff
0x1404da50b mov rax, qword ptr [r8 + 0x210]
0x1404da512 not rcx
0x1404da515 lea rcx, [rcx + rcx*2]
0x1404da519 shl rcx, 5
0x1404da51d cmp qword ptr [rax + rcx + 0x20], 0
0x1404da523 je 0x1404df4ff
0x1404da529 add rax, rcx
0x1404da52c mov rdi, qword ptr [rax + 0x10]
0x1404da530 mov ecx, dword ptr [r8 + 0x5e0]
0x1404da537 mov dword ptr [rax + 0x5c], ecx
0x1404da53a mov rbx, qword ptr [rdi + 0x30]
0x1404da53e mov r14, qword ptr [rdi + 0x40]
0x1404da542 cmp rbx, qword ptr [rdi + 0x20]
0x1404da546 jne 0x1404da558
0x1404da548 lea rcx, [rdi + 0x20]
0x1404da54c lea rdx, [rip + 0x2518bd5]
0x1404da553 call 0x140562db0
0x1404da558 mov rax, qword ptr [rdi + 0x28]
0x1404da55c lea rcx, [rbx + rbx*2]
0x1404da560 mov qword ptr [rax + rcx*8], r14
0x1404da564 mov dword ptr [rax + rcx*8 + 8], 1
0x1404da56c movlpd qword ptr [rax + rcx*8 + 0xc], xmm6
0x1404da572 inc rbx
0x1404da575 mov qword ptr [rdi + 0x30], rbx
0x1404da579 inc qword ptr [rdi + 0x40]
0x1404da57d mov rcx, qword ptr [rbp + 0x1478]
0x1404da584 mov rax, qword ptr [rcx + 0x10f8]
0x1404da58b inc rax
0x1404da58e mov qword ptr [rcx + 0x10f8], rax
0x1404da595 mov dword ptr [rbp + 0xc40], esi
0x1404da59b movlpd qword ptr [rbp + 0xc44], xmm6
0x1404da5a3 mov qword ptr [rbp + 0xc38], rax
0x1404da5aa movabs rax, 0x8000000000000001
0x1404da5b4 mov qword ptr [rbp + 0xc30], rax
0x1404da5bb lea rcx, [rbp + 0x600]
0x1404da5c2 mov rdx, qword ptr [rbp + 0x1418]
0x1404da5c9 lea r8, [rbp + 0xc30]
0x1404da5d0 call 0x1403c5a10
0x1404da5d5 mov rax, qword ptr [rbp + 0x600]
0x1404da5dc movabs rcx, 0x8000000000000007
0x1404da5e6 cmp rax, rcx
0x1404da5e9 je 0x1404da650
0x1404da5eb movabs rcx, 0x8000000000000000
0x1404da5f5 xor rax, rcx
0x1404da5f8 cmp rax, 7
0x1404da5fc mov ecx, 3
0x1404da601 cmovae rax, rcx
0x1404da605 cmp rax, 4
0x1404da609 je 0x1404da61f
0x1404da60b cmp rax, 3
0x1404da60f jne 0x1404da650
0x1404da611 lea rcx, [rbp + 0x600]
0x1404da618 call 0x1405f0d10
0x1404da61d jmp 0x1404da650
0x1404da61f mov rdx, qword ptr [rbp + 0x608]
0x1404da626 movabs rax, 0x8000000000000002
0x1404da630 cmp rdx, rax
0x1404da633 jl 0x1404da650
0x1404da635 test rdx, rdx
0x1404da638 je 0x1404da650
0x1404da63a mov rcx, qword ptr [rbp + 0x610]
0x1404da641 mov r8d, 1
0x1404da647 call 0x140613c20
0x1404da64c nop dword ptr [rax]
0x1404da650 lea rcx, [rbp + 0xc30]
0x1404da657 mov rdx, qword ptr [rbp + 0x1450]
0x1404da65e call 0x1402b47f0
0x1404da663 mov r13, qword ptr [rbp + 0xc40]
0x1404da66a test r13, r13
0x1404da66d je 0x1404da960
0x1404da673 mov rbx, qword ptr [rbp + 0xc38]
0x1404da67a add r13, rbx
0x1404da67d jmp 0x1404da69c
0x1404da67f call 0x140613c20
0x1404da684 nop word ptr cs:[rax + rax]
0x1404da690 dec r13
0x1404da693 cmp rbx, r13
0x1404da696 je 0x1404da960
0x1404da69c mov rcx, qword ptr [rbp + 0x1478]
0x1404da6a3 mov rax, qword ptr [rcx + 0x20]
0x1404da6a7 mov rdx, qword ptr [rcx + 0x28]
0x1404da6ab cmp rax, 2
0x1404da6af je 0x1404da6f0
0x1404da6b1 cmp eax, 1
0x1404da6b4 jne 0x1404da720
0x1404da6b6 movabs rax, 0x8000000000000002
0x1404da6c0 mov qword ptr [rbp + 0xc30], rax
0x1404da6c7 mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da6cf lea rcx, [rbp + 0x600]
0x1404da6d6 lea r8, [rbp + 0xc30]
0x1404da6dd call 0x1402662d0
0x1404da6e2 jmp 0x1404da74c
0x1404da6e4 nop word ptr cs:[rax + rax]
0x1404da6f0 movabs rax, 0x8000000000000002
0x1404da6fa mov qword ptr [rbp + 0xc30], rax
0x1404da701 mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da709 lea rcx, [rbp + 0x600]
0x1404da710 lea r8, [rbp + 0xc30]
0x1404da717 call 0x140517cc0
0x1404da71c jmp 0x1404da74c
0x1404da71e nop 
0x1404da720 movabs rax, 0x8000000000000002
0x1404da72a mov qword ptr [rbp + 0xc30], rax
0x1404da731 mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da739 lea rcx, [rbp + 0x600]
0x1404da740 lea r8, [rbp + 0xc30]
0x1404da747 call 0x1404f0610
0x1404da74c cmp dword ptr [rbp + 0x600], 2
0x1404da753 je 0x1404da690
0x1404da759 movups xmm0, xmmword ptr [rbp + 0x600]
0x1404da760 movdqu xmm1, xmmword ptr [rbp + 0x610]
0x1404da768 movdqu xmm2, xmmword ptr [rbp + 0x620]
0x1404da770 movdqu xmm3, xmmword ptr [rbp + 0x630]
0x1404da778 movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404da77f movups xmm0, xmmword ptr [rbp + 0x670]
0x1404da786 movaps xmmword ptr [rbp + 0xca0], xmm0
0x1404da78d movups xmm0, xmmword ptr [rbp + 0x660]
0x1404da794 movaps xmmword ptr [rbp + 0xc90], xmm0
0x1404da79b movups xmm0, xmmword ptr [rbp + 0x650]
0x1404da7a2 movaps xmmword ptr [rbp + 0xc80], xmm0
0x1404da7a9 movupd xmm0, xmmword ptr [rbp + 0x640]
0x1404da7b1 movapd xmmword ptr [rbp + 0xc70], xmm0
0x1404da7b9 movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404da7c1 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404da7c9 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404da7d1 mov rax, qword ptr [rbp + 0xc30]
0x1404da7d8 test rax, rax
0x1404da7db je 0x1404def5e
0x1404da7e1 mov r12, qword ptr [rbp + 0x608]
0x1404da7e8 movabs rax, 0x8000000000000007
0x1404da7f2 cmp r12, rax
0x1404da7f5 je 0x1404da690
0x1404da7fb mov r14, qword ptr [rbp + 0x610]
0x1404da802 mov rax, r12
0x1404da805 movabs rcx, 0x8000000000000000
0x1404da80f xor rax, rcx
0x1404da812 cmp rax, 7
0x1404da816 mov ecx, 3
0x1404da81b cmovae rax, rcx
0x1404da81f cmp rax, 4
0x1404da823 je 0x1404da90f
0x1404da829 cmp rax, 3
0x1404da82d jne 0x1404da690
0x1404da833 mov rsi, qword ptr [rbp + 0x620]
0x1404da83a mov rdi, qword ptr [rbp + 0x628]
0x1404da841 mov r15, qword ptr [rbp + 0x630]
0x1404da848 mov rdx, qword ptr [rbp + 0x638]
0x1404da84f lea rax, [rdx*2]
0x1404da857 test rax, rax
0x1404da85a jne 0x1404da940
0x1404da860 test r12, r12
0x1404da863 je 0x1404da876
0x1404da865 mov r8d, 1
0x1404da86b mov rcx, r14
0x1404da86e mov rdx, r12
0x1404da871 call 0x140613c20
0x1404da876 test r15, r15
0x1404da879 je 0x1404da8f1
0x1404da87b lea r14, [rdi + 0x50]
0x1404da87f jmp 0x1404da899
0x1404da881 nop word ptr cs:[rax + rax]
0x1404da890 sub r14, -0x80
0x1404da894 dec r15
0x1404da897 je 0x1404da8f1
0x1404da899 mov rdx, qword ptr [r14 - 0x38]
0x1404da89d mov rax, rdx
0x1404da8a0 neg rax
0x1404da8a3 jo 0x1404da8b6
0x1404da8a5 jae 0x1404da8b6
0x1404da8a7 mov rcx, qword ptr [r14 - 0x30]
0x1404da8ab mov r8d, 1
0x1404da8b1 call 0x140613c20
0x1404da8b6 mov rdx, qword ptr [r14 - 0x20]
0x1404da8ba mov rax, rdx
0x1404da8bd neg rax
0x1404da8c0 jo 0x1404da8d3
0x1404da8c2 jae 0x1404da8d3
0x1404da8c4 mov rcx, qword ptr [r14 - 0x18]
0x1404da8c8 mov r8d, 1
0x1404da8ce call 0x140613c20
0x1404da8d3 mov rdx, qword ptr [r14 - 8]
0x1404da8d7 mov rax, rdx
0x1404da8da neg rax
0x1404da8dd jo 0x1404da890
0x1404da8df jae 0x1404da890
0x1404da8e1 mov rcx, qword ptr [r14]
0x1404da8e4 mov r8d, 1
0x1404da8ea call 0x140613c20
0x1404da8ef jmp 0x1404da890
0x1404da8f1 test rsi, rsi
0x1404da8f4 je 0x1404da690
0x1404da8fa shl rsi, 7
0x1404da8fe mov r8d, 8
0x1404da904 mov rcx, rdi
0x1404da907 mov rdx, rsi
0x1404da90a jmp 0x1404da67f
0x1404da90f movabs rax, 0x8000000000000002
0x1404da919 cmp r14, rax
0x1404da91c jl 0x1404da690
0x1404da922 test r14, r14
0x1404da925 je 0x1404da690
0x1404da92b mov rcx, qword ptr [rbp + 0x618]
0x1404da932 mov r8d, 1
0x1404da938 mov rdx, r14
0x1404da93b jmp 0x1404da67f
0x1404da940 mov rcx, qword ptr [rbp + 0x640]
0x1404da947 mov r8d, 1
0x1404da94d call 0x140613c20
0x1404da952 test r12, r12
0x1404da955 jne 0x1404da865
0x1404da95b jmp 0x1404da876
0x1404da960 mov rcx, qword ptr [rbp + 0x1450]
0x1404da967 call 0x1402b4c40
0x1404da96c cmp eax, 4
0x1404da96f je 0x1404daa30
0x1404da975 mov rcx, rax
0x1404da978 shr rcx, 0x20
0x1404da97c mov dword ptr [rbp + 0xc38], eax
0x1404da982 mov dword ptr [rbp + 0xc3c], ecx
0x1404da988 movabs rax, 0x8000000000000000
0x1404da992 mov qword ptr [rbp + 0xc30], rax
0x1404da999 lea rcx, [rbp + 0x600]
0x1404da9a0 mov rdx, qword ptr [rbp + 0x1418]
0x1404da9a7 lea r8, [rbp + 0xc30]
0x1404da9ae call 0x1403c5a10
0x1404da9b3 mov rax, qword ptr [rbp + 0x600]
0x1404da9ba movabs rcx, 0x8000000000000007
0x1404da9c4 cmp rax, rcx
0x1404da9c7 je 0x1404daa30
0x1404da9c9 movabs rcx, 0x8000000000000000
0x1404da9d3 xor rax, rcx
0x1404da9d6 cmp rax, 7
0x1404da9da mov ecx, 3
0x1404da9df cmovae rax, rcx
0x1404da9e3 cmp rax, 4
0x1404da9e7 je 0x1404da9fd
0x1404da9e9 cmp rax, 3
0x1404da9ed jne 0x1404daa30
0x1404da9ef lea rcx, [rbp + 0x600]
0x1404da9f6 call 0x1405f0d10
0x1404da9fb jmp 0x1404daa30
0x1404da9fd mov rdx, qword ptr [rbp + 0x608]
0x1404daa04 movabs rax, 0x8000000000000002
0x1404daa0e cmp rdx, rax
0x1404daa11 jl 0x1404daa30
0x1404daa13 test rdx, rdx
0x1404daa16 je 0x1404daa30
0x1404daa18 mov rcx, qword ptr [rbp + 0x610]
0x1404daa1f mov r8d, 1
0x1404daa25 call 0x140613c20
0x1404daa2a nop word ptr [rax + rax]
0x1404daa30 mov rcx, qword ptr [rbp + 0x1450]
0x1404daa37 call 0x1402b4910
0x1404daa3c test rax, rax
0x1404daa3f je 0x1404daaf0
0x1404daa45 mov qword ptr [rbp + 0xc38], rax
0x1404daa4c movabs rax, 0x8000000000000006
0x1404daa56 mov qword ptr [rbp + 0xc30], rax
0x1404daa5d lea rcx, [rbp + 0x600]
0x1404daa64 mov rdx, qword ptr [rbp + 0x1418]
0x1404daa6b lea r8, [rbp + 0xc30]
0x1404daa72 call 0x1403c5a10
0x1404daa77 mov rax, qword ptr [rbp + 0x600]
0x1404daa7e movabs rcx, 0x8000000000000007
0x1404daa88 cmp rax, rcx
0x1404daa8b je 0x1404daaf0
0x1404daa8d movabs rcx, 0x8000000000000000
0x1404daa97 xor rax, rcx
0x1404daa9a cmp rax, 7
0x1404daa9e mov ecx, 3
0x1404daaa3 cmovae rax, rcx
0x1404daaa7 cmp rax, 4
0x1404daaab je 0x1404daac1
0x1404daaad cmp rax, 3
0x1404daab1 jne 0x1404daaf0
0x1404daab3 lea rcx, [rbp + 0x600]
0x1404daaba call 0x1405f0d10
0x1404daabf jmp 0x1404daaf0
0x1404daac1 mov rdx, qword ptr [rbp + 0x608]
0x1404daac8 movabs rax, 0x8000000000000002
0x1404daad2 cmp rdx, rax
0x1404daad5 jl 0x1404daaf0
0x1404daad7 test rdx, rdx
0x1404daada je 0x1404daaf0
0x1404daadc mov rcx, qword ptr [rbp + 0x610]
0x1404daae3 mov r8d, 1
0x1404daae9 call 0x140613c20
0x1404daaee nop 
0x1404daaf0 mov rdx, qword ptr [rbp + 0x13b8]
0x1404daaf7 mov rcx, qword ptr [rbp + 0x13c0]
0x1404daafe mov qword ptr [rbp + 0x13c0], 0
0x1404dab09 mov rax, rcx
0x1404dab0c shl rax, 7
0x1404dab10 lea rax, [rax + rcx*8]
0x1404dab14 add rax, rdx
0x1404dab17 mov qword ptr [rbp + 0x13e0], rdx
0x1404dab1e mov qword ptr [rbp + 0x13e8], rax
0x1404dab25 lea r8, [rbp + 0x13b0]
0x1404dab2c mov qword ptr [rbp + 0x13f0], r8
0x1404dab33 mov qword ptr [rbp + 0x13f8], rcx
0x1404dab3a mov qword ptr [rbp + 0x1400], 0
0x1404dab45 mov r12, qword ptr [rbp + 0x1478]
0x1404dab4c lea r13, [rbp + 0x12a2]
0x1404dab53 jmp 0x1404dab99
0x1404dab55 mov rax, qword ptr [rsi + 0x28]
0x1404dab59 mov rcx, rdi
0x1404dab5c shl rcx, 5
0x1404dab60 mov qword ptr [rax + rcx], rbx
0x1404dab64 mov rdx, qword ptr [rbp + 0xc40]
0x1404dab6b mov qword ptr [rax + rcx + 0x18], rdx
0x1404dab70 movapd xmm0, xmmword ptr [rbp + 0xc30]
0x1404dab78 movupd xmmword ptr [rax + rcx + 8], xmm0
0x1404dab7e nop 
0x1404dab80 inc rdi
0x1404dab83 mov qword ptr [rsi + 0x30], rdi
0x1404dab87 inc qword ptr [rsi + 0x40]
0x1404dab8b mov rdx, qword ptr [rbp + 0x13e0]
0x1404dab92 mov rax, qword ptr [rbp + 0x13e8]
0x1404dab99 cmp rdx, rax
0x1404dab9c je 0x1404dc850
0x1404daba2 lea rax, [rdx + 0x88]
0x1404daba9 mov qword ptr [rbp + 0x13e0], rax
0x1404dabb0 movzx esi, word ptr [rdx]
0x1404dabb3 cmp si, 0x12
0x1404dabb7 je 0x1404dc850
0x1404dabbd add rdx, 2
0x1404dabc1 mov word ptr [rbp + 0x12a0], si
0x1404dabc8 mov r8d, 0x86
0x1404dabce mov rcx, r13
0x1404dabd1 call 0x142923c10
0x1404dabd6 add esi, -4
0x1404dabd9 cmp si, 0xe
0x1404dabdd mov eax, 7
0x1404dabe2 cmovae esi, eax
0x1404dabe5 movzx eax, si
0x1404dabe8 lea rcx, [rip + 0x250f861]
0x1404dabef movsxd rax, dword ptr [rcx + rax*4]
0x1404dabf3 add rax, rcx
0x1404dabf6 jmp rax
0x1404dabf8 mov edi, dword ptr [rbp + 0x12a4]
0x1404dabfe cmp edi, 0x4e20
0x1404dac04 jae 0x1404dab8b
0x1404dac06 mov esi, dword ptr [rbp + 0x12a8]
0x1404dac0c mov byte ptr [rbp + 0x14a7], 1
0x1404dac13 mov byte ptr [rbp + 0x14a6], 1
0x1404dac1a mov byte ptr [rbp + 0x14a5], 1
0x1404dac21 mov rcx, qword ptr [rbp + 0x1488]
0x1404dac28 movabs rdx, 0x864ddca98c2312a6
0x1404dac32 movabs r8, 0x27b0e9369e9a484e
0x1404dac3c call 0x1400dd6f0
0x1404dac41 test rax, rax
0x1404dac44 je 0x1404df27e
0x1404dac4a cmp qword ptr [r12 + 0x248], rdx
0x1404dac52 jbe 0x1404df27e
0x1404dac58 mov rax, qword ptr [r12 + 0x240]
0x1404dac60 mov rcx, qword ptr [rax + rdx*8]
0x1404dac64 test rcx, rcx
0x1404dac67 je 0x1404df27e
0x1404dac6d mov rax, qword ptr [r12 + 0x210]
0x1404dac75 not rcx
0x1404dac78 lea rcx, [rcx + rcx*2]
0x1404dac7c shl rcx, 5
0x1404dac80 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dac86 je 0x1404df27e
0x1404dac8c add rax, rcx
0x1404dac8f mov rcx, qword ptr [rax + 0x10]
0x1404dac93 mov edx, dword ptr [r12 + 0x5e0]
0x1404dac9b mov dword ptr [rax + 0x5c], edx
0x1404dac9e mov byte ptr [rbp + 0x14a7], 1
0x1404daca5 mov byte ptr [rbp + 0x14a6], 1
0x1404dacac mov byte ptr [rbp + 0x14a5], 1
0x1404dacb3 mov edx, edi
0x1404dacb5 mov r8d, esi
0x1404dacb8 call 0x140949c00
0x1404dacbd mov byte ptr [rbp + 0x14a7], 1
0x1404dacc4 mov byte ptr [rbp + 0x14a6], 1
0x1404daccb mov byte ptr [rbp + 0x14a5], 1
0x1404dacd2 mov rcx, qword ptr [rbp + 0x1488]
0x1404dacd9 movabs rdx, 0xa4c02661e67b048b
0x1404dace3 movabs r8, 0xeae761e6aa2a67ec
0x1404daced call 0x1400dd6f0
0x1404dacf2 test rax, rax
0x1404dacf5 je 0x1404df2a4
0x1404dacfb cmp qword ptr [r12 + 0x248], rdx
0x1404dad03 jbe 0x1404df2a4
0x1404dad09 mov rax, qword ptr [r12 + 0x240]
0x1404dad11 mov rcx, qword ptr [rax + rdx*8]
0x1404dad15 test rcx, rcx
0x1404dad18 je 0x1404df2a4
0x1404dad1e mov rax, qword ptr [r12 + 0x210]
0x1404dad26 not rcx
0x1404dad29 lea rcx, [rcx + rcx*2]
0x1404dad2d shl rcx, 5
0x1404dad31 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dad37 je 0x1404df2a4
0x1404dad3d add rax, rcx
0x1404dad40 mov rcx, qword ptr [rax + 0x10]
0x1404dad44 mov edx, dword ptr [r12 + 0x5e0]
0x1404dad4c mov dword ptr [rax + 0x5c], edx
0x1404dad4f mov dword ptr [rcx], edi
0x1404dad51 mov dword ptr [rcx + 4], esi
0x1404dad54 mov byte ptr [rbp + 0x14a7], 1
0x1404dad5b mov byte ptr [rbp + 0x14a6], 1
0x1404dad62 mov byte ptr [rbp + 0x14a5], 1
0x1404dad69 mov rcx, qword ptr [rbp + 0x1488]
0x1404dad70 movabs rdx, 0x3d7b56e787d47c40
0x1404dad7a movabs r8, 0x67b0b10a8d43bfc7
0x1404dad84 call 0x1400dd6f0
0x1404dad89 test rax, rax
0x1404dad8c je 0x1404df2f0
0x1404dad92 cmp qword ptr [r12 + 0x248], rdx
0x1404dad9a jbe 0x1404df2f0
0x1404dada0 mov rax, qword ptr [r12 + 0x240]
0x1404dada8 mov rax, qword ptr [rax + rdx*8]
0x1404dadac test rax, rax
0x1404dadaf je 0x1404df2f0
0x1404dadb5 mov rbx, qword ptr [r12 + 0x210]
0x1404dadbd not rax
0x1404dadc0 lea rax, [rax + rax*2]
0x1404dadc4 shl rax, 5
0x1404dadc8 cmp qword ptr [rbx + rax + 0x20], 0
0x1404dadce je 0x1404df2f0
0x1404dadd4 add rbx, rax
0x1404dadd7 mov r14, qword ptr [rbx + 0x10]
0x1404daddb cvtsi2sd xmm0, edi
0x1404daddf mov r15d, dword ptr [r12 + 0x5e0]
0x1404dade7 mov byte ptr [rbp + 0x14a7], 1
0x1404dadee mov byte ptr [rbp + 0x14a6], 1
0x1404dadf5 mov byte ptr [rbp + 0x14a5], 1
0x1404dadfc call 0x1406977f0
0x1404dae01 mov edi, eax
0x1404dae03 mov eax, esi
0x1404dae05 xorps xmm0, xmm0
0x1404dae08 cvtsi2sd xmm0, rax
0x1404dae0d mov byte ptr [rbp + 0x14a7], 1
0x1404dae14 mov byte ptr [rbp + 0x14a6], 1
0x1404dae1b mov byte ptr [rbp + 0x14a5], 1
0x1404dae22 call 0x1406977f0
0x1404dae27 shl rax, 0x20
0x1404dae2b mov ecx, edi
0x1404dae2d or rcx, rax
0x1404dae30 mov dword ptr [rbx + 0x5c], r15d
0x1404dae34 mov qword ptr [r14 + 0x60], rcx
0x1404dae38 mov sil, 1
0x1404dae3b cmp qword ptr [r12 + 0xb8], 0
0x1404dae44 jne 0x1404dbd87
0x1404dae4a jmp 0x1404dbe2e
0x1404dae4f mov rax, qword ptr [rbp + 0x12a8]
0x1404dae56 mov rcx, qword ptr [r12 + 0x1100]
0x1404dae5e cmp rax, rcx
0x1404dae61 cmova rcx, rax
0x1404dae65 mov qword ptr [r12 + 0x1100], rcx
0x1404dae6d jmp 0x1404dab8b
0x1404dae72 mov byte ptr [rbp + 0x14a7], 1
0x1404dae79 mov byte ptr [rbp + 0x14a6], 1
0x1404dae80 mov byte ptr [rbp + 0x14a5], 1
0x1404dae87 mov rcx, qword ptr [rbp + 0x1488]
0x1404dae8e movabs rdx, 0x3d7b56e787d47c40
0x1404dae98 movabs r8, 0x67b0b10a8d43bfc7
0x1404daea2 call 0x1400dd6f0
0x1404daea7 test rax, rax
0x1404daeaa je 0x1404df0d9
0x1404daeb0 cmp qword ptr [r12 + 0x248], rdx
0x1404daeb8 jbe 0x1404df0d9
0x1404daebe mov rax, qword ptr [r12 + 0x240]
0x1404daec6 mov rcx, qword ptr [rax + rdx*8]
0x1404daeca test rcx, rcx
0x1404daecd je 0x1404df0d9
0x1404daed3 mov rax, qword ptr [r12 + 0x210]
0x1404daedb not rcx
0x1404daede lea rcx, [rcx + rcx*2]
0x1404daee2 shl rcx, 5
0x1404daee6 cmp qword ptr [rax + rcx + 0x20], 0
0x1404daeec je 0x1404df0d9
0x1404daef2 add rax, rcx
0x1404daef5 mov rbx, qword ptr [rax + 0x10]
0x1404daef9 mov ecx, dword ptr [r12 + 0x5e0]
0x1404daf01 mov dword ptr [rax + 0x5c], ecx
0x1404daf04 mov rcx, qword ptr [r12 + 0x10e8]
0x1404daf0c add rcx, 0x10
0x1404daf10 mov byte ptr [rbp + 0x14a7], 1
0x1404daf17 mov byte ptr [rbp + 0x14a6], 1
0x1404daf1e mov byte ptr [rbp + 0x14a5], 1
0x1404daf25 call 0x14069d680
0x1404daf2a mov r15, rax
0x1404daf2d mov r14, qword ptr [rax + 0x28]
0x1404daf31 cmp r14, 1
0x1404daf35 jne 0x1404daf84
0x1404daf37 mov rdi, qword ptr [r15 + 0x30]
0x1404daf3b test rdi, rdi
0x1404daf3e je 0x1404dc096
0x1404daf44 mov byte ptr [rbp + 0x1498], dl
0x1404daf4a movzx eax, byte ptr [rip + 0x3159cb8]
0x1404daf51 mov ecx, 0xdc
0x1404daf56 mov edx, 4
0x1404daf5b call 0x140613c10
0x1404daf60 test rax, rax
0x1404daf63 je 0x1404df554
0x1404daf69 mov rsi, rax
0x1404daf6c mov r8d, 0xdc
0x1404daf72 mov rcx, rax
0x1404daf75 mov rdx, rdi
0x1404daf78 call 0x142923c00
0x1404daf7d movzx edx, byte ptr [rbp + 0x1498]
0x1404daf84 test dl, 1
0x1404daf87 jne 0x1404daf9f
0x1404daf89 mov rax, qword ptr [rip + 0x3151490]
0x1404daf90 mov rax, qword ptr [rax]
0x1404daf93 shl rax, 1
0x1404daf96 test rax, rax
0x1404daf99 jne 0x1404dc7ab
0x1404daf9f xor eax, eax
0x1404dafa1 xchg byte ptr [r15], al
0x1404dafa4 cmp al, 2
0x1404dafa6 je 0x1404dc6ab
0x1404dafac test r14b, r14b
0x1404dafaf je 0x1404dbc79
0x1404dafb5 mov byte ptr [rbx + 0x7c], 0
0x1404dafb9 test rsi, rsi
0x1404dafbc je 0x1404dbc7d
0x1404dafc2 mov edx, 0xdc
0x1404dafc7 mov r8d, 4
0x1404dafcd mov rcx, rsi
0x1404dafd0 call 0x140613c20
0x1404dafd5 jmp 0x1404dbc7d
0x1404dafda mov rax, qword ptr [r13 + 0x16]
0x1404dafde mov qword ptr [rbp + 0xc40], rax
0x1404dafe5 movupd xmm0, xmmword ptr [r13 + 6]
0x1404dafeb movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404daff3 mov byte ptr [rbp + 0x14a7], 1
0x1404daffa mov byte ptr [rbp + 0x14a6], 1
0x1404db001 mov byte ptr [rbp + 0x14a5], 1
0x1404db008 mov rcx, qword ptr [rbp + 0x1488]
0x1404db00f movabs rdx, 0x2288607094c8a33c
0x1404db019 movabs r8, 0xb2b3168f5a59b2bc
0x1404db023 call 0x1400dd6f0
0x1404db028 test rax, rax
0x1404db02b je 0x1404df075
0x1404db031 cmp qword ptr [r12 + 0x248], rdx
0x1404db039 jbe 0x1404df075
0x1404db03f mov rax, qword ptr [r12 + 0x240]
0x1404db047 mov rcx, qword ptr [rax + rdx*8]
0x1404db04b test rcx, rcx
0x1404db04e je 0x1404df075
0x1404db054 mov rax, qword ptr [r12 + 0x210]
0x1404db05c not rcx
0x1404db05f lea rcx, [rcx + rcx*2]
0x1404db063 shl rcx, 5
0x1404db067 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db06d je 0x1404df075
0x1404db073 add rax, rcx
0x1404db076 mov rsi, qword ptr [rax + 0x10]
0x1404db07a mov ecx, dword ptr [r12 + 0x5e0]
0x1404db082 mov dword ptr [rax + 0x5c], ecx
0x1404db085 mov rdi, qword ptr [rsi + 0x30]
0x1404db089 mov rbx, qword ptr [rsi + 0x40]
0x1404db08d cmp rdi, qword ptr [rsi + 0x20]
0x1404db091 jne 0x1404dab55
0x1404db097 lea rcx, [rsi + 0x20]
0x1404db09b mov byte ptr [rbp + 0x14a7], 1
0x1404db0a2 mov byte ptr [rbp + 0x14a6], 1
0x1404db0a9 mov byte ptr [rbp + 0x14a5], 1
0x1404db0b0 lea rdx, [rip + 0x2518071]
0x1404db0b7 call 0x140562c30
0x1404db0bc jmp 0x1404dab55
0x1404db0c1 movsd xmm6, qword ptr [rbp + 0x12a4]
0x1404db0c9 mov byte ptr [rbp + 0x14a7], 1
0x1404db0d0 mov byte ptr [rbp + 0x14a6], 1
0x1404db0d7 mov byte ptr [rbp + 0x14a5], 1
0x1404db0de mov rcx, qword ptr [rbp + 0x1488]
0x1404db0e5 movabs rdx, 0x923f1ac331d52636
0x1404db0ef movabs r8, 0x4114c9eaff932cf6
0x1404db0f9 call 0x1400dd6f0
0x1404db0fe test rax, rax
0x1404db101 je 0x1404defc5
0x1404db107 cmp qword ptr [r12 + 0x248], rdx
0x1404db10f jbe 0x1404defc5
0x1404db115 mov rax, qword ptr [r12 + 0x240]
0x1404db11d mov rcx, qword ptr [rax + rdx*8]
0x1404db121 test rcx, rcx
0x1404db124 je 0x1404defc5
0x1404db12a mov rax, qword ptr [r12 + 0x210]
0x1404db132 not rcx
0x1404db135 lea rcx, [rcx + rcx*2]
0x1404db139 shl rcx, 5
0x1404db13d cmp qword ptr [rax + rcx + 0x20], 0
0x1404db143 je 0x1404defc5
0x1404db149 add rax, rcx
0x1404db14c mov rsi, qword ptr [rax + 0x10]
0x1404db150 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db158 mov dword ptr [rax + 0x5c], ecx
0x1404db15b mov rdi, qword ptr [rsi + 0x30]
0x1404db15f mov rbx, qword ptr [rsi + 0x40]
0x1404db163 cmp rdi, qword ptr [rsi + 0x20]
0x1404db167 jne 0x1404db18e
0x1404db169 lea rcx, [rsi + 0x20]
0x1404db16d mov byte ptr [rbp + 0x14a7], 1
0x1404db174 mov byte ptr [rbp + 0x14a6], 1
0x1404db17b mov byte ptr [rbp + 0x14a5], 1
0x1404db182 lea rdx, [rip + 0x2517f9f]
0x1404db189 call 0x140562db0
0x1404db18e mov rax, qword ptr [rsi + 0x28]
0x1404db192 lea rcx, [rdi + rdi*2]
0x1404db196 mov qword ptr [rax + rcx*8], rbx
0x1404db19a mov dword ptr [rax + rcx*8 + 8], 2
0x1404db1a2 movlpd qword ptr [rax + rcx*8 + 0xc], xmm6
0x1404db1a8 jmp 0x1404dab80
0x1404db1ad mov byte ptr [rbp + 0x14a7], 1
0x1404db1b4 mov byte ptr [rbp + 0x14a6], 1
0x1404db1bb mov byte ptr [rbp + 0x14a5], 1
0x1404db1c2 mov rcx, qword ptr [rbp + 0x1488]
0x1404db1c9 movabs rdx, 0x2ed189581b92decf
0x1404db1d3 movabs r8, 0x15d3d99396065f38
0x1404db1dd call 0x1400dd6f0
0x1404db1e2 test rax, rax
0x1404db1e5 je 0x1404defeb
0x1404db1eb cmp qword ptr [r12 + 0x248], rdx
0x1404db1f3 jbe 0x1404defeb
0x1404db1f9 mov rax, qword ptr [r12 + 0x240]
0x1404db201 mov rcx, qword ptr [rax + rdx*8]
0x1404db205 test rcx, rcx
0x1404db208 je 0x1404defeb
0x1404db20e mov rax, qword ptr [r12 + 0x210]
0x1404db216 not rcx
0x1404db219 lea rcx, [rcx + rcx*2]
0x1404db21d shl rcx, 5
0x1404db221 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db227 je 0x1404defeb
0x1404db22d add rax, rcx
0x1404db230 mov rax, qword ptr [rax + 0x10]
0x1404db234 mov rdx, qword ptr [rax]
0x1404db237 add rdx, 0x10
0x1404db23b mov byte ptr [rbp + 0x14a7], 1
0x1404db242 mov byte ptr [rbp + 0x14a6], 1
0x1404db249 mov byte ptr [rbp + 0x14a5], 1
0x1404db250 mov rcx, qword ptr [rbp + 0x1420]
0x1404db257 call 0x140c0e950
0x1404db25c jmp 0x1404dab8b
0x1404db261 mov byte ptr [rbp + 0x14a7], 1
0x1404db268 mov byte ptr [rbp + 0x14a6], 1
0x1404db26f mov byte ptr [rbp + 0x14a5], 1
0x1404db276 mov rcx, qword ptr [rbp + 0x1488]
0x1404db27d movabs rdx, 0x2e383ebb9376bfa1
0x1404db287 movabs r8, 0x96b847e2c17c833c
0x1404db291 call 0x1400dd6f0
0x1404db296 test rax, rax
0x1404db299 je 0x1404df04f
0x1404db29f cmp qword ptr [r12 + 0x248], rdx
0x1404db2a7 jbe 0x1404df04f
0x1404db2ad mov rax, qword ptr [r12 + 0x240]
0x1404db2b5 mov rcx, qword ptr [rax + rdx*8]
0x1404db2b9 test rcx, rcx
0x1404db2bc je 0x1404df04f
0x1404db2c2 mov rax, qword ptr [r12 + 0x210]
0x1404db2ca not rcx
0x1404db2cd lea rcx, [rcx + rcx*2]
0x1404db2d1 shl rcx, 5
0x1404db2d5 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db2db je 0x1404df04f
0x1404db2e1 add rax, rcx
0x1404db2e4 mov rsi, qword ptr [rax + 0x10]
0x1404db2e8 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db2f0 mov dword ptr [rax + 0x5c], ecx
0x1404db2f3 mov rdi, qword ptr [rsi + 0x30]
0x1404db2f7 mov rbx, qword ptr [rsi + 0x40]
0x1404db2fb cmp rdi, qword ptr [rsi + 0x20]
0x1404db2ff jne 0x1404db326
0x1404db301 lea rcx, [rsi + 0x20]
0x1404db305 mov byte ptr [rbp + 0x14a7], 1
0x1404db30c mov byte ptr [rbp + 0x14a6], 1
0x1404db313 mov byte ptr [rbp + 0x14a5], 1
0x1404db31a lea rdx, [rip + 0x2517e07]
0x1404db321 call 0x140562870
0x1404db326 mov rax, qword ptr [rsi + 0x28]
0x1404db32a mov qword ptr [rax + rdi*8], rbx
0x1404db32e jmp 0x1404dab80
0x1404db333 movups xmm0, xmmword ptr [rbp + 0x1310]
0x1404db33a movaps xmmword ptr [rbp + 0xca0], xmm0
0x1404db341 movups xmm0, xmmword ptr [rbp + 0x1300]
0x1404db348 movaps xmmword ptr [rbp + 0xc90], xmm0
0x1404db34f movups xmm0, xmmword ptr [rbp + 0x12f0]
0x1404db356 movaps xmmword ptr [rbp + 0xc80], xmm0
0x1404db35d movups xmm0, xmmword ptr [rbp + 0x12e0]
0x1404db364 movaps xmmword ptr [rbp + 0xc70], xmm0
0x1404db36b movupd xmm0, xmmword ptr [rbp + 0x12a0]
0x1404db373 movdqu xmm1, xmmword ptr [rbp + 0x12b0]
0x1404db37b movdqu xmm2, xmmword ptr [rbp + 0x12c0]
0x1404db383 movdqu xmm3, xmmword ptr [rbp + 0x12d0]
0x1404db38b movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404db393 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404db39b movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404db3a3 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404db3ab mov esi, dword ptr [rbp + 0x1320]
0x1404db3b1 test sil, 0x20
0x1404db3b5 je 0x1404db3e7
0x1404db3b7 mov byte ptr [rbp + 0x14a2], 1
0x1404db3be lea rcx, [rbp + 0xc30]
0x1404db3c5 call 0x1406a65f0
0x1404db3ca test rax, rax
0x1404db3cd setne cl
0x1404db3d0 cmp rdx, 1
0x1404db3d4 sete dl
0x1404db3d7 and dl, cl
0x1404db3d9 cmp dl, 1
0x1404db3dc jne 0x1404db3e7
0x1404db3de cmp byte ptr [rax], 0x76
0x1404db3e1 je 0x1404dbf48
0x1404db3e7 mov ebx, esi
0x1404db3e9 and ebx, 4
0x1404db3ec jne 0x1404dbf2c
0x1404db3f2 mov byte ptr [rbp + 0x14a2], 1
0x1404db3f9 lea rcx, [rbp + 0x600]
0x1404db400 lea rdx, [rbp + 0xc30]
0x1404db407 call 0x1400fb850
0x1404db40c movzx r8d, byte ptr [rbp + 0xc90]
0x1404db414 movzx ecx, byte ptr [rbp + 0xca8]
0x1404db41b movzx eax, byte ptr [rbp + 0xca9]
0x1404db422 mov r10b, 0x1b
0x1404db425 cmp r8b, 0x1b
0x1404db429 je 0x1404dc0ec
0x1404db42f lea edx, [r8 - 0x18]
0x1404db433 cmp dl, 3
0x1404db436 movzx edx, dl
0x1404db439 mov r9d, 1
0x1404db43f cmovae edx, r9d
0x1404db443 test dl, dl
0x1404db445 je 0x1404dc0a6
0x1404db44b movzx edx, dl
0x1404db44e cmp edx, 1
0x1404db451 jne 0x1404dc0bc
0x1404db457 lea r9, [rbp + 0xc38]
0x1404db45e mov edx, dword ptr [r9 + 0x59]
0x1404db462 mov r9d, dword ptr [r9 + 0x5c]
0x1404db466 mov dword ptr [rbp + 0x1433], r9d
0x1404db46d mov dword ptr [rbp + 0x1430], edx
0x1404db473 jmp 0x1404dc0bf
0x1404db478 mov rax, qword ptr [r12 + 0x10e8]
0x1404db480 mov rcx, qword ptr [rax + 0x18]
0x1404db484 mov byte ptr [rbp + 0x14a7], 1
0x1404db48b mov byte ptr [rbp + 0x14a6], 1
0x1404db492 mov byte ptr [rbp + 0x14a5], 1
0x1404db499 call 0x140697b00
0x1404db49e mov qword ptr [rbp + 0x600], 1
0x1404db4a9 mov qword ptr [rbp + 0x608], rax
0x1404db4b0 mov byte ptr [rbp + 0x14a7], 1
0x1404db4b7 mov byte ptr [rbp + 0x14a6], 1
0x1404db4be mov byte ptr [rbp + 0x14a5], 1
0x1404db4c5 mov rcx, r12
0x1404db4c8 call 0x1404e2470
0x1404db4cd mov byte ptr [rbp + 0x14a7], 1
0x1404db4d4 mov byte ptr [rbp + 0x14a6], 1
0x1404db4db mov byte ptr [rbp + 0x14a5], 1
0x1404db4e2 mov rcx, qword ptr [rbp + 0x1488]
0x1404db4e9 movabs rdx, 0x3d7b56e787d47c40
0x1404db4f3 movabs r8, 0x67b0b10a8d43bfc7
0x1404db4fd call 0x1400dd6f0
0x1404db502 test rax, rax
0x1404db505 je 0x1404df011
0x1404db50b cmp qword ptr [r12 + 0x248], rdx
0x1404db513 jbe 0x1404df011
0x1404db519 mov rax, qword ptr [r12 + 0x240]
0x1404db521 mov rax, qword ptr [rax + rdx*8]
0x1404db525 test rax, rax
0x1404db528 je 0x1404df011
0x1404db52e mov rdi, qword ptr [r12 + 0x210]
0x1404db536 not rax
0x1404db539 lea rax, [rax + rax*2]
0x1404db53d shl rax, 5
0x1404db541 cmp qword ptr [rdi + rax + 0x20], 0
0x1404db547 je 0x1404df011
0x1404db54d add rdi, rax
0x1404db550 mov rsi, qword ptr [rdi + 0x10]
0x1404db554 mov ebx, dword ptr [r12 + 0x5e0]
0x1404db55c test byte ptr [rbp + 0x600], 1
0x1404db563 je 0x1404dbd0d
0x1404db569 mov byte ptr [rbp + 0x14a7], 1
0x1404db570 mov byte ptr [rbp + 0x14a6], 1
0x1404db577 mov byte ptr [rbp + 0x14a5], 1
0x1404db57e lea rcx, [rbp + 0xc30]
0x1404db585 lea rdx, [rbp + 0x608]
0x1404db58c call 0x140175450
0x1404db591 jmp 0x1404dbd1e
0x1404db596 mov rax, qword ptr [rbp + 0x12a8]
0x1404db59d mov qword ptr [rbp + 0x1468], rax
0x1404db5a4 mov rax, qword ptr [rbp + 0x12b0]
0x1404db5ab mov qword ptr [rbp + 0x1460], rax
0x1404db5b2 movupd xmm0, xmmword ptr [r13 + 0x16]
0x1404db5b8 movdqu xmm1, xmmword ptr [r13 + 0x26]
0x1404db5be movdqu xmm2, xmmword ptr [r13 + 0x36]
0x1404db5c4 movapd xmmword ptr [rbp + 0x1260], xmm0
0x1404db5cc movdqa xmmword ptr [rbp + 0x1270], xmm1
0x1404db5d4 movdqa xmmword ptr [rbp + 0x1280], xmm2
0x1404db5dc mov rax, qword ptr [r13 + 0x46]
0x1404db5e0 mov qword ptr [rbp + 0x1290], rax
0x1404db5e7 mov byte ptr [rbp + 0x1487], 1
0x1404db5ee mov rcx, qword ptr [rbp + 0x1488]
0x1404db5f5 movabs rdx, 0x3d7b56e787d47c40
0x1404db5ff movabs r8, 0x67b0b10a8d43bfc7
0x1404db609 call 0x1400dd6f0
0x1404db60e test rax, rax
0x1404db611 je 0x1404df0c1
0x1404db617 cmp qword ptr [r12 + 0x248], rdx
0x1404db61f jbe 0x1404df0c1
0x1404db625 mov rax, qword ptr [r12 + 0x240]
0x1404db62d mov rcx, qword ptr [rax + rdx*8]
0x1404db631 test rcx, rcx
0x1404db634 je 0x1404df0c1
0x1404db63a mov rax, qword ptr [r12 + 0x210]
0x1404db642 not rcx
0x1404db645 lea rcx, [rcx + rcx*2]
0x1404db649 shl rcx, 5
0x1404db64d cmp qword ptr [rax + rcx + 0x20], 0
0x1404db653 je 0x1404df0c1
0x1404db659 add rax, rcx
0x1404db65c mov rdx, qword ptr [rax + 0x10]
0x1404db660 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db668 mov dword ptr [rax + 0x5c], ecx
0x1404db66b mov rax, qword ptr [rdx + 0x38]
0x1404db66f movabs rcx, 0x8000000000000002
0x1404db679 cmp rax, rcx
0x1404db67c jl 0x1404db69b
0x1404db67e test rax, rax
0x1404db681 je 0x1404db69b
0x1404db683 mov rcx, qword ptr [rdx + 0x40]
0x1404db687 mov r8d, 1
0x1404db68d mov rsi, rdx
0x1404db690 mov rdx, rax
0x1404db693 call 0x140613c20
0x1404db698 mov rdx, rsi
0x1404db69b mov rax, qword ptr [rbp + 0x1468]
0x1404db6a2 mov qword ptr [rdx + 0x38], rax
0x1404db6a6 mov rax, qword ptr [rbp + 0x1460]
0x1404db6ad mov qword ptr [rdx + 0x40], rax
0x1404db6b1 mov rax, qword ptr [rbp + 0x1290]
0x1404db6b8 mov qword ptr [rdx + 0x78], rax
0x1404db6bc movapd xmm0, xmmword ptr [rbp + 0x1260]
0x1404db6c4 movdqa xmm1, xmmword ptr [rbp + 0x1270]
0x1404db6cc movdqa xmm2, xmmword ptr [rbp + 0x1280]
0x1404db6d4 movdqu xmmword ptr [rdx + 0x68], xmm2
0x1404db6d9 movdqu xmmword ptr [rdx + 0x58], xmm1
0x1404db6de movupd xmmword ptr [rdx + 0x48], xmm0
0x1404db6e3 mov byte ptr [rbp + 0x1487], 0
0x1404db6ea lea rcx, [rbp + 0xc30]
0x1404db6f1 call 0x1404d7d50
0x1404db6f6 mov r8d, 0xc8
0x1404db6fc lea rcx, [rbp + 0x600]
0x1404db703 lea rdx, [rbp + 0xc30]
0x1404db70a call 0x142923c00
0x1404db70f mov byte ptr [rbp + 0x1476], 1
0x1404db716 mov rcx, qword ptr [rbp + 0x1488]
0x1404db71d movabs rdx, 0xfe830c34a6496103
0x1404db727 movabs r8, 0xd66ebacc5f29088d
0x1404db731 call 0x1400dd6f0
0x1404db736 test rax, rax
0x1404db739 je 0x1404df0ff
0x1404db73f cmp qword ptr [r12 + 0x248], rdx
0x1404db747 jbe 0x1404df0ff
0x1404db74d mov rax, qword ptr [r12 + 0x240]
0x1404db755 mov rcx, qword ptr [rax + rdx*8]
0x1404db759 test rcx, rcx
0x1404db75c je 0x1404df0ff
0x1404db762 mov rax, qword ptr [r12 + 0x210]
0x1404db76a not rcx
0x1404db76d lea rcx, [rcx + rcx*2]
0x1404db771 shl rcx, 5
0x1404db775 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db77b je 0x1404df0ff
0x1404db781 add rax, rcx
0x1404db784 mov rdx, qword ptr [rax + 0x10]
0x1404db788 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db790 mov dword ptr [rax + 0x5c], ecx
0x1404db793 xor eax, eax
0x1404db795 mov qword ptr [rbp + 0x1498], rdx
0x1404db79c cmp rax, qword ptr [rdx]
0x1404db79f jo 0x1404db7ad
0x1404db7a1 mov rcx, qword ptr [rbp + 0x1498]
0x1404db7a8 call 0x1405fdb80
0x1404db7ad mov r8d, 0xc8
0x1404db7b3 mov rcx, qword ptr [rbp + 0x1498]
0x1404db7ba lea rdx, [rbp + 0x600]
0x1404db7c1 call 0x142923c00
0x1404db7c6 xor esi, esi
0x1404db7c8 cmp qword ptr [r12 + 0xb8], 0
0x1404db7d1 jne 0x1404dbd87
0x1404db7d7 jmp 0x1404dbe2e
0x1404db7dc mov rax, qword ptr [rbp + 0x12a8]
0x1404db7e3 mov qword ptr [rbp + 0x1498], rax
0x1404db7ea mov rax, qword ptr [rbp + 0x12b0]
0x1404db7f1 mov qword ptr [rbp + 0x1468], rax
0x1404db7f8 movupd xmm0, xmmword ptr [r13 + 0x16]
0x1404db7fe movapd xmmword ptr [rbp + 0x5f0], xmm0
0x1404db806 mov byte ptr [rbp + 0x1475], 1
0x1404db80d mov rcx, qword ptr [rbp + 0x1488]
0x1404db814 movabs rdx, 0x9ec27b7f9851aea
0x1404db81e movabs r8, 0x1e73f089b9e6883e
0x1404db828 call 0x1400dd6f0
0x1404db82d test rax, rax
0x1404db830 je 0x1404df037
0x1404db836 cmp qword ptr [r12 + 0x248], rdx
0x1404db83e jbe 0x1404df037
0x1404db844 mov rax, qword ptr [r12 + 0x240]
0x1404db84c mov rcx, qword ptr [rax + rdx*8]
0x1404db850 test rcx, rcx
0x1404db853 je 0x1404df037
0x1404db859 mov rax, qword ptr [r12 + 0x210]
0x1404db861 not rcx
0x1404db864 lea rcx, [rcx + rcx*2]
0x1404db868 shl rcx, 5
0x1404db86c cmp qword ptr [rax + rcx + 0x20], 0
0x1404db872 je 0x1404df037
0x1404db878 add rax, rcx
0x1404db87b mov rsi, qword ptr [rax + 0x10]
0x1404db87f mov ecx, dword ptr [r12 + 0x5e0]
0x1404db887 mov dword ptr [rax + 0x5c], ecx
0x1404db88a mov rdi, qword ptr [rsi + 0x30]
0x1404db88e mov rbx, qword ptr [rsi + 0x40]
0x1404db892 cmp rdi, qword ptr [rsi + 0x20]
0x1404db896 jne 0x1404db8a8
0x1404db898 lea rcx, [rsi + 0x20]
0x1404db89c lea rdx, [rip + 0x2517885]
0x1404db8a3 call 0x140562ab0
0x1404db8a8 mov rax, qword ptr [rsi + 0x28]
0x1404db8ac lea rcx, [rdi + rdi*4]
0x1404db8b0 mov rdx, qword ptr [rbp + 0x1498]
0x1404db8b7 mov qword ptr [rax + rcx*8], rdx
0x1404db8bb mov rdx, qword ptr [rbp + 0x1468]
0x1404db8c2 mov qword ptr [rax + rcx*8 + 8], rdx
0x1404db8c7 movapd xmm0, xmmword ptr [rbp + 0x5f0]
0x1404db8cf movupd xmmword ptr [rax + rcx*8 + 0x10], xmm0
0x1404db8d5 mov qword ptr [rax + rcx*8 + 0x20], rbx
0x1404db8da jmp 0x1404dab80
0x1404db8df mov sil, byte ptr [rbp + 0x12a2]
0x1404db8e6 mov byte ptr [rbp + 0x14a7], 1
0x1404db8ed mov byte ptr [rbp + 0x14a6], 1
0x1404db8f4 mov byte ptr [rbp + 0x14a5], 1
0x1404db8fb mov rcx, qword ptr [rbp + 0x1488]
0x1404db902 movabs rdx, 0x2ed189581b92decf
0x1404db90c movabs r8, 0x15d3d99396065f38
0x1404db916 call 0x1400dd6f0
0x1404db91b test rax, rax
0x1404db91e je 0x1404df13d
0x1404db924 cmp qword ptr [r12 + 0x248], rdx
0x1404db92c jbe 0x1404df13d
0x1404db932 mov rax, qword ptr [r12 + 0x240]
0x1404db93a mov rcx, qword ptr [rax + rdx*8]
0x1404db93e test rcx, rcx
0x1404db941 je 0x1404df13d
0x1404db947 mov rax, qword ptr [r12 + 0x210]
0x1404db94f not rcx
0x1404db952 lea rcx, [rcx + rcx*2]
0x1404db956 shl rcx, 5
0x1404db95a cmp qword ptr [rax + rcx + 0x20], 0
0x1404db960 je 0x1404df13d
0x1404db966 add rax, rcx
0x1404db969 mov rax, qword ptr [rax + 0x10]
0x1404db96d mov rdx, qword ptr [rax]
0x1404db970 add rdx, 0x10
0x1404db974 mov byte ptr [rbp + 0x14a7], 1
0x1404db97b mov byte ptr [rbp + 0x14a6], 1
0x1404db982 mov byte ptr [rbp + 0x14a5], 1
0x1404db989 mov rcx, qword ptr [rbp + 0x1420]
0x1404db990 mov r8d, esi
0x1404db993 call 0x140c0e6b0
0x1404db998 mov byte ptr [rbp + 0x14a7], 1
0x1404db99f mov byte ptr [rbp + 0x14a6], 1
0x1404db9a6 mov byte ptr [rbp + 0x14a5], 1
0x1404db9ad mov rcx, qword ptr [rbp + 0x1488]
0x1404db9b4 movabs rdx, 0xbb7c1673aee764cb
0x1404db9be movabs r8, 0x250aee383945e2d6
0x1404db9c8 call 0x1400dd6f0
0x1404db9cd test rax, rax
0x1404db9d0 je 0x1404df117
0x1404db9d6 cmp qword ptr [r12 + 0x248], rdx
0x1404db9de jbe 0x1404df117
0x1404db9e4 mov rax, qword ptr [r12 + 0x240]
0x1404db9ec mov rcx, qword ptr [rax + rdx*8]
0x1404db9f0 test rcx, rcx
0x1404db9f3 je 0x1404df117
0x1404db9f9 mov rax, qword ptr [r12 + 0x210]
0x1404dba01 not rcx
0x1404dba04 lea rcx, [rcx + rcx*2]
0x1404dba08 shl rcx, 5
0x1404dba0c cmp qword ptr [rax + rcx + 0x20], 0
0x1404dba12 je 0x1404df117
0x1404dba18 add rax, rcx
0x1404dba1b mov rdi, qword ptr [rax + 0x10]
0x1404dba1f mov ecx, dword ptr [r12 + 0x5e0]
0x1404dba27 mov dword ptr [rax + 0x5c], ecx
0x1404dba2a mov rbx, qword ptr [rdi + 0x30]
0x1404dba2e mov r14, qword ptr [rdi + 0x40]
0x1404dba32 cmp rbx, qword ptr [rdi + 0x20]
0x1404dba36 jne 0x1404dba5d
0x1404dba38 lea rcx, [rdi + 0x20]
0x1404dba3c mov byte ptr [rbp + 0x14a7], 1
0x1404dba43 mov byte ptr [rbp + 0x14a6], 1
0x1404dba4a mov byte ptr [rbp + 0x14a5], 1
0x1404dba51 lea rdx, [rip + 0x25176d0]
0x1404dba58 call 0x140562630
0x1404dba5d mov rax, qword ptr [rdi + 0x28]
0x1404dba61 mov rcx, rbx
0x1404dba64 shl rcx, 4
0x1404dba68 mov qword ptr [rax + rcx], r14
0x1404dba6c mov byte ptr [rax + rcx + 8], sil
0x1404dba71 jmp 0x1404dc086
0x1404dba76 mov bl, byte ptr [rbp + 0x12a6]
0x1404dba7c movss xmm6, dword ptr [rbp + 0x12a2]
0x1404dba84 mov byte ptr [rbp + 0x14a7], 1
0x1404dba8b mov byte ptr [rbp + 0x14a6], 1
0x1404dba92 mov byte ptr [rbp + 0x14a5], 1
0x1404dba99 mov rcx, qword ptr [rbp + 0x1488]
0x1404dbaa0 movabs rdx, 0xe71eaa88577fee7a
0x1404dbaaa movabs r8, 0x91a5d4b07c2c8899
0x1404dbab4 call 0x1400dd6f0
0x1404dbab9 test rax, rax
0x1404dbabc je 0x1404df09b
0x1404dbac2 cmp qword ptr [r12 + 0x248], rdx
0x1404dbaca jbe 0x1404df09b
0x1404dbad0 mov rax, qword ptr [r12 + 0x240]
0x1404dbad8 mov rcx, qword ptr [rax + rdx*8]
0x1404dbadc test rcx, rcx
0x1404dbadf je 0x1404df09b
0x1404dbae5 mov rax, qword ptr [r12 + 0x210]
0x1404dbaed not rcx
0x1404dbaf0 lea rcx, [rcx + rcx*2]
0x1404dbaf4 shl rcx, 5
0x1404dbaf8 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dbafe je 0x1404df09b
0x1404dbb04 add rax, rcx
0x1404dbb07 mov rsi, qword ptr [rax + 0x10]
0x1404dbb0b mov ecx, dword ptr [r12 + 0x5e0]
0x1404dbb13 mov dword ptr [rax + 0x5c], ecx
0x1404dbb16 mov rdi, qword ptr [rsi + 0x30]
0x1404dbb1a mov r14, qword ptr [rsi + 0x40]
0x1404dbb1e cmp rdi, qword ptr [rsi + 0x20]
0x1404dbb22 jne 0x1404dbb49
0x1404dbb24 lea rcx, [rsi + 0x20]
0x1404dbb28 mov byte ptr [rbp + 0x14a7], 1
0x1404dbb2f mov byte ptr [rbp + 0x14a6], 1
0x1404dbb36 mov byte ptr [rbp + 0x14a5], 1
0x1404dbb3d lea rdx, [rip + 0x25175e4]
0x1404dbb44 call 0x140562db0
0x1404dbb49 mov rax, qword ptr [rsi + 0x28]
0x1404dbb4d lea rcx, [rdi + rdi*2]
0x1404dbb51 mov qword ptr [rax + rcx*8], r14
0x1404dbb55 movabs rdx, 0x1ffffffff
0x1404dbb5f mov qword ptr [rax + rcx*8 + 8], rdx
0x1404dbb64 movss dword ptr [rax + rcx*8 + 0x10], xmm6
0x1404dbb6a mov byte ptr [rax + rcx*8 + 0x14], bl
0x1404dbb6e jmp 0x1404dab80
0x1404dbb73 mov rax, qword ptr [r12 + 0x1100]
0x1404dbb7b cmp rax, qword ptr [r12 + 0x10f8]
0x1404dbb83 jb 0x1404dab8b
0x1404dbb89 movupd xmm6, xmmword ptr [rbp + 0x12a8]
0x1404dbb91 mov byte ptr [rbp + 0x14a7], 1
0x1404dbb98 mov byte ptr [rbp + 0x14a6], 1
0x1404dbb9f mov byte ptr [rbp + 0x14a5], 1
0x1404dbba6 mov rcx, qword ptr [rbp + 0x1488]
0x1404dbbad movabs rdx, 0x923f1ac331d52636
0x1404dbbb7 movabs r8, 0x4114c9eaff932cf6
0x1404dbbc1 call 0x1400dd6f0
0x1404dbbc6 test rax, rax
0x1404dbbc9 je 0x1404df2ca
0x1404dbbcf cmp qword ptr [r12 + 0x248], rdx
0x1404dbbd7 jbe 0x1404df2ca
0x1404dbbdd mov rax, qword ptr [r12 + 0x240]
0x1404dbbe5 mov rcx, qword ptr [rax + rdx*8]
0x1404dbbe9 test rcx, rcx
0x1404dbbec je 0x1404df2ca
0x1404dbbf2 mov rax, qword ptr [r12 + 0x210]
0x1404dbbfa not rcx
0x1404dbbfd lea rcx, [rcx + rcx*2]
0x1404dbc01 shl rcx, 5
0x1404dbc05 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dbc0b je 0x1404df2ca
0x1404dbc11 add rax, rcx
0x1404dbc14 mov rsi, qword ptr [rax + 0x10]
0x1404dbc18 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dbc20 mov dword ptr [rax + 0x5c], ecx
0x1404dbc23 mov rdi, qword ptr [rsi + 0x30]
0x1404dbc27 mov rbx, qword ptr [rsi + 0x40]
0x1404dbc2b cmp rdi, qword ptr [rsi + 0x20]
0x1404dbc2f jne 0x1404dbc56
0x1404dbc31 lea rcx, [rsi + 0x20]
0x1404dbc35 mov byte ptr [rbp + 0x14a7], 1
0x1404dbc3c mov byte ptr [rbp + 0x14a6], 1
0x1404dbc43 mov byte ptr [rbp + 0x14a5], 1
0x1404dbc4a lea rdx, [rip + 0x25174d7]
0x1404dbc51 call 0x140562db0
0x1404dbc56 cvtpd2ps xmm0, xmm6
0x1404dbc5a mov rax, qword ptr [rsi + 0x28]
0x1404dbc5e lea rcx, [rdi + rdi*2]
0x1404dbc62 mov qword ptr [rax + rcx*8], rbx
0x1404dbc66 mov dword ptr [rax + rcx*8 + 8], 1
0x1404dbc6e movlpd qword ptr [rax + rcx*8 + 0xc], xmm0
0x1404dbc74 jmp 0x1404dab80
0x1404dbc79 mov byte ptr [rbx + 0x7c], 1
0x1404dbc7d xor eax, eax
0x1404dbc7f mov cl, 1
0x1404dbc81 lock cmpxchg byte ptr [rip + 0x3158f08], cl
0x1404dbc89 jne 0x1404dc6cd
0x1404dbc8f mov rax, qword ptr [rip + 0x315078a]
0x1404dbc96 mov rax, qword ptr [rax]
0x1404dbc99 shl rax, 1
0x1404dbc9c test rax, rax
0x1404dbc9f jne 0x1404dc6f3
0x1404dbca5 xor eax, eax
0x1404dbca7 movzx ecx, byte ptr [rip + 0x3158ee4]
0x1404dbcae test cl, cl
0x1404dbcb0 jne 0x1404df4bb
0x1404dbcb6 or byte ptr [rip + 0x3158ed6], 1
0x1404dbcbd test al, al
0x1404dbcbf jne 0x1404dbcd7
0x1404dbcc1 mov rax, qword ptr [rip + 0x3150758]
0x1404dbcc8 mov rax, qword ptr [rax]
0x1404dbccb shl rax, 1
0x1404dbcce test rax, rax
0x1404dbcd1 jne 0x1404dc7d7
0x1404dbcd7 xor eax, eax
0x1404dbcd9 xchg byte ptr [rip + 0x3158eb2], al
0x1404dbcdf cmp al, 2
0x1404dbce1 jne 0x1404dab8b
0x1404dbce7 mov byte ptr [rbp + 0x14a7], 1
0x1404dbcee mov byte ptr [rbp + 0x14a6], 1
0x1404dbcf5 mov byte ptr [rbp + 0x14a5], 1
0x1404dbcfc lea rcx, [rip + 0x3158e8e]
0x1404dbd03 call 0x1428d8810
0x1404dbd08 jmp 0x1404dab8b
0x1404dbd0d movabs rax, 0x8000000000000001
0x1404dbd17 mov qword ptr [rbp + 0xc30], rax
0x1404dbd1e mov dword ptr [rdi + 0x5c], ebx
0x1404dbd21 mov rdx, qword ptr [rsi + 0x38]
0x1404dbd25 movabs rax, 0x8000000000000001
0x1404dbd2f cmp rdx, rax
0x1404dbd32 je 0x1404dbd4d
0x1404dbd34 mov rax, rdx
0x1404dbd37 neg rax
0x1404dbd3a jo 0x1404dbd4d
0x1404dbd3c jae 0x1404dbd4d
0x1404dbd3e mov rcx, qword ptr [rsi + 0x40]
0x1404dbd42 mov r8d, 1
0x1404dbd48 call 0x140613c20
0x1404dbd4d add rsi, 0x38
0x1404dbd51 mov rax, qword ptr [rbp + 0xc50]
0x1404dbd58 mov qword ptr [rsi + 0x20], rax
0x1404dbd5c movupd xmm0, xmmword ptr [rbp + 0xc30]
0x1404dbd64 movdqu xmm1, xmmword ptr [rbp + 0xc40]
0x1404dbd6c movdqu xmmword ptr [rsi + 0x10], xmm1
0x1404dbd71 movupd xmmword ptr [rsi], xmm0
0x1404dbd75 mov sil, 1
0x1404dbd78 cmp qword ptr [r12 + 0xb8], 0
0x1404dbd81 je 0x1404dbe2e
0x1404dbd87 mov rax, qword ptr [r12 + 0xa0]
0x1404dbd8f mov rcx, qword ptr [r12 + 0xa8]
0x1404dbd97 mov rdx, rcx
0x1404dbd9a movabs r8, 0x29b993a986bb8e9d
0x1404dbda4 and rdx, r8
0x1404dbda7 xor r8d, r8d
0x1404dbdaa movdqu xmm0, xmmword ptr [rax + rdx]
0x1404dbdaf movdqa xmm1, xmm0
0x1404dbdb3 pcmpeqb xmm1, xmm9
0x1404dbdb8 pmovmskb r9d, xmm1
0x1404dbdbd test r9d, r9d
0x1404dbdc0 jne 0x1404dbded
0x1404dbdc2 pcmpeqb xmm0, xmm7
0x1404dbdc6 pmovmskb r9d, xmm0
0x1404dbdcb test r9d, r9d
0x1404dbdce jne 0x1404dbe2e
0x1404dbdd0 add rdx, r8
0x1404dbdd3 add rdx, 0x10
0x1404dbdd7 add r8, 0x10
0x1404dbddb and rdx, rcx
0x1404dbdde jmp 0x1404dbdaa
0x1404dbde0 lea r10d, [r9 - 1]
0x1404dbde4 and r10w, r9w
0x1404dbde8 mov r9d, r10d
0x1404dbdeb je 0x1404dbdc2
0x1404dbded tzcnt r10d, r9d
0x1404dbdf2 add r10, rdx
0x1404dbdf5 and r10, rcx
0x1404dbdf8 neg r10
0x1404dbdfb lea r10, [r10 + r10*2]
0x1404dbdff movabs r11, 0xa554fbb3bbf1ca0
0x1404dbe09 cmp qword ptr [rax + r10*8 - 0x18], r11
0x1404dbe0e jne 0x1404dbde0
0x1404dbe10 movabs r11, 0x29b993a986bb8e9d
0x1404dbe1a cmp qword ptr [rax + r10*8 - 0x10], r11
0x1404dbe1f jne 0x1404dbde0
0x1404dbe21 lea rax, [rax + r10*8]
0x1404dbe25 mov rbx, qword ptr [rax - 8]
0x1404dbe29 jmp 0x1404dbff7
0x1404dbe2e mov rax, qword ptr [rbp + 0x13d8]
0x1404dbe35 mov eax, dword ptr [rax]
0x1404dbe37 cmp eax, 0x3ffffffd
0x1404dbe3c ja 0x1404dc658
0x1404dbe42 lea ecx, [rax + 1]
0x1404dbe45 mov rdx, qword ptr [rbp + 0x13d8]
0x1404dbe4c lock cmpxchg dword ptr [rdx], ecx
0x1404dbe50 jne 0x1404dc658
0x1404dbe56 movzx eax, byte ptr [r12 + 0xc8]
0x1404dbe5f mov dil, 1
0x1404dbe62 cmp qword ptr [r12 + 0x120], 0
0x1404dbe6b je 0x1404dbfcd
0x1404dbe71 mov rax, qword ptr [r12 + 0x108]
0x1404dbe79 mov rcx, qword ptr [r12 + 0x110]
0x1404dbe81 mov rdx, rcx
0x1404dbe84 movabs r8, 0x29b993a986bb8e9d
0x1404dbe8e and rdx, r8
0x1404dbe91 xor r8d, r8d
0x1404dbe94 movdqu xmm0, xmmword ptr [rax + rdx]
0x1404dbe99 movdqa xmm1, xmm0
0x1404dbe9d pcmpeqb xmm1, xmm9
0x1404dbea2 pmovmskb r9d, xmm1
0x1404dbea7 test r9d, r9d
0x1404dbeaa jne 0x1404dbedd
0x1404dbeac pcmpeqb xmm0, xmm7
0x1404dbeb0 pmovmskb r9d, xmm0
0x1404dbeb5 test r9d, r9d
0x1404dbeb8 jne 0x1404dbfcd
0x1404dbebe add rdx, r8
0x1404dbec1 add rdx, 0x10
0x1404dbec5 add r8, 0x10
0x1404dbec9 and rdx, rcx
0x1404dbecc jmp 0x1404dbe94
0x1404dbece nop 
0x1404dbed0 lea r10d, [r9 - 1]
0x1404dbed4 and r10w, r9w
0x1404dbed8 mov r9d, r10d
0x1404dbedb je 0x1404dbeac
0x1404dbedd tzcnt r11d, r9d
0x1404dbee2 add r11, rdx
0x1404dbee5 and r11, rcx
0x1404dbee8 mov r10, r11
0x1404dbeeb neg r10
0x1404dbeee shl r11, 3
0x1404dbef2 shl r10, 7
0x1404dbef6 sub r10, r11
0x1404dbef9 movabs r11, 0xa554fbb3bbf1ca0
0x1404dbf03 cmp qword ptr [rax + r10 - 0x88], r11
0x1404dbf0b jne 0x1404dbed0
0x1404dbf0d movabs r11, 0x29b993a986bb8e9d
0x1404dbf17 cmp qword ptr [rax + r10 - 0x80], r11
0x1404dbf1c jne 0x1404dbed0
0x1404dbf1e add rax, r10
0x1404dbf21 mov rbx, qword ptr [rax - 8]
0x1404dbf25 xor edi, edi
0x1404dbf27 jmp 0x1404dbfcd
0x1404dbf2c cmp word ptr [rbp + 0xc30], 0
0x1404dbf34 jne 0x1404db3f2
0x1404dbf3a cmp word ptr [rbp + 0xc32], 0x21
0x1404dbf42 jne 0x1404db3f2
0x1404dbf48 cmp byte ptr [rbp + 0xca8], 0
0x1404dbf4f jne 0x1404dc25a
0x1404dbf55 cmp byte ptr [r12 + 0x1114], 1
0x1404dbf5e jne 0x1404dc25a
0x1404dbf64 mov byte ptr [rbp + 0x14a2], 1
0x1404dbf6b lea rcx, [rbp + 0x600]
0x1404dbf72 mov rdx, qword ptr [rbp + 0x13a8]
0x1404dbf79 call 0x14062eb30
0x1404dbf7e mov rdx, qword ptr [rbp + 0x608]
0x1404dbf85 cmp dword ptr [rbp + 0x600], 1
0x1404dbf8c jne 0x1404dc252
0x1404dbf92 mov al, 1
0x1404dbf94 mov dword ptr [rbp + 0x1498], eax
0x1404dbf9a movabs rax, 0x8000000000000004
0x1404dbfa4 cmp rdx, rax
0x1404dbfa7 jl 0x1404dc5bb
0x1404dbfad test rdx, rdx
0x1404dbfb0 je 0x1404dc5bb
0x1404dbfb6 mov rcx, qword ptr [rbp + 0x610]
0x1404dbfbd mov r8d, 1
0x1404dbfc3 call 0x140613c20
0x1404dbfc8 jmp 0x1404dc5bb
0x1404dbfcd mov edx, 0xffffffff
0x1404dbfd2 mov rax, qword ptr [rbp + 0x13d8]
0x1404dbfd9 lock xadd dword ptr [rax], edx
0x1404dbfdd dec edx
0x1404dbfdf mov eax, edx
0x1404dbfe1 and eax, 0xbfffffff
0x1404dbfe6 neg eax
0x1404dbfe8 jo 0x1404dc785
0x1404dbfee test dil, dil
0x1404dbff1 jne 0x1404def82
0x1404dbff7 cmp qword ptr [r12 + 0x248], rbx
0x1404dbfff jbe 0x1404def82
0x1404dc005 mov rax, qword ptr [r12 + 0x240]
0x1404dc00d mov rcx, qword ptr [rax + rbx*8]
0x1404dc011 test rcx, rcx
0x1404dc014 je 0x1404def82
0x1404dc01a mov rax, qword ptr [r12 + 0x210]
0x1404dc022 not rcx
0x1404dc025 lea rcx, [rcx + rcx*2]
0x1404dc029 shl rcx, 5
0x1404dc02d cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc033 je 0x1404def82
0x1404dc039 add rax, rcx
0x1404dc03c mov rdi, qword ptr [rax + 0x10]
0x1404dc040 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dc048 mov dword ptr [rax + 0x5c], ecx
0x1404dc04b mov rbx, qword ptr [rdi + 0x30]
0x1404dc04f mov r14, qword ptr [rdi + 0x40]
0x1404dc053 cmp rbx, qword ptr [rdi + 0x20]
0x1404dc057 jne 0x1404dc07e
0x1404dc059 lea rcx, [rdi + 0x20]
0x1404dc05d mov byte ptr [rbp + 0x14a7], sil
0x1404dc064 mov byte ptr [rbp + 0x14a6], 1
0x1404dc06b mov byte ptr [rbp + 0x14a5], 1
0x1404dc072 lea rdx, [rip + 0x25170af]
0x1404dc079 call 0x140562870
0x1404dc07e mov rax, qword ptr [rdi + 0x28]
0x1404dc082 mov qword ptr [rax + rbx*8], r14
0x1404dc086 inc rbx
0x1404dc089 mov qword ptr [rdi + 0x30], rbx
0x1404dc08d inc qword ptr [rdi + 0x40]
0x1404dc091 jmp 0x1404dab8b
0x1404dc096 xor esi, esi
0x1404dc098 test dl, 1
0x1404dc09b je 0x1404daf89
0x1404dc0a1 jmp 0x1404daf9f
0x1404dc0a6 mov rdx, qword ptr [rbp + 0xc98]
0x1404dc0ad lock inc qword ptr [rdx]
0x1404dc0b1 jle 0x1404df65a
0x1404dc0b7 mov r8b, 0x18
0x1404dc0ba jmp 0x1404dc0c6
0x1404dc0bc mov r8b, 0x1a
0x1404dc0bf mov rdx, qword ptr [rbp + 0xc98]
0x1404dc0c6 mov r9, qword ptr [rbp + 0xca0]
0x1404dc0cd mov r10d, dword ptr [rbp + 0x1430]
0x1404dc0d4 mov r11d, dword ptr [rbp + 0x1433]
0x1404dc0db mov dword ptr [rbp + 0x1458], r10d
0x1404dc0e2 mov dword ptr [rbp + 0x145b], r11d
0x1404dc0e9 mov r10d, r8d
0x1404dc0ec shr ebx, 2
0x1404dc0ef lea r8d, [rbx + 4]
0x1404dc0f3 test sil, 0x20
0x1404dc0f7 mov r11d, dword ptr [rbp + 0x608]
0x1404dc0fe lea rdi, [rbp]
0x1404dc102 mov dword ptr [rdi + 8], r11d
0x1404dc106 mov r11, qword ptr [rbp + 0x600]
0x1404dc10d mov qword ptr [rdi], r11
0x1404dc110 mov dword ptr [rbp - 0x40], 0xb3
0x1404dc117 mov byte ptr [rbp + 0xc], cl
0x1404dc11a mov byte ptr [rbp - 0x20], r10b
0x1404dc11e mov ecx, dword ptr [rbp + 0x1458]
0x1404dc124 mov r10d, dword ptr [rbp + 0x145b]
0x1404dc12b mov dword ptr [rdi - 0x1c], r10d
0x1404dc12f mov dword ptr [rdi - 0x1f], ecx
0x1404dc132 mov qword ptr [rbp - 0x18], rdx
0x1404dc136 mov qword ptr [rbp - 0x10], r9
0x1404dc13a mov byte ptr [rbp + 0xd], al
0x1404dc13d movabs rax, 0x1ffffffff
0x1404dc147 mov qword ptr [rbp - 8], rax
0x1404dc14b movzx edi, r8b
0x1404dc14f cmove edi, ebx
0x1404dc152 mov byte ptr [rbp + 0x1496], 1
0x1404dc159 mov byte ptr [rbp + 0x1495], 1
0x1404dc160 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc167 movabs rdx, 0xa6181a92acb1ad79
0x1404dc171 movabs r8, 0x6d58b6c9a3407ce9
0x1404dc17b call 0x1400dd6f0
0x1404dc180 mov bl, 1
0x1404dc182 test rax, rax
0x1404dc185 je 0x1404dc46c
0x1404dc18b cmp qword ptr [r12 + 0x248], rdx
0x1404dc193 jbe 0x1404dc46c
0x1404dc199 mov rax, qword ptr [r12 + 0x240]
0x1404dc1a1 mov rcx, qword ptr [rax + rdx*8]
0x1404dc1a5 test rcx, rcx
0x1404dc1a8 je 0x1404dc46c
0x1404dc1ae mov rax, qword ptr [r12 + 0x210]
0x1404dc1b6 not rcx
0x1404dc1b9 lea rcx, [rcx + rcx*2]
0x1404dc1bd shl rcx, 5
0x1404dc1c1 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc1c7 je 0x1404dc46c
0x1404dc1cd add rax, rcx
0x1404dc1d0 mov rcx, qword ptr [rax + 0x10]
0x1404dc1d4 mov edx, dword ptr [r12 + 0x5e0]
0x1404dc1dc mov dword ptr [rax + 0x5c], edx
0x1404dc1df mov ebx, dword ptr [rbp - 0x40]
0x1404dc1e2 lea edx, [rbx - 3]
0x1404dc1e5 cmp edx, 0x132
0x1404dc1eb jae 0x1404dc67e
0x1404dc1f1 movzx r9d, byte ptr [rbp - 0x20]
0x1404dc1f6 movzx edx, byte ptr [rbp + 0xc]
0x1404dc1fa mov r11b, 0x1b
0x1404dc1fd cmp r9b, 0x1b
0x1404dc201 je 0x1404dc386
0x1404dc207 lea r8d, [r9 - 0x18]
0x1404dc20b cmp r8b, 3
0x1404dc20f movzx r8d, r8b
0x1404dc213 mov eax, 1
0x1404dc218 cmovae r8d, eax
0x1404dc21c test r8b, r8b
0x1404dc21f je 0x1404dc365
0x1404dc225 movzx r8d, r8b
0x1404dc229 cmp r8d, 1
0x1404dc22d jne 0x1404dc378
0x1404dc233 lea r10, [rbp]
0x1404dc237 mov r8d, dword ptr [r10 - 0x1f]
0x1404dc23b mov r10d, dword ptr [r10 - 0x1c]
0x1404dc23f mov dword ptr [rbp + 0x145b], r10d
0x1404dc246 mov dword ptr [rbp + 0x1458], r8d
0x1404dc24d jmp 0x1404dc37b
0x1404dc252 mov rax, rdx
0x1404dc255 neg rax
0x1404dc258 jno 0x1404dc267
0x1404dc25a mov al, 1
0x1404dc25c mov dword ptr [rbp + 0x1498], eax
0x1404dc262 jmp 0x1404dc5bb
0x1404dc267 mov qword ptr [rbp + 0x1468], rdx
0x1404dc26e mov rax, qword ptr [rbp + 0x610]
0x1404dc275 mov qword ptr [rbp + 0x1460], rax
0x1404dc27c mov rsi, qword ptr [rbp + 0x618]
0x1404dc283 mov byte ptr [rbp + 0x1473], 1
0x1404dc28a mov rcx, qword ptr [rbp + 0x1488]
0x1404dc291 movabs rdx, 0xa6181a92acb1ad79
0x1404dc29b movabs r8, 0x6d58b6c9a3407ce9
0x1404dc2a5 call 0x1400dd6f0
0x1404dc2aa test rax, rax
0x1404dc2ad je 0x1404dc336
0x1404dc2b3 cmp qword ptr [r12 + 0x248], rdx
0x1404dc2bb jbe 0x1404dc336
0x1404dc2bd mov rax, qword ptr [r12 + 0x240]
0x1404dc2c5 mov rcx, qword ptr [rax + rdx*8]
0x1404dc2c9 test rcx, rcx
0x1404dc2cc je 0x1404dc336
0x1404dc2ce mov rax, qword ptr [r12 + 0x210]
0x1404dc2d6 not rcx
0x1404dc2d9 lea rcx, [rcx + rcx*2]
0x1404dc2dd shl rcx, 5
0x1404dc2e1 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc2e7 je 0x1404dc336
0x1404dc2e9 add rax, rcx
0x1404dc2ec mov rcx, qword ptr [rax + 0x10]
0x1404dc2f0 mov edx, dword ptr [r12 + 0x5e0]
0x1404dc2f8 mov dword ptr [rax + 0x5c], edx
0x1404dc2fb mov rax, qword ptr [rbp + 0x1468]
0x1404dc302 mov qword ptr [rbp + 0x600], rax
0x1404dc309 mov rax, qword ptr [rbp + 0x1460]
0x1404dc310 mov qword ptr [rbp + 0x608], rax
0x1404dc317 mov qword ptr [rbp + 0x610], rsi
0x1404dc31e mov byte ptr [rbp + 0x1473], 0
0x1404dc325 lea rdx, [rbp + 0x600]
0x1404dc32c call 0x1406860d0
0x1404dc331 jmp 0x1404dc25a
0x1404dc336 mov al, 1
0x1404dc338 mov dword ptr [rbp + 0x1498], eax
0x1404dc33e mov rdx, qword ptr [rbp + 0x1468]
0x1404dc345 test rdx, rdx
0x1404dc348 je 0x1404dc5bb
0x1404dc34e mov r8d, 1
0x1404dc354 mov rcx, qword ptr [rbp + 0x1460]
0x1404dc35b call 0x140613c20
0x1404dc360 jmp 0x1404dc5bb
0x1404dc365 mov r8, qword ptr [rbp - 0x18]
0x1404dc369 lock inc qword ptr [r8]
0x1404dc36d jle 0x1404df65a
0x1404dc373 mov r9b, 0x18
0x1404dc376 jmp 0x1404dc37f
0x1404dc378 mov r9b, 0x1a
0x1404dc37b mov r8, qword ptr [rbp - 0x18]
0x1404dc37f mov r10, qword ptr [rbp - 0x10]
0x1404dc383 mov r11d, r9d
0x1404dc386 mov r9d, esi
0x1404dc389 shr r9d, 5
0x1404dc38d and r9b, 8
0x1404dc391 shr esi, 7
0x1404dc394 and sil, 0x10
0x1404dc398 or sil, r9b
0x1404dc39b or sil, dil
0x1404dc39e movzx r9d, byte ptr [rbp + 0xd]
0x1404dc3a3 mov rdi, qword ptr [rbp - 8]
0x1404dc3a7 lea rax, [rbp]
0x1404dc3ab mov r14d, dword ptr [rax + 8]
0x1404dc3af lea r15, [rbp + 0x638]
0x1404dc3b6 mov dword ptr [r15 + 0x10], r14d
0x1404dc3ba mov r14, qword ptr [rax]
0x1404dc3bd mov qword ptr [r15 + 8], r14
0x1404dc3c1 mov dword ptr [rbp + 0x600], ebx
0x1404dc3c7 mov eax, dword ptr [rbp + 0x1498]
0x1404dc3cd mov dword ptr [rbp + 0x604], eax
0x1404dc3d3 mov rax, qword ptr [rbp + 0x1440]
0x1404dc3da mov qword ptr [r15 - 0x20], rax
0x1404dc3de movups xmm0, xmmword ptr [rbp + 0x1430]
0x1404dc3e5 movups xmmword ptr [r15 - 0x30], xmm0
0x1404dc3ea mov byte ptr [rbp + 0x64c], dl
0x1404dc3f0 mov byte ptr [rbp + 0x620], r11b
0x1404dc3f7 mov eax, dword ptr [rbp + 0x1458]
0x1404dc3fd mov edx, dword ptr [rbp + 0x145b]
0x1404dc403 mov dword ptr [r15 - 0x14], edx
0x1404dc407 mov dword ptr [r15 - 0x17], eax
0x1404dc40b mov qword ptr [rbp + 0x628], r8
0x1404dc412 mov qword ptr [rbp + 0x630], r10
0x1404dc419 mov byte ptr [rbp + 0x64d], r9b
0x1404dc420 mov qword ptr [rbp + 0x638], rdi
0x1404dc427 lea rdx, [rbp + 0xc38]
0x1404dc42e mov rax, qword ptr [rdx + 0x68]
0x1404dc432 mov qword ptr [rbp + 0x1440], rax
0x1404dc439 movupd xmm0, xmmword ptr [rdx + 0x58]
0x1404dc43e movapd xmmword ptr [rbp + 0x1430], xmm0
0x1404dc446 mov byte ptr [rbp + 0x1496], 1
0x1404dc44d mov byte ptr [rbp + 0x1495], 0
0x1404dc454 lea rdx, [rbp + 0x600]
0x1404dc45b mov r8d, esi
0x1404dc45e lea r9, [rbp + 0x1430]
0x1404dc465 call 0x140685fc0
0x1404dc46a xor ebx, ebx
0x1404dc46c mov byte ptr [rbp + 0x1496], 1
0x1404dc473 mov byte ptr [rbp + 0x1495], bl
0x1404dc479 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc480 movabs rdx, 0x500f3099f5bff7ba
0x1404dc48a movabs r8, 0x85f3997014c11728
0x1404dc494 call 0x1400dd6f0
0x1404dc499 test rax, rax
0x1404dc49c je 0x1404df185
0x1404dc4a2 cmp qword ptr [r12 + 0x248], rdx
0x1404dc4aa jbe 0x1404df185
0x1404dc4b0 mov rax, qword ptr [r12 + 0x240]
0x1404dc4b8 mov rcx, qword ptr [rax + rdx*8]
0x1404dc4bc test rcx, rcx
0x1404dc4bf je 0x1404df185
0x1404dc4c5 mov rax, qword ptr [r12 + 0x210]
0x1404dc4cd not rcx
0x1404dc4d0 lea rcx, [rcx + rcx*2]
0x1404dc4d4 shl rcx, 5
0x1404dc4d8 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc4de je 0x1404df185
0x1404dc4e4 mov dword ptr [rbp + 0x1498], ebx
0x1404dc4ea add rax, rcx
0x1404dc4ed mov rsi, qword ptr [rax + 0x10]
0x1404dc4f1 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dc4f9 mov dword ptr [rax + 0x5c], ecx
0x1404dc4fc movups xmm0, xmmword ptr [rbp]
0x1404dc500 lea rax, [rbp + 0x608]
0x1404dc507 movups xmmword ptr [rax + 0x40], xmm0
0x1404dc50b movupd xmm0, xmmword ptr [rbp - 0x40]
0x1404dc510 movdqu xmm1, xmmword ptr [rbp - 0x30]
0x1404dc515 movdqu xmm2, xmmword ptr [rbp - 0x20]
0x1404dc51a movdqu xmm3, xmmword ptr [rbp - 0x10]
0x1404dc51f movdqu xmmword ptr [rax + 0x30], xmm3
0x1404dc524 movdqu xmmword ptr [rax + 0x20], xmm2
0x1404dc529 movdqu xmmword ptr [rax + 0x10], xmm1
0x1404dc52e movupd xmmword ptr [rax], xmm0
0x1404dc532 mov rax, qword ptr [rsi + 0x40]
0x1404dc536 mov qword ptr [rbp + 0x600], rax
0x1404dc53d mov rdi, qword ptr [rsi + 0x30]
0x1404dc541 cmp rdi, qword ptr [rsi + 0x20]
0x1404dc545 jne 0x1404dc557
0x1404dc547 lea rcx, [rsi + 0x20]
0x1404dc54b lea rdx, [rip + 0x2516bd6]
0x1404dc552 call 0x140562f30
0x1404dc557 mov rax, qword ptr [rsi + 0x28]
0x1404dc55b imul rcx, rdi, 0x58
0x1404dc55f movupd xmm0, xmmword ptr [rbp + 0x600]
0x1404dc567 movdqu xmm1, xmmword ptr [rbp + 0x610]
0x1404dc56f movdqu xmm2, xmmword ptr [rbp + 0x620]
0x1404dc577 movdqu xmm3, xmmword ptr [rbp + 0x630]
0x1404dc57f movdqu xmm4, xmmword ptr [rbp + 0x640]
0x1404dc587 mov rdx, qword ptr [rbp + 0x650]
0x1404dc58e mov qword ptr [rax + rcx + 0x50], rdx
0x1404dc593 movdqu xmmword ptr [rax + rcx + 0x40], xmm4
0x1404dc599 movdqu xmmword ptr [rax + rcx + 0x30], xmm3
0x1404dc59f movdqu xmmword ptr [rax + rcx + 0x20], xmm2
0x1404dc5a5 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
0x1404dc5ab movupd xmmword ptr [rax + rcx], xmm0
0x1404dc5b0 inc rdi
0x1404dc5b3 mov qword ptr [rsi + 0x30], rdi
0x1404dc5b7 inc qword ptr [rsi + 0x40]
0x1404dc5bb movzx eax, word ptr [rbp + 0xc30]
0x1404dc5c2 cmp eax, 2
0x1404dc5c5 je 0x1404dc5d5
0x1404dc5c7 cmp eax, 1
0x1404dc5ca jne 0x1404dc601
0x1404dc5cc movzx eax, byte ptr [rbp + 0xc38]
0x1404dc5d3 jmp 0x1404dc5e4
0x1404dc5d5 movzx eax, byte ptr [rbp + 0xc38]
0x1404dc5dc lea ecx, [rax - 0x1b]
0x1404dc5df cmp cl, 5
0x1404dc5e2 jb 0x1404dc601
0x1404dc5e4 cmp al, 0x18
0x1404dc5e6 jne 0x1404dc601
0x1404dc5e8 mov rax, qword ptr [rbp + 0xc40]
0x1404dc5ef lock dec qword ptr [rax]
0x1404dc5f3 jne 0x1404dc601
0x1404dc5f5 lea rcx, [rbp + 0xc40]
0x1404dc5fc call 0x14064eb30
0x1404dc601 movzx eax, byte ptr [rbp + 0xc90]
0x1404dc608 cmp al, 0x1b
0x1404dc60a setne cl
0x1404dc60d test byte ptr [rbp + 0x1498], cl
0x1404dc613 je 0x1404dc632
0x1404dc615 cmp al, 0x18
0x1404dc617 jne 0x1404dc632
0x1404dc619 mov rax, qword ptr [rbp + 0xc98]
0x1404dc620 lock dec qword ptr [rax]
0x1404dc624 jne 0x1404dc632
0x1404dc626 lea rcx, [rbp + 0xc98]
0x1404dc62d call 0x14064eb30
0x1404dc632 mov byte ptr [rbp + 0x14a7], 1
0x1404dc639 mov byte ptr [rbp + 0x14a6], 0
0x1404dc640 mov byte ptr [rbp + 0x14a5], 1
0x1404dc647 lea rcx, [rbp + 0xc50]
0x1404dc64e call 0x1405fa110
0x1404dc653 jmp 0x1404dab8b
0x1404dc658 mov byte ptr [rbp + 0x14a7], sil
0x1404dc65f mov byte ptr [rbp + 0x14a6], 1
0x1404dc666 mov byte ptr [rbp + 0x14a5], 1
0x1404dc66d mov rcx, qword ptr [rbp + 0x13d8]
0x1404dc674 call 0x1428d8a30
0x1404dc679 jmp 0x1404dbe56
0x1404dc67e cmp ebx, 2
0x1404dc681 je 0x1404dc723
0x1404dc687 cmp ebx, 1
0x1404dc68a jne 0x1404dc731
0x1404dc690 mov r14, rcx
0x1404dc693 lea rcx, [rbp + 0x1430]
0x1404dc69a lea rdx, [rbp - 0x38]
0x1404dc69e call 0x1404d7c70
0x1404dc6a3 mov rcx, r14
0x1404dc6a6 jmp 0x1404dc1f1
0x1404dc6ab mov byte ptr [rbp + 0x14a7], 1
0x1404dc6b2 mov byte ptr [rbp + 0x14a6], 1
0x1404dc6b9 mov byte ptr [rbp + 0x14a5], 1
0x1404dc6c0 mov rcx, r15
0x1404dc6c3 call 0x1428d8810
0x1404dc6c8 jmp 0x1404dafac
0x1404dc6cd mov byte ptr [rbp + 0x14a7], 1
0x1404dc6d4 mov byte ptr [rbp + 0x14a6], 1
0x1404dc6db mov byte ptr [rbp + 0x14a5], 1
0x1404dc6e2 lea rcx, [rip + 0x31584a8]
0x1404dc6e9 call 0x1428d8750
0x1404dc6ee jmp 0x1404dbc8f
0x1404dc6f3 mov byte ptr [rbp + 0x14a7], 1
0x1404dc6fa mov byte ptr [rbp + 0x14a6], 1
0x1404dc701 mov byte ptr [rbp + 0x14a5], 1
0x1404dc708 call 0x1428d86b0
0x1404dc70d xor al, 1
0x1404dc70f movzx ecx, byte ptr [rip + 0x315847c]
0x1404dc716 test cl, cl
0x1404dc718 je 0x1404dbcb6
0x1404dc71e jmp 0x1404df4bb
0x1404dc723 mov eax, dword ptr [rbp - 0x3c]
0x1404dc726 mov dword ptr [rbp + 0x1498], eax
0x1404dc72c jmp 0x1404dc1f1
0x1404dc731 movzx eax, byte ptr [rbp - 0x38]
0x1404dc735 lea edx, [rax - 0x18]
0x1404dc738 cmp dl, 3
0x1404dc73b movzx edx, dl
0x1404dc73e mov r8d, 1
0x1404dc744 cmovae edx, r8d
0x1404dc748 test dl, dl
0x1404dc74a je 0x1404dc805
0x1404dc750 movzx edx, dl
0x1404dc753 cmp edx, 1
0x1404dc756 jne 0x1404dc831
0x1404dc75c lea r8, [rbp]
0x1404dc760 mov rdx, qword ptr [r8 - 0x28]
0x1404dc764 lea r9, [rbp + 0x1431]
0x1404dc76b mov qword ptr [r9 + 0xf], rdx
0x1404dc76f movupd xmm0, xmmword ptr [r8 - 0x37]
0x1404dc775 movupd xmmword ptr [r9], xmm0
0x1404dc77a mov byte ptr [rbp + 0x1430], al
0x1404dc780 jmp 0x1404dc1f1
0x1404dc785 mov byte ptr [rbp + 0x14a7], sil
0x1404dc78c mov byte ptr [rbp + 0x14a6], 1
0x1404dc793 mov byte ptr [rbp + 0x14a5], 1
0x1404dc79a mov rcx, qword ptr [rbp + 0x13d8]
0x1404dc7a1 call 0x1428d8d40
0x1404dc7a6 jmp 0x1404dbfee
0x1404dc7ab mov byte ptr [rbp + 0x14a7], 1
0x1404dc7b2 mov byte ptr [rbp + 0x14a6], 1
0x1404dc7b9 mov byte ptr [rbp + 0x14a5], 1
0x1404dc7c0 call 0x1428d86b0
0x1404dc7c5 test al, al
0x1404dc7c7 jne 0x1404daf9f
0x1404dc7cd mov byte ptr [r15 + 1], 1
0x1404dc7d2 jmp 0x1404daf9f
0x1404dc7d7 mov byte ptr [rbp + 0x14a7], 1
0x1404dc7de mov byte ptr [rbp + 0x14a6], 1
0x1404dc7e5 mov byte ptr [rbp + 0x14a5], 1
0x1404dc7ec call 0x1428d86b0
0x1404dc7f1 test al, al
0x1404dc7f3 jne 0x1404dbcd7
0x1404dc7f9 mov byte ptr [rip + 0x3158392], 1
0x1404dc800 jmp 0x1404dbcd7
0x1404dc805 mov rax, qword ptr [rbp - 0x30]
0x1404dc809 lock inc qword ptr [rax]
0x1404dc80d jle 0x1404df65a
0x1404dc813 mov rdx, qword ptr [rbp - 0x28]
0x1404dc817 mov qword ptr [rbp + 0x1438], rax
0x1404dc81e mov qword ptr [rbp + 0x1440], rdx
0x1404dc825 mov byte ptr [rbp + 0x1430], 0x18
0x1404dc82c jmp 0x1404dc1f1
0x1404dc831 movupd xmm0, xmmword ptr [rbp - 0x30]
0x1404dc836 movupd xmmword ptr [rbp + 0x1438], xmm0
0x1404dc83e mov byte ptr [rbp + 0x1430], 0x1a
0x1404dc845 jmp 0x1404dc1f1
0x1404dc84a nop word ptr [rax + rax]
0x1404dc850 lea rcx, [rbp + 0x13e0]
0x1404dc857 call 0x140609210
0x1404dc85c mov rcx, qword ptr [rbp + 0x1488]
0x1404dc863 movabs rdx, 0xa4c02661e67b048b
0x1404dc86d movabs r8, 0xeae761e6aa2a67ec
0x1404dc877 call 0x1400dd6f0
0x1404dc87c test rax, rax
0x1404dc87f mov r8, qword ptr [rbp + 0x1478]
0x1404dc886 je 0x1404df174
0x1404dc88c cmp qword ptr [r8 + 0x248], rdx
0x1404dc893 jbe 0x1404df174
0x1404dc899 mov rax, qword ptr [r8 + 0x240]
0x1404dc8a0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc8a4 test rcx, rcx
0x1404dc8a7 je 0x1404df174
0x1404dc8ad mov rax, qword ptr [r8 + 0x210]
0x1404dc8b4 not rcx
0x1404dc8b7 lea rcx, [rcx + rcx*2]
0x1404dc8bb shl rcx, 5
0x1404dc8bf cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc8c5 je 0x1404df174
0x1404dc8cb add rax, rcx
0x1404dc8ce mov rax, qword ptr [rax + 0x10]
0x1404dc8d2 mov rax, qword ptr [rax]
0x1404dc8d5 mov qword ptr [rbp + 0x1498], rax
0x1404dc8dc mov rcx, qword ptr [rbp + 0x1488]
0x1404dc8e3 movabs rdx, 0x3335386a5fbcee80
0x1404dc8ed movabs r8, 0x8f2a59f31a65ec60
0x1404dc8f7 call 0x1400dd6f0
0x1404dc8fc test rax, rax
0x1404dc8ff mov r8, qword ptr [rbp + 0x1478]
0x1404dc906 je 0x1404df163
0x1404dc90c cmp qword ptr [r8 + 0x248], rdx
0x1404dc913 jbe 0x1404df163
0x1404dc919 mov rax, qword ptr [r8 + 0x240]
0x1404dc920 mov rcx, qword ptr [rax + rdx*8]
0x1404dc924 test rcx, rcx
0x1404dc927 je 0x1404df163
0x1404dc92d mov rax, qword ptr [r8 + 0x210]
0x1404dc934 not rcx
0x1404dc937 lea rcx, [rcx + rcx*2]
0x1404dc93b shl rcx, 5
0x1404dc93f cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc945 je 0x1404df163
0x1404dc94b mov r8, qword ptr [rbp + 0x1498]
0x1404dc952 test r8d, r8d
0x1404dc955 setne dl
0x1404dc958 shr r8, 0x20
0x1404dc95c setne r8b
0x1404dc960 test r8b, dl
0x1404dc963 jne 0x1404dc980
0x1404dc965 xor ecx, ecx
0x1404dc967 mov edx, 0xf4240
0x1404dc96c call 0x140d55980
0x1404dc971 jmp 0x1404de84d
0x1404dc976 nop word ptr cs:[rax + rax]
0x1404dc980 add rax, rcx
0x1404dc983 mov rax, qword ptr [rax + 0x10]
0x1404dc987 mov bl, byte ptr [rax + 0x28]
0x1404dc98a mov rcx, qword ptr [rbp + 0x1488]
0x1404dc991 movabs rdx, 0x2ed189581b92decf
0x1404dc99b movabs r8, 0x15d3d99396065f38
0x1404dc9a5 call 0x1400dd6f0
0x1404dc9aa test rax, rax
0x1404dc9ad mov r8, qword ptr [rbp + 0x1478]
0x1404dc9b4 je 0x1404df316
0x1404dc9ba cmp qword ptr [r8 + 0x248], rdx
0x1404dc9c1 jbe 0x1404df316
0x1404dc9c7 mov rax, qword ptr [r8 + 0x240]
0x1404dc9ce mov rcx, qword ptr [rax + rdx*8]
0x1404dc9d2 test rcx, rcx
0x1404dc9d5 je 0x1404df316
0x1404dc9db mov rax, qword ptr [r8 + 0x210]
0x1404dc9e2 not rcx
0x1404dc9e5 lea rcx, [rcx + rcx*2]
0x1404dc9e9 shl rcx, 5
0x1404dc9ed cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc9f3 je 0x1404df316
0x1404dc9f9 add rax, rcx
0x1404dc9fc mov rcx, qword ptr [rax + 0x10]
0x1404dca00 call 0x140987ca0
0x1404dca05 mov rcx, rax
0x1404dca08 call 0x140bf8f80
0x1404dca0d mov rcx, qword ptr [rbp + 0x1488]
0x1404dca14 movabs rdx, 0x2ed189581b92decf
0x1404dca1e movabs r8, 0x15d3d99396065f38
0x1404dca28 call 0x1400dd6f0
0x1404dca2d test rax, rax
0x1404dca30 mov r8, qword ptr [rbp + 0x1478]
0x1404dca37 je 0x1404df327
0x1404dca3d cmp qword ptr [r8 + 0x248], rdx
0x1404dca44 jbe 0x1404df327
0x1404dca4a mov rax, qword ptr [r8 + 0x240]
0x1404dca51 mov rcx, qword ptr [rax + rdx*8]
0x1404dca55 test rcx, rcx
0x1404dca58 je 0x1404df327
0x1404dca5e mov rax, qword ptr [r8 + 0x210]
0x1404dca65 not rcx
0x1404dca68 lea rcx, [rcx + rcx*2]
0x1404dca6c shl rcx, 5
0x1404dca70 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dca76 je 0x1404df327
0x1404dca7c add rax, rcx
0x1404dca7f mov rcx, qword ptr [rax + 0x10]
0x1404dca83 call 0x140987ca0
0x1404dca88 mov rdi, rax
0x1404dca8b movss xmm6, dword ptr [rip + 0x2448eed]
0x1404dca93 test bl, bl
0x1404dca95 je 0x1404dcaaa
0x1404dca97 mov rcx, rdi
0x1404dca9a mov rdx, qword ptr [rbp + 0x1498]
0x1404dcaa1 call 0x1404f7f20
0x1404dcaa6 movapd xmm6, xmm0
0x1404dcaaa mov rcx, qword ptr [rbp + 0x1420]
0x1404dcab1 mov rdx, rdi
0x1404dcab4 call 0x140c0ea90
0x1404dcab9 mov qword ptr [rbp + 0x1468], rax
0x1404dcac0 mov rax, qword ptr [rbp + 0x1468]
0x1404dcac7 cmp rax, qword ptr [rbp + 0x1498]
0x1404dcace je 0x1404dcae6
0x1404dcad0 mov rcx, qword ptr [rbp + 0x1420]
0x1404dcad7 mov rdx, rdi
0x1404dcada mov r8, qword ptr [rbp + 0x1498]
0x1404dcae1 call 0x140c0e7f0
0x1404dcae6 movss dword ptr [rbp + 0x1458], xmm6
0x1404dcaee mov rcx, qword ptr [rbp + 0x1488]
0x1404dcaf5 movabs rdx, 0x3335386a5fbcee80
0x1404dcaff movabs r8, 0x8f2a59f31a65ec60
0x1404dcb09 call 0x1400dd6f0
0x1404dcb0e test rax, rax
0x1404dcb11 mov r8, qword ptr [rbp + 0x1478]
0x1404dcb18 je 0x1404df338
0x1404dcb1e cmp qword ptr [r8 + 0x248], rdx
0x1404dcb25 jbe 0x1404df338
0x1404dcb2b mov rax, qword ptr [r8 + 0x240]
0x1404dcb32 mov rcx, qword ptr [rax + rdx*8]
0x1404dcb36 test rcx, rcx
0x1404dcb39 je 0x1404df338
0x1404dcb3f mov rax, qword ptr [r8 + 0x210]
0x1404dcb46 not rcx
0x1404dcb49 lea rcx, [rcx + rcx*2]
0x1404dcb4d shl rcx, 5
0x1404dcb51 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dcb57 je 0x1404df338
0x1404dcb5d add rax, rcx
0x1404dcb60 mov rax, qword ptr [rax + 0x10]
0x1404dcb64 mov r14, qword ptr [rax]
0x1404dcb67 mov rcx, qword ptr [rax + 8]
0x1404dcb6b mov qword ptr [rbp + 0x1428], rcx
0x1404dcb72 mov r15d, dword ptr [rax + 0x10]
0x1404dcb76 mov r13, qword ptr [rax + 0x18]
0x1404dcb7a mov edi, dword ptr [rax + 0x20]
0x1404dcb7d mov rcx, qword ptr [rbp + 0x1488]
0x1404dcb84 movabs rdx, 0x3d7b56e787d47c40
0x1404dcb8e movabs r8, 0x67b0b10a8d43bfc7
0x1404dcb98 call 0x1400dd6f0
0x1404dcb9d test rax, rax
0x1404dcba0 mov r8, qword ptr [rbp + 0x1478]
0x1404dcba7 je 0x1404df349
0x1404dcbad cmp qword ptr [r8 + 0x248], rdx
0x1404dcbb4 jbe 0x1404df349
0x1404dcbba mov rax, qword ptr [r8 + 0x240]
0x1404dcbc1 mov rcx, qword ptr [rax + rdx*8]
0x1404dcbc5 test rcx, rcx
0x1404dcbc8 je 0x1404df349
0x1404dcbce mov rax, qword ptr [r8 + 0x210]
0x1404dcbd5 not rcx
0x1404dcbd8 lea rcx, [rcx + rcx*2]
0x1404dcbdc shl rcx, 5
0x1404dcbe0 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dcbe6 je 0x1404df349
0x1404dcbec add rax, rcx
0x1404dcbef mov rdx, qword ptr [rax + 0x10]
0x1404dcbf3 lea rcx, [rbp + 0xc30]
0x1404dcbfa call 0x1404d7d50
0x1404dcbff cmp byte ptr [rbp + 0xcb8], 0
0x1404dcc06 jne 0x1404dcc20
0x1404dcc08 movss xmm0, dword ptr [rip + 0x2448cf4]
0x1404dcc10 jmp 0x1404dcc28
0x1404dcc12 nop word ptr cs:[rax + rax]
0x1404dcc20 movss xmm0, dword ptr [rbp + 0xcbc]
0x1404dcc28 movss dword ptr [rbp + 0x1430], xmm0
0x1404dcc30 mov rcx, qword ptr [rbp + 0x1488]
0x1404dcc37 movabs rdx, 0xa4c02661e67b048b
0x1404dcc41 movabs r8, 0xeae761e6aa2a67ec
0x1404dcc4b call 0x1400dd6f0
0x1404dcc50 test rax, rax
0x1404dcc53 mov rcx, qword ptr [rbp + 0x1478]
0x1404dcc5a je 0x1404df35a
0x1404dcc60 cmp qword ptr [rcx + 0x248], rdx
0x1404dcc67 jbe 0x1404df35a
0x1404dcc6d mov rax, qword ptr [rcx + 0x240]
0x1404dcc74 mov rax, qword ptr [rax + rdx*8]
0x1404dcc78 test rax, rax
0x1404dcc7b je 0x1404df35a
0x1404dcc81 mov r12, qword ptr [rcx + 0x210]
0x1404dcc88 not rax
0x1404dcc8b lea rax, [rax + rax*2]
0x1404dcc8f shl rax, 5
0x1404dcc93 cmp qword ptr [r12 + rax + 0x20], 0
0x1404dcc99 je 0x1404df35a
0x1404dcc9f add r12, rax
0x1404dcca2 mov rax, qword ptr [r12 + 0x10]
0x1404dcca7 mov ecx, dword ptr [rcx + 0x5e0]
0x1404dccad mov dword ptr [rbp + 0x1448], ecx
0x1404dccb3 lea rcx, [rax + 0x18]
0x1404dccb7 mov qword ptr [rbp + 0x1460], rcx
0x1404dccbe mov qword ptr [rbp + 0x1328], rax
0x1404dccc5 movss xmm0, dword ptr [rax + 0x18]
0x1404dccca movss xmm6, dword ptr [rbp + 0x1458]
0x1404dccd2 ucomiss xmm0, xmm6
0x1404dccd5 jbe 0x1404dcec0
0x1404dccdb mov rax, qword ptr [rip + 0x314f6c6]
0x1404dcce2 mov rax, qword ptr [rax]
0x1404dcce5 cmp rax, 3
0x1404dcce9 jb 0x1404dce60
0x1404dccef mov rax, qword ptr [rbp + 0x1460]
0x1404dccf6 mov qword ptr [rbp + 0x13e0], rax
0x1404dccfd lea rcx, [rip + 0x8afc0c]
0x1404dcd04 mov qword ptr [rbp + 0x13e8], rcx
0x1404dcd0b lea rax, [rbp + 0x1458]
0x1404dcd12 mov qword ptr [rbp + 0x13f0], rax
0x1404dcd19 mov qword ptr [rbp + 0x13f8], rcx
0x1404dcd20 mov qword ptr [rbp - 0x40], 0
0x1404dcd28 mov qword ptr [rbp - 0x38], 1
0x1404dcd30 mov qword ptr [rbp - 0x30], 0
0x1404dcd38 mov qword ptr [rbp - 0x28], 1
0x1404dcd40 mov qword ptr [rbp - 0x20], 0
0x1404dcd48 mov qword ptr [rbp - 0x18], 0x20
0x1404dcd50 mov byte ptr [rbp - 0x10], 3
0x1404dcd54 mov qword ptr [rbp - 8], 0
0x1404dcd5c mov qword ptr [rbp], 1
0x1404dcd64 mov qword ptr [rbp + 8], 0
0x1404dcd6c mov qword ptr [rbp + 0x10], 1
0x1404dcd74 mov qword ptr [rbp + 0x18], 1
0x1404dcd7c mov qword ptr [rbp + 0x20], 0x20
0x1404dcd84 mov byte ptr [rbp + 0x28], 3
0x1404dcd88 lea rcx, [rip + 0x250d4f1]
0x1404dcd8f call 0x14067dd80
0x1404dcd94 movups xmm0, xmmword ptr [rax]
0x1404dcd97 mov eax, dword ptr [rax + 0x10]
0x1404dcd9a mov qword ptr [rbp + 0x630], 3
0x1404dcda5 lea rdx, [rip + 0x250cf36]
0x1404dcdac mov qword ptr [rbp + 0x638], rdx
0x1404dcdb3 mov qword ptr [rbp + 0x640], 0x15
0x1404dcdbe lea rcx, [rip + 0x250d49b]
0x1404dcdc5 mov qword ptr [rbp + 0x650], rcx
0x1404dcdcc mov qword ptr [rbp + 0x658], 2
0x1404dcdd7 lea rcx, [rbp + 0x13e0]
0x1404dcdde mov qword ptr [rbp + 0x660], rcx
0x1404dcde5 mov qword ptr [rbp + 0x668], 2
0x1404dcdf0 lea rcx, [rbp - 0x40]
0x1404dcdf4 mov qword ptr [rbp + 0x670], rcx
0x1404dcdfb mov qword ptr [rbp + 0x678], 2
0x1404dce06 mov qword ptr [rbp + 0x600], 0
0x1404dce11 mov qword ptr [rbp + 0x608], rdx
0x1404dce18 mov qword ptr [rbp + 0x610], 0x15
0x1404dce23 mov qword ptr [rbp + 0x618], 0
0x1404dce2e movups xmmword ptr [rbp + 0x620], xmm0
0x1404dce35 mov dword ptr [rbp + 0x648], 1
0x1404dce3f mov dword ptr [rbp + 0x64c], eax
0x1404dce45 lea rcx, [rbp + 0x1477]
0x1404dce4c lea rdx, [rbp + 0x600]
0x1404dce53 call 0x140d4a330
0x1404dce58 movss xmm6, dword ptr [rbp + 0x1458]
0x1404dce60 mov eax, dword ptr [rbp + 0x1448]
0x1404dce66 mov dword ptr [r12 + 0x5c], eax
0x1404dce6b mov rax, qword ptr [rbp + 0x1460]
0x1404dce72 movss dword ptr [rax], xmm6
0x1404dce76 lea rcx, [rip + 0x250d61b]
0x1404dce7d movsxd rax, dword ptr [rcx + r14*4]
0x1404dce81 add rax, rcx
0x1404dce84 jmp rax
0x1404dce86 call 0x140d635e0
0x1404dce8b mov rcx, rax
0x1404dce8e xor r8d, r8d
0x1404dce91 mov r9d, 0xee6b280
0x1404dce97 call 0x140d63a20
0x1404dce9c mov r13, rax
0x1404dce9f mov edi, edx
0x1404dcea1 mov r14d, 1
0x1404dcea7 mov r15d, 0xee6b280
0x1404dcead xor esi, esi
0x1404dceaf jmp 0x1404dd1f0
0x1404dceb4 nop word ptr cs:[rax + rax]
0x1404dcec0 call 0x140d635e0
0x1404dcec5 mov rbx, rax
0x1404dcec8 lea rcx, [rip + 0x250d5b9]
0x1404dcecf movsxd rax, dword ptr [rcx + r14*4]
0x1404dced3 add rax, rcx
0x1404dced6 jmp rax
0x1404dced8 xor eax, eax
0x1404dceda cmp rbx, r13
0x1404dcedd setne al
0x1404dcee0 mov ecx, 0xff
0x1404dcee5 cmovb eax, ecx
0x1404dcee8 cmp edx, edi
0x1404dceea mov ecx, 0
0x1404dceef sbb ecx, ecx
0x1404dcef1 test al, al
0x1404dcef3 cmovne ecx, eax
0x1404dcef6 mov r14d, 1
0x1404dcefc cmp cl, 1
0x1404dceff ja 0x1404dcf12
0x1404dcf01 movss xmm0, dword ptr [rbp + 0x1430]
0x1404dcf09 ucomiss xmm6, xmm0
0x1404dcf0c jae 0x1404dcfe4
0x1404dcf12 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcf19 jmp 0x1404dd1f0
0x1404dcf1e call 0x140d635e0
0x1404dcf23 mov rcx, rax
0x1404dcf26 mov r8, qword ptr [rbp + 0x1428]
0x1404dcf2d mov r9d, r15d
0x1404dcf30 call 0x140d63a20
0x1404dcf35 mov r13, rax
0x1404dcf38 mov edi, edx
0x1404dcf3a mov r14d, 1
0x1404dcf40 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcf47 jmp 0x1404dd1f0
0x1404dcf4c cmp qword ptr [rbp + 0x1428], 0
0x1404dcf54 js 0x1404def11
0x1404dcf5a lea eax, [r15 + r15]
0x1404dcf5e xor esi, esi
0x1404dcf60 cmp r15d, 0x1dcd6500
0x1404dcf67 lea r15d, [r15 + r15 - 0x3b9aca00]
0x1404dcf6f cmovb r15d, eax
0x1404dcf73 setae bl
0x1404dcf76 call 0x140d635e0
0x1404dcf7b mov sil, bl
0x1404dcf7e mov rcx, qword ptr [rbp + 0x1428]
0x1404dcf85 lea rsi, [rsi + rcx*2]
0x1404dcf89 mov rcx, rax
0x1404dcf8c mov r8, rsi
0x1404dcf8f mov r9d, r15d
0x1404dcf92 call 0x140d63a20
0x1404dcf97 mov r13, rax
0x1404dcf9a mov edi, edx
0x1404dcf9c mov r14d, 1
0x1404dcfa2 jmp 0x1404dd1f0
0x1404dcfa7 xor eax, eax
0x1404dcfa9 cmp rbx, r13
0x1404dcfac setne al
0x1404dcfaf mov ecx, 0xff
0x1404dcfb4 cmovb eax, ecx
0x1404dcfb7 cmp edx, edi
0x1404dcfb9 mov ecx, 0
0x1404dcfbe sbb ecx, ecx
0x1404dcfc0 test al, al
0x1404dcfc2 cmovne ecx, eax
0x1404dcfc5 cmp cl, 2
0x1404dcfc8 jae 0x1404dcfd2
0x1404dcfca xor r14d, r14d
0x1404dcfcd jmp 0x1404dd1f0
0x1404dcfd2 mov r14d, 3
0x1404dcfd8 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcfdf jmp 0x1404dd1f0
0x1404dcfe4 mov edi, edx
0x1404dcfe6 mov rax, qword ptr [rip + 0x314f3bb]
0x1404dcfed mov rax, qword ptr [rax]
0x1404dcff0 cmp rax, 3
0x1404dcff4 jb 0x1404dd16b
0x1404dcffa mov rax, qword ptr [rbp + 0x1460]
0x1404dd001 mov qword ptr [rbp + 0x13e0], rax
0x1404dd008 lea rax, [rip + 0x8af901]
0x1404dd00f mov qword ptr [rbp + 0x13e8], rax
0x1404dd016 lea rcx, [rbp + 0x1430]
0x1404dd01d mov qword ptr [rbp + 0x13f0], rcx
0x1404dd024 mov qword ptr [rbp + 0x13f8], rax
0x1404dd02b mov qword ptr [rbp - 0x40], 0
0x1404dd033 mov qword ptr [rbp - 0x38], 1
0x1404dd03b mov qword ptr [rbp - 0x30], 0
0x1404dd043 mov qword ptr [rbp - 0x28], 1
0x1404dd04b mov qword ptr [rbp - 0x20], 0
0x1404dd053 mov qword ptr [rbp - 0x18], 0x20
0x1404dd05b mov byte ptr [rbp - 0x10], 3
0x1404dd05f mov qword ptr [rbp - 8], 0
0x1404dd067 mov qword ptr [rbp], 1
0x1404dd06f mov qword ptr [rbp + 8], 0
0x1404dd077 mov qword ptr [rbp + 0x10], 1
0x1404dd07f mov qword ptr [rbp + 0x18], 1
0x1404dd087 mov qword ptr [rbp + 0x20], 0x20
0x1404dd08f mov byte ptr [rbp + 0x28], 3
0x1404dd093 lea rcx, [rip + 0x250d176]
0x1404dd09a call 0x14067dd80
0x1404dd09f movups xmm0, xmmword ptr [rax]
0x1404dd0a2 mov eax, dword ptr [rax + 0x10]
0x1404dd0a5 mov qword ptr [rbp + 0x630], 3
0x1404dd0b0 lea rdx, [rip + 0x250cc2b]
0x1404dd0b7 mov qword ptr [rbp + 0x638], rdx
0x1404dd0be mov qword ptr [rbp + 0x640], 0x15
0x1404dd0c9 lea rcx, [rip + 0x250d120]
0x1404dd0d0 mov qword ptr [rbp + 0x650], rcx
0x1404dd0d7 mov qword ptr [rbp + 0x658], 2
0x1404dd0e2 lea rcx, [rbp + 0x13e0]
0x1404dd0e9 mov qword ptr [rbp + 0x660], rcx
0x1404dd0f0 mov qword ptr [rbp + 0x668], 2
0x1404dd0fb lea rcx, [rbp - 0x40]
0x1404dd0ff mov qword ptr [rbp + 0x670], rcx
0x1404dd106 mov qword ptr [rbp + 0x678], 2
0x1404dd111 mov qword ptr [rbp + 0x600], 0
0x1404dd11c mov qword ptr [rbp + 0x608], rdx
0x1404dd123 mov qword ptr [rbp + 0x610], 0x15
0x1404dd12e mov qword ptr [rbp + 0x618], 0
0x1404dd139 movups xmmword ptr [rbp + 0x620], xmm0
0x1404dd140 mov dword ptr [rbp + 0x648], 1
0x1404dd14a mov dword ptr [rbp + 0x64c], eax
0x1404dd150 lea rcx, [rbp + 0x1477]
0x1404dd157 lea rdx, [rbp + 0x600]
0x1404dd15e call 0x140d4a330
0x1404dd163 movss xmm0, dword ptr [rbp + 0x1430]
0x1404dd16b mov eax, dword ptr [rbp + 0x1448]
0x1404dd171 mov dword ptr [r12 + 0x5c], eax
0x1404dd176 mov rax, qword ptr [rbp + 0x1460]
0x1404dd17d movss dword ptr [rax], xmm0
0x1404dd181 mov rdx, qword ptr [rbp + 0x1428]
0x1404dd188 test rdx, rdx
0x1404dd18b js 0x1404defa8
0x1404dd191 lea eax, [r15 + r15]
0x1404dd195 xor ecx, ecx
0x1404dd197 cmp r15d, 0x1dcd6500
0x1404dd19e lea r15d, [r15 + r15 - 0x3b9aca00]
0x1404dd1a6 cmovb r15d, eax
0x1404dd1aa setae cl
0x1404dd1ad lea rsi, [rcx + rdx*2]
0x1404dd1b1 test rdx, rdx
0x1404dd1b4 mov r9d, r15d
0x1404dd1b7 mov eax, 0
0x1404dd1bc cmove r9d, eax
0x1404dd1c0 cmp rsi, 3
0x1404dd1c4 mov r8d, 2
0x1404dd1ca cmovae r8, rsi
0x1404dd1ce mov rcx, rbx
0x1404dd1d1 mov edx, edi
0x1404dd1d3 call 0x140d63a20
0x1404dd1d8 mov r13, rax
0x1404dd1db mov edi, edx
0x1404dd1dd mov r14d, 3
0x1404dd1e3 nop word ptr cs:[rax + rax]
0x1404dd1f0 mov rax, qword ptr [rbp + 0x1460]
0x1404dd1f7 movss xmm6, dword ptr [rax]
0x1404dd1fb cmp byte ptr [rbp + 0xcdb], 1
0x1404dd202 movss xmm0, dword ptr [rip + 0x24486fa]
0x1404dd20a jne 0x1404dd218
0x1404dd20c lea rcx, [rbp + 0xcf3]
0x1404dd213 call 0x140968bb0
0x1404dd218 mov r11d, dword ptr [rbp + 0x1448]
0x1404dd21f mov dword ptr [r12 + 0x5c], r11d
0x1404dd224 mov r10, qword ptr [rbp + 0x1328]
0x1404dd22b movss dword ptr [r10 + 0x1c], xmm0
0x1404dd231 movsd xmm0, qword ptr [r10]
0x1404dd236 unpcklps xmm0, xmmword ptr [rip + 0x2448763]
0x1404dd23d orpd xmm0, xmm10
0x1404dd242 subpd xmm0, xmm10
0x1404dd247 cvtpd2ps xmm0, xmm0
0x1404dd24b movsldup xmm6, xmm6
0x1404dd24f mulps xmm0, xmm6
0x1404dd252 movaps xmm1, xmm0
0x1404dd255 shufps xmm1, xmm0, 0xff
0x1404dd259 cvttss2si rax, xmm1
0x1404dd25e ucomiss xmm1, xmm11
0x1404dd262 mov r8d, 0
0x1404dd268 cmovb eax, r8d
0x1404dd26c ucomiss xmm1, xmm12
0x1404dd270 mov r9d, 0xffffffff
0x1404dd276 cmova eax, r9d
0x1404dd27a movaps xmm1, xmm0
0x1404dd27d unpckhpd xmm1, xmm0
0x1404dd281 cvttss2si rcx, xmm1
0x1404dd286 ucomiss xmm1, xmm11
0x1404dd28a cmovb ecx, r8d
0x1404dd28e ucomiss xmm1, xmm12
0x1404dd292 movd xmm1, eax
0x1404dd296 cmova ecx, r9d
0x1404dd29a cvttss2si rax, xmm0
0x1404dd29f ucomiss xmm0, xmm11
0x1404dd2a3 cmovb eax, r8d
0x1404dd2a7 ucomiss xmm0, xmm12
0x1404dd2ab cmova eax, r9d
0x1404dd2af movshdup xmm0, xmm0
0x1404dd2b3 cvttss2si rdx, xmm0
0x1404dd2b8 ucomiss xmm0, xmm11
0x1404dd2bc cmovb edx, r8d
0x1404dd2c0 ucomiss xmm0, xmm12
0x1404dd2c4 movd xmm0, ecx
0x1404dd2c8 cmova edx, r9d
0x1404dd2cc movd xmm2, eax
0x1404dd2d0 punpckldq xmm0, xmm1
0x1404dd2d4 movd xmm1, edx
0x1404dd2d8 punpckldq xmm2, xmm1
0x1404dd2dc punpcklqdq xmm2, xmm0
0x1404dd2e0 movdqa xmm0, xmm2
0x1404dd2e4 pxor xmm0, xmm13
0x1404dd2e9 pcmpgtd xmm0, xmmword ptr [rip + 0x250c2cf]
0x1404dd2f1 movdqa xmm1, xmm0
0x1404dd2f5 pandn xmm1, xmm2
0x1404dd2f9 pand xmm0, xmmword ptr [rip + 0x250c2cf]
0x1404dd301 por xmm0, xmm1
0x1404dd305 movdqa xmm1, xmm0
0x1404dd309 pxor xmm1, xmm13
0x1404dd30e pcmpgtd xmm1, xmmword ptr [rip + 0x250c2ca]
0x1404dd316 pand xmm0, xmm1
0x1404dd31a pandn xmm1, xmmword ptr [rip + 0x250c2ce]
0x1404dd322 por xmm1, xmm0
0x1404dd326 mov dword ptr [r12 + 0x5c], r11d
0x1404dd32b movq qword ptr [r10 + 8], xmm1
0x1404dd331 mov rcx, qword ptr [rbp + 0x1488]
0x1404dd338 movabs rdx, 0x3335386a5fbcee80
0x1404dd342 movabs r8, 0x8f2a59f31a65ec60
0x1404dd34c call 0x1400dd6f0
0x1404dd351 test rax, rax
0x1404dd354 mov r8, qword ptr [rbp + 0x1478]
0x1404dd35b je 0x1404df36b
0x1404dd361 cmp qword ptr [r8 + 0x248], rdx
0x1404dd368 jbe 0x1404df36b
0x1404dd36e mov rax, qword ptr [r8 + 0x240]
0x1404dd375 mov rcx, qword ptr [rax + rdx*8]
0x1404dd379 test rcx, rcx
0x1404dd37c je 0x1404df36b
0x1404dd382 mov rax, qword ptr [r8 + 0x210]
0x1404dd389 not rcx
0x1404dd38c lea rcx, [rcx + rcx*2]
0x1404dd390 shl rcx, 5
0x1404dd394 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd39a je 0x1404df36b
0x1404dd3a0 add rax, rcx
0x1404dd3a3 mov rcx, qword ptr [rax + 0x10]
0x1404dd3a7 mov edx, dword ptr [r8 + 0x5e0]
0x1404dd3ae mov dword ptr [rax + 0x5c], edx
0x1404dd3b1 mov qword ptr [rcx], r14
0x1404dd3b4 mov qword ptr [rcx + 8], rsi
0x1404dd3b8 mov dword ptr [rcx + 0x10], r15d
0x1404dd3bc mov qword ptr [rcx + 0x18], r13
0x1404dd3c0 mov dword ptr [rcx + 0x20], edi
0x1404dd3c3 lea rcx, [rbp + 0xc30]
0x1404dd3ca call 0x1405fdb80
0x1404dd3cf mov rcx, qword ptr [rbp + 0x1488]
0x1404dd3d6 movabs rdx, 0xa4c02661e67b048b
0x1404dd3e0 movabs r8, 0xeae761e6aa2a67ec
0x1404dd3ea call 0x1400dd6f0
0x1404dd3ef test rax, rax
0x1404dd3f2 mov r8, qword ptr [rbp + 0x1478]
0x1404dd3f9 je 0x1404df38d
0x1404dd3ff cmp qword ptr [r8 + 0x248], rdx
0x1404dd406 jbe 0x1404df38d
0x1404dd40c mov rax, qword ptr [r8 + 0x240]
0x1404dd413 mov rcx, qword ptr [rax + rdx*8]
0x1404dd417 test rcx, rcx
0x1404dd41a je 0x1404df38d
0x1404dd420 mov rax, qword ptr [r8 + 0x210]
0x1404dd427 not rcx
0x1404dd42a lea rcx, [rcx + rcx*2]
0x1404dd42e shl rcx, 5
0x1404dd432 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd438 je 0x1404df38d
0x1404dd43e movsd xmm0, qword ptr [rbp + 0x1498]
0x1404dd446 add rax, rcx
0x1404dd449 unpcklps xmm0, xmmword ptr [rip + 0x2448550]
0x1404dd450 orpd xmm0, xmm10
0x1404dd455 subpd xmm0, xmm10
0x1404dd45a cvtpd2ps xmm0, xmm0
0x1404dd45e mov rax, qword ptr [rax + 0x10]
0x1404dd462 mulps xmm6, xmm0
0x1404dd465 movss xmm0, dword ptr [rax + 0x1c]
0x1404dd46a movsldup xmm0, xmm0
0x1404dd46e mulps xmm0, xmm6
0x1404dd471 addps xmm0, xmmword ptr [rip + 0x250c188]
0x1404dd478 movaps xmm1, xmm0
0x1404dd47b shufps xmm1, xmm0, 0xff
0x1404dd47f cvttss2si rdx, xmm1
0x1404dd484 ucomiss xmm1, xmm11
0x1404dd488 mov eax, 0
0x1404dd48d cmovb edx, eax
0x1404dd490 ucomiss xmm1, xmm12
0x1404dd494 mov ecx, 0xffffffff
0x1404dd499 cmova edx, ecx
0x1404dd49c mov qword ptr [rbp + 0x1460], rdx
0x1404dd4a3 movaps xmm1, xmm0
0x1404dd4a6 unpckhpd xmm1, xmm0
0x1404dd4aa cvttss2si r14, xmm1
0x1404dd4af ucomiss xmm1, xmm11
0x1404dd4b3 cmovb r14d, eax
0x1404dd4b7 ucomiss xmm1, xmm12
0x1404dd4bb cmova r14d, ecx
0x1404dd4bf cvttss2si r15, xmm0
0x1404dd4c4 ucomiss xmm0, xmm11
0x1404dd4c8 cmovb r15d, eax
0x1404dd4cc ucomiss xmm0, xmm12
0x1404dd4d0 cmova r15d, ecx
0x1404dd4d4 movshdup xmm0, xmm0
0x1404dd4d8 cvttss2si rsi, xmm0
0x1404dd4dd ucomiss xmm0, xmm11
0x1404dd4e1 cmovb esi, eax
0x1404dd4e4 ucomiss xmm0, xmm12
0x1404dd4e8 cmova esi, ecx
0x1404dd4eb movaps xmm0, xmm6
0x1404dd4ee shufps xmm0, xmm6, 0xff
0x1404dd4f2 cvttss2si rbx, xmm0
0x1404dd4f7 ucomiss xmm0, xmm11
0x1404dd4fb cmovb ebx, eax
0x1404dd4fe ucomiss xmm0, xmm12
0x1404dd502 cmova ebx, ecx
0x1404dd505 movaps xmm0, xmm6
0x1404dd508 unpckhpd xmm0, xmm6
0x1404dd50c cvttss2si r12, xmm0
0x1404dd511 ucomiss xmm0, xmm11
0x1404dd515 cmovb r12d, eax
0x1404dd519 ucomiss xmm0, xmm12
0x1404dd51d cmova r12d, ecx
0x1404dd521 cvttss2si r13, xmm6
0x1404dd526 ucomiss xmm6, xmm11
0x1404dd52a cmovb r13d, eax
0x1404dd52e ucomiss xmm6, xmm12
0x1404dd532 cmova r13d, ecx
0x1404dd536 movshdup xmm0, xmm6
0x1404dd53a cvttss2si rdi, xmm0
0x1404dd53f ucomiss xmm0, xmm11
0x1404dd543 cmovb edi, eax
0x1404dd546 ucomiss xmm0, xmm12
0x1404dd54a cmova edi, ecx
0x1404dd54d mov rcx, qword ptr [rbp + 0x1488]
0x1404dd554 movabs rdx, 0xa4c02661e67b048b
0x1404dd55e movabs r8, 0xeae761e6aa2a67ec
0x1404dd568 call 0x1400dd6f0
0x1404dd56d test rax, rax
0x1404dd570 mov r8, qword ptr [rbp + 0x1478]
0x1404dd577 je 0x1404df37c
0x1404dd57d cmp qword ptr [r8 + 0x248], rdx
0x1404dd584 jbe 0x1404df37c
0x1404dd58a mov rax, qword ptr [r8 + 0x240]
0x1404dd591 mov rcx, qword ptr [rax + rdx*8]
0x1404dd595 test rcx, rcx
0x1404dd598 je 0x1404df37c
0x1404dd59e mov rax, qword ptr [r8 + 0x210]
0x1404dd5a5 not rcx
0x1404dd5a8 lea rcx, [rcx + rcx*2]
0x1404dd5ac shl rcx, 5
0x1404dd5b0 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd5b6 je 0x1404df37c
0x1404dd5bc movd xmm0, dword ptr [rbp + 0x1460]
0x1404dd5c4 movd xmm1, r14d
0x1404dd5c9 punpckldq xmm1, xmm0
0x1404dd5cd movd xmm0, r15d
0x1404dd5d2 movd xmm2, esi
0x1404dd5d6 punpckldq xmm0, xmm2
0x1404dd5da punpcklqdq xmm0, xmm1
0x1404dd5de pand xmm0, xmmword ptr [rip + 0x250c02a]
0x1404dd5e6 movdqa xmm1, xmm0
0x1404dd5ea pxor xmm1, xmm13
0x1404dd5ef movdqa xmm6, xmmword ptr [rip + 0x250c029]
0x1404dd5f7 movdqa xmm2, xmm6
0x1404dd5fb pcmpgtd xmm2, xmm1
0x1404dd5ff pand xmm0, xmm2
0x1404dd603 movdqa xmm3, xmmword ptr [rip + 0x250bfc5]
0x1404dd60b pandn xmm2, xmm3
0x1404dd60f por xmm2, xmm0
0x1404dd613 movdqa xmm15, xmm2
0x1404dd618 pxor xmm15, xmm13
0x1404dd61d movdqa xmm4, xmmword ptr [rip + 0x250bfbb]
0x1404dd625 pcmpgtd xmm15, xmm4
0x1404dd62a pand xmm2, xmm15
0x1404dd62f movdqa xmm5, xmmword ptr [rip + 0x250bfb9]
0x1404dd637 pandn xmm15, xmm5
0x1404dd63c por xmm15, xmm2
0x1404dd641 movd xmm0, ebx
0x1404dd645 movd xmm1, r12d
0x1404dd64a punpckldq xmm1, xmm0
0x1404dd64e movd xmm0, r13d
0x1404dd653 movd xmm2, edi
0x1404dd657 punpckldq xmm0, xmm2
0x1404dd65b punpcklqdq xmm0, xmm1
0x1404dd65f movdqa xmm1, xmm0
0x1404dd663 pxor xmm1, xmm13
0x1404dd668 movdqa xmm2, xmm6
0x1404dd66c pcmpgtd xmm2, xmm1
0x1404dd670 pand xmm0, xmm2
0x1404dd674 pandn xmm2, xmm3
0x1404dd678 por xmm2, xmm0
0x1404dd67c movdqa xmm0, xmm2
0x1404dd680 pxor xmm0, xmm13
0x1404dd685 pcmpgtd xmm0, xmm4
0x1404dd689 pand xmm2, xmm0
0x1404dd68d pandn xmm0, xmm5
0x1404dd691 por xmm0, xmm2
0x1404dd695 add rax, rcx
0x1404dd698 mov rcx, qword ptr [rax + 0x10]
0x1404dd69c mov edx, dword ptr [r8 + 0x5e0]
0x1404dd6a3 movq qword ptr [rcx + 0x10], xmm15
0x1404dd6a9 mov dword ptr [rax + 0x5c], edx
0x1404dd6ac movq qword ptr [rcx + 8], xmm0
0x1404dd6b1 movd ebx, xmm15
0x1404dd6b6 pshufd xmm6, xmm15, 0x55
0x1404dd6bc mov rax, qword ptr [rbp + 0x1468]
0x1404dd6c3 cmp rax, qword ptr [rbp + 0x1498]
0x1404dd6ca jne 0x1404dd6f0
0x1404dd6cc cmp dword ptr [r8 + 0x1108], 1
0x1404dd6d4 jne 0x1404dd6f0
0x1404dd6d6 movd eax, xmm6
0x1404dd6da shl rax, 0x20
0x1404dd6de or rax, rbx
0x1404dd6e1 cmp qword ptr [r8 + 0x110c], rax
0x1404dd6e8 je 0x1404ddc37
0x1404dd6ee nop 
0x1404dd6f0 mov rcx, qword ptr [rbp + 0x1350]
0x1404dd6f7 call 0x140be76f0
0x1404dd6fc call 0x140d4d740
0x1404dd701 mov rsi, rax
0x1404dd704 test byte ptr [rax], 1
0x1404dd707 je 0x1404dd71a
0x1404dd709 mov rcx, rsi
0x1404dd70c add rcx, 8
0x1404dd710 mov rax, qword ptr [rsi + 8]
0x1404dd714 mov rdx, qword ptr [rsi + 0x10]
0x1404dd718 jmp 0x1404dd735
0x1404dd71a call 0x140d64e00
0x1404dd71f mov rcx, rsi
0x1404dd722 add rcx, 8
0x1404dd726 mov qword ptr [rsi], 1
0x1404dd72d mov qword ptr [rsi + 8], rax
0x1404dd731 mov qword ptr [rsi + 0x10], rdx
0x1404dd735 lea r8, [rax + 1]
0x1404dd739 mov qword ptr [rcx], r8
0x1404dd73c movups xmm14, xmmword ptr [rip + 0x250c56c]
0x1404dd744 movaps xmmword ptr [rbp + 0x610], xmm14
0x1404dd74c movups xmm8, xmmword ptr [rip + 0x250c54c]
0x1404dd754 movaps xmmword ptr [rbp + 0x600], xmm8
0x1404dd75c mov qword ptr [rbp + 0x620], rax
0x1404dd763 mov qword ptr [rbp + 0x628], rdx
0x1404dd76a call 0x140d4d740
0x1404dd76f mov rsi, rax
0x1404dd772 test byte ptr [rax], 1
0x1404dd775 je 0x1404dd788
0x1404dd777 mov rax, rsi
0x1404dd77a add rax, 8
0x1404dd77e mov r15, qword ptr [rsi + 8]
0x1404dd782 mov r14, qword ptr [rsi + 0x10]
0x1404dd786 jmp 0x1404dd7a9
0x1404dd788 call 0x140d64e00
0x1404dd78d mov r15, rax
0x1404dd790 mov r14, rdx
0x1404dd793 mov rax, rsi
0x1404dd796 add rax, 8
0x1404dd79a mov qword ptr [rsi], 1
0x1404dd7a1 mov qword ptr [rsi + 8], r15
0x1404dd7a5 mov qword ptr [rsi + 0x10], rdx
0x1404dd7a9 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd7b0 lea rcx, [r15 + 1]
0x1404dd7b4 mov qword ptr [rax], rcx
0x1404dd7b7 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd7be movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd7c5 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd7cc movaps xmmword ptr [rbp + 0xc50], xmm2
0x1404dd7d3 movaps xmmword ptr [rbp + 0xc40], xmm1
0x1404dd7da movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404dd7e1 lea rax, [rbp + 0xc38]
0x1404dd7e8 movups xmmword ptr [rax + 0x38], xmm14
0x1404dd7ed movups xmmword ptr [rax + 0x28], xmm8
0x1404dd7f2 movaps xmm0, xmmword ptr [rbp + 0xc30]
0x1404dd7f9 movaps xmm1, xmmword ptr [rbp + 0xc40]
0x1404dd800 movaps xmm2, xmmword ptr [rbp + 0xc50]
0x1404dd807 movaps xmm3, xmmword ptr [rbp + 0xc60]
0x1404dd80e movaps xmmword ptr [rbp + 0x600], xmm0
0x1404dd815 movaps xmmword ptr [rbp + 0x610], xmm1
0x1404dd81c movaps xmmword ptr [rbp + 0x620], xmm2
0x1404dd823 movaps xmmword ptr [rbp + 0x630], xmm3
0x1404dd82a movaps xmm0, xmmword ptr [rbp + 0xc70]
0x1404dd831 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404dd838 mov rax, qword ptr [rbp + 0x13c8]
0x1404dd83f cmp qword ptr [rax], 0
0x1404dd843 je 0x1404dd8c2
0x1404dd845 mov qword ptr [rbp + 0x1468], r15
0x1404dd84c mov qword ptr [rbp + 0x1498], r14
0x1404dd853 mov r8d, 0x68
0x1404dd859 mov r9d, 0x10
0x1404dd85f mov rcx, qword ptr [rbp + 0x13c8]
0x1404dd866 mov rdx, qword ptr [rbp + 0x1358]
0x1404dd86d call 0x14004ceb0
0x1404dd872 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd879 mov rdx, qword ptr [rdi + 0x8c8]
0x1404dd880 test rdx, rdx
0x1404dd883 mov r14, qword ptr [rbp + 0x1498]
0x1404dd88a mov r15, qword ptr [rbp + 0x1468]
0x1404dd891 je 0x1404dd8c2
0x1404dd893 mov rax, rdx
0x1404dd896 shl rax, 4
0x1404dd89a add rdx, rax
0x1404dd89d add rdx, 0x21
0x1404dd8a1 je 0x1404dd8c2
0x1404dd8a3 mov rcx, qword ptr [rbp + 0x1398]
0x1404dd8aa mov rcx, qword ptr [rcx]
0x1404dd8ad neg rax
0x1404dd8b0 add rcx, rax
0x1404dd8b3 add rcx, -0x10
0x1404dd8b7 mov r8d, 0x10
0x1404dd8bd call 0x140613c20
0x1404dd8c2 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404dd8c9 mov rax, qword ptr [rbp + 0x13c8]
0x1404dd8d0 movups xmmword ptr [rax + 0x40], xmm0
0x1404dd8d4 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd8db movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd8e2 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd8e9 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404dd8f0 movups xmmword ptr [rax + 0x30], xmm3
0x1404dd8f4 movups xmmword ptr [rax + 0x20], xmm2
0x1404dd8f8 movups xmmword ptr [rax + 0x10], xmm1
0x1404dd8fc movups xmmword ptr [rax], xmm0
0x1404dd8ff mov qword ptr [rdi + 0x8e0], r15
0x1404dd906 mov qword ptr [rdi + 0x8e8], r14
0x1404dd90d call 0x140d4d740
0x1404dd912 mov rsi, rax
0x1404dd915 test byte ptr [rax], 1
0x1404dd918 je 0x1404dd92b
0x1404dd91a mov rcx, rsi
0x1404dd91d add rcx, 8
0x1404dd921 mov rax, qword ptr [rsi + 8]
0x1404dd925 mov rdx, qword ptr [rsi + 0x10]
0x1404dd929 jmp 0x1404dd94d
0x1404dd92b call 0x140d64e00
0x1404dd930 mov rcx, rsi
0x1404dd933 add rcx, 8
0x1404dd937 mov qword ptr [rsi], 1
0x1404dd93e mov qword ptr [rsi + 8], rax
0x1404dd942 mov qword ptr [rsi + 0x10], rdx
0x1404dd946 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd94d lea r8, [rax + 1]
0x1404dd951 mov qword ptr [rcx], r8
0x1404dd954 movaps xmmword ptr [rbp + 0x610], xmm14
0x1404dd95c movaps xmmword ptr [rbp + 0x600], xmm8
0x1404dd964 mov qword ptr [rbp + 0x620], rax
0x1404dd96b mov qword ptr [rbp + 0x628], rdx
0x1404dd972 call 0x140d4d740
0x1404dd977 mov rsi, rax
0x1404dd97a test byte ptr [rax], 1
0x1404dd97d je 0x1404dd990
0x1404dd97f mov rax, rsi
0x1404dd982 add rax, 8
0x1404dd986 mov r15, qword ptr [rsi + 8]
0x1404dd98a mov r14, qword ptr [rsi + 0x10]
0x1404dd98e jmp 0x1404dd9b1
0x1404dd990 call 0x140d64e00
0x1404dd995 mov r15, rax
0x1404dd998 mov r14, rdx
0x1404dd99b mov rax, rsi
0x1404dd99e add rax, 8
0x1404dd9a2 mov qword ptr [rsi], 1
0x1404dd9a9 mov qword ptr [rsi + 8], r15
0x1404dd9ad mov qword ptr [rsi + 0x10], rdx
0x1404dd9b1 lea rcx, [r15 + 1]
0x1404dd9b5 mov qword ptr [rax], rcx
0x1404dd9b8 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd9bf movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd9c6 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd9cd movaps xmmword ptr [rbp + 0xc50], xmm2
0x1404dd9d4 movaps xmmword ptr [rbp + 0xc40], xmm1
0x1404dd9db movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404dd9e2 lea rax, [rbp + 0xc38]
0x1404dd9e9 movups xmmword ptr [rax + 0x38], xmm14
0x1404dd9ee movups xmmword ptr [rax + 0x28], xmm8
0x1404dd9f3 movaps xmm0, xmmword ptr [rbp + 0xc30]
0x1404dd9fa movaps xmm1, xmmword ptr [rbp + 0xc40]
0x1404dda01 movaps xmm2, xmmword ptr [rbp + 0xc50]
0x1404dda08 movaps xmm3, xmmword ptr [rbp + 0xc60]
0x1404dda0f movaps xmmword ptr [rbp + 0x600], xmm0
0x1404dda16 movaps xmmword ptr [rbp + 0x610], xmm1
0x1404dda1d movaps xmmword ptr [rbp + 0x620], xmm2
0x1404dda24 movaps xmmword ptr [rbp + 0x630], xmm3
0x1404dda2b movaps xmm0, xmmword ptr [rbp + 0xc70]
0x1404dda32 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404dda39 mov rax, qword ptr [rbp + 0x13d0]
0x1404dda40 cmp qword ptr [rax], 0
0x1404dda44 je 0x1404ddac3
0x1404dda46 mov qword ptr [rbp + 0x1468], r15
0x1404dda4d mov qword ptr [rbp + 0x1498], r14
0x1404dda54 mov r8d, 0x68
0x1404dda5a mov r9d, 0x10
0x1404dda60 mov rcx, qword ptr [rbp + 0x13d0]
0x1404dda67 mov rdx, qword ptr [rbp + 0x1360]
0x1404dda6e call 0x14004ceb0
0x1404dda73 mov rdi, qword ptr [rbp + 0x1478]
0x1404dda7a mov rdx, qword ptr [rdi + 0x9b8]
0x1404dda81 test rdx, rdx
0x1404dda84 mov r14, qword ptr [rbp + 0x1498]
0x1404dda8b mov r15, qword ptr [rbp + 0x1468]
0x1404dda92 je 0x1404ddac3
0x1404dda94 mov rax, rdx
0x1404dda97 shl rax, 4
0x1404dda9b add rdx, rax
0x1404dda9e add rdx, 0x21
0x1404ddaa2 je 0x1404ddac3
0x1404ddaa4 mov rcx, qword ptr [rbp + 0x13a0]
0x1404ddaab mov rcx, qword ptr [rcx]
0x1404ddaae neg rax
0x1404ddab1 add rcx, rax
0x1404ddab4 add rcx, -0x10
0x1404ddab8 mov r8d, 0x10
0x1404ddabe call 0x140613c20
0x1404ddac3 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404ddaca mov rax, qword ptr [rbp + 0x13d0]
0x1404ddad1 movups xmmword ptr [rax + 0x40], xmm0
0x1404ddad5 movdqa xmm0, xmmword ptr [rbp + 0x600]
0x1404ddadd movdqa xmm1, xmmword ptr [rbp + 0x610]
0x1404ddae5 movdqa xmm2, xmmword ptr [rbp + 0x620]
0x1404ddaed movdqa xmm3, xmmword ptr [rbp + 0x630]
0x1404ddaf5 movdqu xmmword ptr [rax + 0x30], xmm3
0x1404ddafa movdqu xmmword ptr [rax + 0x20], xmm2
0x1404ddaff movdqu xmmword ptr [rax + 0x10], xmm1
0x1404ddb04 movdqu xmmword ptr [rax], xmm0
0x1404ddb08 mov qword ptr [rdi + 0x9d0], r15
0x1404ddb0f mov qword ptr [rdi + 0x9d8], r14
0x1404ddb16 mov rcx, qword ptr [rbp + 0x1488]
0x1404ddb1d movabs rdx, 0x2ed189581b92decf
0x1404ddb27 movabs r8, 0x15d3d99396065f38
0x1404ddb31 call 0x1400dd6f0
0x1404ddb36 test rax, rax
0x1404ddb39 mov r8, qword ptr [rbp + 0x1478]
0x1404ddb40 je 0x1404df499
0x1404ddb46 cmp qword ptr [r8 + 0x248], rdx
0x1404ddb4d jbe 0x1404df499
0x1404ddb53 mov rax, qword ptr [r8 + 0x240]
0x1404ddb5a mov rcx, qword ptr [rax + rdx*8]
0x1404ddb5e test rcx, rcx
0x1404ddb61 je 0x1404df499
0x1404ddb67 mov rax, qword ptr [r8 + 0x210]
0x1404ddb6e not rcx
0x1404ddb71 lea rcx, [rcx + rcx*2]
0x1404ddb75 shl rcx, 5
0x1404ddb79 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddb7f je 0x1404df499
0x1404ddb85 add rax, rcx
0x1404ddb88 mov rcx, qword ptr [rax + 0x10]
0x1404ddb8c call 0x140987ca0
0x1404ddb91 mov rcx, rax
0x1404ddb94 call 0x140bf8e60
0x1404ddb99 mov rcx, qword ptr [rbp + 0x1488]
0x1404ddba0 movabs rdx, 0xf08e33646236262a
0x1404ddbaa movabs r8, 0xd96640f7ddf9c127
0x1404ddbb4 call 0x1400dd6f0
0x1404ddbb9 test rax, rax
0x1404ddbbc mov r8, qword ptr [rbp + 0x1478]
0x1404ddbc3 je 0x1404df4aa
0x1404ddbc9 cmp qword ptr [r8 + 0x248], rdx
0x1404ddbd0 jbe 0x1404df4aa
0x1404ddbd6 mov rax, qword ptr [r8 + 0x240]
0x1404ddbdd mov rcx, qword ptr [rax + rdx*8]
0x1404ddbe1 test rcx, rcx
0x1404ddbe4 je 0x1404df4aa
0x1404ddbea mov rax, qword ptr [r8 + 0x210]
0x1404ddbf1 not rcx
0x1404ddbf4 lea rcx, [rcx + rcx*2]
0x1404ddbf8 shl rcx, 5
0x1404ddbfc cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddc02 je 0x1404df4aa
0x1404ddc08 add rax, rcx
0x1404ddc0b mov rcx, qword ptr [rax + 0x10]
0x1404ddc0f mov edx, dword ptr [r8 + 0x5e0]
0x1404ddc16 mov dword ptr [rax + 0x5c], edx
0x1404ddc19 mov dword ptr [rcx + 0x108], 0
0x1404ddc23 mov dword ptr [r8 + 0x1108], 1
0x1404ddc2e movq qword ptr [r8 + 0x110c], xmm15
0x1404ddc37 call 0x140c16a00
0x1404ddc3c test al, al
0x1404ddc3e mov qword ptr [rbp + 0x1460], rbx
0x1404ddc45 je 0x1404ddcf0
0x1404ddc4b lea rsi, [rip + 0x250c090]
0x1404ddc52 lea r9, [rip + 0x250c067]
0x1404ddc59 nop dword ptr [rax]
0x1404ddc60 movsx eax, byte ptr [rsi - 1]
0x1404ddc64 test eax, eax
0x1404ddc66 js 0x1404ddc80
0x1404ddc68 dec rsi
0x1404ddc6b cmp eax, 0x5c
0x1404ddc6e jne 0x1404ddcd2
0x1404ddc70 jmp 0x1404ddcf7
0x1404ddc75 nop word ptr cs:[rax + rax]
0x1404ddc80 movzx ecx, byte ptr [rsi - 2]
0x1404ddc84 cmp cl, 0xc0
0x1404ddc87 jge 0x1404ddcab
0x1404ddc89 movzx edx, byte ptr [rsi - 3]
0x1404ddc8d cmp dl, 0xc0
0x1404ddc90 jge 0x1404ddcb4
0x1404ddc92 movzx r8d, byte ptr [rsi - 4]
0x1404ddc97 add rsi, -4
0x1404ddc9b and r8d, 7
0x1404ddc9f shl r8d, 6
0x1404ddca3 and edx, 0x3f
0x1404ddca6 or edx, r8d
0x1404ddca9 jmp 0x1404ddcbb
0x1404ddcab add rsi, -2
0x1404ddcaf and ecx, 0x1f
0x1404ddcb2 jmp 0x1404ddcc3
0x1404ddcb4 add rsi, -3
0x1404ddcb8 and edx, 0xf
0x1404ddcbb shl edx, 6
0x1404ddcbe and ecx, 0x3f
0x1404ddcc1 or ecx, edx
0x1404ddcc3 shl ecx, 6
0x1404ddcc6 and al, 0x3f
0x1404ddcc8 movzx eax, al
0x1404ddccb or eax, ecx
0x1404ddccd cmp eax, 0x5c
0x1404ddcd0 je 0x1404ddcf7
0x1404ddcd2 cmp eax, 0x2f
0x1404ddcd5 je 0x1404ddcf7
0x1404ddcd7 cmp rsi, r9
0x1404ddcda jne 0x1404ddc60
0x1404ddcdc mov edi, 0x22
0x1404ddce1 mov rsi, r9
0x1404ddce4 jmp 0x1404ddd22
0x1404ddce6 nop word ptr cs:[rax + rax]
0x1404ddcf0 xor eax, eax
0x1404ddcf2 jmp 0x1404dddb2
0x1404ddcf7 sub rsi, r9
0x1404ddcfa inc rsi
0x1404ddcfd je 0x1404ddd17
0x1404ddcff cmp rsi, 0x22
0x1404ddd03 jb 0x1404ddd0c
0x1404ddd05 je 0x1404ddd17
0x1404ddd07 jmp 0x1404df5b3
0x1404ddd0c cmp byte ptr [rsi + r9], 0xbf
0x1404ddd11 jle 0x1404df5b3
0x1404ddd17 mov edi, 0x22
0x1404ddd1c sub rdi, rsi
0x1404ddd1f add rsi, r9
0x1404ddd22 call 0x140c18130
0x1404ddd27 mov rcx, rax
0x1404ddd2a mov rax, qword ptr [rax]
0x1404ddd2d test rax, rax
0x1404ddd30 je 0x1404ddd41
0x1404ddd32 cmp eax, 1
0x1404ddd35 jne 0x1404df580
0x1404ddd3b add rcx, 8
0x1404ddd3f jmp 0x1404ddd54
0x1404ddd41 xor edx, edx
0x1404ddd43 call 0x1428a1640
0x1404ddd48 mov rcx, rax
0x1404ddd4b test rax, rax
0x1404ddd4e je 0x1404df580
0x1404ddd54 cmp qword ptr [rcx], 0
0x1404ddd58 jne 0x1404deede
0x1404ddd5e mov qword ptr [rcx], 0xffffffffffffffff
0x1404ddd65 mov qword ptr [rbp + 0x1498], rcx
0x1404ddd6c add rcx, 8
0x1404ddd70 mov qword ptr [rsp + 0x20], rdi
0x1404ddd75 mov qword ptr [rsp + 0x30], 0
0x1404ddd7e mov qword ptr [rsp + 0x28], 1
0x1404ddd87 mov r8d, 9
0x1404ddd8d lea rdx, [rip + 0x250c37c]
0x1404ddd94 mov r9, rsi
0x1404ddd97 call 0x140c16d40
0x1404ddd9c mov rcx, qword ptr [rbp + 0x1498]
0x1404ddda3 inc qword ptr [rcx]
0x1404ddda6 mov qword ptr [rbp + 0x13e8], rax
0x1404dddad mov eax, 1
0x1404dddb2 mov r15, qword ptr [rbp + 0x1478]
0x1404dddb9 mov r13, qword ptr [rbp + 0x1450]
0x1404dddc0 mov qword ptr [rbp + 0x13e0], rax
0x1404dddc7 mov rax, qword ptr [rip + 0x314e52a]
0x1404dddce mov rax, qword ptr [rax]
0x1404dddd1 cmp rax, 2
0x1404dddd5 je 0x1404ddde6
0x1404dddd7 mov rcx, qword ptr [rip + 0x314e51a]
0x1404dddde mov rdx, rcx
0x1404ddde1 call 0x1428a4550
0x1404ddde6 mov rax, qword ptr [rip + 0x314e50b]
0x1404ddded mov rsi, qword ptr [rax + 8]
0x1404dddf1 mov rax, qword ptr [rax]
0x1404dddf4 cmp rax, 2
0x1404dddf8 je 0x1404dde09
0x1404dddfa mov rcx, qword ptr [rip + 0x314e4f7]
0x1404dde01 mov rdx, rcx
0x1404dde04 call 0x1428a4550
0x1404dde09 mov rax, qword ptr [rip + 0x314e4e8]
0x1404dde10 lea rcx, [rax + 0x10]
0x1404dde14 call 0x140d63910
0x1404dde19 mov dword ptr [rbp + 0x1468], edx
0x1404dde1f mov r14, rax
0x1404dde22 call 0x140c17420
0x1404dde27 mov qword ptr [rbp + 0x1498], rax
0x1404dde2e mov rcx, rax
0x1404dde31 add rcx, 8
0x1404dde35 call 0x140c17480
0x1404dde3a mov rdi, qword ptr [rbp + 0x1498]
0x1404dde41 mov rcx, rdi
0x1404dde44 call 0x140c81e10
0x1404dde49 mov al, 1
0x1404dde4b xor ecx, ecx
0x1404dde4d lock cmpxchg byte ptr [rdi], cl
0x1404dde51 jne 0x1404ded86
0x1404dde57 mov rcx, qword ptr [rbp + 0x1488]
0x1404dde5e movabs rdx, 0x905c303950d415d1
0x1404dde68 movabs r8, 0x4667de267035c872
0x1404dde72 call 0x1400dd6f0
0x1404dde77 mov r12b, 1
0x1404dde7a test rax, rax
0x1404dde7d je 0x1404de1b0
0x1404dde83 cmp qword ptr [r15 + 0x248], rdx
0x1404dde8a jbe 0x1404de1b0
0x1404dde90 mov rax, qword ptr [r15 + 0x240]
0x1404dde97 mov rcx, qword ptr [rax + rdx*8]
0x1404dde9b test rcx, rcx
0x1404dde9e je 0x1404de1b0
0x1404ddea4 mov rax, qword ptr [r15 + 0x210]
0x1404ddeab not rcx
0x1404ddeae lea rcx, [rcx + rcx*2]
0x1404ddeb2 shl rcx, 5
0x1404ddeb6 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddebc je 0x1404de1b0
0x1404ddec2 add rax, rcx
0x1404ddec5 mov rdi, qword ptr [rax + 0x10]
0x1404ddec9 mov rcx, qword ptr [rbp + 0x1488]
0x1404dded0 movabs rdx, 0x48b321ef779dc301
0x1404ddeda movabs r8, 0xbd65767194d9ddc8
0x1404ddee4 call 0x1400dd6f0
0x1404ddee9 test rax, rax
0x1404ddeec je 0x1404ddf42
0x1404ddeee cmp qword ptr [r15 + 0x248], rdx
0x1404ddef5 jbe 0x1404ddf42
0x1404ddef7 mov rax, qword ptr [r15 + 0x240]
0x1404ddefe mov rcx, qword ptr [rax + rdx*8]
0x1404ddf02 test rcx, rcx
0x1404ddf05 je 0x1404ddf42
0x1404ddf07 mov rax, qword ptr [r15 + 0x210]
0x1404ddf0e not rcx
0x1404ddf11 lea rcx, [rcx + rcx*2]
0x1404ddf15 shl rcx, 5
0x1404ddf19 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddf1f je 0x1404ddf42
0x1404ddf21 add rax, rcx
0x1404ddf24 mov rax, qword ptr [rax + 0x10]
0x1404ddf28 cmp byte ptr [rax + 0xf0], 0
0x1404ddf2f jne 0x1404ded31
0x1404ddf35 test byte ptr [rax + 0x101], 1
0x1404ddf3c jne 0x1404ded31
0x1404ddf42 lea rcx, [rbp + 0x600]
0x1404ddf49 call 0x140c2c770
0x1404ddf4e mov byte ptr [rbp + 0x14a3], 1
0x1404ddf55 call 0x140c16a00
0x1404ddf5a lea r9, [rip + 0x250bd5f]
0x1404ddf61 test al, al
0x1404ddf63 mov byte ptr [rbp + 0x1498], al
0x1404ddf69 je 0x1404de008
0x1404ddf6f lea r13, [rip + 0x250bd6c]
0x1404ddf76 nop word ptr cs:[rax + rax]
0x1404ddf80 movsx eax, byte ptr [r13 - 1]
0x1404ddf85 test eax, eax
0x1404ddf87 js 0x1404ddfa0
0x1404ddf89 dec r13
0x1404ddf8c cmp eax, 0x5c
0x1404ddf8f jne 0x1404ddff4
0x1404ddf91 jmp 0x1404de00f
0x1404ddf93 nop word ptr cs:[rax + rax]
0x1404ddfa0 movzx ecx, byte ptr [r13 - 2]
0x1404ddfa5 cmp cl, 0xc0
0x1404ddfa8 jge 0x1404ddfcd
0x1404ddfaa movzx edx, byte ptr [r13 - 3]
0x1404ddfaf cmp dl, 0xc0
0x1404ddfb2 jge 0x1404ddfd6
0x1404ddfb4 movzx r8d, byte ptr [r13 - 4]
0x1404ddfb9 add r13, -4
0x1404ddfbd and r8d, 7
0x1404ddfc1 shl r8d, 6
0x1404ddfc5 and edx, 0x3f
0x1404ddfc8 or edx, r8d
0x1404ddfcb jmp 0x1404ddfdd
0x1404ddfcd add r13, -2
0x1404ddfd1 and ecx, 0x1f
0x1404ddfd4 jmp 0x1404ddfe5
0x1404ddfd6 add r13, -3
0x1404ddfda and edx, 0xf
0x1404ddfdd shl edx, 6
0x1404ddfe0 and ecx, 0x3f
0x1404ddfe3 or ecx, edx
0x1404ddfe5 shl ecx, 6
0x1404ddfe8 and al, 0x3f
0x1404ddfea movzx eax, al
0x1404ddfed or eax, ecx
0x1404ddfef cmp eax, 0x5c
0x1404ddff2 je 0x1404de00f
0x1404ddff4 cmp eax, 0x2f
0x1404ddff7 je 0x1404de00f
0x1404ddff9 cmp r13, r9
0x1404ddffc jne 0x1404ddf80
0x1404ddffe mov ebx, 0x22
0x1404de003 mov r13, r9
0x1404de006 jmp 0x1404de03b
0x1404de008 xor eax, eax
0x1404de00a jmp 0x1404de0cf
0x1404de00f sub r13, r9
0x1404de012 inc r13
0x1404de015 je 0x1404de030
0x1404de017 cmp r13, 0x22
0x1404de01b jb 0x1404de024
0x1404de01d je 0x1404de030
0x1404de01f jmp 0x1404df62d
0x1404de024 cmp byte ptr [r13 + r9], 0xbf
0x1404de02a jle 0x1404df62d
0x1404de030 mov ebx, 0x22
0x1404de035 sub rbx, r13
0x1404de038 add r13, r9
0x1404de03b call 0x140c18130
0x1404de040 mov rcx, rax
0x1404de043 mov rax, qword ptr [rax]
0x1404de046 test rax, rax
0x1404de049 je 0x1404de05a
0x1404de04b cmp eax, 1
0x1404de04e jne 0x1404df603
0x1404de054 add rcx, 8
0x1404de058 jmp 0x1404de074
0x1404de05a mov byte ptr [rbp + 0x14a3], 1
0x1404de061 xor edx, edx
0x1404de063 call 0x1428a1640
0x1404de068 mov rcx, rax
0x1404de06b test rax, rax
0x1404de06e je 0x1404df603
0x1404de074 cmp qword ptr [rcx], 0
0x1404de078 jne 0x1404def2e
0x1404de07e mov qword ptr [rcx], 0xffffffffffffffff
0x1404de085 mov qword ptr [rbp + 0x1448], rcx
0x1404de08c add rcx, 8
0x1404de090 mov qword ptr [rsp + 0x20], rbx
0x1404de095 mov qword ptr [rsp + 0x30], 0
0x1404de09e mov qword ptr [rsp + 0x28], 1
0x1404de0a7 mov r8d, 0x10
0x1404de0ad lea rdx, [rip + 0x250c065]
0x1404de0b4 mov r9, r13
0x1404de0b7 call 0x140c16d40
0x1404de0bc mov rcx, qword ptr [rbp + 0x1448]
0x1404de0c3 inc qword ptr [rcx]
0x1404de0c6 mov qword ptr [rbp - 0x38], rax
0x1404de0ca mov eax, 1
0x1404de0cf mov r15, qword ptr [rbp + 0x1478]
0x1404de0d6 lea rbx, [rbp + 0xc30]
0x1404de0dd mov qword ptr [rbp - 0x40], rax
0x1404de0e1 mov byte ptr [rbp + 0x1472], 1
0x1404de0e8 mov rcx, rdi
0x1404de0eb call 0x14067dd80
0x1404de0f0 mov rdi, rax
0x1404de0f3 mov r8d, 0xb0
0x1404de0f9 mov rcx, rbx
0x1404de0fc lea rdx, [rbp + 0x600]
0x1404de103 call 0x142923c00
0x1404de108 mov byte ptr [rbp + 0x1472], 0
0x1404de10f mov rcx, rdi
0x1404de112 mov rdx, rbx
0x1404de115 call 0x140c61300
0x1404de11a cmp byte ptr [rbp + 0x1498], 0
0x1404de121 mov r13, qword ptr [rbp + 0x1450]
0x1404de128 je 0x1404de1a0
0x1404de12a call 0x140c18130
0x1404de12f mov rcx, rax
0x1404de132 mov rax, qword ptr [rax]
0x1404de135 test rax, rax
0x1404de138 je 0x1404de149
0x1404de13a cmp eax, 1
0x1404de13d jne 0x1404df618
0x1404de143 add rcx, 8
0x1404de147 jmp 0x1404de163
0x1404de149 mov byte ptr [rbp + 0x14a3], 0
0x1404de150 xor edx, edx
0x1404de152 call 0x1428a1640
0x1404de157 mov rcx, rax
0x1404de15a test rax, rax
0x1404de15d je 0x1404df618
0x1404de163 cmp qword ptr [rcx], 0
0x1404de167 jne 0x1404def46
0x1404de16d mov qword ptr [rcx], 0xffffffffffffffff
0x1404de174 mov qword ptr [rbp + 0x1498], rcx
0x1404de17b add rcx, 8
0x1404de17f mov rdx, qword ptr [rbp - 0x38]
0x1404de183 call 0x140c170f0
0x1404de188 mov r15, qword ptr [rbp + 0x1478]
0x1404de18f mov rax, qword ptr [rbp + 0x1498]
0x1404de196 inc qword ptr [rax]
0x1404de199 mov r13, qword ptr [rbp + 0x1450]
0x1404de1a0 xor r12d, r12d
0x1404de1a3 nop word ptr cs:[rax + rax]
0x1404de1b0 call 0x140c16a00
0x1404de1b5 mov byte ptr [rbp + 0x1498], al
0x1404de1bb test al, al
0x1404de1bd je 0x1404de270
0x1404de1c3 lea rdi, [rip + 0x250bb18]
0x1404de1ca nop word ptr [rax + rax]
0x1404de1d0 movsx eax, byte ptr [rdi - 1]
0x1404de1d4 test eax, eax
0x1404de1d6 js 0x1404de1f0
0x1404de1d8 dec rdi
0x1404de1db cmp eax, 0x5c
0x1404de1de jne 0x1404de242
0x1404de1e0 jmp 0x1404de277
0x1404de1e5 nop word ptr cs:[rax + rax]
0x1404de1f0 movzx ecx, byte ptr [rdi - 2]
0x1404de1f4 cmp cl, 0xc0
0x1404de1f7 jge 0x1404de21b
0x1404de1f9 movzx edx, byte ptr [rdi - 3]
0x1404de1fd cmp dl, 0xc0
0x1404de200 jge 0x1404de224
0x1404de202 movzx r8d, byte ptr [rdi - 4]
0x1404de207 add rdi, -4
0x1404de20b and r8d, 7
0x1404de20f shl r8d, 6
0x1404de213 and edx, 0x3f
0x1404de216 or edx, r8d
0x1404de219 jmp 0x1404de22b
0x1404de21b add rdi, -2
0x1404de21f and ecx, 0x1f
0x1404de222 jmp 0x1404de233
0x1404de224 add rdi, -3
0x1404de228 and edx, 0xf
0x1404de22b shl edx, 6
0x1404de22e and ecx, 0x3f
0x1404de231 or ecx, edx
0x1404de233 shl ecx, 6
0x1404de236 and al, 0x3f
0x1404de238 movzx eax, al
0x1404de23b or eax, ecx
0x1404de23d cmp eax, 0x5c
0x1404de240 je 0x1404de277
0x1404de242 cmp eax, 0x2f
0x1404de245 je 0x1404de277
0x1404de247 lea rax, [rip + 0x250ba72]
0x1404de24e cmp rdi, rax
0x1404de251 jne 0x1404de1d0
0x1404de257 mov ebx, 0x22
0x1404de25c lea rdi, [rip + 0x250ba5d]
0x1404de263 jmp 0x1404de2af
0x1404de265 nop word ptr cs:[rax + rax]
0x1404de270 xor eax, eax
0x1404de272 jmp 0x1404de33f
0x1404de277 lea rax, [rip + 0x250ba42]
0x1404de27e sub rdi, rax
0x1404de281 inc rdi
0x1404de284 je 0x1404de2a4
0x1404de286 lea rax, [rip + 0x250ba33]
0x1404de28d cmp rdi, 0x22
0x1404de291 jb 0x1404de29a
0x1404de293 je 0x1404de2a4
0x1404de295 jmp 0x1404df5db
0x1404de29a cmp byte ptr [rdi + rax], 0xbf
0x1404de29e jle 0x1404df5db
0x1404de2a4 mov ebx, 0x22
0x1404de2a9 sub rbx, rdi
0x1404de2ac add rdi, rax
0x1404de2af call 0x140c18130
0x1404de2b4 mov rcx, rax
0x1404de2b7 mov rax, qword ptr [rax]
0x1404de2ba test rax, rax
0x1404de2bd je 0x1404de2ce
0x1404de2bf cmp eax, 1
0x1404de2c2 jne 0x1404df591
0x1404de2c8 add rcx, 8
0x1404de2cc jmp 0x1404de2e1
0x1404de2ce xor edx, edx
0x1404de2d0 call 0x1428a1640
0x1404de2d5 mov rcx, rax
0x1404de2d8 test rax, rax
0x1404de2db je 0x1404df591
0x1404de2e1 cmp qword ptr [rcx], 0
0x1404de2e5 jne 0x1404deeef
0x1404de2eb mov qword ptr [rcx], 0xffffffffffffffff
0x1404de2f2 mov qword ptr [rbp + 0x1448], rcx
0x1404de2f9 add rcx, 8
0x1404de2fd mov qword ptr [rsp + 0x20], rbx
0x1404de302 mov qword ptr [rsp + 0x30], 0
0x1404de30b mov qword ptr [rsp + 0x28], 1
0x1404de314 mov r8d, 0xa
0x1404de31a lea rdx, [rip + 0x250be08]
0x1404de321 mov r9, rdi
0x1404de324 call 0x140c16d40
0x1404de329 mov rcx, qword ptr [rbp + 0x1448]
0x1404de330 inc qword ptr [rcx]
0x1404de333 mov qword ptr [rbp + 0xc38], rax
0x1404de33a mov eax, 1
0x1404de33f mov qword ptr [rbp + 0xc30], rax
0x1404de346 mov rcx, r13
0x1404de349 call 0x140cad8a0
0x1404de34e cmp byte ptr [rbp + 0x1498], 0
0x1404de355 je 0x1404de3bb
0x1404de357 call 0x140c18130
0x1404de35c mov rcx, rax
0x1404de35f mov rax, qword ptr [rax]
0x1404de362 test rax, rax
0x1404de365 je 0x1404de376
0x1404de367 cmp eax, 1
0x1404de36a jne 0x1404df5a2
0x1404de370 add rcx, 8
0x1404de374 jmp 0x1404de389
0x1404de376 xor edx, edx
0x1404de378 call 0x1428a1640
0x1404de37d mov rcx, rax
0x1404de380 test rax, rax
0x1404de383 je 0x1404df5a2
0x1404de389 cmp qword ptr [rcx], 0
0x1404de38d jne 0x1404def00
0x1404de393 mov qword ptr [rcx], 0xffffffffffffffff
0x1404de39a mov qword ptr [rbp + 0x1498], rcx
0x1404de3a1 add rcx, 8
0x1404de3a5 mov rdx, qword ptr [rbp + 0xc38]
0x1404de3ac call 0x140c170f0
0x1404de3b1 mov rax, qword ptr [rbp + 0x1498]
0x1404de3b8 inc qword ptr [rax]
0x1404de3bb test r12b, r12b
0x1404de3be jne 0x1404de4e0
0x1404de3c4 lea rcx, [rbp + 0xc30]
0x1404de3cb mov rdx, qword ptr [rbp + 0x1390]
0x1404de3d2 call 0x1400dd910
0x1404de3d7 mov rax, qword ptr [rbp + 0xca0]
0x1404de3de mov qword ptr [rbp + 0x670], rax
0x1404de3e5 movups xmm0, xmmword ptr [rbp + 0xc90]
0x1404de3ec movaps xmmword ptr [rbp + 0x660], xmm0
0x1404de3f3 movups xmm0, xmmword ptr [rbp + 0xc80]
0x1404de3fa movaps xmmword ptr [rbp + 0x650], xmm0
0x1404de401 movups xmm0, xmmword ptr [rbp + 0xc70]
0x1404de408 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404de40f movdqu xmm0, xmmword ptr [rbp + 0xc30]
0x1404de417 movdqu xmm1, xmmword ptr [rbp + 0xc40]
0x1404de41f movdqu xmm2, xmmword ptr [rbp + 0xc50]
0x1404de427 movdqu xmm3, xmmword ptr [rbp + 0xc60]
0x1404de42f movdqa xmmword ptr [rbp + 0x630], xmm3
0x1404de437 movdqa xmmword ptr [rbp + 0x620], xmm2
0x1404de43f movdqa xmmword ptr [rbp + 0x610], xmm1
0x1404de447 movdqa xmmword ptr [rbp + 0x600], xmm0
0x1404de44f lea rcx, [rbp + 0xc30]
0x1404de456 mov rdx, qword ptr [rbp + 0x1418]
0x1404de45d lea r8, [rbp + 0x600]
0x1404de464 call 0x1403c5a10
0x1404de469 mov rax, qword ptr [rbp + 0xc30]
0x1404de470 movabs rcx, 0x8000000000000007
0x1404de47a cmp rax, rcx
0x1404de47d je 0x1404de4e0
0x1404de47f movabs rcx, 0x8000000000000000
0x1404de489 xor rax, rcx
0x1404de48c cmp rax, 7
0x1404de490 mov ecx, 3
0x1404de495 cmovae rax, rcx
0x1404de499 cmp rax, 4
0x1404de49d je 0x1404de4b3
0x1404de49f cmp rax, 3
0x1404de4a3 jne 0x1404de4e0
0x1404de4a5 lea rcx, [rbp + 0xc30]
0x1404de4ac call 0x1405f0d10
0x1404de4b1 jmp 0x1404de4e0
0x1404de4b3 mov rdx, qword ptr [rbp + 0xc38]
0x1404de4ba movabs rax, 0x8000000000000002
0x1404de4c4 cmp rdx, rax
0x1404de4c7 jl 0x1404de4e0
0x1404de4c9 test rdx, rdx
0x1404de4cc je 0x1404de4e0
0x1404de4ce mov rcx, qword ptr [rbp + 0xc40]
0x1404de4d5 mov r8d, 1
0x1404de4db call 0x140613c20
0x1404de4e0 mov rax, qword ptr [rbp + 0x1388]
0x1404de4e7 cmp qword ptr [rax], 0
0x1404de4eb je 0x1404de4f9
0x1404de4ed mov rcx, qword ptr [rbp + 0x1388]
0x1404de4f4 call 0x1400e6f90
0x1404de4f9 mov rcx, qword ptr [rbp + 0x1488]
0x1404de500 movabs rdx, 0xbba451ee6a08a3ff
0x1404de50a movabs r8, 0x8d008d15febcf29d
0x1404de514 call 0x1400dd6f0
0x1404de519 test rax, rax
0x1404de51c je 0x1404de560
0x1404de51e cmp qword ptr [r15 + 0x248], rdx
0x1404de525 jbe 0x1404de560
0x1404de527 mov rax, qword ptr [r15 + 0x240]
0x1404de52e mov rax, qword ptr [rax + rdx*8]
0x1404de532 test rax, rax
0x1404de535 je 0x1404de560
0x1404de537 mov rcx, qword ptr [r15 + 0x210]
0x1404de53e not rax
0x1404de541 lea rax, [rax + rax*2]
0x1404de545 shl rax, 5
0x1404de549 cmp qword ptr [rcx + rax + 0x20], 0
0x1404de54f setne al
0x1404de552 add al, al
0x1404de554 jmp 0x1404de562
0x1404de556 nop word ptr cs:[rax + rax]
0x1404de560 xor eax, eax
0x1404de562 imul rcx, r14, 0x3b9aca00
0x1404de569 mov edx, dword ptr [rbp + 0x1468]
0x1404de56f add rsi, rdx
0x1404de572 add rsi, rcx
0x1404de575 movd edx, xmm6
0x1404de579 shl rdx, 0x20
0x1404de57d mov rcx, qword ptr [rbp + 0x1460]
0x1404de584 or rcx, rdx
0x1404de587 mov byte ptr [rsp + 0x38], al
0x1404de58b mov rax, qword ptr [rbp + 0x1390]
0x1404de592 mov qword ptr [rsp + 0x30], rax
0x1404de597 mov rax, qword ptr [rbp + 0x1420]
0x1404de59e mov qword ptr [rsp + 0x28], rax
0x1404de5a3 mov rax, qword ptr [rbp + 0x1338]
0x1404de5aa mov qword ptr [rsp + 0x20], rax
0x1404de5af mov rdx, r13
0x1404de5b2 mov r8, rsi
0x1404de5b5 mov r9, qword ptr [rbp + 0x1330]
0x1404de5bc call 0x1404e8010
0x1404de5c1 mov rcx, qword ptr [rbp + 0x1488]
0x1404de5c8 movabs rdx, 0xf08e33646236262a
0x1404de5d2 movabs r8, 0xd96640f7ddf9c127
0x1404de5dc call 0x1400dd6f0
0x1404de5e1 test rax, rax
0x1404de5e4 je 0x1404df39e
0x1404de5ea cmp qword ptr [r15 + 0x248], rdx
0x1404de5f1 jbe 0x1404df39e
0x1404de5f7 mov rax, qword ptr [r15 + 0x240]
0x1404de5fe mov rcx, qword ptr [rax + rdx*8]
0x1404de602 test rcx, rcx
0x1404de605 je 0x1404df39e
0x1404de60b mov rax, qword ptr [r15 + 0x210]
0x1404de612 not rcx
0x1404de615 lea rcx, [rcx + rcx*2]
0x1404de619 shl rcx, 5
0x1404de61d cmp qword ptr [rax + rcx + 0x20], 0
0x1404de623 je 0x1404df39e
0x1404de629 add rax, rcx
0x1404de62c mov rcx, qword ptr [rax + 0x10]
0x1404de630 mov edx, dword ptr [r15 + 0x5e0]
0x1404de637 mov dword ptr [rax + 0x5c], edx
0x1404de63a inc dword ptr [rcx + 0x108]
0x1404de640 call 0x1403db200
0x1404de645 mov rcx, qword ptr [rbp + 0x1488]
0x1404de64c movabs rdx, 0x3d7b56e787d47c40
0x1404de656 movabs r8, 0x67b0b10a8d43bfc7
0x1404de660 call 0x1400dd6f0
0x1404de665 test rax, rax
0x1404de668 je 0x1404df3af
0x1404de66e cmp qword ptr [r15 + 0x248], rdx
0x1404de675 jbe 0x1404df3af
0x1404de67b mov rax, qword ptr [r15 + 0x240]
0x1404de682 mov rcx, qword ptr [rax + rdx*8]
0x1404de686 test rcx, rcx
0x1404de689 je 0x1404df3af
0x1404de68f mov rax, qword ptr [r15 + 0x210]
0x1404de696 not rcx
0x1404de699 lea rcx, [rcx + rcx*2]
0x1404de69d shl rcx, 5
0x1404de6a1 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de6a7 je 0x1404df3af
0x1404de6ad add rax, rcx
0x1404de6b0 mov rax, qword ptr [rax + 0x10]
0x1404de6b4 movss xmm6, dword ptr [rax + 0x98]
0x1404de6bc mov rcx, qword ptr [rbp + 0x1488]
0x1404de6c3 movabs rdx, 0xd38dae617fb79c7a
0x1404de6cd movabs r8, 0x22aadf5c5dad1eae
0x1404de6d7 call 0x1400dd6f0
0x1404de6dc test rax, rax
0x1404de6df je 0x1404de7d0
0x1404de6e5 cmp qword ptr [r15 + 0x248], rdx
0x1404de6ec jbe 0x1404de7d0
0x1404de6f2 mov rax, qword ptr [r15 + 0x240]
0x1404de6f9 mov rcx, qword ptr [rax + rdx*8]
0x1404de6fd test rcx, rcx
0x1404de700 je 0x1404de7d0
0x1404de706 mov rax, qword ptr [r15 + 0x210]
0x1404de70d not rcx
0x1404de710 lea rcx, [rcx + rcx*2]
0x1404de714 shl rcx, 5
0x1404de718 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de71e je 0x1404de7d0
0x1404de724 add rax, rcx
0x1404de727 mov rsi, qword ptr [rax + 0x10]
0x1404de72b mov rcx, rsi
0x1404de72e call 0x14093fbc0
0x1404de733 test al, al
0x1404de735 je 0x1404de7d0
0x1404de73b mov rcx, rsi
0x1404de73e call 0x14093fc60
0x1404de743 test al, al
0x1404de745 je 0x1404de7d0
0x1404de74b mov rcx, qword ptr [rbp + 0x1488]
0x1404de752 movabs rdx, 0x6831e8f98c915159
0x1404de75c movabs r8, 0xd515c479e4000725
0x1404de766 call 0x1400dd6f0
0x1404de76b test rax, rax
0x1404de76e je 0x1404de7b3
0x1404de770 cmp qword ptr [r15 + 0x248], rdx
0x1404de777 jbe 0x1404de7b3
0x1404de779 mov rax, qword ptr [r15 + 0x240]
0x1404de780 mov rcx, qword ptr [rax + rdx*8]
0x1404de784 test rcx, rcx
0x1404de787 je 0x1404de7b3
0x1404de789 mov rax, qword ptr [r15 + 0x210]
0x1404de790 not rcx
0x1404de793 lea rcx, [rcx + rcx*2]
0x1404de797 shl rcx, 5
0x1404de79b cmp qword ptr [rax + rcx + 0x20], 0
0x1404de7a1 je 0x1404de7b3
0x1404de7a3 add rax, rcx
0x1404de7a6 mov rcx, qword ptr [rax + 0x10]
0x1404de7aa call 0x14091f0f0
0x1404de7af test al, al
0x1404de7b1 jne 0x1404de7d0
0x1404de7b3 ucomiss xmm6, xmm11
0x1404de7b7 movd xmm1, dword ptr [rip + 0x24f310d]
0x1404de7bf jbe 0x1404de7d3
0x1404de7c1 minss xmm6, dword ptr [rip + 0x24f3103]
0x1404de7c9 nop dword ptr [rax]
0x1404de7d0 movaps xmm1, xmm6
0x1404de7d3 mov rcx, qword ptr [rbp + 0x1340]
0x1404de7da call 0x140270850
0x1404de7df cmp qword ptr [rbp + 0x13e0], 0
0x1404de7e7 je 0x1404de84d
0x1404de7e9 call 0x140c18130
0x1404de7ee mov rcx, rax
0x1404de7f1 mov rax, qword ptr [rax]
0x1404de7f4 test rax, rax
0x1404de7f7 je 0x1404de808
0x1404de7f9 cmp eax, 1
0x1404de7fc jne 0x1404df56f
0x1404de802 add rcx, 8
0x1404de806 jmp 0x1404de81b
0x1404de808 xor edx, edx
0x1404de80a call 0x1428a1640
0x1404de80f mov rcx, rax
0x1404de812 test rax, rax
0x1404de815 je 0x1404df56f
0x1404de81b cmp qword ptr [rcx], 0
0x1404de81f jne 0x1404deecd
0x1404de825 mov qword ptr [rcx], 0xffffffffffffffff
0x1404de82c mov qword ptr [rbp + 0x1498], rcx
0x1404de833 add rcx, 8
0x1404de837 mov rdx, qword ptr [rbp + 0x13e8]
0x1404de83e call 0x140c170f0
0x1404de843 mov rax, qword ptr [rbp + 0x1498]
0x1404de84a inc qword ptr [rax]
0x1404de84d mov rax, qword ptr [rbp + 0x13a8]
0x1404de854 cmp byte ptr [rax], 0
0x1404de857 jne 0x1404de8c5
0x1404de859 mov rcx, qword ptr [rbp + 0x1420]
0x1404de860 call 0x140c0ef70
0x1404de865 cmp qword ptr [rbp + 0x1380], 2
0x1404de86d setae cl
0x1404de870 or al, cl
0x1404de872 cmp al, 1
0x1404de874 jne 0x1404de8c5
0x1404de876 mov rax, qword ptr [rbp + 0x1478]
0x1404de87d mov rcx, qword ptr [rax + 0x10e8]
0x1404de884 mov rax, qword ptr [rcx + 0x10]
0x1404de888 mov rdx, qword ptr [rcx + 0x18]
0x1404de88c lock inc qword ptr [rax]
0x1404de890 jle 0x1404df65a
0x1404de896 add rcx, 0x20
0x1404de89a mov qword ptr [rbp + 0xc38], rdx
0x1404de8a1 mov qword ptr [rbp + 0xc30], rax
0x1404de8a8 mov byte ptr [rbp + 0xc40], 1
0x1404de8af lea rdx, [rbp + 0xc30]
0x1404de8b6 call 0x14053dfb0
0x1404de8bb mov rax, qword ptr [rbp + 0x13a8]
0x1404de8c2 mov byte ptr [rax], 1
0x1404de8c5 xor eax, eax
0x1404de8c7 mov cl, 1
0x1404de8c9 lock cmpxchg byte ptr [rip + 0x31562c0], cl
0x1404de8d1 jne 0x1404ded49
0x1404de8d7 mov rax, qword ptr [rip + 0x314db42]
0x1404de8de mov rax, qword ptr [rax]
0x1404de8e1 shl rax, 1
0x1404de8e4 test rax, rax
0x1404de8e7 jne 0x1404ded5a
0x1404de8ed xor eax, eax
0x1404de8ef movzx ecx, byte ptr [rip + 0x315629c]
0x1404de8f6 test cl, cl
0x1404de8f8 jne 0x1404df510
0x1404de8fe movzx ebx, byte ptr [rip + 0x315628e]
0x1404de905 mov byte ptr [rip + 0x3156287], 0
0x1404de90c test al, al
0x1404de90e jne 0x1404de926
0x1404de910 mov rax, qword ptr [rip + 0x314db09]
0x1404de917 mov rax, qword ptr [rax]
0x1404de91a shl rax, 1
0x1404de91d test rax, rax
0x1404de920 jne 0x1404ded95
0x1404de926 xor eax, eax
0x1404de928 xchg byte ptr [rip + 0x3156263], al
0x1404de92e cmp al, 2
0x1404de930 je 0x1404ded75
0x1404de936 test bl, bl
0x1404de938 je 0x1404d97c5
0x1404de93e mov rcx, qword ptr [rbp + 0x1488]
0x1404de945 movabs rdx, 0x3d7b56e787d47c40
0x1404de94f movabs r8, 0x67b0b10a8d43bfc7
0x1404de959 call 0x1400dd6f0
0x1404de95e test rax, rax
0x1404de961 mov r8, qword ptr [rbp + 0x1478]
0x1404de968 je 0x1404df1a3
0x1404de96e cmp qword ptr [r8 + 0x248], rdx
0x1404de975 jbe 0x1404df1a3
0x1404de97b mov rax, qword ptr [r8 + 0x240]
0x1404de982 mov rcx, qword ptr [rax + rdx*8]
0x1404de986 test rcx, rcx
0x1404de989 je 0x1404df1a3
0x1404de98f mov rax, qword ptr [r8 + 0x210]
0x1404de996 not rcx
0x1404de999 lea rcx, [rcx + rcx*2]
0x1404de99d shl rcx, 5
0x1404de9a1 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de9a7 je 0x1404df1a3
0x1404de9ad add rax, rcx
0x1404de9b0 mov rsi, qword ptr [rax + 0x10]
0x1404de9b4 mov eax, dword ptr [r8 + 0x5e0]
0x1404de9bb mov rcx, qword ptr [rbp + 0x1488]
0x1404de9c2 movabs rdx, 0x2ed189581b92decf
0x1404de9cc movabs r8, 0x15d3d99396065f38
0x1404de9d6 call 0x1400dd6f0
0x1404de9db test rax, rax
0x1404de9de mov r8, qword ptr [rbp + 0x1478]
0x1404de9e5 je 0x1404df1b4
0x1404de9eb cmp qword ptr [r8 + 0x248], rdx
0x1404de9f2 jbe 0x1404df1b4
0x1404de9f8 mov rax, qword ptr [r8 + 0x240]
0x1404de9ff mov rcx, qword ptr [rax + rdx*8]
0x1404dea03 test rcx, rcx
0x1404dea06 je 0x1404df1b4
0x1404dea0c mov rax, qword ptr [r8 + 0x210]
0x1404dea13 not rcx
0x1404dea16 lea rcx, [rcx + rcx*2]
0x1404dea1a shl rcx, 5
0x1404dea1e cmp qword ptr [rax + rcx + 0x20], 0
0x1404dea24 je 0x1404df1b4
0x1404dea2a test bl, 2
0x1404dea2d je 0x1404dea52
0x1404dea2f add rax, rcx
0x1404dea32 mov rcx, qword ptr [rax + 0x10]
0x1404dea36 call 0x140987ca0
0x1404dea3b movzx r8d, byte ptr [rsi + 0xa8]
0x1404dea43 mov rcx, qword ptr [rbp + 0x1420]
0x1404dea4a mov rdx, rax
0x1404dea4d call 0x140c0e8a0
0x1404dea52 test bl, 5
0x1404dea55 je 0x1404debc0
0x1404dea5b xor edi, edi
0x1404dea5d cmp dword ptr [rsi + 0x68], 1
0x1404dea61 jne 0x1404dea75
0x1404dea63 movupd xmm0, xmmword ptr [rsi + 0x6c]
0x1404dea68 movapd xmmword ptr [rbp + 0x13e0], xmm0
0x1404dea70 mov edi, 1
0x1404dea75 movzx ebx, byte ptr [rsi + 0x7c]
0x1404dea79 movzx r14d, byte ptr [rsi + 0x7d]
0x1404dea7e mov r15, qword ptr [rsi + 0x60]
0x1404dea82 mov rax, qword ptr [rsi + 0x38]
0x1404dea86 mov rcx, rax
0x1404dea89 neg rcx
0x1404dea8c lea r8, [rbp + 0xc30]
0x1404dea93 jo 0x1404deac2
0x1404dea95 movabs rcx, 0x8000000000000001
0x1404dea9f cmp rax, rcx
0x1404deaa2 je 0x1404dead6
0x1404deaa4 lea rdx, [rsi + 0x38]
0x1404deaa8 lea rcx, [rbp + 0x600]
0x1404deaaf call 0x140d7bd90
0x1404deab4 mov rax, qword ptr [rbp + 0x600]
0x1404deabb lea r8, [rbp + 0xc30]
0x1404deac2 movups xmm0, xmmword ptr [rsi + 0x50]
0x1404deac6 lea rcx, [rbp + 0x608]
0x1404deacd movdqu xmm1, xmmword ptr [rcx]
0x1404dead1 movdqa xmmword ptr [rbp - 0x40], xmm1
0x1404dead6 mov qword ptr [rbp + 0xc38], rax
0x1404deadd movdqa xmm1, xmmword ptr [rbp - 0x40]
0x1404deae2 lea rax, [rbp + 0xc38]
0x1404deae9 movdqu xmmword ptr [rax + 8], xmm1
0x1404deaee movups xmmword ptr [rbp + 0xc50], xmm0
0x1404deaf5 mov qword ptr [rbp + 0xc60], r15
0x1404deafc mov dword ptr [rbp + 0xc68], edi
0x1404deb02 movapd xmm0, xmmword ptr [rbp + 0x13e0]
0x1404deb0a movupd xmmword ptr [rax + 0x34], xmm0
0x1404deb0f mov byte ptr [rbp + 0xc7c], bl
0x1404deb15 mov byte ptr [rbp + 0xc7d], r14b
0x1404deb1c movabs rax, 0x8000000000000004
0x1404deb26 mov qword ptr [rbp + 0xc30], rax
0x1404deb2d lea rcx, [rbp + 0x600]
0x1404deb34 mov rdx, qword ptr [rbp + 0x1418]
0x1404deb3b call 0x1403c5a10
0x1404deb40 mov rax, qword ptr [rbp + 0x600]
0x1404deb47 movabs rcx, 0x8000000000000007
0x1404deb51 cmp rax, rcx
0x1404deb54 je 0x1404debc0
0x1404deb56 movabs rcx, 0x8000000000000000
0x1404deb60 xor rax, rcx
0x1404deb63 cmp rax, 7
0x1404deb67 mov ecx, 3
0x1404deb6c cmovae rax, rcx
0x1404deb70 cmp rax, 4
0x1404deb74 je 0x1404deb8a
0x1404deb76 cmp rax, 3
0x1404deb7a jne 0x1404debc0
0x1404deb7c lea rcx, [rbp + 0x600]
0x1404deb83 call 0x1405f0d10
0x1404deb88 jmp 0x1404debc0
0x1404deb8a mov rdx, qword ptr [rbp + 0x608]
0x1404deb91 movabs rax, 0x8000000000000002
0x1404deb9b cmp rdx, rax
0x1404deb9e jl 0x1404debc0
0x1404deba0 test rdx, rdx
0x1404deba3 je 0x1404debc0
0x1404deba5 mov rcx, qword ptr [rbp + 0x610]
0x1404debac mov r8d, 1
0x1404debb2 call 0x140613c20
0x1404debb7 nop word ptr [rax + rax]
0x1404debc0 mov rcx, qword ptr [rbp + 0x1488]
0x1404debc7 movabs rdx, 0xa554fbb3bbf1ca0
0x1404debd1 movabs r8, 0x29b993a986bb8e9d
0x1404debdb call 0x1400dd6f0
0x1404debe0 test rax, rax
0x1404debe3 mov r8, qword ptr [rbp + 0x1478]
0x1404debea je 0x1404df1dd
0x1404debf0 cmp qword ptr [r8 + 0x248], rdx
0x1404debf7 jbe 0x1404df1dd
0x1404debfd mov rax, qword ptr [r8 + 0x240]
0x1404dec04 mov rcx, qword ptr [rax + rdx*8]
0x1404dec08 test rcx, rcx
0x1404dec0b je 0x1404df1dd
0x1404dec11 mov rax, qword ptr [r8 + 0x210]
0x1404dec18 not rcx
0x1404dec1b lea rcx, [rcx + rcx*2]
0x1404dec1f shl rcx, 5
0x1404dec23 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dec29 je 0x1404df1dd
0x1404dec2f add rax, rcx
0x1404dec32 mov rdi, qword ptr [rax + 0x10]
0x1404dec36 mov ecx, dword ptr [r8 + 0x5e0]
0x1404dec3d mov dword ptr [rax + 0x5c], ecx
0x1404dec40 mov rbx, qword ptr [rdi + 0x30]
0x1404dec44 mov r14, qword ptr [rdi + 0x40]
0x1404dec48 cmp rbx, qword ptr [rdi + 0x20]
0x1404dec4c jne 0x1404dec5e
0x1404dec4e lea rcx, [rdi + 0x20]
0x1404dec52 lea rdx, [rip + 0x25144cf]
0x1404dec59 call 0x140562870
0x1404dec5e mov rax, qword ptr [rdi + 0x28]
0x1404dec62 mov qword ptr [rax + rbx*8], r14
0x1404dec66 inc rbx
0x1404dec69 mov qword ptr [rdi + 0x30], rbx
0x1404dec6d inc qword ptr [rdi + 0x40]
0x1404dec71 lea rcx, [rbp + 0xc30]
0x1404dec78 mov rdx, rsi
0x1404dec7b call 0x1404d7d50
0x1404dec80 mov byte ptr [rbp + 0x1474], 1
0x1404dec87 mov rcx, qword ptr [rbp + 0x1488]
0x1404dec8e movabs rdx, 0xfe830c34a6496103
0x1404dec98 movabs r8, 0xd66ebacc5f29088d
0x1404deca2 call 0x1400dd6f0
0x1404deca7 test rax, rax
0x1404decaa mov r8, qword ptr [rbp + 0x1478]
0x1404decb1 lea rsi, [rbp + 0xc30]
0x1404decb8 je 0x1404df1c5
0x1404decbe cmp qword ptr [r8 + 0x248], rdx
0x1404decc5 jbe 0x1404df1c5
0x1404deccb mov rax, qword ptr [r8 + 0x240]
0x1404decd2 mov rcx, qword ptr [rax + rdx*8]
0x1404decd6 test rcx, rcx
0x1404decd9 je 0x1404df1c5
0x1404decdf mov rax, qword ptr [r8 + 0x210]
0x1404dece6 not rcx
0x1404dece9 lea rcx, [rcx + rcx*2]
0x1404deced shl rcx, 5
0x1404decf1 cmp qword ptr [rax + rcx + 0x20], 0
0x1404decf7 je 0x1404df1c5
0x1404decfd add rax, rcx
0x1404ded00 mov rdx, qword ptr [rax + 0x10]
0x1404ded04 mov ecx, dword ptr [r8 + 0x5e0]
0x1404ded0b mov dword ptr [rax + 0x5c], ecx
0x1404ded0e xor eax, eax
0x1404ded10 mov qword ptr [rbp + 0x1498], rdx
0x1404ded17 cmp rax, qword ptr [rdx]
0x1404ded1a jo 0x1404d97b0
0x1404ded20 mov rcx, qword ptr [rbp + 0x1498]
0x1404ded27 call 0x1405fdb80
0x1404ded2c jmp 0x1404d97b0
0x1404ded31 lea rcx, [rbp + 0x600]
0x1404ded38 mov rdx, qword ptr [rbp + 0x1368]
0x1404ded3f call 0x14026dfd0
0x1404ded44 jmp 0x1404ddf4e
0x1404ded49 lea rcx, [rip + 0x3155e41]
0x1404ded50 call 0x1428d8750
0x1404ded55 jmp 0x1404de8d7
0x1404ded5a call 0x1428d86b0
0x1404ded5f xor al, 1
0x1404ded61 movzx ecx, byte ptr [rip + 0x3155e2a]
0x1404ded68 test cl, cl
0x1404ded6a je 0x1404de8fe
0x1404ded70 jmp 0x1404df510
0x1404ded75 lea rcx, [rip + 0x3155e15]
0x1404ded7c call 0x1428d8810
0x1404ded81 jmp 0x1404de936
0x1404ded86 mov rcx, rdi
0x1404ded89 xor edx, edx
0x1404ded8b call 0x1428cdb90
0x1404ded90 jmp 0x1404dde57
0x1404ded95 call 0x1428d86b0
0x1404ded9a test al, al
0x1404ded9c jne 0x1404de926
0x1404deda2 mov byte ptr [rip + 0x3155de9], 1
0x1404deda9 jmp 0x1404de926
0x1404dedae mov esi, 1
0x1404dedb3 lea rcx, [rbp + 0x13b0]
0x1404dedba call 0x1406035f0
0x1404dedbf mov rax, rsi
0x1404dedc2 movaps xmm6, xmmword ptr [rbp + 0x14b0]
0x1404dedc9 movaps xmm7, xmmword ptr [rbp + 0x14c0]
0x1404dedd0 movaps xmm8, xmmword ptr [rbp + 0x14d0]
0x1404dedd8 movaps xmm9, xmmword ptr [rbp + 0x14e0]
0x1404dede0 movaps xmm10, xmmword ptr [rbp + 0x14f0]
0x1404dede8 movaps xmm11, xmmword ptr [rbp + 0x1500]
0x1404dedf0 movaps xmm12, xmmword ptr [rbp + 0x1510]
0x1404dedf8 movaps xmm13, xmmword ptr [rbp + 0x1520]
0x1404dee00 movaps xmm14, xmmword ptr [rbp + 0x1530]
0x1404dee08 movaps xmm15, xmmword ptr [rbp + 0x1540]
0x1404dee10 add rsp, 0x15d8
0x1404dee17 pop rbx
0x1404dee18 pop rdi
0x1404dee19 pop rsi
0x1404dee1a pop r12
0x1404dee1c pop r13
0x1404dee1e pop r14
0x1404dee20 pop r15
0x1404dee22 pop rbp
0x1404dee23 ret 
0x1404dee24 movabs rax, 0x8000000000000005
0x1404dee2e mov qword ptr [rbp + 0xc30], rax
0x1404dee35 lea rcx, [rbp + 0x600]
0x1404dee3c lea r8, [rbp + 0xc30]
0x1404dee43 mov rdx, qword ptr [rbp + 0x1418]
0x1404dee4a call 0x1403c5a10
0x1404dee4f mov rax, qword ptr [rbp + 0x600]
0x1404dee56 movabs rcx, 0x8000000000000007
0x1404dee60 cmp rax, rcx
0x1404dee63 je 0x1404deec6
0x1404dee65 movabs rcx, 0x8000000000000000
0x1404dee6f xor rax, rcx
0x1404dee72 cmp rax, 7
0x1404dee76 mov ecx, 3
0x1404dee7b cmovb rcx, rax
0x1404dee7f cmp rcx, 4
0x1404dee83 je 0x1404dee99
0x1404dee85 cmp rcx, 3
0x1404dee89 jne 0x1404deec6
0x1404dee8b lea rcx, [rbp + 0x600]
0x1404dee92 call 0x1405f0d10
0x1404dee97 jmp 0x1404deec6
0x1404dee99 mov rdx, qword ptr [rbp + 0x608]
0x1404deea0 movabs rax, 0x8000000000000002
0x1404deeaa cmp rdx, rax
0x1404deead jl 0x1404deec6
0x1404deeaf test rdx, rdx
0x1404deeb2 je 0x1404deec6
0x1404deeb4 mov rcx, qword ptr [rbp + 0x610]
0x1404deebb mov r8d, 1
0x1404deec1 call 0x140613c20
0x1404deec6 xor esi, esi
0x1404deec8 jmp 0x1404dedb3
0x1404deecd lea rcx, [rip + 0x2453be4]
0x1404deed4 call 0x1428d9250
0x1404deed9 jmp 0x1404df65a
0x1404deede lea rcx, [rip + 0x2453bd3]
0x1404deee5 call 0x1428d9250
0x1404deeea jmp 0x1404df65a
0x1404deeef lea rcx, [rip + 0x2453bc2]
0x1404deef6 call 0x1428d9250
0x1404deefb jmp 0x1404df65a
0x1404def00 lea rcx, [rip + 0x2453bb1]
0x1404def07 call 0x1428d9250
0x1404def0c jmp 0x1404df65a
0x1404def11 lea rcx, [rip + 0x250aaf0]
0x1404def18 lea r8, [rip + 0x250ab19]
0x1404def1f mov edx, 0x2c
0x1404def24 call 0x1428d9330
0x1404def29 jmp 0x1404df65a
0x1404def2e mov byte ptr [rbp + 0x14a3], 1
0x1404def35 lea rcx, [rip + 0x2453b7c]
0x1404def3c call 0x1428d9250
0x1404def41 jmp 0x1404df65a
0x1404def46 mov byte ptr [rbp + 0x14a3], 0
0x1404def4d lea rcx, [rip + 0x2453b64]
0x1404def54 call 0x1428d9250
0x1404def59 jmp 0x1404df65a
0x1404def5e mov qword ptr [rbp + 0x1478], rax
0x1404def65 lea rcx, [rip + 0x24f04cc]
0x1404def6c lea r8, [rip + 0x24f053d]
0x1404def73 mov edx, 0x28
0x1404def78 call 0x1428d9430
0x1404def7d jmp 0x1404df65a
0x1404def82 mov byte ptr [rbp + 0x14a7], sil
0x1404def89 mov byte ptr [rbp + 0x14a6], 1
0x1404def90 mov byte ptr [rbp + 0x14a5], 1
0x1404def97 lea rcx, [rip + 0x250b09a]
0x1404def9e call 0x1428d9310
0x1404defa3 jmp 0x1404df65a
0x1404defa8 lea rcx, [rip + 0x250aa59]
0x1404defaf lea r8, [rip + 0x250aa82]
0x1404defb6 mov edx, 0x2c
0x1404defbb call 0x1428d9330
0x1404defc0 jmp 0x1404df65a
0x1404defc5 mov byte ptr [rbp + 0x14a7], 1
0x1404defcc mov byte ptr [rbp + 0x14a6], 1
0x1404defd3 mov byte ptr [rbp + 0x14a5], 1
0x1404defda lea rcx, [rip + 0x250aff7]
0x1404defe1 call 0x1428d9310
0x1404defe6 jmp 0x1404df65a
0x1404defeb mov byte ptr [rbp + 0x14a7], 1
0x1404deff2 mov byte ptr [rbp + 0x14a6], 1
0x1404deff9 mov byte ptr [rbp + 0x14a5], 1
0x1404df000 lea rcx, [rip + 0x250aef9]
0x1404df007 call 0x1428d9310
0x1404df00c jmp 0x1404df65a
0x1404df011 mov byte ptr [rbp + 0x14a7], 1
0x1404df018 mov byte ptr [rbp + 0x14a6], 1
0x1404df01f mov byte ptr [rbp + 0x14a5], 1
0x1404df026 lea rcx, [rip + 0x250aebb]
0x1404df02d call 0x1428d9310
0x1404df032 jmp 0x1404df65a
0x1404df037 mov byte ptr [rbp + 0x1475], 1
0x1404df03e lea rcx, [rip + 0x250afdb]
0x1404df045 call 0x1428d9310
0x1404df04a jmp 0x1404df65a
0x1404df04f mov byte ptr [rbp + 0x14a7], 1
0x1404df056 mov byte ptr [rbp + 0x14a6], 1
0x1404df05d mov byte ptr [rbp + 0x14a5], 1
0x1404df064 lea rcx, [rip + 0x250aead]
0x1404df06b call 0x1428d9310
0x1404df070 jmp 0x1404df65a
0x1404df075 mov byte ptr [rbp + 0x14a7], 1
0x1404df07c mov byte ptr [rbp + 0x14a6], 1
0x1404df083 mov byte ptr [rbp + 0x14a5], 1
0x1404df08a lea rcx, [rip + 0x250af77]
0x1404df091 call 0x1428d9310
0x1404df096 jmp 0x1404df65a
0x1404df09b mov byte ptr [rbp + 0x14a7], 1
0x1404df0a2 mov byte ptr [rbp + 0x14a6], 1
0x1404df0a9 mov byte ptr [rbp + 0x14a5], 1
0x1404df0b0 lea rcx, [rip + 0x250af39]
0x1404df0b7 call 0x1428d9310
0x1404df0bc jmp 0x1404df65a
0x1404df0c1 mov byte ptr [rbp + 0x1487], 1
0x1404df0c8 lea rcx, [rip + 0x250ae91]
0x1404df0cf call 0x1428d9310
0x1404df0d4 jmp 0x1404df65a
0x1404df0d9 mov byte ptr [rbp + 0x14a7], 1
0x1404df0e0 mov byte ptr [rbp + 0x14a6], 1
0x1404df0e7 mov byte ptr [rbp + 0x14a5], 1
0x1404df0ee lea rcx, [rip + 0x250ae3b]
0x1404df0f5 call 0x1428d9310
0x1404df0fa jmp 0x1404df65a
0x1404df0ff mov byte ptr [rbp + 0x1476], 1
0x1404df106 lea rcx, [rip + 0x250ae6b]
0x1404df10d call 0x1428d9310
0x1404df112 jmp 0x1404df65a
0x1404df117 mov byte ptr [rbp + 0x14a7], 1
0x1404df11e mov byte ptr [rbp + 0x14a6], 1
0x1404df125 mov byte ptr [rbp + 0x14a5], 1
0x1404df12c lea rcx, [rip + 0x250ae75]
0x1404df133 call 0x1428d9310
0x1404df138 jmp 0x1404df65a
0x1404df13d mov byte ptr [rbp + 0x14a7], 1
0x1404df144 mov byte ptr [rbp + 0x14a6], 1
0x1404df14b mov byte ptr [rbp + 0x14a5], 1
0x1404df152 lea rcx, [rip + 0x250ae37]
0x1404df159 call 0x1428d9310
0x1404df15e jmp 0x1404df65a
0x1404df163 lea rcx, [rip + 0x250aefe]
0x1404df16a call 0x1428d9310
0x1404df16f jmp 0x1404df65a
0x1404df174 lea rcx, [rip + 0x250aed5]
0x1404df17b call 0x1428d9310
0x1404df180 jmp 0x1404df65a
0x1404df185 mov byte ptr [rbp + 0x1496], 1
0x1404df18c mov byte ptr [rbp + 0x1495], bl
0x1404df192 lea rcx, [rip + 0x250ae27]
0x1404df199 call 0x1428d9310
0x1404df19e jmp 0x1404df65a
0x1404df1a3 lea rcx, [rip + 0x250b11e]
0x1404df1aa call 0x1428d9310
0x1404df1af jmp 0x1404df65a
0x1404df1b4 lea rcx, [rip + 0x250b125]
0x1404df1bb call 0x1428d9310
0x1404df1c0 jmp 0x1404df65a
0x1404df1c5 mov byte ptr [rbp + 0x1474], 1
0x1404df1cc lea rcx, [rip + 0x250b13d]
0x1404df1d3 call 0x1428d9310
0x1404df1d8 jmp 0x1404df65a
0x1404df1dd lea rcx, [rip + 0x250b114]
0x1404df1e4 call 0x1428d9310
0x1404df1e9 jmp 0x1404df65a
0x1404df1ee mov byte ptr [rbp + 0x14a4], 1
0x1404df1f5 lea rcx, [rip + 0x250abe4]
0x1404df1fc call 0x1428d9310
0x1404df201 jmp 0x1404df65a
0x1404df206 mov byte ptr [rbp + 0x14a4], 1
0x1404df20d lea rcx, [rip + 0x250abb4]
0x1404df214 call 0x1428d9310
0x1404df219 jmp 0x1404df65a
0x1404df21e mov byte ptr [rbp + 0x14a4], 1
0x1404df225 lea rcx, [rip + 0x250abe4]
0x1404df22c call 0x1428d9310
0x1404df231 jmp 0x1404df65a
0x1404df236 mov byte ptr [rbp + 0x14a4], 1
0x1404df23d lea rcx, [rip + 0x250abe4]
0x1404df244 call 0x1428d9310
0x1404df249 jmp 0x1404df65a
0x1404df24e mov byte ptr [rbp + 0x14a4], 1
0x1404df255 lea rcx, [rip + 0x250abe4]
0x1404df25c call 0x1428d9310
0x1404df261 jmp 0x1404df65a
0x1404df266 mov byte ptr [rbp + 0x14a4], 1
0x1404df26d lea rcx, [rip + 0x250abfc]
0x1404df274 call 0x1428d9310
0x1404df279 jmp 0x1404df65a
0x1404df27e mov byte ptr [rbp + 0x14a7], 1
0x1404df285 mov byte ptr [rbp + 0x14a6], 1
0x1404df28c mov byte ptr [rbp + 0x14a5], 1
0x1404df293 lea rcx, [rip + 0x250ac06]
0x1404df29a call 0x1428d9310
0x1404df29f jmp 0x1404df65a
0x1404df2a4 mov byte ptr [rbp + 0x14a7], 1
0x1404df2ab mov byte ptr [rbp + 0x14a6], 1
0x1404df2b2 mov byte ptr [rbp + 0x14a5], 1
0x1404df2b9 lea rcx, [rip + 0x250abf8]
0x1404df2c0 call 0x1428d9310
0x1404df2c5 jmp 0x1404df65a
0x1404df2ca mov byte ptr [rbp + 0x14a7], 1
0x1404df2d1 mov byte ptr [rbp + 0x14a6], 1
0x1404df2d8 mov byte ptr [rbp + 0x14a5], 1
0x1404df2df lea rcx, [rip + 0x250b072]
0x1404df2e6 call 0x1428d9310
0x1404df2eb jmp 0x1404df65a
0x1404df2f0 mov byte ptr [rbp + 0x14a7], 1
0x1404df2f7 mov byte ptr [rbp + 0x14a6], 1
0x1404df2fe mov byte ptr [rbp + 0x14a5], 1
0x1404df305 lea rcx, [rip + 0x250abc4]
0x1404df30c call 0x1428d9310
0x1404df311 jmp 0x1404df65a
0x1404df316 lea rcx, [rip + 0x250ad63]
0x1404df31d call 0x1428d9310
0x1404df322 jmp 0x1404df65a
0x1404df327 lea rcx, [rip + 0x250ad6a]
0x1404df32e call 0x1428d9310
0x1404df333 jmp 0x1404df65a
0x1404df338 lea rcx, [rip + 0x250ae29]
0x1404df33f call 0x1428d9310
0x1404df344 jmp 0x1404df65a
0x1404df349 lea rcx, [rip + 0x250ae30]
0x1404df350 call 0x1428d9310
0x1404df355 jmp 0x1404df65a
0x1404df35a lea rcx, [rip + 0x250ae37]
0x1404df361 call 0x1428d9310
0x1404df366 jmp 0x1404df65a
0x1404df36b lea rcx, [rip + 0x250af26]
0x1404df372 call 0x1428d9310
0x1404df377 jmp 0x1404df65a
0x1404df37c lea rcx, [rip + 0x250ad45]
0x1404df383 call 0x1428d9310
0x1404df388 jmp 0x1404df65a
0x1404df38d lea rcx, [rip + 0x250ad1c]
0x1404df394 call 0x1428d9310
0x1404df399 jmp 0x1404df65a
0x1404df39e lea rcx, [rip + 0x250ad93]
0x1404df3a5 call 0x1428d9310
0x1404df3aa jmp 0x1404df65a
0x1404df3af lea rcx, [rip + 0x250ad9a]
0x1404df3b6 call 0x1428d9310
0x1404df3bb jmp 0x1404df65a
0x1404df3c0 lea rax, [rip + 0x250a701]
0x1404df3c7 mov qword ptr [rbp + 0x13e0], rax
0x1404df3ce mov qword ptr [rbp + 0x13e8], 0x36
0x1404df3d9 lea rax, [rbp + 0x13e0]
0x1404df3e0 mov qword ptr [rbp + 0x1430], rax
0x1404df3e7 lea rax, [rip - 0x3d555e]
0x1404df3ee mov qword ptr [rbp + 0x1438], rax
0x1404df3f5 lea rax, [rip + 0x250a80c]
0x1404df3fc mov qword ptr [rbp + 0xc30], rax
0x1404df403 mov qword ptr [rbp + 0xc38], 2
0x1404df40e mov qword ptr [rbp + 0xc50], 0
0x1404df419 lea rax, [rbp + 0x1430]
0x1404df420 mov qword ptr [rbp + 0xc40], rax
0x1404df427 mov qword ptr [rbp + 0xc48], 1
0x1404df432 mov byte ptr [rbp + 0x1497], 0
0x1404df439 lea rdx, [rip + 0x250a918]
0x1404df440 lea rcx, [rbp + 0xc30]
0x1404df447 call 0x1428d9390
0x1404df44c jmp 0x1404df65a
0x1404df451 mov byte ptr [rbp + 0x14a4], 1
0x1404df458 lea rcx, [rip + 0x250a999]
0x1404df45f call 0x1428d9310
0x1404df464 jmp 0x1404df65a
0x1404df469 mov byte ptr [rbp + 0x14a4], 1
0x1404df470 lea rcx, [rip + 0x250a9e1]
0x1404df477 call 0x1428d9310
0x1404df47c jmp 0x1404df65a
0x1404df481 mov byte ptr [rbp + 0x14a4], 1
0x1404df488 lea rcx, [rip + 0x250a9f9]
0x1404df48f call 0x1428d9310
0x1404df494 jmp 0x1404df65a
0x1404df499 lea rcx, [rip + 0x250ac40]
0x1404df4a0 call 0x1428d9310
0x1404df4a5 jmp 0x1404df65a
0x1404df4aa lea rcx, [rip + 0x250ac47]
0x1404df4b1 call 0x1428d9310
0x1404df4b6 jmp 0x1404df65a
0x1404df4bb lea rcx, [rip + 0x31556cf]
0x1404df4c2 mov qword ptr [rbp + 0xc30], rcx
0x1404df4c9 mov byte ptr [rbp + 0xc38], al
0x1404df4cf lea rax, [rip + 0x250aa72]
0x1404df4d6 mov qword ptr [rsp + 0x20], rax
0x1404df4db lea rcx, [rip + 0x250a29e]
0x1404df4e2 lea r9, [rip + 0x250a2c7]
0x1404df4e9 lea r8, [rbp + 0xc30]
0x1404df4f0 mov edx, 0x2b
0x1404df4f5 call 0x1428d9760
0x1404df4fa jmp 0x1404df65a
0x1404df4ff lea rcx, [rip + 0x250ae52]
0x1404df506 call 0x1428d9310
0x1404df50b jmp 0x1404df65a
0x1404df510 lea rcx, [rip + 0x315567a]
0x1404df517 mov qword ptr [rbp + 0xc30], rcx
0x1404df51e mov byte ptr [rbp + 0xc38], al
0x1404df524 lea rax, [rip + 0x250ad85]
0x1404df52b mov qword ptr [rsp + 0x20], rax
0x1404df530 lea rcx, [rip + 0x250a249]
0x1404df537 lea r9, [rip + 0x250a272]
0x1404df53e lea r8, [rbp + 0xc30]
0x1404df545 mov edx, 0x2b
0x1404df54a call 0x1428d9760
0x1404df54f jmp 0x1404df65a
0x1404df554 mov qword ptr [rbp + 0x1478], r15
0x1404df55b mov ecx, 4
0x1404df560 mov edx, 0xdc
0x1404df565 call 0x1428d9000
0x1404df56a jmp 0x1404df65a
0x1404df56f lea rcx, [rip + 0x245349a]
0x1404df576 call 0x1428d80d0
0x1404df57b jmp 0x1404df65a
0x1404df580 lea rcx, [rip + 0x2453489]
0x1404df587 call 0x1428d80d0
0x1404df58c jmp 0x1404df65a
0x1404df591 lea rcx, [rip + 0x2453478]
0x1404df598 call 0x1428d80d0
0x1404df59d jmp 0x1404df65a
0x1404df5a2 lea rcx, [rip + 0x2453467]
0x1404df5a9 call 0x1428d80d0
0x1404df5ae jmp 0x1404df65a
0x1404df5b3 lea rax, [rip + 0x250a436]
0x1404df5ba mov qword ptr [rsp + 0x20], rax
0x1404df5bf lea rcx, [rip + 0x250a6fa]
0x1404df5c6 mov edx, 0x22
0x1404df5cb mov r9d, 0x22
0x1404df5d1 mov r8, rsi
0x1404df5d4 call 0x1428d9940
0x1404df5d9 jmp 0x1404df65a
0x1404df5db lea rax, [rip + 0x250a40e]
0x1404df5e2 mov qword ptr [rsp + 0x20], rax
0x1404df5e7 lea rcx, [rip + 0x250a6d2]
0x1404df5ee mov edx, 0x22
0x1404df5f3 mov r9d, 0x22
0x1404df5f9 mov r8, rdi
0x1404df5fc call 0x1428d9940
0x1404df601 jmp 0x1404df65a
0x1404df603 mov byte ptr [rbp + 0x14a3], 1
0x1404df60a lea rcx, [rip + 0x24533ff]
0x1404df611 call 0x1428d80d0
0x1404df616 jmp 0x1404df65a
0x1404df618 mov byte ptr [rbp + 0x14a3], 0
0x1404df61f lea rcx, [rip + 0x24533ea]
0x1404df626 call 0x1428d80d0
0x1404df62b jmp 0x1404df65a
0x1404df62d mov byte ptr [rbp + 0x14a3], 1
0x1404df634 lea rax, [rip + 0x250a3b5]
0x1404df63b mov qword ptr [rsp + 0x20], rax
0x1404df640 lea rcx, [rip + 0x250a679]
0x1404df647 mov edx, 0x22
0x1404df64c mov r9d, 0x22
0x1404df652 mov r8, r13
0x1404df655 call 0x1428d9940
0x1404df65a ud2 
0x1404df65c nop dword ptr [rax]
0x1404df660 mov qword ptr [rsp + 0x10], rdx
0x1404df665 push rbp
0x1404df666 push r15
0x1404df668 push r14
0x1404df66a push r13
0x1404df66c push r12
0x1404df66e push rsi
0x1404df66f push rdi
0x1404df670 push rbx
0x1404df671 sub rsp, 0xe8
0x1404df678 lea rbp, [rdx + 0x80]
0x1404df67f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df686 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df68c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df693 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df699 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df6a2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df6ac movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df6b6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df6bf movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df6c8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404df6d1 cmp byte ptr [rbp + 0x1473], 0
0x1404df6d8 je 0x1404df76e
0x1404df6de cmp qword ptr [rbp + 0x1468], 0
0x1404df6e6 je 0x1404df701
0x1404df6e8 mov r8d, 1
0x1404df6ee mov rcx, qword ptr [rbp + 0x1460]
0x1404df6f5 mov rdx, qword ptr [rbp + 0x1468]
0x1404df6fc call 0x140613c20
0x1404df701 mov byte ptr [rbp + 0x14a2], 1
0x1404df708 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404df711 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df71a movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df723 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df72d movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df737 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df740 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df746 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df74d movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df753 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df75a add rsp, 0xe8
0x1404df761 pop rbx
0x1404df762 pop rdi
0x1404df763 pop rsi
0x1404df764 pop r12
0x1404df766 pop r13
0x1404df768 pop r14
0x1404df76a pop r15
0x1404df76c pop rbp
0x1404df76d ret 
0x1404df76e mov byte ptr [rbp + 0x14a2], 1
0x1404df775 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404df77e movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df787 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df790 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df79a movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df7a4 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df7ad movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df7b3 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df7ba movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df7c0 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df7c7 add rsp, 0xe8
0x1404df7ce pop rbx
0x1404df7cf pop rdi
0x1404df7d0 pop rsi
0x1404df7d1 pop r12
0x1404df7d3 pop r13
0x1404df7d5 pop r14
0x1404df7d7 pop r15
0x1404df7d9 pop rbp
0x1404df7da ret 
0x1404df7db nop dword ptr [rax + rax]
0x1404df7e0 mov qword ptr [rsp + 0x10], rdx
0x1404df7e5 push rbp
0x1404df7e6 push r15
0x1404df7e8 push r14
0x1404df7ea push r13
0x1404df7ec push r12
0x1404df7ee push rsi
0x1404df7ef push rdi
0x1404df7f0 push rbx
0x1404df7f1 sub rsp, 0xe8
0x1404df7f8 lea rbp, [rdx + 0x80]
0x1404df7ff movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df806 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df80c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df813 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df819 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df822 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df82c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df836 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df83f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df848 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404df851 mov rax, qword ptr [rbp + 0x1498]
0x1404df858 inc qword ptr [rax]
0x1404df85b mov byte ptr [rbp + 0x14a3], 0
0x1404df862 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404df86b movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df874 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df87d movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df887 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df891 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df89a movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df8a0 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df8a7 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df8ad movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df8b4 add rsp, 0xe8
0x1404df8bb pop rbx
0x1404df8bc pop rdi
0x1404df8bd pop rsi
0x1404df8be pop r12
0x1404df8c0 pop r13
0x1404df8c2 pop r14
0x1404df8c4 pop r15
0x1404df8c6 pop rbp
0x1404df8c7 ret 
0x1404df8c8 nop dword ptr [rax + rax]
0x1404df8d0 mov qword ptr [rsp + 0x10], rdx
0x1404df8d5 push rbp
0x1404df8d6 push r15
0x1404df8d8 push r14
0x1404df8da push r13
0x1404df8dc push r12
0x1404df8de push rsi
0x1404df8df push rdi
0x1404df8e0 push rbx
0x1404df8e1 sub rsp, 0xe8
0x1404df8e8 lea rbp, [rdx + 0x80]
0x1404df8ef movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df8f6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df8fc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df903 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df909 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df912 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df91c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df926 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df92f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df938 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404df941 mov rax, qword ptr [rbp + 0x1448]
0x1404df948 inc qword ptr [rax]
0x1404df94b mov byte ptr [rbp + 0x14a3], 1
0x1404df952 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404df95b movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df964 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df96d movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df977 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df981 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df98a movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df990 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df997 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df99d movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df9a4 add rsp, 0xe8
0x1404df9ab pop rbx
0x1404df9ac pop rdi
0x1404df9ad pop rsi
0x1404df9ae pop r12
0x1404df9b0 pop r13
0x1404df9b2 pop r14
0x1404df9b4 pop r15
0x1404df9b6 pop rbp
0x1404df9b7 ret 
0x1404df9b8 nop dword ptr [rax + rax]
0x1404df9c0 mov qword ptr [rsp + 0x10], rdx
0x1404df9c5 push rbp
0x1404df9c6 push r15
0x1404df9c8 push r14
0x1404df9ca push r13
0x1404df9cc push r12
0x1404df9ce push rsi
0x1404df9cf push rdi
0x1404df9d0 push rbx
0x1404df9d1 sub rsp, 0xe8
0x1404df9d8 lea rbp, [rdx + 0x80]
0x1404df9df movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df9e6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df9ec movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df9f3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df9f9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfa02 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfa0c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfa16 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfa1f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfa28 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfa31 movzx ebx, byte ptr [rbp + 0x1472]
0x1404dfa38 cmp byte ptr [rbp + 0x1498], 0
0x1404dfa3f je 0x1404dfa6d
0x1404dfa41 lea rax, [rbp - 0x38]
0x1404dfa45 mov qword ptr [rbp + 0x1458], rax
0x1404dfa4c lea rax, [rbp + 0x1458]
0x1404dfa53 mov qword ptr [rbp + 0x1430], rax
0x1404dfa5a lea rcx, [rip + 0x2509f2f]
0x1404dfa61 lea rdx, [rbp + 0x1430]
0x1404dfa68 call 0x1400a2580
0x1404dfa6d and bl, 1
0x1404dfa70 mov byte ptr [rbp + 0x14a3], bl
0x1404dfa76 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfa7f movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfa88 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfa91 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfa9b movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfaa5 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfaae movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfab4 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfabb movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfac1 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfac8 add rsp, 0xe8
0x1404dfacf pop rbx
0x1404dfad0 pop rdi
0x1404dfad1 pop rsi
0x1404dfad2 pop r12
0x1404dfad4 pop r13
0x1404dfad6 pop r14
0x1404dfad8 pop r15
0x1404dfada pop rbp
0x1404dfadb ret 
0x1404dfadc nop dword ptr [rax]
0x1404dfae0 mov qword ptr [rsp + 0x10], rdx
0x1404dfae5 push rbp
0x1404dfae6 push r15
0x1404dfae8 push r14
0x1404dfaea push r13
0x1404dfaec push r12
0x1404dfaee push rsi
0x1404dfaef push rdi
0x1404dfaf0 push rbx
0x1404dfaf1 sub rsp, 0xe8
0x1404dfaf8 lea rbp, [rdx + 0x80]
0x1404dfaff movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfb06 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfb0c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfb13 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfb19 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfb22 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfb2c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfb36 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfb3f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfb48 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404dfb51 cmp byte ptr [rbp + 0x1498], 0
0x1404dfb58 je 0x1404dfbec
0x1404dfb5e movzx eax, byte ptr [rbp + 0xc90]
0x1404dfb65 cmp al, 0x1b
0x1404dfb67 je 0x1404dfb86
0x1404dfb69 cmp al, 0x18
0x1404dfb6b jne 0x1404dfb86
0x1404dfb6d mov rax, qword ptr [rbp + 0xc98]
0x1404dfb74 lock dec qword ptr [rax]
0x1404dfb78 jne 0x1404dfb86
0x1404dfb7a lea rcx, [rbp + 0xc98]
0x1404dfb81 call 0x14064eb30
0x1404dfb86 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfb8f movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfb98 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfba1 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfbab movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfbb5 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfbbe movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfbc4 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfbcb movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfbd1 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfbd8 add rsp, 0xe8
0x1404dfbdf pop rbx
0x1404dfbe0 pop rdi
0x1404dfbe1 pop rsi
0x1404dfbe2 pop r12
0x1404dfbe4 pop r13
0x1404dfbe6 pop r14
0x1404dfbe8 pop r15
0x1404dfbea pop rbp
0x1404dfbeb ret 
0x1404dfbec movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfbf5 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfbfe movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfc07 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfc11 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfc1b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfc24 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfc2a movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfc31 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfc37 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfc3e add rsp, 0xe8
0x1404dfc45 pop rbx
0x1404dfc46 pop rdi
0x1404dfc47 pop rsi
0x1404dfc48 pop r12
0x1404dfc4a pop r13
0x1404dfc4c pop r14
0x1404dfc4e pop r15
0x1404dfc50 pop rbp
0x1404dfc51 ret 
0x1404dfc52 nop word ptr cs:[rax + rax]
0x1404dfc60 mov qword ptr [rsp + 0x10], rdx
0x1404dfc65 push rbp
0x1404dfc66 push r15
0x1404dfc68 push r14
0x1404dfc6a push r13
0x1404dfc6c push r12
0x1404dfc6e push rsi
0x1404dfc6f push rdi
0x1404dfc70 push rbx
0x1404dfc71 sub rsp, 0xe8
0x1404dfc78 lea rbp, [rdx + 0x80]
0x1404dfc7f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfc86 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfc8c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfc93 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfc99 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfca2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfcac movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfcb6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfcbf movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfcc8 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfcd1 lea rcx, [rbp + 0x600]
0x1404dfcd8 mov r8d, 0x68
0x1404dfcde mov r9d, 0x10
0x1404dfce4 lea rdx, [rbp + 0x620]
0x1404dfceb call 0x14004ceb0
0x1404dfcf0 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfcf8 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfd00 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfd09 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfd12 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dfd1b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfd24 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfd2a movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dfd30 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfd36 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dfd3c add rsp, 0xe8
0x1404dfd43 pop rbx
0x1404dfd44 pop rdi
0x1404dfd45 pop rsi
0x1404dfd46 pop r12
0x1404dfd48 pop r13
0x1404dfd4a pop r14
0x1404dfd4c pop r15
0x1404dfd4e pop rbp
0x1404dfd4f ret 
0x1404dfd50 mov qword ptr [rsp + 0x10], rdx
0x1404dfd55 push rbp
0x1404dfd56 push r15
0x1404dfd58 push r14
0x1404dfd5a push r13
0x1404dfd5c push r12
0x1404dfd5e push rsi
0x1404dfd5f push rdi
0x1404dfd60 push rbx
0x1404dfd61 sub rsp, 0xe8
0x1404dfd68 lea rbp, [rdx + 0x80]
0x1404dfd6f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfd76 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfd7c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfd83 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfd89 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfd92 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfd9c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfda6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfdaf movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfdb8 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfdc1 lea rcx, [rbp + 0x600]
0x1404dfdc8 mov r8d, 0x68
0x1404dfdce mov r9d, 0x10
0x1404dfdd4 lea rdx, [rbp + 0x620]
0x1404dfddb call 0x14004ceb0
0x1404dfde0 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfde8 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfdf0 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfdf9 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfe02 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dfe0b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfe14 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfe1a movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dfe20 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfe26 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dfe2c add rsp, 0xe8
0x1404dfe33 pop rbx
0x1404dfe34 pop rdi
0x1404dfe35 pop rsi
0x1404dfe36 pop r12
0x1404dfe38 pop r13
0x1404dfe3a pop r14
0x1404dfe3c pop r15
0x1404dfe3e pop rbp
0x1404dfe3f ret 
0x1404dfe40 mov qword ptr [rsp + 0x10], rdx
0x1404dfe45 push rbp
0x1404dfe46 push r15
0x1404dfe48 push r14
0x1404dfe4a push r13
0x1404dfe4c push r12
0x1404dfe4e push rsi
0x1404dfe4f push rdi
0x1404dfe50 push rbx
0x1404dfe51 sub rsp, 0xe8
0x1404dfe58 lea rbp, [rdx + 0x80]
0x1404dfe5f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfe66 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfe6c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfe73 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfe79 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfe82 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfe8c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfe96 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfe9f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfea8 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfeb1 mov r8d, 0x10
0x1404dfeb7 mov r9d, 0x10
0x1404dfebd mov rcx, qword ptr [rbp + 0x13a0]
0x1404dfec4 mov rdx, qword ptr [rbp + 0x1378]
0x1404dfecb call 0x14004c400
0x1404dfed0 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404dfed7 mov rax, qword ptr [rbp + 0x13d0]
0x1404dfede movups xmmword ptr [rax + 0x40], xmm0
0x1404dfee2 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dfee9 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dfef0 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dfef7 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404dfefe movups xmmword ptr [rax + 0x30], xmm3
0x1404dff02 movups xmmword ptr [rax + 0x20], xmm2
0x1404dff06 movups xmmword ptr [rax + 0x10], xmm1
0x1404dff0a movups xmmword ptr [rax], xmm0
0x1404dff0d mov rax, qword ptr [rbp + 0x1478]
0x1404dff14 mov rcx, qword ptr [rbp + 0x1468]
0x1404dff1b mov qword ptr [rax + 0x9d0], rcx
0x1404dff22 mov rcx, qword ptr [rbp + 0x1498]
0x1404dff29 mov qword ptr [rax + 0x9d8], rcx
0x1404dff30 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dff38 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dff40 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dff49 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dff52 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dff5b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dff64 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dff6a movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dff70 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dff76 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dff7c add rsp, 0xe8
0x1404dff83 pop rbx
0x1404dff84 pop rdi
0x1404dff85 pop rsi
0x1404dff86 pop r12
0x1404dff88 pop r13
0x1404dff8a pop r14
0x1404dff8c pop r15
0x1404dff8e pop rbp
0x1404dff8f ret 
0x1404dff90 mov qword ptr [rsp + 0x10], rdx
0x1404dff95 push rbp
0x1404dff96 push r15
0x1404dff98 push r14
0x1404dff9a push r13
0x1404dff9c push r12
0x1404dff9e push rsi
0x1404dff9f push rdi
0x1404dffa0 push rbx
0x1404dffa1 sub rsp, 0xe8
0x1404dffa8 lea rbp, [rdx + 0x80]
0x1404dffaf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dffb6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dffbc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dffc3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dffc9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dffd2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dffdc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dffe6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dffef movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfff8 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0001 mov r8d, 0x10
0x1404e0007 mov r9d, 0x10
0x1404e000d mov rcx, qword ptr [rbp + 0x1398]
0x1404e0014 mov rdx, qword ptr [rbp + 0x1370]
0x1404e001b call 0x14004c400
0x1404e0020 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404e0027 mov rax, qword ptr [rbp + 0x13c8]
0x1404e002e movups xmmword ptr [rax + 0x40], xmm0
0x1404e0032 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404e0039 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404e0040 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404e0047 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404e004e movups xmmword ptr [rax + 0x30], xmm3
0x1404e0052 movups xmmword ptr [rax + 0x20], xmm2
0x1404e0056 movups xmmword ptr [rax + 0x10], xmm1
0x1404e005a movups xmmword ptr [rax], xmm0
0x1404e005d mov rax, qword ptr [rbp + 0x1478]
0x1404e0064 mov rcx, qword ptr [rbp + 0x1468]
0x1404e006b mov qword ptr [rax + 0x8e0], rcx
0x1404e0072 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0079 mov qword ptr [rax + 0x8e8], rcx
0x1404e0080 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0088 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0090 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0099 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e00a2 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e00ab movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e00b4 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e00ba movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e00c0 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e00c6 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e00cc add rsp, 0xe8
0x1404e00d3 pop rbx
0x1404e00d4 pop rdi
0x1404e00d5 pop rsi
0x1404e00d6 pop r12
0x1404e00d8 pop r13
0x1404e00da pop r14
0x1404e00dc pop r15
0x1404e00de pop rbp
0x1404e00df ret 
0x1404e00e0 mov qword ptr [rsp + 0x10], rdx
0x1404e00e5 push rbp
0x1404e00e6 push r15
0x1404e00e8 push r14
0x1404e00ea push r13
0x1404e00ec push r12
0x1404e00ee push rsi
0x1404e00ef push rdi
0x1404e00f0 push rbx
0x1404e00f1 sub rsp, 0xe8
0x1404e00f8 lea rbp, [rdx + 0x80]
0x1404e00ff movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0106 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e010c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0113 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0119 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0122 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e012c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0136 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e013f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0148 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0151 lea rdx, [rbp + 0xc30]
0x1404e0158 mov r8d, 0x628
0x1404e015e mov rcx, qword ptr [rbp + 0x1410]
0x1404e0165 call 0x142923c00
0x1404e016a mov byte ptr [rbp + 0x1497], 0
0x1404e0171 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e017a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0183 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e018c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0196 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e01a0 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e01a9 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e01af movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e01b6 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e01bc movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e01c3 add rsp, 0xe8
0x1404e01ca pop rbx
0x1404e01cb pop rdi
0x1404e01cc pop rsi
0x1404e01cd pop r12
0x1404e01cf pop r13
0x1404e01d1 pop r14
0x1404e01d3 pop r15
0x1404e01d5 pop rbp
0x1404e01d6 ret 
0x1404e01d7 nop word ptr [rax + rax]
0x1404e01e0 mov qword ptr [rsp + 0x10], rdx
0x1404e01e5 push rbp
0x1404e01e6 push r15
0x1404e01e8 push r14
0x1404e01ea push r13
0x1404e01ec push r12
0x1404e01ee push rsi
0x1404e01ef push rdi
0x1404e01f0 push rbx
0x1404e01f1 sub rsp, 0xe8
0x1404e01f8 lea rbp, [rdx + 0x80]
0x1404e01ff movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0206 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e020c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0213 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0219 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0222 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e022c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0236 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e023f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0248 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0251 mov rax, qword ptr [rbp + 0x1498]
0x1404e0258 inc qword ptr [rax]
0x1404e025b movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0264 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e026d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0276 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0280 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e028a movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0293 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0299 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e02a0 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e02a6 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e02ad add rsp, 0xe8
0x1404e02b4 pop rbx
0x1404e02b5 pop rdi
0x1404e02b6 pop rsi
0x1404e02b7 pop r12
0x1404e02b9 pop r13
0x1404e02bb pop r14
0x1404e02bd pop r15
0x1404e02bf pop rbp
0x1404e02c0 ret 
0x1404e02c1 nop word ptr cs:[rax + rax]
0x1404e02d0 mov qword ptr [rsp + 0x10], rdx
0x1404e02d5 push rbp
0x1404e02d6 push r15
0x1404e02d8 push r14
0x1404e02da push r13
0x1404e02dc push r12
0x1404e02de push rsi
0x1404e02df push rdi
0x1404e02e0 push rbx
0x1404e02e1 sub rsp, 0xe8
0x1404e02e8 lea rbp, [rdx + 0x80]
0x1404e02ef movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e02f6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e02fc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0303 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0309 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0312 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e031c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0326 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e032f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0338 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0341 mov rax, qword ptr [rbp + 0x1448]
0x1404e0348 inc qword ptr [rax]
0x1404e034b movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0354 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e035d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0366 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0370 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e037a movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0383 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0389 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0390 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0396 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e039d add rsp, 0xe8
0x1404e03a4 pop rbx
0x1404e03a5 pop rdi
0x1404e03a6 pop rsi
0x1404e03a7 pop r12
0x1404e03a9 pop r13
0x1404e03ab pop r14
0x1404e03ad pop r15
0x1404e03af pop rbp
0x1404e03b0 ret 
0x1404e03b1 nop word ptr cs:[rax + rax]
0x1404e03c0 mov qword ptr [rsp + 0x10], rdx
0x1404e03c5 push rbp
0x1404e03c6 push r15
0x1404e03c8 push r14
0x1404e03ca push r13
0x1404e03cc push r12
0x1404e03ce push rsi
0x1404e03cf push rdi
0x1404e03d0 push rbx
0x1404e03d1 sub rsp, 0xe8
0x1404e03d8 lea rbp, [rdx + 0x80]
0x1404e03df movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e03e6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e03ec movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e03f3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e03f9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0402 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e040c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0416 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e041f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0428 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0431 mov rax, qword ptr [rbp + 0x1498]
0x1404e0438 inc qword ptr [rax]
0x1404e043b movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0443 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e044b movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0454 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e045d movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0466 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e046f movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0475 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e047b movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0481 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0487 add rsp, 0xe8
0x1404e048e pop rbx
0x1404e048f pop rdi
0x1404e0490 pop rsi
0x1404e0491 pop r12
0x1404e0493 pop r13
0x1404e0495 pop r14
0x1404e0497 pop r15
0x1404e0499 pop rbp
0x1404e049a ret 
0x1404e049b nop dword ptr [rax + rax]
0x1404e04a0 mov qword ptr [rsp + 0x10], rdx
0x1404e04a5 push rbp
0x1404e04a6 push r15
0x1404e04a8 push r14
0x1404e04aa push r13
0x1404e04ac push r12
0x1404e04ae push rsi
0x1404e04af push rdi
0x1404e04b0 push rbx
0x1404e04b1 sub rsp, 0xe8
0x1404e04b8 lea rbp, [rdx + 0x80]
0x1404e04bf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e04c6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e04cc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e04d3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e04d9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e04e2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e04ec movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e04f6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e04ff movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0508 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0511 lea rcx, [rbp + 0xc50]
0x1404e0518 call 0x1405fa110
0x1404e051d mov byte ptr [rbp + 0x14a7], 1
0x1404e0524 mov byte ptr [rbp + 0x14a6], 0
0x1404e052b mov byte ptr [rbp + 0x14a5], 1
0x1404e0532 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e053b movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0544 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e054d movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0557 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0561 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e056a movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0570 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0577 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e057d movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0584 add rsp, 0xe8
0x1404e058b pop rbx
0x1404e058c pop rdi
0x1404e058d pop rsi
0x1404e058e pop r12
0x1404e0590 pop r13
0x1404e0592 pop r14
0x1404e0594 pop r15
0x1404e0596 pop rbp
0x1404e0597 ret 
0x1404e0598 nop dword ptr [rax + rax]
0x1404e05a0 mov qword ptr [rsp + 0x10], rdx
0x1404e05a5 push rbp
0x1404e05a6 push r15
0x1404e05a8 push r14
0x1404e05aa push r13
0x1404e05ac push r12
0x1404e05ae push rsi
0x1404e05af push rdi
0x1404e05b0 push rbx
0x1404e05b1 sub rsp, 0xe8
0x1404e05b8 lea rbp, [rdx + 0x80]
0x1404e05bf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e05c6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e05cc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e05d3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e05d9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e05e2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e05ec movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e05f6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e05ff movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0608 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e0610 mov rax, qword ptr [rbp + 0x1498]
0x1404e0617 inc qword ptr [rax]
0x1404e061a movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0622 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e062a movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0633 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e063c movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0645 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e064e movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0654 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e065a movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0660 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0666 add rsp, 0xe8
0x1404e066d pop rbx
0x1404e066e pop rdi
0x1404e066f pop rsi
0x1404e0670 pop r12
0x1404e0672 pop r13
0x1404e0674 pop r14
0x1404e0676 pop r15
0x1404e0678 pop rbp
0x1404e0679 ret 
0x1404e067a nop word ptr [rax + rax]
0x1404e0680 mov qword ptr [rsp + 0x10], rdx
0x1404e0685 push rbp
0x1404e0686 push r15
0x1404e0688 push r14
0x1404e068a push r13
0x1404e068c push r12
0x1404e068e push rsi
0x1404e068f push rdi
0x1404e0690 push rbx
0x1404e0691 sub rsp, 0xe8
0x1404e0698 lea rbp, [rdx + 0x80]
0x1404e069f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e06a6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e06ac movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e06b3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e06b9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e06c2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e06cc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e06d6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e06df movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e06e8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e06f1 lea rdx, [rbp + 0xc30]
0x1404e06f8 mov r8d, 0xc8
0x1404e06fe mov rcx, qword ptr [rbp + 0x1498]
0x1404e0705 call 0x142923c00
0x1404e070a mov byte ptr [rbp + 0x1474], 0
0x1404e0711 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e071a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0723 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e072c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0736 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0740 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0749 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e074f movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0756 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e075c movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0763 add rsp, 0xe8
0x1404e076a pop rbx
0x1404e076b pop rdi
0x1404e076c pop rsi
0x1404e076d pop r12
0x1404e076f pop r13
0x1404e0771 pop r14
0x1404e0773 pop r15
0x1404e0775 pop rbp
0x1404e0776 ret 
0x1404e0777 nop word ptr [rax + rax]
0x1404e0780 mov qword ptr [rsp + 0x10], rdx
0x1404e0785 push rbp
0x1404e0786 push r15
0x1404e0788 push r14
0x1404e078a push r13
0x1404e078c push r12
0x1404e078e push rsi
0x1404e078f push rdi
0x1404e0790 push rbx
0x1404e0791 sub rsp, 0xe8
0x1404e0798 lea rbp, [rdx + 0x80]
0x1404e079f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e07a6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e07ac movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e07b3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e07b9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e07c2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e07cc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e07d6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e07df movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e07e8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e07f1 movzx edx, byte ptr [rbp + 0x1498]
0x1404e07f8 and dl, 1
0x1404e07fb mov byte ptr [rbp + 0x14a7], 1
0x1404e0802 mov byte ptr [rbp + 0x14a6], 1
0x1404e0809 mov byte ptr [rbp + 0x14a5], 1
0x1404e0810 mov rcx, qword ptr [rbp + 0x1478]
0x1404e0817 call 0x140043000
0x1404e081c mov byte ptr [rbp + 0x14a7], 1
0x1404e0823 mov byte ptr [rbp + 0x14a6], 1
0x1404e082a mov byte ptr [rbp + 0x14a5], 1
0x1404e0831 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e083a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0843 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e084c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0856 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0860 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0869 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e086f movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0876 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e087c movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0883 add rsp, 0xe8
0x1404e088a pop rbx
0x1404e088b pop rdi
0x1404e088c pop rsi
0x1404e088d pop r12
0x1404e088f pop r13
0x1404e0891 pop r14
0x1404e0893 pop r15
0x1404e0895 pop rbp
0x1404e0896 ret 
0x1404e0897 nop word ptr [rax + rax]
0x1404e08a0 mov qword ptr [rsp + 0x10], rdx
0x1404e08a5 push rbp
0x1404e08a6 push r15
0x1404e08a8 push r14
0x1404e08aa push r13
0x1404e08ac push r12
0x1404e08ae push rsi
0x1404e08af push rdi
0x1404e08b0 push rbx
0x1404e08b1 sub rsp, 0xe8
0x1404e08b8 lea rbp, [rdx + 0x80]
0x1404e08bf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e08c6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e08cc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e08d3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e08d9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e08e2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e08ec movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e08f6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e08ff movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0908 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0911 mov byte ptr [rbp + 0x1496], 0
0x1404e0918 mov esi, dword ptr [rbp + 0x1498]
0x1404e091e mov byte ptr [rbp + 0x1495], sil
0x1404e0925 lea rcx, [rbp + 0x608]
0x1404e092c call 0x1405f1730
0x1404e0931 mov byte ptr [rbp + 0x1496], 0
0x1404e0938 mov byte ptr [rbp + 0x1495], sil
0x1404e093f movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0948 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0951 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e095a movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0964 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e096e movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0977 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e097d movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0984 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e098a movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0991 add rsp, 0xe8
0x1404e0998 pop rbx
0x1404e0999 pop rdi
0x1404e099a pop rsi
0x1404e099b pop r12
0x1404e099d pop r13
0x1404e099f pop r14
0x1404e09a1 pop r15
0x1404e09a3 pop rbp
0x1404e09a4 ret 
0x1404e09a5 nop word ptr cs:[rax + rax]
0x1404e09b0 mov qword ptr [rsp + 0x10], rdx
0x1404e09b5 push rbp
0x1404e09b6 push r15
0x1404e09b8 push r14
0x1404e09ba push r13
0x1404e09bc push r12
0x1404e09be push rsi
0x1404e09bf push rdi
0x1404e09c0 push rbx
0x1404e09c1 sub rsp, 0xe8
0x1404e09c8 lea rbp, [rdx + 0x80]
0x1404e09cf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e09d6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e09dc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e09e3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e09e9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e09f2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e09fc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0a06 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0a0f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0a18 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0a21 lea rdx, [rbp + 0xc30]
0x1404e0a28 mov r8d, 0x628
0x1404e0a2e mov rcx, qword ptr [rbp + 0x1450]
0x1404e0a35 call 0x142923c00
0x1404e0a3a mov byte ptr [rbp + 0x14a4], 0
0x1404e0a41 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0a4a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0a53 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0a5c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0a66 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0a70 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0a79 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0a7f movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0a86 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0a8c movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0a93 add rsp, 0xe8
0x1404e0a9a pop rbx
0x1404e0a9b pop rdi
0x1404e0a9c pop rsi
0x1404e0a9d pop r12
0x1404e0a9f pop r13
0x1404e0aa1 pop r14
0x1404e0aa3 pop r15
0x1404e0aa5 pop rbp
0x1404e0aa6 ret 
0x1404e0aa7 nop word ptr [rax + rax]
0x1404e0ab0 mov qword ptr [rsp + 0x10], rdx
0x1404e0ab5 push rbp
0x1404e0ab6 push r15
0x1404e0ab8 push r14
0x1404e0aba push r13
0x1404e0abc push r12
0x1404e0abe push rsi
0x1404e0abf push rdi
0x1404e0ac0 push rbx
0x1404e0ac1 sub rsp, 0xe8
0x1404e0ac8 lea rbp, [rdx + 0x80]
0x1404e0acf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0ad6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0adc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ae3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ae9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0af2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0afc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0b06 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0b0f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0b18 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0b21 cmp byte ptr [rbp + 0x1498], 0
0x1404e0b28 je 0x1404e0b53
0x1404e0b2a lea rax, [rbp + 0xc38]
0x1404e0b31 mov qword ptr [rbp - 0x40], rax
0x1404e0b35 lea rax, [rbp - 0x40]
0x1404e0b39 mov qword ptr [rbp + 0x600], rax
0x1404e0b40 lea rcx, [rip + 0x2508e49]
0x1404e0b47 lea rdx, [rbp + 0x600]
0x1404e0b4e call 0x1400a2580
0x1404e0b53 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0b5c movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0b65 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0b6e movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0b78 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0b82 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0b8b movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0b91 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0b98 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0b9e movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0ba5 add rsp, 0xe8
0x1404e0bac pop rbx
0x1404e0bad pop rdi
0x1404e0bae pop rsi
0x1404e0baf pop r12
0x1404e0bb1 pop r13
0x1404e0bb3 pop r14
0x1404e0bb5 pop r15
0x1404e0bb7 pop rbp
0x1404e0bb8 ret 
0x1404e0bb9 nop dword ptr [rax]
0x1404e0bc0 mov qword ptr [rsp + 0x10], rdx
0x1404e0bc5 push rbp
0x1404e0bc6 push r15
0x1404e0bc8 push r14
0x1404e0bca push r13
0x1404e0bcc push r12
0x1404e0bce push rsi
0x1404e0bcf push rdi
0x1404e0bd0 push rbx
0x1404e0bd1 sub rsp, 0xe8
0x1404e0bd8 lea rbp, [rdx + 0x80]
0x1404e0bdf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0be6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0bec movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0bf3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0bf9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0c02 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0c0c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0c16 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0c1f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0c28 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0c31 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0c38 call 0x140161c30
0x1404e0c3d movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0c46 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0c4f movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0c58 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0c62 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0c6c movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0c75 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0c7b movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0c82 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0c88 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0c8f add rsp, 0xe8
0x1404e0c96 pop rbx
0x1404e0c97 pop rdi
0x1404e0c98 pop rsi
0x1404e0c99 pop r12
0x1404e0c9b pop r13
0x1404e0c9d pop r14
0x1404e0c9f pop r15
0x1404e0ca1 pop rbp
0x1404e0ca2 ret 
0x1404e0ca3 nop word ptr cs:[rax + rax]
0x1404e0cb0 mov qword ptr [rsp + 0x10], rdx
0x1404e0cb5 push rbp
0x1404e0cb6 push r15
0x1404e0cb8 push r14
0x1404e0cba push r13
0x1404e0cbc push r12
0x1404e0cbe push rsi
0x1404e0cbf push rdi
0x1404e0cc0 push rbx
0x1404e0cc1 sub rsp, 0xe8
0x1404e0cc8 lea rbp, [rdx + 0x80]
0x1404e0ccf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0cd6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0cdc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ce3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ce9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0cf2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0cfc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0d06 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0d0f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0d18 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0d21 lea rdx, [rbp + 0x600]
0x1404e0d28 mov r8d, 0xc8
0x1404e0d2e mov rcx, qword ptr [rbp + 0x1498]
0x1404e0d35 call 0x142923c00
0x1404e0d3a mov byte ptr [rbp + 0x1476], 0
0x1404e0d41 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0d4a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0d53 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0d5c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0d66 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0d70 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0d79 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0d7f movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0d86 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0d8c movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0d93 add rsp, 0xe8
0x1404e0d9a pop rbx
0x1404e0d9b pop rdi
0x1404e0d9c pop rsi
0x1404e0d9d pop r12
0x1404e0d9f pop r13
0x1404e0da1 pop r14
0x1404e0da3 pop r15
0x1404e0da5 pop rbp
0x1404e0da6 ret 
0x1404e0da7 nop word ptr [rax + rax]
0x1404e0db0 mov qword ptr [rsp + 0x10], rdx
0x1404e0db5 push rbp
0x1404e0db6 push r15
0x1404e0db8 push r14
0x1404e0dba push r13
0x1404e0dbc push r12
0x1404e0dbe push rsi
0x1404e0dbf push rdi
0x1404e0dc0 push rbx
0x1404e0dc1 sub rsp, 0xe8
0x1404e0dc8 lea rbp, [rdx + 0x80]
0x1404e0dcf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0dd6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0ddc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0de3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0de9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0df2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0dfc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0e06 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0e0f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0e18 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0e21 cmp qword ptr [rbp + 0x1498], 0
0x1404e0e29 je 0x1404e0e44
0x1404e0e2b mov r8d, 1
0x1404e0e31 mov rcx, qword ptr [rbp + 0x1468]
0x1404e0e38 mov rdx, qword ptr [rbp + 0x1498]
0x1404e0e3f call 0x140613c20
0x1404e0e44 mov byte ptr [rbp + 0x1475], 0
0x1404e0e4b movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0e54 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0e5d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0e66 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0e70 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0e7a movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0e83 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0e89 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0e90 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0e96 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0e9d add rsp, 0xe8
0x1404e0ea4 pop rbx
0x1404e0ea5 pop rdi
0x1404e0ea6 pop rsi
0x1404e0ea7 pop r12
0x1404e0ea9 pop r13
0x1404e0eab pop r14
0x1404e0ead pop r15
0x1404e0eaf pop rbp
0x1404e0eb0 ret 
0x1404e0eb1 nop word ptr cs:[rax + rax]
0x1404e0ec0 mov qword ptr [rsp + 0x10], rdx
0x1404e0ec5 push rbp
0x1404e0ec6 push r15
0x1404e0ec8 push r14
0x1404e0eca push r13
0x1404e0ecc push r12
0x1404e0ece push rsi
0x1404e0ecf push rdi
0x1404e0ed0 push rbx
0x1404e0ed1 sub rsp, 0xe8
0x1404e0ed8 lea rbp, [rdx + 0x80]
0x1404e0edf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0ee6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0eec movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ef3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ef9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0f02 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0f0c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0f16 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0f1f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0f28 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0f31 lea rcx, [rbp + 0xc30]
0x1404e0f38 call 0x1405d2870
0x1404e0f3d movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0f45 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0f4d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0f56 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0f5f movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0f68 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0f71 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0f77 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e0f7d movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0f83 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0f89 add rsp, 0xe8
0x1404e0f90 pop rbx
0x1404e0f91 pop rdi
0x1404e0f92 pop rsi
0x1404e0f93 pop r12
0x1404e0f95 pop r13
0x1404e0f97 pop r14
0x1404e0f99 pop r15
0x1404e0f9b pop rbp
0x1404e0f9c ret 
0x1404e0f9d nop dword ptr [rax]
0x1404e0fa0 mov qword ptr [rsp + 0x10], rdx
0x1404e0fa5 push rbp
0x1404e0fa6 push r15
0x1404e0fa8 push r14
0x1404e0faa push r13
0x1404e0fac push r12
0x1404e0fae push rsi
0x1404e0faf push rdi
0x1404e0fb0 push rbx
0x1404e0fb1 sub rsp, 0xe8
0x1404e0fb8 lea rbp, [rdx + 0x80]
0x1404e0fbf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0fc6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0fcc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0fd3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0fd9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0fe2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0fec movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0ff6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0fff movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1008 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1011 mov byte ptr [rbp + 0x14a7], 1
0x1404e1018 mov byte ptr [rbp + 0x14a6], 1
0x1404e101f mov byte ptr [rbp + 0x14a5], 1
0x1404e1026 lea rcx, [rbp + 0xc30]
0x1404e102d call 0x1405d2870
0x1404e1032 mov byte ptr [rbp + 0x14a7], 1
0x1404e1039 mov byte ptr [rbp + 0x14a6], 1
0x1404e1040 mov byte ptr [rbp + 0x14a5], 1
0x1404e1047 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1050 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1059 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1062 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e106c movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1076 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e107f movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1085 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e108c movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1092 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1099 add rsp, 0xe8
0x1404e10a0 pop rbx
0x1404e10a1 pop rdi
0x1404e10a2 pop rsi
0x1404e10a3 pop r12
0x1404e10a5 pop r13
0x1404e10a7 pop r14
0x1404e10a9 pop r15
0x1404e10ab pop rbp
0x1404e10ac ret 
0x1404e10ad nop dword ptr [rax]
0x1404e10b0 mov qword ptr [rsp + 0x10], rdx
0x1404e10b5 push rbp
0x1404e10b6 push r15
0x1404e10b8 push r14
0x1404e10ba push r13
0x1404e10bc push r12
0x1404e10be push rsi
0x1404e10bf push rdi
0x1404e10c0 push rbx
0x1404e10c1 sub rsp, 0xe8
0x1404e10c8 lea rbp, [rdx + 0x80]
0x1404e10cf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e10d6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e10dc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e10e3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e10e9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e10f2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e10fc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1106 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e110f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1118 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1121 cmp byte ptr [rbp + 0x1474], 0
0x1404e1128 je 0x1404e11a1
0x1404e112a xor eax, eax
0x1404e112c cmp rax, qword ptr [rbp + 0xc30]
0x1404e1133 jo 0x1404e1141
0x1404e1135 lea rcx, [rbp + 0xc30]
0x1404e113c call 0x1405fdb80
0x1404e1141 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1149 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1151 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e115a movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1163 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e116c movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1175 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e117b movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1181 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1187 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e118d add rsp, 0xe8
0x1404e1194 pop rbx
0x1404e1195 pop rdi
0x1404e1196 pop rsi
0x1404e1197 pop r12
0x1404e1199 pop r13
0x1404e119b pop r14
0x1404e119d pop r15
0x1404e119f pop rbp
0x1404e11a0 ret 
0x1404e11a1 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e11a9 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e11b1 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e11ba movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e11c3 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e11cc movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e11d5 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e11db movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e11e1 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e11e7 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e11ed add rsp, 0xe8
0x1404e11f4 pop rbx
0x1404e11f5 pop rdi
0x1404e11f6 pop rsi
0x1404e11f7 pop r12
0x1404e11f9 pop r13
0x1404e11fb pop r14
0x1404e11fd pop r15
0x1404e11ff pop rbp
0x1404e1200 ret 
0x1404e1201 nop word ptr cs:[rax + rax]
0x1404e1210 mov qword ptr [rsp + 0x10], rdx
0x1404e1215 push rbp
0x1404e1216 push r15
0x1404e1218 push r14
0x1404e121a push r13
0x1404e121c push r12
0x1404e121e push rsi
0x1404e121f push rdi
0x1404e1220 push rbx
0x1404e1221 sub rsp, 0xe8
0x1404e1228 lea rbp, [rdx + 0x80]
0x1404e122f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1236 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e123c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1243 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1249 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1252 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e125c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1266 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e126f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1278 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1281 cmp byte ptr [rbp + 0x1497], 0
0x1404e1288 je 0x1404e12f6
0x1404e128a lea rcx, [rbp + 0x600]
0x1404e1291 call 0x1405e8b60
0x1404e1296 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e129e movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e12a6 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e12af movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e12b8 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e12c1 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e12ca movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e12d0 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e12d6 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e12dc movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e12e2 add rsp, 0xe8
0x1404e12e9 pop rbx
0x1404e12ea pop rdi
0x1404e12eb pop rsi
0x1404e12ec pop r12
0x1404e12ee pop r13
0x1404e12f0 pop r14
0x1404e12f2 pop r15
0x1404e12f4 pop rbp
0x1404e12f5 ret 
0x1404e12f6 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e12fe movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1306 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e130f movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1318 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1321 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e132a movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1330 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1336 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e133c movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1342 add rsp, 0xe8
0x1404e1349 pop rbx
0x1404e134a pop rdi
0x1404e134b pop rsi
0x1404e134c pop r12
0x1404e134e pop r13
0x1404e1350 pop r14
0x1404e1352 pop r15
0x1404e1354 pop rbp
0x1404e1355 ret 
0x1404e1356 nop word ptr cs:[rax + rax]
0x1404e1360 mov qword ptr [rsp + 0x10], rdx
0x1404e1365 push rbp
0x1404e1366 push r15
0x1404e1368 push r14
0x1404e136a push r13
0x1404e136c push r12
0x1404e136e push rsi
0x1404e136f push rdi
0x1404e1370 push rbx
0x1404e1371 sub rsp, 0xe8
0x1404e1378 lea rbp, [rdx + 0x80]
0x1404e137f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1386 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e138c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1393 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1399 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e13a2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e13ac movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e13b6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e13bf movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e13c8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e13d1 movzx ebx, byte ptr [rbp + 0x1495]
0x1404e13d8 cmp byte ptr [rbp + 0x1496], 0
0x1404e13df je 0x1404e1456
0x1404e13e1 lea rcx, [rbp - 0x40]
0x1404e13e5 call 0x1405f1730
0x1404e13ea mov byte ptr [rbp + 0x14a2], bl
0x1404e13f0 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e13f9 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1402 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e140b movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1415 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e141f movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1428 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e142e movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1435 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e143b movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1442 add rsp, 0xe8
0x1404e1449 pop rbx
0x1404e144a pop rdi
0x1404e144b pop rsi
0x1404e144c pop r12
0x1404e144e pop r13
0x1404e1450 pop r14
0x1404e1452 pop r15
0x1404e1454 pop rbp
0x1404e1455 ret 
0x1404e1456 mov byte ptr [rbp + 0x14a2], bl
0x1404e145c movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1465 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e146e movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1477 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1481 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e148b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1494 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e149a movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e14a1 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e14a7 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e14ae add rsp, 0xe8
0x1404e14b5 pop rbx
0x1404e14b6 pop rdi
0x1404e14b7 pop rsi
0x1404e14b8 pop r12
0x1404e14ba pop r13
0x1404e14bc pop r14
0x1404e14be pop r15
0x1404e14c0 pop rbp
0x1404e14c1 ret 
0x1404e14c2 nop word ptr cs:[rax + rax]
0x1404e14d0 mov qword ptr [rsp + 0x10], rdx
0x1404e14d5 push rbp
0x1404e14d6 push r15
0x1404e14d8 push r14
0x1404e14da push r13
0x1404e14dc push r12
0x1404e14de push rsi
0x1404e14df push rdi
0x1404e14e0 push rbx
0x1404e14e1 sub rsp, 0xe8
0x1404e14e8 lea rbp, [rdx + 0x80]
0x1404e14ef movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e14f6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e14fc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1503 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1509 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1512 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e151c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1526 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e152f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1538 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1541 cmp byte ptr [rbp + 0x1476], 0
0x1404e1548 je 0x1404e15d2
0x1404e154e xor eax, eax
0x1404e1550 cmp rax, qword ptr [rbp + 0x600]
0x1404e1557 jo 0x1404e1565
0x1404e1559 lea rcx, [rbp + 0x600]
0x1404e1560 call 0x1405fdb80
0x1404e1565 mov byte ptr [rbp + 0x1487], 0
0x1404e156c movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1575 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e157e movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1587 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1591 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e159b movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e15a4 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e15aa movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e15b1 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e15b7 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e15be add rsp, 0xe8
0x1404e15c5 pop rbx
0x1404e15c6 pop rdi
0x1404e15c7 pop rsi
0x1404e15c8 pop r12
0x1404e15ca pop r13
0x1404e15cc pop r14
0x1404e15ce pop r15
0x1404e15d0 pop rbp
0x1404e15d1 ret 
0x1404e15d2 mov byte ptr [rbp + 0x1487], 0
0x1404e15d9 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e15e2 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e15eb movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e15f4 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e15fe movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1608 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1611 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1617 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e161e movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1624 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e162b add rsp, 0xe8
0x1404e1632 pop rbx
0x1404e1633 pop rdi
0x1404e1634 pop rsi
0x1404e1635 pop r12
0x1404e1637 pop r13
0x1404e1639 pop r14
0x1404e163b pop r15
0x1404e163d pop rbp
0x1404e163e ret 
0x1404e163f nop 
0x1404e1640 mov qword ptr [rsp + 0x10], rdx
0x1404e1645 push rbp
0x1404e1646 push r15
0x1404e1648 push r14
0x1404e164a push r13
0x1404e164c push r12
0x1404e164e push rsi
0x1404e164f push rdi
0x1404e1650 push rbx
0x1404e1651 sub rsp, 0xe8
0x1404e1658 lea rbp, [rdx + 0x80]
0x1404e165f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1666 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e166c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1673 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1679 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1682 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e168c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1696 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e169f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e16a8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e16b1 cmp byte ptr [rbp + 0x1475], 0
0x1404e16b8 je 0x1404e175c
0x1404e16be cmp qword ptr [rbp + 0x1498], 0
0x1404e16c6 je 0x1404e16e1
0x1404e16c8 mov r8d, 1
0x1404e16ce mov rcx, qword ptr [rbp + 0x1468]
0x1404e16d5 mov rdx, qword ptr [rbp + 0x1498]
0x1404e16dc call 0x140613c20
0x1404e16e1 mov byte ptr [rbp + 0x14a7], 1
0x1404e16e8 mov byte ptr [rbp + 0x14a6], 1
0x1404e16ef mov byte ptr [rbp + 0x14a5], 0
0x1404e16f6 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e16ff movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1708 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1711 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e171b movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1725 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e172e movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1734 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e173b movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1741 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1748 add rsp, 0xe8
0x1404e174f pop rbx
0x1404e1750 pop rdi
0x1404e1751 pop rsi
0x1404e1752 pop r12
0x1404e1754 pop r13
0x1404e1756 pop r14
0x1404e1758 pop r15
0x1404e175a pop rbp
0x1404e175b ret 
0x1404e175c mov byte ptr [rbp + 0x14a7], 1
0x1404e1763 mov byte ptr [rbp + 0x14a6], 1
0x1404e176a mov byte ptr [rbp + 0x14a5], 0
0x1404e1771 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e177a movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1783 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e178c movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1796 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e17a0 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e17a9 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e17af movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e17b6 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e17bc movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e17c3 add rsp, 0xe8
0x1404e17ca pop rbx
0x1404e17cb pop rdi
0x1404e17cc pop rsi
0x1404e17cd pop r12
0x1404e17cf pop r13
0x1404e17d1 pop r14
0x1404e17d3 pop r15
0x1404e17d5 pop rbp
0x1404e17d6 ret 
0x1404e17d7 nop word ptr [rax + rax]
0x1404e17e0 mov qword ptr [rsp + 0x10], rdx
0x1404e17e5 push rbp
0x1404e17e6 push r15
0x1404e17e8 push r14
0x1404e17ea push r13
0x1404e17ec push r12
0x1404e17ee push rsi
0x1404e17ef push rdi
0x1404e17f0 push rbx
0x1404e17f1 sub rsp, 0xe8
0x1404e17f8 lea rbp, [rdx + 0x80]
0x1404e17ff movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1806 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e180c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1813 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1819 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1822 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e182c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1836 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e183f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1848 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1851 movzx ebx, byte ptr [rbp + 0x14a2]
0x1404e1858 lea rcx, [rbp + 0xc30]
0x1404e185f call 0x1404d7c30
0x1404e1864 movzx eax, byte ptr [rbp + 0xc90]
0x1404e186b cmp al, 0x1b
0x1404e186d setne cl
0x1404e1870 test bl, cl
0x1404e1872 je 0x1404e1891
0x1404e1874 cmp al, 0x18
0x1404e1876 jne 0x1404e1891
0x1404e1878 mov rax, qword ptr [rbp + 0xc98]
0x1404e187f lock dec qword ptr [rax]
0x1404e1883 jne 0x1404e1891
0x1404e1885 lea rcx, [rbp + 0xc98]
0x1404e188c call 0x14064eb30
0x1404e1891 lea rcx, [rbp + 0xc50]
0x1404e1898 call 0x1405fa110
0x1404e189d mov byte ptr [rbp + 0x14a7], 1
0x1404e18a4 mov byte ptr [rbp + 0x14a6], 0
0x1404e18ab mov byte ptr [rbp + 0x14a5], 1
0x1404e18b2 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e18bb movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e18c4 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e18cd movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e18d7 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e18e1 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e18ea movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e18f0 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e18f7 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e18fd movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1904 add rsp, 0xe8
0x1404e190b pop rbx
0x1404e190c pop rdi
0x1404e190d pop rsi
0x1404e190e pop r12
0x1404e1910 pop r13
0x1404e1912 pop r14
0x1404e1914 pop r15
0x1404e1916 pop rbp
0x1404e1917 ret 
0x1404e1918 nop dword ptr [rax + rax]
0x1404e1920 mov qword ptr [rsp + 0x10], rdx
0x1404e1925 push rbp
0x1404e1926 push r15
0x1404e1928 push r14
0x1404e192a push r13
0x1404e192c push r12
0x1404e192e push rsi
0x1404e192f push rdi
0x1404e1930 push rbx
0x1404e1931 sub rsp, 0xe8
0x1404e1938 lea rbp, [rdx + 0x80]
0x1404e193f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1946 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e194c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1953 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1959 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1962 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e196c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1976 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e197f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1988 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1991 cmp qword ptr [rbp + 0x1478], 0
0x1404e1999 jne 0x1404e19a7
0x1404e199b lea rcx, [rbp + 0xc30]
0x1404e19a2 call 0x1403c6100
0x1404e19a7 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e19af movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e19b7 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e19c0 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e19c9 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e19d2 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e19db movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e19e1 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e19e7 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e19ed movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e19f3 add rsp, 0xe8
0x1404e19fa pop rbx
0x1404e19fb pop rdi
0x1404e19fc pop rsi
0x1404e19fd pop r12
0x1404e19ff pop r13
0x1404e1a01 pop r14
0x1404e1a03 pop r15
0x1404e1a05 pop rbp
0x1404e1a06 ret 
0x1404e1a07 nop word ptr [rax + rax]
0x1404e1a10 mov qword ptr [rsp + 0x10], rdx
0x1404e1a15 push rbp
0x1404e1a16 push r15
0x1404e1a18 push r14
0x1404e1a1a push r13
0x1404e1a1c push r12
0x1404e1a1e push rsi
0x1404e1a1f push rdi
0x1404e1a20 push rbx
0x1404e1a21 sub rsp, 0xe8
0x1404e1a28 lea rbp, [rdx + 0x80]
0x1404e1a2f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1a36 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1a3c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1a43 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1a49 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1a52 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1a5c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1a66 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1a6f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1a78 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1a81 cmp byte ptr [rbp + 0x1487], 0
0x1404e1a88 je 0x1404e1b36
0x1404e1a8e movabs rax, 0x8000000000000002
0x1404e1a98 mov rdx, qword ptr [rbp + 0x1468]
0x1404e1a9f cmp rdx, rax
0x1404e1aa2 jl 0x1404e1abb
0x1404e1aa4 test rdx, rdx
0x1404e1aa7 je 0x1404e1abb
0x1404e1aa9 mov r8d, 1
0x1404e1aaf mov rcx, qword ptr [rbp + 0x1460]
0x1404e1ab6 call 0x140613c20
0x1404e1abb mov byte ptr [rbp + 0x14a7], 0
0x1404e1ac2 mov byte ptr [rbp + 0x14a6], 1
0x1404e1ac9 mov byte ptr [rbp + 0x14a5], 1
0x1404e1ad0 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1ad9 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1ae2 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1aeb movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1af5 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1aff movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1b08 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1b0e movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1b15 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1b1b movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1b22 add rsp, 0xe8
0x1404e1b29 pop rbx
0x1404e1b2a pop rdi
0x1404e1b2b pop rsi
0x1404e1b2c pop r12
0x1404e1b2e pop r13
0x1404e1b30 pop r14
0x1404e1b32 pop r15
0x1404e1b34 pop rbp
0x1404e1b35 ret 
0x1404e1b36 mov byte ptr [rbp + 0x14a7], 0
0x1404e1b3d mov byte ptr [rbp + 0x14a6], 1
0x1404e1b44 mov byte ptr [rbp + 0x14a5], 1
0x1404e1b4b movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1b54 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1b5d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1b66 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1b70 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1b7a movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1b83 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1b89 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1b90 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1b96 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1b9d add rsp, 0xe8
0x1404e1ba4 pop rbx
0x1404e1ba5 pop rdi
0x1404e1ba6 pop rsi
0x1404e1ba7 pop r12
0x1404e1ba9 pop r13
0x1404e1bab pop r14
0x1404e1bad pop r15
0x1404e1baf pop rbp
0x1404e1bb0 ret 
0x1404e1bb1 nop word ptr cs:[rax + rax]
0x1404e1bc0 mov qword ptr [rsp + 0x10], rdx
0x1404e1bc5 push rbp
0x1404e1bc6 push r15
0x1404e1bc8 push r14
0x1404e1bca push r13
0x1404e1bcc push r12
0x1404e1bce push rsi
0x1404e1bcf push rdi
0x1404e1bd0 push rbx
0x1404e1bd1 sub rsp, 0xe8
0x1404e1bd8 lea rbp, [rdx + 0x80]
0x1404e1bdf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1be6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1bec movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1bf3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1bf9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1c02 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1c0c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1c16 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1c1f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1c28 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1c31 lea rcx, [rbp + 0xc30]
0x1404e1c38 call 0x140268ca0
0x1404e1c3d movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1c45 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1c4d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1c56 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1c5f movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1c68 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1c71 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1c77 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1c7d movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1c83 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1c89 add rsp, 0xe8
0x1404e1c90 pop rbx
0x1404e1c91 pop rdi
0x1404e1c92 pop rsi
0x1404e1c93 pop r12
0x1404e1c95 pop r13
0x1404e1c97 pop r14
0x1404e1c99 pop r15
0x1404e1c9b pop rbp
0x1404e1c9c ret 
0x1404e1c9d nop dword ptr [rax]
0x1404e1ca0 mov qword ptr [rsp + 0x10], rdx
0x1404e1ca5 push rbp
0x1404e1ca6 push r15
0x1404e1ca8 push r14
0x1404e1caa push r13
0x1404e1cac push r12
0x1404e1cae push rsi
0x1404e1caf push rdi
0x1404e1cb0 push rbx
0x1404e1cb1 sub rsp, 0xe8
0x1404e1cb8 lea rbp, [rdx + 0x80]
0x1404e1cbf movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1cc6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1ccc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1cd3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1cd9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1ce2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1cec movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1cf6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1cff movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1d08 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e1d11 cmp byte ptr [rbp + 0x14a3], 0
0x1404e1d18 je 0x1404e1d8c
0x1404e1d1a lea rcx, [rbp + 0x600]
0x1404e1d21 call 0x1405e9e10
0x1404e1d26 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1d2f movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1d38 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1d41 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1d4b movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1d55 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1d5e movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1d64 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1d6b movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1d71 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1d78 add rsp, 0xe8
0x1404e1d7f pop rbx
0x1404e1d80 pop rdi
0x1404e1d81 pop rsi
0x1404e1d82 pop r12
0x1404e1d84 pop r13
0x1404e1d86 pop r14
0x1404e1d88 pop r15
0x1404e1d8a pop rbp
0x1404e1d8b ret 
0x1404e1d8c movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1d95 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1d9e movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1da7 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1db1 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1dbb movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1dc4 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1dca movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1dd1 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1dd7 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1dde add rsp, 0xe8
0x1404e1de5 pop rbx
0x1404e1de6 pop rdi
0x1404e1de7 pop rsi
0x1404e1de8 pop r12
0x1404e1dea pop r13
0x1404e1dec pop r14
0x1404e1dee pop r15
0x1404e1df0 pop rbp
0x1404e1df1 ret 
0x1404e1df2 nop word ptr cs:[rax + rax]
0x1404e1e00 mov qword ptr [rsp + 0x10], rdx
0x1404e1e05 push rbp
0x1404e1e06 push r15
0x1404e1e08 push r14
0x1404e1e0a push r13
0x1404e1e0c push r12
0x1404e1e0e push rsi
0x1404e1e0f push rdi
0x1404e1e10 push rbx
0x1404e1e11 sub rsp, 0xe8
0x1404e1e18 lea rbp, [rdx + 0x80]
0x1404e1e1f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1e26 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1e2c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1e33 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1e39 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1e42 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1e4c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1e56 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1e5f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1e68 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1e71 cmp byte ptr [rbp + 0x14a4], 0
0x1404e1e78 je 0x1404e1ee6
0x1404e1e7a lea rcx, [rbp + 0x600]
0x1404e1e81 call 0x1405e8b60
0x1404e1e86 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1e8e movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1e96 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1e9f movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1ea8 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1eb1 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1eba movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1ec0 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1ec6 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1ecc movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1ed2 add rsp, 0xe8
0x1404e1ed9 pop rbx
0x1404e1eda pop rdi
0x1404e1edb pop rsi
0x1404e1edc pop r12
0x1404e1ede pop r13
0x1404e1ee0 pop r14
0x1404e1ee2 pop r15
0x1404e1ee4 pop rbp
0x1404e1ee5 ret 
0x1404e1ee6 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1eee movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1ef6 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1eff movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1f08 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1f11 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1f1a movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1f20 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1f26 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1f2c movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1f32 add rsp, 0xe8
0x1404e1f39 pop rbx
0x1404e1f3a pop rdi
0x1404e1f3b pop rsi
0x1404e1f3c pop r12
0x1404e1f3e pop r13
0x1404e1f40 pop r14
0x1404e1f42 pop r15
0x1404e1f44 pop rbp
0x1404e1f45 ret 
0x1404e1f46 nop word ptr cs:[rax + rax]
0x1404e1f50 mov qword ptr [rsp + 0x10], rdx
0x1404e1f55 push rbp
0x1404e1f56 push r15
0x1404e1f58 push r14
0x1404e1f5a push r13
0x1404e1f5c push r12
0x1404e1f5e push rsi
0x1404e1f5f push rdi
0x1404e1f60 push rbx
0x1404e1f61 sub rsp, 0xe8
0x1404e1f68 lea rbp, [rdx + 0x80]
0x1404e1f6f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1f76 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1f7c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1f83 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1f89 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1f92 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1f9c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1fa6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1faf movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1fb8 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e1fc0 lea rcx, [rbp + 0xc30]
0x1404e1fc7 call 0x1405fdb80
0x1404e1fcc movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1fd4 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1fdc movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1fe5 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1fee movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1ff7 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2000 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2006 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e200c movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2012 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2018 add rsp, 0xe8
0x1404e201f pop rbx
0x1404e2020 pop rdi
0x1404e2021 pop rsi
0x1404e2022 pop r12
0x1404e2024 pop r13
0x1404e2026 pop r14
0x1404e2028 pop r15
0x1404e202a pop rbp
0x1404e202b ret 
0x1404e202c nop dword ptr [rax]
0x1404e2030 mov qword ptr [rsp + 0x10], rdx
0x1404e2035 push rbp
0x1404e2036 push r15
0x1404e2038 push r14
0x1404e203a push r13
0x1404e203c push r12
0x1404e203e push rsi
0x1404e203f push rdi
0x1404e2040 push rbx
0x1404e2041 sub rsp, 0xe8
0x1404e2048 lea rbp, [rdx + 0x80]
0x1404e204f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e2056 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e205c movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e2063 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e2069 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e2072 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e207c movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e2086 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e208f movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e2098 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e20a1 mov ecx, dword ptr [rbp + 0x12a0]
0x1404e20a7 add ecx, -4
0x1404e20aa cmp cx, 0xe
0x1404e20ae mov eax, 7
0x1404e20b3 cmovb eax, ecx
0x1404e20b6 cmp ax, 7
0x1404e20ba jne 0x1404e2131
0x1404e20bc test byte ptr [rbp + 0x14a6], 1
0x1404e20c3 je 0x1404e20d1
0x1404e20c5 lea rcx, [rbp + 0x12a0]
0x1404e20cc call 0x1405e8000
0x1404e20d1 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e20d9 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e20e1 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e20ea movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e20f3 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e20fc movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2105 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e210b movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e2111 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2117 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e211d add rsp, 0xe8
0x1404e2124 pop rbx
0x1404e2125 pop rdi
0x1404e2126 pop rsi
0x1404e2127 pop r12
0x1404e2129 pop r13
0x1404e212b pop r14
0x1404e212d pop r15
0x1404e212f pop rbp
0x1404e2130 ret 
0x1404e2131 movzx eax, ax
0x1404e2134 cmp eax, 5
0x1404e2137 je 0x1404e2150
0x1404e2139 cmp eax, 0xd
0x1404e213c jne 0x1404e20d1
0x1404e213e test byte ptr [rbp + 0x14a5], 1
0x1404e2145 je 0x1404e20d1
0x1404e2147 mov rdx, qword ptr [rbp + 0x12a8]
0x1404e214e jmp 0x1404e2177
0x1404e2150 test byte ptr [rbp + 0x14a7], 1
0x1404e2157 je 0x1404e20d1
0x1404e215d mov rdx, qword ptr [rbp + 0x12a8]
0x1404e2164 movabs rax, 0x8000000000000002
0x1404e216e cmp rdx, rax
0x1404e2171 jl 0x1404e20d1
0x1404e2177 test rdx, rdx
0x1404e217a je 0x1404e20d1
0x1404e2180 mov rcx, qword ptr [rbp + 0x12b0]
0x1404e2187 mov r8d, 1
0x1404e218d call 0x140613c20
0x1404e2192 jmp 0x1404e20d1
0x1404e2197 nop word ptr [rax + rax]
0x1404e21a0 mov qword ptr [rsp + 0x10], rdx
0x1404e21a5 push rbp
0x1404e21a6 push r15
0x1404e21a8 push r14
0x1404e21aa push r13
0x1404e21ac push r12
0x1404e21ae push rsi
0x1404e21af push rdi
0x1404e21b0 push rbx
0x1404e21b1 sub rsp, 0xe8
0x1404e21b8 lea rbp, [rdx + 0x80]
0x1404e21bf movaps xmmword ptr [rsp + 0x40], xmm15
0x1404e21c5 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e21cb movaps xmmword ptr [rsp + 0x60], xmm13
0x1404e21d1 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e21d7 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e21e0 movaps xmmword ptr [rsp + 0x90], xmm10
0x1404e21e9 movaps xmmword ptr [rsp + 0xa0], xmm9
0x1404e21f2 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e21fb movaps xmmword ptr [rsp + 0xc0], xmm7
0x1404e2203 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e220b lea rcx, [rbp + 0x13e0]
0x1404e2212 call 0x140609210
0x1404e2217 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e221f movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e2227 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2230 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e2239 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2242 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e224b movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2251 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e2257 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e225d movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2263 add rsp, 0xe8
0x1404e226a pop rbx
0x1404e226b pop rdi
0x1404e226c pop rsi
0x1404e226d pop r12
0x1404e226f pop r13
0x1404e2271 pop r14
0x1404e2273 pop r15
0x1404e2275 pop rbp
0x1404e2276 ret 
0x1404e2277 nop word ptr [rax + rax]
0x1404e2280 mov qword ptr [rsp + 0x10], rdx
0x1404e2285 push rbp
0x1404e2286 push r15
0x1404e2288 push r14
0x1404e228a push r13
0x1404e228c push r12
0x1404e228e push rsi
0x1404e228f push rdi
0x1404e2290 push rbx
0x1404e2291 sub rsp, 0xe8
0x1404e2298 lea rbp, [rdx + 0x80]
0x1404e229f movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e22a6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e22ac movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e22b3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e22b9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e22c2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e22cc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e22d6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e22df movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e22e8 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e22f1 cmp qword ptr [rbp + 0x13e0], 0
0x1404e22f9 je 0x1404e232a
0x1404e22fb lea rax, [rbp + 0x13e8]
0x1404e2302 mov qword ptr [rbp + 0x1458], rax
0x1404e2309 lea rax, [rbp + 0x1458]
0x1404e2310 mov qword ptr [rbp + 0x1430], rax
0x1404e2317 lea rcx, [rip + 0x2507672]
0x1404e231e lea rdx, [rbp + 0x1430]
0x1404e2325 call 0x1400a2580
0x1404e232a movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e2332 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e233a movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2343 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e234c movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2355 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e235e movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2364 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e236a movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2370 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2376 add rsp, 0xe8
0x1404e237d pop rbx
0x1404e237e pop rdi
0x1404e237f pop rsi
0x1404e2380 pop r12
0x1404e2382 pop r13
0x1404e2384 pop r14
0x1404e2386 pop r15
0x1404e2388 pop rbp
0x1404e2389 ret 
0x1404e238a nop word ptr [rax + rax]
0x1404e2390 mov qword ptr [rsp + 0x10], rdx
0x1404e2395 push rbp
0x1404e2396 push r15
0x1404e2398 push r14
0x1404e239a push r13
0x1404e239c push r12
0x1404e239e push rsi
0x1404e239f push rdi
0x1404e23a0 push rbx
0x1404e23a1 sub rsp, 0xe8
0x1404e23a8 lea rbp, [rdx + 0x80]
0x1404e23af movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e23b6 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e23bc movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e23c3 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e23c9 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e23d2 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e23dc movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e23e6 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e23ef movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e23f8 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e2401 lea rcx, [rbp + 0x13b0]
0x1404e2408 call 0x1406035f0
0x1404e240d movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e2415 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e241d movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2426 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e242f movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2438 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2441 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2447 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e244d movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2453 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2459 add rsp, 0xe8
0x1404e2460 pop rbx
0x1404e2461 pop rdi
0x1404e2462 pop rsi
0x1404e2463 pop r12
0x1404e2465 pop r13
0x1404e2467 pop r14
0x1404e2469 pop r15
0x1404e246b pop rbp
0x1404e246c ret 