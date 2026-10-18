
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14037c160: 55                          	push	rbp
14037c161: 41 56                       	push	r14
14037c163: 56                          	push	rsi
14037c164: 57                          	push	rdi
14037c165: 53                          	push	rbx
14037c166: 48 81 ec a0 00 00 00        	sub	rsp, 0xa0
14037c16d: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14037c175: 48 c7 45 18 fe ff ff ff     	mov	qword ptr [rbp + 0x18], -0x2
14037c17d: 4c 89 ce                    	mov	rsi, r9
14037c180: 4c 89 c7                    	mov	rdi, r8
14037c183: 89 d3                       	mov	ebx, edx
14037c185: 41 89 ce                    	mov	r14d, ecx
14037c188: 0f b6 05 7a 8a 2b 03        	movzx	eax, byte ptr [rip + 0x32b8a7a] # 0x143634c09
14037c18f: b9 02 00 00 00              	mov	ecx, 0x2
14037c194: ba 01 00 00 00              	mov	edx, 0x1
14037c199: e8 72 7a 29 00              	call	0x140613c10 <.text+0x612c10>
14037c19e: 48 85 c0                    	test	rax, rax
14037c1a1: 74 32                       	je	0x14037c1d5 <.text+0x37b1d5>
14037c1a3: 44 88 30                    	mov	byte ptr [rax], r14b
14037c1a6: 88 58 01                    	mov	byte ptr [rax + 0x1], bl
14037c1a9: 4c 8d 0d a0 d6 64 02        	lea	r9, [rip + 0x264d6a0]   # 0x1429c9850
14037c1b0: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
14037c1b4: 48 89 fa                    	mov	rdx, rdi
14037c1b7: 49 89 c0                    	mov	r8, rax
14037c1ba: e8 01 1e 93 00              	call	0x140cadfc0 <.text+0xcacfc0>
14037c1bf: 31 c0                       	xor	eax, eax
14037c1c1: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
14037c1c5: 71 1d                       	jno	0x14037c1e4 <.text+0x37b1e4>
14037c1c7: 48 81 c4 a0 00 00 00        	add	rsp, 0xa0
14037c1ce: 5b                          	pop	rbx
14037c1cf: 5f                          	pop	rdi
14037c1d0: 5e                          	pop	rsi
14037c1d1: 41 5e                       	pop	r14
14037c1d3: 5d                          	pop	rbp
14037c1d4: c3                          	ret
14037c1d5: b9 01 00 00 00              	mov	ecx, 0x1
14037c1da: ba 02 00 00 00              	mov	edx, 0x2
14037c1df: e8 1c ce 55 02              	call	0x1428d9000 <.text+0x28d8000>
14037c1e4: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
14037c1e8: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
14037c1ec: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
14037c1f0: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
14037c1f4: 48 89 e8                    	mov	rax, rbp
14037c1f7: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
14037c1fb: 48 8d 05 ce f8 cb ff        	lea	rax, [rip - 0x340732]   # 0x14003bad0 <.text+0x3aad0>
14037c202: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
14037c206: 48 8d 05 f3 d6 64 02        	lea	rax, [rip + 0x264d6f3]  # 0x1429c9900
14037c20d: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
14037c211: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
14037c219: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
14037c221: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
14037c225: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
14037c229: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
14037c231: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14037c235: 48 89 f2                    	mov	rdx, rsi
14037c238: e8 53 d1 55 02              	call	0x1428d9390 <.text+0x28d8390>
14037c23d: 0f 0b                       	ud2
14037c23f: 90                          	nop
14037c240: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14037c245: 55                          	push	rbp
14037c246: 41 56                       	push	r14
14037c248: 56                          	push	rsi
14037c249: 57                          	push	rdi
14037c24a: 53                          	push	rbx
14037c24b: 48 83 ec 20                 	sub	rsp, 0x20
14037c24f: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14037c256: 48 8b 55 00                 	mov	rdx, qword ptr [rbp]
14037c25a: 48 85 d2                    	test	rdx, rdx
14037c25d: 74 0f                       	je	0x14037c26e <.text+0x37b26e>
14037c25f: 48 8b 4d 08                 	mov	rcx, qword ptr [rbp + 0x8]
14037c263: 41 b8 01 00 00 00           	mov	r8d, 0x1
14037c269: e8 b2 79 29 00              	call	0x140613c20 <.text+0x612c20>
14037c26e: 90                          	nop
14037c26f: 48 83 c4 20                 	add	rsp, 0x20
14037c273: 5b                          	pop	rbx
14037c274: 5f                          	pop	rdi
14037c275: 5e                          	pop	rsi
14037c276: 41 5e                       	pop	r14
14037c278: 5d                          	pop	rbp
14037c279: c3                          	ret
14037c27a: cc                          	int3
14037c27b: cc                          	int3
14037c27c: cc                          	int3
14037c27d: cc                          	int3
14037c27e: cc                          	int3
14037c27f: cc                          	int3
14037c280: 41 57                       	push	r15
14037c282: 41 56                       	push	r14
14037c284: 41 55                       	push	r13
14037c286: 41 54                       	push	r12
14037c288: 56                          	push	rsi
14037c289: 57                          	push	rdi
14037c28a: 55                          	push	rbp
14037c28b: 53                          	push	rbx
14037c28c: 48 83 ec 78                 	sub	rsp, 0x78
14037c290: 48 89 d7                    	mov	rdi, rdx
14037c293: 48 89 ce                    	mov	rsi, rcx
14037c296: 83 39 01                    	cmp	dword ptr [rcx], 0x1
14037c299: 75 60                       	jne	0x14037c2fb <.text+0x37b2fb>
14037c29b: 48 8b 87 a0 05 00 00        	mov	rax, qword ptr [rdi + 0x5a0]
14037c2a2: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
14037c2a7: 48 39 46 78                 	cmp	qword ptr [rsi + 0x78], rax
14037c2ab: 0f 84 3b 01 00 00           	je	0x14037c3ec <.text+0x37b3ec>
14037c2b1: 48 8d 05 58 dc 64 02        	lea	rax, [rip + 0x264dc58]  # 0x1429c9f10
14037c2b8: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
14037c2bd: 48 c7 44 24 50 01 00 00 00  	mov	qword ptr [rsp + 0x50], 0x1
14037c2c6: 48 c7 44 24 58 08 00 00 00  	mov	qword ptr [rsp + 0x58], 0x8
