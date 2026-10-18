0x1415c5460 55 push rbp
0x1415c5461 4157 push r15
0x1415c5463 4156 push r14
0x1415c5465 4155 push r13
0x1415c5467 4154 push r12
0x1415c5469 56 push rsi
0x1415c546a 57 push rdi
0x1415c546b 53 push rbx
0x1415c546c 4881ecc8010000 sub rsp, 0x1c8
0x1415c5473 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415c547b 0f29b530010000 movaps xmmword ptr [rbp + 0x130], xmm6
0x1415c5482 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x1415c548d 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
0x1415c5494 488b4210 mov rax, qword ptr [rdx + 0x10]
0x1415c5498 488945e8 mov qword ptr [rbp - 0x18], rax
0x1415c549c 4c8d7510 lea r14, [rbp + 0x10]
0x1415c54a0 4c89f1 mov rcx, r14
0x1415c54a3 e8282c5cff call 0x140b880d0
0x1415c54a8 488db5c1000000 lea rsi, [rbp + 0xc1]
0x1415c54af c785f400000002000000 mov dword ptr [rbp + 0xf4], 2
0x1415c54b9 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
0x1415c54c4 c6852601000002 mov byte ptr [rbp + 0x126], 2
0x1415c54cb b003 mov al, 3
0x1415c54cd 8985fc000000 mov dword ptr [rbp + 0xfc], eax
0x1415c54d3 4c8da500010000 lea r12, [rbp + 0x100]
0x1415c54da 4c8d3dcb616b01 lea r15, [rip + 0x16b61cb]
0x1415c54e1 4c8dadc0000000 lea r13, [rbp + 0xc0]
0x1415c54e8 c6852701000002 mov byte ptr [rbp + 0x127], 2
0x1415c54ef 488d3d9a616b01 lea rdi, [rip + 0x16b619a]
0x1415c54f6 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x1415c5500 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
0x1415c550b 0f1f440000 nop dword ptr [rax + rax]
0x1415c5510 4c89e1 mov rcx, r12
0x1415c5513 4c89f2 mov rdx, r14
0x1415c5516 e8356fe7ff call 0x14143c450
0x1415c551b 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x1415c5522 4885c0 test rax, rax
0x1415c5525 0f84a0040000 je 0x1415c59cb
0x1415c552b 488b8d10010000 mov rcx, qword ptr [rbp + 0x110]
0x1415c5532 488d1449 lea rdx, [rcx + rcx*2]
0x1415c5536 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1415c553e 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1415c5546 4c898db0000000 mov qword ptr [rbp + 0xb0], r9
0x1415c554d 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1415c5555 48c1e105 shl rcx, 5
0x1415c5559 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1415c555d 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1415c5562 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
0x1415c5569 0f298dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm1
0x1415c5570 4c8985e8000000 mov qword ptr [rbp + 0xe8], r8
0x1415c5577 4c89c0 mov rax, r8
0x1415c557a 48f7d8 neg rax
0x1415c557d 0f8048040000 jo 0x1415c59cb
0x1415c5583 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c5587 4883f806 cmp rax, 6
0x1415c558b 0f8724010000 ja 0x1415c56b5
0x1415c5591 48630487 movsxd rax, dword ptr [rdi + rax*4]
0x1415c5595 4801f8 add rax, rdi
0x1415c5598 ffe0 jmp rax
0x1415c559a 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x1415c55a1 0f288dd0000000 movaps xmm1, xmmword ptr [rbp + 0xd0]
0x1415c55a8 488d4558 lea rax, [rbp + 0x58]
0x1415c55ac 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415c55b0 0f1100 movups xmmword ptr [rax], xmm0
0x1415c55b3 4c89e1 mov rcx, r12
0x1415c55b6 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x1415c55bd 4989d8 mov r8, rbx
0x1415c55c0 e85b531200 call 0x1416ea920
0x1415c55c5 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
0x1415c55cc 4885d2 test rdx, rdx
0x1415c55cf 7412 je 0x1415c55e3
0x1415c55d1 41b801000000 mov r8d, 1
0x1415c55d7 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415c55de e83de604ff call 0x140613c20
0x1415c55e3 80bd0001000000 cmp byte ptr [rbp + 0x100], 0
0x1415c55ea 0f8500040000 jne 0x1415c59f0
0x1415c55f0 0fb68501010000 movzx eax, byte ptr [rbp + 0x101]
0x1415c55f7 49630487 movsxd rax, dword ptr [r15 + rax*4]
0x1415c55fb 4c01f8 add rax, r15
0x1415c55fe ffe0 jmp rax
0x1415c5600 4883bd9800000000 cmp qword ptr [rbp + 0x98], 0
0x1415c5608 0f85bb060000 jne 0x1415c5cc9
0x1415c560e 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c5612 c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c5616 3c06 cmp al, 6
0x1415c5618 0f841b060000 je 0x1415c5c39
0x1415c561e 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c5624 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c5629 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c562e 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c5632 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c5635 4c89e9 mov rcx, r13
0x1415c5638 e8537bfdff call 0x14159d190
0x1415c563d 488995a8000000 mov qword ptr [rbp + 0xa8], rdx
0x1415c5644 4885c0 test rax, rax
0x1415c5647 0f8544060000 jne 0x1415c5c91
0x1415c564d b801000000 mov eax, 1
0x1415c5652 48898598000000 mov qword ptr [rbp + 0x98], rax
0x1415c5659 e9b2feffff jmp 0x1415c5510
0x1415c565e 488b5560 mov rdx, qword ptr [rbp + 0x60]
0x1415c5662 4885d2 test rdx, rdx
0x1415c5665 0f842fffffff je 0x1415c559a
0x1415c566b 488b4d68 mov rcx, qword ptr [rbp + 0x68]
0x1415c566f 41b801000000 mov r8d, 1
0x1415c5675 e8a6e504ff call 0x140613c20
0x1415c567a e91bffffff jmp 0x1415c559a
0x1415c567f 488b4d68 mov rcx, qword ptr [rbp + 0x68]
0x1415c5683 488b5570 mov rdx, qword ptr [rbp + 0x70]
0x1415c5687 48894d78 mov qword ptr [rbp + 0x78], rcx
0x1415c568b e8e0362cff call 0x140888d70
0x1415c5690 488b5560 mov rdx, qword ptr [rbp + 0x60]
0x1415c5694 4885d2 test rdx, rdx
0x1415c5697 0f84fdfeffff je 0x1415c559a
0x1415c569d 48c1e205 shl rdx, 5
0x1415c56a1 41b808000000 mov r8d, 8
0x1415c56a7 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1415c56ab e870e504ff call 0x140613c20
0x1415c56b0 e9e5feffff jmp 0x1415c559a
0x1415c56b5 488d4d60 lea rcx, [rbp + 0x60]
0x1415c56b9 e882382dff call 0x140898f40
0x1415c56be e9d7feffff jmp 0x1415c559a
0x1415c56c3 83bda000000000 cmp dword ptr [rbp + 0xa0], 0
0x1415c56ca 0f853b060000 jne 0x1415c5d0b
0x1415c56d0 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c56d4 c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c56d8 3c06 cmp al, 6
0x1415c56da 0f8443050000 je 0x1415c5c23
0x1415c56e0 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c56e6 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c56eb 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c56f0 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c56f4 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c56f7 4c89e1 mov rcx, r12
0x1415c56fa 4c89ea mov rdx, r13
0x1415c56fd e89e78fdff call 0x14159cfa0
0x1415c5702 f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c5709 0f85e1020000 jne 0x1415c59f0
0x1415c570f 8b8504010000 mov eax, dword ptr [rbp + 0x104]
0x1415c5715 89858c000000 mov dword ptr [rbp + 0x8c], eax
0x1415c571b b801000000 mov eax, 1
0x1415c5720 488985a0000000 mov qword ptr [rbp + 0xa0], rax
0x1415c5727 e9e4fdffff jmp 0x1415c5510
0x1415c572c 83bdf400000002 cmp dword ptr [rbp + 0xf4], 2
0x1415c5733 0f85bc050000 jne 0x1415c5cf5
0x1415c5739 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c573d c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c5741 3c06 cmp al, 6
0x1415c5743 0f8432050000 je 0x1415c5c7b
0x1415c5749 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c574f 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c5754 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c5759 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c575d 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c5760 4c89e1 mov rcx, r12
0x1415c5763 4c89ea mov rdx, r13
0x1415c5766 e8b56e1200 call 0x1416ec620
0x1415c576b f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c5772 0f8578020000 jne 0x1415c59f0
0x1415c5778 8b8504010000 mov eax, dword ptr [rbp + 0x104]
0x1415c577e 8985f4000000 mov dword ptr [rbp + 0xf4], eax
0x1415c5784 488b8508010000 mov rax, qword ptr [rbp + 0x108]
0x1415c578b 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1415c5792 488d8510010000 lea rax, [rbp + 0x110]
0x1415c5799 0f1000 movups xmm0, xmmword ptr [rax]
0x1415c579c 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
0x1415c57a0 e96bfdffff jmp 0x1415c5510
0x1415c57a5 83bdf800000000 cmp dword ptr [rbp + 0xf8], 0
0x1415c57ac 0f852d050000 jne 0x1415c5cdf
0x1415c57b2 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c57b6 c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c57ba 3c06 cmp al, 6
0x1415c57bc 0f848d040000 je 0x1415c5c4f
0x1415c57c2 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c57c8 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c57cd 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c57d2 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c57d6 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c57d9 4c89e1 mov rcx, r12
0x1415c57dc 4c89ea mov rdx, r13
0x1415c57df e88c4bfdff call 0x14159a370
0x1415c57e4 f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c57eb 0f85ff010000 jne 0x1415c59f0
0x1415c57f1 f30f10b504010000 movss xmm6, dword ptr [rbp + 0x104]
0x1415c57f9 c785f800000001000000 mov dword ptr [rbp + 0xf8], 1
0x1415c5803 e908fdffff jmp 0x1415c5510
0x1415c5808 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c580c c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c5810 3c06 cmp al, 6
0x1415c5812 0f84c9030000 je 0x1415c5be1
0x1415c5818 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c581e 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c5823 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c5828 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c582c 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c582f 4c89e9 mov rcx, r13
0x1415c5832 e809e1fdff call 0x1415a3940
0x1415c5837 4885c0 test rax, rax
0x1415c583a 0f84d0fcffff je 0x1415c5510
0x1415c5840 e9c5010000 jmp 0x1415c5a0a
0x1415c5845 80bd2701000002 cmp byte ptr [rbp + 0x127], 2
0x1415c584c 0f85cf040000 jne 0x1415c5d21
0x1415c5852 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c5856 c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c585a 3c06 cmp al, 6
0x1415c585c 0f84ab030000 je 0x1415c5c0d
0x1415c5862 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c5868 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c586d 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c5872 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c5876 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c5879 488d05a8a16c01 lea rax, [rip + 0x16ca1a8]
0x1415c5880 4889442420 mov qword ptr [rsp + 0x20], rax
0x1415c5885 48c74424280a000000 mov qword ptr [rsp + 0x28], 0xa
0x1415c588e 41b90f000000 mov r9d, 0xf
0x1415c5894 4c89e1 mov rcx, r12
0x1415c5897 4c89ea mov rdx, r13
0x1415c589a 4c8d0507a16c01 lea r8, [rip + 0x16ca107]
0x1415c58a1 e88ac2fdff call 0x1415a1b30
0x1415c58a6 f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c58ad 0f853d010000 jne 0x1415c59f0
0x1415c58b3 8b8504010000 mov eax, dword ptr [rbp + 0x104]
0x1415c58b9 898594000000 mov dword ptr [rbp + 0x94], eax
0x1415c58bf 488b8508010000 mov rax, qword ptr [rbp + 0x108]
0x1415c58c6 488945f8 mov qword ptr [rbp - 8], rax
0x1415c58ca 488d8d10010000 lea rcx, [rbp + 0x110]
0x1415c58d1 488b01 mov rax, qword ptr [rcx]
0x1415c58d4 488b4905 mov rcx, qword ptr [rcx + 5]
0x1415c58d8 48894500 mov qword ptr [rbp], rax
0x1415c58dc 48894d05 mov qword ptr [rbp + 5], rcx
0x1415c58e0 0fb6851d010000 movzx eax, byte ptr [rbp + 0x11d]
0x1415c58e7 888527010000 mov byte ptr [rbp + 0x127], al
0x1415c58ed 0fb7851e010000 movzx eax, word ptr [rbp + 0x11e]
0x1415c58f4 668985f2000000 mov word ptr [rbp + 0xf2], ax
0x1415c58fb e910fcffff jmp 0x1415c5510
0x1415c5900 80bd2601000002 cmp byte ptr [rbp + 0x126], 2
0x1415c5907 0f85a6030000 jne 0x1415c5cb3
0x1415c590d 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c5911 c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c5915 3c06 cmp al, 6
0x1415c5917 0f84da020000 je 0x1415c5bf7
0x1415c591d 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c5923 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c5928 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c592d 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c5931 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c5934 4c89e1 mov rcx, r12
0x1415c5937 4c89ea mov rdx, r13
0x1415c593a e8412d1200 call 0x1416e8680
0x1415c593f f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c5946 0f85a4000000 jne 0x1415c59f0
0x1415c594c 0fb68501010000 movzx eax, byte ptr [rbp + 0x101]
0x1415c5953 888526010000 mov byte ptr [rbp + 0x126], al
0x1415c5959 e9b2fbffff jmp 0x1415c5510
0x1415c595e 80bdfc00000003 cmp byte ptr [rbp + 0xfc], 3
0x1415c5965 0f85cc030000 jne 0x1415c5d37
0x1415c596b 0fb64558 movzx eax, byte ptr [rbp + 0x58]
0x1415c596f c6455806 mov byte ptr [rbp + 0x58], 6
0x1415c5973 3c06 cmp al, 6
0x1415c5975 0f84ea020000 je 0x1415c5c65
0x1415c597b 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x1415c5981 410f104649 movups xmm0, xmmword ptr [r14 + 0x49]
0x1415c5986 410f104e58 movups xmm1, xmmword ptr [r14 + 0x58]
0x1415c598b 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c598f 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c5992 4c89e1 mov rcx, r12
0x1415c5995 4c89ea mov rdx, r13
0x1415c5998 e823381200 call 0x1416e91c0
0x1415c599d f6850001000001 test byte ptr [rbp + 0x100], 1
0x1415c59a4 754a jne 0x1415c59f0
0x1415c59a6 8b8d04010000 mov ecx, dword ptr [rbp + 0x104]
0x1415c59ac 488b8508010000 mov rax, qword ptr [rbp + 0x108]
0x1415c59b3 488945f0 mov qword ptr [rbp - 0x10], rax
0x1415c59b7 898dfc000000 mov dword ptr [rbp + 0xfc], ecx
0x1415c59bd c1e908 shr ecx, 8
0x1415c59c0 898d90000000 mov dword ptr [rbp + 0x90], ecx
0x1415c59c6 e945fbffff jmp 0x1415c5510
0x1415c59cb f6859800000001 test byte ptr [rbp + 0x98], 1
0x1415c59d2 7425 je 0x1415c59f9
0x1415c59d4 80bd2701000002 cmp byte ptr [rbp + 0x127], 2
0x1415c59db 7575 jne 0x1415c5a52
0x1415c59dd 488d0da54d6b01 lea rcx, [rip + 0x16b4da5]
0x1415c59e4 ba06000000 mov edx, 6
0x1415c59e9 e8d2413301 call 0x1428f9bc0
0x1415c59ee eb1a jmp 0x1415c5a0a
0x1415c59f0 4c8b8508010000 mov r8, qword ptr [rbp + 0x108]
0x1415c59f7 eb14 jmp 0x1415c5a0d
0x1415c59f9 488d0d714b6b01 lea rcx, [rip + 0x16b4b71]
0x1415c5a00 ba02000000 mov edx, 2
0x1415c5a05 e8b6413301 call 0x1428f9bc0
0x1415c5a0a 4989c0 mov r8, rax
0x1415c5a0d 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x1415c5a14 4c8900 mov qword ptr [rax], r8
0x1415c5a17 c7400802000000 mov dword ptr [rax + 8], 2
0x1415c5a1e 488d4d10 lea rcx, [rbp + 0x10]
0x1415c5a22 e8c9232cff call 0x140887df0
0x1415c5a27 488d4d58 lea rcx, [rbp + 0x58]
0x1415c5a2b e8008a2cff call 0x14088e430
0x1415c5a30 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x1415c5a37 0f28b530010000 movaps xmm6, xmmword ptr [rbp + 0x130]
0x1415c5a3e 4881c4c8010000 add rsp, 0x1c8
0x1415c5a45 5b pop rbx
0x1415c5a46 5f pop rdi
0x1415c5a47 5e pop rsi
0x1415c5a48 415c pop r12
0x1415c5a4a 415d pop r13
0x1415c5a4c 415e pop r14
0x1415c5a4e 415f pop r15
0x1415c5a50 5d pop rbp
0x1415c5a51 c3 ret 
0x1415c5a52 448b8df4000000 mov r9d, dword ptr [rbp + 0xf4]
0x1415c5a59 4183f902 cmp r9d, 2
0x1415c5a5d 7532 jne 0x1415c5a91
0x1415c5a5f 488d15b2516b01 lea rdx, [rip + 0x16b51b2]
0x1415c5a66 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x1415c5a6d 41b80a000000 mov r8d, 0xa
0x1415c5a73 e8f8631200 call 0x1416ebe70
0x1415c5a78 f685c000000001 test byte ptr [rbp + 0xc0], 1
0x1415c5a7f 0f840f010000 je 0x1415c5b94
0x1415c5a85 4c8b85c8000000 mov r8, qword ptr [rbp + 0xc8]
0x1415c5a8c e97cffffff jmp 0x1415c5a0d
0x1415c5a91 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
0x1415c5a95 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
0x1415c5a9c 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
0x1415c5aa3 f685f800000001 test byte ptr [rbp + 0xf8], 1
0x1415c5aaa 0f841b010000 je 0x1415c5bcb
0x1415c5ab0 f685a000000001 test byte ptr [rbp + 0xa0], 1
0x1415c5ab7 0f84e0010000 je 0x1415c5c9d
0x1415c5abd 8b85fc000000 mov eax, dword ptr [rbp + 0xfc]
0x1415c5ac3 3c03 cmp al, 3
0x1415c5ac5 0fb6c8 movzx ecx, al
0x1415c5ac8 b801000000 mov eax, 1
0x1415c5acd 0f45c1 cmovne eax, ecx
0x1415c5ad0 0f288500010000 movaps xmm0, xmmword ptr [rbp + 0x100]
0x1415c5ad7 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1415c5adb 4183f902 cmp r9d, 2
0x1415c5adf 0f8428ffffff je 0x1415c5a0d
0x1415c5ae5 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1415c5ae9 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1415c5aed 48837d5000 cmp qword ptr [rbp + 0x50], 0
0x1415c5af2 0f8555020000 jne 0x1415c5d4d
0x1415c5af8 0f2845d0 movaps xmm0, xmmword ptr [rbp - 0x30]
0x1415c5afc 488b95b8000000 mov rdx, qword ptr [rbp + 0xb8]
0x1415c5b03 0f114214 movups xmmword ptr [rdx + 0x14], xmm0
0x1415c5b07 488b4d00 mov rcx, qword ptr [rbp]
0x1415c5b0b 48894a30 mov qword ptr [rdx + 0x30], rcx
0x1415c5b0f 488b4d05 mov rcx, qword ptr [rbp + 5]
0x1415c5b13 48894a35 mov qword ptr [rdx + 0x35], rcx
0x1415c5b17 884240 mov byte ptr [rdx + 0x40], al
0x1415c5b1a 8b8d90000000 mov ecx, dword ptr [rbp + 0x90]
0x1415c5b20 89c8 mov eax, ecx
0x1415c5b22 c1e810 shr eax, 0x10
0x1415c5b25 884243 mov byte ptr [rdx + 0x43], al
0x1415c5b28 66894a41 mov word ptr [rdx + 0x41], cx
0x1415c5b2c 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1415c5b30 48894244 mov qword ptr [rdx + 0x44], rax
0x1415c5b34 4c8902 mov qword ptr [rdx], r8
0x1415c5b37 44894a08 mov dword ptr [rdx + 8], r9d
0x1415c5b3b 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1415c5b42 4889420c mov qword ptr [rdx + 0xc], rax
0x1415c5b46 8b8594000000 mov eax, dword ptr [rbp + 0x94]
0x1415c5b4c 894224 mov dword ptr [rdx + 0x24], eax
0x1415c5b4f 488b45f8 mov rax, qword ptr [rbp - 8]
0x1415c5b53 48894228 mov qword ptr [rdx + 0x28], rax
0x1415c5b57 0fb68527010000 movzx eax, byte ptr [rbp + 0x127]
0x1415c5b5e 88423d mov byte ptr [rdx + 0x3d], al
0x1415c5b61 0fb785f2000000 movzx eax, word ptr [rbp + 0xf2]
0x1415c5b68 6689423e mov word ptr [rdx + 0x3e], ax
0x1415c5b6c f30f11724c movss dword ptr [rdx + 0x4c], xmm6
0x1415c5b71 8b858c000000 mov eax, dword ptr [rbp + 0x8c]
0x1415c5b77 894250 mov dword ptr [rdx + 0x50], eax
0x1415c5b7a 0fb68526010000 movzx eax, byte ptr [rbp + 0x126]
0x1415c5b81 2401 and al, 1
0x1415c5b83 884254 mov byte ptr [rdx + 0x54], al
0x1415c5b86 488d4d10 lea rcx, [rbp + 0x10]
0x1415c5b8a e861222cff call 0x140887df0
0x1415c5b8f e993feffff jmp 0x1415c5a27
0x1415c5b94 448b8dc4000000 mov r9d, dword ptr [rbp + 0xc4]
0x1415c5b9b 488b85c8000000 mov rax, qword ptr [rbp + 0xc8]
0x1415c5ba2 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1415c5ba9 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x1415c5bb0 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
0x1415c5bb7 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
0x1415c5bbe f685f800000001 test byte ptr [rbp + 0xf8], 1
0x1415c5bc5 0f85e5feffff jne 0x1415c5ab0
0x1415c5bcb 488d0d50506b01 lea rcx, [rip + 0x16b5050]
0x1415c5bd2 ba07000000 mov edx, 7
0x1415c5bd7 e8e43f3301 call 0x1428f9bc0
0x1415c5bdc e929feffff jmp 0x1415c5a0a
0x1415c5be1 488d0d80466b01 lea rcx, [rip + 0x16b4680]
0x1415c5be8 ba10000000 mov edx, 0x10
0x1415c5bed e80e443301 call 0x1428fa000
0x1415c5bf2 e913feffff jmp 0x1415c5a0a
0x1415c5bf7 488d0d6a466b01 lea rcx, [rip + 0x16b466a]
0x1415c5bfe ba10000000 mov edx, 0x10
0x1415c5c03 e8f8433301 call 0x1428fa000
0x1415c5c08 e9fdfdffff jmp 0x1415c5a0a
0x1415c5c0d 488d0d54466b01 lea rcx, [rip + 0x16b4654]
0x1415c5c14 ba10000000 mov edx, 0x10
0x1415c5c19 e8e2433301 call 0x1428fa000
0x1415c5c1e e9e7fdffff jmp 0x1415c5a0a
0x1415c5c23 488d0d3e466b01 lea rcx, [rip + 0x16b463e]
0x1415c5c2a ba10000000 mov edx, 0x10
0x1415c5c2f e8cc433301 call 0x1428fa000
0x1415c5c34 e9d1fdffff jmp 0x1415c5a0a
0x1415c5c39 488d0d28466b01 lea rcx, [rip + 0x16b4628]
0x1415c5c40 ba10000000 mov edx, 0x10
0x1415c5c45 e8b6433301 call 0x1428fa000
0x1415c5c4a e9bbfdffff jmp 0x1415c5a0a
0x1415c5c4f 488d0d12466b01 lea rcx, [rip + 0x16b4612]
0x1415c5c56 ba10000000 mov edx, 0x10
0x1415c5c5b e8a0433301 call 0x1428fa000
0x1415c5c60 e9a5fdffff jmp 0x1415c5a0a
0x1415c5c65 488d0dfc456b01 lea rcx, [rip + 0x16b45fc]
0x1415c5c6c ba10000000 mov edx, 0x10
0x1415c5c71 e88a433301 call 0x1428fa000
0x1415c5c76 e98ffdffff jmp 0x1415c5a0a
0x1415c5c7b 488d0de6456b01 lea rcx, [rip + 0x16b45e6]
0x1415c5c82 ba10000000 mov edx, 0x10
0x1415c5c87 e874433301 call 0x1428fa000
0x1415c5c8c e979fdffff jmp 0x1415c5a0a
0x1415c5c91 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
0x1415c5c98 e970fdffff jmp 0x1415c5a0d
0x1415c5c9d 488d0d85496b01 lea rcx, [rip + 0x16b4985]
0x1415c5ca4 ba08000000 mov edx, 8
0x1415c5ca9 e8123f3301 call 0x1428f9bc0
0x1415c5cae e957fdffff jmp 0x1415c5a0a
0x1415c5cb3 488d0d6f4f6b01 lea rcx, [rip + 0x16b4f6f]
0x1415c5cba ba0b000000 mov edx, 0xb
0x1415c5cbf e8ec403301 call 0x1428f9db0
0x1415c5cc4 e941fdffff jmp 0x1415c5a0a
0x1415c5cc9 488d0da1486b01 lea rcx, [rip + 0x16b48a1]
0x1415c5cd0 ba02000000 mov edx, 2
0x1415c5cd5 e8d6403301 call 0x1428f9db0
0x1415c5cda e92bfdffff jmp 0x1415c5a0a
0x1415c5cdf 488d0d3c4f6b01 lea rcx, [rip + 0x16b4f3c]
0x1415c5ce6 ba07000000 mov edx, 7
0x1415c5ceb e8c0403301 call 0x1428f9db0
0x1415c5cf0 e915fdffff jmp 0x1415c5a0a
0x1415c5cf5 488d0d1c4f6b01 lea rcx, [rip + 0x16b4f1c]
0x1415c5cfc ba0a000000 mov edx, 0xa
0x1415c5d01 e8aa403301 call 0x1428f9db0
0x1415c5d06 e9fffcffff jmp 0x1415c5a0a
0x1415c5d0b 488d0d17496b01 lea rcx, [rip + 0x16b4917]
0x1415c5d12 ba08000000 mov edx, 8
0x1415c5d17 e894403301 call 0x1428f9db0
0x1415c5d1c e9e9fcffff jmp 0x1415c5a0a
0x1415c5d21 488d0d614a6b01 lea rcx, [rip + 0x16b4a61]
0x1415c5d28 ba06000000 mov edx, 6
0x1415c5d2d e87e403301 call 0x1428f9db0
0x1415c5d32 e9d3fcffff jmp 0x1415c5a0a
0x1415c5d37 488d0d514a6b01 lea rcx, [rip + 0x16b4a51]
0x1415c5d3e ba0f000000 mov edx, 0xf
0x1415c5d43 e868403301 call 0x1428f9db0
0x1415c5d48 e9bdfcffff jmp 0x1415c5a0a
0x1415c5d4d 488d1544436b01 lea rdx, [rip + 0x16b4344]
0x1415c5d54 4c8d0505436b01 lea r8, [rip + 0x16b4305]
0x1415c5d5b 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1415c5d5f e8bc3f3301 call 0x1428f9d20
0x1415c5d64 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
0x1415c5d6b 488901 mov qword ptr [rcx], rax
0x1415c5d6e c7410802000000 mov dword ptr [rcx + 8], 2
0x1415c5d75 e90cfeffff jmp 0x1415c5b86
0x1415c5d7a 660f1f440000 nop word ptr [rax + rax]
0x1415c5d80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5d85 55 push rbp
0x1415c5d86 4157 push r15
0x1415c5d88 4156 push r14
0x1415c5d8a 4155 push r13
0x1415c5d8c 4154 push r12
0x1415c5d8e 56 push rsi
0x1415c5d8f 57 push rdi
0x1415c5d90 53 push rbx
0x1415c5d91 4883ec48 sub rsp, 0x48
0x1415c5d95 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5d9c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5da1 488d4d58 lea rcx, [rbp + 0x58]
0x1415c5da5 e886862cff call 0x14088e430
0x1415c5daa 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5daf 4883c448 add rsp, 0x48
0x1415c5db3 5b pop rbx
0x1415c5db4 5f pop rdi
0x1415c5db5 5e pop rsi
0x1415c5db6 415c pop r12
0x1415c5db8 415d pop r13
0x1415c5dba 415e pop r14
0x1415c5dbc 415f pop r15
0x1415c5dbe 5d pop rbp
0x1415c5dbf c3 ret 
0x1415c5dc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5dc5 55 push rbp
0x1415c5dc6 4157 push r15
0x1415c5dc8 4156 push r14
0x1415c5dca 4155 push r13
0x1415c5dcc 4154 push r12
0x1415c5dce 56 push rsi
0x1415c5dcf 57 push rdi
0x1415c5dd0 53 push rbx
0x1415c5dd1 4883ec48 sub rsp, 0x48
0x1415c5dd5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5ddc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5de1 488b5560 mov rdx, qword ptr [rbp + 0x60]
0x1415c5de5 4885d2 test rdx, rdx
0x1415c5de8 7413 je 0x1415c5dfd
0x1415c5dea 48c1e205 shl rdx, 5
0x1415c5dee 41b808000000 mov r8d, 8
0x1415c5df4 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1415c5df8 e823de04ff call 0x140613c20
0x1415c5dfd 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5e02 4883c448 add rsp, 0x48
0x1415c5e06 5b pop rbx
0x1415c5e07 5f pop rdi
0x1415c5e08 5e pop rsi
0x1415c5e09 415c pop r12
0x1415c5e0b 415d pop r13
0x1415c5e0d 415e pop r14
0x1415c5e0f 415f pop r15
0x1415c5e11 5d pop rbp
0x1415c5e12 c3 ret 
0x1415c5e13 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c5e20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5e25 55 push rbp
0x1415c5e26 4157 push r15
0x1415c5e28 4156 push r14
0x1415c5e2a 4155 push r13
0x1415c5e2c 4154 push r12
0x1415c5e2e 56 push rsi
0x1415c5e2f 57 push rdi
0x1415c5e30 53 push rbx
0x1415c5e31 4883ec48 sub rsp, 0x48
0x1415c5e35 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5e3c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5e41 488d4d58 lea rcx, [rbp + 0x58]
0x1415c5e45 e8e6852cff call 0x14088e430
0x1415c5e4a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5e4f 4883c448 add rsp, 0x48
0x1415c5e53 5b pop rbx
0x1415c5e54 5f pop rdi
0x1415c5e55 5e pop rsi
0x1415c5e56 415c pop r12
0x1415c5e58 415d pop r13
0x1415c5e5a 415e pop r14
0x1415c5e5c 415f pop r15
0x1415c5e5e 5d pop rbp
0x1415c5e5f c3 ret 
0x1415c5e60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5e65 55 push rbp
0x1415c5e66 4157 push r15
0x1415c5e68 4156 push r14
0x1415c5e6a 4155 push r13
0x1415c5e6c 4154 push r12
0x1415c5e6e 56 push rsi
0x1415c5e6f 57 push rdi
0x1415c5e70 53 push rbx
0x1415c5e71 4883ec48 sub rsp, 0x48
0x1415c5e75 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5e7c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5e81 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x1415c5e88 0f288dd0000000 movaps xmm1, xmmword ptr [rbp + 0xd0]
0x1415c5e8f 488d4558 lea rax, [rbp + 0x58]
0x1415c5e93 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415c5e97 0f1100 movups xmmword ptr [rax], xmm0
0x1415c5e9a c6852501000001 mov byte ptr [rbp + 0x125], 1
0x1415c5ea1 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5ea6 4883c448 add rsp, 0x48
0x1415c5eaa 5b pop rbx
0x1415c5eab 5f pop rdi
0x1415c5eac 5e pop rsi
0x1415c5ead 415c pop r12
0x1415c5eaf 415d pop r13
0x1415c5eb1 415e pop r14
0x1415c5eb3 415f pop r15
0x1415c5eb5 5d pop rbp
0x1415c5eb6 c3 ret 
0x1415c5eb7 660f1f840000000000 nop word ptr [rax + rax]
0x1415c5ec0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5ec5 55 push rbp
0x1415c5ec6 4157 push r15
0x1415c5ec8 4156 push r14
0x1415c5eca 4155 push r13
0x1415c5ecc 4154 push r12
0x1415c5ece 56 push rsi
0x1415c5ecf 57 push rdi
0x1415c5ed0 53 push rbx
0x1415c5ed1 4883ec48 sub rsp, 0x48
0x1415c5ed5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5edc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5ee1 4883bde800000000 cmp qword ptr [rbp + 0xe8], 0
0x1415c5ee9 7419 je 0x1415c5f04
0x1415c5eeb 41b801000000 mov r8d, 1
0x1415c5ef1 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415c5ef8 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
0x1415c5eff e81cdd04ff call 0x140613c20
0x1415c5f04 c6852501000000 mov byte ptr [rbp + 0x125], 0
0x1415c5f0b 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5f10 4883c448 add rsp, 0x48
0x1415c5f14 5b pop rbx
0x1415c5f15 5f pop rdi
0x1415c5f16 5e pop rsi
0x1415c5f17 415c pop r12
0x1415c5f19 415d pop r13
0x1415c5f1b 415e pop r14
0x1415c5f1d 415f pop r15
0x1415c5f1f 5d pop rbp
0x1415c5f20 c3 ret 
0x1415c5f21 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c5f30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5f35 55 push rbp
0x1415c5f36 4157 push r15
0x1415c5f38 4156 push r14
0x1415c5f3a 4155 push r13
0x1415c5f3c 4154 push r12
0x1415c5f3e 56 push rsi
0x1415c5f3f 57 push rdi
0x1415c5f40 53 push rbx
0x1415c5f41 4883ec48 sub rsp, 0x48
0x1415c5f45 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5f4c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5f51 80bd2501000000 cmp byte ptr [rbp + 0x125], 0
0x1415c5f58 7439 je 0x1415c5f93
0x1415c5f5a 4883bde800000000 cmp qword ptr [rbp + 0xe8], 0
0x1415c5f62 7419 je 0x1415c5f7d
0x1415c5f64 41b801000000 mov r8d, 1
0x1415c5f6a 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415c5f71 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
0x1415c5f78 e8a3dc04ff call 0x140613c20
0x1415c5f7d 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5f82 4883c448 add rsp, 0x48
0x1415c5f86 5b pop rbx
0x1415c5f87 5f pop rdi
0x1415c5f88 5e pop rsi
0x1415c5f89 415c pop r12
0x1415c5f8b 415d pop r13
0x1415c5f8d 415e pop r14
0x1415c5f8f 415f pop r15
0x1415c5f91 5d pop rbp
0x1415c5f92 c3 ret 
0x1415c5f93 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5f98 4883c448 add rsp, 0x48
0x1415c5f9c 5b pop rbx
0x1415c5f9d 5f pop rdi
0x1415c5f9e 5e pop rsi
0x1415c5f9f 415c pop r12
0x1415c5fa1 415d pop r13
0x1415c5fa3 415e pop r14
0x1415c5fa5 415f pop r15
0x1415c5fa7 5d pop rbp
0x1415c5fa8 c3 ret 
0x1415c5fa9 0f1f8000000000 nop dword ptr [rax]
0x1415c5fb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c5fb5 55 push rbp
0x1415c5fb6 4157 push r15
0x1415c5fb8 4156 push r14
0x1415c5fba 4155 push r13
0x1415c5fbc 4154 push r12
0x1415c5fbe 56 push rsi
0x1415c5fbf 57 push rdi
0x1415c5fc0 53 push rbx
0x1415c5fc1 4883ec48 sub rsp, 0x48
0x1415c5fc5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c5fcc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415c5fd1 488d4d10 lea rcx, [rbp + 0x10]
0x1415c5fd5 e826452cff call 0x14088a500
0x1415c5fda 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415c5fdf 4883c448 add rsp, 0x48
0x1415c5fe3 5b pop rbx
0x1415c5fe4 5f pop rdi
0x1415c5fe5 5e pop rsi
0x1415c5fe6 415c pop r12
0x1415c5fe8 415d pop r13
0x1415c5fea 415e pop r14
0x1415c5fec 415f pop r15
0x1415c5fee 5d pop rbp
0x1415c5fef c3 ret 
