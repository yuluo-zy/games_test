
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2b7a0: 55                          	push	rbp
140b2b7a1: 41 57                       	push	r15
140b2b7a3: 41 56                       	push	r14
140b2b7a5: 41 55                       	push	r13
140b2b7a7: 41 54                       	push	r12
140b2b7a9: 56                          	push	rsi
140b2b7aa: 57                          	push	rdi
140b2b7ab: 53                          	push	rbx
140b2b7ac: 48 83 ec 78                 	sub	rsp, 0x78
140b2b7b0: 48 8d 6c 24 70              	lea	rbp, [rsp + 0x70]
140b2b7b5: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
140b2b7bd: 48 89 4d c8                 	mov	qword ptr [rbp - 0x38], rcx
140b2b7c1: 48 c7 45 d0 00 00 00 00     	mov	qword ptr [rbp - 0x30], 0x0
140b2b7c9: 48 c7 45 d8 01 00 00 00     	mov	qword ptr [rbp - 0x28], 0x1
140b2b7d1: 0f 57 c0                    	xorps	xmm0, xmm0
140b2b7d4: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
140b2b7d8: 48 c7 45 f0 04 00 00 00     	mov	qword ptr [rbp - 0x10], 0x4
140b2b7e0: 48 c7 45 f8 00 00 00 00     	mov	qword ptr [rbp - 0x8], 0x0
140b2b7e8: 4c 8b 22                    	mov	r12, qword ptr [rdx]
140b2b7eb: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
140b2b7ef: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
140b2b7f3: 0f b6 7a 10                 	movzx	edi, byte ptr [rdx + 0x10]
140b2b7f7: 45 31 f6                    	xor	r14d, r14d
140b2b7fa: 4c 8d 2d eb 01 fc 01        	lea	r13, [rip + 0x1fc01eb]  # 0x142aeb9ec
140b2b801: eb 0d                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b803: 40 b7 0d                    	mov	dil, 0xd
140b2b806: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140b2b810: 40 0f b6 f7                 	movzx	esi, dil
140b2b814: 49 63 44 b5 00              	movsxd	rax, dword ptr [r13 + 4*rsi]
140b2b819: 4c 01 e8                    	add	rax, r13
140b2b81c: 40 b7 09                    	mov	dil, 0x9
140b2b81f: ff e0                       	jmp	rax
140b2b821: 40 b7 01                    	mov	dil, 0x1
140b2b824: eb 7a                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b826: 40 b7 12                    	mov	dil, 0x12
140b2b829: eb 75                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b82b: 40 b7 10                    	mov	dil, 0x10
140b2b82e: eb 70                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b830: 40 b7 05                    	mov	dil, 0x5
140b2b833: eb 6b                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b835: 40 b7 0e                    	mov	dil, 0xe
140b2b838: eb d6                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b83a: 40 b7 0b                    	mov	dil, 0xb
140b2b83d: eb 61                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b83f: 40 b7 1b                    	mov	dil, 0x1b
140b2b842: eb cc                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b844: 40 b7 1a                    	mov	dil, 0x1a
140b2b847: eb 57                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b849: 40 b7 03                    	mov	dil, 0x3
140b2b84c: eb 52                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b84e: 40 b7 11                    	mov	dil, 0x11
140b2b851: eb 4d                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b853: 40 b7 04                    	mov	dil, 0x4
140b2b856: eb 48                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b858: 40 b7 08                    	mov	dil, 0x8
140b2b85b: eb 43                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b85d: 40 b7 02                    	mov	dil, 0x2
140b2b860: eb 3e                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b862: 40 b7 13                    	mov	dil, 0x13
140b2b865: eb 39                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b867: 40 b7 16                    	mov	dil, 0x16
140b2b86a: eb a4                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b86c: 40 b7 06                    	mov	dil, 0x6
140b2b86f: eb 2f                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b871: 40 b7 17                    	mov	dil, 0x17
140b2b874: eb 2a                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b876: 40 b7 0f                    	mov	dil, 0xf
140b2b879: eb 25                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b87b: 40 b7 07                    	mov	dil, 0x7
140b2b87e: eb 20                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b880: 40 b7 14                    	mov	dil, 0x14
140b2b883: eb 8b                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b885: 40 b7 0c                    	mov	dil, 0xc
140b2b888: eb 86                       	jmp	0x140b2b810 <.text+0xb2a810>
140b2b88a: 40 b7 0a                    	mov	dil, 0xa
140b2b88d: eb 11                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b88f: 40 b7 18                    	mov	dil, 0x18
140b2b892: eb 0c                       	jmp	0x140b2b8a0 <.text+0xb2a8a0>
140b2b894: 40 b7 15                    	mov	dil, 0x15
140b2b897: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140b2b8a0: 48 8d 04 76                 	lea	rax, [rsi + 2*rsi]
140b2b8a4: c1 e0 06                    	shl	eax, 0x6
140b2b8a7: 48 b9 01 00 00 00 00 00 00 80       	movabs	rcx, -0x7fffffffffffffff
140b2b8b1: 49 39 4c 04 08              	cmp	qword ptr [r12 + rax + 0x8], rcx
140b2b8b6: 0f 8e 1c 01 00 00           	jle	0x140b2b9d8 <.text+0xb2a9d8>
140b2b8bc: 49 8d 0c 04                 	lea	rcx, [r12 + rax]
140b2b8c0: 48 83 c1 08                 	add	rcx, 0x8
140b2b8c4: 8b 99 a0 00 00 00           	mov	ebx, dword ptr [rcx + 0xa0]
140b2b8ca: 85 db                       	test	ebx, ebx
140b2b8cc: 0f 84 3e ff ff ff           	je	0x140b2b810 <.text+0xb2a810>
140b2b8d2: 48 83 c1 68                 	add	rcx, 0x68
140b2b8d6: 48 8b 55 c0                 	mov	rdx, qword ptr [rbp - 0x40]
140b2b8da: e8 21 93 0a 00              	call	0x140bd4c00 <.text+0xbd3c00>
140b2b8df: 49 89 c7                    	mov	r15, rax
140b2b8e2: 48 85 c0                    	test	rax, rax
140b2b8e5: 0f 84 d3 00 00 00           	je	0x140b2b9be <.text+0xb2a9be>
140b2b8eb: 48 8b 55 e0                 	mov	rdx, qword ptr [rbp - 0x20]
140b2b8ef: 48 39 55 d0                 	cmp	qword ptr [rbp - 0x30], rdx
140b2b8f3: 74 4e                       	je	0x140b2b943 <.text+0xb2a943>
140b2b8f5: 8d 04 b5 00 00 00 00        	lea	eax, [4*rsi]
140b2b8fc: 0f b6 c0                    	movzx	eax, al
140b2b8ff: 48 8b 4d d8                 	mov	rcx, qword ptr [rbp - 0x28]
140b2b903: 4c 8d 04 92                 	lea	r8, [rdx + 4*rdx]
140b2b907: 4e 89 3c 81                 	mov	qword ptr [rcx + 4*r8], r15
140b2b90b: 42 89 74 81 08              	mov	dword ptr [rcx + 4*r8 + 0x8], esi
140b2b910: 42 89 5c 81 0c              	mov	dword ptr [rcx + 4*r8 + 0xc], ebx
140b2b915: 42 89 44 81 10              	mov	dword ptr [rcx + 4*r8 + 0x10], eax
140b2b91a: 48 ff c2                    	inc	rdx
140b2b91d: 48 89 55 e0                 	mov	qword ptr [rbp - 0x20], rdx
140b2b921: 4c 39 75 e8                 	cmp	qword ptr [rbp - 0x18], r14
140b2b925: 74 44                       	je	0x140b2b96b <.text+0xb2a96b>
140b2b927: 48 8b 45 f0                 	mov	rax, qword ptr [rbp - 0x10]
140b2b92b: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
140b2b92f: 49 ff c6                    	inc	r14
140b2b932: 4c 89 75 f8                 	mov	qword ptr [rbp - 0x8], r14
140b2b936: e9 d5 fe ff ff              	jmp	0x140b2b810 <.text+0xb2a810>
140b2b93b: 40 b7 19                    	mov	dil, 0x19
140b2b93e: e9 cd fe ff ff              	jmp	0x140b2b810 <.text+0xb2a810>
140b2b943: 48 c7 44 24 20 14 00 00 00  	mov	qword ptr [rsp + 0x20], 0x14
140b2b94c: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
140b2b950: 41 b8 01 00 00 00           	mov	r8d, 0x1
140b2b956: 41 b9 01 00 00 00           	mov	r9d, 0x1
140b2b95c: e8 ff e0 d7 01              	call	0x1428a9a60 <.text+0x28a8a60>
140b2b961: 48 8b 55 e0                 	mov	rdx, qword ptr [rbp - 0x20]
140b2b965: 4c 8b 75 f8                 	mov	r14, qword ptr [rbp - 0x8]
140b2b969: eb 8a                       	jmp	0x140b2b8f5 <.text+0xb2a8f5>
140b2b96b: 48 c7 44 24 20 04 00 00 00  	mov	qword ptr [rsp + 0x20], 0x4
140b2b974: 41 b8 01 00 00 00           	mov	r8d, 0x1
140b2b97a: 41 b9 04 00 00 00           	mov	r9d, 0x4
140b2b980: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
140b2b984: 4c 89 f2                    	mov	rdx, r14
140b2b987: e8 d4 e0 d7 01              	call	0x1428a9a60 <.text+0x28a8a60>
140b2b98c: 4c 8b 75 f8                 	mov	r14, qword ptr [rbp - 0x8]
140b2b990: eb 95                       	jmp	0x140b2b927 <.text+0xb2a927>
140b2b992: 0f 10 45 d0                 	movups	xmm0, xmmword ptr [rbp - 0x30]
140b2b996: 0f 10 4d e0                 	movups	xmm1, xmmword ptr [rbp - 0x20]
140b2b99a: 0f 10 55 f0                 	movups	xmm2, xmmword ptr [rbp - 0x10]
140b2b99e: 48 8b 45 c8                 	mov	rax, qword ptr [rbp - 0x38]
140b2b9a2: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
140b2b9a6: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
140b2b9aa: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
140b2b9ad: 48 83 c4 78                 	add	rsp, 0x78
140b2b9b1: 5b                          	pop	rbx
140b2b9b2: 5f                          	pop	rdi
140b2b9b3: 5e                          	pop	rsi
140b2b9b4: 41 5c                       	pop	r12
140b2b9b6: 41 5d                       	pop	r13
140b2b9b8: 41 5e                       	pop	r14
140b2b9ba: 41 5f                       	pop	r15
140b2b9bc: 5d                          	pop	rbp
140b2b9bd: c3                          	ret
140b2b9be: 48 8d 0d 7b fd fb 01        	lea	rcx, [rip + 0x1fbfd7b]  # 0x142aeb740
140b2b9c5: 4c 8d 05 f4 fd fb 01        	lea	r8, [rip + 0x1fbfdf4]   # 0x142aeb7c0
140b2b9cc: ba 2b 00 00 00              	mov	edx, 0x2b
140b2b9d1: e8 5a da da 01              	call	0x1428d9430 <.text+0x28d8430>
140b2b9d6: eb 18                       	jmp	0x140b2b9f0 <.text+0xb2a9f0>
140b2b9d8: 48 8d 0d f9 fd fb 01        	lea	rcx, [rip + 0x1fbfdf9]  # 0x142aeb7d8
140b2b9df: 4c 8d 05 1a fe fb 01        	lea	r8, [rip + 0x1fbfe1a]   # 0x142aeb800
140b2b9e6: ba 28 00 00 00              	mov	edx, 0x28
140b2b9eb: e8 40 da da 01              	call	0x1428d9430 <.text+0x28d8430>
140b2b9f0: 0f 0b                       	ud2
140b2b9f2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140b2ba00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2ba05: 55                          	push	rbp
140b2ba06: 41 57                       	push	r15
140b2ba08: 41 56                       	push	r14
140b2ba0a: 41 55                       	push	r13
140b2ba0c: 41 54                       	push	r12
140b2ba0e: 56                          	push	rsi
140b2ba0f: 57                          	push	rdi
140b2ba10: 53                          	push	rbx
140b2ba11: 48 83 ec 28                 	sub	rsp, 0x28
140b2ba15: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
140b2ba19: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
140b2ba1d: e8 fe f3 ff ff              	call	0x140b2ae20 <.text+0xb29e20>
140b2ba22: 90                          	nop
140b2ba23: 48 83 c4 28                 	add	rsp, 0x28
140b2ba27: 5b                          	pop	rbx
140b2ba28: 5f                          	pop	rdi
140b2ba29: 5e                          	pop	rsi
140b2ba2a: 41 5c                       	pop	r12
140b2ba2c: 41 5d                       	pop	r13
140b2ba2e: 41 5e                       	pop	r14
140b2ba30: 41 5f                       	pop	r15
140b2ba32: 5d                          	pop	rbp
140b2ba33: c3                          	ret
