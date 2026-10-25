
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417a1d00: 55                          	push	rbp
1417a1d01: 41 57                       	push	r15
1417a1d03: 41 56                       	push	r14
1417a1d05: 41 55                       	push	r13
1417a1d07: 41 54                       	push	r12
1417a1d09: 56                          	push	rsi
1417a1d0a: 57                          	push	rdi
1417a1d0b: 53                          	push	rbx
1417a1d0c: 48 81 ec c8 03 00 00        	sub	rsp, 0x3c8
1417a1d13: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1417a1d1b: 48 c7 85 40 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x340], -0x2
1417a1d26: 48 89 cf                    	mov	rdi, rcx
1417a1d29: 48 8b 42 38                 	mov	rax, qword ptr [rdx + 0x38]
1417a1d2d: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
1417a1d34: 0f 10 42 28                 	movups	xmm0, xmmword ptr [rdx + 0x28]
1417a1d38: 0f 29 85 50 01 00 00        	movaps	xmmword ptr [rbp + 0x150], xmm0
1417a1d3f: 48 8b 42 50                 	mov	rax, qword ptr [rdx + 0x50]
1417a1d43: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
1417a1d4a: 0f 10 42 40                 	movups	xmm0, xmmword ptr [rdx + 0x40]
1417a1d4e: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
1417a1d55: 48 8b 42 68                 	mov	rax, qword ptr [rdx + 0x68]
1417a1d59: 48 89 85 40 01 00 00        	mov	qword ptr [rbp + 0x140], rax
1417a1d60: 0f 10 42 58                 	movups	xmm0, xmmword ptr [rdx + 0x58]
1417a1d64: 0f 29 85 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm0
1417a1d6b: 48 8b 82 80 00 00 00        	mov	rax, qword ptr [rdx + 0x80]
1417a1d72: 48 89 85 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rax
1417a1d79: 0f 10 42 70                 	movups	xmm0, xmmword ptr [rdx + 0x70]
1417a1d7d: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
1417a1d84: 48 8b 82 98 00 00 00        	mov	rax, qword ptr [rdx + 0x98]
1417a1d8b: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1417a1d92: 0f 10 82 88 00 00 00        	movups	xmm0, xmmword ptr [rdx + 0x88]
1417a1d99: 0f 29 85 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm0
1417a1da0: 48 8b 82 b0 00 00 00        	mov	rax, qword ptr [rdx + 0xb0]
1417a1da7: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
1417a1dae: 0f 10 82 a0 00 00 00        	movups	xmm0, xmmword ptr [rdx + 0xa0]
1417a1db5: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
1417a1dbc: 48 8b 82 c8 00 00 00        	mov	rax, qword ptr [rdx + 0xc8]
1417a1dc3: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
1417a1dca: 0f 10 82 b8 00 00 00        	movups	xmm0, xmmword ptr [rdx + 0xb8]
1417a1dd1: 0f 29 85 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm0
1417a1dd8: 48 8b 82 58 01 00 00        	mov	rax, qword ptr [rdx + 0x158]
1417a1ddf: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
1417a1de3: 0f 10 82 18 01 00 00        	movups	xmm0, xmmword ptr [rdx + 0x118]
1417a1dea: 0f 10 8a 28 01 00 00        	movups	xmm1, xmmword ptr [rdx + 0x128]
1417a1df1: 0f 10 92 38 01 00 00        	movups	xmm2, xmmword ptr [rdx + 0x138]
1417a1df8: 0f 10 9a 48 01 00 00        	movups	xmm3, xmmword ptr [rdx + 0x148]
1417a1dff: 0f 29 5d 10                 	movaps	xmmword ptr [rbp + 0x10], xmm3
1417a1e03: 0f 29 55 00                 	movaps	xmmword ptr [rbp], xmm2
1417a1e07: 0f 29 4d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm1
1417a1e0b: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
1417a1e0f: 48 8b 82 d0 00 00 00        	mov	rax, qword ptr [rdx + 0xd0]
1417a1e16: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
1417a1e1d: 48 8b 82 d8 00 00 00        	mov	rax, qword ptr [rdx + 0xd8]
1417a1e24: 48 89 85 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rax
1417a1e2b: 4c 8b b2 e0 00 00 00        	mov	r14, qword ptr [rdx + 0xe0]
1417a1e32: f2 0f 10 82 90 01 00 00     	movsd	xmm0, qword ptr [rdx + 0x190]
1417a1e3a: f2 0f 11 45 30              	movsd	qword ptr [rbp + 0x30], xmm0
1417a1e3f: 48 8b 82 f8 00 00 00        	mov	rax, qword ptr [rdx + 0xf8]
1417a1e46: 48 89 85 80 01 00 00        	mov	qword ptr [rbp + 0x180], rax
1417a1e4d: 0f 10 82 e8 00 00 00        	movups	xmm0, xmmword ptr [rdx + 0xe8]
1417a1e54: 0f 29 85 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm0
1417a1e5b: 48 8b 82 10 01 00 00        	mov	rax, qword ptr [rdx + 0x110]
1417a1e62: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1417a1e69: 0f 10 82 00 01 00 00        	movups	xmm0, xmmword ptr [rdx + 0x100]
1417a1e70: 0f 29 85 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm0
1417a1e77: 48 8b 42 20                 	mov	rax, qword ptr [rdx + 0x20]
1417a1e7b: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
1417a1e7f: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
1417a1e82: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
1417a1e86: 0f 29 4d 50                 	movaps	xmmword ptr [rbp + 0x50], xmm1
1417a1e8a: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
1417a1e8e: f3 0f 6f 82 60 01 00 00     	movdqu	xmm0, xmmword ptr [rdx + 0x160]
1417a1e96: 0f 10 8a 70 01 00 00        	movups	xmm1, xmmword ptr [rdx + 0x170]
1417a1e9d: 48 89 95 88 00 00 00        	mov	qword ptr [rbp + 0x88], rdx
1417a1ea4: 0f 10 92 80 01 00 00        	movups	xmm2, xmmword ptr [rdx + 0x180]
1417a1eab: 0f 29 55 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm2
1417a1eaf: 0f 29 4d c0                 	movaps	xmmword ptr [rbp - 0x40], xmm1
1417a1eb3: 66 0f 7f 45 b0              	movdqa	xmmword ptr [rbp - 0x50], xmm0
1417a1eb8: 0f b6 05 4a 2d e9 01        	movzx	eax, byte ptr [rip + 0x1e92d4a] # 0x143634c09
1417a1ebf: b9 00 00 08 00              	mov	ecx, 0x80000
1417a1ec4: ba 01 00 00 00              	mov	edx, 0x1
1417a1ec9: e8 42 1d e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a1ece: 48 85 c0                    	test	rax, rax
1417a1ed1: 0f 84 23 2f 00 00           	je	0x1417a4dfa <.text+0x17a3dfa>
1417a1ed7: 48 c7 85 f8 00 00 00 00 00 08 00    	mov	qword ptr [rbp + 0xf8], 0x80000
1417a1ee2: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
1417a1ee9: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
1417a1ef4: 48 8d 85 f8 00 00 00        	lea	rax, [rbp + 0xf8]
1417a1efb: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
1417a1eff: 66 0f ef c0                 	pxor	xmm0, xmm0
1417a1f03: f3 0f 7f 45 78              	movdqu	xmmword ptr [rbp + 0x78], xmm0
1417a1f08: 48 8b 8d 58 01 00 00        	mov	rcx, qword ptr [rbp + 0x158]
1417a1f0f: 48 8b 95 60 01 00 00        	mov	rdx, qword ptr [rbp + 0x160]
1417a1f16: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
1417a1f1d: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
1417a1f24: 48 83 fa 02                 	cmp	rdx, 0x2
1417a1f28: 0f 83 3f 2c 00 00           	jae	0x1417a4b6d <.text+0x17a3b6d>
1417a1f2e: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1417a1f35: 48 8b 95 c0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c0]
1417a1f3c: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
1417a1f43: 48 83 fa 02                 	cmp	rdx, 0x2
1417a1f47: 0f 83 56 2c 00 00           	jae	0x1417a4ba3 <.text+0x17a3ba3>
1417a1f4d: 48 8b 8d 38 01 00 00        	mov	rcx, qword ptr [rbp + 0x138]
1417a1f54: 48 8b 95 40 01 00 00        	mov	rdx, qword ptr [rbp + 0x140]
1417a1f5b: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
1417a1f62: 48 83 fa 02                 	cmp	rdx, 0x2
1417a1f66: 0f 83 6d 2c 00 00           	jae	0x1417a4bd9 <.text+0x17a3bd9>
1417a1f6c: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
1417a1f73: 48 8b 95 20 01 00 00        	mov	rdx, qword ptr [rbp + 0x120]
1417a1f7a: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
1417a1f81: 48 83 fa 02                 	cmp	rdx, 0x2
1417a1f85: 0f 83 84 2c 00 00           	jae	0x1417a4c0f <.text+0x17a3c0f>
1417a1f8b: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
1417a1f92: 48 83 fa 02                 	cmp	rdx, 0x2
1417a1f96: 0f 83 a9 2c 00 00           	jae	0x1417a4c45 <.text+0x17a3c45>
1417a1f9c: 48 89 bd 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdi
1417a1fa3: 48 c7 85 28 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x228], 0x0
1417a1fae: 48 c7 85 38 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x238], 0x0
1417a1fb9: 0f b6 05 49 2c e9 01        	movzx	eax, byte ptr [rip + 0x1e92c49] # 0x143634c09
1417a1fc0: b9 07 00 00 00              	mov	ecx, 0x7
1417a1fc5: ba 01 00 00 00              	mov	edx, 0x1
1417a1fca: e8 41 1c e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a1fcf: 48 85 c0                    	test	rax, rax
1417a1fd2: 0f 84 44 2e 00 00           	je	0x1417a4e1c <.text+0x17a3e1c>
1417a1fd8: c7 40 03 73 69 6f 6e        	mov	dword ptr [rax + 0x3], 0x6e6f6973
1417a1fdf: c7 00 56 65 72 73           	mov	dword ptr [rax], 0x73726556
1417a1fe5: 48 c7 85 50 02 00 00 07 00 00 00    	mov	qword ptr [rbp + 0x250], 0x7
1417a1ff0: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a1ff7: 48 c7 85 60 02 00 00 07 00 00 00    	mov	qword ptr [rbp + 0x260], 0x7
1417a2002: c6 85 d0 02 00 00 02        	mov	byte ptr [rbp + 0x2d0], 0x2
1417a2009: 48 c7 85 d8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x0
1417a2014: 48 c7 85 e0 02 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x2
1417a201f: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a2026: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a202d: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2034: 4c 8d 8d d0 02 00 00        	lea	r9, [rbp + 0x2d0]
1417a203b: e8 f0 99 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2040: 0f b6 85 90 02 00 00        	movzx	eax, byte ptr [rbp + 0x290]
1417a2047: 48 83 f8 06                 	cmp	rax, 0x6
1417a204b: 77 69                       	ja	0x1417a20b6 <.text+0x17a10b6>
1417a204d: 48 8d 0d 8c a8 53 01        	lea	rcx, [rip + 0x153a88c]  # 0x142cdc8e0
1417a2054: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2058: 48 01 c8                    	add	rax, rcx
1417a205b: ff e0                       	jmp	rax
1417a205d: 48 8b 95 98 02 00 00        	mov	rdx, qword ptr [rbp + 0x298]
1417a2064: 48 85 d2                    	test	rdx, rdx
1417a2067: 74 59                       	je	0x1417a20c2 <.text+0x17a10c2>
1417a2069: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1417a2070: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2076: eb 37                       	jmp	0x1417a20af <.text+0x17a10af>
1417a2078: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1417a207f: 48 8b 95 a8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a8]
1417a2086: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a208d: e8 de 6c 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2092: 48 8b 95 98 02 00 00        	mov	rdx, qword ptr [rbp + 0x298]
1417a2099: 48 85 d2                    	test	rdx, rdx
1417a209c: 74 24                       	je	0x1417a20c2 <.text+0x17a10c2>
1417a209e: 48 c1 e2 05                 	shl	rdx, 0x5
1417a20a2: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a20a8: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a20af: e8 6c 1b e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a20b4: eb 0c                       	jmp	0x1417a20c2 <.text+0x17a10c2>
1417a20b6: 48 8d 8d 98 02 00 00        	lea	rcx, [rbp + 0x298]
1417a20bd: e8 7e 6e 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a20c2: 0f b6 05 40 2b e9 01        	movzx	eax, byte ptr [rip + 0x1e92b40] # 0x143634c09
1417a20c9: b9 05 00 00 00              	mov	ecx, 0x5
1417a20ce: ba 01 00 00 00              	mov	edx, 0x1
1417a20d3: e8 38 1b e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a20d8: 48 85 c0                    	test	rax, rax
1417a20db: 0f 84 56 2d 00 00           	je	0x1417a4e37 <.text+0x17a3e37>
1417a20e1: c6 40 04 73                 	mov	byte ptr [rax + 0x4], 0x73
1417a20e5: c7 00 57 61 6c 6c           	mov	dword ptr [rax], 0x6c6c6157
1417a20eb: 48 c7 85 50 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x250], 0x5
1417a20f6: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a20fd: 48 c7 85 60 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x260], 0x5
1417a2108: c6 85 2d 03 00 00 01        	mov	byte ptr [rbp + 0x32d], 0x1
1417a210f: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2116: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1417a211d: e8 0e e9 de ff              	call	0x141590a30 <.text+0x158fa30>
1417a2122: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2129: 0f 84 23 2d 00 00           	je	0x1417a4e52 <.text+0x17a3e52>
1417a212f: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2137: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a213e: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2145: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a214d: c6 85 2d 03 00 00 00        	mov	byte ptr [rbp + 0x32d], 0x0
1417a2154: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a215b: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2162: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2169: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2170: e8 bb 98 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2175: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a217c: 48 83 f8 06                 	cmp	rax, 0x6
1417a2180: 77 69                       	ja	0x1417a21eb <.text+0x17a11eb>
1417a2182: 48 8d 0d 73 a7 53 01        	lea	rcx, [rip + 0x153a773]  # 0x142cdc8fc
1417a2189: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a218d: 48 01 c8                    	add	rax, rcx
1417a2190: ff e0                       	jmp	rax
1417a2192: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2199: 48 85 d2                    	test	rdx, rdx
1417a219c: 74 59                       	je	0x1417a21f7 <.text+0x17a11f7>
1417a219e: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a21a5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a21ab: eb 37                       	jmp	0x1417a21e4 <.text+0x17a11e4>
1417a21ad: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a21b4: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a21bb: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a21c2: e8 a9 6b 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a21c7: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a21ce: 48 85 d2                    	test	rdx, rdx
1417a21d1: 74 24                       	je	0x1417a21f7 <.text+0x17a11f7>
1417a21d3: 48 c1 e2 05                 	shl	rdx, 0x5
1417a21d7: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a21dd: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a21e4: e8 37 1a e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a21e9: eb 0c                       	jmp	0x1417a21f7 <.text+0x17a11f7>
1417a21eb: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a21f2: e8 49 6d 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a21f7: 0f b6 05 0b 2a e9 01        	movzx	eax, byte ptr [rip + 0x1e92a0b] # 0x143634c09
1417a21fe: b9 05 00 00 00              	mov	ecx, 0x5
1417a2203: ba 01 00 00 00              	mov	edx, 0x1
1417a2208: e8 03 1a e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a220d: 48 85 c0                    	test	rax, rax
1417a2210: 0f 84 7a 2c 00 00           	je	0x1417a4e90 <.text+0x17a3e90>
1417a2216: c6 40 04 73                 	mov	byte ptr [rax + 0x4], 0x73
1417a221a: c7 00 52 6f 6f 66           	mov	dword ptr [rax], 0x666f6f52
1417a2220: 48 c7 85 50 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x250], 0x5
1417a222b: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2232: 48 c7 85 60 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x260], 0x5
1417a223d: c6 85 2c 03 00 00 01        	mov	byte ptr [rbp + 0x32c], 0x1
1417a2244: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a224b: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
1417a2252: e8 99 d8 de ff              	call	0x14158faf0 <.text+0x158eaf0>
1417a2257: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a225e: 0f 84 47 2c 00 00           	je	0x1417a4eab <.text+0x17a3eab>
1417a2264: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a226c: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2273: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a227a: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2282: c6 85 2c 03 00 00 00        	mov	byte ptr [rbp + 0x32c], 0x0
1417a2289: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2290: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2297: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a229e: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a22a5: e8 86 97 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a22aa: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a22b1: 48 83 f8 06                 	cmp	rax, 0x6
1417a22b5: 77 69                       	ja	0x1417a2320 <.text+0x17a1320>
1417a22b7: 48 8d 0d 5a a6 53 01        	lea	rcx, [rip + 0x153a65a]  # 0x142cdc918
1417a22be: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a22c2: 48 01 c8                    	add	rax, rcx
1417a22c5: ff e0                       	jmp	rax
1417a22c7: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a22ce: 48 85 d2                    	test	rdx, rdx
1417a22d1: 74 59                       	je	0x1417a232c <.text+0x17a132c>
1417a22d3: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a22da: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a22e0: eb 37                       	jmp	0x1417a2319 <.text+0x17a1319>
1417a22e2: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a22e9: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a22f0: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a22f7: e8 74 6a 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a22fc: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2303: 48 85 d2                    	test	rdx, rdx
1417a2306: 74 24                       	je	0x1417a232c <.text+0x17a132c>
1417a2308: 48 c1 e2 05                 	shl	rdx, 0x5
1417a230c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2312: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2319: e8 02 19 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a231e: eb 0c                       	jmp	0x1417a232c <.text+0x17a132c>
1417a2320: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2327: e8 14 6c 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a232c: 0f b6 05 d6 28 e9 01        	movzx	eax, byte ptr [rip + 0x1e928d6] # 0x143634c09
1417a2333: b9 0a 00 00 00              	mov	ecx, 0xa
1417a2338: ba 01 00 00 00              	mov	edx, 0x1
1417a233d: e8 ce 18 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2342: 48 85 c0                    	test	rax, rax
1417a2345: 0f 84 9e 2b 00 00           	je	0x1417a4ee9 <.text+0x17a3ee9>
1417a234b: 48 b9 44 65 63 6f 72 61 74 6f       	movabs	rcx, 0x6f7461726f636544
1417a2355: 48 89 08                    	mov	qword ptr [rax], rcx
1417a2358: 66 c7 40 08 72 73           	mov	word ptr [rax + 0x8], 0x7372
1417a235e: 48 c7 85 50 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x250], 0xa
1417a2369: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2370: 48 c7 85 60 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x260], 0xa
1417a237b: c6 85 2b 03 00 00 01        	mov	byte ptr [rbp + 0x32b], 0x1
1417a2382: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2389: 48 8d 95 30 01 00 00        	lea	rdx, [rbp + 0x130]
1417a2390: e8 fb ea db ff              	call	0x141560e90 <.text+0x155fe90>
1417a2395: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a239c: 0f 84 62 2b 00 00           	je	0x1417a4f04 <.text+0x17a3f04>
1417a23a2: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a23aa: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a23b1: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a23b8: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a23c0: c6 85 2b 03 00 00 00        	mov	byte ptr [rbp + 0x32b], 0x0
1417a23c7: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a23ce: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a23d5: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a23dc: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a23e3: e8 48 96 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a23e8: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a23ef: 48 83 f8 06                 	cmp	rax, 0x6
1417a23f3: 77 69                       	ja	0x1417a245e <.text+0x17a145e>
1417a23f5: 48 8d 0d 38 a5 53 01        	lea	rcx, [rip + 0x153a538]  # 0x142cdc934
1417a23fc: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2400: 48 01 c8                    	add	rax, rcx
1417a2403: ff e0                       	jmp	rax
1417a2405: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a240c: 48 85 d2                    	test	rdx, rdx
1417a240f: 74 59                       	je	0x1417a246a <.text+0x17a146a>
1417a2411: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2418: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a241e: eb 37                       	jmp	0x1417a2457 <.text+0x17a1457>
1417a2420: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2427: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a242e: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2435: e8 36 69 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a243a: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2441: 48 85 d2                    	test	rdx, rdx
1417a2444: 74 24                       	je	0x1417a246a <.text+0x17a146a>
1417a2446: 48 c1 e2 05                 	shl	rdx, 0x5
1417a244a: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2450: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2457: e8 c4 17 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a245c: eb 0c                       	jmp	0x1417a246a <.text+0x17a146a>
1417a245e: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2465: e8 d6 6a 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a246a: 0f b6 05 98 27 e9 01        	movzx	eax, byte ptr [rip + 0x1e92798] # 0x143634c09
1417a2471: b9 06 00 00 00              	mov	ecx, 0x6
1417a2476: ba 01 00 00 00              	mov	edx, 0x1
1417a247b: e8 90 17 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2480: 48 85 c0                    	test	rax, rax
1417a2483: 0f 84 b9 2a 00 00           	je	0x1417a4f42 <.text+0x17a3f42>
1417a2489: 66 c7 40 04 65 72           	mov	word ptr [rax + 0x4], 0x7265
1417a248f: c7 00 52 61 73 74           	mov	dword ptr [rax], 0x74736152
1417a2495: 48 c7 85 d8 01 00 00 06 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x6
1417a24a0: 48 89 85 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rax
1417a24a7: 48 c7 85 e8 01 00 00 06 00 00 00    	mov	qword ptr [rbp + 0x1e8], 0x6
1417a24b2: 48 8b bd a0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1a0]
1417a24b9: 48 85 ff                    	test	rdi, rdi
1417a24bc: 74 38                       	je	0x1417a24f6 <.text+0x17a14f6>
1417a24be: 49 89 c7                    	mov	r15, rax
1417a24c1: 48 8b 9d 98 01 00 00        	mov	rbx, qword ptr [rbp + 0x198]
1417a24c8: 48 89 fe                    	mov	rsi, rdi
1417a24cb: 48 c1 e6 04                 	shl	rsi, 0x4
1417a24cf: 0f b6 05 33 27 e9 01        	movzx	eax, byte ptr [rip + 0x1e92733] # 0x143634c09
1417a24d6: ba 08 00 00 00              	mov	edx, 0x8
1417a24db: 48 89 f1                    	mov	rcx, rsi
1417a24de: e8 2d 17 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a24e3: 48 85 c0                    	test	rax, rax
1417a24e6: 0f 84 e1 2d 00 00           	je	0x1417a52cd <.text+0x17a42cd>
1417a24ec: 48 83 ff 01                 	cmp	rdi, 0x1
1417a24f0: 75 0b                       	jne	0x1417a24fd <.text+0x17a14fd>
1417a24f2: 31 c9                       	xor	ecx, ecx
1417a24f4: eb 41                       	jmp	0x1417a2537 <.text+0x17a1537>
1417a24f6: b8 08 00 00 00              	mov	eax, 0x8
1417a24fb: eb 52                       	jmp	0x1417a254f <.text+0x17a154f>
1417a24fd: 48 89 fa                    	mov	rdx, rdi
1417a2500: 48 83 e2 fe                 	and	rdx, -0x2
1417a2504: 4c 8d 43 30                 	lea	r8, [rbx + 0x30]
1417a2508: 4c 8d 48 18                 	lea	r9, [rax + 0x18]
1417a250c: 31 c9                       	xor	ecx, ecx
1417a250e: 66 90                       	nop
1417a2510: 41 0f 10 40 d0              	movups	xmm0, xmmword ptr [r8 - 0x30]
1417a2515: 41 0f 11 41 e8              	movups	xmmword ptr [r9 - 0x18], xmm0
1417a251a: f3 41 0f 6f 40 f8           	movdqu	xmm0, xmmword ptr [r8 - 0x8]
1417a2520: f3 41 0f 7f 41 f8           	movdqu	xmmword ptr [r9 - 0x8], xmm0
1417a2526: 48 83 c1 02                 	add	rcx, 0x2
1417a252a: 49 83 c0 50                 	add	r8, 0x50
1417a252e: 49 83 c1 20                 	add	r9, 0x20
1417a2532: 48 39 ca                    	cmp	rdx, rcx
1417a2535: 75 d9                       	jne	0x1417a2510 <.text+0x17a1510>
1417a2537: 40 f6 c7 01                 	test	dil, 0x1
1417a253b: 74 12                       	je	0x1417a254f <.text+0x17a154f>
1417a253d: 48 8d 14 89                 	lea	rdx, [rcx + 4*rcx]
1417a2541: f3 0f 6f 04 d3              	movdqu	xmm0, xmmword ptr [rbx + 8*rdx]
1417a2546: 48 c1 e1 04                 	shl	rcx, 0x4
1417a254a: f3 0f 7f 04 08              	movdqu	xmmword ptr [rax + rcx], xmm0
1417a254f: 48 89 bd 50 02 00 00        	mov	qword ptr [rbp + 0x250], rdi
1417a2556: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a255d: 48 89 bd 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdi
1417a2564: c6 85 2a 03 00 00 01        	mov	byte ptr [rbp + 0x32a], 0x1
1417a256b: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2572: 48 8d 95 50 02 00 00        	lea	rdx, [rbp + 0x250]
1417a2579: e8 62 e0 de ff              	call	0x1415905e0 <.text+0x158f5e0>
1417a257e: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2585: 0f 84 d2 29 00 00           	je	0x1417a4f5d <.text+0x17a3f5d>
1417a258b: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2593: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a259a: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a25a1: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a25a9: c6 85 2a 03 00 00 00        	mov	byte ptr [rbp + 0x32a], 0x0
1417a25b0: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a25b7: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a25be: 4c 8d 85 d8 01 00 00        	lea	r8, [rbp + 0x1d8]
1417a25c5: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a25cc: e8 5f 94 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a25d1: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a25d8: 48 83 f8 06                 	cmp	rax, 0x6
1417a25dc: 0f 87 9d 01 00 00           	ja	0x1417a277f <.text+0x17a177f>
1417a25e2: 48 8d 0d 67 a3 53 01        	lea	rcx, [rip + 0x153a367]  # 0x142cdc950
1417a25e9: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a25ed: 48 01 c8                    	add	rax, rcx
1417a25f0: ff e0                       	jmp	rax
1417a25f2: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a25f9: 48 85 d2                    	test	rdx, rdx
1417a25fc: 74 16                       	je	0x1417a2614 <.text+0x17a1614>
1417a25fe: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a2605: 48 c1 e2 04                 	shl	rdx, 0x4
1417a2609: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a260f: e8 0c 16 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2614: 0f b6 05 ee 25 e9 01        	movzx	eax, byte ptr [rip + 0x1e925ee] # 0x143634c09
1417a261b: b9 0a 00 00 00              	mov	ecx, 0xa
1417a2620: ba 01 00 00 00              	mov	edx, 0x1
1417a2625: e8 e6 15 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a262a: 48 85 c0                    	test	rax, rax
1417a262d: 0f 84 68 29 00 00           	je	0x1417a4f9b <.text+0x17a3f9b>
1417a2633: 48 b9 57 61 6c 6c 43 6f 6c 6f       	movabs	rcx, 0x6f6c6f436c6c6157
1417a263d: 48 89 08                    	mov	qword ptr [rax], rcx
1417a2640: 66 c7 40 08 72 73           	mov	word ptr [rax + 0x8], 0x7372
1417a2646: 48 c7 85 50 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x250], 0xa
1417a2651: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2658: 48 c7 85 60 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x260], 0xa
1417a2663: c6 85 29 03 00 00 01        	mov	byte ptr [rbp + 0x329], 0x1
1417a266a: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2671: 48 8d 95 e0 00 00 00        	lea	rdx, [rbp + 0xe0]
1417a2678: e8 33 d2 de ff              	call	0x14158f8b0 <.text+0x158e8b0>
1417a267d: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2684: 0f 84 2c 29 00 00           	je	0x1417a4fb6 <.text+0x17a3fb6>
1417a268a: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2692: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2699: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a26a0: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a26a8: c6 85 29 03 00 00 00        	mov	byte ptr [rbp + 0x329], 0x0
1417a26af: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a26b6: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a26bd: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a26c4: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a26cb: e8 60 93 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a26d0: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a26d7: 48 83 f8 06                 	cmp	rax, 0x6
1417a26db: 0f 87 ed 00 00 00           	ja	0x1417a27ce <.text+0x17a17ce>
1417a26e1: 48 8d 0d 84 a2 53 01        	lea	rcx, [rip + 0x153a284]  # 0x142cdc96c
1417a26e8: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a26ec: 48 01 c8                    	add	rax, rcx
1417a26ef: ff e0                       	jmp	rax
1417a26f1: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a26f8: 48 85 d2                    	test	rdx, rdx
1417a26fb: 0f 84 d9 00 00 00           	je	0x1417a27da <.text+0x17a17da>
1417a2701: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2708: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a270e: e9 b4 00 00 00              	jmp	0x1417a27c7 <.text+0x17a17c7>
1417a2713: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a271a: 48 85 d2                    	test	rdx, rdx
1417a271d: 0f 84 cf fe ff ff           	je	0x1417a25f2 <.text+0x17a15f2>
1417a2723: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a272a: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2730: e8 eb 14 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2735: e9 b8 fe ff ff              	jmp	0x1417a25f2 <.text+0x17a15f2>
1417a273a: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2741: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2748: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a274f: e8 1c 66 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2754: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a275b: 48 85 d2                    	test	rdx, rdx
1417a275e: 0f 84 8e fe ff ff           	je	0x1417a25f2 <.text+0x17a15f2>
1417a2764: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2768: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a276e: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2775: e8 a6 14 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a277a: e9 73 fe ff ff              	jmp	0x1417a25f2 <.text+0x17a15f2>
1417a277f: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2786: e8 b5 67 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a278b: e9 62 fe ff ff              	jmp	0x1417a25f2 <.text+0x17a15f2>
1417a2790: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2797: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a279e: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a27a5: e8 c6 65 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a27aa: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a27b1: 48 85 d2                    	test	rdx, rdx
1417a27b4: 74 24                       	je	0x1417a27da <.text+0x17a17da>
1417a27b6: 48 c1 e2 05                 	shl	rdx, 0x5
1417a27ba: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a27c0: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a27c7: e8 54 14 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a27cc: eb 0c                       	jmp	0x1417a27da <.text+0x17a17da>
1417a27ce: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a27d5: e8 66 67 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a27da: 0f b6 05 28 24 e9 01        	movzx	eax, byte ptr [rip + 0x1e92428] # 0x143634c09
1417a27e1: b9 0a 00 00 00              	mov	ecx, 0xa
1417a27e6: ba 01 00 00 00              	mov	edx, 0x1
1417a27eb: e8 20 14 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a27f0: 48 85 c0                    	test	rax, rax
1417a27f3: 0f 84 fb 27 00 00           	je	0x1417a4ff4 <.text+0x17a3ff4>
1417a27f9: 48 b9 47 61 74 65 43 6f 6c 6f       	movabs	rcx, 0x6f6c6f4365746147
1417a2803: 48 89 08                    	mov	qword ptr [rax], rcx
1417a2806: 66 c7 40 08 72 73           	mov	word ptr [rax + 0x8], 0x7372
1417a280c: 48 c7 85 50 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x250], 0xa
1417a2817: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a281e: 48 c7 85 60 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x260], 0xa
1417a2829: c6 85 28 03 00 00 01        	mov	byte ptr [rbp + 0x328], 0x1
1417a2830: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2837: 48 8d 95 c0 00 00 00        	lea	rdx, [rbp + 0xc0]
1417a283e: e8 6d d0 de ff              	call	0x14158f8b0 <.text+0x158e8b0>
1417a2843: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a284a: 0f 84 bf 27 00 00           	je	0x1417a500f <.text+0x17a400f>
1417a2850: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2858: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a285f: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2866: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a286e: c6 85 28 03 00 00 00        	mov	byte ptr [rbp + 0x328], 0x0
1417a2875: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a287c: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2883: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a288a: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2891: e8 9a 91 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2896: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a289d: 48 83 f8 06                 	cmp	rax, 0x6
1417a28a1: 77 69                       	ja	0x1417a290c <.text+0x17a190c>
1417a28a3: 48 8d 0d de a0 53 01        	lea	rcx, [rip + 0x153a0de]  # 0x142cdc988
1417a28aa: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a28ae: 48 01 c8                    	add	rax, rcx
1417a28b1: ff e0                       	jmp	rax
1417a28b3: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a28ba: 48 85 d2                    	test	rdx, rdx
1417a28bd: 74 59                       	je	0x1417a2918 <.text+0x17a1918>
1417a28bf: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a28c6: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a28cc: eb 37                       	jmp	0x1417a2905 <.text+0x17a1905>
1417a28ce: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a28d5: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a28dc: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a28e3: e8 88 64 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a28e8: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a28ef: 48 85 d2                    	test	rdx, rdx
1417a28f2: 74 24                       	je	0x1417a2918 <.text+0x17a1918>
1417a28f4: 48 c1 e2 05                 	shl	rdx, 0x5
1417a28f8: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a28fe: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2905: e8 16 13 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a290a: eb 0c                       	jmp	0x1417a2918 <.text+0x17a1918>
1417a290c: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2913: e8 28 66 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2918: 0f b6 05 ea 22 e9 01        	movzx	eax, byte ptr [rip + 0x1e922ea] # 0x143634c09
1417a291f: b9 10 00 00 00              	mov	ecx, 0x10
1417a2924: ba 01 00 00 00              	mov	edx, 0x1
1417a2929: e8 e2 12 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a292e: 48 85 c0                    	test	rax, rax
1417a2931: 0f 84 16 27 00 00           	je	0x1417a504d <.text+0x17a404d>
1417a2937: f3 0f 6f 05 6b 9c 53 01     	movdqu	xmm0, xmmword ptr [rip + 0x1539c6b] # 0x142cdc5aa
1417a293f: f3 0f 7f 00                 	movdqu	xmmword ptr [rax], xmm0
1417a2943: 48 c7 85 50 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x250], 0x10
1417a294e: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2955: 48 c7 85 60 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x260], 0x10
1417a2960: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
1417a2967: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a296e: 48 8d 95 a0 00 00 00        	lea	rdx, [rbp + 0xa0]
1417a2975: e8 36 cf de ff              	call	0x14158f8b0 <.text+0x158e8b0>
1417a297a: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2981: 0f 84 e1 26 00 00           	je	0x1417a5068 <.text+0x17a4068>
1417a2987: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a298f: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2996: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a299d: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a29a5: c6 85 27 03 00 00 00        	mov	byte ptr [rbp + 0x327], 0x0
1417a29ac: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a29b3: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a29ba: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a29c1: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a29c8: e8 63 90 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a29cd: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a29d4: 48 83 f8 06                 	cmp	rax, 0x6
1417a29d8: 77 69                       	ja	0x1417a2a43 <.text+0x17a1a43>
1417a29da: 48 8d 0d c3 9f 53 01        	lea	rcx, [rip + 0x1539fc3]  # 0x142cdc9a4
1417a29e1: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a29e5: 48 01 c8                    	add	rax, rcx
1417a29e8: ff e0                       	jmp	rax
1417a29ea: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a29f1: 48 85 d2                    	test	rdx, rdx
1417a29f4: 74 59                       	je	0x1417a2a4f <.text+0x17a1a4f>
1417a29f6: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a29fd: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2a03: eb 37                       	jmp	0x1417a2a3c <.text+0x17a1a3c>
1417a2a05: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2a0c: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2a13: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2a1a: e8 51 63 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2a1f: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2a26: 48 85 d2                    	test	rdx, rdx
1417a2a29: 74 24                       	je	0x1417a2a4f <.text+0x17a1a4f>
1417a2a2b: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2a2f: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2a35: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2a3c: e8 df 11 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2a41: eb 0c                       	jmp	0x1417a2a4f <.text+0x17a1a4f>
1417a2a43: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2a4a: e8 f1 64 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2a4f: 0f b6 05 b3 21 e9 01        	movzx	eax, byte ptr [rip + 0x1e921b3] # 0x143634c09
1417a2a56: b9 07 00 00 00              	mov	ecx, 0x7
1417a2a5b: ba 01 00 00 00              	mov	edx, 0x1
1417a2a60: e8 ab 11 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2a65: 48 85 c0                    	test	rax, rax
1417a2a68: 0f 84 38 26 00 00           	je	0x1417a50a6 <.text+0x17a40a6>
1417a2a6e: c7 40 03 72 61 69 6e        	mov	dword ptr [rax + 0x3], 0x6e696172
1417a2a75: c7 00 54 65 72 72           	mov	dword ptr [rax], 0x72726554
1417a2a7b: 48 c7 85 50 02 00 00 07 00 00 00    	mov	qword ptr [rbp + 0x250], 0x7
1417a2a86: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2a8d: 48 c7 85 60 02 00 00 07 00 00 00    	mov	qword ptr [rbp + 0x260], 0x7
1417a2a98: c6 85 26 03 00 00 01        	mov	byte ptr [rbp + 0x326], 0x1
1417a2a9f: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2aa6: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
1417a2aaa: e8 c1 40 14 ff              	call	0x1408e6b70 <.text+0x8e5b70>
1417a2aaf: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2ab6: 0f 84 05 26 00 00           	je	0x1417a50c1 <.text+0x17a40c1>
1417a2abc: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2ac4: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2acb: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2ad2: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2ada: c6 85 26 03 00 00 00        	mov	byte ptr [rbp + 0x326], 0x0
1417a2ae1: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2ae8: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2aef: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2af6: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2afd: e8 2e 8f c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2b02: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a2b09: 48 83 f8 06                 	cmp	rax, 0x6
1417a2b0d: 77 69                       	ja	0x1417a2b78 <.text+0x17a1b78>
1417a2b0f: 48 8d 0d aa 9e 53 01        	lea	rcx, [rip + 0x1539eaa]  # 0x142cdc9c0
1417a2b16: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2b1a: 48 01 c8                    	add	rax, rcx
1417a2b1d: ff e0                       	jmp	rax
1417a2b1f: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2b26: 48 85 d2                    	test	rdx, rdx
1417a2b29: 74 59                       	je	0x1417a2b84 <.text+0x17a1b84>
1417a2b2b: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2b32: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2b38: eb 37                       	jmp	0x1417a2b71 <.text+0x17a1b71>
1417a2b3a: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2b41: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2b48: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2b4f: e8 1c 62 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2b54: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2b5b: 48 85 d2                    	test	rdx, rdx
1417a2b5e: 74 24                       	je	0x1417a2b84 <.text+0x17a1b84>
1417a2b60: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2b64: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2b6a: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2b71: e8 aa 10 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2b76: eb 0c                       	jmp	0x1417a2b84 <.text+0x17a1b84>
1417a2b78: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2b7f: e8 bc 63 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2b84: 0f b6 05 7e 20 e9 01        	movzx	eax, byte ptr [rip + 0x1e9207e] # 0x143634c09
1417a2b8b: b9 12 00 00 00              	mov	ecx, 0x12
1417a2b90: ba 01 00 00 00              	mov	edx, 0x1
1417a2b95: e8 76 10 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2b9a: 48 85 c0                    	test	rax, rax
1417a2b9d: 0f 84 5c 25 00 00           	je	0x1417a50ff <.text+0x17a40ff>
1417a2ba3: f3 0f 6f 05 16 9a 53 01     	movdqu	xmm0, xmmword ptr [rip + 0x1539a16] # 0x142cdc5c1
1417a2bab: f3 0f 7f 00                 	movdqu	xmmword ptr [rax], xmm0
1417a2baf: 66 c7 40 10 67 65           	mov	word ptr [rax + 0x10], 0x6567
1417a2bb5: 48 c7 85 50 02 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x250], 0x12
1417a2bc0: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2bc7: 48 c7 85 60 02 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x260], 0x12
1417a2bd2: c6 85 25 03 00 00 01        	mov	byte ptr [rbp + 0x325], 0x1
1417a2bd9: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2be0: 48 8d 55 30                 	lea	rdx, [rbp + 0x30]
1417a2be4: e8 47 e3 38 ff              	call	0x140b30f30 <.text+0xb2ff30>
1417a2be9: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2bf0: 0f 84 24 25 00 00           	je	0x1417a511a <.text+0x17a411a>
1417a2bf6: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2bfe: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2c05: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2c0c: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2c14: c6 85 25 03 00 00 00        	mov	byte ptr [rbp + 0x325], 0x0
1417a2c1b: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2c22: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2c29: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2c30: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2c37: e8 f4 8d c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2c3c: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a2c43: 48 83 f8 06                 	cmp	rax, 0x6
1417a2c47: 77 69                       	ja	0x1417a2cb2 <.text+0x17a1cb2>
1417a2c49: 48 8d 0d 8c 9d 53 01        	lea	rcx, [rip + 0x1539d8c]  # 0x142cdc9dc
1417a2c50: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2c54: 48 01 c8                    	add	rax, rcx
1417a2c57: ff e0                       	jmp	rax
1417a2c59: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2c60: 48 85 d2                    	test	rdx, rdx
1417a2c63: 74 59                       	je	0x1417a2cbe <.text+0x17a1cbe>
1417a2c65: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2c6c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2c72: eb 37                       	jmp	0x1417a2cab <.text+0x17a1cab>
1417a2c74: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2c7b: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2c82: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2c89: e8 e2 60 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2c8e: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2c95: 48 85 d2                    	test	rdx, rdx
1417a2c98: 74 24                       	je	0x1417a2cbe <.text+0x17a1cbe>
1417a2c9a: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2c9e: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2ca4: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2cab: e8 70 0f e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2cb0: eb 0c                       	jmp	0x1417a2cbe <.text+0x17a1cbe>
1417a2cb2: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2cb9: e8 82 62 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2cbe: 0f b6 05 44 1f e9 01        	movzx	eax, byte ptr [rip + 0x1e91f44] # 0x143634c09
1417a2cc5: b9 05 00 00 00              	mov	ecx, 0x5
1417a2cca: ba 01 00 00 00              	mov	edx, 0x1
1417a2ccf: e8 3c 0f e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2cd4: 48 85 c0                    	test	rax, rax
1417a2cd7: 0f 84 7b 24 00 00           	je	0x1417a5158 <.text+0x17a4158>
1417a2cdd: c6 40 04 73                 	mov	byte ptr [rax + 0x4], 0x73
1417a2ce1: c7 00 54 72 65 65           	mov	dword ptr [rax], 0x65657254
1417a2ce7: 48 c7 85 50 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x250], 0x5
1417a2cf2: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2cf9: 48 c7 85 60 02 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x260], 0x5
1417a2d04: c6 85 24 03 00 00 01        	mov	byte ptr [rbp + 0x324], 0x1
1417a2d0b: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2d12: 48 8d 95 10 01 00 00        	lea	rdx, [rbp + 0x110]
1417a2d19: e8 42 d4 de ff              	call	0x141590160 <.text+0x158f160>
1417a2d1e: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2d25: 0f 84 48 24 00 00           	je	0x1417a5173 <.text+0x17a4173>
1417a2d2b: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2d33: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2d3a: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2d41: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2d49: c6 85 24 03 00 00 00        	mov	byte ptr [rbp + 0x324], 0x0
1417a2d50: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2d57: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2d5e: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2d65: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2d6c: e8 bf 8c c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2d71: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a2d78: 48 83 f8 06                 	cmp	rax, 0x6
1417a2d7c: 77 69                       	ja	0x1417a2de7 <.text+0x17a1de7>
1417a2d7e: 48 8d 0d 73 9c 53 01        	lea	rcx, [rip + 0x1539c73]  # 0x142cdc9f8
1417a2d85: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2d89: 48 01 c8                    	add	rax, rcx
1417a2d8c: ff e0                       	jmp	rax
1417a2d8e: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2d95: 48 85 d2                    	test	rdx, rdx
1417a2d98: 74 59                       	je	0x1417a2df3 <.text+0x17a1df3>
1417a2d9a: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2da1: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2da7: eb 37                       	jmp	0x1417a2de0 <.text+0x17a1de0>
1417a2da9: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2db0: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2db7: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2dbe: e8 ad 5f 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2dc3: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2dca: 48 85 d2                    	test	rdx, rdx
1417a2dcd: 74 24                       	je	0x1417a2df3 <.text+0x17a1df3>
1417a2dcf: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2dd3: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2dd9: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2de0: e8 3b 0e e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2de5: eb 0c                       	jmp	0x1417a2df3 <.text+0x17a1df3>
1417a2de7: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2dee: e8 4d 61 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2df3: 0f b6 05 0f 1e e9 01        	movzx	eax, byte ptr [rip + 0x1e91e0f] # 0x143634c09
1417a2dfa: b9 0b 00 00 00              	mov	ecx, 0xb
1417a2dff: ba 01 00 00 00              	mov	edx, 0x1
1417a2e04: e8 07 0e e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2e09: 48 85 c0                    	test	rax, rax
1417a2e0c: 0f 84 9f 23 00 00           	je	0x1417a51b1 <.text+0x17a41b1>
1417a2e12: 48 b9 41 75 74 6f 43 6c 75 74       	movabs	rcx, 0x74756c436f747541
1417a2e1c: 48 89 08                    	mov	qword ptr [rax], rcx
1417a2e1f: c7 40 07 74 74 65 72        	mov	dword ptr [rax + 0x7], 0x72657474
1417a2e26: 48 c7 85 50 02 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x250], 0xb
1417a2e31: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2e38: 48 c7 85 60 02 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x260], 0xb
1417a2e43: c6 85 23 03 00 00 01        	mov	byte ptr [rbp + 0x323], 0x1
1417a2e4a: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2e51: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
1417a2e58: e8 f3 c1 de ff              	call	0x14158f050 <.text+0x158e050>
1417a2e5d: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2e64: 0f 84 62 23 00 00           	je	0x1417a51cc <.text+0x17a41cc>
1417a2e6a: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2e72: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2e79: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2e80: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2e88: c6 85 23 03 00 00 00        	mov	byte ptr [rbp + 0x323], 0x0
1417a2e8f: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2e96: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2e9d: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2ea4: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2eab: e8 80 8b c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2eb0: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a2eb7: 48 83 f8 06                 	cmp	rax, 0x6
1417a2ebb: 77 69                       	ja	0x1417a2f26 <.text+0x17a1f26>
1417a2ebd: 48 8d 0d 50 9b 53 01        	lea	rcx, [rip + 0x1539b50]  # 0x142cdca14
1417a2ec4: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2ec8: 48 01 c8                    	add	rax, rcx
1417a2ecb: ff e0                       	jmp	rax
1417a2ecd: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2ed4: 48 85 d2                    	test	rdx, rdx
1417a2ed7: 74 59                       	je	0x1417a2f32 <.text+0x17a1f32>
1417a2ed9: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2ee0: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a2ee6: eb 37                       	jmp	0x1417a2f1f <.text+0x17a1f1f>
1417a2ee8: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a2eef: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a2ef6: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a2efd: e8 6e 5e 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a2f02: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a2f09: 48 85 d2                    	test	rdx, rdx
1417a2f0c: 74 24                       	je	0x1417a2f32 <.text+0x17a1f32>
1417a2f0e: 48 c1 e2 05                 	shl	rdx, 0x5
1417a2f12: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a2f18: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a2f1f: e8 fc 0c e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a2f24: eb 0c                       	jmp	0x1417a2f32 <.text+0x17a1f32>
1417a2f26: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a2f2d: e8 0e 60 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a2f32: 0f b6 05 d0 1c e9 01        	movzx	eax, byte ptr [rip + 0x1e91cd0] # 0x143634c09
1417a2f39: b9 06 00 00 00              	mov	ecx, 0x6
1417a2f3e: ba 01 00 00 00              	mov	edx, 0x1
1417a2f43: e8 c8 0c e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a2f48: 48 85 c0                    	test	rax, rax
1417a2f4b: 0f 84 b9 22 00 00           	je	0x1417a520a <.text+0x17a420a>
1417a2f51: 66 c7 40 04 72 73           	mov	word ptr [rax + 0x4], 0x7372
1417a2f57: c7 00 53 74 61 69           	mov	dword ptr [rax], 0x69617453
1417a2f5d: 48 c7 85 50 02 00 00 06 00 00 00    	mov	qword ptr [rbp + 0x250], 0x6
1417a2f68: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a2f6f: 48 c7 85 60 02 00 00 06 00 00 00    	mov	qword ptr [rbp + 0x260], 0x6
1417a2f7a: c6 85 22 03 00 00 01        	mov	byte ptr [rbp + 0x322], 0x1
1417a2f81: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2f88: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
1417a2f8c: e8 7f a8 dd ff              	call	0x14157d810 <.text+0x157c810>
1417a2f91: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a2f98: 0f 84 87 22 00 00           	je	0x1417a5225 <.text+0x17a4225>
1417a2f9e: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a2fa6: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a2fad: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a2fb4: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a2fbc: c6 85 22 03 00 00 00        	mov	byte ptr [rbp + 0x322], 0x0
1417a2fc3: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a2fca: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a2fd1: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a2fd8: 4c 8d 8d 90 02 00 00        	lea	r9, [rbp + 0x290]
1417a2fdf: e8 4c 8a c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a2fe4: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a2feb: 48 83 f8 06                 	cmp	rax, 0x6
1417a2fef: 77 69                       	ja	0x1417a305a <.text+0x17a205a>
1417a2ff1: 48 8d 0d 38 9a 53 01        	lea	rcx, [rip + 0x1539a38]  # 0x142cdca30
1417a2ff8: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a2ffc: 48 01 c8                    	add	rax, rcx
1417a2fff: ff e0                       	jmp	rax
1417a3001: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a3008: 48 85 d2                    	test	rdx, rdx
1417a300b: 74 59                       	je	0x1417a3066 <.text+0x17a2066>
1417a300d: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a3014: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a301a: eb 37                       	jmp	0x1417a3053 <.text+0x17a2053>
1417a301c: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a3023: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a302a: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a3031: e8 3a 5d 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a3036: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a303d: 48 85 d2                    	test	rdx, rdx
1417a3040: 74 24                       	je	0x1417a3066 <.text+0x17a2066>
1417a3042: 48 c1 e2 05                 	shl	rdx, 0x5
1417a3046: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a304c: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a3053: e8 c8 0b e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a3058: eb 0c                       	jmp	0x1417a3066 <.text+0x17a2066>
1417a305a: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a3061: e8 da 5e 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a3066: 0f b6 05 9c 1b e9 01        	movzx	eax, byte ptr [rip + 0x1e91b9c] # 0x143634c09
1417a306d: b9 15 00 00 00              	mov	ecx, 0x15
1417a3072: ba 01 00 00 00              	mov	edx, 0x1
1417a3077: e8 94 0b e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a307c: 48 85 c0                    	test	rax, rax
1417a307f: 0f 84 de 21 00 00           	je	0x1417a5263 <.text+0x17a4263>
1417a3085: f3 0f 6f 05 5c 95 53 01     	movdqu	xmm0, xmmword ptr [rip + 0x153955c] # 0x142cdc5e9
1417a308d: f3 0f 7f 00                 	movdqu	xmmword ptr [rax], xmm0
1417a3091: 48 b9 53 75 70 70 6f 72 74 73       	movabs	rcx, 0x7374726f70707553
1417a309b: 48 89 48 0d                 	mov	qword ptr [rax + 0xd], rcx
1417a309f: 48 c7 85 50 02 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x250], 0x15
1417a30aa: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
1417a30b1: 48 c7 85 60 02 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x260], 0x15
1417a30bc: c6 85 21 03 00 00 01        	mov	byte ptr [rbp + 0x321], 0x1
1417a30c3: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a30ca: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
1417a30ce: e8 fd 89 39 ff              	call	0x140b3bad0 <.text+0xb3aad0>
1417a30d3: 80 bd d0 02 00 00 06        	cmp	byte ptr [rbp + 0x2d0], 0x6
1417a30da: 0f 84 9e 21 00 00           	je	0x1417a527e <.text+0x17a427e>
1417a30e0: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a30e8: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a30ef: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1417a30f6: 66 0f 7f 85 90 02 00 00     	movdqa	xmmword ptr [rbp + 0x290], xmm0
1417a30fe: c6 85 21 03 00 00 00        	mov	byte ptr [rbp + 0x321], 0x0
1417a3105: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a310c: 48 8d 95 28 02 00 00        	lea	rdx, [rbp + 0x228]
1417a3113: 4c 8d 85 50 02 00 00        	lea	r8, [rbp + 0x250]
1417a311a: 48 8d bd 90 02 00 00        	lea	rdi, [rbp + 0x290]
1417a3121: 49 89 f9                    	mov	r9, rdi
1417a3124: e8 07 89 c9 ff              	call	0x14143ba30 <.text+0x143aa30>
1417a3129: 0f b6 85 d0 02 00 00        	movzx	eax, byte ptr [rbp + 0x2d0]
1417a3130: 48 83 f8 06                 	cmp	rax, 0x6
1417a3134: 77 69                       	ja	0x1417a319f <.text+0x17a219f>
1417a3136: 48 8d 0d 0f 99 53 01        	lea	rcx, [rip + 0x153990f]  # 0x142cdca4c
1417a313d: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a3141: 48 01 c8                    	add	rax, rcx
1417a3144: ff e0                       	jmp	rax
1417a3146: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a314d: 48 85 d2                    	test	rdx, rdx
1417a3150: 74 59                       	je	0x1417a31ab <.text+0x17a21ab>
1417a3152: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a3159: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a315f: eb 37                       	jmp	0x1417a3198 <.text+0x17a2198>
1417a3161: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a3168: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1417a316f: 48 89 8d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rcx
1417a3176: e8 f5 5b 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a317b: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a3182: 48 85 d2                    	test	rdx, rdx
1417a3185: 74 24                       	je	0x1417a31ab <.text+0x17a21ab>
1417a3187: 48 c1 e2 05                 	shl	rdx, 0x5
1417a318b: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a3191: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a3198: e8 83 0a e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a319d: eb 0c                       	jmp	0x1417a31ab <.text+0x17a21ab>
1417a319f: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1417a31a6: e8 95 5d 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a31ab: 4c 89 75 38                 	mov	qword ptr [rbp + 0x38], r14
1417a31af: f3 0f 6f 85 28 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x228]
1417a31b7: f3 0f 7f 85 08 02 00 00     	movdqu	xmmword ptr [rbp + 0x208], xmm0
1417a31bf: 48 8b 85 38 02 00 00        	mov	rax, qword ptr [rbp + 0x238]
1417a31c6: 48 89 85 18 02 00 00        	mov	qword ptr [rbp + 0x218], rax
1417a31cd: c6 85 00 02 00 00 05        	mov	byte ptr [rbp + 0x200], 0x5
1417a31d4: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a31db: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a31e2: 4c 8d 05 15 94 53 01        	lea	r8, [rip + 0x1539415]   # 0x142cdc5fe
1417a31e9: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a31f0: 48 8d 55 70                 	lea	rdx, [rbp + 0x70]
1417a31f4: 41 b9 08 00 00 00           	mov	r9d, 0x8
1417a31fa: e8 91 f4 da ff              	call	0x141552690 <.text+0x1551690>
1417a31ff: 4c 8b bd d0 02 00 00        	mov	r15, qword ptr [rbp + 0x2d0]
1417a3206: 48 8b 8d d8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d8]
1417a320d: 4d 85 ff                    	test	r15, r15
1417a3210: 0f 84 a0 00 00 00           	je	0x1417a32b6 <.text+0x17a22b6>
1417a3216: 48 8b 85 e0 02 00 00        	mov	rax, qword ptr [rbp + 0x2e0]
1417a321d: 4c 89 bd 90 02 00 00        	mov	qword ptr [rbp + 0x290], r15
1417a3224: 48 89 8d 98 02 00 00        	mov	qword ptr [rbp + 0x298], rcx
1417a322b: 48 89 85 a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rax
1417a3232: 0f b6 85 00 02 00 00        	movzx	eax, byte ptr [rbp + 0x200]
1417a3239: 48 8d 0d 28 98 53 01        	lea	rcx, [rip + 0x1539828]  # 0x142cdca68
1417a3240: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
1417a3244: 48 01 c8                    	add	rax, rcx
1417a3247: ff e0                       	jmp	rax
1417a3249: 49 8b 37                    	mov	rsi, qword ptr [r15]
1417a324c: 49 8b 5f 08                 	mov	rbx, qword ptr [r15 + 0x8]
1417a3250: 48 89 df                    	mov	rdi, rbx
1417a3253: 48 83 c7 04                 	add	rdi, 0x4
1417a3257: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a325e: 4c 0f 43 c7                 	cmovae	r8, rdi
1417a3262: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a3265: 49 39 c0                    	cmp	r8, rax
1417a3268: 76 13                       	jbe	0x1417a327d <.text+0x17a227d>
1417a326a: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a326e: 49 29 d0                    	sub	r8, rdx
1417a3271: 48 29 d0                    	sub	rax, rdx
1417a3274: 4c 39 c0                    	cmp	rax, r8
1417a3277: 0f 82 a8 1a 00 00           	jb	0x1417a4d25 <.text+0x17a3d25>
1417a327d: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1417a3281: 49 89 d8                    	mov	r8, rbx
1417a3284: 49 29 c8                    	sub	r8, rcx
1417a3287: 76 0f                       	jbe	0x1417a3298 <.text+0x17a2298>
1417a3289: 48 03 4e 08                 	add	rcx, qword ptr [rsi + 0x8]
1417a328d: 31 d2                       	xor	edx, edx
1417a328f: e8 8c 09 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a3294: 48 89 5e 10                 	mov	qword ptr [rsi + 0x10], rbx
1417a3298: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a329c: c7 04 18 6e 75 6c 6c        	mov	dword ptr [rax + rbx], 0x6c6c756e
1417a32a3: 48 3b 7e 10                 	cmp	rdi, qword ptr [rsi + 0x10]
1417a32a7: 76 04                       	jbe	0x1417a32ad <.text+0x17a22ad>
1417a32a9: 48 89 7e 10                 	mov	qword ptr [rsi + 0x10], rdi
1417a32ad: 49 89 7f 08                 	mov	qword ptr [r15 + 0x8], rdi
1417a32b1: e9 bb 01 00 00              	jmp	0x1417a3471 <.text+0x17a2471>
1417a32b6: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a32bd: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a32c4: e8 c7 62 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a32c9: e9 23 12 00 00              	jmp	0x1417a44f1 <.text+0x17a34f1>
1417a32ce: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a32d5: 48 8d 95 08 02 00 00        	lea	rdx, [rbp + 0x208]
1417a32dc: e8 3f ec dd ff              	call	0x141581f20 <.text+0x1580f20>
1417a32e1: 48 89 c7                    	mov	rdi, rax
1417a32e4: 48 85 ff                    	test	rdi, rdi
1417a32e7: 0f 85 37 11 00 00           	jne	0x1417a4424 <.text+0x17a3424>
1417a32ed: e9 7f 01 00 00              	jmp	0x1417a3471 <.text+0x17a2471>
1417a32f2: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
1417a32f9: 48 83 f8 02                 	cmp	rax, 0x2
1417a32fd: 0f 84 d8 0c 00 00           	je	0x1417a3fdb <.text+0x17a2fdb>
1417a3303: 83 f8 01                    	cmp	eax, 0x1
1417a3306: 0f 85 2e 0d 00 00           	jne	0x1417a403a <.text+0x17a303a>
1417a330c: 4c 8b 85 10 02 00 00        	mov	r8, qword ptr [rbp + 0x210]
1417a3313: 4c 89 c1                    	mov	rcx, r8
1417a3316: 48 f7 d9                    	neg	rcx
1417a3319: 49 0f 48 c8                 	cmovs	rcx, r8
1417a331d: 41 be 14 00 00 00           	mov	r14d, 0x14
1417a3323: 48 81 f9 10 27 00 00        	cmp	rcx, 0x2710
1417a332a: 0f 82 9e 0d 00 00           	jb	0x1417a40ce <.text+0x17a30ce>
1417a3330: 41 bb 14 00 00 00           	mov	r11d, 0x14
1417a3336: 49 b9 4b 59 86 38 d6 c5 6d 34       	movabs	r9, 0x346dc5d63886594b
1417a3340: 4c 8d 15 c4 8e 53 01        	lea	r10, [rip + 0x1538ec4]  # 0x142cdc20b
1417a3347: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a3350: 48 89 c8                    	mov	rax, rcx
1417a3353: 49 f7 e1                    	mul	r9
1417a3356: 48 c1 ea 0b                 	shr	rdx, 0xb
1417a335a: 69 c2 10 27 00 00           	imul	eax, edx, 0x2710
1417a3360: 89 ce                       	mov	esi, ecx
1417a3362: 29 c6                       	sub	esi, eax
1417a3364: 69 c6 7b 14 00 00           	imul	eax, esi, 0x147b
1417a336a: c1 e8 13                    	shr	eax, 0x13
1417a336d: 6b f8 64                    	imul	edi, eax, 0x64
1417a3370: 29 fe                       	sub	esi, edi
1417a3372: 0f b7 f6                    	movzx	esi, si
1417a3375: 4d 8d 73 fc                 	lea	r14, [r11 - 0x4]
1417a3379: 41 0f b7 04 42              	movzx	eax, word ptr [r10 + 2*rax]
1417a337e: 66 42 89 84 1d cc 02 00 00  	mov	word ptr [rbp + r11 + 0x2cc], ax
1417a3387: 41 0f b7 04 72              	movzx	eax, word ptr [r10 + 2*rsi]
1417a338c: 66 42 89 84 1d ce 02 00 00  	mov	word ptr [rbp + r11 + 0x2ce], ax
1417a3395: 4d 89 f3                    	mov	r11, r14
1417a3398: 48 81 f9 ff e0 f5 05        	cmp	rcx, 0x5f5e0ff
1417a339f: 48 89 d1                    	mov	rcx, rdx
1417a33a2: 77 ac                       	ja	0x1417a3350 <.text+0x17a2350>
1417a33a4: e9 28 0d 00 00              	jmp	0x1417a40d1 <.text+0x17a30d1>
1417a33a9: 4c 8b 85 10 02 00 00        	mov	r8, qword ptr [rbp + 0x210]
1417a33b0: 4c 8b 8d 18 02 00 00        	mov	r9, qword ptr [rbp + 0x218]
1417a33b7: 48 8d 95 a8 02 00 00        	lea	rdx, [rbp + 0x2a8]
1417a33be: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a33c5: e8 16 cf dd ff              	call	0x1415802e0 <.text+0x157f2e0>
1417a33ca: 48 85 c0                    	test	rax, rax
1417a33cd: 0f 84 9e 00 00 00           	je	0x1417a3471 <.text+0x17a2471>
1417a33d3: 48 89 c1                    	mov	rcx, rax
1417a33d6: e8 c5 64 12 01              	call	0x1428c98a0 <.text+0x28c88a0>
1417a33db: e9 52 0c 00 00              	jmp	0x1417a4032 <.text+0x17a3032>
1417a33e0: 0f b6 85 01 02 00 00        	movzx	eax, byte ptr [rbp + 0x201]
1417a33e7: 48 89 c6                    	mov	rsi, rax
1417a33ea: 48 83 f6 05                 	xor	rsi, 0x5
1417a33ee: 48 8d 0d a1 8c 53 01        	lea	rcx, [rip + 0x1538ca1]  # 0x142cdc096
1417a33f5: 48 8d 1d 95 8c 53 01        	lea	rbx, [rip + 0x1538c95]  # 0x142cdc091
1417a33fc: 48 85 c0                    	test	rax, rax
1417a33ff: 48 0f 45 d9                 	cmovne	rbx, rcx
1417a3403: 49 8b 3f                    	mov	rdi, qword ptr [r15]
1417a3406: 4d 8b 77 08                 	mov	r14, qword ptr [r15 + 0x8]
1417a340a: 4d 89 f4                    	mov	r12, r14
1417a340d: 49 01 f4                    	add	r12, rsi
1417a3410: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a3417: 4d 0f 43 c4                 	cmovae	r8, r12
1417a341b: 48 8b 07                    	mov	rax, qword ptr [rdi]
1417a341e: 49 39 c0                    	cmp	r8, rax
1417a3421: 76 13                       	jbe	0x1417a3436 <.text+0x17a2436>
1417a3423: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1417a3427: 49 29 d0                    	sub	r8, rdx
1417a342a: 48 29 d0                    	sub	rax, rdx
1417a342d: 4c 39 c0                    	cmp	rax, r8
1417a3430: 0f 82 0b 19 00 00           	jb	0x1417a4d41 <.text+0x17a3d41>
1417a3436: 48 8b 4f 10                 	mov	rcx, qword ptr [rdi + 0x10]
1417a343a: 4d 89 f0                    	mov	r8, r14
1417a343d: 49 29 c8                    	sub	r8, rcx
1417a3440: 76 0f                       	jbe	0x1417a3451 <.text+0x17a2451>
1417a3442: 48 03 4f 08                 	add	rcx, qword ptr [rdi + 0x8]
1417a3446: 31 d2                       	xor	edx, edx
1417a3448: e8 d3 07 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a344d: 4c 89 77 10                 	mov	qword ptr [rdi + 0x10], r14
1417a3451: 4c 03 77 08                 	add	r14, qword ptr [rdi + 0x8]
1417a3455: 4c 89 f1                    	mov	rcx, r14
1417a3458: 48 89 da                    	mov	rdx, rbx
1417a345b: 49 89 f0                    	mov	r8, rsi
1417a345e: e8 9d 07 18 01              	call	0x142923c00 <.text+0x2922c00>
1417a3463: 4c 3b 67 10                 	cmp	r12, qword ptr [rdi + 0x10]
1417a3467: 76 04                       	jbe	0x1417a346d <.text+0x17a246d>
1417a3469: 4c 89 67 10                 	mov	qword ptr [rdi + 0x10], r12
1417a346d: 4d 89 67 08                 	mov	qword ptr [r15 + 0x8], r12
1417a3471: 48 8b bd 90 02 00 00        	mov	rdi, qword ptr [rbp + 0x290]
1417a3478: 4c 8b b5 a0 02 00 00        	mov	r14, qword ptr [rbp + 0x2a0]
1417a347f: 48 8b 37                    	mov	rsi, qword ptr [rdi]
1417a3482: 48 8b 5f 08                 	mov	rbx, qword ptr [rdi + 0x8]
1417a3486: 4d 8d 7e 08                 	lea	r15, [r14 + 0x8]
1417a348a: 4c 89 f8                    	mov	rax, r15
1417a348d: 48 83 c0 08                 	add	rax, 0x8
1417a3491: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a3498: 4c 0f 43 c0                 	cmovae	r8, rax
1417a349c: 4c 89 7f 08                 	mov	qword ptr [rdi + 0x8], r15
1417a34a0: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a34a3: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a34a7: 49 39 c0                    	cmp	r8, rax
1417a34aa: 76 0f                       	jbe	0x1417a34bb <.text+0x17a24bb>
1417a34ac: 49 29 d0                    	sub	r8, rdx
1417a34af: 48 29 d0                    	sub	rax, rdx
1417a34b2: 4c 39 c0                    	cmp	rax, r8
1417a34b5: 0f 82 ee 17 00 00           	jb	0x1417a4ca9 <.text+0x17a3ca9>
1417a34bb: 49 89 dc                    	mov	r12, rbx
1417a34be: 4d 29 f4                    	sub	r12, r14
1417a34c1: 49 83 c4 f0                 	add	r12, -0x10
1417a34c5: 4d 89 f8                    	mov	r8, r15
1417a34c8: 49 29 d0                    	sub	r8, rdx
1417a34cb: 76 12                       	jbe	0x1417a34df <.text+0x17a24df>
1417a34cd: 48 03 56 08                 	add	rdx, qword ptr [rsi + 0x8]
1417a34d1: 48 89 d1                    	mov	rcx, rdx
1417a34d4: 31 d2                       	xor	edx, edx
1417a34d6: e8 45 07 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a34db: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
1417a34df: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a34e3: 4e 89 64 30 08              	mov	qword ptr [rax + r14 + 0x8], r12
1417a34e8: 49 83 c6 10                 	add	r14, 0x10
1417a34ec: 4c 3b 76 10                 	cmp	r14, qword ptr [rsi + 0x10]
1417a34f0: 76 04                       	jbe	0x1417a34f6 <.text+0x17a24f6>
1417a34f2: 4c 89 76 10                 	mov	qword ptr [rsi + 0x10], r14
1417a34f6: 48 89 5f 08                 	mov	qword ptr [rdi + 0x8], rbx
1417a34fa: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1417a3501: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
1417a3508: 48 83 38 00                 	cmp	qword ptr [rax], 0x0
1417a350c: 74 0c                       	je	0x1417a351a <.text+0x17a251a>
1417a350e: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a3515: e8 e6 44 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a351a: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a3521: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a3528: 48 8b 95 90 01 00 00        	mov	rdx, qword ptr [rbp + 0x190]
1417a352f: 48 8b 8d 98 01 00 00        	mov	rcx, qword ptr [rbp + 0x198]
1417a3536: 48 8b 85 a0 01 00 00        	mov	rax, qword ptr [rbp + 0x1a0]
1417a353d: 48 c1 e0 03                 	shl	rax, 0x3
1417a3541: 4c 8d 04 80                 	lea	r8, [rax + 4*rax]
1417a3545: 49 01 c8                    	add	r8, rcx
1417a3548: 48 89 8d d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rcx
1417a354f: 48 89 95 90 00 00 00        	mov	qword ptr [rbp + 0x90], rdx
1417a3556: 48 89 95 e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rdx
1417a355d: 4c 89 85 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], r8
1417a3564: 4c 89 85 f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], r8
1417a356b: 4c 8d 61 40                 	lea	r12, [rcx + 0x40]
1417a356f: 48 8d 34 80                 	lea	rsi, [rax + 4*rax]
1417a3573: 48 83 c6 d8                 	add	rsi, -0x28
1417a3577: 48 89 8d 98 00 00 00        	mov	qword ptr [rbp + 0x98], rcx
1417a357e: 48 89 c8                    	mov	rax, rcx
1417a3581: eb 18                       	jmp	0x1417a359b <.text+0x17a259b>
1417a3583: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1417a3590: 49 83 c4 28                 	add	r12, 0x28
1417a3594: 48 83 c6 d8                 	add	rsi, -0x28
1417a3598: 48 89 f8                    	mov	rax, rdi
1417a359b: 48 3b 85 c8 01 00 00        	cmp	rax, qword ptr [rbp + 0x1c8]
1417a35a2: 0f 84 5c 04 00 00           	je	0x1417a3a04 <.text+0x17a2a04>
1417a35a8: 48 8d 78 28                 	lea	rdi, [rax + 0x28]
1417a35ac: 48 89 bd e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rdi
1417a35b3: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
1417a35b7: 48 89 d1                    	mov	rcx, rdx
1417a35ba: 48 f7 d9                    	neg	rcx
1417a35bd: 0f 80 f4 03 00 00           	jo	0x1417a39b7 <.text+0x17a29b7>
1417a35c3: 48 89 b5 20 02 00 00        	mov	qword ptr [rbp + 0x220], rsi
1417a35ca: 48 89 95 40 02 00 00        	mov	qword ptr [rbp + 0x240], rdx
1417a35d1: 48 8b 08                    	mov	rcx, qword ptr [rax]
1417a35d4: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1417a35d8: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
1417a35dc: 4c 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], r8
1417a35e3: 48 8b 70 20                 	mov	rsi, qword ptr [rax + 0x20]
1417a35e7: e8 14 d7 8d 00              	call	0x142080d00 <.text+0x207fd00>
1417a35ec: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a35f3: 48 8d 55 70                 	lea	rdx, [rbp + 0x70]
1417a35f7: 49 89 c0                    	mov	r8, rax
1417a35fa: e8 21 f2 da ff              	call	0x141552820 <.text+0x1551820>
1417a35ff: 48 8b 9d d0 02 00 00        	mov	rbx, qword ptr [rbp + 0x2d0]
1417a3606: 48 8b 8d d8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d8]
1417a360d: 48 85 db                    	test	rbx, rbx
1417a3610: 0f 84 c1 05 00 00           	je	0x1417a3bd7 <.text+0x17a2bd7>
1417a3616: 48 8b 85 e0 02 00 00        	mov	rax, qword ptr [rbp + 0x2e0]
1417a361d: 48 89 9d 28 02 00 00        	mov	qword ptr [rbp + 0x228], rbx
1417a3624: 48 89 8d 30 02 00 00        	mov	qword ptr [rbp + 0x230], rcx
1417a362b: 48 89 85 38 02 00 00        	mov	qword ptr [rbp + 0x238], rax
1417a3632: 4c 8b 3b                    	mov	r15, qword ptr [rbx]
1417a3635: 4c 8b 6b 08                 	mov	r13, qword ptr [rbx + 0x8]
1417a3639: 4d 89 ee                    	mov	r14, r13
1417a363c: 49 83 c6 08                 	add	r14, 0x8
1417a3640: 4d 89 f0                    	mov	r8, r14
1417a3643: 48 c7 c0 ff ff ff ff        	mov	rax, -0x1
1417a364a: 4c 0f 42 c0                 	cmovb	r8, rax
1417a364e: 49 8b 07                    	mov	rax, qword ptr [r15]
1417a3651: 49 39 c0                    	cmp	r8, rax
1417a3654: 76 13                       	jbe	0x1417a3669 <.text+0x17a2669>
1417a3656: 49 8b 57 10                 	mov	rdx, qword ptr [r15 + 0x10]
1417a365a: 49 29 d0                    	sub	r8, rdx
1417a365d: 48 29 d0                    	sub	rax, rdx
1417a3660: 4c 39 c0                    	cmp	rax, r8
1417a3663: 0f 82 40 02 00 00           	jb	0x1417a38a9 <.text+0x17a28a9>
1417a3669: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
1417a366d: 4d 89 e8                    	mov	r8, r13
1417a3670: 49 29 c8                    	sub	r8, rcx
1417a3673: 76 0f                       	jbe	0x1417a3684 <.text+0x17a2684>
1417a3675: 49 03 4f 08                 	add	rcx, qword ptr [r15 + 0x8]
1417a3679: 31 d2                       	xor	edx, edx
1417a367b: e8 a0 05 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a3680: 4d 89 6f 10                 	mov	qword ptr [r15 + 0x10], r13
1417a3684: 49 8b 47 08                 	mov	rax, qword ptr [r15 + 0x8]
1417a3688: 4a 89 34 28                 	mov	qword ptr [rax + r13], rsi
1417a368c: 4d 3b 77 10                 	cmp	r14, qword ptr [r15 + 0x10]
1417a3690: 76 04                       	jbe	0x1417a3696 <.text+0x17a2696>
1417a3692: 4d 89 77 10                 	mov	qword ptr [r15 + 0x10], r14
1417a3696: 4c 89 73 08                 	mov	qword ptr [rbx + 0x8], r14
1417a369a: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a36a1: ba 06 00 00 00              	mov	edx, 0x6
1417a36a6: 45 31 c0                    	xor	r8d, r8d
1417a36a9: e8 92 c4 41 ff              	call	0x140bbfb40 <.text+0xbbeb40>
1417a36ae: 0f b6 05 54 15 e9 01        	movzx	eax, byte ptr [rip + 0x1e91554] # 0x143634c09
1417a36b5: b9 00 80 00 00              	mov	ecx, 0x8000
1417a36ba: ba 01 00 00 00              	mov	edx, 0x1
1417a36bf: e8 4c 05 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a36c4: 48 85 c0                    	test	rax, rax
1417a36c7: 0f 84 12 17 00 00           	je	0x1417a4ddf <.text+0x17a3ddf>
1417a36cd: 48 8d 8d 28 02 00 00        	lea	rcx, [rbp + 0x228]
1417a36d4: 48 89 8d c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rcx
1417a36db: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a36e2: 48 8d 95 a8 02 00 00        	lea	rdx, [rbp + 0x2a8]
1417a36e9: 48 89 4a 10                 	mov	qword ptr [rdx + 0x10], rcx
1417a36ed: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a36f5: f3 0f 7f 02                 	movdqu	xmmword ptr [rdx], xmm0
1417a36f9: 48 c7 85 90 02 00 00 00 80 00 00    	mov	qword ptr [rbp + 0x290], 0x8000
1417a3704: 48 89 85 98 02 00 00        	mov	qword ptr [rbp + 0x298], rax
1417a370b: 48 c7 85 a0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2a0], 0x0
1417a3716: c6 85 2f 03 00 00 01        	mov	byte ptr [rbp + 0x32f], 0x1
1417a371d: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a3724: 48 8b 95 30 03 00 00        	mov	rdx, qword ptr [rbp + 0x330]
1417a372b: 49 89 f0                    	mov	r8, rsi
1417a372e: e8 bd 0f 27 ff              	call	0x140a146f0 <.text+0xa136f0>
1417a3733: 48 85 c0                    	test	rax, rax
1417a3736: 0f 85 d4 04 00 00           	jne	0x1417a3c10 <.text+0x17a2c10>
1417a373c: 48 8b 85 c0 02 00 00        	mov	rax, qword ptr [rbp + 0x2c0]
1417a3743: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
1417a374a: f3 0f 6f 85 90 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x290]
1417a3752: 0f 10 8d a0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2a0]
1417a3759: 0f 10 95 b0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2b0]
1417a3760: 0f 29 95 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm2
1417a3767: 0f 29 8d e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm1
1417a376e: 66 0f 7f 85 d0 02 00 00     	movdqa	xmmword ptr [rbp + 0x2d0], xmm0
1417a3776: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a377d: e8 1e 1f 35 ff              	call	0x140af56a0 <.text+0xaf46a0>
1417a3782: 48 89 c6                    	mov	rsi, rax
1417a3785: 49 89 c7                    	mov	r15, rax
1417a3788: 48 85 c0                    	test	rax, rax
1417a378b: 75 1b                       	jne	0x1417a37a8 <.text+0x17a27a8>
1417a378d: 4c 8b bd 00 03 00 00        	mov	r15, qword ptr [rbp + 0x300]
1417a3794: 48 c7 85 00 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x300], 0x0
1417a379f: 4d 85 ff                    	test	r15, r15
1417a37a2: 0f 84 14 1b 00 00           	je	0x1417a52bc <.text+0x17a42bc>
1417a37a8: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1417a37af: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a37b6: e8 d5 e0 0d ff              	call	0x140881890 <.text+0x880890>
1417a37bb: 48 85 f6                    	test	rsi, rsi
1417a37be: 0f 85 f2 04 00 00           	jne	0x1417a3cb6 <.text+0x17a2cb6>
1417a37c4: 4c 8b bd 28 02 00 00        	mov	r15, qword ptr [rbp + 0x228]
1417a37cb: 4c 8b ad 38 02 00 00        	mov	r13, qword ptr [rbp + 0x238]
1417a37d2: 49 8b 37                    	mov	rsi, qword ptr [r15]
1417a37d5: 49 8b 5f 08                 	mov	rbx, qword ptr [r15 + 0x8]
1417a37d9: 4d 8d 75 08                 	lea	r14, [r13 + 0x8]
1417a37dd: 4d 89 f0                    	mov	r8, r14
1417a37e0: 49 83 c0 08                 	add	r8, 0x8
1417a37e4: 48 c7 c0 ff ff ff ff        	mov	rax, -0x1
1417a37eb: 4c 0f 42 c0                 	cmovb	r8, rax
1417a37ef: 4d 89 77 08                 	mov	qword ptr [r15 + 0x8], r14
1417a37f3: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a37f6: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a37fa: 49 39 c0                    	cmp	r8, rax
1417a37fd: 76 0f                       	jbe	0x1417a380e <.text+0x17a280e>
1417a37ff: 49 29 d0                    	sub	r8, rdx
1417a3802: 48 29 d0                    	sub	rax, rdx
1417a3805: 4c 39 c0                    	cmp	rax, r8
1417a3808: 0f 82 b7 00 00 00           	jb	0x1417a38c5 <.text+0x17a28c5>
1417a380e: 4d 89 f0                    	mov	r8, r14
1417a3811: 49 29 d0                    	sub	r8, rdx
1417a3814: 76 12                       	jbe	0x1417a3828 <.text+0x17a2828>
1417a3816: 48 03 56 08                 	add	rdx, qword ptr [rsi + 0x8]
1417a381a: 48 89 d1                    	mov	rcx, rdx
1417a381d: 31 d2                       	xor	edx, edx
1417a381f: e8 fc 03 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a3824: 4c 89 76 10                 	mov	qword ptr [rsi + 0x10], r14
1417a3828: 48 89 d8                    	mov	rax, rbx
1417a382b: 4c 29 e8                    	sub	rax, r13
1417a382e: 48 83 c0 f0                 	add	rax, -0x10
1417a3832: 48 8b 4e 08                 	mov	rcx, qword ptr [rsi + 0x8]
1417a3836: 4a 89 44 29 08              	mov	qword ptr [rcx + r13 + 0x8], rax
1417a383b: 49 83 c5 10                 	add	r13, 0x10
1417a383f: 4c 3b 6e 10                 	cmp	r13, qword ptr [rsi + 0x10]
1417a3843: 76 04                       	jbe	0x1417a3849 <.text+0x17a2849>
1417a3845: 4c 89 6e 10                 	mov	qword ptr [rsi + 0x10], r13
1417a3849: 49 89 5f 08                 	mov	qword ptr [r15 + 0x8], rbx
1417a384d: 48 8b 85 30 02 00 00        	mov	rax, qword ptr [rbp + 0x230]
1417a3854: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
1417a385b: 48 83 38 00                 	cmp	qword ptr [rax], 0x0
1417a385f: 74 0c                       	je	0x1417a386d <.text+0x17a286d>
1417a3861: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
1417a3868: e8 93 41 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a386d: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1417a3874: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a387b: 48 8b 95 40 02 00 00        	mov	rdx, qword ptr [rbp + 0x240]
1417a3882: 48 85 d2                    	test	rdx, rdx
1417a3885: 48 8b b5 20 02 00 00        	mov	rsi, qword ptr [rbp + 0x220]
1417a388c: 0f 84 fe fc ff ff           	je	0x1417a3590 <.text+0x17a2590>
1417a3892: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a3898: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a389f: e8 7c 03 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a38a4: e9 e7 fc ff ff              	jmp	0x1417a3590 <.text+0x17a2590>
1417a38a9: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a38b2: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a38b8: 4c 89 f9                    	mov	rcx, r15
1417a38bb: e8 a0 61 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a38c0: e9 a4 fd ff ff              	jmp	0x1417a3669 <.text+0x17a2669>
1417a38c5: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a38ce: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a38d4: 48 89 f1                    	mov	rcx, rsi
1417a38d7: e8 84 61 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a38dc: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a38e0: 4d 89 f0                    	mov	r8, r14
1417a38e3: 49 29 d0                    	sub	r8, rdx
1417a38e6: 0f 87 2a ff ff ff           	ja	0x1417a3816 <.text+0x17a2816>
1417a38ec: e9 37 ff ff ff              	jmp	0x1417a3828 <.text+0x17a2828>
1417a38f1: 48 8b 9d 18 02 00 00        	mov	rbx, qword ptr [rbp + 0x218]
1417a38f8: 49 8b 37                    	mov	rsi, qword ptr [r15]
1417a38fb: 4d 8b 67 08                 	mov	r12, qword ptr [r15 + 0x8]
1417a38ff: 4d 89 e6                    	mov	r14, r12
1417a3902: 49 ff c6                    	inc	r14
1417a3905: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a390c: 4d 0f 45 c6                 	cmovne	r8, r14
1417a3910: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a3913: 49 39 c0                    	cmp	r8, rax
1417a3916: 76 13                       	jbe	0x1417a392b <.text+0x17a292b>
1417a3918: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a391c: 49 29 d0                    	sub	r8, rdx
1417a391f: 48 29 d0                    	sub	rax, rdx
1417a3922: 4c 39 c0                    	cmp	rax, r8
1417a3925: 0f 82 32 14 00 00           	jb	0x1417a4d5d <.text+0x17a3d5d>
1417a392b: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1417a392f: 4d 89 e0                    	mov	r8, r12
1417a3932: 49 29 c8                    	sub	r8, rcx
1417a3935: 76 2f                       	jbe	0x1417a3966 <.text+0x17a2966>
1417a3937: 48 03 4e 08                 	add	rcx, qword ptr [rsi + 0x8]
1417a393b: 31 d2                       	xor	edx, edx
1417a393d: e8 de 02 18 01              	call	0x142923c20 <.text+0x2922c20>
1417a3942: 4c 89 66 10                 	mov	qword ptr [rsi + 0x10], r12
1417a3946: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a394a: 42 c6 04 20 7b              	mov	byte ptr [rax + r12], 0x7b
1417a394f: 4c 3b 76 10                 	cmp	r14, qword ptr [rsi + 0x10]
1417a3953: 77 20                       	ja	0x1417a3975 <.text+0x17a2975>
1417a3955: 4d 89 77 08                 	mov	qword ptr [r15 + 0x8], r14
1417a3959: b0 01                       	mov	al, 0x1
1417a395b: 48 85 db                    	test	rbx, rbx
1417a395e: 0f 85 d1 09 00 00           	jne	0x1417a4335 <.text+0x17a3335>
1417a3964: eb 22                       	jmp	0x1417a3988 <.text+0x17a2988>
1417a3966: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a396a: 42 c6 04 20 7b              	mov	byte ptr [rax + r12], 0x7b
1417a396f: 4c 3b 76 10                 	cmp	r14, qword ptr [rsi + 0x10]
1417a3973: 76 e0                       	jbe	0x1417a3955 <.text+0x17a2955>
1417a3975: 4c 89 76 10                 	mov	qword ptr [rsi + 0x10], r14
1417a3979: 4d 89 77 08                 	mov	qword ptr [r15 + 0x8], r14
1417a397d: b0 01                       	mov	al, 0x1
1417a397f: 48 85 db                    	test	rbx, rbx
1417a3982: 0f 85 ad 09 00 00           	jne	0x1417a4335 <.text+0x17a3335>
1417a3988: 48 8d 15 01 87 53 01        	lea	rdx, [rip + 0x1538701]  # 0x142cdc090
1417a398f: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a3996: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a399c: e8 4f c3 da ff              	call	0x14154fcf0 <.text+0x154ecf0>
1417a39a1: 48 85 c0                    	test	rax, rax
1417a39a4: 0f 84 89 09 00 00           	je	0x1417a4333 <.text+0x17a3333>
1417a39aa: 48 89 c1                    	mov	rcx, rax
1417a39ad: e8 ee 5e 12 01              	call	0x1428c98a0 <.text+0x28c88a0>
1417a39b2: e9 7b 06 00 00              	jmp	0x1417a4032 <.text+0x17a3032>
1417a39b7: 48 39 bd c8 01 00 00        	cmp	qword ptr [rbp + 0x1c8], rdi
1417a39be: 74 44                       	je	0x1417a3a04 <.text+0x17a2a04>
1417a39c0: 48 b9 cd cc cc cc cc cc cc cc       	movabs	rcx, -0x3333333333333333
1417a39ca: 48 89 f0                    	mov	rax, rsi
1417a39cd: 48 f7 e1                    	mul	rcx
1417a39d0: 48 89 d6                    	mov	rsi, rdx
1417a39d3: 48 c1 ee 05                 	shr	rsi, 0x5
1417a39d7: eb 10                       	jmp	0x1417a39e9 <.text+0x17a29e9>
1417a39d9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1417a39e0: 49 83 c4 28                 	add	r12, 0x28
1417a39e4: 48 ff ce                    	dec	rsi
1417a39e7: 74 1b                       	je	0x1417a3a04 <.text+0x17a2a04>
1417a39e9: 49 8b 54 24 f8              	mov	rdx, qword ptr [r12 - 0x8]
1417a39ee: 48 85 d2                    	test	rdx, rdx
1417a39f1: 74 ed                       	je	0x1417a39e0 <.text+0x17a29e0>
1417a39f3: 49 8b 0c 24                 	mov	rcx, qword ptr [r12]
1417a39f7: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a39fd: e8 1e 02 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a3a02: eb dc                       	jmp	0x1417a39e0 <.text+0x17a29e0>
1417a3a04: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
1417a3a0b: 48 85 c0                    	test	rax, rax
1417a3a0e: 74 1a                       	je	0x1417a3a2a <.text+0x17a2a2a>
1417a3a10: 48 c1 e0 03                 	shl	rax, 0x3
1417a3a14: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1417a3a18: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a3a1e: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
1417a3a25: e8 f6 01 e7 fe              	call	0x140613c20 <.text+0x612c20>
1417a3a2a: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a3a31: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a3a38: 4c 8d 05 7b 8b 53 01        	lea	r8, [rip + 0x1538b7b]   # 0x142cdc5ba
1417a3a3f: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3a46: 48 8d 55 70                 	lea	rdx, [rbp + 0x70]
1417a3a4a: 41 b9 07 00 00 00           	mov	r9d, 0x7
1417a3a50: e8 3b ec da ff              	call	0x141552690 <.text+0x1551690>
1417a3a55: 48 8b 85 d0 02 00 00        	mov	rax, qword ptr [rbp + 0x2d0]
1417a3a5c: 48 8b 8d d8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d8]
1417a3a63: 48 85 c0                    	test	rax, rax
1417a3a66: 0f 84 f0 03 00 00           	je	0x1417a3e5c <.text+0x17a2e5c>
1417a3a6c: 48 8b 95 e0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e0]
1417a3a73: 48 89 85 d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rax
1417a3a7a: 48 89 8d e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rcx
1417a3a81: 48 89 95 e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rdx
1417a3a88: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3a8f: ba 06 00 00 00              	mov	edx, 0x6
1417a3a94: 45 31 c0                    	xor	r8d, r8d
1417a3a97: e8 a4 c0 41 ff              	call	0x140bbfb40 <.text+0xbbeb40>
1417a3a9c: 48 8b bd 48 02 00 00        	mov	rdi, qword ptr [rbp + 0x248]
1417a3aa3: 0f b6 05 5f 11 e9 01        	movzx	eax, byte ptr [rip + 0x1e9115f] # 0x143634c09
1417a3aaa: b9 00 80 00 00              	mov	ecx, 0x8000
1417a3aaf: ba 01 00 00 00              	mov	edx, 0x1
1417a3ab4: e8 57 01 e7 fe              	call	0x140613c10 <.text+0x612c10>
1417a3ab9: 48 85 c0                    	test	rax, rax
1417a3abc: 4c 8b 45 38                 	mov	r8, qword ptr [rbp + 0x38]
1417a3ac0: 0f 84 e5 18 00 00           	je	0x1417a53ab <.text+0x17a43ab>
1417a3ac6: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1417a3acd: 48 89 8d c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rcx
1417a3ad4: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a3adc: f3 0f 7f 85 a8 02 00 00     	movdqu	xmmword ptr [rbp + 0x2a8], xmm0
1417a3ae4: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1417a3aeb: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1417a3af2: 48 c7 85 90 02 00 00 00 80 00 00    	mov	qword ptr [rbp + 0x290], 0x8000
1417a3afd: 48 89 85 98 02 00 00        	mov	qword ptr [rbp + 0x298], rax
1417a3b04: 48 c7 85 a0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2a0], 0x0
1417a3b0f: c6 85 2e 03 00 00 01        	mov	byte ptr [rbp + 0x32e], 0x1
1417a3b16: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a3b1d: 48 8b 95 f8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1f8]
1417a3b24: e8 c7 0b 27 ff              	call	0x140a146f0 <.text+0xa136f0>
1417a3b29: 48 89 c6                    	mov	rsi, rax
1417a3b2c: 48 85 c0                    	test	rax, rax
1417a3b2f: 0f 84 57 03 00 00           	je	0x1417a3e8c <.text+0x17a2e8c>
1417a3b35: 48 89 b5 28 02 00 00        	mov	qword ptr [rbp + 0x228], rsi
1417a3b3c: c6 85 cf 02 00 00 01        	mov	byte ptr [rbp + 0x2cf], 0x1
1417a3b43: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3b4a: e8 b1 2b 5b ff              	call	0x140d56700 <.text+0xd55700>
1417a3b4f: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a3b57: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a3b5e: 0f 10 95 f0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2f0]
1417a3b65: 0f 29 95 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm2
1417a3b6c: 0f 29 8d 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm1
1417a3b73: 66 0f 7f 85 50 02 00 00     	movdqa	xmmword ptr [rbp + 0x250], xmm0
1417a3b7b: 48 8d 05 84 8a 53 01        	lea	rax, [rip + 0x1538a84]  # 0x142cdc606
1417a3b82: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
1417a3b89: 48 c7 85 d8 02 00 00 18 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x18
1417a3b94: 48 89 b5 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rsi
1417a3b9b: c6 85 cf 02 00 00 00        	mov	byte ptr [rbp + 0x2cf], 0x0
1417a3ba2: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3ba9: 48 8d 95 50 02 00 00        	lea	rdx, [rbp + 0x250]
1417a3bb0: e8 cb 52 10 01              	call	0x1428a8e80 <.text+0x28a7e80>
1417a3bb5: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1417a3bb9: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3bc3: 48 89 07                    	mov	qword ptr [rdi], rax
1417a3bc6: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a3bcd: e8 be dc 0d ff              	call	0x140881890 <.text+0x880890>
1417a3bd2: e9 2b 03 00 00              	jmp	0x1417a3f02 <.text+0x17a2f02>
1417a3bd7: e8 b4 59 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a3bdc: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1417a3be3: 48 89 41 08                 	mov	qword ptr [rcx + 0x8], rax
1417a3be7: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3bf1: 48 89 01                    	mov	qword ptr [rcx], rax
1417a3bf4: 48 8b b5 20 02 00 00        	mov	rsi, qword ptr [rbp + 0x220]
1417a3bfb: 48 8b 95 40 02 00 00        	mov	rdx, qword ptr [rbp + 0x240]
1417a3c02: 48 85 d2                    	test	rdx, rdx
1417a3c05: 0f 85 a7 01 00 00           	jne	0x1417a3db2 <.text+0x17a2db2>
1417a3c0b: e9 b4 01 00 00              	jmp	0x1417a3dc4 <.text+0x17a2dc4>
1417a3c10: 48 89 c6                    	mov	rsi, rax
1417a3c13: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
1417a3c17: c6 85 cc 02 00 00 01        	mov	byte ptr [rbp + 0x2cc], 0x1
1417a3c1e: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3c25: e8 d6 2a 5b ff              	call	0x140d56700 <.text+0xd55700>
1417a3c2a: f3 0f 6f 85 d0 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x2d0]
1417a3c32: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1417a3c39: 0f 10 95 f0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2f0]
1417a3c40: 0f 29 95 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm2
1417a3c47: 0f 29 8d 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm1
1417a3c4e: 66 0f 7f 85 50 02 00 00     	movdqa	xmmword ptr [rbp + 0x250], xmm0
1417a3c56: 48 8d 05 c1 89 53 01        	lea	rax, [rip + 0x15389c1]  # 0x142cdc61e
1417a3c5d: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
1417a3c64: 48 c7 85 d8 02 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x17
1417a3c6f: 48 89 b5 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rsi
1417a3c76: c6 85 cc 02 00 00 00        	mov	byte ptr [rbp + 0x2cc], 0x0
1417a3c7d: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3c84: 48 8d 95 50 02 00 00        	lea	rdx, [rbp + 0x250]
1417a3c8b: e8 f0 51 10 01              	call	0x1428a8e80 <.text+0x28a7e80>
1417a3c90: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1417a3c97: 48 89 41 08                 	mov	qword ptr [rcx + 0x8], rax
1417a3c9b: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3ca5: 48 89 01                    	mov	qword ptr [rcx], rax
1417a3ca8: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a3caf: e8 dc db 0d ff              	call	0x140881890 <.text+0x880890>
1417a3cb4: eb 27                       	jmp	0x1417a3cdd <.text+0x17a2cdd>
1417a3cb6: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1417a3cbd: 4c 89 f9                    	mov	rcx, r15
1417a3cc0: e8 cb 58 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a3cc5: 48 8b 8d 48 02 00 00        	mov	rcx, qword ptr [rbp + 0x248]
1417a3ccc: 48 89 41 08                 	mov	qword ptr [rcx + 0x8], rax
1417a3cd0: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3cda: 48 89 01                    	mov	qword ptr [rcx], rax
1417a3cdd: 4c 8b b5 28 02 00 00        	mov	r14, qword ptr [rbp + 0x228]
1417a3ce4: 4c 8b bd 38 02 00 00        	mov	r15, qword ptr [rbp + 0x238]
1417a3ceb: 49 8b 36                    	mov	rsi, qword ptr [r14]
1417a3cee: 4d 8b 6e 08                 	mov	r13, qword ptr [r14 + 0x8]
1417a3cf2: 49 8d 5f 08                 	lea	rbx, [r15 + 0x8]
1417a3cf6: 48 89 d8                    	mov	rax, rbx
1417a3cf9: 48 83 c0 08                 	add	rax, 0x8
1417a3cfd: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a3d04: 4c 0f 43 c0                 	cmovae	r8, rax
1417a3d08: 49 89 5e 08                 	mov	qword ptr [r14 + 0x8], rbx
1417a3d0c: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a3d0f: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a3d13: 49 39 c0                    	cmp	r8, rax
1417a3d16: 76 0f                       	jbe	0x1417a3d27 <.text+0x17a2d27>
1417a3d18: 49 29 d0                    	sub	r8, rdx
1417a3d1b: 48 29 d0                    	sub	rax, rdx
1417a3d1e: 4c 39 c0                    	cmp	rax, r8
1417a3d21: 0f 82 de 0f 00 00           	jb	0x1417a4d05 <.text+0x17a3d05>
1417a3d27: 4c 89 ad 88 02 00 00        	mov	qword ptr [rbp + 0x288], r13
1417a3d2e: 4d 29 fd                    	sub	r13, r15
1417a3d31: 49 83 c5 f0                 	add	r13, -0x10
1417a3d35: 49 89 d8                    	mov	r8, rbx
1417a3d38: 49 29 d0                    	sub	r8, rdx
1417a3d3b: 76 12                       	jbe	0x1417a3d4f <.text+0x17a2d4f>
1417a3d3d: 48 03 56 08                 	add	rdx, qword ptr [rsi + 0x8]
1417a3d41: 48 89 d1                    	mov	rcx, rdx
1417a3d44: 31 d2                       	xor	edx, edx
1417a3d46: e8 d5 fe 17 01              	call	0x142923c20 <.text+0x2922c20>
1417a3d4b: 48 89 5e 10                 	mov	qword ptr [rsi + 0x10], rbx
1417a3d4f: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a3d53: 4e 89 6c 38 08              	mov	qword ptr [rax + r15 + 0x8], r13
1417a3d58: 49 83 c7 10                 	add	r15, 0x10
1417a3d5c: 4c 3b 7e 10                 	cmp	r15, qword ptr [rsi + 0x10]
1417a3d60: 76 04                       	jbe	0x1417a3d66 <.text+0x17a2d66>
1417a3d62: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
1417a3d66: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1417a3d6d: 49 89 46 08                 	mov	qword ptr [r14 + 0x8], rax
1417a3d71: 48 8b 85 30 02 00 00        	mov	rax, qword ptr [rbp + 0x230]
1417a3d78: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
1417a3d7f: 48 83 38 00                 	cmp	qword ptr [rax], 0x0
1417a3d83: 74 0c                       	je	0x1417a3d91 <.text+0x17a2d91>
1417a3d85: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
1417a3d8c: e8 6f 3c e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a3d91: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1417a3d98: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a3d9f: 48 8b b5 20 02 00 00        	mov	rsi, qword ptr [rbp + 0x220]
1417a3da6: 48 8b 95 40 02 00 00        	mov	rdx, qword ptr [rbp + 0x240]
1417a3dad: 48 85 d2                    	test	rdx, rdx
1417a3db0: 74 12                       	je	0x1417a3dc4 <.text+0x17a2dc4>
1417a3db2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a3db8: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a3dbf: e8 5c fe e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a3dc4: 48 39 bd c8 01 00 00        	cmp	qword ptr [rbp + 0x1c8], rdi
1417a3dcb: 74 47                       	je	0x1417a3e14 <.text+0x17a2e14>
1417a3dcd: 48 b9 cd cc cc cc cc cc cc cc       	movabs	rcx, -0x3333333333333333
1417a3dd7: 48 89 f0                    	mov	rax, rsi
1417a3dda: 48 f7 e1                    	mul	rcx
1417a3ddd: 48 89 d6                    	mov	rsi, rdx
1417a3de0: 48 c1 ee 05                 	shr	rsi, 0x5
1417a3de4: eb 13                       	jmp	0x1417a3df9 <.text+0x17a2df9>
1417a3de6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a3df0: 49 83 c4 28                 	add	r12, 0x28
1417a3df4: 48 ff ce                    	dec	rsi
1417a3df7: 74 1b                       	je	0x1417a3e14 <.text+0x17a2e14>
1417a3df9: 49 8b 54 24 f8              	mov	rdx, qword ptr [r12 - 0x8]
1417a3dfe: 48 85 d2                    	test	rdx, rdx
1417a3e01: 74 ed                       	je	0x1417a3df0 <.text+0x17a2df0>
1417a3e03: 49 8b 0c 24                 	mov	rcx, qword ptr [r12]
1417a3e07: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a3e0d: e8 0e fe e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a3e12: eb dc                       	jmp	0x1417a3df0 <.text+0x17a2df0>
1417a3e14: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
1417a3e1b: 48 85 c0                    	test	rax, rax
1417a3e1e: 74 1a                       	je	0x1417a3e3a <.text+0x17a2e3a>
1417a3e20: 48 c1 e0 03                 	shl	rax, 0x3
1417a3e24: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1417a3e28: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a3e2e: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
1417a3e35: e8 e6 fd e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a3e3a: 40 b6 01                    	mov	sil, 0x1
1417a3e3d: 45 31 ff                    	xor	r15d, r15d
1417a3e40: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a3e47: 0f b6 85 00 02 00 00        	movzx	eax, byte ptr [rbp + 0x200]
1417a3e4e: 83 f8 03                    	cmp	eax, 0x3
1417a3e51: 0f 83 c8 06 00 00           	jae	0x1417a451f <.text+0x17a351f>
1417a3e57: e9 5e 07 00 00              	jmp	0x1417a45ba <.text+0x17a35ba>
1417a3e5c: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a3e63: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a3e6a: e8 21 57 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a3e6f: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a3e76: 49 89 46 08                 	mov	qword ptr [r14 + 0x8], rax
1417a3e7a: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3e84: 49 89 06                    	mov	qword ptr [r14], rax
1417a3e87: e9 34 01 00 00              	jmp	0x1417a3fc0 <.text+0x17a2fc0>
1417a3e8c: 48 8b 85 c0 02 00 00        	mov	rax, qword ptr [rbp + 0x2c0]
1417a3e93: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
1417a3e9a: f3 0f 6f 85 90 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x290]
1417a3ea2: 0f 10 8d a0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2a0]
1417a3ea9: 0f 10 95 b0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2b0]
1417a3eb0: 0f 29 95 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm2
1417a3eb7: 0f 29 8d e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm1
1417a3ebe: 66 0f 7f 85 d0 02 00 00     	movdqa	xmmword ptr [rbp + 0x2d0], xmm0
1417a3ec6: c6 85 2e 03 00 00 00        	mov	byte ptr [rbp + 0x32e], 0x0
1417a3ecd: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a3ed4: e8 47 13 27 ff              	call	0x140a15220 <.text+0xa14220>
1417a3ed9: 48 85 c0                    	test	rax, rax
1417a3edc: 0f 84 9c 0a 00 00           	je	0x1417a497e <.text+0x17a397e>
1417a3ee2: c6 85 2e 03 00 00 00        	mov	byte ptr [rbp + 0x32e], 0x0
1417a3ee9: 48 89 d1                    	mov	rcx, rdx
1417a3eec: e8 9f 56 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a3ef1: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1417a3ef5: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a3eff: 48 89 07                    	mov	qword ptr [rdi], rax
1417a3f02: 48 8b bd d8 01 00 00        	mov	rdi, qword ptr [rbp + 0x1d8]
1417a3f09: 4c 8b b5 e8 01 00 00        	mov	r14, qword ptr [rbp + 0x1e8]
1417a3f10: 48 8b 37                    	mov	rsi, qword ptr [rdi]
1417a3f13: 48 8b 5f 08                 	mov	rbx, qword ptr [rdi + 0x8]
1417a3f17: 4d 8d 7e 08                 	lea	r15, [r14 + 0x8]
1417a3f1b: 4c 89 f8                    	mov	rax, r15
1417a3f1e: 48 83 c0 08                 	add	rax, 0x8
1417a3f22: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a3f29: 4c 0f 43 c0                 	cmovae	r8, rax
1417a3f2d: 4c 89 7f 08                 	mov	qword ptr [rdi + 0x8], r15
1417a3f31: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a3f34: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a3f38: 49 39 c0                    	cmp	r8, rax
1417a3f3b: 76 0f                       	jbe	0x1417a3f4c <.text+0x17a2f4c>
1417a3f3d: 49 29 d0                    	sub	r8, rdx
1417a3f40: 48 29 d0                    	sub	rax, rdx
1417a3f43: 4c 39 c0                    	cmp	rax, r8
1417a3f46: 0f 82 2d 0e 00 00           	jb	0x1417a4d79 <.text+0x17a3d79>
1417a3f4c: 49 89 dc                    	mov	r12, rbx
1417a3f4f: 4d 29 f4                    	sub	r12, r14
1417a3f52: 49 83 c4 f0                 	add	r12, -0x10
1417a3f56: 4d 89 f8                    	mov	r8, r15
1417a3f59: 49 29 d0                    	sub	r8, rdx
1417a3f5c: 76 12                       	jbe	0x1417a3f70 <.text+0x17a2f70>
1417a3f5e: 48 03 56 08                 	add	rdx, qword ptr [rsi + 0x8]
1417a3f62: 48 89 d1                    	mov	rcx, rdx
1417a3f65: 31 d2                       	xor	edx, edx
1417a3f67: e8 b4 fc 17 01              	call	0x142923c20 <.text+0x2922c20>
1417a3f6c: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
1417a3f70: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a3f74: 4e 89 64 30 08              	mov	qword ptr [rax + r14 + 0x8], r12
1417a3f79: 49 83 c6 10                 	add	r14, 0x10
1417a3f7d: 4c 3b 76 10                 	cmp	r14, qword ptr [rsi + 0x10]
1417a3f81: 76 04                       	jbe	0x1417a3f87 <.text+0x17a2f87>
1417a3f83: 4c 89 76 10                 	mov	qword ptr [rsi + 0x10], r14
1417a3f87: 48 89 5f 08                 	mov	qword ptr [rdi + 0x8], rbx
1417a3f8b: 48 8b 85 e0 01 00 00        	mov	rax, qword ptr [rbp + 0x1e0]
1417a3f92: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
1417a3f99: 48 83 38 00                 	cmp	qword ptr [rax], 0x0
1417a3f9d: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a3fa4: 74 0c                       	je	0x1417a3fb2 <.text+0x17a2fb2>
1417a3fa6: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a3fad: e8 4e 3a e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a3fb2: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a3fb9: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a3fc0: 40 b6 01                    	mov	sil, 0x1
1417a3fc3: 45 31 ff                    	xor	r15d, r15d
1417a3fc6: 0f b6 85 00 02 00 00        	movzx	eax, byte ptr [rbp + 0x200]
1417a3fcd: 83 f8 03                    	cmp	eax, 0x3
1417a3fd0: 0f 83 49 05 00 00           	jae	0x1417a451f <.text+0x17a351f>
1417a3fd6: e9 df 05 00 00              	jmp	0x1417a45ba <.text+0x17a35ba>
1417a3fdb: f3 0f 7e 85 10 02 00 00     	movq	xmm0, qword ptr [rbp + 0x210]
1417a3fe3: 66 48 0f 7e c0              	movq	rax, xmm0
1417a3fe8: 48 b9 ff ff ff ff ff ff ff 7f       	movabs	rcx, 0x7fffffffffffffff
1417a3ff2: 48 21 c1                    	and	rcx, rax
1417a3ff5: 48 b8 00 00 00 00 00 00 f0 7f       	movabs	rax, 0x7ff0000000000000
1417a3fff: 48 39 c1                    	cmp	rcx, rax
1417a4002: 0f 8c a3 01 00 00           	jl	0x1417a41ab <.text+0x17a31ab>
1417a4008: 48 8d 15 8b 80 53 01        	lea	rdx, [rip + 0x153808b]  # 0x142cdc09a
1417a400f: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a4016: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a401c: e8 cf bc da ff              	call	0x14154fcf0 <.text+0x154ecf0>
1417a4021: 48 85 c0                    	test	rax, rax
1417a4024: 0f 84 47 f4 ff ff           	je	0x1417a3471 <.text+0x17a2471>
1417a402a: 48 89 c1                    	mov	rcx, rax
1417a402d: e8 6e 58 12 01              	call	0x1428c98a0 <.text+0x28c88a0>
1417a4032: 48 89 c7                    	mov	rdi, rax
1417a4035: e9 ea 03 00 00              	jmp	0x1417a4424 <.text+0x17a3424>
1417a403a: 48 8b 8d 10 02 00 00        	mov	rcx, qword ptr [rbp + 0x210]
1417a4041: 41 be 14 00 00 00           	mov	r14d, 0x14
1417a4047: 48 81 f9 10 27 00 00        	cmp	rcx, 0x2710
1417a404e: 0f 82 ed 01 00 00           	jb	0x1417a4241 <.text+0x17a3241>
1417a4054: 41 ba 14 00 00 00           	mov	r10d, 0x14
1417a405a: 49 b8 4b 59 86 38 d6 c5 6d 34       	movabs	r8, 0x346dc5d63886594b
1417a4064: 4c 8d 0d a0 81 53 01        	lea	r9, [rip + 0x15381a0]   # 0x142cdc20b
1417a406b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1417a4070: 48 89 c8                    	mov	rax, rcx
1417a4073: 49 f7 e0                    	mul	r8
1417a4076: 48 c1 ea 0b                 	shr	rdx, 0xb
1417a407a: 69 c2 10 27 00 00           	imul	eax, edx, 0x2710
1417a4080: 41 89 cb                    	mov	r11d, ecx
1417a4083: 41 29 c3                    	sub	r11d, eax
1417a4086: 41 69 c3 7b 14 00 00        	imul	eax, r11d, 0x147b
1417a408d: c1 e8 13                    	shr	eax, 0x13
1417a4090: 6b f0 64                    	imul	esi, eax, 0x64
1417a4093: 41 29 f3                    	sub	r11d, esi
1417a4096: 45 0f b7 db                 	movzx	r11d, r11w
1417a409a: 4d 8d 72 fc                 	lea	r14, [r10 - 0x4]
1417a409e: 41 0f b7 04 41              	movzx	eax, word ptr [r9 + 2*rax]
1417a40a3: 66 42 89 84 15 cc 02 00 00  	mov	word ptr [rbp + r10 + 0x2cc], ax
1417a40ac: 43 0f b7 04 59              	movzx	eax, word ptr [r9 + 2*r11]
1417a40b1: 66 42 89 84 15 ce 02 00 00  	mov	word ptr [rbp + r10 + 0x2ce], ax
1417a40ba: 4d 89 f2                    	mov	r10, r14
1417a40bd: 48 81 f9 ff e0 f5 05        	cmp	rcx, 0x5f5e0ff
1417a40c4: 48 89 d1                    	mov	rcx, rdx
1417a40c7: 77 a7                       	ja	0x1417a4070 <.text+0x17a3070>
1417a40c9: e9 76 01 00 00              	jmp	0x1417a4244 <.text+0x17a3244>
1417a40ce: 48 89 ca                    	mov	rdx, rcx
1417a40d1: 48 83 fa 63                 	cmp	rdx, 0x63
1417a40d5: 76 32                       	jbe	0x1417a4109 <.text+0x17a3109>
1417a40d7: 0f b7 c2                    	movzx	eax, dx
1417a40da: c1 e8 02                    	shr	eax, 0x2
1417a40dd: 69 c0 7b 14 00 00           	imul	eax, eax, 0x147b
1417a40e3: c1 e8 11                    	shr	eax, 0x11
1417a40e6: 6b c8 64                    	imul	ecx, eax, 0x64
1417a40e9: 29 ca                       	sub	edx, ecx
1417a40eb: 0f b7 ca                    	movzx	ecx, dx
1417a40ee: 48 8d 15 16 81 53 01        	lea	rdx, [rip + 0x1538116]  # 0x142cdc20b
1417a40f5: 0f b7 0c 4a                 	movzx	ecx, word ptr [rdx + 2*rcx]
1417a40f9: 66 42 89 8c 35 ce 02 00 00  	mov	word ptr [rbp + r14 + 0x2ce], cx
1417a4102: 49 83 c6 fe                 	add	r14, -0x2
1417a4106: 48 89 c2                    	mov	rdx, rax
1417a4109: 48 83 fa 0a                 	cmp	rdx, 0xa
1417a410d: 73 10                       	jae	0x1417a411f <.text+0x17a311f>
1417a410f: 80 ca 30                    	or	dl, 0x30
1417a4112: 42 88 94 35 cf 02 00 00     	mov	byte ptr [rbp + r14 + 0x2cf], dl
1417a411a: 49 ff ce                    	dec	r14
1417a411d: eb 18                       	jmp	0x1417a4137 <.text+0x17a3137>
1417a411f: 48 8d 05 e5 80 53 01        	lea	rax, [rip + 0x15380e5]  # 0x142cdc20b
1417a4126: 0f b7 04 50                 	movzx	eax, word ptr [rax + 2*rdx]
1417a412a: 66 42 89 84 35 ce 02 00 00  	mov	word ptr [rbp + r14 + 0x2ce], ax
1417a4133: 49 83 c6 fe                 	add	r14, -0x2
1417a4137: 4d 85 c0                    	test	r8, r8
1417a413a: 79 0c                       	jns	0x1417a4148 <.text+0x17a3148>
1417a413c: 42 c6 84 35 cf 02 00 00 2d  	mov	byte ptr [rbp + r14 + 0x2cf], 0x2d
1417a4145: 49 ff ce                    	dec	r14
1417a4148: 4c 89 f6                    	mov	rsi, r14
1417a414b: 48 83 ee 14                 	sub	rsi, 0x14
1417a414f: 0f 84 1c f3 ff ff           	je	0x1417a3471 <.text+0x17a2471>
1417a4155: 48 f7 de                    	neg	rsi
1417a4158: 49 8b 3f                    	mov	rdi, qword ptr [r15]
1417a415b: 49 8b 5f 08                 	mov	rbx, qword ptr [r15 + 0x8]
1417a415f: 49 89 dc                    	mov	r12, rbx
1417a4162: 49 01 f4                    	add	r12, rsi
1417a4165: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a416c: 4d 0f 43 c4                 	cmovae	r8, r12
1417a4170: 48 8b 07                    	mov	rax, qword ptr [rdi]
1417a4173: 49 39 c0                    	cmp	r8, rax
1417a4176: 0f 86 71 01 00 00           	jbe	0x1417a42ed <.text+0x17a32ed>
1417a417c: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1417a4180: 49 29 d0                    	sub	r8, rdx
1417a4183: 48 29 d0                    	sub	rax, rdx
1417a4186: 4c 39 c0                    	cmp	rax, r8
1417a4189: 0f 83 5e 01 00 00           	jae	0x1417a42ed <.text+0x17a32ed>
1417a418f: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4198: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a419e: 48 89 f9                    	mov	rcx, rdi
1417a41a1: e8 ba 58 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a41a6: e9 42 01 00 00              	jmp	0x1417a42ed <.text+0x17a32ed>
1417a41ab: 48 8d 95 d0 02 00 00        	lea	rdx, [rbp + 0x2d0]
1417a41b2: e8 f9 89 3e ff              	call	0x140b8cbb0 <.text+0xb8bbb0>
1417a41b7: 48 89 c6                    	mov	rsi, rax
1417a41ba: 48 85 c0                    	test	rax, rax
1417a41bd: 0f 84 ae f2 ff ff           	je	0x1417a3471 <.text+0x17a2471>
1417a41c3: 4c 8b b5 90 02 00 00        	mov	r14, qword ptr [rbp + 0x290]
1417a41ca: 49 8b 3e                    	mov	rdi, qword ptr [r14]
1417a41cd: 49 8b 5e 08                 	mov	rbx, qword ptr [r14 + 0x8]
1417a41d1: 49 89 df                    	mov	r15, rbx
1417a41d4: 49 01 f7                    	add	r15, rsi
1417a41d7: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a41de: 4d 0f 43 c7                 	cmovae	r8, r15
1417a41e2: 48 8b 07                    	mov	rax, qword ptr [rdi]
1417a41e5: 49 39 c0                    	cmp	r8, rax
1417a41e8: 76 13                       	jbe	0x1417a41fd <.text+0x17a31fd>
1417a41ea: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1417a41ee: 49 29 d0                    	sub	r8, rdx
1417a41f1: 48 29 d0                    	sub	rax, rdx
1417a41f4: 4c 39 c0                    	cmp	rax, r8
1417a41f7: 0f 82 c6 0b 00 00           	jb	0x1417a4dc3 <.text+0x17a3dc3>
1417a41fd: 48 8b 4f 10                 	mov	rcx, qword ptr [rdi + 0x10]
1417a4201: 49 89 d8                    	mov	r8, rbx
1417a4204: 49 29 c8                    	sub	r8, rcx
1417a4207: 76 0f                       	jbe	0x1417a4218 <.text+0x17a3218>
1417a4209: 48 03 4f 08                 	add	rcx, qword ptr [rdi + 0x8]
1417a420d: 31 d2                       	xor	edx, edx
1417a420f: e8 0c fa 17 01              	call	0x142923c20 <.text+0x2922c20>
1417a4214: 48 89 5f 10                 	mov	qword ptr [rdi + 0x10], rbx
1417a4218: 48 03 5f 08                 	add	rbx, qword ptr [rdi + 0x8]
1417a421c: 48 8d 95 d0 02 00 00        	lea	rdx, [rbp + 0x2d0]
1417a4223: 48 89 d9                    	mov	rcx, rbx
1417a4226: 49 89 f0                    	mov	r8, rsi
1417a4229: e8 d2 f9 17 01              	call	0x142923c00 <.text+0x2922c00>
1417a422e: 4c 3b 7f 10                 	cmp	r15, qword ptr [rdi + 0x10]
1417a4232: 76 04                       	jbe	0x1417a4238 <.text+0x17a3238>
1417a4234: 4c 89 7f 10                 	mov	qword ptr [rdi + 0x10], r15
1417a4238: 4d 89 7e 08                 	mov	qword ptr [r14 + 0x8], r15
1417a423c: e9 30 f2 ff ff              	jmp	0x1417a3471 <.text+0x17a2471>
1417a4241: 48 89 ca                    	mov	rdx, rcx
1417a4244: 48 83 fa 63                 	cmp	rdx, 0x63
1417a4248: 76 32                       	jbe	0x1417a427c <.text+0x17a327c>
1417a424a: 0f b7 c2                    	movzx	eax, dx
1417a424d: c1 e8 02                    	shr	eax, 0x2
1417a4250: 69 c0 7b 14 00 00           	imul	eax, eax, 0x147b
1417a4256: c1 e8 11                    	shr	eax, 0x11
1417a4259: 6b c8 64                    	imul	ecx, eax, 0x64
1417a425c: 29 ca                       	sub	edx, ecx
1417a425e: 0f b7 ca                    	movzx	ecx, dx
1417a4261: 48 8d 15 a3 7f 53 01        	lea	rdx, [rip + 0x1537fa3]  # 0x142cdc20b
1417a4268: 0f b7 0c 4a                 	movzx	ecx, word ptr [rdx + 2*rcx]
1417a426c: 66 42 89 8c 35 ce 02 00 00  	mov	word ptr [rbp + r14 + 0x2ce], cx
1417a4275: 49 83 c6 fe                 	add	r14, -0x2
1417a4279: 48 89 c2                    	mov	rdx, rax
1417a427c: 48 83 fa 0a                 	cmp	rdx, 0xa
1417a4280: 73 10                       	jae	0x1417a4292 <.text+0x17a3292>
1417a4282: 80 ca 30                    	or	dl, 0x30
1417a4285: 42 88 94 35 cf 02 00 00     	mov	byte ptr [rbp + r14 + 0x2cf], dl
1417a428d: 49 ff ce                    	dec	r14
1417a4290: eb 18                       	jmp	0x1417a42aa <.text+0x17a32aa>
1417a4292: 48 8d 05 72 7f 53 01        	lea	rax, [rip + 0x1537f72]  # 0x142cdc20b
1417a4299: 0f b7 04 50                 	movzx	eax, word ptr [rax + 2*rdx]
1417a429d: 66 42 89 84 35 ce 02 00 00  	mov	word ptr [rbp + r14 + 0x2ce], ax
1417a42a6: 49 83 c6 fe                 	add	r14, -0x2
1417a42aa: 4c 89 f6                    	mov	rsi, r14
1417a42ad: 48 83 ee 14                 	sub	rsi, 0x14
1417a42b1: 0f 84 ba f1 ff ff           	je	0x1417a3471 <.text+0x17a2471>
1417a42b7: 48 f7 de                    	neg	rsi
1417a42ba: 49 8b 3f                    	mov	rdi, qword ptr [r15]
1417a42bd: 49 8b 5f 08                 	mov	rbx, qword ptr [r15 + 0x8]
1417a42c1: 49 89 dc                    	mov	r12, rbx
1417a42c4: 49 01 f4                    	add	r12, rsi
1417a42c7: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a42ce: 4d 0f 43 c4                 	cmovae	r8, r12
1417a42d2: 48 8b 07                    	mov	rax, qword ptr [rdi]
1417a42d5: 49 39 c0                    	cmp	r8, rax
1417a42d8: 76 13                       	jbe	0x1417a42ed <.text+0x17a32ed>
1417a42da: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1417a42de: 49 29 d0                    	sub	r8, rdx
1417a42e1: 48 29 d0                    	sub	rax, rdx
1417a42e4: 4c 39 c0                    	cmp	rax, r8
1417a42e7: 0f 82 ba 0a 00 00           	jb	0x1417a4da7 <.text+0x17a3da7>
1417a42ed: 49 01 ee                    	add	r14, rbp
1417a42f0: 49 81 c6 d0 02 00 00        	add	r14, 0x2d0
1417a42f7: 48 8b 4f 10                 	mov	rcx, qword ptr [rdi + 0x10]
1417a42fb: 49 89 d8                    	mov	r8, rbx
1417a42fe: 49 29 c8                    	sub	r8, rcx
1417a4301: 76 0f                       	jbe	0x1417a4312 <.text+0x17a3312>
1417a4303: 48 03 4f 08                 	add	rcx, qword ptr [rdi + 0x8]
1417a4307: 31 d2                       	xor	edx, edx
1417a4309: e8 12 f9 17 01              	call	0x142923c20 <.text+0x2922c20>
1417a430e: 48 89 5f 10                 	mov	qword ptr [rdi + 0x10], rbx
1417a4312: 48 03 5f 08                 	add	rbx, qword ptr [rdi + 0x8]
1417a4316: 48 89 d9                    	mov	rcx, rbx
1417a4319: 4c 89 f2                    	mov	rdx, r14
1417a431c: 49 89 f0                    	mov	r8, rsi
1417a431f: e8 dc f8 17 01              	call	0x142923c00 <.text+0x2922c00>
1417a4324: 4c 3b 67 10                 	cmp	r12, qword ptr [rdi + 0x10]
1417a4328: 0f 87 3b f1 ff ff           	ja	0x1417a3469 <.text+0x17a2469>
1417a432e: e9 3a f1 ff ff              	jmp	0x1417a346d <.text+0x17a246d>
1417a4333: 31 c0                       	xor	eax, eax
1417a4335: 48 89 bd 50 02 00 00        	mov	qword ptr [rbp + 0x250], rdi
1417a433c: 88 85 58 02 00 00           	mov	byte ptr [rbp + 0x258], al
1417a4342: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
1417a4349: 48 8b 8d 10 02 00 00        	mov	rcx, qword ptr [rbp + 0x210]
1417a4350: 31 d2                       	xor	edx, edx
1417a4352: 48 85 c0                    	test	rax, rax
1417a4355: 48 0f 44 d8                 	cmove	rbx, rax
1417a4359: 0f 95 c2                    	setne	dl
1417a435c: 48 89 95 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rdx
1417a4363: 48 c7 85 d8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x0
1417a436e: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
1417a4375: 48 89 8d e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rcx
1417a437c: 48 89 95 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rdx
1417a4383: 48 c7 85 f8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x0
1417a438e: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
1417a4395: 48 89 8d 08 03 00 00        	mov	qword ptr [rbp + 0x308], rcx
1417a439c: 48 89 9d 10 03 00 00        	mov	qword ptr [rbp + 0x310], rbx
1417a43a3: 48 8d b5 d0 02 00 00        	lea	rsi, [rbp + 0x2d0]
1417a43aa: 48 8d 9d 50 02 00 00        	lea	rbx, [rbp + 0x250]
1417a43b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1417a43c0: 48 89 f1                    	mov	rcx, rsi
1417a43c3: e8 08 54 c9 ff              	call	0x1414397d0 <.text+0x14387d0>
1417a43c8: 48 85 c0                    	test	rax, rax
1417a43cb: 74 18                       	je	0x1417a43e5 <.text+0x17a33e5>
1417a43cd: 49 89 d0                    	mov	r8, rdx
1417a43d0: 48 89 d9                    	mov	rcx, rbx
1417a43d3: 48 89 c2                    	mov	rdx, rax
1417a43d6: e8 e5 6d de ff              	call	0x14158b1c0 <.text+0x158a1c0>
1417a43db: 48 89 c7                    	mov	rdi, rax
1417a43de: 48 85 c0                    	test	rax, rax
1417a43e1: 74 dd                       	je	0x1417a43c0 <.text+0x17a33c0>
1417a43e3: eb 3f                       	jmp	0x1417a4424 <.text+0x17a3424>
1417a43e5: 80 bd 58 02 00 00 00        	cmp	byte ptr [rbp + 0x258], 0x0
1417a43ec: 74 2b                       	je	0x1417a4419 <.text+0x17a3419>
1417a43ee: 48 8b 8d 50 02 00 00        	mov	rcx, qword ptr [rbp + 0x250]
1417a43f5: 48 8d 15 94 7c 53 01        	lea	rdx, [rip + 0x1537c94]  # 0x142cdc090
1417a43fc: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4402: e8 e9 b8 da ff              	call	0x14154fcf0 <.text+0x154ecf0>
1417a4407: 48 85 c0                    	test	rax, rax
1417a440a: 74 0d                       	je	0x1417a4419 <.text+0x17a3419>
1417a440c: 48 89 c1                    	mov	rcx, rax
1417a440f: e8 8c 54 12 01              	call	0x1428c98a0 <.text+0x28c88a0>
1417a4414: e9 c8 ee ff ff              	jmp	0x1417a32e1 <.text+0x17a22e1>
1417a4419: 31 ff                       	xor	edi, edi
1417a441b: 48 85 ff                    	test	rdi, rdi
1417a441e: 0f 84 4d f0 ff ff           	je	0x1417a3471 <.text+0x17a2471>
1417a4424: 48 8b 9d 90 02 00 00        	mov	rbx, qword ptr [rbp + 0x290]
1417a442b: 4c 8b bd a0 02 00 00        	mov	r15, qword ptr [rbp + 0x2a0]
1417a4432: 48 8b 33                    	mov	rsi, qword ptr [rbx]
1417a4435: 4c 8b 73 08                 	mov	r14, qword ptr [rbx + 0x8]
1417a4439: 4d 8d 67 08                 	lea	r12, [r15 + 0x8]
1417a443d: 4c 89 e0                    	mov	rax, r12
1417a4440: 48 83 c0 08                 	add	rax, 0x8
1417a4444: 49 c7 c0 ff ff ff ff        	mov	r8, -0x1
1417a444b: 4c 0f 43 c0                 	cmovae	r8, rax
1417a444f: 4c 89 63 08                 	mov	qword ptr [rbx + 0x8], r12
1417a4453: 48 8b 06                    	mov	rax, qword ptr [rsi]
1417a4456: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a445a: 49 39 c0                    	cmp	r8, rax
1417a445d: 76 0f                       	jbe	0x1417a446e <.text+0x17a346e>
1417a445f: 49 29 d0                    	sub	r8, rdx
1417a4462: 48 29 d0                    	sub	rax, rdx
1417a4465: 4c 39 c0                    	cmp	rax, r8
1417a4468: 0f 82 69 08 00 00           	jb	0x1417a4cd7 <.text+0x17a3cd7>
1417a446e: 4d 89 f5                    	mov	r13, r14
1417a4471: 4d 29 fd                    	sub	r13, r15
1417a4474: 49 83 c5 f0                 	add	r13, -0x10
1417a4478: 4d 89 e0                    	mov	r8, r12
1417a447b: 49 29 d0                    	sub	r8, rdx
1417a447e: 76 12                       	jbe	0x1417a4492 <.text+0x17a3492>
1417a4480: 48 03 56 08                 	add	rdx, qword ptr [rsi + 0x8]
1417a4484: 48 89 d1                    	mov	rcx, rdx
1417a4487: 31 d2                       	xor	edx, edx
1417a4489: e8 92 f7 17 01              	call	0x142923c20 <.text+0x2922c20>
1417a448e: 4c 89 66 10                 	mov	qword ptr [rsi + 0x10], r12
1417a4492: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1417a4496: 4e 89 6c 38 08              	mov	qword ptr [rax + r15 + 0x8], r13
1417a449b: 49 83 c7 10                 	add	r15, 0x10
1417a449f: 4c 3b 7e 10                 	cmp	r15, qword ptr [rsi + 0x10]
1417a44a3: 76 04                       	jbe	0x1417a44a9 <.text+0x17a34a9>
1417a44a5: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
1417a44a9: 4c 89 73 08                 	mov	qword ptr [rbx + 0x8], r14
1417a44ad: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1417a44b4: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
1417a44bb: 48 83 38 00                 	cmp	qword ptr [rax], 0x0
1417a44bf: 74 0c                       	je	0x1417a44cd <.text+0x17a34cd>
1417a44c1: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a44c8: e8 33 35 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a44cd: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a44d4: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a44db: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a44e2: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a44e9: 48 89 f9                    	mov	rcx, rdi
1417a44ec: e8 5f 53 10 01              	call	0x1428a9850 <.text+0x28a8850>
1417a44f1: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a44f8: 49 89 46 08                 	mov	qword ptr [r14 + 0x8], rax
1417a44fc: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a4506: 49 89 06                    	mov	qword ptr [r14], rax
1417a4509: 41 b7 01                    	mov	r15b, 0x1
1417a450c: 40 b6 01                    	mov	sil, 0x1
1417a450f: 0f b6 85 00 02 00 00        	movzx	eax, byte ptr [rbp + 0x200]
1417a4516: 83 f8 03                    	cmp	eax, 0x3
1417a4519: 0f 82 9b 00 00 00           	jb	0x1417a45ba <.text+0x17a35ba>
1417a451f: 74 58                       	je	0x1417a4579 <.text+0x17a3579>
1417a4521: 83 f8 04                    	cmp	eax, 0x4
1417a4524: 75 73                       	jne	0x1417a4599 <.text+0x17a3599>
1417a4526: 89 b5 88 02 00 00           	mov	dword ptr [rbp + 0x288], esi
1417a452c: 44 89 bd 30 03 00 00        	mov	dword ptr [rbp + 0x330], r15d
1417a4533: 48 8b 8d 10 02 00 00        	mov	rcx, qword ptr [rbp + 0x210]
1417a453a: 48 8b 95 18 02 00 00        	mov	rdx, qword ptr [rbp + 0x218]
1417a4541: 48 89 8d 40 02 00 00        	mov	qword ptr [rbp + 0x240], rcx
1417a4548: e8 23 48 0e ff              	call	0x140888d70 <.text+0x887d70>
1417a454d: 48 8b 95 08 02 00 00        	mov	rdx, qword ptr [rbp + 0x208]
1417a4554: 48 85 d2                    	test	rdx, rdx
1417a4557: 44 8b bd 30 03 00 00        	mov	r15d, dword ptr [rbp + 0x330]
1417a455e: 8b b5 88 02 00 00           	mov	esi, dword ptr [rbp + 0x288]
1417a4564: 74 54                       	je	0x1417a45ba <.text+0x17a35ba>
1417a4566: 48 c1 e2 05                 	shl	rdx, 0x5
1417a456a: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4570: 48 8b 8d 40 02 00 00        	mov	rcx, qword ptr [rbp + 0x240]
1417a4577: eb 19                       	jmp	0x1417a4592 <.text+0x17a3592>
1417a4579: 48 8b 95 08 02 00 00        	mov	rdx, qword ptr [rbp + 0x208]
1417a4580: 48 85 d2                    	test	rdx, rdx
1417a4583: 74 35                       	je	0x1417a45ba <.text+0x17a35ba>
1417a4585: 48 8b 8d 10 02 00 00        	mov	rcx, qword ptr [rbp + 0x210]
1417a458c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4592: e8 89 f6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4597: eb 21                       	jmp	0x1417a45ba <.text+0x17a35ba>
1417a4599: 44 88 bd 3d 03 00 00        	mov	byte ptr [rbp + 0x33d], r15b
1417a45a0: 40 88 b5 3c 03 00 00        	mov	byte ptr [rbp + 0x33c], sil
1417a45a7: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a45ae: 48 8d 8d 08 02 00 00        	lea	rcx, [rbp + 0x208]
1417a45b5: e8 86 49 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a45ba: 40 84 f6                    	test	sil, sil
1417a45bd: 0f 84 88 00 00 00           	je	0x1417a464b <.text+0x17a364b>
1417a45c3: 48 8b 85 80 00 00 00        	mov	rax, qword ptr [rbp + 0x80]
1417a45ca: 89 c1                       	mov	ecx, eax
1417a45cc: 83 e1 03                    	and	ecx, 0x3
1417a45cf: 83 f9 01                    	cmp	ecx, 0x1
1417a45d2: 75 77                       	jne	0x1417a464b <.text+0x17a364b>
1417a45d4: 44 89 bd 30 03 00 00        	mov	dword ptr [rbp + 0x330], r15d
1417a45db: 48 8d 48 ff                 	lea	rcx, [rax - 0x1]
1417a45df: 48 89 8d 20 02 00 00        	mov	qword ptr [rbp + 0x220], rcx
1417a45e6: 48 8b 48 ff                 	mov	rcx, qword ptr [rax - 0x1]
1417a45ea: 48 89 8d 88 02 00 00        	mov	qword ptr [rbp + 0x288], rcx
1417a45f1: 48 8b 40 07                 	mov	rax, qword ptr [rax + 0x7]
1417a45f5: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
1417a45fc: 48 8b 00                    	mov	rax, qword ptr [rax]
1417a45ff: 48 85 c0                    	test	rax, rax
1417a4602: 74 09                       	je	0x1417a460d <.text+0x17a360d>
1417a4604: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
1417a460b: ff d0                       	call	rax
1417a460d: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
1417a4614: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1417a461b: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1417a461f: 48 85 d2                    	test	rdx, rdx
1417a4622: 74 09                       	je	0x1417a462d <.text+0x17a362d>
1417a4624: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1417a4628: e8 f3 f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a462d: ba 18 00 00 00              	mov	edx, 0x18
1417a4632: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4638: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
1417a463f: e8 dc f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4644: 44 8b bd 30 03 00 00        	mov	r15d, dword ptr [rbp + 0x330]
1417a464b: 48 8b 95 f8 00 00 00        	mov	rdx, qword ptr [rbp + 0xf8]
1417a4652: 48 85 d2                    	test	rdx, rdx
1417a4655: 74 12                       	je	0x1417a4669 <.text+0x17a3669>
1417a4657: 48 8b 8d 00 01 00 00        	mov	rcx, qword ptr [rbp + 0x100]
1417a465e: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4664: e8 b7 f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4669: 48 8b 55 b8                 	mov	rdx, qword ptr [rbp - 0x48]
1417a466d: 48 85 d2                    	test	rdx, rdx
1417a4670: 74 27                       	je	0x1417a4699 <.text+0x17a3699>
1417a4672: 48 8d 04 d5 17 00 00 00     	lea	rax, [8*rdx + 0x17]
1417a467a: 48 83 e0 f0                 	and	rax, -0x10
1417a467e: 48 01 c2                    	add	rdx, rax
1417a4681: 48 83 c2 11                 	add	rdx, 0x11
1417a4685: 74 12                       	je	0x1417a4699 <.text+0x17a3699>
1417a4687: 48 8b 4d b0                 	mov	rcx, qword ptr [rbp - 0x50]
1417a468b: 48 29 c1                    	sub	rcx, rax
1417a468e: 41 b8 10 00 00 00           	mov	r8d, 0x10
1417a4694: e8 87 f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4699: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1417a469d: 48 85 d2                    	test	rdx, rdx
1417a46a0: 74 13                       	je	0x1417a46b5 <.text+0x17a36b5>
1417a46a2: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1417a46a6: 48 c1 e2 03                 	shl	rdx, 0x3
1417a46aa: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a46b0: e8 6b f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a46b5: 48 8b b5 18 01 00 00        	mov	rsi, qword ptr [rbp + 0x118]
1417a46bc: 48 8b bd 20 01 00 00        	mov	rdi, qword ptr [rbp + 0x120]
1417a46c3: 48 85 ff                    	test	rdi, rdi
1417a46c6: 74 2a                       	je	0x1417a46f2 <.text+0x17a36f2>
1417a46c8: 48 8d 5e 10                 	lea	rbx, [rsi + 0x10]
1417a46cc: eb 0b                       	jmp	0x1417a46d9 <.text+0x17a36d9>
1417a46ce: 66 90                       	nop
1417a46d0: 48 83 c3 40                 	add	rbx, 0x40
1417a46d4: 48 ff cf                    	dec	rdi
1417a46d7: 74 19                       	je	0x1417a46f2 <.text+0x17a36f2>
1417a46d9: 48 8b 53 f8                 	mov	rdx, qword ptr [rbx - 0x8]
1417a46dd: 48 85 d2                    	test	rdx, rdx
1417a46e0: 74 ee                       	je	0x1417a46d0 <.text+0x17a36d0>
1417a46e2: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
1417a46e5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a46eb: e8 30 f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a46f0: eb de                       	jmp	0x1417a46d0 <.text+0x17a36d0>
1417a46f2: 48 8b 95 10 01 00 00        	mov	rdx, qword ptr [rbp + 0x110]
1417a46f9: 48 85 d2                    	test	rdx, rdx
1417a46fc: 74 12                       	je	0x1417a4710 <.text+0x17a3710>
1417a46fe: 48 c1 e2 06                 	shl	rdx, 0x6
1417a4702: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4708: 48 89 f1                    	mov	rcx, rsi
1417a470b: e8 10 f5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4710: 48 8b 95 70 01 00 00        	mov	rdx, qword ptr [rbp + 0x170]
1417a4717: 48 85 d2                    	test	rdx, rdx
1417a471a: 74 16                       	je	0x1417a4732 <.text+0x17a3732>
1417a471c: 48 8b 8d 78 01 00 00        	mov	rcx, qword ptr [rbp + 0x178]
1417a4723: 48 c1 e2 03                 	shl	rdx, 0x3
1417a4727: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a472d: e8 ee f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4732: 48 8b 95 d0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d0]
1417a4739: 48 85 d2                    	test	rdx, rdx
1417a473c: 74 12                       	je	0x1417a4750 <.text+0x17a3750>
1417a473e: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4744: 48 8b 8d f8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f8]
1417a474b: e8 d0 f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4750: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
1417a4754: 48 8d 55 08                 	lea	rdx, [rbp + 0x8]
1417a4758: 44 88 bd 1d 03 00 00        	mov	byte ptr [rbp + 0x31d], r15b
1417a475f: 41 b8 30 00 00 00           	mov	r8d, 0x30
1417a4765: 41 b9 10 00 00 00           	mov	r9d, 0x10
1417a476b: e8 00 ae 8a fe              	call	0x14004f570 <.text+0x4e570>
1417a4770: 48 8b 95 a0 00 00 00        	mov	rdx, qword ptr [rbp + 0xa0]
1417a4777: 48 85 d2                    	test	rdx, rdx
1417a477a: 74 16                       	je	0x1417a4792 <.text+0x17a3792>
1417a477c: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1417a4783: 48 c1 e2 04                 	shl	rdx, 0x4
1417a4787: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a478d: e8 8e f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4792: 48 8b 95 c0 00 00 00        	mov	rdx, qword ptr [rbp + 0xc0]
1417a4799: 48 85 d2                    	test	rdx, rdx
1417a479c: 74 16                       	je	0x1417a47b4 <.text+0x17a37b4>
1417a479e: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
1417a47a5: 48 c1 e2 04                 	shl	rdx, 0x4
1417a47a9: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a47af: e8 6c f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a47b4: 48 8b 95 e0 00 00 00        	mov	rdx, qword ptr [rbp + 0xe0]
1417a47bb: 48 85 d2                    	test	rdx, rdx
1417a47be: 74 16                       	je	0x1417a47d6 <.text+0x17a37d6>
1417a47c0: 48 8b 8d e8 00 00 00        	mov	rcx, qword ptr [rbp + 0xe8]
1417a47c7: 48 c1 e2 04                 	shl	rdx, 0x4
1417a47cb: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a47d1: e8 4a f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a47d6: 45 84 ff                    	test	r15b, r15b
1417a47d9: 74 69                       	je	0x1417a4844 <.text+0x17a3844>
1417a47db: 48 8b b5 98 01 00 00        	mov	rsi, qword ptr [rbp + 0x198]
1417a47e2: 48 8b bd a0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1a0]
1417a47e9: 48 85 ff                    	test	rdi, rdi
1417a47ec: 74 34                       	je	0x1417a4822 <.text+0x17a3822>
1417a47ee: 48 8d 5e 18                 	lea	rbx, [rsi + 0x18]
1417a47f2: eb 15                       	jmp	0x1417a4809 <.text+0x17a3809>
1417a47f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1417a4800: 48 83 c3 28                 	add	rbx, 0x28
1417a4804: 48 ff cf                    	dec	rdi
1417a4807: 74 19                       	je	0x1417a4822 <.text+0x17a3822>
1417a4809: 48 8b 53 f8                 	mov	rdx, qword ptr [rbx - 0x8]
1417a480d: 48 85 d2                    	test	rdx, rdx
1417a4810: 74 ee                       	je	0x1417a4800 <.text+0x17a3800>
1417a4812: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
1417a4815: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a481b: e8 00 f4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4820: eb de                       	jmp	0x1417a4800 <.text+0x17a3800>
1417a4822: 48 8b 85 90 01 00 00        	mov	rax, qword ptr [rbp + 0x190]
1417a4829: 48 85 c0                    	test	rax, rax
1417a482c: 74 16                       	je	0x1417a4844 <.text+0x17a3844>
1417a482e: 48 c1 e0 03                 	shl	rax, 0x3
1417a4832: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1417a4836: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a483c: 48 89 f1                    	mov	rcx, rsi
1417a483f: e8 dc f3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4844: 48 8b b5 38 01 00 00        	mov	rsi, qword ptr [rbp + 0x138]
1417a484b: 48 8b bd 40 01 00 00        	mov	rdi, qword ptr [rbp + 0x140]
1417a4852: 48 85 ff                    	test	rdi, rdi
1417a4855: 74 2f                       	je	0x1417a4886 <.text+0x17a3886>
1417a4857: 48 8d 5e 18                 	lea	rbx, [rsi + 0x18]
1417a485b: eb 0c                       	jmp	0x1417a4869 <.text+0x17a3869>
1417a485d: 0f 1f 00                    	nop	dword ptr [rax]
1417a4860: 48 83 c3 28                 	add	rbx, 0x28
1417a4864: 48 ff cf                    	dec	rdi
1417a4867: 74 1d                       	je	0x1417a4886 <.text+0x17a3886>
1417a4869: 48 8b 43 f8                 	mov	rax, qword ptr [rbx - 0x8]
1417a486d: 48 85 c0                    	test	rax, rax
1417a4870: 74 ee                       	je	0x1417a4860 <.text+0x17a3860>
1417a4872: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
1417a4875: 48 6b d0 58                 	imul	rdx, rax, 0x58
1417a4879: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a487f: e8 9c f3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4884: eb da                       	jmp	0x1417a4860 <.text+0x17a3860>
1417a4886: 48 8b 85 30 01 00 00        	mov	rax, qword ptr [rbp + 0x130]
1417a488d: 48 85 c0                    	test	rax, rax
1417a4890: 74 16                       	je	0x1417a48a8 <.text+0x17a38a8>
1417a4892: 48 c1 e0 03                 	shl	rax, 0x3
1417a4896: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1417a489a: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a48a0: 48 89 f1                    	mov	rcx, rsi
1417a48a3: e8 78 f3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a48a8: 48 8b 85 b0 01 00 00        	mov	rax, qword ptr [rbp + 0x1b0]
1417a48af: 48 85 c0                    	test	rax, rax
1417a48b2: 74 16                       	je	0x1417a48ca <.text+0x17a38ca>
1417a48b4: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1417a48bb: 48 6b d0 58                 	imul	rdx, rax, 0x58
1417a48bf: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a48c5: e8 56 f3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a48ca: 48 8b b5 58 01 00 00        	mov	rsi, qword ptr [rbp + 0x158]
1417a48d1: 48 8b bd 60 01 00 00        	mov	rdi, qword ptr [rbp + 0x160]
1417a48d8: 48 85 ff                    	test	rdi, rdi
1417a48db: 74 55                       	je	0x1417a4932 <.text+0x17a3932>
1417a48dd: 48 8d 5e 28                 	lea	rbx, [rsi + 0x28]
1417a48e1: eb 16                       	jmp	0x1417a48f9 <.text+0x17a38f9>
1417a48e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1417a48f0: 48 83 c3 78                 	add	rbx, 0x78
1417a48f4: 48 ff cf                    	dec	rdi
1417a48f7: 74 39                       	je	0x1417a4932 <.text+0x17a3932>
1417a48f9: 48 8b 53 e0                 	mov	rdx, qword ptr [rbx - 0x20]
1417a48fd: 48 85 d2                    	test	rdx, rdx
1417a4900: 74 13                       	je	0x1417a4915 <.text+0x17a3915>
1417a4902: 48 8b 4b e8                 	mov	rcx, qword ptr [rbx - 0x18]
1417a4906: 48 c1 e2 03                 	shl	rdx, 0x3
1417a490a: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a4910: e8 0b f3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4915: 48 8b 53 f8                 	mov	rdx, qword ptr [rbx - 0x8]
1417a4919: 48 85 d2                    	test	rdx, rdx
1417a491c: 74 d2                       	je	0x1417a48f0 <.text+0x17a38f0>
1417a491e: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
1417a4921: 48 c1 e2 02                 	shl	rdx, 0x2
1417a4925: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a492b: e8 f0 f2 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4930: eb be                       	jmp	0x1417a48f0 <.text+0x17a38f0>
1417a4932: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1417a4939: 48 85 c0                    	test	rax, rax
1417a493c: 74 12                       	je	0x1417a4950 <.text+0x17a3950>
1417a493e: 48 6b d0 78                 	imul	rdx, rax, 0x78
1417a4942: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4948: 48 89 f1                    	mov	rcx, rsi
1417a494b: e8 d0 f2 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4950: ba 98 01 00 00              	mov	edx, 0x198
1417a4955: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a495b: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
1417a4962: e8 b9 f2 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4967: 4c 89 f0                    	mov	rax, r14
1417a496a: 48 81 c4 c8 03 00 00        	add	rsp, 0x3c8
1417a4971: 5b                          	pop	rbx
1417a4972: 5f                          	pop	rdi
1417a4973: 5e                          	pop	rsi
1417a4974: 41 5c                       	pop	r12
1417a4976: 41 5d                       	pop	r13
1417a4978: 41 5e                       	pop	r14
1417a497a: 41 5f                       	pop	r15
1417a497c: 5d                          	pop	rbp
1417a497d: c3                          	ret
1417a497e: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a4985: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a498c: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1417a4993: e8 58 62 0d ff              	call	0x14087abf0 <.text+0x879bf0>
1417a4998: 48 8b 8d 80 00 00 00        	mov	rcx, qword ptr [rbp + 0x80]
1417a499f: 48 85 c9                    	test	rcx, rcx
1417a49a2: 74 45                       	je	0x1417a49e9 <.text+0x17a39e9>
1417a49a4: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a49ab: c6 85 3e 03 00 00 00        	mov	byte ptr [rbp + 0x33e], 0x0
1417a49b2: e8 d9 4b 10 01              	call	0x1428a9590 <.text+0x28a8590>
1417a49b7: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a49be: 49 89 46 08                 	mov	qword ptr [r14 + 0x8], rax
1417a49c2: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1417a49cc: 49 89 06                    	mov	qword ptr [r14], rax
1417a49cf: 45 31 ff                    	xor	r15d, r15d
1417a49d2: 31 f6                       	xor	esi, esi
1417a49d4: 0f b6 85 00 02 00 00        	movzx	eax, byte ptr [rbp + 0x200]
1417a49db: 83 f8 03                    	cmp	eax, 0x3
1417a49de: 0f 83 3b fb ff ff           	jae	0x1417a451f <.text+0x17a351f>
1417a49e4: e9 d1 fb ff ff              	jmp	0x1417a45ba <.text+0x17a35ba>
1417a49e9: 48 8b 85 08 01 00 00        	mov	rax, qword ptr [rbp + 0x108]
1417a49f0: 4c 8b b5 48 02 00 00        	mov	r14, qword ptr [rbp + 0x248]
1417a49f7: 49 89 46 10                 	mov	qword ptr [r14 + 0x10], rax
1417a49fb: f3 0f 6f 85 f8 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0xf8]
1417a4a03: f3 41 0f 7f 06              	movdqu	xmmword ptr [r14], xmm0
1417a4a08: c6 85 3d 03 00 00 00        	mov	byte ptr [rbp + 0x33d], 0x0
1417a4a0f: c6 85 3c 03 00 00 00        	mov	byte ptr [rbp + 0x33c], 0x0
1417a4a16: c6 85 3b 03 00 00 00        	mov	byte ptr [rbp + 0x33b], 0x0
1417a4a1d: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
1417a4a24: e8 67 90 0b ff              	call	0x14085da90 <.text+0x85ca90>
1417a4a29: c6 85 ce 02 00 00 00        	mov	byte ptr [rbp + 0x2ce], 0x0
1417a4a30: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
1417a4a34: e8 17 55 9a fe              	call	0x140149f50 <.text+0x148f50>
1417a4a39: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1417a4a3d: 48 85 d2                    	test	rdx, rdx
1417a4a40: 74 13                       	je	0x1417a4a55 <.text+0x17a3a55>
1417a4a42: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1417a4a46: 48 c1 e2 03                 	shl	rdx, 0x3
1417a4a4a: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a4a50: e8 cb f1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4a55: c6 85 cd 02 00 00 00        	mov	byte ptr [rbp + 0x2cd], 0x0
1417a4a5c: 48 8d 8d 10 01 00 00        	lea	rcx, [rbp + 0x110]
1417a4a63: e8 b8 a8 0d ff              	call	0x14087f320 <.text+0x87e320>
1417a4a68: 48 8b 95 70 01 00 00        	mov	rdx, qword ptr [rbp + 0x170]
1417a4a6f: 48 85 d2                    	test	rdx, rdx
1417a4a72: 74 16                       	je	0x1417a4a8a <.text+0x17a3a8a>
1417a4a74: 48 8b 8d 78 01 00 00        	mov	rcx, qword ptr [rbp + 0x178]
1417a4a7b: 48 c1 e2 03                 	shl	rdx, 0x3
1417a4a7f: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4a85: e8 96 f1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4a8a: 48 8b 95 d0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d0]
1417a4a91: 48 85 d2                    	test	rdx, rdx
1417a4a94: 74 12                       	je	0x1417a4aa8 <.text+0x17a3aa8>
1417a4a96: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4a9c: 48 8b 8d f8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f8]
1417a4aa3: e8 78 f1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4aa8: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
1417a4aac: 48 8d 55 08                 	lea	rdx, [rbp + 0x8]
1417a4ab0: c6 85 1d 03 00 00 00        	mov	byte ptr [rbp + 0x31d], 0x0
1417a4ab7: 41 b8 30 00 00 00           	mov	r8d, 0x30
1417a4abd: 41 b9 10 00 00 00           	mov	r9d, 0x10
1417a4ac3: e8 a8 aa 8a fe              	call	0x14004f570 <.text+0x4e570>
1417a4ac8: 48 8b 95 a0 00 00 00        	mov	rdx, qword ptr [rbp + 0xa0]
1417a4acf: 48 85 d2                    	test	rdx, rdx
1417a4ad2: 74 16                       	je	0x1417a4aea <.text+0x17a3aea>
1417a4ad4: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1417a4adb: 48 c1 e2 04                 	shl	rdx, 0x4
1417a4adf: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4ae5: e8 36 f1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4aea: 48 8b 95 c0 00 00 00        	mov	rdx, qword ptr [rbp + 0xc0]
1417a4af1: 48 85 d2                    	test	rdx, rdx
1417a4af4: 74 16                       	je	0x1417a4b0c <.text+0x17a3b0c>
1417a4af6: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
1417a4afd: 48 c1 e2 04                 	shl	rdx, 0x4
1417a4b01: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4b07: e8 14 f1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4b0c: 48 8b 95 e0 00 00 00        	mov	rdx, qword ptr [rbp + 0xe0]
1417a4b13: 48 85 d2                    	test	rdx, rdx
1417a4b16: 74 16                       	je	0x1417a4b2e <.text+0x17a3b2e>
1417a4b18: 48 8b 8d e8 00 00 00        	mov	rcx, qword ptr [rbp + 0xe8]
1417a4b1f: 48 c1 e2 04                 	shl	rdx, 0x4
1417a4b23: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4b29: e8 f2 f0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4b2e: 48 8d 8d 30 01 00 00        	lea	rcx, [rbp + 0x130]
1417a4b35: e8 66 da 0d ff              	call	0x1408825a0 <.text+0x8815a0>
1417a4b3a: 48 8b 85 b0 01 00 00        	mov	rax, qword ptr [rbp + 0x1b0]
1417a4b41: 48 85 c0                    	test	rax, rax
1417a4b44: 74 16                       	je	0x1417a4b5c <.text+0x17a3b5c>
1417a4b46: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1417a4b4d: 48 6b d0 58                 	imul	rdx, rax, 0x58
1417a4b51: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4b57: e8 c4 f0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a4b5c: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1417a4b63: e8 e8 ab 0d ff              	call	0x14087f750 <.text+0x87e750>
1417a4b68: e9 e3 fd ff ff              	jmp	0x1417a4950 <.text+0x17a3950>
1417a4b6d: 48 83 fa 15                 	cmp	rdx, 0x15
1417a4b71: 0f 83 76 07 00 00           	jae	0x1417a52ed <.text+0x17a42ed>
1417a4b77: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a4b7e: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a4b85: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a4b8c: 4c 8d 8d d0 02 00 00        	lea	r9, [rbp + 0x2d0]
1417a4b93: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4b99: e8 92 45 b9 ff              	call	0x141339130 <.text+0x1338130>
1417a4b9e: e9 8b d3 ff ff              	jmp	0x1417a1f2e <.text+0x17a0f2e>
1417a4ba3: 48 83 fa 15                 	cmp	rdx, 0x15
1417a4ba7: 0f 83 66 07 00 00           	jae	0x1417a5313 <.text+0x17a4313>
1417a4bad: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a4bb4: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a4bbb: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a4bc2: 4c 8d 8d d0 02 00 00        	lea	r9, [rbp + 0x2d0]
1417a4bc9: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4bcf: e8 ac 41 b9 ff              	call	0x141338d80 <.text+0x1337d80>
1417a4bd4: e9 74 d3 ff ff              	jmp	0x1417a1f4d <.text+0x17a0f4d>
1417a4bd9: 48 83 fa 15                 	cmp	rdx, 0x15
1417a4bdd: 0f 83 56 07 00 00           	jae	0x1417a5339 <.text+0x17a4339>
1417a4be3: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a4bea: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a4bf1: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a4bf8: 4c 8d 8d d0 02 00 00        	lea	r9, [rbp + 0x2d0]
1417a4bff: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4c05: e8 96 46 b9 ff              	call	0x1413392a0 <.text+0x13382a0>
1417a4c0a: e9 5d d3 ff ff              	jmp	0x1417a1f6c <.text+0x17a0f6c>
1417a4c0f: 48 83 fa 15                 	cmp	rdx, 0x15
1417a4c13: 0f 83 46 07 00 00           	jae	0x1417a535f <.text+0x17a435f>
1417a4c19: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a4c20: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a4c27: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a4c2e: 4c 8d 8d d0 02 00 00        	lea	r9, [rbp + 0x2d0]
1417a4c35: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a4c3b: e8 50 3d b9 ff              	call	0x141338990 <.text+0x1337990>
1417a4c40: e9 46 d3 ff ff              	jmp	0x1417a1f8b <.text+0x17a0f8b>
1417a4c45: 48 8b 8d 78 01 00 00        	mov	rcx, qword ptr [rbp + 0x178]
1417a4c4c: 48 83 fa 15                 	cmp	rdx, 0x15
1417a4c50: 0f 83 2f 07 00 00           	jae	0x1417a5385 <.text+0x17a4385>
1417a4c56: 48 8d 04 d1                 	lea	rax, [rcx + 8*rdx]
1417a4c5a: 48 8d 51 08                 	lea	rdx, [rcx + 0x8]
1417a4c5e: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a4c64: eb 17                       	jmp	0x1417a4c7d <.text+0x17a3c7d>
1417a4c66: 49 89 ca                    	mov	r10, rcx
1417a4c69: 4d 89 0a                    	mov	qword ptr [r10], r9
1417a4c6c: 48 83 c2 08                 	add	rdx, 0x8
1417a4c70: 49 83 c0 08                 	add	r8, 0x8
1417a4c74: 48 39 c2                    	cmp	rdx, rax
1417a4c77: 0f 84 1f d3 ff ff           	je	0x1417a1f9c <.text+0x17a0f9c>
1417a4c7d: 4c 8b 5a f8                 	mov	r11, qword ptr [rdx - 0x8]
1417a4c81: 4c 8b 0a                    	mov	r9, qword ptr [rdx]
1417a4c84: 4d 39 d9                    	cmp	r9, r11
1417a4c87: 73 e3                       	jae	0x1417a4c6c <.text+0x17a3c6c>
1417a4c89: 4d 89 c2                    	mov	r10, r8
1417a4c8c: 4e 89 1c 11                 	mov	qword ptr [rcx + r10], r11
1417a4c90: 49 83 fa 08                 	cmp	r10, 0x8
1417a4c94: 74 d0                       	je	0x1417a4c66 <.text+0x17a3c66>
1417a4c96: 4e 8b 5c 11 f0              	mov	r11, qword ptr [rcx + r10 - 0x10]
1417a4c9b: 49 83 c2 f8                 	add	r10, -0x8
1417a4c9f: 4d 39 d9                    	cmp	r9, r11
1417a4ca2: 72 e8                       	jb	0x1417a4c8c <.text+0x17a3c8c>
1417a4ca4: 49 01 ca                    	add	r10, rcx
1417a4ca7: eb c0                       	jmp	0x1417a4c69 <.text+0x17a3c69>
1417a4ca9: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a4cb0: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a4cb7: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4cc0: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4cc6: 48 89 f1                    	mov	rcx, rsi
1417a4cc9: e8 92 4d 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4cce: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a4cd2: e9 e4 e7 ff ff              	jmp	0x1417a34bb <.text+0x17a24bb>
1417a4cd7: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a4cde: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a4ce5: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4cee: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4cf4: 48 89 f1                    	mov	rcx, rsi
1417a4cf7: e8 64 4d 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4cfc: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a4d00: e9 69 f7 ff ff              	jmp	0x1417a446e <.text+0x17a346e>
1417a4d05: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4d0e: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4d14: 48 89 f1                    	mov	rcx, rsi
1417a4d17: e8 44 4d 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4d1c: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a4d20: e9 02 f0 ff ff              	jmp	0x1417a3d27 <.text+0x17a2d27>
1417a4d25: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4d2e: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4d34: 48 89 f1                    	mov	rcx, rsi
1417a4d37: e8 24 4d 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4d3c: e9 3c e5 ff ff              	jmp	0x1417a327d <.text+0x17a227d>
1417a4d41: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4d4a: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4d50: 48 89 f9                    	mov	rcx, rdi
1417a4d53: e8 08 4d 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4d58: e9 d9 e6 ff ff              	jmp	0x1417a3436 <.text+0x17a2436>
1417a4d5d: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4d66: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4d6c: 48 89 f1                    	mov	rcx, rsi
1417a4d6f: e8 ec 4c 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4d74: e9 b2 eb ff ff              	jmp	0x1417a392b <.text+0x17a292b>
1417a4d79: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a4d80: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a4d87: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4d90: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4d96: 48 89 f1                    	mov	rcx, rsi
1417a4d99: e8 c2 4c 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4d9e: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1417a4da2: e9 a5 f1 ff ff              	jmp	0x1417a3f4c <.text+0x17a2f4c>
1417a4da7: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4db0: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4db6: 48 89 f9                    	mov	rcx, rdi
1417a4db9: e8 a2 4c 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4dbe: e9 2a f5 ff ff              	jmp	0x1417a42ed <.text+0x17a32ed>
1417a4dc3: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1417a4dcc: 41 b9 01 00 00 00           	mov	r9d, 0x1
1417a4dd2: 48 89 f9                    	mov	rcx, rdi
1417a4dd5: e8 86 4c 10 01              	call	0x1428a9a60 <.text+0x28a8a60>
1417a4dda: e9 1e f4 ff ff              	jmp	0x1417a41fd <.text+0x17a31fd>
1417a4ddf: 4c 8d 05 a2 03 34 01        	lea	r8, [rip + 0x13403a2]   # 0x142ae5188
1417a4de6: b9 01 00 00 00              	mov	ecx, 0x1
1417a4deb: ba 00 80 00 00              	mov	edx, 0x8000
1417a4df0: e8 ee 41 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4df5: e9 c7 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4dfa: c6 85 20 03 00 00 01        	mov	byte ptr [rbp + 0x320], 0x1
1417a4e01: 4c 8d 05 38 77 53 01        	lea	r8, [rip + 0x1537738]   # 0x142cdc540
1417a4e08: b9 01 00 00 00              	mov	ecx, 0x1
1417a4e0d: ba 00 00 08 00              	mov	edx, 0x80000
1417a4e12: e8 cc 41 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4e17: e9 a5 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4e1c: 4c 8d 05 fd 74 53 01        	lea	r8, [rip + 0x15374fd]   # 0x142cdc320
1417a4e23: b9 01 00 00 00              	mov	ecx, 0x1
1417a4e28: ba 07 00 00 00              	mov	edx, 0x7
1417a4e2d: e8 b1 41 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4e32: e9 8a 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4e37: 4c 8d 05 e2 74 53 01        	lea	r8, [rip + 0x15374e2]   # 0x142cdc320
1417a4e3e: b9 01 00 00 00              	mov	ecx, 0x1
1417a4e43: ba 05 00 00 00              	mov	edx, 0x5
1417a4e48: e8 96 41 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4e4d: e9 6f 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4e52: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a4e59: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a4e60: 48 8d 05 f9 76 53 01        	lea	rax, [rip + 0x15376f9]  # 0x142cdc560
1417a4e67: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a4e6c: 48 8d 0d 6d 73 53 01        	lea	rcx, [rip + 0x153736d]  # 0x142cdc1e0
1417a4e73: 4c 8d 0d 46 73 53 01        	lea	r9, [rip + 0x1537346]   # 0x142cdc1c0
1417a4e7a: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a4e81: ba 2b 00 00 00              	mov	edx, 0x2b
1417a4e86: e8 d5 48 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a4e8b: e9 31 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4e90: 4c 8d 05 89 74 53 01        	lea	r8, [rip + 0x1537489]   # 0x142cdc320
1417a4e97: b9 01 00 00 00              	mov	ecx, 0x1
1417a4e9c: ba 05 00 00 00              	mov	edx, 0x5
1417a4ea1: e8 3d 41 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4ea6: e9 16 05 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4eab: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a4eb2: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a4eb9: 48 8d 05 a0 76 53 01        	lea	rax, [rip + 0x15376a0]  # 0x142cdc560
1417a4ec0: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a4ec5: 48 8d 0d 14 73 53 01        	lea	rcx, [rip + 0x1537314]  # 0x142cdc1e0
1417a4ecc: 4c 8d 0d ed 72 53 01        	lea	r9, [rip + 0x15372ed]   # 0x142cdc1c0
1417a4ed3: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a4eda: ba 2b 00 00 00              	mov	edx, 0x2b
1417a4edf: e8 7c 48 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a4ee4: e9 d8 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4ee9: 4c 8d 05 30 74 53 01        	lea	r8, [rip + 0x1537430]   # 0x142cdc320
1417a4ef0: b9 01 00 00 00              	mov	ecx, 0x1
1417a4ef5: ba 0a 00 00 00              	mov	edx, 0xa
1417a4efa: e8 e4 40 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4eff: e9 bd 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4f04: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a4f0b: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a4f12: 48 8d 05 47 76 53 01        	lea	rax, [rip + 0x1537647]  # 0x142cdc560
1417a4f19: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a4f1e: 48 8d 0d bb 72 53 01        	lea	rcx, [rip + 0x15372bb]  # 0x142cdc1e0
1417a4f25: 4c 8d 0d 94 72 53 01        	lea	r9, [rip + 0x1537294]   # 0x142cdc1c0
1417a4f2c: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a4f33: ba 2b 00 00 00              	mov	edx, 0x2b
1417a4f38: e8 23 48 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a4f3d: e9 7f 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4f42: 4c 8d 05 d7 73 53 01        	lea	r8, [rip + 0x15373d7]   # 0x142cdc320
1417a4f49: b9 01 00 00 00              	mov	ecx, 0x1
1417a4f4e: ba 06 00 00 00              	mov	edx, 0x6
1417a4f53: e8 8b 40 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4f58: e9 64 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4f5d: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a4f64: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a4f6b: 48 8d 05 ee 75 53 01        	lea	rax, [rip + 0x15375ee]  # 0x142cdc560
1417a4f72: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a4f77: 48 8d 0d 62 72 53 01        	lea	rcx, [rip + 0x1537262]  # 0x142cdc1e0
1417a4f7e: 4c 8d 0d 3b 72 53 01        	lea	r9, [rip + 0x153723b]   # 0x142cdc1c0
1417a4f85: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a4f8c: ba 2b 00 00 00              	mov	edx, 0x2b
1417a4f91: e8 ca 47 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a4f96: e9 26 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4f9b: 4c 8d 05 7e 73 53 01        	lea	r8, [rip + 0x153737e]   # 0x142cdc320
1417a4fa2: b9 01 00 00 00              	mov	ecx, 0x1
1417a4fa7: ba 0a 00 00 00              	mov	edx, 0xa
1417a4fac: e8 32 40 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a4fb1: e9 0b 04 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4fb6: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a4fbd: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a4fc4: 48 8d 05 95 75 53 01        	lea	rax, [rip + 0x1537595]  # 0x142cdc560
1417a4fcb: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a4fd0: 48 8d 0d 09 72 53 01        	lea	rcx, [rip + 0x1537209]  # 0x142cdc1e0
1417a4fd7: 4c 8d 0d e2 71 53 01        	lea	r9, [rip + 0x15371e2]   # 0x142cdc1c0
1417a4fde: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a4fe5: ba 2b 00 00 00              	mov	edx, 0x2b
1417a4fea: e8 71 47 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a4fef: e9 cd 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a4ff4: 4c 8d 05 25 73 53 01        	lea	r8, [rip + 0x1537325]   # 0x142cdc320
1417a4ffb: b9 01 00 00 00              	mov	ecx, 0x1
1417a5000: ba 0a 00 00 00              	mov	edx, 0xa
1417a5005: e8 d9 3f 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a500a: e9 b2 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a500f: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a5016: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a501d: 48 8d 05 3c 75 53 01        	lea	rax, [rip + 0x153753c]  # 0x142cdc560
1417a5024: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a5029: 48 8d 0d b0 71 53 01        	lea	rcx, [rip + 0x15371b0]  # 0x142cdc1e0
1417a5030: 4c 8d 0d 89 71 53 01        	lea	r9, [rip + 0x1537189]   # 0x142cdc1c0
1417a5037: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a503e: ba 2b 00 00 00              	mov	edx, 0x2b
1417a5043: e8 18 47 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a5048: e9 74 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a504d: 4c 8d 05 cc 72 53 01        	lea	r8, [rip + 0x15372cc]   # 0x142cdc320
1417a5054: b9 01 00 00 00              	mov	ecx, 0x1
1417a5059: ba 10 00 00 00              	mov	edx, 0x10
1417a505e: e8 80 3f 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a5063: e9 59 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a5068: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a506f: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a5076: 48 8d 05 e3 74 53 01        	lea	rax, [rip + 0x15374e3]  # 0x142cdc560
1417a507d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a5082: 48 8d 0d 57 71 53 01        	lea	rcx, [rip + 0x1537157]  # 0x142cdc1e0
1417a5089: 4c 8d 0d 30 71 53 01        	lea	r9, [rip + 0x1537130]   # 0x142cdc1c0
1417a5090: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a5097: ba 2b 00 00 00              	mov	edx, 0x2b
1417a509c: e8 bf 46 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a50a1: e9 1b 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a50a6: 4c 8d 05 73 72 53 01        	lea	r8, [rip + 0x1537273]   # 0x142cdc320
1417a50ad: b9 01 00 00 00              	mov	ecx, 0x1
1417a50b2: ba 07 00 00 00              	mov	edx, 0x7
1417a50b7: e8 27 3f 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a50bc: e9 00 03 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a50c1: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a50c8: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a50cf: 48 8d 05 8a 74 53 01        	lea	rax, [rip + 0x153748a]  # 0x142cdc560
1417a50d6: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a50db: 48 8d 0d fe 70 53 01        	lea	rcx, [rip + 0x15370fe]  # 0x142cdc1e0
1417a50e2: 4c 8d 0d d7 70 53 01        	lea	r9, [rip + 0x15370d7]   # 0x142cdc1c0
1417a50e9: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a50f0: ba 2b 00 00 00              	mov	edx, 0x2b
1417a50f5: e8 66 46 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a50fa: e9 c2 02 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a50ff: 4c 8d 05 1a 72 53 01        	lea	r8, [rip + 0x153721a]   # 0x142cdc320
1417a5106: b9 01 00 00 00              	mov	ecx, 0x1
1417a510b: ba 12 00 00 00              	mov	edx, 0x12
1417a5110: e8 ce 3e 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a5115: e9 a7 02 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a511a: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a5121: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a5128: 48 8d 05 31 74 53 01        	lea	rax, [rip + 0x1537431]  # 0x142cdc560
1417a512f: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a5134: 48 8d 0d a5 70 53 01        	lea	rcx, [rip + 0x15370a5]  # 0x142cdc1e0
1417a513b: 4c 8d 0d 7e 70 53 01        	lea	r9, [rip + 0x153707e]   # 0x142cdc1c0
1417a5142: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a5149: ba 2b 00 00 00              	mov	edx, 0x2b
1417a514e: e8 0d 46 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a5153: e9 69 02 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a5158: 4c 8d 05 c1 71 53 01        	lea	r8, [rip + 0x15371c1]   # 0x142cdc320
1417a515f: b9 01 00 00 00              	mov	ecx, 0x1
1417a5164: ba 05 00 00 00              	mov	edx, 0x5
1417a5169: e8 75 3e 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a516e: e9 4e 02 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a5173: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a517a: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a5181: 48 8d 05 d8 73 53 01        	lea	rax, [rip + 0x15373d8]  # 0x142cdc560
1417a5188: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a518d: 48 8d 0d 4c 70 53 01        	lea	rcx, [rip + 0x153704c]  # 0x142cdc1e0
1417a5194: 4c 8d 0d 25 70 53 01        	lea	r9, [rip + 0x1537025]   # 0x142cdc1c0
1417a519b: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a51a2: ba 2b 00 00 00              	mov	edx, 0x2b
1417a51a7: e8 b4 45 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a51ac: e9 10 02 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a51b1: 4c 8d 05 68 71 53 01        	lea	r8, [rip + 0x1537168]   # 0x142cdc320
1417a51b8: b9 01 00 00 00              	mov	ecx, 0x1
1417a51bd: ba 0b 00 00 00              	mov	edx, 0xb
1417a51c2: e8 1c 3e 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a51c7: e9 f5 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a51cc: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a51d3: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a51da: 48 8d 05 7f 73 53 01        	lea	rax, [rip + 0x153737f]  # 0x142cdc560
1417a51e1: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a51e6: 48 8d 0d f3 6f 53 01        	lea	rcx, [rip + 0x1536ff3]  # 0x142cdc1e0
1417a51ed: 4c 8d 0d cc 6f 53 01        	lea	r9, [rip + 0x1536fcc]   # 0x142cdc1c0
1417a51f4: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a51fb: ba 2b 00 00 00              	mov	edx, 0x2b
1417a5200: e8 5b 45 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a5205: e9 b7 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a520a: 4c 8d 05 0f 71 53 01        	lea	r8, [rip + 0x153710f]   # 0x142cdc320
1417a5211: b9 01 00 00 00              	mov	ecx, 0x1
1417a5216: ba 06 00 00 00              	mov	edx, 0x6
1417a521b: e8 c3 3d 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a5220: e9 9c 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a5225: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a522c: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a5233: 48 8d 05 26 73 53 01        	lea	rax, [rip + 0x1537326]  # 0x142cdc560
1417a523a: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a523f: 48 8d 0d 9a 6f 53 01        	lea	rcx, [rip + 0x1536f9a]  # 0x142cdc1e0
1417a5246: 4c 8d 0d 73 6f 53 01        	lea	r9, [rip + 0x1536f73]   # 0x142cdc1c0
1417a524d: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a5254: ba 2b 00 00 00              	mov	edx, 0x2b
1417a5259: e8 02 45 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a525e: e9 5e 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a5263: 4c 8d 05 b6 70 53 01        	lea	r8, [rip + 0x15370b6]   # 0x142cdc320
1417a526a: b9 01 00 00 00              	mov	ecx, 0x1
1417a526f: ba 15 00 00 00              	mov	edx, 0x15
1417a5274: e8 6a 3d 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a5279: e9 43 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a527e: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1417a5285: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1417a528c: 48 8d 05 cd 72 53 01        	lea	rax, [rip + 0x15372cd]  # 0x142cdc560
1417a5293: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1417a5298: 48 8d 0d 41 6f 53 01        	lea	rcx, [rip + 0x1536f41]  # 0x142cdc1e0
1417a529f: 4c 8d 0d 1a 6f 53 01        	lea	r9, [rip + 0x1536f1a]   # 0x142cdc1c0
1417a52a6: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a52ad: ba 2b 00 00 00              	mov	edx, 0x2b
1417a52b2: e8 a9 44 13 01              	call	0x1428d9760 <.text+0x28d8760>
1417a52b7: e9 05 01 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a52bc: 48 8d 0d 35 2e 32 01        	lea	rcx, [rip + 0x1322e35]  # 0x142ac80f8
1417a52c3: e8 48 40 13 01              	call	0x1428d9310 <.text+0x28d8310>
1417a52c8: e9 f4 00 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a52cd: 4c 89 bd 30 03 00 00        	mov	qword ptr [rbp + 0x330], r15
1417a52d4: 4c 8d 05 cd 6e 53 01        	lea	r8, [rip + 0x1536ecd]   # 0x142cdc1a8
1417a52db: b9 08 00 00 00              	mov	ecx, 0x8
1417a52e0: 48 89 f2                    	mov	rdx, rsi
1417a52e3: e8 fb 3c 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a52e8: e9 d4 00 00 00              	jmp	0x1417a53c1 <.text+0x17a43c1>
1417a52ed: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a52f4: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a52fb: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a5302: 4c 8d 85 d0 02 00 00        	lea	r8, [rbp + 0x2d0]
1417a5309: e8 d2 38 1b ff              	call	0x140958be0 <.text+0x957be0>
1417a530e: e9 1b cc ff ff              	jmp	0x1417a1f2e <.text+0x17a0f2e>
1417a5313: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a531a: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a5321: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a5328: 4c 8d 85 d0 02 00 00        	lea	r8, [rbp + 0x2d0]
1417a532f: e8 dc 2d 1b ff              	call	0x140958110 <.text+0x957110>
1417a5334: e9 14 cc ff ff              	jmp	0x1417a1f4d <.text+0x17a0f4d>
1417a5339: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a5340: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a5347: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a534e: 4c 8d 85 d0 02 00 00        	lea	r8, [rbp + 0x2d0]
1417a5355: e8 f6 2e 1b ff              	call	0x140958250 <.text+0x957250>
1417a535a: e9 0d cc ff ff              	jmp	0x1417a1f6c <.text+0x17a0f6c>
1417a535f: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a5366: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a536d: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a5374: 4c 8d 85 d0 02 00 00        	lea	r8, [rbp + 0x2d0]
1417a537b: e8 c0 31 1b ff              	call	0x140958540 <.text+0x957540>
1417a5380: e9 06 cc ff ff              	jmp	0x1417a1f8b <.text+0x17a0f8b>
1417a5385: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a538c: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a5393: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a539a: 4c 8d 85 90 02 00 00        	lea	r8, [rbp + 0x290]
1417a53a1: e8 ba 39 1b ff              	call	0x140958d60 <.text+0x957d60>
1417a53a6: e9 f1 cb ff ff              	jmp	0x1417a1f9c <.text+0x17a0f9c>
1417a53ab: 4c 8d 05 d6 fd 33 01        	lea	r8, [rip + 0x133fdd6]   # 0x142ae5188
1417a53b2: b9 01 00 00 00              	mov	ecx, 0x1
1417a53b7: ba 00 80 00 00              	mov	edx, 0x8000
1417a53bc: e8 22 3c 13 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1417a53c1: 0f 0b                       	ud2
1417a53c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1417a53d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a53d5: 55                          	push	rbp
1417a53d6: 41 57                       	push	r15
1417a53d8: 41 56                       	push	r14
1417a53da: 41 55                       	push	r13
1417a53dc: 41 54                       	push	r12
1417a53de: 56                          	push	rsi
1417a53df: 57                          	push	rdi
1417a53e0: 53                          	push	rbx
1417a53e1: 48 83 ec 28                 	sub	rsp, 0x28
1417a53e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a53ec: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a53f3: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a53fa: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a5401: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a5408: 48 83 c4 28                 	add	rsp, 0x28
1417a540c: 5b                          	pop	rbx
1417a540d: 5f                          	pop	rdi
1417a540e: 5e                          	pop	rsi
1417a540f: 41 5c                       	pop	r12
1417a5411: 41 5d                       	pop	r13
1417a5413: 41 5e                       	pop	r14
1417a5415: 41 5f                       	pop	r15
1417a5417: 5d                          	pop	rbp
1417a5418: c3                          	ret
1417a5419: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1417a5420: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5425: 55                          	push	rbp
1417a5426: 41 57                       	push	r15
1417a5428: 41 56                       	push	r14
1417a542a: 41 55                       	push	r13
1417a542c: 41 54                       	push	r12
1417a542e: 56                          	push	rsi
1417a542f: 57                          	push	rdi
1417a5430: 53                          	push	rbx
1417a5431: 48 83 ec 28                 	sub	rsp, 0x28
1417a5435: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a543c: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1417a5443: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a544a: 48 83 c4 28                 	add	rsp, 0x28
1417a544e: 5b                          	pop	rbx
1417a544f: 5f                          	pop	rdi
1417a5450: 5e                          	pop	rsi
1417a5451: 41 5c                       	pop	r12
1417a5453: 41 5d                       	pop	r13
1417a5455: 41 5e                       	pop	r14
1417a5457: 41 5f                       	pop	r15
1417a5459: 5d                          	pop	rbp
1417a545a: c3                          	ret
1417a545b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1417a5460: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5465: 55                          	push	rbp
1417a5466: 41 57                       	push	r15
1417a5468: 41 56                       	push	r14
1417a546a: 41 55                       	push	r13
1417a546c: 41 54                       	push	r12
1417a546e: 56                          	push	rsi
1417a546f: 57                          	push	rdi
1417a5470: 53                          	push	rbx
1417a5471: 48 83 ec 28                 	sub	rsp, 0x28
1417a5475: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a547c: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a5483: e8 08 24 e4 fe              	call	0x1405e7890 <.text+0x5e6890>
1417a5488: 90                          	nop
1417a5489: 48 83 c4 28                 	add	rsp, 0x28
1417a548d: 5b                          	pop	rbx
1417a548e: 5f                          	pop	rdi
1417a548f: 5e                          	pop	rsi
1417a5490: 41 5c                       	pop	r12
1417a5492: 41 5d                       	pop	r13
1417a5494: 41 5e                       	pop	r14
1417a5496: 41 5f                       	pop	r15
1417a5498: 5d                          	pop	rbp
1417a5499: c3                          	ret
1417a549a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a54a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a54a5: 55                          	push	rbp
1417a54a6: 41 57                       	push	r15
1417a54a8: 41 56                       	push	r14
1417a54aa: 41 55                       	push	r13
1417a54ac: 41 54                       	push	r12
1417a54ae: 56                          	push	rsi
1417a54af: 57                          	push	rdi
1417a54b0: 53                          	push	rbx
1417a54b1: 48 83 ec 28                 	sub	rsp, 0x28
1417a54b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a54bc: 80 bd cf 02 00 00 00        	cmp	byte ptr [rbp + 0x2cf], 0x0
1417a54c3: 74 13                       	je	0x1417a54d8 <.text+0x17a44d8>
1417a54c5: c6 85 2e 03 00 00 01        	mov	byte ptr [rbp + 0x32e], 0x1
1417a54cc: 48 8d 8d 28 02 00 00        	lea	rcx, [rbp + 0x228]
1417a54d3: e8 28 25 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a54d8: c6 85 2e 03 00 00 01        	mov	byte ptr [rbp + 0x32e], 0x1
1417a54df: 48 83 c4 28                 	add	rsp, 0x28
1417a54e3: 5b                          	pop	rbx
1417a54e4: 5f                          	pop	rdi
1417a54e5: 5e                          	pop	rsi
1417a54e6: 41 5c                       	pop	r12
1417a54e8: 41 5d                       	pop	r13
1417a54ea: 41 5e                       	pop	r14
1417a54ec: 41 5f                       	pop	r15
1417a54ee: 5d                          	pop	rbp
1417a54ef: c3                          	ret
1417a54f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a54f5: 55                          	push	rbp
1417a54f6: 41 57                       	push	r15
1417a54f8: 41 56                       	push	r14
1417a54fa: 41 55                       	push	r13
1417a54fc: 41 54                       	push	r12
1417a54fe: 56                          	push	rsi
1417a54ff: 57                          	push	rdi
1417a5500: 53                          	push	rbx
1417a5501: 48 83 ec 28                 	sub	rsp, 0x28
1417a5505: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a550c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5513: 48 85 d2                    	test	rdx, rdx
1417a5516: 74 16                       	je	0x1417a552e <.text+0x17a452e>
1417a5518: 48 c1 e2 05                 	shl	rdx, 0x5
1417a551c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5522: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5529: e8 f2 e6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a552e: 90                          	nop
1417a552f: 48 83 c4 28                 	add	rsp, 0x28
1417a5533: 5b                          	pop	rbx
1417a5534: 5f                          	pop	rdi
1417a5535: 5e                          	pop	rsi
1417a5536: 41 5c                       	pop	r12
1417a5538: 41 5d                       	pop	r13
1417a553a: 41 5e                       	pop	r14
1417a553c: 41 5f                       	pop	r15
1417a553e: 5d                          	pop	rbp
1417a553f: c3                          	ret
1417a5540: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5545: 55                          	push	rbp
1417a5546: 41 57                       	push	r15
1417a5548: 41 56                       	push	r14
1417a554a: 41 55                       	push	r13
1417a554c: 41 54                       	push	r12
1417a554e: 56                          	push	rsi
1417a554f: 57                          	push	rdi
1417a5550: 53                          	push	rbx
1417a5551: 48 83 ec 28                 	sub	rsp, 0x28
1417a5555: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a555c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5563: 48 85 d2                    	test	rdx, rdx
1417a5566: 74 16                       	je	0x1417a557e <.text+0x17a457e>
1417a5568: 48 c1 e2 05                 	shl	rdx, 0x5
1417a556c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5572: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5579: e8 a2 e6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a557e: 90                          	nop
1417a557f: 48 83 c4 28                 	add	rsp, 0x28
1417a5583: 5b                          	pop	rbx
1417a5584: 5f                          	pop	rdi
1417a5585: 5e                          	pop	rsi
1417a5586: 41 5c                       	pop	r12
1417a5588: 41 5d                       	pop	r13
1417a558a: 41 5e                       	pop	r14
1417a558c: 41 5f                       	pop	r15
1417a558e: 5d                          	pop	rbp
1417a558f: c3                          	ret
1417a5590: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5595: 55                          	push	rbp
1417a5596: 41 57                       	push	r15
1417a5598: 41 56                       	push	r14
1417a559a: 41 55                       	push	r13
1417a559c: 41 54                       	push	r12
1417a559e: 56                          	push	rsi
1417a559f: 57                          	push	rdi
1417a55a0: 53                          	push	rbx
1417a55a1: 48 83 ec 28                 	sub	rsp, 0x28
1417a55a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a55ac: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a55b3: 48 85 d2                    	test	rdx, rdx
1417a55b6: 74 16                       	je	0x1417a55ce <.text+0x17a45ce>
1417a55b8: 48 c1 e2 05                 	shl	rdx, 0x5
1417a55bc: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a55c2: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a55c9: e8 52 e6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a55ce: 90                          	nop
1417a55cf: 48 83 c4 28                 	add	rsp, 0x28
1417a55d3: 5b                          	pop	rbx
1417a55d4: 5f                          	pop	rdi
1417a55d5: 5e                          	pop	rsi
1417a55d6: 41 5c                       	pop	r12
1417a55d8: 41 5d                       	pop	r13
1417a55da: 41 5e                       	pop	r14
1417a55dc: 41 5f                       	pop	r15
1417a55de: 5d                          	pop	rbp
1417a55df: c3                          	ret
1417a55e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a55e5: 55                          	push	rbp
1417a55e6: 41 57                       	push	r15
1417a55e8: 41 56                       	push	r14
1417a55ea: 41 55                       	push	r13
1417a55ec: 41 54                       	push	r12
1417a55ee: 56                          	push	rsi
1417a55ef: 57                          	push	rdi
1417a55f0: 53                          	push	rbx
1417a55f1: 48 83 ec 28                 	sub	rsp, 0x28
1417a55f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a55fc: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5603: 48 85 d2                    	test	rdx, rdx
1417a5606: 74 16                       	je	0x1417a561e <.text+0x17a461e>
1417a5608: 48 c1 e2 05                 	shl	rdx, 0x5
1417a560c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5612: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5619: e8 02 e6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a561e: 90                          	nop
1417a561f: 48 83 c4 28                 	add	rsp, 0x28
1417a5623: 5b                          	pop	rbx
1417a5624: 5f                          	pop	rdi
1417a5625: 5e                          	pop	rsi
1417a5626: 41 5c                       	pop	r12
1417a5628: 41 5d                       	pop	r13
1417a562a: 41 5e                       	pop	r14
1417a562c: 41 5f                       	pop	r15
1417a562e: 5d                          	pop	rbp
1417a562f: c3                          	ret
1417a5630: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5635: 55                          	push	rbp
1417a5636: 41 57                       	push	r15
1417a5638: 41 56                       	push	r14
1417a563a: 41 55                       	push	r13
1417a563c: 41 54                       	push	r12
1417a563e: 56                          	push	rsi
1417a563f: 57                          	push	rdi
1417a5640: 53                          	push	rbx
1417a5641: 48 83 ec 28                 	sub	rsp, 0x28
1417a5645: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a564c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5653: 48 85 d2                    	test	rdx, rdx
1417a5656: 74 16                       	je	0x1417a566e <.text+0x17a466e>
1417a5658: 48 c1 e2 05                 	shl	rdx, 0x5
1417a565c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5662: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5669: e8 b2 e5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a566e: 90                          	nop
1417a566f: 48 83 c4 28                 	add	rsp, 0x28
1417a5673: 5b                          	pop	rbx
1417a5674: 5f                          	pop	rdi
1417a5675: 5e                          	pop	rsi
1417a5676: 41 5c                       	pop	r12
1417a5678: 41 5d                       	pop	r13
1417a567a: 41 5e                       	pop	r14
1417a567c: 41 5f                       	pop	r15
1417a567e: 5d                          	pop	rbp
1417a567f: c3                          	ret
1417a5680: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5685: 55                          	push	rbp
1417a5686: 41 57                       	push	r15
1417a5688: 41 56                       	push	r14
1417a568a: 41 55                       	push	r13
1417a568c: 41 54                       	push	r12
1417a568e: 56                          	push	rsi
1417a568f: 57                          	push	rdi
1417a5690: 53                          	push	rbx
1417a5691: 48 83 ec 28                 	sub	rsp, 0x28
1417a5695: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a569c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a56a3: 48 85 d2                    	test	rdx, rdx
1417a56a6: 74 16                       	je	0x1417a56be <.text+0x17a46be>
1417a56a8: 48 c1 e2 05                 	shl	rdx, 0x5
1417a56ac: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a56b2: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a56b9: e8 62 e5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a56be: 90                          	nop
1417a56bf: 48 83 c4 28                 	add	rsp, 0x28
1417a56c3: 5b                          	pop	rbx
1417a56c4: 5f                          	pop	rdi
1417a56c5: 5e                          	pop	rsi
1417a56c6: 41 5c                       	pop	r12
1417a56c8: 41 5d                       	pop	r13
1417a56ca: 41 5e                       	pop	r14
1417a56cc: 41 5f                       	pop	r15
1417a56ce: 5d                          	pop	rbp
1417a56cf: c3                          	ret
1417a56d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a56d5: 55                          	push	rbp
1417a56d6: 41 57                       	push	r15
1417a56d8: 41 56                       	push	r14
1417a56da: 41 55                       	push	r13
1417a56dc: 41 54                       	push	r12
1417a56de: 56                          	push	rsi
1417a56df: 57                          	push	rdi
1417a56e0: 53                          	push	rbx
1417a56e1: 48 83 ec 28                 	sub	rsp, 0x28
1417a56e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a56ec: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a56f3: 48 85 d2                    	test	rdx, rdx
1417a56f6: 74 16                       	je	0x1417a570e <.text+0x17a470e>
1417a56f8: 48 c1 e2 05                 	shl	rdx, 0x5
1417a56fc: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5702: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5709: e8 12 e5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a570e: 90                          	nop
1417a570f: 48 83 c4 28                 	add	rsp, 0x28
1417a5713: 5b                          	pop	rbx
1417a5714: 5f                          	pop	rdi
1417a5715: 5e                          	pop	rsi
1417a5716: 41 5c                       	pop	r12
1417a5718: 41 5d                       	pop	r13
1417a571a: 41 5e                       	pop	r14
1417a571c: 41 5f                       	pop	r15
1417a571e: 5d                          	pop	rbp
1417a571f: c3                          	ret
1417a5720: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5725: 55                          	push	rbp
1417a5726: 41 57                       	push	r15
1417a5728: 41 56                       	push	r14
1417a572a: 41 55                       	push	r13
1417a572c: 41 54                       	push	r12
1417a572e: 56                          	push	rsi
1417a572f: 57                          	push	rdi
1417a5730: 53                          	push	rbx
1417a5731: 48 83 ec 28                 	sub	rsp, 0x28
1417a5735: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a573c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5743: 48 85 d2                    	test	rdx, rdx
1417a5746: 74 16                       	je	0x1417a575e <.text+0x17a475e>
1417a5748: 48 c1 e2 05                 	shl	rdx, 0x5
1417a574c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5752: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5759: e8 c2 e4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a575e: 90                          	nop
1417a575f: 48 83 c4 28                 	add	rsp, 0x28
1417a5763: 5b                          	pop	rbx
1417a5764: 5f                          	pop	rdi
1417a5765: 5e                          	pop	rsi
1417a5766: 41 5c                       	pop	r12
1417a5768: 41 5d                       	pop	r13
1417a576a: 41 5e                       	pop	r14
1417a576c: 41 5f                       	pop	r15
1417a576e: 5d                          	pop	rbp
1417a576f: c3                          	ret
1417a5770: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5775: 55                          	push	rbp
1417a5776: 41 57                       	push	r15
1417a5778: 41 56                       	push	r14
1417a577a: 41 55                       	push	r13
1417a577c: 41 54                       	push	r12
1417a577e: 56                          	push	rsi
1417a577f: 57                          	push	rdi
1417a5780: 53                          	push	rbx
1417a5781: 48 83 ec 28                 	sub	rsp, 0x28
1417a5785: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a578c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5793: 48 85 d2                    	test	rdx, rdx
1417a5796: 74 16                       	je	0x1417a57ae <.text+0x17a47ae>
1417a5798: 48 c1 e2 05                 	shl	rdx, 0x5
1417a579c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a57a2: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a57a9: e8 72 e4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a57ae: 90                          	nop
1417a57af: 48 83 c4 28                 	add	rsp, 0x28
1417a57b3: 5b                          	pop	rbx
1417a57b4: 5f                          	pop	rdi
1417a57b5: 5e                          	pop	rsi
1417a57b6: 41 5c                       	pop	r12
1417a57b8: 41 5d                       	pop	r13
1417a57ba: 41 5e                       	pop	r14
1417a57bc: 41 5f                       	pop	r15
1417a57be: 5d                          	pop	rbp
1417a57bf: c3                          	ret
1417a57c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a57c5: 55                          	push	rbp
1417a57c6: 41 57                       	push	r15
1417a57c8: 41 56                       	push	r14
1417a57ca: 41 55                       	push	r13
1417a57cc: 41 54                       	push	r12
1417a57ce: 56                          	push	rsi
1417a57cf: 57                          	push	rdi
1417a57d0: 53                          	push	rbx
1417a57d1: 48 83 ec 28                 	sub	rsp, 0x28
1417a57d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a57dc: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a57e3: 48 85 d2                    	test	rdx, rdx
1417a57e6: 74 16                       	je	0x1417a57fe <.text+0x17a47fe>
1417a57e8: 48 c1 e2 05                 	shl	rdx, 0x5
1417a57ec: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a57f2: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a57f9: e8 22 e4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a57fe: 90                          	nop
1417a57ff: 48 83 c4 28                 	add	rsp, 0x28
1417a5803: 5b                          	pop	rbx
1417a5804: 5f                          	pop	rdi
1417a5805: 5e                          	pop	rsi
1417a5806: 41 5c                       	pop	r12
1417a5808: 41 5d                       	pop	r13
1417a580a: 41 5e                       	pop	r14
1417a580c: 41 5f                       	pop	r15
1417a580e: 5d                          	pop	rbp
1417a580f: c3                          	ret
1417a5810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5815: 55                          	push	rbp
1417a5816: 41 57                       	push	r15
1417a5818: 41 56                       	push	r14
1417a581a: 41 55                       	push	r13
1417a581c: 41 54                       	push	r12
1417a581e: 56                          	push	rsi
1417a581f: 57                          	push	rdi
1417a5820: 53                          	push	rbx
1417a5821: 48 83 ec 28                 	sub	rsp, 0x28
1417a5825: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a582c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5833: 48 85 d2                    	test	rdx, rdx
1417a5836: 74 16                       	je	0x1417a584e <.text+0x17a484e>
1417a5838: 48 c1 e2 05                 	shl	rdx, 0x5
1417a583c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5842: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5849: e8 d2 e3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a584e: 90                          	nop
1417a584f: 48 83 c4 28                 	add	rsp, 0x28
1417a5853: 5b                          	pop	rbx
1417a5854: 5f                          	pop	rdi
1417a5855: 5e                          	pop	rsi
1417a5856: 41 5c                       	pop	r12
1417a5858: 41 5d                       	pop	r13
1417a585a: 41 5e                       	pop	r14
1417a585c: 41 5f                       	pop	r15
1417a585e: 5d                          	pop	rbp
1417a585f: c3                          	ret
1417a5860: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5865: 55                          	push	rbp
1417a5866: 41 57                       	push	r15
1417a5868: 41 56                       	push	r14
1417a586a: 41 55                       	push	r13
1417a586c: 41 54                       	push	r12
1417a586e: 56                          	push	rsi
1417a586f: 57                          	push	rdi
1417a5870: 53                          	push	rbx
1417a5871: 48 83 ec 28                 	sub	rsp, 0x28
1417a5875: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a587c: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a5883: 48 85 d2                    	test	rdx, rdx
1417a5886: 74 16                       	je	0x1417a589e <.text+0x17a489e>
1417a5888: 48 c1 e2 05                 	shl	rdx, 0x5
1417a588c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5892: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5899: e8 82 e3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a589e: 90                          	nop
1417a589f: 48 83 c4 28                 	add	rsp, 0x28
1417a58a3: 5b                          	pop	rbx
1417a58a4: 5f                          	pop	rdi
1417a58a5: 5e                          	pop	rsi
1417a58a6: 41 5c                       	pop	r12
1417a58a8: 41 5d                       	pop	r13
1417a58aa: 41 5e                       	pop	r14
1417a58ac: 41 5f                       	pop	r15
1417a58ae: 5d                          	pop	rbp
1417a58af: c3                          	ret
1417a58b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a58b5: 55                          	push	rbp
1417a58b6: 41 57                       	push	r15
1417a58b8: 41 56                       	push	r14
1417a58ba: 41 55                       	push	r13
1417a58bc: 41 54                       	push	r12
1417a58be: 56                          	push	rsi
1417a58bf: 57                          	push	rdi
1417a58c0: 53                          	push	rbx
1417a58c1: 48 83 ec 28                 	sub	rsp, 0x28
1417a58c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a58cc: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1417a58d3: 48 85 d2                    	test	rdx, rdx
1417a58d6: 74 16                       	je	0x1417a58ee <.text+0x17a48ee>
1417a58d8: 48 c1 e2 05                 	shl	rdx, 0x5
1417a58dc: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a58e2: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a58e9: e8 32 e3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a58ee: 90                          	nop
1417a58ef: 48 83 c4 28                 	add	rsp, 0x28
1417a58f3: 5b                          	pop	rbx
1417a58f4: 5f                          	pop	rdi
1417a58f5: 5e                          	pop	rsi
1417a58f6: 41 5c                       	pop	r12
1417a58f8: 41 5d                       	pop	r13
1417a58fa: 41 5e                       	pop	r14
1417a58fc: 41 5f                       	pop	r15
1417a58fe: 5d                          	pop	rbp
1417a58ff: c3                          	ret
1417a5900: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5905: 55                          	push	rbp
1417a5906: 41 57                       	push	r15
1417a5908: 41 56                       	push	r14
1417a590a: 41 55                       	push	r13
1417a590c: 41 54                       	push	r12
1417a590e: 56                          	push	rsi
1417a590f: 57                          	push	rdi
1417a5910: 53                          	push	rbx
1417a5911: 48 83 ec 28                 	sub	rsp, 0x28
1417a5915: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a591c: 48 8b 95 98 02 00 00        	mov	rdx, qword ptr [rbp + 0x298]
1417a5923: 48 85 d2                    	test	rdx, rdx
1417a5926: 74 16                       	je	0x1417a593e <.text+0x17a493e>
1417a5928: 48 c1 e2 05                 	shl	rdx, 0x5
1417a592c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5932: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5939: e8 e2 e2 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a593e: 90                          	nop
1417a593f: 48 83 c4 28                 	add	rsp, 0x28
1417a5943: 5b                          	pop	rbx
1417a5944: 5f                          	pop	rdi
1417a5945: 5e                          	pop	rsi
1417a5946: 41 5c                       	pop	r12
1417a5948: 41 5d                       	pop	r13
1417a594a: 41 5e                       	pop	r14
1417a594c: 41 5f                       	pop	r15
1417a594e: 5d                          	pop	rbp
1417a594f: c3                          	ret
1417a5950: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5955: 55                          	push	rbp
1417a5956: 41 57                       	push	r15
1417a5958: 41 56                       	push	r14
1417a595a: 41 55                       	push	r13
1417a595c: 41 54                       	push	r12
1417a595e: 56                          	push	rsi
1417a595f: 57                          	push	rdi
1417a5960: 53                          	push	rbx
1417a5961: 48 83 ec 28                 	sub	rsp, 0x28
1417a5965: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a596c: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a5973: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a597a: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a5981: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a5988: 48 83 c4 28                 	add	rsp, 0x28
1417a598c: 5b                          	pop	rbx
1417a598d: 5f                          	pop	rdi
1417a598e: 5e                          	pop	rsi
1417a598f: 41 5c                       	pop	r12
1417a5991: 41 5d                       	pop	r13
1417a5993: 41 5e                       	pop	r14
1417a5995: 41 5f                       	pop	r15
1417a5997: 5d                          	pop	rbp
1417a5998: c3                          	ret
1417a5999: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1417a59a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a59a5: 55                          	push	rbp
1417a59a6: 41 57                       	push	r15
1417a59a8: 41 56                       	push	r14
1417a59aa: 41 55                       	push	r13
1417a59ac: 41 54                       	push	r12
1417a59ae: 56                          	push	rsi
1417a59af: 57                          	push	rdi
1417a59b0: 53                          	push	rbx
1417a59b1: 48 83 ec 28                 	sub	rsp, 0x28
1417a59b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a59bc: 80 bd cc 02 00 00 00        	cmp	byte ptr [rbp + 0x2cc], 0x0
1417a59c3: 74 10                       	je	0x1417a59d5 <.text+0x17a49d5>
1417a59c5: c6 85 2f 03 00 00 01        	mov	byte ptr [rbp + 0x32f], 0x1
1417a59cc: 48 8d 4d 28                 	lea	rcx, [rbp + 0x28]
1417a59d0: e8 2b 20 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a59d5: c6 85 2f 03 00 00 01        	mov	byte ptr [rbp + 0x32f], 0x1
1417a59dc: 48 83 c4 28                 	add	rsp, 0x28
1417a59e0: 5b                          	pop	rbx
1417a59e1: 5f                          	pop	rdi
1417a59e2: 5e                          	pop	rsi
1417a59e3: 41 5c                       	pop	r12
1417a59e5: 41 5d                       	pop	r13
1417a59e7: 41 5e                       	pop	r14
1417a59e9: 41 5f                       	pop	r15
1417a59eb: 5d                          	pop	rbp
1417a59ec: c3                          	ret
1417a59ed: 0f 1f 00                    	nop	dword ptr [rax]
1417a59f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a59f5: 55                          	push	rbp
1417a59f6: 41 57                       	push	r15
1417a59f8: 41 56                       	push	r14
1417a59fa: 41 55                       	push	r13
1417a59fc: 41 54                       	push	r12
1417a59fe: 56                          	push	rsi
1417a59ff: 57                          	push	rdi
1417a5a00: 53                          	push	rbx
1417a5a01: 48 83 ec 28                 	sub	rsp, 0x28
1417a5a05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5a0c: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
1417a5a13: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1417a5a1a: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1417a5a1e: 48 85 d2                    	test	rdx, rdx
1417a5a21: 74 10                       	je	0x1417a5a33 <.text+0x17a4a33>
1417a5a23: 48 8b 85 40 02 00 00        	mov	rax, qword ptr [rbp + 0x240]
1417a5a2a: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1417a5a2e: e8 ed e1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a5a33: ba 18 00 00 00              	mov	edx, 0x18
1417a5a38: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5a3e: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
1417a5a45: e8 d6 e1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a5a4a: 8b 85 30 03 00 00           	mov	eax, dword ptr [rbp + 0x330]
1417a5a50: 88 85 1f 03 00 00           	mov	byte ptr [rbp + 0x31f], al
1417a5a56: c6 85 1e 03 00 00 01        	mov	byte ptr [rbp + 0x31e], 0x1
1417a5a5d: 48 83 c4 28                 	add	rsp, 0x28
1417a5a61: 5b                          	pop	rbx
1417a5a62: 5f                          	pop	rdi
1417a5a63: 5e                          	pop	rsi
1417a5a64: 41 5c                       	pop	r12
1417a5a66: 41 5d                       	pop	r13
1417a5a68: 41 5e                       	pop	r14
1417a5a6a: 41 5f                       	pop	r15
1417a5a6c: 5d                          	pop	rbp
1417a5a6d: c3                          	ret
1417a5a6e: 66 90                       	nop
1417a5a70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5a75: 55                          	push	rbp
1417a5a76: 41 57                       	push	r15
1417a5a78: 41 56                       	push	r14
1417a5a7a: 41 55                       	push	r13
1417a5a7c: 41 54                       	push	r12
1417a5a7e: 56                          	push	rsi
1417a5a7f: 57                          	push	rdi
1417a5a80: 53                          	push	rbx
1417a5a81: 48 83 ec 28                 	sub	rsp, 0x28
1417a5a85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5a8c: 48 8b 95 08 02 00 00        	mov	rdx, qword ptr [rbp + 0x208]
1417a5a93: 48 85 d2                    	test	rdx, rdx
1417a5a96: 74 16                       	je	0x1417a5aae <.text+0x17a4aae>
1417a5a98: 48 c1 e2 05                 	shl	rdx, 0x5
1417a5a9c: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a5aa2: 48 8b 8d 40 02 00 00        	mov	rcx, qword ptr [rbp + 0x240]
1417a5aa9: e8 72 e1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a5aae: 8b 85 30 03 00 00           	mov	eax, dword ptr [rbp + 0x330]
1417a5ab4: 88 85 3d 03 00 00           	mov	byte ptr [rbp + 0x33d], al
1417a5aba: 8b 85 88 02 00 00           	mov	eax, dword ptr [rbp + 0x288]
1417a5ac0: 88 85 3c 03 00 00           	mov	byte ptr [rbp + 0x33c], al
1417a5ac6: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a5acd: 48 83 c4 28                 	add	rsp, 0x28
1417a5ad1: 5b                          	pop	rbx
1417a5ad2: 5f                          	pop	rdi
1417a5ad3: 5e                          	pop	rsi
1417a5ad4: 41 5c                       	pop	r12
1417a5ad6: 41 5d                       	pop	r13
1417a5ad8: 41 5e                       	pop	r14
1417a5ada: 41 5f                       	pop	r15
1417a5adc: 5d                          	pop	rbp
1417a5add: c3                          	ret
1417a5ade: 66 90                       	nop
1417a5ae0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5ae5: 55                          	push	rbp
1417a5ae6: 41 57                       	push	r15
1417a5ae8: 41 56                       	push	r14
1417a5aea: 41 55                       	push	r13
1417a5aec: 41 54                       	push	r12
1417a5aee: 56                          	push	rsi
1417a5aef: 57                          	push	rdi
1417a5af0: 53                          	push	rbx
1417a5af1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5af5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5afc: 48 8b 85 30 03 00 00        	mov	rax, qword ptr [rbp + 0x330]
1417a5b03: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a5b0a: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a5b11: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a5b18: 48 83 c4 28                 	add	rsp, 0x28
1417a5b1c: 5b                          	pop	rbx
1417a5b1d: 5f                          	pop	rdi
1417a5b1e: 5e                          	pop	rsi
1417a5b1f: 41 5c                       	pop	r12
1417a5b21: 41 5d                       	pop	r13
1417a5b23: 41 5e                       	pop	r14
1417a5b25: 41 5f                       	pop	r15
1417a5b27: 5d                          	pop	rbp
1417a5b28: c3                          	ret
1417a5b29: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1417a5b30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5b35: 55                          	push	rbp
1417a5b36: 41 57                       	push	r15
1417a5b38: 41 56                       	push	r14
1417a5b3a: 41 55                       	push	r13
1417a5b3c: 41 54                       	push	r12
1417a5b3e: 56                          	push	rsi
1417a5b3f: 57                          	push	rdi
1417a5b40: 53                          	push	rbx
1417a5b41: 48 83 ec 28                 	sub	rsp, 0x28
1417a5b45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5b4c: 80 bd 2e 03 00 00 00        	cmp	byte ptr [rbp + 0x32e], 0x0
1417a5b53: 74 1e                       	je	0x1417a5b73 <.text+0x17a4b73>
1417a5b55: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5b5c: e8 2f bd 0d ff              	call	0x140881890 <.text+0x880890>
1417a5b61: 90                          	nop
1417a5b62: 48 83 c4 28                 	add	rsp, 0x28
1417a5b66: 5b                          	pop	rbx
1417a5b67: 5f                          	pop	rdi
1417a5b68: 5e                          	pop	rsi
1417a5b69: 41 5c                       	pop	r12
1417a5b6b: 41 5d                       	pop	r13
1417a5b6d: 41 5e                       	pop	r14
1417a5b6f: 41 5f                       	pop	r15
1417a5b71: 5d                          	pop	rbp
1417a5b72: c3                          	ret
1417a5b73: 48 83 c4 28                 	add	rsp, 0x28
1417a5b77: 5b                          	pop	rbx
1417a5b78: 5f                          	pop	rdi
1417a5b79: 5e                          	pop	rsi
1417a5b7a: 41 5c                       	pop	r12
1417a5b7c: 41 5d                       	pop	r13
1417a5b7e: 41 5e                       	pop	r14
1417a5b80: 41 5f                       	pop	r15
1417a5b82: 5d                          	pop	rbp
1417a5b83: c3                          	ret
1417a5b84: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1417a5b90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5b95: 55                          	push	rbp
1417a5b96: 41 57                       	push	r15
1417a5b98: 41 56                       	push	r14
1417a5b9a: 41 55                       	push	r13
1417a5b9c: 41 54                       	push	r12
1417a5b9e: 56                          	push	rsi
1417a5b9f: 57                          	push	rdi
1417a5ba0: 53                          	push	rbx
1417a5ba1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5ba5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5bac: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1417a5bb3: e8 38 50 0d ff              	call	0x14087abf0 <.text+0x879bf0>
1417a5bb8: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a5bbf: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a5bc6: 48 83 c4 28                 	add	rsp, 0x28
1417a5bca: 5b                          	pop	rbx
1417a5bcb: 5f                          	pop	rdi
1417a5bcc: 5e                          	pop	rsi
1417a5bcd: 41 5c                       	pop	r12
1417a5bcf: 41 5d                       	pop	r13
1417a5bd1: 41 5e                       	pop	r14
1417a5bd3: 41 5f                       	pop	r15
1417a5bd5: 5d                          	pop	rbp
1417a5bd6: c3                          	ret
1417a5bd7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5be0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5be5: 55                          	push	rbp
1417a5be6: 41 57                       	push	r15
1417a5be8: 41 56                       	push	r14
1417a5bea: 41 55                       	push	r13
1417a5bec: 41 54                       	push	r12
1417a5bee: 56                          	push	rsi
1417a5bef: 57                          	push	rdi
1417a5bf0: 53                          	push	rbx
1417a5bf1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5bf5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5bfc: ba 06 00 00 00              	mov	edx, 0x6
1417a5c01: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a5c07: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a5c0e: e8 0d e0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a5c13: 90                          	nop
1417a5c14: 48 83 c4 28                 	add	rsp, 0x28
1417a5c18: 5b                          	pop	rbx
1417a5c19: 5f                          	pop	rdi
1417a5c1a: 5e                          	pop	rsi
1417a5c1b: 41 5c                       	pop	r12
1417a5c1d: 41 5d                       	pop	r13
1417a5c1f: 41 5e                       	pop	r14
1417a5c21: 41 5f                       	pop	r15
1417a5c23: 5d                          	pop	rbp
1417a5c24: c3                          	ret
1417a5c25: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1417a5c30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5c35: 55                          	push	rbp
1417a5c36: 41 57                       	push	r15
1417a5c38: 41 56                       	push	r14
1417a5c3a: 41 55                       	push	r13
1417a5c3c: 41 54                       	push	r12
1417a5c3e: 56                          	push	rsi
1417a5c3f: 57                          	push	rdi
1417a5c40: 53                          	push	rbx
1417a5c41: 48 83 ec 28                 	sub	rsp, 0x28
1417a5c45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5c4c: c6 85 21 03 00 00 01        	mov	byte ptr [rbp + 0x321], 0x1
1417a5c53: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5c5a: e8 41 2c e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5c5f: c6 85 21 03 00 00 01        	mov	byte ptr [rbp + 0x321], 0x1
1417a5c66: 48 83 c4 28                 	add	rsp, 0x28
1417a5c6a: 5b                          	pop	rbx
1417a5c6b: 5f                          	pop	rdi
1417a5c6c: 5e                          	pop	rsi
1417a5c6d: 41 5c                       	pop	r12
1417a5c6f: 41 5d                       	pop	r13
1417a5c71: 41 5e                       	pop	r14
1417a5c73: 41 5f                       	pop	r15
1417a5c75: 5d                          	pop	rbp
1417a5c76: c3                          	ret
1417a5c77: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5c80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5c85: 55                          	push	rbp
1417a5c86: 41 57                       	push	r15
1417a5c88: 41 56                       	push	r14
1417a5c8a: 41 55                       	push	r13
1417a5c8c: 41 54                       	push	r12
1417a5c8e: 56                          	push	rsi
1417a5c8f: 57                          	push	rdi
1417a5c90: 53                          	push	rbx
1417a5c91: 48 83 ec 28                 	sub	rsp, 0x28
1417a5c95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5c9c: c6 85 22 03 00 00 01        	mov	byte ptr [rbp + 0x322], 0x1
1417a5ca3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5caa: e8 f1 2b e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5caf: c6 85 22 03 00 00 01        	mov	byte ptr [rbp + 0x322], 0x1
1417a5cb6: 48 83 c4 28                 	add	rsp, 0x28
1417a5cba: 5b                          	pop	rbx
1417a5cbb: 5f                          	pop	rdi
1417a5cbc: 5e                          	pop	rsi
1417a5cbd: 41 5c                       	pop	r12
1417a5cbf: 41 5d                       	pop	r13
1417a5cc1: 41 5e                       	pop	r14
1417a5cc3: 41 5f                       	pop	r15
1417a5cc5: 5d                          	pop	rbp
1417a5cc6: c3                          	ret
1417a5cc7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5cd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5cd5: 55                          	push	rbp
1417a5cd6: 41 57                       	push	r15
1417a5cd8: 41 56                       	push	r14
1417a5cda: 41 55                       	push	r13
1417a5cdc: 41 54                       	push	r12
1417a5cde: 56                          	push	rsi
1417a5cdf: 57                          	push	rdi
1417a5ce0: 53                          	push	rbx
1417a5ce1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5ce5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5cec: c6 85 23 03 00 00 01        	mov	byte ptr [rbp + 0x323], 0x1
1417a5cf3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5cfa: e8 a1 2b e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5cff: c6 85 23 03 00 00 01        	mov	byte ptr [rbp + 0x323], 0x1
1417a5d06: 48 83 c4 28                 	add	rsp, 0x28
1417a5d0a: 5b                          	pop	rbx
1417a5d0b: 5f                          	pop	rdi
1417a5d0c: 5e                          	pop	rsi
1417a5d0d: 41 5c                       	pop	r12
1417a5d0f: 41 5d                       	pop	r13
1417a5d11: 41 5e                       	pop	r14
1417a5d13: 41 5f                       	pop	r15
1417a5d15: 5d                          	pop	rbp
1417a5d16: c3                          	ret
1417a5d17: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5d20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5d25: 55                          	push	rbp
1417a5d26: 41 57                       	push	r15
1417a5d28: 41 56                       	push	r14
1417a5d2a: 41 55                       	push	r13
1417a5d2c: 41 54                       	push	r12
1417a5d2e: 56                          	push	rsi
1417a5d2f: 57                          	push	rdi
1417a5d30: 53                          	push	rbx
1417a5d31: 48 83 ec 28                 	sub	rsp, 0x28
1417a5d35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5d3c: c6 85 24 03 00 00 01        	mov	byte ptr [rbp + 0x324], 0x1
1417a5d43: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5d4a: e8 51 2b e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5d4f: c6 85 24 03 00 00 01        	mov	byte ptr [rbp + 0x324], 0x1
1417a5d56: 48 83 c4 28                 	add	rsp, 0x28
1417a5d5a: 5b                          	pop	rbx
1417a5d5b: 5f                          	pop	rdi
1417a5d5c: 5e                          	pop	rsi
1417a5d5d: 41 5c                       	pop	r12
1417a5d5f: 41 5d                       	pop	r13
1417a5d61: 41 5e                       	pop	r14
1417a5d63: 41 5f                       	pop	r15
1417a5d65: 5d                          	pop	rbp
1417a5d66: c3                          	ret
1417a5d67: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5d70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5d75: 55                          	push	rbp
1417a5d76: 41 57                       	push	r15
1417a5d78: 41 56                       	push	r14
1417a5d7a: 41 55                       	push	r13
1417a5d7c: 41 54                       	push	r12
1417a5d7e: 56                          	push	rsi
1417a5d7f: 57                          	push	rdi
1417a5d80: 53                          	push	rbx
1417a5d81: 48 83 ec 28                 	sub	rsp, 0x28
1417a5d85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5d8c: c6 85 25 03 00 00 01        	mov	byte ptr [rbp + 0x325], 0x1
1417a5d93: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5d9a: e8 01 2b e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5d9f: c6 85 25 03 00 00 01        	mov	byte ptr [rbp + 0x325], 0x1
1417a5da6: 48 83 c4 28                 	add	rsp, 0x28
1417a5daa: 5b                          	pop	rbx
1417a5dab: 5f                          	pop	rdi
1417a5dac: 5e                          	pop	rsi
1417a5dad: 41 5c                       	pop	r12
1417a5daf: 41 5d                       	pop	r13
1417a5db1: 41 5e                       	pop	r14
1417a5db3: 41 5f                       	pop	r15
1417a5db5: 5d                          	pop	rbp
1417a5db6: c3                          	ret
1417a5db7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5dc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5dc5: 55                          	push	rbp
1417a5dc6: 41 57                       	push	r15
1417a5dc8: 41 56                       	push	r14
1417a5dca: 41 55                       	push	r13
1417a5dcc: 41 54                       	push	r12
1417a5dce: 56                          	push	rsi
1417a5dcf: 57                          	push	rdi
1417a5dd0: 53                          	push	rbx
1417a5dd1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5dd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5ddc: c6 85 26 03 00 00 01        	mov	byte ptr [rbp + 0x326], 0x1
1417a5de3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5dea: e8 b1 2a e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5def: c6 85 26 03 00 00 01        	mov	byte ptr [rbp + 0x326], 0x1
1417a5df6: 48 83 c4 28                 	add	rsp, 0x28
1417a5dfa: 5b                          	pop	rbx
1417a5dfb: 5f                          	pop	rdi
1417a5dfc: 5e                          	pop	rsi
1417a5dfd: 41 5c                       	pop	r12
1417a5dff: 41 5d                       	pop	r13
1417a5e01: 41 5e                       	pop	r14
1417a5e03: 41 5f                       	pop	r15
1417a5e05: 5d                          	pop	rbp
1417a5e06: c3                          	ret
1417a5e07: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5e10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5e15: 55                          	push	rbp
1417a5e16: 41 57                       	push	r15
1417a5e18: 41 56                       	push	r14
1417a5e1a: 41 55                       	push	r13
1417a5e1c: 41 54                       	push	r12
1417a5e1e: 56                          	push	rsi
1417a5e1f: 57                          	push	rdi
1417a5e20: 53                          	push	rbx
1417a5e21: 48 83 ec 28                 	sub	rsp, 0x28
1417a5e25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5e2c: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
1417a5e33: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5e3a: e8 61 2a e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5e3f: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
1417a5e46: 48 83 c4 28                 	add	rsp, 0x28
1417a5e4a: 5b                          	pop	rbx
1417a5e4b: 5f                          	pop	rdi
1417a5e4c: 5e                          	pop	rsi
1417a5e4d: 41 5c                       	pop	r12
1417a5e4f: 41 5d                       	pop	r13
1417a5e51: 41 5e                       	pop	r14
1417a5e53: 41 5f                       	pop	r15
1417a5e55: 5d                          	pop	rbp
1417a5e56: c3                          	ret
1417a5e57: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5e60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5e65: 55                          	push	rbp
1417a5e66: 41 57                       	push	r15
1417a5e68: 41 56                       	push	r14
1417a5e6a: 41 55                       	push	r13
1417a5e6c: 41 54                       	push	r12
1417a5e6e: 56                          	push	rsi
1417a5e6f: 57                          	push	rdi
1417a5e70: 53                          	push	rbx
1417a5e71: 48 83 ec 28                 	sub	rsp, 0x28
1417a5e75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5e7c: c6 85 28 03 00 00 01        	mov	byte ptr [rbp + 0x328], 0x1
1417a5e83: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5e8a: e8 11 2a e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5e8f: c6 85 28 03 00 00 01        	mov	byte ptr [rbp + 0x328], 0x1
1417a5e96: 48 83 c4 28                 	add	rsp, 0x28
1417a5e9a: 5b                          	pop	rbx
1417a5e9b: 5f                          	pop	rdi
1417a5e9c: 5e                          	pop	rsi
1417a5e9d: 41 5c                       	pop	r12
1417a5e9f: 41 5d                       	pop	r13
1417a5ea1: 41 5e                       	pop	r14
1417a5ea3: 41 5f                       	pop	r15
1417a5ea5: 5d                          	pop	rbp
1417a5ea6: c3                          	ret
1417a5ea7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5eb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5eb5: 55                          	push	rbp
1417a5eb6: 41 57                       	push	r15
1417a5eb8: 41 56                       	push	r14
1417a5eba: 41 55                       	push	r13
1417a5ebc: 41 54                       	push	r12
1417a5ebe: 56                          	push	rsi
1417a5ebf: 57                          	push	rdi
1417a5ec0: 53                          	push	rbx
1417a5ec1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5ec5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5ecc: c6 85 29 03 00 00 01        	mov	byte ptr [rbp + 0x329], 0x1
1417a5ed3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5eda: e8 c1 29 e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5edf: c6 85 29 03 00 00 01        	mov	byte ptr [rbp + 0x329], 0x1
1417a5ee6: 48 83 c4 28                 	add	rsp, 0x28
1417a5eea: 5b                          	pop	rbx
1417a5eeb: 5f                          	pop	rdi
1417a5eec: 5e                          	pop	rsi
1417a5eed: 41 5c                       	pop	r12
1417a5eef: 41 5d                       	pop	r13
1417a5ef1: 41 5e                       	pop	r14
1417a5ef3: 41 5f                       	pop	r15
1417a5ef5: 5d                          	pop	rbp
1417a5ef6: c3                          	ret
1417a5ef7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5f00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5f05: 55                          	push	rbp
1417a5f06: 41 57                       	push	r15
1417a5f08: 41 56                       	push	r14
1417a5f0a: 41 55                       	push	r13
1417a5f0c: 41 54                       	push	r12
1417a5f0e: 56                          	push	rsi
1417a5f0f: 57                          	push	rdi
1417a5f10: 53                          	push	rbx
1417a5f11: 48 83 ec 28                 	sub	rsp, 0x28
1417a5f15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5f1c: c6 85 2a 03 00 00 01        	mov	byte ptr [rbp + 0x32a], 0x1
1417a5f23: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5f2a: e8 71 29 e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5f2f: c6 85 2a 03 00 00 01        	mov	byte ptr [rbp + 0x32a], 0x1
1417a5f36: 48 83 c4 28                 	add	rsp, 0x28
1417a5f3a: 5b                          	pop	rbx
1417a5f3b: 5f                          	pop	rdi
1417a5f3c: 5e                          	pop	rsi
1417a5f3d: 41 5c                       	pop	r12
1417a5f3f: 41 5d                       	pop	r13
1417a5f41: 41 5e                       	pop	r14
1417a5f43: 41 5f                       	pop	r15
1417a5f45: 5d                          	pop	rbp
1417a5f46: c3                          	ret
1417a5f47: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5f50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5f55: 55                          	push	rbp
1417a5f56: 41 57                       	push	r15
1417a5f58: 41 56                       	push	r14
1417a5f5a: 41 55                       	push	r13
1417a5f5c: 41 54                       	push	r12
1417a5f5e: 56                          	push	rsi
1417a5f5f: 57                          	push	rdi
1417a5f60: 53                          	push	rbx
1417a5f61: 48 83 ec 28                 	sub	rsp, 0x28
1417a5f65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5f6c: c6 85 2b 03 00 00 01        	mov	byte ptr [rbp + 0x32b], 0x1
1417a5f73: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5f7a: e8 21 29 e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5f7f: c6 85 2b 03 00 00 01        	mov	byte ptr [rbp + 0x32b], 0x1
1417a5f86: 48 83 c4 28                 	add	rsp, 0x28
1417a5f8a: 5b                          	pop	rbx
1417a5f8b: 5f                          	pop	rdi
1417a5f8c: 5e                          	pop	rsi
1417a5f8d: 41 5c                       	pop	r12
1417a5f8f: 41 5d                       	pop	r13
1417a5f91: 41 5e                       	pop	r14
1417a5f93: 41 5f                       	pop	r15
1417a5f95: 5d                          	pop	rbp
1417a5f96: c3                          	ret
1417a5f97: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5fa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5fa5: 55                          	push	rbp
1417a5fa6: 41 57                       	push	r15
1417a5fa8: 41 56                       	push	r14
1417a5faa: 41 55                       	push	r13
1417a5fac: 41 54                       	push	r12
1417a5fae: 56                          	push	rsi
1417a5faf: 57                          	push	rdi
1417a5fb0: 53                          	push	rbx
1417a5fb1: 48 83 ec 28                 	sub	rsp, 0x28
1417a5fb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a5fbc: c6 85 2c 03 00 00 01        	mov	byte ptr [rbp + 0x32c], 0x1
1417a5fc3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a5fca: e8 d1 28 e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a5fcf: c6 85 2c 03 00 00 01        	mov	byte ptr [rbp + 0x32c], 0x1
1417a5fd6: 48 83 c4 28                 	add	rsp, 0x28
1417a5fda: 5b                          	pop	rbx
1417a5fdb: 5f                          	pop	rdi
1417a5fdc: 5e                          	pop	rsi
1417a5fdd: 41 5c                       	pop	r12
1417a5fdf: 41 5d                       	pop	r13
1417a5fe1: 41 5e                       	pop	r14
1417a5fe3: 41 5f                       	pop	r15
1417a5fe5: 5d                          	pop	rbp
1417a5fe6: c3                          	ret
1417a5fe7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a5ff0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a5ff5: 55                          	push	rbp
1417a5ff6: 41 57                       	push	r15
1417a5ff8: 41 56                       	push	r14
1417a5ffa: 41 55                       	push	r13
1417a5ffc: 41 54                       	push	r12
1417a5ffe: 56                          	push	rsi
1417a5fff: 57                          	push	rdi
1417a6000: 53                          	push	rbx
1417a6001: 48 83 ec 28                 	sub	rsp, 0x28
1417a6005: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a600c: c6 85 2d 03 00 00 01        	mov	byte ptr [rbp + 0x32d], 0x1
1417a6013: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a601a: e8 81 28 e4 fe              	call	0x1405e88a0 <.text+0x5e78a0>
1417a601f: c6 85 2d 03 00 00 01        	mov	byte ptr [rbp + 0x32d], 0x1
1417a6026: 48 83 c4 28                 	add	rsp, 0x28
1417a602a: 5b                          	pop	rbx
1417a602b: 5f                          	pop	rdi
1417a602c: 5e                          	pop	rsi
1417a602d: 41 5c                       	pop	r12
1417a602f: 41 5d                       	pop	r13
1417a6031: 41 5e                       	pop	r14
1417a6033: 41 5f                       	pop	r15
1417a6035: 5d                          	pop	rbp
1417a6036: c3                          	ret
1417a6037: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a6040: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6045: 55                          	push	rbp
1417a6046: 41 57                       	push	r15
1417a6048: 41 56                       	push	r14
1417a604a: 41 55                       	push	r13
1417a604c: 41 54                       	push	r12
1417a604e: 56                          	push	rsi
1417a604f: 57                          	push	rdi
1417a6050: 53                          	push	rbx
1417a6051: 48 83 ec 28                 	sub	rsp, 0x28
1417a6055: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a605c: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1417a6063: 48 c7 00 00 00 00 00        	mov	qword ptr [rax], 0x0
1417a606a: 48 83 c4 28                 	add	rsp, 0x28
1417a606e: 5b                          	pop	rbx
1417a606f: 5f                          	pop	rdi
1417a6070: 5e                          	pop	rsi
1417a6071: 41 5c                       	pop	r12
1417a6073: 41 5d                       	pop	r13
1417a6075: 41 5e                       	pop	r14
1417a6077: 41 5f                       	pop	r15
1417a6079: 5d                          	pop	rbp
1417a607a: c3                          	ret
1417a607b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1417a6080: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6085: 55                          	push	rbp
1417a6086: 41 57                       	push	r15
1417a6088: 41 56                       	push	r14
1417a608a: 41 55                       	push	r13
1417a608c: 41 54                       	push	r12
1417a608e: 56                          	push	rsi
1417a608f: 57                          	push	rdi
1417a6090: 53                          	push	rbx
1417a6091: 48 83 ec 28                 	sub	rsp, 0x28
1417a6095: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a609c: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a60a3: e8 e8 17 e4 fe              	call	0x1405e7890 <.text+0x5e6890>
1417a60a8: 90                          	nop
1417a60a9: 48 83 c4 28                 	add	rsp, 0x28
1417a60ad: 5b                          	pop	rbx
1417a60ae: 5f                          	pop	rdi
1417a60af: 5e                          	pop	rsi
1417a60b0: 41 5c                       	pop	r12
1417a60b2: 41 5d                       	pop	r13
1417a60b4: 41 5e                       	pop	r14
1417a60b6: 41 5f                       	pop	r15
1417a60b8: 5d                          	pop	rbp
1417a60b9: c3                          	ret
1417a60ba: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a60c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a60c5: 55                          	push	rbp
1417a60c6: 41 57                       	push	r15
1417a60c8: 41 56                       	push	r14
1417a60ca: 41 55                       	push	r13
1417a60cc: 41 54                       	push	r12
1417a60ce: 56                          	push	rsi
1417a60cf: 57                          	push	rdi
1417a60d0: 53                          	push	rbx
1417a60d1: 48 83 ec 28                 	sub	rsp, 0x28
1417a60d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a60dc: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a60e3: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a60ea: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a60f1: e8 fa 4a 0d ff              	call	0x14087abf0 <.text+0x879bf0>
1417a60f6: c6 85 3f 03 00 00 01        	mov	byte ptr [rbp + 0x33f], 0x1
1417a60fd: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a6104: 48 83 c4 28                 	add	rsp, 0x28
1417a6108: 5b                          	pop	rbx
1417a6109: 5f                          	pop	rdi
1417a610a: 5e                          	pop	rsi
1417a610b: 41 5c                       	pop	r12
1417a610d: 41 5d                       	pop	r13
1417a610f: 41 5e                       	pop	r14
1417a6111: 41 5f                       	pop	r15
1417a6113: 5d                          	pop	rbp
1417a6114: c3                          	ret
1417a6115: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1417a6120: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6125: 55                          	push	rbp
1417a6126: 41 57                       	push	r15
1417a6128: 41 56                       	push	r14
1417a612a: 41 55                       	push	r13
1417a612c: 41 54                       	push	r12
1417a612e: 56                          	push	rsi
1417a612f: 57                          	push	rdi
1417a6130: 53                          	push	rbx
1417a6131: 48 83 ec 28                 	sub	rsp, 0x28
1417a6135: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a613c: 80 bd 21 03 00 00 00        	cmp	byte ptr [rbp + 0x321], 0x0
1417a6143: 74 30                       	je	0x1417a6175 <.text+0x17a5175>
1417a6145: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a614c: 48 85 d2                    	test	rdx, rdx
1417a614f: 74 12                       	je	0x1417a6163 <.text+0x17a5163>
1417a6151: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6158: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a615e: e8 bd da e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6163: 90                          	nop
1417a6164: 48 83 c4 28                 	add	rsp, 0x28
1417a6168: 5b                          	pop	rbx
1417a6169: 5f                          	pop	rdi
1417a616a: 5e                          	pop	rsi
1417a616b: 41 5c                       	pop	r12
1417a616d: 41 5d                       	pop	r13
1417a616f: 41 5e                       	pop	r14
1417a6171: 41 5f                       	pop	r15
1417a6173: 5d                          	pop	rbp
1417a6174: c3                          	ret
1417a6175: 48 83 c4 28                 	add	rsp, 0x28
1417a6179: 5b                          	pop	rbx
1417a617a: 5f                          	pop	rdi
1417a617b: 5e                          	pop	rsi
1417a617c: 41 5c                       	pop	r12
1417a617e: 41 5d                       	pop	r13
1417a6180: 41 5e                       	pop	r14
1417a6182: 41 5f                       	pop	r15
1417a6184: 5d                          	pop	rbp
1417a6185: c3                          	ret
1417a6186: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6190: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6195: 55                          	push	rbp
1417a6196: 41 57                       	push	r15
1417a6198: 41 56                       	push	r14
1417a619a: 41 55                       	push	r13
1417a619c: 41 54                       	push	r12
1417a619e: 56                          	push	rsi
1417a619f: 57                          	push	rdi
1417a61a0: 53                          	push	rbx
1417a61a1: 48 83 ec 28                 	sub	rsp, 0x28
1417a61a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a61ac: 80 bd 22 03 00 00 00        	cmp	byte ptr [rbp + 0x322], 0x0
1417a61b3: 74 30                       	je	0x1417a61e5 <.text+0x17a51e5>
1417a61b5: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a61bc: 48 85 d2                    	test	rdx, rdx
1417a61bf: 74 12                       	je	0x1417a61d3 <.text+0x17a51d3>
1417a61c1: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a61c8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a61ce: e8 4d da e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a61d3: 90                          	nop
1417a61d4: 48 83 c4 28                 	add	rsp, 0x28
1417a61d8: 5b                          	pop	rbx
1417a61d9: 5f                          	pop	rdi
1417a61da: 5e                          	pop	rsi
1417a61db: 41 5c                       	pop	r12
1417a61dd: 41 5d                       	pop	r13
1417a61df: 41 5e                       	pop	r14
1417a61e1: 41 5f                       	pop	r15
1417a61e3: 5d                          	pop	rbp
1417a61e4: c3                          	ret
1417a61e5: 48 83 c4 28                 	add	rsp, 0x28
1417a61e9: 5b                          	pop	rbx
1417a61ea: 5f                          	pop	rdi
1417a61eb: 5e                          	pop	rsi
1417a61ec: 41 5c                       	pop	r12
1417a61ee: 41 5d                       	pop	r13
1417a61f0: 41 5e                       	pop	r14
1417a61f2: 41 5f                       	pop	r15
1417a61f4: 5d                          	pop	rbp
1417a61f5: c3                          	ret
1417a61f6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6200: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6205: 55                          	push	rbp
1417a6206: 41 57                       	push	r15
1417a6208: 41 56                       	push	r14
1417a620a: 41 55                       	push	r13
1417a620c: 41 54                       	push	r12
1417a620e: 56                          	push	rsi
1417a620f: 57                          	push	rdi
1417a6210: 53                          	push	rbx
1417a6211: 48 83 ec 28                 	sub	rsp, 0x28
1417a6215: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a621c: 80 bd 23 03 00 00 00        	cmp	byte ptr [rbp + 0x323], 0x0
1417a6223: 74 30                       	je	0x1417a6255 <.text+0x17a5255>
1417a6225: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a622c: 48 85 d2                    	test	rdx, rdx
1417a622f: 74 12                       	je	0x1417a6243 <.text+0x17a5243>
1417a6231: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6238: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a623e: e8 dd d9 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6243: 90                          	nop
1417a6244: 48 83 c4 28                 	add	rsp, 0x28
1417a6248: 5b                          	pop	rbx
1417a6249: 5f                          	pop	rdi
1417a624a: 5e                          	pop	rsi
1417a624b: 41 5c                       	pop	r12
1417a624d: 41 5d                       	pop	r13
1417a624f: 41 5e                       	pop	r14
1417a6251: 41 5f                       	pop	r15
1417a6253: 5d                          	pop	rbp
1417a6254: c3                          	ret
1417a6255: 48 83 c4 28                 	add	rsp, 0x28
1417a6259: 5b                          	pop	rbx
1417a625a: 5f                          	pop	rdi
1417a625b: 5e                          	pop	rsi
1417a625c: 41 5c                       	pop	r12
1417a625e: 41 5d                       	pop	r13
1417a6260: 41 5e                       	pop	r14
1417a6262: 41 5f                       	pop	r15
1417a6264: 5d                          	pop	rbp
1417a6265: c3                          	ret
1417a6266: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6270: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6275: 55                          	push	rbp
1417a6276: 41 57                       	push	r15
1417a6278: 41 56                       	push	r14
1417a627a: 41 55                       	push	r13
1417a627c: 41 54                       	push	r12
1417a627e: 56                          	push	rsi
1417a627f: 57                          	push	rdi
1417a6280: 53                          	push	rbx
1417a6281: 48 83 ec 28                 	sub	rsp, 0x28
1417a6285: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a628c: 80 bd 24 03 00 00 00        	cmp	byte ptr [rbp + 0x324], 0x0
1417a6293: 74 30                       	je	0x1417a62c5 <.text+0x17a52c5>
1417a6295: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a629c: 48 85 d2                    	test	rdx, rdx
1417a629f: 74 12                       	je	0x1417a62b3 <.text+0x17a52b3>
1417a62a1: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a62a8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a62ae: e8 6d d9 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a62b3: 90                          	nop
1417a62b4: 48 83 c4 28                 	add	rsp, 0x28
1417a62b8: 5b                          	pop	rbx
1417a62b9: 5f                          	pop	rdi
1417a62ba: 5e                          	pop	rsi
1417a62bb: 41 5c                       	pop	r12
1417a62bd: 41 5d                       	pop	r13
1417a62bf: 41 5e                       	pop	r14
1417a62c1: 41 5f                       	pop	r15
1417a62c3: 5d                          	pop	rbp
1417a62c4: c3                          	ret
1417a62c5: 48 83 c4 28                 	add	rsp, 0x28
1417a62c9: 5b                          	pop	rbx
1417a62ca: 5f                          	pop	rdi
1417a62cb: 5e                          	pop	rsi
1417a62cc: 41 5c                       	pop	r12
1417a62ce: 41 5d                       	pop	r13
1417a62d0: 41 5e                       	pop	r14
1417a62d2: 41 5f                       	pop	r15
1417a62d4: 5d                          	pop	rbp
1417a62d5: c3                          	ret
1417a62d6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a62e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a62e5: 55                          	push	rbp
1417a62e6: 41 57                       	push	r15
1417a62e8: 41 56                       	push	r14
1417a62ea: 41 55                       	push	r13
1417a62ec: 41 54                       	push	r12
1417a62ee: 56                          	push	rsi
1417a62ef: 57                          	push	rdi
1417a62f0: 53                          	push	rbx
1417a62f1: 48 83 ec 28                 	sub	rsp, 0x28
1417a62f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a62fc: 80 bd 25 03 00 00 00        	cmp	byte ptr [rbp + 0x325], 0x0
1417a6303: 74 30                       	je	0x1417a6335 <.text+0x17a5335>
1417a6305: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a630c: 48 85 d2                    	test	rdx, rdx
1417a630f: 74 12                       	je	0x1417a6323 <.text+0x17a5323>
1417a6311: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6318: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a631e: e8 fd d8 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6323: 90                          	nop
1417a6324: 48 83 c4 28                 	add	rsp, 0x28
1417a6328: 5b                          	pop	rbx
1417a6329: 5f                          	pop	rdi
1417a632a: 5e                          	pop	rsi
1417a632b: 41 5c                       	pop	r12
1417a632d: 41 5d                       	pop	r13
1417a632f: 41 5e                       	pop	r14
1417a6331: 41 5f                       	pop	r15
1417a6333: 5d                          	pop	rbp
1417a6334: c3                          	ret
1417a6335: 48 83 c4 28                 	add	rsp, 0x28
1417a6339: 5b                          	pop	rbx
1417a633a: 5f                          	pop	rdi
1417a633b: 5e                          	pop	rsi
1417a633c: 41 5c                       	pop	r12
1417a633e: 41 5d                       	pop	r13
1417a6340: 41 5e                       	pop	r14
1417a6342: 41 5f                       	pop	r15
1417a6344: 5d                          	pop	rbp
1417a6345: c3                          	ret
1417a6346: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6350: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6355: 55                          	push	rbp
1417a6356: 41 57                       	push	r15
1417a6358: 41 56                       	push	r14
1417a635a: 41 55                       	push	r13
1417a635c: 41 54                       	push	r12
1417a635e: 56                          	push	rsi
1417a635f: 57                          	push	rdi
1417a6360: 53                          	push	rbx
1417a6361: 48 83 ec 28                 	sub	rsp, 0x28
1417a6365: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a636c: 80 bd 26 03 00 00 00        	cmp	byte ptr [rbp + 0x326], 0x0
1417a6373: 74 30                       	je	0x1417a63a5 <.text+0x17a53a5>
1417a6375: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a637c: 48 85 d2                    	test	rdx, rdx
1417a637f: 74 12                       	je	0x1417a6393 <.text+0x17a5393>
1417a6381: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6388: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a638e: e8 8d d8 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6393: 90                          	nop
1417a6394: 48 83 c4 28                 	add	rsp, 0x28
1417a6398: 5b                          	pop	rbx
1417a6399: 5f                          	pop	rdi
1417a639a: 5e                          	pop	rsi
1417a639b: 41 5c                       	pop	r12
1417a639d: 41 5d                       	pop	r13
1417a639f: 41 5e                       	pop	r14
1417a63a1: 41 5f                       	pop	r15
1417a63a3: 5d                          	pop	rbp
1417a63a4: c3                          	ret
1417a63a5: 48 83 c4 28                 	add	rsp, 0x28
1417a63a9: 5b                          	pop	rbx
1417a63aa: 5f                          	pop	rdi
1417a63ab: 5e                          	pop	rsi
1417a63ac: 41 5c                       	pop	r12
1417a63ae: 41 5d                       	pop	r13
1417a63b0: 41 5e                       	pop	r14
1417a63b2: 41 5f                       	pop	r15
1417a63b4: 5d                          	pop	rbp
1417a63b5: c3                          	ret
1417a63b6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a63c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a63c5: 55                          	push	rbp
1417a63c6: 41 57                       	push	r15
1417a63c8: 41 56                       	push	r14
1417a63ca: 41 55                       	push	r13
1417a63cc: 41 54                       	push	r12
1417a63ce: 56                          	push	rsi
1417a63cf: 57                          	push	rdi
1417a63d0: 53                          	push	rbx
1417a63d1: 48 83 ec 28                 	sub	rsp, 0x28
1417a63d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a63dc: 80 bd 27 03 00 00 00        	cmp	byte ptr [rbp + 0x327], 0x0
1417a63e3: 74 30                       	je	0x1417a6415 <.text+0x17a5415>
1417a63e5: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a63ec: 48 85 d2                    	test	rdx, rdx
1417a63ef: 74 12                       	je	0x1417a6403 <.text+0x17a5403>
1417a63f1: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a63f8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a63fe: e8 1d d8 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6403: 90                          	nop
1417a6404: 48 83 c4 28                 	add	rsp, 0x28
1417a6408: 5b                          	pop	rbx
1417a6409: 5f                          	pop	rdi
1417a640a: 5e                          	pop	rsi
1417a640b: 41 5c                       	pop	r12
1417a640d: 41 5d                       	pop	r13
1417a640f: 41 5e                       	pop	r14
1417a6411: 41 5f                       	pop	r15
1417a6413: 5d                          	pop	rbp
1417a6414: c3                          	ret
1417a6415: 48 83 c4 28                 	add	rsp, 0x28
1417a6419: 5b                          	pop	rbx
1417a641a: 5f                          	pop	rdi
1417a641b: 5e                          	pop	rsi
1417a641c: 41 5c                       	pop	r12
1417a641e: 41 5d                       	pop	r13
1417a6420: 41 5e                       	pop	r14
1417a6422: 41 5f                       	pop	r15
1417a6424: 5d                          	pop	rbp
1417a6425: c3                          	ret
1417a6426: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6430: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6435: 55                          	push	rbp
1417a6436: 41 57                       	push	r15
1417a6438: 41 56                       	push	r14
1417a643a: 41 55                       	push	r13
1417a643c: 41 54                       	push	r12
1417a643e: 56                          	push	rsi
1417a643f: 57                          	push	rdi
1417a6440: 53                          	push	rbx
1417a6441: 48 83 ec 28                 	sub	rsp, 0x28
1417a6445: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a644c: 80 bd 28 03 00 00 00        	cmp	byte ptr [rbp + 0x328], 0x0
1417a6453: 74 30                       	je	0x1417a6485 <.text+0x17a5485>
1417a6455: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a645c: 48 85 d2                    	test	rdx, rdx
1417a645f: 74 12                       	je	0x1417a6473 <.text+0x17a5473>
1417a6461: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6468: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a646e: e8 ad d7 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6473: 90                          	nop
1417a6474: 48 83 c4 28                 	add	rsp, 0x28
1417a6478: 5b                          	pop	rbx
1417a6479: 5f                          	pop	rdi
1417a647a: 5e                          	pop	rsi
1417a647b: 41 5c                       	pop	r12
1417a647d: 41 5d                       	pop	r13
1417a647f: 41 5e                       	pop	r14
1417a6481: 41 5f                       	pop	r15
1417a6483: 5d                          	pop	rbp
1417a6484: c3                          	ret
1417a6485: 48 83 c4 28                 	add	rsp, 0x28
1417a6489: 5b                          	pop	rbx
1417a648a: 5f                          	pop	rdi
1417a648b: 5e                          	pop	rsi
1417a648c: 41 5c                       	pop	r12
1417a648e: 41 5d                       	pop	r13
1417a6490: 41 5e                       	pop	r14
1417a6492: 41 5f                       	pop	r15
1417a6494: 5d                          	pop	rbp
1417a6495: c3                          	ret
1417a6496: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a64a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a64a5: 55                          	push	rbp
1417a64a6: 41 57                       	push	r15
1417a64a8: 41 56                       	push	r14
1417a64aa: 41 55                       	push	r13
1417a64ac: 41 54                       	push	r12
1417a64ae: 56                          	push	rsi
1417a64af: 57                          	push	rdi
1417a64b0: 53                          	push	rbx
1417a64b1: 48 83 ec 28                 	sub	rsp, 0x28
1417a64b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a64bc: 80 bd 29 03 00 00 00        	cmp	byte ptr [rbp + 0x329], 0x0
1417a64c3: 74 30                       	je	0x1417a64f5 <.text+0x17a54f5>
1417a64c5: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a64cc: 48 85 d2                    	test	rdx, rdx
1417a64cf: 74 12                       	je	0x1417a64e3 <.text+0x17a54e3>
1417a64d1: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a64d8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a64de: e8 3d d7 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a64e3: 90                          	nop
1417a64e4: 48 83 c4 28                 	add	rsp, 0x28
1417a64e8: 5b                          	pop	rbx
1417a64e9: 5f                          	pop	rdi
1417a64ea: 5e                          	pop	rsi
1417a64eb: 41 5c                       	pop	r12
1417a64ed: 41 5d                       	pop	r13
1417a64ef: 41 5e                       	pop	r14
1417a64f1: 41 5f                       	pop	r15
1417a64f3: 5d                          	pop	rbp
1417a64f4: c3                          	ret
1417a64f5: 48 83 c4 28                 	add	rsp, 0x28
1417a64f9: 5b                          	pop	rbx
1417a64fa: 5f                          	pop	rdi
1417a64fb: 5e                          	pop	rsi
1417a64fc: 41 5c                       	pop	r12
1417a64fe: 41 5d                       	pop	r13
1417a6500: 41 5e                       	pop	r14
1417a6502: 41 5f                       	pop	r15
1417a6504: 5d                          	pop	rbp
1417a6505: c3                          	ret
1417a6506: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6510: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6515: 55                          	push	rbp
1417a6516: 41 57                       	push	r15
1417a6518: 41 56                       	push	r14
1417a651a: 41 55                       	push	r13
1417a651c: 41 54                       	push	r12
1417a651e: 56                          	push	rsi
1417a651f: 57                          	push	rdi
1417a6520: 53                          	push	rbx
1417a6521: 48 83 ec 28                 	sub	rsp, 0x28
1417a6525: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a652c: 80 bd 2a 03 00 00 00        	cmp	byte ptr [rbp + 0x32a], 0x0
1417a6533: 74 30                       	je	0x1417a6565 <.text+0x17a5565>
1417a6535: 48 8b 95 d8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d8]
1417a653c: 48 85 d2                    	test	rdx, rdx
1417a653f: 74 12                       	je	0x1417a6553 <.text+0x17a5553>
1417a6541: 48 8b 8d e0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e0]
1417a6548: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a654e: e8 cd d6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6553: 90                          	nop
1417a6554: 48 83 c4 28                 	add	rsp, 0x28
1417a6558: 5b                          	pop	rbx
1417a6559: 5f                          	pop	rdi
1417a655a: 5e                          	pop	rsi
1417a655b: 41 5c                       	pop	r12
1417a655d: 41 5d                       	pop	r13
1417a655f: 41 5e                       	pop	r14
1417a6561: 41 5f                       	pop	r15
1417a6563: 5d                          	pop	rbp
1417a6564: c3                          	ret
1417a6565: 48 83 c4 28                 	add	rsp, 0x28
1417a6569: 5b                          	pop	rbx
1417a656a: 5f                          	pop	rdi
1417a656b: 5e                          	pop	rsi
1417a656c: 41 5c                       	pop	r12
1417a656e: 41 5d                       	pop	r13
1417a6570: 41 5e                       	pop	r14
1417a6572: 41 5f                       	pop	r15
1417a6574: 5d                          	pop	rbp
1417a6575: c3                          	ret
1417a6576: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6580: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6585: 55                          	push	rbp
1417a6586: 41 57                       	push	r15
1417a6588: 41 56                       	push	r14
1417a658a: 41 55                       	push	r13
1417a658c: 41 54                       	push	r12
1417a658e: 56                          	push	rsi
1417a658f: 57                          	push	rdi
1417a6590: 53                          	push	rbx
1417a6591: 48 83 ec 28                 	sub	rsp, 0x28
1417a6595: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a659c: 80 bd 2b 03 00 00 00        	cmp	byte ptr [rbp + 0x32b], 0x0
1417a65a3: 74 30                       	je	0x1417a65d5 <.text+0x17a55d5>
1417a65a5: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a65ac: 48 85 d2                    	test	rdx, rdx
1417a65af: 74 12                       	je	0x1417a65c3 <.text+0x17a55c3>
1417a65b1: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a65b8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a65be: e8 5d d6 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a65c3: 90                          	nop
1417a65c4: 48 83 c4 28                 	add	rsp, 0x28
1417a65c8: 5b                          	pop	rbx
1417a65c9: 5f                          	pop	rdi
1417a65ca: 5e                          	pop	rsi
1417a65cb: 41 5c                       	pop	r12
1417a65cd: 41 5d                       	pop	r13
1417a65cf: 41 5e                       	pop	r14
1417a65d1: 41 5f                       	pop	r15
1417a65d3: 5d                          	pop	rbp
1417a65d4: c3                          	ret
1417a65d5: 48 83 c4 28                 	add	rsp, 0x28
1417a65d9: 5b                          	pop	rbx
1417a65da: 5f                          	pop	rdi
1417a65db: 5e                          	pop	rsi
1417a65dc: 41 5c                       	pop	r12
1417a65de: 41 5d                       	pop	r13
1417a65e0: 41 5e                       	pop	r14
1417a65e2: 41 5f                       	pop	r15
1417a65e4: 5d                          	pop	rbp
1417a65e5: c3                          	ret
1417a65e6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a65f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a65f5: 55                          	push	rbp
1417a65f6: 41 57                       	push	r15
1417a65f8: 41 56                       	push	r14
1417a65fa: 41 55                       	push	r13
1417a65fc: 41 54                       	push	r12
1417a65fe: 56                          	push	rsi
1417a65ff: 57                          	push	rdi
1417a6600: 53                          	push	rbx
1417a6601: 48 83 ec 28                 	sub	rsp, 0x28
1417a6605: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a660c: 80 bd 2c 03 00 00 00        	cmp	byte ptr [rbp + 0x32c], 0x0
1417a6613: 74 30                       	je	0x1417a6645 <.text+0x17a5645>
1417a6615: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a661c: 48 85 d2                    	test	rdx, rdx
1417a661f: 74 12                       	je	0x1417a6633 <.text+0x17a5633>
1417a6621: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6628: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a662e: e8 ed d5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6633: 90                          	nop
1417a6634: 48 83 c4 28                 	add	rsp, 0x28
1417a6638: 5b                          	pop	rbx
1417a6639: 5f                          	pop	rdi
1417a663a: 5e                          	pop	rsi
1417a663b: 41 5c                       	pop	r12
1417a663d: 41 5d                       	pop	r13
1417a663f: 41 5e                       	pop	r14
1417a6641: 41 5f                       	pop	r15
1417a6643: 5d                          	pop	rbp
1417a6644: c3                          	ret
1417a6645: 48 83 c4 28                 	add	rsp, 0x28
1417a6649: 5b                          	pop	rbx
1417a664a: 5f                          	pop	rdi
1417a664b: 5e                          	pop	rsi
1417a664c: 41 5c                       	pop	r12
1417a664e: 41 5d                       	pop	r13
1417a6650: 41 5e                       	pop	r14
1417a6652: 41 5f                       	pop	r15
1417a6654: 5d                          	pop	rbp
1417a6655: c3                          	ret
1417a6656: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a6660: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6665: 55                          	push	rbp
1417a6666: 41 57                       	push	r15
1417a6668: 41 56                       	push	r14
1417a666a: 41 55                       	push	r13
1417a666c: 41 54                       	push	r12
1417a666e: 56                          	push	rsi
1417a666f: 57                          	push	rdi
1417a6670: 53                          	push	rbx
1417a6671: 48 83 ec 28                 	sub	rsp, 0x28
1417a6675: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a667c: 80 bd 2d 03 00 00 00        	cmp	byte ptr [rbp + 0x32d], 0x0
1417a6683: 74 30                       	je	0x1417a66b5 <.text+0x17a56b5>
1417a6685: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a668c: 48 85 d2                    	test	rdx, rdx
1417a668f: 74 12                       	je	0x1417a66a3 <.text+0x17a56a3>
1417a6691: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a6698: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a669e: e8 7d d5 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a66a3: 90                          	nop
1417a66a4: 48 83 c4 28                 	add	rsp, 0x28
1417a66a8: 5b                          	pop	rbx
1417a66a9: 5f                          	pop	rdi
1417a66aa: 5e                          	pop	rsi
1417a66ab: 41 5c                       	pop	r12
1417a66ad: 41 5d                       	pop	r13
1417a66af: 41 5e                       	pop	r14
1417a66b1: 41 5f                       	pop	r15
1417a66b3: 5d                          	pop	rbp
1417a66b4: c3                          	ret
1417a66b5: 48 83 c4 28                 	add	rsp, 0x28
1417a66b9: 5b                          	pop	rbx
1417a66ba: 5f                          	pop	rdi
1417a66bb: 5e                          	pop	rsi
1417a66bc: 41 5c                       	pop	r12
1417a66be: 41 5d                       	pop	r13
1417a66c0: 41 5e                       	pop	r14
1417a66c2: 41 5f                       	pop	r15
1417a66c4: 5d                          	pop	rbp
1417a66c5: c3                          	ret
1417a66c6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1417a66d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a66d5: 55                          	push	rbp
1417a66d6: 41 57                       	push	r15
1417a66d8: 41 56                       	push	r14
1417a66da: 41 55                       	push	r13
1417a66dc: 41 54                       	push	r12
1417a66de: 56                          	push	rsi
1417a66df: 57                          	push	rdi
1417a66e0: 53                          	push	rbx
1417a66e1: 48 83 ec 28                 	sub	rsp, 0x28
1417a66e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a66ec: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1417a66f3: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
1417a66fa: e8 91 b1 0d ff              	call	0x140881890 <.text+0x880890>
1417a66ff: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1417a6706: 48 83 c4 28                 	add	rsp, 0x28
1417a670a: 5b                          	pop	rbx
1417a670b: 5f                          	pop	rdi
1417a670c: 5e                          	pop	rsi
1417a670d: 41 5c                       	pop	r12
1417a670f: 41 5d                       	pop	r13
1417a6711: 41 5e                       	pop	r14
1417a6713: 41 5f                       	pop	r15
1417a6715: 5d                          	pop	rbp
1417a6716: c3                          	ret
1417a6717: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1417a6720: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6725: 55                          	push	rbp
1417a6726: 41 57                       	push	r15
1417a6728: 41 56                       	push	r14
1417a672a: 41 55                       	push	r13
1417a672c: 41 54                       	push	r12
1417a672e: 56                          	push	rsi
1417a672f: 57                          	push	rdi
1417a6730: 53                          	push	rbx
1417a6731: 48 83 ec 28                 	sub	rsp, 0x28
1417a6735: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a673c: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
1417a6743: 48 85 d2                    	test	rdx, rdx
1417a6746: 74 16                       	je	0x1417a675e <.text+0x17a575e>
1417a6748: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1417a674f: 48 c1 e2 04                 	shl	rdx, 0x4
1417a6753: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6759: e8 c2 d4 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a675e: 90                          	nop
1417a675f: 48 83 c4 28                 	add	rsp, 0x28
1417a6763: 5b                          	pop	rbx
1417a6764: 5f                          	pop	rdi
1417a6765: 5e                          	pop	rsi
1417a6766: 41 5c                       	pop	r12
1417a6768: 41 5d                       	pop	r13
1417a676a: 41 5e                       	pop	r14
1417a676c: 41 5f                       	pop	r15
1417a676e: 5d                          	pop	rbp
1417a676f: c3                          	ret
1417a6770: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6775: 55                          	push	rbp
1417a6776: 41 57                       	push	r15
1417a6778: 41 56                       	push	r14
1417a677a: 41 55                       	push	r13
1417a677c: 41 54                       	push	r12
1417a677e: 56                          	push	rsi
1417a677f: 57                          	push	rdi
1417a6780: 53                          	push	rbx
1417a6781: 48 83 ec 28                 	sub	rsp, 0x28
1417a6785: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a678c: 80 bd 2f 03 00 00 00        	cmp	byte ptr [rbp + 0x32f], 0x0
1417a6793: 74 1e                       	je	0x1417a67b3 <.text+0x17a57b3>
1417a6795: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1417a679c: e8 ef b0 0d ff              	call	0x140881890 <.text+0x880890>
1417a67a1: 90                          	nop
1417a67a2: 48 83 c4 28                 	add	rsp, 0x28
1417a67a6: 5b                          	pop	rbx
1417a67a7: 5f                          	pop	rdi
1417a67a8: 5e                          	pop	rsi
1417a67a9: 41 5c                       	pop	r12
1417a67ab: 41 5d                       	pop	r13
1417a67ad: 41 5e                       	pop	r14
1417a67af: 41 5f                       	pop	r15
1417a67b1: 5d                          	pop	rbp
1417a67b2: c3                          	ret
1417a67b3: 48 83 c4 28                 	add	rsp, 0x28
1417a67b7: 5b                          	pop	rbx
1417a67b8: 5f                          	pop	rdi
1417a67b9: 5e                          	pop	rsi
1417a67ba: 41 5c                       	pop	r12
1417a67bc: 41 5d                       	pop	r13
1417a67be: 41 5e                       	pop	r14
1417a67c0: 41 5f                       	pop	r15
1417a67c2: 5d                          	pop	rbp
1417a67c3: c3                          	ret
1417a67c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1417a67d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a67d5: 55                          	push	rbp
1417a67d6: 41 57                       	push	r15
1417a67d8: 41 56                       	push	r14
1417a67da: 41 55                       	push	r13
1417a67dc: 41 54                       	push	r12
1417a67de: 56                          	push	rsi
1417a67df: 57                          	push	rdi
1417a67e0: 53                          	push	rbx
1417a67e1: 48 83 ec 28                 	sub	rsp, 0x28
1417a67e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a67ec: 48 8d 8d 28 02 00 00        	lea	rcx, [rbp + 0x228]
1417a67f3: e8 f8 43 0d ff              	call	0x14087abf0 <.text+0x879bf0>
1417a67f8: 90                          	nop
1417a67f9: 48 83 c4 28                 	add	rsp, 0x28
1417a67fd: 5b                          	pop	rbx
1417a67fe: 5f                          	pop	rdi
1417a67ff: 5e                          	pop	rsi
1417a6800: 41 5c                       	pop	r12
1417a6802: 41 5d                       	pop	r13
1417a6804: 41 5e                       	pop	r14
1417a6806: 41 5f                       	pop	r15
1417a6808: 5d                          	pop	rbp
1417a6809: c3                          	ret
1417a680a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a6810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6815: 55                          	push	rbp
1417a6816: 41 57                       	push	r15
1417a6818: 41 56                       	push	r14
1417a681a: 41 55                       	push	r13
1417a681c: 41 54                       	push	r12
1417a681e: 56                          	push	rsi
1417a681f: 57                          	push	rdi
1417a6820: 53                          	push	rbx
1417a6821: 48 83 ec 28                 	sub	rsp, 0x28
1417a6825: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a682c: 48 83 bd 40 02 00 00 00     	cmp	qword ptr [rbp + 0x240], 0x0
1417a6834: 74 19                       	je	0x1417a684f <.text+0x17a584f>
1417a6836: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a683c: 48 8b 8d 30 03 00 00        	mov	rcx, qword ptr [rbp + 0x330]
1417a6843: 48 8b 95 40 02 00 00        	mov	rdx, qword ptr [rbp + 0x240]
1417a684a: e8 d1 d3 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a684f: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1417a6856: e8 a5 03 0d ff              	call	0x140876c00 <.text+0x875c00>
1417a685b: c6 85 3f 03 00 00 00        	mov	byte ptr [rbp + 0x33f], 0x0
1417a6862: c6 85 3e 03 00 00 01        	mov	byte ptr [rbp + 0x33e], 0x1
1417a6869: 48 83 c4 28                 	add	rsp, 0x28
1417a686d: 5b                          	pop	rbx
1417a686e: 5f                          	pop	rdi
1417a686f: 5e                          	pop	rsi
1417a6870: 41 5c                       	pop	r12
1417a6872: 41 5d                       	pop	r13
1417a6874: 41 5e                       	pop	r14
1417a6876: 41 5f                       	pop	r15
1417a6878: 5d                          	pop	rbp
1417a6879: c3                          	ret
1417a687a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a6880: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6885: 55                          	push	rbp
1417a6886: 41 57                       	push	r15
1417a6888: 41 56                       	push	r14
1417a688a: 41 55                       	push	r13
1417a688c: 41 54                       	push	r12
1417a688e: 56                          	push	rsi
1417a688f: 57                          	push	rdi
1417a6890: 53                          	push	rbx
1417a6891: 48 83 ec 28                 	sub	rsp, 0x28
1417a6895: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a689c: 0f b6 9d 3e 03 00 00        	movzx	ebx, byte ptr [rbp + 0x33e]
1417a68a3: 0f b6 b5 3f 03 00 00        	movzx	esi, byte ptr [rbp + 0x33f]
1417a68aa: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
1417a68b1: e8 da 71 0b ff              	call	0x14085da90 <.text+0x85ca90>
1417a68b6: 40 88 b5 3d 03 00 00        	mov	byte ptr [rbp + 0x33d], sil
1417a68bd: 88 9d 3c 03 00 00           	mov	byte ptr [rbp + 0x33c], bl
1417a68c3: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a68ca: 48 83 c4 28                 	add	rsp, 0x28
1417a68ce: 5b                          	pop	rbx
1417a68cf: 5f                          	pop	rdi
1417a68d0: 5e                          	pop	rsi
1417a68d1: 41 5c                       	pop	r12
1417a68d3: 41 5d                       	pop	r13
1417a68d5: 41 5e                       	pop	r14
1417a68d7: 41 5f                       	pop	r15
1417a68d9: 5d                          	pop	rbp
1417a68da: c3                          	ret
1417a68db: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1417a68e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a68e5: 55                          	push	rbp
1417a68e6: 41 57                       	push	r15
1417a68e8: 41 56                       	push	r14
1417a68ea: 41 55                       	push	r13
1417a68ec: 41 54                       	push	r12
1417a68ee: 56                          	push	rsi
1417a68ef: 57                          	push	rdi
1417a68f0: 53                          	push	rbx
1417a68f1: 48 83 ec 28                 	sub	rsp, 0x28
1417a68f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a68fc: 48 8d 8d 28 02 00 00        	lea	rcx, [rbp + 0x228]
1417a6903: e8 38 26 0f ff              	call	0x140898f40 <.text+0x897f40>
1417a6908: c6 85 3d 03 00 00 01        	mov	byte ptr [rbp + 0x33d], 0x1
1417a690f: c6 85 3c 03 00 00 01        	mov	byte ptr [rbp + 0x33c], 0x1
1417a6916: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1417a691d: 48 83 c4 28                 	add	rsp, 0x28
1417a6921: 5b                          	pop	rbx
1417a6922: 5f                          	pop	rdi
1417a6923: 5e                          	pop	rsi
1417a6924: 41 5c                       	pop	r12
1417a6926: 41 5d                       	pop	r13
1417a6928: 41 5e                       	pop	r14
1417a692a: 41 5f                       	pop	r15
1417a692c: 5d                          	pop	rbp
1417a692d: c3                          	ret
1417a692e: 66 90                       	nop
1417a6930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6935: 55                          	push	rbp
1417a6936: 41 57                       	push	r15
1417a6938: 41 56                       	push	r14
1417a693a: 41 55                       	push	r13
1417a693c: 41 54                       	push	r12
1417a693e: 56                          	push	rsi
1417a693f: 57                          	push	rdi
1417a6940: 53                          	push	rbx
1417a6941: 48 83 ec 28                 	sub	rsp, 0x28
1417a6945: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a694c: 0f b6 9d 3b 03 00 00        	movzx	ebx, byte ptr [rbp + 0x33b]
1417a6953: 0f b6 b5 3d 03 00 00        	movzx	esi, byte ptr [rbp + 0x33d]
1417a695a: f6 85 3c 03 00 00 01        	test	byte ptr [rbp + 0x33c], 0x1
1417a6961: 74 37                       	je	0x1417a699a <.text+0x17a599a>
1417a6963: 48 83 bd 80 00 00 00 00     	cmp	qword ptr [rbp + 0x80], 0x0
1417a696b: 74 0c                       	je	0x1417a6979 <.text+0x17a5979>
1417a696d: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
1417a6974: e8 87 10 e4 fe              	call	0x1405e7a00 <.text+0x5e6a00>
1417a6979: 40 88 b5 1f 03 00 00        	mov	byte ptr [rbp + 0x31f], sil
1417a6980: 80 e3 01                    	and	bl, 0x1
1417a6983: 88 9d 1e 03 00 00           	mov	byte ptr [rbp + 0x31e], bl
1417a6989: 48 83 c4 28                 	add	rsp, 0x28
1417a698d: 5b                          	pop	rbx
1417a698e: 5f                          	pop	rdi
1417a698f: 5e                          	pop	rsi
1417a6990: 41 5c                       	pop	r12
1417a6992: 41 5d                       	pop	r13
1417a6994: 41 5e                       	pop	r14
1417a6996: 41 5f                       	pop	r15
1417a6998: 5d                          	pop	rbp
1417a6999: c3                          	ret
1417a699a: 40 88 b5 1f 03 00 00        	mov	byte ptr [rbp + 0x31f], sil
1417a69a1: 80 e3 01                    	and	bl, 0x1
1417a69a4: 88 9d 1e 03 00 00           	mov	byte ptr [rbp + 0x31e], bl
1417a69aa: 48 83 c4 28                 	add	rsp, 0x28
1417a69ae: 5b                          	pop	rbx
1417a69af: 5f                          	pop	rdi
1417a69b0: 5e                          	pop	rsi
1417a69b1: 41 5c                       	pop	r12
1417a69b3: 41 5d                       	pop	r13
1417a69b5: 41 5e                       	pop	r14
1417a69b7: 41 5f                       	pop	r15
1417a69b9: 5d                          	pop	rbp
1417a69ba: c3                          	ret
1417a69bb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1417a69c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a69c5: 55                          	push	rbp
1417a69c6: 41 57                       	push	r15
1417a69c8: 41 56                       	push	r14
1417a69ca: 41 55                       	push	r13
1417a69cc: 41 54                       	push	r12
1417a69ce: 56                          	push	rsi
1417a69cf: 57                          	push	rdi
1417a69d0: 53                          	push	rbx
1417a69d1: 48 83 ec 28                 	sub	rsp, 0x28
1417a69d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a69dc: 0f b6 9d 1f 03 00 00        	movzx	ebx, byte ptr [rbp + 0x31f]
1417a69e3: 80 bd 1e 03 00 00 00        	cmp	byte ptr [rbp + 0x31e], 0x0
1417a69ea: 74 35                       	je	0x1417a6a21 <.text+0x17a5a21>
1417a69ec: 48 8b 95 f8 00 00 00        	mov	rdx, qword ptr [rbp + 0xf8]
1417a69f3: 48 85 d2                    	test	rdx, rdx
1417a69f6: 74 12                       	je	0x1417a6a0a <.text+0x17a5a0a>
1417a69f8: 48 8b 8d 00 01 00 00        	mov	rcx, qword ptr [rbp + 0x100]
1417a69ff: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a6a05: e8 16 d2 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6a0a: 88 9d 20 03 00 00           	mov	byte ptr [rbp + 0x320], bl
1417a6a10: 48 83 c4 28                 	add	rsp, 0x28
1417a6a14: 5b                          	pop	rbx
1417a6a15: 5f                          	pop	rdi
1417a6a16: 5e                          	pop	rsi
1417a6a17: 41 5c                       	pop	r12
1417a6a19: 41 5d                       	pop	r13
1417a6a1b: 41 5e                       	pop	r14
1417a6a1d: 41 5f                       	pop	r15
1417a6a1f: 5d                          	pop	rbp
1417a6a20: c3                          	ret
1417a6a21: 88 9d 20 03 00 00           	mov	byte ptr [rbp + 0x320], bl
1417a6a27: 48 83 c4 28                 	add	rsp, 0x28
1417a6a2b: 5b                          	pop	rbx
1417a6a2c: 5f                          	pop	rdi
1417a6a2d: 5e                          	pop	rsi
1417a6a2e: 41 5c                       	pop	r12
1417a6a30: 41 5d                       	pop	r13
1417a6a32: 41 5e                       	pop	r14
1417a6a34: 41 5f                       	pop	r15
1417a6a36: 5d                          	pop	rbp
1417a6a37: c3                          	ret
1417a6a38: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1417a6a40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6a45: 55                          	push	rbp
1417a6a46: 41 57                       	push	r15
1417a6a48: 41 56                       	push	r14
1417a6a4a: 41 55                       	push	r13
1417a6a4c: 41 54                       	push	r12
1417a6a4e: 56                          	push	rsi
1417a6a4f: 57                          	push	rdi
1417a6a50: 53                          	push	rbx
1417a6a51: 48 83 ec 28                 	sub	rsp, 0x28
1417a6a55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6a5c: 0f b6 9d 20 03 00 00        	movzx	ebx, byte ptr [rbp + 0x320]
1417a6a63: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
1417a6a67: e8 e4 34 9a fe              	call	0x140149f50 <.text+0x148f50>
1417a6a6c: 88 9d ce 02 00 00           	mov	byte ptr [rbp + 0x2ce], bl
1417a6a72: 48 83 c4 28                 	add	rsp, 0x28
1417a6a76: 5b                          	pop	rbx
1417a6a77: 5f                          	pop	rdi
1417a6a78: 5e                          	pop	rsi
1417a6a79: 41 5c                       	pop	r12
1417a6a7b: 41 5d                       	pop	r13
1417a6a7d: 41 5e                       	pop	r14
1417a6a7f: 41 5f                       	pop	r15
1417a6a81: 5d                          	pop	rbp
1417a6a82: c3                          	ret
1417a6a83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1417a6a90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6a95: 55                          	push	rbp
1417a6a96: 41 57                       	push	r15
1417a6a98: 41 56                       	push	r14
1417a6a9a: 41 55                       	push	r13
1417a6a9c: 41 54                       	push	r12
1417a6a9e: 56                          	push	rsi
1417a6a9f: 57                          	push	rdi
1417a6aa0: 53                          	push	rbx
1417a6aa1: 48 83 ec 28                 	sub	rsp, 0x28
1417a6aa5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6aac: 0f b6 9d ce 02 00 00        	movzx	ebx, byte ptr [rbp + 0x2ce]
1417a6ab3: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1417a6ab7: 48 85 d2                    	test	rdx, rdx
1417a6aba: 74 13                       	je	0x1417a6acf <.text+0x17a5acf>
1417a6abc: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1417a6ac0: 48 c1 e2 03                 	shl	rdx, 0x3
1417a6ac4: 41 b8 04 00 00 00           	mov	r8d, 0x4
1417a6aca: e8 51 d1 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6acf: 48 8d 8d 10 01 00 00        	lea	rcx, [rbp + 0x110]
1417a6ad6: e8 45 88 0d ff              	call	0x14087f320 <.text+0x87e320>
1417a6adb: 88 9d cd 02 00 00           	mov	byte ptr [rbp + 0x2cd], bl
1417a6ae1: 48 83 c4 28                 	add	rsp, 0x28
1417a6ae5: 5b                          	pop	rbx
1417a6ae6: 5f                          	pop	rdi
1417a6ae7: 5e                          	pop	rsi
1417a6ae8: 41 5c                       	pop	r12
1417a6aea: 41 5d                       	pop	r13
1417a6aec: 41 5e                       	pop	r14
1417a6aee: 41 5f                       	pop	r15
1417a6af0: 5d                          	pop	rbp
1417a6af1: c3                          	ret
1417a6af2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1417a6b00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6b05: 55                          	push	rbp
1417a6b06: 41 57                       	push	r15
1417a6b08: 41 56                       	push	r14
1417a6b0a: 41 55                       	push	r13
1417a6b0c: 41 54                       	push	r12
1417a6b0e: 56                          	push	rsi
1417a6b0f: 57                          	push	rdi
1417a6b10: 53                          	push	rbx
1417a6b11: 48 83 ec 28                 	sub	rsp, 0x28
1417a6b15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6b1c: 0f b6 9d cd 02 00 00        	movzx	ebx, byte ptr [rbp + 0x2cd]
1417a6b23: 48 8b 95 70 01 00 00        	mov	rdx, qword ptr [rbp + 0x170]
1417a6b2a: 48 85 d2                    	test	rdx, rdx
1417a6b2d: 74 16                       	je	0x1417a6b45 <.text+0x17a5b45>
1417a6b2f: 48 8b 8d 78 01 00 00        	mov	rcx, qword ptr [rbp + 0x178]
1417a6b36: 48 c1 e2 03                 	shl	rdx, 0x3
1417a6b3a: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6b40: e8 db d0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6b45: 48 8b 95 d0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d0]
1417a6b4c: 48 85 d2                    	test	rdx, rdx
1417a6b4f: 74 12                       	je	0x1417a6b63 <.text+0x17a5b63>
1417a6b51: 41 b8 01 00 00 00           	mov	r8d, 0x1
1417a6b57: 48 8b 8d f8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f8]
1417a6b5e: e8 bd d0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6b63: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
1417a6b67: 48 8d 55 08                 	lea	rdx, [rbp + 0x8]
1417a6b6b: 41 b8 30 00 00 00           	mov	r8d, 0x30
1417a6b71: 41 b9 10 00 00 00           	mov	r9d, 0x10
1417a6b77: e8 f4 89 8a fe              	call	0x14004f570 <.text+0x4e570>
1417a6b7c: 88 9d 1d 03 00 00           	mov	byte ptr [rbp + 0x31d], bl
1417a6b82: 48 83 c4 28                 	add	rsp, 0x28
1417a6b86: 5b                          	pop	rbx
1417a6b87: 5f                          	pop	rdi
1417a6b88: 5e                          	pop	rsi
1417a6b89: 41 5c                       	pop	r12
1417a6b8b: 41 5d                       	pop	r13
1417a6b8d: 41 5e                       	pop	r14
1417a6b8f: 41 5f                       	pop	r15
1417a6b91: 5d                          	pop	rbp
1417a6b92: c3                          	ret
1417a6b93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1417a6ba0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6ba5: 55                          	push	rbp
1417a6ba6: 41 57                       	push	r15
1417a6ba8: 41 56                       	push	r14
1417a6baa: 41 55                       	push	r13
1417a6bac: 41 54                       	push	r12
1417a6bae: 56                          	push	rsi
1417a6baf: 57                          	push	rdi
1417a6bb0: 53                          	push	rbx
1417a6bb1: 48 83 ec 28                 	sub	rsp, 0x28
1417a6bb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6bbc: 0f b6 9d 1d 03 00 00        	movzx	ebx, byte ptr [rbp + 0x31d]
1417a6bc3: 48 8b 95 a0 00 00 00        	mov	rdx, qword ptr [rbp + 0xa0]
1417a6bca: 48 85 d2                    	test	rdx, rdx
1417a6bcd: 75 3b                       	jne	0x1417a6c0a <.text+0x17a5c0a>
1417a6bcf: 48 8b 95 c0 00 00 00        	mov	rdx, qword ptr [rbp + 0xc0]
1417a6bd6: 48 85 d2                    	test	rdx, rdx
1417a6bd9: 75 51                       	jne	0x1417a6c2c <.text+0x17a5c2c>
1417a6bdb: 48 8b 95 e0 00 00 00        	mov	rdx, qword ptr [rbp + 0xe0]
1417a6be2: 48 85 d2                    	test	rdx, rdx
1417a6be5: 75 67                       	jne	0x1417a6c4e <.text+0x17a5c4e>
1417a6be7: f6 c3 01                    	test	bl, 0x1
1417a6bea: 74 7d                       	je	0x1417a6c69 <.text+0x17a5c69>
1417a6bec: 48 8d 8d 90 01 00 00        	lea	rcx, [rbp + 0x190]
1417a6bf3: e8 c8 5e e4 fe              	call	0x1405ecac0 <.text+0x5ebac0>
1417a6bf8: 90                          	nop
1417a6bf9: 48 83 c4 28                 	add	rsp, 0x28
1417a6bfd: 5b                          	pop	rbx
1417a6bfe: 5f                          	pop	rdi
1417a6bff: 5e                          	pop	rsi
1417a6c00: 41 5c                       	pop	r12
1417a6c02: 41 5d                       	pop	r13
1417a6c04: 41 5e                       	pop	r14
1417a6c06: 41 5f                       	pop	r15
1417a6c08: 5d                          	pop	rbp
1417a6c09: c3                          	ret
1417a6c0a: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
1417a6c11: 48 c1 e2 04                 	shl	rdx, 0x4
1417a6c15: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6c1b: e8 00 d0 e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6c20: 48 8b 95 c0 00 00 00        	mov	rdx, qword ptr [rbp + 0xc0]
1417a6c27: 48 85 d2                    	test	rdx, rdx
1417a6c2a: 74 af                       	je	0x1417a6bdb <.text+0x17a5bdb>
1417a6c2c: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
1417a6c33: 48 c1 e2 04                 	shl	rdx, 0x4
1417a6c37: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6c3d: e8 de cf e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6c42: 48 8b 95 e0 00 00 00        	mov	rdx, qword ptr [rbp + 0xe0]
1417a6c49: 48 85 d2                    	test	rdx, rdx
1417a6c4c: 74 99                       	je	0x1417a6be7 <.text+0x17a5be7>
1417a6c4e: 48 8b 8d e8 00 00 00        	mov	rcx, qword ptr [rbp + 0xe8]
1417a6c55: 48 c1 e2 04                 	shl	rdx, 0x4
1417a6c59: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6c5f: e8 bc cf e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6c64: f6 c3 01                    	test	bl, 0x1
1417a6c67: 75 83                       	jne	0x1417a6bec <.text+0x17a5bec>
1417a6c69: 48 83 c4 28                 	add	rsp, 0x28
1417a6c6d: 5b                          	pop	rbx
1417a6c6e: 5f                          	pop	rdi
1417a6c6f: 5e                          	pop	rsi
1417a6c70: 41 5c                       	pop	r12
1417a6c72: 41 5d                       	pop	r13
1417a6c74: 41 5e                       	pop	r14
1417a6c76: 41 5f                       	pop	r15
1417a6c78: 5d                          	pop	rbp
1417a6c79: c3                          	ret
1417a6c7a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a6c80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6c85: 55                          	push	rbp
1417a6c86: 41 57                       	push	r15
1417a6c88: 41 56                       	push	r14
1417a6c8a: 41 55                       	push	r13
1417a6c8c: 41 54                       	push	r12
1417a6c8e: 56                          	push	rsi
1417a6c8f: 57                          	push	rdi
1417a6c90: 53                          	push	rbx
1417a6c91: 48 83 ec 28                 	sub	rsp, 0x28
1417a6c95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6c9c: 48 8d 8d 30 01 00 00        	lea	rcx, [rbp + 0x130]
1417a6ca3: e8 f8 b8 0d ff              	call	0x1408825a0 <.text+0x8815a0>
1417a6ca8: 90                          	nop
1417a6ca9: 48 83 c4 28                 	add	rsp, 0x28
1417a6cad: 5b                          	pop	rbx
1417a6cae: 5f                          	pop	rdi
1417a6caf: 5e                          	pop	rsi
1417a6cb0: 41 5c                       	pop	r12
1417a6cb2: 41 5d                       	pop	r13
1417a6cb4: 41 5e                       	pop	r14
1417a6cb6: 41 5f                       	pop	r15
1417a6cb8: 5d                          	pop	rbp
1417a6cb9: c3                          	ret
1417a6cba: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1417a6cc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6cc5: 55                          	push	rbp
1417a6cc6: 41 57                       	push	r15
1417a6cc8: 41 56                       	push	r14
1417a6cca: 41 55                       	push	r13
1417a6ccc: 41 54                       	push	r12
1417a6cce: 56                          	push	rsi
1417a6ccf: 57                          	push	rdi
1417a6cd0: 53                          	push	rbx
1417a6cd1: 48 83 ec 28                 	sub	rsp, 0x28
1417a6cd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6cdc: 48 8b 85 b0 01 00 00        	mov	rax, qword ptr [rbp + 0x1b0]
1417a6ce3: 48 85 c0                    	test	rax, rax
1417a6ce6: 74 16                       	je	0x1417a6cfe <.text+0x17a5cfe>
1417a6ce8: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1417a6cef: 48 6b d0 58                 	imul	rdx, rax, 0x58
1417a6cf3: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6cf9: e8 22 cf e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6cfe: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1417a6d05: e8 46 8a 0d ff              	call	0x14087f750 <.text+0x87e750>
1417a6d0a: 90                          	nop
1417a6d0b: 48 83 c4 28                 	add	rsp, 0x28
1417a6d0f: 5b                          	pop	rbx
1417a6d10: 5f                          	pop	rdi
1417a6d11: 5e                          	pop	rsi
1417a6d12: 41 5c                       	pop	r12
1417a6d14: 41 5d                       	pop	r13
1417a6d16: 41 5e                       	pop	r14
1417a6d18: 41 5f                       	pop	r15
1417a6d1a: 5d                          	pop	rbp
1417a6d1b: c3                          	ret
1417a6d1c: 0f 1f 40 00                 	nop	dword ptr [rax]
1417a6d20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1417a6d25: 55                          	push	rbp
1417a6d26: 41 57                       	push	r15
1417a6d28: 41 56                       	push	r14
1417a6d2a: 41 55                       	push	r13
1417a6d2c: 41 54                       	push	r12
1417a6d2e: 56                          	push	rsi
1417a6d2f: 57                          	push	rdi
1417a6d30: 53                          	push	rbx
1417a6d31: 48 83 ec 28                 	sub	rsp, 0x28
1417a6d35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1417a6d3c: ba 98 01 00 00              	mov	edx, 0x198
1417a6d41: 41 b8 08 00 00 00           	mov	r8d, 0x8
1417a6d47: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
1417a6d4e: e8 cd ce e6 fe              	call	0x140613c20 <.text+0x612c20>
1417a6d53: 90                          	nop
1417a6d54: 48 83 c4 28                 	add	rsp, 0x28
1417a6d58: 5b                          	pop	rbx
1417a6d59: 5f                          	pop	rdi
1417a6d5a: 5e                          	pop	rsi
1417a6d5b: 41 5c                       	pop	r12
1417a6d5d: 41 5d                       	pop	r13
1417a6d5f: 41 5e                       	pop	r14
1417a6d61: 41 5f                       	pop	r15
1417a6d63: 5d                          	pop	rbp
1417a6d64: c3                          	ret
