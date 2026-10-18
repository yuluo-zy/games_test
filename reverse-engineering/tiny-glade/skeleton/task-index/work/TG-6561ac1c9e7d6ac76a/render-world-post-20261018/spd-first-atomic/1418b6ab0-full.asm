
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1418b6ab0: 55                          	push	rbp
1418b6ab1: 41 57                       	push	r15
1418b6ab3: 41 56                       	push	r14
1418b6ab5: 41 55                       	push	r13
1418b6ab7: 41 54                       	push	r12
1418b6ab9: 56                          	push	rsi
1418b6aba: 57                          	push	rdi
1418b6abb: 53                          	push	rbx
1418b6abc: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
1418b6ac3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1418b6acb: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
1418b6ad3: 48 89 d7                    	mov	rdi, rdx
1418b6ad6: 48 89 ce                    	mov	rsi, rcx
1418b6ad9: 44 8b 7a 20                 	mov	r15d, dword ptr [rdx + 0x20]
1418b6add: 44 8b 62 24                 	mov	r12d, dword ptr [rdx + 0x24]
1418b6ae1: 31 c0                       	xor	eax, eax
1418b6ae3: 45 89 e5                    	mov	r13d, r12d
1418b6ae6: 45 29 fd                    	sub	r13d, r15d
1418b6ae9: 44 0f 42 e8                 	cmovb	r13d, eax
1418b6aed: 76 38                       	jbe	0x1418b6b27 <.text+0x18b5b27>
1418b6aef: 4c 89 c3                    	mov	rbx, r8
1418b6af2: 4a 8d 04 ed 00 00 00 00     	lea	rax, [8*r13]
1418b6afa: 4c 8d 34 40                 	lea	r14, [rax + 2*rax]
1418b6afe: 0f b6 05 04 e1 d7 01        	movzx	eax, byte ptr [rip + 0x1d7e104] # 0x143634c09
1418b6b05: ba 08 00 00 00              	mov	edx, 0x8
1418b6b0a: 4c 89 f1                    	mov	rcx, r14
1418b6b0d: e8 fe d0 d5 fe              	call	0x140613c10 <.text+0x612c10>
1418b6b12: 48 85 c0                    	test	rax, rax
1418b6b15: 75 18                       	jne	0x1418b6b2f <.text+0x18b5b2f>
1418b6b17: b9 08 00 00 00              	mov	ecx, 0x8
1418b6b1c: 4c 89 f2                    	mov	rdx, r14
1418b6b1f: 49 89 d8                    	mov	r8, rbx
1418b6b22: e8 bc 24 02 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1418b6b27: b8 08 00 00 00              	mov	eax, 0x8
1418b6b2c: 45 31 ed                    	xor	r13d, r13d
1418b6b2f: 4c 89 6d e8                 	mov	qword ptr [rbp - 0x18], r13
1418b6b33: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1418b6b37: 48 8d 4d f8                 	lea	rcx, [rbp - 0x8]
1418b6b3b: 48 c7 45 f8 00 00 00 00     	mov	qword ptr [rbp - 0x8], 0x0
1418b6b43: 0f 10 07                    	movups	xmm0, xmmword ptr [rdi]
1418b6b46: 0f 10 4f 10                 	movups	xmm1, xmmword ptr [rdi + 0x10]
1418b6b4a: 0f 29 4d b0                 	movaps	xmmword ptr [rbp - 0x50], xmm1
1418b6b4e: 0f 29 45 a0                 	movaps	xmmword ptr [rbp - 0x60], xmm0
1418b6b52: 44 89 7d c0                 	mov	dword ptr [rbp - 0x40], r15d
1418b6b56: 44 89 65 c4                 	mov	dword ptr [rbp - 0x3c], r12d
1418b6b5a: 48 89 4d d0                 	mov	qword ptr [rbp - 0x30], rcx
1418b6b5e: 48 c7 45 d8 00 00 00 00     	mov	qword ptr [rbp - 0x28], 0x0
1418b6b66: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
1418b6b6a: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
1418b6b6e: 48 8d 55 d0                 	lea	rdx, [rbp - 0x30]
1418b6b72: e8 59 db ff ff              	call	0x1418b46d0 <.text+0x18b36d0>
1418b6b77: 48 8b 45 f8                 	mov	rax, qword ptr [rbp - 0x8]
1418b6b7b: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
1418b6b7f: 0f 10 45 e8                 	movups	xmm0, xmmword ptr [rbp - 0x18]
1418b6b83: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
1418b6b86: 48 89 f0                    	mov	rax, rsi
1418b6b89: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1418b6b90: 5b                          	pop	rbx
1418b6b91: 5f                          	pop	rdi
1418b6b92: 5e                          	pop	rsi
1418b6b93: 41 5c                       	pop	r12
1418b6b95: 41 5d                       	pop	r13
1418b6b97: 41 5e                       	pop	r14
1418b6b99: 41 5f                       	pop	r15
1418b6b9b: 5d                          	pop	rbp
1418b6b9c: c3                          	ret
1418b6b9d: 0f 1f 00                    	nop	dword ptr [rax]
1418b6ba0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1418b6ba5: 55                          	push	rbp
1418b6ba6: 41 57                       	push	r15
1418b6ba8: 41 56                       	push	r14
1418b6baa: 41 55                       	push	r13
1418b6bac: 41 54                       	push	r12
1418b6bae: 56                          	push	rsi
1418b6baf: 57                          	push	rdi
1418b6bb0: 53                          	push	rbx
1418b6bb1: 48 83 ec 28                 	sub	rsp, 0x28
1418b6bb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1418b6bbc: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1418b6bc0: 48 85 c0                    	test	rax, rax
1418b6bc3: 74 17                       	je	0x1418b6bdc <.text+0x18b5bdc>
1418b6bc5: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
1418b6bc9: 48 c1 e0 03                 	shl	rax, 0x3
1418b6bcd: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1418b6bd1: 41 b8 08 00 00 00           	mov	r8d, 0x8
1418b6bd7: e8 44 d0 d5 fe              	call	0x140613c20 <.text+0x612c20>
1418b6bdc: 90                          	nop
1418b6bdd: 48 83 c4 28                 	add	rsp, 0x28
1418b6be1: 5b                          	pop	rbx
1418b6be2: 5f                          	pop	rdi
1418b6be3: 5e                          	pop	rsi
1418b6be4: 41 5c                       	pop	r12
1418b6be6: 41 5d                       	pop	r13
1418b6be8: 41 5e                       	pop	r14
1418b6bea: 41 5f                       	pop	r15
1418b6bec: 5d                          	pop	rbp
1418b6bed: c3                          	ret
