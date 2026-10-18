
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141470d00: 55                          	push	rbp
141470d01: 41 57                       	push	r15
141470d03: 41 56                       	push	r14
141470d05: 41 55                       	push	r13
141470d07: 41 54                       	push	r12
141470d09: 56                          	push	rsi
141470d0a: 57                          	push	rdi
141470d0b: 53                          	push	rbx
141470d0c: 48 81 ec 68 04 00 00        	sub	rsp, 0x468
141470d13: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141470d1b: 44 0f 29 bd d0 03 00 00     	movaps	xmmword ptr [rbp + 0x3d0], xmm15
141470d23: 66 44 0f 7f b5 c0 03 00 00  	movdqa	xmmword ptr [rbp + 0x3c0], xmm14
141470d2c: 44 0f 29 ad b0 03 00 00     	movaps	xmmword ptr [rbp + 0x3b0], xmm13
141470d34: 44 0f 29 a5 a0 03 00 00     	movaps	xmmword ptr [rbp + 0x3a0], xmm12
141470d3c: 44 0f 29 9d 90 03 00 00     	movaps	xmmword ptr [rbp + 0x390], xmm11
141470d44: 44 0f 29 95 80 03 00 00     	movaps	xmmword ptr [rbp + 0x380], xmm10
141470d4c: 44 0f 29 8d 70 03 00 00     	movaps	xmmword ptr [rbp + 0x370], xmm9
141470d54: 44 0f 29 85 60 03 00 00     	movaps	xmmword ptr [rbp + 0x360], xmm8
141470d5c: 0f 29 bd 50 03 00 00        	movaps	xmmword ptr [rbp + 0x350], xmm7
141470d63: 0f 29 b5 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm6
141470d6a: 48 c7 85 38 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x338], -0x2
141470d75: 4d 89 ce                    	mov	r14, r9
141470d78: 4c 89 c3                    	mov	rbx, r8
141470d7b: 49 89 d4                    	mov	r12, rdx
141470d7e: 48 89 cf                    	mov	rdi, rcx
141470d81: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
141470d85: e8 b6 83 8e ff              	call	0x140d59140 <.text+0xd58140>
141470d8a: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
141470d8e: 48 8b 75 60                 	mov	rsi, qword ptr [rbp + 0x60]
141470d92: 48 89 c8                    	mov	rax, rcx
141470d95: 48 f7 d8                    	neg	rax
141470d98: 0f 81 8a 00 00 00           	jno	0x141470e28 <.text+0x146fe28>
141470d9e: 48 89 b5 50 02 00 00        	mov	qword ptr [rbp + 0x250], rsi
141470da5: c6 85 2e 03 00 00 01        	mov	byte ptr [rbp + 0x32e], 0x1
141470dac: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141470db3: e8 48 59 8e ff              	call	0x140d56700 <.text+0xd55700>
141470db8: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
141470dbf: 0f 10 8d 70 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x170]
141470dc6: 0f 10 95 80 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x180]
141470dcd: 0f 29 95 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm2
141470dd4: 0f 29 8d 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm1
141470ddb: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
141470de2: 48 8d 05 8a 36 7f 01        	lea	rax, [rip + 0x17f368a]  # 0x142c64473
141470de9: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141470df0: 48 c7 85 68 01 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x168], 0x11
141470dfb: 48 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], rsi
141470e02: c6 85 2e 03 00 00 00        	mov	byte ptr [rbp + 0x32e], 0x0
141470e09: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141470e10: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
141470e17: e8 64 80 43 01              	call	0x1428a8e80 <.text+0x28a7e80>
141470e1c: 48 89 07                    	mov	qword ptr [rdi], rax
141470e1f: c6 47 0f 02                 	mov	byte ptr [rdi + 0xf], 0x2
141470e23: e9 72 06 00 00              	jmp	0x14147149a <.text+0x147049a>
141470e28: 48 89 8d f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rcx
141470e2f: 4c 8b 45 68                 	mov	r8, qword ptr [rbp + 0x68]
141470e33: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
141470e37: 48 89 b5 f8 00 00 00        	mov	qword ptr [rbp + 0xf8], rsi
141470e3e: 48 89 f2                    	mov	rdx, rsi
141470e41: e8 ea 4a f9 ff              	call	0x141405930 <.text+0x1404930>
141470e46: 0f b6 45 58                 	movzx	eax, byte ptr [rbp + 0x58]
141470e4a: 3c 06                       	cmp	al, 0x6
141470e4c: 0f 85 8e 00 00 00           	jne	0x141470ee0 <.text+0x146fee0>
141470e52: 48 8b 75 60                 	mov	rsi, qword ptr [rbp + 0x60]
141470e56: 48 89 b5 50 02 00 00        	mov	qword ptr [rbp + 0x250], rsi
141470e5d: c6 85 2d 03 00 00 01        	mov	byte ptr [rbp + 0x32d], 0x1
141470e64: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141470e6b: e8 90 58 8e ff              	call	0x140d56700 <.text+0xd55700>
141470e70: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
141470e77: 0f 10 8d 70 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x170]
141470e7e: 0f 10 95 80 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x180]
141470e85: 0f 29 95 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm2
141470e8c: 0f 29 8d 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm1
141470e93: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
141470e9a: 48 8d 05 e3 35 7f 01        	lea	rax, [rip + 0x17f35e3]  # 0x142c64484
141470ea1: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141470ea8: 48 c7 85 68 01 00 00 0c 00 00 00    	mov	qword ptr [rbp + 0x168], 0xc
141470eb3: 48 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], rsi
141470eba: c6 85 2d 03 00 00 00        	mov	byte ptr [rbp + 0x32d], 0x0
141470ec1: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141470ec8: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
141470ecf: e8 2c 83 43 01              	call	0x1428a9200 <.text+0x28a8200>
141470ed4: 48 89 07                    	mov	qword ptr [rdi], rax
141470ed7: c6 47 0f 02                 	mov	byte ptr [rdi + 0xf], 0x2
141470edb: e9 9c 05 00 00              	jmp	0x14147147c <.text+0x147047c>
141470ee0: 8b 4d 59                    	mov	ecx, dword ptr [rbp + 0x59]
141470ee3: 8b 55 5c                    	mov	edx, dword ptr [rbp + 0x5c]
141470ee6: 89 95 3c 01 00 00           	mov	dword ptr [rbp + 0x13c], edx
141470eec: 89 8d 39 01 00 00           	mov	dword ptr [rbp + 0x139], ecx
141470ef2: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
141470ef6: f3 0f 6f 45 68              	movdqu	xmm0, xmmword ptr [rbp + 0x68]
141470efb: f3 0f 7f 85 48 01 00 00     	movdqu	xmmword ptr [rbp + 0x148], xmm0
141470f03: 88 85 38 01 00 00           	mov	byte ptr [rbp + 0x138], al
141470f09: 48 89 8d 40 01 00 00        	mov	qword ptr [rbp + 0x140], rcx
141470f10: 48 8d 0d 79 35 7f 01        	lea	rcx, [rip + 0x17f3579]  # 0x142c64490
141470f17: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
141470f1e: ba 07 00 00 00              	mov	edx, 0x7
141470f23: e8 a8 35 00 00              	call	0x1414744d0 <.text+0x14734d0>
141470f28: 48 85 c0                    	test	rax, rax
141470f2b: 75 07                       	jne	0x141470f34 <.text+0x146ff34>
141470f2d: 48 8b 05 24 f8 1b 02        	mov	rax, qword ptr [rip + 0x21bf824] # 0x143630758
141470f34: 0f b6 08                    	movzx	ecx, byte ptr [rax]
141470f37: 48 8d 15 ea 3d 7f 01        	lea	rdx, [rip + 0x17f3dea]  # 0x142c64d28
141470f3e: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
141470f42: 48 01 d1                    	add	rcx, rdx
141470f45: ff e1                       	jmp	rcx
141470f47: c6 85 60 01 00 00 00        	mov	byte ptr [rbp + 0x160], 0x0
141470f4e: e9 ae 00 00 00              	jmp	0x141471001 <.text+0x1470001>
141470f53: 48 83 c0 08                 	add	rax, 0x8
141470f57: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141470f5e: 4c 8d 05 eb 2f 7f 01        	lea	r8, [rip + 0x17f2feb]   # 0x142c63f50
141470f65: 48 89 c2                    	mov	rdx, rax
141470f68: e8 03 73 e8 ff              	call	0x1412f8270 <.text+0x12f7270>
141470f6d: c6 85 60 01 00 00 04        	mov	byte ptr [rbp + 0x160], 0x4
141470f74: e9 88 00 00 00              	jmp	0x141471001 <.text+0x1470001>
141470f79: f3 0f 6f 40 08              	movdqu	xmm0, xmmword ptr [rax + 0x8]
141470f7e: f3 0f 7f 85 68 01 00 00     	movdqu	xmmword ptr [rbp + 0x168], xmm0
141470f86: c6 85 60 01 00 00 02        	mov	byte ptr [rbp + 0x160], 0x2
141470f8d: eb 72                       	jmp	0x141471001 <.text+0x1470001>
141470f8f: 48 83 c0 08                 	add	rax, 0x8
141470f93: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141470f9a: 48 89 c2                    	mov	rdx, rax
141470f9d: e8 ee ad 90 ff              	call	0x140d7bd90 <.text+0xd7ad90>
141470fa2: c6 85 60 01 00 00 03        	mov	byte ptr [rbp + 0x160], 0x3
141470fa9: eb 56                       	jmp	0x141471001 <.text+0x1470001>
141470fab: 0f b6 40 01                 	movzx	eax, byte ptr [rax + 0x1]
141470faf: 88 85 61 01 00 00           	mov	byte ptr [rbp + 0x161], al
141470fb5: c6 85 60 01 00 00 01        	mov	byte ptr [rbp + 0x160], 0x1
141470fbc: eb 43                       	jmp	0x141471001 <.text+0x1470001>
141470fbe: 48 83 78 18 00              	cmp	qword ptr [rax + 0x18], 0x0
141470fc3: 74 1f                       	je	0x141470fe4 <.text+0x146ffe4>
141470fc5: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
141470fc9: 48 85 d2                    	test	rdx, rdx
141470fcc: 0f 84 2e 23 00 00           	je	0x141473300 <.text+0x1472300>
141470fd2: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141470fd9: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
141470fdd: e8 de b8 fc ff              	call	0x14143c8c0 <.text+0x143b8c0>
141470fe2: eb 16                       	jmp	0x141470ffa <.text+0x146fffa>
141470fe4: 48 c7 85 68 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x168], 0x0
141470fef: 48 c7 85 78 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x178], 0x0
141470ffa: c6 85 60 01 00 00 05        	mov	byte ptr [rbp + 0x160], 0x5
141471001: 4c 8d bd 70 02 00 00        	lea	r15, [rbp + 0x270]
141471008: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
14147100f: 4c 89 f9                    	mov	rcx, r15
141471012: e8 d9 98 12 00              	call	0x14159a8f0 <.text+0x15998f0>
141471017: f6 85 70 02 00 00 01        	test	byte ptr [rbp + 0x270], 0x1
14147101e: 74 11                       	je	0x141471031 <.text+0x1470031>
141471020: 48 8b 8d 78 02 00 00        	mov	rcx, qword ptr [rbp + 0x278]
141471027: e8 24 88 43 01              	call	0x1428a9850 <.text+0x28a8850>
14147102c: e9 c7 03 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
141471031: 44 8b ad 74 02 00 00        	mov	r13d, dword ptr [rbp + 0x274]
141471038: 44 89 ad 00 03 00 00        	mov	dword ptr [rbp + 0x300], r13d
14147103f: 41 83 fd 03                 	cmp	r13d, 0x3
141471043: 0f 8d 85 00 00 00           	jge	0x1414710ce <.text+0x14700ce>
141471049: 48 8d 85 00 03 00 00        	lea	rax, [rbp + 0x300]
141471050: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
141471057: 48 8d 05 22 c5 91 ff        	lea	rax, [rip - 0x6e3ade]   # 0x140d8d580 <.text+0xd8c580>
14147105e: 48 89 85 78 02 00 00        	mov	qword ptr [rbp + 0x278], rax
141471065: 48 8d 0d 3c 39 7f 01        	lea	rcx, [rip + 0x17f393c]  # 0x142c649a8
14147106c: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141471073: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
14147107a: 48 8d 05 67 39 7f 01        	lea	rax, [rip + 0x17f3967]  # 0x142c649e8
141471081: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471088: 48 c7 85 68 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x168], 0x2
141471093: 4c 89 bd 70 01 00 00        	mov	qword ptr [rbp + 0x170], r15
14147109a: 48 c7 85 78 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x178], 0x2
1414710a5: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
1414710b0: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1414710b4: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
1414710bb: e8 d0 8e 90 ff              	call	0x140d79f90 <.text+0xd78f90>
1414710c0: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1414710c4: e8 17 73 43 01              	call	0x1428a83e0 <.text+0x28a73e0>
1414710c9: e9 2a 03 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
1414710ce: 41 83 fd 1b                 	cmp	r13d, 0x1b
1414710d2: 0f 86 85 00 00 00           	jbe	0x14147115d <.text+0x147015d>
1414710d8: 48 8d 85 00 03 00 00        	lea	rax, [rbp + 0x300]
1414710df: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
1414710e6: 48 8d 05 93 c4 91 ff        	lea	rax, [rip - 0x6e3b6d]   # 0x140d8d580 <.text+0xd8c580>
1414710ed: 48 89 85 78 02 00 00        	mov	qword ptr [rbp + 0x278], rax
1414710f4: 48 8d 0d e5 34 7f 01        	lea	rcx, [rip + 0x17f34e5]  # 0x142c645e0
1414710fb: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141471102: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
141471109: 48 8d 05 78 38 7f 01        	lea	rax, [rip + 0x17f3878]  # 0x142c64988
141471110: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471117: 48 c7 85 68 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x168], 0x2
141471122: 4c 89 bd 70 01 00 00        	mov	qword ptr [rbp + 0x170], r15
141471129: 48 c7 85 78 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x178], 0x2
141471134: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
14147113f: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
141471143: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
14147114a: e8 41 8e 90 ff              	call	0x140d79f90 <.text+0xd78f90>
14147114f: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
141471153: e8 88 72 43 01              	call	0x1428a83e0 <.text+0x28a73e0>
141471158: e9 9b 02 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
14147115d: 48 8d 0d 33 33 7f 01        	lea	rcx, [rip + 0x17f3333]  # 0x142c64497
141471164: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
14147116b: ba 15 00 00 00              	mov	edx, 0x15
141471170: e8 5b 33 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471175: 48 85 c0                    	test	rax, rax
141471178: 75 07                       	jne	0x141471181 <.text+0x1470181>
14147117a: 48 8b 05 d7 f5 1b 02        	mov	rax, qword ptr [rip + 0x21bf5d7] # 0x143630758
141471181: 0f b6 08                    	movzx	ecx, byte ptr [rax]
141471184: 48 8d 15 b5 3b 7f 01        	lea	rdx, [rip + 0x17f3bb5]  # 0x142c64d40
14147118b: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
14147118f: 48 01 d1                    	add	rcx, rdx
141471192: ff e1                       	jmp	rcx
141471194: c6 85 60 01 00 00 00        	mov	byte ptr [rbp + 0x160], 0x0
14147119b: e9 82 00 00 00              	jmp	0x141471222 <.text+0x1470222>
1414711a0: 48 83 c0 08                 	add	rax, 0x8
1414711a4: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414711ab: 4c 8d 05 9e 2d 7f 01        	lea	r8, [rip + 0x17f2d9e]   # 0x142c63f50
1414711b2: 48 89 c2                    	mov	rdx, rax
1414711b5: e8 b6 70 e8 ff              	call	0x1412f8270 <.text+0x12f7270>
1414711ba: c6 85 60 01 00 00 04        	mov	byte ptr [rbp + 0x160], 0x4
1414711c1: eb 5f                       	jmp	0x141471222 <.text+0x1470222>
1414711c3: f3 0f 6f 40 08              	movdqu	xmm0, xmmword ptr [rax + 0x8]
1414711c8: f3 0f 7f 85 68 01 00 00     	movdqu	xmmword ptr [rbp + 0x168], xmm0
1414711d0: c6 85 60 01 00 00 02        	mov	byte ptr [rbp + 0x160], 0x2
1414711d7: eb 49                       	jmp	0x141471222 <.text+0x1470222>
1414711d9: 48 83 c0 08                 	add	rax, 0x8
1414711dd: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414711e4: 48 89 c2                    	mov	rdx, rax
1414711e7: e8 a4 ab 90 ff              	call	0x140d7bd90 <.text+0xd7ad90>
1414711ec: c6 85 60 01 00 00 03        	mov	byte ptr [rbp + 0x160], 0x3
1414711f3: eb 2d                       	jmp	0x141471222 <.text+0x1470222>
1414711f5: 0f b6 40 01                 	movzx	eax, byte ptr [rax + 0x1]
1414711f9: 88 85 61 01 00 00           	mov	byte ptr [rbp + 0x161], al
1414711ff: c6 85 60 01 00 00 01        	mov	byte ptr [rbp + 0x160], 0x1
141471206: eb 1a                       	jmp	0x141471222 <.text+0x1470222>
141471208: 48 83 c0 08                 	add	rax, 0x8
14147120c: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141471213: 48 89 c2                    	mov	rdx, rax
141471216: e8 a5 bf fc ff              	call	0x14143d1c0 <.text+0x143c1c0>
14147121b: c6 85 60 01 00 00 05        	mov	byte ptr [rbp + 0x160], 0x5
141471222: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471229: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
141471230: e8 bb 96 12 00              	call	0x14159a8f0 <.text+0x15998f0>
141471235: f6 85 70 02 00 00 01        	test	byte ptr [rbp + 0x270], 0x1
14147123c: 74 27                       	je	0x141471265 <.text+0x1470265>
14147123e: 48 8b 85 78 02 00 00        	mov	rax, qword ptr [rbp + 0x278]
141471245: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
141471249: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
14147124d: c7 45 58 01 00 00 00        	mov	dword ptr [rbp + 0x58], 0x1
141471254: c7 85 1c 01 00 00 19 00 00 00       	mov	dword ptr [rbp + 0x11c], 0x19
14147125e: e8 3d 76 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
141471263: eb 0c                       	jmp	0x141471271 <.text+0x1470271>
141471265: 8b 85 74 02 00 00           	mov	eax, dword ptr [rbp + 0x274]
14147126b: 89 85 1c 01 00 00           	mov	dword ptr [rbp + 0x11c], eax
141471271: 48 8d 0d 34 32 7f 01        	lea	rcx, [rip + 0x17f3234]  # 0x142c644ac
141471278: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
14147127f: ba 06 00 00 00              	mov	edx, 0x6
141471284: e8 47 32 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471289: 48 85 c0                    	test	rax, rax
14147128c: 75 07                       	jne	0x141471295 <.text+0x1470295>
14147128e: 48 8b 05 c3 f4 1b 02        	mov	rax, qword ptr [rip + 0x21bf4c3] # 0x143630758
141471295: 0f b6 08                    	movzx	ecx, byte ptr [rax]
141471298: 48 8d 15 b9 3a 7f 01        	lea	rdx, [rip + 0x17f3ab9]  # 0x142c64d58
14147129f: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
1414712a3: 48 01 d1                    	add	rcx, rdx
1414712a6: ff e1                       	jmp	rcx
1414712a8: c6 85 60 01 00 00 00        	mov	byte ptr [rbp + 0x160], 0x0
1414712af: e9 82 00 00 00              	jmp	0x141471336 <.text+0x1470336>
1414712b4: 48 83 c0 08                 	add	rax, 0x8
1414712b8: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414712bf: 4c 8d 05 8a 2c 7f 01        	lea	r8, [rip + 0x17f2c8a]   # 0x142c63f50
1414712c6: 48 89 c2                    	mov	rdx, rax
1414712c9: e8 a2 6f e8 ff              	call	0x1412f8270 <.text+0x12f7270>
1414712ce: c6 85 60 01 00 00 04        	mov	byte ptr [rbp + 0x160], 0x4
1414712d5: eb 5f                       	jmp	0x141471336 <.text+0x1470336>
1414712d7: f3 0f 6f 40 08              	movdqu	xmm0, xmmword ptr [rax + 0x8]
1414712dc: f3 0f 7f 85 68 01 00 00     	movdqu	xmmword ptr [rbp + 0x168], xmm0
1414712e4: c6 85 60 01 00 00 02        	mov	byte ptr [rbp + 0x160], 0x2
1414712eb: eb 49                       	jmp	0x141471336 <.text+0x1470336>
1414712ed: 48 83 c0 08                 	add	rax, 0x8
1414712f1: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414712f8: 48 89 c2                    	mov	rdx, rax
1414712fb: e8 90 aa 90 ff              	call	0x140d7bd90 <.text+0xd7ad90>
141471300: c6 85 60 01 00 00 03        	mov	byte ptr [rbp + 0x160], 0x3
141471307: eb 2d                       	jmp	0x141471336 <.text+0x1470336>
141471309: 0f b6 40 01                 	movzx	eax, byte ptr [rax + 0x1]
14147130d: 88 85 61 01 00 00           	mov	byte ptr [rbp + 0x161], al
141471313: c6 85 60 01 00 00 01        	mov	byte ptr [rbp + 0x160], 0x1
14147131a: eb 1a                       	jmp	0x141471336 <.text+0x1470336>
14147131c: 48 83 c0 08                 	add	rax, 0x8
141471320: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141471327: 48 89 c2                    	mov	rdx, rax
14147132a: e8 91 be fc ff              	call	0x14143d1c0 <.text+0x143c1c0>
14147132f: c6 85 60 01 00 00 05        	mov	byte ptr [rbp + 0x160], 0x5
141471336: 48 8d 05 fb 0e 82 01        	lea	rax, [rip + 0x1820efb]  # 0x142c92238
14147133d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141471342: 48 c7 44 24 28 09 00 00 00  	mov	qword ptr [rsp + 0x28], 0x9
14147134b: 4c 8d 05 74 0e 82 01        	lea	r8, [rip + 0x1820e74]   # 0x142c921c6
141471352: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
141471356: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
14147135d: 41 b9 18 00 00 00           	mov	r9d, 0x18
141471363: e8 c8 01 13 00              	call	0x1415a1530 <.text+0x15a0530>
141471368: 83 7d 58 01                 	cmp	dword ptr [rbp + 0x58], 0x1
14147136c: 0f 85 8d 01 00 00           	jne	0x1414714ff <.text+0x14704ff>
141471372: 48 8b 75 60                 	mov	rsi, qword ptr [rbp + 0x60]
141471376: 48 89 b5 50 02 00 00        	mov	qword ptr [rbp + 0x250], rsi
14147137d: c6 85 2c 03 00 00 01        	mov	byte ptr [rbp + 0x32c], 0x1
141471384: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14147138b: e8 70 53 8e ff              	call	0x140d56700 <.text+0xd55700>
141471390: f3 0f 6f 85 60 01 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x160]
141471398: f3 0f 6f 8d 70 01 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x170]
1414713a0: 0f 10 95 80 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x180]
1414713a7: 0f 29 95 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm2
1414713ae: 66 0f 7f 8d 80 02 00 00     	movdqa	xmmword ptr [rbp + 0x280], xmm1
1414713b6: 66 0f 7f 85 70 02 00 00     	movdqa	xmmword ptr [rbp + 0x270], xmm0
1414713be: 48 8d 05 ed 30 7f 01        	lea	rax, [rip + 0x17f30ed]  # 0x142c644b2
1414713c5: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
1414713cc: 48 c7 85 68 01 00 00 0e 00 00 00    	mov	qword ptr [rbp + 0x168], 0xe
1414713d7: 48 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], rsi
1414713de: c6 85 2c 03 00 00 00        	mov	byte ptr [rbp + 0x32c], 0x0
1414713e5: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414713ec: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1414713f3: e8 08 7e 43 01              	call	0x1428a9200 <.text+0x28a8200>
1414713f8: 48 89 07                    	mov	qword ptr [rdi], rax
1414713fb: c6 47 0f 02                 	mov	byte ptr [rdi + 0xf], 0x2
1414713ff: 0f b6 85 38 01 00 00        	movzx	eax, byte ptr [rbp + 0x138]
141471406: 83 f8 03                    	cmp	eax, 0x3
141471409: 72 71                       	jb	0x14147147c <.text+0x147047c>
14147140b: 74 43                       	je	0x141471450 <.text+0x1470450>
14147140d: 83 f8 04                    	cmp	eax, 0x4
141471410: 75 5e                       	jne	0x141471470 <.text+0x1470470>
141471412: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
141471419: 48 8b 95 50 01 00 00        	mov	rdx, qword ptr [rbp + 0x150]
141471420: 48 89 8d 20 03 00 00        	mov	qword ptr [rbp + 0x320], rcx
141471427: e8 44 79 41 ff              	call	0x140888d70 <.text+0x887d70>
14147142c: 48 8b 95 40 01 00 00        	mov	rdx, qword ptr [rbp + 0x140]
141471433: 48 85 d2                    	test	rdx, rdx
141471436: 74 44                       	je	0x14147147c <.text+0x147047c>
141471438: 48 c1 e2 05                 	shl	rdx, 0x5
14147143c: 41 b8 08 00 00 00           	mov	r8d, 0x8
141471442: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
141471449: e8 d2 27 1a ff              	call	0x140613c20 <.text+0x612c20>
14147144e: eb 2c                       	jmp	0x14147147c <.text+0x147047c>
141471450: 48 8b 95 40 01 00 00        	mov	rdx, qword ptr [rbp + 0x140]
141471457: 48 85 d2                    	test	rdx, rdx
14147145a: 74 20                       	je	0x14147147c <.text+0x147047c>
14147145c: 48 8b 8d 48 01 00 00        	mov	rcx, qword ptr [rbp + 0x148]
141471463: 41 b8 01 00 00 00           	mov	r8d, 0x1
141471469: e8 b2 27 1a ff              	call	0x140613c20 <.text+0x612c20>
14147146e: eb 0c                       	jmp	0x14147147c <.text+0x147047c>
141471470: 48 8d 8d 40 01 00 00        	lea	rcx, [rbp + 0x140]
141471477: e8 c4 7a 42 ff              	call	0x140898f40 <.text+0x897f40>
14147147c: 48 8b 95 f0 00 00 00        	mov	rdx, qword ptr [rbp + 0xf0]
141471483: 48 85 d2                    	test	rdx, rdx
141471486: 74 12                       	je	0x14147149a <.text+0x147049a>
141471488: 41 b8 01 00 00 00           	mov	r8d, 0x1
14147148e: 48 8b 8d f8 00 00 00        	mov	rcx, qword ptr [rbp + 0xf8]
141471495: e8 86 27 1a ff              	call	0x140613c20 <.text+0x612c20>
14147149a: 48 89 f8                    	mov	rax, rdi
14147149d: 0f 28 b5 40 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x340]
1414714a4: 0f 28 bd 50 03 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x350]
1414714ab: 44 0f 28 85 60 03 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x360]
1414714b3: 44 0f 28 8d 70 03 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x370]
1414714bb: 44 0f 28 95 80 03 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x380]
1414714c3: 44 0f 28 9d 90 03 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x390]
1414714cb: 44 0f 28 a5 a0 03 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x3a0]
1414714d3: 44 0f 28 ad b0 03 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x3b0]
1414714db: 44 0f 28 b5 c0 03 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x3c0]
1414714e3: 44 0f 28 bd d0 03 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x3d0]
1414714eb: 48 81 c4 68 04 00 00        	add	rsp, 0x468
1414714f2: 5b                          	pop	rbx
1414714f3: 5f                          	pop	rdi
1414714f4: 5e                          	pop	rsi
1414714f5: 41 5c                       	pop	r12
1414714f7: 41 5d                       	pop	r13
1414714f9: 41 5e                       	pop	r14
1414714fb: 41 5f                       	pop	r15
1414714fd: 5d                          	pop	rbp
1414714fe: c3                          	ret
1414714ff: f3 44 0f 10 5d 5c           	movss	xmm11, dword ptr [rbp + 0x5c]
141471505: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
141471509: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
14147150d: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
141471511: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
141471515: 44 0f 10 55 70              	movups	xmm10, xmmword ptr [rbp + 0x70]
14147151a: 48 8d 0d 9f 2f 7f 01        	lea	rcx, [rip + 0x17f2f9f]  # 0x142c644c0
141471521: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
141471528: ba 0b 00 00 00              	mov	edx, 0xb
14147152d: e8 9e 2f 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471532: 48 85 c0                    	test	rax, rax
141471535: 75 07                       	jne	0x14147153e <.text+0x147053e>
141471537: 48 8b 05 1a f2 1b 02        	mov	rax, qword ptr [rip + 0x21bf21a] # 0x143630758
14147153e: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471545: 48 89 c2                    	mov	rdx, rax
141471548: e8 b3 b9 ff ff              	call	0x14146cf00 <.text+0x146bf00>
14147154d: 48 8d 05 7c cd 67 01        	lea	rax, [rip + 0x167cd7c]  # 0x142aee2d0
141471554: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141471559: 48 c7 44 24 28 03 00 00 00  	mov	qword ptr [rsp + 0x28], 0x3
141471562: 4c 8d 05 2f cd 67 01        	lea	r8, [rip + 0x167cd2f]   # 0x142aee298
141471569: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471570: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
141471577: 41 b9 09 00 00 00           	mov	r9d, 0x9
14147157d: e8 ae e1 12 00              	call	0x14159f730 <.text+0x159e730>
141471582: 8b 85 60 01 00 00           	mov	eax, dword ptr [rbp + 0x160]
141471588: 85 c0                       	test	eax, eax
14147158a: 0f 84 31 01 00 00           	je	0x1414716c1 <.text+0x14706c1>
141471590: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141471597: e8 04 73 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
14147159c: 48 8d 0d 1d 2f 7f 01        	lea	rcx, [rip + 0x17f2f1d]  # 0x142c644c0
1414715a3: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
1414715aa: ba 0b 00 00 00              	mov	edx, 0xb
1414715af: e8 1c 2f 00 00              	call	0x1414744d0 <.text+0x14734d0>
1414715b4: 48 85 c0                    	test	rax, rax
1414715b7: 75 07                       	jne	0x1414715c0 <.text+0x14705c0>
1414715b9: 48 8b 05 98 f1 1b 02        	mov	rax, qword ptr [rip + 0x21bf198] # 0x143630758
1414715c0: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414715c7: 48 89 c2                    	mov	rdx, rax
1414715ca: e8 31 b9 ff ff              	call	0x14146cf00 <.text+0x146bf00>
1414715cf: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1414715d6: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
1414715dd: e8 8e 8d 12 00              	call	0x14159a370 <.text+0x1599370>
1414715e2: 83 bd 70 02 00 00 00        	cmp	dword ptr [rbp + 0x270], 0x0
1414715e9: 0f 84 15 01 00 00           	je	0x141471704 <.text+0x1470704>
1414715ef: 48 8d 8d 78 02 00 00        	lea	rcx, [rbp + 0x278]
1414715f6: e8 a5 72 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
1414715fb: 48 8b 05 a6 ad 1b 02        	mov	rax, qword ptr [rip + 0x21bada6] # 0x14362c3a8
141471602: 48 8b 00                    	mov	rax, qword ptr [rax]
141471605: 44 0f 28 25 e3 26 7f 01     	movaps	xmm12, xmmword ptr [rip + 0x17f26e3] # 0x142c63cf0
14147160d: 44 0f 28 05 eb 26 7f 01     	movaps	xmm8, xmmword ptr [rip + 0x17f26eb] # 0x142c63d00
141471615: c7 85 f4 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x2f4], 0x0
14147161f: 45 0f 57 ed                 	xorps	xmm13, xmm13
141471623: 66 c7 85 16 03 00 00 cf 00  	mov	word ptr [rbp + 0x316], 0xcf
14147162c: 48 83 f8 02                 	cmp	rax, 0x2
141471630: 0f 82 0d 01 00 00           	jb	0x141471743 <.text+0x1470743>
141471636: 48 8d 05 bb 2e 7f 01        	lea	rax, [rip + 0x17f2ebb]  # 0x142c644f8
14147163d: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471644: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
14147164f: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
14147165a: 66 0f ef c0                 	pxor	xmm0, xmm0
14147165e: f3 0f 7f 85 78 01 00 00     	movdqu	xmmword ptr [rbp + 0x178], xmm0
141471666: 48 8d 0d 9b 2e 7f 01        	lea	rcx, [rip + 0x17f2e9b]  # 0x142c64508
14147166d: e8 0e c7 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141471672: 48 8d 0d cf 2a 7f 01        	lea	rcx, [rip + 0x17f2acf]  # 0x142c64148
141471679: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141471680: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
14147168b: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141471692: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
14147169d: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1414716a4: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414716ab: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
1414716b2: ba 02 00 00 00              	mov	edx, 0x2
1414716b7: e8 74 fd 03 ff              	call	0x1404b1430 <.text+0x4b0430>
1414716bc: e9 82 00 00 00              	jmp	0x141471743 <.text+0x1470743>
1414716c1: 8b 85 64 01 00 00           	mov	eax, dword ptr [rbp + 0x164]
1414716c7: 89 85 f4 02 00 00           	mov	dword ptr [rbp + 0x2f4], eax
1414716cd: 44 0f 10 85 68 01 00 00     	movups	xmm8, xmmword ptr [rbp + 0x168]
1414716d5: 44 0f 10 a5 78 01 00 00     	movups	xmm12, xmmword ptr [rbp + 0x178]
1414716dd: f3 44 0f 10 ad 88 01 00 00  	movss	xmm13, dword ptr [rbp + 0x188]
1414716e6: 0f b7 85 8c 01 00 00        	movzx	eax, word ptr [rbp + 0x18c]
1414716ed: 66 89 85 16 03 00 00        	mov	word ptr [rbp + 0x316], ax
1414716f4: 74 4d                       	je	0x141471743 <.text+0x1470743>
1414716f6: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414716fd: e8 9e 71 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
141471702: eb 3f                       	jmp	0x141471743 <.text+0x1470743>
141471704: f3 44 0f 10 85 74 02 00 00  	movss	xmm8, dword ptr [rbp + 0x274]
14147170d: 74 0c                       	je	0x14147171b <.text+0x147071b>
14147170f: 48 8d 8d 78 02 00 00        	lea	rcx, [rbp + 0x278]
141471716: e8 85 71 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
14147171b: 44 0f c6 05 bc 25 7f 01 e2  	shufps	xmm8, xmmword ptr [rip + 0x17f25bc], 0xe2 # xmm8 = xmm8[2,0],mem[2,3]
                                                                        # 0x142c63ce0
141471724: 44 0f 28 25 c4 25 7f 01     	movaps	xmm12, xmmword ptr [rip + 0x17f25c4] # 0x142c63cf0
14147172c: c7 85 f4 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x2f4], 0x0
141471736: 45 0f 57 ed                 	xorps	xmm13, xmm13
14147173a: 66 c7 85 16 03 00 00 cf 00  	mov	word ptr [rbp + 0x316], 0xcf
141471743: 48 8d 0d d6 2d 7f 01        	lea	rcx, [rip + 0x17f2dd6]  # 0x142c64520
14147174a: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
141471751: ba 05 00 00 00              	mov	edx, 0x5
141471756: e8 75 2d 00 00              	call	0x1414744d0 <.text+0x14734d0>
14147175b: 48 85 c0                    	test	rax, rax
14147175e: 75 07                       	jne	0x141471767 <.text+0x1470767>
141471760: 48 8b 05 f1 ef 1b 02        	mov	rax, qword ptr [rip + 0x21beff1] # 0x143630758
141471767: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14147176e: 48 89 c2                    	mov	rdx, rax
141471771: e8 8a b7 ff ff              	call	0x14146cf00 <.text+0x146bf00>
141471776: 48 8d 05 3b 08 82 01        	lea	rax, [rip + 0x182083b]  # 0x142c91fb8
14147177d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141471782: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
14147178b: 4c 8d 05 06 08 82 01        	lea	r8, [rip + 0x1820806]   # 0x142c91f98
141471792: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471799: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
1414717a0: 41 b9 14 00 00 00           	mov	r9d, 0x14
1414717a6: e8 95 09 13 00              	call	0x1415a2140 <.text+0x15a1140>
1414717ab: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414717b2: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1414717b9: e8 f2 b3 ff ff              	call	0x14146cbb0 <.text+0x146bbb0>
1414717be: 31 c0                       	xor	eax, eax
1414717c0: 48 3b 85 60 01 00 00        	cmp	rax, qword ptr [rbp + 0x160]
1414717c7: 71 1c                       	jno	0x1414717e5 <.text+0x14707e5>
1414717c9: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1414717d3: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1414717d7: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1414717de: e8 bd 70 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
1414717e3: eb 18                       	jmp	0x1414717fd <.text+0x14707fd>
1414717e5: 48 8b 85 70 01 00 00        	mov	rax, qword ptr [rbp + 0x170]
1414717ec: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
1414717f0: f3 0f 6f 85 60 01 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x160]
1414717f8: 66 0f 7f 45 40              	movdqa	xmmword ptr [rbp + 0x40], xmm0
1414717fd: 48 8d 0d 21 2d 7f 01        	lea	rcx, [rip + 0x17f2d21]  # 0x142c64525
141471804: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
14147180b: ba 06 00 00 00              	mov	edx, 0x6
141471810: e8 bb 2c 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471815: 48 85 c0                    	test	rax, rax
141471818: 75 07                       	jne	0x141471821 <.text+0x1470821>
14147181a: 48 8b 05 37 ef 1b 02        	mov	rax, qword ptr [rip + 0x21bef37] # 0x143630758
141471821: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471828: 48 89 c2                    	mov	rdx, rax
14147182b: e8 d0 b6 ff ff              	call	0x14146cf00 <.text+0x146bf00>
141471830: 48 8d 05 71 3f 62 01        	lea	rax, [rip + 0x1623f71]  # 0x142a957a8
141471837: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14147183c: 48 c7 44 24 28 0f 00 00 00  	mov	qword ptr [rsp + 0x28], 0xf
141471845: 4c 8d 05 ac 3e 62 01        	lea	r8, [rip + 0x1623eac]   # 0x142a956f8
14147184c: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471853: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
14147185a: 41 b9 17 00 00 00           	mov	r9d, 0x17
141471860: e8 cb e8 12 00              	call	0x1415a0130 <.text+0x159f130>
141471865: 8b 95 60 01 00 00           	mov	edx, dword ptr [rbp + 0x160]
14147186b: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141471872: 83 fa 01                    	cmp	edx, 0x1
141471875: 0f 85 06 01 00 00           	jne	0x141471981 <.text+0x1470981>
14147187b: e8 20 70 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
141471880: 48 8b 05 21 ab 1b 02        	mov	rax, qword ptr [rip + 0x21bab21] # 0x14362c3a8
141471887: 48 8b 00                    	mov	rax, qword ptr [rax]
14147188a: 48 83 f8 03                 	cmp	rax, 0x3
14147188e: 0f 82 86 00 00 00           	jb	0x14147191a <.text+0x147091a>
141471894: 48 8d 05 05 2d 7f 01        	lea	rax, [rip + 0x17f2d05]  # 0x142c645a0
14147189b: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
1414718a2: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
1414718ad: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
1414718b8: 66 0f ef c0                 	pxor	xmm0, xmm0
1414718bc: f3 0f 7f 85 78 01 00 00     	movdqu	xmmword ptr [rbp + 0x178], xmm0
1414718c4: 48 8d 0d e5 2c 7f 01        	lea	rcx, [rip + 0x17f2ce5]  # 0x142c645b0
1414718cb: e8 b0 c4 20 ff              	call	0x14067dd80 <.text+0x67cd80>
1414718d0: 48 8d 0d 71 28 7f 01        	lea	rcx, [rip + 0x17f2871]  # 0x142c64148
1414718d7: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
1414718de: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
1414718e9: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
1414718f0: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
1414718fb: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141471902: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471909: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141471910: ba 03 00 00 00              	mov	edx, 0x3
141471915: e8 16 fb 03 ff              	call	0x1404b1430 <.text+0x4b0430>
14147191a: 41 8b 46 14                 	mov	eax, dword ptr [r14 + 0x14]
14147191e: 89 85 20 01 00 00           	mov	dword ptr [rbp + 0x120], eax
141471924: 66 41 0f 6e 46 18           	movd	xmm0, dword ptr [r14 + 0x18]
14147192a: 66 41 0f 6e 4e 10           	movd	xmm1, dword ptr [r14 + 0x10]
141471930: 66 0f 62 c8                 	punpckldq	xmm1, xmm0      # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
141471934: f3 44 0f 7e f1              	movq	xmm14, xmm1             # xmm14 = xmm1[0],zero
141471939: 0f 28 3d d0 23 7f 01        	movaps	xmm7, xmmword ptr [rip + 0x17f23d0] # 0x142c63d10
141471940: 45 0f 57 c9                 	xorps	xmm9, xmm9
141471944: c6 85 2f 03 00 00 01        	mov	byte ptr [rbp + 0x32f], 0x1
14147194b: f3 44 0f 10 3d b0 3f 4b 01  	movss	xmm15, dword ptr [rip + 0x14b3fb0] # 0x142925904
141471954: c7 85 24 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x124], 0x0
14147195e: c7 85 28 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x128], 0x0
141471968: c7 85 2c 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x12c], 0x0
141471972: c7 85 30 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x130], 0x0
14147197c: e9 11 01 00 00              	jmp	0x141471a92 <.text+0x1470a92>
141471981: 0f 10 bd 64 01 00 00        	movups	xmm7, xmmword ptr [rbp + 0x164]
141471988: 8b 85 74 01 00 00           	mov	eax, dword ptr [rbp + 0x174]
14147198e: 89 85 24 01 00 00           	mov	dword ptr [rbp + 0x124], eax
141471994: 0f b6 85 78 01 00 00        	movzx	eax, byte ptr [rbp + 0x178]
14147199b: 89 85 28 01 00 00           	mov	dword ptr [rbp + 0x128], eax
1414719a1: 8b 85 7c 01 00 00           	mov	eax, dword ptr [rbp + 0x17c]
1414719a7: 89 85 20 01 00 00           	mov	dword ptr [rbp + 0x120], eax
1414719ad: f3 44 0f 6f b5 80 01 00 00  	movdqu	xmm14, xmmword ptr [rbp + 0x180]
1414719b6: 44 0f 10 8d 90 01 00 00     	movups	xmm9, xmmword ptr [rbp + 0x190]
1414719be: f3 44 0f 10 bd a0 01 00 00  	movss	xmm15, dword ptr [rbp + 0x1a0]
1414719c7: 0f b6 85 a4 01 00 00        	movzx	eax, byte ptr [rbp + 0x1a4]
1414719ce: 89 85 2c 01 00 00           	mov	dword ptr [rbp + 0x12c], eax
1414719d4: 0f b6 85 a5 01 00 00        	movzx	eax, byte ptr [rbp + 0x1a5]
1414719db: 89 85 30 01 00 00           	mov	dword ptr [rbp + 0x130], eax
1414719e1: 0f b6 85 a6 01 00 00        	movzx	eax, byte ptr [rbp + 0x1a6]
1414719e8: 88 85 2f 03 00 00           	mov	byte ptr [rbp + 0x32f], al
1414719ee: 48 8b 05 b3 a9 1b 02        	mov	rax, qword ptr [rip + 0x21ba9b3] # 0x14362c3a8
1414719f5: 48 8b 00                    	mov	rax, qword ptr [rax]
1414719f8: 48 83 f8 03                 	cmp	rax, 0x3
1414719fc: 0f 82 87 00 00 00           	jb	0x141471a89 <.text+0x1470a89>
141471a02: 89 95 20 03 00 00           	mov	dword ptr [rbp + 0x320], edx
141471a08: 48 8d 05 41 2b 7f 01        	lea	rax, [rip + 0x17f2b41]  # 0x142c64550
141471a0f: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
141471a16: 48 c7 85 78 02 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x278], 0x1
141471a21: 48 c7 85 80 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x280], 0x8
141471a2c: 66 0f ef c0                 	pxor	xmm0, xmm0
141471a30: f3 0f 7f 85 88 02 00 00     	movdqu	xmmword ptr [rbp + 0x288], xmm0
141471a38: 48 8d 0d 21 2b 7f 01        	lea	rcx, [rip + 0x17f2b21]  # 0x142c64560
141471a3f: e8 3c c3 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141471a44: 48 8d 0d fd 26 7f 01        	lea	rcx, [rip + 0x17f26fd]  # 0x142c64148
141471a4b: 48 89 4d 58                 	mov	qword ptr [rbp + 0x58], rcx
141471a4f: 48 c7 45 60 26 00 00 00     	mov	qword ptr [rbp + 0x60], 0x26
141471a57: 48 89 4d 68                 	mov	qword ptr [rbp + 0x68], rcx
141471a5b: 48 c7 45 70 26 00 00 00     	mov	qword ptr [rbp + 0x70], 0x26
141471a63: 48 89 45 78                 	mov	qword ptr [rbp + 0x78], rax
141471a67: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471a6e: 4c 8d 45 58                 	lea	r8, [rbp + 0x58]
141471a72: ba 03 00 00 00              	mov	edx, 0x3
141471a77: e8 b4 f9 03 ff              	call	0x1404b1430 <.text+0x4b0430>
141471a7c: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141471a83: 8b 95 20 03 00 00           	mov	edx, dword ptr [rbp + 0x320]
141471a89: 85 d2                       	test	edx, edx
141471a8b: 74 05                       	je	0x141471a92 <.text+0x1470a92>
141471a8d: e8 0e 6e 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
141471a92: 48 8d 0d 2f 2b 7f 01        	lea	rcx, [rip + 0x17f2b2f]  # 0x142c645c8
141471a99: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
141471aa0: ba 07 00 00 00              	mov	edx, 0x7
141471aa5: e8 26 2a 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471aaa: 48 85 c0                    	test	rax, rax
141471aad: 74 0b                       	je	0x141471aba <.text+0x1470aba>
141471aaf: 0f b6 08                    	movzx	ecx, byte ptr [rax]
141471ab2: c6 00 00                    	mov	byte ptr [rax], 0x0
141471ab5: 80 f9 06                    	cmp	cl, 0x6
141471ab8: 75 09                       	jne	0x141471ac3 <.text+0x1470ac3>
141471aba: c6 85 50 02 00 00 00        	mov	byte ptr [rbp + 0x250], 0x0
141471ac1: eb 20                       	jmp	0x141471ae3 <.text+0x1470ae3>
141471ac3: 88 8d 50 02 00 00           	mov	byte ptr [rbp + 0x250], cl
141471ac9: f3 0f 6f 40 01              	movdqu	xmm0, xmmword ptr [rax + 0x1]
141471ace: f3 0f 6f 48 10              	movdqu	xmm1, xmmword ptr [rax + 0x10]
141471ad3: f3 0f 7f 85 51 02 00 00     	movdqu	xmmword ptr [rbp + 0x251], xmm0
141471adb: f3 0f 7f 8d 60 02 00 00     	movdqu	xmmword ptr [rbp + 0x260], xmm1
141471ae3: 48 8d 05 66 c5 67 01        	lea	rax, [rip + 0x167c566]  # 0x142aee050
141471aea: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141471aef: 48 c7 44 24 28 04 00 00 00  	mov	qword ptr [rsp + 0x28], 0x4
141471af8: 4c 8d 05 19 c5 67 01        	lea	r8, [rip + 0x167c519]   # 0x142aee018
141471aff: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471b06: 4c 8d b5 50 02 00 00        	lea	r14, [rbp + 0x250]
141471b0d: 41 b9 07 00 00 00           	mov	r9d, 0x7
141471b13: 4c 89 f2                    	mov	rdx, r14
141471b16: e8 15 f4 12 00              	call	0x1415a0f30 <.text+0x159ff30>
141471b1b: 4c 8d 05 ad 2a 7f 01        	lea	r8, [rip + 0x17f2aad]   # 0x142c645cf
141471b22: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141471b29: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
141471b30: 41 b9 0f 00 00 00           	mov	r9d, 0xf
141471b36: e8 95 41 13 00              	call	0x1415a5cd0 <.text+0x15a4cd0>
141471b3b: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
141471b42: 48 8b 95 78 02 00 00        	mov	rdx, qword ptr [rbp + 0x278]
141471b49: 48 89 c1                    	mov	rcx, rax
141471b4c: 48 f7 d9                    	neg	rcx
141471b4f: 71 39                       	jno	0x141471b8a <.text+0x1470b8a>
141471b51: 48 89 17                    	mov	qword ptr [rdi], rdx
141471b54: c6 47 0f 02                 	mov	byte ptr [rdi + 0xf], 0x2
141471b58: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
141471b5c: 48 89 c1                    	mov	rcx, rax
141471b5f: 48 f7 d9                    	neg	rcx
141471b62: 0f 80 97 f8 ff ff           	jo	0x1414713ff <.text+0x14703ff>
141471b68: 0f 83 91 f8 ff ff           	jae	0x1414713ff <.text+0x14703ff>
141471b6e: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
141471b72: 48 c1 e0 03                 	shl	rax, 0x3
141471b76: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
141471b7a: 41 b8 04 00 00 00           	mov	r8d, 0x4
141471b80: e8 9b 20 1a ff              	call	0x140613c20 <.text+0x612c20>
141471b85: e9 75 f8 ff ff              	jmp	0x1414713ff <.text+0x14703ff>
141471b8a: 48 89 5d 28                 	mov	qword ptr [rbp + 0x28], rbx
141471b8e: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
141471b95: 48 89 8d c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rcx
141471b9c: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
141471ba3: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
141471baa: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
141471bb1: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
141471bb8: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
141471bbf: 0f 10 8d 90 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x290]
141471bc6: 0f 10 95 a0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2a0]
141471bcd: 0f 10 9d b0 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2b0]
141471bd4: 0f 11 9d 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm3
141471bdb: 0f 11 95 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm2
141471be2: 0f 11 4d 78                 	movups	xmmword ptr [rbp + 0x78], xmm1
141471be6: 0f 11 45 68                 	movups	xmmword ptr [rbp + 0x68], xmm0
141471bea: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
141471bee: 48 89 55 60                 	mov	qword ptr [rbp + 0x60], rdx
141471bf2: 8b 85 c0 00 00 00           	mov	eax, dword ptr [rbp + 0xc0]
141471bf8: 89 85 d4 00 00 00           	mov	dword ptr [rbp + 0xd4], eax
141471bfe: c6 85 36 03 00 00 00        	mov	byte ptr [rbp + 0x336], 0x0
141471c05: c7 85 f0 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x2f0], 0x0
141471c0f: b8 00 00 00 00              	mov	eax, 0x0
141471c14: bb 00 00 00 00              	mov	ebx, 0x0
141471c19: 41 83 fd 1b                 	cmp	r13d, 0x1b
141471c1d: 0f 85 19 01 00 00           	jne	0x141471d3c <.text+0x1470d3c>
141471c23: 89 85 04 03 00 00           	mov	dword ptr [rbp + 0x304], eax
141471c29: 48 8b 05 78 a7 1b 02        	mov	rax, qword ptr [rip + 0x21ba778] # 0x14362c3a8
141471c30: 48 8b 00                    	mov	rax, qword ptr [rax]
141471c33: 48 83 f8 03                 	cmp	rax, 0x3
141471c37: 0f 82 84 00 00 00           	jb	0x141471cc1 <.text+0x1470cc1>
141471c3d: 48 8d 05 f4 2a 7f 01        	lea	rax, [rip + 0x17f2af4]  # 0x142c64738
141471c44: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471c4b: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
141471c56: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
141471c61: 0f 57 c0                    	xorps	xmm0, xmm0
141471c64: 0f 11 85 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm0
141471c6b: 48 8d 0d d6 2a 7f 01        	lea	rcx, [rip + 0x17f2ad6]  # 0x142c64748
141471c72: e8 09 c1 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141471c77: 48 8d 0d ca 24 7f 01        	lea	rcx, [rip + 0x17f24ca]  # 0x142c64148
141471c7e: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141471c85: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
141471c90: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141471c97: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
141471ca2: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141471ca9: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471cb0: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141471cb7: ba 03 00 00 00              	mov	edx, 0x3
141471cbc: e8 6f f7 03 ff              	call	0x1404b1430 <.text+0x4b0430>
141471cc1: 48 8d 0d 00 2b 7f 01        	lea	rcx, [rip + 0x17f2b00]  # 0x142c647c8
141471cc8: 4c 8d 85 38 01 00 00        	lea	r8, [rbp + 0x138]
141471ccf: ba 08 00 00 00              	mov	edx, 0x8
141471cd4: e8 f7 27 00 00              	call	0x1414744d0 <.text+0x14734d0>
141471cd9: 49 89 c7                    	mov	r15, rax
141471cdc: 48 8d 05 fd 2a 7f 01        	lea	rax, [rip + 0x17f2afd]  # 0x142c647e0
141471ce3: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471cea: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
141471cf5: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
141471d00: 0f 57 c0                    	xorps	xmm0, xmm0
141471d03: 0f 11 85 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm0
141471d0a: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471d11: e8 0a 63 43 01              	call	0x1428a8020 <.text+0x28a7020>
141471d16: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471d1d: 4d 85 ff                    	test	r15, r15
141471d20: 0f 84 b8 0d 00 00           	je	0x141472ade <.text+0x1471ade>
141471d26: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471d2d: e8 fe ae 8d ff              	call	0x140d4cc30 <.text+0xd4bc30>
141471d32: 31 c9                       	xor	ecx, ecx
141471d34: 4c 89 f8                    	mov	rax, r15
141471d37: e9 a7 0d 00 00              	jmp	0x141472ae3 <.text+0x1471ae3>
141471d3c: 48 8b 05 65 a6 1b 02        	mov	rax, qword ptr [rip + 0x21ba665] # 0x14362c3a8
141471d43: 48 8b 00                    	mov	rax, qword ptr [rax]
141471d46: 48 83 f8 03                 	cmp	rax, 0x3
141471d4a: 0f 82 c1 00 00 00           	jb	0x141471e11 <.text+0x1470e11>
141471d50: 48 8d 85 00 03 00 00        	lea	rax, [rbp + 0x300]
141471d57: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
141471d5e: 48 8d 05 1b b8 91 ff        	lea	rax, [rip - 0x6e47e5]   # 0x140d8d580 <.text+0xd8c580>
141471d65: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
141471d6c: 48 8d 0d 6d 28 7f 01        	lea	rcx, [rip + 0x17f286d]  # 0x142c645e0
141471d73: 48 89 8d 60 02 00 00        	mov	qword ptr [rbp + 0x260], rcx
141471d7a: 48 89 85 68 02 00 00        	mov	qword ptr [rbp + 0x268], rax
141471d81: 48 8d 05 c0 28 7f 01        	lea	rax, [rip + 0x17f28c0]  # 0x142c64648
141471d88: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141471d8f: 48 c7 85 68 01 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x168], 0x3
141471d9a: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
141471da5: 4c 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], r14
141471dac: 48 c7 85 78 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x178], 0x2
141471db7: 48 8d 0d ba 28 7f 01        	lea	rcx, [rip + 0x17f28ba]  # 0x142c64678
141471dbe: e8 bd bf 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141471dc3: 48 8d 0d 7e 23 7f 01        	lea	rcx, [rip + 0x17f237e]  # 0x142c64148
141471dca: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141471dd1: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
141471ddc: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141471de3: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
141471dee: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141471df5: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141471dfc: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141471e03: ba 03 00 00 00              	mov	edx, 0x3
141471e08: e8 23 f6 03 ff              	call	0x1404b1430 <.text+0x4b0430>
141471e0d: 48 8b 55 60                 	mov	rdx, qword ptr [rbp + 0x60]
141471e11: f3 44 0f 11 bd 34 01 00 00  	movss	dword ptr [rbp + 0x134], xmm15
141471e1a: 44 0f 29 4d f0              	movaps	xmmword ptr [rbp - 0x10], xmm9
141471e1f: 0f 29 7d e0                 	movaps	xmmword ptr [rbp - 0x20], xmm7
141471e23: 48 89 7d 38                 	mov	qword ptr [rbp + 0x38], rdi
141471e27: 41 83 fd 10                 	cmp	r13d, 0x10
141471e2b: 0f 92 85 36 03 00 00        	setb	byte ptr [rbp + 0x336]
141471e32: c6 85 d8 00 00 00 01        	mov	byte ptr [rbp + 0xd8], 0x1
141471e39: 48 c7 85 b0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x0
141471e44: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
141471e48: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
141471e4c: 48 8d 04 c2                 	lea	rax, [rdx + 8*rax]
141471e50: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141471e54: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
141471e5b: 49 be 0e 00 00 00 00 00 00 80       	movabs	r14, -0x7ffffffffffffff2
141471e65: bf 02 00 00 00              	mov	edi, 0x2
141471e6a: f3 44 0f 10 3d 9d 05 61 01  	movss	xmm15, dword ptr [rip + 0x161059d] # 0x142a82410
141471e73: f3 0f 10 35 89 3a 4b 01     	movss	xmm6, dword ptr [rip + 0x14b3a89] # 0x142925904
141471e7b: f3 0f 10 3d f1 e7 4e 01     	movss	xmm7, dword ptr [rip + 0x14ee7f1] # 0x142960674
141471e83: f3 44 0f 10 0d 84 ab 4d 01  	movss	xmm9, dword ptr [rip + 0x14dab84] # 0x14294ca10
141471e8c: c7 85 04 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x304], 0x0
141471e96: c7 85 f0 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x2f0], 0x0
141471ea0: 44 89 ad 30 03 00 00        	mov	dword ptr [rbp + 0x330], r13d
141471ea7: 4c 89 a5 10 01 00 00        	mov	qword ptr [rbp + 0x110], r12
141471eae: 48 3b 55 20                 	cmp	rdx, qword ptr [rbp + 0x20]
141471eb2: 0f 84 a6 10 00 00           	je	0x141472f5e <.text+0x1471f5e>
141471eb8: 48 89 95 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rdx
141471ebf: 48 8b 42 10                 	mov	rax, qword ptr [rdx + 0x10]
141471ec3: 48 85 c0                    	test	rax, rax
141471ec6: 0f 84 65 04 00 00           	je	0x141472331 <.text+0x1471331>
141471ecc: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
141471ed3: 48 8b 59 08                 	mov	rbx, qword ptr [rcx + 0x8]
141471ed7: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
141471edb: 4c 8d 3c c3                 	lea	r15, [rbx + 8*rax]
141471edf: 4c 89 7d 08                 	mov	qword ptr [rbp + 0x8], r15
141471ee3: eb 12                       	jmp	0x141471ef7 <.text+0x1470ef7>
141471ee5: 44 89 44 08 10              	mov	dword ptr [rax + rcx + 0x10], r8d
141471eea: 48 83 c3 18                 	add	rbx, 0x18
141471eee: 4c 39 fb                    	cmp	rbx, r15
141471ef1: 0f 84 3a 04 00 00           	je	0x141472331 <.text+0x1471331>
141471ef7: 48 8b 03                    	mov	rax, qword ptr [rbx]
141471efa: 48 83 f8 07                 	cmp	rax, 0x7
141471efe: 0f 85 43 02 00 00           	jne	0x141472147 <.text+0x1471147>
141471f04: 48 8b 73 10                 	mov	rsi, qword ptr [rbx + 0x10]
141471f08: 48 85 f6                    	test	rsi, rsi
141471f0b: 0f 84 df 02 00 00           	je	0x1414721f0 <.text+0x14711f0>
141471f11: 48 89 5d 00                 	mov	qword ptr [rbp], rbx
141471f15: 4c 8b 7b 08                 	mov	r15, qword ptr [rbx + 0x8]
141471f19: 48 c1 e6 04                 	shl	rsi, 0x4
141471f1d: 4c 01 fe                    	add	rsi, r15
141471f20: eb 1b                       	jmp	0x141471f3d <.text+0x1470f3d>
141471f22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
141471f30: 49 83 c7 10                 	add	r15, 0x10
141471f34: 49 39 f7                    	cmp	r15, rsi
141471f37: 0f 84 f1 01 00 00           	je	0x14147212e <.text+0x147112e>
141471f3d: 4d 8b 67 08                 	mov	r12, qword ptr [r15 + 0x8]
141471f41: 4d 85 e4                    	test	r12, r12
141471f44: 74 ea                       	je	0x141471f30 <.text+0x1470f30>
141471f46: 49 8b 1f                    	mov	rbx, qword ptr [r15]
141471f49: 49 c1 e4 03                 	shl	r12, 0x3
141471f4d: 45 31 ed                    	xor	r13d, r13d
141471f50: eb 1d                       	jmp	0x141471f6f <.text+0x1470f6f>
141471f52: 31 c0                       	xor	eax, eax
141471f54: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141471f60: 08 85 36 03 00 00           	or	byte ptr [rbp + 0x336], al
141471f66: 49 83 c5 08                 	add	r13, 0x8
141471f6a: 4d 39 ec                    	cmp	r12, r13
141471f6d: 74 c1                       	je	0x141471f30 <.text+0x1470f30>
141471f6f: 4a 8b 0c 2b                 	mov	rcx, qword ptr [rbx + r13]
141471f73: b8 01 00 00 00              	mov	eax, 0x1
141471f78: f0                          	lock
141471f79: 4c 0f b1 51 08              	cmpxchg	qword ptr [rcx + 0x8], r10
141471f7e: 0f 85 34 13 00 00           	jne	0x1414732b8 <.text+0x14722b8>
141471f84: 48 8b 01                    	mov	rax, qword ptr [rcx]
141471f87: 48 c7 41 08 01 00 00 00     	mov	qword ptr [rcx + 0x8], 0x1
141471f8f: 48 83 f8 01                 	cmp	rax, 0x1
141471f93: 0f 85 1f 13 00 00           	jne	0x1414732b8 <.text+0x14722b8>
141471f99: 4e 8b 04 2b                 	mov	r8, qword ptr [rbx + r13]
141471f9d: 49 8b 48 10                 	mov	rcx, qword ptr [r8 + 0x10]
141471fa1: 4c 39 f1                    	cmp	rcx, r14
141471fa4: 0f 85 e6 00 00 00           	jne	0x141472090 <.text+0x1471090>
141471faa: 49 8d 48 10                 	lea	rcx, [r8 + 0x10]
141471fae: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
141471fb2: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
141471fb9: 45 8b 48 20                 	mov	r9d, dword ptr [r8 + 0x20]
141471fbd: 49 8d 40 24                 	lea	rax, [r8 + 0x24]
141471fc1: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
141471fc8: 41 80 78 3c 00              	cmp	byte ptr [r8 + 0x3c], 0x0
141471fcd: 48 8d 85 60 01 00 00        	lea	rax, [rbp + 0x160]
141471fd4: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
141471fdb: 48 0f 45 c2                 	cmovne	rax, rdx
141471fdf: 44 89 ca                    	mov	edx, r9d
141471fe2: 0f 45 d7                    	cmovne	edx, edi
141471fe5: 89 95 58 01 00 00           	mov	dword ptr [rbp + 0x158], edx
141471feb: 44 0f 44 cf                 	cmove	r9d, edi
141471fef: 44 89 8d 20 03 00 00        	mov	dword ptr [rbp + 0x320], r9d
141471ff6: 49 8b 50 34                 	mov	rdx, qword ptr [r8 + 0x34]
141471ffa: 48 89 50 10                 	mov	qword ptr [rax + 0x10], rdx
141471ffe: 41 0f 10 40 24              	movups	xmm0, xmmword ptr [r8 + 0x24]
141472003: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141472006: 4c 89 85 08 03 00 00        	mov	qword ptr [rbp + 0x308], r8
14147200d: e8 2e 09 61 ff              	call	0x140a82940 <.text+0xa81940>
141472012: 48 b8 03 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffd
14147201c: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
141472023: 48 89 42 10                 	mov	qword ptr [rdx + 0x10], rax
141472027: 48 8b 85 00 01 00 00        	mov	rax, qword ptr [rbp + 0x100]
14147202e: 48 89 42 18                 	mov	qword ptr [rdx + 0x18], rax
141472032: 8b 85 58 01 00 00           	mov	eax, dword ptr [rbp + 0x158]
141472038: 89 42 20                    	mov	dword ptr [rdx + 0x20], eax
14147203b: 48 8b 85 70 01 00 00        	mov	rax, qword ptr [rbp + 0x170]
141472042: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
141472049: 48 89 41 10                 	mov	qword ptr [rcx + 0x10], rax
14147204d: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
141472054: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
141472057: 8b 85 20 03 00 00           	mov	eax, dword ptr [rbp + 0x320]
14147205d: 89 42 3c                    	mov	dword ptr [rdx + 0x3c], eax
141472060: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
141472067: 0f 11 42 40                 	movups	xmmword ptr [rdx + 0x40], xmm0
14147206b: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
141472072: 48 89 42 50                 	mov	qword ptr [rdx + 0x50], rax
141472076: b0 01                       	mov	al, 0x1
141472078: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
14147207f: e9 dc fe ff ff              	jmp	0x141471f60 <.text+0x1470f60>
141472084: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141472090: 31 c0                       	xor	eax, eax
141472092: 48 ba 10 00 00 00 00 00 00 80       	movabs	rdx, -0x7ffffffffffffff0
14147209c: 48 39 d1                    	cmp	rcx, rdx
14147209f: 0f 8c bb fe ff ff           	jl	0x141471f60 <.text+0x1470f60>
1414720a5: 41 83 b8 f8 00 00 00 02     	cmp	dword ptr [r8 + 0xf8], 0x2
1414720ad: 0f 83 ad fe ff ff           	jae	0x141471f60 <.text+0x1470f60>
1414720b3: f3 41 0f 10 80 fc 00 00 00  	movss	xmm0, dword ptr [r8 + 0xfc]
1414720bc: 41 0f 2e 80 00 01 00 00     	ucomiss	xmm0, dword ptr [r8 + 0x100]
1414720c4: 0f 85 88 fe ff ff           	jne	0x141471f52 <.text+0x1470f52>
1414720ca: 0f 8a 82 fe ff ff           	jp	0x141471f52 <.text+0x1470f52>
1414720d0: f3 41 0f 58 c7              	addss	xmm0, xmm15
1414720d5: 0f 2e c0                    	ucomiss	xmm0, xmm0
1414720d8: 0f 8a 14 0e 00 00           	jp	0x141472ef2 <.text+0x1471ef2>
1414720de: 66 0f 7e c0                 	movd	eax, xmm0
1414720e2: 25 ff ff ff 7f              	and	eax, 0x7fffffff
1414720e7: 3d 00 00 80 7f              	cmp	eax, 0x7f800000
1414720ec: 0f 84 24 0e 00 00           	je	0x141472f16 <.text+0x1471f16>
1414720f2: 0f 57 c9                    	xorps	xmm1, xmm1
1414720f5: f3 0f 5f c8                 	maxss	xmm1, xmm0
1414720f9: 0f 28 c6                    	movaps	xmm0, xmm6
1414720fc: f3 0f 5d c1                 	minss	xmm0, xmm1
141472100: f3 41 0f 11 80 fc 00 00 00  	movss	dword ptr [r8 + 0xfc], xmm0
141472109: f3 0f 58 c7                 	addss	xmm0, xmm7
14147210d: 41 0f 2e c1                 	ucomiss	xmm0, xmm9
141472111: 0f 83 23 0e 00 00           	jae	0x141472f3a <.text+0x1471f3a>
141472117: 0f 28 ce                    	movaps	xmm1, xmm6
14147211a: f3 0f 5d c8                 	minss	xmm1, xmm0
14147211e: f3 41 0f 11 88 00 01 00 00  	movss	dword ptr [r8 + 0x100], xmm1
141472127: b0 01                       	mov	al, 0x1
141472129: e9 32 fe ff ff              	jmp	0x141471f60 <.text+0x1470f60>
14147212e: 48 8b 5d 00                 	mov	rbx, qword ptr [rbp]
141472132: 48 8b 03                    	mov	rax, qword ptr [rbx]
141472135: 44 8b ad 30 03 00 00        	mov	r13d, dword ptr [rbp + 0x330]
14147213c: 4c 8b a5 10 01 00 00        	mov	r12, qword ptr [rbp + 0x110]
141472143: 4c 8b 7d 08                 	mov	r15, qword ptr [rbp + 0x8]
141472147: 48 85 c0                    	test	rax, rax
14147214a: 0f 85 a0 00 00 00           	jne	0x1414721f0 <.text+0x14711f0>
141472150: 48 8b 4b 08                 	mov	rcx, qword ptr [rbx + 0x8]
141472154: b8 01 00 00 00              	mov	eax, 0x1
141472159: f0                          	lock
14147215a: 4c 0f b1 51 08              	cmpxchg	qword ptr [rcx + 0x8], r10
14147215f: 0f 85 6b 11 00 00           	jne	0x1414732d0 <.text+0x14722d0>
141472165: 48 8b 01                    	mov	rax, qword ptr [rcx]
141472168: 48 c7 41 08 01 00 00 00     	mov	qword ptr [rcx + 0x8], 0x1
141472170: 48 83 f8 01                 	cmp	rax, 0x1
141472174: 0f 85 56 11 00 00           	jne	0x1414732d0 <.text+0x14722d0>
14147217a: 48 8b 43 08                 	mov	rax, qword ptr [rbx + 0x8]
14147217e: 31 c9                       	xor	ecx, ecx
141472180: 48 3b 48 10                 	cmp	rcx, qword ptr [rax + 0x10]
141472184: 71 6a                       	jno	0x1414721f0 <.text+0x14711f0>
141472186: 48 83 78 28 02              	cmp	qword ptr [rax + 0x28], 0x2
14147218b: 75 63                       	jne	0x1414721f0 <.text+0x14711f0>
14147218d: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
141472191: f3 0f 10 50 08              	movss	xmm2, dword ptr [rax + 0x8]
141472196: f2 0f 10 48 0c              	movsd	xmm1, qword ptr [rax + 0xc]
14147219b: f3 0f 10 40 14              	movss	xmm0, dword ptr [rax + 0x14]
1414721a0: f3 0f 5c d0                 	subss	xmm2, xmm0
1414721a4: f2 0f 10 18                 	movsd	xmm3, qword ptr [rax]
1414721a8: 0f 5c d9                    	subps	xmm3, xmm1
1414721ab: 0f 59 db                    	mulps	xmm3, xmm3
1414721ae: f3 0f 16 e3                 	movshdup	xmm4, xmm3              # xmm4 = xmm3[1,1,3,3]
1414721b2: f3 0f 58 e3                 	addss	xmm4, xmm3
1414721b6: f3 0f 59 d2                 	mulss	xmm2, xmm2
1414721ba: f3 0f 58 d4                 	addss	xmm2, xmm4
1414721be: f3 0f 51 d2                 	sqrtss	xmm2, xmm2
1414721c2: 0f 2e 15 c7 e4 4e 01        	ucomiss	xmm2, dword ptr [rip + 0x14ee4c7] # 0x142960690
1414721c9: 75 25                       	jne	0x1414721f0 <.text+0x14711f0>
1414721cb: 7a 23                       	jp	0x1414721f0 <.text+0x14711f0>
1414721cd: f3 0f 10 15 8b ae 55 01     	movss	xmm2, dword ptr [rip + 0x155ae8b] # 0x1429cd060
1414721d5: 0f 58 ca                    	addps	xmm1, xmm2
1414721d8: 0f 13 48 0c                 	movlps	qword ptr [rax + 0xc], xmm1
1414721dc: f3 0f 58 05 ac e4 4e 01     	addss	xmm0, dword ptr [rip + 0x14ee4ac] # 0x142960690
1414721e4: f3 0f 11 40 14              	movss	dword ptr [rax + 0x14], xmm0
1414721e9: c6 85 36 03 00 00 01        	mov	byte ptr [rbp + 0x336], 0x1
1414721f0: 41 83 fd 14                 	cmp	r13d, 0x14
1414721f4: 0f 83 f0 fc ff ff           	jae	0x141471eea <.text+0x1470eea>
1414721fa: 83 3b 09                    	cmp	dword ptr [rbx], 0x9
1414721fd: 0f 85 e7 fc ff ff           	jne	0x141471eea <.text+0x1470eea>
141472203: 48 8b 4b 08                 	mov	rcx, qword ptr [rbx + 0x8]
141472207: b8 01 00 00 00              	mov	eax, 0x1
14147220c: f0                          	lock
14147220d: 4c 0f b1 51 08              	cmpxchg	qword ptr [rcx + 0x8], r10
141472212: 0f 85 d0 10 00 00           	jne	0x1414732e8 <.text+0x14722e8>
141472218: 48 8b 01                    	mov	rax, qword ptr [rcx]
14147221b: 48 c7 41 08 01 00 00 00     	mov	qword ptr [rcx + 0x8], 0x1
141472223: 48 83 f8 01                 	cmp	rax, 0x1
141472227: 0f 85 bb 10 00 00           	jne	0x1414732e8 <.text+0x14722e8>
14147222d: 48 8b 43 08                 	mov	rax, qword ptr [rbx + 0x8]
141472231: 0f b6 48 18                 	movzx	ecx, byte ptr [rax + 0x18]
141472235: 83 f9 04                    	cmp	ecx, 0x4
141472238: 74 15                       	je	0x14147224f <.text+0x147124f>
14147223a: 83 f9 07                    	cmp	ecx, 0x7
14147223d: 0f 85 a7 fc ff ff           	jne	0x141471eea <.text+0x1470eea>
141472243: b9 14 00 00 00              	mov	ecx, 0x14
141472248: ba 10 00 00 00              	mov	edx, 0x10
14147224d: eb 0a                       	jmp	0x141472259 <.text+0x1471259>
14147224f: b9 1c 00 00 00              	mov	ecx, 0x1c
141472254: ba 18 00 00 00              	mov	edx, 0x18
141472259: 44 8b 4c 10 10              	mov	r9d, dword ptr [rax + rdx + 0x10]
14147225e: 49 83 f9 08                 	cmp	r9, 0x8
141472262: 77 42                       	ja	0x1414722a6 <.text+0x14712a6>
141472264: 4c 8d 1d 05 2b 7f 01        	lea	r11, [rip + 0x17f2b05]  # 0x142c64d70
14147226b: 4f 63 04 8b                 	movsxd	r8, dword ptr [r11 + 4*r9]
14147226f: 4d 01 d8                    	add	r8, r11
141472272: 41 ff e0                    	jmp	r8
141472275: 45 89 c8                    	mov	r8d, r9d
141472278: 44 89 44 10 10              	mov	dword ptr [rax + rdx + 0x10], r8d
14147227d: 44 8b 44 08 10              	mov	r8d, dword ptr [rax + rcx + 0x10]
141472282: 49 83 f8 08                 	cmp	r8, 0x8
141472286: 77 2f                       	ja	0x1414722b7 <.text+0x14712b7>
141472288: 4c 8d 0d 05 2b 7f 01        	lea	r9, [rip + 0x17f2b05]   # 0x142c64d94
14147228f: 4b 63 14 81                 	movsxd	rdx, dword ptr [r9 + 4*r8]
141472293: 4c 01 ca                    	add	rdx, r9
141472296: ff e2                       	jmp	rdx
141472298: ba 02 00 00 00              	mov	edx, 0x2
14147229d: 89 54 08 10                 	mov	dword ptr [rax + rcx + 0x10], edx
1414722a1: e9 44 fc ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722a6: 41 b8 01 00 00 00           	mov	r8d, 0x1
1414722ac: 41 81 f9 e7 03 00 00        	cmp	r9d, 0x3e7
1414722b3: 74 c0                       	je	0x141472275 <.text+0x1471275>
1414722b5: eb c1                       	jmp	0x141472278 <.text+0x1471278>
1414722b7: ba 01 00 00 00              	mov	edx, 0x1
1414722bc: 41 81 f8 e7 03 00 00        	cmp	r8d, 0x3e7
1414722c3: 0f 84 1c fc ff ff           	je	0x141471ee5 <.text+0x1470ee5>
1414722c9: eb 5d                       	jmp	0x141472328 <.text+0x1471328>
1414722cb: 41 b8 02 00 00 00           	mov	r8d, 0x2
1414722d1: eb a5                       	jmp	0x141472278 <.text+0x1471278>
1414722d3: 41 b8 03 00 00 00           	mov	r8d, 0x3
1414722d9: eb 9d                       	jmp	0x141472278 <.text+0x1471278>
1414722db: ba 03 00 00 00              	mov	edx, 0x3
1414722e0: 89 54 08 10                 	mov	dword ptr [rax + rcx + 0x10], edx
1414722e4: e9 01 fc ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722e9: 41 b8 05 00 00 00           	mov	r8d, 0x5
1414722ef: eb 87                       	jmp	0x141472278 <.text+0x1471278>
1414722f1: ba 05 00 00 00              	mov	edx, 0x5
1414722f6: 89 54 08 10                 	mov	dword ptr [rax + rcx + 0x10], edx
1414722fa: e9 eb fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722ff: 41 b8 08 00 00 00           	mov	r8d, 0x8
141472305: e9 6e ff ff ff              	jmp	0x141472278 <.text+0x1471278>
14147230a: ba 08 00 00 00              	mov	edx, 0x8
14147230f: 89 54 08 10                 	mov	dword ptr [rax + rcx + 0x10], edx
141472313: e9 d2 fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
141472318: 41 b8 04 00 00 00           	mov	r8d, 0x4
14147231e: e9 55 ff ff ff              	jmp	0x141472278 <.text+0x1471278>
141472323: ba 04 00 00 00              	mov	edx, 0x4
141472328: 89 54 08 10                 	mov	dword ptr [rax + rcx + 0x10], edx
14147232c: e9 b9 fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
141472331: 41 83 fd 10                 	cmp	r13d, 0x10
141472335: 0f 83 71 01 00 00           	jae	0x1414724ac <.text+0x14714ac>
14147233b: 4c 8b 85 f8 02 00 00        	mov	r8, qword ptr [rbp + 0x2f8]
141472342: 49 8b 48 18                 	mov	rcx, qword ptr [r8 + 0x18]
141472346: b8 01 00 00 00              	mov	eax, 0x1
14147234b: 31 d2                       	xor	edx, edx
14147234d: f0                          	lock
14147234e: 48 0f b1 11                 	cmpxchg	qword ptr [rcx], rdx
141472352: 49 8d 70 18                 	lea	rsi, [r8 + 0x18]
141472356: 4d 8b 78 18                 	mov	r15, qword ptr [r8 + 0x18]
14147235a: 75 1a                       	jne	0x141472376 <.text+0x1471376>
14147235c: 49 8b 47 08                 	mov	rax, qword ptr [r15 + 0x8]
141472360: 48 83 f8 01                 	cmp	rax, 0x1
141472364: 0f 85 50 04 00 00           	jne	0x1414727ba <.text+0x14717ba>
14147236a: 49 c7 07 01 00 00 00        	mov	qword ptr [r15], 0x1
141472371: e9 23 01 00 00              	jmp	0x141472499 <.text+0x1471499>
141472376: 48 89 b5 18 03 00 00        	mov	qword ptr [rbp + 0x318], rsi
14147237d: 49 8d 57 10                 	lea	rdx, [r15 + 0x10]
141472381: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141472388: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
14147238f: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
141472396: e8 d5 ad 6e ff              	call	0x140b5d170 <.text+0xb5c170>
14147239b: 44 0f 29 85 00 01 00 00     	movaps	xmmword ptr [rbp + 0x100], xmm8
1414723a3: 66 45 0f 6f c6              	movdqa	xmm8, xmm14
1414723a8: 45 0f 28 f5                 	movaps	xmm14, xmm13
1414723ac: 45 0f 28 ec                 	movaps	xmm13, xmm12
1414723b0: 45 0f 28 e3                 	movaps	xmm12, xmm11
1414723b4: 45 0f 28 da                 	movaps	xmm11, xmm10
1414723b8: 48 8b b5 70 02 00 00        	mov	rsi, qword ptr [rbp + 0x270]
1414723bf: 48 8b 9d 80 02 00 00        	mov	rbx, qword ptr [rbp + 0x280]
1414723c6: 4c 89 bd 20 03 00 00        	mov	qword ptr [rbp + 0x320], r15
1414723cd: 45 0f 10 57 30              	movups	xmm10, xmmword ptr [r15 + 0x30]
1414723d2: c6 85 35 03 00 00 01        	mov	byte ptr [rbp + 0x335], 0x1
1414723d9: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414723e0: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
1414723e7: e8 54 07 70 ff              	call	0x140b72b40 <.text+0xb71b40>
1414723ec: 48 8d 46 ff                 	lea	rax, [rsi - 0x1]
1414723f0: 48 89 f1                    	mov	rcx, rsi
1414723f3: 48 31 c1                    	xor	rcx, rax
1414723f6: 48 8d 56 0f                 	lea	rdx, [rsi + 0xf]
1414723fa: 48 f7 de                    	neg	rsi
1414723fd: 48 21 d6                    	and	rsi, rdx
141472400: 48 83 c6 f0                 	add	rsi, -0x10
141472404: 48 39 c1                    	cmp	rcx, rax
141472407: 48 c7 c0 ff ff ff ff        	mov	rax, -0x1
14147240e: 48 0f 46 f0                 	cmovbe	rsi, rax
141472412: 44 0f 11 95 80 01 00 00     	movups	xmmword ptr [rbp + 0x180], xmm10
14147241a: 0f 10 85 80 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x180]
141472421: 0f 11 44 33 30              	movups	xmmword ptr [rbx + rsi + 0x30], xmm0
141472426: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
14147242d: 0f 10 8d 70 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x170]
141472434: 0f 11 4c 33 20              	movups	xmmword ptr [rbx + rsi + 0x20], xmm1
141472439: 48 89 9d 08 03 00 00        	mov	qword ptr [rbp + 0x308], rbx
141472440: 0f 11 44 33 10              	movups	xmmword ptr [rbx + rsi + 0x10], xmm0
141472445: f6 85 88 02 00 00 01        	test	byte ptr [rbp + 0x288], 0x1
14147244c: 0f 84 e6 0e 00 00           	je	0x141473338 <.text+0x1472338>
141472452: 48 8b 85 20 03 00 00        	mov	rax, qword ptr [rbp + 0x320]
141472459: f0                          	lock
14147245a: 48 ff 08                    	dec	qword ptr [rax]
14147245d: 45 0f 28 d3                 	movaps	xmm10, xmm11
141472461: 75 0c                       	jne	0x14147246f <.text+0x147146f>
141472463: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
14147246a: e8 f1 a6 6e ff              	call	0x140b5cb60 <.text+0xb5bb60>
14147246f: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
141472476: 4c 8b bd 08 03 00 00        	mov	r15, qword ptr [rbp + 0x308]
14147247d: 4c 89 38                    	mov	qword ptr [rax], r15
141472480: 45 0f 28 dc                 	movaps	xmm11, xmm12
141472484: 45 0f 28 e5                 	movaps	xmm12, xmm13
141472488: 45 0f 28 ee                 	movaps	xmm13, xmm14
14147248c: 66 45 0f 6f f0              	movdqa	xmm14, xmm8
141472491: 44 0f 28 85 00 01 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x100]
141472499: 49 83 c7 10                 	add	r15, 0x10
14147249d: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
1414724a4: 4c 89 f9                    	mov	rcx, r15
1414724a7: e8 24 77 70 ff              	call	0x140b79bd0 <.text+0xb78bd0>
1414724ac: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
1414724b3: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
1414724ba: 48 8d 95 36 03 00 00        	lea	rdx, [rbp + 0x336]
1414724c1: 4c 8d 85 00 03 00 00        	lea	r8, [rbp + 0x300]
1414724c8: e8 23 2c e8 ff              	call	0x1412f50f0 <.text+0x12f40f0>
1414724cd: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
1414724d4: 48 8d 50 28                 	lea	rdx, [rax + 0x28]
1414724d8: 41 83 fd 14                 	cmp	r13d, 0x14
1414724dc: 0f 83 95 03 00 00           	jae	0x141472877 <.text+0x1471877>
1414724e2: 48 8b 48 18                 	mov	rcx, qword ptr [rax + 0x18]
1414724e6: 48 83 79 28 00              	cmp	qword ptr [rcx + 0x28], 0x0
1414724eb: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
1414724f2: 0f 84 b6 f9 ff ff           	je	0x141471eae <.text+0x1470eae>
1414724f8: b8 01 00 00 00              	mov	eax, 0x1
1414724fd: f0                          	lock
1414724fe: 4c 0f b1 51 08              	cmpxchg	qword ptr [rcx + 0x8], r10
141472503: 0f 85 52 0e 00 00           	jne	0x14147335b <.text+0x147235b>
141472509: 48 8b 01                    	mov	rax, qword ptr [rcx]
14147250c: 48 c7 41 08 01 00 00 00     	mov	qword ptr [rcx + 0x8], 0x1
141472514: 48 83 f8 01                 	cmp	rax, 0x1
141472518: 0f 85 3d 0e 00 00           	jne	0x14147335b <.text+0x147235b>
14147251e: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
141472525: 48 8b 40 18                 	mov	rax, qword ptr [rax + 0x18]
141472529: 48 8b 48 28                 	mov	rcx, qword ptr [rax + 0x28]
14147252d: 48 85 c9                    	test	rcx, rcx
141472530: 0f 84 54 03 00 00           	je	0x14147288a <.text+0x147188a>
141472536: 48 8b 58 10                 	mov	rbx, qword ptr [rax + 0x10]
14147253a: 48 8d 43 10                 	lea	rax, [rbx + 0x10]
14147253e: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
141472545: 66 0f 6f 03                 	movdqa	xmm0, xmmword ptr [rbx]
141472549: 66 0f d7 c0                 	pmovmskb	eax, xmm0
14147254d: f7 d0                       	not	eax
14147254f: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
141472556: 48 89 95 00 01 00 00        	mov	qword ptr [rbp + 0x100], rdx
14147255d: eb 2c                       	jmp	0x14147258b <.text+0x147158b>
14147255f: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
141472566: 48 ff c9                    	dec	rcx
141472569: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
141472570: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
141472577: 48 8b 95 00 01 00 00        	mov	rdx, qword ptr [rbp + 0x100]
14147257e: 48 8b 9d 08 03 00 00        	mov	rbx, qword ptr [rbp + 0x308]
141472585: 0f 84 f3 02 00 00           	je	0x14147287e <.text+0x147187e>
14147258b: 4c 8b 85 20 03 00 00        	mov	r8, qword ptr [rbp + 0x320]
141472592: 66 45 85 c0                 	test	r8w, r8w
141472596: 48 89 8d 18 03 00 00        	mov	qword ptr [rbp + 0x318], rcx
14147259d: 74 09                       	je	0x1414725a8 <.text+0x14715a8>
14147259f: 41 8d 40 ff                 	lea	eax, [r8 - 0x1]
1414725a3: 44 21 c0                    	and	eax, r8d
1414725a6: eb 3b                       	jmp	0x1414725e3 <.text+0x14715e3>
1414725a8: 48 8b 85 58 01 00 00        	mov	rax, qword ptr [rbp + 0x158]
1414725af: 66 0f 6f 00                 	movdqa	xmm0, xmmword ptr [rax]
1414725b3: 66 44 0f d7 c0              	pmovmskb	r8d, xmm0
1414725b8: 48 81 c3 00 f9 ff ff        	add	rbx, -0x700
1414725bf: 48 83 c0 10                 	add	rax, 0x10
1414725c3: 41 81 f8 ff ff 00 00        	cmp	r8d, 0xffff
1414725ca: 74 e3                       	je	0x1414725af <.text+0x14715af>
1414725cc: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
1414725d3: b9 fe ff ff ff              	mov	ecx, 0xfffffffe
1414725d8: 44 29 c1                    	sub	ecx, r8d
1414725db: 41 f7 d0                    	not	r8d
1414725de: 44 89 c0                    	mov	eax, r8d
1414725e1: 21 c8                       	and	eax, ecx
1414725e3: 44 89 c1                    	mov	ecx, r8d
1414725e6: f3 0f bc c9                 	tzcnt	ecx, ecx
1414725ea: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
1414725f1: 48 f7 d9                    	neg	rcx
1414725f4: 48 6b c9 70                 	imul	rcx, rcx, 0x70
1414725f8: 4c 8b 44 0b a0              	mov	r8, qword ptr [rbx + rcx - 0x60]
1414725fd: 4c 8b 54 0b b8              	mov	r10, qword ptr [rbx + rcx - 0x48]
141472602: 66 41 0f 6f 00              	movdqa	xmm0, xmmword ptr [r8]
141472607: 66 0f d7 d0                 	pmovmskb	edx, xmm0
14147260b: f7 d2                       	not	edx
14147260d: 4d 8d 58 10                 	lea	r11, [r8 + 0x10]
141472611: 48 8b 74 0b d0              	mov	rsi, qword ptr [rbx + rcx - 0x30]
141472616: 66 0f 6f 06                 	movdqa	xmm0, xmmword ptr [rsi]
14147261a: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
14147261f: 41 f7 d1                    	not	r9d
141472622: 48 8d 46 10                 	lea	rax, [rsi + 0x10]
141472626: 48 89 9d 08 03 00 00        	mov	qword ptr [rbp + 0x308], rbx
14147262d: 48 8b 4c 0b e8              	mov	rcx, qword ptr [rbx + rcx - 0x18]
141472632: 4d 85 c0                    	test	r8, r8
141472635: 75 47                       	jne	0x14147267e <.text+0x147167e>
141472637: e9 a0 00 00 00              	jmp	0x1414726dc <.text+0x14716dc>
14147263c: 45 8d 61 ff                 	lea	r12d, [r9 - 0x1]
141472640: 45 21 cc                    	and	r12d, r9d
141472643: 48 ff c9                    	dec	rcx
141472646: 45 89 cf                    	mov	r15d, r9d
141472649: 48 89 f3                    	mov	rbx, rsi
14147264c: 45 31 c0                    	xor	r8d, r8d
14147264f: 45 89 e1                    	mov	r9d, r12d
141472652: f3 45 0f bc ff              	tzcnt	r15d, r15d
141472657: 49 f7 df                    	neg	r15
14147265a: 4d 6b ff 58                 	imul	r15, r15, 0x58
14147265e: 46 0f b6 64 3b fa           	movzx	r12d, byte ptr [rbx + r15 - 0x6]
141472664: 41 80 c4 c8                 	add	r12b, -0x38
141472668: 41 80 fc 02                 	cmp	r12b, 0x2
14147266c: 4c 8b a5 10 01 00 00        	mov	r12, qword ptr [rbp + 0x110]
141472673: 0f 86 bb 00 00 00           	jbe	0x141472734 <.text+0x1471734>
141472679: 4d 85 c0                    	test	r8, r8
14147267c: 74 5e                       	je	0x1414726dc <.text+0x14716dc>
14147267e: 4d 85 d2                    	test	r10, r10
141472681: 74 56                       	je	0x1414726d9 <.text+0x14716d9>
141472683: 66 85 d2                    	test	dx, dx
141472686: 74 18                       	je	0x1414726a0 <.text+0x14716a0>
141472688: 44 8d 62 ff                 	lea	r12d, [rdx - 0x1]
14147268c: 41 21 d4                    	and	r12d, edx
14147268f: 49 ff ca                    	dec	r10
141472692: 41 89 d7                    	mov	r15d, edx
141472695: 4c 89 c3                    	mov	rbx, r8
141472698: 44 89 e2                    	mov	edx, r12d
14147269b: eb b5                       	jmp	0x141472652 <.text+0x1471652>
14147269d: 0f 1f 00                    	nop	dword ptr [rax]
1414726a0: 66 41 0f 6f 03              	movdqa	xmm0, xmmword ptr [r11]
1414726a5: 66 44 0f d7 f8              	pmovmskb	r15d, xmm0
1414726aa: 49 81 c0 80 fa ff ff        	add	r8, -0x580
1414726b1: 49 83 c3 10                 	add	r11, 0x10
1414726b5: 41 81 ff ff ff 00 00        	cmp	r15d, 0xffff
1414726bc: 74 e2                       	je	0x1414726a0 <.text+0x14716a0>
1414726be: bb fe ff ff ff              	mov	ebx, 0xfffffffe
1414726c3: 44 29 fb                    	sub	ebx, r15d
1414726c6: 41 f7 d7                    	not	r15d
1414726c9: 44 89 fa                    	mov	edx, r15d
1414726cc: 21 da                       	and	edx, ebx
1414726ce: 49 ff ca                    	dec	r10
1414726d1: 4c 89 c3                    	mov	rbx, r8
1414726d4: e9 79 ff ff ff              	jmp	0x141472652 <.text+0x1471652>
1414726d9: 45 31 d2                    	xor	r10d, r10d
1414726dc: 48 85 f6                    	test	rsi, rsi
1414726df: 0f 84 7a fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726e5: 48 85 c9                    	test	rcx, rcx
1414726e8: 0f 84 71 fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726ee: 66 45 85 c9                 	test	r9w, r9w
1414726f2: 0f 85 44 ff ff ff           	jne	0x14147263c <.text+0x147163c>
1414726f8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141472700: 66 0f 6f 00                 	movdqa	xmm0, xmmword ptr [rax]
141472704: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
141472709: 48 81 c6 80 fa ff ff        	add	rsi, -0x580
141472710: 48 83 c0 10                 	add	rax, 0x10
141472714: 41 81 f9 ff ff 00 00        	cmp	r9d, 0xffff
14147271b: 74 e3                       	je	0x141472700 <.text+0x1471700>
14147271d: 41 b8 fe ff ff ff           	mov	r8d, 0xfffffffe
141472723: 45 29 c8                    	sub	r8d, r9d
141472726: 41 f7 d1                    	not	r9d
141472729: 45 89 cc                    	mov	r12d, r9d
14147272c: 45 21 c4                    	and	r12d, r8d
14147272f: e9 0f ff ff ff              	jmp	0x141472643 <.text+0x1471643>
141472734: 4c 01 fb                    	add	rbx, r15
141472737: 44 8b 63 d8                 	mov	r12d, dword ptr [rbx - 0x28]
14147273b: 49 83 fc 08                 	cmp	r12, 0x8
14147273f: 77 17                       	ja	0x141472758 <.text+0x1471758>
141472741: 4c 8d 2d 70 26 7f 01        	lea	r13, [rip + 0x17f2670]  # 0x142c64db8
141472748: 4f 63 7c a5 00              	movsxd	r15, dword ptr [r13 + 4*r12]
14147274d: 4d 01 ef                    	add	r15, r13
141472750: 41 ff e7                    	jmp	r15
141472753: 45 89 e7                    	mov	r15d, r12d
141472756: eb 3a                       	jmp	0x141472792 <.text+0x1471792>
141472758: 41 bf 01 00 00 00           	mov	r15d, 0x1
14147275e: 41 81 fc e7 03 00 00        	cmp	r12d, 0x3e7
141472765: 75 32                       	jne	0x141472799 <.text+0x1471799>
141472767: 45 89 e7                    	mov	r15d, r12d
14147276a: eb 2d                       	jmp	0x141472799 <.text+0x1471799>
14147276c: 41 bf 02 00 00 00           	mov	r15d, 0x2
141472772: eb 1e                       	jmp	0x141472792 <.text+0x1471792>
141472774: 41 bf 03 00 00 00           	mov	r15d, 0x3
14147277a: eb 16                       	jmp	0x141472792 <.text+0x1471792>
14147277c: 41 bf 05 00 00 00           	mov	r15d, 0x5
141472782: eb 0e                       	jmp	0x141472792 <.text+0x1471792>
141472784: 41 bf 08 00 00 00           	mov	r15d, 0x8
14147278a: eb 06                       	jmp	0x141472792 <.text+0x1471792>
14147278c: 41 bf 04 00 00 00           	mov	r15d, 0x4
141472792: 44 8b ad 30 03 00 00        	mov	r13d, dword ptr [rbp + 0x330]
141472799: 44 89 7b d8                 	mov	dword ptr [rbx - 0x28], r15d
14147279d: b3 01                       	mov	bl, 0x1
14147279f: 89 9d f0 02 00 00           	mov	dword ptr [rbp + 0x2f0], ebx
1414727a5: 4c 8b a5 10 01 00 00        	mov	r12, qword ptr [rbp + 0x110]
1414727ac: 4d 85 c0                    	test	r8, r8
1414727af: 0f 85 c9 fe ff ff           	jne	0x14147267e <.text+0x147167e>
1414727b5: e9 22 ff ff ff              	jmp	0x1414726dc <.text+0x14716dc>
1414727ba: 49 8d 47 08                 	lea	rax, [r15 + 0x8]
1414727be: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
1414727c5: 4c 89 bd 20 03 00 00        	mov	qword ptr [rbp + 0x320], r15
1414727cc: 49 8d 5f 10                 	lea	rbx, [r15 + 0x10]
1414727d0: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414727d7: 48 89 da                    	mov	rdx, rbx
1414727da: e8 91 a9 6e ff              	call	0x140b5d170 <.text+0xb5c170>
1414727df: 48 8b 85 60 01 00 00        	mov	rax, qword ptr [rbp + 0x160]
1414727e6: 4c 8b bd 70 01 00 00        	mov	r15, qword ptr [rbp + 0x170]
1414727ed: 48 8d 48 ff                 	lea	rcx, [rax - 0x1]
1414727f1: 48 89 c2                    	mov	rdx, rax
1414727f4: 48 31 ca                    	xor	rdx, rcx
1414727f7: 4c 8d 40 0f                 	lea	r8, [rax + 0xf]
1414727fb: 48 f7 d8                    	neg	rax
1414727fe: 4c 21 c0                    	and	rax, r8
141472801: 48 83 c0 f0                 	add	rax, -0x10
141472805: 48 39 ca                    	cmp	rdx, rcx
141472808: 48 c7 c1 ff ff ff ff        	mov	rcx, -0x1
14147280f: 48 0f 46 c1                 	cmovbe	rax, rcx
141472813: 0f 10 03                    	movups	xmm0, xmmword ptr [rbx]
141472816: 0f 10 4b 10                 	movups	xmm1, xmmword ptr [rbx + 0x10]
14147281a: 0f 10 53 20                 	movups	xmm2, xmmword ptr [rbx + 0x20]
14147281e: 41 0f 11 54 07 30           	movups	xmmword ptr [r15 + rax + 0x30], xmm2
141472824: 41 0f 11 4c 07 20           	movups	xmmword ptr [r15 + rax + 0x20], xmm1
14147282a: 41 0f 11 44 07 10           	movups	xmmword ptr [r15 + rax + 0x10], xmm0
141472830: f6 85 78 01 00 00 01        	test	byte ptr [rbp + 0x178], 0x1
141472837: 0f 84 10 0b 00 00           	je	0x14147334d <.text+0x147234d>
14147283d: 4c 89 3e                    	mov	qword ptr [rsi], r15
141472840: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
141472847: 48 83 f9 ff                 	cmp	rcx, -0x1
14147284b: 0f 84 48 fc ff ff           	je	0x141472499 <.text+0x1471499>
141472851: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
141472858: f0                          	lock
141472859: 48 ff 08                    	dec	qword ptr [rax]
14147285c: 0f 85 37 fc ff ff           	jne	0x141472499 <.text+0x1471499>
141472862: ba 40 00 00 00              	mov	edx, 0x40
141472867: 41 b8 08 00 00 00           	mov	r8d, 0x8
14147286d: e8 ae 13 1a ff              	call	0x140613c20 <.text+0x612c20>
141472872: e9 22 fc ff ff              	jmp	0x141472499 <.text+0x1471499>
141472877: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
14147287e: 41 83 fd 15                 	cmp	r13d, 0x15
141472882: 0f 83 26 f6 ff ff           	jae	0x141471eae <.text+0x1470eae>
141472888: eb 07                       	jmp	0x141472891 <.text+0x1471891>
14147288a: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
141472891: 48 8b 48 18                 	mov	rcx, qword ptr [rax + 0x18]
141472895: 48 83 79 28 00              	cmp	qword ptr [rcx + 0x28], 0x0
14147289a: 0f 84 0e f6 ff ff           	je	0x141471eae <.text+0x1470eae>
1414728a0: b8 01 00 00 00              	mov	eax, 0x1
1414728a5: f0                          	lock
1414728a6: 4c 0f b1 51 08              	cmpxchg	qword ptr [rcx + 0x8], r10
1414728ab: 0f 85 bf 0a 00 00           	jne	0x141473370 <.text+0x1472370>
1414728b1: 48 8b 01                    	mov	rax, qword ptr [rcx]
1414728b4: 48 c7 41 08 01 00 00 00     	mov	qword ptr [rcx + 0x8], 0x1
1414728bc: 48 83 f8 01                 	cmp	rax, 0x1
1414728c0: 0f 85 aa 0a 00 00           	jne	0x141473370 <.text+0x1472370>
1414728c6: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
1414728cd: 48 8b 40 18                 	mov	rax, qword ptr [rax + 0x18]
1414728d1: 48 8b 48 28                 	mov	rcx, qword ptr [rax + 0x28]
1414728d5: 48 85 c9                    	test	rcx, rcx
1414728d8: 0f 84 d0 f5 ff ff           	je	0x141471eae <.text+0x1470eae>
1414728de: 48 89 95 00 01 00 00        	mov	qword ptr [rbp + 0x100], rdx
1414728e5: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1414728e9: 48 8d 50 10                 	lea	rdx, [rax + 0x10]
1414728ed: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
1414728f4: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
1414728fb: 66 0f 6f 00                 	movdqa	xmm0, xmmword ptr [rax]
1414728ff: 66 0f d7 c0                 	pmovmskb	eax, xmm0
141472903: f7 d0                       	not	eax
141472905: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14147290c: 49 89 cd                    	mov	r13, rcx
14147290f: eb 09                       	jmp	0x14147291a <.text+0x147191a>
141472911: 49 ff cd                    	dec	r13
141472914: 0f 84 a3 01 00 00           	je	0x141472abd <.text+0x1471abd>
14147291a: 48 8b 95 20 03 00 00        	mov	rdx, qword ptr [rbp + 0x320]
141472921: 66 85 d2                    	test	dx, dx
141472924: 74 07                       	je	0x14147292d <.text+0x147192d>
141472926: 8d 42 ff                    	lea	eax, [rdx - 0x1]
141472929: 21 d0                       	and	eax, edx
14147292b: eb 43                       	jmp	0x141472970 <.text+0x1471970>
14147292d: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
141472934: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
14147293b: 66 0f 6f 01                 	movdqa	xmm0, xmmword ptr [rcx]
14147293f: 66 0f d7 d0                 	pmovmskb	edx, xmm0
141472943: 48 05 00 f9 ff ff           	add	rax, -0x700
141472949: 48 83 c1 10                 	add	rcx, 0x10
14147294d: 81 fa ff ff 00 00           	cmp	edx, 0xffff
141472953: 74 e6                       	je	0x14147293b <.text+0x147193b>
141472955: 48 89 8d 08 03 00 00        	mov	qword ptr [rbp + 0x308], rcx
14147295c: 48 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], rax
141472963: b9 fe ff ff ff              	mov	ecx, 0xfffffffe
141472968: 29 d1                       	sub	ecx, edx
14147296a: f7 d2                       	not	edx
14147296c: 89 d0                       	mov	eax, edx
14147296e: 21 c8                       	and	eax, ecx
141472970: 89 d1                       	mov	ecx, edx
141472972: f3 0f bc c9                 	tzcnt	ecx, ecx
141472976: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
14147297d: 48 f7 d9                    	neg	rcx
141472980: 48 6b c1 70                 	imul	rax, rcx, 0x70
141472984: 48 8b 9d 18 03 00 00        	mov	rbx, qword ptr [rbp + 0x318]
14147298b: 4c 8b 44 03 a0              	mov	r8, qword ptr [rbx + rax - 0x60]
141472990: 48 8b 4c 03 b8              	mov	rcx, qword ptr [rbx + rax - 0x48]
141472995: 66 41 0f 6f 00              	movdqa	xmm0, xmmword ptr [r8]
14147299a: 66 0f d7 d0                 	pmovmskb	edx, xmm0
14147299e: f7 d2                       	not	edx
1414729a0: 4d 8d 50 10                 	lea	r10, [r8 + 0x10]
1414729a4: 4c 8b 5c 03 d0              	mov	r11, qword ptr [rbx + rax - 0x30]
1414729a9: 66 41 0f 6f 03              	movdqa	xmm0, xmmword ptr [r11]
1414729ae: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
1414729b3: 41 f7 d1                    	not	r9d
1414729b6: 49 8d 73 10                 	lea	rsi, [r11 + 0x10]
1414729ba: 48 8b 44 03 e8              	mov	rax, qword ptr [rbx + rax - 0x18]
1414729bf: 4d 85 c0                    	test	r8, r8
1414729c2: 75 37                       	jne	0x1414729fb <.text+0x14719fb>
1414729c4: e9 86 00 00 00              	jmp	0x141472a4f <.text+0x1471a4f>
1414729c9: 45 8d 61 ff                 	lea	r12d, [r9 - 0x1]
1414729cd: 45 21 cc                    	and	r12d, r9d
1414729d0: 48 ff c8                    	dec	rax
1414729d3: 44 89 cb                    	mov	ebx, r9d
1414729d6: 4d 89 df                    	mov	r15, r11
1414729d9: 45 31 c0                    	xor	r8d, r8d
1414729dc: 45 89 e1                    	mov	r9d, r12d
1414729df: f3 0f bc db                 	tzcnt	ebx, ebx
1414729e3: 48 f7 db                    	neg	rbx
1414729e6: 48 6b db 58                 	imul	rbx, rbx, 0x58
1414729ea: 41 80 7c 1f fa 3a           	cmp	byte ptr [r15 + rbx - 0x6], 0x3a
1414729f0: 0f 84 a9 00 00 00           	je	0x141472a9f <.text+0x1471a9f>
1414729f6: 4d 85 c0                    	test	r8, r8
1414729f9: 74 54                       	je	0x141472a4f <.text+0x1471a4f>
1414729fb: 48 85 c9                    	test	rcx, rcx
1414729fe: 74 4d                       	je	0x141472a4d <.text+0x1471a4d>
141472a00: 66 85 d2                    	test	dx, dx
141472a03: 74 14                       	je	0x141472a19 <.text+0x1471a19>
141472a05: 44 8d 62 ff                 	lea	r12d, [rdx - 0x1]
141472a09: 41 21 d4                    	and	r12d, edx
141472a0c: 48 ff c9                    	dec	rcx
141472a0f: 89 d3                       	mov	ebx, edx
141472a11: 4d 89 c7                    	mov	r15, r8
141472a14: 44 89 e2                    	mov	edx, r12d
141472a17: eb c6                       	jmp	0x1414729df <.text+0x14719df>
141472a19: 66 41 0f 6f 02              	movdqa	xmm0, xmmword ptr [r10]
141472a1e: 66 0f d7 d8                 	pmovmskb	ebx, xmm0
141472a22: 49 81 c0 80 fa ff ff        	add	r8, -0x580
141472a29: 49 83 c2 10                 	add	r10, 0x10
141472a2d: 81 fb ff ff 00 00           	cmp	ebx, 0xffff
141472a33: 74 e4                       	je	0x141472a19 <.text+0x1471a19>
141472a35: 41 bf fe ff ff ff           	mov	r15d, 0xfffffffe
141472a3b: 41 29 df                    	sub	r15d, ebx
141472a3e: f7 d3                       	not	ebx
141472a40: 89 da                       	mov	edx, ebx
141472a42: 44 21 fa                    	and	edx, r15d
141472a45: 48 ff c9                    	dec	rcx
141472a48: 4d 89 c7                    	mov	r15, r8
141472a4b: eb 92                       	jmp	0x1414729df <.text+0x14719df>
141472a4d: 31 c9                       	xor	ecx, ecx
141472a4f: 4d 85 db                    	test	r11, r11
141472a52: 0f 84 b9 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a58: 48 85 c0                    	test	rax, rax
141472a5b: 0f 84 b0 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a61: 66 45 85 c9                 	test	r9w, r9w
141472a65: 0f 85 5e ff ff ff           	jne	0x1414729c9 <.text+0x14719c9>
141472a6b: 66 0f 6f 06                 	movdqa	xmm0, xmmword ptr [rsi]
141472a6f: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
141472a74: 49 81 c3 80 fa ff ff        	add	r11, -0x580
141472a7b: 48 83 c6 10                 	add	rsi, 0x10
141472a7f: 41 81 f9 ff ff 00 00        	cmp	r9d, 0xffff
141472a86: 74 e3                       	je	0x141472a6b <.text+0x1471a6b>
141472a88: 41 b8 fe ff ff ff           	mov	r8d, 0xfffffffe
141472a8e: 45 29 c8                    	sub	r8d, r9d
141472a91: 41 f7 d1                    	not	r9d
141472a94: 45 89 cc                    	mov	r12d, r9d
141472a97: 45 21 c4                    	and	r12d, r8d
141472a9a: e9 31 ff ff ff              	jmp	0x1414729d0 <.text+0x14719d0>
141472a9f: 49 01 df                    	add	r15, rbx
141472aa2: 41 c7 47 d8 e7 03 00 00     	mov	dword ptr [r15 - 0x28], 0x3e7
141472aaa: b3 01                       	mov	bl, 0x1
141472aac: 89 9d 04 03 00 00           	mov	dword ptr [rbp + 0x304], ebx
141472ab2: 4d 85 c0                    	test	r8, r8
141472ab5: 0f 85 40 ff ff ff           	jne	0x1414729fb <.text+0x14719fb>
141472abb: eb 92                       	jmp	0x141472a4f <.text+0x1471a4f>
141472abd: 44 8b ad 30 03 00 00        	mov	r13d, dword ptr [rbp + 0x330]
141472ac4: 4c 8b a5 10 01 00 00        	mov	r12, qword ptr [rbp + 0x110]
141472acb: 49 c7 c2 ff ff ff ff        	mov	r10, -0x1
141472ad2: 48 8b 95 00 01 00 00        	mov	rdx, qword ptr [rbp + 0x100]
141472ad9: e9 d0 f3 ff ff              	jmp	0x141471eae <.text+0x1470eae>
141472ade: b9 01 00 00 00              	mov	ecx, 0x1
141472ae3: 48 89 c2                    	mov	rdx, rax
141472ae6: e8 05 a2 ff ff              	call	0x14146ccf0 <.text+0x146bcf0>
141472aeb: 49 89 d7                    	mov	r15, rdx
141472aee: 48 85 c0                    	test	rax, rax
141472af1: 0f 84 98 01 00 00           	je	0x141472c8f <.text+0x1471c8f>
141472af7: 4c 89 bd d8 00 00 00        	mov	qword ptr [rbp + 0xd8], r15
141472afe: 48 8b 35 a3 98 1b 02        	mov	rsi, qword ptr [rip + 0x21b98a3] # 0x14362c3a8
141472b05: 48 8b 06                    	mov	rax, qword ptr [rsi]
141472b08: 48 83 f8 03                 	cmp	rax, 0x3
141472b0c: 0f 82 a8 00 00 00           	jb	0x141472bba <.text+0x1471bba>
141472b12: 48 8d 85 d8 00 00 00        	lea	rax, [rbp + 0xd8]
141472b19: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
141472b20: 48 8d 05 f9 a0 8d ff        	lea	rax, [rip - 0x725f07]   # 0x140d4cc20 <.text+0xd4bc20>
141472b27: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
141472b2e: 48 8d 05 db 1c 7f 01        	lea	rax, [rip + 0x17f1cdb]  # 0x142c64810
141472b35: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141472b3c: 48 c7 85 68 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x168], 0x2
141472b47: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
141472b52: 4c 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], r14
141472b59: 48 c7 85 78 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x178], 0x1
141472b64: 48 8d 0d c5 1c 7f 01        	lea	rcx, [rip + 0x17f1cc5]  # 0x142c64830
141472b6b: e8 10 b2 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141472b70: 48 8d 0d d1 15 7f 01        	lea	rcx, [rip + 0x17f15d1]  # 0x142c64148
141472b77: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141472b7e: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
141472b89: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141472b90: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
141472b9b: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141472ba2: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141472ba9: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141472bb0: ba 03 00 00 00              	mov	edx, 0x3
141472bb5: e8 76 e8 03 ff              	call	0x1404b1430 <.text+0x4b0430>
141472bba: 4c 89 e1                    	mov	rcx, r12
141472bbd: 48 8b 55 28                 	mov	rdx, qword ptr [rbp + 0x28]
141472bc1: e8 7a 05 20 00              	call	0x141673140 <.text+0x1672140>
141472bc6: 49 89 c7                    	mov	r15, rax
141472bc9: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
141472bcd: 48 8b 06                    	mov	rax, qword ptr [rsi]
141472bd0: 48 83 f8 03                 	cmp	rax, 0x3
141472bd4: 0f 82 a9 00 00 00           	jb	0x141472c83 <.text+0x1471c83>
141472bda: 48 8d 45 30                 	lea	rax, [rbp + 0x30]
141472bde: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
141472be5: 48 8d 05 d4 aa 91 ff        	lea	rax, [rip - 0x6e552c]   # 0x140d8d6c0 <.text+0xd8c6c0>
141472bec: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
141472bf3: 48 8d 05 76 1c 7f 01        	lea	rax, [rip + 0x17f1c76]  # 0x142c64870
141472bfa: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141472c01: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
141472c0c: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
141472c17: 4c 89 b5 70 01 00 00        	mov	qword ptr [rbp + 0x170], r14
141472c1e: 48 c7 85 78 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x178], 0x1
141472c29: 48 8d 0d 50 1c 7f 01        	lea	rcx, [rip + 0x17f1c50]  # 0x142c64880
141472c30: e8 4b b1 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141472c35: 48 8d 0d 0c 15 7f 01        	lea	rcx, [rip + 0x17f150c]  # 0x142c64148
141472c3c: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141472c43: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
141472c4e: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141472c55: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
141472c60: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141472c67: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
141472c6e: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141472c75: ba 03 00 00 00              	mov	edx, 0x3
141472c7a: e8 b1 e7 03 ff              	call	0x1404b1430 <.text+0x4b0430>
141472c7f: 4c 8b 7d 30                 	mov	r15, qword ptr [rbp + 0x30]
141472c83: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
141472c8a: e8 a1 9f 8d ff              	call	0x140d4cc30 <.text+0xd4bc30>
141472c8f: f2 0f 10 b5 c0 00 00 00     	movsd	xmm6, qword ptr [rbp + 0xc0]
141472c97: 48 8b 55 60                 	mov	rdx, qword ptr [rbp + 0x60]
141472c9b: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
141472c9f: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
141472ca3: 4c 8d 04 c2                 	lea	r8, [rdx + 8*rax]
141472ca7: 4c 8d 0d 9a 11 7f 01        	lea	r9, [rip + 0x17f119a]   # 0x142c63e48
141472cae: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
141472cb5: e8 06 b2 e9 ff              	call	0x14130dec0 <.text+0x130cec0>
141472cba: 48 8b 55 78                 	mov	rdx, qword ptr [rbp + 0x78]
141472cbe: 4c 8b 85 80 00 00 00        	mov	r8, qword ptr [rbp + 0x80]
141472cc5: 49 c1 e0 04                 	shl	r8, 0x4
141472cc9: 49 01 d0                    	add	r8, rdx
141472ccc: 4c 8d 0d 75 11 7f 01        	lea	r9, [rip + 0x17f1175]   # 0x142c63e48
141472cd3: 48 8d 8d 50 02 00 00        	lea	rcx, [rbp + 0x250]
141472cda: e8 e1 29 e9 ff              	call	0x1413056c0 <.text+0x13046c0>
141472cdf: 48 8b 75 40                 	mov	rsi, qword ptr [rbp + 0x40]
141472ce3: 48 89 f0                    	mov	rax, rsi
141472ce6: 48 f7 d8                    	neg	rax
141472ce9: 71 16                       	jno	0x141472d01 <.text+0x1471d01>
141472ceb: 49 89 fe                    	mov	r14, rdi
141472cee: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
141472cf8: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
141472cff: eb 1a                       	jmp	0x141472d1b <.text+0x1471d1b>
141472d01: 49 89 fe                    	mov	r14, rdi
141472d04: 4c 8d 05 a5 1e 7f 01        	lea	r8, [rip + 0x17f1ea5]   # 0x142c64bb0
141472d0b: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141472d12: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
141472d16: e8 15 5c e8 ff              	call	0x1412f8930 <.text+0x12f7930>
141472d1b: 48 8b 55 18                 	mov	rdx, qword ptr [rbp + 0x18]
141472d1f: 48 89 d0                    	mov	rax, rdx
141472d22: 48 c1 e8 20                 	shr	rax, 0x20
141472d26: 4c 89 bd a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], r15
141472d2d: 8b 8d 1c 01 00 00           	mov	ecx, dword ptr [rbp + 0x11c]
141472d33: 89 8d 20 02 00 00           	mov	dword ptr [rbp + 0x220], ecx
141472d39: f3 44 0f 11 9d 24 02 00 00  	movss	dword ptr [rbp + 0x224], xmm11
141472d42: 89 95 28 02 00 00           	mov	dword ptr [rbp + 0x228], edx
141472d48: 89 85 2c 02 00 00           	mov	dword ptr [rbp + 0x22c], eax
141472d4e: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
141472d52: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
141472d59: 44 0f 11 95 38 02 00 00     	movups	xmmword ptr [rbp + 0x238], xmm10
141472d61: 8b 85 f4 02 00 00           	mov	eax, dword ptr [rbp + 0x2f4]
141472d67: 89 85 f4 01 00 00           	mov	dword ptr [rbp + 0x1f4], eax
141472d6d: 44 0f 11 85 f8 01 00 00     	movups	xmmword ptr [rbp + 0x1f8], xmm8
141472d75: 44 0f 11 a5 08 02 00 00     	movups	xmmword ptr [rbp + 0x208], xmm12
141472d7d: f3 44 0f 11 ad 18 02 00 00  	movss	dword ptr [rbp + 0x218], xmm13
141472d86: 0f b7 85 16 03 00 00        	movzx	eax, word ptr [rbp + 0x316]
141472d8d: 66 89 85 1c 02 00 00        	mov	word ptr [rbp + 0x21c], ax
141472d94: 0f 13 b5 48 02 00 00        	movlps	qword ptr [rbp + 0x248], xmm6
141472d9b: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
141472da2: 0f 29 85 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm0
141472da9: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
141472db0: 48 89 85 70 01 00 00        	mov	qword ptr [rbp + 0x170], rax
141472db7: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
141472dbe: 0f 11 85 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm0
141472dc5: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
141472dcc: 48 89 85 88 01 00 00        	mov	qword ptr [rbp + 0x188], rax
141472dd3: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
141472dda: 48 89 85 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rax
141472de1: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
141472de8: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
141472def: 0f 29 bd b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm7
141472df6: 8b 85 24 01 00 00           	mov	eax, dword ptr [rbp + 0x124]
141472dfc: 89 85 c0 01 00 00           	mov	dword ptr [rbp + 0x1c0], eax
141472e02: 8b 85 28 01 00 00           	mov	eax, dword ptr [rbp + 0x128]
141472e08: 88 85 c4 01 00 00           	mov	byte ptr [rbp + 0x1c4], al
141472e0e: 8b 85 20 01 00 00           	mov	eax, dword ptr [rbp + 0x120]
141472e14: 89 85 c8 01 00 00           	mov	dword ptr [rbp + 0x1c8], eax
141472e1a: f3 44 0f 7f b5 cc 01 00 00  	movdqu	xmmword ptr [rbp + 0x1cc], xmm14
141472e23: 44 0f 11 8d dc 01 00 00     	movups	xmmword ptr [rbp + 0x1dc], xmm9
141472e2b: f3 44 0f 11 bd ec 01 00 00  	movss	dword ptr [rbp + 0x1ec], xmm15
141472e34: 8b 85 2c 01 00 00           	mov	eax, dword ptr [rbp + 0x12c]
141472e3a: 88 85 f0 01 00 00           	mov	byte ptr [rbp + 0x1f0], al
141472e40: 8b 85 30 01 00 00           	mov	eax, dword ptr [rbp + 0x130]
141472e46: 88 85 f1 01 00 00           	mov	byte ptr [rbp + 0x1f1], al
141472e4c: 0f b6 85 2f 03 00 00        	movzx	eax, byte ptr [rbp + 0x32f]
141472e53: 88 85 f2 01 00 00           	mov	byte ptr [rbp + 0x1f2], al
141472e59: e8 62 a0 ff ff              	call	0x14146cec0 <.text+0x146bec0>
141472e5e: 48 89 c7                    	mov	rdi, rax
141472e61: 48 8d 95 60 01 00 00        	lea	rdx, [rbp + 0x160]
141472e68: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
141472e6e: 48 89 c1                    	mov	rcx, rax
141472e71: e8 8a 0d 4b 01              	call	0x142923c00 <.text+0x2922c00>
141472e76: 41 83 fd 1b                 	cmp	r13d, 0x1b
141472e7a: 0f 95 c0                    	setne	al
141472e7d: 0a 85 36 03 00 00           	or	al, byte ptr [rbp + 0x336]
141472e83: 44 8b 85 f0 02 00 00        	mov	r8d, dword ptr [rbp + 0x2f0]
141472e8a: 41 80 e0 01                 	and	r8b, 0x1
141472e8e: 8b 95 04 03 00 00           	mov	edx, dword ptr [rbp + 0x304]
141472e94: 80 e2 01                    	and	dl, 0x1
141472e97: 49 89 3e                    	mov	qword ptr [r14], rdi
141472e9a: 4c 89 f7                    	mov	rdi, r14
141472e9d: 8b 8d d4 00 00 00           	mov	ecx, dword ptr [rbp + 0xd4]
141472ea3: 41 89 4e 08                 	mov	dword ptr [r14 + 0x8], ecx
141472ea7: 41 88 46 0c                 	mov	byte ptr [r14 + 0xc], al
141472eab: 45 88 46 0d                 	mov	byte ptr [r14 + 0xd], r8b
141472eaf: 41 88 56 0e                 	mov	byte ptr [r14 + 0xe], dl
141472eb3: 41 88 5e 0f                 	mov	byte ptr [r14 + 0xf], bl
141472eb7: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
141472ebb: e8 30 7e 41 ff              	call	0x14088acf0 <.text+0x889cf0>
141472ec0: 48 89 f0                    	mov	rax, rsi
141472ec3: 48 f7 d8                    	neg	rax
141472ec6: 70 19                       	jo	0x141472ee1 <.text+0x1471ee1>
141472ec8: 73 17                       	jae	0x141472ee1 <.text+0x1471ee1>
141472eca: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
141472ece: 48 c1 e6 03                 	shl	rsi, 0x3
141472ed2: 48 8d 14 b6                 	lea	rdx, [rsi + 4*rsi]
141472ed6: 41 b8 04 00 00 00           	mov	r8d, 0x4
141472edc: e8 3f 0d 1a ff              	call	0x140613c20 <.text+0x612c20>
141472ee1: 48 8d 8d 38 01 00 00        	lea	rcx, [rbp + 0x138]
141472ee8: e8 a3 ab 3e ff              	call	0x14085da90 <.text+0x85ca90>
141472eed: e9 8a e5 ff ff              	jmp	0x14147147c <.text+0x147047c>
141472ef2: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141472ef9: 48 8d 0d d0 24 63 01        	lea	rcx, [rip + 0x16324d0]  # 0x142aa53d0
141472f00: 4c 8d 05 b1 4b 66 01        	lea	r8, [rip + 0x1664bb1]   # 0x142ad7ab8
141472f07: ba 1d 00 00 00              	mov	edx, 0x1d
141472f0c: e8 1f 65 46 01              	call	0x1428d9430 <.text+0x28d8430>
141472f11: e9 6d 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f16: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141472f1d: 48 8d 0d 74 25 63 01        	lea	rcx, [rip + 0x1632574]  # 0x142aa5498
141472f24: 4c 8d 05 8d 4b 66 01        	lea	r8, [rip + 0x1664b8d]   # 0x142ad7ab8
141472f2b: ba 22 00 00 00              	mov	edx, 0x22
141472f30: e8 fb 64 46 01              	call	0x1428d9430 <.text+0x28d8430>
141472f35: e9 49 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f3a: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141472f41: 48 8d 0d 50 25 63 01        	lea	rcx, [rip + 0x1632550]  # 0x142aa5498
141472f48: 4c 8d 05 81 4b 66 01        	lea	r8, [rip + 0x1664b81]   # 0x142ad7ad0
141472f4f: ba 22 00 00 00              	mov	edx, 0x22
141472f54: e8 d7 64 46 01              	call	0x1428d9430 <.text+0x28d8430>
141472f59: e9 25 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f5e: 41 83 fd 13                 	cmp	r13d, 0x13
141472f62: 0f 9c c3                    	setl	bl
141472f65: 0f 8d 30 02 00 00           	jge	0x14147319b <.text+0x147219b>
141472f6b: 48 8b 05 36 94 1b 02        	mov	rax, qword ptr [rip + 0x21b9436] # 0x14362c3a8
141472f72: 48 8b 00                    	mov	rax, qword ptr [rax]
141472f75: 48 83 f8 03                 	cmp	rax, 0x3
141472f79: 0f 82 bd 00 00 00           	jb	0x14147303c <.text+0x147203c>
141472f7f: 48 8d 05 0a 17 7f 01        	lea	rax, [rip + 0x17f170a]  # 0x142c64690
141472f86: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
141472f8d: 48 8d 05 ec a5 91 ff        	lea	rax, [rip - 0x6e5a14]   # 0x140d8d580 <.text+0xd8c580>
141472f94: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
141472f9b: 48 8d 05 2e 17 7f 01        	lea	rax, [rip + 0x17f172e]  # 0x142c646d0
141472fa2: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141472fa9: 48 c7 85 68 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x168], 0x2
141472fb4: 48 c7 85 80 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x180], 0x0
141472fbf: 48 8d 85 50 02 00 00        	lea	rax, [rbp + 0x250]
141472fc6: 48 89 85 70 01 00 00        	mov	qword ptr [rbp + 0x170], rax
141472fcd: 48 c7 85 78 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x178], 0x1
141472fd8: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141472fdf: 48 8d 0d 0a 17 7f 01        	lea	rcx, [rip + 0x17f170a]  # 0x142c646f0
141472fe6: e8 95 ad 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141472feb: 48 8d 0d 56 11 7f 01        	lea	rcx, [rip + 0x17f1156]  # 0x142c64148
141472ff2: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
141472ff9: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
141473004: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
14147300b: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
141473016: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
14147301d: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473024: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14147302b: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141473032: ba 03 00 00 00              	mov	edx, 0x3
141473037: e8 f4 e3 03 ff              	call	0x1404b1430 <.text+0x4b0430>
14147303c: 41 83 fd 11                 	cmp	r13d, 0x11
141473040: 0f 8d 55 01 00 00           	jge	0x14147319b <.text+0x147219b>
141473046: 48 c7 85 b0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x0
141473051: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
141473055: 48 85 c0                    	test	rax, rax
141473058: 0f 84 3d 01 00 00           	je	0x14147319b <.text+0x147219b>
14147305e: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
141473062: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
141473066: 48 8d 14 c1                 	lea	rdx, [rcx + 8*rax]
14147306a: 45 31 c0                    	xor	r8d, r8d
14147306d: 49 c7 c1 ff ff ff ff        	mov	r9, -0x1
141473074: f3 0f 10 05 2c 15 4c 01     	movss	xmm0, dword ptr [rip + 0x14c152c] # 0x1429345a8
14147307c: f3 0f 10 0d ac c9 60 01     	movss	xmm1, dword ptr [rip + 0x160c9ac] # 0x142a7fa30
141473084: 49 ba 01 00 00 00 00 00 00 80       	movabs	r10, -0x7fffffffffffffff
14147308e: 0f 28 15 4b 60 7b 01        	movaps	xmm2, xmmword ptr [rip + 0x17b604b] # 0x142c290e0
141473095: 0f 28 1d 24 79 64 01        	movaps	xmm3, xmmword ptr [rip + 0x1647924] # 0x142aba9c0
14147309c: eb 0d                       	jmp	0x1414730ab <.text+0x14720ab>
14147309e: 48 83 c1 28                 	add	rcx, 0x28
1414730a2: 48 39 d1                    	cmp	rcx, rdx
1414730a5: 0f 84 f0 00 00 00           	je	0x14147319b <.text+0x147219b>
1414730ab: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1414730af: 48 85 c0                    	test	rax, rax
1414730b2: 74 ea                       	je	0x14147309e <.text+0x147209e>
1414730b4: 4c 8b 59 08                 	mov	r11, qword ptr [rcx + 0x8]
1414730b8: 48 c1 e0 03                 	shl	rax, 0x3
1414730bc: 48 8d 34 40                 	lea	rsi, [rax + 2*rax]
1414730c0: 31 ff                       	xor	edi, edi
1414730c2: eb 10                       	jmp	0x1414730d4 <.text+0x14720d4>
1414730c4: 49 83 3c 3b 00              	cmp	qword ptr [r11 + rdi], 0x0
1414730c9: 74 6f                       	je	0x14147313a <.text+0x147213a>
1414730cb: 48 83 c7 18                 	add	rdi, 0x18
1414730cf: 48 39 fe                    	cmp	rsi, rdi
1414730d2: 74 ca                       	je	0x14147309e <.text+0x147209e>
1414730d4: 49 83 3c 3b 00              	cmp	qword ptr [r11 + rdi], 0x0
1414730d9: 75 f0                       	jne	0x1414730cb <.text+0x14720cb>
1414730db: 4d 8b 74 3b 08              	mov	r14, qword ptr [r11 + rdi + 0x8]
1414730e0: b8 01 00 00 00              	mov	eax, 0x1
1414730e5: f0                          	lock
1414730e6: 4d 0f b1 4e 08              	cmpxchg	qword ptr [r14 + 0x8], r9
1414730eb: 0f 85 1d 02 00 00           	jne	0x14147330e <.text+0x147230e>
1414730f1: 49 8b 06                    	mov	rax, qword ptr [r14]
1414730f4: 49 c7 46 08 01 00 00 00     	mov	qword ptr [r14 + 0x8], 0x1
1414730fc: 48 83 f8 01                 	cmp	rax, 0x1
141473100: 0f 85 08 02 00 00           	jne	0x14147330e <.text+0x147230e>
141473106: 49 8b 44 3b 08              	mov	rax, qword ptr [r11 + rdi + 0x8]
14147310b: 4c 3b 40 10                 	cmp	r8, qword ptr [rax + 0x10]
14147310f: 71 b3                       	jno	0x1414730c4 <.text+0x14720c4>
141473111: f3 0f 10 60 34              	movss	xmm4, dword ptr [rax + 0x34]
141473116: f3 0f 51 e4                 	sqrtss	xmm4, xmm4
14147311a: f3 0f 59 e0                 	mulss	xmm4, xmm0
14147311e: 0f 57 ed                    	xorps	xmm5, xmm5
141473121: f3 0f 5f ec                 	maxss	xmm5, xmm4
141473125: 0f 28 e1                    	movaps	xmm4, xmm1
141473128: f3 0f 5d e5                 	minss	xmm4, xmm5
14147312c: f3 0f 11 60 34              	movss	dword ptr [rax + 0x34], xmm4
141473131: c6 85 36 03 00 00 01        	mov	byte ptr [rbp + 0x336], 0x1
141473138: eb 8a                       	jmp	0x1414730c4 <.text+0x14720c4>
14147313a: 4d 8b 74 3b 08              	mov	r14, qword ptr [r11 + rdi + 0x8]
14147313f: b8 01 00 00 00              	mov	eax, 0x1
141473144: f0                          	lock
141473145: 4d 0f b1 4e 08              	cmpxchg	qword ptr [r14 + 0x8], r9
14147314a: 0f 85 d3 01 00 00           	jne	0x141473323 <.text+0x1472323>
141473150: 49 8b 06                    	mov	rax, qword ptr [r14]
141473153: 49 c7 46 08 01 00 00 00     	mov	qword ptr [r14 + 0x8], 0x1
14147315b: 48 83 f8 01                 	cmp	rax, 0x1
14147315f: 0f 85 be 01 00 00           	jne	0x141473323 <.text+0x1472323>
141473165: 49 8b 44 3b 08              	mov	rax, qword ptr [r11 + rdi + 0x8]
14147316a: 4c 39 50 10                 	cmp	qword ptr [rax + 0x10], r10
14147316e: 0f 85 57 ff ff ff           	jne	0x1414730cb <.text+0x14720cb>
141473174: f2 0f 10 60 1c              	movsd	xmm4, qword ptr [rax + 0x1c]
141473179: 0f 51 e4                    	sqrtps	xmm4, xmm4
14147317c: 0f 59 e2                    	mulps	xmm4, xmm2
14147317f: 0f 57 ed                    	xorps	xmm5, xmm5
141473182: 0f 5f ec                    	maxps	xmm5, xmm4
141473185: 0f 28 e3                    	movaps	xmm4, xmm3
141473188: 0f 5d e5                    	minps	xmm4, xmm5
14147318b: 0f 13 60 1c                 	movlps	qword ptr [rax + 0x1c], xmm4
14147318f: c6 85 36 03 00 00 01        	mov	byte ptr [rbp + 0x336], 0x1
141473196: e9 30 ff ff ff              	jmp	0x1414730cb <.text+0x14720cb>
14147319b: 80 bd 36 03 00 00 01        	cmp	byte ptr [rbp + 0x336], 0x1
1414731a2: 75 09                       	jne	0x1414731ad <.text+0x14721ad>
1414731a4: 0f b6 95 d8 00 00 00        	movzx	edx, byte ptr [rbp + 0xd8]
1414731ab: eb 09                       	jmp	0x1414731b6 <.text+0x14721b6>
1414731ad: c6 85 d8 00 00 00 00        	mov	byte ptr [rbp + 0xd8], 0x0
1414731b4: 31 d2                       	xor	edx, edx
1414731b6: c6 85 37 03 00 00 00        	mov	byte ptr [rbp + 0x337], 0x0
1414731bd: 80 e2 01                    	and	dl, 0x1
1414731c0: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
1414731c4: e8 b7 37 61 ff              	call	0x140a86980 <.text+0xa85980>
1414731c9: f6 85 36 03 00 00 01        	test	byte ptr [rbp + 0x336], 0x1
1414731d0: 0f 84 ba 00 00 00           	je	0x141473290 <.text+0x1472290>
1414731d6: 48 8b 05 cb 91 1b 02        	mov	rax, qword ptr [rip + 0x21b91cb] # 0x14362c3a8
1414731dd: 48 8b 00                    	mov	rax, qword ptr [rax]
1414731e0: 48 83 f8 03                 	cmp	rax, 0x3
1414731e4: 48 8b 7d 38                 	mov	rdi, qword ptr [rbp + 0x38]
1414731e8: 0f 28 7d e0                 	movaps	xmm7, xmmword ptr [rbp - 0x20]
1414731ec: 44 0f 28 4d f0              	movaps	xmm9, xmmword ptr [rbp - 0x10]
1414731f1: f3 44 0f 10 bd 34 01 00 00  	movss	xmm15, dword ptr [rbp + 0x134]
1414731fa: 4c 8d b5 50 02 00 00        	lea	r14, [rbp + 0x250]
141473201: 0f 82 ba ea ff ff           	jb	0x141471cc1 <.text+0x1470cc1>
141473207: 48 8d 05 92 15 7f 01        	lea	rax, [rip + 0x17f1592]  # 0x142c647a0
14147320e: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
141473215: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
141473220: 48 c7 85 70 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x170], 0x8
14147322b: 0f 57 c0                    	xorps	xmm0, xmm0
14147322e: 0f 11 85 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm0
141473235: 48 8d 0d 74 15 7f 01        	lea	rcx, [rip + 0x17f1574]  # 0x142c647b0
14147323c: e8 3f ab 20 ff              	call	0x14067dd80 <.text+0x67cd80>
141473241: 48 8d 0d 00 0f 7f 01        	lea	rcx, [rip + 0x17f0f00]  # 0x142c64148
141473248: 48 89 8d 70 02 00 00        	mov	qword ptr [rbp + 0x270], rcx
14147324f: 48 c7 85 78 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x278], 0x26
14147325a: 48 89 8d 80 02 00 00        	mov	qword ptr [rbp + 0x280], rcx
141473261: 48 c7 85 88 02 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x288], 0x26
14147326c: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
141473273: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14147327a: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
141473281: ba 03 00 00 00              	mov	edx, 0x3
141473286: e8 a5 e1 03 ff              	call	0x1404b1430 <.text+0x4b0430>
14147328b: e9 31 ea ff ff              	jmp	0x141471cc1 <.text+0x1470cc1>
141473290: 48 8b 7d 38                 	mov	rdi, qword ptr [rbp + 0x38]
141473294: 0f 28 7d e0                 	movaps	xmm7, xmmword ptr [rbp - 0x20]
141473298: 44 0f 28 4d f0              	movaps	xmm9, xmmword ptr [rbp - 0x10]
14147329d: f3 44 0f 10 bd 34 01 00 00  	movss	xmm15, dword ptr [rbp + 0x134]
1414732a6: 8b 85 04 03 00 00           	mov	eax, dword ptr [rbp + 0x304]
1414732ac: 4c 8d b5 50 02 00 00        	lea	r14, [rbp + 0x250]
1414732b3: e9 6b e9 ff ff              	jmp	0x141471c23 <.text+0x1470c23>
1414732b8: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
1414732bf: 48 8d 0d 32 16 7f 01        	lea	rcx, [rip + 0x17f1632]  # 0x142c648f8
1414732c6: e8 45 60 46 01              	call	0x1428d9310 <.text+0x28d8310>
1414732cb: e9 b3 00 00 00              	jmp	0x141473383 <.text+0x1472383>
1414732d0: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
1414732d7: 48 8d 0d 32 16 7f 01        	lea	rcx, [rip + 0x17f1632]  # 0x142c64910
1414732de: e8 2d 60 46 01              	call	0x1428d9310 <.text+0x28d8310>
1414732e3: e9 9b 00 00 00              	jmp	0x141473383 <.text+0x1472383>
1414732e8: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
1414732ef: 48 8d 0d 32 16 7f 01        	lea	rcx, [rip + 0x17f1632]  # 0x142c64928
1414732f6: e8 15 60 46 01              	call	0x1428d9310 <.text+0x28d8310>
1414732fb: e9 83 00 00 00              	jmp	0x141473383 <.text+0x1472383>
141473300: 48 8d 0d 71 bd 7e 01        	lea	rcx, [rip + 0x17ebd71]  # 0x142c5f078
141473307: e8 04 60 46 01              	call	0x1428d9310 <.text+0x28d8310>
14147330c: eb 75                       	jmp	0x141473383 <.text+0x1472383>
14147330e: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473315: 48 8d 0d 7c 15 7f 01        	lea	rcx, [rip + 0x17f157c]  # 0x142c64898
14147331c: e8 ef 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
141473321: eb 60                       	jmp	0x141473383 <.text+0x1472383>
141473323: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
14147332a: 48 8d 0d 7f 15 7f 01        	lea	rcx, [rip + 0x17f157f]  # 0x142c648b0
141473331: e8 da 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
141473336: eb 4b                       	jmp	0x141473383 <.text+0x1472383>
141473338: c6 85 35 03 00 00 00        	mov	byte ptr [rbp + 0x335], 0x0
14147333f: 48 8d 0d b2 e0 67 01        	lea	rcx, [rip + 0x167e0b2]  # 0x142af13f8
141473346: e8 c5 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
14147334b: eb 36                       	jmp	0x141473383 <.text+0x1472383>
14147334d: 48 8d 0d a4 e0 67 01        	lea	rcx, [rip + 0x167e0a4]  # 0x142af13f8
141473354: e8 b7 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
141473359: eb 28                       	jmp	0x141473383 <.text+0x1472383>
14147335b: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473362: 48 8d 0d 5f 15 7f 01        	lea	rcx, [rip + 0x17f155f]  # 0x142c648c8
141473369: e8 a2 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
14147336e: eb 13                       	jmp	0x141473383 <.text+0x1472383>
141473370: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473377: 48 8d 0d 62 15 7f 01        	lea	rcx, [rip + 0x17f1562]  # 0x142c648e0
14147337e: e8 8d 5f 46 01              	call	0x1428d9310 <.text+0x28d8310>
141473383: 0f 0b                       	ud2
141473385: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
141473390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473395: 55                          	push	rbp
141473396: 41 57                       	push	r15
141473398: 41 56                       	push	r14
14147339a: 41 55                       	push	r13
14147339c: 41 54                       	push	r12
14147339e: 56                          	push	rsi
14147339f: 57                          	push	rdi
1414733a0: 53                          	push	rbx
1414733a1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1414733a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414733af: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1414733b5: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
1414733bc: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1414733c2: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1414733c8: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1414733ce: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1414733d7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1414733e0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1414733e9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414733f1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414733f9: 80 bd 2e 03 00 00 00        	cmp	byte ptr [rbp + 0x32e], 0x0
141473400: 75 5d                       	jne	0x14147345f <.text+0x147245f>
141473402: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14147340a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473412: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14147341b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473424: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
14147342d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473433: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473439: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14147343f: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
141473445: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14147344b: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473452: 5b                          	pop	rbx
141473453: 5f                          	pop	rdi
141473454: 5e                          	pop	rsi
141473455: 41 5c                       	pop	r12
141473457: 41 5d                       	pop	r13
141473459: 41 5e                       	pop	r14
14147345b: 41 5f                       	pop	r15
14147345d: 5d                          	pop	rbp
14147345e: c3                          	ret
14147345f: 48 8d 8d 50 02 00 00        	lea	rcx, [rbp + 0x250]
141473466: e8 95 45 17 ff              	call	0x1405e7a00 <.text+0x5e6a00>
14147346b: eb 95                       	jmp	0x141473402 <.text+0x1472402>
14147346d: 0f 1f 00                    	nop	dword ptr [rax]
141473470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473475: 55                          	push	rbp
141473476: 41 57                       	push	r15
141473478: 41 56                       	push	r14
14147347a: 41 55                       	push	r13
14147347c: 41 54                       	push	r12
14147347e: 56                          	push	rsi
14147347f: 57                          	push	rdi
141473480: 53                          	push	rbx
141473481: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473488: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147348f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473495: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147349c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
1414734a2: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
1414734a8: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
1414734ae: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1414734b7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1414734c0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1414734c9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414734d1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414734d9: 48 8b 95 f0 00 00 00        	mov	rdx, qword ptr [rbp + 0xf0]
1414734e0: 48 85 d2                    	test	rdx, rdx
1414734e3: 74 12                       	je	0x1414734f7 <.text+0x14724f7>
1414734e5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1414734eb: 48 8b 8d f8 00 00 00        	mov	rcx, qword ptr [rbp + 0xf8]
1414734f2: e8 29 07 1a ff              	call	0x140613c20 <.text+0x612c20>
1414734f7: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414734ff: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473507: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473510: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473519: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473522: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473528: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14147352e: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473534: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
14147353a: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473540: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473547: 5b                          	pop	rbx
141473548: 5f                          	pop	rdi
141473549: 5e                          	pop	rsi
14147354a: 41 5c                       	pop	r12
14147354c: 41 5d                       	pop	r13
14147354e: 41 5e                       	pop	r14
141473550: 41 5f                       	pop	r15
141473552: 5d                          	pop	rbp
141473553: c3                          	ret
141473554: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141473560: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473565: 55                          	push	rbp
141473566: 41 57                       	push	r15
141473568: 41 56                       	push	r14
14147356a: 41 55                       	push	r13
14147356c: 41 54                       	push	r12
14147356e: 56                          	push	rsi
14147356f: 57                          	push	rdi
141473570: 53                          	push	rbx
141473571: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473578: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147357f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473585: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147358c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473592: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473598: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147359e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1414735a7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1414735b0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1414735b9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414735c1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414735c9: 80 bd 2d 03 00 00 00        	cmp	byte ptr [rbp + 0x32d], 0x0
1414735d0: 74 0c                       	je	0x1414735de <.text+0x14725de>
1414735d2: 48 8d 8d 50 02 00 00        	lea	rcx, [rbp + 0x250]
1414735d9: e8 c2 52 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
1414735de: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414735e6: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414735ee: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414735f7: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473600: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473609: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14147360f: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473615: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
14147361b: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141473622: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473628: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14147362f: 5b                          	pop	rbx
141473630: 5f                          	pop	rdi
141473631: 5e                          	pop	rsi
141473632: 41 5c                       	pop	r12
141473634: 41 5d                       	pop	r13
141473636: 41 5e                       	pop	r14
141473638: 41 5f                       	pop	r15
14147363a: 5d                          	pop	rbp
14147363b: c3                          	ret
14147363c: 0f 1f 40 00                 	nop	dword ptr [rax]
141473640: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473645: 55                          	push	rbp
141473646: 41 57                       	push	r15
141473648: 41 56                       	push	r14
14147364a: 41 55                       	push	r13
14147364c: 41 54                       	push	r12
14147364e: 56                          	push	rsi
14147364f: 57                          	push	rdi
141473650: 53                          	push	rbx
141473651: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473658: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147365f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473665: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147366c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473672: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473678: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147367e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473687: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473690: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473699: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414736a1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414736a9: 48 8d 8d 38 01 00 00        	lea	rcx, [rbp + 0x138]
1414736b0: e8 db a3 3e ff              	call	0x14085da90 <.text+0x85ca90>
1414736b5: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414736bd: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414736c5: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414736ce: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414736d7: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414736e0: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414736e6: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414736ec: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414736f2: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414736f9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414736ff: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473706: 5b                          	pop	rbx
141473707: 5f                          	pop	rdi
141473708: 5e                          	pop	rsi
141473709: 41 5c                       	pop	r12
14147370b: 41 5d                       	pop	r13
14147370d: 41 5e                       	pop	r14
14147370f: 41 5f                       	pop	r15
141473711: 5d                          	pop	rbp
141473712: c3                          	ret
141473713: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141473720: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473725: 55                          	push	rbp
141473726: 41 57                       	push	r15
141473728: 41 56                       	push	r14
14147372a: 41 55                       	push	r13
14147372c: 41 54                       	push	r12
14147372e: 56                          	push	rsi
14147372f: 57                          	push	rdi
141473730: 53                          	push	rbx
141473731: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473738: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147373f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473745: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147374c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473752: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473758: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147375e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473767: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473770: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473779: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473781: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473789: 80 bd 2c 03 00 00 00        	cmp	byte ptr [rbp + 0x32c], 0x0
141473790: 74 0c                       	je	0x14147379e <.text+0x147279e>
141473792: 48 8d 8d 50 02 00 00        	lea	rcx, [rbp + 0x250]
141473799: e8 02 51 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
14147379e: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414737a6: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414737ae: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414737b7: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414737c0: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414737c9: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414737cf: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414737d5: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414737db: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414737e2: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414737e8: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414737ef: 5b                          	pop	rbx
1414737f0: 5f                          	pop	rdi
1414737f1: 5e                          	pop	rsi
1414737f2: 41 5c                       	pop	r12
1414737f4: 41 5d                       	pop	r13
1414737f6: 41 5e                       	pop	r14
1414737f8: 41 5f                       	pop	r15
1414737fa: 5d                          	pop	rbp
1414737fb: c3                          	ret
1414737fc: 0f 1f 40 00                 	nop	dword ptr [rax]
141473800: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473805: 55                          	push	rbp
141473806: 41 57                       	push	r15
141473808: 41 56                       	push	r14
14147380a: 41 55                       	push	r13
14147380c: 41 54                       	push	r12
14147380e: 56                          	push	rsi
14147380f: 57                          	push	rdi
141473810: 53                          	push	rbx
141473811: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473818: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147381f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473825: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147382c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473832: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473838: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147383e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473847: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473850: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473859: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473861: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473869: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14147386d: 48 8d 0c 45 00 00 00 00     	lea	rcx, [2*rax]
141473875: 48 85 c9                    	test	rcx, rcx
141473878: 74 17                       	je	0x141473891 <.text+0x1472891>
14147387a: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
14147387e: 48 c1 e0 03                 	shl	rax, 0x3
141473882: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
141473886: 41 b8 04 00 00 00           	mov	r8d, 0x4
14147388c: e8 8f 03 1a ff              	call	0x140613c20 <.text+0x612c20>
141473891: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473899: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414738a1: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414738aa: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414738b3: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414738bc: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414738c2: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414738c8: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414738ce: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414738d5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414738db: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414738e2: 5b                          	pop	rbx
1414738e3: 5f                          	pop	rdi
1414738e4: 5e                          	pop	rsi
1414738e5: 41 5c                       	pop	r12
1414738e7: 41 5d                       	pop	r13
1414738e9: 41 5e                       	pop	r14
1414738eb: 41 5f                       	pop	r15
1414738ed: 5d                          	pop	rbp
1414738ee: c3                          	ret
1414738ef: 90                          	nop
1414738f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414738f5: 55                          	push	rbp
1414738f6: 41 57                       	push	r15
1414738f8: 41 56                       	push	r14
1414738fa: 41 55                       	push	r13
1414738fc: 41 54                       	push	r12
1414738fe: 56                          	push	rsi
1414738ff: 57                          	push	rdi
141473900: 53                          	push	rbx
141473901: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473908: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147390f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473915: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147391c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473922: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473928: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147392e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473937: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473940: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473949: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473951: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473959: 83 bd 20 03 00 00 00        	cmp	dword ptr [rbp + 0x320], 0x0
141473960: 74 0c                       	je	0x14147396e <.text+0x147296e>
141473962: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
141473969: e8 32 4f 17 ff              	call	0x1405e88a0 <.text+0x5e78a0>
14147396e: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473976: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14147397e: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473987: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473990: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473999: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14147399f: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414739a5: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414739ab: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414739b2: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414739b8: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414739bf: 5b                          	pop	rbx
1414739c0: 5f                          	pop	rdi
1414739c1: 5e                          	pop	rsi
1414739c2: 41 5c                       	pop	r12
1414739c4: 41 5d                       	pop	r13
1414739c6: 41 5e                       	pop	r14
1414739c8: 41 5f                       	pop	r15
1414739ca: 5d                          	pop	rbp
1414739cb: c3                          	ret
1414739cc: 0f 1f 40 00                 	nop	dword ptr [rax]
1414739d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414739d5: 55                          	push	rbp
1414739d6: 41 57                       	push	r15
1414739d8: 41 56                       	push	r14
1414739da: 41 55                       	push	r13
1414739dc: 41 54                       	push	r12
1414739de: 56                          	push	rsi
1414739df: 57                          	push	rdi
1414739e0: 53                          	push	rbx
1414739e1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
1414739e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414739ef: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
1414739f5: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
1414739fc: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473a02: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473a08: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473a0e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473a17: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473a20: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473a29: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473a31: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473a39: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
141473a3d: e8 ae 72 41 ff              	call	0x14088acf0 <.text+0x889cf0>
141473a42: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473a4a: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473a52: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473a5b: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473a64: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473a6d: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473a73: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473a79: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473a7f: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141473a86: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473a8c: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473a93: 5b                          	pop	rbx
141473a94: 5f                          	pop	rdi
141473a95: 5e                          	pop	rsi
141473a96: 41 5c                       	pop	r12
141473a98: 41 5d                       	pop	r13
141473a9a: 41 5e                       	pop	r14
141473a9c: 41 5f                       	pop	r15
141473a9e: 5d                          	pop	rbp
141473a9f: c3                          	ret
141473aa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473aa5: 55                          	push	rbp
141473aa6: 41 57                       	push	r15
141473aa8: 41 56                       	push	r14
141473aaa: 41 55                       	push	r13
141473aac: 41 54                       	push	r12
141473aae: 56                          	push	rsi
141473aaf: 57                          	push	rdi
141473ab0: 53                          	push	rbx
141473ab1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473ab8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141473abf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473ac5: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
141473acc: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473ad2: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473ad8: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473ade: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473ae7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473af0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473af9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473b01: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473b09: 48 8b 95 50 02 00 00        	mov	rdx, qword ptr [rbp + 0x250]
141473b10: 48 85 d2                    	test	rdx, rdx
141473b13: 74 16                       	je	0x141473b2b <.text+0x1472b2b>
141473b15: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
141473b1c: 48 c1 e2 04                 	shl	rdx, 0x4
141473b20: 41 b8 08 00 00 00           	mov	r8d, 0x8
141473b26: e8 f5 00 1a ff              	call	0x140613c20 <.text+0x612c20>
141473b2b: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473b33: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473b3b: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473b44: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473b4d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473b56: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473b5c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473b62: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473b68: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141473b6f: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473b75: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473b7c: 5b                          	pop	rbx
141473b7d: 5f                          	pop	rdi
141473b7e: 5e                          	pop	rsi
141473b7f: 41 5c                       	pop	r12
141473b81: 41 5d                       	pop	r13
141473b83: 41 5e                       	pop	r14
141473b85: 41 5f                       	pop	r15
141473b87: 5d                          	pop	rbp
141473b88: c3                          	ret
141473b89: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
141473b90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473b95: 55                          	push	rbp
141473b96: 41 57                       	push	r15
141473b98: 41 56                       	push	r14
141473b9a: 41 55                       	push	r13
141473b9c: 41 54                       	push	r12
141473b9e: 56                          	push	rsi
141473b9f: 57                          	push	rdi
141473ba0: 53                          	push	rbx
141473ba1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473ba8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141473baf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473bb5: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
141473bbc: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473bc2: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473bc8: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473bce: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473bd7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473be0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473be9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473bf1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473bf9: 48 83 bd 20 03 00 00 ff     	cmp	qword ptr [rbp + 0x320], -0x1
141473c01: 74 24                       	je	0x141473c27 <.text+0x1472c27>
141473c03: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
141473c0a: f0                          	lock
141473c0b: 48 ff 08                    	dec	qword ptr [rax]
141473c0e: 75 17                       	jne	0x141473c27 <.text+0x1472c27>
141473c10: ba 40 00 00 00              	mov	edx, 0x40
141473c15: 41 b8 08 00 00 00           	mov	r8d, 0x8
141473c1b: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
141473c22: e8 f9 ff 19 ff              	call	0x140613c20 <.text+0x612c20>
141473c27: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473c2e: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473c36: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473c3e: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473c47: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473c50: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473c59: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473c5f: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473c65: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473c6b: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
141473c71: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473c77: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473c7e: 5b                          	pop	rbx
141473c7f: 5f                          	pop	rdi
141473c80: 5e                          	pop	rsi
141473c81: 41 5c                       	pop	r12
141473c83: 41 5d                       	pop	r13
141473c85: 41 5e                       	pop	r14
141473c87: 41 5f                       	pop	r15
141473c89: 5d                          	pop	rbp
141473c8a: c3                          	ret
141473c8b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
141473c90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473c95: 55                          	push	rbp
141473c96: 41 57                       	push	r15
141473c98: 41 56                       	push	r14
141473c9a: 41 55                       	push	r13
141473c9c: 41 54                       	push	r12
141473c9e: 56                          	push	rsi
141473c9f: 57                          	push	rdi
141473ca0: 53                          	push	rbx
141473ca1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473ca8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141473caf: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473cb5: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
141473cbb: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473cc1: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473cc7: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473ccd: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473cd6: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473cdf: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	xmmword ptr [rsp + 0xa0], xmm8
141473ce9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473cf1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473cf9: 80 bd 35 03 00 00 00        	cmp	byte ptr [rbp + 0x335], 0x0
141473d00: 74 77                       	je	0x141473d79 <.text+0x1472d79>
141473d02: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473d09: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
141473d10: e8 1b 5e 40 ff              	call	0x140879b30 <.text+0x878b30>
141473d15: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473d1c: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473d24: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473d2c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473d35: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473d3e: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473d47: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473d4d: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473d53: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473d59: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
141473d5f: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473d65: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473d6c: 5b                          	pop	rbx
141473d6d: 5f                          	pop	rdi
141473d6e: 5e                          	pop	rsi
141473d6f: 41 5c                       	pop	r12
141473d71: 41 5d                       	pop	r13
141473d73: 41 5e                       	pop	r14
141473d75: 41 5f                       	pop	r15
141473d77: 5d                          	pop	rbp
141473d78: c3                          	ret
141473d79: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473d80: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473d88: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473d90: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473d99: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473da2: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473dab: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473db1: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473db7: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473dbd: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
141473dc3: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473dc9: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473dd0: 5b                          	pop	rbx
141473dd1: 5f                          	pop	rdi
141473dd2: 5e                          	pop	rsi
141473dd3: 41 5c                       	pop	r12
141473dd5: 41 5d                       	pop	r13
141473dd7: 41 5e                       	pop	r14
141473dd9: 41 5f                       	pop	r15
141473ddb: 5d                          	pop	rbp
141473ddc: c3                          	ret
141473ddd: 0f 1f 00                    	nop	dword ptr [rax]
141473de0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473de5: 55                          	push	rbp
141473de6: 41 57                       	push	r15
141473de8: 41 56                       	push	r14
141473dea: 41 55                       	push	r13
141473dec: 41 54                       	push	r12
141473dee: 56                          	push	rsi
141473def: 57                          	push	rdi
141473df0: 53                          	push	rbx
141473df1: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473df8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141473dff: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473e05: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
141473e0c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473e12: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473e18: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473e1e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473e27: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473e30: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473e39: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473e41: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473e49: 48 b8 03 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffd
141473e53: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
141473e5a: 48 89 42 10                 	mov	qword ptr [rdx + 0x10], rax
141473e5e: 48 8b 85 00 01 00 00        	mov	rax, qword ptr [rbp + 0x100]
141473e65: 48 89 42 18                 	mov	qword ptr [rdx + 0x18], rax
141473e69: 8b 85 58 01 00 00           	mov	eax, dword ptr [rbp + 0x158]
141473e6f: 89 42 20                    	mov	dword ptr [rdx + 0x20], eax
141473e72: 48 8b 85 70 01 00 00        	mov	rax, qword ptr [rbp + 0x170]
141473e79: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
141473e80: 48 89 41 10                 	mov	qword ptr [rcx + 0x10], rax
141473e84: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
141473e8b: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
141473e8e: 8b 85 20 03 00 00           	mov	eax, dword ptr [rbp + 0x320]
141473e94: 89 42 3c                    	mov	dword ptr [rdx + 0x3c], eax
141473e97: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
141473e9e: 0f 11 42 40                 	movups	xmmword ptr [rdx + 0x40], xmm0
141473ea2: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
141473ea9: 48 89 42 50                 	mov	qword ptr [rdx + 0x50], rax
141473ead: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
141473eb4: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473ebc: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473ec4: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473ecd: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473ed6: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473edf: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473ee5: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473eeb: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473ef1: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
141473ef7: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473efd: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473f04: 5b                          	pop	rbx
141473f05: 5f                          	pop	rdi
141473f06: 5e                          	pop	rsi
141473f07: 41 5c                       	pop	r12
141473f09: 41 5d                       	pop	r13
141473f0b: 41 5e                       	pop	r14
141473f0d: 41 5f                       	pop	r15
141473f0f: 5d                          	pop	rbp
141473f10: c3                          	ret
141473f11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141473f20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141473f25: 55                          	push	rbp
141473f26: 41 57                       	push	r15
141473f28: 41 56                       	push	r14
141473f2a: 41 55                       	push	r13
141473f2c: 41 54                       	push	r12
141473f2e: 56                          	push	rsi
141473f2f: 57                          	push	rdi
141473f30: 53                          	push	rbx
141473f31: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141473f38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141473f3f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141473f45: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
141473f4c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141473f52: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141473f58: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
141473f5e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141473f67: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141473f70: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141473f79: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141473f81: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141473f89: 80 bd 37 03 00 00 00        	cmp	byte ptr [rbp + 0x337], 0x0
141473f90: 74 6a                       	je	0x141473ffc <.text+0x1472ffc>
141473f92: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
141473f99: e8 12 b9 41 ff              	call	0x14088f8b0 <.text+0x88e8b0>
141473f9e: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141473fa6: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141473fae: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141473fb7: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141473fc0: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141473fc9: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141473fcf: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141473fd5: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141473fdb: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141473fe2: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141473fe8: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141473fef: 5b                          	pop	rbx
141473ff0: 5f                          	pop	rdi
141473ff1: 5e                          	pop	rsi
141473ff2: 41 5c                       	pop	r12
141473ff4: 41 5d                       	pop	r13
141473ff6: 41 5e                       	pop	r14
141473ff8: 41 5f                       	pop	r15
141473ffa: 5d                          	pop	rbp
141473ffb: c3                          	ret
141473ffc: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141474004: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
14147400c: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141474015: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14147401e: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141474027: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14147402d: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141474033: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141474039: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141474040: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141474046: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14147404d: 5b                          	pop	rbx
14147404e: 5f                          	pop	rdi
14147404f: 5e                          	pop	rsi
141474050: 41 5c                       	pop	r12
141474052: 41 5d                       	pop	r13
141474054: 41 5e                       	pop	r14
141474056: 41 5f                       	pop	r15
141474058: 5d                          	pop	rbp
141474059: c3                          	ret
14147405a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
141474060: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141474065: 55                          	push	rbp
141474066: 41 57                       	push	r15
141474068: 41 56                       	push	r14
14147406a: 41 55                       	push	r13
14147406c: 41 54                       	push	r12
14147406e: 56                          	push	rsi
14147406f: 57                          	push	rdi
141474070: 53                          	push	rbx
141474071: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141474078: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147407f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141474085: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147408c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141474092: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141474098: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147409e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
1414740a7: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1414740b0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1414740b9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414740c1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414740c9: 48 8b 95 40 01 00 00        	mov	rdx, qword ptr [rbp + 0x140]
1414740d0: 48 85 d2                    	test	rdx, rdx
1414740d3: 74 16                       	je	0x1414740eb <.text+0x14730eb>
1414740d5: 48 c1 e2 05                 	shl	rdx, 0x5
1414740d9: 41 b8 08 00 00 00           	mov	r8d, 0x8
1414740df: 48 8b 8d 20 03 00 00        	mov	rcx, qword ptr [rbp + 0x320]
1414740e6: e8 35 fb 19 ff              	call	0x140613c20 <.text+0x612c20>
1414740eb: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414740f3: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414740fb: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
141474104: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
14147410d: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141474116: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
14147411c: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
141474122: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141474128: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
14147412f: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
141474135: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
14147413c: 5b                          	pop	rbx
14147413d: 5f                          	pop	rdi
14147413e: 5e                          	pop	rsi
14147413f: 41 5c                       	pop	r12
141474141: 41 5d                       	pop	r13
141474143: 41 5e                       	pop	r14
141474145: 41 5f                       	pop	r15
141474147: 5d                          	pop	rbp
141474148: c3                          	ret
141474149: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
141474150: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141474155: 55                          	push	rbp
141474156: 41 57                       	push	r15
141474158: 41 56                       	push	r14
14147415a: 41 55                       	push	r13
14147415c: 41 54                       	push	r12
14147415e: 56                          	push	rsi
14147415f: 57                          	push	rdi
141474160: 53                          	push	rbx
141474161: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141474168: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147416f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141474175: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147417c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141474182: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141474188: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147418e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141474197: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
1414741a0: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
1414741a9: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
1414741b1: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
1414741b9: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
1414741c0: e8 9b 0b 42 ff              	call	0x140894d60 <.text+0x893d60>
1414741c5: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414741cd: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414741d5: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414741de: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414741e7: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414741f0: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414741f6: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414741fc: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
141474202: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
141474209: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
14147420f: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
141474216: 5b                          	pop	rbx
141474217: 5f                          	pop	rdi
141474218: 5e                          	pop	rsi
141474219: 41 5c                       	pop	r12
14147421b: 41 5d                       	pop	r13
14147421d: 41 5e                       	pop	r14
14147421f: 41 5f                       	pop	r15
141474221: 5d                          	pop	rbp
141474222: c3                          	ret
141474223: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141474230: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141474235: 55                          	push	rbp
141474236: 41 57                       	push	r15
141474238: 41 56                       	push	r14
14147423a: 41 55                       	push	r13
14147423c: 41 54                       	push	r12
14147423e: 56                          	push	rsi
14147423f: 57                          	push	rdi
141474240: 53                          	push	rbx
141474241: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141474248: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147424f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141474255: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147425c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141474262: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141474268: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147426e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141474277: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141474280: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141474289: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141474291: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141474299: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
1414742a0: e8 9b a8 41 ff              	call	0x14088eb40 <.text+0x88db40>
1414742a5: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1414742ad: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414742b5: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
1414742be: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414742c7: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414742d0: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414742d6: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414742dc: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414742e2: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414742e9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414742ef: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414742f6: 5b                          	pop	rbx
1414742f7: 5f                          	pop	rdi
1414742f8: 5e                          	pop	rsi
1414742f9: 41 5c                       	pop	r12
1414742fb: 41 5d                       	pop	r13
1414742fd: 41 5e                       	pop	r14
1414742ff: 41 5f                       	pop	r15
141474301: 5d                          	pop	rbp
141474302: c3                          	ret
141474303: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141474310: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141474315: 55                          	push	rbp
141474316: 41 57                       	push	r15
141474318: 41 56                       	push	r14
14147431a: 41 55                       	push	r13
14147431c: 41 54                       	push	r12
14147431e: 56                          	push	rsi
14147431f: 57                          	push	rdi
141474320: 53                          	push	rbx
141474321: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141474328: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147432f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141474335: 44 0f 29 74 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm14
14147433b: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141474341: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141474347: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147434d: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141474356: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
14147435f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	xmmword ptr [rsp + 0xa0], xmm8
141474369: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141474371: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141474379: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
141474380: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
141474387: 48 89 08                    	mov	qword ptr [rax], rcx
14147438a: c6 85 35 03 00 00 00        	mov	byte ptr [rbp + 0x335], 0x0
141474391: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
141474399: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
1414743a1: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	xmm8, xmmword ptr [rsp + 0xa0]
1414743ab: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
1414743b4: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
1414743bd: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
1414743c3: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
1414743c9: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414743cf: 44 0f 28 74 24 40           	movaps	xmm14, xmmword ptr [rsp + 0x40]
1414743d5: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414743db: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414743e2: 5b                          	pop	rbx
1414743e3: 5f                          	pop	rdi
1414743e4: 5e                          	pop	rsi
1414743e5: 41 5c                       	pop	r12
1414743e7: 41 5d                       	pop	r13
1414743e9: 41 5e                       	pop	r14
1414743eb: 41 5f                       	pop	r15
1414743ed: 5d                          	pop	rbp
1414743ee: c3                          	ret
1414743ef: 90                          	nop
1414743f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414743f5: 55                          	push	rbp
1414743f6: 41 57                       	push	r15
1414743f8: 41 56                       	push	r14
1414743fa: 41 55                       	push	r13
1414743fc: 41 54                       	push	r12
1414743fe: 56                          	push	rsi
1414743ff: 57                          	push	rdi
141474400: 53                          	push	rbx
141474401: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
141474408: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147440f: 44 0f 29 7c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm15
141474415: 66 44 0f 7f 74 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm14
14147441c: 44 0f 29 6c 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm13
141474422: 44 0f 29 64 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm12
141474428: 44 0f 29 5c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm11
14147442e: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
141474437: 44 0f 29 8c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm9
141474440: 44 0f 29 84 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm8
141474449: 0f 29 bc 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm7
141474451: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
141474459: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
141474460: e8 cb 87 8d ff              	call	0x140d4cc30 <.text+0xd4bc30>
141474465: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
14147446d: 0f 28 bc 24 b0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xb0]
141474475: 44 0f 28 84 24 a0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xa0]
14147447e: 44 0f 28 8c 24 90 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x90]
141474487: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
141474490: 44 0f 28 5c 24 70           	movaps	xmm11, xmmword ptr [rsp + 0x70]
141474496: 44 0f 28 64 24 60           	movaps	xmm12, xmmword ptr [rsp + 0x60]
14147449c: 44 0f 28 6c 24 50           	movaps	xmm13, xmmword ptr [rsp + 0x50]
1414744a2: 66 44 0f 6f 74 24 40        	movdqa	xmm14, xmmword ptr [rsp + 0x40]
1414744a9: 44 0f 28 7c 24 30           	movaps	xmm15, xmmword ptr [rsp + 0x30]
1414744af: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1414744b6: 5b                          	pop	rbx
1414744b7: 5f                          	pop	rdi
1414744b8: 5e                          	pop	rsi
1414744b9: 41 5c                       	pop	r12
1414744bb: 41 5d                       	pop	r13
1414744bd: 41 5e                       	pop	r14
1414744bf: 41 5f                       	pop	r15
1414744c1: 5d                          	pop	rbp
1414744c2: c3                          	ret
