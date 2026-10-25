
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416164c0: 55                          	push	rbp
1416164c1: 41 57                       	push	r15
1416164c3: 41 56                       	push	r14
1416164c5: 41 54                       	push	r12
1416164c7: 56                          	push	rsi
1416164c8: 57                          	push	rdi
1416164c9: 53                          	push	rbx
1416164ca: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
1416164d1: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1416164d9: 48 c7 45 78 fe ff ff ff     	mov	qword ptr [rbp + 0x78], -0x2
1416164e1: 4d 85 c0                    	test	r8, r8
1416164e4: 0f 84 9c 00 00 00           	je	0x141616586 <.text+0x1615586>
1416164ea: 4c 89 c3                    	mov	rbx, r8
1416164ed: 48 89 d7                    	mov	rdi, rdx
1416164f0: 48 89 ce                    	mov	rsi, rcx
1416164f3: 49 c1 e1 04                 	shl	r9, 0x4
1416164f7: 4f 8d 34 49                 	lea	r14, [r9 + 2*r9]
1416164fb: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
1416164ff: 4c 89 f9                    	mov	rcx, r15
141616502: 4c 89 f2                    	mov	rdx, r14
141616505: 41 b8 22 00 00 00           	mov	r8d, 0x22
14161650b: 41 b1 01                    	mov	r9b, 0x1
14161650e: e8 cd e8 5b ff              	call	0x140bd4de0 <.text+0xbd3de0>
141616513: 4c 8d 65 68                 	lea	r12, [rbp + 0x68]
141616517: 41 b8 04 00 00 00           	mov	r8d, 0x4
14161651d: 4c 89 e1                    	mov	rcx, r12
141616520: 4c 89 fa                    	mov	rdx, r15
141616523: e8 f8 e9 5b ff              	call	0x140bd4f20 <.text+0xbd3f20>
141616528: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
14161652d: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
141616532: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
141616537: 4c 8d 05 13 94 66 01        	lea	r8, [rip + 0x1669413]   # 0x142c7f951
14161653e: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
141616542: 41 b9 11 00 00 00           	mov	r9d, 0x11
141616548: 48 89 fa                    	mov	rdx, rdi
14161654b: e8 70 c8 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141616550: 83 7d 18 0b                 	cmp	dword ptr [rbp + 0x18], 0xb
141616554: 0f 85 a3 00 00 00           	jne	0x1416165fd <.text+0x16155fd>
14161655a: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14161655e: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141616562: 0f 10 45 20                 	movups	xmm0, xmmword ptr [rbp + 0x20]
141616566: 0f 10 4d 30                 	movups	xmm1, xmmword ptr [rbp + 0x30]
14161656a: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
14161656e: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141616571: 48 89 f0                    	mov	rax, rsi
141616574: 48 81 c4 00 01 00 00        	add	rsp, 0x100
14161657b: 5b                          	pop	rbx
14161657c: 5f                          	pop	rdi
14161657d: 5e                          	pop	rsi
14161657e: 41 5c                       	pop	r12
141616580: 41 5e                       	pop	r14
141616582: 41 5f                       	pop	r15
141616584: 5d                          	pop	rbp
141616585: c3                          	ret
141616586: 48 8d 05 f1 6d 48 01        	lea	rax, [rip + 0x1486df1]  # 0x142a9d37e
14161658d: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141616591: 48 c7 45 70 3a 00 00 00     	mov	qword ptr [rbp + 0x70], 0x3a
141616599: 48 8d 05 c8 93 66 01        	lea	rax, [rip + 0x16693c8]  # 0x142c7f968
1416165a0: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
1416165a4: 48 8d 05 f5 d2 f1 fe        	lea	rax, [rip - 0x10e2d0b]  # 0x1405338a0 <.text+0x5328a0>
1416165ab: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
1416165af: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
1416165b3: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
1416165b7: 48 8d 05 d2 38 af fe        	lea	rax, [rip - 0x150c72e]  # 0x140109e90 <.text+0x108e90>
1416165be: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1416165c2: 48 8d 05 67 91 66 01        	lea	rax, [rip + 0x1669167]  # 0x142c7f730
1416165c9: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1416165cd: 48 c7 45 c8 03 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3
1416165d5: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
1416165dd: 48 8d 45 18                 	lea	rax, [rbp + 0x18]
1416165e1: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
1416165e5: 48 c7 45 d8 02 00 00 00     	mov	qword ptr [rbp - 0x28], 0x2
1416165ed: 48 8d 15 94 91 66 01        	lea	rdx, [rip + 0x1669194]  # 0x142c7f788
1416165f4: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1416165f8: e8 93 2d 2c 01              	call	0x1428d9390 <.text+0x28d8390>
1416165fd: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
141616601: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
141616605: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
141616609: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
14161660d: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
141616611: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
141616615: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
141616619: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
14161661d: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
141616621: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
141616625: 48 8d 05 7c 9b 66 01        	lea	rax, [rip + 0x1669b7c]  # 0x142c801a8
14161662c: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141616631: 48 8d 0d b0 84 66 01        	lea	rcx, [rip + 0x16684b0]  # 0x142c7eae8
141616638: 4c 8d 0d 89 84 66 01        	lea	r9, [rip + 0x1668489]   # 0x142c7eac8
14161663f: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
141616643: ba 2b 00 00 00              	mov	edx, 0x2b
141616648: e8 13 31 2c 01              	call	0x1428d9760 <.text+0x28d8760>
14161664d: 0f 0b                       	ud2
14161664f: 90                          	nop
141616650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141616655: 55                          	push	rbp
141616656: 41 57                       	push	r15
141616658: 41 56                       	push	r14
14161665a: 41 54                       	push	r12
14161665c: 56                          	push	rsi
14161665d: 57                          	push	rdi
14161665e: 53                          	push	rbx
14161665f: 48 83 ec 40                 	sub	rsp, 0x40
141616663: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14161666a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
14161666e: e8 fd 83 1b ff              	call	0x1407cea70 <.text+0x7cda70>
141616673: 90                          	nop
141616674: 48 83 c4 40                 	add	rsp, 0x40
141616678: 5b                          	pop	rbx
141616679: 5f                          	pop	rdi
14161667a: 5e                          	pop	rsi
14161667b: 41 5c                       	pop	r12
14161667d: 41 5e                       	pop	r14
14161667f: 41 5f                       	pop	r15
141616681: 5d                          	pop	rbp
141616682: c3                          	ret
