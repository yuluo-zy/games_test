
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b19d90: 55                          	push	rbp
140b19d91: 41 57                       	push	r15
140b19d93: 41 56                       	push	r14
140b19d95: 41 55                       	push	r13
140b19d97: 41 54                       	push	r12
140b19d99: 56                          	push	rsi
140b19d9a: 57                          	push	rdi
140b19d9b: 53                          	push	rbx
140b19d9c: 48 81 ec 38 02 00 00        	sub	rsp, 0x238
140b19da3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140b19dab: 44 0f 29 85 a0 01 00 00     	movaps	xmmword ptr [rbp + 0x1a0], xmm8
140b19db3: 0f 29 bd 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm7
140b19dba: 0f 29 b5 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm6
140b19dc1: 48 c7 85 78 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x178], -0x2
140b19dcc: 4d 89 cf                    	mov	r15, r9
140b19dcf: 4c 89 c3                    	mov	rbx, r8
140b19dd2: 48 89 d7                    	mov	rdi, rdx
140b19dd5: 48 8b 01                    	mov	rax, qword ptr [rcx]
140b19dd8: 48 8b 30                    	mov	rsi, qword ptr [rax]
140b19ddb: 48 8d 15 66 f5 fc 01        	lea	rdx, [rip + 0x1fcf566]  # 0x142ae9348
140b19de2: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b19de9: 41 b8 22 00 00 00           	mov	r8d, 0x22
140b19def: e8 bc 50 ff ff              	call	0x140b0eeb0 <.text+0xb0deb0>
140b19df4: 31 c0                       	xor	eax, eax
140b19df6: 48 3b 85 e0 00 00 00        	cmp	rax, qword ptr [rbp + 0xe0]
140b19dfd: 0f 80 85 05 00 00           	jo	0x140b1a388 <.text+0xb19388>
140b19e03: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
140b19e0a: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140b19e11: 0f 10 85 40 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x140]
140b19e18: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
140b19e1f: 0f 10 85 30 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x130]
140b19e26: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
140b19e2d: 0f 10 85 20 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x120]
140b19e34: 0f 29 85 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm0
140b19e3b: 0f 10 85 e0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe0]
140b19e42: 0f 10 8d f0 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0xf0]
140b19e49: 0f 10 95 00 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x100]
140b19e50: 0f 10 9d 10 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x110]
140b19e57: 0f 29 9d 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm3
140b19e5e: 0f 29 95 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm2
140b19e65: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b19e69: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b19e6d: 4c 8b 33                    	mov	r14, qword ptr [rbx]
140b19e70: 48 8b 3f                    	mov	rdi, qword ptr [rdi]
140b19e73: c6 85 77 01 00 00 01        	mov	byte ptr [rbp + 0x177], 0x1
140b19e7a: 48 8d 55 60                 	lea	rdx, [rbp + 0x60]
140b19e7e: 4c 89 f1                    	mov	rcx, r14
140b19e81: e8 5a 08 da ff              	call	0x1408ba6e0 <.text+0x8b96e0>
140b19e86: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
140b19e8d: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140b19e94: 0f 28 85 c0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xc0]
140b19e9b: 0f 29 85 40 01 00 00        	movaps	xmmword ptr [rbp + 0x140], xmm0
140b19ea2: 0f 28 85 b0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xb0]
140b19ea9: 0f 29 85 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm0
140b19eb0: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
140b19eb7: 0f 29 85 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm0
140b19ebe: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b19ec2: 0f 28 4d 70                 	movaps	xmm1, xmmword ptr [rbp + 0x70]
140b19ec6: 0f 28 95 80 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x80]
140b19ecd: 0f 28 9d 90 00 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x90]
140b19ed4: 0f 29 9d 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm3
140b19edb: 0f 29 95 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm2
140b19ee2: 0f 29 8d f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm1
140b19ee9: 0f 29 85 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm0
140b19ef0: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
140b19efa: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
140b19f01: 0f b6 05 01 ad b1 02        	movzx	eax, byte ptr [rip + 0x2b1ad01] # 0x143634c09
140b19f08: b9 0f 00 00 00              	mov	ecx, 0xf
140b19f0d: ba 01 00 00 00              	mov	edx, 0x1
140b19f12: e8 f9 9c af ff              	call	0x140613c10 <.text+0x612c10>
140b19f17: 48 85 c0                    	test	rax, rax
140b19f1a: 0f 84 9d 04 00 00           	je	0x140b1a3bd <.text+0xb193bd>
140b19f20: 48 b9 61 64 65 5f 6c 6f 67 6f       	movabs	rcx, 0x6f676f6c5f656461
140b19f2a: 48 89 48 07                 	mov	qword ptr [rax + 0x7], rcx
140b19f2e: 48 b9 74 69 6e 79 5f 67 6c 61       	movabs	rcx, 0x616c675f796e6974
140b19f38: 48 89 08                    	mov	qword ptr [rax], rcx
140b19f3b: 48 c7 85 58 01 00 00 0f 00 00 00    	mov	qword ptr [rbp + 0x158], 0xf
140b19f46: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
140b19f4d: 48 c7 85 68 01 00 00 0f 00 00 00    	mov	qword ptr [rbp + 0x168], 0xf
140b19f58: 48 8d 5d b8                 	lea	rbx, [rbp - 0x48]
140b19f5c: 48 8d 95 e0 00 00 00        	lea	rdx, [rbp + 0xe0]
140b19f63: 41 b8 90 00 00 00           	mov	r8d, 0x90
140b19f69: 48 89 d9                    	mov	rcx, rbx
140b19f6c: e8 8f 9c e0 01              	call	0x142923c00 <.text+0x2922c00>
140b19f71: c6 85 77 01 00 00 00        	mov	byte ptr [rbp + 0x177], 0x0
140b19f78: 48 89 f9                    	mov	rcx, rdi
140b19f7b: 48 89 da                    	mov	rdx, rbx
140b19f7e: e8 dd 2c e1 ff              	call	0x14092cc60 <.text+0x92bc60>
140b19f83: 49 89 c4                    	mov	r12, rax
140b19f86: 4c 89 75 48                 	mov	qword ptr [rbp + 0x48], r14
140b19f8a: 48 89 7d 50                 	mov	qword ptr [rbp + 0x50], rdi
140b19f8e: 48 8b bd 28 02 00 00        	mov	rdi, qword ptr [rbp + 0x228]
140b19f95: 48 8b 85 20 02 00 00        	mov	rax, qword ptr [rbp + 0x220]
140b19f9c: 48 83 c6 10                 	add	rsi, 0x10
140b19fa0: 48 8b 00                    	mov	rax, qword ptr [rax]
140b19fa3: 4d 8b 37                    	mov	r14, qword ptr [r15]
140b19fa6: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140b19fab: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
140b19faf: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140b19fb4: 48 c7 44 24 20 0e 00 00 00  	mov	qword ptr [rsp + 0x20], 0xe
140b19fbd: 48 8d 15 f4 f3 fc 01        	lea	rdx, [rip + 0x1fcf3f4]  # 0x142ae93b8
140b19fc4: 4c 8d 0d 08 f4 fc 01        	lea	r9, [rip + 0x1fcf408]   # 0x142ae93d3
140b19fcb: 41 b8 1b 00 00 00           	mov	r8d, 0x1b
140b19fd1: 48 89 f1                    	mov	rcx, rsi
140b19fd4: e8 67 5e 02 00              	call	0x140b3fe40 <.text+0xb3ee40>
140b19fd9: 49 89 c7                    	mov	r15, rax
140b19fdc: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b19fe3: e8 98 31 18 00              	call	0x140c9d180 <.text+0xc9c180>
140b19fe8: 0f 28 b5 e0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xe0]
140b19fef: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b19ff6: 48 89 fa                    	mov	rdx, rdi
140b19ff9: e8 a2 a6 1f 00              	call	0x140d146a0 <.text+0xd136a0>
140b19ffe: 4c 8b ad e8 00 00 00        	mov	r13, qword ptr [rbp + 0xe8]
140b1a005: 4d 85 ed                    	test	r13, r13
140b1a008: 75 07                       	jne	0x140b1a011 <.text+0xb19011>
140b1a00a: 4c 8b ad f0 00 00 00        	mov	r13, qword ptr [rbp + 0xf0]
140b1a011: 48 8b bd 00 01 00 00        	mov	rdi, qword ptr [rbp + 0x100]
140b1a018: 49 8b 45 00                 	mov	rax, qword ptr [r13]
140b1a01c: 49 8b 5d 10                 	mov	rbx, qword ptr [r13 + 0x10]
140b1a020: 48 29 d8                    	sub	rax, rbx
140b1a023: 48 3d 87 00 00 00           	cmp	rax, 0x87
140b1a029: 0f 86 0f 03 00 00           	jbe	0x140b1a33e <.text+0xb1933e>
140b1a02f: 49 8b 45 08                 	mov	rax, qword ptr [r13 + 0x8]
140b1a033: 48 8d 0d 56 00 7c 00        	lea	rcx, [rip + 0x7c0056]   # 0x1412da090 <.text+0x12d9090>
140b1a03a: 48 89 0c 18                 	mov	qword ptr [rax + rbx], rcx
140b1a03e: 0f 11 74 18 08              	movups	xmmword ptr [rax + rbx + 0x8], xmm6
140b1a043: 0f 28 35 66 ef fc 01        	movaps	xmm6, xmmword ptr [rip + 0x1fcef66] # 0x142ae8fb0
140b1a04a: 0f 11 74 18 18              	movups	xmmword ptr [rax + rbx + 0x18], xmm6
140b1a04f: f2 0f 10 3d f9 2f eb 01     	movsd	xmm7, qword ptr [rip + 0x1eb2ff9] # 0x1429cd050
140b1a057: f2 0f 11 7c 18 28           	movsd	qword ptr [rax + rbx + 0x28], xmm7
140b1a05d: 4c 89 64 18 38              	mov	qword ptr [rax + rbx + 0x38], r12
140b1a062: 4c 89 7c 18 40              	mov	qword ptr [rax + rbx + 0x40], r15
140b1a067: c7 44 18 48 12 00 00 00     	mov	dword ptr [rax + rbx + 0x48], 0x12
140b1a06f: c6 44 18 58 00              	mov	byte ptr [rax + rbx + 0x58], 0x0
140b1a074: 48 89 7c 18 68              	mov	qword ptr [rax + rbx + 0x68], rdi
140b1a079: 4c 8d 3d b0 ac a4 00        	lea	r15, [rip + 0xa4acb0]   # 0x141564d30 <.text+0x1563d30>
140b1a080: 4c 89 7c 18 78              	mov	qword ptr [rax + rbx + 0x78], r15
140b1a085: 48 81 c3 88 00 00 00        	add	rbx, 0x88
140b1a08c: 49 89 5d 10                 	mov	qword ptr [r13 + 0x10], rbx
140b1a090: 48 8d 15 4a f3 fc 01        	lea	rdx, [rip + 0x1fcf34a]  # 0x142ae93e1
140b1a097: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b1a09e: 41 b8 20 00 00 00           	mov	r8d, 0x20
140b1a0a4: e8 07 4e ff ff              	call	0x140b0eeb0 <.text+0xb0deb0>
140b1a0a9: 31 c0                       	xor	eax, eax
140b1a0ab: 48 3b 85 e0 00 00 00        	cmp	rax, qword ptr [rbp + 0xe0]
140b1a0b2: 0f 80 1d 03 00 00           	jo	0x140b1a3d5 <.text+0xb193d5>
140b1a0b8: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
140b1a0bf: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140b1a0c6: 0f 10 85 40 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x140]
140b1a0cd: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
140b1a0d4: 0f 10 85 30 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x130]
140b1a0db: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
140b1a0e2: 0f 10 85 20 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x120]
140b1a0e9: 0f 29 85 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm0
140b1a0f0: 0f 10 85 e0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe0]
140b1a0f7: 0f 10 8d f0 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0xf0]
140b1a0fe: 0f 10 95 00 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x100]
140b1a105: 0f 10 9d 10 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x110]
140b1a10c: 0f 29 9d 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm3
140b1a113: 0f 29 95 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm2
140b1a11a: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b1a11e: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b1a122: c6 85 76 01 00 00 01        	mov	byte ptr [rbp + 0x176], 0x1
140b1a129: 48 8d 55 60                 	lea	rdx, [rbp + 0x60]
140b1a12d: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
140b1a131: e8 aa 05 da ff              	call	0x1408ba6e0 <.text+0x8b96e0>
140b1a136: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
140b1a140: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
140b1a147: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140b1a14e: 0f 28 85 c0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xc0]
140b1a155: 0f 29 85 40 01 00 00        	movaps	xmmword ptr [rbp + 0x140], xmm0
140b1a15c: 0f 28 85 b0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xb0]
140b1a163: 0f 29 85 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm0
140b1a16a: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
140b1a171: 0f 29 85 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm0
140b1a178: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b1a17c: 0f 28 4d 70                 	movaps	xmm1, xmmword ptr [rbp + 0x70]
140b1a180: 0f 28 95 80 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x80]
140b1a187: 0f 28 9d 90 00 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x90]
140b1a18e: 0f 29 9d 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm3
140b1a195: 0f 29 95 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm2
140b1a19c: 0f 29 8d f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm1
140b1a1a3: 0f 29 85 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm0
140b1a1aa: 48 89 8d 58 01 00 00        	mov	qword ptr [rbp + 0x158], rcx
140b1a1b1: 0f b6 05 51 aa b1 02        	movzx	eax, byte ptr [rip + 0x2b1aa51] # 0x143634c09
140b1a1b8: b9 0d 00 00 00              	mov	ecx, 0xd
140b1a1bd: ba 01 00 00 00              	mov	edx, 0x1
140b1a1c2: e8 49 9a af ff              	call	0x140613c10 <.text+0x612c10>
140b1a1c7: 48 85 c0                    	test	rax, rax
140b1a1ca: 0f 84 3a 02 00 00           	je	0x140b1a40a <.text+0xb1940a>
140b1a1d0: 48 b9 62 72 61 6e 63 68 65 73       	movabs	rcx, 0x736568636e617262
140b1a1da: 48 89 48 05                 	mov	qword ptr [rax + 0x5], rcx
140b1a1de: 48 b9 6c 6f 67 6f 5f 62 72 61       	movabs	rcx, 0x6172625f6f676f6c
140b1a1e8: 48 89 08                    	mov	qword ptr [rax], rcx
140b1a1eb: 48 c7 85 58 01 00 00 0d 00 00 00    	mov	qword ptr [rbp + 0x158], 0xd
140b1a1f6: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
140b1a1fd: 48 c7 85 68 01 00 00 0d 00 00 00    	mov	qword ptr [rbp + 0x168], 0xd
140b1a208: 48 8d 5d b8                 	lea	rbx, [rbp - 0x48]
140b1a20c: 48 8d 95 e0 00 00 00        	lea	rdx, [rbp + 0xe0]
140b1a213: 41 b8 90 00 00 00           	mov	r8d, 0x90
140b1a219: 48 89 d9                    	mov	rcx, rbx
140b1a21c: e8 df 99 e0 01              	call	0x142923c00 <.text+0x2922c00>
140b1a221: c6 85 76 01 00 00 00        	mov	byte ptr [rbp + 0x176], 0x0
140b1a228: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
140b1a22c: 48 89 da                    	mov	rdx, rbx
140b1a22f: e8 2c 2a e1 ff              	call	0x14092cc60 <.text+0x92bc60>
140b1a234: 48 89 c7                    	mov	rdi, rax
140b1a237: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140b1a23c: 48 8b 45 58                 	mov	rax, qword ptr [rbp + 0x58]
140b1a240: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140b1a245: 48 c7 44 24 20 17 00 00 00  	mov	qword ptr [rsp + 0x20], 0x17
140b1a24e: 48 8d 15 cb f1 fc 01        	lea	rdx, [rip + 0x1fcf1cb]  # 0x142ae9420
140b1a255: 4c 8d 0d e8 f1 fc 01        	lea	r9, [rip + 0x1fcf1e8]   # 0x142ae9444
140b1a25c: 41 b8 24 00 00 00           	mov	r8d, 0x24
140b1a262: 48 89 f1                    	mov	rcx, rsi
140b1a265: e8 d6 5b 02 00              	call	0x140b3fe40 <.text+0xb3ee40>
140b1a26a: 48 89 c6                    	mov	rsi, rax
140b1a26d: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b1a274: e8 07 2f 18 00              	call	0x140c9d180 <.text+0xc9c180>
140b1a279: 44 0f 28 85 e0 00 00 00     	movaps	xmm8, xmmword ptr [rbp + 0xe0]
140b1a281: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b1a288: 48 8b 95 28 02 00 00        	mov	rdx, qword ptr [rbp + 0x228]
140b1a28f: e8 0c a4 1f 00              	call	0x140d146a0 <.text+0xd136a0>
140b1a294: 48 8b 9d e8 00 00 00        	mov	rbx, qword ptr [rbp + 0xe8]
140b1a29b: 48 85 db                    	test	rbx, rbx
140b1a29e: 75 07                       	jne	0x140b1a2a7 <.text+0xb192a7>
140b1a2a0: 48 8b 9d f0 00 00 00        	mov	rbx, qword ptr [rbp + 0xf0]
140b1a2a7: 4c 8b a5 00 01 00 00        	mov	r12, qword ptr [rbp + 0x100]
140b1a2ae: 48 8b 03                    	mov	rax, qword ptr [rbx]
140b1a2b1: 4c 8b 73 10                 	mov	r14, qword ptr [rbx + 0x10]
140b1a2b5: 4c 29 f0                    	sub	rax, r14
140b1a2b8: 48 3d 87 00 00 00           	cmp	rax, 0x87
140b1a2be: 0f 86 9f 00 00 00           	jbe	0x140b1a363 <.text+0xb19363>
140b1a2c4: 48 8b 43 08                 	mov	rax, qword ptr [rbx + 0x8]
140b1a2c8: 48 8d 0d c1 fd 7b 00        	lea	rcx, [rip + 0x7bfdc1]   # 0x1412da090 <.text+0x12d9090>
140b1a2cf: 4a 89 0c 30                 	mov	qword ptr [rax + r14], rcx
140b1a2d3: 46 0f 11 44 30 08           	movups	xmmword ptr [rax + r14 + 0x8], xmm8
140b1a2d9: 42 0f 11 74 30 18           	movups	xmmword ptr [rax + r14 + 0x18], xmm6
140b1a2df: f2 42 0f 11 7c 30 28        	movsd	qword ptr [rax + r14 + 0x28], xmm7
140b1a2e6: 4a 89 7c 30 38              	mov	qword ptr [rax + r14 + 0x38], rdi
140b1a2eb: 4a 89 74 30 40              	mov	qword ptr [rax + r14 + 0x40], rsi
140b1a2f0: 42 c7 44 30 48 12 00 00 00  	mov	dword ptr [rax + r14 + 0x48], 0x12
140b1a2f9: 42 c6 44 30 58 00           	mov	byte ptr [rax + r14 + 0x58], 0x0
140b1a2ff: 4e 89 64 30 68              	mov	qword ptr [rax + r14 + 0x68], r12
140b1a304: 4e 89 7c 30 78              	mov	qword ptr [rax + r14 + 0x78], r15
140b1a309: 49 81 c6 88 00 00 00        	add	r14, 0x88
140b1a310: 4c 89 73 10                 	mov	qword ptr [rbx + 0x10], r14
140b1a314: 0f 28 b5 80 01 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x180]
140b1a31b: 0f 28 bd 90 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x190]
140b1a322: 44 0f 28 85 a0 01 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x1a0]
140b1a32a: 48 81 c4 38 02 00 00        	add	rsp, 0x238
140b1a331: 5b                          	pop	rbx
140b1a332: 5f                          	pop	rdi
140b1a333: 5e                          	pop	rsi
140b1a334: 41 5c                       	pop	r12
140b1a336: 41 5d                       	pop	r13
140b1a338: 41 5e                       	pop	r14
140b1a33a: 41 5f                       	pop	r15
140b1a33c: 5d                          	pop	rbp
140b1a33d: c3                          	ret
140b1a33e: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
140b1a347: 41 b8 88 00 00 00           	mov	r8d, 0x88
140b1a34d: 41 b9 01 00 00 00           	mov	r9d, 0x1
140b1a353: 4c 89 e9                    	mov	rcx, r13
140b1a356: 48 89 da                    	mov	rdx, rbx
140b1a359: e8 02 f7 d8 01              	call	0x1428a9a60 <.text+0x28a8a60>
140b1a35e: e9 cc fc ff ff              	jmp	0x140b1a02f <.text+0xb1902f>
140b1a363: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
140b1a36c: 41 b8 88 00 00 00           	mov	r8d, 0x88
140b1a372: 41 b9 01 00 00 00           	mov	r9d, 0x1
140b1a378: 48 89 d9                    	mov	rcx, rbx
140b1a37b: 4c 89 f2                    	mov	rdx, r14
140b1a37e: e8 dd f6 d8 01              	call	0x1428a9a60 <.text+0x28a8a60>
140b1a383: e9 3c ff ff ff              	jmp	0x140b1a2c4 <.text+0xb192c4>
140b1a388: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
140b1a38f: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
140b1a393: 48 8d 05 06 f0 fc 01        	lea	rax, [rip + 0x1fcf006]  # 0x142ae93a0
140b1a39a: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b1a39f: 48 8d 0d e2 ec fc 01        	lea	rcx, [rip + 0x1fcece2]  # 0x142ae9088
140b1a3a6: 4c 8d 0d bb ec fc 01        	lea	r9, [rip + 0x1fcecbb]   # 0x142ae9068
140b1a3ad: 4c 8d 45 b8                 	lea	r8, [rbp - 0x48]
140b1a3b1: ba 2b 00 00 00              	mov	edx, 0x2b
140b1a3b6: e8 a5 f3 db 01              	call	0x1428d9760 <.text+0x28d8760>
140b1a3bb: eb 63                       	jmp	0x140b1a420 <.text+0xb19420>
140b1a3bd: 4c 8d 05 f4 4b 11 02        	lea	r8, [rip + 0x2114bf4]   # 0x142c2efb8
140b1a3c4: b9 01 00 00 00              	mov	ecx, 0x1
140b1a3c9: ba 0f 00 00 00              	mov	edx, 0xf
140b1a3ce: e8 10 ec db 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140b1a3d3: eb 4b                       	jmp	0x140b1a420 <.text+0xb19420>
140b1a3d5: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
140b1a3dc: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
140b1a3e0: 48 8d 05 21 f0 fc 01        	lea	rax, [rip + 0x1fcf021]  # 0x142ae9408
140b1a3e7: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b1a3ec: 48 8d 0d 95 ec fc 01        	lea	rcx, [rip + 0x1fcec95]  # 0x142ae9088
140b1a3f3: 4c 8d 0d 6e ec fc 01        	lea	r9, [rip + 0x1fcec6e]   # 0x142ae9068
140b1a3fa: 4c 8d 45 b8                 	lea	r8, [rbp - 0x48]
140b1a3fe: ba 2b 00 00 00              	mov	edx, 0x2b
140b1a403: e8 58 f3 db 01              	call	0x1428d9760 <.text+0x28d8760>
140b1a408: eb 16                       	jmp	0x140b1a420 <.text+0xb19420>
140b1a40a: 4c 8d 05 a7 4b 11 02        	lea	r8, [rip + 0x2114ba7]   # 0x142c2efb8
140b1a411: b9 01 00 00 00              	mov	ecx, 0x1
140b1a416: ba 0d 00 00 00              	mov	edx, 0xd
140b1a41b: e8 c3 eb db 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140b1a420: 0f 0b                       	ud2
140b1a422: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140b1a430: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a435: 55                          	push	rbp
140b1a436: 41 57                       	push	r15
140b1a438: 41 56                       	push	r14
140b1a43a: 41 55                       	push	r13
140b1a43c: 41 54                       	push	r12
140b1a43e: 56                          	push	rsi
140b1a43f: 57                          	push	rdi
140b1a440: 53                          	push	rbx
140b1a441: 48 83 ec 68                 	sub	rsp, 0x68
140b1a445: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a44c: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a452: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a457: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a45c: c6 85 77 01 00 00 00        	mov	byte ptr [rbp + 0x177], 0x0
140b1a463: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b1a46a: e8 d1 e8 d7 ff              	call	0x140898d40 <.text+0x897d40>
140b1a46f: c6 85 77 01 00 00 00        	mov	byte ptr [rbp + 0x177], 0x0
140b1a476: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a47b: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a480: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a486: 48 83 c4 68                 	add	rsp, 0x68
140b1a48a: 5b                          	pop	rbx
140b1a48b: 5f                          	pop	rdi
140b1a48c: 5e                          	pop	rsi
140b1a48d: 41 5c                       	pop	r12
140b1a48f: 41 5d                       	pop	r13
140b1a491: 41 5e                       	pop	r14
140b1a493: 41 5f                       	pop	r15
140b1a495: 5d                          	pop	rbp
140b1a496: c3                          	ret
140b1a497: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140b1a4a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a4a5: 55                          	push	rbp
140b1a4a6: 41 57                       	push	r15
140b1a4a8: 41 56                       	push	r14
140b1a4aa: 41 55                       	push	r13
140b1a4ac: 41 54                       	push	r12
140b1a4ae: 56                          	push	rsi
140b1a4af: 57                          	push	rdi
140b1a4b0: 53                          	push	rbx
140b1a4b1: 48 83 ec 68                 	sub	rsp, 0x68
140b1a4b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a4bc: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a4c2: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a4c7: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a4cc: 80 bd 77 01 00 00 00        	cmp	byte ptr [rbp + 0x177], 0x0
140b1a4d3: 75 21                       	jne	0x140b1a4f6 <.text+0xb194f6>
140b1a4d5: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a4da: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a4df: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a4e5: 48 83 c4 68                 	add	rsp, 0x68
140b1a4e9: 5b                          	pop	rbx
140b1a4ea: 5f                          	pop	rdi
140b1a4eb: 5e                          	pop	rsi
140b1a4ec: 41 5c                       	pop	r12
140b1a4ee: 41 5d                       	pop	r13
140b1a4f0: 41 5e                       	pop	r14
140b1a4f2: 41 5f                       	pop	r15
140b1a4f4: 5d                          	pop	rbp
140b1a4f5: c3                          	ret
140b1a4f6: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
140b1a4fa: e8 31 e6 d6 ff              	call	0x140888b30 <.text+0x887b30>
140b1a4ff: eb d4                       	jmp	0x140b1a4d5 <.text+0xb194d5>
140b1a501: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b1a510: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a515: 55                          	push	rbp
140b1a516: 41 57                       	push	r15
140b1a518: 41 56                       	push	r14
140b1a51a: 41 55                       	push	r13
140b1a51c: 41 54                       	push	r12
140b1a51e: 56                          	push	rsi
140b1a51f: 57                          	push	rdi
140b1a520: 53                          	push	rbx
140b1a521: 48 83 ec 68                 	sub	rsp, 0x68
140b1a525: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a52c: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a532: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a537: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a53c: c6 85 76 01 00 00 00        	mov	byte ptr [rbp + 0x176], 0x0
140b1a543: 48 8d 8d e0 00 00 00        	lea	rcx, [rbp + 0xe0]
140b1a54a: e8 f1 e7 d7 ff              	call	0x140898d40 <.text+0x897d40>
140b1a54f: c6 85 76 01 00 00 00        	mov	byte ptr [rbp + 0x176], 0x0
140b1a556: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a55b: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a560: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a566: 48 83 c4 68                 	add	rsp, 0x68
140b1a56a: 5b                          	pop	rbx
140b1a56b: 5f                          	pop	rdi
140b1a56c: 5e                          	pop	rsi
140b1a56d: 41 5c                       	pop	r12
140b1a56f: 41 5d                       	pop	r13
140b1a571: 41 5e                       	pop	r14
140b1a573: 41 5f                       	pop	r15
140b1a575: 5d                          	pop	rbp
140b1a576: c3                          	ret
140b1a577: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140b1a580: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a585: 55                          	push	rbp
140b1a586: 41 57                       	push	r15
140b1a588: 41 56                       	push	r14
140b1a58a: 41 55                       	push	r13
140b1a58c: 41 54                       	push	r12
140b1a58e: 56                          	push	rsi
140b1a58f: 57                          	push	rdi
140b1a590: 53                          	push	rbx
140b1a591: 48 83 ec 68                 	sub	rsp, 0x68
140b1a595: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a59c: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a5a2: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a5a7: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a5ac: 80 bd 76 01 00 00 00        	cmp	byte ptr [rbp + 0x176], 0x0
140b1a5b3: 75 21                       	jne	0x140b1a5d6 <.text+0xb195d6>
140b1a5b5: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a5ba: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a5bf: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a5c5: 48 83 c4 68                 	add	rsp, 0x68
140b1a5c9: 5b                          	pop	rbx
140b1a5ca: 5f                          	pop	rdi
140b1a5cb: 5e                          	pop	rsi
140b1a5cc: 41 5c                       	pop	r12
140b1a5ce: 41 5d                       	pop	r13
140b1a5d0: 41 5e                       	pop	r14
140b1a5d2: 41 5f                       	pop	r15
140b1a5d4: 5d                          	pop	rbp
140b1a5d5: c3                          	ret
140b1a5d6: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
140b1a5da: e8 51 e5 d6 ff              	call	0x140888b30 <.text+0x887b30>
140b1a5df: eb d4                       	jmp	0x140b1a5b5 <.text+0xb195b5>
140b1a5e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b1a5f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a5f5: 55                          	push	rbp
140b1a5f6: 41 57                       	push	r15
140b1a5f8: 41 56                       	push	r14
140b1a5fa: 41 55                       	push	r13
140b1a5fc: 41 54                       	push	r12
140b1a5fe: 56                          	push	rsi
140b1a5ff: 57                          	push	rdi
140b1a600: 53                          	push	rbx
140b1a601: 48 83 ec 68                 	sub	rsp, 0x68
140b1a605: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a60c: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a612: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a617: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a61c: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140b1a620: e8 0b 26 23 00              	call	0x140d4cc30 <.text+0xd4bc30>
140b1a625: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a62a: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a62f: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a635: 48 83 c4 68                 	add	rsp, 0x68
140b1a639: 5b                          	pop	rbx
140b1a63a: 5f                          	pop	rdi
140b1a63b: 5e                          	pop	rsi
140b1a63c: 41 5c                       	pop	r12
140b1a63e: 41 5d                       	pop	r13
140b1a640: 41 5e                       	pop	r14
140b1a642: 41 5f                       	pop	r15
140b1a644: 5d                          	pop	rbp
140b1a645: c3                          	ret
140b1a646: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140b1a650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1a655: 55                          	push	rbp
140b1a656: 41 57                       	push	r15
140b1a658: 41 56                       	push	r14
140b1a65a: 41 55                       	push	r13
140b1a65c: 41 54                       	push	r12
140b1a65e: 56                          	push	rsi
140b1a65f: 57                          	push	rdi
140b1a660: 53                          	push	rbx
140b1a661: 48 83 ec 68                 	sub	rsp, 0x68
140b1a665: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1a66c: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140b1a672: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140b1a677: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140b1a67c: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140b1a680: e8 ab 25 23 00              	call	0x140d4cc30 <.text+0xd4bc30>
140b1a685: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140b1a68a: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140b1a68f: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140b1a695: 48 83 c4 68                 	add	rsp, 0x68
140b1a699: 5b                          	pop	rbx
140b1a69a: 5f                          	pop	rdi
140b1a69b: 5e                          	pop	rsi
140b1a69c: 41 5c                       	pop	r12
140b1a69e: 41 5d                       	pop	r13
140b1a6a0: 41 5e                       	pop	r14
140b1a6a2: 41 5f                       	pop	r15
140b1a6a4: 5d                          	pop	rbp
140b1a6a5: c3                          	ret
140b1a6a6: cc                          	int3
140b1a6a7: cc                          	int3
140b1a6a8: cc                          	int3
140b1a6a9: cc                          	int3
140b1a6aa: cc                          	int3
140b1a6ab: cc                          	int3
140b1a6ac: cc                          	int3
140b1a6ad: cc                          	int3
140b1a6ae: cc                          	int3
140b1a6af: cc                          	int3
