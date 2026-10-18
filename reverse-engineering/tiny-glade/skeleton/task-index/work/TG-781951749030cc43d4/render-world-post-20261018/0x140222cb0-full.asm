
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140222cb0: 55                          	push	rbp
140222cb1: 41 57                       	push	r15
140222cb3: 41 56                       	push	r14
140222cb5: 56                          	push	rsi
140222cb6: 57                          	push	rdi
140222cb7: 53                          	push	rbx
140222cb8: 48 81 ec b8 02 00 00        	sub	rsp, 0x2b8
140222cbf: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140222cc7: 48 c7 85 30 02 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x230], -0x2
140222cd2: 48 89 ca                    	mov	rdx, rcx
140222cd5: 48 8b 31                    	mov	rsi, qword ptr [rcx]
140222cd8: 48 83 7e 50 00              	cmp	qword ptr [rsi + 0x50], 0x0
140222cdd: 48 b8 7f 84 8c fa 4c 15 f6 43       	movabs	rax, 0x43f6154cfa8c847f
140222ce7: 48 b9 b9 75 23 64 09 2e 14 4a       	movabs	rcx, 0x4a142e09642375b9
140222cf1: 0f 84 8f 00 00 00           	je	0x140222d86 <.text+0x221d86>
140222cf7: 4c 8b 46 38                 	mov	r8, qword ptr [rsi + 0x38]
140222cfb: 4c 8b 4e 40                 	mov	r9, qword ptr [rsi + 0x40]
140222cff: 4d 89 ca                    	mov	r10, r9
140222d02: 49 21 c2                    	and	r10, rax
140222d05: 45 31 db                    	xor	r11d, r11d
140222d08: 66 0f 6f 05 40 d8 72 02     	movdqa	xmm0, xmmword ptr [rip + 0x272d840] # 0x142950550
140222d10: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140222d14: f3 43 0f 6f 14 10           	movdqu	xmm2, xmmword ptr [r8 + r10]
140222d1a: 66 0f 6f da                 	movdqa	xmm3, xmm2
140222d1e: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140222d22: 66 0f d7 fb                 	pmovmskb	edi, xmm3
140222d26: 85 ff                       	test	edi, edi
140222d28: 75 30                       	jne	0x140222d5a <.text+0x221d5a>
140222d2a: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140222d2e: 66 0f d7 fa                 	pmovmskb	edi, xmm2
140222d32: 85 ff                       	test	edi, edi
140222d34: 75 50                       	jne	0x140222d86 <.text+0x221d86>
140222d36: 4d 01 da                    	add	r10, r11
140222d39: 49 83 c2 10                 	add	r10, 0x10
140222d3d: 49 83 c3 10                 	add	r11, 0x10
140222d41: 4d 21 ca                    	and	r10, r9
140222d44: eb ce                       	jmp	0x140222d14 <.text+0x221d14>
140222d46: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140222d50: 8d 5f ff                    	lea	ebx, [rdi - 0x1]
140222d53: 66 21 fb                    	and	bx, di
140222d56: 89 df                       	mov	edi, ebx
140222d58: 74 d0                       	je	0x140222d2a <.text+0x221d2a>
140222d5a: f3 0f bc df                 	tzcnt	ebx, edi
140222d5e: 4c 01 d3                    	add	rbx, r10
140222d61: 4c 21 cb                    	and	rbx, r9
140222d64: 48 f7 db                    	neg	rbx
140222d67: 48 8d 1c 5b                 	lea	rbx, [rbx + 2*rbx]
140222d6b: 49 39 4c d8 e8              	cmp	qword ptr [r8 + 8*rbx - 0x18], rcx
140222d70: 75 de                       	jne	0x140222d50 <.text+0x221d50>
140222d72: 49 39 44 d8 f0              	cmp	qword ptr [r8 + 8*rbx - 0x10], rax
140222d77: 75 d7                       	jne	0x140222d50 <.text+0x221d50>
140222d79: 49 8d 04 d8                 	lea	rax, [r8 + 8*rbx]
140222d7d: 48 8b 78 f8                 	mov	rdi, qword ptr [rax - 0x8]
140222d81: e9 ec 02 00 00              	jmp	0x140223072 <.text+0x222072>
140222d86: 44 0f b6 46 60              	movzx	r8d, byte ptr [rsi + 0x60]
140222d8b: 4c 8b 86 a0 00 00 00        	mov	r8, qword ptr [rsi + 0xa0]
140222d92: 4c 8b 96 a8 00 00 00        	mov	r10, qword ptr [rsi + 0xa8]
140222d99: 4d 89 d3                    	mov	r11, r10
140222d9c: 49 21 c3                    	and	r11, rax
140222d9f: 31 ff                       	xor	edi, edi
140222da1: 66 0f 6f 05 a7 d7 72 02     	movdqa	xmm0, xmmword ptr [rip + 0x272d7a7] # 0x142950550
140222da9: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140222dad: f3 43 0f 6f 14 18           	movdqu	xmm2, xmmword ptr [r8 + r11]
140222db3: 66 0f 6f da                 	movdqa	xmm3, xmm2
140222db7: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140222dbb: 66 0f d7 db                 	pmovmskb	ebx, xmm3
140222dbf: 85 db                       	test	ebx, ebx
140222dc1: 75 3a                       	jne	0x140222dfd <.text+0x221dfd>
140222dc3: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140222dc7: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
140222dcc: 45 85 c9                    	test	r9d, r9d
140222dcf: 0f 85 f2 00 00 00           	jne	0x140222ec7 <.text+0x221ec7>
140222dd5: 49 01 fb                    	add	r11, rdi
140222dd8: 49 83 c3 10                 	add	r11, 0x10
140222ddc: 48 83 c7 10                 	add	rdi, 0x10
140222de0: 4d 21 d3                    	and	r11, r10
140222de3: eb c8                       	jmp	0x140222dad <.text+0x221dad>
140222de5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140222df0: 44 8d 4b ff                 	lea	r9d, [rbx - 0x1]
140222df4: 66 41 21 d9                 	and	r9w, bx
140222df8: 44 89 cb                    	mov	ebx, r9d
140222dfb: 74 c6                       	je	0x140222dc3 <.text+0x221dc3>
140222dfd: f3 44 0f bc f3              	tzcnt	r14d, ebx
140222e02: 4d 01 de                    	add	r14, r11
140222e05: 4d 21 d6                    	and	r14, r10
140222e08: 4d 89 f1                    	mov	r9, r14
140222e0b: 49 f7 d9                    	neg	r9
140222e0e: 4e 8d 3c f5 00 00 00 00     	lea	r15, [8*r14]
140222e16: 49 c1 e1 07                 	shl	r9, 0x7
140222e1a: 4d 29 f9                    	sub	r9, r15
140222e1d: 4b 39 8c 08 78 ff ff ff     	cmp	qword ptr [r8 + r9 - 0x88], rcx
140222e25: 75 c9                       	jne	0x140222df0 <.text+0x221df0>
140222e27: 4b 39 44 08 80              	cmp	qword ptr [r8 + r9 - 0x80], rax
140222e2c: 75 c2                       	jne	0x140222df0 <.text+0x221df0>
140222e2e: 4d 89 f3                    	mov	r11, r14
140222e31: 49 c1 e3 07                 	shl	r11, 0x7
140222e35: 4b 8d 3c f3                 	lea	rdi, [r11 + 8*r14]
140222e39: 48 c1 ff 03                 	sar	rdi, 0x3
140222e3d: 49 bb f1 f0 f0 f0 f0 f0 f0 f0       	movabs	r11, -0xf0f0f0f0f0f0f0f
140222e47: 4c 0f af df                 	imul	r11, rdi
140222e4b: 49 8d 7b f0                 	lea	rdi, [r11 - 0x10]
140222e4f: 4c 21 d7                    	and	rdi, r10
140222e52: f3 41 0f 6f 04 38           	movdqu	xmm0, xmmword ptr [r8 + rdi]
140222e58: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140222e5c: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140222e60: 66 0f d7 d8                 	pmovmskb	ebx, xmm0
140222e64: f3 43 0f 6f 04 18           	movdqu	xmm0, xmmword ptr [r8 + r11]
140222e6a: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140222e6e: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
140222e73: 85 db                       	test	ebx, ebx
140222e75: 74 09                       	je	0x140222e80 <.text+0x221e80>
140222e77: 66 0f bd db                 	bsr	bx, bx
140222e7b: 83 f3 0f                    	xor	ebx, 0xf
140222e7e: eb 04                       	jmp	0x140222e84 <.text+0x221e84>
140222e80: 66 bb 10 00                 	mov	bx, 0x10
140222e84: 4d 01 c1                    	add	r9, r8
140222e87: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
140222e8e: f3 45 0f bc d2              	tzcnt	r10d, r10d
140222e93: 44 01 d3                    	add	ebx, r10d
140222e96: 41 b2 80                    	mov	r10b, -0x80
140222e99: 66 83 fb 0f                 	cmp	bx, 0xf
140222e9d: 77 0a                       	ja	0x140222ea9 <.text+0x221ea9>
140222e9f: 48 ff 86 b0 00 00 00        	inc	qword ptr [rsi + 0xb0]
140222ea6: 41 b2 ff                    	mov	r10b, -0x1
140222ea9: 47 88 14 18                 	mov	byte ptr [r8 + r11], r10b
140222ead: 45 88 54 38 10              	mov	byte ptr [r8 + rdi + 0x10], r10b
140222eb2: 48 ff 8e b8 00 00 00        	dec	qword ptr [rsi + 0xb8]
140222eb9: 4d 8b 41 88                 	mov	r8, qword ptr [r9 - 0x78]
140222ebd: 49 83 f8 02                 	cmp	r8, 0x2
140222ec1: 0f 85 39 01 00 00           	jne	0x140223000 <.text+0x222000>
140222ec7: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
140222ecb: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140222ece: 48 8d 5f 01                 	lea	rbx, [rdi + 0x1]
140222ed2: 48 89 1a                    	mov	qword ptr [rdx], rbx
140222ed5: 48 ba 00 00 00 00 00 00 00 80       	movabs	rdx, -0x8000000000000000
140222edf: 48 89 95 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rdx
140222ee6: 48 8d 15 c0 0d 73 02        	lea	rdx, [rip + 0x2730dc0]  # 0x142953cad
140222eed: 48 89 95 00 02 00 00        	mov	qword ptr [rbp + 0x200], rdx
140222ef4: 48 c7 85 08 02 00 00 3f 00 00 00    	mov	qword ptr [rbp + 0x208], 0x3f
140222eff: 66 c7 85 18 02 00 00 00 01  	mov	word ptr [rbp + 0x218], 0x100
140222f08: 48 c7 85 c0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1c0], 0x1
140222f13: 48 89 8d c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rcx
140222f1a: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
140222f21: 48 c7 85 e8 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x1e8], 0x8
140222f2c: 48 c7 85 f0 01 00 00 c8 00 00 00    	mov	qword ptr [rbp + 0x1f0], 0xc8
140222f37: 48 8d 05 82 73 f2 ff        	lea	rax, [rip - 0xd8c7e]    # 0x14014a2c0 <.text+0x1492c0>
140222f3e: 48 89 85 10 02 00 00        	mov	qword ptr [rbp + 0x210], rax
140222f45: c6 85 1a 02 00 00 01        	mov	byte ptr [rbp + 0x21a], 0x1
140222f4c: 48 c7 85 d8 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x0
140222f57: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140222f5b: 4c 8d 85 c0 01 00 00        	lea	r8, [rbp + 0x1c0]
140222f62: 48 89 fa                    	mov	rdx, rdi
140222f65: e8 06 c9 b1 00              	call	0x140d3f870 <.text+0xd3e870>
140222f6a: 48 39 5e 10                 	cmp	qword ptr [rsi + 0x10], rbx
140222f6e: 73 19                       	jae	0x140222f89 <.text+0x221f89>
140222f70: c6 85 2f 02 00 00 01        	mov	byte ptr [rbp + 0x22f], 0x1
140222f77: 4c 8d 05 4a 48 74 02        	lea	r8, [rip + 0x274484a]   # 0x1429677c8
140222f7e: 48 89 f1                    	mov	rcx, rsi
140222f81: 48 89 da                    	mov	rdx, rbx
140222f84: e8 07 27 09 00              	call	0x1402b5690 <.text+0x2b4690>
140222f89: 48 8b 5e 08                 	mov	rbx, qword ptr [rsi + 0x8]
140222f8d: 4c 69 f7 d0 00 00 00        	imul	r14, rdi, 0xd0
140222f94: 4a 8d 04 33                 	lea	rax, [rbx + r14]
140222f98: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
140222f9f: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140222fa3: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
140222fa7: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140222fad: e8 4e 0c 70 02              	call	0x142923c00 <.text+0x2922c00>
140222fb2: 42 83 3c 33 02              	cmp	dword ptr [rbx + r14], 0x2
140222fb7: 74 0c                       	je	0x140222fc5 <.text+0x221fc5>
140222fb9: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
140222fc0: e8 8b db 3c 00              	call	0x1405f0b50 <.text+0x5efb50>
140222fc5: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140222fc9: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140222fcf: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
140222fd6: e8 25 0c 70 02              	call	0x142923c00 <.text+0x2922c00>
140222fdb: 48 83 c6 38                 	add	rsi, 0x38
140222fdf: 48 ba b9 75 23 64 09 2e 14 4a       	movabs	rdx, 0x4a142e09642375b9
140222fe9: 49 b8 7f 84 8c fa 4c 15 f6 43       	movabs	r8, 0x43f6154cfa8c847f
140222ff3: 48 89 f1                    	mov	rcx, rsi
140222ff6: 49 89 f9                    	mov	r9, rdi
140222ff9: e8 22 fc e5 ff              	call	0x140082c20 <.text+0x81c20>
140222ffe: eb 72                       	jmp	0x140223072 <.text+0x222072>
140223000: 41 0f 10 41 f0              	movups	xmm0, xmmword ptr [r9 - 0x10]
140223005: 0f 11 85 b0 01 00 00        	movups	xmmword ptr [rbp + 0x1b0], xmm0
14022300c: 41 0f 10 41 e0              	movups	xmm0, xmmword ptr [r9 - 0x20]
140223011: 0f 11 85 a0 01 00 00        	movups	xmmword ptr [rbp + 0x1a0], xmm0
140223018: 41 0f 10 41 d0              	movups	xmm0, xmmword ptr [r9 - 0x30]
14022301d: 0f 11 85 90 01 00 00        	movups	xmmword ptr [rbp + 0x190], xmm0
140223024: f3 41 0f 6f 41 90           	movdqu	xmm0, xmmword ptr [r9 - 0x70]
14022302a: f3 41 0f 6f 49 a0           	movdqu	xmm1, xmmword ptr [r9 - 0x60]
140223030: f3 41 0f 6f 51 b0           	movdqu	xmm2, xmmword ptr [r9 - 0x50]
140223036: f3 41 0f 6f 59 c0           	movdqu	xmm3, xmmword ptr [r9 - 0x40]
14022303c: f3 0f 7f 9d 80 01 00 00     	movdqu	xmmword ptr [rbp + 0x180], xmm3
140223044: f3 0f 7f 95 70 01 00 00     	movdqu	xmmword ptr [rbp + 0x170], xmm2
14022304c: f3 0f 7f 8d 60 01 00 00     	movdqu	xmmword ptr [rbp + 0x160], xmm1
140223054: f3 0f 7f 85 50 01 00 00     	movdqu	xmmword ptr [rbp + 0x150], xmm0
14022305c: 4c 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], r8
140223063: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
14022306a: e8 d1 c9 b1 00              	call	0x140d3fa40 <.text+0xd3ea40>
14022306f: 48 89 c7                    	mov	rdi, rax
140223072: 48 89 f8                    	mov	rax, rdi
140223075: 48 81 c4 b8 02 00 00        	add	rsp, 0x2b8
14022307c: 5b                          	pop	rbx
14022307d: 5f                          	pop	rdi
14022307e: 5e                          	pop	rsi
14022307f: 41 5e                       	pop	r14
140223081: 41 5f                       	pop	r15
140223083: 5d                          	pop	rbp
140223084: c3                          	ret
140223085: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140223090: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140223095: 55                          	push	rbp
140223096: 41 57                       	push	r15
140223098: 41 56                       	push	r14
14022309a: 56                          	push	rsi
14022309b: 57                          	push	rdi
14022309c: 53                          	push	rbx
14022309d: 48 83 ec 28                 	sub	rsp, 0x28
1402230a1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1402230a8: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
1402230ac: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
1402230b2: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
1402230b9: e8 42 0b 70 02              	call	0x142923c00 <.text+0x2922c00>
1402230be: c6 85 2f 02 00 00 00        	mov	byte ptr [rbp + 0x22f], 0x0
1402230c5: 48 83 c4 28                 	add	rsp, 0x28
1402230c9: 5b                          	pop	rbx
1402230ca: 5f                          	pop	rdi
1402230cb: 5e                          	pop	rsi
1402230cc: 41 5e                       	pop	r14
1402230ce: 41 5f                       	pop	r15
1402230d0: 5d                          	pop	rbp
1402230d1: c3                          	ret
1402230d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1402230e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1402230e5: 55                          	push	rbp
1402230e6: 41 57                       	push	r15
1402230e8: 41 56                       	push	r14
1402230ea: 56                          	push	rsi
1402230eb: 57                          	push	rdi
1402230ec: 53                          	push	rbx
1402230ed: 48 83 ec 28                 	sub	rsp, 0x28
1402230f1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1402230f8: 80 bd 2f 02 00 00 00        	cmp	byte ptr [rbp + 0x22f], 0x0
1402230ff: 75 0d                       	jne	0x14022310e <.text+0x22210e>
140223101: 48 83 c4 28                 	add	rsp, 0x28
140223105: 5b                          	pop	rbx
140223106: 5f                          	pop	rdi
140223107: 5e                          	pop	rsi
140223108: 41 5e                       	pop	r14
14022310a: 41 5f                       	pop	r15
14022310c: 5d                          	pop	rbp
14022310d: c3                          	ret
14022310e: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140223112: e8 39 da 3c 00              	call	0x1405f0b50 <.text+0x5efb50>
140223117: eb e8                       	jmp	0x140223101 <.text+0x222101>
