0x140b8e4a0 push rbp
0x140b8e4a1 push r15
0x140b8e4a3 push r14
0x140b8e4a5 push r13
0x140b8e4a7 push r12
0x140b8e4a9 push rsi
0x140b8e4aa push rdi
0x140b8e4ab push rbx
0x140b8e4ac sub rsp, 0x5a8
0x140b8e4b3 lea rbp, [rsp + 0x80]
0x140b8e4bb movaps xmmword ptr [rbp + 0x510], xmm7
0x140b8e4c2 movaps xmmword ptr [rbp + 0x500], xmm6
0x140b8e4c9 mov qword ptr [rbp + 0x4f8], 0xfffffffffffffffe
0x140b8e4d4 mov rdi, rdx
0x140b8e4d7 mov rsi, rcx
0x140b8e4da mov qword ptr [rbp + 0x3a8], 0
0x140b8e4e5 mov qword ptr [rbp + 0x3b0], 4
0x140b8e4f0 xorps xmm0, xmm0
0x140b8e4f3 movups xmmword ptr [rbp + 0x3b8], xmm0
0x140b8e4fa mov qword ptr [rbp + 0x3c8], 4
0x140b8e505 movups xmmword ptr [rbp + 0x3d0], xmm0
0x140b8e50c mov byte ptr [rbp + 0x4f6], 1
0x140b8e513 lea rbx, [rbp + 0x298]
0x140b8e51a mov rcx, rbx
0x140b8e51d call 0x1417f9390
0x140b8e522 xor eax, eax
0x140b8e524 cmp rax, qword ptr [rbp + 0x298]
0x140b8e52b jno 0x140b8e637
0x140b8e531 mov rax, qword ptr [rip + 0x2a9de70]
0x140b8e538 mov rax, qword ptr [rax]
0x140b8e53b cmp rax, 1
0x140b8e53f jbe 0x140b8e5dc
0x140b8e545 mov byte ptr [rbp + 0x4f6], 1
0x140b8e54c lea rcx, [rip + 0x1f6de7d]
0x140b8e553 call 0x14067dd80
0x140b8e558 movups xmm0, xmmword ptr [rax]
0x140b8e55b mov eax, dword ptr [rax + 0x10]
0x140b8e55e mov qword ptr [rbp - 0x20], 2
0x140b8e566 lea rcx, [rip + 0x1f6dd33] ; b'tg_text'
0x140b8e56d mov qword ptr [rbp - 0x18], rcx
0x140b8e571 mov qword ptr [rbp - 0x10], 7
0x140b8e579 lea rdx, [rip + 0x1f6de40]
0x140b8e580 mov qword ptr [rbp], rdx
0x140b8e584 mov qword ptr [rbp + 8], 1
0x140b8e58c mov qword ptr [rbp + 0x10], 8
0x140b8e594 mov qword ptr [rbp - 0x50], 0
0x140b8e59c xorps xmm1, xmm1
0x140b8e59f movups xmmword ptr [rbp + 0x18], xmm1
0x140b8e5a3 mov qword ptr [rbp - 0x48], rcx
0x140b8e5a7 mov qword ptr [rbp - 0x40], 7
0x140b8e5af mov qword ptr [rbp - 0x38], 0
0x140b8e5b7 movups xmmword ptr [rbp - 0x30], xmm0
0x140b8e5bb mov dword ptr [rbp - 8], 1
0x140b8e5c2 mov dword ptr [rbp - 4], eax
0x140b8e5c5 mov byte ptr [rbp + 0x4f6], 1
0x140b8e5cc lea rcx, [rbp + 0x3e0]
0x140b8e5d3 lea rdx, [rbp - 0x50]
0x140b8e5d7 call 0x140d4a330
0x140b8e5dc movzx eax, byte ptr [rip + 0x2aa6626]
0x140b8e5e3 mov ecx, 5
0x140b8e5e8 mov edx, 1
0x140b8e5ed call 0x140613c10
0x140b8e5f2 test rax, rax
0x140b8e5f5 je 0x140b8ee43
0x140b8e5fb mov byte ptr [rax + 4], 0x53
0x140b8e5ff mov dword ptr [rax], 0x552d6e65
0x140b8e605 mov qword ptr [rbp + 0x480], 5
0x140b8e610 mov qword ptr [rbp + 0x488], rax
0x140b8e617 mov qword ptr [rbp + 0x490], 5
0x140b8e622 mov rax, qword ptr [rip + 0x2a9dd7f]
0x140b8e629 mov rax, qword ptr [rax]
0x140b8e62c cmp rax, 4
0x140b8e630 jae 0x140b8e667
0x140b8e632 jmp 0x140b8e71f
0x140b8e637 mov rax, qword ptr [rbp + 0x2a8]
0x140b8e63e mov qword ptr [rbp + 0x490], rax
0x140b8e645 movups xmm0, xmmword ptr [rbp + 0x298]
0x140b8e64c movaps xmmword ptr [rbp + 0x480], xmm0
0x140b8e653 mov rax, qword ptr [rip + 0x2a9dd4e]
0x140b8e65a mov rax, qword ptr [rax]
0x140b8e65d cmp rax, 4
0x140b8e661 jb 0x140b8e71f
0x140b8e667 lea rax, [rbp + 0x480]
0x140b8e66e mov qword ptr [rbp + 0x298], rax
0x140b8e675 lea rax, [rip - 0xb52bac]
0x140b8e67c mov qword ptr [rbp + 0x2a0], rax
0x140b8e683 mov byte ptr [rbp + 0x4f5], 1
0x140b8e68a lea rcx, [rip + 0x1f6dcd7]
0x140b8e691 call 0x14067dd80
0x140b8e696 movups xmm0, xmmword ptr [rax]
0x140b8e699 mov eax, dword ptr [rax + 0x10]
0x140b8e69c mov qword ptr [rbp - 0x20], 4
0x140b8e6a4 lea rcx, [rip + 0x1f6dbf5] ; b'tg_text'
0x140b8e6ab mov qword ptr [rbp - 0x18], rcx
0x140b8e6af mov qword ptr [rbp - 0x10], 7
0x140b8e6b7 lea rdx, [rip + 0x1f6dc9a]
0x140b8e6be mov qword ptr [rbp], rdx
0x140b8e6c2 mov qword ptr [rbp + 8], 1
0x140b8e6ca mov qword ptr [rbp + 0x10], rbx
0x140b8e6ce mov qword ptr [rbp + 0x18], 1
0x140b8e6d6 mov qword ptr [rbp + 0x20], 0
0x140b8e6de mov qword ptr [rbp - 0x50], 0
0x140b8e6e6 mov qword ptr [rbp - 0x48], rcx
0x140b8e6ea mov qword ptr [rbp - 0x40], 7
0x140b8e6f2 mov qword ptr [rbp - 0x38], 0
0x140b8e6fa movups xmmword ptr [rbp - 0x30], xmm0
0x140b8e6fe mov dword ptr [rbp - 8], 1
0x140b8e705 mov dword ptr [rbp - 4], eax
0x140b8e708 mov byte ptr [rbp + 0x4f5], 1
0x140b8e70f lea rcx, [rbp + 0x3e0]
0x140b8e716 lea rdx, [rbp - 0x50]
0x140b8e71a call 0x140d4a330
0x140b8e71f mov r15, qword ptr [rbp + 0x480]
0x140b8e726 mov rbx, qword ptr [rbp + 0x488]
0x140b8e72d mov r14, qword ptr [rbp + 0x490]
0x140b8e734 movzx eax, byte ptr [rip + 0x2aa64ce]
0x140b8e73b mov ecx, 0x80
0x140b8e740 mov edx, 8
0x140b8e745 call 0x140613c10
0x140b8e74a test rax, rax
0x140b8e74d je 0x140b8eca2
0x140b8e753 mov dword ptr [rax], 0
0x140b8e759 mov dword ptr [rax + 0x78], 0
0x140b8e760 mov qword ptr [rbp - 0x50], 1
0x140b8e768 mov qword ptr [rbp - 0x48], rax
0x140b8e76c mov qword ptr [rbp - 0x40], 1
0x140b8e774 mov qword ptr [rbp - 0x38], 1
0x140b8e77c movzx eax, byte ptr [rip + 0x2aa6486]
0x140b8e783 mov ecx, 0xf
0x140b8e788 mov edx, 1
0x140b8e78d call 0x140613c10
0x140b8e792 test rax, rax
0x140b8e795 je 0x140b8eccb
0x140b8e79b mov r12, rax
0x140b8e79e movabs rax, 0x6e616d6f52207765
0x140b8e7a8 mov qword ptr [r12 + 7], rax
0x140b8e7ad movabs rax, 0x654e2073656d6954
0x140b8e7b7 mov qword ptr [r12], rax
0x140b8e7bb movzx eax, byte ptr [rip + 0x2aa6447]
0x140b8e7c2 mov ecx, 5
0x140b8e7c7 mov edx, 1
0x140b8e7cc call 0x140613c10
0x140b8e7d1 test rax, rax
0x140b8e7d4 je 0x140b8ecf4
0x140b8e7da mov r13, rax
0x140b8e7dd mov byte ptr [rax + 4], 0x6c
0x140b8e7e1 mov dword ptr [rax], 0x61697241
0x140b8e7e7 movzx eax, byte ptr [rip + 0x2aa641b]
0x140b8e7ee mov ecx, 0xd
0x140b8e7f3 mov edx, 1
0x140b8e7f8 call 0x140613c10
0x140b8e7fd test rax, rax
0x140b8e800 je 0x140b8ed24
0x140b8e806 mov qword ptr [rbp + 0x4e8], rdi
0x140b8e80d movabs rcx, 0x534d20736e615320
0x140b8e817 mov qword ptr [rax + 5], rcx
0x140b8e81b movabs rcx, 0x61532063696d6f43
0x140b8e825 mov qword ptr [rbp + 0x478], rax
0x140b8e82c mov qword ptr [rax], rcx
0x140b8e82f movzx eax, byte ptr [rip + 0x2aa63d3]
0x140b8e836 mov ecx, 6
0x140b8e83b mov edx, 1
0x140b8e840 call 0x140613c10
0x140b8e845 test rax, rax
0x140b8e848 je 0x140b8ed5b
0x140b8e84e mov word ptr [rax + 4], 0x7463
0x140b8e854 mov dword ptr [rax], 0x61706d49
0x140b8e85a mov rdi, rax
0x140b8e85d movzx eax, byte ptr [rip + 0x2aa63a5]
0x140b8e864 mov ecx, 0xb
0x140b8e869 mov edx, 1
0x140b8e86e call 0x140613c10
0x140b8e873 test rax, rax
0x140b8e876 je 0x140b8ed92
0x140b8e87c movabs rcx, 0x2072656972756f43
0x140b8e886 mov qword ptr [rax], rcx
0x140b8e889 mov dword ptr [rax + 7], 0x77654e20
0x140b8e890 movups xmm0, xmmword ptr [rbp - 0x50]
0x140b8e894 movups xmm1, xmmword ptr [rbp - 0x40]
0x140b8e898 movups xmmword ptr [rbp + 0x2c0], xmm1
0x140b8e89f movups xmmword ptr [rbp + 0x2b0], xmm0
0x140b8e8a6 mov qword ptr [rbp + 0x298], r15
0x140b8e8ad mov qword ptr [rbp + 0x2a0], rbx
0x140b8e8b4 mov qword ptr [rbp + 0x2a8], r14
0x140b8e8bb mov qword ptr [rbp + 0x2d0], 0xf
0x140b8e8c6 mov qword ptr [rbp + 0x2d8], r12
0x140b8e8cd mov qword ptr [rbp + 0x2e0], 0xf
0x140b8e8d8 mov qword ptr [rbp + 0x2e8], 5
0x140b8e8e3 mov qword ptr [rbp + 0x2f0], r13
0x140b8e8ea mov qword ptr [rbp + 0x2f8], 5
0x140b8e8f5 mov qword ptr [rbp + 0x300], 0xd
0x140b8e900 mov rcx, qword ptr [rbp + 0x478]
0x140b8e907 mov qword ptr [rbp + 0x308], rcx
0x140b8e90e mov qword ptr [rbp + 0x310], 0xd
0x140b8e919 mov qword ptr [rbp + 0x318], 6
0x140b8e924 mov qword ptr [rbp + 0x320], rdi
0x140b8e92b mov qword ptr [rbp + 0x328], 6
0x140b8e936 mov qword ptr [rbp + 0x330], 0xb
0x140b8e941 mov qword ptr [rbp + 0x338], rax
0x140b8e948 mov qword ptr [rbp + 0x340], 0xb
0x140b8e953 movups xmm6, xmmword ptr [rip + 0x1f6d4b6]
0x140b8e95a movups xmmword ptr [rbp + 0x358], xmm6
0x140b8e961 movups xmm7, xmmword ptr [rip + 0x1f6d498]
0x140b8e968 movups xmmword ptr [rbp + 0x348], xmm7
0x140b8e96f movups xmmword ptr [rbp + 0x368], xmm7
0x140b8e976 movups xmmword ptr [rbp + 0x378], xmm6
0x140b8e97d movups xmmword ptr [rbp + 0x388], xmm7
0x140b8e984 movups xmmword ptr [rbp + 0x398], xmm6
0x140b8e98b mov byte ptr [rbp + 0x4f4], 1
0x140b8e992 lea rcx, [rbp - 0x50]
0x140b8e996 call 0x140e128c0
0x140b8e99b mov byte ptr [rbp + 0x4f3], 1
0x140b8e9a2 lea rcx, [rbp + 0x248]
0x140b8e9a9 mov edx, 0x400
0x140b8e9ae mov r8d, 0x400
0x140b8e9b4 call 0x1417f99e0
0x140b8e9b9 mov rdi, qword ptr [rbp + 0x4e8]
0x140b8e9c0 mov byte ptr [rbp + 0x4f7], 1
0x140b8e9c7 lea rcx, [rbp + 0x480]
0x140b8e9ce movabs rdx, 0x40000000400
0x140b8e9d8 mov r8d, 0x2b
0x140b8e9de call 0x140bd52a0
0x140b8e9e3 mov byte ptr [rbp + 0x4f7], 1
0x140b8e9ea lea rcx, [rbp + 0x3e0]
0x140b8e9f1 lea rdx, [rbp + 0x480]
0x140b8e9f8 call 0x140bd5370
0x140b8e9fd mov byte ptr [rbp + 0x4f7], 1
0x140b8ea04 lea rbx, [rbp + 0x480]
0x140b8ea0b lea rdx, [rbp + 0x3e0]
0x140b8ea12 mov rcx, rbx
0x140b8ea15 call 0x140bd53b0
0x140b8ea1a mov qword ptr [rbp + 0x430], 1
0x140b8ea25 mov byte ptr [rbp + 0x4f7], 1
0x140b8ea2c lea rax, [rbp + 0x430]
0x140b8ea33 mov qword ptr [rsp + 0x28], rax
0x140b8ea38 mov qword ptr [rsp + 0x20], rbx
0x140b8ea3d lea r8, [rip + 0x1f6d724] ; b'Glyph atlascreate glyph atlas texturecrates/tg-text/src/lib.rs'
0x140b8ea44 lea rcx, [rbp + 0x3e0]
0x140b8ea4b mov r9d, 0xb
0x140b8ea51 mov rdx, rdi
0x140b8ea54 call 0x1417faa80
0x140b8ea59 cmp dword ptr [rbp + 0x3e0], 0xb
0x140b8ea60 jne 0x140b8edd0
0x140b8ea66 mov rax, qword ptr [rbp + 0x418]
0x140b8ea6d mov qword ptr [rbp + 0x460], rax
0x140b8ea74 movups xmm0, xmmword ptr [rbp + 0x3e8]
0x140b8ea7b movups xmm1, xmmword ptr [rbp + 0x3f8]
0x140b8ea82 movups xmm2, xmmword ptr [rbp + 0x408]
0x140b8ea89 movaps xmmword ptr [rbp + 0x450], xmm2
0x140b8ea90 movaps xmmword ptr [rbp + 0x440], xmm1
0x140b8ea97 movaps xmmword ptr [rbp + 0x430], xmm0
0x140b8ea9e mov rax, qword ptr [rbp + 0x3d8]
0x140b8eaa5 mov qword ptr [rbp + 0x4b0], rax
0x140b8eaac movups xmm0, xmmword ptr [rbp + 0x3a8]
0x140b8eab3 movups xmm1, xmmword ptr [rbp + 0x3b8]
0x140b8eaba movups xmm2, xmmword ptr [rbp + 0x3c8]
0x140b8eac1 movaps xmmword ptr [rbp + 0x4a0], xmm2
0x140b8eac8 movaps xmmword ptr [rbp + 0x490], xmm1
0x140b8eacf movaps xmmword ptr [rbp + 0x480], xmm0
0x140b8ead6 call 0x140d4d740
0x140b8eadb mov r14, rax
0x140b8eade test byte ptr [rax], 1
0x140b8eae1 je 0x140b8eaf4
0x140b8eae3 mov rax, r14
0x140b8eae6 add rax, 8
0x140b8eaea mov rdi, qword ptr [r14 + 8]
0x140b8eaee mov rbx, qword ptr [r14 + 0x10]
0x140b8eaf2 jmp 0x140b8eb15
0x140b8eaf4 call 0x140d64e00
0x140b8eaf9 mov rdi, rax
0x140b8eafc mov rbx, rdx
0x140b8eaff mov rax, r14
0x140b8eb02 add rax, 8
0x140b8eb06 mov qword ptr [r14], 1
0x140b8eb0d mov qword ptr [r14 + 8], rdi
0x140b8eb11 mov qword ptr [r14 + 0x10], rdx
0x140b8eb15 lea rcx, [rdi + 1]
0x140b8eb19 mov qword ptr [rax], rcx
0x140b8eb1c lea rdx, [rbp + 0x298]
0x140b8eb23 mov r8d, 0x110
0x140b8eb29 mov rcx, rsi
0x140b8eb2c call 0x142923c00
0x140b8eb31 lea rcx, [rsi + 0x110]
0x140b8eb38 lea rdx, [rbp - 0x50]
0x140b8eb3c mov r8d, 0x298
0x140b8eb42 call 0x142923c00
0x140b8eb47 movups xmm0, xmmword ptr [rbp + 0x288]
0x140b8eb4e movups xmmword ptr [rsi + 0x3e8], xmm0
0x140b8eb55 movups xmm0, xmmword ptr [rbp + 0x248]
0x140b8eb5c movups xmm1, xmmword ptr [rbp + 0x258]
0x140b8eb63 movups xmm2, xmmword ptr [rbp + 0x268]
0x140b8eb6a movups xmm3, xmmword ptr [rbp + 0x278]
0x140b8eb71 movups xmmword ptr [rsi + 0x3d8], xmm3
0x140b8eb78 movups xmmword ptr [rsi + 0x3c8], xmm2
0x140b8eb7f movups xmmword ptr [rsi + 0x3b8], xmm1
0x140b8eb86 movups xmmword ptr [rsi + 0x3a8], xmm0
0x140b8eb8d mov qword ptr [rsi + 0x3f8], 0
0x140b8eb98 mov qword ptr [rsi + 0x400], 4
0x140b8eba3 mov qword ptr [rsi + 0x408], 0
0x140b8ebae movaps xmm0, xmmword ptr [rbp + 0x430]
0x140b8ebb5 movaps xmm1, xmmword ptr [rbp + 0x440]
0x140b8ebbc movaps xmm2, xmmword ptr [rbp + 0x450]
0x140b8ebc3 movups xmmword ptr [rsi + 0x460], xmm0
0x140b8ebca movups xmmword ptr [rsi + 0x470], xmm1
0x140b8ebd1 movups xmmword ptr [rsi + 0x480], xmm2
0x140b8ebd8 mov rax, qword ptr [rbp + 0x460]
0x140b8ebdf mov qword ptr [rsi + 0x490], rax
0x140b8ebe6 movaps xmm0, xmmword ptr [rbp + 0x480]
0x140b8ebed movups xmmword ptr [rsi + 0x410], xmm0
0x140b8ebf4 movaps xmm0, xmmword ptr [rbp + 0x490]
0x140b8ebfb movaps xmm1, xmmword ptr [rbp + 0x4a0]
0x140b8ec02 movups xmmword ptr [rsi + 0x420], xmm0
0x140b8ec09 movups xmmword ptr [rsi + 0x430], xmm1
0x140b8ec10 mov rax, qword ptr [rbp + 0x4b0]
0x140b8ec17 mov qword ptr [rsi + 0x440], rax
0x140b8ec1e movups xmmword ptr [rsi + 0x498], xmm7
0x140b8ec25 movups xmmword ptr [rsi + 0x4a8], xmm6
0x140b8ec2c mov qword ptr [rsi + 0x4b8], rdi
0x140b8ec33 mov qword ptr [rsi + 0x4c0], rbx
0x140b8ec3a mov qword ptr [rsi + 0x448], 0
0x140b8ec45 mov qword ptr [rsi + 0x450], 8
0x140b8ec50 mov qword ptr [rsi + 0x458], 0
0x140b8ec5b movabs rax, 0x100000001
0x140b8ec65 mov qword ptr [rsi + 0x4c8], rax
0x140b8ec6c mov dword ptr [rsi + 0x4d0], 0x3f800000
0x140b8ec76 mov byte ptr [rsi + 0x4d4], 0
0x140b8ec7d mov rax, rsi
0x140b8ec80 movaps xmm6, xmmword ptr [rbp + 0x500]
0x140b8ec87 movaps xmm7, xmmword ptr [rbp + 0x510]
0x140b8ec8e add rsp, 0x5a8
0x140b8ec95 pop rbx
0x140b8ec96 pop rdi
0x140b8ec97 pop rsi
0x140b8ec98 pop r12
0x140b8ec9a pop r13
0x140b8ec9c pop r14
0x140b8ec9e pop r15
0x140b8eca0 pop rbp
0x140b8eca1 ret 
0x140b8eca2 mov qword ptr [rbp + 0x4e0], rbx
0x140b8eca9 mov qword ptr [rbp + 0x4e8], r15
0x140b8ecb0 lea r8, [rip + 0x215bf79]
0x140b8ecb7 mov ecx, 8
0x140b8ecbc mov edx, 0x80
0x140b8ecc1 call 0x1428d8fe3
0x140b8ecc6 jmp 0x140b8ee60
0x140b8eccb mov qword ptr [rbp + 0x4e0], rbx
0x140b8ecd2 mov qword ptr [rbp + 0x4e8], r15
0x140b8ecd9 lea r8, [rip + 0x1f6d220]
0x140b8ece0 mov ecx, 1
0x140b8ece5 mov edx, 0xf
0x140b8ecea call 0x1428d8fe3
0x140b8ecef jmp 0x140b8ee60
0x140b8ecf4 mov qword ptr [rbp + 0x4d8], r12
0x140b8ecfb mov qword ptr [rbp + 0x4e0], rbx
0x140b8ed02 mov qword ptr [rbp + 0x4e8], r15
0x140b8ed09 lea r8, [rip + 0x1f6d1f0]
0x140b8ed10 mov ecx, 1
0x140b8ed15 mov edx, 5
0x140b8ed1a call 0x1428d8fe3
0x140b8ed1f jmp 0x140b8ee60
0x140b8ed24 mov qword ptr [rbp + 0x4d0], r13
0x140b8ed2b mov qword ptr [rbp + 0x4d8], r12
0x140b8ed32 mov qword ptr [rbp + 0x4e0], rbx
0x140b8ed39 mov qword ptr [rbp + 0x4e8], r15
0x140b8ed40 lea r8, [rip + 0x1f6d1b9]
0x140b8ed47 mov ecx, 1
0x140b8ed4c mov edx, 0xd
0x140b8ed51 call 0x1428d8fe3
0x140b8ed56 jmp 0x140b8ee60
0x140b8ed5b mov qword ptr [rbp + 0x4d0], r13
0x140b8ed62 mov qword ptr [rbp + 0x4d8], r12
0x140b8ed69 mov qword ptr [rbp + 0x4e0], rbx
0x140b8ed70 mov qword ptr [rbp + 0x4e8], r15
0x140b8ed77 lea r8, [rip + 0x1f6d182]
0x140b8ed7e mov ecx, 1
0x140b8ed83 mov edx, 6
0x140b8ed88 call 0x1428d8fe3
0x140b8ed8d jmp 0x140b8ee60
0x140b8ed92 mov qword ptr [rbp + 0x470], rdi
0x140b8ed99 mov qword ptr [rbp + 0x4d0], r13
0x140b8eda0 mov qword ptr [rbp + 0x4d8], r12
0x140b8eda7 mov qword ptr [rbp + 0x4e0], rbx
0x140b8edae mov qword ptr [rbp + 0x4e8], r15
0x140b8edb5 lea r8, [rip + 0x1f6d144]
0x140b8edbc mov ecx, 1
0x140b8edc1 mov edx, 0xb
0x140b8edc6 call 0x1428d8fe3
0x140b8edcb jmp 0x140b8ee60
0x140b8edd0 movups xmm0, xmmword ptr [rbp + 0x420]
0x140b8edd7 movaps xmmword ptr [rbp + 0x4c0], xmm0
0x140b8edde movups xmm0, xmmword ptr [rbp + 0x3e0]
0x140b8ede5 movups xmm1, xmmword ptr [rbp + 0x3f0]
0x140b8edec movups xmm2, xmmword ptr [rbp + 0x400]
0x140b8edf3 movups xmm3, xmmword ptr [rbp + 0x410]
0x140b8edfa movaps xmmword ptr [rbp + 0x4b0], xmm3
0x140b8ee01 movaps xmmword ptr [rbp + 0x4a0], xmm2
0x140b8ee08 movaps xmmword ptr [rbp + 0x490], xmm1
0x140b8ee0f movaps xmmword ptr [rbp + 0x480], xmm0
0x140b8ee16 lea rax, [rip + 0x1f6d38b]
0x140b8ee1d mov qword ptr [rsp + 0x20], rax
0x140b8ee22 lea rcx, [rip + 0x1f6d34a] ; b'create glyph atlas texturecrates/tg-text/src/lib.rs'
0x140b8ee29 lea r9, [rip + 0x1f6d060]
0x140b8ee30 lea r8, [rbp + 0x480]
0x140b8ee37 mov edx, 0x1a
0x140b8ee3c call 0x1428d9760
0x140b8ee41 jmp 0x140b8ee60
0x140b8ee43 mov byte ptr [rbp + 0x4f6], 1
0x140b8ee4a lea r8, [rip + 0x1f6d0af]
0x140b8ee51 mov ecx, 1
0x140b8ee56 mov edx, 5
0x140b8ee5b call 0x1428d8fe3
0x140b8ee60 ud2 
0x140b8ee62 nop word ptr cs:[rax + rax]
0x140b8ee70 mov qword ptr [rsp + 0x10], rdx
0x140b8ee75 push rbp
0x140b8ee76 push r15
0x140b8ee78 push r14
0x140b8ee7a push r13
0x140b8ee7c push r12
0x140b8ee7e push rsi
0x140b8ee7f push rdi
0x140b8ee80 push rbx
0x140b8ee81 sub rsp, 0x58
0x140b8ee85 lea rbp, [rdx + 0x80]
0x140b8ee8c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8ee91 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8ee96 lea rcx, [rbp - 0x50]
0x140b8ee9a call 0x1417f9260
0x140b8ee9f movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8eea4 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8eea9 add rsp, 0x58
0x140b8eead pop rbx
0x140b8eeae pop rdi
0x140b8eeaf pop rsi
0x140b8eeb0 pop r12
0x140b8eeb2 pop r13
0x140b8eeb4 pop r14
0x140b8eeb6 pop r15
0x140b8eeb8 pop rbp
0x140b8eeb9 ret 
0x140b8eeba nop word ptr [rax + rax]
0x140b8eec0 mov qword ptr [rsp + 0x10], rdx
0x140b8eec5 push rbp
0x140b8eec6 push r15
0x140b8eec8 push r14
0x140b8eeca push r13
0x140b8eecc push r12
0x140b8eece push rsi
0x140b8eecf push rdi
0x140b8eed0 push rbx
0x140b8eed1 sub rsp, 0x58
0x140b8eed5 lea rbp, [rdx + 0x80]
0x140b8eedc movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8eee1 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8eee6 mov edx, 0xf
0x140b8eeeb mov r8d, 1
0x140b8eef1 mov rcx, qword ptr [rbp + 0x4d8]
0x140b8eef8 call 0x140613c20
0x140b8eefd movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8ef02 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8ef07 add rsp, 0x58
0x140b8ef0b pop rbx
0x140b8ef0c pop rdi
0x140b8ef0d pop rsi
0x140b8ef0e pop r12
0x140b8ef10 pop r13
0x140b8ef12 pop r14
0x140b8ef14 pop r15
0x140b8ef16 pop rbp
0x140b8ef17 ret 
0x140b8ef18 nop dword ptr [rax + rax]
0x140b8ef20 mov qword ptr [rsp + 0x10], rdx
0x140b8ef25 push rbp
0x140b8ef26 push r15
0x140b8ef28 push r14
0x140b8ef2a push r13
0x140b8ef2c push r12
0x140b8ef2e push rsi
0x140b8ef2f push rdi
0x140b8ef30 push rbx
0x140b8ef31 sub rsp, 0x58
0x140b8ef35 lea rbp, [rdx + 0x80]
0x140b8ef3c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8ef41 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8ef46 mov edx, 5
0x140b8ef4b mov r8d, 1
0x140b8ef51 mov rcx, qword ptr [rbp + 0x4d0]
0x140b8ef58 call 0x140613c20
0x140b8ef5d movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8ef62 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8ef67 add rsp, 0x58
0x140b8ef6b pop rbx
0x140b8ef6c pop rdi
0x140b8ef6d pop rsi
0x140b8ef6e pop r12
0x140b8ef70 pop r13
0x140b8ef72 pop r14
0x140b8ef74 pop r15
0x140b8ef76 pop rbp
0x140b8ef77 ret 
0x140b8ef78 nop dword ptr [rax + rax]
0x140b8ef80 mov qword ptr [rsp + 0x10], rdx
0x140b8ef85 push rbp
0x140b8ef86 push r15
0x140b8ef88 push r14
0x140b8ef8a push r13
0x140b8ef8c push r12
0x140b8ef8e push rsi
0x140b8ef8f push rdi
0x140b8ef90 push rbx
0x140b8ef91 sub rsp, 0x58
0x140b8ef95 lea rbp, [rdx + 0x80]
0x140b8ef9c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8efa1 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8efa6 mov edx, 0xd
0x140b8efab mov r8d, 1
0x140b8efb1 mov rcx, qword ptr [rbp + 0x478]
0x140b8efb8 call 0x140613c20
0x140b8efbd movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8efc2 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8efc7 add rsp, 0x58
0x140b8efcb pop rbx
0x140b8efcc pop rdi
0x140b8efcd pop rsi
0x140b8efce pop r12
0x140b8efd0 pop r13
0x140b8efd2 pop r14
0x140b8efd4 pop r15
0x140b8efd6 pop rbp
0x140b8efd7 ret 
0x140b8efd8 nop dword ptr [rax + rax]
0x140b8efe0 mov qword ptr [rsp + 0x10], rdx
0x140b8efe5 push rbp
0x140b8efe6 push r15
0x140b8efe8 push r14
0x140b8efea push r13
0x140b8efec push r12
0x140b8efee push rsi
0x140b8efef push rdi
0x140b8eff0 push rbx
0x140b8eff1 sub rsp, 0x58
0x140b8eff5 lea rbp, [rdx + 0x80]
0x140b8effc movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f001 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f006 mov edx, 6
0x140b8f00b mov r8d, 1
0x140b8f011 mov rcx, qword ptr [rbp + 0x470]
0x140b8f018 call 0x140613c20
0x140b8f01d movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f022 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f027 add rsp, 0x58
0x140b8f02b pop rbx
0x140b8f02c pop rdi
0x140b8f02d pop rsi
0x140b8f02e pop r12
0x140b8f030 pop r13
0x140b8f032 pop r14
0x140b8f034 pop r15
0x140b8f036 pop rbp
0x140b8f037 ret 
0x140b8f038 nop dword ptr [rax + rax]
0x140b8f040 mov qword ptr [rsp + 0x10], rdx
0x140b8f045 push rbp
0x140b8f046 push r15
0x140b8f048 push r14
0x140b8f04a push r13
0x140b8f04c push r12
0x140b8f04e push rsi
0x140b8f04f push rdi
0x140b8f050 push rbx
0x140b8f051 sub rsp, 0x58
0x140b8f055 lea rbp, [rdx + 0x80]
0x140b8f05c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f061 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f066 cmp qword ptr [rbp + 0x4e8], 0
0x140b8f06e je 0x140b8f089
0x140b8f070 mov r8d, 1
0x140b8f076 mov rcx, qword ptr [rbp + 0x4e0]
0x140b8f07d mov rdx, qword ptr [rbp + 0x4e8]
0x140b8f084 call 0x140613c20
0x140b8f089 mov byte ptr [rbp + 0x4f5], 0
0x140b8f090 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f095 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f09a add rsp, 0x58
0x140b8f09e pop rbx
0x140b8f09f pop rdi
0x140b8f0a0 pop rsi
0x140b8f0a1 pop r12
0x140b8f0a3 pop r13
0x140b8f0a5 pop r14
0x140b8f0a7 pop r15
0x140b8f0a9 pop rbp
0x140b8f0aa ret 
0x140b8f0ab nop dword ptr [rax + rax]
0x140b8f0b0 mov qword ptr [rsp + 0x10], rdx
0x140b8f0b5 push rbp
0x140b8f0b6 push r15
0x140b8f0b8 push r14
0x140b8f0ba push r13
0x140b8f0bc push r12
0x140b8f0be push rsi
0x140b8f0bf push rdi
0x140b8f0c0 push rbx
0x140b8f0c1 sub rsp, 0x58
0x140b8f0c5 lea rbp, [rdx + 0x80]
0x140b8f0cc movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f0d1 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f0d6 cmp byte ptr [rbp + 0x4f5], 0
0x140b8f0dd je 0x140b8f0fd
0x140b8f0df mov rdx, qword ptr [rbp + 0x480]
0x140b8f0e6 test rdx, rdx
0x140b8f0e9 je 0x140b8f0fd
0x140b8f0eb mov rcx, qword ptr [rbp + 0x488]
0x140b8f0f2 mov r8d, 1
0x140b8f0f8 call 0x140613c20
0x140b8f0fd mov byte ptr [rbp + 0x4f6], 1
0x140b8f104 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f109 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f10e add rsp, 0x58
0x140b8f112 pop rbx
0x140b8f113 pop rdi
0x140b8f114 pop rsi
0x140b8f115 pop r12
0x140b8f117 pop r13
0x140b8f119 pop r14
0x140b8f11b pop r15
0x140b8f11d pop rbp
0x140b8f11e ret 
0x140b8f11f nop 
0x140b8f120 mov qword ptr [rsp + 0x10], rdx
0x140b8f125 push rbp
0x140b8f126 push r15
0x140b8f128 push r14
0x140b8f12a push r13
0x140b8f12c push r12
0x140b8f12e push rsi
0x140b8f12f push rdi
0x140b8f130 push rbx
0x140b8f131 sub rsp, 0x58
0x140b8f135 lea rbp, [rdx + 0x80]
0x140b8f13c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f141 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f146 movzx ebx, byte ptr [rbp + 0x4f4]
0x140b8f14d lea rcx, [rbp + 0x298]
0x140b8f154 call 0x1417f8d60
0x140b8f159 mov byte ptr [rbp + 0x4f6], bl
0x140b8f15f movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f164 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f169 add rsp, 0x58
0x140b8f16d pop rbx
0x140b8f16e pop rdi
0x140b8f16f pop rsi
0x140b8f170 pop r12
0x140b8f172 pop r13
0x140b8f174 pop r14
0x140b8f176 pop r15
0x140b8f178 pop rbp
0x140b8f179 ret 
0x140b8f17a nop word ptr [rax + rax]
0x140b8f180 mov qword ptr [rsp + 0x10], rdx
0x140b8f185 push rbp
0x140b8f186 push r15
0x140b8f188 push r14
0x140b8f18a push r13
0x140b8f18c push r12
0x140b8f18e push rsi
0x140b8f18f push rdi
0x140b8f190 push rbx
0x140b8f191 sub rsp, 0x58
0x140b8f195 lea rbp, [rdx + 0x80]
0x140b8f19c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f1a1 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f1a6 movzx ebx, byte ptr [rbp + 0x4f3]
0x140b8f1ad lea rcx, [rbp - 0x50]
0x140b8f1b1 call 0x1417f8890
0x140b8f1b6 mov byte ptr [rbp + 0x4f4], bl
0x140b8f1bc movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f1c1 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f1c6 add rsp, 0x58
0x140b8f1ca pop rbx
0x140b8f1cb pop rdi
0x140b8f1cc pop rsi
0x140b8f1cd pop r12
0x140b8f1cf pop r13
0x140b8f1d1 pop r14
0x140b8f1d3 pop r15
0x140b8f1d5 pop rbp
0x140b8f1d6 ret 
0x140b8f1d7 nop word ptr [rax + rax]
0x140b8f1e0 mov qword ptr [rsp + 0x10], rdx
0x140b8f1e5 push rbp
0x140b8f1e6 push r15
0x140b8f1e8 push r14
0x140b8f1ea push r13
0x140b8f1ec push r12
0x140b8f1ee push rsi
0x140b8f1ef push rdi
0x140b8f1f0 push rbx
0x140b8f1f1 sub rsp, 0x58
0x140b8f1f5 lea rbp, [rdx + 0x80]
0x140b8f1fc movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f201 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f206 lea rcx, [rbp + 0x480]
0x140b8f20d call 0x140fa5650
0x140b8f212 lea rcx, [rbp + 0x430]
0x140b8f219 call 0x1405ee5e0
0x140b8f21e mov byte ptr [rbp + 0x4f7], 0
0x140b8f225 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f22a movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f22f add rsp, 0x58
0x140b8f233 pop rbx
0x140b8f234 pop rdi
0x140b8f235 pop rsi
0x140b8f236 pop r12
0x140b8f238 pop r13
0x140b8f23a pop r14
0x140b8f23c pop r15
0x140b8f23e pop rbp
0x140b8f23f ret 
0x140b8f240 mov qword ptr [rsp + 0x10], rdx
0x140b8f245 push rbp
0x140b8f246 push r15
0x140b8f248 push r14
0x140b8f24a push r13
0x140b8f24c push r12
0x140b8f24e push rsi
0x140b8f24f push rdi
0x140b8f250 push rbx
0x140b8f251 sub rsp, 0x58
0x140b8f255 lea rbp, [rdx + 0x80]
0x140b8f25c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f261 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f266 cmp byte ptr [rbp + 0x4f6], 0
0x140b8f26d jne 0x140b8f28a
0x140b8f26f movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f274 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f279 add rsp, 0x58
0x140b8f27d pop rbx
0x140b8f27e pop rdi
0x140b8f27f pop rsi
0x140b8f280 pop r12
0x140b8f282 pop r13
0x140b8f284 pop r14
0x140b8f286 pop r15
0x140b8f288 pop rbp
0x140b8f289 ret 
0x140b8f28a lea rcx, [rbp + 0x3a8]
0x140b8f291 call 0x140fa5650
0x140b8f296 jmp 0x140b8f26f
0x140b8f298 nop dword ptr [rax + rax]
0x140b8f2a0 mov qword ptr [rsp + 0x10], rdx
0x140b8f2a5 push rbp
0x140b8f2a6 push r15
0x140b8f2a8 push r14
0x140b8f2aa push r13
0x140b8f2ac push r12
0x140b8f2ae push rsi
0x140b8f2af push rdi
0x140b8f2b0 push rbx
0x140b8f2b1 sub rsp, 0x58
0x140b8f2b5 lea rbp, [rdx + 0x80]
0x140b8f2bc movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f2c1 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f2c6 movzx ebx, byte ptr [rbp + 0x4f7]
0x140b8f2cd lea rcx, [rbp + 0x248]
0x140b8f2d4 call 0x140fa6c40
0x140b8f2d9 mov byte ptr [rbp + 0x4f3], bl
0x140b8f2df movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f2e4 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f2e9 add rsp, 0x58
0x140b8f2ed pop rbx
0x140b8f2ee pop rdi
0x140b8f2ef pop rsi
0x140b8f2f0 pop r12
0x140b8f2f2 pop r13
0x140b8f2f4 pop r14
0x140b8f2f6 pop r15
0x140b8f2f8 pop rbp
0x140b8f2f9 ret 
0x140b8f2fa nop word ptr [rax + rax]
0x140b8f300 mov qword ptr [rsp + 0x10], rdx
0x140b8f305 push rbp
0x140b8f306 push r15
0x140b8f308 push r14
0x140b8f30a push r13
0x140b8f30c push r12
0x140b8f30e push rsi
0x140b8f30f push rdi
0x140b8f310 push rbx
0x140b8f311 sub rsp, 0x58
0x140b8f315 lea rbp, [rdx + 0x80]
0x140b8f31c movaps xmmword ptr [rsp + 0x30], xmm7
0x140b8f321 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b8f326 mov byte ptr [rbp + 0x4f7], 1
0x140b8f32d lea rcx, [rbp + 0x480]
0x140b8f334 call 0x1407cea70
0x140b8f339 mov byte ptr [rbp + 0x4f7], 1
0x140b8f340 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b8f345 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b8f34a add rsp, 0x58
0x140b8f34e pop rbx
0x140b8f34f pop rdi
0x140b8f350 pop rsi
0x140b8f351 pop r12
0x140b8f353 pop r13
0x140b8f355 pop r14
0x140b8f357 pop r15
0x140b8f359 pop rbp
0x140b8f35a ret 