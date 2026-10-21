
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2dc60: 55                          	push	rbp
140b2dc61: 41 57                       	push	r15
140b2dc63: 41 56                       	push	r14
140b2dc65: 56                          	push	rsi
140b2dc66: 57                          	push	rdi
140b2dc67: 53                          	push	rbx
140b2dc68: 48 83 ec 48                 	sub	rsp, 0x48
140b2dc6c: 48 8d 6c 24 40              	lea	rbp, [rsp + 0x40]
140b2dc71: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
140b2dc79: 48 89 ce                    	mov	rsi, rcx
140b2dc7c: 48 89 55 f8                 	mov	qword ptr [rbp - 0x8], rdx
140b2dc80: 4c 8d 71 60                 	lea	r14, [rcx + 0x60]
140b2dc84: 48 8b 41 60                 	mov	rax, qword ptr [rcx + 0x60]
140b2dc88: 89 c1                       	mov	ecx, eax
140b2dc8a: 83 e1 08                    	and	ecx, 0x8
140b2dc8d: c1 e9 03                    	shr	ecx, 0x3
140b2dc90: 48 83 f8 f0                 	cmp	rax, -0x10
140b2dc94: 0f 93 c2                    	setae	dl
140b2dc97: 08 ca                       	or	dl, cl
140b2dc99: 74 12                       	je	0x140b2dcad <.text+0xb2ccad>
140b2dc9b: 4c 89 f1                    	mov	rcx, r14
140b2dc9e: 31 d2                       	xor	edx, edx
140b2dca0: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
140b2dca6: e8 05 e8 d9 01              	call	0x1428cc4b0 <.text+0x28cb4b0>
140b2dcab: eb 0b                       	jmp	0x140b2dcb8 <.text+0xb2ccb8>
140b2dcad: 48 8d 48 10                 	lea	rcx, [rax + 0x10]
140b2dcb1: f0                          	lock
140b2dcb2: 49 0f b1 0e                 	cmpxchg	qword ptr [r14], rcx
140b2dcb6: 75 e3                       	jne	0x140b2dc9b <.text+0xb2cc9b>
140b2dcb8: 4c 89 f1                    	mov	rcx, r14
140b2dcbb: e8 b0 3f 15 00              	call	0x140c81c70 <.text+0xc80c70>
140b2dcc0: 4d 89 f7                    	mov	r15, r14
140b2dcc3: 49 83 cf 01                 	or	r15, 0x1
140b2dcc7: 4c 89 f9                    	mov	rcx, r15
140b2dcca: e8 a1 3f 15 00              	call	0x140c81c70 <.text+0xc80c70>
140b2dccf: 48 83 be 80 00 00 00 00     	cmp	qword ptr [rsi + 0x80], 0x0
140b2dcd7: 0f 84 f9 00 00 00           	je	0x140b2ddd6 <.text+0xb2cdd6>
140b2dcdd: 48 8d 8e 88 00 00 00        	lea	rcx, [rsi + 0x88]
140b2dce4: 48 8d 55 f8                 	lea	rdx, [rbp - 0x8]
140b2dce8: e8 83 30 a8 ff              	call	0x1405b0d70 <.text+0x5afd70>
140b2dced: 48 8b 4e 68                 	mov	rcx, qword ptr [rsi + 0x68]
140b2dcf1: 48 8b 56 70                 	mov	rdx, qword ptr [rsi + 0x70]
140b2dcf5: 49 89 d0                    	mov	r8, rdx
140b2dcf8: 49 21 c0                    	and	r8, rax
140b2dcfb: 48 c1 e8 39                 	shr	rax, 0x39
140b2dcff: 66 0f 6e c0                 	movd	xmm0, eax
140b2dd03: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b2dd07: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b2dd0c: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
140b2dd11: 48 8d 41 f0                 	lea	rax, [rcx - 0x10]
140b2dd15: 45 31 c9                    	xor	r9d, r9d
140b2dd18: 4c 8b 55 f8                 	mov	r10, qword ptr [rbp - 0x8]
140b2dd1c: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140b2dd20: f3 42 0f 6f 14 01           	movdqu	xmm2, xmmword ptr [rcx + r8]
140b2dd26: 66 0f 6f da                 	movdqa	xmm3, xmm2
140b2dd2a: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140b2dd2e: 66 44 0f d7 db              	pmovmskb	r11d, xmm3
140b2dd33: 45 85 db                    	test	r11d, r11d
140b2dd36: 74 38                       	je	0x140b2dd70 <.text+0xb2cd70>
140b2dd38: f3 41 0f bc f3              	tzcnt	esi, r11d
140b2dd3d: 4c 01 c6                    	add	rsi, r8
140b2dd40: 48 21 d6                    	and	rsi, rdx
140b2dd43: 48 89 f7                    	mov	rdi, rsi
140b2dd46: 48 c1 e7 04                 	shl	rdi, 0x4
140b2dd4a: 48 89 c3                    	mov	rbx, rax
140b2dd4d: 48 29 fb                    	sub	rbx, rdi
140b2dd50: 4c 3b 13                    	cmp	r10, qword ptr [rbx]
140b2dd53: 74 39                       	je	0x140b2dd8e <.text+0xb2cd8e>
140b2dd55: 41 8d 73 ff                 	lea	esi, [r11 - 0x1]
140b2dd59: 66 44 21 de                 	and	si, r11w
140b2dd5d: 41 89 f3                    	mov	r11d, esi
140b2dd60: 75 d6                       	jne	0x140b2dd38 <.text+0xb2cd38>
140b2dd62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140b2dd70: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140b2dd74: 66 44 0f d7 da              	pmovmskb	r11d, xmm2
140b2dd79: 45 85 db                    	test	r11d, r11d
140b2dd7c: 75 58                       	jne	0x140b2ddd6 <.text+0xb2cdd6>
140b2dd7e: 4d 01 c8                    	add	r8, r9
140b2dd81: 49 83 c0 10                 	add	r8, 0x10
140b2dd85: 49 83 c1 10                 	add	r9, 0x10
140b2dd89: 49 21 d0                    	and	r8, rdx
140b2dd8c: eb 92                       	jmp	0x140b2dd20 <.text+0xb2cd20>
140b2dd8e: 48 f7 de                    	neg	rsi
140b2dd91: 48 c1 e6 04                 	shl	rsi, 0x4
140b2dd95: 48 8b 74 31 f8              	mov	rsi, qword ptr [rcx + rsi - 0x8]
140b2dd9a: f0                          	lock
140b2dd9b: 48 ff 06                    	inc	qword ptr [rsi]
140b2dd9e: 7e 4a                       	jle	0x140b2ddea <.text+0xb2cdea>
140b2dda0: 4c 89 f1                    	mov	rcx, r14
140b2dda3: e8 68 40 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2dda8: 4c 89 f9                    	mov	rcx, r15
140b2ddab: e8 60 40 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2ddb0: 48 c7 c0 f0 ff ff ff        	mov	rax, -0x10
140b2ddb7: f0                          	lock
140b2ddb8: 49 0f c1 06                 	xadd	qword ptr [r14], rax
140b2ddbc: 48 83 e0 f2                 	and	rax, -0xe
140b2ddc0: 48 83 f8 12                 	cmp	rax, 0x12
140b2ddc4: 74 26                       	je	0x140b2ddec <.text+0xb2cdec>
140b2ddc6: 48 89 f0                    	mov	rax, rsi
140b2ddc9: 48 83 c4 48                 	add	rsp, 0x48
140b2ddcd: 5b                          	pop	rbx
140b2ddce: 5f                          	pop	rdi
140b2ddcf: 5e                          	pop	rsi
140b2ddd0: 41 5e                       	pop	r14
140b2ddd2: 41 5f                       	pop	r15
140b2ddd4: 5d                          	pop	rbp
140b2ddd5: c3                          	ret
140b2ddd6: 4c 89 7d f0                 	mov	qword ptr [rbp - 0x10], r15
140b2ddda: 4c 89 75 e8                 	mov	qword ptr [rbp - 0x18], r14
140b2ddde: 48 8d 0d db da fb 01        	lea	rcx, [rip + 0x1fbdadb]  # 0x142aeb8c0
140b2dde5: e8 26 b5 da 01              	call	0x1428d9310 <.text+0x28d8310>
140b2ddea: 0f 0b                       	ud2
140b2ddec: 4c 89 f1                    	mov	rcx, r14
140b2ddef: e8 5c f1 d9 01              	call	0x1428ccf50 <.text+0x28cbf50>
140b2ddf4: eb d0                       	jmp	0x140b2ddc6 <.text+0xb2cdc6>
140b2ddf6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140b2de00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2de05: 55                          	push	rbp
140b2de06: 41 57                       	push	r15
140b2de08: 41 56                       	push	r14
140b2de0a: 56                          	push	rsi
140b2de0b: 57                          	push	rdi
140b2de0c: 53                          	push	rbx
140b2de0d: 48 83 ec 28                 	sub	rsp, 0x28
140b2de11: 48 8d 6a 40                 	lea	rbp, [rdx + 0x40]
140b2de15: 48 8b 75 e8                 	mov	rsi, qword ptr [rbp - 0x18]
140b2de19: 48 89 f1                    	mov	rcx, rsi
140b2de1c: e8 ef 3f 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2de21: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
140b2de25: e8 e6 3f 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2de2a: 48 c7 c0 f0 ff ff ff        	mov	rax, -0x10
140b2de31: f0                          	lock
140b2de32: 48 0f c1 06                 	xadd	qword ptr [rsi], rax
140b2de36: 48 83 e0 f2                 	and	rax, -0xe
140b2de3a: 48 83 f8 12                 	cmp	rax, 0x12
140b2de3e: 74 0d                       	je	0x140b2de4d <.text+0xb2ce4d>
140b2de40: 48 83 c4 28                 	add	rsp, 0x28
140b2de44: 5b                          	pop	rbx
140b2de45: 5f                          	pop	rdi
140b2de46: 5e                          	pop	rsi
140b2de47: 41 5e                       	pop	r14
140b2de49: 41 5f                       	pop	r15
140b2de4b: 5d                          	pop	rbp
140b2de4c: c3                          	ret
140b2de4d: 48 89 f1                    	mov	rcx, rsi
140b2de50: e8 fb f0 d9 01              	call	0x1428ccf50 <.text+0x28cbf50>
140b2de55: eb e9                       	jmp	0x140b2de40 <.text+0xb2ce40>
