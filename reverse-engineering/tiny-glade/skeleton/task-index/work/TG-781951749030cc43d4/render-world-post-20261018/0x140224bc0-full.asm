
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140224bc0: 55                          	push	rbp
140224bc1: 41 57                       	push	r15
140224bc3: 41 56                       	push	r14
140224bc5: 56                          	push	rsi
140224bc6: 57                          	push	rdi
140224bc7: 53                          	push	rbx
140224bc8: 48 81 ec b8 02 00 00        	sub	rsp, 0x2b8
140224bcf: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140224bd7: 48 c7 85 30 02 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x230], -0x2
140224be2: 48 89 ca                    	mov	rdx, rcx
140224be5: 48 8b 31                    	mov	rsi, qword ptr [rcx]
140224be8: 48 83 7e 50 00              	cmp	qword ptr [rsi + 0x50], 0x0
140224bed: 48 b8 a0 43 da d3 d1 c6 f3 59       	movabs	rax, 0x59f3c6d1d3da43a0
140224bf7: 48 b9 28 64 1a c9 41 e9 b8 c2       	movabs	rcx, -0x3d4716be36e59bd8
140224c01: 0f 84 8f 00 00 00           	je	0x140224c96 <.text+0x223c96>
140224c07: 4c 8b 46 38                 	mov	r8, qword ptr [rsi + 0x38]
140224c0b: 4c 8b 4e 40                 	mov	r9, qword ptr [rsi + 0x40]
140224c0f: 4d 89 ca                    	mov	r10, r9
140224c12: 49 21 c2                    	and	r10, rax
140224c15: 45 31 db                    	xor	r11d, r11d
140224c18: 66 0f 6f 05 60 55 71 02     	movdqa	xmm0, xmmword ptr [rip + 0x2715560] # 0x14293a180
140224c20: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140224c24: f3 43 0f 6f 14 10           	movdqu	xmm2, xmmword ptr [r8 + r10]
140224c2a: 66 0f 6f da                 	movdqa	xmm3, xmm2
140224c2e: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140224c32: 66 0f d7 fb                 	pmovmskb	edi, xmm3
140224c36: 85 ff                       	test	edi, edi
140224c38: 75 30                       	jne	0x140224c6a <.text+0x223c6a>
140224c3a: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140224c3e: 66 0f d7 fa                 	pmovmskb	edi, xmm2
140224c42: 85 ff                       	test	edi, edi
140224c44: 75 50                       	jne	0x140224c96 <.text+0x223c96>
140224c46: 4d 01 da                    	add	r10, r11
140224c49: 49 83 c2 10                 	add	r10, 0x10
140224c4d: 49 83 c3 10                 	add	r11, 0x10
140224c51: 4d 21 ca                    	and	r10, r9
140224c54: eb ce                       	jmp	0x140224c24 <.text+0x223c24>
140224c56: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140224c60: 8d 5f ff                    	lea	ebx, [rdi - 0x1]
140224c63: 66 21 fb                    	and	bx, di
140224c66: 89 df                       	mov	edi, ebx
140224c68: 74 d0                       	je	0x140224c3a <.text+0x223c3a>
140224c6a: f3 0f bc df                 	tzcnt	ebx, edi
140224c6e: 4c 01 d3                    	add	rbx, r10
140224c71: 4c 21 cb                    	and	rbx, r9
140224c74: 48 f7 db                    	neg	rbx
140224c77: 48 8d 1c 5b                 	lea	rbx, [rbx + 2*rbx]
140224c7b: 49 39 4c d8 e8              	cmp	qword ptr [r8 + 8*rbx - 0x18], rcx
140224c80: 75 de                       	jne	0x140224c60 <.text+0x223c60>
140224c82: 49 39 44 d8 f0              	cmp	qword ptr [r8 + 8*rbx - 0x10], rax
140224c87: 75 d7                       	jne	0x140224c60 <.text+0x223c60>
140224c89: 49 8d 04 d8                 	lea	rax, [r8 + 8*rbx]
140224c8d: 48 8b 78 f8                 	mov	rdi, qword ptr [rax - 0x8]
140224c91: e9 e9 02 00 00              	jmp	0x140224f7f <.text+0x223f7f>
140224c96: 44 0f b6 46 60              	movzx	r8d, byte ptr [rsi + 0x60]
140224c9b: 4c 8b 86 a0 00 00 00        	mov	r8, qword ptr [rsi + 0xa0]
140224ca2: 4c 8b 96 a8 00 00 00        	mov	r10, qword ptr [rsi + 0xa8]
140224ca9: 4d 89 d3                    	mov	r11, r10
140224cac: 49 21 c3                    	and	r11, rax
140224caf: 31 ff                       	xor	edi, edi
140224cb1: 66 0f 6f 05 c7 54 71 02     	movdqa	xmm0, xmmword ptr [rip + 0x27154c7] # 0x14293a180
140224cb9: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140224cbd: f3 43 0f 6f 14 18           	movdqu	xmm2, xmmword ptr [r8 + r11]
140224cc3: 66 0f 6f da                 	movdqa	xmm3, xmm2
140224cc7: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140224ccb: 66 0f d7 db                 	pmovmskb	ebx, xmm3
140224ccf: 85 db                       	test	ebx, ebx
140224cd1: 75 3a                       	jne	0x140224d0d <.text+0x223d0d>
140224cd3: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140224cd7: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
140224cdc: 45 85 c9                    	test	r9d, r9d
140224cdf: 0f 85 f2 00 00 00           	jne	0x140224dd7 <.text+0x223dd7>
140224ce5: 49 01 fb                    	add	r11, rdi
140224ce8: 49 83 c3 10                 	add	r11, 0x10
140224cec: 48 83 c7 10                 	add	rdi, 0x10
140224cf0: 4d 21 d3                    	and	r11, r10
140224cf3: eb c8                       	jmp	0x140224cbd <.text+0x223cbd>
140224cf5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140224d00: 44 8d 4b ff                 	lea	r9d, [rbx - 0x1]
140224d04: 66 41 21 d9                 	and	r9w, bx
140224d08: 44 89 cb                    	mov	ebx, r9d
140224d0b: 74 c6                       	je	0x140224cd3 <.text+0x223cd3>
140224d0d: f3 44 0f bc f3              	tzcnt	r14d, ebx
140224d12: 4d 01 de                    	add	r14, r11
140224d15: 4d 21 d6                    	and	r14, r10
140224d18: 4d 89 f1                    	mov	r9, r14
140224d1b: 49 f7 d9                    	neg	r9
140224d1e: 4e 8d 3c f5 00 00 00 00     	lea	r15, [8*r14]
140224d26: 49 c1 e1 07                 	shl	r9, 0x7
140224d2a: 4d 29 f9                    	sub	r9, r15
140224d2d: 4b 39 8c 08 78 ff ff ff     	cmp	qword ptr [r8 + r9 - 0x88], rcx
140224d35: 75 c9                       	jne	0x140224d00 <.text+0x223d00>
140224d37: 4b 39 44 08 80              	cmp	qword ptr [r8 + r9 - 0x80], rax
140224d3c: 75 c2                       	jne	0x140224d00 <.text+0x223d00>
140224d3e: 4d 89 f3                    	mov	r11, r14
140224d41: 49 c1 e3 07                 	shl	r11, 0x7
140224d45: 4b 8d 3c f3                 	lea	rdi, [r11 + 8*r14]
140224d49: 48 c1 ff 03                 	sar	rdi, 0x3
140224d4d: 49 bb f1 f0 f0 f0 f0 f0 f0 f0       	movabs	r11, -0xf0f0f0f0f0f0f0f
140224d57: 4c 0f af df                 	imul	r11, rdi
140224d5b: 49 8d 7b f0                 	lea	rdi, [r11 - 0x10]
140224d5f: 4c 21 d7                    	and	rdi, r10
140224d62: f3 41 0f 6f 04 38           	movdqu	xmm0, xmmword ptr [r8 + rdi]
140224d68: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140224d6c: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140224d70: 66 0f d7 d8                 	pmovmskb	ebx, xmm0
140224d74: f3 43 0f 6f 04 18           	movdqu	xmm0, xmmword ptr [r8 + r11]
140224d7a: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140224d7e: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
140224d83: 85 db                       	test	ebx, ebx
140224d85: 74 09                       	je	0x140224d90 <.text+0x223d90>
140224d87: 66 0f bd db                 	bsr	bx, bx
140224d8b: 83 f3 0f                    	xor	ebx, 0xf
140224d8e: eb 04                       	jmp	0x140224d94 <.text+0x223d94>
140224d90: 66 bb 10 00                 	mov	bx, 0x10
140224d94: 4d 01 c1                    	add	r9, r8
140224d97: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
140224d9e: f3 45 0f bc d2              	tzcnt	r10d, r10d
140224da3: 44 01 d3                    	add	ebx, r10d
140224da6: 41 b2 80                    	mov	r10b, -0x80
140224da9: 66 83 fb 0f                 	cmp	bx, 0xf
140224dad: 77 0a                       	ja	0x140224db9 <.text+0x223db9>
140224daf: 48 ff 86 b0 00 00 00        	inc	qword ptr [rsi + 0xb0]
140224db6: 41 b2 ff                    	mov	r10b, -0x1
140224db9: 47 88 14 18                 	mov	byte ptr [r8 + r11], r10b
140224dbd: 45 88 54 38 10              	mov	byte ptr [r8 + rdi + 0x10], r10b
140224dc2: 48 ff 8e b8 00 00 00        	dec	qword ptr [rsi + 0xb8]
140224dc9: 4d 8b 41 88                 	mov	r8, qword ptr [r9 - 0x78]
140224dcd: 49 83 f8 02                 	cmp	r8, 0x2
140224dd1: 0f 85 36 01 00 00           	jne	0x140224f0d <.text+0x223f0d>
140224dd7: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
140224ddb: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140224dde: 48 8d 5f 01                 	lea	rbx, [rdi + 0x1]
140224de2: 48 89 1a                    	mov	qword ptr [rdx], rbx
140224de5: 48 ba 00 00 00 00 00 00 00 80       	movabs	rdx, -0x8000000000000000
140224def: 48 89 95 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rdx
140224df6: 48 8d 15 96 fd 72 02        	lea	rdx, [rip + 0x272fd96]  # 0x142954b93
140224dfd: 48 89 95 00 02 00 00        	mov	qword ptr [rbp + 0x200], rdx
140224e04: 48 c7 85 08 02 00 00 2c 00 00 00    	mov	qword ptr [rbp + 0x208], 0x2c
140224e0f: 66 c7 85 18 02 00 00 00 01  	mov	word ptr [rbp + 0x218], 0x100
140224e18: 48 c7 85 c0 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1c0], 0x1
140224e23: 48 89 8d c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rcx
140224e2a: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
140224e31: 48 c7 85 e8 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x1e8], 0x8
140224e3c: 48 c7 85 f0 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x1f0], 0x10
140224e47: 48 c7 85 10 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x210], 0x0
140224e52: c6 85 1a 02 00 00 01        	mov	byte ptr [rbp + 0x21a], 0x1
140224e59: 48 c7 85 d8 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x0
140224e64: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140224e68: 4c 8d 85 c0 01 00 00        	lea	r8, [rbp + 0x1c0]
140224e6f: 48 89 fa                    	mov	rdx, rdi
140224e72: e8 f9 a9 b1 00              	call	0x140d3f870 <.text+0xd3e870>
140224e77: 48 39 5e 10                 	cmp	qword ptr [rsi + 0x10], rbx
140224e7b: 73 19                       	jae	0x140224e96 <.text+0x223e96>
140224e7d: c6 85 2f 02 00 00 01        	mov	byte ptr [rbp + 0x22f], 0x1
140224e84: 4c 8d 05 3d 29 74 02        	lea	r8, [rip + 0x274293d]   # 0x1429677c8
140224e8b: 48 89 f1                    	mov	rcx, rsi
140224e8e: 48 89 da                    	mov	rdx, rbx
140224e91: e8 fa 07 09 00              	call	0x1402b5690 <.text+0x2b4690>
140224e96: 48 8b 5e 08                 	mov	rbx, qword ptr [rsi + 0x8]
140224e9a: 4c 69 f7 d0 00 00 00        	imul	r14, rdi, 0xd0
140224ea1: 4a 8d 04 33                 	lea	rax, [rbx + r14]
140224ea5: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
140224eac: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140224eb0: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
140224eb4: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140224eba: e8 41 ed 6f 02              	call	0x142923c00 <.text+0x2922c00>
140224ebf: 42 83 3c 33 02              	cmp	dword ptr [rbx + r14], 0x2
140224ec4: 74 0c                       	je	0x140224ed2 <.text+0x223ed2>
140224ec6: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
140224ecd: e8 7e bc 3c 00              	call	0x1405f0b50 <.text+0x5efb50>
140224ed2: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140224ed6: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140224edc: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
140224ee3: e8 18 ed 6f 02              	call	0x142923c00 <.text+0x2922c00>
140224ee8: 48 83 c6 38                 	add	rsi, 0x38
140224eec: 48 ba 28 64 1a c9 41 e9 b8 c2       	movabs	rdx, -0x3d4716be36e59bd8
140224ef6: 49 b8 a0 43 da d3 d1 c6 f3 59       	movabs	r8, 0x59f3c6d1d3da43a0
140224f00: 48 89 f1                    	mov	rcx, rsi
140224f03: 49 89 f9                    	mov	r9, rdi
140224f06: e8 15 dd e5 ff              	call	0x140082c20 <.text+0x81c20>
140224f0b: eb 72                       	jmp	0x140224f7f <.text+0x223f7f>
140224f0d: 41 0f 10 41 f0              	movups	xmm0, xmmword ptr [r9 - 0x10]
140224f12: 0f 11 85 b0 01 00 00        	movups	xmmword ptr [rbp + 0x1b0], xmm0
140224f19: 41 0f 10 41 e0              	movups	xmm0, xmmword ptr [r9 - 0x20]
140224f1e: 0f 11 85 a0 01 00 00        	movups	xmmword ptr [rbp + 0x1a0], xmm0
140224f25: 41 0f 10 41 d0              	movups	xmm0, xmmword ptr [r9 - 0x30]
140224f2a: 0f 11 85 90 01 00 00        	movups	xmmword ptr [rbp + 0x190], xmm0
140224f31: f3 41 0f 6f 41 90           	movdqu	xmm0, xmmword ptr [r9 - 0x70]
140224f37: f3 41 0f 6f 49 a0           	movdqu	xmm1, xmmword ptr [r9 - 0x60]
140224f3d: f3 41 0f 6f 51 b0           	movdqu	xmm2, xmmword ptr [r9 - 0x50]
140224f43: f3 41 0f 6f 59 c0           	movdqu	xmm3, xmmword ptr [r9 - 0x40]
140224f49: f3 0f 7f 9d 80 01 00 00     	movdqu	xmmword ptr [rbp + 0x180], xmm3
140224f51: f3 0f 7f 95 70 01 00 00     	movdqu	xmmword ptr [rbp + 0x170], xmm2
140224f59: f3 0f 7f 8d 60 01 00 00     	movdqu	xmmword ptr [rbp + 0x160], xmm1
140224f61: f3 0f 7f 85 50 01 00 00     	movdqu	xmmword ptr [rbp + 0x150], xmm0
140224f69: 4c 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], r8
140224f70: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140224f77: e8 c4 aa b1 00              	call	0x140d3fa40 <.text+0xd3ea40>
140224f7c: 48 89 c7                    	mov	rdi, rax
140224f7f: 48 89 f8                    	mov	rax, rdi
140224f82: 48 81 c4 b8 02 00 00        	add	rsp, 0x2b8
140224f89: 5b                          	pop	rbx
140224f8a: 5f                          	pop	rdi
140224f8b: 5e                          	pop	rsi
140224f8c: 41 5e                       	pop	r14
140224f8e: 41 5f                       	pop	r15
140224f90: 5d                          	pop	rbp
140224f91: c3                          	ret
140224f92: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140224fa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140224fa5: 55                          	push	rbp
140224fa6: 41 57                       	push	r15
140224fa8: 41 56                       	push	r14
140224faa: 56                          	push	rsi
140224fab: 57                          	push	rdi
140224fac: 53                          	push	rbx
140224fad: 48 83 ec 28                 	sub	rsp, 0x28
140224fb1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140224fb8: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140224fbc: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140224fc2: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
140224fc9: e8 32 ec 6f 02              	call	0x142923c00 <.text+0x2922c00>
140224fce: c6 85 2f 02 00 00 00        	mov	byte ptr [rbp + 0x22f], 0x0
140224fd5: 48 83 c4 28                 	add	rsp, 0x28
140224fd9: 5b                          	pop	rbx
140224fda: 5f                          	pop	rdi
140224fdb: 5e                          	pop	rsi
140224fdc: 41 5e                       	pop	r14
140224fde: 41 5f                       	pop	r15
140224fe0: 5d                          	pop	rbp
140224fe1: c3                          	ret
140224fe2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140224ff0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140224ff5: 55                          	push	rbp
140224ff6: 41 57                       	push	r15
140224ff8: 41 56                       	push	r14
140224ffa: 56                          	push	rsi
140224ffb: 57                          	push	rdi
140224ffc: 53                          	push	rbx
140224ffd: 48 83 ec 28                 	sub	rsp, 0x28
140225001: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140225008: 80 bd 2f 02 00 00 00        	cmp	byte ptr [rbp + 0x22f], 0x0
14022500f: 75 0d                       	jne	0x14022501e <.text+0x22401e>
140225011: 48 83 c4 28                 	add	rsp, 0x28
140225015: 5b                          	pop	rbx
140225016: 5f                          	pop	rdi
140225017: 5e                          	pop	rsi
140225018: 41 5e                       	pop	r14
14022501a: 41 5f                       	pop	r15
14022501c: 5d                          	pop	rbp
14022501d: c3                          	ret
14022501e: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140225022: e8 29 bb 3c 00              	call	0x1405f0b50 <.text+0x5efb50>
140225027: eb e8                       	jmp	0x140225011 <.text+0x224011>
