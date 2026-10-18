
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a398b0: 55                          	push	rbp
140a398b1: 41 57                       	push	r15
140a398b3: 41 56                       	push	r14
140a398b5: 41 55                       	push	r13
140a398b7: 41 54                       	push	r12
140a398b9: 56                          	push	rsi
140a398ba: 57                          	push	rdi
140a398bb: 53                          	push	rbx
140a398bc: 48 81 ec 88 01 00 00        	sub	rsp, 0x188
140a398c3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a398cb: 66 0f 7f b5 f0 00 00 00     	movdqa	xmmword ptr [rbp + 0xf0], xmm6
140a398d3: 48 c7 85 e8 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xe8], -0x2
140a398de: 4d 89 ce                    	mov	r14, r9
140a398e1: 4c 89 85 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], r8
140a398e8: 48 89 95 d8 00 00 00        	mov	qword ptr [rbp + 0xd8], rdx
140a398ef: 48 89 8d d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rcx
140a398f6: 31 c9                       	xor	ecx, ecx
140a398f8: e8 23 02 6a ff              	call	0x1400d9b20 <.text+0xd8b20>
140a398fd: 48 85 c0                    	test	rax, rax
140a39900: 0f 84 44 04 00 00           	je	0x140a39d4a <.text+0xa38d4a>
140a39906: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
140a39909: 48 ff 00                    	inc	qword ptr [rax]
140a3990c: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140a39913: 0f 10 05 e6 25 23 02        	movups	xmm0, xmmword ptr [rip + 0x22325e6] # 0x142c6bf00
140a3991a: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140a3991e: f3 0f 6f 05 ea 25 23 02     	movdqu	xmm0, xmmword ptr [rip + 0x22325ea] # 0x142c6bf10
140a39926: 66 0f 7f 45 70              	movdqa	xmmword ptr [rbp + 0x70], xmm0
140a3992b: 49 8b 1e                    	mov	rbx, qword ptr [r14]
140a3992e: 49 8b 4e 08                 	mov	rcx, qword ptr [r14 + 0x8]
140a39932: 4c 8b 03                    	mov	r8, qword ptr [rbx]
140a39935: 31 d2                       	xor	edx, edx
140a39937: 4c 89 c0                    	mov	rax, r8
140a3993a: 48 2b 41 18                 	sub	rax, qword ptr [rcx + 0x18]
140a3993e: 48 0f 42 c2                 	cmovb	rax, rdx
140a39942: 4c 2b 41 38                 	sub	r8, qword ptr [rcx + 0x38]
140a39946: 4c 0f 42 c2                 	cmovb	r8, rdx
140a3994a: 4c 6b f0 38                 	imul	r14, rax, 0x38
140a3994e: 4c 03 71 08                 	add	r14, qword ptr [rcx + 0x8]
140a39952: 4c 8b 49 10                 	mov	r9, qword ptr [rcx + 0x10]
140a39956: 49 29 c1                    	sub	r9, rax
140a39959: 4c 0f 42 ca                 	cmovb	r9, rdx
140a3995d: 4c 8b 51 30                 	mov	r10, qword ptr [rcx + 0x30]
140a39961: 41 bb 08 00 00 00           	mov	r11d, 0x8
140a39967: 4d 0f 42 f3                 	cmovb	r14, r11
140a3996b: 49 6b c0 38                 	imul	rax, r8, 0x38
140a3996f: 48 03 41 28                 	add	rax, qword ptr [rcx + 0x28]
140a39973: 4d 29 c2                    	sub	r10, r8
140a39976: 4c 0f 42 d2                 	cmovb	r10, rdx
140a3997a: 49 0f 42 c3                 	cmovb	rax, r11
140a3997e: 48 8b 49 40                 	mov	rcx, qword ptr [rcx + 0x40]
140a39982: 4c 29 d1                    	sub	rcx, r10
140a39985: 4c 29 c9                    	sub	rcx, r9
140a39988: 48 89 0b                    	mov	qword ptr [rbx], rcx
140a3998b: 49 6b f1 38                 	imul	rsi, r9, 0x38
140a3998f: 4c 01 f6                    	add	rsi, r14
140a39992: 49 6b ca 38                 	imul	rcx, r10, 0x38
140a39996: 48 01 c1                    	add	rcx, rax
140a39999: 48 89 8d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rcx
140a399a0: e9 7d 00 00 00              	jmp	0x140a39a22 <.text+0xa38a22>
140a399a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140a399b0: 4d 8b 3e                    	mov	r15, qword ptr [r14]
140a399b3: 49 8b 7e 08                 	mov	rdi, qword ptr [r14 + 0x8]
140a399b7: 66 41 0f 6f 37              	movdqa	xmm6, xmmword ptr [r15]
140a399bc: 49 8b 56 18                 	mov	rdx, qword ptr [r14 + 0x18]
140a399c0: 48 8d 42 01                 	lea	rax, [rdx + 0x1]
140a399c4: 48 d1 e8                    	shr	rax
140a399c7: 48 39 45 70                 	cmp	qword ptr [rbp + 0x70], rax
140a399cb: 0f 82 c8 00 00 00           	jb	0x140a39a99 <.text+0xa38a99>
140a399d1: 49 8d 04 3f                 	lea	rax, [r15 + rdi]
140a399d5: 48 ff c0                    	inc	rax
140a399d8: 66 0f d7 ce                 	pmovmskb	ecx, xmm6
140a399dc: f7 d1                       	not	ecx
140a399de: 4c 8d 45 60                 	lea	r8, [rbp + 0x60]
140a399e2: 4c 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], r8
140a399e9: 4c 8d 85 90 00 00 00        	lea	r8, [rbp + 0x90]
140a399f0: 4c 89 45 c0                 	mov	qword ptr [rbp - 0x40], r8
140a399f4: 4c 89 7d 10                 	mov	qword ptr [rbp + 0x10], r15
140a399f8: 49 83 c7 10                 	add	r15, 0x10
140a399fc: 4c 89 7d 18                 	mov	qword ptr [rbp + 0x18], r15
140a39a00: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140a39a04: 66 89 4d 28                 	mov	word ptr [rbp + 0x28], cx
140a39a08: c6 85 e7 00 00 00 01        	mov	byte ptr [rbp + 0xe7], 0x1
140a39a0f: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
140a39a13: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
140a39a17: e8 a4 c7 13 00              	call	0x140b761c0 <.text+0xb751c0>
140a39a1c: 4d 89 ee                    	mov	r14, r13
140a39a1f: 4c 89 e0                    	mov	rax, r12
140a39a22: 4d 85 f6                    	test	r14, r14
140a39a25: 74 29                       	je	0x140a39a50 <.text+0xa38a50>
140a39a27: 49 39 f6                    	cmp	r14, rsi
140a39a2a: 74 24                       	je	0x140a39a50 <.text+0xa38a50>
140a39a2c: 4d 8d 6e 38                 	lea	r13, [r14 + 0x38]
140a39a30: 49 89 c4                    	mov	r12, rax
140a39a33: 48 ff 03                    	inc	qword ptr [rbx]
140a39a36: 48 83 7d 78 00              	cmp	qword ptr [rbp + 0x78], 0x0
140a39a3b: 0f 85 6f ff ff ff           	jne	0x140a399b0 <.text+0xa389b0>
140a39a41: eb 33                       	jmp	0x140a39a76 <.text+0xa38a76>
140a39a43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a39a50: 48 85 c0                    	test	rax, rax
140a39a53: 74 6c                       	je	0x140a39ac1 <.text+0xa38ac1>
140a39a55: 48 3b 85 b8 00 00 00        	cmp	rax, qword ptr [rbp + 0xb8]
140a39a5c: 74 63                       	je	0x140a39ac1 <.text+0xa38ac1>
140a39a5e: 4c 8d 60 38                 	lea	r12, [rax + 0x38]
140a39a62: 45 31 ed                    	xor	r13d, r13d
140a39a65: 49 89 c6                    	mov	r14, rax
140a39a68: 48 ff 03                    	inc	qword ptr [rbx]
140a39a6b: 48 83 7d 78 00              	cmp	qword ptr [rbp + 0x78], 0x0
140a39a70: 0f 85 3a ff ff ff           	jne	0x140a399b0 <.text+0xa389b0>
140a39a76: c6 85 e7 00 00 00 01        	mov	byte ptr [rbp + 0xe7], 0x1
140a39a7d: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
140a39a81: 4c 89 f2                    	mov	rdx, r14
140a39a84: e8 67 8a 13 00              	call	0x140b724f0 <.text+0xb714f0>
140a39a89: f3 41 0f 6f 46 20           	movdqu	xmm0, xmmword ptr [r14 + 0x20]
140a39a8f: 66 0f 7f 85 80 00 00 00     	movdqa	xmmword ptr [rbp + 0x80], xmm0
140a39a97: eb 83                       	jmp	0x140a39a1c <.text+0xa38a1c>
140a39a99: c6 85 e7 00 00 00 01        	mov	byte ptr [rbp + 0xe7], 0x1
140a39aa0: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
140a39aa4: 49 89 d6                    	mov	r14, rdx
140a39aa7: 48 89 c2                    	mov	rdx, rax
140a39aaa: 4c 8d 85 80 00 00 00        	lea	r8, [rbp + 0x80]
140a39ab1: 41 b1 01                    	mov	r9b, 0x1
140a39ab4: e8 f7 6d e8 01              	call	0x1428c08b0 <.text+0x28bf8b0>
140a39ab9: 4c 89 f2                    	mov	rdx, r14
140a39abc: e9 10 ff ff ff              	jmp	0x140a399d1 <.text+0xa389d1>
140a39ac1: 4c 8b 65 60                 	mov	r12, qword ptr [rbp + 0x60]
140a39ac5: 48 8b 4d 68                 	mov	rcx, qword ptr [rbp + 0x68]
140a39ac9: 66 41 0f 6f 04 24           	movdqa	xmm0, xmmword ptr [r12]
140a39acf: 48 85 c9                    	test	rcx, rcx
140a39ad2: 48 89 8d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rcx
140a39ad9: 74 34                       	je	0x140a39b0f <.text+0xa38b0f>
140a39adb: 48 89 c8                    	mov	rax, rcx
140a39ade: 48 83 e0 f0                 	and	rax, -0x10
140a39ae2: 48 01 c1                    	add	rcx, rax
140a39ae5: 48 83 c1 21                 	add	rcx, 0x21
140a39ae9: 48 89 8d a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rcx
140a39af0: 4c 89 e1                    	mov	rcx, r12
140a39af3: 48 29 c1                    	sub	rcx, rax
140a39af6: 48 83 c1 f0                 	add	rcx, -0x10
140a39afa: 48 89 8d b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rcx
140a39b01: b8 10 00 00 00              	mov	eax, 0x10
140a39b06: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
140a39b0d: eb 0b                       	jmp	0x140a39b1a <.text+0xa38b1a>
140a39b0f: 48 c7 85 c0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xc0], 0x0
140a39b1a: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
140a39b21: 48 8b 95 d0 00 00 00        	mov	rdx, qword ptr [rbp + 0xd0]
140a39b28: 4c 8b 85 d8 00 00 00        	mov	r8, qword ptr [rbp + 0xd8]
140a39b2f: 4c 8b 6d 78                 	mov	r13, qword ptr [rbp + 0x78]
140a39b33: 4d 8d 7c 24 10              	lea	r15, [r12 + 0x10]
140a39b38: 66 0f d7 c0                 	pmovmskb	eax, xmm0
140a39b3c: f7 d0                       	not	eax
140a39b3e: 49 8b 18                    	mov	rbx, qword ptr [r8]
140a39b41: 44 8b 41 1c                 	mov	r8d, dword ptr [rcx + 0x1c]
140a39b45: 44 89 85 b8 00 00 00        	mov	dword ptr [rbp + 0xb8], r8d
140a39b4c: 4c 8b 01                    	mov	r8, qword ptr [rcx]
140a39b4f: 48 8b 71 10                 	mov	rsi, qword ptr [rcx + 0x10]
140a39b53: 48 8b 0a                    	mov	rcx, qword ptr [rdx]
140a39b56: 48 89 8d d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rcx
140a39b5d: 4c 89 85 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], r8
140a39b64: 49 8d 88 58 14 00 00        	lea	rcx, [r8 + 0x1458]
140a39b6b: 48 89 8d d8 00 00 00        	mov	qword ptr [rbp + 0xd8], rcx
140a39b72: eb 0e                       	jmp	0x140a39b82 <.text+0xa38b82>
140a39b74: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a39b80: 89 f8                       	mov	eax, edi
140a39b82: 49 83 ed 01                 	sub	r13, 0x1
140a39b86: 0f 82 17 01 00 00           	jb	0x140a39ca3 <.text+0xa38ca3>
140a39b8c: 66 85 c0                    	test	ax, ax
140a39b8f: 74 0f                       	je	0x140a39ba0 <.text+0xa38ba0>
140a39b91: 8d 78 ff                    	lea	edi, [rax - 0x1]
140a39b94: 21 c7                       	and	edi, eax
140a39b96: eb 2d                       	jmp	0x140a39bc5 <.text+0xa38bc5>
140a39b98: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a39ba0: 66 41 0f 6f 07              	movdqa	xmm0, xmmword ptr [r15]
140a39ba5: 66 0f d7 c0                 	pmovmskb	eax, xmm0
140a39ba9: 49 83 c4 f0                 	add	r12, -0x10
140a39bad: 49 83 c7 10                 	add	r15, 0x10
140a39bb1: 3d ff ff 00 00              	cmp	eax, 0xffff
140a39bb6: 74 e8                       	je	0x140a39ba0 <.text+0xa38ba0>
140a39bb8: b9 fe ff ff ff              	mov	ecx, 0xfffffffe
140a39bbd: 29 c1                       	sub	ecx, eax
140a39bbf: f7 d0                       	not	eax
140a39bc1: 89 c7                       	mov	edi, eax
140a39bc3: 21 cf                       	and	edi, ecx
140a39bc5: f3 0f bc c0                 	tzcnt	eax, eax
140a39bc9: 48 f7 d0                    	not	rax
140a39bcc: 46 8a 34 20                 	mov	r14b, byte ptr [rax + r12]
140a39bd0: 48 89 d9                    	mov	rcx, rbx
140a39bd3: 44 89 f2                    	mov	edx, r14d
140a39bd6: e8 75 cb 0a 00              	call	0x140ae6750 <.text+0xae5750>
140a39bdb: 8b 8d b8 00 00 00           	mov	ecx, dword ptr [rbp + 0xb8]
140a39be1: 89 0e                       	mov	dword ptr [rsi], ecx
140a39be3: 48 8b 88 98 00 00 00        	mov	rcx, qword ptr [rax + 0x98]
140a39bea: 48 85 c9                    	test	rcx, rcx
140a39bed: 74 91                       	je	0x140a39b80 <.text+0xa38b80>
140a39bef: 45 0f b6 c6                 	movzx	r8d, r14b
140a39bf3: 41 c1 e0 0a                 	shl	r8d, 0xa
140a39bf7: 48 c1 e1 04                 	shl	rcx, 0x4
140a39bfb: 4a 8d 14 01                 	lea	rdx, [rcx + r8]
140a39bff: 4c 8b 8d c8 00 00 00        	mov	r9, qword ptr [rbp + 0xc8]
140a39c06: 49 3b 91 70 14 00 00        	cmp	rdx, qword ptr [r9 + 0x1470]
140a39c0d: 0f 87 da 00 00 00           	ja	0x140a39ced <.text+0xa38ced>
140a39c13: 48 8b 95 d0 00 00 00        	mov	rdx, qword ptr [rbp + 0xd0]
140a39c1a: 48 8b 12                    	mov	rdx, qword ptr [rdx]
140a39c1d: 48 83 c2 10                 	add	rdx, 0x10
140a39c21: 4c 8b 88 90 00 00 00        	mov	r9, qword ptr [rax + 0x90]
140a39c28: 48 c7 85 90 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x90], 0x0
140a39c33: 48 89 8d 98 00 00 00        	mov	qword ptr [rbp + 0x98], rcx
140a39c3a: 4c 89 44 24 38              	mov	qword ptr [rsp + 0x38], r8
140a39c3f: 48 8d 85 90 00 00 00        	lea	rax, [rbp + 0x90]
140a39c46: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a39c4b: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
140a39c50: 48 c7 44 24 30 01 00 00 00  	mov	qword ptr [rsp + 0x30], 0x1
140a39c59: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
140a39c5d: 4c 8b 85 d8 00 00 00        	mov	r8, qword ptr [rbp + 0xd8]
140a39c64: e8 37 53 1b 00              	call	0x140beefa0 <.text+0xbedfa0>
140a39c69: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140a39c6d: 48 83 f8 0b                 	cmp	rax, 0xb
140a39c71: 0f 84 09 ff ff ff           	je	0x140a39b80 <.text+0xa38b80>
140a39c77: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
140a39c7b: 48 89 4d 00                 	mov	qword ptr [rbp], rcx
140a39c7f: f3 0f 6f 45 18              	movdqu	xmm0, xmmword ptr [rbp + 0x18]
140a39c84: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
140a39c88: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
140a39c8c: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
140a39c90: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
140a39c94: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
140a39c98: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
140a39c9c: 66 0f 7f 45 c0              	movdqa	xmmword ptr [rbp - 0x40], xmm0
140a39ca1: eb 4f                       	jmp	0x140a39cf2 <.text+0xa38cf2>
140a39ca3: 48 83 bd a0 00 00 00 00     	cmp	qword ptr [rbp + 0xa0], 0x0
140a39cab: 0f 94 c0                    	sete	al
140a39cae: 48 8b 95 a8 00 00 00        	mov	rdx, qword ptr [rbp + 0xa8]
140a39cb5: 48 85 d2                    	test	rdx, rdx
140a39cb8: 0f 94 c1                    	sete	cl
140a39cbb: 08 c1                       	or	cl, al
140a39cbd: 75 13                       	jne	0x140a39cd2 <.text+0xa38cd2>
140a39cbf: 48 8b 8d b0 00 00 00        	mov	rcx, qword ptr [rbp + 0xb0]
140a39cc6: 4c 8b 85 c0 00 00 00        	mov	r8, qword ptr [rbp + 0xc0]
140a39ccd: e8 4e 9f bd ff              	call	0x140613c20 <.text+0x612c20>
140a39cd2: 0f 28 b5 f0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xf0]
140a39cd9: 48 81 c4 88 01 00 00        	add	rsp, 0x188
140a39ce0: 5b                          	pop	rbx
140a39ce1: 5f                          	pop	rdi
140a39ce2: 5e                          	pop	rsi
140a39ce3: 41 5c                       	pop	r12
140a39ce5: 41 5d                       	pop	r13
140a39ce7: 41 5e                       	pop	r14
140a39ce9: 41 5f                       	pop	r15
140a39ceb: 5d                          	pop	rbp
140a39cec: c3                          	ret
140a39ced: b8 0b 00 00 00              	mov	eax, 0xb
140a39cf2: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
140a39cf6: 66 0f 6f 45 c0              	movdqa	xmm0, xmmword ptr [rbp - 0x40]
140a39cfb: 0f 28 4d d0                 	movaps	xmm1, xmmword ptr [rbp - 0x30]
140a39cff: 0f 28 55 e0                 	movaps	xmm2, xmmword ptr [rbp - 0x20]
140a39d03: 0f 28 5d f0                 	movaps	xmm3, xmmword ptr [rbp - 0x10]
140a39d07: f3 0f 7f 45 18              	movdqu	xmmword ptr [rbp + 0x18], xmm0
140a39d0c: 0f 11 4d 28                 	movups	xmmword ptr [rbp + 0x28], xmm1
140a39d10: 0f 11 55 38                 	movups	xmmword ptr [rbp + 0x38], xmm2
140a39d14: 0f 11 5d 48                 	movups	xmmword ptr [rbp + 0x48], xmm3
140a39d18: 48 8b 45 00                 	mov	rax, qword ptr [rbp]
140a39d1c: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
140a39d20: 48 8d 05 b1 07 08 02        	lea	rax, [rip + 0x20807b1]  # 0x142aba4d8
140a39d27: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a39d2c: 48 8d 0d 2d 04 08 02        	lea	rcx, [rip + 0x208042d]  # 0x142aba160
140a39d33: 4c 8d 0d 06 04 08 02        	lea	r9, [rip + 0x2080406]   # 0x142aba140
140a39d3a: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
140a39d3e: ba 2b 00 00 00              	mov	edx, 0x2b
140a39d43: e8 18 fa e9 01              	call	0x1428d9760 <.text+0x28d8760>
140a39d48: 0f 0b                       	ud2
140a39d4a: 48 8d 0d 4f 0d 20 02        	lea	rcx, [rip + 0x2200d4f]  # 0x142c3aaa0
140a39d51: e8 7a e3 e9 01              	call	0x1428d80d0 <.text+0x28d70d0>
140a39d56: cc                          	int3
140a39d57: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a39d60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39d65: 55                          	push	rbp
140a39d66: 41 57                       	push	r15
140a39d68: 41 56                       	push	r14
140a39d6a: 41 55                       	push	r13
140a39d6c: 41 54                       	push	r12
140a39d6e: 56                          	push	rsi
140a39d6f: 57                          	push	rdi
140a39d70: 53                          	push	rbx
140a39d71: 48 83 ec 58                 	sub	rsp, 0x58
140a39d75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a39d7c: 66 0f 7f 74 24 40           	movdqa	xmmword ptr [rsp + 0x40], xmm6
140a39d82: 48 83 bd a0 00 00 00 00     	cmp	qword ptr [rbp + 0xa0], 0x0
140a39d8a: 0f 94 c0                    	sete	al
140a39d8d: 48 8b 95 a8 00 00 00        	mov	rdx, qword ptr [rbp + 0xa8]
140a39d94: 48 85 d2                    	test	rdx, rdx
140a39d97: 0f 94 c1                    	sete	cl
140a39d9a: 08 c1                       	or	cl, al
140a39d9c: 75 13                       	jne	0x140a39db1 <.text+0xa38db1>
140a39d9e: 48 8b 8d b0 00 00 00        	mov	rcx, qword ptr [rbp + 0xb0]
140a39da5: 4c 8b 85 c0 00 00 00        	mov	r8, qword ptr [rbp + 0xc0]
140a39dac: e8 6f 9e bd ff              	call	0x140613c20 <.text+0x612c20>
140a39db1: c6 85 e7 00 00 00 00        	mov	byte ptr [rbp + 0xe7], 0x0
140a39db8: 66 0f 6f 74 24 40           	movdqa	xmm6, xmmword ptr [rsp + 0x40]
140a39dbe: 48 83 c4 58                 	add	rsp, 0x58
140a39dc2: 5b                          	pop	rbx
140a39dc3: 5f                          	pop	rdi
140a39dc4: 5e                          	pop	rsi
140a39dc5: 41 5c                       	pop	r12
140a39dc7: 41 5d                       	pop	r13
140a39dc9: 41 5e                       	pop	r14
140a39dcb: 41 5f                       	pop	r15
140a39dcd: 5d                          	pop	rbp
140a39dce: c3                          	ret
140a39dcf: 90                          	nop
140a39dd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39dd5: 55                          	push	rbp
140a39dd6: 41 57                       	push	r15
140a39dd8: 41 56                       	push	r14
140a39dda: 41 55                       	push	r13
140a39ddc: 41 54                       	push	r12
140a39dde: 56                          	push	rsi
140a39ddf: 57                          	push	rdi
140a39de0: 53                          	push	rbx
140a39de1: 48 83 ec 58                 	sub	rsp, 0x58
140a39de5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a39dec: 66 0f 7f 74 24 40           	movdqa	xmmword ptr [rsp + 0x40], xmm6
140a39df2: 80 bd e7 00 00 00 00        	cmp	byte ptr [rbp + 0xe7], 0x0
140a39df9: 75 16                       	jne	0x140a39e11 <.text+0xa38e11>
140a39dfb: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140a39e00: 48 83 c4 58                 	add	rsp, 0x58
140a39e04: 5b                          	pop	rbx
140a39e05: 5f                          	pop	rdi
140a39e06: 5e                          	pop	rsi
140a39e07: 41 5c                       	pop	r12
140a39e09: 41 5d                       	pop	r13
140a39e0b: 41 5e                       	pop	r14
140a39e0d: 41 5f                       	pop	r15
140a39e0f: 5d                          	pop	rbp
140a39e10: c3                          	ret
140a39e11: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
140a39e15: e8 e6 1c e4 ff              	call	0x14087bb00 <.text+0x87ab00>
140a39e1a: eb df                       	jmp	0x140a39dfb <.text+0xa38dfb>
140a39e1c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a39e20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39e25: 55                          	push	rbp
140a39e26: 41 57                       	push	r15
140a39e28: 41 56                       	push	r14
140a39e2a: 41 55                       	push	r13
140a39e2c: 41 54                       	push	r12
140a39e2e: 56                          	push	rsi
140a39e2f: 57                          	push	rdi
140a39e30: 53                          	push	rbx
140a39e31: 48 83 ec 58                 	sub	rsp, 0x58
140a39e35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a39e3c: 66 0f 7f 74 24 40           	movdqa	xmmword ptr [rsp + 0x40], xmm6
140a39e42: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
140a39e46: e8 c5 79 d7 ff              	call	0x1407b1810 <.text+0x7b0810>
140a39e4b: 66 0f 6f 74 24 40           	movdqa	xmm6, xmmword ptr [rsp + 0x40]
140a39e51: 48 83 c4 58                 	add	rsp, 0x58
140a39e55: 5b                          	pop	rbx
140a39e56: 5f                          	pop	rdi
140a39e57: 5e                          	pop	rsi
140a39e58: 41 5c                       	pop	r12
140a39e5a: 41 5d                       	pop	r13
140a39e5c: 41 5e                       	pop	r14
140a39e5e: 41 5f                       	pop	r15
140a39e60: 5d                          	pop	rbp
140a39e61: c3                          	ret
