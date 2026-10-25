
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14231d6c0: 55                          	push	rbp
14231d6c1: 41 57                       	push	r15
14231d6c3: 41 56                       	push	r14
14231d6c5: 41 55                       	push	r13
14231d6c7: 41 54                       	push	r12
14231d6c9: 56                          	push	rsi
14231d6ca: 57                          	push	rdi
14231d6cb: 53                          	push	rbx
14231d6cc: 48 81 ec 98 02 00 00        	sub	rsp, 0x298
14231d6d3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14231d6db: 44 0f 29 bd 00 02 00 00     	movaps	xmmword ptr [rbp + 0x200], xmm15
14231d6e3: 44 0f 29 b5 f0 01 00 00     	movaps	xmmword ptr [rbp + 0x1f0], xmm14
14231d6eb: 44 0f 29 ad e0 01 00 00     	movaps	xmmword ptr [rbp + 0x1e0], xmm13
14231d6f3: 44 0f 29 a5 d0 01 00 00     	movaps	xmmword ptr [rbp + 0x1d0], xmm12
14231d6fb: 44 0f 29 9d c0 01 00 00     	movaps	xmmword ptr [rbp + 0x1c0], xmm11
14231d703: 44 0f 29 95 b0 01 00 00     	movaps	xmmword ptr [rbp + 0x1b0], xmm10
14231d70b: 44 0f 29 8d a0 01 00 00     	movaps	xmmword ptr [rbp + 0x1a0], xmm9
14231d713: 44 0f 29 85 90 01 00 00     	movaps	xmmword ptr [rbp + 0x190], xmm8
14231d71b: 0f 29 bd 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm7
14231d722: 0f 29 b5 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm6
14231d729: 48 c7 85 68 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x168], -0x2
14231d734: 4c 8b 22                    	mov	r12, qword ptr [rdx]
14231d737: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
14231d73b: 4d 8b 14 24                 	mov	r10, qword ptr [r12]
14231d73f: 45 31 db                    	xor	r11d, r11d
14231d742: 4c 89 d0                    	mov	rax, r10
14231d745: 48 2b 42 18                 	sub	rax, qword ptr [rdx + 0x18]
14231d749: 49 0f 42 c3                 	cmovb	rax, r11
14231d74d: 4c 2b 52 38                 	sub	r10, qword ptr [rdx + 0x38]
14231d751: 4d 0f 42 d3                 	cmovb	r10, r11
14231d755: 48 89 c3                    	mov	rbx, rax
14231d758: 48 c1 e3 05                 	shl	rbx, 0x5
14231d75c: 48 03 5a 08                 	add	rbx, qword ptr [rdx + 0x8]
14231d760: 48 8b 72 10                 	mov	rsi, qword ptr [rdx + 0x10]
14231d764: 48 29 c6                    	sub	rsi, rax
14231d767: 49 0f 42 f3                 	cmovb	rsi, r11
14231d76b: 48 89 4d 40                 	mov	qword ptr [rbp + 0x40], rcx
14231d76f: b8 08 00 00 00              	mov	eax, 0x8
14231d774: 48 0f 42 d8                 	cmovb	rbx, rax
14231d778: 4d 89 d6                    	mov	r14, r10
14231d77b: 49 c1 e6 05                 	shl	r14, 0x5
14231d77f: 4c 03 72 28                 	add	r14, qword ptr [rdx + 0x28]
14231d783: 48 8b 7a 30                 	mov	rdi, qword ptr [rdx + 0x30]
14231d787: 4c 29 d7                    	sub	rdi, r10
14231d78a: 49 0f 42 fb                 	cmovb	rdi, r11
14231d78e: 4c 0f 42 f0                 	cmovb	r14, rax
14231d792: 48 8b 52 40                 	mov	rdx, qword ptr [rdx + 0x40]
14231d796: 48 29 fa                    	sub	rdx, rdi
14231d799: 48 29 f2                    	sub	rdx, rsi
14231d79c: 49 89 14 24                 	mov	qword ptr [r12], rdx
14231d7a0: 48 c1 e6 05                 	shl	rsi, 0x5
14231d7a4: 48 89 9d 50 01 00 00        	mov	qword ptr [rbp + 0x150], rbx
14231d7ab: 48 01 de                    	add	rsi, rbx
14231d7ae: 48 89 75 38                 	mov	qword ptr [rbp + 0x38], rsi
14231d7b2: 48 c1 e7 05                 	shl	rdi, 0x5
14231d7b6: 4c 89 b5 58 01 00 00        	mov	qword ptr [rbp + 0x158], r14
14231d7bd: 4c 01 f7                    	add	rdi, r14
14231d7c0: 48 89 7d 48                 	mov	qword ptr [rbp + 0x48], rdi
14231d7c4: 49 8b 30                    	mov	rsi, qword ptr [r8]
14231d7c7: 49 8b 48 08                 	mov	rcx, qword ptr [r8 + 0x8]
14231d7cb: 48 8b 16                    	mov	rdx, qword ptr [rsi]
14231d7ce: 49 89 d0                    	mov	r8, rdx
14231d7d1: 4c 2b 41 18                 	sub	r8, qword ptr [rcx + 0x18]
14231d7d5: 4d 0f 42 c3                 	cmovb	r8, r11
14231d7d9: 48 2b 51 38                 	sub	rdx, qword ptr [rcx + 0x38]
14231d7dd: 49 0f 42 d3                 	cmovb	rdx, r11
14231d7e1: 4d 89 c7                    	mov	r15, r8
14231d7e4: 49 c1 e7 04                 	shl	r15, 0x4
14231d7e8: 4c 03 79 08                 	add	r15, qword ptr [rcx + 0x8]
14231d7ec: 4c 8b 51 10                 	mov	r10, qword ptr [rcx + 0x10]
14231d7f0: 4d 29 c2                    	sub	r10, r8
14231d7f3: 4d 0f 42 d3                 	cmovb	r10, r11
14231d7f7: 4c 0f 42 f8                 	cmovb	r15, rax
14231d7fb: 48 89 d7                    	mov	rdi, rdx
14231d7fe: 48 c1 e7 04                 	shl	rdi, 0x4
14231d802: 48 03 79 28                 	add	rdi, qword ptr [rcx + 0x28]
14231d806: 4c 8b 41 30                 	mov	r8, qword ptr [rcx + 0x30]
14231d80a: 49 29 d0                    	sub	r8, rdx
14231d80d: 48 0f 42 f8                 	cmovb	rdi, rax
14231d811: 4d 0f 42 c3                 	cmovb	r8, r11
14231d815: 48 8b 41 40                 	mov	rax, qword ptr [rcx + 0x40]
14231d819: 4c 29 c0                    	sub	rax, r8
14231d81c: 4c 29 d0                    	sub	rax, r10
14231d81f: 48 89 06                    	mov	qword ptr [rsi], rax
14231d822: 49 c1 e2 04                 	shl	r10, 0x4
14231d826: 4d 01 fa                    	add	r10, r15
14231d829: 4c 89 55 28                 	mov	qword ptr [rbp + 0x28], r10
14231d82d: 49 c1 e0 04                 	shl	r8, 0x4
14231d831: 49 01 f8                    	add	r8, rdi
14231d834: 4c 89 45 30                 	mov	qword ptr [rbp + 0x30], r8
14231d838: 49 8b 01                    	mov	rax, qword ptr [r9]
14231d83b: 48 89 85 10 01 00 00        	mov	qword ptr [rbp + 0x110], rax
14231d842: 4d 8b 71 08                 	mov	r14, qword ptr [r9 + 0x8]
14231d846: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
14231d84d: 48 8b 00                    	mov	rax, qword ptr [rax]
14231d850: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
14231d854: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
14231d85b: 8b 48 1c                    	mov	ecx, dword ptr [rax + 0x1c]
14231d85e: 89 8d 44 01 00 00           	mov	dword ptr [rbp + 0x144], ecx
14231d864: 48 8b 08                    	mov	rcx, qword ptr [rax]
14231d867: 48 89 8d 08 01 00 00        	mov	qword ptr [rbp + 0x108], rcx
14231d86e: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14231d872: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
14231d876: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
14231d87d: 48 8b 08                    	mov	rcx, qword ptr [rax]
14231d880: 48 89 4d 78                 	mov	qword ptr [rbp + 0x78], rcx
14231d884: 48 8b 48 08                 	mov	rcx, qword ptr [rax + 0x8]
14231d888: 48 89 8d 80 00 00 00        	mov	qword ptr [rbp + 0x80], rcx
14231d88f: 48 8b 48 18                 	mov	rcx, qword ptr [rax + 0x18]
14231d893: 48 89 8d 88 00 00 00        	mov	qword ptr [rbp + 0x88], rcx
14231d89a: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14231d89e: 48 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], rax
14231d8a5: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
14231d8ac: 48 8b 18                    	mov	rbx, qword ptr [rax]
14231d8af: f3 44 0f 10 25 cc 80 60 00  	movss	xmm12, dword ptr [rip + 0x6080cc] # 0x142925984
14231d8b8: f3 0f 10 35 44 80 60 00     	movss	xmm6, dword ptr [rip + 0x608044] # 0x142925904
14231d8c0: f3 44 0f 10 2d a7 7f 60 00  	movss	xmm13, dword ptr [rip + 0x607fa7] # 0x142925870
14231d8c9: f3 44 0f 10 35 06 80 60 00  	movss	xmm14, dword ptr [rip + 0x608006] # 0x1429258d8
14231d8d2: f3 44 0f 10 1d 1d de 6c 00  	movss	xmm11, dword ptr [rip + 0x6cde1d] # 0x1429eb6f8
14231d8db: f3 44 0f 10 3d 4c f7 6a 00  	movss	xmm15, dword ptr [rip + 0x6af74c] # 0x1429cd030
14231d8e4: 4c 89 75 68                 	mov	qword ptr [rbp + 0x68], r14
14231d8e8: 4c 8b 8d 50 01 00 00        	mov	r9, qword ptr [rbp + 0x150]
14231d8ef: 4c 8b 95 58 01 00 00        	mov	r10, qword ptr [rbp + 0x158]
14231d8f6: 4d 85 e4                    	test	r12, r12
14231d8f9: 74 25                       	je	0x14231d920 <.text+0x231c920>
14231d8fb: 4d 85 c9                    	test	r9, r9
14231d8fe: 74 40                       	je	0x14231d940 <.text+0x231c940>
14231d900: 4c 3b 4d 38                 	cmp	r9, qword ptr [rbp + 0x38]
14231d904: 74 3a                       	je	0x14231d940 <.text+0x231c940>
14231d906: 49 8d 41 20                 	lea	rax, [r9 + 0x20]
14231d90a: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
14231d911: 4c 89 95 50 01 00 00        	mov	qword ptr [rbp + 0x150], r10
14231d918: eb 4a                       	jmp	0x14231d964 <.text+0x231c964>
14231d91a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14231d920: 4c 89 95 50 01 00 00        	mov	qword ptr [rbp + 0x150], r10
14231d927: 4c 89 8d 58 01 00 00        	mov	qword ptr [rbp + 0x158], r9
14231d92e: e9 9f 00 00 00              	jmp	0x14231d9d2 <.text+0x231c9d2>
14231d933: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14231d940: 4d 85 d2                    	test	r10, r10
14231d943: 74 50                       	je	0x14231d995 <.text+0x231c995>
14231d945: 4c 3b 55 48                 	cmp	r10, qword ptr [rbp + 0x48]
14231d949: 74 4a                       	je	0x14231d995 <.text+0x231c995>
14231d94b: 49 8d 42 20                 	lea	rax, [r10 + 0x20]
14231d94f: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
14231d956: 48 c7 85 58 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x158], 0x0
14231d961: 4d 89 d1                    	mov	r9, r10
14231d964: 49 ff 04 24                 	inc	qword ptr [r12]
14231d968: 4d 8b 69 08                 	mov	r13, qword ptr [r9 + 0x8]
14231d96c: 4c 89 e8                    	mov	rax, r13
14231d96f: 48 c1 e8 20                 	shr	rax, 0x20
14231d973: 74 5d                       	je	0x14231d9d2 <.text+0x231c9d2>
14231d975: 4c 8b 95 50 01 00 00        	mov	r10, qword ptr [rbp + 0x150]
14231d97c: 4c 8b 8d 58 01 00 00        	mov	r9, qword ptr [rbp + 0x158]
14231d983: 44 89 e8                    	mov	eax, r13d
14231d986: 49 3b 46 10                 	cmp	rax, qword ptr [r14 + 0x10]
14231d98a: 0f 83 66 ff ff ff           	jae	0x14231d8f6 <.text+0x231c8f6>
14231d990: e9 93 00 00 00              	jmp	0x14231da28 <.text+0x231ca28>
14231d995: 4c 89 95 50 01 00 00        	mov	qword ptr [rbp + 0x150], r10
14231d99c: 48 c7 85 58 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x158], 0x0
14231d9a7: eb 29                       	jmp	0x14231d9d2 <.text+0x231c9d2>
14231d9a9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
14231d9b0: 4d 8d 67 10                 	lea	r12, [r15 + 0x10]
14231d9b4: 49 89 fe                    	mov	r14, rdi
14231d9b7: 48 ff 06                    	inc	qword ptr [rsi]
14231d9ba: 49 8b 57 08                 	mov	rdx, qword ptr [r15 + 0x8]
14231d9be: 48 89 d9                    	mov	rcx, rbx
14231d9c1: e8 ea 48 77 fe              	call	0x140a922b0 <.text+0xa912b0>
14231d9c6: 49 89 c5                    	mov	r13, rax
14231d9c9: 48 c1 e8 20                 	shr	rax, 0x20
14231d9cd: 4d 89 e7                    	mov	r15, r12
14231d9d0: 75 2e                       	jne	0x14231da00 <.text+0x231ca00>
14231d9d2: 4d 85 ff                    	test	r15, r15
14231d9d5: 74 06                       	je	0x14231d9dd <.text+0x231c9dd>
14231d9d7: 4c 3b 7d 28                 	cmp	r15, qword ptr [rbp + 0x28]
14231d9db: 75 d3                       	jne	0x14231d9b0 <.text+0x231c9b0>
14231d9dd: 48 85 ff                    	test	rdi, rdi
14231d9e0: 0f 84 2a 07 00 00           	je	0x14231e110 <.text+0x231d110>
14231d9e6: 48 3b 7d 30                 	cmp	rdi, qword ptr [rbp + 0x30]
14231d9ea: 0f 84 20 07 00 00           	je	0x14231e110 <.text+0x231d110>
14231d9f0: 4c 8d 77 10                 	lea	r14, [rdi + 0x10]
14231d9f4: 45 31 e4                    	xor	r12d, r12d
14231d9f7: 49 89 ff                    	mov	r15, rdi
14231d9fa: 4c 89 f7                    	mov	rdi, r14
14231d9fd: eb b8                       	jmp	0x14231d9b7 <.text+0x231c9b7>
14231d9ff: 90                          	nop
14231da00: 4d 89 e7                    	mov	r15, r12
14231da03: 4c 89 f7                    	mov	rdi, r14
14231da06: 4c 8b 95 50 01 00 00        	mov	r10, qword ptr [rbp + 0x150]
14231da0d: 4c 8b 8d 58 01 00 00        	mov	r9, qword ptr [rbp + 0x158]
14231da14: 45 31 e4                    	xor	r12d, r12d
14231da17: 4c 8b 75 68                 	mov	r14, qword ptr [rbp + 0x68]
14231da1b: 44 89 e8                    	mov	eax, r13d
14231da1e: 49 3b 46 10                 	cmp	rax, qword ptr [r14 + 0x10]
14231da22: 0f 83 ce fe ff ff           	jae	0x14231d8f6 <.text+0x231c8f6>
14231da28: 49 8b 4e 08                 	mov	rcx, qword ptr [r14 + 0x8]
14231da2c: 4c 89 ea                    	mov	rdx, r13
14231da2f: 48 c1 ea 20                 	shr	rdx, 0x20
14231da33: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
14231da37: 39 14 81                    	cmp	dword ptr [rcx + 4*rax], edx
14231da3a: 0f 85 b6 fe ff ff           	jne	0x14231d8f6 <.text+0x231c8f6>
14231da40: 48 8d 04 81                 	lea	rax, [rcx + 4*rax]
14231da44: 8b 48 04                    	mov	ecx, dword ptr [rax + 0x4]
14231da47: ba ff ff ff ff              	mov	edx, 0xffffffff
14231da4c: 48 39 d1                    	cmp	rcx, rdx
14231da4f: 0f 84 a1 fe ff ff           	je	0x14231d8f6 <.text+0x231c8f6>
14231da55: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
14231da5c: 48 39 4a 38                 	cmp	qword ptr [rdx + 0x38], rcx
14231da60: 0f 86 90 fe ff ff           	jbe	0x14231d8f6 <.text+0x231c8f6>
14231da66: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
14231da6d: 48 8b 52 28                 	mov	rdx, qword ptr [rdx + 0x28]
14231da71: 41 89 c8                    	mov	r8d, ecx
14231da74: 41 c1 e8 06                 	shr	r8d, 0x6
14231da78: 4a 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*r8]
14231da7c: 48 0f a3 ca                 	bt	rdx, rcx
14231da80: 0f 83 70 fe ff ff           	jae	0x14231d8f6 <.text+0x231c8f6>
14231da86: 49 39 8e 08 01 00 00        	cmp	qword ptr [r14 + 0x108], rcx
14231da8d: 4c 89 8d 50 01 00 00        	mov	qword ptr [rbp + 0x150], r9
14231da94: 76 11                       	jbe	0x14231daa7 <.text+0x231caa7>
14231da96: 4c 8d 1c 89                 	lea	r11, [rcx + 4*rcx]
14231da9a: 49 c1 e3 05                 	shl	r11, 0x5
14231da9e: 4d 03 9e 00 01 00 00        	add	r11, qword ptr [r14 + 0x100]
14231daa5: eb 03                       	jmp	0x14231daaa <.text+0x231caaa>
14231daa7: 45 31 db                    	xor	r11d, r11d
14231daaa: 8b 50 0c                    	mov	edx, dword ptr [rax + 0xc]
14231daad: 4d 8b 86 a8 01 00 00        	mov	r8, qword ptr [r14 + 0x1a8]
14231dab4: 4c 8d 0c d2                 	lea	r9, [rdx + 8*rdx]
14231dab8: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
14231dabf: 48 8b 92 10 01 00 00        	mov	rdx, qword ptr [rdx + 0x110]
14231dac6: 4b 39 54 c8 40              	cmp	qword ptr [r8 + 8*r9 + 0x40], rdx
14231dacb: 76 27                       	jbe	0x14231daf4 <.text+0x231caf4>
14231dacd: 4f 8d 04 c8                 	lea	r8, [r8 + 8*r9]
14231dad1: 4d 8b 48 38                 	mov	r9, qword ptr [r8 + 0x38]
14231dad5: 49 8b 14 d1                 	mov	rdx, qword ptr [r9 + 8*rdx]
14231dad9: 48 85 d2                    	test	rdx, rdx
14231dadc: 74 16                       	je	0x14231daf4 <.text+0x231caf4>
14231dade: 4d 8b 40 18                 	mov	r8, qword ptr [r8 + 0x18]
14231dae2: 48 f7 d2                    	not	rdx
14231dae5: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
14231dae9: 48 c1 e2 04                 	shl	rdx, 0x4
14231daed: 49 8b 4c 10 10              	mov	rcx, qword ptr [r8 + rdx + 0x10]
14231daf2: eb 02                       	jmp	0x14231daf6 <.text+0x231caf6>
14231daf4: 31 c9                       	xor	ecx, ecx
14231daf6: 4c 89 95 58 01 00 00        	mov	qword ptr [rbp + 0x158], r10
14231dafd: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
14231db04: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
14231db0b: 49 39 93 88 00 00 00        	cmp	qword ptr [r11 + 0x88], rdx
14231db12: 76 0c                       	jbe	0x14231db20 <.text+0x231cb20>
14231db14: 4d 8b 83 80 00 00 00        	mov	r8, qword ptr [r11 + 0x80]
14231db1b: 49 83 3c d0 00              	cmp	qword ptr [r8 + 8*rdx], 0x0
14231db20: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14231db23: 48 6b c0 58                 	imul	rax, rax, 0x58
14231db27: 83 7c 01 08 01              	cmp	dword ptr [rcx + rax + 0x8], 0x1
14231db2c: 0f 85 17 01 00 00           	jne	0x14231dc49 <.text+0x231cc49>
14231db32: 48 01 c1                    	add	rcx, rax
14231db35: 48 89 8d 48 01 00 00        	mov	qword ptr [rbp + 0x148], rcx
14231db3c: e8 3f 58 5c fe              	call	0x1408e3380 <.text+0x8e2380>
14231db41: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
14231db48: 84 c0                       	test	al, al
14231db4a: 0f 85 f9 00 00 00           	jne	0x14231dc49 <.text+0x231cc49>
14231db50: 48 8b 41 1c                 	mov	rax, qword ptr [rcx + 0x1c]
14231db54: 48 89 45 00                 	mov	qword ptr [rbp], rax
14231db58: 0f 10 41 0c                 	movups	xmm0, xmmword ptr [rcx + 0xc]
14231db5c: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
14231db60: e8 2b 4b 5c fe              	call	0x1408e2690 <.text+0x8e1690>
14231db65: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
14231db6c: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
14231db73: e8 58 aa d6 fd              	call	0x1400885d0 <.text+0x875d0>
14231db78: 66 0f 6e f8                 	movd	xmm7, eax
14231db7c: 48 c1 e8 20                 	shr	rax, 0x20
14231db80: 66 44 0f 6e c0              	movd	xmm8, eax
14231db85: 0f 57 c9                    	xorps	xmm1, xmm1
14231db88: 0f 57 d2                    	xorps	xmm2, xmm2
14231db8b: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
14231db92: e8 19 55 5c fe              	call	0x1408e30b0 <.text+0x8e20b0>
14231db97: 44 0f 28 c8                 	movaps	xmm9, xmm0
14231db9b: 44 0f 28 d1                 	movaps	xmm10, xmm1
14231db9f: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
14231dba3: 48 8d 55 f0                 	lea	rdx, [rbp - 0x10]
14231dba7: e8 94 b9 97 fe              	call	0x140c99540 <.text+0xc98540>
14231dbac: 48 8d 05 25 da d2 00        	lea	rax, [rip + 0xd2da25]   # 0x14304b5d8
14231dbb3: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
14231dbb8: f3 44 0f 11 54 24 20        	movss	dword ptr [rsp + 0x20], xmm10
14231dbbf: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
14231dbc3: 66 0f 6f cf                 	movdqa	xmm1, xmm7
14231dbc7: 66 41 0f 6f d0              	movdqa	xmm2, xmm8
14231dbcc: 41 0f 28 d9                 	movaps	xmm3, xmm9
14231dbd0: e8 2b ad 97 fe              	call	0x140c98900 <.text+0xc97900>
14231dbd5: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
14231dbdc: e8 cf 57 5c fe              	call	0x1408e33b0 <.text+0x8e23b0>
14231dbe1: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
14231dbe8: 84 c0                       	test	al, al
14231dbea: 74 0a                       	je	0x14231dbf6 <.text+0x231cbf6>
14231dbec: 80 79 54 00                 	cmp	byte ptr [rcx + 0x54], 0x0
14231dbf0: 0f 84 e8 00 00 00           	je	0x14231dcde <.text+0x231ccde>
14231dbf6: 48 83 c1 3c                 	add	rcx, 0x3c
14231dbfa: e8 31 56 51 fe              	call	0x140833230 <.text+0x832230>
14231dbff: 44 0f b6 c0                 	movzx	r8d, al
14231dc03: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
14231dc0a: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
14231dc0e: e8 5d c8 97 fe              	call	0x140c9a470 <.text+0xc99470>
14231dc13: f2 44 0f 10 85 18 01 00 00  	movsd	xmm8, qword ptr [rbp + 0x118]
14231dc1c: f2 0f 10 bd 20 01 00 00     	movsd	xmm7, qword ptr [rbp + 0x120]
14231dc24: 41 0f 28 c0                 	movaps	xmm0, xmm8
14231dc28: 0f 5c c7                    	subps	xmm0, xmm7
14231dc2b: 0f 59 c0                    	mulps	xmm0, xmm0
14231dc2e: f3 0f 16 c8                 	movshdup	xmm1, xmm0              # xmm1 = xmm0[1,1,3,3]
14231dc32: f3 0f 58 c8                 	addss	xmm1, xmm0
14231dc36: 45 0f 57 c9                 	xorps	xmm9, xmm9
14231dc3a: f3 44 0f 51 c9              	sqrtss	xmm9, xmm1
14231dc3f: 45 0f 2e cd                 	ucomiss	xmm9, xmm13
14231dc43: 0f 87 19 01 00 00           	ja	0x14231dd62 <.text+0x231cd62>
14231dc49: 4c 89 ad 38 01 00 00        	mov	qword ptr [rbp + 0x138], r13
14231dc50: 48 8b 45 78                 	mov	rax, qword ptr [rbp + 0x78]
14231dc54: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
14231dc5b: 48 8b 85 80 00 00 00        	mov	rax, qword ptr [rbp + 0x80]
14231dc62: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
14231dc69: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
14231dc70: 48 89 85 28 01 00 00        	mov	qword ptr [rbp + 0x128], rax
14231dc77: 48 8b 85 88 00 00 00        	mov	rax, qword ptr [rbp + 0x88]
14231dc7e: 48 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], rax
14231dc85: 48 c7 85 d0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xd0], 0x0
14231dc90: 48 c7 85 d8 00 00 00 08 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x8
14231dc9b: 48 c7 85 e0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xe0], 0x0
14231dca6: 4c 8d ad b0 00 00 00        	lea	r13, [rbp + 0xb0]
14231dcad: 4c 89 e9                    	mov	rcx, r13
14231dcb0: 48 8b 95 08 01 00 00        	mov	rdx, qword ptr [rbp + 0x108]
14231dcb7: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
14231dcbe: e8 7d 86 c6 fe              	call	0x140f86340 <.text+0xf85340>
14231dcc3: c6 85 c8 00 00 00 00        	mov	byte ptr [rbp + 0xc8], 0x0
14231dcca: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
14231dcd1: 4c 89 ea                    	mov	rdx, r13
14231dcd4: e8 97 32 aa ff              	call	0x141dc0f70 <.text+0x1dbff70>
14231dcd9: e9 0a fc ff ff              	jmp	0x14231d8e8 <.text+0x231c8e8>
14231dcde: 48 83 c1 3c                 	add	rcx, 0x3c
14231dce2: e8 49 55 51 fe              	call	0x140833230 <.text+0x832230>
14231dce7: 44 0f b6 c0                 	movzx	r8d, al
14231dceb: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
14231dcf2: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
14231dcf6: e8 75 c7 97 fe              	call	0x140c9a470 <.text+0xc99470>
14231dcfb: f2 0f 10 bd 18 01 00 00     	movsd	xmm7, qword ptr [rbp + 0x118]
14231dd03: f2 0f 10 85 20 01 00 00     	movsd	xmm0, qword ptr [rbp + 0x120]
14231dd0b: 0f 28 cf                    	movaps	xmm1, xmm7
14231dd0e: 0f 5c c8                    	subps	xmm1, xmm0
14231dd11: 0f 59 c9                    	mulps	xmm1, xmm1
14231dd14: f3 0f 16 d1                 	movshdup	xmm2, xmm1              # xmm2 = xmm1[1,1,3,3]
14231dd18: f3 0f 58 d1                 	addss	xmm2, xmm1
14231dd1c: 0f 57 c9                    	xorps	xmm1, xmm1
14231dd1f: f3 0f 51 ca                 	sqrtss	xmm1, xmm2
14231dd23: 41 0f 28 d4                 	movaps	xmm2, xmm12
14231dd27: f3 0f 5e d1                 	divss	xmm2, xmm1
14231dd2b: 0f 28 ce                    	movaps	xmm1, xmm6
14231dd2e: f3 0f 5c ca                 	subss	xmm1, xmm2
14231dd32: f3 0f 12 c9                 	movsldup	xmm1, xmm1              # xmm1 = xmm1[0,0,2,2]
14231dd36: 0f 28 df                    	movaps	xmm3, xmm7
14231dd39: 0f 59 d9                    	mulps	xmm3, xmm1
14231dd3c: f3 0f 12 d2                 	movsldup	xmm2, xmm2              # xmm2 = xmm2[0,0,2,2]
14231dd40: 44 0f 28 c0                 	movaps	xmm8, xmm0
14231dd44: 44 0f 59 c2                 	mulps	xmm8, xmm2
14231dd48: 44 0f 58 c3                 	addps	xmm8, xmm3
14231dd4c: 0f 59 c1                    	mulps	xmm0, xmm1
14231dd4f: 0f 59 fa                    	mulps	xmm7, xmm2
14231dd52: 0f 58 f8                    	addps	xmm7, xmm0
14231dd55: 44 0f 13 85 18 01 00 00     	movlps	qword ptr [rbp + 0x118], xmm8
14231dd5d: e9 c2 fe ff ff              	jmp	0x14231dc24 <.text+0x231cc24>
14231dd62: 48 c7 85 f0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x0
14231dd6d: 48 c7 85 f8 00 00 00 08 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x8
14231dd78: 48 c7 85 00 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x100], 0x0
14231dd83: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
14231dd87: 8b 48 2c                    	mov	ecx, dword ptr [rax + 0x2c]
14231dd8a: c6 85 66 01 00 00 01        	mov	byte ptr [rbp + 0x166], 0x1
14231dd91: e8 aa 84 97 fe              	call	0x140c96240 <.text+0xc95240>
14231dd96: 41 0f 28 c1                 	movaps	xmm0, xmm9
14231dd9a: f3 41 0f 5e c6              	divss	xmm0, xmm14
14231dd9f: f3 0f 2c c8                 	cvttss2si	ecx, xmm0
14231dda3: 0f 2e 05 12 6c 68 00        	ucomiss	xmm0, dword ptr [rip + 0x686c12] # 0x1429a49bc
14231ddaa: ba ff ff ff 7f              	mov	edx, 0x7fffffff
14231ddaf: 0f 47 ca                    	cmova	ecx, edx
14231ddb2: 0f 2e c0                    	ucomiss	xmm0, xmm0
14231ddb5: ba 00 00 00 00              	mov	edx, 0x0
14231ddba: 0f 4a ca                    	cmovp	ecx, edx
14231ddbd: 83 f9 07                    	cmp	ecx, 0x7
14231ddc0: ba 07 00 00 00              	mov	edx, 0x7
14231ddc5: 0f 42 d1                    	cmovb	edx, ecx
14231ddc8: 85 c9                       	test	ecx, ecx
14231ddca: b9 01 00 00 00              	mov	ecx, 0x1
14231ddcf: 0f 4e d1                    	cmovle	edx, ecx
14231ddd2: 0f 57 c0                    	xorps	xmm0, xmm0
14231ddd5: f3 0f 2a c2                 	cvtsi2ss	xmm0, edx
14231ddd9: 41 0f 28 d1                 	movaps	xmm2, xmm9
14231dddd: f3 0f 5e d0                 	divss	xmm2, xmm0
14231dde1: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
14231dde5: f3 41 0f 58 d3              	addss	xmm2, xmm11
14231ddea: f3 0f 5f 15 9e 28 64 00     	maxss	xmm2, dword ptr [rip + 0x64289e] # 0x142960690
14231ddf2: ff c2                       	inc	edx
14231ddf4: f3 0f 58 d2                 	addss	xmm2, xmm2
14231ddf8: f3 41 0f 5e d1              	divss	xmm2, xmm9
14231ddfd: c6 85 66 01 00 00 01        	mov	byte ptr [rbp + 0x166], 0x1
14231de04: 48 8d 05 e5 d7 d2 00        	lea	rax, [rip + 0xd2d7e5]   # 0x14304b5f0
14231de0b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14231de10: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14231de14: 4c 8d 4d 60                 	lea	r9, [rbp + 0x60]
14231de18: e8 33 25 97 fe              	call	0x140c90350 <.text+0xc8f350>
14231de1d: 48 8b 55 18                 	mov	rdx, qword ptr [rbp + 0x18]
14231de21: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
14231de25: 48 8d 04 82                 	lea	rax, [rdx + 4*rax]
14231de29: 48 89 85 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rax
14231de30: b1 01                       	mov	cl, 0x1
14231de32: 0f 28 45 d0                 	movaps	xmm0, xmmword ptr [rbp - 0x30]
14231de36: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
14231de3a: 48 89 95 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rdx
14231de41: 48 89 d0                    	mov	rax, rdx
14231de44: eb 43                       	jmp	0x14231de89 <.text+0x231ce89>
14231de46: 4c 8b 85 f8 00 00 00        	mov	r8, qword ptr [rbp + 0xf8]
14231de4d: 4b 8d 0c 76                 	lea	rcx, [r14 + 2*r14]
14231de51: 41 c7 44 c8 04 00 00 00 00  	mov	dword ptr [r8 + 8*rcx + 0x4], 0x0
14231de5a: 48 8b 95 b0 00 00 00        	mov	rdx, qword ptr [rbp + 0xb0]
14231de61: 49 89 54 c8 08              	mov	qword ptr [r8 + 8*rcx + 0x8], rdx
14231de66: 8b 95 b8 00 00 00           	mov	edx, dword ptr [rbp + 0xb8]
14231de6c: 41 89 54 c8 10              	mov	dword ptr [r8 + 8*rcx + 0x10], edx
14231de71: 41 89 44 c8 14              	mov	dword ptr [r8 + 8*rcx + 0x14], eax
14231de76: 49 ff c6                    	inc	r14
14231de79: 4c 89 b5 00 01 00 00        	mov	qword ptr [rbp + 0x100], r14
14231de80: 31 c9                       	xor	ecx, ecx
14231de82: 48 8b 85 98 00 00 00        	mov	rax, qword ptr [rbp + 0x98]
14231de89: 48 3b 85 a0 00 00 00        	cmp	rax, qword ptr [rbp + 0xa0]
14231de90: 0f 84 9c 01 00 00           	je	0x14231e032 <.text+0x231d032>
14231de96: 48 8d 50 04                 	lea	rdx, [rax + 0x4]
14231de9a: f6 c1 01                    	test	cl, 0x1
14231de9d: 74 20                       	je	0x14231debf <.text+0x231cebf>
14231de9f: 48 3b 95 a0 00 00 00        	cmp	rdx, qword ptr [rbp + 0xa0]
14231dea6: 0f 84 86 01 00 00           	je	0x14231e032 <.text+0x231d032>
14231deac: 48 8d 48 08                 	lea	rcx, [rax + 0x8]
14231deb0: 48 89 8d 98 00 00 00        	mov	qword ptr [rbp + 0x98], rcx
14231deb7: 48 89 c1                    	mov	rcx, rax
14231deba: 48 89 d0                    	mov	rax, rdx
14231debd: eb 0b                       	jmp	0x14231deca <.text+0x231ceca>
14231debf: 48 89 95 98 00 00 00        	mov	qword ptr [rbp + 0x98], rdx
14231dec6: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
14231deca: f3 44 0f 10 09              	movss	xmm9, dword ptr [rcx]
14231decf: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
14231ded3: f3 44 0f 10 10              	movss	xmm10, dword ptr [rax]
14231ded8: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231dedf: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
14231dee3: e8 98 14 99 fe              	call	0x140caf380 <.text+0xcae380>
14231dee8: 0f 28 ce                    	movaps	xmm1, xmm6
14231deeb: f3 0f 5c c8                 	subss	xmm1, xmm0
14231deef: f3 41 0f 59 ce              	mulss	xmm1, xmm14
14231def4: f3 41 0f 59 c7              	mulss	xmm0, xmm15
14231def9: f3 0f 58 c1                 	addss	xmm0, xmm1
14231defd: 0f 28 ce                    	movaps	xmm1, xmm6
14231df00: f3 0f 5c c8                 	subss	xmm1, xmm0
14231df04: f3 44 0f 59 c9              	mulss	xmm9, xmm1
14231df09: f3 44 0f 59 d0              	mulss	xmm10, xmm0
14231df0e: f3 45 0f 58 d1              	addss	xmm10, xmm9
14231df13: 0f 28 c6                    	movaps	xmm0, xmm6
14231df16: f3 41 0f 5c c2              	subss	xmm0, xmm10
14231df1b: f3 0f 12 c0                 	movsldup	xmm0, xmm0              # xmm0 = xmm0[0,0,2,2]
14231df1f: 41 0f 59 c0                 	mulps	xmm0, xmm8
14231df23: f3 41 0f 12 ca              	movsldup	xmm1, xmm10             # xmm1 = xmm10[0,0,2,2]
14231df28: 0f 59 cf                    	mulps	xmm1, xmm7
14231df2b: 0f 58 c8                    	addps	xmm1, xmm0
14231df2e: 0f 14 c9                    	unpcklps	xmm1, xmm1              # xmm1 = xmm1[0,0,1,1]
14231df31: 0f 59 4d e0                 	mulps	xmm1, xmmword ptr [rbp - 0x20]
14231df35: 0f 28 c1                    	movaps	xmm0, xmm1
14231df38: 66 0f 15 c1                 	unpckhpd	xmm0, xmm1              # xmm0 = xmm0[1],xmm1[1]
14231df3c: 0f 58 c1                    	addps	xmm0, xmm1
14231df3f: f2 0f 10 4d f8              	movsd	xmm1, qword ptr [rbp - 0x8]
14231df44: 0f 58 c8                    	addps	xmm1, xmm0
14231df47: 0f 13 8d 18 01 00 00        	movlps	qword ptr [rbp + 0x118], xmm1
14231df4e: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231df55: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
14231df5c: e8 ff 52 5c fe              	call	0x1408e3260 <.text+0x8e2260>
14231df61: 48 8b 85 48 01 00 00        	mov	rax, qword ptr [rbp + 0x148]
14231df68: f3 0f 58 40 4c              	addss	xmm0, dword ptr [rax + 0x4c]
14231df6d: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231df74: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
14231df7b: 48 8d 95 18 01 00 00        	lea	rdx, [rbp + 0x118]
14231df82: 0f 28 d0                    	movaps	xmm2, xmm0
14231df85: e8 56 ee 97 fe              	call	0x140c9cde0 <.text+0xc9bde0>
14231df8a: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231df91: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
14231df98: 0f 28 ce                    	movaps	xmm1, xmm6
14231df9b: e8 50 69 f1 fe              	call	0x1412348f0 <.text+0x12338f0>
14231dfa0: 8b 85 d8 00 00 00           	mov	eax, dword ptr [rbp + 0xd8]
14231dfa6: 89 85 20 01 00 00           	mov	dword ptr [rbp + 0x120], eax
14231dfac: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
14231dfb3: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
14231dfba: 8b 85 44 01 00 00           	mov	eax, dword ptr [rbp + 0x144]
14231dfc0: 48 8b 4d 70                 	mov	rcx, qword ptr [rbp + 0x70]
14231dfc4: 89 01                       	mov	dword ptr [rcx], eax
14231dfc6: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231dfcd: 48 8b 8d 08 01 00 00        	mov	rcx, qword ptr [rbp + 0x108]
14231dfd4: e8 57 83 c6 fe              	call	0x140f86330 <.text+0xf85330>
14231dfd9: 8b 8d 20 01 00 00           	mov	ecx, dword ptr [rbp + 0x120]
14231dfdf: 89 8d b8 00 00 00           	mov	dword ptr [rbp + 0xb8], ecx
14231dfe5: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
14231dfec: 48 89 8d b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rcx
14231dff3: 4c 8b b5 00 01 00 00        	mov	r14, qword ptr [rbp + 0x100]
14231dffa: 4c 3b b5 f0 00 00 00        	cmp	r14, qword ptr [rbp + 0xf0]
14231e001: 0f 85 3f fe ff ff           	jne	0x14231de46 <.text+0x231ce46>
14231e007: c6 85 67 01 00 00 01        	mov	byte ptr [rbp + 0x167], 0x1
14231e00e: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
14231e015: 48 8d 15 ec d5 d2 00        	lea	rdx, [rip + 0xd2d5ec]   # 0x14304b608
14231e01c: 89 85 ec 00 00 00           	mov	dword ptr [rbp + 0xec], eax
14231e022: e8 89 4d 24 fe              	call	0x140562db0 <.text+0x561db0>
14231e027: 8b 85 ec 00 00 00           	mov	eax, dword ptr [rbp + 0xec]
14231e02d: e9 14 fe ff ff              	jmp	0x14231de46 <.text+0x231ce46>
14231e032: 4c 89 ad 38 01 00 00        	mov	qword ptr [rbp + 0x138], r13
14231e039: 48 8b 45 78                 	mov	rax, qword ptr [rbp + 0x78]
14231e03d: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
14231e044: 48 8b 85 80 00 00 00        	mov	rax, qword ptr [rbp + 0x80]
14231e04b: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
14231e052: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
14231e059: 48 89 85 28 01 00 00        	mov	qword ptr [rbp + 0x128], rax
14231e060: 48 8b 85 88 00 00 00        	mov	rax, qword ptr [rbp + 0x88]
14231e067: 48 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], rax
14231e06e: 48 8b 85 00 01 00 00        	mov	rax, qword ptr [rbp + 0x100]
14231e075: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
14231e07c: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
14231e083: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
14231e08a: c6 85 67 01 00 00 00        	mov	byte ptr [rbp + 0x167], 0x0
14231e091: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
14231e098: 48 8b 95 08 01 00 00        	mov	rdx, qword ptr [rbp + 0x108]
14231e09f: 4c 8d 85 b0 00 00 00        	lea	r8, [rbp + 0xb0]
14231e0a6: e8 95 82 c6 fe              	call	0x140f86340 <.text+0xf85340>
14231e0ab: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
14231e0b2: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
14231e0b9: 0f 10 85 d0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd0]
14231e0c0: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
14231e0c7: c6 85 c8 00 00 00 00        	mov	byte ptr [rbp + 0xc8], 0x0
14231e0ce: c6 85 67 01 00 00 00        	mov	byte ptr [rbp + 0x167], 0x0
14231e0d5: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
14231e0dc: 48 8d 95 b0 00 00 00        	lea	rdx, [rbp + 0xb0]
14231e0e3: e8 88 2e aa ff              	call	0x141dc0f70 <.text+0x1dbff70>
14231e0e8: 48 8b 55 10                 	mov	rdx, qword ptr [rbp + 0x10]
14231e0ec: 48 85 d2                    	test	rdx, rdx
14231e0ef: 74 16                       	je	0x14231e107 <.text+0x231d107>
14231e0f1: 48 c1 e2 02                 	shl	rdx, 0x2
14231e0f5: 41 b8 04 00 00 00           	mov	r8d, 0x4
14231e0fb: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
14231e102: e8 19 5b 2f fe              	call	0x140613c20 <.text+0x612c20>
14231e107: 4c 8b 75 68                 	mov	r14, qword ptr [rbp + 0x68]
14231e10b: e9 d8 f7 ff ff              	jmp	0x14231d8e8 <.text+0x231c8e8>
14231e110: 8b 85 44 01 00 00           	mov	eax, dword ptr [rbp + 0x144]
14231e116: 48 8b 4d 70                 	mov	rcx, qword ptr [rbp + 0x70]
14231e11a: 89 01                       	mov	dword ptr [rcx], eax
14231e11c: 48 8b 8d 08 01 00 00        	mov	rcx, qword ptr [rbp + 0x108]
14231e123: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
14231e127: e8 b4 a5 fd ff              	call	0x1422f86e0 <.text+0x22f76e0>
14231e12c: 0f 28 b5 70 01 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x170]
14231e133: 0f 28 bd 80 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x180]
14231e13a: 44 0f 28 85 90 01 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x190]
14231e142: 44 0f 28 8d a0 01 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x1a0]
14231e14a: 44 0f 28 95 b0 01 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x1b0]
14231e152: 44 0f 28 9d c0 01 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x1c0]
14231e15a: 44 0f 28 a5 d0 01 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x1d0]
14231e162: 44 0f 28 ad e0 01 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x1e0]
14231e16a: 44 0f 28 b5 f0 01 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x1f0]
14231e172: 44 0f 28 bd 00 02 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x200]
14231e17a: 48 81 c4 98 02 00 00        	add	rsp, 0x298
14231e181: 5b                          	pop	rbx
14231e182: 5f                          	pop	rdi
14231e183: 5e                          	pop	rsi
14231e184: 41 5c                       	pop	r12
14231e186: 41 5d                       	pop	r13
14231e188: 41 5e                       	pop	r14
14231e18a: 41 5f                       	pop	r15
14231e18c: 5d                          	pop	rbp
14231e18d: c3                          	ret
14231e18e: 66 90                       	nop
14231e190: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14231e195: 55                          	push	rbp
14231e196: 41 57                       	push	r15
14231e198: 41 56                       	push	r14
14231e19a: 41 55                       	push	r13
14231e19c: 41 54                       	push	r12
14231e19e: 56                          	push	rsi
14231e19f: 57                          	push	rdi
14231e1a0: 53                          	push	rbx
14231e1a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14231e1a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14231e1af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14231e1b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14231e1bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14231e1c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14231e1c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14231e1cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14231e1d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14231e1df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14231e1e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14231e1f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14231e1f8: 0f b6 9d 67 01 00 00        	movzx	ebx, byte ptr [rbp + 0x167]
14231e1ff: 48 8b 55 10                 	mov	rdx, qword ptr [rbp + 0x10]
14231e203: 48 85 d2                    	test	rdx, rdx
14231e206: 74 16                       	je	0x14231e21e <.text+0x231d21e>
14231e208: 48 c1 e2 02                 	shl	rdx, 0x2
14231e20c: 41 b8 04 00 00 00           	mov	r8d, 0x4
14231e212: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
14231e219: e8 02 5a 2f fe              	call	0x140613c20 <.text+0x612c20>
14231e21e: 80 e3 01                    	and	bl, 0x1
14231e221: 88 9d 66 01 00 00           	mov	byte ptr [rbp + 0x166], bl
14231e227: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14231e22f: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14231e237: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14231e240: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14231e249: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14231e252: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14231e258: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14231e25e: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14231e264: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14231e26a: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14231e270: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14231e277: 5b                          	pop	rbx
14231e278: 5f                          	pop	rdi
14231e279: 5e                          	pop	rsi
14231e27a: 41 5c                       	pop	r12
14231e27c: 41 5d                       	pop	r13
14231e27e: 41 5e                       	pop	r14
14231e280: 41 5f                       	pop	r15
14231e282: 5d                          	pop	rbp
14231e283: c3                          	ret
14231e284: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
14231e290: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14231e295: 55                          	push	rbp
14231e296: 41 57                       	push	r15
14231e298: 41 56                       	push	r14
14231e29a: 41 55                       	push	r13
14231e29c: 41 54                       	push	r12
14231e29e: 56                          	push	rsi
14231e29f: 57                          	push	rdi
14231e2a0: 53                          	push	rbx
14231e2a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
14231e2a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14231e2af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
14231e2b5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14231e2bb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
14231e2c1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
14231e2c7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14231e2cd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
14231e2d6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14231e2df: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
14231e2e8: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
14231e2f0: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
14231e2f8: 80 bd 66 01 00 00 00        	cmp	byte ptr [rbp + 0x166], 0x0
14231e2ff: 74 26                       	je	0x14231e327 <.text+0x231d327>
14231e301: 48 8b 85 f0 00 00 00        	mov	rax, qword ptr [rbp + 0xf0]
14231e308: 48 85 c0                    	test	rax, rax
14231e30b: 74 1a                       	je	0x14231e327 <.text+0x231d327>
14231e30d: 48 8b 8d f8 00 00 00        	mov	rcx, qword ptr [rbp + 0xf8]
14231e314: 48 c1 e0 03                 	shl	rax, 0x3
14231e318: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
14231e31c: 41 b8 08 00 00 00           	mov	r8d, 0x8
14231e322: e8 f9 58 2f fe              	call	0x140613c20 <.text+0x612c20>
14231e327: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14231e32f: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14231e337: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14231e340: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14231e349: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14231e352: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14231e358: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14231e35e: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14231e364: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14231e36a: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14231e370: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14231e377: 5b                          	pop	rbx
14231e378: 5f                          	pop	rdi
14231e379: 5e                          	pop	rsi
14231e37a: 41 5c                       	pop	r12
14231e37c: 41 5d                       	pop	r13
14231e37e: 41 5e                       	pop	r14
14231e380: 41 5f                       	pop	r15
14231e382: 5d                          	pop	rbp
14231e383: c3                          	ret
