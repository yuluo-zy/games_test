
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140122c30: 55                          	push	rbp
140122c31: 41 57                       	push	r15
140122c33: 41 56                       	push	r14
140122c35: 41 55                       	push	r13
140122c37: 41 54                       	push	r12
140122c39: 56                          	push	rsi
140122c3a: 57                          	push	rdi
140122c3b: 53                          	push	rbx
140122c3c: 48 81 ec 58 02 00 00        	sub	rsp, 0x258
140122c43: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140122c4b: 48 c7 85 d0 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x1d0], -0x2
140122c56: 48 89 ce                    	mov	rsi, rcx
140122c59: 48 8d 79 38                 	lea	rdi, [rcx + 0x38]
140122c5d: 4c 8d a9 a8 05 00 00        	lea	r13, [rcx + 0x5a8]
140122c64: 48 89 bd a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], rdi
140122c6b: 4c 89 ad b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], r13
140122c72: 48 83 b9 88 00 00 00 00     	cmp	qword ptr [rcx + 0x88], 0x0
140122c7a: 48 b8 7f 84 8c fa 4c 15 f6 43       	movabs	rax, 0x43f6154cfa8c847f
140122c84: 48 b9 b9 75 23 64 09 2e 14 4a       	movabs	rcx, 0x4a142e09642375b9
140122c8e: 48 89 95 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rdx
140122c95: 0f 84 8f 00 00 00           	je	0x140122d2a <.text+0x121d2a>
140122c9b: 4c 8b 46 70                 	mov	r8, qword ptr [rsi + 0x70]
140122c9f: 4c 8b 4e 78                 	mov	r9, qword ptr [rsi + 0x78]
140122ca3: 4d 89 ca                    	mov	r10, r9
140122ca6: 49 21 c2                    	and	r10, rax
140122ca9: 45 31 db                    	xor	r11d, r11d
140122cac: 66 0f 6f 05 9c d8 82 02     	movdqa	xmm0, xmmword ptr [rip + 0x282d89c] # 0x142950550
140122cb4: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140122cb8: f3 43 0f 6f 14 10           	movdqu	xmm2, xmmword ptr [r8 + r10]
140122cbe: 66 0f 6f da                 	movdqa	xmm3, xmm2
140122cc2: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140122cc6: 66 0f d7 db                 	pmovmskb	ebx, xmm3
140122cca: 85 db                       	test	ebx, ebx
140122ccc: 75 2f                       	jne	0x140122cfd <.text+0x121cfd>
140122cce: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140122cd2: 66 0f d7 da                 	pmovmskb	ebx, xmm2
140122cd6: 85 db                       	test	ebx, ebx
140122cd8: 75 50                       	jne	0x140122d2a <.text+0x121d2a>
140122cda: 4d 01 da                    	add	r10, r11
140122cdd: 49 83 c2 10                 	add	r10, 0x10
140122ce1: 49 83 c3 10                 	add	r11, 0x10
140122ce5: 4d 21 ca                    	and	r10, r9
140122ce8: eb ce                       	jmp	0x140122cb8 <.text+0x121cb8>
140122cea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140122cf0: 44 8d 73 ff                 	lea	r14d, [rbx - 0x1]
140122cf4: 66 41 21 de                 	and	r14w, bx
140122cf8: 44 89 f3                    	mov	ebx, r14d
140122cfb: 74 d1                       	je	0x140122cce <.text+0x121cce>
140122cfd: f3 44 0f bc f3              	tzcnt	r14d, ebx
140122d02: 4d 01 d6                    	add	r14, r10
140122d05: 4d 21 ce                    	and	r14, r9
140122d08: 49 f7 de                    	neg	r14
140122d0b: 4f 8d 34 76                 	lea	r14, [r14 + 2*r14]
140122d0f: 4b 39 4c f0 e8              	cmp	qword ptr [r8 + 8*r14 - 0x18], rcx
140122d14: 75 da                       	jne	0x140122cf0 <.text+0x121cf0>
140122d16: 4b 39 44 f0 f0              	cmp	qword ptr [r8 + 8*r14 - 0x10], rax
140122d1b: 75 d3                       	jne	0x140122cf0 <.text+0x121cf0>
140122d1d: 4b 8d 04 f0                 	lea	rax, [r8 + 8*r14]
140122d21: 48 8b 58 f8                 	mov	rbx, qword ptr [rax - 0x8]
140122d25: e9 06 03 00 00              	jmp	0x140123030 <.text+0x122030>
140122d2a: 44 0f b6 86 98 00 00 00     	movzx	r8d, byte ptr [rsi + 0x98]
140122d32: 4c 8b 86 d8 00 00 00        	mov	r8, qword ptr [rsi + 0xd8]
140122d39: 4c 8b 96 e0 00 00 00        	mov	r10, qword ptr [rsi + 0xe0]
140122d40: 4d 89 d3                    	mov	r11, r10
140122d43: 49 21 c3                    	and	r11, rax
140122d46: 31 db                       	xor	ebx, ebx
140122d48: 66 0f 6f 05 00 d8 82 02     	movdqa	xmm0, xmmword ptr [rip + 0x282d800] # 0x142950550
140122d50: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140122d54: f3 43 0f 6f 14 18           	movdqu	xmm2, xmmword ptr [r8 + r11]
140122d5a: 66 0f 6f da                 	movdqa	xmm3, xmm2
140122d5e: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140122d62: 66 44 0f d7 f3              	pmovmskb	r14d, xmm3
140122d67: 45 85 f6                    	test	r14d, r14d
140122d6a: 75 31                       	jne	0x140122d9d <.text+0x121d9d>
140122d6c: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140122d70: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
140122d75: 45 85 c9                    	test	r9d, r9d
140122d78: 0f 85 ef 00 00 00           	jne	0x140122e6d <.text+0x121e6d>
140122d7e: 49 01 db                    	add	r11, rbx
140122d81: 49 83 c3 10                 	add	r11, 0x10
140122d85: 48 83 c3 10                 	add	rbx, 0x10
140122d89: 4d 21 d3                    	and	r11, r10
140122d8c: eb c6                       	jmp	0x140122d54 <.text+0x121d54>
140122d8e: 66 90                       	nop
140122d90: 45 8d 4e ff                 	lea	r9d, [r14 - 0x1]
140122d94: 66 45 21 f1                 	and	r9w, r14w
140122d98: 45 89 ce                    	mov	r14d, r9d
140122d9b: 74 cf                       	je	0x140122d6c <.text+0x121d6c>
140122d9d: f3 45 0f bc fe              	tzcnt	r15d, r14d
140122da2: 4d 01 df                    	add	r15, r11
140122da5: 4d 21 d7                    	and	r15, r10
140122da8: 4d 89 f9                    	mov	r9, r15
140122dab: 49 f7 d9                    	neg	r9
140122dae: 4e 8d 24 fd 00 00 00 00     	lea	r12, [8*r15]
140122db6: 49 c1 e1 07                 	shl	r9, 0x7
140122dba: 4d 29 e1                    	sub	r9, r12
140122dbd: 4b 39 8c 08 78 ff ff ff     	cmp	qword ptr [r8 + r9 - 0x88], rcx
140122dc5: 75 c9                       	jne	0x140122d90 <.text+0x121d90>
140122dc7: 4b 39 44 08 80              	cmp	qword ptr [r8 + r9 - 0x80], rax
140122dcc: 75 c2                       	jne	0x140122d90 <.text+0x121d90>
140122dce: 4d 89 fb                    	mov	r11, r15
140122dd1: 49 c1 e3 07                 	shl	r11, 0x7
140122dd5: 4b 8d 1c fb                 	lea	rbx, [r11 + 8*r15]
140122dd9: 48 c1 fb 03                 	sar	rbx, 0x3
140122ddd: 49 bb f1 f0 f0 f0 f0 f0 f0 f0       	movabs	r11, -0xf0f0f0f0f0f0f0f
140122de7: 4c 0f af db                 	imul	r11, rbx
140122deb: 49 8d 5b f0                 	lea	rbx, [r11 - 0x10]
140122def: 4c 21 d3                    	and	rbx, r10
140122df2: f3 41 0f 6f 04 18           	movdqu	xmm0, xmmword ptr [r8 + rbx]
140122df8: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140122dfc: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140122e00: 66 44 0f d7 f0              	pmovmskb	r14d, xmm0
140122e05: f3 43 0f 6f 04 18           	movdqu	xmm0, xmmword ptr [r8 + r11]
140122e0b: 66 0f 74 c1                 	pcmpeqb	xmm0, xmm1
140122e0f: 66 44 0f d7 d0              	pmovmskb	r10d, xmm0
140122e14: 45 85 f6                    	test	r14d, r14d
140122e17: 74 0b                       	je	0x140122e24 <.text+0x121e24>
140122e19: 66 45 0f bd f6              	bsr	r14w, r14w
140122e1e: 41 83 f6 0f                 	xor	r14d, 0xf
140122e22: eb 05                       	jmp	0x140122e29 <.text+0x121e29>
140122e24: 66 41 be 10 00              	mov	r14w, 0x10
140122e29: 4d 01 c1                    	add	r9, r8
140122e2c: 41 81 ca 00 00 01 00        	or	r10d, 0x10000
140122e33: f3 45 0f bc d2              	tzcnt	r10d, r10d
140122e38: 45 01 d6                    	add	r14d, r10d
140122e3b: 41 b2 80                    	mov	r10b, -0x80
140122e3e: 66 41 83 fe 0f              	cmp	r14w, 0xf
140122e43: 77 0a                       	ja	0x140122e4f <.text+0x121e4f>
140122e45: 48 ff 86 e8 00 00 00        	inc	qword ptr [rsi + 0xe8]
140122e4c: 41 b2 ff                    	mov	r10b, -0x1
140122e4f: 47 88 14 18                 	mov	byte ptr [r8 + r11], r10b
140122e53: 45 88 54 18 10              	mov	byte ptr [r8 + rbx + 0x10], r10b
140122e58: 48 ff 8e f0 00 00 00        	dec	qword ptr [rsi + 0xf0]
140122e5f: 4d 8b 41 88                 	mov	r8, qword ptr [r9 - 0x78]
140122e63: 49 83 f8 02                 	cmp	r8, 0x2
140122e67: 0f 85 42 01 00 00           	jne	0x140122faf <.text+0x121faf>
140122e6d: 49 8b 5d 00                 	mov	rbx, qword ptr [r13]
140122e71: 4c 8d 73 01                 	lea	r14, [rbx + 0x1]
140122e75: 4d 89 75 00                 	mov	qword ptr [r13], r14
140122e79: 48 ba 00 00 00 00 00 00 00 80       	movabs	rdx, -0x8000000000000000
140122e83: 48 89 95 80 01 00 00        	mov	qword ptr [rbp + 0x180], rdx
140122e8a: 48 8d 15 1c 0e 83 02        	lea	rdx, [rip + 0x2830e1c]  # 0x142953cad
140122e91: 48 89 95 88 01 00 00        	mov	qword ptr [rbp + 0x188], rdx
140122e98: 48 c7 85 90 01 00 00 3f 00 00 00    	mov	qword ptr [rbp + 0x190], 0x3f
140122ea3: 66 c7 85 a0 01 00 00 00 01  	mov	word ptr [rbp + 0x1a0], 0x100
140122eac: 48 c7 85 48 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x148], 0x1
140122eb7: 48 89 8d 50 01 00 00        	mov	qword ptr [rbp + 0x150], rcx
140122ebe: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
140122ec5: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
140122ed0: 48 c7 85 78 01 00 00 c8 00 00 00    	mov	qword ptr [rbp + 0x178], 0xc8
140122edb: 48 8d 05 de 73 02 00        	lea	rax, [rip + 0x273de]    # 0x14014a2c0 <.text+0x1492c0>
140122ee2: 48 89 85 98 01 00 00        	mov	qword ptr [rbp + 0x198], rax
140122ee9: c6 85 a2 01 00 00 01        	mov	byte ptr [rbp + 0x1a2], 0x1
140122ef0: 48 c7 85 60 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x160], 0x0
140122efb: c6 85 cf 01 00 00 01        	mov	byte ptr [rbp + 0x1cf], 0x1
140122f02: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140122f06: 4c 8d 85 48 01 00 00        	lea	r8, [rbp + 0x148]
140122f0d: 48 89 da                    	mov	rdx, rbx
140122f10: e8 5b c9 c1 00              	call	0x140d3f870 <.text+0xd3e870>
140122f15: 4c 39 76 48                 	cmp	qword ptr [rsi + 0x48], r14
140122f19: 73 19                       	jae	0x140122f34 <.text+0x121f34>
140122f1b: c6 85 ce 01 00 00 01        	mov	byte ptr [rbp + 0x1ce], 0x1
140122f22: 4c 8d 05 57 d9 82 02        	lea	r8, [rip + 0x282d957]   # 0x142950880
140122f29: 48 89 f9                    	mov	rcx, rdi
140122f2c: 4c 89 f2                    	mov	rdx, r14
140122f2f: e8 5c 27 19 00              	call	0x1402b5690 <.text+0x2b4690>
140122f34: 4c 8b 76 40                 	mov	r14, qword ptr [rsi + 0x40]
140122f38: 4c 69 fb d0 00 00 00        	imul	r15, rbx, 0xd0
140122f3f: 4b 8d 04 3e                 	lea	rax, [r14 + r15]
140122f43: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
140122f4a: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140122f4e: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140122f52: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140122f58: e8 a3 0c 80 02              	call	0x142923c00 <.text+0x2922c00>
140122f5d: 43 83 3c 3e 02              	cmp	dword ptr [r14 + r15], 0x2
140122f62: 74 0c                       	je	0x140122f70 <.text+0x121f70>
140122f64: 48 8b 8d c0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c0]
140122f6b: e8 e0 db 4c 00              	call	0x1405f0b50 <.text+0x5efb50>
140122f70: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
140122f74: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140122f7a: 48 8b 8d c0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c0]
140122f81: e8 7a 0c 80 02              	call	0x142923c00 <.text+0x2922c00>
140122f86: 48 8d 4e 70                 	lea	rcx, [rsi + 0x70]
140122f8a: c6 85 cf 01 00 00 01        	mov	byte ptr [rbp + 0x1cf], 0x1
140122f91: 48 ba b9 75 23 64 09 2e 14 4a       	movabs	rdx, 0x4a142e09642375b9
140122f9b: 49 b8 7f 84 8c fa 4c 15 f6 43       	movabs	r8, 0x43f6154cfa8c847f
140122fa5: 49 89 d9                    	mov	r9, rbx
140122fa8: e8 73 fc f5 ff              	call	0x140082c20 <.text+0x81c20>
140122fad: eb 7a                       	jmp	0x140123029 <.text+0x122029>
140122faf: 41 0f 10 41 f0              	movups	xmm0, xmmword ptr [r9 - 0x10]
140122fb4: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
140122fbb: 41 0f 10 41 e0              	movups	xmm0, xmmword ptr [r9 - 0x20]
140122fc0: 0f 11 85 d0 00 00 00        	movups	xmmword ptr [rbp + 0xd0], xmm0
140122fc7: 41 0f 10 41 d0              	movups	xmm0, xmmword ptr [r9 - 0x30]
140122fcc: 0f 11 85 c0 00 00 00        	movups	xmmword ptr [rbp + 0xc0], xmm0
140122fd3: f3 41 0f 6f 41 90           	movdqu	xmm0, xmmword ptr [r9 - 0x70]
140122fd9: f3 41 0f 6f 49 a0           	movdqu	xmm1, xmmword ptr [r9 - 0x60]
140122fdf: f3 41 0f 6f 51 b0           	movdqu	xmm2, xmmword ptr [r9 - 0x50]
140122fe5: f3 41 0f 6f 59 c0           	movdqu	xmm3, xmmword ptr [r9 - 0x40]
140122feb: f3 0f 7f 9d b0 00 00 00     	movdqu	xmmword ptr [rbp + 0xb0], xmm3
140122ff3: f3 0f 7f 95 a0 00 00 00     	movdqu	xmmword ptr [rbp + 0xa0], xmm2
140122ffb: f3 0f 7f 8d 90 00 00 00     	movdqu	xmmword ptr [rbp + 0x90], xmm1
140123003: f3 0f 7f 85 80 00 00 00     	movdqu	xmmword ptr [rbp + 0x80], xmm0
14012300b: 4c 89 45 78                 	mov	qword ptr [rbp + 0x78], r8
14012300f: c6 85 cf 01 00 00 01        	mov	byte ptr [rbp + 0x1cf], 0x1
140123016: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
14012301a: 48 8d 95 a8 01 00 00        	lea	rdx, [rbp + 0x1a8]
140123021: e8 1a ca c1 00              	call	0x140d3fa40 <.text+0xd3ea40>
140123026: 48 89 c3                    	mov	rbx, rax
140123029: 48 8b 95 b8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b8]
140123030: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
140123034: 41 b8 c8 00 00 00           	mov	r8d, 0xc8
14012303a: e8 c1 0b 80 02              	call	0x142923c00 <.text+0x2922c00>
14012303f: 44 8b b6 b0 05 00 00        	mov	r14d, dword ptr [rsi + 0x5b0]
140123046: c6 85 cf 01 00 00 00        	mov	byte ptr [rbp + 0x1cf], 0x0
14012304d: 48 89 f1                    	mov	rcx, rsi
140123050: e8 fb 51 bd 00              	call	0x140cf8250 <.text+0xcf7250>
140123055: 48 8d 86 f8 00 00 00        	lea	rax, [rsi + 0xf8]
14012305c: 48 89 9d 48 01 00 00        	mov	qword ptr [rbp + 0x148], rbx
140123063: 48 8d 8e d8 01 00 00        	lea	rcx, [rsi + 0x1d8]
14012306a: 48 89 7d a8                 	mov	qword ptr [rbp - 0x58], rdi
14012306e: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140123075: 48 89 55 b0                 	mov	qword ptr [rbp - 0x50], rdx
140123079: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
14012307d: c6 85 cf 01 00 00 00        	mov	byte ptr [rbp + 0x1cf], 0x0
140123084: 4c 8d 45 a8                 	lea	r8, [rbp - 0x58]
140123088: 48 89 da                    	mov	rdx, rbx
14012308b: e8 f0 60 3e 00              	call	0x140509180 <.text+0x508180>
140123090: 48 89 c7                    	mov	rdi, rax
140123093: 48 83 78 20 00              	cmp	qword ptr [rax + 0x20], 0x0
140123098: 74 17                       	je	0x1401230b1 <.text+0x1220b1>
14012309a: c6 85 cf 01 00 00 00        	mov	byte ptr [rbp + 0x1cf], 0x0
1401230a1: 4c 8d 45 78                 	lea	r8, [rbp + 0x78]
1401230a5: 48 89 f9                    	mov	rcx, rdi
1401230a8: 31 d2                       	xor	edx, edx
1401230aa: e8 c1 c5 bd 00              	call	0x140cff670 <.text+0xcfe670>
1401230af: eb 2a                       	jmp	0x1401230db <.text+0x1220db>
1401230b1: 48 83 7f 18 00              	cmp	qword ptr [rdi + 0x18], 0x0
1401230b6: 74 3e                       	je	0x1401230f6 <.text+0x1220f6>
1401230b8: 31 c9                       	xor	ecx, ecx
1401230ba: 48 8d 41 01                 	lea	rax, [rcx + 0x1]
1401230be: 48 89 47 20                 	mov	qword ptr [rdi + 0x20], rax
1401230c2: 4c 8b 47 08                 	mov	r8, qword ptr [rdi + 0x8]
1401230c6: 49 0f af c8                 	imul	rcx, r8
1401230ca: 48 03 4f 10                 	add	rcx, qword ptr [rdi + 0x10]
1401230ce: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
1401230d2: e8 29 0b 80 02              	call	0x142923c00 <.text+0x2922c00>
1401230d7: 44 89 77 58                 	mov	dword ptr [rdi + 0x58], r14d
1401230db: 44 89 77 5c                 	mov	dword ptr [rdi + 0x5c], r14d
1401230df: 48 89 f0                    	mov	rax, rsi
1401230e2: 48 81 c4 58 02 00 00        	add	rsp, 0x258
1401230e9: 5b                          	pop	rbx
1401230ea: 5f                          	pop	rdi
1401230eb: 5e                          	pop	rsi
1401230ec: 41 5c                       	pop	r12
1401230ee: 41 5d                       	pop	r13
1401230f0: 41 5e                       	pop	r14
1401230f2: 41 5f                       	pop	r15
1401230f4: 5d                          	pop	rbp
1401230f5: c3                          	ret
1401230f6: c6 85 cf 01 00 00 00        	mov	byte ptr [rbp + 0x1cf], 0x0
1401230fd: ba 01 00 00 00              	mov	edx, 0x1
140123102: 48 89 f9                    	mov	rcx, rdi
140123105: e8 36 bb 7a 02              	call	0x1428cec40 <.text+0x28cdc40>
14012310a: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14012310e: eb aa                       	jmp	0x1401230ba <.text+0x1220ba>
140123110: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140123115: 55                          	push	rbp
140123116: 41 57                       	push	r15
140123118: 41 56                       	push	r14
14012311a: 41 55                       	push	r13
14012311c: 41 54                       	push	r12
14012311e: 56                          	push	rsi
14012311f: 57                          	push	rdi
140123120: 53                          	push	rbx
140123121: 48 83 ec 28                 	sub	rsp, 0x28
140123125: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14012312c: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
140123130: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
140123136: 48 8b 8d c0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c0]
14012313d: e8 be 0a 80 02              	call	0x142923c00 <.text+0x2922c00>
140123142: c6 85 ce 01 00 00 00        	mov	byte ptr [rbp + 0x1ce], 0x0
140123149: 48 83 c4 28                 	add	rsp, 0x28
14012314d: 5b                          	pop	rbx
14012314e: 5f                          	pop	rdi
14012314f: 5e                          	pop	rsi
140123150: 41 5c                       	pop	r12
140123152: 41 5d                       	pop	r13
140123154: 41 5e                       	pop	r14
140123156: 41 5f                       	pop	r15
140123158: 5d                          	pop	rbp
140123159: c3                          	ret
14012315a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140123160: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140123165: 55                          	push	rbp
140123166: 41 57                       	push	r15
140123168: 41 56                       	push	r14
14012316a: 41 55                       	push	r13
14012316c: 41 54                       	push	r12
14012316e: 56                          	push	rsi
14012316f: 57                          	push	rdi
140123170: 53                          	push	rbx
140123171: 48 83 ec 28                 	sub	rsp, 0x28
140123175: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14012317c: 80 bd ce 01 00 00 00        	cmp	byte ptr [rbp + 0x1ce], 0x0
140123183: 74 10                       	je	0x140123195 <.text+0x122195>
140123185: c6 85 cf 01 00 00 01        	mov	byte ptr [rbp + 0x1cf], 0x1
14012318c: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140123190: e8 bb d9 4c 00              	call	0x1405f0b50 <.text+0x5efb50>
140123195: c6 85 cf 01 00 00 01        	mov	byte ptr [rbp + 0x1cf], 0x1
14012319c: 48 83 c4 28                 	add	rsp, 0x28
1401231a0: 5b                          	pop	rbx
1401231a1: 5f                          	pop	rdi
1401231a2: 5e                          	pop	rsi
1401231a3: 41 5c                       	pop	r12
1401231a5: 41 5d                       	pop	r13
1401231a7: 41 5e                       	pop	r14
1401231a9: 41 5f                       	pop	r15
1401231ab: 5d                          	pop	rbp
1401231ac: c3                          	ret
1401231ad: 0f 1f 00                    	nop	dword ptr [rax]
1401231b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1401231b5: 55                          	push	rbp
1401231b6: 41 57                       	push	r15
1401231b8: 41 56                       	push	r14
1401231ba: 41 55                       	push	r13
1401231bc: 41 54                       	push	r12
1401231be: 56                          	push	rsi
1401231bf: 57                          	push	rdi
1401231c0: 53                          	push	rbx
1401231c1: 48 83 ec 28                 	sub	rsp, 0x28
1401231c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1401231cc: 80 bd cf 01 00 00 00        	cmp	byte ptr [rbp + 0x1cf], 0x0
1401231d3: 75 11                       	jne	0x1401231e6 <.text+0x1221e6>
1401231d5: 48 83 c4 28                 	add	rsp, 0x28
1401231d9: 5b                          	pop	rbx
1401231da: 5f                          	pop	rdi
1401231db: 5e                          	pop	rsi
1401231dc: 41 5c                       	pop	r12
1401231de: 41 5d                       	pop	r13
1401231e0: 41 5e                       	pop	r14
1401231e2: 41 5f                       	pop	r15
1401231e4: 5d                          	pop	rbp
1401231e5: c3                          	ret
1401231e6: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1401231ed: e8 ce 70 02 00              	call	0x14014a2c0 <.text+0x1492c0>
1401231f2: eb e1                       	jmp	0x1401231d5 <.text+0x1221d5>
